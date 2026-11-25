// ---------- rx12_7: the Belgian awards: the object, its eleven awards and their days (award/belgium_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\belgium_awards.cpp" (the error box).
const BELGIUM_AWARDS_CPP: usize = 0x989dfc;
/// `BelgiumAwards::init_awards(file)` (0x424510, thiscall, ret 4), through its hook.
const FN_BELGIUM_AWARDS_INIT: usize = 0x424510;

/// The Belgian awards' class: the vtable 0x967bfc, Belgium (`KEY_NATION_BELGIUM`), nation id 4, type 1, no month
/// ratings.
const BELGIUM_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x967bfc, nation_key: Some(KEY_NATION_BELGIUM), nation_id: 4, kind: 1, month_ratings: None };

/// Builds the Belgian awards (`nation_awards_construct` with `BELGIUM_AWARDS`): the saved awards with a file, else
/// `BelgiumAwards::init_awards(file)`. Returns this.
/// Original: `BelgiumAwards::BelgiumAwards` (0x424480, belgium_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn belgium_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &BELGIUM_AWARDS, file, core::mem::transmute(FN_BELGIUM_AWARDS_INIT))
}

/// Builds the eleven Belgian awards (`nation_awards_build`, the error box at line 0x69), no scope: the manager of the
/// year (national and world reputations 6000/1500, kind 1, not for players) and the golden boot (5000/2500, kind 0),
/// no competition; for the First Division the team of the week (team award, 0/0, span 1, mode 0), the top goalscorer
/// (3000/1500, option 1), the goalkeeper (5000/1500) and the young player of the year (4000/1500); for the Second
/// Division the team of the week, the top goalscorer (1500/750) and the revelation of the year (1000/700); for the
/// Third Division the team of the week and the top goalscorer (750/350); the year awards but the golden boot are kind
/// 1.
/// Original: `BelgiumAwards::init_awards` (0x424510, belgium_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn belgium_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2, d3) = (k(KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION), k(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), k(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, 1]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let specs = [
        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_BELGIAN_MANAGER_OF_THE_YEAR), 6000, 1500, u32::MAX, 1, 0, 0, f, 0]),
        AwardSpec::Year([n, 1, k(KEY_STAFF_COMP_BELGIAN_GOLDEN_BOOT), 5000, 2500, u32::MAX, 0, 0, 0, f, 1]), // rx12_7: the golden boot
        team(2, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(3, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_TOP_GOALSCORER, 3000, 1500, d1, 1),
        year(4, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_GOALKEEPER_OF_THE, 5000, 1500, d1, 0),
        year(5, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_YOUNG_PLAYER_OF_T, 4000, 1500, d1, 0),
        team(6, KEY_STAFF_COMP_BELGIAN_SECOND_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(7, KEY_STAFF_COMP_BELGIAN_SECOND_DIVISION_TOP_GOALSCORER, 1500, 750, d2, 1),
        year(8, KEY_STAFF_COMP_BELGIAN_SECOND_DIVISION_REVELATION_OF_TH, 1000, 700, d2, 0),
        team(9, KEY_STAFF_COMP_BELGIAN_THIRD_DIVISION_TEAM_OF_THE_WEEK, d3),
        year(10, KEY_STAFF_COMP_BELGIAN_THIRD_DIVISION_TOP_GOALSCORER, 750, 350, d3, 1),
    ];
    covered!("rx12_7 belgian awards built");
    nation_awards_build(this, &specs, BELGIUM_AWARDS_CPP, 0x69);
}

/// The Belgian awards' days: on 29 December (`awards_day_is`, month index 11), the golden boot (award 1) from the year
/// ratings (`award_player_of_year_from_ratings`, Belgian players, with the filters as the original passes them); on 6
/// June, from each division's season statistics (`award_player_of_year`): the manager of the year (First Division),
/// the First Division top goalscorer (statistic 2), goalkeeper (positions 1) and young player (up to 23) of the year,
/// the Second and Third Division top goalscorers and the Second Division revelation (up to 23); then, every day, the
/// base daily update.
/// Original: `BelgiumAwards::daily_update` (0x4249c0, belgium_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn belgium_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1d, 0xb) { // rx12_7: 29 December
        covered!("rx12_7 belgian golden boot");
        award_player_of_year_from_ratings(this, today, nation_award(this, 1), [-1, read_i32(KEY_NATION_BELGIUM), -1, -1, -1, 0, -1]);
    }
    if awards_day_is(today, 6, 5) { // rx12_7: 6 June
        covered!("rx12_7 belgian year awards");
        let (d1, d2, d3) = (KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION);
        award_manager_of_year(this, today, nation_award(this, 0), d1);
        award_player_of_year(this, d1, today, nation_award(this, 3), [-1, 0, 2, -1, -1, -1, -1]);
        award_player_of_year(this, d1, today, nation_award(this, 4), [-1, 1, 0x11, -1, -1, -1, -1]); // rx12_7: the goalkeeper
        award_player_of_year(this, d1, today, nation_award(this, 5), [0x17, 0, 0x11, -1, -1, -1, -1]); // rx12_7: the young player, up to 23
        award_player_of_year(this, d2, today, nation_award(this, 7), [-1, 0, 2, -1, -1, -1, -1]);
        award_player_of_year(this, d3, today, nation_award(this, 10), [-1, 0, 2, -1, -1, -1, -1]);
        award_player_of_year(this, d2, today, nation_award(this, 8), [0x17, 0, 0x11, -1, -1, -1, -1]);
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_belgium_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &BELGIUM_AWARDS, 11, "belgium_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_belgium_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "belgium_awards_init");
}

/// Comparer for `belgium_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_belgium_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
