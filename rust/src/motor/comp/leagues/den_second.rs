use super::*;

/// Rule: the Danish Second Division turnover (called by the Superliga's): the season reopened, the stages of the last
/// season destroyed and the league rebuilt for the next year (`rebuild_season`: the table, the round dates, the
/// statistics, the stages 0..=current by their virtual destructor with 1, the year + 1, the current stage −1; the
/// rules, DenSecond::create_table at 0x554a40, the fixtures and the order, each with the box of den_second.cpp at lines
/// 0x1af, 0x1b6, 0x1bc, 0x1c3; the predicted positions), then the board expectations and the officials
/// (`set_expectations_and_renew_officials`), whose eax is returned. After a box: 0.
/// Original: `DenSecond::start_new_season` (0x554c00, den_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x554a40, 0x9b8ea4, [0x1af, 0x1b6, 0x1bc, 0x1c3], 1) {
        return 0;
    }
    covered!("mwc_18 danish second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `den_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_den_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("den_second_start_new_season", League(this), || o(this))
}

/// The Danish Second Division schedule: 30 rounds, from 2 Aug to 31 May; 0 with matches in TV slots; the spring changes
/// with the parity of the season year (from round 15: odd, as 2001/02, and even).
///
/// `DenSecond::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage
/// (-1; the others return 0 without writing); is_league = 1, rounds = 30, extra = 0 and the table of 30 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa2 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `DenSecond::create_schedule` (0x554170, den_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if (year as i32) % 2 != 0 { &DEN_SECOND_ODD_YEAR } else { &DEN_SECOND_EVEN_YEAR };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `den_second_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_den_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_DEN_SECOND_CREATE_SCHEDULE);
    compare_cfa_schedule("den_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_second_create_schedule(this, stage, r, e, l))
}

/// The Danish Second Division: discipline rule 8 (`DENMARK`), only the league; with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `DenSecond::DenSecond` (0x553e80, den_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_second(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &DEN_SECOND_DIVISION, || {})
}

/// Comparer for `construct_den_second`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_den_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_DEN_SECOND);
    compare_nation_division_ctor("construct_den_second", League(this), year, record, &DEN_SECOND_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Danish Second Division: 2 rounds, 3 points for a win and 1 for a draw; 3 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 4 go down; above: `KEY_CLUB_COMP_DANISH_FIRST_DIVISION`; below:
/// none; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3 substitutions; then builds the main-stage
/// round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `DenSecond::set_up_rules` (0x554b60, den_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_second_set_up_rules(this: usize) -> u32 {
    covered!("mwc_7 den second rules");
    fra_first_apply_league_rules(League(this), &den_second_rules(this))
}

/// Comparer for `den_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_den_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_SECOND_SET_UP_RULES);
    compare_nation_league_rules("den_second_set_up_rules", League(this), den_second_rules(this), || o(this), || den_second_set_up_rules(this))
}

/// The Danish Second Division has 16 clubs; prize fund by the share of 5,000 per place; no TV money.
///
/// `DenSecond::create_table()`: `+0x3e` = 16, the table (`exe::crt_malloc` of 16 x 0x3b; out of memory, the error box
/// of line 0x149 and 0) with the clubs of `KEY_CLUB_COMP_DANISH_SECOND_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0x1388), the prize
/// mode `+0xe1` = 1 and 1.
/// Original: `DenSecond::create_table` (0x554a40, den_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b8ea4, 0x149, KEY_CLUB_COMP_DANISH_SECOND_DIVISION, 0x1388, || {
        // no TV money
    })
}

/// Comparer for `den_second_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_den_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_SECOND_CREATE_TABLE);
    compare_nation_league_table("den_second_create_table", League(this), 0x10, KEY_CLUB_COMP_DANISH_SECOND_DIVISION, Some(0x1388), || o(this))
}

/// The rule writes of the Danish Second Division (offset, size, value).
pub(crate) unsafe fn den_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc4, 1, 2), (0xc6, 1, 2), (0xc3, 1, 1), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc7, 1, 0), (0xbe, 1, 3), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x4a, 1, 3), (0x49, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_DANISH_FIRST_DIVISION) as u32), (0x20, 4, 0xffff_ffff)];
    v.push((0xc2, 1, 3)); // mwc7: the Danish Second Division points for a win
    v
}

pub(crate) const DEN_SECOND_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8ea4, row: 0xa2, table: &DEN_SECOND_ODD_YEAR_ROUNDS, branch: "mwc_3 den second schedule" };

pub(crate) const DEN_SECOND_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8ea4, row: 0xa2, table: &DEN_SECOND_EVEN_YEAR_ROUNDS, branch: "mwc_3 den second schedule" };

pub(crate) const DEN_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x969798,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc7: the Danish Second Division discipline rule
    stages: 0,
    set_up_rules: 0x554b60,
    create_table: 0x554a40,
    source_file: 0x9b8ea4,
    box_lines: [0x43, 0x4a, 0x51, 0x5a, 0x61],
};

/// The Danish Second Division in a season starting in an odd year (2001/02): the spring from round 15 differs.
pub(crate) const DEN_SECOND_ODD_YEAR_ROUNDS: [FraThirdLeagueRound; 30] = [
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 2 Aug
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 9 Aug
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 16 Aug
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 3: 23 Aug
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 4: 30 Aug
    (6, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 6 Sep (mwc3 den_second)
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 13 Sep
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 7: 20 Sep
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 8: 27 Sep
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 4 Oct
    (11, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 11 Oct
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 18 Oct
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 12: 25 Oct
    (1, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 1 Nov
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 8 Nov
    (15, 2, 1, 6, 1, &[(-1, -1, -1)]), // 15: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(-1, -1, -1)]), // 16: 22 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(-1, -1, -1)]), // 17: 29 Mar (+1 year)
    (1, 3, 1, 2, 2, &[(-1, -1, -1)]), // 18: 1 Apr (+1 year)
    (5, 3, 1, 6, 1, &[(-1, -1, -1)]), // 19: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(-1, -1, -1)]), // 20: 12 Apr (+1 year)
    (15, 3, 1, 2, 2, &[(-1, -1, -1)]), // 21: 15 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(-1, -1, -1)]), // 22: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(-1, -1, -1)]), // 24: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(-1, -1, -1)]), // 25: 10 May (+1 year)
    (13, 4, 1, 2, 2, &[(-1, -1, -1)]), // 26: 13 May (+1 year)
    (17, 4, 1, 6, 1, &[(-1, -1, -1)]), // 27: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 28: 24 May (+1 year)
    (31, 4, 1, 6, 1, &[(-1, -1, -1)]), // 29: 31 May (+1 year)
];

/// The Danish Second Division in a season starting in an even year.
pub(crate) const DEN_SECOND_EVEN_YEAR_ROUNDS: [FraThirdLeagueRound; 30] = [
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 2 Aug
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 9 Aug
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 16 Aug
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 3: 23 Aug
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 4: 30 Aug
    (6, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 6 Sep
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 13 Sep
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 7: 20 Sep
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 8: 27 Sep
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 4 Oct
    (11, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 11 Oct
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 18 Oct
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 12: 25 Oct
    (1, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 1 Nov
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 8 Nov
    (29, 2, 1, 6, 1, &[(-1, -1, -1)]), // 15: 29 Mar (+1 year)
    (1, 3, 1, 2, 2, &[(-1, -1, -1)]), // 16: 1 Apr (+1 year)
    (5, 3, 1, 6, 1, &[(-1, -1, -1)]), // 17: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(-1, -1, -1)]), // 18: 12 Apr (+1 year)
    (15, 3, 1, 2, 2, &[(-1, -1, -1)]), // 19: 15 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(-1, -1, -1)]), // 20: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(-1, -1, -1)]), // 21: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(-1, -1, -1)]), // 22: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(-1, -1, -1)]), // 23: 10 May (+1 year)
    (13, 4, 1, 2, 2, &[(-1, -1, -1)]), // 24: 13 May (+1 year)
    (17, 4, 1, 6, 1, &[(-1, -1, -1)]), // 25: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 26: 24 May (+1 year)
    (31, 4, 1, 6, 1, &[(-1, -1, -1)]), // 27: 31 May (+1 year)
    (7, 5, 1, 6, 1, &[(-1, -1, -1)]), // 28: 7 Jun (+1 year)
    (14, 5, 1, 6, 1, &[(-1, -1, -1)]), // 29: 14 Jun (+1 year)
];
