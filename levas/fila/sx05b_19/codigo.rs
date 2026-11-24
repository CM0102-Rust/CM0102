// ---------- sx05b_19: the Welsh manager of the month and team of the week (award/wales_awards.cpp) ----------

/// The Welsh month: the manager of the month of the Welsh Premier Division (award 5, `award_month_manager`).
/// Original: `WalesAwards::on_month_change` (0x90f020, wales_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn wales_awards_on_month_change(this: usize) {
    covered!("sx05b_19 welsh manager of the month");
    award_month_manager(nation_award(this, 5), KEY_CLUB_COMP_WELSH_PREMIER_DIVISION);
}

/// Builds the team of the week of a Welsh team award: only for the team of the week (award 0 of the table).
/// Original: `WalesAwards::update_team_of_the_week` (0x90f060, wales_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wales_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[0]);
}

/// Comparer for `wales_awards_on_month_change`: the award cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_wales_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Comparer for `wales_awards_team_of_the_week`: the team of the week cannot be built twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_wales_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, award)
}
