use super::*;

// The comparers of the routines in ../comp/leagues/fra_second.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/fra_second.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_fra_second` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_fra_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_SECOND);
    compare_division_league_ctor("construct_fra_second", this, ano, registro, &FRENCH_SECOND_DIVISION, || o(this, ano, registro))
}

/// Comparer for `fra_second_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_SECOND_SET_UP_RULES);
    fra_first_compare_league_rules("fra_second_set_up_rules", this, fra_first_second_division_rules(this), || o(this), || fra_second_set_up_rules(this))
}

/// Comparer for `fra_second_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_SECOND_CREATE_TABLE);
    fra_first_compare_league_table("fra_second_create_table", this, 0x14, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION, None, || o(this))
}

/// Comparer for `fra_second_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_second_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_SECOND_CREATE_SCHEDULE);
    compare_league_schedule("fra_second_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| fra_second_create_schedule(this, fase, r, e, l))
}
