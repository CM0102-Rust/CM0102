use super::*;

/// Opens an item named nome in the save archive (FN_ZOPEN with 1, 0x16, 4).
pub(crate) unsafe fn zip_open_item(save: usize, name: usize) -> usize {
    let open_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    open_item(save, name, 1, 0x16, 4)
}

/// Writes count items of size bytes to an open archive item (FN_ZWRITE).
pub(crate) unsafe fn zip_write(data: usize, size: u32, how_many: u32, z: usize) -> u32 {
    let write_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    write_item(data, size, how_many, z)
}

/// Closes an open archive item (FN_ZCLOSE).
pub(crate) unsafe fn zip_close(z: &mut usize) {
    let close_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    close_item(z);
}

/// writes the count (+0xc) and the 0x10-byte records from [+8] into the "club history.dat" item and returns 1; no
/// container the "Error" box and 0; on open or write error the box and the game stops
/// Original: `Club_History::save` (0x443f50, club_history.cpp, thiscall).
///
/// [pt] { 0x443f50 } (thiscall, ret 4, club_history.cpp): `Club_History::save(save)`: grava no item `club history.dat` a
/// quantidade (+0xc, u32) e os registros de 0x10 bytes de [+8], e devolve 1. Sem contêiner: a caixa "Error"
/// (club_history.cpp 0x329) e 0; erro ao abrir ou gravar: a caixa (0x331, 0x338, 0x340) e o jogo para (`fatal_error_at`).
pub(crate) unsafe extern "thiscall" fn club_history_save(this: usize, save: usize) -> u32 {
    const FONTE: usize = 0x98b644;
    crate::comparison::branch("cpx_7 gravar historico dos clubes");
    if save == 0 {
        show_error_at(FONTE, 0x329);
        return 0;
    }
    let mut z = zip_open_item(save, 0x98b68c);
    if z == 0 {
        fatal_error_at(FONTE, 0x331);
    }
    if zip_write(this + 0xc, 4, 1, z) != 1 {
        fatal_error_at(FONTE, 0x338);
    }
    if zip_write(read_usize(this + 8), 0x10, read_usize(this + 0xc) as u32, z) != read_usize(this + 0xc) as u32 { // cpx7: the history records
        fatal_error_at(FONTE, 0x340);
    }
    zip_close(&mut z);
    1
}

/// Shows the officials_manager.cpp error box at the line, closes the item and returns 0.
///
/// [pt] A caixa de erro do officials_manager.cpp na linha `linha`, fechando o item; devolve 0.
pub(crate) unsafe fn officials_save_error(row: u32, z: &mut usize) -> u8 {
    show_error_at(0xa6a22c, row);
    zip_close(z);
    0
}

