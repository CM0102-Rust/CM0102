// ---------- sx05b_14: the Turkish awards: the object, its six awards and the season's end (award/turkey_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\turkey_awards.cpp" (the error box).
const TURKEY_AWARDS_CPP: usize = 0xaa1970;
/// `TurkeyAwards::init_awards(file)` (0x901d00, thiscall, ret 4), through its hook.
const FN_TURKEY_AWARDS_INIT: usize = 0x901d00;

/// The Turkish awards' class: the vtable 0x970748, Turkey (`KEY_NATION_TURKEY`), nation id 0x1a, type 1.
const TURKEY_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x970748, nation_key: Some(KEY_NATION_TURKEY), nation_id: 0x1a, kind: 1, month_ratings: None };

/// Builds the Turkish awards (`nation_awards_construct` with `TURKEY_AWARDS`): the saved awards with a file, else
/// `TurkeyAwards::init_awards(file)`. Returns this.
/// Original: `TurkeyAwards::TurkeyAwards` (0x901c70, turkey_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn turkey_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &TURKEY_AWARDS, file, core::mem::transmute(FN_TURKEY_AWARDS_INIT))
}

/// Builds the six Turkish awards (`nation_awards_build`, the error box at line 0x64): the player of the year (national
/// and world reputations 3000 and 1500), the manager of the year (2000 and 1000, not for players), and four team awards
/// without reputations: the Premier Division team of the week and of the year, and the same two of the second
/// division (category A); kind 0, no scope.
/// Original: `TurkeyAwards::init_awards` (0x901d00, turkey_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn turkey_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (premier, second) = (k(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION), k(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A));
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 0, 0, f, 1]);
    let specs = [
        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_TURKISH_PLAYER_OF_THE_YEAR), 3000, 1500, u32::MAX, 0, 0, 0, f, 1]),
        AwardSpec::Year([n, 1, k(KEY_STAFF_COMP_TURKISH_MANAGER_OF_THE_YEAR), 2000, 1000, u32::MAX, 0, 0, 0, f, 0]),
        team(2, KEY_STAFF_COMP_TURKISH_PREMIER_DIVISION_TEAM_OF_THE_WEE, premier),
        team(3, KEY_STAFF_COMP_TURKISH_PREMIER_DIVISION_TEAM_OF_THE_YEA, premier),
        team(4, KEY_STAFF_COMP_TURKISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, second),
        team(5, KEY_STAFF_COMP_TURKISH_FIRST_DIVISION_TEAM_OF_THE_YEAR, second),
    ];
    covered!("sx05b_14 turkish awards built");
    nation_awards_build(this, &specs, TURKEY_AWARDS_CPP, 0x64);
}

/// The Turkish awards' day: on 30 May (`awards_day_is`), the player of the year (the top three of the year ratings,
/// Turkish players, mode 0x11), the manager of the year (from the Premier Division) and the teams of the year of the
/// Premier Division and of the second division (`NationAwards::award_team_of_season`); then, every day, the base daily
/// update.
/// Original: `TurkeyAwards::daily_update` (0x901fd0, turkey_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn turkey_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1e, 4) {
        covered!("sx05b_14 turkish awards day");
        award_top_three_from_ratings(this, today, nation_award(this, 0), [-1, -1, -1, read_i32(KEY_NATION_TURKEY), -1, 0, 0x11]);
        award_manager_of_year(this, today, nation_award(this, 1), KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION);
        award_team_of_season(this, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION, nation_award(this, 3));
        award_team_of_season(this, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, nation_award(this, 5));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_turkey_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TURKEY_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &TURKEY_AWARDS, 6, "turkey_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_turkey_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_TURKEY_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "turkey_awards_init");
}

/// Comparer for `turkey_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_turkey_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_TURKEY_AWARDS_DAILY_UPDATE)(this, today)
}

// ---------- sx05b_15: the Turkish teams of the week (award/turkey_awards.cpp) ----------

/// Builds the team of the week of a Turkish team award: only for the Premier Division and second division teams of
/// the week (awards 2 and 4 of the table), `NationAwards::update_team_of_the_week` with the award's club competition
/// id.
/// Original: `TurkeyAwards::update_team_of_the_week` (0x902080, turkey_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn turkey_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[2, 4]);
}

/// Comparer for `turkey_awards_team_of_the_week`: the team of the week cannot be built twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_turkey_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_TURKEY_AWARDS_TEAM_OF_THE_WEEK)(this, award)
}

// ---------- sx04c_9: the Polish awards: the object, its ten awards and the season's end (award/poland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\poland_awards.cpp" (the error box).
const POLAND_AWARDS_CPP: usize = 0xa78680;
/// `PolandAwards::init_awards(file)` (0x7cd1c0, thiscall, ret 4), through its hook.
const FN_POLAND_AWARDS_INIT: usize = 0x7cd1c0;

/// The Polish awards' class: the vtable 0x96e608, Poland (`KEY_NATION_POLAND`), nation id 0x15, type 1, no month
/// ratings (so the base daily update never runs `PolandAwards::on_month_change`: defect of the original, the two
/// managers of the month are never given).
const POLAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e608, nation_key: Some(KEY_NATION_POLAND), nation_id: 0x15, kind: 1, month_ratings: None };

/// Poland's two divisions: the competition key and the first slot of the division's block (team of the week, player,
/// young player, manager of the year, manager of the month).
const POLAND_DIVISIONS: [(usize, usize); 2] = [(KEY_CLUB_COMP_POLISH_FIRST_DIVISION, 0), (KEY_CLUB_COMP_POLISH_SECOND_DIVISION, 5)];

/// Builds the Polish awards (`nation_awards_construct` with `POLAND_AWARDS`): the saved awards with a file, else
/// `PolandAwards::init_awards(file)`. Returns this.
/// Original: `PolandAwards::PolandAwards` (0x7cd130, poland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn poland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &POLAND_AWARDS, file, core::mem::transmute(FN_POLAND_AWARDS_INIT))
}

/// Builds the ten Polish awards (`nation_awards_build`, the error box at line 0x6b), kind 1, no scope; for each division
/// the team of the week (team award, national and world reputations 0/0, span 1), the player (First Division 3000/0,
/// Second 2000/0), the young player (2000/0, 1000/0) and the manager of the year (2500/1200, 1500/1000, not for
/// players) and the manager of the month (a month award from July of the current year to May of the next, 2000/1000,
/// 1000/500).
/// Original: `PolandAwards::init_awards` (0x7cd1c0, poland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn poland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_POLISH_FIRST_DIVISION), k(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, 0, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 6, y, 4, y + 1, f, 0]);
    let specs = [
        team(0, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(1, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 3000, 0, d1, 1),
        year(2, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_YOUNG_PLAYER_OF_TH, 2000, 0, d1, 1),
        year(3, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_MANAGER_OF_THE_YEA, 2500, 1200, d1, 0),
        month(4, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_MANAGER_OF_THE_MON, 2000, 1000, d1),
        team(5, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(6, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_PLAYER_OF_THE_YEA, 2000, 0, d2, 1),
        year(7, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_YOUNG_PLAYER_OF_T, 1000, 0, d2, 1),
        year(8, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_MANAGER_OF_THE_YE, 1500, 1000, d2, 0),
        month(9, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_MANAGER_OF_THE_MO, 1000, 500, d2),
    ];
    covered!("sx04c_9 polish awards built");
    nation_awards_build(this, &specs, POLAND_AWARDS_CPP, 0x6b);
}

/// The Polish awards' day: on 30 May (`awards_day_is`), for each division the player (statistic 0x11) and the young
/// player (up to 21) of the year from its stats and its manager of the year. Every day, on the nation's season end day
/// (`is_season_end_day`), the two managers of the month move on (`month_award_start_new_season` with today's year);
/// then the base daily update.
/// Original: `PolandAwards::daily_update` (0x7cd630, poland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn poland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1e, 4) {
        covered!("sx04c_9 polish awards day");
        for &(key, first) in &POLAND_DIVISIONS {
            award_player_of_year(this, key, today, nation_award(this, first + 1), [-1, 0, 0x11, -1, -1, -1, -1]);
            award_player_of_year(this, key, today, nation_award(this, first + 2), [0x15, 0, 0x11, -1, -1, -1, -1]);
            award_manager_of_year(this, today, nation_award(this, first + 3), key);
        }
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_9 month awards move on");
        for &(_, first) in &POLAND_DIVISIONS {
            month_award_start_new_season(nation_award(this, first + 4), read_u16(today + cm_date::YEAR));
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_poland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_POLAND_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &POLAND_AWARDS, 10, "poland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_poland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POLAND_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "poland_awards_init");
}

/// Comparer for `poland_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_poland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POLAND_AWARDS_DAILY_UPDATE)(this, today)
}

// ---------- sx04c_10: the Polish managers of the month (award/poland_awards.cpp) ----------

/// The Polish month: the manager of the month (`award_month_manager`) of the First Division (award 4) and of the
/// Second Division (award 9). Never runs in the game: the Polish awards have no month ratings, so the base daily update
/// never calls it (defect of the original).
/// Original: `PolandAwards::on_month_change` (0x7cd790, poland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn poland_awards_on_month_change(this: usize) {
    covered!("sx04c_10 polish managers of the month");
    award_month_manager(nation_award(this, 4), KEY_CLUB_COMP_POLISH_FIRST_DIVISION);
    award_month_manager(nation_award(this, 9), KEY_CLUB_COMP_POLISH_SECOND_DIVISION);
}

/// Comparer for `poland_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_poland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_POLAND_AWARDS_ON_MONTH_CHANGE)(this)
}

// ---------- sx04c_12: the Russian awards: the object, its seven awards and the season's end (award/russia_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\russia_awards.cpp" (the error box).
const RUSSIA_AWARDS_CPP: usize = 0xa7ad00;
/// `RussiaAwards::init_awards(file)` (0x7ec2a0, thiscall, ret 4), through its hook.
const FN_RUSSIA_AWARDS_INIT: usize = 0x7ec2a0;

/// The Russian awards' class: the vtable 0x96f32c, Russia (`KEY_NATION_RUSSIA`), nation id 0x1b, type 1, no month
/// ratings (so `RussiaAwards::on_month_change` never runs: defect of the original, the two players of the month are
/// never given).
const RUSSIA_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96f32c, nation_key: Some(KEY_NATION_RUSSIA), nation_id: 0x1b, kind: 1, month_ratings: None };

/// Builds the Russian awards (`nation_awards_construct` with `RUSSIA_AWARDS`): the saved awards with a file, else
/// `RussiaAwards::init_awards(file)`. Returns this.
/// Original: `RussiaAwards::RussiaAwards` (0x7ec210, russia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn russia_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &RUSSIA_AWARDS, file, core::mem::transmute(FN_RUSSIA_AWARDS_INIT))
}

/// Builds the seven Russian awards (`nation_awards_build`, the error box at line 0x69): the Russian player of the year
/// (national and world reputations 4000/2000, no competition, kind 0); then, for the Premier Division and the First
/// Division, the team of the week and the team of the year (team awards, 0/0, span 0, formation mode 3) and the player
/// of the month (a month award from April to November of the current year, 4000/2000 and 1000/500).
/// Original: `RussiaAwards::init_awards` (0x7ec2a0, russia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn russia_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (prem, first) = (k(KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION), k(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION));
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 0, 3, f, 1]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 3, y, 0xa, y, f, 1]);
    let specs = [
        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_RUSSIAN_PLAYER_OF_THE_YEAR), 4000, 2000, u32::MAX, 0, 0, 0, f, 1]),
        team(1, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_TEAM_OF_THE_WEE, prem),
        team(2, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_TEAM_OF_THE_YEA, prem),
        month(3, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_PLAYER_OF_THE_M, 4000, 2000, prem),
        team(4, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_TEAM_OF_THE_WEEK, first),
        team(5, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_TEAM_OF_THE_YEAR, first),
        month(6, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_PLAYER_OF_THE_MON, 1000, 500, first),
    ];
    covered!("sx04c_12 russian awards built");
    nation_awards_build(this, &specs, RUSSIA_AWARDS_CPP, 0x69);
}

/// The Russian awards' day: on 30 November (`awards_day_is`), the Russian player of the year from the year ratings'
/// top three (`award_top_three_from_ratings`: Russian players, mode 0x11) and the teams of the season of the Premier
/// Division and the First Division; then, every day, the base daily update (the month awards are never moved on).
/// Original: `RussiaAwards::daily_update` (0x7ec600, russia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn russia_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1e, 0xa) {
        covered!("sx04c_12 russian awards day");
        award_top_three_from_ratings(this, today, nation_award(this, 0), [-1, -1, -1, read_i32(KEY_NATION_RUSSIA), -1, 0, 0x11]);
        award_team_of_season(this, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, nation_award(this, 2));
        award_team_of_season(this, KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION, nation_award(this, 5));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_russia_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RUSSIA_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &RUSSIA_AWARDS, 7, "russia_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_russia_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RUSSIA_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "russia_awards_init");
}

/// Comparer for `russia_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_russia_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RUSSIA_AWARDS_DAILY_UPDATE)(this, today)
}
