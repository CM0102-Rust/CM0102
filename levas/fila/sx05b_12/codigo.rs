// ---------- sx05b_12: the Swedish awards: the object, its ten awards and the season's end (award/sweden_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\sweden_awards.cpp" (the error box).
const SWEDEN_AWARDS_CPP: usize = 0xa8b5ec;
/// `SwedenAwards::init_awards(file)` (0x8937b0, thiscall, ret 4), through its hook.
const FN_SWEDEN_AWARDS_INIT: usize = 0x8937b0;

/// Builds the Swedish awards: the `NationAwards` base with the vtable 0x970354, Sweden (`KEY_NATION_SWEDEN`) as the
/// nation, nation id 0x13 and type 1; the saved awards with a file, else `SwedenAwards::init_awards(file)`. Returns
/// this.
/// Original: `SwedenAwards::SwedenAwards` (0x893720, sweden_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sweden_awards_construct(this: usize, file: usize) -> usize {
    let class = NationAwardsClass { vtable: 0x970354, nation_key: Some(KEY_NATION_SWEDEN), nation_id: 0x13, kind: 1, month_ratings: None };
    nation_awards_construct(this, &class, file, core::mem::transmute(FN_SWEDEN_AWARDS_INIT))
}

/// Builds the ten Swedish awards (`nation_awards_build`, the error box at line 0x68 when the table cannot be
/// allocated), each with the nation id and its index: the manager of the year (staff comp key, national and world
/// reputations 4000 and 500, not for players), the player, the fans' player, the goalkeeper, the defender, the
/// midfielder and the forward of the year (5000/2500, 4500/2500, 3000/1000, 3000/1000, 300/1000, 3000/100), the
/// Premier Division team of the week (team award, 1000/0, of the Premier Division), its top goalscorer (5000/2500, of
/// the Premier Division, option 1) and the First Division team of the week (500/0); kind 1 for all, no scope.
/// Original: `SwedenAwards::init_awards` (0x8937b0, sweden_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sweden_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let premier = k(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION);
    let year = |i: u32, key: usize, national: u32, world: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, u32::MAX, 1, 0, 0, f, players]);
    let specs = [
        year(0, KEY_STAFF_COMP_SWEDISH_MANAGER_OF_THE_YEAR, 4000, 500, 0),
        year(1, KEY_STAFF_COMP_SWEDISH_PLAYER_OF_THE_YEAR, 5000, 2500, 1),
        year(2, KEY_STAFF_COMP_SWEDISH_FANS_PLAYER_OF_THE_YEAR, 4500, 2500, 1),
        year(3, KEY_STAFF_COMP_SWEDISH_GOALKEEPER_OF_THE_YEAR, 3000, 1000, 1),
        year(4, KEY_STAFF_COMP_SWEDISH_DEFENDER_OF_THE_YEAR, 3000, 1000, 1),
        year(5, KEY_STAFF_COMP_SWEDISH_MIDFIELDER_OF_THE_YEAR, 300, 1000, 1),
        year(6, KEY_STAFF_COMP_SWEDISH_FORWARD_OF_THE_YEAR, 3000, 100, 1),
        AwardSpec::Team([n, 7, k(KEY_STAFF_COMP_SWEDISH_PREMIER_DIVISION_TEAM_OF_THE_WEE), 1000, 0, premier, 1, 0, f, 1]),
        AwardSpec::Year([n, 8, k(KEY_STAFF_COMP_SWEDISH_PREMIER_DIVISION_TOP_GOALSCORER), 5000, 2500, premier, 1, 0, 1, f, 1]),
        AwardSpec::Team([n, 9, k(KEY_STAFF_COMP_SWEDISH_FIRST_DIVISION_TEAM_OF_THE_WEEK), 500, 0, k(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), 1, 0, f, 1]),
    ];
    covered!("sx05b_12 swedish awards built");
    nation_awards_build(this, &specs, SWEDEN_AWARDS_CPP, 0x68);
}

/// The Swedish awards' day: on 1 December (`awards_day_is`), the manager of the year (from the Premier Division), the
/// player (mode 0) and the fans' player (mode 3) of the year from the year ratings (Swedish players), the goalkeeper,
/// defender, midfielder and forward of the year (the top three of the ratings with the position masks 1, 6, 0x38
/// and 0x40, mode 0x11) and the Premier Division top goalscorer (statistic 2); then, every day, the base daily update.
/// Original: `SwedenAwards::daily_update` (0x893bf0, sweden_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sweden_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 1, 0xb) {
        covered!("sx05b_12 swedish awards day");
        award_manager_of_year(this, today, nation_award(this, 0), KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION);
        let sweden = read_i32(KEY_NATION_SWEDEN);
        for (k, mode) in [(1, 0), (2, 3)] {
            award_player_of_year_from_ratings(this, today, nation_award(this, k), [-1, -1, -1, sweden, -1, mode, -1]);
        }
        for (k, mask) in [(3, 1), (4, 6), (5, 0x38), (6, 0x40)] {
            award_top_three_from_ratings(this, today, nation_award(this, k), [-1, -1, -1, sweden, -1, mask, 0x11]);
        }
        award_player_of_year(this, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, today, nation_award(this, 8), [-1, 0, 2, -1, -1, -1, -1]);
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_sweden_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    let class = NationAwardsClass { vtable: 0x970354, nation_key: Some(KEY_NATION_SWEDEN), nation_id: 0x13, kind: 1, month_ratings: None };
    compare_nation_awards_built(r, this, file, &class, 10, "sweden_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_sweden_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "sweden_awards_init");
}

/// Comparer for `sweden_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_sweden_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
