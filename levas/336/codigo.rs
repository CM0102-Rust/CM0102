// ---------- batch 336: resetting a human manager's record and creating the key nation competitions (human_manager.cpp, key_nation.cpp; sx12_6) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RESET_HUMAN_RECORD);
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
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_KEY_NATION_CREATE_ALL_COMPS)();
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

// ---------- batch 336: loading the hall of fame file and the order of its entries (hall_of_fame.cpp; sx12_7) ----------

/// `hall_of_fame_unscramble_and_check` (cdecl), called through the exe so that the comparison sees it.
const FN_HALL_OF_FAME_UNSCRAMBLE_AND_CHECK: usize = 0x5f0680;
/// The 480 dwords of the hall of fame scrambling key.
const HALL_OF_FAME_KEY: usize = 0x9cdc88;

/// Reads `hall_of_fame.bin` into the 28 hall of fame tables; returns 1, or 0 when the file is missing or bad.
///
/// `HallOfFame::load()`: opens the file ("rb"); missing: 0 with no box. Reads the 5-byte header (failing: the "Error"
/// box, hall_of_fame.cpp line 0x1c0), checks the magic dword 0x3abfb2 (0x1c8) and the version byte 1 (0x1d0); these
/// close the file and return 0. Then the tables (`hall_of_fame_consts`, 28 × 12000 = 0x52080 bytes, into `this`; 0x1d8),
/// the checksum dword (0x1e1) and the unscrambling with its check (`hall_of_fame_unscramble_and_check`; 0x1ea): any of
/// these failing shows the box, clears the whole table, closes the file and returns 0. Otherwise the file is closed and
/// it returns 1. Called once by the constructor `HallOfFame::HallOfFame` (0x5ef0b0), which writes a new file when it
/// fails.
/// Original: `HallOfFame::load` (0x5efaa0, hall_of_fame.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_load(this: usize) -> u32 {
    use hall_of_fame_consts as hf;
    const BYTES: u32 = (hf::TABLES * hf::TABLE_SIZE) as u32;
    let file = exe::crt_fopen(0x9ce450usize, 0x98f928usize); // "hall_of_fame.bin", "rb"
    if file == 0 {
        return 0;
    }
    let failed = |row: u32, clear: bool| {
        show_error_at(hf::HALL_OF_FAME_CPP, row);
        if clear {
            core::ptr::write_bytes(this as *mut u8, 0, BYTES as usize);
        }
        exe::crt_fclose(file);
        0
    };
    let mut header = [0u8; 5];
    if exe::crt_fread(header.as_mut_ptr() as usize, 5, 1, file) != 1 {
        return failed(0x1c0, false);
    }
    if read_u32(header.as_ptr() as usize) != 0x3abfb2 {
        return failed(0x1c8, false); // not a hall of fame file
    }
    if header[4] != 1 {
        return failed(0x1d0, false); // another version
    }
    if exe::crt_fread(this, BYTES, 1, file) != 1 {
        return failed(0x1d8, true);
    }
    let mut checksum = 0u32;
    if exe::crt_fread(&mut checksum as *mut u32 as usize, 4, 1, file) != 1 {
        return failed(0x1e1, true);
    }
    let check: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_UNSCRAMBLE_AND_CHECK);
    if check(this, BYTES as i32, checksum) == 0 {
        return failed(0x1ea, true); // sx12_7: a bad checksum
    }
    covered!("sx12_7 hall of fame loaded");
    exe::crt_fclose(file);
    1
}

/// Unscrambles a hall of fame buffer in place and tells whether its byte sum matches the checksum.
///
/// `hall_of_fame_unscramble_and_check(buffer, length, checksum)`: a null buffer or a negative length shows the "Error"
/// box (hall_of_fame.cpp line 0x39f) and returns 0. From the last byte down (i = length − 1 .. 0), it swaps the bytes
/// at key[i mod 480] mod length and key[(i + 1) mod 480] mod length (the key dwords at 0x9cdc88, signed remainders);
/// then XORs each byte i with the low byte of key[i mod 480] and adds the result to a sum. Returns 1 when the sum
/// equals the checksum, else 0.
/// Original: `hall_of_fame_unscramble_and_check` (0x5f0680, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_unscramble_and_check(buffer: usize, length: i32, checksum: u32) -> u32 {
    if buffer == 0 || length < 0 {
        show_error_at(hall_of_fame_consts::HALL_OF_FAME_CPP, 0x39f);
        return 0;
    }
    let key = |i: u32| read_i32(HALL_OF_FAME_KEY + 4 * (i % 0x1e0) as usize);
    let mut i = length - 1;
    while i >= 0 {
        let a = buffer + (key(i as u32) % length) as usize;
        let b = buffer + (key(i as u32 + 1) % length) as usize;
        let t = read_u8(a);
        write_u8(a, read_u8(b));
        write_u8(b, t);
        i -= 1;
    }
    let mut sum = 0u32;
    for i in 0..length as u32 {
        let v = read_u8(buffer + i as usize) ^ key(i) as u8;
        write_u8(buffer + i as usize, v); // sx12_7: the unscrambled byte
        sum = sum.wrapping_add(v as u32);
    }
    covered!("sx12_7 hall of fame unscrambled");
    (sum == checksum) as u32
}

/// Orders two hall of fame entries for qsort: the higher score (`hall_of_fame_consts::SCORE`, unsigned) first, then
/// the older time stamp (`hall_of_fame_consts::TIME`, signed) first; equal: 0. A null a gives 1 and a null b -1.
/// Original: `hall_of_fame_compare_entries` (0x5f0620, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_compare_entries(a: usize, b: usize) -> i32 {
    use hall_of_fame_consts as hf;
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (sa, sb) = (read_u32(a + hf::SCORE), read_u32(b + hf::SCORE));
    if sa < sb {
        return 1;
    }
    if sa > sb {
        return -1;
    }
    covered!("sx12_7 hall of fame equal scores");
    let (ta, tb) = (read_i32(a + hf::TIME), read_i32(b + hf::TIME));
    if tb < ta {
        1
    } else if ta >= tb {
        0
    } else {
        -1 // sx12_7: the older entry first
    }
}

/// Comparer for `hall_of_fame_load`: the original loads into the table; when it succeeds the copy loads the same file
/// into a buffer of its own and the two tables and returns are compared. A failed load (missing file or the "Error"
/// box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_load(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HALL_OF_FAME_LOAD)(this);
    if r == 0 {
        return r;
    }
    let bytes = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    let mut copy = vec![0u8; bytes];
    let n = hall_of_fame_load(copy.as_mut_ptr() as usize);
    let original = core::slice::from_raw_parts(this as *const u8, bytes);
    let first = original.iter().zip(copy.iter()).position(|(x, y)| x != y);
    SCOREBOARD.record(r == n && first.is_none(), || format!("HallOfFame::load original {} rust {}, first different byte {:?}", r, n, first));
    r
}

/// Comparer for `hall_of_fame_unscramble_and_check`: the buffer and the return, side by side. A null buffer or a
/// negative length (the "Error" box): only the original.
pub(crate) unsafe extern "C" fn compare_hall_of_fame_unscramble_and_check(buffer: usize, length: i32, checksum: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, u32) -> u32>(IDX_HALL_OF_FAME_UNSCRAMBLE_AND_CHECK);
    if buffer == 0 || length < 0 {
        return o(buffer, length, checksum);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(buffer, length as usize)], || o(buffer, length, checksum),
        || hall_of_fame_unscramble_and_check(buffer, length, checksum));
    SCOREBOARD.record(equal && r == n, || format!("hall_of_fame_unscramble_and_check({}) original {} rust {}: {:?}", length, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_compare_entries`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_hall_of_fame_compare_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_HALL_OF_FAME_COMPARE_ENTRIES)(a, b);
    let n = hall_of_fame_compare_entries(a, b);
    SCOREBOARD.record(r == n, || format!("hall_of_fame_compare_entries({:#x}, {:#x}) original {} rust {}", a, b, r, n));
    r
}

// --- sx09b_1: the news item of a new ban (discipline.cpp) ---

/// The comparer's dry run of `post_ban_news`: the copy builds the news item and keeps each send instead of sending it.
pub(crate) static BAN_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
/// The sends the dry run kept: (the club, or 0 for every human manager; the news item's first `news_item::ID` bytes).
pub(crate) static BAN_NEWS_SENT: std::sync::Mutex<Vec<(usize, Vec<u8>)>> = std::sync::Mutex::new(Vec::new());

/// Sends the ban news item to a club's manager (`exe::news_send_to_club_manager`), or to every human manager when
/// `club` is null (`exe::news_send_to_all_managers`); in the dry run, keeps it.
unsafe fn ban_news_send(n: usize, club: Club) {
    if BAN_NEWS_DRY_RUN.load(Relaxed) {
        let bytes = (0..news_item::ID).map(|k| read_u8(n + k)).collect();
        BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).push((club.addr(), bytes));
        return;
    }
    if club.is_null() {
        exe::news_send_to_all_managers(read_usize(G_NEWS), n);
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
}

/// Writes value `v` into slot `*k` of news item `n` and moves to the next slot (the original's byte counter).
unsafe fn ban_news_slot(n: usize, k: &mut u8, v: u32) {
    exe::newsitem_set_slot(n, *k as i8, v);
    *k = k.wrapping_add(1);
}

/// Posts news item 2002 (0x7d2) that a person got a ban, and picks who receives it.
///
/// `post_ban_news(person, ban, fixture)` (the Discipline object in ecx is not read; SEH frame for the local news item
/// and records, no destructor). First, with the transfer manager, the clubs it picks for the player's news
/// (`exe::transfer_manager_collect_player_news_recipients` with 0xda, up to 30; none without it). The slots, in order:
/// 0 = 0 (the appeal made, set later by the appeal); the fixture's `.referee()` (−1 without a fixture); the ban's
/// `.matches()` (signed byte) and `.days()`; the start date (day signed, year unsigned) when it is still to come (today
/// `G_CURRENT_DATE` before it: an earlier year, or the same year and an earlier day), otherwise 0 and 0; the four rules
/// (`ban_record::RULES`, signed bytes); `.reason()` (signed) and `.reason_amount()`; bit 1 of `.flags()` (valid in
/// every competition), bit 2 (extended by a pending ban) and bit 0 (added to an existing ban); today's day (signed) and
/// year. Then the person's reference (`exe::fill_staff_name_ref` into a record whose date starts as `Date::Date`,
/// written by `exe::write_staff_name_ref_to_news`). With a fixture: slot 1, and the person's club in the match, the
/// `.national_job()` when the home side is a national team (`exe::database_is_national_team`), otherwise the
/// `.club_job()`; when that club is neither the home nor the away club, the routine ends there and nothing is sent;
/// otherwise its match record (`exe::fill_match_record_from_fixture`, written by `exe::write_match_record_to_news`).
/// Without a fixture: slot 0. Who gets it: a ban of more than 50 days (`.days()`, signed; a ban by matches has −1)
/// goes to every human manager (`exe::news_send_to_all_managers`) when the player's `.world_reputation()` is above
/// 8000, otherwise to the clubs picked at the start; a shorter ban only to the person's club when it has a human
/// manager (`exe::news_club_has_human_manager`).
/// Original: `Discipline::post_ban_news` (0x557280, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_ban_news(_this: usize, person: usize, ban: usize, fixture: usize) {
    let person = Staff(person);
    let ban = BanRecord(ban);
    let fixture = Fixture(fixture);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let mut recipients = [0usize; 0x1e];
    let count = if tm != 0 { exe::transfer_manager_collect_player_news_recipients(tm, person, 0xda, recipients.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x7d2, 0);
    let mut name = [0u8; 0x18]; // the person's reference: four name ids and the birth date
    name[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
    exe::fill_staff_name_ref(person, name.as_mut_ptr() as usize);
    let mut k = 0u8;
    ban_news_slot(n, &mut k, 0); // sx09b_1: the appeal made
    ban_news_slot(n, &mut k, if fixture.is_null() { u32::MAX } else { fixture.referee() as i32 as u32 });
    ban_news_slot(n, &mut k, ban.matches() as i8 as i32 as u32); // sx09b_1: the matches
    ban_news_slot(n, &mut k, ban.days() as i32 as u32);
    let today_day = read_i16(G_CURRENT_DATE);
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    let start_day = ban.start_date();
    let start_year = read_u16(ban.addr() + ban_record::START_DATE + 2); // the start date's year
    if today_year < start_year || (today_year == start_year && today_day < start_day) { // sx09b_1: the start still to come
        covered!("sx09b_1 ban news with a later start");
        ban_news_slot(n, &mut k, start_day as i32 as u32);
        ban_news_slot(n, &mut k, start_year as u32);
    } else {
        ban_news_slot(n, &mut k, 0);
        ban_news_slot(n, &mut k, 0);
    }
    for j in 0..4 {
        ban_news_slot(n, &mut k, read_i8(ban.addr() + ban_record::RULES + j) as i32 as u32);
    }
    ban_news_slot(n, &mut k, ban.reason() as i8 as i32 as u32);
    ban_news_slot(n, &mut k, ban.reason_amount() as i32 as u32);
    let flags = ban.flags() as u32;
    ban_news_slot(n, &mut k, (flags >> 1) & 1);
    ban_news_slot(n, &mut k, (flags >> 2) & 1);
    ban_news_slot(n, &mut k, flags & 1);
    ban_news_slot(n, &mut k, today_day as i32 as u32);
    ban_news_slot(n, &mut k, today_year as u32);
    exe::write_staff_name_ref_to_news(name.as_mut_ptr() as usize, n, &mut k);
    if fixture.is_null() {
        ban_news_slot(n, &mut k, 0);
    } else {
        ban_news_slot(n, &mut k, 1);
        let mut m = BanMatch([0u8; 0x20]);
        m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
        let club = if exe::database_is_national_team(fixture.home_club()) != 0 { person.national_job() } else { person.club_job() };
        if club != fixture.home_club() && club != fixture.away_club() {
            covered!("sx09b_1 ban news of a club not in the match");
            return;
        }
        exe::fill_match_record_from_fixture(fixture, club, m.0.as_mut_ptr() as usize);
        exe::write_match_record_to_news(m.0.as_mut_ptr() as usize, n, &mut k);
    }
    if ban.days() > 0x32 {
        let sheet = person.player();
        if !sheet.is_null() && sheet.world_reputation() > 0x1f40 {
            covered!("sx09b_1 ban news to every manager");
            ban_news_send(n, Club(0));
        } else {
            covered!("sx09b_1 ban news to the picked clubs");
            for &c in recipients.iter().take(count.max(0) as usize) {
                ban_news_send(n, Club(c));
            }
        }
    } else {
        let club = person.club_job();
        if !club.is_null() && exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
            covered!("sx09b_1 ban news to the club");
            ban_news_send(n, club); // sx09b_1: the club gets it
        }
    }
}

/// The human inboxes' ring positions (entry address, ring start, next slot), or none without the news object.
pub(crate) unsafe fn human_inbox_marks() -> Vec<(usize, i32, i32)> {
    let news = read_usize(G_NEWS);
    if news == 0 || read_usize(news + news_fields::HUMAN_INBOXES) == 0 {
        return Vec::new();
    }
    (0..0x10).map(|i| read_usize(news + news_fields::HUMAN_INBOXES) + i * 12).map(|e| (e, read_i32(e), read_i32(e + 4))).collect()
}

/// The news items of type `kind` that reached the human inboxes since `before` (`human_inbox_marks`): the first
/// `news_item::ID` bytes of each, without `news_item::IMPORTANT` (the delivery sets it), sorted.
pub(crate) unsafe fn delivered_news_items(before: &[(usize, i32, i32)], kind: u32) -> Vec<Vec<u8>> {
    let news = read_usize(G_NEWS);
    let mut out = Vec::new();
    for &(e, start, next) in before {
        let (start_now, next_now) = (read_i32(e), read_i32(e + 4));
        if start_now == -1 || (start_now == start && next_now == next) {
            continue;
        }
        let mut k = if start_now == start { next } else { start_now };
        while k != next_now {
            let r = read_usize(news).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
            if read_u32(r) == kind {
                let mut bytes: Vec<u8> = (0..news_item::ID).map(|j| read_u8(r + j)).collect();
                bytes[news_item::IMPORTANT] = 0;
                out.push(bytes);
            }
            k += 1;
            if k == start_now + 0x64 {
                k = start_now;
            }
        }
    }
    out.sort();
    out
}

/// Comparer for `post_ban_news`, as a dry run: the copy runs first without sending (it keeps each send), then the
/// original from the same random generator state. Compares the generator's state and the ban news items that reached
/// the human managers' inboxes with the copy's sends: one per club with a human manager, and for the send to every
/// manager one per human manager (the last 16 people with `.classification()` 1, as `News::send_to_all_managers`).
pub(crate) unsafe extern "thiscall" fn compare_post_ban_news(this: usize, person: usize, ban: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_POST_BAN_NEWS);
    let inboxes = human_inbox_marks();
    let rng = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).clear();
    BAN_NEWS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| post_ban_news(this, person, ban, fixture)));
    BAN_NEWS_DRY_RUN.store(false, Relaxed);
    let rng_rust = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let sent = core::mem::take(&mut *BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()));
    rng.write_back();
    o(this, person, ban, fixture);
    let rng_original = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let staff_count = read_i32(G_STAFF_COUNT);
    let humans = (0..0x10).filter(|&i| staff_at((staff_count - 0x10 + i) as isize).classification() == 1).count();
    let mut expected: Vec<Vec<u8>> = Vec::new();
    for (c, b) in &sent {
        let mut b = b.clone();
        b[news_item::IMPORTANT] = 0;
        let copies = if *c == 0 {
            humans
        } else {
            let m = Club(*c).manager();
            (!m.is_null() && read_i32(m.addr()) >= staff_count.wrapping_sub(0x10)) as usize
        };
        expected.extend(core::iter::repeat(b).take(copies));
    }
    expected.sort();
    let delivered = delivered_news_items(&inboxes, 0x7d2);
    let same = rng_original == rng_rust && (inboxes.is_empty() || delivered == expected);
    SCOREBOARD.record(same, || {
        format!(
            "post_ban_news(person {:#x}, ban {:#x}, fixture {:#x}) rng same {}; {} sends, human items original {:x?} rust {:x?}",
            person,
            ban,
            fixture,
            rng_original == rng_rust,
            sent.len(),
            delivered,
            expected
        )
    });
}

// --- sx09b_2: the discipline nation index order and the clubs' fine counts (discipline.cpp, Fine.cpp) ---

/// Orders two items of the discipline nation index (`discipline_fields::NATION_INDEX`, 8 bytes: +0 the person id,
/// `nation_index_item::NATION_ID`), for qsort and bsearch: by the nation id (the difference, when they differ), then
/// by the person id; a person id of −1 on either side (the key that `nation_index_range` searches with) matches any
/// person of the nation (0).
/// Original: `compare_nation_index_entries` (0x55e5f0, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_nation_index_entries(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (nation_a, nation_b) = (read_i32(a + nation_index_item::NATION_ID), read_i32(b + nation_index_item::NATION_ID));
    if nation_a != nation_b {
        return nation_a.wrapping_sub(nation_b);
    }
    let (id_a, id_b) = (read_i32(a), read_i32(b));
    if id_a == -1 || id_b == -1 { // sx09b_2: the key matches any person
        covered!("sx09b_2 nation index key");
        return 0;
    }
    id_a.wrapping_sub(id_b)
}

/// The club's fine count: the byte of the club (by its id) in the Fine object's per-club table (the pointer at +8).
unsafe fn club_fine_count(this: usize, club: usize) -> usize {
    read_usize(this + 8).wrapping_add(read_usize(club)) // +8: the per-club fine count table
}

/// Halves the club's count of player fines (at the new season): the signed byte × 0.5 (`rdata::F64_0_5`), truncated
/// (`ftol`). A null club: nothing.
/// Original: `Fine::halve_club_fine_count` (0x5a5930, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn halve_club_fine_count(this: usize, club: usize) {
    if club == 0 {
        return;
    }
    let e = club_fine_count(this, club);
    write_u8(e, ftol(read_i8(e) as f64 * read_f64(rdata::F64_0_5)) as u8);
}

/// Clears the club's count of player fines (when its manager leaves). A null club: nothing.
/// Original: `Fine::clear_club_fine_count` (0x5a5960, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_club_fine_count(this: usize, club: usize) {
    if club == 0 {
        return;
    }
    write_u8(club_fine_count(this, club), 0);
}

compare_order!(compare_compare_nation_index_entries, compare_nation_index_entries, IDX_COMPARE_NATION_INDEX_ENTRIES);

/// Comparer for `halve_club_fine_count`: side by side over the club's byte.
pub(crate) unsafe extern "thiscall" fn compare_halve_club_fine_count(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HALVE_CLUB_FINE_COUNT);
    if club == 0 {
        return o(this, club);
    }
    let e = club_fine_count(this, club);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e, 1)], || o(this, club), || halve_club_fine_count(this, club));
    SCOREBOARD.record(same, || format!("halve_club_fine_count(club {:#x}) original={:x?} rust={:x?}", club, mo, mr));
}

/// Comparer for `clear_club_fine_count`: side by side over the club's byte.
pub(crate) unsafe extern "thiscall" fn compare_clear_club_fine_count(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_CLUB_FINE_COUNT);
    if club == 0 {
        return o(this, club);
    }
    let e = club_fine_count(this, club);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e, 1)], || o(this, club), || clear_club_fine_count(this, club));
    SCOREBOARD.record(same, || format!("clear_club_fine_count(club {:#x}) original={:x?} rust={:x?}", club, mo, mr));
}

// --- sx09b_3: the scope text of a ban (discipline.cpp) ---

/// Writes which ruling bodies a ban covers and returns the grammatical form of the text (the `{}` codes of the
/// templates).
///
/// `get_scope_text(rules, out, size, everywhere, form)`: `rules` are the ban's four rules (`ban_record::RULES`, one per
/// person record slot; 0xff none). A `size` below 70 gives the "Error" box (line 0x1475) and returns 0xff. Three flags
/// start set: world-wide, continental and domestic; the first rule present (in slot order) is the one that speaks.
/// Slot 0 or slot 1 missing clears all three; slot 2 present clears domestic, missing clears world-wide and
/// continental. With slot 3 (the FIFA's) present: `everywhere` set gives "global" and returns 0xff; otherwise "world-wide"
/// when that flag survived, else the speaking rule's own text. Without slot 3: domestic and not continental gives
/// "domestic"; continental gives "continental"; neither, the speaking rule's own text (its vtable slot 0x10 with `out`,
/// `size` and `form`). The form returned is the speaking rule's: by its record slot (`ruling_body::RECORD_SLOT`) 1 →
/// 9, 3 → 1, 0 → 8 when the byte +0x20 is not 0xff (else 0); otherwise by its index (`ruling_body::RULE_SET_INDEX`)
/// 1 → 2, 2 → 6, 35 → 7, 41 → 3, any other → 0xff.
/// Original: `Discipline::get_scope_text` (0x55ce40, discipline.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_scope_text(this: usize, rules: usize, out: usize, size: i32, everywhere: u32, form: u32) -> u8 {
    if size < 0x46 {
        show_error_at(0x9b8f18, 0x1475);
        return 0xff;
    }
    let rule = |k: usize| read_u8(rules + k);
    let (mut world, mut continental, mut domestic) = (true, true, true);
    let mut speaker = 0xffu8;
    for k in 0..2 {
        if rule(k) == 0xff {
            (world, continental, domestic) = (false, false, false);
        } else if speaker == 0xff {
            speaker = rule(k);
        }
    }
    if rule(2) != 0xff {
        if speaker == 0xff {
            speaker = rule(2);
        }
        domestic = false;
    } else {
        (world, continental) = (false, false);
    }
    let body = |s: u8| read_usize(read_usize(this).wrapping_add((s as i8 as isize as usize).wrapping_mul(4)));
    let own_text = |s: u8| {
        let f: unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(body(s)) + 0x10)); // vtable +0x10: the rule's scope text
        f(body(s), out, size, form);
    };
    if rule(3) != 0xff {
        if everywhere != 0 {
            covered!("sx09b_3 global");
            story_lang!(out, 0x9c3478); // "global"
            return 0xff;
        }
        if speaker == 0xff {
            speaker = rule(3);
        }
        if world {
            covered!("sx09b_3 world-wide");
            story_lang!(out, 0x9c346c); // "world-wide"
        } else {
            own_text(speaker);
        }
    } else if domestic && !continental {
        covered!("sx09b_3 domestic");
        story_lang!(out, 0x9c3454); // "domestic"
    } else if continental {
        covered!("sx09b_3 continental");
        story_lang!(out, 0x9c3460); // "continental"
    } else {
        covered!("sx09b_3 the rule's own text");
        own_text(speaker);
    }
    let b = body(speaker);
    match read_u8(b + ruling_body::RECORD_SLOT) {
        1 => 9,
        0 => if read_u8(b + 0x20) != 0xff { 8 } else { 0 }, // sx09b_3: +0x20 of the ruling body (unnamed byte)
        3 => 1,
        _ => match read_u8(b + ruling_body::RULE_SET_INDEX) as i8 {
            1 => 2,
            2 => 6,
            35 => 7,
            41 => 3,
            _ => 0xff,
        },
    }
}

/// Comparer for `get_scope_text`: side by side over the output buffer and the form returned. A short buffer (the
/// "Error" box) runs the original only.
pub(crate) unsafe extern "thiscall" fn compare_get_scope_text(this: usize, rules: usize, out: usize, size: i32, everywhere: u32, form: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32) -> u8>(IDX_GET_SCOPE_TEXT);
    if size < 0x46 {
        return o(this, rules, out, size, everywhere, form);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, size as usize)], || o(this, rules, out, size, everywhere, form), || get_scope_text(this, rules, out, size, everywhere, form));
    SCOREBOARD.record(same && r == n, || {
        format!(
            "get_scope_text(rules {:02x?}, everywhere {}, form {}) original={} rust={} {:?}",
            (0..4).map(|k| read_u8(rules + k)).collect::<Vec<_>>(),
            everywhere,
            form,
            r,
            n,
            first_differences(&mo, &mr)
        )
    });
    r
}

// --- sx09b_5: the contract disputes before the Player Status Committee (dispute.cpp) ---

/// `dispute_post_news` (0x56a3b0, dispute.cpp; thiscall(dispute, person, club, outcome, a, b), ret 0x14).
const FN_DISPUTE_POST_NEWS: usize = 0x56a3b0;
/// `club_remove_favoured_staff` (0x547bd0; cdecl(club, person)): clears the person from the club's favoured slots.
const FN_CLUB_REMOVE_FAVOURED_STAFF: usize = 0x547bd0;

/// The person's byte in the dispute flag table (the pointer at +4 of the dispute object, one byte per person id).
unsafe fn dispute_flag(this: usize, person: usize) -> usize {
    read_usize(this + 4).wrapping_add(read_usize(person)) // +4: the per-person dispute flags
}

/// Returns the person's dispute flag, dropping the dispute when it no longer stands.
///
/// `is_in_dispute(person, flag_only)`: no person, or an id at or above `G_STAFF_COUNT`: 0. A flag set while the FIFA
/// transfer rules are off (`G_FIFA_TRANSFER_RULES_IN_FORCE` 0): the dispute is dropped (`remove_dispute`) and 0. With
/// `flag_only`, the flag. Otherwise, with the flag set, the person's contract (`exe::contract_manager_get_contract`,
/// kept in `G_CONTRACT_LOOKUP_RESULT`): none, or its club's record address null, drops the dispute and gives 0; else
/// the flag.
/// Original: `DISPUTE::is_in_dispute` (0x569e30, dispute.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn is_in_dispute(this: usize, person: usize, flag_only: u32) -> u8 {
    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let flag = read_u8(dispute_flag(this, person));
    if flag != 0 && read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 0 {
        covered!("sx09b_5 dispute dropped without the FIFA rules");
        remove_dispute(this, person);
        return 0;
    }
    if flag_only != 0 || flag == 0 {
        return flag;
    }
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if c.is_null() || read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE)) == 0 {
        covered!("sx09b_5 dispute dropped without a contract");
        remove_dispute(this, person);
        return 0;
    }
    read_u8(dispute_flag(this, person))
}

/// Returns the club id of the person's dispute entry, or −1.
///
/// `get_dispute_club_id(person)`: no person, no flag table, the person's flag clear or no dispute list: −1.
/// Otherwise, from the start of the list (the cursor at the head), the first of the first `count` 17-byte entries with
/// the person's id (+0) gives its club id (+4); none: −1.
/// Original: `DISPUTE::get_dispute_club_id` (0x56a030, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_dispute_club_id(this: usize, person: usize) -> i32 {
    if person == 0 || read_usize(this + 4) == 0 || read_u8(dispute_flag(this, person)) == 0 || read_usize(this) == 0 {
        return -1;
    }
    let list = Llist(read_usize(this));
    list.set_cursor(list.head());
    let mut k = 0i32;
    while k < list.count() as i32 {
        let e = exe::llist_get_next(list);
        if e == 0 {
            return -1;
        }
        if read_i32(e) == read_i32(person) {
            covered!("sx09b_5 dispute club found");
            return read_i32(e + 4); // +4: the club id of the dispute
        }
        k += 1;
    }
    -1
}

/// Resolves every dispute whose hearing date has passed.
///
/// `process_hearings()` (daily): with the dispute list, from its head, for each of the first `count` 17-byte entries
/// (re-read after each step, so a removal skips the next one, as in the original) whose hearing date (+8) is before
/// today (`exe::date_operator_3c` with `G_CURRENT_DATE`) and whose person id (+0) is between 0 and `G_STAFF_COUNT` − 2:
/// when the person's `.club_job()` is the dispute's club (+4): the byte +0x10 set keeps him (the news with outcome 1);
/// otherwise he leaves on a free transfer (`exe::transfer_manager_queue_free_transfer` with 0 and −1), the club's
/// manager stops favouring him (`exe::staff_is_favoured_by` → `exe::staff_remove_favoured_staff`), the club stops
/// favouring him (`exe::club_favours_staff` → `FN_CLUB_REMOVE_FAVOURED_STAFF`), the manager and the club dislike him
/// (`exe::staff_add_disliked_staff`, `exe::club_add_disliked_staff`, both with 1) and the news with outcome 2
/// (`FN_DISPUTE_POST_NEWS`). Then (whatever the club) the person's flag goes to 0 and the entry leaves the list
/// (`exe::llist_remove`).
/// Original: `DISPUTE::process_hearings` (0x569ed0, dispute.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn process_hearings(this: usize) {
    if read_usize(this) == 0 {
        return;
    }
    let list = Llist(read_usize(this));
    list.set_cursor(list.head());
    if list.count() <= 0 {
        return;
    }
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_DISPUTE_POST_NEWS);
    let mut k = 0i32;
    loop {
        let e = exe::llist_get_next(list);
        if e == 0 {
            return;
        }
        let id = read_i32(e);
        if exe::date_operator_3c(e + 8, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0 && id > -1 && id < read_i32(G_STAFF_COUNT) - 1 {
            let person = staff_at(id as isize);
            let club = person.club_job();
            if !club.is_null() && read_i32(club.addr()) == read_i32(e + 4) {
                if read_u8(e + 0x10) != 0 { // +0x10: the player stays at his club
                    covered!("sx09b_5 hearing keeps the player");
                    post(this, person.addr(), club.addr(), 1, 0, 0);
                } else {
                    covered!("sx09b_5 hearing frees the player");
                    exe::transfer_manager_queue_free_transfer(read_usize(G_TRANSFER_MANAGER), person, 0, u32::MAX);
                    let manager = club.manager();
                    if !manager.is_null() {
                        if exe::staff_is_favoured_by(person, manager) != 0 {
                            exe::staff_remove_favoured_staff(person, manager);
                        }
                        if exe::club_favours_staff(person, club) != 0 {
                            let unfavour: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_CLUB_REMOVE_FAVOURED_STAFF);
                            unfavour(club.addr(), person.addr());
                        }
                        exe::staff_add_disliked_staff(person, manager, 1);
                    }
                    exe::club_add_disliked_staff(club, person, 1);
                    post(this, person.addr(), club.addr(), 2, 0, 0); // sx09b_5: the outcome of a freed player
                }
            }
            write_u8(read_usize(this + 4).wrapping_add(read_usize(e)), 0);
            exe::llist_remove(list, e);
        }
        k += 1;
        if k >= read_i16(read_usize(this)) as i32 {
            return;
        }
    }
}

/// Comparer for `is_in_dispute`: side by side over the contract lookup result and the flag returned; a dispute that
/// would be dropped (the list changes) runs the original only, and the flag must end at 0 with 0 returned.
pub(crate) unsafe extern "thiscall" fn compare_is_in_dispute(this: usize, person: usize, flag_only: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_IS_IN_DISPUTE);
    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) || read_u8(dispute_flag(this, person)) == 0 {
        let r = o(this, person, flag_only);
        SCOREBOARD.record(r == 0, || format!("is_in_dispute(person {:#x}) without a dispute returned {}", person, r));
        return r;
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 0 || flag_only == 0 {
        let flag = read_u8(dispute_flag(this, person));
        let r = o(this, person, flag_only);
        let after = read_u8(dispute_flag(this, person));
        SCOREBOARD.record((after == 0 && r == 0) || (r == flag && after == flag), || format!("is_in_dispute(person {:#x}, flag {}) returned {}, flag after {}", person, flag, r, after));
        return r;
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, person, flag_only), || is_in_dispute(this, person, flag_only));
    SCOREBOARD.record(same && r == n, || format!("is_in_dispute(person {:#x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `get_dispute_club_id`: side by side over the list cursor and the result.
pub(crate) unsafe extern "thiscall" fn compare_get_dispute_club_id(this: usize, person: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_GET_DISPUTE_CLUB_ID);
    let list = read_usize(this);
    let regions: &[(usize, usize)] = if list == 0 { &[] } else { &[(list + llist::CURSOR, 4)] };
    let (r, n, same, _, _) = crate::comparison::side_by_side_light(regions, || o(this, person), || get_dispute_club_id(this, person));
    SCOREBOARD.record(same && r == n, || format!("get_dispute_club_id(person {:#x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `process_hearings`: frees players and posts news, so only the original runs; the plan is checked
/// afterwards: the entries the walk resolves (due, with a valid person id; the walk stops early after removals, as in the
/// original) have the flag cleared and left the list.
pub(crate) unsafe extern "thiscall" fn compare_process_hearings(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_HEARINGS);
    let list = read_usize(this);
    if list == 0 || read_i16(list) <= 0 {
        return o(this);
    }
    // the copy's walk over the nodes, without changing anything: the ids of the entries it resolves
    let mut nodes = Vec::new();
    let mut node = read_usize(list + llist::HEAD);
    while node != 0 {
        nodes.push(read_usize(node + llist_node::DATA));
        node = read_usize(node + llist_node::NEXT);
    }
    let due = |e: usize| {
        let id = read_i32(e);
        exe::date_operator_3c(e + 8, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0 && id > -1 && id < read_i32(G_STAFF_COUNT) - 1
    };
    let (mut count, mut resolved) = (nodes.len() as i32, Vec::new());
    for (k, &e) in nodes.iter().enumerate() {
        if due(e) {
            resolved.push(read_usize(e));
            count -= 1;
        }
        if k as i32 + 1 >= count {
            break;
        }
    }
    o(this);
    let flags = read_usize(this + 4);
    let wrong: Vec<usize> = resolved.iter().copied().filter(|&id| read_u8(flags.wrapping_add(id)) != 0).collect();
    let left = if read_usize(this) == 0 { 0 } else { read_i16(read_usize(this)) as i32 };
    SCOREBOARD.record(wrong.is_empty() && left == count, || format!("process_hearings: resolved {:?}, flags still set {:?}, entries left {} (expected {})", resolved, wrong, left, count));
}

// --- sx09b_6: the news item of a contract dispute (dispute.cpp) ---

/// The comparer's dry run of `dispute_post_news`: the copy builds the news item and keeps it and the number of sends,
/// instead of sending it.
pub(crate) static DISPUTE_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
/// What the dry run kept: the news item's first `news_item::ID` bytes and how many sends it would make.
pub(crate) static DISPUTE_NEWS_BUILT: std::sync::Mutex<(Vec<u8>, usize)> = std::sync::Mutex::new((Vec::new(), 0));

/// Sends the dispute news item to every human manager (`nation` null: `exe::news_send_to_all_managers`) or to the
/// managers of a nation (`exe::news_send_to_nation_managers`); in the dry run, counts the send.
unsafe fn dispute_news_send(n: usize, nation: Option<usize>) {
    if DISPUTE_NEWS_DRY_RUN.load(Relaxed) {
        let mut built = DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner());
        built.0 = (0..news_item::ID).map(|k| read_u8(n + k)).collect();
        built.1 += 1;
        return;
    }
    match nation {
        None => {
            exe::news_send_to_all_managers(read_usize(G_NEWS), n);
        }
        Some(x) => {
            exe::news_send_to_nation_managers(read_usize(G_NEWS), n, x);
        }
    }
}

/// Posts news item 3049 (0xbe9) about a player's contract dispute with a club.
///
/// `dispute_post_news(person, club, outcome, a, b)` (the dispute object in ecx is not read; SEH frame for the local
/// news item, no destructor). Without a person, his `.club_job()`, his `.player()` or `club`: nothing. Slots 0-3 the
/// person and his name ids (`set_news_person_slots`), 4 the club's id, 5 his `.club_job()`'s id, 6 `a` and 7 `b`
/// (signed bytes), 8 the id of his club's nation (−1 without one), 9 `outcome` (signed byte: 1 kept at the club, 2
/// freed, from `process_hearings`). A player whose `.world_reputation()` is above 7500 is news to every human manager;
/// otherwise to the managers of his club's nation (when it has one) and, when his own `.nation()` is another, also to
/// those of his nation.
/// Original: `dispute_post_news` (0x56a3b0, dispute.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn dispute_post_news(_this: usize, person: usize, club: usize, outcome: u32, a: u32, b: u32) {
    let person = Staff(person);
    if person.is_null() || person.club_job().is_null() || person.player().is_null() || club == 0 {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbe9, 0);
    set_news_person_slots(n, 0, person);
    set_news_slot(n, 4, read_u32(club));
    let own = person.club_job();
    set_news_slot(n, 5, if own.is_null() { u32::MAX } else { read_u32(own.addr()) });
    set_news_slot(n, 6, a as i8 as i32 as u32);
    set_news_slot(n, 7, b as i8 as i32 as u32);
    let club_nation = if own.is_null() { 0 } else { own.nation().addr() };
    set_news_slot(n, 8, if club_nation == 0 { u32::MAX } else { read_u32(club_nation) });
    set_news_slot(n, 9, outcome as i8 as i32 as u32); // sx09b_6: the outcome
    if person.player().world_reputation() > 0x1d4c {
        covered!("sx09b_6 dispute news to every manager");
        dispute_news_send(n, None);
        return;
    }
    if club_nation != 0 {
        covered!("sx09b_6 dispute news to the club's nation");
        dispute_news_send(n, Some(club_nation));
    }
    if person.club_job().nation().addr() != person.nation().addr() {
        covered!("sx09b_6 dispute news to the player's nation");
        dispute_news_send(n, Some(person.nation().addr()));
    }
}

/// Comparer for `dispute_post_news`, as a dry run: the copy builds the news item without sending it, then the original
/// runs. Every dispute news item (type 0xbe9) that reached a human inbox must be the copy's, and with no send planned
/// none may arrive.
pub(crate) unsafe extern "thiscall" fn compare_dispute_post_news(this: usize, person: usize, club: usize, outcome: u32, a: u32, b: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_DISPUTE_POST_NEWS);
    let inboxes = human_inbox_marks();
    *DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner()) = (Vec::new(), 0);
    DISPUTE_NEWS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| dispute_post_news(this, person, club, outcome, a, b)));
    DISPUTE_NEWS_DRY_RUN.store(false, Relaxed);
    let (mut built, sends) = core::mem::take(&mut *DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner()));
    o(this, person, club, outcome, a, b);
    if !built.is_empty() {
        built[news_item::IMPORTANT] = 0;
    }
    let delivered = delivered_news_items(&inboxes, 0xbe9);
    let same = inboxes.is_empty() || if sends == 0 { delivered.is_empty() } else { delivered.iter().all(|d| *d == built) };
    SCOREBOARD.record(same, || format!("dispute_post_news(person {:#x}, outcome {}) {} sends; human items original {:x?} rust {:x?}", person, outcome as i8, sends, delivered, built));
}

// --- sx09b_8: the text of the board's warning about fining players too often (Fine.cpp) ---

/// Writes the headline or the body of news item 6020 (0x1784): the board's warning about a manager who fines his
/// players too often.
///
/// `write_board_fine_warning_news_text(news, full, out)`: the headline "The board make concerns known" (0x9cba54); the
/// body by the news item's slot 0 (`exe::newsitem_get_slot`): set, the first warning ("The board are concerned that
/// your frequent fining of players will damage the squads morale...", 0x9cbb3c), clear, the last one ("...we may be
/// forced to look for a new manager", 0x9cba74). Formatted into `NEWS_TEXT_BUFFER` and set as the text of `out`
/// (`exe::text_set_string`).
/// Original: `write_board_fine_warning_news_text` (0x5a7ed0, Fine.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_board_fine_warning_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let template = if full == 0 {
        0x9cba54 // "The board make concerns known"
    } else if exe::newsitem_get_slot(news, 0) != 0 { // sx09b_8: the first warning
        covered!("sx09b_8 board fine warning, the first");
        0x9cbb3c
    } else {
        covered!("sx09b_8 board fine warning, the last");
        0x9cba74
    };
    story_lang!(NEWS_TEXT_BUFFER, template);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
}

/// Comparer for `write_board_fine_warning_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_board_fine_warning_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_BOARD_FINE_WARNING_NEWS_TEXT);
    let what = || format!("write_board_fine_warning_news_text(news {:#x}, full {})", news, full);
    compare_text_writer(out, || o(this, news, full, out), |t| write_board_fine_warning_news_text(this, news, full, t), what);
}

// ---------- sx05b_5: the team award, year award and year ratings constructors; the award builders the nations share ----------

/// The award base fields every award class shares (`AWARD`, award.cpp): +4 the award type byte (0 team, 1 year, 2
/// month), +0xd the nation id and +0x11 the award's index in its nation (the news slots 0 and 1); the rest is
/// `award_fields`.
pub(crate) mod award_base {
    pub(crate) const TYPE: usize = 0x4;
    pub(crate) const STAFF_COMP: usize = 0x5;
    pub(crate) const CLUB_COMP: usize = 0x9;
    pub(crate) const NATION_ID: usize = 0xd;
    pub(crate) const INDEX: usize = 0x11;
}

/// The year award (`YEAR_AWARD`, 0x2b bytes): the record kind byte Staff_Records keeps (+0x1d), a second option byte
/// (+0x1e) [inference], the owned shortlist (+0x1f, or 0) and its first and last record offsets in the award manager's
/// year history (+0x23, +0x27; −1 none).
pub(crate) mod year_award_obj {
    pub(crate) const SIZE: u32 = 0x2b;
    pub(crate) const VTABLE: usize = 0x9713b8;
    pub(crate) const KIND: usize = 0x1d;
    pub(crate) const OPTION: usize = 0x1e;
    pub(crate) const SHORTLIST: usize = 0x1f;
    pub(crate) const HISTORY_FIRST: usize = 0x23;
    pub(crate) const HISTORY_LAST: usize = 0x27;
}

/// The team award (`TEAM_AWARD`, 0x25d bytes; the selection itself is `TeamAward`): the update date (+0x1d), the
/// season span in years (+0x25), the period start and end dates (+0x242, +0x24a, inside the 0x205-byte record from
/// +0x4e the history keeps; the filled count +0x252 is `TeamAward::filled`), the substitutes count (+0x253), the
/// history offsets (+0x254, +0x258) and the formation mode (+0x25c).
pub(crate) mod team_award_obj {
    pub(crate) const SIZE: u32 = 0x25d;
    pub(crate) const VTABLE: usize = 0x9703f0;
    pub(crate) const UPDATE_DATE: usize = 0x1d;
    pub(crate) const SEASON_SPAN: usize = 0x25;
    pub(crate) const RECORD: usize = 0x4e;
    pub(crate) const RECORD_SIZE: u32 = 0x205;
    pub(crate) const PERIOD_START: usize = 0x242;
    pub(crate) const PERIOD_END: usize = 0x24a;
    pub(crate) const SUBSTITUTES: usize = 0x253;
    pub(crate) const HISTORY_FIRST: usize = 0x254;
    pub(crate) const HISTORY_LAST: usize = 0x258;
    pub(crate) const FORMATION_MODE: usize = 0x25c;
}

/// `AWARD::AWARD(nation, index, staff comp, national rep, world rep, club comp, file, for players)` (0x412e40,
/// thiscall, ret 0x20): the base fields (`award_base`, `award_fields`).
const FN_AWARD_AWARD: usize = 0x412e40;
/// `YEAR_AWARD::load_from_disk(scope, file)` (0x92fff0, thiscall, ret 8): 1 when read.
const FN_YEAR_AWARD_LOAD_FROM_DISK: usize = 0x92fff0;
/// `TEAM_AWARD::load_from_disk(file)` (0x8af660, thiscall, ret 4).
const FN_TEAM_AWARD_LOAD_FROM_DISK: usize = 0x8af660;
/// `YEAR_RATINGS::load_from_disk(file)` (0x931390, thiscall, ret 4).
const FN_YEAR_RATINGS_LOAD_FROM_DISK: usize = 0x931390;
/// The year award constructor (0x92fe80, thiscall, ret 0x2c) and the team award one (0x8af520, ret 0x28), as the
/// nations' `init_awards` call them (through the hooks).
const FN_YEAR_AWARD_CONSTRUCT: usize = 0x92fe80;
const FN_TEAM_AWARD_CONSTRUCT: usize = 0x8af520;
/// `MONTH_AWARD::MONTH_AWARD` (0x766e10, thiscall, ret 0x30; 12 arguments), 0xa2 bytes.
const FN_MONTH_AWARD_CONSTRUCT: usize = 0x766e10;
const MONTH_AWARD_SIZE: u32 = 0xa2;

/// Runs the award base constructor with the eight arguments the class constructors pass it.
unsafe fn award_construct_base(this: usize, a: [u32; 8]) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_AWARD_AWARD);
    f(this, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]);
}

/// Builds a team award: the award base (nation, index, staff comp, national and world reputations, club comp, the
/// file and the player flag), three empty dates (the update date and the period's), the season span (`span`), the
/// formation mode (`mode`), type 0, three substitutes, no history (−1, −1), the selection cleared
/// (`team_award_clear_selection`) and the slot masks (`team_award_init_slot_positions`); with a file, then
/// `TEAM_AWARD::load_from_disk(file)` (its result not read). Returns this (the original builds the dates under an SEH
/// frame; nothing here throws).
/// Original: `TEAM_AWARD::TEAM_AWARD` (0x8af520, team_award.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn team_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, span: u32, mode: u32, file: usize, for_players: u32) -> usize {
    use team_award_obj as ta;
    award_construct_base(this, [nation, index, staff_comp, national_rep, world_rep, club_comp, file as u32, for_players]);
    exe::date_date_54a110(this + ta::UPDATE_DATE);
    exe::date_date_54a110(this + ta::PERIOD_START);
    exe::date_date_54a110(this + ta::PERIOD_END);
    write_u8(this + ta::SEASON_SPAN, span as u8);
    write_u8(this + ta::SUBSTITUTES, 3);
    write_u8(this + ta::FORMATION_MODE, mode as u8);
    write_usize(this, ta::VTABLE);
    write_u8(this + award_base::TYPE, 0);
    write_i32(this + ta::HISTORY_FIRST, -1);
    write_i32(this + ta::HISTORY_LAST, -1);
    team_award_clear_selection(this);
    team_award_init_slot_positions(this);
    if file != 0 {
        covered!("sx05b_5 team award loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TEAM_AWARD_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// Builds a year award: the award base (with the file and the player flag as its last two arguments), type 1, the
/// kind (`kind`) and option (`option`) bytes, no shortlist and no history (−1, −1). With a file,
/// `YEAR_AWARD::load_from_disk(scope, file)`; when there is none or the load fails, a scope gets a new shortlist
/// (`construct_award_shortlist`(scope, no file) in 0x40 bytes of `exe::crt_operator_new`; 0 when out of memory).
/// Returns this.
/// Original: `YEAR_AWARD::YEAR_AWARD` (0x92fe80, year_award.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn year_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, kind: u32, scope: usize, option: u32, file: usize, for_players: u32) -> usize {
    use year_award_obj as ya;
    award_construct_base(this, [nation, index, staff_comp, national_rep, world_rep, club_comp, file as u32, for_players]);
    write_usize(this, ya::VTABLE);
    write_usize(this + ya::SHORTLIST, 0);
    write_u8(this + ya::KIND, kind as u8);
    write_u8(this + ya::OPTION, option as u8);
    write_i32(this + ya::HISTORY_FIRST, -1);
    write_i32(this + ya::HISTORY_LAST, -1);
    write_u8(this + award_base::TYPE, 1);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_YEAR_AWARD_LOAD_FROM_DISK);
        if load(this, scope, file) != 0 {
            return this;
        }
    }
    if scope != 0 {
        covered!("sx05b_5 year award shortlist");
        let p = exe::crt_operator_new(0x40);
        write_usize(this + ya::SHORTLIST, if p != 0 { construct_award_shortlist(p, scope, 0) } else { 0 });
    }
    this
}

/// Builds an empty year ratings table for `year` (no entries, count and capacity 0, the year word at +0xc) and, with
/// a file, loads it (`YEAR_RATINGS::load_from_disk`, its result not read). Returns this.
/// Original: `YEAR_RATINGS::YEAR_RATINGS` (0x931360, year_ratings.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn year_ratings_construct(this: usize, year: u32, file: usize) -> usize {
    write_u16(this + 0xc, year as u16);
    write_usize(this, 0);
    write_u32(this + 4, 0);
    write_u32(this + 8, 0);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_YEAR_RATINGS_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// One award a nation builds in its `init_awards`: the constructor's arguments in order (the nation id and the
/// award's index first, the file second to last).
pub(crate) enum AwardSpec {
    Year([u32; 11]),
    Team([u32; 10]),
    Month([u32; 12]),
}

/// A new award object from its spec (`exe::crt_operator_new` of the class size, then its constructor through the
/// game's address); 0 when out of memory, as the original leaves it.
pub(crate) unsafe fn award_new(spec: &AwardSpec) -> usize {
    let size = match spec {
        AwardSpec::Year(_) => year_award_obj::SIZE,
        AwardSpec::Team(_) => team_award_obj::SIZE,
        AwardSpec::Month(_) => MONTH_AWARD_SIZE,
    };
    let p = exe::crt_operator_new(size);
    if p == 0 {
        return 0;
    }
    match spec {
        AwardSpec::Year(a) => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_YEAR_AWARD_CONSTRUCT);
            f(p, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10])
        }
        AwardSpec::Team(a) => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TEAM_AWARD_CONSTRUCT);
            f(p, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9])
        }
        AwardSpec::Month(a) => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_MONTH_AWARD_CONSTRUCT);
            f(p, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11])
        }
    }
}

/// A nation's awards object (`NationAwards`): the vtable, the award table (+4) and count (+8, i16), the month ratings
/// (+0xa), the type byte (+0x12), the nation record (+0xe) and the nation id the awards carry (+0x13).
pub(crate) mod nation_awards_fields {
    pub(crate) const TABLE: usize = 0x4;
    pub(crate) const COUNT: usize = 0x8;
    pub(crate) const MONTH_RATINGS: usize = 0xa;
    pub(crate) const NATION: usize = 0xe;
    pub(crate) const TYPE: usize = 0x12;
    pub(crate) const NATION_ID: usize = 0x13;
}

/// `NationAwards::NationAwards` (0x768290, thiscall), `NationAwards::load_from_disk(file)` (0x76a280, thiscall, ret
/// 4: nonzero when read) and `MONTH_RATINGS::MONTH_RATINGS(file)` (0x767af0, thiscall, ret 4; 0xf bytes).
const FN_NATION_AWARDS_CONSTRUCT: usize = 0x768290;
const FN_NATION_AWARDS_LOAD_FROM_DISK: usize = 0x76a280;
const FN_MONTH_RATINGS_CONSTRUCT: usize = 0x767af0;

/// Fills a nation's award table: the count, the table (4 bytes an award, `exe::crt_malloc`; out of memory the error box
/// of `cpp` at `line` and nothing more) and each award built from its spec in order.
pub(crate) unsafe fn nation_awards_build(this: usize, specs: &[AwardSpec], cpp: usize, line: u32) {
    write_i16(this + nation_awards_fields::COUNT, specs.len() as i16);
    let table = exe::crt_malloc(4 * specs.len() as u32);
    write_usize(this + nation_awards_fields::TABLE, table);
    if table == 0 {
        show_error_at(cpp, line);
        return;
    }
    for (k, spec) in specs.iter().enumerate() {
        write_usize(table + 4 * k, award_new(spec));
    }
}

/// How a nation's awards object starts: its vtable, the key of its nation (none for the World Cup), the nation id, the
/// type byte, and the month ratings it needs (with the error line when they cannot be built).
pub(crate) struct NationAwardsClass {
    pub(crate) vtable: usize,
    pub(crate) nation_key: Option<usize>,
    pub(crate) nation_id: u32,
    pub(crate) kind: u8,
    pub(crate) month_ratings: Option<(usize, u32)>,
}

/// The shared constructor of the nations' awards: the `NationAwards` base, the class's vtable, the nation record
/// (`G_NATION_TABLE` at the nation key's index, or 0) and id, the type byte; with a file, the saved awards
/// (`NationAwards::load_from_disk`) and nothing more when they load. Otherwise, a class with month ratings builds them
/// (0xf bytes, `MONTH_RATINGS::MONTH_RATINGS(0)`; out of memory, the box and no awards) and then `init(this, file)`.
pub(crate) unsafe fn nation_awards_construct(this: usize, class: &NationAwardsClass, file: usize, init: unsafe extern "thiscall" fn(usize, usize)) -> usize {
    use nation_awards_fields as na;
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_NATION_AWARDS_CONSTRUCT);
    base(this);
    write_usize(this, class.vtable);
    let nation = match class.nation_key {
        Some(key) => read_usize(G_NATION_TABLE) + read_u32(key) as usize * nation::SIZE,
        None => 0,
    };
    write_usize(this + na::NATION, nation);
    write_u32(this + na::NATION_ID, class.nation_id);
    write_u8(this + na::TYPE, class.kind);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATION_AWARDS_LOAD_FROM_DISK);
        if load(this, file) != 0 {
            return this;
        }
    }
    if let Some((cpp, line)) = class.month_ratings {
        let p = exe::crt_operator_new(0xf);
        let ratings = if p != 0 {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_MONTH_RATINGS_CONSTRUCT);
            f(p, 0)
        } else {
            0
        };
        write_usize(this + na::MONTH_RATINGS, ratings);
        if ratings == 0 {
            show_error_at(cpp, line);
            return this;
        }
    }
    covered!("sx05b_5 nation awards created");
    init(this, file);
    this
}

/// The comparer of a constructor that allocates (and may load): only the original runs; `check` then reads what it
/// built (without a file the plan is known) and `what` names the call.
pub(crate) unsafe fn compare_award_built(r: usize, this: usize, file: usize, check: impl FnOnce() -> bool, what: impl Fn() -> String) -> usize {
    if file != 0 {
        return r;
    }
    let ok = r == this && check();
    SCOREBOARD.record(ok, what);
    r
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, span: u32, mode: u32, file: usize, for_players: u32) -> usize {
    use team_award_obj as ta;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> usize>(IDX_TEAM_AWARD_CONSTRUCT);
    let r = o(this, nation, index, staff_comp, national_rep, world_rep, club_comp, span, mode, file, for_players);
    let t = TeamAward(this);
    compare_award_built(r, this, file, || {
        read_usize(this) == ta::VTABLE && read_u8(this + ta::SEASON_SPAN) == span as u8 && read_u8(this + ta::FORMATION_MODE) == mode as u8
            && read_u8(this + ta::SUBSTITUTES) == 3 && read_i32(this + ta::HISTORY_FIRST) == -1 && read_i32(this + ta::HISTORY_LAST) == -1
            && t.filled() == 0 && (0..20).all(|k| t.slot_id(k) == -1) && read_u32(this + award_base::NATION_ID) == nation
            && read_u32(this + award_base::INDEX) == index
    }, || format!("team_award_construct(nation {}, index {}, span {}, mode {}): vtable {:#x} filled {}", nation, index, span, mode, read_usize(this), t.filled()))
}

pub(crate) unsafe extern "thiscall" fn compare_year_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, kind: u32, scope: usize, option: u32, file: usize, for_players: u32) -> usize {
    use year_award_obj as ya;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, u32, usize, u32) -> usize>(IDX_YEAR_AWARD_CONSTRUCT);
    let r = o(this, nation, index, staff_comp, national_rep, world_rep, club_comp, kind, scope, option, file, for_players);
    compare_award_built(r, this, file, || {
        read_usize(this) == ya::VTABLE && read_u8(this + ya::KIND) == kind as u8 && read_u8(this + ya::OPTION) == option as u8
            && read_i32(this + ya::HISTORY_FIRST) == -1 && read_i32(this + ya::HISTORY_LAST) == -1 && read_u8(this + award_base::TYPE) == 1
            && (read_usize(this + ya::SHORTLIST) != 0) == (scope != 0)
    }, || format!("year_award_construct(nation {}, index {}, kind {}, scope {:#x}): vtable {:#x} shortlist {:#x}", nation, index, kind, scope,
        read_usize(this), read_usize(this + ya::SHORTLIST)))
}

/// Comparer for `year_ratings_construct`: side by side over the 0xe bytes of the table without a file; with one, only
/// the original (it loads).
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_construct(this: usize, year: u32, file: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_YEAR_RATINGS_CONSTRUCT);
    if file != 0 {
        return o(this, year, file);
    }
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(this, 0xe)], || o(this, year, file), || year_ratings_construct(this, year, file));
    SCOREBOARD.record(equal && r == v, || format!("year_ratings_construct(year {}) {:?}", year as u16, first_differences(&a, &b)));
    r
}

/// Where an award or AWOL news item goes: one club's managers, a division's, a nation's, a continent's, or every
/// manager (`News::send_to_*` on `G_NEWS`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum NewsPost {
    Club(usize),
    Division(usize),
    Nation(usize),
    Continent(usize),
    All,
}

/// What a news routine does with its items: post them in the game, or (a comparer's rehearsal) only note where.
pub(crate) type NewsSink<'a> = &'a mut dyn FnMut(usize, NewsPost);

/// The game's sink: posts `item` (`exe::news_send_to_*`, thiscall on `G_NEWS`).
pub(crate) unsafe fn news_post(item: usize, to: NewsPost) {
    let news = read_usize(G_NEWS);
    match to {
        NewsPost::Club(c) => exe::news_send_to_club_manager(news, item, c),
        NewsPost::Division(d) => exe::news_send_to_division_managers(news, item, d),
        NewsPost::Nation(n) => exe::news_send_to_nation_managers(news, item, n),
        NewsPost::Continent(c) => exe::news_send_to_continent_managers(news, item, c),
        NewsPost::All => exe::news_send_to_all_managers(news, item),
    };
}

/// The comparer of a routine that posts news (it cannot run twice: the inbox allocates): the original runs with its
/// random draws traced, then `rehearsal` (the copy, building the same items without posting them, see `NewsSink`)
/// from the same generator state, without counting; the rehearsal's draws must be the start of the original's (the
/// posting may draw more). Returns whether they are and the two draw lists, for the message.
pub(crate) unsafe fn rehearse_news(original: impl FnOnce(), rehearsal: impl FnOnce()) -> (bool, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let (mut from_original, mut from_copy) = (Vec::new(), Vec::new());
    crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || from_original = crate::rng::trace(original).1,
        || from_copy = crate::rng::trace(|| crate::rng::without_counting(rehearsal)).1);
    (from_original.starts_with(&from_copy), from_original, from_copy)
}

/// The `NationAwards` routines the nations' `daily_update` call by address that the England item (sx03_2) has no
/// helper for: the player of the year from the year ratings (today, award, ratings, then seven filters; ret 0x28), the
/// top three from the ratings (today, award, then seven filters ending in the position mask and the mode; ret 0x24)
/// and the base daily update (today; ret 4).
pub(crate) const FN_NATION_AWARDS_PLAYER_OF_YEAR_FROM_RATINGS: usize = 0x7683c0;
pub(crate) const FN_NATION_AWARDS_TOP_THREE_FROM_RATINGS: usize = 0x769380;
pub(crate) const FN_NATION_AWARDS_BASE_DAILY_UPDATE: usize = 0x768310;
/// The award manager's year ratings table (`YEAR_RATINGS`, +0xc of `G_AWARD_MANAGER`) [inference].
pub(crate) const AWARD_MANAGER_YEAR_RATINGS: usize = 0xc;

/// The player of the year of `award` from the year ratings (`NationAwards::award_player_of_year_from_ratings`) with
/// the award manager's ratings table and the seven filters as the nation passes them.
pub(crate) unsafe fn award_player_of_year_from_ratings(this: usize, today: usize, award: usize, filters: [i32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_PLAYER_OF_YEAR_FROM_RATINGS);
    let [a, b, c, d, e, g, h] = filters;
    f(this, today, award, read_usize(read_usize(G_AWARD_MANAGER) + AWARD_MANAGER_YEAR_RATINGS), a, b, c, d, e, g, h)
}

/// The top three of the year ratings for `award` (`nation_awards_award_top_three_from_ratings`) with the seven
/// filters as the nation passes them.
pub(crate) unsafe fn award_top_three_from_ratings(this: usize, today: usize, award: usize, filters: [i32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_TOP_THREE_FROM_RATINGS);
    let [a, b, c, d, e, g, h] = filters;
    f(this, today, award, a, b, c, d, e, g, h)
}

/// The base daily update of a nation's awards (`NationAwards::daily_update`: the month change).
pub(crate) unsafe fn nation_awards_base_daily_update(this: usize, today: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATION_AWARDS_BASE_DAILY_UPDATE);
    f(this, today)
}

/// The day a nation's month award moves on to the next season: today's day word (signed) is the season end day of the
/// nation's key nation record (`key_nation_record::SEASON_END_DAY` of `G_LEAGUE_TABLE` at
/// `exe::key_nation_index_of_nation`).
pub(crate) unsafe fn is_season_end_day(nation: usize, today: usize) -> bool {
    let k = exe::key_nation_index_of_nation(nation) as isize;
    read_i16(today) as i32 == read_u16((G_LEAGUE_TABLE as isize + k * 0x48) as usize + key_nation_record::SEASON_END_DAY) as i32
}

/// The comparer of a nation awards constructor: it allocates (and may load); only the original runs, then the object
/// must carry the class's vtable, nation id and type, and (without a file) `count` awards.
pub(crate) unsafe fn compare_nation_awards_built(r: usize, this: usize, file: usize, class: &NationAwardsClass, count: i16, what: &str) -> usize {
    use nation_awards_fields as na;
    let ok = r == this && read_usize(this) == class.vtable && read_u32(this + na::NATION_ID) == class.nation_id && read_u8(this + na::TYPE) == class.kind
        && (file != 0 || read_usize(this + na::TABLE) == 0 || read_i16(this + na::COUNT) == count);
    SCOREBOARD.record(ok, || format!("{}: vtable {:#x} nation id {:#x} type {} count {}", what, read_usize(this), read_u32(this + na::NATION_ID), read_u8(this + na::TYPE), read_i16(this + na::COUNT)));
    r
}

/// The comparer of a nation's `init_awards`: it allocates the awards; only the original runs, then each award must be
/// built for this nation in its slot (`award_base::NATION_ID`, `award_base::INDEX`) with its staff competition.
pub(crate) unsafe fn compare_nation_awards_init(this: usize, original: impl FnOnce(), what: &str) {
    use nation_awards_fields as na;
    original();
    let table = read_usize(this + na::TABLE);
    if table == 0 {
        return;
    }
    let n = read_i16(this + na::COUNT).max(0) as usize;
    let bad: Vec<usize> = (0..n).filter(|&k| {
        let a = read_usize(table + 4 * k);
        a != 0 && (read_u32(a + award_base::NATION_ID) != read_u32(this + na::NATION_ID) || read_u32(a + award_base::INDEX) != k as u32)
    }).collect();
    SCOREBOARD.record(bad.is_empty(), || format!("{}: awards with the wrong nation or index {:?}", what, bad));
}

/// The shared body of the nations' `update_team_of_the_week`: when `award` is one of the table's `slots`, the team of
/// the week (`build_team_of_the_week`).
pub(crate) unsafe fn nation_awards_team_of_the_week_of(this: usize, award: usize, slots: &[usize]) {
    if slots.iter().any(|&k| nation_award(this, k) == award) {
        covered!("sx05b_5 team of the week");
        build_team_of_the_week(this, award);
    }
}

// ---------- sx05b_11: a player goes AWOL and its news (awol.cpp) ----------

/// One of a player's reputation words cut by a random 0 to 40 percent: × (1 − draw(41) × 0.01), truncated.
unsafe fn awol_cut_reputation(word: usize) {
    let draw = exe::utils_random(0x29);
    let v = ftol((1.0 - draw as f64 * read_f64(rdata::F64_0_01)) * read_i16(word) as f64);
    write_i16(word, v as i16);
}

/// Sends a player AWOL. Nothing without a person, a player sheet, a staff id below `G_STAFF_COUNT`, or when his flag
/// already has bit 1 (`person_flags::ABSENT_WITHOUT_LEAVE`). Otherwise the bit is set and his current, home and world
/// reputations are each cut by a random 0 to 40 percent (`awol_cut_reputation`, one draw each, in that order). At his
/// club: he dislikes the manager; unless a human runs the club (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0)),
/// the manager dislikes him; each squad member but him dislikes him when a draw of his loyalty is not 0 and a draw of
/// his professionalism is 0; the chairman dislikes him (`exe::staff_add_disliked_staff`(who, whom, 1)). The absence
/// (12 bytes: his staff id and the return date, today plus 2 to 32 days for kind 4, else 21 to 170) goes to the
/// absences list. Then the news (`awol_post_news`) with `club` (or his own): kind 2 for kind 1, kind 4 with the days
/// for kind 4, kind 1 otherwise (days −1).
/// Original: `AWOL::start_absence` (0x419840, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn awol_start_absence(this: usize, person: usize, club: usize, kind: u32) {
    let person = Staff(person);
    if person.is_null() || person.player().is_null() || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let flag = read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(person.addr()) as usize;
    if read_u8(flag) & person_flags::ABSENT_WITHOUT_LEAVE as u8 != 0 {
        return;
    }
    covered!("sx05b_11 player goes awol");
    write_u8(flag, read_u8(flag) | person_flags::ABSENT_WITHOUT_LEAVE as u8);
    let sheet = person.player().addr();
    for word in [player_sheet::CURRENT_REPUTATION, player_sheet::HOME_REPUTATION, player_sheet::WORLD_REPUTATION] {
        awol_cut_reputation(sheet + word);
    }
    let own = person.club_job();
    if !own.is_null() {
        exe::staff_add_disliked_staff(person, own.manager(), 1);
        if exe::human_manager_is_human_club(own, 1, 1, 0, 0) == 0 {
            exe::staff_add_disliked_staff(own.manager(), person, 1);
        }
        for k in 0..50 {
            let mate = Staff(read_usize(own.addr() + club::SQUAD + 4 * k));
            if mate.is_null() || mate.addr() == person.addr() {
                continue;
            }
            if exe::utils_random(mate.loyalty() as i32) != 0 && exe::utils_random(mate.professionalism() as i32) == 0 {
                covered!("sx05b_11 squad mate dislikes him");
                exe::staff_add_disliked_staff(mate, person, 1);
            }
        }
        exe::staff_add_disliked_staff(own.chairman(), person, 1);
    }
    let mut record = [0u8; 12];
    let r = record.as_mut_ptr() as usize;
    write_i32(r, read_i32(person.addr()));
    let mut out = [0u8; 8];
    exe::date_operator_3d(r + 4, out.as_mut_ptr() as usize, G_CURRENT_DATE);
    let days = if kind as u8 == 4 { exe::utils_random(0x1f) + 2 } else { exe::utils_random(0x96) + 0x15 };
    let mut later = [0u8; 8];
    let back = exe::date_add_days(r + 4, later.as_mut_ptr() as usize, days as u32);
    exe::date_operator_3d(r + 4, out.as_mut_ptr() as usize, back);
    exe::llist_add_tail(read_usize(this), r, 0xc);
    let to = if club != 0 { club } else { own.addr() };
    let (news_kind, news_days) = match kind as u8 as i8 {
        1 => (2, u32::MAX),
        4 => (4, days as u32),
        _ => (1, u32::MAX),
    };
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_AWOL_POST_NEWS);
    post(this, person.addr(), to, news_kind, news_days);
}

/// Posts the AWOL news (type 0xbc5) of a player: nothing without a person, his club, his player sheet or `club`.
/// Slots: 0 his staff id, 1..3 his three name ids, 4 `club`'s id, 5 his club's id, 6 the kind, 7 the days (−1 none),
/// 8 his club's nation id (−1 none). A player with a world reputation above 7500 is news for every manager;
/// otherwise his club's nation (when it has one) and, when his nationality is another nation, that nation.
/// Original: `awol_post_news` (0x419ad0, awol.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn awol_post_news(this: usize, person: usize, club: usize, kind: u32, days: u32) {
    awol_post_news_with(this, person, club, kind, days, &mut |item, to| news_post(item, to));
}

/// `awol_post_news` with the posts going to `sink`.
pub(crate) unsafe fn awol_post_news_with(_this: usize, person: usize, club: usize, kind: u32, days: u32, sink: NewsSink) {
    let person = Staff(person);
    if person.is_null() || person.club_job().is_null() || person.player().is_null() || club == 0 {
        return;
    }
    covered!("sx05b_11 awol news");
    let mut item = [0u8; NEWS_ITEM_BYTES];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0xbc5, 0);
    let own = person.club_job();
    let nation = read_usize(own.addr() + club::NATION);
    let slots = [
        read_u32(person.addr()),
        read_u32(person.first_name() + names::ID),
        read_u32(person.second_name() + names::ID),
        read_u32(person.common_name() + names::ID),
        read_u32(club),
        read_u32(own.addr()),
        kind as u8 as i8 as i32 as u32,
        days as u16 as i16 as i32 as u32,
        if nation != 0 { read_u32(nation) } else { u32::MAX },
    ];
    for (k, &v) in slots.iter().enumerate() {
        exe::newsitem_set_slot(p, k as i8, v);
    }
    if person.player().world_reputation() > 7500 {
        return sink(p, NewsPost::All);
    }
    if nation != 0 {
        sink(p, NewsPost::Nation(nation));
    }
    let nationality = read_usize(person.addr() + staff::NATION);
    if read_usize(own.addr() + club::NATION) != nationality {
        sink(p, NewsPost::Nation(nationality));
    }
}

/// Comparer for `awol_start_absence`: the draws, the reputations, the dislikes, the list and the news cannot be
/// repeated; only the original runs, and when it could act (a player, his flag free), the flag must have bit 1 and
/// the absences one more record.
pub(crate) unsafe extern "thiscall" fn compare_awol_start_absence(this: usize, person: usize, club: usize, kind: u32) {
    let p = Staff(person);
    let acts = !p.is_null() && !p.player().is_null() && read_i32(p.addr()) < read_i32(G_STAFF_COUNT)
        && read_u8(read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(p.addr()) as usize) & 1 == 0;
    let before = read_i16(read_usize(this) + llist::COUNT);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_AWOL_START_ABSENCE)(this, person, club, kind);
    if acts {
        let after = read_i16(read_usize(this) + llist::COUNT);
        let flag = read_u8(read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(p.addr()) as usize);
        SCOREBOARD.record(flag & 1 != 0 && after == before + 1, || format!("awol_start_absence(person {:#x}, kind {}): flag {:#x}, absences {} -> {}", person, kind as u8, flag, before, after));
    }
}

/// Comparer for `awol_post_news`: the original posts; the copy rehearses (`rehearse_news`, no draws) and must post
/// when the original could.
pub(crate) unsafe extern "thiscall" fn compare_awol_post_news(this: usize, person: usize, club: usize, kind: u32, days: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_AWOL_POST_NEWS);
    let mut posts = Vec::new();
    let (ok, a, b) = rehearse_news(|| o(this, person, club, kind, days), || awol_post_news_with(this, person, club, kind, days, &mut |_, to| posts.push(to)));
    SCOREBOARD.record(ok, || format!("awol_post_news(person {:#x}, kind {}) draws original {:?} rust {:?}, rust posts {:x?}", person, kind as u8, a, b, posts));
}

// ---------- sx05b_21: setting the display currency (cash.cpp) ----------

/// The display currency's prefix text (before the amount) and the round exchange rate (f64) the preset amounts use;
/// the id of the display currency (byte).
pub(crate) const CURRENCY_PREFIX: usize = 0xad9ca8;
pub(crate) const CURRENCY_ROUND_RATE: usize = 0xad9da0;
pub(crate) const CURRENCY_ID: usize = 0x98b140;

/// One display currency as `cash_set_currency` sets it: the exchange rate, the round rate and the scale (f64 bits),
/// the prefix and suffix texts and the unit texts of thousands, millions, billions and thousands of billions (copied
/// from these addresses of the exe), the thousands and decimal separators and the no-separator-under-10,000 flag.
pub(crate) struct Currency {
    rate: u64,
    round_rate: u64,
    scale: u64,
    prefix: usize,
    suffix: usize,
    thousands: u8,
    no_separator_under_10000: u32,
    decimal: u8,
    units: [usize; 4],
}

/// The 26 currencies by id, as the original's jump table sets them.
pub(crate) const CURRENCIES: [Currency; 26] = [
    Currency { rate: 0x3ff0000000000000, round_rate: 0x3ff0000000000000, scale: 0x3ff0000000000000, prefix: 0x98b254, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 0 pound
    Currency { rate: 0x3ff7d70a3d70a3d7, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 1 peso
    Currency { rate: 0x4003eb851eb851ec, round_rate: 0x4004000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 2 Australian dollar
    Currency { rate: 0x405010a3d70a3d71, round_rate: 0x4049000000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b3e8, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3e0, 0x98b3d8, 0x98b3d0, 0x98b3c8] }, // 3 Belgian franc
    Currency { rate: 0x4005d70a3d70a3d7, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3c4, suffix: 0x98b3c0, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 4 real
    Currency { rate: 0x40283851eb851eb8, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b3bc, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b3b8, 0x98b3b0, 0x98b3ac, 0x98b3a8] }, // 5 kuna
    Currency { rate: 0x4027c28f5c28f5c3, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b3a4, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b39c, 0x98b394, 0x98b38c, 0x98b384] }, // 6 Danish krone
    Currency { rate: 0x3ff8cccccccccccd, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b380, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 7 euro
    Currency { rate: 0x4022f0a3d70a3d71, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b37c, thousands: 0x2c, no_separator_under_10000: 0x1, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 8 markka
    Currency { rate: 0x4024e66666666666, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b378, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b374, 0x98b370, 0x98b3e8, 0x98b36c] }, // 9 French franc
    Currency { rate: 0x4008e147ae147ae1, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b368, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b360, 0x98b358, 0x98b350, 0x98b348] }, // 10 Deutschmark
    Currency { rate: 0x400c147ae147ae14, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0x98b344, suffix: 0x98b340, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 11 guilder
    Currency { rate: 0x4080b70a3d70a3d7, round_rate: 0x4081300000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b33c, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b334, 0x98b330, 0x98b3f0, 0x98b3ec] }, // 12 drachma
    Currency { rate: 0x3ff4000000000000, round_rate: 0x3ff0000000000000, scale: 0x3ff0000000000000, prefix: 0x98b328, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b324, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 13 Irish pound
    Currency { rate: 0x40a817dc28f5c28f, round_rate: 0x40a7700000000000, scale: 0x408f400000000000, prefix: 0x98b31c, suffix: 0xad9c64, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3f8, 0x98b318, 0x98b3f4, 0x98b3ec] }, // 14 lira
    Currency { rate: 0x4063be147ae147ae, round_rate: 0x4062c00000000000, scale: 0x4059000000000000, prefix: 0x98b2dc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 15 yen
    Currency { rate: 0x402a28f5c28f5c29, round_rate: 0x402e000000000000, scale: 0x4024000000000000, prefix: 0x98b2d8, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3b8, 0x98b2d4, 0x98b3ac, 0x98b3a8] }, // 16 Norwegian krone
    Currency { rate: 0x401a5c28f5c28f5c, round_rate: 0x4014000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b2d0, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b2c8, 0x98b2c0, 0x98b3ac, 0x98b3a8] }, // 17 zloty
    Currency { rate: 0x3fd460807357e671, round_rate: 0x3fd0000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b2bc, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b2b8, 0x98b2b4, 0x98b2b0, 0x98b2ac] }, // 18 escudo
    Currency { rate: 0x4044dae147ae147b, round_rate: 0x4049000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b2a8, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b2a0, 0x98b294, 0x98b3ac, 0x98b3a8] }, // 19 ruble
    Currency { rate: 0x4070887ae147ae14, round_rate: 0x406f400000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b28c, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b284, 0x98b27c, 0xad9c64, 0x98b274] }, // 20 peseta
    Currency { rate: 0x4030000000000000, round_rate: 0x402e000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b270, thousands: 0x20, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b268, 0x98b260, 0x98b258, 0x98b268] }, // 21 Swedish krona
    Currency { rate: 0x4003ae147ae147ae, round_rate: 0x4004000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b24c, thousands: 0x2c, no_separator_under_10000: 0x1, decimal: 0x2e, units: [0x98b3b8, 0x98b244, 0x98b3ac, 0x98b3a8] }, // 22 Swiss franc
    Currency { rate: 0x412c77273d70a3d7, round_rate: 0x412e848000000000, scale: 0x412e848000000000, prefix: 0xad9c64, suffix: 0x98b240, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3b8, 0x98b238, 0x98b230, 0x98b224] }, // 23 Turkish lira
    Currency { rate: 0x3ff9eb851eb851ec, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 24 US dollar
    Currency { rate: 0x409dc00000000000, round_rate: 0x409f400000000000, scale: 0x409f400000000000, prefix: 0xad9c64, suffix: 0x98b310, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b308, 0x98b2f8, 0x98b2e8, 0x98b2e0] }, // 25 won
];

/// Sets the display currency `id` (0 to 25, signed): its exchange rate (`CURRENCY_RATE`), round rate
/// (`CURRENCY_ROUND_RATE`) and scale (`CURRENCY_SCALE`), its prefix (`CURRENCY_PREFIX`) and suffix (`CURRENCY_SUFFIX`),
/// the unit texts (`CURRENCY_UNIT_THOUSANDS`, `_MILLIONS`, `_BILLIONS`, `_TRILLIONS`), the separators and the
/// no-separator flag, and the id itself (`CURRENCY_ID`); 1. Any other id: the "Error" box (line 0x265) and 0, nothing
/// changed.
/// Original: `cash_set_currency` (0x43f7f0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_set_currency(id: u32) -> u32 {
    let k = id as u8 as i8 as i32;
    if !(0..26).contains(&k) {
        show_error_at(CASH_CPP, 0x265);
        return 0;
    }
    let c = &CURRENCIES[k as usize];
    covered!("sx05b_21 currency set");
    write_u64(CURRENCY_RATE, c.rate);
    write_u64(CURRENCY_ROUND_RATE, c.round_rate);
    write_u64(CURRENCY_SCALE, c.scale);
    tel26_copy_text(CURRENCY_PREFIX as *mut u8, c.prefix as *const u8);
    tel26_copy_text(CURRENCY_SUFFIX as *mut u8, c.suffix as *const u8);
    for (dst, &src) in [CURRENCY_UNIT_THOUSANDS, CURRENCY_UNIT_MILLIONS, CURRENCY_UNIT_BILLIONS, CURRENCY_UNIT_TRILLIONS].iter().zip(c.units.iter()) {
        tel26_copy_text(*dst as *mut u8, src as *const u8);
    }
    write_u8(CURRENCY_THOUSANDS_SEPARATOR, c.thousands);
    write_u8(CURRENCY_DECIMAL_SEPARATOR, c.decimal);
    write_u32(CURRENCY_NO_SEPARATOR_UNDER_10000, c.no_separator_under_10000);
    write_u8(CURRENCY_ID, id as u8);
    1
}

/// Comparer for `cash_set_currency`: side by side over the currency globals (0xad9c70 to the round rate, and the id).
pub(crate) unsafe extern "C" fn compare_cash_set_currency(id: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_CASH_SET_CURRENCY);
    let regions = [(CURRENCY_SUFFIX, CURRENCY_ROUND_RATE + 8 - CURRENCY_SUFFIX), (CURRENCY_ID, 1)];
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&regions, || o(id), || cash_set_currency(id));
    SCOREBOARD.record(equal && r == v, || format!("cash_set_currency({}) original={} rust={} {:?}", id as u8 as i8, r, v, first_differences(&a, &b)));
    r
}

// ---------- sx05b_22: stepping a money amount up or down on the display currency's ladder (cash.cpp) ----------

/// A display currency amount back to a stored cash value: divided by the exchange rate (`CURRENCY_RATE`), truncated to
/// 64 bits (`_ftol`) and kept in its low dword.
pub(crate) unsafe fn cash_from_display(amount: i64) -> u32 {
    (amount as f64 / read_f64(CURRENCY_RATE)) as i64 as u32
}

/// Raises a cash value one step: its display amount (`cash_get_display_amount` with `flags`: rounded to the nearest
/// step unless flag 0x20) plus the rounding step of the next amount (`cash_get_rounding_step`(amount + 1, flags)), back
/// in pounds (`cash_from_display`); stored and returned.
/// Original: `Cash::step_up` (0x43f4e0, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_step_up(cash: usize, flags: u32) -> u32 {
    let amount = cash_get_display_amount(cash, flags);
    let v = cash_from_display(amount.wrapping_add(cash_get_rounding_step(amount.wrapping_add(1), flags)));
    covered!("sx05b_22 cash stepped up");
    write_u32(cash, v);
    v
}

/// Lowers a cash value one step: its display amount (as `cash_step_up`) minus the rounding step of that amount, back
/// in pounds; stored and returned.
/// Original: `Cash::step_down` (0x43f580, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_step_down(cash: usize, flags: u32) -> u32 {
    let amount = cash_get_display_amount(cash, flags);
    let v = cash_from_display(amount.wrapping_sub(cash_get_rounding_step(amount, flags)));
    covered!("sx05b_22 cash stepped down");
    write_u32(cash, v);
    v
}

/// Comparer for `cash_step_up`: side by side over the cash value and the result.
pub(crate) unsafe extern "thiscall" fn compare_cash_step_up(cash: usize, flags: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_CASH_STEP_UP);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, flags), || cash_step_up(cash, flags));
    SCOREBOARD.record(equal && r == v, || format!("cash_step_up(flags {:#x}) original={} rust={} {:?}", flags, r, v, first_differences(&a, &b)));
    r
}

/// Comparer for `cash_step_down`: side by side over the cash value and the result.
pub(crate) unsafe extern "thiscall" fn compare_cash_step_down(cash: usize, flags: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_CASH_STEP_DOWN);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, flags), || cash_step_down(cash, flags));
    SCOREBOARD.record(equal && r == v, || format!("cash_step_down(flags {:#x}) original={} rust={} {:?}", flags, r, v, first_differences(&a, &b)));
    r
}

// ---------- sx05b_23: the display currency's id and the 64-bit byte swap (cash.cpp) ----------

/// The id of the display currency (`CURRENCY_ID`).
/// Original: `cash_get_currency` (0x4415e0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_get_currency() -> u8 {
    covered!("sx05b_23 currency read");
    read_u8(CURRENCY_ID)
}

/// Reverses the byte order of each of the `count` 64-bit words of `buf`, in place (the original builds each swap from
/// 64-bit shifts, `__allshl` and `__allshr`: the same as a full byte reversal). Used by `Formation::load_from_disk` for
/// tactics files of the other byte order.
/// Original: `swap_bytes_64_array` (0x43f350, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn swap_bytes_64_array(buf: usize, count: u32) {
    for k in 0..count as usize {
        let p = buf + 8 * k;
        write_u64(p, read_u64(p).swap_bytes());
    }
}

/// Comparer for `cash_get_currency`: the value side by side.
pub(crate) unsafe extern "C" fn compare_cash_get_currency() -> u8 {
    let r = original_fn::<unsafe extern "C" fn() -> u8>(IDX_CASH_GET_CURRENCY)();
    let v = cash_get_currency();
    SCOREBOARD.record(r == v, || format!("cash_get_currency original={} rust={}", r, v));
    r
}

/// Comparer for `swap_bytes_64_array`: side by side over the buffer.
pub(crate) unsafe extern "C" fn compare_swap_bytes_64_array(buf: usize, count: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_SWAP_BYTES_64_ARRAY);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(buf, 8 * count as usize)], || o(buf, count), || swap_bytes_64_array(buf, count));
    SCOREBOARD.record(equal, || format!("swap_bytes_64_array({} words) {:?}", count, first_differences(&a, &b)));
}
