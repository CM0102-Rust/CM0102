// ---------- sx05b_18: the Welsh awards: the object, its six awards and the season's end (award/wales_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\wales_awards.cpp" (the error boxes).
const WALES_AWARDS_CPP: usize = 0xad43cc;
/// `WalesAwards::init_awards(file)` (0x90ec10, thiscall, ret 4), through its hook.
const FN_WALES_AWARDS_INIT: usize = 0x90ec10;

/// The Welsh awards' class: the vtable 0x970a88, Wales (`KEY_NATION_WALES`), nation id 0x1c, type 1, with month
/// ratings (the error box at line 0x49 when they cannot be built).
const WALES_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x970a88, nation_key: Some(KEY_NATION_WALES), nation_id: 0x1c, kind: 1,
    month_ratings: Some((WALES_AWARDS_CPP, 0x49)) };

/// Builds the Welsh awards (`nation_awards_construct` with `WALES_AWARDS`): the saved awards with a file, else the
/// month ratings and `WalesAwards::init_awards(file)`. Returns this.
/// Original: `WalesAwards::WalesAwards` (0x90ead0, wales_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wales_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &WALES_AWARDS, file, core::mem::transmute(FN_WALES_AWARDS_INIT))
}

/// Builds the six Welsh awards (`nation_awards_build`, the error box at line 0x71), all of the Welsh Premier Division:
/// the team of the week (team award, national and world reputations 1000 and 500), the player (1500/500), the young
/// player (500/250) and the top goalscorer of the year (500/250, option 1), the manager of the year (2500/1200, not for
/// players) and the manager of the month (a month award from August of the current year to April of the next,
/// 2000/1000); kind 1, no scope.
/// Original: `WalesAwards::init_awards` (0x90ec10, wales_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wales_awards_init(this: usize, file: usize) {
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let premier = k(KEY_CLUB_COMP_WELSH_PREMIER_DIVISION);
    let specs = [
        AwardSpec::Team([n, 0, k(KEY_STAFF_COMP_WELSH_TEAM_OF_THE_WEEK), 1000, 500, premier, 1, 0, f, 1]),
        AwardSpec::Year([n, 1, k(KEY_STAFF_COMP_WELSH_PLAYER_OF_THE_YEAR), 1500, 500, premier, 1, 0, 0, f, 1]),
        AwardSpec::Year([n, 2, k(KEY_STAFF_COMP_WELSH_YOUNG_PLAYER_OF_THE_YEAR), 500, 250, premier, 1, 0, 0, f, 1]),
        AwardSpec::Year([n, 3, k(KEY_STAFF_COMP_WELSH_TOP_GOALSCORER), 500, 250, premier, 1, 0, 1, f, 1]),
        AwardSpec::Year([n, 4, k(KEY_STAFF_COMP_WELSH_MANAGER_OF_THE_YEAR), 2500, 1200, premier, 1, 0, 0, f, 0]),
        AwardSpec::Month([n, 5, k(KEY_STAFF_COMP_WELSH_MANAGER_OF_THE_MONTH), 2000, 1000, premier, 7, year, 3, year + 1, f, 0]),
    ];
    covered!("sx05b_18 welsh awards built");
    nation_awards_build(this, &specs, WALES_AWARDS_CPP, 0x71);
}

/// The Welsh awards' day: on 11 May (`awards_day_is`), the player (by the season's ratings, statistic 0x11), the top
/// goalscorer (statistic 2) and the young player (up to 21) of the year, and the manager of the year, all of the Welsh
/// Premier Division. Every day, on the nation's season end day (`is_season_end_day`), the manager of the month award
/// moves on (`month_award_start_new_season` with today's year); then the base daily update.
/// Original: `WalesAwards::daily_update` (0x90ef10, wales_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wales_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0xb, 4) {
        covered!("sx05b_18 welsh awards day");
        for (k, age, stat) in [(1, -1, 0x11), (3, -1, 2), (2, 0x15, 0x11)] {
            award_player_of_year(this, KEY_CLUB_COMP_WELSH_PREMIER_DIVISION, today, nation_award(this, k), [age, 0, stat, -1, -1, -1, -1]);
        }
        award_manager_of_year(this, today, nation_award(this, 4), KEY_CLUB_COMP_WELSH_PREMIER_DIVISION);
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx05b_18 month award moves on");
        month_award_start_new_season(nation_award(this, 5), read_u16(today + cm_date::YEAR));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_wales_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &WALES_AWARDS, 6, "wales_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_wales_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "wales_awards_init");
}

/// Comparer for `wales_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_wales_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
