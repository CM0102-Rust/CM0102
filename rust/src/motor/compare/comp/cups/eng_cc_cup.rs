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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_CC_CUP_COLLECT_ENTRANTS);
    eng_cc_cup_compare_entrant_list("eng_cc_cup_collect_entrants", this, &[(this + cup::TEAM_COUNT, 2), (this + 0xb2, 1), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this),
        || collect_league_cup_entrants(this), |_, n| n.max(0) as usize * 6)
}

/// Comparer for `eng_cc_cup_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_eng_cc_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ENG_CC_CUP_UPDATE_FOR_DATE);
    let before = read_usize(this + cup::SEASON_SET_UP);
    let vira = before == 0 && read_i16(read_usize(G_UEFA_SEEDING) + 0x439) as u16 as i32 == read_i16(this + cup::YEAR) as i32 && stage != 0;
    let r = o(this, today, stage);
    let after = read_usize(this + cup::SEASON_SET_UP);
    SCOREBOARD.record(after == if vira { 1 } else { before }, || format!("eng_cc_cup_update_for_date({:#x}) vira {} +0x45 {} -> {}", this, vira, before, after));
    r
}

