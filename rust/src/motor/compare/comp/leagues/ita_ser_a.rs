use super::*;

// The comparers of the routines in ../comp/leagues/ita_ser_a.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/ita_ser_a.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_ser_a_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_SER_A_CREATE_SCHEDULE);
    ita_ser_a_compare_schedule_all_stages("ita_ser_a_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ita_ser_a_create_schedule(this, stage, r, e, l))
}

/// Comparer for `ita_ser_a_create_schedule_645af0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_create_schedule_645af0(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_SER_A_CREATE_SCHEDULE_645AF0);
    ita_ser_a_compare_schedule_all_stages("ita_ser_a_create_schedule_645af0", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ita_ser_a_create_schedule_645af0(this, stage, r, e, l))
}

/// Comparer for `ita_ser_a_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ITA_SER_A_UPDATE_FOR_DATE);
    compare_league_day("ita_ser_a_update_for_date", this, full, || serie_a_zones(this), || o(this, today, full))
}

/// Comparer for `ita_ser_a_update_for_date_647050` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_update_for_date_647050(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ITA_SER_A_UPDATE_FOR_DATE_647050);
    compare_league_day("ita_ser_a_update_for_date_647050", this, full, serie_b_zones, || o(this, today, full))
}

/// Comparer for `ita_league_sort_clubs_level_on_points` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado na tabela inteira (a 0x682b30 grava +4 de cada linha e solta a sua minitabela; o qsort só mexe no
/// bloco).
pub(crate) unsafe extern "thiscall" fn comparar_ita_league_sort_clubs_level_on_points(this: usize, points: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_ITA_LEAGUE_SORT_CLUBS_LEVEL_ON_POINTS);
    let t = read_u32_as_usize(this + 0xb1);
    let n = read_i16(this + 0x3e).max(0) as usize;
    if t == 0 || n == 0 {
        return o(this, points);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(t, n * 0x3b)], || o(this, points), || ita_league_sort_clubs_level_on_points(this, points));
    PLACAR.registrar(igual, || format!("ita_league_sort_clubs_level_on_points({:#x}, {}) original={:x?} rust={:x?}", this, points as u16 as i16, mo, mr));
}

/// Comparer for `ita_ser_a_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Na liga e no desempate com a situação 1 há chamadas que não rodam duas vezes (os registros da base, a ordenação, as
/// virtuais): só a original. Nas outras situações a cópia só devolve −1: compara a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_update_staff_records_for_status(this: usize, clube: usize, status: u32, stage: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ITA_SER_A_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let r = o(this, clube, status, stage, a4, jogo, a6);
    if stage as u8 != 0xff && status as u8 != 1 {
        PLACAR.registrar(r == 0xffff_ffff, || format!("ita_ser_a_update_staff_records_for_status: devolveu {:#x}", r));
    }
    r
}

/// Comparer for `construct_ita_ser_a` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_ita_ser_a(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_SER_A);
    compare_italian_constructor("construct_ita_ser_a", this, year, record, &SERIE_A, || o(this, year, record))
}

/// Comparer for `construct_ita_ser_b` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_ita_ser_b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_SER_B);
    compare_italian_constructor("construct_ita_ser_b", this, year, record, &SERIE_B_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_ita_ser_c1b` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_ita_ser_c1b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_SER_C1B);
    compare_italian_constructor("construct_ita_ser_c1b", this, year, record, &SERIE_C1B_DIVISION, || o(this, year, record))
}

/// Comparer for `create_fixed_fixtures` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_fixed_fixtures(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_FIXED_FIXTURES);
    ita_ser_a_compare_fixed_fixtures("create_fixed_fixtures", this, || o(this))
}

/// Comparer for `ita_ser_a_set_up_fixed_fixtures` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_set_up_fixed_fixtures(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_A_SET_UP_FIXED_FIXTURES);
    ita_ser_a_compare_fixed_fixtures("ita_ser_a_set_up_fixed_fixtures", this, || o(this))
}

/// Comparer for `ita_ser_a_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_SET_UP_RULES);
    ita_ser_a_compare_league_rules("ita_ser_a_set_up_rules", this, serie_a_rules(this), || o(this), || ita_ser_a_set_up_rules(this))
}

/// Comparer for `ita_ser_a_set_up_rules_6468a0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_set_up_rules_6468a0(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_SET_UP_RULES_6468A0);
    ita_ser_a_compare_league_rules("ita_ser_a_set_up_rules_6468a0", this, serie_b_rules(this), || o(this), || ita_ser_a_set_up_rules_6468a0(this))
}

/// Comparer for `ita_ser_a_set_up_rules_64f920` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_set_up_rules_64f920(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_SET_UP_RULES_64F920);
    ita_ser_a_compare_league_rules("ita_ser_a_set_up_rules_64f920", this, serie_c1b_rules(this), || o(this), || ita_ser_a_set_up_rules_64f920(this))
}

/// Comparer for `ita_ser_a_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_CREATE_TABLE);
    ita_ser_a_compare_league_table("ita_ser_a_create_table", this, 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_A, Some(0x3d090), || o(this))
}

/// Comparer for `ita_ser_a_create_table_646780` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_create_table_646780(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_CREATE_TABLE_646780);
    ita_ser_a_compare_league_table("ita_ser_a_create_table_646780", this, 0x14, KEY_CLUB_COMP_ITALIAN_SERIE_B, Some(0x11170), || o(this))
}

/// Comparer for `ita_ser_a_create_table_64f820` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_a_create_table_64f820(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_CREATE_TABLE_64F820);
    ita_ser_a_compare_league_table("ita_ser_a_create_table_64f820", this, 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, None, || o(this))
}
