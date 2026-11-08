use super::*;

// The comparers of the routines in ../comp/leagues/ita_ser_c1b.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/ita_ser_c1b.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_ser_c1b_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_c1b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_SER_C1B_CREATE_SCHEDULE);
    compare_c1_schedule("ita_ser_c1b_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ita_ser_c1b_create_schedule(this, stage, r, e, l))
}

/// Comparer for `ita_ser_c1b_set_club_progress` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_c1b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_ITA_SER_C1B_SET_CLUB_PROGRESS);
    compare_c1_club_progress("ita_ser_c1b_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || ita_ser_c1b_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparer for `ita_ser_c1b_set_up_fixed_fixtures` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_c1b_set_up_fixed_fixtures(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_C1B_SET_UP_FIXED_FIXTURES);
    fra_first_compare_fixed_fixtures("ita_ser_c1b_set_up_fixed_fixtures", this, || o(this))
}

/// Comparer for `construct_ita_ser_c1b` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_ita_ser_c1b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_SER_C1B);
    compare_italian_constructor("construct_ita_ser_c1b", this, year, record, &SERIE_C1B_DIVISION, || o(this, year, record))
}

/// Comparer for `ita_ser_c1b_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_c1b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_C1B_SET_UP_RULES);
    ita_ser_a_compare_league_rules("ita_ser_c1b_set_up_rules", this, serie_c1b_rules(this), || o(this), || ita_ser_c1b_set_up_rules(this))
}

/// Comparer for `ita_ser_c1b_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_c1b_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_C1B_CREATE_TABLE);
    ita_ser_a_compare_league_table("ita_ser_c1b_create_table", this, 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, None, || o(this))
}

/// Comparer for `ita_ser_c1b_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_c1b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_C1B_ADVANCE_STAGE);
    ita_ser_c1a_compare_advance_stage("ita_ser_c1b_advance_stage", this, || o(this), || ita_ser_c1b_advance_stage(this))
}

/// Comparer for `create_play_off_stages` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_play_off_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_PLAY_OFF_STAGES);
    compare_end_of_league_stages("create_play_off_stages", this, || o(this))
}
