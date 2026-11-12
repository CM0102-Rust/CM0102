// ---------- batch %%LEVA%%: the predicted positions of the state leagues: Rio state league, Santa Catarina state league, São Paulo state league (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

/// The Rio state league: the fixed places 1, 2 (the final between the top two).
const RIO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98b014, lines: [0x33f, 0x347], places: &[1, 2], label: "cqx_6 rio predicted" }; // cqx6: the rio places

/// The predicted positions of the Rio state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, and the others their place in the order (`state_league_predicted_positions`
/// with `RIO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x33f (no statistics) and 0x347 (no memory).
/// Original: `BraRegRio::set_predicted_positions` (0x43add0, bra_reg_rio.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_rio_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &RIO_PREDICTED)
}

/// Comparer for `bra_reg_rio_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_rio_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_state_league_predicted("bra_reg_rio_set_predicted_positions", this, &RIO_PREDICTED, || o(this));
}

/// The Santa Catarina state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const SANTA_CATARINA_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98b050, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_6 santa catarina predicted" }; // cqx6: the santa catarina places

/// The predicted positions of the Santa Catarina state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `SANTA_CATARINA_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegSanta::set_predicted_positions` (0x43c110, bra_reg_santa.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_santa_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &SANTA_CATARINA_PREDICTED)
}

/// Comparer for `bra_reg_santa_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_santa_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_state_league_predicted("bra_reg_santa_set_predicted_positions", this, &SANTA_CATARINA_PREDICTED, || o(this));
}

/// The São Paulo state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const SAO_PAULO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98b090, lines: [0x2f9, 0x301], places: &[1, 2, 3, 3], label: "cqx_6 são paulo predicted" }; // cqx6: the são paulo places

/// The predicted positions of the São Paulo state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `SAO_PAULO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x2f9 (no statistics) and 0x301 (no memory).
/// Original: `BraRegSp::set_predicted_positions` (0x43d140, bra_reg_sp.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_sp_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &SAO_PAULO_PREDICTED)
}

/// Comparer for `bra_reg_sp_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_sp_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_predicted("bra_reg_sp_set_predicted_positions", this, &SAO_PAULO_PREDICTED, || o(this));
}
