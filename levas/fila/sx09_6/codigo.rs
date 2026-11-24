// ---------- batch %%LEVA%%: Date.cpp: the date text without the weekday (sx09_6) ----------

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
        return original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I0%%)(this, out, size, time);
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
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, usize, u32, u32) -> u32>(%%I0%%);
    compare_date_text("date_get_date_string_no_weekday", this, out, size, 0x32, time, || o(this, out, size, time), || date_get_date_string_no_weekday(this, out, size, time))
}
