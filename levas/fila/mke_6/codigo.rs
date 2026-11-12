// ---------- batch %%LEVA%%: the news text of an accepted offer (transfer_manager.cpp) ----------

/// Writes the headline or the body of the news that a club accepted an offer, or that a player agreed to talk about a
/// contract.
///
/// `write_offer_accepted_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3; mode 1 in the body,
/// 5 in the headline), the clubs of slots 4 (−1 none) and 5, slot 6 (a talks agreement), slot 9 (1 a loan). The body:
/// a talks agreement 0xa947d8; no club in slot 4 0xa94aa4 (club 5 and the name); a loan 0xa94a14 when club 5 is the
/// current human manager's club (`G_SCREEN_RECORDS_MANAGER` of the current slot, +0x39), else 0xa94988; a transfer
/// 0xa948fc / 0xa94874 the same way; the others with the two clubs (short names, genders) and the name. The links to
/// club 4 (when ≥ 0) and club 5 (0x7d5) and to the person (0x7d7) when the names still match. The headline: "agrees to
/// contract discussion" (0xa94790) without club 4, "loan offer accepted" (0xa94754) for a loan, else "transfer offer
/// accepted" (0xa94714), and the person's link.
/// Original: `TRANSFER_MANAGER::write_offer_accepted_news_text` (0x8d7870, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_offer_accepted_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full != 0 {
        let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
        let human_club = if hm != 0 { read_usize(hm + 0x39) } else { 0 };
        let (person, s1, s2, s3, s4, s5) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
        let talks = slot(6) != 0;
        let loan = slot(9) as u8 == 1;
        news_text_person_name(s1, s2, s3, &mut name, 1, person);
        let c5 = club_at(s5 as i32 as isize).addr();
        let g5 = read_i8(c5 + club::SHORT_GENDER_NAME) as i32;
        let mine = human_club != 0 && read_u32(human_club) == s5;
        if !talks && s4 as i32 == -1 {
            fmt(NEWS_TEXT_BUFFER, 0xa94aa4, g5, g5, np, c5 + club::SHORT_NAME);
        } else {
            let t = if talks {
                0xa947d8
            } else if loan {
                if mine { 0xa94a14 } else { 0xa94988 }
            } else if mine {
                0xa948fc
            } else {
                0xa94874 // mke6: the transfer accepted body
            };
            let c4 = club_at(s4 as i32 as isize).addr();
            let g4 = read_i8(c4 + club::SHORT_GENDER_NAME) as i32;
            covered!("mke_6 offer accepted body");
            fmt(NEWS_TEXT_BUFFER, t, g4, g4, g5, g5, c4 + club::SHORT_NAME, c5 + club::SHORT_NAME, np);
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if s4 as i32 > -1 {
            news_text_link_club(out, s4);
        }
        news_text_link_club(out, s5);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    } else {
        let (person, s1, s2, s3, s4) = (slot(0), slot(1), slot(2), slot(3), slot(4));
        let loan = slot(9) as u8 == 1;
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        let t = if s4 as i32 == -1 {
            0xa94790 // "<name> agrees to contract discussion"
        } else if loan {
            0xa94754 // "<name> loan offer accepted"
        } else {
            0xa94714 // "<name> transfer offer accepted"
        };
        fmt(NEWS_TEXT_BUFFER, t, np);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    }
}

/// Comparer for `write_offer_accepted_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_offer_accepted_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_accepted_news_text(this, news, full, t), || format!("write_offer_accepted_news_text(news {:#x}, full {})", news, full));
}
