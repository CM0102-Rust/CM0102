use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_pern.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_pern.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_reg_pern_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_pern_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PERN_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_pern_start_new_season", this, || o(this))
}

/// Comparer for `bra_reg_pern_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_pern_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_pern_advance_stage", this, || o(this))
}

/// Comparer for `bra_reg_pern_create_semi_final_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_pern_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_CREATE_SEMI_FINAL_STAGE);
    bra_reg_pern_compare_knockout_from_table("bra_reg_pern_create_semi_final_stage", this, &[0, 3, 1, 2], || o(this))
}

/// Comparer for `construct_bra_reg_pern` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_reg_pern(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_PERN);
    bra_reg_northeast_compare_state_league_ctor("construct_bra_reg_pern", this, year, record, &PERNAMBUCANO_STATE_CHAMPIONSHIP, || o(this, year, record))
}

/// Comparer for `bra_reg_pern_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_pern_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PERN_CREATE_TABLE);
    bra_reg_rio_compare_state_league_table("bra_reg_pern_create_table", this, 0xa, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparador: como o do Paranaense.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_pern_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PERN_SET_UP_RULES)(this), || {
        pernambucano_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_reg_pern_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
