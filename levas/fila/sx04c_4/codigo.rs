// ---------- sx04c_4: the South Korean player of the month (award/korea_awards.cpp) ----------

/// The South Korean month: the player of the month (award 1, a month award) from the nation's month ratings
/// (`award_month_player`), with no age limit (−1).
/// Original: `KoreaAwards::on_month_change` (0x66e610, korea_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn korea_awards_on_month_change(this: usize) {
    covered!("sx04c_4 korean player of the month");
    award_month_player(this, nation_award(this, 1), -1);
}

/// Comparer for `korea_awards_on_month_change`: the award cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_korea_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
