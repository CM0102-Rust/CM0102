use super::*;

/// Sets a date from the day of the month, the month and the year, and moves it to the nearest given weekday.
///
/// `Date::set(day, month, year, weekday)`: a day of the month outside 1..31 (read as `i16`) gives the error box
/// (Date.cpp, line 0x72) and becomes 1; a month outside 0..11 (read as a signed byte) gives the error box (line 0x79)
/// and becomes January. The year (16 bits) and its Gregorian leap flag (`crate::date::is_leap_year`) are stored, and
/// the day of the year is the days before the month (`days_before_month`) plus the day of the month. A weekday other
/// than 0xffff (16 bits) moves the date to the nearest one, three days either way (`exe::date_get_nearest_weekday` on
/// the date itself). Returns what the original leaves in eax: the weekday argument when it is 0xffff, otherwise the
/// adjustment's return (no caller reads it). The same as the constructor `Date::Date` (0x549f70) but for the error
/// lines.
/// Original: `Date::set` (0x54a130, Date.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn date_set(this: *mut GameDate, day: u32, month: u32, year: u32, weekday: u32) -> u32 {
    covered!("sx09_1 date set");
    let mut day = day as i16;
    if !(1..=31).contains(&day) {
        crate::crt::error_box(DATE_CPP, 0x72);
        day = 1;
    }
    let mut month = month as u8 as i8;
    if !(0..=11).contains(&month) {
        crate::crt::error_box(DATE_CPP, 0x79);
        month = 0;
    }
    let year = year as u16;
    let leap = crate::date::is_leap_year(year) != 0;
    (*this).year = year;
    (*this).is_leap_year = leap as i32;
    (*this).day = days_before_month(leap, month as usize).wrapping_add(day); // sx09_1: the day of the year
    if weekday as u16 == 0xffff {
        return weekday;
    }
    exe::date_get_nearest_weekday(this as usize, weekday, this as usize)
}

/// Comparer for `date_set`: the date and `Date::sub_days`' static side by side (the return is the adjustment's leftover
/// and is not compared). A bad day or month: only the original (the error box).
pub(crate) unsafe extern "thiscall" fn compare_date_set(this: *mut GameDate, day: u32, month: u32, year: u32, weekday: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*mut GameDate, u32, u32, u32, u32) -> u32>(IDX_DATE_SET);
    if !(1..=31).contains(&(day as i16)) || !(0..=11).contains(&(month as u8 as i8)) {
        return o(this, day, month, year, weekday);
    }
    let regions = [(this as usize, 8), (SUB_DAYS_RESULT, 8), (SUB_DAYS_GUARD, 1)];
    let (r, _, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, day, month, year, weekday), || date_set(this, day, month, year, weekday));
    SCOREBOARD.record(equal, || format!("date_set({}, {}, {}, weekday {:#x}) original {:x?} rust {:x?}", day as i16, month as u8 as i8, year as u16, weekday as u16, mo, mr));
    r
}

/// Returns the date a number of whole weeks earlier.
///
/// `Date::sub_weeks(out, weeks)`: the local static result (`date_local_static`, built on the first call) takes the
/// date's day of the year minus 7 days per week (the count read as a byte); while the day is negative, the year goes
/// back by one and the day gains that year's length (366 in a leap year, else 365); the static gets the year's leap
/// flag and is copied to `out`, which is returned.
/// Original: `Date::sub_weeks` (0x54cab0, Date.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn date_sub_weeks(this: *const GameDate, out: *mut GameDate, weeks: u32) -> *mut GameDate {
    covered!("sx09_1 weeks back");
    let result = date_local_static(SUB_WEEKS_RESULT, SUB_WEEKS_GUARD, SUB_WEEKS_DESTRUCTOR);
    let mut year = (*this).year;
    let mut day = (*this).day.wrapping_sub((weeks as u8 as u16 * 7) as i16); // sx09_1: seven days a week
    while day < 0 {
        year = year.wrapping_sub(1);
        day = day.wrapping_add(if crate::date::is_leap_year(year) != 0 { 366 } else { 365 });
    }
    *result = GameDate { day, year, is_leap_year: crate::date::is_leap_year(year) };
    *out = *result;
    out
}

/// Comparer for `date_sub_weeks`: the result, the static and its guard side by side, and the return.
pub(crate) unsafe extern "thiscall" fn compare_date_sub_weeks(this: *const GameDate, out: *mut GameDate, weeks: u32) -> *mut GameDate {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, *mut GameDate, u32) -> *mut GameDate>(IDX_DATE_SUB_WEEKS);
    let date = *this;
    let regions = [(out as usize, 8), (SUB_WEEKS_RESULT, 8), (SUB_WEEKS_GUARD, 1)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, out, weeks), || date_sub_weeks(this, out, weeks));
    SCOREBOARD.record(equal && ro == rr, || format!("date_sub_weeks({:?}, {} weeks) original {:x?} rust {:x?}", date, weeks as u8, mo, mr));
    ro
}

/// Writes the gap between two dates as days and whole years.
///
/// `Date::get_difference(out, other)` (the other date by value): when the other date is later (a later year, or the
/// same year and a later day), the two swap places (the routine runs on the other date with this one). Otherwise,
/// in the local static result (`date_local_static`): when this day of the year is not before the other's, the days are
/// the difference of the days and the years the difference of the years; when it is before (only across a year), the
/// days gain the length of this date's year (366 in a leap year, else 365) and the years lose one. The static keeps
/// its leap flag (1 from the default date, never set here) and is copied to `out`, which is returned.
/// Original: `Date::get_difference` (0x54cc30, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_difference(this: *const GameDate, out: *mut GameDate, other: GameDate) -> *mut GameDate {
    let result = date_local_static(DIFFERENCE_RESULT, DIFFERENCE_GUARD, DIFFERENCE_DESTRUCTOR);
    let date = *this;
    if other.year > date.year || other.year == date.year && other.day > date.day {
        return date_get_difference(&other, out, date);
    }
    let (days, years) = if date.day >= other.day {
        (date.day.wrapping_sub(other.day), date.year.wrapping_sub(other.year))
    } else {
        covered!("sx09_1 difference across a year");
        let length = if crate::date::is_leap_year(date.year) != 0 { 366 } else { 365 };
        (date.day.wrapping_sub(other.day).wrapping_add(length), date.year.wrapping_sub(other.year).wrapping_sub(1)) // sx09_1: a year less
    };
    (*result).day = days;
    (*result).year = years;
    *out = *result;
    out
}

/// Comparer for `date_get_difference`: the result, the static and its guard side by side, and the return.
pub(crate) unsafe extern "thiscall" fn compare_date_get_difference(this: *const GameDate, out: *mut GameDate, other: GameDate) -> *mut GameDate {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, *mut GameDate, GameDate) -> *mut GameDate>(IDX_DATE_GET_DIFFERENCE);
    let date = *this;
    let regions = [(out as usize, 8), (DIFFERENCE_RESULT, 8), (DIFFERENCE_GUARD, 1)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, out, other), || date_get_difference(this, out, other));
    SCOREBOARD.record(equal && ro == rr, || format!("date_get_difference({:?}, {:?}) original {:x?} rust {:x?}", date, other, mo, mr));
    ro
}

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
    let r = original_fn::<unsafe extern "thiscall" fn(*const GameDate) -> u32>(IDX_DATE_GET_DAY_OF_MONTH)(this);
    let n = date_get_day_of_month(this);
    SCOREBOARD.record(r == n, || format!("date_get_day_of_month({:?}) original {:#x} rust {:#x}", *this, r, n));
    r
}

/// Comparer for `date_get_month`: the return, side by side.
pub(crate) unsafe extern "thiscall" fn compare_date_get_month(this: *const GameDate) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(*const GameDate) -> u32>(IDX_DATE_GET_MONTH)(this);
    let n = date_get_month(this);
    SCOREBOARD.record(r == n, || format!("date_get_month({:?}) original {:#x} rust {:#x}", *this, r, n));
    r
}

/// Comparer for `date_is_leap_year`: the return, side by side.
pub(crate) unsafe extern "C" fn compare_date_is_leap_year(year: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_DATE_IS_LEAP_YEAR)(year);
    let n = date_is_leap_year(year);
    SCOREBOARD.record(r == n, || format!("date_is_leap_year({}) original {} rust {}", year as u16, r, n));
    r
}

/// The days before `month` (0..11) in an ordinary or a leap year.
pub(crate) unsafe fn days_before_month(leap: bool, month: usize) -> i16 {
    read_i16(if leap { DAYS_BEFORE_MONTH_LEAP } else { DAYS_BEFORE_MONTH } + 2 * month)
}

/// The month (0..11) of a date: the last month whose days before it are fewer than the date's day of the year; none
/// when the day is -1 or less (the tables start at -1).
pub(crate) unsafe fn date_month_index(d: &GameDate) -> Option<usize> {
    (0..12).rev().find(|&k| days_before_month(d.is_leap_year != 0, k) < d.day)
}

/// The local static result date of a Date routine, built on the first call as the original does: the guard bit set,
/// the default date (`crate::date::default_date`) and the empty destructor registered at exit.
pub(crate) unsafe fn date_local_static(value: usize, guard: usize, destructor: usize) -> *mut GameDate {
    if read_u8(guard) & 1 == 0 {
        write_u8(guard, read_u8(guard) | 1);
        crate::date::default_date(value as *mut GameDate);
        crate::crt::atexit(destructor);
    }
    value as *mut GameDate
}

/// The game's date (`Date.cpp`, 8 bytes: day of the year, year, leap-year flag), the type of `crate::date`.
pub(crate) type GameDate = crate::date::Data;

/// Date.cpp's source path in the exe, for the error boxes of the date routines.
pub(crate) const DATE_CPP: usize = 0x9b8478;

/// The days before each month (12 `i16` from January, starting at -1): an ordinary year and a leap year.
pub(crate) const DAYS_BEFORE_MONTH: usize = 0x9b8448;

pub(crate) const DAYS_BEFORE_MONTH_LEAP: usize = 0x9b8460;

/// The local static result of `date_sub_weeks` (its guard byte and the empty destructor registered at exit).
pub(crate) const SUB_WEEKS_RESULT: usize = 0xae24c8;

pub(crate) const SUB_WEEKS_GUARD: usize = 0xae24aa;

pub(crate) const SUB_WEEKS_DESTRUCTOR: usize = 0x54cc20;

/// The local static result of `date_get_difference`.
pub(crate) const DIFFERENCE_RESULT: usize = 0xae2428;

pub(crate) const DIFFERENCE_GUARD: usize = 0xae249d;

pub(crate) const DIFFERENCE_DESTRUCTOR: usize = 0x54cda0;

/// The local static result of `Date::sub_days` (0x54c8e0), which `date_set` reaches through the weekday adjustment.
pub(crate) const SUB_DAYS_RESULT: usize = 0xae24c0;

pub(crate) const SUB_DAYS_GUARD: usize = 0xae249e;
