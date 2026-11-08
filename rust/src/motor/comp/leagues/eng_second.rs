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
pub(crate) type EngSecondScheduleRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets the dates and match slots of rounds `primeira`.. of table `t`, in the original routine's order.
///
/// For each round, the date (0x68a160: push 0; push year; push code; push week; push years; push month; push day; push
/// i; push t; add esp -> cdecl: value 0) and the slots (0x68a1c0: push 0; push c; push b; push a; push j; push i; push
/// t; add esp -> cdecl, value 0).
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn eng_second_set_league_rounds(t: usize, year: u32, first: i32, table: &[EngSecondScheduleRound]) {
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
pub(crate) struct EngSecondChristmasSchedule {
    pub(crate) comeco: &'static [EngSecondScheduleRound],
    pub(crate) referencia: usize,
    pub(crate) ajuste: i32,
    pub(crate) casos: [&'static [EngSecondScheduleRound]; 8],
    pub(crate) fim: &'static [EngSecondScheduleRound],
    pub(crate) linha: u32,
}

/// Sets the rounds around Christmas: start rounds, the middle block chosen by the weekday of the reference date, and
/// end rounds.
pub(crate) unsafe fn eng_second_set_christmas_rounds(t: usize, year: u32, n: &EngSecondChristmasSchedule, file: usize) {
    let construct: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    eng_second_set_league_rounds(t, year, 0, n.comeco);
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
    eng_second_set_league_rounds(t, year, n.comeco.len() as i32, middle);
    eng_second_set_league_rounds(t, year, (n.comeco.len() + middle.len()) as i32, n.fim);
}

/// a segunda e a terceira divisões inglesas (as mesmas datas): as rodadas antes do Natal
pub(crate) const SECOND_DIVISION_START: [EngSecondScheduleRound; 23] = [
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 12/ago
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 19/ago
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 26/ago
    (28, 7, 0, 0, 2, &[(-1, -1, -1)]), // 3: 28/ago
    (2, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 2/set
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 9/set
    (13, 8, 0, 2, 2, &[(-1, -1, -1)]), // 6: 13/set
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 16/set
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 8: 23/set
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 9: 30/set
    (7, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 7/out
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 14/out
    (17, 9, 0, 1, 2, &[(-1, -1, -1)]), // 12: 17/out
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 13: 21/out
    (24, 9, 0, 1, 2, &[(-1, -1, -1)]), // 14: 24/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 15: 28/out
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 16: 4/nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 17: 11/nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 18: 18/nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 19: 25/nov
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 20: 2/dez
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 21: 9/dez
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 22: 16/dez
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 13/dez
pub(crate) const SECOND_DIVISION_CHRISTMAS_0: [EngSecondScheduleRound; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 14/dez
pub(crate) const SECOND_DIVISION_CHRISTMAS_ROUNDS_1: [EngSecondScheduleRound; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 15/dez
pub(crate) const DIVISION_TWO_CHRISTMAS_ROUNDS_2: [EngSecondScheduleRound; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 16/dez
pub(crate) const SECOND_DIVISION_CHRISTMAS_3: [EngSecondScheduleRound; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 17/dez
pub(crate) const SECOND_DIVISION_CHRISTMAS_ROUNDS_4: [EngSecondScheduleRound; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 18/dez
pub(crate) const DIVISION_TWO_CHRISTMAS_ROUNDS_5: [EngSecondScheduleRound; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 19/dez
pub(crate) const SECOND_DIVISION_CHRISTMAS_6: [EngSecondScheduleRound; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 22 fora de 13 a 19/dez: a caixa)
pub(crate) const SECOND_DIVISION_CHRISTMAS_ROUNDS_DEFAULT: [EngSecondScheduleRound; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// a segunda e a terceira divisões inglesas (as mesmas datas): as rodadas depois do Natal
pub(crate) const SECOND_DIVISION_LATE_ROUNDS: [EngSecondScheduleRound; 19] = [
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 27: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 28: 20/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 29: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(-1, -1, -1)]), // 30: 10/fev (+1 ano)
    (17, 1, 1, 5, 1, &[(-1, -1, -1)]), // 31: 17/fev (+1 ano)
    (20, 1, 1, 1, 2, &[(-1, -1, -1)]), // 32: 20/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 33: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 34: 3/mar (+1 ano)
    (7, 2, 1, 2, 2, &[(-1, -1, -1)]), // 35: 7/mar (+1 ano)
    (10, 2, 1, 5, 1, &[(-1, -1, -1)]), // 36: 10/mar (+1 ano)
    (17, 2, 1, 1, 2, &[(-1, -1, -1)]), // 37: 17/mar (+1 ano)
    (24, 2, 1, 5, 1, &[(-1, -1, -1)]), // 38: 24/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 39: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(-1, -1, -1)]), // 40: 7/abr (+1 ano)
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 41: 14/abr (+1 ano)
    (16, 3, 1, 0, 2, &[(-1, -1, -1)]), // 42: 16/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 43: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 44: 28/abr (+1 ano)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 45: 6/mai (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 13/dez
pub(crate) const DIVISION_THREE_CHRISTMAS_ROUNDS_0: [EngSecondScheduleRound; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 14/dez
pub(crate) const THIRD_DIVISION_CHRISTMAS_1: [EngSecondScheduleRound; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 15/dez
pub(crate) const THIRD_DIVISION_CHRISTMAS_ROUNDS_2: [EngSecondScheduleRound; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 16/dez
pub(crate) const DIVISION_THREE_CHRISTMAS_ROUNDS_3: [EngSecondScheduleRound; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 17/dez
pub(crate) const THIRD_DIVISION_CHRISTMAS_4: [EngSecondScheduleRound; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 18/dez
pub(crate) const THIRD_DIVISION_CHRISTMAS_ROUNDS_5: [EngSecondScheduleRound; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 19/dez
pub(crate) const DIVISION_THREE_CHRISTMAS_ROUNDS_6: [EngSecondScheduleRound; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 22 fora de 13 a 19/dez: a caixa)
pub(crate) const THIRD_DIVISION_CHRISTMAS_DEFAULT: [EngSecondScheduleRound; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

pub(crate) const DIVISION_TWO_CHRISTMAS: EngSecondChristmasSchedule = EngSecondChristmasSchedule { comeco: &SECOND_DIVISION_START, referencia: 22, ajuste: 13, casos: [&SECOND_DIVISION_CHRISTMAS_0, &SECOND_DIVISION_CHRISTMAS_ROUNDS_1, &DIVISION_TWO_CHRISTMAS_ROUNDS_2, &SECOND_DIVISION_CHRISTMAS_3, &SECOND_DIVISION_CHRISTMAS_ROUNDS_4, &DIVISION_TWO_CHRISTMAS_ROUNDS_5, &SECOND_DIVISION_CHRISTMAS_6, &SECOND_DIVISION_CHRISTMAS_ROUNDS_DEFAULT], fim: &SECOND_DIVISION_LATE_ROUNDS, linha: 0x15c };
pub(crate) const DIVISION_TWO_PLAYOFF_0: EngCcCupCupRound = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
pub(crate) const SECOND_DIVISION_PLAYOFF_1: EngCcCupCupRound = cql10r((0x11, 4, 1, 3), (0x1b, 4, 1, 6, 1, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));
pub(crate) const THIRD_DIVISION_CHRISTMAS_SCHEDULE: EngSecondChristmasSchedule = EngSecondChristmasSchedule { comeco: &SECOND_DIVISION_START, referencia: 22, ajuste: 13, casos: [&DIVISION_THREE_CHRISTMAS_ROUNDS_0, &THIRD_DIVISION_CHRISTMAS_1, &THIRD_DIVISION_CHRISTMAS_ROUNDS_2, &DIVISION_THREE_CHRISTMAS_ROUNDS_3, &THIRD_DIVISION_CHRISTMAS_4, &THIRD_DIVISION_CHRISTMAS_ROUNDS_5, &DIVISION_THREE_CHRISTMAS_ROUNDS_6, &THIRD_DIVISION_CHRISTMAS_DEFAULT], fim: &SECOND_DIVISION_LATE_ROUNDS, linha: 0x161 };
pub(crate) const THIRD_DIVISION_PLAYOFF_SEMIFINAL: EngCcCupCupRound = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
pub(crate) const DIVISION_THREE_PLAYOFF_1: EngCcCupCupRound = cql10r((0x11, 4, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));

/// the Division Two schedule: 46 rounds (24 clubs), from 12 August to 6 May, nearly all on the same weekday (5) and
/// some midweek; at Christmas, four fixed-date rounds (20 December to 5 January) chosen by the day the 16 December
/// round falls on. The promotion play-offs (stage 0): two-legged semi-finals (4 clubs, draw on 7 May, first leg on 12,
/// 4 days apart) and a one-off final on 27 May [inference: venue 4 is the neutral ground]
/// Original: `EngSecond::create_schedule` (0x5757a0, eng_second.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5757a0 } (thiscall, ret 0x10, eng_second.cpp, 3.692 bytes, método +0x3c): `EngSecond::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase −1, 46 rodadas (`DIVISION_TWO_CHRISTMAS`); a fase 0, os play-offs (semifinais e
/// final); as outras devolvem 0. Fonte 0x9c43e8.
pub(crate) unsafe extern "thiscall" fn eng_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 46;
            *extra = 0;
            let t = alloc(0xbae);
            if t == 0 {
                show_error_at(0x9c43e8, 0xae);
                return 0;
            }
            crate::comparison::branch("cqp_4 calendario da segunda inglesa");
            let year = core::ptr::read_unaligned((this + league::YEAR) as *const u16) as u32;
            eng_second_set_christmas_rounds(t, year, &DIVISION_TWO_CHRISTMAS, 0x9c43e8);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 2;
            *extra = 0xa0;
            let t = alloc(0xd0);
            if t == 0 {
                show_error_at(0x9c43e8, 0x1b5);
                return 0;
            }
            crate::comparison::branch("cqp_4 play-offs da segunda inglesa");
            let year = core::ptr::read_unaligned((this + league::YEAR) as *const i16) as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &DIVISION_TWO_PLAYOFF_0, year);
            eng_cc_cup_write_cup_round(t, 1, &SECOND_DIVISION_PLAYOFF_1, year);
            t
        }
        _ => 0,
    }
}

/// The Staff_Records object (G_STAFF_RECORDS).
///
/// [pt] O registro de staff ([0xdd7f00]).
pub(crate) unsafe fn eng_second_staff_records() -> usize {
    read_usize(G_STAFF_RECORDS)
}

/// Updates the staff records by the club's status in the English Second Division (virtual `+0x44` of the First, Second
/// and Third). Returns 0.
///
/// `EngSecond::update_staff_records_for_status(clube, situação, fase, &saiu, _, _)` (club, status, stage, &removed, _,
/// _), announced by `0x686670`.
///
/// In the league (stage `0xff`), by status: 5, the title (Staff_Records::add_league_champions_record `0x868c50`,
/// thiscall(records, club, comp `+4`), ret 8); 0, promotion (`0x869480`, thiscall(records, club, comp, `0x64`), ret
/// `0xc`); 1, the play-off place (`0x86bdd0`, thiscall(records, club, comp, `0xa0`, `0x82`, `0x1e`), ret `0x14`); 3,
/// relegation (`0x86a1c0`, thiscall(records, club, comp), ret 8).
///
/// In the play-offs (any other stage), on the club's row of the table (`+0xb1`, `0x3b` bytes, `+0x3e` rows): 1 (won the
/// final), promotion with weight `0x32`, `+0x37` = 0 and removed = 1; 0 (went through the round), the qualification
/// (`0xa0`, `0x96`, `0xf`) and it keeps searching; `0xff` or 2 (knocked out), `+0x37` = `0xfe`.
/// Original: `EngSecond::update_staff_records_for_status` (0x576c50, eng_second.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x576c50 } (thiscall, ret 0x18, eng_second.cpp): `EngSecond::update_staff_records_for_status(clube, situação,
/// fase, &saiu, _, _)`, a virtual +0x44 da First, da Second e da Third (anunciada pela 0x686670). Devolve 0.
/// Na liga (fase 0xff), pela situação: 5, o título (Staff_Records::add_league_champions_record 0x868c50,
/// thiscall(registros, clube, comp +4), ret 8); 0, o acesso (0x869480, thiscall(registros, clube, comp, 0x64), ret
/// 0xc); 1, a vaga no mata-mata (0x86bdd0, thiscall(registros, clube, comp, 0xa0, 0x82, 0x1e), ret 0x14); 3, o
/// rebaixamento (0x86a1c0, thiscall(registros, clube, comp), ret 8). No mata-mata (outra fase), na linha do clube da
/// tabela (+0xb1, 0x3b bytes, +0x3e linhas): 1 (venceu a final), o acesso com peso 0x32, +0x37 = 0 e saiu = 1; 0
/// (passou da rodada), a classificação (0xa0, 0x96, 0xf) e segue procurando; 0xff ou 2 (caiu), +0x37 = 0xfe.
pub(crate) unsafe extern "thiscall" fn eng_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, left: usize, _a: u32, _b: u32) -> u32 {
    let s = status as u8;
    let comp = read_usize(this + 4);
    if stage as u8 == 0xff {
        match s {
            5 => {
                let title: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_CHAMPIONS_RECORD);
                title(eng_second_staff_records(), club, comp);
            }
            0 => {
                crate::comparison::branch("cpz_2 acesso na tabela");
                let promotion: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                promotion(eng_second_staff_records(), club, comp, 0x64); // cpz2: o acesso pela tabela pesa 0x64
            }
            1 => {
                crate::comparison::branch("cpz_2 vaga no mata-mata");
                let slot: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
                slot(eng_second_staff_records(), club, comp, 0xa0, 0x82, 0x1e); // cpz2: a vaga no mata-mata pesa 0x1e
            }
            3 => {
                crate::comparison::branch("cpz_2 rebaixamento na tabela");
                let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
                cair(eng_second_staff_records(), club, comp); // cpz2: o rebaixamento pela tabela
            }
            _ => {}
        }
        return 0;
    }
    let mut i: i32 = 0;
    while i < *((this + league::CLUB_COUNT) as *const i16) as i32 {
        let row = read_usize(this + league::TABLE) + (i as usize) * 0x3b;
        if read_usize(row) == club {
            match s {
                1 => {
                    crate::comparison::branch("cpz_2 venceu o mata-mata");
                    let promotion: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                    promotion(eng_second_staff_records(), club, comp, 0x32);
                    *((read_usize(this + league::TABLE) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0;
                    *(left as *mut u8) = 1;
                    return 0;
                }
                0 => {
                    let slot: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
                    slot(eng_second_staff_records(), club, comp, 0xa0, 0x96, 0xf);
                }
                0xff | 2 => {
                    *((read_usize(this + league::TABLE) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                    return 0;
                }
                _ => {}
            }
        }
        i += 1;
    }
    0
}

/// Comp_Stats::find_club_entry (0x4a4850) for a club in this competition.
///
/// [pt] A entrada do clube nas estatísticas da competição (Comp_Stats::find_club_entry 0x4a4850: thiscall(+8, clube), ret 4;
/// põe a tabela na ordem dos clubes se precisa e procura).
pub(crate) unsafe fn eng_second_comp_stats_club_entry(this: usize, club: usize) -> usize {
    let find: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    find(read_usize(this + 8), club)
}

/// Writes the club's situation in the competition stats (+0x73 the stage, +0x74 the best, +0x75 the worst), common to
/// the First and the Third, remapping play-off positions.
///
/// In the league (stage 0xff) the best and the worst from `baixo` to `alto` (the knockout slots) become `baixo`, and
/// with the worst in that range the worst becomes `alto`; in the knockout (stage 0), 1, 2 and 3 become `baixo`, `baixo`
/// + 1 and `baixo` + 2 (the best, the worst and the stage); in any other stage, as they came.
///
/// [pt] A situação do clube nas estatísticas (+0x73 a fase, +0x74 a melhor, +0x75 a pior), comum à First e à Third: na liga
/// (fase 0xff) a melhor e a pior de `baixo` a `alto` (as vagas do mata-mata) viram `baixo`, e com a pior nessa faixa a
/// pior vira `alto`; no mata-mata (fase 0), 1, 2 e 3 viram `baixo`, `baixo` + 1 e `baixo` + 2 (a melhor, a pior e a
/// fase); noutra fase, como vieram.
pub(crate) unsafe fn set_club_stage_status(this: usize, club: usize, stage: u8, current: u8, best: u8, worst: u8, low: u8, high: u8) {
    let e = eng_second_comp_stats_club_entry(this, club);
    let (mut m, mut p, mut a) = (best, worst, current);
    if stage == 0xff {
        if m >= low && m <= high {
            m = low;
        }
        if p >= low && p <= high {
            p = high;
        }
    } else if stage == 0 {
        let map = |v: u8| match v {
            1 => low,
            2 => low + 1,
            3 => low + 2,
            _ => v,
        };
        m = map(m);
        p = map(p);
        a = map(a);
    }
    *((e + 0x73) as *mut u8) = a;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}

/// Comparer of the stage status: side by side on the entry's 3 bytes; only the original without an entry.
///
/// [pt] O comparador das duas: a entrada é achada antes (a original e a cópia acham a mesma; a ordem dos clubes já fica
/// pronta) e as duas rodam lado a lado nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
pub(crate) unsafe fn compare_club_stage_status(name: &str, this: usize, club: usize, original: impl FnOnce(), copy: impl FnOnce()) {
    let e = eng_second_comp_stats_club_entry(this, club);
    if e == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(e + 0x73, 3)], original, copy);
    SCOREBOARD.record(igual, || format!("{}({:#x}, {:#x}) original={:x?} rust={:x?}", name, this, club, mo, mr));
}

/// like the First one (line 0x294; the play-offs 0x576a70)
/// Original: `EngSecond::advance_stage` (0x5769e0, eng_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x5769e0 } (thiscall, ret 0, eng_second.cpp): `EngSecond::advance_stage()` (a virtual +0x28): fonte 0x9c43e8,
/// linha 0x294; o mata-mata 0x576a70.
pub(crate) unsafe extern "thiscall" fn eng_second_advance_stage(this: usize) {
    advance_league_stage(this, 0x9c43e8, 0x294, 0x576a70, "cpz_3 mata-mata da second")
}

/// like the First one (6th, 3rd, 5th and 4th), with +0x3a = the extra from virtual +0x3c (lines 0x2b6 and 0x2cc)
/// Original: `EngSecond::create_play_off_stage` (0x576a70, eng_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x576a70 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_play_off_stage()` (de 0x5769e0). As chaves da
/// First (linhas 5, 2, 4 e 3); +0x3a da fase = o extra da virtual +0x3c. Fonte 0x9c43e8, linhas 0x2b6 e 0x2cc.
pub(crate) unsafe extern "thiscall" fn eng_second_create_play_off_stage(this: usize) {
    create_playoff_stage(this, [5, 2, 4, 3], 0x9c43e8, 0x2b6, 0x2cc, None, "cpz_1 mata-mata da second"); // cpz1: 6º × 3º e 5º × 4º da Second
}

/// the Second Division: discipline 9, no minimum ground, the league and the play-off
/// Original: `EngSecond::EngSecond` (0x5754a0, eng_second.cpp, thiscall, ret 8).
///
/// [pt] { 0x5754a0 } (thiscall, ret 8, eng_second.cpp): `EngSecond::EngSecond(ano, registro)`. A Second Division: 1 fase além
/// da liga (o play-off), sem estádio mínimo.
pub(crate) unsafe extern "thiscall" fn construct_eng_second(this: usize, year: u32, record: usize) -> usize {
    construct_english_division(this, year, record, &ENG_SECOND_DIVISION)
}

/// the Second (24): 2 go up directly, 3rd to 6th to the play-offs, the bottom 4 go down to the Third
/// Original: `EngSecond::set_up_rules` (0x576780, eng_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x576780 } (thiscall, ret 0, eng_second.cpp): `EngSecond::set_up_rules()`. A Second: como a First, mas 4
/// caem (+0xc1); a First acima ([0x9cf5c0]) e a Third abaixo ([0x9cf5c8]).
pub(crate) unsafe extern "thiscall" fn eng_second_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqi_4 regras da second");
    eng_prm_apply_league_rules(this, &eng_prm_second_division_rules())
}

/// the Second has 24 clubs; prize fund of 20,000 per place; TV pays 2,500,000 to each club (300,000 up to 1999)
/// Original: `EngSecond::create_table` (0x576630, eng_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x576630 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_table()`: 24 clubes ([0x9cf5c4]), cota 20.000
/// e a TV: 2.500.000 depois de 1999 (300.000 antes). Fonte 0x9c43e8, linha 0x1f5.
pub(crate) unsafe extern "thiscall" fn eng_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(this, 0x18, 0x9c43e8, 0x1f5, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, 0x4e20, || eng_prm_pay_tv_rights(this, if is_after_1999() { 0x2625a0 } else { 0x493e0 }, 0))
}

