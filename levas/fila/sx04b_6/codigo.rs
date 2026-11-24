// ---------- sx04b_6: the month award in the saved game and the month ratings' load (month_award.cpp, month_ratings.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\month_award.cpp" and "...\month_ratings.cpp" (the error boxes).
pub(crate) const MONTH_AWARD_CPP: usize = 0xa5abe4;
pub(crate) const MONTH_RATINGS_CPP: usize = 0xa5b234;
/// The boxes of a failed load: "Load Game Error" with "The save game is corrupted..." (`LOAD_ERROR`), or "Memory
/// Allocation Error" with its body (`MEMORY_ERROR`); title and body, both translated.
pub(crate) const LOAD_ERROR: (usize, usize) = (0x9af3e0, 0x9af334);
pub(crate) const MEMORY_ERROR: (usize, usize) = (0x9a8108, 0x9a7ff8);
/// The month ratings table (`MONTH_RATINGS`): the 12-byte entries (+0), their count (+4) and capacity (+8).
pub(crate) const MONTH_RATINGS_ENTRIES: usize = 0x0;
pub(crate) const MONTH_RATINGS_COUNT: usize = 0x4;
pub(crate) const MONTH_RATINGS_CAPACITY: usize = 0x8;
pub(crate) const MONTH_RATING_ENTRY_SIZE: u32 = 0xc;

/// The box of a failed load from a saved game: "v<version> <file>.<ext> <line> (<message>)" at 0xae24d0, the `box`'s
/// title (0xae28bc) and body with that text (0xae26c4), both translated, and the message box.
pub(crate) unsafe fn award_load_error_box(cpp: usize, line: u32, message: usize, (title, body): (usize, usize)) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(cpp, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line, message);
    comp_crt_format()(0xae24d0, 0x9a8120, v, name.as_ptr(), ext.as_ptr(), line, message); // "v%s %s.%s %d (%s)"
    comp_lang_format()(0xae28bc, title);
    comp_lang_format()(0xae26c4, body, 0xae24d0);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// Reads a month award from the saved game: its twelve month slots (`MONTH_AWARD_SLOT_SIZE` bytes each, from
/// `month_award_fields::SLOT_WINNER_ID`) and its slot count (`MONTH_AWARD_SLOT_COUNT`, a byte), each with
/// `exe::zread`. Returns 1, or 0 after the Load Game Error box of the first short read (month_award.cpp 110 "cannot
/// read months", 117 "cannot read no of months").
/// Original: `MONTH_AWARD::load_from_disk` (0x766f50, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_load_from_disk(this: usize, file: usize) -> u32 {
    if exe::zread(this + month_award_fields::SLOT_WINNER_ID, MONTH_AWARD_SLOT_SIZE as u32, 12, file) != 12 {
        award_load_error_box(MONTH_AWARD_CPP, 0x6e, 0xa5abb0, LOAD_ERROR);
        return 0;
    }
    if exe::zread(this + MONTH_AWARD_SLOT_COUNT, 1, 1, file) != 1 { // sx04b_6: the slot count
        award_load_error_box(MONTH_AWARD_CPP, 0x75, 0xa5ab78, LOAD_ERROR);
        return 0;
    }
    covered!("sx04b_6 month award loaded");
    1
}

/// Comparer for `month_award_load_from_disk`: the load reads the file once; only the original runs, and a loaded
/// award must keep at most twelve slots.
pub(crate) unsafe extern "thiscall" fn compare_month_award_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file);
    if r == 1 {
        let n = read_i8(this + MONTH_AWARD_SLOT_COUNT);
        SCOREBOARD.record((0..=12).contains(&n), || format!("month_award_load_from_disk({:#x}): {} slots", this, n));
    }
    r
}

/// Writes a month award to the saved game as `month_award_load_from_disk` reads it: the twelve month slots, then the
/// slot count (`exe::zwrite`); a short write shows the "Error" box (month_award.cpp 135, 142) and stops there.
/// Original: `MONTH_AWARD::save_to_disk` (0x767070, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_save_to_disk(this: usize, file: usize) {
    if exe::zwrite(this + month_award_fields::SLOT_WINNER_ID, MONTH_AWARD_SLOT_SIZE as u32, 12, file) != 12 { // sx04b_6: the twelve slots
        return tel26_error_box(MONTH_AWARD_CPP, 0x87);
    }
    if exe::zwrite(this + MONTH_AWARD_SLOT_COUNT, 1, 1, file) != 1 {
        return tel26_error_box(MONTH_AWARD_CPP, 0x8e);
    }
    covered!("sx04b_6 month award saved");
}

/// Comparer for `month_award_save_to_disk`: it writes the file once; only the original runs (the save proof compares
/// the written game).
pub(crate) unsafe extern "thiscall" fn compare_month_award_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, file)
}

/// Reads the month ratings from the saved game: the entry count (4 bytes at `MONTH_RATINGS_COUNT`); with entries, the
/// capacity becomes the count, a block of count × 12 bytes is allocated (`exe::crt_malloc`) for the entries and they
/// are read; then the month (`month_ratings_fields::MONTH`, a byte) and the year (`month_ratings_fields::YEAR`, 2
/// bytes). Returns 1, or 0 after the box of the first failure: Load Game Error (month_ratings.cpp 86 "cannot read
/// number of players", 105 "cannot read players", 113 "cannot read current month", 120 "cannot read current year")
/// or Memory Allocation Error (98 "cannot malloc players array").
/// Original: `MONTH_RATINGS::load_from_disk` (0x767b20, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    if exe::zread(this + MONTH_RATINGS_COUNT, 4, 1, file) != 1 {
        award_load_error_box(MONTH_RATINGS_CPP, 0x56, 0xa5b1f4, LOAD_ERROR);
        return 0;
    }
    let count = read_u32(this + MONTH_RATINGS_COUNT);
    if count != 0 {
        write_u32(this + MONTH_RATINGS_CAPACITY, count);
        let entries = exe::crt_malloc(count.wrapping_mul(MONTH_RATING_ENTRY_SIZE));
        write_usize(this + MONTH_RATINGS_ENTRIES, entries);
        if entries == 0 {
            award_load_error_box(MONTH_RATINGS_CPP, 0x62, 0xa5b1b4, MEMORY_ERROR);
            return 0;
        }
        if exe::zread(entries, MONTH_RATING_ENTRY_SIZE, read_u32(this + MONTH_RATINGS_COUNT), file) != read_u32(this + MONTH_RATINGS_COUNT) {
            award_load_error_box(MONTH_RATINGS_CPP, 0x69, 0xa5b17c, LOAD_ERROR);
            return 0;
        }
    }
    if exe::zread(this + month_ratings_fields::MONTH, 1, 1, file) != 1 { // sx04b_6: the ratings' month
        award_load_error_box(MONTH_RATINGS_CPP, 0x71, 0xa5b140, LOAD_ERROR);
        return 0;
    }
    if exe::zread(this + month_ratings_fields::YEAR, 2, 1, file) != 1 {
        award_load_error_box(MONTH_RATINGS_CPP, 0x78, 0xa5b104, LOAD_ERROR);
        return 0;
    }
    covered!("sx04b_6 month ratings loaded");
    1
}

/// Comparer for `month_ratings_load_from_disk`: the load reads the file once and allocates; only the original runs,
/// and loaded ratings must have their capacity at least their count.
pub(crate) unsafe extern "thiscall" fn compare_month_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, file);
    if r == 1 {
        let (n, cap) = (read_u32(this + MONTH_RATINGS_COUNT), read_u32(this + MONTH_RATINGS_CAPACITY));
        SCOREBOARD.record(n == 0 || (cap == n && read_usize(this + MONTH_RATINGS_ENTRIES) != 0), || format!("month_ratings_load_from_disk({:#x}): count {} capacity {}", this, n, cap));
    }
    r
}
