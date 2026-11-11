use super::*;

/// Argentina's transfer windows: two windows, from 24 July (moved to the Sunday [inference]) to 6 August (moved to the
/// Wednesday [inference]); and from 14 December (moved to the Sunday [inference]) to 4 March (moved to the Wednesday
/// [inference]).
pub(crate) const ARGENTINA_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x01, 0x00, 0x00, 0x18, 0x06, 0x01],
    [0x01, 0x00, 0x03, 0x06, 0x07, 0x00], // cqn1: a Argentina
    [0x01, 0x01, 0x00, 0x0e, 0x0b, 0x01],
    [0x01, 0x01, 0x03, 0x04, 0x02, 0x00],
];

/// the Argentina rule (1): two windows, from 24 July (moved to the Sunday [inference]) to 6 August (moved to the
/// Wednesday [inference]); from 14 December (moved to the Sunday [inference]) to 4 March (moved to the Wednesday
/// [inference])
/// Original: `ARGENTINA_RULES::init_transfer_windows` (0x40a620, argentina_rules.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn argentina_rules_init_transfer_windows(this: usize) {
    covered!("cqn_1 argentina");
    set_transfer_windows(this, 2, &ARGENTINA_TRANSFER_WINDOWS, 0x987960, 0x50);
}

/// As janelas de Portugal: duas janelas, de 1º de junho a 1º de setembro; de 15 de dezembro a 15 de janeiro.
pub(crate) const PORTUGAL_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x13, 0x00, 0xff, 0x01, 0x05, 0x01],
    [0x13, 0x00, 0xff, 0x01, 0x08, 0x00], // cqn1: a Portugal
    [0x13, 0x01, 0xff, 0x0f, 0x0b, 0x01],
    [0x13, 0x01, 0xff, 0x0f, 0x00, 0x00],
];

/// South Korea's transfer windows: one window, from 25 November (moved to the Sunday [inference]) to 25 July.
pub(crate) const SOUTH_KOREA_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x1b, 0x00, 0x00, 0x19, 0x0a, 0x01],
    [0x1b, 0x00, 0xff, 0x19, 0x06, 0x00], // cqn1: South Korea
];

