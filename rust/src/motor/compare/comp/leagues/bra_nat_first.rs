use super::*;

// The comparers of the routines in ../comp/leagues/bra_nat_first.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_nat_first.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_nat_first_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_nat_first_advance_stage", this, || o(this))
}

/// Comparer for `bra_nat_first_create_play_off_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_CREATE_PLAY_OFF_STAGE);
    compare_play_off_stage("bra_nat_first_create_play_off_stage", this, &[0, 3, 1, 2], || o(this))
}

/// Comparer for `bra_nat_first_advance_stage_42cdd0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_advance_stage_42cdd0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_ADVANCE_STAGE_42CDD0);
    compare_state_league_advance_stage("bra_nat_first_advance_stage_42cdd0", this, || o(this))
}

/// Comparer for `bra_nat_first_create_play_off_stage_42ce60` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_create_play_off_stage_42ce60(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_CREATE_PLAY_OFF_STAGE_42CE60);
    compare_play_off_stage("bra_nat_first_create_play_off_stage_42ce60", this, &[0, 3, 1, 2], || o(this))
}

/// Comparer for `bra_nat_first_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("bra_nat_first_start_new_season", this, || o(this))
}

/// Comparer for `bra_nat_first_process_promotion_relegation_chain` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador do acesso e rebaixamento: só a original (simplification: move clubes entre as listas das ligas, que realocam).
/// A cópia roda no modo Rust (contador "cqq_8 acesso e rebaixamento") e é provada pelas mutações e pelas carreiras.
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_process_promotion_relegation_chain(this: usize, pares: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_BRA_NAT_FIRST_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pares)
}

/// Comparer for `bra_nat_first_start_new_season_42cc10` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_start_new_season_42cc10(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_START_NEW_SEASON_42CC10);
    compare_state_league_new_season("bra_nat_first_start_new_season_42cc10", this, || o(this))
}

/// Comparer for `bra_nat_first_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_CREATE_TABLE);
    compare_division_table("bra_nat_first_create_table", this, 0x1c, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, || o(this))
}

/// Comparer for `bra_nat_first_create_table_42ca70` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_create_table_42ca70(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_CREATE_TABLE_42CA70);
    compare_division_table("bra_nat_first_create_table_42ca70", this, 0x1c, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, || o(this))
}

/// Comparer for `bra_nat_first_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto inteiro
/// menos as saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_SET_UP_RULES)(this), || {
        set_serie_a_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_nat_first_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `bra_nat_first_set_up_rules_42cb70` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: como o da Série A.
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_set_up_rules_42cb70(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_SET_UP_RULES_42CB70)(this), || {
        set_serie_b_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_nat_first_set_up_rules_42cb70({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `bra_nat_first_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (simplification: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação): a devolução e o destino gravado na linha.
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_update_staff_records_for_status(this: usize, clube: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, l, dest) = serie_a_staff_records(this, clube, status, stage, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_BRA_NAT_FIRST_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, status, stage, a4, a5, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("bra_nat_first_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, status as u8 as i8, stage as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}
