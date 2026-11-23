use super::*;

/// Rule: the Danish First Division turnover (called by the Superliga's): the season reopened (`league::SEASON_CLOSED`
/// = 0), rebuilt for the next year with the stage fields kept (`rebuild_turkish_league`: the table, the round dates and
/// the statistics; the year + 1; the rules, DenFirst::create_table at 0x552290, the fixtures and the order, each with
/// the box of den_first.cpp at lines 0x1a3, 0x1aa, 0x1b0, 0x1b7), then the predicted positions, the board expectations
/// and the officials (`finish_croatian_turnover`), whose eax is returned. After a box the copy returns 0 (the original
/// returns the box's eax; the caller does not read it).
/// Original: `DenFirst::start_new_season` (0x552450, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwc_18 danish first division turnover");
    if !rebuild_turkish_league(this, 0x552290, 0x9b8e30, [0x1a3, 0x1aa, 0x1b0, 0x1b7]) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Comparer for `den_first_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_den_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("den_first_start_new_season", League(this), || o(this))
}

/// The Danish First Division schedule: 30 rounds, from 3 Aug to 24 May; 0 with matches in TV slots; the spring changes
/// with the parity of the season year (from round 15: odd, as 2001/02, and even).
///
/// `DenFirst::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 30, extra = 0 and the table of 30 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0x9c and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `DenFirst::create_schedule` (0x5519b0, den_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if (year as i32) % 2 != 0 { &DEN_FIRST_ODD_YEAR } else { &DEN_FIRST_EVEN_YEAR };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `den_first_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_den_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_DEN_FIRST_CREATE_SCHEDULE);
    compare_cfa_schedule("den_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_first_create_schedule(this, stage, r, e, l))
}

/// The Danish First Division: discipline rule 8 (`DENMARK`), only the league; with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `DenFirst::DenFirst` (0x5516f0, den_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &DEN_FIRST_DIVISION, || {})
}

/// Comparer for `construct_den_first`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_den_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_DEN_FIRST);
    compare_nation_division_ctor("construct_den_first", League(this), year, record, &DEN_FIRST_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Danish First Division: 2 rounds, 3 points for a win and 1 for a draw; 2 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 3 go down; above: `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION`;
/// below: `KEY_CLUB_COMP_DANISH_SECOND_DIVISION`; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3
/// substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `DenFirst::set_up_rules` (0x5523b0, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_6 den first rules");
    fra_first_apply_league_rules(League(this), &den_first_rules(this))
}

/// Comparer for `den_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_den_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_FIRST_SET_UP_RULES);
    compare_nation_league_rules("den_first_set_up_rules", League(this), den_first_rules(this), || o(this), || den_first_set_up_rules(this))
}

/// The Danish First Division has 16 clubs; prize fund by the share of 10,000 per place; no TV money.
///
/// `DenFirst::create_table()`: `+0x3e` = 16, the table (`exe::crt_malloc` of 16 x 0x3b; out of memory, the error box of
/// line 0x142 and 0) with the clubs of `KEY_CLUB_COMP_DANISH_FIRST_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0x2710), the prize
/// mode `+0xe1` = 1 and 1.
/// Original: `DenFirst::create_table` (0x552290, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b8e30, 0x142, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, 0x2710, || {
        // no TV money
    })
}

/// Comparer for `den_first_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_den_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_FIRST_CREATE_TABLE);
    compare_nation_league_table("den_first_create_table", League(this), 0x10, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, Some(0x2710), || o(this))
}

/// The rebuild of the Turkish turnovers, which keep the stage fields: League::free_table(this, 0); the round dates
/// (+0xba) freed and cleared; the statistics archived (push 1) when there are any; the year (+0x40) + 1; then, each
/// with the box of its line (`rows`) and stopping when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`),
/// the table (`table`, direct call), the fixtures (League::create_fixtures) and the order (League::sort_table: push 0).
/// Unlike the Croatian rebuild, +0xc, +0x2c, +0x30 and +0x4c are not touched.
pub(crate) unsafe fn rebuild_turkish_league(this: League, table: usize, file: usize, rows: [u32; 4]) -> bool {
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    this.set_year(this.year().wrapping_add(1));
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return false;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return false;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return false;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return false;
    }
    true
}

/// The rule writes of the Danish First Division (offset, size, value).
pub(crate) unsafe fn den_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_DANISH_PREMIER_DIVISION) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_DANISH_SECOND_DIVISION) as u32), (0x4a, 1, 3), (0x49, 1, 4)];
    v.push((0xc2, 1, 3)); // mwc6: the Danish First Division points for a win
    v
}

pub(crate) const DEN_FIRST_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8e30, row: 0x9c, table: &DEN_FIRST_ODD_YEAR_ROUNDS, branch: "mwc_3 den first schedule" };

pub(crate) const DEN_FIRST_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8e30, row: 0x9c, table: &DEN_FIRST_EVEN_YEAR_ROUNDS, branch: "mwc_3 den first schedule" };

pub(crate) const DEN_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x969630,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc6: the Danish First Division discipline rule
    stages: 0,
    set_up_rules: 0x5523b0,
    create_table: 0x552290,
    source_file: 0x9b8e30,
    box_lines: [0x42, 0x49, 0x50, 0x59, 0x60],
};

/// The Danish First Division in a season starting in an odd year (2001/02): the spring from round 15 differs.
pub(crate) const DEN_FIRST_ODD_YEAR_ROUNDS: [FraThirdLeagueRound; 30] = [
    (3, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 3 Aug
    (10, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 10 Aug
    (17, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 17 Aug
    (24, 7, 0, 6, 1, &[(-1, -1, -1)]), // 3: 24 Aug
    (31, 7, 0, 6, 1, &[(-1, -1, -1)]), // 4: 31 Aug
    (7, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 7 Sep (mwc3 den_first)
    (14, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 14 Sep
    (21, 8, 0, 6, 1, &[(-1, -1, -1)]), // 7: 21 Sep
    (28, 8, 0, 6, 1, &[(-1, -1, -1)]), // 8: 28 Sep
    (5, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 5 Oct
    (12, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 12 Oct
    (19, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 19 Oct
    (26, 9, 0, 6, 1, &[(-1, -1, -1)]), // 12: 26 Oct
    (2, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2 Nov
    (9, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9 Nov
    (4, 2, 1, 2, 2, &[(-1, -1, -1)]), // 15: 4 Mar (+1 year)
    (8, 2, 1, 6, 1, &[(-1, -1, -1)]), // 16: 8 Mar (+1 year)
    (15, 2, 1, 6, 1, &[(-1, -1, -1)]), // 17: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(-1, -1, -1)]), // 18: 22 Mar (+1 year)
    (25, 2, 1, 2, 2, &[(-1, -1, -1)]), // 19: 25 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(-1, -1, -1)]), // 20: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(-1, -1, -1)]), // 21: 5 Apr (+1 year)
    (8, 3, 1, 2, 2, &[(-1, -1, -1)]), // 22: 8 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(-1, -1, -1)]), // 24: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(-1, -1, -1)]), // 25: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(-1, -1, -1)]), // 26: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(-1, -1, -1)]), // 27: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(-1, -1, -1)]), // 28: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 29: 24 May (+1 year)
];

/// The Danish First Division in a season starting in an even year.
pub(crate) const DEN_FIRST_EVEN_YEAR_ROUNDS: [FraThirdLeagueRound; 30] = [
    (3, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 3 Aug
    (10, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 10 Aug
    (17, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 17 Aug
    (24, 7, 0, 6, 1, &[(-1, -1, -1)]), // 3: 24 Aug
    (31, 7, 0, 6, 1, &[(-1, -1, -1)]), // 4: 31 Aug
    (7, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 7 Sep
    (14, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 14 Sep
    (21, 8, 0, 6, 1, &[(-1, -1, -1)]), // 7: 21 Sep
    (28, 8, 0, 6, 1, &[(-1, -1, -1)]), // 8: 28 Sep
    (5, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 5 Oct
    (12, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 12 Oct
    (19, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 19 Oct
    (26, 9, 0, 6, 1, &[(-1, -1, -1)]), // 12: 26 Oct
    (2, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2 Nov
    (9, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9 Nov
    (15, 2, 1, 6, 1, &[(-1, -1, -1)]), // 15: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(-1, -1, -1)]), // 16: 22 Mar (+1 year)
    (25, 2, 1, 2, 2, &[(-1, -1, -1)]), // 17: 25 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(-1, -1, -1)]), // 18: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(-1, -1, -1)]), // 19: 5 Apr (+1 year)
    (8, 3, 1, 2, 2, &[(-1, -1, -1)]), // 20: 8 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(-1, -1, -1)]), // 21: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(-1, -1, -1)]), // 22: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(-1, -1, -1)]), // 24: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(-1, -1, -1)]), // 25: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(-1, -1, -1)]), // 26: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 27: 24 May (+1 year)
    (31, 4, 1, 6, 1, &[(-1, -1, -1)]), // 28: 31 May (+1 year)
    (7, 5, 1, 6, 1, &[(-1, -1, -1)]), // 29: 7 Jun (+1 year)
];
