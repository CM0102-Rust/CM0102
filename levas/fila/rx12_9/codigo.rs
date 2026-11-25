// ---------- rx12_9: the Danish awards: the object, its ten awards and their days (award/denmark_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\denmark_awards.cpp" (the error box).
const DENMARK_AWARDS_CPP: usize = 0x9b8ee0;
/// `DenmarkAwards::init_awards(file)` (0x554e50, thiscall, ret 4), through its hook.
const FN_DENMARK_AWARDS_INIT: usize = 0x554e50;

/// The Danish awards' class: the vtable 0x96984c, Denmark (`KEY_NATION_DENMARK`), nation id 6, type 1, no month
/// ratings.
const DENMARK_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96984c, nation_key: Some(KEY_NATION_DENMARK), nation_id: 6, kind: 1, month_ratings: None };

/// The three Danish divisions in award order: the competition key and the first award of its block (the team of the
/// week, then the player and the young player of the year).
const DENMARK_DIVISIONS: [(usize, usize); 3] = [(KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, 1), (KEY_CLUB_COMP_DANISH_FIRST_DIVISION, 4),
    (KEY_CLUB_COMP_DANISH_SECOND_DIVISION, 7)];

/// Builds the Danish awards (`nation_awards_construct` with `DENMARK_AWARDS`): the saved awards with a file, else
/// `DenmarkAwards::init_awards(file)`. Returns this.
/// Original: `DenmarkAwards::DenmarkAwards` (0x554dc0, denmark_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn denmark_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &DENMARK_AWARDS, file, core::mem::transmute(FN_DENMARK_AWARDS_INIT))
}

/// Builds the ten Danish awards (`nation_awards_build`, the error box at line 0x67), no scope: the Danish player of the
/// year (national and world reputations 6000/2500, kind 0, no competition); then for each division (`DENMARK_DIVISIONS`)
/// the team of the week (team award, 0/0, span 1, mode 0), the player and the young player of the year (kind 1): the
/// Premier Division's 5000/1500 and 4500/1000, the First Division's 3000/0 and 2000/0, the Second Division's 2000/0 and
/// 1000/0.
/// Original: `DenmarkAwards::init_awards` (0x554e50, denmark_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn denmark_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2, d3) = (k(KEY_CLUB_COMP_DANISH_PREMIER_DIVISION), k(KEY_CLUB_COMP_DANISH_FIRST_DIVISION), k(KEY_CLUB_COMP_DANISH_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, 0, f, 1]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let specs = [
        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_DANISH_PLAYER_OF_THE_YEAR), 6000, 2500, u32::MAX, 0, 0, 0, f, 1]), // rx12_9: the player of the year
        team(1, KEY_STAFF_COMP_DANISH_PREMIER_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(2, KEY_STAFF_COMP_DANISH_PREMIER_DIVISION_PLAYER_OF_THE_YE, 5000, 1500, d1),
        year(3, KEY_STAFF_COMP_DANISH_PREMIER_DIVISION_YOUNG_PLAYER_OF, 4500, 1000, d1),
        team(4, KEY_STAFF_COMP_DANISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(5, KEY_STAFF_COMP_DANISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 3000, 0, d2),
        year(6, KEY_STAFF_COMP_DANISH_FIRST_DIVISION_YOUNG_PLAYER_OF_TH, 2000, 0, d2),
        team(7, KEY_STAFF_COMP_DANISH_SECOND_DIVISION_TEAM_OF_THE_WEEK, d3),
        year(8, KEY_STAFF_COMP_DANISH_SECOND_DIVISION_PLAYER_OF_THE_YEA, 2000, 0, d3),
        year(9, KEY_STAFF_COMP_DANISH_SECOND_DIVISION_YOUNG_PLAYER_OF_Y, 1000, 0, d3),
    ];
    covered!("rx12_9 danish awards built");
    nation_awards_build(this, &specs, DENMARK_AWARDS_CPP, 0x67);
}

/// The Danish awards' days: on 20 December (`awards_day_is`, month index 11), the Danish player of the year (award 0)
/// from the year ratings (`award_player_of_year_from_ratings`, Danish players, with the filters as the original passes
/// them); on 6 June, for each division (`DENMARK_DIVISIONS`) from its season statistics (`award_player_of_year`), the
/// player (statistic 0x11) and the young player (up to 21) of the year; then, every day, the base daily update.
/// Original: `DenmarkAwards::daily_update` (0x5552a0, denmark_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn denmark_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 0xb) { // rx12_9: 20 December
        covered!("rx12_9 danish player of the year");
        award_player_of_year_from_ratings(this, today, nation_award(this, 0), [read_i32(KEY_NATION_DENMARK), -1, -1, -1, -1, 0, -1]);
    }
    if awards_day_is(today, 6, 5) { // rx12_9: 6 June
        covered!("rx12_9 danish division awards");
        for (comp, first) in DENMARK_DIVISIONS {
            award_player_of_year(this, comp, today, nation_award(this, first + 1), [-1, 0, 0x11, -1, -1, -1, -1]);
            award_player_of_year(this, comp, today, nation_award(this, first + 2), [0x15, 0, 0x11, -1, -1, -1, -1]); // rx12_9: the young player, up to 21
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_denmark_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &DENMARK_AWARDS, 10, "denmark_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_denmark_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "denmark_awards_init");
}

/// Comparer for `denmark_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_denmark_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
