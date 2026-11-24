// --- sx17_8: the orders and the position picks of the scout's lists (player_search.cpp): by current rating (0x7bce80),
// by potential rating (0x7bceb0), the report entries' order (0x7be640) and the report's midfielders (0x7bfbd0),
// strikers (0x7bfc50) and defenders (0x7bfcd0) ---

/// The size of a scout report player entry (`scout_day_best_players`): +0 rating, +4 staff id, +8..+0x13 name ref,
/// +0x14 flags (`scout_day_player_flags`), +0x18 current ability.
const REPORT_ENTRY_SIZE: usize = 0x1a;
/// The flags of a report entry (+0x14): left out of the report, and the positions the picks look for.
const REPORT_ENTRY_FLAGS: usize = 0x14;
const REPORT_ENTRY_EXCLUDED: u32 = 0x40000;
const REPORT_ENTRY_STRIKER: u32 = 0x100;
const REPORT_ENTRY_DEFENDER: u32 = 0x200;
const REPORT_ENTRY_MIDFIELDER: u32 = 0x400;

/// A qsort order over search results by a dword field, highest first; a null entry after everything.
unsafe fn player_search_order_by_field_desc(a: usize, b: usize, field: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    read_i32(b + field).wrapping_sub(read_i32(a + field))
}

/// The order of search results by current rating (+0x21), highest first; a null entry after everything.
/// Original: `player_search_compare_results_by_current_rating` (0x7bce80, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_compare_results_by_current_rating(a: usize, b: usize) -> i32 {
    player_search_order_by_field_desc(a, b, 0x21) // sx17_8: the current rating
}

/// The order of search results by potential rating (+0x25), highest first; a null entry after everything.
/// Original: `player_search_compare_results_by_potential_rating` (0x7bceb0, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_compare_results_by_potential_rating(a: usize, b: usize) -> i32 {
    player_search_order_by_field_desc(a, b, 0x25)
}

/// The order of the scout report's player entries: by rating (+0), then by staff id (+4), highest first; a null entry
/// after everything.
/// Original: `player_search_compare_report_entries` (0x7be640, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_compare_report_entries(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    if read_i32(b) != read_i32(a) {
        return read_i32(b).wrapping_sub(read_i32(a));
    }
    read_i32(b + 4).wrapping_sub(read_i32(a + 4))
}

/// Runs an order side by side (it only reads): the answers must match.
unsafe fn player_search_compare_order(name: &str, a: usize, b: usize, original: unsafe extern "C" fn(usize, usize) -> i32, copy: unsafe extern "C" fn(usize, usize) -> i32) -> i32 {
    let r = original(a, b);
    let n = copy(a, b);
    SCOREBOARD.record(r == n, || format!("{}({:#x}, {:#x}) original={} rust={}", name, a, b, r, n));
    r
}

/// Comparer for `player_search_compare_results_by_current_rating`.
pub(crate) unsafe extern "C" fn compare_player_search_compare_results_by_current_rating(a: usize, b: usize) -> i32 {
    player_search_compare_order("player_search_compare_results_by_current_rating", a, b,
        original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%), player_search_compare_results_by_current_rating)
}

/// Comparer for `player_search_compare_results_by_potential_rating`.
pub(crate) unsafe extern "C" fn compare_player_search_compare_results_by_potential_rating(a: usize, b: usize) -> i32 {
    player_search_compare_order("player_search_compare_results_by_potential_rating", a, b,
        original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%), player_search_compare_results_by_potential_rating)
}

/// Comparer for `player_search_compare_report_entries`.
pub(crate) unsafe extern "C" fn compare_player_search_compare_report_entries(a: usize, b: usize) -> i32 {
    player_search_compare_order("player_search_compare_report_entries", a, b,
        original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%), player_search_compare_report_entries)
}

/// Copies the `count` report entries of `entries` that are flagged `position` and not excluded into `out`, in order,
/// and answers how many it copied (nothing without entries, an output or a positive count). An entry needs a staff id
/// from 0 to the staff count.
/// Defect of the original: the id equal to the staff count (one past the table) is accepted.
unsafe fn player_search_collect_report_position(count: u32, entries: usize, out: usize, position: u32) -> u8 {
    let count = count as u8 as i8;
    if count == 0 || entries == 0 || out == 0 {
        return 0;
    }
    let mut copied = 0u8;
    for k in 0..count.max(0) as usize {
        let entry = entries + k * REPORT_ENTRY_SIZE;
        let id = read_i32(entry + 4);
        if id < 0 || id > read_i32(G_STAFF_COUNT) { // sx17_8: the staff id range
            continue;
        }
        let flags = read_u32(entry + REPORT_ENTRY_FLAGS);
        if flags & REPORT_ENTRY_EXCLUDED != 0 || flags & position == 0 {
            continue;
        }
        core::ptr::copy_nonoverlapping(entry as *const u8, (out + copied as i8 as usize * REPORT_ENTRY_SIZE) as *mut u8, REPORT_ENTRY_SIZE);
        copied = copied.wrapping_add(1);
    }
    copied
}

/// The opposition report's midfielders: `player_search_collect_report_position` with the midfielder flag (0x400).
/// Original: `player_search_collect_report_midfielders` (0x7bfbd0, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_collect_report_midfielders(count: u32, entries: usize, out: usize) -> u8 {
    player_search_collect_report_position(count, entries, out, REPORT_ENTRY_MIDFIELDER)
}

/// The opposition report's strikers: `player_search_collect_report_position` with the striker flag (0x100).
/// Original: `player_search_collect_report_strikers` (0x7bfc50, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_collect_report_strikers(count: u32, entries: usize, out: usize) -> u8 {
    player_search_collect_report_position(count, entries, out, REPORT_ENTRY_STRIKER)
}

/// The opposition report's defenders: `player_search_collect_report_position` with the defender flag (0x200).
/// Original: `player_search_collect_report_defenders` (0x7bfcd0, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_collect_report_defenders(count: u32, entries: usize, out: usize) -> u8 {
    player_search_collect_report_position(count, entries, out, REPORT_ENTRY_DEFENDER)
}

/// Runs a position pick side by side over the output (as many entries as the input) and the answer.
unsafe fn player_search_compare_pick(name: &str, count: u32, entries: usize, out: usize,
                                     original: unsafe extern "C" fn(u32, usize, usize) -> u8, copy: unsafe extern "C" fn(u32, usize, usize) -> u8) -> u8 {
    let n = (count as u8 as i8).max(0) as usize;
    if out == 0 || n == 0 {
        return original(count, entries, out);
    }
    let (r, c, same, mo, mr) = crate::comparison::side_by_side(&[(out, n * REPORT_ENTRY_SIZE)], || original(count, entries, out), || copy(count, entries, out));
    SCOREBOARD.record(same && r == c, || format!("{}({}) original={} rust={} {:?}", name, n, r, c, first_differences(&mo, &mr)));
    r
}

/// Comparer for `player_search_collect_report_midfielders`.
pub(crate) unsafe extern "C" fn compare_player_search_collect_report_midfielders(count: u32, entries: usize, out: usize) -> u8 {
    player_search_compare_pick("player_search_collect_report_midfielders", count, entries, out,
        original_fn::<unsafe extern "C" fn(u32, usize, usize) -> u8>(%%I3%%), player_search_collect_report_midfielders)
}

/// Comparer for `player_search_collect_report_strikers`.
pub(crate) unsafe extern "C" fn compare_player_search_collect_report_strikers(count: u32, entries: usize, out: usize) -> u8 {
    player_search_compare_pick("player_search_collect_report_strikers", count, entries, out,
        original_fn::<unsafe extern "C" fn(u32, usize, usize) -> u8>(%%I4%%), player_search_collect_report_strikers)
}

/// Comparer for `player_search_collect_report_defenders`.
pub(crate) unsafe extern "C" fn compare_player_search_collect_report_defenders(count: u32, entries: usize, out: usize) -> u8 {
    player_search_compare_pick("player_search_collect_report_defenders", count, entries, out,
        original_fn::<unsafe extern "C" fn(u32, usize, usize) -> u8>(%%I5%%), player_search_collect_report_defenders)
}
