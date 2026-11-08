use super::*;

// The comparers of the routines in ../comp/cups/spa_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/spa_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_cup_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SPA_CUP_UPDATE_FOR_DATE);
    let before = read_u32_as_usize(this + 0x45);
    let vira = before == 0 && read_i16(read_u32_as_usize(G_UEFA_SEEDING) + 0x439) as u16 as i32 == read_i16(this + 0x40) as i32 && stage != 0;
    let r = o(this, today, stage);
    let after = read_u32_as_usize(this + 0x45);
    SCOREBOARD.record(after == if vira { 1 } else { before }, || format!("spa_cup_update_for_date({:#x}) vira {} +0x45 {} -> {}", this, vira, before, after));
    r
}

/// Comparer for `spa_cup_record_season_honours` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_record_season_honours(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_RECORD_SEASON_HONOURS)(this)
}

/// Comparer for `spa_cup_record_season_honours_5e93e0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_record_season_honours_5e93e0(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_RECORD_SEASON_HONOURS_5E93E0)(this)
}

/// Comparer for `spa_cup_record_season_honours_637750` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_record_season_honours_637750(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_RECORD_SEASON_HONOURS_637750)(this)
}

/// Comparer for `spa_cup_record_season_honours_63d770` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_record_season_honours_63d770(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_RECORD_SEASON_HONOURS_63D770)(this)
}

/// Comparer for `spa_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("spa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| spa_cup_create_round_table(this, stage, r, e, s))
}
