// ---------- rx12_11: the Greek awards: the object, its ten awards and their day (award/greece_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\greece_awards.cpp" (the error box).
const GREECE_AWARDS_CPP: usize = 0x9cdb90;
/// `GreeceAwards::init_awards(file)` (0x5ec8e0, thiscall, ret 4), through its hook.
const FN_GREECE_AWARDS_INIT: usize = 0x5ec8e0;

/// The Greek awards' class: the vtable 0x96b770, Greece (`KEY_NATION_GREECE`), nation id 0x16, type 1, no month
/// ratings.
const GREECE_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96b770, nation_key: Some(KEY_NATION_GREECE), nation_id: 0x16, kind: 1, month_ratings: None };

/// The two Greek divisions in award order: the competition key and the first award of its block (the team of the
/// week, then the player, the manager and the top goalscorer of the year).
const GREECE_DIVISIONS: [(usize, usize); 2] = [(KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 2), (KEY_CLUB_COMP_GREEK_SECOND_DIVISION, 6)];

/// Builds the Greek awards (`nation_awards_construct` with `GREECE_AWARDS`): the saved awards with a file, else
/// `GreeceAwards::init_awards(file)`. Returns this.
/// Original: `GreeceAwards::GreeceAwards` (0x5ec850, greece_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn greece_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &GREECE_AWARDS, file, core::mem::transmute(FN_GREECE_AWARDS_INIT))
}

/// Builds the ten Greek awards (`nation_awards_build`, the error box at line 0x68), kind 1, no scope: the Greek player
/// and the foreign player of the year (national and world reputations 4000/2000, no competition); then for each
/// division (`GREECE_DIVISIONS`) the team of the week (team award, 0/0, span 1, mode 0), the player, the manager (not
/// for players) and the top goalscorer (option 1) of the year, 2000/1000 in the Premier Division and 1000/500 in the
/// Second.
/// Original: `GreeceAwards::init_awards` (0x5ec8e0, greece_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn greece_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_GREEK_PREMIER_DIVISION), k(KEY_CLUB_COMP_GREEK_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let specs = [
        year(0, KEY_STAFF_COMP_GREEK_PLAYER_OF_THE_YEAR, 4000, 2000, u32::MAX, 0, 1), // rx12_11: the player of the year
        year(1, KEY_STAFF_COMP_GREEK_FOREIGN_PLAYER_OF_THE_YEAR, 4000, 2000, u32::MAX, 0, 1),
        team(2, KEY_STAFF_COMP_GREEK_PREMIER_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(3, KEY_STAFF_COMP_GREEK_PREMIER_DIVISION_PLAYER_OF_THE_YEA, 2000, 1000, d1, 0, 1),
        year(4, KEY_STAFF_COMP_GREEK_PREMIER_DIVISION_MANAGER_OF_THE_YE, 2000, 1000, d1, 0, 0),
        year(5, KEY_STAFF_COMP_GREEK_PREMIER_DIVISION_TOP_GOALSCORER, 2000, 1000, d1, 1, 1),
        team(6, KEY_STAFF_COMP_GREEK_SECOND_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(7, KEY_STAFF_COMP_GREEK_SECOND_DIVISION_PLAYER_OF_THE_YEAR, 1000, 500, d2, 0, 1),
        year(8, KEY_STAFF_COMP_GREEK_SECOND_DIVISION_MANAGER_OF_THE_YEA, 1000, 500, d2, 0, 0),
        year(9, KEY_STAFF_COMP_GREEK_SECOND_DIVISION_TOP_GOALSCORER, 1000, 500, d2, 1, 1),
    ];
    covered!("rx12_11 greek awards built");
    nation_awards_build(this, &specs, GREECE_AWARDS_CPP, 0x68);
}

/// The player of the year of `award` from the award manager's world year ratings (+0x10, the players above 2000 of
/// reputation; `NationAwards::award_player_of_year_from_ratings`) with the seven filters as the nation passes them.
unsafe fn award_player_of_year_from_world_ratings(this: usize, today: usize, award: usize, filters: [i32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_PLAYER_OF_YEAR_FROM_RATINGS);
    let [a, b, c, d, e, g, h] = filters;
    f(this, today, award, read_usize(read_usize(G_AWARD_MANAGER) + 0x10), a, b, c, d, e, g, h) // the world year ratings
}

/// The Greek awards' day: on 30 May (`awards_day_is`, month index 4), the Greek player of the year (award 0) from the
/// award manager's world year ratings (`award_player_of_year_from_world_ratings`) and the foreign player of the year
/// (award 1) from the year ratings' top three (`award_top_three_from_ratings`), both with the filters as the original
/// passes them; then for each division (`GREECE_DIVISIONS`) from its season statistics the player of the year
/// (statistic 0x11), the manager of the year and the top goalscorer (statistic 2); then, every day, the base daily
/// update.
/// Original: `GreeceAwards::daily_update` (0x5ecd30, greece_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn greece_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1e, 4) { // rx12_11: 30 May
        covered!("rx12_11 greek awards day");
        let gre = read_i32(KEY_NATION_GREECE);
        award_player_of_year_from_world_ratings(this, today, nation_award(this, 0), [gre, -1, -1, -1, -1, 0, -1]);
        award_top_three_from_ratings(this, today, nation_award(this, 1), [gre, -1, gre, -1, -1, 0, 0x11]); // rx12_11: the foreign player
        for (comp, first) in GREECE_DIVISIONS {
            award_player_of_year(this, comp, today, nation_award(this, first + 1), [-1, 0, 0x11, -1, -1, -1, -1]); // rx12_11: the division player
            award_manager_of_year(this, today, nation_award(this, first + 2), comp);
            award_player_of_year(this, comp, today, nation_award(this, first + 3), [-1, 0, 2, -1, -1, -1, -1]);
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_greece_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &GREECE_AWARDS, 10, "greece_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_greece_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "greece_awards_init");
}

/// Comparer for `greece_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_greece_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
