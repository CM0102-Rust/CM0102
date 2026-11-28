use super::*;

/// Builds the USA awards (`nation_awards_construct` with `USA_AWARDS`): the saved awards with a file, else the month
/// ratings and `UsaAwards::init_awards(file)`. Returns this.
/// Original: `UsaAwards::UsaAwards` (0x906980, usa_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn usa_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &USA_AWARDS, file, core::mem::transmute(FN_USA_AWARDS_INIT))
}

/// Builds the seven USA awards (`nation_awards_build`, the error box at line 0x70), all of the MLS: the player of the
/// month (a month award from March to October of the current year, national reputation 3000), the manager of the year
/// (5000, not for players), the scoring champion (5000/500, option 1), the goalkeeper, the defender (5000/500) and the
/// rookie of the year (4000/500), and the all-pro team (team award, 3500/100); kind 0, no scope.
/// Original: `UsaAwards::init_awards` (0x906ac0, usa_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn usa_awards_init(this: usize, file: usize) {
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let mls = k(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE);
    let specs = [
        AwardSpec::Month([n, 0, k(KEY_STAFF_COMP_MLS_PLAYER_OF_THE_MONTH), 3000, 0, mls, 3, year, 0xa, year, f, 1]),
        AwardSpec::Year([n, 1, k(KEY_STAFF_COMP_MLS_MANAGER_OF_THE_YEAR), 5000, 0, mls, 0, 0, 0, f, 0]),
        AwardSpec::Year([n, 2, k(KEY_STAFF_COMP_MLS_SCORING_CHAMPION), 5000, 500, mls, 0, 0, 1, f, 1]),
        AwardSpec::Year([n, 3, k(KEY_STAFF_COMP_MLS_GOALKEEPER_OF_THE_YEAR), 5000, 500, mls, 0, 0, 0, f, 1]),
        AwardSpec::Year([n, 4, k(KEY_STAFF_COMP_MLS_DEFENDER_OF_THE_YEAR), 5000, 500, mls, 0, 0, 0, f, 1]),
        AwardSpec::Year([n, 5, k(KEY_STAFF_COMP_MLS_ROOKIE_OF_THE_YEAR), 4000, 500, mls, 0, 0, 0, f, 1]),
        AwardSpec::Team([n, 6, k(KEY_STAFF_COMP_MLS_ALL_PRO_TEAM), 3500, 100, mls, 0, 0, f, 1]),
    ];
    covered!("sx05b_16 usa awards built");
    nation_awards_build(this, &specs, USA_AWARDS_CPP, 0x70);
}

/// The USA awards' day: on 25 November (`awards_day_is`), the MLS manager of the year, the goalkeeper (positions 1),
/// the defender (6) and the rookie (age 21) of the year by the season's ratings (statistic 0x11), the scoring champion
/// (statistic 2) and the all-pro team (`NationAwards::award_team_of_season`). Every day, on the nation's season end day
/// (`is_season_end_day`), the month award moves on (`MONTH_AWARD::start_new_season` with today's year); then the base
/// daily update.
/// Original: `UsaAwards::daily_update` (0x906e10, usa_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn usa_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x19, 0xa) {
        covered!("sx05b_16 usa awards day");
        award_manager_of_year(this, today, nation_award(this, 1), KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE);
        for (k, age, positions, stat) in [(3, -1, 1, 0x11), (4, -1, 6, 0x11), (5, 0x15, 0, 0x11), (2, -1, 0, 2)] {
            award_player_of_year(this, KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE, today, nation_award(this, k), [age, positions, stat, -1, -1, -1, -1]);
        }
        award_team_of_season(this, KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE, nation_award(this, 6));
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx05b_16 month award moves on");
        month_award_start_new_season(nation_award(this, 0), read_u16(today + cm_date::YEAR));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_usa_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_USA_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &USA_AWARDS, 7, "usa_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_usa_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_USA_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "usa_awards_init");
}

/// Comparer for `usa_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_usa_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_USA_AWARDS_DAILY_UPDATE)(this, today)
}

/// The USA month: the MLS player of the month (award 0, a month award) from the nation's month ratings
/// (`award_month_player`), with no age limit (−1).
/// Original: `UsaAwards::on_month_change` (0x906f60, usa_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn usa_awards_on_month_change(this: usize) {
    covered!("sx05b_17 mls player of the month");
    award_month_player(this, nation_award(this, 0), -1);
}

/// Comparer for `usa_awards_on_month_change`: the award cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_usa_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_AWARDS_ON_MONTH_CHANGE)(this)
}

/// `UsaAwards::init_awards(file)` (0x906ac0, thiscall, ret 4), through its hook.
pub(crate) const FN_USA_AWARDS_INIT: usize = 0x906ac0;

/// The USA awards' class: the vtable 0x970830, the United States (`KEY_NATION_UNITED_STATES`), nation id 0x14, type 1,
/// with month ratings (the error box at line 0x48 when they cannot be built).
pub(crate) const USA_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x970830, nation_key: Some(KEY_NATION_UNITED_STATES), nation_id: 0x14, kind: 1,
    month_ratings: Some((USA_AWARDS_CPP, 0x48)) };

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\usa_awards.cpp" (the error boxes).
pub(crate) const USA_AWARDS_CPP: usize = 0xaa1f80;
