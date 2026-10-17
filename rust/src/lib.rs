//! Nucleo em Rust do CM 01/02.
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

mod alocador;

#[global_allocator]
static GLOBAL: alocador::Global = alocador::Global;
mod aleatorio;
mod apresentacao;
mod comparacao;
mod data;
mod desvios;
mod espiao;
mod extensao;
mod ganchos;
mod jogo;
mod motor;
mod reducao;
mod registro;
mod simulacao;
mod video;
mod win;

use core::ffi::c_void;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryW(nome: *const u16) -> *mut c_void;
    fn GetProcAddress(modulo: *mut c_void, nome: *const u8) -> *mut c_void;
    fn GetSystemDirectoryW(buf: *mut u16, tam: u32) -> u32;
}

type FnDirectDrawCreate = unsafe extern "system" fn(*mut c_void, *mut *mut c_void, *mut c_void) -> i32;

const DDERR_GENERIC: i32 = 0x8000_4005_u32 as i32;
const DDERR_INVALIDPARAMS: i32 = 0x8007_0057_u32 as i32;

/// Carrega a ddraw.dll verdadeira pelo caminho completo (pelo nome, o Windows devolveria esta).
unsafe fn ddraw_do_sistema() -> Option<FnDirectDrawCreate> {
    let mut buf = [0u16; 260];
    let n = GetSystemDirectoryW(buf.as_mut_ptr(), buf.len() as u32) as usize;
    if n == 0 || n > 240 {
        return None;
    }
    let mut caminho = buf[..n].to_vec();
    caminho.extend("\\ddraw.dll\0".encode_utf16());
    let modulo = LoadLibraryW(caminho.as_ptr());
    if modulo.is_null() {
        return None;
    }
    let f = GetProcAddress(modulo, b"DirectDrawCreate\0".as_ptr());
    (!f.is_null()).then(|| core::mem::transmute::<*mut c_void, FnDirectDrawCreate>(f))
}

/// Carregamento da DLL: o Windows carrega a ddraw.dll junto com o exe, antes do WinMain do jogo.
/// E aqui que os desvios nas rotinas do jogo sao instalados.
#[no_mangle]
pub unsafe extern "system" fn DllMain(_modulo: *mut c_void, motivo: u32, _reservado: *mut c_void) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;
    if motivo == DLL_PROCESS_ATTACH && !win::ambiente("CM_ESPIAO") {
        ganchos::instalar_cedo();
        desvios::instalar();
    }
    1
}

#[no_mangle]
pub unsafe extern "system" fn DirectDrawCreate(guid: *mut c_void, saida: *mut *mut c_void, externo: *mut c_void) -> i32 {
    if saida.is_null() {
        return DDERR_INVALIDPARAMS;
    }
    if win::ambiente("CM_ESPIAO") {
        let Some(real) = ddraw_do_sistema() else {
            registro::linha("ERRO: nao consegui carregar a ddraw.dll do sistema");
            return DDERR_GENERIC;
        };
        let r = real(guid, saida, externo);
        registro::linha(&format!("espiao: DirectDrawCreate(guid={:p}) -> {:#x}", guid, r));
        if r == 0 {
            *saida = espiao::embrulhar(*saida, espiao::Tipo::DirectDraw);
        }
        return r;
    }
    ganchos::instalar();
    *saida = video::criar();
    desvios::autotestes();
    0
}
