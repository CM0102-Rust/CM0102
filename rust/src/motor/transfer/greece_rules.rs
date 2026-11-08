use super::*;

// ---------- 277a leva: as regras de transferência da Grécia, da Rússia e da Bélgica: as janelas de transferência (belgium_rules.cpp, greece_rules.cpp, russia_rules.cpp) ----------




/// As janelas da Grécia: duas janelas, de 22 de maio (andando para o domingo [inferência]) a 29 de agosto (andando para o sábado [inferência]); de 1º de janeiro a 20 de janeiro.
pub(crate) const CQN2_GRECIA: [[u8; 6]; 4] = [
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
/// 2 janelas de transferência de `CQN2_GRECIA`.
pub(crate) unsafe extern "thiscall" fn janelas_grecia(this: usize) {
    crate::comparacao::ramo("cqn_2 grecia");
    cqk1_janelas(this, 2, &CQN2_GRECIA, 0x9cdbc8, 0x51);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_janelas_grecia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_JANELAS_GRECIA);
    cqk1_comparar("janelas_grecia", this, || o(this), |s| janelas_grecia(s));
}

/// As janelas da Rússia: duas janelas, de 15 de dezembro a 24 de março (andando para a quinta-feira [inferência]); de 28 de junho (andando para a terça-feira [inferência]) a 4 de agosto.
pub(crate) const CQN2_RUSSIA: [[u8; 6]; 4] = [
    [0x14, 0x00, 0xff, 0x0f, 0x0b, 0x01],
    [0x14, 0x00, 0x04, 0x18, 0x02, 0x00],
    [0x14, 0x01, 0x02, 0x1c, 0x05, 0x01],
    [0x14, 0x01, 0xff, 0x04, 0x07, 0x00], // cqn2: a Rússia
];

/// the Russia rule (20): two windows, from 15 December to 24 March (moved to the Thursday [inference]); from 28 June
/// (moved to the Tuesday [inference]) to 4 August
/// Original: `RUSSIA_RULES::init_transfer_windows` (0x7ec7e0, russia_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x7ec7e0 } (thiscall, ret 0, russia_rules.cpp, `RUSSIA_RULES::init_transfer_windows`; chamada pelo construtor das regras da Rússia (regra 20), depois da base e da vtable): as
/// 2 janelas de transferência de `CQN2_RUSSIA`.
pub(crate) unsafe extern "thiscall" fn janelas_russia(this: usize) {
    crate::comparacao::ramo("cqn_2 russia");
    cqk1_janelas(this, 2, &CQN2_RUSSIA, 0xa7ad38, 0x4d);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_janelas_russia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_JANELAS_RUSSIA);
    cqk1_comparar("janelas_russia", this, || o(this), |s| janelas_russia(s));
}

/// As janelas da Bélgica: três janelas, de 1º de junho a 30 de junho; de 1º de junho a 31 de dezembro; de 1º de junho a 31 de março.
pub(crate) const CQN2_BELGICA: [[u8; 6]; 6] = [
    [0x03, 0x00, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x00, 0xff, 0x1e, 0x05, 0x00],
    [0x03, 0x01, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x01, 0xff, 0x1f, 0x0b, 0x00], // cqn2: a Bélgica
    [0x03, 0x02, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x02, 0xff, 0x1f, 0x02, 0x00],
];

/// the Belgium rule (3): three windows, from 1 June to 30 June; from 1 June to 31 December; from 1 June to 31 March
/// Original: `BELGIUM_RULES::init_transfer_windows` (0x424c00, belgium_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x424c00 } (thiscall, ret 0, belgium_rules.cpp, `BELGIUM_RULES::init_transfer_windows`; chamada pelo construtor das regras da Bélgica (regra 3), depois da base e da vtable): as
/// 3 janelas de transferência de `CQN2_BELGICA`.
pub(crate) unsafe extern "thiscall" fn janelas_belgica(this: usize) {
    crate::comparacao::ramo("cqn_2 belgica");
    cqk1_janelas(this, 3, &CQN2_BELGICA, 0x989e34, 0x53);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_janelas_belgica(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_JANELAS_BELGICA);
    cqk1_comparar("janelas_belgica", this, || o(this), |s| janelas_belgica(s));
}

