// ---------- batch %%LEVA%%: the abandoned match news text and freeing the weather tables (weather.cpp) ----------

/// Writes the headline or the body of the abandoned match news (slots: 0 home club, 1 away club, 2 competition, 3 the
/// reason bits).
///
/// The headline: "<home> vs <away> <league> match abandoned". The body by the reason, in this order: 0x10 "has been
/// abandoned due to floodlight failure during the match", 4 "was abandoned due to excessive snowfall", 8 "due to a
/// water-logged condition of the pitch", otherwise "was abandoned and will be replayed at a later date"; then the links
/// of both clubs (`weather_news_body`).
/// Original: `weather_write_abandoned_news_text` (0x924160, weather.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_abandoned_news_text(_this: usize, news: usize, full: u32, out: usize) {
    if full == 0 {
        weather_news_headline(news, out, 0xad55f4); // "<home> vs <away> <league> match abandoned"
        return;
    }
    covered!("sx22_2 abandoned body");
    let s: [u32; 4] = awol_news_slots(news);
    let format = if s[3] & 0x10 != 0 {
        0xad5520 // floodlight failure
    } else if s[3] & 4 != 0 {
        0xad544c // snowfall
    } else if s[3] & 8 != 0 {
        0xad5378 // water-logged pitch
    } else {
        0xad52b0 // replayed at a later date
    };
    weather_news_body(s, out, Some(format)); // sx22_2: the body by the reason
}

/// Frees the weather's two tables (+0 and +4, each with the C runtime's `free` when not null) and clears both pointers.
/// Original: `Weather::~Weather` (0x9227d0, weather.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn weather_free_tables(this: usize) {
    for table in [this, this + 4] {
        let p = read_usize(table);
        if p != 0 {
            exe::crt_free(p);
            write_usize(table, 0);
        }
    }
}

/// Comparer for `write_abandoned_news_text`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_abandoned_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_abandoned_news_text(this, news, full, t),
        || format!("write_abandoned_news_text(news {:#x}, full {})", news, full));
}

/// Comparer for `weather_free_tables`: only the original (it frees); both pointers end null.
pub(crate) unsafe extern "thiscall" fn compare_weather_free_tables(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    SCOREBOARD.record(read_usize(this) == 0 && read_usize(this + 4) == 0,
        || format!("weather_free_tables({:#x}): tables {:#x} {:#x}", this, read_usize(this), read_usize(this + 4)));
}
