use super::*;

/// As janelas da Turquia: uma janela, de 1º de junho a 31 de janeiro.
pub(crate) const TURKEY_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x18, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm3: a Turquia
    [0x18, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// Constructs the Turkey transfer rules (rule 24): one window, from 1 June to 31 January;
/// `transfer_rules::FOREIGN_PLAYER_LIMIT` = 8 (base default 50) [inference: the foreign player limit];
/// `transfer_rules::NON_EU_PLAYER_LIMIT` = 8 (default 30).
///
/// Ret 0x14, `TURKEY_RULES::TURKEY_RULES`; SEH frame only for the `new`, no destructor on the normal path: the base
/// (`exe::new_transfer_rules`, through `construct_nation_rules`), vtable 0x97075c, 1 window from
/// `TURKEY_TRANSFER_WINDOWS`; `transfer_rules::FOREIGN_PLAYER_LIMIT` = 8 (the base default is 50) [inference: the
/// foreign player limit]; `transfer_rules::NON_EU_PLAYER_LIMIT` = 8 (the default is 30). Returns the object.
/// Original: `TURKEY_RULES::TURKEY_RULES` (0x9020a0, turkey_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_turkey_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_3 turkey");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x97075c, 1, &TURKEY_TRANSFER_WINDOWS, 0xaa19a8, 0x52);
    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 8);
    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 8);
    this
}

/// As janelas da Polônia: uma janela, de 1º de julho a 20 de fevereiro.
pub(crate) const POLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x12, 0x00, 0xff, 0x01, 0x06, 0x01], // cqm3: a Polônia
    [0x12, 0x00, 0xff, 0x14, 0x01, 0x00],
];

/// As janelas da Finlândia: uma janela, de 16 de novembro a 15 de agosto.
pub(crate) const FINLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x08, 0x00, 0xff, 0x10, 0x0a, 0x01],
    [0x08, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqm3: a Finlândia
];

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

pub(crate) unsafe extern "thiscall" fn compare_turkey_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_TURKEY_RULES_CAN_LOAN_PLAYER);
    england_rules_compare_can_loan(o, turkey_rules_can_loan_player, "turkey_rules_can_loan_player", this, club, player, msg, reason)
}
