// ---------- sx21_27: Japan's signing and offer checks: five foreign players at most (japan_rules.cpp) ----------

/// Counts the foreign players of a squad on top of `start`, for the Japanese limit: true as soon as the count passes 5.
/// The 50 slots of `club.squad()`; a player with an id below `G_STAFF_COUNT` foreign to the rule set's nation
/// (`exe::database_is_staff_foreign(p, nation, 1)`) adds one.
unsafe fn japan_squad_passes_five(club: Club, nation: usize, start: i8) -> bool {
    let mut count = start;
    for k in 0..50usize {
        let p = club.squad(k);
        if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) && exe::database_is_staff_foreign(p, nation, 1) != 0 {
            count = count.wrapping_add(1);
            if count > 5 { // sx21_27: five foreign players
                return true;
            }
        }
    }
    false
}

/// Whether a club may sign a player under the Japanese rules: outside the windows no (9); a foreign player that would
/// take the squad past five foreign players no (0x11); otherwise the base rules.
///
/// `JAPAN_RULES::can_sign_player(club, player, seller, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER` of Japan. No window open on `date` (`exe::transfer_rules_find_open_window`):
/// reason 9. A player foreign to the rule set's nation (`transfer_rules::NATION_ID`): counting him and the squad's
/// foreign players (`japan_squad_passes_five` from 1), above 5: reason 0x11. Otherwise the base answer
/// (`transfer_rules_can_sign_player`).
/// Original: `JAPAN_RULES::can_sign_player` (0x666d40, japan_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn japan_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        if !reason.is_null() {
            *reason = 9;
        }
        return 0;
    }
    let nation = read_usize(this + transfer_rules::NATION_ID);
    if player != 0 && exe::database_is_staff_foreign(player, nation, 1) != 0 && japan_squad_passes_five(Club(club), nation, 1) {
        covered!("sx21_27 japan sixth foreigner refused");
        if !reason.is_null() {
            *reason = 0x11;
        }
        return 0;
    }
    transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason)
}

/// Comparer for `japan_rules_can_sign_player` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_japan_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I0%%);
    compare_rule_check("japan_rules_can_sign_player", reason, || o(this, club, player, seller, date, reason), || japan_rules_can_sign_player(this, club, player, seller, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}, seller {:#x}", this, club, player, seller))
}

/// Whether a club may take part in an offer under the Japanese rules: outside the windows no (9); a move that would
/// leave the buying or the selling club with more than five foreign players no (0x11); otherwise the base rules.
///
/// `JAPAN_RULES::can_make_offer(offer, club, date, reason)`, the vtable slot `transfer_rules_vtable::CAN_MAKE_OFFER` of
/// Japan. The target (`staff_at(offer.target_staff_id())`), his club (the club of his contract, kept in
/// `G_CONTRACT_LOOKUP_RESULT`), the buyer (`club_at(offer.buyer_club_id())`) and the exchange players foreign to the
/// rule set's nation (counted always, walked with `exe::llist_get_next` from the head). The buyer: no window open
/// (`exe::transfer_rules_find_open_window`): reason 9; a foreign target: 1 less the exchange count, when positive,
/// counted with the buyer's squad (`japan_squad_passes_five`): above 5, reason 0x11. The seller without exchange
/// players: the base answer. The seller with them: no window open: reason 9; the exchange count, less one when the
/// target is not foreign, when positive, counted with the target's club squad: above 5, reason 0x11. Otherwise the base
/// answer (`transfer_rules_can_make_offer`).
/// Defect of the original: for the seller, one is taken off when the target is not foreign; the target leaving is a
/// foreigner less only when he is foreign.
/// Original: `JAPAN_RULES::can_make_offer` (0x666e20, japan_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn japan_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = Offer(offer);
    let nation = read_usize(this + transfer_rules::NATION_ID);
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
            if exe::database_is_staff_foreign(staff_at(read_i32(d) as isize).addr(), nation, 1) != 0 {
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
        if exe::database_is_staff_foreign(target, nation, 1) != 0 {
            let start = 1i8.wrapping_sub(count);
            if start > 0 && japan_squad_passes_five(buyer, nation, start) {
                covered!("sx21_27 japan buyer over five");
                return refuse(0x11); // sx21_27: the buyer over five foreigners
            }
        }
    } else if list != 0 {
        if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
            return refuse(9);
        }
        let foreign = exe::database_is_staff_foreign(target, nation, 1) != 0;
        let start = count.wrapping_sub(if foreign { 0 } else { 1 });
        if start > 0 && japan_squad_passes_five(seller, nation, start) {
            covered!("sx21_27 japan seller over five");
            return refuse(0x11);
        }
    }
    transfer_rules_can_make_offer(this, offer, club, date, reason)
}

/// Comparer for `japan_rules_can_make_offer` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_japan_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I1%%);
    compare_rule_check("japan_rules_can_make_offer", reason, || o(this, offer, club, date, reason), || japan_rules_can_make_offer(this, offer, club, date, reason), || format!("rules {:#x}, offer {:#x}, club {:#x}", this, offer, club))
}
