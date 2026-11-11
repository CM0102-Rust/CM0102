use super::*;

/// Greece's transfer windows: two windows, from 22 May (moved to the Sunday [inference]) to 29 August (moved to the
/// Saturday [inference]); from 1 January to 20 January.
///
/// [pt] As janelas da Grécia: duas janelas, de 22 de maio (andando para o domingo [inferência]) a 29 de agosto (andando para o sábado [inferência]); de 1º de janeiro a 20 de janeiro.
pub(crate) const GREECE_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x0b, 0x00, 0x00, 0x16, 0x04, 0x01],
    [0x0b, 0x00, 0x06, 0x1d, 0x07, 0x00], // cqn2: a Grécia
    [0x0b, 0x01, 0xff, 0x01, 0x00, 0x01],
    [0x0b, 0x01, 0xff, 0x14, 0x00, 0x00],
];

/// the Greece rule (11): two windows, from 22 May (moved to the Sunday [inference]) to 29 August (moved to the Saturday
/// [inference]); from 1 January to 20 January
/// Original: `GREECE_RULES::init_transfer_windows` (0x5ecf20, greece_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x5ecf20 } (thiscall, ret 0, greece_rules.cpp, `GREECE_RULES::init_transfer_windows`; chamada pelo construtor das regras da Grécia (regra 11), depois da base e da vtable): as
/// 2 janelas de transferência de `GREECE_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn greece_rules_init_transfer_windows(this: usize) {
    covered!("cqn_2 greece");
    set_transfer_windows(this, 2, &GREECE_TRANSFER_WINDOWS, 0x9cdbc8, 0x51);
}

/// Russia's transfer windows: two windows, from 15 December to 24 March (moved to the Thursday [inference]); and from
/// 28 June (moved to the Tuesday [inference]) to 4 August.
///
/// [pt] As janelas da Rússia: duas janelas, de 15 de dezembro a 24 de março (andando para a quinta-feira [inferência]); de 28 de junho (andando para a terça-feira [inferência]) a 4 de agosto.
pub(crate) const RUSSIA_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x14, 0x00, 0xff, 0x0f, 0x0b, 0x01],
    [0x14, 0x00, 0x04, 0x18, 0x02, 0x00],
    [0x14, 0x01, 0x02, 0x1c, 0x05, 0x01],
    [0x14, 0x01, 0xff, 0x04, 0x07, 0x00], // cqn2: a Rússia
];

/// As janelas da Bélgica: três janelas, de 1º de junho a 30 de junho; de 1º de junho a 31 de dezembro; de 1º de junho a 31 de março.
pub(crate) const BELGIUM_TRANSFER_WINDOWS: [[u8; 6]; 6] = [
    [0x03, 0x00, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x00, 0xff, 0x1e, 0x05, 0x00],
    [0x03, 0x01, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x01, 0xff, 0x1f, 0x0b, 0x00], // cqn2: a Bélgica
    [0x03, 0x02, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x02, 0xff, 0x1f, 0x02, 0x00],
];

