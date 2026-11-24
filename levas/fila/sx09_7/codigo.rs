// ---------- batch %%LEVA%%: Date.cpp: the numeric and short date texts of the screens (sx09_7) ----------

/// The language whose short dates are year first ("%02d-%02d-%02d", `exe::langlib_get_language`).
const LANGUAGE_YEAR_FIRST: i32 = 10;

/// The two-digit year of a date (the year modulo 100).
unsafe fn date_two_digit_year(d: *const GameDate) -> u32 {
    (*d).year as u32 % 100
}

/// Writes the date as a two-digit day, a two-digit month and a four-digit year.
///
/// `Date::get_numeric_date_string(out, size, time)`: a null `out` (Date.cpp, line 0xf8) or a size below 9 (line 0xff)
/// gives the error box and 0 (`date_text_buffer_ok`). With a time of day (0..2), "AM", "PM" or "EVE" first
/// (`write_time_suffix`); then, langlib in `out`, "<10>.<02>.<1972>" (the day of the month, the month from 1 and the
/// year) or, with the time, the same followed by " <AM>". Returns 1.
/// Original: `Date::get_numeric_date_string` (0x54a470, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_numeric_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 9, 0xf8, 0xff) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I0%%)(this, out, size, time);
    };
    let (day, month, year) = (day as i32, month as u32 + 1, (*this).year as u32);
    if date_text_has_time(time) {
        write_time_suffix(time);
        story_lang!(out, 0x9b84a0, day, month, year, TIME_SUFFIX_BUFFER);
    } else {
        story_lang!(out, 0x9b8560, day, month, year); // sx09_7: day, month, year
    }
    1
}

/// Writes the date as the day of the month, the month and a two-digit year.
///
/// `Date::get_short_date_string(out, size, time)`: a null `out` (line 0x132) or a size below 9 (line 0x139) gives the
/// error box and 0. The language is read first (`exe::langlib_get_language`); with a time of day (0..2), "AM", "PM"
/// or "EVE" (`write_time_suffix`). In language 10, the C runtime's sprintf in `out`: "%02d-%02d-%02d" (the two-digit
/// year, the month from 1, the day), with " %s" and the suffix for the time; in the others, langlib:
/// "<10>.<2>.<72>" (day, month, two-digit year), with " <AM>" for the time. Returns 1.
/// Original: `Date::get_short_date_string` (0x54a650, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_short_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 9, 0x132, 0x139) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I1%%)(this, out, size, time);
    };
    let (day, month, year) = (day as i32, month as u32 + 1, date_two_digit_year(this));
    let year_first = exe::langlib_get_language() == LANGUAGE_YEAR_FIRST;
    match (date_text_has_time(time), year_first) {
        (true, true) => {
            write_time_suffix(time);
            story_crt!(out, 0x9b8610, year, month, day, TIME_SUFFIX_BUFFER);
        }
        (true, false) => {
            write_time_suffix(time);
            story_lang!(out, 0x9b85ac, day, month, year, TIME_SUFFIX_BUFFER);
        }
        (false, true) => story_crt!(out, 0x9b866c, year, month, day),
        (false, false) => story_lang!(out, 0x9b8624, day, month, year), // sx09_7: the short date
    }
    1
}

/// Writes the weekday name, a line break and the numeric date with a two-digit year.
///
/// `Date::get_weekday_numeric_date_string(out, size, time)`: a null `out` (line 0x181) or a size below 0x12 (line
/// 0x188) gives the error box and 0. With a time of day (0..2), "AM", "PM" or "EVE" first (`write_time_suffix`). Then
/// the weekday name (`exe::date_get_weekday_name` of `crate::date::weekday`) and, langlib in `out`,
/// "<Saturday>{}\n<10>.<2>.<72>" (the `{}` takes `DATE_TEXT_GRAMMAR`), with " <AM>" for the time. Returns 1.
/// Original: `Date::get_weekday_numeric_date_string` (0x54a950, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_weekday_numeric_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 0x12, 0x181, 0x188) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I2%%)(this, out, size, time);
    };
    let (day, month, year) = (day as i32, month as u32 + 1, date_two_digit_year(this));
    if date_text_has_time(time) {
        write_time_suffix(time);
        let weekday = exe::date_get_weekday_name(crate::date::weekday(this) as u32);
        story_lang!(out, 0x9b867c, DATE_TEXT_GRAMMAR, weekday, day, month, year, TIME_SUFFIX_BUFFER);
    } else {
        covered!("sx09_7 weekday numeric date");
        let weekday = exe::date_get_weekday_name(crate::date::weekday(this) as u32);
        story_lang!(out, 0x9b86f8, DATE_TEXT_GRAMMAR, weekday, day, month, year); // sx09_7: the weekday and the date
    }
    1
}

/// Writes the date as the ordinal day, the short month and the year.
///
/// `Date::get_short_month_date_string(out, size, time)`: a null `out` (line 0x1fb) or a size below 0x12 (line 0x202)
/// gives the error box and 0. With a time of day (0..2), "AM", "PM" or "EVE" first (`write_time_suffix`). Then the
/// short month name, the ordinal day (`date_ordinal`) and, langlib in `out`, "<10th> <Feb> <1972>", with " <AM>" for
/// the time. Returns 1.
/// Original: `Date::get_short_month_date_string` (0x54b2f0, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_short_month_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 0x12, 0x1fb, 0x202) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I3%%)(this, out, size, time);
    };
    let year = (*this).year as u32;
    if date_text_has_time(time) {
        write_time_suffix(time);
        let name = date_short_month_name(month);
        story_lang!(out, 0x9b8898, date_ordinal(day), name, year, TIME_SUFFIX_BUFFER);
    } else {
        let name = date_short_month_name(month);
        story_lang!(out, 0x9b8900, date_ordinal(day), name, year); // sx09_7: the short month date
    }
    1
}

/// Writes the date as the short weekday, the ordinal day and the short month, without the year.
///
/// `Date::get_short_weekday_date_string_no_year(out, size, time)`: a null `out` (line 0x234) or a size below 0x12
/// (line 0x23b) gives the error box and 0. With a time of day (0..2), "AM", "PM" or "EVE" first (`write_time_suffix`).
/// Then the short weekday (`write_short_weekday`), the short month name, the ordinal day and, langlib in `out`,
/// "<Sat> <10th> <Feb>", with " <AM>" for the time. Returns 1.
/// Original: `Date::get_short_weekday_date_string_no_year` (0x54b550, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_short_weekday_date_string_no_year(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    if !date_text_buffer_ok(out, size, 0x12, 0x234, 0x23b) {
        return 0;
    }
    let Some((month, day)) = date_text_parts(&*this) else {
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I4%%)(this, out, size, time);
    };
    if date_text_has_time(time) {
        write_time_suffix(time);
        write_short_weekday(this);
        let name = date_short_month_name(month);
        story_lang!(out, 0x9b894c, SHORT_WEEKDAY_BUFFER, date_ordinal(day), name, TIME_SUFFIX_BUFFER);
    } else {
        covered!("sx09_7 short weekday date without the year");
        write_short_weekday(this);
        let name = date_short_month_name(month);
        story_lang!(out, 0x9b89b0, SHORT_WEEKDAY_BUFFER, date_ordinal(day), name); // sx09_7: weekday, day and month
    }
    1
}

/// Comparer for `date_get_numeric_date_string` (`compare_date_text`).
pub(crate) unsafe extern "thiscall" fn compare_date_get_numeric_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I0%%);
    compare_date_text("date_get_numeric_date_string", this, out, size, 9, time, || o(this, out, size, time), || date_get_numeric_date_string(this, out, size, time))
}

/// Comparer for `date_get_short_date_string` (`compare_date_text`).
pub(crate) unsafe extern "thiscall" fn compare_date_get_short_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I1%%);
    compare_date_text("date_get_short_date_string", this, out, size, 9, time, || o(this, out, size, time), || date_get_short_date_string(this, out, size, time))
}

/// Comparer for `date_get_weekday_numeric_date_string` (`compare_date_text`).
pub(crate) unsafe extern "thiscall" fn compare_date_get_weekday_numeric_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I2%%);
    compare_date_text("date_get_weekday_numeric_date_string", this, out, size, 0x12, time, || o(this, out, size, time), || date_get_weekday_numeric_date_string(this, out, size, time))
}

/// Comparer for `date_get_short_month_date_string` (`compare_date_text`).
pub(crate) unsafe extern "thiscall" fn compare_date_get_short_month_date_string(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I3%%);
    compare_date_text("date_get_short_month_date_string", this, out, size, 0x12, time, || o(this, out, size, time), || date_get_short_month_date_string(this, out, size, time))
}

/// Comparer for `date_get_short_weekday_date_string_no_year` (`compare_date_text`).
pub(crate) unsafe extern "thiscall" fn compare_date_get_short_weekday_date_string_no_year(this: *const GameDate, out: usize, size: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I4%%);
    compare_date_text("date_get_short_weekday_date_string_no_year", this, out, size, 0x12, time, || o(this, out, size, time), || date_get_short_weekday_date_string_no_year(this, out, size, time))
}
