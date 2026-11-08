use super::*;

// The comparers of the routines in ../comp/cups/jap_super.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/jap_super.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `jap_super_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_jap_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_JAP_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("jap_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| jap_super_create_round_table(this, stage, r, e, s))
}

/// Comparer for `jap_super_select_teams` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_jap_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_JAP_SUPER_SELECT_TEAMS);
    compare_team_list("jap_super_select_teams", this, || o(this), || select_japanese_super_cup_teams(this))
}

/// Comparer for `jap_super_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_jap_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_SUPER_START_NEW_SEASON);
    compare_cup_new_season("jap_super_start_new_season", this, || o(this))
}
