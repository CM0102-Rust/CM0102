// ---------- sx21_36: a transfer offer from the offer screens: building a bid, comparing two offers' terms and taking an exchange player out (transfer_offer.cpp) ----------

/// Builds a transfer offer as a bid (the offer screens).
///
/// `TRANSFER_OFFER::TRANSFER_OFFER(a1..a5)`: the Date at `offer::DUE_DATE` (`exe::date_date_54a110`), then
/// `exe::transfer_offer_build_bid` with the five arguments and a last 0. Returns the offer.
/// Original: `TRANSFER_OFFER::TRANSFER_OFFER` (0x8e56a0, transfer_offer.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_transfer_offer_bid(this: usize, a1: usize, a2: usize, a3: usize, a4: u32, a5: u32) -> usize {
    covered!("sx21_36 bid offer built");
    exe::date_date_54a110(this + offer::DUE_DATE);
    exe::transfer_offer_build_bid(this, a1, a2, a3, a4, a5, core::ptr::null_mut()); // sx21_36: no terms out
    this
}

/// Comparer for `construct_transfer_offer_bid`: only the original (the bid draws and builds a contract offer).
pub(crate) unsafe extern "thiscall" fn compare_construct_transfer_offer_bid(this: usize, a1: usize, a2: usize, a3: usize, a4: u32, a5: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> usize>(%%I0%%)(this, a1, a2, a3, a4, a5)
}

/// Whether two offers have the same terms: the fee, the clauses, the sell-on share, the instalments, the completion
/// timing and the exchange players.
///
/// `TRANSFER_OFFER::has_same_terms(other)`, from the offer screens. The amounts are compared as displayed
/// (`exe::cash_get_display_amount` with flags 4). The fees differ: 0. The appearance clause (amount or games) differs and
/// either amount is positive: 0; the same for the international clause. The `offer.sell_on_percent()` differs and
/// either is positive: 0; the same for `offer.instalments()`. The low nibble of `offer.completion_timing()` differs: 0.
/// Exchange players: neither has a list, 1; only one has a list, or the counts differ, 0; each of the other offer's
/// players (walked with `exe::llist_get_next`) must be in this offer's list (searched from its head each time), else
/// 0; otherwise 1.
/// Original: `TRANSFER_OFFER::has_same_terms` (0x8e7f70, transfer_offer.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_offer_has_same_terms(this: usize, other: usize) -> u32 {
    let (a, b) = (Offer(this), Offer(other));
    let shown = |v: usize| {
        let cash = v as u32;
        exe::cash_get_display_amount(&cash as *const u32, 4)
    };
    if shown(b.fee()) != shown(a.fee()) {
        return 0;
    }
    if (shown(b.appearance_fee()) != shown(a.appearance_fee()) || b.appearance_games() != a.appearance_games())
        && ((b.appearance_fee() as i32) > 0 || (a.appearance_fee() as i32) > 0) {
        return 0;
    }
    if (shown(b.international_fee()) != shown(a.international_fee()) || b.international_games() != a.international_games())
        && ((b.international_fee() as i32) > 0 || (a.international_fee() as i32) > 0) {
        return 0;
    }
    let differs = |x: u8, y: u8| x != y && ((x as i8) > 0 || (y as i8) > 0);
    if differs(b.sell_on_percent(), a.sell_on_percent()) || differs(b.instalments(), a.instalments()) {
        return 0;
    }
    if (b.completion_timing() ^ a.completion_timing()) & 0xf != 0 { // sx21_36: the completion timing
        return 0;
    }
    let (lb, la) = (b.exchange_players().addr(), a.exchange_players().addr());
    if lb == 0 || la == 0 {
        return (lb == 0 && la == 0) as u32;
    }
    if read_u16(lb + llist::COUNT) != read_u16(la + llist::COUNT) {
        return 0;
    }
    write_u32(lb + llist::CURSOR, read_u32(lb + llist::HEAD));
    loop {
        let x = exe::llist_get_next(lb);
        if x == 0 {
            break;
        }
        write_u32(la + llist::CURSOR, read_u32(la + llist::HEAD));
        loop {
            let y = exe::llist_get_next(la);
            if y == 0 {
                return 0;
            }
            if read_u32(x) == read_u32(y) {
                break;
            }
        }
    }
    covered!("sx21_36 same terms");
    1
}

/// Comparer for `transfer_offer_has_same_terms`: the answer of both.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_has_same_terms(this: usize, other: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, other);
    let c = transfer_offer_has_same_terms(this, other);
    SCOREBOARD.record(r == c, || format!("transfer_offer_has_same_terms(offer {:#x}, other {:#x}) original={} rust={}", this, other, r, c));
    r
}

/// Takes a player out of an offer's exchange list: the fee grows by his value and his count of offers he is part of
/// goes down.
///
/// `TRANSFER_OFFER::remove_exchange_player(player)`. Without an exchange list: the transfer_offer.cpp line 0x677 box.
/// Otherwise each entry with his id is taken out (`exe::llist_remove`, the walk going on with `exe::llist_get_next`);
/// an emptied list is cleared and freed (`exe::llist_clear`, `exe::crt_operator_delete`) and `offer::EXCHANGE_PLAYERS`
/// set to 0. The fee (`offer::FEE`) grows by his valuation (`exe::contract_manager_calc_player_value(.., player, his
/// club, 0, -1, 0)`, his club the club of his contract, kept in `G_CONTRACT_LOOKUP_RESULT`), and byte +2 of his 8-byte
/// record in the transfer manager's `transfer_manager_fields::PERSON_FLAGS` [inference: the offers he is an exchange
/// player in] goes down by one.
/// Original: `TRANSFER_OFFER::remove_exchange_player` (0x8e81a0, transfer_offer.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_offer_remove_exchange_player(this: usize, player: usize) {
    let o = Offer(this);
    let list = o.exchange_players().addr();
    if list == 0 {
        show_error_at(0xa9e318, 0x677);
        return;
    }
    covered!("sx21_36 exchange player removed");
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    loop {
        let d = exe::llist_get_next(list);
        if d == 0 {
            break;
        }
        if read_u32(d) == read_u32(player) {
            exe::llist_remove(list, d);
        }
    }
    let list = o.exchange_players().addr();
    if read_u16(list + llist::COUNT) == 0 {
        exe::llist_clear(list);
        exe::crt_operator_delete(list);
        write_u32(this + offer::EXCHANGE_PLAYERS, 0);
    }
    let club = game_club_of_contract(game_get_contract_and_store(player));
    let value = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), player, club.addr(), 0, -1, 0);
    o.set_fee((o.fee() as i32).wrapping_add(value) as usize); // sx21_36: his value goes into the fee
    let count = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::PERSON_FLAGS) + read_usize(player) * 8 + 2;
    write_u8(count, read_u8(count).wrapping_sub(1));
}

/// Comparer for `transfer_offer_remove_exchange_player`: only the original (it frees the list); the fee and the count
/// are left to the career.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_remove_exchange_player(this: usize, player: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, player)
}
