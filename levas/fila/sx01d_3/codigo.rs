// --- sx01d_3: Media::Media (0x72b1d0), Media::~Media (0x72b200) and Media::load_media_dat (0x72b240), media.cpp ---

/// The source path of media.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\media.cpp").
const MEDIA_CPP_PATH: usize = 0xa17bec;
/// The save item "media.dat".
const MEDIA_DAT: usize = 0xa17c14;
/// `compare_id_pair_keys` (0x757ea0, cdecl), the order of the transfer interest table.
const MEDIA_INTEREST_ORDER: usize = 0x757ea0;
/// `compare_story_log_records` (0x757ee0, cdecl), the order of the story log.
const MEDIA_STORY_LOG_ORDER: usize = 0x757ee0;
/// The game data version from which the story log records are 23 bytes (before it, 22).
const MEDIA_LOG_23_BYTES_VERSION: i32 = 0x3e037;

/// Builds the media object: the story log and the transfer interest table empty (pointers, capacities and counts at
/// 0), then, with a save, `load_media_dat` reads them from it (its answer is not used). Returns the object.
/// Original: `Media::Media` (0x72b1d0, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_media(this: usize, save: usize) -> usize {
    for field in [media_fields::TRANSFER_INTERESTS, media_fields::INTEREST_CAPACITY, media_fields::INTEREST_COUNT,
                  media_fields::STORY_LOG, media_fields::STORY_LOG_CAPACITY, media_fields::STORY_LOG_COUNT] {
        write_u32(this + field, 0);
    }
    if save != 0 {
        covered!("sx01d_3 media from the save");
        load_media_dat(this, save);
    }
    this
}

/// Comparer for `new_media`: with a save it reads the save item and allocates, so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_new_media(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Frees the media object's tables: the transfer interest table and the story log (`free`), each present one freed and
/// its pointer zeroed (the capacities and counts stay).
/// Original: `Media::~Media` (0x72b200, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_media(this: usize) {
    for field in [media_fields::TRANSFER_INTERESTS, media_fields::STORY_LOG] {
        let table = read_usize(this + field);
        if table != 0 {
            exe::crt_free(table);
            write_usize(this + field, 0);
        }
    }
}

/// Comparer for `drop_media`: it frees, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_media(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Converts the old 22-byte story log records (game data before `MEDIA_LOG_23_BYTES_VERSION`) into the 23-byte
/// `story_log_entry`: the date (`Date::operator=`), +8 and +0xc as they are, the byte +0x10, the story code +0x11 from
/// a byte to a word (sign-extended) and the dword +0x12 to +0x13.
unsafe fn media_convert_old_story_log(this: usize, old: usize) {
    let log = read_usize(this + media_fields::STORY_LOG);
    for k in 0..read_i32(this + media_fields::STORY_LOG_CAPACITY).max(0) as usize {
        let (o, n) = (old + k * 0x16, log + k * 0x17);
        write_u32(n + 0xc, read_u32(o + 0xc));
        let mut date = [0u8; 8];
        exe::date_operator_3d(n, date.as_mut_ptr() as usize, o);
        write_i16(n + 0x11, read_i8(o + 0x11) as i16);
        write_u32(n + 0x13, read_u32(o + 0x12));
        write_u8(n + 0x10, read_u8(o + 0x10));
        write_u32(n + 8, read_u32(o + 8));
    }
}

/// Reads the media object from the save's "media.dat": nothing (0) without a save or the item. The transfer interest
/// count (`media_fields::INTEREST_COUNT`, also the capacity), the 25-byte transfer interest records (`malloc`) sorted
/// with `compare_id_pair_keys`, the story log count (`STORY_LOG_COUNT`, also the capacity) and its records: 23 bytes,
/// or 22 bytes converted (`media_convert_old_story_log`) when `G_GAME_DATA_VERSION` (in absolute value) is below
/// 0x3e037; sorted with `compare_story_log_records`. The item closed, 1. A short read or a failed allocation (with a
/// count) shows the media.cpp "Error" box (lines 0xa5, 0xb1, 0xb9, 0xc4, 0xcf, 0xd8, 0xdf, 0xf5) and answers 0 with the
/// item left open (the original's leak).
/// Original: `Media::load_media_dat` (0x72b240, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_media_dat(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, MEDIA_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx01d_3 load media.dat");
    let fail = |row: u32| {
        show_error_box(MEDIA_CPP_PATH, row);
        0u8
    };
    let count = this + media_fields::INTEREST_COUNT;
    if exe::zread(count, 4, 1, z) != 1 {
        return fail(0xa5);
    }
    write_u32(this + media_fields::INTEREST_CAPACITY, read_u32(count));
    let interests = exe::crt_malloc(read_u32(count).wrapping_mul(0x19));
    write_usize(this + media_fields::TRANSFER_INTERESTS, interests);
    if interests == 0 && read_u32(this + media_fields::INTEREST_CAPACITY) != 0 {
        return fail(0xb1);
    }
    let n = read_u32(count);
    if exe::zread(interests, 0x19, n, z) != n {
        return fail(0xb9);
    }
    exe::crt_qsort(interests, n, 0x19, MEDIA_INTEREST_ORDER);
    let logged = this + media_fields::STORY_LOG_COUNT;
    if exe::zread(logged, 4, 1, z) != 1 {
        return fail(0xc4);
    }
    write_u32(this + media_fields::STORY_LOG_CAPACITY, read_u32(logged));
    let log = exe::crt_malloc(read_u32(logged).wrapping_mul(0x17));
    write_usize(this + media_fields::STORY_LOG, log);
    if log == 0 && read_u32(this + media_fields::STORY_LOG_CAPACITY) != 0 {
        return fail(0xcf);
    }
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < MEDIA_LOG_23_BYTES_VERSION {
        covered!("sx01d_3 old story log");
        let old = exe::crt_malloc(read_u32(this + media_fields::STORY_LOG_CAPACITY).wrapping_mul(0x16));
        if old == 0 && read_u32(this + media_fields::STORY_LOG_CAPACITY) != 0 {
            return fail(0xd8);
        }
        if exe::zread(old, 0x16, read_u32(logged), z) != read_u32(logged) {
            return fail(0xdf);
        }
        media_convert_old_story_log(this, old);
        exe::crt_free(old);
    } else if exe::zread(log, 0x17, read_u32(logged), z) != read_u32(logged) {
        return fail(0xf5);
    }
    exe::crt_qsort(read_usize(this + media_fields::STORY_LOG), read_u32(logged), 0x17, MEDIA_STORY_LOG_ORDER);
    exe::zclose(&mut z);
    1
}

/// Comparer for `load_media_dat`: it reads the save item and allocates, so it cannot run twice; only the original
/// runs. The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_load_media_dat(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(this, save)
}
