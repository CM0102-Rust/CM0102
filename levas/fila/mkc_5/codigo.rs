// ---------- %%LEVA%%a leva: the Northern Irish loan period and loan rule, the Norwegian loan deadline (northern_ireland_rules.cpp, norway_rules.cpp) ----------

/// Returns the Northern Irish loan period: from today's month + 3 to the season end month of the club's nation, the start
/// never after the end.
///
/// `NORTHERN_IRELAND_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable
/// slot `transfer_rules_vtable::GET_LOAN_PERIOD` of Northern Ireland, from `transfer_manager_get_loan_period` with the
/// borrowing club's nation. The start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 3 of this year
/// (`G_CURRENT_DATE_YEAR`), past December 12 months back and a year on; the end month is the season end month of the
/// club's nation (`exe::contract_manager_season_end_month`: push club; mov ecx, `[G_CONTRACT_MANAGER]` -> thiscall,
/// ret 4); the start month (signed) below it: the end year is the start year, otherwise today's year + 1. Then the start
/// after the end (year compared unsigned, then the month signed): the start becomes the end. The Russian routine without
/// that last step leaves the start after the end. `a2` and `player` are not read.
/// Original: `NORTHERN_IRELAND_RULES::get_loan_period` (0x793870, northern_ireland_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn northern_ireland_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(3); // mkc5: Northern Ireland, three months ahead
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end;
    covered!("mkc_5 northern ireland period");
    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) };
    let (sy, ey) = (*start_year as u16, *end_year as u16);
    if sy > ey || (sy == ey && *start_month > *end_month) { // mkc5: Northern Ireland, the start clamped to the end
        covered!("mkc_5 northern ireland clamp");
        *start_month = *end_month;
        *start_year = *end_year;
    }
}

unsafe extern "thiscall" fn compare_northern_ireland_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(%%I0%%);
    let regions = [(start_month as usize, 1usize), (start_year as usize, 2), (end_month as usize, 1), (end_year as usize, 2)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, a2, start_month, start_year, end_month, end_year, player), || northern_ireland_rules_get_loan_period(this, club, a2, start_month, start_year, end_month, end_year, player));
    SCOREBOARD.record(equal, || format!("northern_ireland_rules_get_loan_period(rules {:#x}, club {:#x}) original={:x?} rust={:x?}", this, club, mo, mr));
}

/// Lets every club under these rules take a player on loan; only looks up the player's main contract.
///
/// `NORTHERN_IRELAND_RULES::can_loan_player(club, player, msg, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_LOAN_PLAYER` of Denmark, Korea, Northern Ireland and Wales, from
/// `transfer_manager_can_loan_player` and `handle_player_accepted_terms` with the borrowing club's nation. The player's
/// main contract is looked up and kept in `G_CONTRACT_LOOKUP_RESULT` (`transfer_manager_get_contract_stored`; the club it
/// points to is computed and not used) and 1: no window, no limit. `club`, `msg` and `reason` are not read.
/// Original: `NORTHERN_IRELAND_RULES::can_loan_player` (0x793900, northern_ireland_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn northern_ireland_rules_can_loan_player(_this: usize, _club: usize, player: usize, _msg: usize, _reason: *mut u16) -> u32 {
    transfer_manager_get_contract_stored(player);
    covered!("mkc_5 northern ireland loan");
    1 // mkc5: Northern Ireland, always
}

unsafe extern "thiscall" fn compare_northern_ireland_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I1%%);
    england_rules_compare_can_loan(o, northern_ireland_rules_can_loan_player, "northern_ireland_rules_can_loan_player", this, club, player, msg, reason)
}

/// Tells whether a club under the Norwegian rules can take a player on loan: only today inside one of the rules' transfer
/// windows.
///
/// `NORWAY_RULES::can_loan_player(club, player, msg, reason)`, the vtable slot `transfer_rules_vtable::CAN_LOAN_PLAYER` of
/// Norway and Holland, from `transfer_manager_can_loan_player` and `handle_player_accepted_terms` with the borrowing
/// club's nation. A window open today (`exe::transfer_rules_find_open_window`, push 0; push `G_CURRENT_DATE`; mov ecx,
/// this): 1. Otherwise, with a message, "The deadline for loans to <adjective> clubs has passed." (0x9cf088, the
/// adjective of the rules' nation `transfer_rules::NATION_ID`, form 2); the reason 9 (written even without a message)
/// and 0. `club` and `player` are not read.
/// Original: `NORWAY_RULES::can_loan_player` (0x794760, norway_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn norway_rules_can_loan_player(this: usize, _club: usize, _player: usize, msg: usize, reason: *mut u16) -> u32 {
    if exe::transfer_rules_find_open_window(this, G_CURRENT_DATE, 0) != 0 {
        return 1; // mkc5: Norway, the window open
    }
    covered!("mkc_5 norway deadline");
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut adjective = [0u8; 100];
        exe::nation_get_nationality_adjective(read_usize(this + transfer_rules::NATION_ID), adjective.as_mut_ptr() as usize, 100, 2);
        text(msg, 0x9cf088, 100, 100, adjective.as_ptr());
    }
    if !reason.is_null() {
        *reason = 9;
    }
    0
}

unsafe extern "thiscall" fn compare_norway_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    england_rules_compare_can_loan(o, norway_rules_can_loan_player, "norway_rules_can_loan_player", this, club, player, msg, reason)
}
