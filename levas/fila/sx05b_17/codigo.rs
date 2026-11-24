// ---------- sx05b_17: the MLS player of the month (award/usa_awards.cpp) ----------

/// The USA month: the MLS player of the month (award 0, a month award) from the nation's month ratings
/// (`award_month_player`), with no age limit (−1).
/// Original: `UsaAwards::on_month_change` (0x906f60, usa_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn usa_awards_on_month_change(this: usize) {
    covered!("sx05b_17 mls player of the month");
    award_month_player(this, nation_award(this, 0), -1);
}

/// Comparer for `usa_awards_on_month_change`: the award cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_usa_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
