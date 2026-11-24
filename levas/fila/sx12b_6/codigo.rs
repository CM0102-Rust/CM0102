// ---------- batch %%LEVA%%: the static tables and the destructors of the human managers, injuries and key nations (hall_of_fame.cpp, human_manager.cpp, injury.cpp, key_nation.cpp; sx12b_6) ----------

/// Builds the 35 static key nation entries (`G_LEAGUE_TABLE`, 72 bytes each) at program start
/// (`KEY_NATION::KEY_NATION` by the vector iterator, the empty destructor) and registers their destructor with
/// `atexit`.
/// Original: `key_nation_static_init` (0x6670b0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_static_init() {
    exe::crt_vector_constructor_iterator(G_LEAGUE_TABLE, 72, 0x23, 0x667100usize, 0x5a53c0usize); // KEY_NATION::KEY_NATION, the empty destructor
    exe::crt_atexit(0x6670e0usize); // key_nation_static_destroy
}

/// Destroys the 35 static key nation entries at program exit (the vector iterator with the empty destructor).
/// Original: `key_nation_static_destroy` (0x6670e0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_static_destroy() {
    exe::crt_vector_destructor_iterator(G_LEAGUE_TABLE, 72, 0x23, 0x5a53c0usize);
}

/// The hall of fame import buffer: 2800 entries of 120 bytes, read by the import of a hall of fame file.
pub(crate) const HALL_OF_FAME_IMPORT_BUFFER: usize = 0xb0be38;

/// Builds the 2800 entries of the hall of fame import buffer (`HALL_OF_FAME_IMPORT_BUFFER`) at program start
/// (`HallOfFameEntry::HallOfFameEntry` by the vector iterator, the empty destructor) and registers their destructor
/// with `atexit`.
/// Original: `hall_of_fame_import_buffer_init` (0x5efd00, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_import_buffer_init() {
    exe::crt_vector_constructor_iterator(HALL_OF_FAME_IMPORT_BUFFER, 0x78, 0xaf0, 0x5ef120usize, 0x5a53c0usize); // HallOfFameEntry::HallOfFameEntry
    exe::crt_atexit(0x5efd30usize); // hall_of_fame_import_buffer_destroy
}

/// Destroys the 2800 entries of the hall of fame import buffer at program exit (the vector iterator with the empty
/// destructor).
/// Original: `hall_of_fame_import_buffer_destroy` (0x5efd30, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_import_buffer_destroy() {
    exe::crt_vector_destructor_iterator(HALL_OF_FAME_IMPORT_BUFFER, 0x78, 0xaf0, 0x5a53c0usize);
}

/// Frees the injury object's state (the object itself is deleted by the caller).
///
/// `Injury::~Injury()`: frees (`free`) the physical records and the injuries when present, destroys the history file
/// list (`FILE_LLIST::~FILE_LLIST`, `operator delete`), frees the recovery news queue, destroys the match knock list
/// (`LLIST::~LLIST`, `operator delete`) and frees the outbreaks; nothing is zeroed.
/// Original: `Injury::~Injury` (0x62a8a0, injury.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn injury_destructor(this: usize) {
    use injury_fields as f;
    let free_field = |field: usize| {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
        }
    };
    let delete_field = |field: usize, destructor: usize| {
        let p = read_usize(this + field);
        if p != 0 {
            let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destructor);
            destroy(p);
            exe::crt_operator_delete(p);
        }
    };
    free_field(f::PHYSICAL_RECORDS);
    free_field(f::INJURY_ENTRIES);
    delete_field(f::INJURY_HISTORY, 0x58e330); // FILE_LLIST::~FILE_LLIST
    free_field(f::RECOVERY_NEWS_QUEUE);
    delete_field(f::MATCH_HIT_LIST, FN_LLIST_DESTRUCTOR_LLIST);
    free_field(f::OUTBREAKS);
}

/// Frees the human manager table.
///
/// `HUMAN_MANAGER::~HUMAN_MANAGER()`: with a table, for each record the two pending offers (+0x9a59, +0x9a5d) are
/// dropped (the offer's first dword zeroed, then `TRANSFER_OFFER::~TRANSFER_OFFER` and `operator delete`) and the
/// nickname array (`human_manager_record::NICKNAMES`) freed; then the records are destroyed (`HUMAN_RECORD::~HUMAN_RECORD`
/// by the vector iterator, the count in the dword before them) and the block deleted; the table pointer and
/// `human_manager_fields::RECORD_COUNT` zeroed. Without a table, nothing.
/// Original: `HUMAN_MANAGER::~HUMAN_MANAGER` (0x5fb730, human_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn human_manager_destructor(this: usize) {
    if read_usize(this) == 0 {
        return;
    }
    let mut k = 0;
    while k < read_i32(this + human_manager_fields::RECORD_COUNT) {
        let record = human_manager_record_at(this, k as usize);
        for offer in [0x9a59, 0x9a5d] {
            let p = read_usize(record + offer);
            if p != 0 {
                write_u32(p, 0);
                let p = read_usize(record + offer);
                if p != 0 {
                    exe::drop_transfer_offer(p);
                    exe::crt_operator_delete(p);
                }
            }
        }
        let nicknames = read_usize(record + human_manager_record::NICKNAMES);
        if nicknames != 0 {
            exe::crt_free(nicknames);
        }
        k += 1;
    }
    let records = read_usize(this);
    if records != 0 {
        exe::crt_vector_destructor_iterator(records, 0xa6ef, read_u32(records - 4), 0x5fb700usize); // HUMAN_RECORD::~HUMAN_RECORD
        exe::crt_operator_delete(records - 4);
    }
    write_usize(this, 0);
    write_u32(this + human_manager_fields::RECORD_COUNT, 0);
}

/// Destroys one human record: its five tactic slots by the vector iterator (the empty destructor).
/// Original: `HUMAN_RECORD::~HUMAN_RECORD` (0x5fb700, human_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn human_record_destructor(this: usize) {
    exe::crt_vector_destructor_iterator(this, 0x1ecd, 5, 0x5a53c0usize);
}

/// Deletes the competitions of every key nation row: each row's comp objects (+0x10, +0xc of them) are deleted by their
/// virtual destructor (vtable slot 0 with 1) and their slots zeroed, then the array freed and the array and count
/// zeroed. Rows without an array are left alone.
/// Original: `key_nation_delete_all_comps` (0x669290, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_delete_all_comps() {
    for k in 0..0x23usize {
        let row = G_LEAGUE_TABLE + 72 * k;
        if read_usize(row + 0x10) == 0 {
            continue; // 0x10: the row's comp array, +0xc its count
        }
        let mut i = 0;
        while i < read_i32(row + 0xc) {
            let slot = read_usize(row + 0x10) + 4 * i as usize;
            let comp = read_usize(slot);
            if comp != 0 {
                let delete: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(comp)));
                delete(comp, 1);
                write_usize(read_usize(row + 0x10) + 4 * i as usize, 0);
            }
            i += 1;
        }
        exe::crt_free(read_usize(row + 0x10));
        write_usize(row + 0x10, 0);
        write_u32(row + 0xc, 0);
    }
}

/// Comparer for `key_nation_static_init`: only the original (the atexit registration does not run twice).
pub(crate) unsafe extern "C" fn compare_key_nation_static_init() {
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `key_nation_static_destroy`: only the original (program exit).
pub(crate) unsafe extern "C" fn compare_key_nation_static_destroy() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// Comparer for `hall_of_fame_import_buffer_init`: only the original (the atexit registration does not run twice).
pub(crate) unsafe extern "C" fn compare_hall_of_fame_import_buffer_init() {
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

/// Comparer for `hall_of_fame_import_buffer_destroy`: only the original (program exit).
pub(crate) unsafe extern "C" fn compare_hall_of_fame_import_buffer_destroy() {
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer for `injury_destructor`: it frees the lists and arrays: only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_injury_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

/// Comparer for `human_manager_destructor`: it frees the offers, nicknames and records: only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_human_manager_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

/// Comparer for `human_record_destructor`: only the original (part of freeing the table).
pub(crate) unsafe extern "thiscall" fn compare_human_record_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

/// Comparer for `key_nation_delete_all_comps`: it deletes the competitions: only the original runs.
pub(crate) unsafe extern "C" fn compare_key_nation_delete_all_comps() {
    original_fn::<unsafe extern "C" fn()>(%%I7%%)()
}
