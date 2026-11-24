// ---------- cx15_14: the squad manager's word from the saved game and the stadium bookings to Stadium.tmp ----------

/// `SQUAD_MANAGER::load(save)`: reads the squad manager's stored value from the `squad_manager.dat` item of the save
/// container (`exe::zopen`(save, name, 0, 0x16, 4), cdecl): 4 bytes (`exe::zread`) of which the low word goes to +0
/// (the registration year, written by `process_euro_registration`); then the item is closed (`exe::zclose`) and 1.
/// Error on opening or reading: the "Error" box (squad_manager.cpp 0x43, 0x4a) and 0, the item left open after a short
/// read (as `squad_manager_save`).
/// Original: `SQUAD_MANAGER::load` (0x859bc0, squad_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn squad_manager_load(this: usize, save: usize) -> u32 {
    let mut z = exe::zopen(save, 0xa82bc0, 0, 0x16, 4); // "squad_manager.dat"
    if z == 0 {
        show_error_at(0xa82b88, 0x43);
        return 0;
    }
    let mut v = 0u32;
    if exe::zread(&mut v as *mut u32 as usize, 4, 1, z) != 1 {
        show_error_at(0xa82b88, 0x4a);
        return 0;
    }
    covered!("cx15_14 squad manager loaded");
    write_u16(this, v as u16);
    exe::zclose(&mut z);
    1
}

/// `Stadium::save_to_temp_file()`: writes `CM3_TEMP\Stadium.tmp` (`open_temp_file_for_write`, "wb"; failing: the error
/// box of stadium.cpp line 0x179) with `exe::crt_fwrite` (cdecl): the booking capacity (`stadium_obj::BOOKING_CAPACITY`,
/// 4 bytes; short: the box at line 0x181) and count (`stadium_obj::BOOKING_COUNT`; 0x187) and, with a count, that many
/// 12-byte bookings (0x192); the venue capacity (`stadium_obj::VENUE_CAPACITY`; 0x19b), count
/// (`stadium_obj::VENUE_COUNT`; 0x1a1) and order (`stadium_obj::VENUE_SORT`, 1 byte; 0x1a7); with venues, each venue's
/// stadium (`stadium_venue::STADIUM`) written over by its id (null: −1), the 16-byte venues (0x1be) and the ids turned
/// back (`stadium_by_id`; −1: null); the stashed venue id ([`G_STADIUM_STASHED_VENUE_ID`], 4 bytes; 0x1d1); the file
/// closed. After a box the file stays open, and a short venue write leaves the venues with ids in place of the
/// stadiums (a defect of the original). The reverse of `stadium_load_from_temp_file`.
/// Original: `Stadium::save_to_temp_file` (0x85bea0, stadium.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn stadium_save_to_temp_file(this: usize) {
    const STADIUM_CPP: usize = 0xa835b0; // "...\comp\stadium.cpp"
    let f = open_temp_file_for_write(0xa835e0); // "Stadium.tmp"
    if f == 0 {
        show_error_at(STADIUM_CPP, 0x179);
        return;
    }
    let one = |at: usize, size: u32, row: u32| -> bool {
        if exe::crt_fwrite(at, size, 1, f) != 1 {
            show_error_at(STADIUM_CPP, row);
            return false;
        }
        true
    };
    if !one(this + stadium_obj::BOOKING_CAPACITY, 4, 0x181) || !one(this + stadium_obj::BOOKING_COUNT, 4, 0x187) {
        return;
    }
    let bookings = read_u32(this + stadium_obj::BOOKING_COUNT);
    if bookings != 0 && exe::crt_fwrite(read_usize(this + stadium_obj::BOOKINGS), 0xc, bookings, f) != read_u32(this + stadium_obj::BOOKING_COUNT) {
        show_error_at(STADIUM_CPP, 0x192);
        return;
    }
    if !one(this + stadium_obj::VENUE_CAPACITY, 4, 0x19b) || !one(this + stadium_obj::VENUE_COUNT, 4, 0x1a1) || !one(this + stadium_obj::VENUE_SORT, 1, 0x1a7) {
        return;
    }
    if read_u32(this + stadium_obj::VENUE_COUNT) != 0 {
        covered!("cx15_14 stadium venues saved");
        let venue = |k: i32| read_usize(this + stadium_obj::VENUES) + 0x10 * k as usize + stadium_venue::STADIUM;
        let mut k = 0i32;
        while k < read_i32(this + stadium_obj::VENUE_COUNT) {
            let s = read_usize(venue(k));
            write_u32(venue(k), if s != 0 { read_u32(s) } else { u32::MAX });
            k += 1;
        }
        if exe::crt_fwrite(read_usize(this + stadium_obj::VENUES), 0x10, read_u32(this + stadium_obj::VENUE_COUNT), f) != read_u32(this + stadium_obj::VENUE_COUNT) {
            show_error_at(STADIUM_CPP, 0x1be);
            return;
        }
        let mut k = 0i32;
        while k < read_i32(this + stadium_obj::VENUE_COUNT) {
            let id = read_i32(venue(k));
            write_usize(venue(k), if id != -1 { stadium_by_id(id as usize) } else { 0 });
            k += 1;
        }
    }
    if !one(G_STADIUM_STASHED_VENUE_ID, 4, 0x1d1) {
        return;
    }
    exe::crt_fclose(f);
}

/// Comparer for `squad_manager_load`: it reads the save container (it cannot run twice), so only the original runs;
/// then the plan: 1.
pub(crate) unsafe extern "thiscall" fn compare_squad_manager_load(this: usize, save: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save);
    SCOREBOARD.record(r == 1, || format!("squad_manager_load({:#x}) returned {}", this, r));
    r
}

/// Comparer for `stadium_save_to_temp_file`: it writes the temporary file (it cannot run twice), so only the original
/// runs; then the plan: the venues' stadiums are back as they were.
pub(crate) unsafe extern "thiscall" fn compare_stadium_save_to_temp_file(this: usize) {
    let venues = read_usize(this + stadium_obj::VENUES);
    let n = if venues == 0 { 0 } else { read_i32(this + stadium_obj::VENUE_COUNT).max(0) as usize };
    let before: Vec<usize> = (0..n).map(|k| read_usize(venues + 0x10 * k + stadium_venue::STADIUM)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let after: Vec<usize> = (0..n).map(|k| read_usize(venues + 0x10 * k + stadium_venue::STADIUM)).collect();
    SCOREBOARD.record(before == after, || format!("stadium_save_to_temp_file({:#x}): the venues changed", this));
}
