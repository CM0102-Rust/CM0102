// ---------- rx11_8: the French awards' team of the week (award/france_awards.cpp) ----------

/// rule: the team of the week is built for the team of the week award of the French First Division, Second Division
/// or National
///
/// `FranceAwards::update_team_of_the_week(award)` (virtual +0x10): when `award` is award 6, 11 or 16 of the table,
/// `build_team_of_the_week` with the award's competition (`nation_awards_team_of_the_week_of`).
/// Original: `FranceAwards::update_team_of_the_week` (0x5c1540, france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn france_awards_update_team_of_the_week(this: usize, award: usize) {
    covered!("rx11_8 french team of the week");
    nation_awards_team_of_the_week_of(this, award, &[6, 11, 16]); // rx11_8: the three divisions' team awards
}

/// Comparer for `france_awards_update_team_of_the_week`: the team of the week is chosen once; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_france_awards_update_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, award)
}
