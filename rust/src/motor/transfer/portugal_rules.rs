use super::*;

/// the Portugal rule (19): two windows, from 1 June to 1 September; from 15 December to 15 January
/// Original: `PORTUGAL_RULES::init_transfer_windows` (0x7d3df0, portugal_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x7d3df0 } (thiscall, ret 0, portugal_rules.cpp, `PORTUGAL_RULES::init_transfer_windows`; chamada pelo construtor das regras de Portugal (regra 19), depois da base e da vtable): as
/// 2 janelas de transferência de `PORTUGAL_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn portugal_rules_init_transfer_windows(this: usize) {
    covered!("cqn_1 portugal");
    set_transfer_windows(this, 2, &PORTUGAL_TRANSFER_WINDOWS, 0xa78844, 0x4f);
}
