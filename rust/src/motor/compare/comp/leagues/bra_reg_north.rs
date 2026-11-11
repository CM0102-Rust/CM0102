use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_north.rs.

/// Comparer for `bra_reg_north_create_semi_final_stage`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_north_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_NORTH_CREATE_SEMI_FINAL_STAGE);
    bra_reg_pern_compare_knockout_from_table("bra_reg_north_create_semi_final_stage", League(this), &[0, 3, 1, 2], || o(this))
}

/// Comparer for `construct_bra_reg_north`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_reg_north(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_NORTH);
    bra_reg_goias_compare_state_league_ctor("construct_bra_reg_north", League(this), year, record, &NORTH_CHAMPIONSHIP, || o(this, year, record))
}

/// Comparer for `bra_reg_north_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_north_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_NORTH_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_north_start_new_season", League(this), || o(this))
}

/// Comparer for `bra_reg_north_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_north_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_NORTH_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_north_advance_stage", League(this), || o(this))
}

/// Comparer for `bra_reg_north_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_north_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_NORTH_CREATE_TABLE);
    bra_reg_rio_compare_state_league_table("bra_reg_north_create_table", League(this), 0xa, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, || o(this))
}
