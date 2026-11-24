// ---------- sx04c_8: the Norwegian managers and players of the month (award/norway_awards.cpp) ----------

/// The Norwegian month: the manager of the month (`award_month_manager`) of the Premier Division (award 6) and of the
/// First Division (award 9), then the player of the month of each (awards 7 and 10) from the nation's month ratings
/// (`award_month_player`, no age limit).
/// Original: `NorwayAwards::on_month_change` (0x794430, norway_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn norway_awards_on_month_change(this: usize) {
    covered!("sx04c_8 norwegian month awards");
    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION);
    award_month_manager(nation_award(this, 9), KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION);
    award_month_player(this, nation_award(this, 7), -1);
    award_month_player(this, nation_award(this, 10), -1);
}

/// Comparer for `norway_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_norway_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
