use super::*;

// The comparers of the routines in ../simulated_stats.rs.

/// Comparer for `simulated_stats_save`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_save(this: usize, save: usize) {
    // simplification: writes to the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SIMULATED_STATS_SAVE)(this, save)
}
