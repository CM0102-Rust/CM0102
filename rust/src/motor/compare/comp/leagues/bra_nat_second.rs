use super::*;

// The comparers of the routines in ../comp/leagues/bra_nat_second.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_nat_second.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_nat_second_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_SECOND_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_nat_second_advance_stage", this, || o(this))
}

/// Comparer for `bra_nat_second_create_play_off_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_SECOND_CREATE_PLAY_OFF_STAGE);
    compare_play_off_stage("bra_nat_second_create_play_off_stage", this, &[0, 3, 1, 2], || o(this))
}

/// Comparer for `bra_nat_second_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("bra_nat_second_start_new_season", this, || o(this))
}

/// Comparer for `bra_nat_second_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_SECOND_CREATE_TABLE);
    compare_division_table("bra_nat_second_create_table", this, 0x1c, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, || o(this))
}

/// Comparer for `bra_nat_second_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: como o da Série A.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_SECOND_SET_UP_RULES)(this), || {
        set_serie_b_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_nat_second_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
