// --- sx01d_4: Media::save_media_dat (0x72b5f0, media.cpp) ---

/// Writes the media object into the save's "media.dat": the transfer interest count (`media_fields::INTEREST_COUNT`),
/// its 25-byte records, the story log count (`STORY_LOG_COUNT`) and its 23-byte records; the item closed, 1. Without a
/// save (line 0x112), without the item (0x11a) or on a short write (0x121, 0x129, 0x130, 0x138) the media.cpp "Error"
/// box and 0, with the item left open (the original's leak).
/// Original: `Media::save_media_dat` (0x72b5f0, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_media_dat(this: usize, save: usize) -> u8 {
    const MEDIA_CPP_PATH: usize = 0xa17bec; // "E:\dev\CM3\cm3 00-01\cm3\code\media.cpp"
    let fail = |row: u32| {
        show_error_box(MEDIA_CPP_PATH, row);
        0u8
    };
    if save == 0 {
        return fail(0x112);
    }
    let mut z = exe::zopen(save, 0xa17c14usize, 1, 0x16, 4); // "media.dat"
    if z == 0 {
        return fail(0x11a);
    }
    covered!("sx01d_4 save media.dat");
    let interest_count = this + media_fields::INTEREST_COUNT;
    if exe::zwrite(interest_count, 4, 1, z) != 1 {
        return fail(0x121);
    }
    if exe::zwrite(read_usize(this + media_fields::TRANSFER_INTERESTS), 0x19, read_u32(interest_count), z) != read_u32(interest_count) {
        return fail(0x129);
    }
    let log_count = this + media_fields::STORY_LOG_COUNT;
    if exe::zwrite(log_count, 4, 1, z) != 1 {
        return fail(0x130);
    }
    if exe::zwrite(read_usize(this + media_fields::STORY_LOG), 0x17, read_u32(log_count), z) != read_u32(log_count) {
        return fail(0x138);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `save_media_dat`: it writes the save item, so it cannot run twice; only the original runs. The copy is
/// proven by the save proof (the .qst written by the original and by the copy, item by item).
pub(crate) unsafe extern "thiscall" fn compare_save_media_dat(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I0%%)(this, save)
}
