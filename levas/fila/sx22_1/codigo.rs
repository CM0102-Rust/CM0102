// ---------- batch %%LEVA%%: the news of a postponed or abandoned match (weather.cpp) ----------

/// Posts the news item of kind `kind` about a league match the weather (or the floodlights) stopped: nothing for a null
/// fixture or before the news date (`G_GAME_START_DATE`: the year unsigned, then the day signed). The local news item
/// holds the home club's id (slot 0), the away club's (1), the competition's (2) and the reason bits (3, the signed
/// byte), and goes to the managers of the home club and then of the away club.
pub(crate) unsafe fn post_weather_news(kind: u32, fixture: usize, reason: u32) {
    let game = Fixture(fixture);
    if game.is_null() {
        return;
    }
    let day = read_i16(G_GAME_START_DATE);
    let year = read_u16(G_GAME_START_DATE_YEAR);
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    if year > today_year || (year == today_year && day > read_i16(G_CURRENT_DATE)) {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, kind, 0);
    exe::newsitem_set_slot(n, 0, read_u32(game.home_club().addr())); // the record's id (+0)
    exe::newsitem_set_slot(n, 1, read_u32(game.away_club().addr()));
    exe::newsitem_set_slot(n, 2, read_u32(game.competition().addr()));
    exe::newsitem_set_slot(n, 3, reason as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, game.home_club());
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, game.away_club()); // sx22_1: both managers
}

/// Posts the news that a league match was postponed (kind 2), for the managers of both clubs, from the match day's
/// set-up (`set_up_the_day`) when `Weather::check_postponement` stops the match: `post_weather_news`.
/// Original: `Weather::post_postponed_news` (0x923cc0, weather.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_postponed_news(_this: usize, fixture: usize, reason: u32) {
    covered!("sx22_1 postponed news");
    post_weather_news(2, fixture, reason);
}

/// Posts the news that a league match was abandoned (kind 0x2a), for the managers of both clubs, from the end of the
/// match day (`close_day_match`): `post_weather_news`.
/// Original: `Weather::post_abandoned_news` (0x924050, weather.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_abandoned_news(_this: usize, fixture: usize, reason: u32) {
    covered!("sx22_1 abandoned news");
    post_weather_news(0x2a, fixture, reason);
}

/// The clubs and the competition of a weather news item from its slots 0 (home club id), 1 (away) and 2 (competition).
unsafe fn weather_news_parties(home: u32, away: u32, comp: u32) -> (Club, Club, usize) {
    let competition = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp as usize).wrapping_mul(0x6b)); // 107-byte records
    (club_at(home as i32 as isize), club_at(away as i32 as isize), competition)
}

/// A gender byte of a name as the language table's `{}` reads it (sign-extended).
unsafe fn weather_news_gender(at: usize) -> u32 {
    read_i8(at) as i32 as u32
}

/// The headline of a weather news item ("<home> vs <away> <league> match postponed/abandoned", `format`): the clubs'
/// short names and the competition's name, each with its gender byte, into the shared buffer and then the text. The
/// headline links nothing.
pub(crate) unsafe fn weather_news_headline(news: usize, out: usize, format: usize) {
    let s: [u32; 3] = awol_news_slots(news);
    let (home, away, comp) = weather_news_parties(s[0], s[1], s[2]);
    let (h, a) = (home.addr(), away.addr());
    let (hg, ag, cg) = (weather_news_gender(h + club::SHORT_GENDER_NAME), weather_news_gender(a + club::SHORT_GENDER_NAME),
        weather_news_gender(comp + competition::GENDER_NAME));
    news_text_format()(NEWS_TEXT_BUFFER, format, hg, hg, ag, ag, cg, cg, h + club::SHORT_NAME, a + club::SHORT_NAME, comp + competition::NAME);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
}

/// The body of a weather news item: with a `format` ("The <league> match between <home> and <away> has been ..."), the
/// competition's and the clubs' long names with their gender bytes into the shared buffer; the text takes the buffer
/// (without a format, whatever it last held) and links the home club and the away club (long names).
pub(crate) unsafe fn weather_news_body(s: [u32; 4], out: usize, format: Option<usize>) {
    let (home, away, comp) = weather_news_parties(s[0], s[1], s[2]);
    let (h, a) = (home.addr(), away.addr());
    if let Some(f) = format {
        let (cg, hg, ag) = (weather_news_gender(comp + competition::GENDER_NAME), weather_news_gender(h + club::GENDER_NAME),
            weather_news_gender(a + club::GENDER_NAME));
        news_text_format()(NEWS_TEXT_BUFFER, f, cg, cg, hg, hg, ag, ag, comp + competition::NAME, h + club::NAME, a + club::NAME);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, (h + club::NAME) as *const u8, 0x7d5, home);
    exe::text_add_link(out, (a + club::NAME) as *const u8, 0x7d5, away);
}

/// Writes the headline or the body of the postponed match news (slots: 0 home club, 1 away club, 2 competition, 3 the
/// reason bits).
///
/// The headline: "<home> vs <away> <league> match postponed". The body by the reason, in this order: 4 "has been
/// postponed due to bad weather", 8 "due to a water-logged pitch", 0x10 "due to flood-light failure"; then the links of
/// both clubs.
/// Defect of the original: with none of the three bits the body formats nothing, so the text is whatever the shared
/// buffer 0xde1f64 last held.
/// Original: `weather_write_postponed_news_text` (0x923dd0, weather.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_postponed_news_text(_this: usize, news: usize, full: u32, out: usize) {
    if full == 0 {
        weather_news_headline(news, out, 0xad5220); // sx22_1: "<home> vs <away> <league> match postponed"
        return;
    }
    let s: [u32; 4] = awol_news_slots(news);
    let format = if s[3] & 4 != 0 {
        Some(0xad5164) // bad weather
    } else if s[3] & 8 != 0 {
        Some(0xad50a0) // water-logged pitch
    } else if s[3] & 0x10 != 0 {
        Some(0xad4fdc) // flood-light failure
    } else {
        covered!("sx22_1 postponed body without a reason");
        None
    };
    weather_news_body(s, out, format);
}

/// Comparer for `post_postponed_news`: only the original (it posts the news).
pub(crate) unsafe extern "thiscall" fn compare_post_postponed_news(this: usize, fixture: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, fixture, reason)
}

/// Comparer for `write_postponed_news_text`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_postponed_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_postponed_news_text(this, news, full, t),
        || format!("write_postponed_news_text(news {:#x}, full {})", news, full));
}

/// Comparer for `post_abandoned_news`: only the original (it posts the news).
pub(crate) unsafe extern "thiscall" fn compare_post_abandoned_news(this: usize, fixture: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, fixture, reason)
}
