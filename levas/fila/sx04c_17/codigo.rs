// ---------- sx04c_17: the old Finnish awards: the object, its eleven awards and the season's end (award/old_finland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\old_finland_awards.cpp" (the error boxes).
const OLD_FINLAND_AWARDS_CPP: usize = 0xa6a268;
/// `OldFinlandAwards::init_awards(file)` (0x79d680, thiscall, ret 4), through its hook.
const FN_OLD_FINLAND_AWARDS_INIT: usize = 0x79d680;

/// The old Finnish awards' class: the vtable 0x96e180, Finland (`KEY_NATION_FINLAND`), nation id 8, type 1, with month
/// ratings (the error box at line 0x4d when they cannot be built).
const OLD_FINLAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e180, nation_key: Some(KEY_NATION_FINLAND), nation_id: 8, kind: 1,
    month_ratings: Some((OLD_FINLAND_AWARDS_CPP, 0x4d)) };

/// Builds the old Finnish awards (`nation_awards_construct` with `OLD_FINLAND_AWARDS`): the saved awards with a file,
/// else the month ratings and `OldFinlandAwards::init_awards(file)`. Returns this. The award manager builds this class
/// instead of the current Finnish one only with a database whose newer award keys are missing (−1).
/// Original: `OldFinlandAwards::OldFinlandAwards` (0x79d540, old_finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &OLD_FINLAND_AWARDS, file, core::mem::transmute(FN_OLD_FINLAND_AWARDS_INIT))
}

/// Builds the eleven old Finnish awards (`nation_awards_build`, the error box at line 0x75), kind 0, no scope: the
/// Finnish player (national and world reputations 3000/1500), young player (1500/1000), goalkeeper (1000/500) and
/// manager of the year (2000/1000, not for players), no competition; the Premier Division top goalscorer (2500/1250,
/// option 1), player of the month (a month award from May to October of the current year, 1000/500), team of the year
/// and team of the week (team awards, 0/0, span 0, mode 0); the First Division top goalscorer (1200/600), team of the
/// year and team of the week.
/// Original: `OldFinlandAwards::init_awards` (0x79d680, old_finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION), k(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 0, 0, option, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 0, 0, f, 1]);
    let specs = [
        year(0, KEY_STAFF_COMP_FINNISH_PLAYER_OF_THE_YEAR, 3000, 1500, u32::MAX, 0, 1),
        year(1, KEY_STAFF_COMP_FINNISH_YOUNG_PLAYER_OF_THE_YEAR, 1500, 1000, u32::MAX, 0, 1),
        year(2, KEY_STAFF_COMP_FINNISH_GOALKEEPER_OF_THE_YEAR, 1000, 500, u32::MAX, 0, 1),
        year(3, KEY_STAFF_COMP_FINNISH_MANAGER_OF_THE_YEAR, 2000, 1000, u32::MAX, 0, 0),
        year(4, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TOP_GOALSCORER, 2500, 1250, d1, 1, 1),
        AwardSpec::Month([n, 5, k(KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_PLAYER_OF_THE_M), 1000, 500, d1, 4, y, 9, y, f, 1]),
        team(6, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TEAM_OF_THE_YEA, d1),
        team(7, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TEAM_OF_THE_WEE, d1),
        year(8, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TOP_GOALSCORER, 1200, 600, d2, 1, 1),
        team(9, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TEAM_OF_THE_YEAR, d2),
        team(10, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d2),
    ];
    covered!("sx04c_17 old finnish awards built");
    nation_awards_build(this, &specs, OLD_FINLAND_AWARDS_CPP, 0x75);
}

/// The old Finnish awards' day: on 15 November (`awards_day_is`), the Finnish player, young player (up to 21) and
/// goalkeeper (position mask 1) of the year from the year ratings' top three (`award_top_three_from_ratings`, with the
/// filters as the original passes them), the manager of the year (from the Premier Division), the Premier Division top
/// goalscorer (statistic 2) and team of the season, and the First Division's. Every day, on the nation's season end day
/// (`is_season_end_day`), the player of the month award moves on (with today's year); then the base daily update.
/// Original: `OldFinlandAwards::daily_update` (0x79db30, old_finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0xf, 0xa) {
        covered!("sx04c_17 old finnish awards day");
        let fin = read_i32(KEY_NATION_FINLAND);
        award_top_three_from_ratings(this, today, nation_award(this, 0), [-1, -1, -1, fin, -1, 0, 0x11]);
        award_top_three_from_ratings(this, today, nation_award(this, 1), [fin, fin, -1, -1, 0x15, 0, 0x11]);
        award_top_three_from_ratings(this, today, nation_award(this, 2), [-1, fin, -1, -1, -1, 1, 0x11]);
        let (d1, d2) = (KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION);
        award_manager_of_year(this, today, nation_award(this, 3), d1);
        award_player_of_year(this, d1, today, nation_award(this, 4), [-1, 0, 2, -1, -1, -1, -1]);
        award_team_of_season(this, d1, nation_award(this, 6));
        award_player_of_year(this, d2, today, nation_award(this, 8), [-1, 0, 2, -1, -1, -1, -1]);
        award_team_of_season(this, d2, nation_award(this, 9));
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_17 month award moves on");
        month_award_start_new_season(nation_award(this, 5), read_u16(today + cm_date::YEAR));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &OLD_FINLAND_AWARDS, 11, "old_finland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "old_finland_awards_init");
}

/// Comparer for `old_finland_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
