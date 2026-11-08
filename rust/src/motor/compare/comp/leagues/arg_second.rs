use super::*;

// The comparers of the routines in ../comp/leagues/arg_second.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/arg_second.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `arg_second_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (simplification: grava o histórico e aloca o registro; não pode rodar duas vezes).
pub(crate) unsafe extern "thiscall" fn comparar_arg_second_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ARG_SECOND_RECORD_SEASON_HONOURS)(this)
}
