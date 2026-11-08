use super::*;

// The comparers of the routines in ../comp/cups/fra_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/fra_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fra_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_fra_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("fra_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| fra_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `fra_cup_collect_entrants`.
pub(crate) unsafe extern "thiscall" fn compare_fra_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_FRA_CUP_COLLECT_ENTRANTS);
    eng_cc_cup_compare_entrant_list("fra_cup_collect_entrants", this, &[(this + cup::TEAM_COUNT, 2), (0xb67a34, 4)], || o(this), || french_cup_collect_entrants(this),
        |_, n| n.clamp(0, 0xde) as usize * 6)
}

