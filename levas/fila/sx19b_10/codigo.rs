// ---------- sx19b_10: the find screen's search engine: construction, the clubs and the text matchers (search_eng.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\search_eng.cpp" (the error box).
const SEARCH_ENG_CPP: usize = 0xa7de94;
/// The search engine object: the kind byte (+0x33), the result array (+0x34, one pointer per person) and its count
/// (+0x38).
const SEARCH_ENG_KIND: usize = 0x33;
const SEARCH_ENG_RESULTS: usize = 0x34;
const SEARCH_ENG_COUNT: usize = 0x38;
/// `database_compare_clubs_by_short_name` (0x540080), the qsort order of the club results.
const CLUB_SHORT_NAME_ORDER: usize = 0x540080;

/// Constructs the find screen search engine, allocating a result array of one pointer per staff record and setting the
/// count to zero.
///
/// `SearchEngine::SearchEngine()` (thiscall), from the find screen (0x5a3173, 0x5a362f, 0x5a3b79, 0x5a40e9): the error
/// text buffer (`0xae24d0`) emptied, the kind byte (+0x33) 2, the result array (+0x34) a new block of `G_STAFF_COUNT`
/// pointers (`exe::crt_malloc`; failing: the search_eng.cpp "Error" box, line 0x33, and the array stays null), no
/// results (+0x38). Returns this.
/// Original: `SearchEngine::SearchEngine` (0x804f00, search_eng.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn new_search_engine(this: usize) -> usize {
    write_u8(0xae24d0, 0); // the error text buffer
    write_u8(this + SEARCH_ENG_KIND, 2);
    let results = exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(4));
    write_usize(this + SEARCH_ENG_RESULTS, results);
    if results == 0 {
        show_error_at(SEARCH_ENG_CPP, 0x33);
    }
    write_u32(this + SEARCH_ENG_COUNT, 0);
    this
}

/// Comparer for `new_search_engine`: it allocates, so only the original runs; checked afterwards: kind 2, an array and
/// no results.
pub(crate) unsafe extern "thiscall" fn compare_new_search_engine(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this);
    let ok = r == this && read_u8(this + SEARCH_ENG_KIND) == 2 && read_usize(this + SEARCH_ENG_RESULTS) != 0 && read_u32(this + SEARCH_ENG_COUNT) == 0;
    SCOREBOARD.record(ok, || format!("SearchEngine::SearchEngine (find screen): kind {} array {:#x}", read_u8(this + SEARCH_ENG_KIND), read_usize(this + SEARCH_ENG_RESULTS)));
    r
}

/// Destroys the search engine by freeing its result array when one is allocated (the pointer keeps its value).
/// Original: `SearchEngine::~SearchEngine` (0x804fd0, search_eng.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn search_engine_destructor(this: usize) {
    let results = read_usize(this + SEARCH_ENG_RESULTS);
    if results != 0 {
        exe::crt_free(results);
    }
}

/// Comparer for `search_engine_destructor`: it frees memory, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_search_engine_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
}

/// Reports whether the text occurs anywhere in a field, or with `split` whether any space-separated word of it occurs
/// there.
///
/// `search_text_contains(field, text, split)` (cdecl), from every matcher of the find screen and the news search
/// (0x7887f6...). Without `split`, the text is compared (`utils_string_compare` with 0 and its length, 0 = equal) at
/// each position of the field where it fits: 1 at the first match. With `split`, each word of the text (ended by a
/// space, temporarily replaced by a terminator, or by the text's end) that fits in the field is looked for the same
/// way: 1 at the first word found. A text longer than the field (or an empty text with `split`): 0. A null field or
/// text: the original ("Error" box, line 0x371, 0).
/// Original: `search_text_contains` (0x806260, search_eng.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_text_contains(field: usize, text: usize, split: u32) -> u32 {
    if field == 0 || text == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u32>(%%I3%%)(field, text, split); // the box
    }
    let field_len = c_string_len(field);
    let text_len = c_string_len(text);
    let found_at = |word: usize, len: usize| -> bool {
        (0..=field_len - len).any(|i| exe::utils_string_compare((field + i) as *const u8, word as *const u8, 0, len as u32) == 0)
    };
    if text_len > field_len {
        return 0;
    }
    if split == 0 {
        return found_at(text, text_len) as u32; // sx19b_10: the whole text
    }
    let mut word = text;
    for i in 0..text_len {
        if read_u8(text + i) != b' ' && i != text_len - 1 {
            continue;
        }
        let inside = i < text_len - 1;
        if inside {
            write_u8(text + i, 0);
        }
        let len = c_string_len(word);
        let found = len != 0 && len <= field_len && found_at(word, len);
        if inside {
            write_u8(text + i, b' ');
        }
        if found {
            covered!("sx19b_10 word found");
            return 1;
        }
        word = text + i + 1;
    }
    0
}

/// Comparer for `search_text_contains`: side by side (the split search restores the text it cuts).
pub(crate) unsafe extern "C" fn compare_search_text_contains(field: usize, text: usize, split: u32) -> u32 {
    let ro = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u32>(%%I3%%)(field, text, split);
    if field != 0 && text != 0 {
        let rr = search_text_contains(field, text, split);
        SCOREBOARD.record(ro == rr, || format!("search_text_contains (find screen, split {}): {:?} in {:?}: original {} rust {}", split,
            String::from_utf8_lossy(&c_string_bytes(text)), String::from_utf8_lossy(&c_string_bytes(field)), ro, rr));
    }
    ro
}

/// Whether the text occurs in any of the given name fields of a record (none when the record or the text is null).
/// ponytail: the original also tests each field's address for null, which an offset from a live record never is.
unsafe fn record_names_contain(record: usize, text: usize, fields: &[usize]) -> u32 {
    if record == 0 || text == 0 {
        return 0;
    }
    fields.iter().any(|&f| search_text_contains(record + f, text, 0) != 0) as u32
}

/// The name fields of a continent (5), a nation (4), a club (2) and a club competition (3).
const CONTINENT_NAMES: [usize; 5] = [4, 0x1f, 0x23, 0x3d, 0xa3];
const NATION_NAMES: [usize; 4] = [4, 0x38, 0x53, 0x57];
const CLUB_NAMES: [usize; 2] = [4, 0x38];
const DIVISION_NAMES: [usize; 3] = [4, 0x38, 0x53];

/// Reports whether the search text occurs in any of the five name fields of a continent record.
/// Original: `search_text_matches_continent` (0x805f10, search_eng.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_text_matches_continent(continent: usize, text: usize) -> u32 {
    record_names_contain(continent, text, &CONTINENT_NAMES)
}

/// Reports whether the search text occurs in any of the four name fields of a nation record.
/// Original: `search_text_matches_nation` (0x805fb0, search_eng.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_text_matches_nation(nation: usize, text: usize) -> u32 {
    record_names_contain(nation, text, &NATION_NAMES)
}

/// Reports whether the search text occurs in the long or short name of a club record.
/// Original: `search_text_matches_club` (0x806030, search_eng.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_text_matches_club(club: usize, text: usize) -> u32 {
    record_names_contain(club, text, &CLUB_NAMES) // sx19b_10: the club's names
}

/// Reports whether the search text occurs in any of the three name fields of a club competition record.
/// Original: `search_text_matches_division` (0x8061f0, search_eng.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_text_matches_division(division: usize, text: usize) -> u32 {
    record_names_contain(division, text, &DIVISION_NAMES)
}

/// Comparer of a name matcher: side by side.
unsafe fn compare_name_matcher(name: &str, record: usize, text: usize, ro: u32, rr: u32) -> u32 {
    SCOREBOARD.record(ro == rr, || format!("{} (find screen, record {:#x}, text {:?}): original {} rust {}", name, record,
        if text != 0 { String::from_utf8_lossy(&c_string_bytes(text)).into_owned() } else { String::new() }, ro, rr));
    ro
}

/// Comparer for `search_text_matches_continent`.
pub(crate) unsafe extern "C" fn compare_search_text_matches_continent(continent: usize, text: usize) -> u32 {
    let ro = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I4%%)(continent, text);
    compare_name_matcher("search_text_matches_continent", continent, text, ro, search_text_matches_continent(continent, text))
}

/// Comparer for `search_text_matches_nation`.
pub(crate) unsafe extern "C" fn compare_search_text_matches_nation(nation: usize, text: usize) -> u32 {
    let ro = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I5%%)(nation, text);
    compare_name_matcher("search_text_matches_nation", nation, text, ro, search_text_matches_nation(nation, text))
}

/// Comparer for `search_text_matches_club`.
pub(crate) unsafe extern "C" fn compare_search_text_matches_club(club: usize, text: usize) -> u32 {
    let ro = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I6%%)(club, text);
    compare_name_matcher("search_text_matches_club", club, text, ro, search_text_matches_club(club, text))
}

/// Comparer for `search_text_matches_division`.
pub(crate) unsafe extern "C" fn compare_search_text_matches_division(division: usize, text: usize) -> u32 {
    let ro = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I7%%)(division, text);
    compare_name_matcher("search_text_matches_division", division, text, ro, search_text_matches_division(division, text))
}

/// Fills the result array with clubs or national teams whose names, nation, division or continent text match the
/// search text, then sorts them.
///
/// `SearchEngine::find_clubs(text, national)` (thiscall, ret 8), from the find screen (0x5a3280, 0x5a3736): no
/// results; each of the first `G_CLUB_COUNT` − `G_NATION_COUNT` clubs of `G_CLUB_TABLE` whose national flag
/// (`database_is_national_team`) equals `national`, with a nation (+0x53) that has a continent (+0x71), goes into the
/// array when the text is empty or occurs in its names, its division's (+0x57), its nation's or its continent's
/// (`search_text_matches_*`). The array is sorted by short name (`exe::crt_qsort` with
/// `database_compare_clubs_by_short_name`). No club table (line 0x5b), no array (0x65) or no text (0x6f): the original
/// ("Error" box).
/// Original: `SearchEngine::find_clubs` (0x804fe0, search_eng.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn find_clubs(this: usize, text: usize, national: u32) {
    let mut out = read_usize(this + SEARCH_ENG_RESULTS);
    if read_usize(G_CLUB_TABLE) == 0 || out == 0 || text == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, text, national); // the box
    }
    write_u32(this + SEARCH_ENG_COUNT, 0);
    let count = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT));
    for i in 0..count.max(0) {
        let club = club_at(i as isize).addr();
        let nation = read_usize(club + 0x53); // the club's nation
        if exe::database_is_national_team(club) != national || nation == 0 || read_usize(nation + 0x71) == 0 {
            continue;
        }
        let matches = read_u8(text) == 0
            || search_text_matches_club(club, text) != 0
            || search_text_matches_division(read_usize(club + 0x57), text) != 0 // the club's division
            || search_text_matches_nation(nation, text) != 0
            || search_text_matches_continent(read_usize(nation + 0x71), text) != 0; // the nation's continent
        if matches {
            write_usize(out, club);
            out += 4;
            write_u32(this + SEARCH_ENG_COUNT, read_u32(this + SEARCH_ENG_COUNT) + 1);
        }
    }
    exe::crt_qsort(read_usize(this + SEARCH_ENG_RESULTS), read_u32(this + SEARCH_ENG_COUNT), 4, CLUB_SHORT_NAME_ORDER);
}

/// The results of the engine (count and pointers), for the comparers.
unsafe fn search_engine_results(this: usize) -> Vec<usize> {
    let a = read_usize(this + SEARCH_ENG_RESULTS);
    (0..read_u32(this + SEARCH_ENG_COUNT) as usize).map(|k| read_usize(a + k * 4)).collect()
}

/// Comparer for `find_clubs`: the original fills the array, its results are kept, the copy fills it again and the two
/// lists are compared; the original's list is put back.
pub(crate) unsafe extern "thiscall" fn compare_find_clubs(this: usize, text: usize, national: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, text, national);
    let a = read_usize(this + SEARCH_ENG_RESULTS);
    if read_usize(G_CLUB_TABLE) == 0 || a == 0 || text == 0 {
        return;
    }
    let ro = search_engine_results(this);
    find_clubs(this, text, national);
    let rr = search_engine_results(this);
    for (k, &c) in ro.iter().enumerate() {
        write_usize(a + k * 4, c);
    }
    write_u32(this + SEARCH_ENG_COUNT, ro.len() as u32);
    SCOREBOARD.record(ro == rr, || format!("SearchEngine::find_clubs (find screen, national {}): original {} clubs, rust {}", national, ro.len(), rr.len()));
}
