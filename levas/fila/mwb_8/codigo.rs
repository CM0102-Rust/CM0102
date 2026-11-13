// ---------- mwb_8: the Scottish Third Division (sco_third.cpp): constructor, table and rules ----------

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

/// The Scottish Third Division at career creation: discipline rule 0x18, no stages beyond the league, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5).
/// Original: `ScoThird::ScoThird` (0x7f41f0, sco_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_third(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_THIRD_DIVISION, "mwb_8 new scottish third")
}

/// Comparer for `construct_sco_third` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_third(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_british_division_table("sco_third_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION, None, || o(this))
}

/// The Third Division rule writes (`scottish_division_rules`): the top 2 go up to the Second, nobody goes down (no
/// division below: −1). The original compares the year with 1999 without using it: the bands are the same both ways.
pub(crate) unsafe fn scottish_third_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 2, 0, 0, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, 0) // mwb8: the Third Division rules
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("sco_third_set_up_rules", League(this), scottish_third_rules(League(this)), || o(this), || sco_third_set_up_rules(this))
}
