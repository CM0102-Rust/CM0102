/// Posts the news that an exchange player failed to get a work permit and left the offer, to the human buying and
/// selling clubs.
///
/// `post_exchange_player_permit_refused_news(offer, person, value)`, from `check_offer_work_permits` (0x8c7690). The
/// selling club is the club of the target's main contract (not of type 5, else none); the buyer the offer's
/// `buyer_club_id()`. Without a human manager at the buyer nor at the seller (`exe::news_club_has_human_manager`),
/// nothing. News 0xfbb: 0 the seller's id (none reads address 0, as the original), 1 the buyer's, 2..5 the target
/// (`target_staff_id()`), 6..9 the exchange player `person`, 10 `value`, 11 the offer's `fee()`; it goes to the buyer
/// and, when there is one, to the seller.
/// Original: `TRANSFER_MANAGER::post_exchange_player_permit_refused_news` (0x8e35a0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_exchange_player_permit_refused_news(_this: usize, offer: usize, person: usize, value: u32) {
    let offer = Offer(offer);
    let p = staff_records_staff_by_id(offer.target_staff_id());
    let seller = offer_contract_club(transfer_manager_main_contract_stored(p.addr()), true);
    let buyer = friendly_club_by_id(offer.buyer_club_id());
    let news = read_usize(G_NEWS);
    if exe::news_club_has_human_manager(news, buyer) == 0 && (seller == 0 || exe::news_club_has_human_manager(read_usize(G_NEWS), seller) == 0) {
        return;
    }
    covered!("rx05_4 exchange player permit refused news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfbb, 0);
    exe::newsitem_set_slot(n, 0, read_u32(seller));
    exe::newsitem_set_slot(n, 1, read_u32(buyer.addr()));
    write_person_news_slots(n, 2, p);
    write_person_news_slots(n, 6, Staff(person));
    exe::newsitem_set_slot(n, 10, value);
    exe::newsitem_set_slot(n, 11, offer.fee() as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, buyer);
    if seller != 0 {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, seller);
    }
}

/// Comparer for `post_exchange_player_permit_refused_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_exchange_player_permit_refused_news(this: usize, offer: usize, person: usize, value: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%)(this, offer, person, value) // simplification: original only
}

/// Posts the news that a player's relegation or non-promotion release clause became active.
///
/// `post_release_clause_activated_news(person, competition, kind, contract)`, from the contract manager's clause
/// triggers (0x4cced0, with kind 3). Nothing without a person or a competition, without a main contract
/// (`transfer_manager_main_contract_stored`) or its club, for a retiring player (`exe::player_regen_get_retirement_status`
/// = 1) or one with a future transfer (`exe::transfer_manager_get_future_transfer` with 0). The recipients:
/// `exe::transfer_manager_collect_player_news_recipients` with 0xa, up to 20; with none, nothing. News 0xfbf: 0..3 the
/// person, 4 the id of his contract's club (any contract type; looked up again), 5 the competition's id, 6 `kind`
/// (signed byte), 7 the clause's `release_fee()` from `contract`.
/// Original: `TRANSFER_MANAGER::post_release_clause_activated_news` (0x8e3db0, transfer_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_release_clause_activated_news(this: usize, person: usize, competition: usize, kind: u32, contract: usize) {
    if person == 0 || competition == 0 {
        return;
    }
    let p = Staff(person);
    let c = transfer_manager_main_contract_stored(person);
    if c.is_null() || friendly_club_by_id(c.club()).is_null() {
        return;
    }
    if exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), p) == 1 {
        return;
    }
    if exe::transfer_manager_get_future_transfer(this, p, 0) != 0 {
        return;
    }
    let mut clubs = [0usize; 20];
    let q = exe::transfer_manager_collect_player_news_recipients(this, p, 0xa, clubs.as_mut_ptr(), 0x14) as i8;
    if q == 0 {
        return;
    }
    covered!("rx05_4 release clause news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfbf, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, read_u32(offer_contract_club(transfer_manager_main_contract_stored(person), false)));
    exe::newsitem_set_slot(n, 5, read_u32(competition));
    exe::newsitem_set_slot(n, 6, kind as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 7, Contract(contract).release_fee() as u32);
    send_news_to_clubs(n, &clubs, q);
}

/// Comparer for `post_release_clause_activated_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_release_clause_activated_news(this: usize, person: usize, competition: usize, kind: u32, contract: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(%%I1%%)(this, person, competition, kind, contract) // simplification: original only
}

/// Posts the news that a contract offer is cancelled because the revised work permit was refused.
///
/// `post_contract_permit_refused_news(person, contract_offer)`, from `complete_accepted_contract_offer` (0x8c38b0).
/// News 0xbcc: 0..3 the person, 4 the offering club's id (`contract_offer`'s `.club()`). It goes to the club of the
/// person's main contract (any type; 0 without one) when the contract and its club exist; then, when that club is not
/// the offering one, to the same club again.
///
/// Defect of the original: the second delivery was meant for the offering club (it is tested against it), but it
/// sends to the person's own club again, so the offering club never gets the news and the player's club gets it
/// twice. The copy keeps it.
/// Original: `TRANSFER_MANAGER::post_contract_permit_refused_news` (0x8e4290, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_contract_permit_refused_news(_this: usize, person: usize, contract_offer: usize) {
    let p = Staff(person);
    let offer = Contract(contract_offer);
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbcc, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, offer.club() as u32);
    let has_club = || {
        let c = transfer_manager_main_contract_stored(person);
        !c.is_null() && !friendly_club_by_id(c.club()).is_null()
    };
    if has_club() {
        let club = offer_contract_club(transfer_manager_main_contract_stored(person), false);
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
    if has_club() && read_u32(offer_contract_club(transfer_manager_main_contract_stored(person), false)) != offer.club() as u32 {
        covered!("rx05_4 contract permit refused news sent again");
        let club = offer_contract_club(transfer_manager_main_contract_stored(person), false);
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
}

/// Comparer for `post_contract_permit_refused_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_contract_permit_refused_news(this: usize, person: usize, contract_offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, person, contract_offer) // simplification: original only
}
