// --- sx16_8: the notes manager: Notes::Notes (0x794ad0), Notes::~Notes (0x794ba0), Notes::load_notes (0x794be0) ---

/// The record sizes of a notes slot's two lists: the staff notes (0x114 bytes) and the second list's records.
const NOTES_FIRST_RECORD_SIZE: u32 = 0x114;
const NOTE_SECOND_SIZE: u32 = 0x10c;
/// The source path of notes.cpp in the "Error" box (the save routines; the screens' item has its own name for it).
const NOTES_SAVE_SOURCE: usize = 0xa69a18;
/// The save item "notes.dat".
const NOTES_DAT: usize = 0xa69a40;
/// The game data version from which saves carry notes.dat (above it, as an absolute value).
const NOTES_FIRST_VERSION: i32 = 0x3e032;

/// Builds the notes manager: the 16 manager slots get their two empty note lists (`operator new` of 14 bytes and
/// `LLIST::LLIST`; a failed allocation leaves the pointer 0); with a save of a game data version above 0x3e032 (the
/// absolute `G_GAME_DATA_VERSION`), the notes are read from it (`Notes::load_notes` through the exe; its answer is not
/// used). Returns the manager.
/// Original: `Notes::Notes` (0x794ad0, notes.cpp, thiscall, ret 4; SEH frame for the list allocations, nothing thrown).
pub(crate) unsafe extern "thiscall" fn new_notes(this: usize, save: usize) -> usize {
    for k in 0..0x20usize {
        let list = exe::crt_operator_new(0xe);
        write_usize(this + k * 4, if list != 0 { exe::new_llist(list) } else { 0 });
    }
    if save != 0 && read_i32(G_GAME_DATA_VERSION).wrapping_abs() > NOTES_FIRST_VERSION {
        covered!("sx16_8 notes from the save");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x794be0usize); // Notes::load_notes
        load(this, save);
    }
    this
}

/// Comparer for `new_notes`: it allocates (and may read the save), so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_new_notes(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Destroys the notes manager: each of the 32 note lists present is emptied (`LLIST::~LLIST`) and freed (`operator
/// delete`); the pointers stay.
/// Original: `Notes::~Notes` (0x794ba0, notes.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_notes(this: usize) {
    for k in 0..0x20usize {
        let list = read_usize(this + k * 4);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
    }
}

/// Comparer for `drop_notes`: it frees, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_notes(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// The notes.cpp "Error" box at `row` of a failed save item, the item closed: the answer 0.
unsafe fn notes_item_error(row: u32, z: &mut usize) -> u8 {
    show_error_box(NOTES_SAVE_SOURCE, row);
    exe::zclose(z);
    0
}

/// Reads notes.dat from a saved game: nothing (0) without a save or without the item. For each of the 16 manager
/// slots: the count of the first list and its 0x114-byte notes, each appended (`LLIST::add_tail`); the count of the
/// second list and its 0x10c-byte records, appended to it. Closes the item and answers 1. A short read shows the
/// notes.cpp "Error" box (counts lines 0x73 and 0x88, records 0x7d and 0x91), closes the item and answers 0 (the notes
/// read so far stay).
/// Original: `Notes::load_notes` (0x794be0, notes.cpp, thiscall, ret 4; SEH frame for local dates, nothing thrown).
pub(crate) unsafe extern "thiscall" fn notes_load_notes(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, NOTES_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx16_8 load notes.dat");
    let mut record = [0u8; NOTES_FIRST_RECORD_SIZE as usize];
    for k in 0..0x10usize {
        for (list, size, row_count, row_record) in [(this + k * 8, NOTES_FIRST_RECORD_SIZE, 0x73, 0x7d), (this + k * 8 + 4, NOTE_SECOND_SIZE, 0x88, 0x91)] {
            let mut count = 0i32;
            if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
                return notes_item_error(row_count, &mut z);
            }
            for _ in 0..count.max(0) {
                if exe::zread(record.as_mut_ptr() as usize, size, 1, z) != 1 {
                    return notes_item_error(row_record, &mut z);
                }
                exe::llist_add_tail(read_usize(list), record.as_ptr() as usize, size);
            }
        }
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `notes_load_notes`: it reads the save item and allocates, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_notes_load_notes(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(this, save)
}
