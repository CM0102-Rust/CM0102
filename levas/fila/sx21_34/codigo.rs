// ---------- sx21_34: a transfer offer: finding an exchange player, the total worth and the destructor (transfer_offer.cpp) ----------

/// The exchange list entry of an offer for a player, or 0 when he is not part of it.
///
/// `TRANSFER_OFFER::find_exchange_player(player)`: the entries of `offer.exchange_players()` (walked with
/// `exe::llist_get_next` from the head) whose id (+0) is the player's (`[player]`); none or no list: 0.
/// Original: `TRANSFER_OFFER::find_exchange_player` (0x8e82f0, transfer_offer.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_offer_find_exchange_player(this: usize, player: usize) -> usize {
    let list = Offer(this).exchange_players().addr();
    if list == 0 {
        return 0;
    }
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    loop {
        let d = exe::llist_get_next(list);
        if d == 0 {
            return 0;
        }
        if read_u32(d) == read_u32(player) { // sx21_34: the entry of this player
            covered!("sx21_34 exchange player found");
            return d;
        }
    }
}

/// Comparer for `transfer_offer_find_exchange_player`: the entry of both.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_find_exchange_player(this: usize, player: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, player);
    let c = transfer_offer_find_exchange_player(this, player);
    SCOREBOARD.record(r == c, || format!("transfer_offer_find_exchange_player(offer {:#x}, player {:#x}) original={:#x} rust={:#x}", this, player, r, c));
    r
}

/// An offer's total worth: the fee, the clauses, the sell-on share and the exchange players.
///
/// `TRANSFER_OFFER::get_total_value(fee_only, no_exchange, valued, with_sell_on)`: the fee (`offer.fee()`); without
/// `fee_only`, the positive clauses (`offer.appearance_fee()`, `offer.international_fee()`) and, with `with_sell_on` and a
/// positive `offer.sell_on_percent()`, that share of the target's `staff::VALUE` (divided by 100, toward zero). Without
/// `no_exchange`, each exchange player (`offer.exchange_players()`, from the head): with `valued`, the contract manager's
/// valuation (`exe::contract_manager_calc_player_value(.., target, 0, 0, -1, 0)`); otherwise his `staff::VALUE`.
/// Defect of the original: with `valued`, each exchange player adds the valuation of the target, not his own.
/// Original: `TRANSFER_OFFER::get_total_value` (0x8e7a70, transfer_offer.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn transfer_offer_get_total_value(this: usize, fee_only: u32, no_exchange: u32, valued: u32, with_sell_on: u32) -> i32 {
    let o = Offer(this);
    let target = staff_at(o.target_staff_id() as isize);
    let mut total = o.fee() as i32;
    if fee_only == 0 {
        let (a, b) = (o.appearance_fee() as i32, o.international_fee() as i32);
        if a > 0 {
            total = total.wrapping_add(a);
        }
        if b > 0 {
            total = total.wrapping_add(b);
        }
        let share = o.sell_on_percent() as i8;
        if share > 0 && with_sell_on != 0 {
            covered!("sx21_34 sell-on share");
            total = total.wrapping_add(read_i32(target.addr() + staff::VALUE).wrapping_mul(share as i32) / 100); // sx21_34: the sell-on share
        }
    }
    let list = o.exchange_players().addr();
    if list != 0 && no_exchange == 0 {
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        loop {
            let d = exe::llist_get_next(list);
            if d == 0 {
                break;
            }
            covered!("sx21_34 exchange player worth");
            total = total.wrapping_add(if valued != 0 {
                exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), target.addr(), 0usize, 0, -1, 0)
            } else {
                read_i32(staff_at(read_i32(d) as isize).addr() + staff::VALUE)
            });
        }
    }
    total
}

/// Comparer for `transfer_offer_get_total_value`: the value of both.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_get_total_value(this: usize, fee_only: u32, no_exchange: u32, valued: u32, with_sell_on: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32>(%%I1%%);
    let (r, c, _, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, fee_only, no_exchange, valued, with_sell_on), || transfer_offer_get_total_value(this, fee_only, no_exchange, valued, with_sell_on));
    SCOREBOARD.record(r == c, || format!("transfer_offer_get_total_value(offer {:#x}, {}, {}, {}, {}) original={} rust={}", this, fee_only, no_exchange, valued, with_sell_on, r, c));
    r
}

/// Destroys a transfer offer: it leaves the rival offer list shared by the player's offers (freeing it when it was the
/// last one) and frees its exchange player list.
///
/// `TRANSFER_OFFER::~TRANSFER_OFFER()`: the rival list (+0): with one entry, emptied (`exe::llist_clear`) and freed
/// (`exe::crt_operator_delete`); otherwise the offer's slot (`offer::SLOT_INDEX`) is taken out of it
/// (`exe::llist_remove`). Then `offer.exchange_players()`, when present, emptied and freed.
/// Original: `TRANSFER_OFFER::~TRANSFER_OFFER` (0x8e5710, transfer_offer.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn destroy_transfer_offer(this: usize) {
    covered!("sx21_34 offer destroyed");
    let rivals = read_usize(this);
    if rivals != 0 {
        if read_u16(rivals + llist::COUNT) == 1 {
            exe::llist_clear(rivals);
            exe::crt_operator_delete(rivals);
        } else {
            exe::llist_remove(rivals, this + offer::SLOT_INDEX);
        }
    }
    let list = read_usize(this + offer::EXCHANGE_PLAYERS);
    if list != 0 {
        exe::llist_clear(list);
        exe::crt_operator_delete(list);
    }
}

/// Comparer for `destroy_transfer_offer`: only the original (it frees memory).
pub(crate) unsafe extern "thiscall" fn compare_destroy_transfer_offer(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
