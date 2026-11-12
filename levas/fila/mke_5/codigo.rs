// ---------- batch %%LEVA%%: the news texts of contract talks, contract expiry and contract end (transfer_manager.cpp, contract_manager.cpp) ----------

/// The shared buffer the news texts are written into (`FN_LANGLIB_SPRINTF`), then given to the Text object.
const NEWS_TEXT_BUFFER: usize = 0xde1f64;

/// `langlib_sprintf` into `NEWS_TEXT_BUFFER` (the game's formatter of the news templates).
unsafe fn news_text_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// `Text::append_string` (0x4ae770; thiscall, ret 4): appends a string to a Text object.
unsafe fn news_text_append(out: usize, s: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x4ae770usize);
    f(out, s);
}

/// The person's name from the news item's name ids (`exe::database_format_name_from_ids`, 300 bytes, mode `mode`)
/// into `buf`.
unsafe fn news_text_person_name(first: u32, second: u32, common: u32, buf: &mut [u8; 0x12c], mode: u32, id: u32) {
    exe::database_format_name_from_ids(first, second, common, buf.as_mut_ptr(), 0x12c, mode, id);
}

/// Links the person's name to the person (`exe::text_add_link` with `kind`) when the news item's names still match
/// him (`exe::database_staff_matches_names`).
unsafe fn news_text_link_person(out: usize, buf: &[u8; 0x12c], id: u32, first: u32, second: u32, common: u32, kind: u32) {
    let p = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(p, first, second, common, 0usize) != 0 {
        exe::text_add_link(out, buf.as_ptr(), kind, p);
    }
}

/// Links a club's short name to the club (`exe::text_add_link`, 0x7d5).
unsafe fn news_text_link_club(out: usize, id: u32) {
    let c = club_at(id as i32 as isize);
    exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c);
}

/// Writes the headline or the body of the news that a club may discuss a contract with a person, and the state of the
/// talks.
///
/// `write_contract_talks_news_text(news, full, out)`: the talks state is slot 9 of the news item; when it is 0, the
/// offer in slot 7 of the transfer manager's offers (index past the count reads the first) with the serial of slot 8
/// gives it (`TRANSFER_OFFER::get_player_talks_status`, 0x8e7f10, for the person of slot 0) and, when not 0, it is
/// written back into the stored item of the human manager's inbox (`News::set_item_slot`, 0x786d30, slot 9). The body:
/// the person's name (slots 1-3 and 0, mode 1), the template 0xa94c24 with the clubs of slots 4 and 5 (short names and
/// genders), then 0xa94be4 (state 1) or 0xa94b98 / 0xa94b48 (state 4, by slot 10) with the club of slot 5, or nothing;
/// the links to the club of slot 4 (when ≥ 0) and of slot 5, and to the person (0x7d7) when the names still match. The
/// headline: "Offer contract to" (0xa94b0c) with the name (mode 5) and the person's link.
/// Original: `TRANSFER_MANAGER::write_contract_talks_news_text` (0x8d7fb0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_contract_talks_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let person = slot(0);
    let mut state = slot(9) as u8;
    if state == 0 {
        let index = slot(7) as i32;
        let serial = slot(8);
        let tm = read_usize(G_TRANSFER_MANAGER);
        let arr = read_usize(tm);
        let at = if index < read_i32(tm + 4) { arr.wrapping_add((index as usize).wrapping_mul(4)) } else { arr };
        let o = read_usize(at);
        if o != 0 && read_u32(o + offer::SERIAL) == serial {
            let status: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x8e7f10usize); // TRANSFER_OFFER::get_player_talks_status
            state = status(o, person);
            if state != 0 {
                covered!("mke_5 talks state stored");
                let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
                let set: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, usize) = core::mem::transmute(0x786d30usize); // News::set_item_slot
                set(read_usize(G_NEWS), hm, read_u32(news + 0xd6), 9, state as i8 as i32, 0);
            }
        }
    }
    let mut name = [0u8; 0x12c];
    if full != 0 {
        let (s1, s2, s3, s4, s5, s10) = (slot(1), slot(2), slot(3), slot(4), slot(5), slot(0xa));
        news_text_person_name(s1, s2, s3, &mut name, 1, person);
        let c4 = club_at(s4 as i32 as isize).addr();
        let c5 = club_at(s5 as i32 as isize).addr();
        let g4 = read_i8(c4 + club::SHORT_GENDER_NAME) as i32;
        let g5 = read_i8(c5 + club::SHORT_GENDER_NAME) as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa94c24, g4, g4, g5, g5, c4 + club::SHORT_NAME, c5 + club::SHORT_NAME, name.as_ptr() as usize);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        match state {
            1 => {
                fmt(NEWS_TEXT_BUFFER, 0xa94be4, g5, g5, c5 + club::SHORT_NAME);
            }
            4 => {
                let t = if s10 != 0 { 0xa94b98 } else { 0xa94b48 };
                fmt(NEWS_TEXT_BUFFER, t, g5, g5, c5 + club::SHORT_NAME);
            }
            _ => write_u8(NEWS_TEXT_BUFFER, 0),
        }
        news_text_append(out, NEWS_TEXT_BUFFER);
        if (s4 as i32) > -1 {
            news_text_link_club(out, s4);
        }
        news_text_link_club(out, s5);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    } else {
        let (s1, s2, s3) = (slot(1), slot(2), slot(3));
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa94b0c, name.as_ptr() as usize); // "Offer contract to <name>"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    }
}

/// Comparer for `write_contract_talks_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_contract_talks_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_talks_news_text(this, news, full, t), || format!("write_contract_talks_news_text(news {:#x}, full {})", news, full));
}

/// The body templates of the contract expiry news by the period (slot 6): (another club, own club, other), period 1 = 3
/// months, 2 = 2 months, 3 = next month, 4 = next week, 5 = 6 months, 6 = 12 months; period 7 has one template.
const CONTRACT_EXPIRY_TEMPLATES: [(usize, usize, usize); 6] = [
    (0x9aac74, 0x9aab88, 0x9aab30),
    (0x9aaa68, 0x9aa980, 0x9aa92c),
    (0x9aa86c, 0x9aa78c, 0x9aa740),
    (0x9aa680, 0x9aa5a0, 0x9aa554),
    (0x9aaf0c, 0x9aae24, 0x9aadd0),
    (0x9ab118, 0x9ab02c, 0x9aafd4),
];

/// Writes the headline or the body of the news that a person's contract expires in twelve, six, three, two or one
/// months or next week.
///
/// `write_contract_expiry_news_text(news, full, out)`: the name from slots 3-5 and 2 (mode 1). The body by the period
/// (slot 6, `CONTRACT_EXPIRY_TEMPLATES`; slot 7 not 0 the first template, else slot 8 not 0 the second, else the third;
/// period 7 the template 0x9aad3c; another period leaves the buffer as it was), each with 100 and the name; the link
/// to the club of slot 0 (full name) and to the person (0x7d7) when the names still match. The headline: 0x9aa520
/// ("... contract") with 100 and the name, and the person's link.
/// Original: `CONTRACT_MANAGER::write_contract_expiry_news_text` (0x4d6a00, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_contract_expiry_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, person, s3, s4, s5, period, s7, s8) = (slot(0), slot(2), slot(3), slot(4), slot(5), slot(6), slot(7), slot(8));
    let mut name = [0u8; 0x12c];
    news_text_person_name(s3, s4, s5, &mut name, 1, person);
    let np = name.as_ptr() as usize;
    if full != 0 {
        match period as i32 {
            1..=6 => {
                let (a, b, c) = CONTRACT_EXPIRY_TEMPLATES[(period - 1) as usize];
                let t = if s7 != 0 { a } else if s8 != 0 { b } else { c };
                fmt(NEWS_TEXT_BUFFER, t, 100, np);
            }
            7 => {
                fmt(NEWS_TEXT_BUFFER, 0x9aad3c, 100, np);
            }
            _ => {} // the buffer keeps what it had
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        let c = club_at(club as i32 as isize);
        exe::text_add_link(out, (c.addr() + club::NAME) as *const u8, 0x7d5, c);
    } else {
        fmt(NEWS_TEXT_BUFFER, 0x9aa520, 100, np); // "<name>{s} contract"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
    news_text_link_person(out, &name, person, s3, s4, s5, 0x7d7);
}

/// Comparer for `write_contract_expiry_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_contract_expiry_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_expiry_news_text(this, news, full, t), || format!("write_contract_expiry_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a person leaves when his contract, trial or loan ends.
///
/// `write_contract_end_news_text(news, full, out)`: the club of slot 0, the person of slot 2 (name from slots 3-5, mode
/// 1), the kind of slot 6 (5 trial, 6 loan, 1 ...), the loan club of slot 7, "own" = slot 8 is 1, and whether the club of
/// slot 0 is the current human manager's (`G_SCREEN_RECORDS_MANAGER` of the current slot, its club +0x39). Headline: own
/// and not the human's club 0x9aeff4 ("<name> leaves <club>"); trial 0x9aefbc; loan 0x9aef88; neither kind 1 nor own
/// 0x9aef50 ("contract expires"); otherwise 0x9aef20 ("leaves club"). Body: trial 0x9aeec8; loan 0x9aee54 with the loan
/// club; kind 1 0x9aedc8; own: the human's club 0x9aed5c, otherwise 0x9aeccc with the club; else 0x9aec58. The person's
/// link (0x7da) when the names still match, then (body) the loan club's link when slot 7 ≠ −1, and the club's when slot
/// 0 ≠ −1.
/// Original: `CONTRACT_MANAGER::write_contract_end_news_text` (0x4daa00, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_contract_end_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, person, s3, s4, s5) = (slot(0), slot(2), slot(3), slot(4), slot(5));
    let kind = slot(6) as u8;
    let loan_club = slot(7);
    let own = slot(8) == 1;
    let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    let human_club = hm != 0 && read_usize(hm + 0x39) != 0 && club == read_u32(read_usize(hm + 0x39));
    let mut name = [0u8; 0x12c];
    news_text_person_name(s3, s4, s5, &mut name, 1, person);
    let np = name.as_ptr() as usize;
    let c0 = club_at(club as i32 as isize).addr();
    if full == 0 {
        if own && !human_club {
            let g = read_i8(c0 + club::SHORT_GENDER_NAME) as i32;
            fmt(NEWS_TEXT_BUFFER, 0x9aeff4, g, g, np, c0 + club::SHORT_NAME);
        } else if kind == 5 {
            fmt(NEWS_TEXT_BUFFER, 0x9aefbc, 100, np);
        } else if kind == 6 {
            fmt(NEWS_TEXT_BUFFER, 0x9aef88, 100, np);
        } else if kind != 1 && !own {
            fmt(NEWS_TEXT_BUFFER, 0x9aef50, 100, np);
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9aef20, np);
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s3, s4, s5, 0x7da);
        if club as i32 != -1 {
            news_text_link_club(out, club);
        }
        return;
    }
    if kind == 5 {
        fmt(NEWS_TEXT_BUFFER, 0x9aeec8, np);
    } else if kind == 6 {
        let c7 = club_at(loan_club as i32 as isize).addr();
        let g = read_i8(c7 + club::SHORT_GENDER_NAME) as i32;
        fmt(NEWS_TEXT_BUFFER, 0x9aee54, g, g, np, c7 + club::SHORT_NAME);
    } else if kind == 1 {
        fmt(NEWS_TEXT_BUFFER, 0x9aedc8, 100, np);
    } else if own {
        if human_club {
            fmt(NEWS_TEXT_BUFFER, 0x9aed5c, 100, np);
        } else {
            let g = read_i8(c0 + club::SHORT_GENDER_NAME) as i32;
            covered!("mke_5 contract end at the club");
            fmt(NEWS_TEXT_BUFFER, 0x9aeccc, g, g, 100, c0 + club::SHORT_NAME, np);
        }
    } else {
        fmt(NEWS_TEXT_BUFFER, 0x9aec58, 100, np); // mke5: the contract end body
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_person(out, &name, person, s3, s4, s5, 0x7da);
    if loan_club as i32 != -1 {
        news_text_link_club(out, loan_club);
    }
    if club as i32 != -1 {
        news_text_link_club(out, club);
    }
}

/// Comparer for `write_contract_end_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_contract_end_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_end_news_text(this, news, full, t), || format!("write_contract_end_news_text(news {:#x}, full {})", news, full));
}
