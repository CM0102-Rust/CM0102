use super::*;

/// The Scottish Second Division at career creation: discipline rule 0x18, no stages beyond the league, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5).
/// Original: `ScoSecond::ScoSecond` (0x7f3250, sco_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_second(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_SECOND_DIVISION, "mwb_7 new scottish second")
}

/// Comparer for `construct_sco_second` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_SECOND);
    compare_british_division_ctor("construct_sco_second", League(this), year, record, &SCOTTISH_SECOND_DIVISION, || o(this, year, record))
}

/// The Second Division table: 10 clubs, those whose division is the Scottish Second, in database order; no prize fund
/// and no TV money (returns 1 only when full); out of memory, the box of line 0x157
/// Original: `ScoSecond::create_table` (0x7f3e80, sco_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_second_create_table(this: usize) -> u32 {
    fill_british_division_table(League(this), 0xa, 0xa7afa8, 0x157, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, None, None, 0, "mwb_7 scottish second table") // mwb7: the Second Division rows
}

/// Comparer for `sco_second_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_SECOND_CREATE_TABLE);
    compare_british_division_table("sco_second_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, None, || o(this))
}

/// rule: the Scottish Second Division (10 clubs) plays 4 rounds, 3 points for a win and 1 for a draw; the top 2 go up
/// to the First and the bottom 2 go down to the Third (in 1999 nobody went down); tie-breaks goal difference, goals for
/// and position [inference from codes 1, 2, 3]; 5 substitutes named and 3 allowed; builds the round schedule; returns 1
/// Original: `ScoSecond::set_up_rules` (0x7f3f80, sco_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_second_set_up_rules(this: usize) -> u32 {
    covered!("mwb_7 scottish second rules");
    eng_prm_apply_league_rules(League(this), &scottish_second_rules(League(this)))
}

/// Comparer for `sco_second_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("sco_second_set_up_rules", League(this), scottish_second_rules(League(this)), || o(this), || sco_second_set_up_rules(this))
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_SECOND_CREATE_SCHEDULE);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("sco_second_start_new_season", League(this), || o(this))
}

/// The Second Division rule writes (`scottish_division_rules`): the top 2 go up to the First, the bottom 2 down to the
/// Third (in 1999 nobody down and no play-off).
pub(crate) unsafe fn scottish_second_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 2, 0, 2, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION) // mwb7: the Second Division rules
}

/// The Scottish Second Division: discipline rule 0x18, no stadium requirement, no 1999 play-off.
pub(crate) const SCOTTISH_SECOND_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f6bc,
    stadium: None,
    discipline: 0x18, // mwb7: the Second Division discipline rule
    playoff_in_1999: false,
    split_compare: None,
    rules: 0x7f3f80,
    table: 0x7f3e80,
    file: 0xa7afa8,
    rows: [0x42, 0x49, 0x50, 0x57, 0x5e],
};

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
