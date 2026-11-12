// ---------- batch %%LEVA%%: the Serie C cup: the predicted positions and the clubs' records (ita_c_cup.cpp) ----------

/// The predicted positions of the Serie C cup (at creation): the 90 entrants by reputation; the knockout of stage 0xf
/// gives 1 and 2 to the final's pair, then a shared place to the losers of each round; the rest in bands of 16
/// (`italian_cup_predicted_positions`, cqx_7; the boxes at lines 0x439 and 0x441).
/// Original: `ItaCCup::set_predicted_positions` (0x63e580, ita_c_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn ita_c_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9ea204, (0x439, 0x441), 0xf, 0x10, "cqx_8 serie c cup predicted") // cqx8: the Serie C cup bands of 16
}

/// Comparer for `ita_c_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
unsafe extern "thiscall" fn compare_ita_c_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_italian_cup_predicted("ita_c_cup_set_predicted_positions", this, ita_c_cup_set_predicted_positions, || o(this));
}

/// Rule: in the Serie C cup, as in the Coppa Italia, with the groups below stage 0xf and the group qualification at
/// round 0x14 (weight 0x28) (`italian_cup_staff_records`, cqx_7).
/// Original: `ItaCCup::update_staff_records_for_status` (0x63e270, ita_c_cup.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn ita_c_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    italian_cup_staff_records(this, club, outcome, stage, game, final_, 0xf, 0x14, "cqx_8 serie c cup group qualified")
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: return 0.
unsafe extern "thiscall" fn compare_ita_c_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("ita_c_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}
