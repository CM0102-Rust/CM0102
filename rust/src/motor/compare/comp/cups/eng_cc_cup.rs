use super::*;

// The comparers of the routines in ../comp/cups/eng_cc_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/eng_cc_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eng_cc_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_eng_cc_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_CC_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("eng_cc_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| eng_cc_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `eng_cc_cup_collect_entrants`.
pub(crate) unsafe extern "thiscall" fn compare_eng_cc_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_CC_CUP_COLLECT_ENTRANTS);
    eng_cc_cup_compare_entrant_list("eng_cc_cup_collect_entrants", this, &[(this.addr() + cup::TEAM_COUNT, 2), (this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 1), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this.addr()),
        || collect_league_cup_entrants(this), |_, n| n.max(0) as usize * 6)
}

/// Comparer for `eng_cc_cup_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_eng_cc_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ENG_CC_CUP_UPDATE_FOR_DATE);
    let before = this.season_set_up();
    let vira = before == 0 && read_i16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as u16 as i32 == this.year() as i32 && stage != 0;
    let r = o(this.addr(), today, stage);
    let after = this.season_set_up();
    SCOREBOARD.record(after == if vira { 1 } else { before }, || format!("eng_cc_cup_update_for_date({:#x}) vira {} +0x45 {} -> {}", this.addr(), vira, before, after));
    r
}

