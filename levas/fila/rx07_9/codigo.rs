// ---------- rx07_9: the texts of the loan recall, the Bosman departure and the withdrawn transfer request (contract_manager.cpp) ----------

/// Links a club's name at `field` (`club::NAME` or `club::SHORT_NAME`) to the club (`exe::text_add_link`, 0x7d5).
pub(crate) unsafe fn news_text_link_club_field(out: usize, c: Club, field: usize) {
    exe::text_add_link(out, (c.addr() + field) as *const u8, 0x7d5, c);
}

/// Rule: the text of news item 0xfb8 (a loan recalled or ended): slots 0 the person, 1 to 3 his names, 4 the club he
/// plays for, 5 his main contract's club, 6 the club that ended it; the name in mode 1. The body, when the main club
/// recalls him (slot 6 = slot 5): 0x9af260 with the genders of the main club, of the loan club and of the main club
/// (twice each), the main club's short name, the name, the loan club's and the main club's short names; otherwise
/// 0x9af1b4 with the loan club's gender twice, 0, the main club's gender twice, the loan club's short name, the name
/// and the main club's short name; both clubs linked (short names, the loan club first). The headline: "<name>
/// recalled from loan" (0x9af178) or "<name>{} loan terminated" (0x9af13c, with 100). Then the person's link (0x7d7)
/// when the names still match.
/// Original: `CONTRACT_MANAGER::write_loan_recall_news_text` (0x4db2e0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_loan_recall_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (person, s1, s2, s3) = (slot(0), slot(1), slot(2), slot(3));
    let loan_club = if full != 0 { slot(4) } else { 0 };
    let (main_club, by) = (slot(5), slot(6));
    let mut name = [0u8; 0x12c];
    news_text_person_name(s1, s2, s3, &mut name, 1, person);
    let np = name.as_ptr() as usize;
    let recalled = by == main_club;
    if full != 0 {
        let (m, l) = (club_at(main_club as i32 as isize), club_at(loan_club as i32 as isize));
        let (gm, gl) = (read_i8(m.addr() + club::GENDER_NAME) as i32, read_i8(l.addr() + club::GENDER_NAME) as i32);
        let (ms, ls) = (m.addr() + club::SHORT_NAME, l.addr() + club::SHORT_NAME);
        if recalled {
            covered!("rx07_9 loan recalled text");
            fmt(NEWS_TEXT_BUFFER, 0x9af260, gm, gm, gl, gl, gm, gm, ms, np, ls, ms);
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9af1b4, gl, gl, 0, gm, gm, ls, np, ms);
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club_field(out, l, club::SHORT_NAME);
        news_text_link_club_field(out, m, club::SHORT_NAME);
    } else {
        if recalled {
            fmt(NEWS_TEXT_BUFFER, 0x9af178, np); // "<name> recalled from loan"
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9af13c, 100, np); // "<name>{} loan terminated"
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
}

/// Comparer for `write_loan_recall_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_loan_recall_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_loan_recall_news_text(this, news, full, t),
        || format!("write_loan_recall_news_text(news {:#x}, full {})", news, full));
}

/// Rule: the text of news item 0xbbe (a player leaves at the end of his contract under the Bosman ruling): slots 0 his
/// club, 2 the person, 3 to 5 his names, 6 the club he joins (-1 none). The body (name mode 1): with a new club
/// 0x9ab2ec with the club's gender twice, the new club's gender twice, the name and both clubs' names; without one
/// 0x9ab258 with the club's gender twice, the name and its name; his club linked (name) and the new club too. The
/// headline (name mode 5): "<name> to join {}<club>{}" (0x9ab210, the new club's short-name gender twice and short
/// name) or "<name> to leave club" (0x9ab1e4), the new club linked (short name). Then the person's link (0x7d7) when
/// the names still match.
/// Original: `CONTRACT_MANAGER::write_bosman_departure_news_text` (0x4d6ee0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_bosman_departure_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, person, s3, s4, s5, target) = (slot(0), slot(2), slot(3), slot(4), slot(5), slot(6) as i32);
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let t = club_at(target as isize);
    if full != 0 {
        news_text_person_name(s3, s4, s5, &mut name, 1, person);
        let c = club_at(club as i32 as isize);
        let g = read_i8(c.addr() + club::GENDER_NAME) as i32;
        if target > -1 {
            covered!("rx07_9 bosman to a new club");
            let gt = read_i8(t.addr() + club::GENDER_NAME) as i32;
            fmt(NEWS_TEXT_BUFFER, 0x9ab2ec, g, g, gt, gt, np, c.addr() + club::NAME, t.addr() + club::NAME);
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9ab258, g, g, np, c.addr() + club::NAME);
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club_field(out, c, club::NAME);
        if target > -1 {
            news_text_link_club_field(out, t, club::NAME);
        }
    } else {
        news_text_person_name(s3, s4, s5, &mut name, 5, person);
        if target > -1 {
            let g = read_i8(t.addr() + club::SHORT_GENDER_NAME) as i32;
            fmt(NEWS_TEXT_BUFFER, 0x9ab210, g, g, np, t.addr() + club::SHORT_NAME); // "<name> to join {}<club>{}"
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9ab1e4, np); // "<name> to leave club"
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if target > -1 {
            news_text_link_club_field(out, t, club::SHORT_NAME);
        }
    }
    news_text_link_person(out, &name, person, s3, s4, s5, 0x7d7);
}

/// Comparer for `write_bosman_departure_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_bosman_departure_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_bosman_departure_news_text(this, news, full, t),
        || format!("write_bosman_departure_news_text(news {:#x}, full {})", news, full));
}

/// Rule: the text of news item 0xbc0 (a player withdraws his transfer request): slots 0 the club, 2 the person, 3 to 5
/// his names, 6 the reason, 7 whether he was left out. The body (name mode 1), by the reason's low byte: bit 1,
/// 0x9ad794 with the club's gender four times, the name and the club's name twice; bits 0x14 with slot 7 set, 0x9ad6b4;
/// otherwise 0x9ad608 (both with the gender twice, the name and the club's name); the club linked (name). The headline
/// (name mode 5): "<name> happy to stay" (0x9ad5dc). Then the person's link (0x7d7) when the names still match.
/// Original: `CONTRACT_MANAGER::write_transfer_request_withdrawn_news_text` (0x4d95e0, contract_manager.cpp, thiscall,
/// ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_request_withdrawn_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, person, s3, s4, s5, reason, left_out) = (slot(0), slot(2), slot(3), slot(4), slot(5), slot(6) as u8, slot(7));
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full != 0 {
        news_text_person_name(s3, s4, s5, &mut name, 1, person);
        let c = club_at(club as i32 as isize);
        let g = read_i8(c.addr() + club::GENDER_NAME) as i32;
        let club_name = c.addr() + club::NAME;
        if reason & 1 != 0 {
            covered!("rx07_9 request withdrawn reason 1");
            fmt(NEWS_TEXT_BUFFER, 0x9ad794, g, g, g, g, np, club_name, club_name);
        } else {
            covered!("rx07_9 request withdrawn other reason");
            let template = if reason & 0x14 != 0 && left_out != 0 { 0x9ad6b4 } else { 0x9ad608 };
            fmt(NEWS_TEXT_BUFFER, template, g, g, np, club_name);
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club_field(out, c, club::NAME);
    } else {
        news_text_person_name(s3, s4, s5, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0x9ad5dc, np); // "<name> happy to stay"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
    news_text_link_person(out, &name, person, s3, s4, s5, 0x7d7);
}

/// Comparer for `write_transfer_request_withdrawn_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_request_withdrawn_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_request_withdrawn_news_text(this, news, full, t),
        || format!("write_transfer_request_withdrawn_news_text(news {:#x}, full {})", news, full));
}
