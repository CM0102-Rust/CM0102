use super::*;

// The comparers of the routines in ../comp/eurocomp/asia_super_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/asia_super_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `asia_super_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_asia_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_SUPER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("asia_super_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| asia_super_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `asia_super_cup_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_asia_super_cup_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ASIA_SUPER_CUP_SELECT_TEAMS);
    compare_team_list("asia_super_cup_select_teams", Cup(this), || o(this), || select_asian_super_cup_teams(this))
}
