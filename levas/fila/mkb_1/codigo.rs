// ---------- batch %%LEVA%%: the AI buyer reviewing its bid after the seller's counter offer (transfer_manager.cpp) ----------

/// The unsettling factor of a player whose AI buyer walked away (the `mood` score of the original, built from the
/// contract's `.grievances()`, `.ambition()`, `.loyalty()`, liking the selling club and `.age()`): above 10 → 0.9,
/// above 8 → 0.95, above 5 → 1.0, above 3 → 1.1, above 1 → 1.25, 1 → 1.5, 0 → 1.75, −2 or −1 → 2.0, below −2 → 2.5
/// (the f32 constants of the original).
unsafe fn review_bid_unsettle_factor(player: Staff, seller: Club, grievances: u32) -> f64 {
    let mut mood: i8 = if grievances != 0 { 5 } else { 0 };
    if grievances & 0x101 != 0 {
        mood = mood.wrapping_add(5);
    }
    let ambition = player.ambition();
    if ambition == 20 {
        mood = mood.wrapping_add(2);
    } else if ambition > 15 {
        mood = mood.wrapping_add(1);
    }
    let loyalty = player.loyalty();
    if loyalty == 1 {
        mood = mood.wrapping_add(3);
    } else if loyalty < 5 {
        mood = mood.wrapping_add(1);
    } else if loyalty == 20 {
        mood = mood.wrapping_sub(3);
    } else if loyalty > 15 {
        mood = mood.wrapping_sub(1);
    }
    if exe::staff_favours_club(player, seller) != 0 {
        mood = mood.wrapping_sub(2);
    } else if exe::staff_dislikes_club(player, seller) != 0 {
        mood = mood.wrapping_add(2);
    }
    let age = player.age() as i8;
    if age < 20 {
        mood = mood.wrapping_sub(4);
    } else if age < 23 {
        mood = mood.wrapping_sub(2);
    }
    let k = if mood > 10 {
        rdata::F32_0_8999999762
    } else if mood > 8 {
        rdata::F32_0_9499999881
    } else if mood > 5 {
        rdata::F32_1
    } else if mood > 3 {
        rdata::F32_1_100000024
    } else if mood > 1 {
        rdata::F32_1_25
    } else if mood == 0 {
        rdata::F32_1_75
    } else if mood < -2 {
        rdata::F32_2_5
    } else if mood < 0 {
        rdata::F32_2
    } else {
        rdata::F32_1_5
    };
    read_f32(k) as f64
}

/// The fee level above which the player hears of the bid: his value `v` for the selling club scaled (v × 8 below
/// 500,000; (v − 500,000) × 2 + 2,000,000 below 2,500,000; (v − 2,500,000) × 1.25 + 6,000,000 above), times the
/// unsettling factor.
unsafe fn review_bid_unsettle_level(v: i32, factor: f64) -> i32 {
    let v_f = v as f64;
    let x = if v < 500_000 {
        v_f * read_f64(rdata::F64_8)
    } else if v < 2_500_000 {
        (v_f - read_f32(0x96b000) as f64) * 2.0 + read_f64(rdata::F64_2000000) // f32 500,000
    } else {
        (v_f - read_f32(0x96aff0) as f64) * read_f64(rdata::F64_1_25) + read_f64(rdata::F64_6000000) // f32 2,500,000
    };
    ftol(factor * ftol(x) as f64)
}

/// The player at the selling club of a bid his AI buyer walked away from may become unsettled: with a contract
/// (`contract_of_staff_cached`, not of type 5), a selling club, a contract with it (`exe::contract_manager_get_contract_for_club`)
/// and a buyer; at the club for more than (`.loyalty()` + 42) × 5 + `.professionalism()` days since
/// `.date_joined_club()` (`exe::date_days_since`); and the buyer's `.reputation()` above the seller's + 1250, or
/// listed by request (`exe::staff_is_listed_by_request`), or contract flag 8 (`exe::staff_contract_get_flags`), or
/// `.grievances()` bits 0x14. The level is the player's value for the seller (`exe::contract_manager_calc_player_value`
/// with 0, −1, human seller) through `review_bid_unsettle_level`; a bid `w` above 17,500,000, or above 1.5 × the level,
/// or above 250,000 and the level with random(`.temperament()` + 1) = 0, sets the contract's event flag 0x800000
/// (`exe::staff_contract_apply_event_flags` with 0).
unsafe fn review_bid_maybe_unsettle(player: Staff, buyer: Club, w: i32) {
    let c = Contract(contract_of_staff_cached(player));
    if c.is_null() || c.contract_type() & 0x3f == 5 {
        return;
    }
    let seller = transfer_manager_club_by_id(c.club());
    if seller.is_null() {
        return;
    }
    let cs = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, seller));
    if cs.is_null() || buyer.is_null() {
        return;
    }
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(player.addr() + staff::DATE_JOINED_CLUB), read_u32(player.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    if days <= (player.loyalty() as i32 + 42) * 5 + player.professionalism() as i32 {
        return;
    }
    if !((seller.reputation() as i32 + 1250) < buyer.reputation() as i32
        || exe::staff_is_listed_by_request(player, 0) != 0
        || exe::staff_contract_get_flags(cs) & 8 != 0
        || cs.grievances() as u8 & 0x14 != 0)
    {
        return;
    }
    let human = exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0;
    let v = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), player, seller, 0, -1, human as u32);
    let level = review_bid_unsettle_level(v, review_bid_unsettle_factor(player, seller, cs.grievances()));
    covered!("mkb_1 player hears of the bid");
    let unsettle = w > 17_500_000 // mkb1: the fee that always unsettles
        || (level as f64 * read_f64(rdata::F64_1_5)) < w as f64
        || (w > 250_000 && w > level && exe::utils_random(player.temperament() as i32 + 1) == 0);
    if unsettle {
        exe::staff_contract_apply_event_flags(cs, 0x800000, 0);
    }
}

/// The AI buyer reviews its bid after the selling club's answer: it values the target from its shortlist slot and,
/// when the price asked (the offer's weighted value plus the asking price of each exchange player kept) is above that
/// value, withdraws (and the target may hear of it); otherwise it tops the fee up with what the dropped exchange
/// players were worth, up to the value, and resubmits the bid if the transfer budget covers it.
///
/// `TRANSFER_MANAGER::review_ai_club_bid(slot, keep_existing)`, from `process_due_offer` (0x8c63a0) for a transfer bid
/// with status `offer_status::ON_HOLD` and an AI buyer (keep_existing 0), from the human manager's holiday answers
/// (0x784100), `handle_player_rejected_terms` (0x8c28c0) and the exchange-player dialog (0x8f2870). With keep_existing
/// 0 the offer reviewed is the seller's counter offer, the negotiation `.negotiation_index()` in
/// `transfer_manager_fields::NEGOTIATIONS` (the first one when the index is not below
/// `transfer_manager_fields::NEGOTIATION_CAPACITY`); with no negotiation (−1) the bid gets status 0x10 and is scheduled
/// (`exe::transfer_manager_schedule_offer` with 1, 0). Otherwise the bid in `slot` itself.
///
/// A local offer (`exe::new_transfer_offer`) takes a copy of it (the due date by `exe::date_operator_3d`; the low
/// nibbles of `offer::COMPLETION_TIMING` and `offer::OFFER_KIND` only), with +0 = 0, `.negotiation_index()` = −1 and
/// its own copy of the exchange players (`exe::new_llist`, `exe::llist_copy_from`). The buyer's shortlist slot for the
/// target (`exe::shortlist_manager_find_club_shortlist_entry` with the bid's `.job_type()` ≠ 11): none, the bid is
/// withdrawn (`exe::transfer_manager_withdraw_transfer_offer` with 0, 2). The value: `exe::shortlist_manager_value_player_for_club`
/// (−1, 1); the price `w`: `exe::transfer_offer_get_weighted_value` (0, 1, 1, 1, 1); each exchange player's asking
/// price (`exe::contract_manager_calc_asking_price` with his contract's club, 0, 1, 1) is added to `w` unless that
/// passes the value, when the player leaves the local offer (`exe::llist_remove`) and his price is counted as dropped;
/// an empty list is freed. The local offer's kind becomes 2 (low byte & 0xf2 | 2).
///
/// `w` above the value: the target's status in the shortlist (`exe::shortlist_manager_update_staff_status` with 1,
/// slot role ≠ 11), the withdrawal and `review_bid_maybe_unsettle`. Otherwise: the fee (`.fee()`) + min(dropped, value −
/// `w`, never negative); the total (`exe::transfer_offer_get_total_value` with 0, 1, 0, 1) above the buyer's transfer
/// budget (`exe::finance_get_transfer_budget` with the target, 0, 0, 1, 0): the status and the withdrawal; within it
/// and keep_existing 0, the local offer is submitted as a new bid (`exe::transfer_manager_add_transfer_offer`). The
/// local offer is destroyed (`exe::drop_transfer_offer`). The original's SEH frame only matters for exceptions.
/// Original: `TRANSFER_MANAGER::review_ai_club_bid` (0x8c0810, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn review_ai_club_bid(this: usize, slot: usize, keep_existing: u32) {
    let mut storage = [0u8; 0x44];
    let local = Offer(storage.as_mut_ptr() as usize);
    exe::new_transfer_offer(local);
    review_ai_club_bid_body(this, slot, keep_existing, local);
    exe::drop_transfer_offer(local);
}

/// The local offer takes a copy of `src` as the original copies it (0x8c0896..0x8c0980).
unsafe fn review_bid_copy_offer(local: Offer, src: usize) {
    let l = local.addr();
    for k in [0usize, 4, 8, 0xc, 0x1b, 0x20, 0x24, 0x28, 0x35, 0x3a] {
        write_u32(l + k, read_u32(src + k));
    }
    for k in [0x10usize, 0x11, 0x1a, 0x1f, 0x2c, 0x2d, 0x2e, 0x2f, 0x39, 0x3e, 0x3f, 0x40] {
        write_u8(l + k, read_u8(src + k));
    }
    let mut scratch = [0u32; 2];
    exe::date_operator_3d(l + offer::DUE_DATE, scratch.as_mut_ptr() as usize, src + offer::DUE_DATE);
    write_u8(l + offer::COMPLETION_TIMING, (read_u8(l + offer::COMPLETION_TIMING) & 0xf0) | (read_u8(src + offer::COMPLETION_TIMING) & 0xf));
    write_u32(l + offer::OFFER_KIND, (read_u32(l + offer::OFFER_KIND) & !0xf) | (read_u32(src + offer::OFFER_KIND) & 0xf));
    write_u32(l, 0);
    write_u32(l + offer::NEGOTIATION_INDEX, 0xffff_ffff);
}

/// The body of `review_ai_club_bid` between the construction and the destruction of the local offer.
unsafe fn review_ai_club_bid_body(this: usize, slot: usize, keep_existing: u32, local: Offer) {
    let mut source = slot;
    if keep_existing == 0 {
        let bid = Offer(read_usize(slot));
        let k = bid.negotiation_index() as i32;
        if k == -1 {
            bid.set_status(0x10);
            exe::transfer_manager_schedule_offer(this, bid, 1, 0);
            return;
        }
        let base = read_usize(this + transfer_manager_fields::NEGOTIATIONS);
        source = if k < read_i32(this + transfer_manager_fields::NEGOTIATION_CAPACITY) { base.wrapping_add((k as usize).wrapping_mul(4)) } else { base };
    }
    let src = read_usize(source);
    review_bid_copy_offer(local, src);
    if !local.exchange_players().is_null() {
        let p = exe::crt_operator_new(0xe);
        let list = if p != 0 { exe::new_llist(p) } else { 0 };
        write_usize(local.addr() + offer::EXCHANGE_PLAYERS, list);
        exe::llist_copy_from(list, Offer(read_usize(source)).exchange_players());
    }
    let src = Offer(read_usize(source));
    let player = transfer_manager_staff_by_id(src.target_staff_id());
    let buyer = transfer_manager_club_by_id(src.buyer_club_id());
    let sm = read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER);
    let job_is_player = Offer(read_usize(slot)).job_type() == 0xb;
    let entry = exe::shortlist_manager_find_club_shortlist_entry(sm, buyer, player, (!job_is_player) as u32, 0);
    if entry == 0 {
        exe::transfer_manager_withdraw_transfer_offer(this, slot as *mut usize, 0, 2);
        return;
    }
    let mut scratch: i32 = 0;
    let value = exe::shortlist_manager_value_player_for_club(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), buyer, player, entry, &mut scratch, -1, 1);
    let mut w = exe::transfer_offer_get_weighted_value(local, 0, 1, 1, 1, 1);
    let mut dropped: i32 = 0;
    let list = local.exchange_players();
    if !list.is_null() {
        list.set_cursor(list.head());
        let mut item = exe::llist_get_next(list);
        while item != 0 {
            let other = transfer_manager_staff_by_id(read_usize(item));
            let c = contract_of_staff_cached(other);
            let club = if c == 0 { 0 } else { transfer_manager_club_by_id(read_usize(c + contract::CLUB)).addr() };
            let ask = exe::contract_manager_calc_asking_price(read_usize(G_CONTRACT_MANAGER), buyer, club, other, 0, 1, 1);
            if ask.wrapping_add(w) > value {
                covered!("mkb_1 exchange player dropped");
                dropped = dropped.wrapping_add(ask);
                exe::llist_remove(list, item);
            } else {
                w = ask.wrapping_add(w);
            }
            item = exe::llist_get_next(local.exchange_players());
        }
        let list = local.exchange_players();
        if read_i16(list.addr()) == 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(local.addr() + offer::EXCHANGE_PLAYERS, 0);
        }
    }
    write_u8(local.addr() + offer::OFFER_KIND, (read_u8(local.addr() + offer::OFFER_KIND) & 0xf2) | 2);
    let role_not_player = (read_u8(entry) != 0xb) as u32;
    if w > value {
        // mkb1: the price above the buyer's value of the target
        covered!("mkb_1 buyer walks away");
        exe::shortlist_manager_update_staff_status(sm, player, buyer, 1, role_not_player);
        exe::transfer_manager_withdraw_transfer_offer(this, slot as *mut usize, 0, 2);
        review_bid_maybe_unsettle(player, buyer, w);
        return;
    }
    let room = value.wrapping_sub(w);
    if dropped >= room {
        dropped = room;
    }
    if dropped < 0 {
        dropped = 0;
    }
    write_u32(local.addr() + offer::FEE, read_u32(local.addr() + offer::FEE).wrapping_add(dropped as u32)); // mkb1: the fee topped up
    let total = exe::transfer_offer_get_total_value(local, 0, 1, 0, 1);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), buyer);
    let budget = exe::finance_get_transfer_budget(finance, player.addr() as u32, 0, 0usize, 1, 0);
    if budget < total as i64 {
        // mkb1: the total above the transfer budget
        covered!("mkb_1 over budget");
        exe::shortlist_manager_update_staff_status(sm, player, buyer, 1, role_not_player);
        exe::transfer_manager_withdraw_transfer_offer(this, slot as *mut usize, 0, 2);
    } else if keep_existing == 0 {
        covered!("mkb_1 bid resubmitted");
        exe::transfer_manager_add_transfer_offer(this, local.addr() as *mut u8, core::ptr::null_mut());
    }
}

/// Comparer for `review_ai_club_bid`: the original only (it withdraws and submits offers, moves shortlist slots and
/// may unsettle a contract; it cannot run twice).
pub(crate) unsafe extern "thiscall" fn compare_review_ai_club_bid(this: usize, slot: usize, keep_existing: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, slot, keep_existing) // simplification: original only
}
