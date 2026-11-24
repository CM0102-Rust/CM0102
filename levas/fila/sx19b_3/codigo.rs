// ---------- sx19b_3: a ruling body in the saved game: loading and saving (ruling_body/ruling_body.cpp) ----------

/// The pending ban node of a ruling body's list (0x70 bytes): its fixture copy at +0x20.
const PENDING_BAN_NODE_SIZE: u32 = 0x70;
const PENDING_BAN_FIXTURE: usize = 0x20;

/// The ruling body's virtual slot `slot` (thiscall(body, save item), ret 4): +0x18 `load_extra_data`, +0x1c
/// `save_extra_data`.
unsafe fn ruling_body_extra_data(this: usize, slot: usize, file: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + slot));
    f(this, file)
}

/// Reads the discipline entry array and the pending ban list from a saved game, then the body's own extra data.
///
/// `RulingBody::load(file)` (file = the save item, its stream at +0), from the base constructor with a save
/// (`construct_ruling_body`): the entry count (`ruling_body::ENTRY_COUNT`, 4 bytes); with entries, the capacity
/// (`ENTRY_CAPACITY`) = the count and a new array (`exe::crt_malloc`, 14 bytes each; out of memory: the box at line
/// 0x6c) in `ENTRIES`, read whole; then the number of pending bans (16 bits) and each 0x70-byte node, whose fixture copy
/// (+0x20) gets its competition, stadium and both clubs back as pointers from their ids (`G_CLUB_COMP_TABLE`,
/// `G_STADIUM_TABLE`, `G_CLUB_TABLE`) before it goes to the end of the body's list (`exe::llist_add_tail`, copied).
/// Last, the body's `load_extra_data` (vtable +0x18): returns 1 when it answers. A short read: the ruling_body.cpp
/// "Error" box with the field's line (0x5f, 0x74, 0x7e, 0x8b) and 0.
/// Original: `RulingBody::load` (0x7e78a0, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_load(this: usize, file: usize) -> u32 {
    let stream = read_usize(file);
    if exe::zread(this + ruling_body::ENTRY_COUNT, 4, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0x5f);
        return 0;
    }
    let count = read_u32(this + ruling_body::ENTRY_COUNT);
    if count != 0 {
        write_u32(this + ruling_body::ENTRY_CAPACITY, count);
        let entries = exe::crt_malloc(count.wrapping_mul(14));
        write_usize(this + ruling_body::ENTRIES, entries);
        if entries == 0 {
            show_error_at(RULING_BODY_CPP, 0x6c);
            return 0;
        }
        if exe::zread(entries, 14, count, stream) != read_u32(this + ruling_body::ENTRY_COUNT) {
            show_error_at(RULING_BODY_CPP, 0x74);
            return 0;
        }
    }
    // simplification: the original constructs the node's two dates (+4, +0x14) first; the read overwrites them
    let mut node = [0u8; PENDING_BAN_NODE_SIZE as usize];
    let mut pending = 0i16;
    if exe::zread(&mut pending as *mut i16 as usize, 2, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0x7e);
        return 0;
    }
    let mut i = 0i16;
    while i < pending {
        if exe::zread(node.as_mut_ptr() as usize, PENDING_BAN_NODE_SIZE, 1, stream) != 1 {
            show_error_at(RULING_BODY_CPP, 0x8b);
            return 0;
        }
        covered!("sx19b_3 pending ban loaded");
        let f = Fixture(node.as_mut_ptr() as usize + PENDING_BAN_FIXTURE);
        // the club competition records are 0x6b bytes
        f.set_competition(Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((f.comp_id() as usize).wrapping_mul(0x6b))));
        f.set_stadium(read_usize(G_STADIUM_TABLE).wrapping_add(f.stadium_id().wrapping_mul(stadium::SIZE)));
        f.set_home_club(Club(read_usize(G_CLUB_TABLE).wrapping_add(f.home_club_id().wrapping_mul(club::SIZE)))); // sx19b_3: the home club relinked
        f.set_away_club(Club(read_usize(G_CLUB_TABLE).wrapping_add(f.away_club_id().wrapping_mul(club::SIZE))));
        exe::llist_add_tail(read_usize(this + RULING_BODY_LIST), node.as_ptr() as usize, PENDING_BAN_NODE_SIZE);
        i += 1;
    }
    (ruling_body_extra_data(this, 0x18, file) != 0) as u32
}

/// Comparer for `ruling_body_load`: it reads the save stream, so only the original runs; checked afterwards: the
/// capacity equals the count when there are entries.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_load(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file);
    let (n, c) = (read_u32(this + ruling_body::ENTRY_COUNT), read_u32(this + ruling_body::ENTRY_CAPACITY));
    SCOREBOARD.record(r == 0 || n == 0 || n == c, || format!("ruling_body_load({:#x}): {} entries, capacity {}", this, n, c));
    r
}

/// Writes the discipline entry array and the pending ban list to a saved game, then the body's own extra data.
///
/// `RulingBody::save(file)`, from `Discipline::save` (each of the 43 bodies): the entry count
/// (`ruling_body::ENTRY_COUNT`, 4 bytes) and, with entries, the array (`ENTRIES`, 14 bytes each); the number of
/// pending bans (the list's 16-bit count, 0 without a list) and, with any, each node of the list (0x70 bytes, from the
/// head). Last, the body's `save_extra_data` (vtable +0x1c), whose answer is returned. A short write: the
/// ruling_body.cpp "Error" box with the field's line (0xae, 0xb8, 0xc7, 0xd5) and nothing more (the caller ignores
/// the answer; the copy returns 0).
/// Original: `RulingBody::save` (0x7e7b80, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_save(this: usize, file: usize) -> u32 {
    let stream = read_usize(file);
    if exe::zwrite(this + ruling_body::ENTRY_COUNT, 4, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0xae);
        return 0;
    }
    let count = read_u32(this + ruling_body::ENTRY_COUNT);
    if count != 0 && exe::zwrite(read_usize(this + ruling_body::ENTRIES), 14, count, stream) != read_u32(this + ruling_body::ENTRY_COUNT) {
        show_error_at(RULING_BODY_CPP, 0xb8);
        return 0;
    }
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    let pending: u32 = if list.is_null() { 0 } else { list.count() as u16 as u32 }; // sx19b_3: the pending ban count
    if exe::zwrite(&pending as *const u32 as usize, 2, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0xc7);
        return 0;
    }
    if pending as u16 != 0 {
        list.set_cursor(list.head());
        let mut node = exe::llist_get_next(list);
        while node != 0 {
            covered!("sx19b_3 pending ban saved");
            if exe::zwrite(node, PENDING_BAN_NODE_SIZE, 1, stream) != 1 {
                show_error_at(RULING_BODY_CPP, 0xd5);
                return 0;
            }
            node = exe::llist_get_next(list);
        }
    }
    ruling_body_extra_data(this, 0x1c, file)
}

/// Comparer for `ruling_body_save`: it writes the save stream, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_save(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}
