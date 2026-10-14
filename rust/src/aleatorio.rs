//! Gerador de numeros aleatorios do jogo: `aleatorio(n)` { 0x90cfc0 }, cdecl, chamado de milhares
//! de pontos (partidas, regens, transferencias, lesoes...).
//!
//! Como funciona (listagem original):
//! - ha uma tabela fixa de numeros no exe (0xaa2454..=0xad4130, dwords); `[0xde1f60]` aponta para a
//!   entrada atual e `[0xde1f5c]` guarda um deslocamento de 16 bits;
//! - cada chamada avanca o ponteiro uma entrada; passando do fim (`[0xde2798]`), volta ao inicio e
//!   sorteia um deslocamento novo com o `rand()` da CRT (0x946a04) & 0xffff;
//! - |n| <= 0xffff: (deslocamento + entrada) % n (divisao com sinal; n negativo inverte o sinal);
//! - |n| > 0xffff: parte em ponto flutuante ((s % 0xffff) * (n * 1/65535), truncado) mais uma
//!   chamada recursiva com n*10/65535 para os digitos de baixo, limitado a n-1.
//! A semente (0x90d0a0, opcao `-seed`) faz srand e escolhe o ponto inicial da tabela.

use core::cell::Cell;

use crate::comparacao::{Placar, Sorteio};
use crate::desvios::{self, Prologo};
use crate::registro::linha;
use crate::simulacao;

const PONTEIRO: *mut u32 = 0xde1f60 as *mut u32;
const DESLOCAMENTO: *mut i32 = 0xde1f5c as *mut i32;
const FIM_DA_TABELA: *const u32 = 0xde2798 as *const u32;
const INICIO_DA_TABELA: u32 = 0xaa2454;
/// Estado do rand() da CRT (versao de uma thread so do Visual C++ 6).
const ESTADO_RAND: *mut u32 = 0xad6c30 as *mut u32;
/// 1/65535, double em 0x970a70.
const UM_SOBRE_65535: f64 = 1.0 / 65535.0;

// 0x90cfc0: push ecx; push esi; mov esi, dword ptr [esp + 0xc]
const PROLOGO: Prologo = Prologo { endereco: 0x90cfc0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] };

static mut ORIGINAL: usize = 0;

/// rand() da CRT do proprio jogo (mesmo estado que o jogo usa).
unsafe fn rand_crt() -> i32 {
    let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x946a04usize);
    f()
}

thread_local! {
    /// Desligado durante a replica Rust do modo comparacao, para a contagem de sorteios da
    /// simulacao ser a mesma nos tres modos (entradas na rotina, recursao incluida).
    static CONTAR: Cell<bool> = const { Cell::new(true) };
}

/// Roda `f` sem contar os sorteios dela (replica Rust de rotinas que sorteiam, no modo comparacao).
pub fn sem_contar<R>(f: impl FnOnce() -> R) -> R {
    let antes = CONTAR.with(|c| c.replace(false));
    let r = f();
    CONTAR.with(|c| c.set(antes));
    r
}

/// Estado do gerador (deslocamento e ponteiro da tabela, rand da CRT): regioes para `lado_a_lado`.
pub const REGIOES_DO_ESTADO: [(usize, usize); 2] = [(0xde1f5c, 8), (0xad6c30, 4)];

unsafe fn contar(n: i32) {
    if CONTAR.with(|c| c.get()) {
        simulacao::sorteio(n, *PONTEIRO, *DESLOCAMENTO, *ESTADO_RAND);
    }
}

/// Versao Rust de { 0x90cfc0 }.
pub unsafe extern "C" fn aleatorio(n: i32) -> i32 {
    contar(n);
    if n == 0 {
        return 0;
    }
    let mut p = (*PONTEIRO).wrapping_add(4);
    *PONTEIRO = p;
    let desloc = if p > *FIM_DA_TABELA {
        *PONTEIRO = INICIO_DA_TABELA;
        p = INICIO_DA_TABELA;
        let d = rand_crt() & 0xffff;
        *DESLOCAMENTO = d;
        d
    } else {
        *DESLOCAMENTO
    };
    let entrada = *(p as *const i32);
    if (-0xffff..=0xffff).contains(&n) {
        let r = desloc.wrapping_add(entrada).wrapping_rem(n);
        return if n > 0 { r } else { r.wrapping_neg() };
    }
    // n fora de 16 bits: parte alta em ponto flutuante (x87 em precisao dupla, como o Windows
    // configura), truncada como o _ftol; parte baixa por recursao.
    let s = desloc.wrapping_add(entrada).wrapping_rem(0xffff);
    let alta = (s as f64 * (n as f64 * UM_SOBRE_65535)) as i64 as i32;
    let dez_n = n.wrapping_mul(10);
    let baixa = aleatorio(dividir_por_65535(dez_n));
    let mut r = alta.wrapping_add(baixa.wrapping_add(5) / 10);
    if r > n - 1 {
        r = n - 1;
    }
    r
}

/// O que o compilador gerou para `x / 65535` (com sinal): imul 0x80008001, add, sar 15, + sinal.
fn dividir_por_65535(x: i32) -> i32 {
    let alto = ((x as i64 * 0x8000_8001u32 as i32 as i64) >> 32) as i32;
    let t = alto.wrapping_add(x) >> 15;
    t + ((t as u32) >> 31) as i32
}

// ---------- comparacao com a original ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Estado {
    ponteiro: u32,
    deslocamento: i32,
    rand: u32,
}

impl Estado {
    unsafe fn ler() -> Estado {
        Estado { ponteiro: *PONTEIRO, deslocamento: *DESLOCAMENTO, rand: *ESTADO_RAND }
    }
    unsafe fn escrever(self) {
        *PONTEIRO = self.ponteiro;
        *DESLOCAMENTO = self.deslocamento;
        *ESTADO_RAND = self.rand;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Rodando {
    Nada,
    Original,
}

thread_local! {
    static RODANDO: Cell<Rodando> = const { Cell::new(Rodando::Nada) };
}

static PLACAR: Placar = Placar::novo("aleatorio");

thread_local! {
    /// Rastro de sorteios (diagnostico): (n, resposta) de cada chamada de fora, quando ligado.
    static RASTRO: core::cell::RefCell<Option<Vec<(i32, i32)>>> = const { core::cell::RefCell::new(None) };
}

/// Roda `f` guardando os sorteios de fora que ela fizer (n e resposta).
pub fn rastrear<R>(f: impl FnOnce() -> R) -> (R, Vec<(i32, i32)>) {
    let antes = RASTRO.with(|t| t.borrow_mut().replace(Vec::new()));
    let r = f();
    let v = RASTRO.with(|t| core::mem::replace(&mut *t.borrow_mut(), antes)).unwrap_or_default();
    (r, v)
}

/// Roda `f` sem guardar sorteios no rastro (os de uma unidade gravada: a copia Rust recebe o resultado
/// da fita e nao os refaz, entao a lista da original so tem de conter o que a propria rotina sorteia).
pub fn pausar_rastro<R>(f: impl FnOnce() -> R) -> R {
    let guardado = RASTRO.with(|t| t.borrow_mut().take());
    let r = f();
    RASTRO.with(|t| *t.borrow_mut() = guardado);
    r
}

unsafe fn original(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(ORIGINAL);
    f(n)
}

/// Roda a original e a versao Rust a partir do mesmo estado e compara resultado e estado final.
/// O jogo segue com o resultado e o estado da original.
unsafe extern "C" fn comparar(n: i32) -> i32 {
    contar(n);
    // A original chama a si mesma (recursao para n grande) passando por este desvio.
    if RODANDO.with(|r| r.get()) == Rodando::Original {
        return original(n);
    }
    let rastro_ligado = RASTRO.with(|t| t.borrow().is_some());
    let antes = Estado::ler();
    RODANDO.with(|r| r.set(Rodando::Original));
    let r_orig = original(n);
    RODANDO.with(|r| r.set(Rodando::Nada));
    let depois_orig = Estado::ler();

    antes.escrever();
    // restaura (nao liga): dentro de uma replica Rust (`sem_contar`) que sorteia varias vezes, ligar
    // aqui fazia os sorteios seguintes da replica entrarem na contagem
    let contava = CONTAR.with(|c| c.replace(false));
    let r_rust = aleatorio(n);
    CONTAR.with(|c| c.set(contava));
    let depois_rust = Estado::ler();
    depois_orig.escrever();

    PLACAR.registrar(r_orig == r_rust && depois_orig == depois_rust, || {
        format!(
            "n={} original={} rust={} estado antes={:x?} original={:x?} rust={:x?}",
            n, r_orig, r_rust, antes, depois_orig, depois_rust
        )
    });
    if rastro_ligado {
        RASTRO.with(|t| {
            if let Some(v) = t.borrow_mut().as_mut() {
                v.push((n, r_orig));
            }
        });
    }
    r_orig
}

/// Modo observador (`CM_ORIGINAL=1`): roda a original e so conta/verifica a parada da simulacao.
unsafe extern "C" fn observar(n: i32) -> i32 {
    contar(n);
    original(n)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Modo {
    Rust,
    Comparar,
    Observar,
}

pub unsafe fn instalar(modo: Modo) {
    let destino = match modo {
        Modo::Rust => aleatorio as *const () as usize,
        Modo::Comparar => comparar as *const () as usize,
        Modo::Observar => observar as *const () as usize,
    };
    if let Some(t) = desvios::desviar("aleatorio", &PROLOGO, destino) {
        ORIGINAL = t;
        match modo {
            Modo::Comparar => linha("aleatorio: modo comparacao (original x Rust)"),
            Modo::Observar => linha("aleatorio: modo observador (so a original roda)"),
            Modo::Rust => {}
        }
    }
}

/// Compara original e Rust em `casos` entradas sinteticas (estado e n sorteados, cobrindo n
/// pequeno, grande, negativo, bordas e a volta da tabela). Exige o modo comparacao instalado
/// (a recursao da original tem de cair na original).
pub unsafe fn autoteste(casos: u64) {
    if ORIGINAL == 0 {
        return;
    }
    const FIM_REAL: u32 = 0xad4130; // o que a semente (0x90d0a0) grava em [0xde2798]
    let fim = FIM_DA_TABELA as *mut u32;
    let (guardado, fim_guardado) = (Estado::ler(), *fim);
    let mut s = Sorteio(0x9e37_79b9_7f4a_7c15);
    let mut sorteio = || s.proximo();
    let entradas = ((FIM_REAL - INICIO_DA_TABELA) / 4 + 1) as u64;
    let bordas = [1, 2, 3, 7, 10, 100, 0xfffe, 0xffff, 0x10000, 0x10001, 100_000, 1_000_000, i32::MAX];
    PLACAR.autoteste(|| for i in 0..casos {
        let a = sorteio();
        // Ponteiro: qualquer entrada; um em cada oito perto do fim, para exercitar a volta.
        let entrada = if a % 8 == 0 { entradas - 1 - (a >> 3) % 3 } else { (a >> 3) % entradas };
        *fim = if a % 97 == 0 { 0 } else { FIM_REAL }; // antes da semente o fim ainda vale 0
        Estado {
            ponteiro: INICIO_DA_TABELA + entrada as u32 * 4,
            deslocamento: ((a >> 20) & 0xffff) as i32,
            rand: (a >> 32) as u32,
        }
        .escrever();
        let b = sorteio();
        let n = match b % 6 {
            0 => (b >> 8) as i32 % 100 + 1,
            1 => ((b >> 8) % 0xffff) as i32 + 1,
            2 => -(((b >> 8) % 0xffff) as i32 + 1),
            3 => (b >> 8) as u32 as i32, // qualquer, inclusive enormes e negativos
            4 => bordas[(b >> 8) as usize % bordas.len()],
            _ => -bordas[(b >> 8) as usize % (bordas.len() - 1)],
        };
        comparar(n);
        if i % 1_000_000 == 999_999 {
            linha(&format!("aleatorio: autoteste {} casos...", i + 1));
        }
    });
    guardado.escrever();
    *fim = fim_guardado;
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn divisao_por_65535_igual_a_do_compilador() {
        for x in [0, 1, 65534, 65535, 65536, 655350, -1, -65535, -65536, i32::MAX, i32::MIN, 123456789, -987654321] {
            assert_eq!(dividir_por_65535(x), x / 65535, "x={}", x);
        }
    }
}
