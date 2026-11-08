use super::*;

/// the French rule (9): three windows, from 30 May to 31 August, from 1 September to 20 December and from 21 December
/// to 10 January (the dates move to weekdays [inference])
/// Original: `FRANCE_RULES::init_transfer_windows` (0x5c15e0, france_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x5c15e0 } (thiscall, ret 0, france_rules.cpp, `FRANCE_RULES::init_transfer_windows`; chamada pelo construtor das regras da França (regra 9), depois da base e da vtable): as
/// 3 janelas de transferência de `FRANCE_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn france_rules_init_transfer_windows(this: usize) {
    crate::comparison::branch("cqk_2 france");
    set_transfer_windows(this, 3, &FRANCE_TRANSFER_WINDOWS, 0x9cccf4, 0x4f);
}
