use super::*;

// The comparers of the routines in ../comp/cups/eng_charity.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/eng_charity.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eng_charity_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_charity_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_CHARITY_CREATE_ROUND_TABLE);
    compare_cup_round_table("eng_charity_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| eng_charity_create_round_table(this, stage, r, e, s))
}

/// Comparer for `eng_charity_select_teams` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_charity_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_CHARITY_SELECT_TEAMS);
    eng_cc_cup_compare_entrant_list("eng_charity_select_teams", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || select_charity_shield_teams(this),
        |r, _| if r == 1 { 12 } else { 0 })
}
