// ---------- rx11_10: the Irish awards' month change and team of the week (award/ireland_awards.cpp) ----------

/// rule: at each month change Ireland gives the player of the month from the month ratings and the manager of the month
/// of the Premier and First Divisions
///
/// `IrelandAwards::on_month_change()` (virtual +0xc, from `NationAwards::daily_update` when the month changes): the
/// player of the month (award 3, `award_month_player` with no age limit), then the managers of the month of the Premier
/// (award 10) and First Division (award 17) (`award_month_manager`).
/// Original: `IrelandAwards::on_month_change` (0x63c160, ireland_awards.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ireland_awards_on_month_change(this: usize) {
    covered!("rx11_10 irish month awards");
    award_month_player(this, nation_award(this, 3), -1);
    award_month_manager(nation_award(this, 10), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION); // rx11_10: the Premier's manager of the month
    award_month_manager(nation_award(this, 17), KEY_CLUB_COMP_IRISH_FIRST_DIVISION);
}

/// rule: the team of the week is built for the team of the week award of the Irish Premier or First Division
///
/// `IrelandAwards::update_team_of_the_week(award)` (virtual +0x10): when `award` is award 4 or 11 of the table,
/// `build_team_of_the_week` (`nation_awards_team_of_the_week_of`).
/// Original: `IrelandAwards::update_team_of_the_week` (0x63c1f0, ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ireland_awards_update_team_of_the_week(this: usize, award: usize) {
    covered!("rx11_10 irish team of the week");
    nation_awards_team_of_the_week_of(this, award, &[4, 11]); // rx11_10: the two divisions' team awards
}

/// Comparers: the month awards and the team of the week are given once; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_ireland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_ireland_awards_update_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, award)
}
