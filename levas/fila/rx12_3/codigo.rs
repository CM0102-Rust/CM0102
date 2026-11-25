// ---------- rx12_3: the German awards: the object, its fourteen awards and the season's end (award/germany_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\germany_awards.cpp" (the error boxes).
const GERMANY_AWARDS_CPP: usize = 0x9cd500;
/// `GermanyAwards::init_awards(file)` (0x5df790, thiscall, ret 4), through its hook.
const FN_GERMANY_AWARDS_INIT: usize = 0x5df790;

/// The German awards' class: the vtable 0x96b3b8, Germany (`KEY_NATION_GERMANY`), nation id 0xa, type 1, with month
/// ratings (the error box at line 0x50 when they cannot be built).
const GERMANY_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96b3b8, nation_key: Some(KEY_NATION_GERMANY), nation_id: 0xa, kind: 1,
    month_ratings: Some((GERMANY_AWARDS_CPP, 0x50)) };

/// Builds the German awards (`nation_awards_construct` with `GERMANY_AWARDS`): the saved awards with a file, else the
/// month ratings and `GermanyAwards::init_awards(file)`. Returns this.
/// Original: `GermanyAwards::GermanyAwards` (0x5df650, germany_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn germany_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &GERMANY_AWARDS, file, core::mem::transmute(FN_GERMANY_AWARDS_INIT))
}

/// Builds the fourteen German awards (`nation_awards_build`, the error box at line 0x78), kind 1, no scope: the German
/// goalkeeper (national and world reputations 6000/2000), sweeper, defender and midfielder (6500/2000), striker
/// (7000/2500) and player of the year (7500/2500), no competition; for the First Division the team of the week (team
/// award, 0/0, span 1, mode 2), the player of the month (a month award from August of the current year to May of the
/// next, 6000/2000), the manager of the year (6000/2000, not for players) and the top goalscorer (5000/2000, option
/// 1); the same four for the Second Division (4500/1000 and 3000/1000).
/// Original: `GermanyAwards::init_awards` (0x5df790, germany_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn germany_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION), k(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, players]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 7, y, 4, y + 1, f, 1]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 2, f, 1]);
    let specs = [
        year(0, KEY_STAFF_COMP_GERMAN_GOALKEEPER_OF_THE_YEAR, 6000, 2000, u32::MAX, 0, 1),
        year(1, KEY_STAFF_COMP_GERMAN_SWEEPER_OF_THE_YEAR, 6500, 2000, u32::MAX, 0, 1),
        year(2, KEY_STAFF_COMP_GERMAN_DEFENDER_OF_THE_YEAR, 6500, 2000, u32::MAX, 0, 1),
        year(3, KEY_STAFF_COMP_GERMAN_MIDFIELDER_OF_THE_YEAR, 6500, 2000, u32::MAX, 0, 1),
        year(4, KEY_STAFF_COMP_GERMAN_STRIKER_OF_THE_YEAR, 7000, 2500, u32::MAX, 0, 1),
        year(5, KEY_STAFF_COMP_GERMAN_PLAYER_OF_THE_YEAR, 7500, 2500, u32::MAX, 0, 1), // rx12_3: the player of the year
        team(6, KEY_STAFF_COMP_GERMAN_FIRST_DIVISION_TEAM_OF_THE_WEEK, d1),
        month(7, KEY_STAFF_COMP_GERMAN_FIRST_DIVISION_PLAYER_OF_THE_MONT, 6000, 2000, d1),
        year(8, KEY_STAFF_COMP_GERMAN_FIRST_DIVISION_MANAGER_OF_THE_YEA, 6000, 2000, d1, 0, 0),
        year(9, KEY_STAFF_COMP_GERMAN_FIRST_DIVISION_TOP_GOALSCORER, 5000, 2000, d1, 1, 1),
        team(10, KEY_STAFF_COMP_GERMAN_SECOND_DIVISION_TEAM_OF_THE_WEEK, d2),
        month(11, KEY_STAFF_COMP_GERMAN_SECOND_DIVISION_PLAYER_OF_THE_MON, 4500, 1000, d2),
        year(12, KEY_STAFF_COMP_GERMAN_SECOND_DIVISION_MANAGER_OF_THE_YE, 4500, 1000, d2, 0, 0),
        year(13, KEY_STAFF_COMP_GERMAN_SECOND_DIVISION_TOP_GOALSCORER, 3000, 1000, d2, 1, 1),
    ];
    covered!("rx12_3 german awards built");
    nation_awards_build(this, &specs, GERMANY_AWARDS_CPP, 0x78);
}

/// The German awards' day: on 6 June (`awards_day_is`, month index 5), the goalkeeper, sweeper, defender, midfielder
/// and striker of the year from the year ratings' top three (`award_top_three_from_ratings`, Germans, position masks
/// 1, 2, 4, 0x10 and 0x40), the First and Second Division top goalscorers (statistic 2), the player of the year (top
/// three, any position) and the managers of the year of both divisions. Every day, on the nation's season end day
/// (`is_season_end_day`), the two month awards move on (today's year); then the base daily update.
/// Original: `GermanyAwards::daily_update` (0x5dfd70, germany_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn germany_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 6, 5) { // rx12_3: 6 June
        covered!("rx12_3 german awards day");
        let ger = read_i32(KEY_NATION_GERMANY);
        for (k, mask) in [(0, 1), (1, 2), (2, 4), (3, 0x10), (4, 0x40)] { // rx12_3: the positions of the year
            award_top_three_from_ratings(this, today, nation_award(this, k), [ger, ger, -1, -1, -1, mask, 0x11]);
        }
        let (d1, d2) = (KEY_CLUB_COMP_GERMAN_FIRST_DIVISION, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION);
        award_player_of_year(this, d1, today, nation_award(this, 9), [-1, 0, 2, -1, -1, -1, -1]);
        award_player_of_year(this, d2, today, nation_award(this, 13), [-1, 0, 2, -1, -1, -1, -1]);
        award_top_three_from_ratings(this, today, nation_award(this, 5), [ger, ger, -1, -1, -1, -1, 0x11]);
        award_manager_of_year(this, today, nation_award(this, 8), d1);
        award_manager_of_year(this, today, nation_award(this, 12), d2);
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("rx12_3 month awards move on");
        let year = read_u16(today + cm_date::YEAR);
        month_award_start_new_season(nation_award(this, 7), year);
        month_award_start_new_season(nation_award(this, 11), year);
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_germany_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &GERMANY_AWARDS, 14, "germany_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_germany_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "germany_awards_init");
}

/// Comparer for `germany_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_germany_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
