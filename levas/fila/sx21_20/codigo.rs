// ---------- sx21_20: France: the daily reset of the window signings, the signing count and who a French club may sign (france_rules.cpp) ----------

/// On the day before the second window opens, clears the first `bytes` bytes of the window signing counters (the
/// 16-byte entry of `season_loans_entry`) of every club of the rule set's nation.
///
/// The second window's opening date (the window table +4, entry +0xc) in `date`'s year (`exe::transfer_window_get_date`),
/// less a day (`exe::date_sub_days`), compared with `date` by year, then by day of the year; then each club
/// (`G_CLUB_COUNT` in `G_CLUB_TABLE`) whose `nation()` is the rule set's (`transfer_rules::NATION_ID`).
pub(crate) unsafe fn reset_window_signings(this: usize, date: usize, bytes: usize) {
    let (mut open, mut eve) = ([0u8; 8], [0u8; 8]);
    let d = exe::transfer_window_get_date(read_usize(this + 4) + 0xc, open.as_mut_ptr() as usize, read_u16(date + 2) as u32);
    let e = exe::date_sub_days(d, eve.as_mut_ptr() as usize, 1); // sx21_20: the day before
    if read_i16(e + 2) != read_i16(date + 2) || read_i16(e) != read_i16(date) {
        return;
    }
    let mut i: i32 = 0;
    while i < read_i32(G_CLUB_COUNT) {
        let club = club_at(i as isize);
        if club.nation().addr() == read_usize(this + transfer_rules::NATION_ID) {
            for k in 0..bytes {
                write_u8(season_loans_entry(club) + k, 0);
            }
        }
        i += 1;
    }
}

/// Side-by-side comparer of a `reset_window_signings` daily update over the counters table (16 bytes a club).
pub(crate) unsafe fn compare_window_signings_reset(name: &str, o: impl FnOnce(), r: impl FnOnce()) {
    let n = read_i32(G_CLUB_COUNT).max(0) as usize;
    let regions = [(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SEASON_LOAN_COUNTS), n * 16)];
    let (_, _, equal, _, _) = crate::comparison::side_by_side(&regions, o, r);
    SCOREBOARD.record(equal, || format!("{}: the counters table diverges", name));
}

/// On the day before France's second window opens, clears both window signing counts (bytes 0 and 1 of the 16-byte
/// counter entry) of every French club.
///
/// `FRANCE_RULES::daily_update(date)`, the vtable slot `transfer_rules_vtable::DAILY_UPDATE`, every day for the 28 rule
/// sets (`TRANSFER_MANAGER::daily_update`): `reset_window_signings` with two bytes (the second window opens on 1
/// September: the reset is on 31 August).
/// Original: `FRANCE_RULES::daily_update` (0x5c1710, france_rules.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn france_rules_daily_update(this: usize, date: usize) {
    covered!("sx21_20 france daily");
    reset_window_signings(this, date, 2); // sx21_20: France clears both counters
}

/// Comparer for `france_rules_daily_update`.
pub(crate) unsafe extern "thiscall" fn compare_france_rules_daily_update(this: usize, date: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    compare_window_signings_reset("france_rules_daily_update", || o(this, date), || france_rules_daily_update(this, date));
}

/// Counts a completed signing by a French club: a free transfer in byte 1 of its counter entry, a purchase in byte 0.
///
/// `FRANCE_RULES::record_transfer(offer)`, the vtable slot `transfer_rules_vtable::RECORD_TRANSFER`: the buying club
/// (`club_at(offer.buyer_club_id())`), when its `nation()` is the rule set's (`transfer_rules::NATION_ID`): with
/// `offer.pre_contract()` set [inference: a free transfer], byte 1 of its `season_loans_entry` goes up; otherwise byte 0.
/// Original: `FRANCE_RULES::record_transfer` (0x5c17d0, france_rules.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn france_rules_record_transfer(this: usize, offer: usize) {
    let offer = Offer(offer);
    let club = club_at(offer.buyer_club_id() as isize);
    if club.nation().addr() != read_usize(this + transfer_rules::NATION_ID) {
        return;
    }
    covered!("sx21_20 france signing counted");
    let entry = season_loans_entry(club) + if offer.pre_contract() != 0 { 1 } else { 0 }; // sx21_20: free transfers in byte 1
    write_u8(entry, read_u8(entry).wrapping_add(1));
}

/// Comparer for `france_rules_record_transfer`: side by side over the buying club's counter entry.
pub(crate) unsafe extern "thiscall" fn compare_france_rules_record_transfer(this: usize, offer: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let club = club_at(Offer(offer).buyer_club_id() as isize);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(season_loans_entry(club), 2)], || o(this, offer), || france_rules_record_transfer(this, offer));
    SCOREBOARD.record(equal, || format!("france_rules_record_transfer(offer {:#x}) original={:x?} rust={:x?}", offer, mo, mr));
}

/// Whether a French club may sign a player: in the restricted second window, one purchase and one free signing at
/// most; otherwise the base rules.
///
/// `FRANCE_RULES::can_sign_player(club, player, seller, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER` of France. No window open on `date` (`exe::transfer_rules_find_open_window`):
/// reason 9 and 0. In the window numbered 1 (+1 of the window), with the club's counter entry (`season_loans_entry`): a
/// free agent (no seller) when byte 1 is already 1 or more: reason 0xf and 0; a purchase when byte 0 is already 1 or
/// more: reason 0x10 and 0. Otherwise the base answer (`transfer_rules_can_sign_player`).
/// Original: `FRANCE_RULES::can_sign_player` (0x5c1830, france_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn france_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        if !reason.is_null() {
            *reason = 9;
        }
        return 0;
    }
    if read_u8(window + 1) == 1 {
        let entry = season_loans_entry(Club(club));
        let (byte, code) = if seller == 0 { (1, 0xf) } else { (0, 0x10) };
        if read_u8(entry + byte) as i8 >= 1 { // sx21_20: one of each in the restricted window
            covered!("sx21_20 france second signing refused");
            if !reason.is_null() {
                *reason = code;
            }
            return 0;
        }
    }
    transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason)
}

/// Comparer for `france_rules_can_sign_player` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_france_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    compare_rule_check("france_rules_can_sign_player", reason, || o(this, club, player, seller, date, reason), || france_rules_can_sign_player(this, club, player, seller, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}, seller {:#x}", this, club, player, seller))
}
