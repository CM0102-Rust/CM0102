use super::*;

/// As janelas da Turquia: uma janela, de 1º de junho a 31 de janeiro.
pub(crate) const TURKEY_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x18, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm3: a Turquia
    [0x18, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// Constructs the Turkey transfer rules (rule 24): one window, from 1 June to 31 January; +0x13 = 8 (base default 50)
/// [inference: the foreign player limit]; +0x14 = 8 (default 30).
///
/// Ret 0x14, `TURKEY_RULES::TURKEY_RULES`; SEH frame only for the `new`, no destructor on the normal path: the base
/// (`exe::new_transfer_rules`), vtable 0x97075c, 1 window from `TURKEY_TRANSFER_WINDOWS`; +0x13 = 8 (the base default is 50) [inference:
/// the foreign player limit]; +0x14 = 8 (the default is 30). Returns the object.
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

