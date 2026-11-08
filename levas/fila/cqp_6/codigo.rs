// ---------- %%LEVA%%a leva: o calendário da Primera espanhola: as 38 rodadas, uma tabela para o ano par e outra para o ímpar, com a rodada do Natal conforme o dia da semana (spa_first.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type SpaFirstLeagueRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn spa_first_write_league_rounds(t: usize, ano: u32, primeira: i32, tabela: &[SpaFirstLeagueRound]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(0x68a160usize);
    let band: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            band(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// Os bytes que a tabela de `n` rodadas tem gravados e que o jogo lê: de cada rodada, a data (+0..+4) e +0x3d..+0x40,
/// e as faixas até a que termina a lista (a de dia −1 na tabela `guia`, a da original); o resto da memória alocada não
/// é escrito.
unsafe fn cqp6_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..8usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + 5 + j * 7) as *const u8, 7));
            if *((guia + r * 0x41 + 5 + j * 7 + 4) as *const i8) == -1 {
                break;
            }
        }
    }
    v
}

/// A fase principal com as rodadas do Natal: as do começo em sequência; depois a data de 1º de dezembro do ano
/// (Date::Date 0x549f70: push −1; push ano; push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10) e o caso pelo dia
/// da rodada de referência (+0, palavra; uma rodada de `casos` levada ao dia da semana): k = (dia da rodada − dia da
/// data + 1, em 16 bits) − `ajuste`; de 0 a 6 escolhe as rodadas do Natal; fora disso a caixa da linha `linha` e as do
/// caso padrão (o oitavo); depois as do fim.
struct SpaFirstChristmasSchedule {
    comeco: &'static [SpaFirstLeagueRound],
    referencia: usize,
    ajuste: i32,
    casos: [&'static [SpaFirstLeagueRound]; 8],
    fim: &'static [SpaFirstLeagueRound],
    linha: u32,
}

unsafe fn write_rounds_winter_break(t: usize, ano: u32, n: &SpaFirstChristmasSchedule, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    spa_first_write_league_rounds(t, ano, 0, n.comeco);
    let mut d = [0u16; 4];
    construir(&mut d, 1, 0xb, ano, 0xffff_ffff);
    let dia = core::ptr::read_unaligned((t + n.referencia * 0x41) as *const u16);
    let k = (dia.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(n.ajuste);
    let caso = if k as u32 > 6 {
        show_error_at(arquivo, n.linha);
        7
    } else {
        k as usize
    };
    let meio = n.casos[caso];
    spa_first_write_league_rounds(t, ano, n.comeco.len() as i32, meio);
    spa_first_write_league_rounds(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqp6_gravados`; nas outras as de copa, `eng_cc_cup_written_round_bytes`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn compare_league_round_table(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let gravou = a != 0 || b != 0;
    let saidas = !gravou || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        if fase as u8 == 0xff {
            let n = *rodadas as usize;
            cqp6_gravados(a, a, n) == cqp6_gravados(b, a, n)
        } else {
            eng_cc_cup_written_round_bytes(a, *rodadas) == eng_cc_cup_written_round_bytes(b, *rodadas)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// a Primera espanhola: as rodadas antes do Natal (ano par)
const PRIMERA_EVEN_YEAR_EARLY_ROUNDS: [SpaFirstLeagueRound; 15] = [
    (30, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 0: 30/ago
    (13, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 1: 13/set
    (20, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 2: 20/set
    (27, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 3: 27/set
    (4, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 4: 4/out
    (18, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 5: 18/out
    (25, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 6: 25/out
    (1, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 7: 1/nov
    (8, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 8: 8/nov
    (15, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 9: 15/nov
    (22, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 10: 22/nov
    (29, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 11: 29/nov
    (6, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 12: 6/dez
    (13, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 13: 13/dez
    (20, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 14: 20/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 17/dez (ano par)
const PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_0: [SpaFirstLeagueRound; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 18/dez (ano par)
const PRIMERA_EVEN_YEAR_CHRISTMAS_1: [SpaFirstLeagueRound; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 19/dez (ano par)
const PRIMERA_EVEN_CHRISTMAS_ROUNDS_2: [SpaFirstLeagueRound; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 20/dez (ano par)
const PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_3: [SpaFirstLeagueRound; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 27/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 21/dez (ano par)
const PRIMERA_EVEN_YEAR_CHRISTMAS_4: [SpaFirstLeagueRound; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 28/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 22/dez (ano par)
const PRIMERA_EVEN_CHRISTMAS_ROUNDS_5: [SpaFirstLeagueRound; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 29/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 23/dez (ano par)
const PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_6: [SpaFirstLeagueRound; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 14 fora de 17 a 23/dez: a caixa) (ano par)
const PRIMERA_EVEN_YEAR_CHRISTMAS_DEFAULT: [SpaFirstLeagueRound; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 27/dez
];

/// a Primera espanhola: as rodadas depois do Natal (ano par)
const PRIMERA_EVEN_YEAR_END: [SpaFirstLeagueRound; 22] = [
    (3, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 3/jan (+1 ano)
    (10, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 17: 10/jan (+1 ano)
    (17, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 18: 17/jan (+1 ano)
    (24, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 19: 24/jan (+1 ano)
    (31, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 31/jan (+1 ano)
    (7, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 21: 7/fev (+1 ano)
    (14, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 22: 14/fev (+1 ano)
    (21, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 23: 21/fev (+1 ano)
    (28, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 24: 28/fev (+1 ano)
    (7, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 25: 7/mar (+1 ano)
    (14, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 26: 14/mar (+1 ano)
    (21, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 27: 21/mar (+1 ano)
    (4, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 28: 4/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 29: 11/abr (+1 ano)
    (18, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 30: 18/abr (+1 ano)
    (25, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 31: 25/abr (+1 ano)
    (2, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 32: 2/mai (+1 ano)
    (9, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 33: 9/mai (+1 ano)
    (16, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 34: 16/mai (+1 ano)
    (23, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 35: 23/mai (+1 ano)
    (30, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 36: 30/mai (+1 ano)
    (6, 5, 1, 6, 1, &[(-1, -1, -1)]), // 37: 6/jun (+1 ano)
];

/// a Primera espanhola: as rodadas antes do Natal (ano impar)
const PRIMERA_ODD_YEAR_START: [SpaFirstLeagueRound; 16] = [
    (22, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 0: 22/ago
    (29, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 1: 29/ago
    (12, 8, 0, 6, 2, &[(5, 1, 1), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 4: 26/set
    (3, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 5: 3/out
    (17, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 6: 17/out
    (20, 9, 0, 2, 2, &[(-1, -1, -1)]), // 7: 20/out
    (24, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 8: 24/out
    (31, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 9: 31/out
    (7, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 10: 7/nov
    (21, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 11: 21/nov
    (28, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 12: 28/nov
    (5, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 13: 5/dez
    (12, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 14: 12/dez
    (19, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 19/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 16/dez (ano impar)
const PRIMERA_ODD_YEAR_CHRISTMAS_0: [SpaFirstLeagueRound; 1] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 23/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 17/dez (ano impar)
const PRIMERA_ODD_CHRISTMAS_ROUNDS_1: [SpaFirstLeagueRound; 1] = [
    (20, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 20/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 18/dez (ano impar)
const PRIMERA_ODD_YEAR_CHRISTMAS_ROUNDS_2: [SpaFirstLeagueRound; 1] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 21/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 19/dez (ano impar)
const PRIMERA_ODD_YEAR_CHRISTMAS_3: [SpaFirstLeagueRound; 1] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 22/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 20/dez (ano impar)
const PRIMERA_ODD_CHRISTMAS_ROUNDS_4: [SpaFirstLeagueRound; 1] = [
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 16: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 21/dez (ano impar)
const PRIMERA_ODD_YEAR_CHRISTMAS_ROUNDS_5: [SpaFirstLeagueRound; 1] = [
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 28/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 22/dez (ano impar)
const PRIMERA_ODD_YEAR_CHRISTMAS_6: [SpaFirstLeagueRound; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 29/dez
];

/// as rodadas do Natal no caso padrão (a rodada 15 fora de 16 a 22/dez: a caixa) (ano impar)
const PRIMERA_ODD_CHRISTMAS_ROUNDS_DEFAULT: [SpaFirstLeagueRound; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 27/dez
];

/// a Primera espanhola: as rodadas depois do Natal (ano impar)
const PRIMERA_ODD_LATE_ROUNDS: [SpaFirstLeagueRound; 21] = [
    (5, 0, 1, 2, 2, &[(-1, -1, -1)]), // 17: 5/jan (+1 ano)
    (9, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 18: 9/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 19: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 21: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 22: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 23: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 24: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 25: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 26: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 27: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 28: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 29: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 30: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 31: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 32: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 33: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 34: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 35: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 36: 14/mai (+1 ano)
    (21, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 21/mai (+1 ano)
];

const PRIMERA_EVEN_CHRISTMAS_SCHEDULE: SpaFirstChristmasSchedule = SpaFirstChristmasSchedule { comeco: &PRIMERA_EVEN_YEAR_EARLY_ROUNDS, referencia: 14, ajuste: 17, casos: [&PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_0, &PRIMERA_EVEN_YEAR_CHRISTMAS_1, &PRIMERA_EVEN_CHRISTMAS_ROUNDS_2, &PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_3, &PRIMERA_EVEN_YEAR_CHRISTMAS_4, &PRIMERA_EVEN_CHRISTMAS_ROUNDS_5, &PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_6, &PRIMERA_EVEN_YEAR_CHRISTMAS_DEFAULT], fim: &PRIMERA_EVEN_YEAR_END, linha: 0x11b };
const PRIMERA_ODD_YEAR_CHRISTMAS: SpaFirstChristmasSchedule = SpaFirstChristmasSchedule { comeco: &PRIMERA_ODD_YEAR_START, referencia: 15, ajuste: 16, casos: [&PRIMERA_ODD_YEAR_CHRISTMAS_0, &PRIMERA_ODD_CHRISTMAS_ROUNDS_1, &PRIMERA_ODD_YEAR_CHRISTMAS_ROUNDS_2, &PRIMERA_ODD_YEAR_CHRISTMAS_3, &PRIMERA_ODD_CHRISTMAS_ROUNDS_4, &PRIMERA_ODD_YEAR_CHRISTMAS_ROUNDS_5, &PRIMERA_ODD_YEAR_CHRISTMAS_6, &PRIMERA_ODD_CHRISTMAS_ROUNDS_DEFAULT], fim: &PRIMERA_ODD_LATE_ROUNDS, linha: 0x1e1 };

/// { 0x84e450 } (thiscall, ret 0x10, spa_first.cpp, 6.107 bytes, método +0x3c): `SpaFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1, 38 rodadas, a tabela pela paridade do ano +0x40 (o resto da divisão com
/// sinal: `movsx; and 0x80000001; jns`), cada uma com a rodada do Natal; as outras devolvem 0. Fonte 0xa82944.
unsafe extern "thiscall" fn spa_first_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 38;
            *extra = 0;
            let t = alocar(0x9a6);
            if t == 0 {
                show_error_at(0xa82844, 0xb0);
                return 0;
            }
            crate::comparacao::ramo("cqp_6 calendario da primera");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            if (ano as i16) % 2 != 0 {
                write_rounds_winter_break(t, ano, &PRIMERA_ODD_YEAR_CHRISTMAS, 0xa82844);
            } else {
                write_rounds_winter_break(t, ano, &PRIMERA_EVEN_CHRISTMAS_SCHEDULE, 0xa82844);
            }
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_spa_first_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("spa_first_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| spa_first_create_schedule(this, fase, r, e, l))
}
