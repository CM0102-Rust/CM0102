//! Desvios no proprio cm0102.exe.
//!
//! Por ora so pela tabela de importacoes (IAT): troca o endereco de uma funcao do Windows que o exe
//! chama por uma nossa. Os desvios em rotinas do jogo (codigo dentro do .text) virao aqui tambem.

use core::ffi::c_void;

use crate::registro::linha;
use crate::win;

/// Troca, na IAT do exe principal, `dll!funcao` por `nova`. Devolve o endereco antigo.
pub unsafe fn trocar_importacao(dll: &str, funcao: &str, nova: usize) -> Option<usize> {
    let base = win::GetModuleHandleW(core::ptr::null()) as usize;
    let rd = |off: usize| *((base + off) as *const u32) as usize;
    let pe = rd(0x3c);
    let imp_rva = rd(pe + 0x80); // diretorio de importacoes (PE32)
    let mut desc = base + imp_rva;
    loop {
        let nome_rva = *((desc + 12) as *const u32) as usize;
        if nome_rva == 0 {
            break;
        }
        let nome_dll = cstr(base + nome_rva);
        if nome_dll.eq_ignore_ascii_case(dll) {
            let orig = *(desc as *const u32) as usize; // OriginalFirstThunk (nomes)
            let iat = *((desc + 16) as *const u32) as usize; // FirstThunk (enderecos)
            let mut i = 0;
            loop {
                let thunk = *((base + orig + i * 4) as *const u32) as usize;
                if thunk == 0 {
                    break;
                }
                if thunk & 0x8000_0000 == 0 && cstr(base + thunk + 2) == funcao {
                    let slot = (base + iat + i * 4) as *mut usize;
                    let mut antiga = 0u32;
                    win::VirtualProtect(slot as *mut c_void, 4, win::PAGE_READWRITE, &mut antiga);
                    let anterior = *slot;
                    *slot = nova;
                    win::VirtualProtect(slot as *mut c_void, 4, antiga, &mut antiga);
                    linha(&format!("gancho: {}!{} (IAT {:#x})", dll, funcao, slot as usize));
                    return Some(anterior);
                }
                i += 1;
            }
        }
        desc += 20;
    }
    linha(&format!("gancho: {}!{} nao encontrado na IAT", dll, funcao));
    None
}

/// Visita todas as entradas da IAT do exe: `f(dll, nome ou None se por numero, endereco atual)`;
/// se devolver Some(novo), a entrada passa a apontar para `novo`. Devolve quantas trocou.
pub unsafe fn para_cada_importacao(mut f: impl FnMut(&str, Option<&str>, usize) -> Option<usize>) -> usize {
    let base = win::GetModuleHandleW(core::ptr::null()) as usize;
    let rd = |off: usize| *((base + off) as *const u32) as usize;
    let pe = rd(0x3c);
    let mut desc = base + rd(pe + 0x80);
    let mut trocadas = 0;
    while *((desc + 12) as *const u32) != 0 {
        let dll = cstr(base + *((desc + 12) as *const u32) as usize);
        let orig = *(desc as *const u32) as usize;
        let iat = *((desc + 16) as *const u32) as usize;
        let mut i = 0;
        loop {
            let thunk = *((base + orig + i * 4) as *const u32) as usize;
            if thunk == 0 {
                break;
            }
            let nome = if thunk & 0x8000_0000 == 0 { Some(cstr(base + thunk + 2)) } else { None };
            let slot = (base + iat + i * 4) as *mut usize;
            if let Some(novo) = f(dll, nome, *slot) {
                let mut antiga = 0u32;
                win::VirtualProtect(slot as *mut c_void, 4, win::PAGE_READWRITE, &mut antiga);
                *slot = novo;
                win::VirtualProtect(slot as *mut c_void, 4, antiga, &mut antiga);
                trocadas += 1;
            }
            i += 1;
        }
        desc += 20;
    }
    trocadas
}

unsafe fn cstr(p: usize) -> &'static str {
    let mut n = 0;
    while *((p + n) as *const u8) != 0 {
        n += 1;
    }
    core::str::from_utf8(core::slice::from_raw_parts(p as *const u8, n)).unwrap_or("")
}

/// O procedimento da janela chama ChangeDisplaySettingsA ao ganhar/perder o foco em tela cheia
/// (volta a 800x600x16 / devolve o modo do Windows). Com a nossa camada de video o modo nunca muda.
unsafe extern "system" fn change_display_settings_a(modo: *const c_void, flags: u32) -> i32 {
    linha(&format!("ChangeDisplaySettingsA({:p}, {:#x}) ignorado", modo, flags));
    0 // DISP_CHANGE_SUCCESSFUL
}

pub unsafe fn instalar() {
    trocar_importacao("USER32.dll", "ChangeDisplaySettingsA", change_display_settings_a as *const () as usize);
}

static mut SHOW_WINDOW_ORIGINAL: usize = 0;

/// Modo oculto: o jogo mostra a janela com SW_SHOW (5), que rouba o foco de quem esta usando o PC.
/// Troca por SW_SHOWNA (8): mostra sem ativar. A janela vai para fora da tela logo depois.
unsafe extern "system" fn show_window(hwnd: *mut c_void, modo: i32) -> i32 {
    let f: unsafe extern "system" fn(*mut c_void, i32) -> i32 = core::mem::transmute(SHOW_WINDOW_ORIGINAL);
    f(hwnd, if modo == 5 || modo == 1 { 8 } else { modo })
}

static mut CREATE_WINDOW_ORIGINAL: usize = 0;

/// Modo oculto: a janela ja nasce fora da tela (antes ela aparecia por um instante e piscava).
unsafe extern "system" fn create_window_ex_a(
    ex: u32, classe: *const u8, titulo: *const u8, estilo: u32, _x: i32, _y: i32, w: i32, h: i32,
    pai: *mut c_void, menu: *mut c_void, inst: *mut c_void, param: *mut c_void,
) -> *mut c_void {
    type F = unsafe extern "system" fn(u32, *const u8, *const u8, u32, i32, i32, i32, i32, *mut c_void, *mut c_void, *mut c_void, *mut c_void) -> *mut c_void;
    let f: F = core::mem::transmute(CREATE_WINDOW_ORIGINAL);
    f(ex, classe, titulo, estilo, -32000, -32000, w, h, pai, menu, inst, param)
}

static mut MOVE_WINDOW_ORIGINAL: usize = 0;

/// Modo oculto: o jogo centraliza a janela no tamanho do video (1920x1080) sempre que reinicia o
/// video, cobrindo a tela de quem usa o PC por um instante (a "piscada"). Mantem fora da tela.
unsafe extern "system" fn move_window(hwnd: *mut c_void, _x: i32, _y: i32, w: i32, h: i32, redesenhar: i32) -> i32 {
    let f: unsafe extern "system" fn(*mut c_void, i32, i32, i32, i32, i32) -> i32 = core::mem::transmute(MOVE_WINDOW_ORIGINAL);
    f(hwnd, -32000, -32000, w, h, redesenhar)
}

/// `DirectSoundCreate` (DSOUND, importado so pelo numero 1) nos testes: sem placa de som
/// (DSERR_NODRIVER). O jogo segue sem som, e o PC de quem usa fica em silencio. `simular.py --som`
/// mantem o som.
unsafe extern "system" fn direct_sound_create(_guid: *const c_void, saida: *mut *mut c_void, _externo: *mut c_void) -> u32 {
    if !saida.is_null() {
        *saida = core::ptr::null_mut();
    }
    0x8878_0078
}

static mut CREATE_MUTEX_ORIGINAL: usize = 0;
static mut NOME_DO_MUTEX: [u8; 64] = [0; 64];

/// O jogo cria o mutex "Championship Manager 2001/02 Process" (0x68d3a0) e, se já existe, abre a caixa "Already
/// Running" e sai. O nome ganha o hash do caminho do exe: o mesmo exe no mesmo caminho continua barrado; outro exe
/// ou outra pasta abre ao lado (ex.: o original e o nosso, o jogo do Lucas e o dos testes).
unsafe extern "system" fn create_mutex_a(atributos: *mut c_void, dono: i32, nome: *const u8) -> *mut c_void {
    let f: unsafe extern "system" fn(*mut c_void, i32, *const u8) -> *mut c_void = core::mem::transmute(CREATE_MUTEX_ORIGINAL);
    const DO_JOGO: &str = "Championship Manager 2001/02 Process";
    if nome.is_null() || cstr(nome as usize) != DO_JOGO {
        return f(atributos, dono, nome);
    }
    let mut caminho = [0u8; 520];
    let n = win::GetModuleFileNameA(core::ptr::null_mut(), caminho.as_mut_ptr(), caminho.len() as u32) as usize;
    let h = caminho[..n].iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b.to_ascii_lowercase() as u64).wrapping_mul(0x100_0000_01b3));
    let novo = format!("{} {:016x}\0", DO_JOGO, h);
    NOME_DO_MUTEX[..novo.len()].copy_from_slice(novo.as_bytes());
    linha(&format!("mutex do jogo: {}", novo.trim_end_matches('\0')));
    f(atributos, dono, core::ptr::addr_of!(NOME_DO_MUTEX) as *const u8)
}

/// Chamado no carregamento da DLL (antes de o jogo criar a janela).
pub unsafe fn instalar_cedo() {
    if let Some(a) = trocar_importacao("KERNEL32.dll", "CreateMutexA", create_mutex_a as *const () as usize) {
        CREATE_MUTEX_ORIGINAL = a;
    }
    // CM_OCULTO=2: oculto com som (o valor tem o mesmo tamanho; uma variavel a mais mudaria a heap)
    if crate::apresentacao::oculto() && win::texto_ambiente("CM_OCULTO") != "2" {
        let n = para_cada_importacao(|dll, nome, _| {
            (dll.eq_ignore_ascii_case("DSOUND.dll") && nome.is_none()).then(|| direct_sound_create as *const () as usize)
        });
        linha(&format!("gancho: DSOUND sem som nos testes ({} entrada)", n));
        if let Some(a) = trocar_importacao("USER32.dll", "MoveWindow", move_window as *const () as usize) {
            MOVE_WINDOW_ORIGINAL = a;
        }
        if let Some(a) = trocar_importacao("USER32.dll", "CreateWindowExA", create_window_ex_a as *const () as usize) {
            CREATE_WINDOW_ORIGINAL = a;
        }
        if let Some(a) = trocar_importacao("USER32.dll", "ShowWindow", show_window as *const () as usize) {
            SHOW_WINDOW_ORIGINAL = a;
        }
    }
}
