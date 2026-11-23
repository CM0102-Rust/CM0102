use super::*;

/// Rule: the Greek Second Division has 14 clubs (`KEY_CLUB_COMP_GREEK_SECOND_DIVISION`); when full, the same prize
/// fund (100,000 a share) and TV money (500,000) as the First Division (the box at line 0x169).
/// Original: `GreSecond::create_table` (0x5ebcb0, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xe, 0x9cdb1c, 0x169, KEY_CLUB_COMP_GREEK_SECOND_DIVISION, 0x186a0, || {
        covered!("mwd_15 greek second division tv money");
        eng_prm_pay_tv_rights(this, 0x7a120, 0)
    })
}

/// Comparer for `gre_second_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_SECOND_CREATE_TABLE);
    eng_prm_compare_league_table("gre_second_create_table", League(this), 0xe, KEY_CLUB_COMP_GREEK_SECOND_DIVISION, 0x186a0, || o(this))
}

/// Rule: the Greek Second Division schedule: 30 rounds on Saturdays from 16 September to 28 April (break from 16
/// December to 6 January); stage 0 as the First Division's.
/// Original: `GreSecond::create_schedule` (0x5eb570, gre_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    fixed_league_schedule(League(this), stage, rounds, extra, league, 0x9cdb1c, &LeagueStageRounds { rounds: &GREEK_SECOND_DIVISION_ROUNDS, extra: 0, league: 1, row: 0xa8 },
        Some(&LeagueStageRounds { rounds: &GREEK_PLAYOFF_ROUNDS, extra: 0x44c, league: 1, row: 0x143 }), "mwd_17 greek second division calendar")
}

/// Comparer for `gre_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GRE_SECOND_CREATE_SCHEDULE);
    compare_league_stage_rounds("gre_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| gre_second_create_schedule(this, stage, r, e, l))
}

/// Rule: the Greek Second Division turnover (called by the First Division's): reopened, rebuilt for the next year
/// (`rebuild_croatian_league` without reopening: lines 0x1dc, 0x1e3, 0x1e9, 0x1f0 of gre_second.cpp), the predicted
/// positions, the board expectations and the officials.
/// Original: `GreSecond::start_new_season` (0x5ebe90, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_22 greek second division turnover");
    if !rebuild_croatian_league(this, 0x5ebcb0, 0x9cdb1c, [0x1dc, 0x1e3, 0x1e9, 0x1f0], false) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Comparer for `gre_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("gre_second_start_new_season", League(this), || o(this))
}

/// Rule: the Greek Second Division at career creation, as the First Division (`construct_division_league`).
/// Original: `GreSecond::GreSecond` (0x5eb2b0, gre_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_second(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_11 greek second division");
    construct_division_league(League(this), year, record, &GREEK_SECOND_DIVISION)
}

/// Comparer for `construct_gre_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GRE_SECOND);
    compare_division_league_ctor("construct_gre_second", League(this), year, record, &GREEK_SECOND_DIVISION, || o(this, year, record))
}

/// Rule: the Greek Second Division: as the First Division, 2 go up and 2 go down (`gre_second_rules`).
/// Original: `GreSecond::set_up_rules` (0x5ebde0, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 greek second division rules");
    apply_league_rules_with_flags(League(this), 0x40, &gre_second_rules())
}

/// Comparer for `gre_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("gre_second_set_up_rules", League(this), gre_second_rules(), || o(this), || gre_second_set_up_rules(this))
}

/// The rules of the Greek Second Division: as the First Division, with 2 going up (+0xbe) and 2 down, the First
/// Division above (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`, +0x1c) and nothing below.
pub(crate) unsafe fn gre_second_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3),
        (0xc8, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xbe, 1, 2), (0xc1, 1, 2), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_GREEK_PREMIER_DIVISION) as u32),
        (0x52, 1, 2), (0x20, 4, 0xffff_ffff), (0x49, 1, 7), (0xa7, 2, 0xffff), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The Greek Second Division: 30 rounds on Saturdays (weekday 5), 16 September to 28 April, the break from 16
/// December to 6 January.
pub(crate) const GREEK_SECOND_DIVISION_ROUNDS: [SpaSecondScheduleRound; 30] = [
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 0: 16/set
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 1: 23/set
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 2: 30/set
    (7, 9, 0, 5, 1, &[(-1, -1, -1)]), // 3: 7/out
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 4: 14/out
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 5: 21/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 6: 28/out
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 7: 4/nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 8: 11/nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 9: 18/nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 10: 25/nov
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 11: 2/dez
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 12: 9/dez
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 13: 16/dez
    (6, 0, 1, 5, 1, &[(-1, -1, -1)]), // 14: 6/jan (+1 ano)
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 15: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 16: 20/jan (+1 ano)
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 17: 27/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 18: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(-1, -1, -1)]), // 19: 10/fev (+1 ano)
    (17, 1, 1, 5, 1, &[(-1, -1, -1)]), // 20: 17/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 21: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 22: 3/mar (+1 ano)
    (10, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 10/mar (+1 ano)
    (17, 2, 1, 5, 1, &[(-1, -1, -1)]), // 24: 17/mar (+1 ano)
    (24, 2, 1, 5, 1, &[(-1, -1, -1)]), // 25: 24/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 26: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(-1, -1, -1)]), // 27: 7/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 28: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 29: 28/abr (+1 ano)
];

/// The Greek Second Division (Beta Ethniki).
pub(crate) const GREEK_SECOND_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96b61c,
    discipline: 0x20,
    rules: 0x5ebde0,
    table: 0x5ebcb0,
    file: 0x9cdb1c,
    rows: [0x49, 0x50, 0x57, 0x60, 0x67],
    low: false,
};
