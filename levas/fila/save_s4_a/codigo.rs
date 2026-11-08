// ---------- %%LEVA%%a leva: o save: carregar o jogo inteiro (load_game_data) (Setup.cpp) ----------

/// A caixa "Error" da carga sem volta (memória para os dados de execução): `setup_warning(linha)`,
/// 0x5cc7d0, 0x5a8f60 e `exit(-1)` (0x945934).
unsafe fn load_fatal_error(linha: u32) -> ! {
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
    setup_warning(linha);
    fechar1();
    fechar2();
    sair(-1);
    loop {}
}

/// A leitura do `general.dat` que falhou: a caixa "Load Game Error" com a linha e o texto (como a
/// `save_error_box`) e o item fechado (0x932b00). Devolve 0, a resposta da carga.
unsafe fn load_read_error(linha: u32, texto: usize, z: &mut usize) -> u32 {
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    save_error_box(linha, texto, 0x9af3e0, 0x9af334);
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
unsafe fn convert_staff_state(fonte: usize, tam: usize, n: i32) {
    let d = || *(0xae2d0c as *const usize);
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
enum SubsystemCtorArg {
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
enum LoadStep {
    Rotina(usize, u32),
    Objeto(u32, usize, usize, u32, SubsystemCtorArg),
}

/// Os passos 4..=0x2d da carga, na ordem da original, e o último (PunditList), que repete o 0x2d.
const LOAD_STEPS: [LoadStep; 0x2b] = [
    LoadStep::Rotina(0x539750, 0),                                      // 4
    LoadStep::Rotina(0x667150, 0x2555),                                 // 5 nações-chave
    LoadStep::Rotina(0x669140, 0x256c),                                 // 6
    LoadStep::Objeto(0x609, 0x893dc0, 0xde1d08, 0x2581, SubsystemCtorArg::Caminho), // 7 Tactics
    LoadStep::Objeto(0x10, 0x5fb430, 0xb63c98, 0x2596, SubsystemCtorArg::Caminho),  // 8 HUMAN_MANAGER
    LoadStep::Objeto(0xe8, 0x7f6760, 0xb74448, 0x25d0, SubsystemCtorArg::Caminho),  // 9 SCOUT_MANAGER
    LoadStep::Objeto(0x14, 0x785b20, 0xb7431c, 0x25e5, SubsystemCtorArg::Caminho),  // 0xa News
    LoadStep::Objeto(0x3507, 0x5c1f80, 0xae2c74, 0x25fa, SubsystemCtorArg::Sozinho), // 0xb Friendly
    LoadStep::Objeto(0x698, 0x903360, 0xde1f58, 0x260f, SubsystemCtorArg::Sozinho),  // 0xc UefaSeeding
    LoadStep::Objeto(0x75, 0x4c5fe0, 0xae1000, 0x2624, SubsystemCtorArg::Sozinho),   // 0xd ConmebolSeeding
    LoadStep::Objeto(0x18, 0x868390, 0xdd7f00, 0x2639, SubsystemCtorArg::Caminho),   // 0xe Staff_Records
    LoadStep::Objeto(0x20, 0x729000, 0xb6b7c0, 0x264e, SubsystemCtorArg::Estatisticas), // 0xf Match_Stats
    LoadStep::Objeto(0x30, 0x444520, 0xad9dac, 0x2663, SubsystemCtorArg::Caminho),   // 0x10 Club_Records
    LoadStep::Objeto(0x48, 0x7bfd50, 0xb74350, 0x2678, SubsystemCtorArg::Caminho),   // 0x11 PlayerStats
    LoadStep::Objeto(0x10, 0x442ab0, 0xad9da8, 0x268d, SubsystemCtorArg::Caminho),   // 0x12 Club_History
    LoadStep::Objeto(6, 0x5f97c0, 0xb5e134, 0x26a2, SubsystemCtorArg::Sozinho),      // 0x13 HostCountry
    LoadStep::Objeto(0x2a, 0x5554a0, 0xae28f0, 0x26b7, SubsystemCtorArg::Caminho),   // 0x14 Discipline
    LoadStep::Objeto(0xe0, 0x413180, 0xad9c60, 0x26cc, SubsystemCtorArg::Caminho),   // 0x15 AWARD_MANAGER
    LoadStep::Objeto(0xc, 0x5a4fc0, 0xae2a3c, 0x26e0, SubsystemCtorArg::Caminho),    // 0x16 Fine
    LoadStep::Objeto(0x10, 0x418440, 0xad9c6c, 0x26f4, SubsystemCtorArg::Caminho),   // 0x17 AWOL
    LoadStep::Objeto(8, 0x5697e0, 0xae2a30, 0x2708, SubsystemCtorArg::Caminho),      // 0x18 DISPUTE
    LoadStep::Objeto(0x1070, 0x794ad0, 0xb7433c, 0x271b, SubsystemCtorArg::Caminho), // 0x19 Notes
    LoadStep::Objeto(0x2c, 0x62a680, 0xb63d28, 0x272f, SubsystemCtorArg::Caminho),   // 0x1a Injury
    LoadStep::Objeto(0x3a, 0x76a4e0, 0xb74318, 0x2744, SubsystemCtorArg::Caminho),   // 0x1b NationalTeams
    LoadStep::Objeto(0x6a, 0x90d140, 0xde2f70, 0x2759, SubsystemCtorArg::Sozinho),   // 0x1c VIRTUAL_STAFF
    LoadStep::Objeto(0x19, 0x85b670, 0xdd7ef4, 0x276e, SubsystemCtorArg::Sozinho),   // 0x1d Stadium
    LoadStep::Objeto(0x1184, 0x5a9cf0, 0xae2a58, 0x2783, SubsystemCtorArg::Calendario), // 0x1e FIXTURE_MANAGER
    LoadStep::Rotina(0x669180, 0x2797),                                 // 0x1f competições (key_nation_create_all_comps)
    LoadStep::Rotina(0x8386f0, 0x27ab),                                 // 0x20 build_comp_object_table
    LoadStep::Objeto(2, 0x859aa0, 0xdd7ef0, 0x27c0, SubsystemCtorArg::Caminho),      // 0x21 SQUAD_MANAGER
    LoadStep::Objeto(0xab, 0x7e5fc0, 0xb74434, 0x27d5, SubsystemCtorArg::Caminho),   // 0x22 (depois 0x7e61c0)
    LoadStep::Objeto(4, 0x59a370, 0xae2a38, 0x27ed, SubsystemCtorArg::Caminho),      // 0x23 FinanceTable
    LoadStep::Objeto(0x3c, 0x4c7040, 0xae1050, 0x2802, SubsystemCtorArg::Caminho),   // 0x24 CONTRACT_MANAGER
    LoadStep::Objeto(0x94a, 0x8bba30, 0xde1f54, 0x2817, SubsystemCtorArg::Caminho),  // 0x25 TRANSFER_MANAGER
    LoadStep::Objeto(0x60, 0x7a5380, 0xb7434c, 0x282c, SubsystemCtorArg::Caminho),   // 0x26 PLAYER_REGEN
    LoadStep::Objeto(0x1c, 0x68dbf0, 0xb67a44, 0x2840, SubsystemCtorArg::Caminho),   // 0x27 MANAGER_MANAGER
    LoadStep::Objeto(0x18, 0x72b1d0, 0xb6b7c8, 0x2855, SubsystemCtorArg::Caminho),   // 0x28 Media
    LoadStep::Objeto(0x40, 0x5af630, 0xae2a60, 0x286a, SubsystemCtorArg::Caminho),   // 0x29 FOG_OF_WAR (confere o de Media)
    LoadStep::Objeto(0x10, 0x79b580, 0xb74340, 0x287f, SubsystemCtorArg::Caminho),   // 0x2a OfficialsManager
    LoadStep::Objeto(8, 0x84b940, 0xdd7ee4, 0x2894, SubsystemCtorArg::Caminho),      // 0x2b SimulatedStats
    LoadStep::Objeto(0xc, 0x9226c0, 0xde2f74, 0x28a9, SubsystemCtorArg::Sozinho),    // 0x2c Weather
    LoadStep::Objeto(0x688f, 0x8b0ec0, 0xde1f44, 0x28be, SubsystemCtorArg::Caminho), // 0x2d TRAINING_MANAGER
    LoadStep::Objeto(0xa, 0x7da190, 0xb74430, 0x28d9, SubsystemCtorArg::Idioma),     // 0x2d de novo: PunditList
];

/// A busca do CD da carga ("<raiz>cm0102.exe" em cada unidade de CD; ver `construct_match_stats`).
/// `exigir` = a do começo (com os patches sem CD do cm0102-1080 lidos do exe: 0x82aac4 e 0x82ab07):
/// devolve só quando acha (sem achar, a caixa "Insert CD" e de novo, para sempre). Sem `exigir` = a do fim
/// (0x82e31d, sem patch nos dois exes): numa unidade de CD sem o exe, a caixa "CD Not Required" e para.
unsafe fn check_game_cd(exigir: bool) {
    let units_digit: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(*(0x9670f8 as *const usize));
    let tipo: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(*(0x967100 as *const usize));
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let procurar: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x4c0aa0usize);
    let fechar_busca: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x4c0c10usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    // o cm0102-1080 vem sem a exigência do CD só na busca do começo: o jne do tipo (0x82aac4, 75 55) virou
    // nop e o test do achado (0x82ab07, 85 c0) virou xor (31 c0)
    let qualquer_unidade = exigir && *(0x82aac4 as *const u8) == 0x90;
    let sempre_achou = exigir && *(0x82ab07 as *const u8) != 0x85;
    loop {
        // ponytail: o buffer nasce zerado; a original só leria lixo com mais de 200 bytes de unidades
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
                // ponytail: a original anda com um i8 (passos de 4): só passaria de 127 com mais de 32 unidades
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

/// { 0x82a9c0 } (cdecl, Setup.cpp, ~14,6 KB, SEH): `load_game_data(caminho)`: carrega o save inteiro.
/// Devolve 1 se carregou.
/// - Disco (0x7a1820(0) < 25 liga [0xdd7134]) e memória ([0x9cd1fc] e 0x7a1710() < 50 liga [0xdd7138]);
///   a pasta temporária (0x90bde0 "CM3_TEMP"); o CD (`check_game_cd`); `caminho` nulo dá a caixa "Error"
///   0x233b. O banco: 0x528080 se [0xae23a8] ≠ 0 e 0x524d10(caminho, 1) (a resposta não é olhada).
/// - "Loading game data" e a barra 0x802f80 (passo de 0x2d); limpa os `*.tmp` (0x90cd50), o espaço
///   deles (0x933860, em MB, 0 vira 25) contra o disco, desempacota (0x933320; falha: "Disk Space
///   Error" 0x2382).
/// - `general.dat` (0x9326e0 lendo, 0x15, tipo 4; sem ele: "Load Game Error" 0x2397): a versão
///   ([0xdd7110]); o resumo (|v| ≥ 0x3e023; 0xf70 bytes até 0x3e026 com o tempo de jogo zerado, 0xf74
///   depois, com [0xdd7140] = o tempo de jogo dele); [0xdd7144] = time(); o id único, o tamanho do banco,
///   as datas e horas (a do jogo num local), as ligas, a nação padrão ([0xae2d08] = nação id, ou 0 se
///   < 0), os dados de execução das pessoas ([0xae2d0c], 0x4f × [0xae2d10]; os saves antigos convertidos
///   por `convert_staff_state`; |v| < 0x3e018 apaga o bit 7 de +0xb das [0xae2364] primeiras),
///   dos clubes ([0xae2d14], 6 × [0xae2d18] = [0xae235c]; |v| < 0x3e022: −1, 0, 0) e a reputação
///   ([0xae2d1c], 4 × [0xae235c]; |v| < 0x3e022 não lê e chama 0x53db40(1)). Leitura que falha: "Load
///   Game Error" com a linha e o texto do campo, item fechado; sem memória: "Error" e o jogo fecha.
/// - Os passos 4..=0x2d (`LOAD_STEPS`, disco e memória antes de cada um): no 5, a data do jogo
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
unsafe extern "C" fn load_game_data(caminho: *const u8) -> u32 {
    let disco: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7a1820usize);
    let memoria: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x7a1710usize);
    let pasta: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(0x90bde0usize);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let liberar_tabelas: unsafe extern "C" fn() = core::mem::transmute(0x528080usize);
    let carregar_banco: unsafe extern "C" fn(*const u8, u32) -> u32 = core::mem::transmute(0x524d10usize);
    let traduzir: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let barra: unsafe extern "thiscall" fn(usize, *const u8, u32, u32, i32) = core::mem::transmute(0x802f80usize);
    let limpar_temporarios: unsafe extern "C" fn(usize) = core::mem::transmute(0x90cd50usize);
    let tamanho: unsafe extern "C" fn(*const u8, usize) -> i32 = core::mem::transmute(0x933860usize);
    let desempacotar: unsafe extern "C" fn(*const u8, *const u8, usize) -> i32 = core::mem::transmute(0x933320usize);
    let abrir_item: unsafe extern "C" fn(*const u8, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> i32 = core::mem::transmute(0x932c30usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    let relogio: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(0x946ebbusize);
    let reservar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let reputacao_inicial: unsafe extern "C" fn(u32) = core::mem::transmute(0x53db40usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let regras_fifa: unsafe extern "C" fn(*const u8) = core::mem::transmute(0x839760usize);
    let ligar_regras: unsafe extern "C" fn(u32) = core::mem::transmute(0x58a890usize);
    let existe: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x90c120usize);
    crate::comparacao::ramo("save carregar jogo");
    let u = |a: usize| *(a as *const u32);
    let sem_memoria = |minimo: i32| u(0x9cd1fc) != 0 && memoria() < minimo;
    // as duas datas locais da original (a 2ª não é usada); o destrutor delas é vazio
    let (mut data_do_jogo, mut outra_data) = ([0u8; 8], [0u8; 8]);
    data_padrao(data_do_jogo.as_mut_ptr());
    data_padrao(outra_data.as_mut_ptr());
    if disco(0) < 0x19 {
        *(0xdd7134 as *mut u32) = 1;
        return 0;
    }
    if sem_memoria(0x32) {
        *(0xdd7138 as *mut u32) = 1;
        return 0;
    }
    let mut temporaria = [0u8; 0x104];
    copy_text(temporaria.as_mut_ptr(), pasta(0x987da0, 0xad9c64));
    check_game_cd(true);
    if caminho.is_null() {
        setup_warning(0x233b);
        return 0;
    }
    if u(0xae23a8) != 0 {
        liberar_tabelas();
    }
    carregar_banco(caminho, 1);
    // disco e memória entre os passos: liga a marca e devolve 0
    let falta_espaco = |minimo_disco: i32| {
        if disco(0) < minimo_disco {
            *(0xdd7134 as *mut u32) = 1;
            return true;
        }
        if sem_memoria(0x19) {
            *(0xdd7138 as *mut u32) = 1;
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
        save_error_box(0x2382, 0xa81758, 0x9c7774, 0x9c7680);
        return 0;
    }
    if falta_espaco(0x19) {
        return 0;
    }
    barra(0xb74450, texto.as_ptr(), 3, 0x2d, -1);
    let mut z = abrir_item(caminho, 0x9c7b84, 0, 0x15, 4);
    if z == 0 {
        save_error_box(0x2397, 0xa81720, 0x9af3e0, 0x9af334);
        return 0;
    }
    let mut versao = 0i32;
    if ler(&mut versao as *mut i32 as usize, 4, 1, z) != 1 {
        return load_read_error(0x239e, 0xa816e0, &mut z);
    }
    *(0xdd7110 as *mut i32) = versao;
    let v = versao.wrapping_abs();
    let mut resumo = [0u8; 0xf74];
    if v >= 0x3e023 {
        if v < 0x3e027 {
            if ler(resumo.as_mut_ptr() as usize, 0xf70, 1, z) != 1 {
                return load_read_error(0x23b2, 0xa816a4, &mut z);
            }
            resumo[0xf70..].fill(0);
        } else {
            if ler(resumo.as_mut_ptr() as usize, 0xf74, 1, z) != 1 {
                return load_read_error(0x23bf, 0xa816a4, &mut z);
            }
            *(0xdd7140 as *mut u32) = core::ptr::read_unaligned(resumo.as_ptr().add(0xf6c) as *const u32);
        }
    }
    let mut agora = 0i32;
    *(0xdd7144 as *mut i32) = relogio(&mut agora);
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
            return load_read_error(linha, msg, &mut z);
        }
    }
    *(0xae2d08 as *mut usize) = if nacao > -1 { (u(0xae23a8) as usize).wrapping_add((nacao as usize).wrapping_mul(0x122)) } else { 0 };
    if ler(0xae2d10, 4, 1, z) != 1 {
        return load_read_error(0x2420, 0xa81450, &mut z);
    }
    let pessoas = reservar(u(0xae2d10).wrapping_mul(0x4f));
    *(0xae2d0c as *mut usize) = pessoas;
    if pessoas == 0 {
        load_fatal_error(0x2427);
    }
    if v < 0x3e045 {
        // os saves antigos: registros de 0x26 (|v| < 0x3e042) ou 0x27 bytes, convertidos
        let (tam, linha_mem, linha_ler) = if v < 0x3e042 { (0x26u32, 0x242f, 0x2434) } else { (0x27, 0x24bd, 0x24c2) };
        let antigos = reservar(u(0xae2d10).wrapping_mul(tam));
        if antigos == 0 {
            load_fatal_error(linha_mem);
        }
        if ler(antigos, tam, u(0xae2d10), z) != u(0xae2d10) as i32 {
            return load_read_error(linha_ler, 0xa81418, &mut z);
        }
        convert_staff_state(antigos, tam as usize, u(0xae2d10) as i32);
        soltar(antigos);
    } else if ler(pessoas, 0x4f, u(0xae2d10), z) != u(0xae2d10) as i32 {
        return load_read_error(0x24f0, 0xa81418, &mut z);
    }
    if v < 0x3e018 {
        for i in 0..u(0xae2364) as i32 {
            *((u(0xae2d0c) as usize + 0x4f * i as usize + 0xb) as *mut u8) &= 0x7f;
        }
    }
    *(0xae2d18 as *mut u32) = u(0xae235c);
    let clubes = reservar(u(0xae235c).wrapping_mul(6));
    *(0xae2d14 as *mut usize) = clubes;
    if clubes == 0 {
        load_fatal_error(0x2503);
    }
    if v < 0x3e022 {
        for i in 0..u(0xae2d18) as i32 {
            let c = u(0xae2d14) as usize + 6 * i as usize;
            core::ptr::write_unaligned(c as *mut u32, 0xffffffff);
            *((u(0xae2d14) as usize + 6 * i as usize + 4) as *mut u8) = 0;
            *((u(0xae2d14) as usize + 6 * i as usize + 5) as *mut u8) = 0;
        }
        let r = reservar(u(0xae235c).wrapping_mul(4));
        *(0xae2d1c as *mut usize) = r;
        if r == 0 {
            load_fatal_error(0x2516);
        }
        reputacao_inicial(1);
    } else {
        if ler(clubes, 6, u(0xae2d18), z) != u(0xae2d18) as i32 {
            return load_read_error(0x251f, 0xa813e0, &mut z);
        }
        let r = reservar(u(0xae235c).wrapping_mul(4));
        *(0xae2d1c as *mut usize) = r;
        if r == 0 {
            load_fatal_error(0x2528);
        }
        if ler(r, 4, u(0xae235c), z) != u(0xae235c) as i32 {
            setup_warning(0x252c);
            fechar_item(&mut z);
            return 0;
        }
    }
    fechar_item(&mut z);
    for (i, step_towards) in LOAD_STEPS.iter().enumerate() {
        let k = (i as u32 + 4).min(0x2d);
        if k == 9 {
            undo_holiday_managers();
        }
        if i == LOAD_STEPS.len() - 1 && v < 0x3e032 {
            let acertar: unsafe extern "C" fn(usize) = core::mem::transmute(0x538bc0usize);
            for p in 0..u(0xae2364) as i32 {
                acertar(u(0xae23bc) as usize + 0x6e * p as usize);
            }
        }
        if falta_espaco(0x19) {
            return 0;
        }
        barra(0xb74450, texto.as_ptr(), k, 0x2d, -1);
        match *step_towards {
            LoadStep::Rotina(f, linha) => {
                let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(f);
                if f() == 0 && linha != 0 {
                    setup_warning(linha);
                    return 0;
                }
                if k == 5 {
                    let mut tmp = [0u8; 8];
                    copia_data(0xae2c90, tmp.as_mut_ptr(), data_do_jogo.as_ptr() as usize);
                }
            }
            LoadStep::Objeto(tam, construtor, global, linha, arg) => {
                let p = novo(tam);
                let o = if p == 0 {
                    0
                } else {
                    match arg {
                        SubsystemCtorArg::Caminho => {
                            let f: unsafe extern "thiscall" fn(usize, *const u8) -> usize = core::mem::transmute(construtor);
                            f(p, caminho)
                        }
                        SubsystemCtorArg::Sozinho => {
                            let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(construtor);
                            f(p)
                        }
                        SubsystemCtorArg::Estatisticas => {
                            let f: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(construtor);
                            f(p, caminho, 0x2710)
                        }
                        SubsystemCtorArg::Calendario => {
                            // a original empurra um dword com o ano nos 16 bits baixos e sobra de pilha nos
                            // altos; o construtor só usa a palavra (0x54cfe0 faz & 0xffff)
                            let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(construtor);
                            f(p, *(0xae2c92 as *const u16) as u32, 0, 1)
                        }
                        SubsystemCtorArg::Idioma => {
                            // a original empurra edx com o byte em dl (o resto é sobra); o construtor só lê o byte
                            let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(construtor);
                            f(p, *(0x9b5984 as *const u8) as u32)
                        }
                    }
                };
                *(global as *mut usize) = o;
                // o FOG_OF_WAR confere o global de Media (como na gravação)
                let conferido = if k == 0x29 { u(0xb6b7c8) as usize } else { o };
                if conferido == 0 {
                    setup_warning(linha);
                    return 0;
                }
                if k == 0x22 {
                    let iniciar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7e61c0usize);
                    iniciar(o);
                }
            }
        }
    }
    if v >= 0x3e039 {
        regras_fifa(caminho);
    }
    *(0xdd711c as *mut u32) = 1;
    *(0xdd7130 as *mut u32) = 1;
    *(0xa801bc as *mut u32) = 0;
    let mut tmp = [0u8; 8];
    copia_data(0xdd70a0, tmp.as_mut_ptr(), 0xae2c90);
    if *(0x9b5980 as *const u8) != 0 {
        // a data de mudança das regras [0xae2330] (palavra baixa: dia; alta: ano) contra a do jogo
        // [0xae2c90]: ano sem sinal, dia com sinal
        let em_vigor = *(0xae233c as *const u8);
        let (dia_jogo, ano_jogo) = (*(0xae2c90 as *const i16), *(0xae2c92 as *const u16));
        let (dia, ano) = (*(0xae2330 as *const i16), *(0xae2332 as *const u16));
        let mut desligar = false;
        if !(em_vigor == 0 && *(0xa83608 as *const u8) == 0xa) {
            desligar = ano > ano_jogo || (ano == ano_jogo && dia > dia_jogo);
        }
        if desligar {
            *(0xae233c as *mut u8) = 0;
            *(0xa83608 as *mut u8) = 0xa;
        } else if !(em_vigor != 0 && *(0xa83608 as *const u8) == 5) && (ano < ano_jogo || (ano == ano_jogo && dia < dia_jogo)) {
            *(0xae233c as *mut u8) = 1;
            *(0xa83608 as *mut u8) = 5;
            ligar_regras(1);
        }
    }
    if existe(0xa80340) == 0 {
        check_game_cd(false);
    }
    1
}

/// Antes do passo 9 da carga: dos 16 últimos da tabela de pessoas ([0xae23bc], 0x6e bytes, do último
/// para trás), os com +0x5f = 5 deixam de ser técnico humano: +0x5f = 0, News 0x787630, HUMAN_MANAGER
/// 0x5fd2f0/0x5fd590/0x5fe140 (0, pessoa), CONTRACT_MANAGER 0x4ccbf0/0x4d0b50/0x4d0bd0,
/// NationalTeams 0x775ba0, Staff_Records 0x86fb80/0x86fc70 (cada um com o global como está), os três
/// nomes vazios, +0x10 = data(1, 0, 1966, −1) (0x54a130), +0x18 = 1966, +0x24 = +0x39 = 0 e 0x7ff990
/// (gerente de telas, pessoa, −1).
unsafe fn undo_holiday_managers() {
    let u = |a: usize| *(a as *const usize);
    let um: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x787630usize);
    for i in 0..0x10usize {
        let s = u(0xae23bc).wrapping_add(0x6e * ((*(0xae2364 as *const i32)).wrapping_sub(i as i32).wrapping_sub(1) as usize));
        if *((s + 0x5f) as *const u8) != 5 {
            continue;
        }
        *((s + 0x5f) as *mut u8) = 0;
        um(u(0xb7431c), s);
        for f in [0x5fd2f0usize, 0x5fd590, 0x5fe140] {
            let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(f);
            f(u(0xb63c98), 0, s);
        }
        for (g, f) in [(0xae1050usize, 0x4ccbf0usize), (0xae1050, 0x4d0b50), (0xae1050, 0x4d0bd0), (0xb74318, 0x775ba0), (0xdd7f00, 0x86fb80), (0xdd7f00, 0x86fc70)] {
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
            f(u(g), s);
        }
        *((s + 0x5f) as *mut u8) = 0;
        *(u(s + 4) as *mut u8) = 0;
        *(u(s + 8) as *mut u8) = 0;
        *(u(s + 0xc) as *mut u8) = 0;
        let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(0x54a130usize);
        acertar_data(s + 0x10, 1, 0, 0x7ae, -1);
        *((s + 0x18) as *mut u16) = 0x7ae;
        core::ptr::write_unaligned((s + 0x24) as *mut u32, 0);
        core::ptr::write_unaligned((s + 0x39) as *mut u32, 0);
        let tela: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x7ff990usize);
        tela(0xb74450, s, -1);
    }
}

// ponytail: carrega o save inteiro (apaga e recria os subsistemas, desempacota a pasta temporária);
// não roda duas vezes. Só a original; a cópia leva o ramo e é provada pela carga da prova do save.
unsafe extern "C" fn comparar_load_game_data(caminho: *const u8) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8) -> u32>(%%I0%%)(caminho)
}
