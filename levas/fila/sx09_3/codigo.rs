// ---------- batch %%LEVA%%: Date.cpp: the weekday and month names (sx09_3) ----------

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
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(%%I0%%);
    compare_date_name("date_get_weekday_name", (WEEKDAY_NAME_BUFFER, 0x1c), weekday, || o(weekday), || date_get_weekday_name(weekday))
}

/// Comparer for `date_get_short_month_name`.
pub(crate) unsafe extern "C" fn compare_date_get_short_month_name(month: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(%%I1%%);
    compare_date_name("date_get_short_month_name", (SHORT_MONTH_NAME_BUFFER, 0xa), month, || o(month), || date_get_short_month_name(month))
}

/// Comparer for `date_get_month_name`.
pub(crate) unsafe extern "C" fn compare_date_get_month_name(month: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(%%I2%%);
    compare_date_name("date_get_month_name", (MONTH_NAME_BUFFER, 0x19), month, || o(month), || date_get_month_name(month))
}
