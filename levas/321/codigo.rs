// ---------- tel41_5: the competition statistics in the save: writing them (comp_screens.cpp) ----------

/// One write of `Comp_Stats::save`: `n` records of `size` bytes from `at`; short: the comp_stats.CPP box at `line`,
/// `game_fatal_cleanup` and `exit(-1)` (`comp_stats_fatal`).
unsafe fn comp_stats_write(at: usize, size: u32, n: u32, file: usize, line: u32) {
    if exe::crt_fwrite(at, size, n, file) != n {
        comp_stats_fatal(line);
    }
}

/// Rule: a competition's statistics are written to the competition's temporary file in a fixed order: the club table,
/// the player table, the late block, both record sets, the archive keys, the season end count, the record flags and
/// the saved runs.
///
/// `Comp_Stats::save(file)`: no file: the comp_stats.CPP 6177 box and 0. Then, each write checked (short: the
/// comp_stats.CPP box of its line and the game stops): the club capacity (+0x10) and count (+0x14), 4 bytes each; with
/// a club table, count entries of 0x76 bytes; the player capacity (+0x1d) and count (+0x21); with a player table,
/// count entries of 0x2d bytes; the block +0x56a (0x64 bytes); the all-time records (+0x26) and the season records
/// (+0x28f), 0x269 bytes each; the archive keys +0x4f8 and +0x4fc; the season end count (word +0x500); the block
/// +0x506 (0x5c bytes, the 22 record flags among them) and the "any flag" dword +0x562. Then the saved runs: with a
/// count (+8) of 0 and an array (+0), the array is freed and pointer and capacity cleared first; the capacity (+4) and
/// count (+8), and with a capacity and an array, capacity records of 0x1c bytes. Returns 1.
/// Original: `Comp_Stats::save` (0x4ac4e0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_save(stats: usize, file: usize) -> u32 {
    if file == 0 {
        show_error_box(COMP_STATS_CPP, 0x1821);
        return 0;
    }
    comp_stats_write(stats + comp_stats_obj::CLUB_CAPACITY, 4, 1, file, 0x1828);
    comp_stats_write(stats + comp_stats_obj::CLUB_COUNT, 4, 1, file, 0x182f);
    if read_usize(stats + comp_stats_obj::CLUBS) != 0 {
        // tel41_5: the club entries
        comp_stats_write(read_usize(stats + comp_stats_obj::CLUBS), comp_stats_obj::CLUB_ENTRY_SIZE as u32, read_u32(stats + comp_stats_obj::CLUB_COUNT), file, 0x1839);
    }
    comp_stats_write(stats + comp_stats_obj::PLAYER_CAPACITY, 4, 1, file, 0x1841);
    comp_stats_write(stats + comp_stats_obj::PLAYER_COUNT, 4, 1, file, 0x1848);
    if read_usize(stats + comp_stats_obj::PLAYERS) != 0 {
        // tel41_5: the player entries
        comp_stats_write(read_usize(stats + comp_stats_obj::PLAYERS), comp_stats_obj::PLAYER_ENTRY_SIZE as u32, read_u32(stats + comp_stats_obj::PLAYER_COUNT), file, 0x1852);
    }
    comp_stats_write(stats + 0x56a, 0x64, 1, file, 0x185f);
    comp_stats_write(stats + comp_stats_obj::ALL_TIME_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x1866);
    comp_stats_write(stats + comp_stats_obj::SEASON_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x186d); // tel41_5: the season records
    comp_stats_write(stats + comp_stats_obj::ARCHIVE_KEY1, 4, 1, file, 0x1874);
    comp_stats_write(stats + comp_stats_obj::ARCHIVE_KEY2, 4, 1, file, 0x187b);
    comp_stats_write(stats + 0x500, 2, 1, file, 0x1882);
    comp_stats_write(stats + 0x506, 0x5c, 1, file, 0x1889);
    comp_stats_write(stats + 0x562, 4, 1, file, 0x1890);
    if read_u32(stats + comp_stats_runs::SAVED_COUNT) == 0 && read_usize(stats + comp_stats_runs::SAVED) != 0 {
        covered!("tel41_5 empty saved runs freed");
        exe::crt_free(read_usize(stats + comp_stats_runs::SAVED));
        write_usize(stats + comp_stats_runs::SAVED, 0);
        write_u32(stats + comp_stats_runs::SAVED_CAPACITY, 0);
    }
    comp_stats_write(stats + comp_stats_runs::SAVED_CAPACITY, 4, 1, file, 0x18a0);
    comp_stats_write(stats + comp_stats_runs::SAVED_COUNT, 4, 1, file, 0x18a7);
    let capacity = read_u32(stats + comp_stats_runs::SAVED_CAPACITY);
    if capacity != 0 && read_usize(stats + comp_stats_runs::SAVED) != 0 {
        // the original writes `capacity` records (save_team_runs leaves capacity = count)
        comp_stats_write(read_usize(stats + comp_stats_runs::SAVED), comp_stats_runs::SAVED_SIZE as u32, capacity, file, 0x18b1);
    }
    1
}

/// Comparer for `comp_stats_save`: it writes the competition's temporary file (it cannot run twice), so only the
/// original runs; the copy is proven by the save proof (the .tmp goes into the .qst).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_save(stats: usize, file: usize) -> u32 {
    // ponytail: writes a file; the save proof compares the .qst written by the original and by the copy
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_COMP_STATS_SAVE)(stats, file)
}

// ---------- tel41_6: the competition statistics in the save: reading them (comp_screens.cpp) ----------

/// One read of `Comp_Stats::load`: `n` records of `size` bytes into `at`; short: the comp_stats.CPP box at `line`,
/// `game_fatal_cleanup` and `exit(-1)` (`comp_stats_fatal`).
unsafe fn comp_stats_read(at: usize, size: u32, n: u32, file: usize, line: u32) {
    if exe::crt_fread(at, size, n, file) != n {
        comp_stats_fatal(line);
    }
}

/// A table allocation of `Comp_Stats::load`: `crt_malloc`; failing, the comp_stats.CPP box at `line` and the game
/// stops.
unsafe fn comp_stats_table(size: u32, line: u32) -> usize {
    let block = exe::crt_malloc(size);
    if block == 0 {
        comp_stats_fatal(line);
    }
    block
}

/// The data versions of the save that changed the statistics: from 0x3e015 a player entry is read whole (0x2d bytes;
/// before, 0x1d bytes and the six words +0x1d..+0x27 zeroed), and above 0x3e007 the block +0x56a is in the file.
const COMP_STATS_WHOLE_PLAYER_VERSION: i32 = 0x3e015;
const COMP_STATS_LATE_BLOCK_VERSION: i32 = 0x3e007;
/// The bytes of a player entry in the files before `COMP_STATS_WHOLE_PLAYER_VERSION`.
const COMP_STATS_OLD_PLAYER_BYTES: u32 = 0x1d;

/// Rule: a competition's statistics are read back from the competition's temporary file in the order `save` wrote
/// them; files of older data versions have shorter player entries and no late block.
///
/// `Comp_Stats::load(file)`: no file: the comp_stats.CPP 5975 box and 0. Then, each read checked (short or a failed
/// allocation: the comp_stats.CPP box of its line and the game stops): the club capacity (+0x10) and count (+0x14);
/// with a capacity, a new club table of capacity × 0x76 bytes (`crt_malloc`, the old one not freed) and count entries.
/// The player capacity (+0x1d) and count (+0x21); with a capacity, a new player table of capacity × 0x2d bytes and,
/// when the data version (`G_GAME_DATA_VERSION`, absolute value) is below 0x3e015, count entries read one by one as
/// 0x1d bytes with the six words +0x1d..+0x27 zeroed, otherwise count entries of 0x2d bytes. With a version above
/// 0x3e007, the block +0x56a (0x64 bytes). The all-time (+0x26) and season (+0x28f) records, the archive keys
/// +0x4f8/+0x4fc, the season end count (+0x500), the block +0x506 (0x5c) and the "any flag" dword +0x562. The saved
/// runs: capacity (+4) and count (+8); with a capacity, a new array of capacity × 0x1c bytes (+0) and capacity
/// records, otherwise +0 = 0. The sort bytes are left as they were. Returns 1.
/// Original: `Comp_Stats::load` (0x4ab890, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_load(stats: usize, file: usize) -> u32 {
    if file == 0 {
        show_error_box(COMP_STATS_CPP, 0x1757);
        return 0;
    }
    comp_stats_read(stats + comp_stats_obj::CLUB_CAPACITY, 4, 1, file, 0x175e);
    comp_stats_read(stats + comp_stats_obj::CLUB_COUNT, 4, 1, file, 0x1765);
    let capacity = read_u32(stats + comp_stats_obj::CLUB_CAPACITY);
    if capacity != 0 {
        let table = comp_stats_table(capacity.wrapping_mul(comp_stats_obj::CLUB_ENTRY_SIZE as u32), 0x1770);
        write_usize(stats + comp_stats_obj::CLUBS, table);
        comp_stats_read(table, comp_stats_obj::CLUB_ENTRY_SIZE as u32, read_u32(stats + comp_stats_obj::CLUB_COUNT), file, 0x1777); // tel41_6: club entries read
    }
    comp_stats_read(stats + comp_stats_obj::PLAYER_CAPACITY, 4, 1, file, 0x177f);
    comp_stats_read(stats + comp_stats_obj::PLAYER_COUNT, 4, 1, file, 0x1786);
    let version = read_i32(G_GAME_DATA_VERSION).wrapping_abs();
    let capacity = read_u32(stats + comp_stats_obj::PLAYER_CAPACITY);
    if capacity != 0 {
        let table = comp_stats_table(capacity.wrapping_mul(comp_stats_obj::PLAYER_ENTRY_SIZE as u32), 0x1791);
        write_usize(stats + comp_stats_obj::PLAYERS, table);
        if version < COMP_STATS_WHOLE_PLAYER_VERSION {
            covered!("tel41_6 old player entries");
            let mut i = 0;
            while i < read_i32(stats + comp_stats_obj::PLAYER_COUNT) {
                let entry = read_usize(stats + comp_stats_obj::PLAYERS) + i as usize * comp_stats_obj::PLAYER_ENTRY_SIZE;
                comp_stats_read(entry, COMP_STATS_OLD_PLAYER_BYTES, 1, file, 0x179c);
                for o in (0x1d..=0x27).step_by(2) {
                    write_u16(read_usize(stats + comp_stats_obj::PLAYERS) + i as usize * comp_stats_obj::PLAYER_ENTRY_SIZE + o, 0);
                }
                i += 1;
            }
        } else {
            // tel41_6: the player entries
            comp_stats_read(table, comp_stats_obj::PLAYER_ENTRY_SIZE as u32, read_u32(stats + comp_stats_obj::PLAYER_COUNT), file, 0x17ac);
        }
    }
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() > COMP_STATS_LATE_BLOCK_VERSION {
        comp_stats_read(stats + 0x56a, 0x64, 1, file, 0x17b7);
    }
    comp_stats_read(stats + comp_stats_obj::ALL_TIME_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x17bf);
    comp_stats_read(stats + comp_stats_obj::SEASON_RECORDS, comp_stats_obj::SEASON_RECORDS_SIZE as u32, 1, file, 0x17c6); // tel41_6: the season records
    comp_stats_read(stats + comp_stats_obj::ARCHIVE_KEY1, 4, 1, file, 0x17cd);
    comp_stats_read(stats + comp_stats_obj::ARCHIVE_KEY2, 4, 1, file, 0x17d4);
    comp_stats_read(stats + 0x500, 2, 1, file, 0x17db);
    comp_stats_read(stats + 0x506, 0x5c, 1, file, 0x17e2);
    comp_stats_read(stats + 0x562, 4, 1, file, 0x17e9);
    comp_stats_read(stats + comp_stats_runs::SAVED_CAPACITY, 4, 1, file, 0x17f0);
    comp_stats_read(stats + comp_stats_runs::SAVED_COUNT, 4, 1, file, 0x17f7);
    let capacity = read_u32(stats + comp_stats_runs::SAVED_CAPACITY);
    if capacity != 0 {
        let saved = comp_stats_table(capacity.wrapping_mul(comp_stats_runs::SAVED_SIZE as u32), 0x1802);
        write_usize(stats + comp_stats_runs::SAVED, saved);
        comp_stats_read(saved, comp_stats_runs::SAVED_SIZE as u32, capacity, file, 0x1809);
    } else {
        write_usize(stats + comp_stats_runs::SAVED, 0);
    }
    1
}

/// Comparer for `comp_stats_load`: it reads the open file (it cannot read it twice) and allocates the tables, so only
/// the original runs; the copy is proven by the save proof (the holiday save loaded with and without the routine,
/// CARGA).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_load(stats: usize, file: usize) -> u32 {
    // ponytail: reads a file; the save proof (CARGA) loads the same save with and without the copy
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_COMP_STATS_LOAD)(stats, file)
}

// ---------- batch 321: the club history store: building it, destroying it and loading it (club_history.cpp) ----------

/// Constants of the club history store (`G_CLUB_HISTORY`).
pub(crate) mod club_history_consts {
    /// The object: +0 the league history file list, +4 the cup history file list, +8 the per-club records
    /// (`RECORD_SIZE` bytes each), +0xc their count (`club_history_fields::RECORD_COUNT`, the club count).
    pub(crate) const LEAGUE_LIST: usize = 0x0;
    pub(crate) const CUP_LIST: usize = 0x4;
    pub(crate) const RECORDS: usize = 0x8;
    /// A club's record: +0/+4 the first and last file offsets of its league seasons, +8/+0xc those of its cup runs
    /// (-1 = none), the club id is the index.
    pub(crate) const RECORD_SIZE: usize = 0x10;
    pub(crate) const LEAGUE_FIRST: usize = 0x0;
    pub(crate) const LEAGUE_LAST: usize = 0x4;
    pub(crate) const CUP_FIRST: usize = 0x8;
    pub(crate) const CUP_LAST: usize = 0xc;
    /// The size of a league season record (comp id, year, position and the 14 bytes of the table totals) and of a cup
    /// run record (comp id, year, status, stage label, round code), as the file lists are built with.
    pub(crate) const LEAGUE_ENTRY_SIZE: u32 = 0x15;
    pub(crate) const CUP_ENTRY_SIZE: u32 = 0xb;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\club_history.cpp" (the error boxes).
    pub(crate) const CLUB_HISTORY_CPP: usize = 0x98b644;
    /// "club league history.tmp", "club cup history.tmp", "club history.dat" and "CM3_TEMP".
    pub(crate) const LEAGUE_TMP: usize = 0x98b674;
    pub(crate) const CUP_TMP: usize = 0x98b62c;
    pub(crate) const SAVE_ITEM: usize = 0x98b68c;
    pub(crate) const TEMP_FOLDER: usize = 0x987da0;
    /// `FILE_LLIST::FILE_LLIST(path, record size)` (thiscall, ret 8) and `FILE_LLIST::~FILE_LLIST` (thiscall).
    pub(crate) const FILE_LLIST_CONSTRUCTOR: usize = 0x58e280;
    pub(crate) const FILE_LLIST_DESTRUCTOR: usize = 0x58e330;
    /// The two bytes a no-CD exe patches in this routine's CD search (as `cm0102-1080`): the drive type's `jne` and the
    /// found test.
    pub(crate) const CD_DRIVE_TYPE_JUMP: usize = 0x442b0c;
    pub(crate) const CD_FOUND_TEST: usize = 0x442b4f;
}

/// The CD search at the start of the club history constructor: each CD drive (`GetLogicalDriveStringsA`,
/// `GetDriveTypeA` = 5) is looked at for "<root>cm0102.exe" (`file_find_first`) until one has it, with the
/// "Insert CD" box (OK) in between. A no-CD exe patched in this routine (as `cm0102-1080`: the drive type's `jne` at
/// 0x442b0c made `nop`, the found test at 0x442b4f made `xor`) is honoured by reading those bytes.
unsafe fn club_history_find_cd() {
    use club_history_consts as ch;
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = (ch::CD_DRIVE_TYPE_JUMP as *const u8).read() == 0x90;
    let always_found = (ch::CD_FOUND_TEST as *const u8).read() != 0x85;
    loop {
        // simplification: the buffer starts zeroed (as check_game_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
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
        covered!("tel42_1 club history insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// A history file list in the temporary folder: `operator new(8)` and `FILE_LLIST::FILE_LLIST("CM3_TEMP\<name>",
/// record size)`; a failed allocation gives 0.
unsafe fn club_history_new_file_list(name: usize, record_size: u32) -> usize {
    let construct: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(club_history_consts::FILE_LLIST_CONSTRUCTOR);
    let list = exe::crt_operator_new(8);
    if list == 0 {
        return 0;
    }
    construct(list, exe::utils_build_file_path(club_history_consts::TEMP_FOLDER, name), record_size)
}

/// Builds the club history store: the CD check, the league and cup history file lists, then the per-club records from
/// the saved game or new ones.
///
/// `Club_History::Club_History(save)`: the CD search (`club_history_find_cd`). The league history list (`LEAGUE_LIST`,
/// "CM3_TEMP\club league history.tmp", records of `LEAGUE_ENTRY_SIZE` bytes) and the cup history list (`CUP_LIST`,
/// "CM3_TEMP\club cup history.tmp", `CUP_ENTRY_SIZE` bytes); either one missing stops the game (club_history.cpp
/// 0x41 / 0x49). With a save, `club_history_load` reads the records and, when it answers, that is all. Otherwise the
/// count (`club_history_fields::RECORD_COUNT`) is the club count (`G_CLUB_COUNT`), the records (`RECORDS`, 16 bytes
/// each) are allocated (`malloc`; none stops the game, line 0x5e) and every offset becomes -1 (no season, no cup run).
/// Returns the object.
/// Original: `Club_History::Club_History` (0x442ab0, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_constructor(this: usize, save: usize) -> usize {
    use club_history_consts as ch;
    club_history_find_cd();
    let league = club_history_new_file_list(ch::LEAGUE_TMP, ch::LEAGUE_ENTRY_SIZE);
    write_usize(this + ch::LEAGUE_LIST, league);
    if league == 0 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x41);
    }
    let cup = club_history_new_file_list(ch::CUP_TMP, ch::CUP_ENTRY_SIZE);
    write_usize(this + ch::CUP_LIST, cup);
    if cup == 0 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x49);
    }
    if save != 0 && club_history_load(this, save) != 0 {
        covered!("tel42_1 club history from the save");
        return this;
    }
    covered!("tel42_1 new club history");
    let count = read_u32(G_CLUB_COUNT);
    write_u32(this + club_history_fields::RECORD_COUNT, count);
    let records = exe::crt_malloc(count << 4);
    write_usize(this + ch::RECORDS, records);
    if records == 0 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x5e);
    }
    for k in 0..read_i32(this + club_history_fields::RECORD_COUNT).max(0) as usize {
        let r = records + k * ch::RECORD_SIZE;
        for field in [ch::LEAGUE_FIRST, ch::LEAGUE_LAST, ch::CUP_FIRST, ch::CUP_LAST] {
            write_u32(r + field, u32::MAX); // tel42_1: no season yet
        }
    }
    this
}

/// Comparer for `club_history_constructor`.
///
/// It searches the CD, builds the two file lists and allocates (or loads) the records: it cannot run twice. Only the
/// original runs; the copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_club_history_constructor(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_CONSTRUCTOR)(this, save)
}

/// Destroys the club history store: both history file lists and the per-club records.
///
/// `Club_History::~Club_History()`: each file list (`LEAGUE_LIST`, `CUP_LIST`) present is destroyed
/// (`FILE_LLIST::~FILE_LLIST`, then `operator delete`) and zeroed; the records (`RECORDS`) present are freed (`free`)
/// and the pointer and the count (`club_history_fields::RECORD_COUNT`) zeroed.
/// Original: `Club_History::~Club_History` (0x442e20, club_history.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn club_history_destructor(this: usize) {
    use club_history_consts as ch;
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(ch::FILE_LLIST_DESTRUCTOR);
    for field in [ch::LEAGUE_LIST, ch::CUP_LIST] {
        let list = read_usize(this + field);
        if list != 0 {
            destroy(list);
            exe::crt_operator_delete(list);
            write_usize(this + field, 0);
        }
    }
    let records = read_usize(this + ch::RECORDS);
    if records != 0 {
        exe::crt_free(records);
        write_usize(this + ch::RECORDS, 0);
        write_u32(this + club_history_fields::RECORD_COUNT, 0);
    }
}

/// Comparer for `club_history_destructor`.
///
/// It frees the lists and the records: it cannot run twice. Only the original runs; the copy is proven by loading the
/// holiday save (free_game_data destroys the store first).
pub(crate) unsafe extern "thiscall" fn compare_club_history_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CLUB_HISTORY_DESTRUCTOR)(this)
}

/// Reads the club history records from the saved game.
///
/// `Club_History::load(save)`: no save: the club_history.cpp 0x2ee box and 0. The "club history.dat" item of the save
/// (`zopen` with 0, 0x16, 4; missing stops the game, line 0x2f6); the count (`club_history_fields::RECORD_COUNT`, 4
/// bytes; short read stops the game, 0x2fd); the records (`RECORDS`, `malloc` of count × 16; none stops the game,
/// 0x306) read in one go (count records of 16 bytes; fewer stops the game, 0x30e); the item closed. Returns 1.
/// Original: `Club_History::load` (0x443cd0, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_load(this: usize, save: usize) -> u32 {
    use club_history_consts as ch;
    if save == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x2ee);
        return 0;
    }
    let mut z = exe::zopen(save, ch::SAVE_ITEM, 0, 0x16, 4);
    if z == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x2f6);
        return 0;
    }
    covered!("tel42_1 load club history");
    let count = this + club_history_fields::RECORD_COUNT;
    if exe::zread(count, 4, 1, z) != 1 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x2fd);
    }
    let records = exe::crt_malloc(read_u32(count) << 4);
    write_usize(this + ch::RECORDS, records);
    if records == 0 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x306);
    }
    if exe::zread(records, ch::RECORD_SIZE as u32, read_u32(count), z) != read_u32(count) { // tel42_1: every record read
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x30e);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `club_history_load`.
///
/// It allocates the records and reads the save item: it cannot run twice. Only the original runs; the copy is proven by
/// loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_club_history_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLUB_HISTORY_LOAD)(this, save)
}
