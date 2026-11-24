// ---------- sx05b_9: saving the AWOL state (awol.cpp) ----------

/// Saves the AWOL state to the save's awol.dat as `awol_load` reads it: without a save, the "Error" box (line 0x132)
/// and 0; the file (`exe::zopen`(save, "awol.dat", 1, 0x16, 4); none, the box of line 0x13a and 0, nothing to close);
/// the staff count (`G_STAFF_COUNT`, line 0x141), the flag table (that many bytes, line 0x148) and each of the three
/// lists: its count (the list's signed word, as a dword; lines 0x151, 0x16a, 0x183), then its records walked from the
/// head (`llist::CURSOR` = `llist::HEAD`, `exe::llist_get_next`: a missing record, lines 0x15a, 0x173, 0x18c; a short
/// write, 0x160, 0x179, 0x192) at 12, 13 and 17 bytes. Each failure after the file opens shows the box, closes it and
/// gives 0; otherwise it closes and gives 1.
/// Original: `AWOL::save` (0x418980, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_at(AWOL_CPP, 0x132);
        return 0;
    }
    let mut z = exe::zopen(save, AWOL_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_at(AWOL_CPP, 0x13a);
        return 0;
    }
    let failed = awol_save_lists(this, z);
    if let Some(line) = failed {
        show_error_at(AWOL_CPP, line);
    }
    exe::zclose(&mut z);
    failed.is_none() as u8
}

/// The body of `awol_save` once the file is open: the line of the first failure, if any.
unsafe fn awol_save_lists(this: usize, z: usize) -> Option<u32> {
    if exe::zwrite(G_STAFF_COUNT, 4, 1, z) != 1 {
        return Some(0x141);
    }
    let staff = read_u32(G_STAFF_COUNT);
    if exe::zwrite(read_usize(this + awol_fields::PERSON_FLAGS), 1, staff, z) != staff {
        return Some(0x148);
    }
    const LINES: [(u32, u32, u32); 3] = [(0x151, 0x15a, 0x160), (0x16a, 0x173, 0x179), (0x183, 0x18c, 0x192)];
    for (&(off, size), &(count_line, missing_line, write_line)) in AWOL_LISTS.iter().zip(LINES.iter()) {
        let list = read_usize(this + off);
        let count = read_i16(list + llist::COUNT) as i32;
        if exe::zwrite(&count as *const i32 as usize, 4, 1, z) != 1 {
            return Some(count_line);
        }
        write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
        let mut k = 0;
        while k < read_i16(read_usize(this + off) + llist::COUNT) as i32 {
            let record = exe::llist_get_next(read_usize(this + off));
            if record == 0 {
                return Some(missing_line);
            }
            if exe::zwrite(record, size, 1, z) != 1 {
                return Some(write_line);
            }
            k += 1;
        }
    }
    covered!("sx05b_9 awol saved");
    None
}

/// Comparer for `awol_save`: the save file is written once; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_awol_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I0%%)(this, save)
}
