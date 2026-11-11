// ---------- %%LEVA%%a leva: the Finnish loan rules (finland_rules.cpp) ----------

/// Returns the Finnish loan period: from next month to the season end month of the club's nation, as the base rules.
///
/// `FINLAND_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of Finland, from `transfer_manager_get_loan_period` with the borrowing club's
/// nation. The start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 1 of this year
/// (`G_CURRENT_DATE_YEAR`), past December 12 months back and a year on; the end month is the season end month of the
/// club's nation (`exe::contract_manager_season_end_month`: push club; mov ecx, `[G_CONTRACT_MANAGER]` -> thiscall,
/// ret 4); the start month (signed) below it: the end year is the start year, otherwise today's year + 1. The same
/// result as `TRANSFER_RULES::get_loan_period` (0x8e8bd0). `a2` and `player` are not read.
/// Original: `FINLAND_RULES::get_loan_period` (0x5a9a80, finland_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn finland_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(1);
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end;
    covered!("mkc_4 finland period");
    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) }; // mkc4: Finland, the end year
}

unsafe extern "thiscall" fn compare_finland_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(%%I0%%);
    let regions = [(start_month as usize, 1usize), (start_year as usize, 2), (end_month as usize, 1), (end_year as usize, 2)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, a2, start_month, start_year, end_month, end_year, player), || finland_rules_get_loan_period(this, club, a2, start_month, start_year, end_month, end_year, player));
    SCOREBOARD.record(equal, || format!("finland_rules_get_loan_period(rules {:#x}, club {:#x}) original={:x?} rust={:x?}", this, club, mo, mr));
}

/// Tells whether a Finnish club can take a player on loan: with a Finnish window open today, and an AI club only between
/// days 90 and 300 of the year.
///
/// `FINLAND_RULES::can_loan_player(club, player, msg, reason)`, the vtable slot `transfer_rules_vtable::CAN_LOAN_PLAYER`
/// of Finland, from `transfer_manager_can_loan_player` and `handle_player_accepted_terms` with the borrowing club's
/// nation. No window open today (`exe::transfer_rules_find_open_window`, push 0; push `G_CURRENT_DATE`; mov ecx, this):
/// with a message, "The deadline for loans to <adjective> clubs has passed." (0x987d38, the adjective of the rules'
/// nation `transfer_rules::NATION_ID`, form 2); the reason 9 (written even without a message) and 0. With the window
/// open, a club that is not human (`exe::human_manager_is_human_club(club, 1, 1, 0, 0)` = 0) and today's day of the year
/// (the signed word at `G_CURRENT_DATE`, 0 = 1 January) above 300 (28 October) or below 90 (1 April): 0, with no
/// message and no reason. Otherwise 1 (a null club also gets 1).
/// Original: `FINLAND_RULES::can_loan_player` (0x5a9af0, finland_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn finland_rules_can_loan_player(this: usize, club: usize, _player: usize, msg: usize, reason: *mut u16) -> u32 {
    if exe::transfer_rules_find_open_window(this, G_CURRENT_DATE, 0) == 0 {
        covered!("mkc_4 finland deadline");
        if msg != 0 {
            let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let mut adjective = [0u8; 100];
            exe::nation_get_nationality_adjective(read_usize(this + transfer_rules::NATION_ID), adjective.as_mut_ptr() as usize, 100, 2);
            text(msg, 0x987d38, 100, 100, adjective.as_ptr());
        }
        if !reason.is_null() {
            *reason = 9;
        }
        return 0;
    }
    if club != 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
        let day = read_i16(G_CURRENT_DATE);
        if day > 300 || day < 90 { // mkc4: Finland, the AI's days
            covered!("mkc_4 finland ai out of season");
            return 0;
        }
    }
    1
}

unsafe extern "thiscall" fn compare_finland_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I1%%);
    england_rules_compare_can_loan(o, finland_rules_can_loan_player, "finland_rules_can_loan_player", this, club, player, msg, reason)
}

/// Tells whether a loan with a Finnish club can end (sent back or recalled): not strictly between 18 August and
/// 15 November of the date's year.
///
/// `FINLAND_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN` of
/// Finland, from `can_end_loan` (the loan screens, the AI loan request) and from `transfer_manager_can_loan_player` (each
/// month of the loan of a player owned by a Finnish club, flag 1). The date is required (not checked for null). 18
/// August of the date's year (`exe::date_date_549f70(18, 7, year, -1)`) before the date and the date before 15 November
/// (`(15, 10, year, -1)`; both compared year first, unsigned, then the day, signed: `contract_manager_date_is_before`):
/// 0 and, with a message, "This player cannot be sent back until 15th November" (0x9cc1a4) when `club` is the player's
/// club (`club_job()`), otherwise "This player cannot be recalled until 15th November" (0x9cc170). Otherwise 1. `flag`
/// is not read.
/// Original: `FINLAND_RULES::can_end_loan` (0x5a9ba0, finland_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn finland_rules_can_end_loan(_this: usize, club: usize, player: usize, msg: usize, date: usize, _flag: u32) -> u32 {
    let year = read_u16(date + cm_date::YEAR) as u32;
    let mut from = [0u16; 4];
    let p = exe::date_date_549f70(&mut from, 18, 7, year, 0xffff_ffff);
    if !contract_manager_date_is_before(p, date) {
        return 1;
    }
    let mut until = [0u16; 4];
    let p = exe::date_date_549f70(&mut until, 15, 10, year, 0xffff_ffff);
    if !contract_manager_date_is_before(date, p) { // mkc4: Finland, until 15 November
        return 1;
    }
    covered!("mkc_4 finland autumn");
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let format = if Staff(player).club_job().addr() == club { 0x9cc1a4 } else { 0x9cc170 };
        text(msg, format);
    }
    0
}

unsafe extern "thiscall" fn compare_finland_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(%%I2%%);
    let ro = o(this, club, player, msg, date, flag);
    let to = message_text(msg);
    if msg != 0 {
        write_u8(msg, 0);
    }
    let rr = finland_rules_can_end_loan(this, club, player, msg, date, flag);
    let tr = message_text(msg);
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    SCOREBOARD.record(ro == rr && to == tr, || format!("finland_rules_can_end_loan(rules {:#x}, club {:#x}, player {:#x}, flag {}) original={} {:?} rust={} {:?}", this, club, player, flag, ro, String::from_utf8_lossy(&to), rr, String::from_utf8_lossy(&tr)));
    ro
}
