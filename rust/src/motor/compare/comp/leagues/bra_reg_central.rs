use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_central.rs.

/// Comparer for `bra_reg_central_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_central_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_CENTRAL_SET_UP_RULES)(this), || {
        centro_oeste_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_reg_central_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `bra_reg_central_update_staff_records_for_status`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_central_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let (n, l, dest) = centro_oeste_end_of_stage(this, club, status, stage, game, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_BRA_REG_CENTRAL_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, game, a6);
    let written = if l == 0 { dest } else { read_u8(l + league_table_row::STATUS) };
    SCOREBOARD.record(r == n && written == dest, || format!("bra_reg_central_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} target {} x {}", this, club, status as u8 as i8, stage as u8 as i8, r as i32, n as i32, written as i8, dest as i8));
    r
}

/// Comparer for `construct_bra_reg_central`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_reg_central(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_CENTRAL);
    bra_reg_bahia_compare_state_league_ctor("construct_bra_reg_central", League(this), year, record, &CENTRO_OESTE_STATE_CHAMPIONSHIP, || o(this, year, record))
}

/// Comparer for `bra_reg_central_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_central_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_CENTRAL_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_central_start_new_season", League(this), || o(this))
}

/// Comparer for `bra_reg_central_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_central_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_CENTRAL_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_central_advance_stage", League(this), || o(this))
}

/// Comparer for `bra_reg_central_load_region_clubs`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_central_load_region_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_CENTRAL_LOAD_REGION_CLUBS);
    bra_reg_rio_compare_state_league_table("bra_reg_central_load_region_clubs", League(this), 0xf, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparer for `bra_reg_central_create_knockout_stage`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_central_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_CENTRAL_CREATE_KNOCKOUT_STAGE);
    bra_reg_pern_compare_knockout_from_table("bra_reg_central_create_knockout_stage", League(this), &[0, 7, 2, 5, 6, 1, 4, 3], || o(this))
}
