use super::*;

/// the German rule (10): two windows, from 1 July to 15 August and from 1 July to 15 January [inference: the second for
/// free agents]
/// Original: `GERMANY_RULES::init_transfer_windows` (0x5dffe0, germany_rules.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn germany_rules_init_transfer_windows(this: usize) {
    covered!("cqk_3 germany");
    set_transfer_windows(this, 2, &GERMANY_TRANSFER_WINDOWS, 0x9cd538, 0x4d);
}

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

pub(crate) unsafe extern "thiscall" fn compare_germany_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_GERMANY_RULES_CAN_END_LOAN);
    nation_rules_compare_can_end_loan(o, germany_rules_can_end_loan, "germany_rules_can_end_loan", this, club, player, msg, date, flag)
}

/// Side-by-side comparer of a nation's `can_end_loan`: the return and the message text (the copy starts on an emptied
/// message and the original's text is put back).
pub(crate) unsafe fn nation_rules_compare_can_end_loan(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, name: &str, this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
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
