/// The goals score of sort modes 3 and 5: (5 × goals + 3 × assists + the +0xc rating sum) / matches.
pub(crate) unsafe fn year_goals_score(e: YearEntry) -> f64 {
    (5 * e.goals() + 3 * e.assists() + e.rating_sum(2)) as f64 / e.matches() as f64
}

/// Orders two year ratings entries for mode 3: the goals score (`year_goals_score`) × the reputation of the person's
/// club × 0.0002 (when he has a club) × his current ability × 0.02 (when he has a player sheet); at least 3 matches,
/// the higher score first (`year_entries_order`).
/// Original: `year_ratings_compare_mode_3` (0x9323b0, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_3(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| {
        let mut score = year_goals_score(e);
        let club = e.person().club_job();
        if !club.is_null() {
            covered!("sx05_3 mode 3 by club reputation");
            score = score * club.reputation() as f64 * (0.0002f32 as f64);
        }
        let sheet = e.person().player();
        if !sheet.is_null() {
            score = score * sheet.current_ability() as f64 * (0.02f32 as f64);
        }
        score
    })
}

/// Orders two year ratings entries for mode 4: cards (yellow + 2 × red) per match, the most first.
/// Original: `year_ratings_compare_mode_4` (0x932500, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_4(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| {
        covered!("sx05_3 mode 4 cards");
        e.cards() as f64 / e.matches() as f64
    })
}

/// Orders two year ratings entries for mode 5: the goals score (`year_goals_score`) × (current reputation × 0.0002 +
/// 4) when the person has a player sheet, halved when his determination, loyalty or sportsmanship is below 10.
/// Original: `year_ratings_compare_mode_5` (0x932590, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_5(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| {
        let person = e.person();
        let mut score = year_goals_score(e);
        let sheet = person.player();
        if !sheet.is_null() {
            score *= sheet.current_reputation() as f64 * (0.0002f32 as f64) + 4.0;
        }
        if person.determination() < 10 || person.loyalty() < 10 || person.sportsmanship() < 10 {
            covered!("sx05_3 mode 5 poor character");
            score *= 0.5;
        }
        score
    })
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_3(a: usize, b: usize) -> i32 {
    compare_year_order(%%I0%%, "year_ratings_compare_mode_3", a, b, year_ratings_compare_mode_3)
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_4(a: usize, b: usize) -> i32 {
    compare_year_order(%%I1%%, "year_ratings_compare_mode_4", a, b, year_ratings_compare_mode_4)
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_5(a: usize, b: usize) -> i32 {
    compare_year_order(%%I2%%, "year_ratings_compare_mode_5", a, b, year_ratings_compare_mode_5)
}
