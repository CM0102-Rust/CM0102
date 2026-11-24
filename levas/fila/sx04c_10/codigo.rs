// ---------- sx04c_10: the Polish managers of the month (award/poland_awards.cpp) ----------

/// The Polish month: the manager of the month (`award_month_manager`) of the First Division (award 4) and of the
/// Second Division (award 9). Never runs in the game: the Polish awards have no month ratings, so the base daily update
/// never calls it (defect of the original).
/// Original: `PolandAwards::on_month_change` (0x7cd790, poland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn poland_awards_on_month_change(this: usize) {
    covered!("sx04c_10 polish managers of the month");
    award_month_manager(nation_award(this, 4), KEY_CLUB_COMP_POLISH_FIRST_DIVISION);
    award_month_manager(nation_award(this, 9), KEY_CLUB_COMP_POLISH_SECOND_DIVISION);
}

/// Comparer for `poland_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_poland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
