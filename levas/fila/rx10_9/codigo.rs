// ---------- rx10_9: the Portuguese Segunda B predicted positions and season turnover (por_second_b.cpp) ----------

/// Rule: the Segunda B's predicted positions: each zone's clubs by reputation, the best first, placed 1, 4, 7, ...
/// (the three zones get the same places).
///
/// `section_predicted_positions` with two zone stages, step 3 and the boxes of por_second_b.cpp at lines 0x2b7 (no
/// statistics) and 0x2bf (no memory).
/// Original: `PorSecondB::set_predicted_positions` (0x7d2cd0, por_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_b_set_predicted_positions(this: usize) {
    covered!("rx10_9 segunda b predicted");
    section_predicted_positions(this, 2, 3, 0xa78798, [0x2b7, 0x2bf]) // rx10_9: places 1, 4, 7...
}

/// Comparer for `por_second_b_set_predicted_positions`: the original, then the copy (`compare_section_predicted`).
pub(crate) unsafe extern "thiscall" fn compare_por_second_b_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_section_predicted("por_second_b_set_predicted_positions", this, 2, || o(this), || por_second_b_set_predicted_positions(this))
}

/// Rule: at the season turn the Segunda B archives the season, clears the North table, the dates and the zone stages,
/// moves the year on and rebuilds the rules, the North table, the order, the fixtures and the two zones; then the
/// predicted positions and the referee renewal.
///
/// `divided_league_turnover` with `SEGUNDA_B` and the boxes of por_second_b.cpp at lines 0x1fe, 0x205, 0x20c and
/// 0x212 (the zone index pushed clean).
/// Original: `PorSecondB::start_new_season` (0x7d26b0, por_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_b_start_new_season(this: usize) -> u32 {
    covered!("rx10_9 segunda b turnover");
    divided_league_turnover(this, &SEGUNDA_B, [0x1fe, 0x205, 0x20c, 0x212])
}

/// Comparer for `por_second_b_start_new_season`: only the original (it frees and rebuilds the league); the year must go
/// up by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_por_second_b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("por_second_b_start_new_season", League(this), || o(this))
}
