use super::*;

/// The Brazilian transfer window: from 11 December (moving to the Wednesday [inference]) to 8 November (moving to the
/// Friday [inference]).
///
/// [pt] A janela do Brasil: de 11 de dezembro (andando para a quarta-feira [inferência]) a 8 de novembro (andando para a sexta-feira [inferência]).
pub(crate) const BRAZIL_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x04, 0x00, 0x03, 0x0b, 0x0b, 0x01], // cqk3: Brazil opens in December
    [0x04, 0x00, 0x05, 0x08, 0x0a, 0x00], // cqk3: Brazil closes in November
];

/// the Brazilian rule (4): one window, from 11 December (moved to the Wednesday [inference]) to 8 November (moved to
/// the Friday [inference]): closed only from November to December; +0x13 = 3 (default 50) [inference: the foreign
/// signing limit]
/// Original: `BRAZIL_RULES::BRAZIL_RULES` (0x43f0e0, brazil_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x43f0e0 } (thiscall, ret 0x14, brazil_rules.cpp, `BRAZIL_RULES::BRAZIL_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência do Brasil (regra 4): a base (0x8e84a0), a vtable 0x968838, 1 janela(s) de `BRAZIL_TRANSFER_WINDOWS` e o +0x13 = 3 (o padrão da base é 50) [inferência: o limite de estrangeiros contratados]. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_brazil_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqk_3 brazil");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x968838, 1, &BRAZIL_TRANSFER_WINDOWS, 0x98b104, 0x50);
    *((this + 0x13) as *mut u8) = 3; // cqk3: the Brazil limit
    this
}

/// As janelas da Alemanha: de 1º de julho a 15 de agosto e de 1º de julho a 15 de janeiro.
pub(crate) const GERMANY_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x0a, 0x00, 0xff, 0x01, 0x06, 0x01],
    [0x0a, 0x00, 0xff, 0x0f, 0x07, 0x00],
    [0x0a, 0x01, 0xff, 0x01, 0x06, 0x01],
    [0x0a, 0x01, 0xff, 0x0f, 0x00, 0x00],
];

/// Scotland's transfer window: from 30 May (moved to the Saturday [inference]) to 30 March (moved to the Thursday
/// [inference]).
///
/// [pt] A janela da Escócia: de 30 de maio (andando para o sábado [inferência]) a 30 de março (andando para a quinta-feira [inferência]).
pub(crate) const SCOTLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x15, 0x00, 0x06, 0x1e, 0x04, 0x01],
    [0x15, 0x00, 0x04, 0x1e, 0x02, 0x00],
];

