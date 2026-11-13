// ---------- mwb_10: the Scottish Third Division (sco_third.cpp): the schedule and the turnover ----------

/// The main-stage schedule of the Scottish Third Division: the rounds of the Second (`SCOTTISH_LOWER_BEFORE` and
/// `SCOTTISH_LOWER_AFTER`, mwb_9) and the same year-end cases; out of memory, the box of line 0xa7; the case out of
/// range, 0x10b.
pub(crate) const SCOTTISH_THIRD_SCHEDULE: ScottishSchedule = ScottishSchedule {
    before: &SCOTTISH_LOWER_BEFORE,
    reference: 17,
    cases: SCOTTISH_YEAR_END_CASES, // mwb10: the Third Division year-end cases
    default: &SCOTTISH_BOXING_DAY_SATURDAY,
    after: &SCOTTISH_LOWER_AFTER,
    file: 0xa7afe4,
    rows: (0xa7, 0x10b),
};

/// rule: the Scottish Third Division plays the calendar of the Second: 36 rounds, Saturdays from 7 August to 6 May of
/// the next year, the year-end round on 26, 27 or 28 December by the weekday; only the main stage (any other stage:
/// nothing and 0)
/// Original: `ScoThird::create_schedule` (0x7f44e0, sco_third.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    scottish_league_round_table(League(this), rounds, extra, league, &SCOTTISH_THIRD_SCHEDULE, "mwb_10 scottish third calendar")
}

/// Comparer for `sco_third_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("sco_third_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_third_create_schedule(this, stage, r, e, l))
}

/// rule: at the turnover the Scottish Third Division is rebuilt like the Second (`british_division_new_season`, mwb_6;
/// the error boxes of lines 0x1c5, 0x1cc, 0x1d2 and 0x1d9)
/// Original: `ScoThird::start_new_season` (0x7f4fb0, sco_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_third_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &SCOTTISH_THIRD_DIVISION, [0x1c5, 0x1cc, 0x1d2, 0x1d9], false, 1, "mwb_10 scottish third turnover") // mwb10: the next Third Division season
}

/// Comparer for `sco_third_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("sco_third_start_new_season", League(this), || o(this))
}
