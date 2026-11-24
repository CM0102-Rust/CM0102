// --- sx17_6: the player search's own files (player_search.cpp): load_from_file (0x7b2bf0) and save_to_file (0x7b3660) ---

/// The size of a search file's header (the search from +5: magic, version, list type, names, password, seed...).
const SEARCH_FILE_HEADER_SIZE: usize = 0x16b;
/// The marker a Mac search file ends with (`utils_read_file_format_marker`); a Windows one ends with 0xf2fffff2.
const SEARCH_FILE_MAC_MARKER: u32 = 0xf1fffff1;
const SEARCH_FILE_WINDOWS_MARKER: u32 = 0xf2fffff2;
/// The longest nation name a search file stores (the nation's short name, `nation::SHORT_NAME`).
const SEARCH_FILE_NATION_NAME_MAX: u32 = 0x1a;
/// `utils_append_u32_to_file` (0x90c1a0, cdecl): seeks to the end of a file and writes four bytes there.
const FN_UTILS_APPEND_U32_TO_FILE: usize = 0x90c1a0;
/// "wb" (the mode of the files the game writes).
const WRITE_BINARY_MODE: usize = 0x9a7f28;
/// The SearchFilters record of the search and its two nation filters (nationality +0x18, based in +0x1e).
const SEARCH_FILE_FILTERS: usize = 0x1fa;
const SEARCH_FILTER_NATIONALITY: usize = 0x18;
const SEARCH_FILTER_BASED_IN: usize = 0x1e;

/// A file search answer that failed: unless `quiet`, the player_search.cpp "Error" box at `line`; the file (if open) is
/// closed and the answer is 0.
unsafe fn player_search_file_failed(file: usize, quiet: u32, line: u32) -> u32 {
    if quiet == 0 {
        show_error_box(0xa701b0, line); // player_search.cpp
    }
    if file != 0 {
        exe::crt_fclose(file);
    }
    0
}

/// After a search file was read or written: the title (+0xb) becomes the file's name (`_splitpath`).
unsafe fn player_search_title_from_path(this: usize, path: usize) {
    exe::crt_strtok(path, 0usize, 0usize, (this + 0xb) as *mut u8, core::ptr::null_mut());
}

/// The search's flags after a file: the report mark and +0x2ac set, +0x2a8 = has a password (+0x139).
unsafe fn player_search_file_flags(this: usize) {
    write_u32(this + SCOUT_DAY_REPORT_MARK, 1);
    write_u32(this + 0x2a8, (read_u8(this + 0x139) != 0) as u32);
    write_u32(this + 0x2ac, 1);
}

/// Writes one of the search's nation filters as a name: the length (with the NUL) and the nation's short name, or a
/// length 0 for no nation (−1). Answers the "Error" line of the write that failed: `lines` = (the length of a named
/// nation, its name, the length 0).
unsafe fn player_search_write_nation_filter(file: usize, nation: i32, lines: (u32, u32, u32)) -> Result<(), u32> {
    let (len_line, name_line, none_line) = lines;
    let mut len = 0u32;
    if nation == -1 {
        return if exe::crt_fwrite(&mut len as *mut u32 as usize, 4, 1, file) == 1 { Ok(()) } else { Err(none_line) };
    }
    let name = read_usize(G_NATION_TABLE).wrapping_add(nation as usize * nation::SIZE) + nation::SHORT_NAME;
    len = c_string_len(name) as u32 + 1;
    if exe::crt_fwrite(&mut len as *mut u32 as usize, 4, 1, file) != 1 {
        return Err(len_line);
    }
    if exe::crt_fwrite(name, len, 1, file) != 1 {
        return Err(name_line);
    }
    Ok(())
}

/// Saves the search to a file ("wb"): the header (+5, 0x16b bytes); for lists other than quick and staff searches,
/// the result capacity, count and results; the SearchFilters record; its nationality and based-in nations as names
/// (`player_search_write_nation_filter`); the filter record; the assigned club and, with one (> −1), the scouted club
/// record (0x51a8 bytes, unchecked); and the Windows end marker (`utils_append_u32_to_file`). The file is closed, the
/// title becomes the file's name and the flags are set (`player_search_file_flags`); answers 1. No path, a file that
/// does not open or a short write: unless `quiet`, the player_search.cpp "Error" box (lines 0xad2, 0xaf3, 0xaf6, 0xafc,
/// 0xaff, 0xb06, 0xb0b, 0xb13/0xb14/0xb19, 0xb20/0xb21/0xb26, 0xb2a, 0xb2d); the file closed and 0.
/// Original: `PLAYER_SEARCH::save_to_file` (0x7b3660, player_search.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn player_search_save_to_file(this: usize, path: usize, quiet: u32) -> u32 {
    if path == 0 {
        return player_search_file_failed(0, quiet, 0xad2);
    }
    let file = exe::crt_fopen(path, WRITE_BINARY_MODE);
    if file == 0 {
        return player_search_file_failed(0, quiet, 0xaf3);
    }
    let write = |offset: usize, size: u32, count: u32| exe::crt_fwrite(this + offset, size, count, file) == count;
    if !write(5, SEARCH_FILE_HEADER_SIZE as u32, 1) {
        return player_search_file_failed(file, quiet, 0xaf6);
    }
    if !matches!(read_u8(this + player_search_fields::LIST_TYPE), 2 | 3) {
        if !write(SCOUT_DAY_RESULT_CAPACITY, 4, 1) {
            return player_search_file_failed(file, quiet, 0xafc);
        }
        if !write(player_search_fields::RESULT_COUNT, 4, 1) {
            return player_search_file_failed(file, quiet, 0xaff);
        }
        let count = read_i32(this + player_search_fields::RESULT_COUNT);
        if count > 0 && exe::crt_fwrite(read_usize(this + player_search_fields::RESULTS), SCOUT_DAY_RESULT_SIZE as u32, count as u32, file) != count as u32 {
            return player_search_file_failed(file, quiet, 0xb06);
        }
    }
    if !write(SEARCH_FILE_FILTERS, 0x99, 1) {
        return player_search_file_failed(file, quiet, 0xb0b);
    }
    let filters = this + SEARCH_FILE_FILTERS;
    let nations = player_search_write_nation_filter(file, read_i32(filters + SEARCH_FILTER_NATIONALITY), (0xb13, 0xb14, 0xb19))
        .and_then(|()| player_search_write_nation_filter(file, read_i32(filters + SEARCH_FILTER_BASED_IN), (0xb20, 0xb21, 0xb26)));
    if let Err(line) = nations {
        return player_search_file_failed(file, quiet, line);
    }
    if !write(0x293, 0x11, 1) {
        return player_search_file_failed(file, quiet, 0xb2a);
    }
    if !write(player_search_fields::ASSIGNED_CLUB, 4, 1) {
        return player_search_file_failed(file, quiet, 0xb2d);
    }
    if read_i32(this + player_search_fields::ASSIGNED_CLUB) > -1 {
        covered!("sx17_6 save the club record");
        exe::crt_fwrite(read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD), 0x51a8, 1, file);
    }
    let append: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_UTILS_APPEND_U32_TO_FILE);
    append(file, SEARCH_FILE_WINDOWS_MARKER);
    exe::crt_fclose(file);
    player_search_title_from_path(this, path);
    player_search_file_flags(this);
    1
}

/// Comparer for `player_search_save_to_file`: it writes a file, so only the original runs, and the plan is checked when
/// it answers 1: the file starts with the search's header (+5, 0x16b bytes) and the flags are set.
pub(crate) unsafe extern "thiscall" fn compare_player_search_save_to_file(this: usize, path: usize, quiet: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, path, quiet);
    if r == 0 {
        return r;
    }
    let mut header = [0u8; SEARCH_FILE_HEADER_SIZE];
    let file = exe::crt_fopen(path, BADGE_OPEN_MODE);
    let read = file != 0 && exe::crt_fread(header.as_mut_ptr() as usize, SEARCH_FILE_HEADER_SIZE as u32, 1, file) == 1;
    if file != 0 {
        exe::crt_fclose(file);
    }
    let ok = read && header[..] == *core::slice::from_raw_parts((this + 5) as *const u8, SEARCH_FILE_HEADER_SIZE)
        && read_u32(this + SCOUT_DAY_REPORT_MARK) == 1 && read_u32(this + 0x2ac) == 1 && read_u32(this + 0x2a8) == (read_u8(this + 0x139) != 0) as u32;
    SCOREBOARD.record(ok, || format!("player_search_save_to_file({:#x}) header written {} flags {:?}", this, read,
                                     (read_u32(this + SCOUT_DAY_REPORT_MARK), read_u32(this + 0x2a8), read_u32(this + 0x2ac))));
    r
}

/// Reads one of the search's nation filters back: the length and, when it is 1..0x1a, the name, looked up among the
/// nations' short names (−1 when none matches); a length outside that range gives −1 without reading a name. Answers
/// the nation, or the failed line (`len_line` / `name_line`).
unsafe fn player_search_read_nation_filter(file: usize, len_line: u32, name_line: u32) -> Result<i32, u32> {
    let mut len = 0u32;
    if exe::crt_fread(&mut len as *mut u32 as usize, 4, 1, file) != 1 {
        return Err(len_line);
    }
    if len == 0 || len > SEARCH_FILE_NATION_NAME_MAX {
        return Ok(-1);
    }
    let mut name = [0u8; SEARCH_FILE_NATION_NAME_MAX as usize + 2];
    if exe::crt_fread(name.as_mut_ptr() as usize, len, 1, file) != 1 {
        return Err(name_line);
    }
    let table = read_usize(G_NATION_TABLE);
    Ok((0..read_i32(G_NATION_COUNT))
        .find(|&k| guio_same_c_string(name.as_ptr() as usize, table + k as usize * nation::SIZE + nation::SHORT_NAME))
        .unwrap_or(-1))
}

/// Loads a search from a file ("rb"): a Mac file (`SEARCH_FILE_MAC_MARKER`) shows, unless `quiet`, its own "Error" box
/// and answers 0. The header must have the magic 0x24a9e, version 1 and this search's list type; it replaces +5..+0x16f
/// and the stored game seed (+0x16c) is compared with this game's (`G_GAME_RANDOM_SEED`, then stored). For lists other
/// than quick and staff searches: the result capacity and count, the old results freed and the new ones read (a failed
/// allocation shows the box, line 0xa25, and exits). The SearchFilters record, its nationality and based-in nations by
/// name (`player_search_read_nation_filter`, `SearchFilters::set_nationality_nation` / `set_based_nation`), the filter
/// record and the assigned club: below −1 none; a club another search of the session watches
/// (`is_club_in_session_searches`) none; a club from another game (another seed) none. With a club a new scouted club
/// record is read (listed when it reads whole); without one the old record is deleted. The file is closed, the title
/// becomes its name, the region and competition are cleared and, for lists other than quick and staff searches, the
/// invalid results dropped (relinked when the file is from another game); the flags are set; answers 1. No path, a
/// file that does not open, a bad header or a short read: unless `quiet` the player_search.cpp "Error" box (lines
/// 0x9e2, 0x9e6, 0x9f8, 0x9fc, 0xa00, 0xa04, 0xa11, 0xa14, 0xa2c, 0xa32, 0xa35, 0xa3b, 0xa45, 0xa4b, 0xa57, 0xa5d); the
/// file closed and 0.
/// Original: `PLAYER_SEARCH::load_from_file` (0x7b2bf0, player_search.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn player_search_load_from_file(this: usize, path: usize, quiet: u32) -> u32 {
    if path == 0 {
        return player_search_file_failed(0, quiet, 0x9e2);
    }
    let file = exe::crt_fopen(path, BADGE_OPEN_MODE);
    if file == 0 {
        return player_search_file_failed(0, quiet, 0x9e6);
    }
    if exe::utils_read_file_format_marker(file) == SEARCH_FILE_MAC_MARKER {
        if quiet == 0 {
            exe::gui_show_message_box(0x9870e0usize, 0xa70260usize, 0); // "Error", "Can't load a Mac format search file..."
        }
        exe::crt_fclose(file);
        return 0;
    }
    let mut header = [0u8; SEARCH_FILE_HEADER_SIZE];
    let h = header.as_mut_ptr() as usize;
    if exe::crt_fread(h, SEARCH_FILE_HEADER_SIZE as u32, 1, file) != 1 {
        return player_search_file_failed(file, quiet, 0x9f8);
    }
    if read_u32(h) != 0x24a9e {
        return player_search_file_failed(file, quiet, 0x9fc);
    }
    if read_u8(h + 4) != 1 {
        return player_search_file_failed(file, quiet, 0xa00);
    }
    if read_u8(h + 5) != read_u8(this + player_search_fields::LIST_TYPE) {
        return player_search_file_failed(file, quiet, 0xa04);
    }
    core::ptr::copy_nonoverlapping(h as *const u8, (this + 5) as *mut u8, SEARCH_FILE_HEADER_SIZE);
    let same_game = read_u32(this + 0x16c) == read_u32(G_GAME_RANDOM_SEED);
    write_u32(this + 0x16c, read_u32(G_GAME_RANDOM_SEED));
    let read = |offset: usize, size: u32, count: u32| exe::crt_fread(this + offset, size, count, file) == count;
    let keeps_results = !matches!(read_u8(this + player_search_fields::LIST_TYPE), 2 | 3);
    if keeps_results {
        if !read(SCOUT_DAY_RESULT_CAPACITY, 4, 1) {
            return player_search_file_failed(file, quiet, 0xa11);
        }
        if !read(player_search_fields::RESULT_COUNT, 4, 1) {
            return player_search_file_failed(file, quiet, 0xa14);
        }
        let results = read_usize(this + player_search_fields::RESULTS);
        if results != 0 {
            exe::crt_free(results);
            write_usize(this + player_search_fields::RESULTS, 0);
        }
        if read_i32(this + SCOUT_DAY_RESULT_CAPACITY) > 0 {
            let array = exe::crt_malloc(read_u32(this + SCOUT_DAY_RESULT_CAPACITY).wrapping_mul(SCOUT_DAY_RESULT_SIZE as u32));
            write_usize(this + player_search_fields::RESULTS, array);
            if array == 0 {
                show_error_box(0xa701b0, 0xa25); // player_search.cpp
                exe::game_fatal_cleanup();
                exe::shared_noop_ret0_5a8f60();
                exe::crt_exit(-1);
            }
            let count = read_i32(this + player_search_fields::RESULT_COUNT);
            if count > 0 && exe::crt_fread(array, SCOUT_DAY_RESULT_SIZE as u32, count as u32, file) != count as u32 {
                return player_search_file_failed(file, quiet, 0xa2c);
            }
        }
    }
    if !read(SEARCH_FILE_FILTERS, 0x99, 1) {
        return player_search_file_failed(file, quiet, 0xa32);
    }
    let nation = match player_search_read_nation_filter(file, 0xa35, 0xa3b) {
        Ok(n) => n,
        Err(line) => return player_search_file_failed(file, quiet, line),
    };
    search_filters_call1(SEARCH_FN_FILTERS_SET_NATIONALITY_NATION, this + SEARCH_FILE_FILTERS, nation as u32);
    let based = match player_search_read_nation_filter(file, 0xa45, 0xa4b) {
        Ok(n) => n,
        Err(line) => return player_search_file_failed(file, quiet, line),
    };
    search_filters_call1(SEARCH_FN_FILTERS_SET_BASED_NATION, this + SEARCH_FILE_FILTERS, based as u32);
    if !read(0x293, 0x11, 1) {
        return player_search_file_failed(file, quiet, 0xa57);
    }
    let mut club = 0i32;
    if exe::crt_fread(&mut club as *mut i32 as usize, 4, 1, file) != 1 {
        return player_search_file_failed(file, quiet, 0xa5d);
    }
    let watched = club > -1 && exe::scout_manager_is_club_in_session_searches(read_usize(G_SCOUT_MANAGER), club_at(club as isize)) != 0;
    let assigned = if club < -1 || watched || !same_game { -1 } else { club }; // sx17_6: a club only from the same game
    write_i32(this + player_search_fields::ASSIGNED_CLUB, assigned);
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    if record != 0 {
        exe::llist_remove(G_SCOUT_CLUB_TABLE_LIST, record);
        exe::crt_operator_delete(record);
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, 0);
    }
    if assigned > -1 {
        covered!("sx17_6 load the club record");
        let record = exe::crt_operator_new(0x51a8);
        if record != 0 && exe::crt_fread(record, 0x51a8, 1, file) == 1 {
            exe::llist_add_tail(G_SCOUT_CLUB_TABLE_LIST, record, 4);
        }
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, record);
    }
    exe::crt_fclose(file);
    player_search_title_from_path(this, path);
    write_i8(this + player_search_fields::ASSIGNED_REGION, -1);
    write_i32(this + player_search_fields::ASSIGNED_COMP, -1);
    if keeps_results {
        exe::player_search_purge_invalid_results(this, (!same_game) as u32);
    }
    player_search_file_flags(this);
    1
}

/// Comparer for `player_search_load_from_file`: it reads a file and allocates, so only the original runs, and the plan
/// is checked when it answers 1: the title is the file's name, no region or competition, the flags set.
pub(crate) unsafe extern "thiscall" fn compare_player_search_load_from_file(this: usize, path: usize, quiet: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%)(this, path, quiet);
    if r == 0 {
        return r;
    }
    let mut name = [0u8; 0x100];
    exe::crt_strtok(path, 0usize, 0usize, name.as_mut_ptr(), core::ptr::null_mut());
    let ok = guio_same_c_string(name.as_ptr() as usize, this + 0xb) && read_i8(this + player_search_fields::ASSIGNED_REGION) == -1
        && read_i32(this + player_search_fields::ASSIGNED_COMP) == -1 && read_u32(this + SCOUT_DAY_REPORT_MARK) == 1 && read_u32(this + 0x2ac) == 1;
    SCOREBOARD.record(ok, || format!("player_search_load_from_file({:#x}) returned {}", this, r));
    r
}
