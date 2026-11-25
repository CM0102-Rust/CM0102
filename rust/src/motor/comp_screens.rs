use super::*;

/// The scalar deleting destructor of a linked list: `LLIST::~LLIST`, then operator delete when bit 0 of the flags is
/// set; returns the list.
/// Original: `LLIST::scalar_deleting_destructor` (0x497910, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn llist_scalar_deleting_destructor(this: usize, flags: u32) -> usize {
    covered!("tel39_2 list deleting destructor");
    exe::drop_llist(this);
    if flags & 1 != 0 {
        exe::crt_operator_delete(this);
    }
    this
}

/// Comparer for `llist_scalar_deleting_destructor`: only the original runs (freeing does not run twice).
pub(crate) unsafe extern "thiscall" fn compare_llist_scalar_deleting_destructor(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_LLIST_SCALAR_DELETING_DESTRUCTOR)(this, flags)
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

/// Copies a youngest or oldest player record (0x3f bytes): the age as a date, the holder, the holder's birth date and
/// the match it happened in.
///
/// `player_age_record_assign(this, source)`: the date at +0 (`copy_record_date`), the dwords +8, +0xc, +0x10 and
/// +0x14 [inference: the person, his club, the age in days and years], the date at +0x18, then the match record at
/// +0x20 field by field as `match_record_assign` copies it (the original has it inline). Returns `this`.
/// Original: `player_age_record_assign` (0x4a1fb0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_age_record_assign(this: usize, source: usize) -> usize {
    covered!("tel39_5 player age record copy");
    copy_record_date(this, source);
    for o in [8usize, 0xc, 0x10, 0x14] {
        write_u32(this + o, read_u32(source + o));
    }
    // tel39_5: the second date
    copy_record_date(this + 0x18, source + 0x18);
    match_record_assign(this + 0x20, source + 0x20);
    this
}

/// Copies an attendance record (the highest and lowest attendances): the attendance (dword) and its match record.
///
/// `attendance_record_assign(this, source)`: dword +0, then `match_record_assign(this + 4, source + 4)`. Returns
/// `this`.
/// Original: `attendance_record_assign` (0x4ab5b0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn attendance_record_assign(this: usize, source: usize) -> usize {
    covered!("tel39_5 attendance record copy");
    // tel39_5: the attendance
    write_u32(this, read_u32(source));
    match_record_assign(this + 4, source + 4);
    this
}

/// Whether two game dates are the same day: the year (word +2) and the day of the year (word +0) equal; the third
/// field (the leap-year flag at +4) is not compared.
///
/// `Date::operator==(this, other)`: `other` comes by value (8 bytes on the stack: `day_year` holds the day in its low
/// word and the year in its high word, `_leap` the flag). 1 when both words match, else 0; the year is compared first.
/// Original: `Date::operator==` (0x4a7f20, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn date_equals(this: usize, day_year: u32, _leap: u32) -> u32 {
    covered!("tel39_5 date equals");
    // tel39_5: the year, then the day
    ((day_year >> 16) as u16 == read_u16(this + 2) && day_year as u16 == read_u16(this)) as u32
}

/// Comparer for `player_age_record_assign`: side by side over the record and the return.
pub(crate) unsafe extern "thiscall" fn compare_player_age_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_AGE_RECORD_ASSIGN);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, PLAYER_AGE_RECORD_SIZE)], || o(this, source), || player_age_record_assign(this, source));
    SCOREBOARD.record(same && r == n, || format!("player_age_record_assign({:#x}, {:#x}) original {:x?} rust {:x?}", this, source, mo, mr));
    r
}

/// Comparer for `attendance_record_assign`: side by side over the record and the return.
pub(crate) unsafe extern "thiscall" fn compare_attendance_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_ATTENDANCE_RECORD_ASSIGN);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, ATTENDANCE_RECORD_SIZE)], || o(this, source), || attendance_record_assign(this, source));
    SCOREBOARD.record(same && r == n, || format!("attendance_record_assign({:#x}, {:#x}) original {:x?} rust {:x?}", this, source, mo, mr));
    r
}

/// Comparer for `date_equals`: the answer side by side (nothing is written).
pub(crate) unsafe extern "thiscall" fn compare_date_equals(this: usize, day_year: u32, leap: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_DATE_EQUALS)(this, day_year, leap);
    let n = date_equals(this, day_year, leap);
    SCOREBOARD.record(r == n, || format!("Date::operator==({:#x}: {:#x}, {:#x}) original={} rust={}", this, read_u32(this), day_year, r, n));
    r
}

/// Rule: a club's entry in a competition's statistics is found by binary search on the club table sorted by club id
/// (re-sorted first when another statistic left it in another order).
///
/// `Comp_Stats::find_club_entry(club)`: while a projection lends its temporary entry (+0x566), that entry, whatever the
/// club. No club: the comp_stats.CPP 3297 box and 0. No club table: 0. Unless the table is already in id order (sort
/// byte 0xff), it is marked 0xff and sorted (`G_COMP_STATS_SORT_CONTEXT` = the stats, qsort of `CLUB_COUNT` entries
/// of 0x76 bytes with `comp_stats_compare_team_entries`). Then `bsearch` with a key entry holding the club's id (the
/// same order, the context set again). Returns the entry or 0. The original builds the key as a whole 0x76-byte entry
/// on the stack (its date and run constructors only touch the stack); the order reads only the id.
/// Original: `Comp_Stats::find_club_entry` (0x4a4850, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_find_club_entry(stats: usize, club: usize) -> usize {
    let lent = read_usize(stats + COMP_STATS_PROJECTED_ENTRY);
    if lent != 0 {
        covered!("tel41_1 projected entry lent");
        return lent;
    }
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0xce1);
        return 0;
    }
    if read_usize(stats + comp_stats_obj::CLUBS) == 0 {
        return 0;
    }
    if read_u8(stats + comp_stats_obj::CLUB_SORT) != 0xff {
        write_u8(stats + comp_stats_obj::CLUB_SORT, 0xff);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
            comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
    }
    let mut key = [0u8; comp_stats_obj::CLUB_ENTRY_SIZE];
    write_u32(key.as_mut_ptr() as usize, read_u32(club)); // tel41_1: the key is the club id
    write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
    exe::crt_bsearch(key.as_ptr() as usize, read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
        comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES)
}

/// Rule: a player's entry in a competition's statistics is found by binary search on the player table sorted by
/// player id (re-sorted first when another statistic left it in another order).
///
/// `Comp_Stats::find_player_entry(player)`: no player: the comp_stats.CPP 3342 box and 0. No player table: 0. Unless
/// the table is already in id order (sort byte 0xff), it is marked 0xff and sorted (context = the stats, qsort of
/// `PLAYER_COUNT` entries of 0x2d bytes with `comp_stats_compare_player_entries`). Then `bsearch` with the player's id
/// as the key (the context set again). Returns the entry or 0.
/// Original: `Comp_Stats::find_player_entry` (0x4a4a30, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_find_player_entry(stats: usize, player: usize) -> usize {
    if player == 0 {
        show_error_box(COMP_STATS_CPP, 0xd0e);
        return 0;
    }
    if read_usize(stats + comp_stats_obj::PLAYERS) == 0 {
        return 0;
    }
    if read_u8(stats + comp_stats_obj::PLAYER_SORT) != 0xff {
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xff);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
            comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES);
    }
    let key = read_u32(player); // tel41_1: the key is the player id
    write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
    exe::crt_bsearch(&key as *const u32 as usize, read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
        comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES)
}

/// Rule: a club's statistic in a competition is the value of its entry; a club with no entry has 0.
///
/// `Comp_Stats::get_club_stat(club, stat)`: no club: the comp_stats.CPP 2274 box and 0.0. The entry
/// (`comp_stats_find_club_entry`); none: 0.0; otherwise `comp_stats_get_team_entry_value(entry, stat)`. The value
/// comes back in st0 (0.0 is the float constant `rdata::F32_0`).
/// Original: `Comp_Stats::get_club_stat` (0x4a2e10, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_club_stat(stats: usize, club: usize, stat: u32) -> f64 {
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0x8e2);
        return read_f32(rdata::F32_0) as f64;
    }
    let entry = comp_stats_find_club_entry(stats, club);
    if entry == 0 {
        covered!("tel41_1 club stat with no entry");
        return read_f32(rdata::F32_0) as f64;
    }
    exe::comp_stats_get_team_entry_value(stats, entry, stat) // tel41_1: the club statistic
}

/// Comparer for `comp_stats_find_club_entry`: side by side over the club table (it may be sorted), the sort byte, the
/// context and the entry returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_find_club_entry(stats: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_STATS_FIND_CLUB_ENTRY);
    if club == 0 {
        return o(stats, club);
    }
    let regions = comp_stats_find_regions(stats, false);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club), || comp_stats_find_club_entry(stats, club));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::find_club_entry({:#x}, {:#x}): entry {:#x} x {:#x}, differences in {:?}", stats, club, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_find_player_entry`: side by side over the player table, the sort byte, the context and
/// the entry returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_find_player_entry(stats: usize, player: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_STATS_FIND_PLAYER_ENTRY);
    if player == 0 {
        return o(stats, player);
    }
    let regions = comp_stats_find_regions(stats, true);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, player), || comp_stats_find_player_entry(stats, player));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::find_player_entry({:#x}, {:#x}): entry {:#x} x {:#x}, differences in {:?}", stats, player, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_club_stat`: side by side over the club table, its sort byte, the context and the
/// value (bit for bit).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_club_stat(stats: usize, club: usize, stat: u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> f64>(IDX_COMP_STATS_GET_CLUB_STAT);
    if club == 0 {
        return o(stats, club, stat);
    }
    let regions = comp_stats_find_regions(stats, false);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club, stat), || comp_stats_get_club_stat(stats, club, stat));
    SCOREBOARD.record(equal && ro.to_bits() == rr.to_bits(), || format!("Comp_Stats::get_club_stat({:#x}, {:#x}, {}): {} x {}, differences in {:?}", stats, club, stat as u8, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: a player's statistic in a competition is the value of his entry; a player with no entry has 0.
///
/// `Comp_Stats::get_player_stat(player, stat)`: no player: the comp_stats.CPP 2485 box and 0.0. The entry
/// (`Comp_Stats::find_player_entry`); none: 0.0; otherwise `Comp_Stats::get_player_entry_value(entry, stat)`. The value
/// comes back in st0.
/// Original: `Comp_Stats::get_player_stat` (0x4a31a0, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_player_stat(stats: usize, player: usize, stat: u32) -> f64 {
    if player == 0 {
        show_error_box(COMP_STATS_CPP, 0x9b5);
        return read_f32(rdata::F32_0) as f64;
    }
    let entry = comp_stats_player_entry_of(stats, player);
    if entry == 0 {
        covered!("tel41_2 player stat with no entry");
        return read_f32(rdata::F32_0) as f64;
    }
    exe::comp_stats_get_player_entry_value(stats, entry, stat) // tel41_2: the player statistic
}

/// Rule: a competition's team table for one statistic lists the clubs best first, at most `max` (all when negative);
/// for every statistic but 3, 4 and 5 the clubs at the bottom with 0 are left out.
///
/// `Comp_Stats::get_sorted_team_table(stat, &count, max)`: no club table: count 0 and 0. `max` below 0 is the club
/// count. Unless already sorted by `stat`, the club table is sorted by it (sort byte = stat, context = the stats,
/// qsort with `comp_stats_compare_team_entries`). Count = the smaller of the club count and `max`; for statistics
/// other than 3, 4 and 5, while the count is positive and the last counted club's value
/// (`comp_stats_get_team_entry_value`) is 0.0, the count goes down. Count 0: 0. Otherwise a block of count × 0x76
/// bytes (`crt_malloc`; failing, the comp_stats.CPP 3436 box, count 0 and 0) takes the first entries, copied field by
/// field. Returns the block (the caller frees it).
/// Original: `Comp_Stats::get_sorted_team_table` (0x4a4b20, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_sorted_team_table(stats: usize, stat_arg: u32, count: *mut i32, max: i32) -> usize {
    if read_usize(stats + comp_stats_obj::CLUBS) == 0 {
        *count = 0;
        return 0;
    }
    let max = if max < 0 { read_i32(stats + comp_stats_obj::CLUB_COUNT) } else { max };
    let stat = stat_arg as u8;
    if read_u8(stats + comp_stats_obj::CLUB_SORT) != stat {
        write_u8(stats + comp_stats_obj::CLUB_SORT, stat);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
            comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
    }
    let n = read_i32(stats + comp_stats_obj::CLUB_COUNT);
    *count = if n < max { n } else { max };
    if stat != 4 && stat != 5 && stat != 3 {
        while *count > 0 {
            let last = read_usize(stats + comp_stats_obj::CLUBS) + (*count as usize - 1) * comp_stats_obj::CLUB_ENTRY_SIZE;
            // tel41_2: the trailing clubs with 0 leave the table
            if exe::comp_stats_get_team_entry_value(stats, last, stat_arg) != read_f32(rdata::F32_0) as f64 {
                break;
            }
            *count -= 1;
        }
    }
    if *count == 0 {
        return 0;
    }
    let block = exe::crt_malloc((*count as u32).wrapping_mul(comp_stats_obj::CLUB_ENTRY_SIZE as u32));
    if block == 0 {
        show_error_box(COMP_STATS_CPP, 0xd6c);
        *count = 0;
        return 0;
    }
    let mut i = 0;
    while i < *count {
        let k = i as usize * comp_stats_obj::CLUB_ENTRY_SIZE;
        comp_stats_copy_club_entry(block + k, read_usize(stats + comp_stats_obj::CLUBS) + k);
        i += 1;
    }
    block
}

/// Rule: a competition's player table for one statistic lists the players best first, at most `max` (all when
/// negative), only those of the position groups in `groups` (0xffff = all) and never one with 0.
///
/// `Comp_Stats::get_sorted_player_table(stat, &count, max, groups)`: no player table: count 0 and 0. `max` below 0 is
/// the player count. Unless already sorted by `stat`: with statistic 0x11 the club table is first put in id order
/// (sort byte 0xff, qsort with `comp_stats_compare_team_entries`) when it is not; then the player table is sorted by
/// `stat` (sort byte = stat, context = the stats, qsort with `comp_stats_compare_player_entries`). Count = the player
/// count; while it is positive and the last player's value (`Comp_Stats::get_player_entry_value`) is 0.0, it goes
/// down. Count 0: 0. Otherwise a block of count × 0x2d bytes (`crt_malloc`; failing, the comp_stats.CPP 3526 box,
/// count 0 and 0). The first `count` players are walked while fewer than `max` were taken: with `groups` other than
/// 0xffff a player whose position group (`comp_stats_get_player_position_group` of his record in
/// `G_PLAYER_DATA_TABLE`, by the id at +0) has no bit in `groups` is skipped, and so is one whose value is 0.0; the
/// others are copied whole (0x2d bytes). Count = how many were taken. Returns the block (the caller frees it).
/// Original: `Comp_Stats::get_sorted_player_table` (0x4a4dc0, comp_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_sorted_player_table(stats: usize, stat_arg: u32, count: *mut i32, max: i32, groups: u32) -> usize {
    if read_usize(stats + comp_stats_obj::PLAYERS) == 0 {
        *count = 0;
        return 0;
    }
    let max = if max < 0 { read_i32(stats + comp_stats_obj::PLAYER_COUNT) } else { max };
    let stat = stat_arg as u8;
    if read_u8(stats + comp_stats_obj::PLAYER_SORT) != stat {
        if stat == 0x11 && read_u8(stats + comp_stats_obj::CLUB_SORT) != 0xff {
            write_u8(stats + comp_stats_obj::CLUB_SORT, 0xff);
            write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
            exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
        }
        write_u8(stats + comp_stats_obj::PLAYER_SORT, stat);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
            comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES);
    }
    let zero = read_f32(rdata::F32_0) as f64;
    *count = read_i32(stats + comp_stats_obj::PLAYER_COUNT);
    while *count > 0 {
        let last = read_usize(stats + comp_stats_obj::PLAYERS) + (*count as usize - 1) * comp_stats_obj::PLAYER_ENTRY_SIZE;
        if exe::comp_stats_get_player_entry_value(stats, last, stat_arg) != zero {
            break;
        }
        *count -= 1;
    }
    if *count == 0 {
        return 0;
    }
    let block = exe::crt_malloc((*count as u32).wrapping_mul(comp_stats_obj::PLAYER_ENTRY_SIZE as u32));
    if block == 0 {
        show_error_box(COMP_STATS_CPP, 0xdc6);
        *count = 0;
        return 0;
    }
    let mut taken = 0i32;
    let mut i = 0i32;
    while i < *count {
        if taken >= max {
            break;
        }
        let entry = read_usize(stats + comp_stats_obj::PLAYERS) + i as usize * comp_stats_obj::PLAYER_ENTRY_SIZE;
        let wanted = groups as u16 == 0xffff || {
            let sheet = read_usize(G_PLAYER_DATA_TABLE) + (read_i32(entry) as usize).wrapping_mul(COMP_STATS_PLAYER_SHEET_SIZE);
            // tel41_2: the position group filter
            comp_stats_get_player_position_group(sheet) & groups as u16 != 0
        };
        if wanted && exe::comp_stats_get_player_entry_value(stats, entry, stat_arg) != zero {
            core::ptr::copy_nonoverlapping(entry as *const u8, (block + taken as usize * comp_stats_obj::PLAYER_ENTRY_SIZE) as *mut u8,
                comp_stats_obj::PLAYER_ENTRY_SIZE);
            taken += 1;
        }
        i += 1;
    }
    *count = taken;
    block
}

/// Comparer for `comp_stats_get_player_stat`: side by side over the player table (it may be sorted by id), its sort
/// byte, the context and the value (bit for bit).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_player_stat(stats: usize, player: usize, stat: u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> f64>(IDX_COMP_STATS_GET_PLAYER_STAT);
    if player == 0 {
        return o(stats, player, stat);
    }
    let regions = [
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (stats + comp_stats_obj::PLAYER_SORT, 1),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, player, stat), || comp_stats_get_player_stat(stats, player, stat));
    SCOREBOARD.record(equal && ro.to_bits() == rr.to_bits(), || format!("Comp_Stats::get_player_stat({:#x}, {:#x}, {}): {} x {}, differences in {:?}", stats, player, stat as u8, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_sorted_team_table`: side by side over the tables, sort bytes and context (each side
/// allocates its own block); the two blocks and counts must be equal; the copy's block is freed.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_sorted_team_table(stats: usize, stat: u32, count: *mut i32, max: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i32, i32) -> usize>(IDX_COMP_STATS_GET_SORTED_TEAM_TABLE);
    let regions = comp_stats_sorted_table_regions(stats);
    let mut mine = 0i32;
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, stat, count, max),
        || comp_stats_get_sorted_team_table(stats, stat, &mut mine, max));
    let blocks = comp_stats_sorted_blocks_equal(ro, rr, *count, mine, comp_stats_obj::CLUB_ENTRY_SIZE);
    SCOREBOARD.record(equal && blocks, || format!("Comp_Stats::get_sorted_team_table({:#x}, stat {}, max {}): count {} x {}, blocks equal {}, differences in {:?}", stats, stat as u8, max, *count, mine, blocks, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_sorted_player_table`: as the team table's, over the player block.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_sorted_player_table(stats: usize, stat: u32, count: *mut i32, max: i32, groups: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i32, i32, u32) -> usize>(IDX_COMP_STATS_GET_SORTED_PLAYER_TABLE);
    let regions = comp_stats_sorted_table_regions(stats);
    let mut mine = 0i32;
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, stat, count, max, groups),
        || comp_stats_get_sorted_player_table(stats, stat, &mut mine, max, groups));
    let blocks = comp_stats_sorted_blocks_equal(ro, rr, *count, mine, comp_stats_obj::PLAYER_ENTRY_SIZE);
    SCOREBOARD.record(equal && blocks, || format!("Comp_Stats::get_sorted_player_table({:#x}, stat {}, max {}, groups {:#x}): count {} x {}, blocks equal {}, differences in {:?}", stats, stat as u8, max, groups as u16, *count, mine, blocks, first_differences(&mo, &mr)));
    ro
}

/// Rule: at a season's end the all-time records of a competition take the season's most points and best average
/// rating when they beat them, and the most titles is recounted; from the third season end on, a broken record of a
/// league division (most points, rating with more than 3 games, most titles from 5) raises its news flag.
///
/// `Comp_Stats::update_season_end_records()`, for a competition that is not a friendly one
/// (`Friendly::is_friendly_competition` of the competition's record, asked again before each part):
/// 1. `Comp_Stats::update_most_titles_record`; when the record's club changed and the record (signed byte +0x26) is
///    at least 5: flag 1 and the "any flag" dword set.
/// 2. Unless the competition's byte +0x42 is 4: with no season most points yet (-1) and a club table, the club table
///    sorted by points (statistic 2, unless already) and the leader's points (`_ftol` of statistic 2), its id and 0
///    written as the season record. A season record above -1 that beats the all-time one (signed; or the all-time is
///    -1) is copied to it (points, club, the word, then the competition's year in the word); with at least 3 season
///    ends (signed word +0x500) and a league division (`comp_util_is_league_division` of the competition id): flag 8.
/// 3. With no season rating (-1.0, or unordered) `Comp_Stats::update_season_rating_record`. A season rating above
///    -1.0 that beats the all-time one (or the all-time is -1.0/unordered) is copied to it: the rating, the holder
///    (`record_holder_assign`), the games byte, the club, the word and then the competition's year; with at least 3
///    season ends, a league division and more than 3 games (signed byte): flag 0x12.
/// The season end count goes up by 1 (always). Returns 1.
/// Original: `Comp_Stats::update_season_end_records` (0x4a19f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_update_season_end_records(stats: usize) -> u32 {
    use comp_stats_season_end as f;
    let comp = League(read_usize(stats + comp_stats_obj::COMP));
    let friendly = || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION)) != 0;
    let league_division = || exe::comp_util_is_league_division(read_i32(read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION))) != 0;
    if !friendly() && comp_stats_update_most_titles_record(stats) != 0 && read_u8(stats + comp_stats_obj::MOST_TITLES) as i8 >= 5 {
        covered!("tel41_3 most titles record news");
        write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_MOST_TITLES), 1);
        write_u32(stats + f::ANY_RECORD_FLAG, 1);
    }
    if !friendly() && comp.club_sides() != 4 {
        if read_i32(stats + comp_stats_obj::SEASON_MOST_POINTS) == -1 && read_usize(stats + comp_stats_obj::CLUBS) != 0 {
            if read_u8(stats + comp_stats_obj::CLUB_SORT) != 2 {
                write_u8(stats + comp_stats_obj::CLUB_SORT, 2);
                write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
                exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                    comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
            }
            let leader = read_usize(stats + comp_stats_obj::CLUBS);
            write_i32(stats + comp_stats_obj::SEASON_MOST_POINTS, ftol(team_entry_value(stats, leader, 2)));
            write_u32(stats + comp_stats_obj::SEASON_MOST_POINTS_CLUB, read_u32(leader));
            write_u16(stats + comp_stats_obj::SEASON_MOST_POINTS_EXTRA, 0);
        }
        let season = read_i32(stats + comp_stats_obj::SEASON_MOST_POINTS);
        let all_team = read_i32(stats + f::ALL_TIME_MOST_POINTS);
        // tel41_3: the season's most points beats the all-time record
        if season > -1 && (all_team == -1 || season > all_team) {
            write_i32(stats + f::ALL_TIME_MOST_POINTS, season);
            write_u32(stats + f::ALL_TIME_MOST_POINTS_CLUB, read_u32(stats + comp_stats_obj::SEASON_MOST_POINTS_CLUB));
            write_u16(stats + f::ALL_TIME_MOST_POINTS_YEAR, read_u16(stats + comp_stats_obj::SEASON_MOST_POINTS_EXTRA));
            write_u16(stats + f::ALL_TIME_MOST_POINTS_YEAR, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
            if read_i16(stats + f::SEASONS_ENDED) >= 3 && league_division() {
                covered!("tel41_3 most points record news");
                write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_MOST_POINTS), 1);
                write_u32(stats + f::ANY_RECORD_FLAG, 1);
            }
        }
    }
    if !friendly() {
        let minus_one = read_f64(rdata::F64_NEG_1);
        if comp_stats_c3(read_f32(stats + comp_stats_obj::SEASON_RATING) as f64, minus_one) {
            comp_stats_update_season_rating_record(stats);
        }
        let season = read_f32(stats + comp_stats_obj::SEASON_RATING) as f64;
        if season > minus_one {
            let all_team = read_f32(stats + f::ALL_TIME_RATING) as f64;
            if comp_stats_c3(all_team, minus_one) || season > all_team {
                write_u32(stats + f::ALL_TIME_RATING, read_u32(stats + comp_stats_obj::SEASON_RATING));
                exe::record_holder_assign(stats + f::ALL_TIME_RATING_HOLDER, stats + comp_stats_obj::SEASON_RATING_HOLDER);
                write_u8(stats + f::ALL_TIME_RATING_GAMES, read_u8(stats + comp_stats_obj::SEASON_RATING_GAMES));
                write_u32(stats + f::ALL_TIME_RATING_CLUB, read_u32(stats + comp_stats_obj::SEASON_RATING_CLUB));
                write_u16(stats + f::ALL_TIME_RATING_YEAR, read_u16(stats + comp_stats_obj::SEASON_RATING_EXTRA));
                write_u16(stats + f::ALL_TIME_RATING_YEAR, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
                if read_i16(stats + f::SEASONS_ENDED) >= 3 && league_division() && read_u8(stats + f::ALL_TIME_RATING_GAMES) as i8 > 3 {
                    covered!("tel41_3 rating record news");
                    write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_RATING), 1);
                    write_u32(stats + f::ANY_RECORD_FLAG, 1);
                }
            }
        }
    }
    write_i16(stats + f::SEASONS_ENDED, read_i16(stats + f::SEASONS_ENDED).wrapping_add(1));
    1
}

/// Rule: when a competition's season closes, its season records may be appended to the shared history file (the
/// records screen lists them later), then the season records are cleared, the runs kept for the next season and the
/// two tables freed.
///
/// `Comp_Stats::archive_season_and_reset(archive)`: with `archive` ≠ 0, a row of 0x26b bytes (on the stack, its
/// sub-records constructed first, `comp_season_records_construct`) takes the competition's year (word) and a copy of
/// the season records (+0x28f, field by field as `comp_season_records_assign`), and is appended to the shared file
/// list (`FILE_LLIST::add_node` of `COMP_STATS_ARCHIVE_LIST` with the two archive keys +0x4f8/+0x4fc, which it may
/// set). Then, always: `comp_season_records_clear` of the season records, `Comp_Stats::save_team_runs`, and the club
/// and player tables freed (`crt_free`; pointer, capacity and count 0, sort byte 0xfe). Returns 1.
/// Original: `Comp_Stats::archive_season_and_reset` (0x4a1c50, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_archive_season_and_reset(stats: usize, archive: u32) -> u32 {
    let mut row = [0u8; SEASON_RECORDS_ROW];
    let r = row.as_mut_ptr() as usize;
    comp_season_records_construct(r + 2);
    if archive != 0 {
        covered!("tel41_3 season archived");
        write_u16(r, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
        comp_season_records_assign(r + 2, stats + comp_stats_obj::SEASON_RECORDS);
        // tel41_3: the season row goes to the history file
        exe::file_llist_add_node(read_usize(COMP_STATS_ARCHIVE_LIST), stats + comp_stats_obj::ARCHIVE_KEY1,
            stats + comp_stats_obj::ARCHIVE_KEY2, row.as_ptr());
    }
    comp_season_records_clear(stats + comp_stats_obj::SEASON_RECORDS);
    comp_stats_save_team_runs(stats); // tel41_3: the runs kept for the next season
    if read_usize(stats + comp_stats_obj::CLUBS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::CLUBS));
        write_usize(stats + comp_stats_obj::CLUBS, 0);
        write_u32(stats + comp_stats_obj::CLUB_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::CLUB_COUNT, 0);
        write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    }
    if read_usize(stats + comp_stats_obj::PLAYERS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::PLAYERS));
        write_usize(stats + comp_stats_obj::PLAYERS, 0);
        write_u32(stats + comp_stats_obj::PLAYER_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::PLAYER_COUNT, 0);
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    }
    1
}

/// Rule: each competition record the season broke becomes one news item for the competition's region; then the
/// flags are cleared.
///
/// `Comp_Stats::send_record_news()`: with no flag set ("any flag" dword +0x562 = 0): 0. Otherwise, for kinds 1 to 22
/// in order, each set flag (+0x50a + 4 × (kind − 1)): a news item of type 0x3e9 (`NewsItem::NewsItem` on the stack),
/// filled by `comp_stats_write_record_to_news(competition id, all-time records +0x26, kind, item)` and sent to the
/// competition's region (`News::send_to_comp_region` of `G_NEWS` with the competition's record); the flag is cleared.
/// The "any flag" dword is cleared. Returns 1.
/// Original: `Comp_Stats::send_record_news` (0x4a2450, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_send_record_news(stats: usize) -> u32 {
    if read_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG) == 0 {
        return 0;
    }
    for kind in 1..=22usize {
        let flag = stats + comp_stats_record_flag(kind);
        if read_u32(flag) == 0 {
            continue;
        }
        covered!("tel41_3 record news sent");
        let mut item = [0u32; COMP_STATS_NEWS_ITEM_BYTES / 4];
        let n = item.as_mut_ptr() as usize;
        exe::new_newsitem(n, COMP_STATS_RECORD_NEWS_TYPE, 0);
        let record = read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION);
        // tel41_3: the record kind of the news
        comp_stats_write_record_to_news(read_u32(record), stats + comp_stats_obj::ALL_TIME_RECORDS, kind as u32, n);
        exe::news_send_to_comp_region(read_usize(G_NEWS), n, read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION));
        write_u32(flag, 0); // tel41_3: the flag cleared
    }
    write_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG, 0);
    1
}

/// Comparer for `comp_stats_update_season_end_records`: side by side over the whole object, both tables (the sorts),
/// the history table (the most titles sort) and the sort context; the tally block of the most titles is each side's
/// own and the friendly check only reads.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_update_season_end_records(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_COMP_STATS_UPDATE_SEASON_END_RECORDS);
    let regions = [
        (stats, COMP_STATS_OBJECT_SIZE),
        (read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT) as usize * COMP_HISTORY_ROW_SIZE),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats), || comp_stats_update_season_end_records(stats));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::update_season_end_records({:#x}): differences in {:?}", stats, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_archive_season_and_reset`: it appends to the history file and frees, so only the original
/// runs; the plan, made before, is checked after it: the season records as `comp_season_records_clear` leaves them
/// (cleared on a copy beforehand), the saved runs one per club entry (as `Comp_Stats::save_team_runs` leaves the count), both tables freed and 1 returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_archive_season_and_reset(stats: usize, archive: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
    let mut cleared = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    core::ptr::copy_nonoverlapping((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.as_mut_ptr(), cleared.len());
    comp_season_records_clear(cleared.as_mut_ptr() as usize);
    let teams = read_u32(stats + comp_stats_obj::CLUB_COUNT);
    let clubs = if teams != 0 {
        teams
    } else if read_usize(stats + comp_stats_runs::SAVED) != 0 {
        0
    } else {
        read_u32(stats + comp_stats_runs::SAVED_COUNT)
    };
    let r = o(stats, archive);
    let records = core::slice::from_raw_parts((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.len());
    let ok = r == 1 && records == &cleared[..] && read_u32(stats + comp_stats_runs::SAVED_COUNT) == clubs
        && read_usize(stats + comp_stats_obj::CLUBS) == 0 && read_usize(stats + comp_stats_obj::PLAYERS) == 0;
    SCOREBOARD.record(ok, || format!("Comp_Stats::archive_season_and_reset({:#x}, {}): out of plan (return {}, saved runs {} for {} clubs)", stats, archive, r, read_u32(stats + comp_stats_runs::SAVED_COUNT), clubs));
    r
}

/// Comparer for `comp_stats_send_record_news`: it sends news, so only the original runs; the plan is checked after it:
/// with a flag set it returns 1 and clears the 22 flags and the "any flag" dword; without, 0 and nothing changes.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_send_record_news(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_COMP_STATS_SEND_RECORD_NEWS);
    let any = read_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG) != 0;
    let mut before = [0u8; 0x5c];
    core::ptr::copy_nonoverlapping((stats + comp_stats_season_end::RECORD_FLAGS) as *const u8, before.as_mut_ptr(), before.len());
    let r = o(stats);
    let after = core::slice::from_raw_parts((stats + comp_stats_season_end::RECORD_FLAGS) as *const u8, before.len());
    let ok = if any { r == 1 && after.iter().all(|&b| b == 0) } else { r == 0 && after == &before[..] };
    SCOREBOARD.record(ok, || format!("Comp_Stats::send_record_news({:#x}): out of plan (return {}, flags {:x?})", stats, r, after));
    r
}

/// Rule: a new competition statistics object starts with no tables and every record empty; the first object alive
/// opens the shared file of the archived seasons.
///
/// `Comp_Stats::Comp_Stats(comp)`: the sub-records of the all-time records (+0x26) and the season records block
/// (+0x28f) are constructed (the compiler's member constructors, called by address), the lent projection entry
/// (+0x566) is 0. The instance count (`COMP_STATS_INSTANCES`) goes up; when it becomes 1, a list object of 8 bytes
/// (`crt_operator_new`) is built as the file list "CM3_TEMP\comp records history.tmp" (`utils_build_file_path`) with
/// 0x26b-byte rows (`FILE_LLIST::FILE_LLIST`) into `COMP_STATS_ARCHIVE_LIST` (0 when the allocation failed; then the
/// comp_stats.CPP 89 box, `game_fatal_cleanup` and `exit(-1)`). Both sort bytes 0xfe, the block +0x56a..+0x5ce, the
/// tables (+0xc..+0x14, +0x19..+0x21) and the saved runs (+0, +4, +8) zeroed; both record sets cleared
/// (`comp_season_records_clear`); the archive keys +0x4f8 and +0x4fc -1; the competition at +0x502; the season end
/// count (word +0x500) 0; the block +0x506..+0x562 and the "any flag" dword +0x562 zeroed; then
/// `Comp_Stats::update_most_titles_record` counts the titles already in the history table. Returns the object.
/// Original: `Comp_Stats::Comp_Stats` (0x49ee70, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_construct(stats: usize, comp: usize) -> usize {
    for (offset, constructor) in COMP_STATS_ALL_TIME_CONSTRUCTORS {
        let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(constructor);
        f(stats + comp_stats_obj::ALL_TIME_RECORDS + offset);
    }
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(COMP_STATS_SEASON_RECORDS_CONSTRUCT);
    f(stats + comp_stats_obj::SEASON_RECORDS);
    write_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT, 0);
    let alive = read_i32(COMP_STATS_INSTANCES).wrapping_add(1);
    write_i32(COMP_STATS_INSTANCES, alive);
    if alive == 1 {
        covered!("tel41_4 history file opened");
        let block = exe::crt_operator_new(8);
        let list = if block != 0 {
            let path = exe::utils_build_file_path(COMP_STATS_TEMP_DIR, COMP_STATS_HISTORY_FILE);
            let build: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FILE_LLIST_CONSTRUCT);
            build(block, path, SEASON_RECORDS_ROW as u32)
        } else {
            0
        };
        write_usize(COMP_STATS_ARCHIVE_LIST, list);
        if list == 0 {
            comp_stats_fatal(0x59);
        }
    }
    write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    core::ptr::write_bytes((stats + COMP_STATS_LATE_BLOCK) as *mut u8, 0, COMP_STATS_LATE_BLOCK_SIZE);
    for o in [comp_stats_obj::CLUBS, comp_stats_obj::CLUB_CAPACITY, comp_stats_obj::CLUB_COUNT, comp_stats_obj::PLAYERS,
              comp_stats_obj::PLAYER_CAPACITY, comp_stats_obj::PLAYER_COUNT, comp_stats_runs::SAVED, comp_stats_runs::SAVED_CAPACITY,
              comp_stats_runs::SAVED_COUNT] {
        write_u32(stats + o, 0);
    }
    comp_season_records_clear(stats + comp_stats_obj::ALL_TIME_RECORDS);
    comp_season_records_clear(stats + comp_stats_obj::SEASON_RECORDS);
    write_i32(stats + comp_stats_obj::ARCHIVE_KEY1, -1); // tel41_4: no archived season yet
    write_i32(stats + comp_stats_obj::ARCHIVE_KEY2, -1);
    write_usize(stats + comp_stats_obj::COMP, comp);
    write_u16(stats + COMP_STATS_SEASONS_ENDED, 0);
    core::ptr::write_bytes((stats + COMP_STATS_FLAGS_BLOCK) as *mut u8, 0, COMP_STATS_FLAGS_BLOCK_SIZE);
    write_u32(stats + COMP_STATS_ANY_FLAG, 0);
    // tel41_4: the titles already won count from the start
    comp_stats_update_most_titles_record(stats);
    stats
}

/// Rule: destroying a competition statistics object frees its tables and saved runs; the last object alive closes the
/// shared file of the archived seasons.
///
/// `Comp_Stats::~Comp_Stats()`: the club and player tables freed (`crt_free`; pointer, capacity and count 0, sort byte
/// 0xfe); when this is the only object alive (instance count 1) and the file list exists, it is destroyed
/// (`FILE_LLIST::~FILE_LLIST`, `crt_operator_delete`) and `COMP_STATS_ARCHIVE_LIST` set to 0; the saved runs array
/// (+0) freed and its pointer cleared (capacity and count stay); the instance count goes down. The member destructors
/// that follow are empty (0x5a53c0).
/// Original: `Comp_Stats::~Comp_Stats` (0x49f450, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_destruct(stats: usize) {
    if read_usize(stats + comp_stats_obj::CLUBS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::CLUBS));
        write_usize(stats + comp_stats_obj::CLUBS, 0);
        write_u32(stats + comp_stats_obj::CLUB_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::CLUB_COUNT, 0);
        write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    }
    if read_usize(stats + comp_stats_obj::PLAYERS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::PLAYERS));
        write_usize(stats + comp_stats_obj::PLAYERS, 0);
        write_u32(stats + comp_stats_obj::PLAYER_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::PLAYER_COUNT, 0);
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    }
    let list = read_usize(COMP_STATS_ARCHIVE_LIST);
    if read_i32(COMP_STATS_INSTANCES) == 1 && list != 0 {
        covered!("tel41_4 history file closed");
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FILE_LLIST_DESTRUCT);
        destroy(list);
        exe::crt_operator_delete(list);
        write_usize(COMP_STATS_ARCHIVE_LIST, 0);
    }
    if read_usize(stats + comp_stats_runs::SAVED) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_runs::SAVED));
        write_usize(stats + comp_stats_runs::SAVED, 0);
    }
    write_i32(COMP_STATS_INSTANCES, read_i32(COMP_STATS_INSTANCES).wrapping_sub(1)); // tel41_4: one instance fewer
}

/// Rule: a club's projected final, best and worst positions come from the last stage of its competition that holds
/// it, which projects them into a scratch entry; when no stage holds it or the projection fails, the positions are
/// those of its statistics entry, if any.
///
/// `Comp_Stats::get_club_projected_positions(club, &best, &current, &worst)`: from the competition's current stage
/// index (signed byte +0x30) down to -1 (the competition itself; below -1 at the start: straight to the fallback),
/// the first stage (`STAGES[index]`, or the competition for -1) whose virtual 0x80 answers nonzero for the club. Then
/// a 0x76-byte scratch entry (`crt_operator_new`; its eight run dates constructed, `Date::Date`; 0 when the allocation
/// failed) is lent in +0x566 and the stage's virtual 0x84 (`update_projected_positions`) runs for the club: nonzero,
/// the entry's +0x74, +0x73 and +0x75 go to `best`, `current` and `worst`, the entry is freed (`crt_operator_delete`),
/// +0x566 = 0 and 1 is returned; zero, the entry is freed and +0x566 = 0. Fallback (no stage, or the projection
/// failed): the club's entry (`comp_stats_find_club_entry`), when there is one, gives the same three bytes; returns 0.
/// Original: `Comp_Stats::get_club_projected_positions` (0x4ae300, comp_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_club_projected_positions(stats: usize, club: usize, best: *mut u8, current: *mut u8, worst: *mut u8) -> u32 {
    let comp = League(read_usize(stats + comp_stats_obj::COMP));
    let mut index = (comp.current_stage() as u8) as i8;
    let mut stage = 0;
    while index >= -1 {
        let s = if index == -1 { read_usize(stats + comp_stats_obj::COMP) }
                else { read_usize(read_usize(read_usize(stats + comp_stats_obj::COMP) + league::STAGES) + index as isize as usize * 4) };
        let holds: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(s) + COMP_VT_HOLDS_CLUB));
        if holds(s, club) != 0 {
            stage = s;
            break;
        }
        index = index.wrapping_sub(1);
    }
    if stage != 0 {
        let entry = exe::crt_operator_new(comp_stats_obj::CLUB_ENTRY_SIZE as u32);
        if entry != 0 {
            let date: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(COMP_STATS_DATE_CONSTRUCT);
            for run in [comp_stats_club::WINNING_RUN, comp_stats_club::LOSING_RUN, comp_stats_club::UNBEATEN_RUN, comp_stats_club::WINLESS_RUN] {
                date(entry + run + 4);
                date(entry + run + 0xc);
            }
        }
        write_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT, entry);
        let project: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(stage) + COMP_VT_UPDATE_PROJECTED_POSITIONS));
        let done = project(stage, club) != 0;
        if done {
            covered!("tel41_4 positions projected");
            // the original reads the lent slot again for each byte (a failed allocation leaves 0 there)
            *best = read_u8(read_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT) + comp_stats_club::BEST_POSITION); // tel41_4: projected best
            *current = read_u8(read_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT) + comp_stats_club::CURRENT_POSITION);
            *worst = read_u8(read_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT) + comp_stats_club::WORST_POSITION);
        }
        let lent = read_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT);
        if lent != 0 {
            exe::crt_operator_delete(lent);
        }
        write_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT, 0);
        if done {
            return 1;
        }
    }
    let entry = exe::comp_stats_find_club_entry(stats, club);
    if entry != 0 {
        // tel41_4: the fallback positions of the club's own entry
        *best = read_u8(entry + comp_stats_club::BEST_POSITION); // tel41_4: fallback best
        *current = read_u8(entry + comp_stats_club::CURRENT_POSITION);
        *worst = read_u8(entry + comp_stats_club::WORST_POSITION);
    }
    0
}

/// Comparer for `comp_stats_construct`: it counts instances and may open the shared file, so only the original runs;
/// the plan is checked after it: the tables and saved runs zeroed, both sort bytes 0xfe, both record sets as
/// `comp_season_records_clear` leaves them except the most titles (byte and club, which the titles count rewrites),
/// the keys -1, the competition, the counters and blocks zeroed, one more instance and the file list open.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_construct(stats: usize, comp: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_STATS_CONSTRUCT);
    let alive = read_i32(COMP_STATS_INSTANCES);
    let r = o(stats, comp);
    let mut cleared = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    core::ptr::copy_nonoverlapping((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.as_mut_ptr(), cleared.len());
    comp_season_records_clear(cleared.as_mut_ptr() as usize);
    let season_ok = core::slice::from_raw_parts((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.len()) == &cleared[..];
    let mut all_team = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    core::ptr::copy_nonoverlapping((stats + comp_stats_obj::ALL_TIME_RECORDS) as *const u8, all_team.as_mut_ptr(), all_team.len());
    let mut all_team_cleared = all_team;
    comp_season_records_clear(all_team_cleared.as_mut_ptr() as usize);
    all_team_cleared[..5].copy_from_slice(&all_team[..5]); // the most titles (byte, club) of the count
    let zero = |o: usize, n: usize| core::slice::from_raw_parts((stats + o) as *const u8, n).iter().all(|&b| b == 0);
    let ok = r == stats && season_ok && all_team == all_team_cleared
        && zero(0, 0xc) && zero(comp_stats_obj::CLUBS, 0xc) && zero(comp_stats_obj::PLAYERS, 0xc)
        && read_u8(stats + comp_stats_obj::CLUB_SORT) == 0xfe && read_u8(stats + comp_stats_obj::PLAYER_SORT) == 0xfe
        && read_i32(stats + comp_stats_obj::ARCHIVE_KEY1) == -1 && read_i32(stats + comp_stats_obj::ARCHIVE_KEY2) == -1
        && read_usize(stats + comp_stats_obj::COMP) == comp && read_u16(stats + COMP_STATS_SEASONS_ENDED) == 0
        && zero(COMP_STATS_FLAGS_BLOCK, COMP_STATS_FLAGS_BLOCK_SIZE + 8) && zero(COMP_STATS_LATE_BLOCK, COMP_STATS_LATE_BLOCK_SIZE)
        && read_i32(COMP_STATS_INSTANCES) == alive.wrapping_add(1) && read_usize(COMP_STATS_ARCHIVE_LIST) != 0;
    SCOREBOARD.record(ok, || format!("Comp_Stats::Comp_Stats({:#x}, comp {:#x}): out of plan (instances {} -> {})", stats, comp, alive, read_i32(COMP_STATS_INSTANCES)));
    r
}

/// Comparer for `comp_stats_destruct`: it frees, so only the original runs; the plan is checked after it: tables and
/// saved runs pointer zeroed, one instance fewer and, when it was the last one, the file list gone.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_destruct(stats: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_COMP_STATS_DESTRUCT);
    let alive = read_i32(COMP_STATS_INSTANCES);
    o(stats);
    let ok = read_usize(stats + comp_stats_obj::CLUBS) == 0 && read_usize(stats + comp_stats_obj::PLAYERS) == 0
        && read_usize(stats + comp_stats_runs::SAVED) == 0 && read_i32(COMP_STATS_INSTANCES) == alive.wrapping_sub(1)
        && (alive != 1 || read_usize(COMP_STATS_ARCHIVE_LIST) == 0);
    SCOREBOARD.record(ok, || format!("Comp_Stats::~Comp_Stats({:#x}): out of plan (instances {} -> {})", stats, alive, read_i32(COMP_STATS_INSTANCES)));
}

/// Comparer for `comp_stats_get_club_projected_positions`: side by side over the three outputs, the lent entry slot
/// and the return (each side allocates and frees its own scratch entry; the projection only writes into it, cpr_11).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_club_projected_positions(stats: usize, club: usize, best: *mut u8, current: *mut u8, worst: *mut u8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8, *mut u8) -> u32>(IDX_COMP_STATS_GET_CLUB_PROJECTED_POSITIONS);
    let regions = [(best as usize, 1), (current as usize, 1), (worst as usize, 1), (stats + COMP_STATS_PROJECTED_ENTRY_SLOT, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club, best, current, worst),
        || comp_stats_get_club_projected_positions(stats, club, best, current, worst));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::get_club_projected_positions({:#x}, {:#x}): return {} x {}, differences in {:?}", stats, club, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: opening the competition screen on its fixtures: the Results tab on a given date (`mode` ≠ 0) or the Fixtures
/// tab (`mode` 0) of the competition.
///
/// `comp_screens_open_comp_fixtures(comp, day, year, mode)`: the competition object is the entry of the competition
/// table [`G_COMP_OBJECT_TABLE`] at the record's id (`comp` +0). The screen opens with `open_screen` (builder
/// `comp_screens_draw_comp_screen`, input `comp_screens_handle_comp_screen_click`, flag 0, leave
/// `comp_screens_close_comp_screen`, no exit); if it does not open nothing else happens. Then the 54 parameters: 0 the
/// object; 1 the tab (2 Results with `mode`, else 3 Fixtures); 2 and 3 the stage −2 (current); 8 and 9 the Results
/// date (the day's and the year's low words with `mode`, else 0); 0x11 and 0x12 the Fixtures date (the same words
/// without `mode`, else 0); 0x1e 0x11; 0x23 3; 0x2a and 0x32 −1; 0x2d 1; every other one 0 (4..7, 0xa..0x10,
/// 0x13..0x1d, 0x1f..0x22, 0x24..0x29, 0x2b, 0x2c, 0x2e..0x31, 0x33..0x35), written in the original's order.
/// Original: `comp_screens_open_comp_fixtures` (0x48f8e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_open_comp_fixtures(comp: usize, day: u32, year: u32, mode: u32) {
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x48fcd0usize, 0x4965d0usize, 0, 0x4985e0usize, 0) == 0 {
        return;
    }
    covered!("tel22_1 open comp fixtures");
    let plan = comp_screen_planned_params(object, day, year, mode);
    for k in COMP_SCREEN_OPEN_ORDER {
        set_screen_param(k, plan[k as usize]);
    }
}

/// Comparer for `comp_screens_open_comp_fixtures`: only the original runs (it opens a screen), and the parameters are
/// checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_comp_screens_open_comp_fixtures(comp: usize, day: u32, year: u32, mode: u32) {
    let object = if comp != 0 { read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4) } else { 0 };
    let plan = comp_screen_planned_params(object, day, year, mode);
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32)>(IDX_COMP_SCREENS_OPEN_COMP_FIXTURES)(comp, day, year, mode);
    if object != 0 && get_screen_param(0) == object {
        let mut got = [0usize; 0x36];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("competition screen (open fixtures {:#x}): params {:x?} x plan {:x?}", comp, got, plan));
    }
}

/// Rule: building the competition screen: the title, the tab bar with the tabs the competition has, a Print button
/// with its popup, and the open tab.
///
/// [0x99c00c] = 1, [0xadae0c] = 0, parameter 0x34 = 0. The object is parameter 0; without it the comp_screens.cpp box
/// of line 0x1c6 and nothing else. The tab is parameter 1's low byte. The competition's picture
/// (`background_set_comp_picture`), its colours (`database_get_comp_colours`), the title (the competition's name) and
/// the menu bar (4). The stages from −1 to the current one (`league::CURRENT_STAGE`) are counted: tables (type 2) and
/// rounds (each cup stage's round byte +0xab, at least 1). The tabs, in order: the stages tab ("Stages" with tables
/// and rounds, "Tables"/"Table" with tables only, "Rounds"/"Draw" with rounds only) = 1, "Results" = 2, "Fixtures" =
/// 3, "Schedule" = 4, "Qualifiers" = 5 (when the entrant book holds the qualifiers), "Seedings" = 6 (when it does not
/// and there is a book), "Average Points" = 9 (+0x18 set), "Team Stats" = 7 and "Player Stats" = 8 (with
/// `league::COMP_STATS`), "Referee Stats" = 10 (`comp_util_is_league_division`), "Awards" (kind 3, the award of
/// `AWARD_MANAGER::find_award_by_club_comp`, its id +5) and "History" (kind 4; not for friendlies nor the competition
/// [0x9cf6f8]); a tab not offered while it is the open one sends the screen back to tab 1 (parameter 1 = 1). With
/// tables or statistics: the Print button (0x2c6, 0xf, 0x311, 0x23; black text, pale grey on a black competition
/// colour) and its popup area (flags 0x130) with the items "Table(s)" (data 1, only with tables), "Goals" 2,
/// "Assists" 4, "Average Ratings" 8, "Man of Match" 0x10 and "All" 0x1f (only with statistics; disabled 0x21
/// otherwise), id 0x19. The tab bar (`gui_add_tab_bar`, from 0x50 to 0x221, the narrow style below 8 tabs); then the
/// tab: 1 the current stage (parameter 2; −2 = `Comp::get_current_stage`'s index, written back) as a league table
/// (`comp_screens_draw_league_table_tab`) or a cup draw (`comp_screens_draw_cup_rounds_tab`); 2
/// `comp_screen_results_tab`; 3 Fixtures, 4 Schedule, 6 Seedings, 7 Team Stats, 8 Player Stats and 9 Average Points
/// (their tab routines, with the object); 5 `comp_screen_qualifiers_tab`; 10 the referee stats (0x79a530, with the
/// record). Then the related competitions menu (0x49d070, the colours, 1, 0x18) and the Back/Next buttons.
/// Original: `comp_screens_draw_comp_screen` (0x48fcd0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_comp_screen() {
    let mut items = [[0u8; COMP_SCREEN_MENU_ITEM]; 0x1a];
    for it in items.iter_mut() {
        it[0] = 0xff;
        it[1] = 0xff;
    }
    write_u32(COMP_SCREEN_BUILT_FLAG, 1);
    write_u32(COMP_SCREEN_DATE_BUTTONS_FLAG, 0);
    set_screen_param(0x34, 0);
    let comp = League(get_screen_param(0));
    if comp.is_null() {
        comp_screens_error_box(0x1c6);
        return;
    }
    covered!("tel22_1 build comp screen");
    let mut table = get_screen_param(1) as u8 as i8;
    let record = comp.competition().addr();
    exe::background_set_comp_picture(record, 0, 0);
    let (mut colour_44, mut colour_40) = (0u32, 0u32);
    exe::database_get_comp_colours(record, &mut colour_44, &mut colour_40);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, colour_40, colour_44, comp.competition().addr() + 4);
    exe::menubar_build(4, 0);
    let (mut tables, mut rounds) = (0i32, 0i32);
    let mut s: i32 = -1;
    while s <= (comp.current_stage() as i32) {
        let stage = exe::comp_get_stage(comp, s);
        if read_u8(stage + league::COMP_TYPE) == 2 {
            tables += 1;
        } else {
            let n = read_u8(stage + COMP_SCREEN_STAGE_ROUNDS) as i8;
            rounds += if n > 1 { n as i32 } else { 1 };
        }
        s += 1;
    }
    let heading = if tables != 0 {
        if rounds != 0 { 0x99c154 } else if tables > 1 { 0x99c14c } else { 0x99c144 } // "Stages" / "Tables" / "Table"
    } else if rounds > 1 {
        0x99c13c // "Rounds"
    } else {
        0x99c134 // "Draw"
    };
    comp_lang_format()(JOB_TEXT, heading);
    fn set_kind(items: &mut [[u8; COMP_SCREEN_MENU_ITEM]; 0x1a], k: usize, kind: u32, value: u32) {
        items[k][0xcc..0xd0].copy_from_slice(&kind.to_le_bytes());
        items[k][0xd0..0xd4].copy_from_slice(&value.to_le_bytes());
    }
    fn text(items: &mut [[u8; COMP_SCREEN_MENU_ITEM]; 0x1a], k: usize) -> usize {
        items[k][4..].as_mut_ptr() as usize
    }
    tel26_copy_text(text(&mut items, 0) as *mut u8, JOB_TEXT as *const u8);
    set_kind(&mut items, 0, 2, 1);
    comp_lang_format()(text(&mut items, 1), 0x990a94); // "Results"
    set_kind(&mut items, 1, 2, 2);
    comp_lang_format()(text(&mut items, 2), 0x98e868); // "Fixtures"
    set_kind(&mut items, 2, 2, 3);
    comp_lang_format()(text(&mut items, 3), 0x99c104, 0xad9c64usize); // "Schedule"
    set_kind(&mut items, 3, 2, 4);
    let mut n = 4usize;
    let back_to_table_1 = |table: &mut i8| {
        *table = 1;
        set_screen_param(1, 1);
    };
    if comp.entrants_are_qualifiers() == 1 && comp.entrant_book() != 0 {
        comp_lang_format()(text(&mut items, 4), 0x99c0f8); // "Qualifiers"
        set_kind(&mut items, 4, 2, 5);
        n = 5;
    } else if table == 5 {
        back_to_table_1(&mut table);
    }
    if comp.entrants_are_qualifiers() == 0 && comp.entrant_book() != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0ec); // "Seedings"
        set_kind(&mut items, n, 2, 6);
        n += 1;
    } else if table == 6 {
        back_to_table_1(&mut table);
    }
    if read_usize(comp.addr() + COMP_SCREEN_AVERAGE_POINTS) != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0dc); // "Average Points"
        set_kind(&mut items, n, 2, 9);
        n += 1;
    } else if table == 9 {
        back_to_table_1(&mut table);
    }
    if comp.comp_stats() != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0d0); // "Team Stats"
        set_kind(&mut items, n, 2, 7);
        n += 1;
        comp_lang_format()(text(&mut items, n), 0x99c0c0); // "Player Stats"
        set_kind(&mut items, n, 2, 8);
        n += 1;
    } else if table == 7 || table == 8 {
        back_to_table_1(&mut table);
    }
    if exe::comp_util_is_league_division(read_i32(comp.competition().addr())) != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0b0); // "Referee Stats"
        set_kind(&mut items, n, 2, 10);
        n += 1;
    }
    let award = exe::award_manager_find_award_by_club_comp(read_usize(G_AWARD_MANAGER), comp.competition().addr());
    if award != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0a8); // "Awards"
        set_kind(&mut items, n, 3, read_u32(award + 5));
        n += 1;
    }
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp.competition().addr()) == 0
        && read_u32(comp.competition().addr()) != read_u32(COMP_SCREEN_NO_HISTORY_ID)
    {
        comp_lang_format()(text(&mut items, n), 0x99c074, 0xad9c64usize); // "History"
        set_kind(&mut items, n, 4, 0);
        n += 1;
    }
    if tables != 0 || comp.comp_stats() != 0 {
        covered!("tel22_1 print popup");
        comp_lang_format()(JOB_TEXT, 0x98e5f4); // "Print"
        let text_colour = if colour_40 as u16 == read_u16(G_GUI_COLOUR_BLACK) { job_colour(JOB_COLOUR_3184) } else { job_colour(G_GUI_COLOUR_BLACK) };
        let button = tel26_add_button([0x2c6, 0xf, 0x311, 0x23, 0x30, colour_40, 0xc, 1, text_colour, JOB_TEXT as u32, 0, 0]);
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
        comp_lang_format()(JOB_TEXT, if tables > 1 { 0x99c14c } else { 0x99c144 }); // "Tables" / "Table"
        let popup_item = |enabled: bool, data: u32| {
            let (style, id, value) = if enabled { (1, 0x19, data) } else { (0x21, 0, 0) }; // tel22_1 a Print popup item
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), style, 1,
                job_colour(G_GUI_COLOUR_BLACK), JOB_TEXT as u32, 0, id, value, popup]);
        };
        popup_item(tables != 0, 1);
        for (fmt, data) in [(0x98e9ccusize, 2u32), (0x99c06c, 4), (0x99c05c, 8), (0x99c04c, 0x10)] {
            comp_lang_format()(JOB_TEXT, fmt); // "Goals", "Assists", "Average Ratings", "Man of Match"
            popup_item(comp.comp_stats() != 0, data);
        }
        comp_lang_format()(JOB_TEXT, 0x98e5c8, 0xad9c64usize); // "All"
        popup_item(comp.comp_stats() != 0, 0x1f);
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(items.as_mut_ptr() as usize, n as u32, table as i32, &mut top, &mut bottom, (n < 8) as u32);
    let table_routine = |address: usize, object: usize| {
        let f: unsafe extern "C" fn(usize, i32, i32) = core::mem::transmute(address);
        f(object, top, bottom);
    };
    match table {
        1 => {
            let mut s = get_screen_param(2) as i32;
            if s == -2 {
                s = read_u8(exe::comp_get_current_stage(comp) + league::STAGE_INDEX) as i8 as i32;
                set_screen_param(2, s as isize as usize);
            }
            let stage = exe::comp_get_stage(comp, s);
            if read_u8(stage + league::COMP_TYPE) == 2 {
                table_routine(0x491380, stage);
            } else {
                table_routine(0x492980, stage);
            }
        }
        2 => comp_screen_results_tab(comp.addr(), top, bottom),
        3 => table_routine(0x493ee0, comp.addr()),
        4 => table_routine(0x4947a0, comp.addr()),
        5 => comp_screen_qualifiers_tab(comp.addr(), top, bottom),
        6 => table_routine(0x494c60, comp.addr()),
        7 => table_routine(0x4956e0, comp.addr()),
        8 => table_routine(0x495bc0, comp.addr()),
        9 => table_routine(0x495050, comp.addr()),
        10 => table_routine(0x79a530, comp.competition().addr()),
        _ => {}
    }
    exe::comp_screens_add_related_comps_menu(comp, colour_40, colour_44, 1, 0x18);
    tel26_back_next();
}

/// Comparer for `comp_screens_draw_comp_screen`: side by side over the display list (`compare_match_screen_build`)
/// plus the two flags, when the open tab keeps no list to build.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_comp_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_COMP_SCREENS_DRAW_COMP_SCREEN);
    let comp = get_screen_param(0);
    // simplification: the tabs that build and keep a list in the params on their first build (1: a small league's
    // fixtures, param 5; 2: the day's matches, 0xe; 3: 0x17; 4: 0x1a; 7: 0x24; 8: 0x1f), the referee stats (10, not
    // in this file) and the error box (no object) run the original only; the tabs have their own comparers
    let ready = |table: u8| match table {
        1 => get_screen_param(5) != 0,
        2 => get_screen_param(0xe) != 0,
        3 => get_screen_param(0x17) != 0,
        4 => get_screen_param(0x1a) != 0,
        5 | 6 | 9 => true,
        7 => get_screen_param(0x24) != 0,
        8 => get_screen_param(0x1f) != 0,
        _ => false,
    };
    let table = get_screen_param(1) as u8;
    if comp == 0 || !ready(table) || ((5..=9).contains(&table) && !ready(1)) {
        return o();
    }
    compare_match_screen_build("competition screen", &[(COMP_SCREEN_BUILT_FLAG, 4), (COMP_SCREEN_DATE_BUTTONS_FLAG, 4)], || o(),
        || comp_screens_draw_comp_screen());
}

/// Rule: leaving the competition screen frees what its tabs kept: the LLISTs of parameters 5, 0xe and 0x17 and the
/// blocks of parameters 0x1a, 0x24 and 0x1f (with their counts 0x1b, 0x25 and 0x20 cleared).
///
/// Each list: when the parameter is set, its destructor and operator delete, the parameter cleared; each block: when
/// set, crt free and the parameter and the next one cleared (`comp_free_list_param`, `comp_free_block_param`).
/// Original: `comp_screens_close_comp_screen` (0x4985e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_close_comp_screen() {
    comp_free_list_param(5);
    comp_free_list_param(0xe);
    comp_free_list_param(0x17);
    comp_free_block_param(0x1a);
    comp_free_block_param(0x24);
    comp_free_block_param(0x1f);
}

/// Comparer for `comp_screens_close_comp_screen`: only the original runs (it frees memory), the parameters checked
/// against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_comp_screens_close_comp_screen() {
    let plan = comp_screen_close_plan();
    original_fn::<unsafe extern "C" fn()>(IDX_COMP_SCREENS_CLOSE_COMP_SCREEN)();
    let mut got = [0usize; 0x36];
    for (k, v) in got.iter_mut().enumerate() {
        *v = get_screen_param(k as u32);
    }
    SCOREBOARD.record(got == plan, || format!("competition screen (close): params {:x?} x plan {:x?}", got, plan));
}

/// Rule: the Table tab of a league stage of the competition screen. Before the draw date: "The draw will be made on
/// ..."; a competition not active: "The competition has not been scheduled yet."; otherwise the View popup (Overall,
/// Home Record, Away Record: the table type in screen parameter 4), the stage buttons, the title, and the table:
/// position, status letter, club, nation (national team competitions), played, won, drawn (or SOW / Pen by the match
/// rules), lost, for, against, points, with a divider row before each promotion/play-off/relegation band. A small
/// league (8 clubs or fewer, played in legs) also shows its fixtures from a week before its first round to three
/// weeks after its last (the list kept in screen parameter 5).
///
/// `comp_screens_draw_league_table_tab(stage, top, bottom)`: no stage: the comp_screens.cpp box at line 0x36f; no
/// competition (parameter 0): the box at 0x37b. Parameters read: 2 the stage index, 4 the table type, 5 the fixture
/// list. With a draw day (`league::NEXT_DRAW_DAY` ≠ 0xffff) on a date (year `league::YEAR` + the offset) still ahead of
/// today (years unsigned, days signed): with a `league::STAGE_ID` its name (the parent stage's when
/// `exe::comp_util_get_parent_stage_id` gives one) as a yellow title of 0x23 pixels and 0x28 down, then the sentence in
/// a pale grey text area (font 3). With the competition's `league::ACTIVE` zero: the not-scheduled sentence the same
/// way. Otherwise, with `league::LEGS` ≠ 0: [0xadae0c] = 1, the View button (0x6e, top, 0xeb, top + 0x14), its popup
/// area (flags 0x130) and the three popup items (id 6, data 0/1/2; the current type ticked, its text without the
/// leading space kept for the title). The stage buttons (`comp_screens_add_stage_selector`), 0x19 down; the title:
/// the stage name (or "League Table"), with the table type in brackets when not Overall; 0x28 down. Small league
/// (`league::CLUB_COUNT` ≤ 8, `league::LEGS` > 0, not the American major league): rows = clubs + 1 + one per band
/// (`league::PROMOTED_COUNT` or the champion flag of `league::LEAGUE_FLAGS`, and each of the three other counts), the
/// area exactly as high; otherwise the rows that fit down to `bottom`. National team competitions
/// (`league::CLUB_SIDES` = 2) move 4 of width from the club column to the nation column. The 11-column area, the
/// headings (Pld, Won, SOW/Pen/Drn, Lst, For, Ag, Pts); `league::TABLE_COLUMN` = the table type and
/// `exe::league_sort_table`. Each row k: the divider (an item of kind 1, flags 0x6400, x2 0x30a or 0x2f4 when the
/// table does not fit) when k is the first row after a band (yellow for promotion/champion and relegation, 0xaebdd4
/// for the play-off bands), then the position (an ordinal; a button with id 7 when the league has legs), the status
/// letter (C, Q, P, Pl, R by `league_table_row::STATUS`), the club (yellow when human), the nation, and the numbers of
/// the block of the table type (14 bytes per type; the Polish first division's second stage adds 14 or 7 to the
/// played games). Small league: the fixture list is built when parameter 5 is empty (the competition's virtual round
/// dates +0x3c, a week before the first, three weeks after the last, `exe::fixture_manager_find_fixtures` into a new
/// LLIST), then a 6-column area below the table: date (a button, id 8), home club, score (the extra-time score when
/// there is one; a button with id 0x7d3 when the match has statistics; "view" with id 0x7d2 for a watched match, "v"
/// otherwise), away club, the stadium (club competitions) or city, "-----"; empty rows fill the area. Last: the scroll
/// row of parameter 6 on the last area, parameter 7 = that area.
/// Original: `comp_screens_draw_league_table_tab` (0x491380, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_league_table_tab(stage: usize, top: i32, bottom: i32) {
    use league as l;
    use league_table_row as row;
    let mut weights: [u8; 11] = [4, 2, 0x10, 0, 4, 4, 4, 4, 4, 4, 4];
    let fixtures_weights: [u8; 6] = [3, 8, 2, 8, 8, 3];
    let mut text_buf = [0u8; 0x7d0];
    let mut title = [0u8; 0x7d0];
    if stage == 0 {
        return comp_screens_error_box(0x36f);
    }
    let comp = get_screen_param(0);
    if comp == 0 {
        return comp_screens_error_box(0x37b);
    }
    let stage_index = get_screen_param(2);
    let table_type = get_screen_param(4);
    let mut list = get_screen_param(5);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let black = job_colour(G_GUI_COLOUR_BLACK);
    let popup = job_colour(JOB_COLOUR_31AA);
    let draw_day = read_u16(stage + l::NEXT_DRAW_DAY);
    if draw_day != 0xffff {
        let mut date = [0u8; 8];
        let d = date.as_mut_ptr() as usize;
        exe::new_date(d, draw_day as u32, read_u16(stage + l::NEXT_DRAW_YEAR_OFFSET).wrapping_add(read_u16(stage + l::YEAR)) as u32);
        let (today_day, today_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
        let year = read_u16(d + 2);
        if today_year < year || (today_year == year && today_day < read_i16(d)) {
            covered!("tel22_2 league draw ahead");
            let mut y = top;
            let label = read_i16(stage + l::STAGE_ID);
            if label != 0 {
                let mut id = exe::comp_util_get_parent_stage_id(label as i32);
                if id == -1 {
                    id = label as i32;
                }
                exe::comp_util_get_stage_name(JOB_TEXT as *mut u8, 0x7d0, id as u32, 0);
                comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
                y += 0x28;
            }
            let rows = league_tab_rows(y, bottom);
            league_tab_draw_date_sentence(d, text_buf.as_mut_ptr());
            tel26_text_area([0x6e, y as u32, 0x30c, bottom as u32, 2, 0, rows, 3, pale, JOB_TEXT as u32, 0]);
            return;
        }
    }
    if read_u32(comp + l::ACTIVE) == 0 {
        let rows = league_tab_rows(top, bottom);
        comp_lang_format()(JOB_TEXT, 0x99c2bc); // "The competition has not been scheduled yet."
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, rows, 3, pale, JOB_TEXT as u32, 0]);
        return;
    }
    let mut y = top;
    let display = current_human_display();
    if read_i16(stage + l::LEGS) != 0 {
        comp_lang_format()(JOB_TEXT, 0x988004); // "View"
        write_u32(0xadae0c, 1);
        let button = tel26_add_button([0x6e, y as u32, 0xeb, (y + 0x14) as u32, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
        let area = tel26_add_area(display, [0, 0, 0, 0, 0, 0, 0, 0, 0x130, popup, button]);
        // the three popup items (id 6, data = the table type), the current one ticked and its text kept for the title
        for (kind, label) in [(0u32, 0x99c2b0usize), (1, 0x99c2a0), (2, 0x99c290)] { // " Overall", " Home Record", " Away Record"
            comp_lang_format()(JOB_TEXT, label);
            let flags = if table_type as u32 == kind { 0x2002 } else { 2 };
            tel26_add_item(display, [flags, 0, 0, 0, 0, 0, 0, 0x10, popup, yellow, 1, 1, black, JOB_TEXT as u32, 0, 6, kind, area]);
            if table_type as u32 == kind {
                copy_c_string(text_buf.as_mut_ptr() as usize, JOB_TEXT + 1);
            }
        }
    }
    league_tab_stage_selector(comp, stage_index, y);
    y += 0x19;
    let label = read_u16(stage + l::STAGE_ID);
    if label != 0 {
        exe::comp_util_get_stage_name(title.as_mut_ptr(), 0x7d0, label as u32, 0);
    } else {
        comp_lang_format()(title.as_mut_ptr() as usize, 0x99c280); // "League Table"
    }
    if table_type != 0 {
        comp_lang_format()(JOB_TEXT, 0x99c250, title.as_ptr(), text_buf.as_ptr()); // "%s (%s)"
    } else {
        copy_c_string(JOB_TEXT, title.as_ptr() as usize);
    }
    comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
    y += 0x28;
    let clubs = read_i16(stage + l::CLUB_COUNT);
    let small = clubs <= 8 && read_i16(stage + l::LEGS) > 0
        && read_u32(read_usize(stage + l::COMPETITION)) != read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE);
    let (rows, area_bottom);
    if small {
        covered!("tel22_2 league small table");
        let mut r = (clubs as u8).wrapping_add(1);
        if read_u8(stage + l::PROMOTED_COUNT) != 0 || read_u8(stage + l::LEAGUE_FLAGS) & 1 != 0 {
            r = r.wrapping_add(1);
        }
        for f in [l::PROMOTION_PLAYOFF_COUNT, l::RELEGATION_PLAYOFF_COUNT, l::RELEGATED_COUNT] {
            if read_u8(stage + f) != 0 {
                r = r.wrapping_add(1);
            }
        }
        rows = r;
        area_bottom = y + exe::font_get_height(3) * r as i32 + 0x10;
    } else {
        rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
        area_bottom = bottom;
    }
    if read_u8(comp + l::CLUB_SIDES) == 2 {
        weights[3] = weights[3].wrapping_add(4);
        weights[2] = weights[2].wrapping_sub(4);
    }
    let mut area = tel26_add_area(display, [0x6e, y as u32, 0x30c, area_bottom as u32, 0xb, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let heading = |col: u32| tel26_area_text([area, col, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32]);
    comp_lang_format()(JOB_TEXT, 0x990b3c); // "Pld"
    heading(4);
    comp_lang_format()(JOB_TEXT, 0x990b1c, 0xad9c64usize); // "Won"
    heading(5);
    let rules = read_u16(stage + l::MATCH_RULES);
    comp_lang_format()(JOB_TEXT, if rules & 0x20 != 0 { 0x990b18 } else if rules & 1 != 0 { 0x99c24c } else { 0x990b14 }); // SOW, Pen, Drn
    heading(6);
    comp_lang_format()(JOB_TEXT, 0x990af4, 0xad9c64usize); // "Lst"
    heading(7);
    comp_lang_format()(JOB_TEXT, 0x990af0); // "For"
    heading(8);
    comp_lang_format()(JOB_TEXT, 0x990aec); // "Ag"
    heading(9);
    comp_lang_format()(JOB_TEXT, 0x990ae8); // "Pts"
    heading(0xa);
    write_u32(stage + l::TABLE_COLUMN, table_type as u32);
    exe::league_sort_table(stage, 0usize);
    let block = (table_type as u32).wrapping_mul(14) as usize;
    let mut line: i32 = 1;
    let mut k: i32 = 0;
    while k < read_i16(stage + l::CLUB_COUNT) as i32 {
        let teams = read_i16(stage + l::CLUB_COUNT) as i32;
        let promoted = read_i8(stage + l::PROMOTED_COUNT) as i32;
        let playoff = read_i8(stage + l::PROMOTION_PLAYOFF_COUNT) as i32;
        let relegation_playoff = read_i8(stage + l::RELEGATION_PLAYOFF_COUNT) as i32;
        let relegated = read_i8(stage + l::RELEGATED_COUNT) as i32;
        // the band divider before row k
        let divider = if promoted == 0 && read_u8(stage + l::LEAGUE_FLAGS) & 1 != 0 && k == 1 {
            Some(yellow)
        } else if promoted > 0 && k == promoted {
            Some(yellow)
        } else if playoff > 0 && k == playoff + promoted {
            Some(job_colour(JOB_COLOUR_BDD4))
        } else if relegation_playoff > 0 && k == teams - relegation_playoff - relegated {
            Some(job_colour(JOB_COLOUR_BDD4))
        } else if relegated > 0 && k == teams - relegated {
            Some(yellow)
        } else {
            None
        };
        if let Some(colour) = divider {
            covered!("tel22_2 league band divider");
            let x2 = if teams as i16 <= rows as i16 { 0x30a } else { 0x30a - 0x16 }; // tel22_2 the divider width
            tel26_add_item(display, [1, 0x70, 0, x2, 0, 0, line as u32, 0x6400, colour, 0, 0, 0, 0, 0, 0, 0, 0, area]);
            line += 1;
        }
        let r = exe::league_get_table_row(stage, k);
        let club = Club(read_usize(r + row::CLUB));
        k += 1;
        let ordinal = exe::utils_format_ordinal(k, 3);
        if read_i16(stage + l::LEGS) != 0 {
            tel26_area_button([area, 0, line as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, ordinal as u32, 7, r as u32]);
        } else {
            tel26_area_text([area, 0, line as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, ordinal as u32]);
        }
        write_u8(JOB_TEXT, 0);
        let status = match read_u8(r + row::STATUS) {
            5 => Some(0x99c228usize), // "C"
            6 | 7 | 8 => Some(0x99c1b8), // "Q"
            0 => Some(0x99c20c), // "P"
            1 | 2 => Some(0x99c1d4), // "Pl"
            3 => Some(0x99c1f0), // "R"
            _ => None,
        };
        if let Some(fmt) = status {
            comp_lang_format()(JOB_TEXT, fmt, 0xad9c64usize);
            if read_u8(JOB_TEXT) != 0 {
                tel26_area_text([area, 1, line as u32, 0x30, job_colour(0xaebdcc), 0xc, 1, pale, JOB_TEXT as u32]);
            }
        }
        let club_colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { yellow } else { pale };
        tel26_area_button([area, 2, line as u32, 1, 0, 1, 3, club_colour, (club.addr() + club::SHORT_NAME) as u32, 0x7d5, club.addr() as u32]);
        if read_u8(comp + l::CLUB_SIDES) == 2 && club.nation().addr() != 0 {
            let nation = exe::database_get_national_team(club.nation().addr());
            tel26_area_button([area, 3, line as u32, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDD4), (club.nation().addr() + nation::THREE_LETTER_NAME) as u32, 0x7d5, nation as u32]);
        }
        let played = if read_u32(read_usize(stage + l::COMPETITION)) == read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) && read_u8(stage + l::STAGE_INDEX) == 1 {
            covered!("tel22_2 league polish second stage");
            if table_type == 0 { read_i8(r + row::PLAYED) as i32 + 0xe } else { read_i8(r + block + row::PLAYED) as i32 + 7 }
        } else {
            read_i8(r + block + row::PLAYED) as i32
        };
        let number = |col: u32, v: i32| {
            comp_crt_format()(JOB_TEXT, 0x98806c, v); // "%d"
            tel26_area_text([area, col, line as u32, 1, 0, 0xc, 2, pale, JOB_TEXT as u32]);
        };
        number(4, played);
        number(5, read_i8(r + block + row::WON) as i32);
        number(6, read_i8(r + block + row::DRAWN) as i32);
        number(7, read_i8(r + block + row::LOST) as i32);
        number(8, read_i16(r + block + row::GOALS_FOR) as i32);
        number(9, read_i16(r + block + row::GOALS_AGAINST) as i32);
        comp_crt_format()(JOB_TEXT, 0x98806c, read_i16(r + block + row::POINTS) as i32);
        tel26_area_text([area, 0xa, line as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, pale, JOB_TEXT as u32]);
        line += 1;
    }
    if small {
        let y = area_bottom + 5;
        if list == 0 {
            covered!("tel22_2 league fixture list built");
            // the competition's round dates (virtual +0x3c): an array of 0x41-byte entries, day and year offset
            let (mut count, mut a, mut b) = (0u32, 0u32, 0u32);
            let rounds: unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u32, *mut u32) -> usize = core::mem::transmute(read_usize(read_usize(comp) + 0x3c));
            let dates = rounds(comp, stage_index, &mut count, &mut a, &mut b);
            let count = count as i16 as isize;
            let (mut first, mut last, mut tmp) = ([0u8; 8], [0u8; 8], [0u8; 8]);
            let first_p = first.as_mut_ptr() as usize;
            let last_p = last.as_mut_ptr() as usize;
            let tmp_p = tmp.as_mut_ptr() as usize;
            let mut shifted = [0u8; 8];
            exe::new_date(first_p, read_u16(dates) as u32, read_u16(dates + 2).wrapping_add(read_u16(comp + l::YEAR)) as u32);
            let t = exe::date_sub_weeks(first_p, &mut shifted, 1);
            exe::date_operator_3d(first_p, tmp_p, t);
            let e = (dates as isize + count * 0x41 - 0x41) as usize;
            exe::new_date(last_p, read_u16(e) as u32, read_u16(e + 2).wrapping_add(read_u16(comp + l::YEAR)) as u32);
            let t = exe::date_add_weeks(last_p, &mut shifted, 3);
            exe::date_operator_3d(last_p, tmp_p, t);
            exe::crt_free(dates);
            let node = exe::crt_operator_new(0xe);
            list = if node != 0 { exe::new_llist(node) } else { 0 };
            set_screen_param(5, list);
            exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u32(first_p), read_u32(first_p + 2), read_u32(last_p),
                read_u32(last_p + 2), u32::MAX, list, read_u32(read_usize(comp + l::COMPETITION)), stage_index as u32, u32::MAX);
        }
        let rows = ((bottom - y) / exe::font_get_height(2)) as u8 as i32;
        area = tel26_add_area(display, [0x6e, y as u32, 0x30c, bottom as u32, 6, fixtures_weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
        let mut line: i32 = 0;
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        let mut f = exe::llist_get_next(list);
        while f != 0 {
            if read_u8(f + fixture::HOME_GOALS) != 0xfe && read_u8(f + fixture::AWAY_GOALS) != 0xfd {
                league_tab_fixture_row(comp, area, line, f);
                line += 1;
            }
            f = exe::llist_get_next(list);
        }
        while line < rows {
            for col in 0..8u32 {
                tel26_area_text([area, col, line as u32, 2, 0, 1, 2, pale, 0]);
            }
            line += 1;
        }
    }
    let scroll = get_screen_param(6);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row((display as isize + 4 + area as i16 as isize * 0xbf1) as usize, scroll);
    }
    set_screen_param(7, area as i16 as isize as usize);
}

/// Comparer for `comp_screens_draw_league_table_tab`: side by side over the display list (`compare_match_screen_build`)
/// plus the stage's table type.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_league_table_tab(stage: usize, top: i32, bottom: i32) {
    let stage = League(stage);
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_LEAGUE_TABLE_TAB);
    // simplification: the error boxes and the first build of a small league's fixture list (operator new, kept in
    // parameter 5) do not run twice: original only; the copy is proven in Rust mode
    if stage.is_null() || get_screen_param(0) == 0 || (get_screen_param(5) == 0 && stage.club_count() <= 8 && stage.legs() > 0) {
        return o(stage.addr(), top, bottom);
    }
    compare_match_screen_build("competition screen, league table tab", &[(stage.addr() + league::TABLE_COLUMN, 4)], || o(stage.addr(), top, bottom),
        || comp_screens_draw_league_table_tab(stage.addr(), top, bottom));
}

/// Rule: the Draw tab of a cup stage of the competition screen. Before the first draw: "The draw has not yet been
/// made." or "The draw will be made on ..."; afterwards the round of screen parameter 3 (−2: the last drawn) with its
/// name, the draw buttons (Draw Next Team, Draw All Teams, Automatic Draw) while the round is still to be drawn, and
/// the ties: the possible home teams (with their division or nation), the score of each leg, the away teams once the
/// tie is decided, and, when the round is drawn and the next one is not the third-place round, the teams given a bye.
///
/// `comp_screens_draw_cup_rounds_tab(cup, top, bottom)`: no cup: the box at line 0x69a; no competition (parameter 0):
/// 0x6a6. Parameters 2 (the stage) and 3 (the round; −2 becomes `cup::NEXT_DRAW_ROUND` − 1, written back). The stage
/// buttons, 0x19 down (and [0xadae0c] = 1) when there are some. No draw yet (`cup::NEXT_DRAW_ROUND` = 0): the stage
/// label's name as the title, then not active (`cup::SEASON_SET_UP` zero) "The draw has not yet been made.", otherwise
/// the draw date (round table +0 day, +2 signed year offset) sentence, in a pale grey text area. Otherwise: the round
/// name (`exe::comp_util_format_stage_round_name`) as the title when not empty. Round ≥ `cup::CURRENT_ROUND` and its
/// code not 0x8c: a 5-column area (4 without weights in languages 2, 4 and 5) with the three draw buttons (ids 9, 0xa,
/// 0xb; the automatic one with value 1 unless parameter 0x35; plain texts when the draw cannot go on: the previous round
/// still undecided, or this round's last tie decided); parameter 0x34 = the cup while this round's last tie is not
/// decided; 0x19 down. The 9-column ties area; for each tie with status ≠ 1 (`exe::get_fixture_side_teams` for both
/// sides): the home teams one per row (division/nation in column 0, the name in column 2), the score row (columns
/// 1/7 the penalties or the walkover/away-goals marks, 3/5 the result marks, 4 the goals "x : y" with ":" a button to
/// the match statistics, id 0x7d3, or "v" when not played), and with status 3 the away teams (column 6, origin in 8);
/// the next tie starts after the longer side. Then, the round decided (or not yet current) and a next round that is
/// not the third-place one: the teams of rounds 0..round (`cup_round::NEW_TEAMS` summed) not in any of this round's
/// ties and still in (status 0), under the "Teams given bye" heading. Parameters 0x32 the last tie shown, 0x33 its
/// flag, " N teams to draw" in the draw area (2 × the remaining ties − 2, + 1 with the flag), parameter 6 = the rows
/// when −1, the scroll row on the ties area and parameter 7 = that area.
/// Defect of the original: the row after a tie with status 1 is the larger of the previous ends (uninitialised stack
/// for the first tie; 0 here).
/// Original: `comp_screens_draw_cup_rounds_tab` (0x492980, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_cup_rounds_tab(cup_: usize, top: i32, bottom: i32) {
    use cup as c;
    let ties_weights: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];
    let draw_weights: [u8; 5] = [2, 2, 2, 2, 2];
    let mut draw_area: u32 = u32::MAX;
    if cup_ == 0 {
        return comp_screens_error_box(0x69a);
    }
    let comp = get_screen_param(0);
    if comp == 0 {
        return comp_screens_error_box(0x6a6);
    }
    let stage_index = get_screen_param(2);
    let mut round = get_screen_param(3) as i32;
    if round == -2 {
        round = read_i8(cup_ + c::NEXT_DRAW_ROUND) as i32 - 1;
        set_screen_param(3, round as isize as usize);
    }
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let marks = job_colour(JOB_COLOUR_BDE8);
    let mut y = top;
    if league_tab_stage_selector(comp, stage_index, y) != 0 {
        write_u32(0xadae0c, 1);
        y += 0x19;
    }
    let rounds = read_usize(cup_ + c::ROUND_TABLE);
    if read_u8(cup_ + c::NEXT_DRAW_ROUND) == 0 {
        covered!("tel22_2 cup no draw yet");
        let label = read_u16(cup_ + c::STAGE_LABEL);
        if label != 0 {
            exe::comp_util_get_stage_name(JOB_TEXT as *mut u8, 0x7d0, label as u32, 0);
            comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
            y += 0x28;
        }
        let rows = league_tab_rows(y, bottom);
        if read_u32(cup_ + c::SEASON_SET_UP) == 0 {
            comp_lang_format()(JOB_TEXT, 0x99c3bc); // "The draw has not yet been made."
        } else {
            let mut date = [0u8; 8];
            let mut buf = [0u8; 0x7d0];
            exe::new_date(date.as_mut_ptr() as usize, read_u16(rounds) as u32,
                (read_i8(rounds + 2) as i16).wrapping_add(read_i16(cup_ + c::YEAR)) as u16 as u32);
            league_tab_draw_date_sentence(date.as_mut_ptr() as usize, buf.as_mut_ptr());
        }
        tel26_text_area([0x6e, y as u32, 0x30c, bottom as u32, 2, 0, rows, 3, pale, JOB_TEXT as u32, 0]);
        return;
    }
    let rec = (round as isize * 0x68) as usize;
    exe::comp_util_format_stage_round_name(read_i16(cup_ + c::STAGE_LABEL) as i32, read_i16(rounds.wrapping_add(rec) + cup_round::ROUND_CODE) as i32, JOB_TEXT, 0x7d0, 0);
    if read_u8(JOB_TEXT) != 0 {
        comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
        y += 0x28;
    }
    let current = read_i8(cup_ + c::CURRENT_ROUND) as i32;
    if round >= current && read_u16(rounds.wrapping_add(rec) + cup_round::ROUND_CODE) != 0x8c {
        covered!("tel22_2 cup draw buttons");
        let lang = exe::langlib_get_language();
        let narrow = lang == 5 || exe::langlib_get_language() == 4 || exe::langlib_get_language() == 2;
        let display = current_human_display();
        draw_area = if !narrow {
            tel26_add_area(display, [0x6e, y as u32, 0x30c, (y + 0x14) as u32, 5, draw_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX])
        } else {
            tel26_add_area(display, [0x6e, y as u32, 0x30c, (y + 0x14) as u32, 4, 0, 1, 0, 1, 0, u32::MAX])
        };
        let can_draw = (round <= current || cup_tab_last_tie_status(cup_, round - 1) == 3) && cup_tab_last_tie_status(cup_, round) != 3;
        let button = |col: u32, label: usize, id: u32| {
            comp_lang_format()(JOB_TEXT, label);
            if can_draw {
                tel26_area_button([draw_area, col, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, id, 0]);
            } else {
                tel26_area_text([draw_area, col, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
            }
        };
        button(0, 0x99c3ac, 9); // "Draw Next Team"
        button(1, 0x99c39c, 0xa); // "Draw All Teams"
        comp_lang_format()(JOB_TEXT, 0x99c38c); // "Automatic Draw"
        if can_draw {
            if get_screen_param(0x35) != 0 {
                tel26_area_button([draw_area, 2, 0, 0x70, grey, 0xc, 1, pale, JOB_TEXT as u32, 0xb, 0]);
            } else {
                tel26_area_button([draw_area, 2, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0xb, 1]);
            }
        } else {
            tel26_area_text([draw_area, 2, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
        }
        if cup_tab_last_tie_status(cup_, round) != 3 {
            set_screen_param(0x34, cup_);
        }
        y += 0x19;
    }
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 9, ties_weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut line: i32 = 0;
    let mut last_tie: i32 = -1;
    let mut shown: u32 = 0;
    let (mut home_end, mut away_end): (i32, i32) = (0, 0);
    let ties = |r: usize| read_i16(read_usize(cup_ + c::ROUND_TABLE).wrapping_add(r) + cup_round::TIES) as i32;
    let mut home = [0i16; 100];
    let mut away = [0i16; 100];
    let mut k: i32 = 0;
    while k < ties(rec) {
        let tie = (read_usize((read_usize(cup_ + c::TIE_TABLES) as isize + round as isize * 4) as usize) as isize + k as isize * 0x35) as usize;
        let n_home = exe::cup_get_fixture_side_teams(cup_, round, k, 1, home.as_mut_ptr());
        let n_away = exe::cup_get_fixture_side_teams(cup_, round, k, 0, away.as_mut_ptr());
        if read_u8(tie + 0x34) != 1 {
            last_tie = k;
            home_end = line;
            shown = 1;
            for j in 0..n_home.max(0) {
                let club = cup_tab_team_club(cup_, home[j as usize]);
                cup_tab_team_origin(comp, area, 0, home_end, club);
                cup_tab_team_name(area, 2, home_end, club, j == n_home - 1);
                home_end += 1;
            }
            if read_u8(tie + 0x14) == 0xff {
                comp_lang_format()(JOB_TEXT, 0x99c1b4); // "v"
                tel26_area_text([area, 4, line as u32, 1, 0, 0xc, 3, marks, JOB_TEXT as u32]);
            } else {
                cup_tab_score_row(cup_, area, line, round, k, tie);
            }
        }
        if read_u8(tie + 0x34) == 3 {
            covered!("tel22_2 cup tie decided");
            away_end = line;
            shown = 0;
            for j in 0..n_away.max(0) {
                let club = cup_tab_team_club(cup_, away[j as usize]);
                cup_tab_team_name(area, 6, away_end, club, j == n_away - 1);
                cup_tab_team_origin(comp, area, 8, away_end, club);
                away_end += 1;
            }
        }
        line = if home_end > away_end { home_end } else { away_end };
        k += 1;
    }
    let drawn = round < current || cup_tab_last_tie_status(cup_, round) == 3;
    if drawn && round + 1 < read_i16(cup_ + c::ROUND_COUNT) as i32 && read_u16(rounds.wrapping_add(rec) + 0x68 + cup_round::ROUND_CODE) != 0x8c {
        covered!("tel22_2 cup byes");
        let mut total: i16 = 0;
        for r in 0..=round {
            total = total.wrapping_add(read_i16(rounds + r as usize * 0x68 + cup_round::NEW_TEAMS));
        }
        let total = total as i32;
        let mask = exe::crt_calloc(1, total as u32);
        for t in 0..ties(rec) {
            let n_home = exe::cup_get_fixture_side_teams(cup_, round, t, 1, home.as_mut_ptr());
            let n_away = exe::cup_get_fixture_side_teams(cup_, round, t, 0, away.as_mut_ptr());
            for j in 0..n_home.max(0) {
                if (home[j as usize] as i32) < total as i16 as i32 {
                    write_u8((mask as isize + home[j as usize] as isize) as usize, 1);
                }
            }
            for j in 0..n_away.max(0) {
                if (away[j as usize] as i32) < total as i16 as i32 {
                    write_u8((mask as isize + away[j as usize] as isize) as usize, 1);
                }
            }
        }
        let mut heading = false;
        for i in 0..total {
            if read_u8(mask + i as usize) != 0 || read_u8(read_usize(cup_ + c::TEAM_TABLE) + i as usize * 6 + 5) != 0 {
                continue;
            }
            if !heading {
                comp_lang_format()(JOB_TEXT, 0x99c33c); // "Teams given bye"
                line += 1;
                let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) + 0x73) as u32;
                tel26_add_item(current_human_display(), [1, 0x73, 0, x2, 0, 0, line as u32, 1, 0, 0, 1, 3, yellow, JOB_TEXT as u32, 0, 0, 0, area]);
                line += 1;
                tel26_add_item(current_human_display(), [1, 0x73, 0, x2, 0, 0, line as u32, 0xc400, yellow, 0, 0, 0, 0, 0, 0, 0, 0, area]);
                line += 1;
                heading = true;
            }
            let club = read_usize(read_usize(cup_ + c::TEAM_TABLE) + i as usize * 6);
            cup_tab_team_origin(comp, area, 0, line, club);
            let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, 2, line as u32, 1, 0, 1, 3, colour, (club + club::SHORT_NAME) as u32, 0x7d5, club as u32]);
            line += 1;
        }
        exe::crt_free(mask);
    }
    set_screen_param(0x32, last_tie as i16 as isize as usize);
    set_screen_param(0x33, shown as usize);
    if draw_area as u16 != 0xffff {
        let left = (ties(rec) - last_tie as i16 as i32) * 2 - 2 + shown as i32;
        comp_lang_format()(JOB_TEXT, 0x99c314, left); // " %d teams to draw"
        let col = if exe::langlib_get_language() == 5 || exe::langlib_get_language() == 4 || exe::langlib_get_language() == 2 { 3 } else { 4 };
        tel26_area_text([draw_area, col, 0, 2, 0, 0xc, 1, job_colour(0xae3152), JOB_TEXT as u32]);
    }
    if get_screen_param(6) as i32 == -1 {
        set_screen_param(6, line as isize as usize);
    }
    let scroll = get_screen_param(6);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row((current_human_display() as isize + 4 + area as i16 as isize * 0xbf1) as usize, scroll);
    }
    set_screen_param(7, area as i16 as isize as usize);
}

/// Comparer for `comp_screens_draw_cup_rounds_tab`: side by side over the display list (`compare_match_screen_build`;
/// the params it writes are in the screen's parameter block).
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_cup_rounds_tab(cup_: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_CUP_ROUNDS_TAB);
    // simplification: the error boxes (no cup, no competition) only through the original
    if cup_ == 0 || get_screen_param(0) == 0 {
        return o(cup_, top, bottom);
    }
    compare_match_screen_build("competition screen, cup draw tab", &[], || o(cup_, top, bottom), || comp_screens_draw_cup_rounds_tab(cup_, top, bottom));
}

/// Builds the Fixtures tab of the competition screen: one match date, its matches round by round, and the buttons to
/// the previous and next dates.
///
/// No competition: the comp_screens.cpp error box (line 0xaf5). The date comes from parameters 0x11 (day) and 0x12
/// (year), the neighbours from 0x13/0x14 and 0x15/0x16, the match list from 0x17. With no list yet: the current date
/// when the year is 0; a new LLIST (operator new 0xe) in parameter 0x17, filled with every match of the competition from
/// that date (`fixtures_tab_scan` forward from slot 0, all), sorted (`comp_util_sort_fixture_list`) or, with none, the
/// year 1; the day and year back to 0x11/0x12. Year 1: "No fixtures have been scheduled." in a text area. A neighbour
/// year 0 is looked for (`fixtures_tab_neighbour_date`: the previous date first, then the next). Then the two-column
/// area of the "<< Date" and "Date >>" cells: a button (ids 0x10 and 0x11) when that neighbour year is above 1, plain
/// text otherwise; the long date (without the year in the current year) as the heading; and the nine-column list
/// (`FIXTURES_TAB_WEIGHTS`, as many rows as fit in font 3): at each new stage and round (stage or round not 0, and
/// different from the previous match's) an empty row (except the first), the round's name
/// (`comp_util_format_stage_round_name`, up to 0x2f1 wide), a link (id 0x12, value round + 1000 × (stage + 1)) when the
/// round's fixtures are known or plain text, and an underline row (flags 0xc400); every match a row
/// (`comp_screens_add_fixture_row`, mode 0x13). The scroll row comes back from parameter 0x18 and the area goes to 0x19.
/// Original: `comp_screens_draw_fixtures_tab` (0x493ee0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_fixtures_tab(comp: usize, top: i32, bottom: i32) {
    let mut scratch = [0u8; 0x10];
    let lp = scratch.as_mut_ptr() as usize;
    exe::new_llist(lp);
    let weights = FIXTURES_TAB_WEIGHTS;
    if comp == 0 {
        comp_screens_error_box(0xaf5);
        exe::drop_llist(lp);
        return;
    }
    let mut day = get_screen_param(0x11) as u32;
    let mut year = get_screen_param(0x12) as u32;
    let _prev_day = get_screen_param(0x13);
    let mut prev_year = get_screen_param(0x14) as u32;
    let _next_day = get_screen_param(0x15);
    let mut next_year = get_screen_param(0x16) as u32;
    let mut list = get_screen_param(0x17);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    if list == 0 {
        covered!("tel22_3 fixtures list built");
        if year as u16 == 0 {
            day = read_u32(G_CURRENT_DATE);
            year = (year & 0xffff_0000) | read_u16(G_CURRENT_DATE_YEAR) as u32;
        }
        let block = exe::crt_operator_new(0xe);
        list = if block != 0 { exe::new_llist(block) } else { 0 };
        set_screen_param(0x17, list);
        let mut slot: u8 = 0;
        let comp_id = read_u32(read_usize(comp + 4));
        if fixtures_tab_scan(&mut year, &mut day, &mut slot, 1, comp_id, -2, list) == 0 {
            year = 1;
        } else {
            exe::comp_util_sort_fixture_list(list);
        }
        set_screen_param(0x11, (day & 0xffff) as usize);
        set_screen_param(0x12, (year & 0xffff) as usize);
    }
    if year as u16 == 1 {
        covered!("tel22_3 fixtures none");
        let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
        comp_lang_format()(JOB_TEXT, 0x99c3dc); // "No fixtures have been scheduled."
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, rows as u32, 3, pale, JOB_TEXT as u32, 0]);
        exe::drop_llist(lp);
        return;
    }
    if prev_year as u16 == 0 {
        prev_year = fixtures_tab_neighbour_date(day, year, 0, read_u32(read_usize(comp + 4)), lp, 0x13);
    }
    if next_year as u16 == 0 {
        next_year = fixtures_tab_neighbour_date(day, year, 1, read_u32(read_usize(comp + 4)), lp, 0x15);
    }
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x168, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let mut label = [0u8; 0x100];
    let lb = label.as_mut_ptr() as usize;
    comp_lang_format()(lb, 0x988078); // "Date"
    comp_crt_format()(JOB_TEXT, 0x988090, lb); // "<< %s"
    if prev_year as u16 > 1 {
        tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x10, 0]);
    } else {
        tel26_area_text([area, 0, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
    }
    comp_lang_format()(lb, 0x988078);
    comp_crt_format()(JOB_TEXT, 0x988088, lb); // "%s >>"
    if next_year as u16 > 1 {
        tel26_area_button([area, 1, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x11, 0]);
    } else {
        tel26_area_text([area, 1, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
    }
    let y = top + 0x19;
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    exe::new_date(dp, day, year);
    if year as u16 == read_u16(G_CURRENT_DATE_YEAR) {
        exe::date_get_long_date_string_no_year(dp, JOB_TEXT, 0x7d0, 0xff);
    } else {
        exe::date_get_long_date_string(dp, JOB_TEXT, 0x7d0, 0xff);
    }
    comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
    let y = y + 0x28;
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(),
        [0x6e, y as u32, 0x30c, bottom as u32, 9, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut row: i32 = 0;
    let mut prev: usize = 0;
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut f = exe::llist_get_next(list);
    while f != 0 {
        let same = prev != 0 && read_i16(f + 0x32) == read_i16(prev + 0x32) && read_i16(f + 0x30) == read_i16(prev + 0x30);
        if !same && !(read_i16(f + 0x32) == 0 && read_i16(f + 0x30) == 0) {
            if prev != 0 {
                tel26_area_text([area, 0, row as u32, 1, 0, 1, 1, pale, 0]);
                row += 1;
            }
            exe::comp_util_format_stage_round_name(read_i16(f + 0x32) as i32, read_i16(f + 0x30) as i32, JOB_TEXT, 0x7d0, 0);
            let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) + 0x73).min(0x2f1);
            if fixtures_tab_round_known(comp, read_i8(f + 0x42) as i32, read_i16(f + 0x34) as i32) != 0 {
                covered!("tel22_3 fixtures round link");
                let value = (read_i16(f + 0x34) as i32).wrapping_add((read_i8(f + 0x42) as i32 + 1).wrapping_mul(1000));
                tel26_add_item(current_human_display(), [2, 0x73, 0, x2 as u32, 0, 0, row as u32, 1, 0, 0, 1, 3, yellow,
                    JOB_TEXT as u32, 0, 0x12, value as u32, area]);
            } else {
                tel26_add_item(current_human_display(), [1, 0x73, 0, x2 as u32, 0, 0, row as u32, 1, 0, 0, 1, 3, yellow,
                    JOB_TEXT as u32, 0, 0, 0, area]);
            }
            row += 1;
            tel26_add_item(current_human_display(), [1, 0x73, 0, x2 as u32, 0, 0, row as u32, 0xc400, yellow, 0, 0, 0, 0,
                0, 0, 0, 0, area]);
            row += 1;
        }
        exe::comp_screens_add_fixture_row(f, area as usize, &mut row, 0x13);
        prev = f;
        f = exe::llist_get_next(list);
    }
    comp_tabs_restore_scroll(area, 0x18, 0x19);
    exe::drop_llist(lp);
}

/// Builds the Schedule tab of the competition screen: one row per match date of the competition, with the date (a link)
/// and the stage and round played on it.
///
/// No competition: the error box (line 0xc24). A competition not yet scheduled (`comp + 0x45` = 0): "The competition
/// has not been scheduled yet." in a text area. The date list comes from parameters 0x1a (the 0x14-byte entries) and
/// 0x1b (their count); with none yet it is built (`Comp::build_match_date_list`) and kept there. A two-column area
/// (weights 1 and 4, as many rows as fit in font 3); an entry repeating the previous one's date, stage (the parent stage
/// when there is one, `comp_util_get_parent_stage_id`) and round is skipped. The name: stage 0 and round 0 give "Game n"
/// (n = the entry's round + 1) in a league division, or the competition's short name; otherwise
/// `comp_util_format_stage_round_name`. Column 0 is the short weekday date (a link, id 0x14, value the entry); column 1
/// the name indented, a link (id 0x15, value round + 1000 × (first stage of the group + 1)) when the round's fixtures are
/// known, text in the 0xae3154 colour otherwise. Empty rows fill the area; the scroll row comes back from parameter
/// 0x1c and the area goes to 0x1d.
/// Original: `comp_screens_draw_schedule_tab` (0x4947a0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_schedule_tab(comp: usize, top: i32, bottom: i32) {
    let weights: [u8; 2] = [1, 4];
    if comp == 0 {
        comp_screens_error_box(0xc24);
        return;
    }
    let mut list = get_screen_param(0x1a);
    let mut count = get_screen_param(0x1b) as i32;
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    if read_u32(comp + 0x45) == 0 {
        covered!("tel22_3 schedule not yet");
        let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
        comp_lang_format()(JOB_TEXT, 0x99c2bc); // "The competition has not been scheduled yet."
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, rows as u32, 3, pale, JOB_TEXT as u32, 0]);
        return;
    }
    if list == 0 {
        covered!("tel22_3 schedule list built");
        list = exe::comp_build_match_date_list(comp, &mut count);
        set_screen_param(0x1a, list);
        set_screen_param(0x1b, count as usize);
    }
    let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(),
        [0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let (mut prev_stage, mut prev_round, mut prev_entry) = (-1i32, -1i32, 0usize);
    let mut row: i32 = 0;
    let mut name = [0u8; 0x800];
    let nb = name.as_mut_ptr() as usize;
    let mut k: i32 = 0;
    let mut e = list;
    while k < count {
        let mut stage = read_i16(e + 0x10) as i32;
        let round = read_i16(e + 0x12) as i32;
        let parent = exe::comp_util_get_parent_stage_id(stage);
        if parent > -1 {
            stage = parent;
        }
        let same = prev_entry != 0 && read_i16(e + 0xc) == read_i16(prev_entry + 0xc) && read_i16(e + 0xe) == read_i16(prev_entry + 0xe)
            && stage == prev_stage && round == prev_round;
        if !same {
            if stage == 0 && round == 0 {
                let record = read_usize(comp + 4);
                if exe::comp_util_is_league_division(read_i32(record)) != 0 {
                    comp_lang_format()(nb, 0x99c400, read_i32(e + 4) + 1); // "Game %d"
                } else {
                    copy_c_string(nb, record + competition::SHORT_NAME);
                }
            } else {
                exe::comp_util_format_stage_round_name(stage, round, nb, 0x7d0, 0);
            }
            let mut d = [0u8; 8];
            let dp = d.as_mut_ptr();
            exe::new_date(dp as usize, read_u16(e + 0xc) as u32, read_u16(e + 0xe) as u32);
            exe::date_get_short_weekday_date_string(dp, JOB_TEXT as *mut u8, 0x7d0, 0xff);
            tel26_area_button([area, 0, row as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, JOB_TEXT as u32, 0x14, e as u32]);
            let known = fixtures_tab_round_known(comp, read_i32(e), read_i32(e + 4));
            comp_crt_format()(JOB_TEXT, 0x987ff0, nb); // "  %s"
            if known != 0 {
                let g = exe::comp_get_first_stage_of_group(comp, read_i32(e));
                let value = read_i32(e + 4).wrapping_add((g + 1).wrapping_mul(1000));
                tel26_area_button([area, 1, row as u32, 2, 0, 1, 2, pale, JOB_TEXT as u32, 0x15, value as u32]);
            } else {
                tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, job_colour(COMP_TABS_COLOUR_3154), JOB_TEXT as u32]);
            }
            prev_stage = stage;
            prev_round = round;
            prev_entry = e;
            row += 1;
        }
        k += 1;
        e += 0x14;
    }
    while row < rows as i32 {
        tel26_area_text([area, 0, row as u32, 2, 0, 1, 2, pale, 0]);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, pale, 0]);
        row += 1;
    }
    comp_tabs_restore_scroll(area, 0x1c, 0x1d);
}

/// Builds the Seedings tab of the competition screen: the seeded clubs in two halves side by side, each with its
/// seeding.
///
/// The seedings are the competition's 6-byte entries (`comp + 0x14`, count `comp + 0x36`: the club, the kind byte at
/// +4; kind 0 is not seeded). Rows: as many as fit in font 3 (the whole height); the list takes twice that when the
/// seeded clubs fit, otherwise their count rounded up to even. A four-column area; the first half goes in columns 0
/// and 1, the second in 2 and 3 (rows from the top again). Each slot takes the next seeded entry: the club indented, a
/// link (id 0x7d5, value the club) in yellow for a human's club, pale grey otherwise; its kind in column + 1: 3
/// "Seeded", 4 "Intertoto" and 5 "European Cup" (both links, id 0x7d0, to the competition records of [0x9cf6f8] and
/// [0x9cf6e8]), 7 "First seeds", 8 "Second seeds", 10 to 12 "Pool 1" to "Pool 3", nothing else. Slots past the last
/// entry are empty.
/// Original: `comp_screens_draw_seedings_tab` (0x494c60, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_seedings_tab(comp: usize, top: i32, bottom: i32) {
    let count = read_i16(comp + 0x36);
    let mut seeded: i16 = 0;
    if count > 0 {
        let mut p = read_usize(comp + 0x14) + 4;
        for _ in 0..count {
            if read_u8(p) != 0 {
                seeded = seeded.wrapping_add(1);
            }
            p += 6;
        }
    }
    let rows = ((bottom - top) / exe::font_get_height(3)) as i8;
    let total: i16 = if seeded as i32 <= rows as i32 * 2 {
        (rows as i16) << 1
    } else {
        covered!("tel22_3 seedings longer than the area");
        (((seeded as i32 + 1) / 2) * 2) as i16
    };
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 4, 0, rows as u8 as u32, 0, 0, 0, u32::MAX]);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let mut entry: i16 = 0;
    let mut counter: i16 = 0;
    let mut i: i32 = 0;
    if total <= 0 {
        return;
    }
    loop {
        let n = read_i16(comp + 0x36);
        if entry < n {
            while read_u8(read_usize(comp + 0x14) + entry as isize as usize * 6 + 4) == 0 {
                entry += 1;
                if entry >= n {
                    break;
                }
            }
        }
        let half = total as i32 / 2;
        let (col, row) = if i < half {
            (0u32, counter as u8 as i8 as i32)
        } else {
            (2u32, (counter as u8).wrapping_sub(half as u8) as i8 as i32)
        };
        if entry < n {
            let e = read_usize(comp + 0x14) + entry as isize as usize * 6;
            let club = Club(read_usize(e));
            comp_crt_format()(JOB_TEXT, 0x987ff0, club.addr() + club::SHORT_NAME); // "  %s"
            let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, col, row as u32, 2, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, read_u32(e)]);
            let label = match read_i8(e + 4) as i32 {
                3 => 0x99c468,  // "Seeded"
                4 => 0x99c440,  // "Intertoto"
                5 => 0x99c430,  // "European Cup"
                7 => 0x99c45c,  // "First seeds"
                8 => 0x99c44c,  // "Second seeds"
                10 => 0x99c428, // "Pool 1"
                11 => 0x99c420, // "Pool 2"
                12 => 0x99c418, // "Pool 3"
                _ => 0usize,
            };
            if label != 0 {
                comp_lang_format()(JOB_TEXT, label);
            } else {
                write_u8(JOB_TEXT, 0);
            }
            let kind = read_u8(e + 4);
            if kind == 4 || kind == 5 {
                covered!("tel22_3 seedings european link");
                let id = read_u32(if kind == 4 { SEEDINGS_TAB_COMP_KIND_4 } else { SEEDINGS_TAB_COMP_KIND_5 }) as usize;
                let record = read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(SEEDINGS_TAB_COMPETITION_SIZE));
                tel26_area_button([area, col + 1, row as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32, 0x7d0, record as u32]);
            } else {
                tel26_area_text([area, col + 1, row as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32]);
            }
        } else {
            tel26_area_text([area, col, row as u32, 2, 0, 1, 2, pale, 0]);
            tel26_area_text([area, col + 1, row as u32, 2, 0, 1, 2, pale, 0]);
        }
        counter = counter.wrapping_add(1);
        i += 1;
        entry = entry.wrapping_add(1);
        if counter >= total {
            break;
        }
    }
}

/// Builds the Average Points tab of the competition screen (the relegation averages): each club's points in the last
/// two or three seasons, the total, the games and the average.
///
/// The stage comes from parameter 0x2a (−1 the competition itself; a stage past the last or without an averages table,
/// `+ 0x18`, also gives −1). The competition's virtual slot 0x34 runs with 0 first (`update_relegation_averages`, a
/// no-op except in the Argentine second division). When any stage or the competition has an averages table: a "Stage"
/// menu entry for each (`comp_screens_add_stage_menu_item`, at x 0x28f), the chosen one's name as the heading
/// (`comp_util_get_stage_name`) and the list 0x41 lower. Three seasons (8 columns, weights 2 10 4 4 4 4 4 4) unless the
/// first entry's third season games are 0xff (then 2 and 7 columns); the headings are the seasons
/// ("year/year+1" from the competition's table year back), "Total", "Played" and "Average". Each club entry (0x13 bytes;
/// club −1 skipped): its position, its name (a link, yellow for a human's club), the points of each season, their total,
/// the games of the seasons and the average ("%5.3f").
/// Original: `comp_screens_draw_average_points_tab` (0x495050, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_average_points_tab(comp: usize, top: i32, bottom: i32) {
    let weights8: [u8; 8] = [2, 0xa, 4, 4, 4, 4, 4, 4];
    let weights7: [u8; 7] = [2, 0xa, 4, 4, 4, 4, 4];
    let mut menu_state: i32 = -1;
    let stage_at = |i: i32| read_usize(read_usize(comp + 0xc) + (i as isize * 4) as usize);
    let mut chosen = get_screen_param(0x2a) as u8 as i8;
    if chosen as i32 > read_i32(comp + 0x30) {
        chosen = -1;
    } else if chosen > -1 && read_usize(stage_at(chosen as i32) + 0x18) == 0 {
        chosen = -1;
    }
    // comp_vtable slot 0x34: update_relegation_averages(0) (ArgSecond); the others pop the argument and return
    let update: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(comp) + 0x34));
    update(comp, 0);
    let stages = read_i32(comp + 0x30);
    let mut has_table = false;
    if stages >= 0 {
        let mut i: i8 = 0;
        loop {
            if read_usize(stage_at(i as i32) + 0x18) != 0 {
                has_table = true;
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 > stages {
                break;
            }
        }
    }
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let mut obj = comp;
    let mut y = top;
    if has_table {
        covered!("tel22_3 average points stage menu");
        type MenuItem = unsafe extern "C" fn(*mut i32, u32, i32, u32, i32, u32, i32, u32, i32, u32);
        let menu_item: MenuItem = core::mem::transmute(FN_COMP_SCREENS_ADD_STAGE_MENU_ITEM);
        if stages >= -1 {
            let mut i: i8 = -1;
            loop {
                let o = if i == -1 { comp } else { stage_at(i as i32) };
                if read_usize(o + 0x18) != 0 {
                    menu_item(&mut menu_state, 0x28f, top, 0x30c, top + 0x14, 0x99c518, read_i16(o + 0x3a) as i32, 0, i as i32, 0); // "Stage"
                }
                i = i.wrapping_add(1);
                if i as i32 > read_i32(comp + 0x30) {
                    break;
                }
            }
        }
        y = top + 0x19;
        write_u32(0xadae0c, 1);
        obj = if chosen == -1 { comp } else { stage_at(chosen as i32) };
        exe::comp_util_get_stage_name(JOB_TEXT as *mut u8, 0x7d0, read_u16(obj + 0x3a) as u32, 0);
        comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
        y += 0x28;
    }
    let rows = ((bottom - y) / exe::font_get_height(3)) as u8;
    let seasons: u8;
    let area = if read_u8(read_usize(obj + 0x18) + 0x12) == 0xff {
        seasons = 2;
        tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 7, weights7.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX])
    } else {
        seasons = 3;
        tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 8, weights8.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX])
    };
    let mut col: u32 = 2;
    let mut back = seasons as i32 - 1;
    while back >= 0 {
        let year = read_u16(read_usize(comp + 0x18) + 8) as i32 - back;
        comp_lang_format()(JOB_TEXT, 0x99c4e8, year, year + 1); // "%d/%d"
        tel26_area_text([area, col, 0, 0x30, grey, 0xc, 2, pale, JOB_TEXT as u32]);
        col += 1;
        back -= 1;
    }
    comp_lang_format()(JOB_TEXT, 0x99c4c4, 0xad9c64usize); // "Total"
    tel26_area_text([area, col, 0, 0x30, grey, 0xc, 2, pale, JOB_TEXT as u32]);
    col += 1;
    comp_lang_format()(JOB_TEXT, 0x99c4a0, 0xad9c64usize); // "Played"
    tel26_area_text([area, col, 0, 0x30, grey, 0xc, 2, pale, JOB_TEXT as u32]);
    comp_lang_format()(JOB_TEXT, 0x99c478, 0xad9c64usize); // "Average"
    tel26_area_text([area, col + 1, 0, 0x30, grey, 0xc, 2, pale, JOB_TEXT as u32]);
    let numbers = job_colour(COMP_TABS_COLOUR_3152);
    let mut row: u32 = 1;
    let mut k: i16 = 0;
    while k < read_i16(obj + 0x34) {
        let e = read_usize(obj + 0x18) + (k as i32 * 0x13) as usize;
        let id = read_i32(e);
        if id != -1 {
            comp_crt_format()(JOB_TEXT, 0x98806c, row); // "%d"
            tel26_area_text([area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, JOB_TEXT as u32]);
            let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE)));
            let human = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
            comp_crt_format()(JOB_TEXT, 0x987ff0, club.addr() + club::SHORT_NAME);
            tel26_area_button([area, 1, row, 1, 0, 1, 3, if human { yellow } else { pale }, JOB_TEXT as u32, 0x7d5, club.addr() as u32]);
            let points = |o: usize| read_i16(e + o) as i32;
            comp_crt_format()(JOB_TEXT, 0x98806c, points(0xa));
            tel26_area_text([area, 2, row, 1, 0, 0xc, 2, numbers, JOB_TEXT as u32]);
            comp_crt_format()(JOB_TEXT, 0x98806c, points(0xd));
            tel26_area_text([area, 3, row, 1, 0, 0xc, 2, numbers, JOB_TEXT as u32]);
            let mut c: u32 = 4;
            let total = if seasons == 3 {
                comp_crt_format()(JOB_TEXT, 0x98806c, points(0x10));
                tel26_area_text([area, 4, row, 1, 0, 0xc, 2, numbers, JOB_TEXT as u32]);
                c = 5;
                points(0xd) + points(0x10) + points(0xa)
            } else {
                points(0xd) + points(0xa)
            };
            comp_crt_format()(JOB_TEXT, 0x98806c, total);
            tel26_area_text([area, c, row, 1, 0, 0xc, 2, numbers, JOB_TEXT as u32]);
            c += 1;
            let games = |o: usize| read_i8(e + o) as i32;
            let played = if seasons == 3 { games(0xf) + games(0x12) + games(0xc) } else { games(0xf) + games(0xc) };
            comp_crt_format()(JOB_TEXT, 0x98806c, played);
            tel26_area_text([area, c, row, 1, 0, 0xc, 2, yellow, JOB_TEXT as u32]);
            comp_crt_format()(JOB_TEXT, 0x99c470, read_f32(e + 4) as f64); // "%5.3f"
            tel26_area_text([area, c + 1, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 2, yellow, JOB_TEXT as u32]);
            row += 1;
        }
        k = k.wrapping_add(1);
    }
}

/// Builds the Team Stats tab of the competition screen: the clubs ranked by one statistic, chosen in the View popup.
///
/// The statistic comes from parameter 0x23, the sorted table from 0x24 and its count from 0x25. The View button
/// (0x6e..0xeb) opens a popup with the 17 statistics (`comp_stats_get_team_stat_label`, items with id 0x16 and the
/// statistic as value; the chosen one ticked, and its label, without the leading space, is the heading). A four-column
/// list (weights 2 12 10 3) below the heading. With no table yet: `comp_stats_get_sorted_team_table` (at most 0x4b) into
/// parameters 0x24/0x25; none: no rows. Each entry (0x76 bytes; club −1 skipped): the position (`utils_format_ordinal`),
/// the club (a link, yellow for a human's club), in a national team competition (`comp + 0x42` = 2) the club's nation (a
/// link to its national team), and the value (`comp_stats_format_team_stat`). The scroll row comes back from parameter
/// 0x26 and the area goes to 0x27.
/// Original: `comp_screens_draw_team_stats_tab` (0x4956e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_team_stats_tab(comp: usize, top: i32, bottom: i32) {
    let weights: [u8; 4] = [2, 0xc, 0xa, 3];
    let stat = get_screen_param(0x23) as u8;
    let mut table = get_screen_param(0x24);
    let mut count = get_screen_param(0x25) as i32;
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    comp_lang_format()(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let popup = team_stats_tab_popup_area(button);
    write_u32(0xadae0c, 1);
    let chosen = stat as i8 as u32;
    let mut label = [0u8; 0xc8];
    // simplification: with no statistic ticked the original's heading is stack garbage; empty here
    let mut heading = [0u8; 0x100];
    for k in 1..=0x11u32 {
        exe::comp_stats_get_team_stat_label(read_usize(comp + 8), k, label.as_mut_ptr() as usize, 0xc8);
        comp_crt_format()(JOB_TEXT, 0x98ed14, label.as_ptr() as usize); // " %s"
        if k == chosen {
            team_stats_tab_menu_item(0x2002, 0x16, k, popup);
            copy_c_string(heading.as_mut_ptr() as usize, JOB_TEXT + 1);
        } else {
            team_stats_tab_menu_item(2, 0x16, k, popup);
        }
    }
    let y = top + 0x19;
    comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, heading.as_ptr() as u32, 1]);
    let y = y + 0x28;
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut row: i32 = 0;
    if table == 0 {
        covered!("tel22_3 team stats table built");
        // simplification: the original passes the statistic's dword slot (its other bytes stack residue); the callee reads the byte
        table = exe::comp_stats_get_sorted_team_table(read_usize(comp + 8), stat as u32, &mut count, 0x4b);
        set_screen_param(0x24, table);
        set_screen_param(0x25, count as usize);
        if table == 0 {
            comp_tabs_restore_scroll(area, 0x26, 0x27);
            return;
        }
    }
    let mut k: i32 = 0;
    let mut e = table;
    while k < count {
        let id = read_i32(e);
        if id != -1 {
            let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE)));
            let place = exe::utils_format_ordinal(row + 1, -1);
            tel26_area_text([area, 0, row as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, place as u32]);
            let human = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
            comp_crt_format()(JOB_TEXT, 0x987ff0, club.addr() + club::NAME); // "  %s"
            tel26_area_button([area, 1, row as u32, 1, 0, 9, 3, if human { yellow } else { pale }, JOB_TEXT as u32, 0x7d5, club.addr() as u32]);
            if read_u8(comp + 0x42) == 2 && club.nation().addr() != 0 {
                let team = exe::database_get_national_team(club.nation().addr());
                tel26_area_button([area, 2, row as u32, 1, 0, 1, 2, yellow, (club.nation().addr() + nation::SHORT_NAME) as u32, 0x7d5, team as u32]);
            }
            exe::comp_stats_format_team_stat(read_usize(comp + 8), e, stat as u32, JOB_TEXT, 0x7d0, 1);
            tel26_area_text([area, 3, row as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, pale, JOB_TEXT as u32]);
            row += 1;
        }
        k += 1;
        e += 0x76;
    }
    comp_tabs_restore_scroll(area, 0x26, 0x27);
}

/// Builds the Player Stats tab of the competition screen: the players ranked by one statistic (View popup), filtered
/// by position (Filter popup).
///
/// The statistic comes from parameter 0x1e, the filter from 0x36, the sorted table from 0x1f and its count from 0x20.
/// The View popup has the 17 statistics (`comp_stats_get_player_stat_label`, id 0x17; the chosen one ticked and its
/// label the heading); the Filter button (0x28f..0x30c) opens Goalkeepers (1), Defenders (4), Midfielders (0x10),
/// Attackers (0x40) and All Players (−1) (id 0x1b), the chosen one ticked and named in the heading ("... (Goalkeepers)").
/// With no table yet: `Comp_Stats::get_sorted_player_table` (at most 0x4b, with the filter) into parameters 0x1f/0x20;
/// none: no rows. Each entry (0x2d bytes; player −1 skipped): the position, the player's name (a link, id 0x7d7, yellow
/// when he plays for a human's club, or a human's national team in a national team competition, `comp + 0x42` = 4), his
/// club or nation (a link), and the value (`Comp_Stats::format_player_stat`). The scroll row comes back from parameter
/// 0x21 and the area goes to 0x22.
/// Original: `comp_screens_draw_player_stats_tab` (0x495bc0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_player_stats_tab(comp: usize, top: i32, bottom: i32) {
    let weights: [u8; 4] = [2, 0xc, 0xa, 3];
    let stat = get_screen_param(0x1e) as u8;
    let filter = get_screen_param(0x36) as u8;
    let mut table = get_screen_param(0x1f);
    let mut count = get_screen_param(0x20) as i32;
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    comp_lang_format()(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let popup = team_stats_tab_popup_area(button);
    write_u32(0xadae0c, 1);
    let chosen = stat as i8 as u32;
    let mut label = [0u8; 0xc8];
    // simplification: with no statistic ticked the original's heading is stack garbage; empty here
    let mut heading = [0u8; 0xc8];
    let mut line = [0u8; 0x800];
    let lb = line.as_mut_ptr() as usize;
    for k in 1..=0x11u32 {
        exe::comp_stats_get_player_stat_label(read_usize(comp + 8), k, label.as_mut_ptr() as usize, 0xc8);
        comp_crt_format()(JOB_TEXT, 0x98ed14, label.as_ptr() as usize); // " %s"
        if k == chosen {
            team_stats_tab_menu_item(0x2002, 0x17, k, popup);
            copy_c_string(heading.as_mut_ptr() as usize, JOB_TEXT + 1);
        } else {
            team_stats_tab_menu_item(2, 0x17, k, popup);
        }
    }
    comp_lang_format()(JOB_TEXT, 0x98ebf8); // "Filter"
    let button = tel26_add_button([0x28f, top as u32, 0x30c, (top + 0x14) as u32, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let popup = team_stats_tab_popup_area(button);
    copy_c_string(lb, heading.as_ptr() as usize);
    // the filters: value, the item text, the heading "<stat> (<position>)"
    for (value, text, titled) in [(1u8, 0x99c710usize, 0x99c6e0usize), (4, 0x99c694, 0x99c664), (0x10, 0x99c618, 0x99c5e8),
                                  (0x40, 0x99c59c, 0x99c56c), (0xff, 0x99c520, 0)] {
        comp_lang_format()(JOB_TEXT, text, 0xad9c64usize);
        let on = filter == value;
        team_stats_tab_menu_item(if on { 0x2002 } else { 2 }, 0x1b, value as i8 as u32, popup);
        if on && titled != 0 {
            comp_lang_format()(lb, titled, heading.as_ptr() as usize);
        }
    }
    let y = top + 0x19;
    comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, lb as u32, 1]);
    let y = y + 0x28;
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut row: i32 = 0;
    if table == 0 {
        covered!("tel22_3 player stats table built");
        // simplification: the statistic goes as its dword slot and the filter as a sign-extended word in the original
        // (stack residue and register garbage above them); the callee reads the low parts
        table = exe::comp_stats_get_sorted_player_table(read_usize(comp + 8), stat as u32, &mut count, 0x4b, filter as i8 as i16 as u16 as u32);
        set_screen_param(0x1f, table);
        set_screen_param(0x20, count as usize);
        if table == 0 {
            comp_tabs_restore_scroll(area, 0x21, 0x22);
            return;
        }
    }
    let mut k: i32 = 0;
    let mut p = table + 4;
    while k < count {
        let id = read_i32(p);
        if id != -1 {
            let staff = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
            let place = exe::utils_format_ordinal(row + 1, -1);
            tel26_area_text([area, 0, row as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, place as u32]);
            if read_u8(comp + 0x42) == 4 {
                covered!("tel22_3 player stats national");
                let nation = staff.nation().addr();
                let human = nation != 0 && exe::human_manager_is_human_club(exe::database_get_national_team(nation), 0, 1, 0, 0) != 0;
                exe::database_format_staff_name(staff, lb, 0x7d0, 1);
                comp_crt_format()(JOB_TEXT, 0x987ff0, lb); // "  %s"
                tel26_area_button([area, 1, row as u32, 1, 0, 9, 3, if human { yellow } else { pale }, JOB_TEXT as u32, 0x7d7, staff.addr() as u32]);
                let nation = staff.nation().addr();
                if nation != 0 {
                    let team = exe::database_get_national_team(nation);
                    tel26_area_button([area, 2, row as u32, 1, 0, 1, 2, yellow, (staff.nation().addr() + nation::SHORT_NAME) as u32, 0x7d5, team as u32]);
                }
            } else {
                let club = staff.club_job().addr();
                let human = club != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
                exe::database_format_staff_name(staff, lb, 0x7d0, 1);
                comp_crt_format()(JOB_TEXT, 0x987ff0, lb);
                tel26_area_button([area, 1, row as u32, 1, 0, 9, 3, if human { yellow } else { pale }, JOB_TEXT as u32, 0x7d7, staff.addr() as u32]);
                let club = staff.club_job().addr();
                if club != 0 {
                    tel26_area_button([area, 2, row as u32, 1, 0, 1, 2, yellow, (club + 0x38) as u32, 0x7d5, club as u32]);
                }
            }
            exe::comp_stats_format_player_stat(read_usize(comp + 8), p - 4, stat as u32, JOB_TEXT, 0x7d0, 1);
            tel26_area_text([area, 3, row as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, pale, JOB_TEXT as u32]);
            row += 1;
        }
        k += 1;
        p += 0x2d;
    }
    comp_tabs_restore_scroll(area, 0x21, 0x22);
}

/// Comparer for `comp_screens_draw_fixtures_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_fixtures_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_FIXTURES_TAB);
    // simplification: the first build allocates the match list (parameter 0x17) and the error box shows a message:
    // original only (proven in Rust mode)
    if comp == 0 || get_screen_param(0x17) == 0 {
        return o(comp, top, bottom);
    }
    compare_match_screen_build("competition screen, Fixtures tab", &[], || o(comp, top, bottom), || comp_screens_draw_fixtures_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_schedule_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_schedule_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_SCHEDULE_TAB);
    // simplification: building the date list (parameter 0x1a) allocates, and the error box: original only
    if comp == 0 || (read_u32(comp + 0x45) != 0 && get_screen_param(0x1a) == 0) {
        return o(comp, top, bottom);
    }
    compare_match_screen_build("competition screen, Schedule tab", &[], || o(comp, top, bottom), || comp_screens_draw_schedule_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_seedings_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_seedings_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_SEEDINGS_TAB);
    compare_match_screen_build("competition screen, Seedings tab", &[], || o(comp, top, bottom), || comp_screens_draw_seedings_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_average_points_tab`: side by side over the display list (the averages update runs
/// in both, recomputing the same values from the season points).
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_average_points_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_AVERAGE_POINTS_TAB);
    compare_match_screen_build("competition screen, Average Points tab", &[], || o(comp, top, bottom), || comp_screens_draw_average_points_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_team_stats_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_team_stats_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_TEAM_STATS_TAB);
    // simplification: sorting the table (parameter 0x24) allocates: original only
    if get_screen_param(0x24) == 0 {
        return o(comp, top, bottom);
    }
    compare_match_screen_build("competition screen, Team Stats tab", &[], || o(comp, top, bottom), || comp_screens_draw_team_stats_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_player_stats_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_player_stats_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_PLAYER_STATS_TAB);
    // simplification: sorting the table (parameter 0x1f) allocates: original only
    if get_screen_param(0x1f) == 0 {
        return o(comp, top, bottom);
    }
    compare_match_screen_build("competition screen, Player Stats tab", &[], || o(comp, top, bottom), || comp_screens_draw_player_stats_tab(comp, top, bottom));
}

/// Rule: the competition screen's input: a pending printout to file, the screen's own buttons, then the open tab's
/// clicks, and what is left to the menu bar.
///
/// After `comp_click_pending_printout` (−4 when the write failed), by the widget's id (`tel26_item_id`, the default
/// command id without a widget): 1 opens the competition screen of the widget's value (`comp_screens_open_comp_screen`
/// with −2, −2, 1; −4); 0x18 the history screen of the competition's English name (`history_screen_open`, −4); 0x19 the
/// Print button (`comp_click_print_button`); 2 the tab bar: a different tab (the widget's value) goes to param 1 (−11),
/// the same tab goes on; 3 the award screen of the widget's value (−4); 4 the competition history screen
/// (`comp_screens_open_history_screen` of the competition record, 0; −4). Then by the tab (param 1, its low byte): 1 the
/// Stages tab (`comp_screens_handle_stages_tab_click`), 2 `comp_click_results_tab`, 3 the Fixtures tab, 4 the Schedule
/// tab, 7 `comp_click_team_stats_tab`, 8 the Player Stats tab, 9 the Average Points tab, 10 the referee stats
/// (`official_stats_screen_handle_click`); their nonzero answer comes back, otherwise (and on tabs 5, 6 and the others)
/// the menu bar's answer.
/// Original: `comp_screens_handle_comp_screen_click` (0x4965d0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_comp_screen_click(widget: u32) -> i32 {
    let comp = get_screen_param(0);
    if let Some(r) = comp_click_pending_printout(comp) {
        return r;
    }
    let w = widget as i16;
    match tel26_item_id(w) {
        1 => {
            let open: unsafe extern "C" fn(usize, i32, i32, u32) = core::mem::transmute(FN_COMP_SCREENS_OPEN_COMP_SCREEN);
            open(tel26_item_value(w) as usize, -2, -2, 1);
            return -4;
        }
        0x18 => {
            let open: unsafe extern "C" fn(usize) = core::mem::transmute(FN_HISTORY_SCREEN_OPEN);
            open(comp_get_english_name(read_usize(comp + 4)));
            return -4;
        }
        0x19 => return comp_click_print_button(comp, w),
        _ => {}
    }
    if tel26_item_id(w) == 2 && tel26_item_value(w) as usize != get_screen_param(1) {
        covered!("tel22_4 tab picked");
        set_screen_param(1, tel26_item_value(w) as usize);
        return -11;
    }
    if tel26_item_id(w) == 3 {
        let open: unsafe extern "C" fn(usize) = core::mem::transmute(FN_AWARD_SCREENS_OPEN_AWARD_SCREEN);
        open(tel26_item_value(w) as usize);
        return -4;
    }
    if tel26_item_id(w) == 4 {
        let open: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_COMP_SCREENS_OPEN_HISTORY_SCREEN);
        open(read_usize(get_screen_param(0) + 4), 0);
        return -4;
    }
    let r = match get_screen_param(1) as u8 as i8 {
        1 => comp_click_call(0x497050, widget),
        2 => match comp_click_results_tab(widget) {
            Some(r) => return r,
            None => 0,
        },
        3 => comp_click_call(0x497930, widget),
        4 => comp_click_call(0x497dd0, widget),
        7 => match comp_click_team_stats_tab(widget) {
            Some(r) => return r,
            None => 0,
        },
        8 => comp_click_call(0x498300, widget),
        9 => comp_click_call(0x498520, widget),
        10 => comp_click_call(0x79aba0, widget),
        _ => 0,
    };
    if r != 0 {
        return r;
    }
    comp_click_menubar(widget)
}

/// Rule: the Stages tab's clicks and its live draw: a tie's fixtures are made one at a time, 1.5 s apart, while the
/// draw runs.
///
/// With a widget the list's scroll row is kept (param 6, area of param 7). Without one (the screen's tick) and a cup
/// stage being drawn (param 0x34, round param 3, tie param 0x32): with param 0x33 set, the tie's fixture state
/// (`cup_tie::FIXTURE_STATE`) 3 answers −11; without it, the next tie of the round (when there is one,
/// `cup_round::TIES`) in state 2 answers −11. Then, with the draw running (param 0x35) and more than 1500 ms since
/// `COMP_CLICK_DRAW_TIMER`, the timer is set and `Cup::create_tie_fixtures(round, −1, 1)` makes the next tie (then the
/// tail below). By the id: 5 picks a stage and round (no param 1, inline free; −11 when they change); 0xb sets the draw
/// running flag (param 0x35) to the widget's value (−11); 9 (one tie) and 10 (all) make the ties' fixtures of the stage
/// (param 2) and round (param 3) until it says no more (9 stops after the second); 6 a different view (param 4) is
/// written (−11); 7 opens the league progress screen of the stage (param 2) with the widget's value (params 0..4; −11);
/// 8 a fixture: played goes to the Results tab on its day (params 1 = 2, 8, 9; 0xb, 0xd cleared, list 0xe freed, 0xf
/// cleared), not played to the Fixtures tab (params 1 = 3, 0x11, 0x12; 0x14, 0x16 cleared, list 0x17 freed, 0x18
/// cleared) (−11). The tail of the draw: param 6 = −1, params 0x11..0x17 cleared (the fixtures list of param 0x17 is not
/// freed: a defect of the original), −11. Otherwise 0.
/// Original: `comp_screens_handle_stages_tab_click` (0x497050, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_stages_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    let tail = || {
        set_screen_param(6, usize::MAX);
        for k in 0x11..=0x17 {
            set_screen_param(k, 0);
        }
        -11
    };
    if w != -1 {
        comp_click_keep_scroll(6, 7);
    } else {
        let cup = get_screen_param(0x34);
        if cup != 0 {
            let round_param = get_screen_param(3);
            let round = round_param as i16 as isize;
            let tie = get_screen_param(0x32) as i16 as isize;
            let ties = || read_usize((read_usize(cup + cup::TIE_TABLES) as isize + round * 4) as usize);
            if get_screen_param(0x33) != 0 {
                if read_u8((ties() as isize + tie * 0x35 + cup_tie::FIXTURE_STATE as isize) as usize) == 3 {
                    return -11;
                }
            } else {
                let count = read_i16((read_usize(cup + cup::ROUND_TABLE) as isize + round * cup_round::SIZE as isize + cup_round::TIES as isize) as usize) as isize;
                if tie + 1 < count && read_u8((ties() as isize + (tie + 1) * 0x35 + cup_tie::FIXTURE_STATE as isize) as usize) == 2 {
                    return -11;
                }
            }
            if get_screen_param(0x35) != 0 && exe::utils_get_time_ms() > read_u32(COMP_CLICK_DRAW_TIMER).wrapping_add(1500) {
                covered!("tel22_4 live draw tie");
                write_u32(COMP_CLICK_DRAW_TIMER, exe::utils_get_time_ms());
                exe::cup_create_tie_fixtures(cup, round_param as u32, u32::MAX, 1);
                return tail();
            }
        }
    }
    if tel26_item_id(w) == 5 {
        let (stage, round) = comp_click_split_code(tel26_item_value(w) as i32);
        if comp_click_pick_round(stage, round, false, false) {
            return -11;
        }
    }
    if tel26_item_id(w) == 0xb {
        set_screen_param(0x35, tel26_item_value(w) as usize);
        return -11;
    }
    if tel26_item_id(w) == 9 || tel26_item_id(w) == 0xa {
        covered!("tel22_4 stage fixtures made");
        let comp = get_screen_param(0);
        let stage_index = get_screen_param(2);
        let round = get_screen_param(3) as u32;
        let stage = exe::comp_get_stage(comp, stage_index as i32);
        if exe::cup_create_tie_fixtures(stage, round, u32::MAX, 1) != 0 {
            while tel26_item_id(w) != 9 && exe::cup_create_tie_fixtures(stage, round, u32::MAX, 1) != 0 {}
        }
        return tail();
    }
    if tel26_item_id(w) == 6 {
        if tel26_item_value(w) as usize != get_screen_param(4) {
            set_screen_param(4, tel26_item_value(w) as usize);
            return -11;
        }
    }
    if tel26_item_id(w) == 7 {
        let comp = get_screen_param(0);
        let stage = exe::comp_get_stage(comp, get_screen_param(2) as i32);
        let value = tel26_item_value(w) as usize;
        if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49b990usize, 0x49bef0usize, 0, 0usize, 0) == 0 {
            return -11;
        }
        set_screen_param(0, stage);
        set_screen_param(1, value);
        set_screen_param(2, 0);
        set_screen_param(3, 0);
        set_screen_param(4, 0);
        return -11;
    }
    if tel26_item_id(w) == 8 {
        let fixture = Fixture(tel26_item_value(w) as usize);
        if (fixture.home_goals() as u8) != 0xff {
            covered!("tel22_4 stage fixture to results");
            set_screen_param(1, 2);
            set_screen_param(8, fixture.day() as usize);
            set_screen_param(9, fixture.year() as usize);
            set_screen_param(0xb, 0);
            set_screen_param(0xd, 0);
            comp_free_list_param(0xe);
            set_screen_param(0xf, 0);
        } else {
            covered!("tel22_4 stage fixture to fixtures");
            set_screen_param(1, 3);
            set_screen_param(0x11, fixture.day() as usize);
            set_screen_param(0x12, fixture.year() as usize);
            set_screen_param(0x14, 0);
            set_screen_param(0x16, 0);
            comp_free_list_param(0x17);
            set_screen_param(0x18, 0);
        }
        return -11;
    }
    0
}

/// Rule: the Fixtures tab's clicks: moving to the earlier or later fixture day, picking a stage and round, opening a
/// fixture's league progress.
///
/// With a widget the list's scroll row is kept (param 0x18, area of param 0x19). Id 0x10 (earlier): the shown day
/// (params 0x11 and 0x12) moves to the "later" pair (0x15 and 0x16), the "earlier" pair (0x13 and 0x14) becomes the
/// shown day and is cleared; id 0x11 (later) the mirror; both free the list of param 0x17 and clear its scroll row
/// (param 0x18): −11. Id 0x12 picks a stage and round (`comp_click_pick_round` with param 1 = 1, inline free): −11.
/// Id 0x13 opens the league progress of the widget's fixture (−4). Otherwise 0.
/// Original: `comp_screens_handle_fixtures_tab_click` (0x497930, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_fixtures_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(0x18, 0x19);
    }
    let id = tel26_item_id(w);
    if id == 0x10 || id == 0x11 {
        let (from, to) = if id == 0x10 { (0x13, 0x15) } else { (0x15, 0x13) };
        covered!("tel22_4 fixtures day moved");
        set_screen_param(to, get_screen_param(0x11));
        set_screen_param(to + 1, get_screen_param(0x12));
        set_screen_param(0x11, get_screen_param(from));
        set_screen_param(0x12, get_screen_param(from + 1));
        set_screen_param(from, 0);
        set_screen_param(from + 1, 0);
        comp_free_list_param(0x17);
        set_screen_param(0x18, 0);
        return -11;
    }
    if id == 0x12 {
        let (stage, round) = comp_click_split_code(tel26_item_value(w) as i32);
        comp_click_pick_round(stage, round, true, false);
        return -11;
    }
    if id == 0x13 {
        exe::comp_open_league_progress_for_fixture(tel26_item_value(w) as usize);
        return -4;
    }
    0
}

/// Rule: the Schedule tab's clicks: a schedule date goes to the Results tab when matches were played that day, else to
/// the Fixtures tab when matches are due; or a stage and round are picked.
///
/// With a widget the list's scroll row is kept (param 0x1c, area of param 0x1d). Id 0x14: the widget's schedule entry
/// (day +0xc, year +0xe) is looked up (`FIXTURE_MANAGER::scan_fixtures` of the competition, into a local list): played
/// matches (slice byte 2, the 4th argument 0, the 15th 1) found on exactly that date: Results tab (params 1 = 2, 8 and
/// 9 the day, 0xb and 0xd cleared, list 0xe freed, 0xf cleared; −11); else matches to play (slice 0, the 4th argument
/// 1, the 15th 0) on that date: Fixtures tab (params 1 = 3, 0x11 and 0x12, 0x14 and 0x16 cleared, list 0x17 freed,
/// 0x18 cleared; −11); else the next ids are tried. Id 0x15 picks a stage and round (`comp_click_pick_round` with param
/// 1 = 1, inline free): −11. Otherwise 0.
/// Original: `comp_screens_handle_schedule_tab_click` (0x497dd0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_schedule_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(0x1c, 0x1d);
    }
    if tel26_item_id(w) == 0x14 {
        type Scan = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32, u32, u32, u32) -> u16;
        let scan: Scan = core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
        let mut list = [0u32; 4];
        let lp = list.as_mut_ptr() as usize;
        exe::new_llist(lp);
        let comp = get_screen_param(0);
        let entry = tel26_item_value(w) as usize;
        let comp_id = read_u32(read_usize(comp + 4));
        let m = u32::MAX;
        for played in [true, false] {
            let mut day = read_u16(entry + 0xc) as u32;
            let mut year = read_u16(entry + 0xe) as u32;
            let mut slice: u8 = if played { 2 } else { 0 };
            let found = scan(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slice, (!played) as u32, comp_id, m, m, m, m, m, m, m, 0xffff_fffe,
                lp, played as u32, 0xffff_fffe, m, 1);
            if found != 0 && day & 0xffff == read_i16(entry + 0xc) as i32 as u32 && year & 0xffff == read_i16(entry + 0xe) as i32 as u32 {
                if played {
                    covered!("tel22_4 schedule to results");
                    set_screen_param(1, 2);
                    set_screen_param(8, (day & 0xffff) as usize);
                    set_screen_param(9, (year & 0xffff) as usize);
                    set_screen_param(0xb, 0);
                    set_screen_param(0xd, 0);
                    comp_free_list_param(0xe);
                    set_screen_param(0xf, 0);
                } else {
                    covered!("tel22_4 schedule to fixtures");
                    set_screen_param(1, 3);
                    set_screen_param(0x11, (day & 0xffff) as usize);
                    set_screen_param(0x12, (year & 0xffff) as usize);
                    set_screen_param(0x14, 0);
                    set_screen_param(0x16, 0);
                    comp_free_list_param(0x17);
                    set_screen_param(0x18, 0);
                }
                exe::drop_llist(lp);
                return -11;
            }
        }
        exe::drop_llist(lp);
    }
    if tel26_item_id(w) == 0x15 {
        let (stage, round) = comp_click_split_code(tel26_item_value(w) as i32);
        comp_click_pick_round(stage, round, true, false);
        return -11;
    }
    0
}

/// Rule: the Player Stats tab's clicks: picking the statistic or the position filter rebuilds the table.
///
/// With a widget the list's scroll row is kept (param 0x21, area of param 0x22). Id 0x17: a different statistic (the
/// widget's value) goes to param 0x1e; id 0x1b: a different filter goes to param 0x36; either frees the built table of
/// param 0x1f (crt free, params 0x1f and 0x20 cleared) and answers −11. An unchanged statistic tries the filter id; the
/// rest answer 0.
/// Original: `comp_screens_handle_player_stats_tab_click` (0x498300, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_player_stats_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(0x21, 0x22);
    }
    for (id, k) in [(0x17u16, 0x1eu32), (0x1b, 0x36)] {
        if tel26_item_id(w) == id && tel26_item_value(w) as usize != get_screen_param(k) {
            covered!("tel22_4 player stat picked");
            set_screen_param(k, tel26_item_value(w) as usize);
            comp_free_block_param(0x1f);
            return -11;
        }
    }
    0
}

/// Rule: the Average Points tab's clicks: id 5 picks the stage (the widget's value / 1000 − 1) into param 0x2a when it
/// changes (−11); otherwise 0.
/// Original: `comp_screens_handle_average_points_tab_click` (0x498520, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_average_points_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    if tel26_item_id(w) != 5 {
        return 0;
    }
    let stage = (tel26_item_value(w) as i32 / 1000 - 1) as usize;
    if stage == get_screen_param(0x2a) {
        return 0;
    }
    set_screen_param(0x2a, stage);
    -11
}

/// Comparer for `comp_screens_handle_comp_screen_click`: side by side for the tab bar and the Results and Team Stats
/// clicks that free nothing; the tabs with handlers of their own go through those (only the original here).
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_comp_screen_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_SCREENS_HANDLE_COMP_SCREEN_CLICK);
    let w = widget as i16;
    let pending = read_u32(COMP_CLICK_NETWORK_FLAG) != 0 && read_u32(G_PENDING_COMMAND_CODE) == COMP_CLICK_PRINTOUT_DIALOG;
    let id = tel26_item_id(w);
    let pure = !pending && match id {
        1 | 0x18 | 0x19 | 3 | 4 => false,
        2 if tel26_item_value(w) as usize != get_screen_param(1) => true,
        _ => match get_screen_param(1) as u8 {
            2 => match comp_click_action_id(widget) {
                0xc | 0xd => get_screen_param(0xe) == 0,
                0xe => comp_click_pick_is_pure(comp_click_action_data(widget)),
                _ => false,
            },
            7 => comp_click_action_id(widget) == 0x16 && comp_click_action_data(widget) as usize != get_screen_param(0x23) && get_screen_param(0x24) == 0,
            _ => false,
        },
    };
    comp_click_compare("handle_comp_screen_click", widget, pure, || o(widget), || comp_screens_handle_comp_screen_click(widget))
}

/// Comparer for `comp_screens_handle_stages_tab_click`: side by side unless the live draw may make a tie (the tick with
/// a stage being drawn), ids 9 and 10 (fixtures), 7 (a screen) or 8 with a list to free.
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_stages_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_SCREENS_HANDLE_STAGES_TAB_CLICK);
    let w = widget as i16;
    let pure = !(w == -1 && get_screen_param(0x34) != 0) && match tel26_item_id(w) {
        5 => comp_click_pick_is_pure(tel26_item_value(w) as i32),
        7 | 9 | 0xa => false,
        8 => get_screen_param(0xe) == 0 && get_screen_param(0x17) == 0,
        _ => true,
    };
    comp_click_compare("handle_stages_tab_click", widget, pure, || o(widget), || comp_screens_handle_stages_tab_click(widget))
}

/// Comparer for `comp_screens_handle_fixtures_tab_click`: side by side unless a list is freed or the league progress
/// opens.
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_fixtures_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_SCREENS_HANDLE_FIXTURES_TAB_CLICK);
    let w = widget as i16;
    let pure = match tel26_item_id(w) {
        0x10 | 0x11 => get_screen_param(0x17) == 0,
        0x12 => comp_click_pick_is_pure(tel26_item_value(w) as i32),
        0x13 => false,
        _ => true,
    };
    comp_click_compare("handle_fixtures_tab_click", widget, pure, || o(widget), || comp_screens_handle_fixtures_tab_click(widget))
}

/// Comparer for `comp_screens_handle_schedule_tab_click`: side by side unless a list may be freed (the scan's local
/// list is made and freed in each run).
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_schedule_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_SCREENS_HANDLE_SCHEDULE_TAB_CLICK);
    let w = widget as i16;
    let pure = match tel26_item_id(w) {
        0x14 => get_screen_param(0xe) == 0 && get_screen_param(0x17) == 0,
        0x15 => comp_click_pick_is_pure(tel26_item_value(w) as i32),
        _ => true,
    };
    comp_click_compare("handle_schedule_tab_click", widget, pure, || o(widget), || comp_screens_handle_schedule_tab_click(widget))
}

/// Comparer for `comp_screens_handle_player_stats_tab_click`: side by side unless the built table (param 0x1f) is
/// there to free.
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_player_stats_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_SCREENS_HANDLE_PLAYER_STATS_TAB_CLICK);
    let pure = get_screen_param(0x1f) == 0;
    comp_click_compare("handle_player_stats_tab_click", widget, pure, || o(widget), || comp_screens_handle_player_stats_tab_click(widget))
}

/// Comparer for `comp_screens_handle_average_points_tab_click`: side by side (it only writes param 0x2a).
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_average_points_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_SCREENS_HANDLE_AVERAGE_POINTS_TAB_CLICK);
    comp_click_compare("handle_average_points_tab_click", widget, true, || o(widget), || comp_screens_handle_average_points_tab_click(widget))
}

/// Opens the history screen of a competition (builder `comp_screens_draw_history_screen`, handler 0x49b250, leave
/// 0x49b7e0). `comp` is the competition record: param 0 is its object (`G_COMP_OBJECT_TABLE[comp id]`). The params:
/// 1 the tab (1 Winners), 2 and 3 the first and last row of the Winners table in the club history table (−1: not
/// searched yet), 6 the Records view 1, 0xb, 0xe, 0x11, 0x14 and 0x17 the views of the Attendances, Results,
/// Sequences, Teams and Players tabs (6, 9, 0xb, 3, 0xf), the rest of 0..0x19 zero. With `restore`, params 1, 6, 0xb,
/// 0xe, 0x11, 0x14 and 0x17 come back from the previous screen (its 60 params read before opening).
/// Original: `comp_screens_open_history_screen` (0x498780, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_open_history_screen(comp: usize, restore: u32) {
    covered!("tel22_5 open comp history");
    let mut prev = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in prev.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x498a30, 0x49b250, 0, 0x49b7e0, 0) == 0 {
        return;
    }
    let p = comp_history_planned_params(comp, 0, &prev);
    set_screen_param(0, object);
    for k in 1..0x1a {
        set_screen_param(k as u32, p[k]);
    }
    if restore != 0 {
        for k in [1u32, 6, 0xb, 0xe, 0x11, 0x14, 0x17] {
            set_screen_param(k, prev[k as usize]);
        }
    }
}

/// Builds the history screen of the competition object in param 0 (none: the comp_screens.cpp 5412 box): the section
/// picture 8, the title "<competition> History" in the competition's colours, the menu bar, the tab bar (Winners, when
/// the competition's history count, its virtual slot 0x50, is positive; Records, Attendances, Results, Sequences,
/// Teams, Players: kind 2 with the values 1..7) on param 1, then the open tab: 1 `comp_screens_draw_history_winners_tab`,
/// 2 the Records tab (`comp_history_records_tab`), 3..7 the tabs at 0x499900, 0x499e10, 0x49a320, 0x49a830 and
/// 0x49ad40 (called by address), each with the object and the tab bar's top and bottom; then the related
/// competitions menu (0x49d070, with the colours, 1, 0xa) and the Back/Next buttons. Writes 1 to [0x99c00c] and 0 to
/// [0xadae0c] first.
/// Original: `comp_screens_draw_history_screen` (0x498a30, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_screen() {
    // the 25 MenuItem slots of the tab bar (MenuItem::MenuItem: word +0 = −1, word +2 = 0; text at +4, kind +0xcc,
    // value +0xd0)
    let mut items = [[0u8; 0xd4]; 0x19];
    for it in items.iter_mut() {
        it[0] = 0xff;
        it[1] = 0xff;
    }
    write_u32(COMP_HISTORY_BUILT_FLAG, 1);
    write_u32(COMP_HISTORY_POPUP_FLAG, 0);
    let object = get_screen_param(0);
    if object == 0 {
        comp_screens_error_box(0x1524);
        return;
    }
    covered!("tel22_5 build comp history");
    let table = get_screen_param(1) as u8 as i8;
    exe::background_set_section_picture(8, 0);
    let comp = read_usize(object + 4);
    let (mut c28, mut c2c) = (0u32, 0u32);
    exe::database_get_comp_colours(comp, &mut c28, &mut c2c);
    let gender = read_u8(comp + 0x52) as i8 as i32;
    comp_lang_format()(JOB_TEXT, 0x99c764, gender, gender, comp + 0x38); // "{}<%s - competition>{} History"
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, c2c, c28, JOB_TEXT);
    exe::menubar_build(4, 0);
    let history_count: unsafe extern "thiscall" fn(usize) -> i8 = core::mem::transmute(read_usize(read_usize(object) + 0x50));
    let mut n = 0usize;
    let mut add = |items: &mut [[u8; 0xd4]; 0x19], fmt: usize, value: u32| {
        comp_lang_format()(items[n].as_mut_ptr().add(4) as usize, fmt);
        items[n][0xcc..0xd0].copy_from_slice(&2u32.to_le_bytes());
        items[n][0xd0..0xd4].copy_from_slice(&value.to_le_bytes());
        n += 1;
    };
    if history_count(object) > 0 {
        add(&mut items, 0x99b7d0, 1); // "Winners"
    }
    add(&mut items, 0x990ab4, 2); // "Records"
    add(&mut items, 0x990a9c, 3); // "Attendances"
    add(&mut items, 0x990a94, 4); // "Results"
    add(&mut items, 0x990a88, 5); // "Sequences"
    add(&mut items, 0x99c75c, 6); // "Teams"
    add(&mut items, 0x990a80, 7); // "Players"
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(items.as_mut_ptr() as usize, n as u32, table as i32, &mut top, &mut bottom, 1);
    match table {
        1 => comp_screens_draw_history_winners_tab(object, top, bottom),
        2 => comp_history_records_tab(object, top, bottom),
        3..=7 => {
            let f: unsafe extern "C" fn(usize, i32, i32) = core::mem::transmute(COMP_HISTORY_TEL23_TABS[table as usize - 3]);
            f(object, top, bottom);
        }
        _ => {}
    }
    exe::comp_screens_add_related_comps_menu(object, c2c, c28, 1, 0xa);
    tel26_back_next();
}

/// The Winners tab of the history screen: the competition's rows of the club history table (param 2 the first, 3 the
/// last; −1 at first: the table is sorted, `crt_qsort` by 0x540780, and searched for rows of this competition, +4 its
/// record, then both written). The columns: the year, the winner, the runner-up, a third column for competitions of
/// history kind 3 (the virtual slot 0x50; "Third Place", or "Minor Premiers" for the competition [0x9cf89c]) and a
/// Hosts column when the competition has hosts (slot 0x88): kind 2 with hosts 4 columns (weights 3,8,8,8, hosts in 3),
/// kind 2 without 3 (1,4,4), otherwise with hosts 5 (2,4,4,4,4, hosts in 4) or 4 (3,8,8,8); the Olympics ([0x9cf7a0])
/// head Gold, Silver, Bronze. Each club: "  name", yellow when it is a human's, a button (0x7d5, the club) when its
/// +0x53 record has +0x71, else text; the hosts in colour 0xaebdd4. The scroll row from param 4; the area to param 5.
/// Original: `comp_screens_draw_history_winners_tab` (0x4991f0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_winners_tab(object: usize, top: i32, bottom: i32) {
    let weights_3: [u8; 3] = [1, 4, 4];
    let weights_4: [u8; 4] = [3, 8, 8, 8];
    let weights_5: [u8; 5] = [2, 4, 4, 4, 4];
    let mut first = get_screen_param(2) as i32;
    let mut last = get_screen_param(3) as i32;
    if first == -1 {
        covered!("tel22_5 winners rows searched");
        exe::crt_qsort(read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT), 0x1a, 0x540780usize);
        let comp = read_u32(object + 4);
        let count = read_i32(G_CLUB_COMP_HISTORY_COUNT);
        for i in 0..count {
            if read_u32(read_usize(G_CLUB_COMP_HISTORY_TABLE) + i as usize * 0x1a + 4) == comp {
                if first == -1 {
                    first = i;
                }
                last = i;
            }
        }
        set_screen_param(2, first as isize as usize);
        set_screen_param(3, last as isize as usize);
    }
    let winner_rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
    let vt = read_usize(object);
    let kind_of: unsafe extern "thiscall" fn(usize) -> i8 = core::mem::transmute(read_usize(vt + 0x50));
    let has_hosts: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(vt + 0x88));
    let kind = kind_of(object);
    let (columns, weights, style, hosts): (u8, usize, u32, i8) = if kind == 2 {
        if has_hosts(object) != 0 { (4, weights_4.as_ptr() as usize, 3, 3) } else { (3, weights_3.as_ptr() as usize, 3, -1) }
    } else if has_hosts(object) != 0 {
        (5, weights_5.as_ptr() as usize, 2, 4)
    } else {
        (4, weights_4.as_ptr() as usize, 3, -1)
    };
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, columns as u32, weights as u32,
                                                        winner_rows as u32, 0, 2, 0, u32::MAX]);
    let heading = |col: u32| tel26_area_text([area, col, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
    let comp_id = read_u32(read_usize(object + 4));
    if comp_id == read_u32(COMP_HISTORY_OLYMPICS_ID) {
        comp_lang_format()(JOB_TEXT, 0x99c80c, 0xad9c64); // "Gold"
        heading(1);
        comp_lang_format()(JOB_TEXT, 0x99c7dc, 0xad9c64); // "Silver"
        heading(2);
        comp_lang_format()(JOB_TEXT, 0x99c7ac, 0xad9c64); // "Bronze"
        heading(3);
    } else {
        comp_lang_format()(JOB_TEXT, 0x99b7d0); // "Winners"
        heading(1);
        comp_lang_format()(JOB_TEXT, 0x99c7a0); // "Runners-up"
        heading(2);
        if kind == 3 {
            let third = if comp_id == read_u32(COMP_HISTORY_MINOR_PREMIERS_ID) { 0x99c790 } else { 0x99c784 };
            comp_lang_format()(JOB_TEXT, third); // "Minor Premiers" / "Third Place"
            heading(3);
        }
    }
    if hosts != -1 {
        comp_lang_format()(JOB_TEXT, 0x990fc0); // "Hosts"
        heading(hosts as i32 as u32);
    }
    // one club cell: "  name", a button when the club's +0x53 record has +0x71 (id 0x7d5, the club), else text
    let club_cell = |club: usize, col: u32, row: u32, colour: u32| {
        comp_crt_format()(JOB_TEXT, 0x987ff0, club + 0x38); // "  %s"
        let rec = read_usize(club + 0x53);
        if rec != 0 && read_u32(rec + 0x71) != 0 {
            tel26_area_button([area, col, row, 1, 0, 1, style, colour, JOB_TEXT as u32, 0x7d5, club as u32]);
        } else {
            tel26_area_text([area, col, row, 1, 0, 1, style, colour, JOB_TEXT as u32]);
        }
    };
    let human_colour = |club: usize| {
        if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(G_GUI_COLOUR_PALE_GREY) }
    };
    if first > -1 && first <= last {
        let mut row = 1u32;
        for i in first..=last {
            let entry = read_usize(G_CLUB_COMP_HISTORY_TABLE) + i as usize * 0x1a;
            comp_crt_format()(JOB_TEXT, 0x98806c, read_i16(entry + 8) as i32); // "%d"
            tel26_area_text([area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
            for (off, col) in [(0xa, 1u32), (0xe, 2)] {
                let club = read_usize(entry + off);
                if club != 0 {
                    club_cell(club, col, row, human_colour(club));
                }
            }
            if kind >= 3 {
                let club = read_usize(entry + 0x12);
                if club != 0 {
                    club_cell(club, 3, row, human_colour(club));
                }
            }
            if hosts != -1 {
                let club = read_usize(entry + 0x16);
                if club != 0 {
                    club_cell(club, hosts as i32 as u32, row, job_colour(JOB_COLOUR_BDD4));
                }
            }
            row += 1;
        }
    }
    let scroll = get_screen_param(4);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row(current_human_display().wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(0xbf1)), scroll);
    }
    set_screen_param(5, area as i16 as isize as usize);
}

/// Comparer for `comp_screens_open_history_screen`: opening only through the original (it opens the screen), the
/// params 0..0x19 checked against the plan (`comp_history_planned_params`).
pub(crate) unsafe extern "C" fn compare_comp_screens_open_history_screen(comp: usize, restore: u32) {
    let mut prev = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in prev.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let plan = comp_history_planned_params(comp, restore, &prev);
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_COMP_SCREENS_OPEN_HISTORY_SCREEN)(comp, restore);
    if comp != 0 && get_screen_param(0) == plan[0] && plan[0] != 0 {
        let mut got = [0usize; 0x1a];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("comp history screen (open {:#x}): params {:x?} x plan {:x?}", comp, got, plan));
    }
}

/// Comparer for `comp_screens_draw_history_screen`: side by side over the display list (`compare_match_screen_build`)
/// plus the two flags, for the Records tab and for the Winners tab once its rows are known (param 2 ≠ −1).
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_COMP_SCREENS_DRAW_HISTORY_SCREEN);
    let table = get_screen_param(1) as u8;
    // simplification: without the object (the error box), the first Winners build (it sorts the global club history
    // table and writes params 2 and 3) and the tel23 tabs (their lists are built and kept in the params on the first
    // build; they have their own comparers) only the original runs
    if get_screen_param(0) == 0 || !(table == 2 || (table == 1 && get_screen_param(2) as i32 != -1)) {
        return o();
    }
    compare_match_screen_build("comp history screen", &[(COMP_HISTORY_BUILT_FLAG, 4), (COMP_HISTORY_POPUP_FLAG, 4)], || o(),
        || comp_screens_draw_history_screen());
}

/// Comparer for `comp_screens_draw_history_winners_tab`: side by side over the display list once the rows are known.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_winners_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_HISTORY_WINNERS_TAB);
    // simplification: the first build sorts the global club history table (qsort, not stable: twice could reorder
    // equal rows) and writes params 2 and 3: only the original
    if object == 0 || get_screen_param(2) as i32 == -1 {
        return o(object, top, bottom);
    }
    compare_match_screen_build("comp history winners tab", &[], || o(object, top, bottom), || comp_screens_draw_history_winners_tab(object, top, bottom));
}

/// Rule: the history screen's Attendances tab (tab 3): the attendance records season by season (`history_list_tab`
/// with view param 0xb, popup id 5, scroll param 0xc, area param 0xd).
/// Original: `comp_screens_draw_history_attendances_tab` (0x499900, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_attendances_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 3)
}

/// Rule: the history screen's Results tab (tab 4): the result records season by season (`history_list_tab` with view
/// param 0xe, popup id 6, scroll param 0xf, area param 0x10).
/// Original: `comp_screens_draw_history_results_tab` (0x499e10, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_results_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 4)
}

/// Rule: the history screen's Sequences tab (tab 5): the sequence records season by season (`history_list_tab` with
/// view param 0x11, popup id 7, scroll param 0x12, area param 0x13).
/// Original: `comp_screens_draw_history_sequences_tab` (0x49a320, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_sequences_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 5)
}

/// Rule: the history screen's Teams tab (tab 6): the team records season by season (`history_list_tab` with view
/// param 0x14, popup id 8, scroll param 0x15, area param 0x16).
/// Original: `comp_screens_draw_history_teams_tab` (0x49a830, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_teams_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 6)
}

/// Rule: the history screen's Players tab (tab 7): the player records season by season (`history_list_tab` with view
/// param 0x17, popup id 9, scroll param 0x18, area param 0x19).
/// Original: `comp_history_draw_players_tab` (0x49ad40, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_draw_players_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 7)
}

/// Rule: the history screen's clicks.
///
/// The object is param 0 and the tab param 1. With a widget, the open tab's list keeps its scroll row (Winners: param
/// 4 from the area in 5; Records 7/8; the list tabs their scroll and area params). Records: id 3 (a View period other
/// than param 6) writes param 6 and answers −11; id 4 (a record title) opens the tab of its type
/// (`comp_history_tab_for_record`: param 1 = the tab, its view param = the type, the param after it = 0; −11), unless
/// the type has no tab. The list tabs: their popup id (tab + 2) with a type other than the view writes it (−11). The
/// Players tab: `comp_history_players_tab_handle_click` first (nonzero answers). Then, for every tab: id 1 opens the
/// history screen of the widget's competition (restoring the views; −4), id 0xa the written history
/// (`history_screen_open` of the competition's English name; −4), id 2 a tab other than param 1 (−11); otherwise the
/// menu bar (`menubar_handle_command`, then `menubar_handle_object_link`).
/// Defect of the original: a record title whose type is outside 1..0x16 shows the error box and opens tab 3 with the
/// view written to the param numbered by the widget (the out slot is the widget's own argument).
/// Original: `comp_history_screen_handle_event` (0x49b250, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_screen_handle_event(widget: u32) -> i32 {
    let object = League(get_screen_param(0));
    let table = get_screen_param(1) as u8 as i8;
    let w = widget as i16;
    match table {
        1 => {
            if w != -1 {
                comp_click_keep_scroll(4, 5);
            }
        }
        2 => {
            if w != -1 {
                comp_click_keep_scroll(7, 8);
            }
            if comp_click_action_id(widget) == 3 && exe::display_get_widget_data(current_human_display(), widget) != get_screen_param(6) {
                covered!("tel23_1 history records period");
                set_screen_param(6, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
                return -11;
            }
            if comp_click_action_id(widget) == 4 {
                let kind = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) as u8;
                let mut out = widget;
                let picked = comp_history_tab_for_record(kind as u32, &mut out) as i8;
                if picked > -1 {
                    covered!("tel23_1 history record title");
                    set_screen_param(1, picked as isize as usize);
                    set_screen_param(out, kind as i8 as isize as usize);
                    set_screen_param(out.wrapping_add(1), 0);
                    return -11;
                }
            }
        }
        3..=6 => {
            let view = history_view_param(table);
            if w != -1 {
                comp_click_keep_scroll(view + 1, view + 2);
            }
            if comp_click_action_id(widget) == table as i32 + 2 && exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) != get_screen_param(view) {
                covered!("tel23_1 history list view");
                set_screen_param(view, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
                return -11;
            }
        }
        7 => {
            let r = comp_history_players_tab_handle_click(widget);
            if r != 0 {
                return r;
            }
        }
        _ => {}
    }
    match comp_click_action_id(widget) {
        1 => {
            let comp = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget);
            exe::comp_screens_open_history_screen(comp, 1);
            -4
        }
        0xa => {
            history_screen_open_by_name(comp_get_english_name(object.competition().addr()));
            -4
        }
        2 if exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) != get_screen_param(1) => {
            set_screen_param(1, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
            -11
        }
        _ => comp_click_menubar(widget),
    }
}

/// Rule: the Players tab's clicks: with a widget the list keeps its scroll row (param 0x18 from the area in 0x19); id
/// 9 (the View popup) with a type other than param 0x17 writes it and answers −11; otherwise 0.
/// Original: `comp_history_players_tab_handle_click` (0x49b6e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_players_tab_handle_click(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(0x18, 0x19);
    }
    if tel26_item_id(w) != 9 || tel26_item_value(w) as usize == get_screen_param(0x17) {
        return 0;
    }
    covered!("tel23_1 history players view");
    set_screen_param(0x17, tel26_item_value(w) as usize);
    -11
}

/// Rule: leaving the history screen frees the season records list (param 9, crt free) and clears params 9 and 0xa.
/// Original: `comp_history_screen_close` (0x49b7e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_screen_close() {
    comp_free_block_param(HISTORY_RECORDS_LIST);
}

/// Rule: the history tab of a Comp_Stats record type and the param that holds that tab's view.
///
/// Type 1 has none (−1, nothing written); 2..5 the Teams tab (6, param 0x14); 6..8 Attendances (3, param 0xb); 9 and 10
/// Results (4, param 0xe); 11..14 Sequences (5, param 0x11); 15..22 Players (7, param 0x17). The param is written to
/// `out` when it is given. Any other type shows the comp_screens.cpp error box (line 0x232b) and answers 3 without
/// writing.
/// Original: `comp_history_tab_for_record` (0x49eb80, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_tab_for_record(kind: u32, out: *mut u32) -> u8 {
    let (tab, param) = match kind as u8 as i8 {
        1 => return 0xff,
        2..=5 => (6u8, 0x14u32),
        6..=8 => (3, 0xb),
        9 | 10 => (4, 0xe),
        11..=14 => (5, 0x11),
        15..=22 => (7, 0x17),
        _ => {
            comp_screens_error_box(0x232b);
            return 3;
        }
    };
    if !out.is_null() {
        *out = param;
    }
    tab
}

/// Comparer for `comp_screens_draw_history_attendances_tab`.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_attendances_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_HISTORY_ATTENDANCES_TAB);
    compare_history_list_tab("comp history attendances tab", object, || o(object, top, bottom), || comp_screens_draw_history_attendances_tab(object, top, bottom));
}

/// Comparer for `comp_screens_draw_history_results_tab`.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_results_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_HISTORY_RESULTS_TAB);
    compare_history_list_tab("comp history results tab", object, || o(object, top, bottom), || comp_screens_draw_history_results_tab(object, top, bottom));
}

/// Comparer for `comp_screens_draw_history_sequences_tab`.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_sequences_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_HISTORY_SEQUENCES_TAB);
    compare_history_list_tab("comp history sequences tab", object, || o(object, top, bottom), || comp_screens_draw_history_sequences_tab(object, top, bottom));
}

/// Comparer for `comp_screens_draw_history_teams_tab`.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_teams_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_SCREENS_DRAW_HISTORY_TEAMS_TAB);
    compare_history_list_tab("comp history teams tab", object, || o(object, top, bottom), || comp_screens_draw_history_teams_tab(object, top, bottom));
}

/// Comparer for `comp_history_draw_players_tab`.
pub(crate) unsafe extern "C" fn compare_comp_history_draw_players_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_COMP_HISTORY_DRAW_PLAYERS_TAB);
    compare_history_list_tab("comp history players tab", object, || o(object, top, bottom), || comp_history_draw_players_tab(object, top, bottom));
}

/// Comparer for `comp_history_screen_handle_event`: side by side over the screen manager and the params
/// (`comp_click_compare`) when the click only writes params; opening a screen and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_comp_history_screen_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_HISTORY_SCREEN_HANDLE_EVENT);
    let pure = history_click_is_pure(widget);
    comp_click_compare("history handle_event", widget, pure, || o(widget), || comp_history_screen_handle_event(widget))
}

/// Comparer for `comp_history_players_tab_handle_click`: side by side (it only writes params).
pub(crate) unsafe extern "C" fn compare_comp_history_players_tab_handle_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_HISTORY_PLAYERS_TAB_HANDLE_CLICK);
    comp_click_compare("history players_tab_handle_click", widget, true, || o(widget), || comp_history_players_tab_handle_click(widget))
}

/// Comparer for `comp_history_screen_close`: only the original (it frees the list), the params checked afterwards
/// (9 and 0xa cleared when there was a list, else untouched).
pub(crate) unsafe extern "C" fn compare_comp_history_screen_close() {
    let before = (get_screen_param(HISTORY_RECORDS_LIST), get_screen_param(HISTORY_RECORDS_COUNT));
    original_fn::<unsafe extern "C" fn()>(IDX_COMP_HISTORY_SCREEN_CLOSE)();
    let plan = if before.0 != 0 { (0, 0) } else { before };
    let got = (get_screen_param(HISTORY_RECORDS_LIST), get_screen_param(HISTORY_RECORDS_COUNT));
    SCOREBOARD.record(got == plan, || format!("comp history screen (close): params 9/0xa {:x?} x plan {:x?}", got, plan));
}

/// Comparer for `comp_history_tab_for_record`: both side by side by value (the answer and the param written), a type
/// outside 1..0x16 (the error box) only the original.
pub(crate) unsafe extern "C" fn compare_comp_history_tab_for_record(kind: u32, out: *mut u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut u32) -> u8>(IDX_COMP_HISTORY_TAB_FOR_RECORD);
    if !(1..=HISTORY_LAST_RECORD_TYPE as i8).contains(&(kind as u8 as i8)) {
        return o(kind, out);
    }
    let start = if out.is_null() { 0 } else { *out };
    let (mut out_o, mut out_r) = (start, start);
    let ro = o(kind, if out.is_null() { core::ptr::null_mut() } else { &mut out_o });
    let rr = comp_history_tab_for_record(kind, if out.is_null() { core::ptr::null_mut() } else { &mut out_r });
    SCOREBOARD.record(ro == rr && out_o == out_r, || format!("comp_history_tab_for_record({}): {} {:#x} x {} {:#x}", kind as u8, ro, out_o, rr, out_r));
    if !out.is_null() {
        *out = out_o;
    }
    ro
}

/// Rule: the league's two clubs of a fixture: the league (or league stage) it is played in and the two clubs' table
/// rows.
///
/// Any missing argument shows the comp_screens.cpp error box (line 0x2342) and answers 0. The three outputs start at
/// 0. The competition is the object of the competition table at the fixture's id (none, or an id below 0: 0); a reserve
/// or minor competition (`comp_is_reserve_or_minor_comp` of the fixture's competition) answers 0. The league is the
/// fixture's stage (its stage byte ≥ 0, from the stages table) or the competition itself; it must be a league (type 2).
/// Its table rows (`League::get_table_row`) are searched for the home and the away club. Answers 1 when both were found.
/// Original: `comp_fixture_resolve_league_clubs` (0x49ecd0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_fixture_resolve_league_clubs(fixture: usize, league_out: *mut usize, home_out: *mut usize, away_out: *mut usize) -> u32 {
    let fixture = Fixture(fixture);
    if fixture.is_null() || league_out.is_null() || home_out.is_null() || away_out.is_null() {
        comp_screens_error_box(0x2342);
        return 0;
    }
    *league_out = 0;
    *home_out = 0;
    *away_out = 0;
    let id = fixture.comp_id();
    if id <= -1 {
        return 0;
    }
    let object = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4));
    if object.is_null() || exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), fixture.competition().addr()) != 0 {
        return 0;
    }
    let stage = fixture.stage_index() as i8;
    *league_out = if stage > -1 { read_usize(object.stages() + stage as usize * 4) } else { object.addr() };
    let league = League(*league_out);
    if league.is_null() || league.comp_type() != 2 {
        return 0;
    }
    let mut k: i8 = 0;
    while (k as i16) < league.club_count() {
        let row = exe::league_get_table_row(league, k as i32);
        if read_usize(row + league_table_row::CLUB) == fixture.home_club().addr() {
            *home_out = row;
        }
        if read_usize(row + league_table_row::CLUB) == fixture.away_club().addr() {
            *away_out = row;
        }
        k = k.wrapping_add(1);
    }
    if *home_out == 0 || *away_out == 0 {
        return 0;
    }
    covered!("tel23_2 fixture league clubs");
    1
}

/// Rule: opening the league progress screen of a fixture's league with its two clubs highlighted.
///
/// No fixture: the comp_screens.cpp box (line 0x1bb1). A pointer below 10000 in absolute value does nothing. The league
/// and rows come from `comp_fixture_resolve_league_clubs`; when it fails with neither row found, the box of line
/// 0x1bbd. The screen opens with `open_screen` (builder `comp_league_progress_screen_draw`, input
/// `comp_league_progress_screen_handle_event`, no flag, no leave, no exit); then param 0 the league, 1 the home row,
/// 2 the away row, 3 and 4 zero.
/// Original: `comp_open_league_progress_for_fixture` (0x49b830, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_open_league_progress_for_fixture(fixture: usize) {
    let (mut league, mut home, mut away) = (0usize, 0usize, 0usize);
    if fixture == 0 {
        comp_screens_error_box(0x1bb1);
        return;
    }
    if (fixture as i32).wrapping_abs() < 10000 {
        return;
    }
    if comp_fixture_resolve_league_clubs(fixture, &mut league, &mut home, &mut away) == 0 && home == 0 && away == 0 {
        comp_screens_error_box(0x1bbd);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49b990usize, 0x49bef0usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel23_2 open league progress");
    for (k, v) in league_progress_planned_params(league, home, away).into_iter().enumerate() {
        set_screen_param(k as u32, v);
    }
}

/// Rule: building the league progress screen: the league's position graph of the two clubs and its table.
///
/// The league is param 0 (none: the comp_screens.cpp box, line 0x1bea), the home row param 1 and the away row param 2.
/// The competition's picture and colours, the title (the competition's name), the subtitle "<stage name> Progress"
/// (`comp_util_get_stage_name` of the league's stage id) or "League Progress" (no stage id), the menu bar (4). An empty
/// area for the graph (x 0x6e..0x267, y 0x91..0x217). The table column goes back to 0 (+0xad) and the table is sorted
/// (`League::sort_table`). A Plot (operator new 0x1c: 1, the club count, the games per club, 1, the number of rows
/// given) gets the home row's history in yellow (series 0) and the away row's in red (series 1 when there is a home
/// row, else 0), drawn as a line graph (x 0x73..0x25d, y 0x96..0x212, "Played" and "Position"), then destroyed. The
/// table: a 2-column area (weights 3, 10; x 0x26c..0x30c, y 0x91..0x217) with as many rows as 0x17c pixels hold in font
/// 2; each row its position (`utils_format_ordinal`, dark blue, a button id 1 with the row) and "  <club>" (yellow
/// for the home row, red for the away row, else pale grey; a button id 1 with the row); empty pale grey cells to the
/// last row. Back/Next; the scroll row from param 3 and the area to param 4.
/// Kept from the original: the row count goes as its byte (the original's dword carries the freed Plot pointer's high
/// bytes; Display keeps the byte); the colour words go in dword slots whose high half is register garbage (0 here).
/// Original: `comp_league_progress_screen_draw` (0x49b990, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_league_progress_screen_draw() {
    let weights: [u8; 2] = [3, 0xa];
    let league = League(get_screen_param(0));
    if league.is_null() {
        comp_screens_error_box(0x1bea);
        return;
    }
    let home = get_screen_param(1);
    let away = get_screen_param(2);
    let comp = league.competition().addr();
    exe::background_set_comp_picture(comp, 0, 0);
    comp_colours_title(comp_colours(comp), comp + 4);
    let stage_id = league.stage_id() as u16;
    let mut label = [0u8; 0x7d0];
    if stage_id != 0 {
        exe::comp_util_get_stage_name(label.as_mut_ptr(), 0x7d0, stage_id as u32, 0);
        comp_lang_format()(JOB_TEXT, 0x99c874, label.as_ptr()); // "<%s - table name> Progress"
    } else {
        comp_lang_format()(JOB_TEXT, 0x99c864); // "League Progress"
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    tel26_add_area(current_human_display(), [0x6e, 0x91, 0x267, 0x217, 1, 0, 1, 0, 2, 0, u32::MAX]);
    league.set_table_column((0) as usize);
    exe::league_sort_table(league, 0usize);
    let p = exe::crt_operator_new(0x1c);
    let plot = if p != 0 {
        let series = (away != 0) as u32 + (home != 0) as u32;
        plot_new(p, 1, league.club_count() as i32, league.games_per_club() as i32, 1, series)
    } else {
        0
    };
    if home != 0 {
        league_progress_series(plot, home, 0, job_colour(G_GUI_COLOUR_YELLOW));
    }
    if away != 0 {
        league_progress_series(plot, away, (home != 0) as u32, job_colour(G_GUI_COLOUR_RED));
    }
    let mut position_label = [0u8; 0x7d0];
    comp_lang_format()(label.as_mut_ptr() as usize, 0x99c4a0, 0xad9c64usize); // "Played<%s - COMMENT - games played>"
    comp_lang_format()(position_label.as_mut_ptr() as usize, 0x99c838, 0xad9c64usize); // "Position<%s - COMMENT - league position>"
    exe::plot_draw_line_graph(plot, 0x73, 0x96, 0x25d, 0x212, label.as_ptr() as usize, position_label.as_ptr() as usize);
    if plot != 0 {
        exe::drop_plot(plot);
        exe::crt_operator_delete(plot);
    }
    let rows = (0x17c / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x26c, 0x91, 0x30c, 0x217, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let mut r: i32 = 0;
    while r < league.club_count() as i32 {
        let row = exe::league_get_table_row(league, r);
        let ordinal = exe::utils_format_ordinal(read_u8(row + league_table_row::POSITION) as i8 as i32 + 1, 3);
        tel26_area_button([area, 0, r as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), ordinal as u32, 1, row as u32]);
        comp_crt_format()(JOB_TEXT, 0x987ff0, read_usize(row + league_table_row::CLUB) + club::SHORT_NAME); // "  %s"
        let colour = if row == home {
            job_colour(G_GUI_COLOUR_YELLOW)
        } else if row == away {
            job_colour(G_GUI_COLOUR_RED)
        } else {
            job_colour(G_GUI_COLOUR_PALE_GREY)
        };
        tel26_area_button([area, 1, r as u32, 2, 0, 1, 2, colour, JOB_TEXT as u32, 1, row as u32]); // tel23_2 the club cell's style
        r += 1;
    }
    while r < rows as i32 {
        tel26_area_text([area, 0, r as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_PALE_GREY), 0]);
        tel26_area_text([area, 1, r as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_PALE_GREY), 0]);
        r += 1;
    }
    tel26_back_next();
    comp_restore_area_scroll(area, 3);
    set_screen_param(4, area as i16 as isize as usize);
}

/// Rule: the league progress screen's clicks: with a widget the table keeps its scroll row (param 3 from the area in
/// 4). Id 1 (a table row, the row as value): the home row (param 1) or the away row (param 2) clicked again is cleared;
/// another row fills param 1 when it is empty, else param 2; −11. Other ids go to the menu bar.
/// Original: `comp_league_progress_screen_handle_event` (0x49bef0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_league_progress_screen_handle_event(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(3, 4);
    }
    if tel26_item_id(w) != 1 {
        return comp_click_menubar(widget);
    }
    let row = tel26_item_value(w) as usize;
    covered!("tel23_2 league progress row");
    if row == get_screen_param(1) {
        set_screen_param(1, 0);
    } else if row == get_screen_param(2) {
        set_screen_param(2, 0);
    } else if get_screen_param(1) == 0 {
        set_screen_param(1, row);
    } else {
        set_screen_param(2, row);
    }
    -11
}

/// Rule: opening the FIFA world rankings screen (builder `comp_fifa_rankings_screen_draw`, input
/// `comp_fifa_rankings_screen_handle_event`, no flag, leave `comp_fifa_rankings_screen_close`): params 0 and 1 (the
/// ranked list and its count) 0, 2 (the page) and 3 (the pages) 1.
/// Original: `comp_open_fifa_rankings_screen` (0x49c0b0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_open_fifa_rankings_screen() {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49c120usize, 0x49c510usize, 0, 0x49c6c0usize, 0) == 0 {
        return;
    }
    covered!("tel23_2 open fifa rankings");
    for (k, v) in FIFA_RANKINGS_OPEN_PARAMS.into_iter().enumerate() {
        set_screen_param(k as u32, v);
    }
}

/// Rule: building the FIFA world rankings screen: one page of the ranked nations.
///
/// Params: 0 the ranked list (built the first time: `fifa_rankings_collect_ranked_order` of the FIFA rankings object,
/// [G_NATIONAL_TEAMS] + 0x1c; none: the comp_screens.cpp box, line 0x1d0f), 1 its count, 2 the page. The nation picture,
/// the World Cup's colours with the title "FIFA World Rankings", the menu bar (4) and the subtitle "<month> <year>"
/// (the rankings date at +5). The page holds as many rows as 0x15d pixels in font 3; param 3 = the pages (count / rows,
/// one more for a remainder); the page buttons (`gui_add_page_buttons`, ids 1..4) at (0x30c, 0x91). A 4-column area
/// (weights 4, 0x16, 0x13, 5; x 0x6e..0x30c, y 0xaa..0x217): each nation of the page its rank (ordinal, dark blue),
/// "  <name>" (a button id 0x7d5 to its national team), its continent's name in yellow when it has one, and its
/// points (+0xc × 10, "%4.2f", colour 0xae3192). Back/Next.
/// Original: `comp_fifa_rankings_screen_draw` (0x49c120, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_fifa_rankings_screen_draw() {
    let weights: [u8; 4] = [4, 0x16, 0x13, 5];
    let mut list = get_screen_param(0);
    let mut count = get_screen_param(1) as i32;
    let page = get_screen_param(2);
    exe::background_set_nation_picture(0, 0, 0);
    let colours = comp_colours(comp_record_of_key(KEY_CLUB_COMP_FIFA_WORLD_CUP));
    comp_lang_format()(JOB_TEXT, 0x99c8ac); // "FIFA World Rankings"
    comp_colours_title(colours, JOB_TEXT);
    exe::menubar_build(4, 0);
    let rankings = read_usize(read_usize(G_NATIONAL_TEAMS) + 0x1c);
    let year = read_u16(rankings + 7) as u32;
    let month = exe::date_get_month_name(exe::date_get_month(rankings + 5) as u32);
    comp_lang_format()(JOB_TEXT, 0x99c890, month, year); // "<%s - month> <%d - year>"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x15d / exe::font_get_height(3)) as u8;
    if list == 0 {
        list = exe::fifa_rankings_collect_ranked_order(rankings, &mut count);
        if list == 0 {
            comp_screens_error_box(0x1d0f);
            return;
        }
        covered!("tel23_2 fifa rankings list built");
        set_screen_param(0, list);
        set_screen_param(1, count as usize);
    }
    let per_page = rows as i32;
    let pages = count / per_page + (count % per_page != 0) as i32;
    set_screen_param(3, pages as i16 as isize as usize);
    exe::gui_add_page_buttons(0x30c, 0x91, (page) as usize as u32, (pages) as i32 as u32, 1, 2, 3, 4);
    let area = tel26_add_area(current_human_display(), [0x6e, 0xaa, 0x30c, 0x217, 4, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let first = (page as i16 as i32 - 1) * per_page;
    for k in 0..per_page {
        if first + k >= count {
            break;
        }
        let rank = exe::utils_format_ordinal(first + k + 1, -1);
        tel26_area_text([area, 0, k as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), rank as u32]);
        let entry = read_usize(list + (first + k) as usize * 4);
        let nation = Nation(read_usize(G_NATION_TABLE) + read_u32(entry) as usize * nation::SIZE);
        comp_crt_format()(JOB_TEXT, 0x987ff0, nation.addr() + 4); // "  %s"
        let team = exe::database_get_national_team(nation);
        tel26_area_button([area, 1, k as u32, 1, 0, 1, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0x7d5, team as u32]);
        let continent = nation.continent();
        if continent != 0 {
            tel26_area_text([area, 2, k as u32, 1, 0, 1, 2, job_colour(G_GUI_COLOUR_YELLOW), (continent + 4) as u32]);
        }
        let points = read_f32(entry + 0xc) as f64 * read_f32(rdata::F32_10) as f64;
        comp_crt_format()(JOB_TEXT, 0x98fe94, points); // "%4.2f"
        tel26_area_text([area, 3, k as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
    }
    tel26_back_next();
}

/// Rule: the FIFA world rankings screen's clicks: the page buttons move param 2: id 1 ten pages back, id 2 one back
/// (not below 1), id 3 one on, id 4 ten on (not past param 3, the pages); −11. Other ids go to the menu bar.
/// Original: `comp_fifa_rankings_screen_handle_event` (0x49c510, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_fifa_rankings_screen_handle_event(widget: u32) -> i32 {
    let page = get_screen_param(2) as i16 as i32;
    let pages = get_screen_param(3) as i16 as i32;
    let new = match tel26_item_id(widget as i16) {
        1 => (page - 10).max(1),
        2 => (page - 1).max(1),
        3 => (page + 1).min(pages),
        4 => (page + 10).min(pages),
        _ => return comp_click_menubar(widget),
    };
    covered!("tel23_2 fifa rankings page");
    set_screen_param(2, new as i16 as isize as usize);
    -11
}

/// Rule: leaving the FIFA world rankings screen frees the ranked list (param 0, crt free) and clears params 0 and 1.
/// Original: `comp_fifa_rankings_screen_close` (0x49c6c0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_fifa_rankings_screen_close() {
    comp_free_block_param(0);
}

/// Rule: opening the UEFA coefficients screen (builder `comp_uefa_coefficients_screen_draw`, input
/// `comp_uefa_coefficients_screen_handle_event`, no flag, no leave): params 0 (the scroll row) and 1 (the area) 0.
/// Original: `comp_open_uefa_coefficients_screen` (0x49c700, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_open_uefa_coefficients_screen() {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49c740usize, 0x49cc10usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel23_2 open uefa coefficients");
    set_screen_param(0, 0);
    set_screen_param(1, 0);
}

/// Rule: building the UEFA coefficients screen: the nations' coefficients and their European places.
///
/// The European Cup's picture and colours with the title "UEFA Coefficients", the menu bar (4) and the subtitle
/// "<year>/<next year % 100>" (the seeding's year, +0x439). A 6-column area (weights 4, 0x15, 6, 6, 6, 7; x 0x6e..0x30c,
/// y 0x91..0x217, as many rows as 0x176 pixels in font 3). Row 0 heads the columns: "ECC", "UEFA" and "INTER" as
/// buttons (id 0x7d0) to the European Cup, the UEFA Cup and the Intertoto Cup, and "Coef.". Then each nation of the
/// seeding's table (`UefaSeeding::get_nation_table`, its count as a signed byte), from row 1: its rank (ordinal, dark
/// blue), "  <short name>" (a button id 0x7d5 to its national team), its three places in yellow and its coefficient
/// ("%4.2f", colour 0xae3192). The scroll row from param 0 and the area to param 1; Back/Next.
/// Original: `comp_uefa_coefficients_screen_draw` (0x49c740, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_uefa_coefficients_screen_draw() {
    let weights: [u8; 6] = [4, 0x15, 6, 6, 6, 7];
    let european_cup = comp_record_of_key(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE);
    exe::background_set_comp_picture(european_cup, 0, 0);
    let colours = comp_colours(comp_record_of_key(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE));
    comp_lang_format()(JOB_TEXT, 0x99c924); // "UEFA Coefficients"
    comp_colours_title(colours, JOB_TEXT);
    exe::menubar_build(4, 0);
    let year = read_u16(read_usize(G_UEFA_SEEDING) + 0x439) as i32;
    comp_lang_format()(JOB_TEXT, 0x99c908, year, (year + 1) % 100); // "<%d - year 1>/<%d - year 2>"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x176 / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, 0x91, 0x30c, 0x217, 6, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    comp_lang_format()(JOB_TEXT, 0x99c904); // "ECC"
    tel26_area_button([area, 2, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x7d0, comp_record_of_key(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) as u32]);
    comp_lang_format()(JOB_TEXT, 0x99c8fc); // "UEFA"
    tel26_area_button([area, 3, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x7d0, comp_record_of_key(KEY_CLUB_COMP_UEFA_CUP) as u32]);
    comp_lang_format()(JOB_TEXT, 0x99c8c8, 0xad9c64usize); // "INTER<%s - COMMENT - Intertoto Cup abbreviation>"
    tel26_area_button([area, 4, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x7d0, comp_record_of_key(KEY_CLUB_COMP_INTER_TOTO_CUP) as u32]);
    comp_lang_format()(JOB_TEXT, 0x99c8c0); // "Coef."
    tel26_area_text([area, 5, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32]);
    let mut table = 0usize;
    let n = exe::uefaseeding_get_nation_table(read_usize(G_UEFA_SEEDING), &mut table) as i8 as i32;
    for k in 1..=n {
        let entry = table + (k - 1) as usize * UEFA_COEFFICIENT_ENTRY;
        let rank = exe::utils_format_ordinal(k, -1);
        tel26_area_text([area, 0, k as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), rank as u32]);
        let nation = Nation(read_usize(entry + 4));
        comp_crt_format()(JOB_TEXT, 0x987ff0, nation.addr() + nation::SHORT_NAME); // "  %s"
        let team = exe::database_get_national_team(nation);
        tel26_area_button([area, 1, k as u32, 1, 0, 1, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0x7d5, team as u32]);
        for (column, offset) in [(2u32, 0xcusize), (3, 0xd), (4, 0xe)] {
            comp_crt_format()(JOB_TEXT, 0x98806c, read_u8(entry + offset) as i8 as i32); // "%d"
            tel26_area_text([area, column, k as u32, 1, 0, 0xc, 2, job_colour(G_GUI_COLOUR_YELLOW), JOB_TEXT as u32]);
        }
        comp_crt_format()(JOB_TEXT, 0x98fe94, read_f32(entry + 8) as f64); // "%4.2f"
        tel26_area_text([area, 5, k as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
    }
    comp_restore_area_scroll(area, 0);
    set_screen_param(1, area as i16 as isize as usize);
    tel26_back_next();
}

/// Rule: the UEFA coefficients screen's clicks: with a widget the table keeps its scroll row (param 0 from the area in
/// 1); every click goes to the menu bar.
/// Original: `comp_uefa_coefficients_screen_handle_event` (0x49cc10, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_uefa_coefficients_screen_handle_event(widget: u32) -> i32 {
    if widget as i16 != -1 {
        comp_click_keep_scroll(0, 1);
    }
    comp_click_menubar(widget)
}

/// Comparer for `comp_fixture_resolve_league_clubs`: side by side by value (the answer and the three outputs), the
/// error box (an argument missing) only the original.
pub(crate) unsafe extern "C" fn compare_comp_fixture_resolve_league_clubs(fixture: usize, league_out: *mut usize, home_out: *mut usize, away_out: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize, *mut usize, *mut usize) -> u32>(IDX_COMP_FIXTURE_RESOLVE_LEAGUE_CLUBS);
    if fixture == 0 || league_out.is_null() || home_out.is_null() || away_out.is_null() {
        return o(fixture, league_out, home_out, away_out);
    }
    let (mut lo, mut ho, mut ao) = (0usize, 0usize, 0usize);
    let (mut lr, mut hr, mut ar) = (0usize, 0usize, 0usize);
    let ro = o(fixture, &mut lo, &mut ho, &mut ao);
    let rr = comp_fixture_resolve_league_clubs(fixture, &mut lr, &mut hr, &mut ar);
    SCOREBOARD.record((ro, lo, ho, ao) == (rr, lr, hr, ar), || format!("comp_fixture_resolve_league_clubs({:#x}): {} {:#x} {:#x} {:#x} x {} {:#x} {:#x} {:#x}", fixture, ro, lo, ho, ao, rr, lr, hr, ar));
    *league_out = lo;
    *home_out = ho;
    *away_out = ao;
    ro
}

/// Comparer for `comp_open_league_progress_for_fixture`: only the original (it opens a screen); the params checked
/// against the plan afterwards when the screen opened (param 0 the league found).
pub(crate) unsafe extern "C" fn compare_comp_open_league_progress_for_fixture(fixture: usize) {
    let (mut league, mut home, mut away) = (0usize, 0usize, 0usize);
    let checkable = fixture != 0 && (fixture as i32).wrapping_abs() >= 10000;
    if checkable {
        comp_fixture_resolve_league_clubs(fixture, &mut league, &mut home, &mut away);
    }
    original_fn::<unsafe extern "C" fn(usize)>(IDX_COMP_OPEN_LEAGUE_PROGRESS_FOR_FIXTURE)(fixture);
    if checkable && league != 0 && get_screen_param(0) == league {
        let plan = league_progress_planned_params(league, home, away);
        let got: [usize; 5] = core::array::from_fn(|k| get_screen_param(k as u32));
        SCOREBOARD.record(got == plan, || format!("league progress screen (open {:#x}): params {:x?} x plan {:x?}", fixture, got, plan));
    }
}

/// Comparer for `comp_league_progress_screen_draw`: side by side over the display list (plus the league's table
/// column; the sort finds the same order twice), only the original without a league (the error box).
pub(crate) unsafe extern "C" fn compare_comp_league_progress_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_COMP_LEAGUE_PROGRESS_SCREEN_DRAW);
    let league = League(get_screen_param(0));
    if league.is_null() {
        return o();
    }
    compare_match_screen_build("league progress screen", &[(league.addr() + league::TABLE_COLUMN, 4)], || o(), || comp_league_progress_screen_draw());
}

/// Comparer for `comp_league_progress_screen_handle_event`: side by side over the screen manager and the params for
/// the table rows (id 1); the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_comp_league_progress_screen_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_LEAGUE_PROGRESS_SCREEN_HANDLE_EVENT);
    let pure = tel26_item_id(widget as i16) == 1;
    comp_click_compare("league progress handle_event", widget, pure, || o(widget), || comp_league_progress_screen_handle_event(widget))
}

/// Comparer for `comp_open_fifa_rankings_screen`: only the original, params 0..3 checked afterwards when it opened
/// (param 2 = 1 with no list).
pub(crate) unsafe extern "C" fn compare_comp_open_fifa_rankings_screen() {
    original_fn::<unsafe extern "C" fn()>(IDX_COMP_OPEN_FIFA_RANKINGS_SCREEN)();
    let got: [usize; 4] = core::array::from_fn(|k| get_screen_param(k as u32));
    if got[0] == 0 && got[2] == 1 {
        SCOREBOARD.record(got == FIFA_RANKINGS_OPEN_PARAMS, || format!("fifa rankings screen (open): params {:x?} x plan {:x?}", got, FIFA_RANKINGS_OPEN_PARAMS));
    }
}

/// Comparer for `comp_fifa_rankings_screen_draw`: side by side over the display list once the ranked list exists; its
/// first build (it allocates the list into params 0 and 1) only the original.
pub(crate) unsafe extern "C" fn compare_comp_fifa_rankings_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_COMP_FIFA_RANKINGS_SCREEN_DRAW);
    if get_screen_param(0) == 0 {
        // simplification: the allocation does not run twice; proven in Rust mode
        return o();
    }
    compare_match_screen_build("fifa rankings screen", &[], || o(), || comp_fifa_rankings_screen_draw());
}

/// Comparer for `comp_fifa_rankings_screen_handle_event`: side by side for the page buttons (ids 1..4); the menu bar
/// only the original.
pub(crate) unsafe extern "C" fn compare_comp_fifa_rankings_screen_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_FIFA_RANKINGS_SCREEN_HANDLE_EVENT);
    let pure = (1..=4).contains(&tel26_item_id(widget as i16));
    comp_click_compare("fifa rankings handle_event", widget, pure, || o(widget), || comp_fifa_rankings_screen_handle_event(widget))
}

/// Comparer for `comp_fifa_rankings_screen_close`: only the original (it frees the list), params 0 and 1 checked
/// afterwards (cleared when there was a list).
pub(crate) unsafe extern "C" fn compare_comp_fifa_rankings_screen_close() {
    let before = (get_screen_param(0), get_screen_param(1));
    original_fn::<unsafe extern "C" fn()>(IDX_COMP_FIFA_RANKINGS_SCREEN_CLOSE)();
    let plan = if before.0 != 0 { (0, 0) } else { before };
    let got = (get_screen_param(0), get_screen_param(1));
    SCOREBOARD.record(got == plan, || format!("fifa rankings screen (close): params 0/1 {:x?} x plan {:x?}", got, plan));
}

/// Comparer for `comp_open_uefa_coefficients_screen`: only the original (it opens a screen).
pub(crate) unsafe extern "C" fn compare_comp_open_uefa_coefficients_screen() {
    // simplification: opening a screen does not run twice; the two zero params are not worth a plan
    original_fn::<unsafe extern "C" fn()>(IDX_COMP_OPEN_UEFA_COEFFICIENTS_SCREEN)()
}

/// Comparer for `comp_uefa_coefficients_screen_draw`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_uefa_coefficients_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_COMP_UEFA_COEFFICIENTS_SCREEN_DRAW);
    compare_match_screen_build("uefa coefficients screen", &[], || o(), || comp_uefa_coefficients_screen_draw());
}

/// Comparer for `comp_uefa_coefficients_screen_handle_event`: only the original (every click goes to the menu bar).
pub(crate) unsafe extern "C" fn compare_comp_uefa_coefficients_screen_handle_event(widget: u32) -> i32 {
    // simplification: the menu bar opens screens; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_UEFA_COEFFICIENTS_SCREEN_HANDLE_EVENT)(widget)
}

/// Rule: opening the Intertoto application screen of a news item (builder `comp_intertoto_apply_screen_draw`, input
/// `comp_intertoto_apply_screen_handle_event`, flag 1, no leave): params 0 and 1 the news item's ids.
/// Original: `comp_open_intertoto_apply_screen` (0x49cc60, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_open_intertoto_apply_screen(a: u32, b: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49ccb0usize, 0x49ce50usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel23_3 open intertoto apply");
    set_screen_param(0, a as usize);
    set_screen_param(1, b as usize);
}

/// Rule: building the Intertoto application screen: "Apply for the one English Intertoto Cup place ?" with No and Yes.
///
/// The Intertoto Cup's picture, the menu bar (4), its colours with its name as the title and "Please Confirm" as the
/// subtitle. The question in a text area (x 0x6e..0x30c, y 0x91..0x217, pale grey, font 4) with the cup's name as a
/// link (id 0x7d0). A 2-column area (weights 3, 1; x 0x64..0x316, y 0x22b..0x24e) with the "No" (id 1) and "Yes" (id 2)
/// buttons.
/// Original: `comp_intertoto_apply_screen_draw` (0x49ccb0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_intertoto_apply_screen_draw() {
    let weights: [u8; 2] = [3, 1];
    let cup = comp_record_of_key(KEY_CLUB_COMP_INTER_TOTO_CUP);
    exe::background_set_comp_picture(cup, 0, 0);
    exe::menubar_build(4, 0);
    comp_colours_title(comp_colours(cup), cup + competition::NAME);
    comp_lang_format()(JOB_TEXT, 0x9909b8); // "Please Confirm"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let gender = read_u8(cup + competition::GENDER_NAME) as i8 as i32;
    comp_lang_format()(JOB_TEXT, 0x99c938, gender, gender, cup + competition::NAME); // "Apply for the one English {}<%s - competition>{} place ?"
    type TextArea = unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, usize, u32, usize, u32);
    let text_area: TextArea = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    text_area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT, cup + competition::NAME, 0x7d0, cup, 0);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    comp_lang_format()(JOB_TEXT, 0x98e8cc); // "No"
    tel26_area_button([area, 0, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 1, 0]);
    comp_lang_format()(JOB_TEXT, 0x98e8d0); // "Yes"
    tel26_area_button([area, 1, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 2, 0]);
}

/// Rule: the Intertoto application's answer.
///
/// The news item is (param 0, param 1). "Yes" (id 2): the item is looked up (`News::find_item_by_id`; none: the
/// comp_screens.cpp box, line 0x1e7f, and 0); its club (slot 1, the club table's record) volunteers for the Intertoto
/// Cup (`UefaSeeding::add_intertoto_volunteer`); slot 0 of the item = 1 (answered) and slot 3 = 1; −9. "No" (id 1):
/// slot 0 = 1 and slot 3 = 0; −9. Other ids go to the menu bar.
/// Original: `comp_intertoto_apply_screen_handle_event` (0x49ce50, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_intertoto_apply_screen_handle_event(widget: u32) -> i32 {
    let mut item = [0u8; NEWS_ITEM_BYTES];
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    let a = get_screen_param(0) as u32;
    let b = get_screen_param(1) as u32;
    match tel26_item_id(widget as i16) {
        2 => {
            if news_find_item(a, b, item.as_mut_ptr() as usize) == 0 {
                comp_screens_error_box(0x1e7f);
                return 0;
            }
            covered!("tel23_3 intertoto yes");
            let club = read_usize(G_CLUB_TABLE) + exe::newsitem_get_slot(item.as_mut_ptr() as usize, 1) as usize * club::SIZE;
            exe::uefaseeding_add_intertoto_volunteer(read_usize(G_UEFA_SEEDING), club);
            news_set_item_slot(a, b, 0, 1, 0);
            news_set_item_slot(a, b, 3, 1, 0);
            -9
        }
        1 => {
            covered!("tel23_3 intertoto no");
            news_set_item_slot(a, b, 0, 1, 0);
            news_set_item_slot(a, b, 3, 0, 0);
            -9
        }
        _ => comp_click_menubar(widget),
    }
}

/// Rule: a news item's link to a competition: the item (`a`, `b`) is looked up and the competition screen opens
/// (`comp_screens_open_comp_screen`) on the competition record of slot 0, the stage of slot 4 (a signed byte) and the
/// round of slot 5 (a signed word), mode 0.
/// Original: `comp_screens_open_comp_from_news_item` (0x49eac0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_open_comp_from_news_item(a: u32, b: u32) {
    let mut item = [0u8; NEWS_ITEM_BYTES];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0, 0);
    news_find_item(a, b, p);
    let comp = exe::newsitem_get_slot(p, 0);
    let stage = exe::newsitem_get_slot(p, 4) as u8 as i8;
    let round = exe::newsitem_get_slot(p, 5) as u16 as i16;
    exe::comp_screens_open_comp_screen(read_usize(G_CLUB_COMP_TABLE) + comp as usize * 0x6b, stage as i32, round as i32, 0);
}

/// Rule: the related competitions menu of a competition screen: a small menu button (top left) whose popup lists the
/// other competitions of the competition's nation and its written history.
///
/// Nothing while [0x99c00c] is clear (answers 1). No object: the comp_screens.cpp box (line 0x1ec4), 0. The nation's
/// entry of the league table ([G_LEAGUE_TABLE] + 0x48 × `key_nation_index_of_comp`): each competition object of its
/// list (+0x10, +0xc of them) adds its name (+0x38 of its record) as an item (id `comp_id`, the record as value),
/// creating the button and popup at the first one. When the competition has a written history file
/// (`history_his_file_exists` of its English name): a separator when the popup exists (else the popup is created) and
/// "History" (id `history_id`). Answers whether the popup was created.
/// Original: `comp_screens_add_related_comps_menu` (0x49d070, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_add_related_comps_menu(object: usize, colour_a: u32, colour_b: u32, comp_id: u32, history_id: u32) -> u32 {
    let object = League(object);
    if read_u32(COMP_SCREEN_BUILT_FLAG) == 0 {
        return 1;
    }
    if object.is_null() {
        comp_screens_error_box(0x1ec4);
        return 0;
    }
    let mut area: u32 = u32::MAX;
    let index = exe::key_nation_index_of_comp(object.competition().addr()) as i8 as isize;
    let entry = (G_LEAGUE_TABLE as isize + index * 0x48) as usize;
    let (c31aa, yellow, black) = (job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_BLACK));
    if read_usize(entry + 0x10) != 0 {
        let mut k: i32 = 0;
        while k < read_i32(entry + 0xc) {
            let other = League(read_usize(read_usize(entry + 0x10) + k as usize * 4));
            if !other.is_null() {
                if area as u16 == 0xffff {
                    area = related_comps_popup(colour_a, colour_b);
                }
                let record = other.competition().addr();
                tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, yellow, 1, 1, black, (record + competition::SHORT_NAME) as u32, 0, comp_id, record as u32, area]);
            }
            k += 1;
        }
    }
    if exe::history_his_file_exists(comp_get_english_name(object.competition().addr())) != 0 {
        covered!("tel23_3 related comps history");
        if area as u16 == 0xffff {
            area = related_comps_popup(colour_a, colour_b);
        } else {
            tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 0, 0, 0x1000010, c31aa, yellow, 1, 1, black, 0, 0, 0, 0, area]);
        }
        comp_lang_format()(JOB_TEXT, 0x9914d4, 0xad9c64usize); // "History<%s - COMMENT - written history screen>"
        tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, yellow, 1, 1, black, JOB_TEXT as u32, 0, history_id, 0, area]);
    }
    (area as u16 != 0xffff) as u32
}

/// Rule: the stage menus above a competition tab: a "Stage" menu of the competition's stages and, for the selected
/// one, a second menu of its groups or rounds.
///
/// No competition: the comp_screens.cpp box (line 0x1f1b), 0. A competition still in its first stage (current stage
/// −1) has none when it is a league, or a cup with one round to draw. Then, for each stage k from −1 to the current
/// one (`Comp::get_stage`): a stage with a parent stage id (`comp_util_get_parent_stage_id`) heads a run of siblings
/// (the following stages with the same parent): when the run is every stage, each sibling goes on the main menu as a
/// "Group"; otherwise the run goes on it once as "Stage" (the parent's id) and, when the selected stage is in the run,
/// each sibling goes on the second menu (x 0x210..0x28d) as a "Group"; k skips the run. A cup stage: on a competition
/// still in its first stage its rounds go on the main menu as "Round" (with the stage id); a cup of one round to draw
/// goes on it as "Stage" with its first round's code; a cup without a stage id lists its rounds there as "Stage"
/// (code 0); otherwise it goes on as "Stage" and, when selected, its rounds on the second menu as "Round" (code 0). A
/// league stage goes on the main menu as "Stage". Each item by `comp_screens_add_stage_menu_item` (main menu x
/// 0x28f..0x30c, `top`..`top` + 0x14). Answers whether either menu exists.
/// Original: `comp_screens_add_stage_selector` (0x49d380, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_add_stage_selector(comp: usize, selected: i32, top: i32) -> u32 {
    let comp = League(comp);
    if comp.is_null() {
        comp_screens_error_box(0x1f1b);
        return 0;
    }
    if (comp.current_stage() as i32) == -1 {
        if comp.comp_type() == 2 {
            return 0;
        }
        if comp.comp_type() == 1 && read_u8(comp.addr() + cup::NEXT_DRAW_ROUND) == 1 {
            return 0;
        }
    }
    let (mut stage_text, mut group_text, mut round_text) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    comp_lang_format()(stage_text.as_mut_ptr() as usize, 0x99c518); // "Stage"
    comp_lang_format()(group_text.as_mut_ptr() as usize, 0x99c97c); // "Group"
    comp_lang_format()(round_text.as_mut_ptr() as usize, 0x99c974); // "Round"
    let (stage_text, group_text, round_text) = (stage_text.as_ptr() as usize, group_text.as_ptr() as usize, round_text.as_ptr() as usize);
    let (mut menu, mut sub): (i32, i32) = (-1, -1);
    if (comp.current_stage() as i32) < -1 {
        return 0;
    }
    let stage_id = |s: usize| read_i16(s + league::STAGE_ID) as i32;
    let mut k: i32 = -1;
    loop {
        let s = exe::comp_get_stage(comp, k);
        let parent = exe::comp_util_get_parent_stage_id(stage_id(s));
        if parent != -1 {
            let mut n: i32 = 1;
            let mut j = k + 1;
            while j <= (comp.current_stage() as i32) {
                let sibling = read_usize(comp.stages() + j as usize * 4);
                if exe::comp_util_get_parent_stage_id(stage_id(sibling)) != parent {
                    break;
                }
                n += 1;
                j += 1;
            }
            if n == (comp.current_stage() as i32) + 2 {
                covered!("tel23_3 stage selector all groups");
                let mut i = k;
                while i < n + k {
                    comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, group_text, stage_id(exe::comp_get_stage(comp, i)), 0, i, 0);
                    i += 1;
                }
            } else {
                comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, stage_text, parent, 0, k, 0);
                if selected >= k && selected < n + k {
                    let mut i = k;
                    while i < n + k {
                        comp_screens_add_stage_menu_item(&mut sub as *mut i32 as *mut i16, 0x210, top, 0x28d, top + 0x14, group_text, stage_id(exe::comp_get_stage(comp, i)), 0, i, 0);
                        i += 1;
                    }
                }
            }
            k = n + k - 1;
        } else {
            match read_u8(s + league::COMP_TYPE) {
                1 => {
                    if (comp.current_stage() as i32) == -1 {
                        stage_selector_cup_rounds(&mut menu, 0x28f, 0x30c, top, round_text, stage_id(s), k, s);
                    } else if read_u8(s + cup::NEXT_DRAW_ROUND) == 1 {
                        let first_code = read_i16(read_usize(s + cup::ROUND_TABLE) + cup_round::ROUND_CODE) as i32;
                        comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, stage_text, stage_id(s), first_code, k, 0);
                    } else if read_u16(s + league::STAGE_ID) == 0 {
                        stage_selector_cup_rounds(&mut menu, 0x28f, 0x30c, top, stage_text, 0, k, s);
                    } else {
                        comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, stage_text, stage_id(s), 0, k, 0);
                        if selected == k {
                            stage_selector_cup_rounds(&mut sub, 0x210, 0x28d, top, round_text, 0, k, s);
                        }
                    }
                }
                2 => {
                    comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, stage_text, stage_id(s), 0, k, 0);
                }
                _ => {}
            }
        }
        k += 1;
        if k > (comp.current_stage() as i32) {
            break;
        }
    }
    (menu as i16 != -1 || sub as i16 != -1) as u32
}

/// Rule: an item of a stage menu: the menu's button and popup made at the first item, then the stage or round's name.
///
/// No button text: the comp_screens.cpp box (line 0x2001), 0. While `*area` is −1 the button (x1, y1, x2, y2 with the
/// text, grey on pale grey, font 0xc) and its popup (flags 0x130) are made, the popup's index written to `*area` (a
/// word) and [0xadae0c] = 1. The item's name is the stage and round name (`comp_util_format_stage_round_name` of
/// `code` and `round`), "League" when that is empty; the item (id 5, value 1000 × (`stage` + 1) + `index`) goes in the
/// popup. Answers 1.
/// Original: `comp_screens_add_stage_menu_item` (0x49d930, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_add_stage_menu_item(area: *mut i16, x1: u32, y1: i32, x2: u32, y2: i32, text: usize, code: i32, round: i32, stage: i32, index: i32) -> u32 {
    if text == 0 {
        comp_screens_error_box(0x2001);
        return 0;
    }
    if *area == -1 {
        covered!("tel23_3 stage menu made");
        let button = tel26_add_button([x1, y1 as u32, x2, y2 as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), text as u32, 0, 0]);
        *area = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]) as i16;
        write_u32(COMP_SCREEN_DATE_BUTTONS_FLAG, 1);
    }
    exe::comp_util_format_stage_round_name(code, round, JOB_TEXT, 0x7d0, 0);
    if read_u8(JOB_TEXT) == 0 {
        comp_lang_format()(JOB_TEXT, 0x990f4c); // "League"
    }
    let value = (stage + 1) * 1000 + index; // tel23_3 the stage item's value
    tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), 1, 1, job_colour(G_GUI_COLOUR_BLACK),
                                              JOB_TEXT as u32, 0, 5, value as u32, *area as u16 as u32]);
    1
}

/// Rule: one row of fixtures in a competition list: both clubs with their positions, the score or the "view" link,
/// and a venue line under it when the match is on neutral ground.
///
/// No fixture: the comp_screens.cpp box (line 0x203e), 0. Columns of `area`, row `*row`: 0 the home side cell and 8
/// the away side cell (`fixture_row_side_cell`), 2 and 6 the club names (`fixture_row_club_cell`); 1 and 7 the tie
/// marks in yellow font 0xc (the aggregate +0x4b/+0x4c when there is one, style 3; else the result marks +0x3d/+0x3e,
/// style 1). A played match (+0x43 ≠ 0xff): the event letters of the two marks in columns 3 (font 0x40) and 5, and in
/// column 4 the score in colour 0xaebde8: "<home>    ", ":" and "    <away>" (the extra time goals +0x47/+0x48 when
/// there are, else +0x43/+0x44); with match statistics (+8 ≥ 0, `fixture_entry_matches`) the ":" is a button 0x7d3 to
/// them; a postponed match (0xfe) shows "P    " and "    P", an abandoned one (0xfd) "A    " (a button 0x7d3) and
/// "    A" (style 2). An unplayed match: "view" (yellow, style 1) when a human is watching it, else "v", a button 0x7d2
/// with the fixture. Then the row moves on; on neutral ground (+0x3c) with a stadium (+0x18): "<stadium>, <city>"
/// (or the stadium's name) in colour 0xaebdd4 font 2 from x 0xb4 to its width, and the row moves on again. Answers 1.
/// Original: `comp_screens_add_fixture_row` (0x49db00, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_add_fixture_row(fixture: usize, area: u32, row: *mut i32, id: u32) -> u32 {
    let fixture = Fixture(fixture);
    if fixture.is_null() {
        comp_screens_error_box(0x203e);
        return 0;
    }
    let home = fixture.home_club().addr();
    let away = fixture.away_club().addr();
    let r = *row;
    fixture_row_side_cell(fixture.addr(), home, fixture.home_side_mark(), 0, area, r, id);
    fixture_row_club_cell(home, fixture.home_side_mark(), 2, area, r);
    let (yellow, score) = (job_colour(G_GUI_COLOUR_YELLOW), job_colour(JOB_COLOUR_BDE8));
    let (mut home_mark, mut away_mark) = ([0u8; 8], [0u8; 8]);
    let style = if (fixture.home_aggregate() as u8) != 0xff {
        comp_crt_format()(home_mark.as_mut_ptr() as usize, 0x98806c, (fixture.home_aggregate() as u8) as i8 as i32); // "%d"
        comp_crt_format()(away_mark.as_mut_ptr() as usize, 0x98806c, (fixture.away_aggregate() as u8) as i8 as i32); // "%d"
        3
    } else {
        fixture_result_mark(&mut home_mark, (fixture.home_result_mark() as u8) as i8);
        fixture_result_mark(&mut away_mark, (fixture.away_result_mark() as u8) as i8);
        1
    };
    if home_mark[0] != 0 {
        tel26_area_text([area, 1, r as u32, 1, 0, 0xc, style, yellow, home_mark.as_ptr() as u32]);
    }
    if away_mark[0] != 0 {
        tel26_area_text([area, 7, r as u32, 1, 0, 0xc, style, yellow, away_mark.as_ptr() as u32]);
    }
    let home_goals = fixture.home_goals() as u8;
    if home_goals != 0xff {
        let (mut home_event, mut away_event) = ([0u8; 8], [0u8; 8]);
        fixture_event_letter(&mut home_event, (fixture.home_result_mark() as u8) as i8);
        fixture_event_letter(&mut away_event, (fixture.away_result_mark() as u8) as i8);
        let (mut home_text, mut away_text) = ([0u8; 8], [0u8; 8]);
        let (h, a) = if (fixture.home_goals_extra_time() as u8) != 0xff {
            (fixture::HOME_GOALS_EXTRA_TIME, fixture::AWAY_GOALS_EXTRA_TIME)
        } else {
            (fixture::HOME_GOALS, fixture::AWAY_GOALS)
        };
        comp_crt_format()(home_text.as_mut_ptr() as usize, 0x98806c, read_u8(fixture.addr() + h) as i8 as i32); // "%d"
        comp_crt_format()(away_text.as_mut_ptr() as usize, 0x98806c, read_u8(fixture.addr() + a) as i8 as i32); // "%d"
        if home_event[0] != 0 {
            tel26_area_text([area, 3, r as u32, 1, 0, 0x40, 1, yellow, home_event.as_ptr() as u32]);
        }
        let slot = fixture.match_stats_slot() as i32;
        if home_goals != 0xfe && home_goals != 0xfd && slot > -1
            && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot, fixture.home_club_id() as i32, fixture.away_club_id() as i32,
                                                      fixture.day() as i32, fixture.year() as i32) != 0 {
            covered!("tel23_3 fixture row stats link");
            comp_crt_format()(JOB_TEXT, 0x99c354, home_text.as_ptr()); // "%s    "
            tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32]);
            tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 3, score, 0x989adc, 0x7d3, slot as u32]); // ":"
            comp_crt_format()(JOB_TEXT, 0x99c34c, away_text.as_ptr()); // "    %s"
            tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32]);
        } else {
            match home_goals {
                0xfe => {
                    comp_lang_format()(JOB_TEXT, 0x99c9e4, 0xad9c64usize); // "P    <%s - COMMENT - postponed>"
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 2, score, JOB_TEXT as u32]);
                }
                0xfd => {
                    comp_lang_format()(JOB_TEXT, 0x99c9c4, 0xad9c64usize); // "A    <%s - COMMENT - abandoned>"
                    tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 2, score, JOB_TEXT as u32, 0x7d3, slot as u32]);
                }
                _ => {
                    comp_crt_format()(JOB_TEXT, 0x99c354, home_text.as_ptr()); // "%s    "
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32]);
                }
            }
            tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, 0x989adc]); // ":"
            match home_goals {
                0xfe => {
                    comp_lang_format()(JOB_TEXT, 0x99c9a4, 0xad9c64usize); // "    P<%s - COMMENT - postponed>"
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 2, score, JOB_TEXT as u32]);
                }
                0xfd => {
                    comp_lang_format()(JOB_TEXT, 0x99c984, 0xad9c64usize); // "    A<%s - COMMENT - abandoned>"
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 2, score, JOB_TEXT as u32]);
                }
                _ => {
                    comp_crt_format()(JOB_TEXT, 0x99c34c, away_text.as_ptr()); // "    %s"
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32]);
                }
            }
        }
        if away_event[0] != 0 {
            tel26_area_text([area, 5, r as u32, 1, 0, 1, 1, yellow, away_event.as_ptr() as u32]);
        }
    } else if exe::human_manager_is_watching_fixture(read_usize(G_HUMAN_MANAGER), fixture, 0usize) != 0 {
        comp_lang_format()(JOB_TEXT, 0x9918d4); // "view"
        tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 1, yellow, JOB_TEXT as u32, 0x7d2, fixture.addr() as u32]);
    } else {
        comp_lang_format()(JOB_TEXT, 0x99c1b4); // "v"
        tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32, 0x7d2, fixture.addr() as u32]); // tel23_3 the unplayed link's style
    }
    fixture_row_club_cell(away, fixture.away_side_mark() as u8, 6, area, r);
    fixture_row_side_cell(fixture.addr(), away, fixture.away_side_mark() as u8, 8, area, r, id);
    *row += 1;
    let stadium = fixture.stadium();
    if fixture.neutral_venue() != 0 && stadium != 0 {
        covered!("tel23_3 fixture row venue");
        let mut name = [0u8; 0x80];
        tel26_copy_text(name.as_mut_ptr(), (stadium + stadium::NAME) as *const u8);
        let city = read_usize(stadium + stadium::CITY);
        if city != 0 {
            let (g1, g2) = (read_u8(stadium + stadium::GENDER_NAME) as i8 as i32, read_u8(city + city::GENDER_NAME) as i8 as i32);
            comp_lang_format()(JOB_TEXT, 0x9902a4, g1, g1, g2, g2, name.as_ptr(), city + 4); // the city's name (+4); "{}<%s - stadium>{}, {}<%s - city>{}"
        } else {
            tel26_copy_text(JOB_TEXT as *mut u8, name.as_ptr());
        }
        let x2 = exe::font_text_width(2usize, JOB_TEXT as *const u8) + 0xb4;
        tel26_add_item(current_human_display(), [1, 0xb4, 0, x2 as u32, 0, 0, *row as u32, 1, 0, 0, 1, 2, job_colour(JOB_COLOUR_BDD4), JOB_TEXT as u32, 0, 0, 0, area]);
        *row += 1;
    }
    1
}

/// Comparer for `comp_open_intertoto_apply_screen`: only the original (it opens a screen), params 0 and 1 checked
/// afterwards when they hold the ids.
pub(crate) unsafe extern "C" fn compare_comp_open_intertoto_apply_screen(a: u32, b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_COMP_OPEN_INTERTOTO_APPLY_SCREEN)(a, b);
    let got = (get_screen_param(0), get_screen_param(1));
    if got.0 == a as usize {
        SCOREBOARD.record(got == (a as usize, b as usize), || format!("intertoto apply screen (open {} {}): params 0/1 {:x?}", a, b, got));
    }
}

/// Comparer for `comp_intertoto_apply_screen_draw`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_intertoto_apply_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_COMP_INTERTOTO_APPLY_SCREEN_DRAW);
    compare_match_screen_build("intertoto apply screen", &[], || o(), || comp_intertoto_apply_screen_draw());
}

/// Comparer for `comp_intertoto_apply_screen_handle_event`: only the original (the answer writes the news item and
/// the Intertoto volunteers; the rest is the menu bar), its answer checked for No and Yes (−9).
pub(crate) unsafe extern "C" fn compare_comp_intertoto_apply_screen_handle_event(widget: u32) -> i32 {
    let id = tel26_item_id(widget as i16);
    let r = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_COMP_INTERTOTO_APPLY_SCREEN_HANDLE_EVENT)(widget);
    if id == 1 {
        SCOREBOARD.record(r == -9, || format!("intertoto apply screen (No): answer {}", r));
    }
    r
}

/// Comparer for `comp_screens_open_comp_from_news_item`: only the original (it opens a screen).
pub(crate) unsafe extern "C" fn compare_comp_screens_open_comp_from_news_item(a: u32, b: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_COMP_SCREENS_OPEN_COMP_FROM_NEWS_ITEM)(a, b)
}

/// Comparer for `comp_screens_add_related_comps_menu`: side by side over the display list and the answer; the error
/// box (no object) only the original.
pub(crate) unsafe extern "C" fn compare_comp_screens_add_related_comps_menu(object: usize, colour_a: u32, colour_b: u32, comp_id: u32, history_id: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32>(IDX_COMP_SCREENS_ADD_RELATED_COMPS_MENU);
    if object == 0 && read_u32(COMP_SCREEN_BUILT_FLAG) != 0 {
        return o(object, colour_a, colour_b, comp_id, history_id);
    }
    let (mut ro, mut rr) = (0, 0);
    compare_match_screen_build("related comps menu", &[], || ro = o(object, colour_a, colour_b, comp_id, history_id),
        || rr = comp_screens_add_related_comps_menu(object, colour_a, colour_b, comp_id, history_id));
    SCOREBOARD.record(ro == rr, || format!("related comps menu: answer {} x {}", ro, rr));
    ro
}

/// Comparer for `comp_screens_add_stage_selector`: side by side over the display list (plus [0xadae0c]) and the
/// answer; the error box (no competition) only the original.
pub(crate) unsafe extern "C" fn compare_comp_screens_add_stage_selector(comp: usize, selected: i32, top: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32) -> u32>(IDX_COMP_SCREENS_ADD_STAGE_SELECTOR);
    if comp == 0 {
        return o(comp, selected, top);
    }
    let (mut ro, mut rr) = (0, 0);
    compare_match_screen_build("stage selector", &[(COMP_SCREEN_DATE_BUTTONS_FLAG, 4)], || ro = o(comp, selected, top), || rr = comp_screens_add_stage_selector(comp, selected, top));
    SCOREBOARD.record(ro == rr, || format!("stage selector: answer {} x {}", ro, rr));
    ro
}

/// Comparer for `comp_screens_add_stage_menu_item`: side by side over the display list, [0xadae0c] and the caller's
/// popup word; the error box (no text) only the original.
pub(crate) unsafe extern "C" fn compare_comp_screens_add_stage_menu_item(area: *mut i16, x1: u32, y1: i32, x2: u32, y2: i32, text: usize, code: i32, round: i32, stage: i32, index: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(*mut i16, u32, i32, u32, i32, usize, i32, i32, i32, i32) -> u32>(IDX_COMP_SCREENS_ADD_STAGE_MENU_ITEM);
    if text == 0 || area.is_null() {
        return o(area, x1, y1, x2, y2, text, code, round, stage, index);
    }
    let mut r = 0;
    compare_match_screen_build("stage menu item", &[(COMP_SCREEN_DATE_BUTTONS_FLAG, 4), (area as usize, 2)], || r = o(area, x1, y1, x2, y2, text, code, round, stage, index),
        || { comp_screens_add_stage_menu_item(area, x1, y1, x2, y2, text, code, round, stage, index); });
    r
}

/// Comparer for `comp_screens_add_fixture_row`: side by side over the display list and the caller's row; the error box
/// (no fixture) only the original.
pub(crate) unsafe extern "C" fn compare_comp_screens_add_fixture_row(fixture: usize, area: u32, row: *mut i32, id: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, *mut i32, u32) -> u32>(IDX_COMP_SCREENS_ADD_FIXTURE_ROW);
    if fixture == 0 || row.is_null() {
        return o(fixture, area, row, id);
    }
    let mut r = 0;
    compare_match_screen_build("fixture row", &[(row as usize, 4)], || r = o(fixture, area, row, id), || { comp_screens_add_fixture_row(fixture, area, row, id); });
    r
}

/// Opens the competition screen of a competition (`comp` points to its id, an index of `G_COMP_OBJECT_TABLE`) with the
/// two given parameters (2 and 3); with `restore` the 60 parameters of the current screen are saved first and 1, 0x1e
/// and 0x23 come back after the defaults. The parameters: 0 = the competition, 1 = 1, 2 and 3 the given ones,
/// 4..0x1d = 0, 0x23 = 3, 0x24..0x27 = 0, 0x1e = 0x11, 0x1f..0x22 = 0, 0x28 = 0x29 = 0, 0x2a = -1, 0x2b = 0, (the
/// restored ones), 0x2c = 0, 0x2d = 1, 0x2e..0x31 = 0, 0x32 = -1, 0x33..0x35 = 0, 0x36 = -1. Nothing more when the screen does not open (`ScreenManager::open_screen`
/// with the competition screen's functions, not single instance).
/// Original: `comp_screens_open_comp_screen` (0x48f4a0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_open_comp_screen(comp: usize, second: usize, third: usize, restore: u32) {
    let mut saved = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in saved.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(comp) * 4);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, FN_COMP_SCREENS_DRAW_COMP_SCREEN, FN_COMP_SCREENS_HANDLE_COMP_SCREEN_CLICK, 0,
        FN_COMP_SCREENS_CLOSE_COMP_SCREEN, 0) == 0 {
        return;
    }
    covered!("tel30_10 competition screen opened");
    set_screen_param(0, object);
    set_screen_param(1, 1);
    set_screen_param(2, second);
    set_screen_param(3, third);
    for k in 4..=0x1d {
        set_screen_param(k, 0);
    }
    set_screen_param(0x23, 3);
    for k in 0x24..=0x27 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x1e, 0x11);
    for k in 0x1f..=0x22 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x28, 0);
    set_screen_param(0x29, 0);
    set_screen_param(0x2a, usize::MAX);
    set_screen_param(0x2b, 0);
    if restore != 0 {
        set_screen_param(1, saved[1]);
        set_screen_param(0x1e, saved[0x1e]);
        set_screen_param(0x23, saved[0x23]);
    }
    set_screen_param(0x2c, 0);
    set_screen_param(0x2d, 1);
    for k in 0x2e..=0x31 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x32, usize::MAX);
    for k in 0x33..=0x35 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x36, usize::MAX);
}

/// Builds a competition's season records: the dates at +0x29, +0x4c, +0x75 and +0x94, the four run records at
/// +0xb3, +0xcb, +0xe3 and +0xfb (`run_record_construct` 0x448670), the date at +0x124, the record holder at +0x133
/// and the fixed fixture at +0x14b (`FixedFixture::FixedFixture` 0x645800), the record holders at +0x16b, +0x18d,
/// +0x1ad and +0x1cd, and the youngest and oldest player records at +0x1eb and +0x22a; returns the records.
/// Original: `Comp_Season_Records::Comp_Season_Records` (0x49f160, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_season_records_comp_season_records(this: usize) -> usize {
    for at in [0x29, 0x4c, 0x75, 0x94] {
        comp_records_date(this + at);
    }
    for at in [0xb3, 0xcb, 0xe3, 0xfb] {
        exe::run_record_construct(this + at);
    }
    comp_records_date(this + 0x124);
    exe::record_holder_construct(this + 0x133);
    exe::new_fixedfixture(this + 0x14b);
    for at in [0x16b, 0x18d, 0x1ad, 0x1cd] {
        exe::record_holder_construct(this + at);
    }
    comp_youngest_player_record_construct(this + 0x1eb);
    comp_oldest_player_record_construct(this + 0x22a);
    this
}

/// Builds an unbeaten run record: its two dates (+4, +0xc); returns the record.
/// Original: `comp_unbeaten_run_record_construct` (0x49f2a0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_unbeaten_run_record_construct(this: usize) -> usize {
    comp_records_date(this + 4);
    comp_records_date(this + 0xc);
    this
}

/// Builds a winless run record: its two dates (+4, +0xc); returns the record.
/// Original: `comp_winless_run_record_construct` (0x49f2f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_winless_run_record_construct(this: usize) -> usize {
    comp_records_date(this + 4);
    comp_records_date(this + 0xc);
    this
}

/// Builds a goals in a match record: its two dates (+0x11, +0x19); returns the record.
/// Original: `comp_goals_in_match_record_construct` (0x49f340, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_goals_in_match_record_construct(this: usize) -> usize {
    comp_records_date(this + 0x11);
    comp_records_date(this + 0x19);
    this
}

/// Builds a youngest player record: its three dates (+0, +0x18, +0x20); returns the record.
/// Original: `comp_youngest_player_record_construct` (0x49f390, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_youngest_player_record_construct(this: usize) -> usize {
    comp_records_date(this);
    comp_records_date(this + 0x18);
    comp_records_date(this + 0x20);
    this
}

/// Builds an oldest player record: its three dates (+0, +0x18, +0x20); returns the record.
/// Original: `comp_oldest_player_record_construct` (0x49f3f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_oldest_player_record_construct(this: usize) -> usize {
    comp_records_date(this);
    comp_records_date(this + 0x18);
    comp_records_date(this + 0x20);
    this
}

/// Copies a tally record: the byte at +0, the record holder at +1 (`record_holder_assign`), the dword at +0x19 and the
/// word at +0x1d; returns the destination.
/// Original: `comp_tally_record_assign` (0x4a1f40, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_tally_record_assign(this: usize, src: usize) -> usize {
    write_u8(this, read_u8(src));
    exe::record_holder_assign(this + 1, src + 1);
    write_u32(this + 0x19, read_u32(src + 0x19));
    write_u16(this + 0x1d, read_u16(src + 0x1d));
    this
}

/// Copies a discipline record: the bytes at +0 and +1, the record holder at +2, the dword at +0x1a and the word at
/// +0x1e; returns the destination.
/// Original: `comp_discipline_record_assign` (0x4a1f70, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_discipline_record_assign(this: usize, src: usize) -> usize {
    write_u8(this, read_u8(src));
    write_u8(this + 1, read_u8(src + 1));
    exe::record_holder_assign(this + 2, src + 2);
    write_u32(this + 0x1a, read_u32(src + 0x1a));
    write_u16(this + 0x1e, read_u16(src + 0x1e));
    this
}

/// Copies a rating record: the dword at +0, the record holder at +4, the byte at +0x1c, the dword at +0x1d and the
/// word at +0x21; returns the destination.
/// Original: `comp_rating_record_assign` (0x4a2050, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_rating_record_assign(this: usize, src: usize) -> usize {
    write_u32(this, read_u32(src));
    exe::record_holder_assign(this + 4, src + 4);
    write_u8(this + 0x1c, read_u8(src + 0x1c));
    write_u32(this + 0x1d, read_u32(src + 0x1d));
    write_u16(this + 0x21, read_u16(src + 0x21));
    this
}

/// Comparer for `comp_screens_open_comp_screen`.
pub(crate) unsafe extern "C" fn compare_comp_screens_open_comp_screen(comp: usize, second: usize, third: usize, restore: u32) {
    original_fn::<unsafe extern "C" fn(usize, usize, usize, u32)>(IDX_COMP_SCREENS_OPEN_COMP_SCREEN)(comp, second, third, restore);
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node == 0 || read_usize(node) != FN_COMP_SCREENS_DRAW_COMP_SCREEN {
        return;
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(comp) * 4);
    let got = [get_screen_param(0), get_screen_param(2), get_screen_param(3), get_screen_param(0x2d), get_screen_param(0x36)];
    let want = [object, second, third, 1, usize::MAX];
    SCOREBOARD.record(got == want, || format!("comp_screens_open_comp_screen (competition screen): params original={:?} rust={:?}", got, want));
}

/// Comparer for `comp_season_records_comp_season_records`: side by side over the 0x252 bytes it builds.
pub(crate) unsafe extern "thiscall" fn compare_comp_season_records_comp_season_records(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COMP_SEASON_RECORDS_COMP_SEASON_RECORDS);
    comp_records_compare("Comp_Season_Records::Comp_Season_Records", this, 0x252, || o(this), || comp_season_records_comp_season_records(this))
}

/// Comparer for `comp_unbeaten_run_record_construct`: side by side over its two dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_unbeaten_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COMP_UNBEATEN_RUN_RECORD_CONSTRUCT);
    comp_records_compare("comp_unbeaten_run_record_construct", this, 0x14, || o(this), || comp_unbeaten_run_record_construct(this))
}

/// Comparer for `comp_winless_run_record_construct`: side by side over its two dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_winless_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COMP_WINLESS_RUN_RECORD_CONSTRUCT);
    comp_records_compare("comp_winless_run_record_construct", this, 0x14, || o(this), || comp_winless_run_record_construct(this))
}

/// Comparer for `comp_goals_in_match_record_construct`: side by side over its two dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_goals_in_match_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COMP_GOALS_IN_MATCH_RECORD_CONSTRUCT);
    comp_records_compare("comp_goals_in_match_record_construct", this, 0x21, || o(this), || comp_goals_in_match_record_construct(this))
}

/// Comparer for `comp_youngest_player_record_construct`: side by side over its three dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_youngest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COMP_YOUNGEST_PLAYER_RECORD_CONSTRUCT);
    comp_records_compare("comp_youngest_player_record_construct", this, 0x28, || o(this), || comp_youngest_player_record_construct(this))
}

/// Comparer for `comp_oldest_player_record_construct`: side by side over its three dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_oldest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COMP_OLDEST_PLAYER_RECORD_CONSTRUCT);
    comp_records_compare("comp_oldest_player_record_construct", this, 0x28, || o(this), || comp_oldest_player_record_construct(this))
}

/// Comparer for `comp_tally_record_assign`: side by side over the 0x1f bytes copied.
pub(crate) unsafe extern "thiscall" fn compare_comp_tally_record_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_TALLY_RECORD_ASSIGN);
    comp_records_compare("comp_tally_record_assign", this, 0x1f, || o(this, src), || comp_tally_record_assign(this, src))
}

/// Comparer for `comp_discipline_record_assign`: side by side over the 0x20 bytes copied.
pub(crate) unsafe extern "thiscall" fn compare_comp_discipline_record_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_DISCIPLINE_RECORD_ASSIGN);
    comp_records_compare("comp_discipline_record_assign", this, 0x20, || o(this, src), || comp_discipline_record_assign(this, src))
}

/// Comparer for `comp_rating_record_assign`: side by side over the 0x23 bytes copied.
pub(crate) unsafe extern "thiscall" fn compare_comp_rating_record_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_RATING_RECORD_ASSIGN);
    comp_records_compare("comp_rating_record_assign", this, 0x23, || o(this, src), || comp_rating_record_assign(this, src))
}

/// Saves the four runs of every team entry before the team table is freed: the old saved array is freed; with teams,
/// a new array of one 0x1c-byte record per team (`malloc`; failing, the error box of comp_stats.CPP line 0x5f7 and the
/// capacity cleared) takes each team's club id and the twelve words of its runs (`COMP_STATS_RUN_WORDS`: each run's
/// length and its start date), then is sorted by club id (`qsort`).
/// Original: `Comp_Stats::save_team_runs` (0x4a2090, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_save_team_runs(this: usize) {
    use comp_stats_runs as r;
    if read_usize(this + r::SAVED) != 0 {
        exe::crt_free(read_usize(this + r::SAVED));
        write_usize(this + r::SAVED, 0);
        write_u32(this + r::SAVED_CAPACITY, 0);
        write_u32(this + r::SAVED_COUNT, 0);
    }
    let teams = read_i32(this + r::TEAM_COUNT);
    if teams == 0 {
        return;
    }
    write_i32(this + r::SAVED_CAPACITY, teams);
    write_u32(this + r::SAVED_COUNT, 0);
    let saved = exe::crt_malloc((teams as u32).wrapping_mul(r::SAVED_SIZE as u32));
    write_usize(this + r::SAVED, saved);
    if saved == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x5f7);
        write_u32(this + r::SAVED_CAPACITY, 0);
        return;
    }
    covered!("tel30_11 team runs saved");
    let mut k = 0;
    while k < read_i32(this + r::TEAM_COUNT) {
        let team = read_usize(this + r::TEAMS) + k as usize * r::TEAM_SIZE;
        let rec = read_usize(this + r::SAVED) + read_u32(this + r::SAVED_COUNT) as usize * r::SAVED_SIZE;
        write_u32(rec, read_u32(team));
        for (i, &at) in COMP_STATS_RUN_WORDS.iter().enumerate() {
            write_u16(rec + 4 + i * 2, read_u16(team + at));
        }
        write_u32(this + r::SAVED_COUNT, read_u32(this + r::SAVED_COUNT) + 1);
        k += 1;
    }
    write_u32(this + r::SAVED_CAPACITY, read_u32(this + r::SAVED_COUNT));
    exe::crt_qsort(read_usize(this + r::SAVED), read_u32(this + r::SAVED_COUNT), r::SAVED_SIZE as u32, COMP_STATS_BY_ID);
}

/// Gives a new team entry back the runs saved for its club: the saved run of its club id (`bsearch`), when there is
/// one, sets each run's length (+0x19, +0x2d, +0x41, +0x55: the saved word, signed) and start date (+0x1d, +0x31,
/// +0x45, +0x59: `Date::Date(day, year)`) and puts the current date at +0x25, +0x39, +0x4d and +0x61; the saved count
/// goes down and the array is freed when it reaches 0.
/// Original: `Comp_Stats::restore_team_runs` (0x4a22f0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_restore_team_runs(this: usize, team: usize) {
    use comp_stats_runs as r;
    let saved = read_usize(this + r::SAVED);
    if saved == 0 {
        return;
    }
    let key = read_u32(team);
    let rec = exe::crt_bsearch(&key as *const u32 as usize, saved, read_u32(this + r::SAVED_COUNT), r::SAVED_SIZE as u32, COMP_STATS_BY_ID);
    if rec == 0 {
        return;
    }
    covered!("tel30_11 team runs restored");
    let (mut date, mut scratch) = ([0u32; 2], [0u32; 2]);
    for (k, at) in [0x19usize, 0x2d, 0x41, 0x55].into_iter().enumerate() {
        write_i32(team + at, read_i16(rec + 4 + k * 6) as i32);
        let made = exe::new_date(date.as_mut_ptr() as usize, read_u16(rec + 6 + k * 6) as u32, read_u16(rec + 8 + k * 6) as u32);
        exe::date_operator_3d(team + at + 4, scratch.as_mut_ptr() as usize, made);
        exe::date_operator_3d(team + at + 0xc, scratch.as_mut_ptr() as usize, G_CURRENT_DATE);
    }
    let left = read_i32(this + r::SAVED_COUNT) - 1;
    write_i32(this + r::SAVED_COUNT, left);
    if left == 0 {
        exe::crt_free(read_usize(this + r::SAVED));
        write_usize(this + r::SAVED, 0);
        write_u32(this + r::SAVED_CAPACITY, 0);
    }
}

/// Adds a played match's player figures to the competition's totals; returns 1, or 0 for a match already counted
/// (byte +0x4c) and for a null match (with the error box of comp_stats.CPP line 0x707).
///
/// The match count (+0x5ca) goes up. For each side (two 0x625-byte blocks) and each of its 20 player rows (0x49 bytes
/// from +0x190 of the match) with both ids set (dwords +0 and +4 not -1) and, from the 12th row on, the byte +0x32 not
/// -1: the unsigned bytes +0x3e..+0x43 go to the totals +0x56a..+0x57e, +0x25 / +0x26 / +0x21 to +0x582 / +0x586 /
/// +0x58a and to the side's own (+0x58e.. for the first side, +0x59a.. for the second), +0x27 / +0x28 / +0x47 / +0x2b /
/// +0x29 / +0x2a / +0x35 / +0x2c / +0x2d to +0x5a6..+0x5c6.
/// Original: `Comp_Stats::accumulate_match_player_totals` (0x4a25f0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_accumulate_match_player_totals(this: usize, fixture: usize) -> u32 {
    if fixture == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x707);
        return 0;
    }
    if read_u8(fixture + 0x4c) != 0 {
        return 0;
    }
    let add = |at: usize, v: u8| write_u32(this + at, read_u32(this + at).wrapping_add(v as u32));
    write_u32(this + 0x5ca, read_u32(this + 0x5ca).wrapping_add(1));
    for side in 0..2usize {
        for k in 0..0x14usize {
            let row = fixture + 0x190 + side * 0x625 + k * 0x49;
            if read_i32(row) == -1 || read_i32(row + 4) == -1 || (k >= 0xb && read_i8(row + 0x32) == -1) {
                continue;
            }
            for (i, at) in (0x3e..=0x43).enumerate() {
                add(0x56a + i * 4, read_u8(row + at));
            }
            add(0x582, read_u8(row + 0x25));
            add(0x586, read_u8(row + 0x26));
            add(0x58a, read_u8(row + 0x21));
            let own = if side == 0 { 0x58e } else { 0x59a };
            add(own, read_u8(row + 0x25));
            add(own + 4, read_u8(row + 0x26));
            add(own + 8, read_u8(row + 0x21));
            for (i, at) in [0x27usize, 0x28, 0x47, 0x2b, 0x29, 0x2a, 0x35, 0x2c, 0x2d].into_iter().enumerate() {
                add(0x5a6 + i * 4, read_u8(row + at));
            }
        }
    }
    1
}

/// The name of team statistic `code` (1 "Games" .. 17 "Red Cards", `COMP_STATS_TEAM_LABELS`) into `out` (at least
/// 0x33 bytes; else the box of line 0x7b5); an unknown code gives the box of line 0x838 and 0.
/// Original: `comp_stats_get_team_stat_label` (0x4a2810, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_get_team_stat_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_label(code, out, size, 0x33, &COMP_STATS_TEAM_LABELS, 0x7b5, 0x838)
}

/// The name of player statistic `code` (1 "Appearances" .. 17 "Average Rating", `COMP_STATS_PLAYER_LABELS`) into
/// `out` (at least 0x33 bytes; else the box of line 0x84b); an unknown code gives the box of line 0x8ce and 0.
/// Original: `comp_stats_get_player_stat_label` (0x4a2b10, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_get_player_stat_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_label(code, out, size, 0x33, &COMP_STATS_PLAYER_LABELS, 0x84b, 0x8ce)
}

/// The value of team statistic `code` of a team entry, as the x87 float the original leaves in st0.
///
/// 1 the games (byte +4); 2 the average attendance (+6 / byte +5, 0.0 with no game); 3 / 4 / 5 the sums of the five
/// signed form bytes from +0xa / +0xf / +0x14; 6..9 the runs (+0x19, +0x2d, +0x41, +0x55); 10..12 the bytes +0x69..+0x6b;
/// 13..19 the bytes +0x6d..+0x73. A null entry (line 0x901) or an unknown code (line 0x9a1) gives the error box and 0.0.
/// Original: `comp_stats_get_team_entry_value` (0x4a2ec0, comp_screens.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn comp_stats_get_team_entry_value(entry: usize, code: i8) -> f64 {
    let zero = read_f32(rdata::F32_0) as f64;
    if entry == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x901);
        return zero;
    }
    let form = |at: usize| (0..5).map(|k| read_i8(entry + at + k) as i32).sum::<i32>() as f64;
    match code {
        1 => read_u8(entry + 4) as f64,
        2 => {
            let games = read_u8(entry + 5);
            if games == 0 { zero } else { read_i32(entry + 6) as f64 / games as f64 }
        }
        3 => form(0xa),
        4 => form(0xf),
        5 => form(0x14),
        6 => read_i32(entry + 0x19) as f64,
        7 => read_i32(entry + 0x2d) as f64,
        8 => read_i32(entry + 0x41) as f64,
        9 => read_i32(entry + 0x55) as f64,
        10..=12 => read_u8(entry + 0x69 + (code as usize - 10)) as f64,
        13..=19 => read_u8(entry + 0x6d + (code as usize - 13)) as f64,
        _ => {
            tel26_error_box(COMP_STATS_SOURCE_FILE, 0x9a1);
            zero
        }
    }
}

/// The value of player statistic `code` of a player entry, as the x87 float the original leaves in st0.
///
/// 1..12 the bytes +0xc, +0xe, +0xf, +0x11..+0x19. The rates need the player's minutes (+0x29) to reach 35 × the
/// club's games (`comp_stats_club_games`): 13 pass completion (+0x1f / +0x1d × 100, with more than 20 passes and, up
/// to 500, more than a quarter of the minutes); 14 / 15 tackles / dribbles per game (+0x21 / +0x23 over the
/// appearances +0xc, with more than 0x95 minutes); 16 shots on target (+0x27 / +0x25 × 100, with more than 24 shots,
/// or more than 4 at under 46 minutes each, or more than 10 with more than 0x21c minutes); 17 the average rating
/// (+0x1a / the appearances, with appearances (statistic 1) of at least half the club's games, the games kept as a
/// float, and at least one); 18 +0xf / +0xc (with appearances). A rate short of its condition is 0.0; a null entry
/// (line 0x9d2) or an unknown code (line 0xa7f) gives the error box and 0.0.
/// Original: `Comp_Stats::get_player_entry_value` (0x4a3250, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_player_entry_value(this: usize, entry: usize, code: i8) -> f64 {
    let zero = read_f32(rdata::F32_0) as f64;
    if entry == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x9d2);
        return zero;
    }
    let k35 = read_f32(COMP_STATS_F32_35) as f64;
    let hundred = read_f32(rdata::F32_100) as f64;
    let apps = read_u8(entry + 0xc);
    match code {
        1 => apps as f64,
        2 => read_u8(entry + 0xe) as f64,
        3 => read_u8(entry + 0xf) as f64,
        4..=12 => read_u8(entry + 0x11 + (code as usize - 4)) as f64,
        13 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 {
                return zero;
            }
            let passes = read_i16(entry + 0x1d);
            if passes <= 0x14 || (passes <= 0x1f4 && passes as i32 <= minutes / 4) {
                return zero;
            }
            read_i16(entry + 0x1f) as f64 / passes as f64 * hundred
        }
        14 | 15 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 || minutes <= 0x95 {
                return zero;
            }
            read_i16(entry + if code == 14 { 0x21 } else { 0x23 }) as f64 / apps as f64
        }
        16 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 {
                return zero;
            }
            let shots = read_i16(entry + 0x25);
            let enough = shots > 0x18 || (shots > 4 && minutes / shots as i32 <= 0x2d) || (minutes > 0x21c && shots > 10);
            if !enough {
                return zero;
            }
            read_i16(entry + 0x27) as f64 / shots as f64 * hundred
        }
        17 => {
            let games = comp_stats_club_games(this, entry) as f32 as f64;
            let played = comp_stats_get_player_entry_value(this, entry, 1);
            if games * read_f64(rdata::F64_0_5) > played || apps == 0 {
                return zero;
            }
            read_i16(entry + 0x1a) as f64 / apps as f64
        }
        18 => {
            if apps == 0 {
                return zero;
            }
            read_u8(entry + 0xf) as f64 / apps as f64
        }
        _ => {
            tel26_error_box(COMP_STATS_SOURCE_FILE, 0xa7f);
            zero
        }
    }
}

/// The name of record period `code` (1 "All Time", 2 "Current") into `out` (at least 0x32 bytes; else the box of line
/// 0xdf0); another code gives the box of line 0xe0a and 0.
/// Original: `comp_stats_get_record_period_label` (0x4a5010, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_get_record_period_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_label(code, out, size, 0x32, &[0x99ccd0, 0x99ccdc], 0xdf0, 0xe0a)
}

/// Whether record type `kind` is shown for the competition (+0x502, its first entry at +4): never for a friendly
/// competition (`Friendly::is_friendly_competition`); type 2 only for a league division (`comp_util_is_league_division`
/// of the first entry's id); type 8 not for a competition of kind 4 (+0x42); the rest always.
/// Original: `Comp_Stats::is_record_type_shown` (0x4a54c0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_is_record_type_shown(this: usize, kind: i8) -> u32 {
    let comp = read_usize(this + 0x502);
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(comp + 4)) != 0 {
        return 0;
    }
    match kind {
        2 => (exe::comp_util_is_league_division(read_i32(read_usize(read_usize(this + 0x502) + 4))) != 0) as u32,
        8 if read_u8(read_usize(this + 0x502) + 0x42) == 4 => 0,
        _ => 1,
    }
}

/// Comparer for `comp_stats_save_team_runs`: the original only (it allocates the array), then the plan: one saved run
/// per team, sorted by club id.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_save_team_runs(this: usize) {
    use comp_stats_runs as r;
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_COMP_STATS_SAVE_TEAM_RUNS)(this);
    let (saved, count) = (read_usize(this + r::SAVED), read_i32(this + r::SAVED_COUNT));
    if saved == 0 {
        return;
    }
    let ids: Vec<i32> = (0..count.max(0) as usize).map(|k| read_i32(saved + k * r::SAVED_SIZE)).collect();
    let ok = count == read_i32(this + r::TEAM_COUNT) && ids.windows(2).all(|w| w[0] <= w[1]);
    SCOREBOARD.record(ok, || format!("Comp_Stats::save_team_runs({:#x}): {} saved for {} teams, ids {:?}", this, count, read_i32(this + r::TEAM_COUNT), ids));
}

/// Comparer for `comp_stats_restore_team_runs`: the original only (it may free the array), then the plan: the four run
/// lengths taken from the saved run of the club.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_restore_team_runs(this: usize, team: usize) {
    use comp_stats_runs as r;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_COMP_STATS_RESTORE_TEAM_RUNS);
    let saved = read_usize(this + r::SAVED);
    let key = read_u32(team);
    let rec = if saved == 0 { 0 } else {
        exe::crt_bsearch(&key as *const u32 as usize, saved, read_u32(this + r::SAVED_COUNT), r::SAVED_SIZE as u32, COMP_STATS_BY_ID)
    };
    let want: Option<[i32; 4]> = if rec == 0 { None } else { Some([0usize, 1, 2, 3].map(|k| read_i16(rec + 4 + k * 6) as i32)) };
    o(this, team);
    if let Some(want) = want {
        let got = [0x19usize, 0x2d, 0x41, 0x55].map(|at| read_i32(team + at));
        SCOREBOARD.record(got == want, || format!("Comp_Stats::restore_team_runs: runs original={:?} rust={:?}", got, want));
    }
}

/// Comparer for `comp_stats_accumulate_match_player_totals`: side by side over the totals (+0x56a..+0x5ce) and the
/// answer; a null match (the error box) the original only.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_accumulate_match_player_totals(this: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_COMP_STATS_ACCUMULATE_MATCH_PLAYER_TOTALS);
    if fixture == 0 {
        return o(this, fixture);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this + 0x56a, 0x64)], || o(this, fixture),
        || comp_stats_accumulate_match_player_totals(this, fixture));
    SCOREBOARD.record(same && r == n, || format!("Comp_Stats::accumulate_match_player_totals: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `comp_stats_get_team_stat_label`.
pub(crate) unsafe extern "C" fn compare_comp_stats_get_team_stat_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_compare_label("comp_stats_get_team_stat_label", code, out, size, 0x33, 17, original_fn(IDX_COMP_STATS_GET_TEAM_STAT_LABEL), comp_stats_get_team_stat_label)
}

/// Comparer for `comp_stats_get_player_stat_label`.
pub(crate) unsafe extern "C" fn compare_comp_stats_get_player_stat_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_compare_label("comp_stats_get_player_stat_label", code, out, size, 0x33, 17, original_fn(IDX_COMP_STATS_GET_PLAYER_STAT_LABEL), comp_stats_get_player_stat_label)
}

/// Comparer for `comp_stats_get_team_entry_value`: the value side by side (its bits); the error cases the original only.
pub(crate) unsafe extern "stdcall" fn compare_comp_stats_get_team_entry_value(entry: usize, code: i8) -> f64 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, i8) -> f64>(IDX_COMP_STATS_GET_TEAM_ENTRY_VALUE);
    let r = o(entry, code);
    if entry != 0 && (1..=19).contains(&code) {
        let n = comp_stats_get_team_entry_value(entry, code);
        SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("comp_stats_get_team_entry_value({}): original={} rust={}", code, r, n));
    }
    r
}

/// Comparer for `comp_stats_get_player_entry_value`: the value side by side (its bits); the error cases the original only.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_player_entry_value(this: usize, entry: usize, code: i8) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> f64>(IDX_COMP_STATS_GET_PLAYER_ENTRY_VALUE);
    let r = o(this, entry, code);
    if entry != 0 && (1..=18).contains(&code) {
        let n = comp_stats_get_player_entry_value(this, entry, code);
        SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("Comp_Stats::get_player_entry_value({}): original={} rust={}", code, r, n));
    }
    r
}

/// Comparer for `comp_stats_get_record_period_label`.
pub(crate) unsafe extern "C" fn compare_comp_stats_get_record_period_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_compare_label("comp_stats_get_record_period_label", code, out, size, 0x32, 2, original_fn(IDX_COMP_STATS_GET_RECORD_PERIOD_LABEL), comp_stats_get_record_period_label)
}

/// The text of team statistic `code` of a team entry into `out`; returns 1, or 0 with the error box (comp_stats.CPP)
/// for a null buffer or one under 0x33 bytes (line 0xac4), a null entry (line 0xacf, after the default text) and an
/// unknown code (line 0xb96, after the default text).
///
/// The buffer first takes the default text: "----" for codes 2..5, "0" for the rest. Then 1 the games (byte +4); 2 the
/// average attendance (+6 / the byte +5, "%ld"; with no game the "----" stays); 3 / 4 / 5 the form of the last five
/// results (`comp_stats_form_text` from +0xe / +0x13 / +0x18 down); 6..9 the runs (+0x19, +0x2d, +0x41, +0x55); 10 / 11
/// the bytes +0x69 / +0x6a; 12 the byte +0x6b, with the byte +0x6c in brackets ("%d (%d)") when both it and
/// `with_detail` are set; 13..17 the bytes +0x6d..+0x71. The object in ecx is not read.
/// Original: `comp_stats_format_team_stat` (0x4a39c0, comp_screens.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn comp_stats_format_team_stat(_this: usize, entry: usize, code: i8, out: usize, size: i32, with_detail: u32) -> u32 {
    if out == 0 || size < 0x33 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xac4);
        return 0;
    }
    copy_text(out as *mut u8, (if (2..=5).contains(&code) { COMP_STATS_DASHES_4 } else { COMP_STATS_ZERO_TEXT }) as *const u8);
    if entry == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xacf);
        return 0;
    }
    let sprintf = comp_stats_sprintf();
    let byte = |at: usize| read_u8(entry + at) as i32;
    match code {
        1 => {
            sprintf(out, FORMAT_D, byte(4));
        }
        2 => {
            let games = byte(5);
            if games != 0 {
                sprintf(out, COMP_STATS_FORMAT_LD, read_i32(entry + 6) / games);
            }
        }
        3..=5 => comp_stats_form_text(out, entry + [0xe, 0x13, 0x18][code as usize - 3]),
        6..=9 => {
            sprintf(out, FORMAT_D, read_i32(entry + 0x19 + (code as usize - 6) * 0x14));
        }
        10 | 11 => {
            sprintf(out, FORMAT_D, byte(0x69 + (code as usize - 10)));
        }
        12 => {
            let x = byte(0x6b);
            if x != 0 && with_detail != 0 {
                covered!("tel30_12 team stat with detail");
                sprintf(out, COMP_STATS_FORMAT_WITH_DETAIL, x, byte(0x6c));
            } else {
                sprintf(out, FORMAT_D, x);
            }
        }
        13..=17 => {
            sprintf(out, FORMAT_D, byte(0x6d + (code as usize - 13)));
        }
        _ => {
            tel26_error_box(COMP_STATS_SOURCE_FILE, 0xb96);
            return 0;
        }
    }
    1
}

/// The text of player statistic `code` of a player entry into `out`; returns 1, or 0 with the error box
/// (comp_stats.CPP) for a null buffer or one under 0x33 bytes (line 0xbe5), a null entry (line 0xbfd, after the default
/// text) and an unknown code (line 0xcc7, after the default text).
///
/// The default text: "---" for 13 and 16, "----" for 14, 15 and 17, "0" for the rest; a rate short of its condition
/// leaves it. 1 the appearances (byte +0xc), as the starts and the substitute appearances in brackets ("%d (%d)": +0xc
/// minus +0xd, +0xd) when +0xd and `with_detail` are set; 2 / 3 the bytes +0xe / +0xf; 4 the byte +0x10, with +0x11 in
/// brackets when +0x10 and `with_detail` are set; 5..12 the bytes +0x12..+0x19. The rates need the minutes (+0x29) to
/// reach 35 × the club's games (`comp_stats_club_games`, x87: the games × 35.0 compared with the minutes): 13 the pass
/// completion (+0x1f × 100 / +0x1d passes, truncated, "%d%%"; more than 20 passes and, up to 500, more than a quarter
/// of the minutes); 14 / 15 the tackles / dribbles per game (+0x21 / +0x23 over the appearances, "%3.1f"; at least
/// 0x96 minutes); 16 the shots on target (+0x27 × 100 / +0x25 shots, truncated, "%d%%"; more than 24 shots, or more
/// than 4 at under 46 minutes each, or more than 10 with more than 0x21c minutes); 17 the average rating (+0x1a / the
/// appearances, "%4.2f"; the club's games rounded to an f32, × 0.5, not above the appearances (player statistic 1),
/// and at least one appearance).
/// Original: `Comp_Stats::format_player_stat` (0x4a3ee0, comp_screens.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn comp_stats_format_player_stat(this: usize, entry: usize, code: i8, out: usize, size: i32, with_detail: u32) -> u32 {
    if out == 0 || size < 0x33 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xbe5);
        return 0;
    }
    let default = match code {
        13 | 16 => COMP_STATS_DASHES_3,
        14 | 15 | 17 => COMP_STATS_DASHES_4,
        _ => COMP_STATS_ZERO_TEXT,
    };
    copy_text(out as *mut u8, default as *const u8);
    if entry == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xbfd);
        return 0;
    }
    let sprintf = comp_stats_sprintf();
    let byte = |at: usize| read_u8(entry + at) as i32;
    let k35 = read_f32(COMP_STATS_F32_35) as f64;
    let hundred = read_f32(rdata::F32_100) as f64;
    match code {
        1 => {
            let subs = byte(0xd);
            if subs != 0 && with_detail != 0 {
                covered!("tel30_12 appearances with substitutes");
                sprintf(out, COMP_STATS_FORMAT_WITH_DETAIL, byte(0xc) - subs, subs);
            } else {
                sprintf(out, FORMAT_D, byte(0xc));
            }
        }
        2 | 3 => {
            sprintf(out, FORMAT_D, byte(0xe + (code as usize - 2)));
        }
        4 => {
            let x = byte(0x10);
            if x != 0 && with_detail != 0 {
                sprintf(out, COMP_STATS_FORMAT_WITH_DETAIL, x, byte(0x11));
            } else {
                sprintf(out, FORMAT_D, x);
            }
        }
        5..=12 => {
            sprintf(out, FORMAT_D, byte(0x12 + (code as usize - 5)));
        }
        13 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 {
                return 1;
            }
            let passes = read_i16(entry + 0x1d);
            if passes <= 0x14 || (passes <= 0x1f4 && passes as i32 <= minutes / 4) {
                return 1;
            }
            sprintf(out, FORMAT_PCT, ftol(read_i16(entry + 0x1f) as f64 * hundred / passes as f64));
        }
        14 | 15 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 || minutes < 0x96 {
                return 1;
            }
            let per_game = read_i16(entry + if code == 14 { 0x21 } else { 0x23 }) as f64 / byte(0xc) as f64;
            sprintf(out, COMP_STATS_FORMAT_1_DECIMAL, per_game);
        }
        16 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 {
                return 1;
            }
            let shots = read_i16(entry + 0x25);
            let enough = shots > 0x18 || (shots > 4 && minutes / shots as i32 <= 0x2d) || (minutes > 0x21c && shots > 10);
            if !enough {
                return 1;
            }
            sprintf(out, FORMAT_PCT, ftol(read_i16(entry + 0x27) as f64 * hundred / shots as f64));
        }
        17 => {
            let games = comp_stats_club_games(this, entry) as f32 as f64;
            let played = comp_stats_get_player_entry_value(this, entry, 1);
            let apps = byte(0xc);
            if games * read_f64(rdata::F64_0_5) > played || apps == 0 {
                return 1;
            }
            sprintf(out, COMP_STATS_FORMAT_2_DECIMALS, read_i16(entry + 0x1a) as f64 / apps as f64);
        }
        _ => {
            tel26_error_box(COMP_STATS_SOURCE_FILE, 0xcc7);
            return 0;
        }
    }
    1
}

/// The title of record type `kind` (1 "Most Times Winner" .. 22 "Oldest Player", `COMP_STATS_RECORD_TITLES`) into `out`
/// (at least 0x32 bytes; else the box of line 0xe1d); an unknown type gives the box of line 0xec3 and 0. The object
/// in ecx is not read.
/// Original: `comp_stats_get_record_title` (0x4a5120, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_record_title(_this: usize, kind: i8, out: usize, size: i32) -> u32 {
    comp_stats_label(kind, out, size, 0x32, &COMP_STATS_RECORD_TITLES, 0xe1d, 0xec3)
}

/// The text of record type `kind` (its low byte) for `view` (1 the all-time records, else the season's) into the
/// Text object `text`; returns what `comp_stats_format_record_text` returns, or 0 with the error box of line 0xf08 for
/// a null Text.
///
/// The Text is cleared and set to "-". For the season records, two are filled in first: type 8 (the highest average
/// attendance), when still unset (+0x6b = -1) and with teams, from the first team after sorting the team table by
/// average attendance (`qsort` with 0x4ad1a0, the table's sort mark +0x18 set to 2, skipped when it already is): the
/// value (team statistic 2, truncated) at +0x6b, its club id at +0x6f and 0 at the word +0x73; type 18 (the highest
/// average rating), when its rating (the f32 +0x189) is still -1.0, by `Comp_Stats::update_season_rating_record`.
/// Original: `Comp_Stats::get_record_text` (0x4a5530, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_record_text(this: usize, view: u32, kind: u32, text: usize) -> u32 {
    use comp_stats_runs as r;
    if text == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xf08);
        return 0;
    }
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_TEXT_CLEAR); // Text::clear
    clear(text);
    exe::text_set_string(text, 0x98ba0c as *const u8);
    let rec = this + if view as u8 == 1 { COMP_STATS_ALL_TIME_RECORDS } else { COMP_STATS_SEASON_RECORDS };
    if view as u8 != 1 {
        if kind as u8 == 8 {
            if read_i32(rec + 0x6b) == -1 && read_usize(this + r::TEAMS) != 0 {
                covered!("tel30_12 season attendance record filled");
                if read_u8(this + 0x18) != 2 {
                    write_u8(this + 0x18, 2);
                    write_usize(G_COMP_STATS_SORT_CONTEXT, this);
                    exe::crt_qsort(read_usize(this + r::TEAMS), read_u32(this + r::TEAM_COUNT), r::TEAM_SIZE as u32, 0x4ad1a0usize);
                }
                let first = read_usize(this + r::TEAMS);
                write_i32(rec + 0x6b, ftol(comp_stats_get_team_entry_value(first, 2)));
                write_u32(rec + 0x6f, read_u32(first));
                write_u16(rec + 0x73, 0);
            }
        } else if kind as u8 == 0x12 {
            let rating = read_f32(rec + 0x189) as f64;
            // x87 fcomp with test ah, 0x40: equal or unordered
            if rating == read_f64(rdata::F64_NEG_1) || rating.is_nan() {
                let update: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_STATS_UPDATE_SEASON_RATING_RECORD); // update_season_rating_record
                update(this);
            }
        }
    }
    let format: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(FN_COMP_STATS_FORMAT_RECORD_TEXT); // format_record_text
    format(this, rec, kind, text)
}

/// Comparer for `comp_stats_format_team_stat` (the competition screen's team statistics table).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_format_team_stat(this: usize, entry: usize, code: i8, out: usize, size: i32, with_detail: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, usize, i32, u32) -> u32>(IDX_COMP_STATS_FORMAT_TEAM_STAT);
    comp_stats_compare_stat_text("comp_stats_format_team_stat", code, out, size, entry != 0 && (1..=17).contains(&code),
        || o(this, entry, code, out, size, with_detail), || comp_stats_format_team_stat(this, entry, code, out, size, with_detail))
}

/// Comparer for `comp_stats_format_player_stat` (the competition screen's player statistics table).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_format_player_stat(this: usize, entry: usize, code: i8, out: usize, size: i32, with_detail: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, usize, i32, u32) -> u32>(IDX_COMP_STATS_FORMAT_PLAYER_STAT);
    comp_stats_compare_stat_text("Comp_Stats::format_player_stat", code, out, size, entry != 0 && (1..=17).contains(&code),
        || o(this, entry, code, out, size, with_detail), || comp_stats_format_player_stat(this, entry, code, out, size, with_detail))
}

/// Comparer for `comp_stats_get_record_title`: side by side over the buffer (0x32 bytes) and the answer; the error
/// cases the original only.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_record_title(this: usize, kind: i8, out: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, i32) -> u32>(IDX_COMP_STATS_GET_RECORD_TITLE);
    if out == 0 || size < 0x32 || !(1..=22).contains(&kind) {
        return o(this, kind, out, size);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, 0x32)], || o(this, kind, out, size), || comp_stats_get_record_title(this, kind, out, size));
    SCOREBOARD.record(same && r == n, || format!("comp_stats_get_record_title({}): differences {:?}", kind, first_differences(&mo, &mr)));
    r
}

/// Comparer for `comp_stats_get_record_text`: the original writes into the screen's Text, the copy into a local one
/// (`compare_text_writer`), and the answers must match; the copy runs after the original, so the season records it
/// fills in are already there (filling them again would change nothing). A null Text the original only.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_record_text(this: usize, view: u32, kind: u32, text: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32>(IDX_COMP_STATS_GET_RECORD_TEXT);
    if text == 0 {
        return o(this, view, kind, text);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(text, || ro.set(o(this, view, kind, text)), |t| rr.set(comp_stats_get_record_text(this, view, kind, t)),
        || format!("Comp_Stats::get_record_text(view {}, kind {})", view as u8, kind as u8));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("Comp_Stats::get_record_text(view {}, kind {}): original={} rust={}", view as u8, kind as u8, ro.get(), rr.get()));
    ro.get()
}

/// Rule: a run record of the competition statistics is copied whole: the run (game count and dates) and the dword
/// after it.
///
/// `comp_run_record_assign(source)`: `run_record_assign` (the game count, `run_record::START_DATE` and
/// `run_record::END_DATE`) and the dword at +0x14. Returns `this`.
/// Original: `comp_run_record_assign` (0x4ab5d0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_run_record_assign(this: usize, source: usize) -> usize {
    run_record_assign(this, source);
    write_u32(this + 0x14, read_u32(source + 0x14));
    this
}

/// Rule: the season records of a competition are copied field by field (the compiler's `operator=`).
///
/// `comp_season_records_assign(source)`, in order: the dword at +0 and the byte at +4, the eight dwords +5..+0x25,
/// the match records at +0x29 and +0x4c (with the dword +0x48 between them), the two dwords and the word at +0x6b,
/// the match records +0x75 and +0x94, the four run records +0xb3, +0xcb, +0xe3 and +0xfb (`comp_run_record_assign`),
/// the tally records at +0x113, +0x16a and +0x1ac (byte, record holder, dword, word), the goals-in-match record at
/// +0x132 (byte, holder, match record), the rating record at +0x189 (dword, holder, byte, dword, word), the
/// discipline record at +0x1cb (two bytes, holder, dword, word) and the two age records at +0x1eb and +0x22a
/// (`player_age_record_assign`, 0x4a1fb0). Returns `this`.
/// Original: `comp_season_records_assign` (0x4ab5f0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_season_records_assign(this: usize, source: usize) -> usize {
    let d = |o: usize| write_u32(this + o, read_u32(source + o));
    let w = |o: usize| write_u16(this + o, read_u16(source + o));
    let b = |o: usize| write_u8(this + o, read_u8(source + o));
    d(0);
    b(4);
    for o in [5, 9, 0xd, 0x11, 0x15, 0x19, 0x1d, 0x21, 0x25] {
        d(o);
    }
    match_record_assign(this + 0x29, source + 0x29);
    d(0x48);
    match_record_assign(this + 0x4c, source + 0x4c);
    d(0x6b);
    d(0x6f);
    w(0x73);
    match_record_assign(this + 0x75, source + 0x75);
    match_record_assign(this + 0x94, source + 0x94);
    for o in [0xb3, 0xcb, 0xe3, 0xfb] {
        comp_run_record_assign(this + o, source + o);
    }
    b(0x113);
    record_holder_assign(this + 0x114, source + 0x114);
    d(0x12c);
    w(0x130);
    b(0x132);
    record_holder_assign(this + 0x133, source + 0x133);
    match_record_assign(this + 0x14b, source + 0x14b);
    b(0x16a);
    record_holder_assign(this + 0x16b, source + 0x16b);
    d(0x183);
    w(0x187);
    d(0x189);
    record_holder_assign(this + 0x18d, source + 0x18d);
    b(0x1a5);
    d(0x1a6);
    w(0x1aa);
    b(0x1ac);
    record_holder_assign(this + 0x1ad, source + 0x1ad);
    d(0x1c5);
    w(0x1c9);
    b(0x1cb);
    b(0x1cc);
    record_holder_assign(this + 0x1cd, source + 0x1cd);
    d(0x1e5);
    w(0x1e9);
    exe::player_age_record_assign(this + 0x1eb, source + 0x1eb);
    exe::player_age_record_assign(this + 0x22a, source + 0x22a);
    this
}

/// Rule: the records screen lists the all-time records (year 0), the current season (the competition's year) and
/// then the archived seasons, newest first, at most 500 rows.
///
/// `Comp_Stats::build_season_records_list(&count)`: `count` = 0; a block of 500 rows of 0x26b bytes (`crt_malloc`;
/// on failure the comp_stats.CPP 5909 box and 0). Row 0: year 0 and the all-time records
/// (`comp_stats_obj::ALL_TIME_RECORDS`). Then, with no season record of points yet (`SEASON_MOST_POINTS` = -1) and
/// a club table, the club table sorted by points (statistic 2, qsort with `comp_stats_compare_team_entries` unless
/// already sorted by it) and the leader's points (`_ftol`), its club and 0 written as the season record; with no
/// season rating (-1.0) `Comp_Stats::update_season_rating_record`. Row 1: the competition's year (word +0x40 of the
/// competition) and the season records. Then the archived seasons of the file list (`file_llist_get_nodes` with the
/// two archive keys), from the tail backwards (`llist_get_prev`): the year word and the records of each, until 500
/// rows; the list is deleted (`LLIST::scalar_deleting_destructor` 0x497910 with 1). Returns the block.
/// Original: `Comp_Stats::build_season_records_list` (0x4ab0c0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_build_season_records_list(stats: usize, count: *mut u32) -> usize {
    *count = 0;
    let block = exe::crt_malloc(SEASON_RECORDS_ROW as u32 * SEASON_RECORDS_ROWS);
    if block == 0 {
        show_error_box(COMP_STATS_CPP, 0x1715);
        return 0;
    }
    let row = |n: u32| block + n as usize * SEASON_RECORDS_ROW;
    write_u16(row(*count), 0);
    comp_season_records_assign(row(*count) + 2, stats + comp_stats_obj::ALL_TIME_RECORDS);
    *count += 1;
    if read_i32(stats + comp_stats_obj::SEASON_MOST_POINTS) == -1 && read_usize(stats + comp_stats_obj::CLUBS) != 0 {
        covered!("tel31_1 season points record from the table");
        if read_u8(stats + comp_stats_obj::CLUB_SORT) != 2 {
            write_u8(stats + comp_stats_obj::CLUB_SORT, 2);
            write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
            exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
        }
        let leader = read_usize(stats + comp_stats_obj::CLUBS);
        write_i32(stats + comp_stats_obj::SEASON_MOST_POINTS, ftol(team_entry_value(stats, leader, 2)));
        write_u32(stats + comp_stats_obj::SEASON_MOST_POINTS_CLUB, read_u32(leader));
        write_u16(stats + comp_stats_obj::SEASON_MOST_POINTS_EXTRA, 0);
    }
    // fcomp sets C3 for equal and for unordered: a NaN rating also asks for the record
    let rating = read_f32(stats + comp_stats_obj::SEASON_RATING);
    if rating == -1.0 || rating.is_nan() {
        comp_stats_update_season_rating_record(stats);
    }
    write_u16(row(*count), read_u16(read_usize(stats + comp_stats_obj::COMP) + 0x40));
    comp_season_records_assign(row(*count) + 2, stats + comp_stats_obj::SEASON_RECORDS);
    *count += 1;
    let list = exe::file_llist_get_nodes(read_usize(COMP_STATS_ARCHIVE_LIST), read_u32(stats + comp_stats_obj::ARCHIVE_KEY1),
        read_u32(stats + comp_stats_obj::ARCHIVE_KEY2));
    if list != 0 {
        write_usize(list + llist::CURSOR, read_usize(list + llist::TAIL));
        let mut data = exe::llist_get_prev(list);
        while data != 0 && *count != SEASON_RECORDS_ROWS {
            covered!("tel31_1 archived season row");
            let r = row(*count);
            *count += 1;
            write_u16(r, read_u16(data));
            comp_season_records_assign(r + 2, data + 2);
            data = exe::llist_get_prev(list);
        }
        exe::llist_scalar_deleting_destructor(list, 1);
    }
    block
}

/// Rule: a new club entry of the competition statistics is zeroed with the club's id; the table grows two at a time.
///
/// `Comp_Stats::add_club_entry(club)`: no club: the comp_stats.CPP 6345 box and 0. A full table
/// (`comp_stats_obj::CLUB_COUNT` = capacity) grows by 2 (`crt_realloc` to capacity × 0x76; on failure the
/// comp_stats.CPP 6359 box, `game_fatal_cleanup` and `exit(-1)`). The entry after the last one is zeroed (0x76 bytes),
/// takes the club's id, the count goes up, the table is marked unsorted (0xfe) and `Comp_Stats::restore_team_runs`
/// (0x4a22f0) fills the entry's runs. Returns the entry.
/// Original: `Comp_Stats::add_club_entry` (0x4aceb0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_add_club_entry(stats: usize, club: usize) -> usize {
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0x18c9);
        return 0;
    }
    let capacity = read_u32(stats + comp_stats_obj::CLUB_CAPACITY);
    if read_u32(stats + comp_stats_obj::CLUB_COUNT) == capacity {
        covered!("tel31_1 club table grows");
        let capacity = capacity.wrapping_add(2);
        write_u32(stats + comp_stats_obj::CLUB_CAPACITY, capacity);
        let table = exe::crt_realloc(read_usize(stats + comp_stats_obj::CLUBS), capacity.wrapping_mul(comp_stats_obj::CLUB_ENTRY_SIZE as u32));
        write_usize(stats + comp_stats_obj::CLUBS, table);
        if table == 0 {
            comp_stats_fatal(0x18d7);
        }
    }
    let entry = read_usize(stats + comp_stats_obj::CLUBS) + read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE;
    core::ptr::write_bytes(entry as *mut u8, 0, comp_stats_obj::CLUB_ENTRY_SIZE);
    write_u32(entry, read_u32(club));
    write_u32(stats + comp_stats_obj::CLUB_COUNT, read_u32(stats + comp_stats_obj::CLUB_COUNT).wrapping_add(1));
    write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    exe::comp_stats_restore_team_runs(stats, entry);
    entry
}

/// Rule: a new player entry of the competition statistics is zeroed with the player's id and the club he played
/// for; the table grows 22 at a time.
///
/// `Comp_Stats::add_player_entry(player, club)`: no player: the comp_stats.CPP 6402 box and 0. A full table grows by
/// 0x16 (`crt_realloc` to capacity × 0x2d; on failure the comp_stats.CPP 6416 box and `exit(-1)`). The entry after the
/// last is zeroed (0x2d bytes), takes the player's id and the `club` dword as passed, the new mark 0xff at +0x1c, the
/// table is marked unsorted (0xfe) and the count goes up. Returns the entry.
/// Original: `Comp_Stats::add_player_entry` (0x4ad030, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_add_player_entry(stats: usize, player: usize, club: u32) -> usize {
    if player == 0 {
        show_error_box(COMP_STATS_CPP, 0x1902);
        return 0;
    }
    let capacity = read_u32(stats + comp_stats_obj::PLAYER_CAPACITY);
    if read_u32(stats + comp_stats_obj::PLAYER_COUNT) == capacity {
        covered!("tel31_1 player table grows");
        let capacity = capacity.wrapping_add(0x16);
        write_u32(stats + comp_stats_obj::PLAYER_CAPACITY, capacity);
        let table = exe::crt_realloc(read_usize(stats + comp_stats_obj::PLAYERS), capacity.wrapping_mul(comp_stats_obj::PLAYER_ENTRY_SIZE as u32));
        write_usize(stats + comp_stats_obj::PLAYERS, table);
        if table == 0 {
            comp_stats_fatal(0x1910);
        }
    }
    let entry = read_usize(stats + comp_stats_obj::PLAYERS) + read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE;
    core::ptr::write_bytes(entry as *mut u8, 0, comp_stats_obj::PLAYER_ENTRY_SIZE);
    write_u32(entry + comp_stats_player::PLAYER_ID, read_u32(player));
    write_u32(entry + comp_stats_player::STAFF_ID, club);
    write_u8(entry + comp_stats_player::NEW_MARK, 0xff);
    write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    write_u32(stats + comp_stats_obj::PLAYER_COUNT, read_u32(stats + comp_stats_obj::PLAYER_COUNT).wrapping_add(1));
    entry
}

/// Rule: the club table of the competition statistics sorts by the chosen statistic, highest first (statistic 11,
/// lowest first), then by club id.
///
/// `comp_stats_compare_team_entries(a, b)`, the qsort order: the statistic is the club sort byte of the stats in
/// `G_COMP_STATS_SORT_CONTEXT`; with 0xff only the ids count. Each value is `comp_stats_get_team_entry_value`
/// stored as a float; with statistic 11 a smaller value goes first, otherwise a larger one; equal values (or 0xff):
/// the larger id goes after.
/// Original: `comp_stats_compare_team_entries` (0x4ad1a0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_compare_team_entries(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let stats = read_usize(G_COMP_STATS_SORT_CONTEXT);
    let stat = read_u8(stats + comp_stats_obj::CLUB_SORT);
    if stat != 0xff {
        let va = team_entry_value(stats, a, stat) as f32;
        let vb = team_entry_value(read_usize(G_COMP_STATS_SORT_CONTEXT), b, read_u8(read_usize(G_COMP_STATS_SORT_CONTEXT) + comp_stats_obj::CLUB_SORT)) as f32;
        let below = if read_u8(read_usize(G_COMP_STATS_SORT_CONTEXT) + comp_stats_obj::CLUB_SORT) == 0xb { -1 } else { 1 };
        if let Some(r) = stat_order(va, vb, below) {
            return r;
        }
    }
    id_order(a, b)
}

/// Rule: the player table of the competition statistics sorts by the chosen statistic, highest first (statistic
/// 0x12, lowest first), then by person id; the average rating (0x11) only counts players with half their club's
/// games, and the per-game statistics (0xd..0x10) those with 35 times their club's games in appearances.
///
/// `comp_stats_compare_player_entries(a, b)`, the qsort order: the statistic is the player sort byte of the stats in
/// `G_COMP_STATS_SORT_CONTEXT`; with 0xff only the ids count. Each value is `Comp_Stats::get_player_entry_value`
/// stored as a float; statistic 0x11: a player whose club (`comp_stats_find_club_entry` of the club at +8; no club:
/// the comp_stats.CPP 2274 box) has team statistic 1 × 0.5 above his player statistic 1 counts 0; statistics
/// 0xd..0x10: one whose club's team statistic 1 (0 with no entry) × 35.0 is above his appearances (+0x29) counts 0.
/// Statistic 0x12: smaller first, otherwise larger first; equal values: the larger id goes after.
/// Original: `comp_stats_compare_player_entries` (0x4ad250, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_compare_player_entries(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let sort = || read_u8(read_usize(G_COMP_STATS_SORT_CONTEXT) + comp_stats_obj::PLAYER_SORT);
    let stat = sort();
    if stat != 0xff {
        let mut va = player_entry_value(read_usize(G_COMP_STATS_SORT_CONTEXT), a, stat) as f32;
        let mut vb = player_entry_value(read_usize(G_COMP_STATS_SORT_CONTEXT), b, sort()) as f32;
        let stat = sort();
        if stat == 0x11 {
            va = rated_player_value(a, va);
            vb = rated_player_value(b, vb);
        } else if (0xd..=0x10).contains(&stat) {
            va = per_game_player_value(a, va);
            vb = per_game_player_value(b, vb);
        }
        let below = if sort() == 0x12 { -1 } else { 1 };
        if let Some(r) = stat_order(va, vb, below) {
            return r;
        }
    }
    id_order(a, b)
}

/// Rule: the all-time record of most titles is the club that won the competition most often in the history table;
/// on a tie the first to reach the count keeps it.
///
/// `Comp_Stats::update_most_titles_record()`: the history table (`G_CLUB_COMP_HISTORY_TABLE`, 0x1a-byte rows) sorted
/// (qsort with `database_compare_comp_history_rows`); a word per club (`crt_operator_new` of 2 × `G_CLUB_COUNT`; on
/// failure the comp_stats.CPP 6619 box and 0), zeroed. The rows of this competition (row +4 = the competition's id;
/// the sort keeps them together: the walk stops at the first other row after them): with a champion (+0xa), his count
/// goes up and, when the record is empty (0xff) or the count is above it (signed), the record takes the count's low
/// byte and the club's id. Returns 1 when the record's club ended up different from the one before, else 0.
/// Original: `Comp_Stats::update_most_titles_record` (0x4ad680, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_update_most_titles_record(stats: usize) -> u32 {
    let before = read_u32(stats + comp_stats_obj::MOST_TITLES_CLUB);
    let mut changed = 0;
    exe::crt_qsort(read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT), COMP_HISTORY_ROW_SIZE as u32,
        COMPARE_COMP_HISTORY_ROWS);
    let size = read_u32(G_CLUB_COUNT).wrapping_mul(2);
    let tally = exe::crt_operator_new(size);
    if tally == 0 {
        show_error_box(COMP_STATS_CPP, 0x19db);
        return 0;
    }
    core::ptr::write_bytes(tally as *mut u8, 0, size as usize);
    let mut found = false;
    let mut i = 0;
    while i < read_i32(G_CLUB_COMP_HISTORY_COUNT) {
        let row = read_usize(G_CLUB_COMP_HISTORY_TABLE) + i as usize * COMP_HISTORY_ROW_SIZE;
        if read_u32(row + 4) == read_u32(read_usize(stats + comp_stats_obj::COMP) + 4) {
            let club = read_usize(row + 0xa);
            if club != 0 {
                let id = read_u32(club);
                let slot = tally.wrapping_add((id as usize).wrapping_mul(2));
                write_i16(slot, read_i16(slot).wrapping_add(1));
                let n = read_i16(slot);
                let record = read_u8(stats + comp_stats_obj::MOST_TITLES);
                if record == 0xff || n > record as i8 as i16 {
                    covered!("tel31_1 most titles record taken");
                    write_u8(stats + comp_stats_obj::MOST_TITLES, n as u8);
                    write_u32(stats + comp_stats_obj::MOST_TITLES_CLUB, id);
                    changed = (id != before) as u32;
                }
            }
            found = true;
        } else if found {
            break;
        }
        i += 1;
    }
    exe::crt_operator_delete(tally);
    changed
}

/// Rule: the season's best average rating is the player at the top of the rating sort (statistic 0x11).
///
/// `Comp_Stats::update_season_rating_record()`: no player table: 0. Unless the players are already sorted by 0x11:
/// the clubs sorted by id (sort byte 0xff, qsort with `comp_stats_compare_team_entries`) when they were not, then the
/// players by 0x11 (qsort with `comp_stats_compare_player_entries`, 0x2d bytes). The first player gives the season
/// rating record: the rating (player statistic 0x11, as a float), his games (statistic 1 through `_ftol`, a byte),
/// the name reference (`fill_staff_name_ref` of his person), his club (+8) and 0 in the word after. Returns 1.
/// Original: `Comp_Stats::update_season_rating_record` (0x4ad7f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_update_season_rating_record(stats: usize) -> u32 {
    if read_usize(stats + comp_stats_obj::PLAYERS) == 0 {
        return 0;
    }
    if read_u8(stats + comp_stats_obj::PLAYER_SORT) != 0x11 {
        if read_u8(stats + comp_stats_obj::CLUB_SORT) != 0xff {
            write_u8(stats + comp_stats_obj::CLUB_SORT, 0xff);
            write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
            exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
        }
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0x11);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
            comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES);
    }
    let top = read_usize(stats + comp_stats_obj::PLAYERS);
    write_f32(stats + comp_stats_obj::SEASON_RATING, player_entry_value(stats, top, 0x11) as f32);
    write_u8(stats + comp_stats_obj::SEASON_RATING_GAMES, ftol(player_entry_value(stats, top, 1)) as u8);
    exe::fill_staff_name_ref(staff_at(read_i32(top + comp_stats_player::STAFF_ID) as isize).0, stats + comp_stats_obj::SEASON_RATING_HOLDER);
    write_u32(stats + comp_stats_obj::SEASON_RATING_CLUB, read_u32(top + comp_stats_player::CLUB));
    write_u16(stats + comp_stats_obj::SEASON_RATING_EXTRA, 0);
    1
}

/// Rule: clearing the season records leaves every record empty (-1, the rating -1.0).
///
/// `comp_season_records_clear(records)`: no records: the comp_stats.CPP 6788 box and 0. Otherwise -1 in the rating
/// float (+0x189 as -1.0), the dwords +1, +0x25, +0x48, +0x6b, +0x7d, +0x9c, +0xb3, +0xcb, +0xe3, +0xfb, +0x213 and
/// +0x252, the words +5, +0xd and +0x15 and the bytes +0, +0x1d, +0x113, +0x132, +0x16a, +0x1ac and +0x1cb. Returns 1.
/// Original: `comp_season_records_clear` (0x4ad8c0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_season_records_clear(records: usize) -> u32 {
    if records == 0 {
        show_error_box(COMP_STATS_CPP, 0x1a84);
        return 0;
    }
    write_u32(records + 0x189, 0xbf800000);
    for o in [1, 0x25, 0x48, 0x6b, 0x7d, 0x9c, 0xb3, 0xcb, 0xe3, 0xfb, 0x213, 0x252] {
        write_u32(records + o, u32::MAX);
    }
    for o in [5, 0xd, 0x15] {
        write_u16(records + o, u16::MAX);
    }
    for o in [0, 0x1d, 0x113, 0x132, 0x16a, 0x1ac, 0x1cb] {
        write_u8(records + o, 0xff);
    }
    1
}

/// Rule: a player's position group for the statistics: goalkeeper (1), defender (4), midfielder (0x10) or attacker
/// (0x40), by the positions rated 20.
///
/// `comp_stats_get_player_position_group(sheet)`: goalkeeper 20: 1. A sweeper, defender or wing back 20: defender (4)
/// unless he is also a midfielder, defensive or attacking midfielder 20 and the sum of those three is not below the
/// defender rating × 1.5. Then a midfielder of any kind (20) is a midfielder (0x10) unless he is an attacker 20 with
/// the central side at least as good as the left and the right sides, which makes him an attacker (0x40); anybody
/// else is an attacker (0x40). The group comes back in ax.
/// Original: `comp_stats_get_player_position_group` (0x4ae4f0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_get_player_position_group(sheet: usize) -> u16 {
    let p = PlayerSheet(sheet);
    if p.goalkeeper() == 20 {
        return 1;
    }
    let midfield = p.midfielder() == 20 || p.defensive_midfielder() == 20 || p.attacking_midfielder() == 20;
    if p.sweeper() == 20 || p.defender() == 20 || p.wing_back() == 20 {
        if !midfield {
            return 4;
        }
        let sum = p.midfielder() as i32 + p.defensive_midfielder() as i32 + p.attacking_midfielder() as i32;
        if (sum as f64) < p.defender() as f64 * read_f64(rdata::F64_1_5) {
            covered!("tel31_1 defender before midfielder");
            return 4;
        }
    }
    if !midfield {
        return 0x40;
    }
    if p.attacker() < 20 || p.central() < p.left_side() || p.central() < p.right_side() {
        return 0x10;
    }
    0x40
}

/// Comparer for `comp_run_record_assign`: side by side over the 0x18 bytes copied and the return.
pub(crate) unsafe extern "thiscall" fn compare_comp_run_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_RUN_RECORD_ASSIGN);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x18)], || o(this, source), || comp_run_record_assign(this, source));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_run_record_assign({:#x}, {:#x}): differences in {:?}", this, source, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_season_records_assign`: side by side over the 0x269 bytes copied and the return.
pub(crate) unsafe extern "thiscall" fn compare_comp_season_records_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_SEASON_RECORDS_ASSIGN);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(this, comp_stats_obj::SEASON_RECORDS_SIZE)], || o(this, source),
        || comp_season_records_assign(this, source));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_season_records_assign({:#x}, {:#x}): differences in {:?}", this, source, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_build_season_records_list`: the block is allocated and the archive list is read from
/// disk, so only the original runs; the copy rebuilds the rows into its own block afterwards and the two blocks are
/// compared row by row (the stats it may write were already written by the original, so the copy sees the same).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_build_season_records_list(stats: usize, count: *mut u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u32) -> usize>(IDX_COMP_STATS_BUILD_SEASON_RECORDS_LIST);
    let block = o(stats, count);
    if block == 0 {
        return block;
    }
    let mut n = 0u32;
    let mine = comp_stats_build_season_records_list(stats, &mut n);
    if mine != 0 {
        let size = *count as usize * SEASON_RECORDS_ROW;
        let equal = n == *count && core::slice::from_raw_parts(block as *const u8, size) == core::slice::from_raw_parts(mine as *const u8, size);
        SCOREBOARD.record(equal, || format!("comp_stats_build_season_records_list({:#x}): rows {} x {}", stats, *count, n));
        exe::crt_free(mine);
    }
    block
}

/// Comparer for `comp_stats_add_club_entry`: it reallocates, so only the original runs; the plan is checked after
/// it: the entry is the last one, with the club's id, and the table is marked unsorted.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_add_club_entry(stats: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_STATS_ADD_CLUB_ENTRY);
    let entry = o(stats, club);
    if entry != 0 {
        let n = read_u32(stats + comp_stats_obj::CLUB_COUNT);
        let ok = n >= 1 && entry == read_usize(stats + comp_stats_obj::CLUBS) + (n as usize - 1) * comp_stats_obj::CLUB_ENTRY_SIZE
            && read_u32(entry) == read_u32(club) && read_u8(stats + comp_stats_obj::CLUB_SORT) == 0xfe;
        SCOREBOARD.record(ok, || format!("comp_stats_add_club_entry({:#x}, {:#x}): entry {:#x} out of plan", stats, club, entry));
    }
    entry
}

/// Comparer for `comp_stats_add_player_entry`: it reallocates, so only the original runs; the plan is checked after
/// it: the last entry, the player's id, the club dword, the new mark and the table marked unsorted.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_add_player_entry(stats: usize, player: usize, club: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_COMP_STATS_ADD_PLAYER_ENTRY);
    let entry = o(stats, player, club);
    if entry != 0 {
        let n = read_u32(stats + comp_stats_obj::PLAYER_COUNT);
        let ok = n >= 1 && entry == read_usize(stats + comp_stats_obj::PLAYERS) + (n as usize - 1) * comp_stats_obj::PLAYER_ENTRY_SIZE
            && read_u32(entry) == read_u32(player) && read_u32(entry + comp_stats_player::STAFF_ID) == club
            && read_u8(entry + comp_stats_player::NEW_MARK) == 0xff && read_u8(stats + comp_stats_obj::PLAYER_SORT) == 0xfe;
        SCOREBOARD.record(ok, || format!("comp_stats_add_player_entry({:#x}, {:#x}): entry {:#x} out of plan", stats, player, entry));
    }
    entry
}

/// Comparer for `comp_stats_update_most_titles_record`: side by side over the record (titles byte and club), the
/// history table (the sort rewrites it) and the return; the tally block is the copy's own.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_update_most_titles_record(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_COMP_STATS_UPDATE_MOST_TITLES_RECORD);
    let history = (read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT) as usize * COMP_HISTORY_ROW_SIZE);
    let regions = [(stats + comp_stats_obj::MOST_TITLES, 5), history];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats), || comp_stats_update_most_titles_record(stats));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_stats_update_most_titles_record({:#x}): return {} x {}, differences in {:?}", stats, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_update_season_rating_record`: side by side over the two tables (the sorts rewrite them),
/// the sort bytes, the rating record and the sort context.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_update_season_rating_record(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_COMP_STATS_UPDATE_SEASON_RATING_RECORD);
    let regions = [
        (read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (stats + comp_stats_obj::CLUB_SORT, 1),
        (stats + comp_stats_obj::PLAYER_SORT, 1),
        (stats + comp_stats_obj::SEASON_RATING, 0x23),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats), || comp_stats_update_season_rating_record(stats));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_stats_update_season_rating_record({:#x}): differences in {:?}", stats, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_season_records_clear`: side by side over the 0x269 bytes of the records and the return.
pub(crate) unsafe extern "C" fn compare_comp_season_records_clear(records: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_COMP_SEASON_RECORDS_CLEAR);
    if records == 0 {
        return o(records);
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(records, comp_stats_obj::SEASON_RECORDS_SIZE)], || o(records),
        || comp_season_records_clear(records));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_season_records_clear({:#x}): differences in {:?}", records, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_player_position_group`: the group, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_comp_stats_get_player_position_group(sheet: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_COMP_STATS_GET_PLAYER_POSITION_GROUP)(sheet);
    let n = comp_stats_get_player_position_group(sheet);
    SCOREBOARD.record(r == n, || format!("comp_stats_get_player_position_group({:#x}) original={:#x} rust={:#x}", sheet, r, n));
    r
}

/// Rule: a competition record becomes one line of text with links: "-" first, then, when the record holds
/// something, the kind's sentence (team records, attendances, results, runs, player records), with the club, the
/// opponent, the stage, the match and the person linked.
///
/// `comp_stats_format_record_text(records, kind, text)`: no text: the comp_stats.CPP 3911 box and 0; `Text::clear`
/// and "-"; no records: the comp_stats.CPP 3923 box and 0. By kind (1..22; any other: the comp_stats.CPP 4973 box and
/// 0), a record that is empty (its count below 0, the rating not above -1.0) stays "-":
/// 1 most wins (byte +0, club +1); 2 points (+5, club +7, year +0xb), 3 and 4 goals (+0xd / +0x15) and 5 cards
/// (yellow +0x1d, red +0x1e) as team values with the season when the year is set; 6 and 7 the attendances (+0x25 /
/// +0x48, with the match after them); 8 the average attendance (+0x6b, club +0x6f, year +0x73: an average of 0 or less
/// gives "-", the club still linked); 9 and 10 the results (match records +0x75 / +0x94, the names in venue order:
/// both read the venue byte at +0xa4, the second record's, a defect of the original for kind 9); 11 to 14 the runs
/// (+0xb3, +0xcb, +0xe3, +0xfb; the unbeaten and winless ones end in "?" while still going); 15 the top scorer and 16
/// the most goals in a match (holder, club, year); 17 assists, 18 the average rating ("%4.2f", the games), 19 man of
/// the match, 20 the cards; 21 and 22 the youngest and oldest (the age in years at the match). Returns 1.
/// Original: `comp_stats_format_record_text` (0x4a5690, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_format_record_text(stats: usize, rec: usize, kind: u32, text: usize) -> u32 {
    if text == 0 || rec == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_COMP_STATS_FORMAT_RECORD_TEXT)(stats, rec, kind, text);
    }
    let kind = kind as u8 as i8;
    if !(1..=22).contains(&kind) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_COMP_STATS_FORMAT_RECORD_TEXT)(stats, rec, kind as u8 as u32, text);
    }
    exe::text_clear((text) as usize as *mut usize);
    exe::text_set_string(text, 0x98ba0c as *const u8);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match kind {
        1 => {
            let wins = read_i8(rec);
            if wins > -1 {
                let mut out = [0u8; 0x200];
                let mut name = [0u8; 0xc8];
                record_club_name(record_club(rec, 1), &mut name);
                f(out.as_mut_ptr(), 0x99d80c, name.as_ptr(), wins as i32);
                exe::text_set_string(text, out.as_ptr());
                exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(rec, 1));
            }
        }
        2 if read_i16(rec + 5) > -1 => record_team_value(stats, text, rec, read_i16(rec + 5) as i32, 7, 0xb, 0x99d7d0, 0x99d7b0),
        3 if read_i16(rec + 0xd) > -1 => record_team_value(stats, text, rec, read_i16(rec + 0xd) as i32, 0xf, 0x13, 0x99d774, 0x99d754),
        4 if read_i16(rec + 0x15) > -1 => record_team_value(stats, text, rec, read_i16(rec + 0x15) as i32, 0x17, 0x1b, 0x99d774, 0x99d754),
        5 if read_i8(rec + 0x1d) > -1 => {
            let mut out = [0u8; 0x200];
            let mut name = [0u8; 0xc8];
            let mut season = [0u8; 0x14];
            let club = record_club(rec, 0x1f);
            record_club_name(club, &mut name);
            let gender = record_gender(club);
            let year = read_u16(rec + 0x23);
            let (yellow, red) = (read_i8(rec + 0x1d) as i32, read_i8(rec + 0x1e) as i32);
            if year > 0 {
                record_season(stats, year, &mut season);
                f(out.as_mut_ptr(), 0x99d6e8, gender, gender, name.as_ptr(), yellow, red, season.as_ptr());
            } else {
                f(out.as_mut_ptr(), 0x99d698, gender, gender, name.as_ptr(), yellow, red);
            }
            exe::text_set_string(text, out.as_ptr());
            exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(rec, 0x1f));
        }
        6 if read_i32(rec + 0x25) > -1 => record_attendance(text, rec, 0x25),
        7 if read_i32(rec + 0x48) > -1 => record_attendance(text, rec, 0x48),
        8 if read_i32(rec + 0x6b) > -1 => {
            let mut out = [0u8; 0x200];
            let mut name = [0u8; 0xc8];
            let mut season = [0u8; 0x14];
            let club = record_club(rec, 0x6f);
            record_club_name(club, &mut name);
            let gender = record_gender(club);
            let year = read_u16(rec + 0x73);
            let average = read_i32(rec + 0x6b);
            if year > 0 {
                record_season(stats, year, &mut season);
            }
            if average > 0 {
                if year > 0 {
                    f(out.as_mut_ptr(), 0x99d58c, gender, gender, average, name.as_ptr(), season.as_ptr());
                } else {
                    f(out.as_mut_ptr(), 0x99d55c, gender, gender, average, name.as_ptr());
                }
            } else {
                let crt: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                crt(out.as_mut_ptr(), 0x991944, 0x98ba0c);
            }
            exe::text_set_string(text, out.as_ptr());
            exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(rec, 0x6f));
        }
        // the original reads the venue at +0xa4 for both results: the second record's venue (kind 10's) also for
        // kind 9 (defect of the original, kept)
        9 if read_i32(rec + 0x7d) > -1 => record_result(text, rec, 0x75, read_u8(rec + 0xa4)),
        10 if read_i32(rec + 0x9c) > -1 => record_result(text, rec, 0x94, read_u8(rec + 0xa4)),
        11 if read_i32(rec + 0xb3) > -1 => record_run(stats, text, rec, 0xb3, None, true, (0x99d454, 0x99d428)),
        12 if read_i32(rec + 0xcb) > -1 => record_run(stats, text, rec, 0xcb, None, true, (0x99d3d4, 0x99d3a8)),
        13 if read_i32(rec + 0xe3) > -1 => record_run(stats, text, rec, 0xe3, Some(comp_stats_club::UNBEATEN_RUN), false, (0x99d354, 0x99d324)),
        14 if read_i32(rec + 0xfb) > -1 => record_run(stats, text, rec, 0xfb, Some(comp_stats_club::WINLESS_RUN), false, (0x99d2cc, 0x99d29c)),
        15 if read_i8(rec + 0x113) > -1 => {
            let holder = rec + 0x114;
            let mut out = [0u8; 0x200];
            let (mut name, mut team) = ([0u8; 0xc8], [0u8; 0xc8]);
            let mut season = [0u8; 0x14];
            record_holder_name(holder, &mut name, 1);
            record_club_name(record_club(rec, 0x12c), &mut team);
            let year = read_u16(rec + 0x130);
            let goals = read_i8(rec + 0x113) as i32;
            if year > 0 {
                record_season(stats, year, &mut season);
                f(out.as_mut_ptr(), 0x99d250, name.as_ptr(), team.as_ptr(), goals, season.as_ptr());
            } else {
                f(out.as_mut_ptr(), 0x99d220, name.as_ptr(), team.as_ptr(), goals);
            }
            exe::text_set_string(text, out.as_ptr());
            exe::text_add_link(text, team.as_ptr(), LINK_CLUB, record_club(rec, 0x12c));
            record_holder_link(text, holder, &name);
        }
        16 if read_i8(rec + 0x132) > -1 => record_player_match(text, rec + 0x133, rec + 0x14b, RecordLead::Goals(read_i8(rec + 0x132) as i32)),
        17 if read_i8(rec + 0x16a) > -1 => record_player_count(stats, text, rec, 0x16a, false, (0x99d0cc, 0x99d098)),
        18 => {
            let rating = read_f32(rec + 0x189);
            if rating > -1.0 {
                let holder = rec + 0x18d;
                let mut out = [0u8; 0x200];
                let (mut name, mut team) = ([0u8; 0xc8], [0u8; 0xc8]);
                let mut season = [0u8; 0x14];
                record_holder_name(holder, &mut name, 1);
                record_club_name(record_club(rec, 0x1a6), &mut team);
                let year = read_u16(rec + 0x1aa);
                let games = read_i8(rec + 0x1a5) as i32;
                if year > 0 {
                    record_season(stats, year, &mut season);
                    f(out.as_mut_ptr(), 0x99d024, read_f32(rec + 0x189) as f64, name.as_ptr(), team.as_ptr(), games, season.as_ptr());
                } else {
                    f(out.as_mut_ptr(), 0x99cfcc, read_f32(rec + 0x189) as f64, name.as_ptr(), team.as_ptr(), games);
                }
                exe::text_set_string(text, out.as_ptr());
                exe::text_add_link(text, team.as_ptr(), LINK_CLUB, record_club(rec, 0x1a6));
                record_holder_link(text, holder, &name);
            }
        }
        19 if read_i8(rec + 0x1ac) > -1 => record_player_count(stats, text, rec, 0x1ac, false, (0x99cf78, 0x99cf40)),
        20 if read_i8(rec + 0x1cb) > -1 => record_player_count(stats, text, rec, 0x1cb, true, (0x99cec4, 0x99ce60)),
        21 if read_i32(rec + 0x213) > -1 => record_player_match(text, rec + 0x1f3, rec + 0x20b, RecordLead::Age(rec + 0x1eb)),
        22 if read_i32(rec + 0x252) > -1 => record_player_match(text, rec + 0x232, rec + 0x24a, RecordLead::Age(rec + 0x22a)),
        _ => {}
    }
    1
}

/// Comparer for `comp_stats_format_record_text`: the text, side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_format_record_text(stats: usize, rec: usize, kind: u32, text: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_COMP_STATS_FORMAT_RECORD_TEXT);
    let k = kind as u8 as i8;
    if text == 0 || rec == 0 || !(1..=22).contains(&k) {
        return o(stats, rec, kind, text);
    }
    let mut r = 0;
    compare_text_writer(text, || r = o(stats, rec, kind, text), |t| { comp_stats_format_record_text(stats, rec, kind, t); },
        || format!("comp_stats_format_record_text(kind {})", k));
    r
}

/// Rule: a competition record goes into a news item's slots: the first value, the kind and then the kind's fields
/// in order (counts, clubs, years, match records, date spans and the holders' names), each in the next slot.
///
/// `comp_stats_write_record_to_news(first, records, kind, news)`: no records: the comp_stats.CPP 6839 box and 0; no
/// news: the comp_stats.CPP 6847 box and 0. Slot 0 takes `first`, slot 1 the kind; then by kind (1..22, any other: the
/// comp_stats.CPP 7104 box and 0): the bytes sign-extended, the words of counts sign-extended and the year words
/// zero-extended, the dwords as they are; the match records through `write_match_record_to_news`, the run dates
/// through `write_date_span_record_to_news` and the holders through `write_staff_name_ref_to_news` (each moves the
/// slot on); the average rating as `_ftol`(rating × 100). Returns 1.
/// Original: `comp_stats_write_record_to_news` (0x4ad9c0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_write_record_to_news(first: u32, rec: usize, kind: u32, news: usize) -> u32 {
    if rec == 0 {
        show_error_box(COMP_STATS_CPP, 0x1ab7);
        return 0;
    }
    if news == 0 {
        show_error_box(COMP_STATS_CPP, 0x1abf);
        return 0;
    }
    let mut slot: u8 = 1;
    exe::newsitem_set_slot(news, 0, first);
    let kind = kind as u8 as i8;
    let put = |slot: &mut u8, v: u32| {
        let s = *slot;
        *slot = s.wrapping_add(1);
        exe::newsitem_set_slot(news, s as i8, v);
    };
    put(&mut slot, kind as i32 as u32);
    let b = |o: usize| read_i8(rec + o) as i32 as u32;
    let sw = |o: usize| read_i16(rec + o) as i32 as u32;
    let zw = |o: usize| read_u16(rec + o) as u32;
    let d = |o: usize| read_u32(rec + o);
    let span: NewsDateSpanWriter = core::mem::transmute(FN_WRITE_DATE_SPAN_RECORD_TO_NEWS);
    let matched = |slot: &mut u8, o: usize| { exe::write_match_record_to_news(rec + o, news, slot as *mut u8); };
    let named = |slot: &mut u8, o: usize| exe::write_staff_name_ref_to_news(rec + o, news, slot as *mut u8);
    match kind {
        1 => {
            put(&mut slot, b(0));
            put(&mut slot, d(1));
        }
        2 | 3 | 4 => {
            let o = [5usize, 0xd, 0x15][(kind - 2) as usize];
            put(&mut slot, sw(o));
            put(&mut slot, d(o + 2));
            put(&mut slot, zw(o + 6));
        }
        5 => {
            put(&mut slot, b(0x1d));
            put(&mut slot, b(0x1e));
            put(&mut slot, d(0x1f));
            put(&mut slot, zw(0x23));
        }
        6 => {
            put(&mut slot, d(0x25));
            matched(&mut slot, 0x29);
        }
        7 => {
            put(&mut slot, d(0x48));
            matched(&mut slot, 0x4c);
        }
        8 => {
            put(&mut slot, d(0x6b));
            put(&mut slot, d(0x6f));
            put(&mut slot, zw(0x73));
        }
        9 => matched(&mut slot, 0x75),
        10 => matched(&mut slot, 0x94),
        11..=14 => {
            let o = [0xb3usize, 0xcb, 0xe3, 0xfb][(kind - 11) as usize];
            span(rec + o, news, &mut slot);
            put(&mut slot, d(o + 0x14));
        }
        15 | 17 | 19 => {
            let o = match kind { 15 => 0x113usize, 17 => 0x16a, _ => 0x1ac };
            put(&mut slot, b(o));
            named(&mut slot, o + 1);
            put(&mut slot, d(o + 0x19));
            put(&mut slot, zw(o + 0x1d));
        }
        16 => {
            put(&mut slot, b(0x132));
            named(&mut slot, 0x133);
            matched(&mut slot, 0x14b);
        }
        18 => {
            let rating = ftol(read_f32(rec + 0x189) as f64 * read_f32(rdata::F32_100) as f64);
            put(&mut slot, rating as u32);
            named(&mut slot, 0x18d);
            put(&mut slot, b(0x1a5));
            put(&mut slot, d(0x1a6));
            put(&mut slot, zw(0x1aa));
        }
        20 => {
            put(&mut slot, b(0x1cb));
            put(&mut slot, b(0x1cc));
            named(&mut slot, 0x1cd);
            put(&mut slot, d(0x1e5));
            put(&mut slot, zw(0x1e9));
        }
        21 | 22 => {
            let o = if kind == 21 { 0x1ebusize } else { 0x22a };
            put(&mut slot, sw(o));
            put(&mut slot, zw(o + 2));
            named(&mut slot, o + 8);
            matched(&mut slot, o + 0x20);
        }
        _ => {
            show_error_box(COMP_STATS_CPP, 0x1bc0);
            return 0;
        }
    }
    1
}

/// Comparer for `comp_stats_write_record_to_news`: side by side over the news item (0xdf bytes) and the return.
pub(crate) unsafe extern "C" fn compare_comp_stats_write_record_to_news(first: u32, rec: usize, kind: u32, news: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, usize) -> u32>(IDX_COMP_STATS_WRITE_RECORD_TO_NEWS);
    if rec == 0 || news == 0 || !(1..=22).contains(&(kind as u8 as i8)) {
        return o(first, rec, kind, news);
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(news, NEWS_NEWS_ITEM_SIZE)], || o(first, rec, kind, news),
        || comp_stats_write_record_to_news(first, rec, kind, news));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_stats_write_record_to_news(kind {}): differences in {:?}", kind as u8 as i8, first_differences(&mo, &mr)));
    ro
}

/// One write of `Comp_Stats::save`: `n` records of `size` bytes from `at`; short: the comp_stats.CPP box at `line`,
/// `game_fatal_cleanup` and `exit(-1)` (`comp_stats_fatal`).
pub(crate) unsafe fn comp_stats_write(at: usize, size: u32, n: u32, file: usize, line: u32) {
    if exe::crt_fwrite(at, size, n, file) != n {
        comp_stats_fatal(line);
    }
}

/// One read of `Comp_Stats::load`: `n` records of `size` bytes into `at`; short: the comp_stats.CPP box at `line`,
/// `game_fatal_cleanup` and `exit(-1)` (`comp_stats_fatal`).
pub(crate) unsafe fn comp_stats_read(at: usize, size: u32, n: u32, file: usize, line: u32) {
    if exe::crt_fread(at, size, n, file) != n {
        comp_stats_fatal(line);
    }
}

/// A table allocation of `Comp_Stats::load`: `crt_malloc`; failing, the comp_stats.CPP box at `line` and the game
/// stops.
pub(crate) unsafe fn comp_stats_table(size: u32, line: u32) -> usize {
    let block = exe::crt_malloc(size);
    if block == 0 {
        comp_stats_fatal(line);
    }
    block
}

/// The regions the two entry finders may rewrite: a table (it may be re-sorted by id), its sort byte and the sort
/// context.
pub(crate) unsafe fn comp_stats_find_regions(stats: usize, players: bool) -> [(usize, usize); 3] {
    if players {
        [(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
         (stats + comp_stats_obj::PLAYER_SORT, 1), (G_COMP_STATS_SORT_CONTEXT, 4)]
    } else {
        [(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
         (stats + comp_stats_obj::CLUB_SORT, 1), (G_COMP_STATS_SORT_CONTEXT, 4)]
    }
}

/// `Comp_Stats::find_player_entry` (0x4a4a30, thiscall(stats, player), ret 4), through its address.
pub(crate) unsafe fn comp_stats_player_entry_of(stats: usize, player: usize) -> usize {
    exe::comp_stats_find_player_entry(stats, player)
}

/// Copies one club entry (0x76 bytes) as the original does it, field by field: the id, the two match counts, the
/// home attendance, the three 5-byte arrays at +0xa, the four runs (`run_record_assign`) and the bytes +0x69..+0x75.
pub(crate) unsafe fn comp_stats_copy_club_entry(dst: usize, src: usize) {
    write_u32(dst, read_u32(src));
    write_u8(dst + 4, read_u8(src + 4));
    write_u8(dst + 5, read_u8(src + 5));
    write_u32(dst + 6, read_u32(src + 6));
    for o in 0xa..0x19 {
        write_u8(dst + o, read_u8(src + o));
    }
    for o in [comp_stats_club::WINNING_RUN, comp_stats_club::LOSING_RUN, comp_stats_club::UNBEATEN_RUN, comp_stats_club::WINLESS_RUN] {
        run_record_assign(dst + o, src + o);
    }
    for o in 0x69..comp_stats_obj::CLUB_ENTRY_SIZE {
        write_u8(dst + o, read_u8(src + o));
    }
}

/// The regions a sorted table may rewrite: both tables (the sorts), both sort bytes and the context.
pub(crate) unsafe fn comp_stats_sorted_table_regions(stats: usize) -> [(usize, usize); 5] {
    [
        (read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (stats + comp_stats_obj::CLUB_SORT, 1),
        (stats + comp_stats_obj::PLAYER_SORT, 1),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ]
}

/// The two blocks (and counts) of a sorted table side by side: equal counts and bytes; the copy's block is freed.
pub(crate) unsafe fn comp_stats_sorted_blocks_equal(ro: usize, rr: usize, in_the: i32, nr: i32, size: usize) -> bool {
    let equal = in_the == nr && (ro == 0) == (rr == 0)
        && (ro == 0 || core::slice::from_raw_parts(ro as *const u8, in_the as usize * size) == core::slice::from_raw_parts(rr as *const u8, nr as usize * size));
    if rr != 0 {
        exe::crt_free(rr);
    }
    equal
}

/// x87 `fcomp` + `test ah, 0x40`: C3 set, i.e. equal or unordered.
pub(crate) fn comp_stats_c3(a: f64, b: f64) -> bool {
    a == b || a.is_nan() || b.is_nan()
}

/// `comp_season_records_construct` (0x49f160, thiscall, ret 0): the constructors of the sub-records of a season
/// record block (dates, runs, holders); writes only inside the block.
pub(crate) unsafe fn comp_season_records_construct(records: usize) {
    exe::new_comp_season_records(records);
}

/// `langlib_sprintf` (0x66f4e0, cdecl): the format translated to the game's language, then printed.
pub(crate) unsafe fn comp_lang_format() -> CompFormatFn {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// `crt_sprintf` (0x944c9f, cdecl).
pub(crate) unsafe fn comp_crt_format() -> CompFormatFn {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// The comp_screens.cpp "Error" box at a line ("v<version> comp_screens.cpp <line>"), the left button let go.
pub(crate) unsafe fn comp_screens_error_box(line: u32) {
    tel26_error_box(COMP_SCREENS_CPP, line);
}

/// `ScreenManager::add_text_item` (0x803fd0, thiscall, ret 0x2c): x1, y1, x2, y2, the font, the flags, the style,
/// the colour, the text, the id and the value (an item of kind 1 in the current slot's Display); returns eax.
pub(crate) unsafe fn comp_text_item(f: [u32; 11]) -> u32 {
    type TextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: TextItem = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// Frees the LLIST in screen parameter `k` (its destructor and operator delete) when there is one, and clears the
/// parameter (as `comp_screens_close_comp_screen` does for 5, 0xe and 0x17).
pub(crate) unsafe fn comp_free_list_param(k: u32) {
    if get_screen_param(k) == 0 {
        return;
    }
    let list = get_screen_param(k);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    set_screen_param(k, 0);
}

/// Frees the heap block in screen parameter `k` (crt free) when there is one and clears it and the parameter after it
/// (its count), as `comp_screens_close_comp_screen` does for 0x1a/0x1b, 0x24/0x25 and 0x1f/0x20.
pub(crate) unsafe fn comp_free_block_param(k: u32) {
    if get_screen_param(k) == 0 {
        return;
    }
    exe::crt_free(get_screen_param(k));
    set_screen_param(k, 0);
    set_screen_param(k + 1, 0);
}

/// The 54 parameters of the competition screen as `comp_screens_open_comp_fixtures` leaves them.
pub(crate) fn comp_screen_planned_params(object: usize, day: u32, year: u32, mode: u32) -> [usize; 0x36] {
    let mut p = [0usize; 0x36];
    let (day, year) = ((day & 0xffff) as usize, (year & 0xffff) as usize);
    p[0] = object;
    p[1] = if mode != 0 { 2 } else { 3 };
    p[2] = -2isize as usize;
    p[3] = -2isize as usize;
    if mode != 0 {
        p[8] = day;
        p[9] = year;
    } else {
        p[0x11] = day;
        p[0x12] = year;
    }
    p[0x23] = 3;
    p[0x1e] = 0x11;
    p[0x2a] = usize::MAX;
    p[0x2d] = 1;
    p[0x32] = usize::MAX;
    p
}

/// The Results tab of the competition screen (tab 2): one match day of the competition's fixtures.
///
/// The day comes from parameters 8 and 9 (day, year) with the day's match list in parameter 0xe. Without the list,
/// unless the year is 1 (no matches): a year 0 takes today's date ([`G_CURRENT_DATE`]); a new LLIST (operator new 0xe)
/// goes to parameter 0xe and `scan_fixtures` (backward, mode 2) fills it with the match day found from the date (none:
/// year 1; else `comp_util_sort_fixture_list`), and the day and year go back to parameters 8 and 9. Year 1: "No matches
/// have been played." in a text area (pale grey, font 3). Otherwise the previous and next match days (parameters
/// 0xa/0xb and 0xc/0xd, looked up when their year is 0, `comp_screen_neighbour_day`, into a local list) give the
/// "<< Date" and "Date >>" cells (buttons 0xc and 0xd when the year is above 1, else grey text) in a small area; the
/// date (`Date::get_long_date_string`, without the year in the current year) in yellow font 3 under it ([0xadae0c] =
/// 1); then a 9-column area with the fixtures: each new round (stage code +0x32 / round +0x30, not both 0) gets an empty
/// row (but the first), its name (`comp_util_format_stage_round_name`) as a yellow item (a link, id 0xe and value
/// (stage byte + 1) × 1000 + stage index, when `Comp::are_round_fixtures_known`; up to x 0x2f1 by its width) and a
/// yellow underline (flags 0xc400); then each fixture's row (`comp_screens_add_fixture_row`, id 0xf). The scroll row
/// comes back from parameter 0xf and the area goes to parameter 0x10.
/// Original: part of `comp_screens_draw_comp_screen` (0x48fcd0, comp_screens.cpp).
pub(crate) unsafe fn comp_screen_results_tab(comp: usize, top: i32, bottom: i32) {
    let comp = League(comp);
    let mut local_list = [0u8; 0xe];
    exe::new_llist(local_list.as_mut_ptr() as usize);
    let mut day = get_screen_param(8) as u32;
    let mut year = get_screen_param(9) as u32;
    let mut prev_year = get_screen_param(0xb) as u32;
    let mut next_year = get_screen_param(0xd) as u32;
    let mut list = get_screen_param(0xe);
    let comp_id = read_u32(comp.competition().addr());
    if list == 0 && year as u16 != 1 {
        if year as u16 == 0 {
            day = read_u32(G_CURRENT_DATE);
            year = (year & 0xffff_0000) | read_u16(G_CURRENT_DATE_YEAR) as u32;
        }
        let p = exe::crt_operator_new(0xe);
        list = if p != 0 { exe::new_llist(p) } else { 0 };
        set_screen_param(0xe, list);
        let mut mode: u8 = 2;
        if comp_screen_scan(&mut year, &mut day, &mut mode, 0, comp_id, list) == 0 {
            year = 1;
        } else {
            exe::comp_util_sort_fixture_list(list);
        }
        set_screen_param(8, (day & 0xffff) as usize);
        set_screen_param(9, (year & 0xffff) as usize);
    }
    if year as u16 == 1 {
        covered!("tel22_1 results no matches");
        let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as u32;
        comp_lang_format()(JOB_TEXT, 0x99c02c); // "No matches have been played."
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, rows, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
        exe::drop_llist(local_list.as_mut_ptr() as usize);
        return;
    }
    let (mut date, mut out, mut other) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    if prev_year as u16 == 0 {
        let (d, y) = comp_screen_neighbour_day(day, year, 0, comp_id, local_list.as_mut_ptr() as usize, &mut date, &mut out, &mut other);
        prev_year = y;
        set_screen_param(0xa, (d & 0xffff) as usize);
        set_screen_param(0xb, (prev_year & 0xffff) as usize);
    }
    if next_year as u16 == 0 {
        // the next day's temporaries swap: the moved date in the third buffer, the assigned one in the second
        let (d, y) = comp_screen_neighbour_day(day, year, 1, comp_id, local_list.as_mut_ptr() as usize, &mut date, &mut other, &mut out);
        next_year = y;
        set_screen_param(0xc, (d & 0xffff) as usize);
        set_screen_param(0xd, (next_year & 0xffff) as usize);
    }
    let arrows = comp_add_area([0x6e, top as u32, 0x168, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let mut word = [0u8; 0x200];
    let (grey, pale) = (job_colour(JOB_COLOUR_317E), job_colour(JOB_COLOUR_3184));
    for (column, fmt, other_year, id) in [(0u32, 0x988090usize, prev_year, 0xcu32), (1, 0x988088, next_year, 0xd)] {
        comp_lang_format()(word.as_mut_ptr() as usize, 0x988078); // "Date"
        comp_crt_format()(JOB_TEXT, fmt, word.as_ptr()); // "<< %s" / "%s >>"
        if other_year as u16 > 1 {
            tel26_area_button([arrows, column, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, id, 0]);
        } else {
            tel26_area_text([arrows, column, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
        }
    }
    let top = top + 0x19;
    write_u32(COMP_SCREEN_DATE_BUTTONS_FLAG, 1);
    if year as u16 == read_u16(G_CURRENT_DATE_YEAR) {
        exe::new_date(date.as_mut_ptr() as usize, day, year);
        exe::date_get_long_date_string_no_year(date.as_mut_ptr() as usize, JOB_TEXT, 0x7d0, 0xff);
    } else {
        exe::new_date(out.as_mut_ptr() as usize, day, year);
        exe::date_get_long_date_string(out.as_mut_ptr() as usize, JOB_TEXT, 0x7d0, 0xff);
    }
    comp_text_item([0x6e, top as u32, 0x30c, (top + 0x23) as u32, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), JOB_TEXT as u32, 1]);
    let top = top + 0x28;
    // simplification: the row count goes as its byte; the original's dword carries stack garbage above it
    let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as u32;
    let area = comp_add_area([0x6e, top as u32, 0x30c, bottom as u32, 9, COMP_SCREEN_RESULTS_WEIGHTS.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    let mut row: i32 = 0;
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut prev = 0usize;
    let mut fx = exe::llist_get_next(list);
    while fx != 0 {
        let code = read_i16(fx + COMP_SCREEN_FIXTURE_STAGE_CODE);
        let round = read_i16(fx + COMP_SCREEN_FIXTURE_ROUND);
        let same = prev != 0 && code == read_i16(prev + COMP_SCREEN_FIXTURE_STAGE_CODE) && round == read_i16(prev + COMP_SCREEN_FIXTURE_ROUND);
        if !same && (code != 0 || round != 0) {
            covered!("tel22_1 results round heading");
            if prev != 0 {
                tel26_area_text([area, 0, row as u32, 1, 0, 1, 1, job_colour(JOB_COLOUR_3184), 0]);
                row += 1;
            }
            exe::comp_util_format_stage_round_name(code as i32, round as i32, JOB_TEXT, 0x7d0, 0);
            let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) + 0x73).min(0x2f1); // tel22_1 the round heading's width
            let stage_byte = read_u8(fx + COMP_SCREEN_FIXTURE_STAGE_BYTE) as i8 as i32;
            let stage_index = read_i16(fx + COMP_SCREEN_FIXTURE_STAGE_INDEX) as i32;
            let yellow = job_colour(G_GUI_COLOUR_YELLOW);
            if exe::comp_are_round_fixtures_known(comp, stage_byte, stage_index) != 0 {
                let value = (stage_byte + 1) * 1000 + stage_index;
                comp_add_item([2, 0x73, 0, x2 as u32, 0, 0, row as u32, 1, 0, 0, 1, 3, yellow, JOB_TEXT as u32, 0, 0xe, value as u32, area]);
            } else {
                comp_add_item([1, 0x73, 0, x2 as u32, 0, 0, row as u32, 1, 0, 0, 1, 3, yellow, JOB_TEXT as u32, 0, 0, 0, area]);
            }
            row += 1;
            comp_add_item([1, 0x73, 0, x2 as u32, 0, 0, row as u32, 0xc400, yellow, 0, 0, 0, 0, 0, 0, 0, 0, area]);
            row += 1;
        }
        exe::comp_screens_add_fixture_row(fx, area as usize, &mut row, 0xf);
        prev = fx;
        fx = exe::llist_get_next(list);
    }
    comp_set_area_scroll_row(area, get_screen_param(0xf) as u32);
    set_screen_param(0x10, area as u16 as i16 as isize as usize);
    exe::drop_llist(local_list.as_mut_ptr() as usize);
}

/// The Qualifiers tab of the competition screen (tab 5): the clubs of the entrant book with a class (+4 ≠ 0), in two
/// halves side by side (columns 0-1 and 2-3) of as many rows as fit in font 3 (more: half of them, rounded up), each
/// "  short name" as a button (id 0x7d5, the club; yellow for a human's club, else pale grey) and its class ("Host
/// nation" for 1, "Previous winner" for 2, else empty) in yellow font 0xc; the cells past the last entrant stay empty
/// (pale grey).
/// Original: part of `comp_screens_draw_comp_screen` (0x48fcd0, comp_screens.cpp).
pub(crate) unsafe fn comp_screen_qualifiers_tab(comp: usize, top: i32, bottom: i32) {
    let comp = League(comp);
    let entry_at = |k: u32| comp.entrant_book() + (k as i16 as isize * 6) as usize;
    let count = comp.entrant_count();
    let mut classed: i32 = 0;
    for k in 0..count.max(0) as u32 {
        if read_u8(entry_at(k) + 4) != 0 {
            classed += 1;
        }
    }
    let rows = ((bottom - top) / exe::font_get_height(3)) as i8;
    let half: i32 = if classed as i16 as i32 > rows as i32 * 2 { (classed + 1) / 2 } else { rows as i32 };
    let total = (half as u32).wrapping_shl(1);
    // simplification: the row count goes as its byte; the original's dword carries stack garbage above it
    let area = comp_add_area([0x6e, top as u32, 0x30c, bottom as u32, 4, 0, rows as u8 as u32, 0, 0, 0, u32::MAX]);
    if total as i16 <= 0 {
        return;
    }
    covered!("tel22_1 qualifiers rows");
    let (mut r, mut k, mut i): (u32, u32, i32) = (0, 0, 0);
    loop {
        let count = comp.entrant_count();
        while (k as i16) < count && read_u8(entry_at(k) + 4) == 0 {
            k = k.wrapping_add(1);
        }
        let halfway = total as i16 as i32 / 2;
        let (column, row) = if i < halfway { (0i32, r as u8 as i8 as i32) } else { (2, (r as u8).wrapping_sub(halfway as u8) as i8 as i32) };
        if (k as i16) < count {
            let club = Club(read_usize(entry_at(k)));
            comp_crt_format()(JOB_TEXT, 0x987ff0, club.addr() + club::SHORT_NAME); // "  %s"
            let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(JOB_COLOUR_3184) };
            tel26_area_button([area, column as u32, row as u32, 2, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, club.addr() as u32]);
            match read_u8(entry_at(k) + 4) as i8 {
                1 => {
                    comp_lang_format()(JOB_TEXT, 0x99c010); // "Host nation"
                }
                2 => {
                    comp_lang_format()(JOB_TEXT, 0x99c01c); // "Previous winner"
                }
                _ => write_u8(JOB_TEXT, 0),
            }
            tel26_area_text([area, (column + 1) as u32, row as u32, 2, 0, 0xc, 2, job_colour(G_GUI_COLOUR_YELLOW), JOB_TEXT as u32]);
        } else {
            tel26_area_text([area, column as u32, row as u32, 2, 0, 1, 2, job_colour(JOB_COLOUR_3184), 0]);
            tel26_area_text([area, (column + 1) as u32, row as u32, 2, 0, 1, 2, job_colour(JOB_COLOUR_3184), 0]);
        }
        r = r.wrapping_add(1);
        i += 1;
        k = k.wrapping_add(1);
        if (r as i16) >= (total as i16) {
            break;
        }
    }
}

/// The parameters `comp_screens_close_comp_screen` leaves: those it frees cleared (with the count after each block),
/// the others as they were.
pub(crate) unsafe fn comp_screen_close_plan() -> [usize; 0x36] {
    let mut p = [0usize; 0x36];
    for (k, v) in p.iter_mut().enumerate() {
        *v = get_screen_param(k as u32);
    }
    for k in [5usize, 0xe, 0x17] {
        p[k] = 0;
    }
    for k in [0x1ausize, 0x24, 0x1f] {
        if p[k] != 0 {
            p[k] = 0;
            p[k + 1] = 0;
        }
    }
    p
}

/// "The draw will be made on {}<%s - date >." with the long date of `date` (an 8-byte Date): the long date string into
/// `buf` (0x7d0, 0xff), the weekday's grammatical gender in the game's language, then the sentence into the screens'
/// text buffer (the weekday asked before the language, as the original).
pub(crate) unsafe fn league_tab_draw_date_sentence(date: usize, buf: *mut u8) {
    exe::date_get_long_date_string(date, buf as usize, 0x7d0, 0xff);
    let weekday = exe::date_get_weekday(date);
    let gender = exe::date_get_weekday_gender(weekday as u32, exe::langlib_get_language() as u32);
    comp_lang_format()(JOB_TEXT, 0x99c2e8, gender as i32, buf);
}

/// `comp_screens_add_stage_selector(comp, stage, top)` (0x49d380, cdecl): the stage buttons above a tab; returns
/// nonzero when it added them.
pub(crate) unsafe fn league_tab_stage_selector(comp: usize, stage: usize, top: i32) -> u32 {
    exe::comp_screens_add_stage_selector(comp, stage, top)
}

/// The rows a text area of `bottom - top - 0x10` pixels holds in font 3 (signed division, the low byte kept).
pub(crate) unsafe fn league_tab_rows(top: i32, bottom: i32) -> u32 {
    ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as u32
}

/// One row of a small league's fixture list: the date (a button, id 8), the home club (yellow when human), the score
/// or "view"/"v", the away club, the stadium or city, "-----".
pub(crate) unsafe fn league_tab_fixture_row(comp: usize, area: u32, line: i32, f: usize) {
    let comp = League(comp);
    let f = Fixture(f);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
    exe::date_get_short_date_string(date.as_mut_ptr() as usize, JOB_TEXT, 0x7d0, 0xff);
    tel26_area_button([area, 0, line as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, JOB_TEXT as u32, 8, f.addr() as u32]);
    let side = |col: u32, club: usize| {
        comp_crt_format()(JOB_TEXT, 0x987ff0, club + club::SHORT_NAME); // "  %s"
        let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { yellow } else { pale };
        tel26_area_button([area, col, line as u32, 2, 0, 1, 2, colour, JOB_TEXT as u32, 0x7d5, club as u32]);
    };
    side(1, f.home_club().addr());
    let score = if (f.home_goals_extra_time() as u8) != 0xff {
        Some((fixture::HOME_GOALS_EXTRA_TIME, fixture::AWAY_GOALS_EXTRA_TIME))
    } else if (f.home_goals() as u8) != 0xff {
        Some((fixture::HOME_GOALS, fixture::AWAY_GOALS))
    } else {
        None
    };
    if let Some((h, a)) = score {
        comp_lang_format()(JOB_TEXT, 0x990bb4, read_i8(f.addr() + h) as i32, read_i8(f.addr() + a) as i32); // "%d:%d"
        let has_stats = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), f.match_stats_slot() as i32,
            f.home_club_id() as i32, f.away_club_id() as i32, f.day() as i32, f.year() as i32);
        if has_stats != 0 {
            tel26_area_button([area, 2, line as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32, 0x7d3, (f.match_stats_slot() as u32)]);
        } else {
            tel26_area_text([area, 2, line as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32]);
        }
    } else if exe::human_manager_is_watching_fixture(read_usize(G_HUMAN_MANAGER), f, 0usize) != 0 {
        comp_lang_format()(JOB_TEXT, 0x9918d4); // "view"
        tel26_area_button([area, 2, line as u32, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0x7d2, f.addr() as u32]);
    } else {
        comp_lang_format()(JOB_TEXT, 0x99c1b4); // "v"
        tel26_area_button([area, 2, line as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32, 0x7d2, f.addr() as u32]);
    }
    side(3, f.away_club().addr());
    let stadium = f.stadium();
    if comp.club_sides() == 1 {
        if stadium != 0 {
            comp_crt_format()(JOB_TEXT, 0x987ff0, stadium + stadium::NAME);
        } else {
            comp_lang_format()(JOB_TEXT, 0x99c1a4); // "Unknown Stadium"
        }
    } else if stadium != 0 && read_usize(stadium + stadium::CITY) != 0 {
        comp_crt_format()(JOB_TEXT, 0x987ff0, read_usize(stadium + stadium::CITY) + 4); // the city's name
    } else {
        comp_lang_format()(JOB_TEXT, 0x99c194); // "Unknown City"
    }
    tel26_area_text([area, 4, line as u32, 2, 0, 0xc, 2, job_colour(JOB_COLOUR_BDD4), JOB_TEXT as u32]);
    tel26_area_text([area, 5, line as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, pale, 0x99c18c]); // "-----"
}

/// The division (club competitions) or national team (national team competitions) column of a cup team, or an empty
/// cell: the division's three-letter name as a button (id 0x7d0) when its competition object is loaded, otherwise the
/// empty string; the nation's three-letter name as a button (id 0x7d5) with its national team; nothing else.
pub(crate) unsafe fn cup_tab_team_origin(comp: usize, area: u32, col: u32, line: i32, club: usize) {
    let comp = League(comp);
    let club = Club(club);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let blue = job_colour(G_GUI_COLOUR_DARK_BLUE);
    let sides = comp.club_sides();
    if sides == 1 && club.division().addr() != 0 {
        let division = club.division().addr();
        if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
            tel26_area_button([area, col, line as u32, 0x30, blue, 0xc, 1, pale, (division + 0x53) as u32, 0x7d0, division as u32]); // the three-letter name [inference]
        } else {
            tel26_area_text([area, col, line as u32, 0x30, blue, 0xc, 1, pale, 0xad9c64]);
        }
    } else if sides == 2 && club.nation().addr() != 0 {
        let nation = exe::database_get_national_team(club.nation().addr());
        tel26_area_button([area, col, line as u32, 0x30, blue, 0xc, 1, pale, (club.nation().addr() + nation::THREE_LETTER_NAME) as u32, 0x7d5, nation as u32]);
    } else {
        tel26_area_text([area, col, line as u32, 0x30, blue, 0xc, 1, pale, 0]);
    }
}

/// The club name of one of the possible teams of a cup side: the last one plain, the others with "or" after it
/// ("{}<%s - club name>{} or" with the gender byte twice), as a button (id 0x7d5, yellow when human).
pub(crate) unsafe fn cup_tab_team_name(area: u32, col: u32, line: i32, club: usize, last: bool) {
    let club = Club(club);
    if last {
        copy_c_string(JOB_TEXT, club.addr() + club::SHORT_NAME);
    } else {
        let g = (club.short_gender_name() as i8) as i32;
        comp_lang_format()(JOB_TEXT, 0x99c374, g, g, club.addr() + club::SHORT_NAME);
    }
    let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(G_GUI_COLOUR_PALE_GREY) };
    tel26_area_button([area, col, line as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, club.addr() as u32]);
}

/// The cup's team table entry `team` (6-byte entries: +0 the club, +5 the status).
pub(crate) unsafe fn cup_tab_team_club(cup_: usize, team: i16) -> usize {
    read_usize((read_usize(cup_ + cup::TEAM_TABLE) as isize + team as isize * 6) as usize)
}

/// The status byte of the last tie of round `round` (0x35-byte ties; 3 = decided).
pub(crate) unsafe fn cup_tab_last_tie_status(cup_: usize, round: i32) -> u8 {
    let ties = read_i16((read_usize(cup_ + cup::ROUND_TABLE) as isize + round as isize * 0x68 + cup_round::TIES as isize) as usize) as isize;
    let table = read_usize((read_usize(cup_ + cup::TIE_TABLES) as isize + round as isize * 4) as usize);
    read_u8((table as isize + ties * 0x35 - 1) as usize)
}

/// The score row of a played cup tie: columns 1 and 7 the home and away penalties (style 3) or the marks of byte +4/+5
/// ("w", "ww", or the negated value; style 1), 3 and 5 the result letters (s, p, e, a, w by +4/+5), and 4 the goals:
/// "x    " ":" "    y" (the extra-time, second-leg or first-leg goals), ":" a button to the match statistics (id
/// 0x7d3) when the tie's scheduled fixture has them.
pub(crate) unsafe fn cup_tab_score_row(cup_: usize, area: u32, line: i32, round: i32, k: i32, tie: usize) {
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let marks = job_colour(JOB_COLOUR_BDE8);
    let (mut away_pens, mut home_pens) = ([0u8; 8], [0u8; 8]);
    let (mut home_letter, mut away_letter) = ([0u8; 8], [0u8; 8]);
    let (mut home_goals, mut away_goals) = ([0u8; 8], [0u8; 8]);
    let d = |b: &mut [u8; 8], v: i8| { comp_crt_format()(b.as_mut_ptr() as usize, 0x98806c, v as i32); };
    let style: u32;
    if read_u8(tie + 0x26) != 0xff {
        d(&mut home_pens, read_i8(tie + 0x27));
        d(&mut away_pens, read_i8(tie + 0x26));
        style = 3;
    } else {
        for (b, at) in [(&mut home_pens, 4usize), (&mut away_pens, 5)] {
            let v = read_i8(tie + at);
            match v {
                1 => { comp_lang_format()(b.as_mut_ptr() as usize, 0x99c370); } // "w"
                2 => { comp_lang_format()(b.as_mut_ptr() as usize, 0x99c36c); } // "ww"
                v if v < 0 => { comp_crt_format()(b.as_mut_ptr() as usize, 0x98806c, -(v as i32)); }
                _ => {}
            }
        }
        style = 1;
    }
    for (b, at) in [(&mut home_letter, 4usize), (&mut away_letter, 5)] {
        let fmt = match read_u8(tie + at) {
            5 => 0x99c368usize, // "s"
            4 => 0x99c364, // "p"
            3 => 0x99c360, // "e"
            6 => 0x99c35c, // "a"
            8 => 0x99c370, // "w"
            _ => 0,
        };
        if fmt != 0 {
            comp_lang_format()(b.as_mut_ptr() as usize, fmt);
        }
    }
    let (h, a) = if read_u8(tie + 0x1e) == 0xff {
        if read_u8(tie + 0x18) != 0xff { (0x18, 0x19) } else { (0x14, 0x15) }
    } else if read_u8(tie + 0x28) == 0xff {
        if read_u8(tie + 0x22) != 0xff { (0x23, 0x22) } else { (0x1f, 0x1e) }
    } else if read_u8(tie + 0x2c) != 0xff {
        (0x2c, 0x2d)
    } else {
        (0x28, 0x29)
    };
    d(&mut home_goals, read_i8(tie + h));
    d(&mut away_goals, read_i8(tie + a));
    if home_pens[0] != 0 {
        tel26_area_text([area, 1, line as u32, 1, 0, 0xc, style, yellow, home_pens.as_ptr() as u32]);
    }
    if home_letter[0] != 0 {
        tel26_area_text([area, 3, line as u32, 1, 0, 0x40, 1, yellow, home_letter.as_ptr() as u32]);
    }
    let f = Fixture(exe::cup_find_scheduled_fixture_for_tie(cup_, round, k));
    let with_stats = !f.is_null() && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), f.match_stats_slot() as i32,
        f.home_club_id() as i32, f.away_club_id() as i32, f.day() as i32, f.year() as i32) != 0;
    comp_crt_format()(JOB_TEXT, 0x99c354, home_goals.as_ptr()); // "%s    "
    tel26_area_text([area, 4, line as u32, 1, 0, 0xc, 3, marks, JOB_TEXT as u32]);
    if with_stats {
        covered!("tel22_2 cup score with stats");
        tel26_area_button([area, 4, line as u32, 1, 0, 0xc, 3, marks, 0x989adc, 0x7d3, (f.match_stats_slot() as u32)]); // ":"
    } else {
        tel26_area_text([area, 4, line as u32, 1, 0, 0xc, 3, marks, 0x989adc]);
    }
    comp_crt_format()(JOB_TEXT, 0x99c34c, away_goals.as_ptr()); // "    %s"
    tel26_area_text([area, 4, line as u32, 1, 0, 0xc, 3, marks, JOB_TEXT as u32]);
    if away_letter[0] != 0 {
        tel26_area_text([area, 5, line as u32, 1, 0, 1, 1, yellow, away_letter.as_ptr() as u32]);
    }
    if away_pens[0] != 0 {
        tel26_area_text([area, 7, line as u32, 1, 0, 0xc, style, yellow, away_pens.as_ptr() as u32]);
    }
}

/// Restores the scroll row of a tab's list area from screen parameter `scroll_param` (when the area exists) and
/// keeps the area index (sign-extended) in `area_param`, as every list tab of the competition screen ends.
pub(crate) unsafe fn comp_tabs_restore_scroll(area: u32, scroll_param: u32, area_param: u32) {
    let scroll = get_screen_param(scroll_param);
    let display = current_human_display();
    if area as i16 != -1 {
        exe::area_set_scroll_row(display.wrapping_add(4).wrapping_add((area as i16 as isize * 0xbf1) as usize), scroll);
    }
    set_screen_param(area_param, area as i16 as isize as usize);
}

/// `Comp::are_round_fixtures_known(stage, round)` (0x48d410, thiscall, ret 8) through the exe address.
pub(crate) unsafe fn fixtures_tab_round_known(comp: usize, stage: i32, round: i32) -> u32 {
    exe::comp_are_round_fixtures_known(comp, stage, round)
}

/// `FIXTURE_MANAGER::scan_fixtures` (0x5acee0, thiscall, ret 0x48) as the Fixtures tab asks it: from the date
/// (`year`, `day`, both moved by the scan) and the slot, forward or not, the competition's matches (every club, round
/// and stage: −1), at most `count` (−2 all, −1 one), into `list`; returns how many were found (ax).
pub(crate) unsafe fn fixtures_tab_scan(year: &mut u32, day: &mut u32, slot: &mut u8, forward: u32, comp_id: u32, count: i32, list: usize) -> u16 {
    type Scan = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, i32, i32, i32, i32, i32,
                                            i32, usize, u32, i32, i32, u32) -> u32;
    let f: Scan = core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
    f(read_usize(G_FIXTURE_MANAGER), year, day, slot, forward, comp_id, -1, -1, -1, -1, -1, -1, -1, count, list, 0, -2, -1, 1) as u16
}

/// The neighbouring match date of the Fixtures tab: from (`day`, `year`) one day back (`forward` 0, slot 2) or ahead
/// (`forward` 1, slot 0) (`Date::sub_days`/`add_days` and `Date::operator=`), then the first match from there
/// (`fixtures_tab_scan`, one, into the local list `scratch`); none found leaves the year 1 (no button). The day and
/// year go to parameters `param` and `param + 1` (their words).
pub(crate) unsafe fn fixtures_tab_neighbour_date(day: u32, year: u32, forward: u32, comp_id: u32, scratch: usize, param: u32) -> u32 {
    let (mut d, mut t, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let dp = d.as_mut_ptr() as usize;
    exe::new_date(dp, day, year);
    let r = if forward != 0 {
        exe::date_add_days(dp, t.as_mut_ptr() as usize, 1)
    } else {
        exe::date_sub_days(dp, t.as_mut_ptr() as usize, 1)
    };
    exe::date_operator_3d(dp, t2.as_mut_ptr() as usize, r);
    let (mut n_day, mut n_year) = (read_u32(dp), read_u32(dp + 2));
    let mut slot: u8 = if forward != 0 { 0 } else { 2 };
    if fixtures_tab_scan(&mut n_year, &mut n_day, &mut slot, forward, comp_id, -1, scratch) == 0 {
        n_year = 1;
    }
    set_screen_param(param, (n_day & 0xffff) as usize);
    set_screen_param(param + 1, (n_year & 0xffff) as usize);
    n_year
}

/// A popup area for a View or Filter button (empty, flags 0x130, the popup colour, owned by the button).
pub(crate) unsafe fn team_stats_tab_popup_area(owner: u32) -> u32 {
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), owner])
}

/// An item of a stats popup list with the text in `JOB_TEXT`: `kind` 0x2002 (ticked) or 2, the popup colours, the id
/// and the value.
pub(crate) unsafe fn team_stats_tab_menu_item(kind: u32, id: u32, value: u32, area: u32) {
    tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), 1, 1,
        job_colour(G_GUI_COLOUR_BLACK), JOB_TEXT as u32, 0, id, value, area]);
}

/// `ScreenManager::get_widget_action_id` (0x468b40, thiscall, ret 4) of a widget of the current screen.
pub(crate) unsafe fn comp_click_action_id(widget: u32) -> i32 {
    exe::screenmanager_get_widget_action_id(G_SCREEN_MANAGER, widget)
}

/// `ScreenManager::get_widget_action_data` (0x468b90) of a widget of the current screen, as a signed number.
pub(crate) unsafe fn comp_click_action_data(widget: u32) -> i32 {
    exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) as i32
}

/// Keeps a list's scroll position: param `row` = the scroll row (`ScreenManager::get_area_scroll_row`) of the area in
/// param `area`.
pub(crate) unsafe fn comp_click_keep_scroll(row: u32, area: u32) {
    let r = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area));
    set_screen_param(row, r);
}

/// A competition screen routine called through its exe address (the hook routes it): cdecl(widget) -> eax.
pub(crate) unsafe fn comp_click_call(address: usize, widget: u32) -> i32 {
    let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(address);
    f(widget)
}

/// The menu bar's answer to a widget the screen does not handle: `menubar_handle_command`, and when it does not take
/// it, `menubar_handle_object_link`.
pub(crate) unsafe fn comp_click_menubar(widget: u32) -> i32 {
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// A stage and round picked (from a 1000 × (stage + 1) + round code, see `comp_click_split_code`): with `results` param 1 = 1 (the Results tab) first; when the stage (param 2) or the round (param 3) change,
/// they are written, param 4 = 0, the list of param 5 freed (`destructor`: through 0x497910, otherwise inline) and
/// param 6 = 0. Returns whether they changed.
pub(crate) unsafe fn comp_click_pick_round(stage: i32, round: i32, results: bool, destructor: bool) -> bool {
    if results {
        set_screen_param(1, 1);
    }
    if stage as usize == get_screen_param(2) && round as usize == get_screen_param(3) {
        return false;
    }
    set_screen_param(2, stage as usize);
    set_screen_param(3, round as usize);
    set_screen_param(4, 0);
    if destructor {
        comp_click_delete_list(5);
    } else {
        comp_free_list_param(5);
    }
    set_screen_param(6, 0);
    true
}

/// The stage and round of a 1000 × (stage + 1) + round code (division and remainder truncated, as the original's
/// `idiv`).
pub(crate) fn comp_click_split_code(code: i32) -> (i32, i32) {
    (code / 1000 - 1, code % 1000)
}

/// Writes the printout of the pending "print to file" command (`G_PENDING_COMMAND_CODE` = 0x39, only on a network
/// game, `COMP_CLICK_NETWORK_FLAG`): the command is cleared; with a file name (`G_PENDING_COMMAND_ARG`, then cleared)
/// the printout kind of param 0x2b is written into it (the last argument 1 with `COMP_CLICK_PRINTOUT_SAVE_OPTION`).
/// A failed write shows "The printout "<file>" could not be saved." ("Unable To Save", "Save Printout", OK only, the
/// competition's colours) and answers −4; the name is freed either way. `None` when the click goes on.
pub(crate) unsafe fn comp_click_pending_printout(comp: usize) -> Option<i32> {
    if read_u32(COMP_CLICK_NETWORK_FLAG) == 0 || read_u32(G_PENDING_COMMAND_CODE) != COMP_CLICK_PRINTOUT_DIALOG {
        return None;
    }
    let file = read_usize(G_PENDING_COMMAND_ARG);
    write_u32(G_PENDING_COMMAND_CODE, 0);
    if file == 0 {
        return None;
    }
    covered!("tel22_4 pending printout");
    write_u32(G_PENDING_COMMAND_ARG, 0);
    let option = (comp_click_options() & COMP_CLICK_PRINTOUT_SAVE_OPTION != 0) as u32;
    if comp_click_print(read_usize(comp + 4), get_screen_param(0x2b), file, option) != 0 {
        exe::crt_operator_delete(file);
        return None;
    }
    let (mut c1, mut c2) = (0u32, 0u32);
    exe::database_get_comp_colours(read_usize(comp + 4), &mut c1, &mut c2);
    let (mut heading, mut title) = ([0u8; 0xd0], [0u8; 0xd0]);
    let message = comp_lang_format()(JOB_TEXT, 0x99082c, file) as u32 as usize; // "The printout "<%s - file>" could not be saved."
    let h = comp_lang_format()(heading.as_mut_ptr() as usize, 0x99081c) as u32 as usize; // "Unable To Save"
    let t = comp_lang_format()(title.as_mut_ptr() as usize, 0x99080c) as u32 as usize; // "Save Printout"
    exe::confirm_push(t, h, message, 0, c1, c2, 0);
    exe::crt_operator_delete(file);
    Some(-4)
}

/// The Print button (id 0x19, the printout kind the widget's value): with `COMP_CLICK_PRINTOUT_STRAIGHT_AWAY` the
/// printout goes to the printer at once (answer 0); otherwise the kind goes to param 0x2b and the "Save Printout" /
/// "Enter File Name" file box opens on the human's printouts folder (`HUMAN_MANAGER::get_folder_path` of the screen
/// slot's human, kind 4), ".txt", the name buffer `COMP_CLICK_PRINTOUT_FILE_NAME`, the competition's colours and the
/// dialog code 0x39 (−4).
pub(crate) unsafe fn comp_click_print_button(comp: usize, w: i16) -> i32 {
    if comp_click_options() & COMP_CLICK_PRINTOUT_STRAIGHT_AWAY != 0 {
        covered!("tel22_4 printout straight away");
        comp_click_print(read_usize(comp + 4), tel26_item_value(w) as usize, 0, 1);
        return 0;
    }
    set_screen_param(0x2b, tel26_item_value(w) as usize);
    let (mut c1, mut c2) = (0u32, 0u32);
    exe::database_get_comp_colours(read_usize(comp + 4), &mut c1, &mut c2);
    let folder: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_FOLDER_PATH);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let dir = folder(read_usize(G_HUMAN_MANAGER), human, 4);
    let (mut heading, mut title) = ([0u8; 0xd0], [0u8; 0xd0]);
    let h = comp_lang_format()(heading.as_mut_ptr() as usize, 0x9907f4) as u32 as usize; // "Enter File Name"
    let t = comp_lang_format()(title.as_mut_ptr() as usize, 0x99080c) as u32 as usize; // "Save Printout"
    exe::open_file_dialog(t, h, 1, dir, 0x990804, COMP_CLICK_PRINTOUT_FILE_NAME, (0) as u32 as usize, (c1) as u32 as usize, (c2) as u32 as usize, COMP_CLICK_PRINTOUT_DIALOG, 0, 0, 0, 0); // ".txt"
    -4
}

/// The Results tab's clicks (tab 2): with a widget the list's scroll row is kept (param 0xf, area of param 0x10).
/// Action 0xc ("<< Date", the earlier match day): the shown day (params 8 and 9) moves to the "later" pair (0xc and
/// 0xd), the "earlier" pair (0xa and 0xb) becomes the shown day and is cleared; action 0xd ("Date >>") the mirror. Both
/// free the day's fixture list (param 0xe, through 0x497910) and clear its scroll row (param 0xf): −11. Action 0xe
/// picks a stage and round (`comp_click_pick_round`, through the destructor): −11, changed or not. Action 0xf opens the
/// league progress of the fixture in the widget's data (−4). Otherwise `None` (the menu bar).
pub(crate) unsafe fn comp_click_results_tab(widget: u32) -> Option<i32> {
    if widget as i16 != -1 {
        comp_click_keep_scroll(0xf, 0x10);
    }
    if comp_click_action_id(widget) == 0xc {
        covered!("tel22_4 results earlier day");
        set_screen_param(0xc, get_screen_param(8));
        set_screen_param(0xd, get_screen_param(9));
        set_screen_param(8, get_screen_param(0xa));
        set_screen_param(9, get_screen_param(0xb));
        set_screen_param(0xa, 0);
        set_screen_param(0xb, 0);
        comp_click_delete_list(0xe);
        set_screen_param(0xf, 0);
        return Some(-11);
    }
    if comp_click_action_id(widget) == 0xd {
        covered!("tel22_4 results later day");
        set_screen_param(0xa, get_screen_param(8));
        set_screen_param(0xb, get_screen_param(9));
        set_screen_param(8, get_screen_param(0xc));
        set_screen_param(9, get_screen_param(0xd));
        set_screen_param(0xc, 0);
        set_screen_param(0xd, 0);
        comp_click_delete_list(0xe);
        set_screen_param(0xf, 0);
        return Some(-11);
    }
    if comp_click_action_id(widget) == 0xe {
        let stage = comp_click_action_data(widget) / 1000 - 1;
        let round = comp_click_action_data(widget) % 1000;
        comp_click_pick_round(stage, round, true, true);
        return Some(-11);
    }
    if comp_click_action_id(widget) == 0xf {
        exe::comp_open_league_progress_for_fixture(comp_click_action_data(widget) as usize);
        return Some(-4);
    }
    None
}

/// The Team Stats tab's clicks (tab 7): with a widget the list's scroll row is kept (param 0x26, area of param 0x27);
/// action 0x16 picks the statistic (the widget's data) into param 0x23 when it changes, frees the sorted table of param
/// 0x24 (crt free, params 0x24 and 0x25 cleared) and answers −11. Otherwise `None` (the menu bar).
pub(crate) unsafe fn comp_click_team_stats_tab(widget: u32) -> Option<i32> {
    if widget as i16 != -1 {
        comp_click_keep_scroll(0x26, 0x27);
    }
    if comp_click_action_id(widget) != 0x16 {
        return None;
    }
    if comp_click_action_data(widget) as usize == get_screen_param(0x23) {
        return None;
    }
    covered!("tel22_4 team stat picked");
    set_screen_param(0x23, comp_click_action_data(widget) as usize);
    comp_free_block_param(0x24);
    Some(-11)
}

/// The competition screen's click comparer: side by side over the screen manager and the current screen's parameter
/// block (`screen_param_regions`) and the answer, when `pure` (the click only reads and writes params); otherwise only
/// the original runs.
pub(crate) unsafe fn comp_click_compare(name: &str, widget: u32, pure: bool, original: impl FnOnce() -> i32, rust: impl FnOnce() -> i32) -> i32 {
    if !pure {
        // simplification: opening screens, printing, the file box, making fixtures, freeing lists and the menu bar do
        // not run twice; those clicks are proven in Rust mode
        return original();
    }
    let mut regions = screen_param_regions();
    regions.push((G_SCREEN_MANAGER, 0x3840));
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("competition screen {}(widget {}): return {} x {}, differences in {:?}", name, widget as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// The competition screen's pick of a stage and round frees nothing: unchanged, or no list in param 5.
pub(crate) unsafe fn comp_click_pick_is_pure(code: i32) -> bool {
    let (stage, round) = comp_click_split_code(code);
    (stage as usize == get_screen_param(2) && round as usize == get_screen_param(3)) || get_screen_param(5) == 0
}

/// The 26 parameters 0..0x19 of the history screen as `comp_screens_open_history_screen` leaves them (`prev` = the
/// previous screen's 60 params, read before opening, used with `restore`).
pub(crate) unsafe fn comp_history_planned_params(comp: usize, restore: u32, prev: &[usize; 0x3c]) -> [usize; 0x1a] {
    let mut p = [0usize; 0x1a];
    p[0] = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    p[1] = 1;
    p[2] = usize::MAX;
    p[3] = usize::MAX;
    p[6] = 1;
    p[0xb] = 6;
    p[0xe] = 9;
    p[0x11] = 0xb;
    p[0x14] = 3;
    p[0x17] = 0xf;
    if restore != 0 {
        for k in [1usize, 6, 0xb, 0xe, 0x11, 0x14, 0x17] {
            p[k] = prev[k];
        }
    }
    p
}

/// The Records tab of the history screen (inline in the builder): the View button (period popup with the two periods
/// of `comp_stats_get_record_period_label`, the chosen one, param 6, ticked and copied without its leading space as
/// the heading), the heading, and one row per record type 1..0x16 that `Comp_Stats::is_record_type_shown`: its title
/// ("  %s", a button with id 4 and the type as value) and its text (`Comp_Stats::get_record_text` wrapped from x1 + 8
/// to 0x2ee in font 1, style 2); empty rows fill the visible ones; the scroll row from param 7, the area to param 8.
/// Kept from the original: with a view other than 1 and 2 the heading is stack garbage there (empty here).
/// Original: part of `comp_screens_draw_history_screen` (0x498a30, comp_screens.cpp).
pub(crate) unsafe fn comp_history_records_tab(object: usize, top: i32, bottom: i32) {
    let weights: [u8; 2] = [1, 2];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let view = get_screen_param(6) as u8;
    comp_lang_format()(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1,
                                   job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0, 0]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    write_u32(COMP_HISTORY_POPUP_FLAG, 1);
    let mut heading = [0u8; 0x7d0];
    let mut label = [0u8; 0xc8];
    let stats = read_usize(object + 8);
    for k in 1u32..=2 {
        exe::comp_stats_get_record_period_label(stats, k, label.as_mut_ptr(), 0xc8);
        comp_crt_format()(JOB_TEXT, 0x98ed14, label.as_ptr()); // " %s"
        if k == view as i8 as i32 as u32 {
            exe::screenmanager_add_ticked_popup_item(G_SCREEN_MANAGER, popup, (JOB_TEXT) as usize as u32, 3, k);
            tel26_copy_text(heading.as_mut_ptr(), (JOB_TEXT + 1) as *const u8);
        } else {
            exe::screenmanager_add_menu_item(G_SCREEN_MANAGER, popup, (JOB_TEXT) as usize as u32, 3, k);
        }
    }
    let top = top + 0x19;
    comp_text_item([0x6e, top as u32, 0x30c, (top + 0x23) as u32, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), heading.as_ptr() as u32, 1]);
    let top = top + 0x28;
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = comp_add_area([0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let (w1, w2) = (weights[0] as i32, weights[1] as i32);
    let x = w1 * 0x288 / (w1 + w2) + 0x6e;
    let mut row: i32 = 0;
    for kind in 1u32..=0x16 {
        if exe::comp_stats_is_record_type_shown(stats, kind) == 0 {
            continue;
        }
        exe::comp_stats_get_record_title(stats, kind, heading.as_mut_ptr(), 0x7d0);
        comp_crt_format()(JOB_TEXT, 0x987ff0, heading.as_ptr()); // "  %s"
        tel26_area_button([area, 0, row as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 4, kind]);
        exe::comp_stats_get_record_text(stats, view as u32, kind, t);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 1, 0, 0]);
        (exe::text_draw_wrapped((t) as *mut usize as usize, area, x + 8, 0x2ee, &mut row, 1, job_colour(G_GUI_COLOUR_YELLOW), 2, 0) as u32);
        row += 1;
    }
    while row < rows as i32 {
        tel26_area_text([area, 0, row as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_PALE_GREY), 0]);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 1, job_colour(G_GUI_COLOUR_PALE_GREY), 0]);
        row += 1;
    }
    exe::display_set_area_scroll_row(current_human_display(), area, get_screen_param(7));
    set_screen_param(8, area as u16 as i16 as isize as usize);
    exe::drop_text(t);
}

/// One of the history screen's list tabs (`tab` 3..7: Attendances, Results, Sequences, Teams, Players), the five
/// routines being the same code with the tab's numbers.
///
/// The tab's view (the record type shown) is param `0xb + 3 × (tab − 3)`, its scroll row the next param and its area
/// the one after. A "View" button (x 0x6e..0xeb, 0x14 high) with its popup (flags 0x130, [0xadae0c] = 1) lists the
/// record types 1..0x16 of this tab (`comp_history_tab_for_record`): the ones the competition keeps
/// (`Comp_Stats::is_record_type_shown`) as " <title>" items (id `tab + 2`, the type as value; the view ticked, flags
/// 0x2002, and its title without the leading space kept as the heading), the others greyed (flags 0x21, no id). The
/// heading in yellow font 3 (0x23 high) 0x19 below the top; 0x28 further down a 2-column area (weights 1, 8) of as many
/// rows as fit in font 2. The season records list (params 9 and 0xa) is built on the first use
/// (`Comp_Stats::build_season_records_list`); every record with a season gets a row: the season label
/// (`key_nation_format_comp_season`, dark blue, font 0xc) and the record's text for the view
/// (`comp_stats_format_record_text`) wrapped from x 0xbe to 0x2ee in yellow font 2. The rows left are filled with
/// empty yellow cells; the scroll row comes back and the area goes to its param.
/// Kept from the original: with no type ticked the heading is stack garbage there (empty here); the row count goes
/// as its byte (the original's dword carries the popup area's high bytes; Display keeps the byte).
pub(crate) unsafe fn history_list_tab(object: usize, top: i32, bottom: i32, table: u32) {
    let object = League(object);
    let view_param = 0xb + 3 * (table - 3);
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut list = get_screen_param(HISTORY_RECORDS_LIST);
    let mut count = get_screen_param(HISTORY_RECORDS_COUNT) as i32;
    let view = get_screen_param(view_param) as u8;
    comp_lang_format()(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1,
                                   job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0, 0]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    write_u32(COMP_SCREEN_DATE_BUTTONS_FLAG, 1);
    let stats = object.comp_stats();
    let mut heading = [0u8; 0x7d0];
    let mut title = [0u8; 0xc8];
    for kind in 1..=HISTORY_LAST_RECORD_TYPE {
        if comp_history_tab_for_record(kind, core::ptr::null_mut()) as u32 != table {
            continue;
        }
        let (c31aa, yellow, black) = (job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_BLACK));
        if comp_stats_record_shown(stats, kind) {
            comp_stats_record_title(stats, kind, title.as_mut_ptr() as usize, 0xc8);
            comp_crt_format()(JOB_TEXT, 0x98ed14, title.as_ptr()); // " %s"
            let ticked = kind as u8 == view;
            let flags = if ticked { 0x2002 } else { 2 };
            tel26_add_item(current_human_display(), [flags, 0, 0, 0, 0, 0, 0, 0x10, c31aa, yellow, 1, 1, black, JOB_TEXT as u32, 0, table + 2, kind, popup]);
            if ticked {
                tel26_copy_text(heading.as_mut_ptr(), (JOB_TEXT + 1) as *const u8);
            }
        } else {
            comp_stats_record_title(stats, kind, JOB_TEXT, 0x7d0);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, yellow, 0x21, 1, black, JOB_TEXT as u32, 0, 0, 0, popup]);
        }
    }
    let top = top + 0x19;
    comp_text_item([0x6e, top as u32, 0x30c, (top + 0x23) as u32, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), heading.as_ptr() as u32, 1]);
    let top = top + 0x28;
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 2, HISTORY_LIST_WEIGHTS.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    if list == 0 {
        covered!("tel23_1 history records list built");
        list = comp_stats_build_season_records(stats, &mut count);
        set_screen_param(HISTORY_RECORDS_LIST, list);
        set_screen_param(HISTORY_RECORDS_COUNT, count as usize);
    }
    for k in 0..count.max(0) as usize {
        let record = list + k * HISTORY_SEASON_RECORD_SIZE;
        let season = read_u16(record);
        if season == 0 {
            continue;
        }
        comp_season_label(season as u32, object.competition().addr(), JOB_TEXT, 0x7d0);
        tel26_area_text([area, 0, row as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
        comp_stats_season_record_text(stats, record + 2, view as u32, t);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, 0, 0]);
        history_draw_wrapped(t, area, 0xbe, 0x2ee, &mut row, 2, job_colour(G_GUI_COLOUR_YELLOW), 2); // tel23_1 the record text's x
        row += 1;
    }
    while row < rows as i32 {
        tel26_area_text([area, 0, row as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_YELLOW), 0]);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_YELLOW), 0]);
        row += 1;
    }
    let scroll = get_screen_param(view_param + 1);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), scroll);
    }
    set_screen_param(view_param + 2, area as i16 as isize as usize);
    exe::drop_text(t);
}

/// `history_screen_open` (0x5f11c0, cdecl): the written history screen of a competition by its English name.
pub(crate) unsafe fn history_screen_open_by_name(name: usize) {
    exe::history_screen_open(name)
}

/// The view param a list tab's popup id picks (ids 5..8: tabs 3..6, params 0xb, 0xe, 0x11, 0x14).
pub(crate) fn history_view_param(table: i8) -> u32 {
    0xb + 3 * (table as u32 - 3)
}

/// The comparer of a history list tab: side by side over the display list (plus [0xadae0c]) once the season records
/// list exists; its first build (it allocates the list into params 9 and 0xa) only the original.
pub(crate) unsafe fn compare_history_list_tab(name: &str, object: usize, original: impl FnOnce(), rust: impl FnOnce()) {
    if object == 0 || get_screen_param(HISTORY_RECORDS_LIST) == 0 {
        // simplification: the allocation does not run twice; proven in Rust mode
        return original();
    }
    compare_match_screen_build(name, &[(COMP_SCREEN_DATE_BUTTONS_FLAG, 4)], original, rust);
}

/// Whether a history screen click only reads and writes params (the view, tab and scroll params): not opening a
/// screen, not the error box of a type without a tab, not the menu bar.
pub(crate) unsafe fn history_click_is_pure(widget: u32) -> bool {
    let w = widget as i16;
    let table = get_screen_param(1) as u8 as i8;
    let id = comp_click_action_id(widget);
    let data = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget);
    match table {
        2 => {
            if id == 3 && exe::display_get_widget_data(current_human_display(), widget) != get_screen_param(6) {
                return true;
            }
            if id == 4 {
                let kind = data as u8 as i8;
                if !(1..=HISTORY_LAST_RECORD_TYPE as i8).contains(&kind) {
                    return false;
                }
                if kind != 1 {
                    return true;
                }
            }
        }
        3..=6 if id == table as i32 + 2 && data != get_screen_param(history_view_param(table)) => return true,
        7 if tel26_item_id(w) == 9 && tel26_item_value(w) as usize != get_screen_param(0x17) => return true,
        _ => {}
    }
    id == 2 && data != get_screen_param(1)
}

/// A competition's two colours (`database_get_comp_colours`).
pub(crate) unsafe fn comp_colours(comp: usize) -> (u32, u32) {
    let (mut c1, mut c2) = (0u32, 0u32);
    exe::database_get_comp_colours(comp, &mut c1, &mut c2);
    (c1, c2)
}

/// The screen title in a competition's colours (`add_screen_title`, the second colour first).
pub(crate) unsafe fn comp_colours_title(colours: (u32, u32), text: usize) {
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, colours.1, colours.0, text);
}

/// Area `area` of the current human's Display gets its scroll row back from param `param` (when there is an area).
pub(crate) unsafe fn comp_restore_area_scroll(area: u32, param: u32) {
    let scroll = get_screen_param(param);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), scroll);
    }
}

/// The five params of the league progress screen as `comp_open_league_progress_for_fixture` writes them.
pub(crate) fn league_progress_planned_params(league: usize, home: usize, away: usize) -> [usize; 5] {
    [league, home, away, 0, 0]
}

/// `Plot::Plot` (0x7c6fc0, thiscall, ret 0x14).
pub(crate) unsafe fn plot_new(p: usize, a: u32, clubs: i32, games: i32, b: u32, series: u32) -> usize {
    exe::new_plot(p, a, clubs, games, b, series)
}

/// A table row's position history as a plot series: one point per game played (+0x11), x the game (1..), y the
/// position after it (the history byte + 1).
pub(crate) unsafe fn league_progress_series(plot: usize, row: usize, series: u32, colour: u32) {
    let n = read_u8(row + league_table_row::PLAYED) as i8 as i32;
    plot_add_series(plot, n, colour, read_usize(row + league_table_row::CLUB) + club::SHORT_NAME);
    for k in 0..n.max(0) {
        let position = read_u8(read_usize(row + league_table_row::POSITION_HISTORY) + k as usize) as i8 as i32;
        plot_add_point(plot, series, k + 1, position + 1);
    }
}

/// `News::find_item_by_id` (0x786b70, thiscall, ret 0xc): the news item of ids (`a`, `b`) copied into `item`; zero
/// when there is none.
pub(crate) unsafe fn news_find_item(a: u32, b: u32, item: usize) -> u32 {
    exe::news_find_item_by_id(read_usize(G_NEWS), (a) as u32 as usize, b, item)
}

/// `News::set_item_slot` (0x786d30, thiscall, ret 0x14): slot `slot` of news item (`a`, `b`) = `value`.
pub(crate) unsafe fn news_set_item_slot(a: u32, b: u32, slot: u32, value: u32, extra: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    f(read_usize(G_NEWS), a, b, slot, value, extra)
}

/// The popup of a menu button (x 0x69..0x78, y 0xf..0x23, colours `colour_a`/`colour_b`, no text): the button
/// (`add_button_item`) and its popup area (flags 0x30); returns the area.
pub(crate) unsafe fn related_comps_popup(colour_a: u32, colour_b: u32) -> u32 {
    let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, colour_a, 0xc, 1, colour_b, 0, 0, 0]);
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, job_colour(JOB_COLOUR_31AA), button])
}

/// The rounds of a cup stage `s` (`cup::NEXT_DRAW_ROUND` of them, re-read each round) as items of the menu in `area`:
/// each round's code (+7 of its 0x68-byte record), preceded, at the round before the last when the last is the third
/// place play-off (code 0x8c), by that play-off.
pub(crate) unsafe fn stage_selector_cup_rounds(area: *mut i32, x1: u32, x2: u32, top: i32, text: usize, code: i32, stage: i32, s: usize) {
    let mut r: i32 = 0;
    while r < read_u8(s + cup::NEXT_DRAW_ROUND) as i8 as i32 {
        let rounds = read_i16(s + cup::ROUND_COUNT) as i32;
        let table = read_usize(s + cup::ROUND_TABLE);
        if r == rounds - 2 && read_i16((table as isize + (rounds - 1) as isize * cup_round::SIZE as isize) as usize + cup_round::ROUND_CODE) == 0x8c {
            comp_screens_add_stage_menu_item(area as *mut i16, x1, top, x2, top + 0x14, text, code, 0x8c, stage, rounds - 1);
        }
        let round_code = read_i16(table + r as usize * cup_round::SIZE + cup_round::ROUND_CODE) as i32;
        comp_screens_add_stage_menu_item(area as *mut i16, x1, top, x2, top + 0x14, text, code, round_code, stage, r);
        r += 1;
    }
}

/// The cell a fixture row gives one side's club beside its name (column 0 for the home club, 8 for the away club):
/// its league position, its MLS conference, its division or its nation.
///
/// No competition: an empty cell. A league fixture (the fixture's stage, up to the competition's current stage, a
/// league of a competition other than the MLS and not reserve or minor): the club's position in the stage's table
/// (`utils_format_ordinal`, a button `id` with the fixture; empty text when the club is not in it). The MLS: "E", "C"
/// or "W" (`usa_mls_get_club_division` 0x413, 0x47b, other) as a button 0x7e5 with (the MLS id << 22) | 0x209800,
/// 0x23d800 or 0x20a000. A friendly: between clubs of the same nation, the club's division (its +0x53 name, a button
/// 0x7d0, when the division has an object; else empty text) when it has one; otherwise a national team gets an empty
/// cell and a club its nation's three-letter name (a button 0x7d5 to the national team). Other competitions: played
/// by club sides (+0x42 = 1) with the side's mark (+0x40/+0x41) and a division, the division as above; by nations
/// (+0x42 = 2) the nation as above; else empty. Dark blue text, font 0xc, pale grey.
pub(crate) unsafe fn fixture_row_side_cell(fixture: usize, club: usize, mark: u8, column: u32, area: u32, row: i32, id: u32) {
    let fixture = Fixture(fixture);
    let club = Club(club);
    let cell = |text: usize, link: Option<(u32, usize)>| {
        let (dark_blue, pale) = (job_colour(G_GUI_COLOUR_DARK_BLUE), job_colour(G_GUI_COLOUR_PALE_GREY));
        match link {
            Some((id, value)) => {
                tel26_area_button([area, column, row as u32, 0x30, dark_blue, 0xc, 1, pale, text as u32, id, value as u32]);
            }
            None => {
                tel26_area_text([area, column, row as u32, 0x30, dark_blue, 0xc, 1, pale, text as u32]);
            }
        }
    };
    let division_cell = |division: usize| {
        if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
            cell(division + 0x53, Some((0x7d0, division)));
        } else {
            cell(0xad9c64, None);
        }
    };
    let nation_cell = |nation: usize| cell(nation + nation::THREE_LETTER_NAME, Some((0x7d5, exe::database_get_national_team(nation))));
    let comp = fixture.competition().addr();
    if comp == 0 {
        return cell(0, None);
    }
    let object = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4));
    let stage = fixture.stage_index() as i8 as i32;
    if stage <= (object.current_stage() as i32) {
        let s = League(exe::comp_get_stage(object, stage));
        if !s.is_null() && s.comp_type() == 2 && read_u32(comp) != read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)
            && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) == 0 {
            let mut position = [0u8; 0x40];
            let mut k: i32 = 0;
            while k < s.club_count() as i32 {
                let r = exe::league_get_table_row(s, k);
                if read_usize(r + league_table_row::CLUB) == club.addr() {
                    let ordinal = exe::utils_format_ordinal(read_u8(r + league_table_row::POSITION) as i8 as i32 + 1, 3);
                    tel26_copy_text(position.as_mut_ptr(), ordinal as *const u8);
                    break;
                }
                k += 1;
            }
            let p = position.as_ptr() as usize;
            return cell(p, if position[0] != 0 { Some((id, fixture.addr())) } else { None });
        }
    }
    let comp_id = read_u32(comp);
    if comp_id == read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        let mut conference = [0u8; 0x40];
        let (fmt, bits) = match exe::usa_mls_get_club_division(club) {
            0x413 => (0x99ca64usize, 0x209800u32), // "E<%s - COMMENT, Eastern Conference of the MLS>"
            0x47b => (0x99ca34, 0x23d800),          // "C<%s - COMMENT, Central Conference of the MLS>"
            _ => (0x99ca04, 0x20a000),              // "W<%s - COMMENT, Western Conference of the MLS>"
        };
        comp_lang_format()(conference.as_mut_ptr() as usize, fmt, 0xad9c64usize);
        return cell(conference.as_ptr() as usize, Some((0x7e5, ((read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) << 22) | bits) as usize)));
    }
    if comp_id == read_u32(KEY_CLUB_COMP_FRIENDLY) {
        let (home, away) = (fixture.home_club().addr(), fixture.away_club().addr());
        if read_usize(home + club::NATION) == read_usize(away + club::NATION) && club.division().addr() != 0 {
            return division_cell(club.division().addr());
        }
        if exe::database_is_national_team(club) != 0 {
            return cell(0, None);
        }
        return nation_cell(club.nation().addr());
    }
    match object.club_sides() {
        1 if club.division().addr() != 0 && mark != 0 => division_cell(club.division().addr()),
        2 if club.nation().addr() != 0 => nation_cell(club.nation().addr()),
        _ => cell(0, None),
    }
}

/// A fixture row's club name cell (column 2 home, 6 away): the club's name, yellow for a human's club, else pale
/// grey; a button 0x7d5 with the club when the side's mark is set, else 0x7d6.
pub(crate) unsafe fn fixture_row_club_cell(club: usize, mark: u8, column: u32, area: u32, row: i32) {
    let club = Club(club);
    let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(G_GUI_COLOUR_PALE_GREY) };
    let id = if mark != 0 { 0x7d5 } else { 0x7d6 };
    tel26_area_button([area, column, row as u32, 1, 0, 1, 3, colour, (club.addr() + club::SHORT_NAME) as u32, id, club.addr() as u32]);
}

/// A tie result mark as text: 1 "w", 2 "ww", a negative mark its absolute value; otherwise nothing.
pub(crate) unsafe fn fixture_result_mark(buf: &mut [u8; 8], mark: i8) {
    match mark {
        1 => {
            comp_lang_format()(buf.as_mut_ptr() as usize, 0x99c370); // "w"
        }
        2 => {
            comp_lang_format()(buf.as_mut_ptr() as usize, 0x99c36c); // "ww"
        }
        m if m < 0 => {
            comp_crt_format()(buf.as_mut_ptr() as usize, 0x98806c, -(m as i32)); // "%d"
        }
        _ => {}
    }
}

/// A result mark's event letter: 5 "s", 4 "p", 3 "e", 6 "a", 8 "w"; otherwise nothing.
pub(crate) unsafe fn fixture_event_letter(buf: &mut [u8; 8], mark: i8) {
    let fmt = match mark {
        5 => 0x99c368usize, // "s"
        4 => 0x99c364,      // "p"
        3 => 0x99c360,      // "e"
        6 => 0x99c35c,      // "a"
        8 => 0x99c370,      // "w"
        _ => return,
    };
    comp_lang_format()(buf.as_mut_ptr() as usize, fmt);
}

/// `Display::add_item` (0x55f990) by address on the Display `d`: the 18 fields; returns eax.
pub(crate) unsafe fn tel26_add_item(d: usize, f: DisplayItemFields) -> u32 {
    type AddItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddItem = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    g(d, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12], f[13], f[14], f[15], f[16], f[17])
}

/// `Display::add_area` (0x55fba0) by address on the Display `d`: x1, y1, x2, y2, columns, column weights, rows, and
/// four more fields; returns eax (the area index in ax).
pub(crate) unsafe fn tel26_add_area(d: usize, f: [u32; 11]) -> u32 {
    type AddArea = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddArea = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    g(d, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// `ScreenManager::add_area_text` (0x804100): area, column, row, five style fields, text.
pub(crate) unsafe fn tel26_area_text(f: [u32; 9]) -> u32 {
    type AreaText = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AreaText = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8])
}

/// `gui_add_text_area` (0x5ed070, cdecl): x1, y1, x2, y2, two colours, lines, font, colour, text, 0.
pub(crate) unsafe fn tel26_text_area(f: [u32; 11]) {
    let g: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    g(f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// The Back and Next buttons of a screen: `gui_add_back_next_buttons(back available, next available)`, the next
/// asked first (`is_forward_unavailable`, then `is_back_unavailable(0)`), like the originals.
pub(crate) unsafe fn tel26_back_next() {
    let next = (exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(G_SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, next);
}

/// The id of item `item` of the current human's display (0 and -1 become the default command id [0xdd6bde]).
pub(crate) unsafe fn tel26_item_id(item: i16) -> u16 {
    if item != -1 { display_item_id(item) } else { read_u16(G_CURRENT_COMMAND_ID) }
}

/// The value of item `item` of the current human's display (0 for -1).
pub(crate) unsafe fn tel26_item_value(item: i16) -> u32 {
    if item == -1 {
        return 0;
    }
    read_u32((current_human_display() as isize + item as isize * display_list::ITEM_SIZE as isize + display::ITEM_VALUE as isize) as usize)
}

/// A colour word zero-extended (the original pushes the register with only its low half loaded).
pub(crate) unsafe fn job_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

/// `Date::Date()` (0x54a110) on `at`.
pub(crate) unsafe fn comp_records_date(at: usize) {
    exe::date_date_54a110(at);
}

/// A record routine side by side over the object it builds or copies.
pub(crate) unsafe fn comp_records_compare(name: &str, at: usize, size: usize, original: impl FnOnce() -> usize, rust: impl FnOnce() -> usize) -> usize {
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(at, size)], original, rust);
    SCOREBOARD.record(same && r == n, || format!("{}({:#x}): differences {:?}", name, at, first_differences(&mo, &mr)));
    r
}

/// A label routine's body: a null buffer or one shorter than `min` gives the error box at `short_line` and 0; the
/// buffer is emptied, a known code writes its name (langlib) and returns 1, an unknown one gives the box at
/// `bad_line` and 0.
pub(crate) unsafe fn comp_stats_label(code: i8, out: usize, size: i32, min: i32, labels: &[usize], short_line: u32, bad_line: u32) -> u32 {
    if out == 0 || size < min {
        tel26_error_box(COMP_STATS_SOURCE_FILE, short_line);
        return 0;
    }
    write_u8(out, 0);
    let k = (code as i32).wrapping_sub(1) as u32 as usize;
    if k >= labels.len() {
        tel26_error_box(COMP_STATS_SOURCE_FILE, bad_line);
        return 0;
    }
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    langlib(out, labels[k]);
    1
}

/// The games of the player entry's club in this competition (`find_club_entry`, then team statistic 1), 0.0 without
/// an entry; a null club (the club table + id × 0x245 at 0) gives the error box of line 0x8e2.
pub(crate) unsafe fn comp_stats_club_games(this: usize, entry: usize) -> f64 {
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(entry + 8).wrapping_mul(club::SIZE));
    if club == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x8e2);
        return read_f32(rdata::F32_0) as f64;
    }
    let team = exe::comp_stats_find_club_entry(this, club);
    if team == 0 {
        return read_f32(rdata::F32_0) as f64;
    }
    comp_stats_get_team_entry_value(team, 1)
}

/// A label routine side by side over its buffer and its answer; the error cases (the box) the original only.
pub(crate) unsafe fn comp_stats_compare_label(name: &str, code: i8, out: usize, size: i32, min: i32, known: i8,
    original: unsafe extern "C" fn(i8, usize, i32) -> u32, copy: unsafe extern "C" fn(i8, usize, i32) -> u32) -> u32 {
    if out == 0 || size < min || code < 1 || code > known {
        return original(code, out, size);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, min as usize)], || original(code, out, size), || copy(code, out, size));
    SCOREBOARD.record(same && r == n, || format!("{}({}): differences {:?}", name, code, first_differences(&mo, &mr)));
    r
}

/// The game's sprintf (0x944c9f) on a statistic buffer.
pub(crate) unsafe fn comp_stats_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// A team's form, the five result bytes from `last` down to `last - 4` (3 won, 2 score draw, 1 drawn, -3 lost; any
/// other byte adds nothing), as letters appended to the emptied `out`. The four letters are translated first (langlib:
/// "W<..won>", "X<..score draw>", "D<..drawn>", "L<..lost>"), each into its 2-byte slot of the original's frame (L, W,
/// D, X from the frame's +0x10), in the original's order W, X, D, L.
pub(crate) unsafe fn comp_stats_form_text(out: usize, last: usize) {
    // ponytail: 16 bytes where the original has 8 (the counter follows), so a longer translation stays in the slots
    let mut marks = [0u8; 16];
    let m = marks.as_mut_ptr() as usize;
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    langlib(m + 2, 0x99ccb0usize, EMPTY_TEXT);
    langlib(m + 6, 0x99cc90usize, EMPTY_TEXT);
    langlib(m + 4, 0x99cc78usize, EMPTY_TEXT);
    langlib(m, 0x99cc60usize, EMPTY_TEXT);
    write_u8(out, 0);
    for k in 0..5 {
        let slot = match read_u8(last - k) {
            3 => 2,
            2 => 6,
            1 => 4,
            0xfd => 0,
            _ => continue,
        };
        append_text(out as *mut u8, marks.as_ptr().add(slot));
    }
}

/// A statistic text routine side by side over its buffer (0x33 bytes) and its answer; the error cases (the box) the
/// original only.
pub(crate) unsafe fn comp_stats_compare_stat_text(name: &str, code: i8, out: usize, size: i32, known: bool, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    if out == 0 || size < 0x33 || !known {
        return original();
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, 0x33)], original, copy);
    SCOREBOARD.record(same && r == n, || format!("{}({}): original={} rust={} differences {:?}", name, code, r, n, first_differences(&mo, &mr)));
    r
}

/// `comp_stats_get_team_entry_value` (0x4a2ec0, thiscall(stats, entry, stat byte), ret 8, float in st0).
pub(crate) unsafe fn team_entry_value(stats: usize, entry: usize, stat: u8) -> f64 {
    exe::comp_stats_get_team_entry_value(stats, entry, stat as u32)
}

/// `Comp_Stats::get_player_entry_value` (0x4a3250, thiscall(stats, entry, stat byte), ret 8, float in st0).
pub(crate) unsafe fn player_entry_value(stats: usize, entry: usize, stat: u8) -> f64 {
    exe::comp_stats_get_player_entry_value(stats, entry, stat as u32)
}

/// The failed reallocation of a statistics table: the comp_stats.CPP box at `line`, `game_fatal_cleanup`, the no-op
/// 0x5a8f60 and `exit(-1)`.
pub(crate) unsafe fn comp_stats_fatal(line: u32) -> ! {
    show_error_box(COMP_STATS_CPP, line);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
    unreachable!()
}

/// The three-way float order of the statistics sorts: `a` below `b` (or unordered) gives `below`, above gives
/// `-below`, equal gives `None` (the tie goes to the ids).
pub(crate) fn stat_order(a: f32, b: f32, below: i32) -> Option<i32> {
    if !(a >= b) {
        Some(below)
    } else if a > b {
        Some(-below)
    } else {
        None
    }
}

/// The tie-break of the statistics sorts: the first dword (the id), ascending, signed.
pub(crate) unsafe fn id_order(a: usize, b: usize) -> i32 {
    let (x, y) = (read_i32(a), read_i32(b));
    if x > y {
        1
    } else if x >= y {
        0
    } else {
        -1
    }
}

/// Statistic 0x11's value of a player entry: 0 when his club's games (team statistic 1) × 0.5 are more than his
/// appearances (player statistic 1), i.e. he played under half his club's games.
pub(crate) unsafe fn rated_player_value(player: usize, value: f32) -> f32 {
    let stats = read_usize(G_COMP_STATS_SORT_CONTEXT);
    let club_games = match player_club_entry(stats, player) {
        Some(e) => team_entry_value(stats, e, 1) as f32,
        None => 0.0,
    };
    let games = player_entry_value(read_usize(G_COMP_STATS_SORT_CONTEXT), player, 1);
    if club_games as f64 * 0.5 > games {
        covered!("tel31_1 rating below half the games");
        0.0
    } else {
        value
    }
}

/// Statistics 0xd..0x10's value of a player entry: 0 when his club's games (team statistic 1, 0 with no entry) × 35
/// are more than his appearances (`comp_stats_player::APPEARANCES`).
pub(crate) unsafe fn per_game_player_value(player: usize, value: f32) -> f32 {
    let stats = read_usize(G_COMP_STATS_SORT_CONTEXT);
    let club_games = match player_club_entry(stats, player) {
        Some(e) => team_entry_value(stats, e, 1),
        None => 0.0,
    };
    if club_games * 35.0 > read_i32(player + comp_stats_player::APPEARANCES) as f64 {
        covered!("tel31_1 per-game stat below the minimum");
        0.0
    } else {
        value
    }
}

/// The record's club name, as the original copies it (`club::SHORT_NAME`, a C string).
pub(crate) unsafe fn record_club_name(club: usize, buf: &mut [u8; 0xc8]) {
    let club = Club(club);
    let mut k = 0;
    loop {
        let c = read_u8(club.addr() + club::SHORT_NAME + k);
        buf[k] = c;
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// The club with the id at `rec + o`.
pub(crate) unsafe fn record_club(rec: usize, o: usize) -> usize {
    club_at(read_i32(rec + o) as isize).0
}

/// The gender byte of a club's short name (`club::SHORT_GENDER_NAME`), sign-extended, for the `{}` of the formats.
pub(crate) unsafe fn record_gender(club: usize) -> i32 {
    let club = Club(club);
    (club.short_gender_name() as i8) as i32
}

/// The season text of a record's year (`key_nation_format_comp_season` with the competition of the stats).
pub(crate) unsafe fn record_season(stats: usize, year: u16, buf: &mut [u8; 0x14]) {
    let f: RecordSeasonText = core::mem::transmute(FN_KEY_NATION_FORMAT_COMP_SEASON);
    f(year as u32, read_u32(read_usize(stats + comp_stats_obj::COMP) + 4), buf.as_mut_ptr(), 0x14);
}

/// The person name of a record holder (first, second and common name ids at +0, +4, +8, the person at +0xc) into
/// `buf`, with the name `style` of `database_format_name_from_ids`.
pub(crate) unsafe fn record_holder_name(holder: usize, buf: &mut [u8; 0xc8], style: u32) {
    exe::database_format_name_from_ids(read_u32(holder), read_u32(holder + 4), read_u32(holder + 8), buf.as_mut_ptr(), 0xc8, style,
        read_u32(holder + 0xc));
}

/// The person link of a record holder: only when the person still has those names (`database_staff_matches_names`
/// with the three ids and the date at +0x10).
pub(crate) unsafe fn record_holder_link(text: usize, holder: usize, label: &[u8; 0xc8]) {
    let person = staff_at(read_i32(holder + 0xc) as isize).0;
    if exe::database_staff_matches_names(person, read_u32(holder), read_u32(holder + 4), read_u32(holder + 8), holder + 0x10) != 0 {
        let person = staff_at(read_i32(holder + 0xc) as isize).0;
        exe::text_add_link(text, label.as_ptr(), LINK_PERSON, person);
    }
}

/// A team record of one number: `value` - team (gender twice, the value, the name and, with a year, the season),
/// the club linked.
pub(crate) unsafe fn record_team_value(stats: usize, text: usize, rec: usize, value: i32, club_at_: usize, year_at: usize, with_season: usize, without: usize) {
    let mut out = [0u8; 0x200];
    let mut name = [0u8; 0xc8];
    let mut season = [0u8; 0x14];
    let club = record_club(rec, club_at_);
    record_club_name(club, &mut name);
    let gender = record_gender(club);
    let year = read_u16(rec + year_at);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if year > 0 {
        record_season(stats, year, &mut season);
        f(out.as_mut_ptr(), with_season, gender, gender, value, name.as_ptr(), season.as_ptr());
    } else {
        f(out.as_mut_ptr(), without, gender, gender, value, name.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(rec, club_at_));
}

/// An attendance record (kinds 6 and 7): the attendance at `rec + at` and the match record after it (date +0,
/// competition +8, round +0xc/+0xe, the club +0x11 and the opponent +0x15).
pub(crate) unsafe fn record_attendance(text: usize, rec: usize, at: usize) {
    let m = rec + at + 4;
    let mut out = [0u8; 0x200];
    let (mut home, mut away, mut stage) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let mut date = [0u8; 0x14];
    let (hc, ac) = (record_club(m, 0x11), record_club(m, 0x15));
    record_club_name(hc, &mut home);
    record_club_name(ac, &mut away);
    let (hg, ag) = (record_gender(hc), record_gender(ac));
    record_stage(m, &mut stage);
    record_date(m, &mut date);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let attendance = read_i32(rec + at);
    if stage[0] != 0 {
        f(out.as_mut_ptr(), 0x99d62c, hg, hg, ag, ag, 0x64, attendance, home.as_ptr(), away.as_ptr(), stage.as_ptr(), date.as_ptr());
    } else {
        f(out.as_mut_ptr(), 0x99d5dc, hg, hg, ag, ag, 0x64, attendance, home.as_ptr(), away.as_ptr(), date.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, home.as_ptr(), LINK_CLUB, record_club(m, 0x11));
    exe::text_add_link(text, away.as_ptr(), LINK_CLUB, record_club(m, 0x15));
    if stage[0] != 0 {
        record_stage_link(text, m, &stage);
    }
}

/// A result record (kinds 9 and 10): the match record at `rec + at` (date +0, competition +8, round +0xc/+0xe, the
/// club +0x11, the opponent +0x15, the goals +0x19/+0x1a, the match statistics slot +0x1b); with `venue` 2 the two
/// names change places in the text (the genders do not).
pub(crate) unsafe fn record_result(text: usize, rec: usize, at: usize, venue: u8) {
    let m = rec + at;
    let mut out = [0u8; 0x200];
    let mut score = [0u8; 0x14];
    let (mut home, mut away, mut stage) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let mut date = [0u8; 0x14];
    let crt: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    crt(score.as_mut_ptr(), 0x98c114, read_i8(m + 0x19) as i32, read_i8(m + 0x1a) as i32);
    let (hc, ac) = (record_club(m, 0x11), record_club(m, 0x15));
    record_club_name(hc, &mut home);
    record_club_name(ac, &mut away);
    let (hg, ag) = (record_gender(hc), record_gender(ac));
    record_stage(m, &mut stage);
    record_date(m, &mut date);
    let (first, second) = if venue == 2 { (away.as_ptr(), home.as_ptr()) } else { (home.as_ptr(), away.as_ptr()) };
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if stage[0] != 0 {
        f(out.as_mut_ptr(), 0x99d4f4, hg, hg, ag, ag, 0x64, score.as_ptr(), first, second, stage.as_ptr(), date.as_ptr());
    } else {
        f(out.as_mut_ptr(), 0x99d4a8, hg, hg, ag, ag, 0x64, score.as_ptr(), first, second, date.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, home.as_ptr(), LINK_CLUB, record_club(m, 0x11));
    exe::text_add_link(text, away.as_ptr(), LINK_CLUB, record_club(m, 0x15));
    if stage[0] != 0 {
        record_stage_link(text, m, &stage);
    }
    let slot = read_i32(m + 0x1b);
    if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot, read_i32(m + 0x11), read_i32(m + 0x15),
        read_i16(m) as i32, read_i16(m + 2) as i32) != 0 {
        exe::text_add_link(text, score.as_ptr(), LINK_MATCH, read_u32(m + 0x1b) as usize);
    }
}

/// A run record (kinds 11 to 14) at `rec + at`: the games (+0), the start (+4) and end (+0xc) dates and the club
/// (+0x14). One game: only the number and the team. The unbeaten and winless runs (`ongoing` = the club entry's run
/// offset) end in "?" while the club's current run is this one (same games and same start date).
pub(crate) unsafe fn record_run(stats: usize, text: usize, rec: usize, at: usize, ongoing: Option<usize>, gendered_dates: bool, formats: (usize, usize)) {
    let r = rec + at;
    let mut out = [0u8; 0x200];
    let mut name = [0u8; 0xc8];
    let (mut start, mut end) = ([0u8; 0x14], [0u8; 0x14]);
    let club = record_club(r, 0x14);
    record_club_name(club, &mut name);
    let gender = record_gender(club);
    let games = read_i32(r);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if games > 1 {
        record_date(r + 4, &mut start);
        let mut current = false;
        if let Some(run) = ongoing {
            let entry = exe::comp_stats_find_club_entry(stats, record_club(r, 0x14));
            current = entry != 0 && read_i32(r) == read_i32(entry + run)
                && exe::date_operator_3d_3d(r + 4, read_u32(entry + run + 4), read_u32(entry + run + 8)) != 0;
        }
        if current {
            covered!("tel31_2 run still going");
            end[0] = b'?';
            end[1] = 0;
        } else {
            record_date(r + 0xc, &mut end);
        }
        if gendered_dates {
            f(out.as_mut_ptr(), formats.0, gender, gender, 0x64, read_i32(r), name.as_ptr(), start.as_ptr(), end.as_ptr());
        } else {
            f(out.as_mut_ptr(), formats.0, gender, gender, read_i32(r), name.as_ptr(), start.as_ptr(), end.as_ptr());
        }
    } else {
        f(out.as_mut_ptr(), formats.1, gender, gender, games, name.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(r, 0x14));
}

/// A player record with a count (kinds 17, 19 and 20) at `rec + at`: the counts, the holder (+1 or +2), the club and
/// the year after the holder.
pub(crate) unsafe fn record_player_count(stats: usize, text: usize, rec: usize, at: usize, two_counts: bool, formats: (usize, usize)) {
    let r = rec + at;
    let holder = r + if two_counts { 2 } else { 1 };
    let mut out = [0u8; 0x200];
    let (mut name, mut team) = ([0u8; 0xc8], [0u8; 0xc8]);
    let mut season = [0u8; 0x14];
    record_holder_name(holder, &mut name, 1);
    let club = record_club(holder, 0x18);
    record_club_name(club, &mut team);
    let year = read_u16(holder + 0x1c);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (a, b) = (read_i8(r) as i32, read_i8(r + 1) as i32);
    if year > 0 {
        record_season(stats, year, &mut season);
        if two_counts {
            f(out.as_mut_ptr(), formats.0, name.as_ptr(), team.as_ptr(), a, b, season.as_ptr());
        } else {
            f(out.as_mut_ptr(), formats.0, a, name.as_ptr(), team.as_ptr(), season.as_ptr());
        }
    } else if two_counts {
        f(out.as_mut_ptr(), formats.1, name.as_ptr(), team.as_ptr(), a, b);
    } else {
        f(out.as_mut_ptr(), formats.1, a, name.as_ptr(), team.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, team.as_ptr(), LINK_CLUB, record_club(holder, 0x18));
    record_holder_link(text, holder, &name);
}

/// A player record against an opponent (kind 16: goals in a match; kinds 21 and 22: the age): the holder, the match
/// record after it (date, competition, round, venue, club and opponent), the club linked, the opponent linked, the
/// stage linked and the person linked.
pub(crate) unsafe fn record_player_match(text: usize, holder: usize, m: usize, lead: RecordLead) {
    let mut out = [0u8; 0x200];
    let (mut name, mut team, mut opponent, mut stage) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let (mut venue, mut date, mut age) = ([0u8; 0x14], [0u8; 0x14], [0u8; 0x14]);
    record_holder_name(holder, &mut name, 2);
    if let RecordLead::Age(birth) = lead {
        database_format_years_between_dates(m, birth, age.as_mut_ptr() as usize, 5);
    }
    record_club_name(record_club(m, 0x11), &mut team);
    let oc = record_club(m, 0x15);
    record_club_name(oc, &mut opponent);
    let og = record_gender(oc);
    let marker: RecordVenueMarker = core::mem::transmute(FN_FORMAT_VENUE_MARKER);
    marker(read_u8(m + 0x10) as u32, venue.as_mut_ptr(), 5);
    record_stage(m, &mut stage);
    record_date(m, &mut date);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match lead {
        RecordLead::Goals(goals) => {
            if stage[0] != 0 {
                f(out.as_mut_ptr(), 0x99d190, og, og, 0x64, goals, name.as_ptr(), team.as_ptr(), opponent.as_ptr(), venue.as_ptr(),
                  stage.as_ptr(), date.as_ptr());
            } else {
                f(out.as_mut_ptr(), 0x99d11c, og, og, 0x64, goals, name.as_ptr(), team.as_ptr(), opponent.as_ptr(), venue.as_ptr(),
                  date.as_ptr());
            }
        }
        RecordLead::Age(_) => {
            if stage[0] != 0 {
                f(out.as_mut_ptr(), 0x99cdd4, og, og, 0x64, name.as_ptr(), team.as_ptr(), age.as_ptr(), opponent.as_ptr(), venue.as_ptr(),
                  stage.as_ptr(), date.as_ptr());
            } else {
                f(out.as_mut_ptr(), 0x99cd64, og, og, 0x64, name.as_ptr(), team.as_ptr(), age.as_ptr(), opponent.as_ptr(), venue.as_ptr(),
                  date.as_ptr());
            }
        }
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, team.as_ptr(), LINK_CLUB, record_club(m, 0x11));
    exe::text_add_link(text, opponent.as_ptr(), LINK_CLUB, record_club(m, 0x15));
    if stage[0] != 0 {
        record_stage_link(text, m, &stage);
    }
    record_holder_link(text, holder, &name);
}

/// `ScreenManager::add_area` (0x4567f0, thiscall, ret 0x2c) through the exe address (the hook routes it): x1, y1,
/// x2, y2, the columns, the column weights, the rows and four more fields; returns the area index (ax).
pub(crate) unsafe fn comp_add_area(f: [u32; 11]) -> u32 {
    type AddArea = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddArea = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// `ScreenManager::set_area_scroll_row` (0x4567a0, thiscall, ret 8) through the exe address.
pub(crate) unsafe fn comp_set_area_scroll_row(area: u32, row: u32) {
    exe::screenmanager_set_area_scroll_row(G_SCREEN_MANAGER, area, row)
}

/// `ScreenManager::add_item` (0x45cb60, thiscall, ret 0x48) through the exe address: the 18 fields of an item.
pub(crate) unsafe fn comp_add_item(f: [u32; 18]) -> u32 {
    type AddItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddItem = core::mem::transmute(FN_SCREENMANAGER_ADD_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12], f[13], f[14], f[15], f[16], f[17])
}

/// `FIXTURE_MANAGER::scan_fixtures` (0x5acee0, thiscall, ret 0x48) as the Results tab asks it: from the date (`year`,
/// `day`, both moved to the match day found) and the mode byte, forward or not, the competition's matches (every club,
/// round and stage: −1), all of that day (−2), into `list`; returns how many (ax).
pub(crate) unsafe fn comp_screen_scan(year: &mut u32, day: &mut u32, mode: &mut u8, forward: u32, comp_id: u32, list: usize) -> u16 {
    type Scan = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, i32, i32, i32, i32, i32,
                                            i32, usize, u32, i32, i32, u32) -> u32;
    let f: Scan = core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
    f(read_usize(G_FIXTURE_MANAGER), year, day, mode, forward, comp_id, -1, -1, -1, -1, -1, -1, -1, -2, list, 1, -2, -1, 1) as u16
}

/// The match day before (`forward` 0, mode 2) or after (`forward` 1, mode 0) the date (`day`, `year`) for the Results
/// tab's arrows: `Date::Date`, one day back or ahead (`Date::sub_days`/`add_days` into `out`) and `Date::operator=`
/// (into `other`), then the first match day from there (`comp_screen_scan` into the local `list`). Returns the found
/// day and year dwords (the date's +0 and +2 dwords as the scan left them); none found leaves the year 1.
pub(crate) unsafe fn comp_screen_neighbour_day(day: u32, year: u32, forward: u32, comp_id: u32, list: usize, date: &mut [u8; 8], out: &mut [u8; 8], other: &mut [u8; 8]) -> (u32, u32) {
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    let moved = if forward != 0 {
        exe::date_add_days(date.as_mut_ptr() as usize, out.as_mut_ptr() as usize, 1)
    } else {
        exe::date_sub_days(date.as_mut_ptr() as usize, out.as_mut_ptr() as usize, 1)
    };
    exe::date_operator_3d(date.as_mut_ptr() as usize, other.as_mut_ptr() as usize, moved);
    let mut found_day = u32::from_le_bytes([date[0], date[1], date[2], date[3]]);
    let mut found_year = u32::from_le_bytes([date[2], date[3], date[4], date[5]]);
    let mut mode: u8 = if forward != 0 { 0 } else { 2 };
    if comp_screen_scan(&mut found_year, &mut found_day, &mut mode, forward, comp_id, list) == 0 {
        found_year = 1;
    }
    (found_day, found_year)
}

/// The flag dword of the game options ([G_CONFIG] + 4).
pub(crate) unsafe fn comp_click_options() -> u32 {
    read_u32(read_usize(G_CONFIG) + 4)
}

/// Frees the list of param `k` through `LLIST::scalar_deleting_destructor(1)` (0x497910) when there is one and clears
/// the param (the main handler's way; the tab handlers destroy and delete inline, `comp_free_list_param`).
pub(crate) unsafe fn comp_click_delete_list(k: u32) {
    if get_screen_param(k) == 0 {
        return;
    }
    let list = get_screen_param(k);
    if list != 0 {
        exe::llist_scalar_deleting_destructor(list, 1);
    }
    set_screen_param(k, 0);
}

/// `printouts_print_comp_report` (0x7d7c80, cdecl): the competition's printout `kind` into `file` (0 = to the
/// printer) with the option `option`; nonzero when it was written.
pub(crate) unsafe fn comp_click_print(comp_record: usize, kind: usize, file: usize, option: u32) -> u32 {
    exe::printouts_print_comp_report(comp_record, kind, file, option)
}

/// `Comp_Stats::is_record_type_shown` (0x4a54c0, thiscall, ret 4): whether the competition keeps records of `kind`.
pub(crate) unsafe fn comp_stats_record_shown(stats: usize, kind: u32) -> bool {
    exe::comp_stats_is_record_type_shown(stats, kind) != 0
}

/// `comp_stats_get_record_title` (0x4a5120, thiscall, ret 0xc): the title of record type `kind` into `buf`.
pub(crate) unsafe fn comp_stats_record_title(stats: usize, kind: u32, buf: usize, size: u32) {
    exe::comp_stats_get_record_title(stats, kind, (buf) as usize as *mut u8, size)
}

/// `Comp_Stats::build_season_records_list` (0x4ab0c0, thiscall, ret 4): a new heap list of the competition's season
/// records, its count into `count`.
pub(crate) unsafe fn comp_stats_build_season_records(stats: usize, count: &mut i32) -> usize {
    exe::comp_stats_build_season_records_list(stats, count)
}

/// `comp_stats_format_record_text` (0x4a5690, thiscall, ret 0xc): the text of a season record for record type `kind`
/// (its low byte) into the Text object.
pub(crate) unsafe fn comp_stats_season_record_text(stats: usize, record: usize, kind: u32, text: *mut usize) {
    exe::comp_stats_format_record_text(stats, record, kind, text)
}

/// `key_nation_format_comp_season` (0x6697c0, cdecl): the season label of `season` for the competition record.
pub(crate) unsafe fn comp_season_label(season: u32, comp: usize, buf: usize, size: u32) {
    let f: unsafe extern "C" fn(u32, usize, usize, u32) = core::mem::transmute(FN_KEY_NATION_FORMAT_COMP_SEASON);
    f(season, comp, buf, size)
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): the Text into an area from x1 to x2 starting at `row` (moved
/// to its last row), in a font, colour and style.
pub(crate) unsafe fn history_draw_wrapped(text: *mut usize, area: u32, x1: i32, x2: i32, row: &mut i32, font: u32, colour: u32, style: u32) {
    (exe::text_draw_wrapped((text) as *mut usize as usize, area, x1, x2, row, font, colour, style, 0) as u32);
}

/// The club entry of a player entry's club in the sort stats, or `None` (no club: the comp_stats.CPP 2274 box).
pub(crate) unsafe fn player_club_entry(stats: usize, player: usize) -> Option<usize> {
    let club = club_at(read_i32(player + comp_stats_player::CLUB) as isize).0;
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0x8e2);
        return None;
    }
    let entry = exe::comp_stats_find_club_entry(stats, club);
    if entry == 0 { None } else { Some(entry) }
}

/// The short date of the date at `date` into `buf`.
pub(crate) unsafe fn record_date(date: usize, buf: &mut [u8; 0x14]) {
    let f: RecordDateString = core::mem::transmute(FN_DATE_GET_SHORT_DATE_STRING);
    f(date, buf.as_mut_ptr(), 0x14, 0xff);
}

/// The stage and round name of a match record (round prefix and name words at `+0xc`, `+0xe`) into `buf`.
pub(crate) unsafe fn record_stage(m: usize, buf: &mut [u8; 0xc8]) {
    exe::comp_util_format_stage_round_name(read_i16(m + 0xc) as i32, read_i16(m + 0xe) as i32, buf.as_mut_ptr() as usize, 0xc8, 1);
}

/// The competition link of a match record's stage (the competition id at `m + 8`).
pub(crate) unsafe fn record_stage_link(text: usize, m: usize, stage: &[u8; 0xc8]) {
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(m + 8) as usize).wrapping_mul(0x6b));
    exe::text_add_link(text, stage.as_ptr(), LINK_COMPETITION, comp);
}

pub(crate) const COMP_STATS_LATE_BLOCK_VERSION: i32 = 0x3e007;

/// The bytes of a player entry in the files before `COMP_STATS_WHOLE_PLAYER_VERSION`.
pub(crate) const COMP_STATS_OLD_PLAYER_BYTES: u32 = 0x1d;

/// The size of a youngest or oldest player record (Comp_Stats and Club_Records).
pub(crate) const PLAYER_AGE_RECORD_SIZE: usize = 0x3f;

/// The size of an attendance record: the attendance and the match record (0x1f bytes) after it.
pub(crate) const ATTENDANCE_RECORD_SIZE: usize = 4 + 0x1f;

/// `Comp_Stats` +0x566: the temporary club entry `get_club_projected_positions` lends to the stage's projection; while it
/// is set, `find_club_entry` returns it for any club.
pub(crate) const COMP_STATS_PROJECTED_ENTRY: usize = 0x566;

/// The player record of `G_PLAYER_DATA_TABLE` (0x46 bytes), indexed by the id at +0 of a player entry.
pub(crate) const COMP_STATS_PLAYER_SHEET_SIZE: usize = 0x46;

/// The record kinds the season end flags: most titles (1), most points in a season (8) and the best average
/// rating (0x12).
pub(crate) const COMP_STATS_KIND_MOST_TITLES: usize = 1;

pub(crate) const COMP_STATS_KIND_MOST_POINTS: usize = 8;

pub(crate) const COMP_STATS_KIND_RATING: usize = 0x12;

/// The size of a news item on the stack (`NewsItem::NewsItem`, 0xe0 bytes).
pub(crate) const COMP_STATS_NEWS_ITEM_BYTES: usize = 0xe0;

/// The news type of a broken competition record.
pub(crate) const COMP_STATS_RECORD_NEWS_TYPE: u32 = 0x3e9;

/// The size of the `Comp_Stats` object.
pub(crate) const COMP_STATS_OBJECT_SIZE: usize = 0x5ce;

/// The sub-record constructors of the all-time records (thiscall, ret 0; each writes only inside its sub-record), by
/// their offset in the records block: the four match records (`FixedFixture::FixedFixture`), the two runs
/// (`run_record_construct`), the two other runs, the three tallies, the goals-in-match record, the rating record, the
/// discipline record and the two age records.
pub(crate) const COMP_STATS_ALL_TIME_CONSTRUCTORS: [(usize, usize); 16] = [
    (0x29, 0x645800), (0x4c, 0x645800), (0x75, 0x645800), (0x94, 0x645800),
    (0xb3, 0x448670), (0xcb, 0x448670), (0xe3, 0x49f2a0), (0xfb, 0x49f2f0),
    (0x113, 0x450c60), (0x132, 0x49f340), (0x16a, 0x450c60), (0x189, 0x453c50),
    (0x1ac, 0x450c60), (0x1cb, 0x450d30), (0x1eb, 0x49f390), (0x22a, 0x49f3f0),
];

/// `comp_season_records_construct` (0x49f160, thiscall, ret 0): the constructors of a season records block.
pub(crate) const COMP_STATS_SEASON_RECORDS_CONSTRUCT: usize = 0x49f160;

/// How many `Comp_Stats` objects are alive; the first opens the shared history file, the last closes it.
pub(crate) const COMP_STATS_INSTANCES: usize = 0xadae18;

/// `FILE_LLIST::FILE_LLIST` (0x58e280, thiscall(list, path, record size), ret 8) and `FILE_LLIST::~FILE_LLIST`
/// (0x58e330, thiscall, ret 0).
pub(crate) const FILE_LLIST_CONSTRUCT: usize = 0x58e280;

pub(crate) const FILE_LLIST_DESTRUCT: usize = 0x58e330;

/// "CM3_TEMP" and "comp records history.tmp", and the record size of the history file (a season row).
pub(crate) const COMP_STATS_TEMP_DIR: usize = 0x987da0;

pub(crate) const COMP_STATS_HISTORY_FILE: usize = 0x99cac8;

/// The `Comp_Stats` fields the constructor clears besides the tables: the dword block +0x506..+0x562 (the 22 record
/// flags at +0x50a among them), the "any flag" dword, the season end count and the block +0x56a..+0x5ce (0x64
/// bytes; saved since data version 0x3e008).
pub(crate) const COMP_STATS_FLAGS_BLOCK: usize = 0x506;

pub(crate) const COMP_STATS_FLAGS_BLOCK_SIZE: usize = 0x5c;

pub(crate) const COMP_STATS_ANY_FLAG: usize = 0x562;

pub(crate) const COMP_STATS_SEASONS_ENDED: usize = 0x500;

pub(crate) const COMP_STATS_LATE_BLOCK: usize = 0x56a;

pub(crate) const COMP_STATS_LATE_BLOCK_SIZE: usize = 0x64;

/// The lent projection entry (+0x566, see `get_club_projected_positions`).
pub(crate) const COMP_STATS_PROJECTED_ENTRY_SLOT: usize = 0x566;

/// The two competition virtuals of the projection: slot 0x80, whether the competition (or stage) holds the club
/// [inference: read from this caller], and slot 0x84, `League::update_projected_positions` (cpr_11), which writes the
/// projected positions into the club's statistics entry (here, the lent one).
pub(crate) const COMP_VT_HOLDS_CLUB: usize = 0x80;

pub(crate) const COMP_VT_UPDATE_PROJECTED_POSITIONS: usize = 0x84;

/// `Date::Date` (0x54a110, thiscall, ret 0): the default date (day 0x28, year 0x7b4, 1).
pub(crate) const COMP_STATS_DATE_CONSTRUCT: usize = 0x54a110;

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp_screens.cpp", the path the comp_screens.cpp error boxes print.
pub(crate) const COMP_SCREENS_CPP: usize = 0x99c15c;

/// The flag the competition screens raise on every build [0x99c00c] and the Results tab's date flag [0xadae0c] (0 at
/// the start of a build, 1 once the Results tab has its date heading) [inference].
pub(crate) const COMP_SCREEN_BUILT_FLAG: usize = 0x99c00c;

pub(crate) const COMP_SCREEN_DATE_BUTTONS_FLAG: usize = 0xadae0c;

/// The competition id the History tab is not offered for [0x9cf6f8] [inference].
pub(crate) const COMP_SCREEN_NO_HISTORY_ID: usize = 0x9cf6f8;

/// A stage byte: how many rounds a cup stage has (+0xab; a league stage counts as one table) [inference].
pub(crate) const COMP_SCREEN_STAGE_ROUNDS: usize = 0xab;

/// The competition's average points table (+0x18; the Average Points tab is offered when it is set) [inference].
pub(crate) const COMP_SCREEN_AVERAGE_POINTS: usize = 0x18;

/// A fixture: its round (+0x30), stage code (+0x32), stage index (+0x34) and stage byte (+0x42), as the Results tab
/// groups and links them [inference].
pub(crate) const COMP_SCREEN_FIXTURE_ROUND: usize = 0x30;

pub(crate) const COMP_SCREEN_FIXTURE_STAGE_CODE: usize = 0x32;

pub(crate) const COMP_SCREEN_FIXTURE_STAGE_INDEX: usize = 0x34;

pub(crate) const COMP_SCREEN_FIXTURE_STAGE_BYTE: usize = 0x42;

/// The column weights of the Results tab's fixture list (9 columns).
pub(crate) const COMP_SCREEN_RESULTS_WEIGHTS: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];

/// A MenuItem of the tab bar (0xd4 bytes): word +0 = −1 and word +2 = 0 (MenuItem::MenuItem 0x4568f0), the text at
/// +4, the kind at +0xcc and the value at +0xd0.
pub(crate) const COMP_SCREEN_MENU_ITEM: usize = 0xd4;

/// The order `comp_screens_open_comp_fixtures` writes its 54 parameters in.
pub(crate) const COMP_SCREEN_OPEN_ORDER: [u32; 54] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x23, 0x24, 0x25, 0x26, 0x27, 0x1e, 0x1f, 0x20,
    0x21, 0x22, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35];

/// The weights of the nine columns of the Fixtures tab's list (3, 2, 18, 1, 4, 1, 18, 2, 3).
pub(crate) const FIXTURES_TAB_WEIGHTS: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];

/// Two colour words of the competition tabs with no name yet (the average points columns and the unknown rounds).
pub(crate) const COMP_TABS_COLOUR_3152: usize = 0xae3152;

pub(crate) const COMP_TABS_COLOUR_3154: usize = 0xae3154;

/// The ids of the European competitions the Seedings tab links (Intertoto for kind 4, European Cup for kind 5)
/// [inference: the labels next to them].
pub(crate) const SEEDINGS_TAB_COMP_KIND_4: usize = 0x9cf6f8;

pub(crate) const SEEDINGS_TAB_COMP_KIND_5: usize = 0x9cf6e8;

/// Size of a competition record (`competition::REPUTATION` + 2).
pub(crate) const SEEDINGS_TAB_COMPETITION_SIZE: usize = 0x6b;

/// Set on a network game [inference: match_screens]; with it the competition screen takes a pending "print to file"
/// command (code `COMP_CLICK_PRINTOUT_DIALOG`) left by the Save Printout file box.
pub(crate) const COMP_CLICK_NETWORK_FLAG: usize = 0xb77c84;

/// The game option bit (in the dword at [G_CONFIG] + 4) that prints a printout straight away instead of asking for a
/// file name [inference].
pub(crate) const COMP_CLICK_PRINTOUT_STRAIGHT_AWAY: u32 = 0x1000000;

/// The game option bit (same dword) passed on to the printout writer as its last argument when the file name box
/// answers [inference].
pub(crate) const COMP_CLICK_PRINTOUT_SAVE_OPTION: u32 = 0x4000000;

/// The default file name buffer the Save Printout box edits (0xad9c64).
pub(crate) const COMP_CLICK_PRINTOUT_FILE_NAME: usize = 0xad9c64;

/// The time (ms) the live draw of the Stages tab last made a tie's fixtures [inference].
pub(crate) const COMP_CLICK_DRAW_TIMER: usize = 0xadae08;

/// The ids of the two competitions whose history columns change: the Olympics (Gold, Silver, Bronze) and the one whose
/// third column is "Minor Premiers" [inference: the Australian NSL] (dwords of the exe).
pub(crate) const COMP_HISTORY_OLYMPICS_ID: usize = 0x9cf7a0;

pub(crate) const COMP_HISTORY_MINOR_PREMIERS_ID: usize = 0x9cf89c;

/// The two flags the history builder writes outside the screen (1 on entry; [0xadae0c] 0 on entry and 1 once the
/// Records tab has its View popup) [inference: the screen's "built" marks, read by the menu bar].
pub(crate) const COMP_HISTORY_BUILT_FLAG: usize = 0x99c00c;

pub(crate) const COMP_HISTORY_POPUP_FLAG: usize = 0xadae0c;

/// The tabs of other items (tel23), called by address: Attendances, Results, Sequences, Teams, Players.
pub(crate) const COMP_HISTORY_TEL23_TABS: [usize; 5] = [0x499900, 0x499e10, 0x49a320, 0x49a830, 0x49ad40];

/// The history screen's season records list (param 9; entries of `HISTORY_SEASON_RECORD_SIZE` bytes: the season word
/// at +0, the record at +2) and its count (param 0xa), shared by the list tabs and built on the first one shown
/// (`Comp_Stats::build_season_records_list`) [inference].
pub(crate) const HISTORY_RECORDS_LIST: u32 = 9;

pub(crate) const HISTORY_RECORDS_COUNT: u32 = 0xa;

/// The last Comp_Stats record type (1..=0x16).
pub(crate) const HISTORY_LAST_RECORD_TYPE: u32 = 0x16;

/// The column weights of a list tab (2 columns: the season and the record).
pub(crate) const HISTORY_LIST_WEIGHTS: [u8; 2] = [1, 8];

/// The four params `comp_open_fifa_rankings_screen` writes.
pub(crate) const FIFA_RANKINGS_OPEN_PARAMS: [usize; 4] = [0, 0, 1, 1];

/// A UEFA coefficients table entry (0x12 bytes): +4 the nation, +8 the coefficient (f32), +0xc/+0xd/+0xe the places in
/// the European Cup, the UEFA Cup and the Intertoto Cup [inference].
pub(crate) const UEFA_COEFFICIENT_ENTRY: usize = 0x12;

/// A NewsItem on the stack (0xe0 bytes reserved; `NewsItem::NewsItem(0, 0)`, no destructor).
pub(crate) const NEWS_ITEM_BYTES: usize = 0xe0;

/// The competition screen's build, input and leave functions (passed to `ScreenManager::open_screen`).
pub(crate) const FN_COMP_SCREENS_DRAW_COMP_SCREEN: usize = 0x48fcd0;

pub(crate) const FN_COMP_SCREENS_HANDLE_COMP_SCREEN_CLICK: usize = 0x4965d0;

pub(crate) const FN_COMP_SCREENS_CLOSE_COMP_SCREEN: usize = 0x4985e0;

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\comp_stats.CPP" (the error box), the 35.0 of the qualification thresholds and
/// the qsort / bsearch comparator of the saved runs (by the first dword, `training_compare_entries_by_staff_id`).
pub(crate) const COMP_STATS_SOURCE_FILE: usize = 0x99ca94;

pub(crate) const COMP_STATS_F32_35: usize = 0x968a7c;

pub(crate) const COMP_STATS_BY_ID: usize = 0x8b4e00;

/// The run fields of a team entry copied to a saved run (0x1c bytes: the club id, then twelve words).
pub(crate) const COMP_STATS_RUN_WORDS: [usize; 12] = [0x19, 0x1d, 0x1f, 0x2d, 0x31, 0x33, 0x41, 0x45, 0x47, 0x55, 0x59, 0x5b];

/// The names of the team statistics 1..=17 and of the player statistics 1..=17.
pub(crate) const COMP_STATS_TEAM_LABELS: [usize; 17] = [0x99cbc0, 0x98b988, 0x98e9f8, 0x99cbb4, 0x99cba8, 0x99cb94, 0x99cb80, 0x99cb68, 0x99cb50,
    0x98e9cc, 0x99cb44, 0x99cb38, 0x99cb24, 0x99cb14, 0x99cb04, 0x99caf4, 0x99cae8];

pub(crate) const COMP_STATS_PLAYER_LABELS: [usize; 17] = [0x99cc40, 0x98e9cc, 0x99cb44, 0x99cb38, 0x99c06c, 0x99cc34, 0x99cc24, 0x990eb8, 0x99cc18,
    0x99caf4, 0x99cae8, 0x99c04c, 0x99cbf8, 0x99cbe8, 0x99cbd8, 0x99cbc8, 0x99cc08];

/// The default texts of a statistic ("----", "---", "0") and the formats "%ld", "%d (%d)", "%4.2f" and "%3.1f" ("%d"
/// and "%d%%" are `FORMAT_D` / `FORMAT_PCT`).
pub(crate) const COMP_STATS_DASHES_4: usize = 0x99cc4c;

pub(crate) const COMP_STATS_DASHES_3: usize = 0x98f858;

pub(crate) const COMP_STATS_ZERO_TEXT: usize = 0x99cc54;

pub(crate) const COMP_STATS_FORMAT_LD: usize = 0x98c11c;

pub(crate) const COMP_STATS_FORMAT_WITH_DETAIL: usize = 0x99cc58;

pub(crate) const COMP_STATS_FORMAT_2_DECIMALS: usize = 0x98fe94;

pub(crate) const COMP_STATS_FORMAT_1_DECIMAL: usize = 0x99ccc8;

/// The titles of the record types 1..=22 ("Most Times Winner" .. "Oldest Player").
pub(crate) const COMP_STATS_RECORD_TITLES: [usize; 22] = [0x99cd50, 0x99cd3c, 0x99cd2c, 0x99cd18, 0x99cd00, 0x98b9c8, 0x98b99c, 0x99cce4,
    0x98b97c, 0x98b940, 0x98b90c, 0x98b8f4, 0x98b8d8, 0x98b8bc, 0x98b8ac, 0x98b880, 0x98b870, 0x98b858, 0x98b844, 0x98b830,
    0x98b7e8, 0x98b7d8];

/// The competition's all-time records (+0x26) and current-season records (+0x28f).
pub(crate) const COMP_STATS_ALL_TIME_RECORDS: usize = 0x26;

pub(crate) const COMP_STATS_SEASON_RECORDS: usize = 0x28f;

/// The comp_stats.CPP path for the error box.
pub(crate) const COMP_STATS_CPP: usize = 0x99ca94;

/// `database_compare_comp_history_rows` (0x540780), the qsort order of the competition history table.
pub(crate) const COMPARE_COMP_HISTORY_ROWS: usize = 0x540780;

/// The two qsort orders of this item, by address (the hooks send them to the copy).
pub(crate) const COMPARE_TEAM_ENTRIES: usize = 0x4ad1a0;

pub(crate) const COMPARE_PLAYER_ENTRIES: usize = 0x4ad250;

/// One row of the season records list: the year (word) and the records.
pub(crate) const SEASON_RECORDS_ROW: usize = 2 + comp_stats_obj::SEASON_RECORDS_SIZE;

/// At most 500 rows.
pub(crate) const SEASON_RECORDS_ROWS: u32 = 500;

/// One row of the competition history table (0x1a bytes): the competition id at +4 and the champion club at +0xa.
pub(crate) const COMP_HISTORY_ROW_SIZE: usize = 0x1a;

/// `langlib_sprintf` (0x66f4e0): the language library's printf into a buffer; `{}` takes an int (the gender of the
/// word that follows, for the grammar) before the `%` arguments.
pub(crate) type RecordSprintf = unsafe extern "C" fn(*mut u8, usize, ...) -> i32;

/// `Date::get_short_date_string` (0x54a650, thiscall(date, buffer, size, flags), ret 0xc).
pub(crate) type RecordDateString = unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize;

/// `key_nation_format_comp_season` (0x6697c0, cdecl(year, competition id, buffer, size)): "1998/99" and the like.
pub(crate) type RecordSeasonText = unsafe extern "C" fn(u32, u32, *mut u8, u32) -> usize;

/// `format_venue_marker` (0x7e4ca0, cdecl(venue, buffer, size)): "H", "A" or "N".
pub(crate) type RecordVenueMarker = unsafe extern "C" fn(u32, *mut u8, u32) -> usize;

/// The link kinds of a record text: a competition, a match, a club and a person.
pub(crate) const LINK_COMPETITION: u32 = 0x7d0;

pub(crate) const LINK_MATCH: u32 = 0x7d3;

pub(crate) const LINK_CLUB: u32 = 0x7d5;

pub(crate) const LINK_PERSON: u32 = 0x7d7;

/// What leads a player-against-opponent record: the goals, or the age (from the birth date at the address).
#[derive(Clone, Copy)]
pub(crate) enum RecordLead {
    Goals(i32),
    Age(usize),
}

/// `write_date_span_record_to_news` (0x7e5420, cdecl(record, news, &slot)).
pub(crate) type NewsDateSpanWriter = unsafe extern "C" fn(usize, usize, *mut u8);

/// The data versions of the save that changed the statistics: from 0x3e015 a player entry is read whole (0x2d bytes;
/// before, 0x1d bytes and the six words +0x1d..+0x27 zeroed), and above 0x3e007 the block +0x56a is in the file.
pub(crate) const COMP_STATS_WHOLE_PLAYER_VERSION: i32 = 0x3e015;

/// The dialog code of the competition screen's Save Printout file name box.
pub(crate) const COMP_CLICK_PRINTOUT_DIALOG: u32 = 0x39;

/// A season record of that list (0x26b bytes) [inference].
pub(crate) const HISTORY_SEASON_RECORD_SIZE: usize = 0x26b;

/// `Plot::add_series` (0x7c7100, thiscall, ret 0xc): `n` points in a colour with a name.
pub(crate) unsafe fn plot_add_series(plot: usize, n: i32, colour: u32, name: usize) {
    exe::plot_add_series(plot, n, colour, name)
}

/// `Plot::add_point` (0x7c7240, thiscall, ret 0xc): point (x, y) of a series.
pub(crate) unsafe fn plot_add_point(plot: usize, series: u32, x: i32, y: i32) {
    exe::plot_add_point(plot, series, x, y)
}

/// Writes the body of the news that a competition broke one of its records (news type 0x3e9, from
/// `comp_stats_send_record_news`, written by `news_club_record` through the competition's statistics object).
///
/// `comp_stats_write_record_news_text(news, out)`. Without the Text, the comp_stats.CPP error box (line 0x1383) and 0;
/// the Text is cleared and set empty; without the news item, the box (0x138f) and 0. The local record copy is
/// constructed as the original does (`COMP_RECORD_NEWS_CONSTRUCTORS`); slot 0 (the competition id) is read and not
/// used, slot 1 is the record kind, then the kind's fields (`read_comp_record_news_fields`; an unknown kind: the box at
/// line 0x1ce2, then the box at line 0x16fe and 0).
///
/// The text, by kind, each only when its record holds (otherwise the empty text), with the clubs' short names and
/// genders and their links (0x7d5):
/// - 1 most titles ("titles" in a league division, `exe::comp_util_is_league_division` of the competition, otherwise
///   "trophies"), 2 points, 3 goals scored, 4 goals conceded, 5 yellow and red cards, 8 the average attendance;
/// - 6 and 7 the attendances, with both clubs and the round;
/// - 9 the record win ("%d-%d", the template by the venue) and 10 the highest scoring game (the higher count first,
///   the winner named first, win or draw); the score's link (0x7d3) when the fixture still matches;
/// - 11 to 14 the runs (more than one game): wins and defeats with the start and end dates, unbeaten and winless from
///   the start date;
/// - 15 goals, 17 assists, 18 the average rating with the games, 19 man of the match, 20 cards, with the player;
/// - 16 the most goals in one match, 21 and 22 the youngest and oldest player (the age at the match), with both clubs,
///   the round and the player.
/// The round label (`comp_util_format_stage_round_name`, mode 0) is linked to the match's club competition (0x7d0)
/// when not empty; the player (0x7d7) when his names still match. Returns 1.
///
/// Defects of the original: in kind 10 the genders stay the record club's first also when the opponent is named first
/// (the winner); the original's check that its local record copy is not null (line 0x1bdf) can never fail and is left
/// out.
/// Original: `comp_stats_write_record_news_text` (0x4a7f50, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_write_record_news_text(stats: usize, news: usize, out: usize) -> u32 {
    if out == 0 {
        show_error_box(COMP_STATS_CPP, 0x1383);
        return 0;
    }
    exe::text_clear(out as *mut usize);
    exe::text_set_string(out, EMPTY_STRING as *const u8);
    if news == 0 {
        show_error_box(COMP_STATS_CPP, 0x138f);
        return 0;
    }
    covered!("tel44_1 comp record news text");
    let mut record = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    let r = record.as_mut_ptr() as usize;
    for (o, f) in COMP_RECORD_NEWS_CONSTRUCTORS {
        let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
        construct(r + o);
    }
    exe::newsitem_get_slot(news, 0);
    let kind = exe::newsitem_get_slot(news, 1) as u8 as i8; // tel44_1: slot 1 holds the kind
    if !read_comp_record_news_fields(news, kind, r) {
        show_error_box(COMP_STATS_CPP, 0x1ce2);
        show_error_box(COMP_STATS_CPP, 0x16fe);
        return 0;
    }
    let mut text = [0u8; 1000];
    let tp = text.as_mut_ptr() as usize;
    match kind {
        1..=5 | 8 => {
            comp_record_team_text(stats, out, r, kind, tp);
        }
        6 => comp_record_attendance_text(out, r, 0x25, (0x99e9a4, 0x99e92c), tp),
        7 => comp_record_attendance_text(out, r, 0x48, (0x99e898, 0x99e824), tp),
        9 | 10 => comp_record_result_text(out, r, kind, tp),
        11..=14 => comp_record_run_text(out, r, kind, tp),
        16 | 21 | 22 => comp_record_player_match_text(out, r, kind, tp),
        _ => comp_record_player_text(out, r, kind, tp),
    }
    1
}

/// Comparer for `comp_stats_write_record_news_text`: the text side by side (`compare_text_writer`: the text, the links
/// and the random generator) and the return. Without the Text or the news item, or with an unknown kind (the error
/// boxes), only the original.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_write_record_news_text(stats: usize, news: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_COMP_STATS_WRITE_RECORD_NEWS_TEXT);
    if out == 0 || news == 0 {
        return o(stats, news, out);
    }
    let kind = exe::newsitem_get_slot(news, 1) as u8 as i8;
    if comp_record_news_fields(kind).is_empty() {
        return o(stats, news, out);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(out, || ro.set(o(stats, news, out)), |t| rr.set(comp_stats_write_record_news_text(stats, news, t)),
        || format!("comp_stats_write_record_news_text(kind {})", kind));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("comp_stats_write_record_news_text(kind {}) returned {} x {}", kind, ro.get(), rr.get()));
    ro.get()
}

/// The fields of each competition record kind (1..=22) as the news item carries them, in the original's order (the
/// inverse of `comp_stats_write_record_to_news`); kinds 21 and 22 start with the day and the year of the youngest or
/// oldest player's birth date.
pub(crate) fn comp_record_news_fields(kind: i8) -> &'static [RecordNewsField] {
    use RecordNewsField::*;
    match kind {
        1 => &[Byte(0), Dword(1)],
        2 => &[SWord(5), Dword(7), Word(0xb)],
        3 => &[SWord(0xd), Dword(0xf), Word(0x13)],
        4 => &[SWord(0x15), Dword(0x17), Word(0x1b)],
        5 => &[SByte(0x1d), SByte(0x1e), Dword(0x1f), Word(0x23)],
        6 => &[Dword(0x25), Match(0x29)],
        7 => &[Dword(0x48), Match(0x4c)],
        8 => &[Dword(0x6b), Dword(0x6f), Word(0x73)],
        9 => &[Match(0x75)],
        10 => &[Match(0x94)],
        11 => &[Run(0xb3), Dword(0xc7)],
        12 => &[Run(0xcb), Dword(0xdf)],
        13 => &[Run(0xe3), Dword(0xf7)],
        14 => &[Run(0xfb), Dword(0x10f)],
        15 => &[SByte(0x113), Holder(0x114), Dword(0x12c), Word(0x130)],
        16 => &[SByte(0x132), Holder(0x133), Match(0x14b)],
        17 => &[SByte(0x16a), Holder(0x16b), Dword(0x183), Word(0x187)],
        18 => &[Percent(0x189), Holder(0x18d), SByte(0x1a5), Dword(0x1a6), Word(0x1aa)],
        19 => &[SByte(0x1ac), Holder(0x1ad), Dword(0x1c5), Word(0x1c9)],
        20 => &[SByte(0x1cb), SByte(0x1cc), Holder(0x1cd), Dword(0x1e5), Word(0x1e9)],
        21 => &[SWord(0x1eb), Word(0x1ed), Holder(0x1f3), Match(0x20b)],
        22 => &[SWord(0x22a), Word(0x22c), Holder(0x232), Match(0x24a)],
        _ => &[],
    }
}

/// Reads the broken record of `kind` from the news item's slots (from slot 2 on) into the local record copy `r`:
/// bytes, words and dwords stored at their offsets, the average rating (kind 18) as the slot × 0.01 in a float, the
/// match, holder and run records by their readers (`exe::read_match_record_from_news`,
/// `exe::read_staff_name_ref_from_news`, `read_run_record_from_news`, each moving the slot on); kinds 21 and 22 read the
/// day and the year into a Date (`exe::new_date`, then `exe::date_operator_3d` into the record). Returns false for a
/// kind outside 1..=22.
pub(crate) unsafe fn read_comp_record_news_fields(news: usize, kind: i8, r: usize) -> bool {
    use RecordNewsField::*;
    let fields = comp_record_news_fields(kind);
    if fields.is_empty() {
        return false;
    }
    let mut k: u8 = 2;
    let slot = |k: &mut u8| {
        let v = exe::newsitem_get_slot(news, *k as u32);
        *k = k.wrapping_add(1);
        v
    };
    let mut rest = fields;
    if kind == 21 || kind == 22 {
        let (day, year) = (slot(&mut k), slot(&mut k));
        let (mut date, mut copy) = ([0u8; 8], [0u8; 8]);
        exe::new_date(date.as_mut_ptr() as usize, day, year);
        // the birth date of the youngest or oldest player's record
        exe::date_operator_3d(r + if kind == 21 { 0x1eb } else { 0x22a }, copy.as_mut_ptr() as usize, date.as_ptr() as usize);
        rest = &fields[2..];
    }
    let run: unsafe extern "C" fn(usize, *mut u8, usize) -> u32 = core::mem::transmute(FN_READ_RUN_RECORD_FROM_NEWS);
    for f in rest {
        match *f {
            Byte(o) | SByte(o) => write_u8(r + o, slot(&mut k) as u8),
            Word(o) | SWord(o) => write_u16(r + o, slot(&mut k) as u16),
            Dword(o) => write_u32(r + o, slot(&mut k)),
            Percent(o) => write_f32(r + o, (slot(&mut k) as i32 as f64 * read_f64(rdata::F64_0_01)) as f32),
            Match(o) => {
                exe::read_match_record_from_news(news, &mut k, (r + o) as *mut u8);
            }
            Holder(o) => {
                exe::read_staff_name_ref_from_news(news, &mut k, (r + o) as *mut u8);
            }
            Run(o) => {
                run(news, &mut k, r + o);
            }
        }
    }
    true
}

/// Kinds 1 to 5 and 8, a team record: most titles (or trophies, outside a league division), points, goals scored and
/// conceded, cards and the average attendance; empty (no text) when the count is below 0.
pub(crate) unsafe fn comp_record_team_text(stats: usize, out: usize, r: usize, kind: i8, tp: usize) -> bool {
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let team = |o: usize| CompRecordTeam::at(r + o);
    let t = match kind {
        1 => {
            let wins = read_i8(r);
            if wins <= -1 {
                return false;
            }
            let t = team(1);
            let comp_id = read_i32(read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION));
            let template = if exe::comp_util_is_league_division(comp_id) != 0 { 0x99eba0 } else { 0x99eb68 };
            f(tp as *mut u8, template, t.gender, t.gender, t.name(), wins as i32);
            t
        }
        2..=4 => {
            let at = [5usize, 0xd, 0x15][kind as usize - 2];
            let value = read_i16(r + at);
            if value <= -1 {
                return false;
            }
            let t = team(at + 2);
            f(tp as *mut u8, [0x99eb24usize, 0x99eae0, 0x99ea9c][kind as usize - 2], t.gender, t.gender, value as i32, t.name());
            t
        }
        5 => {
            let yellow = read_i8(r + 0x1d);
            if yellow <= -1 {
                return false;
            }
            let t = team(0x1f);
            f(tp as *mut u8, 0x99ea38, t.gender, t.gender, t.name(), yellow as i32, read_i8(r + 0x1e) as i32);
            t
        }
        _ => {
            let average = read_i32(r + 0x6b);
            if average <= -1 {
                return false;
            }
            let t = team(0x6f);
            f(tp as *mut u8, 0x99e7cc, t.gender, t.gender, t.name(), average);
            t
        }
    };
    exe::text_set_string(out, tp as *const u8);
    t.link(out);
    true
}

/// Kinds 6 and 7, the attendance at a match (`at`: the attendance, the match record after it), with the round label
/// (form 1) when there is one.
pub(crate) unsafe fn comp_record_attendance_text(out: usize, r: usize, at: usize, templates: (usize, usize), tp: usize) {
    let attendance = read_i32(r + at);
    if attendance <= -1 {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let x = CompRecordMatch::at(r + at + 4, 1);
    let (hg, ag) = (x.home.gender, x.away.gender);
    if x.has_label() {
        f(tp as *mut u8, templates.0, hg, hg, ag, ag, x.label_gender, attendance, x.home.name(), x.away.name(), x.label.as_ptr());
    } else {
        f(tp as *mut u8, templates.1, hg, hg, ag, ag, attendance, x.home.name(), x.away.name());
    }
    exe::text_set_string(out, tp as *const u8);
    x.link(out);
}

/// Kinds 9 and 10, a result: the record win (the score as it is, the template by the venue) and the highest scoring
/// game (the higher count first, the winner named first, the template by the result). Round label form 0. The genders
/// always follow the record's club and its opponent, also when the opponent is named first (as the original does).
pub(crate) unsafe fn comp_record_result_text(out: usize, r: usize, kind: i8, tp: usize) {
    let m = RecordMatch(r + if kind == 9 { 0x75 } else { 0x94 });
    if m.comp_index() <= -1 {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let crt: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (g1, g2) = m.goals();
    let mut score = [0u8; 0x14];
    if kind == 9 {
        crt(score.as_mut_ptr(), 0x98c114, g1 as i32, g2 as i32); // "%d-%d"
    } else {
        crt(score.as_mut_ptr(), 0x98c114, g1.max(g2) as i32, g1.min(g2) as i32);
    }
    let x = CompRecordMatch::at(m.0, 0);
    let (hg, ag, lg) = (x.home.gender, x.away.gender, x.label_gender);
    let (home, away, sp) = (x.home.name(), x.away.name(), score.as_ptr());
    let (first, second) = if kind == 10 && g1 < g2 { (away, home) } else { (home, away) };
    if x.has_label() {
        let template = match kind {
            9 => comp_record_by_venue(m.0, [0x99e748, 0x99e6c8, 0x99e648]),
            _ => if g1 == g2 { 0x99e414 } else { 0x99e4a0 },
        };
        f(tp as *mut u8, template, hg, hg, ag, ag, lg, first, sp, second, x.label.as_ptr());
    } else {
        let template = match kind {
            9 => comp_record_by_venue(m.0, [0x99e5e8, 0x99e58c, 0x99e530]),
            _ => if g1 == g2 { 0x99e344 } else { 0x99e3ac },
        };
        f(tp as *mut u8, template, hg, hg, ag, ag, first, sp, second);
    }
    exe::text_set_string(out, tp as *const u8);
    x.link(out);
    m.link_score(out, &score);
}

/// Kinds 11 to 14, a run (more than one game): the runs of wins and defeats between the start and the end dates, the
/// unbeaten and winless runs from the start date (`exe::date_get_long_date_string_no_year`); the club's link.
pub(crate) unsafe fn comp_record_run_text(out: usize, r: usize, kind: i8, tp: usize) {
    let run = r + [0xb3usize, 0xcb, 0xe3, 0xfb][kind as usize - 11];
    let games = read_i32(run);
    if games <= -1 || games <= 1 {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let t = CompRecordTeam::at(run + 0x14);
    let (mut start, mut end) = ([0u8; 0xc8], [0u8; 0xc8]);
    exe::date_get_long_date_string_no_year(run + run_record::START_DATE, start.as_mut_ptr() as usize, 0xc8, 0xff);
    if kind <= 12 {
        exe::date_get_long_date_string_no_year(run + run_record::END_DATE, end.as_mut_ptr() as usize, 0xc8, 0xff);
        let template = if kind == 11 { 0x99e2c4 } else { 0x99e240 };
        f(tp as *mut u8, template, t.gender, t.gender, 0x64, 0x64, t.name(), games, start.as_ptr(), end.as_ptr());
    } else {
        let template = if kind == 13 { 0x99e1d8 } else { 0x99e168 };
        f(tp as *mut u8, template, t.gender, t.gender, 0x64, t.name(), games, start.as_ptr());
    }
    exe::text_set_string(out, tp as *const u8);
    t.link(out);
}

/// Kinds 15 and 17 to 20, a player's season record: goals, assists, the average rating (above −1.0, "%4.2f", with the
/// games), man of the match awards, yellow and red cards; the holder's name (style 1), the club's link and the
/// player's when his names still match.
pub(crate) unsafe fn comp_record_player_text(out: usize, r: usize, kind: i8, tp: usize) {
    let (holder, club_at) = match kind {
        15 => (0x114usize, 0x12cusize),
        17 => (0x16b, 0x183),
        18 => (0x18d, 0x1a6),
        19 => (0x1ad, 0x1c5),
        _ => (0x1cd, 0x1e5),
    };
    // 15, 17, 19: the count byte just before the holder; 20: the yellow cards
    let count = read_i8(r + holder - 1);
    let rating = read_f32(r + 0x189);
    let holds = match kind {
        18 => rating as f64 > read_f64(rdata::F64_NEG_1),
        20 => read_i8(r + 0x1cb) > -1,
        _ => count > -1,
    };
    if !holds {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut player = [0u8; 0xc8];
    record_holder_name(r + holder, &mut player, 1);
    let t = CompRecordTeam::at(r + club_at);
    let (g, p) = (t.gender, player.as_ptr());
    match kind {
        15 => f(tp as *mut u8, 0x99e110, g, g, 0x64, count as i32, t.name(), p),
        17 => f(tp as *mut u8, 0x99dd40, g, g, 0x64, count as i32, t.name(), p),
        18 => f(tp as *mut u8, 0x99dcb0, g, g, 0x64, rating as f64, t.name(), p, read_i8(r + 0x1a5) as i32),
        19 => f(tp as *mut u8, 0x99dc3c, g, g, count as i32, t.name(), p),
        _ => f(tp as *mut u8, 0x99dbb4, g, g, read_i8(r + 0x1cb) as i32, read_i8(r + 0x1cc) as i32, t.name(), p),
    };
    exe::text_set_string(out, tp as *const u8);
    t.link(out);
    record_holder_link(out, r + holder, &player);
}

/// Kinds 16, 21 and 22, a player's record in one match: the most goals (the count first) and the youngest and oldest
/// player (the age in years at the match, `database_format_years_between_dates` from the birth date to the match date,
/// 5 bytes); the template by the venue, with the round label (form 0) when there is one; the links of both clubs, the
/// round and the player.
pub(crate) unsafe fn comp_record_player_match_text(out: usize, r: usize, kind: i8, tp: usize) {
    let (base, holder, at) = match kind {
        16 => (0x132usize, 0x133usize, 0x14busize),
        21 => (0x1eb, 0x1f3, 0x20b),
        _ => (0x22a, 0x232, 0x24a),
    };
    let holds = if kind == 16 { read_i8(r + base) > -1 } else { RecordMatch(r + at).comp_index() > -1 };
    if !holds {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut player = [0u8; 0xc8];
    record_holder_name(r + holder, &mut player, 1);
    let mut years = [0u8; 8];
    if kind != 16 {
        let between: unsafe extern "C" fn(usize, usize, usize, i32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES);
        between(r + at, r + base, years.as_mut_ptr() as usize, 5);
    }
    let x = CompRecordMatch::at(r + at, 0);
    let (hg, ag, lg) = (x.home.gender, x.away.gender, x.label_gender);
    let (home, away, p, y) = (x.home.name(), x.away.name(), player.as_ptr(), years.as_ptr());
    let goals = read_i8(r + base) as i32;
    match (kind == 16, x.has_label()) {
        (true, true) => f(tp as *mut u8, comp_record_by_venue(r + at, [0x99e068, 0x99dfc4, 0x99df20]), 0x64, hg, hg, ag, ag, lg, goals, p, home, away, x.label.as_ptr()),
        (true, false) => f(tp as *mut u8, comp_record_by_venue(r + at, [0x99de9c, 0x99de1c, 0x99dd9c]), 0x64, hg, hg, ag, ag, goals, p, home, away),
        (false, true) => f(tp as *mut u8, comp_record_by_venue(r + at, [0x99db08, 0x99da60, 0x99d9b8]), 0x64, hg, hg, ag, ag, lg, p, y, home, away, x.label.as_ptr()),
        (false, false) => f(tp as *mut u8, comp_record_by_venue(r + at, [0x99d930, 0x99d8ac, 0x99d828]), 0x64, hg, hg, ag, ag, p, y, home, away),
    };
    exe::text_set_string(out, tp as *const u8);
    x.link(out);
    record_holder_link(out, r + holder, &player);
}

/// The template by the venue of a record's match (+0x10: 1 at home, 2 away, otherwise neutral).
pub(crate) unsafe fn comp_record_by_venue(m: usize, templates: [usize; 3]) -> usize {
    match read_u8(m + club_match_record::VENUE) {
        1 => templates[0],
        2 => templates[1],
        _ => templates[2],
    }
}

/// The constructors the original runs over its local copy of the broken record (the `Comp_Season_Records` layout,
/// `comp_stats_obj::SEASON_RECORDS_SIZE` bytes), in its order: the attendance and result match records (0x645800,
/// FixedFixture::FixedFixture: only their Date), the four run records (`run_record_construct`), the two holders, the
/// goals-in-a-match record, the tallies, the rating record, the count record and the youngest and oldest player
/// records. All thiscall, returning `this`; they only write default Dates.
pub(crate) const COMP_RECORD_NEWS_CONSTRUCTORS: [(usize, usize); 17] = [
    (0x29, 0x645800), (0x4c, 0x645800), (0x75, 0x645800), (0x94, 0x645800),
    (0xb3, 0x448670), (0xcb, 0x448670), (0xe3, 0x448670), (0xfb, 0x448670),
    (0x114, 0x450d40), (0x133, 0x450d40), (0x14b, 0x645800), (0x16a, 0x450c60), (0x189, 0x453c50), (0x1ac, 0x450c60),
    (0x1cb, 0x450d30), (0x1eb, 0x49f390), (0x22a, 0x49f3f0),
];

/// The two clubs of a record's match `m` (the club +0x11 and its opponent +0x15) and its round label (`form`).
pub(crate) struct CompRecordMatch {
    pub(crate) m: usize,
    pub(crate) home: CompRecordTeam,
    pub(crate) away: CompRecordTeam,
    pub(crate) label: [u8; 0xc8],
    pub(crate) label_gender: i32,
}

impl CompRecordMatch {
    pub(crate) unsafe fn at(m: usize, form: u32) -> Self {
        let home = CompRecordTeam::at(m + club_match_record::CLUB_ID);
        let away = CompRecordTeam::at(m + club_match_record::OPPONENT_ID);
        let mut label = [0u8; 0xc8];
        let label_gender = comp_record_round_label(m, form, &mut label);
        CompRecordMatch { m, home, away, label, label_gender }
    }
    pub(crate) fn has_label(&self) -> bool {
        self.label[0] != 0
    }
    /// The links of both clubs, then the round's (0x7d0, to the match's club competition) when there is a label.
    pub(crate) unsafe fn link(&self, out: usize) {
        self.home.link(out);
        self.away.link(out);
        if self.has_label() {
            record_stage_link(out, self.m, &self.label);
        }
    }
}

/// A club of a competition record: its short name copied (`club::SHORT_NAME`), the gender of that name
/// (`club::SHORT_GENDER_NAME`, sign-extended for the `{}` of the formats) and the club, from the club id at `id_at`.
pub(crate) struct CompRecordTeam {
    pub(crate) club: usize,
    pub(crate) name: [u8; 0xc8],
    pub(crate) gender: i32,
}

impl CompRecordTeam {
    pub(crate) unsafe fn at(id_at: usize) -> Self {
        let club = record_club(id_at, 0);
        let mut name = [0u8; 0xc8];
        record_club_name(club, &mut name);
        CompRecordTeam { club, name, gender: record_gender(club) } // tel44_1: the team's gender
    }
    pub(crate) fn name(&self) -> *const u8 {
        self.name.as_ptr()
    }
    /// The club's link (0x7d5) under its short name.
    pub(crate) unsafe fn link(&self, out: usize) {
        exe::text_add_link(out, self.name.as_ptr(), LINK_CLUB, self.club); // tel44_1: the club's link
    }
}

/// Rule: every played match goes into its competition's statistics: both clubs' entries (games, home crowds, form,
/// runs, goals, cards and the team totals), the entry of every player who took part, and the season's records,
/// which pass to the all-time records when they beat them (the news flag only from the third season end, for most
/// kinds only in a league division). A run's end can make a media story, and a club's cards can bring an FA fine.
///
/// `Comp_Stats::record_match_result(fixture, game, home_points, away_points)`: a local match record is constructed
/// (`Date::Date`); no fixture: the comp_stats.CPP 237 box and 0; no match record: the comp_stats.CPP 245 box and 0.
/// For each side (home, then away): `fill_match_record_from_fixture` of the side's club into the local record; the
/// club's entry (`comp_stats_find_club_entry`, or `comp_stats_add_club_entry`) counts one more game; the result code
/// (`match_record_result_code`) goes into the form lists (`record_side_venue`, then the all-matches list); the runs and
/// the result records (`record_side_result`); the points (`record_side_points`, the short of its side); the goal
/// totals (`record_side_goals`); the five team totals (`comp_club_figures::TEAM_TOTALS`, from the side block's bytes
/// +3..+7); the cards and the fine (`record_side_cards`); the highest scoring match (`record_highest_scoring`, home
/// side only); then each of the 20 lineup rows that counts (`match_lineup_row`) through `record_player_row`. At the end
/// the season's cached average attendance (`SEASON_MOST_POINTS`, -1) and average rating (`SEASON_RATING`, -1.0) are
/// cleared and, when the match has passes (`match_record::HOME_PASSES` ≠ 0), `accumulate_match_player_totals`. Returns
/// 1. The two points arguments are dwords whose high half the caller does not set; only their shorts are read.
/// Original: `Comp_Stats::record_match_result` (0x49f570, comp_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn comp_stats_record_match_result(stats: usize, fixture: usize, game: usize, home_points: u32, away_points: u32) -> u32 {
    let mut record = [0u8; 0x20];
    let record_at = record.as_mut_ptr() as usize;
    exe::date_date_54a110(record_at);
    if fixture == 0 {
        show_error_box(COMP_STATS_CPP, 0xed);
        return 0;
    }
    if game == 0 {
        show_error_box(COMP_STATS_CPP, 0xf5);
        return 0;
    }
    let fixture = Fixture(fixture);
    for side in 0..2usize {
        let club = if side == 0 { fixture.home_club() } else { fixture.away_club() };
        fill_match_record_from_fixture(fixture.addr(), club.addr(), record_at);
        let mut entry = comp_stats_find_club_entry(stats, club.addr());
        if entry == 0 {
            covered!("tel43_1 club entry added");
            entry = comp_stats_add_club_entry(stats, club.addr());
        }
        let s = RecordedSide { stats, fixture, game, side, club, entry: CompStatsClub(entry), record: record_at };
        s.entry.set_matches_played(s.entry.matches_played().wrapping_add(1)); // tel43_1: one more game
        let result = match_record_result_code(record_at);
        record_side_venue(&s, result);
        push_form(entry + comp_club_figures::FORM, result);
        record_side_result(&s);
        let points = if side == 0 { home_points } else { away_points };
        record_side_points(&s, points as u16 as i16);
        record_side_goals(&s);
        for k in 0..5 {
            let at = entry + comp_club_figures::TEAM_TOTALS + k;
            write_u8(at, read_u8(at).wrapping_add(read_u8(s.team() + 3 + k))); // the side block's bytes +3..+7
        }
        record_side_cards(&s);
        record_highest_scoring(&s);
        for k in 0..20 {
            if let Some((row, person, sheet)) = match_lineup_row(game, side, k) {
                record_player_row(&s, k, row, Staff(person), sheet);
            }
        }
    }
    write_i32(stats + comp_stats_obj::SEASON_MOST_POINTS, -1);
    write_f32(stats + comp_stats_obj::SEASON_RATING, -1.0);
    if read_i16(game + match_record::HOME_PASSES) != 0 {
        comp_stats_accumulate_match_player_totals(stats, game);
    }
    1
}

/// Comparer for `comp_stats_record_match_result`: side by side over the statistics object, both tables (their whole
/// capacity: new player entries go after the count), the sort context and the random generator's state (the story's
/// draws), when the match only touches memory (`record_match_memory_only`); otherwise (a new club entry, a table that
/// must grow, a possible story or fine, a null argument) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_record_match_result(stats: usize, fixture: usize, game: usize, home_points: u32, away_points: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_COMP_STATS_RECORD_MATCH_RESULT);
    if fixture == 0 || game == 0 || !record_match_memory_only(stats, Fixture(fixture), game) {
        return o(stats, fixture, game, home_points, away_points);
    }
    let mut regions = vec![(stats, COMP_STATS_OBJECT_SIZE), (G_COMP_STATS_SORT_CONTEXT, 4)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    for (table, capacity, size) in [(comp_stats_obj::CLUBS, comp_stats_obj::CLUB_CAPACITY, comp_stats_obj::CLUB_ENTRY_SIZE),
        (comp_stats_obj::PLAYERS, comp_stats_obj::PLAYER_CAPACITY, comp_stats_obj::PLAYER_ENTRY_SIZE)] {
        let (at, n) = (read_usize(stats + table), read_u32(stats + capacity) as usize * size);
        if at != 0 && n != 0 {
            regions.push((at, n));
        }
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, fixture, game, home_points, away_points),
        || crate::rng::without_counting(|| comp_stats_record_match_result(stats, fixture, game, home_points, away_points)));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::record_match_result({:#x}, fixture {:#x}): {} x {}, differences in {:?}",
        stats, fixture, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Puts a result code at the head of a five-byte form list, the older ones moving down one place.
pub(crate) unsafe fn push_form(list: usize, result: u8) {
    for k in (1..comp_club_figures::FORM_LENGTH).rev() {
        write_u8(list + k, read_u8(list + k - 1));
    }
    write_u8(list, result);
}

/// The venue part of a side: at home (venue 1) one more home game, the attendance added to the home total, the
/// result into the home form and the attendance records (`record_attendance_record`, highest then lowest); at a neutral
/// ground (venue 3) the attendance records only for the home side (once per match); away (venue 2) the result into
/// the away form.
pub(crate) unsafe fn record_side_venue(s: &RecordedSide, result: u8) {
    match read_u8(s.record + club_match_record::VENUE) {
        1 => {
            s.entry.set_home_matches_played(s.entry.home_matches_played().wrapping_add(1));
            s.entry.set_home_attendance_total(s.entry.home_attendance_total().wrapping_add(read_i32(s.game + match_record::ATTENDANCE)));
            push_form(s.entry.addr() + comp_club_figures::HOME_FORM, result);
            record_attendance_record(s, comp_records::HIGHEST_ATTENDANCE, comp_record_kind::HIGHEST_ATTENDANCE, true);
            record_attendance_record(s, comp_records::LOWEST_ATTENDANCE, comp_record_kind::LOWEST_ATTENDANCE, false);
        }
        3 => {
            if s.side == 0 {
                covered!("tel43_1 neutral ground attendance");
                record_attendance_record(s, comp_records::HIGHEST_ATTENDANCE, comp_record_kind::HIGHEST_ATTENDANCE, true);
                record_attendance_record(s, comp_records::LOWEST_ATTENDANCE, comp_record_kind::LOWEST_ATTENDANCE, false);
            }
        }
        2 => push_form(s.entry.addr() + comp_club_figures::AWAY_FORM, result),
        _ => {}
    }
}

/// The runs of a side by the result (goals, signed): a win records the biggest win, extends the winning run, may
/// make the "winless run ended" story and clears the losing and winless runs; a defeat extends the losing run, may
/// make the "unbeaten run ended" story and clears the winning and unbeaten runs. Then a win or draw extends the
/// unbeaten run and clears the losing one, and a draw or defeat extends the winless run and clears the winning one.
pub(crate) unsafe fn record_side_result(s: &RecordedSide) {
    let entry = s.entry.addr();
    if s.goals_for() > s.goals_against() {
        record_biggest_win(s);
        extend_run(s, comp_stats_club::WINNING_RUN, comp_records::WINNING_RUN, comp_record_kind::WINNING_RUN, 12);
        run_ended_story(s, STORY_WINLESS_RUN_ENDED, comp_stats_club::WINLESS_RUN, true);
        write_i32(entry + comp_stats_club::LOSING_RUN, 0);
        write_i32(entry + comp_stats_club::WINLESS_RUN, 0);
    } else if s.goals_for() < s.goals_against() {
        extend_run(s, comp_stats_club::LOSING_RUN, comp_records::LOSING_RUN, comp_record_kind::LOSING_RUN, 12);
        run_ended_story(s, STORY_UNBEATEN_RUN_ENDED, comp_stats_club::UNBEATEN_RUN, false);
        write_i32(entry + comp_stats_club::WINNING_RUN, 0);
        write_i32(entry + comp_stats_club::UNBEATEN_RUN, 0);
    }
    if s.goals_for() >= s.goals_against() {
        extend_run(s, comp_stats_club::UNBEATEN_RUN, comp_records::UNBEATEN_RUN, comp_record_kind::UNBEATEN_RUN, 24);
        write_i32(entry + comp_stats_club::LOSING_RUN, 0); // tel43_1: a win or a draw ends the losing run
    }
    if s.goals_for() <= s.goals_against() {
        extend_run(s, comp_stats_club::WINLESS_RUN, comp_records::WINLESS_RUN, comp_record_kind::WINLESS_RUN, 24);
        write_i32(entry + comp_stats_club::WINNING_RUN, 0);
    }
}

/// The points record (kind 2): outside friendlies, in a league division and with positive points
/// (`record_team_value_record`).
pub(crate) unsafe fn record_side_points(s: &RecordedSide, points: i16) {
    if s.friendly() || !s.league_division() || points <= 0 {
        return;
    }
    record_team_value_record(s, comp_records::POINTS, comp_record_kind::POINTS, points);
}

/// The goal totals: the goals scored, when any, go into the entry's total and its record (kind 3); the goals
/// conceded the same (kind 4); outside friendlies (`record_team_value_record`).
pub(crate) unsafe fn record_side_goals(s: &RecordedSide) {
    for (goals, total, at, kind) in [(s.goals_for(), comp_club_figures::GOALS_FOR, comp_records::GOALS_FOR, comp_record_kind::GOALS_FOR),
        (s.goals_against(), comp_club_figures::GOALS_AGAINST, comp_records::GOALS_AGAINST, comp_record_kind::GOALS_AGAINST)] {
        if goals == 0 {
            continue;
        }
        let total = s.entry.addr() + total;
        write_u8(total, read_u8(total).wrapping_add(goals as u8));
        if !s.friendly() {
            record_team_value_record(s, at, kind, read_u8(total) as i16);
        }
    }
}

/// The cards of a side: with any card in the match, the entry's yellow and red totals go up; with the finance table
/// and the manager manager, the FA fine (`charge_discipline_fine`, by the points yellow + 4 × red); then, outside
/// friendlies, the cards record (kind 5, ordered by red × 3 + yellow): above the season's (or with none, 0xff), the
/// totals, a cleared year and the club; above the all-time one too, that takes them and the competition's year,
/// flagged from the third season end with 2 or more of each and another holder or year, in a league division.
pub(crate) unsafe fn record_side_cards(s: &RecordedSide) {
    let entry = s.entry.addr();
    let (yellow, red) = (read_u8(s.team() + match_record_team::YELLOW_CARDS), read_u8(s.team() + match_record_team::RED_CARDS));
    if yellow == 0 && red == 0 {
        return;
    }
    write_u8(entry + comp_club_figures::YELLOW_CARDS, read_u8(entry + comp_club_figures::YELLOW_CARDS).wrapping_add(yellow));
    write_u8(entry + comp_club_figures::RED_CARDS, read_u8(entry + comp_club_figures::RED_CARDS).wrapping_add(red));
    let (yellow, red) = (read_u8(entry + comp_club_figures::YELLOW_CARDS), read_u8(entry + comp_club_figures::RED_CARDS));
    if read_usize(G_FINANCE_TABLE) != 0 && read_usize(G_MANAGER_MANAGER) != 0 {
        charge_discipline_fine(s.club, (yellow as u16).wrapping_add((red as u16).wrapping_mul(4)) as i16);
    }
    let weight = |y: usize| read_i8(y + 1) as i32 * 3 + read_i8(y) as i32;
    let value = red as i32 * 3 + yellow as i32;
    if s.friendly() {
        return;
    }
    let season = s.season(comp_records::CARDS);
    if read_u8(season) != 0xff && value <= weight(season) {
        return;
    }
    write_u8(season, yellow);
    write_u8(season + 1, red);
    write_u16(season + comp_records::VALUE_YEAR, 0);
    write_u32(season + comp_records::VALUE_CLUB, s.club_id());
    let all = s.all_time(comp_records::CARDS);
    if read_u8(all) != 0xff && value <= weight(all) {
        return;
    }
    if s.seasons_ended() >= 3 && yellow >= 2 && red >= 2
        && (s.other_year(all + comp_records::VALUE_YEAR) || s.club_id() != read_u32(all + comp_records::VALUE_CLUB)) && s.league_division() {
        covered!("tel43_1 cards record news");
        s.flag(comp_record_kind::CARDS);
    }
    write_u32(all, read_u32(season));
    write_u32(all + 4, read_u32(season + 4));
    write_i16(all + comp_records::VALUE_YEAR, s.year());
}

/// The highest scoring match (kind 10): only for the home side, outside friendlies and with goals; the club's
/// match record replaces the season's when there is none or it has more goals (`match_record_has_more_goals`); the
/// all-time one the same way, flagged for 10 goals or more.
pub(crate) unsafe fn record_highest_scoring(s: &RecordedSide) {
    if s.friendly() || s.side != 0 {
        return;
    }
    let goals = s.goals_for() as i32 + s.goals_against() as i32;
    if goals <= 0 {
        return;
    }
    let season = s.season(comp_records::HIGHEST_SCORING);
    if read_i32(season + club_match_record::FIXTURE_ID) != -1 && match_record_has_more_goals(s.record, season) == 0 {
        return;
    }
    match_record_assign(season, s.record);
    let all = s.all_time(comp_records::HIGHEST_SCORING);
    if read_i32(all + club_match_record::FIXTURE_ID) != -1 && match_record_has_more_goals(s.record, all) == 0 {
        return;
    }
    if goals >= 10 {
        covered!("tel43_1 highest scoring news");
        s.flag(comp_record_kind::HIGHEST_SCORING);
    }
    match_record_assign(all, season);
}

/// One lineup row (row `k`, 11 and up the substitutes) of a side: the player's entry (`comp_stats_find_player_entry`,
/// or `comp_stats_add_player_entry` with the person's id) takes the club and the fixture's round byte, one more game
/// (and one as a substitute); the goals (with the top scorer and goals-in-a-match records), the goals conceded and
/// the penalties, the assists (record), the team's goals and the win or defeat, the cards (record), the man of the
/// match (record) and the rating; with the match's statistics (`match_record::NO_STATISTICS` = 0) the passes,
/// tackles, dribbles and shots and the minutes; then the youngest and the oldest player records.
pub(crate) unsafe fn record_player_row(s: &RecordedSide, k: usize, row: usize, person: Staff, sheet: usize) {
    use comp_player_figures as f;
    let mut entry = comp_stats_find_player_entry(s.stats, sheet);
    if entry == 0 {
        entry = comp_stats_add_player_entry(s.stats, sheet, read_u32(person.addr()));
    }
    let add = |at: usize, v: u8| write_u8(entry + at, read_u8(entry + at).wrapping_add(v));
    write_u32(entry + comp_stats_player::CLUB, s.club_id());
    write_u8(entry + comp_stats_player::NEW_MARK, s.fixture.round_number() as u8);
    add(f::GAMES, 1); // tel43_1: one more game for the player
    if k > 10 {
        add(f::GAMES_AS_SUBSTITUTE, 1);
    }
    let goals = read_u8(row + lineup_row::GOALS);
    if goals != 0 {
        add(f::GOALS, goals);
        record_player_tally(s, comp_records::TOP_SCORER, comp_record_kind::TOP_SCORER, read_u8(entry + f::GOALS), person);
        record_goals_in_a_match(s, goals, person);
    }
    add(f::GOALS_CONCEDED, read_u8(row + lineup_row::GOALS_CONCEDED));
    add(f::PENALTIES_TAKEN, read_u8(row + lineup_row::PENALTIES_TAKEN));
    add(f::PENALTIES_SCORED, read_u8(row + lineup_row::PENALTIES_SCORED));
    let assists = read_u8(row + lineup_row::ASSISTS);
    if assists != 0 {
        add(f::ASSISTS, assists);
        record_player_tally(s, comp_records::ASSISTS, comp_record_kind::ASSISTS, read_u8(entry + f::ASSISTS), person);
    }
    let (scored, conceded) = (s.goals_for(), s.goals_against());
    if scored != 0 {
        add(f::TEAM_GOALS_FOR, scored as u8);
    }
    if conceded != 0 {
        add(f::TEAM_GOALS_AGAINST, conceded as u8);
    }
    if scored > conceded {
        add(f::WINS, 1);
    } else if scored < conceded {
        add(f::DEFEATS, 1);
    }
    let (yellow, red) = (read_u8(row + lineup_row::YELLOW_CARDS), read_u8(row + lineup_row::RED_CARD));
    if yellow != 0 || red != 0 {
        add(f::YELLOW_CARDS, yellow);
        add(f::RED_CARDS, red);
        record_player_cards(s, entry, person);
    }
    if read_u32(row + lineup_row::PLAYER_ID) == read_u32(s.game + match_record::MAN_OF_THE_MATCH_ID) {
        add(f::MAN_OF_THE_MATCH, 1);
        record_player_tally(s, comp_records::MAN_OF_THE_MATCH, comp_record_kind::MAN_OF_THE_MATCH, read_u8(entry + f::MAN_OF_THE_MATCH), person);
    }
    write_u16(entry + f::RATING_TOTAL, read_u16(entry + f::RATING_TOTAL).wrapping_add(read_u8(row + lineup_row::RATING) as u16));
    if read_u8(s.game + match_record::NO_STATISTICS) == 0 {
        add_player_match_figures(s, k, row, entry);
    }
    record_age(s, comp_records::YOUNGEST, comp_record_kind::YOUNGEST, person, true);
    record_age(s, comp_records::OLDEST, comp_record_kind::OLDEST, person, false);
}

/// Whether recording the match only touches memory the comparer can put back: both clubs have their entry already
/// (adding one may free the saved runs), the players without an entry fit the table (no `realloc`), no club of a
/// league match of its own division has a run above 10 that a result could end (the media story), and no club's
/// cards reach its fine (the FA story and the payment).
pub(crate) unsafe fn record_match_memory_only(stats: usize, fixture: Fixture, game: usize) -> bool {
    let mut missing = 0u32;
    for side in 0..2usize {
        let club = if side == 0 { fixture.home_club() } else { fixture.away_club() };
        if club.is_null() {
            return false;
        }
        let entry = comp_stats_find_club_entry(stats, club.addr());
        if entry == 0 {
            return false;
        }
        let e = CompStatsClub(entry);
        if !club.division().is_null() && club.division() == fixture.competition() && e.unbeaten_run().max(e.winless_run()) > 10 {
            return false;
        }
        let team = recorded_team_block(game, side);
        let (yellow, red) = (read_u8(team + match_record_team::YELLOW_CARDS), read_u8(team + match_record_team::RED_CARDS));
        if (yellow != 0 || red != 0) && read_usize(G_FINANCE_TABLE) != 0 && read_usize(G_MANAGER_MANAGER) != 0 {
            let yellow = read_u8(entry + comp_club_figures::YELLOW_CARDS).wrapping_add(yellow);
            let red = read_u8(entry + comp_club_figures::RED_CARDS).wrapping_add(red);
            let points = (yellow as u16).wrapping_add((red as u16).wrapping_mul(4)) as i16;
            if discipline_fine_threshold(discipline_fine_level(club)).is_some_and(|t| points > t) {
                return false;
            }
        }
        for k in 0..20 {
            if let Some((_, _, sheet)) = match_lineup_row(game, side, k) {
                if comp_stats_find_player_entry(stats, sheet) == 0 {
                    missing += 1;
                }
            }
        }
    }
    missing <= read_u32(stats + comp_stats_obj::PLAYER_CAPACITY).wrapping_sub(read_u32(stats + comp_stats_obj::PLAYER_COUNT))
}

/// The round label of a record's match `m` (`exe::comp_util_format_stage_round_name` of the round and stage words at
/// +0xc and +0xe, 200 bytes, mode 0) and its gender (`exe::comp_util_get_label_gender` with no competition, the two
/// dwords at +0xc and +0xe and `form`).
pub(crate) unsafe fn comp_record_round_label(m: usize, form: u32, label: &mut [u8; 0xc8]) -> i32 {
    exe::comp_util_format_stage_round_name(read_i16(m + club_match_record::ROUND_PREFIX) as i32, read_i16(m + club_match_record::ROUND_NAME) as i32,
        label.as_mut_ptr() as usize, 0xc8, 0);
    exe::comp_util_get_label_gender(0, read_u32(m + club_match_record::ROUND_PREFIX), read_u32(m + club_match_record::ROUND_NAME), form) as i8 as i32
}

/// Side `side`'s block of a match record (`match_record_team`).
pub(crate) unsafe fn recorded_team_block(game: usize, side: usize) -> usize {
    game + match_record::HOME_TEAM + side * match_record::SIDE_SIZE
}

/// An attendance record (highest or lowest): outside friendlies, a positive attendance that beats the season's (or
/// fills it) becomes it with the club's match record; when it also beats the all-time one, that is copied from the
/// season's (`attendance_record_assign`), flagged from the third season end in a league division.
pub(crate) unsafe fn record_attendance_record(s: &RecordedSide, at: usize, kind: usize, highest: bool) {
    if s.friendly() {
        return;
    }
    let attendance = read_i32(s.game + match_record::ATTENDANCE);
    if attendance <= 0 {
        return;
    }
    let beats = |held: i32| held == -1 || if highest { attendance > held } else { attendance < held };
    let season = s.season(at);
    if !beats(read_i32(season)) {
        return;
    }
    write_i32(season, attendance);
    match_record_assign(season + comp_records::ATTENDANCE_GAME, s.record);
    let all = s.all_time(at);
    if !beats(read_i32(all)) {
        return;
    }
    if s.seasons_ended() >= 3 && s.league_division() {
        covered!("tel43_1 attendance record news");
        s.flag(kind);
    }
    attendance_record_assign(all, season);
}

/// The biggest win: outside friendlies, the club's match record replaces the season's when there is none or it is
/// a bigger win (`match_record_is_bigger_win`); the all-time one the same way, flagged for a margin of 8 or more.
pub(crate) unsafe fn record_biggest_win(s: &RecordedSide) {
    if s.friendly() {
        return;
    }
    let season = s.season(comp_records::BIGGEST_WIN);
    if read_i32(season + club_match_record::FIXTURE_ID) != -1 && match_record_is_bigger_win(s.record, season) == 0 {
        return;
    }
    match_record_assign(season, s.record);
    let all = s.all_time(comp_records::BIGGEST_WIN);
    if read_i32(all + club_match_record::FIXTURE_ID) != -1 && match_record_is_bigger_win(s.record, all) == 0 {
        return;
    }
    if s.goals_for() as i32 - s.goals_against() as i32 >= 8 {
        covered!("tel43_1 biggest win news");
        s.flag(comp_record_kind::BIGGEST_WIN);
    }
    match_record_assign(all, season);
}

/// A run of the club entry goes up by one game: the first game sets its start date and every game its end date (the
/// current date). Outside friendlies, a run longer than the season's record (or with none) becomes it with the
/// club's id (`run_record_assign`); longer than the all-time one too, that is copied from the season's
/// (`comp_run_record_assign`), flagged when the run reached `flag_from` games and the all-time run had another start
/// date or club (the same run still going raises no news).
pub(crate) unsafe fn extend_run(s: &RecordedSide, run_at: usize, record_at: usize, kind: usize, flag_from: i32) {
    let run = s.entry.addr() + run_at;
    let games = read_i32(run).wrapping_add(1);
    write_i32(run, games);
    if games == 1 {
        copy_record_date(run + run_record::START_DATE, G_CURRENT_DATE);
    }
    copy_record_date(run + run_record::END_DATE, G_CURRENT_DATE);
    if s.friendly() {
        return;
    }
    let season = s.season(record_at);
    let held = read_i32(season);
    if held != -1 && games <= held {
        return;
    }
    run_record_assign(season, run);
    write_u32(season + comp_records::RUN_CLUB, s.club_id());
    let all = s.all_time(record_at);
    let best = read_i32(all);
    if best != -1 && games <= best {
        return;
    }
    if games >= flag_from {
        let (start, held_start) = (run + run_record::START_DATE, all + run_record::START_DATE);
        if read_u16(held_start + 2) != read_u16(start + 2) || read_u16(held_start) != read_u16(start)
            || s.club_id() != read_u32(all + comp_records::RUN_CLUB) {
            covered!("tel43_1 run record news");
            s.flag(kind);
        }
    }
    comp_run_record_assign(all, season);
}

/// The media story of a run a result ends (the winless run ended by a win, the unbeaten run ended by a defeat): only
/// for a league match of the club's own division (`club.division()` = the fixture's competition) with a nation whose
/// league is selected and a manager. The run must beat `random(10) + 10`, not exceed the club's games, and then
/// `random(10000)` must stay below the club's reputation and `random(3)` be 0. The story names the man of the match
/// (the first person whose player record has the match's man of the match id, a valid id and, for the win, the club
/// as his club; for the defeat, another club), or nobody. The variant is the run's low byte over the search index's
/// high bytes, as the original pushes it (`post_team_media_story` reads only the byte).
pub(crate) unsafe fn run_ended_story(s: &RecordedSide, kind: u32, run_at: usize, own_club: bool) {
    let club = s.club;
    if club.division().is_null() || club.division().addr() != s.fixture.competition().addr() {
        return;
    }
    if club.nation().is_null() || club.nation().league_selected() == 0 || club.manager().is_null() {
        return;
    }
    let run = read_i32(s.entry.addr() + run_at);
    if run <= exe::utils_random(10) + 10 || (s.entry.matches_played() as i32) < run {
        return;
    }
    if exe::utils_random(10000) >= club.reputation() as i32 || exe::utils_random(3) != 0 {
        return;
    }
    covered!("tel43_1 run ended story");
    let count = read_i32(G_STAFF_COUNT);
    let star = read_u32(s.game + match_record::MAN_OF_THE_MATCH_ID);
    let (mut person, mut k) = (0usize, 0i32);
    while k < count {
        let staff = staff_at(k as isize);
        let player = staff.player();
        if !player.is_null() && read_u32(player.addr()) == star && (staff.is_null() || read_i32(staff.addr()) < count)
            && (staff.club_job() == club) == own_club {
            person = staff.addr();
            break;
        }
        k += 1;
    }
    let variant = (k as u32 & 0xffff_ff00) | read_u8(s.entry.addr() + run_at) as u32;
    exe::media_post_team_media_story(read_usize(G_MEDIA), club, kind, person as u32, s.fixture, variant, -1);
}

/// A team value record: above the season's (or with none, -1), the value, the club and a cleared year become it;
/// above the all-time one too, that takes the season's value and club and the competition's year, flagged from the
/// third season end when the holder or the year changes, in a league division.
pub(crate) unsafe fn record_team_value_record(s: &RecordedSide, at: usize, kind: usize, value: i16) {
    let season = s.season(at);
    let held = read_i16(season);
    if held != -1 && value <= held {
        return;
    }
    write_i16(season, value);
    write_u32(season + comp_records::VALUE_CLUB, s.club_id());
    write_u16(season + comp_records::VALUE_YEAR, 0);
    let all = s.all_time(at);
    let best = read_i16(all);
    if best != -1 && value <= best {
        return;
    }
    if s.seasons_ended() >= 3 && (s.other_year(all + comp_records::VALUE_YEAR) || s.club_id() != read_u32(all + comp_records::VALUE_CLUB))
        && s.league_division() {
        covered!("tel43_1 team value record news");
        s.flag(kind);
    }
    write_u32(all, read_u32(season));
    write_u32(all + 4, read_u32(season + 4));
    write_i16(all + comp_records::VALUE_YEAR, s.year());
}

/// A club's FA fine level: the low three bits of its board row's discipline counters, signed (0 without a club).
pub(crate) unsafe fn discipline_fine_level(club: Club) -> i8 {
    if club.is_null() {
        return 0;
    }
    let counters = BoardRecord(board_row(read_usize(G_MANAGER_MANAGER), club)).discipline_counters();
    ((counters & discipline_counters::FINE_LEVEL) << 5) as i8 >> 5
}

/// The card points a club must pass to be fined at `level` (0, 1, 2, and 3 for the last level); none below 0.
pub(crate) fn discipline_fine_threshold(level: i8) -> Option<i16> {
    match level {
        0 => Some(0x41),
        1 => Some(0x5a),
        2 => Some(0x73),
        l if l > 2 => Some(0x8c),
        _ => None,
    }
}

/// Rule: a club's cards in a competition bring an FA fine when their points (yellow + 4 × red, the competition's
/// totals) pass the threshold of its fine level: above 65 at level 0 (reputation × 3 + 2500), above 90 at level 1
/// ((reputation + 2000) × 5), above 115 at level 2 ((reputation + 2500) × 10) and, at level 3, above 140 one time in
/// ten (`random(10)` = 0, the same amount).
///
/// The club's finance (`financetable_get_club_finance`) is fetched first. A fine posts the FA's story
/// (`news_post_fa_disciplinary_fine` 0x58a4d0, cdecl(club, level, amount); it reads the level as a byte: the
/// original pushes it at level 3 as a dword whose high bytes it never set), raises the club's fine level by one and
/// takes the amount (`pay_league_fine`). Defect of the original: the level is three signed bits and the raise checks
/// "below 4", always true: level 3 becomes 4, read back as -4, and the club is never fined again.
pub(crate) unsafe fn charge_discipline_fine(club: Club, points: i16) {
    let level = discipline_fine_level(club);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let Some(threshold) = discipline_fine_threshold(level) else { return };
    if points <= threshold {
        return;
    }
    let reputation = || club.reputation() as i32;
    let amount = match level {
        0 => reputation() * 3 + 2500,
        1 => (reputation() + 2000) * 5,
        2 => (reputation() + 2500) * 10,
        _ => {
            if exe::utils_random(10) != 0 {
                return;
            }
            (reputation() + 2500) * 10
        }
    };
    covered!("tel43_1 FA fine");
    let post: unsafe extern "C" fn(usize, u32, i32) = core::mem::transmute(FN_NEWS_POST_FA_DISCIPLINARY_FINE);
    post(club.addr(), level as u8 as u32, amount);
    if !club.is_null() {
        let row = BoardRecord(board_row(read_usize(G_MANAGER_MANAGER), club));
        let counters = row.discipline_counters();
        let current = ((counters & discipline_counters::FINE_LEVEL) << 5) as i8 >> 5;
        if current < 4 {
            row.set_discipline_counters((counters & !discipline_counters::FINE_LEVEL) | ((current + 1) as u8 & discipline_counters::FINE_LEVEL));
        }
    }
    pay_league_fine(finance, amount); // tel43_1: the fine is paid
}

/// The figures of a match with statistics: the six u16 counts (passes, passes completed, tackles won, dribbles,
/// shots, shots on target) and the minutes: up to the minute he left the pitch, else the minute he was taken off,
/// else the match's length (`match_record::MINUTE`); a substitute (row 11 and up) who came on counts from his
/// entry minute.
pub(crate) unsafe fn add_player_match_figures(s: &RecordedSide, k: usize, row: usize, entry: usize) {
    use comp_player_figures as f;
    for (at, from) in [(f::PASSES, lineup_row::PASSES), (f::PASSES_COMPLETED, lineup_row::PASSES_COMPLETED), (f::TACKLES_WON, lineup_row::TACKLES_WON),
        (f::DRIBBLES, lineup_row::DRIBBLES), (f::SHOTS, lineup_row::SHOTS), (f::SHOTS_ON_TARGET, lineup_row::SHOTS_ON_TARGET)] {
        write_u16(entry + at, read_u16(entry + at).wrapping_add(read_u8(row + from) as u16));
    }
    let left = read_i8(row + lineup_row::MINUTE_LEFT_PITCH);
    let off = read_i8(row + lineup_row::MINUTE_OFF);
    let until = if left > -1 { left as i32 } else if off > -1 { off as i32 } else { read_i16(s.game + match_record::MINUTE) as i32 };
    let minutes = entry + f::MINUTES;
    if k < 11 {
        write_i32(minutes, read_i32(minutes).wrapping_add(until)); // tel43_1: a starter's minutes
    } else if read_u8(row + lineup_row::ENTRY_MINUTE) != 0xff {
        write_i32(minutes, read_i32(minutes).wrapping_add(until - read_i8(row + lineup_row::ENTRY_MINUTE) as i32));
    }
}

/// A player tally record (top scorer, assists, man of the match): outside friendlies, a count above the season's
/// (signed byte, or with none, 0xff) becomes it with the holder (`fill_staff_name_ref`), a cleared year and the club;
/// above the all-time one too, that takes them (`record_holder_assign`) and the competition's year, flagged from the
/// third season end when the person or the year changes, in a league division.
pub(crate) unsafe fn record_player_tally(s: &RecordedSide, at: usize, kind: usize, count: u8, person: Staff) {
    if s.friendly() {
        return;
    }
    let beats = |held: u8| held == 0xff || count as i32 > held as i8 as i32;
    let season = s.season(at);
    if !beats(read_u8(season)) {
        return;
    }
    write_u8(season, count);
    fill_staff_name_ref(person.addr(), season + comp_records::TALLY_HOLDER);
    write_u16(season + comp_records::TALLY_YEAR, 0);
    write_u32(season + comp_records::TALLY_CLUB, s.club_id());
    let all = s.all_time(at);
    if !beats(read_u8(all)) {
        return;
    }
    if s.seasons_ended() >= 3
        && (read_u32(all + comp_records::TALLY_HOLDER + record_holder::STAFF_ID) != read_u32(person.addr()) || s.other_year(all + comp_records::TALLY_YEAR))
        && s.league_division() {
        covered!("tel43_1 player tally news");
        s.flag(kind);
    }
    write_u8(all, read_u8(season));
    record_holder_assign(all + comp_records::TALLY_HOLDER, season + comp_records::TALLY_HOLDER);
    write_u32(all + comp_records::TALLY_CLUB, read_u32(season + comp_records::TALLY_CLUB));
    write_u16(all + comp_records::TALLY_YEAR, read_u16(season + comp_records::TALLY_YEAR));
    write_i16(all + comp_records::TALLY_YEAR, s.year());
}

/// The most goals in a match (kind 16): outside friendlies, more goals than the season's (signed byte, or none,
/// 0xff) make it, with the holder and the club's match record; more than the all-time one too, that is copied from
/// the season's, flagged for 5 goals or more.
pub(crate) unsafe fn record_goals_in_a_match(s: &RecordedSide, goals: u8, person: Staff) {
    if s.friendly() {
        return;
    }
    let beats = |held: u8| held == 0xff || goals as i32 > held as i8 as i32;
    let season = s.season(comp_records::GOALS_IN_A_MATCH);
    if !beats(read_u8(season)) {
        return;
    }
    write_u8(season, goals);
    fill_staff_name_ref(person.addr(), season + comp_records::TALLY_HOLDER);
    match_record_assign(season + comp_records::GOALS_IN_A_MATCH_GAME, s.record);
    let all = s.all_time(comp_records::GOALS_IN_A_MATCH);
    if !beats(read_u8(all)) {
        return;
    }
    if goals >= 5 {
        covered!("tel43_1 goals in a match news");
        s.flag(comp_record_kind::GOALS_IN_A_MATCH);
    }
    write_u8(all, read_u8(season));
    record_holder_assign(all + comp_records::TALLY_HOLDER, season + comp_records::TALLY_HOLDER);
    match_record_assign(all + comp_records::GOALS_IN_A_MATCH_GAME, season + comp_records::GOALS_IN_A_MATCH_GAME);
}

/// A player's cards record (kind 20, ordered by red × 3 + yellow of his totals): outside friendlies, above the
/// season's (or none, 0xff) the totals, the holder, a cleared year and the club make it; above the all-time one too,
/// that takes them and the competition's year, flagged from the third season end with 2 or more of each and another
/// person or year, in a league division.
pub(crate) unsafe fn record_player_cards(s: &RecordedSide, entry: usize, person: Staff) {
    use comp_player_figures as f;
    let (yellow, red) = (read_u8(entry + f::YELLOW_CARDS), read_u8(entry + f::RED_CARDS));
    let value = red as i32 * 3 + yellow as i32;
    let weight = |y: usize| read_i8(y + 1) as i32 * 3 + read_i8(y) as i32;
    if s.friendly() {
        return;
    }
    let season = s.season(comp_records::PLAYER_CARDS);
    if read_u8(season) != 0xff && value <= weight(season) {
        return;
    }
    write_u8(season, yellow);
    write_u8(season + 1, red);
    fill_staff_name_ref(person.addr(), season + comp_records::PLAYER_CARDS_HOLDER);
    write_u16(season + comp_records::PLAYER_CARDS_YEAR, 0);
    write_u32(season + comp_records::PLAYER_CARDS_CLUB, s.club_id());
    let all = s.all_time(comp_records::PLAYER_CARDS);
    if read_u8(all) != 0xff && value <= weight(all) {
        return;
    }
    if s.seasons_ended() >= 3 && yellow >= 2 && red >= 2
        && (read_u32(all + comp_records::PLAYER_CARDS_HOLDER + record_holder::STAFF_ID) != read_u32(person.addr()) || s.other_year(all + comp_records::PLAYER_CARDS_YEAR))
        && s.league_division() {
        covered!("tel43_1 player cards news");
        s.flag(comp_record_kind::PLAYER_CARDS);
    }
    write_u8(all, read_u8(season));
    write_u8(all + 1, read_u8(season + 1));
    record_holder_assign(all + comp_records::PLAYER_CARDS_HOLDER, season + comp_records::PLAYER_CARDS_HOLDER);
    write_u32(all + comp_records::PLAYER_CARDS_CLUB, read_u32(season + comp_records::PLAYER_CARDS_CLUB));
    write_u16(all + comp_records::PLAYER_CARDS_YEAR, read_u16(season + comp_records::PLAYER_CARDS_YEAR));
    write_i16(all + comp_records::PLAYER_CARDS_YEAR, s.year());
}

/// The youngest (kind 21) and oldest (kind 22) player records: outside friendlies, with no season record (its match
/// record's fixture id -1) or a person who beats it (`beats_age_record`), his birth date, the holder and the club's
/// match record make it; beating the all-time one too, that is copied from the season's (`player_age_record_assign`),
/// flagged from the third season end for an age under 20 years (youngest) or over 35 (oldest) of another person.
pub(crate) unsafe fn record_age(s: &RecordedSide, at: usize, kind: usize, person: Staff, youngest: bool) {
    if s.friendly() {
        return;
    }
    let season = s.season(at);
    if read_i32(season + comp_records::AGE_GAME + club_match_record::FIXTURE_ID) != -1 && !beats_age_record(season, person, youngest) {
        return;
    }
    copy_record_date(season, person.addr() + staff::DATE_OF_BIRTH);
    fill_staff_name_ref(person.addr(), season + comp_records::AGE_HOLDER);
    match_record_assign(season + comp_records::AGE_GAME, s.record);
    let all = s.all_time(at);
    if read_i32(all + comp_records::AGE_GAME + club_match_record::FIXTURE_ID) != -1 && !beats_age_record(all, person, youngest) {
        return;
    }
    if s.seasons_ended() >= 3 {
        let (_, years) = age_on(G_CURRENT_DATE, person.addr() + staff::DATE_OF_BIRTH);
        let notable = if youngest { years < 20 } else { years > 35 };
        if notable && read_u32(all + comp_records::AGE_HOLDER + record_holder::STAFF_ID) != read_u32(person.addr()) {
            covered!("tel43_1 age record news");
            s.flag(kind);
        }
    }
    player_age_record_assign(all, season);
}

/// A person's age as the date difference (`Date::get_difference`) from his birth date to `on`: (day, year).
pub(crate) unsafe fn age_on(on: usize, birth: usize) -> (i16, u16) {
    let mut out = [0u8; 8];
    let d = exe::date_get_difference(on, out.as_mut_ptr(), read_u32(birth), read_u32(birth + 4));
    (read_i16(d), read_u16(d + 2))
}

/// Whether the person beats an age record: his age today against the holder's age on the record's match date, the
/// years first (unsigned) and then the days (signed): younger for the youngest record, older for the oldest.
pub(crate) unsafe fn beats_age_record(record: usize, person: Staff, youngest: bool) -> bool {
    let (held_day, held_year) = age_on(record + comp_records::AGE_GAME, record);
    let (day, year) = age_on(G_CURRENT_DATE, person.addr() + staff::DATE_OF_BIRTH);
    if youngest {
        held_year > year || (held_year == year && held_day > day)
    } else {
        held_year < year || (held_year == year && held_day < day)
    }
}

/// `news_post_fa_disciplinary_fine` (0x58a4d0, cdecl(club, level, amount)): the story of a national FA fining a club
/// for its disciplinary record.
pub(crate) const FN_NEWS_POST_FA_DISCIPLINARY_FINE: usize = 0x58a4d0;

/// The media story kinds of the runs a result ends: a winless run ended by a win and an unbeaten run ended by a
/// defeat (`Media::post_team_media_story`).
pub(crate) const STORY_WINLESS_RUN_ENDED: u32 = 0x51;

pub(crate) const STORY_UNBEATEN_RUN_ENDED: u32 = 0x50;

/// One side of the match `record_match_result` is recording.
pub(crate) struct RecordedSide {
    pub(crate) stats: usize,
    pub(crate) fixture: Fixture,
    /// The match record (`match_record`: attendance, man of the match, the two side blocks and lineups).
    pub(crate) game: usize,
    pub(crate) side: usize,
    pub(crate) club: Club,
    pub(crate) entry: CompStatsClub,
    /// The club's match record (`club_match_record`, filled by `fill_match_record_from_fixture`).
    pub(crate) record: usize,
}

impl RecordedSide {
    pub(crate) unsafe fn comp(&self) -> usize {
        read_usize(self.stats + comp_stats_obj::COMP)
    }
    pub(crate) unsafe fn season(&self, at: usize) -> usize {
        self.stats + comp_stats_obj::SEASON_RECORDS + at
    }
    pub(crate) unsafe fn all_time(&self, at: usize) -> usize {
        self.stats + comp_stats_obj::ALL_TIME_RECORDS + at
    }
    pub(crate) unsafe fn friendly(&self) -> bool {
        exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(self.comp() + league::COMPETITION)) != 0
    }
    pub(crate) unsafe fn league_division(&self) -> bool {
        exe::comp_util_is_league_division(read_i32(read_usize(self.comp() + league::COMPETITION))) != 0
    }
    pub(crate) unsafe fn seasons_ended(&self) -> i16 {
        read_i16(self.stats + COMP_STATS_SEASONS_ENDED)
    }
    pub(crate) unsafe fn year(&self) -> i16 {
        read_i16(self.comp() + league::YEAR)
    }
    pub(crate) unsafe fn club_id(&self) -> u32 {
        read_u32(self.club.addr())
    }
    pub(crate) unsafe fn goals_for(&self) -> i8 {
        read_i8(self.record + club_match_record::GOALS_FOR)
    }
    pub(crate) unsafe fn goals_against(&self) -> i8 {
        read_i8(self.record + club_match_record::GOALS_AGAINST)
    }
    /// The club's block of the match record (`match_record_team`).
    pub(crate) unsafe fn team(&self) -> usize {
        recorded_team_block(self.game, self.side)
    }
    /// Raises record flag `kind` and the "any flag" dword.
    pub(crate) unsafe fn flag(&self, kind: usize) {
        write_u32(self.stats + comp_stats_record_flag(kind), 1);
        write_u32(self.stats + COMP_STATS_ANY_FLAG, 1);
    }
    /// A record's year word against the competition's year (signed against unsigned, as the original compares).
    pub(crate) unsafe fn other_year(&self, year_at: usize) -> bool {
        self.year() as i32 != read_u16(year_at) as i32
    }
}

/// The id of the record a pointer field points to (its first dword), or None when the pointer is null.
pub(crate) unsafe fn id_at(record: usize) -> Option<i32> {
    if record == 0 { None } else { Some(read_i32(record)) }
}
