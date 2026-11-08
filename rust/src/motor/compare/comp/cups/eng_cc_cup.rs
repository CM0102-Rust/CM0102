use super::*;

// The comparers of the routines in ../comp/cups/eng_cc_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/eng_cc_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eng_cc_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_cc_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_CC_CUP_CREATE_ROUND_TABLE);
    eng_cc_cup_compare_round_table("eng_cc_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| eng_cc_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `eng_cc_cup_create_round_table_56f320` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_cc_cup_create_round_table_56f320(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_CC_CUP_CREATE_ROUND_TABLE_56F320);
    eng_cc_cup_compare_round_table("eng_cc_cup_create_round_table_56f320", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| eng_cc_cup_create_round_table_56f320(this, stage, r, e, s))
}

/// Comparer for `eng_cc_cup_create_round_table_56d640` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_cc_cup_create_round_table_56d640(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_CC_CUP_CREATE_ROUND_TABLE_56D640);
    eng_cc_cup_compare_round_table("eng_cc_cup_create_round_table_56d640", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| eng_cc_cup_create_round_table_56d640(this, stage, r, e, s))
}

/// Comparer for `eng_cc_cup_collect_entrants` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_cc_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_CC_CUP_COLLECT_ENTRANTS);
    eng_cc_cup_compare_entrant_list("eng_cc_cup_collect_entrants", this, &[(this + 0x3e, 2), (this + 0xb2, 1), (0xb67a34, 4)], || o(this),
        || collect_league_cup_entrants(this), |_, n| n.max(0) as usize * 6)
}

/// Comparer for `eng_cc_cup_select_teams` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_cc_cup_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_CC_CUP_SELECT_TEAMS);
    eng_cc_cup_compare_entrant_list("eng_cc_cup_select_teams", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || select_charity_shield_teams(this),
        |r, _| if r == 1 { 12 } else { 0 })
}

/// Comparer for `eng_cc_cup_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_cc_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ENG_CC_CUP_UPDATE_FOR_DATE);
    let before = read_u32_as_usize(this + 0x45);
    let vira = before == 0 && read_i16(read_u32_as_usize(G_UEFA_SEEDING) + 0x439) as u16 as i32 == read_i16(this + 0x40) as i32 && stage != 0;
    let r = o(this, today, stage);
    let after = read_u32_as_usize(this + 0x45);
    PLACAR.registrar(after == if vira { 1 } else { before }, || format!("eng_cc_cup_update_for_date({:#x}) vira {} +0x45 {} -> {}", this, vira, before, after));
    r
}

/// Comparer for `eng_cc_cup_update_for_date_570550` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_cc_cup_update_for_date_570550(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ENG_CC_CUP_UPDATE_FOR_DATE_570550);
    let before = read_u32_as_usize(this + 0x45);
    let vira = before == 0 && stage != 0 && is_date_today(make_date_dword(3, 9, read_i16(today + 2) as u16 as u32, 0xffff_ffff), today)
        && !(is_date_today(make_date_dword(1, 9, 0x7cf, 0xffff_ffff), today) && *((this + 0xb2) as *const u8) == 0);
    let r = o(this, today, stage);
    let after = read_u32_as_usize(this + 0x45);
    PLACAR.registrar(!vira || after == 1, || format!("eng_cc_cup_update_for_date_570550({:#x}) vira {} +0x45 {} -> {}", this, vira, before, after));
    r
}
