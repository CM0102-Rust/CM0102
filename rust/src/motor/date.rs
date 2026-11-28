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

/// Writes the name of a weekday to the shared buffer and returns it.
///
/// `date_get_weekday_name(weekday)` (the weekday as `i16`, 0 for Monday to 6 for Sunday): the translated name
/// (`WEEKDAY_NAMES`, langlib) in `WEEKDAY_NAME_BUFFER`; any other value leaves the empty string there.
/// Original: `date_get_weekday_name` (0x54d170, Date.cpp, cdecl).
pub(crate) unsafe extern "C" fn date_get_weekday_name(weekday: u32) -> usize {
    covered!("sx09_3 weekday name");
    write_date_name(WEEKDAY_NAME_BUFFER, date_name_template(&WEEKDAY_NAMES, weekday as i16 as i32)) // sx09_3: the weekday
}

/// Writes the three-letter name of a month to the shared buffer and returns it.
///
/// `date_get_short_month_name(month)` (the month as a signed byte, 0 for January): the translated name
/// (`SHORT_MONTH_NAMES`, langlib) in `SHORT_MONTH_NAME_BUFFER`; any other value leaves the empty string there.
/// Original: `date_get_short_month_name` (0x54d210, Date.cpp, cdecl).
pub(crate) unsafe extern "C" fn date_get_short_month_name(month: u32) -> usize {
    covered!("sx09_3 short month name");
    write_date_name(SHORT_MONTH_NAME_BUFFER, date_name_template(&SHORT_MONTH_NAMES, month as i8 as i32)) // sx09_3: the short month
}

/// Writes the full name of a month to the shared buffer and returns it.
///
/// `date_get_month_name(month)` (the month as a signed byte, 0 for January): the translated name (`MONTH_NAMES`,
/// langlib) in `MONTH_NAME_BUFFER`; any other value leaves the empty string there.
/// Original: `date_get_month_name` (0x54d3b0, Date.cpp, cdecl).
pub(crate) unsafe extern "C" fn date_get_month_name(month: u32) -> usize {
    covered!("sx09_3 month name");
    write_date_name(MONTH_NAME_BUFFER, date_name_template(&MONTH_NAMES, month as i8 as i32)) // sx09_3: the month
}

/// Comparer for `date_get_weekday_name`.
pub(crate) unsafe extern "C" fn compare_date_get_weekday_name(weekday: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_DATE_GET_WEEKDAY_NAME);
    compare_date_name("date_get_weekday_name", (WEEKDAY_NAME_BUFFER, 0x1c), weekday, || o(weekday), || date_get_weekday_name(weekday))
}

/// Comparer for `date_get_short_month_name`.
pub(crate) unsafe extern "C" fn compare_date_get_short_month_name(month: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_DATE_GET_SHORT_MONTH_NAME);
    compare_date_name("date_get_short_month_name", (SHORT_MONTH_NAME_BUFFER, 0xa), month, || o(month), || date_get_short_month_name(month))
}

/// Comparer for `date_get_month_name`.
pub(crate) unsafe extern "C" fn compare_date_get_month_name(month: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_DATE_GET_MONTH_NAME);
    compare_date_name("date_get_month_name", (MONTH_NAME_BUFFER, 0x19), month, || o(month), || date_get_month_name(month))
}

/// Writes the time of day name to the shared buffer and returns it.
///
/// `date_get_time_of_day_name(time, capital)` (the time as a byte: 0 morning, 1 afternoon, 2 evening): "Morning",
/// "Afternoon" or "Evening" (capitalised when `capital` is not 0, else in lower case; langlib) in
/// `TIME_OF_DAY_BUFFER`; any other time leaves the empty string there.
/// Original: `date_get_time_of_day_name` (0x54d040, Date.cpp, cdecl).
pub(crate) unsafe extern "C" fn date_get_time_of_day_name(time: u32, capital: u32) -> usize {
    covered!("sx09_4 time of day name");
    let names = if capital != 0 { &TIME_OF_DAY_NAMES } else { &TIME_OF_DAY_NAMES_LOWER }; // sx09_4: the case
    write_date_name(TIME_OF_DAY_BUFFER, names.get(time as u8 as usize).copied())
}

/// Returns the grammar code of the time of day word in the current language, which picks the matching article.
///
/// `date_get_time_of_day_gender(time)` (the time as a byte), by the language (`exe::langlib_get_language`): 9 and 3,
/// 1; 5, 2; 4, 1 for the morning and the afternoon (0 and 1) and 3 for the evening (2); otherwise, and for any other
/// time in language 4, 0. Only the low byte is the result.
/// Original: `date_get_time_of_day_gender` (0x54d130, Date.cpp, cdecl).
pub(crate) unsafe extern "C" fn date_get_time_of_day_gender(time: u32) -> u32 {
    covered!("sx09_4 time of day gender");
    // ponytail: the original asks for the language up to four times; the getter has no side effect, once is the same
    match exe::langlib_get_language() {
        LANGUAGE_9 | LANGUAGE_3 => 1,
        LANGUAGE_5 => 2,
        LANGUAGE_4 => match time as u8 {
            0 | 1 => 1,
            2 => 3,
            _ => 0,
        },
        _ => 0, // sx09_4: no gender
    }
}

/// Returns the grammar flag of a weekday name in a language, which picks the matching article in translated dates.
///
/// `date_get_weekday_gender(language, weekday)`: in language 9, 0 for Saturday and Sunday (5 and 6, the weekday as
/// `i16`) and 1 for the other days; in any other language, 1 only for language 3. Only the low byte is the result.
/// Original: `date_get_weekday_gender` (0x54d550, Date.cpp, cdecl).
pub(crate) unsafe extern "C" fn date_get_weekday_gender(language: u32, weekday: u32) -> u32 {
    covered!("sx09_4 weekday gender");
    if language as i32 == LANGUAGE_9 {
        return !(5..=6).contains(&(weekday as i16)) as u32; // sx09_4: the weekend days
    }
    (language as i32 == LANGUAGE_3) as u32 // sx09_4: language 3
}

/// Comparer for `date_get_time_of_day_name`: the buffer and the returned address side by side.
pub(crate) unsafe extern "C" fn compare_date_get_time_of_day_name(time: u32, capital: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> usize>(IDX_DATE_GET_TIME_OF_DAY_NAME);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&[(TIME_OF_DAY_BUFFER, 0x19)], || o(time, capital), || date_get_time_of_day_name(time, capital));
    SCOREBOARD.record(equal && ro == rr, || format!("date_get_time_of_day_name({}, {}) original {:x?} rust {:x?}", time as u8, capital, mo, mr));
    ro
}

/// Comparer for `date_get_time_of_day_gender`: the low byte, side by side.
pub(crate) unsafe extern "C" fn compare_date_get_time_of_day_gender(time: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_DATE_GET_TIME_OF_DAY_GENDER)(time);
    let n = date_get_time_of_day_gender(time);
    SCOREBOARD.record(r as u8 == n as u8, || format!("date_get_time_of_day_gender({}) original {} rust {}", time as u8, r as u8, n as u8));
    r
}

/// Comparer for `date_get_weekday_gender`: the low byte, side by side.
pub(crate) unsafe extern "C" fn compare_date_get_weekday_gender(language: u32, weekday: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(IDX_DATE_GET_WEEKDAY_GENDER)(language, weekday);
    let n = date_get_weekday_gender(language, weekday);
    SCOREBOARD.record(r as u8 == n as u8, || format!("date_get_weekday_gender({}, {}) original {} rust {}", language, weekday as i16, r as u8, n as u8));
    r
}

/// Writes the date as the short weekday, the ordinal day, the short month and the year.
///
/// `Date::get_short_weekday_date_string(out, size, time)`: a null `out` (Date.cpp, line 0x1bf) or a size below 0x12
/// (line 0x1c6) gives the error box and 0 (`date_text_buffer_ok`). With a time of day (0..2), "AM", "PM" or "EVE" first
/// (`write_time_suffix`). Then the short weekday (`write_short_weekday`), the short month name, the ordinal day of the
/// month (`date_ordinal`) and, langlib in `out`, "<Sat> <10th> <Feb> <1972>" or, with the time, "<Sat> <10th> <Feb>
/// <1972> <AM>". Returns 1.
/// Original: `Date::get_short_weekday_date_string` (0x54ae10, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_short_weekday_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 0x12, 0x1bf, 0x1c6) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(IDX_DATE_GET_SHORT_WEEKDAY_DATE_STRING)(this, out, size, time);
    };
    let year = (*this).year as u32;
    if date_text_has_time(time) {
        covered!("sx09_5 short weekday date with the time");
        write_time_suffix(time);
        write_short_weekday(this);
        let name = date_short_month_name(month);
        story_lang!(out, 0x9b87a0, SHORT_WEEKDAY_BUFFER, date_ordinal(day), name, year, TIME_SUFFIX_BUFFER);
    } else {
        covered!("sx09_5 short weekday date");
        write_short_weekday(this);
        let name = date_short_month_name(month);
        story_lang!(out, 0x9b881c, SHORT_WEEKDAY_BUFFER, date_ordinal(day), name, year); // sx09_5: day, month, year
    }
    1
}

/// Writes the date as the weekday, the ordinal day, the month name and the year.
///
/// `Date::get_long_date_string(out, size, time)`: a null `out` (line 0x270) or a size below 0x32 (line 0x277) gives the
/// error box and 0. With a time of day (0..2), "Morning", "Afternoon" or "Evening" first
/// (`exe::date_get_time_of_day_name`, capitalised). Then the weekday name (`exe::date_get_weekday_name` of
/// `crate::date::weekday`), the month name (`exe::date_get_month_name`), the ordinal day (`date_ordinal`) and, langlib
/// in `out`, "<Saturday>{} <10th> <February> <1972>" or, with the time, the same followed by "(<Morning>)" (the `{}`
/// takes `DATE_TEXT_GRAMMAR`). Returns 1.
/// Original: `Date::get_long_date_string` (0x54ba20, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_long_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 0x32, 0x270, 0x277) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(IDX_DATE_GET_LONG_DATE_STRING)(this, out, size, time);
    };
    let year = (*this).year as u32;
    if date_text_has_time(time) {
        covered!("sx09_5 long date with the time");
        let when = exe::date_get_time_of_day_name(time, 1);
        let weekday = exe::date_get_weekday_name(crate::date::weekday(this) as u32);
        let name = exe::date_get_month_name(month as u32);
        story_lang!(out, 0x9b89f8, DATE_TEXT_GRAMMAR, weekday, date_ordinal(day), name, year, when);
    } else {
        covered!("sx09_5 long date");
        let weekday = exe::date_get_weekday_name(crate::date::weekday(this) as u32);
        let name = exe::date_get_month_name(month as u32);
        story_lang!(out, 0x9b8aa0, DATE_TEXT_GRAMMAR, weekday, date_ordinal(day), name, year); // sx09_5: the long date
    }
    1
}

/// Writes the date as the weekday, the ordinal day and the month name, without the year.
///
/// `Date::get_long_date_string_no_year(out, size, time)`: as `date_get_long_date_string` (error lines 0x2af and 0x2b6),
/// with "<Saturday>{} <10th> <February>" or, with the time, the same followed by "(<Morning>)". Returns 1.
/// Original: `Date::get_long_date_string_no_year` (0x54bef0, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_long_date_string_no_year(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 0x32, 0x2af, 0x2b6) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(IDX_DATE_GET_LONG_DATE_STRING_NO_YEAR)(this, out, size, time);
    };
    if date_text_has_time(time) {
        covered!("sx09_5 long date without the year, with the time");
        let when = exe::date_get_time_of_day_name(time, 1);
        let weekday = exe::date_get_weekday_name(crate::date::weekday(this) as u32);
        let name = exe::date_get_month_name(month as u32);
        story_lang!(out, 0x9b8b0c, DATE_TEXT_GRAMMAR, weekday, date_ordinal(day), name, when);
    } else {
        covered!("sx09_5 long date without the year");
        let weekday = exe::date_get_weekday_name(crate::date::weekday(this) as u32);
        let name = exe::date_get_month_name(month as u32);
        story_lang!(out, 0x9b8b84, DATE_TEXT_GRAMMAR, weekday, date_ordinal(day), name); // sx09_5: without the year
    }
    1
}

/// Comparer for `date_get_short_weekday_date_string`.
pub(crate) unsafe extern "thiscall" fn compare_date_get_short_weekday_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(IDX_DATE_GET_SHORT_WEEKDAY_DATE_STRING);
    compare_date_text("date_get_short_weekday_date_string", this, out, size, 0x12, time, || o(this, out, size, time), || date_get_short_weekday_date_string(this, out, size, time))
}

/// Comparer for `date_get_long_date_string`.
pub(crate) unsafe extern "thiscall" fn compare_date_get_long_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(IDX_DATE_GET_LONG_DATE_STRING);
    compare_date_text("date_get_long_date_string", this, out, size, 0x32, time, || o(this, out, size, time), || date_get_long_date_string(this, out, size, time))
}

/// Comparer for `date_get_long_date_string_no_year`.
pub(crate) unsafe extern "thiscall" fn compare_date_get_long_date_string_no_year(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(IDX_DATE_GET_LONG_DATE_STRING_NO_YEAR);
    compare_date_text("date_get_long_date_string_no_year", this, out, size, 0x32, time, || o(this, out, size, time), || date_get_long_date_string_no_year(this, out, size, time))
}

/// Writes the date as the ordinal day, the month name and the year.
///
/// `Date::get_date_string_no_weekday(out, size, time)`: a null `out` (Date.cpp, line 0x2ed) or a size below 0x32
/// (line 0x2f4) gives the error box and 0 (`date_text_buffer_ok`). With a time of day (0..2), "Morning", "Afternoon" or
/// "Evening" first (`exe::date_get_time_of_day_name`, capitalised). Then the month name (`exe::date_get_month_name`),
/// the ordinal day of the month (`date_ordinal`) and, langlib in `out`, "<10th> <February> <1972>" or, with the time,
/// the same followed by "(<Morning>)". Returns 1.
/// Original: `Date::get_date_string_no_weekday` (0x54c3b0, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_date_string_no_weekday(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 0x32, 0x2ed, 0x2f4) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(IDX_DATE_GET_DATE_STRING_NO_WEEKDAY)(this, out, size, time);
    };
    let year = (*this).year as u32;
    if date_text_has_time(time) {
        covered!("sx09_6 date without the weekday, with the time");
        let when = exe::date_get_time_of_day_name(time, 1);
        let name = exe::date_get_month_name(month as u32);
        story_lang!(out, 0x9b8bd8, date_ordinal(day), name, year, when);
    } else {
        covered!("sx09_6 date without the weekday");
        let name = exe::date_get_month_name(month as u32);
        story_lang!(out, 0x9b8c4c, date_ordinal(day), name, year); // sx09_6: day, month and year
    }
    1
}

/// Comparer for `date_get_date_string_no_weekday` (`compare_date_text`).
pub(crate) unsafe extern "thiscall" fn compare_date_get_date_string_no_weekday(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(IDX_DATE_GET_DATE_STRING_NO_WEEKDAY);
    compare_date_text("date_get_date_string_no_weekday", this, out, size, 0x32, time, || o(this, out, size, time), || date_get_date_string_no_weekday(this, out, size, time))
}

/// The template of entry `k` of a name table, none when `k` is negative or past the end.
pub(crate) fn date_name_template(names: &[usize], k: i32) -> Option<usize> {
    usize::try_from(k).ok().and_then(|k| names.get(k).copied())
}

/// The comparer of a date name routine: the buffer (up to the next variable) and the returned address side by side.
pub(crate) unsafe fn compare_date_name(name: &str, buffer: (usize, usize), arg: u32, original: impl FnOnce() -> usize, rust: impl FnOnce() -> usize) -> usize {
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&[buffer], original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}({}) original {:x?} rust {:x?}", name, arg as i32, mo, mr));
    ro
}

/// The checks every date text routine makes first: a null buffer gives the error box (Date.cpp, `null_line`) and a
/// size below `min` (read as `i16`) gives the error box (`size_line`); either way the routine returns 0.
pub(crate) unsafe fn date_text_buffer_ok(out: usize, size: u32, min: i16, null_line: u32, size_line: u32) -> bool {
    if out == 0 {
        crate::crt::error_box(DATE_CPP, null_line);
        return false;
    }
    if (size as i16) < min {
        crate::crt::error_box(DATE_CPP, size_line);
        return false;
    }
    true
}

/// The month (0..11) and the day of the month a date text writes; none when the day of the year is -1 or less, where
/// the original reads the month from an uninitialised stack byte (only the original runs then).
pub(crate) unsafe fn date_text_parts(d: &GameDate) -> Option<(usize, i16)> {
    let month = date_month_index(d)?;
    Some((month, d.day.wrapping_sub(days_before_month(d.is_leap_year != 0, month))))
}

/// Whether a date text adds the time of day: the time (a byte) is 0 (morning), 1 (afternoon) or 2 (evening).
pub(crate) fn date_text_has_time(time: u32) -> bool {
    time as u8 <= 2
}

/// Writes "AM", "PM" or "EVE" for the time of day to `TIME_SUFFIX_BUFFER` (langlib, with the empty string for the
/// comment); a time outside 0..2 leaves the empty string.
pub(crate) unsafe fn write_time_suffix(time: u32) {
    write_date_name(TIME_SUFFIX_BUFFER, TIME_SUFFIXES.get(time as u8 as usize).copied());
}

/// Writes the short weekday name of a date ("Mon" .. "Sun" by `crate::date::weekday`, langlib) to
/// `SHORT_WEEKDAY_BUFFER`; a weekday outside 0..6 (a year before 1600) leaves the empty string.
pub(crate) unsafe fn write_short_weekday(d: *const GameDate) {
    let weekday = crate::date::weekday(d);
    write_date_name(SHORT_WEEKDAY_BUFFER, usize::try_from(weekday).ok().and_then(|k| SHORT_WEEKDAY_NAMES.get(k).copied()));
}

/// The short month name (`FN_DATE_GET_SHORT_MONTH_NAME`).
pub(crate) unsafe fn date_short_month_name(month: usize) -> usize {
    let f: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_DATE_GET_SHORT_MONTH_NAME);
    f(month as u32)
}

/// The day of the month as an ordinal ("10th", `exe::utils_format_ordinal` with 1).
pub(crate) unsafe fn date_ordinal(day: i16) -> usize {
    exe::utils_format_ordinal(day as i32, 1)
}

/// The comparer of a date text routine: `out` (up to its size, at most 0x400 bytes), the shared name buffers and the
/// return side by side. A null buffer, a short size or a day of -1 or less: only the original (the error box, or the
/// uninitialised month).
pub(crate) unsafe fn compare_date_text(name: &str, this: *const GameDate, out: usize, size: u32, min: i16, time: u32, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    if out == 0 || (size as i16) < min || date_text_parts(&*this).is_none() {
        return original();
    }
    let mut regions = DATE_TEXT_BUFFERS.to_vec();
    regions.push((out, (size as i16 as usize).min(0x400)));
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || {
        let text = |m: &crate::comparison::MemorySnapshot| m.0.last().map(|(_, b)| String::from_utf8_lossy(&b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]).into_owned());
        format!("{}({:?}, team {}) original {} {:?} rust {} {:?}; memory {:x?} x {:x?}", name, *this, time as u8, ro, text(&mo), rr, text(&mr), mo, mr)
    });
    ro
}

/// The shared buffers of the date names.
pub(crate) const WEEKDAY_NAME_BUFFER: usize = 0xae2434;

pub(crate) const SHORT_MONTH_NAME_BUFFER: usize = 0xae24a0;

pub(crate) const MONTH_NAME_BUFFER: usize = 0xae2450;

/// The weekday names as langlib templates, 0 for Monday to 6 for Sunday ("Monday" .. "Sunday").
pub(crate) const WEEKDAY_NAMES: [usize; 7] = [0x9b8790, 0x9b8788, 0x9b877c, 0x9b8770, 0x9b8768, 0x9b875c, 0x9b8798];

/// The three-letter month names, January to December ("Jan" .. "Dec"; March and December carry a translator's
/// comment).
pub(crate) const SHORT_MONTH_NAMES: [usize; 12] = [0x9b8d2c, 0x9b8d28, 0x9b8d00, 0x9b8cfc, 0x9b8cf8, 0x9b8cf4, 0x9b8cf0, 0x9b8cec, 0x9b8ce8, 0x9b8ce4, 0x9b8ce0, 0x9b8cb8];

/// The full month names, January to December (May carries a translator's comment).
pub(crate) const MONTH_NAMES: [usize; 12] = [0x9b8db8, 0x9b8dac, 0x9b8da4, 0x9b8d9c, 0x9b8d74, 0x9b8d6c, 0x9b8d64, 0x9b8d5c, 0x9b8d50, 0x9b8d48, 0x9b8d3c, 0x9b8d30];

/// The shared buffer of the time of day name.
pub(crate) const TIME_OF_DAY_BUFFER: usize = 0xae2484;

/// The times of day (morning, afternoon, evening) capitalised and in lower case.
pub(crate) const TIME_OF_DAY_NAMES: [usize; 3] = [0x9b8a84, 0x9b8a8c, 0x9b8a98];

pub(crate) const TIME_OF_DAY_NAMES_LOWER: [usize; 3] = [0x9b8c9c, 0x9b8ca4, 0x9b8cb0];

/// The languages whose date words have grammatical gender (`exe::langlib_get_language`).
pub(crate) const LANGUAGE_3: i32 = 3;

pub(crate) const LANGUAGE_4: i32 = 4;

pub(crate) const LANGUAGE_5: i32 = 5;

pub(crate) const LANGUAGE_9: i32 = 9;

/// `date_get_short_month_name(month)` (0x54d210, Date.cpp, cdecl): "Jan".."Dec" in its shared buffer.
pub(crate) const FN_DATE_GET_SHORT_MONTH_NAME: usize = 0x54d210;

/// The shared buffers of the date texts: the short weekday name and the AM, PM or EVE suffix.
pub(crate) const SHORT_WEEKDAY_BUFFER: usize = 0xae2478;

pub(crate) const TIME_SUFFIX_BUFFER: usize = 0xae24ac;

/// The short weekday names, 0 for Monday to 6 for Sunday ("Mon" .. "Sun").
pub(crate) const SHORT_WEEKDAY_NAMES: [usize; 7] = [0x9b8890, 0x9b888c, 0x9b8888, 0x9b8884, 0x9b8880, 0x9b887c, 0x9b8894];

/// The time suffixes of the numeric dates: "AM", "PM" and "EVE" (each with a translator's comment).
pub(crate) const TIME_SUFFIXES: [usize; 3] = [0x9b8508, 0x9b8524, 0x9b8544];

/// The argument of the `{}` grammar code that the long date templates carry after the weekday.
pub(crate) const DATE_TEXT_GRAMMAR: u32 = 0x64;

/// The shared buffers a date text may write besides its own (the weekday, month, short weekday, short month, time
/// of day and suffix names, and the ordinal's at 0xde2734).
pub(crate) const DATE_TEXT_BUFFERS: [(usize, usize); 7] = [(0xae2434, 0x1c), (0xae2450, 0x19), (0xae2478, 0xc), (0xae2484, 0x19), (0xae24a0, 0xa), (0xae24ac, 0x14), (0xde2734, 0x20)];

/// Day `day` of the current month (`Date::set` with today's month and year); when today is past it, day `day` of the
/// next month (January of the next year after December).
pub(crate) unsafe fn day_of_this_or_next_month(day: u32) -> [u8; 8] {
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let mut d = [0u8; 8];
    exe::date_set(d.as_mut_ptr() as usize, day, exe::date_get_month(G_CURRENT_DATE) as u32, year, -1);
    if cm_date_after(G_CURRENT_DATE, d.as_ptr() as usize) {
        let month = exe::date_get_month(G_CURRENT_DATE).wrapping_add(1);
        if (month as i8) < 0xc {
            exe::date_set(d.as_mut_ptr() as usize, day, month as u32, year, -1);
        } else {
            exe::date_set(d.as_mut_ptr() as usize, day, 0, (year as u16).wrapping_add(1) as u32, -1);
        }
    }
    d
}
