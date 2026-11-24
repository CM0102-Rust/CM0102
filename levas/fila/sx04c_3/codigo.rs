// ---------- sx04c_3: the South Korean awards: the object, its seven awards and the season's end (award/korea_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\korea_awards.cpp" (the error boxes).
const KOREA_AWARDS_CPP: usize = 0x9eb1d4;
/// `KoreaAwards::init_awards(file)` (0x66e170, thiscall, ret 4), through its hook.
const FN_KOREA_AWARDS_INIT: usize = 0x66e170;
/// The data version from which the Korean top goalscorer and most assisted player awards are in the save.
const KOREA_LATE_AWARDS_SAVED: i32 = 0x3e047;

/// The South Korean awards' class: the vtable 0x96cfd8, South Korea (`KEY_NATION_SOUTH_KOREA`), nation id 0x1d,
/// type 1, with month ratings (the error box at line 0x4a when they cannot be built).
const KOREA_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96cfd8, nation_key: Some(KEY_NATION_SOUTH_KOREA), nation_id: 0x1d, kind: 1,
    month_ratings: Some((KOREA_AWARDS_CPP, 0x4a)) };

/// Builds the South Korean awards (`nation_awards_construct` with `KOREA_AWARDS`): the saved awards with a file, else
/// the month ratings and `KoreaAwards::init_awards(file)`. Returns this.
/// Original: `KoreaAwards::KoreaAwards` (0x66e030, korea_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn korea_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &KOREA_AWARDS, file, core::mem::transmute(FN_KOREA_AWARDS_INIT))
}

/// Builds the seven South Korean awards (`nation_awards_build`, the error box at line 0x74), none tied to a
/// competition: the best 11 of the year (team award, national and world reputations 2500/1200, span 0, mode 0), the
/// player of the month (a month award from March to November of the current year, 1500/750), the manager (1000/500,
/// not for players), the player (1500/500), the top goalscorer (1000/500, option 1), the young player (500/400) and the
/// most assisted player of the year (1000/500); kind 0. The top goalscorer and the most assisted player are loaded from
/// the file only when the save's data version reaches 0x3e047 (`awards_file_from_version`). The staff competition keys
/// are the South Korean ones of the key table (0x9d0084 to 0x9d00a0, no name in `keys.rs` yet).
/// Original: `KoreaAwards::init_awards` (0x66e170, korea_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn korea_awards_init(this: usize, file: usize) {
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let fg = awards_file_from_version(file, KOREA_LATE_AWARDS_SAVED) as u32;
    let k = |key: usize| read_u32(key);
    let specs = [
        AwardSpec::Team([n, 0, k(0x9d0084 /* best 11 of the year */), 2500, 1200, u32::MAX, 0, 0, f, 1]),
        AwardSpec::Month([n, 1, k(0x9d0094 /* player of the month */), 1500, 750, u32::MAX, 2, year, 0xa, year, f, 1]),
        AwardSpec::Year([n, 2, k(0x9d008c /* manager of the year */), 1000, 500, u32::MAX, 0, 0, 0, f, 0]),
        AwardSpec::Year([n, 3, k(0x9d0098 /* player of the year */), 1500, 500, u32::MAX, 0, 0, 0, f, 1]),
        AwardSpec::Year([n, 4, k(0x9d009c /* top goalscorer of the year */), 1000, 500, u32::MAX, 0, 0, 1, fg, 1]),
        AwardSpec::Year([n, 5, k(0x9d00a0 /* young player of the year */), 500, 400, u32::MAX, 0, 0, 0, f, 1]),
        AwardSpec::Year([n, 6, k(0x9d0090 /* most assisted player of the year */), 1000, 500, u32::MAX, 0, 0, 0, fg, 1]),
    ];
    covered!("sx04c_3 korean awards built");
    nation_awards_build(this, &specs, KOREA_AWARDS_CPP, 0x74);
}

/// The South Korean awards' day: on 26 November (`awards_day_is`), the K-League manager of the year, the player (filters
/// Korean, mode 0x11), the young player (up to 21), the top goalscorer (up to 21, statistic 2) and the most assisted
/// player (up to 21, statistic 5) of the year from the year ratings (`award_top_three_from_ratings`), and the K-League
/// team of the season into the best 11. Every day, on the nation's season end day (`is_season_end_day`), the player of
/// the month award moves on (`month_award_start_new_season` with today's year); then the base daily update.
/// Original: `KoreaAwards::daily_update` (0x66e4d0, korea_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn korea_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1a, 0xa) {
        covered!("sx04c_3 korean awards day");
        award_manager_of_year(this, today, nation_award(this, 2), KEY_CLUB_COMP_KOREAN_LEAGUE);
        let korea = read_i32(KEY_NATION_SOUTH_KOREA);
        for (k, age, mode) in [(3, -1, 0x11), (5, 0x15, 0x11), (4, 0x15, 2), (6, 0x15, 5)] {
            award_top_three_from_ratings(this, today, nation_award(this, k), [korea, -1, -1, -1, age, 0, mode]);
        }
        award_team_of_season(this, KEY_CLUB_COMP_KOREAN_LEAGUE, nation_award(this, 0));
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_3 month award moves on");
        month_award_start_new_season(nation_award(this, 1), read_u16(today + cm_date::YEAR));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_korea_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &KOREA_AWARDS, 7, "korea_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_korea_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "korea_awards_init");
}

/// Comparer for `korea_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_korea_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
