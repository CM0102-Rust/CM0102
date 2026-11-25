// ---------- rx13_5: the grammatical gender of an injury type's name (injury.cpp) ----------

/// The gender class of each injury type's name (0..=0x4c) in language 9, read from the original's jump table
/// 0x63074c (3 and 2 for a few names, 0 for none).
const INJURY_NAME_GENDER_LANGUAGE_9: [u8; 0x4d] = [
    1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1,
    1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 3, 1, 1, 0, 1, 0, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1,
];
/// The same in language 3 (the jump table 0x630880).
const INJURY_NAME_GENDER_LANGUAGE_3: [u8; 0x4d] = [
    2, 2, 1, 2, 2, 1, 0, 0, 1, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 2, 0, 0, 2, 2, 2,
    1, 2, 2, 2, 4, 1, 0, 2, 0, 2, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, 2, 1, 2, 2, 0,
    2, 0, 2, 0, 0, 1, 2, 2, 2, 1, 2, 1, 1, 2, 2, 2, 1, 2, 0, 2, 0, 1, 1, 1, 2,
];
/// The same in language 5 (the jump table 0x6309b4).
const INJURY_NAME_GENDER_LANGUAGE_5: [u8; 0x4d] = [
    1, 2, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 2, 1, 2, 1,
    1, 3, 1, 4, 1, 1, 2, 1, 2, 1, 1, 2, 2, 2, 2, 1, 2, 4, 2, 2, 1, 1, 1, 2, 1, 1,
    1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 4, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2,
];

/// rule: the gender class of an injury's name, which picks the article of the news text: in languages 9, 3 and 5 each
/// injury type has its own (a type out of the table, 1); in any other language, 0
///
/// `injury_get_type_name_gender(kind)` (the type as a signed byte), by the language (`exe::langlib_get_language`):
/// 9, 3 or 5, the type's entry in `INJURY_NAME_GENDER_LANGUAGE_9`, `_3` or `_5`, and 1 for a type outside 0..=0x4c
/// (negative too: the original's unsigned compare); otherwise 0. Only the low byte is the result; `this` (the injury
/// object) is not read.
/// Original: `Injury::get_type_name_gender` (0x6306e0, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn injury_get_type_name_gender(_this: usize, kind: u32) -> u32 {
    covered!("rx13_5 injury name gender");
    // ponytail: the original asks for the language up to three times; the getter has no side effect, once is the same
    let table = match exe::langlib_get_language() {
        LANGUAGE_9 => &INJURY_NAME_GENDER_LANGUAGE_9,
        LANGUAGE_3 => &INJURY_NAME_GENDER_LANGUAGE_3,
        LANGUAGE_5 => &INJURY_NAME_GENDER_LANGUAGE_5,
        _ => return 0, // rx13_5: no gender
    };
    table.get(kind as u8 as i8 as i32 as u32 as usize).copied().unwrap_or(1) as u32
}

/// Comparer for `injury_get_type_name_gender`: the low byte, side by side.
pub(crate) unsafe extern "thiscall" fn compare_injury_get_type_name_gender(this: usize, kind: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, kind);
    let n = injury_get_type_name_gender(this, kind);
    SCOREBOARD.record(r as u8 == n as u8, || format!("injury_get_type_name_gender({}) original {} rust {}", kind as u8 as i8, r as u8, n as u8));
    r
}
