/// The person's name from a news item's name ids into `buf` (`exe::database_format_name_from_ids`, the size of `buf`,
/// mode `mode`: 1 in a body, 5 in a headline).
pub(crate) unsafe fn news_text_name_n<const N: usize>(ids: (u32, u32, u32), buf: &mut [u8; N], mode: u32, id: u32) {
    exe::database_format_name_from_ids(ids.0, ids.1, ids.2, buf.as_mut_ptr(), N as u32, mode, id);
}

/// Links the person's name in `buf` to the person (0x7d7) when the news item's names still match him
/// (`exe::database_staff_matches_names`).
pub(crate) unsafe fn news_text_link_name_n<const N: usize>(out: usize, buf: &[u8; N], id: u32, ids: (u32, u32, u32)) {
    let p = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(p, ids.0, ids.1, ids.2, 0usize) != 0 {
        exe::text_add_link(out, buf.as_ptr(), 0x7d7, p);
    }
}

/// The club of a news slot with its short name's address and gender (signed, as the templates take it).
pub(crate) unsafe fn news_text_club(id: u32) -> (usize, i32) {
    let c = club_at(id as i32 as isize);
    (c.addr() + club::SHORT_NAME, c.short_gender_name() as i8 as i32)
}

/// Writes the headline or the body of the news that a club gets a percentage of a player's transfer fee by a
/// sell-on clause.
///
/// `write_sale_percentage_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 400 bytes; mode 1
/// in the body, 5 in the headline), the percentage in slot 4 (signed byte), the club of slot 5. The headline: "<club>
/// get <percentage>% of <name>'s transfer fee" (0xa9721c) and the links to the club (0x7d5) and the person (0x7d7).
/// The body: "<club> have been given <money> because of a <percentage>% sell-on clause for <name>'s transfer from
/// <club 2> to <club 3>." (0xa97290; the money of slot 7, `news_text_money`; club 2 of slot 6, club 3 of slot 9) and
/// the links to the three clubs and the person.
///
/// Defect of the original: the body passes the grammar markers as club 1, club 1, club 2, club 2, 0x64, club 3,
/// club 3, while the template asks club 1, club 1, the name's 0x64, club 2, club 2: the name's possessive takes club
/// 2's gender and club 2's second mark takes 0x64. The copy keeps it.
/// Original: `TRANSFER_MANAGER::write_sale_percentage_news_text` (0x8db580, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_sale_percentage_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x190];
    let np = name.as_ptr() as usize;
    let (person, ids) = (slot(0), (slot(1), slot(2), slot(3)));
    let percentage = slot(4) as u8 as i8 as i32;
    let s5 = slot(5);
    let (c1, g1) = news_text_club(s5);
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9721c, g1, g1, 0x64, c1, percentage, np); // "<club> get <n>% of <name>'s transfer fee"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s5);
    } else {
        let (s6, s9) = (slot(6), slot(9));
        let mut money = [0u8; 0x34];
        news_text_money(slot(7), &mut money);
        news_text_name_n(ids, &mut name, 1, person);
        let ((c2, g2), (c3, g3)) = (news_text_club(s6), news_text_club(s9));
        covered!("rxs1_1 sale percentage body");
        fmt(NEWS_TEXT_BUFFER, 0xa97290, g1, g1, g2, g2, 0x64, g3, g3, c1, money.as_ptr() as usize, percentage, np, c2, c3); // rxs1_1: sell-on clause
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s5);
        news_text_link_club(out, s6);
        news_text_link_club(out, s9);
    }
    news_text_link_name_n(out, &name, person, ids);
}

/// Comparer for `write_sale_percentage_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_sale_percentage_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_sale_percentage_news_text(this, news, full, t), || format!("write_sale_percentage_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a club paid another club money after a player's league or
/// international appearances.
///
/// `write_appearance_money_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 400 bytes; mode
/// 1 in the body, 5 in the headline), the money of slot 4 (`news_text_money`), the receiving club of slot 6. The
/// headline: "<club> receive <money> from <name>'s transfer deal" (0xa973b4) and the links to the club (0x7d5) and the
/// person (0x7d7). The body: "<payer> have paid <club> <money> as part of the <name>'s transfer deal." with the paying
/// club of slot 8 and, by the kind (slot 7 as a byte: 2 league games, otherwise international games) and the count
/// (slot 5: 1 "his first ... game", otherwise "<count> ... games"), the second sentence naming the payer again:
/// 0xa977b4 / 0xa97690 (league) or 0xa97564 / 0xa9742c (international); the links to the payer, the club and the
/// person.
/// Original: `TRANSFER_MANAGER::write_appearance_money_news_text` (0x8dba80, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_appearance_money_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x190];
    let np = name.as_ptr() as usize;
    let mut money = [0u8; 0x34];
    let mp = money.as_ptr() as usize;
    let (person, ids) = (slot(0), (slot(1), slot(2), slot(3)));
    let (cash, s6) = (slot(4), slot(6));
    let (club, gc) = news_text_club(s6);
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        news_text_money(cash, &mut money);
        fmt(NEWS_TEXT_BUFFER, 0xa973b4, gc, gc, 0x64, club, mp, np); // "<club> receive <money> from <name>'s transfer deal"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s6);
    } else {
        let (count, kind, s8) = (slot(5), slot(7) as u8, slot(8));
        news_text_name_n(ids, &mut name, 1, person);
        news_text_money(cash, &mut money);
        let (payer, gp) = news_text_club(s8);
        if count == 1 {
            let t = if kind == 2 { 0xa977b4 } else { 0xa97564 }; // "... his first league / international game ..."
            fmt(NEWS_TEXT_BUFFER, t, gp, gp, gc, gc, 0x64, gp, gp, payer, club, mp, np, payer);
        } else {
            covered!("rxs1_1 appearance money with a count");
            let t = if kind == 2 { 0xa97690 } else { 0xa9742c }; // "... <count> league / international games ..."
            fmt(NEWS_TEXT_BUFFER, t, gp, gp, gc, gc, 0x64, gp, gp, payer, club, mp, np, count, payer); // rxs1_1: appearance count
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s8);
        news_text_link_club(out, s6);
    }
    news_text_link_name_n(out, &name, person, ids);
}

/// Comparer for `write_appearance_money_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_appearance_money_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_appearance_money_news_text(this, news, full, t), || format!("write_appearance_money_news_text(news {:#x}, full {})", news, full));
}

/// The template of the transfer delay request by the reason (slot 10 as a byte), for an exchange deal (slot 6 = 0x11,
/// "transfer to") or a plain one ("transfer from"); `None` leaves the text buffer as it was.
fn transfer_delay_template(exchange: bool, reason: u8) -> Option<usize> {
    if exchange {
        match reason {
            0 => Some(0xa994a4), // "... by a week."
            r if r & 5 != 0 => Some(0xa993b8), // "... no space in their squad for the exchanged player(s)."
            r if r & 0x23 != 0 => Some(0xa992ec), // "... too many staff employed."
            _ => None,
        }
    } else {
        match reason {
            0 => Some(0xa9924c), // "... by a week."
            3 => Some(0xa9917c), // "... cannot afford the transfer fee."
            4 => Some(0xa990a4), // "... not have enough space in their squad."
            0x22 => Some(0xa98fd8), // "... too many staff employed."
            _ => None,
        }
    }
}

/// Writes the headline or the body of the news that a club would like to delay a player's transfer by a week, and
/// whether the delay was accepted.
///
/// `write_transfer_delay_request_news_text(news, full, out)`: the person of slot 2 (name from slots 3-5 into 300
/// bytes; mode 1 in the body, 5 in the headline). The headline: "<name> transfer delayed" (0xa98f64) and the person's
/// link (0x7d7). The body: clubs of slots 0 and 1; in an exchange deal (slot 6 = 0x11) the asking club is slot 0 and
/// the other slot 1, otherwise the reverse; "<asking club> would like to delay <name>'s transfer to/from <other> by a
/// week" with the reason of slot 10 (`transfer_delay_template`); a reason without a template sets the text buffer as
/// it was. With an answer (slot 7 ≠ 0), "<club> have accepted the delay." (slot 8 ≠ 0, 0xa98f98) or "<club> have
/// cancelled the transfer." (0xa96bb4) is appended, the club being slot 1 in an exchange deal and slot 0 otherwise.
/// Then the links to the person, the club of slot 1 and the club of slot 0.
///
/// Defect of the original: an exchange deal with a reason outside 0, 5 and 0x23 (and a plain deal outside 0, 3, 4 and
/// 0x22) writes no body and sets the text to whatever the shared text buffer held before. The copy keeps it.
/// Original: `TRANSFER_MANAGER::write_transfer_delay_request_news_text` (0x8ddc70, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_delay_request_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, ids) = (slot(2), (slot(3), slot(4), slot(5)));
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa98f64, 0x64, np); // "<name> transfer delayed"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        return;
    }
    let (s0, s1) = (slot(0), slot(1));
    let exchange = slot(6) as u8 == 0x11;
    let answered = slot(7) as u8 != 0;
    let accepted = slot(8) as u8 != 0;
    let reason = slot(10) as u8;
    news_text_name_n(ids, &mut name, 1, person);
    let (asking, other) = if exchange { (s0, s1) } else { (s1, s0) };
    if let Some(t) = transfer_delay_template(exchange, reason) {
        let ((ca, ga), (co, go)) = (news_text_club(asking), news_text_club(other));
        covered!("rxs1_1 delay request body");
        fmt(NEWS_TEXT_BUFFER, t, ga, ga, 0x64, go, go, ca, np, co); // rxs1_1: the delay request
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if answered {
        let (c, g) = news_text_club(if exchange { s1 } else { s0 });
        let t = if accepted { 0xa98f98 } else { 0xa96bb4 }; // "<club> have accepted the delay." / "... cancelled the transfer."
        fmt(NEWS_TEXT_BUFFER, t, g, g, c);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_text_link_name_n(out, &name, person, ids);
    news_text_link_club(out, s1);
    news_text_link_club(out, s0);
}

/// Comparer for `write_transfer_delay_request_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_delay_request_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_delay_request_news_text(this, news, full, t), || format!("write_transfer_delay_request_news_text(news {:#x}, full {})", news, full));
}
