use super::*;

// The comparers of the routines in ../club_history.rs.

/// Comparer for `club_history_save`.
pub(crate) unsafe extern "thiscall" fn compare_club_history_save(this: usize, save: usize) -> u32 {
    // simplification: writes to the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLUB_HISTORY_SAVE)(this, save)
}

