use super::*;

// The comparers of the routines in ../transfer/russia_rules.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer/russia_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `russia_rules_init_transfer_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_russia_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RUSSIA_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("russia_rules_init_transfer_windows", this, || o(this), |s| russia_rules_init_transfer_windows(s));
}
