use super::*;

// The comparers of the routines in ../transfer/northern_ireland_rules.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer/northern_ireland_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_northern_ireland_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_northern_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_NORTHERN_IRELAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_northern_ireland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_northern_ireland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
