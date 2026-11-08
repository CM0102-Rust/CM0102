//! Placar do modo comparacao (original x Rust): conta comparacoes e divergencias de uma rotina,
//! registra as primeiras divergencias com detalhes e marca 10, 100, 1000... comparacoes.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::registro::linha;

/// Quantas vezes cada ramo marcado de uma rotina portada rodou (mutacoes.py le da base e pula a
/// mutacao de ramo que nao rodou). No registro a cada potencia de 2: "ramo <nome>: N".
static RAMOS: std::sync::Mutex<Vec<(&'static str, u64)>> = std::sync::Mutex::new(Vec::new());

/// Conta uma passagem pelo ramo `nome` (para rotinas que nao rodam bilhoes de vezes).
pub fn branch(nome: &'static str) {
    let n = {
        let mut r = RAMOS.lock().unwrap_or_else(|e| e.into_inner());
        match r.iter_mut().find(|x| x.0 == nome) {
            Some(x) => {
                x.1 += 1;
                x.1
            }
            None => {
                r.push((nome, 1));
                1
            }
        }
    };
    if n.is_power_of_two() {
        linha(&format!("ramo {}: {}", nome, n));
    }
}

/// Quantas divergencias de cada rotina saem detalhadas no registro.
const DETALHADAS: u64 = 20;

pub struct Scoreboard {
    nome: &'static str,
    total: AtomicU64,
    divergencias: AtomicU64,
    /// o proximo marco (10, 100, 1000...): sem divisao de 64 bits a cada comparacao
    marco: AtomicU64,
}

impl Scoreboard {
    pub const fn new(nome: &'static str) -> Scoreboard {
        Scoreboard { nome, total: AtomicU64::new(0), divergencias: AtomicU64::new(0), marco: AtomicU64::new(10) }
    }

    /// Registra uma comparacao. `detalhe` so e montado se for uma das primeiras divergencias.
    pub fn record(&self, igual: bool, detalhe: impl FnOnce() -> String) {
        let total = self.total.fetch_add(1, Ordering::Relaxed) + 1;
        if !igual {
            let d = self.divergencias.fetch_add(1, Ordering::Relaxed) + 1;
            if d <= DETALHADAS {
                linha(&format!("{}: DIVERGENCIA {}", self.nome, detalhe()));
            }
        }
        if total == self.marco.load(Ordering::Relaxed) {
            self.marco.store(total * 10, Ordering::Relaxed);
            linha(&format!("{}: {} comparacoes, {} divergencias", self.nome, total, self.divergencias.load(Ordering::Relaxed)));
        }
    }

    pub fn counts(&self) -> (u64, u64) {
        (self.total.load(Ordering::Relaxed), self.divergencias.load(Ordering::Relaxed))
    }

    /// Roda `f` (o autoteste) e registra quantos casos e divergencias ele somou.
    pub fn autoteste(&self, f: impl FnOnce()) {
        let (t0, d0) = self.counts();
        f();
        let (t1, d1) = self.counts();
        linha(&format!("{}: AUTOTESTE {} casos, {} divergencias", self.nome, t1 - t0, d1 - d0));
    }
}

/// Copia de regioes de memoria (endereco, tamanho).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemorySnapshot(pub Vec<(usize, Vec<u8>)>);

/// Buffers de fotos devolvidos (as fotos do motor inteiro tem 63 KB e eram alocadas e liberadas
/// a cada comparacao: a maior parte do tempo do modo comparacao).
/// Buffers devolvidos pelas fotos (reaproveitados em vez de alocar e liberar a cada comparacao).
static SOBRAS: std::sync::Mutex<Vec<Vec<u8>>> = std::sync::Mutex::new(Vec::new());

fn buffer(n: usize) -> Vec<u8> {
    let mut sobras = SOBRAS.lock().unwrap_or_else(|e| e.into_inner());
    match sobras.iter().rposition(|v| v.capacity() >= n && v.capacity() <= 2 * n + 64) {
        Some(i) => {
            let mut v = sobras.swap_remove(i);
            v.clear();
            v
        }
        None => Vec::with_capacity(n),
    }
}

impl Drop for MemorySnapshot {
    fn drop(&mut self) {
        let mut sobras = SOBRAS.lock().unwrap_or_else(|e| e.into_inner());
        for (_, v) in self.0.drain(..) {
            if sobras.len() < 256 && v.capacity() > 0 {
                sobras.push(v);
            }
        }
    }
}

impl MemorySnapshot {
    pub unsafe fn read(regioes: &[(usize, usize)]) -> MemorySnapshot {
        MemorySnapshot(regioes.iter().map(|&(a, n)| {
            let mut v = buffer(n);
            v.extend_from_slice(core::slice::from_raw_parts(a as *const u8, n));
            (a, v)
        }).collect())
    }
    pub unsafe fn write_back(&self) {
        for (a, b) in &self.0 {
            core::ptr::copy_nonoverlapping(b.as_ptr(), *a as *mut u8, b.len());
        }
    }
}

/// Roda a original e a versao Rust a partir do mesmo estado das `regioes` e devolve
/// (resultado original, resultado Rust, memoria igual?). Ao sair, a memoria fica como a original deixou.
pub unsafe fn side_by_side<R>(regioes: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> (R, R, bool, MemorySnapshot, MemorySnapshot) {
    let antes = MemorySnapshot::read(regioes);
    let r_orig = original();
    let depois_orig = MemorySnapshot::read(regioes);
    antes.write_back();
    // os sorteios da copia nao entram no rastro de uma comparacao pela fita que esteja gravando a
    // original (ela chama esta rotina pelo desvio): o rastro da original so tem os dela
    let r_rust = crate::rng::pause_trace(rust);
    let depois_rust = MemorySnapshot::read(regioes);
    depois_orig.write_back();
    let igual = depois_orig == depois_rust;
    (r_orig, r_rust, igual, depois_orig, depois_rust)
}

/// Como `lado_a_lado`, sem alocar quando as regioes somam ate 256 bytes: as copias ficam na pilha e
/// so viram `Memoria` (para a mensagem) se a memoria divergir; iguais, voltam vazias. Para as rotinas
/// que o jogo chama bilhoes de vezes (datas), em que as copias alocadas tomavam a maior parte do tempo.
pub unsafe fn side_by_side_light<R>(regioes: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> (R, R, bool, MemorySnapshot, MemorySnapshot) {
    const MAX: usize = 256;
    let total: usize = regioes.iter().map(|r| r.1).sum();
    if total > MAX {
        return side_by_side(regioes, original, rust);
    }
    unsafe fn copiar(regioes: &[(usize, usize)], buf: &mut [u8; MAX]) {
        let mut k = 0;
        for &(a, n) in regioes {
            core::ptr::copy_nonoverlapping(a as *const u8, buf.as_mut_ptr().add(k), n);
            k += n;
        }
    }
    unsafe fn write_back(regioes: &[(usize, usize)], buf: &[u8; MAX]) {
        let mut k = 0;
        for &(a, n) in regioes {
            core::ptr::copy_nonoverlapping(buf.as_ptr().add(k), a as *mut u8, n);
            k += n;
        }
    }
    let (mut antes, mut d_orig, mut d_rust) = ([0u8; MAX], [0u8; MAX], [0u8; MAX]);
    copiar(regioes, &mut antes);
    let r_orig = original();
    copiar(regioes, &mut d_orig);
    write_back(regioes, &antes);
    let r_rust = crate::rng::pause_trace(rust);
    copiar(regioes, &mut d_rust);
    write_back(regioes, &d_orig);
    let igual = d_orig[..total] == d_rust[..total];
    if igual {
        return (r_orig, r_rust, true, MemorySnapshot(Vec::new()), MemorySnapshot(Vec::new()));
    }
    let memoria = |buf: &[u8; MAX]| {
        let mut k = 0;
        MemorySnapshot(regioes.iter().map(|&(a, n)| {
            k += n;
            (a, buf[k - n..k].to_vec())
        }).collect())
    };
    (r_orig, r_rust, false, memoria(&d_orig), memoria(&d_rust))
}

/// Gerador simples para os autotestes (xorshift64); nao tem relacao com o sorteio do jogo.
pub struct TestRng(pub u64);

impl TestRng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
