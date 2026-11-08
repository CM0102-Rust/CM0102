use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_central.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_central.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_reg_central_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_central_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_CENTRAL_SET_UP_RULES)(this), || {
        centro_oeste_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_central_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparador: como o do Centro-Oeste.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_central_set_up_rules_43bc70(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_CENTRAL_SET_UP_RULES_43BC70)(this), || {
        catarinense_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_central_set_up_rules_43bc70({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `bra_reg_central_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (simplification: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação, rodada antes): a devolução e o destino na linha.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_central_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = centro_oeste_end_of_stage(this, clube, situacao, fase, jogo, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_BRA_REG_CENTRAL_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("bra_reg_central_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// Comparer for `construct_bra_reg_central` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_central(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_CENTRAL);
    bra_reg_bahia_compare_state_league_ctor("construct_bra_reg_central", this, ano, registro, &CENTRO_OESTE_STATE_CHAMPIONSHIP, || o(this, ano, registro))
}

/// Comparer for `bra_reg_central_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_central_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_CENTRAL_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_central_start_new_season", this, || o(this))
}

/// Comparer for `bra_reg_central_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_central_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_CENTRAL_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_central_advance_stage", this, || o(this))
}

/// Comparer for `bra_reg_central_load_region_clubs` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_central_load_region_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_CENTRAL_LOAD_REGION_CLUBS);
    bra_reg_bahia_compare_state_league_table("bra_reg_central_load_region_clubs", this, 0xf, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparer for `bra_reg_central_create_knockout_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_central_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_CENTRAL_CREATE_KNOCKOUT_STAGE);
    bra_reg_gaucho_compare_knockout_from_table("bra_reg_central_create_knockout_stage", this, &[0, 7, 2, 5, 6, 1, 4, 3], || o(this))
}
