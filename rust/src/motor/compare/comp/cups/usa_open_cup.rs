use super::*;

// The comparers of the routines in ../comp/cups/usa_open_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/usa_open_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `usa_open_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_usa_open_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_USA_OPEN_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("usa_open_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| usa_open_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `usa_open_cup_collect_entrants` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_usa_open_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_USA_OPEN_CUP_COLLECT_ENTRANTS);
    jap_j_cup_compare_entrant_list("usa_open_cup_collect_entrants", this, || o(this), || collect_us_open_cup_entrants(this))
}

/// Comparer for `usa_open_cup_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_usa_open_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_OPEN_CUP_START_NEW_SEASON);
    compare_cup_new_season("usa_open_cup_start_new_season", this, || o(this))
}
