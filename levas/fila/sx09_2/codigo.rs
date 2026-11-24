// ---------- batch %%LEVA%%: Date.cpp: the day of the month, the month and the leap year (sx09_2) ----------

/// Returns the day of the month of a date (in ax; the caller reads the low word).
///
/// `Date::get_day_of_month()`: searching from December back, the first month whose days before it
/// (`days_before_month` in the date's year kind) are fewer than the day of the year: the day of the year minus those
/// days. None (a day of -1 or less): -1. The high word is what the original leaves in eax: the high word of the leap
/// flag, minus (when a month matched) the high word of the table pointer (0x9b), which the callers never read.
/// Original: `Date::get_day_of_month` (0x54cf70, Date.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn date_get_day_of_month(this: *const GameDate) -> u32 {
    let d = &*this;
    let high = d.is_leap_year as u32 & 0xffff_0000;
    match date_month_index(d) {
        Some(month) => {
            covered!("sx09_2 day of month");
            let before = days_before_month(d.is_leap_year != 0, month) as u16 as u32;
            (high | d.day as u16 as u32).wrapping_sub(0x009b_0000 | before) // sx09_2: the day of the month
        }
        None => high | 0xffff,
    }
}

/// Returns the month of a date, 0 for January to 11 for December, or -1 (all of eax) when none matches.
///
/// `Date::get_month()`: searching from December back, the first month whose days before it (`days_before_month`)
/// are fewer than the date's day of the year (`date_month_index`).
/// Original: `Date::get_month` (0x54cfb0, Date.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn date_get_month(this: *const GameDate) -> u32 {
    covered!("sx09_2 month");
    match date_month_index(&*this) {
        Some(month) => month as u32, // sx09_2: the month
        None => u32::MAX,
    }
}

/// Returns 1 when a year (the low 16 bits) is a Gregorian leap year and 0 otherwise (`crate::date::is_leap_year`: odd
/// or not divisible by 4, no; not divisible by 100, yes; otherwise only when divisible by 400).
/// Original: `date_is_leap_year` (0x54cfe0, Date.cpp, cdecl).
pub(crate) unsafe extern "C" fn date_is_leap_year(year: u32) -> u32 {
    covered!("sx09_2 leap year");
    crate::date::is_leap_year(year as u16) as u32 // sx09_2: the Gregorian rule
}

/// Comparer for `date_get_day_of_month`: the whole eax, side by side (the routine only reads).
pub(crate) unsafe extern "thiscall" fn compare_date_get_day_of_month(this: *const GameDate) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(*const GameDate) -> u32>(%%I0%%)(this);
    let n = date_get_day_of_month(this);
    SCOREBOARD.record(r == n, || format!("date_get_day_of_month({:?}) original {:#x} rust {:#x}", *this, r, n));
    r
}

/// Comparer for `date_get_month`: the return, side by side.
pub(crate) unsafe extern "thiscall" fn compare_date_get_month(this: *const GameDate) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(*const GameDate) -> u32>(%%I1%%)(this);
    let n = date_get_month(this);
    SCOREBOARD.record(r == n, || format!("date_get_month({:?}) original {:#x} rust {:#x}", *this, r, n));
    r
}

/// Comparer for `date_is_leap_year`: the return, side by side.
pub(crate) unsafe extern "C" fn compare_date_is_leap_year(year: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I2%%)(year);
    let n = date_is_leap_year(year);
    SCOREBOARD.record(r == n, || format!("date_is_leap_year({}) original {} rust {}", year as u16, r, n));
    r
}
