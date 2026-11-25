// ---------- rx12_8: the Dutch awards: the object, its ten awards and their days (award/holland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\holland_awards.cpp" (the error box).
const HOLLAND_AWARDS_CPP: usize = 0x9cf014;
/// `HollandAwards::init_awards(file)` (0x5f9070, thiscall, ret 4), through its hook.
const FN_HOLLAND_AWARDS_INIT: usize = 0x5f9070;

/// The Dutch awards' class: the vtable 0x96ba74, Holland (`KEY_NATION_HOLLAND`), nation id 0xb, type 1, no month
/// ratings.
const HOLLAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96ba74, nation_key: Some(KEY_NATION_HOLLAND), nation_id: 0xb, kind: 1, month_ratings: None };

/// Builds the Dutch awards (`nation_awards_construct` with `HOLLAND_AWARDS`): the saved awards with a file, else
/// `HollandAwards::init_awards(file)`. Returns this.
/// Original: `HollandAwards::HollandAwards` (0x5f8fe0, holland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn holland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &HOLLAND_AWARDS, file, core::mem::transmute(FN_HOLLAND_AWARDS_INIT))
}

/// Builds the ten Dutch awards (`nation_awards_build`, the error box at line 0x67), no scope: the manager of the year
/// (national and world reputations 6000/1500, kind 1, not for players), the goalkeeper (5000/1500) and the young
/// player of the year (6000/2500), kind 0, no competition; for the KPN Telecompetitie (the premier division) the team
/// of the week (team award, 0/0, span 1, mode 0), the player (kind 1) and the top goalscorer (kind 1, option 1) of the
/// year and the golden boot (kind 0), all 6000/2500; for the Toto division the team of the week, the player of the year
/// (kind 1) and the golden boot (kind 0), 4000/1000.
/// Original: `HollandAwards::init_awards` (0x5f9070, holland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn holland_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION), k(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, kind: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, kind, 0, option, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let specs = [
        year(0, KEY_STAFF_COMP_DUTCH_MANAGER_OF_THE_YEAR, 6000, 1500, u32::MAX, 1, 0, 0),
        year(1, KEY_STAFF_COMP_DUTCH_GOALKEEPER_OF_THE_YEAR, 5000, 1500, u32::MAX, 0, 0, 1),
        year(2, KEY_STAFF_COMP_DUTCH_YOUNG_PLAYER_OF_THE_YEAR, 6000, 2500, u32::MAX, 0, 0, 1),
        team(3, KEY_STAFF_COMP_DUTCH_PREMIER_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(4, KEY_STAFF_COMP_DUTCH_PREMIER_DIVISION_PLAYER_OF_THE_YEA, 6000, 2500, d1, 1, 0, 1), // rx12_8: the player of the year
        year(5, KEY_STAFF_COMP_DUTCH_PREMIER_DIVISION_TOP_GOALSCORER, 6000, 2500, d1, 1, 1, 1),
        year(6, KEY_STAFF_COMP_DUTCH_PREMIER_DIVISION_GOLDEN_BOOT, 6000, 2500, d1, 0, 0, 1),
        team(7, KEY_STAFF_COMP_DUTCH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(8, KEY_STAFF_COMP_DUTCH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 4000, 1000, d2, 1, 0, 1),
        year(9, KEY_STAFF_COMP_DUTCH_FIRST_DIVISION_GOLDEN_BOOT, 4000, 1000, d2, 0, 0, 1),
    ];
    covered!("rx12_8 dutch awards built");
    nation_awards_build(this, &specs, HOLLAND_AWARDS_CPP, 0x67);
}

/// The Dutch awards' days: on 20 December (`awards_day_is`, month index 11), the golden boots of the KPN
/// Telecompetitie (award 6) and of the Toto division (award 9) from the year ratings
/// (`award_player_of_year_from_ratings`, Dutch players of that competition, with the filters as the original passes
/// them); on 25 March, from the KPN Telecompetitie's season statistics (`award_player_of_year`): the manager of the
/// year, the goalkeeper (positions 1), the young player (up to 21), the player (statistic 0x11) and the top goalscorer
/// (statistic 2) of the year, then the Toto division player of the year; then, every day, the base daily update.
/// Original: `HollandAwards::daily_update` (0x5f94c0, holland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn holland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 0xb) { // rx12_8: 20 December
        covered!("rx12_8 dutch golden boots");
        let hol = read_i32(KEY_NATION_HOLLAND);
        award_player_of_year_from_ratings(this, today, nation_award(this, 6), [-1, hol, -1, -1, read_i32(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION), 0, -1]);
        award_player_of_year_from_ratings(this, today, nation_award(this, 9), [-1, hol, -1, -1, read_i32(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION), 0, -1]);
    }
    if awards_day_is(today, 0x19, 2) { // rx12_8: 25 March
        covered!("rx12_8 dutch year awards");
        let (d1, d2) = (KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION);
        award_manager_of_year(this, today, nation_award(this, 0), d1);
        award_player_of_year(this, d1, today, nation_award(this, 1), [-1, 1, 0x11, -1, -1, -1, -1]); // rx12_8: the goalkeeper
        award_player_of_year(this, d1, today, nation_award(this, 2), [0x15, 0, 0x11, -1, -1, -1, -1]); // rx12_8: the young player, up to 21
        award_player_of_year(this, d1, today, nation_award(this, 4), [-1, 0, 0x11, -1, -1, -1, -1]);
        award_player_of_year(this, d1, today, nation_award(this, 5), [-1, 0, 2, -1, -1, -1, -1]);
        award_player_of_year(this, d2, today, nation_award(this, 8), [-1, 0, 0x11, -1, -1, -1, -1]);
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_holland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &HOLLAND_AWARDS, 10, "holland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_holland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "holland_awards_init");
}

/// Comparer for `holland_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_holland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
