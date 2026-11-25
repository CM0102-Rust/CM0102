// ---------- rx13_1: the European awards: the object, its four awards and the award day (award/european_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\european_awards.cpp" (the error boxes).
const EUROPEAN_AWARDS_CPP: usize = 0x9c48c8;
/// `EuropeanAwards::init_awards(file)` (0x583480, thiscall, ret 4), through its hook.
const FN_EUROPEAN_AWARDS_INIT: usize = 0x583480;

/// The European awards' class: the vtable 0x96a1bc, no nation record, nation id 2, type 2, no month ratings.
const EUROPEAN_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96a1bc, nation_key: None, nation_id: 2, kind: 2, month_ratings: None };

/// The four European player awards of the year in table order (goalkeeper, defender, midfielder, striker) with the
/// position mask each one's top three is chosen from.
const EUROPEAN_YEAR_AWARDS: [(usize, i32); 4] = [
    (KEY_STAFF_COMP_EUROPEAN_GOALKEEPER_OF_THE_YEAR, 1),
    (KEY_STAFF_COMP_EUROPEAN_DEFENDER_OF_THE_YEAR, 6),
    (KEY_STAFF_COMP_EUROPEAN_MIDFIELDER_OF_THE_YEAR, 0x38),
    (KEY_STAFF_COMP_EUROPEAN_STRIKER_OF_THE_YEAR, 0x40),
];

/// rule: the European awards object belongs to no nation; from a saved game it loads its awards, otherwise it creates
/// them
///
/// `nation_awards_construct` with `EUROPEAN_AWARDS`: the `NationAwards` base, the vtable, no nation record, the id 2
/// and the type 2; with a file, the saved awards; when they do not load (or without a file),
/// `EuropeanAwards::init_awards(file)`. Returns this.
/// Original: `EuropeanAwards::EuropeanAwards` (0x583400, european_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn european_awards_construct(this: usize, file: usize) -> usize {
    covered!("rx13_1 european awards object");
    nation_awards_construct(this, &EUROPEAN_AWARDS, file, core::mem::transmute(FN_EUROPEAN_AWARDS_INIT))
}

/// rule: Europe has four awards of the year, the goalkeeper, the defender, the midfielder and the striker of the year,
/// each with national reputation 8000 and world reputation 7000, for players, with no club competition
///
/// The count +8 = 4 and the table (the error box at line 0x60 without it), then the four year awards
/// (`nation_awards_build`): nation id, slot, the staff competition of `EUROPEAN_YEAR_AWARDS`, 8000, 7000, no club
/// competition (−1), kind 1, no scope, option 0, the file, for players. The original counts the slot index only when an
/// award's new succeeds (so one failed new shifts the rest); the copy keeps the fixed slots.
/// Original: `EuropeanAwards::init_awards` (0x583480, european_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn european_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let specs: Vec<AwardSpec> = EUROPEAN_YEAR_AWARDS.iter().enumerate()
        .map(|(k, &(key, _))| AwardSpec::Year([n, k as u32, read_u32(key), 8000, 7000, u32::MAX, 1, 0, 0, file as u32, 1])) // rx13_1: 8000 and 7000
        .collect();
    covered!("rx13_1 european awards built");
    nation_awards_build(this, &specs, EUROPEAN_AWARDS_CPP, 0x60);
}

/// rule: on 16 June of every season but 2001, the best three European players of the year by position: the
/// goalkeepers, the defenders, the midfielders and the strikers, from the world year ratings
///
/// Nothing when today's year is 2001 (`cm_date::YEAR` 0x7d1). On 16 June (`awards_day_is`(16, month 5)): a local
/// record list (`exe::new_llist`), then for each award of `EUROPEAN_YEAR_AWARDS` in order the top three
/// (`FN_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS`(today, the award, the award manager's world year ratings +0x10,
/// `KEY_CONTINENT_EUROPE`, the position mask, 1, the list)), and the list freed (`exe::drop_llist`). No base month
/// check.
/// Original: `EuropeanAwards::daily_update` (0x5836a0, european_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn european_awards_daily_update(this: usize, today: usize) {
    if read_u16(today + cm_date::YEAR) == 0x7d1 {
        return;
    }
    if !awards_day_is(today, 0x10, 5) { // rx13_1: 16 June
        return;
    }
    covered!("rx13_1 european awards given");
    let mut list = [0u8; 0xe];
    let list = list.as_mut_ptr() as usize;
    exe::new_llist(list);
    let top_three: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, u32, usize) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS);
    let ratings = read_usize(read_usize(G_AWARD_MANAGER) + 0x10); // the world year ratings
    for (k, &(_, mask)) in EUROPEAN_YEAR_AWARDS.iter().enumerate() {
        top_three(this, today, nation_award(this, k), ratings, read_u32(KEY_CONTINENT_EUROPE), mask, 1, list); // rx13_1: one shared list
    }
    exe::drop_llist(list);
}

pub(crate) unsafe extern "thiscall" fn compare_european_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &EUROPEAN_AWARDS, 4, "european_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_european_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "european_awards_init");
}

/// Only the original (the awards are given once and post news).
pub(crate) unsafe extern "thiscall" fn compare_european_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
