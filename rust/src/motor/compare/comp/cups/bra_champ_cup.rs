use super::*;

// The comparers of the routines in ../comp/cups/bra_champ_cup.rs.

/// Comparer for `bra_champ_cup_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_CHAMP_CUP_CREATE_SCHEDULE);
    compare_stage_schedule("bra_champ_cup_create_schedule", stage, champions_cup_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_champ_cup_create_schedule(this, stage, r, e, l))
}

/// Comparer for `bra_champ_cup_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CHAMP_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("bra_champ_cup_set_up_rules", League(this), &CHAMPIONS_CUP_RULES, r);
    r
}
