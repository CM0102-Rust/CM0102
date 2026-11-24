// ---------- sx04c_16: the Spanish awards: the object, its eight awards and the season's end (award/spain_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\spain_awards.cpp" (the error box).
const SPAIN_AWARDS_CPP: usize = 0xa82a90;
/// `SpainAwards::init_awards(file)` (0x858f30, thiscall, ret 4), through its hook.
const FN_SPAIN_AWARDS_INIT: usize = 0x858f30;

/// The Spanish awards' class: the vtable 0x96fdd8, Spain (`KEY_NATION_SPAIN`), nation id 0x12, type 1, no month ratings.
const SPAIN_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96fdd8, nation_key: Some(KEY_NATION_SPAIN), nation_id: 0x12, kind: 1, month_ratings: None };

/// Builds the Spanish awards (`nation_awards_construct` with `SPAIN_AWARDS`): the saved awards with a file, else
/// `SpainAwards::init_awards(file)`. Returns this.
/// Original: `SpainAwards::SpainAwards` (0x858ea0, spain_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn spain_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &SPAIN_AWARDS, file, core::mem::transmute(FN_SPAIN_AWARDS_INIT))
}

/// Builds the eight Spanish awards (`nation_awards_build`, the error box at line 0x66), kind 1, no scope: the Spanish
/// manager (national and world reputations 6000/2500, not for players) and player of the year (8000/4500), no
/// competition; then the First Division team of the week (team award, 0/0, span 1), top goalscorer (8000/5500, option
/// 1) and goalkeeper of the year (6000/3000), and the Second Division's (0/0, 4000/2700, 3000/1500).
/// Original: `SpainAwards::init_awards` (0x858f30, spain_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn spain_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION), k(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let specs = [
        year(0, KEY_STAFF_COMP_SPANISH_MANAGER_OF_THE_YEAR, 6000, 2500, u32::MAX, 0, 0),
        year(1, KEY_STAFF_COMP_SPANISH_PLAYER_OF_THE_YEAR, 8000, 4500, u32::MAX, 0, 1),
        team(2, KEY_STAFF_COMP_SPANISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(3, KEY_STAFF_COMP_SPANISH_FIRST_DIVISION_TOP_GOALSCORER, 8000, 5500, d1, 1, 1),
        year(4, KEY_STAFF_COMP_SPANISH_FIRST_DIVISION_GOALKEEPER_OF_THE, 6000, 3000, d1, 0, 1),
        team(5, KEY_STAFF_COMP_SPANISH_SECOND_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(6, KEY_STAFF_COMP_SPANISH_SECOND_DIVISION_TOP_GOALSCORER, 4000, 2700, d2, 1, 1),
        year(7, KEY_STAFF_COMP_SPANISH_SECOND_DIVISION_GOALKEEPER_OF_TH, 3000, 1500, d2, 0, 1),
    ];
    covered!("sx04c_16 spanish awards built");
    nation_awards_build(this, &specs, SPAIN_AWARDS_CPP, 0x66);
}

/// The Spanish awards' day: on 16 June (`awards_day_is`), the manager of the year (from the First Division); then from
/// the stats (`award_player_of_year`) the top goalscorers (statistic 2) of the First and Second Divisions, their
/// goalkeepers of the year (positions 1, statistic 0x12) and the player of the year (First Division, statistic 0x11);
/// then, every day, the base daily update.
/// Original: `SpainAwards::daily_update` (0x8592c0, spain_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn spain_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x10, 5) {
        covered!("sx04c_16 spanish awards day");
        let (d1, d2) = (KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION);
        award_manager_of_year(this, today, nation_award(this, 0), d1);
        for (key, k, positions, stat) in [(d1, 3, 0, 2), (d2, 6, 0, 2), (d1, 4, 1, 0x12), (d2, 7, 1, 0x12), (d1, 1, 0, 0x11)] {
            award_player_of_year(this, key, today, nation_award(this, k), [-1, positions, stat, -1, -1, -1, -1]);
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_spain_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &SPAIN_AWARDS, 8, "spain_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_spain_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "spain_awards_init");
}

/// Comparer for `spain_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_spain_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
