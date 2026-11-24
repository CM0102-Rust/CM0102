use super::*;

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

pub(crate) unsafe extern "thiscall" fn compare_international_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_INTERNATIONAL_AWARDS_DAILY_UPDATE)(this, today)
}

/// Whether `today` (a Date: day of the year, year) is day `day` of month `month` (zero-based) of its own year
/// (`exe::date_date_549f70` with any weekday).
pub(crate) unsafe fn awards_day_is(today: usize, day: u32, month: u32) -> bool {
    is_date_today(make_date_dword(day, month, read_u16(today + cm_date::YEAR) as u32, u32::MAX), today)
}

/// The player of the year from the award manager's world year ratings, with the award's position filter (2 or 3).
pub(crate) unsafe fn award_world_player_from_ratings(this: usize, today: usize, award: usize, filter: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR_FROM_RATINGS);
    let ratings = read_usize(read_usize(G_AWARD_MANAGER) + 0x10); // the world year ratings
    f(this, today, award, ratings, -1, -1, -1, -1, -1, filter, -1)
}

/// NationAwards::award_player_of_year_from_ratings (0x7683c0, thiscall(today, award, year ratings, 7 filters), ret 0x28).
pub(crate) const FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR_FROM_RATINGS: usize = 0x7683c0;

/// nation_awards_award_from_shortlist (0x768b20, thiscall(today, award), ret 8).
pub(crate) const FN_NATION_AWARDS_AWARD_FROM_SHORTLIST: usize = 0x768b20;

/// nation_awards_award_top_three_from_ratings (0x768720, thiscall(today, award, year ratings, continent, a, b, c), ret 0x1c).
pub(crate) const FN_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS: usize = 0x768720;
