// ---------- sx03_4: the international player awards, the league matches in the month ratings and Denmark's transfer rules (international_awards.cpp, finland_awards.cpp, denmark_awards.cpp) ----------

/// NationAwards::award_player_of_year_from_ratings (0x7683c0, thiscall(today, award, year ratings, 7 filters), ret 0x28).
const FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR_FROM_RATINGS: usize = 0x7683c0;
/// nation_awards_award_from_shortlist (0x768b20, thiscall(today, award), ret 8).
const FN_NATION_AWARDS_AWARD_FROM_SHORTLIST: usize = 0x768b20;
/// nation_awards_award_top_three_from_ratings (0x768720, thiscall(today, award, year ratings, continent, a, b, c), ret 0x1c).
const FN_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS: usize = 0x768720;
/// NationAwards::record_match_ratings (0x7683a0, thiscall(fixture, record), ret 8).
const FN_NATION_AWARDS_RECORD_MATCH_RATINGS: usize = 0x7683a0;

/// The player of the year from the award manager's world year ratings, with the award's position filter (2 or 3).
unsafe fn award_world_player_from_ratings(this: usize, today: usize, award: usize, filter: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR_FROM_RATINGS);
    let ratings = read_usize(read_usize(G_AWARD_MANAGER) + 0x10); // the world year ratings
    f(this, today, award, ratings, -1, -1, -1, -1, -1, filter, -1)
}

/// Gives the world and continental player awards on their dates, outside 2001.
///
/// `InternationalAwards::daily_update(today)`, the virtual +4 of the international awards (from
/// `award_manager_process_date`). Nothing in 2001 (`cm_date::YEAR` 0x7d1). On 6 May, the world player of the year
/// (award 0) from the award manager's world year ratings (+0x10), filter 2; on 6 June, award 0 is decided from its
/// shortlist (`nation_awards_award_from_shortlist`) and award 1 is given from the same ratings, filter 3; on 16 June,
/// the best three of Africa, Europe, South America and Oceania (awards 2 to 5,
/// `nation_awards_award_top_three_from_ratings` with the continent key and 0, 1, 0). No base month check.
/// Original: `InternationalAwards::daily_update` (0x632e20, international_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn international_awards_daily_update(this: usize, today: usize) {
    if read_u16(today + cm_date::YEAR) == 0x7d1 {
        return;
    }
    if awards_day_is(today, 6, 4) {
        covered!("sx03_4 world player of the year");
        award_world_player_from_ratings(this, today, nation_award(this, 0), 2); // sx03_4: 6 May
    }
    if awards_day_is(today, 6, 5) {
        let shortlist: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATION_AWARDS_AWARD_FROM_SHORTLIST);
        shortlist(this, today, nation_award(this, 0));
        award_world_player_from_ratings(this, today, nation_award(this, 1), 3);
    }
    if awards_day_is(today, 0x10, 5) {
        let top_three: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS);
        let ratings = read_usize(read_usize(G_AWARD_MANAGER) + 0x10);
        for (k, continent) in [KEY_CONTINENT_AFRICA, KEY_CONTINENT_EUROPE, KEY_CONTINENT_SOUTH_AMERICA, KEY_CONTINENT_OCEANIA].iter().enumerate() {
            top_three(this, today, nation_award(this, 2 + k), ratings, read_u32(*continent), 0, 1, 0);
        }
    }
}

// simplification: the awards post news, write history records and raise reputations: the routine cannot run twice.
// Original only; the copy is proven by the career (6 May 2002).
pub(crate) unsafe extern "thiscall" fn compare_international_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, today)
}

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

// simplification: the month ratings add entries (they may allocate): it cannot run twice. Original only; the copy is
// proven by the career (the player of the month).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_record_league_match_ratings(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, fixture, record)
}

/// Builds Denmark's transfer rules: the base rules, Denmark's vtable and no transfer windows, so Danish clubs sign at
/// any time.
///
/// `DENMARK_RULES::DENMARK_RULES(a1..a5)`, once, when the transfer manager builds its rules (0x8d2892). The base
/// constructor (`exe::new_transfer_rules`), the vtable 0x969860, the window count (+8) and the window table (+4) at 0.
/// Returns the object.
/// Original: `DENMARK_RULES::DENMARK_RULES` (0x555460, denmark_awards.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_denmark_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx03_4 denmark rules");
    exe::new_transfer_rules(this, a1, a2, a3, a4, a5);
    write_u32(this, 0x969860);
    write_u8(this + 8, 0); // sx03_4: no windows
    write_u32(this + 4, 0);
    this
}

/// Comparer for `construct_denmark_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_denmark_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_denmark_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_denmark_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
