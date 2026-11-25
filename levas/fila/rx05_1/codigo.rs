/// The id of the club of the person's main contract (`transfer_manager_main_contract_stored`, not of type 5), or −1.
/// The original looks the contract up twice (both lookups land in `G_CONTRACT_LOOKUP_RESULT`); a contract that
/// vanished between them would read address 0, as the copy does.
pub(crate) unsafe fn news_contract_club_id_or_none(p: Staff) -> u32 {
    let c = transfer_manager_main_contract_stored(p.addr());
    if c.is_null() || c.contract_type() & 0x3f == 5 || friendly_club_by_id(c.club()).is_null() {
        return u32::MAX;
    }
    read_u32(offer_contract_club(transfer_manager_main_contract_stored(p.addr()), true))
}

/// The id of the club of the person's main contract (not of type 5) for a news slot; without one the original reads
/// address 0 (and crashes), as the copy does.
pub(crate) unsafe fn news_contract_club_id(p: Staff) -> u32 {
    read_u32(offer_contract_club(transfer_manager_main_contract_stored(p.addr()), true))
}

/// Sends the local news item to the first `count` clubs of `clubs` (`exe::news_send_to_club_manager`).
pub(crate) unsafe fn send_news_to_clubs(n: usize, clubs: &[usize], count: i8) {
    for &c in clubs.iter().take(if count > 0 { count as usize } else { 0 }) {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, c);
    }
}

/// Posts the news that the selling club answered a transfer bid with a counter demand, to the bidding club.
///
/// `post_offer_negotiation_news(negotiation, offer)`, from `add_transfer_negotiation` (0x8be9d0) when the buyer is
/// human and the offer is not a loan. The recipients come from the offer
/// (`exe::transfer_manager_collect_offer_news_recipients` with 1, up to 30); the news 0xfa2 is built even with none.
/// Slots: 0..3 the player (the negotiation's `target_staff_id()`, `write_person_news_slots`), 4 the negotiation's
/// `buyer_club_id()`, 5 the club of the player's main contract (`news_contract_club_id`), 6 and 7 the offer's
/// `slot_index()` and `serial()`, 8 and 9 the negotiation's, 10 the negotiation's `fee()`, 11 the offer's `status()`
/// (signed), 12 the negotiation's total value (`exe::transfer_offer_get_total_value` with 0, 1, 0, 1); from 14, four
/// slots for each of the first three exchange players of the negotiation (`exchange_players()`, rewound), and 13 the
/// count of all of them. Then the item goes to each recipient.
/// Original: `TRANSFER_MANAGER::post_offer_negotiation_news` (0x8d6bf0, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_offer_negotiation_news(this: usize, negotiation: usize, offer: usize) {
    let negotiation = Offer(negotiation);
    let offer = Offer(offer);
    let p = staff_records_staff_by_id(negotiation.target_staff_id());
    let mut clubs = [0usize; 30];
    let q = exe::transfer_manager_collect_offer_news_recipients(this, offer, 1, clubs.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfa2, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, negotiation.buyer_club_id() as u32);
    exe::newsitem_set_slot(n, 5, news_contract_club_id(p));
    exe::newsitem_set_slot(n, 6, offer.slot_index() as u32);
    exe::newsitem_set_slot(n, 7, offer.serial() as u32);
    exe::newsitem_set_slot(n, 8, negotiation.slot_index() as u32);
    exe::newsitem_set_slot(n, 9, negotiation.serial() as u32);
    exe::newsitem_set_slot(n, 10, negotiation.fee() as u32);
    exe::newsitem_set_slot(n, 11, offer.status() as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 12, exe::transfer_offer_get_total_value(negotiation, 0, 1, 0, 1) as u32);
    let mut count: i8 = 0;
    let list = negotiation.exchange_players();
    if !list.is_null() {
        transfer_manager_llist_rewind(list);
        let mut x = exe::llist_get_next(list);
        while x != 0 {
            let e = staff_records_staff_by_id(read_usize(x));
            if count < 3 {
                covered!("rx05_1 exchange player in the negotiation news");
                write_person_news_slots(n, 0xe + 4 * count, e);
            }
            count += 1;
            x = exe::llist_get_next(negotiation.exchange_players());
        }
    }
    exe::newsitem_set_slot(n, 0xd, count as i32 as u32);
    send_news_to_clubs(n, &clubs, q);
}

/// Comparer for `post_offer_negotiation_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_offer_negotiation_news(this: usize, negotiation: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, negotiation, offer) // simplification: original only
}

/// Posts the news that the player's club wants to negotiate over a loan offer, to the bidding club.
///
/// `post_loan_negotiation_news(negotiation, offer)`, from `add_transfer_negotiation` (0x8be9d0) when the buyer is human
/// and the offer is a loan. News 0xfc0: 0..3 the player (the offer's `target_staff_id()`), 4 the negotiation's
/// `buyer_club_id()`, 5 the club of the player's main contract (`news_contract_club_id`), 6 and 7 the offer's
/// `slot_index()` and `serial()`, 8 and 9 the negotiation's, 10 the offer's `status()` (signed); it goes to the club
/// of the offer's `buyer_club_id()`.
/// Original: `TRANSFER_MANAGER::post_loan_negotiation_news` (0x8e4610, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_loan_negotiation_news(_this: usize, negotiation: usize, offer: usize) {
    let negotiation = Offer(negotiation);
    let offer = Offer(offer);
    let p = staff_records_staff_by_id(offer.target_staff_id());
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfc0, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, negotiation.buyer_club_id() as u32);
    exe::newsitem_set_slot(n, 5, news_contract_club_id(p));
    exe::newsitem_set_slot(n, 6, offer.slot_index() as u32);
    exe::newsitem_set_slot(n, 7, offer.serial() as u32);
    exe::newsitem_set_slot(n, 8, negotiation.slot_index() as u32);
    exe::newsitem_set_slot(n, 9, negotiation.serial() as u32);
    exe::newsitem_set_slot(n, 10, offer.status() as i8 as i32 as u32);
    covered!("rx05_1 loan negotiation news");
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, friendly_club_by_id(offer.buyer_club_id()));
}

/// Comparer for `post_loan_negotiation_news`: the original only (the item goes to the manager's inbox).
pub(crate) unsafe extern "thiscall" fn compare_post_loan_negotiation_news(this: usize, negotiation: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, negotiation, offer) // simplification: original only
}

/// Posts the news that a player offered in a part exchange refused the move and left the deal.
///
/// `post_swap_player_rejects_news(offer, person, reason)`, from `accept_transfer_bid` (0x8c0080). The recipients come
/// from the offer (`exe::transfer_manager_collect_offer_news_recipients` with 3, up to 2); with none, nothing. News
/// 0xfaf: 0 the id of the club of the target's main contract (`news_contract_club_id`; none reads address 0), 1 the
/// offer's `buyer_club_id()`, 2..5 the target (`target_staff_id()`), 6..9 the exchange player `person`, 10 `reason`,
/// 11 the offer's `fee()`.
/// Original: `TRANSFER_MANAGER::post_swap_player_rejects_news` (0x8d8910, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_swap_player_rejects_news(this: usize, offer: usize, person: usize, reason: u32) {
    let offer = Offer(offer);
    let p = staff_records_staff_by_id(offer.target_staff_id());
    let seller = offer_contract_club(transfer_manager_main_contract_stored(p.addr()), true);
    let buyer = friendly_club_by_id(offer.buyer_club_id());
    let mut clubs = [0usize; 2];
    let q = exe::transfer_manager_collect_offer_news_recipients(this, offer, 3, clubs.as_mut_ptr(), 2) as i8;
    if q == 0 {
        return;
    }
    covered!("rx05_1 swap player rejects news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfaf, 0);
    exe::newsitem_set_slot(n, 0, read_u32(seller));
    exe::newsitem_set_slot(n, 1, read_u32(buyer.addr()));
    write_person_news_slots(n, 2, p);
    write_person_news_slots(n, 6, Staff(person));
    exe::newsitem_set_slot(n, 10, reason);
    exe::newsitem_set_slot(n, 11, offer.fee() as u32);
    send_news_to_clubs(n, &clubs, q);
}

/// Comparer for `post_swap_player_rejects_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_swap_player_rejects_news(this: usize, offer: usize, person: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I2%%)(this, offer, person, reason) // simplification: original only
}
