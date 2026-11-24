// ---------- cx08_14: the Belgian Second Division schedule (bel_second.cpp) ----------

/// The Belgian Second Division schedule: the season (the main stage) in 34 rounds from 23 August to 6 May of the next
/// year, with no TV slots; the promotion play-off group (stage 3) in 6 rounds from 13 to 31 May; the play-off cup (stage
/// 4) in three single-leg rounds in May (draws on 8, 15 and 22 May, matches from 14, 21 and 28 May).
///
/// `BelSecond::create_schedule(stage, &rounds, &extra, &is_league)`. Stage −1: is_league (if asked) = 1, rounds = 34,
/// extra = 0, the league table of 34 × 0x41 bytes (box of line 0xb1). Stage 3: is_league = 1, rounds = 6, extra =
/// 0xa0, 6 league rounds (box 0x12b). Stage 4: is_league = 0, rounds = 3, extra = 0x452, the cup table of 3 × 0x68
/// bytes (box 0x151). Any other stage: 0 and nothing written. The tables come from `exe::crt_malloc` (out of memory:
/// the bel_second.cpp box and 0).
/// Original: `BelSecond::create_schedule` (0x41f340, bel_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let (is_league, n, code, row): (i32, i16, i16, u32) = match stage as u8 {
        0xff => (1, 0x22, 0, 0xb1),
        3 => (1, 6, 0xa0, 0x12b),
        4 => (0, 3, 0x452, 0x151),
        _ => return 0,
    };
    if !league.is_null() {
        *league = is_league;
    }
    *rounds = n;
    *extra = code;
    let t = exe::crt_malloc(n as u32 * if is_league != 0 { 0x41 } else { cup_round::SIZE as u32 });
    if t == 0 {
        show_error_at(0x989b54, row);
        return 0;
    }
    covered!("cx08_14 belgian second schedule");
    let year = this.year();
    match stage as u8 {
        0xff => fra_third_write_league_rounds(t, year as u16 as u32, 0, &BEL_SECOND_SEASON_ROUNDS),
        3 => fra_third_write_league_rounds(t, year as u16 as u32, 0, &BEL_SECOND_PLAYOFF_GROUP_ROUNDS),
        _ => {
            for (i, r) in BEL_SECOND_PLAYOFF_CUP_ROUNDS.iter().enumerate() {
                eng_cc_cup_write_cup_round(t, i as u32, r, year as i32 as u32);
            }
        }
    }
    t
}

/// Comparer for `bel_second_create_schedule`: side by side (`compare_staged_schedule`; the cup rounds at stage 4).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    let kind = if stage as u8 == 4 { ScheduleTable::CupRounds } else { ScheduleTable::LeagueRounds };
    compare_staged_schedule("bel_second_create_schedule", stage, rounds, extra, league, kind,
        || o(this, stage, rounds, extra, league), |r, e, l| bel_second_create_schedule(this, stage, r, e, l))
}

/// The play-off cup (stage 4): three single-leg rounds (6, 4 and 2 clubs), no prizes.
pub(crate) const BEL_SECOND_PLAYOFF_CUP_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((8, 4, 1, 0), (14, 4, 1, 6, 2, 0x28), 0x78, (0, 3, 0), 5, (6, 3, 6, 0), (0, 1, 0), (0, 0, 0)),
    cql10r((15, 4, 1, 0), (21, 4, 1, 6, 2, 0x28), 0x82, (0, 3, 0), 5, (4, 2, 1, 6), (0, 1, 0), (0, 0, 0)),
    cql10r((22, 4, 1, 0), (28, 4, 1, 6, 2, 0x28), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0)),
];

/// The season: 34 rounds (date, TV slots).
pub(crate) const BEL_SECOND_SEASON_ROUNDS: [FraThirdLeagueRound; 34] = [
    (23, 7, 0, 2, 2, &[(-1, -1, -1)]), // 0: 23 Aug
    (30, 7, 0, 2, 2, &[(-1, -1, -1)]), // 1: 30 Aug
    (3, 8, 0, 6, 1, &[(-1, -1, -1)]), // 2: 3 Sep
    (10, 8, 0, 6, 1, &[(-1, -1, -1)]), // 3: 10 Sep
    (17, 8, 0, 6, 1, &[(-1, -1, -1)]), // 4: 17 Sep
    (24, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 24 Sep
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]), // 6: 1 Oct
    (8, 9, 0, 6, 1, &[(-1, -1, -1)]), // 7: 8 Oct
    (15, 9, 0, 6, 1, &[(-1, -1, -1)]), // 8: 15 Oct
    (22, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 22 Oct
    (29, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 29 Oct
    (12, 10, 0, 6, 1, &[(-1, -1, -1)]), // 11: 12 Nov
    (19, 10, 0, 6, 1, &[(-1, -1, -1)]), // 12: 19 Nov
    (26, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 26 Nov
    (3, 11, 0, 6, 1, &[(-1, -1, -1)]), // 14: 3 Dec
    (10, 11, 0, 6, 1, &[(-1, -1, -1)]), // 15: 10 Dec
    (17, 11, 0, 6, 1, &[(-1, -1, -1)]), // 16: 17 Dec
    (7, 0, 1, 6, 1, &[(-1, -1, -1)]), // 17: 7 Jan (+1 year)
    (14, 0, 1, 6, 1, &[(-1, -1, -1)]), // 18: 14 Jan (+1 year)
    (21, 0, 1, 6, 1, &[(-1, -1, -1)]), // 19: 21 Jan (+1 year)
    (28, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 28 Jan (+1 year)
    (4, 1, 1, 6, 1, &[(-1, -1, -1)]), // 21: 4 Feb (+1 year)
    (11, 1, 1, 6, 1, &[(-1, -1, -1)]), // 22: 11 Feb (+1 year)
    (18, 1, 1, 6, 1, &[(-1, -1, -1)]), // 23: 18 Feb (+1 year)
    (25, 1, 1, 6, 1, &[(-1, -1, -1)]), // 24: 25 Feb (+1 year)
    (4, 2, 1, 6, 1, &[(-1, -1, -1)]), // 25: 4 Mar (+1 year)
    (11, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26: 11 Mar (+1 year)
    (18, 2, 1, 6, 1, &[(-1, -1, -1)]), // 27: 18 Mar (+1 year)
    (25, 2, 1, 6, 1, &[(-1, -1, -1)]), // 28: 25 Mar (+1 year)
    (8, 3, 1, 6, 1, &[(-1, -1, -1)]), // 29: 8 Apr (+1 year)
    (15, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 15 Apr (+1 year)
    (22, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 22 Apr (+1 year)
    (29, 3, 1, 6, 1, &[(-1, -1, -1)]), // 32: 29 Apr (+1 year)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 6 May (+1 year)
];

/// The promotion play-off group: 6 rounds (date, TV slots).
pub(crate) const BEL_SECOND_PLAYOFF_GROUP_ROUNDS: [FraThirdLeagueRound; 6] = [
    (13, 4, 1, 6, 2, &[(-1, -1, -1)]), // 0: 13 May (+1 year)
    (17, 4, 1, 3, 2, &[(-1, -1, -1)]), // 1: 17 May (+1 year)
    (20, 4, 1, 6, 2, &[(-1, -1, -1)]), // 2: 20 May (+1 year)
    (24, 4, 1, 3, 2, &[(-1, -1, -1)]), // 3: 24 May (+1 year)
    (27, 4, 1, 6, 2, &[(-1, -1, -1)]), // 4: 27 May (+1 year)
    (31, 4, 1, 3, 2, &[(-1, -1, -1)]), // 5: 31 May (+1 year)
];
