use super::*;

// The comparers of the routines in ../comp/leagues/ita_ser_a.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/ita_ser_a.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_ser_a_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_SER_A_CREATE_SCHEDULE);
    compare_c1_schedule("ita_ser_a_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ita_ser_a_create_schedule(this, stage, r, e, l))
}

/// Comparer for `ita_ser_a_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ITA_SER_A_UPDATE_FOR_DATE);
    compare_league_day("ita_ser_a_update_for_date", League(this), full, || serie_a_zones(League(this)), || o(this, today, full))
}

/// Comparer for `ita_ser_a_update_staff_records_for_status`.
///
/// [pt] Na liga e no desempate com a situação 1 há chamadas que não rodam duas vezes (os registros da base, a ordenação, as
/// virtuais): só a original. Nas outras situações a cópia só devolve −1: compara a devolução.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ITA_SER_A_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let r = o(this, club, status, stage, a4, game, a6);
    if stage as u8 != 0xff && status as u8 != 1 {
        SCOREBOARD.record(r == 0xffff_ffff, || format!("ita_ser_a_update_staff_records_for_status: devolveu {:#x}", r));
    }
    r
}

/// Comparer for `construct_ita_ser_a`.
pub(crate) unsafe extern "thiscall" fn compare_construct_ita_ser_a(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_SER_A);
    compare_italian_constructor("construct_ita_ser_a", League(this), year, record, &SERIE_A, || o(this, year, record))
}

/// Comparer for `create_fixed_fixtures`.
pub(crate) unsafe extern "thiscall" fn compare_create_fixed_fixtures(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_FIXED_FIXTURES);
    fra_first_compare_fixed_fixtures("create_fixed_fixtures", League(this), || o(this))
}

/// Comparer for `ita_ser_a_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_SET_UP_RULES);
    ita_ser_a_compare_league_rules("ita_ser_a_set_up_rules", League(this), serie_a_rules(this), || o(this), || ita_ser_a_set_up_rules(this))
}

/// Comparer for `ita_ser_a_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_CREATE_TABLE);
    ita_ser_a_compare_league_table("ita_ser_a_create_table", League(this), 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_A, Some(0x3d090), || o(this))
}

