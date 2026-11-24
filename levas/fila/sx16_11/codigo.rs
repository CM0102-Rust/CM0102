// --- sx16_11: the regen manager: PLAYER_REGEN::PLAYER_REGEN (0x7a5380), ~PLAYER_REGEN (0x7a56f0),
// load_retire_dat (0x7adcd0) ---

/// The four regen tables in their order (retire.dat and the constructor): (table, count, capacity, record size, the
/// constructor's out-of-memory line, the loading lines of the count, the allocation and the records).
const REGEN_TABLES: [(usize, usize, usize, u32, u32, [u32; 3]); 4] = [
    (player_regen_fields::RETIREMENTS, player_regen_fields::RETIREMENT_COUNT, player_regen_fields::RETIREMENT_CAPACITY, 0x1a, 0x7f, [0x1aac, 0x1ab6, 0x1abe]),
    (player_regen_fields::REGEN_QUEUE, player_regen_fields::REGEN_QUEUE_COUNT, player_regen_fields::REGEN_QUEUE_CAPACITY, 0xc, 0x83, [0x1ac7, 0x1ad1, 0x1ad9]),
    (player_regen_fields::SPARE_NON_PLAYER_RECORDS, player_regen_fields::SPARE_NON_PLAYER_COUNT, player_regen_fields::SPARE_NON_PLAYER_CAPACITY, 0x10, 0x87, [0x1ae2, 0x1aec, 0x1af4]),
    (player_regen_fields::PLAYER_DATA_POOL, player_regen_fields::PLAYER_DATA_POOL_COUNT, player_regen_fields::PLAYER_DATA_POOL_CAPACITY, 0x10, 0x8b, [0x1afd, 0x1b07, 0x1b0f]),
];
/// The room the regen tables get beyond their count (250 records).
const REGEN_TABLE_SLACK: u32 = 0xfa;
/// The source path of player_regen.cpp in the "Error" box.
const PLAYER_REGEN_CPP_PATH: usize = 0xa6bf9c;

/// Builds the regen manager: first the game CD (`game_cd_check`, item sx16_7); the four tables empty with room for 250
/// records each (`malloc`, `REGEN_TABLES`); the per-nation club lists (`build_nation_club_lists` through the exe). A new
/// game: the retirement list built fresh (`build_initial_retirement_list`) and the last ten non-player rows
/// (`G_NON_PLAYER_COUNT` − k − 1) added as spare records with no nations (`add_spare_non_player_record(row, −1, −1)`); a
/// save: the tables read from retire.dat (`load_retire_dat`, its answer unused), all through the exe. Then a table left
/// without memory stops the game (player_regen.cpp fatal box, lines 0x7f, 0x83, 0x87, 0x8b). Returns the manager.
/// Defect of the original: the allocations are tested only after they were used.
/// Original: `PLAYER_REGEN::PLAYER_REGEN` (0x7a5380, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_player_regen(this: usize, save: usize) -> usize {
    game_cd_check(0x7a53c9, 0x7a540d);
    for (table, count, capacity, size, _, _) in REGEN_TABLES {
        write_usize(this + table, exe::crt_malloc(size * REGEN_TABLE_SLACK));
        write_u32(this + capacity, REGEN_TABLE_SLACK);
        write_u32(this + count, 0);
    }
    let call1 = |f: usize| {
        let g: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        g(this)
    };
    call1(0x7acfc0); // build_nation_club_lists
    if save == 0 {
        covered!("sx16_11 new game regens");
        call1(0x7a58f0); // build_initial_retirement_list
        let add: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x7a81a0usize); // add_spare_non_player_record
        for k in 0..0xa {
            add(this, (read_i32(G_NON_PLAYER_COUNT) - k - 1) as u32, u32::MAX, u32::MAX);
        }
    } else {
        covered!("sx16_11 regens from the save");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7adcd0usize); // load_retire_dat
        load(this, save);
    }
    for (table, _, _, _, row, _) in REGEN_TABLES {
        if read_usize(this + table) == 0 {
            show_error_box(PLAYER_REGEN_CPP_PATH, row);
            fatal_exit();
        }
    }
    this
}

/// Comparer for `new_player_regen`: it asks for the CD, allocates and builds the lists (or reads the save); only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_new_player_regen(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Frees the regen manager: the four tables (each present one freed and its pointer zeroed) and, when there are any,
/// the per-nation club lists (`G_NATION_COUNT` of them: each present list emptied, `LLIST::~LLIST`, and deleted), then
/// their array (freed and zeroed).
/// Original: `PLAYER_REGEN::~PLAYER_REGEN` (0x7a56f0, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_player_regen(this: usize) {
    for (table, _, _, _, _, _) in REGEN_TABLES {
        let p = read_usize(this + table);
        if p != 0 {
            exe::crt_free(p);
            write_usize(this + table, 0);
        }
    }
    let lists = read_usize(this); // the per-nation club lists
    if lists == 0 {
        return;
    }
    let mut k = 0i32;
    while k < read_i32(G_NATION_COUNT) {
        let list = read_usize(read_usize(this) + k as usize * 4);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
        k += 1;
    }
    exe::crt_free(read_usize(this));
    write_usize(this, 0);
}

/// Comparer for `drop_player_regen`: it frees, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_player_regen(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Reads the four regen tables from "retire.dat" in the save: for each (`REGEN_TABLES`), the count, the capacity count
/// + 250 and the table reallocated to it (`realloc`), then the records (when any). Closes the item and answers 1.
/// Without a save (line 0x1a9d) or the item (0x1aa5): the player_regen.cpp "Error" box and 0. A short read or a failed
/// reallocation: the box, the item closed and 0.
/// Original: `PLAYER_REGEN::load_retire_dat` (0x7adcd0, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_retire_dat(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(PLAYER_REGEN_CPP_PATH, 0x1a9d);
        return 0;
    }
    let mut z = exe::zopen(save, 0xa701a4usize, 0, 0x16, 4); // "retire.dat"
    if z == 0 {
        show_error_box(PLAYER_REGEN_CPP_PATH, 0x1aa5);
        return 0;
    }
    covered!("sx16_11 load retire.dat");
    for (table, count, capacity, size, _, [row_count, row_alloc, row_records]) in REGEN_TABLES {
        let fail = |row: u32, z: &mut usize| {
            show_error_box(PLAYER_REGEN_CPP_PATH, row);
            exe::zclose(z);
            0u32
        };
        if exe::zread(this + count, 4, 1, z) != 1 {
            return fail(row_count, &mut z);
        }
        let room = read_u32(this + count).wrapping_add(REGEN_TABLE_SLACK);
        write_u32(this + capacity, room);
        let records = exe::crt_realloc(read_usize(this + table), room.wrapping_mul(size));
        write_usize(this + table, records);
        if records == 0 {
            return fail(row_alloc, &mut z);
        }
        let n = read_u32(this + count);
        if n != 0 && exe::zread(records, size, n, z) != n {
            return fail(row_records, &mut z);
        }
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `load_retire_dat`: it reads the save item and reallocates, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_load_retire_dat(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, save)
}
