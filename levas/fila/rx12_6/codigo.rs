// ---------- rx12_6: the Australian month awards (award/australia_awards.cpp) ----------

/// The Australian month: the National Soccer League manager of the month (award 1, `award_month_manager`), then the
/// player of the month (award 2, no age limit) and the young player of the month (award 3, up to 21) from the nation's
/// month ratings (`award_month_player`).
/// Original: `AustraliaAwards::on_month_change` (0x412c20, australia_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn australia_awards_on_month_change(this: usize) {
    covered!("rx12_6 australian month awards");
    award_month_manager(nation_award(this, 1), KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE); // rx12_6: the manager of the month
    award_month_player(this, nation_award(this, 2), -1);
    award_month_player(this, nation_award(this, 3), 0x15); // rx12_6: the young player, up to 21
}

/// Comparer for `australia_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_australia_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
