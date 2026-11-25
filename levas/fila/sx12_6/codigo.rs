// ---------- batch %%LEVA%%: resetting a human manager's record and creating the key nation competitions (human_manager.cpp, key_nation.cpp; sx12_6) ----------

/// Fills the 15-byte squad view settings block with its defaults (`SQUAD_VIEW_SETTINGS` constructor, thiscall).
const FN_SQUAD_VIEW_SETTINGS_DEFAULTS: usize = 0x487810;
/// Builds the path of a game folder ("tactics", "search", ...) into a 0x104-byte buffer, creating the folder (cdecl).
const FN_UTILS_BUILD_FOLDER_PATH: usize = 0x90bea0;

/// Resets one tactic slot of a human record (0x1ecd bytes at `slot`): the preset tactic 0, the cleared screen state, the
/// tactics folder and no recent files.
unsafe fn reset_tactic_slot(slot: usize, tactics_folder: &[u8; 0x104]) {
    let preset = exe::tactics_get_tactic_block(read_usize(G_TACTICS), 0, 0);
    core::ptr::copy_nonoverlapping(preset as *const u8, slot as *mut u8, 0x18e3); // the preset tactic
    for at in [0x18e3, 0x18e5, 0x18e7, 0x18e9] {
        write_u16(slot + at, 0); // the zone selections
    }
    write_u8(slot + 0x18eb, 1); // the view mode: overview
    write_u8(slot + 0x18ec, 1); // the previous view mode
    write_u32(slot + 0x18ed, 0); // Show Runs off
    exe::at_9466d0(slot + 0x18f1, tactics_folder.as_ptr(), 0x104); // strncpy: the tactics folder
    write_u8(slot + 0x19f4, 0);
    core::ptr::write_bytes((slot + 0x19f5) as *mut u8, 0, 0x410); // no recent files
    core::ptr::write_bytes((slot + 0x1e05) as *mut u8, 0xff, 0xc8); // 50 empty entries (-1)
}

/// Destroys a pending transfer offer of the record (the pointer at `at`): clears its first dword, then the offer's
/// destructor and `operator delete`.
unsafe fn drop_record_offer(at: usize) {
    if read_usize(at) == 0 {
        return;
    }
    write_u32(read_usize(at), 0);
    let offer = read_usize(at);
    if offer != 0 {
        exe::drop_transfer_offer(offer);
        exe::crt_operator_delete(offer);
    }
}

/// Resets a human manager's record to the defaults of a new manager; returns 1, or 0 after the "Error" box.
///
/// `HUMAN_MANAGER::reset_human_record(manager)`: null manager: the current human (`manager_or_current_human`); none:
/// the box (human_manager.cpp line 0xae); index out of range (`human_manager_human_manager_index`): line 0xba. It builds
/// the four folder paths "tactics", "search", "training" and "print" (0x90bea0) and, in each of the five tactic slots,
/// the preset tactic 0 (`exe::tactics_get_tactic_block`) with the screen state cleared, the tactics folder and no
/// recent files (`reset_tactic_slot`). A manager under computer control (`human_manager_record::COMPUTER_CONTROLLED`)
/// lowers the count at +8, one on holiday (`human_manager_record::ON_HOLIDAY`) the count at +0xc; the two pending offers
/// (+0x9a59, +0x9a5d) are destroyed (`drop_record_offer`). Then: not on holiday, Control Reserve Team off, the draft
/// contract (+0x9a09) a new `STAFF_CONTRACT` (copied field by field, the two dates by `exe::date_operator_3d`), no
/// offers, the dword +0x9a61 and `HOLIDAY_STAMP` 0, tactic slot 0, empty password and typed password, not under
/// computer control, the commentary speed 0x7d (Slow), no watched fixture (79 bytes), the search folder (+0x9ba8),
/// the training folder (+0xa0bc) and the print folder (+0xa5d0) with no recent files, the default squad view (+0xa6d4)
/// and no nicknames (+0xa6e3, +0xa6e7, `NICKNAME_COUNT`). The nickname array is dropped without being freed (a leak
/// when the record had nicknames).
/// Original: `HUMAN_MANAGER::reset_human_record` (0x5fb810, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn reset_human_record(this: usize, manager: usize) -> u32 {
    let mut contract = [0u8; 0x50];
    exe::new_staff_contract(contract.as_mut_ptr() as usize);
    let mut view = [0u8; 0xf];
    let defaults: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_SQUAD_VIEW_SETTINGS_DEFAULTS);
    defaults(view.as_mut_ptr());
    let m = manager_or_current_human(manager);
    if m == 0 {
        show_error_at(0x9cf11c, 0xae); // "...\human_manager.cpp"
        return 0;
    }
    let Some(k) = human_manager_human_manager_index(this, m) else {
        show_error_at(0x9cf11c, 0xba);
        return 0;
    };
    let record = human_manager_record_at(this, k);
    let folder = |name: usize| {
        let mut path = [0u8; 0x104];
        let build: unsafe extern "C" fn(usize, *mut u8) -> u32 = core::mem::transmute(FN_UTILS_BUILD_FOLDER_PATH);
        build(name, path.as_mut_ptr());
        path
    };
    let tactics = folder(0x9cf15c); // "tactics"
    let search = folder(0x9cf154); // "search"
    let training = folder(0x9898f4); // "training"
    let print = folder(0x9cf14c); // "print"
    for k in 0..5 {
        reset_tactic_slot(record + k * 0x1ecd, &tactics);
    }
    covered!("sx12_6 reset human record");
    if read_u32(record + human_manager_record::COMPUTER_CONTROLLED) != 0 {
        write_u32(this + 8, read_u32(this + 8).wrapping_sub(1));
    }
    if read_u32(record + human_manager_record::ON_HOLIDAY) != 0 {
        write_u32(this + 0xc, read_u32(this + 0xc).wrapping_sub(1));
    }
    drop_record_offer(record + 0x9a59);
    drop_record_offer(record + 0x9a5d);
    write_u32(record + human_manager_record::ON_HOLIDAY, 0);
    write_u8(record + human_manager_record::CONTROLS_RESERVE_TEAM, 0);
    let draft = record + 0x9a09; // the draft contract
    let c = contract.as_ptr() as usize;
    core::ptr::copy_nonoverlapping(c as *const u8, draft as *mut u8, 0x25);
    let mut t = [0u8; 8];
    exe::date_operator_3d(draft + 0x25, t.as_mut_ptr() as usize, c + 0x25);
    exe::date_operator_3d(draft + 0x2d, t.as_mut_ptr() as usize, c + 0x2d);
    core::ptr::copy_nonoverlapping((c + 0x35) as *const u8, (draft + 0x35) as *mut u8, 0x1b);
    for at in [0x9a59, 0x9a5d, 0x9a61, human_manager_record::HOLIDAY_STAMP] {
        write_u32(record + at, 0);
    }
    write_u8(record + human_manager_record::ACTIVE_TACTIC_SLOT, 0);
    write_u8(record + 0x9a6a, 0); // the password
    write_u8(record + 0x9a9c, 0); // the typed password
    write_u32(record + human_manager_record::COMPUTER_CONTROLLED, 0);
    write_u16(record + 0x9ad2, 0x7d); // sx12_6: the commentary speed, Slow
    core::ptr::write_bytes((record + human_manager_record::WATCHED_FIXTURE) as *mut u8, 0, 0x4f);
    for (at, path) in [(0x9ba8, &search), (0xa0bc, &training)] {
        exe::at_9466d0(record + at, path.as_ptr(), 0x104);
        write_u8(record + at + 0x103, 0);
        core::ptr::write_bytes((record + at + 0x104) as *mut u8, 0, 0x410); // no recent files
    }
    exe::at_9466d0(record + 0xa5d0, print.as_ptr(), 0x104);
    write_u8(record + 0xa6d3, 0);
    core::ptr::copy_nonoverlapping(view.as_ptr(), (record + 0xa6d4) as *mut u8, 0xf); // the squad view
    write_u32(record + human_manager_record::NICKNAMES, 0);
    write_u32(record + 0xa6e7, 0);
    write_u32(record + human_manager_record::NICKNAME_COUNT, 0);
    1
}

/// Builds the competitions of every active key nation row and sets each row's season dates; returns 1, or 0 after the
/// "Error" box when a row's creation fails.
///
/// `key_nation_create_all_comps()`: for each of the 35 rows of `G_LEAGUE_TABLE` whose nation (+0) is null or chosen
/// to be played (`.league_selected()`) and which has a creation function (`G_LEAGUE_WORLD_CUP_SETUP_FUNC`, +8, cdecl with
/// the row): the function; returning 0: the box (key_nation.cpp line 0x1dc) and 0. Then the row's season start
/// (`key_nation_record::START_DATE`) becomes the date of day +0x32 (word), month +0x34 (byte), year +0x35 and weekday
/// +0x37 (words, `exe::date_date_549f70`), and the season end (`key_nation_record::END_DATE`) the date of day +0x41,
/// month +0x43, year +0x44 and weekday +0x46. Called by the game setup (0x828110, 0x82a9c0: load step 0x1f).
/// Original: `key_nation_create_all_comps` (0x669180, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_create_all_comps() -> u32 {
    for k in 0..0x23usize {
        let row = G_LEAGUE_TABLE + 72 * k;
        let n = Nation(read_usize(row));
        if !n.is_null() && n.league_selected() == 0 {
            continue;
        }
        let create = read_usize(row + 8); // G_LEAGUE_WORLD_CUP_SETUP_FUNC of the row
        if create == 0 {
            continue;
        }
        let create: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(create);
        if create(row) == 0 {
            show_error_at(0x9eaff4, 0x1dc); // "...\key_nation.cpp"
            return 0;
        }
        covered!("sx12_6 key nation competitions");
        set_key_nation_season_dates(row); // sx12_6: the season dates
    }
    1
}

/// Sets a key nation row's season start and end dates from their day, month, year and weekday fields (see
/// `key_nation_create_all_comps`).
unsafe fn set_key_nation_season_dates(row: usize) {
    let mut t = [0u8; 8];
    for (date, parts) in [(key_nation_record::START_DATE, 0x32), (key_nation_record::END_DATE, 0x41)] {
        let mut d = [0u16; 4];
        let built = exe::date_date_549f70(&mut d, read_u16(row + parts) as u32, read_u8(row + parts + 2) as u32,
            read_u16(row + parts + 3) as u32, read_u16(row + parts + 5) as u32);
        exe::date_operator_3d(row + date, t.as_mut_ptr() as usize, built);
    }
}

/// Comparer for `reset_human_record`: the whole record (0xa6ef bytes) and the two counts (+8, +0xc), side by side. With
/// an offer to destroy or the "Error" box, only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_reset_human_record(this: usize, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    let m = manager_or_current_human(manager);
    let Some(k) = (if m != 0 { human_manager_human_manager_index(this, m) } else { None }) else {
        return o(this, manager);
    };
    let record = human_manager_record_at(this, k);
    if read_usize(record + 0x9a59) != 0 || read_usize(record + 0x9a5d) != 0 {
        covered!("sx12_6 reset with offers only in the original");
        return o(this, manager);
    }
    // the draft contract (+0x9a09, 0x50 bytes) is left out: STAFF_CONTRACT::STAFF_CONTRACT (0x85e410) sets only a few
    // fields (+4, +8, the dates, +0x3b, +0x40, +0x45, +0x49..+0x4e) and the original copies the rest of that local, stack
    // leftovers, into the record (DEFEITOS_DO_ORIGINAL); the copy starts from a zeroed contract
    let regions = [(record, 0x9a09), (record + 0x9a59, 0xa6ef - 0x9a59), (this + 8, 8)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, manager), || reset_human_record(this, manager));
    SCOREBOARD.record(equal && r == n, || format!("HUMAN_MANAGER::reset_human_record(record {}) original {} rust {}: {:?}", k, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `key_nation_create_all_comps`: the creation functions build the competitions once, so only the original
/// runs; afterwards each active row's season dates are checked against the copy's (`set_key_nation_season_dates` on a
/// copy of the row).
pub(crate) unsafe extern "C" fn compare_key_nation_create_all_comps() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(%%I1%%)();
    if r == 0 {
        return r;
    }
    for k in 0..0x23usize {
        let row = G_LEAGUE_TABLE + 72 * k;
        let n = Nation(read_usize(row));
        if (!n.is_null() && n.league_selected() == 0) || read_usize(row + 8) == 0 {
            continue;
        }
        let mut copy = [0u8; 72];
        core::ptr::copy_nonoverlapping(row as *const u8, copy.as_mut_ptr(), 72);
        set_key_nation_season_dates(copy.as_mut_ptr() as usize);
        let (a, b) = (core::slice::from_raw_parts(row as *const u8, 72), &copy[..]);
        SCOREBOARD.record(a == b, || format!("key_nation_create_all_comps row {}: original {:02x?} rust {:02x?}", k, a, b));
    }
    r
}
