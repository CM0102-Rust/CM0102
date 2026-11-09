use super::*;

/// Unrecoverable load error box (setup warning with the line), fatal cleanup and exit(-1).
///
/// [pt] A caixa "Error" da carga sem volta (memória para os dados de execução): `setup_warning(linha)`,
/// 0x5cc7d0, 0x5a8f60 e `exit(-1)` (0x945934).
pub(crate) unsafe fn load_fatal_error(row: u32) -> ! {
    setup_warning(row);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
    loop {}
}

/// Failed general.dat read: "Load Game Error" box with line and text, closes the item; returns 0.
///
/// [pt] A leitura do `general.dat` que falhou: a caixa "Load Game Error" com a linha e o texto (como a
/// `save_error_box`) e o item fechado (0x932b00). Devolve 0, a resposta da carga.
pub(crate) unsafe fn load_read_error(row: u32, text: usize, z: &mut usize) -> u32 {
    save_error_box(row, text, 0x9af3e0, 0x9af334);
    exe::zclose(z);
    0
}

/// Converts an old save's person runtime data (records of `tam` bytes) into the current 0x4f-byte record at [0xae2d0c],
/// in the original's order (re-reading the global for each field).
///
/// +0, +4, +8, +9, +0xa copied; +0xb: bit 7 of the old one and bits 0x7f of the destination (malloc leftover), then
/// bits 0xf, 0x40 and 0x30 of the old one; +0xc: bit 0 of the old one. `tam` 0x26 (version < 0x3e042): +0xd = 0 and bit
/// 1 of +0xc cleared; 0x27 (0x3e042..0x3e044): +0xd from the old one and bit 1 of +0xc from the old one. The 5 pairs
/// (+0x12 + 5j = 0, +0xe + 5j = −1) and the dwords +0x27/+0x2b/+0x2f/+0x33/+0x37 (+0x14j, j = 0, 1) = −1.
///
/// [pt] Os dados de execução das pessoas de um save antigo (registros de `tam` bytes) passados para o
/// registro atual de 0x4f bytes em [0xae2d0c], na ordem da original (relê o global a cada campo):
/// +0, +4, +8, +9, +0xa copiados; +0xb: o bit 7 do antigo e os bits 0x7f do destino (sobra do
/// malloc), depois os bits 0xf, 0x40 e 0x30 do antigo; +0xc: o bit 0 do antigo. `tam` 0x26 (versão <
/// 0x3e042): +0xd = 0 e o bit 1 de +0xc apagado; 0x27 (0x3e042..0x3e044): +0xd do antigo e o bit 1
/// de +0xc do antigo. Os 5 pares (+0x12 + 5j = 0, +0xe + 5j = −1) e os dwords +0x27/+0x2b/+0x2f/
/// +0x33/+0x37 (+0x14j, j = 0, 1) = −1.
pub(crate) unsafe fn convert_staff_state(font: usize, size: usize, n: i32) {
    let d = || *(G_STAFF_STATE_TABLE as *const usize);
    let mesclar = |base: u8, other: u8, m: u8| base ^ ((base ^ other) & m);
    for i in 0..n.max(0) as usize {
        let (o, f) = (i * 0x4f, i * size);
        *((d() + o) as *mut u32) = *((font + f) as *const u32);
        let x = d() + o + 0xb;
        *(x as *mut u8) = mesclar(read_u8(font + (f + 0xb)), *(x as *const u8), 0x7f);
        *((d() + o + 0xa) as *mut u8) = read_u8(font + (f + 0xa));
        *((d() + o + 4) as *mut u32) = core::ptr::read_unaligned((font + f + 4) as *const u32);
        *((d() + o + 9) as *mut u8) = read_u8(font + (f + 9));
        *((d() + o + 8) as *mut u8) = read_u8(font + (f + 8));
        for m in [0xfu8, 0x40, 0x30] {
            let x = d() + o + 0xb;
            *(x as *mut u8) = mesclar(*(x as *const u8), read_u8(font + (f + 0xb)), m);
        }
        let c = d() + o + 0xc;
        *(c as *mut u8) = mesclar(*(c as *const u8), read_u8(font + (f + 0xc)), 1);
        if size == 0x26 {
            *((d() + o + 0xd) as *mut u8) = 0;
            *((d() + o + 0xc) as *mut u8) &= 0xfd;
        } else {
            *((d() + o + 0xd) as *mut u8) = read_u8(font + (f + 0xd));
            let c = d() + o + 0xc;
            *(c as *mut u8) = mesclar(*(c as *const u8), read_u8(font + (f + 0xc)), 2);
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
pub(crate) enum SubsystemCtorArg {
    /// (this, caminho do save)
    Caminho,
    /// (this)
    Sozinho,
    /// (this, caminho, 0x2710): Match_Stats
    Estatisticas,
    /// (this, game year, 0, 1): FIXTURE_MANAGER.
    ///
    /// Without the save: the original routine pushes ebp, which is 0 there; the constructor reads the fixtures from the
    /// temporary folder.
    ///
    /// [pt] (this, ano do jogo, 0, 1): FIXTURE_MANAGER (sem o save: a original empurra ebp, que ali é 0;
    /// o construtor lê os jogos da pasta temporária)
    Calendario,
    /// (this, idioma dos nomes [0x9b5984] (byte)): PunditList
    Idioma,
}

/// Um passo da carga depois do `general.dat`: a barra (passo de 0x2d), disco e memória, e
/// `Rotina(f, linha)`: cdecl() que dá erro na `linha` se devolver 0 (linha 0: sem conferência), ou
/// `Objeto(tamanho do new, construtor, global, linha do erro se o global ficar 0, argumentos)`.
#[derive(Clone, Copy)]
pub(crate) enum LoadStep {
    Rotina(usize, u32),
    Objeto(u32, usize, usize, u32, SubsystemCtorArg),
}

/// Os passos 4..=0x2d da carga, na ordem da original, e o último (PunditList), que repete o 0x2d.
pub(crate) const LOAD_STEPS: [LoadStep; 0x2b] = [
    LoadStep::Rotina(0x539750, 0),                                      // 4
    LoadStep::Rotina(0x667150, 0x2555),                                 // 5 key nations
    LoadStep::Rotina(0x669140, 0x256c),                                 // 6
    LoadStep::Objeto(0x609, 0x893dc0, G_TACTICS, 0x2581, SubsystemCtorArg::Caminho), // 7 Tactics
    LoadStep::Objeto(0x10, 0x5fb430, G_HUMAN_MANAGER, 0x2596, SubsystemCtorArg::Caminho),  // 8 HUMAN_MANAGER
    LoadStep::Objeto(0xe8, 0x7f6760, G_SCOUT_MANAGER, 0x25d0, SubsystemCtorArg::Caminho),  // 9 SCOUT_MANAGER
    LoadStep::Objeto(0x14, 0x785b20, G_NEWS, 0x25e5, SubsystemCtorArg::Caminho),  // 0xa News
    LoadStep::Objeto(0x3507, 0x5c1f80, G_FRIENDLY, 0x25fa, SubsystemCtorArg::Sozinho), // 0xb Friendly
    LoadStep::Objeto(0x698, 0x903360, G_UEFA_SEEDING, 0x260f, SubsystemCtorArg::Sozinho),  // 0xc UefaSeeding
    LoadStep::Objeto(0x75, 0x4c5fe0, G_CONMEBOL_SEEDING, 0x2624, SubsystemCtorArg::Sozinho),   // 0xd ConmebolSeeding
    LoadStep::Objeto(0x18, 0x868390, G_STAFF_RECORDS, 0x2639, SubsystemCtorArg::Caminho),   // 0xe Staff_Records
    LoadStep::Objeto(0x20, 0x729000, G_MATCH_STATS, 0x264e, SubsystemCtorArg::Estatisticas), // 0xf Match_Stats
    LoadStep::Objeto(0x30, 0x444520, G_CLUB_RECORDS, 0x2663, SubsystemCtorArg::Caminho),   // 0x10 Club_Records
    LoadStep::Objeto(0x48, 0x7bfd50, G_PLAYER_STATS, 0x2678, SubsystemCtorArg::Caminho),   // 0x11 PlayerStats
    LoadStep::Objeto(0x10, 0x442ab0, G_CLUB_HISTORY, 0x268d, SubsystemCtorArg::Caminho),   // 0x12 Club_History
    LoadStep::Objeto(6, 0x5f97c0, G_HOST_COUNTRY, 0x26a2, SubsystemCtorArg::Sozinho),      // 0x13 HostCountry
    LoadStep::Objeto(0x2a, 0x5554a0, G_DISCIPLINE, 0x26b7, SubsystemCtorArg::Caminho),   // 0x14 Discipline
    LoadStep::Objeto(0xe0, 0x413180, G_AWARD_MANAGER, 0x26cc, SubsystemCtorArg::Caminho),   // 0x15 AWARD_MANAGER
    LoadStep::Objeto(0xc, 0x5a4fc0, G_FINE, 0x26e0, SubsystemCtorArg::Caminho),    // 0x16 Fine
    LoadStep::Objeto(0x10, 0x418440, G_AWOL, 0x26f4, SubsystemCtorArg::Caminho),   // 0x17 AWOL
    LoadStep::Objeto(8, 0x5697e0, G_DISPUTE, 0x2708, SubsystemCtorArg::Caminho),      // 0x18 DISPUTE
    LoadStep::Objeto(0x1070, 0x794ad0, G_NOTES, 0x271b, SubsystemCtorArg::Caminho), // 0x19 Notes
    LoadStep::Objeto(0x2c, 0x62a680, G_INJURY, 0x272f, SubsystemCtorArg::Caminho),   // 0x1a Injury
    LoadStep::Objeto(0x3a, 0x76a4e0, G_NATIONAL_TEAMS, 0x2744, SubsystemCtorArg::Caminho),   // 0x1b NationalTeams
    LoadStep::Objeto(0x6a, 0x90d140, G_VIRTUAL_STAFF, 0x2759, SubsystemCtorArg::Sozinho),   // 0x1c VIRTUAL_STAFF
    LoadStep::Objeto(0x19, 0x85b670, G_STADIUM, 0x276e, SubsystemCtorArg::Sozinho),   // 0x1d Stadium
    LoadStep::Objeto(0x1184, 0x5a9cf0, G_FIXTURE_MANAGER, 0x2783, SubsystemCtorArg::Calendario), // 0x1e FIXTURE_MANAGER
    LoadStep::Rotina(0x669180, 0x2797),                                 // 0x1f competitions (key_nation_create_all_comps)
    LoadStep::Rotina(0x8386f0, 0x27ab),                                 // 0x20 build_comp_object_table
    LoadStep::Objeto(2, 0x859aa0, G_SQUAD_MANAGER, 0x27c0, SubsystemCtorArg::Caminho),      // 0x21 SQUAD_MANAGER
    LoadStep::Objeto(0xab, 0x7e5fc0, G_RESERVE_COMP, 0x27d5, SubsystemCtorArg::Caminho),   // 0x22 (depois 0x7e61c0)
    LoadStep::Objeto(4, 0x59a370, G_FINANCE_TABLE, 0x27ed, SubsystemCtorArg::Caminho),      // 0x23 FinanceTable
    LoadStep::Objeto(0x3c, 0x4c7040, G_CONTRACT_MANAGER, 0x2802, SubsystemCtorArg::Caminho),   // 0x24 CONTRACT_MANAGER
    LoadStep::Objeto(0x94a, 0x8bba30, G_TRANSFER_MANAGER, 0x2817, SubsystemCtorArg::Caminho),  // 0x25 TRANSFER_MANAGER
    LoadStep::Objeto(0x60, 0x7a5380, G_PLAYER_REGEN, 0x282c, SubsystemCtorArg::Caminho),   // 0x26 PLAYER_REGEN
    LoadStep::Objeto(0x1c, 0x68dbf0, G_MANAGER_MANAGER, 0x2840, SubsystemCtorArg::Caminho),   // 0x27 MANAGER_MANAGER
    LoadStep::Objeto(0x18, 0x72b1d0, G_MEDIA, 0x2855, SubsystemCtorArg::Caminho),   // 0x28 Media
    LoadStep::Objeto(0x40, 0x5af630, G_FOG_OF_WAR, 0x286a, SubsystemCtorArg::Caminho),   // 0x29 FOG_OF_WAR (checks the Media one)
    LoadStep::Objeto(0x10, 0x79b580, G_OFFICIALS_MANAGER, 0x287f, SubsystemCtorArg::Caminho),   // 0x2a OfficialsManager
    LoadStep::Objeto(8, 0x84b940, G_SIMULATED_STATS, 0x2894, SubsystemCtorArg::Caminho),      // 0x2b SimulatedStats
    LoadStep::Objeto(0xc, 0x9226c0, G_WEATHER, 0x28a9, SubsystemCtorArg::Sozinho),    // 0x2c Weather
    LoadStep::Objeto(0x688f, 0x8b0ec0, G_TRAINING_MANAGER, 0x28be, SubsystemCtorArg::Caminho), // 0x2d TRAINING_MANAGER
    LoadStep::Objeto(0xa, 0x7da190, G_PUNDIT_LIST, 0x28d9, SubsystemCtorArg::Idioma),     // 0x2d again: PunditList
];

/// Looks for "<root>cm0102.exe" on each CD drive: with `exigir` (start of load, honouring no-CD patches in the exe)
/// loops on the "Insert CD" box until found; without it (end of load) shows "CD Not Required" on a CD drive lacking the
/// exe.
///
/// [pt] A busca do CD da carga ("<raiz>cm0102.exe" em cada unidade de CD; ver `construct_match_stats`).
/// `exigir` = a do começo (com os patches sem CD do cm0102-1080 lidos do exe: 0x82aac4 e 0x82ab07):
/// devolve só quando acha (sem achar, a caixa "Insert CD" e de novo, para sempre). Sem `exigir` = a do fim
/// (0x82e31d, sem patch nos dois exes): numa unidade de CD sem o exe, a caixa "CD Not Required" e para.
pub(crate) unsafe fn check_game_cd(exigir: bool) {
    let units_digit: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(*(0x9670f8 as *const usize));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(*(0x967100 as *const usize));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    // cm0102-1080 comes without the CD requirement only in the initial search: the type’s jne (0x82aac4, 75 55) became
    // nop and the found test (0x82ab07, 85 c0) became xor (31 c0)
    let any_unit = exigir && *(0x82aac4 as *const u8) == 0x90;
    let always_found = exigir && *(0x82ab07 as *const u8) != 0x85;
    loop {
        // simplification: the buffer starts zeroed; the original would only read garbage with more than 200 bytes of units
        let mut list = [0u8; 0x100];
        if units_digit(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let raiz = list.as_ptr() as usize + i;
                // the type is always requested (in cm0102-1080 only the jump after it became a nop)
                let t = kind(raiz);
                if any_unit || t == 5 {
                    let mut name = [0u8; 0x104];
                    let mut found = [0u8; 0x114];
                    let nm = name.as_mut_ptr() as usize;
                    format(nm, 0x987e00, raiz, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let ach = found.as_mut_ptr() as usize;
                    if exe::file_find_first(nm, ach, 0) == 0 || always_found {
                        if exigir {
                            return;
                        }
                        let (mut titulo, mut msg) = ([0u8; 0x100], [0u8; 0x100]);
                        text(titulo.as_mut_ptr() as usize, 0xa81384);
                        text(msg.as_mut_ptr() as usize, 0xa81354);
                        exe::gui_show_message_box(titulo.as_ptr() as usize, msg.as_ptr() as usize, 0);
                        return;
                    }
                    exe::file_find_close(ach);
                }
                // simplification: the original walks with an i8 (steps of 4): it would only pass 127 with more than 32 units
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        if !exigir {
            return;
        }
        let (mut titulo, mut msg) = ([0u8; 0x100], [0u8; 0x100]);
        text(titulo.as_mut_ptr() as usize, 0x987df4);
        text(msg.as_mut_ptr() as usize, 0x987dc4);
        exe::gui_show_message_box(titulo.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }
}

/// `load_game_data(path)` (cdecl, ~14.6 KB, SEH): loads the whole saved game. Returns 1 if it loaded.
///
/// Disk (`exe::os_get_free_disk_space_mb`(0) < 25 sets [0xdd7134]) and memory ([0x9cd1fc] and `exe::os_get_free_memory_mb`() < 50 set [0xdd7138]); the temporary
/// folder (`exe::utils_build_file_path` "CM3_TEMP"); the CD (`check_game_cd`); a null `path` gives the "Error" box 0x233b. The database:
/// `exe::database_free_all_tables` if [0xae23a8] ≠ 0 and `load_database_from_stream`(path, 1) (the answer is not looked at).
///
/// "Loading game data" and the progress bar `exe::screenmanager_send_network_status_message` (step 0x2d); clears the `*.tmp` files (`utils_clear_temp_folder`), checks their
/// space (`zget_matching_size`, in MB, 0 becomes 25) against the disk, unpacks (`zextract_matching_files`; on failure: "Disk Space Error" 0x2382).
///
/// `general.dat` (`exe::zopen` reading, 0x15, type 4; without it: "Load Game Error" 0x2397): the version ([0xdd7110]); the
/// summary (|v| ≥ 0x3e023; 0xf70 bytes up to 0x3e026 with the play time zeroed, 0xf74 after that, with [0xdd7140] = its
/// play time); [0xdd7144] = time(); the unique id, the database size, the dates and times (the game's in a local), the
/// leagues, the default nation ([0xae2d08] = nation id, or 0 if < 0), the runtime data of people ([0xae2d0c], 0x4f ×
/// [0xae2d10]; old saves converted by `convert_staff_state`; |v| < 0x3e018 clears bit 7 of +0xb of the first
/// [0xae2364]), of clubs ([0xae2d14], 6 × [0xae2d18] = [0xae235c]; |v| < 0x3e022: −1, 0, 0) and the reputation
/// ([0xae2d1c], 4 × [0xae235c]; |v| < 0x3e022 does not read it and calls `database_sort_clubs_by_reputation`(1)). A failed read: "Load Game Error"
/// with the line and the field's text, item closed; without memory: "Error" and the game closes.
///
/// Steps 4..=0x2d (`LOAD_STEPS`, disk and memory checked before each): at step 5, the game date (`exe::date_operator_3d` into
/// 0xae2c90); before step 9, the last 16 managers with +0x5f = 5 are undone (News, HUMAN_MANAGER, CONTRACT_MANAGER,
/// NationalTeams, Staff_Records; empty names, date 1/0/1966, +0x24 and +0x39 = 0, `exe::screenmanager_remove_manager_record`); after 0x22, `exe::reservecomp_create_sub_comps`;
/// before the last one, |v| < 0x3e032 passes each person through `staff_normalise_player_attributes`.
///
/// FIFA rules (`load_fifa_transfer_rules`, |v| ≥ 0x3e039); 0xdd70a0 = the game date; [0xdd711c] = [0xdd7130] = 1, [0xa801bc] = 0; with
/// [0x9b5980], it switches the rules on or off by the date of [0xae2330] (`exe::news_post_transfer_system_change`(1) when switching on); without
/// `auto_setup.cfg`, the CD search at the end.
///
/// The original's branches for 0x3e042 ≤ |v| < 0x3e038 (0xd-byte records) and 0x3e042 ≤ |v| < 0x3e034 (0xc) are
/// unreachable (the comparisons are chained with `jge`): they are left out. The original's two-pass loop ([esp+0x24];
/// the 1st only sets the total 0x2d) runs the body once.
/// Original: `load_game_data` (0x82a9c0, Setup.cpp, cdecl).
///
/// [pt] { 0x82a9c0 } (cdecl, Setup.cpp, ~14,6 KB, SEH): `load_game_data(caminho)`: carrega o save inteiro.
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
pub(crate) unsafe extern "C" fn load_game_data(path: *const u8) -> u32 {
    let folder: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let translate: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    crate::comparison::branch("save load game");
    let u = |a: usize| *(a as *const u32);
    let sem_memoria = |minimum: i32| u(G_MEMORY_CHECK_ENABLED) != 0 && exe::os_get_free_memory_mb() < minimum;
    // the original’s two local dates (the 2nd is unused); their destructor is empty
    let (mut data_do_jogo, mut outra_data) = ([0u8; 8], [0u8; 8]);
    exe::date_date_54a110((data_do_jogo.as_mut_ptr()) as usize);
    exe::date_date_54a110((outra_data.as_mut_ptr()) as usize);
    if exe::os_get_free_disk_space_mb(0) < 0x19 {
        *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
        return 0;
    }
    if sem_memoria(0x32) {
        *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
        return 0;
    }
    let mut temporary = [0u8; 0x104];
    copy_text(temporary.as_mut_ptr(), folder(0x987da0, 0xad9c64));
    check_game_cd(true);
    if path.is_null() {
        setup_warning(0x233b);
        return 0;
    }
    if u(G_NATION_TABLE) != 0 {
        exe::database_free_all_tables();
    }
    exe::load_database_from_stream(path, 1);
    // disk and memory between steps: sets the flag and returns 0
    let falta_espaco = |minimo_disco: i32| {
        if exe::os_get_free_disk_space_mb(0) < minimo_disco {
            *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
            return true;
        }
        if sem_memoria(0x19) {
            *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
            return true;
        }
        false
    };
    let mut text = [0u8; 0xc8];
    translate(text.as_mut_ptr(), 0xa81790);
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, (text.as_ptr()) as usize, (1) as i32, 0x2d, -1);
    exe::utils_clear_temp_folder(0xa813ac);
    let n = exe::zget_matching_size(path, 0xa813ac);
    let mb = match n.wrapping_add((n >> 31) & 0xfffff) >> 20 {
        0 => 0x19,
        x => x,
    };
    if falta_espaco(mb) {
        return 0;
    }
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, (text.as_ptr()) as usize, (2) as i32, 0x2d, -1);
    if exe::zextract_matching_files(path, temporary.as_ptr(), 0xa813ac) == 0 {
        save_error_box(0x2382, 0xa81758, 0x9c7774, 0x9c7680);
        return 0;
    }
    if falta_espaco(0x19) {
        return 0;
    }
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, (text.as_ptr()) as usize, (3) as i32, 0x2d, -1);
    let mut z = exe::zopen((path) as usize, 0x9c7b84, 0, 0x15, 4);
    if z == 0 {
        save_error_box(0x2397, 0xa81720, 0x9af3e0, 0x9af334);
        return 0;
    }
    let mut version = 0i32;
    if (exe::zread(&mut version as *mut i32 as usize, 4, 1, z) as i32) != 1 {
        return load_read_error(0x239e, 0xa816e0, &mut z);
    }
    *(G_GAME_DATA_VERSION as *mut i32) = version;
    let v = version.wrapping_abs();
    let mut summary = [0u8; 0xf74];
    if v >= 0x3e023 {
        if v < 0x3e027 {
            if (exe::zread(summary.as_mut_ptr() as usize, 0xf70, 1, z) as i32) != 1 {
                return load_read_error(0x23b2, 0xa816a4, &mut z);
            }
            summary[0xf70..].fill(0);
        } else {
            if (exe::zread(summary.as_mut_ptr() as usize, 0xf74, 1, z) as i32) != 1 {
                return load_read_error(0x23bf, 0xa816a4, &mut z);
            }
            *(G_GAME_PLAY_TIME_TOTAL as *mut u32) = core::ptr::read_unaligned(summary.as_ptr().add(0xf6c) as *const u32);
        }
    }
    let mut now = 0i32;
    *(G_GAME_PLAY_TIME_LAST_STAMP as *mut i32) = exe::at_946ebb(&mut now);
    let mut nation = 0i32;
    let fields: [(usize, u32, u32, usize); 10] = [
        (G_GAME_RANDOM_SEED, 4, 0x23cf, 0xa8166c),
        (0xdd7114, 1, 0x23d7, 0xa81634),
        (G_GAME_START_DATE, 8, 0x23df, 0xa81604),
        (data_do_jogo.as_mut_ptr() as usize, 8, 0x23e7, 0xa81604),
        (G_CURRENT_DAY_SLICE, 1, 0x23ef, 0xa815d0),
        (G_LAST_PROCESSED_DATE, 8, 0x23f7, 0xa8159c),
        (0xae2cfc, 1, 0x23ff, 0xa81560),
        (G_NATIONS_TO_LOAD_COUNT, 4, 0x2407, 0xa8151c),
        (G_SELECTED_NATION_COUNT, 4, 0x240f, 0xa814cc),
        (&mut nation as *mut i32 as usize, 4, 0x2417, 0xa81490),
    ];
    for (buf, tam, linha, msg) in fields {
        if (exe::zread(buf, tam, 1, z) as i32) != 1 {
            return load_read_error(linha, msg, &mut z);
        }
    }
    *(G_MAIN_NATION as *mut usize) = if nation > -1 { (u(G_NATION_TABLE) as usize).wrapping_add((nation as usize).wrapping_mul(nation::SIZE)) } else { 0 };
    if (exe::zread(G_STAFF_STATE_COUNT, 4, 1, z) as i32) != 1 {
        return load_read_error(0x2420, 0xa81450, &mut z);
    }
    let persons = exe::crt_malloc(u(G_STAFF_STATE_COUNT).wrapping_mul(0x4f));
    *(G_STAFF_STATE_TABLE as *mut usize) = persons;
    if persons == 0 {
        load_fatal_error(0x2427);
    }
    if v < 0x3e045 {
        // old saves: records of 0x26 (|v| < 0x3e042) or 0x27 bytes, converted
        let (tam, linha_mem, linha_ler) = if v < 0x3e042 { (0x26u32, 0x242f, 0x2434) } else { (0x27, 0x24bd, 0x24c2) };
        let antigos = exe::crt_malloc(u(G_STAFF_STATE_COUNT).wrapping_mul(tam));
        if antigos == 0 {
            load_fatal_error(linha_mem);
        }
        if (exe::zread(antigos, tam, u(G_STAFF_STATE_COUNT), z) as i32) != u(G_STAFF_STATE_COUNT) as i32 {
            return load_read_error(linha_ler, 0xa81418, &mut z);
        }
        convert_staff_state(antigos, tam as usize, u(G_STAFF_STATE_COUNT) as i32);
        exe::crt_free(antigos);
    } else if (exe::zread(persons, 0x4f, u(G_STAFF_STATE_COUNT), z) as i32) != u(G_STAFF_STATE_COUNT) as i32 {
        return load_read_error(0x24f0, 0xa81418, &mut z);
    }
    if v < 0x3e018 {
        for i in 0..u(G_STAFF_COUNT) as i32 {
            *((u(G_STAFF_STATE_TABLE) as usize + 0x4f * i as usize + 0xb) as *mut u8) &= 0x7f;
        }
    }
    *(G_CLUB_STATE_COUNT as *mut u32) = u(G_CLUB_COUNT);
    let clubs = exe::crt_malloc(u(G_CLUB_COUNT).wrapping_mul(6));
    *(G_CLUB_STATE_TABLE as *mut usize) = clubs;
    if clubs == 0 {
        load_fatal_error(0x2503);
    }
    if v < 0x3e022 {
        for i in 0..u(G_CLUB_STATE_COUNT) as i32 {
            let c = u(G_CLUB_STATE_TABLE) as usize + 6 * i as usize;
            core::ptr::write_unaligned(c as *mut u32, 0xffffffff);
            *((u(G_CLUB_STATE_TABLE) as usize + 6 * i as usize + 4) as *mut u8) = 0;
            *((u(G_CLUB_STATE_TABLE) as usize + 6 * i as usize + 5) as *mut u8) = 0;
        }
        let r = exe::crt_malloc(u(G_CLUB_COUNT).wrapping_mul(4));
        *(G_CLUB_REPUTATION_ORDER as *mut usize) = r;
        if r == 0 {
            load_fatal_error(0x2516);
        }
        exe::database_sort_clubs_by_reputation(1);
    } else {
        if (exe::zread(clubs, 6, u(G_CLUB_STATE_COUNT), z) as i32) != u(G_CLUB_STATE_COUNT) as i32 {
            return load_read_error(0x251f, 0xa813e0, &mut z);
        }
        let r = exe::crt_malloc(u(G_CLUB_COUNT).wrapping_mul(4));
        *(G_CLUB_REPUTATION_ORDER as *mut usize) = r;
        if r == 0 {
            load_fatal_error(0x2528);
        }
        if (exe::zread(r, 4, u(G_CLUB_COUNT), z) as i32) != u(G_CLUB_COUNT) as i32 {
            setup_warning(0x252c);
            exe::zclose(&mut z);
            return 0;
        }
    }
    exe::zclose(&mut z);
    for (i, step_towards) in LOAD_STEPS.iter().enumerate() {
        let k = (i as u32 + 4).min(0x2d);
        if k == 9 {
            undo_holiday_managers();
        }
        if i == LOAD_STEPS.len() - 1 && v < 0x3e032 {
            for p in 0..u(G_STAFF_COUNT) as i32 {
                exe::staff_normalise_player_attributes(u(G_STAFF_TABLE) as usize + 0x6e * p as usize);
            }
        }
        if falta_espaco(0x19) {
            return 0;
        }
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, (text.as_ptr()) as usize, (k) as i32, 0x2d, -1);
        match *step_towards {
            LoadStep::Rotina(f, linha) => {
                let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(f);
                if f() == 0 && linha != 0 {
                    setup_warning(linha);
                    return 0;
                }
                if k == 5 {
                    let mut tmp = [0u8; 8];
                    exe::date_operator_3d(G_CURRENT_DATE, (tmp.as_mut_ptr()) as usize, data_do_jogo.as_ptr() as usize);
                }
            }
            LoadStep::Objeto(tam, construtor, global, linha, arg) => {
                let p = exe::crt_operator_new(tam);
                let o = if p == 0 {
                    0
                } else {
                    match arg {
                        SubsystemCtorArg::Caminho => {
                            let f: unsafe extern "thiscall" fn(usize, *const u8) -> usize = core::mem::transmute(construtor);
                            f(p, path)
                        }
                        SubsystemCtorArg::Sozinho => {
                            let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(construtor);
                            f(p)
                        }
                        SubsystemCtorArg::Estatisticas => {
                            let f: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(construtor);
                            f(p, path, 0x2710)
                        }
                        SubsystemCtorArg::Calendario => {
                            // the original pushes a dword with the year in the low 16 bits and stack leftover in the
                            // high ones; the constructor only uses the word (0x54cfe0 does & 0xffff)
                            let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(construtor);
                            f(p, *(G_CURRENT_DATE_YEAR as *const u16) as u32, 0, 1)
                        }
                        SubsystemCtorArg::Idioma => {
                            // the original pushes edx with the byte in dl (the rest is leftover); the constructor only reads the byte
                            let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(construtor);
                            f(p, *(G_DATABASE_NAMES_LANGUAGE as *const u8) as u32)
                        }
                    }
                };
                *(global as *mut usize) = o;
                // FOG_OF_WAR checks the Media global (as in saving)
                let conferido = if k == 0x29 { u(G_MEDIA) as usize } else { o };
                if conferido == 0 {
                    setup_warning(linha);
                    return 0;
                }
                if k == 0x22 {
                    exe::reservecomp_create_sub_comps(o);
                }
            }
        }
    }
    if v >= 0x3e039 {
        exe::load_fifa_transfer_rules(path);
    }
    *(G_GAME_DATA_LOADED_FLAG as *mut u32) = 1;
    *(G_GAME_HAS_SAVE_FILE_FLAG as *mut u32) = 1;
    *(G_GAME_UNSAVED_CHANGES_FLAG as *mut u32) = 0;
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(G_LAST_AUTOSAVE_DATE, (tmp.as_mut_ptr()) as usize, G_CURRENT_DATE);
    if *(G_FIFA_TRANSFER_RULES_FEATURE_ENABLED as *const u8) != 0 {
        // the rule-change date [0xae2330] (low word: day; high: year) against the game’s
        // [0xae2c90]: unsigned year, signed day
        let em_vigor = *(G_FIFA_TRANSFER_RULES_IN_FORCE as *const u8);
        let (dia_jogo, ano_jogo) = (*(G_CURRENT_DATE as *const i16), *(G_CURRENT_DATE_YEAR as *const u16));
        let (dia, ano) = (*(G_TRANSFER_RULE_CHANGE_DATE as *const i16), *(G_TRANSFER_RULE_CHANGE_DATE_YEAR as *const u16));
        let mut disable = false;
        if !(em_vigor == 0 && *(G_MAX_CONTRACT_LENGTH_YEARS as *const u8) == 0xa) {
            disable = ano > ano_jogo || (ano == ano_jogo && dia > dia_jogo);
        }
        if disable {
            *(G_FIFA_TRANSFER_RULES_IN_FORCE as *mut u8) = 0;
            *(G_MAX_CONTRACT_LENGTH_YEARS as *mut u8) = 0xa;
        } else if !(em_vigor != 0 && *(G_MAX_CONTRACT_LENGTH_YEARS as *const u8) == 5) && (ano < ano_jogo || (ano == ano_jogo && dia < dia_jogo)) {
            *(G_FIFA_TRANSFER_RULES_IN_FORCE as *mut u8) = 1;
            *(G_MAX_CONTRACT_LENGTH_YEARS as *mut u8) = 5;
            exe::news_post_transfer_system_change(1);
        }
    }
    if exe::utils_file_exists(0xa80340) == 0 {
        check_game_cd(false);
    }
    1
}

/// Before step 9 of the load: among the last 16 of the person table ([`0xae23bc`], 0x6e bytes, from the last
/// backwards), those with `+0x5f` = 5 stop being human managers.
///
/// `+0x5f` = 0, News `0x787630`, HUMAN_MANAGER `0x5fd2f0`/`0x5fd590`/`0x5fe140` (0, person), CONTRACT_MANAGER
/// `0x4ccbf0`/`0x4d0b50`/`0x4d0bd0`, NationalTeams `0x775ba0`, Staff_Records `0x86fb80`/`0x86fc70` (each with the
/// global as it is), the three names emptied, `+0x10` = date(1, 0, 1966, −1) (`0x54a130`), `+0x18` = 1966, `+0x24` =
/// `+0x39` = 0 and `0x7ff990` (screen manager, person, −1).
///
/// [pt] Antes do passo 9 da carga: dos 16 últimos da tabela de pessoas ([0xae23bc], 0x6e bytes, do último
/// para trás), os com +0x5f = 5 deixam de ser técnico humano: +0x5f = 0, News 0x787630, HUMAN_MANAGER
/// 0x5fd2f0/0x5fd590/0x5fe140 (0, pessoa), CONTRACT_MANAGER 0x4ccbf0/0x4d0b50/0x4d0bd0,
/// NationalTeams 0x775ba0, Staff_Records 0x86fb80/0x86fc70 (cada um com o global como está), os três
/// nomes vazios, +0x10 = data(1, 0, 1966, −1) (0x54a130), +0x18 = 1966, +0x24 = +0x39 = 0 e 0x7ff990
/// (gerente de telas, pessoa, −1).
pub(crate) unsafe fn undo_holiday_managers() {
    let u = |a: usize| *(a as *const usize);
    for i in 0..0x10usize {
        let s = Staff(u(G_STAFF_TABLE).wrapping_add(0x6e * ((*(G_STAFF_COUNT as *const i32)).wrapping_sub(i as i32).wrapping_sub(1) as usize)));
        if s.classification() != 5 {
            continue;
        }
        s.set_classification(0);
        exe::news_clear_inbox(u(G_NEWS), s);
        for f in [0x5fd2f0usize, 0x5fd590, 0x5fe140] {
            let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(f);
            f(u(G_HUMAN_MANAGER), 0, s.addr());
        }
        for (g, f) in [(G_CONTRACT_MANAGER, 0x4ccbf0usize), (G_CONTRACT_MANAGER, 0x4d0b50), (G_CONTRACT_MANAGER, 0x4d0bd0), (G_NATIONAL_TEAMS, 0x775ba0), (G_STAFF_RECORDS, 0x86fb80), (G_STAFF_RECORDS, 0x86fc70)] {
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
            f(u(g), s.addr());
        }
        s.set_classification(0);
        *(u(s.addr() + staff::FIRST_NAME) as *mut u8) = 0;
        *(u(s.addr() + staff::SECOND_NAME) as *mut u8) = 0;
        *(u(s.addr() + staff::COMMON_NAME) as *mut u8) = 0;
        exe::date_set(s.addr() + staff::DATE_OF_BIRTH, 1, 0, 0x7ae, -1);
        s.set_age((0x7ae) as i16);
        core::ptr::write_unaligned((s.addr() + staff::NATIONAL_JOB) as *mut u32, 0);
        core::ptr::write_unaligned((s.addr() + staff::CLUB_JOB) as *mut u32, 0);
        exe::screenmanager_remove_manager_record(G_SCREEN_MANAGER, s, -1);
    }
}

// simplification: loads the whole save (deletes and recreates the subsystems, unpacks the temporary folder);
// does not run twice. Original only; the copy carries the branch and is proven by the load in the save proof.

/// Setup.cpp "Error" box (setup_warning) followed by fatal cleanup and exit(-1).
///
/// [pt] A caixa "Error" de Setup.cpp (`setup_warning`) e o jogo para, como a original: 0x5cc7d0,
/// 0x5a8f60 e `exit(-1)` (0x945934).
pub(crate) unsafe fn setup_fatal_error(row: u32) -> ! {
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    setup_warning(row);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exit(-1)
}

/// `create_game_from_setup_file(cfg, qst)` (cdecl, ~3 KB): reads the configuration file `cfg` (`auto_setup.cfg` or
/// `quick_start.cfg`: database, stats, foreground and background leagues, language, engine, bids, holiday), creates the
/// game and, with `qst`, saves the quick start `CM3_QSTART\<qst>.qst` and frees everything; without `qst`, puts the
/// human manager on holiday at a drawn club. Returns 1 if it created the game.
///
/// Folders CM3_TEMP (`exe::utils_build_file_path`, only copied, as in the original) and CM3_DATA. A null `cfg` or one that does not open
/// (`exe::crt_fopen` "r"): the "Error" box (0x1c13, 0x1c1b) and 0.
///
/// Each line (`exe::utils_read_config_line`, 1000-byte buffer) goes through `exe::utils_trim_control_chars` and loses its spaces (one by one, like the original's
/// `strcpy`); the value is searched for with `strstr` (`exe::crt_strstr`) after the prefix: `DATABASE=` NORMAL/MINIMUM/MAXIMUM →
/// `exe::config_set_option`(1/2/4) in [0xae3138]; `MATCH_STATS=` NORMAL/SMALL/LARGE → 0x200/0x400/0x800; `FOREGROUND_LEAGUES=` and
/// `BACKGROUND_LEAGUES=`: on the first one, without a database ([0xae23a8] = 0), it loads the database (`exe::load_database_from_files`,
/// CM3_DATA; a failure stops the game), applies it (`exe::config_apply_and_save`), zeroes byte +0x11c of the nations, [0xae2d00],
/// [0xae2d04] (and [0xae2d08] for the foreground) and builds the key nations (`exe::key_nation_build_table`; a failure stops the game);
/// then, for each league in the table 0xb63d60 (0x23 of 0x48 bytes) with a nation and +8, if the name (`nation_get_english_three_letter_name`) is on
/// the line or `exe::nation_matches_region_filter` accepts it: foreground, flag 0/1 → ((~options & 2) | 1) << 1 and counts (+[0xae2d00] only from
/// 0, +[0xae2d04]) and the 1st that matches by `strncmp` (`exe::crt_strncmp`) or `exe::nation_matches_region_filter` becomes [0xae2d08]; background, 0 →
/// ((~options & 2) << 1) | 1 (+[0xae2d00]) and 2 → the same (−[0xae2d04]); `LANGUAGE=`: the 1st language i (table
/// 0x9eb244, 19) that exists (`exe::langlib_is_language_available`), is English, there is no database or the database has it (`database_language_file_exists`), and is on
/// the line: `exe::config_set_language`(i) and `exe::config_apply_and_save`; `ENABLE_MATCH_ENGINE=` YES/NO → byte [0xa5a390]; `ENABLE_COMPUTER_BIDS=` →
/// [0x9cd1f8]; `ON_HOLIDAY=` → the manager's holiday.
///
/// Closes the file; with no league flagged, no foreground league, no default nation or one without bit 2: 0. [0xdd7124]
/// = [0xdd713c] = 1; creates the game (`exe::create_game`(qst ≠ 0); a failure stops the game). With `qst`: `exe::crt_makepath` (CM3_QSTART,
/// `qst`, ".qst"), `save_game_data`(path, 0x17), `free_game_data`(1) and 1.
///
/// Without it: "auto_setup_game" in [0xae3138]+0x113, [0xdd7130] = 1, the date in 0xdd70a0; if the current human
/// (0xb74c26 + 0x300 × (i16)[0xb77c7a]) already has a person, 0. Otherwise, the 1st of the last 16 people (0xae23bc,
/// 0x6e) with +0x5f = 0 (without a table, box 0x39b3 and person 0) becomes the human: text +0xc zeroed, date +0x10
/// (`exe::date_set`: 1, 0, 1966, −1), +0x18 = 1966, +0x24 and +0x39 = 0, +0x5f = 1, `exe::human_manager_reset_human_record` and `set_on_holiday`(holiday, 0) on
/// [0xb63c98]; it draws (`exe::utils_random`) clubs (0xae23b4, 0x245) until one that is not a national side (`exe::database_is_national_team`), with a
/// nation of a foreground league (bit 2) and accepted by `club_can_be_managed`: its nation into +0x1a, the names (`database_pick_first_name_for_nation`,
/// `database_pick_second_name_for_nation`) into +4 and +8, and `exe::appoint_human_manager_to_club`(person, club). Returns 1.
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
    crate::comparison::branch("save create game from configuration");
    let opcoes = || *(G_CONFIG as *const usize);
    let u = |a: usize| *(a as *const u32);
    let i = |a: usize| *(a as *const i32);
    let mut temporary = [0u8; 0x104];
    copy_text(temporary.as_mut_ptr(), (exe::utils_build_file_path(0x987da0, 0xad9c64) as *const u8));
    let mut dados = [0u8; 0x104];
    copy_text(dados.as_mut_ptr(), (exe::utils_build_file_path(0x9b5348, 0xad9c64) as *const u8));
    if cfg.is_null() {
        setup_warning(0x1c13);
        return 0;
    }
    let f = exe::crt_fopen((cfg) as usize, 0x9899fc);
    if f == 0 {
        setup_warning(0x1c1b);
        return 0;
    }
    let mut holiday = 0u32;
    let mut row = [0u8; 0x3e8];
    let b = row.as_mut_ptr();
    // no database: loads, applies, zeroes the nations’ marks and the counts, builds the key nations
    let prepare_bench = |of_front: bool, error_bench: u32, error_key: u32| {
        if exe::load_database_from_files(dados.as_ptr(), 1) == 0 {
            setup_fatal_error(error_bench);
        }
        exe::config_apply_and_save(opcoes());
        let mut k = 0i32;
        let mut d = 0usize;
        while k < i(G_NATION_COUNT) {
            k += 1;
            *((u(G_NATION_TABLE) as usize + d + 0x11c) as *mut u8) = 0;
            d += 0x122;
        }
        *(G_NATIONS_TO_LOAD_COUNT as *mut u32) = 0;
        *(G_SELECTED_NATION_COUNT as *mut u32) = 0;
        if of_front {
            *(G_MAIN_NATION as *mut u32) = 0;
        }
        if exe::key_nation_build_table() == 0 {
            setup_fatal_error(error_key);
        }
    };
    while exe::utils_read_config_line(f, b) != 0 {
        exe::utils_trim_control_chars(b);
        if *b != 0 {
            let mut p = b;
            loop {
                if *p == b' ' {
                    copy_text(p, p.add(1));
                }
                p = p.add(1);
                if *p == 0 {
                    break;
                }
            }
        }
        let has = |o: usize, value: usize| exe::crt_strstr((b.add(o)) as usize, value) != 0;
        if exe::at_947120(b, 0xa81348, 9) == 0 {
            // DATABASE=
            if has(9, 0xa81340) {
                exe::config_set_option(opcoes(), 1);
            } else if has(9, 0xa81338) {
                exe::config_set_option(opcoes(), 2);
            } else if has(9, 0xa81330) {
                exe::config_set_option(opcoes(), 4);
            }
        } else if exe::at_947120(b, 0xa81320, 0xc) == 0 {
            // MATCH_STATS=
            if has(0xc, 0xa81340) {
                exe::config_set_option(opcoes(), 0x200);
            } else if has(0xc, 0xa81318) {
                exe::config_set_option(opcoes(), 0x400);
            } else if has(0xc, 0xa81310) {
                exe::config_set_option(opcoes(), 0x800);
            }
        } else if exe::at_947120(b, 0xa812fc, 0x13) == 0 {
            // FOREGROUND_LEAGUES=
            if u(G_NATION_TABLE) == 0 {
                prepare_bench(true, 0x1c49, 0x1c5d);
            }
            let v = b.add(0x13);
            let mut league = G_LEAGUE_WORLD_CUP_SETUP_FUNC;
            while league < G_KEY_NATION_SETUP_FUNC_LOOP_END {
                let n = u(league - 8) as usize;
                if n != 0 && u(league) != 0 && (exe::crt_strstr((v) as usize, exe::nation_get_english_three_letter_name(n) as usize) != 0 || exe::nation_matches_region_filter(n, v) != 0) {
                    let m = (n + nation::LEAGUE_SELECTED) as *mut u8;
                    let mark = ((!(u(opcoes() + 4) as u8) & 2) | 1) << 1;
                    if *m == 0 {
                        *m = mark;
                        *(G_NATIONS_TO_LOAD_COUNT as *mut i32) += 1;
                        *(G_SELECTED_NATION_COUNT as *mut i32) += 1;
                    } else if *m == 1 {
                        *m = mark;
                        *(G_SELECTED_NATION_COUNT as *mut i32) += 1;
                    }
                    let size = core::ffi::CStr::from_ptr(exe::nation_get_english_three_letter_name(n) as *const core::ffi::c_char).to_bytes().len() as u32;
                    if (exe::crt_strncmp((v) as usize, (exe::nation_get_english_three_letter_name(n)) as usize, size) == 0 || exe::nation_matches_region_filter(n, v) != 0) && u(G_MAIN_NATION) == 0 {
                        *(G_MAIN_NATION as *mut u32) = n as u32;
                    }
                }
                league += 0x48;
            }
        } else if exe::at_947120(b, 0xa812e8, 0x13) == 0 {
            // BACKGROUND_LEAGUES=
            if u(G_NATION_TABLE) == 0 {
                prepare_bench(false, 0x1c92, 0x1ca3);
            }
            let v = b.add(0x13);
            let mut league = G_LEAGUE_WORLD_CUP_SETUP_FUNC;
            while league < G_KEY_NATION_SETUP_FUNC_LOOP_END {
                let n = u(league - 8) as usize;
                if n != 0 && u(league) != 0 && (exe::crt_strstr((v) as usize, exe::nation_get_english_three_letter_name(n) as usize) != 0 || exe::nation_matches_region_filter(n, v) != 0) {
                    let m = (n + nation::LEAGUE_SELECTED) as *mut u8;
                    if *m == 0 {
                        *m = ((!(u(opcoes() + 4) as u8) & 2) << 1) | 1;
                        *(G_NATIONS_TO_LOAD_COUNT as *mut i32) += 1;
                    } else if *m == 2 {
                        *m = ((!(u(opcoes() + 4) as u8) & 2) << 1) | 1;
                        *(G_SELECTED_NATION_COUNT as *mut i32) -= 1;
                    }
                }
                league += 0x48;
            }
        } else if exe::at_947120(b, 0xa812dc, 9) == 0 {
            // LANGUAGE=
            let mut k = 0u32;
            let mut t = 0x9eb244usize;
            while t < 0x9eb290 {
                if exe::langlib_is_language_available(k) != 0
                    && (t == 0x9eb244 || u(G_NATION_TABLE) == 0 || exe::database_language_file_exists(k) != 0)
                    && has(9, u(t) as usize)
                {
                    exe::config_set_language(opcoes(), k);
                    exe::config_apply_and_save(opcoes());
                    break;
                }
                t += 4;
                k += 1;
            }
        } else if exe::at_947120(b, 0xa812c4, 0x14) == 0 {
            // ENABLE_MATCH_ENGINE=
            if has(0x14, 0xa812c0) {
                *(G_MATCH_ENGINE_ENABLED as *mut u8) = 1;
            } else if has(0x14, 0xa812bc) {
                *(G_MATCH_ENGINE_ENABLED as *mut u8) = 0;
            }
        } else if exe::at_947120(b, 0xa812a4, 0x15) == 0 {
            // ENABLE_COMPUTER_BIDS=
            if has(0x15, 0xa812c0) {
                *(G_COMPUTER_BIDS_ENABLED as *mut u32) = 1;
            } else if has(0x15, 0xa812bc) {
                *(G_COMPUTER_BIDS_ENABLED as *mut u32) = 0;
            }
        } else if exe::at_947120(b, 0xa81298, 0xb) == 0 {
            // ON_HOLIDAY=
            if has(0xb, 0xa812c0) {
                holiday = 1;
            } else if has(0xb, 0xa812bc) {
                holiday = 0;
            }
        }
    }
    exe::crt_fclose(f);
    if i(G_NATIONS_TO_LOAD_COUNT) <= 0 || i(G_SELECTED_NATION_COUNT) <= 0 {
        return 0;
    }
    let default = u(G_MAIN_NATION) as usize;
    if default == 0 || *((default + nation::LEAGUE_SELECTED) as *const u8) & 2 == 0 {
        return 0;
    }
    *(G_NEW_GAME_SETUP_DONE_FLAG as *mut u32) = 1;
    *(G_SETUP_FILE_GAME_FLAG as *mut u32) = 1;
    if exe::create_game(!qst.is_null() as u32) == 0 {
        setup_fatal_error(0x1d11);
    }
    if !qst.is_null() {
        let mut file = [0u8; 0x104];
        exe::crt_makepath(file.as_mut_ptr(), 0, ((exe::utils_build_file_path(0xa80204, 0xad9c64) as *const u8)) as usize, (qst) as usize, 0xa803b4);
        exe::save_game_data(file.as_ptr(), 0x17);
        exe::free_game_data(1);
        return 1;
    }
    copy_text((opcoes() + 0x113) as *mut u8, 0xa81288 as *const u8);
    *(G_GAME_HAS_SAVE_FILE_FLAG as *mut u32) = 1;
    let mut data = [0u8; 8];
    exe::date_operator_3d(G_LAST_AUTOSAVE_DATE, (data.as_mut_ptr()) as usize, G_CURRENT_DATE);
    let slot = || G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize).wrapping_mul(0x300));
    if core::ptr::read_unaligned(slot() as *const u32) != 0 {
        return 0;
    }
    let table = u(G_STAFF_TABLE) as usize;
    let mut p = 0usize;
    if table == 0 {
        setup_warning(0x39b3);
    } else {
        let mut q = table.wrapping_add((i(G_STAFF_COUNT) as usize).wrapping_mul(staff::SIZE)).wrapping_sub(0x6e);
        for _ in 0..0x10 {
            if *((q + staff::CLASSIFICATION) as *const u8) == 0 {
                p = q;
                break;
            }
            q = q.wrapping_sub(0x6e);
        }
    }
    core::ptr::write_unaligned(slot() as *mut u32, p as u32);
    *core::ptr::read_unaligned((p + staff::COMMON_NAME) as *const *mut u8) = 0;
    exe::date_set(p + staff::DATE_OF_BIRTH, 1, 0, 0x7ae, -1);
    core::ptr::write_unaligned((p + staff::YEAR_OF_BIRTH) as *mut u16, 0x7ae);
    core::ptr::write_unaligned((p + staff::NATIONAL_JOB) as *mut u32, 0);
    core::ptr::write_unaligned((p + staff::CLUB_JOB) as *mut u32, 0);
    *((p + staff::CLASSIFICATION) as *mut u8) = 1;
    exe::human_manager_reset_human_record(u(G_HUMAN_MANAGER) as usize, p);
    exe::human_manager_set_on_holiday(u(G_HUMAN_MANAGER) as usize, holiday, 0);
    let club = Club(loop {
        let r = exe::utils_random(i(G_CLUB_COUNT));
        let c = Club((u(G_CLUB_TABLE) as usize).wrapping_add((r as usize).wrapping_mul(club::SIZE)));
        if exe::database_is_national_team(c) != 0 {
            continue;
        }
        let nation = core::ptr::read_unaligned((c.addr() + club::NATION) as *const usize);
        if nation == 0 || *((nation + nation::LEAGUE_SELECTED) as *const u8) & 2 == 0 || exe::club_can_be_managed(c) == 0 {
            continue;
        }
        break c.addr();
    });
    let nation = core::ptr::read_unaligned((club.addr() + club::NATION) as *const usize);
    core::ptr::write_unaligned((p + staff::NATION) as *mut usize, nation);
    core::ptr::write_unaligned((p + staff::FIRST_NAME) as *mut u32, (exe::database_pick_first_name_for_nation(nation) as u32));
    let n2 = (exe::database_pick_second_name_for_nation(core::ptr::read_unaligned((p + staff::NATION) as *const usize)) as u32);
    core::ptr::write_unaligned((p + staff::SECOND_NAME) as *mut u32, n2);
    exe::appoint_human_manager_to_club(p, club);
    1
}

// simplification: creates the whole game (loads the database, rolls, writes the .qst and frees everything); does not run
// twice. Original only; the copy takes the branch and is proved by the save proof (the .qst) and by the
// carreiras (o auto_setup.cfg).

/// The save error box; the game continues.
///
/// The text "v%s %s.%s %d (%s)" (version 0x90d130, name and extension of Setup.cpp, `linha`, `texto`) at 0xae24d0; the
/// title `titulo` translated (0x66f4e0) at 0xae28bc, the body `corpo` (with the text at 0xae24d0) translated at
/// 0xae26c4 and the box `exe::gui_show_message_box`.
///
/// The game continues ([0xb67a34], G_MOUSE_LEFT_BUTTON_DOWN, stays as it is, unlike `setup_warning`).
///
/// [pt] A caixa de erro da gravação: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de Setup.cpp,
/// `linha`, `texto`) em 0xae24d0; o título `titulo` traduzido (0x66f4e0) em 0xae28bc, o corpo
/// `corpo` (com o texto de 0xae24d0) traduzido em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está, ao contrário do `setup_warning`).
pub(crate) unsafe fn save_error_box(row: u32, text: usize, title: usize, body: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, title);
    translate(0xae26c4, body, 0xae24d0);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// Steps 2..=0x24 of the save, in the original's order: (global that must be ≠ 0, routine, how to call).
///
/// `this` is the global itself, except for 0x5afad0 (FOG_OF_WAR), which the original checks via [0xb6b7c8] (Media's)
/// and calls with [0xae2a60]. Step 0x19 (`exe::key_nation_save_all_comps`, the competitions) has no global and returns 0 when it fails.
///
/// [pt] Os passos 2..=0x24 da gravação, na ordem da original: (global que tem de ser ≠ 0, rotina, como
/// chamar). `this` é o próprio global, menos no 0x5afad0 (FOG_OF_WAR), que a original confere por
/// [0xb6b7c8] (o de Media) e chama com [0xae2a60]. O passo 0x19 (0x6692f0, as competições) não
/// tem global e devolve 0 quando falha.
#[derive(Clone, Copy)]
pub(crate) enum SaveStep {
    /// thiscall(this, caminho), ret 4
    Caminho(usize, usize),
    /// thiscall(this), sem argumento
    Sozinho(usize, usize),
    /// thiscall(this, 1), ret 4 (FIXTURE_MANAGER::save_fixtures)
    Um(usize, usize),
    /// o método +0x1c da vtable de [global](1) (Friendly)
    Virtual(usize),
    /// Checks the 1st global and calls the routine with the 2nd (FOG_OF_WAR).
    ///
    /// [pt] confere o 1º global e chama a rotina com o 2º (FOG_OF_WAR)
    Outro(usize, usize, usize),
    /// cdecl() → 0 é erro (key_nation_save_all_comps)
    Competicoes,
}

pub(crate) const SAVE_STEPS: [SaveStep; 0x23] = [
    SaveStep::Caminho(G_HUMAN_MANAGER, 0x5ff9e0), // 2 HUMAN_MANAGER
    SaveStep::Caminho(G_SCOUT_MANAGER, 0x7f8020), // 3 SCOUT_MANAGER
    SaveStep::Caminho(G_NEWS, 0x787c30), // 4 News
    SaveStep::Virtual(G_FRIENDLY),           // 5 Friendly
    SaveStep::Sozinho(G_UEFA_SEEDING, 0x905510), // 6 UefaSeeding
    SaveStep::Sozinho(G_CONMEBOL_SEEDING, 0x4c6b70), // 7 ConmebolSeeding
    SaveStep::Caminho(G_STAFF_RECORDS, 0x86ffd0), // 8 Staff_Records
    SaveStep::Caminho(G_MATCH_STATS, 0x72adf0), // 9 Match_Stats
    SaveStep::Caminho(G_PLAYER_STATS, 0x7c55a0), // 0xa PlayerStats
    SaveStep::Caminho(G_CLUB_HISTORY, 0x443f50), // 0xb Club_History
    SaveStep::Caminho(G_CLUB_RECORDS, 0x451ff0), // 0xc Club_Records
    SaveStep::Caminho(G_TACTICS, 0x8995d0), // 0xd Tactics
    SaveStep::Sozinho(G_HOST_COUNTRY, 0x5fb320), // 0xe HostCountry
    SaveStep::Caminho(G_DISCIPLINE, 0x555dd0), // 0xf Discipline
    SaveStep::Caminho(G_AWARD_MANAGER, 0x4141a0), // 0x10 AWARD_MANAGER
    SaveStep::Caminho(G_FINE, 0x5a53d0), // 0x11 Fine
    SaveStep::Caminho(G_AWOL, 0x418980), // 0x12 AWOL
    SaveStep::Caminho(G_DISPUTE, 0x569ae0), // 0x13 DISPUTE
    SaveStep::Caminho(G_NOTES, 0x794f00), // 0x14 Notes
    SaveStep::Caminho(G_INJURY, 0x62b2d0), // 0x15 Injury
    SaveStep::Caminho(G_NATIONAL_TEAMS, 0x776a20), // 0x16 NationalTeams
    SaveStep::Sozinho(G_STADIUM, 0x85bea0), // 0x17 Stadium
    SaveStep::Um(G_FIXTURE_MANAGER, 0x5ac060),      // 0x18 FIXTURE_MANAGER
    SaveStep::Competicoes,                 // 0x19 key_nation_save_all_comps
    SaveStep::Caminho(G_CONTRACT_MANAGER, 0x4dc3a0), // 0x1a CONTRACT_MANAGER
    SaveStep::Caminho(G_FINANCE_TABLE, 0x59b3b0), // 0x1b FinanceTable
    SaveStep::Caminho(G_TRANSFER_MANAGER, 0x8bd4b0), // 0x1c TRANSFER_MANAGER
    SaveStep::Caminho(G_PLAYER_REGEN, 0x7ad7c0), // 0x1d PLAYER_REGEN
    SaveStep::Caminho(G_MEDIA, 0x72b5f0), // 0x1e Media
    SaveStep::Outro(G_MEDIA, G_FOG_OF_WAR, 0x5afad0), // 0x1f FOG_OF_WAR
    SaveStep::Caminho(G_MANAGER_MANAGER, 0x68e920), // 0x20 MANAGER_MANAGER
    SaveStep::Caminho(G_OFFICIALS_MANAGER, 0x79bb80), // 0x21 OfficialsManager
    SaveStep::Caminho(G_SIMULATED_STATS, 0x84c680), // 0x22 SimulatedStats
    SaveStep::Caminho(G_TRAINING_MANAGER, 0x8b1450), // 0x23 TRAINING_MANAGER
    SaveStep::Caminho(G_SQUAD_MANAGER, 0x859cc0), // 0x24 SQUAD_MANAGER
];

/// `save_game_data(path, a4)` (cdecl, ~6.8 KB, SEH): saves the whole game into the container `path` (a4 0x16 `.sav`,
/// 0x17 `.qst`; type 3 with bit 0x8000 of `game.cfg` [[0xae3138]+4], otherwise 4 = RLE): the database, general.dat, the
/// subsystem saves in the original order with disk and memory checks between them, the packed *.tmp files and the FIFA
/// rules; on a network game it writes to temp_save_file and renames. Returns 1 if it saved.
///
/// [0xdd7110] = 0xfffc1fb9 (the save version). On a network game (bit 0x20000000), before anything, disk (`exe::os_get_free_disk_space_mb`(0) <
/// 25) sets [0xdd7134] and memory ([0x9cd1fc] and `exe::os_get_free_memory_mb`() < 1) sets [0xdd7138]: deletes `path` and returns 0.
///
/// Temporary folder (`exe::utils_build_file_path` "CM3_TEMP"); a null `path` gives the "Error" box (0x2958). The working path is `path` or,
/// on a network game, the same with the name `temp_save_file` (`exe::crt_strtok`/`exe::crt_makepath`); if it already exists (`exe::utils_file_exists`) and
/// cannot be removed (0x94643a = −1), returns 0. With bit 0x40000000, disk and memory again (deleting `path`).
///
/// `save_database_for_game` (the database, step 1 of the save); disk and memory (deleting the working path); the translated "Saving
/// game data" and the progress bar `exe::screenmanager_send_network_status_message` (step 1 of 0x25).
///
/// `general.dat` (`exe::zopen`): 16 fields (the version, the 0xf74-byte summary `build_save_game_summary`, the unique id, the database size,
/// the dates and times, the leagues, the default nation or −1, the runtime data of people, clubs and reputation); a
/// write error gives the "Disk Space Error" box with the field's line and closes the item.
///
/// Steps 2..=0x24 (`SAVE_STEPS`): disk and memory (before 0x11, without deleting and with memory < 25), the progress
/// bar and the subsystem's `save`; 0x25: packs the `*.tmp` files (`zadd_matching_files`).
///
/// Hall of fame (`exe::halloffame_save`, outside the save), FIFA rules (`save_fifa_transfer_rules`); on a network game, swaps the old save for the new
/// one (`exe::crt_rename`); [0xdd7130] = 1, [0xa801bc] = 0 and the date of the last save (0xdd70a0).
///
/// The original's local date ([esp+0x1758], `exe::date_date_54a110`, empty destructor 0x5a53c0 in the SEH) is not used: it is left
/// out. The original's loop ([esp+0x11c] from 1 to 2) runs the body once, with the total 0x25. The summary is a zeroed
/// buffer here; in the original, what `build_save_game_summary` does not write (the club of the 16 non-human managers, the rest of the
/// texts) is leftover from its stack, saved into `general.dat`.
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
///   (0x944cff/0x945b13); se já exe::utils_file_exists((0x90c120) as usize) e não sai (0x94643a = −1), devolve 0. Com o bit
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
pub(crate) unsafe extern "C" fn save_game_data(path: *const u8, a4: u32) -> u32 {
    let folder: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let translate: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let write: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    crate::comparison::branch("save write game");
    let opcoes = || *((*(G_CONFIG as *const usize) + 4) as *const u32);
    let kind = || if opcoes() & 0x8000 != 0 { 3u32 } else { 4 };
    let sem_memoria = || *(G_MEMORY_CHECK_ENABLED as *const u32) != 0 && exe::os_get_free_memory_mb() < 1;
    *(G_GAME_DATA_VERSION as *mut u32) = 0xfffc1fb9;
    if opcoes() & 0x20000000 != 0 {
        let foul = if exe::os_get_free_disk_space_mb(0) < 0x19 {
            *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
            true
        } else if sem_memoria() {
            *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
            true
        } else {
            false
        };
        if foul {
            if !path.is_null() {
                exe::crt_remove(path);
            }
            return 0;
        }
    }
    let mut temporary = [0u8; 0x104];
    copy_text(temporary.as_mut_ptr(), folder(0x987da0, 0xad9c64));
    if path.is_null() {
        setup_warning(0x2958);
        return 0;
    }
    let mut local = [0u8; 0x104];
    if opcoes() & 0x20000000 != 0 {
        let (mut unidade, mut dir, mut nome, mut ext) = ([0u8; 0x104], [0u8; 0x104], [0u8; 0x100], [0u8; 0x100]);
        exe::crt_strtok((path) as usize, (unidade.as_mut_ptr()) as usize, (dir.as_mut_ptr()) as usize, nome.as_mut_ptr(), ext.as_mut_ptr());
        exe::crt_makepath(local.as_mut_ptr(), (unidade.as_ptr()) as usize, (dir.as_ptr()) as usize, 0xa81cf4, (ext.as_ptr()) as usize);
    } else {
        copy_text(local.as_mut_ptr(), path);
    }
    let l = local.as_ptr();
    if exe::utils_file_exists((l) as usize) != 0 && exe::crt_remove(l) == -1 {
        return 0;
    }
    if opcoes() & 0x40000000 != 0 {
        if exe::os_get_free_disk_space_mb(0) < 0x19 {
            *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
            exe::crt_remove(path);
            return 0;
        }
        if sem_memoria() {
            *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
            exe::crt_remove(path);
            return 0;
        }
    }
    exe::save_database_for_game(l, a4, kind());
    // disk and memory between the steps: sets the mark, deletes the working path and returns 0
    let falta_espaco = || {
        if exe::os_get_free_disk_space_mb(0) < 0x19 {
            *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
            exe::crt_remove(l);
            return true;
        }
        if sem_memoria() {
            *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
            exe::crt_remove(l);
            return true;
        }
        false
    };
    if falta_espaco() {
        return 0;
    }
    let mut text = [0u8; 0xc8];
    translate(text.as_mut_ptr(), 0xa81ce0);
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, (text.as_ptr()) as usize, (1) as i32, 0x25, -1);
    let mut z = exe::zopen((l) as usize, 0x9c7b84, 1, a4, kind());
    if z == 0 {
        save_error_box(0x29a3, 0xa81c88, 0xa81c70, 0xa81b5c);
        return 0;
    }
    // general.dat: (address, size, count, line and error text); the summary and the nation
    // default come from local buffers; the last three write vectors and check the count
    let mut summary = [0u8; 0xf74];
    let mut field = |buf: usize, size: u32, n: u32, row: u32, msg: usize| -> bool {
        if write(buf, size, n, z) == n {
            return true;
        }
        save_error_box(row, msg, 0x9c7774, 0x9c7680);
        exe::zclose(&mut z);
        false
    };
    let u = |a: usize| *(a as *const u32);
    if !field(G_GAME_DATA_VERSION, 4, 1, 0x29aa, 0xa81b1c) {
        return 0;
    }
    exe::build_save_game_summary(summary.as_mut_ptr());
    if !field(summary.as_ptr() as usize, 0xf74, 1, 0x29b5, 0xa81ae0)
        || !field(G_GAME_RANDOM_SEED, 4, 1, 0x29bd, 0xa81aa4)
        || !field(0xdd7114, 1, 1, 0x29c5, 0xa81a6c)
        || !field(G_GAME_START_DATE, 8, 1, 0x29cd, 0xa81a3c)
        || !field(G_CURRENT_DATE, 8, 1, 0x29d5, 0xa81a3c)
        || !field(G_CURRENT_DAY_SLICE, 1, 1, 0x29dd, 0xa81a04)
        || !field(G_LAST_PROCESSED_DATE, 8, 1, 0x29e5, 0xa819d0)
        || !field(0xae2cfc, 1, 1, 0x29ed, 0xa81994)
        || !field(G_NATIONS_TO_LOAD_COUNT, 4, 1, 0x29f5, 0xa8194c)
        || !field(G_SELECTED_NATION_COUNT, 4, 1, 0x29fd, 0xa818fc)
    {
        return 0;
    }
    let default = u(G_MAIN_NATION);
    let nation_default: u32 = if default != 0 { u(default as usize) } else { 0xffffffff };
    if !field(&nation_default as *const u32 as usize, 4, 1, 0x2a06, 0xa818c0)
        || !field(G_STAFF_STATE_COUNT, 4, 1, 0x2a0e, 0xa81880)
        || !field(u(G_STAFF_STATE_TABLE) as usize, 0x4f, u(G_STAFF_STATE_COUNT), 0x2a14, 0xa81848)
        || !field(u(G_CLUB_STATE_TABLE) as usize, 6, u(G_CLUB_STATE_COUNT), 0x2a1c, 0xa81814)
        || !field(u(G_CLUB_REPUTATION_ORDER) as usize, 4, u(G_CLUB_COUNT), 0x2a24, 0xa817dc)
    {
        return 0;
    }
    exe::zclose(&mut z);
    for (i, step_towards) in SAVE_STEPS.iter().enumerate() {
        let k = i as u32 + 2;
        // before step 0x11 (after AWARD_MANAGER) the original does not delete the path and asks
        // memory >= 25 (and not >= 1)
        if k == 0x11 {
            if exe::os_get_free_disk_space_mb(0) < 0x19 {
                *(G_GAME_IN_PROGRESS_FLAG as *mut u32) = 1;
                return 0;
            }
            if *(G_MEMORY_CHECK_ENABLED as *const u32) != 0 && exe::os_get_free_memory_mb() < 0x19 {
                *(G_GAME_SESSION_STARTED_FLAG as *mut u32) = 1;
                return 0;
            }
        } else if falta_espaco() {
            return 0;
        }
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, (text.as_ptr()) as usize, (k) as i32, 0x25, -1);
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
                if exe::key_nation_save_all_comps() == 0 {
                    setup_warning(0x2baa);
                    return 0;
                }
            }
        }
    }
    if falta_espaco() {
        return 0;
    }
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, (text.as_ptr()) as usize, (0x25) as i32, 0x25, -1);
    let (t, row) = if opcoes() & 0x8000 != 0 { (3, 0x2c82) } else { (4, 0x2c8a) };
    if exe::zadd_matching_files(l, temporary.as_ptr(), 0xa813ac, a4, t) == 0 {
        save_error_box(row, 0xa817a4, 0x9c7774, 0x9c7680);
        return 0;
    }
    exe::halloffame_save(u(G_HALL_OF_FAME) as usize);
    exe::save_fifa_transfer_rules(l);
    if opcoes() & 0x20000000 != 0 {
        if exe::utils_file_exists((path) as usize) != 0 && exe::crt_remove(path) == -1 {
            save_error_box(0x2ca7, 0xa81cc0, 0x9af90c, 0x9af828);
        }
        exe::crt_rename(l, path);
    }
    *(G_GAME_HAS_SAVE_FILE_FLAG as *mut u32) = 1;
    *(G_GAME_UNSAVED_CHANGES_FLAG as *mut u32) = 0;
    let mut data = [0u8; 8];
    exe::date_operator_3d(G_LAST_AUTOSAVE_DATE, (data.as_mut_ptr()) as usize, G_CURRENT_DATE);
    1
}

// simplification: writes the whole save (deletes the old one, creates the container, packs the temporary folder);
// does not run twice. Original only; the copy takes the branch and is proven by the save proof.

/// `check_saved_game_file(path)` (cdecl): file box filter for saved games: 1 if the container has id 0x16 (`.sav`,
/// `zget_archive_id`), otherwise 2.
///
/// The original only sets `ax`: the upper 16 bits of `eax` are those of the id answer, as here. A null path is the
/// Setup.cpp error 0x3a1b ("Error" box): the port calls the original.
/// Original: `check_saved_game_file` (0x838e10, Setup.cpp, cdecl).
///
/// [pt] { 0x838e10 } (cdecl, Setup.cpp): `check_saved_game_file(caminho)`: filtro da caixa de arquivos
/// dos jogos salvos: 1 se o contêiner tem o id 0x16 (`.sav`, 0x9331c0), senão 2. A original só
/// acerta `ax`: os 16 bits altos de `eax` ficam os da resposta do id, como aqui. Caminho nulo é o
/// erro de Setup.cpp 0x3a1b (caixa "Error"): a cópia chama a original.
pub(crate) unsafe extern "C" fn check_saved_game_file(path: usize) -> u32 {
    if path == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_SAVED_GAME_FILE)(path);
    }
    let id = exe::zget_archive_id(path);
    (id & 0xffff0000) | if id != 0x16 { 2 } else { 1 }
}

// The id only reads the file and frees what it allocates: both run side by side. Null path (box of
// error) original only.

/// `check_quick_start_file(path)` (cdecl): file box filter for quick starts: 1 if the container has id 0x17 (`.qst`)
/// and the saved version is the exe's (`valid_saved_game_version`), otherwise 2.
///
/// Only `ax` is set: the upper 16 bits of `eax` are those of the last call (the id, or the version). A null path:
/// Setup.cpp error 0x3a6d, the port calls the original.
/// Original: `check_quick_start_file` (0x839290, Setup.cpp, cdecl).
///
/// [pt] { 0x839290 } (cdecl, Setup.cpp): `check_quick_start_file(caminho)`: filtro da caixa de arquivos
/// do início rápido: 1 se o contêiner tem o id 0x17 (`.qst`) e a versão gravada é a do exe
/// (0x838eb0), senão 2. Os 16 bits altos de `eax` são os da última chamada (o id, ou a versão).
/// Caminho nulo: erro de Setup.cpp 0x3a6d, a cópia chama a original.
pub(crate) unsafe extern "C" fn check_quick_start_file(path: usize) -> u32 {
    if path == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_QUICK_START_FILE)(path);
    }
    crate::comparison::branch("save quick start filter");
    let id = exe::zget_archive_id(path);
    if id != 0x17 {
        return (id & 0xffff0000) | 2;
    }
    let v = exe::valid_saved_game_version(path);
    (v & 0xffff0000) | if v != 0 { 1 } else { 2 }
}

// simplification: the version (0x838eb0) opens general.dat and does not close it when it succeeds (the ZFILE and the
// file stay open, as in the original): running twice would leave two. Original only; the copy
// takes the branch and runs in Rust mode (only the file box calls it).

/// The "Load Game Error" box of `valid_saved_game_version`, with line and reason.
///
/// Text "v%s %s.%s %d (%s)" (version 0x90d130, file name and extension of Setup.cpp, line, reason) at 0xae24d0, the
/// translated title at 0xae28bc, the translated message (0x9af334) at 0xae26c4, and the box `exe::gui_show_message_box`. Unlike
/// `show_error_at`, it does not zero [0xb67a34].
///
/// [pt] A caixa "Load Game Error" de `valid_saved_game_version`: o texto "v%s %s.%s %d (%s)" (versão
/// 0x90d130, arquivo e extensão de Setup.cpp, linha, motivo) em 0xae24d0, o título traduzido em
/// 0xae28bc, a mensagem traduzida (0x9af334) em 0xae26c4 e a caixa 0x5e8290. Ao contrário do
/// `show_error_at`, não zera [0xb67a34].
pub(crate) unsafe fn load_version_error_box(row: u32, reason: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    // 0x90d130 ignores the arguments (returns the version); the original’s sprintf reads name, extension,
    // row and reason from the stack left by the call: here they go explicitly
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, reason);
    translate(0xae28bc, 0x9af3e0);
    translate(0xae26c4, 0x9af334, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// `valid_saved_game_version(path)` (cdecl): opens the container's `general.dat` item, reads the version and the
/// summary and returns 1 if the summary's version text is the exe's.
///
/// Opens the item (`exe::zopen` reading, a4 0x15, type 4), reads the saved version (4 bytes) and the save summary (0xf70
/// bytes if |version| < 0x3e027, otherwise 0xf74) and returns 1 if the summary's version text (+0x7d0) equals the exe's
/// (0x90d130), otherwise 0. Each failed read gives the "Load Game Error" box (Setup.cpp 0x3a36, 0x3a3d, 0x3a48, 0x3a55)
/// and 0. As in the original, on success the item is NOT closed (the ZFILE and the file stay open). The original's SEH
/// frame only protects the local date (`exe::date_date_54a110`, no destructor), which falls inside the summary (+0xf64) and is
/// overwritten by the read.
/// Original: `valid_saved_game_version` (0x838eb0, Setup.cpp, cdecl).
///
/// [pt] { 0x838eb0 } (cdecl, Setup.cpp): `valid_saved_game_version(caminho)`: abre o item `general.dat`
/// do contêiner (0x9326e0 lendo, a4 0x15, tipo 4), lê a versão gravada (4 bytes) e o resumo do save
/// (0xf70 bytes se |versão| < 0x3e027, senão 0xf74) e devolve 1 se o texto da versão do resumo
/// (+0x7d0) é igual ao do exe (0x90d130), senão 0. Cada leitura que falha dá a caixa "Load Game
/// Error" (Setup.cpp 0x3a36, 0x3a3d, 0x3a48, 0x3a55) e 0. Como a original, quando dá certo o item
/// NÃO é fechado (o ZFILE e o arquivo ficam abertos). O quadro SEH da original só protege a data
/// local (0x54a110, sem destrutor), que cai dentro do resumo (+0xf64) e é sobrescrita pela leitura.
pub(crate) unsafe extern "C" fn valid_saved_game_version(path: usize) -> u32 {
    let version: unsafe extern "C" fn() -> *const u8 = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    crate::comparison::branch("save saved game version");
    let mut summary = [0u8; 0xf74];
    let mut v = 0i32;
    exe::date_date_54a110(summary.as_mut_ptr() as usize + 0xf64);
    let mut z = exe::zopen(path, 0x9c7b84, 0, 0x15, 4);
    if z == 0 {
        load_version_error_box(0x3a36, 0xa82494);
        return 0;
    }
    if (exe::zread(&mut v as *mut i32 as usize, (4) as u32, (1) as u32, z) as i32) != 1 {
        load_version_error_box(0x3a3d, 0xa8244c);
        exe::zclose(&mut z);
        return 0;
    }
    if v.wrapping_abs() < 0x3e027 {
        if (exe::zread(summary.as_mut_ptr() as usize, (0xf70) as u32, (1) as u32, z) as i32) != 1 {
            load_version_error_box(0x3a48, 0xa82408);
            exe::zclose(&mut z);
            return 0;
        }
        // the old summary lacks the last field (+0xf70): the original zeroes it on the stack
        summary[0xf70..0xf74].fill(0);
    } else if (exe::zread(summary.as_mut_ptr() as usize, (0xf74) as u32, (1) as u32, z) as i32) != 1 {
        load_version_error_box(0x3a55, 0xa82408);
        exe::zclose(&mut z);
        return 0;
    }
    let exe = core::ffi::CStr::from_ptr(version() as *const core::ffi::c_char).to_bytes();
    let gravada = core::ffi::CStr::from_ptr(summary.as_ptr().add(0x7d0) as *const core::ffi::c_char).to_bytes();
    (exe == gravada) as u32
}

// simplification: opens the item and, when it succeeds, does not close it (the ZFILE and the file stay open, as in the
// original); on errors, the box. Does not run twice: original only; the copy takes the branch and runs in
// Rust mode (only the file box, through the quick-start filter, calls it).

/// The FIFA rules "Error" box: the text "v%s %s.%s %d" (version 0x90d130, file name and extension of Setup.cpp, line)
/// at 0xae24d0, the box `exe::gui_show_message_box` and [0xb67a34] (G_MOUSE_LEFT_BUTTON_DOWN) = 0.
///
/// Like `show_error_at`, but with the arguments that the original routine's sprintf reads from the stack (name,
/// extension and line) passed explicitly.
///
/// [pt] A caixa "Error" das regras FIFA: o texto "v%s %s.%s %d" (versão 0x90d130, arquivo e extensão de
/// Setup.cpp, linha) em 0xae24d0, a caixa 0x5e8290 e [0xb67a34] = 0. Como o `show_error_at`, mas com os
/// argumentos que o sprintf da original lê da pilha (nome, extensão e linha) passados explícitos.
pub(crate) unsafe fn setup_warning(row: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), row);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
}

/// `save_fifa_transfer_rules(save)` (cdecl): writes the last item of the save, `fifa_transfer_rules.zip`: 2 bytes,
/// [0xae233c] and the rule in force [0xa83608], then `zclose`.
///
/// The item is opened with `exe::zopen` writing, a4 0x16, type 4 (in an already created container its header applies). A
/// null path, an item that does not open or a failed write: the "Error" box (Setup.cpp 0x3b24, 0x3b2c, 0x3b32, 0x3b38);
/// on the writes, the item is closed after the box. The original returns nothing (`eax` is leftover).
/// Original: `save_fifa_transfer_rules` (0x8395b0, Setup.cpp, cdecl).
///
/// [pt] { 0x8395b0 } (cdecl, Setup.cpp): `save_fifa_transfer_rules(save)`: o último item do save,
/// `fifa_transfer_rules.zip` (0x9326e0 gravando, a4 0x16, tipo 4; no contêiner já criado vale o
/// cabeçalho dele): 2 bytes, [0xae233c] e [0xa83608], e `zclose`. Caminho nulo, item que não abre
/// ou escrita que falha: a caixa "Error" (Setup.cpp 0x3b24, 0x3b2c, 0x3b32, 0x3b38); nas escritas,
/// depois da caixa o item é fechado. A original não devolve nada (`eax` sobra).
pub(crate) unsafe extern "C" fn save_fifa_transfer_rules(save: usize) {
    crate::comparison::branch("save write fifa rules");
    if save == 0 {
        setup_warning(0x3b24);
        return;
    }
    let mut z = exe::zopen(save, 0xa82518, 1, 0x16, 4);
    if z == 0 {
        setup_warning(0x3b2c);
        return;
    }
    if (exe::zwrite(G_FIFA_TRANSFER_RULES_IN_FORCE, (1) as u32, (1) as u32, z) as i32) != 1 {
        setup_warning(0x3b32);
        exe::zclose(&mut z);
        return;
    }
    if (exe::zwrite(G_MAX_CONTRACT_LENGTH_YEARS, (1) as u32, (1) as u32, z) as i32) != 1 {
        setup_warning(0x3b38);
        exe::zclose(&mut z);
        return;
    }
    exe::zclose(&mut z);
}

// simplification: appends an item to the container; it does not run twice. Original only; the copy takes the
// branch and is proved by the save proof (the .qst written with and without the batch).

/// `load_fifa_transfer_rules(save)` (cdecl): reads the 2 bytes of the `fifa_transfer_rules.zip` item (`exe::zopen` reading)
/// into [0xae233c] and [0xa83608]; if [0xae233c] = 1, the rule in force [0xa83608] becomes 5. Closes the item.
///
/// Errors: the "Error" box (Setup.cpp 0x3b4f, 0x3b57, 0x3b5d, 0x3b63), as in the save.
/// Original: `load_fifa_transfer_rules` (0x839760, Setup.cpp, cdecl).
///
/// [pt] { 0x839760 } (cdecl, Setup.cpp): `load_fifa_transfer_rules(save)`: lê os 2 bytes do item
/// `fifa_transfer_rules.zip` (0x9326e0 lendo) em [0xae233c] e [0xa83608]; se [0xae233c] = 1, a
/// regra em vigor [0xa83608] vira 5. Fecha o item. Erros: a caixa "Error" (Setup.cpp 0x3b4f, 0x3b57,
/// 0x3b5d, 0x3b63), como na gravação.
pub(crate) unsafe extern "C" fn load_fifa_transfer_rules(save: usize) {
    crate::comparison::branch("save load fifa rules");
    if save == 0 {
        setup_warning(0x3b4f);
        return;
    }
    let mut z = exe::zopen(save, 0xa82518, 0, 0x16, 4);
    if z == 0 {
        setup_warning(0x3b57);
        return;
    }
    if (exe::zread(G_FIFA_TRANSFER_RULES_IN_FORCE, (1) as u32, (1) as u32, z) as i32) != 1 {
        setup_warning(0x3b5d);
        exe::zclose(&mut z);
        return;
    }
    if (exe::zread(G_MAX_CONTRACT_LENGTH_YEARS, (1) as u32, (1) as u32, z) as i32) != 1 {
        setup_warning(0x3b63);
        exe::zclose(&mut z);
        return;
    }
    if *(G_FIFA_TRANSFER_RULES_IN_FORCE as *const u8) == 1 {
        *(G_MAX_CONTRACT_LENGTH_YEARS as *mut u8) = 5;
    }
    exe::zclose(&mut z);
}

// simplification: opens and reads the container (allocates the ZFILE) and only runs on load; follows the save block rule
// (file = original only). The copy takes the branch; the load step of the save proof exercises it.

/// strcat: appends fonte to the end of destino.
///
/// [pt] O `strcat` da original (repne scasb + rep movs): `fonte` com o zero no fim de `destino`.
pub(crate) unsafe fn append_text(target: *mut u8, font: *const u8) {
    let d = target.add(core::ffi::CStr::from_ptr(target as *const core::ffi::c_char).to_bytes().len());
    copy_text(d, font);
}

/// strcpy: copies fonte with its terminator into destino.
///
/// [pt] O `strcpy` da original, com o zero.
pub(crate) unsafe fn copy_text(target: *mut u8, font: *const u8) {
    let n = core::ffi::CStr::from_ptr(font as *const core::ffi::c_char).to_bytes_with_nul().len();
    core::ptr::copy(font, target, n);
}

// Side by side over the whole summary (0xf74) and the match-time clock ([0xdd7140], [0xdd7144]),
// restored between the two; compares only +0..+0xf6c: the match time and time() (+0xf6c, +0xf70) are
// clock (fixed only in simulation mode), as in the save proof.

/// `free_game_data(screen)` (cdecl): releases the screens and destroys the subsystems held in globals, in the original
/// order; returns 1.
///
/// The screen manager (0xb74450) releases the screens (`exe::screenmanager_end_network_session`) and, if `screen` ≠ 0, also `exe::screenmanager_close_all_screens`; `free_event_tables`; then
/// each subsystem kept in a global, in the original's order: destructor (thiscall) + `delete` (`exe::crt_operator_delete`) and the global
/// zeroed; the loose vectors through `free` (`exe::crt_free`); two objects through the virtual destructor (vtable[0](1)); the
/// `*.tmp` files of the temporary folder (`utils_clear_temp_folder`), [0xae2338], the database tables (`exe::database_free_all_tables`), the game date
/// (`exe::date_set`: 1, 0, 2001, −1) and three flags zeroed.
/// Original: `free_game_data` (0x82ff40, Setup.cpp, cdecl).
///
/// [pt] { 0x82ff40 } (cdecl, Setup.cpp): `free_game_data(tela)`: o gerente de telas (0xb74450) solta as
/// telas (0x7ff5f0) e, se `tela` ≠ 0, mais 0x7fc5e0; 0x6d5b50; depois cada subsistema guardado num
/// global, na ordem da original: destrutor (thiscall) + `delete` (0x944c94) e o global zerado; os
/// vetores soltos pelo `free` (0x9452ca); dois objetos pelo destrutor virtual (vtable[0](1)); os
/// `*.tmp` da pasta temporária (0x90cd50), [0xae2338], as tabelas do banco (0x528080), a data do
/// jogo (0x54a130: 1, 0, 2001, −1) e três marcas zeradas. Devolve 1.
pub(crate) unsafe extern "C" fn free_game_data(tela: u32) -> u32 {
    crate::comparison::branch("save free game data");
    // destrutor + delete + global zerado
    let objetos = |list: &[(usize, usize)]| {
        for &(g, destrutor) in list {
            let p = *(g as *const usize);
            if p != 0 {
                let d: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destrutor);
                d(p);
                exe::crt_operator_delete(p);
                *(g as *mut usize) = 0;
            }
        }
    };
    // free + global zerado
    let soltos = |list: &[usize]| {
        for &g in list {
            let p = *(g as *const usize);
            if p != 0 {
                exe::crt_free(p);
                *(g as *mut usize) = 0;
            }
        }
    };
    // virtual destructor (vtable[0], with 1 = delete) + zeroed global
    let virtual_ = |g: usize| {
        let p = *(g as *const usize);
        if p != 0 {
            let d: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(**(p as *const *const usize));
            d(p, 1);
            *(g as *mut usize) = 0;
        }
    };
    exe::screenmanager_end_network_session(G_SCREEN_MANAGER);
    if tela != 0 {
        exe::screenmanager_close_all_screens(G_SCREEN_MANAGER);
    }
    exe::match_events_free_event_tables();
    objetos(&[(G_MATCH_DAY, 0x6b4190)]);
    let p = *(G_COMP_OBJECT_TABLE as *const usize);
    if p != 0 {
        exe::crt_free(p);
        *(G_COMP_OBJECT_TABLE as *mut usize) = 0;
        *(G_COMP_OBJECT_COUNT as *mut usize) = 0;
    }
    exe::key_nation_delete_all_comps();
    objetos(&[(G_MEDIA, 0x72b200), (G_FOG_OF_WAR, 0x5af6a0), (G_MANAGER_MANAGER, 0x68e0e0), (G_CLUB_HISTORY, 0x442e20),
              (G_CLUB_RECORDS, 0x4448a0), (G_TACTICS, 0x8961e0), (G_DISCIPLINE, 0x555790), (G_TRAINING_MANAGER, 0x8b1070),
              (G_AWARD_MANAGER, 0x413410), (G_TRANSFER_MANAGER, 0x8bbe20), (G_INJURY, 0x62a8a0), (G_FINE, 0x5a5120),
              (G_AWOL, 0x4185f0), (G_DISPUTE, 0x5698d0), (G_NOTES, 0x794ba0), (G_STADIUM, 0x85b7f0)]);
    virtual_(G_FRIENDLY);
    objetos(&[(G_NATIONAL_TEAMS, 0x76a710), (G_VIRTUAL_STAFF, 0x90d2e0), (G_UEFA_SEEDING, 0x903570), (G_CONMEBOL_SEEDING, 0x5a8f60),
              (G_STAFF_RECORDS, 0x8687c0), (G_MATCH_STATS, 0x7293c0), (G_PLAYER_STATS, 0x7c0d00), (G_HOST_COUNTRY, 0x58e330),
              (G_FIXTURE_MANAGER, 0x5aa0d0), (G_HUMAN_MANAGER, 0x5fb730), (G_SCOUT_MANAGER, 0x7f6aa0), (G_NEWS, 0x785d00)]);
    // [0xdd7ef0]: only the delete, no destructor
    let p = *(G_SQUAD_MANAGER as *const usize);
    if p != 0 {
        exe::crt_operator_delete(p);
        *(G_SQUAD_MANAGER as *mut usize) = 0;
    }
    virtual_(G_RESERVE_COMP);
    objetos(&[(G_CONTRACT_MANAGER, 0x4c74f0), (G_FINANCE_TABLE, 0x59a5a0)]);
    soltos(&[G_STAFF_STATE_TABLE, G_CLUB_STATE_TABLE, G_CLUB_REPUTATION_ORDER, G_PLAYER_ABILITY_VALUES_SORTED, G_PLAYER_DATA_PTRS_BY_ABILITY]);
    objetos(&[(G_PLAYER_REGEN, 0x7a56f0), (G_OFFICIALS_MANAGER, 0x79b800), (G_SIMULATED_STATS, 0x84be10), (G_WEATHER, 0x9227d0),
              (G_PUNDIT_LIST, 0x7da690)]);
    exe::utils_clear_temp_folder(0xa813ac);
    soltos(&[G_NATION_NAME_INDEX_TABLE]);
    exe::database_free_all_tables();
    exe::date_set(G_CURRENT_DATE, 1, 0, 0x7d1, -1);
    *(G_CURRENT_DAY_SLICE as *mut u8) = 0;
    *(G_GAME_DATA_LOADED_FLAG as *mut u32) = 0;
    *(G_GAME_HAS_SAVE_FILE_FLAG as *mut u32) = 0;
    1
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

