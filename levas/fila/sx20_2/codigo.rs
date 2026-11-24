// --- sx20_2: building and freeing the simulated statistics groups (simulated_stats.cpp, SimulatedStats::SimulatedStats
// 0x84b940 and SimulatedStats::~SimulatedStats 0x84be10) ---

/// simulated_stats.cpp, the source path of the error boxes.
pub(crate) const SIMULATED_STATS_SOURCE: usize = 0xa8278c;
/// The two bytes of the constructor's CD search that a no-CD exe patches (as `cm0102-1080`: the drive type's `jne`
/// made `nop`, the found test made `xor`).
const SIMULATED_STATS_CD_DRIVE_TYPE_JUMP: usize = 0x84b9a0;
const SIMULATED_STATS_CD_FOUND_TEST: usize = 0x84b9e3;

/// The CD search some constructors start with: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA` = 5) is
/// looked at for "<root>cm0102.exe" (`file_find_first`) until one has it, with the "Insert CD" box (OK) in between. A
/// no-CD exe patched in the calling routine is honoured by reading its two patched bytes: the drive type's `jne` at
/// `drive_type_jump` made `nop` (any drive) and the found test at `found_test` no longer a `test` (always found). The
/// same search as `club_history_find_cd`, for any routine's addresses.
pub(crate) unsafe fn wait_for_game_cd(drive_type_jump: usize, found_test: usize) {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(drive_type_jump) == 0x90;
    let always_found = read_u8(found_test) != 0x85;
    loop {
        // simplification: the drive list starts zeroed (as club_history_find_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                if any_drive || kind(root) == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        covered!("sx20_2 insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// Whether a club's division or reserve division is a competition the game plays (its object in
/// `G_COMP_OBJECT_TABLE`).
unsafe fn simulated_stats_is_played(division: Competition) -> bool {
    !division.is_null() && read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(division.addr()) as usize).wrapping_mul(4))) != 0
}

/// Writes into `list` the ids of the clubs that play simulated matches: neither their division nor their reserve
/// division is played and they are not a national or all-star team. Returns how many.
unsafe fn simulated_stats_collect_clubs(list: usize) -> i32 {
    let mut n = 0i32;
    for i in 0..read_i32(G_CLUB_COUNT) {
        let club = club_at(i as isize);
        if simulated_stats_is_played(club.division()) || simulated_stats_is_played(club.reserve_division()) {
            continue;
        }
        if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0 {
            continue;
        }
        write_i32(list + 4 * n as usize, read_i32(club.addr())); // the club id (+0)
        n += 1;
    }
    n
}

/// Sorts the club ids of `list` by reputation, highest first (a pairwise exchange sort, as the original).
unsafe fn simulated_stats_sort_by_reputation(list: usize, n: i32) {
    for i in 0..(n - 1).max(0) as usize {
        for j in i + 1..n as usize {
            let (a, b) = (read_i32(list + 4 * i), read_i32(list + 4 * j));
            if club_at(a as isize).reputation() < club_at(b as isize).reputation() { // sx20_2: highest reputation first
                write_i32(list + 4 * i, b);
                write_i32(list + 4 * j, a);
            }
        }
    }
}

/// Shuffles the sorted clubs a little: n / 2 times a random club swaps with one near it (`simulated_nearby_index`).
unsafe fn simulated_stats_shuffle_clubs(list: usize, n: i32) {
    let swaps = n / 2; // sx20_2: how many swaps
    for _ in 0..swaps {
        let a = exe::utils_random(n);
        let b = simulated_nearby_index(a, || n);
        let club = read_i32(list + 4 * b as usize);
        write_i32(list + 4 * b as usize, read_i32(list + 4 * a as usize));
        write_i32(list + 4 * a as usize, club);
    }
}

/// Deals the clubs of `list` in order into the groups: 16 each, except that, when the club count is not a multiple
/// of 16 (`rest` = count % 16), the first 16 − `rest` groups take 15. Running out of clubs shows the error box (line
/// 0x95) once and goes on reading past the list.
unsafe fn simulated_stats_fill_groups(this: usize, list: usize, n: i32, rest: i32) {
    let mut next = 0i32;
    for g in 0..read_i32(this) {
        let group = simulated_group(this, g as usize);
        write_i32(group, 0);
        for slot in 0..SIMULATED_GROUP_CLUBS as i32 {
            if next == n {
                show_error_at(SIMULATED_STATS_SOURCE, 0x95);
            }
            let count = read_i32(group);
            write_i32(group + 4 + 4 * count as usize, read_i32(list.wrapping_add((next as usize).wrapping_mul(4))));
            write_i32(group, count + 1);
            next += 1;
            if rest != 0 && g < 16 - rest && slot == 14 { // sx20_2: the short groups take 15
                break;
            }
        }
    }
}

/// Builds the simulated statistics: first waits for the game CD (unless the interactive mode flag
/// `G_INTERACTIVE_MODE_FLAG` is set: `wait_for_game_cd`). With a saved game `save`, loads the groups from it
/// (`SimulatedStats::load`). Otherwise takes the clubs outside the played competitions that are not national teams
/// (`simulated_stats_collect_clubs`; without memory for the list, the error box, line 0x49, the fatal cleanup and
/// `exit(-1)`), the group count = ⌈count / 16⌉ at +0, sorts them by reputation, shuffles them a little and deals them
/// into a new table of groups at +4 (`simulated_stats_fill_groups`). Returns `this`.
/// Defects of the original: with fewer than 16 × (16 − count % 16) clubs the short groups ask for more clubs than
/// there are (the box, then ids read past the list); the group table is not checked for null.
/// Original: `SimulatedStats::SimulatedStats` (0x84b940, simulated_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn simulated_stats_simulated_stats(this: usize, save: usize) -> usize {
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        wait_for_game_cd(SIMULATED_STATS_CD_DRIVE_TYPE_JUMP, SIMULATED_STATS_CD_FOUND_TEST);
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x84c510usize); // SimulatedStats::load
        load(this, save);
        return this;
    }
    let list = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(4));
    if list == 0 {
        fatal_error_at(SIMULATED_STATS_SOURCE, 0x49);
    }
    let n = simulated_stats_collect_clubs(list);
    let rest = n % 16;
    write_i32(this, n / 16 + (rest != 0) as i32);
    covered!("sx20_2 simulated stats groups");
    simulated_stats_sort_by_reputation(list, n);
    simulated_stats_shuffle_clubs(list, n);
    write_usize(this + 4, exe::crt_malloc((read_i32(this) as u32).wrapping_mul(SIMULATED_GROUP_SIZE as u32)));
    simulated_stats_fill_groups(this, list, n, rest);
    exe::crt_free(list);
    this
}

/// Frees the group table (+4) when there is one.
/// Original: `SimulatedStats::~SimulatedStats` (0x84be10, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn simulated_stats_destructor(this: usize) {
    if read_usize(this + 4) != 0 {
        exe::crt_free(read_usize(this + 4));
    }
}

/// Comparer for `simulated_stats_simulated_stats`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_simulated_stats(this: usize, save: usize) -> usize {
    // simplification: allocates and draws random numbers (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Comparer for `simulated_stats_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_destructor(this: usize) {
    // simplification: frees memory (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}
