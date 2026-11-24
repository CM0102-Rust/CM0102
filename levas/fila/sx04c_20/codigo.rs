// ---------- sx04c_20: the old international awards: the object, its five awards and their days (award/old_international_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\old_international_awards.cpp" (the error box).
const OLD_INTERNATIONAL_AWARDS_CPP: usize = 0xa6a2e0;
/// `OldInternationalAwards::init_awards(file)` (0x79e5f0, thiscall, ret 4), through its hook.
const FN_OLD_INTERNATIONAL_AWARDS_INIT: usize = 0x79e5f0;
/// `nation_awards_award_from_shortlist` (0x768b20, thiscall(today, award), ret 8).
const FN_NATION_AWARDS_WINNER_FROM_SHORTLIST: usize = 0x768b20;
/// `nation_awards_award_top_three_from_ratings` (0x768720, thiscall(today, award, year ratings, continent, a, b, c), ret
/// 0x1c).
const FN_NATION_AWARDS_CONTINENT_TOP_THREE: usize = 0x768720;

/// The old international awards' class: the vtable 0x96e1a8, no nation (id 1), type 4, no month ratings.
const OLD_INTERNATIONAL_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e1a8, nation_key: None, nation_id: 1, kind: 4, month_ratings: None };

/// Builds the old international awards (`nation_awards_construct` with `OLD_INTERNATIONAL_AWARDS`): the saved awards
/// with a file, else `OldInternationalAwards::init_awards(file)`. Returns this. The award manager builds this class
/// instead of the current international one only with a database whose newer award keys are missing (−1).
/// Original: `OldInternationalAwards::OldInternationalAwards` (0x79e580, old_international_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_international_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &OLD_INTERNATIONAL_AWARDS, file, core::mem::transmute(FN_OLD_INTERNATIONAL_AWARDS_INIT))
}

/// Builds the five old international year awards (`nation_awards_build`, the error box at line 0x62), no competition,
/// kind 1: the FIFA world player of the year (national and world reputations 9000/8000), with a shortlist of the scope
/// the original keeps on its stack (a 14-byte record: kind 5, three −1 dwords, 0xa); the world (9000/8000), African
/// (7000/5250), European (9000/8000) and South American footballers of the year (7000/6000).
/// Original: `OldInternationalAwards::init_awards` (0x79e5f0, old_international_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_international_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let mut scope = [0u8; 14];
    scope[0] = 5;
    scope[1..13].fill(0xff);
    scope[13] = 0xa;
    let year = |i: u32, key: usize, national: u32, world: u32, scope: u32| AwardSpec::Year([n, i, k(key), national, world, u32::MAX, 1, scope, 0, f, 1]);
    let specs = [
        year(0, KEY_STAFF_COMP_FIFA_WORLD_PLAYER_OF_THE_YEAR, 9000, 8000, scope.as_ptr() as u32),
        year(1, KEY_STAFF_COMP_WORLD_FOOTBALLER_OF_THE_YEAR, 9000, 8000, 0),
        year(2, KEY_STAFF_COMP_AFRICAN_FOOTBALLER_OF_THE_YEAR, 7000, 5250, 0),
        year(3, KEY_STAFF_COMP_EUROPEAN_FOOTBALLER_OF_THE_YEAR, 9000, 8000, 0),
        year(4, KEY_STAFF_COMP_SOUTH_AMERICAN_FOOTBALLER_OF_THE_YEAR, 7000, 6000, 0),
    ];
    covered!("sx04c_20 old international awards built");
    nation_awards_build(this, &specs, OLD_INTERNATIONAL_AWARDS_CPP, 0x62);
}

/// The old international awards' days, outside 2001: on 6 May, the FIFA world player of the year (award 0) from the
/// award manager's world ratings (`award_player_of_year_from_world_ratings`, mode 2); on 6 June, award 0's winner from
/// its shortlist (`nation_awards_award_from_shortlist`) and the world footballer of the year (award 1, mode 3); on 16
/// June, the best three of Africa, Europe and South America (awards 2 to 4, `nation_awards_award_top_three_from_ratings`
/// with the world ratings, the continent key and 0, 1, 0). No base month check.
/// Original: `OldInternationalAwards::daily_update` (0x79e890, old_international_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_international_awards_daily_update(this: usize, today: usize) {
    if read_u16(today + cm_date::YEAR) == 0x7d1 {
        return;
    }
    if awards_day_is(today, 6, 4) {
        covered!("sx04c_20 old world player of the year");
        award_player_of_year_from_world_ratings(this, today, nation_award(this, 0), [-1, -1, -1, -1, -1, 2, -1]);
    }
    if awards_day_is(today, 6, 5) {
        let shortlist: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATION_AWARDS_WINNER_FROM_SHORTLIST);
        shortlist(this, today, nation_award(this, 0));
        award_player_of_year_from_world_ratings(this, today, nation_award(this, 1), [-1, -1, -1, -1, -1, 3, -1]);
    }
    if awards_day_is(today, 0x10, 5) {
        let top_three: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_NATION_AWARDS_CONTINENT_TOP_THREE);
        let ratings = read_usize(read_usize(G_AWARD_MANAGER) + AWARD_MANAGER_WORLD_RATINGS);
        for (k, continent) in [KEY_CONTINENT_AFRICA, KEY_CONTINENT_EUROPE, KEY_CONTINENT_SOUTH_AMERICA].iter().enumerate() {
            top_three(this, today, nation_award(this, 2 + k), ratings, read_u32(*continent), 0, 1, 0);
        }
    }
}

pub(crate) unsafe extern "thiscall" fn compare_old_international_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &OLD_INTERNATIONAL_AWARDS, 5, "old_international_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_old_international_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "old_international_awards_init");
}

/// Comparer for `old_international_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_international_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
