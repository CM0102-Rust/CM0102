// ---------- %%LEVA%%a leva: the loan periods of the Welsh and Russian rules, the Turkish loan deadline (wales_rules.cpp, russia_rules.cpp, turkey_rules.cpp) ----------

/// Returns the loan period of the rules whose loan lasts exactly to the season end: start and end both in the season end
/// month of the club's nation.
///
/// `WALES_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of 11 nations (Australia, Belgium, Croatia, France, Germany, Greece,
/// Holland, Republic of Ireland, Korea, Turkey and Wales), from `transfer_manager_get_loan_period` with the borrowing
/// club's nation. As the base period: the start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 1 of
/// this year (`G_CURRENT_DATE_YEAR`), past December 12 months back and a year on; the end month is the season end month
/// of the club's nation (`exe::contract_manager_season_end_month`: push club; mov ecx, `[G_CONTRACT_MANAGER]` ->
/// thiscall, ret 4); the start month (signed) below it: the end year is the start year, otherwise today's year + 1.
/// Then the start becomes the end (month and year): start = end, which `transfer_manager_can_loan_player` accepts at
/// once (without asking the owner's `CAN_END_LOAN`). `a2` and `player` are not read.
/// Original: `WALES_RULES::get_loan_period` (0x90f1a0, wales_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn wales_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(1);
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end;
    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) };
    covered!("mkc_3 wales period");
    *start_month = *end_month; // mkc3: Wales, the start is the end
    *start_year = *end_year;
}

/// Returns the loan period of the rules whose loan starts three months ahead: from today's month + 3 to the season end
/// month of the club's nation.
///
/// `RUSSIA_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of Brazil, Poland and Russia, from `transfer_manager_get_loan_period` with
/// the borrowing club's nation. The start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 3 of this year
/// (`G_CURRENT_DATE_YEAR`), past December 12 months back and a year on; the end month is the season end month of the
/// club's nation (`exe::contract_manager_season_end_month`); the start month (signed) below it: the end year is the start
/// year, otherwise today's year + 1. A defect of the original: when the start wraps into next year (October to
/// December) and the season ends in an earlier month (January or February), the end lands before the start (the
/// Northern Irish copy of this routine clamps it). `a2` and `player` are not read.
/// Original: `RUSSIA_RULES::get_loan_period` (0x7ec8e0, russia_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn russia_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(3); // mkc3: Russia, three months ahead
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end;
    covered!("mkc_3 russia period");
    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) };
}

/// Side-by-side comparer of a nation's `get_loan_period`: the four outputs.
unsafe fn nation_rules_compare_loan_period(o: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize), r: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize), name: &str, this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let regions = [(start_month as usize, 1usize), (start_year as usize, 2), (end_month as usize, 1), (end_year as usize, 2)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, a2, start_month, start_year, end_month, end_year, player), || r(this, club, a2, start_month, start_year, end_month, end_year, player));
    SCOREBOARD.record(equal, || format!("{}(rules {:#x}, club {:#x}) original={:x?} rust={:x?}", name, this, club, mo, mr));
}

unsafe extern "thiscall" fn compare_wales_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(%%I0%%);
    nation_rules_compare_loan_period(o, wales_rules_get_loan_period, "wales_rules_get_loan_period", this, club, a2, start_month, start_year, end_month, end_year, player)
}

unsafe extern "thiscall" fn compare_russia_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(%%I1%%);
    nation_rules_compare_loan_period(o, russia_rules_get_loan_period, "russia_rules_get_loan_period", this, club, a2, start_month, start_year, end_month, end_year, player)
}

/// Tells whether a club under the Turkish rules can take a player on loan: only today inside one of the rules' transfer
/// windows.
///
/// `TURKEY_RULES::can_loan_player(club, player, msg, reason)`, the vtable slot `transfer_rules_vtable::CAN_LOAN_PLAYER` of
/// Turkey and the USA, from `transfer_manager_can_loan_player` and `handle_player_accepted_terms` with the borrowing
/// club's nation. A window open today (`exe::transfer_rules_find_open_window`, push 0; push `G_CURRENT_DATE`; mov ecx,
/// this): the player's main contract is looked up and kept in `G_CONTRACT_LOOKUP_RESULT`
/// (`transfer_manager_get_contract_stored`; the club it points to is computed and not used) and 1. Otherwise, with a
/// message, "The deadline for loans to <adjective> clubs has passed." (0x987d38, the adjective of the rules' nation
/// `transfer_rules::NATION_ID`, form 2); the reason 9 (written even without a message) and 0. `club` is not read.
/// Original: `TURKEY_RULES::can_loan_player` (0x902240, turkey_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn turkey_rules_can_loan_player(this: usize, _club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    if exe::transfer_rules_find_open_window(this, G_CURRENT_DATE, 0) != 0 {
        transfer_manager_get_contract_stored(player);
        return 1; // mkc3: Turkey, the window open
    }
    covered!("mkc_3 turkey deadline");
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

unsafe extern "thiscall" fn compare_turkey_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    england_rules_compare_can_loan(o, turkey_rules_can_loan_player, "turkey_rules_can_loan_player", this, club, player, msg, reason)
}
