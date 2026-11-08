use super::*;

// The comparers of the routines in ../comp/leagues/ita_ser_b.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/ita_ser_b.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_ser_b_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_SER_B_CREATE_SCHEDULE);
    compare_c1_schedule("ita_ser_b_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ita_ser_b_create_schedule(this, stage, r, e, l))
}

/// Comparer for `ita_ser_b_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ITA_SER_B_UPDATE_FOR_DATE);
    compare_league_day("ita_ser_b_update_for_date", this, full, serie_b_zones, || o(this, today, full))
}

/// Comparer for `ita_league_sort_clubs_level_on_points`.
///
/// [pt] Lado a lado na tabela inteira (a 0x682b30 grava +4 de cada linha e solta a sua minitabela; o qsort só mexe no
/// bloco).
pub(crate) unsafe extern "thiscall" fn compare_ita_league_sort_clubs_level_on_points(this: usize, points: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_ITA_LEAGUE_SORT_CLUBS_LEVEL_ON_POINTS);
    let t = read_usize(this + league::TABLE);
    let n = read_i16(this + league::CLUB_COUNT).max(0) as usize;
    if t == 0 || n == 0 {
        return o(this, points);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(t, n * 0x3b)], || o(this, points), || ita_league_sort_clubs_level_on_points(this, points));
    SCOREBOARD.record(igual, || format!("ita_league_sort_clubs_level_on_points({:#x}, {}) original={:x?} rust={:x?}", this, points as u16 as i16, mo, mr));
}

/// Comparer for `construct_ita_ser_b`.
pub(crate) unsafe extern "thiscall" fn compare_construct_ita_ser_b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_SER_B);
    compare_italian_constructor("construct_ita_ser_b", this, year, record, &SERIE_B_DIVISION, || o(this, year, record))
}

/// Comparer for `ita_ser_b_set_up_fixed_fixtures`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_set_up_fixed_fixtures(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_B_SET_UP_FIXED_FIXTURES);
    fra_first_compare_fixed_fixtures("ita_ser_b_set_up_fixed_fixtures", this, || o(this))
}

/// Comparer for `ita_ser_b_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_B_SET_UP_RULES);
    ita_ser_a_compare_league_rules("ita_ser_b_set_up_rules", this, serie_b_rules(this), || o(this), || ita_ser_b_set_up_rules(this))
}

/// Comparer for `ita_ser_b_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_B_CREATE_TABLE);
    ita_ser_a_compare_league_table("ita_ser_b_create_table", this, 0x14, KEY_CLUB_COMP_ITALIAN_SERIE_B, Some(0x11170), || o(this))
}
