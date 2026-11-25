/// Writes the headline or the body of the news that a player offered in a swap rejected the move and left the deal.
///
/// `write_swap_player_rejects_news_text(news, full, out)`: the exchange player of slot 6 (name from slots 7-9 into 300
/// bytes; mode 1 in the body, 5 in the headline). The headline: "<name> removed from transfer deal" (0xa94fc0) and
/// his link (0x7d7). The body: the clubs of slots 0 (his club) and 1 (the other club), the target of slot 2 (name
/// from slots 3-5), the fees of slots 10 and 11. "<club 1>'s <exchange player> has rejected a possible move to
/// <club 0> as part of a swap deal for <target>." (0xa95000) and, when slot 11 is below slot 10, "The transfer fee
/// will be adjusted to <slot 10>." (0xa950d4, the money in 100 bytes). Then the links to the clubs of slots 0 and 1
/// (0x7d5), the target and the exchange player (0x7d7).
/// Original: `TRANSFER_MANAGER::write_swap_player_rejects_news_text` (0x8d8af0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_swap_player_rejects_news_text(_this: usize, news: usize, full: u32, out: usize) {
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
    let (fee, old_fee) = (slot(10) as i32, slot(11) as i32);
    let mut target_name = [0u8; 0x12c];
    let tp = target_name.as_ptr() as usize;
    news_text_name_n(target_ids, &mut target_name, 1, target);
    news_text_name_n(ids, &mut name, 1, person);
    let ((c0, g0), (c1, g1)) = (news_text_club(s0), news_text_club(s1));
    if old_fee < fee {
        let mut money = [0u8; 0x64];
        news_text_money_100(fee as u32, &mut money);
        covered!("rxs1_4 swap player rejects with the fee adjusted");
        fmt(NEWS_TEXT_BUFFER, 0xa950d4, g1, g1, g0, g0, c1, np, c0, tp, money.as_ptr() as usize); // rxs1_4: swap rejected
    } else {
        fmt(NEWS_TEXT_BUFFER, 0xa95000, g1, g1, g0, g0, c1, np, c0, tp);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club(out, s0);
    news_text_link_club(out, s1);
    news_text_link_name_n(out, &target_name, target, target_ids);
    news_text_link_name_n(out, &name, person, ids);
}

/// Comparer for `write_swap_player_rejects_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_swap_player_rejects_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_swap_player_rejects_news_text(this, news, full, t), || format!("write_swap_player_rejects_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a player was released on a free transfer, and that a bid or a
/// part exchange deal for him ended.
///
/// `write_player_released_news_text(news, full, out)`: the person of slot 0. The headline: "<name> given free
/// transfer" (0xa9b79c; the name from slots 1-3 into 500 bytes, mode 5) and his link (0x7d7). The body names him by
/// his record (`exe::database_format_staff_name`, 500 bytes, mode 1) and goes by the kind in slot 12 (a byte):
/// - 0: "<name> has been released on a free tranfer by <club 5>. <club 4>'s transfer bid for the player has thus been
///   cancelled." (0xa9bad0, the original's spelling); the links to the person, club 5 and club 4.
/// - 1: the second person of slot 8 (by his record; names in slots 9-11), the fees of slots 6 and 7: "<name> has been
///   released on a free transfer by <club 5>. He has been removed from <club 5>'s part exchange transfer bid for
///   <club 4>'s <second person>." (0xa9b844) and, when slot 7 is above slot 6, "The transfer fee has been increased
///   to <slot 7>." (0xa9b964, `news_text_money`); the links to the two people, club 5 and club 4.
/// - otherwise: "<name> has been released on a free transfer by <club 5>." (0xa9b7d0); the links to the person and
///   club 5.
/// Original: `TRANSFER_MANAGER::write_player_released_news_text` (0x8dffe0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_released_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x1f4];
    let np = name.as_ptr() as usize;
    let (person, ids) = (slot(0), (slot(1), slot(2), slot(3)));
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9b79c, np); // "<name> given free transfer"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        return;
    }
    match slot(12) as u8 {
        0 => {
            let (s4, s5) = (slot(4), slot(5));
            exe::database_format_staff_name(staff_at(person as i32 as isize), name.as_mut_ptr() as usize, 0x1f4, 1);
            let ((c4, g4), (c5, g5)) = (news_text_club(s4), news_text_club(s5));
            fmt(NEWS_TEXT_BUFFER, 0xa9bad0, g5, g5, g4, g4, np, c5, c4); // "... released ... <club 4>'s transfer bid ... cancelled."
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            news_text_link_name_n(out, &name, person, ids);
            news_text_link_club(out, s5);
            news_text_link_club(out, s4);
        }
        1 => {
            let (s4, s5, s6, s7) = (slot(4), slot(5), slot(6) as i32, slot(7) as i32);
            let (second, second_ids) = (slot(8), (slot(9), slot(10), slot(11)));
            exe::database_format_staff_name(staff_at(person as i32 as isize), name.as_mut_ptr() as usize, 0x1f4, 1);
            let mut second_name = [0u8; 0x1f4];
            let sp = second_name.as_mut_ptr() as usize;
            exe::database_format_staff_name(staff_at(second as i32 as isize), sp, 0x1f4, 1);
            let ((c4, g4), (c5, g5)) = (news_text_club(s4), news_text_club(s5));
            if s6 < s7 {
                let mut money = [0u8; 0x34];
                news_text_money(s7 as u32, &mut money);
                covered!("rxs1_4 released exchange player with the fee increased");
                fmt(NEWS_TEXT_BUFFER, 0xa9b964, g5, g5, g5, g5, g4, g4, np, c5, c5, c4, sp, money.as_ptr() as usize); // rxs1_4: released from the exchange
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa9b844, g5, g5, g5, g5, g4, g4, np, c5, c5, c4, sp);
            }
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            news_text_link_name_n(out, &name, person, ids);
            news_text_link_name_n(out, &second_name, second, second_ids);
            news_text_link_club(out, s5);
            news_text_link_club(out, s4);
        }
        _ => {
            let s5 = slot(5);
            exe::database_format_staff_name(staff_at(person as i32 as isize), name.as_mut_ptr() as usize, 0x1f4, 1);
            let (c5, g5) = news_text_club(s5);
            fmt(NEWS_TEXT_BUFFER, 0xa9b7d0, g5, g5, np, c5); // "<name> has been released on a free transfer by <club>."
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            news_text_link_name_n(out, &name, person, ids);
            news_text_link_club(out, s5);
        }
    }
}

/// Comparer for `write_player_released_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_player_released_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_player_released_news_text(this, news, full, t), || format!("write_player_released_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of transfer negotiation news: the selling club's demand, and whether the bidder
/// cancelled or made a new bid.
///
/// `write_offer_negotiation_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 300 bytes; mode
/// 1 in the body, 5 in the headline), the bidding club of slot 4, the state in slot 11. With state 1 (not known yet)
/// the offer is looked up (`news_offer_at` with slot 6; slot 7 its serial) as in `write_offer_on_hold_news_text` and a
/// new state is stored in slot 11 (`news_store_item_slot`). The headline: "<club 5> negotiation over <name>"
/// (0xa94304), the links to the bidder (0x7d5, not club 5) and the person (0x7d7). The body: the selling club of slot
/// 5, the exchange players of slots 13 (count) and 14 on (`NewsPlayerList`), the fees of slots 10 and 12. When slot 10
/// is below slot 12: "<seller> will settle for a deal worth <slot 12> for <name>." (0xa94690) or "... want a deal worth
/// <slot 12> plus <players> for <name>." (0xa945dc). Otherwise, with the money of slot 10: no players, "<seller> want
/// nothing for <name>." (0xa94518, slot 10 below 1) or "... want <money> for <name>." (0xa94570); with players, "...
/// want <players> in exchange for <name>." (0xa943dc, slot 10 below 1) or "... want <money> plus <players> for
/// <name>." (0xa94470). State 3 appends "<bidder> have cancelled the deal." (0xa9439c), state 0 "<bidder> have made a
/// new bid." (0xa94360); any other state empties `NEWS_TEXT_BUFFER`. Then the links to the bidder, the seller, the
/// person and the named exchange players.
/// Original: `TRANSFER_MANAGER::write_offer_negotiation_news_text` (0x8d6e80, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_offer_negotiation_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let (person, s4) = (slot(0), slot(4));
    let mut state = slot(11) as u8;
    if state == 1 {
        let (index, serial) = (slot(6) as i32, slot(7));
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
            news_store_item_slot(news, 0xb, v as i8 as i32 as u32, 0);
        }
    }
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let ids = (slot(1), slot(2), slot(3));
    let s5 = slot(5);
    let (seller, gs) = news_text_club(s5);
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa94304, gs, gs, seller, np); // "<club> negotiation over <name>"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s4);
        news_text_link_name_n(out, &name, person, ids);
        return;
    }
    let (fee, asked) = (slot(10) as i32, slot(12) as i32);
    let players = NewsPlayerList::read(news, 0xe, slot(0xd) as i8);
    let lp = players.text.as_ptr() as usize;
    news_text_name_n(ids, &mut name, 1, person);
    let mut money = [0u8; 0x34];
    let mp = money.as_ptr() as usize;
    if fee < asked {
        news_text_money(asked as u32, &mut money);
        if players.count == 0 {
            fmt(NEWS_TEXT_BUFFER, 0xa94690, gs, gs, seller, mp, np); // "<seller> will settle for a deal worth <money> ..."
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa945dc, gs, gs, seller, mp, lp, np); // "... a deal worth <money> plus <players> ..."
        }
    } else {
        news_text_money(fee as u32, &mut money);
        match (players.count == 0, fee < 1) {
            (true, true) => fmt(NEWS_TEXT_BUFFER, 0xa94518, gs, gs, seller, np), // "<seller> want nothing for <name>."
            (true, false) => fmt(NEWS_TEXT_BUFFER, 0xa94570, gs, gs, seller, mp, np), // rxs1_4: want money
            (false, true) => fmt(NEWS_TEXT_BUFFER, 0xa943dc, gs, gs, seller, lp, np), // "... want <players> in exchange ..."
            (false, false) => {
                covered!("rxs1_4 negotiation for money and players");
                fmt(NEWS_TEXT_BUFFER, 0xa94470, gs, gs, seller, mp, lp, np) // rxs1_4: money and players
            }
        };
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    let (bidder, gb) = news_text_club(s4);
    let t = match state {
        3 => Some(0xa9439c), // "<bidder> have cancelled the deal."
        0 => Some(0xa94360), // "<bidder> have made a new bid."
        _ => None,
    };
    match t {
        Some(t) => {
            fmt(NEWS_TEXT_BUFFER, t, gb, gb, bidder);
            if read_u8(NEWS_TEXT_BUFFER) != 0 {
                news_text_append(out, NEWS_TEXT_BUFFER);
            }
        }
        None => write_u8(NEWS_TEXT_BUFFER, 0),
    }
    news_text_link_club(out, s4);
    news_text_link_club(out, s5);
    news_text_link_name_n(out, &name, person, ids);
    players.link(out);
}

/// Comparer for `write_offer_negotiation_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_offer_negotiation_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_negotiation_news_text(this, news, full, t), || format!("write_offer_negotiation_news_text(news {:#x}, full {})", news, full));
}
