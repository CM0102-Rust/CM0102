// --- sx09c_4: discipline.dat in the save: Discipline::load (0x555860) and Discipline::save (0x555dd0), discipline.cpp ---

/// The save item "discipline.dat".
const DISCIPLINE_DAT: usize = 0x9b8f58;
/// The source path of discipline.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\discipline.cpp").
const DISCIPLINE_CPP_PATH: usize = 0x9b8f18;
/// `RulingBody::save` (0x7e7b80, ruling_body.cpp; thiscall(body, &z), ret 4): writes one rule set's entries and pending
/// bans to the open item.
const FN_RULING_BODY_SAVE: usize = 0x7e7b80;

/// Reads one counted table of the discipline object from the open item: the i16 count into `count` (box `row_count`
/// when short); with a count, the capacity field takes it, the array (`malloc` of count × `size`) goes into `table`
/// and its records are read (boxes `row_alloc`, `row_read`). Returns the line of the failure.
unsafe fn discipline_read_table(z: usize, this: usize, count: usize, capacity: usize, table: usize, size: u32,
                                rows: (u32, u32, u32)) -> Result<(), u32> {
    if exe::zread(this + count, 2, 1, z) != 1 {
        return Err(rows.0);
    }
    let n = read_i16(this + count);
    if n == 0 {
        return Ok(());
    }
    write_i16(this + capacity, n);
    let array = exe::crt_malloc((n as i32 as u32).wrapping_mul(size));
    write_usize(this + table, array);
    if array == 0 {
        return Err(rows.1);
    }
    if exe::zread(array, size, n as i32 as u32, z) != read_i16(this + count) as i32 as u32 {
        return Err(rows.2);
    }
    Ok(())
}

/// Reads the discipline object from the save's "discipline.dat". Without a save (line 0xda) or the item (0xe2): the
/// discipline.cpp "Error" box and 0. Then: the per-person records (`G_STAFF_COUNT` × 29 bytes, `malloc` into
/// `discipline_fields::PERSON_RECORDS`; lines 0xea, 0xf2); the rule sets (`discipline_create_rule_sets` with the address
/// of the open item: each rule set reads its entries and pending bans); the ban array (`BAN_COUNT`, `BAN_CAPACITY`,
/// `BANS`, 20 bytes each; lines 0xfd, 0x10a, 0x112) and the nation index (`NATION_INDEX_COUNT`, `NATION_INDEX_CAPACITY`,
/// `NATION_INDEX`, 8 bytes each; lines 0x11b, 0x128, 0x130), each allocated only with a count; those failures close the
/// item and answer 0. Then the held-ban count (i16; a short read shows the box of line 0x139 and counts 0) and that many 10-byte
/// held bans into the `HELD_BANS` list (`LLIST::add_tail`; a short read shows the box of line 0x143 and goes on with the
/// next one). The item closed, 1.
/// Original: `Discipline::load` (0x555860, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_load(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0xda);
        return 0;
    }
    let mut z = exe::zopen(save, DISCIPLINE_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0xe2);
        return 0;
    }
    covered!("sx09c_4 load discipline.dat");
    let tables = |z: &mut usize| -> Result<(), u32> {
        let records = exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(0x1d));
        write_usize(this + discipline_fields::PERSON_RECORDS, records);
        if records == 0 {
            return Err(0xea);
        }
        if exe::zread(records, 0x1d, read_u32(G_STAFF_COUNT), *z) != read_u32(G_STAFF_COUNT) {
            return Err(0xf2);
        }
        discipline_create_rule_sets(this, z as *mut usize as u32); // sx09c_4: the rule sets read their entries
        discipline_read_table(*z, this, discipline_fields::BAN_COUNT, discipline_fields::BAN_CAPACITY,
                              discipline_fields::BANS, 0x14, (0xfd, 0x10a, 0x112))?;
        discipline_read_table(*z, this, discipline_fields::NATION_INDEX_COUNT, discipline_fields::NATION_INDEX_CAPACITY,
                              discipline_fields::NATION_INDEX, 8, (0x11b, 0x128, 0x130))
    };
    if let Err(row) = tables(&mut z) {
        show_error_box(DISCIPLINE_CPP_PATH, row);
        exe::zclose(&mut z);
        return 0;
    }
    let mut held = 0i16;
    if exe::zread(&mut held as *mut i16 as usize, 2, 1, z) != 1 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x139);
        held = 0;
    }
    let mut record = [0u8; 0xa];
    let mut k = 0i16;
    while k < held {
        if exe::zread(record.as_mut_ptr() as usize, 0xa, 1, z) != 1 {
            show_error_box(DISCIPLINE_CPP_PATH, 0x143);
        } else {
            exe::llist_add_tail(this + discipline_fields::HELD_BANS, record.as_ptr() as usize, 0xa);
        }
        k += 1;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `discipline_load`: it reads the save item and allocates, so it cannot run twice; only the original
/// runs. The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_discipline_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}

/// Writes one counted table of the discipline object: the i16 count (line `rows.0` when short) and, with a count,
/// its records of `size` bytes (line `rows.1`).
unsafe fn discipline_write_table(z: usize, this: usize, count: usize, table: usize, size: u32, rows: (u32, u32)) -> Result<(), u32> {
    if exe::zwrite(this + count, 2, 1, z) != 1 {
        return Err(rows.0);
    }
    let n = read_i16(this + count);
    if n != 0 && exe::zwrite(read_usize(this + table), size, n as i32 as u32, z) != read_i16(this + count) as i32 as u32 {
        return Err(rows.1);
    }
    Ok(())
}

/// Writes the discipline object to the save's "discipline.dat": the per-person records (`G_STAFF_COUNT` × 29 bytes), each
/// existing rule set of the 43 (`RulingBody::save` with the address of the open item), the ban array (i16 count and
/// 20-byte bans), the nation index (i16 count and 8-byte items), the held-ban count (the list's i16) and the 10-byte
/// held bans from the list's head until it runs out. Without a save (line 0x163) or the item (0x16b): the
/// discipline.cpp "Error" box. A short write of the records or tables (lines 0x172, 0x185, 0x18f, 0x198, 0x1a2): the box
/// and the item closed. A short held-ban count (0x1ac) only shows the box; a short held ban (0x1b7) shows it and stops
/// the list. The item closed. The original returns nothing (eax is left over; the caller ignores it).
/// Original: `Discipline::save` (0x555dd0, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_save(this: usize, save: usize) {
    if save == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x163);
        return;
    }
    let mut z = exe::zopen(save, DISCIPLINE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x16b);
        return;
    }
    covered!("sx09c_4 save discipline.dat");
    let tables = |z: &mut usize| -> Result<(), u32> {
        let n = read_u32(G_STAFF_COUNT);
        if exe::zwrite(read_usize(this + discipline_fields::PERSON_RECORDS), 0x1d, n, *z) != read_u32(G_STAFF_COUNT) {
            return Err(0x172);
        }
        let save_body: unsafe extern "thiscall" fn(usize, *mut usize) = core::mem::transmute(FN_RULING_BODY_SAVE);
        for k in 0..0x2busize {
            let body = read_usize(read_usize(this + discipline_fields::RULING_BODIES) + 4 * k);
            if body != 0 {
                save_body(body, z as *mut usize); // sx09c_4: each rule set
            }
        }
        discipline_write_table(*z, this, discipline_fields::BAN_COUNT, discipline_fields::BANS, 0x14, (0x185, 0x18f))?;
        discipline_write_table(*z, this, discipline_fields::NATION_INDEX_COUNT, discipline_fields::NATION_INDEX, 8, (0x198, 0x1a2))
    };
    if let Err(row) = tables(&mut z) {
        show_error_box(DISCIPLINE_CPP_PATH, row);
        exe::zclose(&mut z);
        return;
    }
    let held_list = Llist(this + discipline_fields::HELD_BANS);
    let held = held_list.count() as i32; // the original stores the i16 in a dword and writes 2 bytes
    if exe::zwrite(&held as *const i32 as usize, 2, 1, z) != 1 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x1ac);
    }
    held_list.set_cursor(held_list.head());
    let mut record = exe::llist_get_next(held_list);
    while record != 0 {
        if exe::zwrite(record, 0xa, 1, z) != 1 {
            show_error_box(DISCIPLINE_CPP_PATH, 0x1b7);
            break;
        }
        record = exe::llist_get_next(held_list);
    }
    exe::zclose(&mut z);
}

/// Comparer for `discipline_save`: it writes into the save container, so it cannot run twice; only the original runs.
/// The copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_discipline_save(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, save)
}
