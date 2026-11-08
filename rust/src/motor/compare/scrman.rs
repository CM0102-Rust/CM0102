use super::*;

// The comparers of the routines in ../scrman.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../scrman.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `is_forward_unavailable` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_forward_unavailable(manager: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IS_FORWARD_UNAVAILABLE)(manager);
    let n = is_forward_unavailable(manager);
    PLACAR.record(r == n, || format!("is_forward_unavailable original={} rust={}", r, n));
    r
}
