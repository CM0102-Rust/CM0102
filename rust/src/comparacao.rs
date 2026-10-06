//! Placar do modo comparacao (original x Rust): conta comparacoes e divergencias de uma rotina,
//! registra as primeiras divergencias com detalhes e marca 10, 100, 1000... comparacoes.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::registro::linha;

/// Quantas divergencias de cada rotina saem detalhadas no registro.
const DETALHADAS: u64 = 20;

pub struct Placar {
    nome: &'static str,
    total: AtomicU64,
    divergencias: AtomicU64,
}

impl Placar {
    pub const fn novo(nome: &'static str) -> Placar {
        Placar { nome, total: AtomicU64::new(0), divergencias: AtomicU64::new(0) }
    }

    /// Registra uma comparacao. `detalhe` so e montado se for uma das primeiras divergencias.
    pub fn registrar(&self, igual: bool, detalhe: impl FnOnce() -> String) {
        let total = self.total.fetch_add(1, Ordering::Relaxed) + 1;
        if !igual {
            let d = self.divergencias.fetch_add(1, Ordering::Relaxed) + 1;
            if d <= DETALHADAS {
                linha(&format!("{}: DIVERGENCIA {}", self.nome, detalhe()));
            }
        }
        if total >= 10 && 10u64.pow(total.ilog10()) == total {
            linha(&format!("{}: {} comparacoes, {} divergencias", self.nome, total, self.divergencias.load(Ordering::Relaxed)));
        }
    }

    pub fn contagem(&self) -> (u64, u64) {
        (self.total.load(Ordering::Relaxed), self.divergencias.load(Ordering::Relaxed))
    }

    /// Roda `f` (o autoteste) e registra quantos casos e divergencias ele somou.
    pub fn autoteste(&self, f: impl FnOnce()) {
        let (t0, d0) = self.contagem();
        f();
        let (t1, d1) = self.contagem();
        linha(&format!("{}: AUTOTESTE {} casos, {} divergencias", self.nome, t1 - t0, d1 - d0));
    }
}

/// Copia de regioes de memoria (endereco, tamanho).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Memoria(pub Vec<(usize, Vec<u8>)>);

impl Memoria {
    pub unsafe fn ler(regioes: &[(usize, usize)]) -> Memoria {
        Memoria(regioes.iter().map(|&(a, n)| (a, core::slice::from_raw_parts(a as *const u8, n).to_vec())).collect())
    }
    pub unsafe fn escrever(&self) {
        for (a, b) in &self.0 {
            core::ptr::copy_nonoverlapping(b.as_ptr(), *a as *mut u8, b.len());
        }
    }
}

/// Roda a original e a versao Rust a partir do mesmo estado das `regioes` e devolve
/// (resultado original, resultado Rust, memoria igual?). Ao sair, a memoria fica como a original deixou.
pub unsafe fn lado_a_lado<R>(regioes: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> (R, R, bool, Memoria, Memoria) {
    let antes = Memoria::ler(regioes);
    let r_orig = original();
    let depois_orig = Memoria::ler(regioes);
    antes.escrever();
    let r_rust = rust();
    let depois_rust = Memoria::ler(regioes);
    depois_orig.escrever();
    let igual = depois_orig == depois_rust;
    (r_orig, r_rust, igual, depois_orig, depois_rust)
}

/// Gerador simples para os autotestes (xorshift64); nao tem relacao com o sorteio do jogo.
pub struct Sorteio(pub u64);

impl Sorteio {
    pub fn proximo(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
