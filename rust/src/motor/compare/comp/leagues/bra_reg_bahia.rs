use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_bahia.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_bahia.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_bra_reg_bahia`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_reg_bahia(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_BAHIA);
    bra_reg_bahia_compare_state_league_ctor("construct_bra_reg_bahia", League(this), year, record, &BAHIA_CHAMPIONSHIP, || o(this, year, record))
}

/// Comparer for `bra_reg_bahia_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_bahia_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_BAHIA_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_bahia_start_new_season", League(this), || o(this))
}

/// Comparer for `bra_reg_bahia_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_bahia_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_BAHIA_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_bahia_advance_stage", League(this), || o(this))
}

/// Comparer for `bra_reg_bahia_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_bahia_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_BAHIA_CREATE_TABLE);
    bra_reg_rio_compare_state_league_table("bra_reg_bahia_create_table", League(this), 0xa, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparer for `bra_reg_bahia_create_semi_final_stage`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_bahia_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_BAHIA_CREATE_SEMI_FINAL_STAGE);
    bra_reg_pern_compare_knockout_from_table("bra_reg_bahia_create_semi_final_stage", League(this), &[0, 3, 1, 2], || o(this))
}

/// Comparer for `bra_reg_bahia_update_staff_records_for_status`.
///
/// [pt] Comparador: como o da Série A.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_bahia_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let (n, l, dest) = baiano_staff_records(League(this), club, status, stage, game, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_BRA_REG_BAHIA_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, game, a6);
    let written = if l == 0 { dest } else { *((l + league_table_row::STATUS) as *const u8) };
    SCOREBOARD.record(r == n && written == dest, || format!("bra_reg_bahia_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, club, status as u8 as i8, stage as u8 as i8, r as i32, n as i32, written as i8, dest as i8));
    r
}
