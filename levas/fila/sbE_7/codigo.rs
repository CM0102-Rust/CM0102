// ---------- sbE_7: the Polish First and Second Division schedules (pol_first.cpp, pol_second.cpp) ----------

/// Rule: the Polish First Division schedule: each group stage (the main stage −1 and stage 0, the two groups of eight)
/// in 14 weekly rounds from 22 July to 4 November; the final stage (stage 1, the championship and relegation groups) in
/// 14 rounds from 17 November to 18 May of the next year, one of them midweek (16 March); no TV slots.
///
/// `PolFirst::create_schedule(stage, &rounds, &extra, &is_league)`, the stage a signed byte: below 1, is_league (if
/// asked) = 1, rounds = 14 and extra = 0x420 + stage (0x41f for −1, 0x420 for 0), the table of 14 × 0x41 bytes
/// (`exe::crt_malloc`; none: the pol_first.cpp (0xa78598) box at line 0xb2 and 0) with the group rounds; at 1, the same
/// with extra = 0, the final rounds and the box at line 0xf1; above 1, 0 with nothing written. The dates and slots by
/// `fra_third_write_league_rounds`.
/// Original: `PolFirst::create_schedule` (0x7c8df0, pol_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn pol_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    let s = stage as u8 as i8;
    if s > 1 {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0xe;
    let (code, row, table): (i16, u32, &[FraThirdLeagueRound]) = if s < 1 {
        (0x420 + s as i16, 0xb2, &POL_FIRST_GROUP_ROUNDS) // sbE_7: the group stages
    } else {
        (0, 0xf1, &POL_FIRST_FINAL_ROUNDS)
    };
    *extra = code;
    let t = exe::crt_malloc(0xe * 0x41);
    if t == 0 {
        show_error_at(0xa78598, row);
        return 0;
    }
    covered!("sbE_7 polish first division schedule");
    fra_third_write_league_rounds(t, (l.year() as u16) as u32, 0, table);
    t
}

/// Comparer for `pol_first_create_schedule`: side by side (`compare_staged_schedule`, league rounds).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_staged_schedule("pol_first_create_schedule", stage, rounds, extra, league, ScheduleTable::LeagueRounds,
        || o(this, stage, rounds, extra, league), |r, e, lg| pol_first_create_schedule(this, stage, r, e, lg))
}

/// The Polish promotion play-off (stage 0 of the Second Division): one two-legged round of 4 clubs (2 ties) drawn on 28
/// May and played from 3 June of the next year.
pub(crate) const POL_SECOND_PLAY_OFF_ROUND: EngCcCupCupRound = cql10r((0x1c, 4, 1, 6), (3, 5, 1, 5, 1, 0), 0xa0, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));

/// Rule: the Polish Second Division schedule: the season (the main stage) in 34 rounds from 29 July to 27 May of the
/// next year, two of them midweek, or 38 in 2001 (four more midweek rounds); the promotion play-off (stage 0) a single
/// two-legged cup round at the start of June.
///
/// `PolSecond::create_schedule(stage, &rounds, &extra, &is_league)`: at −1, n = 38 when `league::YEAR` is 2001 and 34
/// otherwise; is_league (if asked) = 1, rounds = n, extra = 0, the table of n × 0x41 bytes (`exe::crt_malloc`; none:
/// the pol_second.cpp (0xa7860c) box at line 0xa8 and 0) with the season's rounds (`fra_third_write_league_rounds`; the
/// original's box of line 0x153 for a round count other than n is never reached). At 0: is_league = 0, rounds = 1,
/// extra = 0, the table of 0x68 bytes (the box at line 0x165) with `POL_SECOND_PLAY_OFF_ROUND`
/// (`write_play_off_round`). Any other stage: 0, nothing written.
/// Original: `PolSecond::create_schedule` (0x7cb4a0, pol_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn pol_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    match stage as u8 {
        0xff => {
            let table: &[FraThirdLeagueRound] = if l.year() == 0x7d1 { &POL_SECOND_ROUNDS_2001 } else { &POL_SECOND_ROUNDS }; // sbE_7: 38 rounds in 2001
            if !league.is_null() {
                *league = 1;
            }
            *rounds = table.len() as i16;
            *extra = 0;
            let t = exe::crt_malloc(table.len() as u32 * 0x41);
            if t == 0 {
                show_error_at(0xa7860c, 0xa8);
                return 0;
            }
            covered!("sbE_7 polish second division schedule");
            fra_third_write_league_rounds(t, (l.year() as u16) as u32, 0, table);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0xa7860c, 0x165);
                return 0;
            }
            covered!("sbE_7 polish play-off round");
            write_play_off_round(l, t, 0, &POL_SECOND_PLAY_OFF_ROUND);
            t
        }
        _ => 0,
    }
}

/// Comparer for `pol_second_create_schedule`: side by side (`compare_staged_schedule`; league rounds, the play-off as
/// cup rounds).
pub(crate) unsafe extern "thiscall" fn compare_pol_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    let kind = if stage as u8 == 0 { ScheduleTable::CupRounds } else { ScheduleTable::LeagueRounds };
    compare_staged_schedule("pol_second_create_schedule", stage, rounds, extra, league, kind,
        || o(this, stage, rounds, extra, league), |r, e, lg| pol_second_create_schedule(this, stage, r, e, lg))
}

/// The Polish First Division group stages (-1 and 0): 14 rounds (date, TV slots).
pub(crate) const POL_FIRST_GROUP_ROUNDS: [FraThirdLeagueRound; 14] = [
    (22, 6, 0, 5, 1, &[(-1, -1, -1)]), // 0: 22 Jul
    (29, 6, 0, 5, 1, &[(-1, -1, -1)]), // 1: 29 Jul
    (5, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 5 Aug
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // 3: 12 Aug
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 4: 19 Aug
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 5: 26 Aug
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 9 Sep
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 16 Sep
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 8: 23 Sep
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 9: 30 Sep
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 14 Oct
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 21 Oct
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 12: 28 Oct
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 4 Nov
];

/// The Polish First Division final stage (1): 14 rounds (date, TV slots).
pub(crate) const POL_FIRST_FINAL_ROUNDS: [FraThirdLeagueRound; 14] = [
    (17, 10, 0, 5, 1, &[(-1, -1, -1)]), // 0: 17 Nov
    (24, 10, 0, 5, 1, &[(-1, -1, -1)]), // 1: 24 Nov
    (2, 2, 1, 5, 1, &[(-1, -1, -1)]), // 2: 2 Mar (+1 year)
    (9, 2, 1, 5, 1, &[(-1, -1, -1)]), // 3: 9 Mar (+1 year)
    (16, 2, 1, 2, 2, &[(-1, -1, -1)]), // 4: 16 Mar (+1 year)
    (23, 2, 1, 5, 1, &[(-1, -1, -1)]), // 5: 23 Mar (+1 year)
    (30, 2, 1, 5, 1, &[(-1, -1, -1)]), // 6: 30 Mar (+1 year)
    (6, 3, 1, 5, 1, &[(-1, -1, -1)]), // 7: 6 Apr (+1 year)
    (13, 3, 1, 5, 1, &[(-1, -1, -1)]), // 8: 13 Apr (+1 year)
    (20, 3, 1, 5, 1, &[(-1, -1, -1)]), // 9: 20 Apr (+1 year)
    (27, 3, 1, 5, 1, &[(-1, -1, -1)]), // 10: 27 Apr (+1 year)
    (4, 4, 1, 5, 1, &[(-1, -1, -1)]), // 11: 4 May (+1 year)
    (11, 4, 1, 5, 1, &[(-1, -1, -1)]), // 12: 11 May (+1 year)
    (18, 4, 1, 5, 1, &[(-1, -1, -1)]), // 13: 18 May (+1 year)
];

/// The Polish Second Division in 2001: 38 rounds (date, TV slots).
pub(crate) const POL_SECOND_ROUNDS_2001: [FraThirdLeagueRound; 38] = [
    (29, 6, 0, 5, 1, &[(-1, -1, -1)]), // 0: 29 Jul
    (5, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 5 Aug
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 12 Aug
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 3: 19 Aug
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 4: 26 Aug
    (2, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 2 Sep
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 9 Sep
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 16 Sep
    (20, 8, 0, 2, 2, &[(-1, -1, -1)]), // 8: 20 Sep
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 9: 30 Sep
    (4, 9, 0, 2, 2, &[(-1, -1, -1)]), // 10: 4 Oct
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 14 Oct
    (18, 9, 0, 2, 2, &[(-1, -1, -1)]), // 12: 18 Oct
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 13: 21 Oct
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 14: 28 Oct
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 4 Nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 16: 11 Nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 17: 18 Nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 18: 25 Nov
    (19, 1, 1, 5, 1, &[(-1, -1, -1)]), // 19: 19 Feb (+1 year)
    (23, 1, 1, 2, 2, &[(-1, -1, -1)]), // 20: 23 Feb (+1 year)
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 21: 26 Feb (+1 year)
    (1, 2, 1, 2, 2, &[(-1, -1, -1)]), // 22: 1 Mar (+1 year)
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 4 Mar (+1 year)
    (11, 2, 1, 5, 1, &[(-1, -1, -1)]), // 24: 11 Mar (+1 year)
    (15, 2, 1, 2, 2, &[(-1, -1, -1)]), // 25: 15 Mar (+1 year)
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 26: 18 Mar (+1 year)
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 25 Mar (+1 year)
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 28: 1 Apr (+1 year)
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 29: 8 Apr (+1 year)
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 15 Apr (+1 year)
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 31: 22 Apr (+1 year)
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 29 Apr (+1 year)
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 33: 6 May (+1 year)
    (13, 4, 1, 5, 1, &[(-1, -1, -1)]), // 34: 13 May (+1 year)
    (20, 4, 1, 5, 1, &[(-1, -1, -1)]), // 35: 20 May (+1 year)
    (24, 4, 1, 2, 2, &[(-1, -1, -1)]), // 36: 24 May (+1 year)
    (27, 4, 1, 5, 1, &[(-1, -1, -1)]), // 37: 27 May (+1 year)
];

/// The Polish Second Division from 2002: 34 rounds (date, TV slots).
pub(crate) const POL_SECOND_ROUNDS: [FraThirdLeagueRound; 34] = [
    (29, 6, 0, 5, 1, &[(-1, -1, -1)]), // 0: 29 Jul
    (5, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 5 Aug
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 12 Aug
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 3: 19 Aug
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 4: 26 Aug
    (2, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 2 Sep
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 9 Sep
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 16 Sep
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 8: 30 Sep
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 14 Oct
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 21 Oct
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 28 Oct
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 4 Nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 11 Nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 18 Nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 25 Nov
    (19, 1, 1, 5, 1, &[(-1, -1, -1)]), // 16: 19 Feb (+1 year)
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 17: 26 Feb (+1 year)
    (1, 2, 1, 2, 2, &[(-1, -1, -1)]), // 18: 1 Mar (+1 year)
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 19: 4 Mar (+1 year)
    (11, 2, 1, 5, 1, &[(-1, -1, -1)]), // 20: 11 Mar (+1 year)
    (15, 2, 1, 2, 2, &[(-1, -1, -1)]), // 21: 15 Mar (+1 year)
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 22: 18 Mar (+1 year)
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 25 Mar (+1 year)
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 24: 1 Apr (+1 year)
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 25: 8 Apr (+1 year)
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 26: 15 Apr (+1 year)
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 27: 22 Apr (+1 year)
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 28: 29 Apr (+1 year)
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 29: 6 May (+1 year)
    (13, 4, 1, 5, 1, &[(-1, -1, -1)]), // 30: 13 May (+1 year)
    (20, 4, 1, 5, 1, &[(-1, -1, -1)]), // 31: 20 May (+1 year)
    (24, 4, 1, 2, 2, &[(-1, -1, -1)]), // 32: 24 May (+1 year)
    (27, 4, 1, 5, 1, &[(-1, -1, -1)]), // 33: 27 May (+1 year)
];
