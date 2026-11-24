// ---------- batch %%LEVA%%: the weather and pitch texts of a match (weather.cpp) ----------

/// Writes a match's weather as text into `out`, from the weather record (+0 the wind, +1 the precipitation, +2 the
/// temperature, signed bytes).
///
/// The wind: 0 none, 1 "Breezy", 2 "Gusty", 3 "Strong Wind", 4 "Gale". The precipitation: 0 "Dry" when there is no
/// wind word (else none), 1 "Wet", 2 "Drizzle", 3 "Shower", 5 "Downpour", 6 "Sleet", 7 "Hail", 8 "Light Snow", 9
/// "Heavy Snow", any other (4 included) "Invalid WeatherSeasonPrecipitation: %d". The temperature through its language
/// line. The words through the language table; then "<wind>, <precipitation>, <temperature>", leaving out an empty
/// wind or (with a wind) an empty precipitation.
/// Defect of the original: an invalid wind (above 4 or negative) writes "Invalid Wind: %d" into the temperature buffer,
/// which the temperature then overwrites, and leaves the wind text as the stack held it (here empty).
/// Original: `weather_format_description` (0x9233a0, weather.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn weather_format_description(_this: usize, out: usize, weather: usize) {
    let fmt = news_text_format();
    let crt = background_sprintf();
    let mut rain = [0u8; 0x1f4];
    let mut wind = [0u8; 0x1f4];
    let mut temperature = [0u8; 0x1f4];
    let (rp, wp, tp) = (rain.as_mut_ptr() as usize, wind.as_mut_ptr() as usize, temperature.as_mut_ptr() as usize);
    let wind_word = match read_i8(weather) { // +0: the wind
        1 => 0xad4f54, // "Breezy"
        2 => 0xad4f4c, // "Gusty"
        3 => 0xad4f40, // "Strong Wind"
        4 => 0xad4f38, // "Gale"
        _ => 0,        // 0: none (the empty string copied); invalid: see the defect
    };
    if wind_word != 0 {
        fmt(wp, wind_word);
    }
    let rain_kind = read_i8(weather + 1) as i32; // +1: the precipitation
    let rain_word = match rain_kind {
        0 if wind[0] == 0 => 0xad4f20, // "Dry"
        0 => 0,
        1 => 0xad4f1c, // "Wet"
        2 => 0xad4f14, // "Drizzle"
        3 => 0xad4f00, // "Shower"
        5 => 0xad4f08, // "Downpour"
        6 => 0xad4ee0, // "Sleet"
        7 => 0xad4ed8, // "Hail"
        8 => 0xad4ef4, // "Light Snow"
        9 => 0xad4ee8, // "Heavy Snow"
        _ => {
            crt(rp, 0xad4eb0, rain_kind); // "Invalid WeatherSeasonPrecipitation: %d"
            0
        }
    };
    if rain_word != 0 {
        fmt(rp, rain_word);
    }
    fmt(tp, 0xad4e94, read_i8(weather + 2) as i32); // +2: the temperature
    if wind[0] == 0 {
        crt(out, 0x98f6ac, rp, tp); // "%s, %s"
    } else if rain[0] == 0 {
        crt(out, 0x98f6ac, wp, tp);
    } else {
        covered!("sx22_3 wind and rain");
        crt(out, 0xad4e88, wp, rp, tp); // sx22_3: "%s, %s, %s"
    }
}

/// Writes the text of a pitch condition flag into `out`, through the language table: 1 "Muddy", 2 "Damp", 4 "Dry", 8
/// "Water-logged", 0x10 "Frozen", 0x20 "Snowy", 0x40 "Hard"; 0 and any other value "Normal" (the two commented lines
/// take the empty string).
/// Original: `pitch_condition_text` (0x923b60, weather.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn pitch_condition_text(_this: usize, out: usize, condition: u32) {
    let fmt = news_text_format();
    match condition as i8 {
        1 => fmt(out, 0xad4fac), // "Muddy"
        2 => fmt(out, 0xad4fa4), // "Damp"
        4 => fmt(out, 0xad4f20), // "Dry"
        8 => fmt(out, 0xad4f94), // "Water-logged"
        0x10 => fmt(out, 0xad4f8c), // "Frozen"
        0x20 => fmt(out, 0xad4f84), // "Snowy" (sx22_3)
        0x40 => fmt(out, 0xad4f5c, 0xad9c64), // "Hard<%s - COMMENT - pitch condition>"
        _ => fmt(out, 0xad4fb4, 0xad9c64),    // "Normal<%s - COMMENT - pitch condition>"
    };
}

/// Comparer for `weather_format_description`: the original into `out`, the copy into a local buffer, the texts equal.
/// An invalid wind is not compared (the original's wind text is stack garbage).
pub(crate) unsafe extern "thiscall" fn compare_weather_format_description(this: usize, out: usize, weather: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, out, weather);
    if read_u8(weather) > 4 {
        return;
    }
    let mut mine = [0u8; 0x600];
    weather_format_description(this, mine.as_mut_ptr() as usize, weather);
    let (o, r) = (c_string_bytes(out), c_string_bytes(mine.as_ptr() as usize));
    SCOREBOARD.record(o == r, || format!("weather_format_description(weather {:02x?}) original {:?} rust {:?}",
        core::slice::from_raw_parts(weather as *const u8, 3), String::from_utf8_lossy(&o), String::from_utf8_lossy(&r)));
}

/// Comparer for `pitch_condition_text`: the original into `out`, the copy into a local buffer, the texts equal.
pub(crate) unsafe extern "thiscall" fn compare_pitch_condition_text(this: usize, out: usize, condition: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%)(this, out, condition);
    let mut mine = [0u8; 0x400];
    pitch_condition_text(this, mine.as_mut_ptr() as usize, condition);
    let (o, r) = (c_string_bytes(out), c_string_bytes(mine.as_ptr() as usize));
    SCOREBOARD.record(o == r, || format!("pitch_condition_text({:#x}) original {:?} rust {:?}", condition as i8, String::from_utf8_lossy(&o),
        String::from_utf8_lossy(&r)));
}
