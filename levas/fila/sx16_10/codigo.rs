// --- sx16_10: the officials manager: OfficialsManager::OfficialsManager (0x79b580), ~OfficialsManager (0x79b800),
// load_official_stats (0x79b850) ---

/// The save item "off.dat".
const OFF_DAT: usize = 0xa6a260;
/// The game data version from which off.dat holds the 17-byte statistics rows (before it, 16-byte rows).
const OFF_DAT_17_BYTES_VERSION: i32 = 0x3e043;

/// The harshness code of an official from his discipline (`official::DISCIPLINE`, signed): above 18 harsh (1), above 12
/// strict (2), above 5 fair (3), otherwise lenient (4).
unsafe fn official_harshness(official: usize) -> u8 {
    match read_i8(official + official::DISCIPLINE) {
        d if d > 0x12 => 1,
        d if d > 0xc => 2,
        d if d > 5 => 3,
        _ => 4,
    }
}

/// Builds the officials manager: first the game CD (`game_cd_check`, item sx16_7); with a save whose off.dat reads
/// (`OfficialsManager::load_official_stats` through the exe), that is all. Otherwise the officials are set up
/// (`OfficialsManager::set_up_officials` through the exe) and a fresh 17-byte statistics row is made for each official
/// (`malloc`, `G_OFFICIAL_COUNT` rows): his id, the average grade, fouls, grade sum, matches and cards 0, the harshness
/// by his discipline (`official_harshness`) and, without a save, the last grade 0xff (none).
/// Defect of the original: when a save is given but its off.dat does not read, the rows are made without the last
/// grade (`official_stats::LAST_GRADE` keeps what `malloc` left).
/// Original: `OfficialsManager::OfficialsManager` (0x79b580, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_officials_manager(this: usize, save: usize) -> usize {
    game_cd_check(0x79b5c2, 0x79b604);
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x79b850usize); // load_official_stats
        if load(this, save) != 0 {
            covered!("sx16_10 officials from the save");
            return this;
        }
    }
    covered!("sx16_10 fresh officials");
    let set_up: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x79bf60usize); // set_up_officials
    set_up(this);
    let count = read_i32(G_OFFICIAL_COUNT);
    let rows = exe::crt_malloc((count as u32).wrapping_mul(OFFICIAL_STATS_SIZE as u32));
    write_usize(this + officials_manager_fields::GRADE_ROWS, rows);
    for k in 0..count.max(0) {
        let row = read_usize(this + officials_manager_fields::GRADE_ROWS) + k as usize * OFFICIAL_STATS_SIZE;
        write_i32(row, k); // the official's id
        write_u32(row + official_stats::AVERAGE_GRADE, 0);
        write_u16(row + official_stats::MATCHES, 0);
        write_u8(row + official_stats::CARDS, 0);
        write_u16(row + official_stats::GRADE_SUM, 0);
        write_u16(row + official_stats::FOULS, 0);
        if save == 0 {
            write_u8(row + official_stats::LAST_GRADE, 0xff);
        }
        write_u8(row + 0xe, official_harshness(official_record(k))); // the harshness code
    }
    this
}

/// Comparer for `new_officials_manager`: it asks for the CD, allocates and may read the save; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_new_officials_manager(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Frees the officials manager's buffers: the busy flags, the nation blocks and the statistics rows, each present one
/// freed and its pointer zeroed.
/// Original: `OfficialsManager::~OfficialsManager` (0x79b800, officials_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_officials_manager(this: usize) {
    for field in [officials_manager_fields::BUSY_FLAGS, officials_manager_fields::NATION_BLOCKS, officials_manager_fields::GRADE_ROWS] {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
            write_usize(this + field, 0);
        }
    }
}

/// Comparer for `drop_officials_manager`: it frees, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_officials_manager(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Converts the 16-byte statistics rows of an old off.dat into the 17-byte rows: the id, the average grade, the fouls,
/// the grade sum, the matches, the harshness and the cards copied, the last grade 0xff.
unsafe fn officials_convert_old_rows(this: usize, old: usize, count: i32) {
    let rows = read_usize(this + officials_manager_fields::GRADE_ROWS);
    for k in 0..count.max(0) as usize {
        let (o, n) = (old + k * 0x10, rows + k * OFFICIAL_STATS_SIZE);
        write_u32(n, read_u32(o));
        write_u32(n + official_stats::AVERAGE_GRADE, read_u32(o + 4));
        write_u16(n + official_stats::GRADE_SUM, read_u16(o + 0xa));
        write_u16(n + official_stats::MATCHES, read_u16(o + 0xc));
        write_u8(n + 0xe, read_u8(o + 0xe)); // the harshness code
        write_u8(n + official_stats::CARDS, read_u8(o + 0xf));
        write_u16(n + official_stats::FOULS, read_u16(o + 8));
        write_u8(n + official_stats::LAST_GRADE, 0xff);
    }
}

/// Reads off.dat from a saved game: nothing (0) without a save or without the item. The official count
/// (`officials_manager_fields::BUSY_COUNT`; 0 answers 0); the statistics rows: 17 bytes from game data version 0x3e043
/// (absolute `G_GAME_DATA_VERSION`), before it 16-byte rows converted (`officials_convert_old_rows`); the nation count
/// and the 0x102-byte nation blocks; the busy flags (`calloc`, one byte per official; no memory: the fatal box at
/// line 0x125). Closes the item and answers 1; a short read or a failed allocation closes it and answers 0 (no box).
/// Defects of the original: the 17-byte rows' allocation and the nation blocks' are not checked; the old rows' buffer
/// leaks when their read fails.
/// Original: `OfficialsManager::load_official_stats` (0x79b850, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_official_stats(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, OFF_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx16_10 load off.dat");
    let fail = |z: &mut usize| {
        exe::zclose(z);
        0u8
    };
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        return fail(&mut z);
    }
    write_i32(this + officials_manager_fields::BUSY_COUNT, count);
    if count == 0 {
        return fail(&mut z);
    }
    let n = count as u32;
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < OFF_DAT_17_BYTES_VERSION {
        covered!("sx16_10 old off.dat rows");
        let old = exe::crt_malloc(n.wrapping_mul(0x10));
        if old == 0 || exe::zread(old, 0x10, n, z) != n {
            return fail(&mut z);
        }
        let rows = exe::crt_malloc(n.wrapping_mul(OFFICIAL_STATS_SIZE as u32));
        write_usize(this + officials_manager_fields::GRADE_ROWS, rows);
        if rows == 0 {
            return fail(&mut z);
        }
        officials_convert_old_rows(this, old, count);
        exe::crt_free(old);
    } else {
        let rows = exe::crt_malloc(n.wrapping_mul(OFFICIAL_STATS_SIZE as u32));
        write_usize(this + officials_manager_fields::GRADE_ROWS, rows);
        if exe::zread(rows, OFFICIAL_STATS_SIZE as u32, n, z) != n {
            return fail(&mut z);
        }
    }
    let mut nations = 0u32;
    if exe::zread(&mut nations as *mut u32 as usize, 4, 1, z) != 1 {
        return fail(&mut z);
    }
    let blocks = exe::crt_malloc(nations.wrapping_mul(0x102));
    write_usize(this + officials_manager_fields::NATION_BLOCKS, blocks);
    if exe::zread(blocks, 0x102, nations, z) != nations {
        return fail(&mut z);
    }
    let busy = crt_calloc(read_u32(this + officials_manager_fields::BUSY_COUNT), 1);
    write_usize(this + officials_manager_fields::BUSY_FLAGS, busy);
    if busy == 0 {
        officials_out_of_memory(0x125);
    }
    if exe::zread(busy, 1, n, z) != n {
        return fail(&mut z);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `load_official_stats`: it reads the save item and allocates, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_load_official_stats(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(this, save)
}
