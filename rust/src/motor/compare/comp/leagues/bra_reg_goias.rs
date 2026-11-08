use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_goias.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_goias.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_bra_reg_goias` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_goias(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_GOIAS);
    bra_reg_goias_compare_state_league_ctor("construct_bra_reg_goias", this, ano, registro, &GOIANO_STATE_CHAMPIONSHIP, || o(this, ano, registro))
}

/// Comparer for `construct_bra_reg_north` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_north(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_NORTH);
    bra_reg_goias_compare_state_league_ctor("construct_bra_reg_north", this, ano, registro, &NORTH_CHAMPIONSHIP, || o(this, ano, registro))
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

/// Comparer for `bra_reg_goias_start_new_season_435e80` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_start_new_season_435e80(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GOIAS_START_NEW_SEASON_435E80);
    compare_state_league_new_season("bra_reg_goias_start_new_season_435e80", this, || o(this))
}

/// Comparer for `bra_reg_goias_advance_stage_436030` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_advance_stage_436030(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GOIAS_ADVANCE_STAGE_436030);
    compare_state_league_advance_stage("bra_reg_goias_advance_stage_436030", this, || o(this))
}

/// Comparer for `bra_reg_goias_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GOIAS_CREATE_TABLE);
    bra_reg_goias_compare_state_league_table("bra_reg_goias_create_table", this, 0xe, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparer for `bra_reg_goias_create_table_435d80` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_create_table_435d80(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GOIAS_CREATE_TABLE_435D80);
    bra_reg_goias_compare_state_league_table("bra_reg_goias_create_table_435d80", this, 0xa, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparer for `bra_reg_goias_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GOIAS_SET_UP_RULES)(this), || {
        goiano_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_goias_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparador: como o do Goiano.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_set_up_rules_665140(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_GOIAS_SET_UP_RULES_665140)(this), || {
        j2_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_goias_set_up_rules_665140({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `bra_reg_goias_create_knockout_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_goias_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GOIAS_CREATE_KNOCKOUT_STAGE);
    bra_reg_gaucho_compare_knockout_from_table("bra_reg_goias_create_knockout_stage", this, &[0, 7, 2, 5, 6, 1, 4, 3], || o(this))
}
