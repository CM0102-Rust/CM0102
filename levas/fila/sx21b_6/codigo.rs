// ---------- sx21b_6: the training manager in the saved game: training.dat loaded and saved, and the destructor (training_manager.cpp) ----------

/// `TRAINING_SCHEDULE::~TRAINING_SCHEDULE` (thiscall), the vector destructor's element destructor.
const FN_TRAINING_SCHEDULE_DESTROY: usize = 0x5a53c0;
/// "training.dat", the saved game's item.
const TRAINING_DAT: usize = 0xa8ccc0;
/// "...\training_manager.cpp", for the error box.
const TRAINING_MANAGER_CPP: usize = 0xa8ccd0;

/// One block of training.dat: its address, record size, record count and the error box line of a short read/write.
unsafe fn training_dat_blocks(this: usize) -> [(usize, u32, u32, u32, u32); 4] {
    use training_manager_fields as tm;
    let players = read_u32(G_PLAYER_DATA_COUNT);
    [
        (read_usize(this + tm::PLAYER_ENTRIES), 6, players, 0x96, 0xf1),
        (this + tm::WEEK_COUNTER, 2, 1, 0x9d, 0xf8),
        (this, 0x9b, 0xa7, 0xa4, 0xff), // sx21b_6: the 167 schedules
        (this + 0x6529, 4, 1, 0xab, 0x106), // the position training count as saved
    ]
}

/// Reads training.dat from the saved game into the training manager; 1 when everything was read.
///
/// `TRAINING_MANAGER::load_training_dat(save)`: no save: the box (line 0x7f) and 0. The item (`exe::zopen` "training.dat",
/// reading) or the box (0x87). Then, each short read showing its box and returning 0: the player entries (6 bytes for
/// each of `G_PLAYER_DATA_COUNT` player sheets, into a new `malloc` block at `PLAYER_ENTRIES`; none: box 0x8f), the
/// `WEEK_COUNTER`, the 167 0x9b-byte schedules, the position training count (into +0x6529, copied to
/// `POSITION_TRAINING_COUNT`) and, when not 0, that many 8-byte items into a new block at `POSITION_TRAINING_ITEMS`
/// (none: box 0xb9; short: 0xc0), and the 0x60 nine-byte coach slots of `COACH_ASSIGNMENTS` (0xc8). The item is
/// closed (`exe::zclose`) only on success.
/// Original: `TRAINING_MANAGER::load_training_dat` (0x8b10d0, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_training_dat(this: usize, save: usize) -> u32 {
    use training_manager_fields as tm;
    if save == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0x7f);
        return 0;
    }
    let mut z = exe::zopen(save, TRAINING_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0x87);
        return 0;
    }
    let entries = exe::crt_malloc(read_u32(G_PLAYER_DATA_COUNT).wrapping_mul(6));
    write_usize(this + tm::PLAYER_ENTRIES, entries);
    if entries == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0x8f);
        return 0;
    }
    for (at, size, count, line, _) in training_dat_blocks(this) {
        if exe::zread(at, size, count, z) != count {
            show_error_at(TRAINING_MANAGER_CPP, line);
            return 0;
        }
    }
    let count = read_u32(this + 0x6529);
    write_u32(this + tm::POSITION_TRAINING_COUNT, count);
    if count != 0 {
        covered!("sx21b_6 position training loaded");
        let items = exe::crt_malloc(count.wrapping_mul(8));
        write_usize(this + tm::POSITION_TRAINING_ITEMS, items);
        if items == 0 {
            show_error_at(TRAINING_MANAGER_CPP, 0xb9);
            return 0;
        }
        if exe::zread(items, 8, read_u32(this + 0x6529), z) != read_u32(this + 0x6529) {
            show_error_at(TRAINING_MANAGER_CPP, 0xc0);
            return 0;
        }
    }
    if exe::zread(this + tm::COACH_ASSIGNMENTS, 9, 0x60, z) != 0x60 { // sx21b_6: the coach slots
        show_error_at(TRAINING_MANAGER_CPP, 0xc8);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `load_training_dat`: only the original (it reads the saved game's item and allocates); the copy is
/// proven by loading the holiday save with and without the batch.
pub(crate) unsafe extern "thiscall" fn compare_load_training_dat(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}

/// Writes the training manager into the saved game's training.dat; 1 when everything was written.
///
/// `TRAINING_MANAGER::save_training_dat(save)`: no save: the box (line 0xe2) and 0. The item (`exe::zopen`
/// "training.dat", writing) or the box (0xea). Then, each short write showing its box and returning 0: the player
/// entries (0xf1), the `WEEK_COUNTER` (0xf8), the 167 schedules (0xff), the dword at +0x6529 (0x106), the position
/// training items when there is a block (as many as that dword; 0x110), and the coach slots (0x118). The item is
/// closed only on success.
/// Original: `TRAINING_MANAGER::save_training_dat` (0x8b1450, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_training_dat(this: usize, save: usize) -> u32 {
    use training_manager_fields as tm;
    if save == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0xe2);
        return 0;
    }
    let mut z = exe::zopen(save, TRAINING_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0xea);
        return 0;
    }
    for (at, size, count, _, line) in training_dat_blocks(this) {
        if exe::zwrite(at, size, count, z) != count {
            show_error_at(TRAINING_MANAGER_CPP, line);
            return 0;
        }
    }
    let items = read_usize(this + tm::POSITION_TRAINING_ITEMS);
    if items != 0 {
        covered!("sx21b_6 position training saved");
        if exe::zwrite(items, 8, read_u32(this + 0x6529), z) != read_u32(this + 0x6529) {
            show_error_at(TRAINING_MANAGER_CPP, 0x110);
            return 0;
        }
    }
    if exe::zwrite(this + tm::COACH_ASSIGNMENTS, 9, 0x60, z) != 0x60 {
        show_error_at(TRAINING_MANAGER_CPP, 0x118);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `save_training_dat`: only the original (it writes the saved game's item); the copy is proven by the
/// saved game written with and without the batch.
pub(crate) unsafe extern "thiscall" fn compare_save_training_dat(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
}

/// Destroys the training manager: frees the player entries and the position training items, and destroys the 167
/// schedules.
///
/// `TRAINING_MANAGER::~TRAINING_MANAGER()`: `PLAYER_ENTRIES` and `POSITION_TRAINING_ITEMS`, when present, freed
/// (`exe::crt_free`) and cleared; then `exe::crt_vector_destructor_iterator(this, 0x9b, 0xa7, ~TRAINING_SCHEDULE)`.
/// Original: `TRAINING_MANAGER::~TRAINING_MANAGER` (0x8b1070, training_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn destroy_training_manager(this: usize) {
    use training_manager_fields as tm;
    for at in [tm::PLAYER_ENTRIES, tm::POSITION_TRAINING_ITEMS] {
        let p = read_usize(this + at);
        if p != 0 {
            exe::crt_free(p);
            write_u32(this + at, 0);
        }
    }
    exe::crt_vector_destructor_iterator(this, 0x9b, 0xa7, FN_TRAINING_SCHEDULE_DESTROY);
}

/// Comparer for `destroy_training_manager`: only the original (it frees memory).
pub(crate) unsafe extern "thiscall" fn compare_destroy_training_manager(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
