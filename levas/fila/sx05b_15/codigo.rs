// ---------- sx05b_15: the Turkish teams of the week (award/turkey_awards.cpp) ----------

/// Builds the team of the week of a Turkish team award: only for the Premier Division and second division teams of
/// the week (awards 2 and 4 of the table), `NationAwards::update_team_of_the_week` with the award's club competition
/// id.
/// Original: `TurkeyAwards::update_team_of_the_week` (0x902080, turkey_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn turkey_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[2, 4]);
}

/// Comparer for `turkey_awards_team_of_the_week`: the team of the week cannot be built twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_turkey_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, award)
}
