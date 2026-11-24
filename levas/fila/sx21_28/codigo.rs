// ---------- sx21_28: Portugal's signing and offer checks: the non-EU limit by division (portugal_rules.cpp) ----------

/// The Portuguese non-EU limit of a club by its division: 6 in the Premier Division
/// (`KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION`), 5 in the Second Division (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION`),
/// 4 elsewhere or without a division.
unsafe fn portugal_non_eu_limit(club: Club) -> i8 {
    let d = club.division().addr();
    if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION) {
        6 // sx21_28: six in the Premier Division
    } else if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION) {
        5
    } else {
        4
    }
}

/// Whether a person is a non-EU foreigner for a club of `nation` (`exe::database_is_staff_foreign(p, nation, 1)` and
/// not `exe::database_is_staff_eu_national(p, 0)`).
unsafe fn portugal_non_eu(p: usize, nation: usize) -> bool {
    exe::database_is_staff_foreign(p, nation, 1) != 0 && exe::database_is_staff_eu_national(p, 0usize) == 0
}

/// Counts a squad on top of `start` (the 50 slots of `club.squad()`, ids below `G_STAFF_COUNT`, those for whom
/// `counted` holds): true as soon as the count passes `limit`.
unsafe fn portugal_squad_passes(club: Club, start: i8, limit: i8, counted: impl Fn(usize) -> bool) -> bool {
    let mut count = start;
    for k in 0..50usize {
        let p = club.squad(k);
        if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) && counted(p) {
            count = count.wrapping_add(1);
            if count > limit {
                return true;
            }
        }
    }
    false
}

/// Whether a club may sign a player under the Portuguese rules: outside the windows no (9); a non-EU foreigner that
/// would take the squad past the division's non-EU limit no (0x15); otherwise the base rules.
///
/// `PORTUGAL_RULES::can_sign_player(club, player, seller, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER` of Portugal. No window open on `date` (`exe::transfer_rules_find_open_window`):
/// reason 9. A player who is a non-EU foreigner for the club's `nation()` (`portugal_non_eu`): counting him and the
/// squad's non-EU foreigners (from 1), above the limit (`portugal_non_eu_limit`): reason 0x15. Otherwise the base
/// answer (`transfer_rules_can_sign_player`).
/// Original: `PORTUGAL_RULES::can_sign_player` (0x7d3ef0, portugal_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn portugal_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        if !reason.is_null() {
            *reason = 9;
        }
        return 0;
    }
    let c = Club(club);
    let limit = portugal_non_eu_limit(c);
    if player != 0 && portugal_non_eu(player, c.nation().addr()) && portugal_squad_passes(c, 1, limit, |p| portugal_non_eu(p, c.nation().addr())) {
        covered!("sx21_28 portugal non-EU signing refused");
        if !reason.is_null() {
            *reason = 0x15; // sx21_28: over the non-EU limit
        }
        return 0;
    }
    transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason)
}

/// Comparer for `portugal_rules_can_sign_player` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_portugal_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I0%%);
    compare_rule_check("portugal_rules_can_sign_player", reason, || o(this, club, player, seller, date, reason), || portugal_rules_can_sign_player(this, club, player, seller, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}, seller {:#x}", this, club, player, seller))
}

/// Whether a club may take part in an offer under the Portuguese rules: outside the windows no (9); a move that would
/// pass the non-EU limit no (0x15); otherwise the base rules.
///
/// `PORTUGAL_RULES::can_make_offer(offer, club, date, reason)`, the vtable slot `transfer_rules_vtable::CAN_MAKE_OFFER` of
/// Portugal. The target (`staff_at(offer.target_staff_id())`), his club (the club of his contract, kept in
/// `G_CONTRACT_LOOKUP_RESULT`), the buyer (`club_at(offer.buyer_club_id())`) and the exchange players that are non-EU
/// foreigners for the club's `nation()` (counted always, walked with `exe::llist_get_next` from the head). The buyer:
/// no window open: reason 9; a non-EU target: 1 less the exchange count, when positive, counted with the buyer's non-EU
/// squad players against the buyer's limit (`portugal_non_eu_limit`): above it, reason 0x15. The seller without exchange
/// players: the base answer. The seller with them: no window open: reason 9; the exchange count, less one when the
/// target is a non-EU foreigner for his club's nation, when positive, counted against the buyer's limit with one more
/// for each real player of the target's club squad while the target is a non-EU foreigner there: above it, reason
/// 0x15. Otherwise the base answer (`transfer_rules_can_make_offer`).
/// Defects of the original: for the seller, the squad loop tests the target instead of each squad player, and the
/// limit is the buyer's division's.
/// Original: `PORTUGAL_RULES::can_make_offer` (0x7d4040, portugal_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn portugal_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = Offer(offer);
    let target = staff_at(o.target_staff_id() as isize).addr();
    let seller = game_club_of_contract(game_get_contract_and_store(target));
    let buyer = club_at(o.buyer_club_id() as isize);
    let list = o.exchange_players().addr();
    let mut count: i8 = 0;
    if list != 0 {
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        loop {
            let d = exe::llist_get_next(list);
            if d == 0 {
                break;
            }
            if portugal_non_eu(staff_at(read_i32(d) as isize).addr(), Club(club).nation().addr()) {
                count = count.wrapping_add(1);
            }
        }
    }
    let refuse = |code: u16| {
        if !reason.is_null() {
            *reason = code;
        }
        0
    };
    let mut window = 0usize;
    if club == buyer.addr() {
        if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
            return refuse(9);
        }
        if portugal_non_eu(target, Club(club).nation().addr()) {
            let limit = portugal_non_eu_limit(buyer);
            let start = 1i8.wrapping_sub(count);
            if start > 0 && portugal_squad_passes(buyer, start, limit, |p| portugal_non_eu(p, Club(club).nation().addr())) {
                covered!("sx21_28 portugal buyer over the non-EU limit");
                return refuse(0x15); // sx21_28: the buyer over the non-EU limit
            }
        }
    } else if list != 0 {
        if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
            return refuse(9);
        }
        let mut start = count;
        if portugal_non_eu(target, seller.nation().addr()) {
            start = start.wrapping_sub(1);
        }
        if start > 0 {
            let limit = portugal_non_eu_limit(buyer);
            if portugal_squad_passes(seller, start, limit, |_| portugal_non_eu(target, seller.nation().addr())) {
                covered!("sx21_28 portugal seller over the non-EU limit");
                return refuse(0x15);
            }
        }
    }
    transfer_rules_can_make_offer(this, offer, club, date, reason)
}

/// Comparer for `portugal_rules_can_make_offer` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_portugal_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I1%%);
    compare_rule_check("portugal_rules_can_make_offer", reason, || o(this, offer, club, date, reason), || portugal_rules_can_make_offer(this, offer, club, date, reason), || format!("rules {:#x}, offer {:#x}, club {:#x}", this, offer, club))
}
