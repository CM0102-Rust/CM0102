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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_WALES_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &WALES_AWARDS, 6, "wales_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_wales_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_WALES_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "wales_awards_init");
}

/// Comparer for `wales_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_wales_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_WALES_AWARDS_DAILY_UPDATE)(this, today)
}

// ---------- sx05b_19: the Welsh manager of the month and team of the week (award/wales_awards.cpp) ----------

/// The Welsh month: the manager of the month of the Welsh Premier Division (award 5, `award_month_manager`).
/// Original: `WalesAwards::on_month_change` (0x90f020, wales_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn wales_awards_on_month_change(this: usize) {
    covered!("sx05b_19 welsh manager of the month");
    award_month_manager(nation_award(this, 5), KEY_CLUB_COMP_WELSH_PREMIER_DIVISION);
}

/// Builds the team of the week of a Welsh team award: only for the team of the week (award 0 of the table).
/// Original: `WalesAwards::update_team_of_the_week` (0x90f060, wales_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wales_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[0]);
}

/// Comparer for `wales_awards_on_month_change`: the award cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_wales_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WALES_AWARDS_ON_MONTH_CHANGE)(this)
}

/// Comparer for `wales_awards_team_of_the_week`: the team of the week cannot be built twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_wales_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_WALES_AWARDS_TEAM_OF_THE_WEEK)(this, award)
}

// ---------- sx04c_14: the Scottish awards: the object, its 27 awards and the season's end (award/scotland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\scotland_awards.cpp" (the error boxes).
const SCOTLAND_AWARDS_CPP: usize = 0xa7b020;
/// `ScotlandAwards::init_awards(file)` (0x7f52b0, thiscall, ret 4), through its hook.
const FN_SCOTLAND_AWARDS_INIT: usize = 0x7f52b0;

/// The Scottish awards' class: the vtable 0x96f824, Scotland (`KEY_NATION_SCOTLAND`), nation id 0x11, type 1, with
/// month ratings (the error box at line 0x60 when they cannot be built).
const SCOTLAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96f824, nation_key: Some(KEY_NATION_SCOTLAND), nation_id: 0x11, kind: 1,
    month_ratings: Some((SCOTLAND_AWARDS_CPP, 0x60)) };

/// Scotland's four divisions in award order: each a block of six slots from 3 + 6 × division (team of the week, manager
/// of the month, player of the month, young player of the month, manager of the year, team of the year).
const SCOTLAND_DIVISIONS: [usize; 4] = [KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION,
    KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION];

/// The first slot of a Scottish division's block.
fn scotland_block(division: usize) -> usize {
    3 + 6 * division
}

/// Builds the Scottish awards (`nation_awards_construct` with `SCOTLAND_AWARDS`): the saved awards with a file, else
/// the month ratings and `ScotlandAwards::init_awards(file)`. Returns this.
/// Original: `ScotlandAwards::ScotlandAwards` (0x7f5170, scotland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn scotland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &SCOTLAND_AWARDS, file, core::mem::transmute(FN_SCOTLAND_AWARDS_INIT))
}

/// Builds the 27 Scottish awards (`nation_awards_build`, the error box at line 0x88), kind 1, no scope: the Scottish
/// player (national and world reputations 6000/2000), top goalscorer (5000/1500, option 1) and young player of the year
/// (4000/1000), no competition; then for each division (`SCOTLAND_DIVISIONS`) the team of the week (team award, span
/// 1), the manager, the player and the young player of the month (month awards from August of the current year to May
/// of the next), the manager of the year (not for players) and the team of the year, with the reputations, in that
/// order: Premier Division 2000/0, 3000/1000, 3500/500, 2500/0, 5000/1500, 4000/500; First Division 1000, 2000, 1500,
/// 1000, 3000, 3000; Second Division 0, 500, 500, 200, 1000, 1000; Third Division 0, 200, 200, 100, 500, 500 (world 0).
/// Original: `ScotlandAwards::init_awards` (0x7f52b0, scotland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn scotland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2, d3, d4) = (k(SCOTLAND_DIVISIONS[0]), k(SCOTLAND_DIVISIONS[1]), k(SCOTLAND_DIVISIONS[2]), k(SCOTLAND_DIVISIONS[3]));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, players]);
    let team = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Team([n, i, k(key), national, world, comp, 1, 0, f, 1]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32, players: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 7, y, 4, y + 1, f, players]);
    let specs = [
        year(0, KEY_STAFF_COMP_SCOTTISH_PLAYER_OF_THE_YEAR, 6000, 2000, u32::MAX, 0, 1),
        year(1, KEY_STAFF_COMP_SCOTTISH_TOP_GOALSCORER, 5000, 1500, u32::MAX, 1, 1),
        year(2, KEY_STAFF_COMP_SCOTTISH_YOUNG_PLAYER_OF_THE_YEAR, 4000, 1000, u32::MAX, 0, 1),
        team(3, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_TEAM_OF_THE_WE, 2000, 0, d1),
        month(4, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFC18, 3000, 1000, d1, 0),
        month(5, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_PLAYER_OF_THE, 3500, 500, d1, 1),
        month(6, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_YOUNG_PLAYER_O, 2500, 0, d1, 1),
        year(7, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFC14, 5000, 1500, d1, 0, 0),
        team(8, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_TEAM_OF_THE_YE, 4000, 500, d1),
        team(9, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, 1000, 0, d2),
        month(10, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_MANAGER_OF_THE_M, 2000, 0, d2, 0),
        month(11, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_PLAYER_OF_THE_MO, 1500, 0, d2, 1),
        month(12, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_YOUNG_PLAYER_OF, 1000, 0, d2, 1),
        year(13, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_MANAGER_OF_THE_Y, 3000, 0, d2, 0, 0),
        team(14, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_TEAM_OF_THE_YEAR, 3000, 0, d2),
        team(15, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_TEAM_OF_THE_WEE, 0, 0, d3),
        month(16, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_MANAGER_OF_THE_9CFC28, 500, 0, d3, 0),
        month(17, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_PLAYER_OF_THE_M, 500, 0, d3, 1),
        month(18, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_YOUNG_PLAYER_OF, 200, 0, d3, 1),
        year(19, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_MANAGER_OF_THE_9CFC24, 1000, 0, d3, 0, 0),
        team(20, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_TEAM_OF_THE_YEA, 1000, 0, d3),
        team(21, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_TEAM_OF_THE_WEEK, 0, 0, d4),
        month(22, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_MANAGER_OF_THE_M, 200, 0, d4, 0),
        month(23, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_PLAYER_OF_THE_MO, 200, 0, d4, 1),
        month(24, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_YOUNG_PLAYER_OF, 100, 0, d4, 1),
        year(25, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_MANAGER_OF_THE_Y, 500, 0, d4, 0, 0),
        team(26, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_TEAM_OF_THE_YEAR, 500, 0, d4),
    ];
    covered!("sx04c_14 scottish awards built");
    nation_awards_build(this, &specs, SCOTLAND_AWARDS_CPP, 0x88);
}

/// The Scottish awards' day: on the Saturday the game's date builder makes of 23 May (`exe::date_date_549f70` with
/// weekday 6), the manager of the year of each division; the Scottish player (mode 0x11), the young player (up to 21)
/// and the top goalscorer (statistic 2) of the year from the year ratings' top three
/// (`award_top_three_from_ratings`, Scottish players); and the team of the season of each division. Every day, on the
/// nation's season end day (`is_season_end_day`), the twelve month awards move on (`month_award_start_new_season` with
/// today's year); then the base daily update.
/// Original: `ScotlandAwards::daily_update` (0x7f5dc0, scotland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn scotland_awards_daily_update(this: usize, today: usize) {
    if is_date_today(make_date_dword(0x17, 4, read_u16(today + cm_date::YEAR) as u32, 6), today) {
        covered!("sx04c_14 scottish awards day");
        for (d, key) in SCOTLAND_DIVISIONS.iter().enumerate() {
            award_manager_of_year(this, today, nation_award(this, scotland_block(d) + 4), *key);
        }
        let scotland = read_i32(KEY_NATION_SCOTLAND);
        for (k, age, mode) in [(0, -1, 0x11), (2, 0x15, 0x11), (1, -1, 2)] {
            award_top_three_from_ratings(this, today, nation_award(this, k), [scotland, -1, -1, -1, age, 0, mode]);
        }
        for (d, key) in SCOTLAND_DIVISIONS.iter().enumerate() {
            award_team_of_season(this, *key, nation_award(this, scotland_block(d) + 5));
        }
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_14 month awards move on");
        for d in 0..4 {
            for k in 1..4 {
                month_award_start_new_season(nation_award(this, scotland_block(d) + k), read_u16(today + cm_date::YEAR));
            }
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_scotland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_SCOTLAND_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &SCOTLAND_AWARDS, 27, "scotland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_scotland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SCOTLAND_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "scotland_awards_init");
}

/// Comparer for `scotland_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_scotland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SCOTLAND_AWARDS_DAILY_UPDATE)(this, today)
}
