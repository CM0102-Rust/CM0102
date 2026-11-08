use super::*;

// The comparers of the routines in ../comp/eurocomp/eur_super_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/eur_super_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eur_super_cup_select_teams` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eur_super_cup_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_EUR_SUPER_CUP_SELECT_TEAMS);
    compare_team_list("eur_super_cup_select_teams", this, || o(this), || select_european_super_cup_teams(this))
}

/// Comparer for `eur_super_cup_select_teams_410a50` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eur_super_cup_select_teams_410a50(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_EUR_SUPER_CUP_SELECT_TEAMS_410A50);
    compare_team_list("eur_super_cup_select_teams_410a50", this, || o(this), || select_asian_super_cup_teams(this))
}

/// Comparer for `eur_super_cup_init_team_list` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eur_super_cup_init_team_list(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_EUR_SUPER_CUP_INIT_TEAM_LIST);
    compare_team_list("eur_super_cup_init_team_list", this, || o(this), || init_c1_super_cup_list(this))
}

/// Comparer for `eur_super_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eur_super_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EUR_SUPER_CUP_CREATE_ROUND_TABLE);
    eng_cc_cup_compare_round_table("eur_super_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| eur_super_cup_create_round_table(this, fase, r, e, s))
}
