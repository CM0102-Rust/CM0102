// ---------- %%LEVA%%a leva: ending a loan in Germany and Ireland, the Irish season loan count (germany_rules.cpp, ireland_rules.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(%%I0%%);
    nation_rules_compare_can_end_loan(o, germany_rules_can_end_loan, "germany_rules_can_end_loan", this, club, player, msg, date, flag)
}

unsafe extern "thiscall" fn compare_ireland_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 8).wrapping_mul(club::SIZE));
    let regions = [(season_loans_entry(Club(club)), 1usize)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, offer), || ireland_rules_record_loan(this, offer));
    SCOREBOARD.record(equal, || format!("ireland_rules_record_loan(offer {:#x}) original={:x?} rust={:x?}", offer, mo, mr));
}
