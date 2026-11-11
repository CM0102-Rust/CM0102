use super::*;

// The comparers of the routines in ../utils.rs.

/// Comparer for `utils_clear_temp_folder`.
pub(crate) unsafe extern "C" fn compare_utils_clear_temp_folder(default: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_UTILS_CLEAR_TEMP_FOLDER)(default)
}
