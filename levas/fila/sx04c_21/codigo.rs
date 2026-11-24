// ---------- sx04c_21: the old Irish awards: the object, its seventeen awards and the season's end (award/old_ireland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\old_ireland_awards.cpp" (the error boxes).
const OLD_IRELAND_AWARDS_CPP: usize = 0xa6a324;
/// `OldIrelandAwards::init_awards(file)` (0x79eb40, thiscall, ret 4), through its hook.
const FN_OLD_IRELAND_AWARDS_INIT: usize = 0x79eb40;

/// The old Irish awards' class: the vtable 0x96e1bc, the Republic of Ireland (`KEY_NATION_REPUBLIC_OF_IRELAND`), nation
/// id 0xc, type 1, with month ratings (the error box at line 0x56 when they cannot be built).
const OLD_IRELAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e1bc, nation_key: Some(KEY_NATION_REPUBLIC_OF_IRELAND), nation_id: 0xc, kind: 1,
    month_ratings: Some((OLD_IRELAND_AWARDS_CPP, 0x56)) };

/// Builds the old Irish awards (`nation_awards_construct` with `OLD_IRELAND_AWARDS`): the saved awards with a file,
/// else the month ratings and `OldIrelandAwards::init_awards(file)`. Returns this. The award manager builds this class
/// instead of the current Irish one only with a database whose newer award keys are missing (−1).
/// Original: `OldIrelandAwards::OldIrelandAwards` (0x79ea00, old_ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_ireland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &OLD_IRELAND_AWARDS, file, core::mem::transmute(FN_OLD_IRELAND_AWARDS_INIT))
}

/// Builds the seventeen old Irish awards (`nation_awards_build`, the error box at line 0x7e), kind 1, no scope: the
/// Irish player (national and world reputations 3000/1500), young player (1500/750) and personality of the year (0/0),
/// no competition; then for the Premier Division the team of the week (team award, 1000/500, span 1), the player
/// (1500/750) and young player of the year (500/250), the golden boot (500/250, option 1), the team of the year
/// (1500/750), the manager of the year (2500/1200, not for players) and the manager of the month (a month award from
/// September of the current year to April of the next, 2000/1000); and the same for the First Division (500/250,
/// 750/400, 300/150, the bronze boot 500/250, 500/250, 1000/500, 1000/500).
/// Original: `OldIrelandAwards::init_awards` (0x79eb40, old_ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_ireland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION), k(KEY_CLUB_COMP_IRISH_FIRST_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, players]);
    let team = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Team([n, i, k(key), national, world, comp, 1, 0, f, 1]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 8, y, 3, y + 1, f, 0]);
    let none = u32::MAX;
    let specs = [
        year(0, KEY_STAFF_COMP_IRISH_PLAYER_OF_THE_YEAR, 3000, 1500, none, 0, 1),
        year(1, KEY_STAFF_COMP_IRISH_YOUNG_PLAYER_OF_THE_YEAR, 1500, 750, none, 0, 1),
        year(2, KEY_STAFF_COMP_IRISH_PERSONALITY_OF_THE_YEAR, 0, 0, none, 0, 1),
        team(3, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_TEAM_OF_THE_WEEK, 1000, 500, d1),
        year(4, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_PLAYER_OF_THE_YEA, 1500, 750, d1, 0, 1),
        year(5, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_YOUNG_PLAYER_OF_T, 500, 250, d1, 0, 1),
        year(6, KEY_STAFF_COMP_IRISH_LEAGUE_GOLDEN_BOOT, 500, 250, d1, 1, 1),
        team(7, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_TEAM_OF_THE_YEAR, 1500, 750, d1),
        year(8, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_YE, 2500, 1200, d1, 0, 0),
        month(9, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_MO, 2000, 1000, d1),
        team(10, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, 500, 250, d2),
        year(11, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 750, 400, d2, 0, 1),
        year(12, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_YOUNG_PLAYER_OF_THE, 300, 150, d2, 0, 1),
        year(13, KEY_STAFF_COMP_IRISH_LEAGUE_BRONZE_BOOT, 500, 250, d2, 1, 1),
        team(14, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_TEAM_OF_THE_YEAR, 500, 250, d2),
        year(15, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_MANAGER_OF_THE_YEAR, 1000, 500, d2, 0, 0),
        month(16, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_MANAGER_OF_THE_MONT, 1000, 500, d2),
    ];
    covered!("sx04c_21 old irish awards built");
    nation_awards_build(this, &specs, OLD_IRELAND_AWARDS_CPP, 0x7e);
}

/// The old Irish awards' day: on 1 May (`awards_day_is`), the Irish player, young player (filters as the original passes
/// them, the last 0x17) and personality of the year (mode 5) from the award manager's world ratings
/// (`award_player_of_year_from_world_ratings`); then, for each division, the player (statistic 0x11), the young player
/// (up to 23) and the top goalscorer (2) of the year from its stats, its team of the season and its manager of the
/// year, both managers of the year from the Premier Division (defect of the original: the First Division award goes to
/// a Premier Division manager). Every day, on the nation's season end day (`is_season_end_day`), the two managers of
/// the month move on (with today's year); then the base daily update.
/// Original: `OldIrelandAwards::daily_update` (0x79f250, old_ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_ireland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 1, 4) {
        covered!("sx04c_21 old irish awards day");
        let irl = read_i32(KEY_NATION_REPUBLIC_OF_IRELAND);
        for (k, filters) in [(0, [irl, -1, -1, -1, -1, 0, -1]), (1, [irl, -1, -1, -1, -1, 0, 0x17]), (2, [-1, irl, -1, -1, -1, 5, -1])] {
            award_player_of_year_from_world_ratings(this, today, nation_award(this, k), filters);
        }
        for (key, first) in [(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, 4), (KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 11)] {
            for (k, age, stat) in [(0, -1, 0x11), (1, 0x17, 0x11), (2, -1, 2)] {
                award_player_of_year(this, key, today, nation_award(this, first + k), [age, 0, stat, -1, -1, -1, -1]);
            }
            award_team_of_season(this, key, nation_award(this, first + 3));
            award_manager_of_year(this, today, nation_award(this, first + 4), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION);
        }
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_21 month awards move on");
        month_award_start_new_season(nation_award(this, 9), read_u16(today + cm_date::YEAR));
        month_award_start_new_season(nation_award(this, 16), read_u16(today + cm_date::YEAR));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_old_ireland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &OLD_IRELAND_AWARDS, 17, "old_ireland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_old_ireland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "old_ireland_awards_init");
}

/// Comparer for `old_ireland_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_ireland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
