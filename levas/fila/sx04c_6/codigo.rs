// ---------- sx04c_6: the Northern Irish managers of the month (award/northern_ireland_awards.cpp) ----------

/// The Northern Irish month: the manager of the month (`award_month_manager`) of the Premier Division (award 6) and of
/// the First Division (award 12).
/// Original: `NorthernIrelandAwards::on_month_change` (0x793630, northern_ireland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn northern_ireland_awards_on_month_change(this: usize) {
    covered!("sx04c_6 northern irish managers of the month");
    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION);
    award_month_manager(nation_award(this, 12), KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION);
}

/// Comparer for `northern_ireland_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_northern_ireland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
