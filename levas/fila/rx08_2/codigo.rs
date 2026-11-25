// ---------- rx08_2: the club records written to the saved game (club_records.cpp) ----------

/// Writes `n` items of `size` bytes to an open save item; a short write shows the error box of `file` (the source
/// file's path) at `row` and stops the game.
pub(crate) unsafe fn zwrite_or_stop(file: usize, row: u32, z: usize, data: usize, size: u32, n: u32) {
    if exe::zwrite(data, size, n, z) != n {
        fatal_error_at(file, row);
    }
}

/// `zwrite_or_stop` with club_records.cpp's boxes.
unsafe fn club_records_write_or_stop(z: usize, data: usize, size: u32, n: u32, row: u32) {
    zwrite_or_stop(club_records_consts::CLUB_RECORDS_CPP, row, z, data, size, n);
}

/// Writes one pool of the club records object: its capacity and its count (dwords), then, when the count is positive,
/// the count's records of `size` bytes (the three rows are the boxes of the three writes).
unsafe fn club_records_write_pool(this: usize, z: usize, (pointer, capacity, count): (usize, usize, usize), size: u32, rows: [u32; 3]) {
    club_records_write_or_stop(z, this + capacity, 4, 1, rows[0]);
    club_records_write_or_stop(z, this + count, 4, 1, rows[1]);
    let n = read_i32(this + count);
    if n > 0 {
        club_records_write_or_stop(z, read_usize(this + pointer), size, n as u32, rows[2]);
    }
}

/// Writes the club records into the saved game's "club records.dat" item.
///
/// `Club_Records::save(save)`: no save: the club_records.cpp 0x15fa box and 0. The item opened for writing (`zopen`
/// with 1, 0x16, 4; failing stops the game, line 0x1602). Then, each short write stopping the game: the all-time pool
/// (`club_records_write_pool`: capacity, count and, with a positive count, the 0x38b-byte records; rows 0x1609,
/// 0x1611, 0x161c), the season pool (0x1625, 0x162d, 0x1638), the entry count (0x1641) and the 0x6a-byte entries,
/// written whatever the count (0x1649), the news queue (capacity, count and the 6-byte items; 0x1651, 0x1659, 0x1664).
/// The item closed; 1. The layout is the one `club_records_load` reads.
/// Original: `Club_Records::save` (0x451ff0, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_records_save(this: usize, save: usize) -> u32 {
    use club_records_consts as c;
    use club_records_fields as f;
    if save == 0 {
        show_error_at(c::CLUB_RECORDS_CPP, 0x15fa);
        return 0;
    }
    let mut z = zip_open_item(save, c::SAVE_ITEM);
    if z == 0 {
        fatal_error_at(c::CLUB_RECORDS_CPP, 0x1602);
    }
    covered!("rx08_2 save club records");
    club_records_write_pool(this, z, (f::ALL_TIME_RECORDS, f::ALL_TIME_CAPACITY, f::ALL_TIME_COUNT), c::RECORD_SIZE, [0x1609, 0x1611, 0x161c]);
    club_records_write_pool(this, z, (f::SEASON_RECORDS, f::SEASON_CAPACITY, f::SEASON_COUNT), c::RECORD_SIZE, [0x1625, 0x162d, 0x1638]);
    club_records_write_or_stop(z, this + c::CLUB_ENTRY_COUNT, 4, 1, 0x1641);
    let entries = read_u32(this + c::CLUB_ENTRY_COUNT); // rx08_2: every club entry
    club_records_write_or_stop(z, read_usize(this + f::CLUB_ENTRIES), c::ENTRY_SIZE, entries, 0x1649);
    club_records_write_pool(this, z, (f::NEWS_QUEUE, f::NEWS_QUEUE_CAPACITY, f::NEWS_QUEUE_COUNT), c::NEWS_SIZE, [0x1651, 0x1659, 0x1664]);
    zip_close(&mut z);
    1
}

/// Comparer for `club_records_save`: it writes to the save container and cannot run twice; only the original runs. The
/// copy is proven by the save proof (the .qst written by the original and by the copy, item by item).
pub(crate) unsafe extern "thiscall" fn compare_club_records_save(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}

// ---------- rx08_2: the player statistics written to the saved game (player_stats.cpp) ----------

/// The player statistics' source file and save item ("E:\dev\CM3\cm3 00-01\cm3\code\player_stats.cpp" and "player
/// stats.dat"), the entry count after `player_stats_fields::PLAYER_ENTRIES` and the sizes of the three arrays.
pub(crate) mod player_stats_save_consts {
    pub(crate) const PLAYER_STATS_CPP: usize = 0xa78310;
    pub(crate) const SAVE_ITEM: usize = 0xa784e8;
    pub(crate) const ENTRY_COUNT: usize = 0x34;
    pub(crate) const CATEGORY_SIZE: u32 = 0x20;
    pub(crate) const ENTRY_SIZE: u32 = 0x27;
    pub(crate) const LANGUAGE_SIZE: u32 = 6;
}

/// Writes the player statistics into the saved game's "player stats.dat" item.
///
/// `PlayerStats::save_stats(save)`: no save: the player_stats.cpp 0xe09 box and 0. The item opened for writing
/// (`zip_open_item`; failing stops the game, line 0xe11). Then, each short write stopping the game: the category
/// capacity and count (0xe18, 0xe20) and the count's 0x20-byte category records (0xe28), the entry count (0xe30) and
/// its 0x27-byte per-player entries (0xe38), the four rating doubles (`RATING_SUM`, `RATED_COUNT`,
/// `FLAGGED_RATING_SUM`, `FLAGGED_RATED_COUNT`; 0xe40, 0xe48, 0xe50, 0xe58) and the free category count (0xe60). The
/// languages last, where a short write only shows the box (0xe68, 0xe6e, 0xe74), closes the item and answers 0: the
/// capacity, the count and as many 6-byte records as the capacity (not the count; `player_stats_load` reads the
/// same). The item closed; 1.
/// Original: `PlayerStats::save_stats` (0x7c55a0, player_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_stats_save(this: usize, save: usize) -> u32 {
    use player_stats_fields as f;
    use player_stats_save_consts as c;
    if save == 0 {
        show_error_at(c::PLAYER_STATS_CPP, 0xe09);
        return 0;
    }
    let mut z = zip_open_item(save, c::SAVE_ITEM);
    if z == 0 {
        fatal_error_at(c::PLAYER_STATS_CPP, 0xe11);
    }
    covered!("rx08_2 save player stats");
    let write = |data: usize, size: u32, n: u32, row: u32| zwrite_or_stop(c::PLAYER_STATS_CPP, row, z, data, size, n);
    write(this + f::CATEGORY_CAPACITY, 4, 1, 0xe18);
    write(this + f::CATEGORY_COUNT, 4, 1, 0xe20);
    write(read_usize(this + f::CATEGORY_RECORDS), c::CATEGORY_SIZE, read_u32(this + f::CATEGORY_COUNT), 0xe28);
    write(this + c::ENTRY_COUNT, 4, 1, 0xe30);
    let entries = read_u32(this + c::ENTRY_COUNT); // rx08_2: every player's entry
    write(read_usize(this + f::PLAYER_ENTRIES), c::ENTRY_SIZE, entries, 0xe38);
    for (field, row) in [(f::RATING_SUM, 0xe40), (f::RATED_COUNT, 0xe48), (f::FLAGGED_RATING_SUM, 0xe50), (f::FLAGGED_RATED_COUNT, 0xe58)] {
        write(this + field, 8, 1, row);
    }
    write(this + f::FREE_CATEGORY_COUNT, 4, 1, 0xe60);
    let languages = [(this + f::LANGUAGE_CAPACITY, 4, 1, 0xe68), (this + f::LANGUAGE_COUNT, 4, 1, 0xe6e),
        (read_usize(this + f::LANGUAGES), c::LANGUAGE_SIZE, read_u32(this + f::LANGUAGE_CAPACITY), 0xe74)];
    for (data, size, n, row) in languages {
        if exe::zwrite(data, size, n, z) != n {
            show_error_at(c::PLAYER_STATS_CPP, row);
            zip_close(&mut z);
            return 0;
        }
    }
    zip_close(&mut z);
    1
}

/// Comparer for `player_stats_save`: it writes to the save container and cannot run twice; only the original runs. The
/// copy is proven by the save proof (the .qst written by the original and by the copy, item by item).
pub(crate) unsafe extern "thiscall" fn compare_player_stats_save(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
}
