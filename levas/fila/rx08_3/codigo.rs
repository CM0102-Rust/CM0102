// ---------- rx08_3: the player statistics object destroyed and loaded from the saved game (player_stats.cpp) ----------

/// The player statistics' history temp file ("player stats history.tmp" in "CM3_TEMP"), the scratch file of the old
/// history conversion ("new_history.tmp", in the current folder) and the last data version with the old layout
/// (0x10-byte category records and 0x1b-byte history nodes).
pub(crate) mod player_stats_load_consts {
    pub(crate) const TEMP_FOLDER: usize = 0x987da0;
    pub(crate) const HISTORY_TMP: usize = 0xa78340;
    pub(crate) const NEW_HISTORY_TMP: usize = 0xa784d8;
    pub(crate) const NEW_LAYOUT_VERSION: i32 = 0x3e015;
    pub(crate) const OLD_CATEGORY_SIZE: u32 = 0x10;
    pub(crate) const OLD_NODE_SIZE: u32 = 0x1b;
    pub(crate) const NODE_SIZE: u32 = 0x2b;
}

/// Utils: copies a file (src, dst), cdecl.
const FN_UTILS_COPY_FILE: usize = 0x90cc50;

/// Destroys the player statistics object: the history file list, the category records, the per-player entries and the
/// language records.
///
/// `PlayerStats::~PlayerStats()`: the history list present is destroyed (FILE_LLIST::~FILE_LLIST, then `operator
/// delete`) and zeroed; the category records present freed with their pointer, capacity and count zeroed; the entries
/// present freed with their pointer and count zeroed; the languages present freed with their pointer and capacity
/// zeroed (their count `player_stats_fields::LANGUAGE_COUNT` is left as it was).
/// Original: `PlayerStats::~PlayerStats` (0x7c0d00, player_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_player_stats(this: usize) {
    use player_stats_fields as f;
    let list = read_usize(this + f::HISTORY_FILE);
    if list != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(club_history_consts::FILE_LLIST_DESTRUCTOR);
        destroy(list);
        exe::crt_operator_delete(list);
        write_usize(this + f::HISTORY_FILE, 0);
    }
    let arrays: [(usize, &[usize]); 3] = [(f::CATEGORY_RECORDS, &[f::CATEGORY_CAPACITY, f::CATEGORY_COUNT]),
        (f::PLAYER_ENTRIES, &[player_stats_save_consts::ENTRY_COUNT]), (f::LANGUAGES, &[f::LANGUAGE_CAPACITY])];
    for (pointer, sizes) in arrays {
        let p = read_usize(this + pointer);
        if p != 0 {
            exe::crt_free(p);
            write_usize(this + pointer, 0);
            for &k in sizes {
                write_u32(this + k, 0);
            }
        }
    }
}

/// Comparer for `drop_player_stats`: it frees, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_player_stats(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Whether the saved game predates the new player statistics layout: the absolute value of the data version
/// (`G_GAME_DATA_VERSION`, signed) below 0x3e015.
unsafe fn player_stats_old_layout() -> bool {
    read_i32(G_GAME_DATA_VERSION).wrapping_abs() < player_stats_load_consts::NEW_LAYOUT_VERSION
}

/// A new 0x2b-byte history node from an old 0x1b-byte one: the first 0x16 bytes as they are, 0x10 new bytes of zero,
/// then the old bytes 0x16..0x1b.
fn player_stats_convert_node(old: &[u8]) -> [u8; 0x2b] {
    let mut node = [0u8; 0x2b];
    node[..0x16].copy_from_slice(&old[..0x16]);
    node[0x26..0x2b].copy_from_slice(&old[0x16..0x1b]);
    node
}

/// A new FILE_LLIST of `record_size`-byte nodes on `path` (operator new(8), FILE_LLIST::FILE_LLIST), or 0 without
/// memory.
unsafe fn player_stats_new_file_list(path: usize, record_size: u32) -> usize {
    let p = exe::crt_operator_new(8);
    if p == 0 {
        return 0;
    }
    exe::new_file_llist(p, path, record_size)
}

/// Destroys a FILE_LLIST present (FILE_LLIST::~FILE_LLIST, then `operator delete`).
unsafe fn player_stats_drop_file_list(list: usize) {
    if list != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(club_history_consts::FILE_LLIST_DESTRUCTOR);
        destroy(list);
        exe::crt_operator_delete(list);
    }
}

/// Converts the career history of an old saved game: the history file of 0x1b-byte nodes is rewritten as one of
/// 0x2b-byte nodes, player by player.
///
/// The history list replaced by one over the same file ("CM3_TEMP\player stats history.tmp") with the old 0x1b-byte
/// nodes, and a scratch list on "new_history.tmp" with 0x2b-byte nodes. Each per-player entry (count +0x34): its nodes
/// (`exe::file_llist_get_nodes` from `player_stats_entry::HISTORY_START` to `HISTORY_LAST`); none: both offsets −1.
/// Otherwise each node, from the head, converted (`player_stats_convert_node`) and appended to a local list
/// (`exe::llist_add_tail`, 0x2b), both offsets −1, the local list written to the scratch file
/// (`exe::file_llist_add_node_list`, which sets the new offsets), the local list cleared and the node list destroyed.
/// Then both lists destroyed (the history pointer is not zeroed: it is replaced at the end), the history file removed,
/// the scratch file copied over it and removed, and the history list opened again with 0x2b-byte nodes (0 without
/// memory).
unsafe fn player_stats_convert_history(this: usize) {
    use player_stats_fields as f;
    use player_stats_load_consts as c;
    covered!("rx08_3 old player history converted");
    let mut local = [0u8; 0x10];
    let local = exe::new_llist(local.as_mut_ptr() as usize);
    player_stats_drop_file_list(read_usize(this + f::HISTORY_FILE));
    write_usize(this + f::HISTORY_FILE, player_stats_new_file_list(exe::utils_build_file_path(c::TEMP_FOLDER, c::HISTORY_TMP), c::OLD_NODE_SIZE));
    let scratch = player_stats_new_file_list(c::NEW_HISTORY_TMP, c::NODE_SIZE);
    for k in 0..read_i32(this + player_stats_save_consts::ENTRY_COUNT).max(0) as usize {
        let e = read_usize(this + f::PLAYER_ENTRIES) + k * player_stats_save_consts::ENTRY_SIZE as usize;
        let (start, last) = (e + player_stats_entry::HISTORY_START, e + player_stats_entry::HISTORY_LAST);
        let nodes = exe::file_llist_get_nodes(read_usize(this + f::HISTORY_FILE), read_u32(start), read_u32(last));
        if nodes != 0 {
            write_u32(nodes + 0xa, read_u32(nodes + 2)); // the walk starts at the head
            loop {
                let old = exe::llist_get_next(nodes);
                if old == 0 {
                    break;
                }
                let node = player_stats_convert_node(core::slice::from_raw_parts(old as *const u8, c::OLD_NODE_SIZE as usize));
                exe::llist_add_tail(local, node.as_ptr() as usize, c::NODE_SIZE);
            }
        }
        write_u32(start, u32::MAX);
        write_u32(last, u32::MAX);
        if nodes != 0 {
            exe::file_llist_add_node_list(scratch, start, last, local);
            exe::llist_clear(local);
            exe::drop_llist(nodes);
            exe::crt_operator_delete(nodes);
        }
    }
    player_stats_drop_file_list(read_usize(this + f::HISTORY_FILE));
    player_stats_drop_file_list(scratch);
    exe::crt_remove(exe::utils_build_file_path(c::TEMP_FOLDER, c::HISTORY_TMP) as *const u8);
    let copy: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_UTILS_COPY_FILE);
    copy(c::NEW_HISTORY_TMP, exe::utils_build_file_path(c::TEMP_FOLDER, c::HISTORY_TMP));
    exe::crt_remove(c::NEW_HISTORY_TMP as *const u8);
    write_usize(this + f::HISTORY_FILE, player_stats_new_file_list(exe::utils_build_file_path(c::TEMP_FOLDER, c::HISTORY_TMP), c::NODE_SIZE));
    exe::drop_llist(local);
}

/// Reads the player statistics from the saved game's "player stats.dat" item.
///
/// `PlayerStats::load_stats(save)`: no save: the player_stats.cpp 0xd1a box and 0; the item missing (`zopen` with 0,
/// 0x16, 4): the 0xd22 box and 0. Then, each short read or failed allocation stopping the game: the category capacity
/// and count (0xd29, 0xd31), the category records (`malloc` of capacity × 0x20, 0xd3a): in an old saved game
/// (`player_stats_old_layout`) one 0x10-byte record at a time with the new +0x10..+0x1f zeroed (0xd46), otherwise the
/// count's 0x20-byte records at once (0xd58); the entry count (0xd61), the entries (`malloc` of count × 0x27, 0xd6a;
/// read at once, 0xd72); the four rating doubles (0xd7a, 0xd82, 0xd8a, 0xd92) and the free category count (0xd9a).
/// The languages: the capacity and the count, where a short read only shows the box (0xda2, 0xda8), closes the item
/// and answers 0; the records (`malloc` of capacity × 6 into `LANGUAGES`, but the original tests the capacity, not the
/// block: a zero capacity stops the game, line 0xdaf, and a failed allocation goes on), as many 6-byte records as the
/// capacity (short: the 0xdb3 box, closed, 0). The item closed; an old saved game has its history converted
/// (`player_stats_convert_history`). Returns 1.
/// Original: `PlayerStats::load_stats` (0x7c48b0, player_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_stats_load(this: usize, save: usize) -> u32 {
    use player_stats_fields as f;
    use player_stats_load_consts as c;
    use player_stats_save_consts as s;
    if save == 0 {
        show_error_at(s::PLAYER_STATS_CPP, 0xd1a);
        return 0;
    }
    let mut z = exe::zopen(save, s::SAVE_ITEM, 0, 0x16, 4);
    if z == 0 {
        show_error_at(s::PLAYER_STATS_CPP, 0xd22);
        return 0;
    }
    covered!("rx08_3 load player stats");
    let read = |data: usize, size: u32, n: u32, row: u32| zread_or_stop(s::PLAYER_STATS_CPP, row, z, data, size, n);
    read(this + f::CATEGORY_CAPACITY, 4, 1, 0xd29);
    read(this + f::CATEGORY_COUNT, 4, 1, 0xd31);
    let records = exe::crt_malloc(read_u32(this + f::CATEGORY_CAPACITY) << 5);
    write_usize(this + f::CATEGORY_RECORDS, records);
    if records == 0 {
        fatal_error_at(s::PLAYER_STATS_CPP, 0xd3a);
    }
    if player_stats_old_layout() {
        for k in 0..read_i32(this + f::CATEGORY_COUNT).max(0) as usize {
            let r = read_usize(this + f::CATEGORY_RECORDS) + k * s::CATEGORY_SIZE as usize;
            read(r, c::OLD_CATEGORY_SIZE, 1, 0xd46);
            core::ptr::write_bytes((r + 0x10) as *mut u8, 0, 0x10); // the fields the old records lack
        }
    } else {
        read(records, s::CATEGORY_SIZE, read_u32(this + f::CATEGORY_COUNT), 0xd58);
    }
    read(this + s::ENTRY_COUNT, 4, 1, 0xd61);
    let entries = exe::crt_malloc(read_u32(this + s::ENTRY_COUNT).wrapping_mul(s::ENTRY_SIZE));
    write_usize(this + f::PLAYER_ENTRIES, entries);
    if entries == 0 {
        fatal_error_at(s::PLAYER_STATS_CPP, 0xd6a);
    }
    read(entries, s::ENTRY_SIZE, read_u32(this + s::ENTRY_COUNT), 0xd72);
    for (field, row) in [(f::RATING_SUM, 0xd7a), (f::RATED_COUNT, 0xd82), (f::FLAGGED_RATING_SUM, 0xd8a), (f::FLAGGED_RATED_COUNT, 0xd92)] {
        read(this + field, 8, 1, row);
    }
    read(this + f::FREE_CATEGORY_COUNT, 4, 1, 0xd9a);
    for (field, row) in [(f::LANGUAGE_CAPACITY, 0xda2), (f::LANGUAGE_COUNT, 0xda8)] {
        if exe::zread(this + field, 4, 1, z) != 1 {
            show_error_at(s::PLAYER_STATS_CPP, row);
            exe::zclose(&mut z);
            return 0;
        }
    }
    let capacity = read_u32(this + f::LANGUAGE_CAPACITY);
    write_usize(this + f::LANGUAGES, exe::crt_malloc(capacity.wrapping_mul(s::LANGUAGE_SIZE)));
    if capacity == 0 {
        fatal_error_at(s::PLAYER_STATS_CPP, 0xdaf);
    }
    if exe::zread(read_usize(this + f::LANGUAGES), s::LANGUAGE_SIZE, capacity, z) != capacity {
        show_error_at(s::PLAYER_STATS_CPP, 0xdb3);
        exe::zclose(&mut z);
        return 0;
    }
    exe::zclose(&mut z);
    if player_stats_old_layout() {
        player_stats_convert_history(this);
    }
    1 // rx08_3: the statistics loaded
}

/// Comparer for `player_stats_load`: it allocates and reads the save item (and may rewrite the history file), so it
/// cannot run twice; only the original runs. The copy is proven by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_player_stats_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
}
