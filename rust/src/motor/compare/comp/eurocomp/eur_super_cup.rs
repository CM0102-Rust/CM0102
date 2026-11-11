use super::*;

// The comparers of the routines in ../comp/eurocomp/eur_super_cup.rs.

/// Comparer for `eur_super_cup_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_eur_super_cup_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_EUR_SUPER_CUP_SELECT_TEAMS);
    compare_team_list("eur_super_cup_select_teams", Cup(this), || o(this), || select_european_super_cup_teams(Cup(this)))
}

/// Comparer for `eur_super_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_eur_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EUR_SUPER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("eur_super_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| eur_super_cup_create_round_table(this, stage, r, e, s))
}
