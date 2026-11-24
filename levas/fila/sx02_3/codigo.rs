// The transfer offer method that drops an exchange player from the offer's exchange list (transfer_offer.cpp,
// thiscall(offer, person), ret 4).
const FN_TRANSFER_OFFER_REMOVE_EXCHANGE_PLAYER: usize = 0x8e81a0;

/// Cancels every offer and every exchange offer for a released player, removes their contract offers and posts the
/// release news (0xfb1) to the clubs involved.
///
/// - The offers for him (`exe::transfer_manager_get_offers_for_player` with 0, into a local list): a future transfer
/// (`offer_status::FUTURE_TRANSFER`) loses its record (`exe::transfer_manager_remove_future_transfer`); each one becomes
/// `offer_status::CANCELLED`, is due again in 10 days (`exe::transfer_manager_schedule_offer` with 10, 0), loses its
/// contract offers (`exe::transfer_manager_close_offer_contract_offers` and `exe::contract_manager_remove_contract_offer`
/// for the target and the buyer), `terms_state()` = 0, and each exchange player loses the contract offer of the offer's
/// slot (`exe::contract_manager_remove_contract_offer_for_transfer`) and his "terms offered" byte (+4 of the list item).
/// A human buyer (`exe::news_club_has_human_manager` on `G_NEWS`) gets news 0xfb1: 0..3 the player and his names, 4 the
/// buyer, 5 the club of his contract (not type 5; 0 without one), 0xc = 0.
/// - The offers he is an exchange player in (`exe::transfer_manager_get_exchange_offers_for_player`, into the same list
/// cleared): a human seller (the club of the target's contract) gets news 0xfb1: 0..3 the player, 4 the seller, 5 the
/// club of the player's contract, 6 the fee, 7 the player's `value()` + the fee, 8..0xb the target and his names, 0xc =
/// 1; then he leaves the offer's exchange (`FN_TRANSFER_OFFER_REMOVE_EXCHANGE_PLAYER`).
/// - The human clubs with news of him (`collect_player_news_recipients` with 0xa, up to 20) not already told get news
/// 0xfb1: 0..3 the player, 5 the club of his contract, 0xc = 2.
///
/// Defect of the original: news slot 5 reads the id of the contract club through a null pointer when the second
/// contract lookup finds none (the exchange and the last news read it with no check at all); a released player always
/// still has his contract here, so it never shows.
/// Original: `TRANSFER_MANAGER::cancel_offers_for_released_player` (0x8df9b0, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cancel_offers_for_released_player(this: usize, player: usize) {
    let this = TransferManagerFields(this);
    let player = Staff(player);
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let mut told = [0usize; 0x28]; // the original's stack array (0x158..0x1f8)
    let mut k = 0usize;
    let mut news = [0u32; 0x38];
    let n = news.as_mut_ptr() as usize;
    // the id of the club of the player's contract (not type 5), as the original reads it
    let contract_club_id = || read_usize(contract_club_excluding_type5(player.addr())) as u32;
    if exe::transfer_manager_get_offers_for_player(this, player, l, 0) != 0 {
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let o = Offer(offer_at(this.addr(), read_usize(e)));
            covered!("sx02_3 released player offers");
            if o.status() == offer_status::FUTURE_TRANSFER {
                exe::transfer_manager_remove_future_transfer(this, o);
            }
            o.set_status(offer_status::CANCELLED); // sx02_3: the offer cancelled
            exe::transfer_manager_schedule_offer(this, o, 10, 0);
            exe::transfer_manager_close_offer_contract_offers(this, o);
            let buyer = transfer_manager_club_by_id(o.buyer_club_id());
            exe::contract_manager_remove_contract_offer(read_usize(G_CONTRACT_MANAGER), transfer_manager_staff_by_id(o.target_staff_id()), buyer);
            o.set_terms_state(0);
            let exchange = o.exchange_players();
            if !exchange.is_null() {
                exchange.set_cursor(exchange.head());
                loop {
                    let x = exe::llist_get_next(exchange);
                    if x == 0 {
                        break;
                    }
                    exe::contract_manager_remove_contract_offer_for_transfer(read_usize(G_CONTRACT_MANAGER), transfer_manager_staff_by_id(read_usize(x)), o.slot_index());
                    write_u8(x + 4, 0); // exchange item +4: terms offered
                }
            }
            if exe::news_club_has_human_manager(read_usize(G_NEWS), buyer) != 0 {
                exe::new_newsitem(n, 0xfb1, 0);
                set_news_person_slots(n, 0, player);
                set_news_slot(n, 4, o.buyer_club_id() as u32);
                let club = if contract_club_excluding_type5(player.addr()) != 0 { contract_club_id() } else { 0 };
                set_news_slot(n, 5, club);
                set_news_slot(n, 0xc, 0);
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, buyer);
                told[k] = buyer.addr();
                k += 1;
            }
        }
    }
    exe::llist_clear(l);
    if exe::transfer_manager_get_exchange_offers_for_player(this, player, l) != 0 {
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let o = Offer(offer_at(this.addr(), read_usize(e)));
            let target = transfer_manager_staff_by_id(o.target_staff_id());
            let seller = contract_club_excluding_type5(target.addr());
            if exe::news_club_has_human_manager(read_usize(G_NEWS), seller) != 0 {
                exe::new_newsitem(n, 0xfb1, 0);
                set_news_person_slots(n, 0, player);
                set_news_slot(n, 4, read_u32(seller));
                set_news_slot(n, 5, contract_club_id());
                set_news_slot(n, 6, o.fee() as u32);
                set_news_slot(n, 7, (player.value() as u32).wrapping_add(o.fee() as u32));
                set_news_person_slots(n, 8, target);
                set_news_slot(n, 0xc, 1);
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, seller);
                told[k] = seller;
                k += 1;
            }
            let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_OFFER_REMOVE_EXCHANGE_PLAYER);
            remove(o.addr(), player.addr()); // sx02_3: the player leaves the exchange
        }
    }
    let mut clubs = [0usize; 0x14];
    let count = exe::transfer_manager_collect_player_news_recipients(this, player, 0xa, clubs.as_mut_ptr(), 0x14) as i8;
    if count != 0 {
        exe::new_newsitem(n, 0xfb1, 0);
        set_news_person_slots(n, 0, player);
        set_news_slot(n, 5, contract_club_id());
        set_news_slot(n, 0xc, 2);
        for &club in clubs.iter().take(count.max(0) as usize) {
            if !told[..k].contains(&club) {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
            }
        }
    }
    exe::drop_llist(l);
}

/// Comparer for `cancel_offers_for_released_player`: only the original runs (it schedules, removes contract offers
/// and posts news, none of which can run twice).
pub(crate) unsafe extern "thiscall" fn compare_cancel_offers_for_released_player(this: usize, player: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, player) // simplification: original only
}

/// The clubs of the `refused` list (club ids) kept for the news of a player's answer: the first three, then each later
/// one replaces the lowest-reputation kept club (the first of equals) when its reputation is higher.
unsafe fn keep_three_refused_clubs(refused: Llist, kept: &mut [usize; 3]) -> usize {
    let mut k = 0usize;
    if refused.is_null() || refused.count() == 0 {
        return 0;
    }
    refused.set_cursor(refused.head());
    loop {
        let e = exe::llist_get_next(refused);
        if e == 0 {
            return k;
        }
        let club = transfer_manager_club_by_id(read_usize(e));
        if k < 3 {
            kept[k] = club.addr();
            k += 1;
            continue;
        }
        let (mut lowest, mut at) = (0x2711i16, None);
        for (i, &c) in kept.iter().enumerate() {
            let reputation = Club(c).reputation() as i16;
            if reputation < lowest {
                (lowest, at) = (reputation, Some(i));
            }
        }
        if club.reputation() as i16 > lowest {
            if let Some(i) = at {
                kept[i] = club.addr(); // sx02_3: a better-known refused club
            }
        }
    }
}

/// The player's interest in joining `club` (`exe::shortlist_manager_staff_interest_in_joining_club` with 0, the loan
/// flag and the squad status of `contract`, the high nibble of `contract::SQUAD_STATUS`): (its out flags, interest x
/// 1000 truncated).
unsafe fn interest_slots(this: TransferManagerFields, person: Staff, club: Club, o: Offer, contract: Contract) -> (u32, u32) {
    let mut flags = 0u32;
    let interest = exe::shortlist_manager_staff_interest_in_joining_club(this.shortlist_manager(), person, club, 0,
        (o.offer_type() == 1) as u32, (contract.squad_status() >> 4) as u32, &mut flags as *mut u32 as u32) as f32;
    (flags, ftol_x87(interest as f64 * read_f64(rdata::F64_1000)) as u32)
}

/// Posts news 0xfa4, a player's answer to a contract offer from `club`, to the human clubs of the offer and, on a
/// refusal of all of them (`answer` 2), to the best-known clubs he turned down.
///
/// The recipients (`exe::transfer_manager_collect_offer_news_recipients`, up to 30): flags 0x2f; with a `contract`,
/// 0x2d; for answer 3 without one, 0x2d if `club` is the buyer, otherwise 0x2e. None: nothing. With answer 2, the
/// `refused` list (club ids) gives up to three clubs (`keep_three_refused_clubs`). News 0xfa4: 0..3 the person and his
/// names, 4 `other` (-1 without), 5 `club`, 6 the answer, 7 `slot_index()`, 8 `serial()`, 9 `offer_type()`, 0xa
/// `value`, 0xb `fee()`, 0xc..0xf the target and his names, 0x10 the contract's club (-1) and 0x11 its type (6 signed
/// bits; 0), 0x12..0x15 the manager of `other` and his names (-1 each), 0x16..0x19 the manager of `club`, 0x1a and 0x1b
/// the interest flags and the interest x 1000 (`interest_slots`, by the `contract` or the contract offer of the person
/// at `club`, `exe::contract_manager_find_contract_offer`; neither: 0, 0), 0x1c the length of `refused` (0), 0x1d.. the
/// kept clubs. Sent to each recipient (a recipient on answer 2 is struck from the kept clubs) and then to each kept club
/// left.
/// Original: `TRANSFER_MANAGER::post_contract_response_news` (0x8d8e90, transfer_manager.cpp, thiscall, ret 0x20).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn post_contract_response_news(this: usize, person: usize, club: usize, other: usize, answer: u32, o: usize,
                                                                   contract: usize, refused: usize, value: u32) {
    let this = TransferManagerFields(this);
    let (person, club, other, o, contract, refused) = (Staff(person), Club(club), Club(other), Offer(o), Contract(contract), Llist(refused));
    let answer = answer as u8;
    let target = transfer_manager_staff_by_id(o.target_staff_id());
    let mut kept = [0usize; 3];
    let mut k = 0usize;
    let mut flags = 0x2f;
    if answer == 2 {
        k = keep_three_refused_clubs(refused, &mut kept);
    } else if !contract.is_null() {
        flags = 0x2d;
    } else if answer == 3 {
        flags = 0x2d + (read_usize(club.addr()) != o.buyer_club_id()) as u32; // sx02_3: an answer to the buyer
    }
    let mut recipients = [0usize; 0x1e];
    let count = exe::transfer_manager_collect_offer_news_recipients(this, o, flags, recipients.as_mut_ptr(), 0x1e) as i8;
    if count == 0 {
        return;
    }
    covered!("sx02_3 contract response news");
    let mut news = [0u32; 0x38];
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfa4, 0);
    set_news_person_slots(n, 0, person);
    set_news_slot(n, 4, if other.is_null() { u32::MAX } else { read_u32(other.addr()) });
    set_news_slot(n, 5, read_u32(club.addr()));
    set_news_slot(n, 6, answer as i8 as i32 as u32);
    set_news_slot(n, 7, o.slot_index() as u32);
    set_news_slot(n, 8, o.serial() as u32);
    set_news_slot(n, 9, o.offer_type() as i8 as i32 as u32);
    set_news_slot(n, 0xa, value);
    set_news_slot(n, 0xb, o.fee() as u32);
    set_news_person_slots(n, 0xc, target);
    if !contract.is_null() {
        set_news_slot(n, 0x10, contract.club() as u32);
        set_news_slot(n, 0x11, (((contract.contract_type() << 2) as i8) >> 2) as i32 as u32);
    } else {
        set_news_slot(n, 0x10, u32::MAX);
        set_news_slot(n, 0x11, 0);
    }
    // the managers of the other club and of the club
    for (base, c) in [(0x12i8, other), (0x16, club)] {
        let manager = if c.is_null() { Staff(0) } else { c.manager() };
        if manager.is_null() {
            for i in 0..4 {
                set_news_slot(n, base + i, u32::MAX);
            }
        } else {
            set_news_person_slots(n, base, manager);
        }
    }
    let (interest_flags, interest) = if !contract.is_null() {
        interest_slots(this, person, club, o, contract)
    } else {
        let offered = Contract(exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(person.addr()), read_u32(club.addr())));
        if offered.is_null() { (0, 0) } else { interest_slots(this, person, club, o, offered) }
    };
    set_news_slot(n, 0x1a, interest_flags);
    set_news_slot(n, 0x1b, interest);
    set_news_slot(n, 0x1c, if refused.is_null() { 0 } else { refused.count() as i32 as u32 });
    for (i, &c) in kept.iter().take(k).enumerate() {
        set_news_slot(n, 0x1d + i as i8, read_u32(c));
    }
    for &r in recipients.iter().take(count.max(0) as usize) {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, r);
        if answer == 2 {
            if let Some(i) = kept[..k].iter().position(|&c| c == r) {
                kept[i] = 0;
            }
        }
    }
    for &c in kept.iter().take(k) {
        if c != 0 {
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, c);
        }
    }
}

/// Comparer for `post_contract_response_news`: only the original runs (the news goes to the managers' inboxes).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_post_contract_response_news(this: usize, person: usize, club: usize, other: usize, answer: u32, o: usize,
                                                                           contract: usize, refused: usize, value: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, usize, usize, u32)>(%%I1%%)(this, person, club, other, answer, o, contract, refused, value) // simplification: original only
}
