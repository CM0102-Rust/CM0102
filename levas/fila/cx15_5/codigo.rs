// ---------- cx15_5: the MLS division of a club and the order of the play-off entries by points (usa_mls.cpp) ----------

/// The keys of the twelve MLS clubs of each division (club ids): 0x413 (East: New England, MetroStars, D.C. United,
/// Miami), 0x47b (Central: Columbus, Tampa Bay, Dallas, Chicago) and 0x414 (West: Colorado, Kansas City, Los Angeles,
/// San Jose), in the original's test order.
const MLS_DIVISION_KEYS: [(u16, [usize; 4]); 3] = [
    (0x413, [0x9d0390, 0x9d0394, 0x9d0384, 0x9d0380]),
    (0x47b, [0x9d03a4, 0x9d0388, 0x9d03a0, 0x9d037c]),
    (0x414, [0x9d03a8, 0x9d039c, 0x9d0398, 0x9d038c]),
];

/// The MLS division code of a club: 0 for a club whose league (`.division()`) is missing or is not the MLS
/// (`KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE`); for an MLS club, the code of the division whose keys hold its id
/// (`MLS_DIVISION_KEYS`); an MLS club outside the twelve: the "Error" box (usa_mls.cpp, line 0x71b) and 0. Only the
/// word is the result.
///
/// Called by the MLS table and stages (`create_table`, `create_division_stage`), the All-Star draw and two screens.
/// Original: `usa_mls_get_club_division` (0x909ec0, usa_mls.cpp, cdecl).
pub(crate) unsafe extern "C" fn usa_mls_get_club_division(club: usize) -> u16 {
    let league = Club(club).division().addr();
    if league == 0 || read_usize(league) != read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        return 0;
    }
    let id = read_usize(club);
    for (code, keys) in MLS_DIVISION_KEYS {
        if keys.iter().any(|&k| read_usize(k) == id) {
            covered!("cx15_5 mls division of a club");
            return code;
        }
    }
    show_error_at(0xaa1fb4, 0x71b); // "...\comp\leagues\usa_mls.cpp"
    0
}

/// qsort order of the six-byte play-off entries (club, then the points as a signed word at +4): the most points first
/// (b − a).
/// Original: `usa_mls_compare_points_desc` (0x90a130, usa_mls.cpp, cdecl).
pub(crate) unsafe extern "C" fn usa_mls_compare_points_desc(a: usize, b: usize) -> i32 {
    covered!("cx15_5 play-off entries by points");
    read_i16(b + 4) as i32 - read_i16(a + 4) as i32
}

/// Comparer for `usa_mls_get_club_division`: both run on the same club (an MLS club outside the twelve would show the
/// box twice), the words must be equal.
pub(crate) unsafe extern "C" fn compare_usa_mls_get_club_division(club: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(%%I0%%)(club);
    let n = usa_mls_get_club_division(club);
    SCOREBOARD.record(r == n, || format!("usa_mls_get_club_division({:#x}) original={:#x} rust={:#x}", club, r, n));
    r
}

/// Comparer for `usa_mls_compare_points_desc`: both on the same pair, the results must be equal.
pub(crate) unsafe extern "C" fn compare_usa_mls_compare_points_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = usa_mls_compare_points_desc(a, b);
    SCOREBOARD.record(r == n, || format!("usa_mls_compare_points_desc({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
