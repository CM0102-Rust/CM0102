// ---------- rx12_13: the Greek and Croatian teams of the week (award/greece_awards.cpp, croatia_awards.cpp) ----------

/// Builds the team of the week of a Greek team award: only for the two divisions' teams of the week (awards 2 and 6 of
/// the table).
/// Original: `GreeceAwards::update_team_of_the_week` (0x5ece90, greece_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn greece_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[2, 6]); // rx12_13: the Greek teams of the week
}

/// Builds the team of the week of a Croatian team award: only for the three divisions' teams of the week (awards 0, 3
/// and 6).
/// Original: `CroatiaAwards::update_team_of_the_week` (0x5184d0, croatia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn croatia_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[0, 3, 6]); // rx12_13: the Croatian teams of the week
}

/// Comparer for `greece_awards_team_of_the_week`: the team of the week cannot be built twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_greece_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, award)
}

/// Comparer for `croatia_awards_team_of_the_week`: only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_croatia_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, award)
}
