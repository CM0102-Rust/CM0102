use super::*;

/// the French rule (9): three windows, from 30 May to 31 August, from 1 September to 20 December and from 21 December
/// to 10 January (the dates move to weekdays [inference])
/// Original: `FRANCE_RULES::init_transfer_windows` (0x5c15e0, france_rules.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn france_rules_init_transfer_windows(this: usize) {
    covered!("cqk_2 france");
    set_transfer_windows(this, 3, &FRANCE_TRANSFER_WINDOWS, 0x9cccf4, 0x4f);
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

pub(crate) unsafe extern "thiscall" fn compare_france_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_FRANCE_RULES_CAN_END_LOAN);
    rules_compare_can_end_loan(o, france_rules_can_end_loan, "france_rules_can_end_loan", this, club, player, msg, date, flag)
}

/// The refusal text of a loan that cannot end before the deadline of nation `key` (`exe::nation_get_nationality_adjective`
/// of `[G_NATION_TABLE] + [key] · nation::SIZE`, 100 bytes, form 10: push; add esp, 0x10 -> cdecl): the club that
/// asks is the player's own (`club_job()`): "cannot be sent back" (0x9cd030); otherwise "cannot be recalled" (0x9ccfbc).
pub(crate) unsafe fn loan_end_refusal_text(key: usize, club: usize, player: usize, msg: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut adjective = [0u8; 100];
    let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE));
    exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 100, 10);
    let format = if Staff(player).club_job().addr() == club { 0x9cd030 } else { 0x9ccfbc };
    text(msg, format, 100, 100, adjective.as_ptr());
}

/// Side-by-side comparer of a `can_end_loan`: the return and the message text (the copy starts on an emptied message and
/// the original's text is put back). Not named `compare_can_end_loan`: that is the 0x8d0530 comparer, glob-imported into
/// `motor.rs`, and a local function of the same name shadows it in the `PROLOGOS` table (the detour of 0x8d0530 then
/// lands here with the wrong arguments and calls a garbage `o`).
pub(crate) unsafe fn rules_compare_can_end_loan(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, name: &str, this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
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
