use super::*;

// The comparers of the routines in ../transfer/argentina_rules.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer/argentina_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `argentina_rules_init_transfer_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_argentina_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ARGENTINA_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("argentina_rules_init_transfer_windows", this, || o(this), |s| argentina_rules_init_transfer_windows(s));
}

/// Comparer for `argentina_rules_init_transfer_windows_7d3df0` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_argentina_rules_init_transfer_windows_7d3df0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ARGENTINA_RULES_INIT_TRANSFER_WINDOWS_7D3DF0);
    compare_transfer_rules_ctor("argentina_rules_init_transfer_windows_7d3df0", this, || o(this), |s| argentina_rules_init_transfer_windows_7d3df0(s));
}

/// Comparer for `construct_korea_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_korea_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_KOREA_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_korea_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_korea_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
