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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    if stage as u8 == 0xff {
        return compare_league_round_table("sco_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_prm_create_schedule(this, stage, r, e, l));
    }
    compare_league_round_table("sco_prm_create_schedule", 0xff, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_prm_create_schedule(this, stage, r, e, l))
}
