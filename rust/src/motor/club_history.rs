use super::*;


/// Opens an item named nome in the save archive (FN_ZOPEN with 1, 0x16, 4).
pub(crate) unsafe fn zip_open_item(save: usize, nome: usize) -> usize {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    abrir_item(save, nome, 1, 0x16, 4)
}

/// Writes count items of size bytes to an open archive item (FN_ZWRITE).
pub(crate) unsafe fn zip_write(dados: usize, tamanho: u32, quantos: u32, z: usize) -> u32 {
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    gravar_item(dados, tamanho, quantos, z)
}

/// Closes an open archive item (FN_ZCLOSE).
pub(crate) unsafe fn zip_close(z: &mut usize) {
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    fechar_item(z);
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
    crate::comparacao::ramo("cpx_7 gravar historico dos clubes");
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
    if zip_write(read_usize(this + 8), 0x10, read_usize(this + 0xc) as u32, z) != read_usize(this + 0xc) as u32 { // cpx7: os registros do histórico
        fatal_error_at(FONTE, 0x340);
    }
    zip_close(&mut z);
    1
}


/// Shows the officials_manager.cpp error box at the line, closes the item and returns 0.
///
/// [pt] A caixa de erro do officials_manager.cpp na linha `linha`, fechando o item; devolve 0.
pub(crate) unsafe fn officials_save_error(linha: u32, z: &mut usize) -> u8 {
    show_error_at(0xa6a22c, linha);
    zip_close(z);
    0
}



/// writes the count (+0) and the 0x44-byte records from [+4] into the simulated_stats.dat item; on error the "Error"
/// box without closing the item
/// Original: `SimulatedStats::save` (0x84c680, simulated_stats.cpp, thiscall).
///
/// [pt] { 0x84c680 } (thiscall, ret 4, simulated_stats.cpp): `SimulatedStats::save(save)`: grava no item
/// `simulated_stats.dat` a quantidade (+0, u32) e os registros de 0x44 bytes de [+4]. Erro ao abrir ou gravar: a caixa
/// "Error" (simulated_stats.cpp 0x209, 0x210, 0x218), sem fechar o item. Não confere o contêiner nulo.
pub(crate) unsafe extern "thiscall" fn club_history_save_84c680(this: usize, save: usize) {
    const FONTE: usize = 0xa8278c;
    crate::comparacao::ramo("cpx_7 gravar estatisticas simuladas");
    let mut z = zip_open_item(save, 0xa827c0);
    if z == 0 {
        show_error_at(FONTE, 0x209);
        return;
    }
    if zip_write(this, 4, 1, z) != 1 {
        show_error_at(FONTE, 0x210);
        return;
    }
    if zip_write(read_usize(this + 4), 0x44, read_usize(this) as u32, z) != read_usize(this) as u32 { // cpx7: os registros simulados
        show_error_at(FONTE, 0x218);
        return;
    }
    zip_close(&mut z);
}


