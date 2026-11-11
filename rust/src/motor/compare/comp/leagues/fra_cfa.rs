use super::*;

// The comparers of the routines in ../comp/leagues/fra_cfa.rs.

/// Comparer for `fra_cfa_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_fra_cfa_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_CFA_CREATE_SCHEDULE);
    compare_cfa_schedule("fra_cfa_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| fra_cfa_create_schedule(this, stage, r, e, l))
}

/// Comparer for `fra_cfa_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_fra_cfa_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_CFA_SET_UP_RULES);
    let r = o(this.addr());
    let ok = r == 1 && (this.legs() as u16) == 2 && this.win_points() == 3 && this.draw_points() == 1 && this.match_category() == match_category::LEAGUE
        && (this.tiebreaks() as u8) == 4 && read_u8(this.addr() + league::TIEBREAKS + 1) == 1 && read_u8(this.addr() + league::TIEBREAKS + 2) == 2 && this.club_sides() == 1 && this.promoted_count() == 0 && this.promotion_playoff_count() == 0 && this.relegation_playoff_count() == 0 && this.relegated_count() == 0
        && (this.upper_division_id() as usize) == read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && this.lower_division_id() == 0xffff_ffff
        && (this.league_flags() as u16) == 0x43;
    SCOREBOARD.record(ok, || format!("fra_cfa_set_up_rules({:#x}): returned {}, fields away of the expected", this.addr(), r));
    r
}

/// Comparer for `construct_fra_cfa`.
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_cfa(this: usize, year: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_CONSTRUCT_FRA_CFA);
    compare_construct_group_league("construct_fra_cfa", League(this), year, &FRA_CFA_CFA_LEAGUE, || o(this, year))
}

/// Comparer for `destroy_fra_cfa`.
pub(crate) unsafe extern "thiscall" fn compare_destroy_fra_cfa(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_FRA_CFA)(this)
}

/// Comparer for `fra_cfa_create_group_table`.
pub(crate) unsafe extern "thiscall" fn compare_fra_cfa_create_group_table(this: usize, clubs: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FRA_CFA_CREATE_GROUP_TABLE);
    compare_french_table("fra_cfa_create_group_table", League(this), clubs, 0x12, &[KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION], || o(this, clubs))
}

