// ---------- batch 333: Date.cpp: the weekday and month names (sx09_3) ----------

/// The shared buffers of the date names.
pub(crate) const WEEKDAY_NAME_BUFFER: usize = 0xae2434;
pub(crate) const SHORT_MONTH_NAME_BUFFER: usize = 0xae24a0;
pub(crate) const MONTH_NAME_BUFFER: usize = 0xae2450;

/// The weekday names as langlib templates, 0 for Monday to 6 for Sunday ("Monday" .. "Sunday").
pub(crate) const WEEKDAY_NAMES: [usize; 7] = [0x9b8790, 0x9b8788, 0x9b877c, 0x9b8770, 0x9b8768, 0x9b875c, 0x9b8798];
/// The three-letter month names, January to December ("Jan" .. "Dec"; March and December carry a translator's
/// comment).
const SHORT_MONTH_NAMES: [usize; 12] = [0x9b8d2c, 0x9b8d28, 0x9b8d00, 0x9b8cfc, 0x9b8cf8, 0x9b8cf4, 0x9b8cf0, 0x9b8cec, 0x9b8ce8, 0x9b8ce4, 0x9b8ce0, 0x9b8cb8];
/// The full month names, January to December (May carries a translator's comment).
const MONTH_NAMES: [usize; 12] = [0x9b8db8, 0x9b8dac, 0x9b8da4, 0x9b8d9c, 0x9b8d74, 0x9b8d6c, 0x9b8d64, 0x9b8d5c, 0x9b8d50, 0x9b8d48, 0x9b8d3c, 0x9b8d30];

/// The template of entry `k` of a name table, none when `k` is negative or past the end.
fn date_name_template(names: &[usize], k: i32) -> Option<usize> {
    usize::try_from(k).ok().and_then(|k| names.get(k).copied())
}

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

/// The comparer of a date name routine: the buffer (up to the next variable) and the returned address side by side.
unsafe fn compare_date_name(name: &str, buffer: (usize, usize), arg: u32, original: impl FnOnce() -> usize, rust: impl FnOnce() -> usize) -> usize {
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&[buffer], original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}({}) original {:x?} rust {:x?}", name, arg as i32, mo, mr));
    ro
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

// ---------- batch 333: Date.cpp: the time of day name and the grammar of the date words (sx09_4) ----------

/// The shared buffer of the time of day name.
pub(crate) const TIME_OF_DAY_BUFFER: usize = 0xae2484;
/// The times of day (morning, afternoon, evening) capitalised and in lower case.
const TIME_OF_DAY_NAMES: [usize; 3] = [0x9b8a84, 0x9b8a8c, 0x9b8a98];
const TIME_OF_DAY_NAMES_LOWER: [usize; 3] = [0x9b8c9c, 0x9b8ca4, 0x9b8cb0];
/// The languages whose date words have grammatical gender (`exe::langlib_get_language`).
const LANGUAGE_3: i32 = 3;
const LANGUAGE_4: i32 = 4;
const LANGUAGE_5: i32 = 5;
const LANGUAGE_9: i32 = 9;

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

// ---------- batch 333: Date.cpp: the date texts with the weekday (short and long) and the month name (sx09_5) ----------

/// `date_get_short_month_name(month)` (0x54d210, Date.cpp, cdecl): "Jan".."Dec" in its shared buffer.
const FN_DATE_GET_SHORT_MONTH_NAME: usize = 0x54d210;

/// The shared buffers of the date texts: the short weekday name and the AM, PM or EVE suffix.
pub(crate) const SHORT_WEEKDAY_BUFFER: usize = 0xae2478;
pub(crate) const TIME_SUFFIX_BUFFER: usize = 0xae24ac;
/// The short weekday names, 0 for Monday to 6 for Sunday ("Mon" .. "Sun").
const SHORT_WEEKDAY_NAMES: [usize; 7] = [0x9b8890, 0x9b888c, 0x9b8888, 0x9b8884, 0x9b8880, 0x9b887c, 0x9b8894];
/// The time suffixes of the numeric dates: "AM", "PM" and "EVE" (each with a translator's comment).
const TIME_SUFFIXES: [usize; 3] = [0x9b8508, 0x9b8524, 0x9b8544];
/// The argument of the `{}` grammar code that the long date templates carry after the weekday.
pub(crate) const DATE_TEXT_GRAMMAR: u32 = 0x64;

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

/// The shared buffers a date text may write besides its own (the weekday, month, short weekday, short month, time
/// of day and suffix names, and the ordinal's at 0xde2734).
const DATE_TEXT_BUFFERS: [(usize, usize); 7] = [(0xae2434, 0x1c), (0xae2450, 0x19), (0xae2478, 0xc), (0xae2484, 0x19), (0xae24a0, 0xa), (0xae24ac, 0x14), (0xde2734, 0x20)];

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
        format!("{}({:?}, time {}) original {} {:?} rust {} {:?}; memory {:x?} x {:x?}", name, *this, time as u8, ro, text(&mo), rr, text(&mr), mo, mr)
    });
    ro
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

// ---------- batch 333: Date.cpp: the date text without the weekday (sx09_6) ----------

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

/// The C string at `p` (up to its NUL).
unsafe fn lng_c_str<'a>(p: usize) -> &'a [u8] {
    core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes()
}

/// `strncpy(dst, src, n)` of the game's CRT (pads with NULs up to `n`).
unsafe fn lng_strncpy(dst: usize, src: usize, n: u32) {
    exe::at_9466d0(dst, src as *const u8, n);
}

/// A gender byte of the names: a value below 1 (signed, the "none" 0xff included) becomes 0.
unsafe fn lng_clamp_gender(p: usize) {
    if read_i8(p) < 1 {
        write_u8(p, 0);
    }
}

/// The name, gender and short name of a record laid out like `TComp` (also `TClub`, `TNation`, `TStaffComp`: NAME +4,
/// GENDER_NAME +0x37, SHORT_NAME +0x38, its terminator +0x51, SHORT_GENDER_NAME +0x52) from a .lng row (name +0, gender
/// +0x33, short name +0x34, short gender +0x4e): an empty name keeps the old one; the gender is always taken; an empty
/// short name becomes the first 26 bytes of the name (terminated at +0x51) with the name's gender.
unsafe fn lng_apply_named_row(rec: usize, row: usize) {
    if read_u8(row) != 0 {
        lng_strncpy(rec + competition::NAME, row, 0x33);
    }
    write_u8(rec + competition::GENDER_NAME, read_u8(row + 0x33));
    if read_u8(row + 0x34) == 0 {
        lng_strncpy(rec + competition::SHORT_NAME, rec + competition::NAME, 0x1a);
        write_u8(rec + 0x51, 0); // the short name's last byte
        write_u8(rec + competition::SHORT_GENDER_NAME, read_u8(rec + competition::GENDER_NAME));
    } else {
        lng_strncpy(rec + competition::SHORT_NAME, row + 0x34, 0x1a);
        write_u8(rec + competition::SHORT_GENDER_NAME, read_u8(row + 0x4e));
    }
}

/// A competition from a .lng row of 83 bytes: the names (`lng_apply_named_row`), the three-letter name (+0x53 from row
/// +0x4f, when not empty) and both genders clamped.
unsafe fn lng_apply_comp_row(comp: usize, row: usize) {
    lng_apply_named_row(comp, row);
    if read_u8(row + 0x4f) != 0 {
        lng_strncpy(comp + 0x53, row + 0x4f, 4); // three-letter name
    }
    lng_clamp_gender(comp + competition::GENDER_NAME);
    lng_clamp_gender(comp + competition::SHORT_GENDER_NAME);
}

/// A club from a .lng row of 79 bytes (`i` = its row in the club table). When the row has a name but no short name, a
/// club whose short name equals its name gets the new name as short name too, if it is shorter than 26 bytes. Then the
/// name (when not empty) and the gender; a short name in the row is taken with its gender; without one, the national
/// teams (the last 2 × nations rows) get the first 26 bytes of the name with the name's gender, and the other clubs keep
/// theirs and get the name's gender only when the short name's first letter equals the name's and the row's short
/// gender is 0xff ("none"), the row's short gender otherwise. Spanish names (language 5) turn a "none" gender into 1 and
/// a "none" short gender into the gender; the other languages clamp both.
unsafe fn lng_apply_club_row(language: u32, i: i32, club: Club, row: usize) {
    let c = club.addr();
    if read_u8(row + 0x34) == 0 && read_u8(row) != 0 && lng_c_str(c + club::NAME) == lng_c_str(c + club::SHORT_NAME) && lng_c_str(row).len() < 0x1a {
        covered!("sx08b_1 club short name follows the name");
        lng_strncpy(c + club::SHORT_NAME, row, 0x1a);
    }
    if read_u8(row) != 0 {
        lng_strncpy(c + club::NAME, row, 0x33);
    }
    write_u8(c + club::GENDER_NAME, read_u8(row + 0x33));
    if read_u8(row + 0x34) != 0 {
        lng_strncpy(c + club::SHORT_NAME, row + 0x34, 0x1a);
        write_u8(c + club::SHORT_GENDER_NAME, read_u8(row + 0x4e));
    } else if i >= read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
        covered!("sx08b_1 national team short name from the name");
        lng_strncpy(c + club::SHORT_NAME, c + club::NAME, 0x1a);
        write_u8(c + 0x51, 0); // the short name's last byte
        write_u8(c + club::SHORT_GENDER_NAME, read_u8(c + club::GENDER_NAME));
    } else if read_u8(c + club::SHORT_NAME) == read_u8(c + club::NAME) && read_u8(row + 0x4e) == 0xff {
        write_u8(c + club::SHORT_GENDER_NAME, read_u8(c + club::GENDER_NAME));
    } else {
        write_u8(c + club::SHORT_GENDER_NAME, read_u8(row + 0x4e));
    }
    if language == 5 {
        covered!("sx08b_1 spanish club genders");
        if read_u8(c + club::GENDER_NAME) == 0xff {
            write_u8(c + club::GENDER_NAME, 1);
        }
        if read_u8(c + club::SHORT_GENDER_NAME) == 0xff {
            write_u8(c + club::SHORT_GENDER_NAME, read_u8(c + club::GENDER_NAME));
        }
    } else {
        lng_clamp_gender(c + club::GENDER_NAME);
        lng_clamp_gender(c + club::SHORT_GENDER_NAME);
    }
}

/// Reads `n` rows of `size` bytes from the .lng file and hands each (index, row) to `apply`: all at once into a
/// `malloc` block, freed after; when `malloc` fails, one row at a time into a stack buffer (the original repeats each
/// loop for that case).
unsafe fn lng_for_each_row(file: usize, n: i32, size: u32, mut apply: impl FnMut(i32, usize)) {
    let rows = exe::crt_malloc((n as u32).wrapping_mul(size));
    if rows == 0 {
        covered!("sx08b_1 rows one by one");
        let mut row = [0u8; 0xba];
        for k in 0..n {
            exe::crt_fread(row.as_mut_ptr() as usize, size, 1, file);
            apply(k, row.as_ptr() as usize);
        }
        return;
    }
    exe::crt_fread(rows, size, n as u32, file);
    for k in 0..n {
        apply(k, rows + k as usize * size as usize);
    }
    exe::crt_free(rows);
}

/// The club names (79-byte rows). With the English-name index ([`G_CLUB_ENGLISH_NAME_INDEX_TABLE`], built by the
/// database cuts: club row → row of the file), club i takes file row index[i]: the whole block read, every club of the
/// table; without the block, one `fseek` per club (to 0x20 + index[i] × 79) for the file's count of clubs, then to the
/// end of the club rows. Without the index, club k takes file row k.
unsafe fn lng_load_club_names(file: usize, n: i32, language: u32) {
    let index = read_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE);
    if index == 0 {
        lng_for_each_row(file, n, 0x4f, |k, row| lng_apply_club_row(language, k, club_at(k as isize), row));
        return;
    }
    covered!("sx08b_1 club names through the index");
    let file_row = |i: i32| read_i32(index + 4 * i as usize);
    let rows = exe::crt_malloc((n as u32).wrapping_mul(0x4f));
    if rows != 0 {
        exe::crt_fread(rows, 0x4f, n as u32, file);
        let mut i = 0;
        while i < read_i32(G_CLUB_COUNT) {
            lng_apply_club_row(language, i, club_at(i as isize), rows.wrapping_add((file_row(i) as usize).wrapping_mul(0x4f)));
            i += 1;
        }
        exe::crt_free(rows);
        return;
    }
    let mut row = [0u8; 0x4f];
    for i in 0..n {
        exe::crt_fseek(file, file_row(i).wrapping_mul(0x4f).wrapping_add(0x20), 0);
        exe::crt_fread(row.as_mut_ptr() as usize, 0x4f, 1, file);
        lng_apply_club_row(language, i, club_at(i as isize), row.as_ptr() as usize);
    }
    exe::crt_fseek(file, n.wrapping_mul(0x4f).wrapping_add(0x20), 0);
}

/// A continent from a 186-byte row: name (+4, 26 bytes) when not empty, gender (+0x1e), the second name (+0x23, 26),
/// the three-letter name (+0x1f, 4), the long name (+0x3d, 101), its gender (+0xa2), another name (+0xa3, 26) and its
/// gender (+0xbd), each string only when not empty; the first gender clamped.
unsafe fn lng_apply_continent_row(c: usize, row: usize) {
    if read_u8(row) != 0 {
        lng_strncpy(c + 4, row, 0x1a); // name
    }
    write_u8(c + 0x1e, read_u8(row + 0x1a)); // name gender
    if read_u8(row + 0x1f) != 0 {
        lng_strncpy(c + 0x23, row + 0x1f, 0x1a); // continental name
    }
    if read_u8(row + 0x1b) != 0 {
        lng_strncpy(c + 0x1f, row + 0x1b, 4); // three-letter name
    }
    if read_u8(row + 0x39) != 0 {
        lng_strncpy(c + 0x3d, row + 0x39, 0x65); // federation name
    }
    write_u8(c + 0xa2, read_u8(row + 0x9e)); // federation name gender
    if read_u8(row + 0x9f) != 0 {
        lng_strncpy(c + 0xa3, row + 0x9f, 0x1a); // federation short name
    }
    write_u8(c + 0xbd, read_u8(row + 0xb9)); // federation short name gender
    lng_clamp_gender(c + 0x1e);
}

/// A nation from a 109-byte row: the names (`lng_apply_named_row`), the three-letter name (+0x53 from row +0x4f) and
/// the nationality (+0x57 from row +0x53), each when not empty; both genders clamped.
unsafe fn lng_apply_nation_row(nat: Nation, row: usize) {
    let n = nat.addr();
    lng_apply_named_row(n, row);
    if read_u8(row + 0x4f) != 0 {
        lng_strncpy(n + nation::THREE_LETTER_NAME, row + 0x4f, 4);
    }
    if read_u8(row + 0x53) != 0 {
        lng_strncpy(n + nation::NATIONALITY, row + 0x53, 0x1a);
    }
    lng_clamp_gender(n + nation::GENDER_NAME);
    lng_clamp_gender(n + nation::SHORT_GENDER_NAME);
}

/// Loads the translated names of the database from the language's .lng in CM3_TEMP.
///
/// Nothing without the nation table or when the names are already in that language ([`G_DATABASE_NAMES_LANGUAGE`]).
/// English (0, 1, 11, 12): eng.lng, and the English-name tables object ([`G_ENGLISH_NAME_TABLES`], kept for the other
/// languages) is freed; the bar has 16 steps when `two_part_bar` (the first half of a switch), 8 otherwise. Any other
/// language first goes back to English (a call to itself with (0, 1)) and then reads its own file in the second half
/// of the bar (steps 9..16 of 16, without starting it again). An unknown language: the Database.cpp error box (line
/// 0x1e37), then the French names. The bar text is "Re-loading Database"; it starts at step 0 except in the second half.
///
/// The file missing: error box (line 0x1e48) and nothing else. Then the format marker and the 8-dword header (byte-
/// swapped in a big-endian file); counts that do not match the loaded tables (the check of `eng_lng_load_tables`): the
/// file is closed, error box (line 0x1e68), the language stays. Otherwise, in file order and a bar step after each:
/// clubs (`lng_load_club_names`), continents, the nation competitions (written to the club competition table before
/// its 127 reserve rows), nations, stadiums (name 51 bytes and clamped gender), cities (name 26 bytes, gender at +0x1e),
/// staff competitions (names only), club competitions; each table read whole into a `malloc` block or, failing that,
/// row by row. The file is closed, the last step shown, the language set, and the pundit list ([`G_PUNDIT_LIST`]), when
/// there is one, rebuilt in the new language (destroyed, deleted and a new one of 10 bytes built; 0 when `new` fails).
///
/// Defects of the original: an unknown language loads the French names after the error box; when the club row block
/// cannot be allocated with the English-name index, the loop runs over the file's count of clubs, not the table's.
/// Original: `database_load_language_names` (0x5339a0, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_load_language_names(language: u32, two_part_bar: u32) {
    if read_usize(G_NATION_TABLE) == 0 || read_u32(G_DATABASE_NAMES_LANGUAGE) == language {
        return;
    }
    covered!("sx08b_1 names reloaded");
    let mut path = [0u8; 0x104];
    let bar: u8;
    if matches!(language, 0 | 1 | 11 | 12) {
        copy_c_string(path.as_mut_ptr() as usize, exe::utils_build_file_path(0x987da0usize, LANGUAGE_FILES[0])); // "CM3_TEMP"
        bar = (two_part_bar != 0) as u8;
        let t = read_usize(G_ENGLISH_NAME_TABLES);
        if t != 0 {
            covered!("sx08b_1 english name tables freed");
            eng_lng_free_tables(t);
            exe::crt_operator_delete(t);
            write_usize(G_ENGLISH_NAME_TABLES, 0);
        }
    } else {
        let file = match LANGUAGE_FILES.get(language as usize) {
            Some(&f) => f,
            None => {
                covered!("sx08b_1 unknown language");
                database_error_box(0x1e37);
                LANGUAGE_FILES[2] // fra.lng
            }
        };
        database_load_language_names(0, 1); // back to the English names first
        copy_c_string(path.as_mut_ptr() as usize, exe::utils_build_file_path(0x987da0usize, file)); // "CM3_TEMP"
        bar = 2;
    }
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut text = [0u8; 200];
    langlib(text.as_mut_ptr() as usize, 0x9b62e8usize); // "Re-loading Database"
    let total: u32 = if bar != 0 { 16 } else { 8 };
    let first: i32 = if bar >= 2 { 8 } else { 0 };
    let step = |k: i32| exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, first + k, total, -1);
    if bar < 2 {
        step(0);
    }
    let file = exe::crt_fopen(path.as_ptr() as usize, 0x98f928usize); // "rb"
    if file == 0 {
        database_error_box(0x1e48);
        return;
    }
    let big_endian = exe::utils_read_file_format_marker(file) == 0xf1fffff1;
    let mut header = [0u32; 8];
    exe::crt_fread(header.as_mut_ptr() as usize, 0x20, 1, file);
    if big_endian {
        for v in header.iter_mut() {
            *v = v.swap_bytes();
        }
    }
    if !eng_lng_counts_match(header.as_ptr() as usize) {
        exe::crt_fclose(file);
        database_error_box(0x1e68);
        return;
    }
    let count = |k: usize| header[k] as i32;
    lng_load_club_names(file, count(0), language);
    step(1);
    lng_for_each_row(file, count(1), 0xba, |k, row| lng_apply_continent_row(read_usize(G_CONTINENT_TABLE) + k as usize * 0xc6, row));
    step(2);
    // the nation competitions sit right before the 127 reserve rows of the club competition table
    let nation_comps = count(2);
    lng_for_each_row(file, nation_comps, 0x53, |k, row| {
        let id = k - nation_comps - 0x7f + read_i32(G_CLUB_COMP_COUNT);
        lng_apply_comp_row(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b)), row);
    });
    step(3);
    lng_for_each_row(file, count(3), 0x6d, |k, row| lng_apply_nation_row(Nation(read_usize(G_NATION_TABLE) + k as usize * nation::SIZE), row));
    step(4);
    lng_for_each_row(file, count(6), 0x34, |k, row| {
        let s = read_usize(G_STADIUM_TABLE) + k as usize * stadium::SIZE;
        if read_u8(row) != 0 {
            lng_strncpy(s + stadium::NAME, row, 0x33);
        }
        write_u8(s + stadium::GENDER_NAME, read_u8(row + 0x33));
        lng_clamp_gender(s + stadium::GENDER_NAME);
    });
    step(5);
    lng_for_each_row(file, count(4), 0x34, |k, row| {
        let c = read_usize(G_CITY_TABLE) + k as usize * 0x38; // city record, 56 bytes
        if read_u8(row) != 0 {
            lng_strncpy(c + 4, row, 0x1a); // name
        }
        write_u8(c + city::GENDER_NAME, read_u8(row + 0x33));
        lng_clamp_gender(c + city::GENDER_NAME);
    });
    step(6);
    lng_for_each_row(file, count(7), 0x4f, |k, row| {
        let s = read_usize(G_STAFF_COMP_TABLE) + k as usize * 0x65; // staff competition record, 101 bytes
        lng_apply_named_row(s, row);
        lng_clamp_gender(s + competition::GENDER_NAME);
        lng_clamp_gender(s + competition::SHORT_GENDER_NAME);
    });
    step(7);
    lng_for_each_row(file, count(5), 0x53, |k, row| lng_apply_comp_row(read_usize(G_CLUB_COMP_TABLE) + k as usize * 0x6b, row));
    exe::crt_fclose(file);
    step(8);
    let pundits = read_usize(G_PUNDIT_LIST);
    write_u32(G_DATABASE_NAMES_LANGUAGE, language);
    if pundits == 0 {
        return;
    }
    covered!("sx08b_1 pundit list rebuilt");
    pundit_list_destruct(pundits);
    exe::crt_operator_delete(pundits);
    let p = exe::crt_operator_new(10);
    write_usize(G_PUNDIT_LIST, if p != 0 { pundit_list_construct(p, language) } else { 0 });
}

/// The tables `database_load_language_names` rewrites: (table pointer, row count, record size).
const LNG_NAME_TABLES: [(usize, usize, usize); 7] = [
    (G_CLUB_TABLE, G_CLUB_COUNT, club::SIZE),
    (G_CONTINENT_TABLE, G_CONTINENT_COUNT, 0xc6),
    (G_NATION_TABLE, G_NATION_COUNT, nation::SIZE),
    (G_STADIUM_TABLE, G_STADIUM_COUNT, stadium::SIZE),
    (G_CITY_TABLE, G_CITY_COUNT, 0x38),
    (G_STAFF_COMP_TABLE, 0xae2388, 0x65), // staff competition count
    (G_CLUB_COMP_TABLE, G_CLUB_COMP_COUNT, 0x6b),
];

/// A copy of the bytes of the seven name tables (empty for a table not allocated).
unsafe fn lng_snapshot() -> Vec<Vec<u8>> {
    LNG_NAME_TABLES.iter().map(|&(table, count, size)| {
        let (t, n) = (read_usize(table), read_i32(count).max(0) as usize);
        if t == 0 { Vec::new() } else { core::slice::from_raw_parts(t as *const u8, n * size).to_vec() }
    }).collect()
}

/// Puts the bytes of `lng_snapshot` back.
unsafe fn lng_restore(s: &[Vec<u8>]) {
    for (&(table, _, _), bytes) in LNG_NAME_TABLES.iter().zip(s) {
        let t = read_usize(table);
        if t != 0 {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), t as *mut u8, bytes.len());
        }
    }
}

/// Comparer for `database_load_language_names`, side by side on the seven name tables: the original runs (its call to
/// itself for English passes through this comparer again and leaves the original's result), then the tables and the
/// names language go back to before and the copy runs from the same state (the English tables object the original
/// freed stays 0; the pundit list the original built is rebuilt by the copy). The tables, the language, the English
/// tables object and the pundit list must match; the game goes on with the original's tables.
pub(crate) unsafe extern "C" fn compare_database_load_language_names(language: u32, two_part_bar: u32) {
    let before = lng_snapshot();
    let language_before = read_u32(G_DATABASE_NAMES_LANGUAGE);
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_DATABASE_LOAD_LANGUAGE_NAMES)(language, two_part_bar);
    let original = lng_snapshot();
    let (language_original, english_original, pundits_original) =
        (read_u32(G_DATABASE_NAMES_LANGUAGE), read_usize(G_ENGLISH_NAME_TABLES), read_usize(G_PUNDIT_LIST) != 0);
    lng_restore(&before);
    write_u32(G_DATABASE_NAMES_LANGUAGE, language_before);
    database_load_language_names(language, two_part_bar);
    let copy = lng_snapshot();
    let tables_equal = original == copy;
    let language_copy = read_u32(G_DATABASE_NAMES_LANGUAGE);
    SCOREBOARD.record(tables_equal && language_original == language_copy
            && (english_original == 0) == (read_usize(G_ENGLISH_NAME_TABLES) == 0)
            && pundits_original == (read_usize(G_PUNDIT_LIST) != 0),
        || format!("database_load_language_names({}, {}): tables equal {} (first differing table {:?}) language {} / {}",
            language, two_part_bar, tables_equal, original.iter().zip(&copy).position(|(a, b)| a != b), language_original, language_copy));
    lng_restore(&original);
    write_u32(G_DATABASE_NAMES_LANGUAGE, language_original);
}

// ---------- sx10_6: the default formation (formation.cpp) ----------

/// The name of a formation nothing has named yet: "Uninitialised".
const FORMATION_UNINITIALISED_NAME: usize = 0x9cc3a8;
/// The default 4-4-2 of slots 0..10 (general position codes): GK; D R, D L, D CL, D CR; M CL, M R, M CR; F CL, F CR;
/// M L.
const FORMATION_DEFAULT_POSITIONS: [u32; 11] = [0x1, 0x804, 0x84, 0x104, 0x404, 0x110, 0x810, 0x410, 0x140, 0x440, 0x90];
/// The team instructions of a new formation.
const FORMATION_DEFAULT_TEAM_INSTRUCTIONS: u32 = 0x22a91;
/// The instructions of each player of a new formation (the low dword 0x15552221, the high dword 0xa).
const FORMATION_DEFAULT_PLAYER_INSTRUCTIONS: u64 = 0x0000_000a_1555_2221;
/// The marking type of each slot of a new formation.
const FORMATION_DEFAULT_MARKING_TYPE: u8 = 0x12;
/// The dword after `formation_fields::HAS_PASSWORD`: 1 once the formation was read from or written to a file
/// [inference: "saved"].
const FORMATION_SAVED: usize = 0x1887;

/// The common tail of both formation constructors: the default player instructions, marking types 0x12, no designated
/// people (-1), the position lookup rebuilt, then changed = 1, no password and not saved.
pub(crate) unsafe fn formation_reset_instructions(this: usize) {
    for k in 0..11usize {
        write_u64(this + formation_fields::PLAYER_INSTRUCTIONS + 8 * k, FORMATION_DEFAULT_PLAYER_INSTRUCTIONS); // sx10_6: default player instructions
        write_u8(this + formation_fields::MARKING_TYPES + k, FORMATION_DEFAULT_MARKING_TYPE);
        write_i32(this + formation_fields::DESIGNATED_PEOPLE + 4 * k, -1); // sx10_6: nobody designated
    }
    exe::formation_build_position_lookup(this);
    write_u32(this + formation_fields::CHANGED_FLAG, 1);
    write_u32(this + formation_fields::HAS_PASSWORD, 0);
    write_u32(this + FORMATION_SAVED, 0);
}

/// Builds an empty formation: a 4-4-2 with the default instructions.
///
/// `Formation::Formation()`: the whole record (`formation_fields::SIZE` bytes) zeroed; the name "Uninitialised"; the
/// password (`formation_fields::PASSWORD`) and creator (`formation_fields::CREATOR`) empty and the display order
/// (`formation_fields::DISPLAY_ORDER`, 11 bytes) zeroed; the 11 slots get the default 4-4-2 through
/// `Formation::set_slot_position` (general position, not the extra one: GK; D R, D L, D CL, D CR; M CL, M R, M CR; F CL,
/// F CR; M L), which also sets each slot's pitch coordinates from its positions; the team instructions
/// (`formation_fields::FLAGS`) 0x22a91; each player's instructions (`formation_fields::PLAYER_INSTRUCTIONS`) 0x15552221
/// with 0xa in the high dword; marking types 0x12; no designated people (-1); the position lookup rebuilt
/// (`Formation::build_position_lookup`); changed = 1, no password, not saved. Returns `this`. Runs for the two
/// formations of each side when a match is set up (match_day.cpp, a vector constructor), for each tactics record and for
/// the human manager's own formation.
/// Original: `Formation::Formation` (0x5b27c0, formation.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn formation_new(this: usize) -> usize {
    core::ptr::write_bytes(this as *mut u8, 0, formation_fields::SIZE);
    copy_c_string(this, FORMATION_UNINITIALISED_NAME);
    write_u8(this + formation_fields::PASSWORD, 0);
    write_u8(this + formation_fields::CREATOR, 0);
    core::ptr::write_bytes((this + formation_fields::DISPLAY_ORDER) as *mut u8, 0, 11);
    for (slot, position) in FORMATION_DEFAULT_POSITIONS.iter().enumerate() {
        exe::formation_set_slot_position(this, slot as u32, *position, 0); // sx10_6: the default 4-4-2
    }
    write_u32(this + formation_fields::FLAGS, FORMATION_DEFAULT_TEAM_INSTRUCTIONS); // sx10_6: default team instructions
    formation_reset_instructions(this);
    covered!("sx10_6 default formation");
    this
}

/// Comparer for `formation_new`: side by side over the whole record.
pub(crate) unsafe extern "thiscall" fn compare_formation_new(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_FORMATION_NEW);
    let reg = [(this, formation_fields::SIZE)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || formation_new(this));
    SCOREBOARD.record(same && r == n, || format!("Formation::Formation({:#x}) {:#x}/{:#x} {:?}", this, r, n, first_differences(&mo, &mr)));
    r
}

// ---------- batch 333: the run-up to the start date (game.cpp) ----------

/// Exe routines the run-up to the start date calls without an `exe::` wrapper (cm0102.exe 3.9.68; most are Rust and
/// run through their detour). Conventions checked at the call sites in 0x5cc850.
#[allow(dead_code)]
pub(crate) mod start_date_fns {
    pub(crate) const SET_UP_THE_DAY: usize = 0x6b3c10; // MATCH_DAY::set_up_the_day thiscall(day, date, slice) ret 8
    pub(crate) const PLAY_THE_DAY: usize = 0x6b4360; // MATCH_DAY::play_the_day thiscall(day)
    pub(crate) const END_DAY: usize = 0x6b4190; // MATCH_DAY::end_day thiscall(day)
    pub(crate) const STAFF_RECORDS_DAILY_UPDATE: usize = 0x86dda0; // thiscall(this, date) ret 4
    pub(crate) const CONTRACT_MANAGER_DAILY_UPDATE: usize = 0x4c7ac0; // thiscall(this, date, slice, people) ret 0xc
    pub(crate) const UPDATE_CITY_WEATHER: usize = 0x922ee0; // Weather thiscall(this)
    pub(crate) const DISCIPLINE_DAILY_UPDATE: usize = 0x556240; // thiscall(this, date) ret 4
    pub(crate) const INJURY_DAILY_UPDATE: usize = 0x62b7b0; // thiscall(this)
    pub(crate) const AWARD_MANAGER_PROCESS_DATE: usize = 0x413540; // thiscall(this, date) ret 4
    pub(crate) const HOST_COUNTRY_DAILY_UPDATE: usize = 0x5fa850; // thiscall(this, date) ret 4
    pub(crate) const NATIONAL_TEAMS_DAILY_UPDATE: usize = 0x76b100; // thiscall(this, date) ret 4
    pub(crate) const FINANCE_TABLE_DAILY_UPDATE: usize = 0x59b9c0; // thiscall(this, date) ret 4
    pub(crate) const OFFICIALS_CLEAR_BUSY_FLAGS: usize = 0x79bf40; // thiscall(this)
    pub(crate) const MANAGER_MANAGER_DAILY_UPDATE: usize = 0x68f9c0; // thiscall(this)
    pub(crate) const STADIUM_DAILY_UPDATE: usize = 0x85b820; // thiscall(this, date) ret 4
    pub(crate) const CLUB_RECORDS_SEND_RECORD_NEWS: usize = 0x448e30; // thiscall(this)
    pub(crate) const ASSESS_CLUB_SQUADS: usize = 0x83a1b0; // Shortlist_Manager thiscall(this) ("Assessing Club Squads")
    pub(crate) const INJURY_INIT_SQUAD_FITNESS: usize = 0x62a920; // thiscall(this)
    pub(crate) const GAME_SEASON_UPDATE_PASS: usize = 0x5d5ae0; // cdecl()
    pub(crate) const GAME_UPDATE_NATION_COEFFICIENTS: usize = 0x5d5f20; // cdecl()
    /// The game phase byte saved with the game (+ field of the save at 0xae2cfc): 2 once the run-up is over [inference].
    pub(crate) const G_GAME_PHASE: usize = 0xae2cfc;
    /// The progress bar's status text buffer (the "Initialising Leagues" text).
    pub(crate) const G_STATUS_TEXT: usize = 0xde1f64;
    pub(crate) const TEXT_INITIALISING_LEAGUES: usize = 0x9cd244; // "Initialising Leagues"
}

/// `this` routine of the exe called with no argument (thiscall).
unsafe fn start_date_call0(f: usize, this: usize) {
    let g: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    g(this)
}

/// `this` routine of the exe called with the game date (thiscall, ret 4).
unsafe fn start_date_call_today(f: usize, this: usize) {
    let g: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
    g(this, G_CURRENT_DATE)
}

/// Whether the game date is still before the start date `start` (day i16, year u16): the year unsigned first, then the
/// day of the year signed.
unsafe fn before_start_date(start: &[u16; 4]) -> bool {
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if start[1] != year {
        return start[1] > year;
    }
    start[0] as i16 > read_i16(G_CURRENT_DATE)
}

/// The first part of a day slice: the match day of the slice (built, played, ended and freed; `G_MATCH_DAY` points at
/// it meanwhile), the fixtures, the staff records, the competitions (`game_update_comps_for_date(0)`) and the
/// contracts.
unsafe fn start_date_slice_matches(slice: u8) {
    let day = exe::crt_operator_new(0x38);
    let day = if day != 0 {
        let set_up: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(start_date_fns::SET_UP_THE_DAY);
        set_up(day, G_CURRENT_DATE, slice as u32)
    } else {
        0
    };
    write_usize(G_MATCH_DAY, day);
    start_date_call0(start_date_fns::PLAY_THE_DAY, day); // the original calls it even on a null day
    exe::fixture_manager_daily_update(read_usize(G_FIXTURE_MANAGER));
    start_date_call_today(start_date_fns::STAFF_RECORDS_DAILY_UPDATE, read_usize(G_STAFF_RECORDS));
    game_update_comps_for_date(0);
    let day = read_usize(G_MATCH_DAY);
    if day != 0 {
        start_date_call0(start_date_fns::END_DAY, day);
        exe::crt_operator_delete(day);
    }
    write_usize(G_MATCH_DAY, 0);
    let contracts: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(start_date_fns::CONTRACT_MANAGER_DAILY_UPDATE);
    contracts(read_usize(G_CONTRACT_MANAGER), G_CURRENT_DATE, read_u8(G_CURRENT_DAY_SLICE) as u32, 1); // the slice read again
}

/// The third slice of a day: every person's age (`staff::AGE`, from `exe::database_get_staff_age` on the game date),
/// the weather, the "Initialising Leagues" progress step, then the world's daily updates in the original's order.
unsafe fn start_date_slice_world(counter: &mut i32, total: i32) {
    for i in 0..read_usize(G_STAFF_COUNT) as i32 {
        let p = staff_at(i as isize);
        p.set_age(exe::database_get_staff_age(p, G_CURRENT_DATE as *const u16) as i16);
    }
    start_date_call0(start_date_fns::UPDATE_CITY_WEATHER, read_usize(G_WEATHER));
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(start_date_fns::G_STATUS_TEXT, start_date_fns::TEXT_INITIALISING_LEAGUES);
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, start_date_fns::G_STATUS_TEXT, *counter, total as u32, -1);
    *counter += 1;
    start_date_call_today(start_date_fns::DISCIPLINE_DAILY_UPDATE, read_usize(G_DISCIPLINE));
    start_date_call0(start_date_fns::INJURY_DAILY_UPDATE, read_usize(G_INJURY));
    exe::uefaseeding_update_season(read_usize(G_UEFA_SEEDING), 1);
    start_date_call_today(start_date_fns::AWARD_MANAGER_PROCESS_DATE, read_usize(G_AWARD_MANAGER));
    game_update_comps_for_date(1); // the competitions' day (phase 1) and the fixtures again
    start_date_call_today(start_date_fns::HOST_COUNTRY_DAILY_UPDATE, read_usize(G_HOST_COUNTRY));
    let cdecl0 = |f: usize| {
        let g: unsafe extern "C" fn() = core::mem::transmute(f);
        g()
    };
    cdecl0(start_date_fns::GAME_SEASON_UPDATE_PASS);
    cdecl0(start_date_fns::GAME_UPDATE_NATION_COEFFICIENTS);
    start_date_call_today(start_date_fns::NATIONAL_TEAMS_DAILY_UPDATE, read_usize(G_NATIONAL_TEAMS));
    start_date_call_today(start_date_fns::FINANCE_TABLE_DAILY_UPDATE, read_usize(G_FINANCE_TABLE));
    start_date_call0(start_date_fns::OFFICIALS_CLEAR_BUSY_FLAGS, read_usize(G_OFFICIALS_MANAGER));
    start_date_call0(start_date_fns::MANAGER_MANAGER_DAILY_UPDATE, read_usize(G_MANAGER_MANAGER));
    start_date_call_today(start_date_fns::STADIUM_DAILY_UPDATE, read_usize(G_STADIUM));
}

/// After the third slice the date moves on one day (`exe::date_add_days`, `exe::date_operator_3d`), and the FIFA
/// transfer rules (`G_FIFA_TRANSFER_RULES_FEATURE_ENABLED` on and not yet `G_FIFA_TRANSFER_RULES_IN_FORCE`) check their
/// date `G_TRANSFER_RULE_CHANGE_DATE`: already past (year unsigned, then day signed), the rules come into force
/// (`G_MAX_CONTRACT_LENGTH_YEARS` = 5, news `exe::news_post_transfer_system_change(1)`); exactly 25 days before it, the
/// warning news (`news_post_transfer_system_change(0)`).
unsafe fn start_date_next_day() {
    let (mut out, mut tmp) = ([0u16; 4], [0u16; 4]);
    let next = exe::date_add_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, 1);
    exe::date_operator_3d(G_CURRENT_DATE, tmp.as_mut_ptr() as usize, next);
    if read_u8(G_FIFA_TRANSFER_RULES_FEATURE_ENABLED) == 0 || read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) != 0 {
        return;
    }
    let (rule_day, rule_year) = (read_i16(G_TRANSFER_RULE_CHANGE_DATE), read_u16(G_TRANSFER_RULE_CHANGE_DATE + 2));
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if rule_year < year || (rule_year == year && rule_day < read_i16(G_CURRENT_DATE)) {
        covered!("sx11_1 transfer rules in force");
        write_u8(G_FIFA_TRANSFER_RULES_IN_FORCE, 1);
        write_u8(G_MAX_CONTRACT_LENGTH_YEARS, 5);
        exe::news_post_transfer_system_change(1);
        return;
    }
    let mut warn = [0u16; 4];
    let d = exe::date_sub_days(G_TRANSFER_RULE_CHANGE_DATE, warn.as_mut_ptr() as usize, 0x19);
    if read_u16(d + 2) == read_u16(G_CURRENT_DATE_YEAR) && read_i16(d) == read_i16(G_CURRENT_DATE) {
        covered!("sx11_1 transfer rules warning");
        exe::news_post_transfer_system_change(0);
    }
}

/// Runs the game day by day from the game date to the chosen start date before the career begins.
///
/// First the player setup files (`player_setup_cfg_load` with the masks 0x100, 0x1000, 0x800 and 0x200, no error
/// reports). The progress total is the number of days to go: (start year − game year) × 365 − game day + start day.
/// Then, while the game date is before `G_GAME_START_DATE` (year unsigned, then day signed), each of the three slices of
/// a day (`G_CURRENT_DAY_SLICE` 0..2):
///
/// - a new year sets the fixtures' year (`exe::fixture_manager_set_year`);
/// - the matches, fixtures, staff records, competitions and contracts of the slice (`start_date_slice_matches`);
/// - on the third slice, the ages, the weather, the "Initialising Leagues" progress step and the world's daily updates
///   (`start_date_slice_world`: discipline, injuries, UEFA seeding, awards, competitions, host countries, the season
///   pass, the nation coefficients, national teams, finances, officials, managers, stadiums);
/// - the club records' news (`Club_Records::send_record_news`);
/// - the slice moves on; after the third, the date moves one day (`start_date_next_day`, with the FIFA rules check);
/// - the Windows messages (`exe::pump_messages`).
///
/// At the end `G_LAST_PROCESSED_DATE` is the day before the game date, the game phase byte 0xae2cfc becomes 2, the
/// computer clubs assess their squads (shortlists, `Shortlist_Manager` at `[G_TRANSFER_MANAGER] + 0x856`), the player
/// setup file is read with mask 0x203b and the squads' fitness is set (`Injury::init_squad_fitness`).
///
/// The original passes the year and the slice in registers whose high bits are leftovers; the callees read only the
/// word or the byte (`set_year` compares `bx`, `set_up_the_day` and the contracts read the byte).
/// Original: `game_run_to_start_date` (0x5cc850, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_run_to_start_date() {
    for mask in [0x100u32, 0x1000, 0x800, 0x200] {
        let load: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
        load(mask, 0);
    }
    let start: [u16; 4] = core::array::from_fn(|k| read_u16(G_GAME_START_DATE + 2 * k));
    let total = (start[1] as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32) * 365 - read_i16(G_CURRENT_DATE) as i32 + start[0] as i16 as i32;
    let (mut last_year, mut counter) = (0u16, 0i32);
    loop {
        let start: [u16; 4] = core::array::from_fn(|k| read_u16(G_GAME_START_DATE + 2 * k)); // re-read every day, as the original
        if !before_start_date(&start) {
            break;
        }
        let year = read_u16(G_CURRENT_DATE_YEAR);
        if year != last_year {
            exe::fixture_manager_set_year(read_usize(G_FIXTURE_MANAGER), year as u32);
            last_year = read_u16(G_CURRENT_DATE_YEAR);
        }
        let slice = read_u8(G_CURRENT_DAY_SLICE);
        start_date_slice_matches(slice);
        if read_u8(G_CURRENT_DAY_SLICE) == 2 {
            start_date_slice_world(&mut counter, total);
        }
        start_date_call0(start_date_fns::CLUB_RECORDS_SEND_RECORD_NEWS, read_usize(G_CLUB_RECORDS));
        let next = read_u8(G_CURRENT_DAY_SLICE).wrapping_add(1);
        write_u8(G_CURRENT_DAY_SLICE, next);
        if next > 2 {
            write_u8(G_CURRENT_DAY_SLICE, 0);
            start_date_next_day();
        }
        exe::pump_messages();
    }
    let (mut out, mut tmp) = ([0u16; 4], [0u16; 4]);
    let yesterday = exe::date_sub_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, 1);
    exe::date_operator_3d(G_LAST_PROCESSED_DATE, tmp.as_mut_ptr() as usize, yesterday);
    write_u8(start_date_fns::G_GAME_PHASE, 2);
    start_date_call0(start_date_fns::ASSESS_CLUB_SQUADS, read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856)); // the Shortlist_Manager
    let load: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
    load(0x203b, 0);
    start_date_call0(start_date_fns::INJURY_INIT_SQUAD_FITNESS, read_usize(G_INJURY));
}

/// Comparer for `game_run_to_start_date`: it plays every day up to the start date and cannot run twice, so only the
/// original runs and the plan is checked afterwards: the game date reached the start date (when it was before it), the
/// last processed date is the day before, no match day is left, the slice is back to 0 and the phase byte is 2.
pub(crate) unsafe extern "C" fn compare_game_run_to_start_date() {
    let start: [u16; 4] = core::array::from_fn(|k| read_u16(G_GAME_START_DATE + 2 * k));
    let ran = before_start_date(&start);
    original_fn::<unsafe extern "C" fn()>(IDX_GAME_RUN_TO_START_DATE)();
    let mut out = [0u16; 4];
    let yesterday = exe::date_sub_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, 1);
    let last_ok = read_u16(G_LAST_PROCESSED_DATE) == read_u16(yesterday) && read_u16(G_LAST_PROCESSED_DATE + 2) == read_u16(yesterday + 2);
    let date_ok = !ran || (read_u16(G_CURRENT_DATE) == start[0] && read_u16(G_CURRENT_DATE_YEAR) == start[1] && read_u8(G_CURRENT_DAY_SLICE) == 0);
    let ok = last_ok && date_ok && read_usize(G_MATCH_DAY) == 0 && read_u8(start_date_fns::G_GAME_PHASE) == 2;
    SCOREBOARD.record(ok, || format!("game_run_to_start_date: ran {} date {}/{} slice {} (start {}/{}), last processed {}/{}, match day {:x}, phase {}",
        ran, read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR), read_u8(G_CURRENT_DAY_SLICE), start[0] as i16, start[1],
        read_i16(G_LAST_PROCESSED_DATE), read_u16(G_LAST_PROCESSED_DATE + 2), read_usize(G_MATCH_DAY), read_u8(start_date_fns::G_GAME_PHASE)));
}

// ---------- batch 333: the ability order and the game's static dates (game.cpp) ----------

/// `compare_shorts_ascending(a, b)`: qsort comparison of two 16-bit values, ascending: `*a − *b`, both sign-extended.
/// Used by the daily person pass (`game_daily_staff_update` 0x5ce2d0) to sort the two tables of the day's abilities
/// (current and potential) that the players' development reads.
/// Original: `compare_shorts_ascending` (0x5d6200, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_shorts_ascending(a: *const i16, b: *const i16) -> i32 {
    read_i16(a as usize) as i32 - read_i16(b as usize) as i32
}

/// Comparer for `compare_shorts_ascending`: the order of the original and of the copy.
pub(crate) unsafe extern "C" fn compare_compare_shorts_ascending(a: *const i16, b: *const i16) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const i16, *const i16) -> i32>(IDX_COMPARE_SHORTS_ASCENDING)(a, b);
    let n = compare_shorts_ascending(a, b);
    SCOREBOARD.record(r == n, || format!("compare_shorts_ascending({}, {}) original={} rust={}", read_i16(a as usize), read_i16(b as usize), r, n));
    r
}

/// Builds a game date with the default `Date` (`exe::date_date_54a110`: day 40 of 1972).
unsafe fn construct_static_date(date: usize) {
    exe::date_date_54a110(date);
}

/// `game_date_static_init()`: static initialiser of the exe (runs in the CRT start-up, before WinMain, with the detours
/// already installed by the DLL load): builds the game date `G_CURRENT_DATE` with the default `Date` and registers its
/// destructor (`game_date_static_dtor` 0x5cc670, an empty body) with `exe::crt_atexit`. The game date is set for real
/// when a game is created or loaded.
/// Original: `game_date_static_init` (0x5cc650, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_date_static_init() {
    construct_static_date(G_CURRENT_DATE);
    exe::crt_atexit(0x5cc670usize); // game_date_static_dtor: an empty body
}

/// `last_processed_date_static_init()`: static initialiser of the exe, like `game_date_static_init`, for the last
/// processed date `G_LAST_PROCESSED_DATE` (destructor `last_processed_date_static_dtor` 0x5cc6a0, an empty body).
/// Original: `last_processed_date_static_init` (0x5cc680, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn last_processed_date_static_init() {
    construct_static_date(G_LAST_PROCESSED_DATE);
    exe::crt_atexit(0x5cc6a0usize); // last_processed_date_static_dtor: an empty body
}

/// Comparer of a static date initialiser: the original runs (it registers the destructor once); the copy builds the date
/// into a local and its 8 bytes are compared with the global's.
unsafe fn compare_static_date(name: &str, original: unsafe extern "C" fn(), global: usize) {
    original();
    let mut copy = [0u16; 4];
    construct_static_date(copy.as_mut_ptr() as usize);
    let orig: [u16; 4] = core::array::from_fn(|k| read_u16(global + 2 * k));
    SCOREBOARD.record(copy == orig, || format!("{}: original={:x?} rust={:x?}", name, orig, copy));
}

/// Comparer for `game_date_static_init`.
pub(crate) unsafe extern "C" fn compare_game_date_static_init() {
    compare_static_date("game_date_static_init", original_fn::<unsafe extern "C" fn()>(IDX_GAME_DATE_STATIC_INIT), G_CURRENT_DATE);
}

/// Comparer for `last_processed_date_static_init`.
pub(crate) unsafe extern "C" fn compare_last_processed_date_static_init() {
    compare_static_date("last_processed_date_static_init", original_fn::<unsafe extern "C" fn()>(IDX_LAST_PROCESSED_DATE_STATIC_INIT), G_LAST_PROCESSED_DATE);
}

// --- sx01d_1: Media::apply_team_story_response (0x73e870, media.cpp, 10,160 bytes), the whole routine in named pieces ---

/// The constants of `apply_team_story_response` that have no name in `rdata_consts.rs` yet (cm0102.exe .rdata); the
/// f32 30.0 is `STORY_F32_30` (sx01c_1).
const TEAM_STORY_F32_67: usize = 0x96d8a8; // f32 = 67.0
const TEAM_STORY_F32_NEG_30: usize = 0x96d8a4; // f32 = -30.0
const TEAM_STORY_F32_NEG_1500: usize = 0x96d8a0; // f32 = -1500.0
const TEAM_STORY_F64_NEG_0_25: usize = 0x96d218; // f64 = -0.25
const TEAM_STORY_F64_NEG_250: usize = 0x96d890; // f64 = -250.0
const TEAM_STORY_F64_NEG_500: usize = 0x96d898; // f64 = -500.0
const TEAM_STORY_F64_250: usize = 0x968898; // f64 = 250.0

/// What `apply_team_story_response` keeps in its frame.
struct TeamStory {
    this: usize,
    club: Club,
    /// the person the story is about (may be null), passed on to the confidence adjustments
    subject: usize,
    /// the story kind as passed (its word picks the branch; some branches pass the whole dword on)
    kind: u32,
    /// the manager's answer as passed (its low byte, 1..3, picks the branch; the whole dword is passed on)
    response: u32,
    /// the club's division stats object (0 for a national team)
    stats: usize,
    /// the chairman's weight: (ambition - 10) x 0.05 + 1, 0.25 without a chairman
    weight: f64,
    /// the club's stat 3 in the division stats (0 for a national team), the weight of the story effects
    stat: f32,
    /// trunc(stat x 67): a dword whose low word (`x`) is the story's base
    base: i32,
    /// the kind's multiplier from the board's expectation flags (1.0 by default)
    mult: f64,
}

/// `v`, or `cap` when `cap < v` (the x87 `fcomp`/`fstp` pairs of the original).
fn team_story_at_most(v: f64, cap: f64) -> f64 {
    if cap < v { cap } else { v }
}

impl TeamStory {
    /// The base as the original reads it, `movsx` of the low word.
    fn x(&self) -> i32 {
        self.base as i16 as i32
    }

    /// The low word of `high` replaced by `t` (the original writes a word into a dword slot and passes the dword on).
    fn word_into(high: u32, t: i32) -> u32 {
        (high & 0xffff_0000) | (t as u32 & 0xffff)
    }

    /// The club's `board_record::EXPECTATION_FLAGS` word.
    unsafe fn expectations(&self) -> u16 {
        read_u16(manager_manager_row(self.club) + board_record::EXPECTATION_FLAGS)
    }

    /// Clears `clear` and sets `set` in byte `k` (0 or 1) of the club's expectation flags.
    unsafe fn swap_expectations(&self, k: usize, clear: u8, set: u8) {
        let p = manager_manager_row(self.club) + board_record::EXPECTATION_FLAGS + k;
        write_u8(p, read_u8(p) & !clear | set);
    }

    /// The division's competition object in `G_COMP_OBJECT_TABLE` (read without a null check, as the original).
    unsafe fn competition_object(&self) -> usize {
        read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(self.club.division().addr())))
    }

    /// The club's entry in the division stats exists and its byte +4 is below 25 [inference: the league position].
    unsafe fn table_entry_ok(&self) -> bool {
        exe::comp_stats_find_club_entry(self.stats, self.club) != 0
            && read_u8(exe::comp_stats_find_club_entry(self.stats, self.club) + 4) < 0x19
    }

    /// `Media::adjust_supporter_confidence(club, subject, kind, target, response)`.
    unsafe fn supporters(&self, kind: u32, target: u32) {
        player_story_supporters(self.this, self.club, Staff(self.subject), kind, target as i32, self.response)
    }

    /// `Media::adjust_board_confidence(club, subject, kind, target, response, sixth)`.
    unsafe fn board(&self, kind: u32, target: u32, sixth: u32) {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(player_story_fns::ADJUST_BOARD_CONFIDENCE);
        f(self.this, self.club.addr(), self.subject, kind, target, self.response, sixth)
    }

    /// `Media::apply_story_effect_flags(club, kind, flags, stat, response, change, player)`.
    unsafe fn effects(&self, kind: u32, flags: u32, change: u32, player: usize) {
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, usize) = core::mem::transmute(player_story_fns::APPLY_STORY_EFFECT_FLAGS);
        f(self.this, self.club.addr(), kind, flags, self.stat.to_bits(), self.response, change, player)
    }

    /// Adds trunc(v) to the club's `board_record::STAFF_CONFIDENCE`, kept between 1 and 10000.
    unsafe fn staff_confidence_add(&self, v: f64) {
        add_clamped_confidence(manager_manager_row(self.club) + board_record::STAFF_CONFIDENCE, ftol(v) as i16);
    }
}

/// `Media::apply_team_story_response(club, kind, response, subject)`: the effects of the manager's answer (1, 2 or 3,
/// the low byte of `response`) to a media story about his club, through the supporters' confidence
/// (`Media::adjust_supporter_confidence`), the board's (`Media::adjust_board_confidence`), the staff's
/// (`board_record::STAFF_CONFIDENCE`) and the story effects on the squad (`Media::apply_story_effect_flags`, weighted by
/// the club's stat).
///
/// Nothing without the club, its manager (+0xcf) with a `.non_player()` sheet or, for a club that is not a national
/// team (`exe::database_is_national_team`), its division's object in `G_COMP_OBJECT_TABLE` with the division stats
/// (+8). The chairman's weight is (`.ambition()` - 10) x 0.05 + 1 (0.25 without a chairman); the stat is the club's
/// stat 3 in the division stats (`exe::comp_stats_get_club_stat`, as a float, 0 for a national team) and the base
/// x = trunc(stat x 67) (a word). The kind picks the piece: 0x3c/0x3d `team_story_crowd`, 0x63/0x5b/0x61, 0x62/0x5d,
/// 0x5f and 0x5c `team_story_standard`, 0x3f and 0x60 `team_story_relegation`, 0x40/0x41 `team_story_minus_thousand`
/// (with `team_story_promotion_reply`), 0x43 `team_story_injuries`, 0x42 `team_story_fixed`, 0x52
/// `team_story_subject`, 0x44 and 0x5a `team_story_supporters_only`; any other kind or an answer outside 1..3 does
/// nothing.
///
/// Defects of the original: answer 2 to kind 0x43 with the board expecting promotion or a title (flags & 6) passes the
/// story effects the change computed with the formula of flags & 9 (-500 - stat x 30 instead of -1500 - stat x 40);
/// answer 1 to kind 0x60 passes the story effects trunc(-x / multiplier) while the confidences get +x / multiplier;
/// several targets are a dword whose low word is the computed value and whose high word is the kind's or the base's
/// (the adjustments only use the low word).
/// Original: `Media::apply_team_story_response` (0x73e870, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn apply_team_story_response(this: usize, club: usize, kind: u32, response: u32, subject: usize) {
    let club = Club(club);
    if club.is_null() {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().is_null() {
        return;
    }
    let comp_object = || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(club.division().addr())));
    if exe::database_is_national_team(club) == 0 && (club.division().is_null() || comp_object() == 0) {
        return;
    }
    let mut stats = 0usize;
    if exe::database_is_national_team(club) == 0 {
        stats = read_usize(comp_object() + 8);
        if stats == 0 {
            return;
        }
    }
    let chairman = club.chairman();
    let weight = if chairman.is_null() {
        0.25
    } else {
        (chairman.ambition() as i32 - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1)
    };
    let stat = if stats != 0 { exe::comp_stats_get_club_stat(stats, club, 3) as f32 } else { 0.0 };
    let base = ftol(stat as f64 * read_f32(TEAM_STORY_F32_67) as f64);
    let mut s = TeamStory { this, club, subject, kind, response, stats, weight, stat, base, mult: 1.0 };
    let flags = s.expectations();
    match kind as u16 {
        0x3c | 0x3d => team_story_crowd(&s),
        0x63 | 0x5b | 0x61 => {
            s.mult = if flags & 2 != 0 { 1.5 } else if flags & 1 != 0 { 1.1 } else { 1.0 };
            covered!("sx01d_1 team story bad run");
            team_story_standard(&s, kind, 100, read_f64(rdata::F64_500), false, 0x10, |s| team_story_raise(s, 0x71, 2));
        }
        0x62 | 0x5d => {
            s.mult = if flags & 2 != 0 { 1.75 } else if flags & 1 != 0 { 1.3 } else { 1.0 };
            covered!("sx01d_1 team story good run");
            team_story_standard(&s, kind, 0x14, read_f64(rdata::F64_200), true, 0x50, |s| team_story_raise(s, 0x71, 2));
        }
        0x5f => {
            s.mult = if flags & 4 != 0 { 1.5 } else if flags & 8 != 0 { 1.1 } else { 1.0 };
            covered!("sx01d_1 team story 0x5f");
            team_story_standard(&s, 0x5f, 100, read_f64(rdata::F64_500), true, 0x50, |s| team_story_raise(s, 0x78, 4));
        }
        0x5c => {
            s.mult = if flags & 0x400 != 0 { 1.5 } else { 1.0 };
            covered!("sx01d_1 team story continental");
            team_story_standard(&s, 0x5c, 100, read_f64(rdata::F64_500), true, 0x50, |s| {
                s.swap_expectations(1, 0, 4); // board_record::EXPECTATION_FLAGS | 0x400, no table test
                true
            });
        }
        0x3f | 0x60 => {
            s.mult = team_story_relegation_multiplier(flags, 0.5, 0.75, 1.5, 3.0);
            team_story_relegation(&s, kind as u16 as u32);
        }
        0x43 => team_story_injuries(&s, flags),
        0x40 | 0x41 => {
            s.mult = team_story_relegation_multiplier(flags, 0.1, 0.2, 1.75, 3.0);
            team_story_minus_thousand(&s);
        }
        0x42 => {
            s.mult = if flags & 6 != 0 { 1.5 } else { 1.0 };
            team_story_fixed(&s);
        }
        0x52 => team_story_subject(&s),
        0x44 | 0x5a => team_story_supporters_only(&s, kind as u16 as u32),
        _ => {}
    }
}

/// The multiplier of kinds 0x3f, 0x60, 0x40 and 0x41 by the expectation flags: & 0x20, & 0x10, & 9, & 6 (1.0 without).
fn team_story_relegation_multiplier(flags: u16, a: f64, b: f64, c: f64, d: f64) -> f64 {
    if flags & 0x20 != 0 {
        a
    } else if flags & 0x10 != 0 {
        b
    } else if flags & 9 != 0 {
        c
    } else if flags & 6 != 0 {
        d
    } else {
        1.0
    }
}

/// Kinds 0x63/0x5b/0x61, 0x62/0x5d, 0x5f and 0x5c, by answer:
/// - 3: when the stat - 5 is below draw(-100), the supporters get trunc(x × multiplier) and the board that × the
///   chairman's weight; story effects 4;
/// - 2: the supporters get trunc(-x × multiplier) and the board trunc(-x × multiplier × weight), both at most 0; story
///   effects 8 with trunc(x × -0.25), at most 0;
/// - 1: the supporters get trunc(x × multiplier); when draw(`draw`) is below the stat and `raise` raises the
///   expectations (`team_story_raise`; 0x5c sets 0x400 without the table test), the board gets trunc(x × multiplier × weight + `bonus`) with the sixth argument
///   1, otherwise without the bonus and 0; then with `staff` the staff's confidence moves by trunc(x × 0.25) and the
///   story effects get `flags` with 0, otherwise `flags` with trunc(x × 0.25).
unsafe fn team_story_standard(s: &TeamStory, kind: u32, draw: i32, bonus: f64, staff: bool, flags: u32, raise: impl FnOnce(&TeamStory) -> bool) {
    let x = s.x() as f64;
    match s.response as i8 {
        3 => {
            let f = (s.stat as f64 - read_f32(rdata::F32_5) as f64) as f32;
            if exe::utils_random(-100) as f64 > f as f64 {
                covered!("sx01d_1 team story standard 3");
                let p = x * s.mult;
                s.supporters(kind, ftol(p) as u32);
                s.board(kind, ftol(p * s.weight) as u32, 0);
            }
            s.effects(kind, 4, 0, 0);
        }
        2 => {
            covered!("sx01d_1 team story standard 2");
            let p = x * s.mult;
            s.supporters(kind, ftol(team_story_at_most(-p, read_f64(rdata::F64_0))) as u32);
            s.board(kind, ftol(team_story_at_most(-(p * s.weight), read_f64(rdata::F64_0))) as u32, 0);
            s.effects(kind, 8, ftol(team_story_at_most(x * read_f64(TEAM_STORY_F64_NEG_0_25), read_f64(rdata::F64_0))) as u32, 0);
        }
        1 => {
            let p = x * s.mult;
            s.supporters(kind, ftol(p) as u32);
            if (exe::utils_random(draw) as f64) < s.stat as f64 && raise(s) {
                covered!("sx01d_1 team story standard 1 raised");
                s.board(kind, ftol(p * s.weight + bonus) as u32, 1);
            } else {
                s.board(kind, ftol(p * s.weight) as u32, 0);
            }
            if staff {
                s.staff_confidence_add(x * read_f64(rdata::F64_0_25));
                s.effects(kind, flags, 0, 0);
            } else {
                s.effects(kind, flags, ftol(x * read_f64(rdata::F64_0_25)) as u32, 0);
            }
        }
        _ => {}
    }
}

/// The raised expectations of answer 1 to kinds 0x63/0x5b/0x61, 0x62/0x5d and 0x5f: with `table_entry_ok`, the
/// expectation flags lose `clear` and take `set`.
unsafe fn team_story_raise(s: &TeamStory, clear: u8, set: u8) -> bool {
    if !s.table_entry_ok() {
        return false;
    }
    s.swap_expectations(0, clear, set);
    true
}

/// Answers 3 and 2 of kinds 0x3f, 0x60, 0x40 and 0x41, with e = -1000 - x:
/// - 3 (`won`): the supporters get trunc(e × multiplier) and the board that × the weight, both at most 0; story effects
///   8 with e (at most 0); without `won` story effects 4;
/// - 2: the same confidences; story effects 8 with trunc(e × 0.5), at most 0.
unsafe fn team_story_minus_thousand_low(s: &TeamStory, kind: u32, won: bool) {
    let e = -1000 - s.x();
    if s.response as i8 == 3 && !won {
        s.effects(kind, 4, 0, 0);
        return;
    }
    let p = e as f64 * s.mult;
    s.supporters(kind, ftol(team_story_at_most(p, read_f64(rdata::F64_0))) as u32);
    s.board(kind, ftol(team_story_at_most(p * s.weight, read_f64(rdata::F64_0))) as u32, 0);
    if s.response as i8 == 3 {
        covered!("sx01d_1 team story minus thousand 3");
        s.effects(kind, 8, e.min(0) as u32, 0);
    } else {
        covered!("sx01d_1 team story minus thousand 2");
        s.effects(kind, 8, ftol(team_story_at_most(e as f64 * read_f64(rdata::F64_0_5), read_f64(rdata::F64_0))) as u32, 0);
    }
}

/// The board's part of answer 1 to kinds 0x3f, 0x60, 0x40 and 0x41 with the capped value c (at most 500): with
/// `raised` trunc(c × weight + 250) and the sixth argument 1, otherwise trunc(c × weight) and 0.
unsafe fn team_story_promotion_board(s: &TeamStory, kind: u32, c: f64, raised: bool) {
    if raised {
        covered!("sx01d_1 team story expectations raised");
        s.board(kind, ftol(c * s.weight + read_f64(TEAM_STORY_F64_250)) as u32, 1);
    } else {
        s.board(kind, ftol(c * s.weight) as u32, 0);
    }
}

/// Kinds 0x3f and 0x60: answers 3 and 2 by `team_story_minus_thousand_low` (answer 3 when draw(100) is below the
/// stat + 5). Answer 1: q = -x / multiplier for 0x3f, +x / multiplier for 0x60, at most 500 (c); the supporters get
/// trunc(c); when draw(100) is below the stat + 5, `table_entry_ok` and the expectation flags have 0x20, the flags
/// trade 0x20 for 0x10 and the board is raised (`team_story_promotion_board`); story effects 0x50 with trunc(c) for
/// 0x3f and trunc(-x / multiplier), at most 500, for 0x60.
unsafe fn team_story_relegation(s: &TeamStory, kind: u32) {
    let up = |s: &TeamStory| (exe::utils_random(100) as f64) < ((s.stat as f64 + read_f32(rdata::F32_5) as f64) as f32) as f64;
    match s.response as i8 {
        3 => {
            let won = up(s);
            team_story_minus_thousand_low(s, kind, won);
        }
        2 => team_story_minus_thousand_low(s, kind, false),
        1 => {
            covered!("sx01d_1 team story relegation 1");
            let x = s.x() as f64;
            let q = if kind == 0x3f { -x / s.mult } else { x / s.mult };
            let c = team_story_at_most(q, read_f64(rdata::F64_500));
            s.supporters(kind, ftol(c) as u32);
            let raised = up(s) && s.table_entry_ok() && s.expectations() & 0x20 != 0;
            if raised {
                s.swap_expectations(0, 0x20, 0x10);
            }
            team_story_promotion_board(s, kind, c, raised);
            let change = if kind == 0x3f { c } else { team_story_at_most(-x / s.mult, read_f64(rdata::F64_500)) };
            s.effects(kind, 0x50, ftol(change) as u32, 0);
        }
        _ => {}
    }
}

/// Kinds 0x40 and 0x41 (the whole kind dword is passed on): answers 3 and 2 by `team_story_minus_thousand_low`
/// (answer 3 when the stat - 5 is below draw(-100)). Answer 1: c = -x / multiplier, at most 500; the supporters get
/// trunc(c); with `table_entry_ok` the expectation flags may be raised (`team_story_promotion_reply`) and the board
/// follows (`team_story_promotion_board`); story effects 0x50 with trunc(c).
unsafe fn team_story_minus_thousand(s: &TeamStory) {
    match s.response as i8 {
        3 => {
            let f = (s.stat as f64 - read_f32(rdata::F32_5) as f64) as f32;
            let won = exe::utils_random(-100) as f64 > f as f64;
            team_story_minus_thousand_low(s, s.kind, won);
        }
        2 => team_story_minus_thousand_low(s, s.kind, false),
        1 => {
            covered!("sx01d_1 team story minus thousand 1");
            let c = team_story_at_most(-(s.x() as f64) / s.mult, read_f64(rdata::F64_500));
            s.supporters(s.kind, ftol(c) as u32);
            let raised = s.table_entry_ok() && team_story_promotion_reply(s);
            team_story_promotion_board(s, s.kind, c, raised);
            s.effects(s.kind, 0x50, ftol(c) as u32, 0);
        }
        _ => {}
    }
}

/// The raised expectations of answer 1 to kinds 0x40 and 0x41: up to four draws(100), each below the stat + 5, tried
/// in turn: flags 8 become 4; 1 becomes 2; 0x40 becomes 8 when the division's object has +0x1c not -1 and the byte
/// +0xbf set; 0x40 becomes 1 when +0x1c is -1. The first that applies raises the board.
unsafe fn team_story_promotion_reply(s: &TeamStory) -> bool {
    let f = ((s.stat as f64 + read_f32(rdata::F32_5) as f64) as f32) as f64;
    if (exe::utils_random(100) as f64) < f && s.expectations() & 8 != 0 {
        s.swap_expectations(0, 8, 4);
        return true;
    }
    if (exe::utils_random(100) as f64) < f && s.expectations() & 1 != 0 {
        s.swap_expectations(0, 1, 2);
        return true;
    }
    if (exe::utils_random(100) as f64) < f && s.expectations() & 0x40 != 0 {
        let o = s.competition_object();
        if read_i32(o + 0x1c) != -1 && read_u8(o + 0xbf) != 0 {
            // +0x1c and +0xbf of the division's competition object, unnamed
            s.swap_expectations(0, 0x40, 8);
            return true;
        }
    }
    if (exe::utils_random(100) as f64) < f && s.expectations() & 0x40 != 0 && read_i32(s.competition_object() + 0x1c) == -1 {
        s.swap_expectations(0, 0x40, 1);
        return true;
    }
    false
}

/// Kind 0x43 (the multiplier and the weight are not used). Answer 3: story effects 4. Answer 2, by the expectation
/// flags: & 0x20 the supporters and the board get 0 and story effects 4; & 0x10 v = stat × -30 (a float, at most 0) to
/// both and story effects 4 with v; & 9 v = -500 - stat × 30 and story effects 8 with v; & 6 v = -1500 - stat × 40 and
/// story effects 8 with -500 - stat × 30 (the original's slip); otherwise story effects 8. Answer 1: & 0x20 or & 0x10
/// the supporters get trunc(v + 200) with v = stat × 20 or × 30, and when draw(100) is below the stat + 5 and
/// `table_entry_ok`, the flags trade 0x20 for 0x10 (0x10 for 0x40) and the board gets trunc(v + 400), otherwise the
/// supporters' value (in a dword with the kind's or the base's high word); story effects 0x50 with that dword, or with
/// trunc(stat × -30) (at most 0) for 0x10; & 9 and & 6 t = trunc(stat × 20) (- 150 for 6) to both and to the story
/// effects 0x50; otherwise story effects 0x50.
unsafe fn team_story_injuries(s: &TeamStory, flags: u16) {
    let stat = s.stat as f64;
    let zero = read_f32(rdata::F32_0) as f64;
    match s.response as i8 {
        3 => s.effects(0x43, 4, 0, 0),
        2 => {
            covered!("sx01d_1 team story injuries 2");
            let both = |v: f32| {
                let c = ftol(team_story_at_most(v as f64, zero)) as u32;
                s.supporters(0x43, c);
                s.board(0x43, c, 0);
                c
            };
            let alt = || ftol(team_story_at_most(read_f32(rdata::F32_NEG_500) as f64 - stat * read_f32(STORY_F32_30) as f64, zero)) as u32;
            if flags & 0x20 != 0 {
                s.supporters(0x43, 0);
                s.board(0x43, 0, 0);
                s.effects(0x43, 4, 0, 0);
            } else if flags & 0x10 != 0 {
                let c = both((stat * read_f32(TEAM_STORY_F32_NEG_30) as f64) as f32);
                s.effects(0x43, 4, c, 0);
            } else if flags & 9 != 0 {
                let c = both((read_f32(rdata::F32_NEG_500) as f64 - stat * read_f32(STORY_F32_30) as f64) as f32);
                s.effects(0x43, 8, c, 0);
            } else if flags & 6 != 0 {
                both((read_f32(TEAM_STORY_F32_NEG_1500) as f64 - stat * read_f32(rdata::F32_40) as f64) as f32);
                s.effects(0x43, 8, alt(), 0);
            } else {
                s.effects(0x43, 8, 0, 0);
            }
        }
        1 => {
            covered!("sx01d_1 team story injuries 1");
            if flags & 0x30 != 0 {
                let (scale, clear, set) = if flags & 0x20 != 0 { (read_f32(rdata::F32_20), 0x20, 0x10) } else { (read_f32(STORY_F32_30), 0x10, 0x40) };
                let v = (stat * scale as f64) as f32;
                let t = ftol(stat * scale as f64 + read_f32(rdata::F32_200) as f64);
                s.supporters(0x43, t as u32);
                let high = if flags & 0x20 != 0 { s.kind } else { s.base as u32 };
                let kept = TeamStory::word_into(high, t);
                let up = (exe::utils_random(100) as f64) < ((stat + read_f32(rdata::F32_5) as f64) as f32) as f64;
                if up && s.table_entry_ok() {
                    s.swap_expectations(0, clear, set);
                    s.board(0x43, ftol(v as f64 + read_f32(rdata::F32_400) as f64) as u32, 0);
                } else {
                    s.board(0x43, kept, 0);
                }
                let change = if flags & 0x20 != 0 { kept } else { ftol(team_story_at_most(stat * read_f32(TEAM_STORY_F32_NEG_30) as f64, zero)) as u32 };
                s.effects(0x43, 0x50, change, 0);
            } else if flags & 0xf != 0 {
                let mut v = stat * read_f32(rdata::F32_20) as f64;
                if flags & 9 == 0 {
                    v -= read_f32(rdata::F32_150) as f64;
                }
                let t = TeamStory::word_into(s.kind, ftol(v));
                s.supporters(0x43, t);
                s.board(0x43, t, 0);
                s.effects(0x43, 0x50, t, 0);
            } else {
                s.effects(0x43, 0x50, 0, 0);
            }
        }
        _ => {}
    }
}

/// Kind 0x42. Answer 3: story effects 4. Answer 2: the supporters get trunc(multiplier × -500), the board
/// trunc(multiplier × weight × -500), story effects 8 with trunc(multiplier × -250). Answer 1: q = 500 / multiplier;
/// the supporters get trunc(q) (in a dword with the kind's high word), the board trunc(q × weight), story effects 0x50
/// with the supporters' dword.
unsafe fn team_story_fixed(s: &TeamStory) {
    match s.response as i8 {
        3 => s.effects(0x42, 4, 0, 0),
        2 => {
            covered!("sx01d_1 team story 0x42 2");
            s.supporters(0x42, ftol(s.mult * read_f64(TEAM_STORY_F64_NEG_500)) as u32);
            s.board(0x42, ftol(s.mult * s.weight * read_f64(TEAM_STORY_F64_NEG_500)) as u32, 0);
            s.effects(0x42, 8, ftol(s.mult * read_f64(TEAM_STORY_F64_NEG_250)) as u32, 0);
        }
        1 => {
            covered!("sx01d_1 team story 0x42 1");
            let q = read_f64(rdata::F64_500) / s.mult;
            let t = TeamStory::word_into(s.kind, ftol(q));
            s.supporters(0x42, t);
            s.board(0x42, ftol(q * s.weight) as u32, 0);
            s.effects(0x42, 0x50, t, 0);
        }
        _ => {}
    }
}

/// Kind 0x52, about a person (nothing without the subject). The supporters move by 200 or 1000 when the club favours
/// (`exe::club_favours_staff`) or dislikes (`exe::club_dislikes_staff`) the subject, the board by the weight × 200 or
/// × 1000 when the chairman favours (`exe::staff_is_favoured_by`) or dislikes him; the sign follows the answer: 3
/// (favoured -200 / disliked +200, story effects 4), 2 (-1000 / +1000, story effects 0x80), 1 (+1000 / -1000, story
/// effects 0x300); the story effects get the subject.
unsafe fn team_story_subject(s: &TeamStory) {
    if s.subject == 0 {
        return;
    }
    let (fans, board, flags) = match s.response as i8 {
        3 => (-200, (rdata::F64_NEG_200, rdata::F64_200), 4),
        2 => (-1000, (rdata::F64_NEG_1000, rdata::F64_1000), 0x80),
        1 => (1000, (rdata::F64_1000, rdata::F64_NEG_1000), 0x300),
        _ => return,
    };
    covered!("sx01d_1 team story subject");
    let subject = Staff(s.subject);
    if exe::club_favours_staff(subject, s.club) != 0 {
        s.supporters(0x52, fans as u32);
    } else if exe::club_dislikes_staff(subject, s.club) != 0 {
        s.supporters(0x52, (-fans) as u32);
    }
    let chairman = s.club.chairman();
    if exe::staff_is_favoured_by(subject, chairman) != 0 {
        s.board(0x52, ftol(s.weight * read_f64(board.0)) as u32, 0);
    } else if exe::staff_is_disliked_by(subject, chairman) != 0 {
        s.board(0x52, ftol(s.weight * read_f64(board.1)) as u32, 0);
    }
    s.effects(0x52, flags, 0, s.subject);
}

/// Kinds 0x44 and 0x5a: answer 1 moves the supporters by draw(200), answer 2 by draw(-200); nothing else.
unsafe fn team_story_supporters_only(s: &TeamStory, kind: u32) {
    match s.response as i8 {
        1 => {
            covered!("sx01d_1 team story national 1");
            s.supporters(kind, exe::utils_random(200) as u32);
        }
        2 => s.supporters(kind, exe::utils_random(-200) as u32),
        _ => {}
    }
}

/// Kinds 0x3c and 0x3d (the whole kind dword is passed on). Answer 3: with a negative stat and expectation flags in
/// 0x48f, the supporters get trunc(x × 0.65) and the board that × the weight; otherwise the supporters get
/// trunc((draw(20) - draw(20)) × stat); story effects 0x44. Answer 2: p = x × (0.5 with flags in 0x48f, otherwise the
/// multiplier); the supporters get trunc(-p), the board draw(trunc(-p × weight)); story effects 2 with the base + 100.
/// Answer 1: p = x × (1.5 with flags in 0x48f, 0.65 with 0x30, otherwise the multiplier); the supporters get trunc(p),
/// the board trunc(p × weight); story effects 0x41 with trunc(x × -1.5).
unsafe fn team_story_crowd(s: &TeamStory) {
    let flags = s.expectations();
    let x = s.x() as f64;
    match s.response as i8 {
        3 => {
            covered!("sx01d_1 team story crowd 3");
            if (s.stat as f64) < read_f32(rdata::F32_0) as f64 && flags & 0x48f != 0 {
                let p = x * read_f64(rdata::F64_0_65);
                s.supporters(s.kind, ftol(p) as u32);
                s.board(s.kind, ftol(p * s.weight) as u32, 0);
            } else {
                let d = exe::utils_random(0x14).wrapping_sub(exe::utils_random(0x14));
                s.supporters(s.kind, ftol(d as f64 * s.stat as f64) as u32);
            }
            s.effects(s.kind, 0x44, 0, 0);
        }
        2 => {
            covered!("sx01d_1 team story crowd 2");
            let m = if flags & 0x48f != 0 { read_f64(rdata::F64_0_5) } else { s.mult };
            let p = x * m;
            s.supporters(s.kind, ftol(-p) as u32);
            s.board(s.kind, exe::utils_random(ftol(-(p * s.weight))) as u32, 0);
            s.effects(s.kind, 2, s.base.wrapping_add(100) as u32, 0);
        }
        1 => {
            covered!("sx01d_1 team story crowd 1");
            let m = if flags & 0x48f != 0 {
                read_f64(rdata::F64_1_5)
            } else if flags & 0x30 != 0 {
                read_f64(rdata::F64_0_65)
            } else {
                s.mult
            };
            let p = x * m;
            s.supporters(s.kind, ftol(p) as u32);
            s.board(s.kind, ftol(p * s.weight) as u32, 0);
            s.effects(s.kind, 0x41, ftol(x * read_f64(rdata::F64_NEG_1_5)) as u32, 0);
        }
        _ => {}
    }
}

/// Comparer for `apply_team_story_response`: the routine draws, posts news (through the confidence adjustments) and
/// passes the reaction to the squad, so it cannot run twice; only the original runs. The proof is the Rust careers and
/// the mutations (the routine runs on every answer to a team story).
pub(crate) unsafe extern "thiscall" fn compare_apply_team_story_response(this: usize, club: usize, kind: u32, response: u32, subject: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize)>(IDX_APPLY_TEAM_STORY_RESPONSE)(this, club, kind, response, subject)
}

// --- sx01d_2: Media::queue_obscene_gesture_story (0x7570b0) and Media::queue_wants_first_team_story (0x75a640), media.cpp ---

/// Writes a person's id and three name keys into news slots k..k+3, the common name (+0xc) as -1 when the person has
/// none (the first and second names are read without a test, as the original).
unsafe fn set_person_name_slots_or_none(n: usize, k: i8, p: Staff) {
    exe::newsitem_set_slot(n, k, read_u32(p.addr()));
    exe::newsitem_set_slot(n, k + 1, read_u32(p.first_name() + names::ID));
    exe::newsitem_set_slot(n, k + 2, read_u32(p.second_name() + names::ID));
    let common = p.common_name();
    exe::newsitem_set_slot(n, k + 3, if common != 0 { read_u32(common + names::ID) } else { u32::MAX });
}

/// The shoot-out slot of a match story: 0 without a shoot-out (`fixture::HOME_PENALTIES` 0xff), 1 when `club` won
/// it, 2 when it lost it, none when the shoot-out goals are equal (signed bytes).
pub(crate) unsafe fn news_shootout_result(fixture: Fixture, club: Club) -> Option<u32> {
    let home = read_i8(fixture.addr() + fixture::HOME_PENALTIES);
    let away = read_i8(fixture.addr() + fixture::AWAY_PENALTIES);
    if home == -1 {
        return Some(0);
    }
    if home == away {
        return None;
    }
    covered!("sx01d_2 match story shoot-out");
    let winner = read_usize(fixture.addr() + if home > away { fixture::HOME_CLUB } else { fixture::AWAY_CLUB });
    Some(if winner == club.addr() { 1 } else { 2 })
}

/// Queues news item 0xbcb about a player filmed making obscene gestures to the fans.
///
/// Nothing without the player, his `.player()` sheet with a `.current_reputation()` of at least 3500, or the club. The
/// local news item 0xbcb: 0..3 the player and his names (the common name -1 when he has none), 4 the club id, 5
/// draw(2), then the match record (`exe::fill_match_record_from_fixture` with the club, `exe::write_match_record_to_news`
/// from slot 6) and, in the next slot, the shoot-out: 0 when there was none (`fixture::HOME_PENALTIES` 0xff), 1 when the
/// club won it, 2 when it lost it, no slot when the shoot-out goals are equal. It goes to every human club following the
/// player (`exe::transfer_manager_collect_player_news_recipients` with 0xda, up to 30, only with `G_TRANSFER_MANAGER`)
/// through `exe::news_send_to_club_manager`.
///
/// Called by `apply_obscene_gesture_ban` (0x7daf50, printouts.rs).
/// Original: `Media::queue_obscene_gesture_story` (0x7570b0, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn queue_obscene_gesture_story(_this: usize, player: usize, club: usize, fixture: usize) {
    let mut record = [0u8; 0x20]; // the 0x1f-byte match record
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let (player, club, fixture) = (Staff(player), Club(club), Fixture(fixture));
    if player.is_null() || player.player().is_null() || club.is_null() || (player.player().current_reputation() as i16) < 0xdac {
        return;
    }
    covered!("sx01d_2 obscene gesture story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbcb, 0);
    set_person_name_slots_or_none(n, 0, player);
    exe::newsitem_set_slot(n, 4, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 5, exe::utils_random(2) as u32);
    exe::fill_match_record_from_fixture(fixture, club, record.as_mut_ptr() as usize);
    let mut k: u8 = 6;
    exe::write_match_record_to_news(record.as_mut_ptr() as usize, n, &mut k);
    if let Some(result) = news_shootout_result(fixture, club) {
        exe::newsitem_set_slot(n, k as i8, result);
    }
    let mut clubs = [0usize; 30];
    let count = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0xda, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for &c in clubs.iter().take(count.max(0) as usize) {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, c);
    }
}

/// Comparer for `queue_obscene_gesture_story`: it draws and posts news, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_queue_obscene_gesture_story(this: usize, player: usize, club: usize, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_QUEUE_OBSCENE_GESTURE_STORY)(this, player, club, fixture)
}

/// Queues news item 0xbe8 about an unhappy player who wants first team football.
///
/// Nothing without the player, his `.nation()`, his club (`.club_job()`) with a `.manager()` and a `.stadium()`, his
/// `.player()` sheet or his contract at the club (`exe::contract_manager_get_contract_for_club`). The local news item
/// 0xbe8: 0..3 the player and his names, 4..7 the manager and his names, 8 the club id, 9 his natural positions
/// (`exe::staff_get_natural_position_mask`), 10 draw(2), 11 the outlet (`exe::media_pick_outlet_type_for_person`), 12
/// the pundit (`exe::punditlist_pick_random_pundit` with the club, the outlet and the club's `.nation()` or -1), 13 the
/// strength flags (`exe::staff_get_player_strength_flags`), 14 the sheet's position mask
/// (`exe::formation_get_position_mask`), 15 `reaction`, 16 the contract's `.morale()`, 17 the player's age, 18 the
/// computer manager's response (`exe::media_pick_ai_manager_response` with 0x83; 0 for a human club), 19 the contract's
/// `.squad_status()` >> 4, 20 one when `.aggression()` + `.temperament()` (a byte) is below 10 and `.ambition()` above
/// 10, 21 `exe::staff_is_transfer_listed_or_requested`. With `reaction` it goes to the club's manager; otherwise to
/// every manager of the player's nation when his `.world_reputation()` is above 4750, else to the club's manager.
///
/// Called by the squad's daily review (0x5d24a2) and by `apply_wants_first_team_response` (the player's reaction).
/// Original: `Media::queue_wants_first_team_story` (0x75a640, media.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn queue_wants_first_team_story(this: usize, player: usize, reaction: u32) {
    let player = Staff(player);
    if player.is_null() || player.nation().is_null() {
        return;
    }
    let club = player.club_job();
    if club.is_null() || club.manager().is_null() || club.stadium().addr() == 0 || player.player().is_null() {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if contract.is_null() {
        return;
    }
    covered!("sx01d_2 wants first team story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbe8, 0);
    let write = news_item_fns().1;
    set_person_name_slots(write, n, 0, player.addr());
    set_person_name_slots(write, n, 4, club.manager().addr());
    exe::newsitem_set_slot(n, 8, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 9, exe::staff_get_natural_position_mask(player));
    exe::newsitem_set_slot(n, 10, exe::utils_random(2) as u32);
    let outlet = exe::media_pick_outlet_type_for_person(player);
    exe::newsitem_set_slot(n, 11, outlet);
    let nation = club.nation().addr();
    exe::newsitem_set_slot(n, 12, exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, outlet, if nation != 0 { read_i32(nation) } else { -1 }) as u32);
    exe::newsitem_set_slot(n, 13, exe::staff_get_player_strength_flags(player, club));
    exe::newsitem_set_slot(n, 14, exe::formation_get_position_mask(player.player()) as i16 as i32 as u32);
    exe::newsitem_set_slot(n, 15, reaction);
    exe::newsitem_set_slot(n, 16, contract.morale() as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 17, player.age() as i8 as i32 as u32);
    let response = if media_is_human_club(club.addr(), 0, 1) == 0 {
        exe::media_pick_ai_manager_response(this, club.manager(), player, 0x83)
    } else {
        0
    };
    exe::newsitem_set_slot(n, 18, response);
    exe::newsitem_set_slot(n, 19, (contract.squad_status() as u8 >> 4) as u32);
    let hot = ((player.player().aggression() as u8).wrapping_add(player.temperament() as u8) as i8) < 10 && player.ambition() as i8 > 10;
    exe::newsitem_set_slot(n, 20, hot as u32);
    exe::newsitem_set_slot(n, 21, exe::staff_is_transfer_listed_or_requested(player, 0));
    if reaction == 0 && player.player().world_reputation() as i16 > 0x128e {
        covered!("sx01d_2 wants first team story to the nation");
        exe::news_send_to_nation_managers(read_usize(G_NEWS), n, player.nation());
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
}

/// Comparer for `queue_wants_first_team_story`: it draws (also through the computer manager's response) and posts
/// news, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_queue_wants_first_team_story(this: usize, player: usize, reaction: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_QUEUE_WANTS_FIRST_TEAM_STORY)(this, player, reaction)
}

// --- sx01d_5: Media::create_player_outburst_story (0x733a60, media.cpp) ---

/// A local news item filled slot after slot (the original's running byte index).
struct NewsSlots {
    n: usize,
    k: u8,
}

impl NewsSlots {
    /// Writes `v` into the next slot.
    unsafe fn put(&mut self, v: u32) {
        exe::newsitem_set_slot(self.n, self.k as i8, v);
        self.k = self.k.wrapping_add(1);
    }

    /// The person's id and his three name keys (read without a test, as the original).
    unsafe fn put_person(&mut self, p: Staff) {
        self.put(read_u32(p.addr()));
        for name in [p.first_name(), p.second_name(), p.common_name()] {
            self.put(read_u32(name + names::ID));
        }
    }

    /// The match record (`exe::fill_match_record_from_fixture` with the club, `exe::write_match_record_to_news` from
    /// the next slot) and the shoot-out slot (`news_shootout_result`); `record` holds the record's date.
    unsafe fn put_match(&mut self, record: &mut [u8; 0x20], fixture: Fixture, club: Club) {
        exe::fill_match_record_from_fixture(fixture, club, record.as_mut_ptr() as usize);
        exe::write_match_record_to_news(record.as_mut_ptr() as usize, self.n, &mut self.k);
        if let Some(result) = news_shootout_result(fixture, club) {
            self.put(result);
        }
    }
}

/// Creates the news item 0xbd9 of a player's outburst at his club or team-mates (or, with the report flag, the
/// assistant manager's report on his attitude) after a match, from the outburst record `story`: +0 the player's staff
/// id, +4 the club index, +0xc a signed byte, +0xe the report flag, +0xf the other person's staff id (-1 none), +0x13
/// the story kind (a word).
///
/// Nothing without the record, the player's `.player()` sheet with a `.current_reputation()` of at least 3500, a club
/// index from 0 to the club count (`G_CLUB_COUNT`, inclusive: one past the table passes, a defect of the original) or
/// the club's `.nation()`. Without the report flag, nothing when the story is already logged (`exe::media_has_logged_story`
/// with 0xbd9 and the kind); with it, nothing without the other person. The slots: the player and his names, the club
/// index, the byte +0xc, the kind, draw(2), the report flag, 30, the computer manager's response
/// (`exe::media_pick_ai_manager_response` with the kind, only without the flag and for a club that is not human;
/// otherwise 0), the other person and his names (-1 x 4 without him), then with a match 1, the match record and the
/// shoot-out (`NewsSlots::put_match`), without one 0, then the outlet (`exe::media_pick_outlet_type_for_person`) and
/// the pundit (`exe::punditlist_pick_random_pundit` with the club's nation). With the report flag it goes to the club's
/// manager; otherwise to every human club following the player (`exe::transfer_manager_collect_player_news_recipients`
/// with 0x1a, up to 30, only with `G_TRANSFER_MANAGER`), then the squad reacts
/// (`exe::media_apply_squad_reaction_to_player_story`) and the story is logged (`log_story`, the kind's byte). Last,
/// without the flag and for a club that is not human, the computer manager answers (0,
/// `exe::media_apply_player_story_response`).
///
/// Called by the post-match events (0x6b7787) and by `create_player_media_story` (0x72c4cd, 0x72cc59); behind the
/// post-match gate (`POS_PROLOGUES`, match_day.rs).
/// Original: `Media::create_player_outburst_story` (0x733a60, media.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn create_player_outburst_story(this: usize, story: usize, fixture: usize) {
    let mut record = [0u8; 0x20]; // the 0x1f-byte match record
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    if story == 0 {
        return;
    }
    let player = staff_at(read_i32(story) as isize);
    if player.player().is_null() || (player.player().current_reputation() as i16) < 0xdac {
        return;
    }
    let index = read_i32(story + 4); // the club index
    if index < 0 || index > read_i32(G_CLUB_COUNT) {
        return;
    }
    let club = club_at(index as isize);
    if club.nation().is_null() {
        return;
    }
    let other_id = read_i32(story + 0xf); // the other person's staff id
    let other = if other_id > -1 { staff_at(other_id as isize) } else { Staff(0) };
    let report = read_u8(story + 0xe) != 0; // the assistant's report flag
    let kind = read_u16(story + 0x13) as u32; // the story kind
    if !report && exe::media_has_logged_story(this, player, club, 0xbd9, kind, core::ptr::null()) != 0 {
        return;
    }
    if report && other.is_null() {
        return;
    }
    covered!("sx01d_5 outburst story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let mut s = NewsSlots { n: local.as_mut_ptr() as usize, k: 0 };
    exe::new_newsitem(s.n, 0xbd9, 0);
    s.put_person(player);
    s.put(index as u32);
    s.put(read_i8(story + 0xc) as i32 as u32);
    s.put(kind as u16 as i16 as i32 as u32);
    s.put(exe::utils_random(2) as u32);
    s.put(read_i8(story + 0xe) as i32 as u32);
    s.put(0x1e);
    let ai = media_is_human_club(club.addr(), 0, 1) == 0 && !report;
    s.put(if ai { exe::media_pick_ai_manager_response(this, club.manager(), player, kind) } else { 0 });
    if other_id > -1 {
        s.put(other_id as u32);
        for name in [other.first_name(), other.second_name(), other.common_name()] {
            s.put(read_u32(name + names::ID));
        }
    } else {
        for _ in 0..4 {
            s.put(u32::MAX);
        }
    }
    if fixture != 0 {
        covered!("sx01d_5 outburst story after a match");
        s.put(1);
        s.put_match(&mut record, Fixture(fixture), club);
    } else {
        s.put(0);
    }
    let outlet = exe::media_pick_outlet_type_for_person(player);
    s.put(outlet);
    let nation = club.nation().addr();
    s.put(exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, outlet, if nation != 0 { read_i32(nation) } else { -1 }) as u32);
    if report {
        covered!("sx01d_5 assistant report");
        exe::news_send_to_club_manager(read_usize(G_NEWS), s.n, club_at(index as isize));
    } else {
        let mut clubs = [0usize; 30];
        let count = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0x1a, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
        for &c in clubs.iter().take(count.max(0) as usize) {
            exe::news_send_to_club_manager(read_usize(G_NEWS), s.n, c);
        }
        exe::media_apply_squad_reaction_to_player_story(this, kind, player, club, core::ptr::null_mut(), u32::MAX);
        log_story(this, player.addr(), club.addr(), 0xbd9, read_u8(story + 0x13) as u32, 0);
    }
    if media_is_human_club(club.addr(), 0, 1) == 0 && !report {
        exe::media_apply_player_story_response(this, player, club, kind, 0);
    }
}

/// Comparer for `create_player_outburst_story`: it draws, posts news and answers the story, so it cannot run twice;
/// only the original runs (in the comparison mode the post-match gate of 0x733a60 is installed first and keeps the
/// address, see the item).
pub(crate) unsafe extern "thiscall" fn compare_create_player_outburst_story(this: usize, story: usize, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_CREATE_PLAYER_OUTBURST_STORY)(this, story, fixture)
}

// ---------- batch 333: the player's current injury and two key nation table counts (injury.cpp, key_nation.cpp; sx12_1) ----------

/// Returns the address of a person's current injury entry, or 0 when he has none.
///
/// `Injury::get_player_injury(person)`: a null person shows the "Error" box (injury.cpp line 0xe3a) and returns 0; a
/// person id at or above `G_STAFF_COUNT` returns 0. Otherwise it runs the day's fitness update
/// (`exe::injury_update_player_fitness`, once a day) and returns the 0x30-byte entry of `injury_fields::INJURY_ENTRIES`
/// indexed by the 31-byte physical record's `physical_record::CURRENT_INJURY`, or 0 when that index is -1.
/// Original: `Injury::get_player_injury` (0x62ed70, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_player_injury(this: usize, person: usize) -> usize {
    if person == 0 {
        show_error_at(0x9e7f28, 0xe3a); // "...\injury.cpp"
        return 0;
    }
    if read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    exe::injury_update_player_fitness(this, person);
    let id = read_i32(person) as usize;
    let record = read_usize(this + injury_fields::PHYSICAL_RECORDS).wrapping_add(id.wrapping_mul(31));
    let k = read_i32(record + physical_record::CURRENT_INJURY);
    if k == -1 {
        return 0;
    }
    covered!("sx12_1 current injury");
    read_usize(this + injury_fields::INJURY_ENTRIES).wrapping_add((k as usize).wrapping_mul(0x30)) // sx12_1: the injury entry
}

/// Returns 1 when the key nation table holds a nation of the given continent chosen to be played, else 0.
///
/// `key_nation_continent_has_selected_nation(continent)`: over the 35 rows of `G_LEAGUE_TABLE` (72 bytes, nation at
/// +0), the first row whose nation is not null, has `.continent()` equal to the given one and `.league_selected()` not
/// zero gives 1. Called by the awards (0x4135b0), the discipline (0x55d420), the continental cups and the setup.
/// Original: `key_nation_continent_has_selected_nation` (0x6699d0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_continent_has_selected_nation(continent: usize) -> u32 {
    for k in 0..0x23usize {
        let n = Nation(read_usize(G_LEAGUE_TABLE + 72 * k));
        if !n.is_null() && n.continent() == continent && n.league_selected() != 0 {
            covered!("sx12_1 continent has a selected nation");
            return 1; // sx12_1: a chosen nation of the continent
        }
    }
    0
}

/// Numbers the active rows of the key nation table and stores how many there are; returns 1.
///
/// `key_nation_number_active_entries()`: clears the count (`G_KEY_NATION_ACTIVE_COUNT`, a byte); a row of
/// `G_LEAGUE_TABLE` is active when its nation (+0) is null (the world and continent rows) or chosen to be played
/// (`.league_selected()` not zero); each active row gets the running count at `key_nation_record::ACTIVE_INDEX`, and
/// the count rises. The inactive rows keep their old index. Called by the game setup (0x828110, 0x82a9c0).
/// Original: `key_nation_number_active_entries` (0x669140, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_number_active_entries() -> u32 {
    let mut count: u8 = 0;
    write_u8(G_KEY_NATION_ACTIVE_COUNT, 0);
    for k in 0..0x23usize {
        let row = KeyNationRecord(G_LEAGUE_TABLE + 72 * k);
        let n = Nation(read_usize(row.addr())); // sx12_1: the row's nation
        if n.is_null() || n.league_selected() != 0 {
            covered!("sx12_1 active key nation row");
            row.set_active_index(count);
            count = count.wrapping_add(1); // sx12_1: the next active index
            write_u8(G_KEY_NATION_ACTIVE_COUNT, count);
        }
    }
    1
}

/// Comparer for `get_player_injury`: the returned entry, side by side. The original runs first and does the day's
/// fitness update (with its draws), so the copy finds the day already done. A null person (the "Error" box): only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_get_player_injury(this: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_PLAYER_INJURY);
    if person == 0 {
        return o(this, person);
    }
    let r = o(this, person);
    let n = get_player_injury(this, person);
    SCOREBOARD.record(r == n, || format!("get_player_injury(person {:#x}) original {:#x} rust {:#x}", person, r, n));
    r
}

/// Comparer for `key_nation_continent_has_selected_nation`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_key_nation_continent_has_selected_nation(continent: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_KEY_NATION_CONTINENT_HAS_SELECTED_NATION)(continent);
    let n = key_nation_continent_has_selected_nation(continent);
    SCOREBOARD.record(r == n, || format!("key_nation_continent_has_selected_nation({:#x}) original {} rust {}", continent, r, n));
    r
}

/// Comparer for `key_nation_number_active_entries`: the 35 rows of the table and the count, side by side.
pub(crate) unsafe extern "C" fn compare_key_nation_number_active_entries() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_KEY_NATION_NUMBER_ACTIVE_ENTRIES);
    let regions = [(G_LEAGUE_TABLE, 0x23 * 72), (G_KEY_NATION_ACTIVE_COUNT, 1)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || key_nation_number_active_entries());
    SCOREBOARD.record(equal && r == n, || format!("key_nation_number_active_entries() original {} rust {}: {:?}", r, n, first_differences(&mo, &mr)));
    r
}

// ---------- batch 333: the key nation menu order and the nation menu split count (key_nation.cpp; sx12_2) ----------

/// Sorts the 35 key nation row indices into menu order and returns the static index array (0xb63d38).
///
/// `key_nation_sorted_order(use_short)`: writes 0..34 into the byte array at 0xb63d38, stores the flag in the dword at
/// 0xb63d5c (read by `key_nation_compare_entries`: compare the nations by their short name) and sorts the array with
/// the C runtime's qsort (`exe::crt_qsort`, 35 one-byte elements) and `key_nation_compare_entries` (0x66a100). Called
/// by the menu bar (0x75db00, every time it is built), the manager, search, setup and transfer screens.
/// Original: `key_nation_sorted_order` (0x66a0c0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_sorted_order(use_short: u32) -> usize {
    for k in 0..0x23u8 {
        write_u8(0xb63d38 + k as usize, k); // the static index array
    }
    write_u32(0xb63d5c, use_short); // the flag of the comparison
    covered!("sx12_2 key nation order");
    exe::crt_qsort(0xb63d38usize, 0x23, 1, 0x66a100usize); // sx12_2: key_nation_compare_entries
    0xb63d38
}

/// Orders the three world rows first by their index, then those of the continents by the continent's name, then the
/// nations by name.
///
/// `key_nation_compare_entries(a, b)` (qsort, one-byte row indices of `G_LEAGUE_TABLE`, nation at +0 and continent at
/// +4 (`G_LEAGUE_WORLD_CUP_CONTINENT`)): a null a gives 1 and a null b -1. A row with neither nation nor continent (a
/// world row) goes before any other; two of them by the index (0 before 1 before 2; other indices: equal). A row with
/// a continent goes before a row with neither: with both continents, `exe::utils_string_compare` of their names (+4); a
/// with a continent and b without: -1; a without (a nation row) and b with: 1. Two nation rows: by the nations' names
/// (+4), or their short names (`nation::SHORT_NAME`) when the flag at 0xb63d5c is set.
/// Original: `key_nation_compare_entries` (0x66a100, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_compare_entries(a: *const i8, b: *const i8) -> i32 {
    if a.is_null() {
        return 1;
    }
    if b.is_null() {
        return -1;
    }
    let (ia, ib) = (*a as isize, *b as isize);
    let row = |i: isize| (i * 72) as usize;
    let (na, ca) = (read_usize(G_LEAGUE_TABLE.wrapping_add(row(ia))), read_usize(G_LEAGUE_WORLD_CUP_CONTINENT.wrapping_add(row(ia))));
    let (nb, cb) = (read_usize(G_LEAGUE_TABLE.wrapping_add(row(ib))), read_usize(G_LEAGUE_WORLD_CUP_CONTINENT.wrapping_add(row(ib))));
    if na == 0 && ca == 0 {
        if nb != 0 || cb != 0 {
            return -1;
        }
        covered!("sx12_2 world rows");
        // the world rows by their index (only 0, 1 and 2 are ordered)
        return if (0..=2).contains(&ia) && (0..=2).contains(&ib) { (ia - ib).signum() as i32 } else { 0 };
    }
    if nb == 0 && cb == 0 {
        return 1;
    }
    if ca != 0 {
        if cb == 0 {
            return -1;
        }
        covered!("sx12_2 continent rows");
        return exe::utils_string_compare((ca + 4) as *const u8, (cb + 4) as *const u8, 1, 0); // the continents' names
    }
    if cb != 0 {
        return 1;
    }
    if na == 0 || nb == 0 {
        return 0;
    }
    covered!("sx12_2 nation rows");
    if read_u32(0xb63d5c) != 0 {
        exe::utils_string_compare((na + nation::SHORT_NAME) as *const u8, (nb + nation::SHORT_NAME) as *const u8, 1, 0)
    } else {
        exe::utils_string_compare((na + 4) as *const u8, (nb + 4) as *const u8, 1, 0) // sx12_2: the nations' names
    }
}

/// Returns 2 when a nation menu holds more than 16 nations and is shown in two parts, else 1.
///
/// `NATION_MENU_SPLIT::part_count()`: the signed byte count at +9 above 16 gives 2. Only al is the return value (the
/// original increments the whole eax, whose high bytes are left over).
/// Original: `NATION_MENU_SPLIT::part_count` (0x66a350, key_nation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_part_count(this: usize) -> u8 {
    covered!("sx12_2 menu parts");
    (read_i8(this + 9) > 16) as u8 + 1 // sx12_2: more than 16 nations
}

/// Comparer for `key_nation_sorted_order`: the index array and the flag (0xb63d38, 0x28 bytes) and the return, side by
/// side.
pub(crate) unsafe extern "C" fn compare_key_nation_sorted_order(use_short: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_KEY_NATION_SORTED_ORDER);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(0xb63d38, 0x28)], || o(use_short), || key_nation_sorted_order(use_short));
    SCOREBOARD.record(equal && r == n, || format!("key_nation_sorted_order({}) original {:#x} rust {:#x}: {:?}", use_short, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `key_nation_compare_entries`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_key_nation_compare_entries(a: *const i8, b: *const i8) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const i8, *const i8) -> i32>(IDX_KEY_NATION_COMPARE_ENTRIES)(a, b);
    let n = key_nation_compare_entries(a, b);
    SCOREBOARD.record(r == n, || format!("key_nation_compare_entries({:?}, {:?}) original {} rust {}",
        if a.is_null() { None } else { Some(*a) }, if b.is_null() { None } else { Some(*b) }, r, n));
    r
}

/// Comparer for `nation_menu_split_part_count`: al, side by side.
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_part_count(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_NATION_MENU_SPLIT_PART_COUNT)(this);
    let n = nation_menu_split_part_count(this);
    SCOREBOARD.record(r == n, || format!("NATION_MENU_SPLIT::part_count(count {}) original {} rust {}", read_i8(this + 9), r, n));
    r
}
