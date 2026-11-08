use super::*;

// The comparers of the routines in ../comp/leagues/eng_first.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/eng_first.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eng_first_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_eng_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("eng_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| eng_first_create_schedule(this, stage, r, e, l))
}

/// Comparer for `eng_first_create_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_eng_first_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_FIRST_CREATE_PLAYOFF_STAGE);
    compare_playoff_stage("eng_first_create_playoff_stage", this, [5, 2, 4, 3], || o(this))
}

/// Comparer for `eng_first_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_eng_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_FIRST_ADVANCE_STAGE);
    compare_stage_advance("eng_first_advance_stage", this, || o(this), || eng_first_advance_stage(this))
}

/// Comparer for `construct_eng_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_FIRST);
    compare_english_constructor("construct_eng_first", this, year, record, &FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `eng_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_eng_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_first_set_up_rules", this, eng_prm_first_division_rules(), || o(this), || eng_first_set_up_rules(this))
}

/// Comparer for `eng_first_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_eng_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_FIRST_CREATE_TABLE);
    eng_prm_compare_league_table("eng_first_create_table", this, 0x18, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, 0x11170, || o(this))
}

/// Comparer for `eng_first_set_club_progress`.
pub(crate) unsafe extern "thiscall" fn compare_eng_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_ENG_FIRST_SET_CLUB_PROGRESS);
    compare_club_stage_status("eng_first_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || eng_first_set_club_progress(this, club, stage, current, best, worst))
}
