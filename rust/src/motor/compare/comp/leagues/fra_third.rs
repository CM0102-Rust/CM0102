use super::*;

// The comparers of the routines in ../comp/leagues/fra_third.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/fra_third.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fra_third_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_fra_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_THIRD_CREATE_SCHEDULE);
    compare_cfa_schedule("fra_third_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| fra_third_create_schedule(this, stage, r, e, l))
}

/// Comparer for `fra_third_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_fra_third_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_THIRD_SET_UP_RULES);
    fra_first_compare_league_rules("fra_third_set_up_rules", this, national_division_rules(this), || o(this), || fra_third_set_up_rules(this))
}

/// Comparer for `fra_third_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_fra_third_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_THIRD_CREATE_TABLE);
    fra_first_compare_league_table("fra_third_create_table", this, 0x14, KEY_CLUB_COMP_FRENCH_NATIONAL, Some(0x4e20), || o(this))
}

/// Comparer for `construct_fra_third`.
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_third(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_THIRD);
    compare_division_league_ctor("construct_fra_third", this, year, record, &FRENCH_THIRD_DIVISION, || o(this, year, record))
}
