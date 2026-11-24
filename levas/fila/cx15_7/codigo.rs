// ---------- cx15_7: the text of the European registration news (squad_manager.cpp) ----------

/// The slots of the European registration news 0x19 (`send_euro_registration_news`) that its text reads.
struct EuroRegistrationNews {
    club: Club,
    comp: Competition,
    step: i8,
    /// Slot 3, a dword: whether the registration was made (the "registered" lines follow).
    made: u32,
    places: i8,
    registered: i8,
    /// Slot 6: how many players were registered (−1 or below: none counted).
    done: i8,
}

/// Reads the slots of the registration news in the original's order (0 to 6): the club by index (slot 0) and the
/// competition record by id (slot 1, 0x6b-byte records from [`G_CLUB_COMP_TABLE`]).
unsafe fn read_euro_registration_news(news: usize) -> EuroRegistrationNews {
    let club = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    let comp = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((exe::newsitem_get_slot(news, 1) as usize).wrapping_mul(0x6b)));
    let step = exe::newsitem_get_slot(news, 2) as u8 as i8;
    let made = exe::newsitem_get_slot(news, 3);
    let places = exe::newsitem_get_slot(news, 4) as u8 as i8;
    let registered = exe::newsitem_get_slot(news, 5) as u8 as i8;
    let done = exe::newsitem_get_slot(news, 6) as u8 as i8;
    EuroRegistrationNews { club, comp, step, made, places, registered, done }
}

/// The body line of the second step (the first group stage of the Champions League, `KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE`,
/// or the first three rounds of the other cups) and its number: one more player (no number), or with players already
/// registered the difference of the places and the registered, otherwise the places.
unsafe fn group_stage_line(r: &EuroRegistrationNews) -> (usize, Option<i32>) {
    let champions_league = read_u32(r.comp.addr()) == read_u32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE);
    let left = r.places as i32 - r.registered as i32;
    // (one more, n more, n players): the Champions League's group stage, the other cups' first three rounds
    let formats = if champions_league { (0xa8340c, 0xa83360, 0xa832b8) } else { (0xa8321c, 0xa8316c, 0xa830c4) };
    if left == 1 {
        (formats.0, None)
    } else if r.registered != 0 {
        (formats.1, Some(left))
    } else {
        (formats.2, Some(r.places as i32))
    }
}

/// The closing line of the second step, when the registration was made (slot 3): none (0xa8309c), one more (0xa83074),
/// n more (0xa8303c) when players were already registered, n (0xa8300c) otherwise; a negative count gives an empty
/// line (the empty string 0xad9c64 copied into the buffer).
unsafe fn group_stage_done_line(r: &EuroRegistrationNews, lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32) {
    match r.done {
        0 => {
            lang_sprintf(0xde1f64, 0xa8309c);
        }
        1 => {
            lang_sprintf(0xde1f64, 0xa83074);
        }
        n if n < 0 => {
            covered!("cx15_7 registration count negative");
            write_u8(0xde1f64, 0); // strcpy of the empty string 0xad9c64
        }
        n => {
            lang_sprintf(0xde1f64, if r.registered != 0 { 0xa8303c } else { 0xa8300c }, n as i32);
        }
    }
}

/// Writes the text of the European registration news (type 0x19) into Text `text`. `this` is not read.
///
/// The headline (`body` 0), by the step (slot 2): "<club> to register players for" the qualifying phase (1, 0xa82d3c),
/// the next phase (2, 0xa82cf4) or the later stages (3, 0xa82cac), with the club link (0x7d5); any other step leaves
/// the text untouched. The body, with the club (short name and gender, link 0x7d5) and the competition (short name and
/// gender, link 0x7d0):
/// - step 1: the request to register at most the places (slot 4) for the qualifying phase (0xa834dc); when the
///   registration was made (slot 3) and the count (slot 6) is not negative, "<n> players were registered." (0xa834a8)
///   appended;
/// - step 2: `group_stage_line` and, when the registration was made, `group_stage_done_line` appended;
/// - step 3: by 25 − registered: from 2, two additional and two replacement players (0xa82dc4); 0, two additional
///   players (0xa82f34); 1, one additional and two replacement players (0xa82e7c); a negative difference falls to the
///   first; when the registration was made, "The registration list for this season has been finalised." (0xa82d88);
/// - any other step: the text untouched.
/// Each line goes to the global text buffer 0xde1f64 (`FN_LANGLIB_SPRINTF`, cdecl, the club's gender twice and the
/// competition's twice first), then `exe::text_set_string` or `exe::text_append_string`.
///
/// Called by the news text dispatcher (`NewsItem::write_text`, type 0x19).
/// Original: `SQUAD_MANAGER::write_euro_registration_news_text` (0x85b190, squad_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_euro_registration_news_text(_this: usize, news: usize, body: u32, text: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if body == 0 {
        let c = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
        let format = match exe::newsitem_get_slot(news, 2) as u8 {
            1 => 0xa82d3c,
            2 => 0xa82cf4,
            3 => 0xa82cac,
            _ => return,
        };
        let g = c.short_gender_name() as i8 as i32;
        let name = c.addr() + club::SHORT_NAME;
        lang_sprintf(0xde1f64, format, g, g, name);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, name as *const u8, 0x7d5, c);
        return;
    }
    let r = read_euro_registration_news(news);
    let (cg, kg) = (r.club.short_gender_name() as i8 as i32, r.comp.short_gender_name() as i8 as i32);
    let (club_name, comp_name) = (r.club.addr() + club::SHORT_NAME, r.comp.short_name_addr());
    match r.step {
        1 => {
            covered!("cx15_7 qualifying phase request");
            lang_sprintf(0xde1f64, 0xa834dc, cg, cg, kg, kg, club_name, r.places as i32, comp_name);
        }
        2 => match group_stage_line(&r) {
            (format, None) => {
                lang_sprintf(0xde1f64, format, cg, cg, kg, kg, club_name, comp_name);
            }
            (format, Some(n)) => {
                lang_sprintf(0xde1f64, format, cg, cg, kg, kg, club_name, n, comp_name);
            }
        },
        3 => {
            let left = 25 - r.registered as i32;
            let format = if left >= 2 {
                0xa82dc4
            } else if left as u8 == 0 {
                0xa82f34
            } else if left as u8 == 1 {
                0xa82e7c
            } else {
                0xa82dc4
            };
            lang_sprintf(0xde1f64, format, cg, cg, kg, kg, club_name, comp_name);
        }
        _ => return,
    }
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, club_name as *const u8, 0x7d5, r.club);
    exe::text_add_link(text, comp_name as *const u8, 0x7d0, r.comp);
    if r.made == 0 {
        return;
    }
    match r.step {
        1 => {
            if r.done < 0 {
                return;
            }
            lang_sprintf(0xde1f64, 0xa834a8, r.done as i32);
        }
        2 => group_stage_done_line(&r, lang_sprintf),
        _ => {
            lang_sprintf(0xde1f64, 0xa82d88);
        }
    }
    exe::text_append_string(text, 0xde1f64);
}

/// Comparer for `write_euro_registration_news_text`: side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_euro_registration_news_text(this: usize, news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, news, body, text), |t| write_euro_registration_news_text(this, news, body, t),
        || format!("write_euro_registration_news_text(news {:#x}, body {})", news, body));
}
