//! Rust core of CM 01/02.
//!
//! cm0102.exe imports `DirectDrawCreate` from ddraw.dll, and Windows looks for the DLL first in the
//! exe's folder. This library is that ddraw.dll: it is how our code enters the game, without
//! changing the executable.
//!
//! - Default: our own video layer (`video`), without changing the Windows mode; detours in the exe (`ganchos`).
//! - With the environment variable `CM_ESPIAO=1`: forwards everything to the system ddraw.dll and logs each
//!   call (`espiao`), for diagnosis.
//!
//! The log goes to `ddraw.log`, next to the exe.
//!
//! [pt] Nucleo em Rust do CM 01/02.
//!
//! O cm0102.exe importa `DirectDrawCreate` da ddraw.dll, e o Windows procura a DLL primeiro na
//! pasta do exe. Esta biblioteca e essa ddraw.dll: e por ela que o nosso codigo entra no jogo,
//! sem alterar o executavel.
//!
//! - Padrao: camada de video propria (`video`), sem trocar o modo do Windows; desvios no exe (`ganchos`).
//! - Com a variavel de ambiente `CM_ESPIAO=1`: repassa tudo a ddraw.dll do sistema e registra cada
//!   chamada (`espiao`), para diagnostico.
//!
//! O registro sai em `ddraw.log`, ao lado do exe.

#![allow(linker_messages)]

mod allocator;
// the shared base (cm0102-base): same paths as before (crate::video, crate::registro...)
use cm0102_base::{apresentacao, conteiner, registro, video, win};
mod signatures;

#[global_allocator]
static GLOBAL: allocator::Global = allocator::Global;
mod rng;
mod comparison;
mod date;
mod detours;
mod spy;
mod extension;
mod iat_hooks;
mod crt;
mod motor;
mod resolution;
mod simulation;
mod stack_probe;

use core::ffi::c_void;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryW(name: *const u16) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn GetSystemDirectoryW(buf: *mut u16, size: u32) -> u32;
}

type FnDirectDrawCreate = unsafe extern "system" fn(*mut c_void, *mut *mut c_void, *mut c_void) -> i32;

const DDERR_GENERIC: i32 = 0x8000_4005_u32 as i32;
const DDERR_INVALIDPARAMS: i32 = 0x8007_0057_u32 as i32;

/// Loads the real ddraw.dll by its full path (by name, Windows would return this one).
///
/// [pt] Carrega a ddraw.dll verdadeira pelo caminho completo (pelo nome, o Windows devolveria esta).
unsafe fn system_ddraw_create() -> Option<FnDirectDrawCreate> {
    let mut buf = [0u16; 260];
    let n = GetSystemDirectoryW(buf.as_mut_ptr(), buf.len() as u32) as usize;
    if n == 0 || n > 240 {
        return None;
    }
    let mut path = buf[..n].to_vec();
    path.extend("\\ddraw.dll\0".encode_utf16());
    let module = LoadLibraryW(path.as_ptr());
    if module.is_null() {
        return None;
    }
    let f = GetProcAddress(module, b"DirectDrawCreate\0".as_ptr());
    (!f.is_null()).then(|| core::mem::transmute::<*mut c_void, FnDirectDrawCreate>(f))
}

/// DLL load: Windows loads ddraw.dll together with the exe, before the game's WinMain.
/// This is where the detours in the game's routines are installed.
///
/// [pt] Carregamento da DLL: o Windows carrega a ddraw.dll junto com o exe, antes do WinMain do jogo.
/// E aqui que os desvios nas rotinas do jogo sao instalados.
#[no_mangle]
pub unsafe extern "system" fn DllMain(_module: *mut c_void, reason: u32, _reserved: *mut c_void) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;
    if reason == DLL_PROCESS_ATTACH {
        cm0102_base::ganchos::definir_ramo(comparison::branch);
        cm0102_base::ganchos::definir_visivel(allocator::entrar_visivel, allocator::sair_visivel);
        detours::aplicar_resolucao(); // CM_RESOLUCAO=1080 (experiment): before everything
    }
    if reason == DLL_PROCESS_ATTACH && !win::ambiente("CM_ESPIAO") {
        iat_hooks::instalar_cedo();
        detours::instalar();
    }
    1
}

#[no_mangle]
pub unsafe extern "system" fn DirectDrawCreate(guid: *mut c_void, out: *mut *mut c_void, outer: *mut c_void) -> i32 {
    if out.is_null() {
        return DDERR_INVALIDPARAMS;
    }
    if win::ambiente("CM_ESPIAO") {
        let Some(real) = system_ddraw_create() else {
            registro::linha("ERRO: nao consegui carregar a ddraw.dll do sistema");
            return DDERR_GENERIC;
        };
        let r = real(guid, out, outer);
        registro::linha(&format!("espiao: DirectDrawCreate(guid={:p}) -> {:#x}", guid, r));
        if r == 0 {
            *out = spy::embrulhar(*out, spy::Tipo::DirectDraw);
        }
        return r;
    }
    detours::instalar_adiado(); // exe with third-party patch: after the whole game load
    iat_hooks::instalar();
    *out = video::criar();
    detours::autotestes();
    0
}
