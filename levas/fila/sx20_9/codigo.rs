// --- sx20_9: writing and freeing the tactics store (tactics.cpp: Tactics::write_to_save 0x8995d0, Tactics::free_data
// 0x8961e0) and the old shortlist slot record (shortlist_manager.cpp: ShortlistOldSlot::ShortlistOldSlot 0x83b360) ---

/// tactics.cpp, the source path of the error boxes.
const TACTICS_SOURCE: usize = 0xa8b908;
/// The size of a tactic block (`tactics_fields::CUSTOM_TACTICS`, `PRESET_TACTICS`), of a club tactics row and of a
/// per-tactic statistics record.
const TACTIC_BLOCK_SIZE: u32 = 0x18e3;
const TACTICS_ROW_SIZE: u32 = 0x91;
const TACTIC_STATS_SIZE: u32 = 0x28;

/// Writes the tactics store into the saved game: without a save the error box (line 0x96c) and 0; the "tactics.dat"
/// item (missing: line 0x974 and 0); the custom tactic count and blocks (0x18e3 bytes), the preset count and blocks,
/// the club row count and rows (0x91 bytes), the statistics count (+4) and records (0x28 bytes from `[+0]`) and the
/// ten default values from +8; closes the item and returns 1. A short write shows the box (lines 0x97b to 0x9bb),
/// closes the item and returns 0.
/// Original: `Tactics::write_to_save` (0x8995d0, tactics.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tactics_write_to_save(this: usize, save: usize) -> u32 {
    use tactics_fields as f;
    if save == 0 {
        show_error_at(TACTICS_SOURCE, 0x96c);
        return 0;
    }
    let mut z = zip_open_item(save, 0xa8b940); // "tactics.dat"
    if z == 0 {
        show_error_at(TACTICS_SOURCE, 0x974);
        return 0;
    }
    covered!("sx20_9 save tactics");
    let parts: [(usize, u32, u32, u32); 9] = [
        (this + f::CUSTOM_TACTIC_COUNT, 4, 1, 0x97b),
        (read_usize(this + f::CUSTOM_TACTICS), TACTIC_BLOCK_SIZE, read_u32(this + f::CUSTOM_TACTIC_COUNT), 0x983),
        (this + f::PRESET_TACTIC_COUNT, 4, 1, 0x98b),
        (read_usize(this + f::PRESET_TACTICS), TACTIC_BLOCK_SIZE, read_u32(this + f::PRESET_TACTIC_COUNT), 0x993),
        (this + f::ROW_COUNT, 4, 1, 0x99b),
        (read_usize(this + f::ROWS), TACTICS_ROW_SIZE, read_u32(this + f::ROW_COUNT), 0x9a3), // sx20_9: the club rows
        (this + 4, 4, 1, 0x9ab), // the statistics count
        (read_usize(this + f::TACTIC_STATS), TACTIC_STATS_SIZE, read_u32(this + 4), 0x9b3),
        (this + 8, 4, 10, 0x9bb), // the ten default values
    ];
    for (data, size, count, line) in parts {
        if zip_write(data, size, count, z) != count {
            show_error_at(TACTICS_SOURCE, line);
            zip_close(&mut z);
            return 0;
        }
    }
    zip_close(&mut z);
    1
}

/// Frees an array of tactic blocks built with `new[]` (the `eh vector destructor iterator` with the empty 0x5a53c0
/// over the count stored just before it, then `delete` of the allocation) and clears the pointer and its count.
unsafe fn tactics_free_blocks(this: usize, blocks: usize, count: usize) {
    let p = read_usize(this + blocks);
    if p == 0 {
        return;
    }
    exe::crt_vector_destructor_iterator(p, TACTIC_BLOCK_SIZE, read_u32(p - 4), 0x5a53c0usize);
    exe::crt_operator_delete(p - 4);
    write_usize(this + blocks, 0);
    write_usize(this + count, 0);
}

/// Frees the tactics store: the custom and preset tactic blocks (`tactics_free_blocks`), the club rows (`free`) and
/// the per-tactic statistics (`delete`), each with its pointer and count cleared.
/// Original: `Tactics::free_data` (0x8961e0, tactics.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tactics_free_data(this: usize) {
    use tactics_fields as f;
    tactics_free_blocks(this, f::CUSTOM_TACTICS, f::CUSTOM_TACTIC_COUNT);
    tactics_free_blocks(this, f::PRESET_TACTICS, f::PRESET_TACTIC_COUNT);
    if read_usize(this + f::ROWS) != 0 {
        exe::crt_free(read_usize(this + f::ROWS));
        write_usize(this + f::ROWS, 0);
        write_usize(this + f::ROW_COUNT, 0);
    }
    covered!("sx20_9 tactics freed");
    if read_usize(this + f::TACTIC_STATS) != 0 {
        exe::crt_operator_delete(read_usize(this + f::TACTIC_STATS));
        write_usize(this + f::TACTIC_STATS, 0);
        write_usize(this + 4, 0); // the statistics count
    }
}

/// Builds one 0x24-byte old-format shortlist slot record (from saves of an older version): the default date
/// (`Date::Date`) in its field at +0x1a. Returns the record.
/// Original: `ShortlistOldSlot::ShortlistOldSlot` (0x83b360, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn shortlist_old_slot_shortlist_old_slot(this: usize) -> usize {
    exe::date_date_54a110(this + 0x1a);
    this
}

/// Comparer for `tactics_write_to_save`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_write_to_save(this: usize, save: usize) -> u32 {
    // simplification: writes into the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}

/// Comparer for `tactics_free_data`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_free_data(this: usize) {
    // simplification: frees memory (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Comparer for `shortlist_old_slot_shortlist_old_slot`: side by side over the record's date (8 bytes at +0x1a).
pub(crate) unsafe extern "thiscall" fn compare_shortlist_old_slot_shortlist_old_slot(this: usize) -> usize {
    let original = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, c, same, mo, mr) = crate::comparison::side_by_side(&[(this + 0x1a, 8)], || original(this), || shortlist_old_slot_shortlist_old_slot(this));
    SCOREBOARD.record(same && r == c, || format!("shortlist_old_slot_shortlist_old_slot({:#x}) {:?}", this, first_differences(&mo, &mr)));
    r
}
