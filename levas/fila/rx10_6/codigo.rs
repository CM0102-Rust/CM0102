// ---------- rx10_6: the German Regionalliga predicted positions and season turnover (ger_regional.cpp) ----------

/// Rule: the Regionalliga's predicted positions: each section's clubs by reputation, the best first, placed 1, 3, 5,
/// ... (both sections get the same odd places).
///
/// `section_predicted_positions` with one section stage, step 2 and the boxes of ger_regional.cpp at lines 0x275 (no
/// statistics) and 0x27d (no memory).
/// Original: `GerRegional::set_predicted_positions` (0x5de130, ger_regional.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_regional_set_predicted_positions(this: usize) {
    covered!("rx10_6 regionalliga predicted");
    section_predicted_positions(this, 1, 2, 0x9cd488, [0x275, 0x27d]) // rx10_6: the odd places
}

/// Comparer for `ger_regional_set_predicted_positions`: the original, then the copy (`compare_section_predicted`).
pub(crate) unsafe extern "thiscall" fn compare_ger_regional_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_section_predicted("ger_regional_set_predicted_positions", this, 1, || o(this), || ger_regional_set_predicted_positions(this))
}

/// Rule: at the season turn the Regionalliga archives the season, clears the North table, the dates and the South
/// stage, moves the year on and rebuilds the rules, the North table, the order, the fixtures and the South stage;
/// then the predicted positions and the referee renewal.
///
/// `divided_league_turnover` with `GER_REGIONAL` and the boxes of ger_regional.cpp at lines 0x1b2, 0x1b9, 0x1c0 and
/// 0x1c6.
/// Original: `GerRegional::start_new_season` (0x5dda90, ger_regional.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_regional_start_new_season(this: usize) -> u32 {
    covered!("rx10_6 regionalliga turnover");
    divided_league_turnover(this, &GER_REGIONAL, [0x1b2, 0x1b9, 0x1c0, 0x1c6])
}

/// Comparer for `ger_regional_start_new_season`: only the original (it frees and rebuilds the league); the year must
/// go up by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_ger_regional_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("ger_regional_start_new_season", League(this), || o(this))
}
