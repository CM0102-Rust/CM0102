// --- sx09c_3: dispute.dat in the save: DISPUTE::load (0x569910) and DISPUTE::save (0x569ae0), dispute.cpp ---

/// The save item "dispute.dat".
const DISPUTE_DAT: usize = 0x9c384c;
/// The source path of dispute.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\dispute.cpp").
const DISPUTE_CPP_PATH: usize = 0x9c3858;
/// The game data version from which saves carry dispute.dat (older ones have none).
const DISPUTE_DAT_VERSION: i32 = 0x3e046;

/// Reads the dispute object from the save's "dispute.dat": nothing (0) when the game data version (`G_GAME_DATA_VERSION`,
/// in absolute value) is below 0x3e046, without a save or without the item. The flag table length (dword; 0 closes
/// the item and answers 0), the table (`malloc` into +4; failing, the item closed and 0) and its bytes; the dispute
/// count (dword), a new list (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) in +0 and that
/// many 17-byte disputes (`LLIST::add_tail`). The item closed, 1. A short read closes the item and answers 0.
/// Original: `DISPUTE::load` (0x569910, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn dispute_load(this: usize, save: usize) -> u8 {
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < DISPUTE_DAT_VERSION || save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, DISPUTE_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx09c_3 load dispute.dat");
    let read = |z: usize| -> bool {
        let mut length = 0u32;
        if exe::zread(&mut length as *mut u32 as usize, 4, 1, z) != 1 || length == 0 {
            return false;
        }
        let flags = exe::crt_malloc(length);
        write_usize(this + 4, flags); // sx09c_3: the per-person dispute flags
        if flags == 0 || exe::zread(flags, 1, length, z) != length {
            return false;
        }
        let mut count = 0i32;
        if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
            return false;
        }
        let list = club_history_new_list();
        write_usize(this, list);
        let mut record = [0u8; 0x11];
        for _ in 0..count.max(0) {
            if exe::zread(record.as_mut_ptr() as usize, 0x11, 1, z) != 1 {
                return false;
            }
            exe::llist_add_tail(list, record.as_ptr() as usize, 0x11);
        }
        true
    };
    let ok = read(z);
    exe::zclose(&mut z);
    ok as u8
}

/// Comparer for `dispute_load`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_dispute_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I0%%)(this, save)
}

/// Writes the dispute object to the save's "dispute.dat": the staff count (`G_STAFF_COUNT`, the global itself), the flag
/// table (that many bytes), the dispute count (the list's i16 as a dword) and the 17-byte disputes (from the list's
/// head); the item closed, 1. Without a save (line 0xe9) or the item (0xf1): the dispute.cpp "Error" box and 0. A short
/// write or a missing node: the box (lines 0xf8, 0xff, 0x108, 0x111, 0x117), the item closed and 0.
/// Original: `DISPUTE::save` (0x569ae0, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn dispute_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_box(DISPUTE_CPP_PATH, 0xe9);
        return 0;
    }
    let mut z = exe::zopen(save, DISPUTE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(DISPUTE_CPP_PATH, 0xf1);
        return 0;
    }
    covered!("sx09c_3 save dispute.dat");
    let write = |z: usize| -> Result<(), u32> {
        if exe::zwrite(G_STAFF_COUNT, 4, 1, z) != 1 { // sx09c_3: the people count
            return Err(0xf8);
        }
        if exe::zwrite(read_usize(this + 4), 1, read_u32(G_STAFF_COUNT), z) != read_u32(G_STAFF_COUNT) { // sx09c_3: the flags
            return Err(0xff);
        }
        let list = Llist(read_usize(this));
        let n = list.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x108);
        }
        list.set_cursor(list.head());
        let mut k = 0i32;
        while k < list.count() as i32 {
            let record = exe::llist_get_next(list);
            if record == 0 {
                return Err(0x111);
            }
            if exe::zwrite(record, 0x11, 1, z) != 1 {
                return Err(0x117);
            }
            k += 1;
        }
        Ok(())
    };
    let r = write(z);
    if let Err(row) = r {
        show_error_box(DISPUTE_CPP_PATH, row);
    }
    exe::zclose(&mut z);
    r.is_ok() as u8
}

/// Comparer for `dispute_save`: it writes into the save container, so it cannot run twice; only the original runs.
/// The copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_dispute_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, save)
}
