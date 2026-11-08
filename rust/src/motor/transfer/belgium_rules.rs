use super::*;

/// the Belgium rule (3): three windows, from 1 June to 30 June; from 1 June to 31 December; from 1 June to 31 March
/// Original: `BELGIUM_RULES::init_transfer_windows` (0x424c00, belgium_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x424c00 } (thiscall, ret 0, belgium_rules.cpp, `BELGIUM_RULES::init_transfer_windows`; chamada pelo construtor das regras da Bélgica (regra 3), depois da base e da vtable): as
/// 3 janelas de transferência de `BELGIUM_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn belgium_rules_init_transfer_windows(this: usize) {
    crate::comparison::branch("cqn_2 belgica");
    set_transfer_windows(this, 3, &BELGIUM_TRANSFER_WINDOWS, 0x989e34, 0x53);
}
