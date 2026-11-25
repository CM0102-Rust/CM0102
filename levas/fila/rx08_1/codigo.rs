// ---------- rx08_1: the club records object: building, destroying and loading from the save (club_records.cpp) ----------

/// The club records object's own constants: the source file of its error boxes, the history temp file and the save
/// item, the sizes of its arrays and the two bytes a no-CD exe patches in the constructor's CD search.
pub(crate) mod club_records_consts {
    /// "E:\dev\CM3\cm3 00-01\cm3\code\club_records.cpp" (the error boxes).
    pub(crate) const CLUB_RECORDS_CPP: usize = 0x98b6a0;
    /// "club records history.tmp" (in CM3_TEMP) and its record size.
    pub(crate) const HISTORY_TMP: usize = 0x98b6d0;
    pub(crate) const HISTORY_RECORD_SIZE: u32 = 0x38d;
    /// "club records.dat", the item of the saved game.
    pub(crate) const SAVE_ITEM: usize = 0x98de70;
    /// The history file list (FILE_LLIST) at +0 and the number of per-club entries at +0x20 (the club count).
    pub(crate) const HISTORY_LIST: usize = 0;
    pub(crate) const CLUB_ENTRY_COUNT: usize = 0x20;
    /// Size of one all-time or season record set, of one per-club entry and of one queued record news item.
    pub(crate) const RECORD_SIZE: u32 = 0x38b;
    pub(crate) const ENTRY_SIZE: u32 = 0x6a;
    pub(crate) const NEWS_SIZE: u32 = 6;
    /// The constructor's CD search, as patched by a no-CD exe (cm0102-1080): the drive type's `jne` and the found test.
    pub(crate) const CD_DRIVE_TYPE_JUMP: usize = 0x444577;
    pub(crate) const CD_FOUND_TEST: usize = 0x4445b9;
}

/// Empties one per-club entry: the all-time and season indices and the two other indices −1; the season games, the
/// byte +0x11, the word +0x12, the dwords +0x14, +0x18, +0x2c, +0x40 and +0x54 and the seasons recorded zero. The
/// other bytes are left as they are.
pub(crate) unsafe fn reset_club_record_entry(e: usize) {
    for k in [0usize, club_record_entry::SEASON_INDEX, 8, 0xc] {
        write_u32(e + k, u32::MAX);
    }
    write_u8(e + club_record_entry::SEASON_GAMES, 0);
    write_u8(e + 0x11, 0); // a second season counter [inference]
    write_u16(e + 0x12, 0);
    for k in [0x14usize, 0x18, 0x2c, 0x40, 0x54] {
        write_u32(e + k, 0);
    }
    write_u16(e + club_record_entry::SEASONS, 0);
}

/// Builds the club records object (`G_CLUB_RECORDS`): waits for the game CD, opens the history temp file and, with a
/// save, loads the records; without one (or when the load answers 0) starts empty: one entry per club and, for every
/// national team, the most capped of its players as the "international caps" record.
///
/// `Club_Records::Club_Records(save)`: the CD search (`find_game_cd` with this routine's two patchable bytes). The
/// history file list (`club_history_new_file_list`: operator new(8) and FILE_LLIST::FILE_LLIST of
/// "CM3_TEMP\club records history.tmp" with records of 0x38d bytes) at +0; none stops the game (line 0x50). With a
/// save, `club_records_load`; when it answers, that is all. Otherwise the two record pools with their capacities and
/// counts (`club_records_fields::ALL_TIME_*`, `SEASON_*`) zero, the entry count the club count (`G_CLUB_COUNT`), the
/// entries (`malloc` of count × 0x6a; none stops the game, line 0x77), the news queue with its capacity and count
/// zero, every entry emptied (`reset_club_record_entry`). Then each person (`G_STAFF_COUNT`, signed) with
/// international appearances and a nation: the nation's national team (`exe::database_get_national_team`), its
/// all-time records (`exe::club_records_get_all_time_record`, creating them), and when they have no caps holder (−1)
/// or fewer caps (signed word) than the person, the person becomes the holder (`club_record::INTERNATIONAL_CAPS` and
/// the name reference just after it, `exe::fill_staff_name_ref`). Returns the object.
/// Original: `Club_Records::Club_Records` (0x444520, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_club_records(this: usize, save: usize) -> usize {
    use club_records_consts as c;
    use club_records_fields as f;
    find_game_cd(c::CD_DRIVE_TYPE_JUMP, c::CD_FOUND_TEST);
    let list = club_history_new_file_list(c::HISTORY_TMP, c::HISTORY_RECORD_SIZE);
    write_usize(this + c::HISTORY_LIST, list);
    if list == 0 {
        fatal_error_at(c::CLUB_RECORDS_CPP, 0x50);
    }
    if save != 0 && club_records_load(this, save) != 0 {
        covered!("rx08_1 club records from the save");
        return this;
    }
    covered!("rx08_1 new club records");
    for k in [f::ALL_TIME_CAPACITY, f::ALL_TIME_COUNT, f::ALL_TIME_RECORDS, f::SEASON_COUNT, f::SEASON_RECORDS, f::SEASON_CAPACITY] {
        write_u32(this + k, 0);
    }
    let count = read_u32(G_CLUB_COUNT);
    write_u32(this + c::CLUB_ENTRY_COUNT, count);
    let entries = exe::crt_malloc(count.wrapping_mul(c::ENTRY_SIZE));
    write_usize(this + f::CLUB_ENTRIES, entries);
    if entries == 0 {
        fatal_error_at(c::CLUB_RECORDS_CPP, 0x77);
    }
    for k in [f::NEWS_QUEUE_CAPACITY, f::NEWS_QUEUE_COUNT, f::NEWS_QUEUE] {
        write_u32(this + k, 0);
    }
    for k in 0..read_i32(this + c::CLUB_ENTRY_COUNT).max(0) as usize {
        reset_club_record_entry(entries + k * c::ENTRY_SIZE as usize);
    }
    for id in 0..read_i32(G_STAFF_COUNT).max(0) as isize {
        let person = staff_at(id);
        let caps = person.int_apps();
        if caps == 0 || person.nation().is_null() {
            continue;
        }
        let team = exe::database_get_national_team(person.nation());
        let record = ClubRecord(exe::club_records_get_all_time_record(this, team, 1));
        let held = record.international_caps() as i16;
        if held == -1 || (caps as i16) > held { // rx08_1: the most capped player holds the record
            record.set_international_caps(caps as u16);
            exe::fill_staff_name_ref(person, record.addr() + club_record::INTERNATIONAL_CAPS + 2);
        }
    }
    this
}

/// Comparer for `new_club_records`: it searches the CD, opens the temp file and allocates (or loads): it cannot run
/// twice. Only the original runs; the copy is proven by the new career (Rust mode) and by loading the holiday save
/// (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_new_club_records(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Destroys the club records object: the history file list, the two record pools, the per-club entries and the news
/// queue.
///
/// `Club_Records::~Club_Records()`: the history list present is destroyed (FILE_LLIST::~FILE_LLIST, then `operator
/// delete`) and zeroed; each array present is freed (`free`) and its pointer zeroed with its capacity and count (the
/// entries with their count).
/// Original: `Club_Records::~Club_Records` (0x4448a0, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_club_records(this: usize) {
    use club_records_consts as c;
    use club_records_fields as f;
    let list = read_usize(this + c::HISTORY_LIST);
    if list != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(club_history_consts::FILE_LLIST_DESTRUCTOR);
        destroy(list);
        exe::crt_operator_delete(list);
        write_usize(this + c::HISTORY_LIST, 0);
    }
    let arrays: [(usize, &[usize]); 4] = [(f::ALL_TIME_RECORDS, &[f::ALL_TIME_CAPACITY, f::ALL_TIME_COUNT]),
        (f::SEASON_RECORDS, &[f::SEASON_CAPACITY, f::SEASON_COUNT]), (f::CLUB_ENTRIES, &[c::CLUB_ENTRY_COUNT]),
        (f::NEWS_QUEUE, &[f::NEWS_QUEUE_CAPACITY, f::NEWS_QUEUE_COUNT])];
    for (pointer, sizes) in arrays {
        let p = read_usize(this + pointer);
        if p != 0 {
            exe::crt_free(p);
            write_usize(this + pointer, 0);
            for &k in sizes {
                write_u32(this + k, 0);
            }
        }
    }
}

/// Comparer for `drop_club_records`: it frees, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_club_records(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Reads `n` items of `size` bytes from an open save item; a short read shows the error box of `file` (the source
/// file's path) at `row` and stops the game.
pub(crate) unsafe fn zread_or_stop(file: usize, row: u32, z: usize, data: usize, size: u32, n: u32) {
    if exe::zread(data, size, n, z) != n {
        fatal_error_at(file, row);
    }
}

/// `zread_or_stop` with club_records.cpp's boxes.
unsafe fn club_records_read_or_stop(z: usize, data: usize, size: u32, n: u32, row: u32) {
    zread_or_stop(club_records_consts::CLUB_RECORDS_CPP, row, z, data, size, n);
}

/// Reads one array of the save item: its capacity and its count (dwords), then, with a positive capacity, the array
/// (`malloc` of capacity × `size`, kept in `pointer` before the check) and its `count` items when there are any;
/// without a capacity the pointer is zero. The four rows are the boxes of the two reads, the allocation and the items.
unsafe fn club_records_read_array(this: usize, z: usize, (pointer, capacity, count): (usize, usize, usize), size: u32, rows: [u32; 4]) {
    club_records_read_or_stop(z, this + capacity, 4, 1, rows[0]);
    club_records_read_or_stop(z, this + count, 4, 1, rows[1]);
    let cap = read_i32(this + capacity);
    if cap <= 0 {
        write_usize(this + pointer, 0);
        return;
    }
    let p = exe::crt_malloc((cap as u32).wrapping_mul(size));
    write_usize(this + pointer, p);
    if p == 0 {
        fatal_error_at(club_records_consts::CLUB_RECORDS_CPP, rows[2]);
    }
    let n = read_i32(this + count);
    if n > 0 {
        club_records_read_or_stop(z, p, size, n as u32, rows[3]);
    }
}

/// Whether a loaded per-club entry is usable: the season index below the season capacity and the all-time index below
/// the all-time capacity (the capacities, not the counts), and the four indices not below −1 (signed).
unsafe fn club_record_entry_is_valid(this: usize, e: usize) -> bool {
    use club_records_fields as f;
    let (all_time, season) = (read_i32(e), read_i32(e + club_record_entry::SEASON_INDEX));
    season < read_i32(this + f::SEASON_CAPACITY) && season >= -1 && all_time < read_i32(this + f::ALL_TIME_CAPACITY)
        && all_time >= -1 && read_i32(e + 8) >= -1 && read_i32(e + 0xc) >= -1
}

/// Reads the club records from the saved game's "club records.dat" item.
///
/// `Club_Records::load(save)`: no save: the club_records.cpp 0x1515 box and 0; the item missing (`zopen` with 0, 0x16,
/// 4): the 0x151d box and 0. Then, each short read or failed allocation stopping the game: the all-time pool
/// (`club_records_read_array`: capacity, count, 0x38b-byte records; rows 0x1524, 0x152c, 0x1538, 0x1543), the season
/// pool (0x1552, 0x155a, 0x1566, 0x1571); the entry count (0x1580), the entries (`malloc` of count × 0x6a, 0x1589;
/// count entries read, 0x1591); each entry checked (`club_record_entry_is_valid`): a bad one shows the 0x15a2 box the
/// first time (the game goes on) and is emptied (`reset_club_record_entry`); the news queue (capacity, count, 6-byte
/// items; 0x15b9, 0x15c1, 0x15cd, 0x15d8). The item closed; 1.
/// Original: `Club_Records::load` (0x4515a0, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_records_load(this: usize, save: usize) -> u32 {
    use club_records_consts as c;
    use club_records_fields as f;
    if save == 0 {
        show_error_at(c::CLUB_RECORDS_CPP, 0x1515);
        return 0;
    }
    let mut z = exe::zopen(save, c::SAVE_ITEM, 0, 0x16, 4);
    if z == 0 {
        show_error_at(c::CLUB_RECORDS_CPP, 0x151d);
        return 0;
    }
    covered!("rx08_1 load club records");
    club_records_read_array(this, z, (f::ALL_TIME_RECORDS, f::ALL_TIME_CAPACITY, f::ALL_TIME_COUNT), c::RECORD_SIZE, [0x1524, 0x152c, 0x1538, 0x1543]);
    club_records_read_array(this, z, (f::SEASON_RECORDS, f::SEASON_CAPACITY, f::SEASON_COUNT), c::RECORD_SIZE, [0x1552, 0x155a, 0x1566, 0x1571]);
    club_records_read_or_stop(z, this + c::CLUB_ENTRY_COUNT, 4, 1, 0x1580);
    let count = read_i32(this + c::CLUB_ENTRY_COUNT);
    let entries = exe::crt_malloc((count as u32).wrapping_mul(c::ENTRY_SIZE));
    write_usize(this + f::CLUB_ENTRIES, entries);
    if entries == 0 {
        fatal_error_at(c::CLUB_RECORDS_CPP, 0x1589);
    }
    club_records_read_or_stop(z, entries, c::ENTRY_SIZE, count as u32, 0x1591);
    let mut warned = false;
    for k in 0..count.max(0) as usize {
        let e = entries + k * c::ENTRY_SIZE as usize;
        if club_record_entry_is_valid(this, e) {
            continue;
        }
        covered!("rx08_1 bad club records entry");
        if !warned {
            warned = true;
            show_error_at(c::CLUB_RECORDS_CPP, 0x15a2);
        }
        reset_club_record_entry(e); // rx08_1: a bad entry starts over
    }
    club_records_read_array(this, z, (f::NEWS_QUEUE, f::NEWS_QUEUE_CAPACITY, f::NEWS_QUEUE_COUNT), c::NEWS_SIZE, [0x15b9, 0x15c1, 0x15cd, 0x15d8]);
    exe::zclose(&mut z);
    1
}

/// Comparer for `club_records_load`: it allocates and reads the save item, so it cannot run twice; only the original
/// runs. The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_club_records_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, save)
}
