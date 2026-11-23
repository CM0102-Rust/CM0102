use super::*;

/// end of the Argentine second division and Series A and B: the honours come from the first stage, not the overall
/// table
/// Original: `ArgSecond::record_season_honours` (0x408eb0, arg_second.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn arg_second_record_season_honours(this: usize) -> u32 {
    covered!("cqj_8 first stage history");
    record_first_stage_honours(League(this))
}

/// Rule: the Argentine Second Division (Nacional B) at career creation: discipline rule 3 and, with no saved state,
/// importance 15, six stages (the three zones, the promotion cup and the play-offs), the rules (ArgSecond::set_up_rules
/// 0x407e50; 0: the box at line 0x4e), the table (ArgSecond::fill_league_table 0x407cd0; line 0x55), the fixtures
/// (`exe::league_create_fixtures`; line 0x5b), the order (`exe::league_sort_table`(this, 0); line 0x62), the statistics
/// (line 0x69), the relegation averages (ArgSecond::init_relegation_averages 0x4093f0) and its own predicted positions
/// (ArgSecond::set_predicted_positions 0x409fe0). The Brazilian state leagues' body (`bra_reg_goias_construct_state_league`)
/// with discipline rule 3 and the averages before the predicted positions. Every box returns this; returns this.
/// Original: `ArgSecond::ArgSecond` (0x406bf0, arg_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_arg_second(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    if argentine_league_start(this, year, record, 0x9673d8, 6) {
        return this.addr();
    }
    covered!("mwf_23 argentine second new");
    if argentine_league_call(this, 0x407e50) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x4e);
        return this.addr();
    }
    if argentine_league_call(this, 0x407cd0) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x55);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x5b);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x62);
        return this.addr();
    }
    if !argentine_league_stats(this) {
        show_error_at(ARG_SECOND_FILE, 0x69);
        return this.addr();
    }
    argentine_league_call(this, 0x4093f0);
    argentine_league_call(this, 0x409fe0);
    this.addr()
}

/// Comparer for `construct_arg_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_arg_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ARG_SECOND);
    compare_argentine_league_ctor("construct_arg_second", League(this), year, record, 0x9673d8, 6, || o(this, year, record))
}

pub(crate) const ARG_SECOND_FILE: usize = 0x9874ec;
