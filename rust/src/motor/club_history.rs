use super::*;

/// Opens an item named nome in the save archive (FN_ZOPEN with 1, 0x16, 4).
pub(crate) unsafe fn zip_open_item(save: usize, name: usize) -> usize {
    exe::zopen(save, name, 1, 0x16, 4)
}

/// Writes count items of size bytes to an open archive item (FN_ZWRITE).
pub(crate) unsafe fn zip_write(data: usize, size: u32, how_many: u32, z: usize) -> u32 {
    exe::zwrite(data, size, how_many, z)
}

/// Closes an open archive item (FN_ZCLOSE).
pub(crate) unsafe fn zip_close(z: &mut usize) {
    exe::zclose(z);
}

/// writes the count (+0xc) and the 0x10-byte records from [+8] into the "club history.dat" item and returns 1; no
/// container the "Error" box and 0; on open or write error the box and the game stops
/// Original: `Club_History::save` (0x443f50, club_history.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn club_history_save(this: usize, save: usize) -> u32 {
    const FONTE: usize = 0x98b644;
    covered!("cpx_7 save club history");
    if save == 0 {
        show_error_at(FONTE, 0x329);
        return 0;
    }
    let mut z = zip_open_item(save, 0x98b68c);
    if z == 0 {
        fatal_error_at(FONTE, 0x331);
    }
    if zip_write(this + club_history_fields::RECORD_COUNT, 4, 1, z) != 1 {
        fatal_error_at(FONTE, 0x338);
    }
    if zip_write(read_usize(this + 8), 0x10, read_usize(this + club_history_fields::RECORD_COUNT) as u32, z) != read_usize(this + club_history_fields::RECORD_COUNT) as u32 { // cpx7: the history records
        fatal_error_at(FONTE, 0x340);
    }
    zip_close(&mut z);
    1
}

/// Shows the officials_manager.cpp error box at the line, closes the item and returns 0.
pub(crate) unsafe fn officials_save_error(row: u32, z: &mut usize) -> u8 {
    show_error_at(0xa6a22c, row);
    zip_close(z);
    0
}

