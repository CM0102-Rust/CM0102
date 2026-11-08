use super::*;

// ---------- 265a leva: o save dos históricos: o histórico dos clubes, as estatísticas dos árbitros e as estatísticas simuladas (club_history.cpp, officials_manager.cpp, simulated_stats.cpp) ----------


pub(crate) unsafe fn cpx7_abrir(save: usize, nome: usize) -> usize {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    abrir_item(save, nome, 1, 0x16, 4)
}

pub(crate) unsafe fn cpx7_gravar(dados: usize, tamanho: u32, quantos: u32, z: usize) -> u32 {
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    gravar_item(dados, tamanho, quantos, z)
}

pub(crate) unsafe fn cpx7_fechar(z: &mut usize) {
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    fechar_item(z);
}

/// writes the count (+0xc) and the 0x10-byte records from [+8] into the "club history.dat" item and returns 1; no
/// container the "Error" box and 0; on open or write error the box and the game stops
/// Original: `Club_History::save` (0x443f50, club_history.cpp, thiscall).
///
/// [pt] { 0x443f50 } (thiscall, ret 4, club_history.cpp): `Club_History::save(save)`: grava no item `club history.dat` a
/// quantidade (+0xc, u32) e os registros de 0x10 bytes de [+8], e devolve 1. Sem contêiner: a caixa "Error"
/// (club_history.cpp 0x329) e 0; erro ao abrir ou gravar: a caixa (0x331, 0x338, 0x340) e o jogo para (`erro_em`).
pub(crate) unsafe extern "thiscall" fn club_history_save(this: usize, save: usize) -> u32 {
    const FONTE: usize = 0x98b644;
    crate::comparacao::ramo("cpx_7 gravar historico dos clubes");
    if save == 0 {
        aviso_em(FONTE, 0x329);
        return 0;
    }
    let mut z = cpx7_abrir(save, 0x98b68c);
    if z == 0 {
        erro_em(FONTE, 0x331);
    }
    if cpx7_gravar(this + 0xc, 4, 1, z) != 1 {
        erro_em(FONTE, 0x338);
    }
    if cpx7_gravar(read_usize(this + 8), 0x10, read_usize(this + 0xc) as u32, z) != read_usize(this + 0xc) as u32 { // cpx7: os registros do histórico
        erro_em(FONTE, 0x340);
    }
    cpx7_fechar(&mut z);
    1
}

/// Comparador da 0x443f50: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_club_history_save(this: usize, save: usize) -> u32 {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLUB_HISTORY_SAVE)(this, save)
}

/// A caixa de erro do officials_manager.cpp na linha `linha`, fechando o item; devolve 0.
pub(crate) unsafe fn cpx7_falha_dos_arbitros(linha: u32, z: &mut usize) -> u8 {
    aviso_em(0xa6a22c, linha);
    cpx7_fechar(z);
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
    let mut z = cpx7_abrir(save, 0xa827c0);
    if z == 0 {
        aviso_em(FONTE, 0x209);
        return;
    }
    if cpx7_gravar(this, 4, 1, z) != 1 {
        aviso_em(FONTE, 0x210);
        return;
    }
    if cpx7_gravar(read_usize(this + 4), 0x44, read_usize(this) as u32, z) != read_usize(this) as u32 { // cpx7: os registros simulados
        aviso_em(FONTE, 0x218);
        return;
    }
    cpx7_fechar(&mut z);
}

/// Comparador da 0x84c680: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_club_history_save_84c680(this: usize, save: usize) {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLUB_HISTORY_SAVE_84C680)(this, save)
}

