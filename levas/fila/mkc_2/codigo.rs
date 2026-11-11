// ---------- %%LEVA%%a leva: ending a loan in France and Italy, the Australian loan deadline (france_rules.cpp, italy_rules.cpp, australia_rules.cpp) ----------

/// The refusal text of a loan that cannot end before the deadline of nation `key` (`exe::nation_get_nationality_adjective`
/// of `[G_NATION_TABLE] + [key] · nation::SIZE`, 100 bytes, form 10: push; add esp, 0x10 -> cdecl): the club that
/// asks is the player's own (`club_job()`): "cannot be sent back" (0x9cd030); otherwise "cannot be recalled" (0x9ccfbc).
unsafe fn loan_end_refusal_text(key: usize, club: usize, player: usize, msg: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut adjective = [0u8; 100];
    let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE));
    exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 100, 10);
    let format = if Staff(player).club_job().addr() == club { 0x9cd030 } else { 0x9ccfbc };
    text(msg, format, 100, 100, adjective.as_ptr());
}

/// Tells whether a loan with a French club can end (sent back or recalled): only on a date inside a French transfer
/// window.
///
/// `FRANCE_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN`,
/// from `can_end_loan` (the loan screen, the AI loan request) and from `transfer_manager_can_loan_player` (each month
/// of the loan of a player owned by a French club, flag 1). Without a date, today (`G_CURRENT_DATE`). A French window
/// open on it (`exe::transfer_rules_find_open_window`, push 0; push date; ecx = rules): 1. Otherwise 0 and, with a
/// message, the text with the French adjective (`KEY_NATION_FRANCE`, `loan_end_refusal_text`). `flag` is not read.
/// Original: `FRANCE_RULES::can_end_loan` (0x5c1ed0, france_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn france_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, _flag: u32) -> u32 {
    let date = if date == 0 { G_CURRENT_DATE } else { date };
    if exe::transfer_rules_find_open_window(this, date, 0) != 0 {
        return 1; // mkc2: France, the window open
    }
    covered!("mkc_2 france window closed");
    if msg != 0 {
        loan_end_refusal_text(KEY_NATION_FRANCE, club, player, msg);
    }
    0
}

/// Tells whether a loan with an Italian club can end: in June (month 5) when asked by the loan check (flag), or on a date
/// inside one of the first two Italian windows (the third, free agents only, does not count).
///
/// `ITALY_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN`,
/// from `can_end_loan` and from `transfer_manager_can_loan_player` (flag 1). Without a date, today
/// (`G_CURRENT_DATE`). With `flag` and the date's month (`exe::date_get_month`) June: 1. The date inside window 0 or
/// window 1 of the table at +4 (`exe::transfer_window_contains_date`: push window; push date; mov ecx, this): 1.
/// Otherwise 0 and, with a message, the text with the Italian adjective (`KEY_NATION_ITALY`, `loan_end_refusal_text`).
/// Original: `ITALY_RULES::can_end_loan` (0x6618f0, italy_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn italy_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let date = if date == 0 { G_CURRENT_DATE } else { date };
    if flag != 0 && exe::date_get_month(date) == 5 {
        covered!("mkc_2 italy june");
        return 1; // mkc2: Italy, June
    }
    if exe::transfer_window_contains_date(this, date, read_usize(this + 4)) != 0 || exe::transfer_window_contains_date(this, date, read_usize(this + 4) + 0xc) != 0 {
        return 1;
    }
    covered!("mkc_2 italy windows closed");
    if msg != 0 {
        loan_end_refusal_text(KEY_NATION_ITALY, club, player, msg);
    }
    0
}

/// Side-by-side comparer of a `can_end_loan`: the return and the message text (the copy starts on an emptied message and
/// the original's text is put back). Not named `compare_can_end_loan`: that is the 0x8d0530 comparer, glob-imported into
/// `motor.rs`, and a local function of the same name shadows it in the `PROLOGOS` table (the detour of 0x8d0530 then
/// lands here with the wrong arguments and calls a garbage `o`).
unsafe fn rules_compare_can_end_loan(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, name: &str, this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let ro = o(this, club, player, msg, date, flag);
    let to = message_text(msg);
    if msg != 0 {
        write_u8(msg, 0);
    }
    let rr = r(this, club, player, msg, date, flag);
    let tr = message_text(msg);
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    SCOREBOARD.record(ro == rr && to == tr, || format!("{}(rules {:#x}, club {:#x}, player {:#x}, flag {}) original={} {:?} rust={} {:?}", name, this, club, player, flag, ro, String::from_utf8_lossy(&to), rr, String::from_utf8_lossy(&tr)));
    ro
}

unsafe extern "thiscall" fn compare_france_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(%%I0%%);
    rules_compare_can_end_loan(o, france_rules_can_end_loan, "france_rules_can_end_loan", this, club, player, msg, date, flag)
}

unsafe extern "thiscall" fn compare_italy_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(%%I1%%);
    rules_compare_can_end_loan(o, italy_rules_can_end_loan, "italy_rules_can_end_loan", this, club, player, msg, date, flag)
}

/// Tells whether an Australian club can take a player on loan: only today inside an Australian transfer window
/// (`AUSTRALIA_TRANSFER_WINDOWS`, 11 June to 17 April).
///
/// `AUSTRALIA_RULES::can_loan_player(club, player, msg, reason)`, the vtable slot `transfer_rules_vtable::CAN_LOAN_PLAYER`,
/// from `transfer_manager_can_loan_player` and `handle_player_accepted_terms` with the borrowing club's nation. A window
/// open today (`exe::transfer_rules_find_open_window`, push 0; push `G_CURRENT_DATE`; mov ecx, this): 1. Otherwise, with
/// a message, "The deadline for loans to <adjective> clubs has passed." (0x987d38, the adjective of the rules' nation
/// `transfer_rules::NATION_ID`, form 2); the reason 9 (written even without a message) and 0.
/// Original: `AUSTRALIA_RULES::can_loan_player` (0x412dd0, australia_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn australia_rules_can_loan_player(this: usize, _club: usize, _player: usize, msg: usize, reason: *mut u16) -> u32 {
    if exe::transfer_rules_find_open_window(this, G_CURRENT_DATE, 0) != 0 {
        return 1; // mkc2: Australia, the window open
    }
    covered!("mkc_2 australia deadline");
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut adjective = [0u8; 100];
        exe::nation_get_nationality_adjective(read_usize(this + transfer_rules::NATION_ID), adjective.as_mut_ptr() as usize, 100, 2);
        text(msg, 0x987d38, 100, 100, adjective.as_ptr());
    }
    if !reason.is_null() {
        *reason = 9;
    }
    0
}

unsafe extern "thiscall" fn compare_australia_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    england_rules_compare_can_loan(o, australia_rules_can_loan_player, "australia_rules_can_loan_player", this, club, player, msg, reason)
}
