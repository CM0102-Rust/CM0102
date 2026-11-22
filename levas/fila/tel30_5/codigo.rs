// ---------- batch %%LEVA%%: the squad list comparators (1) (club_screens.cpp) ----------

/// The sort context of the squad list comparators, written by `build_filtered_sorted_squad_list` (0x4841d0) before
/// its qsort: the club whose list it is.
pub(crate) const PLAYER_SORT_CLUB: usize = 0xad9db0;
/// u8: the attribute of the "attribute" column.
pub(crate) const PLAYER_SORT_ATTRIBUTE: usize = 0xad9db4;
/// u8: the field of the "statistic" column (`PlayerStats::get_stat`).
pub(crate) const PLAYER_SORT_STAT_FIELD: usize = 0xad9db5;
/// u8: the name style passed to `database_format_staff_name`.
pub(crate) const PLAYER_SORT_NAME_STYLE: usize = 0xadab33;
/// u32: the team index passed to `Tactics::find_selection_slot` (1 = reserves).
pub(crate) const PLAYER_SORT_RESERVES: usize = 0xadab34;
/// usize: the team sheet of a club the human does not control (0 = none).
pub(crate) const PLAYER_SORT_TEAM_SHEET: usize = 0xadab38;
/// usize: the club or its linked side, whose contracts and squad numbers the columns show.
pub(crate) const PLAYER_SORT_SIDE: usize = 0xadab3c;
/// usize: the fixture of the status column.
pub(crate) const PLAYER_SORT_FIXTURE: usize = 0xadab44;
/// u8: the category of the "statistic" column (`PlayerStats::get_stat`).
pub(crate) const PLAYER_SORT_STAT_CATEGORY: usize = 0xadab4a;
/// usize: the viewer whose knowledge hides attributes (`staff_is_player_attribute_visible`).
pub(crate) const PLAYER_SORT_VIEWER: usize = 0xadadf0;
/// `G_SQUAD_MANAGER` (0xdd7ef0), the squad number owner.
const PLAYER_SORT_SQUAD_MANAGER: usize = 0xdd7ef0;

/// The tie-break of every squad list comparator: a null first entry sorts last (1), a null second one first (-1);
/// otherwise the two names, written in the list's name style (`PLAYER_SORT_NAME_STYLE`) into 200-byte buffers by
/// `database_format_staff_name`, compared with `utils_string_compare(first, second, 1, 0)`.
pub(crate) unsafe fn player_sort_by_name(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let style = read_u8(PLAYER_SORT_NAME_STYLE) as u32;
    let (mut x, mut y) = ([0u8; 200], [0u8; 200]);
    exe::database_format_staff_name(a, x.as_mut_ptr() as usize, 200, style);
    exe::database_format_staff_name(b, y.as_mut_ptr() as usize, 200, style);
    exe::utils_string_compare(x.as_ptr(), y.as_ptr(), 1, 0)
}

/// The two entries of a qsort call (pointers to staff pointers).
unsafe fn player_sort_pair(a: *const usize, b: *const usize) -> (usize, usize) {
    (*a, *b)
}

/// Squad list order "selection": the players by their slot in the team selection, the unselected last.
///
/// A null first entry sorts last (1), a null second first (-1). When the human controls the list's club
/// (`human_manager_current_controls_club(club, 0, 1)`), each player's slot is `Tactics::find_selection_slot(club,
/// player, PLAYER_SORT_RESERVES)`; otherwise, with no team sheet (`PLAYER_SORT_TEAM_SHEET` 0) the result is 1, and
/// with one the slot is the last of its 20 entries (0x49 bytes apart, from +0x190 of the sheet's first or second half:
/// the second, 0x625 bytes on, when the club's id matches the sheet's first dword) holding the player's id. An
/// unselected first player (0xff) sorts last (1), then an unselected second one first (-1); different slots give
/// the signed byte difference first - second; equal slots go to the names (`player_sort_by_name`).
/// Original: `compare_players_by_selection_slot_then_name` (0x485470, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_selection_slot_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let club = read_usize(PLAYER_SORT_CLUB);
    let (mut sa, mut sb) = (0xffu8, 0xffu8);
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        covered!("tel30_5 selection slot of the human's club");
        let reserves = read_u32(PLAYER_SORT_RESERVES);
        sa = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, a, reserves);
        sb = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, b, reserves);
    } else {
        let sheet = read_usize(PLAYER_SORT_TEAM_SHEET);
        if sheet == 0 {
            return 1;
        }
        let (ida, idb) = (read_u32(a), read_u32(b));
        let half = (read_u32(club) == read_u32(sheet)) as usize;
        let mut entry = sheet + half * 0x625 + 0x190;
        for k in 0u8..0x14 {
            let id = read_u32(entry);
            if id == ida {
                sa = k;
            }
            if id == idb {
                sb = k;
            }
            entry += 0x49;
        }
    }
    if sa == 0xff {
        return 1;
    }
    if sb == 0xff {
        return -1;
    }
    let d = sa.wrapping_sub(sb) as i8;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "squad slot": the players by the human manager's squad number index
/// (`HUMAN_MANAGER::get_player_squad_number(player, 0)`), the unnumbered (0xff) last, no name tie-break.
///
/// A null first entry sorts last (1), a null second first (-1). An unnumbered first player gives 1 when the second
/// has a number and 0 when neither has; an unnumbered second one gives -1; otherwise first - second (signed bytes).
/// Original: `compare_players_by_human_squad_slot` (0x485630, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_human_squad_slot(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let number: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x601880usize);
    let na = number(read_usize(G_HUMAN_MANAGER), a, 0);
    let nb = number(read_usize(G_HUMAN_MANAGER), b, 0);
    if na == 0xff {
        return (nb != 0xff) as i32;
    }
    if nb == 0xff {
        return -1;
    }
    na as i8 as i32 - nb as i8 as i32
}

/// Squad list order "status": the players by their status code (`get_player_status_code(player, club, fixture)`,
/// 0x4849f0), then by name.
///
/// A null first entry sorts last (1), a null second first (-1); then the signed byte difference of the codes
/// first - second; equal codes go to the names.
/// Original: `compare_players_by_status_code_then_name` (0x4856a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_status_code_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let status: unsafe extern "C" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4849f0usize);
    let (club, fixture) = (read_usize(PLAYER_SORT_CLUB), read_usize(PLAYER_SORT_FIXTURE));
    let sb = status(b, club, fixture);
    let sa = status(a, club, fixture);
    let d = sa.wrapping_sub(sb) as i8;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "name": the players by name (`player_sort_by_name`).
/// Original: `compare_players_by_name` (0x4857a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "position": the players by their positions text, then by name.
///
/// A null first entry (or one with no player record at `staff::PLAYER`) sorts last (1), a null second one first
/// (-1); the positions are written by `database_format_player_positions(player, buffer, 200, 4)` and compared with
/// `utils_string_compare(first, second, 1, 0)`; equal texts go to the names.
/// Original: `compare_players_by_positions_then_name` (0x485830, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_positions_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ra, rb) = (read_usize(a + staff::PLAYER), read_usize(b + staff::PLAYER));
    if ra == 0 {
        return 1;
    }
    if rb == 0 {
        return -1;
    }
    let positions: unsafe extern "C" fn(usize, *mut u8, u32, u32) -> u32 = core::mem::transmute(0x53ee50usize);
    let (mut x, mut y) = ([0u8; 200], [0u8; 200]);
    positions(ra, x.as_mut_ptr(), 200, 4);
    positions(rb, y.as_mut_ptr(), 200, 4);
    let r = exe::utils_string_compare(x.as_ptr(), y.as_ptr(), 1, 0);
    if r != 0 {
        return r;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "club": the players by their club's name (the text at +0x38 of the `staff::CLUB_JOB` club),
/// then by name.
///
/// A null first entry (or one without a club) sorts last (1), a null second one first (-1); the club names are
/// compared with `utils_string_compare(first, second, 1, 0)`; equal names go to the players' names.
/// Original: `compare_players_by_club_name_then_name` (0x485960, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_club_name_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ca, cb) = (read_usize(a + staff::CLUB_JOB), read_usize(b + staff::CLUB_JOB));
    if ca == 0 {
        return 1;
    }
    if cb == 0 {
        return -1;
    }
    let r = exe::utils_string_compare((ca + 0x38) as *const u8, (cb + 0x38) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "squad status": the players by the squad status of their contract with the list's side, then
/// by name.
///
/// A null first entry sorts last (1), a null second one first (-1). Each player's contract with
/// `PLAYER_SORT_SIDE` comes from `CONTRACT_MANAGER::get_contract_for_club`; when the human controls the side
/// (`human_manager_current_controls_club(side, 0, 1)`) the status is the high nibble of the contract's byte +0x4f
/// (100 without a contract), otherwise both are 100. The signed difference first - second decides; equal statuses
/// go to the names.
/// Original: `compare_players_by_squad_status_then_name` (0x485a60, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_squad_status_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let side = read_usize(PLAYER_SORT_SIDE);
    let ca = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), a, side);
    let cb = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), b, side);
    let (mut sa, mut sb) = (100u8, 100u8);
    if exe::human_manager_current_controls_club(side, 0, 1) != 0 {
        if ca != 0 {
            sa = read_u8(ca + 0x4f) >> 4;
        }
        if cb != 0 {
            sb = read_u8(cb + 0x4f) >> 4;
        }
    }
    let d = sa as i8 as i32 - sb as i8 as i32;
    if d != 0 {
        return d;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "squad number": the players by the absolute value of their squad number at the list's side,
/// then by name.
///
/// A null first entry (or one with no player record) sorts last (1), a null second one first (-1); then a first
/// player with squad number 0 (`SQUAD_MANAGER::get_squad_number(player, PLAYER_SORT_SIDE)`) sorts last (1) and a
/// second one first (-1); otherwise the byte difference |first| - |second| (signed) decides, and equal numbers go
/// to the names.
/// Original: `compare_players_by_squad_number_then_name` (0x485ba0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_squad_number_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    if read_usize(a + staff::PLAYER) == 0 {
        return 1;
    }
    if read_usize(b + staff::PLAYER) == 0 {
        return -1;
    }
    let number = |p: usize| exe::squad_manager_get_squad_number(read_usize(PLAYER_SORT_SQUAD_MANAGER), p, read_usize(PLAYER_SORT_SIDE));
    if number(a) == 0 {
        return 1;
    }
    if number(b) == 0 {
        return -1;
    }
    let nb = (number(b) as i32).abs();
    let na = (number(a) as i32).abs();
    let d = (na as u8).wrapping_sub(nb as u8) as i8;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "release clause": the players by the code of their release clause, then by its abbreviation,
/// then by name.
///
/// A null first entry sorts last (1), a null second one first (-1). Each player's buffer starts as "  -" (the four
/// bytes at 0x98e8c8) and 26 zero bytes, and the code as -1; a contract (`CONTRACT_MANAGER::get_contract`) replaces
/// both with `STAFF_CONTRACT::get_release_clause_abbreviation(buffer, 30)`. The signed byte difference
/// second - first decides; then the two texts byte by byte (1 when the first is greater, -1 when smaller, the low
/// byte of the result); equal texts go to the names.
/// Original: `compare_players_by_release_clause_then_name` (0x485d20, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_release_clause_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    let (mut ta, mut tb) = ([0u8; 30], [0u8; 30]);
    let dash = read_u32(0x98e8c8).to_le_bytes();
    ta[..4].copy_from_slice(&dash);
    tb[..4].copy_from_slice(&dash);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let ca = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), a);
    let cb = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), b);
    let clause: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> u8 = core::mem::transmute(0x868020usize);
    let (mut ka, mut kb) = (0xffu8, 0xffu8);
    if ca != 0 {
        ka = clause(ca, ta.as_mut_ptr(), 0x1e);
    }
    if cb != 0 {
        kb = clause(cb, tb.as_mut_ptr(), 0x1e);
    }
    let d = kb.wrapping_sub(ka) as i8;
    if d != 0 {
        return d as i32;
    }
    let mut k = 0;
    let order = loop {
        let (x, y) = (ta[k], tb[k]);
        if x != y {
            break if x < y { -1i8 } else { 1 };
        }
        if x == 0 || k + 1 >= ta.len() {
            break 0;
        }
        k += 1;
    };
    if order != 0 {
        return order as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// The rank of a European registration byte (`staff::SQUAD_SELECTED_FOR`): 1 → 4, 2 → 3, 4 → 2, 0 → 1, others 0.
fn euro_registration_rank(v: u8) -> u8 {
    match v {
        1 => 4,
        2 => 3,
        4 => 2,
        0 => 1,
        _ => 0,
    }
}

/// Squad list order "European registration": the players by the rank of their registration byte
/// (`staff::SQUAD_SELECTED_FOR`: 1, 2, 4, 0, then the others), then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then the signed byte difference of the ranks
/// second - first (1 → 4, 2 → 3, 4 → 2, 0 → 1, others 0); equal ranks go to the names.
/// Original: `compare_players_by_euro_registration_then_name` (0x485ed0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_euro_registration_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let rb = euro_registration_rank(read_u8(b + staff::SQUAD_SELECTED_FOR));
    let ra = euro_registration_rank(read_u8(a + staff::SQUAD_SELECTED_FOR));
    let d = rb.wrapping_sub(ra) as i8;
    if d != 0 {
        return d as i32;
    }
    player_sort_by_name(a, b)
}

/// Squad list order "form": the players by their recent form (`PlayerStats::get_recent_form`), best first, then by
/// name.
///
/// A null first entry, or one without a player record or with an id not below `G_STAFF_COUNT`, sorts last (1); the
/// same for the second gives -1. Then the signed difference second - first of the forms; equal forms go to the names.
/// Original: `compare_players_by_form_then_name` (0x485fd0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_form_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ra, rb) = (read_usize(a + staff::PLAYER), read_usize(b + staff::PLAYER));
    let count = read_i32(G_STAFF_COUNT);
    if ra == 0 || read_i32(a) >= count {
        return 1;
    }
    if rb == 0 || read_i32(b) >= count {
        return -1;
    }
    let fb = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), rb) as i32;
    let fa = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), ra) as i32;
    if fb - fa != 0 {
        return fb - fa;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// The original of one of this item's comparators side by side with the copy, by return value.
unsafe fn player_sort_compare(name: &str, original: unsafe extern "C" fn(*const usize, *const usize) -> i32,
    copy: unsafe extern "C" fn(*const usize, *const usize) -> i32, a: *const usize, b: *const usize) -> i32 {
    let r = original(a, b);
    let v = copy(a, b);
    SCOREBOARD.record(r == v, || format!("squad list sort {} ({:#x}, {:#x}): original={} rust={}", name, *a, *b, r, v));
    r
}

/// Comparer for `compare_players_by_selection_slot_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_selection_slot_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("selection", original_fn(%%I0%%), compare_players_by_selection_slot_then_name, a, b)
}

/// Comparer for `compare_players_by_human_squad_slot`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_human_squad_slot(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("human squad slot", original_fn(%%I1%%), compare_players_by_human_squad_slot, a, b)
}

/// Comparer for `compare_players_by_status_code_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_status_code_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("status", original_fn(%%I2%%), compare_players_by_status_code_then_name, a, b)
}

/// Comparer for `compare_players_by_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("name", original_fn(%%I3%%), compare_players_by_name, a, b)
}

/// Comparer for `compare_players_by_positions_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_positions_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("positions", original_fn(%%I4%%), compare_players_by_positions_then_name, a, b)
}

/// Comparer for `compare_players_by_club_name_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_club_name_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("club", original_fn(%%I5%%), compare_players_by_club_name_then_name, a, b)
}

/// Comparer for `compare_players_by_squad_status_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_squad_status_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("squad status", original_fn(%%I6%%), compare_players_by_squad_status_then_name, a, b)
}

/// Comparer for `compare_players_by_squad_number_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_squad_number_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("squad number", original_fn(%%I7%%), compare_players_by_squad_number_then_name, a, b)
}

/// Comparer for `compare_players_by_release_clause_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_release_clause_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("release clause", original_fn(%%I8%%), compare_players_by_release_clause_then_name, a, b)
}

/// Comparer for `compare_players_by_euro_registration_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_euro_registration_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("European registration", original_fn(%%I9%%), compare_players_by_euro_registration_then_name, a, b)
}

/// Comparer for `compare_players_by_form_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_form_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("form", original_fn(%%I10%%), compare_players_by_form_then_name, a, b)
}
