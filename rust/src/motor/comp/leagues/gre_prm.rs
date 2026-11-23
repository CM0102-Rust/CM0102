use super::*;

/// Rule: the Greek First Division has 14 clubs, those whose national division is it
/// (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`), in database order; when full, the prize fund (100,000 a share) and TV
/// pays 500,000 to each club (`eng_prm_fill_league_table`: the table of 14 × 0x3b, the box at line 0x163; League::
/// calc_prize_fund push 100000; League::pay_tv_rights push 0; push 500000).
/// Original: `GrePrm::create_table` (0x5ead60, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xe, 0x9cdae4, 0x163, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x186a0, || {
        covered!("mwd_15 greek first division tv money");
        eng_prm_pay_tv_rights(this, 0x7a120, 0) // mwd15: the Greek TV money
    })
}

/// Comparer for `gre_prm_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("gre_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x186a0, || o(this))
}

/// Rule: the Greek First Division schedule: 26 rounds on Sundays from 17 September to 6 May (winter break from 17
/// December to 28 January); stage 0, a three-round league stage on 13, 20 and 27 May (code 0x44c), which the league
/// never reaches (it has no extra stage).
/// Original: `GrePrm::create_schedule` (0x5ea6d0, gre_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    fixed_league_schedule(League(this), stage, rounds, extra, league, 0x9cdae4, &LeagueStageRounds { rounds: &GREEK_FIRST_DIVISION_ROUNDS, extra: 0, league: 1, row: 0xa4 },
        Some(&LeagueStageRounds { rounds: &GREEK_PLAYOFF_ROUNDS, extra: 0x44c, league: 1, row: 0x13f }), "mwd_17 greek first division calendar")
}

/// Comparer for `gre_prm_create_schedule` (side by side: the outputs and the written table).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GRE_PRM_CREATE_SCHEDULE);
    compare_league_stage_rounds("gre_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| gre_prm_create_schedule(this, stage, r, e, l))
}

/// Rule: the Greek turnover, led by the First Division: the season reopened (+0x4c = 0); promotion and relegation along
/// the chain (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN` with 0: push 0 -> thiscall, ret 4); the two
/// best Third Division clubs up and the Second Division's relegated down (`gre_prm_swap_second_and_third_division_clubs`,
/// ecx = this); the First Division rebuilt for the next year (`rebuild_croatian_league` without reopening the season:
/// lines 0x1dc, 0x1e3, 0x1e9, 0x1f0); the Second Division turned over (virtual `comp_vtable::START_NEW_SEASON`); then
/// the predicted positions, the board expectations and the officials (`finish_croatian_turnover`).
/// Original: `GrePrm::start_new_season` (0x5eaf50, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 0);
    exe::gre_prm_swap_second_and_third_division_clubs(this);
    covered!("mwd_22 greek first division turnover");
    if !rebuild_croatian_league(this, 0x5ead60, 0x9cdae4, [0x1dc, 0x1e3, 0x1e9, 0x1f0], false) {
        return 0;
    }
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION), comp_vtable::START_NEW_SEASON); // mwd22: the Second Division turnover
    finish_croatian_turnover(this)
}

/// Rule: at the Greek turnover, the two best clubs of the Third Division (`KEY_CLUB_COMP_GREEK_LOWER_DIVISION`, sorted
/// by strength with the best nine shuffled) go up to the Second Division, and the Second Division's relegated clubs
/// (status 3) go down to the Third; with fewer than 2 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x2d5 and return) with the clubs
/// of the Third Division in database order (a 16-bit index; no nation check). With at least 2:
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 3) (cdecl), the first two moved to the Second Division
/// (League::move_club_to_division(this, club, the Second Division's record, 1)); then each row i of the Second Division
/// (its club count read again on each step) with status 3 goes down (League::get_table_row, called twice as the
/// original, and League::relegate_club_to_division(this, club, the Third Division's record, 1)). The list is freed.
/// Original: `gre_prm_swap_second_and_third_division_clubs` (0x5eb120, gre_prm.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_swap_second_and_third_division_clubs(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9cdae4, 0x2d5);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 2 {
        covered!("mwd_22 greek third division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 3); // mwd22: the best nine shuffled
        for k in 0..2 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_GREEK_SECOND_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < League(second).club_count() as i32 {
            if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
                let club = read_usize(exe::league_get_table_row(second, i));
                exe::league_relegate_club_to_division(this, club, comp_record_of(KEY_CLUB_COMP_GREEK_LOWER_DIVISION), 1);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `gre_prm_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_PRM_START_NEW_SEASON);
    compare_state_league_new_season("gre_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `gre_prm_swap_second_and_third_division_clubs`: only the original runs (it draws); with at least 2
/// Third Division clubs, the Second Division's clubs of status 3 must end in the Third Division.
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_swap_second_and_third_division_clubs(this: usize) {
    let lower = comp_record_of(KEY_CLUB_COMP_GREEK_LOWER_DIVISION);
    let l = League(comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION));
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let d = read_usize(read_usize(G_CLUB_TABLE) + i * club::SIZE + club::DIVISION);
        d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    if l.table() != 0 && candidates >= 2 {
        for i in 0..l.club_count() as i32 {
            let row = l.table() + i as usize * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                planned.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GRE_PRM_SWAP_SECOND_AND_THIRD_DIVISION_CLUBS)(this);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("gre_prm_swap_second_and_third_division_clubs({:#x}): relegated clubs not in the Third Division {:x?}", this, wrong));
}

/// Rule: the Greek First Division at career creation: discipline rule 0x20 and, with no saved state, the main stage
/// only, the rules, the table, the fixtures, the order, the statistics and the predicted positions (the division
/// constructor template, `construct_division_league`).
/// Original: `GrePrm::GrePrm` (0x5ea410, gre_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_prm(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_11 greek first division");
    construct_division_league(League(this), year, record, &GREEK_FIRST_DIVISION)
}

/// Comparer for `construct_gre_prm` (only the original runs; `compare_division_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GRE_PRM);
    compare_division_league_ctor("construct_gre_prm", League(this), year, record, &GREEK_FIRST_DIVISION, || o(this, year, record))
}

/// Rule: the Greek First Division: 16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks goal difference, goals
/// for, wins; 2 go down to the Second Division; 7 substitutes, 3 substitutions; rule bit 0x40 (`gre_prm_rules`).
/// Original: `GrePrm::set_up_rules` (0x5eae90, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 greek first division rules");
    apply_league_rules_with_flags(League(this), 0x40, &gre_prm_rules())
}

/// Comparer for `gre_prm_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("gre_prm_set_up_rules", League(this), gre_prm_rules(), || o(this), || gre_prm_set_up_rules(this))
}

/// The body of a league `create_schedule` whose stages are fixed tables (thiscall(this, stage, &rounds, &extra,
/// &league), ret 0x10): for stage −1 (`main`) or stage 0 (`stage0`, when the league has one), the league output (if
/// given), the round count and the extra; the table (`exe::crt_malloc` of 0x41 per round; out of memory: the box at
/// the stage's line and 0) with the stage's rounds (League::set_round_date / set_round_match_slot, cdecl; the year
/// pushed as the word +0x40). Other stages: 0, nothing written.
pub(crate) unsafe fn fixed_league_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, file: usize, main: &LeagueStageRounds,
    stage0: Option<&LeagueStageRounds>, branch: &'static str) -> usize {
    let s = match (stage as u8, stage0) {
        (0xff, _) => main,
        (0, Some(s)) => s,
        _ => return 0,
    };
    if !league.is_null() {
        *league = s.league;
    }
    *rounds = s.rounds.len() as i16;
    *extra = s.extra;
    let t = exe::crt_malloc((s.rounds.len() * 0x41) as u32);
    if t == 0 {
        show_error_at(file, s.row);
        return 0;
    }
    covered!(branch);
    spa_second_set_league_rounds(t, (this.year() as u16) as u32, 0, s.rounds);
    t
}

/// Side-by-side comparer for a league `create_schedule` whose every stage is a league round table (0x41-byte rounds):
/// as `compare_league_round_table`, but the written rounds compared as league rounds (`written_league_round_bytes`) at
/// every stage.
pub(crate) unsafe fn compare_league_stage_rounds(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32,
    original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], original, || copy(&mut r2, &mut e2, pl2));
    let outs = (a == 0 && b == 0) || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        let n = *rounds as usize;
        written_league_round_bytes(a, a, n) == written_league_round_bytes(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("{}(stage {}): outs {} table {}", name, stage as u8 as i8, outs, table));
    a
}

/// The rules of the Greek First Division (16 clubs): 2 legs, 3 points a win and 1 a draw, league matches (+0xc4 = 2),
/// +0xab = +0xdb = 0, national clubs; tie-breaks 1, 2, 3 (goal difference, goals for, wins; +0xc8 = 0 ends); nobody
/// goes up nor to the play-offs, 2 go down (+0xc1) to the Second Division (`KEY_CLUB_COMP_GREEK_SECOND_DIVISION`,
/// +0x20); nothing above; +0x52 = 2; 7 substitutes, 3 substitutions; games per club not computed yet (+0xa7 = −1); the
/// rule bit 0x40 (+0xd9, listed for the comparer).
pub(crate) unsafe fn gre_prm_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3),
        (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, read_usize(KEY_CLUB_COMP_GREEK_SECOND_DIVISION) as u32), (0x49, 1, 7), (0xa7, 2, 0xffff), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The Greek First Division: 26 rounds (14 clubs, 2 legs) on Sundays (weekday 6), 17 September to 6 May, the winter
/// break from 17 December to 28 January.
pub(crate) const GREEK_FIRST_DIVISION_ROUNDS: [SpaSecondScheduleRound; 26] = [
    (17, 8, 0, 6, 1, &[(-1, -1, -1)]), // 0: 17/set
    (24, 8, 0, 6, 1, &[(-1, -1, -1)]), // 1: 24/set
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]), // 2: 1/out
    (15, 9, 0, 6, 1, &[(-1, -1, -1)]), // 3: 15/out
    (22, 9, 0, 6, 1, &[(-1, -1, -1)]), // 4: 22/out
    (29, 9, 0, 6, 1, &[(-1, -1, -1)]), // 5: 29/out
    (5, 10, 0, 6, 1, &[(-1, -1, -1)]), // 6: 5/nov
    (12, 10, 0, 6, 1, &[(-1, -1, -1)]), // 7: 12/nov
    (19, 10, 0, 6, 1, &[(-1, -1, -1)]), // 8: 19/nov
    (26, 10, 0, 6, 1, &[(-1, -1, -1)]), // 9: 26/nov
    (3, 11, 0, 6, 1, &[(-1, -1, -1)]), // 10: 3/dez
    (10, 11, 0, 6, 1, &[(-1, -1, -1)]), // 11: 10/dez
    (17, 11, 0, 6, 1, &[(-1, -1, -1)]), // 12: 17/dez
    (28, 0, 1, 6, 1, &[(-1, -1, -1)]), // 13: 28/jan (+1 ano)
    (4, 1, 1, 6, 1, &[(-1, -1, -1)]), // 14: 4/fev (+1 ano)
    (11, 1, 1, 6, 1, &[(-1, -1, -1)]), // 15: 11/fev (+1 ano)
    (18, 1, 1, 6, 1, &[(-1, -1, -1)]), // 16: 18/fev (+1 ano)
    (25, 1, 1, 6, 1, &[(-1, -1, -1)]), // 17: 25/fev (+1 ano)
    (4, 2, 1, 6, 1, &[(-1, -1, -1)]), // 18: 4/mar (+1 ano)
    (11, 2, 1, 6, 1, &[(-1, -1, -1)]), // 19: 11/mar (+1 ano)
    (18, 2, 1, 6, 1, &[(-1, -1, -1)]), // 20: 18/mar (+1 ano)
    (1, 3, 1, 6, 1, &[(-1, -1, -1)]), // 21: 1/abr (+1 ano)
    (8, 3, 1, 6, 1, &[(-1, -1, -1)]), // 22: 8/abr (+1 ano)
    (22, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23: 22/abr (+1 ano)
    (29, 3, 1, 6, 1, &[(-1, -1, -1)]), // 24: 29/abr (+1 ano)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 25: 6/mai (+1 ano)
];

/// The Greek First Division (Alpha Ethniki): discipline rule 0x20 (the Greek one).
pub(crate) const GREEK_FIRST_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96b568,
    discipline: 0x20, // mwd11: the Greek discipline rule
    rules: 0x5eae90,
    table: 0x5ead60,
    file: 0x9cdae4,
    rows: [0x49, 0x50, 0x57, 0x60, 0x67],
    low: false,
};
