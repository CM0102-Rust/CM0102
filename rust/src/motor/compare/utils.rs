use super::*;

// The comparers of the routines in ../utils.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../utils.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `utils_clear_temp_folder` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_utils_clear_temp_folder(default: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_UTILS_CLEAR_TEMP_FOLDER)(default)
}
