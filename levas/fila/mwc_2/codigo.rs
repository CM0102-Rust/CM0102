// ---------- batch %%LEVA%%: the schedules of the Belgian First Division, the 2. Bundesliga and the Eredivisie (bel_first.cpp, ger_second.cpp, hol_prm.cpp) ----------

/// The Belgian First Division: 34 rounds (date, TV slots).
pub(crate) const BEL_FIRST_SCHEDULE_ROUNDS: [FraThirdLeagueRound; 34] = [
    (12, 7, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 0: 12 Aug
    (19, 7, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 1: 19 Aug
    (26, 7, 0, 5, 2, &[(-1, -1, -1)]), // 2: 26 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 3: 6 Sep
    (9, 8, 0, 5, 2, &[(-1, -1, -1)]), // 4: 9 Sep
    (16, 8, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 5: 16 Sep (mwc2 bel_first)
    (23, 8, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 6: 23 Sep
    (30, 8, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 7: 30 Sep
    (14, 9, 0, 5, 2, &[(-1, -1, -1)]), // 8: 14 Oct
    (21, 9, 0, 5, 2, &[(-1, -1, -1)]), // 9: 21 Oct
    (28, 9, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 10: 28 Oct
    (11, 10, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 11: 11 Nov
    (18, 10, 0, 5, 2, &[(-1, -1, -1)]), // 12: 18 Nov
    (25, 10, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 13: 25 Nov
    (2, 11, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 14: 2 Dec
    (9, 11, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 15: 9 Dec
    (16, 11, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 16: 16 Dec
    (20, 11, 0, 2, 2, &[(-1, -1, -1)]), // 17: 20 Dec
    (20, 0, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 18: 20 Jan (+1 year)
    (27, 0, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 19: 27 Jan (+1 year)
    (3, 1, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 20: 3 Feb (+1 year)
    (10, 1, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 21: 10 Feb (+1 year)
    (17, 1, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 22: 17 Feb (+1 year)
    (3, 2, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 23: 3 Mar (+1 year)
    (10, 2, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 24: 10 Mar (+1 year)
    (17, 2, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 25: 17 Mar (+1 year)
    (31, 2, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 26: 31 Mar (+1 year)
    (7, 3, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 27: 7 Apr (+1 year)
    (14, 3, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 28: 14 Apr (+1 year)
    (21, 3, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 29: 21 Apr (+1 year)
    (28, 3, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 30: 28 Apr (+1 year)
    (5, 4, 1, 5, 2, &[(-1, -1, -1)]), // 31: 5 May (+1 year)
    (13, 4, 1, 6, 2, &[(-1, -1, -1)]), // 32: 13 May (+1 year)
    (20, 4, 1, 6, 2, &[(-1, -1, -1)]), // 33: 20 May (+1 year)
];
pub(crate) const BEL_FIRST_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x989b18, row: 0xa5, table: &BEL_FIRST_SCHEDULE_ROUNDS, branch: "mwc_2 bel first schedule" };

/// The Belgian First Division schedule: 34 rounds, from 12 Aug to 20 May; 24 with matches in TV slots.
///
/// `BelFirst::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa5 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `BelFirst::create_schedule` (0x41e010, bel_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &BEL_FIRST_SCHEDULE)
}

/// Comparer for `bel_first_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_bel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_cfa_schedule("bel_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| bel_first_create_schedule(this, stage, r, e, l))
}

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
pub(crate) const GER_SECOND_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd4c4, row: 0xa7, table: &GER_SECOND_SCHEDULE_ROUNDS, branch: "mwc_2 ger second schedule" };

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
unsafe extern "thiscall" fn compare_ger_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cfa_schedule("ger_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ger_second_create_schedule(this, stage, r, e, l))
}

/// The Eredivisie: 34 rounds (date, TV slots).
pub(crate) const HOL_PRM_SCHEDULE_ROUNDS: [FraThirdLeagueRound; 34] = [
    (14, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 0: 14 Aug
    (21, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 1: 21 Aug
    (28, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 2: 28 Aug
    (11, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 3: 11 Sep
    (15, 8, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 4: 15 Sep
    (18, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 5: 18 Sep (mwc2 hol_prm)
    (25, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 6: 25 Sep
    (29, 8, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 7: 29 Sep
    (2, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 8: 2 Oct
    (13, 9, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 9: 13 Oct
    (16, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 10: 16 Oct
    (23, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 11: 23 Oct
    (30, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 12: 30 Nov
    (6, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 13: 6 Nov
    (20, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 14: 20 Nov
    (27, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 15: 27 Nov
    (4, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 16: 4 Dec
    (11, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 17: 11 Dec
    (18, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 18: 18 Dec
    (5, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 19: 5 Feb (+1 year)
    (12, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 20: 12 Feb (+1 year)
    (19, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 21: 19 Feb (+1 year)
    (26, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 22: 26 Feb (+1 year)
    (4, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 23: 4 Mar (+1 year)
    (11, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 24: 11 Mar (+1 year)
    (18, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 25: 18 Mar (+1 year)
    (25, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 26: 25 Mar (+1 year)
    (1, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 27: 1 Apr (+1 year)
    (8, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 28: 8 Apr (+1 year)
    (15, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 29: 15 Apr (+1 year)
    (22, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 30: 22 Apr (+1 year)
    (29, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 31: 29 Apr (+1 year)
    (7, 4, 1, 6, 2, &[(-1, -1, -1)]), // 32: 7 May (+1 year)
    (14, 4, 1, 6, 2, &[(-1, -1, -1)]), // 33: 14 May (+1 year)
];
pub(crate) const HOL_PRM_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cefa4, row: 0x9d, table: &HOL_PRM_SCHEDULE_ROUNDS, branch: "mwc_2 hol prm schedule" };

/// The Dutch Eredivisie schedule: 34 rounds, from 14 Aug to 14 May; 32 with matches in TV slots.
///
/// `HolPrm::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0x9d and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `HolPrm::create_schedule` (0x5f78d0, hol_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn hol_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &HOL_PRM_SCHEDULE)
}

/// Comparer for `hol_prm_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_hol_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_cfa_schedule("hol_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| hol_prm_create_schedule(this, stage, r, e, l))
}
