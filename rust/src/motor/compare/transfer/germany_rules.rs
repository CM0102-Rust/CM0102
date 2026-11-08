use super::*;

// The comparers of the routines in ../transfer/germany_rules.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer/germany_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `germany_rules_init_transfer_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_germany_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GERMANY_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("germany_rules_init_transfer_windows", this, || o(this), |s| germany_rules_init_transfer_windows(s));
}
