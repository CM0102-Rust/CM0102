use super::*;

// The comparers of the routines in ../comp/cups/fra_super.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/fra_super.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fra_super_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_super_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_SUPER_CREATE_ROUND_TABLE);
    if core::ptr::read_unaligned((this + 0x40) as *const i16) != 0x7d1 {
        return o(this, fase, rodadas, extra, saida);
    }
    fra_super_compare_round_table("fra_super_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| fra_super_create_round_table(this, fase, r, e, s))
}

/// Comparer for `fra_super_create_round_table_666020` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_super_create_round_table_666020(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_SUPER_CREATE_ROUND_TABLE_666020);
    fra_super_compare_round_table("fra_super_create_round_table_666020", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| fra_super_create_round_table_666020(this, fase, r, e, s))
}

/// Comparer for `fra_super_select_teams` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_FRA_SUPER_SELECT_TEAMS);
    eng_cc_cup_compare_entrant_list("fra_super_select_teams", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || french_super_cup_select_teams(this),
        |r, _| if r == 1 { 12 } else { 0 })
}
