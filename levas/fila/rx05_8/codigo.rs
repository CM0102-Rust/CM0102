/// The nationality adjective of a club's nation (`exe::nation_get_nationality_adjective`, 100 bytes, form 8), as the
/// work permit news texts write it.
unsafe fn club_nationality_adjective(club: Club, out: &mut [u8; 0x64]) {
    exe::nation_get_nationality_adjective(read_u32(club.addr() + club::NATION) as usize, out.as_mut_ptr() as usize, 0x64, 8);
}

/// Writes the headline or the body of the news that a player has a conditional work permit for his move to a club.
///
/// `write_work_permit_granted_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3; mode 1 in the
/// body, 5 in the headline), the destination club of slot 4 and the player's club of slot 5 (−1 none). The body:
/// "<club 5>'s <name> has been granted a conditional <nationality> work permit for his move to <club 4>." (0xa9d444)
/// or, without club 5, "<name> has been granted ..." (0xa9d39c); the nationality is the adjective of club 4's nation
/// (`club_nationality_adjective`). The links: club 4, club 5 when there is one (0x7d5) and the person (0x7d7). The
/// headline: "<name> granted work permit" (0xa9d364) and the person's link.
/// Original: `TRANSFER_MANAGER::write_work_permit_granted_news_text` (0x8e32c0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_work_permit_granted_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full == 0 {
        let (person, s1, s2, s3) = (slot(0), slot(1), slot(2), slot(3));
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9d364, 0x64, np); // "<name> granted work permit"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
        return;
    }
    let (person, s1, s2, s3, s4, s5) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
    news_text_person_name(s1, s2, s3, &mut name, 1, person);
    let to = club_at(s4 as i32 as isize);
    let mut nationality = [0u8; 0x64];
    club_nationality_adjective(to, &mut nationality);
    let g4 = to.short_gender_name() as i8 as i32;
    let (n4, nat) = (to.addr() + club::SHORT_NAME, nationality.as_ptr() as usize);
    if s5 as i32 != -1 {
        let from = club_at(s5 as i32 as isize);
        let g5 = from.short_gender_name() as i8 as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa9d444, g5, g5, 0x64, 0x64, g4, g4, from.addr() + club::SHORT_NAME, np, nat, n4); // rx05_8: the permit granted
    } else {
        fmt(NEWS_TEXT_BUFFER, 0xa9d39c, 0x64, 0x64, g4, g4, np, nat, n4);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club(out, s4);
    if s5 as i32 != -1 {
        covered!("rx05_8 permit granted from a club");
        news_text_link_club(out, s5);
    }
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
}

/// Comparer for `write_work_permit_granted_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_work_permit_granted_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_work_permit_granted_news_text(this, news, full, t), || format!("write_work_permit_granted_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a player's release clause is active after his club's result in
/// a competition.
///
/// `write_release_clause_activated_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3 into 200
/// bytes; mode 1 in the body, 5 in the headline), the club of slot 4, the competition of slot 5, slot 6 the clause
/// kind (3 relegation) and slot 7 the fee (`exe::cash_format_money`, 200 bytes). The body: "<name> can now leave
/// <club> for a transfer fee of <fee> due to the club being relegated from" (0xa9da60, kind 3) or "... not achieving
/// promotion from" (0xa9d960) "the <competition>" (its long name and gender); the links to the person (0x7d7), the
/// club (0x7d5) and the competition (0x7d0) when it has an object (`G_COMP_OBJECT_TABLE`). The headline: "<name>
/// release clause activated" (0xa9d924) and the person's link.
/// Original: `TRANSFER_MANAGER::write_release_clause_activated_news_text` (0x8e3f90, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_release_clause_activated_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, s1, s2, s3) = (slot(0), slot(1), slot(2), slot(3));
    if full == 0 {
        exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0xc8, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9d924, 0x64, np); // "<name> release clause activated"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
        return;
    }
    let (s4, s5) = (slot(4), slot(5));
    let kind = slot(6) as u8;
    let fee = slot(7);
    let mut money = [0u8; 0xc8];
    let amount = exe::cash_get_display_amount(&fee, 6);
    exe::cash_format_money(money.as_mut_ptr() as usize, 0xc8, amount, 6);
    exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0xc8, 1, person);
    let comp = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((s5 as usize).wrapping_mul(0x6b)));
    let club = club_at(s4 as i32 as isize);
    let gc = club.short_gender_name() as i8 as i32;
    let gp = comp.gender_name() as i8 as i32;
    let t = if kind == 3 { 0xa9da60 } else { 0xa9d960 }; // rx05_8: relegated / not promoted
    fmt(NEWS_TEXT_BUFFER, t, gc, gc, gp, gp, np, club.addr() + club::SHORT_NAME, money.as_ptr() as usize, comp.addr() + competition::NAME);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    news_text_link_club(out, s4);
    if read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((s5 as usize).wrapping_mul(4))) != 0 {
        covered!("rx05_8 release clause competition link");
        exe::text_add_link(out, (comp.addr() + competition::NAME) as *const u8, 0x7d0, comp.addr());
    }
}

/// Comparer for `write_release_clause_activated_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_release_clause_activated_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_release_clause_activated_news_text(this, news, full, t), || format!("write_release_clause_activated_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a player who failed to get a work permit is removed from a part
/// exchange offer.
///
/// `write_exchange_player_permit_refused_news_text(news, full, out)`: the exchange player of slot 6 (name from slots
/// 7-9; mode 1 in the body, 5 in the headline), the selling club of slot 0, the buying club of slot 1 (his club), the
/// target of slot 2 (name from slots 3-5), slot 10 the new fee and slot 11 the old one. The body: "<buyer>'s
/// <player> has been removed from the part exchange offer for <seller>'s <target> as he failed to get a <nationality>
/// work permit" (0xa9d550; the nationality of the seller's nation, `club_nationality_adjective`), with "The transfer
/// fee will be adjusted to <fee>." (0xa9d660, `exe::cash_format_money` with 100 bytes) when the new fee is above the
/// old one (signed). The links: the seller, the buyer (0x7d5), the target and the exchange player (0x7d7). The
/// headline: "<player> fails to get work permit" (0xa9d514) and his link.
/// Original: `TRANSFER_MANAGER::write_exchange_player_permit_refused_news_text` (0x8e3780, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_exchange_player_permit_refused_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut player = [0u8; 0x12c];
    let pp = player.as_ptr() as usize;
    let (exchange, e1, e2, e3) = (slot(6), slot(7), slot(8), slot(9));
    if full == 0 {
        news_text_person_name(e1, e2, e3, &mut player, 5, exchange);
        fmt(NEWS_TEXT_BUFFER, 0xa9d514, 0x64, pp); // "<name> fails to get work permit"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &player, exchange, e1, e2, e3, 0x7d7);
        return;
    }
    let (s0, s1, target, t1, t2, t3) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
    let (new_fee, old_fee) = (slot(10), slot(11));
    let mut target_name = [0u8; 0x12c];
    news_text_person_name(t1, t2, t3, &mut target_name, 1, target);
    news_text_person_name(e1, e2, e3, &mut player, 1, exchange);
    let seller = club_at(s0 as i32 as isize);
    let buyer = club_at(s1 as i32 as isize);
    let mut nationality = [0u8; 0x64];
    let tp = target_name.as_ptr() as usize;
    let (ns, nb, nat) = (seller.addr() + club::SHORT_NAME, buyer.addr() + club::SHORT_NAME, nationality.as_ptr() as usize);
    if new_fee as i32 > old_fee as i32 {
        let mut money = [0u8; 0x64];
        let amount = exe::cash_get_display_amount(&new_fee, 6);
        exe::cash_format_money(money.as_mut_ptr() as usize, 0x64, amount, 6);
        club_nationality_adjective(seller, &mut nationality);
        let (gs, gb) = (seller.short_gender_name() as i8 as i32, buyer.short_gender_name() as i8 as i32);
        covered!("rx05_8 exchange player permit refused with the fee");
        fmt(NEWS_TEXT_BUFFER, 0xa9d660, gb, gb, gs, gs, 0x64, 0x64, nb, pp, ns, tp, nat, money.as_ptr() as usize);
    } else {
        club_nationality_adjective(seller, &mut nationality);
        let (gs, gb) = (seller.short_gender_name() as i8 as i32, buyer.short_gender_name() as i8 as i32);
        fmt(NEWS_TEXT_BUFFER, 0xa9d550, gb, gb, gs, gs, 0x64, 0x64, nb, pp, ns, tp, nat); // rx05_8: removed from the swap
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club(out, s0);
    news_text_link_club(out, s1);
    news_text_link_person(out, &target_name, target, t1, t2, t3, 0x7d7);
    news_text_link_person(out, &player, exchange, e1, e2, e3, 0x7d7);
}

/// Comparer for `write_exchange_player_permit_refused_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_exchange_player_permit_refused_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_exchange_player_permit_refused_news_text(this, news, full, t), || format!("write_exchange_player_permit_refused_news_text(news {:#x}, full {})", news, full));
}
