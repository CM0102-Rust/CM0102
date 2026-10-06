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

/// Chamado no carregamento da DLL (antes de o jogo criar a janela).
pub unsafe fn instalar_cedo() {
    if crate::apresentacao::oculto() {
        if let Some(a) = trocar_importacao("USER32.dll", "ShowWindow", show_window as *const () as usize) {
            SHOW_WINDOW_ORIGINAL = a;
        }
    }
}
