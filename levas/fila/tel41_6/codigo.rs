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
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(stats, file)
}
