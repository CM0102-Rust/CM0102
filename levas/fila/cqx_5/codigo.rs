// ---------- batch %%LEVA%%: the predicted positions of the state leagues: Northeast league, Paraná state league, Pernambuco state league (bra_reg_northeast.cpp, bra_reg_parana.cpp, bra_reg_pern.cpp) ----------

/// The Northeast league: the fixed places 1, 2, 3, 3 (the semi-finals).
const NORTHEAST_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98af54, lines: [0x2f9, 0x301], places: &[1, 2, 3, 3], label: "cqx_5 northeast predicted" }; // cqx5: the northeast places

/// The predicted positions of the Northeast league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `NORTHEAST_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x2f9 (no statistics) and 0x301 (no memory).
/// Original: `BraRegNortheast::set_predicted_positions` (0x437290, bra_reg_northeast.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_northeast_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &NORTHEAST_PREDICTED)
}

/// Comparer for `bra_reg_northeast_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_northeast_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_state_league_predicted("bra_reg_northeast_set_predicted_positions", this, &NORTHEAST_PREDICTED, || o(this));
}

/// The Paraná state league: the fixed places 1, 2 (the final between the top two).
const PARANA_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98af98, lines: [0x33f, 0x347], places: &[1, 2], label: "cqx_5 paraná predicted" }; // cqx5: the paraná places

/// The predicted positions of the Paraná state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, and the others their place in the order (`state_league_predicted_positions`
/// with `PARANA_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x33f (no statistics) and 0x347 (no memory).
/// Original: `BraRegParana::set_predicted_positions` (0x438590, bra_reg_parana.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_parana_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &PARANA_PREDICTED)
}

/// Comparer for `bra_reg_parana_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_parana_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_state_league_predicted("bra_reg_parana_set_predicted_positions", this, &PARANA_PREDICTED, || o(this));
}

/// The Pernambuco state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const PERNAMBUCO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98afd8, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_5 pernambuco predicted" }; // cqx5: the pernambuco places

/// The predicted positions of the Pernambuco state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `PERNAMBUCO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegPern::set_predicted_positions` (0x4398d0, bra_reg_pern.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_pern_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &PERNAMBUCO_PREDICTED)
}

/// Comparer for `bra_reg_pern_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_pern_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_predicted("bra_reg_pern_set_predicted_positions", this, &PERNAMBUCO_PREDICTED, || o(this));
}
