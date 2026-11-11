use super::*;

/// Describes one round of the 0x41-byte table.
///
/// It holds the date (day, month 0..11, years to add to the league year +0x40, the weekday the date is moved to), the
/// code of byte +4 ([inference] the half of the season) and the TV slots in order (weekday, half, how many matches; the
/// list ends with −1).
pub(crate) type FraCfaScheduleRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets rounds `primeira`.. of round table `t` from a static list, in the original's order.
///
/// For each one, the date (`league_round_set_date`: push 0; push year; push code; push week; push years; push month; push day; push i;
/// push t; add esp -> cdecl: value 0) and the match ranges (`store_round_match`: push 0; push c; push b; push a; push j; push i;
/// push t; add esp -> cdecl, value 0).
pub(crate) unsafe fn fra_cfa_set_league_rounds(t: usize, year: u32, first: i32, table: &[FraCfaScheduleRound]) {
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in table.iter().enumerate() {
        let i = first + k as i32;
        exe::league_round_set_date(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, year, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            exe::league_round_set_match_slot(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// O que muda entre as ligas de fase única.
pub(crate) struct SinglePhaseLeague {
    pub(crate) rounds: i16,
    pub(crate) file: usize,
    pub(crate) row: u32,
    pub(crate) table: &'static [FraCfaScheduleRound],
    pub(crate) branch: &'static str,
}

/// The common body of the French CFA schedule (thiscall(league, stage, &rodadas, &extra, &é_liga), ret 0x10; no saved
/// state): returns the round table.
///
/// Only the main stage (−1) builds it; the others return 0 without writing anything. é_liga (if requested) = 1, the
/// rounds, extra = 0, a table of rounds × 0x41 bytes (malloc `exe::crt_malloc`: push; add esp, 4 -> cdecl; out of memory: the
/// error box and 0) with the league's dates and slots.
pub(crate) unsafe fn create_cfa_schedule(this: League, stage: u32, rodadas: *mut i16, extra: *mut i16, league: *mut i32, p: &SinglePhaseLeague) -> usize {
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
    fra_cfa_set_league_rounds(t, ano, 0, p.table);
    t
}

/// Side-by-side comparer of the CFA schedule: outputs and written table bytes must match; the copy's table is freed.
pub(crate) unsafe fn compare_cfa_schedule(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32,
                       original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, original, || copy(&mut r2, &mut e2, pl2));
    let outs = stage as u8 != 0xff || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        let n = *rounds as usize;
        written_league_round_bytes(a, a, n) == written_league_round_bytes(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(stage {}): outs {} table {}", name, stage as u8 as i8, outs, table));
    a
}

/// Side-by-side comparer of the schedule over all stages, starting from the same [`0xb67a34`].
///
/// The original routine runs with the caller's outputs, the copy with its own outputs; the outputs and the written
/// bytes of both tables must be equal (at stage −1 the league rounds, `written_league_round_bytes`; at the others the
/// cup rounds, `eng_cc_cup_written_round_bytes`); the copy's table is freed (`0x9452ca`, cdecl).
pub(crate) unsafe fn compare_c1_schedule(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32,
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

/// a CFA (de fundo, 4 grupos de 18), 34 rodadas de 31/jul a 20/mai; quase todas com duas faixas de TV
pub(crate) const CFA_ROUNDS: [FraCfaScheduleRound; 34] = [
    (31, 6, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 0: 31/jul
    (6, 7, 0, 4, 2, &[(-1, -1, -1)]), // 1: 6/ago
    (14, 7, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 2: 14/ago
    (20, 7, 0, 4, 2, &[(-1, -1, -1)]), // 3: 20/ago
    (28, 7, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 4: 28/ago
    (10, 8, 0, 4, 2, &[(-1, -1, -1)]), // 5: 10/set
    (18, 8, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 6: 18/set
    (25, 8, 0, 4, 2, &[(-1, -1, -1)]), // 7: 25/set
    (2, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 8: 2/out
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 9: 12/out
    (15, 9, 0, 4, 2, &[(-1, -1, -1)]), // 10: 15/out
    (23, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 11: 23/out
    (30, 9, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 12: 30/out
    (6, 10, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 13: 6/nov
    (10, 10, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 14: 10/nov
    (19, 10, 0, 4, 2, &[(-1, -1, -1)]), // 15: 19/nov
    (27, 10, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 16: 27/nov
    (3, 11, 0, 4, 2, &[(-1, -1, -1)]), // 17: 3/dez
    (11, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 18: 11/dez
    (15, 11, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 19: 15/dez
    (18, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 20: 18/dez
    (15, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 21: 15/jan (+1 ano)
    (26, 0, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 22: 26/jan (+1 ano)
    (2, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 23: 2/fev (+1 ano)
    (5, 1, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 24: 5/fev (+1 ano)
    (16, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 25: 16/fev (+1 ano)
    (27, 1, 1, 4, 2, &[(-1, -1, -1)]), // 26: 27/fev (+1 ano)
    (11, 2, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 27: 11/mar (+1 ano)
    (25, 2, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 28: 25/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 29: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 30: 15/abr (+1 ano)
    (29, 3, 1, 5, 2, &[(6, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 31: 29/abr (+1 ano)
    (13, 4, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 32: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(4, 2, 1), (4, 1, 1), (-1, -1, -1)]), // 33: 20/mai (+1 ano)
];

/// a Série C1 (as duas chaves, a mesma tabela), 34 rodadas de domingo de 5/set a 7/mai, sem faixas de TV
pub(crate) const SERIE_C1_ROUNDS: [FraCfaScheduleRound; 34] = [
    (5, 8, 0, 6, 1, &[(-1, -1, -1)]), // 0: 5/set
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 1: 12/set
    (19, 8, 0, 6, 1, &[(-1, -1, -1)]), // 2: 19/set
    (26, 8, 0, 6, 1, &[(-1, -1, -1)]), // 3: 26/set
    (3, 9, 0, 6, 1, &[(-1, -1, -1)]), // 4: 3/out
    (10, 9, 0, 6, 1, &[(-1, -1, -1)]), // 5: 10/out
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 6: 17/out
    (24, 9, 0, 6, 1, &[(-1, -1, -1)]), // 7: 24/out
    (31, 9, 0, 6, 1, &[(-1, -1, -1)]), // 8: 31/out
    (7, 10, 0, 6, 1, &[(-1, -1, -1)]), // 9: 7/nov
    (14, 10, 0, 6, 1, &[(-1, -1, -1)]), // 10: 14/nov
    (21, 10, 0, 6, 1, &[(-1, -1, -1)]), // 11: 21/nov
    (28, 10, 0, 6, 1, &[(-1, -1, -1)]), // 12: 28/nov
    (5, 11, 0, 6, 1, &[(-1, -1, -1)]), // 13: 5/dez
    (12, 11, 0, 6, 1, &[(-1, -1, -1)]), // 14: 12/dez
    (19, 11, 0, 6, 1, &[(-1, -1, -1)]), // 15: 19/dez
    (6, 0, 1, 3, 2, &[(-1, -1, -1)]), // 16: 6/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(-1, -1, -1)]), // 17: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(-1, -1, -1)]), // 18: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(-1, -1, -1)]), // 19: 30/jan (+1 ano)
    (13, 1, 1, 6, 1, &[(-1, -1, -1)]), // 20: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(-1, -1, -1)]), // 21: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(-1, -1, -1)]), // 22: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(-1, -1, -1)]), // 23: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(-1, -1, -1)]), // 24: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(-1, -1, -1)]), // 25: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(-1, -1, -1)]), // 27: 2/abr (+1 ano)
    (5, 3, 1, 2, 2, &[(-1, -1, -1)]), // 28: 5/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(-1, -1, -1)]), // 29: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 32: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 7/mai (+1 ano)
];

pub(crate) const CFA_LEAGUE: SinglePhaseLeague = SinglePhaseLeague { rounds: 0x22, file: 0x9ccaf0, row: 0xab, table: &CFA_ROUNDS, branch: "cqo_5 calendario da cfa" };

/// the CFA schedule (the French fourth level, background, 4 groups of 18): 34 rounds, from 31 July to 20 May of the
/// next year; in 25 rounds two matches go to TV slots (mostly day 6 or 4, slots 1 and 2); main stage only
/// Original: `FraCfa::create_schedule` (0x5b9510, fra_cfa.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fra_cfa_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_cfa_schedule(League(this), stage, rounds, extra, league, &CFA_LEAGUE)
}

/// The cup stages of C1 (0x68-byte rounds, `eng_cc_cup_write_cup_round`; category 5, no prize money, the year +0x40
/// signed).
///
/// Stage 0, the promotion play-offs: the semi-finals (draw on 8 May, first leg on 21 May, two legs with 7 days between
/// the matches, 4 teams in 2 ties, all new; 0xa00 for the single match = no draw (0x200) plus 0x800, 0x802 for the
/// two-match tie) and the final (draw on 29 May, first leg on 4 June, 2 teams).
///
/// Stage 1, the play-outs [inference: relegation]: one round of 2 two-legged ties on the semi-final dates, no draw
/// (0x200), name 0xbe.
pub(crate) const CFA_SEMIFINAL_ROUND: EngCcCupCupRound = cql10r((8, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0x82, (0, 0xa00, 0x802), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0)); // cqo5: the C1 semifinals
pub(crate) const CFA_FINAL_ROUND: EngCcCupCupRound = cql10r((0x1d, 4, 1, 0), (4, 5, 1, 6, 1, 0), 0x96, (0, 0x800, 0x802), 5, (2, 1, 0, 0), (0, 2, 7), (0, 0, 0));
pub(crate) const C1_PLAYOUT: EngCcCupCupRound = cql10r((8, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0xbe, (0, 0x200, 0x800), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0));

/// O que muda entre as duas chaves da C1: a fonte e as linhas das caixas (fase −1, 0 e 1).
pub(crate) struct C1BracketVariant {
    pub(crate) file: usize,
    pub(crate) rows: [u32; 3],
}

/// The common create_schedule body of French C1 A and C1 B (thiscall(league, stage, &rounds, &extra, &é_liga), ret
/// 0x10; no saved state), by stage (the byte); returns the table or 0.
///
/// −1 the main stage (é_liga = 1, 34 rounds, extra 0, the table of 0x8a2 bytes with `SERIE_C1_ROUNDS`, the year +0x40
/// as a word); 0 the play-offs (é_liga = 0, 2 rounds, extra 0xa0, the table of 0xd0 bytes with the semi-finals and the
/// final); 1 the play-outs (é_liga = 0, 1 round, extra 0, 0x68 bytes); the others return 0 without writing anything.
///
/// Out of memory (malloc `exe::crt_malloc`: push; add esp, 4 -> cdecl): the box for the stage's line and 0.
pub(crate) unsafe fn create_c1_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, p: &C1BracketVariant) -> usize {
    let (k, l, n, x, tamanho) = match stage as u8 {
        0xff => (0, 1, 0x22, 0, 0x8a2),
        0 => (1, 0, 2, 0xa0, 0xd0),
        1 => (2, 0, 1, 0, 0x68),
        _ => return 0,
    };
    if !league.is_null() {
        *league = l;
    }
    *rounds = n;
    *extra = x;
    let t = exe::crt_malloc(tamanho);
    if t == 0 {
        show_error_at(p.file, p.rows[k]);
        return 0;
    }
    let year = this.year();
    if k == 0 {
        covered!("cqo_5 c1 calendar");
        fra_cfa_set_league_rounds(t, year as u16 as u32, 0, &SERIE_C1_ROUNDS);
    } else {
        covered!("cqo_5 c1 play-offs");
        let year = year as i32 as u32;
        if k == 1 {
            eng_cc_cup_write_cup_round(t, 0, &CFA_SEMIFINAL_ROUND, year);
            eng_cc_cup_write_cup_round(t, 1, &CFA_FINAL_ROUND, year);
        } else {
            eng_cc_cup_write_cup_round(t, 0, &C1_PLAYOUT, year);
        }
    }
    t
}

/// the rules of the two French background leagues: home and away, 3 points a win and 1 a draw, tie-breaks 4, 1, 2
/// [inference: position, goal difference, goals], no promotion, play-off or relegation places, rule bits 0x43 (a
/// champion, top places as promotion and fixed round order); the league above gets the id at
/// `KEY_CLUB_COMP_SPANISH_FIRST_DIVISION` (the same global the Spanish Segunda uses for the Primera) [inference: an
/// original slip, harmless since there is no promotion]
/// Original: `FraCfa::set_up_rules` (0x5ba0a0, fra_cfa.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_cfa_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    this.set_legs((2) as i16);
    this.set_match_category(match_category::LEAGUE);
    write_u8(this.addr() + league::TIEBREAKS + 2, 2);
    this.set_win_points(3); // cqn4: the points per win in the French lower leagues
    this.set_draw_points(1);
    this.set_club_sides(1);
    write_u8(this.addr() + league::TIEBREAKS, 4);
    write_u8(this.addr() + league::TIEBREAKS + 1, 1);
    this.set_promoted_count(0);
    this.set_promotion_playoff_count(0);
    this.set_relegation_playoff_count(0);
    this.set_relegated_count(0);
    // simplification: the original writes to +0x1c the id of [0x9cf624] (the same global the Spanish Segunda uses for the
    // Primera); fiel.
    this.set_upper_division_id(read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) as i32);
    this.set_lower_division_id(0xffff_ffff);
    this.set_league_flags((0x43) as i16); // cqn4: the rule bits of the lower leagues
    let calendario: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let g = calendario(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    this.set_round_dates(g);
    1
}

/// O que muda entre as duas ligas de fundo.
pub(crate) struct BackgroundLeague {
    pub(crate) vtabela: usize,
    /// o id do registro da competição (+4)
    pub(crate) competition: usize,
    /// The ids of the competitions whose clubs all enter; the others come from the generic division ([0x9cf764]).
    pub(crate) all: &'static [usize],
    /// Only the clubs of the nation with id [g] (the lower division: France) among those of the generic one, and with
    /// no reserve competition (`+0x60` = 0).
    ///
    /// The CFA, on the contrary, requires the reserve competition to be itself ([[`+0x60`]] = [competicao]).
    pub(crate) nation: Option<usize>,
    /// o máximo da lista (72 a CFA, 40 a de baixo) e o máximo dos da genérica (34 e 20)
    pub(crate) maximo: i16,
    pub(crate) genericos: i16,
    /// os grupos e a tabela de cada um (thiscall(liga, lista), ret 4: `fra_cfa_create_group_table` e `fra_lower_create_table`)
    pub(crate) groups: usize,
    pub(crate) by_group: usize,
    pub(crate) table: usize,
    pub(crate) file: usize,
    /// as caixas: regras, tabela do grupo, jogos
    pub(crate) rows: [u32; 3],
}

/// The body of the two French lower-league constructors (thiscall(this, year), ret 4, SEH frame only on the exception
/// path; no saved state).
///
/// The base (League::League `exe::new_league`: mov ecx -> thiscall, ret 0), the vtable, the year (+0x40), +0x2c = 0, no
/// discipline (+0x50 = 0xff), the record (+4), stage 0xff (+0x44) and −1 (+0x30); the rules (`fra_cfa_set_up_rules`: mov ecx ->
/// thiscall, ret 0; on failure, the error box and stop).
///
/// Then the club list, in database order (the first [0xae235c] − 2 × [0xae2350]), up to the maximum: those of the
/// `todas` competitions; and, up to the generic maximum, those of the generic division meeting the league's condition.
/// The list is shuffled (comp_util_shuffle_pointer_array `comp_util_shuffle_pointer_array`: push count; push list; add esp, 8 -> cdecl; uses the
/// random draw) and each group, with its clubs consecutive in the list, gets the table (on failure, the error box), the
/// fixtures (League::create_fixtures `create_fixtures`: mov ecx -> thiscall, ret 0; on failure, the error box) and the table is
/// released (League::free_table `free_table`: push 0; mov ecx -> thiscall, ret 4): the group's matches stay in the game
/// calendar and the league carries on without a table.
///
/// The list is a local record of 72 or 40 pointers; the copy uses a zeroed array (the career fills the list; the
/// original would read stack garbage without enough clubs).
pub(crate) unsafe fn construct_group_league(this: League, year: u32, p: &BackgroundLeague) -> usize {
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(p.table);
    exe::new_league(this);
    write_usize(this.addr(), p.vtabela);
    this.set_year((year as u16) as i16);
    this.set_stage_count(0);
    this.set_discipline_rule(discipline_rule::NONE);
    write_usize(this.addr() + league::COMPETITION, comp_record_of_key(p.competition));
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    if exe::fracfa_set_up_rules(this) == 0 {
        show_error_at(p.file, p.rows[0]);
        return this.addr();
    }
    covered!("cqn_4 french bottom league");
    let mut list = vec![0usize; p.maximo as usize];
    let mut k: i16 = 0;
    let mut genericos: i16 = 0;
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let base_clubs = read_usize(G_CLUB_TABLE);
    let mut i: i32 = 0;
    while i < n {
        let c = Club(base_clubs + (i as usize) * club::SIZE);
        let comp = c.division().addr();
        let id = if comp != 0 { read_usize(comp) } else { 0 };
        if comp != 0 && p.all.iter().any(|&g| id == read_usize(g)) {
            list[k as usize] = c.addr();
            k += 1;
            if k == p.maximo {
                break;
            }
        } else {
            let enters = match p.nation {
                Some(g) => {
                    let nacao = c.nation().addr();
                    nacao != 0 && read_usize(nacao) == read_usize(g) && c.reserve_division().addr() == 0 && comp != 0 && id == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)
                }
                None => {
                    let reserva = c.reserve_division().addr();
                    comp != 0 && id == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && reserva != 0 && read_usize(reserva) == read_usize(p.competition)
                }
            };
            if enters && genericos < p.genericos { // cqn4: the maximum clubs of the generic division
                list[k as usize] = c.addr();
                k += 1;
                genericos += 1;
                if k == p.maximo {
                    break;
                }
            }
        }
        i += 1;
    }
    exe::comp_util_shuffle_pointer_array(list.as_mut_ptr() as usize, k as u16 as u32); // cqn4: the list random draw
    for g in 0..p.groups {
        if tabela(this.addr(), list.as_ptr() as usize + g * p.by_group * 4) == 0 {
            show_error_at(p.file, p.rows[1]);
            return this.addr();
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(p.file, p.rows[2]);
            return this.addr();
        }
        exe::league_free_table(this, 0);
    }
    this.addr()
}

pub(crate) const FRA_CFA_CFA_LEAGUE: BackgroundLeague = BackgroundLeague {
    vtabela: 0x96a7b0,
    competition: KEY_CLUB_COMP_FRENCH_CFA,
    all: &[KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION], // cqn4: the first and second division clubs in the CFA
    nation: None,
    maximo: 0x48,
    genericos: 0x22,
    groups: 4,
    by_group: 0x12,
    table: 0x5b9fa0,
    file: 0x9ccaf0,
    rows: [0x40, 0x64, 0x6b],
};
pub(crate) const FRENCH_LOWER_DIVISION: BackgroundLeague = BackgroundLeague {
    vtabela: 0x96ab78,
    competition: KEY_CLUB_COMP_FRENCH_LOWER_DIVISION,
    all: &[KEY_CLUB_COMP_FRENCH_NATIONAL],
    nation: Some(KEY_NATION_FRANCE),
    maximo: 0x28,
    genericos: 0x14,
    groups: 2,
    by_group: 0x14,
    table: 0x5be230,
    file: 0x9ccbd0,
    rows: [0x40, 0x62, 0x69],
};

/// the CFA (the French fourth level, background): up to 72 clubs, all of the first and second divisions [inference: the
/// reserve sides] and up to 34 of the generic division whose reserve competition is the CFA, drawn into 4 groups of 18;
/// each group gets the table and the fixtures (in the game calendar) and the table is freed
/// Original: `FraCfa::FraCfa` (0x5b91c0, fra_cfa.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_fra_cfa(this: usize, year: u32) -> usize {
    construct_group_league(League(this), year, &FRA_CFA_CFA_LEAGUE)
}

/// Comparer of the constructors (they draw and allocate): original only, checking return, vtable, year, discipline,
/// record and no table left (+0x3e = 0, +0xb1 = 0).
pub(crate) unsafe fn compare_construct_group_league(name: &str, this: League, year: u32, p: &BackgroundLeague, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let ok = r == this.addr() && read_usize(this.addr()) == p.vtabela && (this.year() as u16) == year as u16
        && this.discipline_rule() == discipline_rule::NONE && this.competition().addr() == comp_record_of_key(p.competition)
        && this.table() == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +4 = {:#x}, +0xb1 = {:#x}", name, this.addr(), r, read_usize(this.addr()), this.competition().addr(), this.table()));
    r
}

/// The body of the two destructors (thiscall, ret 0, SEH frame only on the exception path; identical apart from the
/// vtable): frees the table, schedule and stages, then calls League::~League.
///
/// Sets the class vtable, frees the table (League::free_table `0x687970`: push 0 -> thiscall, ret 4), frees
/// (`0x9452ca`: push; add esp, 4 -> cdecl) and zeroes the schedule [`+0xba`], destroys each stage from 0 to `+0x30`
/// (with `+0x30` ≥ 0) via its virtual 0 with 1 (the freeing destructor: push 1; mov ecx, stage -> thiscall, ret 4),
/// frees and zeroes the vector [`+0xc`] and, finally, League::~League (`0x682300`: mov ecx -> thiscall, ret 0).
pub(crate) unsafe fn destroy_group_league(this: League, vtabela: usize) {
    write_usize(this.addr(), vtabela);
    exe::league_free_table(this, 0);
    let g = this.round_dates();
    if g != 0 {
        exe::crt_free(g);
        this.set_round_dates(0);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i: i32 = 0;
        loop {
            let f = read_usize(this.stages() + i as usize * 4);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    let v = this.stages();
    if v != 0 {
        exe::crt_free(v);
        this.set_stages(0);
    }
    exe::drop_league(this);
}

/// frees the table, the schedule, the stages and their vector and destroys the league
/// Original: `FraCfa::~FraCfa` (0x5b9460, fra_cfa.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn destroy_fra_cfa(this: usize) {
    covered!("cqn_4 cfa destructor");
    destroy_group_league(League(this), 0x96a7b0)
}

/// Shared body of the two French table creations: `n` clubs (+0x3e), allocates the table and fills the rows from the
/// list `clubes`, marking clubs whose competition is in `marcadas`; returns whether it filled up.
///
/// +0x3e = `n`, the table (malloc `exe::crt_malloc` of n × 0x3b; when out of memory: the error box of source file `arquivo` at
/// line `linha` and 0) and the `n` clubs of the list `clubes`, in order, each in the next row (League::init_table_row
/// `init_table_row`: thiscall(league, row, club, mark, 0), ret 0x10), with mark 1 on the clubs whose competition (+0x57, the id
/// at +0) is one of `marcadas`.
///
/// Returns whether it filled up (the row count = +0x3e).
pub(crate) unsafe fn create_french_table(this: League, clubs: usize, n: i16, file: usize, row: u32, marcadas: &[usize]) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    while i < this.club_count() {
        let club = Club(read_usize(clubs + (i as usize) * 4));
        let id = read_usize(club.division().addr());
        let mark = marcadas.iter().any(|&g| read_usize(g) == id) as u32; // cqk6: the mark of the higher-division clubs
        // simplification: the original pushes the row in ecx with the upper bits of the competition id; 0x687430 only reads the
        // byte.
        let row = k; // cqk6: row k receives the k-th club of the list
        exe::league_init_table_row(this, row as u32, club, mark, 0);
        k = k.wrapping_add(1);
        i = i.wrapping_add(1);
    }
    covered!("cqk_6 french lower fixtures");
    (k as i8 as i16 == this.club_count()) as u32
}

/// a CFA group has 18 clubs (4 groups, drawn by the constructor); clubs that are in the first or second division get
/// the row mark [inference: the reserve sides of professional clubs, which cannot go up]
/// Original: `FraCfa::create_group_table` (0x5b9fa0, fra_cfa.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fra_cfa_create_group_table(this: usize, clubs: usize) -> u32 {
    create_french_table(League(this), clubs, 0x12, 0x9ccaf0, 0x185, &[KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION])
}

/// Comparer of the French table creation: original only, then checks +0x3e, return 1 and rows with the planned clubs
/// and marks (+0x38).
pub(crate) unsafe fn compare_french_table(name: &str, this: League, clubs: usize, n: i16, marcadas: &[usize], original: impl FnOnce() -> u32) -> u32 {
    let plan: Vec<(usize, u8)> = (0..n as usize)
        .map(|i| {
            let c = Club(read_usize(clubs + i * 4));
            let id = read_usize(c.division().addr());
            (c.addr(), marcadas.iter().any(|&g| read_usize(g) == id) as u8)
        })
        .collect();
    let r = original();
    let t = this.table();
    let mut found: Vec<(usize, u8)> = Vec::new();
    let mut same = r == 1 && this.club_count() == n && t != 0;
    if same {
        found = (0..n as usize).map(|k| (read_usize(t + k * league_table_row::SIZE), read_u8(t + k * league_table_row::SIZE + league_table_row::MARK))).collect();
        same = found == plan;
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// The club's situation in the Serie C1 stats (the body of `ita_ser_c1a_set_club_progress` and `ita_ser_c1b_set_club_progress`): in the club's entry
/// (Comp_Stats::find_club_entry `exe::comp_stats_find_club_entry`: thiscall(+8, club), ret 4) it writes the current position (+0x73), the best
/// (+0x74) and the worst (+0x75), folding play-off/play-out ranges by stage.
///
/// In the league (stage 0xff): a best of 2 to 5 (the promotion play-off) becomes 2 and of 14 to 17 (the play-out)
/// becomes 14; a worst of 2 to 5 becomes 5 and of 14 to 17 becomes 17.
///
/// In the play-off (stage 0): all three + 1. In the play-out (stage 1): best 1 becomes 14 and 2 becomes 16. In any
/// other stage, as they came.
pub(crate) unsafe fn set_c1_club_progress(this: usize, club: usize, stage: u8, current: u8, best: u8, worst: u8) {
    let e = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(this + 8), club));
    let (mut a, mut m, mut p) = (current, best, worst);
    if stage == 0xff {
        m = match m {
            2..=5 => 2,
            14..=17 => {
                covered!("cqk_6 play-out band");
                0xe // cqk6: the best in the play-out band
            }
            v => v,
        };
        p = match p {
            2..=5 => 5,
            14..=17 => 0x11,
            v => v,
        };
    } else if stage == 0 {
        m = m.wrapping_add(1);
        p = p.wrapping_add(1);
        a = a.wrapping_add(1);
    } else if stage == 1 {
        m = match m {
            1 => 0xe,
            2 => 0x10,
            v => v,
        };
    }
    e.set_current_position(a);
    e.set_best_position(m);
    e.set_worst_position(p);
}

/// Comparer of set_club_progress: finds the stats entry first and runs both side by side on its 3 bytes; without entry,
/// original only.
pub(crate) unsafe fn compare_c1_club_progress(name: &str, this: usize, club: usize, original: impl FnOnce(), copy: impl FnOnce()) {
    let e = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(this + 8), club));
    if e.is_null() {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(e.addr() + comp_stats_club::CURRENT_POSITION, 3)], original, copy);
    SCOREBOARD.record(igual, || format!("{}({:#x}, {:#x}) original={:x?} rust={:x?}", name, this, club, mo, mr));
}

