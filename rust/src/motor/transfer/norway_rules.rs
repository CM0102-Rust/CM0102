use super::*;

/// Constructs a nation's transfer rules: base TRANSFER_RULES ctor, the nation vtable and the transfer windows.
pub(crate) unsafe fn construct_nation_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], file: usize, row: u32) {
    exe::new_transfer_rules(this, a[0], a[1], a[2], a[3], a[4]);
    write_u32(this, vtable);
    set_transfer_windows(this, n, t, file, row);
}

/// As janelas da Noruega: uma janela, de 1º de setembro a 31 de agosto.
pub(crate) const NORWAY_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x11, 0x00, 0xff, 0x01, 0x08, 0x01],
    [0x11, 0x00, 0xff, 0x1f, 0x07, 0x00], // cqm2: a Noruega
];

/// Constructs the Norway transfer rules (rule 17): one window, from 1 September to 31 August.
///
/// `NORWAY_RULES::NORWAY_RULES`; SEH frame only for the `new`, no destructor on the normal path: the transfer rules of
/// Norway (rule 17): the base (`exe::new_transfer_rules`), the vtable 0x96dfe4, 1 window from `NORWAY_TRANSFER_WINDOWS`. Returns the
/// object.
/// Original: `NORWAY_RULES::NORWAY_RULES` (0x7944f0, norway_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_norway_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_2 norway");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96dfe4, 1, &NORWAY_TRANSFER_WINDOWS, 0xa69870, 0x51);
    this
}

/// Northern Ireland's transfer windows: one window, from 6 May (moved to the Saturday [inference]) to 20 March.
pub(crate) const NORTHERN_IRELAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x10, 0x00, 0x06, 0x06, 0x04, 0x01], // cqm2: Northern Ireland
    [0x10, 0x00, 0xff, 0x14, 0x02, 0x00],
];

/// The transfer windows of Wales: one window, from 7 May (moving to the Sunday [inference]) to 20 March.
pub(crate) const WALES_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x1a, 0x00, 0x00, 0x07, 0x04, 0x01], // cqm2: Wales
    [0x1a, 0x00, 0xff, 0x14, 0x02, 0x00],
];

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

pub(crate) unsafe extern "thiscall" fn compare_norway_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_NORWAY_RULES_CAN_LOAN_PLAYER);
    england_rules_compare_can_loan(o, norway_rules_can_loan_player, "norway_rules_can_loan_player", this, club, player, msg, reason)
}
