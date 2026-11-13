// ---------- batch %%LEVA%%: the Bundesliga: the constructor, the rules and the schedule (ger_first.cpp) ----------

pub(crate) const GER_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b0e4,
    bench: None,
    discipline: discipline_rule::GERMANY_LEAGUE, // mwc11: the Bundesliga discipline rule
    stages: 0,
    set_up_rules: 0x5db400,
    create_table: 0x5db230,
    source_file: 0x9cd414,
    box_lines: [0x47, 0x4e, 0x55, 0x5e, 0x65],
};

/// The Bundesliga: discipline rule 13 (`GERMANY_LEAGUE`), only the league; with no saved state, the rules, the table,
/// the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `GerFirst::GerFirst` (0x5d99b0, ger_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &GER_FIRST_DIVISION, || {})
}

/// Comparer for `construct_ger_first`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_ger_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_division_ctor("construct_ger_first", League(this), year, record, &GER_FIRST_DIVISION, || o(this, year, record))
}

/// The rule writes of the Bundesliga (offset, size, value).
pub(crate) unsafe fn ger_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, 0xffff_ffff), (0x4a, 1, 3), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) as u32), (0x49, 1, 6)];
    v.push((0xc2, 1, 3)); // mwc11: the Bundesliga points for a win
    v
}

/// Sets up the rules of the Bundesliga: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the promotion
/// play-off, 0 to the relegation play-off and 3 go down; above: none; below: `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`;
/// tie-breaker codes 1, 2, 3 (`+0xc5`..); 6 substitutes named and 3 substitutions; then builds the main-stage round
/// table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `GerFirst::set_up_rules` (0x5db400, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_11 ger first rules");
    fra_first_apply_league_rules(League(this), &ger_first_rules(this))
}

/// Comparer for `ger_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_ger_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("ger_first_set_up_rules", League(this), ger_first_rules(this), || o(this), || ger_first_set_up_rules(this))
}

/// The Bundesliga 2001/02: 34 rounds (the first date with the literal year 2001).
pub(crate) const GER_FIRST_2001_ROUNDS: [FraThirdLeagueRound; 34] = [
    (29, 6, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 0: 29 Jul
    (5, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 1: 5 Aug
    (12, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 2: 12 Aug
    (19, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 3: 19 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 4: 6 Sep
    (9, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 5: 9 Sep (mwc11 ger_first)
    (16, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 6: 16 Sep
    (23, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 7: 23 Sep
    (30, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 8: 30 Sep
    (14, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 9: 14 Oct
    (21, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 10: 21 Oct
    (28, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 11: 28 Oct
    (4, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 12: 4 Nov
    (11, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 13: 11 Nov
    (18, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 14: 18 Nov
    (25, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 15: 25 Nov
    (2, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 16: 2 Dec
    (9, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 17: 9 Dec
    (13, 11, 0, 2, 2, &[(-1, -1, -1)]), // 18: 13 Dec
    (16, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 19: 16 Dec
    (27, 0, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 20: 27 Jan (+1 year)
    (3, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 21: 3 Feb (+1 year)
    (10, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 22: 10 Feb (+1 year)
    (17, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 23: 17 Feb (+1 year)
    (24, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 24: 24 Feb (+1 year)
    (3, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 25: 3 Mar (+1 year)
    (10, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 26: 10 Mar (+1 year)
    (17, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 27: 17 Mar (+1 year)
    (31, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 28: 31 Mar (+1 year)
    (7, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 29: 7 Apr (+1 year)
    (14, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 30: 14 Apr (+1 year)
    (21, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 31: 21 Apr (+1 year)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 28 Apr (+1 year)
    (5, 4, 1, 5, 1, &[(-1, -1, -1)]), // 33: 5 May (+1 year)
];
pub(crate) const GER_FIRST_2001: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd414, row: 0xa1, table: &GER_FIRST_2001_ROUNDS, branch: "mwc_11 ger first schedule" };

/// The Bundesliga from 2002/03: 34 rounds.
pub(crate) const GER_FIRST_LATER_ROUNDS: [FraThirdLeagueRound; 34] = [
    (12, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 0: 12 Aug
    (19, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 1: 19 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 2: 6 Sep
    (9, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 3: 9 Sep
    (16, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 4: 16 Sep
    (23, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 5: 23 Sep
    (30, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 6: 30 Sep
    (14, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 7: 14 Oct
    (21, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 8: 21 Oct
    (28, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 9: 28 Oct
    (4, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 10: 4 Nov
    (11, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 11: 11 Nov
    (18, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 12: 18 Nov
    (25, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 13: 25 Nov
    (2, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 14: 2 Dec
    (9, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 15: 9 Dec
    (13, 11, 0, 2, 2, &[(-1, -1, -1)]), // 16: 13 Dec
    (16, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 17: 16 Dec
    (27, 0, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 18: 27 Jan (+1 year)
    (3, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 19: 3 Feb (+1 year)
    (10, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 20: 10 Feb (+1 year)
    (17, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 21: 17 Feb (+1 year)
    (24, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 22: 24 Feb (+1 year)
    (3, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 23: 3 Mar (+1 year)
    (10, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 24: 10 Mar (+1 year)
    (17, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 25: 17 Mar (+1 year)
    (31, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 26: 31 Mar (+1 year)
    (7, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 27: 7 Apr (+1 year)
    (14, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 28: 14 Apr (+1 year)
    (21, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 29: 21 Apr (+1 year)
    (28, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 30: 28 Apr (+1 year)
    (5, 4, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 31: 5 May (+1 year)
    (12, 4, 1, 5, 1, &[(-1, -1, -1)]), // 32: 12 May (+1 year)
    (19, 4, 1, 5, 1, &[(-1, -1, -1)]), // 33: 19 May (+1 year)
];
pub(crate) const GER_FIRST_LATER: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd414, row: 0xa1, table: &GER_FIRST_LATER_ROUNDS, branch: "mwc_11 ger first schedule" };

/// The Bundesliga schedule: 34 rounds, from 29 Jul to 5 May; 30 with matches in TV slots; in 2001/02 its own table, in
/// the following seasons another.
///
/// `GerFirst::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa1 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `GerFirst::create_schedule` (0x5d9c70, ger_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if year as u16 == 0x7d1 { &GER_FIRST_2001 } else { &GER_FIRST_LATER };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `ger_first_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_ger_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_cfa_schedule("ger_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ger_first_create_schedule(this, stage, r, e, l))
}
