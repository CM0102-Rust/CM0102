// ---------- sx04c_2: the Japanese awards: the object, its eight awards and the season's end (award/japan_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\japan_awards.cpp" (the error box).
const JAPAN_AWARDS_CPP: usize = 0x9eaf84;
/// `JapanAwards::init_awards(file)` (0x666720, thiscall, ret 4), through its hook.
const FN_JAPAN_AWARDS_INIT: usize = 0x666720;

/// The Japanese awards' class: the vtable 0x96cce8, Japan (`KEY_NATION_JAPAN`), nation id 0xe, type 1, no month
/// ratings.
const JAPAN_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96cce8, nation_key: Some(KEY_NATION_JAPAN), nation_id: 0xe, kind: 1, month_ratings: None };

/// Builds the Japanese awards (`nation_awards_construct` with `JAPAN_AWARDS`): the saved awards with a file, else
/// `JapanAwards::init_awards(file)`. Returns this.
/// Original: `JapanAwards::JapanAwards` (0x666690, japan_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn japan_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &JAPAN_AWARDS, file, core::mem::transmute(FN_JAPAN_AWARDS_INIT))
}

/// Builds the eight Japanese awards (`nation_awards_build`, the error box at line 0x66), kind 0 and no competition for
/// the year awards: the manager (national and world reputations 5000/0, not for players), the player (5500/1500), the
/// top goalscorer (5000/500) and the young player of the year (3000/100); then the J1 League team of the week (2000/0,
/// mode 1) and team of the year (3000/100) and the J2 League team of the week (1000/0, mode 1) and team of the year
/// (1500/50), span 0.
/// Original: `JapanAwards::init_awards` (0x666720, japan_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn japan_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let year = |i: u32, key: usize, national: u32, world: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, u32::MAX, 0, 0, 0, f, players]);
    let team = |i: u32, key: usize, national: u32, world: u32, comp: usize, mode: u32| AwardSpec::Team([n, i, k(key), national, world, k(comp), 0, mode, f, 1]);
    let (j1, j2) = (KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2);
    let specs = [
        year(0, KEY_STAFF_COMP_JAPANESE_MANAGER_OF_THE_YEAR, 5000, 0, 0),
        year(1, KEY_STAFF_COMP_JAPANESE_PLAYER_OF_THE_YEAR, 5500, 1500, 1),
        year(2, KEY_STAFF_COMP_JAPANESE_TOP_GOALSCORER, 5000, 500, 1),
        year(3, KEY_STAFF_COMP_JAPANESE_YOUNG_PLAYER_OF_THE_YEAR, 3000, 100, 1),
        team(4, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_1_TEAM_OF_THE_WEEK, 2000, 0, j1, 1),
        team(5, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_1_TEAM_OF_THE_YEAR, 3000, 100, j1, 0),
        team(6, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_2_TEAM_OF_THE_WEEK, 1000, 0, j2, 1),
        team(7, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_2_TEAM_OF_THE_YEAR, 1500, 50, j2, 0),
    ];
    covered!("sx04c_2 japanese awards built");
    nation_awards_build(this, &specs, JAPAN_AWARDS_CPP, 0x66);
}

/// The Japanese awards' day: on 20 December (`awards_day_is`), from the J1 League's stats (`award_player_of_year`) the
/// player (statistic 0x11), the top goalscorer (2) and the young player (up to 21) of the year, the J1 manager of the
/// year and the teams of the season of the J1 and J2 Leagues; then, every day, the base daily update.
/// Original: `JapanAwards::daily_update` (0x666ab0, japan_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn japan_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 0xb) {
        covered!("sx04c_2 japanese awards day");
        let j1 = KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1;
        for (k, age, stat) in [(1, -1, 0x11), (2, -1, 2), (3, 0x15, 0x11)] {
            award_player_of_year(this, j1, today, nation_award(this, k), [age, 0, stat, -1, -1, -1, -1]);
        }
        award_manager_of_year(this, today, nation_award(this, 0), j1);
        award_team_of_season(this, j1, nation_award(this, 5));
        award_team_of_season(this, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, nation_award(this, 7));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_japan_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &JAPAN_AWARDS, 8, "japan_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_japan_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "japan_awards_init");
}

/// Comparer for `japan_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_japan_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
