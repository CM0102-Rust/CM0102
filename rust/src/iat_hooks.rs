//! Detours in cm0102.exe itself.
//!
//! For now only through the import table (IAT): swaps the address of a Windows function the exe
//! calls for one of ours. The detours in game routines (code inside .text) will come here too.
//!
//! [pt] Desvios no proprio cm0102.exe.
//!
//! Por ora so pela tabela de importacoes (IAT): troca o endereco de uma funcao do Windows que o exe
//! chama por uma nossa. Os desvios em rotinas do jogo (codigo dentro do .text) virao aqui tambem.

use core::ffi::c_void;

use crate::registro::linha;
use crate::win;

/// Swaps, in the main exe's IAT, `dll!function` for `replacement`. Returns the old address.
///
/// [pt] Troca, na IAT do exe principal, `dll!funcao` por `nova`. Devolve o endereco antigo.
pub unsafe fn replace_import(dll: &str, function: &str, replacement: usize) -> Option<usize> {
    let base = win::GetModuleHandleW(core::ptr::null()) as usize;
    let rd = |off: usize| *((base + off) as *const u32) as usize;
    let pe = rd(0x3c);
    let imp_rva = rd(pe + 0x80); // import directory (PE32)
    let mut desc = base + imp_rva;
    loop {
        let name_rva = *((desc + 12) as *const u32) as usize;
        if name_rva == 0 {
            break;
        }
        let dll_name = cstr(base + name_rva);
        if dll_name.eq_ignore_ascii_case(dll) {
            let original_fn = *(desc as *const u32) as usize; // OriginalFirstThunk (names)
            let iat = *((desc + 16) as *const u32) as usize; // FirstThunk (addresses)
            let mut i = 0;
            loop {
                let thunk = *((base + original_fn + i * 4) as *const u32) as usize;
                if thunk == 0 {
                    break;
                }
                if thunk & 0x8000_0000 == 0 && cstr(base + thunk + 2) == function {
                    let slot = (base + iat + i * 4) as *mut usize;
                    let mut old_protect = 0u32;
                    win::VirtualProtect(slot as *mut c_void, 4, win::PAGE_READWRITE, &mut old_protect);
                    let previous = *slot;
                    *slot = replacement;
                    win::VirtualProtect(slot as *mut c_void, 4, old_protect, &mut old_protect);
                    linha(&format!("gancho: {}!{} (IAT {:#x})", dll, function, slot as usize));
                    return Some(previous);
                }
                i += 1;
            }
        }
        desc += 20;
    }
    linha(&format!("hook: {}!{} not found in the IAT", dll, function));
    None
}

/// Visits every IAT entry of the exe: `f(dll, name or None if by ordinal, current address)`;
/// if it returns Some(new), the entry points to `new` from then on. Returns how many it swapped.
///
/// [pt] Visita todas as entradas da IAT do exe: `f(dll, nome ou None se por numero, endereco atual)`;
/// se devolver Some(novo), a entrada passa a apontar para `novo`. Devolve quantas trocou.
pub unsafe fn for_each_import(mut f: impl FnMut(&str, Option<&str>, usize) -> Option<usize>) -> usize {
    let base = win::GetModuleHandleW(core::ptr::null()) as usize;
    let rd = |off: usize| *((base + off) as *const u32) as usize;
    let pe = rd(0x3c);
    let mut desc = base + rd(pe + 0x80);
    let mut swapped = 0;
    while *((desc + 12) as *const u32) != 0 {
        let dll = cstr(base + *((desc + 12) as *const u32) as usize);
        let original_fn = *(desc as *const u32) as usize;
        let iat = *((desc + 16) as *const u32) as usize;
        let mut i = 0;
        loop {
            let thunk = *((base + original_fn + i * 4) as *const u32) as usize;
            if thunk == 0 {
                break;
            }
            let name = if thunk & 0x8000_0000 == 0 { Some(cstr(base + thunk + 2)) } else { None };
            let slot = (base + iat + i * 4) as *mut usize;
            if let Some(new_addr) = f(dll, name, *slot) {
                let mut old_protect = 0u32;
                win::VirtualProtect(slot as *mut c_void, 4, win::PAGE_READWRITE, &mut old_protect);
                *slot = new_addr;
                win::VirtualProtect(slot as *mut c_void, 4, old_protect, &mut old_protect);
                swapped += 1;
            }
            i += 1;
        }
        desc += 20;
    }
    swapped
}

unsafe fn cstr(p: usize) -> &'static str {
    let mut n = 0;
    while *((p + n) as *const u8) != 0 {
        n += 1;
    }
    core::str::from_utf8(core::slice::from_raw_parts(p as *const u8, n)).unwrap_or("")
}

/// The window procedure calls ChangeDisplaySettingsA when gaining/losing focus in full screen
/// (back to 800x600x16 / restores the Windows mode). With our video layer the mode never changes.
///
/// [pt] O procedimento da janela chama ChangeDisplaySettingsA ao ganhar/perder o foco em tela cheia
/// (volta a 800x600x16 / devolve o modo do Windows). Com a nossa camada de video o modo nunca muda.
unsafe extern "system" fn change_display_settings_a(mode: *const c_void, flags: u32) -> i32 {
    linha(&format!("ChangeDisplaySettingsA({:p}, {:#x}) ignored", mode, flags));
    0 // DISP_CHANGE_SUCCESSFUL
}

pub unsafe fn install() {
    replace_import("USER32.dll", "ChangeDisplaySettingsA", change_display_settings_a as *const () as usize);
}

static mut SHOW_WINDOW_ORIGINAL: usize = 0;

/// Hidden mode: the game shows the window with SW_SHOW (5), which steals the focus from whoever is using the PC.
/// Swaps it for SW_SHOWNA (8): shows without activating. The window goes off screen right after.
///
/// [pt] Modo oculto: o jogo mostra a janela com SW_SHOW (5), que rouba o foco de quem esta usando o PC.
/// Troca por SW_SHOWNA (8): mostra sem ativar. A janela vai para fora da tela logo depois.
unsafe extern "system" fn show_window(hwnd: *mut c_void, mode: i32) -> i32 {
    let f: unsafe extern "system" fn(*mut c_void, i32) -> i32 = core::mem::transmute(SHOW_WINDOW_ORIGINAL);
    f(hwnd, if mode == 5 || mode == 1 { 8 } else { mode })
}

static mut CREATE_WINDOW_ORIGINAL: usize = 0;

/// Hidden mode: the window is already born off screen (before, it showed up for an instant and flickered).
///
/// [pt] Modo oculto: a janela ja nasce fora da tela (antes ela aparecia por um instante e piscava).
unsafe extern "system" fn create_window_ex_a(
    ex: u32, class: *const u8, title: *const u8, style: u32, _x: i32, _y: i32, w: i32, h: i32,
    parent: *mut c_void, menu: *mut c_void, inst: *mut c_void, param: *mut c_void,
) -> *mut c_void {
    type F = unsafe extern "system" fn(u32, *const u8, *const u8, u32, i32, i32, i32, i32, *mut c_void, *mut c_void, *mut c_void, *mut c_void) -> *mut c_void;
    let f: F = core::mem::transmute(CREATE_WINDOW_ORIGINAL);
    f(ex, class, title, style, -32000, -32000, w, h, parent, menu, inst, param)
}

static mut MOVE_WINDOW_ORIGINAL: usize = 0;

/// Hidden mode: the game centres the window at the video size (1920x1080) whenever it restarts the
/// video, covering the screen of whoever uses the PC for an instant (the "flicker"). Keeps it off screen.
///
/// [pt] Modo oculto: o jogo centraliza a janela no tamanho do video (1920x1080) sempre que reinicia o
/// video, cobrindo a tela de quem usa o PC por um instante (a "piscada"). Mantem fora da tela.
unsafe extern "system" fn move_window(hwnd: *mut c_void, _x: i32, _y: i32, w: i32, h: i32, repaint: i32) -> i32 {
    let f: unsafe extern "system" fn(*mut c_void, i32, i32, i32, i32, i32) -> i32 = core::mem::transmute(MOVE_WINDOW_ORIGINAL);
    f(hwnd, -32000, -32000, w, h, repaint)
}

/// `DirectSoundCreate` (DSOUND, imported only by ordinal 1) in the tests: no sound card
/// (DSERR_NODRIVER). The game goes on without sound, and the PC of whoever uses it stays silent. `simular.py --som`
/// keeps the sound.
///
/// [pt] `DirectSoundCreate` (DSOUND, importado so pelo numero 1) nos testes: sem placa de som
/// (DSERR_NODRIVER). O jogo segue sem som, e o PC de quem usa fica em silencio. `simular.py --som`
/// mantem o som.
unsafe extern "system" fn direct_sound_create(_guid: *const c_void, out: *mut *mut c_void, _outer: *mut c_void) -> u32 {
    if !out.is_null() {
        *out = core::ptr::null_mut();
    }
    0x8878_0078
}

static mut CREATE_MUTEX_ORIGINAL: usize = 0;
static mut MUTEX_NAME: [u8; 64] = [0; 64];

/// The game creates the mutex "Championship Manager 2001/02 Process" (0x68d3a0) and, if it already exists, opens the "Already
/// Running" box and quits. The name gets the hash of the exe's path: the same exe in the same path stays blocked; another exe
/// or another folder opens alongside (e.g. the original and ours, Lucas's game and the tests' one).
///
/// [pt] O jogo cria o mutex "Championship Manager 2001/02 Process" (0x68d3a0) e, se já existe, abre a caixa "Already
/// Running" e sai. O nome ganha o hash do caminho do exe: o mesmo exe no mesmo caminho continua barrado; outro exe
/// ou outra pasta abre ao lado (ex.: o original e o nosso, o jogo do Lucas e o dos testes).
unsafe extern "system" fn create_mutex_a(attributes: *mut c_void, initial_owner: i32, name: *const u8) -> *mut c_void {
    let f: unsafe extern "system" fn(*mut c_void, i32, *const u8) -> *mut c_void = core::mem::transmute(CREATE_MUTEX_ORIGINAL);
    const GAME_MUTEX: &str = "Championship Manager 2001/02 Process";
    if name.is_null() || cstr(name as usize) != GAME_MUTEX {
        return f(attributes, initial_owner, name);
    }
    let mut path = [0u8; 520];
    let n = win::GetModuleFileNameA(core::ptr::null_mut(), path.as_mut_ptr(), path.len() as u32) as usize;
    let h = path[..n].iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b.to_ascii_lowercase() as u64).wrapping_mul(0x100_0000_01b3));
    let new_name = format!("{} {:016x}\0", GAME_MUTEX, h);
    MUTEX_NAME[..new_name.len()].copy_from_slice(new_name.as_bytes());
    linha(&format!("game mutex: {}", new_name.trim_end_matches('\0')));
    f(attributes, initial_owner, core::ptr::addr_of!(MUTEX_NAME) as *const u8)
}

/// Called when the DLL loads (before the game creates the window).
///
/// [pt] Chamado no carregamento da DLL (antes de o jogo criar a janela).
pub unsafe fn install_early() {
    if let Some(a) = replace_import("KERNEL32.dll", "CreateMutexA", create_mutex_a as *const () as usize) {
        CREATE_MUTEX_ORIGINAL = a;
    }
    // CM_OCULTO=2: hidden with sound (the value has the same length; one extra variable would change the heap)
    if crate::apresentacao::oculto() && win::texto_ambiente("CM_OCULTO") != "2" {
        let n = for_each_import(|dll, name, _| {
            (dll.eq_ignore_ascii_case("DSOUND.dll") && name.is_none()).then(|| direct_sound_create as *const () as usize)
        });
        linha(&format!("hook: DSOUND silent in the tests ({} entry)", n));
        if let Some(a) = replace_import("USER32.dll", "MoveWindow", move_window as *const () as usize) {
            MOVE_WINDOW_ORIGINAL = a;
        }
        if let Some(a) = replace_import("USER32.dll", "CreateWindowExA", create_window_ex_a as *const () as usize) {
            CREATE_WINDOW_ORIGINAL = a;
        }
        if let Some(a) = replace_import("USER32.dll", "ShowWindow", show_window as *const () as usize) {
            SHOW_WINDOW_ORIGINAL = a;
        }
    }
}
