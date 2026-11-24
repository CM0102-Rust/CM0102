// ---------- sx04c_18: the old Finnish player of the month (award/old_finland_awards.cpp) ----------

/// The old Finnish month: the Premier Division player of the month (award 5) from the nation's month ratings
/// (`award_month_player`), with no age limit.
/// Original: `OldFinlandAwards::on_month_change` (0x79dcb0, old_finland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_on_month_change(this: usize) {
    covered!("sx04c_18 old finnish player of the month");
    award_month_player(this, nation_award(this, 5), -1);
}

/// Comparer for `old_finland_awards_on_month_change`: the award cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
