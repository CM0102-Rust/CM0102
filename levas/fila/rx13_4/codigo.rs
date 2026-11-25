// ---------- rx13_4: the Argentine awards' team of the week (award/argentina_awards.cpp) ----------

/// rule: each week, the team of the week of the Argentine first and second divisions, when the award asked for is one
/// of the two
///
/// `nation_awards_team_of_the_week_of` with the slots 0 and 1: when `award` is one of them, the team of the week of its
/// competition (`build_team_of_the_week`); any other award, nothing.
/// Original: `ArgentinaAwards::update_team_of_the_week` (0x40a560, argentina_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn argentina_awards_update_team_of_the_week(this: usize, award: usize) {
    covered!("rx13_4 argentine team of the week");
    nation_awards_team_of_the_week_of(this, award, &[0, 1]) // rx13_4: both divisions
}

/// Only the original (the team of the week is chosen once).
pub(crate) unsafe extern "thiscall" fn compare_argentina_awards_update_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, award)
}
