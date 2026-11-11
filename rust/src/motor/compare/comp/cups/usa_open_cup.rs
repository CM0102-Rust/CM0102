use super::*;

// The comparers of the routines in ../comp/cups/usa_open_cup.rs.

/// Comparer for `usa_open_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_usa_open_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_USA_OPEN_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("usa_open_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| usa_open_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `usa_open_cup_collect_entrants`.
pub(crate) unsafe extern "thiscall" fn compare_usa_open_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_USA_OPEN_CUP_COLLECT_ENTRANTS);
    jap_j_cup_compare_entrant_list("usa_open_cup_collect_entrants", Cup(this), || o(this), || collect_us_open_cup_entrants(Cup(this)))
}

/// Comparer for `usa_open_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_usa_open_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_OPEN_CUP_START_NEW_SEASON);
    compare_cup_new_season("usa_open_cup_start_new_season", Cup(this), || o(this))
}
