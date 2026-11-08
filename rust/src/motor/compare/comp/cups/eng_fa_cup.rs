use super::*;

// The comparers of the routines in ../comp/cups/eng_fa_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/eng_fa_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eng_fa_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_FA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("eng_fa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| eng_fa_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `eng_fa_cup_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_fa_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ENG_FA_CUP_UPDATE_FOR_DATE);
    let before = read_u32_as_usize(this + 0x45);
    let vira = before == 0 && stage != 0 && is_date_today(make_date_dword(3, 9, read_i16(today + 2) as u16 as u32, 0xffff_ffff), today)
        && !(is_date_today(make_date_dword(1, 9, 0x7cf, 0xffff_ffff), today) && *((this + 0xb2) as *const u8) == 0);
    let r = o(this, today, stage);
    let after = read_u32_as_usize(this + 0x45);
    PLACAR.record(!vira || after == 1, || format!("eng_fa_cup_update_for_date({:#x}) vira {} +0x45 {} -> {}", this, vira, before, after));
    r
}
