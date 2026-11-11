use super::*;

/// the Russia rule (20): two windows, from 15 December to 24 March (moved to the Thursday [inference]); from 28 June
/// (moved to the Tuesday [inference]) to 4 August
/// Original: `RUSSIA_RULES::init_transfer_windows` (0x7ec7e0, russia_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x7ec7e0 } (thiscall, ret 0, russia_rules.cpp, `RUSSIA_RULES::init_transfer_windows`; chamada pelo construtor das regras da Rússia (regra 20), depois da base e da vtable): as
/// 2 janelas de transferência de `RUSSIA_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn russia_rules_init_transfer_windows(this: usize) {
    covered!("cqn_2 russia");
    set_transfer_windows(this, 2, &RUSSIA_TRANSFER_WINDOWS, 0xa7ad38, 0x4d);
}
