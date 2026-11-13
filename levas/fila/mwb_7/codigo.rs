// ---------- mwb_7: the Scottish Second Division (sco_second.cpp): constructor, table and rules ----------

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

/// The Scottish Second Division at career creation: discipline rule 0x18, no stages beyond the league, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5).
/// Original: `ScoSecond::ScoSecond` (0x7f3250, sco_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_second(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_SECOND_DIVISION, "mwb_7 new scottish second")
}

/// Comparer for `construct_sco_second` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_british_division_table("sco_second_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, None, || o(this))
}

/// The Second Division rule writes (`scottish_division_rules`): the top 2 go up to the First, the bottom 2 down to the
/// Third (in 1999 nobody down and no play-off).
pub(crate) unsafe fn scottish_second_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 2, 0, 2, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION) // mwb7: the Second Division rules
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("sco_second_set_up_rules", League(this), scottish_second_rules(League(this)), || o(this), || sco_second_set_up_rules(this))
}
