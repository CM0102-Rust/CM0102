use super::*;

// The comparers of the routines in ../comp/cups/spa_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/spa_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_cup_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SPA_CUP_UPDATE_FOR_DATE);
    let before = this.season_set_up();
    let vira = before == 0 && read_i16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as u16 as i32 == this.year() as i32 && stage != 0;
    let r = o(this.addr(), today, stage);
    let after = this.season_set_up();
    SCOREBOARD.record(after == if vira { 1 } else { before }, || format!("spa_cup_update_for_date({:#x}) vira {} +0x45 {} -> {}", this.addr(), vira, before, after));
    r
}

/// Comparer for `spa_cup_record_season_honours`.
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_record_season_honours(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_RECORD_SEASON_HONOURS)(this)
}

/// Comparer for `spa_cup_record_season_honours_5e93e0`.
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_record_season_honours_5e93e0(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_RECORD_SEASON_HONOURS_5E93E0)(this)
}

/// Comparer for `spa_cup_record_season_honours_637750`.
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_record_season_honours_637750(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_RECORD_SEASON_HONOURS_637750)(this)
}

/// Comparer for `spa_cup_record_season_honours_63d770`.
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_record_season_honours_63d770(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_RECORD_SEASON_HONOURS_63D770)(this)
}

/// Comparer for `spa_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("spa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| spa_cup_create_round_table(this, stage, r, e, s))
}
