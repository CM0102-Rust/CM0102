use super::*;

/// The Scottish Third Division at career creation: discipline rule 0x18, no stages beyond the league, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5).
/// Original: `ScoThird::ScoThird` (0x7f41f0, sco_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_third(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_THIRD_DIVISION, "mwb_8 new scottish third")
}

/// Comparer for `construct_sco_third` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_third(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_THIRD);
    compare_british_division_ctor("construct_sco_third", League(this), year, record, &SCOTTISH_THIRD_DIVISION, || o(this, year, record))
}

/// The Third Division table: 10 clubs, those whose division is the Scottish Third, in database order; no prize fund
/// and no TV money (returns 1 only when full); out of memory, the box of line 0x159
/// Original: `ScoThird::create_table` (0x7f4e20, sco_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_third_create_table(this: usize) -> u32 {
    fill_british_division_table(League(this), 0xa, 0xa7afe4, 0x159, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION, None, None, 0, "mwb_8 scottish third table") // mwb8: the Third Division rows
}

/// Comparer for `sco_third_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_third_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_THIRD_CREATE_TABLE);
    compare_british_division_table("sco_third_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION, None, || o(this))
}

/// rule: the Scottish Third Division (10 clubs) plays 4 rounds, 3 points for a win and 1 for a draw; the top 2 go up to
/// the Second and nobody goes down (it is the last division); tie-breaks goal difference, goals for and position
/// [inference from codes 1, 2, 3]; 5 substitutes named and 3 allowed; builds the round schedule; returns 1
/// Original: `ScoThird::set_up_rules` (0x7f4f20, sco_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_third_set_up_rules(this: usize) -> u32 {
    covered!("mwb_8 scottish third rules");
    eng_prm_apply_league_rules(League(this), &scottish_third_rules(League(this)))
}

/// Comparer for `sco_third_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_third_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_THIRD_SET_UP_RULES);
    eng_prm_compare_league_rules("sco_third_set_up_rules", League(this), scottish_third_rules(League(this)), || o(this), || sco_third_set_up_rules(this))
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_THIRD_CREATE_SCHEDULE);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_THIRD_START_NEW_SEASON);
    compare_state_league_new_season("sco_third_start_new_season", League(this), || o(this))
}

/// The Third Division rule writes (`scottish_division_rules`): the top 2 go up to the Second, nobody goes down (no
/// division below: −1). The original compares the year with 1999 without using it: the bands are the same both ways.
pub(crate) unsafe fn scottish_third_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 2, 0, 0, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, 0) // mwb8: the Third Division rules
}

/// The Scottish Third Division: discipline rule 0x18, no stadium requirement, no 1999 play-off.
pub(crate) const SCOTTISH_THIRD_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f770,
    stadium: None,
    discipline: 0x18, // mwb8: the Third Division discipline rule
    playoff_in_1999: false,
    split_compare: None,
    rules: 0x7f4f20,
    table: 0x7f4e20,
    file: 0xa7afe4,
    rows: [0x44, 0x4b, 0x52, 0x59, 0x60],
};

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
