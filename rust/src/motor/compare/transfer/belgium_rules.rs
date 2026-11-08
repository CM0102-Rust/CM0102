use super::*;

// The comparers of the routines in ../transfer/belgium_rules.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer/belgium_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `belgium_rules_init_transfer_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_belgium_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BELGIUM_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("belgium_rules_init_transfer_windows", this, || o(this), |s| belgium_rules_init_transfer_windows(s));
}
