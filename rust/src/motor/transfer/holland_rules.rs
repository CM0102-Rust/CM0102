use super::*;

/// The Netherlands' transfer windows: one window, from 8 June (moved to the Saturday [inference]) to 4 April (moved to
/// the Friday [inference]).
pub(crate) const HOLLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x0c, 0x00, 0x06, 0x08, 0x05, 0x01], // cqm1: a Holanda
    [0x0c, 0x00, 0x05, 0x04, 0x03, 0x00],
];

/// the Holland rule (12): one window, from 8 June (moved to the Saturday [inference]) to 4 April (moved to the Friday
/// [inference]); `transfer_rules::FOREIGN_PLAYER_LIMIT` = 50 (base default 50) [inference: the foreign limit];
/// `transfer_rules::NON_EU_PLAYER_LIMIT` = 50 (default 30)
/// Original: `HOLLAND_RULES::HOLLAND_RULES` (0x5f9690, holland_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_holland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_1 netherlands");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96ba88, 1, &HOLLAND_TRANSFER_WINDOWS, 0x9cf04c, 0x52);
    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 50);
    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 50);
    this
}

/// As janelas da Croácia: uma janela, de 1º de junho a 30 de março.
pub(crate) const CROATIA_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x05, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm1: a Croácia
    [0x05, 0x00, 0xff, 0x1e, 0x02, 0x00],
];

/// As janelas da Irlanda: uma janela, de 10 de maio a 31 de janeiro.
pub(crate) const IRELAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x0d, 0x00, 0xff, 0x0a, 0x04, 0x01], // cqm1: a Irlanda
    [0x0d, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

