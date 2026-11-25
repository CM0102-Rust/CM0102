/// The transfer manager's offer a news item points to: the offer at `index` in its offer array (the first one when
/// the index is not below the count); null when that place is empty.
pub(crate) unsafe fn news_offer_at(index: i32) -> Offer {
    let tm = read_usize(G_TRANSFER_MANAGER);
    let offers = read_usize(tm);
    let at = if index < read_i32(tm + 4) { offers.wrapping_add((index as usize).wrapping_mul(4)) } else { offers };
    Offer(read_usize(at))
}

/// Stores `value` in slot `k` of the news item for the current human manager (`exe::news_set_item_slot`, with the
/// news item's id at +0xd6 and the last argument `last`).
pub(crate) unsafe fn news_store_item_slot(news: usize, k: u32, value: u32, last: u32) {
    let human = scout_manager_current_human_manager();
    exe::news_set_item_slot(read_usize(G_NEWS), human, read_usize(news + 0xd6), k, value, last);
}

/// The cash amount `value` as display text (unit 6) into a 100-byte buffer.
pub(crate) unsafe fn news_text_money_100(value: u32, buf: &mut [u8; 0x64]) {
    let amount = exe::cash_get_display_amount(&value, 6);
    exe::cash_format_money(buf.as_mut_ptr() as usize, 0x64, amount, 6);
}

/// Writes the headline or the body of the news that a club removed, or no longer wants, a player in a part exchange
/// deal.
///
/// `write_exchange_player_removed_news_text(news, full, out)`: the exchange player of slot 6 (name from slots 7-9 into
/// 300 bytes; mode 1 in the body, 5 in the headline). The headline: "<name> removed from transfer deal" (0xa94fc0) and
/// his link (0x7d7). The body: the clubs of slots 0 and 1, the target of slot 2 (name from slots 3-5), the fees of
/// slots 10 and 11 and the reason of slot 12 (a word). When slot 11 is below slot 10, slot 10 is written as money
/// (100 bytes) and the template adds "The transfer fee will be adjusted to <money>.". Reason 0x16: "<club 0> have
/// removed <exchange player> from <club 1>'s part exchange deal for <target> as they cannot afford the player's
/// wages." (0xa997fc / 0xa9971c); otherwise "<club 0> have decided that they do not want <exchange player> in part
/// exchange for <club 1>'s <target>." (0xa99608 / 0xa99540). Then the links to the two clubs (0x7d5), the target and
/// the exchange player (0x7d7).
/// Original: `TRANSFER_MANAGER::write_exchange_player_removed_news_text` (0x8de320, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_exchange_player_removed_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, ids) = (slot(6), (slot(7), slot(8), slot(9)));
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa94fc0, np); // "<name> removed from transfer deal"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        return;
    }
    let (s0, s1, target, target_ids) = (slot(0), slot(1), slot(2), (slot(3), slot(4), slot(5)));
    let (fee, old_fee, reason) = (slot(10) as i32, slot(11) as i32, slot(12) as u16);
    let mut target_name = [0u8; 0x12c];
    news_text_name_n(target_ids, &mut target_name, 1, target);
    news_text_name_n(ids, &mut name, 1, person);
    let mut money = [0u8; 0x64];
    let adjusted = old_fee < fee;
    if adjusted {
        news_text_money_100(fee as u32, &mut money);
    }
    let ((c0, g0), (c1, g1)) = (news_text_club(s0), news_text_club(s1));
    let tp = target_name.as_ptr() as usize;
    let t = match (reason == 0x16, adjusted) {
        (true, true) => 0xa997fc, // "... cannot afford the player's wages. ... adjusted to <money>."
        (true, false) => 0xa9971c,
        (false, true) => 0xa99608, // "... do not want <player> in part exchange ... adjusted to <money>."
        (false, false) => 0xa99540,
    };
    if adjusted {
        covered!("rxs1_2 exchange player removed with the fee adjusted");
        fmt(NEWS_TEXT_BUFFER, t, g0, g0, g1, g1, c0, np, c1, tp, money.as_ptr() as usize); // rxs1_2: exchange player removed
    } else {
        fmt(NEWS_TEXT_BUFFER, t, g0, g0, g1, g1, c0, np, c1, tp);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club(out, s0);
    news_text_link_club(out, s1);
    news_text_link_name_n(out, &target_name, target, target_ids);
    news_text_link_name_n(out, &name, person, ids);
}

/// Comparer for `write_exchange_player_removed_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_exchange_player_removed_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_exchange_player_removed_news_text(this, news, full, t), || format!("write_exchange_player_removed_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a proposed transfer cannot proceed because of a nation's
/// transfer deadline.
///
/// `write_offer_on_hold_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 300 bytes; mode 1
/// in the body, 5 in the headline), the buying club of slot 4, the state in slot 10. With state 1 (not known yet) the
/// offer is looked up (`news_offer_at` with slot 11; slot 12 its serial): none gives 4; one with that serial keeps
/// state 1 in status 1 and takes its status otherwise; one for the same player (`.target_staff_id()`) and buyer
/// (`.buyer_club_id()`) gives 0, anything else 4; a new state is stored in slot 10 (`news_store_item_slot`). The
/// headline: "<name> transfer to <buyer> put on hold" (0xa9c180), the links to the person (0x7d7) and the buyer
/// (0x7d5). The body: the nationality adjective of the nation of slot 9 (100 bytes,
/// `exe::nation_get_nationality_adjective` with 10); with no selling club (slot 5 = −1) "<name>'s proposed transfer to
/// <buyer> cannot proceed at the current time due to the <adjective> transfer deadline." (0xa9c218), otherwise "...
/// from <seller> to <buyer> ..." (0xa9c2e0); then the links to the person, the buyer and the seller (when there is
/// one).
///
/// Defect of the original: in states 0 and 3 it formats "A new offer was made by <buyer>" (0xa9c1dc) into the text
/// buffer but never appends it, so the news never shows it. The copy keeps it.
/// Original: `TRANSFER_MANAGER::write_offer_on_hold_news_text` (0x8e1370, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_offer_on_hold_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let (person, ids) = (slot(0), (slot(1), slot(2), slot(3)));
    let s4 = slot(4);
    let mut state = slot(10) as u8;
    if state == 1 {
        let (index, serial) = (slot(11) as i32, slot(12));
        let o = news_offer_at(index);
        let found = if o.is_null() {
            Some(4)
        } else if o.serial() as u32 == serial {
            if o.status() == 1 { None } else { Some(o.status()) }
        } else if o.target_staff_id() as u32 == person && o.buyer_club_id() as u32 == s4 {
            Some(0)
        } else {
            Some(4)
        };
        if let Some(v) = found {
            state = v;
            news_store_item_slot(news, 10, v as i8 as i32 as u32, 0);
        }
    }
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (buyer, gb) = news_text_club(s4);
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9c180, 0x64, gb, gb, np, buyer); // "<name> transfer to <buyer> put on hold"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        news_text_link_club(out, s4);
        return;
    }
    let (s5, s9) = (slot(5), slot(9));
    news_text_name_n(ids, &mut name, 1, person);
    let mut adjective = [0u8; 0x64];
    let ap = adjective.as_mut_ptr() as usize;
    exe::nation_get_nationality_adjective(nation_by_id(s9 as usize), ap, 0x64, 10);
    if s5 == u32::MAX {
        covered!("rxs1_2 offer on hold without a seller");
        fmt(NEWS_TEXT_BUFFER, 0xa9c218, 0x64, gb, gb, 0x64, 0x64, np, buyer, ap); // rxs1_2: on hold, no seller
    } else {
        let (seller, gs) = news_text_club(s5);
        fmt(NEWS_TEXT_BUFFER, 0xa9c2e0, 0x64, gs, gs, gb, gb, 0x64, 0x64, np, seller, buyer, ap);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if state == 0 || state == 3 {
        fmt(NEWS_TEXT_BUFFER, 0xa9c1dc, gb, gb, buyer); // "A new offer was made by <buyer>", never appended
    }
    news_text_link_name_n(out, &name, person, ids);
    news_text_link_club(out, s4);
    if s5 != u32::MAX {
        news_text_link_club(out, s5);
    }
}

/// Comparer for `write_offer_on_hold_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_offer_on_hold_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_on_hold_news_text(this, news, full, t), || format!("write_offer_on_hold_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a club offered to loan a player until a date, and the parent
/// club's answer.
///
/// `write_loan_offer_news_text(news, full, out)`: the answer in slot 9; with none yet (0) the offer is looked up
/// (`news_offer_at` with slot 6; slot 7 its serial), and an offer with that serial and a status stores the status in
/// slot 9 (`news_store_item_slot`, the news item itself as the last argument). The person of slot 0 (name from slots
/// 1-3 into 300 bytes; mode 1 in the body, 5 in the headline), the bidding club of slot 4. The headline: "<bidder> loan
/// offer for <name>" (0xa9c3d4), the links to the bidder (0x7d5) and the person (0x7d7). The body: the parent club of
/// slot 5, the wage of slot 8 (a local `STAFF_CONTRACT` and `exe::staff_contract_format_wage_text` with 2, 2), the
/// date of slots 10 (day) and 11 (year) as a long date (200 bytes). When the parent club's `.manager()` is not the
/// current human manager (or there is none): "<bidder> have made an offer to loan <parent>'s <name> until <date>."
/// (0xa9c550), otherwise "<bidder> have made an offer to loan <name> until <date>." (0xa9c4c0). When the manager is the
/// human (by equality alone) and the wage is above 0: "They will take over this player's wages for that period."
/// (0xa9c484) when it equals slot 12, otherwise "They will pay <wage> towards the player's wages for that period."
/// (0xa9c42c), appended. The answer: 0xf or 8 "<parent> have accepted the offer." (0xa8f660), 2 "... rejected ..."
/// (0xa8f620), 1 "... decided to negotiate ..." (0xa8f6a0, the parent's long name and gender), appended when not empty;
/// any other empties the text buffer. Then the links to the bidder, the parent and the person.
/// Original: `TRANSFER_MANAGER::write_loan_offer_news_text` (0x8e19c0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_loan_offer_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut answer = slot(9) as u8;
    if answer == 0 {
        let (index, serial) = (slot(6) as i32, slot(7));
        let o = news_offer_at(index);
        if !o.is_null() && o.serial() as u32 == serial && o.status() != 0 {
            answer = o.status();
            news_store_item_slot(news, 9, answer as i8 as i32 as u32, news as u32);
        }
    }
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, ids) = (slot(0), (slot(1), slot(2), slot(3)));
    let s4 = slot(4);
    let (bidder, gb) = news_text_club(s4);
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9c3d4, gb, gb, bidder, np); // "<bidder> loan offer for <name>"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s4);
        news_text_link_name_n(out, &name, person, ids);
        return;
    }
    let (s5, wage, day, year, s12) = (slot(5), slot(8) as i32, slot(10), slot(11), slot(12) as i32);
    let parent = club_at(s5 as i32 as isize);
    let human = scout_manager_current_human_manager();
    let manager = read_usize(parent.addr() + club::MANAGER);
    news_text_name_n(ids, &mut name, 1, person);
    let mut contract = [0u32; 0x14]; // a local STAFF_CONTRACT (0x50 bytes)
    let cp = contract.as_mut_ptr() as usize;
    exe::new_staff_contract(cp);
    write_u32(cp + 0x10, wage as u32); // the contract's wage
    let mut wage_text = [0u8; 0x64];
    exe::staff_contract_format_wage_text(cp, wage_text.as_mut_ptr() as usize, 2, 2);
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    let mut until = [0u8; 0xc8];
    let up = until.as_mut_ptr() as usize;
    exe::date_get_long_date_string(date.as_ptr() as usize, up, 0xc8, 0xff);
    if manager == 0 || manager != human {
        let gp = parent.short_gender_name() as i8 as i32;
        covered!("rxs1_2 loan offer for another club's player");
        fmt(NEWS_TEXT_BUFFER, 0xa9c550, gb, gb, gp, gp, 0x64, bidder, parent.addr() + club::SHORT_NAME, np, up); // rxs1_2: loan offer
    } else {
        fmt(NEWS_TEXT_BUFFER, 0xa9c4c0, gb, gb, 0x64, bidder, np, up);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if manager == human && wage > 0 {
        if wage == s12 {
            fmt(NEWS_TEXT_BUFFER, 0xa9c484); // "They will take over this player's wages for that period."
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa9c42c, wage_text.as_ptr() as usize); // "They will pay <wage> towards ..."
        }
        news_text_append(out, NEWS_TEXT_BUFFER);
    }
    let reply = match answer {
        0xf | 8 => Some((0xa8f660, parent.short_gender_name(), club::SHORT_NAME)), // "... have accepted the offer."
        2 => Some((0xa8f620, parent.short_gender_name(), club::SHORT_NAME)), // "... have rejected the offer."
        1 => Some((0xa8f6a0, read_u8(parent.addr() + club::GENDER_NAME), club::NAME)), // "... decided to negotiate ..."
        _ => None,
    };
    match reply {
        Some((t, g, field)) => {
            let g = g as i8 as i32;
            fmt(NEWS_TEXT_BUFFER, t, g, g, parent.addr() + field);
            if read_u8(NEWS_TEXT_BUFFER) != 0 {
                news_text_append(out, NEWS_TEXT_BUFFER);
            }
        }
        None => write_u8(NEWS_TEXT_BUFFER, 0),
    }
    news_text_link_club(out, s4);
    news_text_link_club(out, s5);
    news_text_link_name_n(out, &name, person, ids);
}

/// Comparer for `write_loan_offer_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_loan_offer_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_loan_offer_news_text(this, news, full, t), || format!("write_loan_offer_news_text(news {:#x}, full {})", news, full));
}
