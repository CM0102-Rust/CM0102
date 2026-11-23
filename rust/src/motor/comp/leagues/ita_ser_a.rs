use super::*;

/// Describes one round of the 0x41-byte table.
///
/// It holds the date (day, month 0..11, years to add to the league year +0x40, the weekday the date is moved to), the
/// code of byte +4 ([inference] the half of the season) and the TV slots in order (weekday, half, how many matches; the
/// list ends with −1).
pub(crate) type ItaSerALeagueRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Writes rounds `primeira`.. into round table `t`, in the original routine's order.
///
/// For each round: the date (`0x68a160`: push 0; push year; push code; push week; push years; push month; push day;
/// push i; push t; add esp -> cdecl: value 0) and the slots (`0x68a1c0`: push 0; push c; push b; push a; push j; push
/// i; push t; add esp -> cdecl, value 0).
pub(crate) unsafe fn ita_ser_a_set_league_rounds(t: usize, year: u32, first: i32, table: &[ItaSerALeagueRound]) {
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in table.iter().enumerate() {
        let i = first + k as i32;
        exe::league_round_set_date(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, year, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            exe::league_round_set_match_slot(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// O que muda entre as ligas de fase única.
pub(crate) struct ItaSerASingleStageLeague {
    pub(crate) rounds: i16,
    pub(crate) file: usize,
    pub(crate) row: u32,
    pub(crate) table: &'static [ItaSerALeagueRound],
    pub(crate) branch: &'static str,
}

/// The common create_schedule body (thiscall(league, stage, &rounds, &extra, &é_liga), ret 0x10; no saved state): only
/// the main stage (−1; the others return 0 without writing anything).
///
/// é_liga (if requested) = 1, the rounds, extra = 0, the table of rounds × 0x41 bytes (malloc `exe::crt_malloc`: push; add esp,
/// 4 -> cdecl; out of memory: the box and 0) with the league's dates and slots. Returns the table.
pub(crate) unsafe fn build_league_schedule(this: League, stage: u32, rodadas: *mut i16, extra: *mut i16, league: *mut i32, p: &ItaSerASingleStageLeague) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rodadas = p.rounds;
    *extra = 0;
    let t = exe::crt_malloc(p.rounds as u32 * 0x41);
    if t == 0 {
        show_error_at(p.file, p.row);
        return 0;
    }
    covered!(p.branch);
    let ano = (this.year() as u16) as u32;
    ita_ser_a_set_league_rounds(t, ano, 0, p.table);
    t
}

/// a Série A, 34 rodadas de 28/ago a 12/mai; quase todas com duas faixas de TV (dias 6 e 5, turno 2, 2 jogos cada)
pub(crate) const SERIE_A_ROUNDS: [ItaSerALeagueRound; 34] = [
    (28, 7, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 0: 28/ago
    (9, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 1: 9/set
    (16, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 2: 16/set
    (23, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 3: 23/set
    (30, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 4: 30/set
    (10, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 5: 10/out
    (14, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 6: 14/out
    (21, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 7: 21/out
    (28, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 8: 28/out
    (4, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 9: 4/nov
    (18, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 10: 18/nov
    (25, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 11: 25/nov
    (2, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 12: 2/dez
    (9, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 13: 9/dez
    (16, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 14: 16/dez
    (6, 0, 1, 2, 2, &[(-1, -1, -1)]), // 15: 6/jan (+1 ano)
    (13, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 16: 13/jan (+1 ano)
    (20, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 17: 20/jan (+1 ano)
    (27, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 18: 27/jan (+1 ano)
    (3, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 19: 3/fev (+1 ano)
    (10, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 20: 10/fev (+1 ano)
    (17, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 21: 17/fev (+1 ano)
    (24, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 22: 24/fev (+1 ano)
    (3, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 23: 3/mar (+1 ano)
    (10, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 24: 10/mar (+1 ano)
    (17, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 25: 17/mar (+1 ano)
    (24, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 26: 24/mar (+1 ano)
    (30, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 30/mar (+1 ano)
    (7, 3, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 28: 7/abr (+1 ano)
    (14, 3, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 29: 14/abr (+1 ano)
    (21, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 21/abr (+1 ano)
    (28, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 28/abr (+1 ano)
    (5, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 5/mai (+1 ano)
    (12, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 12/mai (+1 ano)
];

/// a Série B, 38 rodadas de 29/ago a 14/mai; quase todas com duas faixas de TV (dias 4 e 0, turno 2, 2 jogos cada)
pub(crate) const SERIE_B_ROUNDS: [ItaSerALeagueRound; 38] = [
    (29, 7, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 0: 29/ago
    (5, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 1: 5/set
    (12, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 4: 26/set
    (3, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 5: 3/out
    (10, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 6: 10/out
    (17, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 7: 17/out
    (24, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 8: 24/out
    (31, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 9: 31/out
    (7, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 10: 7/nov
    (14, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 11: 14/nov
    (21, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 12: 21/nov
    (28, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 13: 28/nov
    (5, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 14: 5/dez
    (12, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 15: 12/dez
    (19, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 16: 19/dez
    (6, 0, 1, 3, 2, &[(-1, -1, -1)]), // 17: 6/jan (+1 ano)
    (9, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 18: 9/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 19: 16/jan (+1 ano)
    (19, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 19/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 21: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 22: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 23: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 24: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 25: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 26: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 27: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 28: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 29: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 30: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 31: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 32: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 33: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 34: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 35: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 36: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 14/mai (+1 ano)
];

/// The French first division: the rounds in order, each marked `true` if it is only used with 38 rounds (20 clubs; with
/// 34, the `false` ones, in sequence). The index in the comment is the one of the 38-round season.
pub(crate) const FRENCH_FIRST_DIVISION_ROUNDS: [(bool, ItaSerALeagueRound); 38] = [
    (false, (31, 6, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 0: 31/jul
    (false, (6, 7, 0, 4, 2, &[(-1, -1, -1)])), // 1: 6/ago
    (false, (14, 7, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 2: 14/ago
    (false, (20, 7, 0, 4, 2, &[(-1, -1, -1)])), // 3: 20/ago
    (false, (28, 7, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 4: 28/ago
    (false, (10, 8, 0, 4, 2, &[(-1, -1, -1)])), // 5: 10/set
    (false, (18, 8, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 6: 18/set
    (false, (25, 8, 0, 4, 2, &[(-1, -1, -1)])), // 7: 25/set
    (false, (2, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 8: 2/out
    (false, (12, 9, 0, 1, 2, &[(-1, -1, -1)])), // 9: 12/out
    (false, (15, 9, 0, 4, 2, &[(-1, -1, -1)])), // 10: 15/out
    (false, (23, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 11: 23/out
    (false, (30, 9, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 12: 30/out
    (false, (6, 10, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 13: 6/nov
    (false, (10, 10, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 14: 10/nov
    (false, (19, 10, 0, 4, 2, &[(-1, -1, -1)])), // 15: 19/nov
    (false, (27, 10, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 16: 27/nov
    (false, (3, 11, 0, 4, 2, &[(-1, -1, -1)])), // 17: 3/dez
    (false, (11, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 18: 11/dez
    (false, (15, 11, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 19: 15/dez
    (false, (18, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 20: 18/dez
    (true, (8, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 21: 8/jan (+1 ano)
    (false, (15, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 22: 15/jan (+1 ano)
    (true, (22, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 23: 22/jan (+1 ano)
    (false, (26, 0, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 24: 26/jan (+1 ano)
    (false, (2, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 25: 2/fev (+1 ano)
    (false, (5, 1, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 26: 5/fev (+1 ano)
    (false, (16, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 27: 16/fev (+1 ano)
    (false, (27, 1, 1, 4, 2, &[(-1, -1, -1)])), // 28: 27/fev (+1 ano)
    (true, (4, 2, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 29: 4/mar (+1 ano)
    (false, (11, 2, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 30: 11/mar (+1 ano)
    (true, (18, 2, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 31: 18/mar (+1 ano)
    (false, (25, 2, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 32: 25/mar (+1 ano)
    (false, (8, 3, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 33: 8/abr (+1 ano)
    (false, (15, 3, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 34: 15/abr (+1 ano)
    (false, (29, 3, 1, 5, 2, &[(6, 2, 1), (6, 2, 1), (-1, -1, -1)])), // 35: 29/abr (+1 ano)
    (false, (13, 4, 1, 5, 2, &[(-1, -1, -1)])), // 36: 13/mai (+1 ano)
    (false, (20, 4, 1, 5, 2, &[(-1, -1, -1)])), // 37: 20/mai (+1 ano)
];

pub(crate) const SERIE_A_LEAGUE: ItaSerASingleStageLeague = ItaSerASingleStageLeague { rounds: 0x22, file: 0x9ea2a8, row: 0xb0, table: &SERIE_A_ROUNDS, branch: "cqo_6 calendario da serie a" };
pub(crate) const SERIE_B_LEAGUE: ItaSerASingleStageLeague = ItaSerASingleStageLeague { rounds: 0x26, file: 0x9ea384, row: 0xa8, table: &SERIE_B_ROUNDS, branch: "cqo_6 calendario da serie b" };

/// The play-off match of the Italian leagues (stage 0, one 0x68-byte cup round, `eng_cc_cup_write_cup_round`; the year
/// +0x40 signed).
///
/// The draw and the match in May of the following year (Serie A: draw on 13 May, match on 19 May; Serie B: 15 May and
/// 21 May), name 0xa0, no draw (0x200), tie-break 3, category 5, two legs (+0x21 = 2) with 3 days between the matches,
/// no prizes; the teams (2n teams, n matches, 2n new entrants) from the league's byte +0xee (n, signed; arithmetic in
/// 16 bits as in the original) [inference: the requested play-off ties].
pub(crate) fn italian_playoff_round(sorteio: (u32, u32, u32, u32), game: (u32, u32, u32, u32, u32, u32), n: i8) -> EngCcCupCupRound {
    let n = n as i16;
    let two = n.wrapping_shl(1) as u16;
    cql10r(sorteio, game, 0xa0, (0, 0x200, 3), 5, (two, n as u16, two, 0), (0, 2, 3), (0, 0, 0))
}

/// O que muda entre a Série A e a Série B.
pub(crate) struct ItalianLeagueWithTiebreak {
    pub(crate) league: &'static ItaSerASingleStageLeague,
    pub(crate) draw: (u32, u32, u32, u32),
    pub(crate) game: (u32, u32, u32, u32, u32, u32),
    pub(crate) row_tiebreak: u32,
}

/// The shared create_schedule body of the two Italian leagues (thiscall(league, stage, &rounds, &extra, &é_liga), ret
/// 0x10; no saved state).
///
/// Stage −1 is the main one (`build_league_schedule`); stage 0, the play-off match: é_liga (if requested) = 0, 1 round,
/// extra 0, the table of 0x68 bytes (malloc `exe::crt_malloc`; out of memory: the box and 0); the others return 0 without
/// writing anything. Returns the table.
pub(crate) unsafe fn italian_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, liga: *mut i32, p: &ItalianLeagueWithTiebreak) -> usize {
    match stage as u8 {
        0xff => build_league_schedule(this, stage, rounds, extra, liga, p.league),
        0 => {
            if !liga.is_null() {
                *liga = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(p.league.file, p.row_tiebreak);
                return 0;
            }
            covered!("cqo_6 italian tiebreak match");
            let ano = this.year() as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &italian_playoff_round(p.draw, p.game, read_i8(this.addr() + italian_league::TIEBREAK_TIE_COUNT)), ano);
            t
        }
        _ => 0,
    }
}

pub(crate) const SERIE_A_WITH_PLAYOFF: ItalianLeagueWithTiebreak = ItalianLeagueWithTiebreak { league: &SERIE_A_LEAGUE, draw: (0xd, 4, 1, 0), game: (0x13, 4, 1, 6, 1, 0), row_tiebreak: 0x226 };
pub(crate) const SERIE_B_WITH_TIEBREAK: ItalianLeagueWithTiebreak = ItalianLeagueWithTiebreak { league: &SERIE_B_LEAGUE, draw: (0xf, 4, 1, 0), game: (0x15, 4, 1, 6, 1, 0), row_tiebreak: 0x170 };

/// the Serie A schedule: 34 rounds (18 clubs), from 28 August to 12 May of the next year, all on the same weekday (6)
/// except two (6 January, 30 March); in nearly all, two matches go to each of two TV slots (days 6 and 5, slot 2)
/// [inference: the televised weekend evening matches]. Stage 0, the play-off match (spareggio) [inference: clubs level
/// on points for a place]: two legs 3 days apart, draw on 13 and match on 19 May, ties in fixed order, as many ties as
/// the byte +0xee
/// Original: `ItaSerA::create_schedule` (0x6409b0, ita_ser_a.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    italian_schedule(League(this), stage, rounds, extra, league, &SERIE_A_WITH_PLAYOFF)
}

/// Row i of the league table [+0xb1] (0x3b bytes each).
pub(crate) unsafe fn ita_ser_a_table_row(league: League, i: usize) -> usize {
    league.table() + i * league_table_row::SIZE
}

/// The Serie A zones by position (0 = first): 1 champion, 2 the other Champions League places, 3 UEFA Cup places, 4 the
/// next two [inference: Intertoto], 5 relegation (+0xc1), 0 the rest.
///
/// 2 covers the nation's `cl` places (UefaSeeding::get_nation_places `exe::uefaseeding_get_nation_places`: push &uefa; push &cl; push
/// [[liga+4]+0x5d] (the competition's nation); mov ecx, [0xde1f58] -> thiscall, ret 0xc, which writes 1 to both before
/// searching); 3 the `uefa` UEFA Cup places, right after; 4 the next two [inference: Intertoto]; 5 the last `+0xc1` of
/// 18 (relegation); 0 the rest.
///
/// The original routine builds the 18 zones in a local record (16 + 2 bytes) and reads two beyond it with a table of
/// more than 18 clubs; the copy uses 64 bytes with zeros after the 18 (Serie A has 18).
pub(crate) unsafe fn serie_a_zones(league: League) -> [u8; 64] {
    let mut z = [0u8; 64];
    z[0] = 1;
    let (mut cl, mut uefa) = (0i8, 0i8);
    exe::uefaseeding_get_nation_places(read_usize(G_UEFA_SEEDING), league.competition().nation().addr(), &mut cl, &mut uefa);
    // simplification: the nation’s counts stay from 1 to 16 in the game; the copy ignores the bands that would fall outside the record.
    let cl = cl as i32;
    let uefa = uefa as i32;
    if cl > 1 {
        for k in 1..cl {
            if (k as usize) < 64 {
                z[k as usize] = 2;
            }
        }
    }
    if uefa > 0 {
        for k in 0..uefa {
            if ((cl + k) as usize) < 64 {
                z[(cl + k) as usize] = 3;
            }
        }
    }
    let i = uefa + cl;
    if i >= 0 && (i as usize) + 1 < 64 {
        z[i as usize] = 4; // cql4: the two positions after the European slots
        z[i as usize + 1] = 4;
    }
    let relegated = (league.relegated_count() as i8) as i32;
    if relegated > 0 {
        for k in 0..relegated {
            if 17 - k >= 0 {
                z[(17 - k) as usize] = 5;
            }
        }
    }
    z
}

/// Per-position zones of Serie B: 1 champion, 2 the 3 promotion places, 3 the last 4 of 20 (relegation), 0 the rest.
pub(crate) fn serie_b_zones() -> [u8; 64] {
    let mut z = [0u8; 64];
    z[0] = 1;
    z[1] = 2;
    z[2] = 2;
    z[3] = 2; // cql4: the Serie B promotion slots
    for k in 16..20 {
        z[k] = 3;
    }
    z
}

/// True if two adjacent table rows have equal points (+0x19) but different zones.
pub(crate) unsafe fn tie_on_zone_boundary(league: League, z: &[u8; 64]) -> bool {
    let n = league.club_count() as i32 - 1;
    (0..n.max(0) as usize).any(|i| {
        let l = LeagueTableRow(ita_ser_a_table_row(league, i));
        l.points() == read_i16(l.addr() + 0x3b + league_table_row::POINTS) && z[i.min(63)] != z[(i + 1).min(63)]
    })
}

/// The Serie A day; at season end, two clubs level on points in different bands are not split by goal difference: the
/// first such pair from the top is ordered by head-to-head and goes to an extra match (the next stage); with no tie on
/// a boundary, the season closes. Convention: thiscall, ret 8, `ita_ser_a.cpp`, slot `+0x14`.
/// `ItaSerA::update_for_date(today, full)` (today, complete). First the league day (`exe::league_update_for_date`: push
/// complete; push today; mov ecx, this -> thiscall, ret 8), whose return value it passes on. Then, at the end of the
/// season (`complete()`), in the main stage (`stage_count()` = 0, no stages created), with `full` and with
/// `tiebreak_matches()` [inference: the table closed]: it clears `season_closed` and looks, from top to bottom, for the
/// first pair of neighbouring clubs with the same points in different bands (serie_a_zones). If found: it orders the
/// tied clubs by head-to-head (`exe::ita_league_sort_clubs_level_on_points`: push points; mov ecx, this -> thiscall,
/// ret 4), creates the array of 1 stage (`stage_count` = 1, `stages` = `exe::crt_malloc` of 4: push 4; add esp, 4 ->
/// cdecl) and calls virtual `comp_vtable::ADVANCE_STAGE` (the next stage: the tie-break match). With no tie on a
/// boundary: `complete` = 0, `tiebreak_matches` = 0 and virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`
/// (League::update_positions_and_status). The rule: in Italy, a tie on points between clubs in different bands (the
/// title; Italy's Champions League places; the UEFA Cup places; the next two [inference: the Intertoto]; relegation,
/// the last `+0xc1` of 18) is not decided by goal difference: it goes to an extra match.
/// Original: `ItaSerA::update_for_date` (0x642880, ita_ser_a.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let r = exe::league_update_for_date(this, today, full) as u32;
    // simplification: the original returns the league day byte with the upper bits of eax left over from the last call; the
    // the competitions loop only reads the byte.
    if this.complete() == 0 || this.stage_count() != 0 || full == 0 || this.tiebreak_matches() == 0 {
        return r;
    }
    let z = serie_a_zones(this);
    let n = this.club_count() as i32 - 1;
    this.set_season_closed(0);
    covered!("cql_4 serie a end");
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let l = LeagueTableRow(ita_ser_a_table_row(this, i as usize));
            let points = l.points();
            if points == read_i16(l.addr() + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: the Serie A pair at the boundary
                exe::ita_league_sort_clubs_level_on_points(this, points as u16 as u32);
                this.set_stage_count(1);
                let v = exe::crt_malloc(4);
                this.set_stages(v);
                let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::ADVANCE_STAGE));
                proxima(this.addr());
                return r;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    this.set_complete(0);
    this.set_tiebreak_matches(0);
    let status: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
    status(this.addr());
    r
}

/// The comparer of the two routines: the league day and the play-off stage cannot run twice, so only the original
/// routine runs and its guarantees are checked.
///
/// In the main stage and with `completo`: if the play-off stage was created (`+0x2c` changed), there is a pair of
/// neighbouring rows in the table tied across different zones; and the original routine never leaves the season end
/// pending (`+0xdd` and `+0xe6` set and `+0x2c` = 0), because it either creates the stage or zeroes them.
pub(crate) unsafe fn compare_league_day(name: &str, this: League, full: u32, faixas: impl Fn() -> [u8; 64], original: impl FnOnce() -> u32) -> u32 {
    let stages = this.stage_count();
    let r = original();
    if full != 0 && stages == 0 {
        if this.stage_count() != 0 {
            let z = faixas();
            SCOREBOARD.record(tie_on_zone_boundary(this, &z) && this.stages() != 0, || format!("{}({:#x}): stage of the tiebreak without draw in the boundary (+league::STAGES = {:#x})", name, this.addr(), this.stages()));
        } else {
            SCOREBOARD.record(!(this.complete() != 0 && this.tiebreak_matches() != 0), || format!("{}({:#x}): o end of the season ficou pendente (+league::COMPLETE e +league::TIEBREAK_MATCHES ligados)", name, this.addr()));
        }
    }
    r
}

/// Updates the staff records by status in Serie A: in the league, the base records; in the tie-break match, the winner
/// (status 1 [inference: the club won]) is placed above the other in the table (the rows swap if it was below) and,
/// with no play-off pending, the season closes. Returns −1 in the tie-break case. Convention: thiscall, ret `0x18`,
/// `ita_ser_a.cpp`, slot `+0x44`. `ItaSerA::update_staff_records_for_status(club, status, stage, a4, game, a6)` (club,
/// status, stage, a4, match, a6). In the league (stage `0xff`): the base version
/// (`exe::league_update_staff_records_for_status`: push a6; push match; push a4; push -1; push status; push club; mov
/// ecx, this -> thiscall, ret `0x18`), whose return value it passes on. In the tie-break match (any other stage), only
/// with status 1: `table_column` = 0, sorts the table (`exe::league_sort_table`: push 0; mov ecx -> thiscall, ret 4)
/// and finds the club's row: if the club above is one of the two in the match (`fixture::HOME_CLUB`,
/// `fixture::AWAY_CLUB`), the two rows swap places; otherwise, if the club below is one of them, they stay. In both
/// cases the two rows get their position (`+4`) and an open destination (`league_table_row::STATUS` = `0xff`). Then, if
/// no row has destination 1 (the play-off above), it closes the season: `tiebreak_matches` = 0, `complete` = 0, virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`, virtual `comp_vtable::CHECK_COMPLETE` (0, 0) and `complete` = 1. The
/// rule: the winner of the tie-break match finishes ahead of the other.
/// Original: `ItaSerA::update_staff_records_for_status` (0x641dc0, ita_ser_a.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let game = Fixture(game);
    let this = League(this);
    if stage as u8 == 0xff {
        return exe::league_update_staff_records_for_status(this, club, status, 0xffff_ffff, a4, game, a6);
    }
    if status as u8 != 1 {
        return 0xffff_ffff;
    }
    covered!("cql_4 tiebreak result");
    this.set_table_column(0);
    exe::league_sort_table(this, (0) as u32 as usize);
    let t = this.table();
    let (a, b) = (game.home_club().addr(), game.away_club().addr());
    let mut i: i16 = 0;
    while i < this.club_count() {
        let l = t + i as usize * 0x3b;
        if read_usize(l) == club {
            if i > 0 {
                let above = read_usize(l - 0x3b);
                if above == a || above == b {
                    // the club was below the other: they swap
                    let mut tmp = [0u8; 0x3b];
                    core::ptr::copy_nonoverlapping(l as *const u8, tmp.as_mut_ptr(), 0x3b);
                    core::ptr::copy_nonoverlapping((l - 0x3b) as *const u8, l as *mut u8, 0x3b);
                    let l2 = this.table() + i as usize * league_table_row::SIZE;
                    core::ptr::copy_nonoverlapping(tmp.as_ptr(), (l2 - 0x3b) as *mut u8, 0x3b);
                    let l3 = LeagueTableRow(this.table() + i as usize * league_table_row::SIZE);
                    write_u8(l3.addr() + 4, i as u8);
                    write_u8(l3.addr() - 0x3b + 4, (i as u8).wrapping_sub(1));
                    l3.set_status(table_status::UNDECIDED);
                    write_u8(l3.addr() - 0x3b + league_table_row::STATUS, 0xff);
                    break;
                }
            }
            if i < this.club_count() {
                // simplification: the original reads the club of the next row even on the last one (4 bytes past the table), as here.
                let below = read_usize(l + 0x3b);
                if below == a || below == b {
                    write_u8(t + i as usize * 0x3b + 4, i as u8);
                    let t2 = this.table();
                    write_u8(t2 + i as usize * 0x3b + 0x3f, (i as u8).wrapping_add(1));
                    write_u8(this.table() + i as usize * league_table_row::SIZE + league_table_row::STATUS, 0xff);
                    write_u8(this.table() + i as usize * league_table_row::SIZE + 0x72, 0xff);
                    break;
                }
            }
        }
        i += 1;
    }
    let n = this.club_count();
    let mut k: i16 = 0;
    while k < n {
        if read_u8(ita_ser_a_table_row(this, k as usize) + league_table_row::STATUS) == 1 {
            return 0xffff_ffff;
        }
        k += 1;
    }
    this.set_tiebreak_matches(0);
    this.set_complete(0);
    let status_f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
    status_f(this.addr());
    let end: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CHECK_COMPLETE));
    end(this.addr(), 0, 0);
    this.set_complete(1);
    0xffff_ffff
}

/// What changes from one Italian division to another in the constructor.
pub(crate) struct ItalianDivision {
    pub(crate) vtabela: usize,
    /// a Série A e a Série B gravam +0xee = 0 (o campo próprio da classe [inferência: o desempate em curso]) e +0x4c = 1
    pub(crate) series: bool,
    /// fases além da liga (+0x2c): 0 na A e na B; 2 na C1 (o play-off e o play-out; o vetor [+0xc] = malloc de 8)
    pub(crate) stages: u32,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    /// The real 2001/02 calendar by club names (set_up_fixed_fixtures; mov ecx, this -> thiscall, ret 0).
    pub(crate) fixed: usize,
    pub(crate) file: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    pub(crate) rows: [u32; 5],
}

/// The body of the four Italian division constructors (thiscall(this, year, record), ret 8, SEH frame only for the
/// operator new); returns this.
///
/// The base (League::League `exe::new_league`: mov ecx -> thiscall, ret 0), the record (+4), the vtable, the year (+0x40) and
/// discipline rule 0x11 (+0x50, the Italian leagues' one); it tries to load the state (League::load_from_temp_file
/// `exe::league_load_from_temp_file`: push 1; mov ecx -> thiscall, ret 4). If it loaded: done.
///
/// Otherwise, the new league: in A and B, +0xee = 0 and +0x4c = 1; stage 0xff (+0x44), current stage −1 (+0x30), +0x2c
/// stages and the array [+0xc] (malloc `exe::crt_malloc` of 4 × stages: push; add esp, 4 -> cdecl; 0 with no stages); the rules
/// and the table (error box and stop if they fail); the real 2001/02 calendar (unchecked); the fixtures
/// (League::create_fixtures `create_fixtures`), the sort (League::sort_table `sort_table`: push 0 -> thiscall, ret 4), the stats
/// (operator new `exe::crt_operator_new` of 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats `exe::new_comp_stats`: push league; mov ecx,
/// new -> thiscall, ret 4) at +8 and the predicted positions (`league_set_predicted_positions`: mov ecx -> thiscall, ret 0).
pub(crate) unsafe fn construct_italian_division(this: League, year: u32, registro: usize, p: &ItalianDivision) -> usize {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.rules);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.table);
    let fixos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(p.fixed);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, registro);
    write_usize(this.addr(), p.vtabela);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(discipline_rule::ITALY_LEAGUE); // cqm4: the discipline rule of the Italian leagues
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("cqm_4 new italian division");
    if p.series {
        write_u8(this.addr() + italian_league::TIEBREAK_TIE_COUNT, 0);
        this.set_season_closed(1); // cqm4: the history mark of A and B
    }
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(p.stages as usize);
    if p.stages != 0 {
        let v = exe::crt_malloc(p.stages * 4);
        this.set_stages(v);
    } else {
        this.set_stages(0);
    }
    if regras(this.addr()) == 0 {
        show_error_at(p.file, p.rows[0]);
        return this.addr();
    }
    if tabela(this.addr()) == 0 {
        show_error_at(p.file, p.rows[1]);
        return this.addr();
    }
    fixos(this.addr()); // cqm4: the real 2001/02 calendar
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(p.file, p.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(p.file, p.rows[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.file, p.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this); // cqm4: the expected positions
    this.addr()
}

pub(crate) const SERIE_A: ItalianDivision = ItalianDivision {
    vtabela: 0x96c3cc,
    series: true,
    stages: 0,
    rules: 0x641640,
    table: 0x641510,
    fixed: 0x642a70,
    file: 0x9ea2a8,
    rows: [0x53, 0x5a, 0x64, 0x6d, 0x74],
};
pub(crate) const SERIE_B_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c480,
    series: true,
    stages: 0,
    rules: 0x6468a0,
    table: 0x646780,
    fixed: 0x647230,
    file: 0x9ea384,
    rows: [0x4b, 0x52, 0x5c, 0x65, 0x6c],
};
pub(crate) const SERIE_C1A_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c534,
    series: false,
    stages: 2, // cqm4: the C1/A play-off and play-out
    rules: 0x64b770,
    table: 0x64b670,
    fixed: 0x64be90,
    file: 0x9ea474,
    rows: [0x49, 0x50, 0x5a, 0x63, 0x6a],
};
pub(crate) const SERIE_C1B_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c5e8,
    series: false,
    stages: 2,
    rules: 0x64f920,
    table: 0x64f820,
    fixed: 0x650100,
    file: 0x9ea558,
    rows: [0x4a, 0x51, 0x5b, 0x64, 0x6b],
};

/// Serie A: discipline rule 0x11 (the Italian leagues one), only the league (the tie-break is born at season end); with
/// no saved state, the rules, the table, the real 2001/02 fixture list by club name, the fixtures, the order, the stats
/// and the predicted positions
/// Original: `ItaSerA::ItaSerA` (0x6406d0, ita_ser_a.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_a(this: usize, year: u32, record: usize) -> usize {
    construct_italian_division(League(this), year, record, &SERIE_A)
}

/// Comparer of the four constructors: runs only the original and checks return value, vtable, year, record, discipline
/// and, for a new league, stages and Serie A/B fields.
pub(crate) unsafe fn compare_italian_constructor(name: &str, this: League, year: u32, record: usize, p: &ItalianDivision, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtabela && built_year_ok(this.year() as u16, year as u16)
        && this.competition().addr() == record && this.discipline_rule() == discipline_rule::ITALY_LEAGUE;
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 && !loaded {
        ok = ok && this.stage_count() == p.stages as usize && (this.stages() != 0) == (p.stages != 0);
        if p.series {
            ok = ok && *((this.addr() + italian_league::TIEBREAK_TIE_COUNT) as *const u8) == 0;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}, importance {}, stage index {}]: returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, this.competition().addr(), record, this.discipline_rule(), this.importance(), this.stage_index() as i32, r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

/// Number of league matches: (clubs +0x3e / 2) x rounds +0xa9.
pub(crate) unsafe fn league_match_count(league: League) -> i32 {
    let clubs = league.club_count() as i32;
    let rounds = league.round_count() as i32;
    (clubs / 2).wrapping_mul(rounds)
}

/// The body of set_up_fixed_fixtures for both leagues (thiscall, ret 0, SEH frame only for the vector constructor):
/// allocates the fixed-fixture vector (+0xea), fills each fixture from the table by club names, and destroys it (+0xea
/// = 0) if any club was not found.
///
/// The vector of fixed fixtures (operator new `exe::crt_operator_new` of n × 16 + 4: push; add esp, 4 -> cdecl; the count n in the
/// first dword and each fixture through constructor 0x645800, with destructor 0x5a53c0 for the exception: `eh vector
/// constructor iterator` `exe::crt_vector_constructor_iterator`, push 0x5a53c0; push 0x645800; push n; push 0x10; push vector -> stdcall, ret 0x14)
/// in +0xea; out of memory, the box for line `linha` of source file `arquivo` and end.
///
/// Then each fixture of `tabela`, in order, in its place in the vector (fixed_fixture_set_by_club_names `exe::fixed_fixture_set_by_club_names`: push
/// the 6th (away, via the map); push the 5th (home); push year; push month (0 = January); push day; push league; mov
/// ecx, [+0xea] + 16 × k -> thiscall, ret 0x18: the date and the clubs found by name, −1 in +8/+0xc if not found).
///
/// Finally, if any of the n fixtures was left with a club not found (+8 or +0xc = −1), the vector is destroyed (`eh
/// vector destructor iterator` `exe::crt_vector_destructor_iterator`: push 0x5a53c0; push count; push 0x10; push vector -> stdcall, ret 0x10;
/// operator delete `exe::crt_operator_delete` of the block: push; add esp, 4 -> cdecl) and +0xea = 0: the league goes back to the
/// randomly drawn calendar.
pub(crate) unsafe fn set_up_fixed_fixtures(this: League, table: &[(u8, u8, u16, u32, u32)], file: usize, row: u32) {
    let n = league_match_count(this);
    let block = exe::crt_operator_new((n as u32).wrapping_mul(16).wrapping_add(4));
    let array = if block != 0 {
        write_usize(block, n as usize);
        exe::crt_vector_constructor_iterator(block + 4, 0x10, n as u32, 0x645800, 0x5a53c0);
        block + 4
    } else {
        0
    };
    this.set_fixed_fixtures(array);
    if array == 0 {
        show_error_at(file, row);
        return;
    }
    for (k, &(dia, mes, ano, casa, fora)) in table.iter().enumerate() {
        exe::fixed_fixture_set_by_club_names(this.fixed_fixtures() + k * 0x10, this, dia as u32, mes as u32, ano as u32, casa, fora); // cqn5: each fixed match
    }
    covered!("cqn_5 fixed calendar");
    let n = league_match_count(this);
    if n <= 0 {
        return;
    }
    let v = this.fixed_fixtures();
    for i in 0..n as usize {
        let f = v + i * 0x10;
        if read_usize(f + 8) == 0xffff_ffff || read_usize(f + fixed_fixture::AWAY_CLUB_ID) == 0xffff_ffff { // cqn5: a club not found drops the list
            if v != 0 {
                exe::crt_vector_destructor_iterator(v, 0x10, read_usize(v - 4) as u32, 0x5a53c0);
                exe::crt_operator_delete(v - 4);
            }
            this.set_fixed_fixtures(0);
            return;
        }
    }
}

/// the 2001/02 Serie A does not draw its fixtures: the 306 matches (34 rounds of 9) are fixed, with date and both clubs
/// by name; if any name is missing from the database, the whole list is dropped and the league falls back to the drawn
/// schedule
/// Original: `ItaSerA::create_fixed_fixtures` (0x642a70, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_fixed_fixtures(this: usize) {
    set_up_fixed_fixtures(League(this), &SERIE_A_FIXED_FIXTURES, 0x9ea2a8, 0x60f)
}

/// The fixed matches (day, month with 0 = January, year, the 5th argument (home, by the map), the 6th (away)), with the
/// names in the exe's `.rdata`.
///
/// In the original routine's order (extracted from the disassembly and checked: the vector positions go in steps of
/// 16).
pub(crate) const SERIE_A_FIXED_FIXTURES: [(u8, u8, u16, u32, u32); 306] = [
    (28, 7, 2001, 0x9ea370, 0x9ea378), // Bologna × Atalanta
    (28, 7, 2001, 0x9ea360, 0x9ea368), // Brescia × Milan
    (28, 7, 2001, 0x9ea34c, 0x9ea358), // Fiorentina × Chievo
    (28, 7, 2001, 0x9ea33c, 0x9ea344), // Inter × Perugia
    (28, 7, 2001, 0x9ea328, 0x9ea334), // Juventus × Venezia
    (28, 7, 2001, 0x9ea314, 0x9ea31c), // Lazio × Piacenza
    (28, 7, 2001, 0x9ea304, 0x9ea30c), // Lecce × Parma
    (28, 7, 2001, 0x9ea2f4, 0x9ea2fc), // Udinese × Torino
    (28, 7, 2001, 0x9ea2e4, 0x9ea2ec), // Verona × Roma
    (9, 8, 2001, 0x9ea378, 0x9ea328), // Atalanta × Juventus
    (9, 8, 2001, 0x9ea358, 0x9ea370), // Chievo × Bologna
    (9, 8, 2001, 0x9ea368, 0x9ea34c), // Milan × Fiorentina
    (9, 8, 2001, 0x9ea30c, 0x9ea33c), // Parma × Inter
    (9, 8, 2001, 0x9ea344, 0x9ea314), // Perugia × Lazio
    (9, 8, 2001, 0x9ea31c, 0x9ea304), // Piacenza × Lecce
    (9, 8, 2001, 0x9ea2ec, 0x9ea2f4), // Roma × Udinese
    (9, 8, 2001, 0x9ea2fc, 0x9ea360), // Torino × Brescia
    (9, 8, 2001, 0x9ea334, 0x9ea2e4), // Venezia × Verona
    (16, 8, 2001, 0x9ea370, 0x9ea30c), // Bologna × Parma
    (16, 8, 2001, 0x9ea360, 0x9ea304), // Brescia × Lecce
    (16, 8, 2001, 0x9ea34c, 0x9ea378), // Fiorentina × Atalanta
    (16, 8, 2001, 0x9ea33c, 0x9ea334), // Inter × Venezia
    (16, 8, 2001, 0x9ea328, 0x9ea358), // Juventus × Chievo
    (16, 8, 2001, 0x9ea314, 0x9ea2fc), // Lazio × Torino
    (16, 8, 2001, 0x9ea31c, 0x9ea2ec), // Piacenza × Roma
    (16, 8, 2001, 0x9ea2f4, 0x9ea368), // Udinese × Milan
    (16, 8, 2001, 0x9ea2e4, 0x9ea344), // Verona × Perugia
    (23, 8, 2001, 0x9ea378, 0x9ea2e4), // Atalanta × Verona
    (23, 8, 2001, 0x9ea358, 0x9ea31c), // Chievo × Piacenza
    (23, 8, 2001, 0x9ea304, 0x9ea328), // Lecce × Juventus
    (23, 8, 2001, 0x9ea368, 0x9ea314), // Milan × Lazio
    (23, 8, 2001, 0x9ea30c, 0x9ea360), // Parma × Brescia
    (23, 8, 2001, 0x9ea344, 0x9ea2f4), // Perugia × Udinese
    (23, 8, 2001, 0x9ea2ec, 0x9ea34c), // Roma × Fiorentina
    (23, 8, 2001, 0x9ea2fc, 0x9ea33c), // Torino × Inter
    (23, 8, 2001, 0x9ea334, 0x9ea370), // Venezia × Bologna
    (30, 8, 2001, 0x9ea360, 0x9ea378), // Brescia × Atalanta
    (30, 8, 2001, 0x9ea34c, 0x9ea334), // Fiorentina × Venezia
    (30, 8, 2001, 0x9ea33c, 0x9ea370), // Inter × Bologna
    (30, 8, 2001, 0x9ea328, 0x9ea2ec), // Juventus × Roma
    (30, 8, 2001, 0x9ea314, 0x9ea30c), // Lazio × Parma
    (30, 8, 2001, 0x9ea344, 0x9ea368), // Perugia × Milan
    (30, 8, 2001, 0x9ea31c, 0x9ea2fc), // Piacenza × Torino
    (30, 8, 2001, 0x9ea2f4, 0x9ea358), // Udinese × Chievo
    (30, 8, 2001, 0x9ea2e4, 0x9ea304), // Verona × Lecce
    (10, 9, 2001, 0x9ea378, 0x9ea368), // Atalanta × Milan
    (10, 9, 2001, 0x9ea370, 0x9ea2f4), // Bologna × Udinese
    (10, 9, 2001, 0x9ea358, 0x9ea314), // Chievo × Lazio
    (10, 9, 2001, 0x9ea34c, 0x9ea328), // Fiorentina × Juventus
    (10, 9, 2001, 0x9ea33c, 0x9ea2e4), // Inter × Verona
    (10, 9, 2001, 0x9ea304, 0x9ea344), // Lecce × Perugia
    (10, 9, 2001, 0x9ea2ec, 0x9ea360), // Roma × Brescia
    (10, 9, 2001, 0x9ea2fc, 0x9ea30c), // Torino × Parma
    (10, 9, 2001, 0x9ea334, 0x9ea31c), // Venezia × Piacenza
    (14, 9, 2001, 0x9ea360, 0x9ea358), // Brescia × Chievo
    (14, 9, 2001, 0x9ea328, 0x9ea2fc), // Juventus × Torino
    (14, 9, 2001, 0x9ea314, 0x9ea378), // Lazio × Atalanta
    (14, 9, 2001, 0x9ea304, 0x9ea34c), // Lecce × Fiorentina
    (14, 9, 2001, 0x9ea368, 0x9ea334), // Milan × Venezia
    (14, 9, 2001, 0x9ea30c, 0x9ea31c), // Parma × Piacenza
    (14, 9, 2001, 0x9ea344, 0x9ea2ec), // Perugia × Roma
    (14, 9, 2001, 0x9ea2f4, 0x9ea33c), // Udinese × Inter
    (14, 9, 2001, 0x9ea2e4, 0x9ea370), // Verona × Bologna
    (21, 9, 2001, 0x9ea378, 0x9ea2f4), // Atalanta × Udinese
    (21, 9, 2001, 0x9ea370, 0x9ea328), // Bologna × Juventus
    (21, 9, 2001, 0x9ea358, 0x9ea30c), // Chievo × Parma
    (21, 9, 2001, 0x9ea34c, 0x9ea2e4), // Fiorentina × Verona
    (21, 9, 2001, 0x9ea33c, 0x9ea368), // Inter × Milan
    (21, 9, 2001, 0x9ea31c, 0x9ea360), // Piacenza × Brescia
    (21, 9, 2001, 0x9ea2ec, 0x9ea304), // Roma × Lecce
    (21, 9, 2001, 0x9ea2fc, 0x9ea344), // Torino × Perugia
    (21, 9, 2001, 0x9ea334, 0x9ea314), // Venezia × Lazio
    (28, 9, 2001, 0x9ea360, 0x9ea334), // Brescia × Venezia
    (28, 9, 2001, 0x9ea358, 0x9ea2fc), // Chievo × Torino
    (28, 9, 2001, 0x9ea328, 0x9ea33c), // Juventus × Inter
    (28, 9, 2001, 0x9ea304, 0x9ea378), // Lecce × Atalanta
    (28, 9, 2001, 0x9ea368, 0x9ea370), // Milan × Bologna
    (28, 9, 2001, 0x9ea30c, 0x9ea2e4), // Parma × Verona
    (28, 9, 2001, 0x9ea344, 0x9ea31c), // Perugia × Piacenza
    (28, 9, 2001, 0x9ea2ec, 0x9ea314), // Roma × Lazio
    (28, 9, 2001, 0x9ea2f4, 0x9ea34c), // Udinese × Fiorentina
    (4, 10, 2001, 0x9ea378, 0x9ea2ec), // Atalanta × Roma
    (4, 10, 2001, 0x9ea370, 0x9ea34c), // Bologna × Fiorentina
    (4, 10, 2001, 0x9ea33c, 0x9ea304), // Inter × Lecce
    (4, 10, 2001, 0x9ea314, 0x9ea360), // Lazio × Brescia
    (4, 10, 2001, 0x9ea30c, 0x9ea344), // Parma × Perugia
    (4, 10, 2001, 0x9ea31c, 0x9ea2f4), // Piacenza × Udinese
    (4, 10, 2001, 0x9ea2fc, 0x9ea368), // Torino × Milan
    (4, 10, 2001, 0x9ea334, 0x9ea358), // Venezia × Chievo
    (4, 10, 2001, 0x9ea2e4, 0x9ea328), // Verona × Juventus
    (18, 10, 2001, 0x9ea378, 0x9ea334), // Atalanta × Venezia
    (18, 10, 2001, 0x9ea34c, 0x9ea2fc), // Fiorentina × Torino
    (18, 10, 2001, 0x9ea328, 0x9ea30c), // Juventus × Parma
    (18, 10, 2001, 0x9ea304, 0x9ea370), // Lecce × Bologna
    (18, 10, 2001, 0x9ea368, 0x9ea31c), // Milan × Piacenza
    (18, 10, 2001, 0x9ea344, 0x9ea360), // Perugia × Brescia
    (18, 10, 2001, 0x9ea2ec, 0x9ea33c), // Roma × Inter
    (18, 10, 2001, 0x9ea2f4, 0x9ea314), // Udinese × Lazio
    (18, 10, 2001, 0x9ea2e4, 0x9ea358), // Verona × Chievo
    (25, 10, 2001, 0x9ea370, 0x9ea2ec), // Bologna × Roma
    (25, 10, 2001, 0x9ea360, 0x9ea2f4), // Brescia × Udinese
    (25, 10, 2001, 0x9ea358, 0x9ea344), // Chievo × Perugia
    (25, 10, 2001, 0x9ea33c, 0x9ea34c), // Inter × Fiorentina
    (25, 10, 2001, 0x9ea314, 0x9ea328), // Lazio × Juventus
    (25, 10, 2001, 0x9ea30c, 0x9ea368), // Parma × Milan
    (25, 10, 2001, 0x9ea31c, 0x9ea378), // Piacenza × Atalanta
    (25, 10, 2001, 0x9ea2fc, 0x9ea2e4), // Torino × Verona
    (25, 10, 2001, 0x9ea334, 0x9ea304), // Venezia × Lecce
    (2, 11, 2001, 0x9ea378, 0x9ea33c), // Atalanta × Inter
    (2, 11, 2001, 0x9ea370, 0x9ea2fc), // Bologna × Torino
    (2, 11, 2001, 0x9ea34c, 0x9ea31c), // Fiorentina × Piacenza
    (2, 11, 2001, 0x9ea328, 0x9ea344), // Juventus × Perugia
    (2, 11, 2001, 0x9ea304, 0x9ea314), // Lecce × Lazio
    (2, 11, 2001, 0x9ea368, 0x9ea358), // Milan × Chievo
    (2, 11, 2001, 0x9ea2ec, 0x9ea334), // Roma × Venezia
    (2, 11, 2001, 0x9ea2f4, 0x9ea30c), // Udinese × Parma
    (2, 11, 2001, 0x9ea2e4, 0x9ea360), // Verona × Brescia
    (9, 11, 2001, 0x9ea360, 0x9ea33c), // Brescia × Inter
    (9, 11, 2001, 0x9ea358, 0x9ea304), // Chievo × Lecce
    (9, 11, 2001, 0x9ea314, 0x9ea34c), // Lazio × Fiorentina
    (9, 11, 2001, 0x9ea368, 0x9ea328), // Milan × Juventus
    (9, 11, 2001, 0x9ea30c, 0x9ea2ec), // Parma × Roma
    (9, 11, 2001, 0x9ea344, 0x9ea334), // Perugia × Venezia
    (9, 11, 2001, 0x9ea31c, 0x9ea370), // Piacenza × Bologna
    (9, 11, 2001, 0x9ea2fc, 0x9ea378), // Torino × Atalanta
    (9, 11, 2001, 0x9ea2f4, 0x9ea2e4), // Udinese × Verona
    (16, 11, 2001, 0x9ea378, 0x9ea30c), // Atalanta × Parma
    (16, 11, 2001, 0x9ea370, 0x9ea344), // Bologna × Perugia
    (16, 11, 2001, 0x9ea34c, 0x9ea360), // Fiorentina × Brescia
    (16, 11, 2001, 0x9ea33c, 0x9ea358), // Inter × Chievo
    (16, 11, 2001, 0x9ea328, 0x9ea31c), // Juventus × Piacenza
    (16, 11, 2001, 0x9ea304, 0x9ea2fc), // Lecce × Torino
    (16, 11, 2001, 0x9ea2ec, 0x9ea368), // Roma × Milan
    (16, 11, 2001, 0x9ea334, 0x9ea2f4), // Venezia × Udinese
    (16, 11, 2001, 0x9ea2e4, 0x9ea314), // Verona × Lazio
    (6, 0, 2002, 0x9ea360, 0x9ea328), // Brescia × Juventus
    (6, 0, 2002, 0x9ea358, 0x9ea2ec), // Chievo × Roma
    (6, 0, 2002, 0x9ea314, 0x9ea370), // Lazio × Bologna
    (6, 0, 2002, 0x9ea368, 0x9ea2e4), // Milan × Verona
    (6, 0, 2002, 0x9ea30c, 0x9ea34c), // Parma × Fiorentina
    (6, 0, 2002, 0x9ea344, 0x9ea378), // Perugia × Atalanta
    (6, 0, 2002, 0x9ea31c, 0x9ea33c), // Piacenza × Inter
    (6, 0, 2002, 0x9ea2fc, 0x9ea334), // Torino × Venezia
    (6, 0, 2002, 0x9ea2f4, 0x9ea304), // Udinese × Lecce
    (13, 0, 2002, 0x9ea378, 0x9ea358), // Atalanta × Chievo
    (13, 0, 2002, 0x9ea370, 0x9ea360), // Bologna × Brescia
    (13, 0, 2002, 0x9ea34c, 0x9ea344), // Fiorentina × Perugia
    (13, 0, 2002, 0x9ea33c, 0x9ea314), // Inter × Lazio
    (13, 0, 2002, 0x9ea328, 0x9ea2f4), // Juventus × Udinese
    (13, 0, 2002, 0x9ea304, 0x9ea368), // Lecce × Milan
    (13, 0, 2002, 0x9ea2ec, 0x9ea2fc), // Roma × Torino
    (13, 0, 2002, 0x9ea334, 0x9ea30c), // Venezia × Parma
    (13, 0, 2002, 0x9ea2e4, 0x9ea31c), // Verona × Piacenza
    (20, 0, 2002, 0x9ea378, 0x9ea370), // Atalanta × Bologna
    (20, 0, 2002, 0x9ea368, 0x9ea360), // Milan × Brescia
    (20, 0, 2002, 0x9ea358, 0x9ea34c), // Chievo × Fiorentina
    (20, 0, 2002, 0x9ea344, 0x9ea33c), // Perugia × Inter
    (20, 0, 2002, 0x9ea334, 0x9ea328), // Venezia × Juventus
    (20, 0, 2002, 0x9ea31c, 0x9ea314), // Piacenza × Lazio
    (20, 0, 2002, 0x9ea30c, 0x9ea304), // Parma × Lecce
    (20, 0, 2002, 0x9ea2fc, 0x9ea2f4), // Torino × Udinese
    (20, 0, 2002, 0x9ea2ec, 0x9ea2e4), // Roma × Verona
    (27, 0, 2002, 0x9ea328, 0x9ea378), // Juventus × Atalanta
    (27, 0, 2002, 0x9ea370, 0x9ea358), // Bologna × Chievo
    (27, 0, 2002, 0x9ea34c, 0x9ea368), // Fiorentina × Milan
    (27, 0, 2002, 0x9ea33c, 0x9ea30c), // Inter × Parma
    (27, 0, 2002, 0x9ea314, 0x9ea344), // Lazio × Perugia
    (27, 0, 2002, 0x9ea304, 0x9ea31c), // Lecce × Piacenza
    (27, 0, 2002, 0x9ea2f4, 0x9ea2ec), // Udinese × Roma
    (27, 0, 2002, 0x9ea360, 0x9ea2fc), // Brescia × Torino
    (27, 0, 2002, 0x9ea2e4, 0x9ea334), // Verona × Venezia
    (3, 1, 2002, 0x9ea30c, 0x9ea370), // Parma × Bologna
    (3, 1, 2002, 0x9ea304, 0x9ea360), // Lecce × Brescia
    (3, 1, 2002, 0x9ea378, 0x9ea34c), // Atalanta × Fiorentina
    (3, 1, 2002, 0x9ea334, 0x9ea33c), // Venezia × Inter
    (3, 1, 2002, 0x9ea358, 0x9ea328), // Chievo × Juventus
    (3, 1, 2002, 0x9ea2fc, 0x9ea314), // Torino × Lazio
    (3, 1, 2002, 0x9ea2ec, 0x9ea31c), // Roma × Piacenza
    (3, 1, 2002, 0x9ea368, 0x9ea2f4), // Milan × Udinese
    (3, 1, 2002, 0x9ea344, 0x9ea2e4), // Perugia × Verona
    (10, 1, 2002, 0x9ea2e4, 0x9ea378), // Verona × Atalanta
    (10, 1, 2002, 0x9ea31c, 0x9ea358), // Piacenza × Chievo
    (10, 1, 2002, 0x9ea328, 0x9ea304), // Juventus × Lecce
    (10, 1, 2002, 0x9ea314, 0x9ea368), // Lazio × Milan
    (10, 1, 2002, 0x9ea360, 0x9ea30c), // Brescia × Parma
    (10, 1, 2002, 0x9ea2f4, 0x9ea344), // Udinese × Perugia
    (10, 1, 2002, 0x9ea34c, 0x9ea2ec), // Fiorentina × Roma
    (10, 1, 2002, 0x9ea33c, 0x9ea2fc), // Inter × Torino
    (10, 1, 2002, 0x9ea370, 0x9ea334), // Bologna × Venezia
    (17, 1, 2002, 0x9ea378, 0x9ea360), // Atalanta × Brescia
    (17, 1, 2002, 0x9ea334, 0x9ea34c), // Venezia × Fiorentina
    (17, 1, 2002, 0x9ea370, 0x9ea33c), // Bologna × Inter
    (17, 1, 2002, 0x9ea2ec, 0x9ea328), // Roma × Juventus
    (17, 1, 2002, 0x9ea30c, 0x9ea314), // Parma × Lazio
    (17, 1, 2002, 0x9ea368, 0x9ea344), // Milan × Perugia
    (17, 1, 2002, 0x9ea2fc, 0x9ea31c), // Torino × Piacenza
    (17, 1, 2002, 0x9ea358, 0x9ea2f4), // Chievo × Udinese
    (17, 1, 2002, 0x9ea304, 0x9ea2e4), // Lecce × Verona
    (24, 1, 2002, 0x9ea368, 0x9ea378), // Milan × Atalanta
    (24, 1, 2002, 0x9ea2f4, 0x9ea370), // Udinese × Bologna
    (24, 1, 2002, 0x9ea314, 0x9ea358), // Lazio × Chievo
    (24, 1, 2002, 0x9ea328, 0x9ea34c), // Juventus × Fiorentina
    (24, 1, 2002, 0x9ea2e4, 0x9ea33c), // Verona × Inter
    (24, 1, 2002, 0x9ea344, 0x9ea304), // Perugia × Lecce
    (24, 1, 2002, 0x9ea360, 0x9ea2ec), // Brescia × Roma
    (24, 1, 2002, 0x9ea30c, 0x9ea2fc), // Parma × Torino
    (24, 1, 2002, 0x9ea31c, 0x9ea334), // Piacenza × Venezia
    (3, 2, 2002, 0x9ea358, 0x9ea360), // Chievo × Brescia
    (3, 2, 2002, 0x9ea2fc, 0x9ea328), // Torino × Juventus
    (3, 2, 2002, 0x9ea378, 0x9ea314), // Atalanta × Lazio
    (3, 2, 2002, 0x9ea34c, 0x9ea304), // Fiorentina × Lecce
    (3, 2, 2002, 0x9ea334, 0x9ea368), // Venezia × Milan
    (3, 2, 2002, 0x9ea31c, 0x9ea30c), // Piacenza × Parma
    (3, 2, 2002, 0x9ea2ec, 0x9ea344), // Roma × Perugia
    (3, 2, 2002, 0x9ea33c, 0x9ea2f4), // Inter × Udinese
    (3, 2, 2002, 0x9ea370, 0x9ea2e4), // Bologna × Verona
    (10, 2, 2002, 0x9ea2f4, 0x9ea378), // Udinese × Atalanta
    (10, 2, 2002, 0x9ea328, 0x9ea370), // Juventus × Bologna
    (10, 2, 2002, 0x9ea30c, 0x9ea358), // Parma × Chievo
    (10, 2, 2002, 0x9ea2e4, 0x9ea34c), // Verona × Fiorentina
    (10, 2, 2002, 0x9ea368, 0x9ea33c), // Milan × Inter
    (10, 2, 2002, 0x9ea360, 0x9ea31c), // Brescia × Piacenza
    (10, 2, 2002, 0x9ea304, 0x9ea2ec), // Lecce × Roma
    (10, 2, 2002, 0x9ea344, 0x9ea2fc), // Perugia × Torino
    (10, 2, 2002, 0x9ea314, 0x9ea334), // Lazio × Venezia
    (17, 2, 2002, 0x9ea334, 0x9ea360), // Venezia × Brescia
    (17, 2, 2002, 0x9ea2fc, 0x9ea358), // Torino × Chievo
    (17, 2, 2002, 0x9ea33c, 0x9ea328), // Inter × Juventus
    (17, 2, 2002, 0x9ea378, 0x9ea304), // Atalanta × Lecce
    (17, 2, 2002, 0x9ea370, 0x9ea368), // Bologna × Milan
    (17, 2, 2002, 0x9ea2e4, 0x9ea30c), // Verona × Parma
    (17, 2, 2002, 0x9ea31c, 0x9ea344), // Piacenza × Perugia
    (17, 2, 2002, 0x9ea314, 0x9ea2ec), // Lazio × Roma
    (17, 2, 2002, 0x9ea34c, 0x9ea2f4), // Fiorentina × Udinese
    (24, 2, 2002, 0x9ea2ec, 0x9ea378), // Roma × Atalanta
    (24, 2, 2002, 0x9ea34c, 0x9ea370), // Fiorentina × Bologna
    (24, 2, 2002, 0x9ea304, 0x9ea33c), // Lecce × Inter
    (24, 2, 2002, 0x9ea360, 0x9ea314), // Brescia × Lazio
    (24, 2, 2002, 0x9ea344, 0x9ea30c), // Perugia × Parma
    (24, 2, 2002, 0x9ea2f4, 0x9ea31c), // Udinese × Piacenza
    (24, 2, 2002, 0x9ea368, 0x9ea2fc), // Milan × Torino
    (24, 2, 2002, 0x9ea358, 0x9ea334), // Chievo × Venezia
    (24, 2, 2002, 0x9ea328, 0x9ea2e4), // Juventus × Verona
    (30, 2, 2002, 0x9ea334, 0x9ea378), // Venezia × Atalanta
    (30, 2, 2002, 0x9ea2fc, 0x9ea34c), // Torino × Fiorentina
    (30, 2, 2002, 0x9ea30c, 0x9ea328), // Parma × Juventus
    (30, 2, 2002, 0x9ea370, 0x9ea304), // Bologna × Lecce
    (30, 2, 2002, 0x9ea31c, 0x9ea368), // Piacenza × Milan
    (30, 2, 2002, 0x9ea360, 0x9ea344), // Brescia × Perugia
    (30, 2, 2002, 0x9ea33c, 0x9ea2ec), // Inter × Roma
    (30, 2, 2002, 0x9ea314, 0x9ea2f4), // Lazio × Udinese
    (30, 2, 2002, 0x9ea358, 0x9ea2e4), // Chievo × Verona
    (7, 3, 2002, 0x9ea2ec, 0x9ea370), // Roma × Bologna
    (7, 3, 2002, 0x9ea2f4, 0x9ea360), // Udinese × Brescia
    (7, 3, 2002, 0x9ea344, 0x9ea358), // Perugia × Chievo
    (7, 3, 2002, 0x9ea34c, 0x9ea33c), // Fiorentina × Inter
    (7, 3, 2002, 0x9ea328, 0x9ea314), // Juventus × Lazio
    (7, 3, 2002, 0x9ea368, 0x9ea30c), // Milan × Parma
    (7, 3, 2002, 0x9ea378, 0x9ea31c), // Atalanta × Piacenza
    (7, 3, 2002, 0x9ea2e4, 0x9ea2fc), // Verona × Torino
    (7, 3, 2002, 0x9ea304, 0x9ea334), // Lecce × Venezia
    (14, 3, 2002, 0x9ea33c, 0x9ea378), // Inter × Atalanta
    (14, 3, 2002, 0x9ea2fc, 0x9ea370), // Torino × Bologna
    (14, 3, 2002, 0x9ea31c, 0x9ea34c), // Piacenza × Fiorentina
    (14, 3, 2002, 0x9ea344, 0x9ea328), // Perugia × Juventus
    (14, 3, 2002, 0x9ea314, 0x9ea304), // Lazio × Lecce
    (14, 3, 2002, 0x9ea358, 0x9ea368), // Chievo × Milan
    (14, 3, 2002, 0x9ea334, 0x9ea2ec), // Venezia × Roma
    (14, 3, 2002, 0x9ea30c, 0x9ea2f4), // Parma × Udinese
    (14, 3, 2002, 0x9ea360, 0x9ea2e4), // Brescia × Verona
    (21, 3, 2002, 0x9ea33c, 0x9ea360), // Inter × Brescia
    (21, 3, 2002, 0x9ea304, 0x9ea358), // Lecce × Chievo
    (21, 3, 2002, 0x9ea34c, 0x9ea314), // Fiorentina × Lazio
    (21, 3, 2002, 0x9ea328, 0x9ea368), // Juventus × Milan
    (21, 3, 2002, 0x9ea2ec, 0x9ea30c), // Roma × Parma
    (21, 3, 2002, 0x9ea334, 0x9ea344), // Venezia × Perugia
    (21, 3, 2002, 0x9ea370, 0x9ea31c), // Bologna × Piacenza
    (21, 3, 2002, 0x9ea378, 0x9ea2fc), // Atalanta × Torino
    (21, 3, 2002, 0x9ea2e4, 0x9ea2f4), // Verona × Udinese
    (28, 3, 2002, 0x9ea30c, 0x9ea378), // Parma × Atalanta
    (28, 3, 2002, 0x9ea344, 0x9ea370), // Perugia × Bologna
    (28, 3, 2002, 0x9ea360, 0x9ea34c), // Brescia × Fiorentina
    (28, 3, 2002, 0x9ea358, 0x9ea33c), // Chievo × Inter
    (28, 3, 2002, 0x9ea31c, 0x9ea328), // Piacenza × Juventus
    (28, 3, 2002, 0x9ea2fc, 0x9ea304), // Torino × Lecce
    (28, 3, 2002, 0x9ea368, 0x9ea2ec), // Milan × Roma
    (28, 3, 2002, 0x9ea2f4, 0x9ea334), // Udinese × Venezia
    (28, 3, 2002, 0x9ea314, 0x9ea2e4), // Lazio × Verona
    (5, 4, 2002, 0x9ea328, 0x9ea360), // Juventus × Brescia
    (5, 4, 2002, 0x9ea2ec, 0x9ea358), // Roma × Chievo
    (5, 4, 2002, 0x9ea370, 0x9ea314), // Bologna × Lazio
    (5, 4, 2002, 0x9ea2e4, 0x9ea368), // Verona × Milan
    (5, 4, 2002, 0x9ea34c, 0x9ea30c), // Fiorentina × Parma
    (5, 4, 2002, 0x9ea378, 0x9ea344), // Atalanta × Perugia
    (5, 4, 2002, 0x9ea33c, 0x9ea31c), // Inter × Piacenza
    (5, 4, 2002, 0x9ea334, 0x9ea2fc), // Venezia × Torino
    (5, 4, 2002, 0x9ea304, 0x9ea2f4), // Lecce × Udinese
    (12, 4, 2002, 0x9ea358, 0x9ea378), // Chievo × Atalanta
    (12, 4, 2002, 0x9ea360, 0x9ea370), // Brescia × Bologna
    (12, 4, 2002, 0x9ea344, 0x9ea34c), // Perugia × Fiorentina
    (12, 4, 2002, 0x9ea314, 0x9ea33c), // Lazio × Inter
    (12, 4, 2002, 0x9ea2f4, 0x9ea328), // Udinese × Juventus
    (12, 4, 2002, 0x9ea368, 0x9ea304), // Milan × Lecce
    (12, 4, 2002, 0x9ea2fc, 0x9ea2ec), // Torino × Roma
    (12, 4, 2002, 0x9ea30c, 0x9ea334), // Parma × Venezia
    (12, 4, 2002, 0x9ea31c, 0x9ea2e4), // Piacenza × Verona
];

pub(crate) const SERIE_B_FIXTURES: [(u8, u8, u16, u32, u32); 380] = [
    (26, 7, 2001, 0x9ea464, 0x9ea46c), // Ancona × Reggina
    (26, 7, 2001, 0x9ea450, 0x9ea45c), // Cagliari × Messina
    (26, 7, 2001, 0x9ea440, 0x9ea448), // Como × Crotone
    (26, 7, 2001, 0x9ea42c, 0x9ea434), // Cosenza × Cittadella
    (26, 7, 2001, 0x9ea41c, 0x9ea424), // Empoli × Palermo
    (26, 7, 2001, 0x9ea40c, 0x9ea414), // Genoa × Napoli
    (26, 7, 2001, 0x9ea3fc, 0x9ea404), // Modena × Bari
    (26, 7, 2001, 0x9ea3e4, 0x9ea3f0), // Salernitana × Sampdoria
    (26, 7, 2001, 0x9ea3d4, 0x9ea3dc), // Ternana × Siena
    (26, 7, 2001, 0x9ea3c0, 0x9ea3c8), // Vicenza × Pistoiese
    (2, 8, 2001, 0x9ea404, 0x9ea41c), // Bari × Empoli
    (2, 8, 2001, 0x9ea434, 0x9ea3e4), // Cittadella × Salernitana
    (2, 8, 2001, 0x9ea448, 0x9ea3fc), // Crotone × Modena
    (2, 8, 2001, 0x9ea45c, 0x9ea3d4), // Messina × Ternana
    (2, 8, 2001, 0x9ea414, 0x9ea464), // Napoli × Ancona
    (2, 8, 2001, 0x9ea424, 0x9ea450), // Palermo × Cagliari
    (2, 8, 2001, 0x9ea3c8, 0x9ea40c), // Pistoiese × Genoa
    (2, 8, 2001, 0x9ea46c, 0x9ea440), // Reggina × Como
    (2, 8, 2001, 0x9ea3f0, 0x9ea42c), // Sampdoria × Cosenza
    (2, 8, 2001, 0x9ea3dc, 0x9ea3c0), // Siena × Vicenza
    (9, 8, 2001, 0x9ea464, 0x9ea3dc), // Ancona × Siena
    (9, 8, 2001, 0x9ea450, 0x9ea404), // Cagliari × Bari
    (9, 8, 2001, 0x9ea42c, 0x9ea3d4), // Cosenza × Ternana
    (9, 8, 2001, 0x9ea41c, 0x9ea45c), // Empoli × Messina
    (9, 8, 2001, 0x9ea40c, 0x9ea440), // Genoa × Como
    (9, 8, 2001, 0x9ea3fc, 0x9ea414), // Modena × Napoli
    (9, 8, 2001, 0x9ea424, 0x9ea3c8), // Palermo × Pistoiese
    (9, 8, 2001, 0x9ea46c, 0x9ea3f0), // Reggina × Sampdoria
    (9, 8, 2001, 0x9ea3e4, 0x9ea448), // Salernitana × Crotone
    (9, 8, 2001, 0x9ea3c0, 0x9ea434), // Vicenza × Cittadella
    (16, 8, 2001, 0x9ea404, 0x9ea424), // Bari × Palermo
    (16, 8, 2001, 0x9ea434, 0x9ea46c), // Cittadella × Reggina
    (16, 8, 2001, 0x9ea440, 0x9ea3e4), // Como × Salernitana
    (16, 8, 2001, 0x9ea448, 0x9ea450), // Crotone × Cagliari
    (16, 8, 2001, 0x9ea45c, 0x9ea464), // Messina × Ancona
    (16, 8, 2001, 0x9ea414, 0x9ea3c0), // Napoli × Vicenza
    (16, 8, 2001, 0x9ea3c8, 0x9ea42c), // Pistoiese × Cosenza
    (16, 8, 2001, 0x9ea3f0, 0x9ea41c), // Sampdoria × Empoli
    (16, 8, 2001, 0x9ea3dc, 0x9ea40c), // Siena × Genoa
    (16, 8, 2001, 0x9ea3d4, 0x9ea3fc), // Ternana × Modena
    (23, 8, 2001, 0x9ea464, 0x9ea448), // Ancona × Crotone
    (23, 8, 2001, 0x9ea450, 0x9ea3f0), // Cagliari × Sampdoria
    (23, 8, 2001, 0x9ea42c, 0x9ea440), // Cosenza × Como
    (23, 8, 2001, 0x9ea41c, 0x9ea414), // Empoli × Napoli
    (23, 8, 2001, 0x9ea40c, 0x9ea45c), // Genoa × Messina
    (23, 8, 2001, 0x9ea3fc, 0x9ea3dc), // Modena × Siena
    (23, 8, 2001, 0x9ea424, 0x9ea434), // Palermo × Cittadella
    (23, 8, 2001, 0x9ea46c, 0x9ea404), // Reggina × Bari
    (23, 8, 2001, 0x9ea3e4, 0x9ea3c8), // Salernitana × Pistoiese
    (23, 8, 2001, 0x9ea3c0, 0x9ea3d4), // Vicenza × Ternana
    (30, 8, 2001, 0x9ea404, 0x9ea448), // Bari × Crotone
    (30, 8, 2001, 0x9ea434, 0x9ea41c), // Cittadella × Empoli
    (30, 8, 2001, 0x9ea440, 0x9ea464), // Como × Ancona
    (30, 8, 2001, 0x9ea42c, 0x9ea46c), // Cosenza × Reggina
    (30, 8, 2001, 0x9ea45c, 0x9ea3c0), // Messina × Vicenza
    (30, 8, 2001, 0x9ea414, 0x9ea450), // Napoli × Cagliari
    (30, 8, 2001, 0x9ea3c8, 0x9ea3fc), // Pistoiese × Modena
    (30, 8, 2001, 0x9ea3f0, 0x9ea424), // Sampdoria × Palermo
    (30, 8, 2001, 0x9ea3dc, 0x9ea3e4), // Siena × Salernitana
    (30, 8, 2001, 0x9ea3d4, 0x9ea40c), // Ternana × Genoa
    (7, 9, 2001, 0x9ea464, 0x9ea40c), // Ancona × Genoa
    (7, 9, 2001, 0x9ea450, 0x9ea3dc), // Cagliari × Siena
    (7, 9, 2001, 0x9ea448, 0x9ea434), // Crotone × Cittadella
    (7, 9, 2001, 0x9ea41c, 0x9ea3c8), // Empoli × Pistoiese
    (7, 9, 2001, 0x9ea3fc, 0x9ea440), // Modena × Como
    (7, 9, 2001, 0x9ea46c, 0x9ea424), // Reggina × Palermo
    (7, 9, 2001, 0x9ea3e4, 0x9ea45c), // Salernitana × Messina
    (7, 9, 2001, 0x9ea3f0, 0x9ea404), // Sampdoria × Bari
    (7, 9, 2001, 0x9ea3d4, 0x9ea414), // Ternana × Napoli
    (7, 9, 2001, 0x9ea3c0, 0x9ea42c), // Vicenza × Cosenza
    (10, 9, 2001, 0x9ea464, 0x9ea3f0), // Ancona × Sampdoria
    (10, 9, 2001, 0x9ea440, 0x9ea3d4), // Como × Ternana
    (10, 9, 2001, 0x9ea42c, 0x9ea3fc), // Cosenza × Modena
    (10, 9, 2001, 0x9ea41c, 0x9ea450), // Empoli × Cagliari
    (10, 9, 2001, 0x9ea40c, 0x9ea434), // Genoa × Cittadella
    (10, 9, 2001, 0x9ea45c, 0x9ea3dc), // Messina × Siena
    (10, 9, 2001, 0x9ea414, 0x9ea3e4), // Napoli × Salernitana
    (10, 9, 2001, 0x9ea424, 0x9ea448), // Palermo × Crotone
    (10, 9, 2001, 0x9ea3c8, 0x9ea46c), // Pistoiese × Reggina
    (10, 9, 2001, 0x9ea3c0, 0x9ea404), // Vicenza × Bari
    (14, 9, 2001, 0x9ea404, 0x9ea40c), // Bari × Genoa
    (14, 9, 2001, 0x9ea434, 0x9ea414), // Cittadella × Napoli
    (14, 9, 2001, 0x9ea448, 0x9ea3c0), // Crotone × Vicenza
    (14, 9, 2001, 0x9ea45c, 0x9ea424), // Messina × Palermo
    (14, 9, 2001, 0x9ea3fc, 0x9ea450), // Modena × Cagliari
    (14, 9, 2001, 0x9ea46c, 0x9ea41c), // Reggina × Empoli
    (14, 9, 2001, 0x9ea3e4, 0x9ea42c), // Salernitana × Cosenza
    (14, 9, 2001, 0x9ea3f0, 0x9ea3c8), // Sampdoria × Pistoiese
    (14, 9, 2001, 0x9ea3dc, 0x9ea440), // Siena × Como
    (14, 9, 2001, 0x9ea3d4, 0x9ea464), // Ternana × Ancona
    (28, 9, 2001, 0x9ea464, 0x9ea434), // Ancona × Cittadella
    (28, 9, 2001, 0x9ea450, 0x9ea46c), // Cagliari × Reggina
    (28, 9, 2001, 0x9ea440, 0x9ea45c), // Como × Messina
    (28, 9, 2001, 0x9ea42c, 0x9ea3dc), // Cosenza × Siena
    (28, 9, 2001, 0x9ea41c, 0x9ea448), // Empoli × Crotone
    (28, 9, 2001, 0x9ea40c, 0x9ea3e4), // Genoa × Salernitana
    (28, 9, 2001, 0x9ea414, 0x9ea3f0), // Napoli × Sampdoria
    (28, 9, 2001, 0x9ea424, 0x9ea3d4), // Palermo × Ternana
    (28, 9, 2001, 0x9ea3c8, 0x9ea404), // Pistoiese × Bari
    (28, 9, 2001, 0x9ea3c0, 0x9ea3fc), // Vicenza × Modena
    (4, 10, 2001, 0x9ea404, 0x9ea414), // Bari × Napoli
    (4, 10, 2001, 0x9ea434, 0x9ea450), // Cittadella × Cagliari
    (4, 10, 2001, 0x9ea440, 0x9ea3c0), // Como × Vicenza
    (4, 10, 2001, 0x9ea448, 0x9ea46c), // Crotone × Reggina
    (4, 10, 2001, 0x9ea40c, 0x9ea3f0), // Genoa × Sampdoria
    (4, 10, 2001, 0x9ea45c, 0x9ea42c), // Messina × Cosenza
    (4, 10, 2001, 0x9ea3fc, 0x9ea41c), // Modena × Empoli
    (4, 10, 2001, 0x9ea3e4, 0x9ea464), // Salernitana × Ancona
    (4, 10, 2001, 0x9ea3dc, 0x9ea424), // Siena × Palermo
    (4, 10, 2001, 0x9ea3d4, 0x9ea3c8), // Ternana × Pistoiese
    (11, 10, 2001, 0x9ea404, 0x9ea3dc), // Bari × Siena
    (11, 10, 2001, 0x9ea450, 0x9ea3d4), // Cagliari × Ternana
    (11, 10, 2001, 0x9ea42c, 0x9ea40c), // Cosenza × Genoa
    (11, 10, 2001, 0x9ea41c, 0x9ea3e4), // Empoli × Salernitana
    (11, 10, 2001, 0x9ea414, 0x9ea448), // Napoli × Crotone
    (11, 10, 2001, 0x9ea424, 0x9ea3fc), // Palermo × Modena
    (11, 10, 2001, 0x9ea3c8, 0x9ea440), // Pistoiese × Como
    (11, 10, 2001, 0x9ea46c, 0x9ea45c), // Reggina × Messina
    (11, 10, 2001, 0x9ea3f0, 0x9ea434), // Sampdoria × Cittadella
    (11, 10, 2001, 0x9ea3c0, 0x9ea464), // Vicenza × Ancona
    (18, 10, 2001, 0x9ea464, 0x9ea42c), // Ancona × Cosenza
    (18, 10, 2001, 0x9ea434, 0x9ea404), // Cittadella × Bari
    (18, 10, 2001, 0x9ea440, 0x9ea424), // Como × Palermo
    (18, 10, 2001, 0x9ea448, 0x9ea3f0), // Crotone × Sampdoria
    (18, 10, 2001, 0x9ea40c, 0x9ea3c0), // Genoa × Vicenza
    (18, 10, 2001, 0x9ea45c, 0x9ea414), // Messina × Napoli
    (18, 10, 2001, 0x9ea3fc, 0x9ea46c), // Modena × Reggina
    (18, 10, 2001, 0x9ea3e4, 0x9ea450), // Salernitana × Cagliari
    (18, 10, 2001, 0x9ea3dc, 0x9ea3c8), // Siena × Pistoiese
    (18, 10, 2001, 0x9ea3d4, 0x9ea41c), // Ternana × Empoli
    (25, 10, 2001, 0x9ea404, 0x9ea440), // Bari × Como
    (25, 10, 2001, 0x9ea450, 0x9ea464), // Cagliari × Ancona
    (25, 10, 2001, 0x9ea434, 0x9ea3d4), // Cittadella × Ternana
    (25, 10, 2001, 0x9ea448, 0x9ea42c), // Crotone × Cosenza
    (25, 10, 2001, 0x9ea41c, 0x9ea3dc), // Empoli × Siena
    (25, 10, 2001, 0x9ea424, 0x9ea40c), // Palermo × Genoa
    (25, 10, 2001, 0x9ea3c8, 0x9ea45c), // Pistoiese × Messina
    (25, 10, 2001, 0x9ea46c, 0x9ea414), // Reggina × Napoli
    (25, 10, 2001, 0x9ea3e4, 0x9ea3c0), // Salernitana × Vicenza
    (25, 10, 2001, 0x9ea3f0, 0x9ea3fc), // Sampdoria × Modena
    (2, 11, 2001, 0x9ea464, 0x9ea3c8), // Ancona × Pistoiese
    (2, 11, 2001, 0x9ea440, 0x9ea450), // Como × Cagliari
    (2, 11, 2001, 0x9ea42c, 0x9ea404), // Cosenza × Bari
    (2, 11, 2001, 0x9ea40c, 0x9ea448), // Genoa × Crotone
    (2, 11, 2001, 0x9ea45c, 0x9ea3f0), // Messina × Sampdoria
    (2, 11, 2001, 0x9ea3fc, 0x9ea434), // Modena × Cittadella
    (2, 11, 2001, 0x9ea414, 0x9ea424), // Napoli × Palermo
    (2, 11, 2001, 0x9ea3dc, 0x9ea46c), // Siena × Reggina
    (2, 11, 2001, 0x9ea3d4, 0x9ea3e4), // Ternana × Salernitana
    (2, 11, 2001, 0x9ea3c0, 0x9ea41c), // Vicenza × Empoli
    (9, 11, 2001, 0x9ea404, 0x9ea464), // Bari × Ancona
    (9, 11, 2001, 0x9ea450, 0x9ea3c8), // Cagliari × Pistoiese
    (9, 11, 2001, 0x9ea434, 0x9ea45c), // Cittadella × Messina
    (9, 11, 2001, 0x9ea448, 0x9ea3dc), // Crotone × Siena
    (9, 11, 2001, 0x9ea41c, 0x9ea440), // Empoli × Como
    (9, 11, 2001, 0x9ea3fc, 0x9ea40c), // Modena × Genoa
    (9, 11, 2001, 0x9ea414, 0x9ea42c), // Napoli × Cosenza
    (9, 11, 2001, 0x9ea424, 0x9ea3c0), // Palermo × Vicenza
    (9, 11, 2001, 0x9ea46c, 0x9ea3e4), // Reggina × Salernitana
    (9, 11, 2001, 0x9ea3f0, 0x9ea3d4), // Sampdoria × Ternana
    (16, 11, 2001, 0x9ea464, 0x9ea3fc), // Ancona × Modena
    (16, 11, 2001, 0x9ea440, 0x9ea3f0), // Como × Sampdoria
    (16, 11, 2001, 0x9ea42c, 0x9ea450), // Cosenza × Cagliari
    (16, 11, 2001, 0x9ea40c, 0x9ea41c), // Genoa × Empoli
    (16, 11, 2001, 0x9ea45c, 0x9ea404), // Messina × Bari
    (16, 11, 2001, 0x9ea3c8, 0x9ea414), // Pistoiese × Napoli
    (16, 11, 2001, 0x9ea3e4, 0x9ea424), // Salernitana × Palermo
    (16, 11, 2001, 0x9ea3dc, 0x9ea434), // Siena × Cittadella
    (16, 11, 2001, 0x9ea3d4, 0x9ea448), // Ternana × Crotone
    (16, 11, 2001, 0x9ea3c0, 0x9ea46c), // Vicenza × Reggina
    (23, 11, 2001, 0x9ea404, 0x9ea3e4), // Bari × Salernitana
    (23, 11, 2001, 0x9ea450, 0x9ea40c), // Cagliari × Genoa
    (23, 11, 2001, 0x9ea434, 0x9ea440), // Cittadella × Como
    (23, 11, 2001, 0x9ea448, 0x9ea3c8), // Crotone × Pistoiese
    (23, 11, 2001, 0x9ea41c, 0x9ea464), // Empoli × Ancona
    (23, 11, 2001, 0x9ea3fc, 0x9ea45c), // Modena × Messina
    (23, 11, 2001, 0x9ea414, 0x9ea3dc), // Napoli × Siena
    (23, 11, 2001, 0x9ea424, 0x9ea42c), // Palermo × Cosenza
    (23, 11, 2001, 0x9ea46c, 0x9ea3d4), // Reggina × Ternana
    (23, 11, 2001, 0x9ea3f0, 0x9ea3c0), // Sampdoria × Vicenza
    (6, 0, 2002, 0x9ea464, 0x9ea424), // Ancona × Palermo
    (6, 0, 2002, 0x9ea440, 0x9ea414), // Como × Napoli
    (6, 0, 2002, 0x9ea42c, 0x9ea41c), // Cosenza × Empoli
    (6, 0, 2002, 0x9ea40c, 0x9ea46c), // Genoa × Reggina
    (6, 0, 2002, 0x9ea45c, 0x9ea448), // Messina × Crotone
    (6, 0, 2002, 0x9ea3c8, 0x9ea434), // Pistoiese × Cittadella
    (6, 0, 2002, 0x9ea3e4, 0x9ea3fc), // Salernitana × Modena
    (6, 0, 2002, 0x9ea3dc, 0x9ea3f0), // Siena × Sampdoria
    (6, 0, 2002, 0x9ea3d4, 0x9ea404), // Ternana × Bari
    (6, 0, 2002, 0x9ea3c0, 0x9ea450), // Vicenza × Cagliari
    (13, 0, 2002, 0x9ea46c, 0x9ea464), // Reggina × Ancona
    (13, 0, 2002, 0x9ea45c, 0x9ea450), // Messina × Cagliari
    (13, 0, 2002, 0x9ea448, 0x9ea440), // Crotone × Como
    (13, 0, 2002, 0x9ea434, 0x9ea42c), // Cittadella × Cosenza
    (13, 0, 2002, 0x9ea424, 0x9ea41c), // Palermo × Empoli
    (13, 0, 2002, 0x9ea414, 0x9ea40c), // Napoli × Genoa
    (13, 0, 2002, 0x9ea404, 0x9ea3fc), // Bari × Modena
    (13, 0, 2002, 0x9ea3f0, 0x9ea3e4), // Sampdoria × Salernitana
    (13, 0, 2002, 0x9ea3dc, 0x9ea3d4), // Siena × Ternana
    (13, 0, 2002, 0x9ea3c8, 0x9ea3c0), // Pistoiese × Vicenza
    (20, 0, 2002, 0x9ea41c, 0x9ea404), // Empoli × Bari
    (20, 0, 2002, 0x9ea3e4, 0x9ea434), // Salernitana × Cittadella
    (20, 0, 2002, 0x9ea3fc, 0x9ea448), // Modena × Crotone
    (20, 0, 2002, 0x9ea3d4, 0x9ea45c), // Ternana × Messina
    (20, 0, 2002, 0x9ea464, 0x9ea414), // Ancona × Napoli
    (20, 0, 2002, 0x9ea450, 0x9ea424), // Cagliari × Palermo
    (20, 0, 2002, 0x9ea40c, 0x9ea3c8), // Genoa × Pistoiese
    (20, 0, 2002, 0x9ea440, 0x9ea46c), // Como × Reggina
    (20, 0, 2002, 0x9ea42c, 0x9ea3f0), // Cosenza × Sampdoria
    (20, 0, 2002, 0x9ea3c0, 0x9ea3dc), // Vicenza × Siena
    (3, 1, 2002, 0x9ea3dc, 0x9ea464), // Siena × Ancona
    (3, 1, 2002, 0x9ea404, 0x9ea450), // Bari × Cagliari
    (3, 1, 2002, 0x9ea3d4, 0x9ea42c), // Ternana × Cosenza
    (3, 1, 2002, 0x9ea45c, 0x9ea41c), // Messina × Empoli
    (3, 1, 2002, 0x9ea440, 0x9ea40c), // Como × Genoa
    (3, 1, 2002, 0x9ea414, 0x9ea3fc), // Napoli × Modena
    (3, 1, 2002, 0x9ea3c8, 0x9ea424), // Pistoiese × Palermo
    (3, 1, 2002, 0x9ea3f0, 0x9ea46c), // Sampdoria × Reggina
    (3, 1, 2002, 0x9ea448, 0x9ea3e4), // Crotone × Salernitana
    (3, 1, 2002, 0x9ea434, 0x9ea3c0), // Cittadella × Vicenza
    (10, 1, 2002, 0x9ea424, 0x9ea404), // Palermo × Bari
    (10, 1, 2002, 0x9ea46c, 0x9ea434), // Reggina × Cittadella
    (10, 1, 2002, 0x9ea3e4, 0x9ea440), // Salernitana × Como
    (10, 1, 2002, 0x9ea450, 0x9ea448), // Cagliari × Crotone
    (10, 1, 2002, 0x9ea464, 0x9ea45c), // Ancona × Messina
    (10, 1, 2002, 0x9ea3c0, 0x9ea414), // Vicenza × Napoli
    (10, 1, 2002, 0x9ea42c, 0x9ea3c8), // Cosenza × Pistoiese
    (10, 1, 2002, 0x9ea41c, 0x9ea3f0), // Empoli × Sampdoria
    (10, 1, 2002, 0x9ea40c, 0x9ea3dc), // Genoa × Siena
    (10, 1, 2002, 0x9ea3fc, 0x9ea3d4), // Modena × Ternana
    (17, 1, 2002, 0x9ea448, 0x9ea464), // Crotone × Ancona
    (17, 1, 2002, 0x9ea3f0, 0x9ea450), // Sampdoria × Cagliari
    (17, 1, 2002, 0x9ea440, 0x9ea42c), // Como × Cosenza
    (17, 1, 2002, 0x9ea414, 0x9ea41c), // Napoli × Empoli
    (17, 1, 2002, 0x9ea45c, 0x9ea40c), // Messina × Genoa
    (17, 1, 2002, 0x9ea3dc, 0x9ea3fc), // Siena × Modena
    (17, 1, 2002, 0x9ea434, 0x9ea424), // Cittadella × Palermo
    (17, 1, 2002, 0x9ea404, 0x9ea46c), // Bari × Reggina
    (17, 1, 2002, 0x9ea3c8, 0x9ea3e4), // Pistoiese × Salernitana
    (17, 1, 2002, 0x9ea3d4, 0x9ea3c0), // Ternana × Vicenza
    (24, 1, 2002, 0x9ea448, 0x9ea404), // Crotone × Bari
    (24, 1, 2002, 0x9ea41c, 0x9ea434), // Empoli × Cittadella
    (24, 1, 2002, 0x9ea464, 0x9ea440), // Ancona × Como
    (24, 1, 2002, 0x9ea46c, 0x9ea42c), // Reggina × Cosenza
    (24, 1, 2002, 0x9ea3c0, 0x9ea45c), // Vicenza × Messina
    (24, 1, 2002, 0x9ea450, 0x9ea414), // Cagliari × Napoli
    (24, 1, 2002, 0x9ea3fc, 0x9ea3c8), // Modena × Pistoiese
    (24, 1, 2002, 0x9ea424, 0x9ea3f0), // Palermo × Sampdoria
    (24, 1, 2002, 0x9ea3e4, 0x9ea3dc), // Salernitana × Siena
    (24, 1, 2002, 0x9ea40c, 0x9ea3d4), // Genoa × Ternana
    (3, 2, 2002, 0x9ea40c, 0x9ea464), // Genoa × Ancona
    (3, 2, 2002, 0x9ea3dc, 0x9ea450), // Siena × Cagliari
    (3, 2, 2002, 0x9ea434, 0x9ea448), // Cittadella × Crotone
    (3, 2, 2002, 0x9ea3c8, 0x9ea41c), // Pistoiese × Empoli
    (3, 2, 2002, 0x9ea440, 0x9ea3fc), // Como × Modena
    (3, 2, 2002, 0x9ea424, 0x9ea46c), // Palermo × Reggina
    (3, 2, 2002, 0x9ea45c, 0x9ea3e4), // Messina × Salernitana
    (3, 2, 2002, 0x9ea404, 0x9ea3f0), // Bari × Sampdoria
    (3, 2, 2002, 0x9ea414, 0x9ea3d4), // Napoli × Ternana
    (3, 2, 2002, 0x9ea42c, 0x9ea3c0), // Cosenza × Vicenza
    (10, 2, 2002, 0x9ea3f0, 0x9ea464), // Sampdoria × Ancona
    (10, 2, 2002, 0x9ea3d4, 0x9ea440), // Ternana × Como
    (10, 2, 2002, 0x9ea3fc, 0x9ea42c), // Modena × Cosenza
    (10, 2, 2002, 0x9ea450, 0x9ea41c), // Cagliari × Empoli
    (10, 2, 2002, 0x9ea434, 0x9ea40c), // Cittadella × Genoa
    (10, 2, 2002, 0x9ea3dc, 0x9ea45c), // Siena × Messina
    (10, 2, 2002, 0x9ea3e4, 0x9ea414), // Salernitana × Napoli
    (10, 2, 2002, 0x9ea448, 0x9ea424), // Crotone × Palermo
    (10, 2, 2002, 0x9ea46c, 0x9ea3c8), // Reggina × Pistoiese
    (10, 2, 2002, 0x9ea404, 0x9ea3c0), // Bari × Vicenza
    (17, 2, 2002, 0x9ea40c, 0x9ea404), // Genoa × Bari
    (17, 2, 2002, 0x9ea414, 0x9ea434), // Napoli × Cittadella
    (17, 2, 2002, 0x9ea3c0, 0x9ea448), // Vicenza × Crotone
    (17, 2, 2002, 0x9ea424, 0x9ea45c), // Palermo × Messina
    (17, 2, 2002, 0x9ea450, 0x9ea3fc), // Cagliari × Modena
    (17, 2, 2002, 0x9ea41c, 0x9ea46c), // Empoli × Reggina
    (17, 2, 2002, 0x9ea42c, 0x9ea3e4), // Cosenza × Salernitana
    (17, 2, 2002, 0x9ea3c8, 0x9ea3f0), // Pistoiese × Sampdoria
    (17, 2, 2002, 0x9ea440, 0x9ea3dc), // Como × Siena
    (17, 2, 2002, 0x9ea464, 0x9ea3d4), // Ancona × Ternana
    (30, 2, 2002, 0x9ea434, 0x9ea464), // Cittadella × Ancona
    (30, 2, 2002, 0x9ea46c, 0x9ea450), // Reggina × Cagliari
    (30, 2, 2002, 0x9ea45c, 0x9ea440), // Messina × Como
    (30, 2, 2002, 0x9ea3dc, 0x9ea42c), // Siena × Cosenza
    (30, 2, 2002, 0x9ea448, 0x9ea41c), // Crotone × Empoli
    (30, 2, 2002, 0x9ea3e4, 0x9ea40c), // Salernitana × Genoa
    (30, 2, 2002, 0x9ea3f0, 0x9ea414), // Sampdoria × Napoli
    (30, 2, 2002, 0x9ea3d4, 0x9ea424), // Ternana × Palermo
    (30, 2, 2002, 0x9ea404, 0x9ea3c8), // Bari × Pistoiese
    (30, 2, 2002, 0x9ea3fc, 0x9ea3c0), // Modena × Vicenza
    (7, 3, 2002, 0x9ea414, 0x9ea404), // Napoli × Bari
    (7, 3, 2002, 0x9ea450, 0x9ea434), // Cagliari × Cittadella
    (7, 3, 2002, 0x9ea3c0, 0x9ea440), // Vicenza × Como
    (7, 3, 2002, 0x9ea46c, 0x9ea448), // Reggina × Crotone
    (7, 3, 2002, 0x9ea3f0, 0x9ea40c), // Sampdoria × Genoa
    (7, 3, 2002, 0x9ea42c, 0x9ea45c), // Cosenza × Messina
    (7, 3, 2002, 0x9ea41c, 0x9ea3fc), // Empoli × Modena
    (7, 3, 2002, 0x9ea464, 0x9ea3e4), // Ancona × Salernitana
    (7, 3, 2002, 0x9ea424, 0x9ea3dc), // Palermo × Siena
    (7, 3, 2002, 0x9ea3c8, 0x9ea3d4), // Pistoiese × Ternana
    (14, 3, 2002, 0x9ea3dc, 0x9ea404), // Siena × Bari
    (14, 3, 2002, 0x9ea3d4, 0x9ea450), // Ternana × Cagliari
    (14, 3, 2002, 0x9ea40c, 0x9ea42c), // Genoa × Cosenza
    (14, 3, 2002, 0x9ea3e4, 0x9ea41c), // Salernitana × Empoli
    (14, 3, 2002, 0x9ea448, 0x9ea414), // Crotone × Napoli
    (14, 3, 2002, 0x9ea3fc, 0x9ea424), // Modena × Palermo
    (14, 3, 2002, 0x9ea440, 0x9ea3c8), // Como × Pistoiese
    (14, 3, 2002, 0x9ea45c, 0x9ea46c), // Messina × Reggina
    (14, 3, 2002, 0x9ea434, 0x9ea3f0), // Cittadella × Sampdoria
    (14, 3, 2002, 0x9ea464, 0x9ea3c0), // Ancona × Vicenza
    (21, 3, 2002, 0x9ea42c, 0x9ea464), // Cosenza × Ancona
    (21, 3, 2002, 0x9ea404, 0x9ea434), // Bari × Cittadella
    (21, 3, 2002, 0x9ea424, 0x9ea440), // Palermo × Como
    (21, 3, 2002, 0x9ea3f0, 0x9ea448), // Sampdoria × Crotone
    (21, 3, 2002, 0x9ea3c0, 0x9ea40c), // Vicenza × Genoa
    (21, 3, 2002, 0x9ea414, 0x9ea45c), // Napoli × Messina
    (21, 3, 2002, 0x9ea46c, 0x9ea3fc), // Reggina × Modena
    (21, 3, 2002, 0x9ea450, 0x9ea3e4), // Cagliari × Salernitana
    (21, 3, 2002, 0x9ea3c8, 0x9ea3dc), // Pistoiese × Siena
    (21, 3, 2002, 0x9ea41c, 0x9ea3d4), // Empoli × Ternana
    (28, 3, 2002, 0x9ea440, 0x9ea404), // Como × Bari
    (28, 3, 2002, 0x9ea464, 0x9ea450), // Ancona × Cagliari
    (28, 3, 2002, 0x9ea3d4, 0x9ea434), // Ternana × Cittadella
    (28, 3, 2002, 0x9ea42c, 0x9ea448), // Cosenza × Crotone
    (28, 3, 2002, 0x9ea3dc, 0x9ea41c), // Siena × Empoli
    (28, 3, 2002, 0x9ea40c, 0x9ea424), // Genoa × Palermo
    (28, 3, 2002, 0x9ea45c, 0x9ea3c8), // Messina × Pistoiese
    (28, 3, 2002, 0x9ea414, 0x9ea46c), // Napoli × Reggina
    (28, 3, 2002, 0x9ea3c0, 0x9ea3e4), // Vicenza × Salernitana
    (28, 3, 2002, 0x9ea3fc, 0x9ea3f0), // Modena × Sampdoria
    (5, 4, 2002, 0x9ea3c8, 0x9ea464), // Pistoiese × Ancona
    (5, 4, 2002, 0x9ea450, 0x9ea440), // Cagliari × Como
    (5, 4, 2002, 0x9ea404, 0x9ea42c), // Bari × Cosenza
    (5, 4, 2002, 0x9ea448, 0x9ea40c), // Crotone × Genoa
    (5, 4, 2002, 0x9ea3f0, 0x9ea45c), // Sampdoria × Messina
    (5, 4, 2002, 0x9ea434, 0x9ea3fc), // Cittadella × Modena
    (5, 4, 2002, 0x9ea424, 0x9ea414), // Palermo × Napoli
    (5, 4, 2002, 0x9ea46c, 0x9ea3dc), // Reggina × Siena
    (5, 4, 2002, 0x9ea3e4, 0x9ea3d4), // Salernitana × Ternana
    (5, 4, 2002, 0x9ea41c, 0x9ea3c0), // Empoli × Vicenza
    (12, 4, 2002, 0x9ea464, 0x9ea404), // Ancona × Bari
    (12, 4, 2002, 0x9ea3c8, 0x9ea450), // Pistoiese × Cagliari
    (12, 4, 2002, 0x9ea45c, 0x9ea434), // Messina × Cittadella
    (12, 4, 2002, 0x9ea3dc, 0x9ea448), // Siena × Crotone
    (12, 4, 2002, 0x9ea440, 0x9ea41c), // Como × Empoli
    (12, 4, 2002, 0x9ea40c, 0x9ea3fc), // Genoa × Modena
    (12, 4, 2002, 0x9ea42c, 0x9ea414), // Cosenza × Napoli
    (12, 4, 2002, 0x9ea3c0, 0x9ea424), // Vicenza × Palermo
    (12, 4, 2002, 0x9ea3e4, 0x9ea46c), // Salernitana × Reggina
    (12, 4, 2002, 0x9ea3d4, 0x9ea3f0), // Ternana × Sampdoria
    (19, 4, 2002, 0x9ea3fc, 0x9ea464), // Modena × Ancona
    (19, 4, 2002, 0x9ea3f0, 0x9ea440), // Sampdoria × Como
    (19, 4, 2002, 0x9ea450, 0x9ea42c), // Cagliari × Cosenza
    (19, 4, 2002, 0x9ea41c, 0x9ea40c), // Empoli × Genoa
    (19, 4, 2002, 0x9ea404, 0x9ea45c), // Bari × Messina
    (19, 4, 2002, 0x9ea414, 0x9ea3c8), // Napoli × Pistoiese
    (19, 4, 2002, 0x9ea424, 0x9ea3e4), // Palermo × Salernitana
    (19, 4, 2002, 0x9ea434, 0x9ea3dc), // Cittadella × Siena
    (19, 4, 2002, 0x9ea448, 0x9ea3d4), // Crotone × Ternana
    (19, 4, 2002, 0x9ea46c, 0x9ea3c0), // Reggina × Vicenza
    (26, 4, 2002, 0x9ea3e4, 0x9ea404), // Salernitana × Bari
    (26, 4, 2002, 0x9ea40c, 0x9ea450), // Genoa × Cagliari
    (26, 4, 2002, 0x9ea440, 0x9ea434), // Como × Cittadella
    (26, 4, 2002, 0x9ea3c8, 0x9ea448), // Pistoiese × Crotone
    (26, 4, 2002, 0x9ea464, 0x9ea41c), // Ancona × Empoli
    (26, 4, 2002, 0x9ea45c, 0x9ea3fc), // Messina × Modena
    (26, 4, 2002, 0x9ea3dc, 0x9ea414), // Siena × Napoli
    (26, 4, 2002, 0x9ea42c, 0x9ea424), // Cosenza × Palermo
    (26, 4, 2002, 0x9ea3d4, 0x9ea46c), // Ternana × Reggina
    (26, 4, 2002, 0x9ea3c0, 0x9ea3f0), // Vicenza × Sampdoria
    (2, 5, 2002, 0x9ea424, 0x9ea464), // Palermo × Ancona
    (2, 5, 2002, 0x9ea414, 0x9ea440), // Napoli × Como
    (2, 5, 2002, 0x9ea41c, 0x9ea42c), // Empoli × Cosenza
    (2, 5, 2002, 0x9ea46c, 0x9ea40c), // Reggina × Genoa
    (2, 5, 2002, 0x9ea448, 0x9ea45c), // Crotone × Messina
    (2, 5, 2002, 0x9ea434, 0x9ea3c8), // Cittadella × Pistoiese
    (2, 5, 2002, 0x9ea3fc, 0x9ea3e4), // Modena × Salernitana
    (2, 5, 2002, 0x9ea3f0, 0x9ea3dc), // Sampdoria × Siena
    (2, 5, 2002, 0x9ea404, 0x9ea3d4), // Bari × Ternana
    (2, 5, 2002, 0x9ea450, 0x9ea3c0), // Cagliari × Vicenza
];

/// In the rehearsal (the comparer) the copy writes the rules and does not call virtual +0x3c (the round table, which
/// allocates).
pub(crate) static ITA_SER_A_RULES_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Writes the rule fields (offset, size, value) into the league and builds the main-stage round table via virtual +0x3c
/// into +0xba (skipped in dry run); returns 1.
pub(crate) unsafe fn ita_ser_a_apply_league_rules(this: League, written: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in written {
        match t {
            1 => *((this.addr() + d) as *mut u8) = v as u8,
            2 => write_u16(this.addr() + d, v as u16),
            _ => write_u32(this.addr() + d, v),
        }
    }
    if !ITA_SER_A_RULES_DRY_RUN.load(Relaxed) {
        let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
        let t = rounds(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
        this.set_round_dates(t);
    }
    1
}

/// The rules common to the four Italian divisions.
///
/// 1 point for a draw (`+0xc3`), category 2 for the matches (`+0xc4`), `+0xab` and `+0xdb` = 0 (the words copied into
/// the matches), national clubs (`+0x42` = 1), the 1st tie-breaker 1 (`+0xc5`) and the 2nd 2 (`+0xc6`), the fixed round
/// order in the grid (bit 0x40 of `+0xd9`, with the low byte left as is), 7 substitutes on the bench (`+0x49`) and 3
/// substitutions (`+0x4a`).
pub(crate) unsafe fn italian_common_rules(this: League) -> Vec<(usize, u8, u32)> {
    let d9 = (this.league_flags() as u8) | 0x40;
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xd9, 1, d9 as u32), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// Serie A (18 clubs): 2 rounds, 3 points for a win and 1 for a draw, nobody goes up, the bottom 4 go down to B; 7
/// substitutes and 3 substitutions; tie-breaks 1, 2 and 4 [codes; inference: goal difference, goals for, head to head];
/// a points tie across a place border goes to a tie-break match (+0xe6); fixed round order
/// Original: `ItaSerA::set_up_rules` (0x641640, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_set_up_rules(this: usize) -> u32 {
    covered!("cqi_6 serie a rules");
    ita_ser_a_apply_league_rules(League(this), &serie_a_rules(this))
}

/// Rule writes for Serie A (common rules plus rounds, win points, 4 relegated, lower division Serie B).
pub(crate) unsafe fn serie_a_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 4)); // cqi6: a Série A rebaixa 4
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B) as u32), (0xe6, 4, 1)]);
    v
}

/// Rule writes for Serie B (common rules plus rounds, promotion/relegation and links to Serie A and C1).
pub(crate) unsafe fn serie_b_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a Série B joga 2 turnos
    v.extend_from_slice(&[(0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_A) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A) as u32), (0x52, 1, 2), (0xe6, 4, 1)]);
    v
}

/// Rule writes for Serie C1/A.
pub(crate) unsafe fn serie_c1a_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: the C1/A gives 3 points per win
    v.extend_from_slice(&[(0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B) as u32)]);
    v
}

/// Rule writes for Serie C1/B (lower division C2/A only when that league is selected).
pub(crate) unsafe fn serie_c1b_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A) as u32)]);
    let nation = Nation(read_usize(read_usize(this + 4) + competition::NATION));
    let below = if nation.league_selected() & 4 != 0 { read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) as u32 } else { 0xffff_ffff };
    v.push((0x20, 4, below));
    v
}

/// Dry-run comparer: runs the original, restores +0x1c..+0xec, runs the port without the virtual call and compares
/// every written field and the return value; keeps the original's state.
pub(crate) unsafe fn ita_ser_a_compare_league_rules(name: &str, this: League, written: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let band = [(this.addr() + league::UPPER_DIVISION_ID, 0xd0)];
    let before = crate::comparison::MemorySnapshot::read(&band);
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&band);
    before.write_back();
    ITA_SER_A_RULES_DRY_RUN.store(true, Relaxed);
    let rr = crate::rng::pause_trace(copy);
    ITA_SER_A_RULES_DRY_RUN.store(false, Relaxed);
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

/// Shared table body: sets the club count, allocates the table and fills the rows with the division's clubs in database
/// order; returns whether it filled up.
///
/// +0x3e = `n` clubs; the table (malloc `exe::crt_malloc` of n × 0x3b; when out of memory: the error box of source file
/// `arquivo` at line `linha` and None); the clubs of [0xae23b4] (0x245 bytes, [0xae235c] of them) whose competition
/// (+0x57, the id at +0) is that of global `g`, in database order, each in the next row (League::init_table_row
/// `init_table_row`: thiscall(league, row, club, 0, 0), ret 0x10), until full.
///
/// Returns whether it filled up.
pub(crate) unsafe fn ita_ser_a_fill_league_table(this: League, n: i16, file: usize, row: u32, g: usize) -> Option<bool> {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return None;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            let row = k; // cqj4: row k receives the k-th club of the division
            // simplification: the original pushes the row in eax with the high bits of the competition id; 0x687430 only reads
            // o byte.
            exe::league_init_table_row(this, row as u32, club, 0, 0); // cqj4: the rows without a mark
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    Some(k as i8 as i16 == this.club_count())
}

/// Sets the prize fund (+0x28 = calc_prize_fund(cota)) and +0xe1 = 1 (paid by table at league end).
pub(crate) unsafe fn set_prize_fund(this: League, quota: i32) {
    let f = exe::league_calc_prize_fund(this, quota);
    this.set_prize_fund(f);
    this.set_prize_mode(1);
}

/// Creates the Serie A table: 18 clubs; the prize fund starts at 250,000 per place (paid by table position when the
/// league ends); TV pays 5,000,000 to each club. `ItaSerA::create_table()`: 18 clubs (`KEY_CLUB_COMP_ITALIAN_SERIE_A`);
/// once full, the fund by the 250,000 share and the TV money of 5,000,000 to each club (`exe::league_pay_tv_rights`:
/// thiscall(league, value, 0), ret 8); returns 1 (0 if it could not fill). Source `0x9ea2a8`, line `0x253`.
/// Original: `ItaSerA::create_table` (0x641510, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_create_table(this: usize) -> u32 {
    match ita_ser_a_fill_league_table(League(this), 0x12, 0x9ea2a8, 0x253, KEY_CLUB_COMP_ITALIAN_SERIE_A) {
        Some(true) => {}
        _ => return 0,
    }
    covered!("cqj_4 serie a fixtures");
    set_prize_fund(League(this), 0x3d090); // cqj4: the Serie A quota
    exe::league_pay_tv_rights(this, 0x4c4b40, 0); // cqj4: the Serie A TV money
    1
}

/// The comparer of the four table routines: the original routine allocates the table (and Serie A's also pays the TV
/// money, with news), so only it runs.
///
/// The plan, made beforehand: the division's clubs in database order (up to `n`). Afterwards it checks: `+0x3e` = `n`,
/// the return value = 1 if the plan was full, the rows holding the plan's clubs in order and, with `cota` and a full
/// table, `+0xe1` = 1 and `+0x28` = the fund for that share.
pub(crate) unsafe fn ita_ser_a_compare_league_table(name: &str, this: League, n: i16, g: usize, quota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let club = Club(read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE);
        let comp = club.division().addr();
        if comp != 0 && read_usize(comp) == read_usize(g) {
            plan.push(club.addr());
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let mut same = r == (plan.len() as i16 == n) as u32 && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
        if let (1, Some(c)) = (r, quota) {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, c);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// The Italian promotion and relegation chain: the final positions of Serie A, B, C1 A and C1 B; four up and four down
/// between Serie A and B, the swaps between Serie B and each C1 group; when Serie C2 is loaded, its three groups'
/// positions and the swaps between each C1 group and each C2 group.
///
/// The final positions (virtual `league_vtable::RECORD_FINAL_POSITIONS`, thiscall, ret 0) of the objects of
/// [`KEY_CLUB_COMP_ITALIAN_SERIE_A`], [`KEY_CLUB_COMP_ITALIAN_SERIE_B`], [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_A`] and
/// [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_B`] (`G_COMP_OBJECT_TABLE`, none checked for null); League::promote_and_relegate
/// (`exe::league_promote_and_relegate`, thiscall(this, upper, lower, a, b, c, d), ret 0x18) for A and B with (1, 1, 4,
/// 4), B and C1 A and B and C1 B with (1, 1, −1, −1). When the Serie C2 A object exists: the final positions of C2 A,
/// C2 B and C2 C, then C1 A with C2 A (1, 1, −1, −1), C1 B with C2 A (0, 1, −1, −1), C1 A with C2 B (1, 1), C1 B with
/// C2 B (0, 1), C1 A with C2 C (1, 1) and C1 B with C2 C (0, 1); the last one's eax is returned. Without C2 A it returns
/// the competition object table's address (the eax the original leaves; the caller does not read it).
/// Original: `ItaSerA::process_promotion_relegation_chain` (0x642410, ita_ser_a.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (a, b, c1a, c1b) = (KEY_CLUB_COMP_ITALIAN_SERIE_A, KEY_CLUB_COMP_ITALIAN_SERIE_B, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B);
    covered!("mwf_16 italian chain");
    for k in [a, b, c1a, c1b] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), league_vtable::RECORD_FINAL_POSITIONS);
    }
    exe::league_promote_and_relegate(this, comp_by_global(a), comp_by_global(b), 1, 1, 4, 4); // mwf16: four up and four down between A and B
    exe::league_promote_and_relegate(this, comp_by_global(b), comp_by_global(c1a), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(b), comp_by_global(c1b), 1, 1, -1, -1);
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    covered!("mwf_16 serie c2 chain");
    let (c2a, c2b, c2c) = (KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C);
    for k in [c2a, c2b, c2c] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), league_vtable::RECORD_FINAL_POSITIONS);
    }
    let mut r = 0;
    for lower in [c2a, c2b, c2c] {
        exe::league_promote_and_relegate(this, comp_by_global(c1a), comp_by_global(lower), 1, 1, -1, -1);
        r = exe::league_promote_and_relegate(this, comp_by_global(c1b), comp_by_global(lower), 0, 1, -1, -1);
    }
    r
}

/// Comparer for `ita_ser_a_process_promotion_relegation_chain`: only the original (it moves the clubs between the
/// divisions, as the Brazilian chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_ITA_SER_A_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
}

/// Without Serie C2: the six best Serie C2 clubs go up to Serie C1 A and the clubs relegated from either C1 group go
/// down to Serie C2 A (the C1 regrouping that follows rebalances the two groups).
///
/// C1 A and C1 B are the objects of [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_A`] and [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_B`]. A
/// list of `G_CLUB_COUNT` × 4 bytes (`exe::crt_malloc`; out of memory, the box of ita_ser_a.cpp line 0x419 and it
/// returns) takes, in club table order, every club whose division has the id of C2 A, C2 B or C2 C (the count is a
/// word). With fewer than six, nothing moves. Otherwise the list is sorted by reputation and shuffled in sixes
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 6), cdecl) and its first six go up to C1 A
/// (League::move_club_to_division `exe::league_move_club_to_division`, thiscall(this, club, the C1 A record, 1), ret
/// 0xc). Then for each row i below C1 A's `.club_count()` (read once; C1 B is assumed the same size): a C1 A row with
/// `league_table_row::STATUS` 3 and then a C1 B row with status 3 send their clubs down to C2 A
/// (League::relegate_club_to_division 0x6831a0, thiscall(this, club, the C2 A record, 1), ret 0xc). The list is freed.
/// Original: `ItaSerA::exchange_serie_c2_clubs_unplayed` (0x641fc0, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_exchange_serie_c2_clubs_unplayed(this: usize) {
    let this = League(this);
    let (c1a, c1b) = (League(comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A)), League(comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B)));
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9ea2a8, 0x419);
        return;
    }
    let c2 = [KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C];
    let mut n = 0i16;
    let mut k = 0i32;
    while k < read_i32(G_CLUB_COUNT) {
        let c = Club(read_usize(G_CLUB_TABLE) + k as usize * club::SIZE);
        let d = c.division().addr();
        if d != 0 && c2.iter().any(|&g| read_u32(d) == read_u32(g)) {
            write_usize(list + n as usize * 4, c.addr());
            n = n.wrapping_add(1);
        }
        k += 1;
    }
    if n >= 6 {
        covered!("mwf_16 serie c2 up to c1");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 6); // mwf16: the C2 clubs by reputation, shuffled in sixes
        for t in 0..6 {
            exe::league_move_club_to_division(this, read_usize(list + t * 4), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), 1);
        }
        let rows = c1a.club_count() as i32;
        for i in 0..rows.max(0) {
            for group in [c1a, c1b] {
                if read_u8(exe::league_get_table_row(group, i) + league_table_row::STATUS) == 3 {
                    exe::league_relegate_club_to_division(this, read_usize(exe::league_get_table_row(group, i)), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), 1);
                }
            }
        }
    }
    exe::crt_free(list);
}

/// Comparer for `ita_ser_a_exchange_serie_c2_clubs_unplayed`: only the original runs (it draws and moves clubs);
/// afterwards every club in a C1 A or C1 B row marked relegated (status 3) must be in Serie C2 A.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_exchange_serie_c2_clubs_unplayed(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_A_EXCHANGE_SERIE_C2_CLUBS_UNPLAYED)(this);
    let c2a = comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A);
    let mut wrong = Vec::new();
    for key in [KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B] {
        let g = League(comp_by_global(key));
        for i in 0..g.club_count().max(0) as i32 {
            let row = exe::league_get_table_row(g, i);
            if read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != c2a {
                wrong.push(read_usize(row));
            }
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("ita_ser_a_exchange_serie_c2_clubs_unplayed({:#x}) relegated clubs not in C2 A: {:x?}", this, wrong));
}

/// The Serie C1 regrouping: the 36 clubs of the two C1 groups sorted from north to south (by their stadium's city) and
/// split, the first 18 into C1 B and the other 18 into C1 A; the human manager of a club that changes group gets the
/// news.
///
/// In club table order (the index compared as a word with `G_CLUB_COUNT`, read once), the clubs whose division is the
/// C1 A or the C1 B record (`comp_record_of_key`) enter a local list of 36 (the count is a signed byte; it stops at
/// 36). The list is sorted (`exe::crt_qsort`(list, n, 4, 0x53fd40), the latitude comparer). Then the 36 entries (always
/// 36: with fewer clubs the original reads stack garbage, a defect [inference]): entry k < 18 goes to C1 B, the others
/// to C1 A; a club that was not already there and whose manager is human (`exe::news_club_has_human_manager`, thiscall
/// (`G_NEWS`, club)) gets the news item 0x13 (NewsItem::NewsItem `exe::new_newsitem`(item, 0x13, 0); slot 0 the club's
/// id, slot 1 the group's id; News::send_to_club_manager `exe::news_send_to_club_manager`(`G_NEWS`, item, club)); every
/// club's `club::DIVISION` becomes its group's record.
/// Original: `ita_ser_a_regroup_serie_c1_clubs` (0x642630, ita_ser_a.cpp, called with ecx = the Serie A object and no
/// arguments, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_regroup_serie_c1_clubs(_this: usize) {
    let (a, b) = (comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let count = read_i32(G_CLUB_COUNT);
    let mut list = [0usize; 36];
    let mut n = 0i8;
    let mut k = 0i16;
    if count > 0 {
        loop {
            let c = read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE;
            let d = read_usize(c + club::DIVISION);
            if d == a || d == b {
                list[n as usize] = c;
                n += 1;
                if n == 36 {
                    break;
                }
            }
            k = k.wrapping_add(1);
            if k as i32 >= count {
                break;
            }
        }
    }
    covered!("mwf_16 serie c1 regrouping");
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x53fd40usize); // mwf16: north to south
    for (k, &c) in list.iter().enumerate() {
        if c == 0 {
            continue; // simplification: an empty entry (fewer than 36 clubs) is skipped; the original reads stack garbage
        }
        let group = if k < 18 { b } else { a };
        if read_usize(c + club::DIVISION) != group && exe::news_club_has_human_manager(read_usize(G_NEWS), c) != 0 {
            let mut news = [0u32; 0x38];
            let nt = news.as_mut_ptr() as usize;
            exe::new_newsitem(nt, 0x13, 0);
            exe::newsitem_set_slot(nt, 0, read_u32(c));
            exe::newsitem_set_slot(nt, 1, read_u32(group));
            exe::news_send_to_club_manager(read_usize(G_NEWS), nt, c);
        }
        write_usize(c + club::DIVISION, group);
    }
}

/// Comparer for `ita_ser_a_regroup_serie_c1_clubs`: only the original runs (it sends news); the plan, made before from
/// the same sort over a copy of the list, is the group of each of the 36 clubs.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_regroup_serie_c1_clubs(this: usize) {
    let (a, b) = (comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let mut list: Vec<usize> = (0..read_i32(G_CLUB_COUNT).max(0) as usize)
        .map(|k| read_usize(G_CLUB_TABLE) + k * club::SIZE)
        .filter(|&c| { let d = read_usize(c + club::DIVISION); d == a || d == b })
        .take(36)
        .collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_A_REGROUP_SERIE_C1_CLUBS)(this);
    if list.len() != 36 {
        return; // simplification: with fewer than 36 the original reads stack garbage; nothing to check
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, 36, 4, 0x53fd40usize);
    let wrong: Vec<usize> = list.iter().enumerate().filter(|&(k, &c)| read_usize(c + club::DIVISION) != if k < 18 { b } else { a }).map(|(_, &c)| c).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("ita_ser_a_regroup_serie_c1_clubs: clubs in the wrong group {:x?}", wrong));
}

/// The Italian season turnover, led by Serie A: promotion and relegation, the exchange with the level below (Serie D
/// and the C2 regrouping when Serie C2 is loaded, otherwise the C2 exchange), the C1 regrouping, then Serie A rebuilt
/// (next year, rules, 18 clubs, fixtures, order); the division tier capped in the lowest loaded groups; Serie B, C1 A,
/// C1 B (and C2 A, B and C) turned over; then the predicted positions, the board expectations and the referee renewal.
///
/// `league::SEASON_CLOSED` = 0; the chain (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`, thiscall(this,
/// 1), ret 4); with the Serie C2 A object ([`KEY_CLUB_COMP_ITALIAN_SERIE_C2_A`] in `G_COMP_OBJECT_TABLE`),
/// ItaSerA::exchange_serie_d_clubs (0x6421c0) and ita_ser_a_regroup_serie_c2_clubs (0x642790), otherwise
/// ItaSerA::exchange_serie_c2_clubs_unplayed (0x641fc0); then ita_ser_a_regroup_serie_c1_clubs (0x642630); all called
/// with ecx = this and no arguments. The table freed (League::free_table `exe::league_free_table`, thiscall(this, 0),
/// ret 4); `.round_dates()` freed and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the stage array freed when there is one
/// (without destroying stages) and zeroed, `league::STAGE_COUNT` = 0; the year up by one; `league::CURRENT_STAGE` = −1;
/// and `league::SEASON_CLOSED` = 1 (as in Serie B; the sisters leave 0).
///
/// Then, each with its own box (ita_ser_a.cpp, lines 0x2da, 0x2e1, 0x2e7 and 0x2ee) when it returns 0: the rules
/// (virtual `comp_vtable::SET_UP_RULES`), the exe::itasera_create_table(ItaSerA::create_table 0x641510), the fixtures
/// (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table `exe::league_sort_table`,
/// thiscall(this, 0), ret 4). Then League::limit_club_division_tier (`exe::league_limit_club_division_tier`, thiscall(
/// group, 1, 3, −3, 1), ret 0x10) on C2 A, C2 B and C2 C when C2 A exists, otherwise on C1 A and C1 B; the turnovers
/// (virtual `comp_vtable::START_NEW_SEASON`) of Serie B, C1 A and C1 B (none checked for null) and, when C2 A exists,
/// of C2 A, C2 B and C2 C; the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and
/// `set_expectations_and_renew_officials`, whose eax (the referee renewal's) it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerA::start_new_season` (0x641700, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    };
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1); // mwf17: the Italian promotion and relegation
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0 {
        call(0x6421c0);
        call(0x642790);
    } else {
        call(0x641fc0);
    }
    call(0x642630); // the Serie C1 regrouping
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if this.stages() != 0 {
        exe::crt_free(this.stages());
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf17: the year of Serie A
    this.set_stages(0);
    write_u32(this.addr() + league::STAGE_COUNT, 0);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_season_closed(1);
    covered!("mwf_17 serie a rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x9ea2a8, 0x2da);
        return 0;
    }
    if exe::itasera_create_table(this) == 0 {
        show_error_at(0x9ea2a8, 0x2e1);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9ea2a8, 0x2e7);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x9ea2a8, 0x2ee);
        return 0;
    }
    let c2 = comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0;
    let capped: &[usize] = if c2 {
        &[KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C]
    } else {
        &[KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B]
    };
    for &k in capped {
        exe::league_limit_club_division_tier(comp_by_global(k), 1, 3, -3, 1);
    }
    for k in [KEY_CLUB_COMP_ITALIAN_SERIE_B, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), comp_vtable::START_NEW_SEASON);
    }
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0 {
        for k in [KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C] {
            bra_nat_first_call_virtual_no_args(comp_by_global(k), comp_vtable::START_NEW_SEASON);
        }
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_a_start_new_season`: only the original runs (it moves clubs and rebuilds the Italian
/// leagues); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_START_NEW_SEASON);
    compare_state_league_new_season("ita_ser_a_start_new_season", League(this), || o(this))
}
