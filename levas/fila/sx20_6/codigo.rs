// --- sx20_6: building the staff records (staff_records.cpp, Staff_Records::Staff_Records 0x868390) ---

/// staff_records.cpp, the source path of the error boxes.
pub(crate) const STAFF_RECORDS_SOURCE: usize = 0xa83b78;
/// The size of a manager record (`manager_record`) and the records beyond the non-player count (16 humans).
pub(crate) const MANAGER_RECORD_SIZE: usize = 0xd2;
const STAFF_RECORDS_CD_DRIVE_TYPE_JUMP: usize = 0x86840b;
const STAFF_RECORDS_CD_FOUND_TEST: usize = 0x86844e;

/// The manager record of a non-player (its index is the first dword of the non-player record).
unsafe fn staff_records_record_of_non_player(this: usize, non_player: usize) -> usize {
    read_usize(this + staff_records_fields::MANAGER_RECORDS).wrapping_add((read_i32(non_player) as usize).wrapping_mul(MANAGER_RECORD_SIZE))
}

/// Builds the staff records: waits for the game CD (`wait_for_game_cd`), clears the stored month and year, opens the
/// person and club history temporary lists ("staff history.tmp" with 0x35-byte records, "club history.tmp" with 0x31;
/// `club_history_new_file_list`), and allocates the manager records (non-player count + 16, zeroed) and the history
/// list heads (8 bytes per person and per club); out of memory, the error box (lines 0x87, 0x8f, 0x97) and the
/// records stay half built. With a saved game `save`, loads them (`Staff_Records::load`); without one, or when the
/// load fails, every manager record gets no owner, season nation, national nation, current, longest or shortest club
/// (-1), each club or national team manager (job 5 or 12) with a non-player record owns his record with one club or
/// national job, and every list head is empty (-1). Returns `this`.
/// Original: `Staff_Records::Staff_Records` (0x868390, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_records_staff_records(this: usize, save: usize) -> usize {
    use staff_records_fields as f;
    // simplification: the original also builds a local Date (13/11/1975) that nothing reads
    wait_for_game_cd(STAFF_RECORDS_CD_DRIVE_TYPE_JUMP, STAFF_RECORDS_CD_FOUND_TEST);
    write_u16(this + f::CURRENT_YEAR, 0);
    write_u16(this + f::CURRENT_MONTH, 0);
    write_usize(this + f::PERSON_HISTORY_FILE, club_history_new_file_list(0xa83bbc, 0x35)); // "staff history.tmp"
    write_usize(this + f::CLUB_HISTORY_FILE, club_history_new_file_list(0xa83ba8, 0x31)); // "club history.tmp"
    let records = exe::crt_calloc(read_u32(G_NON_PLAYER_COUNT).wrapping_add(16), MANAGER_RECORD_SIZE as u32);
    write_usize(this + f::MANAGER_RECORDS, records);
    if records == 0 {
        show_error_at(STAFF_RECORDS_SOURCE, 0x87);
        return this;
    }
    write_usize(this + f::PERSON_HISTORY_LISTS, exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(8)));
    if read_usize(this + f::PERSON_HISTORY_LISTS) == 0 {
        show_error_at(STAFF_RECORDS_SOURCE, 0x8f);
        return this;
    }
    write_usize(this + f::CLUB_HISTORY_LISTS, exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(8)));
    if read_usize(this + f::CLUB_HISTORY_LISTS) == 0 {
        show_error_at(STAFF_RECORDS_SOURCE, 0x97);
        return this;
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x86fce0usize); // Staff_Records::load
        if load(this, save) != 0 {
            return this;
        }
    }
    covered!("sx20_6 staff records built");
    let mut k = 0i32;
    while k < read_i32(G_NON_PLAYER_COUNT) + 16 {
        let r = records + k as usize * MANAGER_RECORD_SIZE;
        for o in [manager_record::OWNER, manager_record::SEASON_NATION, manager_record::NATIONAL_NATION, manager_record::CURRENT_CLUB,
                  manager_record::LONGEST_CLUB, manager_record::SHORTEST_CLUB] {
            write_i32(r + o, -1);
        }
        k += 1;
    }
    for i in 0..read_i32(G_STAFF_COUNT) {
        let person = staff_at(i as isize);
        let manager = |job: u8| job == 5 || job == 12; // sx20_6: the manager jobs
        if manager(person.job_for_club()) && !person.club_job().is_null() && read_usize(person.addr() + staff::NON_PLAYER) != 0 {
            let r = staff_records_record_of_non_player(this, read_usize(person.addr() + staff::NON_PLAYER));
            write_i32(r + manager_record::OWNER, read_i32(person.addr()));
            write_u8(r + manager_record::CLUB_JOBS, 1);
        }
        if manager(person.job_for_nation()) && !person.national_job().is_null() && read_usize(person.addr() + staff::NON_PLAYER) != 0 {
            let r = staff_records_record_of_non_player(this, read_usize(person.addr() + staff::NON_PLAYER));
            write_i32(r + manager_record::OWNER, read_i32(person.addr()));
            write_u8(r + manager_record::NATIONAL_JOBS, 1);
        }
    }
    core::ptr::write_bytes(read_usize(this + f::PERSON_HISTORY_LISTS) as *mut u8, 0xff, read_u32(G_STAFF_COUNT).wrapping_mul(8) as usize);
    core::ptr::write_bytes(read_usize(this + f::CLUB_HISTORY_LISTS) as *mut u8, 0xff, read_u32(G_CLUB_COUNT).wrapping_mul(8) as usize);
    this
}

/// Comparer for `staff_records_staff_records`.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_staff_records(this: usize, save: usize) -> usize {
    // simplification: opens the temporary lists and allocates the tables (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}
