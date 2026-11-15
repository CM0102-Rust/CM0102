// ---------- mwb_14: the Scottish Premier Division (sco_prm.cpp): the schedule ----------

/// The main-stage schedule of the Scottish Premier Division: the rounds before the year end, the seven year-end blocks
/// chosen by the day the 21st round (index 20) falls on (k = that day − 1 December + 1 − 13) and the default after the
/// box, the index the rounds after the year end start at (23 in both seasons: in 1999 the cases write round 21 only and
/// round 22 stays as malloc left it, a defect of the original), the rounds after, the number of rounds, the error-box
/// lines (out of memory, the case out of range).
pub(crate) struct ScottishPremierSchedule {
    pub(crate) before: &'static [EngFirstScheduleRound],
    pub(crate) cases: [&'static [EngFirstScheduleRound]; 7],
    pub(crate) default: &'static [EngFirstScheduleRound],
    pub(crate) after_at: i32,
    pub(crate) after: &'static [EngFirstScheduleRound],
    pub(crate) rounds: i16,
    pub(crate) rows: (u32, u32),
}

/// The Scottish Premier Division (1999) before the year end.
pub(crate) const SCOTTISH_PREMIER_1999_BEFORE: [EngFirstScheduleRound; 21] = [
    (29, 6, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 0: 29/jul
    (5, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 1: 5/ago
    (12, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 2: 12/ago
    (19, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 3: 19/ago
    (26, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 4: 26/ago
    (9, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 5: 9/set
    (16, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 6: 16/set
    (23, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 7: 23/set
    (30, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 8: 30/set
    (14, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 9: 14/out
    (17, 9, 0, 1, 2, &[(-1, -1, -1)]), // 10: 17/out
    (21, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 11: 21/out
    (28, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 12: 28/out
    (4, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 13: 4/nov
    (11, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 14: 11/nov
    (18, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 15: 18/nov
    (25, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 16: 25/nov
    (29, 10, 0, 2, 2, &[(-1, -1, -1)]), // 17: 29/nov
    (2, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 18: 2/dez
    (9, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 19: 9/dez
    (16, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 20: 16/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 13 December (case 0).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_0: [EngFirstScheduleRound; 1] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 20/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 14 December (case 1).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_1: [EngFirstScheduleRound; 1] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 21/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 15 December (case 2).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_2: [EngFirstScheduleRound; 1] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 22/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 16 December (case 3).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_3: [EngFirstScheduleRound; 1] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 23/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 17 December (case 4).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_4: [EngFirstScheduleRound; 1] = [
    (20, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 20/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 18 December (case 5).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_5: [EngFirstScheduleRound; 1] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 21/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 19 December (case 6).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_6: [EngFirstScheduleRound; 1] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 22/dez
];

/// The Scottish Premier Division (1999) year-end rounds out of range (after the error box).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_DEFAULT: [EngFirstScheduleRound; 2] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 22/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division (1999) after the year end.
pub(crate) const SCOTTISH_PREMIER_1999_AFTER: [EngFirstScheduleRound; 13] = [
    (2, 0, 1, 1, 2, &[(-1, -1, -1)]), // 23: 2/jan
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 24: 20/jan
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 25: 27/jan
    (3, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 26: 3/fev
    (10, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 27: 10/fev
    (24, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 28: 24/fev
    (3, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 29: 3/mar
    (17, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 30: 17/mar
    (31, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 31: 31/mar
    (7, 3, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 32: 7/abr
    (6, 4, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 33: 6/mai
    (13, 4, 1, 5, 1, &[(-1, -1, -1)]), // 34: 13/mai
    (20, 4, 1, 5, 1, &[(-1, -1, -1)]), // 35: 20/mai
];


/// The Scottish Premier Division before the year end.
pub(crate) const SCOTTISH_PREMIER_BEFORE: [EngFirstScheduleRound; 21] = [
    (29, 6, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 0: 29/jul
    (5, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 1: 5/ago
    (12, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 2: 12/ago
    (19, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 3: 19/ago
    (26, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 4: 26/ago
    (9, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 5: 9/set
    (16, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 6: 16/set
    (23, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 7: 23/set
    (30, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 8: 30/set
    (14, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 9: 14/out, mwb14: a Premier Division round
    (17, 9, 0, 1, 2, &[(-1, -1, -1)]), // 10: 17/out
    (21, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 11: 21/out
    (28, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 12: 28/out
    (4, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 13: 4/nov
    (11, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 14: 11/nov
    (18, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 15: 18/nov
    (25, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 16: 25/nov
    (29, 10, 0, 2, 2, &[(-1, -1, -1)]), // 17: 29/nov
    (2, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 18: 2/dez
    (9, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 19: 9/dez
    (16, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 20: 16/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 13 December (case 0).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_0: [EngFirstScheduleRound; 2] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 20/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 14 December (case 1).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_1: [EngFirstScheduleRound; 2] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 21/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 15 December (case 2).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_2: [EngFirstScheduleRound; 2] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 22/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 16 December (case 3).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_3: [EngFirstScheduleRound; 2] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 17 December (case 4).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_4: [EngFirstScheduleRound; 2] = [
    (20, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 20/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 18 December (case 5).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_5: [EngFirstScheduleRound; 2] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 19 December (case 6).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_6: [EngFirstScheduleRound; 2] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds out of range (after the error box).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_DEFAULT: [EngFirstScheduleRound; 2] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 22/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division after the year end.
pub(crate) const SCOTTISH_PREMIER_AFTER: [EngFirstScheduleRound; 15] = [
    (2, 0, 1, 1, 2, &[(-1, -1, -1)]), // 23: 2/jan
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 24: 20/jan
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 25: 27/jan
    (3, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 26: 3/fev
    (10, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 27: 10/fev
    (24, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 28: 24/fev
    (3, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 29: 3/mar
    (17, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 30: 17/mar
    (31, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 31: 31/mar
    (7, 3, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 32: 7/abr
    (14, 3, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 33: 14/abr
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 21/abr (the original first writes (5, 2, 2) into slot 0 and then the end mark over it: no TV slot)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 35: 28/abr
    (5, 4, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 36: 5/mai
    (12, 4, 1, 5, 1, &[(-1, -1, -1)]), // 37: 12/mai
];

/// The two split stages of the Scottish Premier Division (stage 0, the top six, and stage 1, the bottom six): five
/// Saturdays from 14 April to 12 May of the next year.
pub(crate) const SCOTTISH_PREMIER_SPLIT: [EngFirstScheduleRound; 5] = [
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 0: 14/abr
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 1: 21/abr
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 2: 28/abr
    (5, 4, 1, 5, 1, &[(-1, -1, -1)]), // 3: 5/mai
    (12, 4, 1, 5, 1, &[(-1, -1, -1)]), // 4: 12/mai, mwb14: the last split round
];

pub(crate) const SCOTTISH_PREMIER_1999_SCHEDULE: ScottishPremierSchedule = ScottishPremierSchedule {
    before: &SCOTTISH_PREMIER_1999_BEFORE,
    cases: [&SCOTTISH_PREMIER_1999_CHRISTMAS_0, &SCOTTISH_PREMIER_1999_CHRISTMAS_1, &SCOTTISH_PREMIER_1999_CHRISTMAS_2, &SCOTTISH_PREMIER_1999_CHRISTMAS_3,
            &SCOTTISH_PREMIER_1999_CHRISTMAS_4, &SCOTTISH_PREMIER_1999_CHRISTMAS_5, &SCOTTISH_PREMIER_1999_CHRISTMAS_6],
    default: &SCOTTISH_PREMIER_1999_CHRISTMAS_DEFAULT,
    after_at: 23,
    after: &SCOTTISH_PREMIER_1999_AFTER,
    rounds: 0x24,
    rows: (0xaf, 0x148),
};

pub(crate) const SCOTTISH_PREMIER_SCHEDULE: ScottishPremierSchedule = ScottishPremierSchedule {
    before: &SCOTTISH_PREMIER_BEFORE,
    cases: [&SCOTTISH_PREMIER_CHRISTMAS_0, &SCOTTISH_PREMIER_CHRISTMAS_1, &SCOTTISH_PREMIER_CHRISTMAS_2, &SCOTTISH_PREMIER_CHRISTMAS_3,
            &SCOTTISH_PREMIER_CHRISTMAS_4, &SCOTTISH_PREMIER_CHRISTMAS_5, &SCOTTISH_PREMIER_CHRISTMAS_6],
    default: &SCOTTISH_PREMIER_CHRISTMAS_DEFAULT,
    after_at: 23,
    after: &SCOTTISH_PREMIER_AFTER,
    rounds: 0x26,
    rows: (0x193, 0x223),
};

/// The main stage of the Scottish Premier Division: `league` (when given) = 1, `rounds`, `extra` = 0, the table
/// (`exe::crt_malloc` of 0x41 per round; out of memory, the box of the first line and 0); the rounds before; the date of
/// 1 December of the year (`exe::date_date_549f70`) and k = (the day of round 20 − the day of that date + 1, in 16
/// bits) − 13: from 0 to 6 the block of case k, outside the box of the second line and the default block; then the
/// rounds after, from `after_at`.
pub(crate) unsafe fn scottish_premier_round_table(this: League, rounds: *mut i16, extra: *mut i16, league: *mut i32, s: &ScottishPremierSchedule) -> usize {
    if !league.is_null() {
        *league = 1;
    }
    *rounds = s.rounds;
    *extra = 0;
    let t = exe::crt_malloc(s.rounds as u32 * 0x41);
    if t == 0 {
        show_error_at(0xa7af70, s.rows.0);
        return 0;
    }
    let year = (this.year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, s.before);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let day = read_u16(t + 20 * 0x41);
    let k = (day.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xd);
    let middle = if k as u32 > 6 {
        show_error_at(0xa7af70, s.rows.1);
        s.default
    } else {
        s.cases[k as usize]
    };
    eng_first_set_league_rounds(t, year, s.before.len() as i32, middle);
    eng_first_set_league_rounds(t, year, s.after_at, s.after);
    t
}

/// rule: the Scottish Premier Division plays 38 rounds (12 clubs): 33 in three rounds from 29 July to 7 April of the next
/// year, Saturdays with the TV slot (5, 2, 2) in most rounds and three midweek rounds (17 October, 29 November, 2
/// January), the year-end block of two rounds chosen by the day of the 21st round (20 to 23 December and 26 December);
/// then the split (stages 0 and 1: the top six and the bottom six, five Saturdays from 14 April to 12 May, stage label
/// 0x44b or 0x44c). In 1999 (10 clubs, 4 rounds): 36 rounds and no split. Any other stage: nothing and 0
/// Original: `ScoPrm::create_schedule` (0x7f0e80, sco_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let stage = stage as u8;
    if this.year() == 0x7cf {
        if stage != 0xff {
            return 0;
        }
        covered!("mwb_14 scottish premier calendar 1999");
        return scottish_premier_round_table(this, rounds, extra, league, &SCOTTISH_PREMIER_1999_SCHEDULE);
    }
    if stage == 0xff {
        covered!("mwb_14 scottish premier calendar");
        return scottish_premier_round_table(this, rounds, extra, league, &SCOTTISH_PREMIER_SCHEDULE);
    }
    if stage != 0 && stage != 1 {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 5;
    *extra = 0x44b + (stage != 0) as i16;
    let t = exe::crt_malloc(0x145);
    if t == 0 {
        show_error_at(0xa7af70, 0x279);
        return 0;
    }
    covered!("mwb_14 scottish premier split calendar");
    eng_first_set_league_rounds(t, (this.year() as u16) as u32, 0, &SCOTTISH_PREMIER_SPLIT);
    t
}

/// Comparer for `sco_prm_create_schedule` (side by side, the written rounds; the split tables by their 0x41-byte rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_PRM_CREATE_SCHEDULE);
    if stage as u8 == 0xff {
        return compare_league_round_table("sco_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_prm_create_schedule(this, stage, r, e, l));
    }
    compare_league_round_table("sco_prm_create_schedule", 0xff, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_prm_create_schedule(this, stage, r, e, l))
}

// ---------- mwb_15: the Scottish Premier Division (sco_prm.cpp): the turnover, the day update and the split ----------

/// The global holding the club index of Elgin City (moved into the Scottish Third Division at the 1999 turnover).
pub(crate) const ELGIN_CITY_INDEX_GLOBAL: usize = 0x9d029c;
/// The global holding the club index of Peterhead (the same).
pub(crate) const PETERHEAD_INDEX_GLOBAL: usize = 0x9d02a0;

/// rule: the Scottish turnover, led by the Premier Division: the promotion and relegation step (virtual +0xb0 with 0);
/// in 1999 Elgin City and Peterhead join the Third Division (the 2000 expansion); the Premier's old table, rounds and
/// stages are freed, the season archived, the year moves on, with no stages beyond the league; the rules, the table, the
/// fixtures and the order (each with its error box: lines 0x338, 0x33f, 0x345, 0x34c, and 0); the division tier of the
/// Second Division clubs capped (League::limit_club_division_tier 2, −3, 3, 0) and of the Third's (2, 0, −3, 0); then the
/// First, the Second and the Third turn over (virtual +8), and the Premier's predicted positions, the board expectations
/// and the referee renewal, whose eax it returns
/// Original: `ScoPrm::start_new_season` (0x7f2a70, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_start_new_season(this: usize) -> u32 {
    let league = League(this);
    let call = |o: usize, slot: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(o) + slot));
        f(o)
    };
    write_u32(this + league::SEASON_CLOSED, 0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this, 0);
    if league.year() == 0x7cf {
        covered!("mwb_15 scottish expansion 1999");
        for g in [ELGIN_CITY_INDEX_GLOBAL, PETERHEAD_INDEX_GLOBAL] {
            let c = read_usize(G_CLUB_TABLE) + read_usize(g) * club::SIZE;
            write_usize(c + club::DIVISION, comp_record_of(KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION));
        }
    }
    exe::league_free_table(league, 0);
    let l = league.round_dates();
    if l != 0 {
        exe::crt_free(l);
        league.set_round_dates(0);
    }
    if (league.current_stage() as i32) >= 0 {
        let mut i: i8 = 0;
        loop {
            let f = read_usize(league.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i = i.wrapping_add(1);
            if i as i32 > league.current_stage() as i32 {
                break;
            }
        }
    }
    league.set_current_stage(0xffff_ffff);
    exe::crt_free(league.stages());
    let stats = league.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    league.set_year(league.year().wrapping_add(1)); // mwb15: the next Premier season
    league.set_stage_count(0);
    league.set_stages(0);
    covered!("mwb_15 scottish premier turnover");
    if call(this, comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0xa7af70, 0x338);
        return 0;
    }
    if sco_prm_create_table_via_exe(this) == 0 {
        show_error_at(0xa7af70, 0x33f);
        return 0;
    }
    if exe::league_create_fixtures(league) == 0 {
        show_error_at(0xa7af70, 0x345);
        return 0;
    }
    if exe::league_sort_table(league, 0) == 0 {
        show_error_at(0xa7af70, 0x34c);
        return 0;
    }
    exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION), 2, (-3i32) as u32, 3, 0);
    exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION), 2, 0, -3, 0);
    for g in [KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION] {
        call(comp_by_global(g), comp_vtable::START_NEW_SEASON);
    }
    call(this, comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(league)
}

/// The Premier table routine of the original (ScoPrm::create_table 0x7f2820, thiscall, ret 0), called directly as the
/// turnover does.
pub(crate) unsafe fn sco_prm_create_table_via_exe(this: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7f2820usize);
    f(this)
}

/// Comparer for `sco_prm_start_new_season`: only the original runs (it turns the whole Scottish league over); the year
/// went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_PRM_START_NEW_SEASON);
    compare_state_league_new_season("sco_prm_start_new_season", League(this), || o(this))
}

/// Whether every club of the Premier table has played at least 33 games (the byte at +0x11 of each 0x3b-byte row,
/// signed); true with no rows.
pub(crate) unsafe fn all_played_33(league: League) -> bool {
    let n = league.club_count();
    let mut k: i16 = 0;
    while k < n {
        if read_i8(league.table() + k as usize * 0x3b + 0x11) < 0x21 { // mwb15: the split after 33 games
            return false;
        }
        k += 1;
    }
    true
}

/// rule: in the season played in three rounds (`league::LEGS` = 3), on a day with results (`results` ≠ 0) when every
/// club has played 33 games, the Premier splits (`ScoPrm::create_split_stages`); then the League day update
/// (League::update_for_date 0x6847c0, the same arguments: thiscall, ret 8), whose eax it returns
/// Original: `ScoPrm::update_for_date` (0x7f2d20, sco_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn sco_prm_update_for_date(this: usize, today: usize, results: u32) -> u32 {
    let league = League(this);
    if results != 0 && league.legs() == 3 && all_played_33(league) {
        covered!("mwb_15 scottish premier split day");
        let split: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7f2d90usize);
        split(this);
    }
    let update: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    update(this, today, results) as u32 // the callers read only al
}

/// Comparer for `sco_prm_update_for_date`: only the original runs (the split builds stages and the day update plays
/// on); the plan: whether the split was due (it leaves the league in four rounds with the split compare) and the return.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_update_for_date(this: usize, today: usize, results: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SCO_PRM_UPDATE_FOR_DATE);
    let league = League(this);
    let due = results != 0 && league.legs() == 3 && all_played_33(league);
    let r = o(this, today, results);
    let ok = !due || (league.legs() == 4 && league.custom_compare() == 0x7f3220);
    SCOREBOARD.record(ok, || format!("sco_prm_update_for_date({:#x}) split due {} legs {} compare {:#x}", this, due, league.legs(), league.custom_compare()));
    r
}

/// One half of the split: the round table of `stage` (virtual +0x3c: rounds and stage label into locals, no league
/// flag), a League_Stage (operator new of 0xee and League_Stage::League_Stage 0x68b100, thiscall with 21 arguments, ret
/// 0x58: this league as the parent, 6 clubs, the list, 1, the record, the round table and its count, the win, draw and
/// category bytes, the tie-breaks (+0xc5) and bands (+0xbe), the year, −1, the stage label, 0x14, 1, 0, 3, −1, 0, 2)
/// which is then destroyed at once (virtual 0 with 1: the fixtures it registered stay); returns the round table.
pub(crate) unsafe fn build_split_half(league: League, stage: u32, clubs: usize) -> usize {
    let t = league.addr();
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(t) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(t, stage, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count and the stage label as dwords with the stack's high
        // bytes; League_Stage::League_Stage reads the words
        let s = ctor(p, t, 6, clubs, 1, read_usize(t + league::COMPETITION), table, rounds as u16 as u32, read_u8(t + league::WIN_POINTS) as u32,
            read_u8(t + league::DRAW_POINTS) as u32, read_u8(t + league::MATCH_CATEGORY) as u32, t + league::TIEBREAKS, t + league::PROMOTED_COUNT,
            league.year() as u16 as u32, 0xffff_ffff, label as u16 as u32, 0x14, 1, 0, 3, 0xffff_ffff, 0, 2);
        if s != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(s)));
            destroy(s, 1);
        }
    }
    table
}

/// rule: the Scottish Premier split after 33 games: the top six (table rows 1 to 6) and the bottom six (from the last
/// row up) each get five more rounds among themselves (the split stages 0 and 1 of `ScoPrm::create_schedule`, built as
/// League_Stage objects whose fixtures stay); then the league plays on in four rounds (`league::LEGS` = 4) with the
/// split's table compare (`league::CUSTOM_COMPARE` = 0x7f3220), which keeps each half apart. Out of memory for the club
/// list: the box of line 0x3a2 and nothing more
/// Original: `ScoPrm::create_split_stages` (0x7f2d90, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_create_split_stages(this: usize) {
    let league = League(this);
    let clubs = exe::crt_malloc(0x18);
    if clubs == 0 {
        show_error_at(0xa7af70, 0x3a2);
        return;
    }
    covered!("mwb_15 scottish premier split");
    for k in 0..6usize {
        write_usize(clubs + 4 * k, read_usize(league.table() + k * 0x3b));
    }
    let top = build_split_half(league, 0, clubs);
    exe::crt_free(top);
    for k in 0..6usize {
        let row = (league.club_count() as i32 - k as i32 - 1) as usize; // mwb15: the bottom six from the last row up
        write_usize(clubs + 4 * k, read_usize(league.table() + row * 0x3b));
    }
    let bottom = build_split_half(league, 1, clubs);
    exe::crt_free(clubs);
    exe::crt_free(bottom);
    write_u16(this + league::LEGS, 4);
    league.set_custom_compare(0x7f3220);
}

/// Comparer for `sco_prm_create_split_stages`: only the original runs (it builds stages and fixtures); then the league
/// is in four rounds with the split compare.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_create_split_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCO_PRM_CREATE_SPLIT_STAGES);
    o(this);
    let league = League(this);
    SCOREBOARD.record(league.legs() == 4 && league.custom_compare() == 0x7f3220, || format!("sco_prm_create_split_stages({:#x}) legs {} compare {:#x}", this, league.legs(), league.custom_compare()));
}

// ---------- mwb_16: the Scottish League Cup (sco_lge_cup.cpp): constructor, round table and day update ----------

/// The Scottish League Cup at career creation.
///
/// Cup::Cup (`exe::new_cup`); the record, the vtable 0x96f568, the year; `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` =
/// 0xff, `cup::CURRENT_STAGE` = −1, `cup::STAGE_COUNT` = 0, `cup::CLUB_SIDES` = 1, 5 substitutes named and 3 allowed,
/// the discipline rule of the Scottish cups (0x19), `cup::RUN_BY_STAGES` = 0, `cup::SEASON_SET_UP` = 0 and the European
/// entrant count (+0xb2) = 0. If the saved state loads (Cup::load_from_disk 0x51fc00 with 1), returns this.
///
/// Otherwise: 40 teams (`cup::TEAM_COUNT`) and an empty team table (`exe::crt_calloc(0x28, 6)`; null: the box of line
/// 0x4a and this); the prize fund of 1,000,000; the round table of stage −1 (virtual +0x3c) into `cup::ROUND_TABLE`; the
/// fixture tree (Cup::map_fixture_tree 0x518790; 0 = the box of line 0x58); the statistics (operator new 0x5ce and
/// Comp_Stats::Comp_Stats; null: the box of line 0x5f). Unlike the other Scottish cups, neither the entrants (they come
/// when the season starts: `ScoLgeCup::update_for_date`) nor the predicted positions. Returns this.
/// Original: `ScoLgeCup::ScoLgeCup` (0x7ef9e0, sco_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(cup);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, 0x96f568);
    write_u16(this + cup::YEAR, year as u16);
    write_u8(this + cup::CURRENT_ROUND, 0);
    write_u8(this + cup::STAGE_INDEX, 0xff);
    write_u32(this + cup::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + cup::STAGE_COUNT, 0);
    write_u8(this + cup::CLUB_SIDES, 1);
    write_u8(this + cup::SUBSTITUTES_NAMED, 5);
    write_u8(this + cup::SUBSTITUTIONS_ALLOWED, 3); // mwb16: the League Cup substitutions
    write_u8(this + cup::DISCIPLINE_INDEX, 0x19);
    write_u8(this + cup::RUN_BY_STAGES, 0);
    write_u32(this + cup::SEASON_SET_UP, 0);
    write_u8(this + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("mwb_16 new scottish league cup");
    cup.set_team_count(0x28);
    let l = exe::crt_calloc(0x28, 6);
    cup.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7af38, 0x4a);
        return this;
    }
    write_u32(this + cup::PRIZE_FUND, 0xf4240);
    let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let table = rounds(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    write_usize(this + cup::ROUND_TABLE, table);
    let tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if tree(this) == 0 {
        show_error_at(0xa7af38, 0x58);
        return this;
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, cup) } else { 0 };
    write_usize(this + league::COMP_STATS, s);
    if s == 0 {
        show_error_at(0xa7af38, 0x5f);
    }
    this
}

/// Comparer for `construct_sco_lge_cup`: only the original runs (it allocates); then the plan: this, the vtable, the
/// year, the record, the discipline rule, the bench and, in a new career, the 40 teams, the prize fund and the rounds.
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_LGE_CUP);
    let loaded = league_will_load_saved_state(record);
    let r = o(this, year, record);
    let mut ok = r == this && read_usize(this) == 0x96f568 && read_u16(this + cup::YEAR) == year as u16 && read_usize(this + league::COMPETITION) == record
        && read_u8(this + cup::DISCIPLINE_INDEX) == 0x19;
    if !loaded && read_usize(this + league::COMP_STATS) != 0 {
        ok = ok && read_u8(this + cup::SUBSTITUTIONS_ALLOWED) == 3 && Cup(this).team_count() == 0x28 && read_u32(this + cup::PRIZE_FUND) == 0xf4240 && read_usize(this + cup::ROUND_TABLE) != 0;
    }
    SCOREBOARD.record(ok, || format!("construct_sco_lge_cup({:#x}) [year arg {}, record arg {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, teams {}, prize fund {}, rounds {:#x}",
        this, year as u16, record, loaded, r, read_usize(this), Cup(this).team_count(), read_u32(this + cup::PRIZE_FUND), read_usize(this + cup::ROUND_TABLE)));
    r
}

/// The Scottish League Cup rounds with no European entrants: the first round drawn on 10 July and played on 31 July,
/// the second on 1/8 and 17/8 (code 0x403 for the draw), the third on 19/8 and 22/9, the quarter-finals on 23/9 and
/// 1/12 (a replay 4 days later), the semi-finals on 2/12 and 16/2 at a neutral ground (venue 0x28) and the final drawn
/// on 17/2 and played on 19/3 at the national stadium (venue 4); single matches, tie-break code 3; category 4 in the
/// first three rounds, 6 after; no prize in the first two rounds, none (−1) after (`scottish_league_cup_rounds` fills
/// the team counts).
pub(crate) const SCOTTISH_LEAGUE_CUP_ROUNDS: [EngCcCupCupRound; 6] = [
    cql10r((0xa, 6, 0, 6), (0x1f, 6, 0, 5, 1, 0), 0xa, (0, 3, 0), 4, (0, 0, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql10r((1, 7, 0, 6), (0x11, 7, 0, 1, 2, 0), 0x14, (0, 0x403, 0), 4, (0, 0, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql10r((0x13, 7, 0, 3), (0x16, 8, 0, 2, 2, 0), 0x1e, (0, 3, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x17, 8, 0, 3), (1, 0xb, 0, 2, 2, 0), 0x78, (0, 3, 0), 6, (8, 4, 0, 0), (0, 1, 4), (0, 0xffff_ffff, 0)), // mwb16: the League Cup quarter-finals
    cql10r((2, 0xb, 0, 3), (0x10, 1, 1, 2, 2, 0x28), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x11, 1, 1, 3), (0x13, 2, 1, 6, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// The Scottish League Cup round table with the team counts adjusted by `b` = the European entrant count (+0xb2, signed
/// byte; arithmetic in 16 bits as in the original).
///
/// From 2000: 1st round 2b + 20 teams, b + 10 matches, 2b + 20 entering; 2nd round (16 − b) × 2 teams, 16 − b matches,
/// 22 − 3b entering and 2b + 20 already in; 3rd round b entering and 42 − b already in. In 1999 (30 clubs): 2b + 16, b + 8,
/// 2b + 16; (16 − b) × 2, 16 − b, (8 − b) × 3 and 2b + 16; b and 40 − b.
pub(crate) fn scottish_league_cup_rounds(b: i16, y1999: bool) -> [EngCcCupCupRound; 6] {
    let mut t = SCOTTISH_LEAGUE_CUP_ROUNDS;
    let x = |v: i16| v as u16;
    let (base, carried) = if y1999 { (0x10i16, 0x28i16) } else { (0x14, 0x2a) };
    t[0].teams = (x(b.wrapping_mul(2).wrapping_add(base)), x(b.wrapping_add(base / 2)), x(b.wrapping_mul(2).wrapping_add(base)), 0);
    let new2 = if y1999 { 8i16.wrapping_sub(b).wrapping_mul(3) } else { 0x16i16.wrapping_sub(b.wrapping_mul(3)) };
    t[1].teams = (x(0x10i16.wrapping_sub(b).wrapping_mul(2)), x(0x10i16.wrapping_sub(b)), x(new2), x(b.wrapping_mul(2).wrapping_add(base)));
    t[2].teams.2 = x(b);
    t[2].teams.3 = x(carried.wrapping_sub(b));
    t
}

/// rule: the Scottish League Cup has 6 single-match rounds from July to March: 1st round drawn 10/7, played 31/7, 2nd
/// 1/8 and 17/8, 3rd 19/8 and 22/9, quarter-finals 23/9 and 1/12, semi-finals 2/12 and 16/2 at a neutral ground, final
/// drawn 17/2 and played on 19/3 at the national stadium; the clubs in the European cups (+0xb2) only enter in the 3rd
/// round (from 2000: 2b + 20 teams in the 1st round, (16 − b) × 2 in the 2nd); category 4 in the first three rounds, 6
/// after; no prize per round. Outside stage −1 it writes nothing and returns 0; out of memory, the box of line 0xa0 and
/// 0 (`eng_cc_cup_create_single_stage_round_table`)
/// Original: `ScoLgeCup::create_round_table` (0x7efcb0, sco_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = scottish_league_cup_rounds(read_i8(this + eng_cc_cup::EUROPEAN_ENTRANT_COUNT) as i16, Cup(this).year() == 0x7cf);
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &t, 0xa7af38, 0xa0, "mwb_16 scottish league cup rounds")
}

/// Comparer for `sco_lge_cup_create_round_table` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_LGE_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("sco_lge_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| sco_lge_cup_create_round_table(this, stage, r, e, s))
}

/// rule: the Scottish League Cup is rebuilt once per season (`cup::SEASON_SET_UP`), on the first day with results when
/// the game year (the UEFA seeding's season year) reaches the cup year: the old tables are freed (the pointers left as
/// they are) and the entrants (ScoLgeCup::collect_entrants 0x7f0400, thiscall, ret 0; negative: the box of line 0x15e),
/// the rounds (virtual +0x8c; 0x165), the fixture tree (virtual +0x94; 0x16c) and the predicted positions (virtual
/// +0x5c) are built, after a box returning 0; then, every day, the cup update (Cup::update_for_date 0x51f890, the same
/// arguments), whose eax it returns
/// Original: `ScoLgeCup::update_for_date` (0x7f0230, sco_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let cup = Cup(this);
    if read_u32(this + cup::SEASON_SET_UP) == 0 && read_i16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as u16 as i32 == cup.year() as i32 && stage != 0 {
        free_season_tables(cup);
        covered!("mwb_16 scottish league cup season");
        write_u32(this + cup::SEASON_SET_UP, 1);
        // simplification: in the message boxes the original zeroes only al (the rest of eax comes from the box); the
        // caller reads the byte
        let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x7f0400usize);
        if entrants(this) < 0 {
            show_error_at(0xa7af38, 0x15e);
            return 0;
        }
        if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
            show_error_at(0xa7af38, 0x165);
            return 0;
        }
        if call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 {
            show_error_at(0xa7af38, 0x16c);
            return 0;
        }
        call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS); // mwb16: the League Cup predicted positions
    }
    exe::cup_update_for_date(cup, today, stage)
}

/// Comparer for `sco_lge_cup_update_for_date`: only the original runs (it frees, rebuilds, draws and plays); the plan:
/// `cup::SEASON_SET_UP` goes to 1 when the season was due, otherwise it stays.
pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SCO_LGE_CUP_UPDATE_FOR_DATE);
    let before = read_u32(this + cup::SEASON_SET_UP);
    let due = before == 0 && read_i16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as u16 as i32 == Cup(this).year() as i32 && stage != 0;
    let r = o(this, today, stage);
    let after = read_u32(this + cup::SEASON_SET_UP);
    SCOREBOARD.record(after == if due { 1 } else { before }, || format!("sco_lge_cup_update_for_date({:#x}) due {} +0x45 {} -> {}", this, due, before, after));
    r
}

// ---------- mwb_17: the Scottish League Cup (sco_lge_cup.cpp): the turnover ----------

/// rule: at the turnover the Scottish League Cup frees last season's teams, rounds and ties (zeroing the pointers),
/// archives the statistics, moves to the next year and goes back to 40 empty places (`exe::crt_calloc(0x28, 6)`; null:
/// the box of line 0x281 and 0), with no current round, not run by stages and the season not set up (the entrants come
/// in the day update); then the rounds (virtual +0x8c; negative: the box of line 0x288 and 0) and the fixture tree
/// (virtual +0x94; negative: 0x28f and 0), whose eax it returns (no predicted positions here)
/// Original: `ScoLgeCup::start_new_season` (0x7f0930, sco_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_start_new_season(this: usize) -> u32 {
    let cup = Cup(this);
    write_u32(this + cup::SEASON_CLOSED, 0);
    for o in [cup::TEAM_TABLE, cup::ROUND_TABLE] {
        let l = read_usize(this + o);
        if l != 0 {
            exe::crt_free(l);
            write_u32(this + o, 0);
        }
    }
    if cup.tie_tables() != 0 {
        let mut i = 0i32;
        while i < cup.round_count() as i32 {
            let p = read_usize(cup.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_u32(cup.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(cup.tie_tables());
        cup.set_tie_tables(0);
    }
    let stats = read_usize(this + league::COMP_STATS);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_year(cup.year().wrapping_add(1)); // mwb17: the next League Cup
    covered!("mwb_17 scottish league cup turnover");
    write_u8(this + cup::CURRENT_ROUND, 0);
    write_u8(this + cup::RUN_BY_STAGES, 0);
    write_u32(this + cup::SEASON_SET_UP, 0);
    cup.set_team_count(0x28);
    let l = exe::crt_calloc(0x28, 6);
    cup.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7af38, 0x281);
        return 0;
    }
    // simplification: in the boxes the original returns the box's eax (0x5e8290); the caller of the turnover does not
    // read it
    if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0xa7af38, 0x288);
        return 0;
    }
    let r = call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(0xa7af38, 0x28f);
        return 0;
    }
    r as u32
}

/// Comparer for `sco_lge_cup_start_new_season`: only the original runs (it frees and reallocates); the year went up by
/// one.
pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_LGE_CUP_START_NEW_SEASON);
    compare_cup_new_season("sco_lge_cup_start_new_season", Cup(this), || o(this))
}

// ---------- mwb_18: the Welsh League Cup (wel_lge_cup.cpp): the group table, the rules and the next stage; the moulds of the two Welsh cups played in groups ----------

/// The entrant pool of the Welsh League Cup and the Welsh Premier Cup (pointer to an array of club pointers, taken
/// group by group: each club is zeroed when it goes into a group; the count, 18 or 12, is at +0xf2).
pub(crate) const WELSH_CUP_POOL: usize = 0xee;

/// The own table of a Welsh group cup (the league object's table holds one group): `league::CLUB_COUNT` = `n`, the
/// table (`exe::crt_malloc` of n × 0x3b; out of memory, the box of `row` and 0), row k gets pool entry k × `stride`
/// (`exe::league_init_table_row(this, k, club, 0, 0)`) and that pool entry is zeroed; then the group draw news
/// (League::send_group_draw_news 0x684230, thiscall, ret 0); returns 1 (`rotate` is 0 in the original: row k gets the
/// k-th club; it exists only for the mutations).
pub(crate) unsafe fn welsh_cup_group_table(this: League, n: i16, stride: usize, file: usize, row: u32, rotate: u8, ramo: &'static str) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(ramo);
    let mut k: u8 = 0;
    while (k as i8 as i16) < this.club_count() {
        let e = read_usize(this.addr() + WELSH_CUP_POOL) + k as i8 as usize * stride * 4;
        // simplification: the original pushes the row as a stack dword with stale high bytes; 0x687430 reads the byte
        exe::league_init_table_row(this, ((k as u32 + rotate as u32) % n as u32) as u8 as u32, read_usize(e), 0, 0);
        write_usize(e, 0);
        k = k.wrapping_add(1);
    }
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    news(this.addr());
    1
}

/// Comparer for the Welsh group tables: the pool entries planned beforehand; only the original runs (it allocates and
/// sends news); then the count, the return 1, the rows and the pool entries zeroed.
pub(crate) unsafe fn compare_welsh_cup_group_table(name: &str, this: League, n: i16, stride: usize, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + WELSH_CUP_POOL);
    let plan: Vec<usize> = (0..n.max(0) as usize).map(|k| read_usize(pool + k * stride * 4)).collect();
    let r = original();
    let t = this.table();
    let mut ok = r == 1 && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if ok {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        ok = found == plan && (0..plan.len()).all(|k| read_usize(pool + k * stride * 4) == 0);
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// The rules of the two Welsh group cups: category 4, 2 rounds, no promotion play-off or relegation, no division above
/// or below (−1), 3 points for a win and 1 for a draw, club sides, a prize fund of 100,000; `promoted` go through from
/// each group (`league::PROMOTED_COUNT`) and the tie-breaks (`tiebreaks`, the fourth 0).
pub(crate) fn welsh_cup_rules(promoted: u8, tiebreaks: [u8; 3]) -> Vec<(usize, u8, u32)> {
    vec![(league::MATCH_CATEGORY, 1, 4), (league::TIEBREAKS, 1, tiebreaks[0] as u32), (league::TIEBREAKS + 1, 1, tiebreaks[1] as u32), (league::TIEBREAKS + 2, 1, tiebreaks[2] as u32),
         (league::TIEBREAKS + 3, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0),
         (league::LEGS, 2, 2), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::WIN_POINTS, 1, 3),
         (league::DRAW_POINTS, 1, 1), (league::PROMOTED_COUNT, 1, promoted as u32), (league::CLUB_SIDES, 1, 1), (league::PRIZE_FUND, 4, 0x186a0)]
}

/// The next stage of a Welsh group cup: at the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1,
/// signed) the error box and nothing else; otherwise the current stage goes up by one and, on reaching `at`, the
/// knockout stage is built (`knockout`, thiscall, ret 0).
pub(crate) unsafe fn welsh_cup_advance_stage(this: League, file: usize, row: u32, at: i32, knockout: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    this.set_current_stage(stage as usize);
    if stage == at {
        covered!("mwb_18 welsh cup knockout stage");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(knockout);
        f(this.addr());
    }
}

/// rule: the Welsh League Cup's own table holds one group of three: the first entrant of each pool row (the pool is 6
/// columns of 3; the column of group 0 is the first), taken out of the pool; then the group draw news; out of memory,
/// the box of line 0x182 and 0
/// Original: `WelLgeCup::create_table` (0x926700, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_table(this: usize) -> u32 {
    welsh_cup_group_table(League(this), 3, 6, 0xad56f4, 0x182, 0, "mwb_18 welsh league cup table") // mwb18: the League Cup first group
}

/// Comparer for `wel_lge_cup_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_LGE_CUP_CREATE_TABLE);
    compare_welsh_cup_group_table("wel_lge_cup_create_table", League(this), 3, 6, || o(this))
}

/// The Welsh League Cup rule writes: the group winner goes through, tie-breaks 4, 1, 2 [inference: head-to-head, goal
/// difference, goals for].
pub(crate) fn welsh_league_cup_rules() -> Vec<(usize, u8, u32)> {
    welsh_cup_rules(1, [4, 1, 2]) // mwb18: the League Cup group rules
}

/// rule: the Welsh League Cup groups play 2 rounds, 3 points for a win and 1 for a draw, category 4; the winner of each
/// group goes through; tie-breaks codes 4, 1 and 2 [inference: head-to-head, goal difference, goals for]; a prize fund
/// of 100,000; builds the round schedule of the main stage (virtual +0x3c); returns 1
/// Original: `WelLgeCup::set_up_rules` (0x926800, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwb_18 welsh league cup rules");
    eng_prm_apply_league_rules(League(this), &welsh_league_cup_rules())
}

/// Comparer for `wel_lge_cup_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_LGE_CUP_SET_UP_RULES);
    eng_prm_compare_league_rules("wel_lge_cup_set_up_rules", League(this), welsh_league_cup_rules(), || o(this), || wel_lge_cup_set_up_rules(this))
}

/// rule: the Welsh League Cup moves from group to group (stages 0 to 4 are the other five groups) and, on reaching stage
/// 5, builds the knockout stage of the eight best (WelLgeCup::create_knockout_stage 0x926d00); past the last stage, the
/// box of line 0x222
/// Original: `WelLgeCup::advance_stage` (0x926a70, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0xad56f4, 0x222, 5, 0x926d00)
}

/// Comparer for `wel_lge_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage
/// stays at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WEL_LGE_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("wel_lge_cup_advance_stage", League(this), || o(this))
}

// ---------- mwb_19: the Welsh League Cup (wel_lge_cup.cpp): the honours, the staff records and the group stages; the moulds of the two Welsh cups ----------

/// rule: the Welsh League Cup's season honours are those of its knockout stage (the sixth stage slot): the call goes on
/// to that stage's own `record_season_honours` (virtual +0x30), whose eax it returns
/// Original: `WelLgeCup::record_season_honours` (0x926890, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_record_season_honours(this: usize) -> u32 {
    covered!("mwb_19 welsh league cup honours");
    let knockout = read_usize(League(this).stages() + 4 * 5); // mwb19: the League Cup knockout slot
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(knockout) + comp_vtable::RECORD_SEASON_HONOURS));
    f(knockout)
}

// simplification: the honours go into the history: they do not run twice and the routine writes nothing besides the
// knockout's own. Original only.

/// Comparer for `wel_lge_cup_record_season_honours` (original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_LGE_CUP_RECORD_SEASON_HONOURS)(this)
}

/// Virtual +0x44 of the two Welsh group cups: the staff records by outcome and stage; returns 0.
///
/// In the groups (`stage`, signed byte, below `groups`): outcome 6, qualification (`european_cup_add_qualified_record`:
/// stage 0, round 0x78, weight 0x28); 0xfe, failure to qualify (`european_cup_add_failed_to_qualify_record`: 0x41c,
/// weight 0x28). In the knockout (stage `groups`, the stage slot `groups`): 0xff, knocked out with the match's round
/// prefix (+0x32), round name (+0x30) and the weight `scale_by_round(0x32, 0x64, round number, scale)`; 0, qualification
/// with the round prefix, the code of the next round of the knockout's round table and the same weight; 1 and 2, the
/// winner's and the runner-up's records (0x86a800, 0x86b0b0: the staff records, club, match, a6).
pub(crate) unsafe fn welsh_cup_staff_records(this: League, club: usize, outcome: u32, stage: u32, game: usize, a6: u32, groups: i8, scale: u32) -> u32 {
    let game = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = this.competition().addr();
    if f < groups {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x78, 0x28); // mwb19: the group qualification weight
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == groups {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), scale);
                // simplification: +0x32 and +0x30 go in registers with garbage in the high 16 bits; 0x86c000 only uses
                // the words
                european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
            0 => {
                let r = game.round_number();
                let weight = scale_by_round(0x32, 0x64, r, scale);
                let knockout = Cup(read_usize(this.stages() + groups as usize * 4));
                let round = read_i16(knockout.round_table().wrapping_add((r as isize * (cup_round::SIZE as isize)) as usize) + cup_round::ROUND_CODE + 0x68) as u16 as u32;
                european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, round, weight);
            }
            1 => african_nations_record_final_result(0x86a800, club, game.addr(), a6),
            2 => african_nations_record_final_result(0x86b0b0, club, game.addr(), a6),
            _ => {}
        }
    }
    0
}

/// rule: in the six Welsh League Cup groups, a club going through gets the qualification record (weight 40) and one
/// going out the failed-to-qualify record (weight 40); in the knockout (stage 5) a club going through gets the
/// qualification by round and one going out the knocked-out record (weight 50 to 100 over 5 rounds), and the winner
/// and the runner-up their final records
/// Original: `WelLgeCup::update_staff_records_for_status` (0x9271e0, wel_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwb_19 welsh league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 5, 5)
}

// simplification: the records go into the managers' history: they do not run twice and the routine writes nothing
// besides them. Original only; the plan checks the return of 0.

/// Comparer for `wel_lge_cup_update_staff_records_for_status` (original only, the return 0).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_WEL_LGE_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("wel_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// Builds group `g` of a Welsh group cup as a League_Stage, in the original routine's order.
///
/// The round table of stage `g` (virtual +0x3c: the round count and the stage label into locals, no league flag); the
/// club list (`exe::crt_malloc` of 4 × `n`; null: the box of the first line and 0) with pool entries g + 1 + k × `stride`
/// (k = 0..n), each zeroed in the pool; a League_Stage (operator new of 0xee and League_Stage::League_Stage 0x68b100:
/// this cup as the parent, `n` clubs, the list, 2, the record, the round table and its count, the win, draw and
/// category bytes, the tie-breaks (+0xc5), a local tie-break list (`first_tiebreak`, 0, 0, 0), the year, `g`, the stage
/// label, the importance (+0x51), 1, 0, the league flags (+0xd9), −1, 0, 2) into stage slot `g` (0 without memory); the
/// list and the round table freed; no stage: the box of the second line and 0; otherwise the group draw news of the
/// stage (League::send_group_draw_news), the current stage = `g` and 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn welsh_cup_group_stage(this: League, g: u32, n: u32, stride: usize, first_tiebreak: u8, file: usize, rows: (u32, u32), ramo: &'static str) -> u32 {
    let t = this.addr();
    let tiebreaks: [u8; 4] = [first_tiebreak, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(t) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(t, g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * n);
    if clubs == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(ramo);
    for k in 0..n as usize {
        let e = read_usize(t + WELSH_CUP_POOL) + (g as usize + 1 + k * stride) * 4; // mwb19: the group clubs in pool order
        write_usize(clubs + 4 * k, read_usize(e));
        write_usize(e, 0);
    }
    let p = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count, the stage label and the flags as stack or register
        // dwords with stale high bytes; League_Stage::League_Stage reads the words and bytes
        s = ctor(p, t, n, clubs, 2, read_usize(t + league::COMPETITION), table, rounds as u16 as u32, read_u8(t + league::WIN_POINTS) as u32,
            read_u8(t + league::DRAW_POINTS) as u32, read_u8(t + league::MATCH_CATEGORY) as u32, t + league::TIEBREAKS, tiebreaks.as_ptr() as usize,
            this.year() as u16 as u32, g, label as u16 as u32, read_u8(t + league::IMPORTANCE) as u32, 1, 0, read_u16(t + league::LEAGUE_FLAGS) as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(file, rows.1);
        return 0;
    }
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    news(s);
    this.set_current_stage(g as usize);
    1
}

/// Comparer for the Welsh group stages: the pool entries of the group planned beforehand; only the original runs (it
/// builds a stage with its fixtures); then the return 1, the stage in its slot, the current stage and the pool entries
/// zeroed.
pub(crate) unsafe fn compare_welsh_cup_group_stage(name: &str, this: League, g: u32, n: u32, stride: usize, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + WELSH_CUP_POOL);
    let r = original();
    let ok = r == 1 && read_usize(this.stages() + 4 * g as usize) != 0 && this.current_stage() as u32 == g
        && (0..n as usize).all(|k| read_usize(pool + (g as usize + 1 + k * stride) * 4) == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}, group {}) returned {}, stage {:#x}, current {}", name, this.addr(), g, r, read_usize(this.stages() + 4 * g as usize), this.current_stage() as i32));
    r
}

/// rule: Welsh League Cup group `g` (0 to 4; the sixth group is the cup's own table): three clubs, pool entries g + 1,
/// g + 7 and g + 13 (the pool's column g + 1), in a league stage of their own with tie-break list (1, 0, 0, 0); then the
/// group draw news; out of memory the box of line 0x249, without the stage that of 0x25f (and 0)
/// Original: `WelLgeCup::create_group_stage` (0x926b00, wel_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    welsh_cup_group_stage(League(this), g, 3, 6, 1, 0xad56f4, (0x249, 0x25f), "mwb_19 welsh league cup group")
}

/// Comparer for `wel_lge_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_WEL_LGE_CUP_CREATE_GROUP_STAGE);
    compare_welsh_cup_group_stage("wel_lge_cup_create_group_stage", League(this), g, 3, 6, || o(this, g))
}

// ---------- mwb_20: the Welsh Premier Cup (wel_prm_cup.cpp): the group table, the rules and the next stage ----------

/// rule: the Welsh Premier Cup's own table holds the first group of four: pool entries 0, 2, 4 and 6, taken out of the
/// pool; then the group draw news; out of memory, the box of line 0x181 and 0 (`welsh_cup_group_table`, mwb_18)
/// Original: `WelPrmCup::create_group_table` (0x928060, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_group_table(this: usize) -> u32 {
    welsh_cup_group_table(League(this), 4, 2, 0xad572c, 0x181, 0, "mwb_20 welsh premier cup table") // mwb20: the Premier Cup first group
}

/// Comparer for `wel_prm_cup_create_group_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_PRM_CUP_CREATE_GROUP_TABLE);
    compare_welsh_cup_group_table("wel_prm_cup_create_group_table", League(this), 4, 2, || o(this))
}

/// The Welsh Premier Cup rule writes: the top 2 of each group go through, tie-breaks 1, 2, 4 [inference: goal
/// difference, goals for, head-to-head].
pub(crate) fn welsh_premier_cup_rules() -> Vec<(usize, u8, u32)> {
    welsh_cup_rules(2, [1, 2, 4]) // mwb20: the Premier Cup group rules
}

/// rule: the Welsh Premier Cup groups play 2 rounds, 3 points for a win and 1 for a draw, category 4; the top 2 of each
/// group go through; tie-breaks codes 1, 2 and 4 [inference: goal difference, goals for, head-to-head]; a prize fund of
/// 100,000; builds the round schedule of the main stage (virtual +0x3c); returns 1
/// Original: `WelPrmCup::set_up_rules` (0x928160, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwb_20 welsh premier cup rules");
    eng_prm_apply_league_rules(League(this), &welsh_premier_cup_rules())
}

/// Comparer for `wel_prm_cup_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_PRM_CUP_SET_UP_RULES);
    eng_prm_compare_league_rules("wel_prm_cup_set_up_rules", League(this), welsh_premier_cup_rules(), || o(this), || wel_prm_cup_set_up_rules(this))
}

/// rule: the Welsh Premier Cup moves from the first group (its own table) to the second (stage 0) and, on reaching
/// stage 1, builds the knockout stage of the eight (WelPrmCup::create_knockout_stage 0x928650); past the last stage,
/// the box of line 0x220 (`welsh_cup_advance_stage`, mwb_18)
/// Original: `WelPrmCup::advance_stage` (0x9283c0, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0xad572c, 0x220, 1, 0x928650)
}

/// Comparer for `wel_prm_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage
/// stays at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WEL_PRM_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("wel_prm_cup_advance_stage", League(this), || o(this))
}

// ---------- mwb_21: the Welsh Premier Cup (wel_prm_cup.cpp): the staff records and the second group ----------

/// rule: in the two Welsh Premier Cup groups, a club going through gets the qualification record (weight 40) and one
/// going out the failed-to-qualify record (weight 40); in the knockout (stage 1) a club going through gets the
/// qualification by round and one going out the knocked-out record (weight 50 to 100 over 3 rounds), and the winner
/// and the runner-up their final records (`welsh_cup_staff_records`, mwb_19)
/// Original: `WelPrmCup::update_staff_records_for_status` (0x928af0, wel_prm_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwb_21 welsh premier cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 1, 3) // mwb21: the Premier Cup knockout over 3 rounds
}

/// Comparer for `wel_prm_cup_update_staff_records_for_status` (original only, the return 0).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_WEL_PRM_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("wel_prm_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// rule: the second Welsh Premier Cup group (stage 0): four clubs, pool entries 1, 3, 5 and 7, in a league stage of
/// their own with tie-break list (2, 0, 0, 0); then the group draw news; out of memory the box of line 0x247, without
/// the stage that of 0x25d (and 0) (`welsh_cup_group_stage`, mwb_19)
/// Original: `WelPrmCup::create_group_stage` (0x928450, wel_prm_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_group_stage(this: usize, g: u32) -> u32 {
    welsh_cup_group_stage(League(this), g, 4, 2, 2, 0xad572c, (0x247, 0x25d), "mwb_21 welsh premier cup group")
}

/// Comparer for `wel_prm_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_WEL_PRM_CUP_CREATE_GROUP_STAGE);
    compare_welsh_cup_group_stage("wel_prm_cup_create_group_stage", League(this), g, 4, 2, || o(this, g))
}
