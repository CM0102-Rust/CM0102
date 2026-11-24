// ---------- cx02_21: the Dutch Cup rules, predicted positions and staff records (hol_cup.cpp) ----------

/// The Dutch Cup groups: as the Greek Cup's (3 points for a win, 1 for a draw, category 4, the tie-breaks 4, 1, 2, two
/// places through per group), with an 800,000 prize fund.
pub(crate) const DUTCH_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [4, 1, 2, 0], slots: 2, prize: Some(0xc3500), bits: None };

/// The Dutch Cup rules: `set_cup_league_rules` with `DUTCH_CUP_GROUP_RULES` and one meeting (checked instruction by
/// instruction against NirLgeCup::set_up_rules: only the tie-break order and the prize change), then the round table.
/// Returns 1.
/// Original: `HolCup::set_up_rules` (0x5f3e70, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_set_up_rules(this: usize) -> u32 {
    covered!("cx02_21 dutch cup rules");
    set_cup_league_rules(League(this), &DUTCH_CUP_GROUP_RULES, 1)
}

/// The Dutch Cup predicted positions: `italian_cup_predicted_positions` with the knockout at stage 19, bands of 20 and
/// the boxes at lines 0x4be and 0x4c6 of hol_cup.cpp (checked instruction by instruction against
/// ItaCup::set_predicted_positions: only the file, the lines, the stage and the band change).
/// Original: `HolCup::set_predicted_positions` (0x5f5120, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9cedd4, (0x4be, 0x4c6), 0x13, 0x14, "cx02_21 dutch cup predicted")
}

/// The Dutch Cup staff records: `italian_cup_staff_records` with the knockout at stage 19 and the group qualification
/// round 0x14 (checked instruction by instruction against ItaCup::update_staff_records_for_status). Returns 0.
/// Original: `HolCup::update_staff_records_for_status` (0x5f4d40, hol_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn hol_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    italian_cup_staff_records(this, club, outcome, stage, game, final_, 0x13, 0x14, "cx02_21 dutch cup group qualified")
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    check_cup_league_rules("hol_cup_set_up_rules", League(this), &DUTCH_CUP_GROUP_RULES, r);
    r
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_italian_cup_predicted("hol_cup_set_predicted_positions", this, hol_cup_set_predicted_positions, || o(this));
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I2%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("hol_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}
