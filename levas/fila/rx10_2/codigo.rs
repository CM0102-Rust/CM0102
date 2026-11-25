// ---------- rx10_2: the Football Conference schedule (eng_conf.cpp) ----------

/// The Conference season: 21 rounds, the two Christmas rounds chosen by the weekday (`eng_second_set_christmas_rounds`:
/// the day of round 20 against 1 December, minus 16) and 19 rounds after; the box at line 0x13d outside the seven cases.
pub(crate) const CONFERENCE_CHRISTMAS_SCHEDULE: EngSecondChristmasSchedule = EngSecondChristmasSchedule { comeco: &CONFERENCE_START, reference: 20, adjust: 16,
    cases: [&CONFERENCE_CHRISTMAS_0, &CONFERENCE_CHRISTMAS_1, &CONFERENCE_CHRISTMAS_2, &CONFERENCE_CHRISTMAS_3, &CONFERENCE_CHRISTMAS_4, &CONFERENCE_CHRISTMAS_5,
        &CONFERENCE_CHRISTMAS_6, &CONFERENCE_CHRISTMAS_7], end: &CONFERENCE_LATE_ROUNDS, row: 0x13d };

/// The Conference play-off (stage 0), first round: 4 clubs in two-legged ties, drawn on 7 May, from 12 May (code 0x82).
pub(crate) const CONFERENCE_PLAYOFF_0: EngCcCupCupRound = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
/// The Conference play-off, second round: 2 clubs, two legs, drawn on 17 May, from 19 May (code 0x96).
pub(crate) const CONFERENCE_PLAYOFF_1: EngCcCupCupRound = cql10r((0x11, 4, 1, 3), (0x13, 4, 1, 5, 1, 0), 0x96, (0, 0x204, 3), 5, (2, 1, 0, 0), (0, 2, 4), (0, 0, 0));
/// The Conference play-off, third round: a single match on 26 May at venue 4 [inference: the neutral ground] (code
/// 0x168); no prizes in any round.
pub(crate) const CONFERENCE_PLAYOFF_2: EngCcCupCupRound = cql10r((0x18, 4, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x168, (0, 3, 0), 5, (2, 1, 1, 4), (0, 1, 0), (0, 0, 0));

/// Rule: the Football Conference schedule: 42 rounds (22 clubs) from 15 August to 1 May, on weekday 5 [inference: Saturday] with
/// midweek rounds (code 2); at Christmas two rounds (21 to 30 December) chosen by the day the 19 December round falls
/// on. Stage 0 is a three-round play-off in May, which the Conference rules never ask for (no play-off places).
///
/// `EngConf::create_schedule(stage, &rounds, &extra, &league)` (virtual `comp_vtable::CREATE_SCHEDULE`). Stage −1
/// (byte): league = 1 (if asked), 42 rounds, extra 0; the table (`exe::crt_malloc`(0xaaa); none: the box at line 0xb1
/// and 0) by `eng_second_set_christmas_rounds` with `CONFERENCE_CHRISTMAS_SCHEDULE` and the year (word). Stage 0:
/// league = 0, 3 rounds, extra 0xa0; the table (0x138; none: the box at line 0x19f and 0) with the three cup rounds
/// (`eng_cc_cup_write_cup_round`, the year signed). Other stages: 0 with nothing written.
/// Original: `EngConf::create_schedule` (0x56de60, eng_conf.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn eng_conf_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 42;
            *extra = 0;
            let t = exe::crt_malloc(0xaaa);
            if t == 0 {
                show_error_at(0x9c4294, 0xb1);
                return 0;
            }
            covered!("rx10_2 conference calendar");
            eng_second_set_christmas_rounds(t, this.year() as u16 as u32, &CONFERENCE_CHRISTMAS_SCHEDULE, 0x9c4294);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 3;
            *extra = 0xa0;
            let t = exe::crt_malloc(0x138);
            if t == 0 {
                show_error_at(0x9c4294, 0x19f);
                return 0;
            }
            covered!("rx10_2 conference play-off");
            let year = this.year() as i32 as u32;
            for (i, r) in [&CONFERENCE_PLAYOFF_0, &CONFERENCE_PLAYOFF_1, &CONFERENCE_PLAYOFF_2].iter().enumerate() {
                eng_cc_cup_write_cup_round(t, i as u32, r, year);
            }
            t
        }
        _ => 0,
    }
}

/// Comparer for `eng_conf_create_schedule`: side by side (`compare_league_round_table`: the outputs and the bytes the
/// rounds write).
pub(crate) unsafe extern "thiscall" fn compare_eng_conf_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("eng_conf_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| eng_conf_create_schedule(this, stage, r, e, l))
}

/// The Conference: the rounds before Christmas (stage −1).
pub(crate) const CONFERENCE_START: [EngSecondScheduleRound; 21] = [
    (15, 7, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 0: 15 Aug
    (18, 7, 0, 1, 2, &[(-1, -1, -1)]), // 1: 18 Aug
    (22, 7, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 2: 22 Aug
    (26, 7, 0, 2, 1, &[(-1, -1, -1)]), // 3: 26 Aug
    (29, 7, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 4: 29 Aug
    (31, 7, 0, 0, 2, &[(-1, -1, -1)]), // 5: 31 Aug
    (5, 8, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 6: 5 Sep
    (8, 8, 0, 1, 2, &[(-1, -1, -1)]), // 7: 8 Sep
    (12, 8, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 8: 12 Sep
    (19, 8, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 9: 19 Sep
    (22, 8, 0, 1, 2, &[(-1, -1, -1)]), // 10: 22 Sep
    (26, 8, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 11: 26 Sep
    (3, 9, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 12: 3 Oct
    (10, 9, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 13: 10 Oct
    (24, 9, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 14: 24 Oct
    (7, 10, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 15: 7 Nov
    (14, 10, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 16: 14 Nov
    (28, 10, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 17: 28 Nov
    (5, 11, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 18: 5 Dec
    (12, 11, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 19: 12 Dec
    (19, 11, 0, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 20: 19 Dec
];
/// The Conference: the two Christmas rounds of case 0.
pub(crate) const CONFERENCE_CHRISTMAS_0: [EngSecondScheduleRound; 2] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 23 Dec
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26 Dec
];
/// The Conference: the two Christmas rounds of case 1.
pub(crate) const CONFERENCE_CHRISTMAS_1: [EngSecondScheduleRound; 2] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 21 Dec
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26 Dec
];
/// The Conference: the two Christmas rounds of case 2.
pub(crate) const CONFERENCE_CHRISTMAS_2: [EngSecondScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 26 Dec
    (29, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 29 Dec
];
/// The Conference: the two Christmas rounds of case 3.
pub(crate) const CONFERENCE_CHRISTMAS_3: [EngSecondScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 26 Dec
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 28 Dec
];
/// The Conference: the two Christmas rounds of case 4.
pub(crate) const CONFERENCE_CHRISTMAS_4: [EngSecondScheduleRound; 2] = [
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 27 Dec
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 30 Dec
];
/// The Conference: the two Christmas rounds of case 5.
pub(crate) const CONFERENCE_CHRISTMAS_5: [EngSecondScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 26 Dec
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 28 Dec
];
/// The Conference: the two Christmas rounds of case 6.
pub(crate) const CONFERENCE_CHRISTMAS_6: [EngSecondScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 26 Dec
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 29 Dec
];
/// The Conference: the two Christmas rounds of case 7 (the default, after the box).
pub(crate) const CONFERENCE_CHRISTMAS_7: [EngSecondScheduleRound; 2] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 21: 26 Dec
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 22: 28 Dec
];
/// The Conference: the rounds after Christmas.
pub(crate) const CONFERENCE_LATE_ROUNDS: [EngSecondScheduleRound; 19] = [
    (2, 0, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 23: 2 Jan (+1 year)
    (9, 0, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 24: 9 Jan (+1 year)
    (16, 0, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 25: 16 Jan (+1 year)
    (23, 0, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 26: 23 Jan (+1 year)
    (30, 0, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 27: 30 Jan (+1 year)
    (6, 1, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 28: 6 Feb (+1 year)
    (13, 1, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 29: 13 Feb (+1 year)
    (20, 1, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 30: 20 Feb (+1 year)
    (27, 1, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 31: 27 Feb (+1 year)
    (6, 2, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 32: 6 Mar (+1 year)
    (9, 2, 1, 1, 2, &[(-1, -1, -1)]), // 33: 9 Mar (+1 year)
    (13, 2, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 34: 13 Mar (+1 year)
    (20, 2, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 35: 20 Mar (+1 year)
    (27, 2, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 36: 27 Mar (+1 year)
    (3, 3, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 37: 3 Apr (+1 year)
    (5, 3, 1, 0, 2, &[(-1, -1, -1)]), // 38: 5 Apr (+1 year)
    (10, 3, 1, 5, 1, &[(5, 0, 2), (-1, -1, -1)]), // 39: 10 Apr (+1 year)
    (24, 3, 1, 5, 1, &[(-1, -1, -1)]), // 40: 24 Apr (+1 year)
    (1, 4, 1, 5, 1, &[(-1, -1, -1)]), // 41: 1 May (+1 year)
];
