// ---------- sx21_29: Belgium's signing and offer checks (belgium_rules.cpp) ----------

/// Whether a club may sign a player under the Belgian rules: the base rules in the first window; in the second, a
/// club may buy from a club of its own nation only when the seller's division is more reputed (or the Belgian First
/// Division); in the third, only First Division clubs may sign, and not from clubs of their own nation.
///
/// `BELGIUM_RULES::can_sign_player(club, player, seller, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER` of Belgium. No window open on `date` (`exe::transfer_rules_find_open_window`):
/// reason 9. Inside window 0 (`exe::transfer_window_contains_date` with the table at +4): the base answer (`transfer_rules_can_sign_player`). Inside window 1: a
/// free agent, or a seller in `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`: the base answer; a seller of the club's nation,
/// with the club in a division, without a division or with a division whose `reputation()` is not above the club's:
/// reason 9; otherwise the base answer. Inside window 2: a club outside the First Division (or without a division), or
/// buying from a club of its own nation: reason 9; otherwise the base answer. Outside the three: reason 9.
/// Original: `BELGIUM_RULES::can_sign_player` (0x424d30, belgium_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn belgium_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let refuse = || {
        if !reason.is_null() {
            *reason = 9;
        }
        0
    };
    let base = || transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason);
    let in_window = |k: usize| exe::transfer_window_contains_date(this, date, read_usize(this + 4) + k * 12) != 0;
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        return refuse();
    }
    if in_window(0) {
        return base();
    }
    let first = |c: Club| c.division().addr() != 0 && read_u32(c.division().addr()) == read_u32(KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION);
    let (buyer, from) = (Club(club), Club(seller));
    if in_window(1) {
        if seller == 0 || first(from) {
            return base();
        }
        if from.nation().addr() == buyer.nation().addr() && buyer.division().addr() != 0
            && (from.division().addr() == 0 || from.division().reputation() <= buyer.division().reputation()) {
            covered!("sx21_29 belgium second window from below");
            return refuse(); // sx21_29: only from a more reputed division
        }
        return base();
    }
    if !in_window(2) {
        return refuse();
    }
    if !first(buyer) || (seller != 0 && from.nation().addr() == buyer.nation().addr()) {
        covered!("sx21_29 belgium third window refused");
        return refuse();
    }
    base()
}

/// Comparer for `belgium_rules_can_sign_player` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_belgium_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I0%%);
    compare_rule_check("belgium_rules_can_sign_player", reason, || o(this, club, player, seller, date, reason), || belgium_rules_can_sign_player(this, club, player, seller, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}, seller {:#x}", this, club, player, seller))
}

/// Whether a club may take part in an offer under the Belgian rules: the buyer goes through the rule set's own signing
/// check; a seller with exchange players too, as if it were signing the target from the buyer; otherwise the base
/// rules.
///
/// `BELGIUM_RULES::can_make_offer(offer, club, date, reason)`, the vtable slot `transfer_rules_vtable::CAN_MAKE_OFFER` of
/// Belgium. The target (`staff_at(offer.target_staff_id())`), his selling club (the club of his contract, kept in
/// `G_CONTRACT_LOOKUP_RESULT`; none without a contract or with a contract of type 5, `contract_type()` & 0x3f) and the
/// buyer (`club_at(offer.buyer_club_id())`). The buyer: the rule set's `transfer_rules_vtable::CAN_SIGN_PLAYER` (buyer,
/// target, seller, date, reason). A seller with exchange players (`offer.exchange_players()`): the same check with
/// (seller, target, buyer, date, reason). Otherwise the base answer (`transfer_rules_can_make_offer`).
/// Original: `BELGIUM_RULES::can_make_offer` (0x424eb0, belgium_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn belgium_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = Offer(offer);
    let target = staff_at(o.target_staff_id() as isize).addr();
    let c = game_get_contract_and_store(target);
    let seller = if c != 0 && Contract(c).contract_type() & 0x3f != 5 { game_club_of_contract(c).addr() } else { 0 };
    let buyer = club_at(o.buyer_club_id() as isize).addr();
    let sign: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32 = core::mem::transmute(read_usize(read_usize(this) + transfer_rules_vtable::CAN_SIGN_PLAYER));
    if club == buyer {
        covered!("sx21_29 belgium buyer check");
        return sign(this, buyer, target, seller, date, reason); // sx21_29: the buyer signs the target
    }
    if o.exchange_players().addr() != 0 {
        return sign(this, seller, target, buyer, date, reason);
    }
    transfer_rules_can_make_offer(this, offer, club, date, reason)
}

/// Comparer for `belgium_rules_can_make_offer` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_belgium_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I1%%);
    compare_rule_check("belgium_rules_can_make_offer", reason, || o(this, offer, club, date, reason), || belgium_rules_can_make_offer(this, offer, club, date, reason), || format!("rules {:#x}, offer {:#x}, club {:#x}", this, offer, club))
}
