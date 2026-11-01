// ---------- 265a leva: o save das finanças e do gerente de elencos: gravar e carregar o finance.dat e gravar o squad_manager.dat (fin_prm.cpp, squad_manager.cpp) ----------

unsafe fn cpx5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx5_clubes() -> u32 {
    *(0xae235c as *const u32)
}

/// { 0x59b3b0 } (thiscall, ret 4, fin_prm.cpp; o fonte se chama finance.CPP): `FinanceTable::save_finance_dat(save)`:
/// grava no item `finance.dat` (0x9326e0 gravando) as finanças de todos os clubes, 0x167 bytes × [0xae235c] a partir
/// de [this]. Sem contêiner, nada; erro ao abrir ou ao gravar: a caixa "Error" (finance.CPP 0x115e, 0x1166), sem
/// fechar o item (como a original).
unsafe extern "thiscall" fn gravar_financas(this: usize, save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("cpx_5 gravar financas");
    if save == 0 {
        return;
    }
    let mut z = abrir_item(save, 0x9c87b0, 1, 0x16, 4);
    if z == 0 {
        aviso_em(0x9c8784, 0x115e);
        return;
    }
    if gravar_item(cpx5_d(this), 0x167, cpx5_clubes(), z) != cpx5_clubes() { // cpx5: os registros das finanças
        aviso_em(0x9c8784, 0x1166);
        return;
    }
    fechar_item(&mut z);
}

/// Comparador da 0x59b3b0: só a original.
unsafe extern "thiscall" fn comparar_gravar_financas(this: usize, save: usize) {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    // (o .qst gravado no modo rust com e sem a leva, item a item)
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1497)(this, save)
}

/// { 0x59b2a0 } (thiscall, ret 4, fin_prm.cpp): `FinanceTable::load_from_save(save)`: lê do item `finance.dat`
/// (0x9326e0 lendo) as finanças de todos os clubes, 0x167 bytes × [0xae235c] em [this], e devolve 1. Sem contêiner,
/// 0 sem caixa; erro ao abrir ou leitura curta: a caixa "Error" (finance.CPP 0x1137, 0x113f), 0 e o item fica aberto
/// (como a original). Com 0, o construtor (0x59a55c) refaz as finanças de cada clube do zero (0x5961d0).
unsafe extern "thiscall" fn carregar_financas(this: usize, save: usize) -> u32 {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let ler_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("cpx_5 carregar financas");
    if save == 0 {
        return 0;
    }
    let mut z = abrir_item(save, 0x9c87b0, 0, 0x16, 4);
    if z == 0 {
        aviso_em(0x9c8784, 0x1137);
        return 0;
    }
    if ler_item(cpx5_d(this), 0x167, cpx5_clubes(), z) != cpx5_clubes() {
        aviso_em(0x9c8784, 0x113f);
        return 0;
    }
    fechar_item(&mut z);
    1 // cpx5: carregou (o construtor não refaz as finanças)
}

/// Comparador da 0x59b2a0: só a original.
unsafe extern "thiscall" fn comparar_carregar_financas(this: usize, save: usize) -> u32 {
    // ponytail: lê do contêiner (a posição do item anda; não pode rodar duas vezes); a cópia é provada pela carga
    // do save de férias na prova do save (levas/N/CARGA)
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1498)(this, save)
}

/// { 0x859cc0 } (thiscall, ret 4, squad_manager.cpp): `SQUAD_MANAGER::save(save)`: grava no item `squad_manager.dat`
/// o u16 em +0 do gerente de elencos, estendido a 4 bytes (a original zera eax antes do `mov ax`), e devolve 1. Erro
/// ao abrir ou ao gravar: a caixa "Error" (squad_manager.cpp 0x68, 0x6f) e 0, sem fechar o item. Não confere o
/// contêiner nulo (o 0x9326e0 recebe como vier).
unsafe extern "thiscall" fn gravar_gerente_de_elencos(this: usize, save: usize) -> u32 {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("cpx_5 gravar gerente de elencos");
    let mut v = *(this as *const u16) as u32; // cpx5: o valor do gerente de elencos
    let mut z = abrir_item(save, 0xa82bc0, 1, 0x16, 4);
    if z == 0 {
        aviso_em(0xa82b88, 0x68);
        return 0;
    }
    if gravar_item(&mut v as *mut u32 as usize, 4, 1, z) != 1 {
        aviso_em(0xa82b88, 0x6f);
        return 0;
    }
    fechar_item(&mut z);
    1
}

/// Comparador da 0x859cc0: só a original.
unsafe extern "thiscall" fn comparar_gravar_gerente_de_elencos(this: usize, save: usize) -> u32 {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1499)(this, save)
}

// ---------- 265a leva: os temporários do save: as sementes da UEFA e da CONMEBOL e os países anfitriões (uefa_seeding.cpp, conmebol_seeding.cpp, host_country.cpp) ----------

unsafe fn cpx6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A contagem de um byte com sinal (a original usa `movsx`).
unsafe fn cpx6_n8(a: usize) -> u32 {
    *(a as *const i8) as i32 as u32
}

/// Abre `CM3_TEMP\<nome>` para gravar ("wb"): o caminho pela 0x90bde0 e o fopen do jogo (0x945b00).
unsafe fn cpx6_abrir(nome: usize) -> usize {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    abrir(caminho(0x987da0, nome), 0x9a7f28)
}

unsafe fn cpx6_escrever(dados: usize, tamanho: u32, quantos: u32, f: usize) -> u32 {
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    escrever(dados, tamanho, quantos, f)
}

unsafe fn cpx6_fechar(f: usize) {
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    fechar(f);
}

/// { 0x905510 } (thiscall, ret 0, uefa_seeding.cpp): `UefaSeeding::save_to_temp_file`: grava `CM3_TEMP\uefa_seeding.tmp`:
/// a quantidade de países (+0x438, i8), os registros de 0x12 bytes de +0, +0x439 (u16), a quantidade +0x43f (i8) e, se
/// não for zero, essa quantidade de u32 de [+0x43b], e os 50 registros fixos de 0xc bytes de +0x440. Erro ao abrir ou
/// gravar: a caixa "Error" (uefa_seeding.cpp 0x64f, 0x657, 0x660, 0x668, 0x66f, 0x679, 0x682), sem fechar o arquivo
/// (como a original).
unsafe extern "thiscall" fn gravar_sementes_da_uefa(this: usize) {
    const FONTE: usize = 0xaa1a1c;
    crate::comparacao::ramo("cpx_6 gravar sementes da uefa");
    let f = cpx6_abrir(0xaa1a50);
    if f == 0 {
        aviso_em(FONTE, 0x64f);
        return;
    }
    if cpx6_escrever(this + 0x438, 1, 1, f) != 1 {
        aviso_em(FONTE, 0x657);
        return;
    }
    if cpx6_escrever(this, 0x12, cpx6_n8(this + 0x438), f) != cpx6_n8(this + 0x438) {
        aviso_em(FONTE, 0x660);
        return;
    }
    if cpx6_escrever(this + 0x439, 2, 1, f) != 1 {
        aviso_em(FONTE, 0x668);
        return;
    }
    if cpx6_escrever(this + 0x43f, 1, 1, f) != 1 {
        aviso_em(FONTE, 0x66f);
        return;
    }
    if *((this + 0x43f) as *const u8) != 0 && cpx6_escrever(cpx6_d(this + 0x43b), 4, cpx6_n8(this + 0x43f), f) != cpx6_n8(this + 0x43f) {
        aviso_em(FONTE, 0x679);
        return;
    }
    if cpx6_escrever(this + 0x440, 0xc, 0x32, f) != 0x32 { // cpx6: os 50 registros fixos
        aviso_em(FONTE, 0x682);
        return;
    }
    cpx6_fechar(f);
}

/// Comparador da 0x905510: só a original.
unsafe extern "thiscall" fn comparar_gravar_sementes_da_uefa(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    // (o .tmp vai no .qst pelo zadd_matching_files)
    orig::<unsafe extern "thiscall" fn(usize)>(1500)(this)
}

/// { 0x4c6b70 } (thiscall, ret 0, conmebol_seeding.cpp): `ConmebolSeeding::save_to_temp_file`: grava
/// `CM3_TEMP\conmebol_seeding.tmp`: a quantidade de países (+0x6e, i8), os registros de 0xa bytes de +0, +0x6f (u16) e
/// +0x71 (u32). Erro: a caixa "Error" (conmebol_seeding.cpp 0x2ee, 0x2f6, 0x2ff, 0x307, 0x30f), sem fechar o arquivo.
unsafe extern "thiscall" fn gravar_sementes_da_conmebol(this: usize) {
    const FONTE: usize = 0x9a7ed8;
    crate::comparacao::ramo("cpx_6 gravar sementes da conmebol");
    let f = cpx6_abrir(0x9a7f10);
    if f == 0 {
        aviso_em(FONTE, 0x2ee);
        return;
    }
    if cpx6_escrever(this + 0x6e, 1, 1, f) != 1 {
        aviso_em(FONTE, 0x2f6);
        return;
    }
    if cpx6_escrever(this, 0xa, cpx6_n8(this + 0x6e), f) != cpx6_n8(this + 0x6e) { // cpx6: os registros dos países
        aviso_em(FONTE, 0x2ff);
        return;
    }
    if cpx6_escrever(this + 0x6f, 2, 1, f) != 1 {
        aviso_em(FONTE, 0x307);
        return;
    }
    if cpx6_escrever(this + 0x71, 4, 1, f) != 1 {
        aviso_em(FONTE, 0x30f);
        return;
    }
    cpx6_fechar(f);
}

/// Comparador da 0x4c6b70: só a original.
unsafe extern "thiscall" fn comparar_gravar_sementes_da_conmebol(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize)>(1501)(this)
}

/// { 0x5fb320 } (thiscall, ret 0, host_country.cpp): `HostCountry::save_temp_file`: grava `CM3_TEMP\HostCountry.tmp`:
/// a quantidade (+4, i16) e os registros de 0x22 bytes de [+0], sem conferir a gravação. Erro ao abrir: a caixa
/// "Error" (host_country.cpp 0x475).
unsafe extern "thiscall" fn gravar_paises_anfitrioes(this: usize) {
    crate::comparacao::ramo("cpx_6 gravar paises anfitrioes");
    let f = cpx6_abrir(0x9cf10c);
    if f == 0 {
        aviso_em(0x9cf0d8, 0x475);
        return;
    }
    cpx6_escrever(this + 4, 2, 1, f);
    cpx6_escrever(cpx6_d(this), 0x22, *((this + 4) as *const i16) as i32 as u32, f); // cpx6: os anfitriões
    cpx6_fechar(f);
}

/// Comparador da 0x5fb320: só a original.
unsafe extern "thiscall" fn comparar_gravar_paises_anfitrioes(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize)>(1502)(this)
}

// ---------- 265a leva: o save dos históricos: o histórico dos clubes, as estatísticas dos árbitros e as estatísticas simuladas (club_history.cpp, officials_manager.cpp, simulated_stats.cpp) ----------

unsafe fn cpx7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

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
    if cpx7_gravar(cpx7_d(this + 8), 0x10, cpx7_d(this + 0xc) as u32, z) != cpx7_d(this + 0xc) as u32 { // cpx7: os registros do histórico
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
    if cpx7_gravar(cpx7_d(this), 0x11, arbitros(), z) != arbitros() {
        return cpx7_falha_dos_arbitros(0x15c, &mut z);
    }
    if cpx7_gravar(0xae2350, 4, 1, z) != 1 {
        return cpx7_falha_dos_arbitros(0x164, &mut z);
    }
    if cpx7_gravar(cpx7_d(this + 0xc), 0x102, nacoes(), z) != nacoes() { // cpx7: os registros das nações
        return cpx7_falha_dos_arbitros(0x16c, &mut z);
    }
    if cpx7_gravar(cpx7_d(this + 8), 1, arbitros(), z) != arbitros() {
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
    if cpx7_gravar(cpx7_d(this + 4), 0x44, cpx7_d(this) as u32, z) != cpx7_d(this) as u32 { // cpx7: os registros simulados
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

// ---------- 265a leva: o save do calendário: gravar e carregar as datas dos clubes e gravar a conferência das pernas (fix_man.cpp) ----------

unsafe fn cpy7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy7_clubes() -> u32 {
    *(0xae235c as *const u32)
}

/// Abre `nome` em CM3_TEMP (0x90bde0: cdecl(pasta, nome); fopen 0x945b00: cdecl(caminho, modo)).
unsafe fn cpy7_abrir(nome: usize, modo: usize) -> usize {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    abrir(caminho(0x987da0, nome), modo)
}

unsafe fn cpy7_fechar(f: usize) {
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    fechar(f);
}

/// { 0x5af040 } (cdecl, fix_man.cpp): `fix_man_save_club_dates()`, do save do calendário (FIXTURE_MANAGER::save
/// 0x5ac060, 0x5ac394): sem a tabela das datas dos clubes ([0xae2a5c]), a caixa da linha 0x1011. Grava em
/// CM3_TEMP\club_dates.tmp ("wb") uma palavra por clube ([0xae235c] delas, fwrite 0x945ce6); gravação curta: a caixa
/// da linha 0x101b; fecha. Sem abrir: nada.
unsafe extern "C" fn gravar_datas_dos_clubes() {
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    if cpy7_d(0xae2a5c) == 0 {
        return aviso_em(0x9cc208, 0x1011);
    }
    let f = cpy7_abrir(0x9cc248, 0x9a7f28);
    if f == 0 {
        return;
    }
    crate::comparacao::ramo("cpy_7 gravar datas dos clubes");
    if escrever(cpy7_d(0xae2a5c), 2, cpy7_clubes(), f) != cpy7_clubes() { // cpy7: as datas dos clubes
        aviso_em(0x9cc208, 0x101b);
    }
    cpy7_fechar(f);
}

/// Comparador da 0x5af040: só a original.
unsafe extern "C" fn comparar_gravar_datas_dos_clubes() {
    // ponytail: grava o arquivo do save (não pode rodar duas vezes); a cópia é provada pela prova do save (o .qst
    // gravado no modo rust com e sem a leva, item a item: club_dates.tmp)
    orig::<unsafe extern "C" fn()>(1506)()
}

/// { 0x5aee90 } (cdecl, fix_man.cpp): `fix_man_load_club_dates()`, do construtor do calendário (0x5a9cf0, 0x5a9e38:
/// na carreira nova e na carga do save). Com a tabela já alocada ([0xae2a5c]), a caixa da linha 0xfe4. Aloca uma
/// palavra por clube (malloc 0x944e46 de 2 × [0xae235c]; sem memória, a caixa da linha 0xfec); lê de
/// CM3_TEMP\club_dates.tmp ("rb", fread 0x945bfe; leitura curta: a caixa da linha 0xff6) e fecha; sem o arquivo (a
/// carreira nova), zera a tabela.
unsafe extern "C" fn carregar_datas_dos_clubes() {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    if cpy7_d(0xae2a5c) != 0 {
        return aviso_em(0x9cc208, 0xfe4);
    }
    let t = alocar(cpy7_clubes().wrapping_mul(2));
    *(0xae2a5c as *mut usize) = t;
    if t == 0 {
        return aviso_em(0x9cc208, 0xfec);
    }
    let f = cpy7_abrir(0x9cc248, 0x98f928);
    if f == 0 {
        crate::comparacao::ramo("cpy_7 datas dos clubes zeradas");
        core::ptr::write_bytes(cpy7_d(0xae2a5c) as *mut u8, 0, cpy7_clubes().wrapping_mul(2) as usize);
        return;
    }
    crate::comparacao::ramo("cpy_7 datas dos clubes lidas");
    if ler(cpy7_d(0xae2a5c), 2, cpy7_clubes(), f) != cpy7_clubes() { // cpy7: a leitura das datas
        aviso_em(0x9cc208, 0xff6);
    }
    cpy7_fechar(f);
}

/// Comparador da 0x5aee90: só a original.
unsafe extern "C" fn comparar_carregar_datas_dos_clubes() {
    // ponytail: aloca a tabela global e lê o arquivo (não pode rodar duas vezes); a cópia é provada pela carga do
    // save de férias na prova do save (levas/N/CARGA) e, sem o arquivo, pela carreira nova
    orig::<unsafe extern "C" fn()>(1507)()
}

/// { 0x5af4a0 } (cdecl, fix_man.cpp): `fix_man_save_fixture_leg_checks()`, do save do calendário (0x5ac399): com a
/// lista de conferência das pernas ([0xae2a48], LLIST) vazia (a palavra da conta zero), nada. Grava em
/// CM3_TEMP\check_fixture_legs.tmp ("wb") a conta (2 bytes) e, do começo da lista (o cursor +0xa ← a cabeça +2), cada
/// item de 15 bytes (0x68d1b0; gravação curta: a caixa da linha 0x10e0, e segue); fecha. Sem abrir: nada.
unsafe extern "C" fn gravar_conferencia_das_pernas() {
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let conta = cpy7_d(0xae2a48);
    if conta as u16 == 0 {
        return;
    }
    let f = cpy7_abrir(0x9cc258, 0x9a7f28);
    if f == 0 {
        return;
    }
    crate::comparacao::ramo("cpy_7 gravar conferencia das pernas");
    escrever(&conta as *const usize as usize, 2, 1, f);
    *(0xae2a52 as *mut usize) = cpy7_d(0xae2a4a);
    loop {
        let item = proximo(0xae2a48);
        if item == 0 {
            break;
        }
        if escrever(item, 0xf, 1, f) != 1 {
            aviso_em(0x9cc208, 0x10e0);
        }
    }
    cpy7_fechar(f);
}

/// Comparador da 0x5af4a0: só a original.
unsafe extern "C" fn comparar_gravar_conferencia_das_pernas() {
    // ponytail: grava o arquivo do save e anda o cursor da lista; a cópia é provada pela prova do save
    orig::<unsafe extern "C" fn()>(1508)()
}
