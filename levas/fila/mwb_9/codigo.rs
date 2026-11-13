// ---------- mwb_9: the Scottish Second Division (sco_second.cpp): the schedule and the turnover; the schedule shared with the Third ----------

/// The Scottish Second and Third Divisions before the turn of the year: 18 Saturday rounds from 7 August to 18 December
/// (none on 9 October and 11 December).
pub(crate) const SCOTTISH_LOWER_BEFORE: [EngFirstScheduleRound; 18] = [
    (7, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 7/ago
    (14, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 14/ago
    (21, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 21/ago
    (28, 7, 0, 5, 1, &[(-1, -1, -1)]), // 3: 28/ago
    (4, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 4/set
    (11, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 11/set
    (18, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 18/set
    (25, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 25/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: 2/out
    (16, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 16/out, mwb9: a Second and Third Division round
    (23, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 23/out
    (30, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 30/out
    (6, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 6/nov
    (13, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 13/nov
    (20, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 20/nov
    (27, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 27/nov
    (4, 11, 0, 5, 1, &[(-1, -1, -1)]), // 16: 4/dez
    (18, 11, 0, 5, 1, &[(-1, -1, -1)]), // 17: 18/dez
];

/// The Scottish Second and Third Divisions after the turn of the year: 3 January (on its own weekday, code 2), then
/// Saturdays from 15 January to 6 May of the next year (none on 8 January).
pub(crate) const SCOTTISH_LOWER_AFTER: [EngFirstScheduleRound; 17] = [
    (3, 0, 1, 0, 2, &[(-1, -1, -1)]), // 19: 3/jan
    (15, 0, 1, 5, 1, &[(-1, -1, -1)]), // 20: 15/jan
    (22, 0, 1, 5, 1, &[(-1, -1, -1)]), // 21: 22/jan
    (5, 1, 1, 5, 1, &[(-1, -1, -1)]), // 22: 5/fev
    (12, 1, 1, 5, 1, &[(-1, -1, -1)]), // 23: 12/fev
    (19, 1, 1, 5, 1, &[(-1, -1, -1)]), // 24: 19/fev
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 25: 26/fev
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 26: 4/mar
    (11, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 11/mar
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 28: 18/mar
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 25/mar
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 1/abr
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 31: 8/abr
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 15/abr
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 22/abr
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 29/abr
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 35: 6/mai
];

/// The main-stage schedule of the Scottish Second Division: the reference round is the 18th (index 17); the year-end
/// cases of the First Division (`SCOTTISH_YEAR_END_CASES`, mwb_6); out of memory, the box of line 0xa5; the case out of
/// range, 0x109.
pub(crate) const SCOTTISH_SECOND_SCHEDULE: ScottishSchedule = ScottishSchedule {
    before: &SCOTTISH_LOWER_BEFORE,
    reference: 17,
    cases: SCOTTISH_YEAR_END_CASES,
    default: &SCOTTISH_BOXING_DAY_SATURDAY,
    after: &SCOTTISH_LOWER_AFTER,
    file: 0xa7afa8,
    rows: (0xa5, 0x109),
};

/// rule: the Scottish Second Division plays 36 rounds (10 clubs, 4 rounds), Saturdays from 7 August to 6 May of the next
/// year: 18 before the year end (none on 9 October and 11 December), the year-end round on 26, 27 or 28 December by the
/// weekday and 17 after (3 January on its own day, none on 8 January); no TV slots; only the main stage (any other
/// stage: nothing and 0)
/// Original: `ScoSecond::create_schedule` (0x7f3540, sco_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    scottish_league_round_table(League(this), rounds, extra, league, &SCOTTISH_SECOND_SCHEDULE, "mwb_9 scottish second calendar")
}

/// Comparer for `sco_second_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("sco_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_second_create_schedule(this, stage, r, e, l))
}

/// rule: at the turnover the Scottish Second Division is rebuilt: the season archived in the statistics, the old table,
/// rounds and stages freed, the next year, the rules, the table, the fixtures and the order; then the predicted
/// positions, the board expectations and the referees renewed (`british_division_new_season`, mwb_6; the error boxes of
/// lines 0x1c3, 0x1ca, 0x1d0 and 0x1d7)
/// Original: `ScoSecond::start_new_season` (0x7f4030, sco_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_second_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &SCOTTISH_SECOND_DIVISION, [0x1c3, 0x1ca, 0x1d0, 0x1d7], false, 1, "mwb_9 scottish second turnover") // mwb9: the next Second Division season
}

/// Comparer for `sco_second_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("sco_second_start_new_season", League(this), || o(this))
}
