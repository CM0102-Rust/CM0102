use super::*;

// The comparers of the routines in ../transfer/poland_rules.rs.
// [pt] Os comparadores das rotinas de ../transfer/poland_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_poland_rules`.
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_poland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_POLAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_poland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_poland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
