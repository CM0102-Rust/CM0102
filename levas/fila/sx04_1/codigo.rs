// ---------- sx04_1: the month change of a nation's awards, the month award's new season and the month ratings order (nation_awards.cpp, month_award.cpp, month_ratings.cpp) ----------

/// A nation's awards object (`NationAwards`, nation_awards.cpp): +0 vtable, +4 award table, +8 award count (i16), +0xa
/// month ratings (`MONTH_RATINGS`, 0 when the nation has none).
pub(crate) mod nation_awards_obj {
    pub(crate) const MONTH_RATINGS: usize = 0xa;
    /// The virtual `on_month_change()` (thiscall, no arguments): the month awards of the nation.
    pub(crate) const ON_MONTH_CHANGE_SLOT: usize = 0xc;
}

/// The month award's slot count (i8): how many of the twelve 11-byte month slots are in use.
const MONTH_AWARD_SLOT_COUNT: usize = 0xa1;
/// One month slot of a month award (12 of them from `month_award_fields::SLOT_WINNER_ID`).
const MONTH_AWARD_SLOT_SIZE: usize = 11;

/// `MONTH_RATINGS::reset(month, year)` (0x767f60, thiscall, ret 8); the month and the year go as the original pushes
/// them, in whole dwords.
const FN_MONTH_RATINGS_RESET: usize = 0x767f60;

/// The month and the year (with the leap-year word above it, as the original pushes it) of tomorrow, when tomorrow is
/// in another month than the one the nation's month ratings cover; `None` when it is the same month or the nation has
/// no month ratings.
unsafe fn nation_awards_new_month(this: usize, today: usize) -> Option<(u32, u32)> {
    let mut tomorrow = [0u8; 8];
    exe::date_add_days(today, tomorrow.as_mut_ptr() as usize, 1);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    if ratings == 0 {
        return None;
    }
    let month = exe::date_get_month(tomorrow.as_ptr() as usize);
    let year = read_u16(tomorrow.as_ptr() as usize + cm_date::YEAR);
    // the ratings' year is signed, tomorrow's is not
    if read_u8(ratings + month_ratings_fields::MONTH) == month && read_i16(ratings + month_ratings_fields::YEAR) as i32 == year as i32 {
        return None;
    }
    Some((month as u32, read_u32(tomorrow.as_ptr() as usize + cm_date::YEAR)))
}

/// Closes a nation's month on its last day: when tomorrow (`today` + 1 day) falls in another month or year than the
/// one the nation's month ratings cover, it runs the nation's month awards (the virtual `on_month_change`, slot +0xc:
/// player, young player and manager of the month) and then empties the month ratings for tomorrow's month and year
/// (`MONTH_RATINGS::reset`, the ratings pointer read again after the awards). A nation without month ratings does
/// nothing. The base of every nation's `daily_update`, called at its end with the day being played.
/// Original: `NationAwards::daily_update` (0x768310, nation_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_awards_daily_update(this: usize, today: usize) {
    let Some((month, year)) = nation_awards_new_month(this, today) else {
        return;
    };
    covered!("sx04_1 month change");
    let on_month_change: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + nation_awards_obj::ON_MONTH_CHANGE_SLOT));
    on_month_change(this); // sx04_1: the month awards
    let reset: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_MONTH_RATINGS_RESET);
    reset(read_usize(this + nation_awards_obj::MONTH_RATINGS), month, year); // sx04_1: the new month's ratings
}

/// Comparer for `nation_awards_daily_update`.
///
/// The month awards and the reset cannot run twice: only the original runs. The copy's decision is worked out first
/// (pure: tomorrow's date and the ratings' month); then, on a month change, the ratings must be empty and cover
/// tomorrow's month and year; otherwise their month and year must be unchanged.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_daily_update(this: usize, today: usize) {
    let plan = nation_awards_new_month(this, today);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    let before = if ratings != 0 { (read_u8(ratings + month_ratings_fields::MONTH), read_i16(ratings + month_ratings_fields::YEAR)) } else { (0, 0) };
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, today);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    if ratings == 0 {
        SCOREBOARD.record(plan.is_none(), || format!("nation_awards_daily_update(today {:#x}): no month ratings, rust planned {:?}", read_u32(today), plan));
        return;
    }
    let after = (read_u8(ratings + month_ratings_fields::MONTH), read_i16(ratings + month_ratings_fields::YEAR));
    let ok = match plan {
        Some((m, y)) => after == (m as u8, y as u16 as i16) && read_u32(ratings + 4) == 0,
        None => after == before,
    };
    SCOREBOARD.record(ok, || format!("nation_awards_daily_update(today {:#x}): month/year before {:?} after {:?}, rust planned {:?}", read_u32(today), before, after, plan));
}

/// Moves a month award on to the next season: unless the first month slot is already in `year` (or no slot is in
/// use), every used slot (`MONTH_AWARD_SLOT_COUNT`, read again at each step) goes one year on
/// (`month_award_fields::SLOT_YEAR` + 1) and loses its winner and club (`SLOT_WINNER_ID`, `SLOT_CLUB_ID` = −1). Called
/// by each nation's `daily_update` on the nation's season start day, with the current year.
/// Original: `MONTH_AWARD::start_new_season` (0x7672e0, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_new_season(this: usize, year: u16) {
    if read_i8(this + MONTH_AWARD_SLOT_COUNT) <= 0 || read_u16(this + month_award_fields::SLOT_YEAR) == year {
        return;
    }
    covered!("sx04_1 month slots moved on");
    let mut k = 0;
    while k < read_i8(this + MONTH_AWARD_SLOT_COUNT) as i32 {
        let slot = this + k as usize * MONTH_AWARD_SLOT_SIZE;
        write_u16(slot + month_award_fields::SLOT_YEAR, read_u16(slot + month_award_fields::SLOT_YEAR).wrapping_add(1)); // sx04_1: one year on
        write_i32(slot + month_award_fields::SLOT_WINNER_ID, -1);
        write_i32(slot + month_award_fields::SLOT_CLUB_ID, -1);
        k += 1;
    }
}

/// Comparer for `month_award_new_season`: side by side over the twelve month slots.
pub(crate) unsafe extern "thiscall" fn compare_month_award_new_season(this: usize, year: u16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u16)>(%%I1%%);
    let regions = [(this + month_award_fields::SLOT_WINNER_ID, 12 * MONTH_AWARD_SLOT_SIZE)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, year), || month_award_new_season(this, year));
    SCOREBOARD.record(same, || format!("month_award_new_season(year {}) original={:x?} rust={:x?}", year, mo, mr));
}

/// The qsort order of the month ratings (12-byte `month_rating_entry` records): the best average rating first, with
/// those of fewer than 3 matches after all the others. `a` with fewer than 3 matches: 1 when `b` has at least 3, else 0;
/// `b` with fewer than 3 (and `a` not): −1; otherwise the averages (the rating sum, i16 at +8, / `month_rating_entry::MATCHES`, signed)
/// decide: −1 when `a`'s is higher, 1 when lower, 0 when equal. Used by `find_best_player`.
/// Original: `month_ratings_compare_average` (0x768200, month_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn month_ratings_compare_average(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (ma, mb) = (read_i8(a + month_rating_entry::MATCHES), read_i8(b + month_rating_entry::MATCHES));
    if ma < 3 { // sx04_1: fewer than three matches go last
        return (mb >= 3) as i32;
    }
    if mb < 3 {
        return -1;
    }
    covered!("sx04_1 averages compared");
    let average = |e: usize, m: i8| read_i16(e + 8) as i32 as f64 / m as i32 as f64; // +8: the rating sum
    let (va, vb) = (average(a, ma), average(b, mb));
    if va > vb {
        -1
    } else if va < vb {
        1
    } else {
        0
    }
}

compare_order!(compare_month_ratings_compare_average, month_ratings_compare_average, %%I2%%);
