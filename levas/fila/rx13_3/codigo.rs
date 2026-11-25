// ---------- rx13_3: the Argentine awards: the object, its two team awards and the day (award/argentina_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\argentina_awards.cpp" (the error boxes).
const ARGENTINA_AWARDS_CPP: usize = 0x987924;
/// `ArgentinaAwards::init_awards(file)` (0x40a3e0, thiscall, ret 4), through its hook.
const FN_ARGENTINA_AWARDS_INIT: usize = 0x40a3e0;

/// The Argentine awards' class: the vtable 0x96748c, Argentina (`KEY_NATION_ARGENTINA`), nation id 3, type 1, no month
/// ratings.
const ARGENTINA_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96748c, nation_key: Some(KEY_NATION_ARGENTINA), nation_id: 3, kind: 1,
    month_ratings: None };

/// The Argentine team of the week awards in table order: the staff competition, the national reputation and the club
/// competition (the first and the second division).
const ARGENTINA_TEAM_AWARDS: [(usize, u32, usize); 2] = [
    (KEY_STAFF_COMP_ARGENTINIAN_PREMIER_DIVISION_TEAM_OF_THE, 2000, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION),
    (KEY_STAFF_COMP_ARGENTINIAN_SECOND_DIVISION_TEAM_OF_THE, 1000, KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION),
];

/// rule: the Argentine awards object is built for Argentina; from a saved game it loads its awards, otherwise it
/// creates them
///
/// `nation_awards_construct` with `ARGENTINA_AWARDS`: the `NationAwards` base, the vtable, Argentina's record from
/// `G_NATION_TABLE`, the id 3 and the type 1 (the original writes the id and the type before the record: the fields do
/// not overlap); with a file, the saved awards; when they do not load (or without a file),
/// `ArgentinaAwards::init_awards(file)`. Returns this.
/// Original: `ArgentinaAwards::ArgentinaAwards` (0x40a350, argentina_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn argentina_awards_construct(this: usize, file: usize) -> usize {
    covered!("rx13_3 argentine awards object");
    nation_awards_construct(this, &ARGENTINA_AWARDS, file, core::mem::transmute(FN_ARGENTINA_AWARDS_INIT))
}

/// rule: Argentina has two awards, the team of the week of the first division (national reputation 2000) and of the
/// second division (1000), no world reputation, for players
///
/// The count +8 = 2 and the table (the error box at line 0x5b without it), then the two team awards
/// (`nation_awards_build`): nation id, slot, the staff competition of `ARGENTINA_TEAM_AWARDS`, the national
/// reputation, world reputation 0, the division, span 1, mode 0, the file, for players. The original counts the slot
/// index only when the first award's new succeeds; the copy keeps the fixed slots.
/// Original: `ArgentinaAwards::init_awards` (0x40a3e0, argentina_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn argentina_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let specs: Vec<AwardSpec> = ARGENTINA_TEAM_AWARDS.iter().enumerate()
        .map(|(k, &(key, national, comp))| AwardSpec::Team([n, k as u32, read_u32(key), national, 0, read_u32(comp), 1, 0, file as u32, 1]))
        .collect();
    covered!("rx13_3 argentine awards built");
    nation_awards_build(this, &specs, ARGENTINA_AWARDS_CPP, 0x5b);
}

/// rule: the Argentine awards' day is only the month change of every nation's awards
///
/// `nation_awards_base_daily_update(this, today)`.
/// Original: `ArgentinaAwards::daily_update` (0x40a550, argentina_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn argentina_awards_daily_update(this: usize, today: usize) {
    covered!("rx13_3 argentine awards day");
    nation_awards_base_daily_update(this, today)
}

pub(crate) unsafe extern "thiscall" fn compare_argentina_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &ARGENTINA_AWARDS, 2, "argentina_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_argentina_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "argentina_awards_init");
}

/// Only the original (the month change gives the month awards once).
pub(crate) unsafe extern "thiscall" fn compare_argentina_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
