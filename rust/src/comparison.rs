//! Comparison-mode scoreboard (original x Rust): counts comparisons and divergences of a routine,
//! logs the first divergences in detail and marks 10, 100, 1000... comparisons.
//!
//! [pt] Placar do modo comparacao (original x Rust): conta comparacoes e divergencias de uma rotina,
//! registra as primeiras divergencias com detalhes e marca 10, 100, 1000... comparacoes.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::registro::linha;

/// How many times each marked branch of a ported routine ran (mutacoes.py reads it from the base and skips the
/// mutation of a branch that did not run). In the log at each power of 2: "ramo <name>: N".
///
/// [pt] Quantas vezes cada ramo marcado de uma rotina portada rodou (mutacoes.py le da base e pula a
/// mutacao de ramo que nao rodou). No registro a cada potencia de 2: "ramo <nome>: N".
static BRANCHES: std::sync::Mutex<Vec<(&'static str, u64)>> = std::sync::Mutex::new(Vec::new());

/// Counts one pass through branch `name` (for routines that do not run billions of times).
///
/// [pt] Conta uma passagem pelo ramo `nome` (para rotinas que nao rodam bilhoes de vezes).
pub fn branch(name: &'static str) {
    let n = {
        let mut r = BRANCHES.lock().unwrap_or_else(|e| e.into_inner());
        match r.iter_mut().find(|x| x.0 == name) {
            Some(x) => {
                x.1 += 1;
                x.1
            }
            None => {
                r.push((name, 1));
                1
            }
        }
    };
    if n.is_power_of_two() {
        linha(&format!("ramo {}: {}", name, n));
    }
}

/// How many divergences of each routine are logged in detail.
///
/// [pt] Quantas divergencias de cada rotina saem detalhadas no registro.
const DETAILED: u64 = 20;

pub struct Scoreboard {
    name: &'static str,
    total: AtomicU64,
    divergences: AtomicU64,
    /// the next milestone (10, 100, 1000...): no 64-bit division on every comparison
    ///
    /// [pt] o proximo marco (10, 100, 1000...): sem divisao de 64 bits a cada comparacao
    milestone: AtomicU64,
}

impl Scoreboard {
    pub const fn new(name: &'static str) -> Scoreboard {
        Scoreboard { name, total: AtomicU64::new(0), divergences: AtomicU64::new(0), milestone: AtomicU64::new(10) }
    }

    /// Records one comparison. `detail` is only built if it is one of the first divergences.
    ///
    /// [pt] Registra uma comparacao. `detalhe` so e montado se for uma das primeiras divergencias.
    pub fn record(&self, equal: bool, detail: impl FnOnce() -> String) {
        let total = self.total.fetch_add(1, Ordering::Relaxed) + 1;
        if !equal {
            let d = self.divergences.fetch_add(1, Ordering::Relaxed) + 1;
            if d <= DETAILED {
                linha(&format!("{}: DIVERGENCIA {}", self.name, detail()));
            }
        }
        if total == self.milestone.load(Ordering::Relaxed) {
            self.milestone.store(total * 10, Ordering::Relaxed);
            linha(&format!("{}: {} comparacoes, {} divergencias", self.name, total, self.divergences.load(Ordering::Relaxed)));
        }
    }

    pub fn counts(&self) -> (u64, u64) {
        (self.total.load(Ordering::Relaxed), self.divergences.load(Ordering::Relaxed))
    }

    /// Runs `f` (the self-test) and logs how many cases and divergences it added.
    ///
    /// [pt] Roda `f` (o autoteste) e registra quantos casos e divergencias ele somou.
    pub fn autoteste(&self, f: impl FnOnce()) {
        let (t0, d0) = self.counts();
        f();
        let (t1, d1) = self.counts();
        linha(&format!("{}: AUTOTESTE {} casos, {} divergencias", self.name, t1 - t0, d1 - d0));
    }
}

/// Copy of memory regions (address, size).
///
/// [pt] Copia de regioes de memoria (endereco, tamanho).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemorySnapshot(pub Vec<(usize, Vec<u8>)>);

/// Returned snapshot buffers (snapshots of the whole engine are 63 KB and used to be allocated and freed
/// on every comparison: most of the comparison mode's time).
/// Buffers returned by the snapshots (reused instead of allocating and freeing on every comparison).
///
/// [pt] Buffers de fotos devolvidos (as fotos do motor inteiro tem 63 KB e eram alocadas e liberadas
/// a cada comparacao: a maior parte do tempo do modo comparacao).
/// Buffers devolvidos pelas fotos (reaproveitados em vez de alocar e liberar a cada comparacao).
static SPARE: std::sync::Mutex<Vec<Vec<u8>>> = std::sync::Mutex::new(Vec::new());

fn buffer(n: usize) -> Vec<u8> {
    let mut spare = SPARE.lock().unwrap_or_else(|e| e.into_inner());
    match spare.iter().rposition(|v| v.capacity() >= n && v.capacity() <= 2 * n + 64) {
        Some(i) => {
            let mut v = spare.swap_remove(i);
            v.clear();
            v
        }
        None => Vec::with_capacity(n),
    }
}

impl Drop for MemorySnapshot {
    fn drop(&mut self) {
        let mut spare = SPARE.lock().unwrap_or_else(|e| e.into_inner());
        for (_, v) in self.0.drain(..) {
            if spare.len() < 256 && v.capacity() > 0 {
                spare.push(v);
            }
        }
    }
}

impl MemorySnapshot {
    pub unsafe fn read(regions: &[(usize, usize)]) -> MemorySnapshot {
        MemorySnapshot(regions.iter().map(|&(a, n)| {
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

/// Runs the original and the Rust version from the same state of the `regions` and returns
/// (original result, Rust result, memory equal?). On exit, memory is as the original left it.
///
/// [pt] Roda a original e a versao Rust a partir do mesmo estado das `regioes` e devolve
/// (resultado original, resultado Rust, memoria igual?). Ao sair, a memoria fica como a original deixou.
pub unsafe fn side_by_side<R>(regions: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> (R, R, bool, MemorySnapshot, MemorySnapshot) {
    let before = MemorySnapshot::read(regions);
    let r_orig = original();
    let after_orig = MemorySnapshot::read(regions);
    before.write_back();
    // the copy's draws do not enter the trace of a tape comparison that is recording the
    // original (it calls this routine through the detour): the original's trace only has its own
    let r_rust = crate::rng::pause_trace(rust);
    let after_rust = MemorySnapshot::read(regions);
    after_orig.write_back();
    let equal = after_orig == after_rust;
    (r_orig, r_rust, equal, after_orig, after_rust)
}

/// Like `side_by_side`, without allocating when the regions add up to at most 256 bytes: the copies stay on the stack and
/// only become `MemorySnapshot` (for the message) if memory diverges; when equal, they come back empty. For the routines
/// the game calls billions of times (dates), where the allocated copies took most of the time.
///
/// [pt] Como `lado_a_lado`, sem alocar quando as regioes somam ate 256 bytes: as copias ficam na pilha e
/// so viram `Memoria` (para a mensagem) se a memoria divergir; iguais, voltam vazias. Para as rotinas
/// que o jogo chama bilhoes de vezes (datas), em que as copias alocadas tomavam a maior parte do tempo.
pub unsafe fn side_by_side_light<R>(regions: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> (R, R, bool, MemorySnapshot, MemorySnapshot) {
    const MAX: usize = 256;
    let total: usize = regions.iter().map(|r| r.1).sum();
    if total > MAX {
        return side_by_side(regions, original, rust);
    }
    unsafe fn copy_in(regions: &[(usize, usize)], buf: &mut [u8; MAX]) {
        let mut k = 0;
        for &(a, n) in regions {
            core::ptr::copy_nonoverlapping(a as *const u8, buf.as_mut_ptr().add(k), n);
            k += n;
        }
    }
    unsafe fn write_back(regions: &[(usize, usize)], buf: &[u8; MAX]) {
        let mut k = 0;
        for &(a, n) in regions {
            core::ptr::copy_nonoverlapping(buf.as_ptr().add(k), a as *mut u8, n);
            k += n;
        }
    }
    let (mut before, mut d_orig, mut d_rust) = ([0u8; MAX], [0u8; MAX], [0u8; MAX]);
    copy_in(regions, &mut before);
    let r_orig = original();
    copy_in(regions, &mut d_orig);
    write_back(regions, &before);
    let r_rust = crate::rng::pause_trace(rust);
    copy_in(regions, &mut d_rust);
    write_back(regions, &d_orig);
    let equal = d_orig[..total] == d_rust[..total];
    if equal {
        return (r_orig, r_rust, true, MemorySnapshot(Vec::new()), MemorySnapshot(Vec::new()));
    }
    let snapshot = |buf: &[u8; MAX]| {
        let mut k = 0;
        MemorySnapshot(regions.iter().map(|&(a, n)| {
            k += n;
            (a, buf[k - n..k].to_vec())
        }).collect())
    };
    (r_orig, r_rust, false, snapshot(&d_orig), snapshot(&d_rust))
}

/// Simple generator for the self-tests (xorshift64); unrelated to the game's draw.
///
/// [pt] Gerador simples para os autotestes (xorshift64); nao tem relacao com o sorteio do jogo.
pub struct TestRng(pub u64);

impl TestRng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
