//! Extension point: an external DLL next to the exe (`botao.dll`, Lucas's button football game) receives the
//! match engine at every instant of the loop (0x6b98c0), read-only. Without the DLL, nothing happens
//! (one `OnceLock` read per instant). The DLL exports, in C:
//!   `extern "C" fn cm_instante(motor: *const u8)`  called at the start of each instant of the loop;
//!   `extern "C" fn cm_versao() -> u32`             (optional) the contract version it expects.
//! The engine fields (positions, ball, clock, score) are documented in `motor.rs` and in
//! `docs/ROTINAS.md`; the extension may not write to the engine nor call the game (the career has to
//! stay identical with or without it).
//!
//! [pt] Ponto de extensao: uma DLL externa ao lado do exe (`botao.dll`, o jogo de botao do Lucas) recebe o
//! motor de partidas a cada instante do laco (0x6b98c0), so para leitura. Sem a DLL, nada acontece
//! (uma leitura de `OnceLock` por instante). A DLL exporta, em C:
//!   `extern "C" fn cm_instante(motor: *const u8)`  chamada no inicio de cada instante do laco;
//!   `extern "C" fn cm_versao() -> u32`             (opcional) a versao do contrato que ela espera.
//! Os campos do motor (posicoes, bola, relogio, placar) estao documentados em `motor.rs` e em
//! `docs/ROTINAS.md`; a extensao nao pode escrever no motor nem chamar o jogo (a carreira tem de
//! continuar identica com ou sem ela).

use std::sync::OnceLock;

/// Version of the contract with the external DLL (bumped when the call or what it receives changes).
///
/// [pt] Versao do contrato com a DLL externa (sobe quando mudar a chamada ou o que ela recebe).
pub const VERSAO: u32 = 1;

type InstantFn = unsafe extern "C" fn(motor: *const u8);

static CALLBACK: OnceLock<Option<InstantFn>> = OnceLock::new();

/// At the start of each instant of the engine loop.
///
/// [pt] No inicio de cada instante do laco do motor.
pub fn instante(m: usize) {
    if let Some(f) = CALLBACK.get_or_init(|| unsafe { load() }) {
        unsafe { f(m as *const u8) }
    }
}

unsafe fn load() -> Option<InstantFn> {
    let h = crate::win::LoadLibraryA(b"botao.dll\0".as_ptr());
    if h.is_null() {
        return None;
    }
    let p = crate::win::GetProcAddress(h, b"cm_instante\0".as_ptr());
    if p.is_null() {
        crate::registro::linha("extensao: botao.dll sem cm_instante");
        return None;
    }
    let v = crate::win::GetProcAddress(h, b"cm_versao\0".as_ptr());
    if !v.is_null() {
        let version: unsafe extern "C" fn() -> u32 = core::mem::transmute(v);
        if version() != VERSAO {
            crate::registro::linha(&format!("extensao: botao.dll espera a versao {}, o nucleo e a {}", version(), VERSAO));
            return None;
        }
    }
    crate::registro::linha("extensao: botao.dll carregada");
    Some(core::mem::transmute::<*mut core::ffi::c_void, InstantFn>(p))
}
