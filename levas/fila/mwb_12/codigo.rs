// ---------- mwb_12: the League of Wales (wel_first.cpp): the schedule ----------

/// The League of Wales before the year end: 16 rounds from 19 August to 16 December (4 October on a Wednesday, code 2;
/// none on 7 and 14 October and 18 November).
pub(crate) const LEAGUE_OF_WALES_BEFORE: [EngFirstScheduleRound; 16] = [
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 19/ago
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 26/ago
    (2, 8, 0, 5, 1, &[(-1, -1, -1)]), // 2: 2/set
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 3: 9/set
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 16/set
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 23/set
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 30/set
    (4, 9, 0, 2, 2, &[(-1, -1, -1)]), // 7: 4/out
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: 21/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 28/out
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 10: 4/nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 11: 11/nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 25/nov, mwb12: a League of Wales round
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 13: 2/dez
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 14: 9/dez
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 15: 16/dez
];

/// The League of Wales year-end rounds when the reference round falls on 13 December (case 0).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_0: [EngFirstScheduleRound; 2] = [
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 30/dez
];

/// The League of Wales year-end rounds when the reference round falls on 14 December (case 1).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_1: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 28/dez
];

/// The League of Wales year-end rounds when the reference round falls on 15 December (case 2).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_2: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 29/dez
];

/// The League of Wales year-end rounds when the reference round falls on 16 December (case 3).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_3: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 30/dez
];

/// The League of Wales year-end rounds when the reference round falls on 17 December (case 4).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_4: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 31/dez
];

/// The League of Wales year-end rounds when the reference round falls on 18 December (case 5).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_5: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 28/dez
];

/// The League of Wales year-end rounds when the reference round falls on 19 December (case 6).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_6: [EngFirstScheduleRound; 2] = [
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 17: 2/jan
];

/// The League of Wales year-end rounds out of range (after the error box): 28 December on Sunday and 2 January.
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_DEFAULT: [EngFirstScheduleRound; 2] = [
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 16: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 17: 2/jan
];

/// The League of Wales after the year end: 16 rounds from 6 January to 28 April of the next year (21 February and 17
/// April midweek, code 2).
pub(crate) const LEAGUE_OF_WALES_AFTER: [EngFirstScheduleRound; 16] = [
    (6, 0, 1, 5, 1, &[(-1, -1, -1)]), // 18: 6/jan
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 19: 13/jan
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 20: 20/jan
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 21: 27/jan
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 22: 3/fev
    (17, 1, 1, 5, 1, &[(-1, -1, -1)]), // 23: 17/fev
    (21, 1, 1, 2, 2, &[(-1, -1, -1)]), // 24: 21/fev
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 25: 24/fev
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 26: 3/mar
    (17, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 17/mar
    (24, 2, 1, 5, 1, &[(-1, -1, -1)]), // 28: 24/mar
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 31/mar
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 14/abr
    (17, 3, 1, 1, 2, &[(-1, -1, -1)]), // 31: 17/abr
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 21/abr
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 28/abr
];

/// The League of Wales calendar with its year-end block chosen by the day the 16th round (index 15) falls on (k = day
/// − 1 December + 1 − 13: 0 to 6; the default after the box of line 0x12e).
pub(crate) const LEAGUE_OF_WALES_SCHEDULE: EngFirstChristmasSchedule = EngFirstChristmasSchedule {
    comeco: &LEAGUE_OF_WALES_BEFORE,
    reference: 15,
    adjust: 0xd,
    cases: [&LEAGUE_OF_WALES_CHRISTMAS_0, &LEAGUE_OF_WALES_CHRISTMAS_1, &LEAGUE_OF_WALES_CHRISTMAS_2, &LEAGUE_OF_WALES_CHRISTMAS_3,
            &LEAGUE_OF_WALES_CHRISTMAS_4, &LEAGUE_OF_WALES_CHRISTMAS_5, &LEAGUE_OF_WALES_CHRISTMAS_6, &LEAGUE_OF_WALES_CHRISTMAS_DEFAULT],
    end: &LEAGUE_OF_WALES_AFTER,
    row: 0x12e,
};

/// rule: the League of Wales plays 34 rounds (18 clubs, 2 rounds) from 19 August to 28 April of the next year, mostly on
/// Saturdays; around the year end two rounds chosen by the day the 16th round falls on (26 to 31 December and 2
/// January); no TV slots; only the main stage (`league` = 1, `extra` = 0; out of memory, the box of line 0xa1 and 0;
/// any other stage: nothing and 0; `eng_first_set_christmas_rounds`)
/// Original: `WelFirst::create_schedule` (0x9250f0, wel_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x22;
    *extra = 0;
    let t = exe::crt_malloc(0x8a2);
    if t == 0 {
        show_error_at(0xad56b8, 0xa1);
        return 0;
    }
    covered!("mwb_12 league of wales calendar");
    eng_first_set_christmas_rounds(t, (this.year() as u16) as u32, &LEAGUE_OF_WALES_SCHEDULE, 0xad56b8);
    t
}

/// Comparer for `wel_first_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_wel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("wel_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| wel_first_create_schedule(this, stage, r, e, l))
}
