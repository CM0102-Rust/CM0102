// ---------- sx04c_5: the Northern Irish awards: the object, its thirteen awards and the season's end (award/northern_ireland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\northern_ireland_awards.cpp" (the error boxes).
const NORTHERN_IRELAND_AWARDS_CPP: usize = 0xa697b4;
/// `NorthernIrelandAwards::init_awards(file)` (0x792ec0, thiscall, ret 4), through its hook.
const FN_NORTHERN_IRELAND_AWARDS_INIT: usize = 0x792ec0;

/// The Northern Irish awards' class: the vtable 0x96df88, Northern Ireland (`KEY_NATION_NORTHERN_IRELAND`), nation id
/// 0x19, type 1, with month ratings (the error box at line 0x52 when they cannot be built).
const NORTHERN_IRELAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96df88, nation_key: Some(KEY_NATION_NORTHERN_IRELAND),
    nation_id: 0x19, kind: 1, month_ratings: Some((NORTHERN_IRELAND_AWARDS_CPP, 0x52)) };

/// Northern Ireland's two divisions: the competition key and the first slot of the division's block (team of the
/// week, player, young player, top goalscorer, manager of the year, manager of the month).
const NORTHERN_IRELAND_DIVISIONS: [(usize, usize); 2] = [(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 1), (KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, 7)];

/// Builds the Northern Irish awards (`nation_awards_construct` with `NORTHERN_IRELAND_AWARDS`): the saved awards with a
/// file, else the month ratings and `NorthernIrelandAwards::init_awards(file)`. Returns this.
/// Original: `NorthernIrelandAwards::NorthernIrelandAwards` (0x792d80, northern_ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn northern_ireland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &NORTHERN_IRELAND_AWARDS, file, core::mem::transmute(FN_NORTHERN_IRELAND_AWARDS_INIT))
}

/// Builds the thirteen Northern Irish awards (`nation_awards_build`, the error box at line 0x7a), kind 1, no scope: the
/// Northern Irish player of the year (national and world reputations 3000/1500, no competition); then, of the Premier
/// Division, the team of the week (team award, 1000/500, span 1), the player (1500/500), the young player (500/250),
/// the top goalscorer (500/250, option 1), the manager of the year (2500/1200, not for players) and the manager of the
/// month (a month award from August of the current year to April of the next, 2000/1000); and of the First Division
/// the same (500/250, 500/200, 500/250, 500/250, 1000/500, 500/300).
/// Original: `NorthernIrelandAwards::init_awards` (0x792ec0, northern_ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn northern_ireland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (prem, first) = (k(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION), k(KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, players]);
    let team = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Team([n, i, k(key), national, world, comp, 1, 0, f, 1]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 7, y, 3, y + 1, f, 0]);
    let specs = [
        year(0, KEY_STAFF_COMP_N_IRISH_PLAYER_OF_THE_YEAR, 3000, 1500, u32::MAX, 0, 1),
        team(1, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_TEAM_OF_THE_WEE, 1000, 500, prem),
        year(2, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_PLAYER_OF_THE_Y, 1500, 500, prem, 0, 1),
        year(3, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_YOUNG_PLAYER_OF, 500, 250, prem, 0, 1),
        year(4, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_TOP_GOALSCORER, 500, 250, prem, 1, 1),
        year(5, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFF14, 2500, 1200, prem, 0, 0),
        month(6, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFF18, 2000, 1000, prem),
        team(7, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, 500, 250, first),
        year(8, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_PLAYER_OF_THE_YEA, 500, 200, first, 0, 1),
        year(9, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_YOUNG_PLAYER_OF_T, 500, 250, first, 0, 1),
        year(10, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_TOP_GOALSCORER, 500, 250, first, 1, 1),
        year(11, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_MANAGER_OF_THE_YE, 1000, 500, first, 0, 0),
        month(12, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_MANAGER_OF_THE_MO, 500, 300, first),
    ];
    covered!("sx04c_5 northern irish awards built");
    nation_awards_build(this, &specs, NORTHERN_IRELAND_AWARDS_CPP, 0x7a);
}

/// The Northern Irish awards' day: on 20 May (`awards_day_is`), the Northern Irish player of the year from the award
/// manager's world ratings (`award_player_of_year_from_world_ratings`, Northern Irish players, mode 0); then, for each
/// division, the player (statistic 0x11), the top goalscorer (2) and the young player (up to 21) of the year from its
/// stats and its manager of the year. Every day, on the nation's season end day (`is_season_end_day`), the two managers
/// of the month move on (`month_award_start_new_season` with today's year); then the base daily update.
/// Original: `NorthernIrelandAwards::daily_update` (0x793460, northern_ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn northern_ireland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 4) {
        covered!("sx04c_5 northern irish awards day");
        let ni = read_i32(KEY_NATION_NORTHERN_IRELAND);
        award_player_of_year_from_world_ratings(this, today, nation_award(this, 0), [ni, -1, -1, -1, -1, 0, -1]);
        for &(key, first) in &NORTHERN_IRELAND_DIVISIONS {
            for (k, age, stat) in [(1, -1, 0x11), (3, -1, 2), (2, 0x15, 0x11)] {
                award_player_of_year(this, key, today, nation_award(this, first + k), [age, 0, stat, -1, -1, -1, -1]);
            }
            award_manager_of_year(this, today, nation_award(this, first + 4), key);
        }
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_5 month awards move on");
        for &(_, first) in &NORTHERN_IRELAND_DIVISIONS {
            month_award_start_new_season(nation_award(this, first + 5), read_u16(today + cm_date::YEAR));
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_northern_ireland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &NORTHERN_IRELAND_AWARDS, 13, "northern_ireland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_northern_ireland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "northern_ireland_awards_init");
}

/// Comparer for `northern_ireland_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_northern_ireland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
