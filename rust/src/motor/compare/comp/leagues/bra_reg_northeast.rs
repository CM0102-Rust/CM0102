use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_northeast.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_northeast.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_bra_reg_northeast` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_northeast(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_NORTHEAST);
    bra_reg_northeast_compare_state_league_ctor("construct_bra_reg_northeast", this, year, record, &NORDESTE_STATE_LEAGUE, || o(this, year, record))
}

/// Comparer for `bra_reg_northeast_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_northeast_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_NORTHEAST_CREATE_TABLE);
    bra_reg_rio_compare_state_league_table("bra_reg_northeast_create_table", this, 0x10, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparer for `bra_reg_northeast_create_semi_final_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_northeast_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_NORTHEAST_CREATE_SEMI_FINAL_STAGE);
    bra_reg_pern_compare_knockout_from_table("bra_reg_northeast_create_semi_final_stage", this, &[0, 3, 1, 2], || o(this))
}

/// Comparer for `bra_reg_northeast_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_northeast_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_NORTHEAST_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_northeast_start_new_season", this, || o(this))
}

/// Comparer for `bra_reg_northeast_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_northeast_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_NORTHEAST_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_northeast_advance_stage", this, || o(this))
}
