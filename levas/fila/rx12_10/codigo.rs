// ---------- rx12_10: the Belgian, Dutch and Danish teams of the week (award/belgium_awards.cpp, holland_awards.cpp, denmark_awards.cpp) ----------

/// Builds the team of the week of a Belgian team award: only for the three divisions' teams of the week (awards 2, 6
/// and 9 of the table).
/// Original: `BelgiumAwards::update_team_of_the_week` (0x424b60, belgium_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn belgium_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[2, 6, 9]);
}

/// Builds the team of the week of a Dutch team award: only for the two divisions' teams of the week (awards 3 and 7).
/// Original: `HollandAwards::update_team_of_the_week` (0x5f9670, holland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn holland_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[3, 7]);
}

/// Builds the team of the week of a Danish team award: only for the three divisions' teams of the week (awards 1, 4
/// and 7).
/// Original: `DenmarkAwards::update_team_of_the_week` (0x555430, denmark_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn denmark_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[1, 4, 7]);
}

/// Comparer for `belgium_awards_team_of_the_week`: the team of the week cannot be built twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_belgium_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, award)
}

/// Comparer for `holland_awards_team_of_the_week`: only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_holland_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, award)
}

/// Comparer for `denmark_awards_team_of_the_week`: only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_denmark_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, award)
}
