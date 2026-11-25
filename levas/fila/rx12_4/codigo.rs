// ---------- rx12_4: the German players of the month (award/germany_awards.cpp) ----------

/// The German month: the First Division player of the month (award 7) with no age limit and the Second Division's
/// (award 11) up to 20 (`award_month_player`, from the nation's month ratings).
/// Original: `GermanyAwards::on_month_change` (0x5dff40, germany_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn germany_awards_on_month_change(this: usize) {
    covered!("rx12_4 german players of the month");
    award_month_player(this, nation_award(this, 7), -1); // rx12_4: the First Division player
    award_month_player(this, nation_award(this, 11), 0x14); // rx12_4: the Second Division player, up to 20
}

/// Comparer for `germany_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_germany_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
