//! Desvios em rotinas do jogo: o inicio da rotina original vira um `jmp` para a versao em Rust.
//!
//! Os bytes deslocados vao para um trampolim (memoria executavel nossa) seguido de um `jmp` de
//! volta para o resto da rotina; chamar o trampolim e chamar a rotina original.
//! A tabela de prologos vem de `engenharia-reversa/desvios.py`, que confere que os bytes copiados
//! nao tem instrucao relativa nem salto do jogo para o meio deles. Antes de escrever, o nucleo
//! confere os bytes no exe carregado e recusa o desvio se nao baterem (exe diferente do esperado).
//!
//! Variaveis de ambiente: `CM_SEM_DESVIOS=1` nao instala nenhum; `CM_COMPARAR=1` roda a original e a
//! versao Rust lado a lado e registra divergencias (o jogo segue com o resultado da original).

use core::ffi::c_void;

use crate::registro::linha;
use crate::{aleatorio, data, win};

pub struct Prologo {
    pub endereco: usize,
    pub bytes: &'static [u8],
}

/// Desvia `p.endereco` para `destino`. Devolve o trampolim (a rotina original), ou None se recusou.
pub unsafe fn desviar(nome: &str, p: &Prologo, destino: usize) -> Option<usize> {
    let atual = core::slice::from_raw_parts(p.endereco as *const u8, p.bytes.len());
    if atual != p.bytes {
        linha(&format!("desvio {} recusado: bytes em {:#x} sao {:02x?}, esperava {:02x?}", nome, p.endereco, atual, p.bytes));
        return None;
    }
    let n = p.bytes.len();
    let tramp = win::VirtualAlloc(core::ptr::null_mut(), 32, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as *mut u8;
    if tramp.is_null() {
        linha(&format!("desvio {} recusado: VirtualAlloc falhou", nome));
        return None;
    }
    core::ptr::copy_nonoverlapping(p.bytes.as_ptr(), tramp, n);
    escrever_jmp(tramp.add(n), p.endereco + n);

    let mut antiga = 0u32;
    win::VirtualProtect(p.endereco as *mut c_void, n, win::PAGE_EXECUTE_READWRITE, &mut antiga);
    escrever_jmp(p.endereco as *mut u8, destino);
    for i in 5..n {
        *(p.endereco as *mut u8).add(i) = 0xcc; // nunca executados (o jmp sai antes)
    }
    win::VirtualProtect(p.endereco as *mut c_void, n, antiga, &mut antiga);
    win::FlushInstructionCache(win::GetCurrentProcess(), p.endereco as *const c_void, n);
    linha(&format!("desvio {}: {:#x} -> {:#x} (original em {:#x})", nome, p.endereco, destino, tramp as usize));
    Some(tramp as usize)
}

unsafe fn escrever_jmp(onde: *mut u8, para: usize) {
    *onde = 0xe9;
    let rel = (para as isize - (onde as isize + 5)) as i32;
    core::ptr::write_unaligned(onde.add(1) as *mut i32, rel);
}

/// Instala todos os desvios. Chamado no carregamento da DLL, antes do WinMain do jogo.
pub unsafe fn instalar() {
    if win::ambiente("CM_SEM_DESVIOS") {
        linha("desvios: desligados (CM_SEM_DESVIOS)");
        return;
    }
    // Simulacao em lote (CM_PARAR_EM): relogio fixo, para a simulacao ser reproduzivel.
    crate::simulacao::instalar();
    // CM_ORIGINAL=1: nenhuma rotina substituida; o sorteio so e observado (contagem e parada da
    // simulacao), para comparar uma simulacao so com as originais contra outra com o Rust.
    if win::ambiente("CM_ORIGINAL") {
        aleatorio::instalar(aleatorio::Modo::Observar);
        return;
    }
    // CM_AUTOTESTE=1 liga o modo comparacao; os autotestes em si rodam depois (ver `autotestes`).
    let comparar = win::ambiente("CM_COMPARAR") || win::ambiente("CM_AUTOTESTE");
    aleatorio::instalar(if comparar { aleatorio::Modo::Comparar } else { aleatorio::Modo::Rust });
    data::instalar(comparar);
}

/// CM_AUTOTESTE=1: cada rotina reescrita se compara com a original em entradas sinteticas
/// (resultado em ddraw.log). Chamado na primeira DirectDrawCreate: a essa altura o WinMain ja
/// esta rodando e a CRT do jogo esta inicializada (algumas rotinas chamam atexit, malloc...).
pub unsafe fn autotestes() {
    static FEITO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !win::ambiente("CM_AUTOTESTE") || FEITO.swap(true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    aleatorio::autoteste(5_000_000);
    data::autoteste();
}
