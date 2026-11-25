// ---------- rx12_2: the Finnish month awards and team of the week (award/finland_awards.cpp) ----------

/// The Finnish month: the managers of the month of the Premier Division (award 6) and of the First Division (award 11,
/// `award_month_manager`), then the players of the month of both (awards 5 and 10, `award_month_player`, no age limit).
/// Original: `FinlandAwards::on_month_change` (0x5a9890, finland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn finland_awards_on_month_change(this: usize) {
    covered!("rx12_2 finnish month awards");
    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION); // rx12_2: the Premier Division manager
    award_month_manager(nation_award(this, 11), KEY_CLUB_COMP_FINNISH_FIRST_DIVISION);
    award_month_player(this, nation_award(this, 5), -1); // rx12_2: the Premier Division player
    award_month_player(this, nation_award(this, 10), -1);
}

/// Builds the team of the week of a Finnish team award: only for the two teams of the week (awards 8 and 13 of the
/// table; the teams of the year, 7 and 12, are not).
/// Original: `FinlandAwards::update_team_of_the_week` (0x5a9930, finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn finland_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[8, 13]);
}

/// Comparer for `finland_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_finland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Comparer for `finland_awards_team_of_the_week`: the team of the week cannot be built twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_finland_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, award)
}
