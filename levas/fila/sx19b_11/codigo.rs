// ---------- sx19b_11: the find screen's search engine: the people (search_eng.cpp) ----------

/// The global text the staff name matcher formats each name style into (200 bytes).
const STAFF_NAME_TEXT: usize = 0xdd6bfc;

/// Formats the staff member's name in each of seven styles and reports whether the search text occurs in any of them.
///
/// `search_text_matches_staff_name(person, text)` (cdecl), from the find screen's people search (0x805b8c) and the
/// news search (0x807ef8): each style 1..7 of `database_format_staff_name` into `STAFF_NAME_TEXT` (200 bytes), looked
/// up with `search_text_contains`: 1 at the first; none, or a null person or text: 0.
/// Original: `search_text_matches_staff_name` (0x806080, search_eng.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_text_matches_staff_name(person: usize, text: usize) -> u32 {
    if person == 0 || text == 0 {
        return 0;
    }
    for style in 1..=7u32 { // sx19b_11: the seven name styles
        exe::database_format_staff_name(person, STAFF_NAME_TEXT, 200, style);
        if search_text_contains(STAFF_NAME_TEXT, text, 0) != 0 {
            return 1;
        }
    }
    0
}

/// Comparer for `search_text_matches_staff_name`: side by side (both format into the same global text, one after the
/// other).
pub(crate) unsafe extern "C" fn compare_search_text_matches_staff_name(person: usize, text: usize) -> u32 {
    let ro = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%)(person, text);
    let rr = search_text_matches_staff_name(person, text);
    SCOREBOARD.record(ro == rr, || format!("search_text_matches_staff_name (find screen, person {}): original {} rust {}",
        if person != 0 { read_i32(person) } else { -1 }, ro, rr));
    ro
}

/// Whether a person's classification (`staff::CLASSIFICATION`) fits the search: 3 always; otherwise 1 when looking for
/// players, 2 when looking for non-players.
unsafe fn staff_kind_fits(p: usize, players: u32) -> bool {
    let k = read_u8(p + staff::CLASSIFICATION);
    k == 3 || if players != 0 { k == 1 } else { k == 2 }
}

/// The text matchers of a person for the plain search: an empty text, his names, his nation and its continent, his
/// second nation, his club, its division, its nation and that nation's continent.
unsafe fn staff_matches_text(p: usize, text: usize) -> bool {
    let nation = read_usize(p + staff::NATION);
    let club = read_usize(p + staff::CLUB_JOB);
    let club_nation = if club != 0 { read_usize(club + 0x53) } else { 0 }; // the club's nation
    read_u8(text) == 0
        || search_text_matches_staff_name(p, text) != 0
        || search_text_matches_nation(nation, text) != 0
        || (nation != 0 && search_text_matches_continent(read_usize(nation + 0x71), text) != 0) // the nation's continent
        || search_text_matches_nation(read_usize(p + staff::SECOND_NATION), text) != 0
        || search_text_matches_club(club, text) != 0
        || (club != 0 && search_text_matches_division(read_usize(club + 0x57), text) != 0) // the club's division
        || search_text_matches_nation(club_nation, text) != 0
        || (club_nation != 0 && search_text_matches_continent(read_usize(club_nation + 0x71), text) != 0)
}

/// The special words of the people search, compared whole (case-sensitive).
const KEYWORD_NO_CLUB: &[u8] = b"No Club";
const KEYWORD_RECURRING: &[u8] = b"Recurring";
const KEYWORD_SCHOOL: &[u8] = b"School";
const KEYWORD_RETIRED: &[u8] = b"Retired";
const KEYWORD_RETIRING: &[u8] = b"Retiring";
const KEYWORD_SEDGEFORD: &[u8] = b"Sedgeford";
const KEYWORD_AWOL: &[u8] = b"awol";
const KEYWORD_SKIVE: &[u8] = b"skive";
/// The words that look at the person's contract with his club, and the flag bits (contract +0x45) each wants.
const CONTRACT_KEYWORDS: [(&[u8], u32); 4] = [(b"big club", 1), (b"contract", 2), (b"dislikes", 0x3800), (b"homesick", 0x20000)];

/// One pass over the people for a special word: each person that `keep` accepts goes into the results.
unsafe fn staff_pass(this: usize, keep: impl Fn(usize, usize) -> bool) {
    let mut out = read_usize(this + SEARCH_ENG_RESULTS);
    for i in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
        let p = staff_at(i as isize).addr();
        if keep(p, i) {
            write_usize(out, p);
            out += 4;
            write_u32(this + SEARCH_ENG_COUNT, read_u32(this + SEARCH_ENG_COUNT) + 1);
        }
    }
}

/// The pass of a contract word, as the original walks it: the person pointer only moves on past a person with a club,
/// so from the first person without a club to the end of the count the same person is looked at again (a defect of
/// the original: those words only list people up to the first one without a club).
unsafe fn staff_contract_pass(this: usize, bits: u32) {
    let mut out = read_usize(this + SEARCH_ENG_RESULTS);
    let mut p = read_usize(G_STAFF_TABLE);
    for _ in 0..read_i32(G_STAFF_COUNT).max(0) {
        let club = read_usize(p + staff::CLUB_JOB);
        if club == 0 {
            continue; // sx19b_11: the pointer stays (the original's defect)
        }
        let c = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
        if c != 0 && read_u32(c + 0x45) & bits != 0 { // the contract's flags
            write_usize(out, p);
            out += 4;
            write_u32(this + SEARCH_ENG_COUNT, read_u32(this + SEARCH_ENG_COUNT) + 1);
        }
        p += staff::SIZE;
    }
}

/// Fills the result array with players or non-players that match the search text, or a special keyword such as No
/// Club, Retired or awol.
///
/// `SearchEngine::find_staff(text, players)` (thiscall, ret 8), from the find screen (0x5a3c8c, 0x5a41fc). No results;
/// with a non-empty text, a special word (compared whole) gives its own pass over the people (`staff_kind_fits` with
/// `players` where noted):
/// - "No Club": fitting people without a club job; "School": fitting people under 18; "Retired" / "Retiring": fitting
///   people whose state byte (`G_STAFF_STATE_TABLE`, 0x4f bytes each, +0xb) has bits 0x30 = 0x20 / 0x10;
/// - "Recurring": players (`staff::PLAYER`) with a recurring injury (`Injury::get_recurring_injury_level` of
///   `G_INJURY`); "awol": people away without leave (`AWOL::is_player_awol` with 0); "skive": people who missed training
///   (`AWOL::has_missed_training`);
/// - "Sedgeford": people of classification 3 (or 2 when not looking for players) with a player record whose +7 is over
///   160 and +5 over 140, and no club;
/// - "big club", "contract", "dislikes", "homesick": people whose contract with their club
///   (`get_contract_for_club`) has the word's flag (+0x45; `staff_contract_pass`).
/// Otherwise every fitting person whose texts match (`staff_matches_text`). No staff table (line 0xac), no array (0xb6)
/// or no text (0xc0): the original ("Error" box).
/// Original: `SearchEngine::find_staff` (0x805310, search_eng.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn find_staff(this: usize, text: usize, players: u32) {
    if read_usize(G_STAFF_TABLE) == 0 || read_usize(this + SEARCH_ENG_RESULTS) == 0 || text == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, text, players); // the box
    }
    write_u32(this + SEARCH_ENG_COUNT, 0);
    let word = c_string_bytes(text);
    let state = |i: usize| read_u8(read_usize(G_STAFF_STATE_TABLE) + i * 0x4f + 0xb) & 0x30;
    if !word.is_empty() {
        let w = word.as_slice();
        if w == KEYWORD_NO_CLUB {
            return staff_pass(this, |p, _| staff_kind_fits(p, players) && read_usize(p + staff::CLUB_JOB) == 0);
        }
        if w == KEYWORD_RECURRING {
            return staff_pass(this, |p, _| read_usize(p + staff::PLAYER) != 0 && exe::injury_get_recurring_injury_level(read_usize(G_INJURY), p) != 0);
        }
        if w == KEYWORD_SCHOOL {
            return staff_pass(this, |p, _| staff_kind_fits(p, players) && read_i8(p + staff::AGE) < 18); // sx19b_11: school age
        }
        if w == KEYWORD_RETIRED {
            return staff_pass(this, |p, i| staff_kind_fits(p, players) && state(i) == 0x20);
        }
        if w == KEYWORD_RETIRING {
            return staff_pass(this, |p, i| staff_kind_fits(p, players) && state(i) == 0x10);
        }
        if w == KEYWORD_SEDGEFORD {
            return staff_pass(this, |p, _| {
                let k = read_u8(p + staff::CLASSIFICATION);
                let player = read_usize(p + staff::PLAYER);
                (k == 3 || (players == 0 && k == 2)) && player != 0 && read_i16(player + 7) > 0xa0 && read_i16(player + 5) > 0x8c
                    && read_usize(p + staff::CLUB_JOB) == 0
            });
        }
        if w == KEYWORD_AWOL {
            return staff_pass(this, |p, _| exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0);
        }
        if w == KEYWORD_SKIVE {
            return staff_pass(this, |p, _| exe::awol_has_missed_training(read_usize(G_AWOL), p) != 0);
        }
        if let Some(&(_, bits)) = CONTRACT_KEYWORDS.iter().find(|(k, _)| w == *k) {
            covered!("sx19b_11 contract keyword");
            return staff_contract_pass(this, bits);
        }
    }
    staff_pass(this, |p, _| staff_kind_fits(p, players) && staff_matches_text(p, text));
}

/// Comparer for `find_staff`: the original fills the array, its results are kept, the copy fills it again and the two
/// lists are compared; the original's list is put back.
pub(crate) unsafe extern "thiscall" fn compare_find_staff(this: usize, text: usize, players: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, text, players);
    let a = read_usize(this + SEARCH_ENG_RESULTS);
    if read_usize(G_STAFF_TABLE) == 0 || a == 0 || text == 0 {
        return;
    }
    let ro: Vec<usize> = (0..read_u32(this + SEARCH_ENG_COUNT) as usize).map(|k| read_usize(a + k * 4)).collect();
    find_staff(this, text, players);
    let rr: Vec<usize> = (0..read_u32(this + SEARCH_ENG_COUNT) as usize).map(|k| read_usize(a + k * 4)).collect();
    for (k, &p) in ro.iter().enumerate() {
        write_usize(a + k * 4, p);
    }
    write_u32(this + SEARCH_ENG_COUNT, ro.len() as u32);
    SCOREBOARD.record(ro == rr, || format!("SearchEngine::find_staff (find screen, {:?}, players {}): original {} people, rust {}",
        String::from_utf8_lossy(&c_string_bytes(text)), players, ro.len(), rr.len()));
}
