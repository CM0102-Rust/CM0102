// ---------- batch %%LEVA%%: Date.cpp: the time of day name and the grammar of the date words (sx09_4) ----------

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
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> usize>(%%I0%%);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&[(TIME_OF_DAY_BUFFER, 0x19)], || o(time, capital), || date_get_time_of_day_name(time, capital));
    SCOREBOARD.record(equal && ro == rr, || format!("date_get_time_of_day_name({}, {}) original {:x?} rust {:x?}", time as u8, capital, mo, mr));
    ro
}

/// Comparer for `date_get_time_of_day_gender`: the low byte, side by side.
pub(crate) unsafe extern "C" fn compare_date_get_time_of_day_gender(time: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I1%%)(time);
    let n = date_get_time_of_day_gender(time);
    SCOREBOARD.record(r as u8 == n as u8, || format!("date_get_time_of_day_gender({}) original {} rust {}", time as u8, r as u8, n as u8));
    r
}

/// Comparer for `date_get_weekday_gender`: the low byte, side by side.
pub(crate) unsafe extern "C" fn compare_date_get_weekday_gender(language: u32, weekday: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(%%I2%%)(language, weekday);
    let n = date_get_weekday_gender(language, weekday);
    SCOREBOARD.record(r as u8 == n as u8, || format!("date_get_weekday_gender({}, {}) original {} rust {}", language, weekday as i16, r as u8, n as u8));
    r
}
