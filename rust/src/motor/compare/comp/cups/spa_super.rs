use super::*;

// The comparers of the routines in ../comp/cups/spa_super.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/spa_super.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_super_select_teams` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spa_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_SUPER_SELECT_TEAMS);
    compare_team_list("spa_super_select_teams", this, || o(this), || select_spanish_super_cup_teams(this))
}

/// Comparer for `spa_super_select_teams_666160` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spa_super_select_teams_666160(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_SUPER_SELECT_TEAMS_666160);
    compare_team_list("spa_super_select_teams_666160", this, || o(this), || select_japanese_super_cup_teams(this))
}

/// Comparer for `spa_super_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spa_super_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_SUPER_CREATE_ROUND_TABLE);
    fra_super_compare_round_table("spa_super_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| spa_super_create_round_table(this, fase, r, e, s))
}
