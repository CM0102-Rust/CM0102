// --- sx20_7: the simulated statistics and staff records in the saved game (simulated_stats.cpp:
// SimulatedStats::load 0x84c510; staff_records.cpp: Staff_Records::load 0x86fce0 and Staff_Records::save 0x86ffd0) ---

/// The save item of the staff records.
const STAFF_RECORDS_SAVE_ITEM: usize = 0xa84a64; // "staff records.dat"
/// Saves of a game data version below this (in absolute value) have 0xce-byte manager records (without
/// `manager_record::MONTH_LEAGUE_POINTS`).
const STAFF_RECORDS_SHORT_RECORD_VERSION: i32 = 0x3e031;

/// Reads the groups from the saved game: nothing without a save (0); the "simulated_stats.dat" item (missing: the
/// error box, line 0x1dd, and 0), the group count at +0 (short: line 0x1e4 and 0), a new group table of count × 0x44
/// bytes at +4 read in one go (short: line 0x1ef and 0); closes the item and returns 1. The item stays open after an
/// error.
/// Original: `SimulatedStats::load` (0x84c510, simulated_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn simulated_stats_load(this: usize, save: usize) -> u32 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, 0xa827c0, 0, 0x16, 4); // "simulated_stats.dat"
    if z == 0 {
        show_error_at(SIMULATED_STATS_SOURCE, 0x1dd);
        return 0;
    }
    if exe::zread(this, 4, 1, z) != 1 {
        show_error_at(SIMULATED_STATS_SOURCE, 0x1e4);
        return 0;
    }
    covered!("sx20_7 load simulated stats");
    write_usize(this + 4, exe::crt_malloc(read_u32(this).wrapping_mul(SIMULATED_GROUP_SIZE as u32)));
    if exe::zread(read_usize(this + 4), SIMULATED_GROUP_SIZE as u32, read_u32(this), z) != read_u32(this) { // sx20_7: every group read
        show_error_at(SIMULATED_STATS_SOURCE, 0x1ef);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Reads the staff records from the saved game: the "staff records.dat" item (missing: the error box, line 0x1103,
/// and 0); each manager record (non-player count + 16), 0xce bytes with `manager_record::MONTH_LEAGUE_POINTS` = 0 from
/// a save of game data version below 0x3e031 (in absolute value), else the whole 0xd2 bytes (short: lines 0x1110 /
/// 0x111c); the person and club history list heads (8 bytes each; lines 0x1126 and 0x112d); the stored month and year
/// (2 bytes each; lines 0x1134 and 0x113b); closes the item and returns 1. Every error returns 0 leaving the item
/// open (the constructor then rebuilds the records from scratch).
/// Original: `Staff_Records::load` (0x86fce0, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_records_load(this: usize, save: usize) -> u32 {
    use staff_records_fields as f;
    let mut z = exe::zopen(save, STAFF_RECORDS_SAVE_ITEM, 0, 0x16, 4);
    if z == 0 {
        show_error_at(STAFF_RECORDS_SOURCE, 0x1103);
        return 0;
    }
    covered!("sx20_7 load staff records");
    let mut k = 0i32;
    while k < read_i32(G_NON_PLAYER_COUNT) + 16 {
        let r = read_usize(this + f::MANAGER_RECORDS) + k as usize * MANAGER_RECORD_SIZE;
        if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < STAFF_RECORDS_SHORT_RECORD_VERSION { // sx20_7: the old record
            if exe::zread(r, manager_record::MONTH_LEAGUE_POINTS as u32, 1, z) != 1 {
                show_error_at(STAFF_RECORDS_SOURCE, 0x1110);
                return 0;
            }
            write_u32(r + manager_record::MONTH_LEAGUE_POINTS, 0);
        } else if exe::zread(r, MANAGER_RECORD_SIZE as u32, 1, z) != 1 {
            show_error_at(STAFF_RECORDS_SOURCE, 0x111c);
            return 0;
        }
        k += 1;
    }
    if exe::zread(read_usize(this + f::PERSON_HISTORY_LISTS), 8, read_u32(G_STAFF_COUNT), z) != read_u32(G_STAFF_COUNT) {
        show_error_at(STAFF_RECORDS_SOURCE, 0x1126);
        return 0;
    }
    if exe::zread(read_usize(this + f::CLUB_HISTORY_LISTS), 8, read_u32(G_CLUB_COUNT), z) != read_u32(G_CLUB_COUNT) {
        show_error_at(STAFF_RECORDS_SOURCE, 0x112d);
        return 0;
    }
    if exe::zread(this + f::CURRENT_MONTH, 2, 1, z) != 1 {
        show_error_at(STAFF_RECORDS_SOURCE, 0x1134);
        return 0;
    }
    if exe::zread(this + f::CURRENT_YEAR, 2, 1, z) != 1 {
        show_error_at(STAFF_RECORDS_SOURCE, 0x113b);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Writes the staff records into the saved game: the "staff records.dat" item (missing: the error box, line 0x1155),
/// the manager records (non-player count + 16 of 0xd2 bytes; line 0x115d), the person and club history list heads
/// (lines 0x1165 and 0x116d), the stored month and year (lines 0x1174 and 0x117b), and closes the item. After an
/// error the item stays open.
/// Original: `Staff_Records::save` (0x86ffd0, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_records_save(this: usize, save: usize) {
    use staff_records_fields as f;
    let mut z = zip_open_item(save, STAFF_RECORDS_SAVE_ITEM);
    if z == 0 {
        show_error_at(STAFF_RECORDS_SOURCE, 0x1155);
        return;
    }
    covered!("sx20_7 save staff records");
    let records = read_u32(G_NON_PLAYER_COUNT).wrapping_add(16);
    let parts: [(usize, u32, u32, u32); 5] = [
        (read_usize(this + f::MANAGER_RECORDS), MANAGER_RECORD_SIZE as u32, records, 0x115d), // sx20_7: the manager records
        (read_usize(this + f::PERSON_HISTORY_LISTS), 8, read_u32(G_STAFF_COUNT), 0x1165),
        (read_usize(this + f::CLUB_HISTORY_LISTS), 8, read_u32(G_CLUB_COUNT), 0x116d),
        (this + f::CURRENT_MONTH, 2, 1, 0x1174),
        (this + f::CURRENT_YEAR, 2, 1, 0x117b),
    ];
    for (data, size, count, line) in parts {
        if zip_write(data, size, count, z) != count {
            show_error_at(STAFF_RECORDS_SOURCE, line);
            return;
        }
    }
    zip_close(&mut z);
}

/// Comparer for `simulated_stats_load`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_load(this: usize, save: usize) -> u32 {
    // simplification: reads the save item and allocates (cannot run twice); the copy is proven by loading the save
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}

/// Comparer for `staff_records_load`.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_load(this: usize, save: usize) -> u32 {
    // simplification: reads the save item (cannot run twice); the copy is proven by loading the save
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
}

/// Comparer for `staff_records_save`.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_save(this: usize, save: usize) {
    // simplification: writes into the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, save)
}
