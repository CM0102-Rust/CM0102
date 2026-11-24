// ---------- cx08_7: the Argentine Second Division schedule (arg_second.cpp) ----------

/// The Argentine Second Division schedule: the first phase (the main stage) in 25 rounds from 7 August to 18 December,
/// weekly on weekday 5 with one TV slot and six midweek rounds; the three zones (stages 0 to 2) share 14 rounds from 2
/// February to 13 April of the next year; the promotion cup (stage 3) three two-legged rounds (quarter-finals drawn
/// 14 April and played from 20 April, semi-finals 27 April, final from 4 May); the two promotion play-offs (stages 4
/// and 5) one two-legged tie each, from 15 May.
///
/// `ArgSecond::create_schedule(stage, &rounds, &extra, &is_league)`. Stage −1: is_league (if asked) = 1, rounds = 25
/// (extra not written), the league table of 25 × 0x41 bytes (box of line 0xb1). Any other stage below 3 (signed): is_league
/// = 1, rounds = 14, extra = 0x3e8 + the stage, the 14 league rounds (box 0x148). Stage 3: is_league = 0, rounds = 3,
/// extra = 0x45a, the cup table of 3 × 0x68 bytes (box 0x1a3). Stages 4 and 5: is_league = 0, rounds = 1, extra 0x45b or
/// 0x45c, one cup round (boxes 0x1f1 and 0x219). Any other stage: 0 and nothing written. The tables come from
/// `exe::crt_malloc` (out of memory: the arg_second.cpp box and 0); league rounds by `fra_third_write_league_rounds`, cup
/// rounds by `eng_cc_cup_write_cup_round` with the year sign-extended.
/// Original: `ArgSecond::create_schedule` (0x406f00, arg_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn arg_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let s = stage as u8 as i8;
    let (is_league, n, row): (i32, i16, u32) = match s {
        -1 => (1, 0x19, 0xb1),
        3 => (0, 3, 0x1a3),
        4 => (0, 1, 0x1f1),
        5 => (0, 1, 0x219),
        _ if s < 3 => (1, 0xe, 0x148),
        _ => return 0,
    };
    if !league.is_null() {
        *league = is_league;
    }
    *rounds = n;
    match s {
        -1 => {}
        3 => *extra = 0x45a,
        4 => *extra = 0x45b,
        5 => *extra = 0x45c,
        _ => *extra = 0x3e8 + s as i16,
    }
    let t = exe::crt_malloc(n as u32 * if is_league != 0 { 0x41 } else { cup_round::SIZE as u32 });
    if t == 0 {
        show_error_at(ARG_SECOND_FILE, row);
        return 0;
    }
    covered!("cx08_7 argentine second schedule");
    let year = this.year();
    match s {
        -1 => fra_third_write_league_rounds(t, year as u16 as u32, 0, &ARG_SECOND_FIRST_PHASE_ROUNDS),
        3 => {
            for (i, r) in ARG_SECOND_PROMOTION_CUP_ROUNDS.iter().enumerate() {
                eng_cc_cup_write_cup_round(t, i as u32, r, year as i32 as u32);
            }
        }
        4 | 5 => eng_cc_cup_write_cup_round(t, 0, &ARG_SECOND_PLAYOFF_ROUND, year as i32 as u32),
        _ => fra_third_write_league_rounds(t, year as u16 as u32, 0, &ARG_SECOND_ZONE_ROUNDS),
    }
    t
}

/// Comparer for `arg_second_create_schedule`: side by side (`compare_staged_schedule`; the cup rounds at stages 3 to 5).
pub(crate) unsafe extern "thiscall" fn compare_arg_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    let kind = if matches!(stage as u8, 3..=5) { ScheduleTable::CupRounds } else { ScheduleTable::LeagueRounds };
    compare_staged_schedule("arg_second_create_schedule", stage, rounds, extra, league, kind,
        || o(this, stage, rounds, extra, league), |r, e, l| arg_second_create_schedule(this, stage, r, e, l))
}

/// The promotion cup (stage 3): the quarter-finals, the semi-finals and the final, two-legged, no prizes.
pub(crate) const ARG_SECOND_PROMOTION_CUP_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((14, 3, 1, 6), (20, 3, 1, 5, 1, 0), 0x78, (0, 0xa00, 0x804), 8, (8, 4, 8, 0), (0, 2, 3), (0, 0, 0)),
    cql10r((24, 3, 1, 2), (27, 3, 1, 5, 1, 0), 0x82, (0, 0, 0x804), 8, (4, 2, 0, 0), (0, 2, 3), (0, 0, 0)),
    cql10r((1, 4, 1, 2), (4, 4, 1, 5, 1, 0), 0x96, (0, 0, 0x804), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0, 0)),
];

/// A promotion play-off (stages 4 and 5): one two-legged tie between two clubs, from 15 May.
pub(crate) const ARG_SECOND_PLAYOFF_ROUND: EngCcCupCupRound = cql10r((12, 4, 1, 6), (15, 4, 1, 2, 2, 0), 0, (0, 0x200, 0x804), 6, (2, 1, 2, 0), (0, 2, 4), (0, 0, 0));

/// The first phase: 25 rounds (date, TV slots).
pub(crate) const ARG_SECOND_FIRST_PHASE_ROUNDS: [FraThirdLeagueRound; 25] = [
    (7, 7, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 0: 7 Aug
    (14, 7, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 1: 14 Aug
    (21, 7, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 2: 21 Aug
    (24, 7, 0, 1, 2, &[(-1, -1, -1)]), // 3: 24 Aug
    (28, 7, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 4: 28 Aug
    (4, 8, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 5: 4 Sep
    (11, 8, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 6: 11 Sep
    (14, 8, 0, 1, 2, &[(-1, -1, -1)]), // 7: 14 Sep
    (18, 8, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 8: 18 Sep
    (25, 8, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 9: 25 Sep
    (29, 8, 0, 1, 2, &[(-1, -1, -1)]), // 10: 29 Sep
    (2, 9, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 11: 2 Oct
    (9, 9, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 12: 9 Oct
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 13: 12 Oct
    (16, 9, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 14: 16 Oct
    (23, 9, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 15: 23 Oct
    (26, 9, 0, 1, 2, &[(-1, -1, -1)]), // 16: 26 Oct
    (30, 9, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 17: 30 Oct
    (6, 10, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 18: 6 Nov
    (13, 10, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 19: 13 Nov
    (20, 10, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 20: 20 Nov
    (27, 10, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 21: 27 Nov
    (4, 11, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 22: 4 Dec
    (11, 11, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 23: 11 Dec
    (18, 11, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 24: 18 Dec
];

/// The zones: 14 rounds (date, TV slots).
pub(crate) const ARG_SECOND_ZONE_ROUNDS: [FraThirdLeagueRound; 14] = [
    (2, 1, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 0: 2 Feb (+1 year)
    (9, 1, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 1: 9 Feb (+1 year)
    (12, 1, 1, 1, 2, &[(-1, -1, -1)]), // 2: 12 Feb (+1 year)
    (16, 1, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 3: 16 Feb (+1 year)
    (23, 1, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 4: 23 Feb (+1 year)
    (2, 2, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 5: 2 Mar (+1 year)
    (9, 2, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 6: 9 Mar (+1 year)
    (12, 2, 1, 1, 2, &[(-1, -1, -1)]), // 7: 12 Mar (+1 year)
    (16, 2, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 8: 16 Mar (+1 year)
    (23, 2, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 9: 23 Mar (+1 year)
    (30, 2, 1, 5, 1, &[(-1, -1, -1)]), // 10: 30 Mar (+1 year)
    (2, 3, 1, 1, 2, &[(-1, -1, -1)]), // 11: 2 Apr (+1 year)
    (6, 3, 1, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 12: 6 Apr (+1 year)
    (13, 3, 1, 5, 1, &[(-1, -1, -1)]), // 13: 13 Apr (+1 year)
];
