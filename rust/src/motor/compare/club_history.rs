use super::*;

// The comparers of the routines in ../club_history.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../club_history.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `club_history_save` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x443f50: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_club_history_save(this: usize, save: usize) -> u32 {
    // simplification: writes to the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLUB_HISTORY_SAVE)(this, save)
}

