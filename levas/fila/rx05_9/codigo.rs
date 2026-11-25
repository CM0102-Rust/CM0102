/// Writes the headline or the body of the news that a player decided to join a club on loan until a date.
///
/// `write_loan_accepted_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3; mode 1 in the body,
/// 5 in the headline), the loan club of slot 4, his club of slot 5, the end date in slots 8 (day) and 9 (year). The
/// body: "<club 5>'s <name> has decided to join <club 4> on loan until <date>." (0xa9c6d8), or without the first
/// club when club 5's `.manager()` is the current human manager (`G_SCREEN_RECORDS_MANAGER` of the current slot)
/// (0xa9c644); the date by `exe::date_get_long_date_string`. The links: club 4, club 5 (0x7d5) and the person (0x7d7).
/// The headline: "<name> accepts loan contract" (0xa9c604) and the person's link.
/// Original: `TRANSFER_MANAGER::write_loan_accepted_news_text` (0x8e20a0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_loan_accepted_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full == 0 {
        let (person, s1, s2, s3) = (slot(0), slot(1), slot(2), slot(3));
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9c604, 0x64, np); // "<name> accepts loan contract"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
        return;
    }
    let (person, s1, s2, s3, s4, s5) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
    let (day, year) = (slot(8), slot(9));
    let from = club_at(s5 as i32 as isize);
    let human = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    let manager = read_usize(from.addr() + club::MANAGER);
    let mine = manager != 0 && manager == human;
    news_text_person_name(s1, s2, s3, &mut name, 1, person);
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    let mut until = [0u8; 0xc8];
    exe::date_get_long_date_string(date.as_ptr() as usize, until.as_mut_ptr() as usize, 0xc8, 0xff);
    let to = club_at(s4 as i32 as isize);
    let g4 = to.short_gender_name() as i8 as i32;
    let up = until.as_ptr() as usize;
    if !mine {
        let g5 = from.short_gender_name() as i8 as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa9c6d8, g5, g5, 0x64, g4, g4, 0x64, from.addr() + club::SHORT_NAME, np, to.addr() + club::SHORT_NAME, up); // rx05_9: loan accepted
    } else {
        covered!("rx05_9 loan accepted from the human's club");
        fmt(NEWS_TEXT_BUFFER, 0xa9c644, 0x64, g4, g4, 0x64, np, to.addr() + club::SHORT_NAME, up);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club(out, s4);
    news_text_link_club(out, s5);
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
}

/// Comparer for `write_loan_accepted_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_loan_accepted_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_loan_accepted_news_text(this, news, full, t), || format!("write_loan_accepted_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of loan negotiation news, adding a line when the bidder cancelled or made a new
/// offer.
///
/// `write_loan_negotiation_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 200 bytes;
/// mode 1 in the body, 5 in the headline), the bidding club of slot 4, the player's club of slot 5, slot 10 the state.
/// With state 1 (not known yet) the offer is looked up (slot 6 its index in `G_TRANSFER_MANAGER`'s offers, the first
/// one when the index is not below the count; slot 7 its serial): an offer with that serial in status 1 keeps state
/// 1; in another status the state becomes that status and is stored in the news item's slot 11
/// (`exe::news_set_item_slot` for the current human manager); an offer for the same player (`.target_staff_id()`)
/// and bidder (`.buyer_club_id()`) gives 0, anything else 4, both stored in slot 10. The body: "<club 5> want to
/// negotiate over <club 4>'s loan offer for <name>." (0xa9dcf4, the long names and genders), the links to both clubs
/// (their long names, 0x7d5) and the person (0x7d7); then state 3 "<club 4> have cancelled the deal." (0xa9439c),
/// state 0 "<club 4> have made a new offer." (0xa9dcb8), appended when not empty; any other state empties
/// `NEWS_TEXT_BUFFER`. The headline: "<name> loan offer negotiation" (0xa9dc7c) and the person's link.
///
/// Defect of the original: the status of a found offer is stored in slot 11, not in the state slot 10, so the next
/// writing looks the offer up again. The copy keeps it.
/// Original: `TRANSFER_MANAGER::write_loan_negotiation_news_text` (0x8e47a0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_loan_negotiation_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let (person, s1, s2, s3, s4, s5) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
    let mut state = slot(10) as u8;
    if state == 1 {
        let index = slot(6) as i32;
        let serial = slot(7);
        let tm = read_usize(G_TRANSFER_MANAGER);
        let offers = read_usize(tm);
        let at = if index < read_i32(tm + 4) { offers.wrapping_add((index as usize).wrapping_mul(4)) } else { offers };
        let o = Offer(read_usize(at));
        let stored = if !o.is_null() && o.serial() as u32 == serial {
            if o.status() == 1 { None } else { Some((0xb, o.status())) }
        } else if !o.is_null() && o.target_staff_id() as u32 == person && o.buyer_club_id() as u32 == s4 {
            Some((0xa, 0))
        } else {
            Some((0xa, 4))
        };
        if let Some((k, v)) = stored {
            state = v;
            let human = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
            exe::news_set_item_slot(read_usize(G_NEWS), human, read_usize(news + 0xd6), k, v as i8 as i32 as u32, 0);
        }
    }
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full == 0 {
        exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0xc8, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9dc7c, np); // "<name> loan offer negotiation"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
        return;
    }
    exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0xc8, 1, person);
    let bidder = club_at(s4 as i32 as isize);
    let owner = club_at(s5 as i32 as isize);
    let (gb, go) = (bidder.gender_name() as i8 as i32, owner.gender_name() as i8 as i32);
    fmt(NEWS_TEXT_BUFFER, 0xa9dcf4, go, go, gb, gb, owner.addr() + club::NAME, bidder.addr() + club::NAME, np);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, (bidder.addr() + club::NAME) as *const u8, 0x7d5, bidder);
    exe::text_add_link(out, (owner.addr() + club::NAME) as *const u8, 0x7d5, owner);
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    let g = bidder.short_gender_name() as i8 as i32;
    let t = match state {
        3 => 0xa9439c, // "<club> have cancelled the deal." rx05_9
        0 => 0xa9dcb8, // "<club> have made a new offer."
        _ => {
            write_u8(NEWS_TEXT_BUFFER, 0);
            return;
        }
    };
    covered!("rx05_9 loan negotiation outcome");
    fmt(NEWS_TEXT_BUFFER, t, g, g, bidder.addr() + club::SHORT_NAME);
    if read_u8(NEWS_TEXT_BUFFER) != 0 {
        news_text_append(out, NEWS_TEXT_BUFFER);
    }
}

/// Comparer for `write_loan_negotiation_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_loan_negotiation_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_loan_negotiation_news_text(this, news, full, t), || format!("write_loan_negotiation_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a former club gets a cut of a player's transfer fee.
///
/// `write_former_club_fee_cut_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 400 bytes;
/// mode 1 in the body, 5 in the headline), the buying club of slot 4, the cut in slot 5 (`exe::cash_format_money`,
/// 0x32 bytes), the former club of slot 6 and the selling club of slot 7. The body: "<former club> have been given
/// <cut> as part of <name>'s transfer from <seller> to <buyer>." (0xa9e1cc) and the links to the three clubs (former,
/// seller, buyer; 0x7d5) and the person (0x7d7). The headline: "<former club> get cut of <name>'s transfer fee"
/// (0xa9e2bc) with the links to the former club and the person.
/// Original: `TRANSFER_MANAGER::write_former_club_fee_cut_news_text` (0x8e51d0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_former_club_fee_cut_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let (person, s1, s2, s3, s4, cut, s6, s7) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5), slot(6), slot(7));
    let mut name = [0u8; 0x190];
    let np = name.as_ptr() as usize;
    let former = club_at(s6 as i32 as isize);
    let gf = former.short_gender_name() as i8 as i32;
    let link_person = |name: &[u8; 0x190]| {
        let p = staff_at(person as i32 as isize);
        if exe::database_staff_matches_names(p, s1, s2, s3, 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, p);
        }
    };
    if full == 0 {
        exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0x190, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9e2bc, gf, gf, 0x64, former.addr() + club::SHORT_NAME, np); // "... get cut of ..."
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s6);
        link_person(&name);
        return;
    }
    let mut money = [0u8; 0x34];
    news_text_money(cut, &mut money);
    exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0x190, 1, person);
    let buyer = club_at(s4 as i32 as isize);
    let seller = club_at(s7 as i32 as isize);
    let (gb, gs) = (buyer.short_gender_name() as i8 as i32, seller.short_gender_name() as i8 as i32);
    covered!("rx05_9 former club fee cut body");
    fmt(NEWS_TEXT_BUFFER, 0xa9e1cc, gf, gf, 0x64, gs, gs, gb, gb, former.addr() + club::SHORT_NAME, money.as_ptr() as usize, np, seller.addr() + club::SHORT_NAME, buyer.addr() + club::SHORT_NAME); // rx05_9: fee cut
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club(out, s6);
    news_text_link_club(out, s7);
    news_text_link_club(out, s4);
    link_person(&name);
}

/// Comparer for `write_former_club_fee_cut_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_former_club_fee_cut_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_former_club_fee_cut_news_text(this, news, full, t), || format!("write_former_club_fee_cut_news_text(news {:#x}, full {})", news, full));
}
