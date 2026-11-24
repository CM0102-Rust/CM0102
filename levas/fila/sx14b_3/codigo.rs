// ---------- batch %%LEVA%%: the saving of man_conf.dat (manager_manager.cpp) ----------

/// Writes one list to man_conf.dat: the count (`LLIST` +0, a signed word, as a dword) and each item of `size` bytes
/// from the head. Answers the manager_manager.cpp box row of the failure: `rows` = (count not written, list shorter
/// than its count, item not written).
unsafe fn man_conf_write_list(z: usize, list: usize, size: u32, rows: (u32, u32, u32)) -> Result<(), u32> {
    let mut count = read_i16(list) as i32;
    if exe::zwrite(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        return Err(rows.0);
    }
    write_u32(list + 0xa, read_u32(list + 2)); // the cursor at the head
    let mut k = 0i32;
    while k < read_i16(list) as i32 {
        let item = exe::llist_get_next(list);
        if item == 0 {
            return Err(rows.1);
        }
        if exe::zwrite(item, size, 1, z) != 1 {
            return Err(rows.2);
        }
        k += 1;
    }
    Ok(())
}

/// The body of `save_man_conf` with the item open: the error row on a failure.
unsafe fn man_conf_write(this: usize, z: usize) -> Result<(), u32> {
    use manager_manager_fields as f;
    if exe::zwrite(G_CLUB_COUNT, 4, 1, z) != 1 {
        return Err(0x228);
    }
    let clubs = read_u32(G_CLUB_COUNT);
    if clubs == 0 {
        return Err(0x22e);
    }
    if exe::zwrite(read_usize(this + f::BOARD_RECORDS), 0x49, clubs, z) != read_u32(G_CLUB_COUNT) {
        return Err(0x234);
    }
    man_conf_write_list(z, read_usize(this + f::VACANCY_LIST), 0x26, (0x23d, 0x247, 0x24d))?;
    man_conf_write_list(z, read_usize(this + f::DEPARTURE_LIST), 6, (0x256, 0x260, 0x266))?;
    man_conf_write_list(z, read_usize(this + 0xc), 0xe, (0x26f, 0x279, 0x27f))?;
    man_conf_write_list(z, read_usize(this + f::ULTIMATUM_LIST), 4, (0x288, 0x292, 0x298))?;
    man_conf_write_list(z, read_usize(this + f::BAN_APPEAL_LIST), 0x19, (0x2a2, 0x2ac, 0x2b2))
}

/// Writes the board object to the save's "man_conf.dat" (`exe::zopen` for writing): the club count, the board
/// records (`G_CLUB_COUNT` × 0x49 bytes) and the five lists (vacancies 0x26-byte items, departures 6, +0xc 0xe,
/// ultimatums 4, ban appeals 0x19), each with its count. Without a save (row 0x21a) or the item (0x222): the
/// manager_manager.cpp "Error" box; any later failure (club count not written 0x228, no clubs 0x22e, records 0x234,
/// then the rows of each list) shows the box and closes the item; done, the item is closed.
/// Original: `MANAGER_MANAGER::save_man_conf` (0x68e920, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_man_conf(this: usize, save: usize) {
    const MANAGER_MANAGER_CPP: usize = 0x9f18a4;
    if save == 0 {
        return show_error_box(MANAGER_MANAGER_CPP, 0x21a);
    }
    let mut z = exe::zopen(save, 0x9f18d8, 1, 0x16, 4); // "man_conf.dat"
    if z == 0 {
        return show_error_box(MANAGER_MANAGER_CPP, 0x222);
    }
    covered!("sx14b_3 save man_conf.dat");
    if let Err(row) = man_conf_write(this, z) {
        show_error_box(MANAGER_MANAGER_CPP, row);
    }
    exe::zclose(&mut z);
}

/// Comparer for `save_man_conf`: it writes the save item, so it cannot run twice; only the original runs. The copy is
/// proven by the save proof (the .qst written with and without the routine, compared item by item).
pub(crate) unsafe extern "thiscall" fn compare_save_man_conf(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, save)
}
