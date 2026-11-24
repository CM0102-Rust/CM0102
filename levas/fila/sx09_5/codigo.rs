// ---------- batch %%LEVA%%: Date.cpp: the date texts with the weekday (short and long) and the month name (sx09_5) ----------

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
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I0%%)(this, out, size, time);
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
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I1%%)(this, out, size, time);
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
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I2%%)(this, out, size, time);
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
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I0%%);
    compare_date_text("date_get_short_weekday_date_string", this, out, size, 0x12, time, || o(this, out, size, time), || date_get_short_weekday_date_string(this, out, size, time))
}

/// Comparer for `date_get_long_date_string`.
pub(crate) unsafe extern "thiscall" fn compare_date_get_long_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I1%%);
    compare_date_text("date_get_long_date_string", this, out, size, 0x32, time, || o(this, out, size, time), || date_get_long_date_string(this, out, size, time))
}

/// Comparer for `date_get_long_date_string_no_year`.
pub(crate) unsafe extern "thiscall" fn compare_date_get_long_date_string_no_year(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I2%%);
    compare_date_text("date_get_long_date_string_no_year", this, out, size, 0x32, time, || o(this, out, size, time), || date_get_long_date_string_no_year(this, out, size, time))
}
