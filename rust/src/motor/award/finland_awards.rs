use super::*;

/// Passes a played match to the month ratings when its competition is a league division.
///
/// `FinlandAwards::record_match_ratings(fixture, record)` in the map, the virtual +8 of England's and Finland's awards
/// (from `award_manager_process_fixture`). When the fixture's competition id is a league division
/// (`exe::comp_util_is_league_division`), `NationAwards::record_match_ratings(fixture, record)` (the month ratings, when
/// the object has them).
/// Original: `FinlandAwards::record_match_ratings` (0x5a96b0, finland_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_record_league_match_ratings(this: usize, fixture: usize, record: usize) {
    if exe::comp_util_is_league_division(read_i32(Fixture(fixture).competition().addr())) != 0 { // sx03_4: league divisions only
        covered!("sx03_4 league match rated");
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATION_AWARDS_RECORD_MATCH_RATINGS);
        f(this, fixture, record);
    }
}

pub(crate) unsafe extern "thiscall" fn compare_nation_awards_record_league_match_ratings(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_NATION_AWARDS_RECORD_LEAGUE_MATCH_RATINGS)(this, fixture, record)
}

/// NationAwards::record_match_ratings (0x7683a0, thiscall(fixture, record), ret 8).
pub(crate) const FN_NATION_AWARDS_RECORD_MATCH_RATINGS: usize = 0x7683a0;
