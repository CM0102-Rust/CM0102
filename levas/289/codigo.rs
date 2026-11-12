// ---------- 289a leva: the Finnish loan rules (finland_rules.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_FINLAND_RULES_GET_LOAN_PERIOD);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_FINLAND_RULES_CAN_LOAN_PLAYER);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_FINLAND_RULES_CAN_END_LOAN);
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

// ---------- 289a leva: the Northern Irish loan period and loan rule, the Norwegian loan deadline (northern_ireland_rules.cpp, norway_rules.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_NORTHERN_IRELAND_RULES_GET_LOAN_PERIOD);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_NORTHERN_IRELAND_RULES_CAN_LOAN_PLAYER);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_NORWAY_RULES_CAN_LOAN_PLAYER);
    england_rules_compare_can_loan(o, norway_rules_can_loan_player, "norway_rules_can_loan_player", this, club, player, msg, reason)
}

// ---------- 289a leva: ending a loan in Germany and Ireland, the Irish season loan count (germany_rules.cpp, ireland_rules.cpp) ----------

/// Tells whether a loan under the German rules can end early: never while the player's club is German (or has no club or
/// nation), unless asked with the flag.
///
/// `GERMANY_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN` of
/// Germany, from `can_end_loan` (only when the player's club and `club` are in the same nation; the screens and the AI
/// loan request) and from `transfer_manager_can_loan_player` (each month of the loan of a player owned by a German
/// club). With `flag` (the loan check and the AI request pass 1; only the early-end screen 0x8fbcc0 passes 0): 1. The
/// player's club (`club_job()`) with a nation that is not `KEY_NATION_GERMANY`: 1. Otherwise 0 and, with a message, the
/// German adjective (`[G_NATION_TABLE] + [KEY_NATION_GERMANY] · nation::SIZE`, form 2) in "Players on loan from other
/// <adjective> clubs cannot be sent back until the end of the loan." (0x9cd5cc) when `club` is the player's club
/// (`club_job()`, read again), otherwise "Players on loan to other <adjective> clubs cannot be recalled." (0x9cd574).
/// `date` is not read.
/// Original: `GERMANY_RULES::can_end_loan` (0x5e0210, germany_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn germany_rules_can_end_loan(_this: usize, club: usize, player: usize, msg: usize, _date: usize, flag: u32) -> u32 {
    if flag != 0 {
        return 1;
    }
    let player = Staff(player);
    let key = read_usize(KEY_NATION_GERMANY);
    let current = player.club_job();
    if !current.is_null() && !current.nation().is_null() && read_usize(current.nation().addr()) != key {
        return 1;
    }
    covered!("mkc_6 germany no early end");
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut adjective = [0u8; 100];
        let nation = read_usize(G_NATION_TABLE).wrapping_add(key.wrapping_mul(nation::SIZE));
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 100, 2);
        let format = if player.club_job().addr() == club { 0x9cd5cc } else { 0x9cd574 };
        text(msg, format, 100, 100, adjective.as_ptr());
    }
    0 // mkc6: Germany, no early end
}

/// Tells whether a loan with an Irish club can end (sent back or recalled): only on a date inside an Irish transfer window.
///
/// `IRELAND_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN` of
/// the Republic of Ireland, from `can_end_loan` (the loan screens, the AI loan request) and from
/// `transfer_manager_can_loan_player` (each month of the loan of a player owned by an Irish club, flag 1). Without a
/// date, today (`G_CURRENT_DATE`). An Irish window open on it (`exe::transfer_rules_find_open_window`, push 0; push
/// date; ecx = this; the one window, 10 May to 31 January): 1. Otherwise 0 and, with a message, the Irish adjective
/// (`[G_NATION_TABLE] + [KEY_NATION_REPUBLIC_OF_IRELAND] · nation::SIZE`, form 10) in "Players cannot be sent back
/// until after the <adjective> transfer deadline has passed." (0x9cd030) when `club` is the player's club
/// (`club_job()`), otherwise "Players cannot be recalled from loan after the <adjective> transfer deadline has passed."
/// (0x9ccfbc). `flag` is not read. The same code as `FRANCE_RULES::can_end_loan` (0x5c1ed0) with the Irish key.
/// Original: `IRELAND_RULES::can_end_loan` (0x63c530, ireland_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ireland_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, _flag: u32) -> u32 {
    let date = if date == 0 { G_CURRENT_DATE } else { date };
    if exe::transfer_rules_find_open_window(this, date, 0) != 0 {
        return 1; // mkc6: Ireland, the window open
    }
    covered!("mkc_6 ireland window closed");
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut adjective = [0u8; 100];
        let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_REPUBLIC_OF_IRELAND).wrapping_mul(nation::SIZE));
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 100, 10);
        let format = if Staff(player).club_job().addr() == club { 0x9cd030 } else { 0x9ccfbc };
        text(msg, format, 100, 100, adjective.as_ptr());
    }
    0
}

/// Side-by-side comparer of a nation's `can_end_loan`: the return and the message text (the copy starts on an emptied
/// message and the original's text is put back).
unsafe fn nation_rules_compare_can_end_loan(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, name: &str, this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
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

unsafe extern "thiscall" fn compare_germany_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_GERMANY_RULES_CAN_END_LOAN);
    nation_rules_compare_can_end_loan(o, germany_rules_can_end_loan, "germany_rules_can_end_loan", this, club, player, msg, date, flag)
}

unsafe extern "thiscall" fn compare_ireland_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_IRELAND_RULES_CAN_END_LOAN);
    nation_rules_compare_can_end_loan(o, ireland_rules_can_end_loan, "ireland_rules_can_end_loan", this, club, player, msg, date, flag)
}

/// Adds 1 to the season loan count of the borrowing club when it is Irish (the count that `ireland_rules_can_loan_player`
/// limits to five; `england_rules_daily_update`, also the Irish rules' daily method, zeroes it on 20 June).
///
/// `IRELAND_RULES::record_loan(offer)`, the vtable slot `transfer_rules_vtable::RECORD_LOAN` of the Republic of Ireland,
/// from the loan signing with the borrowing club's nation. The club of the offer (id at +8, in `G_CLUB_TABLE`) with a
/// nation whose id is `KEY_NATION_REPUBLIC_OF_IRELAND`: byte 0 of its 16-byte row (`season_loans_entry`) + 1. The same
/// code as `ENGLAND_RULES::record_loan` (0x579fd0) with the Irish key.
/// Original: `IRELAND_RULES::record_loan` (0x63c4e0, ireland_rules.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ireland_rules_record_loan(_this: usize, offer: usize) {
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 8).wrapping_mul(club::SIZE)));
    let nation = club.nation().addr();
    if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) {
        covered!("mkc_6 ireland loan counted");
        let p = season_loans_entry(club);
        write_u8(p, read_u8(p).wrapping_add(1)); // mkc6: Ireland, the season count
    }
}

unsafe extern "thiscall" fn compare_ireland_rules_record_loan(this: usize, offer: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_IRELAND_RULES_RECORD_LOAN);
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 8).wrapping_mul(club::SIZE));
    let regions = [(season_loans_entry(Club(club)), 1usize)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, offer), || ireland_rules_record_loan(this, offer));
    SCOREBOARD.record(equal, || format!("ireland_rules_record_loan(offer {:#x}) original={:x?} rust={:x?}", offer, mo, mr));
}
