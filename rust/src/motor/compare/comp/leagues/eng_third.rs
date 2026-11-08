use super::*;

// The comparers of the routines in ../comp/leagues/eng_third.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/eng_third.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eng_third_create_play_off_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_eng_third_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_THIRD_CREATE_PLAY_OFF_STAGE);
    compare_playoff_stage("eng_third_create_play_off_stage", this, [6, 3, 5, 4], || o(this))
}

/// Comparer for `construct_eng_third` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_third(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_THIRD);
    compare_english_constructor("construct_eng_third", this, year, record, &THIRD_DIVISION, || o(this, year, record))
}

/// Comparer for `eng_third_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_eng_third_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_THIRD_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_third_set_up_rules", this, third_division_rules(this), || o(this), || eng_third_set_up_rules(this))
}

/// Comparer for `eng_third_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_eng_third_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_THIRD_CREATE_TABLE);
    eng_prm_compare_league_table("eng_third_create_table", this, 0x18, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || o(this))
}

/// Comparer for `eng_third_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_eng_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_THIRD_CREATE_SCHEDULE);
    compare_league_round_table("eng_third_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| eng_third_create_schedule(this, stage, r, e, l))
}

/// Comparer for `eng_third_set_club_progress` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_eng_third_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_ENG_THIRD_SET_CLUB_PROGRESS);
    compare_club_stage_status("eng_third_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || eng_third_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparer for `eng_third_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_eng_third_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_THIRD_ADVANCE_STAGE);
    ita_ser_c1a_compare_advance_stage("eng_third_advance_stage", this, || o(this), || eng_third_advance_stage(this))
}
