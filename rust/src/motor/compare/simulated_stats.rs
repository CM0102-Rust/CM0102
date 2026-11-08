use super::*;

// The comparers of the routines in ../simulated_stats.rs.
// [pt] Os comparadores das rotinas de ../simulated_stats.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `simulated_stats_save`.
///
/// [pt] Comparador da 0x84c680: só a original.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_save(this: usize, save: usize) {
    // simplification: writes to the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SIMULATED_STATS_SAVE)(this, save)
}
