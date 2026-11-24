// ---------- batch %%LEVA%%: loading the injuries and the human managers from a saved game (injury.cpp, human_manager.cpp; sx12b_4) ----------

/// The absolute value of the saved game's data version (`G_GAME_DATA_VERSION`), as the loaders compare it.
unsafe fn saved_data_version() -> i32 {
    read_i32(G_GAME_DATA_VERSION).wrapping_abs()
}

/// Reads the injury state from the `injury.dat` item of a saved game; returns 1, or 0 after an error.
///
/// `Injury::load(save)`: no container: the "Error" box (injury.cpp line 0x15f) and 0; `injury.dat` not opened
/// (`exe::zopen`): line 0x167 and 0. Then, in order, each failing with the box, the item closed and 0: the physical
/// records (`injury_fields::PHYSICAL_RECORDS`, `malloc` of `G_STAFF_COUNT` x 31 bytes, line 0x16f; read, 0x177), the
/// injury slot count (`injury_fields::INJURY_ENTRY_COUNT`, 0x17f) and the 0x30-byte injuries
/// (`injury_fields::INJURY_ENTRIES`, `malloc`, 0x188). Saves older than data version 0x3e024 stored 0x2c bytes per
/// injury (each read alone, 0x196): the days left (`injury_entry::DAYS_LEFT`) and the training days left
/// (`injury_entry::TRAINING_DAYS_LEFT`) become the days from today (`G_CURRENT_DATE`, `Date::days_since`) to the
/// injury date plus `DAYS_OUT` and plus `TRAINING_RETURN_DAYS` (`Date::add_days`); newer ones store all 0x30 (0x1a5).
/// The recovery news count (+0x14, 0x1ae) and, when not zero, `injury_fields::RECOVERY_NEWS_COUNT` = it and the
/// 22-byte entries (`injury_fields::RECOVERY_NEWS_QUEUE`, `malloc`, 0x1bd; read, 0x1c5); the outbreak count
/// (`injury_fields::OUTBREAK_CAPACITY`, 0x1ce) and, when not zero, `injury_fields::OUTBREAK_COUNT` = it and the 17-byte
/// outbreaks (`injury_fields::OUTBREAKS`, `malloc`, 0x1de; read, 0x1e7); the word count of the match knock list (0x1f0)
/// and, when not zero, a new list (`injury_fields::MATCH_HIT_LIST`: the old one destroyed, `operator new` of 14 bytes,
/// `LLIST::LLIST`) and its 6-byte items (each read, 0x207, and added at the tail). The item closed, 1.
/// Original: `Injury::load` (0x62ab00, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn injury_load(this: usize, save: usize) -> u32 {
    use injury_fields as f;
    const FILE: usize = 0x9e7f28; // "...\injury.cpp"
    if save == 0 {
        show_error_at(FILE, 0x15f);
        return 0;
    }
    let mut z = exe::zopen(save, 0x9e7f68usize, 0, 0x16, 4); // "injury.dat"
    if z == 0 {
        show_error_at(FILE, 0x167);
        return 0;
    }
    let failed = |z: &mut usize, row: u32| {
        show_error_at(FILE, row);
        exe::zclose(z);
        0
    };
    let records = exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(31));
    write_usize(this + f::PHYSICAL_RECORDS, records);
    if records == 0 {
        return failed(&mut z, 0x16f);
    }
    if exe::zread(records, 0x1f, read_u32(G_STAFF_COUNT), z) != read_u32(G_STAFF_COUNT) {
        return failed(&mut z, 0x177);
    }
    if exe::zread(this + f::INJURY_ENTRY_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x17f);
    }
    let entries = exe::crt_malloc(read_u32(this + f::INJURY_ENTRY_COUNT).wrapping_mul(0x30));
    write_usize(this + f::INJURY_ENTRIES, entries);
    if entries == 0 {
        return failed(&mut z, 0x188);
    }
    if saved_data_version() < 0x3e024 {
        covered!("sx12b_4 injuries of an old save");
        for k in 0..read_i32(this + f::INJURY_ENTRY_COUNT).max(0) as usize {
            let e = read_usize(this + f::INJURY_ENTRIES) + k * 0x30;
            if exe::zread(e, 0x2c, 1, z) != 1 {
                return failed(&mut z, 0x196);
            }
            for (days, left) in [(injury_entry::DAYS_OUT, injury_entry::DAYS_LEFT), (injury_entry::TRAINING_RETURN_DAYS, injury_entry::TRAINING_DAYS_LEFT)] {
                let mut end = [0u8; 8];
                let d = exe::date_add_days(e + injury_entry::DATE, end.as_mut_ptr() as usize, read_u16(e + days) as u32);
                let n = exe::date_days_since(d, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4));
                write_u16(read_usize(this + f::INJURY_ENTRIES) + k * 0x30 + left, n as u16);
            }
        }
    } else if exe::zread(entries, 0x30, read_u32(this + f::INJURY_ENTRY_COUNT), z) != read_u32(this + f::INJURY_ENTRY_COUNT) {
        return failed(&mut z, 0x1a5);
    }
    if exe::zread(this + 0x14, 4, 1, z) != 1 {
        return failed(&mut z, 0x1ae); // 0x14: the recovery news capacity [inference]
    }
    let news = read_u32(this + 0x14);
    if news != 0 {
        write_u32(this + f::RECOVERY_NEWS_COUNT, news); // sx12b_4: the recovery news count
        let queue = exe::crt_malloc(news.wrapping_mul(0x16));
        write_usize(this + f::RECOVERY_NEWS_QUEUE, queue);
        if queue == 0 {
            return failed(&mut z, 0x1bd);
        }
        if exe::zread(queue, 0x16, read_u32(this + 0x14), z) != read_u32(this + 0x14) {
            return failed(&mut z, 0x1c5);
        }
    }
    if exe::zread(this + f::OUTBREAK_CAPACITY, 4, 1, z) != 1 {
        return failed(&mut z, 0x1ce);
    }
    let outbreaks = read_u32(this + f::OUTBREAK_CAPACITY);
    if outbreaks != 0 {
        write_u32(this + f::OUTBREAK_COUNT, outbreaks);
        let array = exe::crt_malloc(outbreaks.wrapping_mul(0x11));
        write_usize(this + f::OUTBREAKS, array);
        if array == 0 {
            return failed(&mut z, 0x1de);
        }
        if exe::zread(array, 0x11, read_u32(this + f::OUTBREAK_CAPACITY), z) != read_u32(this + f::OUTBREAK_CAPACITY) {
            return failed(&mut z, 0x1e7);
        }
    }
    let mut knocks = 0i16;
    if exe::zread(&mut knocks as *mut i16 as usize, 2, 1, z) != 1 {
        return failed(&mut z, 0x1f0);
    }
    if knocks != 0 {
        let old = read_usize(this + f::MATCH_HIT_LIST);
        if old != 0 {
            let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
            destroy(old);
            exe::crt_operator_delete(old);
        }
        let list = exe::crt_operator_new(0xe);
        write_usize(this + f::MATCH_HIT_LIST, if list != 0 { exe::new_llist(list) } else { 0 });
        let mut k = 0i16;
        while k < knocks {
            let mut item = [0u8; 6];
            if exe::zread(item.as_mut_ptr() as usize, 6, 1, z) != 1 {
                return failed(&mut z, 0x207);
            }
            exe::llist_add_tail(read_usize(this + f::MATCH_HIT_LIST), item.as_ptr() as usize, 6);
            k += 1;
        }
    }
    covered!("sx12b_4 injuries loaded");
    exe::zclose(&mut z);
    1
}

/// The static buffer the human manager loader reads an old-layout tactic slot into (three of 0x1e75 bytes).
const OLD_TACTIC_BUFFER: usize = 0xb5e138;
const OLD_TACTIC_SLOT_SIZE: usize = 0x1e75;

/// Reads one tactic slot stored in the layout of saves older than data version 0x3e021 into `slot` (0x1ecd bytes).
///
/// The old slot (0x1e75 bytes) is read into the static buffer `OLD_TACTIC_BUFFER` + `buffer` x 0x1e75; its first 0x188b
/// bytes (the formation) are copied to the slot; then the eleven new 8-byte player entries at +0x188b each get the
/// dword at +0x588 + 4 x `which` of the slot (read again for each player) and 0, and the player instructions 8 and 2
/// are set (`Formation::set_player_instruction`); last, 0x642 bytes from the buffer's +0x188b go to the slot's +0x18e3.
/// Defects of the original: the old slot's tail is only 0x5ea bytes, so the copy takes 0x58 bytes of the next buffer
/// (past the third one: the globals from 0xb63c97) and writes 0x58 bytes past the slot (into the next one, or past the
/// fifth slot into the record's +0x9a01..); the dword it spreads over the players is picked by the slot's position in
/// its pass (`which`), not by the player.
unsafe fn read_old_tactic_slot(z: usize, buffer: usize, slot: usize, which: usize) {
    let old = OLD_TACTIC_BUFFER + buffer * OLD_TACTIC_SLOT_SIZE;
    exe::zread(old, OLD_TACTIC_SLOT_SIZE as u32, 1, z);
    core::ptr::copy(old as *const u8, slot as *mut u8, 0x188b);
    for j in 0..11usize {
        let legacy = read_u32(slot + 0x588 + 4 * which);
        write_u32(slot + 0x188b + 8 * j, legacy);
        write_u32(slot + 0x188f + 8 * j, 0);
        exe::formation_set_player_instruction(slot, j as i8, 8u64 << 32);
        exe::formation_set_player_instruction(slot, j as i8, 2u64 << 32);
    }
    core::ptr::copy((old + 0x188b) as *const u8, (slot + 0x18e3) as *mut u8, 0x642);
}

/// The fields of a human record read from the save in order (offset, size, count), after its first two tactic slots
/// and before the saved lineup: the holiday flag, the option bytes, the draft contract, the two offers, +0x9a61, the
/// holiday stamp, the active slot, the password and its typed entry, the computer control flag, the commentary speed,
/// the watched fixture, the search, training and print folders with their recent files, the squad view and the five
/// holiday options.
const HUMAN_RECORD_SAVED_FIELDS: [(usize, u32, u32); 20] = [
    (0x9a01, 4, 1), (0x9a05, 4, 1), (0x9a09, 0x50, 1), (0x9a59, 4, 1), (0x9a5d, 4, 1), (0x9a61, 4, 1), (0x9a65, 4, 1),
    (0x9a69, 1, 1), (0x9a6a, 1, 0x32), (0x9a9c, 1, 0x32), (0x9ace, 4, 1), (0x9ad2, 2, 1), (0x9ad4, 0x4f, 1),
    (0x9ba8, 1, 0x104), (0x9cac, 1, 0x410), (0xa0bc, 1, 0x104), (0xa1c0, 1, 0x410), (0xa5d0, 1, 0x104), (0xa6d4, 0xf, 1),
    (0x9b23, 1, 5),
];

/// Reads the human manager table from the `human_manager.dat` item of a saved game; returns 1, or 0 after an error.
///
/// `HUMAN_MANAGER::load(save)`: no container: the "Error" box (human_manager.cpp line 0x828) and 0; the item not
/// opened: line 0x830 and 0. The record count (`human_manager_fields::RECORD_COUNT`; failing: 0x837, the item closed,
/// 0) and the records (`operator new` of count x 0xa6ef + 4, the count first, `HUMAN_RECORD` constructors; none:
/// 0x840, closed, 0). For each record: the first two tactic slots (older than data version 0x3e021: each in the old
/// layout, `read_old_tactic_slot`), each with a team instruction word (+0x584) below 0x8000 given 0x20000
/// (`Formation::set_team_instruction`); the fields of `HUMAN_RECORD_SAVED_FIELDS` (unchecked); the 16 saved lineup
/// pairs (older than 0x3e005: 16 players, each with position -1); the nickname count (`NICKNAME_COUNT`, failing:
/// 0x892 and 0 with the item left open), the capacity (+0xa6e7) = it and, when not zero, the 30-byte nicknames
/// (`malloc`, 0x8a2; read, 0x8aa; both 0 with the item left open), otherwise no array. Then for each record: the two
/// offers cleared and, with a watched fixture's competition (+0x9ae8), its pointers rebuilt from the stored indices:
/// competition (`G_CLUB_COMP_TABLE` + [+0x9ad4] x 0x6b), the two clubs (+0x9af0, +0x9af4 from +0x9ae0, +0x9ae4,
/// `G_CLUB_TABLE`) and the stadium (+0x9aec from +0x9ad8, `G_STADIUM_TABLE`). Older than 0x3e001: the byte +0x59 of
/// the 16 human manager staff rows = 20. The dwords +8 (0x8d1) and `human_manager_fields::HOLIDAY_COUNT` (0x8d9), each
/// failing with the item closed and 0; for each record the last three tactic slots (old layout: `read_old_tactic_slot`
/// from the first buffer). The item closed, 1. Defects of the original: the three errors of the nicknames leave the
/// item open; the old [this] table is not freed.
/// Original: `HUMAN_MANAGER::load` (0x5fee50, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn human_manager_load(this: usize, save: usize) -> u32 {
    const FILE: usize = 0x9cf11c; // "...\human_manager.cpp"
    if save == 0 {
        show_error_at(FILE, 0x828);
        return 0;
    }
    let mut z = exe::zopen(save, 0x9cf164usize, 0, 0x16, 4); // "human_manager.dat"
    if z == 0 {
        show_error_at(FILE, 0x830);
        return 0;
    }
    let failed = |z: &mut usize, row: u32| {
        show_error_at(FILE, row);
        exe::zclose(z);
        0
    };
    if exe::zread(this + human_manager_fields::RECORD_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x837);
    }
    let count = read_u32(this + human_manager_fields::RECORD_COUNT);
    let block = exe::crt_operator_new(count.wrapping_mul(0xa6ef).wrapping_add(4));
    let records = if block != 0 {
        write_u32(block, count);
        exe::crt_vector_constructor_iterator(block + 4, 0xa6ef, count, 0x5fb690usize, 0x5fb700usize); // HUMAN_RECORD: 0xa6ef bytes, constructor, destructor
        block + 4
    } else {
        0
    };
    write_usize(this, records);
    if records == 0 {
        return failed(&mut z, 0x840);
    }
    let record_count = || read_i32(this + human_manager_fields::RECORD_COUNT).max(0) as usize;
    for i in 0..record_count() {
        let record = human_manager_record_at(this, i);
        if saved_data_version() < 0x3e021 {
            covered!("sx12b_4 human tactics of an old save");
            for k in 0..2 {
                read_old_tactic_slot(z, k, record + k * 0x1ecd, k);
            }
        } else {
            exe::zread(record, 0x1ecd, 2, z);
        }
        for k in 0..2 {
            if read_i32(record + k * 0x1ecd + 0x584) < 0x8000 { // sx12b_4: +0x584, the team instruction word
                exe::formation_set_team_instruction(record + k * 0x1ecd, 0x20000);
            }
        }
        for (field, size, n) in HUMAN_RECORD_SAVED_FIELDS {
            exe::zread(record + field, size, n, z);
        }
        if saved_data_version() < 0x3e005 {
            let mut players = [0u32; 0x10];
            exe::zread(players.as_mut_ptr() as usize, 4, 0x10, z);
            for (k, player) in players.iter().enumerate() {
                write_u32(record + human_manager_record::SAVED_LINEUP + 8 * k, *player);
                write_u32(record + human_manager_record::SAVED_LINEUP + 8 * k + 4, u32::MAX);
            }
        } else {
            exe::zread(record + human_manager_record::SAVED_LINEUP, 8, 0x10, z);
        }
        if exe::zread(record + human_manager_record::NICKNAME_COUNT, 4, 1, z) != 1 {
            show_error_at(FILE, 0x892); // the original leaves the item open here
            return 0;
        }
        write_u32(record + 0xa6e7, read_u32(record + human_manager_record::NICKNAME_COUNT)); // the nickname capacity
        let n = read_u32(record + 0xa6e7);
        if n != 0 {
            let list = exe::crt_malloc(n.wrapping_mul(0x1e));
            write_usize(record + human_manager_record::NICKNAMES, list);
            if list == 0 {
                show_error_at(FILE, 0x8a2);
                return 0;
            }
            if exe::zread(list, 0x1e, read_u32(record + human_manager_record::NICKNAME_COUNT), z) != read_u32(record + human_manager_record::NICKNAME_COUNT) {
                show_error_at(FILE, 0x8aa);
                return 0;
            }
        } else {
            write_usize(record + human_manager_record::NICKNAMES, 0);
        }
    }
    for i in 0..record_count() {
        let record = human_manager_record_at(this, i);
        write_u32(record + 0x9a59, 0); // the bid offer
        write_u32(record + 0x9a5d, 0); // the confirm offer
        if read_usize(record + 0x9ae8) != 0 {
            // the watched fixture's pointers from the indices the save keeps
            write_usize(record + 0x9ae8, read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32(record + human_manager_record::WATCHED_FIXTURE) as usize).wrapping_mul(0x6b)));
            write_usize(record + 0x9af0, club_at(read_i32(record + 0x9ae0) as isize).0);
            write_usize(record + 0x9af4, club_at(read_i32(record + 0x9ae4) as isize).0);
            write_usize(record + 0x9aec, read_usize(G_STADIUM_TABLE).wrapping_add((read_u32(record + 0x9ad8) as usize).wrapping_mul(stadium::SIZE)));
        }
    }
    if saved_data_version() < 0x3e001 {
        let staff = read_i32(G_STAFF_COUNT);
        for row in staff - 0x10..staff {
            write_u8(read_usize(G_STAFF_TABLE).wrapping_add((row as usize).wrapping_mul(staff::SIZE)) + 0x59, 0x14);
        }
    }
    if exe::zread(this + 8, 4, 1, z) != 1 {
        return failed(&mut z, 0x8d1);
    }
    if exe::zread(this + human_manager_fields::HOLIDAY_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x8d9);
    }
    for i in 0..record_count() {
        let record = human_manager_record_at(this, i);
        if saved_data_version() < 0x3e021 {
            for m in 0..3 {
                read_old_tactic_slot(z, m, record + 0x3d9a + m * 0x1ecd, m);
            }
        } else {
            exe::zread(record + 0x3d9a, 0x1ecd, 3, z); // sx12b_4: the last three tactic slots
        }
    }
    covered!("sx12b_4 human managers loaded");
    exe::zclose(&mut z);
    1
}

/// Comparer for `injury_load`: it allocates and reads the save item: it cannot run twice. Only the original runs; the
/// copy is proven by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_injury_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}

/// Comparer for `human_manager_load`: it allocates and reads the save item: it cannot run twice. Only the original
/// runs; the copy is proven by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_human_manager_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
}
