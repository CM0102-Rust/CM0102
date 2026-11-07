// ---------- 265a leva: o save dos históricos: o histórico dos clubes, as estatísticas dos árbitros e as estatísticas simuladas (club_history.cpp, officials_manager.cpp, simulated_stats.cpp) ----------


unsafe fn cpx7_abrir(save: usize, nome: usize) -> usize {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    abrir_item(save, nome, 1, 0x16, 4)
}

unsafe fn cpx7_gravar(dados: usize, tamanho: u32, quantos: u32, z: usize) -> u32 {
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
    gravar_item(dados, tamanho, quantos, z)
}

unsafe fn cpx7_fechar(z: &mut usize) {
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    fechar_item(z);
}

/// { 0x443f50 } (thiscall, ret 4, club_history.cpp): `Club_History::save(save)`: grava no item `club history.dat` a
/// quantidade (+0xc, u32) e os registros de 0x10 bytes de [+8], e devolve 1. Sem contêiner: a caixa "Error"
/// (club_history.cpp 0x329) e 0; erro ao abrir ou gravar: a caixa (0x331, 0x338, 0x340) e o jogo para (`erro_em`).
unsafe extern "thiscall" fn gravar_historico_dos_clubes(this: usize, save: usize) -> u32 {
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
unsafe extern "thiscall" fn comparar_gravar_historico_dos_clubes(this: usize, save: usize) -> u32 {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1503)(this, save)
}

/// A caixa de erro do officials_manager.cpp na linha `linha`, fechando o item; devolve 0.
unsafe fn cpx7_falha_dos_arbitros(linha: u32, z: &mut usize) -> u8 {
    aviso_em(0xa6a22c, linha);
    cpx7_fechar(z);
    0
}

/// { 0x79bb80 } (thiscall, ret 4, officials_manager.cpp): `OfficialsManager::save_official_stats(save)`: grava no item
/// `off.dat` a quantidade de árbitros [0xae2390], os registros de 0x11 bytes de [+0] (um por árbitro), a quantidade de
/// nações [0xae2350], os registros de 0x102 bytes de [+0xc] (um por nação) e um byte por árbitro de [+8]; devolve 1
/// (no byte baixo). Sem contêiner ou erro ao abrir: a caixa "Error" (officials_manager.cpp 0x142, 0x14a) e 0; sem
/// árbitros ou erro ao gravar: a caixa (0x150, 0x156, 0x15c, 0x164, 0x16c, 0x174), fecha o item e 0.
unsafe extern "thiscall" fn gravar_estatisticas_dos_arbitros(this: usize, save: usize) -> u8 {
    const FONTE: usize = 0xa6a22c;
    let arbitros = || *(0xae2390 as *const u32);
    let nacoes = || *(0xae2350 as *const u32);
    crate::comparacao::ramo("cpx_7 gravar estatisticas dos arbitros");
    if save == 0 {
        aviso_em(FONTE, 0x142);
        return 0;
    }
    let mut z = cpx7_abrir(save, 0xa6a260);
    if z == 0 {
        aviso_em(FONTE, 0x14a);
        return 0;
    }
    if cpx7_gravar(0xae2390, 4, 1, z) != 1 {
        return cpx7_falha_dos_arbitros(0x150, &mut z);
    }
    if arbitros() == 0 {
        return cpx7_falha_dos_arbitros(0x156, &mut z);
    }
    if cpx7_gravar(read_usize(this), 0x11, arbitros(), z) != arbitros() {
        return cpx7_falha_dos_arbitros(0x15c, &mut z);
    }
    if cpx7_gravar(0xae2350, 4, 1, z) != 1 {
        return cpx7_falha_dos_arbitros(0x164, &mut z);
    }
    if cpx7_gravar(read_usize(this + 0xc), 0x102, nacoes(), z) != nacoes() { // cpx7: os registros das nações
        return cpx7_falha_dos_arbitros(0x16c, &mut z);
    }
    if cpx7_gravar(read_usize(this + 8), 1, arbitros(), z) != arbitros() {
        return cpx7_falha_dos_arbitros(0x174, &mut z);
    }
    cpx7_fechar(&mut z);
    1
}

/// Comparador da 0x79bb80: só a original.
unsafe extern "thiscall" fn comparar_gravar_estatisticas_dos_arbitros(this: usize, save: usize) -> u8 {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1504)(this, save)
}

/// { 0x84c680 } (thiscall, ret 4, simulated_stats.cpp): `SimulatedStats::save(save)`: grava no item
/// `simulated_stats.dat` a quantidade (+0, u32) e os registros de 0x44 bytes de [+4]. Erro ao abrir ou gravar: a caixa
/// "Error" (simulated_stats.cpp 0x209, 0x210, 0x218), sem fechar o item. Não confere o contêiner nulo.
unsafe extern "thiscall" fn gravar_estatisticas_simuladas(this: usize, save: usize) {
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
unsafe extern "thiscall" fn comparar_gravar_estatisticas_simuladas(this: usize, save: usize) {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1505)(this, save)
}

