//! Simulacao em lote (ver `docs/VERSAO_SEM_TELA.md` e `engenharia-reversa/simular.py`).
//!
//! `CM_PARAR_EM=AAAA-MM-DD`: na primeira chamada do sorteio depois de a data atual do jogo
//! (`Data` em 0xae2c90) alcancar o alvo, grava a impressao digital em `ddraw.log` e encerra o
//! processo. O ponto de parada e fixo na sequencia do jogo, entao duas execucoes com a mesma
//! semente (`-seed`) e as mesmas rotinas tem de dar a mesma impressao.
//!
//! Impressao digital: quantidade de sorteios desde o inicio + estado completo do gerador
//! (ponteiro da tabela, deslocamento, estado do rand() da CRT). Qualquer diferenca de
//! comportamento que mude o numero ou a ordem dos sorteios aparece aqui.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

use crate::registro::linha;

const DATA_ATUAL: *const [u16; 2] = 0xae2c90 as *const [u16; 2]; // (dia do ano, ano)

static SORTEIOS: AtomicU64 = AtomicU64::new(0);

/// Alvo (ano, dia do ano a partir de 0) lido de CM_PARAR_EM.
fn alvo() -> Option<(u16, u16)> {
    static ALVO: OnceLock<Option<(u16, u16)>> = OnceLock::new();
    *ALVO.get_or_init(|| {
        let n: Vec<u16> = "CM_PARAR_EM".encode_utf16().chain([0]).collect();
        let mut buf = [0u16; 32];
        let k = unsafe { crate::win::GetEnvironmentVariableW(n.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
        let s = String::from_utf16_lossy(&buf[..k.min(32)]);
        let a = interpretar(&s);
        if let Some((ano, dia)) = a {
            linha(&format!("simulacao: parar em {} (ano {}, dia {})", s, ano, dia));
        }
        a
    })
}

/// "AAAA-MM-DD" -> (ano, dia do ano a partir de 0), com a regra de bissexto do jogo.
pub fn interpretar(s: &str) -> Option<(u16, u16)> {
    let mut p = s.trim().split('-').map(|x| x.parse::<u32>().ok());
    let (a, m, d) = (p.next()??, p.next()??, p.next()??);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || a > 65535 {
        return None;
    }
    let acumulado = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let extra = (m > 2 && crate::data::bissexto(a as u16) == 1) as u32;
    Some((a as u16, (acumulado[m as usize - 1] + extra + d - 1) as u16))
}

/// Chamado no inicio de cada sorteio (versao Rust e modo comparacao), com o estado do gerador antes da chamada.
pub unsafe fn sorteio(ponteiro: u32, deslocamento: i32, rand: u32) {
    let n = SORTEIOS.fetch_add(1, Ordering::Relaxed);
    let Some((ano, dia)) = alvo() else { return };
    let [d, a] = *DATA_ATUAL;
    // Diario: uma linha por dia do jogo, para achar onde duas execucoes comecam a divergir.
    static ULTIMO_DIA: AtomicU64 = AtomicU64::new(0);
    let hoje = (a as u64) << 16 | d as u64;
    if ULTIMO_DIA.swap(hoje, Ordering::Relaxed) != hoje {
        linha(&format!("dia {}-{:03} sorteios={} ponteiro={:#x} deslocamento={:#x} rand={:#x}", a, d, n, ponteiro, deslocamento, rand));
    }
    if (a, d) < (ano, dia) || a < 1900 {
        return;
    }
    linha(&format!(
        "SIMULACAO parou em {}-{:03} sorteios={} ponteiro={:#x} deslocamento={:#x} rand={:#x}",
        a, d, n, ponteiro, deslocamento, rand
    ));
    // Encerra sem rodar finalizacoes do jogo (nada de gravar preferencias).
    crate::win::TerminateProcess(crate::win::GetCurrentProcess(), 0);
}

// ---------- relogio fixo ----------

/// Instante devolvido pelo time() do jogo durante a simulacao (9/9/2001 01:46:40 UTC).
const RELOGIO_FIXO: u32 = 1_000_000_000;

// 0x946ebb: push ebp; mov ebp, esp; sub esp, 0xcc
const PROLOGO_TIME: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x946ebb, bytes: &[0x55, 0x8b, 0xec, 0x81, 0xec, 0xcc, 0x00, 0x00, 0x00] };

/// time() da CRT { 0x946ebb } na simulacao: sempre o mesmo instante. O jogo ressemeia o rand()
/// com o relogio (manager_manager.cpp, 0x696d4b), o que tornaria a simulacao irreproduzivel.
unsafe extern "C" fn time_fixo(saida: *mut u32) -> u32 {
    if !saida.is_null() {
        *saida = RELOGIO_FIXO;
    }
    RELOGIO_FIXO
}

/// Chamado no carregamento da DLL: so age em modo simulacao (CM_PARAR_EM definido).
pub unsafe fn instalar() {
    if alvo().is_some() {
        crate::desvios::desviar("time (relogio fixo da simulacao)", &PROLOGO_TIME, time_fixo as *const () as usize);
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn datas() {
        assert_eq!(interpretar("2001-01-01"), Some((2001, 0)));
        assert_eq!(interpretar("2001-08-09"), Some((2001, 220))); // a data vista na memoria do jogo
        assert_eq!(interpretar("2004-03-01"), Some((2004, 60)));
        assert_eq!(interpretar("2001-13-01"), None);
        assert_eq!(interpretar("lixo"), None);
    }
}
