// ---------- batch %%LEVA%%: Date.cpp: setting a date, going back whole weeks and the gap between two dates (sx09_1) ----------

/// The game's date (`Date.cpp`, 8 bytes: day of the year, year, leap-year flag), the type of `crate::date`.
pub(crate) type GameDate = crate::date::Data;

/// Date.cpp's source path in the exe, for the error boxes of the date routines.
pub(crate) const DATE_CPP: usize = 0x9b8478;
/// The days before each month (12 `i16` from January, starting at -1): an ordinary year and a leap year.
pub(crate) const DAYS_BEFORE_MONTH: usize = 0x9b8448;
pub(crate) const DAYS_BEFORE_MONTH_LEAP: usize = 0x9b8460;

/// The local static result of `date_sub_weeks` (its guard byte and the empty destructor registered at exit).
const SUB_WEEKS_RESULT: usize = 0xae24c8;
const SUB_WEEKS_GUARD: usize = 0xae24aa;
const SUB_WEEKS_DESTRUCTOR: usize = 0x54cc20;
/// The local static result of `date_get_difference`.
const DIFFERENCE_RESULT: usize = 0xae2428;
const DIFFERENCE_GUARD: usize = 0xae249d;
const DIFFERENCE_DESTRUCTOR: usize = 0x54cda0;
/// The local static result of `Date::sub_days` (0x54c8e0), which `date_set` reaches through the weekday adjustment.
const SUB_DAYS_RESULT: usize = 0xae24c0;
const SUB_DAYS_GUARD: usize = 0xae249e;

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

/// Writes a date name into one of the shared name buffers and returns the buffer: the langlib template (its extra
/// argument is the empty string, which only the templates with a translator's comment read), or the empty string when
/// there is none.
pub(crate) unsafe fn write_date_name(buffer: usize, template: Option<usize>) -> usize {
    match template {
        Some(t) => story_lang!(buffer, t, EMPTY_STRING),
        None => copy_c_string(buffer, EMPTY_STRING),
    }
    buffer
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(*mut GameDate, u32, u32, u32, u32) -> u32>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, *mut GameDate, u32) -> *mut GameDate>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, *mut GameDate, GameDate) -> *mut GameDate>(%%I2%%);
    let date = *this;
    let regions = [(out as usize, 8), (DIFFERENCE_RESULT, 8), (DIFFERENCE_GUARD, 1)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, out, other), || date_get_difference(this, out, other));
    SCOREBOARD.record(equal && ro == rr, || format!("date_get_difference({:?}, {:?}) original {:x?} rust {:x?}", date, other, mo, mr));
    ro
}
