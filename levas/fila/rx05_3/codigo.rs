/// Posts the news that a proposed loan was cancelled, with the reason, to the human clubs involved.
///
/// `post_loan_cancelled_news(offer, reason)`, from `handle_player_accepted_terms` (0x8c2180) when the squad is full or
/// the buyer nation's rule refuses the loan. The recipients: `exe::transfer_manager_collect_offer_news_recipients`
/// with 0xf, up to 30; with none, nothing. News 0xfb7: 0..3 the player (`target_staff_id()`), 4 the offer's
/// `buyer_club_id()`, 5 the club of his main contract (`news_contract_club_id`; none reads address 0, as the
/// original), 6 `reason` (signed word), 7 the id of the buyer's `.division()` or −1.
/// Original: `TRANSFER_MANAGER::post_loan_cancelled_news` (0x8e23d0, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_loan_cancelled_news(this: usize, offer: usize, reason: u32) {
    let offer = Offer(offer);
    let p = staff_records_staff_by_id(offer.target_staff_id());
    let buyer = friendly_club_by_id(offer.buyer_club_id());
    let mut clubs = [0usize; 30];
    let q = exe::transfer_manager_collect_offer_news_recipients(this, offer, 0xf, clubs.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    covered!("rx05_3 loan cancelled news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfb7, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, offer.buyer_club_id() as u32);
    exe::newsitem_set_slot(n, 5, news_contract_club_id(p));
    exe::newsitem_set_slot(n, 6, reason as i16 as i32 as u32);
    let division = buyer.division();
    exe::newsitem_set_slot(n, 7, if division.is_null() { u32::MAX } else { read_u32(division.addr()) });
    send_news_to_clubs(n, &clubs, q);
}

/// Comparer for `post_loan_cancelled_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_loan_cancelled_news(this: usize, offer: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, offer, reason) // simplification: original only
}

/// Posts the news that a transfer waits a few weeks for work permit decisions, to the human clubs involved.
///
/// `post_work_permit_delay_news(offer)`, from `advance_offer_to_club_approval` (0x8c2f50) before the offer waits for
/// the permits. The selling club is the club of the target's main contract (not of type 5, else none); the buyer the
/// offer's `buyer_club_id()`. The recipients: `exe::transfer_manager_collect_offer_news_recipients` with 0xf, up to
/// 30; with none, nothing. News 0xfb9: 0..3 the target, 4 the buyer's id, 5 the club of his main contract or −1
/// (`news_contract_club_id_or_none`), 6 whether the target needs a permit at the buyer
/// (`exe::transfer_manager_player_needs_work_permit` with 0), 7 how many exchange players (`exchange_players()`,
/// rewound) need one at the seller, and 8..11 the first of them, when there is one.
/// Original: `TRANSFER_MANAGER::post_work_permit_delay_news` (0x8e2a00, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn post_work_permit_delay_news(this: usize, offer: usize) {
    let offer = Offer(offer);
    let p = staff_records_staff_by_id(offer.target_staff_id());
    let seller = offer_contract_club(transfer_manager_main_contract_stored(p.addr()), true);
    let buyer = friendly_club_by_id(offer.buyer_club_id());
    let mut clubs = [0usize; 30];
    let q = exe::transfer_manager_collect_offer_news_recipients(this, offer, 0xf, clubs.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfb9, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, read_u32(buyer.addr()));
    exe::newsitem_set_slot(n, 5, news_contract_club_id_or_none(p));
    exe::newsitem_set_slot(n, 6, exe::transfer_manager_player_needs_work_permit(this, p, buyer, 0));
    let mut count: i8 = 0;
    let mut first = Staff(0);
    let list = offer.exchange_players();
    if !list.is_null() {
        transfer_manager_llist_rewind(list);
        let mut x = exe::llist_get_next(list);
        while x != 0 {
            let e = staff_records_staff_by_id(read_usize(x));
            if exe::transfer_manager_player_needs_work_permit(this, e, seller, 0) != 0 {
                if first.is_null() {
                    first = e;
                }
                count += 1;
            }
            x = exe::llist_get_next(offer.exchange_players());
        }
    }
    exe::newsitem_set_slot(n, 7, count as i32 as u32);
    if !first.is_null() {
        covered!("rx05_3 exchange player needs a permit");
        write_person_news_slots(n, 8, first);
    }
    send_news_to_clubs(n, &clubs, q);
}

/// Comparer for `post_work_permit_delay_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_work_permit_delay_news(this: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, offer) // simplification: original only
}

/// Posts the news that a player received a work permit for his move to a club, to the human clubs of the offer.
///
/// `post_work_permit_granted_news(person, club, offer)`, from `check_offer_work_permits` (0x8c7690) for the target
/// and for each exchange player. The recipients: `exe::transfer_manager_collect_offer_news_recipients` with 3, up to
/// 30; with none, nothing. News 0xfba: 0..3 the person, 4 the club's id, 5 the club of his main contract or −1
/// (`news_contract_club_id_or_none`).
/// Original: `TRANSFER_MANAGER::post_work_permit_granted_news` (0x8e3130, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_work_permit_granted_news(this: usize, person: usize, club: usize, offer: usize) {
    let mut clubs = [0usize; 30];
    let q = exe::transfer_manager_collect_offer_news_recipients(this, offer, 3, clubs.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    covered!("rx05_3 work permit granted news");
    let p = Staff(person);
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfba, 0);
    write_person_news_slots(n, 0, p);
    exe::newsitem_set_slot(n, 4, read_u32(club));
    exe::newsitem_set_slot(n, 5, news_contract_club_id_or_none(p));
    send_news_to_clubs(n, &clubs, q);
}

/// Comparer for `post_work_permit_granted_news`: the original only (the item goes to the managers' inboxes).
pub(crate) unsafe extern "thiscall" fn compare_post_work_permit_granted_news(this: usize, person: usize, club: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I2%%)(this, person, club, offer) // simplification: original only
}
