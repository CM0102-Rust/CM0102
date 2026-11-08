use super::*;

// The comparers of the routines in ../comp/leagues/eng_prm.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/eng_prm.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_eng_prm` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_eng_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_PRM);
    compare_english_constructor("construct_eng_prm", this, year, record, &PREMIER_DIVISION, || o(this, year, record))
}


/// Comparer for `eng_prm_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_prm_set_up_rules", this, premier_rules(), || o(this), || eng_prm_set_up_rules(this))
}


/// Comparer for `eng_prm_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("eng_prm_create_table", this, 0x14, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || o(this))
}


/// Comparer for `eng_prm_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_PRM_CREATE_SCHEDULE);
    compare_league_round_table("eng_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| eng_prm_create_schedule(this, stage, r, e, l))
}
