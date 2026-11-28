// ---------- r7_02_1: loading the database (Database.cpp) ----------

/// `g_index_dat_entries`: the 22 index.dat entries (67 bytes: the file name, the table id +0x33, the count
/// `db_index_entry::RECORD_COUNT`, where the table starts +0x3b, the version `db_index_entry::VERSION`).
const LOAD_DB_INDEX: usize = 0xae1a80;
/// Number of index.dat entries.
const LOAD_DB_INDEX_SLOTS: usize = 22;
/// Size of an index.dat entry.
const LOAD_DB_ENTRY_SIZE: usize = 0x43;
/// index.dat entry: the table id (4 bytes).
const LOAD_DB_ENTRY_TABLE_ID: usize = 0x33;
/// index.dat entry: where the table starts in its file (4 bytes).
const LOAD_DB_ENTRY_START: usize = 0x3b;
/// `g_reserve_comp_row_count`: the reserve competition rows at the end of the club competition table.
const LOAD_DB_RESERVE_COMP_ROWS: usize = 0xae2344;
/// `g_reserve_manager_row_count`: the spare manager rows (index.dat header; 0x10 after a linked load).
const LOAD_DB_RESERVE_MANAGER_ROWS: usize = 0xae2348;
/// `g_nat_club_count`: the national teams at the end of the club table.
const LOAD_DB_NAT_CLUB_COUNT: usize = 0xae2360;
/// `g_preferences_count`.
const LOAD_DB_PREFERENCES_COUNT: usize = 0xae2370;
/// `g_nation_comp_count`: the nation competitions inside the club competition table.
const LOAD_DB_NATION_COMP_COUNT: usize = 0xae2380;
/// `g_staff_comp_count`.
const LOAD_DB_STAFF_COMP_COUNT: usize = 0xae2388;
/// `g_colour_count`.
const LOAD_DB_COLOUR_COUNT: usize = 0xae2394;
/// `g_first_name_count`.
const LOAD_DB_FIRST_NAME_COUNT: usize = 0xae2398;
/// `g_second_name_count`.
const LOAD_DB_SECOND_NAME_COUNT: usize = 0xae239c;
/// `g_nation_comp_table` (inside the club competition table).
const LOAD_DB_NATION_COMP_TABLE: usize = 0xae23d8;
/// The people of a version 1 staff.dat (157 bytes each) before they are turned into 110-byte records: count and table.
const LOAD_DB_STAFF_V1_COUNT: usize = 0xae2400;
const LOAD_DB_STAFF_V1_TABLE: usize = 0xae2404;
/// Size of a version 1 person record.
const LOAD_DB_STAFF_V1_SIZE: usize = 0x9d;
/// The text buffer the load writes its warnings into (`sprintf`, never shown).
const LOAD_DB_LOG_LINE: usize = 0xde1f64;
/// The exe's imports of GlobalAlloc and GlobalFree (the DLL's allocator replaces them, see allocator.rs).
const LOAD_DB_IAT_GLOBAL_ALLOC: usize = 0x9670f0;
const LOAD_DB_IAT_GLOBAL_FREE: usize = 0x9670f4;
/// `utils_copy_file(source, target)` (0x90cc50, cdecl): copies a file ("rb" into "wb").
const FN_UTILS_COPY_FILE: usize = 0x90cc50;
/// The call to `set_key_ids` at the end of the load: its displacement and the address after it (cm0102-1080.exe calls
/// its own entry 0x603834; the copy follows whatever the running exe calls).
const LOAD_DB_SET_KEYS_CALL: usize = 0x52d8fb;
const LOAD_DB_SET_KEYS_NEXT: usize = 0x52d8ff;
/// Texts: "Loading database", "index.dat", "*.lng", "CM3_TEMP", "", "duff.out", "Reserve".
const LOAD_DB_PROGRESS_TEXT: usize = 0x9b5c54;
const LOAD_DB_INDEX_DAT: usize = 0x9b5c48;
const LOAD_DB_LNG_PATTERN: usize = 0x9b5b38;
const LOAD_DB_TEMP_KEY: usize = 0x987da0;
const LOAD_DB_EMPTY_TEXT: usize = 0xad9c64;
const LOAD_DB_DUFF_OUT: usize = 0x9b5c3c;
const LOAD_DB_RESERVE_TEXT: usize = 0x9b5b40;
/// The warnings: "Staff %s (%s) belongs to extinct club (%s)", "'%s (%ld)' has no nation set", "No NON_PLAYER Data for
/// %s", "No PLAYER Data for %s", "(DUO) No NON_PLAYER Data for %s", "(DUO) No PLAYER Data for %s", "(NEW) No staff
/// nation set (despite national job) for %s".
const LOAD_DB_LOG_EXTINCT_CLUB: usize = 0x9b5c10;
const LOAD_DB_LOG_NO_NATION: usize = 0x9b5bf0;
const LOAD_DB_LOG_NO_NON_PLAYER: usize = 0x9b5bd4;
const LOAD_DB_LOG_NO_PLAYER: usize = 0x9b5bbc;
const LOAD_DB_LOG_DUO_NO_NON_PLAYER: usize = 0x9b5b9c;
const LOAD_DB_LOG_DUO_NO_PLAYER: usize = 0x9b5b80;
const LOAD_DB_LOG_NEW_NO_NATION: usize = 0x9b5b48;
/// The progress steps: 0, 1 after index.dat, one per entry in each of the two passes, and the last (46).
const LOAD_DB_STEPS: u32 = 0x2e;
/// The rows a linked load adds: the reserve competitions, the free people and non-player sheets (when index.dat has
/// no extension) and the slack of the people and player tables.
const LOAD_DB_RESERVE_COMPS: i32 = 0x7f;
const LOAD_DB_FREE_PEOPLE: i32 = 0x59d8;
const LOAD_DB_STAFF_SLACK: i32 = 0x52d;
/// The spare rows of the last load (the comparer masks the bytes the original never sets in them), the people of the
/// version 1 staff.dat and the entries of the clubs' English name index.
static LOAD_DB_LAST_SPARE: AtomicUsize = AtomicUsize::new(0);
static LOAD_DB_LAST_V1_ROWS: AtomicUsize = AtomicUsize::new(0);
static LOAD_DB_LAST_NAME_INDEX: AtomicUsize = AtomicUsize::new(0);

/// A table reader of Db_files.cpp: (stream, use_zip, records, index entry) → 0 on a short read.
type LoadDbReader = unsafe extern "C" fn(usize, i32, usize, usize) -> i32;

/// A database file: a C stream of the Data folder (fread) or an item of the saved game's container (zread).
#[derive(Clone, Copy)]
struct LoadDbFile {
    handle: usize,
    zip: bool,
}

impl LoadDbFile {
    /// Opens `name` in the folder (`db_files_open_for_read`) or in the container (`zopen`(source, name, 0, 0x15, 4)).
    unsafe fn open(source: usize, name: usize, zip: bool) -> LoadDbFile {
        let handle = if zip { exe::zopen(source, name, 0, 0x15, 4) } else { db_files_open_for_read(source, name) };
        LoadDbFile { handle, zip }
    }
    unsafe fn read(self, to: usize, size: u32, count: u32) -> u32 {
        if self.zip {
            exe::zread(to, size, count, self.handle)
        } else {
            exe::crt_fread(to, size, count, self.handle)
        }
    }
    unsafe fn seek(self, offset: i32) {
        if self.zip {
            buscar_no_conteiner(self.handle, offset, 0);
        } else {
            exe::crt_fseek(self.handle, offset, 0);
        }
    }
    unsafe fn close(self) {
        if self.zip {
            let mut handle = self.handle;
            exe::zclose(&mut handle);
        } else {
            exe::crt_fclose(self.handle);
        }
    }
}

/// One step of the "Loading database" progress bar.
unsafe fn load_db_progress(text: &[u8], step: i32) {
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, step as i16 as i32, LOAD_DB_STEPS, -1);
}

/// `malloc` into a table global; a null block stops the game with the Database.cpp box at `line`.
unsafe fn load_db_malloc(table: usize, bytes: i32, line: u32) -> usize {
    let p = exe::crt_malloc(bytes as u32);
    write_usize(table, p);
    if p == 0 {
        database_fatal_error(line);
    }
    p
}

/// `GlobalAlloc(0, bytes)` through the exe's import (the linked database blocks).
unsafe fn load_db_global_alloc(bytes: i32) -> usize {
    let f: unsafe extern "system" fn(u32, u32) -> usize = core::mem::transmute(read_usize(LOAD_DB_IAT_GLOBAL_ALLOC));
    f(0, bytes as u32)
}

/// `GlobalFree(p)` through the exe's import.
unsafe fn load_db_global_free(p: usize) {
    let f: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(read_usize(LOAD_DB_IAT_GLOBAL_FREE));
    f(p);
}

/// A block of the linked layout: GlobalAlloc in the linked mode, malloc otherwise.
unsafe fn load_db_block(bytes: i32) -> usize {
    if read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0 { load_db_global_alloc(bytes) } else { exe::crt_malloc(bytes as u32) }
}

/// Adds `n` to a count global.
unsafe fn load_db_add(count: usize, n: i32) {
    write_i32(count, read_i32(count).wrapping_add(n));
}

/// The language files: from the saved game's container into the temporary folder (none: the configured language goes
/// back to 0), or every *.lng of the Data folder copied into it; then, when the configured language has no file, the
/// language goes back to 0 (English).
unsafe fn load_db_language_files(source: usize, zip: bool, temp: &[u8; 0x104]) {
    if zip {
        if exe::zextract_matching_files(source as *const u8, temp.as_ptr(), LOAD_DB_LNG_PATTERN) == 0 {
            exe::config_set_language(read_usize(G_CONFIG), 0);
        }
    } else {
        let copy: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_UTILS_COPY_FILE);
        let mut path = [0u8; 0x104];
        let mut found = [0u8; 0x114];
        let mut target = [0u8; 0x104];
        exe::crt_makepath(path.as_mut_ptr(), 0usize, source, LOAD_DB_LNG_PATTERN, 0usize);
        let mut r = exe::file_find_first(path.as_ptr() as usize, found.as_mut_ptr() as usize, 0);
        while r == 0 {
            let name = found.as_ptr() as usize + 0x10;
            exe::crt_makepath(path.as_mut_ptr(), 0usize, source, name, 0usize);
            copy_text(target.as_mut_ptr(), exe::utils_build_file_path(LOAD_DB_TEMP_KEY, name) as *const u8);
            copy(path.as_ptr() as usize, target.as_ptr() as usize);
            r = exe::file_find_next(found.as_mut_ptr());
        }
        exe::file_find_close(found.as_ptr() as usize);
    }
    let config = read_usize(G_CONFIG);
    if exe::database_language_file_exists(read_u32(config + 8)) == 0 {
        exe::config_set_language(read_usize(G_CONFIG), 0);
    }
}

/// index.dat: the two header counts, the 22 entries and, after them, an optional extension byte. With it: 460 bytes
/// skipped, the clubs' English name index (a count, −1 for none, then the count of 4-byte entries, malloc'ed into
/// `G_CLUB_ENGLISH_NAME_INDEX_TABLE`) and the names language; without it the names language is 0. Returns the
/// extension byte, or None when the file is missing or the header or the entries come short.
unsafe fn load_db_read_index(source: usize, zip: bool) -> Option<u8> {
    let f = LoadDbFile::open(source, LOAD_DB_INDEX_DAT, zip);
    if f.handle == 0 {
        return None;
    }
    if f.read(LOAD_DB_RESERVE_COMP_ROWS, 4, 1) != 1 || f.read(LOAD_DB_RESERVE_MANAGER_ROWS, 4, 1) != 1
        || f.read(LOAD_DB_INDEX, LOAD_DB_ENTRY_SIZE as u32, LOAD_DB_INDEX_SLOTS as u32) != LOAD_DB_INDEX_SLOTS as u32 {
        f.close();
        return None;
    }
    let mut extension = 0u8;
    f.read(&mut extension as *mut u8 as usize, 1, 1);
    LOAD_DB_LAST_NAME_INDEX.store(0, Relaxed);
    if extension == 0 {
        covered!("r7_02_1 index without extension");
        write_i32(G_DATABASE_NAMES_LANGUAGE, 0);
    } else {
        covered!("r7_02_1 index with extension");
        let mut skipped = [0u8; 0x1cc];
        f.read(skipped.as_mut_ptr() as usize, 0x1cc, 1);
        let mut n = 0u32;
        f.read(&mut n as *mut u32 as usize, 4, 1);
        if n != u32::MAX {
            let p = exe::crt_malloc(n << 2);
            write_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE, p);
            if p == 0 {
                database_fatal_error(0x917);
            }
            LOAD_DB_LAST_NAME_INDEX.store(n as usize, Relaxed);
            f.read(p, n.wrapping_mul(4), 1);
        }
        f.read(G_DATABASE_NAMES_LANGUAGE, 4, 1);
    }
    f.close();
    Some(extension)
}

/// The separate layout: each entry's table gets its own malloc (a null one stops the game with its line). The
/// reserve competition rows are zeroed at each entry; the people, the three name tables and their entries lose the
/// spare manager rows; the non-players and the preferences too, in their counts only.
unsafe fn load_db_allocate_separately(text: &[u8], step: &mut i32) {
    for k in 0..LOAD_DB_INDEX_SLOTS {
        load_db_progress(text, *step);
        *step += 1;
        write_i32(LOAD_DB_RESERVE_COMP_ROWS, 0);
        let e = LOAD_DB_INDEX + k * LOAD_DB_ENTRY_SIZE;
        let at = e + db_index_entry::RECORD_COUNT;
        let n = read_i32(at);
        let spare = read_i32(LOAD_DB_RESERVE_MANAGER_ROWS);
        let without_spare = |count: usize| {
            let m = read_i32(at).wrapping_sub(read_i32(LOAD_DB_RESERVE_MANAGER_ROWS));
            write_i32(at, m);
            write_i32(count, m);
            m
        };
        match read_i32(e + LOAD_DB_ENTRY_TABLE_ID) {
            0 => { load_db_add(G_CLUB_COUNT, n); load_db_malloc(G_CLUB_TABLE, n.wrapping_mul(club::SIZE as i32), 0x982); }
            1 => { load_db_add(LOAD_DB_NAT_CLUB_COUNT, n); load_db_malloc(G_NAT_CLUB_TABLE, n.wrapping_mul(club::SIZE as i32), 0x98b); }
            2 => { load_db_add(LOAD_DB_COLOUR_COUNT, n); load_db_malloc(G_COLOUR_TABLE, n.wrapping_mul(0x3a), 0x9af); }
            3 => { load_db_add(G_CONTINENT_COUNT, n); load_db_malloc(G_CONTINENT_TABLE, n.wrapping_mul(0xc6), 0x94c); }
            4 => { write_i32(G_NATION_COUNT, n); load_db_malloc(G_NATION_TABLE, n.wrapping_mul(nation::SIZE as i32), 0x9b8); }
            5 => { write_i32(G_STADIUM_COUNT, n); load_db_malloc(G_STADIUM_TABLE, n.wrapping_mul(0x4e), 0x9c1); }
            6 => {
                let m = without_spare(G_STAFF_COUNT);
                load_db_malloc(G_STAFF_TABLE, m.wrapping_mul(staff::SIZE as i32), 0x9cc);
                if read_i32(e + db_index_entry::VERSION) == 1 {
                    covered!("r7_02_1 separate version 1 people");
                    let v1 = without_spare(LOAD_DB_STAFF_V1_COUNT);
                    load_db_malloc(LOAD_DB_STAFF_V1_TABLE, v1.wrapping_mul(LOAD_DB_STAFF_V1_SIZE as i32), 0x9d8);
                }
            }
            7 => { load_db_add(G_OFFICIAL_COUNT, n); load_db_malloc(G_OFFICIAL_TABLE, n.wrapping_mul(0x2b), 0x955); }
            9 => { write_i32(G_NON_PLAYER_COUNT, n.wrapping_sub(spare)); load_db_malloc(G_NON_PLAYER_TABLE, n.wrapping_mul(0x44), 0x9ef); }
            10 => { write_i32(G_PLAYER_DATA_COUNT, n); load_db_malloc(G_PLAYER_DATA_TABLE, n.wrapping_mul(0x46), 0x9f8); }
            0xb => { load_db_add(LOAD_DB_STAFF_COMP_COUNT, n); load_db_malloc(G_STAFF_COMP_TABLE, n.wrapping_mul(0x65), 0x994); }
            0xc => { load_db_add(G_CLUB_COMP_COUNT, n); load_db_malloc(G_CLUB_COMP_TABLE, n.wrapping_mul(0x6b), 0x99d); }
            0xd => { let m = without_spare(LOAD_DB_FIRST_NAME_COUNT); load_db_malloc(G_FIRST_NAME_TABLE, m.wrapping_mul(0x3c), 0x95e); }
            0xe => { let m = without_spare(LOAD_DB_SECOND_NAME_COUNT); load_db_malloc(G_SECOND_NAME_TABLE, m.wrapping_mul(0x3c), 0x967); }
            0xf => { let m = without_spare(G_COMMON_NAME_COUNT); load_db_malloc(G_COMMON_NAME_TABLE, m.wrapping_mul(0x3c), 0x970); }
            0x10 => { load_db_add(LOAD_DB_NATION_COMP_COUNT, n); load_db_malloc(LOAD_DB_NATION_COMP_TABLE, n.wrapping_mul(0x6b), 0x9a6); }
            0x11 => { write_i32(G_STAFF_HISTORY_COUNT, n); load_db_malloc(G_STAFF_HISTORY_TABLE, n.wrapping_mul(0x11), 0xa03); }
            0x12 => { load_db_add(G_STAFF_COMP_HISTORY_COUNT, n); load_db_malloc(G_STAFF_COMP_HISTORY_TABLE, n.wrapping_mul(0x3a), 0xa0e); }
            0x13 => { load_db_add(G_CLUB_COMP_HISTORY_COUNT, n); load_db_malloc(G_CLUB_COMP_HISTORY_TABLE, n.wrapping_mul(0x1a), 0xa19); }
            0x14 => { load_db_add(G_NATION_COMP_HISTORY_COUNT, n); load_db_malloc(G_NATION_COMP_HISTORY_TABLE, n.wrapping_mul(0x1a), 0xa22); }
            0x15 => { load_db_add(G_CITY_COUNT, n); load_db_malloc(G_CITY_TABLE, n.wrapping_mul(0x38), 0x979); }
            0x16 => { write_i32(LOAD_DB_PREFERENCES_COUNT, n.wrapping_sub(spare)); load_db_malloc(G_PREFERENCES_TABLE, n.wrapping_mul(0x34), 0x9e6); }
            _ => {}
        }
    }
}

/// The byte sizes of the linked layout, summed per block while the counts are set.
#[derive(Default)]
struct LoadDbBlocks {
    shared: i32,
    clubs: i32,
    staff: i32,
    non_players: i32,
    player_data: i32,
    staff_history: i32,
    staff_comp_history: i32,
    comp_history: i32,
}

/// The linked layout, first pass: the counts and the bytes of each block. The people get the spare rows plus 0x5f05
/// (0x52d with the index extension), the non-players the spare rows plus 23,000 (none with the extension), the player
/// sheets 0x52d, the club competitions the 127 reserve rows and the name tables the spare rows. A version 1 staff.dat
/// gets its own 157-byte table, and the preferences theirs (the spare rows added).
unsafe fn load_db_size_linked(text: &[u8], step: &mut i32, extension: u8, spare: i32) -> LoadDbBlocks {
    let mut b = LoadDbBlocks::default();
    for k in 0..LOAD_DB_INDEX_SLOTS {
        load_db_progress(text, *step);
        *step += 1;
        let e = LOAD_DB_INDEX + k * LOAD_DB_ENTRY_SIZE;
        let n = read_i32(e + db_index_entry::RECORD_COUNT);
        let mul = |rows: i32, size: usize| rows.wrapping_mul(size as i32);
        match read_i32(e + LOAD_DB_ENTRY_TABLE_ID) {
            0 => { load_db_add(G_CLUB_COUNT, n); b.clubs = b.clubs.wrapping_add(mul(n, club::SIZE)); }
            1 => {
                load_db_add(LOAD_DB_NAT_CLUB_COUNT, n);
                load_db_add(G_CLUB_COUNT, n);
                b.clubs = b.clubs.wrapping_add(mul(n, club::SIZE));
            }
            2 => { load_db_add(LOAD_DB_COLOUR_COUNT, n); b.shared = b.shared.wrapping_add(mul(n, 0x3a)); }
            3 => { load_db_add(G_CONTINENT_COUNT, n); b.shared = b.shared.wrapping_add(mul(n, 0xc6)); }
            4 => { write_i32(G_NATION_COUNT, n); b.shared = b.shared.wrapping_add(mul(n, nation::SIZE)); }
            5 => { write_i32(G_STADIUM_COUNT, n); b.shared = b.shared.wrapping_add(mul(n, 0x4e)); }
            6 => {
                let rows = n.wrapping_add(spare).wrapping_add(if extension == 0 { 0x5f05 } else { LOAD_DB_STAFF_SLACK });
                write_i32(G_STAFF_COUNT, rows);
                b.staff = b.staff.wrapping_add(mul(rows, staff::SIZE));
                if read_i32(e + db_index_entry::VERSION) == 1 {
                    covered!("r7_02_1 linked version 1 people");
                    write_i32(LOAD_DB_STAFF_V1_COUNT, n);
                    load_db_malloc(LOAD_DB_STAFF_V1_TABLE, mul(n, LOAD_DB_STAFF_V1_SIZE), 0xa7f);
                }
            }
            7 => load_db_add(G_OFFICIAL_COUNT, n),
            9 => {
                let rows = if extension == 0 { n.wrapping_add(spare).wrapping_add(LOAD_DB_FREE_PEOPLE) } else { n.wrapping_add(spare) };
                write_i32(G_NON_PLAYER_COUNT, rows);
                b.non_players = b.non_players.wrapping_add(mul(rows, 0x44));
            }
            10 => {
                let rows = n.wrapping_add(LOAD_DB_STAFF_SLACK);
                write_i32(G_PLAYER_DATA_COUNT, rows);
                b.player_data = b.player_data.wrapping_add(mul(rows, 0x46));
            }
            0xb => { load_db_add(LOAD_DB_STAFF_COMP_COUNT, n); b.shared = b.shared.wrapping_add(mul(n, 0x65)); }
            0xc => {
                load_db_add(G_CLUB_COMP_COUNT, n.wrapping_add(LOAD_DB_RESERVE_COMPS));
                b.shared = b.shared.wrapping_add(mul(n.wrapping_add(LOAD_DB_RESERVE_COMPS), 0x6b));
            }
            0xd => { load_db_add(LOAD_DB_FIRST_NAME_COUNT, n.wrapping_add(spare)); b.shared = b.shared.wrapping_add(mul(n.wrapping_add(spare), 0x3c)); }
            0xe => { load_db_add(LOAD_DB_SECOND_NAME_COUNT, n.wrapping_add(spare)); b.shared = b.shared.wrapping_add(mul(n.wrapping_add(spare), 0x3c)); }
            0xf => { load_db_add(G_COMMON_NAME_COUNT, n.wrapping_add(spare)); b.shared = b.shared.wrapping_add(mul(n.wrapping_add(spare), 0x3c)); }
            0x10 => {
                load_db_add(G_CLUB_COMP_COUNT, n);
                load_db_add(LOAD_DB_NATION_COMP_COUNT, n);
                b.shared = b.shared.wrapping_add(mul(n, 0x6b));
            }
            0x11 => { write_i32(G_STAFF_HISTORY_COUNT, n); b.staff_history = b.staff_history.wrapping_add(mul(n, 0x11)); }
            0x12 => { load_db_add(G_STAFF_COMP_HISTORY_COUNT, n); b.staff_comp_history = b.staff_comp_history.wrapping_add(mul(n, 0x3a)); }
            0x13 => { load_db_add(G_CLUB_COMP_HISTORY_COUNT, n); b.comp_history = b.comp_history.wrapping_add(mul(n, 0x1a)); }
            0x14 => {
                load_db_add(G_CLUB_COMP_HISTORY_COUNT, n);
                load_db_add(G_NATION_COMP_HISTORY_COUNT, n);
                b.comp_history = b.comp_history.wrapping_add(mul(n, 0x1a));
            }
            0x15 => { load_db_add(G_CITY_COUNT, n); b.shared = b.shared.wrapping_add(mul(n, 0x38)); }
            0x16 => {
                write_i32(LOAD_DB_PREFERENCES_COUNT, n.wrapping_add(spare));
                load_db_malloc(G_PREFERENCES_TABLE, mul(n.wrapping_add(spare), 0x34), 0xa8c);
            }
            _ => {}
        }
    }
    b
}

/// The linked layout, second pass: the blocks (the people history, the shared block and the staff competition history
/// through GlobalAlloc in the linked mode, the rest through malloc; any null stops the game, line 0xac6), then each
/// table inside the shared block in a fixed order (continents, the three name tables, cities, staff competitions,
/// club competitions with the nation competitions before the reserve rows, colours, nations, stadiums), the nation
/// competition history at the end of the competition history and the national teams at the end of the clubs. The
/// people and player sheet counts lose their 0x52d slack rows.
unsafe fn load_db_place_linked(b: &LoadDbBlocks) {
    write_usize(G_STAFF_HISTORY_TABLE, load_db_block(b.staff_history));
    write_usize(G_CONTINENT_TABLE, load_db_block(b.shared));
    write_usize(G_CLUB_TABLE, exe::crt_malloc(b.clubs as u32));
    write_usize(G_STAFF_TABLE, exe::crt_malloc(b.staff as u32));
    write_usize(G_PLAYER_DATA_TABLE, exe::crt_malloc(b.player_data as u32));
    write_usize(G_NON_PLAYER_TABLE, exe::crt_malloc(b.non_players as u32));
    write_usize(G_STAFF_COMP_HISTORY_TABLE, load_db_block(b.staff_comp_history));
    write_usize(G_CLUB_COMP_HISTORY_TABLE, exe::crt_malloc(b.comp_history as u32));
    write_usize(G_OFFICIAL_TABLE, exe::crt_malloc(read_i32(G_OFFICIAL_COUNT).wrapping_mul(0x2b) as u32));
    let blocks = [G_STAFF_HISTORY_TABLE, G_CONTINENT_TABLE, G_CLUB_TABLE, G_STAFF_TABLE, G_PLAYER_DATA_TABLE,
        G_NON_PLAYER_TABLE, G_STAFF_COMP_HISTORY_TABLE, G_CLUB_COMP_HISTORY_TABLE, G_OFFICIAL_TABLE];
    if blocks.iter().any(|&t| read_usize(t) == 0) {
        database_fatal_error(0xac6);
    }
    let base = read_usize(G_CONTINENT_TABLE);
    let count = |g: usize| read_i32(g) as isize;
    // each table starts where the previous one ends: (table, the previous table's rows, its record size)
    let mut at = 0isize;
    let mut place = |table: usize, rows: isize, size: isize| -> isize {
        at += rows * size;
        write_usize(table, (base as isize + at) as usize);
        at
    };
    place(G_FIRST_NAME_TABLE, count(G_CONTINENT_COUNT), 0xc6);
    place(G_SECOND_NAME_TABLE, count(LOAD_DB_FIRST_NAME_COUNT), 0x3c);
    place(G_COMMON_NAME_TABLE, count(LOAD_DB_SECOND_NAME_COUNT), 0x3c);
    place(G_CITY_TABLE, count(G_COMMON_NAME_COUNT), 0x3c);
    place(G_STAFF_COMP_TABLE, count(G_CITY_COUNT), 0x38);
    let comps = place(G_CLUB_COMP_TABLE, count(LOAD_DB_STAFF_COMP_COUNT), 0x65);
    let nation_rows = count(LOAD_DB_NATION_COMP_COUNT) + LOAD_DB_RESERVE_COMPS as isize;
    write_usize(LOAD_DB_NATION_COMP_TABLE, (base as isize + comps + (count(G_CLUB_COMP_COUNT) - nation_rows) * 0x6b) as usize);
    place(G_COLOUR_TABLE, count(G_CLUB_COMP_COUNT), 0x6b);
    place(G_NATION_TABLE, count(LOAD_DB_COLOUR_COUNT), 0x3a);
    place(G_STADIUM_TABLE, count(G_NATION_COUNT), nation::SIZE as isize);
    let history = read_usize(G_CLUB_COMP_HISTORY_TABLE) as isize;
    write_usize(G_NATION_COMP_HISTORY_TABLE, (history + (count(G_CLUB_COMP_HISTORY_COUNT) - count(G_NATION_COMP_HISTORY_COUNT)) * 0x1a) as usize);
    let clubs = read_usize(G_CLUB_TABLE) as isize;
    write_usize(G_NAT_CLUB_TABLE, (clubs + (count(G_CLUB_COUNT) - count(LOAD_DB_NAT_CLUB_COUNT)) * club::SIZE as isize) as usize);
    write_i32(G_STAFF_COUNT, read_i32(G_STAFF_COUNT).wrapping_sub(LOAD_DB_STAFF_SLACK));
    write_i32(G_PLAYER_DATA_COUNT, read_i32(G_PLAYER_DATA_COUNT).wrapping_sub(LOAD_DB_STAFF_SLACK));
}

/// A free person row of the linked layout: the id, no job, the default dates, born 1971 (0x7b3), the classification
/// (6 for the free rows, which the post-load checks skip) and the non-player sheet `non_player` (an id); `names`
/// (first, second, common) and `preferences` are ids or −1.
unsafe fn load_db_blank_person(row: i32, names: [i32; 3], preferences: i32, non_player: i32, classification: u8) {
    let p = read_usize(G_STAFF_TABLE) + row as usize * staff::SIZE;
    write_i32(p, row);
    write_i32(p + staff::FIRST_NAME, names[0]);
    write_i32(p + staff::SECOND_NAME, names[1]);
    write_i32(p + staff::COMMON_NAME, names[2]);
    exe::date_date_54a110(p + staff::DATE_OF_BIRTH);
    write_u16(p + staff::YEAR_OF_BIRTH, 0x7b3); // r7_02_1: born 1971
    write_i32(p + staff::NATION, -1);
    write_i32(p + staff::SECOND_NATION, -1);
    write_u8(p + staff::INT_APPS, 0);
    write_u8(p + staff::INT_GOALS, 0);
    write_i32(p + staff::NATIONAL_JOB, -1);
    write_u8(p + staff::JOB_FOR_NATION, 0);
    exe::date_date_54a110(p + staff::DATE_JOINED_NATION);
    exe::date_date_54a110(p + 0x31); // the date the nation job ends
    write_i32(p + staff::CLUB_JOB, -1);
    write_u8(p + staff::JOB_FOR_CLUB, 0);
    exe::date_date_54a110(p + staff::DATE_JOINED_CLUB);
    exe::date_date_54a110(p + staff::DATE_EXPIRES_CLUB);
    write_i32(p + staff::WAGE, 0);
    write_i32(p + staff::VALUE, 0);
    for o in staff::ADAPTABILITY..=staff::PLAYING_SQUAD {
        write_u8(p + o, 0);
    }
    write_u8(p + staff::CLASSIFICATION, classification);
    write_u8(p + staff::CLASSIFICATION + 1, 0);
    write_i32(p + staff::PLAYER, -1);
    write_i32(p + staff::STAFF_PREFERENCES, preferences);
    write_i32(p + staff::NON_PLAYER, non_player);
    write_u8(p + staff::SQUAD_SELECTED_FOR, 0);
}

/// A blank non-player sheet: the id, zero abilities and reputations, zero attributes, no tactics (−1) and 0 at the end.
unsafe fn load_db_blank_non_player(row: i32) {
    let p = read_usize(G_NON_PLAYER_TABLE) + row as usize * 0x44;
    write_i32(p, row);
    for o in (non_player::CURRENT_ABILITY..=non_player::WORLD_REPUTATION).step_by(2) {
        write_u16(p + o, 0);
    }
    for o in non_player::ATTACKING..=non_player::YOUNGSTERS {
        write_u8(p + o, 0);
    }
    for o in (non_player::GOALKEEPER..non_player::FORMATION).step_by(4) {
        write_i32(p + o, -1);
    }
    write_u8(p + non_player::FORMATION, 0);
}

/// The rows the linked layout adds: the 127 reserve competitions (id, empty names, no nation, continent or colours),
/// with no index extension the 23,000 free people (classification 6) with their free non-player sheets, and the spare
/// rows at the end of the people, the name tables (empty name, weight 1), the non-players and the preferences.
unsafe fn load_db_fill_linked_rows(extension: u8, spare: i32) {
    for i in 1..=LOAD_DB_RESERVE_COMPS {
        let row = read_i32(G_CLUB_COMP_COUNT) - i;
        let c = read_usize(G_CLUB_COMP_TABLE) + row as usize * 0x6b;
        write_i32(c, row);
        write_u8(c + competition::NAME, 0);
        write_u8(c + competition::SHORT_NAME, 0);
        write_u8(c + 0x53, 0); // a byte the original zeroes
        write_u8(c + 0x57, 0); // two bytes the original zeroes
        write_u8(c + 0x58, 0);
        for o in [competition::CONTINENT, competition::NATION, competition::FOREGROUND_COLOUR, competition::BACKGROUND_COLOUR] {
            write_i32(c + o, -1);
        }
        write_u16(c + competition::REPUTATION, 0); // r7_02_1: reserve competition reputation
    }
    if extension == 0 {
        covered!("r7_02_1 free people rows");
        for i in 1..=LOAD_DB_FREE_PEOPLE {
            let row = read_i32(G_STAFF_COUNT) - i - spare;
            let sheet = read_i32(G_NON_PLAYER_COUNT) - i - spare;
            load_db_blank_person(row, [-1, -1, 0], -1, sheet, 6);
            load_db_blank_non_player(sheet);
        }
    }
    for i in 1..=spare {
        let names = [read_i32(LOAD_DB_FIRST_NAME_COUNT) - i, read_i32(LOAD_DB_SECOND_NAME_COUNT) - i, read_i32(G_COMMON_NAME_COUNT) - i];
        load_db_blank_person(read_i32(G_STAFF_COUNT) - i, names, read_i32(LOAD_DB_PREFERENCES_COUNT) - i, read_i32(G_NON_PLAYER_COUNT) - i, 0);
        for (table, row) in [(G_FIRST_NAME_TABLE, names[0]), (G_SECOND_NAME_TABLE, names[1]), (G_COMMON_NAME_TABLE, names[2])] {
            let n = read_usize(table) + row as usize * 0x3c;
            write_u8(n, 0);
            write_i32(n + names::ID, row);
            write_i32(n + names::NATION, -1);
            write_u8(n + names::WEIGHT, 1);
        }
        load_db_blank_non_player(read_i32(G_NON_PLAYER_COUNT) - i);
        if read_usize(G_PREFERENCES_TABLE) != 0 {
            let row = read_i32(LOAD_DB_PREFERENCES_COUNT) - i;
            let p = read_usize(G_PREFERENCES_TABLE) + row as usize * 0x34;
            write_i32(p, row);
            for o in (preferences::STAFF_FAVOURITE_CLUBS1..0x34).step_by(4) {
                write_i32(p + o, -1);
            }
        }
    }
}

/// One table file: opened by its entry's name, positioned at the entry's start and read by `reader` into the table in
/// `table` (nothing when the table is null). False when the file is missing or the reader comes short (the file is
/// closed): the load then returns 0.
unsafe fn load_db_read_table(source: usize, zip: bool, entry: usize, table: usize, reader: LoadDbReader) -> bool {
    let records = read_usize(table);
    if records == 0 {
        return true;
    }
    let f = LoadDbFile::open(source, entry, zip);
    if f.handle == 0 {
        return false;
    }
    f.seek(read_i32(entry + LOAD_DB_ENTRY_START));
    if reader(f.handle, zip as i32, records, entry) == 0 {
        f.close();
        return false;
    }
    f.close();
    true
}

/// The nation competitions and, with reserve competition rows in index.dat, as many more rows of the same file into
/// the reserve rows (through a local entry with the name, that count and the version; the original leaves its table id
/// and start unset, the reader does not read them).
unsafe fn load_db_read_nation_comps(source: usize, zip: bool, entry: usize) -> bool {
    let table = read_usize(LOAD_DB_NATION_COMP_TABLE);
    if table == 0 {
        return true;
    }
    let f = LoadDbFile::open(source, entry, zip);
    if f.handle == 0 {
        return false;
    }
    f.seek(read_i32(entry + LOAD_DB_ENTRY_START));
    if db_files_read_comps(f.handle, zip as i32, table, entry) == 0 {
        f.close();
        return false;
    }
    let reserve = read_i32(LOAD_DB_RESERVE_COMP_ROWS);
    if reserve > 0 {
        covered!("r7_02_1 reserve competitions from the file");
        let mut local = [0u8; LOAD_DB_ENTRY_SIZE];
        copy_text(local.as_mut_ptr(), entry as *const u8);
        write_i32(local.as_mut_ptr() as usize + db_index_entry::RECORD_COUNT, reserve);
        write_i32(local.as_mut_ptr() as usize + db_index_entry::VERSION, read_i32(entry + db_index_entry::VERSION));
        let rows = read_usize(G_CLUB_COMP_TABLE) + (read_i32(G_CLUB_COMP_COUNT) - LOAD_DB_RESERVE_COMPS) as usize * 0x6b;
        if db_files_read_comps(f.handle, zip as i32, rows, local.as_ptr() as usize) == 0 {
            f.close();
            return false;
        }
    }
    f.close();
    true
}

/// The people history and the staff competition history are optional: a missing file closes a null stream (a defect
/// of the original, copied) and frees the table, which stays empty.
unsafe fn load_db_read_optional(source: usize, zip: bool, entry: usize, table: usize, count: usize, reader: LoadDbReader) -> bool {
    if read_usize(table) == 0 {
        return true;
    }
    let f = LoadDbFile::open(source, entry, zip);
    if f.handle != 0 {
        f.seek(read_i32(entry + LOAD_DB_ENTRY_START));
        let ok = reader(f.handle, zip as i32, read_usize(table), entry) != 0;
        f.close();
        return ok;
    }
    covered!("r7_02_1 optional table missing");
    f.close();
    let p = read_usize(table);
    if p != 0 {
        if read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0 { load_db_global_free(p) } else { exe::crt_free(p) }
    }
    write_usize(table, 0);
    write_i32(count, 0);
    true
}

/// The second pass over index.dat: each entry's file read into its table by the Db_files.cpp reader of its table id.
/// False when a file is missing or comes short.
unsafe fn load_db_read_tables(text: &[u8], step: &mut i32, source: usize, zip: bool) -> bool {
    for k in 0..LOAD_DB_INDEX_SLOTS {
        load_db_progress(text, *step);
        *step += 1;
        let e = LOAD_DB_INDEX + k * LOAD_DB_ENTRY_SIZE;
        let read = |table: usize, reader: LoadDbReader| load_db_read_table(source, zip, e, table, reader);
        let ok = match read_i32(e + LOAD_DB_ENTRY_TABLE_ID) {
            0 => read(G_CLUB_TABLE, db_files_read_clubs),
            1 => read(G_NAT_CLUB_TABLE, db_files_read_clubs),
            2 => read(G_COLOUR_TABLE, db_files_read_colours),
            3 => read(G_CONTINENT_TABLE, db_files_read_continents),
            4 => read(G_NATION_TABLE, db_files_read_nations),
            5 => read(G_STADIUM_TABLE, db_files_read_stadiums),
            6 if read_usize(LOAD_DB_STAFF_V1_TABLE) != 0 => read(LOAD_DB_STAFF_V1_TABLE, db_files_read_staff_v1),
            6 => read(G_STAFF_TABLE, db_files_read_staff),
            7 => read(G_OFFICIAL_TABLE, db_files_read_officials),
            9 => read(G_NON_PLAYER_TABLE, db_files_read_players),
            10 => read(G_PLAYER_DATA_TABLE, db_files_read_non_players),
            0xb => read(G_STAFF_COMP_TABLE, db_files_read_staff_comps),
            0xc => read(G_CLUB_COMP_TABLE, db_files_read_comps),
            0xd => read(G_FIRST_NAME_TABLE, db_files_read_names),
            0xe => read(G_SECOND_NAME_TABLE, db_files_read_names),
            0xf => read(G_COMMON_NAME_TABLE, db_files_read_names),
            0x10 => load_db_read_nation_comps(source, zip, e),
            0x11 => load_db_read_optional(source, zip, e, G_STAFF_HISTORY_TABLE, G_STAFF_HISTORY_COUNT, db_files_read_staff_history),
            0x12 => load_db_read_optional(source, zip, e, G_STAFF_COMP_HISTORY_TABLE, G_STAFF_COMP_HISTORY_COUNT, db_files_read_staff_comp_history),
            0x13 => read(G_CLUB_COMP_HISTORY_TABLE, db_files_read_comp_history),
            0x14 => read(G_NATION_COMP_HISTORY_TABLE, db_files_read_comp_history),
            0x15 => read(G_CITY_TABLE, db_files_read_cities),
            0x16 => read(G_PREFERENCES_TABLE, db_files_read_staff_extra),
            _ => true,
        };
        if !ok {
            return false;
        }
    }
    true
}

/// A version 1 staff.dat becomes the 110-byte people table: the fields copied from each 157-byte record (the player
/// sheet from +0x91, the non-player sheet from +0x99, the preferences moved out into a new preferences table of
/// 52-byte rows, one per person plus the spare rows, whose 12 links come from +0x61..+0x90), then the version 1
/// table freed. Without the people table, or with preferences already loaded, the game stops (0xf9b, 0xfa2).
/// Defect of the original: the converted people never get `staff::SQUAD_SELECTED_FOR` (heap leftovers stay there).
unsafe fn load_db_convert_staff_v1() {
    if read_usize(G_STAFF_TABLE) == 0 {
        database_fatal_error(0xf9b);
    }
    if read_usize(G_PREFERENCES_TABLE) != 0 {
        database_fatal_error(0xfa2);
    }
    let n = read_i32(LOAD_DB_STAFF_V1_COUNT);
    write_i32(LOAD_DB_PREFERENCES_COUNT, n.wrapping_add(read_i32(LOAD_DB_RESERVE_MANAGER_ROWS)));
    load_db_malloc(G_PREFERENCES_TABLE, read_i32(LOAD_DB_PREFERENCES_COUNT).wrapping_mul(0x34), 0xfaa);
    let copy = |to: usize, from: usize, bytes: usize| core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, bytes);
    for k in 0..n.max(0) as usize {
        let p = read_usize(G_STAFF_TABLE) + k * staff::SIZE;
        let v = read_usize(LOAD_DB_STAFF_V1_TABLE) + k * LOAD_DB_STAFF_V1_SIZE;
        copy(p, v, 0x10); // id and the three names
        copy(p + staff::DATE_OF_BIRTH, v + staff::DATE_OF_BIRTH, 8);
        copy(p + staff::YEAR_OF_BIRTH, v + staff::YEAR_OF_BIRTH, 0xc); // year of birth, nations, international caps
        copy(p + staff::CLUB_JOB, v + staff::CLUB_JOB, 5);
        copy(p + staff::DATE_JOINED_CLUB, v + staff::DATE_JOINED_CLUB, 8);
        copy(p + staff::DATE_EXPIRES_CLUB, v + staff::DATE_EXPIRES_CLUB, 8);
        copy(p + staff::NATIONAL_JOB, v + staff::NATIONAL_JOB, 5);
        copy(p + staff::DATE_JOINED_NATION, v + staff::DATE_JOINED_NATION, 8);
        copy(p + 0x31, v + 0x31, 8); // the date the nation job ends
        copy(p + staff::WAGE, v + staff::WAGE, 0x12); // wage, value and the ten personality bytes
        write_i32(p + staff::PLAYER, read_i32(v + 0x91));
        write_i32(p + staff::STAFF_PREFERENCES, k as i32);
        write_i32(p + staff::NON_PLAYER, read_i32(v + 0x99));
        write_u8(p + staff::CLASSIFICATION + 1, read_u8(v + staff::CLASSIFICATION + 1));
        let q = read_usize(G_PREFERENCES_TABLE) + k * 0x34;
        write_i32(q, k as i32);
        copy(q + preferences::STAFF_FAVOURITE_CLUBS1, v + 0x61, 0x30);
    }
    for i in 1..=read_i32(LOAD_DB_RESERVE_MANAGER_ROWS) {
        let row = read_i32(LOAD_DB_PREFERENCES_COUNT) - i;
        let q = read_usize(G_PREFERENCES_TABLE) + row as usize * 0x34;
        write_i32(q, row);
        for o in (preferences::STAFF_FAVOURITE_CLUBS1..0x34).step_by(4) {
            write_i32(q + o, -1);
        }
    }
    let v = read_usize(LOAD_DB_STAFF_V1_TABLE);
    if v != 0 {
        exe::crt_free(v);
        write_usize(LOAD_DB_STAFF_V1_TABLE, 0);
        write_i32(LOAD_DB_STAFF_V1_COUNT, 0);
    }
}

/// The default name of a person without one: the first empty common name (its row kept in
/// `G_EMPTY_COMMON_NAME_INDEX`), or a random common name when none is empty (drawn again each time).
unsafe fn load_db_empty_name() -> usize {
    let mut row = read_i32(G_EMPTY_COMMON_NAME_INDEX);
    if row == -1 {
        let n = read_i32(G_COMMON_NAME_COUNT);
        let table = read_usize(G_COMMON_NAME_TABLE);
        match (0..n.max(0)).find(|&k| read_u8(table + k as usize * 0x3c) == 0) {
            Some(k) => {
                write_i32(G_EMPTY_COMMON_NAME_INDEX, k);
                row = k;
            }
            None => {
                covered!("r7_02_1 random empty name");
                row = exe::utils_random(n);
            }
        }
    }
    read_usize(G_COMMON_NAME_TABLE).wrapping_add((row as isize * 0x3c) as usize)
}

/// Writes one warning into the load's text buffer, with the person's name.
unsafe fn load_db_warn(p: usize, format: usize) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut name = [0u8; 500];
    exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 500, 1);
    sprintf(LOAD_DB_LOG_LINE, format, name.as_ptr());
}

/// The check of a job byte (`job`: the club job or the nation job) against the sheets: a non-playing job (1..10)
/// without a non-player sheet and a player (11) without a player sheet lose the job (the player also leaves his club's
/// squad); a player-manager (12, 13, 15) without the non-player sheet becomes a player, and without the player sheet
/// loses the job. Each case writes its warning.
unsafe fn load_db_check_job(p: usize, job: usize) {
    let no_sheet = |o: usize| read_usize(p + o) == 0;
    match read_i8(p + job) {
        1..=10 => {
            if no_sheet(staff::NON_PLAYER) {
                load_db_warn(p, LOAD_DB_LOG_NO_NON_PLAYER);
                write_u8(p + job, 0);
            }
        }
        11 => {
            if no_sheet(staff::PLAYER) {
                load_db_warn(p, LOAD_DB_LOG_NO_PLAYER);
                write_u8(p + job, 0);
                let club = read_usize(p + staff::CLUB_JOB);
                if club != 0 {
                    for k in 0..50 {
                        if read_usize(club + club::SQUAD + 4 * k) == p {
                            write_usize(club + club::SQUAD + 4 * k, 0);
                        }
                    }
                }
            }
        }
        12 | 13 | 15 => {
            if no_sheet(staff::NON_PLAYER) {
                load_db_warn(p, LOAD_DB_LOG_DUO_NO_NON_PLAYER);
                write_u8(p + job, 11);
            }
            if no_sheet(staff::PLAYER) {
                load_db_warn(p, LOAD_DB_LOG_DUO_NO_PLAYER);
                write_u8(p + job, 0);
            }
        }
        _ => {}
    }
}

/// The checks of one person after the load (all but the last 16 rows and the free rows, classification 6).
unsafe fn load_db_fix_person(p: usize, from_save: bool) {
    let club = read_usize(p + staff::CLUB_JOB);
    if club != 0 && read_usize(club + club::NATION) == 0 {
        covered!("r7_02_1 person of an extinct club");
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let mut name = [0u8; 200];
        exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 200, 1);
        let nation = read_usize(p + staff::NATION);
        let nationality = if nation == 0 { LOAD_DB_EMPTY_TEXT } else { nation + nation::NATIONALITY };
        sprintf(LOAD_DB_LOG_LINE, LOAD_DB_LOG_EXTINCT_CLUB, name.as_ptr(), nationality, club + club::NAME);
        exe::database_remove_staff_from_club(club, p, 0usize, 1, 0);
        write_usize(p + staff::CLUB_JOB, 0);
    }
    let club = read_usize(p + staff::CLUB_JOB);
    if read_u8(p + staff::PLAYING_SQUAD) & 2 != 0 && club != 0 && exe::database_club_has_reserve_squad(club) == 0 {
        write_u8(p + staff::PLAYING_SQUAD, read_u8(p + staff::PLAYING_SQUAD) & 0xfd | 1);
    }
    if read_u8(p + staff::INT_GOALS) != 0 && read_u8(p + staff::INT_APPS) == 0 {
        write_u8(p + staff::INT_GOALS, 0);
    }
    if read_usize(p + staff::CLUB_JOB) == 0 && read_usize(p + staff::NATIONAL_JOB) == 0 && read_usize(p + staff::NON_PLAYER) != 0 {
        let mut limit = [0u16; 4];
        exe::date_date_549f70(&mut limit, 1, 0, 0x7ce, u32::MAX);
        let joined_year = read_u16(p + staff::DATE_JOINED_CLUB_YEAR);
        if limit[1] > joined_year || (limit[1] == joined_year && limit[0] as i16 > read_i16(p + staff::DATE_JOINED_CLUB)) {
            covered!("r7_02_1 joined before 1998");
            let mut date = [0u16; 4];
            exe::date_date_549f70(&mut date, 1, 8, 0x7cd, u32::MAX);
            core::ptr::copy_nonoverlapping(date.as_ptr() as *const u8, (p + staff::DATE_JOINED_CLUB) as *mut u8, 8);
        }
    }
    if read_u8(p + staff::JOB_FOR_CLUB) == 7 {
        write_u8(p + staff::JOB_FOR_CLUB, 6);
    }
    if read_u8(p + staff::JOB_FOR_NATION) == 7 {
        write_u8(p + staff::JOB_FOR_NATION, 6);
    }
    if read_u8(p + staff::JOB_FOR_NATION) == 5 && read_usize(p + staff::NON_PLAYER) == 0 {
        let team = read_usize(p + staff::NATIONAL_JOB);
        if team != 0 {
            write_usize(team + club::MANAGER, 0);
        }
        write_u8(p + staff::JOB_FOR_NATION, 0);
        write_usize(p + staff::NATIONAL_JOB, 0);
    }
    if read_usize(p + staff::NATIONAL_JOB) != 0 && read_u8(p + staff::JOB_FOR_NATION) == 0 {
        write_usize(p + staff::NATIONAL_JOB, 0);
    }
    if read_usize(p + staff::NATION) == 0 {
        covered!("r7_02_1 person without nation");
        let club = read_usize(p + staff::CLUB_JOB);
        let team = read_usize(p + staff::NATIONAL_JOB);
        let nation = if club != 0 {
            read_usize(club + club::NATION)
        } else if team != 0 {
            read_usize(team + club::NATION)
        } else {
            read_usize(G_NATION_TABLE) + exe::utils_random(read_i32(G_NATION_COUNT)) as usize * nation::SIZE
        };
        write_usize(p + staff::NATION, nation);
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let mut name = [0u8; 500];
        exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 500, 1);
        sprintf(LOAD_DB_LOG_LINE, LOAD_DB_LOG_NO_NATION, name.as_ptr(), read_i32(p));
    }
    load_db_check_job(p, staff::JOB_FOR_CLUB);
    for o in [staff::FIRST_NAME, staff::SECOND_NAME, staff::COMMON_NAME] {
        if read_usize(p + o) == 0 {
            write_usize(p + o, load_db_empty_name());
        }
    }
    load_db_check_job(p, staff::JOB_FOR_NATION);
    let team = read_usize(p + staff::NATIONAL_JOB);
    if !from_save && read_usize(p + staff::NON_PLAYER) != 0 && team != 0 && read_u8(p + staff::JOB_FOR_NATION) != 5 {
        covered!("r7_02_1 national job left");
        exe::database_remove_staff_from_club(team, p, 0usize, 1, 0);
        write_u8(p + staff::JOB_FOR_NATION, 0);
        write_usize(p + staff::NATIONAL_JOB, 0);
    }
    let job = read_i8(p + staff::JOB_FOR_NATION);
    if job > 0 && (job <= 0xd || job == 0xf) {
        if read_usize(p + staff::NATION) == 0 {
            write_usize(p + staff::NATIONAL_JOB, 0);
            write_u8(p + staff::JOB_FOR_NATION, 0);
            load_db_warn(p, LOAD_DB_LOG_NEW_NO_NATION);
        } else if read_usize(p + staff::NATIONAL_JOB) == 0 {
            write_usize(p + staff::NATIONAL_JOB, 0);
            write_u8(p + staff::JOB_FOR_NATION, 0);
        }
    }
}

/// With no index extension: each national team (the last clubs, one per nation) gets half the reputation of its
/// nation's national team club (rounded toward zero), and a club whose division is named "Reserve" loses it.
unsafe fn load_db_fix_clubs() {
    let mut k = read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT);
    while k < read_i32(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + k as usize * club::SIZE;
        let team = exe::database_get_national_team(read_usize(c + club::NATION));
        write_i16(c + club::REPUTATION, (read_i16(team + club::REPUTATION) as i32 / 2) as i16);
        k += 1;
    }
    let mut k = 0;
    while k < read_i32(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + k as usize * club::SIZE;
        let division = read_usize(c + club::DIVISION);
        if division != 0 && load_db_same_text(division + competition::NAME, LOAD_DB_RESERVE_TEXT) {
            covered!("r7_02_1 reserve division cleared");
            write_usize(c + club::DIVISION, 0);
        }
        k += 1;
    }
}

/// `strcmp(a, b) == 0`.
unsafe fn load_db_same_text(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// Loads the database from index.dat and the table files, from the Data folder (`from_save` = 0, fread) or from the
/// saved game's container (`from_save` ≠ 0, zread), in the linked layout (`shared` ≠ 0: a few big blocks with the
/// tables inside, spare rows for the people the game creates, and the ids turned into pointers at the end) or in the
/// separate one (`shared` = 0: one malloc per table, ids kept).
///
/// In order: the temporary folder's path; every table freed (`exe::database_free_all_tables`); the "Loading database"
/// progress bar (46 steps); the language files (see `load_db_language_files`); index.dat (see `load_db_read_index`;
/// missing or short: returns 0). Then `G_DATABASE_SHARED_ALLOC_MODE` = `shared` and the first pass over the 22 entries:
/// the separate layout (`load_db_allocate_separately`) or the linked one (`load_db_size_linked`, `load_db_place_linked`
/// and `load_db_fill_linked_rows`, with spare rows = 16 − `LOAD_DB_RESERVE_MANAGER_ROWS`). The second pass reads every
/// file into its table (`load_db_read_tables`; a missing file or a short read returns 0). The reserve rows become 127
/// and 16 in the linked layout (0 and 0 in the separate one); a version 1 staff.dat is converted
/// (`load_db_convert_staff_v1`); the linked layout resolves the ids (`resolve_database_indexes`(0)), the separate one
/// numbers the nation competitions and their history from 0. "duff.out" is deleted; every person but the last 16 and
/// the free ones is checked (`load_db_fix_person`); with no index extension the national teams and the reserve
/// divisions are fixed (`load_db_fix_clubs`); last, the fixed keys (`set_key_ids`). Returns 1.
/// Original: `load_database` (0x528540, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn load_database(source: usize, shared: u32, from_save: u32) -> u32 {
    covered!("r7_02_1 load database");
    let zip = from_save != 0;
    let mut temp = [0u8; 0x104];
    copy_text(temp.as_mut_ptr(), exe::utils_build_file_path(LOAD_DB_TEMP_KEY, LOAD_DB_EMPTY_TEXT) as *const u8);
    exe::database_free_all_tables();
    let progress_text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut text = [0u8; 0x100];
    progress_text(text.as_mut_ptr() as usize, LOAD_DB_PROGRESS_TEXT);
    load_db_progress(&text, 0);
    load_db_language_files(source, zip, &temp);
    let extension = match load_db_read_index(source, zip) {
        Some(e) => e,
        None => return 0,
    };
    let spare = 0x10 - read_i32(LOAD_DB_RESERVE_MANAGER_ROWS);
    load_db_progress(&text, 1);
    let mut step = 2;
    write_u32(G_DATABASE_SHARED_ALLOC_MODE, shared);
    if shared == 0 {
        covered!("r7_02_1 separate layout");
        LOAD_DB_LAST_SPARE.store(0, Relaxed);
        load_db_allocate_separately(&text, &mut step);
    } else {
        covered!("r7_02_1 linked layout");
        LOAD_DB_LAST_SPARE.store(spare.max(0) as usize, Relaxed);
        let blocks = load_db_size_linked(&text, &mut step, extension, spare);
        load_db_place_linked(&blocks);
        load_db_fill_linked_rows(extension, spare);
    }
    if !load_db_read_tables(&text, &mut step, source, zip) {
        return 0;
    }
    load_db_progress(&text, step);
    if read_u32(G_DATABASE_SHARED_ALLOC_MODE) == 0 {
        write_i32(LOAD_DB_RESERVE_COMP_ROWS, 0);
        write_i32(LOAD_DB_RESERVE_MANAGER_ROWS, 0);
    } else {
        write_i32(LOAD_DB_RESERVE_COMP_ROWS, LOAD_DB_RESERVE_COMPS);
        write_i32(LOAD_DB_RESERVE_MANAGER_ROWS, read_i32(LOAD_DB_RESERVE_MANAGER_ROWS).wrapping_add(spare));
    }
    LOAD_DB_LAST_V1_ROWS.store(read_i32(LOAD_DB_STAFF_V1_COUNT).max(0) as usize, Relaxed);
    if read_usize(LOAD_DB_STAFF_V1_TABLE) != 0 {
        covered!("r7_02_1 version 1 people converted");
        load_db_convert_staff_v1();
    }
    if read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0 {
        resolve_database_indexes(0);
    } else {
        for k in 0..read_i32(LOAD_DB_NATION_COMP_COUNT).max(0) {
            write_i32(read_usize(LOAD_DB_NATION_COMP_TABLE) + k as usize * 0x6b, k);
        }
        for k in 0..read_i32(G_NATION_COMP_HISTORY_COUNT).max(0) {
            write_i32(read_usize(G_NATION_COMP_HISTORY_TABLE) + k as usize * 0x1a, k);
        }
    }
    exe::crt_remove(LOAD_DB_DUFF_OUT as *const u8);
    let table = read_usize(G_STAFF_TABLE);
    let mut k = 0;
    while k < read_i32(G_STAFF_COUNT) {
        let p = table + k as usize * staff::SIZE;
        if read_i32(p) < read_i32(G_STAFF_COUNT) - 0x10 && read_u8(p + staff::CLASSIFICATION) != 6 {
            load_db_fix_person(p, zip);
        }
        k += 1;
    }
    if extension == 0 {
        load_db_fix_clubs();
    }
    let set_key_ids: unsafe extern "C" fn() -> u32 =
        core::mem::transmute(LOAD_DB_SET_KEYS_NEXT.wrapping_add(exe_immediate(LOAD_DB_SET_KEYS_CALL as *const i32) as usize));
    set_key_ids();
    1
}

// ---------- r7_02_1: the comparer ----------

/// The tables of the loaded database: (global table pointer, global count, record size). The national teams, the
/// nation competitions and their history (inside other tables in the linked layout) are listed on their own too.
const LOAD_DB_PHOTO_TABLES: [(usize, usize, usize); 22] = [
    (G_CONTINENT_TABLE, G_CONTINENT_COUNT, 0xc6), (G_NATION_TABLE, G_NATION_COUNT, nation::SIZE),
    (G_CITY_TABLE, G_CITY_COUNT, 0x38), (G_STADIUM_TABLE, G_STADIUM_COUNT, 0x4e), (G_CLUB_TABLE, G_CLUB_COUNT, club::SIZE),
    (G_NAT_CLUB_TABLE, LOAD_DB_NAT_CLUB_COUNT, club::SIZE), (G_STAFF_TABLE, G_STAFF_COUNT, staff::SIZE),
    (G_NON_PLAYER_TABLE, G_NON_PLAYER_COUNT, 0x44), (G_PLAYER_DATA_TABLE, G_PLAYER_DATA_COUNT, 0x46),
    (G_PREFERENCES_TABLE, LOAD_DB_PREFERENCES_COUNT, 0x34), (G_STAFF_HISTORY_TABLE, G_STAFF_HISTORY_COUNT, 0x11),
    (G_CLUB_COMP_TABLE, G_CLUB_COMP_COUNT, 0x6b), (LOAD_DB_NATION_COMP_TABLE, LOAD_DB_NATION_COMP_COUNT, 0x6b),
    (G_CLUB_COMP_HISTORY_TABLE, G_CLUB_COMP_HISTORY_COUNT, 0x1a), (G_NATION_COMP_HISTORY_TABLE, G_NATION_COMP_HISTORY_COUNT, 0x1a),
    (G_STAFF_COMP_TABLE, LOAD_DB_STAFF_COMP_COUNT, 0x65), (G_STAFF_COMP_HISTORY_TABLE, G_STAFF_COMP_HISTORY_COUNT, 0x3a),
    (G_OFFICIAL_TABLE, G_OFFICIAL_COUNT, 0x2b), (G_COLOUR_TABLE, LOAD_DB_COLOUR_COUNT, 0x3a),
    (G_FIRST_NAME_TABLE, LOAD_DB_FIRST_NAME_COUNT, 0x3c), (G_SECOND_NAME_TABLE, LOAD_DB_SECOND_NAME_COUNT, 0x3c),
    (G_COMMON_NAME_TABLE, G_COMMON_NAME_COUNT, 0x3c),
];

/// The bytes the original sets in a reserve competition row and in a spare name row (the rest is heap leftovers).
const LOAD_DB_RESERVE_COMP_SET: [(usize, usize); 6] = [(0, 5), (competition::SHORT_NAME, 1), (0x53, 1), (0x57, 2),
    (competition::CONTINENT, 0x10), (competition::REPUTATION, 2)];
const LOAD_DB_SPARE_NAME_SET: [(usize, usize); 2] = [(0, 1), (names::ID, 9)];

/// What the load leaves: each table, the clubs' English name index, the count and table globals, index.dat's entries,
/// the names language, the empty common name and the warning buffer; with the address ranges of the tables, for the
/// pointers. The bytes the original never sets are zeroed: the reserve competition rows and the spare name rows of
/// the linked layout but their set fields, and `staff::SQUAD_SELECTED_FOR` of the people converted from version 1.
struct LoadDbPhoto {
    parts: Vec<(usize, Vec<u8>)>,
    ranges: Vec<(usize, usize)>,
}

unsafe fn load_db_photo() -> LoadDbPhoto {
    let bytes = |a: usize, n: usize| if a == 0 || n == 0 { Vec::new() } else { core::slice::from_raw_parts(a as *const u8, n).to_vec() };
    let mut parts = Vec::new();
    let mut ranges = Vec::new();
    for &(t, c, size) in LOAD_DB_PHOTO_TABLES.iter() {
        let base = read_usize(t);
        let len = if base == 0 { 0 } else { read_i32(c).max(0) as usize * size };
        ranges.push((base, len));
        parts.push((t, bytes(base, len)));
    }
    let index = read_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE);
    let len = LOAD_DB_LAST_NAME_INDEX.load(Relaxed) * 4;
    ranges.push((index, if index == 0 { 0 } else { len }));
    parts.push((G_CLUB_ENGLISH_NAME_INDEX_TABLE, bytes(index, len)));
    let mut globals = bytes(G_DATABASE_SHARED_ALLOC_MODE, 0xcc);
    let names_at = G_ENGLISH_NAME_TABLES - G_DATABASE_SHARED_ALLOC_MODE;
    globals[names_at..names_at + 4].copy_from_slice(&((read_usize(G_ENGLISH_NAME_TABLES) != 0) as u32).to_le_bytes());
    parts.push((G_DATABASE_SHARED_ALLOC_MODE, globals));
    parts.push((LOAD_DB_INDEX, bytes(LOAD_DB_INDEX, LOAD_DB_INDEX_SLOTS * LOAD_DB_ENTRY_SIZE)));
    parts.push((G_DATABASE_NAMES_LANGUAGE, bytes(G_DATABASE_NAMES_LANGUAGE, 4)));
    parts.push((G_EMPTY_COMMON_NAME_INDEX, bytes(G_EMPTY_COMMON_NAME_INDEX, 4)));
    parts.push((LOAD_DB_LOG_LINE, bytes(LOAD_DB_LOG_LINE, 0x80)));
    let mut photo = LoadDbPhoto { parts, ranges };
    let keep = |photo: &mut LoadDbPhoto, table: usize, size: usize, first: usize, rows: usize, set: &[(usize, usize)]| {
        let mut mask = vec![false; size];
        for &(o, n) in set {
            mask[o..o + n].iter_mut().for_each(|m| *m = true);
        }
        if let Some((_, v)) = photo.parts.iter_mut().find(|(t, _)| *t == table) {
            for r in first..(first + rows).min(v.len() / size) {
                let row = &mut v[r * size..(r + 1) * size];
                row.iter_mut().zip(mask.iter()).filter(|(_, m)| !**m).for_each(|(b, _)| *b = 0);
            }
        }
    };
    if read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0 {
        let comps = read_i32(G_CLUB_COMP_COUNT).max(0) as usize;
        let reserve = (LOAD_DB_RESERVE_COMPS as usize).min(comps);
        keep(&mut photo, G_CLUB_COMP_TABLE, 0x6b, comps - reserve, reserve, &LOAD_DB_RESERVE_COMP_SET);
        let spare = LOAD_DB_LAST_SPARE.load(Relaxed);
        for (table, count) in [(G_FIRST_NAME_TABLE, LOAD_DB_FIRST_NAME_COUNT), (G_SECOND_NAME_TABLE, LOAD_DB_SECOND_NAME_COUNT), (G_COMMON_NAME_TABLE, G_COMMON_NAME_COUNT)] {
            let n = read_i32(count).max(0) as usize;
            keep(&mut photo, table, 0x3c, n - spare.min(n), spare.min(n), &LOAD_DB_SPARE_NAME_SET);
        }
    }
    let converted = LOAD_DB_LAST_V1_ROWS.load(Relaxed);
    let all_but_squad: [(usize, usize); 1] = [(0, staff::SQUAD_SELECTED_FOR)];
    keep(&mut photo, G_STAFF_TABLE, staff::SIZE, 0, converted, &all_but_squad);
    photo
}

/// The table and offset a pointer falls in (the end of a table counts), by the photo's ranges.
fn load_db_locate(p: u32, ranges: &[(usize, usize)]) -> Option<(usize, usize)> {
    let p = p as usize;
    ranges.iter().enumerate().find(|(_, &(b, n))| b != 0 && p >= b && p <= b + n).map(|(k, &(b, _))| (k, p - b))
}

/// The first difference between the two photos that is not the same pointer (the same table and offset) in both.
fn load_db_first_difference(a: &LoadDbPhoto, b: &LoadDbPhoto) -> Option<String> {
    let word = |v: &[u8], s: usize| u32::from_le_bytes([v[s], v[s + 1], v[s + 2], v[s + 3]]);
    for ((label, x), (_, y)) in a.parts.iter().zip(b.parts.iter()) {
        if x.len() != y.len() {
            return Some(format!("{:#x}: {} x {} bytes", label, x.len(), y.len()));
        }
        let mut i = 0;
        while i < x.len() {
            if x[i] == y[i] {
                i += 1;
                continue;
            }
            let same_pointer = (i.saturating_sub(3)..=i).filter(|&s| s + 4 <= x.len()).find(|&s| {
                matches!((load_db_locate(word(x, s), &a.ranges), load_db_locate(word(y, s), &b.ranges)), (Some(m), Some(n)) if m == n)
            });
            match same_pointer {
                Some(s) => i = s + 4,
                None => return Some(format!("{:#x}+{:#x}: original={:02x} rust={:02x}", label, i, x[i], y[i])),
            }
        }
    }
    None
}

/// Comparer: the copy loads first (its own free of every table first) and the photo is taken; the draws and the empty
/// common name go back to before; then the original loads over it (freeing the copy's tables) and its photo is
/// compared, the pointers by the table and offset they point to. The original's database stays.
pub(crate) unsafe extern "C" fn compare_load_database(source: usize, shared: u32, from_save: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I0%%);
    let before = crate::comparison::MemorySnapshot::read(&[(0xde1f5c, 8), (0xad6c30, 4), (G_EMPTY_COMMON_NAME_INDEX, 4)]);
    let n = crate::rng::pause_trace(|| load_database(source, shared, from_save));
    let rust = load_db_photo();
    before.write_back();
    let r = o(source, shared, from_save);
    let original = load_db_photo();
    let difference = load_db_first_difference(&original, &rust);
    SCOREBOARD.record(r == n && difference.is_none(), || {
        format!("load_database(shared {}, save {}) original={} rust={} {}", shared, from_save, r, n, difference.clone().unwrap_or_default())
    });
    r
}
