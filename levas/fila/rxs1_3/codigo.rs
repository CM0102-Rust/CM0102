/// The body template of a cancelled loan move by the reason (slot 6 as a byte) and whether the player's club is
/// managed by someone other than the current human (`other`); `None` leaves the text buffer as it was.
fn loan_cancelled_template(reason: u8, other: bool) -> Option<usize> {
    Some(match reason {
        4 if other => 0xa9cdc4, // "<club>'s <name> proposed loan move to <club 2> ... <club 3> do not have any space in their squad."
        4 => 0xa9cd04, // "<name> proposed loan move to <club 1> ... <club 2> do not have any space in their squad."
        9 if other => 0xa9cc44, // "<club>'s <name> proposed loan move to <club 2> ... the transfer deadline has passed."
        9 => 0xa9cbac, // "<name> proposed loan move to <club 1> ... the transfer deadline has passed."
        0x26 => 0xa9cae0, // "... cannot loan more than two players at one time."
        0x1e => 0xa9ca18, // "... has already loaned five players this season."
        0x25 => 0xa9c958, // "... have four players already on loan."
        0x1d => 0xa9c888, // "... loans between <competition> clubs are not allowed."
        0x1f => 0xa9c7c8, // "... he has played at least 5 games for <club 3>."
        _ => return None,
    })
}

/// Writes the headline or the body of the news that a proposed loan move was cancelled, with the reason.
///
/// `write_loan_cancelled_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 300 bytes; mode 1
/// in the body, 5 in the headline). The headline: "<name> loan cancelled" (0xa9c790) and the person's link (0x7d7).
/// The body: the loan club of slot 4, the player's club of slot 5, the reason of slot 6 (`loan_cancelled_template`;
/// "other" when the player's club `.manager()` is none or not the current human manager), the competition of slot 7
/// (reason 0x1d). The arguments follow the original: the genders are always the clubs' short-name genders, while the
/// names are the long names (`club::NAME`), except the loan club's short name in the "other" templates. A reason
/// without a template sets the text to whatever the shared text buffer held. Then the links to the loan club and the
/// player's club (their long names, 0x7d5) and the person.
///
/// Defects of the original (the copy keeps them): the long club names go with the short names' genders; a reason
/// without a template shows the text buffer's previous contents.
/// Original: `TRANSFER_MANAGER::write_loan_cancelled_news_text` (0x8e2570, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_loan_cancelled_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, ids) = (slot(0), (slot(1), slot(2), slot(3)));
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9c790, np); // "<name> loan cancelled"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        return;
    }
    let (s4, s5, reason, s7) = (slot(4), slot(5), slot(6) as u8, slot(7));
    let (to, from) = (club_at(s4 as i32 as isize), club_at(s5 as i32 as isize));
    let manager = read_usize(from.addr() + club::MANAGER);
    let other = manager == 0 || manager != scout_manager_current_human_manager();
    news_text_name_n(ids, &mut name, 1, person);
    if let Some(t) = loan_cancelled_template(reason, other) {
        let (g4, g5) = (to.short_gender_name() as i8 as i32, from.short_gender_name() as i8 as i32);
        let (long4, short4, long5) = (to.addr() + club::NAME, to.addr() + club::SHORT_NAME, from.addr() + club::NAME);
        match t {
            0xa9cdc4 => fmt(NEWS_TEXT_BUFFER, t, g5, g5, 0x64, g4, g4, g4, g4, long5, np, short4, short4),
            0xa9cc44 => fmt(NEWS_TEXT_BUFFER, t, g5, g5, 0x64, g4, g4, long5, np, short4),
            0xa9cbac => fmt(NEWS_TEXT_BUFFER, t, 0x64, g4, g4, np, long4),
            0xa9c888 => {
                let comp = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((s7 as usize).wrapping_mul(0x6b)));
                let gc = comp.gender_name() as i8 as i32;
                fmt(NEWS_TEXT_BUFFER, t, 0x64, g4, g4, gc, np, long4, comp.name_addr())
            }
            0xa9c7c8 => fmt(NEWS_TEXT_BUFFER, t, 0x64, g4, g4, g5, g5, np, long4, long5),
            _ => {
                covered!("rxs1_3 loan cancelled for the loan club's reason");
                fmt(NEWS_TEXT_BUFFER, t, 0x64, g4, g4, g4, g4, np, long4, long4) // rxs1_3: loan cancelled
            }
        };
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club_field(out, to, club::NAME);
    news_text_link_club_field(out, from, club::NAME);
    news_text_link_name_n(out, &name, person, ids);
}

/// Comparer for `write_loan_cancelled_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_loan_cancelled_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_loan_cancelled_news_text(this, news, full, t), || format!("write_loan_cancelled_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a proposed transfer is delayed for a few weeks awaiting one or
/// more work permit decisions.
///
/// `write_work_permit_delay_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 300 bytes;
/// mode 1 in the body, 5 in the headline). The headline: "<name> transfer delayed due to work permit" (0xa9ceac) and
/// the person's link (0x7d7). The body: the buying club of slot 4, the selling club of slot 5 (−1 none), slot 6, the
/// kind of slot 7 (a byte). Kind 0, one permit: "The proposed transfer of <name> from <seller> to <buyer> has been
/// delayed ... awaiting a decision to grant a work permit." (0xa9d17c), without "from <seller>" when there is none
/// (0xa9d0cc). Another kind names a second person (slot 8, names from slots 9-11, 300 bytes, mode 1): kind 1 with slot
/// 6 = 0 "... a work permit for <second person>." (0xa9d25c, with the seller even when there is none); otherwise
/// "... decisions to grant work permits for players involved in the transfer." (0xa9cfc8, or 0xa9cef4 without a
/// seller). The short names and their genders. Then the links to the buyer, the seller (when there is one), the person
/// and, with another kind, the second person.
///
/// Defect of the original: kind 1 with slot 6 = 0 reads the selling club even when slot 5 is −1 (the record before the
/// club table). The copy keeps it.
/// Original: `TRANSFER_MANAGER::write_work_permit_delay_news_text` (0x8e2cd0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_work_permit_delay_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, ids) = (slot(0), (slot(1), slot(2), slot(3)));
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9ceac, 0x64, np); // "<name> transfer delayed due to work permit"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        return;
    }
    let (s4, s5, s6, kind) = (slot(4), slot(5), slot(6), slot(7) as u8);
    news_text_name_n(ids, &mut name, 1, person);
    let (buyer, gb) = news_text_club(s4);
    let has_seller = s5 != u32::MAX;
    let mut second = [0u8; 0x12c];
    let (mut second_id, mut second_ids) = (0, (0, 0, 0));
    if kind == 0 {
        if has_seller {
            let (seller, gs) = news_text_club(s5);
            fmt(NEWS_TEXT_BUFFER, 0xa9d17c, 0x64, gs, gs, gb, gb, np, seller, buyer); // "... from <seller> to <buyer> ..."
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa9d0cc, 0x64, gb, gb, np, buyer); // "... to <buyer> ..."
        }
    } else {
        second_id = slot(8);
        second_ids = (slot(9), slot(10), slot(11));
        news_text_name_n(second_ids, &mut second, 1, second_id);
        if kind == 1 && s6 == 0 {
            covered!("rxs1_3 work permit delay for a second person");
            let (seller, gs) = news_text_club(s5);
            fmt(NEWS_TEXT_BUFFER, 0xa9d25c, 0x64, gs, gs, gb, gb, np, seller, buyer, second.as_ptr() as usize); // rxs1_3: second person
        } else if has_seller {
            let (seller, gs) = news_text_club(s5);
            fmt(NEWS_TEXT_BUFFER, 0xa9cfc8, 0x64, gs, gs, gb, gb, np, seller, buyer); // "... work permits for players involved ..."
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa9cef4, 0x64, gb, gb, np, buyer);
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club(out, s4);
    if has_seller {
        news_text_link_club(out, s5);
    }
    news_text_link_name_n(out, &name, person, ids);
    if kind != 0 {
        news_text_link_name_n(out, &second, second_id, second_ids);
    }
}

/// Comparer for `write_work_permit_delay_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_work_permit_delay_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_work_permit_delay_news_text(this, news, full, t), || format!("write_work_permit_delay_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a player rejected a loan or a move to a club, with his reasons.
///
/// `write_player_rejects_move_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 300 bytes;
/// mode 1 in the body, 5 in the headline), the bidding club of slot 5. The headline: "<name> rejects move to
/// <bidder>" (0xa94cc4), the links to the person (0x7d7) and the bidder (0x7d5). The body: the player's club of slot 4
/// (−1 none), the loan flag of slot 6 (a byte), the reasons from `refusal_reasons_text` (slot 4, slot 5, slots 8 and
/// 9, the flags of slot 7, no interest; 400 bytes, empty first). Without a club: "<name> has rejected the move to
/// <bidder>." (0xa94d20) or "... because he <reasons>." (0xa94d84). Otherwise, a loan: "<club>'s <name> has rejected
/// the loan offer from <bidder>." (0xa94f30); a move: "<club>'s <name> has rejected the move to <bidder>." (0xa94e04)
/// or "... because he <reasons>." (0xa94e8c). The short names and their genders. Then the links to the player's club
/// (when its id is not negative), the bidder and the person.
/// Original: `TRANSFER_MANAGER::write_player_rejects_move_news_text` (0x8d84e0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_rejects_move_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, ids) = (slot(0), (slot(1), slot(2), slot(3)));
    let (s5, loan, flags, s8, s9) = (slot(5), slot(6) as u8 == 1, slot(7), slot(8), slot(9));
    let (bidder, gb) = news_text_club(s5);
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa94cc4, gb, gb, np, bidder); // "<name> rejects move to <bidder>"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        news_text_link_club(out, s5);
        return;
    }
    let s4 = slot(4);
    news_text_name_n(ids, &mut name, 1, person);
    let mut reasons = [0u8; 0x190];
    let rp = reasons.as_mut_ptr() as usize;
    let count = refusal_reasons_text(s4, s5, s8, s9, flags, 0.0, rp);
    if s4 == u32::MAX {
        if count == 0 {
            fmt(NEWS_TEXT_BUFFER, 0xa94d20, gb, gb, np, bidder); // "<name> has rejected the move to <bidder>."
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa94d84, gb, gb, np, bidder, rp); // "... because he <reasons>."
        }
    } else {
        let (club, gc) = news_text_club(s4);
        if loan {
            fmt(NEWS_TEXT_BUFFER, 0xa94f30, gc, gc, gb, gb, club, np, bidder); // "... rejected the loan offer from <bidder>."
        } else if count == 0 {
            fmt(NEWS_TEXT_BUFFER, 0xa94e04, gc, gc, gb, gb, club, np, bidder);
        } else {
            covered!("rxs1_3 player rejects move with reasons");
            fmt(NEWS_TEXT_BUFFER, 0xa94e8c, gc, gc, gb, gb, club, np, bidder, rp); // rxs1_3: rejects with reasons
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if (s4 as i32) >= 0 {
        news_text_link_club(out, s4);
    }
    news_text_link_club(out, s5);
    news_text_link_name_n(out, &name, person, ids);
}

/// Comparer for `write_player_rejects_move_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_player_rejects_move_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_player_rejects_move_news_text(this, news, full, t), || format!("write_player_rejects_move_news_text(news {:#x}, full {})", news, full));
}
