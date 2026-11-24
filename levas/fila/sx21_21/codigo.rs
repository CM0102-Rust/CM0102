// ---------- sx21_21: France's offer check and Italy's signing and offer checks (france_rules.cpp, italy_rules.cpp) ----------

/// Whether `date` is inside window `k` of the rule set (`exe::transfer_window_contains_date` with the 12-byte entry `k`
/// of the table at +4).
pub(crate) unsafe fn date_in_window(this: usize, date: usize, k: usize) -> bool {
    exe::transfer_window_contains_date(this, date, read_usize(this + 4) + k * 12) != 0
}

/// Whether a club may take part in an offer under the French rules: in the restricted second window, a buyer with a
/// purchase (0x10) or a free signing (0xf) already made, or a seller whose exchange players would pass one purchase
/// (0x10), may not; otherwise the base rules.
///
/// `FRANCE_RULES::can_make_offer(offer, club, date, reason)`, the vtable slot `transfer_rules_vtable::CAN_MAKE_OFFER` of
/// France. A selling club (not `offer.buyer_club_id()`) without exchange players (`offer.exchange_players()`): the base
/// answer (`transfer_rules_can_make_offer`). No window open on `date` (`exe::transfer_rules_find_open_window`): reason 9
/// and 0. In the window numbered 1, with the club's counter entry (`season_loans_entry`): the buyer with
/// `offer.pre_contract()` [inference: a free transfer] and byte 1 plus one above 1: reason 0xf; the buyer otherwise with
/// byte 0 plus one above 1: reason 0x10; the seller with the exchange list's count (its low byte) plus byte 0 above 1:
/// reason 0x10. Otherwise the base answer.
/// Original: `FRANCE_RULES::can_make_offer` (0x5c1930, france_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn france_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = Offer(offer);
    let buyer = club_at(o.buyer_club_id() as isize).addr();
    let list = o.exchange_players().addr();
    if club != buyer && list == 0 {
        return transfer_rules_can_make_offer(this, offer, club, date, reason);
    }
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        if !reason.is_null() {
            *reason = 9;
        }
        return 0;
    }
    if read_u8(window + 1) == 1 {
        let entry = season_loans_entry(Club(club));
        let (count, code) = if club != buyer {
            (read_u8(list) as i8 as i32 + read_u8(entry) as i8 as i32, 0x10)
        } else if o.pre_contract() != 0 {
            (read_u8(entry + 1) as i8 as i32 + 1, 0xf)
        } else {
            (read_u8(entry) as i8 as i32 + 1, 0x10)
        };
        if count > 1 { // sx21_21: one of each in the restricted window
            covered!("sx21_21 france offer over the window count");
            if !reason.is_null() {
                *reason = code;
            }
            return 0;
        }
    }
    transfer_rules_can_make_offer(this, offer, club, date, reason)
}

/// Comparer for `france_rules_can_make_offer` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_france_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I0%%);
    compare_rule_check("france_rules_can_make_offer", reason, || o(this, offer, club, date, reason), || france_rules_can_make_offer(this, offer, club, date, reason), || format!("rules {:#x}, offer {:#x}, club {:#x}", this, offer, club))
}

/// Whether a club may sign a player under the Italian rules: the base rules in the first two windows; in the third
/// only a free agent or a player leaving his club on a free (Bosman); outside the windows no (9).
///
/// `ITALY_RULES::can_sign_player(club, player, seller, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER` of Italy. No window open on `date` (`exe::transfer_rules_find_open_window`):
/// reason 9. Inside window 0 or 1 (`date_in_window`): the base answer (`transfer_rules_can_sign_player`). Not inside
/// window 2 either: reason 9. Inside window 2: with a seller, a player without a contract
/// (`exe::contract_manager_get_contract`) or whose contract is not `leaving_on_bosman()`: reason 9; otherwise the base
/// answer.
/// Original: `ITALY_RULES::can_sign_player` (0x661520, italy_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn italy_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let refuse = || {
        if !reason.is_null() {
            *reason = 9;
        }
        0
    };
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        return refuse();
    }
    if date_in_window(this, date, 0) || date_in_window(this, date, 1) {
        return transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason);
    }
    if !date_in_window(this, date, 2) {
        return refuse();
    }
    let contract = || exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    if seller != 0 && (contract() == 0 || Contract(contract()).leaving_on_bosman() == 0) {
        covered!("sx21_21 italy third window not on a free");
        return refuse(); // sx21_21: the third window is for free transfers only
    }
    transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason)
}

/// Comparer for `italy_rules_can_sign_player` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_italy_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I1%%);
    compare_rule_check("italy_rules_can_sign_player", reason, || o(this, club, player, seller, date, reason), || italy_rules_can_sign_player(this, club, player, seller, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}, seller {:#x}", this, club, player, seller))
}

/// Whether a club may take part in an offer under the Italian rules: the buyer only inside a window, and in the third
/// only with a pre-contract (a free transfer); a seller with exchange players only inside a window; otherwise the base
/// rules.
///
/// `ITALY_RULES::can_make_offer(offer, club, date, reason)`, the vtable slot `transfer_rules_vtable::CAN_MAKE_OFFER` of
/// Italy. A selling club (not `offer.buyer_club_id()`) without exchange players: the base answer
/// (`transfer_rules_can_make_offer`). Otherwise no window open on `date` (`exe::transfer_rules_find_open_window`): reason
/// 9. Inside window 0 or 1 (`date_in_window`): the base answer. Not inside window 2 either: reason 9. Inside window 2:
/// the buyer without `offer.pre_contract()`: reason 9; otherwise the base answer.
/// Original: `ITALY_RULES::can_make_offer` (0x661630, italy_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn italy_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = Offer(offer);
    let buyer = club == club_at(o.buyer_club_id() as isize).addr();
    if !buyer && o.exchange_players().addr() == 0 {
        return transfer_rules_can_make_offer(this, offer, club, date, reason);
    }
    let refuse = || {
        if !reason.is_null() {
            *reason = 9;
        }
        0
    };
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        return refuse();
    }
    if date_in_window(this, date, 0) || date_in_window(this, date, 1) {
        return transfer_rules_can_make_offer(this, offer, club, date, reason);
    }
    if !date_in_window(this, date, 2) {
        return refuse();
    }
    if buyer && o.pre_contract() == 0 {
        covered!("sx21_21 italy third window bid not on a free");
        return refuse(); // sx21_21: third window bids only on a free
    }
    transfer_rules_can_make_offer(this, offer, club, date, reason)
}

/// Comparer for `italy_rules_can_make_offer` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_italy_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    compare_rule_check("italy_rules_can_make_offer", reason, || o(this, offer, club, date, reason), || italy_rules_can_make_offer(this, offer, club, date, reason), || format!("rules {:#x}, offer {:#x}, club {:#x}", this, offer, club))
}
