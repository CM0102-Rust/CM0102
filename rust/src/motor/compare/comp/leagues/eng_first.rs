use super::*;

// The comparers of the routines in ../comp/leagues/eng_first.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/eng_first.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eng_first_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_first_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_FIRST_CREATE_SCHEDULE);
    eng_first_compare_round_table("eng_first_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_first_create_schedule(this, fase, r, e, l))
}

/// Comparer for `eng_first_create_playoff_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_first_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_FIRST_CREATE_PLAYOFF_STAGE);
    compare_playoff_stage("eng_first_create_playoff_stage", this, [5, 2, 4, 3], || o(this))
}

/// Comparer for `eng_first_create_play_off_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_FIRST_CREATE_PLAY_OFF_STAGE);
    compare_playoff_stage("eng_first_create_play_off_stage", this, [6, 3, 5, 4], || o(this))
}

/// Comparer for `eng_first_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_FIRST_ADVANCE_STAGE);
    compare_stage_advance("eng_first_advance_stage", this, || o(this), || eng_first_advance_stage(this))
}

/// Comparer for `construct_eng_first` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_eng_first(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_FIRST);
    compare_english_constructor("construct_eng_first", this, ano, registro, &FIRST_DIVISION, || o(this, ano, registro))
}

/// Comparer for `eng_first_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_first_set_up_rules", this, eng_prm_first_division_rules(), || o(this), || eng_first_set_up_rules(this))
}

/// Comparer for `eng_first_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_FIRST_CREATE_TABLE);
    eng_prm_compare_league_table("eng_first_create_table", this, 0x18, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, 0x11170, || o(this))
}

/// Comparer for `eng_first_set_club_progress` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_first_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_ENG_FIRST_SET_CLUB_PROGRESS);
    compare_club_stage_status("eng_first_set_club_progress", this, clube, || o(this, clube, fase, atual, melhor, pior), || eng_first_set_club_progress(this, clube, fase, atual, melhor, pior))
}
