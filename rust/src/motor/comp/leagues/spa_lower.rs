use super::*;

/// Describes one round of the 0x41-byte table.
///
/// It holds the date (day, month 0..11, years to add to the league year +0x40, the weekday the date is moved to), the
/// code of byte +4 ([inference] the half of the season) and the TV slots in order (weekday, half, how many matches; the
/// list ends with −1).
pub(crate) type SpaLowerLeagueRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets the dates and match slots of rounds `primeira`.. of table `t`, in the original routine's order.
///
/// For each round, the date (`league_round_set_date`: push 0; push year; push code; push week; push years; push month; push day; push
/// i; push t; add esp -> cdecl: value 0) and the slots (`store_round_match`: push 0; push c; push b; push a; push j; push i; push
/// t; add esp -> cdecl, value 0).
pub(crate) unsafe fn spa_lower_set_league_rounds(t: usize, year: u32, first: i32, table: &[SpaLowerLeagueRound]) {
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
pub(crate) struct SpaLowerChristmasSchedule {
    pub(crate) comeco: &'static [SpaLowerLeagueRound],
    pub(crate) reference: usize,
    pub(crate) adjust: i32,
    pub(crate) cases: [&'static [SpaLowerLeagueRound]; 8],
    pub(crate) end: &'static [SpaLowerLeagueRound],
    pub(crate) row: u32,
}

/// Writes the start rounds, then a middle block chosen by the weekday offset of a reference round from a December date
/// [inference: winter break], then the end rounds.
pub(crate) unsafe fn set_rounds_around_christmas(t: usize, year: u32, n: &SpaLowerChristmasSchedule, file: usize) {
    spa_lower_set_league_rounds(t, year, 0, n.comeco);
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
    spa_lower_set_league_rounds(t, year, n.comeco.len() as i32, middle);
    spa_lower_set_league_rounds(t, year, (n.comeco.len() + middle.len()) as i32, n.end);
}

/// a divisão de baixo espanhola: as rodadas antes do Natal (ano par)
pub(crate) const LOWER_EVEN_YEAR_START: [SpaLowerLeagueRound; 17] = [
    (30, 7, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 0: 30/ago
    (6, 8, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 1: 6/set
    (13, 8, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 2: 13/set
    (20, 8, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 3: 20/set
    (27, 8, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 4: 27/set
    (4, 9, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 5: 4/out
    (11, 9, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 6: 11/out
    (18, 9, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 7: 18/out
    (25, 9, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 8: 25/out
    (1, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 9: 1/nov
    (8, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 10: 8/nov
    (15, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 11: 15/nov
    (22, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 12: 22/nov
    (29, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 13: 29/nov
    (6, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 14: 6/dez
    (13, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 15: 13/dez
    (20, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 16: 20/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 17/dez (ano par)
pub(crate) const LOWER_EVEN_YEAR_CHRISTMAS_0: [SpaLowerLeagueRound; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 18/dez (ano par)
pub(crate) const LOWER_EVEN_CHRISTMAS_ROUNDS_1: [SpaLowerLeagueRound; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 19/dez (ano par)
pub(crate) const LOWER_EVEN_YEAR_CHRISTMAS_ROUNDS_2: [SpaLowerLeagueRound; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 20/dez (ano par)
pub(crate) const LOWER_EVEN_YEAR_CHRISTMAS_3: [SpaLowerLeagueRound; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 21/dez (ano par)
pub(crate) const LOWER_EVEN_CHRISTMAS_ROUNDS_4: [SpaLowerLeagueRound; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 28/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 22/dez (ano par)
pub(crate) const LOWER_EVEN_YEAR_CHRISTMAS_ROUNDS_5: [SpaLowerLeagueRound; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 29/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 23/dez (ano par)
pub(crate) const LOWER_EVEN_YEAR_CHRISTMAS_6: [SpaLowerLeagueRound; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 16 fora de 17 a 23/dez: a caixa) (ano par)
pub(crate) const LOWER_EVEN_CHRISTMAS_ROUNDS_DEFAULT: [SpaLowerLeagueRound; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// a divisão de baixo espanhola: as rodadas depois do Natal (ano par)
pub(crate) const LOWER_EVEN_LATE_ROUNDS: [SpaLowerLeagueRound; 24] = [
    (3, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 18: 3/jan (+1 ano)
    (10, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 19: 10/jan (+1 ano)
    (17, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 20: 17/jan (+1 ano)
    (24, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 21: 24/jan (+1 ano)
    (31, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 22: 31/jan (+1 ano)
    (7, 1, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 23: 7/fev (+1 ano)
    (14, 1, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 24: 14/fev (+1 ano)
    (21, 1, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 25: 21/fev (+1 ano)
    (28, 1, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 26: 28/fev (+1 ano)
    (7, 2, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 27: 7/mar (+1 ano)
    (14, 2, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 28: 14/mar (+1 ano)
    (21, 2, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 29: 21/mar (+1 ano)
    (28, 2, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 30: 28/mar (+1 ano)
    (4, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 31: 4/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 32: 11/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 33: 11/abr (+1 ano)
    (18, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 34: 18/abr (+1 ano)
    (25, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 35: 25/abr (+1 ano)
    (2, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 36: 2/mai (+1 ano)
    (9, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 37: 9/mai (+1 ano)
    (16, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 38: 16/mai (+1 ano)
    (23, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 39: 23/mai (+1 ano)
    (30, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 40: 30/mai (+1 ano)
    (6, 5, 1, 6, 1, &[(-1, -1, -1)]), // 41: 6/jun (+1 ano)
];

/// a divisão de baixo espanhola: as rodadas antes do Natal (ano impar)
pub(crate) const LOWER_ODD_EARLY_ROUNDS: [SpaLowerLeagueRound; 20] = [
    (16, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 0: 16/ago
    (23, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 1: 23/ago
    (26, 7, 0, 2, 2, &[(-1, -1, -1)]), // 2: 26/ago
    (30, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 3: 30/ago
    (2, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 4: 2/set
    (13, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 5: 13/set
    (20, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 6: 20/set
    (27, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 7: 27/set
    (4, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 8: 4/out
    (11, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 9: 11/out
    (18, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 10: 18/out
    (25, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 11: 25/out
    (1, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 12: 1/nov
    (8, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 13: 8/nov
    (15, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 14: 15/nov
    (22, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 22/nov
    (29, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 29/nov
    (6, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 17: 6/dez
    (13, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 18: 13/dez
    (20, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 19: 20/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 17/dez (ano impar)
pub(crate) const LOWER_ODD_CHRISTMAS_ROUNDS_0: [SpaLowerLeagueRound; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 18/dez (ano impar)
pub(crate) const LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_1: [SpaLowerLeagueRound; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 19/dez (ano impar)
pub(crate) const LOWER_ODD_YEAR_CHRISTMAS_2: [SpaLowerLeagueRound; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 20/dez (ano impar)
pub(crate) const LOWER_ODD_CHRISTMAS_ROUNDS_3: [SpaLowerLeagueRound; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 27/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 21/dez (ano impar)
pub(crate) const LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_4: [SpaLowerLeagueRound; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 28/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 22/dez (ano impar)
pub(crate) const LOWER_ODD_YEAR_CHRISTMAS_5: [SpaLowerLeagueRound; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 29/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 23/dez (ano impar)
pub(crate) const LOWER_ODD_CHRISTMAS_ROUNDS_6: [SpaLowerLeagueRound; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 19 fora de 17 a 23/dez: a caixa) (ano impar)
pub(crate) const LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_DEFAULT: [SpaLowerLeagueRound; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 27/dez
];

/// a divisão de baixo espanhola: as rodadas depois do Natal (ano impar)
pub(crate) const LOWER_ODD_YEAR_LATE_ROUNDS: [SpaLowerLeagueRound; 21] = [
    (3, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 21: 3/jan (+1 ano)
    (10, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 22: 10/jan (+1 ano)
    (17, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 23: 17/jan (+1 ano)
    (24, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 24: 24/jan (+1 ano)
    (31, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 25: 31/jan (+1 ano)
    (7, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 26: 7/fev (+1 ano)
    (14, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 27: 14/fev (+1 ano)
    (21, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 28: 21/fev (+1 ano)
    (28, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 29: 28/fev (+1 ano)
    (7, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 30: 7/mar (+1 ano)
    (14, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 31: 14/mar (+1 ano)
    (21, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 32: 21/mar (+1 ano)
    (28, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 33: 28/mar (+1 ano)
    (4, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 34: 4/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 35: 11/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 36: 11/abr (+1 ano)
    (18, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 37: 18/abr (+1 ano)
    (25, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 38: 25/abr (+1 ano)
    (2, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 39: 2/mai (+1 ano)
    (9, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 40: 9/mai (+1 ano)
    (16, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 41: 16/mai (+1 ano)
];

pub(crate) const LOWER_EVEN_YEAR_CHRISTMAS: SpaLowerChristmasSchedule = SpaLowerChristmasSchedule { comeco: &LOWER_EVEN_YEAR_START, reference: 16, adjust: 17, cases: [&LOWER_EVEN_YEAR_CHRISTMAS_0, &LOWER_EVEN_CHRISTMAS_ROUNDS_1, &LOWER_EVEN_YEAR_CHRISTMAS_ROUNDS_2, &LOWER_EVEN_YEAR_CHRISTMAS_3, &LOWER_EVEN_CHRISTMAS_ROUNDS_4, &LOWER_EVEN_YEAR_CHRISTMAS_ROUNDS_5, &LOWER_EVEN_YEAR_CHRISTMAS_6, &LOWER_EVEN_CHRISTMAS_ROUNDS_DEFAULT], end: &LOWER_EVEN_LATE_ROUNDS, row: 0x128 };
pub(crate) const LOWER_ODD_YEAR_CHRISTMAS: SpaLowerChristmasSchedule = SpaLowerChristmasSchedule { comeco: &LOWER_ODD_EARLY_ROUNDS, reference: 19, adjust: 17, cases: [&LOWER_ODD_CHRISTMAS_ROUNDS_0, &LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_1, &LOWER_ODD_YEAR_CHRISTMAS_2, &LOWER_ODD_CHRISTMAS_ROUNDS_3, &LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_4, &LOWER_ODD_YEAR_CHRISTMAS_5, &LOWER_ODD_CHRISTMAS_ROUNDS_6, &LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_DEFAULT], end: &LOWER_ODD_YEAR_LATE_ROUNDS, row: 0x237 };

/// the Spanish lower division schedule (background): 42 rounds, nearly all on Sunday (day 6); in odd years (2001/02)
/// from 16 August to 16 May, in even years from 30 August to 6 June; in each, one fixed-date Christmas round chosen by
/// the day the mid-December round falls on
/// Original: `SpaLower::create_schedule` (0x851260, spa_lower.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn spa_lower_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 42;
            *extra = 0;
            let t = exe::crt_malloc(0xaaa);
            if t == 0 {
                show_error_at(0xa82880, 0x8b);
                return 0;
            }
            covered!("cqq_5 spanish lower division calendar");
            let year = (this.year() as u16) as u32;
            if (year as i16) % 2 != 0 {
                set_rounds_around_christmas(t, year, &LOWER_ODD_YEAR_CHRISTMAS, 0xa82880);
            } else {
                set_rounds_around_christmas(t, year, &LOWER_EVEN_YEAR_CHRISTMAS, 0xa82880);
            }
            t
        }
        _ => 0,
    }
}

/// Competition record (0x6b bytes in [0xae23d0]) of key id [g].
pub(crate) unsafe fn comp_record_of_key(g: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(g).wrapping_mul(0x6b))
}

/// Is the club's competition (+0x57) the one with key id [g]?
pub(crate) unsafe fn spa_lower_is_in_comp(c: Club, g: usize) -> bool {
    let comp = c.division().addr();
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// `SpaLower::SpaLower(year)` (SEH frame only on the exception path): the Spanish lower division, a background league
/// with no discipline (created by the Primera when the nation does not have bit 4 of +0x11c). The base (League::League
/// `exe::new_league`: mov ecx -> thiscall, ret 0), the vtable 0x96fb1c, the year (`.year()`), discipline rule
/// `discipline_rule::NONE` (0xff, none) and the record of the competition of id `KEY_CLUB_COMP_SPANISH_LOWER_DIVISION`
/// (`league::COMPETITION`). Tries to load the saved state (`exe::league_load_from_temp_file`: push 1 -> thiscall, ret
/// 4); otherwise, just the league (`.stage_index()` = `stage_index::MAIN` (0xff), `.current_stage()` = −1,
/// `.stage_count()` = 0), the rules (`exe::spalower_set_up_rules`), the table (`exe::spalower_create_table`) and the
/// fixtures (League::create_fixtures `exe::league_create_fixtures`), all mov ecx -> thiscall, ret 0 (no stats nor
/// predicted positions); each failure, the error box of its line (0x3f, 0x46, 0x4d of source 0xa82880) and the end.
/// Returns this.
/// Original: `SpaLower::SpaLower` (0x851020, spa_lower.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_spa_lower(this: usize, year: u32) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr(), 0x96fb1c);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(discipline_rule::NONE);
    write_usize(this.addr() + league::COMPETITION, comp_record_of_key(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION));
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("cqm_6 new lower division");
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(0);
    if exe::spalower_set_up_rules(this) == 0 {
        show_error_at(0xa82880, 0x3f);
        return this.addr();
    }
    if exe::spalower_create_table(this) == 0 {
        show_error_at(0xa82880, 0x46);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0xa82880, 0x4d);
    }
    this.addr()
}

/// Is club c the B team of a parent club (club_get_linked_b_team) that plays in competition [g]?
pub(crate) unsafe fn is_b_team_of_comp_club(c: usize, g: usize) -> bool {
    let mut eh_a: u32 = 0;
    let p = exe::club_get_linked_b_team(c, &mut eh_a, 1);
    p != 0 && eh_a == 0 && spa_lower_is_in_comp(Club(p), g)
}

/// The candidates of the Spanish lower division, from the original routine's local record of 100.
///
/// The clubs (all of them, in database order, up to 100) of the competitions with id [`0x9cf62c`], [`0x9cf630`],
/// [`0x9cf634`], [`0x9cf638`] [inference: the 4 Segunda B groups] or, if Spanish, of the one with id [`0x9cf764`]
/// [inference: the generic lower division], that are neither a B team nor anyone's A team (club_get_linked_b_team
/// returns 0).
pub(crate) unsafe fn spanish_lower_candidates() -> Vec<usize> {
    let mut v = Vec::new();
    let n = read_usize(G_CLUB_COUNT) as i32;
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n && v.len() < 100 {
        let c = Club(base + (i as u16 as usize) * club::SIZE);
        let comp = c.division().addr();
        let nation = c.nation().addr();
        if comp != 0 && nation != 0 {
            let id = read_usize(comp);
            let group = [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4].iter().any(|&g| id == read_usize(g));
            if group || (read_usize(nation) == read_usize(KEY_NATION_SPAIN) && id == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) {
                let mut eh_a: u32 = 0;
                if exe::club_get_linked_b_team(c, &mut eh_a, 1) == 0 {
                    v.push(c.addr());
                }
            }
        }
        i = i.wrapping_add(1);
    }
    v
}

/// Calls por(club, enters) for Spanish clubs outside Primera/Segunda; enters when its parent plays in competition [g]
/// and rows remain (counts k).
pub(crate) unsafe fn for_each_spanish_b_team(g: usize, k: &mut u8, rows: i16, mut por: impl FnMut(usize, bool)) {
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = Club(base + (i as u16 as usize) * club::SIZE);
        let comp = c.division().addr();
        let nation = c.nation().addr();
        if comp != 0 && nation != 0 && read_usize(nation) == read_usize(KEY_NATION_SPAIN) && read_usize(comp) != read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && read_usize(comp) != read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) {
            let mut eh_a: u32 = 0;
            let p = Club(exe::club_get_linked_b_team(c, &mut eh_a, 1));
            let enters = !p.is_null() && eh_a == 0 && p.division().addr() != 0 && (*k as i8 as i16) < rows && read_usize(p.division().addr()) == read_usize(g);
            if enters {
                *k = k.wrapping_add(1);
            }
            por(c.addr(), enters);
        }
        i = i.wrapping_add(1);
    }
}

/// the lower division has 22 clubs: first the Spanish B teams whose parent club is in the Primera, then those of the
/// Segunda (database order); the rest come from the Segunda B clubs [inference: competitions
/// `KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1`..`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4`] and the Spanish generic
/// division that are not B teams, ordered by reputation with the best shuffled; each club placed keeps the competition
/// at `club::RESERVE_DIVISION`, but the Segunda B-team pass zeroes `club::RESERVE_DIVISION` of every Spanish side not
/// entering it, including the Primera ones already placed [observed in the comparison]
/// Original: `SpaLower::create_table` (0x8533d0, spa_lower.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_lower_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x16);
    let t = exe::crt_malloc(0x512);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xa82880, 0x2ac);
        return 0;
    }
    // the clubs of this division move to the generic one
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * club::SIZE;
        if spa_lower_is_in_comp(Club(c), KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            write_usize(c + club::DIVISION, comp_record_of_key(KEY_CLUB_COMP_A_LOWER_DIVISION));
        }
        i = i.wrapping_add(1);
    }
    let mut k: u8 = 0;
    for g in [KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION] { // cqm6: the B teams of the Primera and the Segunda
        let rows = this.club_count();
        // the row count for the condition advances together with `k`
        let mut kk = k;
        for_each_spanish_b_team(g, &mut kk, rows, |c, entra| {
            if entra {
                exe::league_init_table_row(this, k as u32, c, 0, 0);
                k = k.wrapping_add(1);
                write_usize(c + club::RESERVE_DIVISION, this.competition().addr());
            } else {
                write_usize(c + club::RESERVE_DIVISION, 0);
            }
        });
    }
    covered!("cqm_6 lower division fixtures");
    let mut list = spanish_lower_candidates();
    let how_many = list.len() as u16 as u32;
    let missing = (this.club_count() as u16).wrapping_sub(k as i8 as i16 as u16) as u32; // cqm6: quantos faltam
    list.resize(100, 0);
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, how_many, missing);
    let mut j: i16 = 0;
    while j < how_many as u16 as i16 && (k as i8 as i16) < this.club_count() {
        let c = list[j as usize];
        exe::league_init_table_row(this, k as u32, c, 0, 0);
        k = k.wrapping_add(1);
        write_usize(c + club::RESERVE_DIVISION, this.competition().addr());
        j += 1;
    }
    (k as i8 as i16 == this.club_count()) as u32
}

/// the lower division (22: B teams and the best minor clubs by reputation): 2 go up directly and 2 go to the promotion
/// play-off, 4 go down; the league above stored is La Primera (as in the original)
/// Original: `SpaLower::set_up_rules` (0x853790, spa_lower.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_lower_set_up_rules(this: usize) -> u32 {
    covered!("cqk_4 lower division rules");
    spa_first_apply_league_rules(League(this), &lower_division_rules())
}

