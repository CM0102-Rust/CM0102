use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_gaucho.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_gaucho.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_reg_gaucho_create_knockout_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_gaucho_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GAUCHO_CREATE_KNOCKOUT_STAGE);
    bra_reg_gaucho_compare_knockout_from_table("bra_reg_gaucho_create_knockout_stage", this, &[0, 3, 1, 2], || o(this))
}

/// Comparer for `bra_reg_gaucho_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (simplification: grava o histórico, aloca o registro dele e pode montar a Copa dos Campeões; não
/// pode rodar duas vezes). A cópia roda no modo Rust (contador "cqj_8 historico do estadual") e é provada pelas
/// mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_gaucho_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GAUCHO_RECORD_SEASON_HONOURS)(this)
}

/// Comparer for `construct_bra_reg_gaucho` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_gaucho(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_GAUCHO);
    bra_reg_bahia_compare_state_league_ctor("construct_bra_reg_gaucho", this, year, record, &GAUCHO_STATE_LEAGUE, || o(this, year, record))
}

/// Comparer for `bra_reg_gaucho_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_gaucho_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GAUCHO_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_gaucho_start_new_season", this, || o(this))
}

/// Comparer for `bra_reg_gaucho_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_gaucho_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GAUCHO_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_gaucho_advance_stage", this, || o(this))
}

/// Comparer for `bra_reg_gaucho_load_region_clubs` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_gaucho_load_region_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GAUCHO_LOAD_REGION_CLUBS);
    bra_reg_bahia_compare_state_league_table("bra_reg_gaucho_load_region_clubs", this, 0x11, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparador: como o do Centro-Oeste.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_gaucho_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GAUCHO_SET_UP_RULES)(this), || {
        gaucho_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_gaucho_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
