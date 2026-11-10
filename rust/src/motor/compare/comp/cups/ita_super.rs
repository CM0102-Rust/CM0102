use super::*;

// The comparers of the routines in ../comp/cups/ita_super.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/ita_super.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_super_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_ita_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("ita_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ita_super_create_round_table(this, stage, r, e, s))
}

/// Comparer for `ita_super_init_team_list`.
pub(crate) unsafe extern "thiscall" fn compare_ita_super_init_team_list(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ITA_SUPER_INIT_TEAM_LIST);
    compare_team_list("ita_super_init_team_list", Cup(this), || o(this), || init_supercoppa_teams(this))
}
