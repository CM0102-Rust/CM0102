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
pub(crate) type EngFirstScheduleRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets the dates and match slots of rounds `primeira`.. of table `t`, in the original routine's order.
///
/// For each round, the date (`league_round_set_date`: push 0; push year; push code; push week; push years; push month; push day; push
/// i; push t; add esp -> cdecl: value 0) and the slots (`store_round_match`: push 0; push c; push b; push a; push j; push i; push
/// t; add esp -> cdecl, value 0).
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn eng_first_set_league_rounds(t: usize, year: u32, first: i32, table: &[EngFirstScheduleRound]) {
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
pub(crate) struct EngFirstChristmasSchedule {
    pub(crate) comeco: &'static [EngFirstScheduleRound],
    pub(crate) reference: usize,
    pub(crate) adjust: i32,
    pub(crate) cases: [&'static [EngFirstScheduleRound]; 8],
    pub(crate) end: &'static [EngFirstScheduleRound],
    pub(crate) row: u32,
}

/// Sets the rounds around Christmas: start rounds, the middle block chosen by the weekday of the reference date, and
/// end rounds.
pub(crate) unsafe fn eng_first_set_christmas_rounds(t: usize, year: u32, n: &EngFirstChristmasSchedule, file: usize) {
    eng_first_set_league_rounds(t, year, 0, n.comeco);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let day = core::ptr::read_unaligned((t + n.reference * 0x41) as *const u16);
    let k = (day.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(n.adjust);
    let case = if k as u32 > 6 {
        show_error_at(file, n.row);
        7
    } else {
        k as usize
    };
    let middle = n.cases[case];
    eng_first_set_league_rounds(t, year, n.comeco.len() as i32, middle);
    eng_first_set_league_rounds(t, year, (n.comeco.len() + middle.len()) as i32, n.end);
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

pub(crate) const DIVISION_ONE_CHRISTMAS: EngFirstChristmasSchedule = EngFirstChristmasSchedule { comeco: &FIRST_DIVISION_START, reference: 22, adjust: 13, cases: [&FIRST_DIVISION_CHRISTMAS_0, &FIRST_DIVISION_CHRISTMAS_ROUNDS_1, &DIVISION_ONE_CHRISTMAS_ROUNDS_2, &FIRST_DIVISION_CHRISTMAS_3, &FIRST_DIVISION_CHRISTMAS_ROUNDS_4, &DIVISION_ONE_CHRISTMAS_ROUNDS_5, &FIRST_DIVISION_CHRISTMAS_6, &FIRST_DIVISION_CHRISTMAS_ROUNDS_DEFAULT], end: &FIRST_DIVISION_LATE_ROUNDS, row: 0x17c };
pub(crate) const DIVISION_ONE_PLAYOFF_0: EngCcCupCupRound = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
pub(crate) const FIRST_DIVISION_PLAYOFF_1: EngCcCupCupRound = cql10r((0x11, 4, 1, 3), (0x1c, 4, 1, 0, 2, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));
pub(crate) const PREMIERSHIP_CHRISTMAS: EngFirstChristmasSchedule = EngFirstChristmasSchedule { comeco: &PREMIERSHIP_EARLY_ROUNDS, reference: 17, adjust: 13, cases: [&PREMIERSHIP_CHRISTMAS_ROUNDS_0, &PREMIERSHIP_CHRISTMAS_ROUNDS_1, &PREMIERSHIP_CHRISTMAS_2, &PREMIERSHIP_CHRISTMAS_ROUNDS_3, &PREMIERSHIP_CHRISTMAS_ROUNDS_4, &PREMIERSHIP_CHRISTMAS_5, &PREMIERSHIP_CHRISTMAS_ROUNDS_6, &PREMIERSHIP_CHRISTMAS_ROUNDS_DEFAULT], end: &PREMIERSHIP_LATE_ROUNDS, row: 0x173 };

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
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 46;
            *extra = 0;
            let t = exe::crt_malloc(0xbae);
            if t == 0 {
                show_error_at(0x9c4374, 0xae);
                return 0;
            }
            crate::comparison::branch("cqp_5 english first division calendar");
            let year = (this.year() as u16) as u32;
            eng_first_set_christmas_rounds(t, year, &DIVISION_ONE_CHRISTMAS, 0x9c4374);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 2;
            *extra = 0xa0;
            let t = exe::crt_malloc(0xd0);
            if t == 0 {
                show_error_at(0x9c4374, 0x1ed);
                return 0;
            }
            crate::comparison::branch("cqp_5 english first division play-offs");
            let year = this.year() as i32 as u32;
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
pub(crate) unsafe fn eng_first_league_row_club(league: League, k: usize) -> usize {
    read_usize(league.table() + k * 0x3b)
}

/// Creates the play-off stage of the three divisions from 4 table rows: club list, round table, Cup_Stage in the first
/// stage slot, then Cup::set_up_round.
///
/// This is the body of `eng_first_create_playoff_stage`, `eng_second_create_play_off_stage` and `eng_third_create_play_off_stage`, which differ only in the lines, the source file and the stage's
/// +0x3a.
///
/// The list of `+0xbf` clubs (malloc `exe::crt_malloc` of 4 × the signed byte; when out of memory: the error box
/// `linha_sem_lista` and nothing else) takes the clubs of the 4 table `linhas`, in this order (the brackets: 0 × 1 and
/// 2 × 3). Virtual +0x3c (create_round_table: thiscall(this, stage 0, &rodadas, &extra, null), ret 0x10) builds the
/// round table; the new stage (operator new `exe::crt_operator_new` of 0xb2 bytes; Cup_Stage::Cup_Stage `construct_cup_stage`, thiscall with 15
/// arguments, ret 0x3c: the league, the count, the list, the rounds, +4, the round table, the word +0x40, 0, 1, `a10`
/// (or the virtual's extra), 0x14, 0, 0, 0, 0) goes into the first stage slot ([+0xc]); the list and the round table
/// are freed (free `exe::crt_free`).
///
/// Without a stage: the error box `linha_sem_fase`; otherwise Cup::set_up_round (`set_up_round`, thiscall(stage, 0), ret 4).
///
/// The original routine's SEH frame only covers the constructor (no destructor).
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
pub(crate) unsafe fn create_playoff_stage(this: League, rows: [usize; 4], file: usize, row_without_list: u32, row_without_stage: u32, a10: Option<u32>, name: &'static str) {
    let n = this.promotion_playoff_count() as i8 as i32;
    let list = exe::crt_malloc((n << 2) as u32);
    if list == 0 {
        return show_error_at(file, row_without_list);
    }
    for (k, &l) in rows.iter().enumerate() {
        core::ptr::write_unaligned((list + 4 * k) as *mut usize, eng_first_league_row_club(this, l));
    }
    crate::comparison::branch(name);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the count, the rounds and the word +0x40 in integer registers with garbage in the high 16 bits
    // high; the constructor only reads the words ([esp+0x230], [esp+0x238], [esp+0x244] and [esp+0x250] with mov ax/dx).
    let stage = if p != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        let a10 = a10.unwrap_or(extra as u16 as u32);
        construct(p, this.addr(), n as u16 as u32, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, a10, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned(this.stages() as *mut usize, stage);
    exe::crt_free(list);
    exe::crt_free(t);
    let stage = read_usize(this.stages());
    if stage == 0 {
        return show_error_at(file, row_without_stage);
    }
    exe::cup_set_up_round(stage, 0);
}

/// builds the play-off stage with the 6th, 3rd, 5th and 4th of the table (ties 6th v 3rd and 5th v 4th), the round
/// table from virtual +0x3c and +0x3a = 0xa0; puts the stage in the first slot (+0xc) and sets up the round (`set_up_round`);
/// out of memory, the error box (lines 0x2f0 and 0x306)
/// Original: `EngFirst::create_playoff_stage` (0x573080, eng_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x573080 } (thiscall, ret 0, eng_first.cpp): `EngFirst::create_playoff_stage()`, da fase seguinte da First
/// (0x572ff0, quando a fase passa de −1 a 0). As chaves: o 6º contra o 3º e o 5º contra o 4º (linhas 5, 2, 4 e 3);
/// +0x3a da fase = 0xa0. Fonte 0x9c4374, linhas 0x2f0 (sem lista) e 0x306 (sem fase).
pub(crate) unsafe extern "thiscall" fn eng_first_create_playoff_stage(this: usize) {
    create_playoff_stage(League(this), [5, 2, 4, 3], 0x9c4374, 0x2f0, 0x306, Some(0xa0), "cpz_1 mata-mata da first"); // cpz1: 6º × 3º e 5º × 4º da First
}

/// The comparer of the three play-off stage routines: runs only the original and checks the stage, the count and the 4
/// clubs (as a set).
///
/// The original allocates the list, the round table and the stage and builds the round (only it runs). The plan, made
/// beforehand: the clubs of the 4 rows. Afterwards, the stage of the first slot must exist, with +0x3e = the count
/// (+0xbf), +4 = the league's +4, and the first 4 clubs of its list (+0xa7, 6 bytes) equal to those of the plan (as a
/// set: the round draw `set_up_round` may reorder the ties).
///
/// [pt] O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das 4 linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = a conta
/// (+0xbf), +4 = o +4 da liga e os 4 primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como
/// conjunto: o sorteio da rodada 0x51c800 pode reordenar as chaves).
pub(crate) unsafe fn compare_playoff_stage(name: &str, this: League, rows: [usize; 4], original: impl FnOnce()) {
    let mut plan: [usize; 4] = [0; 4];
    for (k, &l) in rows.iter().enumerate() {
        plan[k] = eng_first_league_row_club(this, l);
    }
    let n = this.promotion_playoff_count() as i8 as i16;
    original();
    let stage = Cup(read_usize(this.stages()));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("{}({:#x}) sem fase", name, this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found: [usize; 4] = [0; 4];
    for (k, a) in found.iter_mut().enumerate() {
        *a = read_usize(list + 6 * k);
    }
    let (mut x, mut y) = (plan, found);
    x.sort_unstable();
    y.sort_unstable();
    let same = x == y && stage.team_count() == n && read_usize(stage.addr() + 4) == this.competition().addr();
    SCOREBOARD.record(same, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={} n={}", name, this.addr(), plan, stage.addr(), found, stage.team_count(), n));
}

/// at the last stage (+0x30 >= +0x2c - 1) the error box (line 0x2ce); otherwise +0x30 + 1 and, on reaching 0, the
/// play-offs (`eng_first_create_playoff_stage`)
/// Original: `EngFirst::advance_stage` (0x572ff0, eng_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x572ff0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::advance_stage()` (a virtual +0x28): fonte 0x9c4374,
/// linha 0x2ce; o mata-mata 0x573080.
pub(crate) unsafe extern "thiscall" fn eng_first_advance_stage(this: usize) {
    advance_league_stage(League(this), 0x9c4374, 0x2ce, 0x573080, "cpz_3 mata-mata da first")
}

/// the First Division: discipline 9, no minimum ground, the league and 1 stage (the promotion play-off)
/// Original: `EngFirst::EngFirst` (0x571680, eng_first.cpp, thiscall, ret 8).
///
/// [pt] { 0x571680 } (thiscall, ret 8, eng_first.cpp): `EngFirst::EngFirst(ano, registro)`. A First Division: 1 fase além da
/// liga (o play-off de acesso), sem estádio mínimo.
pub(crate) unsafe extern "thiscall" fn construct_eng_first(this: usize, year: u32, record: usize) -> usize {
    construct_english_division(League(this), year, record, &FIRST_DIVISION)
}

/// Sets up the rules of the English First Division (24 clubs): the top 2 go up directly, 3rd to 6th go to the promotion
/// play-offs, the bottom 3 go down to the Second; 5 substitutes and 3 substitutions; 2 rounds, 3 points.
///
/// `EngFirst::set_up_rules()`: the common rules, 2 promoted directly (`+0xbe`), 4 to the promotion play-off (`+0xbf`,
/// 3rd to 6th), no relegation play-off (`+0xc0`), 3 relegated (`+0xc1`), the Premier above (`+0x1c` = `[0x9cf5bc]`) and
/// the Second below (`+0x20` = `[0x9cf5c4]`), 5 substitutes on the bench (`+0x49`) and 3 substitutions (`+0x4a`).
/// Original: `EngFirst::set_up_rules` (0x572d90, eng_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x572d90 } (thiscall, ret 0, eng_first.cpp): `EngFirst::set_up_rules()`. A First: as comuns, 2 sobem direto
/// (+0xbe), 4 vão ao mata-mata de acesso (+0xbf, do 3º ao 6º), nenhum play-off de permanência (+0xc0), 3 caem
/// (+0xc1), a Premier acima (+0x1c = [0x9cf5bc]) e a Second abaixo (+0x20 = [0x9cf5c4]), 5 reservas no banco
/// (+0x49) e 3 substituições (+0x4a).
pub(crate) unsafe extern "thiscall" fn eng_first_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqi_4 first division rules");
    eng_prm_apply_league_rules(League(this), &eng_prm_first_division_rules())
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
    eng_prm_fill_league_table(League(this), 0x18, 0x9c4374, 0x22d, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, 0x11170, || {
        if is_after_1999() {
            eng_prm_pay_tv_rights(this, 0x59b011, comp_record_of(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION)); // cqi5: the TV money of those relegated from the Premier
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
pub(crate) unsafe extern "thiscall" fn eng_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_club_stage_status(this, club, stage as u8, current as u8, best as u8, worst as u8, 3, 6)
}

