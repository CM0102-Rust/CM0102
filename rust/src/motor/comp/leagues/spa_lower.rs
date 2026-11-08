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
pub(crate) type SpaLowerLeagueRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets the dates and match slots of rounds `primeira`.. of table `t`, in the original routine's order.
///
/// For each round, the date (0x68a160: push 0; push year; push code; push week; push years; push month; push day; push
/// i; push t; add esp -> cdecl: value 0) and the slots (0x68a1c0: push 0; push c; push b; push a; push j; push i; push
/// t; add esp -> cdecl, value 0).
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn spa_lower_set_league_rounds(t: usize, year: u32, first: i32, table: &[SpaLowerLeagueRound]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in table.iter().enumerate() {
        let i = first + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, year, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            exe::league_round_set_match_slot(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// The main stage with the Christmas rounds.
///
/// The opening rounds in sequence; then the date of 1 December of the year (Date::Date 0x549f70: push −1; push year;
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
pub(crate) struct SpaLowerChristmasSchedule {
    pub(crate) comeco: &'static [SpaLowerLeagueRound],
    pub(crate) referencia: usize,
    pub(crate) ajuste: i32,
    pub(crate) casos: [&'static [SpaLowerLeagueRound]; 8],
    pub(crate) fim: &'static [SpaLowerLeagueRound],
    pub(crate) linha: u32,
}

/// Writes the start rounds, then a middle block chosen by the weekday offset of a reference round from a December date
/// [inference: winter break], then the end rounds.
pub(crate) unsafe fn set_rounds_around_christmas(t: usize, year: u32, n: &SpaLowerChristmasSchedule, file: usize) {
    let construct: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    spa_lower_set_league_rounds(t, year, 0, n.comeco);
    let mut d = [0u16; 4];
    construct(&mut d, 1, 0xb, year, 0xffff_ffff);
    let day = core::ptr::read_unaligned((t + n.referencia * 0x41) as *const u16);
    let k = (day.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(n.ajuste);
    let case = if k as u32 > 6 {
        show_error_at(file, n.linha);
        7
    } else {
        k as usize
    };
    let middle = n.casos[case];
    spa_lower_set_league_rounds(t, year, n.comeco.len() as i32, middle);
    spa_lower_set_league_rounds(t, year, (n.comeco.len() + middle.len()) as i32, n.fim);
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

pub(crate) const LOWER_EVEN_YEAR_CHRISTMAS: SpaLowerChristmasSchedule = SpaLowerChristmasSchedule { comeco: &LOWER_EVEN_YEAR_START, referencia: 16, ajuste: 17, casos: [&LOWER_EVEN_YEAR_CHRISTMAS_0, &LOWER_EVEN_CHRISTMAS_ROUNDS_1, &LOWER_EVEN_YEAR_CHRISTMAS_ROUNDS_2, &LOWER_EVEN_YEAR_CHRISTMAS_3, &LOWER_EVEN_CHRISTMAS_ROUNDS_4, &LOWER_EVEN_YEAR_CHRISTMAS_ROUNDS_5, &LOWER_EVEN_YEAR_CHRISTMAS_6, &LOWER_EVEN_CHRISTMAS_ROUNDS_DEFAULT], fim: &LOWER_EVEN_LATE_ROUNDS, linha: 0x128 };
pub(crate) const LOWER_ODD_YEAR_CHRISTMAS: SpaLowerChristmasSchedule = SpaLowerChristmasSchedule { comeco: &LOWER_ODD_EARLY_ROUNDS, referencia: 19, ajuste: 17, casos: [&LOWER_ODD_CHRISTMAS_ROUNDS_0, &LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_1, &LOWER_ODD_YEAR_CHRISTMAS_2, &LOWER_ODD_CHRISTMAS_ROUNDS_3, &LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_4, &LOWER_ODD_YEAR_CHRISTMAS_5, &LOWER_ODD_CHRISTMAS_ROUNDS_6, &LOWER_ODD_YEAR_CHRISTMAS_ROUNDS_DEFAULT], fim: &LOWER_ODD_YEAR_LATE_ROUNDS, linha: 0x237 };

/// the Spanish lower division schedule (background): 42 rounds, nearly all on Sunday (day 6); in odd years (2001/02)
/// from 16 August to 16 May, in even years from 30 August to 6 June; in each, one fixed-date Christmas round chosen by
/// the day the mid-December round falls on
/// Original: `SpaLower::create_schedule` (0x851260, spa_lower.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x851260 } (thiscall, ret 0x10, spa_lower.cpp, 8.495 bytes, método +0x3c): `SpaLower::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1, 42 rodadas, a tabela pela paridade do ano +0x40, cada uma com a rodada
/// do Natal; as outras devolvem 0. Fonte 0xa82980.
pub(crate) unsafe extern "thiscall" fn spa_lower_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 42;
            *extra = 0;
            let t = alloc(0xaaa);
            if t == 0 {
                show_error_at(0xa82880, 0x8b);
                return 0;
            }
            crate::comparison::branch("cqq_5 calendario da divisao de baixo espanhola");
            let year = core::ptr::read_unaligned((this + league::YEAR) as *const u16) as u32;
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
///
/// [pt] O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes).
pub(crate) unsafe fn comp_record_of_key(g: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(g).wrapping_mul(0x6b))
}

/// Is the club's competition (+0x57) the one with key id [g]?
///
/// [pt] A competição do clube (+0x57, o id no +0) é a de id [g]?
pub(crate) unsafe fn spa_lower_is_in_comp(c: usize, g: usize) -> bool {
    let comp = read_usize(c + club::DIVISION);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// `SpaLower::SpaLower(year)` (SEH frame only on the exception path): the Spanish lower division, a background league
/// with no discipline (created by the Primera when the nation does not have bit 4 of +0x11c).
///
/// The base (League::League 0x682200: mov ecx -> thiscall, ret 0), the vtable 0x96fb1c, the year (+0x40), discipline
/// rule 0xff (none) and the record of the competition of id [0x9cf6b0] (+4). Tries to load the saved state (0x687b10:
/// push 1 -> thiscall, ret 4); otherwise, just the league (+0x44 = 0xff, +0x30 = −1, +0x2c = 0), the rules (0x853790),
/// the table (0x8533d0) and the fixtures (League::create_fixtures 0x6835c0), all mov ecx -> thiscall, ret 0 (no stats
/// nor predicted positions); each failure, the error box of its line (0x3f, 0x46, 0x4d of source 0xa82880) and the end.
/// Returns this.
/// Original: `SpaLower::SpaLower` (0x851020, spa_lower.cpp, thiscall, ret 4).
///
/// [pt] { 0x851020 } (thiscall, ret 4, spa_lower.cpp, quadro SEH só no caminho de exceção): `SpaLower::SpaLower(ano)`, a
/// divisão de baixo espanhola (criada pela Primera quando a nação não tem o bit 4 de +0x11c): a base (League::League
/// 0x682200: mov ecx -> thiscall, ret 0), a vtable 0x96fb1c, o ano (+0x40), a regra de disciplina 0xff (nenhuma) e o
/// registro da competição de id [0x9cf6b0] (+4). Tenta carregar o estado (0x687b10: push 1 -> thiscall, ret 4);
/// senão, só a liga (+0x44 = 0xff, +0x30 = −1, +0x2c = 0), as regras (0x853790), a tabela (0x8533d0) e os jogos
/// (League::create_fixtures 0x6835c0), todos mov ecx -> thiscall, ret 0; cada falha, a caixa da sua linha (0x3f,
/// 0x46, 0x4d do fonte 0xa82880) e fim. Devolve this.
pub(crate) unsafe extern "thiscall" fn construct_spa_lower(this: usize, year: u32) -> usize {
    exe::new_league(this);
    write_usize(this, 0x96fb1c);
    core::ptr::write_unaligned((this + league::YEAR) as *mut u16, year as u16);
    *((this + league::DISCIPLINE_RULE) as *mut u8) = 0xff;
    write_usize(this + 4, comp_record_of_key(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION));
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqm_6 divisao de baixo nova");
    *((this + league::STAGE_INDEX) as *mut u8) = 0xff;
    write_usize(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_usize(this + league::STAGE_COUNT, 0);
    if exe::spalower_set_up_rules(this) == 0 {
        show_error_at(0xa82880, 0x3f);
        return this;
    }
    if exe::spalower_create_table(this) == 0 {
        show_error_at(0xa82880, 0x46);
        return this;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0xa82880, 0x4d);
    }
    this
}

/// Is club c the B team of a parent club (club_get_linked_b_team) that plays in competition [g]?
///
/// [pt] O clube de cima do clube `c` (club_get_linked_b_team 0x540a50: push 1; push &eh_a; push c; add esp, 0xc -> cdecl),
/// se `c` é o time B dele (eh_a = 0) e ele está na competição de id [g].
pub(crate) unsafe fn is_b_team_of_comp_club(c: usize, g: usize) -> bool {
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut eh_a: u32 = 0;
    let p = partner(c, &mut eh_a, 1);
    p != 0 && eh_a == 0 && spa_lower_is_in_comp(p, g)
}

/// The candidates of the Spanish lower division, from the original routine's local record of 100.
///
/// The clubs (all of them, in database order, up to 100) of the competitions with id [`0x9cf62c`], [`0x9cf630`],
/// [`0x9cf634`], [`0x9cf638`] [inference: the 4 Segunda B groups] or, if Spanish, of the one with id [`0x9cf764`]
/// [inference: the generic lower division], that are neither a B team nor anyone's A team (club_get_linked_b_team
/// returns 0).
///
/// [pt] Os candidatos da divisão de baixo espanhola, do registro local de 100 da original: os clubes (todos, na ordem do
/// banco, até 100) das competições de id [0x9cf62c], [0x9cf630], [0x9cf634], [0x9cf638] [inferência: os 4 grupos da
/// Segunda B] ou, se espanhóis, da de id [0x9cf764] [inferência: a divisão genérica de baixo], que não são time B nem
/// time A de ninguém (club_get_linked_b_team devolve 0).
pub(crate) unsafe fn spanish_lower_candidates() -> Vec<usize> {
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut v = Vec::new();
    let n = read_usize(G_CLUB_COUNT) as i32;
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n && v.len() < 100 {
        let c = base + (i as u16 as usize) * club::SIZE;
        let comp = read_usize(c + club::DIVISION);
        let nation = read_usize(c + club::NATION);
        if comp != 0 && nation != 0 {
            let id = read_usize(comp);
            let group = [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4].iter().any(|&g| id == read_usize(g));
            if group || (read_usize(nation) == read_usize(KEY_NATION_SPAIN) && id == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) {
                let mut eh_a: u32 = 0;
                if partner(c, &mut eh_a, 1) == 0 {
                    v.push(c);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    v
}

/// Calls por(club, enters) for Spanish clubs outside Primera/Segunda; enters when its parent plays in competition [g]
/// and rows remain (counts k).
///
/// [pt] Os times B espanhóis (fora da Primera e da Segunda) cujo clube de cima está na competição de id [g], na ordem do
/// banco, até 22 linhas (`k` já usadas). Os últimos `2 × [0xae2350]` clubes do banco ficam de fora [inferência: os
/// clubes criados depois, sem divisão].
pub(crate) unsafe fn for_each_spanish_b_team(g: usize, k: &mut u8, rows: i16, mut por: impl FnMut(usize, bool)) {
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * club::SIZE;
        let comp = read_usize(c + club::DIVISION);
        let nation = read_usize(c + club::NATION);
        if comp != 0 && nation != 0 && read_usize(nation) == read_usize(KEY_NATION_SPAIN) && read_usize(comp) != read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && read_usize(comp) != read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) {
            let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
            let mut eh_a: u32 = 0;
            let p = partner(c, &mut eh_a, 1);
            let enters = p != 0 && eh_a == 0 && read_usize(p + club::DIVISION) != 0 && (*k as i8 as i16) < rows && read_usize(read_usize(p + club::DIVISION)) == read_usize(g);
            if enters {
                *k = k.wrapping_add(1);
            }
            por(c, enters);
        }
        i = i.wrapping_add(1);
    }
}

/// the lower division has 22 clubs: first the Spanish B teams whose parent club is in the Primera, then those of the
/// Segunda (database order); the rest come from the Segunda B clubs [inference: competitions [0x9cf62c..0x9cf638]] and
/// the Spanish generic division that are not B teams, ordered by reputation with the best shuffled; each club placed
/// keeps the competition at +0x60, but the Segunda B-team pass zeroes +0x60 of every Spanish side not entering it,
/// including the Primera ones already placed [observed in the comparison]
/// Original: `SpaLower::create_table` (0x8533d0, spa_lower.cpp, thiscall, ret 0).
///
/// [pt] { 0x8533d0 } (thiscall, ret 0, spa_lower.cpp, 947 bytes): `SpaLower::create_table()`: a divisão de baixo tem 22
/// clubes (+0x3e), a tabela (malloc de 0x512; sem memória, a caixa da linha 0x2ac e 0). Primeiro, os clubes que estão
/// nela (a competição de id [0x9cf6b0]) passam à de id [0x9cf764] [inferência: a divisão genérica]. Depois enche, na
/// ordem do banco: os times B espanhóis cujo clube de cima está na Primera, depois os da Segunda (cada um com +0x60 =
/// o registro desta competição; os outros times espanhóis de fora das duas, com +0x60 = 0); o resto com os
/// candidatos (spanish_lower_candidates), ordenados pela reputação e com os 3 × faltam primeiros embaralhados
/// (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push faltam; push quantos; push lista; add esp, 0xc -> cdecl; sorteia),
/// cada um com +0x60 = o registro. As linhas: League::init_table_row 0x687430 (push 0; push 0; push clube; push eax
/// (al = a linha, os bits de cima são lixo de outra conta; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret
/// 0x10). Devolve se encheu.
pub(crate) unsafe extern "thiscall" fn spa_lower_create_table(this: usize) -> u32 {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    core::ptr::write_unaligned((this + league::CLUB_COUNT) as *mut i16, 0x16);
    let t = alloc(0x512);
    write_usize(this + league::TABLE, t);
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
        if spa_lower_is_in_comp(c, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            write_usize(c + club::DIVISION, comp_record_of_key(KEY_CLUB_COMP_A_LOWER_DIVISION));
        }
        i = i.wrapping_add(1);
    }
    let mut k: u8 = 0;
    for g in [KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION] { // cqm6: the B teams of the Primera and the Segunda
        let rows = read_i16(this + league::CLUB_COUNT);
        // the row count for the condition advances together with `k`
        let mut kk = k;
        for_each_spanish_b_team(g, &mut kk, rows, |c, entra| {
            if entra {
                iniciar(this, k as u32, c, 0, 0);
                k = k.wrapping_add(1);
                write_usize(c + club::RESERVE_DIVISION, read_usize(this + 4));
            } else {
                write_usize(c + club::RESERVE_DIVISION, 0);
            }
        });
    }
    crate::comparison::branch("cqm_6 tabela da divisao de baixo");
    let mut list = spanish_lower_candidates();
    let how_many = list.len() as u16 as u32;
    let missing = (read_i16(this + league::CLUB_COUNT) as u16).wrapping_sub(k as i8 as i16 as u16) as u32; // cqm6: quantos faltam
    list.resize(100, 0);
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, how_many, missing);
    let mut j: i16 = 0;
    while j < how_many as u16 as i16 && (k as i8 as i16) < read_i16(this + league::CLUB_COUNT) {
        let c = list[j as usize];
        iniciar(this, k as u32, c, 0, 0);
        k = k.wrapping_add(1);
        write_usize(c + club::RESERVE_DIVISION, read_usize(this + 4));
        j += 1;
    }
    (k as i8 as i16 == read_i16(this + league::CLUB_COUNT)) as u32
}

/// the lower division (22: B teams and the best minor clubs by reputation): 2 go up directly and 2 go to the promotion
/// play-off, 4 go down; the league above stored is La Primera (as in the original)
/// Original: `SpaLower::set_up_rules` (0x853790, spa_lower.cpp, thiscall, ret 0).
///
/// [pt] { 0x853790 } (thiscall, ret 0, spa_lower.cpp): `SpaLower::set_up_rules()`. A divisão de baixo (os times B e os
/// melhores das divisões menores): as comuns, 2 turnos, 3 pontos, 2 sobem direto (+0xbe) e 2 vão ao play-off de
/// acesso (+0xbf), 4 caem (+0xc1), a Primera como liga de cima ([0x9cf624], assim no original) e nenhuma abaixo.
pub(crate) unsafe extern "thiscall" fn spa_lower_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqk_4 regras da divisao de baixo");
    spa_first_apply_league_rules(this, &lower_division_rules())
}

