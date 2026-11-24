// ---------- cx02_18: the Greek Cup rules, turnover and predicted positions (gre_cup.cpp) ----------

/// The Greek Cup groups: 3 points for a win and 1 for a draw, category 4 (national cup), the tie-breaks 4, 1, 2 (the
/// head-to-head first [inference]), two places through per group, a 600,000 prize fund.
pub(crate) const GREEK_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [4, 1, 2, 0], slots: 2, prize: Some(0x927c0), bits: None };

/// The Greek Cup rules: `set_cup_league_rules` with `GREEK_CUP_GROUP_RULES` and one meeting (checked instruction by
/// instruction against NirLgeCup::set_up_rules: only the tie-break order and the prize change), then the round table.
/// Returns 1.
/// Original: `GreCup::set_up_rules` (0x5e9350, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_set_up_rules(this: usize) -> u32 {
    covered!("cx02_18 greek cup rules");
    set_cup_league_rules(League(this), &GREEK_CUP_GROUP_RULES, 1)
}

/// The Greek Cup turnover: `start_irish_league_cup_season` with GreCup::collect_entrants (0x5e9a80), GreCup::create_table
/// (0x5e9260), GreCup::create_group_stage (0x5e9650) for the ten groups, the year up by one and the boxes at lines
/// 0x21c, 0x223, 0x229 and 0x230 of gre_cup.cpp (checked instruction by instruction against
/// NirLgeCup::start_new_season: only the routines, the group count and the lines change).
/// Original: `GreCup::start_new_season` (0x5e93f0, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_start_new_season(this: usize) -> u32 {
    covered!("cx02_18 greek cup turnover");
    start_irish_league_cup_season(League(this), 0x5e9a80, 0x5e9260, 0x5e9650, 10, 0x9cdab0, [0x21c, 0x223, 0x229, 0x230], 1)
}

/// The Greek Cup predicted positions: `italian_cup_predicted_positions` with the knockout at stage 10, bands of 11 and
/// the boxes at lines 0x3f0 and 0x3f8 of gre_cup.cpp (checked instruction by instruction against
/// ItaCup::set_predicted_positions: only the file, the lines, the stage and the band change).
/// Original: `GreCup::set_predicted_positions` (0x5ea060, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9cdab0, (0x3f0, 0x3f8), 0xa, 0xb, "cx02_18 greek cup predicted")
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    check_cup_league_rules("gre_cup_set_up_rules", League(this), &GREEK_CUP_GROUP_RULES, r);
    r
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("gre_cup_start_new_season", League(this), || o(this))
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_italian_cup_predicted("gre_cup_set_predicted_positions", this, gre_cup_set_predicted_positions, || o(this));
}
