use super::*;

// ---------- 221a leva: o save: carregar o jogo inteiro (load_game_data) (Setup.cpp) ----------

/// A caixa "Error" da carga sem volta (memória para os dados de execução): `aviso_do_setup(linha)`,
/// 0x5cc7d0, 0x5a8f60 e `exit(-1)` (0x945934).
pub(crate) unsafe fn fatal_da_carga(linha: u32) -> ! {
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
    aviso_do_setup(linha);
    fechar1();
    fechar2();
    sair(-1);
    loop {}
}

/// A leitura do `general.dat` que falhou: a caixa "Load Game Error" com a linha e o texto (como a
/// `caixa_do_save`) e o item fechado (0x932b00). Devolve 0, a resposta da carga.
pub(crate) unsafe fn erro_na_carga(linha: u32, texto: usize, z: &mut usize) -> u32 {
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    caixa_do_save(linha, texto, 0x9af3e0, 0x9af334);
    fechar_item(z);
    0
}

/// Os dados de execução das pessoas de um save antigo (registros de `tam` bytes) passados para o
/// registro atual de 0x4f bytes em [0xae2d0c], na ordem da original (relê o global a cada campo):
/// +0, +4, +8, +9, +0xa copiados; +0xb: o bit 7 do antigo e os bits 0x7f do destino (sobra do
/// malloc), depois os bits 0xf, 0x40 e 0x30 do antigo; +0xc: o bit 0 do antigo. `tam` 0x26 (versão <
/// 0x3e042): +0xd = 0 e o bit 1 de +0xc apagado; 0x27 (0x3e042..0x3e044): +0xd do antigo e o bit 1
/// de +0xc do antigo. Os 5 pares (+0x12 + 5j = 0, +0xe + 5j = −1) e os dwords +0x27/+0x2b/+0x2f/
/// +0x33/+0x37 (+0x14j, j = 0, 1) = −1.
pub(crate) unsafe fn converter_dados_das_pessoas(fonte: usize, tam: usize, n: i32) {
    let d = || *(G_STAFF_STATE_TABLE as *const usize);
    let b = |o: usize| *((fonte + o) as *const u8);
    let mesclar = |base: u8, outro: u8, m: u8| base ^ ((base ^ outro) & m);
    for i in 0..n.max(0) as usize {
        let (o, f) = (i * 0x4f, i * tam);
        *((d() + o) as *mut u32) = *((fonte + f) as *const u32);
        let x = d() + o + 0xb;
        *(x as *mut u8) = mesclar(b(f + 0xb), *(x as *const u8), 0x7f);
        *((d() + o + 0xa) as *mut u8) = b(f + 0xa);
        *((d() + o + 4) as *mut u32) = core::ptr::read_unaligned((fonte + f + 4) as *const u32);
        *((d() + o + 9) as *mut u8) = b(f + 9);
        *((d() + o + 8) as *mut u8) = b(f + 8);
        for m in [0xfu8, 0x40, 0x30] {
            let x = d() + o + 0xb;
            *(x as *mut u8) = mesclar(*(x as *const u8), b(f + 0xb), m);
        }
        let c = d() + o + 0xc;
        *(c as *mut u8) = mesclar(*(c as *const u8), b(f + 0xc), 1);
        if tam == 0x26 {
            *((d() + o + 0xd) as *mut u8) = 0;
            *((d() + o + 0xc) as *mut u8) &= 0xfd;
        } else {
            *((d() + o + 0xd) as *mut u8) = b(f + 0xd);
            let c = d() + o + 0xc;
            *(c as *mut u8) = mesclar(*(c as *const u8), b(f + 0xc), 2);
        }
        for j in 0..5 {
            *((d() + o + 0x12 + 5 * j) as *mut u8) = 0;
            core::ptr::write_unaligned((d() + o + 0xe + 5 * j) as *mut u32, 0xffffffff);
        }
        for j in 0..2 {
            for c in [0x33usize, 0x2b, 0x2f, 0x27, 0x37] {
                core::ptr::write_unaligned((d() + o + c + 0x14 * j) as *mut u32, 0xffffffff);
            }
        }
    }
}

/// Como o construtor de um subsistema é chamado (thiscall, `this` = o `new`).
#[derive(Clone, Copy)]
pub(crate) enum ArgDaCarga {
    /// (this, caminho do save)
    Caminho,
    /// (this)
    Sozinho,
    /// (this, caminho, 0x2710): Match_Stats
    Estatisticas,
    /// (this, ano do jogo, 0, 1): FIXTURE_MANAGER (sem o save: a original empurra ebp, que ali é 0;
    /// o construtor lê os jogos da pasta temporária)
    Calendario,
    /// (this, idioma dos nomes [0x9b5984] (byte)): PunditList
    Idioma,
}

/// Um passo da carga depois do `general.dat`: a barra (passo de 0x2d), disco e memória, e
/// `Rotina(f, linha)`: cdecl() que dá erro na `linha` se devolver 0 (linha 0: sem conferência), ou
/// `Objeto(tamanho do new, construtor, global, linha do erro se o global ficar 0, argumentos)`.
#[derive(Clone, Copy)]
pub(crate) enum PassoDaCarga {
    Rotina(usize, u32),
    Objeto(u32, usize, usize, u32, ArgDaCarga),
}

/// Os passos 4..=0x2d da carga, na ordem da original, e o último (PunditList), que repete o 0x2d.
pub(crate) const PASSOS_DA_CARGA: [PassoDaCarga; 0x2b] = [
    PassoDaCarga::Rotina(0x539750, 0),                                      // 4
    PassoDaCarga::Rotina(0x667150, 0x2555),                                 // 5 nações-chave
    PassoDaCarga::Rotina(0x669140, 0x256c),                                 // 6
    PassoDaCarga::Objeto(0x609, 0x893dc0, 0xde1d08, 0x2581, ArgDaCarga::Caminho), // 7 Tactics
    PassoDaCarga::Objeto(0x10, 0x5fb430, 0xb63c98, 0x2596, ArgDaCarga::Caminho),  // 8 HUMAN_MANAGER
    PassoDaCarga::Objeto(0xe8, 0x7f6760, 0xb74448, 0x25d0, ArgDaCarga::Caminho),  // 9 SCOUT_MANAGER
    PassoDaCarga::Objeto(0x14, 0x785b20, 0xb7431c, 0x25e5, ArgDaCarga::Caminho),  // 0xa News
    PassoDaCarga::Objeto(0x3507, 0x5c1f80, 0xae2c74, 0x25fa, ArgDaCarga::Sozinho), // 0xb Friendly
    PassoDaCarga::Objeto(0x698, 0x903360, 0xde1f58, 0x260f, ArgDaCarga::Sozinho),  // 0xc UefaSeeding
    PassoDaCarga::Objeto(0x75, 0x4c5fe0, 0xae1000, 0x2624, ArgDaCarga::Sozinho),   // 0xd ConmebolSeeding
    PassoDaCarga::Objeto(0x18, 0x868390, 0xdd7f00, 0x2639, ArgDaCarga::Caminho),   // 0xe Staff_Records
    PassoDaCarga::Objeto(0x20, 0x729000, 0xb6b7c0, 0x264e, ArgDaCarga::Estatisticas), // 0xf Match_Stats
    PassoDaCarga::Objeto(0x30, 0x444520, 0xad9dac, 0x2663, ArgDaCarga::Caminho),   // 0x10 Club_Records
    PassoDaCarga::Objeto(0x48, 0x7bfd50, 0xb74350, 0x2678, ArgDaCarga::Caminho),   // 0x11 PlayerStats
    PassoDaCarga::Objeto(0x10, 0x442ab0, 0xad9da8, 0x268d, ArgDaCarga::Caminho),   // 0x12 Club_History
    PassoDaCarga::Objeto(6, 0x5f97c0, 0xb5e134, 0x26a2, ArgDaCarga::Sozinho),      // 0x13 HostCountry
    PassoDaCarga::Objeto(0x2a, 0x5554a0, 0xae28f0, 0x26b7, ArgDaCarga::Caminho),   // 0x14 Discipline
    PassoDaCarga::Objeto(0xe0, 0x413180, 0xad9c60, 0x26cc, ArgDaCarga::Caminho),   // 0x15 AWARD_MANAGER
    PassoDaCarga::Objeto(0xc, 0x5a4fc0, 0xae2a3c, 0x26e0, ArgDaCarga::Caminho),    // 0x16 Fine
    PassoDaCarga::Objeto(0x10, 0x418440, 0xad9c6c, 0x26f4, ArgDaCarga::Caminho),   // 0x17 AWOL
    PassoDaCarga::Objeto(8, 0x5697e0, 0xae2a30, 0x2708, ArgDaCarga::Caminho),      // 0x18 DISPUTE
    PassoDaCarga::Objeto(0x1070, 0x794ad0, 0xb7433c, 0x271b, ArgDaCarga::Caminho), // 0x19 Notes
    PassoDaCarga::Objeto(0x2c, 0x62a680, 0xb63d28, 0x272f, ArgDaCarga::Caminho),   // 0x1a Injury
    PassoDaCarga::Objeto(0x3a, 0x76a4e0, 0xb74318, 0x2744, ArgDaCarga::Caminho),   // 0x1b NationalTeams
    PassoDaCarga::Objeto(0x6a, 0x90d140, 0xde2f70, 0x2759, ArgDaCarga::Sozinho),   // 0x1c VIRTUAL_STAFF
    PassoDaCarga::Objeto(0x19, 0x85b670, 0xdd7ef4, 0x276e, ArgDaCarga::Sozinho),   // 0x1d Stadium
    PassoDaCarga::Objeto(0x1184, 0x5a9cf0, 0xae2a58, 0x2783, ArgDaCarga::Calendario), // 0x1e FIXTURE_MANAGER
    PassoDaCarga::Rotina(0x669180, 0x2797),                                 // 0x1f competições (key_nation_create_all_comps)
    PassoDaCarga::Rotina(0x8386f0, 0x27ab),                                 // 0x20 build_comp_object_table
    PassoDaCarga::Objeto(2, 0x859aa0, 0xdd7ef0, 0x27c0, ArgDaCarga::Caminho),      // 0x21 SQUAD_MANAGER
    PassoDaCarga::Objeto(0xab, 0x7e5fc0, 0xb74434, 0x27d5, ArgDaCarga::Caminho),   // 0x22 (depois 0x7e61c0)
    PassoDaCarga::Objeto(4, 0x59a370, 0xae2a38, 0x27ed, ArgDaCarga::Caminho),      // 0x23 FinanceTable
    PassoDaCarga::Objeto(0x3c, 0x4c7040, 0xae1050, 0x2802, ArgDaCarga::Caminho),   // 0x24 CONTRACT_MANAGER
    PassoDaCarga::Objeto(0x94a, 0x8bba30, 0xde1f54, 0x2817, ArgDaCarga::Caminho),  // 0x25 TRANSFER_MANAGER
    PassoDaCarga::Objeto(0x60, 0x7a5380, 0xb7434c, 0x282c, ArgDaCarga::Caminho),   // 0x26 PLAYER_REGEN
    PassoDaCarga::Objeto(0x1c, 0x68dbf0, 0xb67a44, 0x2840, ArgDaCarga::Caminho),   // 0x27 MANAGER_MANAGER
    PassoDaCarga::Objeto(0x18, 0x72b1d0, 0xb6b7c8, 0x2855, ArgDaCarga::Caminho),   // 0x28 Media
    PassoDaCarga::Objeto(0x40, 0x5af630, 0xae2a60, 0x286a, ArgDaCarga::Caminho),   // 0x29 FOG_OF_WAR (confere o de Media)
    PassoDaCarga::Objeto(0x10, 0x79b580, 0xb74340, 0x287f, ArgDaCarga::Caminho),   // 0x2a OfficialsManager
    PassoDaCarga::Objeto(8, 0x84b940, 0xdd7ee4, 0x2894, ArgDaCarga::Caminho),      // 0x2b SimulatedStats
    PassoDaCarga::Objeto(0xc, 0x9226c0, 0xde2f74, 0x28a9, ArgDaCarga::Sozinho),    // 0x2c Weather
    PassoDaCarga::Objeto(0x688f, 0x8b0ec0, 0xde1f44, 0x28be, ArgDaCarga::Caminho), // 0x2d TRAINING_MANAGER
    PassoDaCarga::Objeto(0xa, 0x7da190, 0xb74430, 0x28d9, ArgDaCarga::Idioma),     // 0x2d de novo: PunditList
];

/// A busca do CD da carga ("<raiz>cm0102.exe" em cada unidade de CD; ver `construct_match_stats`).
/// `exigir` = a do começo (com os patches sem CD do cm0102-1080 lidos do exe: 0x82aac4 e 0x82ab07):
/// devolve só quando acha (sem achar, a caixa "Insert CD" e de novo, para sempre). Sem `exigir` = a do fim
/// (0x82e31d, sem patch nos dois exes): numa unidade de CD sem o exe, a caixa "CD Not Required" e para.
pub(crate) unsafe fn cd_da_carga(exigir: bool) {
    let units_digit: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(*(0x9670f8 as *const usize));
    let tipo: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(*(0x967100 as *const usize));
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let procurar: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FILE_FIND_FIRST);
    let fechar_busca: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_FILE_FIND_CLOSE);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    // o cm0102-1080 vem sem a exigência do CD só na busca do começo: o jne do tipo (0x82aac4, 75 55) virou
    // nop e o test do achado (0x82ab07, 85 c0) virou xor (31 c0)
    let qualquer_unidade = exigir && *(0x82aac4 as *const u8) == 0x90;
    let sempre_achou = exigir && *(0x82ab07 as *const u8) != 0x85;
    loop {
        // ponytail: o buffer nasce zerado; a original só leria lixo com mais de 200 bytes de units_digit
        let mut lista = [0u8; 0x100];
        if units_digit(200, lista.as_mut_ptr()) != 0 && lista[0] != 0 {
            let mut i = 0usize;
            loop {
                let raiz = lista.as_ptr() as usize + i;
                // o tipo é pedido sempre (no cm0102-1080 só o salto depois dele virou nop)
                let t = tipo(raiz);
                if qualquer_unidade || t == 5 {
                    let mut nome = [0u8; 0x104];
                    let mut achado = [0u8; 0x114];
                    let nm = nome.as_mut_ptr() as usize;
                    formatar(nm, 0x987e00, raiz, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let ach = achado.as_mut_ptr() as usize;
                    if procurar(nm, ach, 0) == 0 || sempre_achou {
                        if exigir {
                            return;
                        }
                        let (mut titulo, mut msg) = ([0u8; 0x100], [0u8; 0x100]);
                        texto(titulo.as_mut_ptr() as usize, 0xa81384);
                        texto(msg.as_mut_ptr() as usize, 0xa81354);
                        caixa(titulo.as_ptr() as usize, msg.as_ptr() as usize, 0);
                        return;
                    }
                    fechar_busca(ach);
                }
                // ponytail: a original anda com um i8 (passos de 4): só passaria de 127 com mais de 32 units_digit
                i += 4;
                if lista[i] == 0 {
                    break;
                }
            }
        }
        if !exigir {
            return;
        }
        let (mut titulo, mut msg) = ([0u8; 0x100], [0u8; 0x100]);
        texto(titulo.as_mut_ptr() as usize, 0x987df4);
        texto(msg.as_mut_ptr() as usize, 0x987dc4);
        caixa(titulo.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }
}

/// loads the saved game: disk, memory, CD, the database (0x524d10), unpacks the *.tmp files, general.dat (version,
/// summary, ids, dates, leagues, default nation, the runtime data of people, clubs and reputation, converting old
/// saves), the subsystem constructors in the original order with disk and memory checks between them, the FIFA rules
/// and the last save date
/// Original: `load_game_data` (0x82a9c0, Setup.cpp, cdecl).
///
/// [pt] { 0x82a9c0 } (cdecl, Setup.cpp, ~14,6 KB, SEH): `load_game_data(caminho)`: carrega o save inteiro.
/// Devolve 1 se carregou.
/// - Disco (0x7a1820(0) < 25 liga [0xdd7134]) e memória ([0x9cd1fc] e 0x7a1710() < 50 liga [0xdd7138]);
///   a pasta temporária (0x90bde0 "CM3_TEMP"); o CD (`cd_da_carga`); `caminho` nulo dá a caixa "Error"
///   0x233b. O banco: 0x528080 se [0xae23a8] ≠ 0 e 0x524d10(caminho, 1) (a resposta não é olhada).
/// - "Loading game data" e a barra 0x802f80 (passo de 0x2d); limpa os `*.tmp` (0x90cd50), o espaço
///   deles (0x933860, em MB, 0 vira 25) contra o disco, desempacota (0x933320; falha: "Disk Space
///   Error" 0x2382).
/// - `general.dat` (0x9326e0 lendo, 0x15, tipo 4; sem ele: "Load Game Error" 0x2397): a versão
///   ([0xdd7110]); o resumo (|v| ≥ 0x3e023; 0xf70 bytes até 0x3e026 com o tempo de jogo zerado, 0xf74
///   depois, com [0xdd7140] = o tempo de jogo dele); [0xdd7144] = time(); o id único, o tamanho do banco,
///   as datas e horas (a do jogo num local), as ligas, a nação padrão ([0xae2d08] = nação id, ou 0 se
///   < 0), os dados de execução das pessoas ([0xae2d0c], 0x4f × [0xae2d10]; os saves antigos convertidos
///   por `converter_dados_das_pessoas`; |v| < 0x3e018 apaga o bit 7 de +0xb das [0xae2364] primeiras),
///   dos clubes ([0xae2d14], 6 × [0xae2d18] = [0xae235c]; |v| < 0x3e022: −1, 0, 0) e a reputação
///   ([0xae2d1c], 4 × [0xae235c]; |v| < 0x3e022 não lê e chama 0x53db40(1)). Leitura que falha: "Load
///   Game Error" com a linha e o texto do campo, item fechado; sem memória: "Error" e o jogo fecha.
/// - Os passos 4..=0x2d (`PASSOS_DA_CARGA`, disco e memória antes de cada um): no 5, a data do jogo
///   (0x417c10 em 0xae2c90); antes do 9, os 16 últimos técnicos com +0x5f = 5 são desfeitos (News,
///   HUMAN_MANAGER, CONTRACT_MANAGER, NationalTeams, Staff_Records; nomes vazios, data 1/0/1966, +0x24 e
///   +0x39 = 0, 0x7ff990); depois do 0x22, 0x7e61c0; antes do último, |v| < 0x3e032 passa cada pessoa
///   por 0x538bc0.
/// - Regras FIFA (0x839760, |v| ≥ 0x3e039); 0xdd70a0 = a data do jogo; [0xdd711c] = [0xdd7130] = 1,
///   [0xa801bc] = 0; com [0x9b5980], liga ou desliga as regras pela data de [0xae2330] (0x58a890(1) ao
///   ligar); sem `auto_setup.cfg`, a busca do CD do fim.
/// Os ramos da original para 0x3e042 ≤ |v| < 0x3e038 (registros de 0xd bytes) e 0x3e042 ≤ |v| <
/// 0x3e034 (0xc) não são alcançáveis (as comparações estão encadeadas em `jge`): ficam de fora. O laço
/// de duas voltas da original ([esp+0x24]; a 1ª só põe o total 0x2d) roda o corpo uma vez.
pub(crate) unsafe extern "C" fn load_game_data(caminho: *const u8) -> u32 {
    let disco: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_OS_GET_FREE_DISK_SPACE_MB);
    let memoria: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_OS_GET_FREE_MEMORY_MB);
    let pasta: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let liberar_tabelas: unsafe extern "C" fn() = core::mem::transmute(FN_DATABASE_FREE_ALL_TABLES);
    let carregar_banco: unsafe extern "C" fn(*const u8, u32) -> u32 = core::mem::transmute(FN_LOAD_DATABASE_FROM_STREAM);
    let traduzir: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let barra: unsafe extern "thiscall" fn(usize, *const u8, u32, u32, i32) = core::mem::transmute(FN_SCREENMANAGER_SEND_NETWORK_STATUS_MESSAGE);
    let limpar_temporarios: unsafe extern "C" fn(usize) = core::mem::transmute(FN_UTILS_CLEAR_TEMP_FOLDER);
    let tamanho: unsafe extern "C" fn(*const u8, usize) -> i32 = core::mem::transmute(FN_ZGET_MATCHING_SIZE);
    let desempacotar: unsafe extern "C" fn(*const u8, *const u8, usize) -> i32 = core::mem::transmute(FN_ZEXTRACT_MATCHING_FILES);
    let abrir_item: unsafe extern "C" fn(*const u8, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> i32 = core::mem::transmute(FN_ZREAD);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    let relogio: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(FN_946EBB);
    let reservar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let reputacao_inicial: unsafe extern "C" fn(u32) = core::mem::transmute(FN_DATABASE_SORT_CLUBS_BY_REPUTATION);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let regras_fifa: unsafe extern "C" fn(*const u8) = core::mem::transmute(FN_LOAD_FIFA_TRANSFER_RULES);
    let ligar_regras: unsafe extern "C" fn(u32) = core::mem::transmute(FN_NEWS_POST_TRANSFER_SYSTEM_CHANGE);
    let existe: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_UTILS_FILE_EXISTS);
    crate::comparacao::ramo("save carregar jogo");
    let u = |a: usize| *(a as *const u32);
    let sem_memoria = |minimo: i32| u(G_MEMORY_CHECK_ENABLED) != 0 && memoria() < minimo;
    // as duas datas locais da original (a 2ª não é usada); o destrutor delas é vazio
    let (mut data_do_jogo, mut outra_data) = ([0u8; 8], [0u8; 8]);
    data_padrao(data_do_jogo.as_mut_ptr());
    data_padrao(outra_data.as_mut_ptr());
    if disco(0) < 0x19 {
        *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
        return 0;
    }
    if sem_memoria(0x32) {
        *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
        return 0;
    }
    let mut temporaria = [0u8; 0x104];
    copiar_texto(temporaria.as_mut_ptr(), pasta(0x987da0, 0xad9c64));
    cd_da_carga(true);
    if caminho.is_null() {
        aviso_do_setup(0x233b);
        return 0;
    }
    if u(G_NATION_TABLE) != 0 {
        liberar_tabelas();
    }
    carregar_banco(caminho, 1);
    // disco e memória entre os passos: liga a marca e devolve 0
    let falta_espaco = |minimo_disco: i32| {
        if disco(0) < minimo_disco {
            *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
            return true;
        }
        if sem_memoria(0x19) {
            *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
            return true;
        }
        false
    };
    let mut texto = [0u8; 0xc8];
    traduzir(texto.as_mut_ptr(), 0xa81790);
    barra(0xb74450, texto.as_ptr(), 1, 0x2d, -1);
    limpar_temporarios(0xa813ac);
    let n = tamanho(caminho, 0xa813ac);
    let mb = match n.wrapping_add((n >> 31) & 0xfffff) >> 20 {
        0 => 0x19,
        x => x,
    };
    if falta_espaco(mb) {
        return 0;
    }
    barra(0xb74450, texto.as_ptr(), 2, 0x2d, -1);
    if desempacotar(caminho, temporaria.as_ptr(), 0xa813ac) == 0 {
        caixa_do_save(0x2382, 0xa81758, 0x9c7774, 0x9c7680);
        return 0;
    }
    if falta_espaco(0x19) {
        return 0;
    }
    barra(0xb74450, texto.as_ptr(), 3, 0x2d, -1);
    let mut z = abrir_item(caminho, 0x9c7b84, 0, 0x15, 4);
    if z == 0 {
        caixa_do_save(0x2397, 0xa81720, 0x9af3e0, 0x9af334);
        return 0;
    }
    let mut versao = 0i32;
    if ler(&mut versao as *mut i32 as usize, 4, 1, z) != 1 {
        return erro_na_carga(0x239e, 0xa816e0, &mut z);
    }
    *(G_GAME_DATA_VERSION as *mut i32) = versao;
    let v = versao.wrapping_abs();
    let mut resumo = [0u8; 0xf74];
    if v >= 0x3e023 {
        if v < 0x3e027 {
            if ler(resumo.as_mut_ptr() as usize, 0xf70, 1, z) != 1 {
                return erro_na_carga(0x23b2, 0xa816a4, &mut z);
            }
            resumo[0xf70..].fill(0);
        } else {
            if ler(resumo.as_mut_ptr() as usize, 0xf74, 1, z) != 1 {
                return erro_na_carga(0x23bf, 0xa816a4, &mut z);
            }
            *(G_GAME_PLAY_TIME_TOTAL as *mut u32) = core::ptr::read_unaligned(resumo.as_ptr().add(0xf6c) as *const u32);
        }
    }
    let mut agora = 0i32;
    *(G_GAME_PLAY_TIME_LAST_STAMP as *mut i32) = relogio(&mut agora);
    let mut nacao = 0i32;
    let campos: [(usize, u32, u32, usize); 10] = [
        (0xdd7120, 4, 0x23cf, 0xa8166c),
        (0xdd7114, 1, 0x23d7, 0xa81634),
        (0xdd6ec8, 8, 0x23df, 0xa81604),
        (data_do_jogo.as_mut_ptr() as usize, 8, 0x23e7, 0xa81604),
        (0xae2c88, 1, 0x23ef, 0xa815d0),
        (0xae2c78, 8, 0x23f7, 0xa8159c),
        (0xae2cfc, 1, 0x23ff, 0xa81560),
        (0xae2d00, 4, 0x2407, 0xa8151c),
        (0xae2d04, 4, 0x240f, 0xa814cc),
        (&mut nacao as *mut i32 as usize, 4, 0x2417, 0xa81490),
    ];
    for (buf, tam, linha, msg) in campos {
        if ler(buf, tam, 1, z) != 1 {
            return erro_na_carga(linha, msg, &mut z);
        }
    }
    *(G_MAIN_NATION as *mut usize) = if nacao > -1 { (u(G_NATION_TABLE) as usize).wrapping_add((nacao as usize).wrapping_mul(0x122)) } else { 0 };
    if ler(0xae2d10, 4, 1, z) != 1 {
        return erro_na_carga(0x2420, 0xa81450, &mut z);
    }
    let pessoas = reservar(u(G_STAFF_STATE_COUNT).wrapping_mul(0x4f));
    *(G_STAFF_STATE_TABLE as *mut usize) = pessoas;
    if pessoas == 0 {
        fatal_da_carga(0x2427);
    }
    if v < 0x3e045 {
        // os saves antigos: registros de 0x26 (|v| < 0x3e042) ou 0x27 bytes, convertidos
        let (tam, linha_mem, linha_ler) = if v < 0x3e042 { (0x26u32, 0x242f, 0x2434) } else { (0x27, 0x24bd, 0x24c2) };
        let antigos = reservar(u(G_STAFF_STATE_COUNT).wrapping_mul(tam));
        if antigos == 0 {
            fatal_da_carga(linha_mem);
        }
        if ler(antigos, tam, u(G_STAFF_STATE_COUNT), z) != u(G_STAFF_STATE_COUNT) as i32 {
            return erro_na_carga(linha_ler, 0xa81418, &mut z);
        }
        converter_dados_das_pessoas(antigos, tam as usize, u(G_STAFF_STATE_COUNT) as i32);
        soltar(antigos);
    } else if ler(pessoas, 0x4f, u(G_STAFF_STATE_COUNT), z) != u(G_STAFF_STATE_COUNT) as i32 {
        return erro_na_carga(0x24f0, 0xa81418, &mut z);
    }
    if v < 0x3e018 {
        for i in 0..u(G_STAFF_COUNT) as i32 {
            *((u(G_STAFF_STATE_TABLE) as usize + 0x4f * i as usize + 0xb) as *mut u8) &= 0x7f;
        }
    }
    *(G_CLUB_STATE_COUNT as *mut u32) = u(G_CLUB_COUNT);
    let clubes = reservar(u(G_CLUB_COUNT).wrapping_mul(6));
    *(G_CLUB_STATE_TABLE as *mut usize) = clubes;
    if clubes == 0 {
        fatal_da_carga(0x2503);
    }
    if v < 0x3e022 {
        for i in 0..u(G_CLUB_STATE_COUNT) as i32 {
            let c = u(G_CLUB_STATE_TABLE) as usize + 6 * i as usize;
            core::ptr::write_unaligned(c as *mut u32, 0xffffffff);
            *((u(G_CLUB_STATE_TABLE) as usize + 6 * i as usize + 4) as *mut u8) = 0;
            *((u(G_CLUB_STATE_TABLE) as usize + 6 * i as usize + 5) as *mut u8) = 0;
        }
        let r = reservar(u(G_CLUB_COUNT).wrapping_mul(4));
        *(G_CLUB_REPUTATION_ORDER as *mut usize) = r;
        if r == 0 {
            fatal_da_carga(0x2516);
        }
        reputacao_inicial(1);
    } else {
        if ler(clubes, 6, u(G_CLUB_STATE_COUNT), z) != u(G_CLUB_STATE_COUNT) as i32 {
            return erro_na_carga(0x251f, 0xa813e0, &mut z);
        }
        let r = reservar(u(G_CLUB_COUNT).wrapping_mul(4));
        *(G_CLUB_REPUTATION_ORDER as *mut usize) = r;
        if r == 0 {
            fatal_da_carga(0x2528);
        }
        if ler(r, 4, u(G_CLUB_COUNT), z) != u(G_CLUB_COUNT) as i32 {
            aviso_do_setup(0x252c);
            fechar_item(&mut z);
            return 0;
        }
    }
    fechar_item(&mut z);
    for (i, passo) in PASSOS_DA_CARGA.iter().enumerate() {
        let k = (i as u32 + 4).min(0x2d);
        if k == 9 {
            tecnicos_de_ferias_desfeitos();
        }
        if i == PASSOS_DA_CARGA.len() - 1 && v < 0x3e032 {
            let acertar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_PLAYER_ATTRIBUTES);
            for p in 0..u(G_STAFF_COUNT) as i32 {
                acertar(u(G_STAFF_TABLE) as usize + 0x6e * p as usize);
            }
        }
        if falta_espaco(0x19) {
            return 0;
        }
        barra(0xb74450, texto.as_ptr(), k, 0x2d, -1);
        match *passo {
            PassoDaCarga::Rotina(f, linha) => {
                let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(f);
                if f() == 0 && linha != 0 {
                    aviso_do_setup(linha);
                    return 0;
                }
                if k == 5 {
                    let mut tmp = [0u8; 8];
                    copia_data(0xae2c90, tmp.as_mut_ptr(), data_do_jogo.as_ptr() as usize);
                }
            }
            PassoDaCarga::Objeto(tam, construtor, global, linha, arg) => {
                let p = novo(tam);
                let o = if p == 0 {
                    0
                } else {
                    match arg {
                        ArgDaCarga::Caminho => {
                            let f: unsafe extern "thiscall" fn(usize, *const u8) -> usize = core::mem::transmute(construtor);
                            f(p, caminho)
                        }
                        ArgDaCarga::Sozinho => {
                            let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(construtor);
                            f(p)
                        }
                        ArgDaCarga::Estatisticas => {
                            let f: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(construtor);
                            f(p, caminho, 0x2710)
                        }
                        ArgDaCarga::Calendario => {
                            // a original empurra um dword com o ano nos 16 bits baixos e sobra de pilha nos
                            // altos; o construtor só usa a palavra (0x54cfe0 faz & 0xffff)
                            let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(construtor);
                            f(p, *(G_CURRENT_DATE_YEAR as *const u16) as u32, 0, 1)
                        }
                        ArgDaCarga::Idioma => {
                            // a original empurra edx com o byte em dl (o resto é sobra); o construtor só lê o byte
                            let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(construtor);
                            f(p, *(G_DATABASE_NAMES_LANGUAGE as *const u8) as u32)
                        }
                    }
                };
                *(global as *mut usize) = o;
                // o FOG_OF_WAR confere o global de Media (como na gravação)
                let conferido = if k == 0x29 { u(G_MEDIA) as usize } else { o };
                if conferido == 0 {
                    aviso_do_setup(linha);
                    return 0;
                }
                if k == 0x22 {
                    let iniciar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_RESERVECOMP_CREATE_SUB_COMPS);
                    iniciar(o);
                }
            }
        }
    }
    if v >= 0x3e039 {
        regras_fifa(caminho);
    }
    *(G_GAME_DATA_LOADED_FLAG as *mut u32) = 1;
    *(G_GAME_HAS_SAVE_FILE_FLAG as *mut u32) = 1;
    *(G_GAME_UNSAVED_CHANGES_FLAG as *mut u32) = 0;
    let mut tmp = [0u8; 8];
    copia_data(0xdd70a0, tmp.as_mut_ptr(), 0xae2c90);
    if *(G_FIFA_TRANSFER_RULES_FEATURE_ENABLED as *const u8) != 0 {
        // a data de mudança das regras [0xae2330] (palavra baixa: dia; alta: ano) contra a do jogo
        // [0xae2c90]: ano sem sinal, dia com sinal
        let em_vigor = *(G_FIFA_TRANSFER_RULES_IN_FORCE as *const u8);
        let (dia_jogo, ano_jogo) = (*(G_CURRENT_DATE as *const i16), *(G_CURRENT_DATE_YEAR as *const u16));
        let (dia, ano) = (*(G_TRANSFER_RULE_CHANGE_DATE as *const i16), *(G_TRANSFER_RULE_CHANGE_DATE_YEAR as *const u16));
        let mut desligar = false;
        if !(em_vigor == 0 && *(G_MAX_CONTRACT_LENGTH_YEARS as *const u8) == 0xa) {
            desligar = ano > ano_jogo || (ano == ano_jogo && dia > dia_jogo);
        }
        if desligar {
            *(G_FIFA_TRANSFER_RULES_IN_FORCE as *mut u8) = 0;
            *(G_MAX_CONTRACT_LENGTH_YEARS as *mut u8) = 0xa;
        } else if !(em_vigor != 0 && *(G_MAX_CONTRACT_LENGTH_YEARS as *const u8) == 5) && (ano < ano_jogo || (ano == ano_jogo && dia < dia_jogo)) {
            *(G_FIFA_TRANSFER_RULES_IN_FORCE as *mut u8) = 1;
            *(G_MAX_CONTRACT_LENGTH_YEARS as *mut u8) = 5;
            ligar_regras(1);
        }
    }
    if existe(0xa80340) == 0 {
        cd_da_carga(false);
    }
    1
}

/// Antes do passo 9 da carga: dos 16 últimos da tabela de pessoas ([0xae23bc], 0x6e bytes, do último
/// para trás), os com +0x5f = 5 deixam de ser técnico humano: +0x5f = 0, News 0x787630, HUMAN_MANAGER
/// 0x5fd2f0/0x5fd590/0x5fe140 (0, pessoa), CONTRACT_MANAGER 0x4ccbf0/0x4d0b50/0x4d0bd0,
/// NationalTeams 0x775ba0, Staff_Records 0x86fb80/0x86fc70 (cada um com o global como está), os três
/// nomes vazios, +0x10 = data(1, 0, 1966, −1) (0x54a130), +0x18 = 1966, +0x24 = +0x39 = 0 e 0x7ff990
/// (gerente de telas, pessoa, −1).
pub(crate) unsafe fn tecnicos_de_ferias_desfeitos() {
    let u = |a: usize| *(a as *const usize);
    let um: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NEWS_CLEAR_INBOX);
    for i in 0..0x10usize {
        let s = u(G_STAFF_TABLE).wrapping_add(0x6e * ((*(G_STAFF_COUNT as *const i32)).wrapping_sub(i as i32).wrapping_sub(1) as usize));
        if *((s + 0x5f) as *const u8) != 5 {
            continue;
        }
        *((s + 0x5f) as *mut u8) = 0;
        um(u(G_NEWS), s);
        for f in [0x5fd2f0usize, 0x5fd590, 0x5fe140] {
            let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(f);
            f(u(G_HUMAN_MANAGER), 0, s);
        }
        for (g, f) in [(0xae1050usize, 0x4ccbf0usize), (0xae1050, 0x4d0b50), (0xae1050, 0x4d0bd0), (0xb74318, 0x775ba0), (0xdd7f00, 0x86fb80), (0xdd7f00, 0x86fc70)] {
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
            f(u(g), s);
        }
        *((s + 0x5f) as *mut u8) = 0;
        *(u(s + 4) as *mut u8) = 0;
        *(u(s + 8) as *mut u8) = 0;
        *(u(s + 0xc) as *mut u8) = 0;
        let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_SET);
        acertar_data(s + 0x10, 1, 0, 0x7ae, -1);
        *((s + 0x18) as *mut u16) = 0x7ae;
        core::ptr::write_unaligned((s + 0x24) as *mut u32, 0);
        core::ptr::write_unaligned((s + 0x39) as *mut u32, 0);
        let tela: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_SCREENMANAGER_REMOVE_MANAGER_RECORD);
        tela(0xb74450, s, -1);
    }
}

// ponytail: carrega o save inteiro (apaga e recria os subsistemas, desempacota a pasta temporária);
// não roda duas vezes. Só a original; a cópia leva o ramo e é provada pela carga da prova do save.
pub(crate) unsafe extern "C" fn comparar_load_game_data(caminho: *const u8) -> u32 {
    orig::<unsafe extern "C" fn(*const u8) -> u32>(IDX_LOAD_GAME_DATA)(caminho)
}

// ---------- 220a leva: o save: criar o jogo pelo arquivo de configuração (o início rápido e a carreira automática) (Setup.cpp) ----------

/// A caixa "Error" de Setup.cpp (`aviso_do_setup`) e o jogo para, como a original: 0x5cc7d0,
/// 0x5a8f60 e `exit(-1)` (0x945934).
pub(crate) unsafe fn erro_fatal_do_setup(linha: u32) -> ! {
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    aviso_do_setup(linha);
    fechar1();
    fechar2();
    sair(-1)
}

/// reads auto_setup.cfg/quick_start.cfg (database, stats, foreground and background leagues, language, engine, bids,
/// holiday), creates the game (0x828110) and saves the .qst (0x82e430 with 0x17) and frees everything, or puts the
/// human manager on holiday at a drawn club
/// Original: `create_game_from_setup_file` (0x827110, Setup.cpp, cdecl).
///
/// [pt] { 0x827110 } (cdecl, Setup.cpp, ~3 KB): `create_game_from_setup_file(cfg, qst)`: lê o arquivo de
/// configuração `cfg` (`auto_setup.cfg` ou `quick_start.cfg`), cria o jogo e, com `qst`, grava o
/// início rápido `CM3_QSTART\<qst>.qst`; sem `qst`, põe o técnico humano de férias num clube
/// sorteado. Devolve 1 se criou.
/// - Pastas CM3_TEMP (0x90bde0, só copiada, como a original) e CM3_DATA. `cfg` nulo ou que não abre
///   (0x945b00 "r"): a caixa "Error" (0x1c13, 0x1c1b) e 0.
/// - Cada linha (0x90cba0, buffer de 1000 bytes) passa por 0x90bb70 e perde os espaços (um a um,
///   como o `strcpy` da original); o valor é procurado por `strstr` (0x945a60) depois do prefixo:
///   `DATABASE=` NORMAL/MINIMUM/MAXIMUM → 0x5d7c60(1/2/4) em [0xae3138]; `MATCH_STATS=`
///   NORMAL/SMALL/LARGE → 0x200/0x400/0x800; `FOREGROUND_LEAGUES=` e `BACKGROUND_LEAGUES=`: na
///   primeira, sem banco ([0xae23a8] = 0), carrega o banco (0x524d50, CM3_DATA; falha para o jogo),
///   aplica (0x5d8480), zera o byte +0x11c das nações, [0xae2d00], [0xae2d04] (e [0xae2d08] na
///   de frente) e monta as nações-chave (0x667150; falha para o jogo); depois, para cada liga da
///   tabela 0xb63d60 (0x23 de 0x48) com nação e +8, se o nome (0x5454c0) está na linha ou 0x839340
///   aceita: de frente, marca 0/1 → ((~opções & 2) | 1) << 1 e conta (+0xae2d00 só do 0,
///   +0xae2d04) e a 1ª que casa pelo `strncmp` (0x946fa0) ou 0x839340 vira [0xae2d08]; de fundo,
///   0 → ((~opções & 2) << 1) | 1 (+[0xae2d00]) e 2 → o mesmo (−[0xae2d04]); `LANGUAGE=`: o 1º
///   idioma i (tabela 0x9eb244, 19) que existe (0x66ea50), é o inglês, não tem banco ou o banco
///   tem (0x535930) e está na linha: 0x5d8020(i) e 0x5d8480; `ENABLE_MATCH_ENGINE=` YES/NO →
///   byte [0xa5a390]; `ENABLE_COMPUTER_BIDS=` → [0x9cd1f8]; `ON_HOLIDAY=` → as férias do técnico.
/// - Fecha; sem liga marcada, sem liga de frente, sem nação padrão ou com ela sem o bit 2: 0.
/// - [0xdd7124] = [0xdd713c] = 1; cria o jogo (0x828110(qst ≠ 0); falha para o jogo).
/// - Com `qst`: 0x945b13 (CM3_QSTART, `qst`, ".qst"), 0x82e430(caminho, 0x17), 0x82ff40(1) e 1.
/// - Sem: "auto_setup_game" em [0xae3138]+0x113, [0xdd7130] = 1, data em 0xdd70a0; se o humano da
///   vez (0xb74c26 + 0x300 × (i16)[0xb77c7a]) já tem pessoa, 0. Senão, a 1ª das 16 últimas pessoas
///   (0xae23bc, 0x6e) com +0x5f = 0 (sem tabela, a caixa 0x39b3 e pessoa 0) vira o humano: texto
///   +0xc zerado, data +0x10 (0x54a130: 1, 0, 1966, −1), +0x18 = 1966, +0x24 e +0x39 = 0, +0x5f = 1,
///   0x5fb810 e 0x5fd2f0(férias, 0) em [0xb63c98]; sorteia (0x90cfc0) clubes (0xae23b4, 0x245) até
///   um fora de seleção (0x53b900) com nação de liga de frente (bit 2) e aceito por 0x544820: a
///   nação dele em +0x1a, os nomes (0x539c60, 0x53a1b0) em +4 e +8, e 0x826f20(pessoa, clube). 1.
pub(crate) unsafe extern "C" fn create_game_from_setup_file(cfg: *const u8, qst: *const u8) -> u32 {
    let pasta: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(*const u8, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let ler_linha: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(FN_UTILS_READ_CONFIG_LINE);
    let ajeitar: unsafe extern "C" fn(*mut u8) = core::mem::transmute(FN_UTILS_TRIM_CONTROL_CHARS);
    let comeca: unsafe extern "C" fn(*const u8, usize, u32) -> i32 = core::mem::transmute(FN_947120);
    let procurar: unsafe extern "C" fn(*const u8, usize) -> usize = core::mem::transmute(FN_CRT_STRSTR);
    let comparar_n: unsafe extern "C" fn(*const u8, *const u8, u32) -> i32 = core::mem::transmute(FN_CRT_STRNCMP);
    let opcao: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONFIG_SET_OPTION);
    let aplicar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONFIG_APPLY_AND_SAVE);
    let trocar_idioma: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONFIG_SET_LANGUAGE);
    let carregar_banco: unsafe extern "C" fn(*const u8, u32) -> i32 = core::mem::transmute(FN_LOAD_DATABASE_FROM_FILES);
    let nacoes_chave: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_KEY_NATION_BUILD_TABLE);
    let nome_da_nacao: unsafe extern "C" fn(usize) -> *const u8 = core::mem::transmute(FN_NATION_GET_ENGLISH_THREE_LETTER_NAME);
    let nacao_casa: unsafe extern "C" fn(usize, *const u8) -> i32 = core::mem::transmute(FN_NATION_MATCHES_REGION_FILTER);
    let idioma_existe: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_LANGLIB_IS_LANGUAGE_AVAILABLE);
    let idioma_no_banco: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_DATABASE_LANGUAGE_FILE_EXISTS);
    let criar_jogo: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_CREATE_GAME);
    let juntar: unsafe extern "C" fn(*mut u8, usize, *const u8, *const u8, usize) = core::mem::transmute(FN_CRT_MAKEPATH);
    let save_game_data: unsafe extern "C" fn(*const u8, u32) -> u32 = core::mem::transmute(FN_SAVE_GAME_DATA);
    let liberar: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_FREE_GAME_DATA);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_SET);
    let novo_humano: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_HUMAN_MANAGER_RESET_HUMAN_RECORD);
    let set_on_holiday: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_ON_HOLIDAY);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let aceita: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let primeiro_nome: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_FOR_NATION);
    let segundo_nome: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_PICK_SECOND_NAME_FOR_NATION);
    let pos_humano: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_APPOINT_HUMAN_MANAGER_TO_CLUB);
    crate::comparacao::ramo("save criar jogo pela configuracao");
    let opcoes = || *(G_CONFIG as *const usize);
    let u = |a: usize| *(a as *const u32);
    let i = |a: usize| *(a as *const i32);
    let mut temporaria = [0u8; 0x104];
    copiar_texto(temporaria.as_mut_ptr(), pasta(0x987da0, 0xad9c64));
    let mut dados = [0u8; 0x104];
    copiar_texto(dados.as_mut_ptr(), pasta(0x9b5348, 0xad9c64));
    if cfg.is_null() {
        aviso_do_setup(0x1c13);
        return 0;
    }
    let f = abrir(cfg, 0x9899fc);
    if f == 0 {
        aviso_do_setup(0x1c1b);
        return 0;
    }
    let mut ferias = 0u32;
    let mut linha = [0u8; 0x3e8];
    let b = linha.as_mut_ptr();
    // sem banco: carrega, aplica, zera as marcas das nações e as contagens, monta as nações-chave
    let preparar_banco = |de_frente: bool, erro_banco: u32, erro_chave: u32| {
        if carregar_banco(dados.as_ptr(), 1) == 0 {
            erro_fatal_do_setup(erro_banco);
        }
        aplicar(opcoes());
        let mut k = 0i32;
        let mut d = 0usize;
        while k < i(G_NATION_COUNT) {
            k += 1;
            *((u(G_NATION_TABLE) as usize + d + 0x11c) as *mut u8) = 0;
            d += 0x122;
        }
        *(G_NATIONS_TO_LOAD_COUNT as *mut u32) = 0;
        *(G_SELECTED_NATION_COUNT as *mut u32) = 0;
        if de_frente {
            *(G_MAIN_NATION as *mut u32) = 0;
        }
        if nacoes_chave() == 0 {
            erro_fatal_do_setup(erro_chave);
        }
    };
    while ler_linha(f, b) != 0 {
        ajeitar(b);
        if *b != 0 {
            let mut p = b;
            loop {
                if *p == b' ' {
                    copiar_texto(p, p.add(1));
                }
                p = p.add(1);
                if *p == 0 {
                    break;
                }
            }
        }
        let tem = |o: usize, valor: usize| procurar(b.add(o), valor) != 0;
        if comeca(b, 0xa81348, 9) == 0 {
            // DATABASE=
            if tem(9, 0xa81340) {
                opcao(opcoes(), 1);
            } else if tem(9, 0xa81338) {
                opcao(opcoes(), 2);
            } else if tem(9, 0xa81330) {
                opcao(opcoes(), 4);
            }
        } else if comeca(b, 0xa81320, 0xc) == 0 {
            // MATCH_STATS=
            if tem(0xc, 0xa81340) {
                opcao(opcoes(), 0x200);
            } else if tem(0xc, 0xa81318) {
                opcao(opcoes(), 0x400);
            } else if tem(0xc, 0xa81310) {
                opcao(opcoes(), 0x800);
            }
        } else if comeca(b, 0xa812fc, 0x13) == 0 {
            // FOREGROUND_LEAGUES=
            if u(G_NATION_TABLE) == 0 {
                preparar_banco(true, 0x1c49, 0x1c5d);
            }
            let v = b.add(0x13);
            let mut liga = 0xb63d68usize;
            while liga < 0xb64740 {
                let n = u(liga - 8) as usize;
                if n != 0 && u(liga) != 0 && (procurar(v, nome_da_nacao(n) as usize) != 0 || nacao_casa(n, v) != 0) {
                    let m = (n + 0x11c) as *mut u8;
                    let marca = ((!(u(opcoes() + 4) as u8) & 2) | 1) << 1;
                    if *m == 0 {
                        *m = marca;
                        *(G_NATIONS_TO_LOAD_COUNT as *mut i32) += 1;
                        *(G_SELECTED_NATION_COUNT as *mut i32) += 1;
                    } else if *m == 1 {
                        *m = marca;
                        *(G_SELECTED_NATION_COUNT as *mut i32) += 1;
                    }
                    let tam = core::ffi::CStr::from_ptr(nome_da_nacao(n) as *const core::ffi::c_char).to_bytes().len() as u32;
                    if (comparar_n(v, nome_da_nacao(n), tam) == 0 || nacao_casa(n, v) != 0) && u(G_MAIN_NATION) == 0 {
                        *(G_MAIN_NATION as *mut u32) = n as u32;
                    }
                }
                liga += 0x48;
            }
        } else if comeca(b, 0xa812e8, 0x13) == 0 {
            // BACKGROUND_LEAGUES=
            if u(G_NATION_TABLE) == 0 {
                preparar_banco(false, 0x1c92, 0x1ca3);
            }
            let v = b.add(0x13);
            let mut liga = 0xb63d68usize;
            while liga < 0xb64740 {
                let n = u(liga - 8) as usize;
                if n != 0 && u(liga) != 0 && (procurar(v, nome_da_nacao(n) as usize) != 0 || nacao_casa(n, v) != 0) {
                    let m = (n + 0x11c) as *mut u8;
                    if *m == 0 {
                        *m = ((!(u(opcoes() + 4) as u8) & 2) << 1) | 1;
                        *(G_NATIONS_TO_LOAD_COUNT as *mut i32) += 1;
                    } else if *m == 2 {
                        *m = ((!(u(opcoes() + 4) as u8) & 2) << 1) | 1;
                        *(G_SELECTED_NATION_COUNT as *mut i32) -= 1;
                    }
                }
                liga += 0x48;
            }
        } else if comeca(b, 0xa812dc, 9) == 0 {
            // LANGUAGE=
            let mut k = 0u32;
            let mut t = 0x9eb244usize;
            while t < 0x9eb290 {
                if idioma_existe(k) != 0
                    && (t == 0x9eb244 || u(G_NATION_TABLE) == 0 || idioma_no_banco(k) != 0)
                    && tem(9, u(t) as usize)
                {
                    trocar_idioma(opcoes(), k);
                    aplicar(opcoes());
                    break;
                }
                t += 4;
                k += 1;
            }
        } else if comeca(b, 0xa812c4, 0x14) == 0 {
            // ENABLE_MATCH_ENGINE=
            if tem(0x14, 0xa812c0) {
                *(G_MATCH_ENGINE_ENABLED as *mut u8) = 1;
            } else if tem(0x14, 0xa812bc) {
                *(G_MATCH_ENGINE_ENABLED as *mut u8) = 0;
            }
        } else if comeca(b, 0xa812a4, 0x15) == 0 {
            // ENABLE_COMPUTER_BIDS=
            if tem(0x15, 0xa812c0) {
                *(G_COMPUTER_BIDS_ENABLED as *mut u32) = 1;
            } else if tem(0x15, 0xa812bc) {
                *(G_COMPUTER_BIDS_ENABLED as *mut u32) = 0;
            }
        } else if comeca(b, 0xa81298, 0xb) == 0 {
            // ON_HOLIDAY=
            if tem(0xb, 0xa812c0) {
                ferias = 1;
            } else if tem(0xb, 0xa812bc) {
                ferias = 0;
            }
        }
    }
    fechar(f);
    if i(G_NATIONS_TO_LOAD_COUNT) <= 0 || i(G_SELECTED_NATION_COUNT) <= 0 {
        return 0;
    }
    let padrao = u(G_MAIN_NATION) as usize;
    if padrao == 0 || *((padrao + 0x11c) as *const u8) & 2 == 0 {
        return 0;
    }
    *(G_NEW_GAME_SETUP_DONE_FLAG as *mut u32) = 1;
    *(G_SETUP_FILE_GAME_FLAG as *mut u32) = 1;
    if criar_jogo(!qst.is_null() as u32) == 0 {
        erro_fatal_do_setup(0x1d11);
    }
    if !qst.is_null() {
        let mut arquivo = [0u8; 0x104];
        juntar(arquivo.as_mut_ptr(), 0, pasta(0xa80204, 0xad9c64), qst, 0xa803b4);
        save_game_data(arquivo.as_ptr(), 0x17);
        liberar(1);
        return 1;
    }
    copiar_texto((opcoes() + 0x113) as *mut u8, 0xa81288 as *const u8);
    *(G_GAME_HAS_SAVE_FILE_FLAG as *mut u32) = 1;
    let mut data = [0u8; 8];
    copia_data(0xdd70a0, data.as_mut_ptr(), 0xae2c90);
    let vaga = || 0xb74c26usize.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize).wrapping_mul(0x300));
    if core::ptr::read_unaligned(vaga() as *const u32) != 0 {
        return 0;
    }
    let tabela = u(G_STAFF_TABLE) as usize;
    let mut p = 0usize;
    if tabela == 0 {
        aviso_do_setup(0x39b3);
    } else {
        let mut q = tabela.wrapping_add((i(G_STAFF_COUNT) as usize).wrapping_mul(0x6e)).wrapping_sub(0x6e);
        for _ in 0..0x10 {
            if *((q + 0x5f) as *const u8) == 0 {
                p = q;
                break;
            }
            q = q.wrapping_sub(0x6e);
        }
    }
    core::ptr::write_unaligned(vaga() as *mut u32, p as u32);
    *core::ptr::read_unaligned((p + 0xc) as *const *mut u8) = 0;
    acertar_data(p + 0x10, 1, 0, 0x7ae, -1);
    core::ptr::write_unaligned((p + 0x18) as *mut u16, 0x7ae);
    core::ptr::write_unaligned((p + 0x24) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x39) as *mut u32, 0);
    *((p + 0x5f) as *mut u8) = 1;
    novo_humano(u(G_HUMAN_MANAGER) as usize, p);
    set_on_holiday(u(G_HUMAN_MANAGER) as usize, ferias, 0);
    let clube = loop {
        let r = sorteio(i(G_CLUB_COUNT));
        let c = (u(G_CLUB_TABLE) as usize).wrapping_add((r as usize).wrapping_mul(0x245));
        if selecao(c) != 0 {
            continue;
        }
        let nacao = core::ptr::read_unaligned((c + 0x53) as *const usize);
        if nacao == 0 || *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) & 2 == 0 || aceita(c) == 0 {
            continue;
        }
        break c;
    };
    let nacao = core::ptr::read_unaligned((clube + club::NATION) as *const usize);
    core::ptr::write_unaligned((p + 0x1a) as *mut usize, nacao);
    core::ptr::write_unaligned((p + 4) as *mut u32, primeiro_nome(nacao));
    let n2 = segundo_nome(core::ptr::read_unaligned((p + 0x1a) as *const usize));
    core::ptr::write_unaligned((p + 8) as *mut u32, n2);
    pos_humano(p, clube);
    1
}

// ponytail: cria o jogo inteiro (carrega o banco, sorteia, grava o .qst e libera tudo); não roda
// duas vezes. Só a original; a cópia leva o ramo e é provada pela prova do save (o .qst) e pelas
// carreiras (o auto_setup.cfg).
pub(crate) unsafe extern "C" fn comparar_create_game_from_setup_file(cfg: *const u8, qst: *const u8) -> u32 {
    orig::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(IDX_CREATE_GAME_FROM_SETUP_FILE)(cfg, qst)
}

// ---------- 220a leva: o save: gravar o jogo inteiro (save_game_data) (Setup.cpp) ----------

/// A caixa de erro da gravação: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de Setup.cpp,
/// `linha`, `texto`) em 0xae24d0; o título `titulo` traduzido (0x66f4e0) em 0xae28bc, o corpo
/// `corpo` (com o texto de 0xae24d0) traduzido em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está, ao contrário do `aviso_do_setup`).
pub(crate) unsafe fn caixa_do_save(linha: u32, texto: usize, titulo: usize, corpo: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, titulo);
    traduzir(0xae26c4, corpo, 0xae24d0);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// Os passos 2..=0x24 da gravação, na ordem da original: (global que tem de ser ≠ 0, rotina, como
/// chamar). `this` é o próprio global, menos no 0x5afad0 (FOG_OF_WAR), que a original confere por
/// [0xb6b7c8] (o de Media) e chama com [0xae2a60]. O passo 0x19 (0x6692f0, as competições) não
/// tem global e devolve 0 quando falha.
#[derive(Clone, Copy)]
pub(crate) enum PassoDoSave {
    /// thiscall(this, caminho), ret 4
    Caminho(usize, usize),
    /// thiscall(this), sem argumento
    Sozinho(usize, usize),
    /// thiscall(this, 1), ret 4 (FIXTURE_MANAGER::save_fixtures)
    Um(usize, usize),
    /// o método +0x1c da vtable de [global](1) (Friendly)
    Virtual(usize),
    /// confere o 1º global e chama a rotina com o 2º (FOG_OF_WAR)
    Outro(usize, usize, usize),
    /// cdecl() → 0 é erro (key_nation_save_all_comps)
    Competicoes,
}

pub(crate) const PASSOS_DO_SAVE: [PassoDoSave; 0x23] = [
    PassoDoSave::Caminho(0xb63c98, 0x5ff9e0), // 2 HUMAN_MANAGER
    PassoDoSave::Caminho(0xb74448, 0x7f8020), // 3 SCOUT_MANAGER
    PassoDoSave::Caminho(0xb7431c, 0x787c30), // 4 News
    PassoDoSave::Virtual(0xae2c74),           // 5 Friendly
    PassoDoSave::Sozinho(0xde1f58, 0x905510), // 6 UefaSeeding
    PassoDoSave::Sozinho(0xae1000, 0x4c6b70), // 7 ConmebolSeeding
    PassoDoSave::Caminho(0xdd7f00, 0x86ffd0), // 8 Staff_Records
    PassoDoSave::Caminho(0xb6b7c0, 0x72adf0), // 9 Match_Stats
    PassoDoSave::Caminho(0xb74350, 0x7c55a0), // 0xa PlayerStats
    PassoDoSave::Caminho(0xad9da8, 0x443f50), // 0xb Club_History
    PassoDoSave::Caminho(0xad9dac, 0x451ff0), // 0xc Club_Records
    PassoDoSave::Caminho(0xde1d08, 0x8995d0), // 0xd Tactics
    PassoDoSave::Sozinho(0xb5e134, 0x5fb320), // 0xe HostCountry
    PassoDoSave::Caminho(0xae28f0, 0x555dd0), // 0xf Discipline
    PassoDoSave::Caminho(0xad9c60, 0x4141a0), // 0x10 AWARD_MANAGER
    PassoDoSave::Caminho(0xae2a3c, 0x5a53d0), // 0x11 Fine
    PassoDoSave::Caminho(0xad9c6c, 0x418980), // 0x12 AWOL
    PassoDoSave::Caminho(0xae2a30, 0x569ae0), // 0x13 DISPUTE
    PassoDoSave::Caminho(0xb7433c, 0x794f00), // 0x14 Notes
    PassoDoSave::Caminho(0xb63d28, 0x62b2d0), // 0x15 Injury
    PassoDoSave::Caminho(0xb74318, 0x776a20), // 0x16 NationalTeams
    PassoDoSave::Sozinho(0xdd7ef4, 0x85bea0), // 0x17 Stadium
    PassoDoSave::Um(0xae2a58, 0x5ac060),      // 0x18 FIXTURE_MANAGER
    PassoDoSave::Competicoes,                 // 0x19 key_nation_save_all_comps
    PassoDoSave::Caminho(0xae1050, 0x4dc3a0), // 0x1a CONTRACT_MANAGER
    PassoDoSave::Caminho(0xae2a38, 0x59b3b0), // 0x1b FinanceTable
    PassoDoSave::Caminho(0xde1f54, 0x8bd4b0), // 0x1c TRANSFER_MANAGER
    PassoDoSave::Caminho(0xb7434c, 0x7ad7c0), // 0x1d PLAYER_REGEN
    PassoDoSave::Caminho(0xb6b7c8, 0x72b5f0), // 0x1e Media
    PassoDoSave::Outro(0xb6b7c8, 0xae2a60, 0x5afad0), // 0x1f FOG_OF_WAR
    PassoDoSave::Caminho(0xb67a44, 0x68e920), // 0x20 MANAGER_MANAGER
    PassoDoSave::Caminho(0xb74340, 0x79bb80), // 0x21 OfficialsManager
    PassoDoSave::Caminho(0xdd7ee4, 0x84c680), // 0x22 SimulatedStats
    PassoDoSave::Caminho(0xde1f44, 0x8b1450), // 0x23 TRAINING_MANAGER
    PassoDoSave::Caminho(0xdd7ef0, 0x859cc0), // 0x24 SQUAD_MANAGER
];

/// saves the game: deletes the old file, the database (0x524d30), general.dat (16 fields and the summary), the
/// subsystem saves in the original order with disk and memory checks between them, packs the *.tmp files, the FIFA
/// rules; on a network game, writes to temp_save_file and renames
/// Original: `save_game_data` (0x82e430, Setup.cpp, cdecl).
///
/// [pt] { 0x82e430 } (cdecl, Setup.cpp, ~6,8 KB, SEH): `save_game_data(caminho, a4)`: grava o jogo
/// inteiro no contêiner `caminho` (a4 0x16 `.sav`, 0x17 `.qst`; tipo 3 com o bit 0x8000 do
/// `game.cfg` [[0xae3138]+4], senão 4 = RLE). Devolve 1 se gravou.
/// - [0xdd7110] = 0xfffc1fb9 (a versão do save). Em rede (bit 0x20000000), antes de tudo, disco
///   (0x7a1820(0) < 25) liga [0xdd7134] e memória ([0x9cd1fc] e 0x7a1710() < 1) liga [0xdd7138]:
///   apaga `caminho` e devolve 0.
/// - Pasta temporária (0x90bde0 "CM3_TEMP"); `caminho` nulo dá a caixa "Error" (0x2958). O
///   caminho de trabalho é `caminho` ou, em rede, o mesmo com o nome `temp_save_file`
///   (0x944cff/0x945b13); se já existe (0x90c120) e não sai (0x94643a = −1), devolve 0. Com o bit
///   0x40000000, disco e memória de novo (apagando `caminho`).
/// - 0x524d30 (o banco, passo 1 do save); disco e memória (apagando o caminho de trabalho);
///   "Saving game data" traduzido e a barra 0x802f80 (passo 1 de 0x25).
/// - `general.dat` (0x9326e0): 16 campos (a versão, o resumo 0x591f80 de 0xf74 bytes, o id único,
///   o tamanho do banco, as datas e horas, as ligas, a nação padrão ou −1, os dados de execução de
///   pessoas, clubes e reputação); erro de escrita dá a caixa "Disk Space Error" com a linha do
///   campo e fecha o item.
/// - Os passos 2..=0x24 (`PASSOS_DO_SAVE`): disco e memória (antes do 0x11, sem apagar e com
///   memória < 25), a barra e o `save` do subsistema; 0x25: empacota os `*.tmp` (0x933610).
/// - Hall da fama (0x5f00c0, fora do save), regras FIFA (0x8395b0); em rede, troca o save antigo
///   pelo novo (0x945ba4); [0xdd7130] = 1, [0xa801bc] = 0 e a data do último save (0xdd70a0).
/// A data local da original ([esp+0x1758], 0x54a110, destrutor vazio 0x5a53c0 no SEH) não é usada:
/// fica de fora. O laço da original ([esp+0x11c] de 1 a 2) roda o corpo uma vez, com o total 0x25.
/// O resumo é um buffer zerado aqui; na original, o que o 0x591f80 não escreve (o clube dos 16
/// técnicos não humanos, o resto dos textos) é sobra da pilha dela, gravada no `general.dat`.
pub(crate) unsafe extern "C" fn save_game_data(caminho: *const u8, a4: u32) -> u32 {
    let disco: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_OS_GET_FREE_DISK_SPACE_MB);
    let memoria: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_OS_GET_FREE_MEMORY_MB);
    let apagar: unsafe extern "C" fn(*const u8) -> i32 = core::mem::transmute(FN_CRT_REMOVE);
    let pasta: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let existe: unsafe extern "C" fn(*const u8) -> i32 = core::mem::transmute(FN_UTILS_FILE_EXISTS);
    let separar: unsafe extern "C" fn(*const u8, *mut u8, *mut u8, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let juntar: unsafe extern "C" fn(*mut u8, *const u8, *const u8, usize, *const u8) = core::mem::transmute(FN_CRT_MAKEPATH);
    let renomear: unsafe extern "C" fn(*const u8, *const u8) -> i32 = core::mem::transmute(FN_CRT_RENAME);
    let gravar_banco: unsafe extern "C" fn(*const u8, u32, u32) -> u32 = core::mem::transmute(FN_SAVE_DATABASE_FOR_GAME);
    let traduzir: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let barra: unsafe extern "thiscall" fn(usize, *const u8, u32, u32, i32) = core::mem::transmute(FN_SCREENMANAGER_SEND_NETWORK_STATUS_MESSAGE);
    let abrir_item: unsafe extern "C" fn(*const u8, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let gravar: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    let montar_resumo: unsafe extern "C" fn(*mut u8) = core::mem::transmute(FN_BUILD_SAVE_GAME_SUMMARY);
    let empacotar: unsafe extern "C" fn(*const u8, *const u8, usize, u32, u32) -> i32 = core::mem::transmute(FN_ZADD_MATCHING_FILES);
    let competicoes: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_KEY_NATION_SAVE_ALL_COMPS);
    let hall_da_fama: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_HALLOFFAME_SAVE);
    let regras_fifa: unsafe extern "C" fn(*const u8) = core::mem::transmute(FN_SAVE_FIFA_TRANSFER_RULES);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    crate::comparacao::ramo("save gravar jogo");
    let opcoes = || *((*(G_CONFIG as *const usize) + 4) as *const u32);
    let tipo = || if opcoes() & 0x8000 != 0 { 3u32 } else { 4 };
    let sem_memoria = || *(G_MEMORY_CHECK_ENABLED as *const u32) != 0 && memoria() < 1;
    *(G_GAME_DATA_VERSION as *mut u32) = 0xfffc1fb9;
    if opcoes() & 0x20000000 != 0 {
        let falta = if disco(0) < 0x19 {
            *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
            true
        } else if sem_memoria() {
            *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
            true
        } else {
            false
        };
        if falta {
            if !caminho.is_null() {
                apagar(caminho);
            }
            return 0;
        }
    }
    let mut temporaria = [0u8; 0x104];
    copiar_texto(temporaria.as_mut_ptr(), pasta(0x987da0, 0xad9c64));
    if caminho.is_null() {
        aviso_do_setup(0x2958);
        return 0;
    }
    let mut local = [0u8; 0x104];
    if opcoes() & 0x20000000 != 0 {
        let (mut unidade, mut dir, mut nome, mut ext) = ([0u8; 0x104], [0u8; 0x104], [0u8; 0x100], [0u8; 0x100]);
        separar(caminho, unidade.as_mut_ptr(), dir.as_mut_ptr(), nome.as_mut_ptr(), ext.as_mut_ptr());
        juntar(local.as_mut_ptr(), unidade.as_ptr(), dir.as_ptr(), 0xa81cf4, ext.as_ptr());
    } else {
        copiar_texto(local.as_mut_ptr(), caminho);
    }
    let l = local.as_ptr();
    if existe(l) != 0 && apagar(l) == -1 {
        return 0;
    }
    if opcoes() & 0x40000000 != 0 {
        if disco(0) < 0x19 {
            *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
            apagar(caminho);
            return 0;
        }
        if sem_memoria() {
            *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
            apagar(caminho);
            return 0;
        }
    }
    gravar_banco(l, a4, tipo());
    // disco e memória entre os passos: liga a marca, apaga o caminho de trabalho e devolve 0
    let falta_espaco = || {
        if disco(0) < 0x19 {
            *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
            apagar(l);
            return true;
        }
        if sem_memoria() {
            *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
            apagar(l);
            return true;
        }
        false
    };
    if falta_espaco() {
        return 0;
    }
    let mut texto = [0u8; 0xc8];
    traduzir(texto.as_mut_ptr(), 0xa81ce0);
    barra(0xb74450, texto.as_ptr(), 1, 0x25, -1);
    let mut z = abrir_item(l, 0x9c7b84, 1, a4, tipo());
    if z == 0 {
        caixa_do_save(0x29a3, 0xa81c88, 0xa81c70, 0xa81b5c);
        return 0;
    }
    // general.dat: (endereço, tamanho, quantidade, linha e texto do erro); o resumo e a nação
    // padrão saem de buffers locais; os três últimos gravam vetores e conferem a quantidade
    let mut resumo = [0u8; 0xf74];
    let mut campo = |buf: usize, tam: u32, n: u32, linha: u32, msg: usize| -> bool {
        if gravar(buf, tam, n, z) == n {
            return true;
        }
        caixa_do_save(linha, msg, 0x9c7774, 0x9c7680);
        fechar_item(&mut z);
        false
    };
    let u = |a: usize| *(a as *const u32);
    if !campo(0xdd7110, 4, 1, 0x29aa, 0xa81b1c) {
        return 0;
    }
    montar_resumo(resumo.as_mut_ptr());
    if !campo(resumo.as_ptr() as usize, 0xf74, 1, 0x29b5, 0xa81ae0)
        || !campo(0xdd7120, 4, 1, 0x29bd, 0xa81aa4)
        || !campo(0xdd7114, 1, 1, 0x29c5, 0xa81a6c)
        || !campo(0xdd6ec8, 8, 1, 0x29cd, 0xa81a3c)
        || !campo(0xae2c90, 8, 1, 0x29d5, 0xa81a3c)
        || !campo(0xae2c88, 1, 1, 0x29dd, 0xa81a04)
        || !campo(0xae2c78, 8, 1, 0x29e5, 0xa819d0)
        || !campo(0xae2cfc, 1, 1, 0x29ed, 0xa81994)
        || !campo(0xae2d00, 4, 1, 0x29f5, 0xa8194c)
        || !campo(0xae2d04, 4, 1, 0x29fd, 0xa818fc)
    {
        return 0;
    }
    let padrao = u(G_MAIN_NATION);
    let nacao_padrao: u32 = if padrao != 0 { u(padrao as usize) } else { 0xffffffff };
    if !campo(&nacao_padrao as *const u32 as usize, 4, 1, 0x2a06, 0xa818c0)
        || !campo(0xae2d10, 4, 1, 0x2a0e, 0xa81880)
        || !campo(u(G_STAFF_STATE_TABLE) as usize, 0x4f, u(G_STAFF_STATE_COUNT), 0x2a14, 0xa81848)
        || !campo(u(G_CLUB_STATE_TABLE) as usize, 6, u(G_CLUB_STATE_COUNT), 0x2a1c, 0xa81814)
        || !campo(u(G_CLUB_REPUTATION_ORDER) as usize, 4, u(G_CLUB_COUNT), 0x2a24, 0xa817dc)
    {
        return 0;
    }
    fechar_item(&mut z);
    for (i, passo) in PASSOS_DO_SAVE.iter().enumerate() {
        let k = i as u32 + 2;
        // antes do passo 0x11 (depois do AWARD_MANAGER) a original não apaga o caminho e pede
        // memória >= 25 (e não >= 1)
        if k == 0x11 {
            if disco(0) < 0x19 {
                *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
                return 0;
            }
            if *(G_MEMORY_CHECK_ENABLED as *const u32) != 0 && memoria() < 0x19 {
                *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
                return 0;
            }
        } else if falta_espaco() {
            return 0;
        }
        barra(0xb74450, texto.as_ptr(), k, 0x25, -1);
        match *passo {
            PassoDoSave::Caminho(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(f);
                    f(u(g) as usize, l);
                }
            }
            PassoDoSave::Sozinho(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
                    f(u(g) as usize);
                }
            }
            PassoDoSave::Um(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
                    f(u(g) as usize, 1);
                }
            }
            PassoDoSave::Virtual(g) => {
                let o = u(g) as usize;
                if o != 0 {
                    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(*((*(o as *const usize) + 0x1c) as *const usize));
                    f(o, 1);
                }
            }
            PassoDoSave::Outro(g, this, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(f);
                    f(u(this) as usize, l);
                }
            }
            PassoDoSave::Competicoes => {
                if competicoes() == 0 {
                    aviso_do_setup(0x2baa);
                    return 0;
                }
            }
        }
    }
    if falta_espaco() {
        return 0;
    }
    barra(0xb74450, texto.as_ptr(), 0x25, 0x25, -1);
    let (t, linha) = if opcoes() & 0x8000 != 0 { (3, 0x2c82) } else { (4, 0x2c8a) };
    if empacotar(l, temporaria.as_ptr(), 0xa813ac, a4, t) == 0 {
        caixa_do_save(linha, 0xa817a4, 0x9c7774, 0x9c7680);
        return 0;
    }
    hall_da_fama(u(G_HALL_OF_FAME) as usize);
    regras_fifa(l);
    if opcoes() & 0x20000000 != 0 {
        if existe(caminho) != 0 && apagar(caminho) == -1 {
            caixa_do_save(0x2ca7, 0xa81cc0, 0x9af90c, 0x9af828);
        }
        renomear(l, caminho);
    }
    *(G_GAME_HAS_SAVE_FILE_FLAG as *mut u32) = 1;
    *(G_GAME_UNSAVED_CHANGES_FLAG as *mut u32) = 0;
    let mut data = [0u8; 8];
    copia_data(0xdd70a0, data.as_mut_ptr(), 0xae2c90);
    1
}

// ponytail: grava o save inteiro (apaga o antigo, cria o contêiner, empacota a pasta temporária);
// não roda duas vezes. Só a original; a cópia leva o ramo e é provada pela prova do save.
pub(crate) unsafe extern "C" fn comparar_save_game_data(caminho: *const u8, a4: u32) -> u32 {
    orig::<unsafe extern "C" fn(*const u8, u32) -> u32>(IDX_SAVE_GAME_DATA)(caminho, a4)
}

// ---------- 219a leva: o save: os filtros da caixa de arquivos (o jogo salvo, o início rápido e a versão) (Setup.cpp) ----------

/// file box filter for saved games: 1 if the container has id 0x16 (.sav), else 2 (ax only); a null path gives the
/// error box
/// Original: `check_saved_game_file` (0x838e10, Setup.cpp, cdecl).
///
/// [pt] { 0x838e10 } (cdecl, Setup.cpp): `check_saved_game_file(caminho)`: filtro da caixa de arquivos
/// dos jogos salvos: 1 se o contêiner tem o id 0x16 (`.sav`, 0x9331c0), senão 2. A original só
/// acerta `ax`: os 16 bits altos de `eax` ficam os da resposta do id, como aqui. Caminho nulo é o
/// erro de Setup.cpp 0x3a1b (caixa "Error"): a cópia chama a original.
pub(crate) unsafe extern "C" fn check_saved_game_file(caminho: usize) -> u32 {
    if caminho == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_SAVED_GAME_FILE)(caminho);
    }
    let zget_archive_id: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_ZGET_ARCHIVE_ID);
    let id = zget_archive_id(caminho);
    (id & 0xffff0000) | if id != 0x16 { 2 } else { 1 }
}

// O id só lê o arquivo e libera o que aloca: as duas rodam lado a lado. Caminho nulo (caixa de
// erro) só a original.
pub(crate) unsafe extern "C" fn comparar_check_saved_game_file(caminho: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_SAVED_GAME_FILE)(caminho);
    if caminho == 0 {
        return r;
    }
    let n = check_saved_game_file(caminho);
    PLACAR.registrar(r == n, || format!("check_saved_game_file {:#x} x {:#x}", r, n));
    r
}

/// file box filter for quick starts: 1 if the container has id 0x17 (.qst) and the saved version is the exe one, else 2
/// (ax only)
/// Original: `check_quick_start_file` (0x839290, Setup.cpp, cdecl).
///
/// [pt] { 0x839290 } (cdecl, Setup.cpp): `check_quick_start_file(caminho)`: filtro da caixa de arquivos
/// do início rápido: 1 se o contêiner tem o id 0x17 (`.qst`) e a versão gravada é a do exe
/// (0x838eb0), senão 2. Os 16 bits altos de `eax` são os da última chamada (o id, ou a versão).
/// Caminho nulo: erro de Setup.cpp 0x3a6d, a cópia chama a original.
pub(crate) unsafe extern "C" fn check_quick_start_file(caminho: usize) -> u32 {
    if caminho == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_QUICK_START_FILE)(caminho);
    }
    let zget_archive_id: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_ZGET_ARCHIVE_ID);
    let versao_valida: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_VALID_SAVED_GAME_VERSION);
    crate::comparacao::ramo("save filtro do inicio rapido");
    let id = zget_archive_id(caminho);
    if id != 0x17 {
        return (id & 0xffff0000) | 2;
    }
    let v = versao_valida(caminho);
    (v & 0xffff0000) | if v != 0 { 1 } else { 2 }
}

// ponytail: a versão (0x838eb0) abre o general.dat e não o fecha quando dá certo (o ZFILE e o
// arquivo ficam abertos, como na original): rodar duas vezes deixaria dois. Só a original; a cópia
// leva o ramo e roda no modo Rust (só a caixa de arquivos chama).
pub(crate) unsafe extern "C" fn comparar_check_quick_start_file(caminho: usize) -> u32 {
    orig::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_QUICK_START_FILE)(caminho)
}

/// A caixa "Load Game Error" de `valid_saved_game_version`: o texto "v%s %s.%s %d (%s)" (versão
/// 0x90d130, arquivo e extensão de Setup.cpp, linha, motivo) em 0xae24d0, o título traduzido em
/// 0xae28bc, a mensagem traduzida (0x9af334) em 0xae26c4 e a caixa 0x5e8290. Ao contrário do
/// `aviso_em`, não zera [0xb67a34].
pub(crate) unsafe fn erro_de_carga_do_setup(linha: u32, motivo: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    // a 0x90d130 ignora os argumentos (devolve a versão); o sprintf da original lê nome, extensão,
    // linha e motivo da pilha que ficou da chamada: aqui vão explícitos
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, motivo);
    traduzir(0xae28bc, 0x9af3e0);
    traduzir(0xae26c4, 0x9af334, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// opens the container general.dat, reads the version and the summary (0xf70 or 0xf74 bytes) and returns 1 if the
/// summary version text is the exe one; on success it does not close the item (as the original)
/// Original: `valid_saved_game_version` (0x838eb0, Setup.cpp, cdecl).
///
/// [pt] { 0x838eb0 } (cdecl, Setup.cpp): `valid_saved_game_version(caminho)`: abre o item `general.dat`
/// do contêiner (0x9326e0 lendo, a4 0x15, tipo 4), lê a versão gravada (4 bytes) e o resumo do save
/// (0xf70 bytes se |versão| < 0x3e027, senão 0xf74) e devolve 1 se o texto da versão do resumo
/// (+0x7d0) é igual ao do exe (0x90d130), senão 0. Cada leitura que falha dá a caixa "Load Game
/// Error" (Setup.cpp 0x3a36, 0x3a3d, 0x3a48, 0x3a55) e 0. Como a original, quando dá certo o item
/// NÃO é fechado (o ZFILE e o arquivo ficam abertos). O quadro SEH da original só protege a data
/// local (0x54a110, sem destrutor), que cai dentro do resumo (+0xf64) e é sobrescrita pela leitura.
pub(crate) unsafe extern "C" fn valid_saved_game_version(caminho: usize) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let ler_item: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_ZREAD);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    let versao: unsafe extern "C" fn() -> *const u8 = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    crate::comparacao::ramo("save versao do jogo salvo");
    let mut resumo = [0u8; 0xf74];
    let mut v = 0i32;
    data_padrao(resumo.as_mut_ptr() as usize + 0xf64);
    let mut z = abrir_item(caminho, 0x9c7b84, 0, 0x15, 4);
    if z == 0 {
        erro_de_carga_do_setup(0x3a36, 0xa82494);
        return 0;
    }
    if ler_item(&mut v as *mut i32 as usize, 4, 1, z) != 1 {
        erro_de_carga_do_setup(0x3a3d, 0xa8244c);
        fechar_item(&mut z);
        return 0;
    }
    if v.wrapping_abs() < 0x3e027 {
        if ler_item(resumo.as_mut_ptr() as usize, 0xf70, 1, z) != 1 {
            erro_de_carga_do_setup(0x3a48, 0xa82408);
            fechar_item(&mut z);
            return 0;
        }
        // o resumo antigo não tem o último campo (+0xf70): a original o zera na pilha
        resumo[0xf70..0xf74].fill(0);
    } else if ler_item(resumo.as_mut_ptr() as usize, 0xf74, 1, z) != 1 {
        erro_de_carga_do_setup(0x3a55, 0xa82408);
        fechar_item(&mut z);
        return 0;
    }
    let exe = core::ffi::CStr::from_ptr(versao() as *const core::ffi::c_char).to_bytes();
    let gravada = core::ffi::CStr::from_ptr(resumo.as_ptr().add(0x7d0) as *const core::ffi::c_char).to_bytes();
    (exe == gravada) as u32
}

// ponytail: abre o item e, quando dá certo, não o fecha (o ZFILE e o arquivo ficam abertos, como na
// original); nos erros, a caixa. Não roda duas vezes: só a original; a cópia leva o ramo e roda no
// modo Rust (só a caixa de arquivos, pelo filtro do início rápido, chama).
pub(crate) unsafe extern "C" fn comparar_valid_saved_game_version(caminho: usize) -> u32 {
    orig::<unsafe extern "C" fn(usize) -> u32>(IDX_VALID_SAVED_GAME_VERSION)(caminho)
}

// ---------- 219a leva: o save: as regras FIFA de transferência (gravar e carregar) e o resumo do save (Setup.cpp, file_screens.cpp) ----------

/// A caixa "Error" das regras FIFA: o texto "v%s %s.%s %d" (versão 0x90d130, arquivo e extensão de
/// Setup.cpp, linha) em 0xae24d0, a caixa 0x5e8290 e [0xb67a34] = 0. Como o `aviso_em`, mas com os
/// argumentos que o sprintf da original lê da pilha (nome, extensão e linha) passados explícitos.
pub(crate) unsafe fn aviso_do_setup(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
}

/// writes the last saved game item, fifa_transfer_rules.zip: 2 bytes ([0xae233c] and the rule in force [0xa83608]);
/// errors give the "Error" box
/// Original: `save_fifa_transfer_rules` (0x8395b0, Setup.cpp, cdecl).
///
/// [pt] { 0x8395b0 } (cdecl, Setup.cpp): `save_fifa_transfer_rules(save)`: o último item do save,
/// `fifa_transfer_rules.zip` (0x9326e0 gravando, a4 0x16, tipo 4; no contêiner já criado vale o
/// cabeçalho dele): 2 bytes, [0xae233c] e [0xa83608], e `zclose`. Caminho nulo, item que não abre
/// ou escrita que falha: a caixa "Error" (Setup.cpp 0x3b24, 0x3b2c, 0x3b32, 0x3b38); nas escritas,
/// depois da caixa o item é fechado. A original não devolve nada (`eax` sobra).
pub(crate) unsafe extern "C" fn save_fifa_transfer_rules(save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let gravar: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_ZWRITE);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    crate::comparacao::ramo("save gravar regras fifa");
    if save == 0 {
        aviso_do_setup(0x3b24);
        return;
    }
    let mut z = abrir_item(save, 0xa82518, 1, 0x16, 4);
    if z == 0 {
        aviso_do_setup(0x3b2c);
        return;
    }
    if gravar(0xae233c, 1, 1, z) != 1 {
        aviso_do_setup(0x3b32);
        fechar_item(&mut z);
        return;
    }
    if gravar(0xa83608, 1, 1, z) != 1 {
        aviso_do_setup(0x3b38);
        fechar_item(&mut z);
        return;
    }
    fechar_item(&mut z);
}

// ponytail: acrescenta um item ao contêiner; não roda duas vezes. Só a original; a cópia leva o
// ramo e é provada pela prova do save (o .qst gravado com e sem a leva).
pub(crate) unsafe extern "C" fn comparar_save_fifa_transfer_rules(save: usize) {
    orig::<unsafe extern "C" fn(usize)>(IDX_SAVE_FIFA_TRANSFER_RULES)(save)
}

/// reads the 2 bytes of fifa_transfer_rules.zip; if the first is 1, the rule in force becomes 5
/// Original: `load_fifa_transfer_rules` (0x839760, Setup.cpp, cdecl).
///
/// [pt] { 0x839760 } (cdecl, Setup.cpp): `load_fifa_transfer_rules(save)`: lê os 2 bytes do item
/// `fifa_transfer_rules.zip` (0x9326e0 lendo) em [0xae233c] e [0xa83608]; se [0xae233c] = 1, a
/// regra em vigor [0xa83608] vira 5. Fecha o item. Erros: a caixa "Error" (Setup.cpp 0x3b4f, 0x3b57,
/// 0x3b5d, 0x3b63), como na gravação.
pub(crate) unsafe extern "C" fn load_fifa_transfer_rules(save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let ler_item: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_ZREAD);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    crate::comparacao::ramo("save carregar regras fifa");
    if save == 0 {
        aviso_do_setup(0x3b4f);
        return;
    }
    let mut z = abrir_item(save, 0xa82518, 0, 0x16, 4);
    if z == 0 {
        aviso_do_setup(0x3b57);
        return;
    }
    if ler_item(0xae233c, 1, 1, z) != 1 {
        aviso_do_setup(0x3b5d);
        fechar_item(&mut z);
        return;
    }
    if ler_item(0xa83608, 1, 1, z) != 1 {
        aviso_do_setup(0x3b63);
        fechar_item(&mut z);
        return;
    }
    if *(G_FIFA_TRANSFER_RULES_IN_FORCE as *const u8) == 1 {
        *(G_MAX_CONTRACT_LENGTH_YEARS as *mut u8) = 5;
    }
    fechar_item(&mut z);
}

// ponytail: abre e lê o contêiner (aloca o ZFILE) e só roda na carga; segue a regra do bloco do save
// (arquivo = só a original). A cópia leva o ramo; a etapa de carregar da prova do save a exercita.
pub(crate) unsafe extern "C" fn comparar_load_fifa_transfer_rules(save: usize) {
    orig::<unsafe extern "C" fn(usize)>(IDX_LOAD_FIFA_TRANSFER_RULES)(save)
}

/// O `strcat` da original (repne scasb + rep movs): `fonte` com o zero no fim de `destino`.
pub(crate) unsafe fn emendar_texto(destino: *mut u8, fonte: *const u8) {
    let d = destino.add(core::ffi::CStr::from_ptr(destino as *const core::ffi::c_char).to_bytes().len());
    copiar_texto(d, fonte);
}

/// O `strcpy` da original, com o zero.
pub(crate) unsafe fn copiar_texto(destino: *mut u8, fonte: *const u8) {
    let n = core::ffi::CStr::from_ptr(fonte as *const core::ffi::c_char).to_bytes_with_nul().len();
    core::ptr::copy(fonte, destino, n);
}

/// builds the general.dat summary: the active nations (two groups), the human managers with club and nation, the game
/// date, the version, the play time and time()
/// Original: `build_save_game_summary` (0x591f80, file_screens.cpp, cdecl).
///
/// [pt] { 0x591f80 } (cdecl, file_screens.cpp): `build_save_game_summary(resumo)`: o resumo que o
/// `save_game_data` grava no `general.dat` (logo depois da versão):
/// - +0: as nações da tabela de ligas (0xb63d60, 0x23 de 0x48 bytes) com o bit 2 em +0x11c, a
///   primeira pelo sprintf com o próprio nome como formato (0x944c9f), as outras por ", <%s>"
///   traduzido (0x66f4e0); +1000 (0x3e8): o mesmo com o bit 1;
/// - 16 técnicos, do último da tabela de pessoas (0xae23bc, 0x6e bytes) para trás: humano (+0x5f
///   = 1) dá o nome em +0x7e4 + 0x78 × i (0x53dba0, 200) e em +0x820 + 0x78 × i "clube / nação"
///   (+0x39 e +0x24, nome em +0x38 de cada), só um deles, ou nada; não humano só zera o nome (o
///   clube fica como estava no buffer do chamador);
/// - +0xf64 a data do jogo (0x417c10 de 0xae2c90), +0x7d0 a versão (0x90d130), +0xf6c o tempo de
///   jogo ([0xdd7140] += time() − [0xdd7144]; [0xdd7144] = time()) e +0xf70 o time().
pub(crate) unsafe extern "C" fn build_save_game_summary(resumo: *mut u8) {
    let formatar: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzir: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let nome_do: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let versao: unsafe extern "C" fn() -> *const u8 = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let relogio: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(FN_946EBB);
    let mut nacao = [0u8; 100];
    *resumo = 0;
    *resumo.add(0x3e8) = 0;
    for (bit, destino) in [(2u8, resumo), (1u8, resumo.add(0x3e8))] {
        let mut primeira = true;
        for i in 0..0x23usize {
            let n = *((0xb63d60 + i * 0x48) as *const usize);
            if n == 0 || *((n + 0x11c) as *const u8) & bit == 0 {
                continue;
            }
            if primeira {
                formatar(nacao.as_mut_ptr(), n + 0x38);
                primeira = false;
            } else {
                traduzir(nacao.as_mut_ptr(), 0x9c8134, n + 0x38);
            }
            emendar_texto(destino, nacao.as_ptr());
        }
        if bit == 1 && primeira {
            *resumo.add(0x3e8) = 0;
        }
    }
    let quantos = *(G_STAFF_COUNT as *const i32);
    let tabela = *(G_STAFF_TABLE as *const usize);
    for i in 0..0x10i32 {
        let clube_txt = resumo.add(0x820 + 0x78 * i as usize);
        let p = tabela.wrapping_add((quantos - i - 1).wrapping_mul(0x6e) as usize);
        if *((p + 0x5f) as *const u8) != 1 {
            *clube_txt.sub(0x3c) = 0;
            continue;
        }
        nome_do(p, clube_txt.sub(0x3c), 200, 1);
        let clube = core::ptr::read_unaligned((p + 0x39) as *const usize);
        let nacao_do = core::ptr::read_unaligned((p + 0x24) as *const usize);
        if clube != 0 && nacao_do != 0 {
            traduzir(clube_txt, 0x9c80f4, clube + club::SHORT_NAME, nacao_do + nation::SHORT_NAME);
        } else if clube != 0 {
            copiar_texto(clube_txt, (clube + club::SHORT_NAME) as *const u8);
        } else if nacao_do != 0 {
            copiar_texto(clube_txt, (nacao_do + nation::SHORT_NAME) as *const u8);
        } else {
            *clube_txt = 0;
        }
    }
    let mut copia = [0u8; 8];
    copia_data(resumo as usize + 0xf64, copia.as_mut_ptr(), 0xae2c90);
    copiar_texto(resumo.add(0x7d0), versao());
    let mut agora = 0i32;
    let t = relogio(&mut agora);
    let total = (*(G_GAME_PLAY_TIME_TOTAL as *const i32)).wrapping_add(t.wrapping_sub(*(G_GAME_PLAY_TIME_LAST_STAMP as *const i32)));
    *(G_GAME_PLAY_TIME_TOTAL as *mut i32) = total;
    core::ptr::write_unaligned(resumo.add(0xf6c) as *mut i32, total);
    *(G_GAME_PLAY_TIME_LAST_STAMP as *mut i32) = t;
    let t2 = relogio(core::ptr::null_mut());
    core::ptr::write_unaligned(resumo.add(0xf70) as *mut i32, t2);
}

// Lado a lado sobre o resumo inteiro (0xf74) e o relógio do tempo de jogo ([0xdd7140], [0xdd7144]),
// restaurados entre as duas; compara só +0..+0xf6c: o tempo de jogo e o time() (+0xf6c, +0xf70) são
// relógio (fixos só no modo simulação), como na prova do save.
pub(crate) unsafe extern "C" fn comparar_build_save_game_summary(resumo: *mut u8) {
    let regioes = [(resumo as usize, 0xf6c), (resumo as usize + 0xf6c, 8), (0xdd7140, 8)];
    let (_, _, _, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || orig::<unsafe extern "C" fn(*mut u8)>(IDX_BUILD_SAVE_GAME_SUMMARY)(resumo),
        || build_save_game_summary(resumo));
    PLACAR.registrar(mo.0[0] == mr.0[0], || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("build_save_game_summary: primeira diferença em +{:#x}: {:#04x} x {:#04x}", k, a[k], b[k])
    });
}

/// releases the screens, destroys the subsystems held in globals (destructor + delete, free, virtual destructor) in the
/// original order, deletes the *.tmp files, frees the database tables, resets the game date and clears three flags;
/// returns 1
/// Original: `free_game_data` (0x82ff40, Setup.cpp, cdecl).
///
/// [pt] { 0x82ff40 } (cdecl, Setup.cpp): `free_game_data(tela)`: o gerente de telas (0xb74450) solta as
/// telas (0x7ff5f0) e, se `tela` ≠ 0, mais 0x7fc5e0; 0x6d5b50; depois cada subsistema guardado num
/// global, na ordem da original: destrutor (thiscall) + `delete` (0x944c94) e o global zerado; os
/// vetores soltos pelo `free` (0x9452ca); dois objetos pelo destrutor virtual (vtable[0](1)); os
/// `*.tmp` da pasta temporária (0x90cd50), [0xae2338], as tabelas do banco (0x528080), a data do
/// jogo (0x54a130: 1, 0, 2001, −1) e três marcas zeradas. Devolve 1.
pub(crate) unsafe extern "C" fn free_game_data(tela: u32) -> u32 {
    let soltar_telas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SCREENMANAGER_END_NETWORK_SESSION);
    let soltar_mais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SCREENMANAGER_CLOSE_ALL_SCREENS);
    let antes: unsafe extern "C" fn() = core::mem::transmute(FN_MATCH_EVENTS_FREE_EVENT_TABLES);
    let competicoes: unsafe extern "C" fn() = core::mem::transmute(FN_KEY_NATION_DELETE_ALL_COMPS);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let livre: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let limpar_temporarios: unsafe extern "C" fn(usize) = core::mem::transmute(FN_UTILS_CLEAR_TEMP_FOLDER);
    let liberar_tabelas: unsafe extern "C" fn() = core::mem::transmute(FN_DATABASE_FREE_ALL_TABLES);
    let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_SET);
    crate::comparacao::ramo("save liberar dados do jogo");
    // destrutor + delete + global zerado
    let objetos = |lista: &[(usize, usize)]| {
        for &(g, destrutor) in lista {
            let p = *(g as *const usize);
            if p != 0 {
                let d: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destrutor);
                d(p);
                apagar(p);
                *(g as *mut usize) = 0;
            }
        }
    };
    // free + global zerado
    let soltos = |lista: &[usize]| {
        for &g in lista {
            let p = *(g as *const usize);
            if p != 0 {
                livre(p);
                *(g as *mut usize) = 0;
            }
        }
    };
    // destrutor virtual (vtable[0], com 1 = apagar) + global zerado
    let virtual_ = |g: usize| {
        let p = *(g as *const usize);
        if p != 0 {
            let d: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(**(p as *const *const usize));
            d(p, 1);
            *(g as *mut usize) = 0;
        }
    };
    soltar_telas(0xb74450);
    if tela != 0 {
        soltar_mais(0xb74450);
    }
    antes();
    objetos(&[(0xb67a50, 0x6b4190)]);
    let p = *(G_COMP_OBJECT_TABLE as *const usize);
    if p != 0 {
        livre(p);
        *(G_COMP_OBJECT_TABLE as *mut usize) = 0;
        *(G_COMP_OBJECT_COUNT as *mut usize) = 0;
    }
    competicoes();
    objetos(&[(0xb6b7c8, 0x72b200), (0xae2a60, 0x5af6a0), (0xb67a44, 0x68e0e0), (0xad9da8, 0x442e20),
              (0xad9dac, 0x4448a0), (0xde1d08, 0x8961e0), (0xae28f0, 0x555790), (0xde1f44, 0x8b1070),
              (0xad9c60, 0x413410), (0xde1f54, 0x8bbe20), (0xb63d28, 0x62a8a0), (0xae2a3c, 0x5a5120),
              (0xad9c6c, 0x4185f0), (0xae2a30, 0x5698d0), (0xb7433c, 0x794ba0), (0xdd7ef4, 0x85b7f0)]);
    virtual_(0xae2c74);
    objetos(&[(0xb74318, 0x76a710), (0xde2f70, 0x90d2e0), (0xde1f58, 0x903570), (0xae1000, 0x5a8f60),
              (0xdd7f00, 0x8687c0), (0xb6b7c0, 0x7293c0), (0xb74350, 0x7c0d00), (0xb5e134, 0x58e330),
              (0xae2a58, 0x5aa0d0), (0xb63c98, 0x5fb730), (0xb74448, 0x7f6aa0), (0xb7431c, 0x785d00)]);
    // [0xdd7ef0]: só o delete, sem destrutor
    let p = *(G_SQUAD_MANAGER as *const usize);
    if p != 0 {
        apagar(p);
        *(G_SQUAD_MANAGER as *mut usize) = 0;
    }
    virtual_(0xb74434);
    objetos(&[(0xae1050, 0x4c74f0), (0xae2a38, 0x59a5a0)]);
    soltos(&[0xae2d0c, 0xae2d14, 0xae2d1c, 0xae2d20, 0xae2d24]);
    objetos(&[(0xb7434c, 0x7a56f0), (0xb74340, 0x79b800), (0xdd7ee4, 0x84be10), (0xde2f74, 0x9227d0),
              (0xb74430, 0x7da690)]);
    limpar_temporarios(0xa813ac);
    soltos(&[0xae2338]);
    liberar_tabelas();
    acertar_data(0xae2c90, 1, 0, 0x7d1, -1);
    *(G_CURRENT_DAY_SLICE as *mut u8) = 0;
    *(G_GAME_DATA_LOADED_FLAG as *mut u32) = 0;
    *(G_GAME_HAS_SAVE_FILE_FLAG as *mut u32) = 0;
    1
}

pub(crate) unsafe extern "C" fn comparar_free_game_data(tela: u32) -> u32 {
    orig::<unsafe extern "C" fn(u32) -> u32>(IDX_FREE_GAME_DATA)(tela)
}

/// the heading text colour (two colours by the flag)
/// Original: `get_heading_colour` (0x839590, Setup.cpp, cdecl).
///
/// [pt] { 0x839590 } (cdecl, Setup.cpp): a cor do título: [0xae3184] com `x` ≠ 0, senão [0xae31a8]. A
/// original só troca `ax`: os 16 bits de cima da devolução são os de `x`.
pub(crate) unsafe extern "C" fn get_heading_colour(x: u32) -> u32 {
    let c = if x != 0 { *(G_GUI_COLOUR_PALE_GREY as *const u16) } else { *(G_GUI_COLOUR_RED as *const u16) };
    (x & 0xffff_0000) | c as u32
}

pub(crate) unsafe extern "C" fn comparar_get_heading_colour(x: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(u32) -> u32>(IDX_GET_HEADING_COLOUR)(x);
    let n = get_heading_colour(x);
    PLACAR.registrar(r == n, || format!("get_heading_colour({:#x}) original={:#x} rust={:#x}", x, r, n));
    r
}
