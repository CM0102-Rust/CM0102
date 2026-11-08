use super::*;

// The comparers of the routines in ../comp/cups/spa_super.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/spa_super.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_super_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_spa_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_SUPER_SELECT_TEAMS);
    compare_team_list("spa_super_select_teams", this, || o(this), || select_spanish_super_cup_teams(this))
}

/// Comparer for `spa_super_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_spa_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("spa_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| spa_super_create_round_table(this, stage, r, e, s))
}
