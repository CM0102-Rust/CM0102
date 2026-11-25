// ---------- rx09_15: the Brazilian team of the week choice (award/brazil_awards.cpp) ----------

/// The Brazilian teams of the week: the First and Second Divisions (awards 0 and 5) and the twelve state
/// championships (10 + 4k).
const BRAZIL_TEAMS_OF_THE_WEEK: [usize; 14] = [0, 5, 10, 14, 18, 22, 26, 30, 34, 38, 42, 46, 50, 54];

/// Builds the team of the week of a Brazilian team award: only for the teams of the week (`BRAZIL_TEAMS_OF_THE_WEEK`,
/// compared in that order), `build_team_of_the_week` with the award's competition.
/// Original: `BrazilAwards::update_team_of_the_week` (0x43f070, brazil_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn brazil_awards_update_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &BRAZIL_TEAMS_OF_THE_WEEK);
}

/// Comparer for `brazil_awards_update_team_of_the_week` (`compare_team_of_the_week_of`).
pub(crate) unsafe extern "thiscall" fn compare_brazil_awards_update_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    compare_team_of_the_week_of(this, award, &BRAZIL_TEAMS_OF_THE_WEEK, || o(this, award), "brazil_awards_update_team_of_the_week");
}
