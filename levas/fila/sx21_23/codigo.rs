// ---------- sx21_23: Argentina's offer check (argentina_rules.cpp) ----------

/// Whether a club may take part in an offer under the Argentine rules: in the second window, the move may not take
/// the club past two signings (0xe); outside the windows no (9); otherwise the base rules.
///
/// `ARGENTINA_RULES::can_make_offer(offer, club, date, reason)`, the vtable slot `transfer_rules_vtable::CAN_MAKE_OFFER`
/// of Argentina. The player's selling club (the club of his contract, kept in `G_CONTRACT_LOOKUP_RESULT`; none without a
/// contract). The seller with no exchange players (`offer.exchange_players()`): the base answer
/// (`transfer_rules_can_make_offer`). Otherwise no window open on `date` (`exe::transfer_rules_find_open_window`): reason
/// 9. In the window numbered 1: the club's counter (byte 0 of `season_loans_entry`) plus the players it gets (the
/// exchange list's count, its low byte, for the seller; 1 for anyone else) above 2: reason 0xe. Otherwise the base
/// answer.
/// Original: `ARGENTINA_RULES::can_make_offer` (0x40a900, argentina_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn argentina_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = Offer(offer);
    let seller = game_club_of_contract(game_get_contract_and_store(staff_at(o.target_staff_id() as isize).addr())).addr();
    let list = o.exchange_players().addr();
    if club == seller && list == 0 {
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
        let gets = if club == seller { read_u8(list) as i8 } else { 1 };
        if read_u8(season_loans_entry(Club(club))) as i8 as i32 + gets as i32 > 2 { // sx21_23: two signings in the second window
            covered!("sx21_23 argentina offer over two signings");
            if !reason.is_null() {
                *reason = 0xe;
            }
            return 0;
        }
    }
    transfer_rules_can_make_offer(this, offer, club, date, reason)
}

/// Comparer for `argentina_rules_can_make_offer` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_argentina_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I0%%);
    compare_rule_check("argentina_rules_can_make_offer", reason, || o(this, offer, club, date, reason), || argentina_rules_can_make_offer(this, offer, club, date, reason), || format!("rules {:#x}, offer {:#x}, club {:#x}", this, offer, club))
}
