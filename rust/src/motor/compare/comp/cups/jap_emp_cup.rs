use super::*;

// The comparers of the routines in ../comp/cups/jap_emp_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/jap_emp_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `jap_emp_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_jap_emp_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_EMP_CUP_START_NEW_SEASON);
    compare_cup_new_season("jap_emp_cup_start_new_season", this, || o(this))
}

/// Comparer for `jap_emp_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_jap_emp_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_JAP_EMP_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("jap_emp_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| jap_emp_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `jap_emp_cup_collect_entrants`.
pub(crate) unsafe extern "thiscall" fn compare_jap_emp_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_JAP_EMP_CUP_COLLECT_ENTRANTS);
    jap_j_cup_compare_entrant_list("jap_emp_cup_collect_entrants", this, || o(this), || collect_emperors_cup_entrants(this))
}
