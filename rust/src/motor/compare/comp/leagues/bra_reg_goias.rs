use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_goias.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_goias.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_bra_reg_goias` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_goias(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_GOIAS);
    bra_reg_goias_compare_state_league_ctor("construct_bra_reg_goias", this, year, record, &GOIANO_STATE_CHAMPIONSHIP, || o(this, year, record))
}

/// Comparer for `bra_reg_goias_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GOIAS_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_goias_start_new_season", this, || o(this))
}

/// Comparer for `bra_reg_goias_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GOIAS_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_goias_advance_stage", this, || o(this))
}

/// Comparer for `bra_reg_goias_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GOIAS_CREATE_TABLE);
    bra_reg_rio_compare_state_league_table("bra_reg_goias_create_table", this, 0xe, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparer for `bra_reg_goias_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GOIAS_SET_UP_RULES)(this), || {
        goiano_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_reg_goias_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `bra_reg_goias_create_knockout_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GOIAS_CREATE_KNOCKOUT_STAGE);
    bra_reg_pern_compare_knockout_from_table("bra_reg_goias_create_knockout_stage", this, &[0, 7, 2, 5, 6, 1, 4, 3], || o(this))
}
