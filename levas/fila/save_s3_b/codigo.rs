// ---------- %%LEVA%%a leva: o save: gravar o jogo inteiro (save_game_data) (Setup.cpp) ----------

/// A caixa de erro da gravação: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de Setup.cpp,
/// `linha`, `texto`) em 0xae24d0; o título `titulo` traduzido (0x66f4e0) em 0xae28bc, o corpo
/// `corpo` (com o texto de 0xae24d0) traduzido em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está, ao contrário do `setup_warning`).
unsafe fn save_error_box(linha: u32, texto: usize, titulo: usize, corpo: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
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
enum SaveStep {
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

const SAVE_STEPS: [SaveStep; 0x23] = [
    SaveStep::Caminho(0xb63c98, 0x5ff9e0), // 2 HUMAN_MANAGER
    SaveStep::Caminho(0xb74448, 0x7f8020), // 3 SCOUT_MANAGER
    SaveStep::Caminho(0xb7431c, 0x787c30), // 4 News
    SaveStep::Virtual(0xae2c74),           // 5 Friendly
    SaveStep::Sozinho(0xde1f58, 0x905510), // 6 UefaSeeding
    SaveStep::Sozinho(0xae1000, 0x4c6b70), // 7 ConmebolSeeding
    SaveStep::Caminho(0xdd7f00, 0x86ffd0), // 8 Staff_Records
    SaveStep::Caminho(0xb6b7c0, 0x72adf0), // 9 Match_Stats
    SaveStep::Caminho(0xb74350, 0x7c55a0), // 0xa PlayerStats
    SaveStep::Caminho(0xad9da8, 0x443f50), // 0xb Club_History
    SaveStep::Caminho(0xad9dac, 0x451ff0), // 0xc Club_Records
    SaveStep::Caminho(0xde1d08, 0x8995d0), // 0xd Tactics
    SaveStep::Sozinho(0xb5e134, 0x5fb320), // 0xe HostCountry
    SaveStep::Caminho(0xae28f0, 0x555dd0), // 0xf Discipline
    SaveStep::Caminho(0xad9c60, 0x4141a0), // 0x10 AWARD_MANAGER
    SaveStep::Caminho(0xae2a3c, 0x5a53d0), // 0x11 Fine
    SaveStep::Caminho(0xad9c6c, 0x418980), // 0x12 AWOL
    SaveStep::Caminho(0xae2a30, 0x569ae0), // 0x13 DISPUTE
    SaveStep::Caminho(0xb7433c, 0x794f00), // 0x14 Notes
    SaveStep::Caminho(0xb63d28, 0x62b2d0), // 0x15 Injury
    SaveStep::Caminho(0xb74318, 0x776a20), // 0x16 NationalTeams
    SaveStep::Sozinho(0xdd7ef4, 0x85bea0), // 0x17 Stadium
    SaveStep::Um(0xae2a58, 0x5ac060),      // 0x18 FIXTURE_MANAGER
    SaveStep::Competicoes,                 // 0x19 key_nation_save_all_comps
    SaveStep::Caminho(0xae1050, 0x4dc3a0), // 0x1a CONTRACT_MANAGER
    SaveStep::Caminho(0xae2a38, 0x59b3b0), // 0x1b FinanceTable
    SaveStep::Caminho(0xde1f54, 0x8bd4b0), // 0x1c TRANSFER_MANAGER
    SaveStep::Caminho(0xb7434c, 0x7ad7c0), // 0x1d PLAYER_REGEN
    SaveStep::Caminho(0xb6b7c8, 0x72b5f0), // 0x1e Media
    SaveStep::Outro(0xb6b7c8, 0xae2a60, 0x5afad0), // 0x1f FOG_OF_WAR
    SaveStep::Caminho(0xb67a44, 0x68e920), // 0x20 MANAGER_MANAGER
    SaveStep::Caminho(0xb74340, 0x79bb80), // 0x21 OfficialsManager
    SaveStep::Caminho(0xdd7ee4, 0x84c680), // 0x22 SimulatedStats
    SaveStep::Caminho(0xde1f44, 0x8b1450), // 0x23 TRAINING_MANAGER
    SaveStep::Caminho(0xdd7ef0, 0x859cc0), // 0x24 SQUAD_MANAGER
];

/// { 0x82e430 } (cdecl, Setup.cpp, ~6,8 KB, SEH): `save_game_data(caminho, a4)`: grava o jogo
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
/// - Os passos 2..=0x24 (`SAVE_STEPS`): disco e memória (antes do 0x11, sem apagar e com
///   memória < 25), a barra e o `save` do subsistema; 0x25: empacota os `*.tmp` (0x933610).
/// - Hall da fama (0x5f00c0, fora do save), regras FIFA (0x8395b0); em rede, troca o save antigo
///   pelo novo (0x945ba4); [0xdd7130] = 1, [0xa801bc] = 0 e a data do último save (0xdd70a0).
/// A data local da original ([esp+0x1758], 0x54a110, destrutor vazio 0x5a53c0 no SEH) não é usada:
/// fica de fora. O laço da original ([esp+0x11c] de 1 a 2) roda o corpo uma vez, com o total 0x25.
/// O resumo é um buffer zerado aqui; na original, o que o 0x591f80 não escreve (o clube dos 16
/// técnicos não humanos, o resto dos textos) é sobra da pilha dela, gravada no `general.dat`.
unsafe extern "C" fn save_game_data(caminho: *const u8, a4: u32) -> u32 {
    let disco: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7a1820usize);
    let memoria: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x7a1710usize);
    let apagar: unsafe extern "C" fn(*const u8) -> i32 = core::mem::transmute(0x94643ausize);
    let pasta: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(0x90bde0usize);
    let existe: unsafe extern "C" fn(*const u8) -> i32 = core::mem::transmute(0x90c120usize);
    let separar: unsafe extern "C" fn(*const u8, *mut u8, *mut u8, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let juntar: unsafe extern "C" fn(*mut u8, *const u8, *const u8, usize, *const u8) = core::mem::transmute(0x945b13usize);
    let renomear: unsafe extern "C" fn(*const u8, *const u8) -> i32 = core::mem::transmute(0x945ba4usize);
    let gravar_banco: unsafe extern "C" fn(*const u8, u32, u32) -> u32 = core::mem::transmute(0x524d30usize);
    let traduzir: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let barra: unsafe extern "thiscall" fn(usize, *const u8, u32, u32, i32) = core::mem::transmute(0x802f80usize);
    let abrir_item: unsafe extern "C" fn(*const u8, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    let montar_resumo: unsafe extern "C" fn(*mut u8) = core::mem::transmute(0x591f80usize);
    let empacotar: unsafe extern "C" fn(*const u8, *const u8, usize, u32, u32) -> i32 = core::mem::transmute(0x933610usize);
    let competicoes: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x6692f0usize);
    let hall_da_fama: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5f00c0usize);
    let regras_fifa: unsafe extern "C" fn(*const u8) = core::mem::transmute(0x8395b0usize);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    crate::comparison::branch("save gravar jogo");
    let opcoes = || *((*(0xae3138 as *const usize) + 4) as *const u32);
    let tipo = || if opcoes() & 0x8000 != 0 { 3u32 } else { 4 };
    let sem_memoria = || *(0x9cd1fc as *const u32) != 0 && memoria() < 1;
    *(0xdd7110 as *mut u32) = 0xfffc1fb9;
    if opcoes() & 0x20000000 != 0 {
        let falta = if disco(0) < 0x19 {
            *(0xdd7134 as *mut u32) = 1;
            true
        } else if sem_memoria() {
            *(0xdd7138 as *mut u32) = 1;
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
    copy_text(temporaria.as_mut_ptr(), pasta(0x987da0, 0xad9c64));
    if caminho.is_null() {
        setup_warning(0x2958);
        return 0;
    }
    let mut local = [0u8; 0x104];
    if opcoes() & 0x20000000 != 0 {
        let (mut unidade, mut dir, mut nome, mut ext) = ([0u8; 0x104], [0u8; 0x104], [0u8; 0x100], [0u8; 0x100]);
        separar(caminho, unidade.as_mut_ptr(), dir.as_mut_ptr(), nome.as_mut_ptr(), ext.as_mut_ptr());
        juntar(local.as_mut_ptr(), unidade.as_ptr(), dir.as_ptr(), 0xa81cf4, ext.as_ptr());
    } else {
        copy_text(local.as_mut_ptr(), caminho);
    }
    let l = local.as_ptr();
    if existe(l) != 0 && apagar(l) == -1 {
        return 0;
    }
    if opcoes() & 0x40000000 != 0 {
        if disco(0) < 0x19 {
            *(0xdd7134 as *mut u32) = 1;
            apagar(caminho);
            return 0;
        }
        if sem_memoria() {
            *(0xdd7138 as *mut u32) = 1;
            apagar(caminho);
            return 0;
        }
    }
    gravar_banco(l, a4, tipo());
    // disco e memória entre os passos: liga a marca, apaga o caminho de trabalho e devolve 0
    let falta_espaco = || {
        if disco(0) < 0x19 {
            *(0xdd7134 as *mut u32) = 1;
            apagar(l);
            return true;
        }
        if sem_memoria() {
            *(0xdd7138 as *mut u32) = 1;
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
        save_error_box(0x29a3, 0xa81c88, 0xa81c70, 0xa81b5c);
        return 0;
    }
    // general.dat: (endereço, tamanho, quantidade, linha e texto do erro); o resumo e a nação
    // padrão saem de buffers locais; os três últimos gravam vetores e conferem a quantidade
    let mut resumo = [0u8; 0xf74];
    let mut campo = |buf: usize, tam: u32, n: u32, linha: u32, msg: usize| -> bool {
        if gravar(buf, tam, n, z) == n {
            return true;
        }
        save_error_box(linha, msg, 0x9c7774, 0x9c7680);
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
    let padrao = u(0xae2d08);
    let nacao_padrao: u32 = if padrao != 0 { u(padrao as usize) } else { 0xffffffff };
    if !campo(&nacao_padrao as *const u32 as usize, 4, 1, 0x2a06, 0xa818c0)
        || !campo(0xae2d10, 4, 1, 0x2a0e, 0xa81880)
        || !campo(u(0xae2d0c) as usize, 0x4f, u(0xae2d10), 0x2a14, 0xa81848)
        || !campo(u(0xae2d14) as usize, 6, u(0xae2d18), 0x2a1c, 0xa81814)
        || !campo(u(0xae2d1c) as usize, 4, u(0xae235c), 0x2a24, 0xa817dc)
    {
        return 0;
    }
    fechar_item(&mut z);
    for (i, step_towards) in SAVE_STEPS.iter().enumerate() {
        let k = i as u32 + 2;
        // antes do passo 0x11 (depois do AWARD_MANAGER) a original não apaga o caminho e pede
        // memória >= 25 (e não >= 1)
        if k == 0x11 {
            if disco(0) < 0x19 {
                *(0xdd7134 as *mut u32) = 1;
                return 0;
            }
            if *(0x9cd1fc as *const u32) != 0 && memoria() < 0x19 {
                *(0xdd7138 as *mut u32) = 1;
                return 0;
            }
        } else if falta_espaco() {
            return 0;
        }
        barra(0xb74450, texto.as_ptr(), k, 0x25, -1);
        match *step_towards {
            SaveStep::Caminho(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(f);
                    f(u(g) as usize, l);
                }
            }
            SaveStep::Sozinho(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
                    f(u(g) as usize);
                }
            }
            SaveStep::Um(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
                    f(u(g) as usize, 1);
                }
            }
            SaveStep::Virtual(g) => {
                let o = u(g) as usize;
                if o != 0 {
                    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(*((*(o as *const usize) + 0x1c) as *const usize));
                    f(o, 1);
                }
            }
            SaveStep::Outro(g, this, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(f);
                    f(u(this) as usize, l);
                }
            }
            SaveStep::Competicoes => {
                if competicoes() == 0 {
                    setup_warning(0x2baa);
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
        save_error_box(linha, 0xa817a4, 0x9c7774, 0x9c7680);
        return 0;
    }
    hall_da_fama(u(0xb5ded8) as usize);
    regras_fifa(l);
    if opcoes() & 0x20000000 != 0 {
        if existe(caminho) != 0 && apagar(caminho) == -1 {
            save_error_box(0x2ca7, 0xa81cc0, 0x9af90c, 0x9af828);
        }
        renomear(l, caminho);
    }
    *(0xdd7130 as *mut u32) = 1;
    *(0xa801bc as *mut u32) = 0;
    let mut data = [0u8; 8];
    copia_data(0xdd70a0, data.as_mut_ptr(), 0xae2c90);
    1
}

// ponytail: grava o save inteiro (apaga o antigo, cria o contêiner, empacota a pasta temporária);
// não roda duas vezes. Só a original; a cópia leva o ramo e é provada pela prova do save.
unsafe extern "C" fn comparar_save_game_data(caminho: *const u8, a4: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8, u32) -> u32>(%%I0%%)(caminho, a4)
}
