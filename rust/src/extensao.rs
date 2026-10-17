//! Ponto de extensao: uma DLL externa ao lado do exe (`botao.dll`, o jogo de botao do Lucas) recebe o
//! motor de partidas a cada instante do laco (0x6b98c0), so para leitura. Sem a DLL, nada acontece
//! (uma leitura de `OnceLock` por instante). A DLL exporta, em C:
//!   `extern "C" fn cm_instante(motor: *const u8)`  chamada no inicio de cada instante do laco;
//!   `extern "C" fn cm_versao() -> u32`             (opcional) a versao do contrato que ela espera.
//! Os campos do motor (posicoes, bola, relogio, placar) estao documentados em `motor.rs` e em
//! `docs/ROTINAS.md`; a extensao nao pode escrever no motor nem chamar o jogo (a carreira tem de
//! continuar identica com ou sem ela).

use std::sync::OnceLock;

/// Versao do contrato com a DLL externa (sobe quando mudar a chamada ou o que ela recebe).
pub const VERSAO: u32 = 1;

type Instante = unsafe extern "C" fn(motor: *const u8);

static FUNCAO: OnceLock<Option<Instante>> = OnceLock::new();

/// No inicio de cada instante do laco do motor.
pub fn instante(m: usize) {
    if let Some(f) = FUNCAO.get_or_init(|| unsafe { carregar() }) {
        unsafe { f(m as *const u8) }
    }
}

unsafe fn carregar() -> Option<Instante> {
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
        let versao: unsafe extern "C" fn() -> u32 = core::mem::transmute(v);
        if versao() != VERSAO {
            crate::registro::linha(&format!("extensao: botao.dll espera a versao {}, o nucleo e a {}", versao(), VERSAO));
            return None;
        }
    }
    crate::registro::linha("extensao: botao.dll carregada");
    Some(core::mem::transmute::<*mut core::ffi::c_void, Instante>(p))
}
