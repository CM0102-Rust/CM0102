use super::*;

/// Rule: the German 2. Bundesliga turnover (called by the Bundesliga's): the same template as the Danish Second
/// Division (`rebuild_season` with GerSecond::create_table at 0x5df2b0 and the boxes of ger_second.cpp at lines 0x20a,
/// 0x211, 0x217, 0x21e; then `set_expectations_and_renew_officials`). After a box: 0.
/// Original: `GerSecond::start_new_season` (0x5df490, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5df2b0, 0x9cd4c4, [0x20a, 0x211, 0x217, 0x21e], 1) {
        return 0;
    }
    covered!("mwc_18 german second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ger_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_ger_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("ger_second_start_new_season", League(this), || o(this))
}

/// The 2. Bundesliga schedule: 34 rounds, from 30 Jul to 6 May; 28 with matches in TV slots.
///
/// `GerSecond::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage
/// (-1; the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa7 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `GerSecond::create_schedule` (0x5de5d0, ger_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &GER_SECOND_SCHEDULE)
}

/// Comparer for `ger_second_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ger_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GER_SECOND_CREATE_SCHEDULE);
    compare_cfa_schedule("ger_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ger_second_create_schedule(this, stage, r, e, l))
}

/// The 2. Bundesliga: discipline rule 13 (`GERMANY_LEAGUE`), only the league; with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `GerSecond::GerSecond` (0x5de2e0, ger_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_second(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &GER_SECOND_DIVISION, || {})
}

/// Comparer for `construct_ger_second`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_ger_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GER_SECOND);
    compare_nation_division_ctor("construct_ger_second", League(this), year, record, &GER_SECOND_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the 2. Bundesliga: 2 rounds, 3 points for a win and 1 for a draw; 3 go up, 0 to the promotion
/// play-off, 0 to the relegation play-off and 4 go down; above: `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`; below:
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` when the German lower division is playable (bit 4 of the nation's
/// `LEAGUE_SELECTED`), else none; tie-breaker codes 1, 2, 3 (`+0xc5`..); 6 substitutes named and 3 substitutions; then
/// builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`).
/// Returns 1.
/// Original: `GerSecond::set_up_rules` (0x5df3e0, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_set_up_rules(this: usize) -> u32 {
    covered!("mwc_8 ger second rules");
    fra_first_apply_league_rules(League(this), &ger_second_rules(this))
}

/// Comparer for `ger_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_ger_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_SECOND_SET_UP_RULES);
    compare_nation_league_rules("ger_second_set_up_rules", League(this), ger_second_rules(this), || o(this), || ger_second_set_up_rules(this))
}

/// The 2. Bundesliga has 18 clubs; prize fund by the share of 50,000 per place; TV pays 1,300,000 to each club.
///
/// `GerSecond::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box
/// of line 0x19f and 0) with the clubs of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0xc350), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0x13d620 to every club) and 1.
/// Original: `GerSecond::create_table` (0x5df2b0, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x12, 0x9cd4c4, 0x19f, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0x13d620, 0); // mwc8: the TV money of each club
    })
}

/// Comparer for `ger_second_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_ger_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_SECOND_CREATE_TABLE);
    compare_nation_league_table("ger_second_create_table", League(this), 0x12, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION, Some(0xc350), || o(this))
}

/// The rule writes of the 2. Bundesliga (offset, size, value).
pub(crate) unsafe fn ger_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let this = League(this);
    // the Regionalliga below only when the nation has its lower division playable (bit 4 of `nation::LEAGUE_SELECTED`)
    let nation = Nation(read_usize(this.competition().addr() + competition::NATION));
    let below = if nation.league_selected() & 4 != 0 { read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) as u32 } else { 0xffff_ffff };
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbe, 1, 3), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION) as u32), (0x20, 4, below), (0x52, 1, 2), (0x4a, 1, 3), (0x49, 1, 6)];
    v.push((0xc2, 1, 3)); // mwc8: the 2. Bundesliga points for a win
    v
}

pub(crate) const GER_SECOND_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd4c4, row: 0xa7, table: &GER_SECOND_SCHEDULE_ROUNDS, branch: "mwc_2 ger second schedule" };

pub(crate) const GER_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b304,
    bench: None,
    discipline: discipline_rule::GERMANY_LEAGUE, // mwc8: the 2. Bundesliga discipline rule
    stages: 0,
    set_up_rules: 0x5df3e0,
    create_table: 0x5df2b0,
    source_file: 0x9cd4c4,
    box_lines: [0x43, 0x4a, 0x51, 0x5a, 0x61],
};

/// The 2. Bundesliga: 34 rounds (date, TV slots).
pub(crate) const GER_SECOND_SCHEDULE_ROUNDS: [FraThirdLeagueRound; 34] = [
    (30, 6, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 0: 30 Jul
    (6, 7, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 1: 6 Aug
    (13, 7, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 2: 13 Aug
    (20, 7, 0, 6, 1, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 3: 20 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 4: 6 Sep
    (10, 8, 0, 6, 1, &[(0, 2, 1), (-1, -1, -1)]), // 5: 10 Sep (mwc2 ger_second)
    (17, 8, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 6: 17 Sep
    (24, 8, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 7: 24 Sep
    (1, 9, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 8: 1 Oct
    (15, 9, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 9: 15 Oct
    (22, 9, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 10: 22 Oct
    (29, 9, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 11: 29 Oct
    (5, 10, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 12: 5 Nov
    (12, 10, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 13: 12 Nov
    (19, 10, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 14: 19 Nov
    (26, 10, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 15: 26 Nov
    (3, 11, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 16: 3 Dec
    (10, 11, 0, 6, 1, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 17: 10 Dec
    (13, 11, 0, 2, 2, &[(-1, -1, -1)]), // 18: 13 Dec
    (17, 11, 0, 6, 1, &[(-1, -1, -1)]), // 19: 17 Dec
    (28, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 28 Jan (+1 year)
    (4, 1, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 21: 4 Feb (+1 year)
    (11, 1, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 22: 11 Feb (+1 year)
    (18, 1, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 23: 18 Feb (+1 year)
    (25, 1, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 24: 25 Feb (+1 year)
    (4, 2, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 25: 4 Mar (+1 year)
    (11, 2, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 26: 11 Mar (+1 year)
    (18, 2, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 27: 18 Mar (+1 year)
    (1, 3, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 28: 1 Apr (+1 year)
    (8, 3, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 29: 8 Apr (+1 year)
    (15, 3, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 30: 15 Apr (+1 year)
    (22, 3, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 31: 22 Apr (+1 year)
    (29, 3, 1, 6, 1, &[(-1, -1, -1)]), // 32: 29 Apr (+1 year)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 6 May (+1 year)
];
