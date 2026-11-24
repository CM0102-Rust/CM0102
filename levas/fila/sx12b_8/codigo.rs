// ---------- batch %%LEVA%%: the human manager's folders, recent files and options (human_manager.cpp; sx12b_8) ----------

/// The kinds of file a human manager keeps a folder and four recent files for (the `kind` byte of the routines):
/// 1 tactics (per tactic slot), 2 search, 3 training, anything else print; 4 is refused by the recent file routines.
const FILE_KIND_TACTICS: i8 = 1;
const FILE_KIND_TRAINING: i8 = 3;
const FILE_KIND_REFUSED: i8 = 4;
/// One folder or recent file path (0x104 bytes).
const PATH_SIZE: usize = 0x104;

/// The record offset of the human manager's tactic slot block in use (`human_manager_record::ACTIVE_TACTIC_SLOT`,
/// signed, x 0x1ecd).
unsafe fn active_slot_offset(record: usize) -> usize {
    (read_i8(record + human_manager_record::ACTIVE_TACTIC_SLOT) as isize * 0x1ecd) as usize
}

/// The first of the four recent file paths of a kind in a human record (tactics: of the active slot, +0x19f5; training
/// +0xa1c0; the others, the search list +0x9cac).
unsafe fn recent_files(record: usize, kind: i8) -> usize {
    match kind {
        FILE_KIND_TACTICS => record.wrapping_add(active_slot_offset(record)) + 0x19f5,
        FILE_KIND_TRAINING => record + 0xa1c0,
        _ => record + 0x9cac,
    }
}

/// Returns the human manager's folder path for a kind of file: tactics (1, the active tactic slot's, +0x18f1), search
/// (2, +0x9ba8), training (3, +0xa0bc), anything else print (+0xa5d0). No manager or out of range: the "Error" box
/// (human_manager.cpp lines 0x249, 0x255) and 0.
/// Original: `HUMAN_MANAGER::get_folder_path` (0x5fc5b0, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_folder_path(this: usize, manager: usize, kind: i8) -> usize {
    let Some(record) = human_record_or_error(this, manager, 0x249, 0x255) else {
        return 0;
    };
    match kind {
        FILE_KIND_TACTICS => record.wrapping_add(active_slot_offset(record)) + 0x18f1,
        FILE_KIND_TRAINING => record + 0xa0bc,
        2 => record + 0x9ba8, // sx12b_8: the search folder
        _ => record + 0xa5d0,
    }
}

/// Copies recent file `index` (0..3) of a kind into `out` (at least 0x104 bytes); with `strip`, a file in the kind's
/// own folder keeps only its name and extension. Returns 1.
///
/// `HUMAN_MANAGER::get_recent_file(index, out, size, strip, manager, kind)`: a null `out` or a `size` below 0x104: the
/// "Error" box (line 0x286) and 0. `out` is emptied; an index out of 0..3: line 0x291; no manager (null: the current
/// human): 0x29c; kind 4: 0x2a5; out of range: 0x2b0; each with 0. The path is copied from the kind's list
/// (`recent_files`). With `strip`, it is split (`_splitpath`); its drive and folder (`_makepath`) are compared, case
/// blind (`stricmp`), with the kind's folder (tactics: the active slot's +0x18f1; training +0xa0bc; any other the
/// search folder +0x9ba8) ended by "\"; when equal, `out` becomes the name and extension. Defect of the original: with
/// an empty folder it tests the byte before its buffer for the "\".
/// Original: `HUMAN_MANAGER::get_recent_file` (0x5fc760, human_manager.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn get_recent_file(this: usize, index: i8, out: usize, size: i32, strip: u32, manager: usize,
                                                       kind: i8) -> u32 {
    const FILE: usize = 0x9cf11c; // "...\human_manager.cpp"
    if out == 0 || size < PATH_SIZE as i32 {
        show_error_at(FILE, 0x286);
        return 0;
    }
    write_u8(out, 0);
    if !(0..4).contains(&index) {
        show_error_at(FILE, 0x291);
        return 0;
    }
    if manager_or_current_human(manager) == 0 {
        show_error_at(FILE, 0x29c);
        return 0;
    }
    if kind == FILE_KIND_REFUSED {
        show_error_at(FILE, 0x2a5);
        return 0;
    }
    let Some(k) = human_manager_human_manager_index(this, manager_or_current_human(manager)) else {
        show_error_at(FILE, 0x2b0);
        return 0;
    };
    let record = human_manager_record_at(this, k);
    match_screens_c_strcpy(out as *mut u8, (recent_files(record, kind) + index as usize * PATH_SIZE) as *const u8);
    if strip != 0 {
        let (mut drive, mut dir, mut name, mut ext) = ([0u8; 0x100], [0u8; 0x100], [0u8; 0x100], [0u8; 0x100]);
        exe::crt_strtok(out, drive.as_mut_ptr() as usize, dir.as_mut_ptr() as usize, name.as_mut_ptr(), ext.as_mut_ptr()); // _splitpath
        let mut where_ = [0u8; 0x104];
        exe::crt_makepath(where_.as_mut_ptr(), drive.as_ptr() as usize, dir.as_ptr() as usize, 0xad9c64usize, 0xad9c64usize); // "" ""
        let folder = match kind {
            FILE_KIND_TACTICS => record.wrapping_add(active_slot_offset(record)) + 0x18f1,
            FILE_KIND_TRAINING => record + 0xa0bc,
            _ => record + 0x9ba8,
        };
        let mut own = [0u8; 0x108];
        match_screens_c_strcpy(own.as_mut_ptr(), folder as *const u8);
        let n = own.iter().position(|&c| c == 0).unwrap_or(0);
        if n == 0 || own[n - 1] != b'\\' {
            match_screens_c_strcat(own.as_mut_ptr(), 0x9c3730 as *const u8); // "\"
        }
        if exe::crt_stricmp(where_.as_ptr() as usize, own.as_ptr() as usize) == 0 {
            covered!("sx12b_8 recent file in its own folder");
            exe::crt_makepath(out as *mut u8, 0xad9c64usize, 0xad9c64usize, name.as_ptr() as usize, ext.as_ptr() as usize); // sx12b_8: the bare name
        }
    }
    1
}

/// Puts a file path at the top of the four recent files of a kind: when it is already in the list it moves up from
/// there, otherwise the last one is dropped; the ones above move down one place (each copied with `strncpy` of 0x104
/// and cut at 0x103). Returns 1.
///
/// `HUMAN_MANAGER::add_recent_file(path, manager, kind)`: a null path: the "Error" box (line 0x328) and 0; no manager
/// (null: the current human): 0x333; kind 4: 0x33c; out of range: 0x347; each with 0. The path is looked for in the
/// list case blind (`stricmp`).
/// Original: `HUMAN_MANAGER::add_recent_file` (0x5fcb60, human_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_recent_file(this: usize, path: usize, manager: usize, kind: i8) -> u32 {
    const FILE: usize = 0x9cf11c; // "...\human_manager.cpp"
    if path == 0 {
        show_error_at(FILE, 0x328);
        return 0;
    }
    if manager_or_current_human(manager) == 0 {
        show_error_at(FILE, 0x333);
        return 0;
    }
    if kind == FILE_KIND_REFUSED {
        show_error_at(FILE, 0x33c);
        return 0;
    }
    let Some(k) = human_manager_human_manager_index(this, manager_or_current_human(manager)) else {
        show_error_at(FILE, 0x347);
        return 0;
    };
    let entry = |i: usize| recent_files(human_manager_record_at(this, k), kind) + i * PATH_SIZE;
    let found = (0..4).find(|&i| exe::crt_stricmp(entry(i), path) == 0).unwrap_or(3); // sx12b_8: the place it leaves
    for i in (1..=found).rev() {
        exe::at_9466d0(entry(i), entry(i - 1) as *const u8, PATH_SIZE as u32);
        write_u8(entry(i) + PATH_SIZE - 1, 0);
    }
    exe::at_9466d0(entry(0), path as *const u8, PATH_SIZE as u32);
    write_u8(entry(0) + PATH_SIZE - 1, 0);
    covered!("sx12b_8 recent file added");
    1
}

/// Sets one option byte of a human record (`option`) to `value` and, with a match day running (`G_MATCH_DAY`), refreshes
/// its human interest flags; no manager or out of range: the "Error" box with the given lines.
unsafe fn set_option_byte(this: usize, value: u8, manager: usize, option: usize, rows: (u32, u32)) {
    if let Some(record) = human_record_or_error(this, manager, rows.0, rows.1) {
        write_u8(record + option, value);
        let match_day = read_usize(G_MATCH_DAY);
        if match_day != 0 {
            exe::match_day_refresh_human_interest_flags(match_day);
        }
    }
}

/// Sets the human manager's Control Reserve Team option (`human_manager_record::CONTROLS_RESERVE_TEAM`) and, with a
/// match day running, refreshes its human interest flags; no manager or out of range: the "Error" box (lines 0x488,
/// 0x494).
/// Original: `HUMAN_MANAGER::set_controls_reserve_team` (0x5fd590, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_controls_reserve_team(this: usize, value: u8, manager: usize) {
    set_option_byte(this, value, manager, human_manager_record::CONTROLS_RESERVE_TEAM, (0x488, 0x494)); // sx12b_8: the reserve option
}

/// Sets the human manager's control all national teams option (`human_manager_record::CONTROLS_ALL_NATIONAL_TEAMS`)
/// and, with a match day running, refreshes its human interest flags; no manager or out of range: the "Error" box
/// (lines 0x4e4, 0x4f0).
/// Original: `HUMAN_MANAGER::set_controls_all_national_teams` (0x5fd810, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_controls_all_national_teams(this: usize, value: u8, manager: usize) {
    set_option_byte(this, value, manager, human_manager_record::CONTROLS_ALL_NATIONAL_TEAMS, (0x4e4, 0x4f0));
}

/// Sets or clears the human manager's computer control flag (`human_manager_record::COMPUTER_CONTROLLED`), keeping the
/// count of computer controlled managers (+8: one more when it turns on, one less when it turns off), and with a match
/// day running refreshes its human interest flags; no manager or out of range: the "Error" box (lines 0x642, 0x64e).
/// Original: `HUMAN_MANAGER::set_computer_controlled` (0x5fe140, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_computer_controlled(this: usize, flag: u32, manager: usize) {
    let Some(record) = human_record_or_error(this, manager, 0x642, 0x64e) else {
        return;
    };
    let was = read_u32(record + human_manager_record::COMPUTER_CONTROLLED);
    if flag != 0 && was == 0 {
        write_u32(this + 8, read_u32(this + 8).wrapping_add(1)); // sx12b_8: one more under computer control
    } else if flag == 0 && was != 0 {
        write_u32(this + 8, read_u32(this + 8).wrapping_sub(1));
    }
    write_u32(record + human_manager_record::COMPUTER_CONTROLLED, flag);
    let match_day = read_usize(G_MATCH_DAY);
    if match_day != 0 {
        exe::match_day_refresh_human_interest_flags(match_day);
    }
}

/// Whether a fixture (0x4f bytes) is one of the live match day's: its day (+0x2a), year (+0x28) and time slice (+0x3f)
/// are today's (`G_CURRENT_DATE`, `G_CURRENT_DAY_SLICE`), with a match day running and its screen not showing.
unsafe fn is_live_fixture(fixture: usize) -> bool {
    read_u16(fixture + 0x2a) as i32 == read_i16(G_CURRENT_DATE) as i32 && read_u16(fixture + 0x28) == read_u16(G_CURRENT_DATE_YEAR)
        && read_u8(fixture + 0x3f) == read_u8(G_CURRENT_DAY_SLICE) && read_usize(G_MATCH_DAY) != 0
        && read_u32(G_MATCH_DAY_SCREEN_SHOWING_FLAG) == 0
}

/// Stores (a fixture) or clears (null) the fixture the human manager watches (the 0x4f bytes at
/// `human_manager_record::WATCHED_FIXTURE`); returns 1, or 0 when refused.
///
/// `HUMAN_MANAGER::set_watched_fixture(fixture, manager)`: no manager (null: the current human): 0 with no box; out of
/// range: the "Error" box (line 0x6d9) and 0. A fixture the manager cannot watch (`HUMAN_MANAGER::can_watch_fixture`):
/// 0. A fixture of the live match day (`is_live_fixture`), or clearing while the watched one is of it: 0. Otherwise the
/// fixture is copied (or the block zeroed) and, with a match day running, its human interest flags refreshed; 1.
/// Original: `HUMAN_MANAGER::set_watched_fixture` (0x5fe500, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_watched_fixture(this: usize, fixture: usize, manager: usize) -> u32 {
    let m = manager_or_current_human(manager);
    if m == 0 {
        return 0;
    }
    let Some(k) = human_manager_human_manager_index(this, m) else {
        show_error_at(0x9cf11c, 0x6d9); // "...\human_manager.cpp"
        return 0;
    };
    let watched = human_manager_record_at(this, k) + human_manager_record::WATCHED_FIXTURE;
    if fixture != 0 {
        if exe::human_manager_can_watch_fixture(this, fixture, m) == 0 || is_live_fixture(fixture) {
            return 0;
        }
        core::ptr::copy(fixture as *const u8, watched as *mut u8, 0x4f);
    } else {
        if is_live_fixture(watched) {
            return 0; // sx12b_8: the live match stays watched
        }
        core::ptr::write_bytes(watched as *mut u8, 0, 0x4f);
    }
    let match_day = read_usize(G_MATCH_DAY);
    if match_day != 0 {
        exe::match_day_refresh_human_interest_flags(match_day);
    }
    1
}

/// Sets one holiday option byte of the human manager (`human_manager_record::HOLIDAY_OPTIONS` + option) and, for option
/// 3 turned on with the manager at a club (`staff` +0x39), saves the club's current selection: for each of the 16
/// `SAVED_LINEUP` pairs, the player in that slot of the club's tactic (`Tactics::get_selected_player`; none: -1, -1)
/// and, for the first eleven, his position (`Formation::get_slot_position` of the club's tactic,
/// `Tactics::get_club_tactic`), -1 for the substitutes. No manager (null: the current human) or out of range: the
/// "Error" box (lines 0x7bc, 0x7c8).
/// Original: `HUMAN_MANAGER::set_holiday_option` (0x5feb60, human_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn human_manager_set_holiday_option(this: usize, option: i8, value: u32, manager: usize) {
    let Some(record) = human_record_or_error(this, manager, 0x7bc, 0x7c8) else {
        return;
    };
    let m = manager_or_current_human(manager);
    write_u8(record.wrapping_add(option as isize as usize) + human_manager_record::HOLIDAY_OPTIONS, value as u8);
    let club = read_usize(m + 0x39); // the manager's club
    if option != 3 || value == 0 || club == 0 {
        return;
    }
    covered!("sx12b_8 holiday selection saved");
    let tactic = exe::tactics_get_club_tactic(read_usize(G_TACTICS), club, 0);
    for k in 0..0x10u32 {
        let pair = record + human_manager_record::SAVED_LINEUP + 8 * k as usize;
        let player = exe::tactics_get_selected_player(read_usize(G_TACTICS), read_usize(m + 0x39), k, 0);
        if player == 0 {
            write_u32(pair, u32::MAX);
            write_u32(pair + 4, u32::MAX);
            continue;
        }
        write_u32(pair, read_u32(player));
        let position = if k < 0xb { exe::formation_get_slot_position(tactic, k, 0) as i16 as i32 } else { -1 }; // sx12b_8: the slot position
        write_i32(pair + 4, position);
    }
}

/// Comparer for `get_folder_path`: the return (it only reads); the "Error" box: only the original.
pub(crate) unsafe extern "thiscall" fn compare_get_folder_path(this: usize, manager: usize, kind: i8) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> usize>(%%I0%%);
    if !human_record_found(this, manager) {
        return o(this, manager, kind);
    }
    let r = o(this, manager, kind);
    let n = get_folder_path(this, manager, kind);
    SCOREBOARD.record(r == n, || format!("HUMAN_MANAGER::get_folder_path(kind {}) (file screens) original {:#x} rust {:#x}", kind, r, n));
    r
}

/// Whether `get_recent_file` passes its checks (no "Error" box).
unsafe fn recent_file_ok(this: usize, index: i8, out: usize, size: i32, manager: usize, kind: i8) -> bool {
    out != 0 && size >= PATH_SIZE as i32 && (0..4).contains(&index) && kind != FILE_KIND_REFUSED && human_record_found(this, manager)
}

/// Comparer for `get_recent_file`: the path written (0x104 bytes) and the return, side by side; a box: only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_get_recent_file(this: usize, index: i8, out: usize, size: i32, strip: u32, manager: usize,
                                                               kind: i8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, i32, u32, usize, i8) -> u32>(%%I1%%);
    if !recent_file_ok(this, index, out, size, manager, kind) {
        return o(this, index, out, size, strip, manager, kind);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(out, PATH_SIZE)], || o(this, index, out, size, strip, manager, kind),
        || get_recent_file(this, index, out, size, strip, manager, kind));
    SCOREBOARD.record(equal && r == n, || format!("HUMAN_MANAGER::get_recent_file({}, strip {}, kind {}) (file screens): {:?}", index, strip, kind,
        first_differences(&mo, &mr)));
    r
}

/// Comparer for `add_recent_file`: the record and the return, side by side; a box: only the original.
pub(crate) unsafe extern "thiscall" fn compare_add_recent_file(this: usize, path: usize, manager: usize, kind: i8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i8) -> u32>(%%I2%%);
    let m = manager_or_current_human(manager);
    if path == 0 || kind == FILE_KIND_REFUSED || !human_record_found(this, manager) {
        return o(this, path, manager, kind);
    }
    let record = human_manager_record_at(this, human_manager_human_manager_index(this, m).unwrap_or(0));
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(record, 0xa6ef)], || o(this, path, manager, kind),
        || add_recent_file(this, path, manager, kind));
    SCOREBOARD.record(equal && r == n, || format!("HUMAN_MANAGER::add_recent_file(kind {}) (file screens): {:?}", kind, first_differences(&mo, &mr)));
    r
}

/// Comparer of the option setters: the record (0xa6ef bytes) and the manager's counts (+8, +0xc), side by side, with the
/// match day's refresh run by both (it only rebuilds the flags from the records); a box: only the original.
unsafe fn compare_human_option(name: &str, this: usize, manager: usize, original: impl FnOnce(), copy: impl FnOnce()) {
    if !human_record_found(this, manager) {
        return original();
    }
    let m = manager_or_current_human(manager);
    let record = human_manager_record_at(this, human_manager_human_manager_index(this, m).unwrap_or(0));
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(record, 0xa6ef), (this + 8, 8)], original, copy);
    SCOREBOARD.record(equal, || format!("HUMAN_MANAGER::{} (options screens): {:?}", name, first_differences(&mo, &mr)));
}

/// Comparer for `set_controls_reserve_team` (see `compare_human_option`).
pub(crate) unsafe extern "thiscall" fn compare_set_controls_reserve_team(this: usize, value: u8, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8, usize)>(%%I3%%);
    compare_human_option("set_controls_reserve_team", this, manager, || o(this, value, manager), || set_controls_reserve_team(this, value, manager));
}

/// Comparer for `set_controls_all_national_teams` (see `compare_human_option`).
pub(crate) unsafe extern "thiscall" fn compare_set_controls_all_national_teams(this: usize, value: u8, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8, usize)>(%%I4%%);
    compare_human_option("set_controls_all_national_teams", this, manager, || o(this, value, manager),
        || set_controls_all_national_teams(this, value, manager));
}

/// Comparer for `set_computer_controlled` (see `compare_human_option`).
pub(crate) unsafe extern "thiscall" fn compare_set_computer_controlled(this: usize, flag: u32, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(%%I5%%);
    compare_human_option("set_computer_controlled", this, manager, || o(this, flag, manager), || set_computer_controlled(this, flag, manager));
}

/// Comparer for `set_watched_fixture`: the record and the return, side by side; no manager or out of range: only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_set_watched_fixture(this: usize, fixture: usize, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I6%%);
    if !human_record_found(this, manager) {
        return o(this, fixture, manager);
    }
    let m = manager_or_current_human(manager);
    let record = human_manager_record_at(this, human_manager_human_manager_index(this, m).unwrap_or(0));
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(record, 0xa6ef)], || o(this, fixture, manager), || set_watched_fixture(this, fixture, manager));
    SCOREBOARD.record(equal && r == n, || format!("HUMAN_MANAGER::set_watched_fixture (watch match link) original {} rust {}: {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `human_manager_set_holiday_option` (see `compare_human_option`).
pub(crate) unsafe extern "thiscall" fn compare_human_manager_set_holiday_option(this: usize, option: i8, value: u32, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u32, usize)>(%%I7%%);
    compare_human_option("set_holiday_option", this, manager, || o(this, option, value, manager), || human_manager_set_holiday_option(this, option, value, manager));
}
