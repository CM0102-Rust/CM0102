use super::*;

// The comparers of the routines in ../comp/intercomp/fifa_confed.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/intercomp/fifa_confed.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fifa_confed_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FIFA_CONFED_CREATE_SCHEDULE);
    compare_phase_table("fifa_confed_create_schedule", stage, confed_cup_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| fifa_confed_create_schedule(this, stage, r, e, l))
}
