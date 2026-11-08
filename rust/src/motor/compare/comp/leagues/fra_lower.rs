use super::*;

// The comparers of the routines in ../comp/leagues/fra_lower.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/fra_lower.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_fra_lower` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_lower(this: usize, year: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_CONSTRUCT_FRA_LOWER);
    compare_construct_group_league("construct_fra_lower", this, year, &FRENCH_LOWER_DIVISION, || o(this, year))
}

/// Comparer for `destroy_fra_lower` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_destroy_fra_lower(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_FRA_LOWER)(this)
}

/// Comparer for `fra_lower_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_fra_lower_create_table(this: usize, clubs: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FRA_LOWER_CREATE_TABLE);
    compare_french_table("fra_lower_create_table", this, clubs, 0x14, &[KEY_CLUB_COMP_FRENCH_NATIONAL], || o(this, clubs))
}

/// Comparer for `fra_lower_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_fra_lower_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_LOWER_CREATE_SCHEDULE);
    compare_cfa_schedule("fra_lower_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| fra_lower_create_schedule(this, stage, r, e, l))
}
