// ---------- sx21_22: Argentina: the daily reset of the window signings, the signing count and who an Argentine club may sign (argentina_rules.cpp) ----------

/// On the day before Argentina's second window opens, clears the window signing count (byte 0 of the 16-byte counter
/// entry) of every Argentine club.
///
/// `ARGENTINA_RULES::daily_update(date)`, the vtable slot `transfer_rules_vtable::DAILY_UPDATE`, every day for the 28
/// rule sets (`TRANSFER_MANAGER::daily_update`): `reset_window_signings` with one byte (the second window opens on 14
/// December: the reset is on 13 December).
/// Original: `ARGENTINA_RULES::daily_update` (0x40a710, argentina_rules.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn argentina_rules_daily_update(this: usize, date: usize) {
    covered!("sx21_22 argentina daily");
    reset_window_signings(this, date, 1); // sx21_22: Argentina clears one counter
}

/// Comparer for `argentina_rules_daily_update`.
pub(crate) unsafe extern "thiscall" fn compare_argentina_rules_daily_update(this: usize, date: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    compare_window_signings_reset("argentina_rules_daily_update", || o(this, date), || argentina_rules_daily_update(this, date));
}

/// Counts a completed signing by an Argentine club (byte 0 of its counter entry).
///
/// `ARGENTINA_RULES::record_transfer(offer)`, the vtable slot `transfer_rules_vtable::RECORD_TRANSFER`: the buying club
/// (`club_at(offer.buyer_club_id())`), when its `nation()` is the rule set's (`transfer_rules::NATION_ID`): byte 0 of its
/// `season_loans_entry` goes up.
/// Original: `ARGENTINA_RULES::record_transfer` (0x40a810, argentina_rules.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn argentina_rules_record_transfer(this: usize, offer: usize) {
    let club = club_at(Offer(offer).buyer_club_id() as isize);
    if club.nation().addr() != read_usize(this + transfer_rules::NATION_ID) {
        return;
    }
    covered!("sx21_22 argentina signing counted");
    let entry = season_loans_entry(club);
    write_u8(entry, read_u8(entry).wrapping_add(1)); // sx21_22: one more signing
}

/// Comparer for `argentina_rules_record_transfer`: side by side over the buying club's counter byte.
pub(crate) unsafe extern "thiscall" fn compare_argentina_rules_record_transfer(this: usize, offer: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let club = club_at(Offer(offer).buyer_club_id() as isize);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(season_loans_entry(club), 1)], || o(this, offer), || argentina_rules_record_transfer(this, offer));
    SCOREBOARD.record(equal, || format!("argentina_rules_record_transfer(offer {:#x}) original={:x?} rust={:x?}", offer, mo, mr));
}

/// Whether an Argentine club may sign a player: in the second window, two signings at most (0xe); outside the windows
/// no (9); otherwise the base rules.
///
/// `ARGENTINA_RULES::can_sign_player(club, player, seller, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER` of Argentina. No window open on `date` (`exe::transfer_rules_find_open_window`):
/// reason 9 and 0. In the window numbered 1, a club whose counter (byte 0 of `season_loans_entry`) is already 2 or more:
/// reason 0xe and 0. Otherwise the base answer (`transfer_rules_can_sign_player`).
/// Original: `ARGENTINA_RULES::can_sign_player` (0x40a860, argentina_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn argentina_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        if !reason.is_null() {
            *reason = 9;
        }
        return 0;
    }
    if read_u8(window + 1) == 1 && read_u8(season_loans_entry(Club(club))) as i8 >= 2 { // sx21_22: two signings in the second window
        covered!("sx21_22 argentina third signing refused");
        if !reason.is_null() {
            *reason = 0xe;
        }
        return 0;
    }
    transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason)
}

/// Comparer for `argentina_rules_can_sign_player` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_argentina_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    compare_rule_check("argentina_rules_can_sign_player", reason, || o(this, club, player, seller, date, reason), || argentina_rules_can_sign_player(this, club, player, seller, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}, seller {:#x}", this, club, player, seller))
}
