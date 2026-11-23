use super::*;

/// Constructs the Australian transfer rules (rule 2): one window, from 11 June (moved to the Saturday [inference]) to
/// 17 April; +0x13 = 6 [inference: the foreign squad limit].
///
/// `AUSTRALIA_RULES::Australia_rules`; SEH frame only for the `new`, no destructor on the normal path: the transfer
/// rules of Australia (rule 2): the base (`exe::new_transfer_rules`), the vtable 0x967830, 1 window from `AUSTRALIA_TRANSFER_WINDOWS`
/// and +0x13 = 6 [inference: the foreign player limit]. Returns the object.
/// Original: `AUSTRALIA_RULES::Australia_rules` (0x412c80, australia_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn australia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqk_1 australia");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x967830, 1, &AUSTRALIA_TRANSFER_WINDOWS, 0x987cfc, 0x51);
    write_u8(this + 0x13, 6);
    this
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

pub(crate) unsafe extern "thiscall" fn compare_australia_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_AUSTRALIA_RULES_CAN_LOAN_PLAYER);
    england_rules_compare_can_loan(o, australia_rules_can_loan_player, "australia_rules_can_loan_player", this, club, player, msg, reason)
}
