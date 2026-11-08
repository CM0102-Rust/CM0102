use super::*;


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type EngFirstScheduleRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets dates and match slots of the rounds from `primeira` in table t (League round set_date / set_match_slot).
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn eng_first_set_league_rounds(t: usize, year: u32, first: i32, table: &[EngFirstScheduleRound]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let band: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in table.iter().enumerate() {
        let i = first + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, year, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            band(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}


/// A fase principal com as rodadas do Natal: as do começo em sequência; depois a data de 1º de dezembro do ano
/// (Date::Date 0x549f70: push −1; push ano; push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10) e o caso pelo dia
/// da rodada de referência (+0, palavra; uma rodada de `casos` levada ao dia da semana): k = (dia da rodada − dia da
/// data + 1, em 16 bits) − `ajuste`; de 0 a 6 escolhe as rodadas do Natal; fora disso a caixa da linha `linha` e as do
/// caso padrão (o oitavo); depois as do fim.
pub(crate) struct EngFirstChristmasSchedule {
    pub(crate) comeco: &'static [EngFirstScheduleRound],
    pub(crate) referencia: usize,
    pub(crate) ajuste: i32,
    pub(crate) casos: [&'static [EngFirstScheduleRound]; 8],
    pub(crate) fim: &'static [EngFirstScheduleRound],
    pub(crate) linha: u32,
}

/// Sets the rounds around Christmas: start rounds, the middle block chosen by the weekday of the reference date, and
/// end rounds.
pub(crate) unsafe fn eng_first_set_christmas_rounds(t: usize, year: u32, n: &EngFirstChristmasSchedule, file: usize) {
    let construct: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    eng_first_set_league_rounds(t, year, 0, n.comeco);
    let mut d = [0u16; 4];
    construct(&mut d, 1, 0xb, year, 0xffff_ffff);
    let day = core::ptr::read_unaligned((t + n.referencia * 0x41) as *const u16);
    let k = (day.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(n.ajuste);
    let caso = if k as u32 > 6 {
        show_error_at(file, n.linha);
        7
    } else {
        k as usize
    };
    let middle = n.casos[caso];
    eng_first_set_league_rounds(t, year, n.comeco.len() as i32, middle);
    eng_first_set_league_rounds(t, year, (n.comeco.len() + middle.len()) as i32, n.fim);
}

/// Side-by-side comparer of a round table: outputs and written bytes must match; frees the copy's table.
///
/// [pt] Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `written_league_round_bytes`; nas outras as de copa, `eng_cc_cup_written_round_bytes`); a tabela da cópia é solta (0x9452ca, cdecl).
pub(crate) unsafe fn eng_first_compare_round_table(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32,
                       original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copy(&mut r2, &mut e2, pl2));
    let gravou = a != 0 || b != 0;
    let outs = !gravou || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        if stage as u8 == 0xff {
            let n = *rounds as usize;
            written_league_round_bytes(a, a, n) == written_league_round_bytes(b, a, n)
        } else {
            eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, *rounds)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && outs && table, || format!("{}(fase {}): saídas {} tabela {}", name, stage as u8 as i8, outs, table));
    a
}

/// a primeira divisão inglesa: as rodadas antes do Natal
pub(crate) const FIRST_DIVISION_START: [EngFirstScheduleRound; 23] = [
    (12, 7, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 0: 12/ago
    (19, 7, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 1: 19/ago
    (26, 7, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 2: 26/ago
    (28, 7, 0, 0, 2, &[(-1, -1, -1)]), // 3: 28/ago
    (2, 8, 0, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 4: 2/set
    (9, 8, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 5: 9/set
    (13, 8, 0, 2, 2, &[(-1, -1, -1)]), // 6: 13/set
    (16, 8, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 7: 16/set
    (23, 8, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 8: 23/set
    (30, 8, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 9: 30/set
    (7, 9, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 10: 7/out
    (14, 9, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 11: 14/out
    (17, 9, 0, 1, 2, &[(2, 2, 2), (2, 2, 2), (-1, -1, -1)]), // 12: 17/out
    (21, 9, 0, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 13: 21/out
    (24, 9, 0, 1, 2, &[(2, 2, 2), (2, 2, 2), (-1, -1, -1)]), // 14: 24/out
    (28, 9, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 15: 28/out
    (4, 10, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 16: 4/nov
    (11, 10, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 17: 11/nov
    (18, 10, 0, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 18: 18/nov
    (25, 10, 0, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 19: 25/nov
    (2, 11, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 20: 2/dez
    (9, 11, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 21: 9/dez
    (16, 11, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 22: 16/dez
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 13/dez
pub(crate) const FIRST_DIVISION_CHRISTMAS_0: [EngFirstScheduleRound; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 14/dez
pub(crate) const FIRST_DIVISION_CHRISTMAS_ROUNDS_1: [EngFirstScheduleRound; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 15/dez
pub(crate) const DIVISION_ONE_CHRISTMAS_ROUNDS_2: [EngFirstScheduleRound; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 16/dez
pub(crate) const FIRST_DIVISION_CHRISTMAS_3: [EngFirstScheduleRound; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 17/dez
pub(crate) const FIRST_DIVISION_CHRISTMAS_ROUNDS_4: [EngFirstScheduleRound; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 18/dez
pub(crate) const DIVISION_ONE_CHRISTMAS_ROUNDS_5: [EngFirstScheduleRound; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 19/dez
pub(crate) const FIRST_DIVISION_CHRISTMAS_6: [EngFirstScheduleRound; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 22 fora de 13 a 19/dez: a caixa)
pub(crate) const FIRST_DIVISION_CHRISTMAS_ROUNDS_DEFAULT: [EngFirstScheduleRound; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// a primeira divisão inglesa: as rodadas depois do Natal
pub(crate) const FIRST_DIVISION_LATE_ROUNDS: [EngFirstScheduleRound; 19] = [
    (13, 0, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 27: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 28: 20/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 29: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 30: 10/fev (+1 ano)
    (17, 1, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 31: 17/fev (+1 ano)
    (20, 1, 1, 1, 2, &[(2, 2, 2), (-1, -1, -1)]), // 32: 20/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 33: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 34: 3/mar (+1 ano)
    (7, 2, 1, 2, 2, &[(-1, -1, -1)]), // 35: 7/mar (+1 ano)
    (10, 2, 1, 5, 1, &[(-1, -1, -1)]), // 36: 10/mar (+1 ano)
    (17, 2, 1, 1, 2, &[(2, 2, 2), (2, 2, 2), (-1, -1, -1)]), // 37: 17/mar (+1 ano)
    (24, 2, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 38: 24/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 39: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 40: 7/abr (+1 ano)
    (14, 3, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 41: 14/abr (+1 ano)
    (16, 3, 1, 0, 2, &[(-1, -1, -1)]), // 42: 16/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 43: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 44: 28/abr (+1 ano)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 45: 6/mai (+1 ano)
];

/// a Premiership: as rodadas antes do Natal
pub(crate) const PREMIERSHIP_EARLY_ROUNDS: [EngFirstScheduleRound; 18] = [
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 19/ago
    (23, 7, 0, 2, 2, &[(1, 2, 2), (-1, -1, -1)]), // 1: 23/ago
    (26, 7, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 2: 26/ago
    (6, 8, 0, 2, 2, &[(1, 2, 2), (-1, -1, -1)]), // 3: 6/set
    (9, 8, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 4: 9/set
    (16, 8, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 5: 16/set
    (23, 8, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 6: 23/set
    (30, 8, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 7: 30/set
    (13, 9, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 8: 13/out
    (21, 9, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 9: 21/out
    (28, 9, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 10: 28/out
    (4, 10, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 11: 4/nov
    (11, 10, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 12: 11/nov
    (18, 10, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 13: 18/nov
    (25, 10, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 14: 25/nov
    (2, 11, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 15: 2/dez
    (9, 11, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 16: 9/dez
    (16, 11, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 17: 16/dez
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 13/dez
pub(crate) const PREMIERSHIP_CHRISTMAS_ROUNDS_0: [EngFirstScheduleRound; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 14/dez
pub(crate) const PREMIERSHIP_CHRISTMAS_ROUNDS_1: [EngFirstScheduleRound; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 15/dez
pub(crate) const PREMIERSHIP_CHRISTMAS_2: [EngFirstScheduleRound; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 16/dez
pub(crate) const PREMIERSHIP_CHRISTMAS_ROUNDS_3: [EngFirstScheduleRound; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 17/dez
pub(crate) const PREMIERSHIP_CHRISTMAS_ROUNDS_4: [EngFirstScheduleRound; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 18: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 21: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 18/dez
pub(crate) const PREMIERSHIP_CHRISTMAS_5: [EngFirstScheduleRound; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 18: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 19/dez
pub(crate) const PREMIERSHIP_CHRISTMAS_ROUNDS_6: [EngFirstScheduleRound; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 19: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 20: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 21: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 17 fora de 13 a 19/dez: a caixa)
pub(crate) const PREMIERSHIP_CHRISTMAS_ROUNDS_DEFAULT: [EngFirstScheduleRound; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 18: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 19: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 20: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 21: 5/jan (+1 ano)
];

/// a Premiership: as rodadas depois do Natal
pub(crate) const PREMIERSHIP_LATE_ROUNDS: [EngFirstScheduleRound; 16] = [
    (13, 0, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 22: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 23: 20/jan (+1 ano)
    (31, 0, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 24: 31/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 25: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 26: 10/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 27: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 28: 3/mar (+1 ano)
    (17, 2, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 29: 17/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 30: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 31: 7/abr (+1 ano)
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 14/abr (+1 ano)
    (18, 3, 1, 2, 2, &[(-1, -1, -1)]), // 33: 18/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 34: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 35: 28/abr (+1 ano)
    (5, 4, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 36: 5/mai (+1 ano)
    (20, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 20/mai (+1 ano)
];

pub(crate) const DIVISION_ONE_CHRISTMAS: EngFirstChristmasSchedule = EngFirstChristmasSchedule { comeco: &FIRST_DIVISION_START, referencia: 22, ajuste: 13, casos: [&FIRST_DIVISION_CHRISTMAS_0, &FIRST_DIVISION_CHRISTMAS_ROUNDS_1, &DIVISION_ONE_CHRISTMAS_ROUNDS_2, &FIRST_DIVISION_CHRISTMAS_3, &FIRST_DIVISION_CHRISTMAS_ROUNDS_4, &DIVISION_ONE_CHRISTMAS_ROUNDS_5, &FIRST_DIVISION_CHRISTMAS_6, &FIRST_DIVISION_CHRISTMAS_ROUNDS_DEFAULT], fim: &FIRST_DIVISION_LATE_ROUNDS, linha: 0x17c };
pub(crate) const DIVISION_ONE_PLAYOFF_0: EngCcCupCupRound = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
pub(crate) const FIRST_DIVISION_PLAYOFF_1: EngCcCupCupRound = cql10r((0x11, 4, 1, 3), (0x1c, 4, 1, 0, 2, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));
pub(crate) const PREMIERSHIP_CHRISTMAS: EngFirstChristmasSchedule = EngFirstChristmasSchedule { comeco: &PREMIERSHIP_EARLY_ROUNDS, referencia: 17, ajuste: 13, casos: [&PREMIERSHIP_CHRISTMAS_ROUNDS_0, &PREMIERSHIP_CHRISTMAS_ROUNDS_1, &PREMIERSHIP_CHRISTMAS_2, &PREMIERSHIP_CHRISTMAS_ROUNDS_3, &PREMIERSHIP_CHRISTMAS_ROUNDS_4, &PREMIERSHIP_CHRISTMAS_5, &PREMIERSHIP_CHRISTMAS_ROUNDS_6, &PREMIERSHIP_CHRISTMAS_ROUNDS_DEFAULT], fim: &PREMIERSHIP_LATE_ROUNDS, linha: 0x173 };

/// the Division One schedule: the same 46 dates as Division Two, with TV slots (usually one or two matches on day 6 or
/// 4, slot 1 or 2); the same Christmas by the day of the 16 December round. The promotion play-offs (stage 0):
/// two-legged semi-finals (4 clubs, draw on 7 May, first leg on 12, 4 days apart) and a one-off final on 28 (weekday 0,
/// slot 2) May [inference: venue 4 is the neutral ground]
/// Original: `EngFirst::create_schedule` (0x571980, eng_first.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x571980 } (thiscall, ret 0x10, eng_first.cpp, 4.688 bytes, método +0x3c): `EngFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase −1, 46 rodadas (`DIVISION_ONE_CHRISTMAS`, as datas da segunda com faixas de TV);
/// a fase 0, os play-offs; as outras devolvem 0. Fonte 0x9c4374.
pub(crate) unsafe extern "thiscall" fn eng_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 46;
            *extra = 0;
            let t = alocar(0xbae);
            if t == 0 {
                show_error_at(0x9c4374, 0xae);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 calendario da primeira inglesa");
            let year = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            eng_first_set_christmas_rounds(t, year, &DIVISION_ONE_CHRISTMAS, 0x9c4374);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 2;
            *extra = 0xa0;
            let t = alocar(0xd0);
            if t == 0 {
                show_error_at(0x9c4374, 0x1ed);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 play-offs da primeira inglesa");
            let year = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &DIVISION_ONE_PLAYOFF_0, year);
            eng_cc_cup_write_cup_round(t, 1, &FIRST_DIVISION_PLAYOFF_1, year);
            t
        }
        _ => 0,
    }
}







/// Club of row k of the league table (+0xb1, 0x3b bytes).
///
/// [pt] O clube da linha `k` da tabela da liga (+0xb1, linhas de 0x3b bytes; o clube no +0).
pub(crate) unsafe fn eng_first_league_row_club(league: usize, k: usize) -> usize {
    read_usize(read_usize(league + 0xb1) + k * 0x3b)
}

/// Creates the play-off stage from 4 table rows: club list, round table, Cup_Stage in the first stage slot, then
/// Cup::set_up_round.
///
/// [pt] O mata-mata das três divisões (o corpo de 0x573080, 0x576a70 e 0x5783c0, que só mudam nas linhas, no fonte e no
/// +0x3a da fase). A lista de `+0xbf` clubes (malloc 0x944e46 de 4 × o byte com sinal; sem memória: a caixa
/// `linha_sem_lista` e nada mais) leva os clubes das 4 `linhas` da tabela, nesta ordem (as chaves: 0 × 1 e 2 × 3).
/// A virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, a conta, a lista, as rodadas, +4, a tabela de rodadas, a palavra +0x40, 0, 1,
/// `a10` (ou o extra da virtual), 0x14, 0, 0, 0, 0) vai para a primeira vaga de fases ([+0xc]); a lista e a tabela de
/// rodadas são liberadas (free 0x9452ca). Sem fase: a caixa `linha_sem_fase`; senão Cup::set_up_round (0x51c800,
/// thiscall(fase, 0), ret 4). O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe fn create_playoff_stage(this: usize, rows: [usize; 4], file: usize, row_without_list: u32, row_without_stage: u32, a10: Option<u32>, name: &'static str) {
    let n = read_u8(this + 0xbf) as i8 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let list = alocar((n << 2) as u32);
    if list == 0 {
        return show_error_at(file, row_without_list);
    }
    for (k, &l) in rows.iter().enumerate() {
        core::ptr::write_unaligned((list + 4 * k) as *mut usize, eng_first_league_row_club(this, l));
    }
    crate::comparacao::ramo(name);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, 0, &mut rodadas, &mut extra, 0);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = new(0xb2);
    // simplification: a original empurra a conta, as rodadas e a palavra +0x40 em registradores inteiros com lixo nos 16 bits
    // altos; o construtor só lê as palavras ([esp+0x230], [esp+0x238], [esp+0x244] e [esp+0x250] com mov ax/dx).
    let stage = if p != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        let a10 = a10.unwrap_or(extra as u16 as u32);
        construct(p, this, n as u16 as u32, list, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 0, 1, a10, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned(read_usize(this + 0xc) as *mut usize, stage);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    free(list);
    free(t);
    let stage = read_usize(read_usize(this + 0xc));
    if stage == 0 {
        return show_error_at(file, row_without_stage);
    }
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    build(stage, 0);
}

/// builds the play-off stage with the 6th, 3rd, 5th and 4th of the table (ties 6th v 3rd and 5th v 4th), the round
/// table from virtual +0x3c and +0x3a = 0xa0; puts the stage in the first slot (+0xc) and sets up the round (0x51c800);
/// out of memory, the error box (lines 0x2f0 and 0x306)
/// Original: `EngFirst::create_playoff_stage` (0x573080, eng_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x573080 } (thiscall, ret 0, eng_first.cpp): `EngFirst::create_playoff_stage()`, da fase seguinte da First
/// (0x572ff0, quando a fase passa de −1 a 0). As chaves: o 6º contra o 3º e o 5º contra o 4º (linhas 5, 2, 4 e 3);
/// +0x3a da fase = 0xa0. Fonte 0x9c4374, linhas 0x2f0 (sem lista) e 0x306 (sem fase).
pub(crate) unsafe extern "thiscall" fn eng_first_create_playoff_stage(this: usize) {
    create_playoff_stage(this, [5, 2, 4, 3], 0x9c4374, 0x2f0, 0x306, Some(0xa0), "cpz_1 mata-mata da first"); // cpz1: 6º × 3º e 5º × 4º da First
}


/// like the Second one, with the 7th, 4th, 6th and 5th (ties 7th v 4th and 6th v 5th; lines 0x2be and 0x2d5)
/// Original: `EngThird::create_play_off_stage` (0x5783c0, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5783c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_play_off_stage()` (de 0x578330). As chaves: o 7º
/// contra o 4º e o 6º contra o 5º (linhas 6, 3, 5 e 4); +0x3a = o extra da virtual. Fonte 0x9c4424, linhas 0x2be e
/// 0x2d5.
pub(crate) unsafe extern "thiscall" fn eng_first_create_play_off_stage(this: usize) {
    create_playoff_stage(this, [6, 3, 5, 4], 0x9c4424, 0x2be, 0x2d5, None, "cpz_1 mata-mata da third"); // cpz1: 7º × 4º e 6º × 5º da Third
}

/// Comparer of the play-off stage: runs only the original and checks the stage, count and the 4 clubs (as a set).
///
/// [pt] O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das 4 linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = a conta
/// (+0xbf), +4 = o +4 da liga e os 4 primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como
/// conjunto: o sorteio da rodada 0x51c800 pode reordenar as chaves).
pub(crate) unsafe fn compare_playoff_stage(name: &str, this: usize, rows: [usize; 4], original: impl FnOnce()) {
    let mut plan: [usize; 4] = [0; 4];
    for (k, &l) in rows.iter().enumerate() {
        plan[k] = eng_first_league_row_club(this, l);
    }
    let n = read_u8(this + 0xbf) as i8 as i16;
    original();
    let stage = read_usize(read_usize(this + 0xc));
    if stage == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", name, this));
        return;
    }
    let list = read_usize(stage + 0xa7);
    let mut found: [usize; 4] = [0; 4];
    for (k, a) in found.iter_mut().enumerate() {
        *a = read_usize(list + 6 * k);
    }
    let (mut x, mut y) = (plan, found);
    x.sort_unstable();
    y.sort_unstable();
    let same = x == y && read_i16(stage + 0x3e) == n && read_usize(stage + 4) == read_usize(this + 4);
    PLACAR.registrar(same, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={} n={}", name, this, plan, stage, found, read_i16(stage + 0x3e), n));
}




/// at the last stage (+0x30 >= +0x2c - 1) the error box (line 0x2ce); otherwise +0x30 + 1 and, on reaching 0, the
/// play-offs (0x573080)
/// Original: `EngFirst::advance_stage` (0x572ff0, eng_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x572ff0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::advance_stage()` (a virtual +0x28): fonte 0x9c4374,
/// linha 0x2ce; o mata-mata 0x573080.
pub(crate) unsafe extern "thiscall" fn eng_first_advance_stage(this: usize) {
    advance_league_stage(this, 0x9c4374, 0x2ce, 0x573080, "cpz_3 mata-mata da first")
}


/// the First Division: discipline 9, no minimum ground, the league and 1 stage (the promotion play-off)
/// Original: `EngFirst::EngFirst` (0x571680, eng_first.cpp, thiscall, ret 8).
///
/// [pt] { 0x571680 } (thiscall, ret 8, eng_first.cpp): `EngFirst::EngFirst(ano, registro)`. A First Division: 1 fase além da
/// liga (o play-off de acesso), sem estádio mínimo.
pub(crate) unsafe extern "thiscall" fn construct_eng_first(this: usize, year: u32, record: usize) -> usize {
    construct_english_division(this, year, record, &FIRST_DIVISION)
}


/// the First (24): the top 2 go up directly, 3rd to 6th go to the promotion play-offs, the bottom 3 go down to the
/// Second; 5 substitutes and 3 substitutions; 2 rounds, 3 points
/// Original: `EngFirst::set_up_rules` (0x572d90, eng_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x572d90 } (thiscall, ret 0, eng_first.cpp): `EngFirst::set_up_rules()`. A First: as comuns, 2 sobem direto
/// (+0xbe), 4 vão ao mata-mata de acesso (+0xbf, do 3º ao 6º), nenhum play-off de permanência (+0xc0), 3 caem
/// (+0xc1), a Premier acima (+0x1c = [0x9cf5bc]) e a Second abaixo (+0x20 = [0x9cf5c4]), 5 reservas no banco
/// (+0x49) e 3 substituições (+0x4a).
pub(crate) unsafe extern "thiscall" fn eng_first_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da first");
    eng_prm_apply_league_rules(this, &eng_prm_first_division_rules())
}


/// the First has 24 clubs; prize fund of 70,000 per place; TV pays by the division the club came from: 5,877,777 to
/// those relegated from the Premier and 4,099,999 to those already in the First and those promoted from the Second
/// (1,000,000 to all up to 1999)
/// Original: `EngFirst::create_table` (0x572bf0, eng_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x572bf0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::create_table()`: 24 clubes ([0x9cf5c0]), cota 70.000
/// e a TV pela divisão de onde o clube veio (+0x5b): depois de 1999, 5.877.777 aos que caíram da Premier (o registro
/// de [0x9cf5bc]), 4.099.999 aos que já estavam na First (o próprio registro, +4) e 4.099.999 aos que subiram da
/// Second ([0x9cf5c4]); antes, 1.000.000 a todos. Fonte 0x9c4374, linha 0x22d.
pub(crate) unsafe extern "thiscall" fn eng_first_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(this, 0x18, 0x9c4374, 0x22d, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, 0x11170, || {
        if is_after_1999() {
            eng_prm_pay_tv_rights(this, 0x59b011, comp_record_of(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION)); // cqi5: a TV de quem caiu da Premier
            eng_prm_pay_tv_rights(this, 0x3e8fa0, read_usize(this + 4));
            eng_prm_pay_tv_rights(this, 0x3e8fa0, comp_record_of(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION));
        } else {
            eng_prm_pay_tv_rights(this, 0xf4240, 0);
        }
    })
}


/// writes stage, best and worst in the club stats entry (+0x73..+0x75): in the league, best 3 to 6 becomes 3 and worst
/// 3 to 6 becomes 6; in the play-offs (stage 0), 1, 2 and 3 become 3, 4 and 5
/// Original: `EngFirst::set_club_progress` (0x573260, eng_first.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x573260 } (thiscall, ret 0x14, eng_first.cpp): `EngFirst::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 da First e da Second (da 0x684cd0 a cada linha da tabela): as vagas do mata-mata são 3 a 6.
pub(crate) unsafe extern "thiscall" fn eng_first_set_club_progress(this: usize, clube: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_club_stage_status(this, clube, stage as u8, current as u8, best as u8, worst as u8, 3, 6)
}

