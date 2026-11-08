use super::*;

// The comparers of the routines in ../comp/leagues/eng_prm.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/eng_prm.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_eng_prm` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_eng_prm(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_PRM);
    compare_english_constructor("construct_eng_prm", this, ano, registro, &PREMIER_DIVISION, || o(this, ano, registro))
}

/// Comparer for `construct_eng_third` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_eng_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_THIRD);
    compare_english_constructor("construct_eng_third", this, ano, registro, &THIRD_DIVISION, || o(this, ano, registro))
}

/// Comparer for `eng_prm_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_prm_set_up_rules", this, premier_rules(), || o(this), || eng_prm_set_up_rules(this))
}

/// Comparer for `eng_prm_set_up_rules_5780c0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_set_up_rules_5780c0(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_SET_UP_RULES_5780C0);
    eng_prm_compare_league_rules("eng_prm_set_up_rules_5780c0", this, third_division_rules(this), || o(this), || eng_prm_set_up_rules_5780c0(this))
}

/// Comparer for `eng_prm_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("eng_prm_create_table", this, 0x14, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || o(this))
}

/// Comparer for `eng_prm_create_table_577f70` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_create_table_577f70(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_CREATE_TABLE_577F70);
    eng_prm_compare_league_table("eng_prm_create_table_577f70", this, 0x18, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || o(this))
}

/// Comparer for `eng_prm_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_PRM_CREATE_SCHEDULE);
    eng_first_compare_round_table("eng_prm_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_prm_create_schedule(this, fase, r, e, l))
}
