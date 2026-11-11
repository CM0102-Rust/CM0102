use super::*;

/// Describes one round of the 0x41-byte table.
///
/// It holds the date (day, month 0..11, years to add to the league year +0x40, the weekday the date is moved to), the
/// code of byte +4 ([inference] the half of the season) and the TV slots in order (weekday, half, how many matches; the
/// list ends with −1).
///
/// [pt] Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type SpaFirstLeagueRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets the dates and match slots of rounds `primeira`.. of table `t`, in the original routine's order.
///
/// For each round, the date (`league_round_set_date`: push 0; push year; push code; push week; push years; push month; push day; push
/// i; push t; add esp -> cdecl: value 0) and the slots (`store_round_match`: push 0; push c; push b; push a; push j; push i; push
/// t; add esp -> cdecl, value 0).
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn spa_first_write_league_rounds(t: usize, year: u32, first: i32, table: &[SpaFirstLeagueRound]) {
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in table.iter().enumerate() {
        let i = first + k as i32;
        exe::league_round_set_date(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, year, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            exe::league_round_set_match_slot(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// The main stage with the Christmas rounds.
///
/// The opening rounds in sequence; then the date of 1 December of the year (Date::Date `exe::date_date_549f70`: push −1; push year;
/// push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10) and the case chosen by the day of the reference round (+0,
/// word; a round of `casos` moved to the weekday): k = (round day − date day + 1, in 16 bits) − `ajuste`; from 0 to 6
/// it picks the Christmas rounds; outside that, the error box for line `linha` and those of the default case (the
/// eighth); then the closing rounds.
///
/// [pt] A fase principal com as rodadas do Natal: as do começo em sequência; depois a data de 1º de dezembro do ano
/// (Date::Date 0x549f70: push −1; push ano; push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10) e o caso pelo dia
/// da rodada de referência (+0, palavra; uma rodada de `casos` levada ao dia da semana): k = (dia da rodada − dia da
/// data + 1, em 16 bits) − `ajuste`; de 0 a 6 escolhe as rodadas do Natal; fora disso a caixa da linha `linha` e as do
/// caso padrão (o oitavo); depois as do fim.
pub(crate) struct SpaFirstChristmasSchedule {
    pub(crate) comeco: &'static [SpaFirstLeagueRound],
    pub(crate) reference: usize,
    pub(crate) adjust: i32,
    pub(crate) cases: [&'static [SpaFirstLeagueRound]; 8],
    pub(crate) end: &'static [SpaFirstLeagueRound],
    pub(crate) row: u32,
}

/// Writes the league round table with a winter break: start rounds, a middle block chosen by the day offset from 1
/// November, and end rounds [inference].
pub(crate) unsafe fn write_rounds_winter_break(t: usize, year: u32, n: &SpaFirstChristmasSchedule, file: usize) {
    spa_first_write_league_rounds(t, year, 0, n.comeco);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let day = read_u16(t + n.reference * 0x41);
    let k = (day.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(n.adjust);
    let case = if k as u32 > 6 {
        show_error_at(file, n.row);
        7
    } else {
        k as usize
    };
    let middle = n.cases[case];
    spa_first_write_league_rounds(t, year, n.comeco.len() as i32, middle);
    spa_first_write_league_rounds(t, year, (n.comeco.len() + middle.len()) as i32, n.end);
}

/// Side-by-side comparer for create_round_table at all stages: the original routine with the caller's outputs, the copy
/// with its own outputs, starting from the same [0xb67a34].
///
/// The outputs and the written bytes of both tables must be equal (at stage −1 the league rounds,
/// `written_league_round_bytes`; at the others the cup rounds, `eng_cc_cup_written_round_bytes`); the copy's table is
/// freed (`exe::crt_free`, cdecl).
///
/// [pt] Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `written_league_round_bytes`; nas outras as de copa, `eng_cc_cup_written_round_bytes`); a tabela da cópia é solta (0x9452ca, cdecl).
pub(crate) unsafe fn compare_league_round_table(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32,
                       original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, original, || copy(&mut r2, &mut e2, pl2));
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
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(stage {}): outs {} table {}", name, stage as u8 as i8, outs, table));
    a
}

/// a Primera espanhola: as rodadas antes do Natal (ano par)
pub(crate) const PRIMERA_EVEN_YEAR_EARLY_ROUNDS: [SpaFirstLeagueRound; 15] = [
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
pub(crate) const PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_0: [SpaFirstLeagueRound; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 18/dez (ano par)
pub(crate) const PRIMERA_EVEN_YEAR_CHRISTMAS_1: [SpaFirstLeagueRound; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 19/dez (ano par)
pub(crate) const PRIMERA_EVEN_CHRISTMAS_ROUNDS_2: [SpaFirstLeagueRound; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 20/dez (ano par)
pub(crate) const PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_3: [SpaFirstLeagueRound; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 27/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 21/dez (ano par)
pub(crate) const PRIMERA_EVEN_YEAR_CHRISTMAS_4: [SpaFirstLeagueRound; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 28/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 22/dez (ano par)
pub(crate) const PRIMERA_EVEN_CHRISTMAS_ROUNDS_5: [SpaFirstLeagueRound; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 29/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 23/dez (ano par)
pub(crate) const PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_6: [SpaFirstLeagueRound; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 14 fora de 17 a 23/dez: a caixa) (ano par)
pub(crate) const PRIMERA_EVEN_YEAR_CHRISTMAS_DEFAULT: [SpaFirstLeagueRound; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 27/dez
];

/// a Primera espanhola: as rodadas depois do Natal (ano par)
pub(crate) const PRIMERA_EVEN_YEAR_END: [SpaFirstLeagueRound; 22] = [
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
pub(crate) const PRIMERA_ODD_YEAR_START: [SpaFirstLeagueRound; 16] = [
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
pub(crate) const PRIMERA_ODD_YEAR_CHRISTMAS_0: [SpaFirstLeagueRound; 1] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 23/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 17/dez (ano impar)
pub(crate) const PRIMERA_ODD_CHRISTMAS_ROUNDS_1: [SpaFirstLeagueRound; 1] = [
    (20, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 20/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 18/dez (ano impar)
pub(crate) const PRIMERA_ODD_YEAR_CHRISTMAS_ROUNDS_2: [SpaFirstLeagueRound; 1] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 21/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 19/dez (ano impar)
pub(crate) const PRIMERA_ODD_YEAR_CHRISTMAS_3: [SpaFirstLeagueRound; 1] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 22/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 20/dez (ano impar)
pub(crate) const PRIMERA_ODD_CHRISTMAS_ROUNDS_4: [SpaFirstLeagueRound; 1] = [
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 16: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 21/dez (ano impar)
pub(crate) const PRIMERA_ODD_YEAR_CHRISTMAS_ROUNDS_5: [SpaFirstLeagueRound; 1] = [
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 28/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 22/dez (ano impar)
pub(crate) const PRIMERA_ODD_YEAR_CHRISTMAS_6: [SpaFirstLeagueRound; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 29/dez
];

/// as rodadas do Natal no caso padrão (a rodada 15 fora de 16 a 22/dez: a caixa) (ano impar)
pub(crate) const PRIMERA_ODD_CHRISTMAS_ROUNDS_DEFAULT: [SpaFirstLeagueRound; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 27/dez
];

/// a Primera espanhola: as rodadas depois do Natal (ano impar)
pub(crate) const PRIMERA_ODD_LATE_ROUNDS: [SpaFirstLeagueRound; 21] = [
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

pub(crate) const PRIMERA_EVEN_CHRISTMAS_SCHEDULE: SpaFirstChristmasSchedule = SpaFirstChristmasSchedule { comeco: &PRIMERA_EVEN_YEAR_EARLY_ROUNDS, reference: 14, adjust: 17, cases: [&PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_0, &PRIMERA_EVEN_YEAR_CHRISTMAS_1, &PRIMERA_EVEN_CHRISTMAS_ROUNDS_2, &PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_3, &PRIMERA_EVEN_YEAR_CHRISTMAS_4, &PRIMERA_EVEN_CHRISTMAS_ROUNDS_5, &PRIMERA_EVEN_YEAR_CHRISTMAS_ROUNDS_6, &PRIMERA_EVEN_YEAR_CHRISTMAS_DEFAULT], end: &PRIMERA_EVEN_YEAR_END, row: 0x11b };
pub(crate) const PRIMERA_ODD_YEAR_CHRISTMAS: SpaFirstChristmasSchedule = SpaFirstChristmasSchedule { comeco: &PRIMERA_ODD_YEAR_START, reference: 15, adjust: 16, cases: [&PRIMERA_ODD_YEAR_CHRISTMAS_0, &PRIMERA_ODD_CHRISTMAS_ROUNDS_1, &PRIMERA_ODD_YEAR_CHRISTMAS_ROUNDS_2, &PRIMERA_ODD_YEAR_CHRISTMAS_3, &PRIMERA_ODD_CHRISTMAS_ROUNDS_4, &PRIMERA_ODD_YEAR_CHRISTMAS_ROUNDS_5, &PRIMERA_ODD_YEAR_CHRISTMAS_6, &PRIMERA_ODD_CHRISTMAS_ROUNDS_DEFAULT], end: &PRIMERA_ODD_LATE_ROUNDS, row: 0x1e1 };

/// the Primera schedule: 38 rounds (20 clubs), nearly all on Sunday (day 6) with one TV slot (one match on day 5); two
/// tables, by the parity of the year: in odd years (2001/02) from 22 August to 21 May, in even years from 30 August to
/// 6 June; in each, one fixed-date Christmas round (between 20 and 30 December, or 3 January) chosen by the day the
/// mid-December round falls on
/// Original: `SpaFirst::create_schedule` (0x84e450, spa_first.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x84e450 } (thiscall, ret 0x10, spa_first.cpp, 6.107 bytes, método +0x3c): `SpaFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1, 38 rodadas, a tabela pela paridade do ano +0x40 (o resto da divisão com
/// sinal: `movsx; and 0x80000001; jns`), cada uma com a rodada do Natal; as outras devolvem 0. Fonte 0xa82944.
pub(crate) unsafe extern "thiscall" fn spa_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 38;
            *extra = 0;
            let t = exe::crt_malloc(0x9a6);
            if t == 0 {
                show_error_at(0xa82844, 0xb0);
                return 0;
            }
            crate::comparison::branch("cqp_6 primera calendar");
            let year = (this.year() as u16) as u32;
            if (year as i16) % 2 != 0 {
                write_rounds_winter_break(t, year, &PRIMERA_ODD_YEAR_CHRISTMAS, 0xa82844);
            } else {
                write_rounds_winter_break(t, year, &PRIMERA_EVEN_CHRISTMAS_SCHEDULE, 0xa82844);
            }
            t
        }
        _ => 0,
    }
}

/// In the rehearsal (the comparer) the copy writes the rules and does not call virtual +0x3c (the round table, which
/// allocates).
///
/// [pt] No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
pub(crate) static SPA_FIRST_RULES_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Writes the (offset, size, value) rules into the league and builds the main stage round table via virtual +0x3c into
/// +0xba (skipped in rehearsal); returns 1.
///
/// [pt] Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
pub(crate) unsafe fn spa_first_apply_league_rules(this: League, written: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in written {
        match t {
            1 => *((this.addr() + d) as *mut u8) = v as u8,
            2 => write_u16(this.addr() + d, v as u16),
            _ => write_u32(this.addr() + d, v),
        }
    }
    if !SPA_FIRST_RULES_DRY_RUN.load(Relaxed) {
        let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
        let t = rounds(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
        this.set_round_dates(t);
    }
    1
}

/// The rules common to the three Spanish leagues.
///
/// 1 point for a draw (+0xc3), category 2 for the matches (+0xc4), national clubs (+0x42 = 1), tie-breakers 4, 1 and 2
/// (+0xc5..+0xc7: head-to-head first [inference from code 4], then goal difference and goals scored), no relegation
/// play-off (+0xc0 = 0), rule bits +0xd9 = 0x43 (champion, top promotion places and the fixed round order), 7
/// substitutes (+0x49) and 3 substitutions (+0x4a).
///
/// [pt] As regras comuns às três: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), clubes nacionais (+0x42
/// = 1), os desempates 4, 1 e 2 (+0xc5..+0xc7: o confronto direto primeiro [inferência pelo código 4], depois o
/// saldo e os gols pró), nenhum play-off de permanência (+0xc0 = 0), os bits de regra +0xd9 = 0x43 (campeão, vagas de
/// cima de acesso e a ordem fixa das rodadas), 7 reservas (+0x49) e 3 substituições (+0x4a).
pub(crate) fn spanish_common_rules() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2), (0xc0, 1, 0), (0xd9, 2, 0x43), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// La Primera (20 clubs, 38 rounds): 2 rounds, 3 points for a win and 1 for a draw; nobody goes up, the bottom 3 go
/// down to the Segunda; tie-breaks 4, 1, 2 and 3 [codes; inference: head to head, goal difference, goals for, wins]; 7
/// substitutes and 3 substitutions; fixed round order
/// Original: `SpaFirst::set_up_rules` (0x84fdf0, spa_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x84fdf0 } (thiscall, ret 0, spa_first.cpp): `SpaFirst::set_up_rules()`. A Primera: +0xa9 = 38 (as rodadas;
/// a virtual reescreve), as comuns, 2 turnos, 3 pontos, o 4º desempate 3 (+0xc8), +0xab e +0xdb = 0, ninguém sobe
/// (+0xbe = +0xbf = 0), 3 caem (+0xc1), nenhuma liga acima (+0x1c = −1), a Segunda abaixo (+0x20 = [0x9cf628]) e
/// +0x52 = 2 [sem nome].
pub(crate) unsafe extern "thiscall" fn spa_first_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    crate::comparison::branch("cqk_4 primera rules");
    this.set_round_count((0x26) as i16);
    spa_first_apply_league_rules(this, &primera_rules())
}

/// Rule list for the Spanish First Division: common rules plus 3 relegated to the Second Division.
pub(crate) unsafe fn primera_rules() -> Vec<(usize, u8, u32)> {
    let mut v = spanish_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc8, 1, 3), (0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqk4: a Primera rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) as u32), (0x52, 1, 2)]);
    v
}

/// Rule list for the Spanish Second Division: common rules, promotion to the First and relegation to Second B when
/// selected.
pub(crate) unsafe fn segunda_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = spanish_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a Segunda joga 2 turnos
    v.extend_from_slice(&[(0xc8, 1, 3), (0xbe, 1, 3), (0xbf, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) as u32)]);
    let nation = Nation(read_usize(read_usize(this + 4) + competition::NATION));
    let below = if nation.league_selected() & 4 != 0 { read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, below), (0x52, 1, 2)]);
    v
}

/// Rule list for the lower Spanish division: common rules, promotion places and the First Division above.
pub(crate) unsafe fn lower_division_rules() -> Vec<(usize, u8, u32)> {
    let mut v = spanish_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: the lower division gives 3 points per win
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 2), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) as u32), (0x20, 4, 0xffff_ffff)]);
    v
}

/// The comparer of the three Spanish rule routines (dry run, like that of `cqi_4`).
///
/// The original routine runs fully; the range `+0x1c`..`+0xec` is restored and the copy writes the rules without
/// virtual `+0x3c`; the listed fields must equal the original's (the Primera's `+0xa9`, which the virtual rewrites, is
/// left out), and the return value 1; at the end the original's range is kept.
///
/// [pt] O comparador das três (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos da lista saem iguais aos da original (o +0xa9 da Primera, que a
/// virtual reescreve, fica de fora), e a devolução 1; no fim fica a faixa da original.
pub(crate) unsafe fn spa_first_compare_league_rules(name: &str, this: League, written: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let band = [(this.addr() + league::UPPER_DIVISION_ID, 0xd0)];
    let before = crate::comparison::MemorySnapshot::read(&band);
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&band);
    before.write_back();
    SPA_FIRST_RULES_DRY_RUN.store(true, Relaxed);
    let rr = crate::rng::pause_trace(copy);
    SPA_FIRST_RULES_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&band);
    mo.write_back();
    let field = |m: &crate::comparison::MemorySnapshot, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = written.iter().filter(|e| field(&mo, e.0, e.1) != field(&mr, e.0, e.1)).map(|e| (e.0, field(&mo, e.0, e.1), field(&mr, e.0, e.1))).collect();
    SCOREBOARD.record(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (offset, original, rust)={:x?}", name, this.addr(), r, rr, difs));
    r
}

/// Creates the La Primera table of 20 clubs; first the clubs of the Spanish lower division move to the generic lower
/// division; prize fund of 250,000 per place; TV pays 5,000,000 to each club.
///
/// `SpaFirst::create_table()`. +0x3e = 20 and the table (malloc `exe::crt_malloc` of 0x49c; out of memory: the box with source
/// 0xa82844, line 0x250, and 0). Before filling, the clubs of the Spanish lower division ([0x9cf6b0]) move to the
/// generic lower division (+0x57 = the record of [0x9cf764]). Then the Primera clubs ([0x9cf624]) in database order,
/// each in the next row (League::init_table_row `init_table_row`: thiscall(league, row, club, 0, 0), ret 0x10), until full (not
/// filled: 0). Full: the prize fund (+0x28 = League::calc_prize_fund `calc_prize_fund` with the share of 250,000), +0xe1 = 1,
/// the TV money of 5,000,000 to each club (`pay_tv_rights`: thiscall(league, value, 0), ret 8) and 1.
/// Original: `SpaFirst::create_table` (0x84fc70, spa_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x84fc70 } (thiscall, ret 0, spa_first.cpp): `SpaFirst::create_table()`. +0x3e = 20 e a tabela (malloc 0x944e46
/// de 0x49c; sem memória: a caixa do fonte 0xa82844, linha 0x250, e 0). Antes de encher, os clubes da divisão de
/// baixo espanhola ([0x9cf6b0]) passam à divisão de baixo genérica (+0x57 = o registro de [0x9cf764]). Depois os
/// clubes da Primera ([0x9cf624]) na ordem do banco, cada um na linha seguinte (League::init_table_row 0x687430:
/// thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher (sem encher: 0). Cheia: o fundo de prêmios (+0x28 =
/// League::calc_prize_fund 0x682f70 pela cota de 250.000), +0xe1 = 1, a TV de 5.000.000 a cada clube (0x683010:
/// thiscall(liga, valor, 0), ret 8) e 1.
pub(crate) unsafe extern "thiscall" fn spa_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x14);
    let t = exe::crt_malloc(0x49c);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xa82844, 0x250);
        return 0;
    }
    // the Spanish lower division becomes the generic one
    let mut i: i16 = 0;
    let mut field = read_usize(G_CLUB_TABLE) + club::DIVISION;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(field);
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            write_usize(field, comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION));
        }
        i = i.wrapping_add(1);
        field = field.wrapping_add(club::SIZE);
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) {
            let row = k; // cqk5: row k receives the k-th club of the Primera
            // simplification: the original pushes the row in eax with the high bits of the competition id; 0x687430 only reads
            // o byte.
            exe::league_init_table_row(this, row as u32, club, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    crate::comparison::branch("cqk_5 primera fixtures");
    let f = exe::league_calc_prize_fund(this, 0x3d090); // cqk5: the Primera quota
    this.set_prize_fund(f);
    this.set_prize_mode(1); // cqk5: the table prize at the end of the league
    exe::league_pay_tv_rights(this, 0x4c4b40, 0); // cqk5: the Primera TV money
    1
}

/// Writes the club's current, best and worst possible position into La Primera statistics; places 17 and 18 become 16
/// for the best and 19 for the worst.
///
/// `SpaFirst::set_club_progress(club, stage, current, best, worst)`, virtual +0x68 (from `update_positions_and_status` for each table row):
/// in the club's entry in the statistics (Comp_Stats::find_club_entry `exe::comp_stats_find_club_entry`: thiscall(+8, club), ret 4) it writes
/// the current position (+0x73), the best possible (+0x74; 17 and 18 become 16) and the worst (+0x75; 17 and 18 become
/// 19): the 17 to 18 band of the Primera does not count as a band of its own [inference: positions 17 to 18 join the
/// neighbouring band above for the best and the one below for the worst].
/// Original: `SpaFirst::set_club_progress` (0x850880, spa_first.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x850880 } (thiscall, ret 0x14, spa_first.cpp): `SpaFirst::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 (da 0x684cd0 a cada linha da tabela): na entrada do clube nas estatísticas (Comp_Stats::
/// find_club_entry 0x4a4850: thiscall(+8, clube), ret 4) grava a posição atual (+0x73), a melhor possível (+0x74;
/// 17 e 18 viram 16) e a pior (+0x75; 17 e 18 viram 19): a faixa de 17 a 18 da Primera não conta como faixa própria
/// [inferência: as posições de 17 a 18 juntam-se à vizinha de cima na melhor e à de baixo na pior].
pub(crate) unsafe extern "thiscall" fn spa_first_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    let e = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(this + 8), club));
    let m = match best as u8 {
        0x11 | 0x12 => 0x10,
        v => v,
    };
    let p = match worst as u8 {
        0x11 | 0x12 => 0x13,
        v => v,
    };
    e.set_current_position(current as u8);
    e.set_best_position(m);
    e.set_worst_position(p);
}

/// the Primera: discipline 0x1a, only the league; finally, if the nation lacks bit 4 of +0x11c [inference: the lower
/// division playable], it creates the Spanish lower division (the B teams and the best of Segunda B) at +0xee
/// Original: `SpaFirst::SpaFirst` (0x84e130, spa_first.cpp, thiscall, ret 8).
///
/// [pt] { 0x84e130 } (thiscall, ret 8, spa_first.cpp, 606 bytes): `SpaFirst::SpaFirst(ano, registro)`: a Primera,
/// disciplina 0x1a, só a liga, e a divisão de baixo espanhola em +0xee.
pub(crate) unsafe extern "thiscall" fn construct_spa_first(this: usize, year: u32, record: usize) -> usize {
    construct_division_league(League(this), year, record, &SPAIN_PRIMERA_DIVISION)
}

