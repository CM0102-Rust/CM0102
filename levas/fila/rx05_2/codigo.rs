/// Posts the news that a player left a part exchange deal, to the buying club when a human manages it.
///
/// `post_exchange_player_removed_news(offer, person, value, reason)`, from `remove_exchange_player` (0x8c41c0) and the
/// release of the exchange players (0x8c6ae0). The selling club is the club of the target's main contract (not of
/// type 5; none reads address 0 below, as the original); the buyer is the offer's `buyer_club_id()`. Without a human
/// manager at the buyer (`exe::news_club_has_human_manager`), nothing. News 0xfad: 0 the seller's id, 1 the buyer's,
/// 2..5 the target (`target_staff_id()`), 6..9 the exchange player `person`, 10 `value`, 11 the offer's `fee()`, 12
/// `reason` (signed word); it goes to the buyer.
/// Original: `TRANSFER_MANAGER::post_exchange_player_removed_news` (0x8de150, transfer_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_exchange_player_removed_news(_this: usize, offer: usize, person: usize, value: u32, reason: u32) {
    let offer = Offer(offer);
    let p = staff_records_staff_by_id(offer.target_staff_id());
    let seller = offer_contract_club(transfer_manager_main_contract_stored(p.addr()), true);
    let buyer = friendly_club_by_id(offer.buyer_club_id());
    if exe::news_club_has_human_manager(read_usize(G_NEWS), buyer) == 0 {
        return;
    }
    covered!("rx05_2 exchange player removed news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfad, 0);
    exe::newsitem_set_slot(n, 0, read_u32(seller));
    exe::newsitem_set_slot(n, 1, read_u32(buyer.addr()));
    write_person_news_slots(n, 2, p);
    write_person_news_slots(n, 6, Staff(person));
    exe::newsitem_set_slot(n, 10, value);
    exe::newsitem_set_slot(n, 11, offer.fee() as u32);
    exe::newsitem_set_slot(n, 12, reason as i16 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, buyer);
}

/// Comparer for `post_exchange_player_removed_news`: the original only (the item goes to the manager's inbox).
pub(crate) unsafe extern "thiscall" fn compare_post_exchange_player_removed_news(this: usize, offer: usize, person: usize, value: u32, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I0%%)(this, offer, person, value, reason) // simplification: original only
}

/// Posts the news that a transfer is put on hold by a nation's transfer deadline, to the human clubs involved.
///
/// `post_offer_on_hold_news(offer, nation)`, from `hold_offers_at_window_close` (0x8c4e10) when the transfer window of
/// the buyer's or the seller's nation shuts. The recipients: `exe::transfer_manager_collect_offer_news_recipients` with
/// 0xf, up to 30; with none, nothing. News 0xfb4: 0..3 the target (`target_staff_id()`), 4 the offer's
/// `buyer_club_id()`, 5 the club of his main contract or −1 (`news_contract_club_id_or_none`), 6 the `fee()`, 7
/// `pre_contract()` (signed), 8 the total value (`exe::transfer_offer_get_total_value` with 0, 0, 0, 1), 9 the
/// nation's id, 10 the `status()`, 11 `slot_index()`, 12 `serial()`, 13 `offer_type()` (signed bytes).
/// Original: `TRANSFER_MANAGER::post_offer_on_hold_news` (0x8e1150, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_offer_on_hold_news(this: usize, offer: usize, nation: usize) {
    let offer = Offer(offer);
    let p = staff_records_staff_by_id(offer.target_staff_id());
    let mut clubs = [0usize; 30];
    let q = exe::transfer_manager_collect_offer_news_recipients(this, offer, 0xf, clubs.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    covered!("rx05_2 offer on hold news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfb4, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, offer.buyer_club_id() as u32);
    exe::newsitem_set_slot(n, 5, news_contract_club_id_or_none(p));
    exe::newsitem_set_slot(n, 6, offer.fee() as u32);
    exe::newsitem_set_slot(n, 7, offer.pre_contract() as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 8, exe::transfer_offer_get_total_value(offer, 0, 0, 0, 1) as u32);
    exe::newsitem_set_slot(n, 9, read_u32(nation));
    exe::newsitem_set_slot(n, 10, offer.status() as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 11, offer.slot_index() as u32);
    exe::newsitem_set_slot(n, 12, offer.serial() as u32);
    exe::newsitem_set_slot(n, 13, offer.offer_type() as i8 as i32 as u32);
    send_news_to_clubs(n, &clubs, q);
}

/// Comparer for `post_offer_on_hold_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_offer_on_hold_news(this: usize, offer: usize, nation: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, offer, nation) // simplification: original only
}

/// Posts the news that a player accepted a loan move until a date, to the human clubs involved.
///
/// `post_loan_accepted_news(offer, terms)`, from `handle_player_accepted_terms` (0x8c2180) when the loan completes at
/// once. The recipients: `exe::transfer_manager_collect_offer_news_recipients` with 0x5f, up to 30; with none,
/// nothing. News 0xfb6: 0..3 the player (`target_staff_id()`), 4 the offer's `buyer_club_id()`, 5 the club of his
/// main contract (`news_contract_club_id`; none reads address 0, as the original), 7 = 0 (slot 6 stays unset), 8 and 9
/// the loan's end (`contract_expires()` signed, `contract_expires_year()` unsigned) from `terms`.
/// Original: `TRANSFER_MANAGER::post_loan_accepted_news` (0x8e1f10, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_loan_accepted_news(this: usize, offer: usize, terms: usize) {
    let offer = Offer(offer);
    let terms = Contract(terms);
    let p = staff_records_staff_by_id(offer.target_staff_id());
    let mut clubs = [0usize; 30];
    let q = exe::transfer_manager_collect_offer_news_recipients(this, offer, 0x5f, clubs.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    covered!("rx05_2 loan accepted news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfb6, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, offer.buyer_club_id() as u32);
    exe::newsitem_set_slot(n, 5, news_contract_club_id(p));
    exe::newsitem_set_slot(n, 7, 0);
    exe::newsitem_set_slot(n, 8, terms.contract_expires() as i32 as u32);
    exe::newsitem_set_slot(n, 9, terms.contract_expires_year() as u16 as u32);
    send_news_to_clubs(n, &clubs, q);
}

/// Comparer for `post_loan_accepted_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_loan_accepted_news(this: usize, offer: usize, terms: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, offer, terms) // simplification: original only
}
