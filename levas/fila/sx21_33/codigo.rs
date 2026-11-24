// ---------- sx21_33: a transfer offer: the empty constructor, the clause total and a player's contract talks status (transfer_offer.cpp) ----------

/// Builds an empty transfer offer: its due date and no lists.
///
/// `TRANSFER_OFFER::TRANSFER_OFFER()`: the Date at `offer::DUE_DATE` (`exe::date_date_54a110`), then the rival offer list
/// (+0) and `offer::EXCHANGE_PLAYERS` (+4) at 0. Returns the offer.
/// Original: `TRANSFER_OFFER::TRANSFER_OFFER` (0x8e5680, transfer_offer.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn construct_transfer_offer(this: usize) -> usize {
    covered!("sx21_33 empty offer");
    exe::date_date_54a110(this + offer::DUE_DATE); // sx21_33: the due date
    write_u32(this, 0); // the rival offer list
    write_u32(this + offer::EXCHANGE_PLAYERS, 0); // sx21_33: no exchange players
    this
}

/// Comparer for `construct_transfer_offer`: side by side over the lists and the due date.
pub(crate) unsafe extern "thiscall" fn compare_construct_transfer_offer(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 8), (this + offer::DUE_DATE, 8)], || o(this), || construct_transfer_offer(this));
    SCOREBOARD.record(equal && ro == rr, || format!("construct_transfer_offer(offer {:#x}) original={:x?} rust={:x?}", this, mo, mr));
    ro
}

/// The sum of an offer's two clause amounts, counting only positive ones.
///
/// `TRANSFER_OFFER::get_clause_total()`: `offer.appearance_fee()` when above 0 plus `offer.international_fee()` when above
/// 0.
/// Original: `TRANSFER_OFFER::get_clause_total` (0x8e7ef0, transfer_offer.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn transfer_offer_get_clause_total(this: usize) -> i32 {
    covered!("sx21_33 clause total");
    let o = Offer(this);
    let (a, b) = (o.appearance_fee() as i32, o.international_fee() as i32);
    (if a > 0 { a } else { 0 }) + if b > 0 { b } else { 0 } // sx21_33: only positive clauses
}

/// Comparer for `transfer_offer_get_clause_total`: the value of both.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_get_clause_total(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%)(this);
    let c = transfer_offer_get_clause_total(this);
    SCOREBOARD.record(r == c, || format!("transfer_offer_get_clause_total(offer {:#x}) original={} rust={}", this, r, c));
    r
}

/// The contract talks status of a player in an offer: the target's, or an exchange player's; 4 when he is not in it.
///
/// `TRANSFER_OFFER::get_player_talks_status(player_id)`: the target (`offer.target_staff_id()`): when the offer is
/// withdrawn (`offer.status()` 3), `offer::TERMS_STATE` becomes 4 (collapsed) first; returns `offer.terms_state()`.
/// Otherwise the exchange player with that id (`offer.exchange_players()`, walked with `exe::llist_get_next` from the
/// head): his `exchange_player::TERMS_STATE`; none: 4.
/// Original: `TRANSFER_OFFER::get_player_talks_status` (0x8e7f10, transfer_offer.cpp, thiscall, ret 4, al).
pub(crate) unsafe extern "thiscall" fn transfer_offer_get_player_talks_status(this: usize, player: u32) -> u32 {
    let o = Offer(this);
    if player as usize == o.target_staff_id() {
        if o.status() == 3 {
            covered!("sx21_33 withdrawn offer talks collapse");
            o.set_terms_state(4); // sx21_33: the talks collapse
        }
        return o.terms_state() as u32;
    }
    let list = o.exchange_players().addr();
    if list != 0 {
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        loop {
            let d = exe::llist_get_next(list);
            if d == 0 {
                break;
            }
            if read_u32(d) == player {
                return read_u8(d + exchange_player::TERMS_STATE) as u32;
            }
        }
    }
    4
}

/// Comparer for `transfer_offer_get_player_talks_status`: the status (al) and the terms byte, side by side.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_get_player_talks_status(this: usize, player: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(this + offer::TERMS_STATE, 1)], || o(this, player), || transfer_offer_get_player_talks_status(this, player));
    SCOREBOARD.record(equal && ro as u8 == rr as u8, || format!("transfer_offer_get_player_talks_status(offer {:#x}, player {}) original={} {:x?} rust={} {:x?}", this, player, ro as u8, mo, rr as u8, mr));
    ro
}
