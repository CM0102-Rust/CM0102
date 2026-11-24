// ---------- sx05b_14: the Turkish awards: the object, its six awards and the season's end (award/turkey_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\turkey_awards.cpp" (the error box).
const TURKEY_AWARDS_CPP: usize = 0xaa1970;
/// `TurkeyAwards::init_awards(file)` (0x901d00, thiscall, ret 4), through its hook.
const FN_TURKEY_AWARDS_INIT: usize = 0x901d00;

/// The Turkish awards' class: the vtable 0x970748, Turkey (`KEY_NATION_TURKEY`), nation id 0x1a, type 1.
const TURKEY_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x970748, nation_key: Some(KEY_NATION_TURKEY), nation_id: 0x1a, kind: 1, month_ratings: None };

/// Builds the Turkish awards (`nation_awards_construct` with `TURKEY_AWARDS`): the saved awards with a file, else
/// `TurkeyAwards::init_awards(file)`. Returns this.
/// Original: `TurkeyAwards::TurkeyAwards` (0x901c70, turkey_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn turkey_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &TURKEY_AWARDS, file, core::mem::transmute(FN_TURKEY_AWARDS_INIT))
}

/// Builds the six Turkish awards (`nation_awards_build`, the error box at line 0x64): the player of the year (national
/// and world reputations 3000 and 1500), the manager of the year (2000 and 1000, not for players), and four team awards
/// without reputations: the Premier Division team of the week and of the year, and the same two of the second
/// division (category A); kind 0, no scope.
/// Original: `TurkeyAwards::init_awards` (0x901d00, turkey_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn turkey_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (premier, second) = (k(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION), k(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A));
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 0, 0, f, 1]);
    let specs = [
        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_TURKISH_PLAYER_OF_THE_YEAR), 3000, 1500, u32::MAX, 0, 0, 0, f, 1]),
        AwardSpec::Year([n, 1, k(KEY_STAFF_COMP_TURKISH_MANAGER_OF_THE_YEAR), 2000, 1000, u32::MAX, 0, 0, 0, f, 0]),
        team(2, KEY_STAFF_COMP_TURKISH_PREMIER_DIVISION_TEAM_OF_THE_WEE, premier),
        team(3, KEY_STAFF_COMP_TURKISH_PREMIER_DIVISION_TEAM_OF_THE_YEA, premier),
        team(4, KEY_STAFF_COMP_TURKISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, second),
        team(5, KEY_STAFF_COMP_TURKISH_FIRST_DIVISION_TEAM_OF_THE_YEAR, second),
    ];
    covered!("sx05b_14 turkish awards built");
    nation_awards_build(this, &specs, TURKEY_AWARDS_CPP, 0x64);
}

/// The Turkish awards' day: on 30 May (`awards_day_is`), the player of the year (the top three of the year ratings,
/// Turkish players, mode 0x11), the manager of the year (from the Premier Division) and the teams of the year of the
/// Premier Division and of the second division (`NationAwards::award_team_of_season`); then, every day, the base daily
/// update.
/// Original: `TurkeyAwards::daily_update` (0x901fd0, turkey_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn turkey_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1e, 4) {
        covered!("sx05b_14 turkish awards day");
        award_top_three_from_ratings(this, today, nation_award(this, 0), [-1, -1, -1, read_i32(KEY_NATION_TURKEY), -1, 0, 0x11]);
        award_manager_of_year(this, today, nation_award(this, 1), KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION);
        award_team_of_season(this, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION, nation_award(this, 3));
        award_team_of_season(this, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, nation_award(this, 5));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_turkey_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &TURKEY_AWARDS, 6, "turkey_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_turkey_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "turkey_awards_init");
}

/// Comparer for `turkey_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_turkey_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
