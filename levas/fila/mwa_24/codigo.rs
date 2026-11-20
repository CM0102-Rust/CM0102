// ---------- mwa_24: the Northern Irish League Cup (nir_lge_cup.cpp): the group stages, the managers' records and the predicted positions ----------

/// Rule: the Northern Irish League Cup's groups 2 to 4 (stages 0 to 2; the cup's own table is the first group): five
/// clubs each, the next entrants of the pool in order, in a league stage of their own where two places go through;
/// then the group draw news; out of memory the box of line 0x241, without the stage that of 0x25a (and 0).
///
/// `irish_league_cup_group_stage` (mwa_23; read instruction by instruction against IreLgeCup::create_group_stage: the
/// group size is fixed at 5 and the local place list holds 2; the pool scan tests the count before the first entry,
/// the same for a group of five).
/// Original: `NirLgeCup::create_group_stage` (0x78d3c0, nir_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    irish_league_cup_group_stage(League(this), g, 5, 2, 0xa69698, (0x241, 0x25a), "mwa_24 northern irish league cup group") // mwa24: two places through
}

/// Comparer for `nir_lge_cup_create_group_stage` (original only, the plan: `compare_irish_league_cup_group_stage`).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%);
    compare_irish_league_cup_group_stage("nir_lge_cup_create_group_stage", League(this), g, 5, || o(this, g))
}

/// Rule: in the four Northern Irish League Cup groups (stages below 3), a club going through gets the qualification
/// record (round 0x78, weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 3)
/// a club going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100
/// over 3 rounds), and the winner and the runner-up their final records.
///
/// `welsh_cup_staff_records` with 3 group stages and 3 knockout rounds (read instruction by instruction against
/// WelLgeCup::update_staff_records_for_status: only the group stage count, the knockout slot and the round count
/// change). Returns 0.
/// Original: `NirLgeCup::update_staff_records_for_status` (0x78d9c0, nir_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwa_24 northern irish league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 3, 3) // mwa24: the knockout is stage 3
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: return 0.
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("nir_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// Rule: the Northern Irish League Cup's predicted positions (at creation and at the turnover): the 20 entrants by
/// reputation; the knockout (stage 3) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers
/// of each earlier round; the rest in bands of 4; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 3, bands of 4 and the boxes at lines 0x3bc and 0x3c4 of
/// nir_lge_cup.cpp (read instruction by instruction against IreLgeCup::set_predicted_positions: the registers are
/// allocated differently; only the file, the lines, the stage and the band change).
/// Original: `NirLgeCup::set_predicted_positions` (0x78dcd0, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0xa69698, (0x3bc, 0x3c4), 3, 4, "mwa_24 northern irish league cup predicted") // mwa24: the Northern Irish League Cup bands of 4
}

/// Comparer for `nir_lge_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_italian_cup_predicted("nir_lge_cup_set_predicted_positions", this, nir_lge_cup_set_predicted_positions, || o(this));
}
