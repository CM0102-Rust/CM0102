// ---------- sx04c_13: the Russian players of the month (award/russia_awards.cpp) ----------

/// The Russian month: except when the month ratings cover June (month 5), the player of the month of the Premier
/// Division (award 3) and of the First Division (award 6), each the best of the month ratings in its division with no
/// age limit (`award_month_best_player`). Never runs in the game: the Russian awards have no month ratings, so the base
/// daily update never calls it (defect of the original).
/// Original: `RussiaAwards::on_month_change` (0x7ec6c0, russia_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn russia_awards_on_month_change(this: usize) {
    if read_u8(read_usize(this + nation_awards_fields::MONTH_RATINGS) + month_ratings_fields::MONTH) == 5 {
        return;
    }
    covered!("sx04c_13 russian players of the month");
    award_month_best_player(this, nation_award(this, 3), KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, -1);
    award_month_best_player(this, nation_award(this, 6), KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION, -1);
}

/// Comparer for `russia_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_russia_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
