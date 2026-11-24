// ---------- sx21_26: Spain's signing and offer checks (spain_rules.cpp) ----------

/// Whether a Spanish club's division keeps it out of the non-EU restriction: true without a division or in the First
/// or Second Division (`KEY_CLUB_COMP_SPANISH_FIRST_DIVISION`, `KEY_CLUB_COMP_SPANISH_SECOND_DIVISION`).
unsafe fn spanish_top_or_no_division(club: Club) -> bool {
    let d = club.division().addr();
    d == 0 || read_u32(d) == read_u32(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) || read_u32(d) == read_u32(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION)
}

/// Whether a person is a non-EU foreigner for a club of `nation`: foreign there (`exe::database_is_staff_foreign(p,
/// nation, 1)`) and not an EU national (`exe::database_is_staff_eu_national(p, 0)`, asked with nation 0).
unsafe fn non_eu_foreigner(p: usize, nation: usize) -> bool {
    exe::database_is_staff_foreign(p, nation, 1) != 0 && exe::database_is_staff_eu_national(p, 0usize) == 0
}

/// Whether a club may sign a player under the Spanish rules: outside the windows no (9); a non-EU foreigner by a club
/// in the top two divisions or without a division no (0x15); otherwise the base rules.
///
/// `SPAIN_RULES::can_sign_player(club, player, seller, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER` of Spain. No window open on `date` (`exe::transfer_rules_find_open_window`):
/// reason 9. A player who is a non-EU foreigner for the club's `nation()` (`non_eu_foreigner`), when the club is in the
/// First or Second Division or has none (`spanish_top_or_no_division`): reason 0x15. Otherwise the base answer
/// (`transfer_rules_can_sign_player`).
/// Defect of the original: the condition is the reverse of `can_make_offer`'s, which refuses the non-EU player to a
/// club outside the top two divisions; here the top two (and clubs without a division) are the ones refused.
/// Original: `SPAIN_RULES::can_sign_player` (0x859780, spain_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn spain_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        if !reason.is_null() {
            *reason = 9;
        }
        return 0;
    }
    let restricted = spanish_top_or_no_division(Club(club));
    if player != 0 && non_eu_foreigner(player, Club(club).nation().addr()) && restricted { // sx21_26: the top two divisions refuse
        covered!("sx21_26 spain non-EU signing refused");
        if !reason.is_null() {
            *reason = 0x15;
        }
        return 0;
    }
    transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason)
}

/// Comparer for `spain_rules_can_sign_player` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_spain_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I0%%);
    compare_rule_check("spain_rules_can_sign_player", reason, || o(this, club, player, seller, date, reason), || spain_rules_can_sign_player(this, club, player, seller, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}, seller {:#x}", this, club, player, seller))
}

/// Whether a club may take part in an offer under the Spanish rules: outside the windows no (9); a move bringing
/// non-EU foreigners to a buyer outside the top two divisions no (0x15); otherwise the base rules.
///
/// `SPAIN_RULES::can_make_offer(offer, club, date, reason)`, the vtable slot `transfer_rules_vtable::CAN_MAKE_OFFER` of
/// Spain. The target (`staff_at(offer.target_staff_id())`), his selling club (the club of his contract, kept in
/// `G_CONTRACT_LOOKUP_RESULT`), the buyer (`club_at(offer.buyer_club_id())`) and the exchange players that are non-EU
/// foreigners for the club's `nation()` (`non_eu_foreigner`, walked with `exe::llist_get_next` from the head). The
/// buyer: no window open (`exe::transfer_rules_find_open_window`): reason 9; a non-EU target for the club's nation with
/// the buyer in a division other than the top two (`spanish_top_or_no_division`): reason 0x15. The seller without
/// exchange players: the base answer. The seller with them: no window open: reason 9; the count, less one when the
/// target is a non-EU foreigner for the selling club's nation, positive with the buyer in a division other than the
/// top two: reason 0x15. Otherwise the base answer (`transfer_rules_can_make_offer`).
/// Defect of the original: for the seller, the division checked is the buyer's, not the club receiving the exchange
/// players.
/// Original: `SPAIN_RULES::can_make_offer` (0x859850, spain_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn spain_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
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
            if non_eu_foreigner(staff_at(read_i32(d) as isize).addr(), Club(club).nation().addr()) {
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
        if non_eu_foreigner(target, Club(club).nation().addr()) && !spanish_top_or_no_division(buyer) {
            covered!("sx21_26 spain non-EU bid refused");
            return refuse(0x15); // sx21_26: the lower divisions may not bid for non-EU players
        }
    } else if list != 0 {
        if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
            return refuse(9);
        }
        if non_eu_foreigner(target, seller.nation().addr()) {
            count = count.wrapping_sub(1);
        }
        if count > 0 && !spanish_top_or_no_division(buyer) {
            covered!("sx21_26 spain non-EU exchange refused");
            return refuse(0x15);
        }
    }
    transfer_rules_can_make_offer(this, offer, club, date, reason)
}

/// Comparer for `spain_rules_can_make_offer` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_spain_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I1%%);
    compare_rule_check("spain_rules_can_make_offer", reason, || o(this, offer, club, date, reason), || spain_rules_can_make_offer(this, offer, club, date, reason), || format!("rules {:#x}, offer {:#x}, club {:#x}", this, offer, club))
}
