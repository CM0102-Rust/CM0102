// --- sx16_9: saving news.dat (0x787c30), notes.dat (0x794f00) and retire.dat (0x7ad7c0) ---

/// The news.cpp "Error" box at `row` that stops the game while saving (the fatal cleanup and exit(-1)).
unsafe fn news_save_fatal(row: u32) -> ! {
    show_error_box(0xa693ec, row); // news.cpp
    fatal_exit()
}

/// Writes the news store to "news.dat" in the save: the item count (`news_fields::SLOT_COUNT`), the 223-byte items (when
/// any), the inbox count, the 12-byte inboxes and the last news id; closes the item and answers 1. Without a save:
/// the "Error" box (line 0x2565) and 0. Not opening the item or a short write stops the game (fatal box, lines 0x256d,
/// 0x2574, 0x257f, 0x2588, 0x2590, 0x2598).
/// Original: `News::save_to_zip` (0x787c30, news.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn news_save_to_zip(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(0xa693ec, 0x2565);
        return 0;
    }
    let mut z = exe::zopen(save, 0xa69490usize, 1, 0x16, 4); // "news.dat"
    if z == 0 {
        news_save_fatal(0x256d);
    }
    covered!("sx16_9 save news.dat");
    let count = this + news_fields::SLOT_COUNT;
    if exe::zwrite(count, 4, 1, z) != 1 {
        news_save_fatal(0x2574);
    }
    let n = read_u32(count);
    if n != 0 && exe::zwrite(read_usize(this), 0xdf, n, z) != n {
        news_save_fatal(0x257f);
    }
    let inbox_count = this + 0xc;
    if exe::zwrite(inbox_count, 4, 1, z) != 1 {
        news_save_fatal(0x2588);
    }
    let n = read_u32(inbox_count);
    if exe::zwrite(read_usize(this + news_fields::HUMAN_INBOXES), 0xc, n, z) != n {
        news_save_fatal(0x2590);
    }
    if exe::zwrite(this + news_fields::LAST_ID, 4, 1, z) != 1 {
        news_save_fatal(0x2598);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `news_save_to_zip`: it writes to the save container, so only the original runs; the copy is proven by
/// the save proof.
pub(crate) unsafe extern "thiscall" fn compare_news_save_to_zip(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}

/// The notes.cpp "Error" box at `row` while saving, the item closed: the answer 0.
unsafe fn notes_save_error(row: u32, z: &mut usize) -> u8 {
    show_error_box(0xa69a18, row); // notes.cpp
    exe::zclose(z);
    0
}

/// Writes notes.dat to the saved game: for each of the 16 manager slots, the count of the first list (its signed
/// word) and its 0x114-byte notes, then the count of the second list and its 0x10c-byte records, each list walked from
/// its head (`LLIST::get_next`); closes the item and answers 1. Without a save (box line 0xb2) or not opening the item
/// (0xba): the "Error" box and 0. A failed write (counts 0xc4 and 0xdd, records 0xd4 and 0xed) or a list shorter than
/// its count (0xce, 0xe7): the box, the item closed and 0.
/// Original: `Notes::save_notes` (0x794f00, notes.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn notes_save_notes(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_box(0xa69a18, 0xb2);
        return 0;
    }
    let mut z = exe::zopen(save, 0xa69a40usize, 1, 0x16, 4); // "notes.dat"
    if z == 0 {
        show_error_box(0xa69a18, 0xba);
        return 0;
    }
    covered!("sx16_9 save notes.dat");
    for k in 0..0x10usize {
        for (slot, size, row_count, row_missing, row_record) in [(k * 8, 0x114u32, 0xc4, 0xce, 0xd4), (k * 8 + 4, 0x10c, 0xdd, 0xe7, 0xed)] {
            let list = read_usize(this + slot);
            let mut count = read_i16(list + llist::COUNT) as i32;
            if exe::zwrite(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
                return notes_save_error(row_count, &mut z);
            }
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            let mut i = 0i32;
            while i < read_i16(read_usize(this + slot) + llist::COUNT) as i32 {
                let record = exe::llist_get_next(read_usize(this + slot));
                if record == 0 {
                    return notes_save_error(row_missing, &mut z);
                }
                if exe::zwrite(record, size, 1, z) != 1 {
                    return notes_save_error(row_record, &mut z);
                }
                i += 1;
            }
        }
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `notes_save_notes`: only the original (it writes to the save container).
pub(crate) unsafe extern "thiscall" fn compare_notes_save_notes(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, save)
}

/// The four regen tables of retire.dat in their order: (table, count, record size, the player_regen.cpp lines of the
/// count and of the records when saving).
const RETIRE_DAT_SAVE: [(usize, usize, u32, u32, u32); 4] = [
    (player_regen_fields::RETIREMENTS, player_regen_fields::RETIREMENT_COUNT, 0x1a, 0x1a55, 0x1a5c),
    (player_regen_fields::REGEN_QUEUE, player_regen_fields::REGEN_QUEUE_COUNT, 0xc, 0x1a63, 0x1a6a),
    (player_regen_fields::SPARE_NON_PLAYER_RECORDS, player_regen_fields::SPARE_NON_PLAYER_COUNT, 0x10, 0x1a71, 0x1a78),
    (player_regen_fields::PLAYER_DATA_POOL, player_regen_fields::PLAYER_DATA_POOL_COUNT, 0x10, 0x1a7f, 0x1a86),
];

/// Writes the four regen tables (`RETIRE_DAT_SAVE`) to "retire.dat" in the save, each as its count and then its
/// records (when any); closes the item. Without a save (line 0x1a46) or not opening the item (0x1a4e): the
/// player_regen.cpp "Error" box and nothing more.
/// Defect of the original: a failed write shows the box and closes the item but goes on writing the rest into the
/// closed item (more boxes), and the item is closed again at the end.
/// Original: `PLAYER_REGEN::save_retire_dat` (0x7ad7c0, player_regen.cpp, thiscall, ret 4; no answer).
pub(crate) unsafe extern "thiscall" fn save_retire_dat(this: usize, save: usize) {
    const SOURCE: usize = 0xa6bf9c; // player_regen.cpp
    if save == 0 {
        show_error_box(SOURCE, 0x1a46);
        return;
    }
    let mut z = exe::zopen(save, 0xa701a4usize, 1, 0x16, 4); // "retire.dat"
    if z == 0 {
        show_error_box(SOURCE, 0x1a4e);
        return;
    }
    covered!("sx16_9 save retire.dat");
    for (table, count, size, row_count, row_records) in RETIRE_DAT_SAVE {
        if exe::zwrite(this + count, 4, 1, z) != 1 {
            show_error_box(SOURCE, row_count);
            exe::zclose(&mut z);
        }
        let n = read_u32(this + count);
        if n != 0 && exe::zwrite(read_usize(this + table), size, n, z) != n {
            show_error_box(SOURCE, row_records);
            exe::zclose(&mut z);
        }
    }
    exe::zclose(&mut z);
}

/// Comparer for `save_retire_dat`: only the original (it writes to the save container).
pub(crate) unsafe extern "thiscall" fn compare_save_retire_dat(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, save)
}
