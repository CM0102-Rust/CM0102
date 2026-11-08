use super::*;

// The comparers of the routines in ../comp/leagues/fra_third.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/fra_third.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fra_third_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_third_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_THIRD_CREATE_SCHEDULE);
    compare_league_schedule("fra_third_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| fra_third_create_schedule(this, fase, r, e, l))
}

/// Comparer for `fra_third_create_schedule_5bd920` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_third_create_schedule_5bd920(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_THIRD_CREATE_SCHEDULE_5BD920);
    compare_league_schedule("fra_third_create_schedule_5bd920", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| fra_third_create_schedule_5bd920(this, fase, r, e, l))
}

/// Comparer for `fra_third_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_third_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_THIRD_SET_UP_RULES);
    fra_first_compare_league_rules("fra_third_set_up_rules", this, national_division_rules(this), || o(this), || fra_third_set_up_rules(this))
}

/// Comparer for `fra_third_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_third_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_THIRD_CREATE_TABLE);
    fra_first_compare_league_table("fra_third_create_table", this, 0x14, KEY_CLUB_COMP_FRENCH_NATIONAL, Some(0x4e20), || o(this))
}

/// Comparer for `construct_fra_third` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_fra_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_THIRD);
    compare_division_league_ctor("construct_fra_third", this, ano, registro, &FRENCH_THIRD_DIVISION, || o(this, ano, registro))
}
