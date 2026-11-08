use super::*;

// The comparers of the routines in ../transfer/france_rules.rs.
// [pt] Os comparadores das rotinas de ../transfer/france_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `france_rules_init_transfer_windows`.
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_france_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FRANCE_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("france_rules_init_transfer_windows", this, || o(this), |s| france_rules_init_transfer_windows(s));
}
