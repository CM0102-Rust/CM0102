// rx04_5: the Norwegian and German forms of the player type words (Langlib.cpp, used by the grammar modifier).

/// The Norwegian player type words: rows of four strings (the base form, then forms 1, 2 and 3), ended by a row whose
/// base form is empty.
pub(crate) const LANGLIB_NORWEGIAN_PLAYER_TYPES: usize = 0x9eb328;
/// The German player type words, as `LANGLIB_NORWEGIAN_PLAYER_TYPES`.
pub(crate) const LANGLIB_GERMAN_PLAYER_TYPES: usize = 0x9eb7ac;
/// The size of one row of a player type table.
const LANGLIB_PLAYER_TYPE_ROW: usize = 0x10;

/// Finds `text` among the base forms of a player type table and overwrites it with its form `form` (1 to 3); any
/// other form, or a word not in the table, leaves the text as it is.
unsafe fn langlib_inflect_player_type(table: usize, text: usize, form: u8) {
    let mut row = table;
    while read_u8(read_usize(row)) != 0 {
        if c_string_bytes_eq(read_usize(row), text) {
            if (1..=3).contains(&form) {
                copy_c_string(text, read_usize(row + 4 * form as usize));
            }
            return;
        }
        row += LANGLIB_PLAYER_TYPE_ROW;
    }
}

/// Overwrites a Norwegian player type word with its form 1, 2 or 3 (such as the definite form), when it is one of the
/// words of `LANGLIB_NORWEGIAN_PLAYER_TYPES`.
/// Original: `langlib_inflect_norwegian_player_type` (0x6820a0, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_inflect_norwegian_player_type(text: usize, form: u32) {
    covered!("rx04_5 norwegian player type");
    langlib_inflect_player_type(LANGLIB_NORWEGIAN_PLAYER_TYPES, text, form as u8);
}

/// Overwrites a German player type word with its case form 1, 2 or 3, when it is one of the words of
/// `LANGLIB_GERMAN_PLAYER_TYPES`.
/// Original: `langlib_inflect_german_player_type` (0x682150, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_inflect_german_player_type(text: usize, form: u32) {
    covered!("rx04_5 german player type");
    langlib_inflect_player_type(LANGLIB_GERMAN_PLAYER_TYPES, text, form as u8);
}

/// Side by side: the copy on a copy of the text (0x100 bytes, more than the longest form), the original on the
/// caller's; the texts must be equal.
unsafe fn compare_inflect(o: unsafe extern "C" fn(usize, u32), copy: unsafe extern "C" fn(usize, u32), what: &str, text: usize, form: u32) {
    let mut mine = [0u8; 0x100];
    let before = c_string_bytes(text);
    mine[..before.len().min(0xff)].copy_from_slice(&before[..before.len().min(0xff)]);
    copy(mine.as_mut_ptr() as usize, form);
    o(text, form);
    let (ta, tb) = (c_string_bytes(text), c_string_bytes(mine.as_ptr() as usize));
    SCOREBOARD.record(ta == tb, || format!("{}({:?}, {}): original {:?} rust {:?}", what, String::from_utf8_lossy(&before), form as u8,
        String::from_utf8_lossy(&ta), String::from_utf8_lossy(&tb)));
}

pub(crate) unsafe extern "C" fn compare_langlib_inflect_norwegian_player_type(text: usize, form: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%);
    compare_inflect(o, langlib_inflect_norwegian_player_type, "langlib_inflect_norwegian_player_type", text, form);
}

pub(crate) unsafe extern "C" fn compare_langlib_inflect_german_player_type(text: usize, form: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(%%I1%%);
    compare_inflect(o, langlib_inflect_german_player_type, "langlib_inflect_german_player_type", text, form);
}
