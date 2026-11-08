use super::*;

// The comparers of the routines in ../comp/cups/ita_c_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/ita_c_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_c_cup_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_C_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("ita_c_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| ita_c_cup_create_schedule(this, stage, r, e, l), 0xf)
}

/// Comparer for `ita_c_cup_select_entrants`.
pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_select_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ITA_C_CUP_SELECT_ENTRANTS);
    compare_select_entrants("ita_c_cup_select_entrants", this, || o(this), || select_serie_c_cup_entrants(this))
}

/// Comparer for `ita_c_cup_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_C_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("ita_c_cup_set_up_rules", this, &SERIE_C_CUP_RULES, r);
    r
}
