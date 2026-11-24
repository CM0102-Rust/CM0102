// --- sx20_8: writing the shortlists and freeing the shortlist manager and the staff records (shortlist_manager.cpp:
// Shortlist_Manager::save 0x83b370, Shortlist_Manager::~Shortlist_Manager 0x839d90; staff_records.cpp:
// Staff_Records::~Staff_Records 0x8687c0) ---

/// Writes the shortlists into the open save item `item` (the transfer manager's): the by-person pair count and its
/// pairs (8 bytes; lines 0x3b6 and 0x3be), the 16 position list counts (0x3c7) and their capacity (0x3cf), the 32
/// position lists of 12-byte entries, each as long as its count (0x3db), the 9-byte club entries (0x3e5) and each
/// club's 40 slots of 0x25 bytes (0x3f3). An error shows the box and stops writing. The by-club pairs are not written
/// (the load copies and sorts the by-person ones).
/// Original: `Shortlist_Manager::save` (0x83b370, shortlist_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_save(this: usize, item: usize) {
    use shortlist_manager_fields as f;
    let write = |data: usize, size: u32, count: u32, line: u32| -> bool {
        if zip_write(data, size, count, item) != count {
            show_error_at(SHORTLIST_MANAGER_SOURCE, line);
            return false;
        }
        true
    };
    if !write(this + f::PAIRS_BY_PERSON_COUNT, 4, 1, 0x3b6) || !write(read_usize(this), 8, read_u32(this + f::PAIRS_BY_PERSON_COUNT), 0x3be) {
        return;
    }
    covered!("sx20_8 save shortlists");
    for k in 0..16 {
        if !write(this + f::POSITION_LIST_COUNTS + 4 * k, 4, 1, 0x3c7) {
            return;
        }
    }
    if !write(this + f::POSITION_LIST_CAPACITY, 4, 1, 0x3cf) {
        return;
    }
    for lists in [f::POSITION_LISTS_BY_PERSON, f::POSITION_LISTS_BY_KEY] {
        for k in 0..16 {
            if !write(read_usize(this + lists + 4 * k), 0xc, read_u32(this + f::POSITION_LIST_COUNTS + 4 * k), 0x3db) { // sx20_8: the position lists
                return;
            }
        }
    }
    if !write(read_usize(this + f::CLUB_SHORTLISTS), SHORTLIST_CLUB_ENTRY_SIZE as u32, read_u32(G_CLUB_COUNT), 0x3e5) {
        return;
    }
    for i in 0..read_i32(G_CLUB_COUNT) {
        let slots = read_usize(read_usize(this + f::CLUB_SHORTLISTS) + SHORTLIST_CLUB_ENTRY_SIZE * i as usize);
        if slots != 0 && !write(slots, SHORTLIST_SLOT_SIZE as u32, SHORTLIST_SLOTS as u32, 0x3f3) {
            return;
        }
    }
}

/// Frees the shortlist manager: when there is a club entry table, each club's slots (the pointer cleared) and the
/// table (cleared); the 32 position lists, the by-person and the by-club pair arrays, each cleared.
/// Original: `Shortlist_Manager::~Shortlist_Manager` (0x839d90, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_destructor(this: usize) {
    use shortlist_manager_fields as f;
    let free = |at: usize| {
        if read_usize(at) != 0 {
            exe::crt_free(read_usize(at));
            write_usize(at, 0);
        }
    };
    if read_usize(this + f::CLUB_SHORTLISTS) != 0 {
        for i in 0..read_i32(G_CLUB_COUNT) {
            let entry = read_usize(this + f::CLUB_SHORTLISTS) + SHORTLIST_CLUB_ENTRY_SIZE * i as usize;
            exe::crt_free(read_usize(entry));
            write_usize(entry, 0);
        }
        exe::crt_free(read_usize(this + f::CLUB_SHORTLISTS));
        write_usize(this + f::CLUB_SHORTLISTS, 0);
    }
    covered!("sx20_8 shortlist manager freed");
    for k in 0..32 {
        free(this + f::POSITION_LISTS_BY_PERSON + 4 * k);
    }
    free(this);
    free(this + f::PAIRS_BY_CLUB);
}

/// Frees the staff records: the person and club history temporary lists (`FILE_LLIST` destructor and `delete`), the
/// manager records and the two history list head tables; the pointers keep their values.
/// Original: `Staff_Records::~Staff_Records` (0x8687c0, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_records_destructor(this: usize) {
    use staff_records_fields as f;
    let destruct: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FILE_LLIST_DESTRUCT);
    for at in [f::PERSON_HISTORY_FILE, f::CLUB_HISTORY_FILE] {
        let list = read_usize(this + at);
        if list != 0 {
            destruct(list);
            exe::crt_operator_delete(list);
        }
    }
    covered!("sx20_8 staff records freed");
    for at in [f::MANAGER_RECORDS, f::PERSON_HISTORY_LISTS, f::CLUB_HISTORY_LISTS] {
        if read_usize(this + at) != 0 {
            exe::crt_free(read_usize(this + at));
        }
    }
}

/// Comparer for `shortlist_manager_save`.
pub(crate) unsafe extern "thiscall" fn compare_shortlist_manager_save(this: usize, item: usize) {
    // simplification: writes into the save item (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, item)
}

/// Comparer for `shortlist_manager_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_shortlist_manager_destructor(this: usize) {
    // simplification: frees memory (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Comparer for `staff_records_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_destructor(this: usize) {
    // simplification: frees memory (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
