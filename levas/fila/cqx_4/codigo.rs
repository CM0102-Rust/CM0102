// ---------- batch %%LEVA%%: the predicted positions of the state leagues: Goiás state league, Minas Gerais state league, North league (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp) ----------

/// The Goiás state league: the fixed places 1, 2, 3, 3, 5, 5, 5, 5 (the quarter-finals).
const GOIAS_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98ae90, lines: [0x304, 0x30c], places: &[1, 2, 3, 3, 5, 5, 5, 5], label: "cqx_4 goiás predicted" }; // cqx4: the goiás places

/// The predicted positions of the Goiás state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, 5, 5, 5, 5, and the others their place in the order (`state_league_predicted_positions`
/// with `GOIAS_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x304 (no statistics) and 0x30c (no memory).
/// Original: `BraRegGoias::set_predicted_positions` (0x433b40, bra_reg_goias.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_goias_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &GOIAS_PREDICTED)
}

/// Comparer for `bra_reg_goias_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_goias_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_state_league_predicted("bra_reg_goias_set_predicted_positions", this, &GOIAS_PREDICTED, || o(this));
}

/// The Minas Gerais state league: the fixed places 1, 2 (the final between the top two).
const MINAS_GERAIS_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98aed0, lines: [0x33f, 0x347], places: &[1, 2], label: "cqx_4 minas gerais predicted" }; // cqx4: the minas gerais places

/// The predicted positions of the Minas Gerais state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, and the others their place in the order (`state_league_predicted_positions`
/// with `MINAS_GERAIS_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x33f (no statistics) and 0x347 (no memory).
/// Original: `BraRegMinasGerais::set_predicted_positions` (0x434f80, bra_reg_minas_gerais.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_minas_gerais_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &MINAS_GERAIS_PREDICTED)
}

/// Comparer for `bra_reg_minas_gerais_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_state_league_predicted("bra_reg_minas_gerais_set_predicted_positions", this, &MINAS_GERAIS_PREDICTED, || o(this));
}

/// The North league: the fixed places 1, 2, 3, 3 (the semi-finals).
const NORTH_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98af14, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_4 north predicted" }; // cqx4: the north places

/// The predicted positions of the North league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `NORTH_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegNorth::set_predicted_positions` (0x436280, bra_reg_north.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_north_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &NORTH_PREDICTED)
}

/// Comparer for `bra_reg_north_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_north_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_predicted("bra_reg_north_set_predicted_positions", this, &NORTH_PREDICTED, || o(this));
}
