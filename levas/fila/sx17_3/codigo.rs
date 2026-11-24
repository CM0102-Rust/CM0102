// --- sx17_3: the player search's owner, club record, copy and saved files (player_search.cpp): set_manager_name
// (0x7ae780), set_password (0x7ae7d0), set_assigned_club (0x7b3d50), operator= (0x7b7690), read_file_header
// (0x7bcee0) and the four load dialog filters (0x7bd010, 0x7bd0d0, 0x7bd190, 0x7bd250) ---

/// The size of the header of a saved search file: the search from +5 (magic, version, list type, names...).
const PLAYER_SEARCH_FILE_HEADER_SIZE: usize = 0x16b;
/// The magic number at the start of a saved search file (+5 of the search).
const PLAYER_SEARCH_FILE_MAGIC: u32 = 0x24a9e;
/// The offset of the owner's password inside the file header (+0x139 of the search).
const PLAYER_SEARCH_FILE_PASSWORD: usize = 0x134;

/// Writes the formatted name of `person`, or of the owning manager when null, into the 201-byte name field at +0x70
/// (`database_format_staff_name`, style 1). Answers 1.
/// Original: `PLAYER_SEARCH::set_manager_name` (0x7ae780, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_set_manager_name(this: usize, person: Staff) -> u32 {
    let person = if person.is_null() { staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize) } else { person };
    exe::database_format_staff_name(person, this + 0x70, 0xc9, 1); // sx17_3: the name field, 201 bytes
    1
}

/// Comparer for `player_search_set_manager_name`: side by side over the name field and the return.
pub(crate) unsafe extern "thiscall" fn compare_player_search_set_manager_name(this: usize, person: Staff) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, Staff) -> u32>(%%I0%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this + 0x70, 0xc9)], || o(this, person),
        || player_search_set_manager_name(this, person));
    SCOREBOARD.record(same && r == n, || format!("player_search_set_manager_name({:#x}, {:#x}) {:?}", this, person.addr(), first_differences(&mo, &mr)));
    r
}

/// Copies the password of the human manager `person` (`HUMAN_MANAGER::get_password`) into the search at +0x139 and
/// answers 1; without a password (null or empty), the player_search.cpp "Error" box (line 0x131) and 0.
/// Original: `PLAYER_SEARCH::set_password` (0x7ae7d0, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_set_password(this: usize, person: Staff) -> u32 {
    let password = exe::human_manager_get_password(read_usize(G_HUMAN_MANAGER), person);
    if password == 0 || read_u8(password) == 0 {
        show_error_box(0xa701b0, 0x131); // player_search.cpp
        return 0;
    }
    praise_strcpy(this + 0x139, password);
    1
}

/// Comparer for `player_search_set_password`: side by side over the password field (up to +0x16c) and the return;
/// without a password only the original runs (the box).
pub(crate) unsafe extern "thiscall" fn compare_player_search_set_password(this: usize, person: Staff) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, Staff) -> u32>(%%I1%%);
    let password = exe::human_manager_get_password(read_usize(G_HUMAN_MANAGER), person);
    if password == 0 || read_u8(password) == 0 {
        return o(this, person);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this + 0x139, 0x33)], || o(this, person),
        || player_search_set_password(this, person));
    SCOREBOARD.record(same && r == n, || format!("player_search_set_password({:#x}) {:?}", this, first_differences(&mo, &mr)));
    r
}

/// Sets the club a scout list watches: a scouted club record of another club leaves the global list of club records
/// and is deleted; the club is stored (`player_search_fields::ASSIGNED_CLUB`); with a club (> −1) and no record, a new
/// one (`operator new`, 0x51a8 bytes: the club id and the 50 player slots free, −1) joins the global list. A failed
/// allocation leaves no record.
/// Original: `PLAYER_SEARCH::set_assigned_club` (0x7b3d50, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_set_assigned_club(this: usize, club: i32) {
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    if record != 0 && club != read_i32(this + player_search_fields::ASSIGNED_CLUB) {
        covered!("sx17_3 replace the club record");
        exe::llist_remove(G_SCOUT_CLUB_TABLE_LIST, record);
        exe::crt_operator_delete(record);
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, 0);
    }
    write_i32(this + player_search_fields::ASSIGNED_CLUB, club);
    if club > -1 && read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD) == 0 {
        let record = exe::crt_operator_new(0x51a8);
        if record != 0 {
            write_i32(record, read_i32(this + player_search_fields::ASSIGNED_CLUB));
            for slot in 0..0x32 {
                write_i32(record + 4 + slot * 0x1a2, -1); // sx17_3: the slot is free
            }
            exe::llist_add_tail(G_SCOUT_CLUB_TABLE_LIST, record, 4);
        }
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, record);
    }
}

/// Comparer for `player_search_set_assigned_club`: it allocates and frees, so only the original runs, and the plan is
/// checked: the club stored and, with a club, a record of that club with its 50 slots free (when it is new).
pub(crate) unsafe extern "thiscall" fn compare_player_search_set_assigned_club(this: usize, club: i32) {
    let fresh = club > -1 && (read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD) == 0 || club != read_i32(this + player_search_fields::ASSIGNED_CLUB));
    original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I2%%)(this, club);
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    let ok = read_i32(this + player_search_fields::ASSIGNED_CLUB) == club
        && (!fresh || record == 0 || (read_i32(record) == club && (0..0x32).all(|k| read_i32(record + 4 + k * 0x1a2) == -1)));
    SCOREBOARD.record(ok, || format!("player_search_set_assigned_club({:#x}, {}) record {:#x}", this, club, record));
}

/// Copies a whole player search: the own result array is freed and the own scouted club record leaves the global list
/// and is deleted; the 0x2b0 bytes of `other` are copied over; with a capacity, a new result array (`malloc`; when it
/// fails: the box, line 0x127f, `game_fatal_cleanup` and `exit(-1)`) with `other`'s results copied; with a club record,
/// a new copy of it (`operator new`, 0x51a8 bytes) that joins the global list (0 when the allocation fails).
/// Defect of the original: without a capacity the copy keeps `other`'s result array pointer (both searches then share
/// and free the same array).
/// Original: `PLAYER_SEARCH::operator=` (0x7b7690, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_assign(this: usize, other: usize) {
    let results = read_usize(this + player_search_fields::RESULTS);
    if results != 0 {
        exe::crt_free(results);
        write_usize(this + player_search_fields::RESULTS, 0);
    }
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    if record != 0 {
        exe::llist_remove(G_SCOUT_CLUB_TABLE_LIST, record);
        exe::crt_operator_delete(record);
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, 0);
    }
    core::ptr::copy_nonoverlapping(other as *const u8, this as *mut u8, 0x2b0);
    let capacity = read_i32(this + SCOUT_DAY_RESULT_CAPACITY);
    if capacity > 0 {
        let array = exe::crt_malloc((capacity as u32).wrapping_mul(SCOUT_DAY_RESULT_SIZE as u32));
        write_usize(this + player_search_fields::RESULTS, array);
        if array == 0 {
            show_error_box(0xa701b0, 0x127f); // player_search.cpp
            exe::game_fatal_cleanup();
            exe::shared_noop_ret0_5a8f60();
            exe::crt_exit(-1);
        }
        let count = read_i32(this + player_search_fields::RESULT_COUNT);
        if count > 0 {
            core::ptr::copy_nonoverlapping(read_usize(other + player_search_fields::RESULTS) as *const u8, array as *mut u8,
                                           count as usize * SCOUT_DAY_RESULT_SIZE);
        }
    }
    if read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD) != 0 {
        covered!("sx17_3 copy the club record");
        let copy = exe::crt_operator_new(0x51a8);
        if copy != 0 {
            core::ptr::copy_nonoverlapping(read_usize(other + player_search_fields::SCOUTED_CLUB_RECORD) as *const u8, copy as *mut u8, 0x51a8);
            exe::llist_add_tail(G_SCOUT_CLUB_TABLE_LIST, copy, 4);
        }
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, copy);
    }
}

/// Comparer for `player_search_assign`: it allocates and frees, so only the original runs, and the plan is checked:
/// the record equal to `other` outside the two pointers, the results and the club record copied (own arrays when the
/// capacity is positive).
pub(crate) unsafe extern "thiscall" fn compare_player_search_assign(this: usize, other: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I3%%)(this, other);
    let same_bytes = |a: usize, b: usize, n: usize| core::slice::from_raw_parts(a as *const u8, n) == core::slice::from_raw_parts(b as *const u8, n);
    let mut ok = same_bytes(this, other, player_search_fields::SCOUTED_CLUB_RECORD)
        && same_bytes(this + 0x174, other + 0x174, player_search_fields::RESULTS - 0x174)
        && same_bytes(this + 0x1f2, other + 0x1f2, 0x2b0 - 0x1f2);
    let count = read_i32(this + player_search_fields::RESULT_COUNT);
    if read_i32(this + SCOUT_DAY_RESULT_CAPACITY) > 0 && count > 0 {
        ok &= same_bytes(read_usize(this + player_search_fields::RESULTS), read_usize(other + player_search_fields::RESULTS), count as usize * SCOUT_DAY_RESULT_SIZE);
    }
    let (mine, theirs) = (read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD), read_usize(other + player_search_fields::SCOUTED_CLUB_RECORD));
    if mine != 0 {
        ok &= same_bytes(mine, theirs, 0x51a8);
    }
    SCOREBOARD.record(ok, || format!("player_search_assign({:#x}, {:#x})", this, other));
}

/// Reads the 0x16b-byte header of a saved search file into `out` (zeroed first) and answers whether it is a search file
/// of the list type `kind`: the magic 0x24a9e, version 1 and the type byte. Without a path, the player_search.cpp
/// "Error" box (line 0x1ef6) and 0; a file that does not open ("rb") or a short read, 0.
/// Original: `player_search_read_file_header` (0x7bcee0, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_read_file_header(path: usize, out: usize, kind: u32) -> u32 {
    core::ptr::write_bytes(out as *mut u8, 0, PLAYER_SEARCH_FILE_HEADER_SIZE);
    if path == 0 {
        show_error_box(0xa701b0, 0x1ef6); // player_search.cpp
        return 0;
    }
    let file = exe::crt_fopen(path, BADGE_OPEN_MODE);
    if file == 0 {
        return 0;
    }
    let read = exe::crt_fread(out, PLAYER_SEARCH_FILE_HEADER_SIZE as u32, 1, file);
    exe::crt_fclose(file);
    if read != 1 || read_u32(out) != PLAYER_SEARCH_FILE_MAGIC || read_u8(out + 4) != 1 {
        return 0;
    }
    (read_u8(out + 5) == kind as u8) as u32 // sx17_3: the list type must match
}

/// Comparer for `player_search_read_file_header`: the file is only read, so both run side by side over the header and
/// the answer; without a path only the original runs (the box).
pub(crate) unsafe extern "C" fn compare_player_search_read_file_header(path: usize, out: usize, kind: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u32>(%%I4%%);
    if path == 0 {
        return o(path, out, kind);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, PLAYER_SEARCH_FILE_HEADER_SIZE)], || o(path, out, kind),
        || player_search_read_file_header(path, out, kind));
    SCOREBOARD.record(same && r == n, || format!("player_search_read_file_header(kind {}) original={} rust={} {:?}", kind, r, n, first_differences(&mo, &mr)));
    r
}

/// The load dialog's answer for a file of the list type `kind`: 2 when it is not a search file of that type, 1 when it
/// has no owner password, 9 when the password is the current human manager's (`HUMAN_MANAGER::get_password`) and 5
/// when it is someone else's.
unsafe fn player_search_filter_file(path: usize, kind: u32) -> u32 {
    let mut header = [0u8; PLAYER_SEARCH_FILE_HEADER_SIZE + 1];
    let h = header.as_mut_ptr() as usize;
    if player_search_read_file_header(path, h, kind) == 0 {
        return 2;
    }
    if read_u8(h + PLAYER_SEARCH_FILE_PASSWORD) == 0 {
        return 1;
    }
    let password = exe::human_manager_get_password(read_usize(G_HUMAN_MANAGER), 0usize);
    if guio_same_c_string(password, h + PLAYER_SEARCH_FILE_PASSWORD) { 9 } else { 5 } // sx17_3: own file or someone else's
}

/// The load dialog's filter for saved short lists (list type 1): see `player_search_filter_file`.
/// Original: `player_search_filter_short_list_file` (0x7bd010, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_filter_short_list_file(path: usize) -> u32 {
    player_search_filter_file(path, 1)
}

/// The load dialog's filter for saved quick searches (list type 2): see `player_search_filter_file`.
/// Original: `player_search_filter_quick_search_file` (0x7bd0d0, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_filter_quick_search_file(path: usize) -> u32 {
    player_search_filter_file(path, 2)
}

/// The load dialog's filter for saved staff searches (list type 3): see `player_search_filter_file`.
/// Original: `player_search_filter_staff_search_file` (0x7bd190, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_filter_staff_search_file(path: usize) -> u32 {
    player_search_filter_file(path, 3)
}

/// The load dialog's filter for saved scout searches (list type 4): see `player_search_filter_file`.
/// Original: `player_search_filter_scout_search_file` (0x7bd250, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_filter_scout_search_file(path: usize) -> u32 {
    player_search_filter_file(path, 4)
}

/// Runs a load dialog filter side by side (it only reads the file): the answers must match.
unsafe fn player_search_compare_filter(name: &str, path: usize, original: unsafe extern "C" fn(usize) -> u32, copy: unsafe extern "C" fn(usize) -> u32) -> u32 {
    if path == 0 {
        return original(path);
    }
    let r = original(path);
    let n = copy(path);
    SCOREBOARD.record(r == n, || format!("{} original={} rust={}", name, r, n));
    r
}

/// Comparer for `player_search_filter_short_list_file`.
pub(crate) unsafe extern "C" fn compare_player_search_filter_short_list_file(path: usize) -> u32 {
    player_search_compare_filter("player_search_filter_short_list_file", path, original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I5%%),
                                 player_search_filter_short_list_file)
}

/// Comparer for `player_search_filter_quick_search_file`.
pub(crate) unsafe extern "C" fn compare_player_search_filter_quick_search_file(path: usize) -> u32 {
    player_search_compare_filter("player_search_filter_quick_search_file", path, original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I6%%),
                                 player_search_filter_quick_search_file)
}

/// Comparer for `player_search_filter_staff_search_file`.
pub(crate) unsafe extern "C" fn compare_player_search_filter_staff_search_file(path: usize) -> u32 {
    player_search_compare_filter("player_search_filter_staff_search_file", path, original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I7%%),
                                 player_search_filter_staff_search_file)
}

/// Comparer for `player_search_filter_scout_search_file`.
pub(crate) unsafe extern "C" fn compare_player_search_filter_scout_search_file(path: usize) -> u32 {
    player_search_compare_filter("player_search_filter_scout_search_file", path, original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I8%%),
                                 player_search_filter_scout_search_file)
}
