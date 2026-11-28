use super::*;

/// The old Irish month: the manager of the month (`award_month_manager`) of the Premier Division (award 9) and of the
/// First Division (award 16).
/// Original: `OldIrelandAwards::on_month_change` (0x79f490, old_ireland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn old_ireland_awards_on_month_change(this: usize) {
    covered!("sx04c_22 old irish managers of the month");
    award_month_manager(nation_award(this, 9), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION);
    award_month_manager(nation_award(this, 16), KEY_CLUB_COMP_IRISH_FIRST_DIVISION);
}

/// Comparer for `old_ireland_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_ireland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OLD_IRELAND_AWARDS_ON_MONTH_CHANGE)(this)
}
