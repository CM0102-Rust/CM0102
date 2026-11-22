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
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(stats, file)
}
