use super::*;

// The comparers of the routines in ../comp/cups/bra_champ_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/bra_champ_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_champ_cup_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_champ_cup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_CHAMP_CUP_CREATE_SCHEDULE);
    compare_stage_schedule("bra_champ_cup_create_schedule", fase, champions_cup_stage(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| bra_champ_cup_create_schedule(this, fase, r, e, l))
}

/// Comparer for `bra_champ_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_champ_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_CHAMP_CUP_CREATE_ROUND_TABLE);
    eng_cc_cup_compare_round_table("bra_champ_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| bra_champ_cup_create_round_table(this, fase, r, e, s))
}

/// Comparer for `bra_champ_cup_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_champ_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CHAMP_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("bra_champ_cup_set_up_rules", this, &CHAMPIONS_CUP_RULES, r);
    r
}
