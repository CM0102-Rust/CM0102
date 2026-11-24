// --- sx09c_2: fine.dat in the save: Fine::load (0x5a5180) and Fine::save (0x5a53d0), Fine.cpp ---

/// The save item "fine.dat".
const FINE_DAT: usize = 0x9ca1d4;
/// The source path of Fine.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\Fine.cpp").
const FINE_CPP_PATH: usize = 0x9ca1e0;

/// Reads `count` records of `size` bytes from the open save item into the list `list` (`LLIST::add_tail` each); false at
/// the first short read (the records read so far stay in the list).
unsafe fn fine_read_records(z: usize, list: usize, count: i32, size: u32) -> bool {
    let mut record = [0u8; 0x11];
    for _ in 0..count.max(0) {
        if exe::zread(record.as_mut_ptr() as usize, size, 1, z) != 1 {
            return false;
        }
        exe::llist_add_tail(list, record.as_ptr() as usize, size);
    }
    true
}

/// Reads the Fine manager from the save's "fine.dat": without a save or the item, 0. The fine count (dword), a new list
/// (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) in +0 with that many 14-byte fines; the
/// length of the per-club fine count table (dword), the table (`malloc`, not checked) in +8 and its bytes; a second new
/// list in +4 with the coach feedback count (dword) of 17-byte records. The item closed, 1. A short read closes the
/// item and answers 0 (what was built stays: the constructor destroys the lists and builds new ones).
/// Original: `Fine::load` (0x5a5180, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fine_load(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, FINE_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx09c_2 load fine.dat");
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        exe::zclose(&mut z);
        return 0;
    }
    write_usize(this, club_history_new_list());
    if !fine_read_records(z, read_usize(this), count, 0xe) {
        exe::zclose(&mut z);
        return 0;
    }
    let mut length = 0u32;
    if exe::zread(&mut length as *mut u32 as usize, 4, 1, z) != 1 {
        exe::zclose(&mut z);
        return 0;
    }
    let counts = exe::crt_malloc(length);
    write_usize(this + 8, counts); // sx09c_2: the per-club fine counts
    if exe::zread(counts, 1, length, z) != length {
        exe::zclose(&mut z);
        return 0;
    }
    write_usize(this + 4, club_history_new_list());
    let mut reports = 0i32;
    if exe::zread(&mut reports as *mut i32 as usize, 4, 1, z) != 1 || !fine_read_records(z, read_usize(this + 4), reports, 0x11) {
        exe::zclose(&mut z);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `fine_load`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_fine_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I0%%)(this, save)
}

/// Writes the records of `list` (from its head, `count` re-read each step) to the open save item, `size` bytes each;
/// the line of the "Error" box when a node is missing (`missing`) or a write is short (`short`).
unsafe fn fine_write_records(z: usize, list: Llist, size: u32, missing: u32, short: u32) -> Result<(), u32> {
    list.set_cursor(list.head());
    let mut k = 0i32;
    while k < list.count() as i32 {
        let record = exe::llist_get_next(list);
        if record == 0 {
            return Err(missing);
        }
        if exe::zwrite(record, size, 1, z) != 1 {
            return Err(short);
        }
        k += 1;
    }
    Ok(())
}

/// Writes the Fine manager to the save's "fine.dat": the fine count (the list's i16 as a dword), the 14-byte fines,
/// the club count (`G_CLUB_COUNT`, the global itself), the per-club fine count table (that many bytes), the coach
/// feedback count and its 17-byte records; the item closed, 1. Without a save (line 0x109) or the item (0x111): the
/// Fine.cpp "Error" box and 0. A short write or a missing node: the box (lines 0x119, 0x123, 0x129, 0x132, 0x13a,
/// 0x143, 0x14d, 0x153), the item closed and 0.
/// Original: `Fine::save` (0x5a53d0, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fine_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_box(FINE_CPP_PATH, 0x109);
        return 0;
    }
    let mut z = exe::zopen(save, FINE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(FINE_CPP_PATH, 0x111);
        return 0;
    }
    covered!("sx09c_2 save fine.dat");
    let write = |z: usize| -> Result<(), u32> {
        let fines = Llist(read_usize(this));
        let n = fines.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x119);
        }
        fine_write_records(z, fines, 0xe, 0x123, 0x129)?;
        if exe::zwrite(G_CLUB_COUNT, 4, 1, z) != 1 {
            return Err(0x132);
        }
        if exe::zwrite(read_usize(this + 8), 1, read_u32(G_CLUB_COUNT), z) != read_u32(G_CLUB_COUNT) { // sx09c_2: the fine counts
            return Err(0x13a);
        }
        let feedback = Llist(read_usize(this + 4));
        let n = feedback.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x143);
        }
        fine_write_records(z, feedback, 0x11, 0x14d, 0x153)
    };
    let r = write(z);
    if let Err(row) = r {
        show_error_box(FINE_CPP_PATH, row);
    }
    exe::zclose(&mut z);
    r.is_ok() as u8
}

/// Comparer for `fine_save`: it writes into the save container, so it cannot run twice; only the original runs. The
/// copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_fine_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, save)
}
