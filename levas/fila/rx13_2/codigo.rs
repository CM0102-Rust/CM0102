// ---------- rx13_2: the international awards: the object and its six awards (award/international_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\international_awards.cpp" (the error boxes).
const INTERNATIONAL_AWARDS_CPP: usize = 0x9e9dd4;
/// `InternationalAwards::init_awards(file)` (0x632b30, thiscall, ret 4), through its hook.
const FN_INTERNATIONAL_AWARDS_INIT: usize = 0x632b30;

/// The international awards' class: the vtable 0x96bb78, no nation record, nation id 1, type 4, no month ratings.
const INTERNATIONAL_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96bb78, nation_key: None, nation_id: 1, kind: 4, month_ratings: None };

/// The six international player awards of the year in table order with their (national, world) reputations: the FIFA
/// world player, the world footballer, the African, European and South American footballers and the Oceania player.
const INTERNATIONAL_YEAR_AWARDS: [(usize, u32, u32); 6] = [
    (KEY_STAFF_COMP_FIFA_WORLD_PLAYER_OF_THE_YEAR, 9000, 8000),
    (KEY_STAFF_COMP_WORLD_FOOTBALLER_OF_THE_YEAR, 9000, 8000),
    (KEY_STAFF_COMP_AFRICAN_FOOTBALLER_OF_THE_YEAR, 7000, 5250),
    (KEY_STAFF_COMP_EUROPEAN_FOOTBALLER_OF_THE_YEAR, 9000, 8000),
    (KEY_STAFF_COMP_SOUTH_AMERICAN_FOOTBALLER_OF_THE_YEAR, 7000, 6000),
    (KEY_STAFF_COMP_OCEANIA_PLAYER_OF_THE_YEAR, 5000, 3000),
];

/// The 14-byte scope of the FIFA world player award's shortlist: the kind byte 5, three dwords −1 and the byte 10 [the
/// meaning of each: inference; the YEAR_AWARD constructor hands it to the shortlist's constructor].
const FIFA_WORLD_PLAYER_SCOPE: [u8; 0xe] = [5, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xa];

/// rule: the international awards object belongs to no nation; from a saved game it loads its awards, otherwise it
/// creates them
///
/// `nation_awards_construct` with `INTERNATIONAL_AWARDS`: the `NationAwards` base, the vtable, no nation record, the
/// id 1 and the type 4; with a file, the saved awards; when they do not load (or without a file),
/// `InternationalAwards::init_awards(file)`. Returns this.
/// Original: `InternationalAwards::InternationalAwards` (0x632ac0, international_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn international_awards_construct(this: usize, file: usize) -> usize {
    covered!("rx13_2 international awards object");
    nation_awards_construct(this, &INTERNATIONAL_AWARDS, file, core::mem::transmute(FN_INTERNATIONAL_AWARDS_INIT))
}

/// rule: the six international awards of the year are the FIFA world player (from a shortlist), the world footballer,
/// the African, European and South American footballers and the Oceania player of the year, for players, with no club
/// competition; the reputations by award as in `INTERNATIONAL_YEAR_AWARDS`
///
/// The count +8 = 6 and the table (the error box at line 0x63 without it), then the six year awards
/// (`nation_awards_build`): nation id, slot, the staff competition, the two reputations, no club competition (−1), kind
/// 1, the scope (only the first award: a copy of `FIFA_WORLD_PLAYER_SCOPE` on the stack, which gives it a shortlist;
/// the others none), option 0, the file, for players. The original counts the slot index only when an award's new
/// succeeds (so one failed new shifts the rest); the copy keeps the fixed slots.
/// Original: `InternationalAwards::init_awards` (0x632b30, international_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn international_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let scope = FIFA_WORLD_PLAYER_SCOPE;
    let specs: Vec<AwardSpec> = INTERNATIONAL_YEAR_AWARDS.iter().enumerate().map(|(k, &(key, national, world))| {
        let shortlist = if k == 0 { scope.as_ptr() as u32 } else { 0 }; // rx13_2: only the FIFA world player
        AwardSpec::Year([n, k as u32, read_u32(key), national, world, u32::MAX, 1, shortlist, 0, file as u32, 1])
    }).collect();
    covered!("rx13_2 international awards built");
    nation_awards_build(this, &specs, INTERNATIONAL_AWARDS_CPP, 0x63);
}

pub(crate) unsafe extern "thiscall" fn compare_international_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &INTERNATIONAL_AWARDS, 6, "international_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_international_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "international_awards_init");
}
