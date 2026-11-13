// ---------- batch %%LEVA%%: the schedules of the three Danish divisions (odd and even years) (den_first.cpp, den_second.cpp, den_prm.cpp) ----------

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
pub(crate) const DEN_FIRST_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8e30, row: 0x9c, table: &DEN_FIRST_ODD_YEAR_ROUNDS, branch: "mwc_3 den first schedule" };

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
pub(crate) const DEN_FIRST_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8e30, row: 0x9c, table: &DEN_FIRST_EVEN_YEAR_ROUNDS, branch: "mwc_3 den first schedule" };

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
unsafe extern "thiscall" fn compare_den_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_cfa_schedule("den_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_first_create_schedule(this, stage, r, e, l))
}

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
pub(crate) const DEN_SECOND_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8ea4, row: 0xa2, table: &DEN_SECOND_ODD_YEAR_ROUNDS, branch: "mwc_3 den second schedule" };

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
pub(crate) const DEN_SECOND_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8ea4, row: 0xa2, table: &DEN_SECOND_EVEN_YEAR_ROUNDS, branch: "mwc_3 den second schedule" };

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
unsafe extern "thiscall" fn compare_den_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cfa_schedule("den_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_second_create_schedule(this, stage, r, e, l))
}

/// The Danish Superliga in a season starting in an odd year (2001/02): the spring from round 18 differs.
pub(crate) const DEN_PRM_ODD_YEAR_ROUNDS: [FraThirdLeagueRound; 33] = [
    (27, 6, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 27 Jul
    (3, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 3 Aug
    (10, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 10 Aug
    (17, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 17 Aug
    (24, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 24 Aug
    (31, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 31 Aug (mwc3 den_prm)
    (7, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 7 Sep
    (14, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 14 Sep
    (21, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 21 Sep
    (28, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 28 Sep
    (5, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 5 Oct
    (12, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 12 Oct
    (19, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 19 Oct
    (26, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 26 Oct
    (2, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 2 Nov
    (9, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 9 Nov
    (16, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 16 Nov
    (23, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 23 Nov
    (1, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 18: 1 Mar (+1 year)
    (4, 2, 1, 2, 2, &[(-1, -1, -1)]), // 19: 4 Mar (+1 year)
    (8, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 20: 8 Mar (+1 year)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 21: 11 Apr (+1 year)
    (15, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 22: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 23: 22 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 24: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 25: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 26: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 27: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 28: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 29: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 30: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 31: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 24 May (+1 year)
];
pub(crate) const DEN_PRM_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 33, file: 0x9b8e6c, row: 0xa6, table: &DEN_PRM_ODD_YEAR_ROUNDS, branch: "mwc_3 den prm schedule" };

/// The Danish Superliga in a season starting in an even year.
pub(crate) const DEN_PRM_EVEN_YEAR_ROUNDS: [FraThirdLeagueRound; 33] = [
    (27, 6, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 27 Jul
    (3, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 3 Aug
    (10, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 10 Aug
    (17, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 17 Aug
    (24, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 24 Aug
    (31, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 31 Aug
    (7, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 7 Sep
    (14, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 14 Sep
    (21, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 21 Sep
    (28, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 28 Sep
    (5, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 5 Oct
    (12, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 12 Oct
    (19, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 19 Oct
    (26, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 26 Oct
    (2, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 2 Nov
    (9, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 9 Nov
    (16, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 16 Nov
    (23, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 23 Nov
    (15, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 18: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 19: 22 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 20: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 21: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 22: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 23: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 24: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 25: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 26: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 27: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 28: 24 May (+1 year)
    (28, 4, 1, 2, 2, &[(-1, -1, -1)]), // 29: 28 May (+1 year)
    (31, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 30: 31 May (+1 year)
    (7, 5, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 31: 7 Jun (+1 year)
    (14, 5, 1, 6, 1, &[(-1, -1, -1)]), // 32: 14 Jun (+1 year)
];
pub(crate) const DEN_PRM_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 33, file: 0x9b8e6c, row: 0xa6, table: &DEN_PRM_EVEN_YEAR_ROUNDS, branch: "mwc_3 den prm schedule" };

/// The Danish Superliga schedule: 33 rounds, from 27 Jul to 24 May; 30 with matches in TV slots; the spring changes
/// with the parity of the season year (from round 18: odd, as 2001/02, and even).
///
/// `DenPrm::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 33, extra = 0 and the table of 33 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa6 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `DenPrm::create_schedule` (0x552900, den_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if (year as i32) % 2 != 0 { &DEN_PRM_ODD_YEAR } else { &DEN_PRM_EVEN_YEAR };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `den_prm_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_den_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_cfa_schedule("den_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_prm_create_schedule(this, stage, r, e, l))
}
