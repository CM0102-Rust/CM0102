use super::*;

/// Unrecoverable load error box (setup warning with the line), fatal cleanup and exit(-1).
pub(crate) unsafe fn load_fatal_error(row: u32) -> ! {
    setup_warning(row);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
    loop {}
}

/// Failed general.dat read: "Load Game Error" box with line and text, closes the item; returns 0.
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
pub(crate) unsafe fn convert_staff_state(font: usize, size: usize, n: i32) {
    let d = || read_usize(G_STAFF_STATE_TABLE);
    let mesclar = |base: u8, other: u8, m: u8| base ^ ((base ^ other) & m);
    for i in 0..n.max(0) as usize {
        let (o, f) = (i * 0x4f, i * size);
        write_u32(d() + o, read_u32(font + f));
        let x = d() + o + staff_state::STATUS_FLAGS;
        write_u8(x, mesclar(read_u8(font + (f + staff_state::STATUS_FLAGS)), read_u8(x), 0x7f));
        write_u8(d() + o + staff_state::INJURY_COUNTER, read_u8(font + (f + staff_state::INJURY_COUNTER)));
        write_u32(d() + o + 4, read_u32(font + f + 4));
        write_u8(d() + o + 9, read_u8(font + (f + 9)));
        write_u8(d() + o + 8, read_u8(font + (f + 8)));
        for m in [0xfu8, 0x40, 0x30] {
            let x = d() + o + staff_state::STATUS_FLAGS;
            write_u8(x, mesclar(read_u8(x), read_u8(font + (f + staff_state::STATUS_FLAGS)), m));
        }
        let c = d() + o + staff_state::EXTRA_FLAGS;
        write_u8(c, mesclar(read_u8(c), read_u8(font + (f + staff_state::EXTRA_FLAGS)), 1));
        if size == 0x26 {
            write_u8(d() + o + staff_state::B_INTERNATIONAL_PLAYED, 0);
            *((d() + o + staff_state::EXTRA_FLAGS) as *mut u8) &= !extra_flags::PLAYED_U21_INTERNATIONAL;
        } else {
            write_u8(d() + o + staff_state::B_INTERNATIONAL_PLAYED, read_u8(font + (f + staff_state::B_INTERNATIONAL_PLAYED)));
            let c = d() + o + staff_state::EXTRA_FLAGS;
            write_u8(c, mesclar(read_u8(c), read_u8(font + (f + staff_state::EXTRA_FLAGS)), extra_flags::PLAYED_U21_INTERNATIONAL));
        }
        for j in 0..5 {
            write_u8(d() + o + staff_state::FORMER_CLUB_WEIGHT + 5 * j, 0);
            write_u32(d() + o + staff_state::FORMER_CLUB_ID + 5 * j, 0xffffffff);
        }
        for j in 0..2 {
            for c in [0x33usize, 0x2b, 0x2f, 0x27, 0x37] {
                write_u32(d() + o + c + 0x14 * j, 0xffffffff);
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
pub(crate) unsafe fn check_game_cd(exigir: bool) {
    let units_digit: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
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

/// `load_game_data(path)` (cdecl, ~14.6 KB, SEH): loads the whole saved game. Returns 1 if it loaded. Disk
/// (`exe::os_get_free_disk_space_mb`(0) < 25 sets `G_GAME_IN_PROGRESS_FLAG`) and memory (`G_MEMORY_CHECK_ENABLED` and
/// `exe::os_get_free_memory_mb`() < 50 set `G_GAME_SESSION_STARTED_FLAG`); the temporary folder
/// (`exe::utils_build_file_path` "CM3_TEMP"); the CD (`check_game_cd`); a null `path` gives the "Error" box 0x233b. The
/// database: `exe::database_free_all_tables` if `G_NATION_TABLE` ≠ 0 and `load_database_from_stream`(path, 1) (the
/// answer is not looked at). "Loading game data" and the progress bar `exe::screenmanager_send_network_status_message`
/// (step 0x2d); clears the `*.tmp` files (`utils_clear_temp_folder`), checks their space (`zget_matching_size`, in MB,
/// 0 becomes 25) against the disk, unpacks (`zextract_matching_files`; on failure: "Disk Space Error" 0x2382).
/// `general.dat` (`exe::zopen` reading, 0x15, type 4; without it: "Load Game Error" 0x2397): the version
/// (`G_GAME_DATA_VERSION`); the summary (|v| ≥ 0x3e023; 0xf70 bytes up to 0x3e026 with the play time zeroed, 0xf74
/// after that, with `G_GAME_PLAY_TIME_TOTAL` = its play time); `G_GAME_PLAY_TIME_LAST_STAMP` = time(); the unique id,
/// the database size, the dates and times (the game's in a local), the leagues, the default nation (`G_MAIN_NATION` =
/// nation id, or 0 if < 0), the runtime data of people (`G_STAFF_STATE_TABLE`, 0x4f × `G_STAFF_STATE_COUNT`; old saves
/// converted by `convert_staff_state`; |v| < 0x3e018 clears bit 7 of `staff_state::STATUS_FLAGS` of the first
/// `G_STAFF_COUNT`), of clubs (`G_CLUB_STATE_TABLE`, 6 × `G_CLUB_STATE_COUNT` = `G_CLUB_COUNT`; |v| < 0x3e022: −1, 0,
/// 0) and the reputation (`G_CLUB_REPUTATION_ORDER`, 4 × `G_CLUB_COUNT`; |v| < 0x3e022 does not read it and calls
/// `database_sort_clubs_by_reputation`(1)). A failed read: "Load Game Error" with the line and the field's text, item
/// closed; without memory: "Error" and the game closes. Steps 4..=0x2d (`LOAD_STEPS`, disk and memory checked before
/// each): at step 5, the game date (`exe::date_operator_3d` into `G_CURRENT_DATE`); before step 9, the last 16 managers
/// with +0x5f = 5 are undone (News, HUMAN_MANAGER, CONTRACT_MANAGER, NationalTeams, Staff_Records; empty names, date
/// 1/0/1966, +0x24 and +0x39 = 0, `exe::screenmanager_remove_manager_record`); after 0x22,
/// `exe::reservecomp_create_sub_comps`; before the last one, |v| < 0x3e032 passes each person through
/// `staff_normalise_player_attributes`. FIFA rules (`load_fifa_transfer_rules`, |v| ≥ 0x3e039); `G_LAST_AUTOSAVE_DATE`
/// = the game date; `G_GAME_DATA_LOADED_FLAG` = `G_GAME_HAS_SAVE_FILE_FLAG` = 1, `G_GAME_UNSAVED_CHANGES_FLAG` = 0;
/// with `G_FIFA_TRANSFER_RULES_FEATURE_ENABLED`, it switches the rules on or off by the date of
/// `G_TRANSFER_RULE_CHANGE_DATE` (`exe::news_post_transfer_system_change`(1) when switching on); without
/// `auto_setup.cfg`, the CD search at the end. The original's branches for 0x3e042 ≤ |v| < 0x3e038 (0xd-byte records)
/// and 0x3e042 ≤ |v| < 0x3e034 (0xc) are unreachable (the comparisons are chained with `jge`): they are left out. The
/// original's two-pass loop ([esp+0x24]; the 1st only sets the total 0x2d) runs the body once.
/// Original: `load_game_data` (0x82a9c0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn load_game_data(path: *const u8) -> u32 {
    let translate: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("save load game");
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
    copy_text(temporary.as_mut_ptr(), exe::utils_build_file_path(0x987da0, 0xad9c64) as *const u8);
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
            *((u(G_STAFF_STATE_TABLE) as usize + staff_state::SIZE * i as usize + staff_state::STATUS_FLAGS) as *mut u8) &= 0x7f;
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
                exe::staff_normalise_player_attributes(u(G_STAFF_TABLE) as usize + staff::SIZE * p as usize);
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
                let checked = if k == 0x29 { u(G_MEDIA) as usize } else { o };
                if checked == 0 {
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
pub(crate) unsafe fn undo_holiday_managers() {
    let u = |a: usize| read_usize(a);
    for i in 0..0x10usize {
        let s = Staff(u(G_STAFF_TABLE).wrapping_add(staff::SIZE * (read_i32(G_STAFF_COUNT).wrapping_sub(i as i32).wrapping_sub(1) as usize)));
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
        write_u8(s.first_name(), 0);
        write_u8(s.second_name(), 0);
        write_u8(s.common_name(), 0);
        exe::date_set(s.addr() + staff::DATE_OF_BIRTH, 1, 0, 0x7ae, -1);
        s.set_age(0x7aeu16 as i16);
        s.set_national_job(Club((0) as usize));
        s.set_club_job(Club((0) as usize));
        exe::screenmanager_remove_manager_record(G_SCREEN_MANAGER, s, -1);
    }
}

// simplification: loads the whole save (deletes and recreates the subsystems, unpacks the temporary folder);
// does not run twice. Original only; the copy carries the branch and is proven by the load in the save proof.

/// Setup.cpp "Error" box (setup_warning) followed by fatal cleanup and exit(-1).
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
/// human manager on holiday at a drawn club. Returns 1 if it created the game. Folders CM3_TEMP
/// (`exe::utils_build_file_path`, only copied, as in the original) and CM3_DATA. A null `cfg` or one that does not open
/// (`exe::crt_fopen` "r"): the "Error" box (0x1c13, 0x1c1b) and 0. Each line (`exe::utils_read_config_line`, 1000-byte
/// buffer) goes through `exe::utils_trim_control_chars` and loses its spaces (one by one, like the original's
/// `strcpy`); the value is searched for with `strstr` (`exe::crt_strstr`) after the prefix: `DATABASE=`
/// NORMAL/MINIMUM/MAXIMUM → `exe::config_set_option`(1/2/4) in `G_CONFIG`; `MATCH_STATS=` NORMAL/SMALL/LARGE →
/// 0x200/0x400/0x800; `FOREGROUND_LEAGUES=` and `BACKGROUND_LEAGUES=`: on the first one, without a database
/// (`G_NATION_TABLE` = 0), it loads the database (`exe::load_database_from_files`, CM3_DATA; a failure stops the game),
/// applies it (`exe::config_apply_and_save`), zeroes byte `nation::LEAGUE_SELECTED` of the nations,
/// `G_NATIONS_TO_LOAD_COUNT`, `G_SELECTED_NATION_COUNT` (and `G_MAIN_NATION` for the foreground) and builds the key
/// nations (`exe::key_nation_build_table`; a failure stops the game); then, for each league in the table 0xb63d60 (0x23
/// of 0x48 bytes) with a nation and +8, if the name (`nation_get_english_three_letter_name`) is on the line or
/// `exe::nation_matches_region_filter` accepts it: foreground, flag 0/1 → ((~options & 2) | 1) << 1 and counts
/// (+`G_NATIONS_TO_LOAD_COUNT` only from 0, +`G_SELECTED_NATION_COUNT`) and the 1st that matches by `strncmp`
/// (`exe::crt_strncmp`) or `exe::nation_matches_region_filter` becomes `G_MAIN_NATION`; background, 0 → ((~options & 2)
/// << 1) | 1 (+`G_NATIONS_TO_LOAD_COUNT`) and 2 → the same (−`G_SELECTED_NATION_COUNT`); `LANGUAGE=`: the 1st language
/// i (table 0x9eb244, 19) that exists (`exe::langlib_is_language_available`), is English, there is no database or the
/// database has it (`database_language_file_exists`), and is on the line: `exe::config_set_language`(i) and
/// `exe::config_apply_and_save`; `ENABLE_MATCH_ENGINE=` YES/NO → byte `G_MATCH_ENGINE_ENABLED`; `ENABLE_COMPUTER_BIDS=`
/// → `G_COMPUTER_BIDS_ENABLED`; `ON_HOLIDAY=` → the manager's holiday. Closes the file; with no league flagged, no
/// foreground league, no default nation or one without bit 2: 0. `G_NEW_GAME_SETUP_DONE_FLAG` =
/// `G_SETUP_FILE_GAME_FLAG` = 1; creates the game (`exe::create_game`(qst ≠ 0); a failure stops the game). With `qst`:
/// `exe::crt_makepath` (CM3_QSTART, `qst`, ".qst"), `save_game_data`(path, 0x17), `free_game_data`(1) and 1. Without
/// it: "auto_setup_game" in `G_CONFIG` + `config::SAVE_NAME`, `G_GAME_HAS_SAVE_FILE_FLAG` = 1, the date in
/// `G_LAST_AUTOSAVE_DATE`; if the current human (`G_SCREEN_RECORDS_MANAGER` + 0x300 ×
/// (i16)`G_SCREEN_MANAGER_CURRENT_SLOT`) already has a person, 0. Otherwise, the 1st of the last 16 people
/// (`G_STAFF_TABLE`, 0x6e) with `staff::CLASSIFICATION` = 0 (without a table, box 0x39b3 and person 0) becomes the
/// human: text `staff::COMMON_NAME` zeroed, date `staff::DATE_OF_BIRTH` (`exe::date_set`: 1, 0, 1966, −1),
/// `staff::YEAR_OF_BIRTH` = 1966, `staff::NATIONAL_JOB` and `staff::CLUB_JOB` = 0, `staff::CLASSIFICATION` = 1,
/// `exe::human_manager_reset_human_record` and `exe::human_manager_set_on_holiday`(holiday, 0) on `G_HUMAN_MANAGER`; it
/// draws (`exe::utils_random`) clubs (`G_CLUB_TABLE`, `club::SIZE`) until one that is not a national side
/// (`exe::database_is_national_team`), with a nation of a foreground league (bit 2) and accepted by
/// `club_can_be_managed`: its nation into `staff::NATION`, the names (`exe::database_pick_first_name_for_nation`,
/// `exe::database_pick_second_name_for_nation`) into `staff::FIRST_NAME` and `staff::SECOND_NAME`, and
/// `exe::appoint_human_manager_to_club`(person, club). Returns 1.
/// Original: `create_game_from_setup_file` (0x827110, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_game_from_setup_file(cfg: *const u8, qst: *const u8) -> u32 {
    covered!("save create game from configuration");
    let options = || read_usize(G_CONFIG);
    let u = |a: usize| read_u32(a);
    let i = |a: usize| read_i32(a);
    let mut temporary = [0u8; 0x104];
    copy_text(temporary.as_mut_ptr(), exe::utils_build_file_path(0x987da0, 0xad9c64) as *const u8);
    let mut dados = [0u8; 0x104];
    copy_text(dados.as_mut_ptr(), exe::utils_build_file_path(0x9b5348, 0xad9c64) as *const u8);
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
        exe::config_apply_and_save(options());
        let mut k = 0i32;
        let mut d = 0usize;
        while k < i(G_NATION_COUNT) {
            k += 1;
            write_u8(u(G_NATION_TABLE) as usize + d + nation::LEAGUE_SELECTED, 0);
            d += 0x122;
        }
        write_u32(G_NATIONS_TO_LOAD_COUNT, 0);
        write_u32(G_SELECTED_NATION_COUNT, 0);
        if of_front {
            write_u32(G_MAIN_NATION, 0);
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
                exe::config_set_option(options(), 1);
            } else if has(9, 0xa81338) {
                exe::config_set_option(options(), 2);
            } else if has(9, 0xa81330) {
                exe::config_set_option(options(), 4);
            }
        } else if exe::at_947120(b, 0xa81320, 0xc) == 0 {
            // MATCH_STATS=
            if has(0xc, 0xa81340) {
                exe::config_set_option(options(), 0x200);
            } else if has(0xc, 0xa81318) {
                exe::config_set_option(options(), 0x400);
            } else if has(0xc, 0xa81310) {
                exe::config_set_option(options(), 0x800);
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
                    let mark = ((!(u(options() + 4) as u8) & 2) | 1) << 1;
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
                        write_u32(G_MAIN_NATION, n as u32);
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
                        *m = ((!(u(options() + 4) as u8) & 2) << 1) | 1;
                        *(G_NATIONS_TO_LOAD_COUNT as *mut i32) += 1;
                    } else if *m == 2 {
                        *m = ((!(u(options() + 4) as u8) & 2) << 1) | 1;
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
                    exe::config_set_language(options(), k);
                    exe::config_apply_and_save(options());
                    break;
                }
                t += 4;
                k += 1;
            }
        } else if exe::at_947120(b, 0xa812c4, 0x14) == 0 {
            // ENABLE_MATCH_ENGINE=
            if has(0x14, 0xa812c0) {
                write_u8(G_MATCH_ENGINE_ENABLED, 1);
            } else if has(0x14, 0xa812bc) {
                write_u8(G_MATCH_ENGINE_ENABLED, 0);
            }
        } else if exe::at_947120(b, 0xa812a4, 0x15) == 0 {
            // ENABLE_COMPUTER_BIDS=
            if has(0x15, 0xa812c0) {
                write_u32(G_COMPUTER_BIDS_ENABLED, 1);
            } else if has(0x15, 0xa812bc) {
                write_u32(G_COMPUTER_BIDS_ENABLED, 0);
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
    let default = Nation(u(G_MAIN_NATION) as usize);
    if default.is_null() || default.league_selected() & 2 == 0 {
        return 0;
    }
    write_u32(G_NEW_GAME_SETUP_DONE_FLAG, 1);
    write_u32(G_SETUP_FILE_GAME_FLAG, 1);
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
    copy_text((options() + config::SAVE_NAME) as *mut u8, 0xa81288 as *const u8);
    write_u32(G_GAME_HAS_SAVE_FILE_FLAG, 1);
    let mut data = [0u8; 8];
    exe::date_operator_3d(G_LAST_AUTOSAVE_DATE, (data.as_mut_ptr()) as usize, G_CURRENT_DATE);
    let slot = || G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize).wrapping_mul(0x300));
    if read_u32(slot()) != 0 {
        return 0;
    }
    let table = u(G_STAFF_TABLE) as usize;
    let mut p = 0usize;
    if table == 0 {
        setup_warning(0x39b3);
    } else {
        let mut q = table.wrapping_add((i(G_STAFF_COUNT) as usize).wrapping_mul(staff::SIZE)).wrapping_sub(0x6e);
        for _ in 0..0x10 {
            if read_u8(q + staff::CLASSIFICATION) == 0 {
                p = q;
                break;
            }
            q = q.wrapping_sub(0x6e);
        }
    }
    write_u32(slot(), p as u32);
    *core::ptr::read_unaligned((p + staff::COMMON_NAME) as *const *mut u8) = 0;
    exe::date_set(p + staff::DATE_OF_BIRTH, 1, 0, 0x7ae, -1);
    write_u16(p + staff::YEAR_OF_BIRTH, 0x7ae);
    write_u32(p + staff::NATIONAL_JOB, 0);
    write_u32(p + staff::CLUB_JOB, 0);
    write_u8(p + staff::CLASSIFICATION, 1);
    exe::human_manager_reset_human_record(u(G_HUMAN_MANAGER) as usize, p);
    exe::human_manager_set_on_holiday(u(G_HUMAN_MANAGER) as usize, holiday, 0);
    let club = Club(loop {
        let r = exe::utils_random(i(G_CLUB_COUNT));
        let c = Club((u(G_CLUB_TABLE) as usize).wrapping_add((r as usize).wrapping_mul(club::SIZE)));
        if exe::database_is_national_team(c) != 0 {
            continue;
        }
        let nation = c.nation().addr();
        if nation == 0 || read_u8(nation + nation::LEAGUE_SELECTED) & 2 == 0 || exe::club_can_be_managed(c) == 0 {
            continue;
        }
        break c.addr();
    });
    let nation = club.nation().addr();
    write_usize(p + staff::NATION, nation);
    write_u32(p + staff::FIRST_NAME, exe::database_pick_first_name_for_nation(nation) as u32);
    let n2 = exe::database_pick_second_name_for_nation(read_usize(p + staff::NATION)) as u32;
    write_u32(p + staff::SECOND_NAME, n2);
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
/// 0x17 `.qst`; type 3 with bit 0x8000 of `game.cfg` [`G_CONFIG`+4], otherwise 4 = RLE): the database, general.dat, the
/// subsystem saves in the original order with disk and memory checks between them, the packed *.tmp files and the FIFA
/// rules; on a network game it writes to temp_save_file and renames. Returns 1 if it saved. `G_GAME_DATA_VERSION` =
/// 0xfffc1fb9 (the save version). On a network game (bit 0x20000000), before anything, disk
/// (`exe::os_get_free_disk_space_mb`(0) < 25) sets `G_GAME_IN_PROGRESS_FLAG` and memory (`G_MEMORY_CHECK_ENABLED` and
/// `exe::os_get_free_memory_mb`() < 1) sets `G_GAME_SESSION_STARTED_FLAG`: deletes `path` and returns 0. Temporary
/// folder (`exe::utils_build_file_path` "CM3_TEMP"); a null `path` gives the "Error" box (0x2958). The working path is
/// `path` or, on a network game, the same with the name `temp_save_file` (`exe::crt_strtok`/`exe::crt_makepath`); if it
/// already exists (`exe::utils_file_exists`) and cannot be removed (`exe::crt_remove` = −1), returns 0. With bit
/// 0x40000000, disk and memory again (deleting `path`). `save_database_for_game` (the database, step 1 of the save);
/// disk and memory (deleting the working path); the translated "Saving game data" and the progress bar
/// `exe::screenmanager_send_network_status_message` (step 1 of 0x25). `general.dat` (`exe::zopen`): 16 fields (the
/// version, the 0xf74-byte summary `build_save_game_summary`, the unique id, the database size, the dates and times,
/// the leagues, the default nation or −1, the runtime data of people, clubs and reputation); a write error gives the
/// "Disk Space Error" box with the field's line and closes the item. Steps 2..=0x24 (`SAVE_STEPS`): disk and memory
/// (before 0x11, without deleting and with memory < 25), the progress bar and the subsystem's `save`; 0x25: packs the
/// `*.tmp` files (`zadd_matching_files`). Hall of fame (`exe::halloffame_save`, outside the save), FIFA rules
/// (`save_fifa_transfer_rules`); on a network game, swaps the old save for the new one (`exe::crt_rename`);
/// `G_GAME_HAS_SAVE_FILE_FLAG` = 1, `G_GAME_UNSAVED_CHANGES_FLAG` = 0 and the date of the last save
/// (`G_LAST_AUTOSAVE_DATE`). The original's local date ([esp+0x1758], `exe::date_date_54a110`, empty destructor
/// 0x5a53c0 in the SEH) is not used: it is left out. The original's loop ([esp+0x11c] from 1 to 2) runs the body once,
/// with the total 0x25. The summary is a zeroed buffer here; in the original, what `build_save_game_summary` does not
/// write (the club of the 16 non-human managers, the rest of the texts) is leftover from its stack, saved into
/// `general.dat`.
/// Original: `save_game_data` (0x82e430, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn save_game_data(path: *const u8, a4: u32) -> u32 {
    let folder: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let translate: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let write: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    covered!("save write game");
    let options = || read_u32(read_usize(G_CONFIG) + 4);
    let kind = || if options() & 0x8000 != 0 { 3u32 } else { 4 };
    let sem_memoria = || read_u32(G_MEMORY_CHECK_ENABLED) != 0 && exe::os_get_free_memory_mb() < 1;
    write_u32(G_GAME_DATA_VERSION, 0xfffc1fb9);
    if options() & 0x20000000 != 0 {
        let foul = if exe::os_get_free_disk_space_mb(0) < 0x19 {
            write_u32(G_GAME_IN_PROGRESS_FLAG, 1);
            true
        } else if sem_memoria() {
            write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
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
    if options() & 0x20000000 != 0 {
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
    if options() & 0x40000000 != 0 {
        if exe::os_get_free_disk_space_mb(0) < 0x19 {
            write_u32(G_GAME_IN_PROGRESS_FLAG, 1);
            exe::crt_remove(path);
            return 0;
        }
        if sem_memoria() {
            write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
            exe::crt_remove(path);
            return 0;
        }
    }
    exe::save_database_for_game(l, a4, kind());
    // disk and memory between the steps: sets the mark, deletes the working path and returns 0
    let falta_espaco = || {
        if exe::os_get_free_disk_space_mb(0) < 0x19 {
            write_u32(G_GAME_IN_PROGRESS_FLAG, 1);
            exe::crt_remove(l);
            return true;
        }
        if sem_memoria() {
            write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
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
    let u = |a: usize| read_u32(a);
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
                write_u32(G_GAME_IN_PROGRESS_FLAG, 1);
                return 0;
            }
            if read_u32(G_MEMORY_CHECK_ENABLED) != 0 && exe::os_get_free_memory_mb() < 0x19 {
                write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
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
                    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(o) + comp_vtable::SAVE_TO_TEMP_FILE));
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
    let (t, row) = if options() & 0x8000 != 0 { (3, 0x2c82) } else { (4, 0x2c8a) };
    if exe::zadd_matching_files(l, temporary.as_ptr(), 0xa813ac, a4, t) == 0 {
        save_error_box(row, 0xa817a4, 0x9c7774, 0x9c7680);
        return 0;
    }
    exe::halloffame_save(u(G_HALL_OF_FAME) as usize);
    exe::save_fifa_transfer_rules(l);
    if options() & 0x20000000 != 0 {
        if exe::utils_file_exists((path) as usize) != 0 && exe::crt_remove(path) == -1 {
            save_error_box(0x2ca7, 0xa81cc0, 0x9af90c, 0x9af828);
        }
        exe::crt_rename(l, path);
    }
    write_u32(G_GAME_HAS_SAVE_FILE_FLAG, 1);
    write_u32(G_GAME_UNSAVED_CHANGES_FLAG, 0);
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
pub(crate) unsafe extern "C" fn check_quick_start_file(path: usize) -> u32 {
    if path == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_QUICK_START_FILE)(path);
    }
    covered!("save quick start filter");
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
/// summary and returns 1 if the summary's version text is the exe's. Opens the item (`exe::zopen` reading, a4 0x15,
/// type 4), reads the saved version (4 bytes) and the save summary (0xf70 bytes if |version| < 0x3e027, otherwise
/// 0xf74) and returns 1 if the summary's version text (+0x7d0) equals the exe's (`exe::utils_get_version_string`),
/// otherwise 0. Each failed read gives the "Load Game Error" box (Setup.cpp 0x3a36, 0x3a3d, 0x3a48, 0x3a55) and 0. As
/// in the original, on success the item is NOT closed (the ZFILE and the file stay open). The original's SEH frame only
/// protects the local date (`exe::date_date_54a110`, no destructor), which falls inside the summary
/// (`save_summary::GAME_DATE`) and is overwritten by the read.
/// Original: `valid_saved_game_version` (0x838eb0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn valid_saved_game_version(path: usize) -> u32 {
    covered!("save saved game version");
    let mut summary = [0u8; 0xf74];
    let mut v = 0i32;
    exe::date_date_54a110(summary.as_mut_ptr() as usize + save_summary::GAME_DATE);
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
    let exe = core::ffi::CStr::from_ptr(exe::utils_get_version_string() as *const core::ffi::c_char).to_bytes();
    let written = core::ffi::CStr::from_ptr(summary.as_ptr().add(0x7d0) as *const core::ffi::c_char).to_bytes();
    (exe == written) as u32
}

// simplification: opens the item and, when it succeeds, does not close it (the ZFILE and the file stay open, as in the
// original); on errors, the box. Does not run twice: original only; the copy takes the branch and runs in
// Rust mode (only the file box, through the quick-start filter, calls it).

/// The FIFA rules "Error" box: the text "v%s %s.%s %d" (version 0x90d130, file name and extension of Setup.cpp, line)
/// at 0xae24d0, the box `exe::gui_show_message_box` and [0xb67a34] (G_MOUSE_LEFT_BUTTON_DOWN) = 0.
///
/// Like `show_error_at`, but with the arguments that the original routine's sprintf reads from the stack (name,
/// extension and line) passed explicitly.
pub(crate) unsafe fn setup_warning(row: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), row);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// `save_fifa_transfer_rules(save)` (cdecl): writes the last item of the save, `fifa_transfer_rules.zip`: 2 bytes,
/// `G_FIFA_TRANSFER_RULES_IN_FORCE` and the rule in force `G_MAX_CONTRACT_LENGTH_YEARS`, then `zclose`. The item is
/// opened with `exe::zopen` writing, a4 0x16, type 4 (in an already created container its header applies). A null path,
/// an item that does not open or a failed write: the "Error" box (Setup.cpp 0x3b24, 0x3b2c, 0x3b32, 0x3b38); on the
/// writes, the item is closed after the box. The original returns nothing (`eax` is leftover).
/// Original: `save_fifa_transfer_rules` (0x8395b0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn save_fifa_transfer_rules(save: usize) {
    covered!("save write fifa rules");
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

/// `load_fifa_transfer_rules(save)` (cdecl): reads the 2 bytes of the `fifa_transfer_rules.zip` item (`exe::zopen`
/// reading) into `G_FIFA_TRANSFER_RULES_IN_FORCE` and `G_MAX_CONTRACT_LENGTH_YEARS`; if
/// `G_FIFA_TRANSFER_RULES_IN_FORCE` = 1, the rule in force `G_MAX_CONTRACT_LENGTH_YEARS` becomes 5. Closes the item.
/// Errors: the "Error" box (Setup.cpp 0x3b4f, 0x3b57, 0x3b5d, 0x3b63), as in the save.
/// Original: `load_fifa_transfer_rules` (0x839760, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn load_fifa_transfer_rules(save: usize) {
    covered!("save load fifa rules");
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
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 {
        write_u8(G_MAX_CONTRACT_LENGTH_YEARS, 5);
    }
    exe::zclose(&mut z);
}

// simplification: opens and reads the container (allocates the ZFILE) and only runs on load; follows the save block rule
// (file = original only). The copy takes the branch; the load step of the save proof exercises it.

/// strcat: appends fonte to the end of destino.
pub(crate) unsafe fn append_text(target: *mut u8, font: *const u8) {
    let d = target.add(core::ffi::CStr::from_ptr(target as *const core::ffi::c_char).to_bytes().len());
    copy_text(d, font);
}

/// strcpy: copies fonte with its terminator into destino.
pub(crate) unsafe fn copy_text(target: *mut u8, font: *const u8) {
    let n = core::ffi::CStr::from_ptr(font as *const core::ffi::c_char).to_bytes_with_nul().len();
    core::ptr::copy(font, target, n);
}

// Side by side over the whole summary (0xf74) and the match-time clock ([0xdd7140], [0xdd7144]),
// restored between the two; compares only +0..+0xf6c: the match time and time() (+0xf6c, +0xf70) are
// clock (fixed only in simulation mode), as in the save proof.

/// `free_game_data(screen)` (cdecl): releases the screens and destroys the subsystems held in globals, in the original
/// order; returns 1. The screen manager (`G_SCREEN_MANAGER`) releases the screens
/// (`exe::screenmanager_end_network_session`) and, if `screen` ≠ 0, also `exe::screenmanager_close_all_screens`;
/// `free_event_tables`; then each subsystem kept in a global, in the original's order: destructor (thiscall) + `delete`
/// (`exe::crt_operator_delete`) and the global zeroed; the loose vectors through `free` (`exe::crt_free`); two objects
/// through the virtual destructor (vtable[0](1)); the `*.tmp` files of the temporary folder
/// (`exe::utils_clear_temp_folder`), `G_NATION_NAME_INDEX_TABLE`, the database tables
/// (`exe::database_free_all_tables`), the game date (`exe::date_set`: 1, 0, 2001, −1) and three flags zeroed.
/// Original: `free_game_data` (0x82ff40, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn free_game_data(tela: u32) -> u32 {
    covered!("save free game data");
    // destrutor + delete + global zerado
    let objetos = |list: &[(usize, usize)]| {
        for &(g, destrutor) in list {
            let p = read_usize(g);
            if p != 0 {
                let d: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destrutor);
                d(p);
                exe::crt_operator_delete(p);
                write_usize(g, 0);
            }
        }
    };
    // free + global zerado
    let soltos = |list: &[usize]| {
        for &g in list {
            let p = read_usize(g);
            if p != 0 {
                exe::crt_free(p);
                write_usize(g, 0);
            }
        }
    };
    // virtual destructor (vtable[0], with 1 = delete) + zeroed global
    let virtual_ = |g: usize| {
        let p = read_usize(g);
        if p != 0 {
            let d: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(**(p as *const *const usize));
            d(p, 1);
            write_usize(g, 0);
        }
    };
    exe::screenmanager_end_network_session(G_SCREEN_MANAGER);
    if tela != 0 {
        exe::screenmanager_close_all_screens(G_SCREEN_MANAGER);
    }
    exe::match_events_free_event_tables();
    objetos(&[(G_MATCH_DAY, 0x6b4190)]);
    let p = read_usize(G_COMP_OBJECT_TABLE);
    if p != 0 {
        exe::crt_free(p);
        write_usize(G_COMP_OBJECT_TABLE, 0);
        write_usize(G_COMP_OBJECT_COUNT, 0);
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
    let p = read_usize(G_SQUAD_MANAGER);
    if p != 0 {
        exe::crt_operator_delete(p);
        write_usize(G_SQUAD_MANAGER, 0);
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
    write_u8(G_CURRENT_DAY_SLICE, 0);
    write_u32(G_GAME_DATA_LOADED_FLAG, 0);
    write_u32(G_GAME_HAS_SAVE_FILE_FLAG, 0);
    1
}

/// the heading text colour (two colours by the flag)
/// Original: `get_heading_colour` (0x839590, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_heading_colour(x: u32) -> u32 {
    let c = if x != 0 { read_u16(G_GUI_COLOUR_PALE_GREY) } else { read_u16(G_GUI_COLOUR_RED) };
    (x & 0xffff_0000) | c as u32
}

/// Rule: when a saved game is restored (`-load` and the network game), each human manager with no password (in a game
/// with no network name) takes the screen manager: the first one the current record, the others a record each; a
/// manager with a password who is not on holiday goes to computer control until he resumes it. Returns whether
/// Manager Status must open (a manager locked, or one with a password on holiday).
///
/// For k from 0 to 15, the person `G_STAFF_COUNT` − 1 − k of the person table (`G_STAFF_TABLE`, `staff::SIZE` bytes
/// each; the count read anew each time) with `staff::CLASSIFICATION` 1 (a human manager): with an empty password
/// (`HUMAN_MANAGER::get_password` 0x5fde40) and `G_NETWORK_GAME_NAMED_FLAG` 0, a current record (current channel) with no
/// key gets him as its key and the join and leave hooks (`NETWORK_JOIN_CALLBACK`, `NETWORK_LEAVE_CALLBACK`) are set;
/// with a key he gets a record of his own (`ScreenManager::add_manager_record` with the two hooks and him). Otherwise,
/// not on holiday (`HUMAN_MANAGER::is_on_holiday` 0x5fd210): `HUMAN_MANAGER::set_computer_controlled(1, him)`
/// (0x5fe140); on holiday: the result 1. Returns 1 when the HUMAN_MANAGER's locked count (`HUMAN_MANAGER_LOCKED_COUNT`)
/// is nonzero, otherwise the result.
/// With `CM_FERIAS` (the save proof) `simulation.rs` hooks this entry first: the engine's detour is refused (the bytes
/// are its jump) and the original runs inside that wrapper; without it the copy runs.
/// Original: `setup_assign_restored_network_managers` (0x8387e0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_assign_restored_network_managers() -> u32 {
    covered!("tel40_5 restored managers");
    let sm = G_SCREEN_MANAGER;
    let mut result = 0u32;
    for k in 0..0x10isize {
        let index = read_i32(G_STAFF_COUNT) as isize - k - 1;
        let person = Staff((read_usize(G_STAFF_TABLE) as isize + index * staff::SIZE as isize) as usize);
        if person.classification() != 1 {
            continue;
        }
        let password = exe::human_manager_get_password(read_usize(G_HUMAN_MANAGER), person);
        if read_u8(password) == 0 && read_u32(G_NETWORK_GAME_NAMED_FLAG) == 0 {
            let key = screen_record(sm) + screen_history::KEY;
            if read_u32(key) == 0 {
                covered!("tel40_5 the first manager takes the current record");
                write_usize(key, person.addr());
                write_usize(sm + SCRMAN_JOIN_HOOK, NETWORK_JOIN_CALLBACK);
                write_usize(sm + SCRMAN_LEAVE_HOOK, NETWORK_LEAVE_CALLBACK);
            } else {
                exe::screenmanager_add_manager_record(sm, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK, person); // tel40_5: a record of his own
            }
        } else if exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), person) == 0 {
            covered!("tel40_5 a manager with a password goes to the computer");
            let computer: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_COMPUTER_CONTROLLED);
            computer(read_usize(G_HUMAN_MANAGER), 1, person.addr());
        } else {
            result = 1;
        }
    }
    if read_u32(read_usize(G_HUMAN_MANAGER) + HUMAN_MANAGER_LOCKED_COUNT) != 0 {
        return 1;
    }
    result
}

/// Comparer for `setup_assign_restored_network_managers`: side by side over the screen manager's records
/// (`scrman_records_regions`), the HUMAN_MANAGER's header (records, count, locked and watched counts) and each human
/// record's computer control flag; during a match (the match day object set, which `set_computer_controlled` tells)
/// only the original.
pub(crate) unsafe extern "C" fn compare_setup_assign_restored_network_managers() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_SETUP_ASSIGN_RESTORED_NETWORK_MANAGERS);
    let hm = read_usize(G_HUMAN_MANAGER);
    if hm == 0 || read_usize(G_MATCH_DAY_OBJECT) != 0 {
        // simplification: the match day object is not told twice
        return o();
    }
    let mut reg = scrman_records_regions(G_SCREEN_MANAGER).to_vec();
    reg.push((hm, 0x10));
    let base = read_usize(hm);
    for i in 0..read_i32(hm + 4).clamp(0, 0x10) as usize {
        reg.push((base + i * HUMAN_MANAGER_RECORD_SIZE + human_manager_record::COMPUTER_CONTROLLED, 4));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || setup_assign_restored_network_managers());
    SCOREBOARD.record(same && r == n, || format!("setup_assign_restored_network_managers: returned {} / {}, differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Creates the World Cup and its qualifying sections on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the
/// Setup.cpp "Error" box 0x2ede and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go
/// from 1 July of the game year (weekday 5) to 1 July of the game year (weekday 5). 7 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): FIFA World Cup, World Cup African
/// Qualifying Section, World Cup Asian Qualifying Section, World Cup CONCACAF Qualifying Section, World Cup European
/// Qualifying Section, World Cup Oceania Qualifying Section, World Cup South American Qualifying Sect; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x2efb,
/// "create_world_cup_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(25, 6, 2001, -1) (25 July 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 1996,
/// `key_nation_record::SEASON_SPANS_YEAR` = 2; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_world_cup_qualifying_comps` (0x830620, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_world_cup_qualifying_comps(entry: usize) -> u32 {
    covered!("tel13_2 world cup qualifying");
    key_nation_create_comps(entry, &WORLD_CUP_QUALIFYING_COMPS)
}

/// Creates the other international tournaments on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the
/// Setup.cpp "Error" box 0x2f22 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go
/// from 1 July of the game year (weekday 5) to 1 July of the game year (weekday 5). 8 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Football Tournament Of The Olympic Games,
/// European Football Championship, European Championship Qualifying, African Cup Of Nations, Copa America, Oceania
/// Nations Cup, Asian Cup, Confederations Cup; every constructor gets the game year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x2f3e, "create_other_int_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(15, 7, 2001, -1) (15 August 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 2; `key_nation_record::LEAGUE_COMP`
/// = null; returns 1. /// Original: `create_other_int_comps` (0x830ac0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_other_int_comps(entry: usize) -> u32 {
    covered!("tel13_2 other int");
    key_nation_create_comps(entry, &OTHER_INT_COMPS)
}

/// Creates the world club tournaments on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x2f69 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July
/// of the game year (weekday 5) to 1 July of the game year (weekday 5). 3 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): FIFA Club World Championship, World Club Cup, Inter American Cup; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x2f88,
/// "create_world_club_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_world_club_comps` (0x830fe0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_world_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 world club");
    key_nation_create_comps(entry, &WORLD_CLUB_COMPS)
}

/// Creates the European club cups on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x2fac and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 23 June of the
/// game year (weekday -1) to 20 May of the game year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): UEFA European Champions League, UEFA Cup, Inter Toto Cup, European Super Cup
/// only when a nation of the entry's continent has a selected league (`key_nation_continent_has_selected_nation` of
/// `KEY_NATION_RECORD_CONTINENT`); otherwise the count and the array stay 0; every constructor gets the game year. A
/// failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x2fcd, "create_european_club_comps() : unable to
/// allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(21, 5, 2001, -1) (21 June
/// 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_european_club_comps` (0x8312e0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_european_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 european club");
    key_nation_create_comps(entry, &EUROPEAN_CLUB_COMPS)
}

/// Creates the CONCACAF club cups on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x2ff9 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July of the
/// game year (weekday 5) to 1 July of the game year (weekday 5). 2 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): CONCACAF Champions Cup, CONCACAF Merconorte Cup only when a nation of the
/// entry's continent has a selected league (`key_nation_continent_has_selected_nation` of
/// `KEY_NATION_RECORD_CONTINENT`); otherwise the count and the array stay 0; every constructor gets the start date
/// year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x301d, "create_north_american_club_comps()
/// : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(21, 1, 2001, -1)
/// (21 February 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_north_american_club_comps` (0x831650,
/// Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_north_american_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 north american club");
    key_nation_create_comps(entry, &NORTH_AMERICAN_CLUB_COMPS)
}

/// Creates the South American club cups on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x3045 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July
/// of the game year (weekday 5) to 1 July of the game year (weekday 5). 2 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS` (allocated as calloc(4, 2))): South American Copa Libertadores, South
/// American Copa Mercosur only when a nation of the entry's continent has a selected league
/// (`key_nation_continent_has_selected_nation` of `KEY_NATION_RECORD_CONTINENT`); otherwise the count and the array
/// stay 0; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp
/// 0x3065, "create_south_american_club_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY`
/// = the day of Date::Date(10, 0, 2001, -1) (10 January 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_south_american_club_comps` (0x8318f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_south_american_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 south american club");
    key_nation_create_comps(entry, &SOUTH_AMERICAN_CLUB_COMPS)
}

/// Creates the Asian club cups on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error" box
/// 0x308d and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July of the game
/// year (weekday 5) to 1 July of the game year (weekday 5). 3 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the array
/// in `KEY_NATION_RECORD_COMPS`): Asian Champions Cup, Asian Cup Winners Cup, Asian Super Cup only when a nation of the
/// entry's continent has a selected league (`key_nation_continent_has_selected_nation` of
/// `KEY_NATION_RECORD_CONTINENT`); otherwise the count and the array stay 0; every constructor gets the game year. A
/// failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x30ad, "create_asian_club_comps() : unable to
/// allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(28, 7, 2001, -1) (28 August
/// 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_asian_club_comps` (0x831ba0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_asian_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 asian club");
    key_nation_create_comps(entry, &ASIAN_CLUB_COMPS)
}

/// Creates the Oceania club championship on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x30d6 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July
/// of the game year (weekday 5) to 1 July of the game year (weekday 5). 1 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Oceania Club Championship only when a nation of the entry's continent has a
/// selected league (`key_nation_continent_has_selected_nation` of `KEY_NATION_RECORD_CONTINENT`); otherwise the count
/// and the array stay 0; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x30f6, "create_asian_club_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(21, 6, 2001, -1) (21 July 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP`
/// = null; returns 1. The original's allocation message names another routine (create_asian_club_comps()). ///
/// Original: `create_oceania_club_comps` (0x831eb0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_oceania_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 oceania club");
    key_nation_create_comps(entry, &OCEANIA_CLUB_COMPS)
}

/// Creates Argentina's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x311f and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 23
/// August of the game year (weekday 5) to 19 July of the next year (weekday 6). 2 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Argentine Premier Division, Argentine
/// Second Division; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x313c, "create_argentine_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(1, 7, 2001, -1) (1 August 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_argentine_comps` (0x832100, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_argentine_comps(entry: usize) -> u32 {
    covered!("tel13_2 argentine");
    key_nation_create_comps(entry, &ARGENTINE_COMPS)
}

/// Creates Australia's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x3160 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 2
/// October of the game year (weekday 5) to 10 June of the next year (weekday 5). 1 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Australian National Soccer League; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x317d,
/// "create_australian_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 6, 2001, -1) (20 July 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_australian_comps` (0x832380, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_australian_comps(entry: usize) -> u32 {
    covered!("tel13_2 australian");
    key_nation_create_comps(entry, &AUSTRALIAN_COMPS)
}

/// Creates Belgium's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x31a1 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 3 August of
/// the game year (weekday 5) to 1 June of the next year (weekday 6). 5 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Belgian First Division, Belgian Second Division, Belgian Third Division,
/// Belgian Cup, Belgian Super Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x31be, "create_belgian_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the Belgian Super Cup record; returns 1. /// Original: `create_belgian_comps`
/// (0x8325d0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_belgian_comps(entry: usize) -> u32 {
    covered!("tel13_2 belgian");
    key_nation_create_comps(entry, &BELGIAN_COMPS)
}

/// Creates Brazil's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x31e6 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 18 January of
/// the year 4 weeks after the start date (weekday 6) to 14 December of the year 4 weeks after the start date (weekday
/// 6) (Brazil reads its year from the start date moved 4 weeks on, `Date::add_weeks`; its constructors get the dword
/// from that year on: the year and the next word). 17 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the array in
/// `KEY_NATION_RECORD_COMPS`): Brazilian National First Division, Brazilian National Second Division, Brazilian
/// National Third Division, Brazilian Bahia State Championship, Brazilian Central State Championship, Brazilian Gaucho
/// State Championship, Brazilian Goias State Championship, Brazilian Minas Gerais State Championshi, Brazilian North
/// State Championship, Brazilian Northeast State Championship, Brazilian Parana State Championship, Brazilian
/// Pernambuco State Championship, Brazilian Rio De Janeiro State Champions, Brazilian Santa Catarina State Champions,
/// Brazilian Sao Paulo State Champ, Brazilian Cup, Brazilian Champions Cup; every constructor gets the year 4 weeks
/// after the start date. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3205,
/// "create_norwegian_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(12, 0, 2001, -1) (12 January 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the year 4 weeks after the start
/// date, `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. The original's
/// allocation message names another routine (create_norwegian_comps()). /// Original: `create_brazilian_comps`
/// (0x832980, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_brazilian_comps(entry: usize) -> u32 {
    covered!("tel13_2 brazilian");
    key_nation_create_comps(entry, &BRAZILIAN_COMPS)
}

/// Comparer of `create_world_cup_qualifying_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_world_cup_qualifying_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_WORLD_CUP_QUALIFYING_COMPS)(entry);
    key_nation_check_comps("create_world_cup_qualifying_comps", entry, r, &WORLD_CUP_QUALIFYING_COMPS);
    r
}

/// Comparer of `create_other_int_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_other_int_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_OTHER_INT_COMPS)(entry);
    key_nation_check_comps("create_other_int_comps", entry, r, &OTHER_INT_COMPS);
    r
}

/// Comparer of `create_world_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_world_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_WORLD_CLUB_COMPS)(entry);
    key_nation_check_comps("create_world_club_comps", entry, r, &WORLD_CLUB_COMPS);
    r
}

/// Comparer of `create_european_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_european_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_EUROPEAN_CLUB_COMPS)(entry);
    key_nation_check_comps("create_european_club_comps", entry, r, &EUROPEAN_CLUB_COMPS);
    r
}

/// Comparer of `create_north_american_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_north_american_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_NORTH_AMERICAN_CLUB_COMPS)(entry);
    key_nation_check_comps("create_north_american_club_comps", entry, r, &NORTH_AMERICAN_CLUB_COMPS);
    r
}

/// Comparer of `create_south_american_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_south_american_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_SOUTH_AMERICAN_CLUB_COMPS)(entry);
    key_nation_check_comps("create_south_american_club_comps", entry, r, &SOUTH_AMERICAN_CLUB_COMPS);
    r
}

/// Comparer of `create_asian_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_asian_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_ASIAN_CLUB_COMPS)(entry);
    key_nation_check_comps("create_asian_club_comps", entry, r, &ASIAN_CLUB_COMPS);
    r
}

/// Comparer of `create_oceania_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_oceania_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_OCEANIA_CLUB_COMPS)(entry);
    key_nation_check_comps("create_oceania_club_comps", entry, r, &OCEANIA_CLUB_COMPS);
    r
}

/// Comparer of `create_argentine_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_argentine_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_ARGENTINE_COMPS)(entry);
    key_nation_check_comps("create_argentine_comps", entry, r, &ARGENTINE_COMPS);
    r
}

/// Comparer of `create_australian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_australian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_AUSTRALIAN_COMPS)(entry);
    key_nation_check_comps("create_australian_comps", entry, r, &AUSTRALIAN_COMPS);
    r
}

/// Comparer of `create_belgian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_belgian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_BELGIAN_COMPS)(entry);
    key_nation_check_comps("create_belgian_comps", entry, r, &BELGIAN_COMPS);
    r
}

/// Comparer of `create_brazilian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_brazilian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_BRAZILIAN_COMPS)(entry);
    key_nation_check_comps("create_brazilian_comps", entry, r, &BRAZILIAN_COMPS);
    r
}

/// Creates Croatia's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3239 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 24 July of the
/// game year (weekday 5) to 14 May of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): Croatian First Division, Croatian Second Division North, Croatian Second
/// Division South, Croatian Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x3256, "create_croatian_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_croatian_comps` (0x8330b0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_croatian_comps(entry: usize) -> u32 {
    covered!("tel13_3 croatian");
    key_nation_create_comps(entry, &CROATIAN_COMPS)
}

/// Creates Denmark's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x327e and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 27 July of the
/// game year (weekday 6) to 14 June of the next year (weekday 6). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): Danish Premier Division, Danish First Division, Danish Second Division, Danish
/// Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x329a,
/// "create_danish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_danish_comps` (0x833400, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_danish_comps(entry: usize) -> u32 {
    covered!("tel13_3 danish");
    key_nation_create_comps(entry, &DANISH_COMPS)
}

/// Creates England's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x32c1 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 10 August of
/// the game year (weekday 5) to 24 May of the next year (weekday 5). 8 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): English Premier Division, English First Division, English Second Division,
/// English Third Division, English FA Cup, English League Cup, English Autoglass Trophy, English Charity Shield; with
/// the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 10, adding English Conference, English FA Trophy
/// (created when the count is 10); every constructor gets the game year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x32e0, "create_english_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the English Charity Shield record; returns 1. /// Original:
/// `create_english_comps` (0x833750, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_english_comps(entry: usize) -> u32 {
    covered!("tel13_3 english");
    key_nation_create_comps(entry, &ENGLISH_COMPS)
}

/// Creates Finland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3311 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 29 April of
/// the start date year (weekday 5) to 12 November of the next year (weekday 6). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Finnish Premier Division, Finnish First
/// Division, Finnish Cup; every constructor gets the start date year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x332d, "create_finnish_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(27, 11, 2001, -1) (27 December 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 0;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_finnish_comps` (0x833cf0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_finnish_comps(entry: usize) -> u32 {
    covered!("tel13_3 finnish");
    key_nation_create_comps(entry, &FINNISH_COMPS)
}

/// Creates France's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3352 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 9 August of
/// the game year (weekday 5) to 24 May of the next year (weekday 5). 6 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): French First Division, French Second Division, French National, French Cup,
/// French League Cup, French Champions Trophy; every constructor gets the game year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x336e, "create_french_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the French Champions Trophy record; returns 1. /// Original:
/// `create_french_comps` (0x833ff0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_french_comps(entry: usize) -> u32 {
    covered!("tel13_3 french");
    key_nation_create_comps(entry, &FRENCH_COMPS)
}

/// Creates Germany's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3397 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 10 August of
/// the game year (weekday 5) to 14 June of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): German First Division, German Second Division, German Cup, German League
/// Cup; with the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 5, adding German Regional (created when the
/// count is 5); every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp
/// 0x33b6, "create_german_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_german_comps` (0x8343f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_german_comps(entry: usize) -> u32 {
    covered!("tel13_3 german");
    key_nation_create_comps(entry, &GERMAN_COMPS)
}

/// Creates Greece's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x33e1 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 15 August of
/// the game year (weekday 5) to 21 May of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Greek Premier Division, Greek Second Division, Greek Cup, Greek Super Cup;
/// every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x33fd,
/// "create_german_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. The original's
/// allocation message names another routine (create_german_comps()). /// Original: `create_greek_comps` (0x8347c0,
/// Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_greek_comps(entry: usize) -> u32 {
    covered!("tel13_3 greek");
    key_nation_create_comps(entry, &GREEK_COMPS)
}

/// Creates Holland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3423 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 16 August of
/// the game year (weekday 5) to 7 June of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Dutch KPN Telecompetition, Dutch Toto Division, Dutch Amstel Cup, Dutch
/// Super Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp
/// 0x343f, "create_dutch_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_dutch_comps` (0x834b10, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_dutch_comps(entry: usize) -> u32 {
    covered!("tel13_3 dutch");
    key_nation_create_comps(entry, &DUTCH_COMPS)
}

/// Creates the Republic of Ireland on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3466 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 August of
/// the game year (weekday 6) to 23 April of the next year (weekday 6). 8 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Irish Premier Division, Irish First Division, Irish Senior Challenge Cup,
/// Irish League Cup, Irish Super Cup, Irish Presidents Cup, Irish Munster Senior Cup, Irish Leinster Senior Cup; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3482,
/// "create_irish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = the Irish Super Cup record;
/// `KEY_NATION_RECORD_CUP_COMP` = the Irish Senior Challenge Cup record; returns 1. /// Original: `create_irish_comps`
/// (0x834e60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_irish_comps(entry: usize) -> u32 {
    covered!("tel13_3 irish");
    key_nation_create_comps(entry, &IRISH_COMPS)
}

/// Creates Italy's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x34af and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 6 September of
/// the game year (weekday 6) to 8 June of the next year (weekday 6). 8 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Italian Serie A, Italian Serie B, Italian Serie C1 A, Italian Serie C1 B,
/// Italian Cup, Italian Serie C Cup, Italian Super Cup, Italian C1 Super Cup; with the nation's extra division
/// (`nation::LEAGUE_SELECTED` bit 2) 11, adding Italian Serie C2 A, Italian Serie C2 B, Italian Serie C2 C (created
/// when the nation has it); every constructor gets the game year. A failed allocation: the "Memory Allocation Error"
/// box (Setup.cpp 0x34ce, "create_italian_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the Italian Super Cup record; returns 1. /// Original: `create_italian_comps`
/// (0x835320, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_italian_comps(entry: usize) -> u32 {
    covered!("tel13_3 italian");
    key_nation_create_comps(entry, &ITALIAN_COMPS)
}

/// Creates Japan's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3502 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 14 March of
/// the start date year (weekday 5) to 28 November of the start date year (weekday 5). 5 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Japanese J League 1, Japanese J League 2,
/// Japanese Emperor's Cup, Japanese Yamazaki Nabisco Cup, Japanese Xerox Super Cup; every constructor gets the start
/// date year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x351e, "create_japanese_comps() :
/// unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(25, 0, 2001, -1) (25
/// January 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_japanese_comps` (0x835900, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_japanese_comps(entry: usize) -> u32 {
    covered!("tel13_3 japanese");
    key_nation_create_comps(entry, &JAPANESE_COMPS)
}

/// Creates Northern Ireland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x3546 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 12
/// August of the game year (weekday 5) to 5 May of the next year (weekday 5). 5 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Northern Ireland Premier Division,
/// Northern Ireland First Division, Northern Irish Cup, Northern Irish League Cup, Northern Irish Charity Shield; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3562,
/// "create_northern_irish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = the Northern Irish Charity Shield
/// record; `KEY_NATION_RECORD_CUP_COMP` = the Northern Irish Cup record; returns 1. /// Original:
/// `create_northern_irish_comps` (0x835c90, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_northern_irish_comps(entry: usize) -> u32 {
    covered!("tel13_3 northern irish");
    key_nation_create_comps(entry, &NORTHERN_IRISH_COMPS)
}

/// Comparer of `create_croatian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_croatian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_CROATIAN_COMPS)(entry);
    key_nation_check_comps("create_croatian_comps", entry, r, &CROATIAN_COMPS);
    r
}

/// Comparer of `create_danish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_danish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_DANISH_COMPS)(entry);
    key_nation_check_comps("create_danish_comps", entry, r, &DANISH_COMPS);
    r
}

/// Comparer of `create_english_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_english_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_ENGLISH_COMPS)(entry);
    key_nation_check_comps("create_english_comps", entry, r, &ENGLISH_COMPS);
    r
}

/// Comparer of `create_finnish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_finnish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_FINNISH_COMPS)(entry);
    key_nation_check_comps("create_finnish_comps", entry, r, &FINNISH_COMPS);
    r
}

/// Comparer of `create_french_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_french_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_FRENCH_COMPS)(entry);
    key_nation_check_comps("create_french_comps", entry, r, &FRENCH_COMPS);
    r
}

/// Comparer of `create_german_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_german_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_GERMAN_COMPS)(entry);
    key_nation_check_comps("create_german_comps", entry, r, &GERMAN_COMPS);
    r
}

/// Comparer of `create_greek_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_greek_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_GREEK_COMPS)(entry);
    key_nation_check_comps("create_greek_comps", entry, r, &GREEK_COMPS);
    r
}

/// Comparer of `create_dutch_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_dutch_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_DUTCH_COMPS)(entry);
    key_nation_check_comps("create_dutch_comps", entry, r, &DUTCH_COMPS);
    r
}

/// Comparer of `create_irish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_irish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_IRISH_COMPS)(entry);
    key_nation_check_comps("create_irish_comps", entry, r, &IRISH_COMPS);
    r
}

/// Comparer of `create_italian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_italian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_ITALIAN_COMPS)(entry);
    key_nation_check_comps("create_italian_comps", entry, r, &ITALIAN_COMPS);
    r
}

/// Comparer of `create_japanese_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_japanese_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_JAPANESE_COMPS)(entry);
    key_nation_check_comps("create_japanese_comps", entry, r, &JAPANESE_COMPS);
    r
}

/// Comparer of `create_northern_irish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_northern_irish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_NORTHERN_IRISH_COMPS)(entry);
    key_nation_check_comps("create_northern_irish_comps", entry, r, &NORTHERN_IRISH_COMPS);
    r
}

/// Creates Norway's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x358c and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 12 April of
/// the start date year (weekday 6) to 31 October of the next year (weekday 6). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Norwegian Premier Division, Norwegian
/// First Division, Norwegian Cup; every constructor gets the start date year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x35a8, "create_norwegian_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(27, 10, 2001, -1) (27 November 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_norwegian_comps` (0x836050, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_norwegian_comps(entry: usize) -> u32 {
    covered!("tel13_4 norwegian");
    key_nation_create_comps(entry, &NORWEGIAN_COMPS)
}

/// Creates Poland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x35cf and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 17 July of the
/// game year (weekday 5) to 27 May of the next year (weekday 5). 5 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): Polish First Division, Polish Second Division, Polish FA Cup, Polish League
/// Cup, Polish Super Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x35eb, "create_polish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` =
/// the day of Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` is not written; returns 1. ///
/// Original: `create_polish_comps` (0x836350, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_polish_comps(entry: usize) -> u32 {
    covered!("tel13_4 polish");
    key_nation_create_comps(entry, &POLISH_COMPS)
}

/// Creates Portugal's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3612 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 25 August of
/// the game year (weekday 6) to 8 June of the next year (weekday 6). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Portuguese Premier Division, Portuguese Second Division, Portuguese Cup,
/// Portuguese Super Cup; with the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 5, adding Portuguese Second
/// Division B (created when the nation has it); every constructor gets the game year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x3631, "create_portuguese_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the Portuguese Super Cup record; returns 1. /// Original:
/// `create_portuguese_comps` (0x8366e0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_portuguese_comps(entry: usize) -> u32 {
    covered!("tel13_4 portuguese");
    key_nation_create_comps(entry, &PORTUGUESE_COMPS)
}

/// Creates Russia's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x365a and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 25 March of
/// the start date year (weekday 5) to 12 November of the start date year (weekday 6). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Russian Premier Division, Russian First
/// Division, Russian Cup; the constructors get the game year (the start date year: Russian Premier Division, Russian
/// First Division). A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3679, "create_russian_comps() :
/// unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(27, 11, 2001, -1) (27
/// December 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_russian_comps` (0x836ac0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_russian_comps(entry: usize) -> u32 {
    covered!("tel13_4 russian");
    key_nation_create_comps(entry, &RUSSIAN_COMPS)
}

/// Creates Scotland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x36a1 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 2 August of
/// the game year (weekday 5) to 24 May of the next year (weekday 5). 7 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Scottish Premier Division, Scottish First Division, Scottish Second
/// Division, Scottish Third Division, Scottish Cup, Scottish League Cup, Scottish League Challenge Cup; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x36bd,
/// "create_scottish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_scottish_comps` (0x836dc0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_scottish_comps(entry: usize) -> u32 {
    covered!("tel13_4 scottish");
    key_nation_create_comps(entry, &SCOTTISH_COMPS)
}

/// Creates South Korea's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x36e9 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 25
/// March of the start date year (weekday 6) to 25 November of the start date year (weekday 5). 4 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Korean League, Korean FA Cup, Korean
/// League Cup, Korean Super Cup; every constructor gets the start date year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x3705, "create_spanish_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 0, 2001, -1) (20 January 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the Korean Super Cup record; returns 1. The original's allocation message names
/// another routine (create_spanish_comps()). /// Original: `create_south_korean_comps` (0x837200, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_south_korean_comps(entry: usize) -> u32 {
    covered!("tel13_4 south korean");
    key_nation_create_comps(entry, &SOUTH_KOREAN_COMPS)
}

/// Creates Spain's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x372e and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 September of
/// the game year (weekday 6) to 14 June of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Spanish First Division, Spanish Second Division, Spanish Cup, Spanish Super
/// Cup; with the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 5, adding Spanish Second Division B (created
/// when the count is 5); every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x374d, "create_spanish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY`
/// = the day of Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = the Spanish Super Cup record; returns
/// 1. /// Original: `create_spanish_comps` (0x837560, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_spanish_comps(entry: usize) -> u32 {
    covered!("tel13_4 spanish");
    key_nation_create_comps(entry, &SPANISH_COMPS)
}

/// Creates Sweden's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x377d and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 6 April of the
/// start date year (weekday 6) to 26 October of the start date year (weekday 6). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Swedish Premier Division, Swedish First
/// Division, Swedish Cup; with the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 4, adding Swedish Second
/// Division (created when the count is 4); the constructors get the game year (the start date year: Swedish Premier
/// Division, Swedish First Division, Swedish Second Division). A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x379f, "create_swedish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY`
/// = the day of Date::Date(27, 11, 2001, -1) (27 December 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the start date year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_swedish_comps` (0x8379f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_swedish_comps(entry: usize) -> u32 {
    covered!("tel13_4 swedish");
    key_nation_create_comps(entry, &SWEDISH_COMPS)
}

/// Creates Turkey's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x37c9 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 8 August of
/// the game year (weekday 6) to 21 May of the next year (weekday 6). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Turkish Premier Division, Turkish 2 Division Category A, Turkish 2 Division
/// Category B, Turkish FA Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation Error"
/// box (Setup.cpp 0x37e5, "create_spanish_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. The original's allocation message names another routine
/// (create_spanish_comps()). /// Original: `create_turkish_comps` (0x837d70, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_turkish_comps(entry: usize) -> u32 {
    covered!("tel13_4 turkish");
    key_nation_create_comps(entry, &TURKISH_COMPS)
}

/// Creates the United States on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error" box
/// 0x380b and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 22 March of the
/// start date year (weekday 5) to 6 January of the year after the start date (weekday 2). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): American Major League, US Open Cup,
/// American MLS All Stars; every constructor gets the start date year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x3827, "create_united_states_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 1, 2001, -1) (20 February 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_united_states_comps` (0x8380c0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_united_states_comps(entry: usize) -> u32 {
    covered!("tel13_4 united states");
    key_nation_create_comps(entry, &UNITED_STATES_COMPS)
}

/// Creates Wales's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x384c and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 19 August of
/// the game year (weekday 5) to 6 May of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): Welsh Premier Division, Welsh Cup, Welsh League Cup, Welsh Premier Cup; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3868,
/// "create_spanish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` is not written; returns 1. The
/// original's allocation message names another routine (create_spanish_comps()). /// Original: `create_welsh_comps`
/// (0x8383b0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_welsh_comps(entry: usize) -> u32 {
    covered!("tel13_4 welsh");
    key_nation_create_comps(entry, &WELSH_COMPS)
}

/// Comparer of `create_norwegian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_norwegian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_NORWEGIAN_COMPS)(entry);
    key_nation_check_comps("create_norwegian_comps", entry, r, &NORWEGIAN_COMPS);
    r
}

/// Comparer of `create_polish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_polish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_POLISH_COMPS)(entry);
    key_nation_check_comps("create_polish_comps", entry, r, &POLISH_COMPS);
    r
}

/// Comparer of `create_portuguese_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_portuguese_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_PORTUGUESE_COMPS)(entry);
    key_nation_check_comps("create_portuguese_comps", entry, r, &PORTUGUESE_COMPS);
    r
}

/// Comparer of `create_russian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_russian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_RUSSIAN_COMPS)(entry);
    key_nation_check_comps("create_russian_comps", entry, r, &RUSSIAN_COMPS);
    r
}

/// Comparer of `create_scottish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_scottish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_SCOTTISH_COMPS)(entry);
    key_nation_check_comps("create_scottish_comps", entry, r, &SCOTTISH_COMPS);
    r
}

/// Comparer of `create_south_korean_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_south_korean_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_SOUTH_KOREAN_COMPS)(entry);
    key_nation_check_comps("create_south_korean_comps", entry, r, &SOUTH_KOREAN_COMPS);
    r
}

/// Comparer of `create_spanish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_spanish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_SPANISH_COMPS)(entry);
    key_nation_check_comps("create_spanish_comps", entry, r, &SPANISH_COMPS);
    r
}

/// Comparer of `create_swedish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_swedish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_SWEDISH_COMPS)(entry);
    key_nation_check_comps("create_swedish_comps", entry, r, &SWEDISH_COMPS);
    r
}

/// Comparer of `create_turkish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_turkish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_TURKISH_COMPS)(entry);
    key_nation_check_comps("create_turkish_comps", entry, r, &TURKISH_COMPS);
    r
}

/// Comparer of `create_united_states_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_united_states_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_UNITED_STATES_COMPS)(entry);
    key_nation_check_comps("create_united_states_comps", entry, r, &UNITED_STATES_COMPS);
    r
}

/// Comparer of `create_welsh_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_welsh_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CREATE_WELSH_COMPS)(entry);
    key_nation_check_comps("create_welsh_comps", entry, r, &WELSH_COMPS);
    r
}

/// The game's front end: checks the quick start folder, starts the date and the random generator, runs the Setup
/// Game screen until a game exists, then makes Manager Status the default screen.
///
/// The folder of key `CM3_QSTART` (`exe::utils_get_folder_for_key`) must be a folder (`utils_is_directory`) or be
/// created (`_mkdir`); otherwise the Setup.cpp error box (line 0x209), the fatal cleanup and exit(−1). The game date
/// `G_CURRENT_DATE` becomes 1 January 2001 (`exe::date_set`: 1, 0, 2001, −1), `G_CURRENT_DAY_SLICE` 0,
/// `G_GAME_DATA_VERSION` 0xfffc1fb9 and `G_GAME_RANDOM_SEED` the clock (`exe::utils_get_time_ms`); a `-seed N` on the
/// command line (`G_COMMAND_LINE_TEXT`, sscanf "%d" right after "-seed") seeds the generator with N
/// (`utils_seed_random`) and becomes the seed, otherwise the generator is seeded with 0 (the clock seed stays).
/// `G_NETWORK_GAME_NAME` is emptied and `-nonetwork` clears `G_NETWORK_PLAY_ENABLED`. Loads `player_setup.cfg`
/// (`player_setup_cfg_load` with masks 0x80 and 0x400, no error reports). Then the loop: the network flags
/// (`G_NEW_GAME_SETUP_DONE_FLAG`, `G_NETWORK_SETUP_HOST_FLAG`, `G_NETWORK_GAME_NAMED_FLAG`) zeroed, the Setup Game
/// screen opened (`exe::screenmanager_open_screen_for_each_record`, key 0, `build_setup_game_screen` and
/// `handle_setup_game_input`) with its param 0 = 1 while the game settings were never confirmed (Config +0x213 = 0),
/// and the screen manager runs (`exe::screenmanager_monitor`); if a game came out half made (`G_GAME_IN_PROGRESS_FLAG`
/// or `G_GAME_SESSION_STARTED_FLAG`), it is freed (`exe::free_game_data`(1)), both flags zeroed, and the loop starts
/// again. Finally `G_INTERACTIVE_MODE_FLAG` = 1 and the default screen becomes Manager Status
/// (`ScreenManager::set_default_screen`: `setup_build_manager_status_screen`, `setup_handle_manager_status_input`, with
/// the 5 values of `G_DEFAULT_SCREEN_SLOT_VALUES`). Returns 1.
/// Original: `setup_run_front_end` (0x81b890, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_run_front_end() -> i32 {
    covered!("tel11_1 front end");
    let mkdir: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_MKDIR);
    let sscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let seed_random: unsafe extern "C" fn(i32) = core::mem::transmute(FN_UTILS_SEED_RANDOM);
    let player_setup: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
    let set_default: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_DEFAULT_SCREEN);
    if exe::utils_is_directory(exe::utils_get_folder_for_key(0xa80204)) == 0 && mkdir(exe::utils_get_folder_for_key(0xa80204)) != 0 {
        setup_fatal_exit(Some(0x209), -1);
    }
    exe::date_set(G_CURRENT_DATE, 1, 0, 0x7d1, -1);
    write_u8(G_CURRENT_DAY_SLICE, 0);
    write_u32(G_GAME_DATA_VERSION, 0xfffc1fb9);
    write_u32(G_GAME_RANDOM_SEED, exe::utils_get_time_ms());
    let p = exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa801d4); // "-seed"
    let mut seed = 0i32;
    if p != 0 && sscanf(p + 5, 0x98806c, &mut seed as *mut i32) == 1 {
        seed_random(seed);
        write_i32(G_GAME_RANDOM_SEED, seed);
    } else {
        seed_random(0);
    }
    write_u8(G_NETWORK_GAME_NAME, 0);
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa801c8) != 0 {
        // "-nonetwork"
        write_u32(G_NETWORK_PLAY_ENABLED, 0);
    }
    player_setup(0x80, 0);
    player_setup(0x400, 0);
    loop {
        write_u32(G_NEW_GAME_SETUP_DONE_FLAG, 0);
        write_u32(G_NETWORK_SETUP_HOST_FLAG, 0);
        write_u32(G_NETWORK_GAME_NAMED_FLAG, 0);
        if exe::screenmanager_open_screen_for_each_record(SCREEN_MANAGER, 0, 0x81bab0usize, 0x81bd70usize, 0, 0usize, 0) != 0 {
            let first_time = (read_u32(read_usize(G_CONFIG) + CONFIG_SETTINGS_SEEN) == 0) as usize;
            exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, first_time, 0);
        }
        exe::screenmanager_monitor(SCREEN_MANAGER);
        if read_u32(G_GAME_IN_PROGRESS_FLAG) == 0 && read_u32(G_GAME_SESSION_STARTED_FLAG) == 0 {
            break;
        }
        exe::free_game_data(1);
        write_u32(G_GAME_IN_PROGRESS_FLAG, 0);
        write_u32(G_GAME_SESSION_STARTED_FLAG, 0);
    }
    write_u32(G_INTERACTIVE_MODE_FLAG, 1);
    set_default(SCREEN_MANAGER, 0x81ff10, 0x820950, 0, 0, 0, G_DEFAULT_SCREEN_SLOT_VALUES, 5);
    1
}

/// Builds the Setup Game screen: the title, the menu of nine buttons and the back button.
///
/// `G_NETWORK_SETUP_HOST_FLAG` = 0; a random background (`exe::background_set_random_picture`); the title bar
/// (`setup_screen_header`, subtitle "Setup Game"). The 25 menu items (0xd4 bytes, built and destroyed by the CRT vector
/// iterators with 0x4568f0/0x5a53c0, as in the original) get their translated texts and ids: Start New Game 1, Quick
/// Start Game 2, Restore Saved Game 3, Delete Saved Game 4, Network Play 5 (id 0 and the grey colour word in the item's
/// +2 when `G_NETWORK_PLAY_ENABLED` = 0), Game Settings 6, Hall Of Fame 7, Game Credits 8, Web Sites 9; then the menu
/// (`gui_add_menu_buttons`: 0x6e, 0x91 to 0x30c, 0x217, 2 columns, 6 rows, font 3, 9 items, no bold, unsorted), the
/// back/next buttons (back only while the screen manager can go back, next never) and, when the screen's param 0 is set
/// (the game settings box is about to open), an empty area over the whole screen (0, 0 to 0x31f, 0x257).
/// Original: `setup_build_setup_game_screen` (0x81bab0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_setup_game_screen() {
    covered!("tel11_1 build setup game");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let menu: unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, usize, u32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_MENU_BUTTONS);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let mut items: MatchTabs = [0; 0x14b4 / 4];
    construct_match_tabs(&mut items);
    write_u32(G_NETWORK_SETUP_HOST_FLAG, 0);
    exe::background_set_random_picture();
    setup_screen_header(0xa8027c); // "Setup Game"
    // the nine buttons: text and id (Network Play is checked below)
    let entries: [(usize, u32); 9] = [(0xa8026c, 1), (0xa80258, 2), (0xa80244, 3), (0xa80230, 4), (0xa80220, 5),
        (0xa5a46c, 6), (0xa80210, 7), (0x9b52fc, 8), (0x9b53b4, 9)];
    for (k, &(label, id)) in entries.iter().enumerate() {
        text(match_tab_title(&mut items, k), label);
        if k == 4 && read_u32(G_NETWORK_PLAY_ENABLED) == 0 {
            items[4 * 53 + 51] = 0;
            write_u16(items.as_mut_ptr() as usize + 4 * 0xd4 + 2, read_u16(G_GUI_COLOUR_GREY));
        } else {
            items[k * 53 + 51] = id;
        }
    }
    menu(0x6e, 0x91, 0x30c, 0x217, 2, 6, 3, 9, items.as_mut_ptr() as usize, 0, 0);
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, 0);
    if exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0) != 0 {
        // the right and bottom edges are the pushes at 0x81bd16 and 0x81bd11 in the running exe (0x31f and 0x257;
        // 0x77f and 0x437 in the 1920x1080 exe)
        let (x2, y2) = (read_i32(0x81bd17), read_i32(0x81bd12));
        area(current_human_display(), 0, 0, x2 as usize, y2 as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
    }
    destruct_match_tabs(&mut items);
}

/// Starts a new game: loads the database if needed and opens Select League(s).
///
/// Without a database (`G_NATION_TABLE` = 0): below 50 MB free (`exe::os_get_free_memory_mb`) the no memory box opens
/// (`G_GAME_SESSION_STARTED_FLAG` = 1, with `G_INTERACTIVE_MODE_FLAG`, param 0 = 0) and that is all; otherwise the
/// database is loaded from the CM3_DATA folder (`exe::load_database_from_files`(folder, 1); a failure: box 0x688, fatal
/// cleanup, exit(−1)), the Config applied (`exe::config_apply_and_save`), the default league picked by language
/// (`select_default_league_by_language`) and the key nations built (`exe::key_nation_build_table`; a failure: box 0x693,
/// exit(−1)). Then Select League(s) opens (`setup_build_select_leagues_screen`, `setup_handle_select_leagues_input`)
/// with params 0 and 1 zeroed and `G_QUICK_SIM_COMP_FLAGS_PENDING` = 1.
/// Original: `setup_start_new_game` (0x81ce00, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_start_new_game() {
    covered!("tel11_1 start new game");
    let default_league: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_SELECT_DEFAULT_LEAGUE_BY_LANGUAGE);
    if read_usize(G_NATION_TABLE) == 0 {
        if exe::os_get_free_memory_mb() < 0x32 {
            write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
            if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x826d20usize, 0x826e60usize, read_u32(G_INTERACTIVE_MODE_FLAG), 0usize, 0) != 0 {
                exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, 0usize, 0);
            }
            return;
        }
        let mut folder = [0u8; 0x104];
        copy_text(folder.as_mut_ptr(), exe::utils_build_file_path(0x9b5348, 0xad9c64) as *const u8); // "CM3_DATA"
        if exe::load_database_from_files(folder.as_ptr(), 1) == 0 {
            setup_fatal_exit(Some(0x688), -1);
        }
        exe::config_apply_and_save(read_usize(G_CONFIG));
        default_league();
        if exe::key_nation_build_table() == 0 {
            setup_fatal_exit(Some(0x693), -1);
        }
    }
    if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x81d010usize, 0x81dc30usize, 0, 0usize, 0) != 0 {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, 0usize, 0);
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 1, 0usize, 0);
        write_u32(G_QUICK_SIM_COMP_FLAGS_PENDING, 1);
    }
}

/// Builds the Select League(s) screen: the Use Real Players and Attribute Masking options, Select All and De-Select All,
/// and the list of the key nations with their SELECTED, BACKGROUND and extra division buttons.
///
/// The memory limit (`setup_league_memory_limit`); `G_NEW_GAME_SETUP_DONE_FLAG` = 0; a random background; the title
/// bar with "Selected League(s)" when the game is already running (`G_INTERACTIVE_MODE_FLAG`, a read-only view) or
/// "Select League(s)". Without the German player setup (`G_PLAYER_SETUP_GERMAN_FLAG`) the options area (0x6e, 0x91 to
/// 0x20d, 0xa5, nine columns 0x2a, 0xd, 1, 0xd, 0xa, 0x2a, 0xd, 1, 0xd) shows "  Use Real Players:" and Yes/No (columns 1
/// and 3; the chosen one orange with style 0x801, the other grey; ids 0x39/0x3a, or no id in the read-only view); then,
/// when that area is index 0 or with the German setup, a second area (columns 0, 0, 0, 0, 0, 0x2a, 0xd, 1, 0xd) takes the
/// "  Attribute Masking:" row (column 5; Yes in column 6 with id 0x3b; No with id 0x3c in column 8, but in column 7, the
/// one-pixel spacer, in the read-only view: a defect of the original). The buttons area (0x212, 0x91 to 0x30c, 0xa5, two
/// columns): Select All (id 0xa) clickable only with fewer than 15 leagues selected (`G_SELECTED_NATION_COUNT` before it
/// is recounted below) and a memory limit of 16 or more; De-Select All (id 0xb) only with leagues to load
/// (`G_NATIONS_TO_LOAD_COUNT` > 0); plain texts in the read-only view.
///
/// The list area (0x6e, 0xaa to 0x30c, 0x217; columns 0x19, 1, 0xd, 1, 0xf, 1, 0x13; 16 rows) follows the key nation
/// order (`key_nation_sorted_order`(0)); `G_SELECTED_NATION_COUNT` is recounted as the nations with bit 2 of
/// `nation::LEAGUE_SELECTED`. One row per entry with a nation: in the read-only view, the name ("  %s", yellow when the
/// nation loads), SELECTED (orange when bit 2; a button id 0xd when only bit 1, to promote it; grey otherwise),
/// BACKGROUND (orange when bit 1; a button id 0xe to demote it when the nation loads, more than one league is selected
/// and none of the last 16 staff manages a club of that nation, `staff::CLUB_JOB`; grey otherwise) and the extra
/// division (`key_nation_extra_division_name`, orange when bit 4). Otherwise the name is a button (id 0xc, yellow when it
/// loads, pale grey otherwise) when the entry has a setup function, or grey text; with a setup function come SELECTED
/// (0xd), BACKGROUND (0xe) and the extra division (0x38) buttons, orange with style 0x801 when their bit is set.
/// Finally the back/next buttons (next: in the read-only view while the screen manager can go forward, otherwise with a
/// league selected), the list scrolled to param 0 and the list's index in param 1.
/// Original: `setup_build_select_leagues_screen` (0x81d010, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_select_leagues_screen() {
    covered!("tel11_1 build select leagues");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let extra_division: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_KEY_NATION_EXTRA_DIVISION_NAME);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let sm = SCREEN_MANAGER;
    let mut list_columns: [u8; 7] = [0x19, 1, 0xd, 1, 0xf, 1, 0x13];
    let mut options_columns: [u8; 9] = [0x2a, 0xd, 1, 0xd, 0xa, 0x2a, 0xd, 1, 0xd];
    let mut masking_columns: [u8; 9] = [0, 0, 0, 0, 0, 0x2a, 0xd, 1, 0xd];
    let limit = setup_league_memory_limit();
    write_u32(G_NEW_GAME_SETUP_DONE_FLAG, 0);
    exe::background_set_random_picture();
    let read_only = read_u32(G_INTERACTIVE_MODE_FLAG) != 0;
    setup_screen_header(if read_only { 0xa8042c } else { 0xa80418 });
    let grey = setup_colour(G_GUI_COLOUR_GREY);
    let pale = setup_colour(G_GUI_COLOUR_PALE_GREY);
    let orange = setup_colour(G_GUI_COLOUR_ORANGE);
    let yellow = setup_colour(G_GUI_COLOUR_YELLOW);
    // a Yes/No pair: the chosen one orange (style 0x801), the other grey; `ids` = (yes, no), none in the read-only view
    let yes_no = |a: usize, on: bool, yes_col: usize, no_col: usize, ids: (usize, usize)| {
        text(SETUP_TEXT, 0x98e8d0); // "Yes"
        let (style, tc) = if on { (0x801, orange) } else { (1, grey) };
        button(sm, a, yes_col, 0, style, grey, 0xc, 1, tc, SETUP_TEXT, ids.0, 0);
        text(SETUP_TEXT, 0x98e8cc); // "No"
        let (style, tc) = if on { (1, grey) } else { (0x801, orange) };
        button(sm, a, no_col, 0, style, grey, 0xc, 1, tc, SETUP_TEXT, ids.1, 0);
    };
    let mut options = 0usize;
    let german = read_u8(G_PLAYER_SETUP_GERMAN_FLAG) != 0;
    if !german {
        options = area(current_human_display(), 0x6e, 0x91, 0x20d, 0xa5, 9, options_columns.as_mut_ptr() as usize, 1, 0, 2, 0, usize::MAX);
        text(SETUP_TEXT, 0xa80404); // "  Use Real Players:"
        button(sm, options, 0, 0, 1, grey, 1, 1, pale, SETUP_TEXT, 0, 0);
        let real = read_u8(G_USE_REAL_PLAYERS) != 0;
        yes_no(options, real, 1, 3, if read_only { (0, 0) } else { (0x39, 0x3a) });
    }
    if german || options as i16 == 0 {
        options = area(current_human_display(), 0x6e, 0x91, 0x20d, 0xa5, 9, masking_columns.as_mut_ptr() as usize, 1, 0, 2, 0, usize::MAX);
    }
    text(SETUP_TEXT, 0xa803ec); // "  Attribute Masking:"
    button(sm, options, 5, 0, 1, grey, 1, 1, pale, SETUP_TEXT, 0, 0);
    let masking = read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0;
    if read_only {
        yes_no(options, masking, 6, 7, (0, 0)); // tel11_1 the read-only No in the spacer column
    } else {
        yes_no(options, masking, 6, 8, (0x3b, 0x3c));
    }
    let all = area(current_human_display(), 0x212, 0x91, 0x30c, 0xa5, 2, 0, 1, 0, 1, 0, usize::MAX);
    text(SETUP_TEXT, 0x99b4c8); // "Select All"
    if !read_only && read_i32(G_SELECTED_NATION_COUNT) < 0xf && limit >= 0x10 {
        button(sm, all, 0, 0, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0xa, 0);
    } else {
        label(sm, all, 0, 0, 0x30, grey, 0x2c, 1, pale, SETUP_TEXT);
    }
    text(SETUP_TEXT, 0xa803dc); // "De-Select All"
    if !read_only && read_i32(G_NATIONS_TO_LOAD_COUNT) > 0 {
        button(sm, all, 1, 0, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0xb, 0);
    } else {
        label(sm, all, 1, 0, 0x30, grey, 0x2c, 1, pale, SETUP_TEXT);
    }
    // the rows are the push at 0x81d6dc in the running exe (0x10; 0x15 in the 1920x1080 exe)
    let rows = read_i8(0x81d6dd) as usize;
    let list = area(current_human_display(), 0x6e, 0xaa, 0x30c, 0x217, 7, list_columns.as_mut_ptr() as usize, rows, 0, 2, 0, usize::MAX);
    let order = exe::key_nation_sorted_order(0);
    write_u32(G_SELECTED_NATION_COUNT, 0);
    for k in 0..0x23 {
        let n = read_usize(setup_league_entry(order, k));
        if n != 0 && read_u8(n + nation::LEAGUE_SELECTED) & 2 != 0 {
            setup_count(G_SELECTED_NATION_COUNT, 1);
        }
    }
    let mut row = 0usize;
    for k in 0..0x23 {
        let e = setup_league_entry(order, k);
        let n = read_usize(e);
        if n == 0 {
            continue;
        }
        let flags = || read_u8(n + nation::LEAGUE_SELECTED);
        format(SETUP_TEXT, 0x987ff0, n + 4); // "  %s"
        if read_only {
            label(sm, list, 0, row, 1, 0, 1, 3, if flags() == 0 { grey } else { yellow }, SETUP_TEXT);
            text(SETUP_TEXT, 0xa803d0); // "SELECTED"
            let b = flags();
            if b & 2 != 0 {
                label(sm, list, 2, row, 0x801, 0, 0xc, 2, orange, SETUP_TEXT);
            } else if b & 1 != 0 {
                button(sm, list, 2, row, 1, 0, 0xc, 2, pale, SETUP_TEXT, 0xd, n);
            } else {
                label(sm, list, 2, row, 1, 0, 0xc, 2, grey, SETUP_TEXT);
            }
            text(SETUP_TEXT, 0xa803c4); // "BACKGROUND"
            let b = flags();
            if b & 1 != 0 {
                label(sm, list, 4, row, 0x801, 0, 0xc, 2, orange, SETUP_TEXT);
            } else {
                let mut demotable = read_i32(G_SELECTED_NATION_COUNT) != 1;
                if demotable {
                    let staff = read_usize(G_STAFF_TABLE);
                    let count = read_i32(G_STAFF_COUNT) as isize;
                    for j in 0..0x10isize {
                        let s = Staff((staff as isize + (count - j - 1) * staff::SIZE as isize) as usize);
                        if s.classification() != 0 {
                            let club = s.club_job();
                            if !club.is_null() && club.nation().addr() == n {
                                demotable = false;
                                break;
                            }
                        }
                    }
                }
                if demotable && b != 0 {
                    button(sm, list, 4, row, 1, 0, 0xc, 2, pale, SETUP_TEXT, 0xe, n);
                } else {
                    label(sm, list, 4, row, 1, 0, 0xc, 2, grey, SETUP_TEXT);
                }
            }
            let b = flags();
            if extra_division(n, SETUP_TEXT, 0x7d0) != 0 {
                if b & 4 != 0 {
                    label(sm, list, 6, row, 0x801, 0, 0xc, 2, orange, SETUP_TEXT);
                } else {
                    label(sm, list, 6, row, 1, 0, 0xc, 2, grey, SETUP_TEXT);
                }
            }
        } else {
            let selectable = read_usize(e + KEY_NATION_RECORD_SETUP_FUNC) != 0;
            if !selectable {
                label(sm, list, 0, row, 1, 0, 1, 3, grey, SETUP_TEXT);
            } else {
                button(sm, list, 0, row, 1, 0, 1, 3, if flags() != 0 { yellow } else { pale }, SETUP_TEXT, 0xc, n);
                let marked = |bit: u8| if flags() & bit != 0 { (0x801usize, orange) } else { (1usize, grey) };
                text(SETUP_TEXT, 0xa803d0); // "SELECTED"
                let (style, tc) = marked(2);
                button(sm, list, 2, row, style, 0, 0xc, 2, tc, SETUP_TEXT, 0xd, n);
                text(SETUP_TEXT, 0xa803c4); // "BACKGROUND"
                let (style, tc) = marked(1);
                button(sm, list, 4, row, style, 0, 0xc, 2, tc, SETUP_TEXT, 0xe, n);
                let b = flags();
                if extra_division(n, SETUP_TEXT, 0x7d0) != 0 {
                    let (style, tc) = if b & 4 != 0 { (0x801usize, orange) } else { (1usize, grey) };
                    button(sm, list, 6, row, style, 0, 0xc, 2, tc, SETUP_TEXT, 0x38, n);
                }
            }
        }
        row += 1;
    }
    let next = if read_only {
        (exe::screenmanager_is_forward_unavailable(sm) == 0) as u32
    } else {
        (read_i32(G_SELECTED_NATION_COUNT) != 0) as u32
    };
    let back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, next);
    let scroll = exe::screenmanager_get_screen_param(sm, 0);
    if list as i16 != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + (list as i16 as isize * 0xbf1) as usize, scroll);
    }
    exe::screenmanager_set_screen_param(sm, 1, list as i16 as isize as usize, 0);
}

/// The Select League(s) screen's answer to item `item`.
///
/// Without a database: −2. A real item first keeps the list's scroll row in param 0. Then, by the clicked id
/// (`setup_clicked_id`), all −11 unless said otherwise. 0xa Select All: each league entry with a nation and a setup
/// function that is not loaded becomes selected (bit 2; one more to load and one more selected), one in the background
/// (bit 1) is promoted (bit 1 off, bit 2 on; one more selected); each of them becomes `G_MAIN_NATION` if there is none.
/// 0xb De-Select All: `nation::LEAGUE_SELECTED` of every nation and the three counters zeroed. 0x39/0x3a: Use Real
/// Players yes/no; 0x3b/0x3c: Attribute Masking yes/no. 0xc (the name) and 0xd (SELECTED) on nation n (the item's
/// value): not loaded, it becomes selected (after the memory check `setup_room_for_one_more_league`; both counters, the
/// main nation if none); 0xc on a loaded one unloads it (one less to load, one less selected when it was, the flags
/// zeroed); 0xd on a selected one unloads it (one less to load, again when it had the extra division, one less selected)
/// and on a background one promotes it. When the main nation is unloaded, the first entry with bit 2 takes its place.
/// 0x38 (extra division): a nation not loaded is first selected (memory check), then the extra division (bit 4)
/// toggles, with one more or one less to load. 0xe (BACKGROUND): not loaded, it loads in the background (memory check;
/// one more to load); in the background, it is unloaded (one less to load, again with the extra division); selected, it
/// goes to the background (one less selected) and, if it was the main nation, the first entry whose flags are exactly 2
/// takes its place. −3 (next): −3 in the read-only view; 0 with nothing selected; with more than one league the Select
/// Start Season screen (−4); with one, the game is created (`setup_create_chosen_game`, fatal line 0xa0c). Any other goes
/// to the menu bar.
/// Original: `setup_handle_select_leagues_input` (0x81dc30, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_select_leagues_input(item: u32) -> i32 {
    covered!("tel11_1 select leagues input");
    let sm = SCREEN_MANAGER;
    if read_usize(G_NATION_TABLE) == 0 {
        return -2;
    }
    let raw = item;
    let item = item as i16;
    if item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(sm, exe::screenmanager_get_screen_param(sm, 1));
        exe::screenmanager_set_screen_param(sm, 0, row, 0);
    }
    let flags = |n: usize| read_u8(n + nation::LEAGUE_SELECTED);
    let set_flags = |n: usize, b: u8| write_u8(n + nation::LEAGUE_SELECTED, b);
    let main_if_none = |n: usize| {
        if read_usize(G_MAIN_NATION) == 0 {
            write_usize(G_MAIN_NATION, n);
        }
    };
    match setup_clicked_id(item) {
        0xa => {
            let mut e = G_LEAGUE_TABLE;
            while e < G_KEY_NATION_ACTIVE_COUNT {
                let n = read_usize(e);
                if n != 0 && read_usize(e + KEY_NATION_RECORD_SETUP_FUNC) != 0 {
                    let b = flags(n);
                    if b == 0 {
                        setup_count(G_NATIONS_TO_LOAD_COUNT, 1);
                        setup_count(G_SELECTED_NATION_COUNT, 1);
                        set_flags(n, flags(n) | 2);
                        main_if_none(n);
                    } else if b & 1 != 0 {
                        setup_count(G_SELECTED_NATION_COUNT, 1);
                        set_flags(n, flags(n).wrapping_sub(1) | 2);
                        main_if_none(n);
                    }
                }
                e += 0x48;
            }
            -11
        }
        0x39 => {
            write_u8(G_USE_REAL_PLAYERS, 1);
            -11
        }
        0x3a => {
            write_u8(G_USE_REAL_PLAYERS, 0);
            -11
        }
        0x3c => {
            write_u8(G_ATTRIBUTE_MASKING_ENABLED, 0);
            -11
        }
        0x3b => {
            write_u8(G_ATTRIBUTE_MASKING_ENABLED, 1);
            -11
        }
        0xb => {
            let mut k = 0;
            while k < read_i32(G_NATION_COUNT) {
                write_u8(read_usize(G_NATION_TABLE) + k as usize * nation::SIZE + nation::LEAGUE_SELECTED, 0);
                k += 1;
            }
            write_u32(G_NATIONS_TO_LOAD_COUNT, 0);
            write_u32(G_SELECTED_NATION_COUNT, 0);
            write_usize(G_MAIN_NATION, 0);
            -11
        }
        0xc => {
            let n = setup_clicked_value(item);
            if flags(n) == 0 {
                if !setup_room_for_one_more_league() {
                    return -11;
                }
                setup_count(G_NATIONS_TO_LOAD_COUNT, 1);
                setup_count(G_SELECTED_NATION_COUNT, 1);
                set_flags(n, 2);
                main_if_none(n);
            } else {
                setup_count(G_NATIONS_TO_LOAD_COUNT, -1);
                if flags(n) & 2 != 0 {
                    setup_count(G_SELECTED_NATION_COUNT, -1);
                }
                set_flags(n, 0);
                if n == read_usize(G_MAIN_NATION) {
                    setup_pick_main_nation(|b| b & 2 != 0);
                }
            }
            -11
        }
        0x38 => {
            let n = setup_clicked_value(item);
            if flags(n) == 0 {
                if !setup_room_for_one_more_league() {
                    return -11;
                }
                set_flags(n, 2);
                setup_count(G_NATIONS_TO_LOAD_COUNT, 1);
                setup_count(G_SELECTED_NATION_COUNT, 1);
                main_if_none(n);
            }
            let b = flags(n);
            if b & 4 != 0 {
                set_flags(n, b - 4);
                setup_count(G_NATIONS_TO_LOAD_COUNT, -1); // tel11_1 the extra division leaves the load
            } else {
                set_flags(n, b | 4);
                setup_count(G_NATIONS_TO_LOAD_COUNT, 1);
            }
            -11
        }
        0xd => {
            let n = setup_clicked_value(item);
            let b = flags(n);
            if b == 0 {
                if !setup_room_for_one_more_league() {
                    return -11;
                }
                setup_count(G_NATIONS_TO_LOAD_COUNT, 1);
                setup_count(G_SELECTED_NATION_COUNT, 1);
                set_flags(n, 2);
                main_if_none(n);
            } else if b & 2 != 0 {
                setup_count(G_NATIONS_TO_LOAD_COUNT, -1);
                if flags(n) & 4 != 0 {
                    setup_count(G_NATIONS_TO_LOAD_COUNT, -1);
                }
                setup_count(G_SELECTED_NATION_COUNT, -1);
                set_flags(n, 0);
                if n == read_usize(G_MAIN_NATION) {
                    setup_pick_main_nation(|b| b & 2 != 0);
                }
            } else {
                setup_count(G_SELECTED_NATION_COUNT, 1);
                set_flags(n, (flags(n) & 0xfe) | 2);
                main_if_none(n);
            }
            -11
        }
        0xe => {
            let n = setup_clicked_value(item);
            let b = flags(n);
            if b == 0 {
                if !setup_room_for_one_more_league() {
                    return -11;
                }
                setup_count(G_NATIONS_TO_LOAD_COUNT, 1);
                set_flags(n, 1);
            } else if b & 1 != 0 {
                setup_count(G_NATIONS_TO_LOAD_COUNT, -1);
                if flags(n) & 4 != 0 {
                    setup_count(G_NATIONS_TO_LOAD_COUNT, -1);
                }
                set_flags(n, 0);
            } else {
                setup_count(G_SELECTED_NATION_COUNT, -1);
                set_flags(n, (flags(n) & 0xfd) | 1);
                if n == read_usize(G_MAIN_NATION) {
                    setup_pick_main_nation(|b| b == 2);
                }
            }
            -11
        }
        0xfffd => {
            if read_u32(G_INTERACTIVE_MODE_FLAG) != 0 {
                return -3;
            }
            let selected = read_i32(G_SELECTED_NATION_COUNT);
            if selected == 0 {
                return 0;
            }
            if selected > 1 {
                exe::screenmanager_open_screen(sm, 0x81e8d0usize, 0x81eb00usize, 0, 0usize, 0);
                return -4;
            }
            setup_create_chosen_game(0xa0c)
        }
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Builds the Select Start Season screen: one menu button per selected league with its season.
///
/// A random background and the title bar ("Select Start Season"). Following the key nation order
/// (`key_nation_sorted_order`(1)), each entry whose nation has bit 2 of `nation::LEAGUE_SELECTED` gets one of the 35 menu
/// items (0xd4 bytes, CRT vector iterators): the text "%s %02d/%02d" with the nation's +0x38 name, the start year
/// (`key_nation_record::START_DATE_YEAR`) mod 100 and the next year mod 10 (a defect of the original: from 2009 the
/// second year shows only its last digit) for a season over two years (`key_nation_record::SPLIT_SEASON_FLAG`),
/// otherwise "%s %d" with the year; id 0xf and the nation as value. The item of `G_MAIN_NATION` is bold (when it has
/// none, the original shifts by stack garbage; here the first item). The menu (0x6e, 0x91 to 0x30c, 0x217, font 3,
/// sorted, `G_SELECTED_NATION_COUNT` items) has 3 columns and 7 rows above 12 leagues, otherwise 2 and 6; then the back
/// (while the screen manager can go back) and next buttons.
/// Original: `setup_build_select_start_season_screen` (0x81e8d0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_select_start_season_screen() {
    covered!("tel11_1 build start season");
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let menu: unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, usize, u32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_MENU_BUTTONS);
    let mut items = [0u32; 0x23 * 0xd4 / 4];
    let base = items.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(base, 0xd4, 0x23, 0x4568f0usize, 0x5a53c0usize);
    exe::background_set_random_picture();
    setup_screen_header(0xa804ac); // "Select Start Season"
    let order = exe::key_nation_sorted_order(1);
    let mut count = 0usize;
    let mut bold = 0u32; // simplification: uninitialised in the original when the main nation has no item
    for k in 0..0x23 {
        let e = setup_league_entry(order, k);
        let n = Nation(read_usize(e));
        if n.is_null() || n.league_selected() & 2 == 0 {
            continue;
        }
        let item = base + count * 0xd4;
        let year = read_u16(e + key_nation_record::START_DATE_YEAR) as i32;
        if read_u32(e + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
            format(item + 4, 0xa8049c, n.addr() + nation::SHORT_NAME, year % 100, (year + 1) % 10); // "%s %02d/%02d"
        } else {
            format(item + 4, 0xa80494, n.addr() + nation::SHORT_NAME, year); // "%s %d"
        }
        write_u32(item + 0xcc, 0xf);
        write_usize(item + 0xd0, n.addr());
        if n.addr() == read_usize(G_MAIN_NATION) {
            bold = count as u32;
        }
        count += 1;
    }
    let selected = read_i32(G_SELECTED_NATION_COUNT);
    let mask = 1u32.wrapping_shl(bold);
    if selected > 0xc {
        menu(0x6e, 0x91, 0x30c, 0x217, 3, 7, 3, selected, base, mask, 1);
    } else {
        menu(0x6e, 0x91, 0x30c, 0x217, 2, 6, 3, selected, base, mask, 1);
    }
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, 1);
    exe::crt_vector_destructor_iterator(base, 0xd4, 0x23, 0x5a53c0usize);
}

/// The Select Start Season screen's answer to item `item`: id 0xf makes the button's nation (its value; none without
/// an item) the main nation `G_MAIN_NATION` (−11); −3 (next) creates the game (`setup_create_chosen_game`, fatal line
/// 0xaa5); any other goes to the menu bar.
/// Original: `setup_handle_select_start_season_input` (0x81eb00, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_select_start_season_input(item: u32) -> i32 {
    covered!("tel11_1 start season input");
    let raw = item;
    let item = item as i16;
    match setup_clicked_id(item) {
        0xf => {
            write_usize(G_MAIN_NATION, setup_clicked_value(item));
            -11
        }
        0xfffd => setup_create_chosen_game(0xaa5),
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Comparer of `setup_run_front_end`: only the original runs (the whole Setup runs inside it, with the screen loop, the
/// database load and the game's creation; it cannot run twice). Its pieces are compared by their own comparers; the copy
/// is proved in Rust mode, where the automatic career goes through it.
pub(crate) unsafe extern "C" fn compare_setup_run_front_end() -> i32 {
    original_fn::<unsafe extern "C" fn() -> i32>(IDX_SETUP_RUN_FRONT_END)()
}

/// Comparer of `setup_build_setup_game_screen`: side by side over the display list (`compare_by_display_list_with`,
/// which also restores the generator for the random background), with `G_NETWORK_SETUP_HOST_FLAG`.
pub(crate) unsafe extern "C" fn compare_setup_build_setup_game_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_SETUP_GAME_SCREEN);
    // the original's nine tab objects leave the value dword (+0xd0) uninitialised (the constructor 0x4568f0 writes +0 and
    // +2 only) and gui_add_menu_buttons passes it to each button as the item value (0x5edb6f): stack garbage, unread by
    // the input routine (it reads the id); the copy passes 0. The buttons' values (ids 1..9) are not compared.
    let clear_values = || {
        let d = current_human_display();
        for k in 0..read_i16(d + display_list::ITEM_COUNT) {
            let item = display_item_at(d, k);
            if (1..=9).contains(&read_u16(item + 8)) {
                write_u32(item + 0x48, 0);
            }
        }
    };
    compare_by_display_list_with("Setup Game screen", &[(G_NETWORK_SETUP_HOST_FLAG, 4)], || { o(); clear_values() }, || {
        setup_build_setup_game_screen();
        clear_values()
    });
}

/// Comparer of `setup_start_new_game`: only the original runs (it loads the database and opens Select League(s)).
pub(crate) unsafe extern "C" fn compare_setup_start_new_game() {
    original_fn::<unsafe extern "C" fn()>(IDX_SETUP_START_NEW_GAME)()
}

/// Comparer of `setup_build_select_leagues_screen`: side by side over the display list, with the counter of selected
/// leagues it recounts, `G_NEW_GAME_SETUP_DONE_FLAG` and the key nation order that `key_nation_sorted_order` rewrites.
pub(crate) unsafe extern "C" fn compare_setup_build_select_leagues_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_SELECT_LEAGUES_SCREEN);
    let extra = [(G_SELECTED_NATION_COUNT, 4), (G_NEW_GAME_SETUP_DONE_FLAG, 4), (G_KEY_NATION_ORDER, 0x23)];
    compare_by_display_list_with("Select League(s) screen", &extra, || o(), || setup_build_select_leagues_screen());
}

/// Comparer of `setup_handle_select_leagues_input`: the answers that only change the league flags, the counters and the
/// options (Select All, De-Select All, the four option buttons, and the league buttons when no memory box will open)
/// run side by side (`compare_setup_answer`); next (−3), the memory box and the menu bar run only in the original.
pub(crate) unsafe extern "C" fn compare_setup_handle_select_leagues_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_SELECT_LEAGUES_INPUT);
    if read_usize(G_NATION_TABLE) == 0 {
        return o(item);
    }
    let id = setup_clicked_id(item as i16);
    let pure = match id {
        0xa | 0xb | 0x39 | 0x3a | 0x3b | 0x3c => true,
        0xc | 0xd | 0xe | 0x38 => {
            let n = Nation(setup_clicked_value(item as i16));
            // simplification: the memory box (a modal box) does not run twice: original only when it would open
            !n.is_null() && (n.league_selected() != 0
                || exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa80484) != 0
                || read_u32(G_MEMORY_CHECK_ENABLED) == 0
                || read_i32(G_NATIONS_TO_LOAD_COUNT) < setup_league_memory_limit())
        }
        _ => false,
    };
    if !pure {
        return o(item);
    }
    compare_setup_answer("Select League(s)", item, || o(item), || setup_handle_select_leagues_input(item))
}

/// Comparer of `setup_build_select_start_season_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_setup_build_select_start_season_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_SELECT_START_SEASON_SCREEN);
    compare_by_display_list_with("Select Start Season screen", &[(G_KEY_NATION_ORDER, 0x23)], || o(), || setup_build_select_start_season_screen());
}

/// Comparer of `setup_handle_select_start_season_input`: the season button (id 0xf) runs side by side over the main
/// nation; next (it creates the game) and the menu bar run only in the original.
pub(crate) unsafe extern "C" fn compare_setup_handle_select_start_season_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_SELECT_START_SEASON_INPUT);
    if setup_clicked_id(item as i16) != 0xf {
        return o(item);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(G_MAIN_NATION, 4)], || o(item), || setup_handle_select_start_season_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Select Start Season answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Builds the Network Play screen: the title bar ("Network Play"), the menu of four buttons (Start Network Game 0x10,
/// Quick Start Network Game 0x11, Restore Network Game 0x12, Join Network Game 0x13; 4 menu items of 0xd4 bytes, CRT
/// vector iterators) at 0x6e, 0x91 to 0x30c, 0x217 (2 columns, 6 rows, font 3) and the back button.
/// Original: `setup_build_network_play_screen` (0x81edd0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_network_play_screen() {
    covered!("tel11_2 build network play");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let menu: unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, usize, u32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_MENU_BUTTONS);
    let mut items = [0u32; 4 * 0xd4 / 4];
    let base = items.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(base, 0xd4, 4, 0x4568f0usize, 0x5a53c0usize);
    exe::background_set_random_picture();
    setup_screen_header(0xa80220); // "Network Play"
    let entries: [(usize, u32); 4] = [(0xa80508, 0x10), (0xa804ec, 0x11), (0xa804d4, 0x12), (0xa804c0, 0x13)];
    for (k, &(label, id)) in entries.iter().enumerate() {
        text(base + k * 0xd4 + 4, label);
        write_u32(base + k * 0xd4 + 0xcc, id);
    }
    menu(0x6e, 0x91, 0x30c, 0x217, 2, 6, 3, 4, base, 0, 0);
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, 0);
    exe::crt_vector_destructor_iterator(base, 0xd4, 4, 0x5a53c0usize);
}

/// The Network Play screen's answer to item `item`: Start (0x10), Quick Start (0x11) and Restore (0x12) open the Enter
/// Network Game Name screen with params 0 and 1 = (0, 0), (1, 1) and (1, 0) and the game name emptied; Join (0x13) opens
/// Select Network Game with param 0 = 0 and the name emptied; all −4 (also when the screen does not open). −3 returns 0;
/// any other goes to the menu bar.
/// Original: `setup_handle_network_play_input` (0x81ef70, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_network_play_input(item: u32) -> i32 {
    covered!("tel11_2 network play input");
    let sm = SCREEN_MANAGER;
    let id = setup_clicked_id(item as i16);
    let (build, input, params): (usize, usize, &[(u32, usize)]) = match id {
        0x10 => (0x81f1a0, 0x81f2c0, &[(0, 0), (1, 0)]),
        0x11 => (0x81f1a0, 0x81f2c0, &[(0, 1), (1, 1)]),
        0x12 => (0x81f1a0, 0x81f2c0, &[(0, 1), (1, 0)]),
        0x13 => (0x81f8d0, 0x81fbd0, &[(0, 0)]),
        0xfffd => return 0,
        _ => {
            let r = exe::menubar_handle_command(item);
            return if r != 0 { r } else { exe::menubar_handle_object_link(item) };
        }
    };
    if exe::screenmanager_open_screen(sm, build, input, 0, 0usize, 0) != 0 {
        for &(k, v) in params {
            exe::screenmanager_set_screen_param(sm, k, v, 0);
        }
        write_u8(G_NETWORK_GAME_NAME, 0);
    }
    -4
}

/// Builds the Enter Network Game Name screen: `G_NETWORK_SETUP_HOST_FLAG` and `G_NETWORK_GAME_NAMED_FLAG` zeroed; without
/// a network object (`G_SCREEN_MANAGER_NETWORK`) a network session starts (`ScreenManager::start_network_session` with
/// the join and leave hooks 0x838a80/0x838c00); a random background, the title bar ("Enter Network Game Name"), the
/// text entry of the game name (`Display::add_item` type 0x18 at 0x96, 0x136 to 0x2e4, 0x172, style 0x22, dark blue, font
/// 4, pale grey, `G_NETWORK_GAME_NAME`, 0x32 characters, id 0x14) and the back/next buttons (next with a name).
/// Original: `setup_build_enter_network_game_name_screen` (0x81f1a0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_enter_network_game_name_screen() {
    covered!("tel11_2 build enter network game name");
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    write_u32(G_NETWORK_SETUP_HOST_FLAG, 0);
    write_u32(G_NETWORK_GAME_NAMED_FLAG, 0);
    if read_usize(G_SCREEN_MANAGER_NETWORK) == 0 {
        exe::screenmanager_start_network_session(SCREEN_MANAGER, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK);
    }
    exe::background_set_random_picture();
    setup_screen_header(0xa8051c); // "Enter Network Game Name"
    item(current_human_display(), 0x18, 0x96, 0x136, 0x2e4, 0x172, 0, 0, 0x22, setup_colour(G_GUI_COLOUR_DARK_BLUE), 0, 0xc, 4,
        setup_colour(G_GUI_COLOUR_PALE_GREY), G_NETWORK_GAME_NAME, 0x32, 0x14, 0, usize::MAX);
    let named = (read_u8(G_NETWORK_GAME_NAME) != 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, named);
}

/// The Enter Network Game Name screen's answer to item `item`.
///
/// Params 0 (0: start a new game) and 1 (quick start rather than restore) are read first. With a command waiting
/// (`G_PENDING_COMMAND_DISPATCH_FLAG`), codes 4 and 5 from the file dialog load the chosen file
/// (`setup_load_chosen_game`, freeing the name, the boxes opened here, fatal line 0xbd0; no save flag change); then the
/// network game is created (`setup_create_or_join_network_game`(0, `G_NETWORK_GAME_NAME`), whose nonzero answer is
/// returned): with no human manager (or without staff, box 0x39d8) Manager Status opens with params 0..4 zeroed and a
/// new manager is added (−4); with restored network managers to assign (`setup_assign_restored_network_managers`, asked
/// before the game is created) Manager Status opens with params 0..4 zeroed (−4); otherwise the name is cleared and the
/// click is handled. Then: the name entry (0x14) −11; next (−3) with a name: `G_NETWORK_GAME_NAMED_FLAG` = 1 and a new
/// game starts (`setup_start_new_game`, −4) or the file dialog opens on the quick starts (param 1) or the saved games
/// (as in Setup Game, with the network titles; −4); without a name −11. Back (−2) ends the network session and goes, as
/// any other, to the menu bar.
/// Original: `setup_handle_enter_network_game_name_input` (0x81f2c0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_enter_network_game_name_input(item: u32) -> i32 {
    covered!("tel11_2 enter network game name input");
    let sm = SCREEN_MANAGER;
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let assign: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_SETUP_ASSIGN_RESTORED_NETWORK_MANAGERS);
    let add_manager: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_ADD_HUMAN_MANAGER);
    let new_game: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_START_NEW_GAME);
    let new_game_param = exe::screenmanager_get_screen_param(sm, 0);
    let quick_param = exe::screenmanager_get_screen_param(sm, 1);
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 {
        let code = read_u32(G_PENDING_COMMAND_CODE);
        if code == 4 || code == 5 {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            exe::config_apply_and_save(read_usize(G_CONFIG));
            let name = read_usize(G_PENDING_COMMAND_ARG);
            if name != 0 {
                covered!("tel11_2 enter network game name load chosen");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                if let Some(r) = setup_load_chosen_game(name, code, true, false, 0xbd0, false) {
                    return r;
                }
                if setup_count_human_managers(false) == 0 {
                    let r = exe::setup_create_or_join_network_game(0, G_NETWORK_GAME_NAME);
                    if r != 0 {
                        return r;
                    }
                    setup_open_screen(0x81ff10, 0x820950, 0, 5);
                    add_manager();
                    return -4;
                }
                let restored = assign();
                let r = exe::setup_create_or_join_network_game(0, G_NETWORK_GAME_NAME);
                if r != 0 {
                    return r;
                }
                if restored != 0 {
                    setup_open_screen(0x81ff10, 0x820950, 0, 5);
                    return -4;
                }
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
    }
    match setup_clicked_id(item as i16) {
        0x14 => -11,
        0xfffd => {
            if read_u8(G_NETWORK_GAME_NAME) == 0 {
                return -11;
            }
            write_u32(G_NETWORK_GAME_NAMED_FLAG, 1);
            if new_game_param == 0 {
                new_game();
                return -4;
            }
            copy_text(G_QUICK_START_FILE_PATH as *mut u8, exe::utils_build_file_path(0xa80204, 0xad9c64) as *const u8);
            let (pale, red) = (setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_RED));
            let mut title = [0u8; 0x100];
            let mut heading = [0u8; 0x100];
            let h = text(heading.as_mut_ptr() as usize, 0xa7fc84); // "Select File"
            if quick_param != 0 {
                let t = text(title.as_mut_ptr() as usize, 0xa804ec); // "Quick Start Network Game"
                exe::open_file_dialog(t, h, 0, G_QUICK_START_FILE_PATH, 0xa803b4, 0, 0x839290, pale, red, 4, 0, 1, 0, 1);
            } else {
                let config = read_usize(G_CONFIG);
                let t = text(title.as_mut_ptr() as usize, 0xa804d4); // "Restore Network Game"
                exe::open_file_dialog(t, h, 0, config + CONFIG_SAVE_FOLDER, 0xa5aab8, config + config::SAVE_NAME, 0x838e10, pale, red, 5, 0, 0, 0, 1);
            }
            -4
        }
        id => {
            if id == 0xfffe {
                exe::screenmanager_end_network_session(sm);
            }
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Builds the Select Network Game screen: the list of the network games found, Refresh and the name to join.
///
/// Without a network object a session starts and, if it exists then, the list request goes out
/// (`setup_request_network_games`). A random background and the title bar ("Select Network Game"); the Refresh button
/// (`ScreenManager::add_button_item` at 0x28f, 0x91 to 0x30c, 0xa5, id 0x15); the "Join :" area (0xbe, 0xc3 to 0x2bc,
/// 0xe7, columns 1, 3) with its label and the name entry (`Display::add_item` type 0x18 in that area, pale cyan,
/// `G_NETWORK_GAME_NAME`, 0x32 characters, id 0x16); the list area (0xbe, 0xea to 0x2bc, 0x1fe, 6 rows). With a network
/// object its 10 names (500 bytes) are copied into `G_NETWORK_GAME_LIST_COPY` and each found game is a button (style
/// 0x22, dark blue, font 4, id 0x17, the name as value); without one the count is the word of the column weights
/// (0x301: a defect of the original, it reads 769 names past the copy); param 0 = the count; the rows left up to 6 get an
/// empty text; the back/next buttons (next with a name).
/// Original: `setup_build_select_network_game_screen` (0x81f8d0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_select_network_game_screen() {
    covered!("tel11_2 build select network game");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let screen_button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let sm = SCREEN_MANAGER;
    let mut columns: [u8; 2] = [1, 3];
    if read_usize(G_SCREEN_MANAGER_NETWORK) == 0 {
        exe::screenmanager_start_network_session(sm, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK);
        let net = read_usize(G_SCREEN_MANAGER_NETWORK);
        if net != 0 {
            setup_request_network_games(net);
        }
    }
    exe::background_set_random_picture();
    setup_screen_header(0xa80544); // "Select Network Game"
    let (grey, pale, dark_blue) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_DARK_BLUE));
    text(SETUP_TEXT, 0xa8053c); // "Refresh"
    screen_button(sm, 0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0x15, 0);
    let join = area(current_human_display(), 0xbe, 0xc3, 0x2bc, 0xe7, 2, columns.as_mut_ptr() as usize, 1, 0, 2, 0, usize::MAX);
    text(SETUP_TEXT, 0xa80534); // "Join :"
    label(sm, join, 0, 0, 1, 0, 0xc, 3, pale, SETUP_TEXT);
    item(current_human_display(), 0x18, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 3, setup_colour(G_GUI_COLOUR_PALE_CYAN), G_NETWORK_GAME_NAME, 0x32, 0x16, 0, join);
    let list = area(current_human_display(), 0xbe, 0xea, 0x2bc, 0x1fe, 1, 0, 6, 0, 1, 0, usize::MAX);
    let net = read_usize(G_SCREEN_MANAGER_NETWORK);
    let count = if net != 0 {
        core::ptr::copy((net + NETWORK_GAME_LIST) as *const u8, G_NETWORK_GAME_LIST_COPY as *mut u8, 500);
        read_i16(net + NETWORK_GAME_COUNT) as i32
    } else {
        // tel11_2 without a network the original takes the column weights' word as the count
        i16::from_le_bytes(columns) as i32
    };
    let mut row = 0i32;
    while row < count {
        let name = G_NETWORK_GAME_LIST_COPY + row as usize * 0x32;
        button(sm, list, 0, row as usize, 0x22, dark_blue, 0xc, 4, pale, name, 0x17, name);
        row += 1;
    }
    exe::screenmanager_set_screen_param(sm, 0, count as isize as usize, 0);
    while row < 6 {
        label(sm, list, 0, row as usize, 0x22, dark_blue, 0xc, 4, pale, 0);
        row += 1;
    }
    let named = (read_u8(G_NETWORK_GAME_NAME) != 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, named);
}

/// The Select Network Game screen's answer to item `item`.
///
/// Refresh (0x15) sends the list request again (with a network object). When the count of games found (the network
/// object's, 0 without one) is no longer the one shown (param 0, as a word): −11 (the screen is rebuilt). The name entry
/// (0x16) −11; a game of the list (0x17) copies its name (the item's text) into `G_NETWORK_GAME_NAME`, −11. Next (−3)
/// with a name joins it (`setup_create_or_join_network_game`(1, name)): a nonzero answer sends the list request again and
/// is returned; success runs the network client (`ScreenManager::run_network_client`), disconnects and ends the session,
/// −2; without a name −11. Back (−2) ends the network session and goes, as any other, to the menu bar.
/// Original: `setup_handle_select_network_game_input` (0x81fbd0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_select_network_game_input(item: u32) -> i32 {
    covered!("tel11_2 select network game input");
    let sm = SCREEN_MANAGER;
    let run_client: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SCREENMANAGER_RUN_NETWORK_CLIENT);
    let item16 = item as i16;
    let id = setup_clicked_id(item16);
    let mut net = read_usize(G_SCREEN_MANAGER_NETWORK);
    if id == 0x15 && net != 0 {
        setup_request_network_games(net);
        net = read_usize(G_SCREEN_MANAGER_NETWORK);
    }
    let found = if net != 0 { read_i16(net + NETWORK_GAME_COUNT) } else { 0 };
    if found as u16 != exe::screenmanager_get_screen_param(sm, 0) as u16 {
        return -11;
    }
    match id {
        0x16 => -11,
        0x17 => {
            let name = if item16 == -1 { 0 } else { current_human_display() + item16 as isize as usize * 0x18c + DISPLAY_ITEM_TEXT };
            copy_text(G_NETWORK_GAME_NAME as *mut u8, name as *const u8);
            -11
        }
        0xfffd => {
            if read_u8(G_NETWORK_GAME_NAME) == 0 {
                return -11;
            }
            let r = exe::setup_create_or_join_network_game(1, G_NETWORK_GAME_NAME);
            if r != 0 {
                let net = read_usize(G_SCREEN_MANAGER_NETWORK);
                if net != 0 {
                    setup_request_network_games(net);
                }
                return r;
            }
            run_client(sm);
            exe::screenmanager_disconnect_network(sm);
            exe::screenmanager_end_network_session(sm);
            -2
        }
        _ => {
            if id == 0xfffe {
                exe::screenmanager_end_network_session(sm);
            }
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Opens the Manager Status screen (`setup_build_manager_status_screen`, `setup_handle_manager_status_input`) with its
/// five params zeroed.
/// Original: `setup_open_manager_status_screen` (0x81fea0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_open_manager_status_screen() {
    covered!("tel11_2 open manager status");
    setup_open_screen(0x81ff10, 0x820950, 0, 5);
}

/// Builds the Manager Status screen: the server addresses, the list of the human managers with their job and state,
/// and Finish.
///
/// A random background and the title bar ("Manager Status"). With a network object and local addresses
/// (`ScreenManager::get_local_addresses`, 20-byte texts): "Server IP Address : <first>" followed by " / <next>" for the
/// others, a text item in yellow (0x6e, 0x91 to 0x7e past the text's width in font 2, 0xa5), and the list starts at 0xaa
/// instead of 0x91. Without human managers among the last 16 staff (or without staff: box 0x39d8) a text area says so
/// (with the demo mode hint before the game runs, `G_INTERACTIVE_MODE_FLAG` = 0). Otherwise the list area (0x6e, top to
/// 0x30c, 0x217; 3 columns; as many rows as font 3 lines fit up to 0x207) gets one row per human manager: the name
/// (`exe::database_format_staff_name`, yellow), the job (the national side `staff::NATIONAL_JOB` and/or the club
/// `staff::CLUB_JOB`, `exe::club_get_national_team_name`, "%s/%s" with both) and the state: on holiday ("On Holiday",
/// "(Watching)" when he has a channel; a password blocks Finish; without a channel the row resumes him, id 0x18),
/// computer controlled ("Not Playing", "(locked)" with a password; the row resumes him, id 0x18) or playing ("Playing"
/// until he is done, `ScreenManager::is_player_done` before the game runs, `exe::screenmanager_is_manager_done` after,
/// then "Waiting"); the rows of the others link to their national side or club (id 0x7d5). Before the game runs, the
/// bottom area (0x64, 0x22b to 0x316, 0x24e) has "Back" and "Finish" ("Fortsätt" in Swedish): a button (id 0x19, style
/// 0x70 with "Waiting" while the current manager is done) when no HUMAN_MANAGER +8 count blocks it and no password does,
/// otherwise text; after it, the back/next buttons. Finally the params the answer watches: 0 the human managers, 1 the
/// idle channels (`ScreenManager::count_idle_channels`), 2 HUMAN_MANAGER +8, 3 the connected human managers, 4
/// HUMAN_MANAGER +0xc.
/// Original: `setup_build_manager_status_screen` (0x81ff10, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_manager_status_screen() {
    covered!("tel11_2 build manager status");
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let addresses: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_SCREENMANAGER_GET_LOCAL_ADDRESSES);
    let player_done: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_PLAYER_DONE);
    let idle: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_SCREENMANAGER_COUNT_IDLE_CHANNELS);
    let text_area: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32, usize, usize, ...) -> u32 = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let screen_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let row_item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let sm = SCREEN_MANAGER;
    let hm = || read_usize(G_HUMAN_MANAGER);
    let mut bottom_columns: [u8; 2] = [3, 1];
    let mut may_finish = true;
    exe::background_set_random_picture();
    setup_screen_header(0xa5a47c); // "Manager Status"
    let (grey, pale, yellow) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_YELLOW));
    let mut top = 0x91;
    if read_usize(G_SCREEN_MANAGER_NETWORK) != 0 {
        let mut found = [0u8; 0x180];
        let n = addresses(sm, found.as_mut_ptr() as usize);
        if n != 0 {
            for k in 0..n.max(0) as usize {
                if k == 0 {
                    text(SETUP_TEXT, 0xa806c8, found.as_ptr()); // "Server IP Address : <%s - address>"
                } else {
                    append_text(SETUP_TEXT as *mut u8, 0xa806c4 as *const u8); // " / "
                    append_text(SETUP_TEXT as *mut u8, found.as_ptr().add(k * 20));
                }
            }
            let w = exe::font_text_width(2usize, SETUP_TEXT as *const u8);
            screen_text(sm, 0x6e, 0x91, (w + 0x7e) as usize, 0xa5, 2, 0, 0xc, 2, yellow, SETUP_TEXT, 1);
            top = 0xaa;
        }
    }
    if setup_count_human_managers(false) == 0 {
        text(SETUP_TEXT, if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 { 0xa8063c } else { 0xa805e0 });
        text_area(0x6e, top, 0x30c, 0x217, 2, 0, 0x10, 4, pale, SETUP_TEXT, 0);
    } else {
        let rows = ((0x207 - top) / exe::font_get_height(3)) as u8;
        let list = area(current_human_display(), 0x6e, top as usize, 0x30c, 0x217, 3, 0, rows as usize, 0, 2, 0, usize::MAX);
        let link = |row: usize, id: usize, target: usize| {
            row_item(current_human_display(), 2, 0x6e, 0, 0x30c, 0, 0, row, 1, 0, 0, 0xc, 1, 0, 0, 0, id, target, list);
        };
        let staff = read_usize(G_STAFF_TABLE);
        let count = read_i32(G_STAFF_COUNT) as isize;
        let mut row = 0usize;
        for k in 1..=16isize {
            let s = Staff((staff as isize + (count - k) * staff::SIZE as isize) as usize);
            if s.classification() != 1 {
                continue;
            }
            let mut name = [0u8; 0xc8];
            exe::database_format_staff_name(s, name.as_mut_ptr() as usize, 0xc8, 1);
            format(SETUP_TEXT, 0x987ff0, name.as_ptr()); // "  %s"
            label(sm, list, 0, row, 1, 0, 1, 3, yellow, SETUP_TEXT);
            let (national, club) = (s.national_job().addr(), s.club_job().addr());
            if national != 0 && club != 0 {
                let (mut a, mut b) = ([0u8; 0xc8], [0u8; 0xc8]);
                exe::club_get_national_team_name(national, 0usize, a.as_mut_ptr() as usize, 0xc8, 1, 1);
                exe::club_get_national_team_name(club, 0usize, b.as_mut_ptr() as usize, 0xc8, 1, 1);
                format(SETUP_TEXT, 0x9ce448, a.as_ptr(), b.as_ptr()); // "%s/%s"
                label(sm, list, 1, row, 1, 0, 1, 3, pale, SETUP_TEXT);
            } else if national != 0 || club != 0 {
                let side = if national != 0 { national } else { club };
                exe::club_get_national_team_name(side, 0usize, SETUP_TEXT, 0x7d0, 1, 1);
                label(sm, list, 1, row, 1, 0, 1, 3, pale, SETUP_TEXT);
            }
            if exe::human_manager_is_on_holiday(hm(), s) != 0 {
                let watching = exe::screenmanager_find_manager_channel(sm, s) != 0xffff;
                text(SETUP_TEXT, if watching { 0xa805bc } else { 0xa805d4 }); // "On Holiday (Watching)" / "On Holiday"
                label(sm, list, 2, row, 1, 0, 1, 3, grey, SETUP_TEXT);
                if read_u8(exe::human_manager_get_password(hm(), s)) != 0 {
                    may_finish = false;
                }
                if exe::screenmanager_find_manager_channel(sm, s) == 0xffff {
                    link(row, 0x18, s.addr());
                } else if national != 0 {
                    link(row, 0x7d5, national);
                } else if club != 0 {
                    link(row, 0x7d5, club);
                }
            } else if exe::human_manager_is_computer_controlled(hm(), s) != 0 {
                let locked = read_u8(exe::human_manager_get_password(hm(), s)) != 0;
                text(SETUP_TEXT, if locked { 0xa805a4 } else { 0xa80598 }); // "Not Playing (locked)" / "Not Playing"
                label(sm, list, 2, row, 1, 0, 1, 3, grey, SETUP_TEXT);
                link(row, 0x18, s.addr());
            } else {
                let playing = if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
                    player_done(sm, exe::screenmanager_find_manager_channel(sm, s) as u32) == 0
                } else {
                    exe::screenmanager_is_manager_done(sm, s.addr() as u32) == 0
                };
                if playing {
                    text(SETUP_TEXT, 0xa8056c, 0xad9c64); // "Playing<%s - COMMENT - manager is playing>"
                } else {
                    text(SETUP_TEXT, 0xa16d8c); // "Waiting"
                }
                label(sm, list, 2, row, 1, 0, 1, 3, grey, SETUP_TEXT);
                if national != 0 {
                    link(row, 0x7d5, national);
                } else if club != 0 {
                    link(row, 0x7d5, club);
                }
            }
            row += 1;
        }
    }
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        let bottom = area(current_human_display(), 0x64, 0x22b, 0x316, 0x24e, 2, bottom_columns.as_mut_ptr() as usize, 1, 0, 1, 0, usize::MAX);
        text(SETUP_TEXT, 0x9880a8); // "Back"
        label(sm, bottom, 0, 0, 0x30, grey, 0x2c, 3, pale, SETUP_TEXT);
        if exe::langlib_get_language() == 10 {
            copy_text(SETUP_TEXT as *mut u8, 0xa80560 as *const u8); // "Fortsätt"
        } else {
            text(SETUP_TEXT, 0xa80558); // "Finish"
        }
        if read_u32(hm() + HUMAN_MANAGER_LOCKED_COUNT) == 0 && may_finish {
            let current = read_u32(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
            if exe::screenmanager_is_manager_done(sm, current) != 0 {
                text(SETUP_TEXT, 0xa16d8c); // "Waiting"
                button(sm, bottom, 1, 0, 0x70, grey, 0xc, 3, pale, SETUP_TEXT, 0x19, 0);
            } else {
                button(sm, bottom, 1, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x19, 0);
            }
        } else {
            label(sm, bottom, 1, 0, 0x30, grey, 0x2c, 3, pale, SETUP_TEXT);
        }
    } else {
        let next = (exe::screenmanager_is_forward_unavailable(sm) == 0) as u32;
        let back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
        exe::gui_add_back_next_buttons(back, next);
    }
    exe::screenmanager_set_screen_param(sm, 0, setup_count_human_managers(false) as usize, 0);
    exe::screenmanager_set_screen_param(sm, 1, idle(sm) as i16 as isize as usize, 0);
    exe::screenmanager_set_screen_param(sm, 2, read_usize(hm() + HUMAN_MANAGER_LOCKED_COUNT), 0); // tel11_2 param 2
    exe::screenmanager_set_screen_param(sm, 3, setup_count_human_managers(true) as usize, 0);
    exe::screenmanager_set_screen_param(sm, 4, read_usize(hm() + HUMAN_MANAGER_WATCHED_COUNT), 0);
}

/// The Manager Status screen's answer to item `item`.
///
/// When anything shown changed (params 0..4 against the human managers, the idle channels, HUMAN_MANAGER +8, the
/// connected human managers and HUMAN_MANAGER +0xc, recounted in that order; without staff, box 0x39d8 each time): −11
/// (the screen is rebuilt). A manager's row (0x18, the manager as value): with a password the Enter Password box opens
/// (flag 1; param 0 = the manager and his typed password emptied), otherwise the resume control box (param 0 = the
/// manager); −4. Before the game runs, back (−2) returns 0 and Finish (0x19) sets `G_MANAGER_STATUS_CONFIRMED_FLAG` and
/// returns −5. Any other goes to the menu bar.
/// Original: `setup_handle_manager_status_input` (0x820950, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_manager_status_input(item: u32) -> i32 {
    covered!("tel11_2 manager status input");
    let sm = SCREEN_MANAGER;
    let idle: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_SCREENMANAGER_COUNT_IDLE_CHANNELS);
    let typed: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY);
    let param = |k: u32| exe::screenmanager_get_screen_param(sm, k);
    let hm = || read_usize(G_HUMAN_MANAGER);
    if param(0) != setup_count_human_managers(false) as usize {
        return -11;
    }
    if param(1) != idle(sm) as i16 as isize as usize {
        return -11;
    }
    if param(2) != read_usize(hm() + HUMAN_MANAGER_LOCKED_COUNT) {
        return -11;
    }
    if param(3) != setup_count_human_managers(true) as usize {
        return -11;
    }
    if param(4) != read_usize(hm() + HUMAN_MANAGER_WATCHED_COUNT) {
        return -11;
    }
    let item16 = item as i16;
    let id = setup_clicked_id(item16);
    if id == 0x18 {
        let s = setup_clicked_value(item16);
        if read_u8(exe::human_manager_get_password(hm(), s)) != 0 {
            if exe::screenmanager_open_screen(sm, 0x822ec0usize, 0x8230b0usize, 1, 0usize, 0) != 0 {
                exe::screenmanager_set_screen_param(sm, 0, s, 0);
                write_u8(typed(hm(), s), 0);
            }
        } else if exe::screenmanager_open_screen(sm, 0x823420usize, 0x823660usize, 1, 0usize, 0) != 0 {
            exe::screenmanager_set_screen_param(sm, 0, s, 0);
        }
        return -4;
    }
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        if id == 0xfffe {
            return 0;
        }
        if id == 0x19 {
            write_u32(G_MANAGER_STATUS_CONFIRMED_FLAG, 1);
            return -5;
        }
    }
    let r = exe::menubar_handle_command(item);
    if r != 0 { r } else { exe::menubar_handle_object_link(item) }
}

/// Adds a human manager and opens the Enter Name screen.
///
/// When the current human (`G_SCREEN_RECORDS_MANAGER` of the current slot) has no person yet, the first of the last 16
/// staff with `staff::CLASSIFICATION` 0 (none: no person; without staff, box 0x39b3 and no person) becomes his: the three
/// name texts emptied, born 1 January 1966 (`exe::date_set`: 1, 0, 1966, −1), age 35, the nation of the language
/// (`exe::langlib_get_language`: 1 United States, 2 France, 3 Germany, 4 Italy, 5 Spain, 6 Denmark, 7 Holland, 8
/// Norway, 9 Portugal, 10 Sweden, 11 Japan, 12 South Korea, otherwise England), no national or club job,
/// `staff::CLASSIFICATION` 5 and his HUMAN_MANAGER record reset (`exe::human_manager_reset_human_record`). Then Enter
/// Name opens (flag 1) with params 0 and 1 zeroed.
/// Original: `setup_add_human_manager` (0x820d40, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_add_human_manager() {
    covered!("tel11_2 add human manager");
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300;
    if read_usize(G_SCREEN_RECORDS_MANAGER + slot) == 0 {
        let staff = read_usize(G_STAFF_TABLE);
        let mut s = 0usize;
        if staff == 0 {
            setup_warning(0x39b3);
        } else {
            let count = read_i32(G_STAFF_COUNT) as isize;
            for k in 1..=16isize {
                let r = Staff((staff as isize + (count - k) * staff::SIZE as isize) as usize);
                if r.classification() == 0 {
                    s = r.addr();
                    break;
                }
            }
        }
        write_usize(G_SCREEN_RECORDS_MANAGER + slot, s);
        write_u8(read_usize(s + staff::FIRST_NAME), 0);
        write_u8(read_usize(s + staff::SECOND_NAME), 0);
        write_u8(read_usize(s + staff::COMMON_NAME), 0);
        exe::date_set(s + staff::DATE_OF_BIRTH, 1, 0, 0x7ae, -1);
        write_u16(s + staff::AGE, 0x23); // tel11_2 the new manager's age
        let key = match exe::langlib_get_language() {
            1 => KEY_NATION_UNITED_STATES,
            2 => KEY_NATION_FRANCE,
            3 => KEY_NATION_GERMANY,
            4 => KEY_NATION_ITALY,
            5 => KEY_NATION_SPAIN,
            6 => KEY_NATION_DENMARK,
            7 => KEY_NATION_HOLLAND,
            8 => KEY_NATION_NORWAY,
            9 => KEY_NATION_PORTUGAL,
            10 => KEY_NATION_SWEDEN,
            11 => KEY_NATION_JAPAN,
            12 => KEY_NATION_SOUTH_KOREA,
            _ => KEY_NATION_ENGLAND,
        };
        write_usize(s + staff::NATION, read_usize(G_NATION_TABLE) + read_i32(key) as isize as usize * nation::SIZE);
        write_u32(s + staff::NATIONAL_JOB, 0);
        write_u32(s + staff::CLUB_JOB, 0);
        write_u8(s + staff::CLASSIFICATION, 5);
        exe::human_manager_reset_human_record(read_usize(G_HUMAN_MANAGER), s);
    }
    setup_open_screen(0x820f50, 0x821520, 1, 2);
}

/// Comparer of `setup_build_network_play_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_setup_build_network_play_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_NETWORK_PLAY_SCREEN);
    compare_by_display_list_with("Network Play screen", &[], || o(), || setup_build_network_play_screen());
}

/// Comparer of `setup_handle_network_play_input`: only the original runs (every answer opens a screen or goes to the
/// menu bar).
pub(crate) unsafe extern "C" fn compare_setup_handle_network_play_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_NETWORK_PLAY_INPUT)(item)
}

/// Comparer of `setup_build_enter_network_game_name_screen`: side by side over the display list (with the two network
/// flags) when the network object already exists; without it only the original runs (it starts the network session).
pub(crate) unsafe extern "C" fn compare_setup_build_enter_network_game_name_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_ENTER_NETWORK_GAME_NAME_SCREEN);
    if read_usize(G_SCREEN_MANAGER_NETWORK) == 0 {
        return o();
    }
    compare_by_display_list_with("Enter Network Game Name screen", &[(G_NETWORK_SETUP_HOST_FLAG, 8)], || o(), || setup_build_enter_network_game_name_screen());
}

/// Comparer of `setup_handle_enter_network_game_name_input`: only the original runs (the answers load files, create the
/// network game, open screens and dialogs or end the session).
pub(crate) unsafe extern "C" fn compare_setup_handle_enter_network_game_name_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_ENTER_NETWORK_GAME_NAME_INPUT)(item)
}

/// Comparer of `setup_build_select_network_game_screen`: side by side over the display list and the copy of the game
/// list when the network object exists; without it only the original runs (it starts the session and sends the request).
pub(crate) unsafe extern "C" fn compare_setup_build_select_network_game_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_SELECT_NETWORK_GAME_SCREEN);
    if read_usize(G_SCREEN_MANAGER_NETWORK) == 0 {
        return o();
    }
    compare_by_display_list_with("Select Network Game screen", &[(G_NETWORK_GAME_LIST_COPY, 500)], || o(), || setup_build_select_network_game_screen());
}

/// Comparer of `setup_handle_select_network_game_input`: the name entry and a game of the list (ids 0x16 and 0x17, with
/// an item) run side by side over the game name; Refresh, next, back and the menu bar only in the original (network).
pub(crate) unsafe extern "C" fn compare_setup_handle_select_network_game_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_SELECT_NETWORK_GAME_INPUT);
    let id = setup_clicked_id(item as i16);
    if !(id == 0x16 || (id == 0x17 && item as i16 != -1)) {
        return o(item);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(G_NETWORK_GAME_NAME, 0x34)], || o(item), || setup_handle_select_network_game_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Select Network Game answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `setup_open_manager_status_screen`: only the original runs (it opens the screen).
pub(crate) unsafe extern "C" fn compare_setup_open_manager_status_screen() {
    original_fn::<unsafe extern "C" fn()>(IDX_SETUP_OPEN_MANAGER_STATUS_SCREEN)()
}

/// Comparer of `setup_build_manager_status_screen`: side by side over the display list and the screen's params; without
/// staff only the original runs (the error box does not run twice).
pub(crate) unsafe extern "C" fn compare_setup_build_manager_status_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_MANAGER_STATUS_SCREEN);
    if read_usize(G_STAFF_TABLE) == 0 {
        return o();
    }
    compare_by_display_list_with("Manager Status screen", &[], || o(), || setup_build_manager_status_screen());
}

/// Comparer of `setup_handle_manager_status_input`: back and Finish (ids -2 and 0x19, which also answer -11 when the
/// screen is out of date) run side by side over the Finish flag; a manager's row and the menu bar only in the original
/// (they open screens); without staff only the original (error box).
pub(crate) unsafe extern "C" fn compare_setup_handle_manager_status_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_MANAGER_STATUS_INPUT);
    let id = setup_clicked_id(item as i16);
    if read_usize(G_STAFF_TABLE) == 0 || !(id == 0x19 || id == 0xfffe) {
        return o(item);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(G_MANAGER_STATUS_CONFIRMED_FLAG, 4)], || o(item), || setup_handle_manager_status_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Manager Status answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `setup_add_human_manager`: only the original runs (it takes a staff record and opens Enter Name).
pub(crate) unsafe extern "C" fn compare_setup_add_human_manager() {
    original_fn::<unsafe extern "C" fn()>(IDX_SETUP_ADD_HUMAN_MANAGER)()
}

/// Builds the Enter Name screen: the four entry rows (first name, second name, password, re-type) and Cancel/Next.
///
/// Param 0 says which entry has the focus (0..3; the focused one gets flag 0x10). The current human
/// (`G_SCREEN_RECORDS_MANAGER` of the slot) gives the name texts. A random background and the title bar ("Enter Name").
/// Each row is an area (0x96 to 0x2e4, 0x3c high from 0xd9: columns 8, 1, 0xc; style 0x22 on dark blue) with the
/// label, ":" and the entry (`Display::add_item`): first name (`staff::FIRST_NAME` text, 0x1a characters, id 0x4d, type
/// 0x80008), second name (`staff::SECOND_NAME`, id 0x4e), password (`HUMAN_MANAGER::get_password` of the current one,
/// 0x32 characters, flags 0x81, id 0x1a, type 8) and re-type (grey label when there is no password; the entry,
/// `get_password_entry`, only with a password, no id). The bottom area (0x64, 0x22b to 0x316, 0x24e) has Cancel (0x1b)
/// and Next (−3).
/// Original: `setup_build_enter_name_screen` (0x820f50, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_enter_name_screen() {
    covered!("tel11_3 build enter name");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let password: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD);
    let typed: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let sm = SCREEN_MANAGER;
    let hm = || read_usize(G_HUMAN_MANAGER);
    let mut columns: [u8; 3] = [8, 1, 0xc];
    let mut bottom_columns: [u8; 2] = [3, 1];
    let focus = exe::screenmanager_get_screen_param(sm, 0) as u8;
    exe::screenmanager_get_screen_param(sm, 1);
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300));
    exe::background_set_random_picture();
    setup_screen_header(0xa80730); // "Enter Name"
    let (grey, pale, dark_blue) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_DARK_BLUE));
    let focused = |k: u8| if focus == k { 0x10usize } else { 0 };
    // one row: its area, the label and ":" in `colour`
    let mut row = |top: usize, caption: usize, colour: usize| {
        let a = area(current_human_display(), 0x96, top, 0x2e4, top + 0x3c, 3, columns.as_mut_ptr() as usize, 1, 0, 0x22, dark_blue, usize::MAX);
        text(SETUP_TEXT, caption);
        label(sm, a, 0, 0, 1, 0, 1, 3, colour, SETUP_TEXT);
        label(sm, a, 1, 0, 1, 0, 0xc, 3, colour, 0x989adc); // ":"
        a
    };
    let a = row(0xd9, 0xa80720, pale); // "   First Name"
    item(current_human_display(), 0x80008 | focused(0), 0, 0, 0, 0, 2, 0, 1, 0, 0, 1, 3, pale, manager.first_name(), 0x1a, 0x4d, 0, a);
    let a = row(0x117, 0xa80710, pale); // "   Second Name"
    item(current_human_display(), 0x80008 | focused(1), 0, 0, 0, 0, 2, 0, 1, 0, 0, 1, 3, pale, manager.second_name(), 0x1a, 0x4e, 0, a);
    let a = row(0x155, 0xa806f8, pale); // "   Password (Optional)"
    item(current_human_display(), 8 | focused(2), 0, 0, 0, 0, 2, 0, 1, 0, 0, 0x81, 3, pale, password(hm(), 0), 0x32, 0x1a, 0, a);
    let a = row(0x193, 0xa806ec, if read_u8(password(hm(), 0)) != 0 { pale } else { grey }); // "   Re-Type"
    if read_u8(password(hm(), 0)) != 0 {
        item(current_human_display(), 8 | focused(3), 0, 0, 0, 0, 2, 0, 1, 0, 0, 0x81, 3, pale, typed(hm(), 0), 0x32, 0, 0, a);
    }
    let bottom = area(current_human_display(), 0x64, 0x22b, 0x316, 0x24e, 2, bottom_columns.as_mut_ptr() as usize, 1, 0, 1, 0, usize::MAX);
    text(SETUP_TEXT, 0x98a1a0); // "Cancel"
    button(sm, bottom, 0, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x1b, 0);
    text(SETUP_TEXT, 0x9880a0); // "Next"
    button(sm, bottom, 1, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, usize::MAX - 2, 0);
}

/// The Enter Name screen's answer to item `item`.
///
/// No item: 0. The password entry (0x1a) empties the re-typed password and, with a password, moves the focus to the
/// re-type (param 0 = 3); −11. Cancel (0x1b) frees the staff record (`staff::CLASSIFICATION` 0); with more than one
/// record in the slot (`G_SCREEN_MANAGER_SLOT_RECORD_COUNTS`) the record is removed (`ScreenManager::remove_manager_record`,
/// 0), otherwise the slot loses its manager (−9). Next (−3): with both names, a password must match its re-typing (when
/// not, both are emptied, the focus goes to the password, param 0 = 2, and the "Passwords Don't Match" box; −4), then
/// Select Nationality opens with params 0, 1 and 3 zeroed and 2 = 1 (−4); a missing name gives the "Names Are Blank",
/// "First Name Is Blank" or "Second Name Is Blank" box (−4). Any other goes to the menu bar (0 when it says nothing).
/// Original: `setup_handle_enter_name_input` (0x821520, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_enter_name_input(item: u32) -> i32 {
    covered!("tel11_3 enter name input");
    let sm = SCREEN_MANAGER;
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let typed: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY);
    let hm = || read_usize(G_HUMAN_MANAGER);
    let item16 = item as i16;
    if item16 == -1 {
        return 0;
    }
    let slot = || read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize;
    let manager_slot = || G_SCREEN_RECORDS_MANAGER + slot() * 0x300;
    match setup_clicked_id(item16) as i16 as i32 {
        0x1a => {
            write_u8(typed(hm(), 0), 0);
            if read_u8(exe::human_manager_get_password(hm(), 0)) != 0 {
                exe::screenmanager_set_screen_param(sm, 0, 3usize, 0);
            }
            -11
        }
        0x1b => {
            write_u8(read_usize(manager_slot()) + staff::CLASSIFICATION, 0);
            if read_i16(G_SCREEN_MANAGER_SLOT_RECORD_COUNTS + slot() * 2) > 1 {
                exe::screenmanager_remove_manager_record(sm, read_usize(manager_slot()), -1);
                return 0;
            }
            write_usize(manager_slot(), 0);
            -9
        }
        -3 => {
            let manager = Staff(read_usize(manager_slot()));
            let (first, second) = (read_u8(manager.first_name()), read_u8(manager.second_name()));
            let (pale, dark_red) = (setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_DARK_RED));
            if first != 0 && second != 0 {
                if read_u8(exe::human_manager_get_password(hm(), 0)) != 0 && setup_texts_differ(exe::human_manager_get_password(hm(), 0), typed(hm(), 0)) {
                    write_u8(exe::human_manager_get_password(hm(), 0), 0);
                    write_u8(typed(hm(), 0), 0);
                    exe::screenmanager_set_screen_param(sm, 0, 2usize, 0);
                    setup_format_title();
                    text(SETUP_TEXT, 0xa80808); // "The password and re-typed password do not match."
                    setup_confirm_box(0xa807f0, pale, dark_red, 0); // "Passwords Don't Match"
                    return -4;
                }
                if exe::screenmanager_open_screen(sm, 0x821900usize, 0x822040usize, 0, 0usize, 0) != 0 {
                    for (k, v) in [(0u32, 0usize), (1, 0), (2, 1), (3, 0)] {
                        exe::screenmanager_set_screen_param(sm, k, v, 0);
                    }
                }
                return -4;
            }
            setup_format_title();
            let (body, heading) = if first == 0 && second == 0 {
                (0xa807c0, 0xa807b0) // "The first and second names entered are blank.", "Names Are Blank"
            } else if first == 0 {
                (0xa8078c, 0xa80778) // "The first name entered is blank.", "First Name Is Blank"
            } else {
                (0xa80754, 0xa8073c) // "The second name entered is blank.", "Second Name Is Blank"
            };
            text(SETUP_TEXT, body);
            setup_confirm_box(heading, pale, dark_red, 0); // tel11_3 the blank name box
            -4
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 {
                return r;
            }
            exe::menubar_handle_object_link(item)
        }
    }
}

/// Builds the Select Nationality screen: the All/Major Nations filter and the list of the nations for the new manager.
///
/// Param 3 is the filter (nonzero: all nations), the current human gives the chosen nation (`staff::NATION`). A random
/// background and the title bar ("Select Nationality"); the Filter button (0x28f, 0x91 to 0x30c, 0xa5) with its popup
/// area holding " All Nations" (id 0x1c) and " Major Nations" (id 0x1d), the chosen one marked (0x2002, no id). The list
/// area (0x6e, 0xaa to 0x30c, 0x217; columns 6, 1, 6, 1; as many rows as font 3 lines fit in 0x15d) gets the nations
/// with a continent (`nation::CONTINENT`): every nation, or the 33 major ones of `SETUP_MAJOR_NATIONS`; at most 250
/// (more: box 0x107b), sorted (`qsort` with 0x839530). Two per row: the name ("  %s" of `nation::NATIONALITY`, or of
/// `nation_get_nationality_adjective`(…, 0xc) in language 7) with its first letter in upper case (`toupper`), a button
/// id 0x1e with the nation as value (style 0x801 for the chosen one, whose row is remembered), and the continent's
/// name in yellow. With param 2 set the list scrolls to put the chosen row in the middle (param 0 = row − rows / 2, param
/// 2 cleared); the list scrolls to param 0 and its index goes to param 1; the back and next buttons.
/// Original: `setup_build_select_nationality_screen` (0x821900, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_select_nationality_screen() {
    // the 1920x1080 exe rewrites this list's layout (at 0x821d1d, `mov [esp+..], al` there): in it the
    // exe's own routine runs (EXE in rotinas.py; the signature table still detours it)
    if (0x821d1d as *const u8).read() != 0x88 {
        return original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_SELECT_NATIONALITY_SCREEN)();
    }
    covered!("tel11_3 build select nationality");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let toupper: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_CRT_TOUPPER);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let screen_button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let sm = SCREEN_MANAGER;
    let mut columns: [u8; 4] = [6, 1, 6, 1];
    let filter = exe::screenmanager_get_screen_param(sm, 3);
    let mut chosen_row = filter; // the original keeps the chosen row in the filter's local
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300));
    exe::background_set_random_picture();
    setup_screen_header(0xa8084c); // "Select Nationality"
    let (grey, pale, green, yellow, black) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY),
        setup_colour(0xae31aa), setup_colour(G_GUI_COLOUR_YELLOW), setup_colour(G_GUI_COLOUR_BLACK));
    text(SETUP_TEXT, 0x98ebf8); // "Filter"
    let filter_button = screen_button(sm, 0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0, 0);
    let popup = area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, green, filter_button);
    // a popup entry: marked (no id) when it is the current filter, otherwise clickable with `id`
    let entry = |marked: bool, id: usize| {
        let (kind, id) = if marked { (0x2002, 0) } else { (2, id) };
        item(current_human_display(), kind, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, id, 0, popup);
    };
    text(SETUP_TEXT, 0x990da4); // " All Nations"
    entry(filter != 0, 0x1c);
    text(SETUP_TEXT, 0xa8083c); // " Major Nations"
    entry(filter == 0, 0x1d);
    let rows = (0x15d / exe::font_get_height(3)) as u8;
    let list = area(current_human_display(), 0x6e, 0xaa, 0x30c, 0x217, 4, columns.as_mut_ptr() as usize, rows as usize, 0, 2, 0, usize::MAX);
    let mut nations = [0usize; 0xfa];
    let mut count = 0usize;
    let table = read_usize(G_NATION_TABLE);
    let total = read_i32(G_NATION_COUNT);
    let mut k = 0;
    while k < total {
        let n = if filter == 0 {
            if k as usize >= SETUP_MAJOR_NATIONS.len() {
                break; // the original's list ends with −1
            }
            table + read_i32(SETUP_MAJOR_NATIONS[k as usize]) as isize as usize * nation::SIZE
        } else {
            table + k as usize * nation::SIZE
        };
        if read_usize(n + nation::CONTINENT) != 0 {
            if count >= 0xfa {
                setup_warning(0x107b);
                break;
            }
            nations[count] = n;
            count += 1;
        }
        k += 1;
    }
    if count != 0 {
        exe::crt_qsort(nations.as_mut_ptr() as usize, count as u32, 4, SETUP_NATIONALITY_COMPARE);
    }
    let (mut col, mut row) = (0usize, 0usize);
    for &n in &nations[..count] {
        if exe::langlib_get_language() == 7 {
            let mut adjective = [0u8; 0x100];
            exe::nation_get_nationality_adjective(n, adjective.as_mut_ptr() as usize, 0x64, 0xc);
            format(SETUP_TEXT, 0x987ff0, adjective.as_ptr()); // "  %s"
        } else {
            format(SETUP_TEXT, 0x987ff0, n + nation::NATIONALITY);
        }
        write_u8(G_TEXT_BUFFER_CHAR_2, toupper(read_i8(G_TEXT_BUFFER_CHAR_2) as i32) as u8);
        if n == manager.nation().addr() {
            button(sm, list, col, row, 0x801, 0, 1, 3, pale, SETUP_TEXT, 0x1e, n);
            chosen_row = row;
        } else {
            button(sm, list, col, row, 1, 0, 1, 3, pale, SETUP_TEXT, 0x1e, n);
        }
        label(sm, list, col + 1, row, 1, 0, 0xc, 2, yellow, read_usize(n + nation::CONTINENT) + CONTINENT_SHORT_NAME);
        col += 2;
        if col > 3 {
            col = 0;
            row += 1;
        }
    }
    if exe::screenmanager_get_screen_param(sm, 2) != 0 {
        exe::screenmanager_set_screen_param(sm, 0, chosen_row.wrapping_sub((rows >> 1) as usize), 0); // tel11_3 the chosen row in the middle
        exe::screenmanager_set_screen_param(sm, 2, 0usize, 0);
    }
    let scroll = exe::screenmanager_get_screen_param(sm, 0);
    if list as i16 != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + (list as i16 as isize * 0xbf1) as usize, scroll);
    }
    exe::screenmanager_set_screen_param(sm, 1, list as i16 as isize as usize, 0);
    let back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, 1);
}

/// The Select Nationality screen's answer to item `item`: a real item keeps the list's scroll row in param 0; All
/// Nations (0x1c) and Major Nations (0x1d) set the filter (param 3 = 1 or 0), a nation (0x1e) becomes the current
/// human's (`staff::NATION`; none without an item), all −11; Next (−3) opens Select Team with param 0 = the main nation
/// `G_MAIN_NATION` and params 1 and 2 zeroed (−4); any other goes to the menu bar.
/// Original: `setup_handle_select_nationality_input` (0x822040, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_select_nationality_input(item: u32) -> i32 {
    covered!("tel11_3 select nationality input");
    let sm = SCREEN_MANAGER;
    let item16 = item as i16;
    if item16 != -1 {
        let row = exe::screenmanager_get_area_scroll_row(sm, exe::screenmanager_get_screen_param(sm, 1));
        exe::screenmanager_set_screen_param(sm, 0, row, 0);
    }
    match setup_clicked_id(item16) {
        0x1c => {
            exe::screenmanager_set_screen_param(sm, 3, 1usize, 0);
            -11
        }
        0x1d => {
            exe::screenmanager_set_screen_param(sm, 3, 0usize, 0);
            -11
        }
        0x1e => {
            let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300));
            write_usize(manager.addr() + staff::NATION, setup_clicked_value(item16));
            -11
        }
        0xfffd => {
            if exe::screenmanager_open_screen(sm, 0x822260usize, 0x8228f0usize, 0, 0usize, 0) != 0 {
                exe::screenmanager_set_screen_param(sm, 0, read_usize(G_MAIN_NATION), 0);
                exe::screenmanager_set_screen_param(sm, 1, 0usize, 0);
                exe::screenmanager_set_screen_param(sm, 2, 0usize, 0);
            }
            -4
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Builds the Select Team screen: the nation chooser (with more than one league) and the list of the clubs the new
/// manager can take.
///
/// Param 0 is the nation shown. A random background and the title bar ("Select Team"). With more than one selected
/// league (`G_SELECTED_NATION_COUNT`): the Nation button (0x6e, 0x91 to 0xeb, 0xa5) with a popup area of the selected
/// nations (`key_nation_sorted_order`(0)), split in two parts by `NATION_MENU_SPLIT` when it says so (each part a
/// header item, its `format_header`, with its own sub-area 0x30), each nation an item id 0x1f with the nation as value;
/// the list then starts at 0xaa instead of 0x91. The list area (0x6e, top to 0x30c, 0x217; columns 5, 1, 1, 5, 1, 1;
/// as many rows as font 3 lines fit up to 0x207). The clubs (`G_CLUB_TABLE`; a vector of `G_CLUB_COUNT` pointers from
/// `malloc`, box 0x1187 when it fails) of that nation (any nation with one league) that can be managed
/// (`club_can_be_managed`) and are not national sides (`exe::database_is_national_team`), sorted (`qsort` with
/// 0x540080); two per row: the name ("  %s", `exe::club_get_national_team_name`) as a button id 0x7d5 (yellow when a
/// human runs it, `exe::human_manager_is_human_club`), then for a national side the continent's name, otherwise the
/// nation's three letters (a button 0x7d5 to the national side, `exe::database_get_national_team`) and the division's
/// abbreviation in orange (a button 0x7d0 to it). The vector is freed, the list scrolls to param 1 and its index goes to
/// param 2; the back button.
/// Original: `setup_build_select_team_screen` (0x822260, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_build_select_team_screen() {
    // the 1920x1080 exe rewrites this list's layout (its own column table, 0x82259d, `mov [esp+..], al` there): in it the
    // exe's own routine runs (EXE in rotinas.py; the signature table still detours it)
    if (0x82259d as *const u8).read() != 0x88 {
        return original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_SELECT_TEAM_SCREEN)();
    }
    covered!("tel11_3 build select team");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let split_new: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(FN_NATION_MENU_SPLIT_CONSTRUCT);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let screen_button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let sm = SCREEN_MANAGER;
    let mut columns: [u8; 6] = [5, 1, 1, 5, 1, 1];
    let shown = exe::screenmanager_get_screen_param(sm, 0);
    exe::background_set_random_picture();
    setup_screen_header(0xa80860); // "Select Team"
    let (grey, pale, green, yellow, black, orange) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY),
        setup_colour(0xae31aa), setup_colour(G_GUI_COLOUR_YELLOW), setup_colour(G_GUI_COLOUR_BLACK), setup_colour(G_GUI_COLOUR_ORANGE));
    let mut top = 0x91;
    if read_i32(G_SELECTED_NATION_COUNT) > 1 {
        text(SETUP_TEXT, 0x9ca010); // "Nation"
        let nation_button = screen_button(sm, 0x6e, 0x91, 0xeb, 0xa5, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0, 0);
        let popup = area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, green, nation_button);
        let order = exe::key_nation_sorted_order(0);
        let mut split = [0u8; 0x40];
        let sp = split.as_mut_ptr() as usize;
        split_new(sp, 1, order, 0);
        let parts = exe::nation_menu_split_part_count(sp) as i8;
        let nation_item = |n: usize, under: usize| {
            item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, n + 4, 0, 0x1f, n, under);
        };
        let selected = |k: usize| {
            let n = read_usize(setup_league_entry(order, k));
            if n != 0 && read_u8(n + nation::LEAGUE_SELECTED) & 2 != 0 { n } else { 0 }
        };
        if parts > 1 {
            let mut part = 0i8;
            while part < parts {
                exe::nation_menu_split_format_header(sp, SETUP_TEXT, 0x7d0, part as u8 as u32);
                let header = item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, 0, 0, popup);
                let sub = area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, green, header);
                for k in 0..0x23 {
                    let n = selected(k);
                    if n != 0 && exe::nation_menu_split_in_second_part(sp, n) == part as u8 {
                        nation_item(n, sub);
                    }
                }
                part += 1;
            }
        } else {
            for k in 0..0x23 {
                let n = selected(k);
                if n != 0 {
                    nation_item(n, popup);
                }
            }
        }
        top = 0xaa;
    }
    let rows = ((0x207 - top) / exe::font_get_height(3)) as u8;
    let list = area(current_human_display(), 0x6e, top as usize, 0x30c, 0x217, 6, columns.as_mut_ptr() as usize, rows as usize, 0, 2, 0, usize::MAX);
    let clubs = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if clubs == 0 {
        setup_warning(0x1187);
        return;
    }
    let mut count = 0usize;
    let mut k = 0;
    while k < read_i32(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + k as usize * club::SIZE;
        if (read_i32(G_SELECTED_NATION_COUNT) <= 1 || read_usize(c + club::NATION) == shown)
            && exe::club_can_be_managed(c) != 0
            && exe::database_is_national_team(c) == 0
        {
            write_usize(clubs + count * 4, c);
            count += 1;
        }
        k += 1;
    }
    exe::crt_qsort(clubs, count as u32, 4, SETUP_CLUB_COMPARE);
    let (mut col, mut row) = (0usize, 0usize);
    for j in 0..count {
        let c = read_usize(clubs + j * 4);
        let mut name = [0u8; 0xc8];
        exe::club_get_national_team_name(c, 0usize, name.as_mut_ptr() as usize, 0xc8, 1, 1);
        format(SETUP_TEXT, 0x987ff0, name.as_ptr()); // "  %s"
        let human = exe::human_manager_is_human_club(c, 1, 1, 0, 0) != 0;
        button(sm, list, col, row, 1, 0, 1, 3, if human { yellow } else { pale }, SETUP_TEXT, 0x7d5, c);
        let n = read_usize(c + club::NATION);
        if exe::database_is_national_team(c) != 0 {
            label(sm, list, col + 1, row, 1, 0, 0xc, 2, yellow, read_usize(n + nation::CONTINENT) + CONTINENT_SHORT_NAME);
        } else {
            let side = exe::database_get_national_team(n);
            button(sm, list, col + 1, row, 1, 0, 0xc, 2, yellow, n + nation::THREE_LETTER_NAME, 0x7d5, side);
            let division = read_usize(c + club::DIVISION);
            button(sm, list, col + 2, row, 1, 0, 0xc, 2, orange, division + COMPETITION_ABBREVIATION, 0x7d0, division);
        }
        col += 3;
        if col > 3 {
            col = 0;
            row += 1;
        }
    }
    exe::crt_free(clubs);
    let scroll = exe::screenmanager_get_screen_param(sm, 1);
    if list as i16 != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + (list as i16 as isize * 0xbf1) as usize, scroll);
    }
    exe::screenmanager_set_screen_param(sm, 2, list as i16 as isize as usize, 0);
    let back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, 0); // tel11_3 no next on Select Team
}

/// The Select Team screen's answer to item `item`: a real item keeps the list's scroll row in param 1; a nation of the
/// chooser (0x1f, its value; none without an item) different from the one shown becomes param 0 with the scroll zeroed
/// (−11); Next (−3) returns 0; any other (a club: the take control box) goes to the menu bar.
/// Original: `select_team_screen_handle_input` (0x8228f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn select_team_screen_handle_input(item: u32) -> i32 {
    covered!("tel11_3 select team input");
    let sm = SCREEN_MANAGER;
    let item16 = item as i16;
    if item16 != -1 {
        let row = exe::screenmanager_get_area_scroll_row(sm, exe::screenmanager_get_screen_param(sm, 2));
        exe::screenmanager_set_screen_param(sm, 1, row, 0);
    }
    let id = setup_clicked_id(item16);
    if id == 0x1f {
        let n = setup_clicked_value(item16);
        if n != exe::screenmanager_get_screen_param(sm, 0) {
            exe::screenmanager_set_screen_param(sm, 0, n, 0);
            exe::screenmanager_set_screen_param(sm, 1, 0usize, 0);
            return -11;
        }
    }
    if id == 0xfffd {
        return 0;
    }
    let r = exe::menubar_handle_command(item);
    if r != 0 { r } else { exe::menubar_handle_object_link(item) }
}

/// Comparer of `setup_build_enter_name_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_setup_build_enter_name_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_ENTER_NAME_SCREEN);
    compare_by_display_list_with("Enter Name screen", &[], || o(), || setup_build_enter_name_screen());
}

/// Comparer of `setup_handle_enter_name_input`: the password entry (0x1a) runs side by side over the screen's params
/// and the typed password; Cancel, Next and the menu bar only in the original (they free records, open boxes and
/// screens).
pub(crate) unsafe extern "C" fn compare_setup_handle_enter_name_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_ENTER_NAME_INPUT);
    let typed: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY);
    if item as i16 == -1 || setup_clicked_id(item as i16) != 0x1a {
        return o(item);
    }
    let mut regions = screen_param_regions();
    regions.push((typed(read_usize(G_HUMAN_MANAGER), 0), 1));
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || setup_handle_enter_name_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Enter Name answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `setup_build_select_nationality_screen`: side by side over the display list and params (without the
/// error box of more than 250 nations, which does not happen with the game's database).
pub(crate) unsafe extern "C" fn compare_setup_build_select_nationality_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_SELECT_NATIONALITY_SCREEN);
    compare_by_display_list_with("Select Nationality screen", &[], || o(), || setup_build_select_nationality_screen());
}

/// Comparer of `setup_handle_select_nationality_input`: the filter and nation buttons (0x1c, 0x1d, 0x1e) run side by
/// side over the screen's params and the current human's nation; Next and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_setup_handle_select_nationality_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_SELECT_NATIONALITY_INPUT);
    if !matches!(setup_clicked_id(item as i16), 0x1c | 0x1d | 0x1e) {
        return o(item);
    }
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300));
    let mut regions = vec![(SCREEN_MANAGER, 0x3840)];
    regions.extend(screen_param_regions());
    if !manager.is_null() {
        regions.push((manager.addr() + staff::NATION, 4));
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || setup_handle_select_nationality_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Select Nationality answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `setup_build_select_team_screen`: side by side over the display list (the club vector is born and freed
/// inside); with the club vector's allocation the heap differs between the two runs, which the comparison does not see.
pub(crate) unsafe extern "C" fn compare_setup_build_select_team_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SETUP_BUILD_SELECT_TEAM_SCREEN);
    compare_by_display_list_with("Select Team screen", &[], || o(), || setup_build_select_team_screen());
}

/// Comparer of `select_team_screen_handle_input`: the nation chooser (0x1f, with a different nation) runs side by side
/// over the screen's params; Next and the menu bar (a club opens the take control box) only in the original.
pub(crate) unsafe extern "C" fn compare_select_team_screen_handle_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SELECT_TEAM_SCREEN_HANDLE_INPUT);
    if setup_clicked_id(item as i16) != 0x1f || setup_clicked_value(item as i16) == exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0) {
        return o(item);
    }
    let mut regions = vec![(SCREEN_MANAGER, 0x3840)];
    regions.extend(screen_param_regions());
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || select_team_screen_handle_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Select Team answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Opens the take control box for `club` (`take_control_confirm_build`, `take_control_confirm_handle_input`) with the
/// club in param 0.
/// Original: `open_take_control_confirm` (0x822a60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_take_control_confirm(club: usize) {
    covered!("tel11_4 open take control");
    if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x822aa0usize, 0x822c50usize, 0, 0usize, 0) != 0 {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, club, 0);
    }
}

/// Builds the take control box: the club's picture (`exe::background_set_club_picture`), the current manager's name as
/// title, "Please Confirm", the question "Take control of {}<club>{} ?" (`club::NAME` with its `club::GENDER_NAME`) and
/// No (0x20) / Yes (0x21).
/// Original: `take_control_confirm_build` (0x822aa0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn take_control_confirm_build() {
    covered!("tel11_4 build take control");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club = Club(exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0));
    let manager = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    exe::background_set_club_picture(club, 0, 0);
    setup_box_header(manager, 0x9909b8); // "Please Confirm"
    let g = club.gender_name() as i32;
    text(SETUP_TEXT, 0xa8086c, g, g, club.addr() + club::NAME); // "Take control of {}<%s - club name>{} ?"
    setup_box_question();
    setup_box_buttons(0x98e8cc, 0x20, 0x98e8d0, 0x21); // "No", "Yes"
}

/// The take control box's answer to item `item`.
///
/// After its own "Already Controlled" box (`G_PENDING_COMMAND_CODE` 0x3b): cleared, −2. No (0x20): −2. Yes (0x21): a
/// club a human already runs (`exe::human_manager_is_human_club`) gives the "Already Controlled" box with his name
/// (code 0x3b); a club playing now (`MATCH_DAY::is_club_playing`) the "Playing In Match" box; both −4, red with pale grey
/// text. Otherwise the current manager becomes human (`staff::CLASSIFICATION` 1), is appointed
/// (`exe::appoint_human_manager_to_club`), gets his attribute map when Attribute Masking is on
/// (`FOG_OF_WAR::add_manager_map`), `G_PENDING_COMMAND_ARG` = 0, −10. Any other goes to the menu bar.
/// Original: `take_control_confirm_handle_input` (0x822c50, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn take_control_confirm_handle_input(item: u32) -> i32 {
    covered!("tel11_4 take control input");
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let playing: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_MATCH_DAY_IS_CLUB_PLAYING);
    let fog: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_ADD_MANAGER_MAP);
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 0x3b {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        return -2;
    }
    match setup_clicked_id(item as i16) {
        0x20 => -2,
        0x21 => {
            let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300));
            let club = Club(exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0));
            let g = (club.short_gender_name() as i8) as i32;
            if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
                let mut name = [0u8; 0xc8];
                exe::database_format_staff_name(read_usize(club.addr() + CLUB_HUMAN_MANAGER), name.as_mut_ptr() as usize, 0xc8, 1);
                let (red, pale) = (setup_colour(G_GUI_COLOUR_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
                setup_format_title();
                text(SETUP_TEXT, 0xa80914, g, g, name.as_ptr(), club.addr() + club::SHORT_NAME); // "<%s - manager> has already taken control of {}<%s - club>{}."
                setup_confirm_box(0xa80900, pale, red, 0x3b); // "Already Controlled"
                return -4;
            }
            let match_day = read_usize(G_MATCH_DAY);
            if match_day != 0 && playing(match_day, club.addr(), 0) != 0 {
                let (red, pale) = (setup_colour(G_GUI_COLOUR_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
                setup_format_title();
                text(SETUP_TEXT, 0xa808a8, g, g, club.addr() + club::SHORT_NAME); // "{}<%s - club>{} are currently playing in a match. ..."
                setup_confirm_box(0xa80894, pale, red, 0); // "Playing In Match"
                return -4;
            }
            manager.set_classification(1); // tel11_4 the manager becomes human
            exe::appoint_human_manager_to_club(manager, club);
            if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
                fog(read_usize(G_FOG_OF_WAR), manager.addr());
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
            -10
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Builds the Enter Password box of the manager in param 0: a random background, his name as title, "Enter Password",
/// the password entry (`Display::add_item` type 0x18 at 0x96, 0x136 to 0x2e4, 0x172, style 0x22, dark blue, flags 0x8c,
/// font 4, his `get_password_entry`, 0x32 characters, id 0x22), Cancel (0x23) and Ok (0x24, a button only when something
/// is typed, otherwise text).
/// Original: `enter_password_box_build` (0x822ec0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn enter_password_box_build() {
    covered!("tel11_4 build enter password");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let typed: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let sm = SCREEN_MANAGER;
    let mut columns: [u8; 2] = [3, 1];
    let manager = exe::screenmanager_get_screen_param(sm, 0);
    exe::background_set_random_picture();
    setup_box_header(manager, 0xa80954); // "Enter Password"
    let (grey, pale) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY));
    let entry = typed(read_usize(G_HUMAN_MANAGER), manager);
    item(current_human_display(), 0x18, 0x96, 0x136, 0x2e4, 0x172, 0, 0, 0x22, setup_colour(G_GUI_COLOUR_DARK_BLUE), 0, 0x8c, 4, pale, entry, 0x32, 0x22, 0, usize::MAX);
    let a = area(current_human_display(), 0x64, 0x22b, 0x316, 0x24e, 2, columns.as_mut_ptr() as usize, 1, 0, 1, 0, usize::MAX);
    text(SETUP_TEXT, 0x98a1a0); // "Cancel"
    button(sm, a, 0, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x23, 0);
    text(SETUP_TEXT, 0x990f7c); // "Ok"
    if read_u8(typed(read_usize(G_HUMAN_MANAGER), manager)) != 0 {
        button(sm, a, 1, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x24, 0);
    } else {
        label(sm, a, 1, 0, 0x30, grey, 0x2c, 3, pale, SETUP_TEXT);
    }
}

/// The Enter Password box's answer to item `item`.
///
/// After its own box (code 0x3b): cleared, −9. The entry (0x22) −11; Cancel (0x23) −9. Ok (0x24): a manager who
/// already has a channel gives the "Already Controlled" box (−4); a typed password different from his and from the
/// master password "sportsinteractive" is emptied and gives the "Incorrect Password" box (pale grey on dark red, −4);
/// otherwise he resumes control (`setup_resume_manager`, −9). Any other goes to the menu bar.
/// Original: `enter_password_box_handle_input` (0x8230b0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn enter_password_box_handle_input(item: u32) -> i32 {
    covered!("tel11_4 enter password input");
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let typed: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY);
    let hm = || read_usize(G_HUMAN_MANAGER);
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 0x3b {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        return -9;
    }
    let manager = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    match setup_clicked_id(item as i16) {
        0x22 => -11,
        0x23 => -9,
        0x24 => {
            if setup_already_resumed(manager) {
                return -4;
            }
            let entry = typed(hm(), manager);
            if setup_texts_differ(exe::human_manager_get_password(hm(), manager), entry) && setup_texts_differ(typed(hm(), manager), SETUP_MASTER_PASSWORD) {
                write_u8(typed(hm(), manager), 0);
                setup_format_title();
                text(SETUP_TEXT, 0xa80978); // "The password entered is incorrect."
                setup_confirm_box(0xa80964, setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_DARK_RED), 0); // tel11_4 "Incorrect Password"
                return -4;
            }
            setup_resume_manager(manager)
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Builds the resume control box of the manager in param 0: his name as title, "Please Confirm" and the question with
/// what he runs: the national side and the club ("Resume control of {}<nation>{} and {}<club>{} ?", the national side's
/// picture; the gender bytes go club, club, nation, nation and the short names nation, club, as in the original), the
/// national side only, the club only (their pictures, `club::NAME`) or nothing ("Resume control ?", a random
/// background); No (0x25) / Yes (0x26).
/// Original: `resume_control_confirm_build` (0x823420, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn resume_control_confirm_build() {
    covered!("tel11_4 build resume control");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let manager = Staff(exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0));
    setup_box_header(manager.addr(), 0x9909b8); // "Please Confirm"
    let (national, club) = (manager.national_job().addr(), manager.club_job().addr());
    if national != 0 && club != 0 {
        exe::background_set_club_picture(national, 0, 0);
        let (gc, gn) = (read_i8(club + club::SHORT_GENDER_NAME) as i32, read_i8(national + club::SHORT_GENDER_NAME) as i32);
        text(SETUP_TEXT, 0xa80a3c, gc, gc, gn, gn, national + club::SHORT_NAME, club + club::SHORT_NAME);
    } else if national != 0 || club != 0 {
        let side = Club(if national != 0 { national } else { club });
        exe::background_set_club_picture(side, 0, 0);
        let g = side.gender_name() as i32;
        text(SETUP_TEXT, if national != 0 { 0xa80a14 } else { 0xa809f0 }, g, g, side.addr() + club::NAME);
    } else {
        exe::background_set_random_picture();
        text(SETUP_TEXT, 0xa809dc); // "Resume control ?"
    }
    setup_box_question();
    setup_box_buttons(0x98e8cc, 0x25, 0x98e8d0, 0x26); // "No", "Yes"
}

/// The resume control box's answer to item `item`: after its own box (code 0x3b) cleared and −9; No (0x25) −9; Yes
/// (0x26) the "Already Controlled" box when the manager already has a channel (−4), otherwise he resumes control
/// (`setup_resume_manager`, −9); any other goes to the menu bar.
/// Original: `resume_control_confirm_handle_input` (0x823660, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn resume_control_confirm_handle_input(item: u32) -> i32 {
    covered!("tel11_4 resume control input");
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 0x3b {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        return -9;
    }
    let manager = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    match setup_clicked_id(item as i16) {
        0x25 => -9,
        0x26 => {
            if setup_already_resumed(manager) {
                return -4;
            }
            setup_resume_manager(manager)
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Comparer of `open_take_control_confirm`: only the original runs (it opens the box).
pub(crate) unsafe extern "C" fn compare_open_take_control_confirm(club: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_TAKE_CONTROL_CONFIRM)(club)
}

/// Comparer of `take_control_confirm_build`: side by side over the display list (with the club's picture: the
/// generator is restored by `compare_by_display_list_with`).
pub(crate) unsafe extern "C" fn compare_take_control_confirm_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TAKE_CONTROL_CONFIRM_BUILD);
    compare_by_display_list_with("Take Control box", &[], || o(), || take_control_confirm_build());
}

/// Comparer of `take_control_confirm_handle_input`: only the original runs (it appoints the manager or opens boxes).
pub(crate) unsafe extern "C" fn compare_take_control_confirm_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TAKE_CONTROL_CONFIRM_HANDLE_INPUT)(item)
}

/// Comparer of `enter_password_box_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_enter_password_box_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_ENTER_PASSWORD_BOX_BUILD);
    compare_by_display_list_with("Enter Password box", &[], || o(), || enter_password_box_build());
}

/// Comparer of `enter_password_box_handle_input`: only the original runs (it resumes the manager or opens boxes).
pub(crate) unsafe extern "C" fn compare_enter_password_box_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_ENTER_PASSWORD_BOX_HANDLE_INPUT)(item)
}

/// Comparer of `resume_control_confirm_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_resume_control_confirm_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_RESUME_CONTROL_CONFIRM_BUILD);
    compare_by_display_list_with("Resume Control box", &[], || o(), || resume_control_confirm_build());
}

/// Comparer of `resume_control_confirm_handle_input`: only the original runs (it resumes the manager or opens a box).
pub(crate) unsafe extern "C" fn compare_resume_control_confirm_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_RESUME_CONTROL_CONFIRM_HANDLE_INPUT)(item)
}

/// Opens the Game Settings box (`game_settings_box_build`, `game_settings_box_handle_input`, flag 1) with a copy of the
/// live Config (`operator new` 0x31b bytes, `Config::Config` from `G_CONFIG`; 0 when the allocation fails) in param 0
/// and `origin` (0 first run, 1 Setup Game, 2/3 the side bar) in param 1.
/// Original: `open_game_settings_box` (0x823890, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_game_settings_box(origin: usize) {
    covered!("tel12_1 open game settings");
    let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONFIG_COPY);
    if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x823950usize, 0x8259a0usize, 1, 0usize, 0) == 0 {
        return;
    }
    let p = exe::crt_operator_new(CONFIG_SIZE as u32);
    let config = if p != 0 { copy(p, read_usize(G_CONFIG)) } else { 0 };
    exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, config, 0);
    exe::screenmanager_set_screen_param(SCREEN_MANAGER, 1, origin, 0);
}

/// Opens the Game Settings box from the side bar: origin 2 for item 0, 3 for any other.
/// Original: `open_game_settings_from_side_bar` (0x823930, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_game_settings_from_side_bar(item: u32) {
    covered!("tel12_1 settings from side bar");
    let open: unsafe extern "C" fn(usize) = core::mem::transmute(FN_OPEN_GAME_SETTINGS_BOX);
    open(if item == 0 { 2 } else { 3 });
}

/// The Game Settings box's answer to item `item` (param 0 the Config copy, param 1 the origin), by the clicked id:
/// 0x27 Reset (`Config::set_defaults` on the copy), −11; 0x28 Cancel: the live Config is applied again
/// (`Config::apply_and_save` on `G_CONFIG`) and the copy destroyed and freed, −9; 0x29 Ok: the copy is applied
/// (`Config::apply_and_save`) and destroyed and freed, −9; 0x2a a language (`Config::set_language` with the item's value;
/// from Setup, origin 0 or 1, with a database loaded, `select_default_league_by_language`), −11; 0x3d a font
/// (`Config::set_font`), 0x2b a currency (`Config::set_currency`), 0x2c an option bit (`Config::set_option`), all −11;
/// 0x2d an auto save interval (`Config::set_option`(0x2000) and `Config::set_auto_save_interval` with the item's action
/// data), −11; anything else 0.
/// Original: `game_settings_box_handle_input` (0x8259a0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_settings_box_handle_input(item: u32) -> i32 {
    covered!("tel12_1 game settings input");
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONFIG_DESTROY);
    let defaults: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_DEFAULTS);
    let apply: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONFIG_APPLY_AND_SAVE);
    let set_language: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_LANGUAGE);
    let set_font: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONFIG_SET_FONT);
    let set_currency: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_CURRENCY);
    let set_option: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_OPTION);
    let set_interval: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_AUTO_SAVE_INTERVAL);
    let default_league: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_SELECT_DEFAULT_LEAGUE_BY_LANGUAGE);
    let raw = item;
    let item = item as i16;
    let config = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    let origin = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 1);
    let free_copy = || {
        if config != 0 {
            destroy(config);
            exe::crt_operator_delete(config);
        }
    };
    match setup_clicked_id(item) {
        0x27 => {
            defaults(config);
            -11
        }
        0x28 => {
            apply(read_usize(G_CONFIG));
            free_copy();
            -9
        }
        0x29 => {
            apply(config);
            free_copy();
            -9
        }
        0x2a => {
            set_language(config, setup_clicked_value(item));
            if (origin == 0 || origin == 1) && read_usize(G_NATION_TABLE) != 0 {
                default_league();
            }
            -11
        }
        0x3d => {
            set_font(config, setup_clicked_value(item));
            -11
        }
        0x2b => {
            set_currency(config, setup_clicked_value(item));
            -11
        }
        0x2c => {
            set_option(config, setup_clicked_value(item));
            -11
        }
        0x2d => {
            set_option(config, 0x2000);
            set_interval(config, exe::screenmanager_get_widget_action_data(SCREEN_MANAGER, raw));
            -11
        }
        _ => 0,
    }
}

/// Comparer of `open_game_settings_box`: only the original runs (it pushes the box and allocates the copy).
pub(crate) unsafe extern "C" fn compare_open_game_settings_box(origin: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_GAME_SETTINGS_BOX)(origin)
}

/// Comparer of `open_game_settings_from_side_bar`: only the original runs (it opens the box).
pub(crate) unsafe extern "C" fn compare_open_game_settings_from_side_bar(item: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(IDX_OPEN_GAME_SETTINGS_FROM_SIDE_BAR)(item)
}

/// Comparer of `game_settings_box_handle_input`: the answers that only change the copy (Reset, a language, font,
/// currency, option or auto save interval) run side by side over the copy and the Setup regions (a language can pick
/// the default league); Cancel and Ok (they apply a Config and free the copy) and a box without a copy run only in the
/// original.
pub(crate) unsafe extern "C" fn compare_game_settings_box_handle_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_GAME_SETTINGS_BOX_HANDLE_INPUT);
    let config = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    let id = setup_clicked_id(item as i16);
    if config == 0 || !matches!(id, 0x27 | 0x2a | 0x2b | 0x2c | 0x2d | 0x3d) {
        return o(item);
    }
    let mut regions = setup_answer_regions();
    regions.push((config, CONFIG_SIZE));
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || game_settings_box_handle_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Game Settings answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Opens the Hall of Fame screen (`hall_of_fame_screen_build`, `hall_of_fame_screen_handle_input`) with its params 0, 1
/// and 2 (the table shown, the list's scroll row and the list's area) zeroed.
/// Original: `open_hall_of_fame_screen` (0x825cc0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_hall_of_fame_screen() {
    covered!("tel12_2 open hall of fame");
    setup_open_screen(0x825d10, 0x826690, 0, 3);
}

/// Builds the Hall of Fame screen: the table shown, the View and File menus and the list of the best 100.
///
/// The table shown is param 0 (its low byte). A random background, the title "Hall of Fame" in red with pale grey text,
/// the menu bar (mode 4) and the table's name as subtitle (`hall_of_fame_get_table_name`, short form). The View menu
/// (button at 0x6e, 0x91 to 0xeb, 0xa5, with its menu area 0x130, green): the first table of the order
/// (`hall_of_fame_sort_league_order`(1), 28 tables), then the submenus " Nations 1" (tables 2 to 14 of the order) and "
/// Nations 2" (15 to 28); each table is a button (`hall_of_fame_table_button`: id 0x2e, the table shown without id). The
/// File menu (button at 0x28f, 0x91 to 0x30c, 0xa5): Save As Text (id 0x2f) and Merge In (id 0x30; greyed, without id,
/// while a game runs, `G_INTERACTIVE_MODE_FLAG`). The list (0x6e, 0xaa to 0x30c, 0x217; 5 columns 4, 0x10, 0xf, 0xa, 5;
/// 0x14 rows): for each rank 0..99 the entry (`HallOfFame::get_entry` on `G_HALL_OF_FAME`), stopping at the first empty
/// one (no points) from row 20 on: the ordinal in column 0 (dark blue, centred); with points, the manager (column 1, "
/// %s"), the club (column 2, yellow), the date (column 3, orange: `localtime` of the entry's time into a game date,
/// `Date::get_short_weekday_date_string`) when it has one, and the points ("%ld", column 4, dark purple); without
/// points, "------" in column 4. The list keeps its scroll row (param 1, `Area::set_scroll_row`) and its area goes into
/// param 2. The back/next buttons by what the screen manager allows.
/// Original: `hall_of_fame_screen_build` (0x825d10, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_screen_build() {
    covered!("tel12_2 build hall of fame");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let name: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_GET_TABLE_NAME);
    let order: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_HALL_OF_FAME_SORT_LEAGUE_ORDER);
    let get_entry: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_GET_ENTRY);
    let localtime: unsafe extern "C" fn(*const i32) -> usize = core::mem::transmute(FN_CRT_LOCALTIME);
    let date_text: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_GET_SHORT_WEEKDAY_DATE_STRING);
    let button: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, usize, usize, u32, u32, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let mut columns: [u8; 5] = [4, 0x10, 0xf, 0xa, 5];
    let mut date = [0u8; 8];
    exe::date_date_54a110(date.as_mut_ptr() as usize);
    let shown = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0) as u8;
    exe::background_set_random_picture();
    text(SETUP_TEXT, 0x9ce514); // "Hall of Fame"
    exe::screenmanager_add_screen_title(SCREEN_MANAGER, setup_colour(G_GUI_COLOUR_RED) as u32, setup_colour(G_GUI_COLOUR_PALE_GREY) as u32, SETUP_TEXT);
    exe::menubar_build(4, 0);
    name(shown as u32, SETUP_TEXT, 0x7d0, 0);
    exe::screenmanager_add_screen_subtitle(SCREEN_MANAGER, SETUP_TEXT, 0);
    let (grey, pale, green) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_GREEN));
    // the View menu
    text(SETUP_TEXT, 0x988004); // "View"
    let view = button(SCREEN_MANAGER, 0x6e, 0x91, 0xeb, 0xa5, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0, 0);
    let menu = add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, green, view);
    let tables = order(1);
    hall_of_fame_table_button(read_u8(tables), shown, menu);
    let nations_1 = hall_of_fame_submenu(0xa80e94, menu); // " Nations 1"
    for k in 1..0xe {
        hall_of_fame_table_button(read_u8(tables + k), shown, nations_1);
    }
    let nations_2 = hall_of_fame_submenu(0xa80e88, menu); // " Nations 2"
    for k in 0xe..0x1c {
        hall_of_fame_table_button(read_u8(tables + k), shown, nations_2);
    }
    // the File menu
    text(SETUP_TEXT, 0xa80a80); // "File"
    let file = button(SCREEN_MANAGER, 0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0, 0);
    let file_menu = add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, green, file);
    let (yellow, black) = (setup_colour(G_GUI_COLOUR_YELLOW), setup_colour(G_GUI_COLOUR_BLACK));
    text(SETUP_TEXT, 0xa80e78); // "Save As Text"
    item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, 0x2f, 0, file_menu);
    text(SETUP_TEXT, 0xa80e6c); // "Merge In"
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, 0x30, 0, file_menu);
    } else {
        item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 0x21, 1, black, SETUP_TEXT, 0, 0, 0, file_menu);
    }
    // the list of the best 100
    let list = add_area(current_human_display(), 0x6e, 0xaa, 0x30c, 0x217, 5, columns.as_mut_ptr() as usize, 0x14, 0, 2, 0, usize::MAX);
    let mut entry = [0u8; 0x70];
    let e = entry.as_mut_ptr() as usize;
    let (dark_blue, orange, purple) = (setup_colour(G_GUI_COLOUR_DARK_BLUE), setup_colour(G_GUI_COLOUR_ORANGE), setup_colour(G_GUI_COLOUR_DARK_PURPLE));
    let mut row = 0u32;
    for rank in 0..100u8 {
        get_entry(read_usize(G_HALL_OF_FAME), shown as u32, rank as u32, e);
        let points = read_u32(e + HALL_OF_FAME_ENTRY_POINTS);
        if points == 0 && row >= 0x14 {
            break;
        }
        let ordinal = exe::utils_format_ordinal(rank as i8 as i32 + 1, -1);
        exe::screenmanager_add_area_text(SCREEN_MANAGER, list, 0, row, 0x30, dark_blue, 0xc, 1, pale, ordinal);
        if points != 0 {
            format(SETUP_TEXT, 0x987ff0, e + HALL_OF_FAME_ENTRY_NAME); // "  %s"
            exe::screenmanager_add_area_text(SCREEN_MANAGER, list, 1, row, 1, 0, 1, 2, pale, SETUP_TEXT);
            exe::screenmanager_add_area_text(SCREEN_MANAGER, list, 2, row, 1, 0, 1, 2, yellow, e + HALL_OF_FAME_ENTRY_CLUB);
            let when = read_i32(e + HALL_OF_FAME_ENTRY_DATE);
            if when > 0 {
                let tm = localtime(&when);
                let year = read_u16(tm + 0x14).wrapping_add(0x76c);
                exe::date_set(date.as_mut_ptr() as usize, read_u16(tm + 0xc) as u32, read_u8(tm + 0x10) as u32, year as u32, -1);
                let mut buffer = [0u8; 0xc8];
                date_text(date.as_mut_ptr() as usize, buffer.as_mut_ptr() as usize, 0xc8, 0xff);
                exe::screenmanager_add_area_text(SCREEN_MANAGER, list, 3, row, 1, 0, 1, 1, orange, buffer.as_ptr() as usize);
            }
            format(SETUP_TEXT, 0x98c11c, points); // "%ld"
            exe::screenmanager_add_area_text(SCREEN_MANAGER, list, 4, row, 0x30, purple, 0xc, 1, pale, SETUP_TEXT);
        } else {
            exe::screenmanager_add_area_text(SCREEN_MANAGER, list, 4, row, 0x30, purple, 0xc, 1, pale, 0xa80e64); // "------"
        }
        row += 1;
    }
    let scroll = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 1);
    let list = list as i16;
    if list != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + list as isize as usize * 0xbf1, scroll);
    }
    exe::screenmanager_set_screen_param(SCREEN_MANAGER, 2, list as isize as usize, 0);
    let next = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, next);
}

/// The Hall of Fame screen's answer to item `item`.
///
/// With a command waiting (`G_PENDING_COMMAND_DISPATCH_FLAG`) from the file dialogs: code 0x28 (Save As Text) clears the
/// code and, with a path (`G_PENDING_COMMAND_ARG`, cleared), exports the table (`HallOfFame::export_to_csv`); code 0x29
/// (Merge In) clears the code and, with a path, imports the file (`HallOfFame::import_file`, check only 0) and saves
/// the Hall of Fame (`HallOfFame::save`). A failure frees the path and shows the "Error" box (Setup.cpp 0x18c3, 0x18e0);
/// then the path is freed (DEFECT of the original: freed twice after a failure). A real item keeps the list's scroll row
/// (param 1, from the area in param 2). By the clicked id: 0x2e (a table of the View menu) other than the one shown
/// makes it param 0 (−11); 0x2f opens the save dialog ("Save Hall Of Fame", "Enter File Name", the save folder, ".txt",
/// "hall_of_fame.txt", code 0x28) and 0x30 the merge dialog ("Merge Hall Of Fame", "Select File", ".bin", checked by
/// `hall_of_fame_try_load_quiet`, code 0x29), pale grey on red, −4; anything else goes to the menu bar.
/// Original: `hall_of_fame_screen_handle_input` (0x826690, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_screen_handle_input(item: u32) -> i32 {
    covered!("tel12_2 hall of fame input");
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let export: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_EXPORT_TO_CSV);
    let import: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_IMPORT_FILE);
    let save: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_HALLOFFAME_SAVE);
    let dialog: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_OPEN_FILE_DIALOG);
    let raw = item;
    let item = item as i16;
    let shown = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0) as u8;
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 {
        let code = read_u32(G_PENDING_COMMAND_CODE);
        if code == 0x28 || code == 0x29 {
            let path = read_usize(G_PENDING_COMMAND_ARG);
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if path != 0 {
                write_u32(G_PENDING_COMMAND_ARG, 0);
                let hall = read_usize(G_HALL_OF_FAME);
                if code == 0x28 {
                    if export(hall, path) == 0 {
                        hall_of_fame_file_error(path, 0x18c3);
                    }
                } else {
                    if import(hall, path, 0) == 0 {
                        hall_of_fame_file_error(path, 0x18e0);
                    }
                    save(read_usize(G_HALL_OF_FAME));
                }
                exe::crt_operator_delete(path);
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
    }
    if item != -1 {
        let area = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 2);
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 1, exe::screenmanager_get_area_scroll_row(SCREEN_MANAGER, area), 0);
    }
    let id = setup_clicked_id(item);
    if id == 0x2e {
        let table = setup_clicked_value(item);
        if table as u8 != shown {
            exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, table, 0);
            return -11;
        }
    }
    let (red, pale) = (setup_colour(G_GUI_COLOUR_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
    let folder = read_usize(G_CONFIG) + CONFIG_SAVE_FOLDER;
    let (mut title, mut heading) = ([0u8; 0x100], [0u8; 0x100]);
    match id {
        0x2f => {
            let h = text(heading.as_mut_ptr() as usize, 0x9907f4); // "Enter File Name"
            let t = text(title.as_mut_ptr() as usize, 0xa80ebc); // "Save Hall Of Fame"
            dialog(t, h, 1, folder, 0x990804, 0xa80ed0, 0, pale, red, 0x28, 0, 0, 0, 0); // ".txt", "hall_of_fame.txt"
            -4
        }
        0x30 => {
            let h = text(heading.as_mut_ptr() as usize, 0xa7fc84); // "Select File"
            let t = text(title.as_mut_ptr() as usize, 0xa80ea0); // "Merge Hall Of Fame"
            dialog(t, h, 0, folder, 0xa80eb4, 0xad9c64, HALL_OF_FAME_TRY_LOAD_QUIET, pale, red, 0x29, 0, 0, 0, 0); // ".bin", ""
            -4
        }
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Opens the disk full box (`disk_full_box_build`, `disk_full_box_handle_input`) with `reason` in param 0.
/// Original: `open_disk_full_box` (0x826a60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_disk_full_box(reason: usize) {
    covered!("tel12_2 open disk full");
    setup_open_reason_box(0x826aa0, 0x826c20, reason);
}

/// Builds the disk full box: the header ("Disk Full"), the message chosen by the reason in param 0 (0, 1, 2 or any
/// other: the four "not enough free disk space" texts) with the current drive's letter (`_getdrive` + 0x40, "%c"), and
/// the Ok button (id 0x33).
/// Original: `disk_full_box_build` (0x826aa0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn disk_full_box_build() {
    covered!("tel12_2 build disk full");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let getdrive: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_CRT_GETDRIVE);
    let reason = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    setup_reason_box_header(0xa810c8); // "Disk Full"
    let mut drive = [0u8; 4];
    format(drive.as_mut_ptr() as usize, 0x99ec1c, getdrive() + 0x40); // "%c"
    let message = match reason {
        0 => 0xa81050,
        1 => 0xa80fd8,
        2 => 0xa80f60,
        _ => 0xa80ee4,
    };
    text(SETUP_TEXT, message, drive.as_ptr());
    setup_box_question();
    setup_reason_box_ok(0x33);
}

/// The disk full box's answer to item `item` (`setup_reason_box_answer` with Ok 0x33).
/// Original: `disk_full_box_handle_input` (0x826c20, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn disk_full_box_handle_input(item: u32) -> i32 {
    covered!("tel12_2 disk full input");
    setup_reason_box_answer(item, 0x33)
}

/// Opens the no memory box (`no_memory_box_build`, `no_memory_box_handle_input`) with `reason` in param 0.
/// Original: `open_no_memory_box` (0x826ce0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_no_memory_box(reason: usize) {
    covered!("tel12_2 open no memory");
    setup_open_reason_box(0x826d20, 0x826e60, reason);
}

/// Builds the no memory box: the header ("No Memory"), the message chosen by the reason in param 0 (0, 1, 2 or any
/// other: the four "not enough system memory" texts) and the Ok button (id 0x34).
/// Original: `no_memory_box_build` (0x826d20, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn no_memory_box_build() {
    covered!("tel12_2 build no memory");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let reason = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    setup_reason_box_header(0xa8127c); // "No Memory"
    let message = match reason {
        0 => 0xa81214,
        1 => 0xa811a8,
        2 => 0xa81140,
        _ => 0xa810d4,
    };
    text(SETUP_TEXT, message);
    setup_box_question();
    setup_reason_box_ok(0x34);
}

/// The no memory box's answer to item `item` (`setup_reason_box_answer` with Ok 0x34).
/// Original: `no_memory_box_handle_input` (0x826e60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn no_memory_box_handle_input(item: u32) -> i32 {
    covered!("tel12_2 no memory input");
    setup_reason_box_answer(item, 0x34)
}

/// Comparer of `open_hall_of_fame_screen`: only the original runs (it pushes a screen).
pub(crate) unsafe extern "C" fn compare_open_hall_of_fame_screen() {
    original_fn::<unsafe extern "C" fn()>(IDX_OPEN_HALL_OF_FAME_SCREEN)()
}

/// Comparer of `hall_of_fame_screen_build`: side by side over the display list, with the screen params (the list's
/// area goes into param 2).
pub(crate) unsafe extern "C" fn compare_hall_of_fame_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_HALL_OF_FAME_SCREEN_BUILD);
    let extra = screen_param_regions();
    compare_by_display_list_with("Hall of Fame screen", &extra, || o(), || hall_of_fame_screen_build());
}

/// Comparer of `hall_of_fame_screen_handle_input`: a click with no file command waiting that only picks a table or
/// goes nowhere (id 0x2e) runs side by side over the screen manager and the screen params; the file dialogs, the
/// waiting commands (they write files) and the menu bar run only in the original.
pub(crate) unsafe extern "C" fn compare_hall_of_fame_screen_handle_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_HALL_OF_FAME_SCREEN_HANDLE_INPUT);
    let waiting = read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 && matches!(read_u32(G_PENDING_COMMAND_CODE), 0x28 | 0x29);
    if waiting || setup_clicked_id(item as i16) != 0x2e {
        return o(item);
    }
    let mut regions = vec![(SCREEN_MANAGER, 0x3840)];
    regions.extend(screen_param_regions());
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || hall_of_fame_screen_handle_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Hall of Fame answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `open_disk_full_box`: only the original runs (it pushes a screen).
pub(crate) unsafe extern "C" fn compare_open_disk_full_box(reason: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_DISK_FULL_BOX)(reason)
}

/// Comparer of `disk_full_box_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_disk_full_box_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DISK_FULL_BOX_BUILD);
    compare_by_display_list_with("Disk Full box", &[], || o(), || disk_full_box_build());
}

/// Comparer of `disk_full_box_handle_input`: only the original runs (Ok closes the box, the menu bar opens screens).
pub(crate) unsafe extern "C" fn compare_disk_full_box_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_DISK_FULL_BOX_HANDLE_INPUT)(item)
}

/// Comparer of `open_no_memory_box`: only the original runs (it pushes a screen).
pub(crate) unsafe extern "C" fn compare_open_no_memory_box(reason: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_NO_MEMORY_BOX)(reason)
}

/// Comparer of `no_memory_box_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_no_memory_box_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_NO_MEMORY_BOX_BUILD);
    compare_by_display_list_with("No Memory box", &[], || o(), || no_memory_box_build());
}

/// Comparer of `no_memory_box_handle_input`: only the original runs (Ok closes the box, the menu bar opens screens).
pub(crate) unsafe extern "C" fn compare_no_memory_box_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_NO_MEMORY_BOX_HANDLE_INPUT)(item)
}

/// Appoints the human `manager` to `club` (returns 1; 0 with a null argument, after the Setup.cpp "Error" box 0x1a4d).
///
/// The club's manager leaves (`MANAGER_MANAGER::remove_club_manager`, departure 3, reason 0, keeping the ratings), or,
/// without one, the manager of its linked B team (`club_get_linked_b_team`(club, 0, 1)) when it has one. For a national
/// side (`database_is_national_team`) that is not a club's national team (`club_is_national_team`), the manager of the
/// nation's under-21 side (`database_get_national_u21_team` of `club::NATION`) leaves too, unless a human runs it
/// (`human_manager_is_human_club`(u21, 1, 1, 0, 0)). The manager's reputations come from the club's (`club::REPUTATION`):
/// current and home reputation = reputation / 10 + 3000, world reputation = reputation × 0.4 + 750 (truncated);
/// `staff::LOYALTY` = 20. The club leaves the friendlies' AI plans (`Friendly::remove_club_from_ai_plans`) and the
/// manager is appointed (`MANAGER_MANAGER::appoint_manager_to_club`); outside a running game (`G_INTERACTIVE_MODE_FLAG`
/// = 0) bit 0 of byte 2 of the club's 0x49-byte record in the MANAGER_MANAGER table is cleared.
/// Original: `appoint_human_manager_to_club` (0x826f20, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn appoint_human_manager_to_club(manager: usize, club: usize) -> u32 {
    covered!("tel12_3 appoint human manager");
    let (manager, club) = (Staff(manager), Club(club));
    let friendly: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FRIENDLY_REMOVE_CLUB_FROM_AI_PLANS);
    if manager.0 == 0 || club.0 == 0 {
        setup_warning(0x1a4d);
        return 0;
    }
    if club.manager().0 != 0 {
        exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), club.0, 3, 0, 1);
    } else {
        let b = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
        if b != 0 && Club(b).manager().0 != 0 {
            exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), b, 3, 0, 1);
        }
    }
    if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 {
        let u21 = exe::database_get_national_u21_team(read_usize(club.0 + club::NATION));
        if Club(u21).manager().0 != 0 && exe::human_manager_is_human_club(u21, 1, 1, 0, 0) == 0 {
            exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), u21, 3, 0, 1);
        }
    }
    let reputation = club.reputation() as i32;
    let np = manager.non_player();
    np.set_current_reputation((reputation / 10 + 0xbb8) as i16);
    np.set_home_reputation((reputation / 10 + 0xbb8) as i16);
    let world = ftol(reputation as f64 * read_f64(rdata::F64_0_4) + read_f64(rdata::F64_750));
    manager.non_player().set_world_reputation(world as i16);
    manager.set_loyalty(0x14);
    friendly(read_usize(G_FRIENDLY), club.0);
    exe::manager_manager_appoint_manager_to_club(read_usize(G_MANAGER_MANAGER), club.0, manager.0);
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        let id = read_u32(club.0) as usize;
        let flags = read_usize(read_usize(G_MANAGER_MANAGER) + MANAGER_MANAGER_CLUB_RECORDS) + id * 0x49 + 2;
        write_u8(flags, read_u8(flags) & 0xfe);
    }
    1
}

/// Picks the default league by the game's language: with a database (`G_NATION_TABLE`; without one, the Setup.cpp
/// "Error" box 0x1d88 and 0), every nation's `nation::LEAGUE_SELECTED` is zeroed, then the key nation of the language
/// (`langlib_get_language`: 0 and 1 England, 2 France, 3 Germany, 4 Italy, 5 Spain, 6 Denmark, 7 Holland, 8 Norway, 9
/// Portugal, 10 Sweden, 11 Japan, 12 South Korea) gets 2 (selected, in the foreground), `G_NATIONS_TO_LOAD_COUNT` =
/// `G_SELECTED_NATION_COUNT` = 1 and becomes `G_MAIN_NATION`; returns 1. Any other language: the counters and the main
/// nation stay zeroed, the "Error" box 0x1e09 and 0.
/// Original: `select_default_league_by_language` (0x827d50, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn select_default_league_by_language() -> u32 {
    covered!("tel12_3 default league");
    if read_usize(G_NATION_TABLE) == 0 {
        setup_warning(0x1d88);
        return 0;
    }
    for k in 0..read_i32(G_NATION_COUNT).max(0) as usize {
        write_u8(read_usize(G_NATION_TABLE) + k * nation::SIZE + nation::LEAGUE_SELECTED, 0);
    }
    write_u32(G_NATIONS_TO_LOAD_COUNT, 0);
    write_u32(G_SELECTED_NATION_COUNT, 0);
    write_u32(G_MAIN_NATION, 0);
    let key = match exe::langlib_get_language() {
        0 | 1 => KEY_NATION_ENGLAND,
        2 => KEY_NATION_FRANCE,
        3 => KEY_NATION_GERMANY,
        4 => KEY_NATION_ITALY,
        5 => KEY_NATION_SPAIN,
        6 => KEY_NATION_DENMARK,
        7 => KEY_NATION_HOLLAND,
        8 => KEY_NATION_NORWAY,
        9 => KEY_NATION_PORTUGAL,
        10 => KEY_NATION_SWEDEN,
        11 => KEY_NATION_JAPAN,
        12 => KEY_NATION_SOUTH_KOREA,
        _ => {
            setup_warning(0x1e09);
            return 0;
        }
    };
    let selected = |k: usize| read_usize(G_NATION_TABLE) + read_u32(k) as usize * nation::SIZE;
    write_u8(selected(key) + nation::LEAGUE_SELECTED, 2);
    write_u32(G_NATIONS_TO_LOAD_COUNT, 1);
    write_u32(G_SELECTED_NATION_COUNT, 1);
    write_usize(G_MAIN_NATION, selected(key));
    1
}

/// Creates a new game (`quick_start` is not read: the original's argument is ignored); returns 1 when it is made.
///
/// Below 25 MB of free disk (`os_get_free_disk_space_mb`) `G_GAME_IN_PROGRESS_FLAG` = 1, below 50 MB of free memory
/// (with `G_MEMORY_CHECK_ENABLED`) `G_GAME_SESSION_STARTED_FLAG` = 1, and 0. The game CD (`create_game_find_cd`); without
/// a database, it is loaded from CM3_DATA (`load_database_from_files`(path, 1); a failure: the Setup.cpp "Error" box
/// 0x1e3d and 0) and the Config applied (`Config::apply_and_save`). Then: `database_trim_senior_and_b_team_squads`; the
/// command line checks "-verifypics" (`verify_background_config`), "-verifyhist" (`verify_history_files`),
/// "-verifyplcfg" (`player_setup_cfg_load`(0x203b, 1)) and "-verifyeurocfg" (`validate_euro_cfg`);
/// `database_build_nation_name_index`, `database_prune_comp_history_table`; the database size (Config options & 7) into
/// `G_DATABASE_SIZE_OPTION` trims the people and the clubs (`database_trim_staff_tables`, `database_trim_club_table`);
/// with Greece selected, its two divisions get 14 clubs (`fix_division_club_count`); `database_merge_b_team_staff`; the
/// club reputation order (`G_CLUB_REPUTATION_ORDER`, 4 × `G_CLUB_COUNT`; fatal 0x1ea0) sorted
/// (`database_sort_clubs_by_reputation`(1)); the start date (`setup_set_game_start_date`) and `G_GAME_START_DATE` moved
/// back 4 weeks (`Date::sub_weeks`); the people's runtime records (`create_game_staff_states`; fatal 0x1eaf) and the
/// clubs' (`G_CLUB_STATE_TABLE`, 6 × `G_CLUB_COUNT`: −1, 0, 0; fatal 0x1f0c); `G_GAME_PLAY_TIME_LAST_STAMP` = time(),
/// `G_GAME_PLAY_TIME_TOTAL` = 0; each club's `club::CONTINENTAL_COMP_ID` = −1 and its squad members without a club job
/// get it; `database_player_initialisation` at the game date and, with the German setup or without real players,
/// `database_apply_fake_players`. Then "Initialising game data" with the progress bar (0x2c steps): step 1 clears the
/// `*.tmp` files, and before each later step the disk and memory are checked again (25 MB each; before Match_Stats the
/// disk must hold its matches: 5000 with option 0x400, else 20000 with 0x800, else 10000, × 3450 >> 20 MB, 25 when 0):
/// the subsystems of `CREATE_STEPS` (each `operator new` and its constructor into its global; a null global gives its
/// "Error" box and 0, FOG_OF_WAR checking the Media global as in the original; CONTRACT_MANAGER then
/// `initialise_all_staff_contracts`), `key_nation_number_active_entries`, `key_nation_create_all_comps`,
/// `build_comp_object_table`, `ReserveComp::create_sub_comps`, `FinanceTable::refresh_all_wage_bills`, the club pass
/// (`create_game_clubs`) and `SQUAD_MANAGER::init_new_game`. Finally `game_run_to_start_date`,
/// `G_GAME_DATA_LOADED_FLAG` = 1, `G_GAME_HAS_SAVE_FILE_FLAG` = 0 and, without `auto_setup.cfg`, the "CD Not Required"
/// search (`check_game_cd`(false)). The original's two-pass loop (the first only sets the total 0x2c) runs the body once.
/// Original: `create_game` (0x828110, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_game(_quick_start: u32) -> u32 {
    covered!("tel12_3 create game");
    let call = |f: usize| {
        let f: unsafe extern "C" fn() = core::mem::transmute(f);
        f()
    };
    let size_call = |f: usize, size: u32| {
        let f: unsafe extern "C" fn(u32) = core::mem::transmute(f);
        f(size)
    };
    let text: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let player_setup: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
    let fix_division: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_FIX_DIVISION_CLUB_COUNT);
    let initialise: unsafe extern "C" fn(usize) = core::mem::transmute(FN_DATABASE_PLAYER_INITIALISATION);
    let apply: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONFIG_APPLY_AND_SAVE);
    let no_memory = |minimum: i32| read_u32(G_MEMORY_CHECK_ENABLED) != 0 && exe::os_get_free_memory_mb() < minimum;
    if exe::os_get_free_disk_space_mb(0) < 0x19 {
        write_u32(G_GAME_IN_PROGRESS_FLAG, 1);
        return 0;
    }
    if no_memory(0x32) {
        write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
        return 0;
    }
    create_game_find_cd();
    if read_usize(G_NATION_TABLE) == 0 {
        let mut data = [0u8; 0x104];
        copy_text(data.as_mut_ptr(), exe::utils_build_file_path(0x9b5348, 0xad9c64) as *const u8); // "CM3_DATA"
        if exe::load_database_from_files(data.as_ptr(), 1) == 0 {
            setup_warning(0x1e3d);
            return 0;
        }
        apply(read_usize(G_CONFIG));
    }
    call(FN_DATABASE_TRIM_SENIOR_AND_B_TEAM_SQUADS);
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0x9899d4) != 0 {
        call(FN_VERIFY_BACKGROUND_CONFIG); // "-verifypics"
    }
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa813d4) != 0 {
        call(FN_VERIFY_HISTORY_FILES); // "-verifyhist"
    }
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa813c4) != 0 {
        player_setup(0x203b, 1); // "-verifyplcfg"
    }
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa813b4) != 0 {
        call(FN_VALIDATE_EURO_CFG); // "-verifyeurocfg"
    }
    call(FN_DATABASE_BUILD_NATION_NAME_INDEX);
    call(FN_DATABASE_PRUNE_COMP_HISTORY_TABLE);
    let size = (read_u32(read_usize(G_CONFIG) + CONFIG_OPTIONS) & 7) as u8;
    write_u8(G_DATABASE_SIZE_OPTION, size);
    size_call(FN_DATABASE_TRIM_STAFF_TABLES, size as i8 as i32 as u32);
    size_call(FN_DATABASE_TRIM_CLUB_TABLE, read_u8(G_DATABASE_SIZE_OPTION) as u32);
    let greece = Nation(read_usize(G_NATION_TABLE) + read_u32(KEY_NATION_GREECE) as usize * nation::SIZE);
    if greece.league_selected() != 0 {
        fix_division(read_u32(KEY_CLUB_COMP_GREEK_PREMIER_DIVISION), 0xe);
        fix_division(read_u32(KEY_CLUB_COMP_GREEK_SECOND_DIVISION), 0xe);
    }
    call(FN_DATABASE_MERGE_B_TEAM_STAFF);
    let order = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(4));
    write_usize(G_CLUB_REPUTATION_ORDER, order);
    if order == 0 {
        setup_fatal_error(0x1ea0);
    }
    exe::database_sort_clubs_by_reputation(1);
    call(FN_SETUP_SET_GAME_START_DATE);
    let (mut earlier, mut tmp) = ([0u8; 8], [0u8; 8]);
    let r = exe::date_sub_weeks(G_GAME_START_DATE, &mut earlier, 4);
    exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, r);
    write_u32(G_STAFF_STATE_COUNT, read_u32(G_STAFF_COUNT));
    let states = exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(staff_state::SIZE as u32));
    write_usize(G_STAFF_STATE_TABLE, states);
    if states == 0 {
        setup_fatal_error(0x1eaf);
    }
    create_game_staff_states(states);
    write_u32(G_CLUB_STATE_COUNT, read_u32(G_CLUB_COUNT));
    let clubs = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(6));
    write_usize(G_CLUB_STATE_TABLE, clubs);
    if clubs == 0 {
        setup_fatal_error(0x1f0c);
    }
    for i in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let c = clubs + 6 * i;
        write_u32(c, 0xffffffff);
        write_u8(c + 4, 0);
        write_u8(c + 5, 0);
    }
    let mut now = 0i32;
    write_i32(G_GAME_PLAY_TIME_LAST_STAMP, exe::at_946ebb(&mut now));
    write_u32(G_GAME_PLAY_TIME_TOTAL, 0);
    for i in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let club = Club(read_usize(G_CLUB_TABLE) + i * club::SIZE);
        club.set_continental_comp_id(0xffffffff);
        for k in 0..0x32 {
            let p = read_usize(read_usize(G_CLUB_TABLE) + i * club::SIZE + club::SQUAD + 4 * k);
            if p != 0 && read_usize(p + staff::CLUB_JOB) == 0 {
                write_usize(p + staff::CLUB_JOB, read_usize(G_CLUB_TABLE) + i * club::SIZE);
            }
        }
    }
    initialise(G_CURRENT_DATE);
    if read_u8(G_PLAYER_SETUP_GERMAN_FLAG) != 0 || read_u8(G_USE_REAL_PLAYERS) == 0 {
        call(FN_DATABASE_APPLY_FAKE_PLAYERS);
    }
    // the progress bar: "Initialising game data", 0x2c steps
    let mut label = [0u8; 0xc8];
    text(label.as_mut_ptr(), 0x9cc2ac);
    let progress = |k: u32| exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, label.as_ptr() as usize, k as i32, 0x2c, -1);
    // disk and memory between the steps: sets the flag and returns true
    let short_of = |minimum_disk: i32| {
        if exe::os_get_free_disk_space_mb(0) < minimum_disk {
            write_u32(G_GAME_IN_PROGRESS_FLAG, 1);
            return true;
        }
        if no_memory(0x19) {
            write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
            return true;
        }
        false
    };
    progress(1);
    exe::utils_clear_temp_folder(0xa813ac); // "*.tmp"
    let options = read_u32(read_usize(G_CONFIG) + CONFIG_OPTIONS);
    let matches: u32 = if options & 0x400 != 0 { 5000 } else if options & 0x800 != 0 { 20000 } else { 10000 };
    for &(k, step) in CREATE_STEPS.iter() {
        let disk = if k == 0xb {
            match matches.wrapping_mul(0xd7a) >> 20 {
                0 => 0x19,
                mb => mb as i32,
            }
        } else {
            0x19
        };
        if short_of(disk) {
            return 0;
        }
        progress(k);
        match step {
            CreateStep::Routine(f, row) => {
                let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(f);
                if f() == 0 {
                    setup_warning(row);
                    return 0;
                }
            }
            CreateStep::Method(f, global) => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
                f(read_usize(global));
            }
            CreateStep::Object(size, constructor, global, row, arg) => {
                let p = exe::crt_operator_new(size);
                let o = if p == 0 {
                    0
                } else {
                    match arg {
                        CreateArg::Zero => {
                            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(constructor);
                            f(p, 0)
                        }
                        CreateArg::Alone => {
                            let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(constructor);
                            f(p)
                        }
                        CreateArg::Stats => {
                            let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(constructor);
                            f(p, 0, matches)
                        }
                        CreateArg::Fixture => {
                            let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(constructor);
                            f(p, read_u16(G_CURRENT_DATE_YEAR) as u32, 0, 1)
                        }
                        CreateArg::Language => {
                            let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(constructor);
                            f(p, read_u8(G_DATABASE_NAMES_LANGUAGE) as u32)
                        }
                    }
                };
                write_usize(global, o);
                let checked = if global == G_FOG_OF_WAR { read_usize(G_MEDIA) } else { o };
                if checked == 0 {
                    setup_warning(row);
                    return 0;
                }
                if global == G_CONTRACT_MANAGER {
                    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONTRACT_MANAGER_INITIALISE_ALL_STAFF_CONTRACTS);
                    f(o);
                }
            }
            CreateStep::Clubs => create_game_clubs(),
        }
    }
    call(FN_GAME_RUN_TO_START_DATE);
    write_u32(G_GAME_DATA_LOADED_FLAG, 1);
    write_u32(G_GAME_HAS_SAVE_FILE_FLAG, 0);
    if exe::utils_file_exists(0xa80340) == 0 {
        check_game_cd(false); // "auto_setup.cfg"
    }
    1
}

/// Comparer of `appoint_human_manager_to_club`: only the original runs (managers leave and are appointed, with their
/// news and the friendlies' plans; it cannot run twice). The copy is proved in Rust mode: the automatic career runs it
/// once, when its human manager is put on a club (`create_game_from_setup_file`).
pub(crate) unsafe extern "C" fn compare_appoint_human_manager_to_club(manager: usize, club: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_APPOINT_HUMAN_MANAGER_TO_CLUB)(manager, club)
}

/// Comparer of `select_default_league_by_language`: side by side over the nations' league flags, the three league
/// counters and the main nation (the "Error" boxes only open without a database or with an unknown language: original
/// only then).
pub(crate) unsafe extern "C" fn compare_select_default_league_by_language() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_SELECT_DEFAULT_LEAGUE_BY_LANGUAGE);
    let nations = read_usize(G_NATION_TABLE);
    let count = read_i32(G_NATION_COUNT);
    if nations == 0 || count <= 0 || exe::langlib_get_language() > 12 {
        return o();
    }
    let regions = [(nations, count as usize * nation::SIZE), (G_NATIONS_TO_LOAD_COUNT, 12)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || select_default_league_by_language());
    SCOREBOARD.record(equal && r == n, || format!("select_default_league_by_language: original={} rust={} differences at {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `create_game`: only the original runs (it builds the whole game: it cannot run twice). The copy is proved
/// in Rust mode: the automatic career creates its game through it (`create_game_from_setup_file`).
pub(crate) unsafe extern "C" fn compare_create_game(quick_start: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_CREATE_GAME)(quick_start)
}

/// Sets the new game's start date (`G_GAME_START_DATE`) from the key nation entries; returns 1, or 0 when no entry
/// fits (the date stays).
///
/// The 35 entries of `G_LEAGUE_TABLE` are walked in order, skipping those without a nation
/// (`KEY_NATION_RECORD_NATION`). The entry of the main nation (`G_MAIN_NATION`) gives its `key_nation_record::START_DATE`
/// (`Date::operator=`); then, for a European main nation (the id of its `nation::CONTINENT` is `KEY_CONTINENT_EUROPE`'s)
/// in a 2001 start, a start after Date::Date(1, 8, 2001, −1) (1 September 2001: the later year, unsigned, or the same
/// year and a later day, signed) is brought back to it. Without a main nation, the first entry whose nation has bit 1
/// of `nation::LEAGUE_SELECTED` gives its start date as it is.
/// Original: `setup_set_game_start_date` (0x830520, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_set_game_start_date() -> u32 {
    covered!("tel13_1 game start date");
    let main = read_usize(G_MAIN_NATION);
    for k in 0..KEY_NATION_ENTRY_COUNT {
        let entry = G_LEAGUE_TABLE + k * KEY_NATION_RECORD_SIZE;
        let nation = read_usize(entry + KEY_NATION_RECORD_NATION);
        if nation == 0 {
            continue;
        }
        let mut tmp = [0u8; 8];
        if nation == main {
            exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, entry + key_nation_record::START_DATE);
            if read_u32(Nation(nation).continent()) == read_u32(KEY_CONTINENT_EUROPE) && read_u16(G_GAME_START_DATE_YEAR) == 0x7d1 {
                let mut september = [0u16; 4];
                exe::date_date_549f70(&mut september, 1, 8, 0x7d1, 0xffff_ffff);
                let start_year = read_u16(G_GAME_START_DATE_YEAR);
                let later = september[1] < start_year || (september[1] == start_year && (september[0] as i16) < read_i16(G_GAME_START_DATE));
                if later {
                    let mut again = [0u16; 4];
                    let d = exe::date_date_549f70(&mut again, 1, 8, 0x7d1, 0xffff_ffff);
                    exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, d);
                }
            }
            return 1;
        }
        if main == 0 && Nation(nation).league_selected() & 2 != 0 {
            exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, entry + key_nation_record::START_DATE);
            return 1;
        }
    }
    0
}

/// Builds the table of competition objects by competition id; returns 1, or 0 when it cannot be allocated (the
/// Setup.cpp "Error" box 0x3896).
///
/// `G_COMP_OBJECT_COUNT` = `G_CLUB_COMP_COUNT` and `G_COMP_OBJECT_TABLE` = calloc(count × 4, 1). Each key nation entry
/// with a competition array (`KEY_NATION_RECORD_COMPS`) puts its non-null objects (up to `KEY_NATION_RECORD_COMP_COUNT`,
/// signed, read again each turn) at the id of their competition record (`league::COMPETITION`); then the friendlies
/// add theirs (`Friendly::register_comp_objects` on `G_FRIENDLY`).
/// Original: `build_comp_object_table` (0x8386f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn build_comp_object_table() -> u32 {
    covered!("tel13_1 comp object table");
    let count = read_u32(G_CLUB_COMP_COUNT);
    write_u32(G_COMP_OBJECT_COUNT, count);
    let table = exe::crt_calloc(count.wrapping_mul(4), 1);
    write_usize(G_COMP_OBJECT_TABLE, table);
    if table == 0 {
        setup_warning(0x3896);
        return 0;
    }
    for k in 0..KEY_NATION_ENTRY_COUNT {
        let entry = G_LEAGUE_TABLE + k * KEY_NATION_RECORD_SIZE;
        if read_usize(entry + KEY_NATION_RECORD_COMPS) == 0 {
            continue;
        }
        let mut i = 0i32;
        while i < read_i32(entry + KEY_NATION_RECORD_COMP_COUNT) {
            let comp = League(read_usize(read_usize(entry + KEY_NATION_RECORD_COMPS) + i as usize * 4));
            if !comp.is_null() {
                let id = read_u32(comp.competition().addr()) as usize;
                write_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4)), comp.addr());
            }
            i += 1;
        }
    }
    let register: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FRIENDLY_REGISTER_COMP_OBJECTS);
    register(read_usize(G_FRIENDLY));
    1
}

/// Creates (`join` 0) or joins the network game `game_name`; returns 0 when it worked, −4 when not (0 with a null name,
/// after the Setup.cpp "Error" box 0x391a).
///
/// The status message "Creating network game ..." or "Joining network game ..." (langlib into `SETUP_TEXT`) goes out
/// (`ScreenManager::send_network_status_message`, 0, 0, −1); `G_SCREEN_MANAGER_NETWORK_HOST_FLAG` = 1 when creating;
/// the widget snapshot pool gets 16 slots when creating, 1 when joining (`display_alloc_guio_snapshot_pool`; the
/// original loads the remote Display into ecx, which the stdcall ignores). With a network object
/// (`G_SCREEN_MANAGER_NETWORK`) `Network::host_or_join_game`(join, name) nonzero means done. Otherwise a game in memory
/// (`G_COMP_OBJECT_TABLE`) is freed (`free_game_data`(0)) and the "Unable To Create" / "Unable To Join" box opens with
/// "The network game "<name>" could not be created." / "... could not be joined." (`setup_confirm_message`).
/// Original: `setup_create_or_join_network_game` (0x8388e0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_create_or_join_network_game(join: u32, game_name: usize) -> i32 {
    covered!("tel13_1 network game");
    if game_name == 0 {
        setup_warning(0x391a);
        return 0;
    }
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(SETUP_TEXT, if join == 0 { 0xa823ec } else { 0xa823d0 }); // "Creating network game ...", "Joining network game ..."
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, SETUP_TEXT, 0, 0, -1);
    write_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG, (join == 0) as u32);
    let pool: unsafe extern "stdcall" fn(u32) = core::mem::transmute(FN_DISPLAY_ALLOC_GUIO_SNAPSHOT_POOL);
    pool(if join == 0 { 0x10 } else { 1 });
    let network = read_usize(G_SCREEN_MANAGER_NETWORK);
    if network != 0 {
        let host_or_join: unsafe extern "thiscall" fn(usize, u32, usize) -> i32 = core::mem::transmute(FN_NETWORK_HOST_OR_JOIN_GAME);
        if host_or_join(network, join, game_name) != 0 {
            return 0;
        }
    }
    if read_usize(G_COMP_OBJECT_TABLE) != 0 {
        exe::free_game_data(0);
    }
    let mut heading = [0u8; 0x100];
    if join == 0 {
        setup_confirm_message(&mut heading, 0x9c7c98, 0xa8238c, game_name); // "Unable To Create"
    } else {
        setup_confirm_message(&mut heading, 0xa82338, 0xa82348, game_name); // "Unable To Join"
    }
    -4
}

/// The network hook for a player who joined (`player_id`); returns nothing.
///
/// With a positive `HUMAN_MANAGER_COUNT_8` or `human_manager_fields::HOLIDAY_COUNT` on `G_HUMAN_MANAGER`, Manager
/// Status opens (`setup_open_screen` with its five params zeroed) and that is all. In a running game
/// (`G_INTERACTIVE_MODE_FLAG`) the squad screen opens (`open_club_squad_screen`(0, 0, 0, 0, 0)) and a human manager is
/// added (`setup_add_human_manager`). Otherwise, when the current slot goes over the network
/// (`screen_manager::NETWORK_SLOT` of slot `G_SCREEN_MANAGER_CURRENT_SLOT`) and the player has exactly one record
/// (`G_SCREEN_MANAGER_SLOT_RECORD_COUNTS`), Manager Status opens; on the host (`G_NETWORK_SETUP_HOST_FLAG`) that is all,
/// on a client a human manager is added after it (whether it opened or not); in every other case a human manager is
/// added.
/// Original: `setup_on_network_player_joined` (0x838a80, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_on_network_player_joined(player_id: i16) {
    covered!("tel13_1 player joined");
    let add: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_ADD_HUMAN_MANAGER);
    let humans = read_usize(G_HUMAN_MANAGER);
    if read_i32(humans + HUMAN_MANAGER_COUNT_8) > 0 || read_i32(humans + human_manager_fields::HOLIDAY_COUNT) > 0 {
        setup_open_screen(0x81ff10, 0x820950, 0, 5);
        return;
    }
    if read_u32(G_INTERACTIVE_MODE_FLAG) != 0 {
        exe::open_club_squad_screen(0, (0) as u32 as usize, 0, 0, 0);
        add();
        return;
    }
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300;
    if read_u16(G_SCREEN_MANAGER + slot + screen_manager::NETWORK_SLOT) != 0
        && read_u16(G_SCREEN_MANAGER_SLOT_RECORD_COUNTS + player_id as isize as usize * 2) == 1
    {
        if read_u32(G_NETWORK_SETUP_HOST_FLAG) != 0 {
            setup_open_screen(0x81ff10, 0x820950, 0, 5);
            return;
        }
        setup_open_screen(0x81ff10, 0x820950, 0, 5);
    }
    add();
}

/// The network hook for a player who left; returns nothing.
///
/// The manager of the current slot (`G_SCREEN_RECORDS_MANAGER` of slot `G_SCREEN_MANAGER_CURRENT_SLOT`), when there is
/// one: with `staff::CLASSIFICATION` 0 or 5 it becomes 0; otherwise, with a `G_HUMAN_MANAGER` that is not on holiday
/// (`HUMAN_MANAGER::is_on_holiday`(0)), `HUMAN_MANAGER::set_on_holiday`(1, 0). Then the viewer leaves the match
/// (`match_day_drop_viewer_and_finish_match`, the original's tail jump).
/// Original: `setup_on_network_player_left` (0x838c00, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_on_network_player_left() {
    covered!("tel13_1 player left");
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300;
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + slot));
    if manager.0 != 0 {
        let classification = manager.classification();
        if classification == 0 || classification == 5 {
            manager.set_classification(0);
        } else {
            let humans = read_usize(G_HUMAN_MANAGER);
            if humans != 0 && exe::human_manager_is_on_holiday(humans, 0usize) == 0 {
                exe::human_manager_set_on_holiday(humans, 1, 0usize);
            }
        }
    }
    let finish: unsafe extern "C" fn() = core::mem::transmute(FN_MATCH_DAY_DROP_VIEWER_AND_FINISH_MATCH);
    finish();
}

/// Whether `nation` passes the region filter text `filter`: 1 or 0.
///
/// "ALL" anywhere: 1. ", EUROPE" or ",EUROPE" anywhere, or a filter starting with "EUROPE": the nation (its id against
/// the key's) is England, Scotland, Northern Ireland, the Republic of Ireland, Wales, France, Spain, Italy, Germany,
/// Holland, Belgium, Portugal, Turkey or Greece. Else "EASTERN_EUROPE": Russia, Croatia or Poland. Else "SCANDINAVIA":
/// Sweden, Norway, Finland or Denmark. Else "AMERICAS": Brazil, Argentina or the United States. Else "ASIA": Australia
/// or Japan. Anything else: 0.
/// Original: `nation_matches_region_filter` (0x839340, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn nation_matches_region_filter(nation: usize, filter: usize) -> u32 {
    covered!("tel13_1 region filter");
    let has = |text: usize| exe::crt_strstr(filter, text);
    let one_of = |keys: &[usize]| keys.iter().any(|&k| read_u32(nation) == read_u32(k)) as u32;
    if has(0xa82514) != 0 {
        return 1; // "ALL"
    }
    if has(0xa82508) != 0 || has(0xa82500) != 0 || has(0x9d6814) == filter {
        // ", EUROPE", ",EUROPE", "EUROPE"
        return one_of(&[
            KEY_NATION_ENGLAND, KEY_NATION_SCOTLAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_WALES,
            KEY_NATION_FRANCE, KEY_NATION_SPAIN, KEY_NATION_ITALY, KEY_NATION_GERMANY, KEY_NATION_HOLLAND, KEY_NATION_BELGIUM,
            KEY_NATION_PORTUGAL, KEY_NATION_TURKEY, KEY_NATION_GREECE,
        ]);
    }
    if has(0xa824f0) != 0 {
        return one_of(&[KEY_NATION_RUSSIA, KEY_NATION_CROATIA, KEY_NATION_POLAND]); // "EASTERN_EUROPE"
    }
    if has(0xa824e4) != 0 {
        return one_of(&[KEY_NATION_SWEDEN, KEY_NATION_NORWAY, KEY_NATION_FINLAND, KEY_NATION_DENMARK]); // "SCANDINAVIA"
    }
    if has(0xa824d8) != 0 {
        return one_of(&[KEY_NATION_BRAZIL, KEY_NATION_ARGENTINA, KEY_NATION_UNITED_STATES]); // "AMERICAS"
    }
    if has(0x9d681c) != 0 {
        return one_of(&[KEY_NATION_AUSTRALIA, KEY_NATION_JAPAN]); // "ASIA"
    }
    0
}

/// The qsort comparison of two nation pointers by nationality: a null first one sorts after (1), a null second one
/// before (−1); otherwise `utils_string_compare` of their `nation::NATIONALITY` texts (1, 0).
/// Original: `compare_nations_by_nationality` (0x839530, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_nations_by_nationality(a: usize, b: usize) -> i32 {
    covered!("tel13_1 nations by nationality");
    let (x, y) = (read_usize(a), read_usize(b));
    if x == 0 {
        return 1;
    }
    if y == 0 {
        return -1;
    }
    exe::utils_string_compare((x + nation::NATIONALITY) as *const u8, (y + nation::NATIONALITY) as *const u8, 1, 0)
}

/// The game title: "Championship Manager 2001/02" translated into `G_SETUP_SCREEN_TITLE` (`setup_format_title`),
/// which is returned.
/// Original: `get_game_title` (0x839570, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_game_title() -> usize {
    covered!("tel13_1 game title");
    setup_format_title();
    G_SETUP_SCREEN_TITLE
}

/// Comparer of `setup_set_game_start_date`: side by side over `G_GAME_START_DATE` and the result.
pub(crate) unsafe extern "C" fn compare_setup_set_game_start_date() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_SETUP_SET_GAME_START_DATE);
    let regions = [(G_GAME_START_DATE, 8)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || setup_set_game_start_date());
    SCOREBOARD.record(equal && r == n, || format!("setup_set_game_start_date: original={} rust={} differences at {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `build_comp_object_table`: only the original runs (it allocates the table); then the plan: the count is
/// `G_CLUB_COMP_COUNT` and every competition object of the key nation entries sits at its id (unless a later one, or a
/// friendly, has the same id).
pub(crate) unsafe extern "C" fn compare_build_comp_object_table() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_BUILD_COMP_OBJECT_TABLE)();
    let table = read_usize(G_COMP_OBJECT_TABLE);
    if r == 0 || table == 0 {
        return r;
    }
    let mut ok = read_u32(G_COMP_OBJECT_COUNT) == read_u32(G_CLUB_COMP_COUNT);
    let mut missing = 0usize;
    for k in 0..KEY_NATION_ENTRY_COUNT {
        let entry = G_LEAGUE_TABLE + k * KEY_NATION_RECORD_SIZE;
        let comps = read_usize(entry + KEY_NATION_RECORD_COMPS);
        if comps == 0 {
            continue;
        }
        for i in 0..read_i32(entry + KEY_NATION_RECORD_COMP_COUNT).max(0) as usize {
            let comp = League(read_usize(comps + i * 4));
            if !comp.is_null() && read_usize(table + read_u32(comp.competition().addr()) as usize * 4) == 0 {
                missing += 1;
            }
        }
    }
    ok &= missing == 0;
    SCOREBOARD.record(ok, || format!("build_comp_object_table: count {} of {}, {} objects missing", read_u32(G_COMP_OBJECT_COUNT), read_u32(G_CLUB_COMP_COUNT), missing));
    r
}

/// Comparer of `setup_create_or_join_network_game`: only the original runs (network, messages, boxes).
pub(crate) unsafe extern "C" fn compare_setup_create_or_join_network_game(join: u32, game_name: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(u32, usize) -> i32>(IDX_SETUP_CREATE_OR_JOIN_NETWORK_GAME)(join, game_name)
}

/// Comparer of `setup_on_network_player_joined`: only the original runs (it opens screens and adds managers).
pub(crate) unsafe extern "C" fn compare_setup_on_network_player_joined(player_id: i16) {
    original_fn::<unsafe extern "C" fn(i16)>(IDX_SETUP_ON_NETWORK_PLAYER_JOINED)(player_id)
}

/// Comparer of `setup_on_network_player_left`: only the original runs (holidays and the match viewer).
pub(crate) unsafe extern "C" fn compare_setup_on_network_player_left() {
    original_fn::<unsafe extern "C" fn()>(IDX_SETUP_ON_NETWORK_PLAYER_LEFT)()
}

/// Comparer of `nation_matches_region_filter`: both run (it only reads) and the results must match.
pub(crate) unsafe extern "C" fn compare_nation_matches_region_filter(nation: usize, filter: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_NATION_MATCHES_REGION_FILTER)(nation, filter);
    if nation == 0 || filter == 0 {
        return r;
    }
    let n = nation_matches_region_filter(nation, filter);
    SCOREBOARD.record(r == n, || format!("nation_matches_region_filter(nation {:#x}, filter {:#x}): original={} rust={}", nation, filter, r, n));
    r
}

/// Comparer of `compare_nations_by_nationality`: both run (it only reads) and the results must match.
pub(crate) unsafe extern "C" fn compare_compare_nations_by_nationality(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_NATIONS_BY_NATIONALITY)(a, b);
    let n = compare_nations_by_nationality(a, b);
    SCOREBOARD.record(r == n, || format!("compare_nations_by_nationality({:#x}, {:#x}): original={} rust={}", a, b, r, n));
    r
}

/// Comparer of `get_game_title`: side by side over the title buffer (up to `G_NETWORK_GAME_NAME`) and the result.
pub(crate) unsafe extern "C" fn compare_get_game_title() -> usize {
    let o = original_fn::<unsafe extern "C" fn() -> usize>(IDX_GET_GAME_TITLE);
    let regions = [(G_SETUP_SCREEN_TITLE, 0x34)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || get_game_title());
    SCOREBOARD.record(equal && r == n, || format!("get_game_title: original={:#x} rust={:#x} differences at {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Rule: Setup.cpp's start date is a static Date built empty at program start.
///
/// `setup_static_init_start_date()`: `Date::Date` on 0xdd6ec8 and its destructor (`setup_static_destroy_start_date`,
/// empty) registered with `atexit`.
/// Original: `setup_static_init_start_date` (0x81b830, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_static_init_start_date() {
    exe::date_date_54a110(0xdd6ec8usize);
    exe::crt_atexit(0x81b850usize);
}

/// The start date's destructor: nothing.
/// Original: `setup_static_destroy_start_date` (0x81b850, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_static_destroy_start_date() {}

/// Rule: Setup.cpp's saved date is a static Date built empty at program start.
///
/// `setup_static_init_saved_date()`: `Date::Date` on 0xdd70a0 and its empty destructor registered with `atexit`.
/// Original: `setup_static_init_saved_date` (0x81b860, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_static_init_saved_date() {
    exe::date_date_54a110(0xdd70a0usize);
    exe::crt_atexit(0x81b880usize);
}

/// The saved date's destructor: nothing.
/// Original: `setup_static_destroy_saved_date` (0x81b880, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_static_destroy_saved_date() {}

/// Rule: when a new game loads Greece, each Greek division must have exactly its number of clubs (14): clubs past the
/// number go to the lower division placeholder, and missing places are filled with clubs of the nation that have no
/// division, play in the placeholder or Premier divisions, or play in a division of lower reputation.
///
/// `fix_division_club_count(comp_id, expected)`: the clubs in table order, stopping at the first national team
/// (`database_is_national_team`). First pass: a club whose `club::DIVISION` is the competition counts while the count
/// is below `expected`; past it the club's division becomes `KEY_CLUB_COMP_A_LOWER_DIVISION`'s competition and the
/// text "DB_ERROR: Found 1 more team in db for <name>" is written (C `sprintf`) into the screens' text buffer. With
/// fewer than `expected`: "DB_ERROR: Not enough teams in database for <name> (Found n, Expected m)". Second pass, while
/// the count is below `expected` (stopping at a national team): a club of the competition's nation (`club::NATION` ==
/// `competition::NATION`) whose division is none, the lower placeholder, the Premier placeholder
/// (`KEY_CLUB_COMP_A_PREMIER_DIVISION`) or one of lower `competition::REPUTATION` moves to the competition and counts.
/// The texts are never shown (only written to the buffer).
/// Original: `fix_division_club_count` (0x839920, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn fix_division_club_count(comp_id: i32, expected: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let expected = expected as i16;
    let comps = || read_usize(G_CLUB_COMP_TABLE);
    let comp_at = |id: u32| comps().wrapping_add(id.wrapping_mul(TEL36_CLUB_COMP_SIZE) as usize);
    let comp = comp_at(comp_id as u32);
    let mut found = 0i16;
    let mut club = read_usize(G_CLUB_TABLE);
    for _ in 0..read_i32(G_CLUB_COUNT) {
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let division = read_usize(club + club::DIVISION);
        if division != 0 && read_i32(division) == comp_id {
            if found < expected {
                found += 1;
            } else {
                covered!("tel36_12 one more club in the division");
                write_usize(club + club::DIVISION, comp_at(read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION)));
                format(JOB_TEXT, 0xa82578, comp_at(comp_id as u32) + competition::NAME);
            }
        }
        club += club::SIZE;
    }
    if found < expected {
        covered!("tel36_12 not enough clubs in the division");
        format(JOB_TEXT, 0xa82530, comp + competition::NAME, found as i32, expected as i32);
    }
    let mut club = read_usize(G_CLUB_TABLE);
    for _ in 0..read_i32(G_CLUB_COUNT) {
        if found >= expected || exe::database_is_national_team(club) != 0 {
            return;
        }
        let nation = read_u32(club + club::NATION);
        if nation != 0 && nation == read_u32(comp_at(comp_id as u32) + competition::NATION) {
            let division = read_usize(club + club::DIVISION);
            let takes = division == 0 || {
                let id = read_u32(division);
                id == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION) || id == read_u32(KEY_CLUB_COMP_A_PREMIER_DIVISION)
                    || read_i16(division + competition::REPUTATION) < read_i16(comp_at(comp_id as u32) + competition::REPUTATION)
            };
            if takes {
                found += 1;
                write_usize(club + club::DIVISION, comp_at(comp_id as u32));
            }
        }
        club += club::SIZE;
    }
}

/// Comparer for `setup_static_init_start_date`.
pub(crate) unsafe extern "C" fn compare_setup_static_init_start_date() {
    // simplification: the atexit registration does not run twice
    original_fn::<unsafe extern "C" fn()>(IDX_SETUP_STATIC_INIT_START_DATE)()
}

/// Comparer for `setup_static_destroy_start_date`.
pub(crate) unsafe extern "C" fn compare_setup_static_destroy_start_date() {
    original_fn::<unsafe extern "C" fn()>(IDX_SETUP_STATIC_DESTROY_START_DATE)()
}

/// Comparer for `setup_static_init_saved_date`.
pub(crate) unsafe extern "C" fn compare_setup_static_init_saved_date() {
    // simplification: the atexit registration does not run twice
    original_fn::<unsafe extern "C" fn()>(IDX_SETUP_STATIC_INIT_SAVED_DATE)()
}

/// Comparer for `setup_static_destroy_saved_date`.
pub(crate) unsafe extern "C" fn compare_setup_static_destroy_saved_date() {
    original_fn::<unsafe extern "C" fn()>(IDX_SETUP_STATIC_DESTROY_SAVED_DATE)()
}

/// Comparer for `fix_division_club_count`: the club table and the text buffer side by side.
pub(crate) unsafe extern "C" fn compare_fix_division_club_count(comp_id: i32, expected: u32) {
    let o = original_fn::<unsafe extern "C" fn(i32, u32)>(IDX_FIX_DIVISION_CLUB_COUNT);
    let reg = [(read_usize(G_CLUB_TABLE), read_i32(G_CLUB_COUNT).max(0) as usize * club::SIZE), (JOB_TEXT, 0x100)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(comp_id, expected), || fix_division_club_count(comp_id, expected));
    SCOREBOARD.record(same, || format!("fix_division_club_count({}, {}): differences {:?}", comp_id, expected as i16, first_differences(&mo, &mr)));
}

/// The first of the 16 human manager rows that is free (classification 0), from the last staff record back, or 0.
///
/// `find_free_human_manager_row()`. Without the staff table, the game's error box (line 0x39b3) and 0.
/// Original: `find_free_human_manager_row` (0x838c60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn find_free_human_manager_row() -> usize {
    if read_usize(G_STAFF_TABLE) == 0 {
        show_error_at(SETUP_CPP_FILE, 0x39b3);
        return 0;
    }
    covered!("mwg_4 free row");
    (0..16).map(|k| human_manager_row(k)).find(|s| s.classification() == 0).map_or(0, |s| s.addr()) // mwg4: a free row
}

/// Comparer for `find_free_human_manager_row`.
pub(crate) unsafe extern "C" fn compare_find_free_human_manager_row() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(IDX_FIND_FREE_HUMAN_MANAGER_ROW)();
    let n = find_free_human_manager_row();
    SCOREBOARD.record(r == n, || format!("find_free_human_manager_row() original={:#x} rust={:#x}", r, n));
    r
}

/// How many of the 16 human manager rows are in use (classification 1); with `network_only`, only those whose
/// manager is done on the screen manager (`exe::screenmanager_is_manager_done`).
///
/// `count_human_managers(network_only)`. Without the staff table, the game's error box (line 0x39d8) and 0.
/// Original: `count_human_managers` (0x838d10, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn count_human_managers(network_only: u32) -> u32 {
    if read_usize(G_STAFF_TABLE) == 0 {
        show_error_at(SETUP_CPP_FILE, 0x39d8);
        return 0;
    }
    covered!("mwg_4 count");
    (0..16)
        .map(|k| human_manager_row(k))
        .filter(|s| s.classification() == 1 && (network_only == 0 || exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, s.addr() as u32) != 0)) // mwg4: a human in use
        .count() as u32
}

/// Comparer for `count_human_managers`.
pub(crate) unsafe extern "C" fn compare_count_human_managers(network_only: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_COUNT_HUMAN_MANAGERS)(network_only);
    let n = count_human_managers(network_only);
    SCOREBOARD.record(r == n, || format!("count_human_managers({}) original={} rust={}", network_only, r, n));
    r
}

/// 1 when the current screen slot has a manager whose row is still being made (classification 5), else 0.
///
/// `is_current_manager_being_created()`.
/// Original: `is_current_manager_being_created` (0x838de0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn is_current_manager_being_created() -> u32 {
    covered!("mwg_4 being created");
    let s = current_screen_manager();
    (!s.is_null() && s.classification() == 5) as u32 // mwg4: being created
}

/// Comparer for `is_current_manager_being_created`.
pub(crate) unsafe extern "C" fn compare_is_current_manager_being_created() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_IS_CURRENT_MANAGER_BEING_CREATED)();
    let n = is_current_manager_being_created();
    SCOREBOARD.record(r == n, || format!("is_current_manager_being_created() original={} rust={}", r, n));
    r
}

/// The regions the player records touch: the 16 channels of records, their counts, the current and the next channel and
/// the two hooks.
pub(crate) fn scrman_records_regions(sm: usize) -> [(usize, usize); 4] {
    [(sm + screen_history::NETWORK_PLAYER, 0x10 * screen_history::CHANNEL_SIZE), (sm + screen_history::CHANNEL_RECORD_COUNTS, 0x20),
        (sm + screen_manager::CURRENT_SLOT, 4), (sm + SCRMAN_JOIN_HOOK, 8)]
}

/// The mould of the `create_*_comps` routines: fills the key nation entry `entry` from `spec`; returns 1, or 0 with a
/// null entry (the Setup.cpp "Error" box) or when the array cannot be allocated (the "Memory Allocation Error" box).
///
/// The season bounds are written; the count (`key_nation_comp_count`) goes to `KEY_NATION_RECORD_COMP_COUNT` and the
/// array (calloc) to `KEY_NATION_RECORD_COMPS` (a continental entry without a selected nation gets 0 in both and no
/// competition); each competition of the list (the extra ones only when `key_nation_extra_guard`) is `operator new`
/// and its constructor (year, the key's competition record), a failed `new` leaving the slot null, in consecutive
/// slots. Then `key_nation_record::SEASON_END_DAY`, `KEY_NATION_RECORD_SEASON_YEAR`,
/// `key_nation_record::SEASON_SPANS_YEAR` and the links. The original's SEH frame (which deletes an object whose
/// constructor throws) has no counterpart: the constructors do not throw.
pub(crate) unsafe fn key_nation_create_comps(entry: usize, spec: &KeyNationComps) -> u32 {
    if entry == 0 {
        setup_warning(spec.rows.0);
        return 0;
    }
    let weeks = key_nation_weeks(spec);
    let year = |y: EntryYear| entry_year(y, &weeks);
    let (start, end) = (spec.start, spec.end);
    write_u16(entry + KEY_NATION_RECORD_START_DAY, start.0);
    write_u8(entry + key_nation_record::START_MONTH, start.1);
    write_u16(entry + KEY_NATION_RECORD_START_YEAR, year(start.2) as u16);
    write_u16(entry + KEY_NATION_RECORD_START_WEEKDAY, start.3);
    write_u16(entry + KEY_NATION_RECORD_END_DAY, end.0);
    write_u8(entry + KEY_NATION_RECORD_END_MONTH, end.1);
    write_u16(entry + KEY_NATION_RECORD_END_YEAR, year(end.2) as u16);
    write_u16(entry + KEY_NATION_RECORD_END_WEEKDAY, end.3);
    match key_nation_comp_count(entry, spec) {
        None => {
            write_u32(entry + KEY_NATION_RECORD_COMP_COUNT, 0);
            write_usize(entry + KEY_NATION_RECORD_COMPS, 0);
        }
        Some(count) => {
            write_u32(entry + KEY_NATION_RECORD_COMP_COUNT, count);
            let (size, number) = spec.calloc.unwrap_or((read_u32(entry + KEY_NATION_RECORD_COMP_COUNT).wrapping_mul(4), 1));
            let comps = exe::crt_calloc(size, number);
            write_usize(entry + KEY_NATION_RECORD_COMPS, comps);
            if comps == 0 {
                save_error_box(spec.rows.1, spec.rows.2, 0x9a8108, 0x9a7ff8); // "Memory Allocation Error"
                return 0;
            }
            let (mut slot, mut guard) = (0usize, None::<bool>);
            for &(size, constructor, key, comp_year, extra) in spec.comps {
                if extra {
                    if !*guard.get_or_insert_with(|| key_nation_extra_guard(entry, spec)) {
                        continue;
                    }
                } else {
                    guard = None;
                }
                let p = exe::crt_operator_new(size);
                let object = if p == 0 {
                    0
                } else {
                    let construct: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(constructor);
                    construct(p, year(comp_year), key_comp_record(key))
                };
                write_usize(read_usize(entry + KEY_NATION_RECORD_COMPS) + slot * 4, object);
                slot += 1;
            }
        }
    }
    let e = KeyNationRecord(entry);
    e.set_season_end_day(key_nation_end_day(spec));
    write_u16(entry + KEY_NATION_RECORD_SEASON_YEAR, year(spec.season_year) as u16);
    e.set_season_spans_year(spec.spans);
    if let Some(v) = entry_link(spec.league) {
        e.set_league_comp(v);
    }
    if let Some(v) = entry_link(spec.cup) {
        write_usize(entry + KEY_NATION_RECORD_CUP_COMP, v);
    }
    1
}

/// The plan of a `create_*_comps` routine checked on the entry after the original ran (`result` its result; nothing to
/// check with a null entry or a failed allocation): the season bounds, the count, each slot's competition (a non-null
/// object's `league::COMPETITION` is the key's record; the extra ones by the same guard), the season end day, the
/// season year, the spans flag and the links.
pub(crate) unsafe fn key_nation_check_comps(name: &str, entry: usize, result: u32, spec: &KeyNationComps) {
    if entry == 0 || result == 0 {
        return;
    }
    let weeks = key_nation_weeks(spec);
    let year = |y: EntryYear| entry_year(y, &weeks) as u16 as u32;
    let mut wrong: Vec<String> = Vec::new();
    let mut check = |what: &str, got: u32, plan: u32| {
        if got != plan {
            wrong.push(format!("{} {:#x} (plan {:#x})", what, got, plan));
        }
    };
    let (start, end) = (spec.start, spec.end);
    check("start day", read_u16(entry + KEY_NATION_RECORD_START_DAY) as u32, start.0 as u32);
    check("start month", read_u8(entry + key_nation_record::START_MONTH) as u32, start.1 as u32);
    check("start year", read_u16(entry + KEY_NATION_RECORD_START_YEAR) as u32, year(start.2));
    check("start weekday", read_u16(entry + KEY_NATION_RECORD_START_WEEKDAY) as u32, start.3 as u32);
    check("end day", read_u16(entry + KEY_NATION_RECORD_END_DAY) as u32, end.0 as u32);
    check("end month", read_u8(entry + KEY_NATION_RECORD_END_MONTH) as u32, end.1 as u32);
    check("end year", read_u16(entry + KEY_NATION_RECORD_END_YEAR) as u32, year(end.2));
    check("end weekday", read_u16(entry + KEY_NATION_RECORD_END_WEEKDAY) as u32, end.3 as u32);
    let count = key_nation_comp_count(entry, spec);
    check("count", read_u32(entry + KEY_NATION_RECORD_COMP_COUNT), count.unwrap_or(0));
    let comps = read_usize(entry + KEY_NATION_RECORD_COMPS);
    if count.is_none() {
        check("array", comps as u32, 0);
    } else if comps != 0 {
        let (mut slot, mut guard) = (0usize, None::<bool>);
        for &(_, _, key, _, extra) in spec.comps {
            if extra {
                if !*guard.get_or_insert_with(|| key_nation_extra_guard(entry, spec)) {
                    continue;
                }
            } else {
                guard = None;
            }
            let object = League(read_usize(comps + slot * 4));
            if !object.is_null() {
                check("competition of a slot", object.competition().addr() as u32, key_comp_record(key) as u32);
            }
            slot += 1;
        }
    }
    check("season end day", KeyNationRecord(entry).season_end_day() as u32, key_nation_end_day(spec) as u32);
    check("season year", read_u16(entry + KEY_NATION_RECORD_SEASON_YEAR) as u32, year(spec.season_year));
    check("spans year", KeyNationRecord(entry).season_spans_year() as u32, spec.spans as u32);
    if let Some(v) = entry_link(spec.league) {
        check("league comp", KeyNationRecord(entry).league_comp() as u32, v as u32);
    }
    if let Some(v) = entry_link(spec.cup) {
        check("cup comp", read_usize(entry + KEY_NATION_RECORD_CUP_COMP) as u32, v as u32);
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("{}(entry {:#x}): {}", name, entry, wrong.join(", ")));
}

/// The id of the item the player clicked on the current screen: the item's id in the current human's display
/// (`display::ITEM_ID`, +0x18c × item), the default command id `G_CURRENT_COMMAND_ID` when it is 0 or when there is no
/// item (−1). Read the way the Setup handlers read it (the id as a word).
pub(crate) unsafe fn setup_clicked_id(item: i16) -> u16 {
    let id = if item == -1 {
        0
    } else {
        read_u16(current_human_display() + item as isize as usize * 0x18c + display::ITEM_ID)
    };
    if id == 0 { read_u16(G_CURRENT_COMMAND_ID) } else { id }
}

/// The game's colour word at `a`, zero-extended (the original pushes it in a register whose high word is leftover).
pub(crate) unsafe fn setup_colour(a: usize) -> usize {
    read_u16(a) as usize
}

/// The Setup screens' title: "Championship Manager 2001/02" (langlib) into `G_SETUP_SCREEN_TITLE`.
pub(crate) unsafe fn setup_format_title() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(G_SETUP_SCREEN_TITLE, SETUP_TITLE_TEXT);
}

/// The title bar of a Setup screen: the title (`setup_format_title`) in red with pale grey text, the menu bar (mode 4)
/// and the translated subtitle `subtitle` (langlib into the shared text).
pub(crate) unsafe fn setup_screen_header(subtitle: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    setup_format_title();
    exe::screenmanager_add_screen_title(SCREEN_MANAGER, setup_colour(G_GUI_COLOUR_RED) as u32, setup_colour(G_GUI_COLOUR_PALE_GREY) as u32, G_SETUP_SCREEN_TITLE);
    exe::menubar_build(4, 0);
    text(SETUP_TEXT, subtitle);
    exe::screenmanager_add_screen_subtitle(SCREEN_MANAGER, SETUP_TEXT, 0);
}

/// The Setup error box at `row` followed by the fatal cleanup and exit(code) (the original's exit(-1), and exit(0) once).
pub(crate) unsafe fn setup_fatal_exit(row: Option<u32>, code: i32) -> ! {
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    if let Some(r) = row {
        setup_warning(r);
    }
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exit(code)
}

/// Opens the screen `build`/`input` on the screen manager with flag `flag`; on success zeroes its params 0..`params`.
/// Returns whether it opened.
pub(crate) unsafe fn setup_open_screen(build: usize, input: usize, flag: u32, params: u32) -> bool {
    if exe::screenmanager_open_screen(SCREEN_MANAGER, build, input, flag, 0usize, 0) == 0 {
        return false;
    }
    for k in 0..params {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, k, 0usize, 0);
    }
    true
}

/// The Setup "Please Confirm" box with one message: the title (`setup_format_title`), the translated heading into
/// `heading` and the translated text `body` (with one argument) into the shared text; no buttons, pale grey text on dark
/// red, code 0.
pub(crate) unsafe fn setup_confirm_message(heading_buffer: &mut [u8; 0x100], heading: usize, body: usize, arg: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    setup_format_title();
    let (dark_red, pale) = (setup_colour(G_GUI_COLOUR_DARK_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
    let b = text(SETUP_TEXT, body, arg);
    let h = text(heading_buffer.as_mut_ptr() as usize, heading);
    exe::confirm_push(G_SETUP_SCREEN_TITLE, h, b, 0, (pale) as usize as u32, (dark_red) as usize as u32, 0);
}

/// Loads the saved game or quick start `path` chosen in Setup Game (`mode` 4 quick start, 5 saved game) and returns the
/// screen result.
///
/// Quick start: `G_NEW_GAME_SETUP_DONE_FLAG` = 1; saved game: the file name (`_splitpath`) becomes the Config's save
/// name (`config::SAVE_NAME`) and `G_NETWORK_SETUP_HOST_FLAG` = 1. Then `load_game_data`; `delete_path` frees `path`
/// right after (the dialog's copy). A failed load with a game in progress opens the disk full box, with a session started
/// the no memory box (reason 0 after a quick start, 1 otherwise; through the box functions when `boxes`, otherwise opened
/// here with their param 0), and any other failure is fatal (`fatal_row`). A good load applies the Config
/// (`exe::config_apply_and_save`) and, for a quick start with `clear_save_flag` (Setup Game, not the network name
/// screen), clears `G_GAME_HAS_SAVE_FILE_FLAG`.
pub(crate) unsafe fn setup_load_chosen_game(path: usize, mode: u32, delete_path: bool, boxes: bool, fatal_row: u32, clear_save_flag: bool) -> Option<i32> {
    let load: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_LOAD_GAME_DATA);
    let disk_full: unsafe extern "C" fn(u32) = core::mem::transmute(FN_OPEN_DISK_FULL_BOX);
    let no_memory: unsafe extern "C" fn(u32) = core::mem::transmute(FN_OPEN_NO_MEMORY_BOX);
    if mode == 4 {
        write_u32(G_NEW_GAME_SETUP_DONE_FLAG, 1);
    } else {
        exe::crt_strtok(path, 0usize, 0usize, (read_usize(G_CONFIG) + config::SAVE_NAME) as *mut u8, core::ptr::null_mut());
        write_u32(G_NETWORK_SETUP_HOST_FLAG, 1);
    }
    let r = load(path);
    if delete_path {
        exe::crt_operator_delete(path);
    }
    if r == 0 {
        let reason = (read_u32(G_NEW_GAME_SETUP_DONE_FLAG) == 0) as u32;
        let boxes_fns = [(G_GAME_IN_PROGRESS_FLAG, 0x826aa0usize, 0x826c20usize), (G_GAME_SESSION_STARTED_FLAG, 0x826d20, 0x826e60)];
        for (k, &(flag, build, input)) in boxes_fns.iter().enumerate() {
            if read_u32(flag) == 0 {
                continue;
            }
            if boxes {
                if k == 0 { disk_full(reason) } else { no_memory(reason) }
            } else if exe::screenmanager_open_screen(SCREEN_MANAGER, build, input, read_u32(G_INTERACTIVE_MODE_FLAG), 0usize, 0) != 0 {
                exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, reason as usize, 0);
            }
            return Some(-4);
        }
        setup_fatal_exit(Some(fatal_row), -1);
    }
    exe::config_apply_and_save(read_usize(G_CONFIG));
    if mode == 4 && clear_save_flag {
        write_u32(G_GAME_HAS_SAVE_FILE_FLAG, 0);
    }
    None
}

/// Free memory in sevenths of a megabyte, at least 1: the original's limit on the leagues loaded (`os_get_free_memory_mb`
/// / 7, signed, 0 becoming 1).
pub(crate) unsafe fn setup_league_memory_limit() -> i32 {
    let m = exe::os_get_free_memory_mb() / 7;
    if m == 0 { 1 } else { m }
}

/// The league entry (0x48 bytes in `G_LEAGUE_TABLE`) at position `k` of the key nation order `order`.
pub(crate) unsafe fn setup_league_entry(order: usize, k: usize) -> usize {
    G_LEAGUE_TABLE + (read_i8(order + k) as isize * 0x48) as usize
}

/// The memory check before one more league is loaded: unless the command line says "-dontcheckram" or the memory
/// check is off (`G_MEMORY_CHECK_ENABLED`), with as many leagues to load (`G_NATIONS_TO_LOAD_COUNT`) as the memory limit
/// (`setup_league_memory_limit`) the "Not enough memory" box opens (`exe::gui_show_message_box`) and it says no.
pub(crate) unsafe fn setup_room_for_one_more_league() -> bool {
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let loaded = read_i32(G_NATIONS_TO_LOAD_COUNT);
    let limit = setup_league_memory_limit();
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa80484) == 0 && read_u32(G_MEMORY_CHECK_ENABLED) != 0 && loaded >= limit {
        covered!("tel11_1 select leagues memory box");
        let mut title = [0u8; 0x100];
        text(title.as_mut_ptr() as usize, 0xa80470); // "Not enough memory"
        text(SETUP_TEXT, 0xa80440); // "Not enough memory to select that many leagues"
        exe::gui_show_message_box(title.as_ptr() as usize, SETUP_TEXT, 0);
        return false;
    }
    true
}

/// The main nation `G_MAIN_NATION` is cleared and becomes the first entry of the league table (in table order) whose
/// nation's flags pass `chosen`.
pub(crate) unsafe fn setup_pick_main_nation(chosen: impl Fn(u8) -> bool) {
    write_usize(G_MAIN_NATION, 0);
    let mut e = G_LEAGUE_TABLE;
    while e < G_KEY_NATION_ACTIVE_COUNT && read_usize(G_MAIN_NATION) == 0 {
        let n = Nation(read_usize(e));
        if !n.is_null() && chosen(n.league_selected()) {
            write_usize(G_MAIN_NATION, n.addr());
        }
        e += 0x48;
    }
}

/// Adds `d` to the counter at `a`.
pub(crate) unsafe fn setup_count(a: usize, d: i32) {
    write_i32(a, read_i32(a).wrapping_add(d));
}

/// Creates the game chosen on Select League(s) or Select Start Season (`G_NEW_GAME_SETUP_DONE_FLAG` = 1,
/// `exe::create_game`(0)). A failure opens the disk full box (game in progress) or the no memory box (session started),
/// each with param 0 = 0, or is fatal (`fatal_row`). Then, unless it is a network game (`G_NETWORK_GAME_NAMED_FLAG`:
/// `setup_create_or_join_network_game`(0, `G_NETWORK_GAME_NAME`), whose nonzero answer is returned), Manager Status
/// opens with params 0..4 zeroed and a new manager is added; −4.
pub(crate) unsafe fn setup_create_chosen_game(fatal_row: u32) -> i32 {
    let add_manager: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_ADD_HUMAN_MANAGER);
    write_u32(G_NEW_GAME_SETUP_DONE_FLAG, 1);
    if exe::create_game(0) == 0 {
        let flag = read_u32(G_INTERACTIVE_MODE_FLAG);
        if read_u32(G_GAME_IN_PROGRESS_FLAG) != 0 {
            setup_open_screen_with_flag(0x826aa0, 0x826c20, flag);
            return -4;
        }
        if read_u32(G_GAME_SESSION_STARTED_FLAG) != 0 {
            setup_open_screen_with_flag(0x826d20, 0x826e60, flag);
            return -4;
        }
        setup_fatal_exit(Some(fatal_row), -1);
    }
    if read_u32(G_NETWORK_GAME_NAMED_FLAG) != 0 {
        let r = exe::setup_create_or_join_network_game(0, G_NETWORK_GAME_NAME);
        if r != 0 {
            return r;
        }
    }
    setup_open_screen(0x81ff10, 0x820950, 0, 5);
    add_manager();
    -4
}

/// The nation attached to item `item` of the current display (`display::ITEM_VALUE`), 0 without an item.
pub(crate) unsafe fn setup_clicked_value(item: i16) -> usize {
    if item == -1 { 0 } else { read_usize(current_human_display() + item as isize as usize * 0x18c + display::ITEM_VALUE) }
}

/// The regions a Setup screen's answer changes when it opens nothing: the screen manager and the current screen's
/// params (the list's scroll row), the league flags of every nation, the three league counters, the two options and the
/// Setup flags.
pub(crate) unsafe fn setup_answer_regions() -> Vec<(usize, usize)> {
    let mut r = vec![(SCREEN_MANAGER, 0x3840)];
    r.extend(screen_param_regions());
    let nations = read_usize(G_NATION_TABLE);
    let count = read_i32(G_NATION_COUNT);
    if nations != 0 && count > 0 {
        r.push((nations, count as usize * nation::SIZE));
    }
    r.push((G_NATIONS_TO_LOAD_COUNT, 12));
    r.push((G_USE_REAL_PLAYERS, 1));
    r.push((G_ATTRIBUTE_MASKING_ENABLED, 1));
    r.push((G_NEW_GAME_SETUP_DONE_FLAG, 4));
    r
}

/// Side by side over `setup_answer_regions`, with the screen's name in the message.
pub(crate) unsafe fn compare_setup_answer(screen: &str, item: u32, original: impl FnOnce() -> i32, rust: impl FnOnce() -> i32) -> i32 {
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&setup_answer_regions(), original, rust);
    SCOREBOARD.record(equal && r == n, || format!("{} answer to item {}: original={} rust={} differences at {:?}", screen, item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Sends the network game list request ("CM3 LIST SERVER REQ", with its terminator, port 0x1a0b) on the network
/// object `net`, after zeroing its count of games found.
pub(crate) unsafe fn setup_request_network_games(net: usize) {
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TCPIP_SEND_DATAGRAM);
    write_u16(net + NETWORK_GAME_COUNT, 0);
    let n = core::ffi::CStr::from_ptr(0xa649d4 as *const core::ffi::c_char).to_bytes_with_nul().len() as u32;
    send(net, 0xa649d4, n, 0x1a0b, 0, 0, 0);
}

/// The human managers among the last 16 staff (`staff::CLASSIFICATION` = 1), only the connected ones
/// (`exe::screenmanager_is_manager_done`) when `connected`; without staff, the Setup error box (0x39d8) and 0.
pub(crate) unsafe fn setup_count_human_managers(connected: bool) -> i32 {
    let staff = read_usize(G_STAFF_TABLE);
    if staff == 0 {
        setup_warning(0x39d8);
        return 0;
    }
    let count = read_i32(G_STAFF_COUNT) as isize;
    let mut n = 0;
    for k in 1..=16isize {
        let s = Staff((staff as isize + (count - k) * staff::SIZE as isize) as usize);
        if s.classification() == 1 && (!connected || exe::screenmanager_is_manager_done(SCREEN_MANAGER, s.addr() as u32) != 0) {
            n += 1;
        }
    }
    n
}

/// The C strcmp of two texts: whether they differ.
pub(crate) unsafe fn setup_texts_differ(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return true;
        }
        if x == 0 {
            return false;
        }
        k += 1;
    }
}

/// The Setup "Please Confirm" box: the title (`setup_format_title`), the heading `heading` translated into a local
/// buffer, the text `body` already in the shared text, no buttons, `text_colour` on `colour`, result code `code`.
pub(crate) unsafe fn setup_confirm_box(heading: usize, text_colour: usize, colour: usize, code: u32) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut buffer = [0u8; 0x100];
    let h = text(buffer.as_mut_ptr() as usize, heading);
    exe::confirm_push(G_SETUP_SCREEN_TITLE, h, SETUP_TEXT, 0, (text_colour) as usize as u32, (colour) as usize as u32, code);
}

/// The bottom area of the Setup confirmation boxes (0x64, 0x22b to 0x316, 0x24e; columns 3, 1) with two buttons:
/// `left` (id `left_id`) and `right` (id `right_id`), translated.
pub(crate) unsafe fn setup_box_buttons(left: usize, left_id: usize, right: usize, right_id: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let mut columns: [u8; 2] = [3, 1];
    let (grey, pale) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY));
    let a = area(current_human_display(), 0x64, 0x22b, 0x316, 0x24e, 2, columns.as_mut_ptr() as usize, 1, 0, 1, 0, usize::MAX);
    text(SETUP_TEXT, left);
    button(SCREEN_MANAGER, a, 0, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, left_id, 0);
    text(SETUP_TEXT, right);
    button(SCREEN_MANAGER, a, 1, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, right_id, 0);
}

/// The title bar of a Setup box: the manager's name (`exe::database_format_staff_name` into the shared text) in red
/// with pale grey text, the menu bar (mode 4) and the subtitle `subtitle`.
pub(crate) unsafe fn setup_box_header(manager: usize, subtitle: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    exe::database_format_staff_name(manager, SETUP_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(SCREEN_MANAGER, setup_colour(G_GUI_COLOUR_RED) as u32, setup_colour(G_GUI_COLOUR_PALE_GREY) as u32, SETUP_TEXT);
    exe::menubar_build(4, 0);
    text(SETUP_TEXT, subtitle);
    exe::screenmanager_add_screen_subtitle(SCREEN_MANAGER, SETUP_TEXT, 0);
}

/// The question of a Setup box (already in the shared text) as a text area (0x6e, 0x91 to 0x30c, 0x217, font 4, pale
/// grey).
pub(crate) unsafe fn setup_box_question() {
    let text_area: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32, usize, usize, ...) -> u32 = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    text_area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, setup_colour(G_GUI_COLOUR_PALE_GREY), SETUP_TEXT, 0);
}

/// Resumes control of `manager`: no longer computer controlled nor on holiday; when the current slot has no manager
/// he becomes its manager (and the player join/leave hooks are set to 0x838a80/0x838c00), otherwise a record is added
/// for him (`ScreenManager::add_manager_record`). −9.
pub(crate) unsafe fn setup_resume_manager(manager: usize) -> i32 {
    let computer: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_COMPUTER_CONTROLLED);
    computer(read_usize(G_HUMAN_MANAGER), 0, manager);
    exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, manager);
    let slot = G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300;
    if read_usize(slot) == 0 {
        write_usize(slot, manager);
        write_usize(G_SCREEN_MANAGER_PLAYER_JOINED_CALLBACK, NETWORK_JOIN_CALLBACK);
        write_usize(G_SCREEN_MANAGER_PLAYER_LEFT_CALLBACK, NETWORK_LEAVE_CALLBACK);
    } else {
        exe::screenmanager_add_manager_record(SCREEN_MANAGER, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK, manager);
    }
    -9
}

/// The "Already Controlled" box for a manager who already has a channel (`exe::screenmanager_find_manager_channel` ≥ 0):
/// "<manager> has already resumed control.", red with pale grey text, code 0x3b. Returns whether it opened.
pub(crate) unsafe fn setup_already_resumed(manager: usize) -> bool {
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if (exe::screenmanager_find_manager_channel(SCREEN_MANAGER, manager) as i16) <= -1 {
        return false;
    }
    let mut name = [0u8; 0xc8];
    exe::database_format_staff_name(manager, name.as_mut_ptr() as usize, 0xc8, 1);
    let (red, pale) = (setup_colour(G_GUI_COLOUR_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
    setup_format_title();
    text(SETUP_TEXT, 0xa809b0, name.as_ptr()); // "<%s - manager> has already resumed control."
    setup_confirm_box(0xa80900, pale, red, 0x3b); // "Already Controlled"
    true
}

/// Builds the Game Settings box over the screen.
///
/// The option colour `G_SETTINGS_OPTION_COLOUR` is made once (`graphics_rgb_to_pixel`(0, 0xc0, 0, 0), guard bit 0 of
/// `G_SETTINGS_STATICS_GUARD`). Param 0 is the Config copy, param 1 the origin (3: read-only lists; 2 and 3: the
/// database and match store rows cannot change). It counts the languages available (`langlib_is_language_available`,
/// 19), clears the background (`background_set_picture`(0)) and, with an origin, covers the screen with an empty area.
/// The box is 500 pixels high (0x1db with fewer than two languages), centred in 600: its area (0x91 to 0x2e8, colour
/// `graphics_rgb_to_pixel`(0, 0x38, 0, 0)), the title "Game Settings" (black, font 4), the version ("Version <%s>" from
/// `utils_get_version_string`, with the build date and time "Nov 21 2002" "13:10:31" in English), Reset (id 0x27) and,
/// at the bottom, Cancel (0x28) and Ok (0x29) in two columns (3, 1); a pitch drawing (`draw_pitch`) under the rows. The
/// rows go in six areas at the same place (0xaa to 0x2cf), the first, third with the columns 0xc, 9, 1, 5, 1 and the
/// others 3, 1, 1, 1, 1, 15 rows (14 with one language), all row heights 1:
/// - (with two or more languages) Language: the current one ("  %s" of its name, yellow) and the drop-down of the
///   available ones (id 0x2a with the language; greyed and without id when not English, a database is loaded and it has
///   no language file);
/// - Currency: the current one and the 26 currencies (`cash_get_currency_name`, id 0x2b with the index);
/// - Fonts: the current font (`font_get_mode_name` of `Config::get_font_index`, yellow on yellow) and, unless read-only,
///   the drop-down: Traditional (id 0x3d, −1) and Futuristic (−2) unless the language is 0xb or 0xc, a separator and up
///   to 20 font files allowed for the language (`is_font_allowed_for_language`, id 0x3d with the index);
/// - the on/off rows (`settings_toggle`, id 0x2c with the bit): Wages (Weekly 0x200000, Monthly 0x400000, Yearly
///   0x800000), Screen Mode (Full 8, Windowed 0x10), Background Changes (On 0x20, Off 0x40), Sound (On 0x80, Off
///   0x100), Database (Normal 1, Minimum 2, Maximum 4) and Matches Stored (10000 0x200, 5000 0x400, 20000 0x800);
/// - Auto Save: the drop-down Never (id 0x2c, 0x1000) and every 7, 14, 30, 90, 180 and 360 days (id 0x2d with the days)
///   and the current one: the last choice that matches (Never with bit 0x1000; the interval with bit 0x2000 and
///   `CONFIG_AUTO_SAVE_DAYS`); DEFECT of the original: with none, the name buffer keeps its last text (a font name or
///   a currency);
/// - Save Compressed (Yes 0x4000, No 0x8000), Save Safely (Yes 0x20000000, a button only when 0x40000000 is set and
///   0x20000000 is not; No 0x40000000), Flashing Text (Off 0x10000, On 0x20000), Background Matches (Normal 0x40000,
///   Off 0x80000, High 0x100000) and Print To (Printer 0x1000000, File 0x2000000, 0x4000000).
/// Original: `game_settings_box_build` (0x823950, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_settings_box_build() {
    covered!("tel12_1 build game settings");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let version: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let pitch: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, usize, u32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    let currency_name: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CASH_GET_CURRENCY_NAME);
    let font_index: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONFIG_GET_FONT_INDEX);
    let font_name: unsafe extern "C" fn(u32, usize, u32) = core::mem::transmute(FN_FONT_GET_MODE_NAME);
    let font_count: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_FONT_GET_FILE_COUNT);
    let font_allowed: unsafe extern "C" fn(u32, usize) -> u32 = core::mem::transmute(FN_IS_FONT_ALLOWED_FOR_LANGUAGE);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    if read_u8(G_SETTINGS_STATICS_GUARD) & 1 == 0 {
        write_u8(G_SETTINGS_STATICS_GUARD, read_u8(G_SETTINGS_STATICS_GUARD) | 1);
        write_u16(G_SETTINGS_OPTION_COLOUR, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let mut buttons: [u8; 7] = [3, 1, 0xc, 9, 1, 5, 1]; // the bottom buttons' 2 columns, then the list columns
    let mut toggles: [u8; 5] = [3, 1, 1, 1, 1];
    let mut heights = [1u8; 15];
    let mut rows = 0xfu32;
    let config = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    let origin = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 1) as i32;
    let mut languages = 0i8;
    for k in 0..0x13 {
        if exe::langlib_is_language_available(k) != 0 {
            languages = languages.wrapping_add(1);
        }
    }
    exe::background_set_picture(0);
    if origin != 0 {
        add_area(current_human_display(), 0, 0, 0x31f, 0x257, 1, 0, 1, 0, 2, 0, usize::MAX);
    }
    let height = if languages >= 2 { 0x1f4 } else { 0x1db };
    let top = (0x258 - height) / 2;
    let bottom = height + top - 1;
    let background = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0) as usize;
    add_area(current_human_display(), 0x91, top as usize, 0x2e8, bottom as usize, 1, 0, 1, 0, 0x30, background, usize::MAX);
    let (grey, pale, black, silver) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_BLACK), setup_colour(G_GUI_COLOUR_SILVER));
    text(SETUP_TEXT, 0xa5a46c); // "Game Settings"
    let y = top + 0xa;
    (exe::screenmanager_add_text_item(SCREEN_MANAGER, (0x9b) as i32 as u32, (y) as i32 as u32, (0x2de) as i32 as u32, (y + 0x2d) as i32 as u32, 0x30, (black) as usize as u32, 0xc, 4, (pale) as usize as u32, (SETUP_TEXT) as usize as u32, 1) as usize);
    let y = y + 0x37;
    if exe::langlib_get_language() == 0 {
        let v = version(0xa80e4c, 0xa80e58);
        text(SETUP_TEXT, 0xa80e04, v, 0xa80e4cusize, 0xa80e58usize); // "Version: <%s> - Build: <%s> <%s>", "Nov 21 2002", "13:10:31"
    } else {
        let v = exe::utils_get_version_string() as usize;
        text(SETUP_TEXT, 0xa80dec, v); // "Version <%s - version>"
    }
    (exe::screenmanager_add_text_item(SCREEN_MANAGER, (0xa5) as i32 as u32, (y) as i32 as u32, (0x22b) as i32 as u32, (y + 0x14) as i32 as u32, 1, (0) as usize as u32, 1, 1, (silver) as usize as u32, (SETUP_TEXT) as usize as u32, 1) as usize);
    text(SETUP_TEXT, 0xa7e768); // "Reset"
    exe::screenmanager_add_button_item(SCREEN_MANAGER, 0x261, y, 0x2de, y + 0x14, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0x27, 0);
    let low = bottom - 0xa;
    let bottom_area = add_area(current_human_display(), 0x9b, (low - 0x1e) as usize, 0x2de, low as usize, 2, buttons.as_mut_ptr() as usize, 1, 0, 1, 0, usize::MAX);
    text(SETUP_TEXT, 0x98a1a0); // "Cancel"
    exe::screenmanager_add_area_button(SCREEN_MANAGER, bottom_area, 0, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x28, 0);
    text(SETUP_TEXT, 0x990f7c); // "Ok"
    exe::screenmanager_add_area_button(SCREEN_MANAGER, bottom_area, 1, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x29, 0);
    let (mut left, mut upper, mut right, mut lower) = (0i32, 0i32, 0i32, 0i32);
    pitch(0xa5, y + 0x1e, 0x2d4, low - 0x32, &mut left, &mut upper, &mut right, &mut lower, 1, bottom_area, 0x40800000);
    let (list_top, list_bottom) = ((y + 0x23) as usize, (low - 0x37) as usize);
    if languages == 1 {
        rows = 0xe;
    }
    let list_area = |columns: *mut u8, rows: u32, heights: *mut u8| add_area(current_human_display(), 0xaa, list_top, 0x2cf, list_bottom, 5, columns as usize, rows as usize, heights as usize, 1, 0, usize::MAX);
    let cfg = |offset: usize| read_u32(config + offset);
    let read_only = origin == 3;
    let fixed = origin == 2 || origin == 3;
    let mut name = [0u8; 0xc8];
    let mut area = list_area(buttons.as_mut_ptr().add(2), rows, heights.as_mut_ptr());
    let mut row = 0u32;
    // Language
    if languages > 1 {
        settings_label(area, 0, 0xa7ffcc); // " Language"
        let menu = settings_dropdown(area, 0, read_only);
        for k in 0..0x13u32 {
            if exe::langlib_is_language_available(k) == 0 {
                continue;
            }
            copy_text(name.as_mut_ptr(), read_usize(SETTINGS_LANGUAGE_NAMES + 4 * k as usize) as *const u8);
            if k == cfg(CONFIG_LANGUAGE) {
                settings_current(area, 0, &name);
            }
            if read_only {
                continue;
            }
            if k == 0 || read_usize(G_NATION_TABLE) == 0 || exe::database_language_file_exists(k) != 0 {
                settings_choice(menu, name.as_ptr() as usize, 1, 0x2a, k as usize);
            } else {
                settings_choice(menu, name.as_ptr() as usize, 0x21, 0, 0);
            }
        }
        row = 1;
    }
    // Currency
    settings_label(area, row, 0xa80de0); // " Currency"
    let menu = settings_dropdown(area, row, read_only);
    for k in 0..0x1au8 {
        currency_name(name.as_mut_ptr() as usize, 0xc8, k as u32);
        if k == read_u8(config + CONFIG_CURRENCY) {
            settings_current(area, row, &name);
        }
        if !read_only {
            settings_choice(menu, name.as_ptr() as usize, 1, 0x2b, k as usize);
        }
    }
    // Fonts
    settings_label(area, row + 1, 0xa80db8); // " Fonts<%s - COMMENT - game settings>"
    let b = exe::screenmanager_add_area_button(SCREEN_MANAGER, area, 2, row + 1, 0x30, black, 0xc, 1, pale, 0, 0, 0);
    let menu = add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x100030, setup_colour(G_GUI_COLOUR_GREEN), b);
    let language = cfg(CONFIG_LANGUAGE);
    font_name(font_index(config), SETUP_TEXT, 0x7d0);
    let yellow = setup_colour(G_GUI_COLOUR_YELLOW);
    exe::screenmanager_add_area_text(SCREEN_MANAGER, area, 1, row + 1, 1, yellow, 1, 2, yellow, SETUP_TEXT);
    if !read_only {
        let named_fonts = language != 0xb && language != 0xc;
        if named_fonts {
            text(SETUP_TEXT, 0x9cc33c, 0xad9c64usize); // "Traditional<%s - COMMENT - fonts>"
            settings_choice(menu, SETUP_TEXT, 1, 0x3d, usize::MAX);
            text(SETUP_TEXT, 0x9cc318, 0xad9c64usize); // "Futuristic<%s - COMMENT - fonts>"
            settings_choice(menu, SETUP_TEXT, 1, 0x3d, usize::MAX - 1);
        }
        let count = font_count().min(0x14);
        if count > 0 {
            if named_fonts {
                let (green, black) = (setup_colour(G_GUI_COLOUR_GREEN), setup_colour(G_GUI_COLOUR_BLACK));
                item(current_human_display(), 1, 0, 0, 0, 0, 0, 0, 0x1000010, green, yellow, 1, 1, black, 0, 0, 0, 0, menu);
            }
            for k in 0..count as u32 {
                font_name(k, name.as_mut_ptr() as usize, 0xc8);
                if font_allowed(language, name.as_ptr() as usize) != 0 {
                    settings_choice(menu, name.as_ptr() as usize, 1, 0x3d, k as usize);
                }
            }
        }
    }
    // Wages .. Matches Stored
    area = list_area(toggles.as_mut_ptr(), rows, heights.as_mut_ptr());
    let r = row + 2;
    settings_label(area, r, 0xa80db0); // "Wages"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x200000, 0xa80da8, false); // "Weekly"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x400000, 0xa80da0, false); // "Monthly"
    settings_toggle_text(area, 3, r, cfg(CONFIG_OPTIONS), 0x800000, 0xa80d98, false); // "Yearly"
    let r = row + 3;
    settings_label(area, r, 0xa80d88); // "Screen Mode"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 8, 0xa80d60, false); // "Full<%s - COMMENT - full screen mode>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x10, 0xa80d54, false); // "Windowed"
    let r = row + 4;
    settings_label(area, r, 0xa80d40); // "Background Changes"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x20, 0xa80d18, false); // "On<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x40, 0xa80cf0, false); // "Off<..>"
    let r = row + 5;
    settings_label(area, r, 0xa80ce8); // "Sound"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x80, 0xa80ccc, false); // "On<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x100, 0xa80cb0, false); // "Off<..>"
    let r = row + 6;
    settings_label(area, r, 0xa80ca4); // "Database"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 1, 0xa80c78, fixed); // "Normal<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 2, 0xa80c48, fixed); // "Minimum<..>"
    settings_toggle_text(area, 3, r, cfg(CONFIG_OPTIONS), 4, 0xa80c40, fixed); // "Maximum"
    let r = row + 7;
    settings_label(area, r, 0xa80c30); // "Matches Stored"
    for (column, bit, number) in [(1u32, 0x200u32, 0xa80c28usize), (2, 0x400, 0xa80c20), (3, 0x800, 0xa80c18)] {
        format(SETUP_TEXT, number); // "10000", "5000", "20000"
        settings_toggle(area, column, r, cfg(CONFIG_OPTIONS), bit, fixed);
    }
    // Auto Save
    area = list_area(buttons.as_mut_ptr().add(2), rows, heights.as_mut_ptr());
    let r = row + 8;
    settings_label(area, r, 0xa80c0c); // "Auto Save"
    let menu = settings_dropdown(area, r, read_only);
    text(SETUP_TEXT, 0xa80c04); // "Never"
    if !read_only {
        settings_choice(menu, SETUP_TEXT, 1, 0x2c, 0x1000);
    }
    if cfg(CONFIG_OPTIONS) & 0x1000 != 0 {
        copy_text(name.as_mut_ptr(), SETUP_TEXT as *const u8);
    }
    for (label, days) in [(0xa80bf8usize, 7i16), (0xa80be8, 0xe), (0xa80bdc, 0x1e), (0xa80bc8, 0x5a), (0xa80bb4, 0xb4), (0xa80ba8, 0x168)] {
        text(SETUP_TEXT, label); // "Every Week" .. "Every Year"
        if !read_only {
            settings_choice(menu, SETUP_TEXT, 1, 0x2d, days as usize);
        }
        if cfg(CONFIG_OPTIONS) & 0x2000 != 0 && read_i16(config + CONFIG_AUTO_SAVE_DAYS) == days {
            copy_text(name.as_mut_ptr(), SETUP_TEXT as *const u8);
        }
    }
    settings_current(area, r, &name);
    // Save Compressed
    area = list_area(toggles.as_mut_ptr(), rows, heights.as_mut_ptr());
    let r = row + 9;
    settings_label(area, r, 0xa80b94); // "Save Compressed"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x4000, 0x98e8d0, false); // "Yes"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x8000, 0x98e8cc, false); // "No"
    // Save Safely
    area = list_area(toggles.as_mut_ptr(), rows, heights.as_mut_ptr());
    let r = row + 10;
    settings_label(area, r, 0xa80b84); // "Save Safely"
    text(SETUP_TEXT, 0x98e8d0); // "Yes"
    let options = cfg(CONFIG_OPTIONS);
    // the Yes button only when No is set and Yes is not (the original's test); otherwise the selected text
    let yes_set = if options & 0x20000000 == 0 && options & 0x40000000 != 0 { 0 } else { 0x20000000 };
    settings_toggle(area, 1, r, yes_set, 0x20000000, false);
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x40000000, 0x98e8cc, false); // "No"
    // Flashing Text, Background Matches, Print To
    area = list_area(toggles.as_mut_ptr(), rows, heights.as_mut_ptr());
    let r = row + 0xb;
    settings_label(area, r, 0xa80b74); // "Flashing Text"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x10000, 0xa80b50, false); // "Off<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x20000, 0xa80b2c, false); // "On<..>"
    let r = row + 0xc;
    settings_label(area, r, 0xa80b18); // "Background Matches"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x40000, 0xa80aec, false); // "Normal<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x80000, 0xa80ac4, false); // "Off<..>"
    settings_toggle_text(area, 3, r, cfg(CONFIG_OPTIONS), 0x100000, 0xa80a9c, false); // "High<..>"
    let r = row + 0xd;
    settings_label(area, r, 0xa80a90); // "Print To"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x1000000, 0xa80a88, false); // "Printer"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x2000000, 0xa80a80, false); // "File"
    settings_toggle_text(area, 3, r, cfg(CONFIG_OPTIONS), 0x4000000, 0xa80a78, false);
}

/// One button of the Hall of Fame's View menu: table `table`'s name (`hall_of_fame_get_table_name`, long form) after a
/// space in the shared text; the table shown (`shown`) is a plain item (type 0x2002, no id), the others id 0x2e with the
/// table as value; green, yellow and black, in the menu area `area`.
pub(crate) unsafe fn hall_of_fame_table_button(table: u8, shown: u8, area: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let name: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_GET_TABLE_NAME);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let mut buffer = [0u8; 0xc8];
    name(table as u32, buffer.as_mut_ptr() as usize, 0xc8, 1);
    format(SETUP_TEXT, 0x98ed14, buffer.as_ptr()); // " %s"
    let (green, yellow, black) = (setup_colour(G_GUI_COLOUR_GREEN), setup_colour(G_GUI_COLOUR_YELLOW), setup_colour(G_GUI_COLOUR_BLACK));
    if table == shown {
        item(current_human_display(), 0x2002, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, 0, 0, area);
    } else {
        item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, 0x2e, table as i8 as isize as usize, area);
    }
}

/// A submenu of the Hall of Fame's View menu: the item `label` (translated) in the menu area `area` and the area of its
/// own (0x30, green) that it opens; returns that area.
pub(crate) unsafe fn hall_of_fame_submenu(label: usize, area: usize) -> usize {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    text(SETUP_TEXT, label);
    let (green, yellow, black) = (setup_colour(G_GUI_COLOUR_GREEN), setup_colour(G_GUI_COLOUR_YELLOW), setup_colour(G_GUI_COLOUR_BLACK));
    let id = item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, 0, 0, area);
    add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, green, id)
}

/// Shows the Setup "Error" box at `row` after a Hall of Fame file failed, freeing the dialog's path first.
pub(crate) unsafe fn hall_of_fame_file_error(path: usize, row: u32) {
    exe::crt_operator_delete(path);
    setup_warning(row);
}

/// Opens a Setup message box (`build`/`input`) with the interactive flag and `reason` in param 0.
pub(crate) unsafe fn setup_open_reason_box(build: usize, input: usize, reason: usize) {
    if exe::screenmanager_open_screen(SCREEN_MANAGER, build, input, read_u32(G_INTERACTIVE_MODE_FLAG), 0usize, 0) != 0 {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, reason, 0);
    }
}

/// The common part of the disk full and no memory boxes: a random background, the menu bar (mode 4), the Setup title in
/// red with pale grey text and the subtitle `subtitle` (translated).
pub(crate) unsafe fn setup_reason_box_header(subtitle: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    exe::background_set_random_picture();
    exe::menubar_build(4, 0);
    setup_format_title();
    exe::screenmanager_add_screen_title(SCREEN_MANAGER, setup_colour(G_GUI_COLOUR_RED) as u32, setup_colour(G_GUI_COLOUR_PALE_GREY) as u32, G_SETUP_SCREEN_TITLE);
    text(SETUP_TEXT, subtitle);
    exe::screenmanager_add_screen_subtitle(SCREEN_MANAGER, SETUP_TEXT, 0);
}

/// The Ok button of the disk full and no memory boxes (0x64, 0x22b to 0x316, 0x24e; grey, font 4, pale grey), id `id`.
pub(crate) unsafe fn setup_reason_box_ok(id: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    text(SETUP_TEXT, 0x990f7c); // "Ok"
    let (grey, pale) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY));
    item(current_human_display(), 2, 0x64, 0x22b, 0x316, 0x24e, 0, 0, 0x30, grey, grey, 0xc, 4, pale, SETUP_TEXT, 0, id, 0, usize::MAX);
}

/// The answer of the disk full (`ok` 0x33) and no memory (0x34) boxes: Ok closes the box (−9 when the screen manager's
/// current screen has a flag, −5 otherwise); back (−2) gives 0; anything else goes to the menu bar.
pub(crate) unsafe fn setup_reason_box_answer(item: u32, ok: u16) -> i32 {
    let flag: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_GET_CURRENT_SCREEN_FLAG); // ScreenManager::get_current_screen_flag
    match setup_clicked_id(item as i16) {
        id if id == ok => {
            if flag(SCREEN_MANAGER) != 0 { -9 } else { -5 }
        }
        0xfffe => 0,
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// The CD search at the start of `create_game`: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA` = 5) is looked
/// at for "<root>cm0102.exe" (`file_find_first`) until one has it, with the "Insert CD" box in between. A no-CD exe
/// patched in this routine (as `cm0102-1080`: the drive type's `jne` at 0x8281ae made `nop`, the found test at 0x8281f0
/// made `xor`) is honoured by reading those bytes.
pub(crate) unsafe fn create_game_find_cd() {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = (0x8281ae as *const u8).read() == 0x90;
    let always_found = (0x8281f0 as *const u8).read() != 0x85;
    loop {
        // simplification: the buffer starts zeroed (as check_game_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// The runtime record of each person for a new game (`G_STAFF_STATE_TABLE`, `staff_state::SIZE` × `G_STAFF_COUNT`).
///
/// For person `s`: the two ids −1, byte +8 = 0, `staff_state::STATUS_FLAGS` keeps only bit 7. A club join date after the
/// game's start date (`G_GAME_START_DATE`; unsigned year, signed day) becomes that date (`Date::operator=`). With a
/// player record (`staff::PLAYER`) and a club job, and fewer days at the club (`Date::days_since` from the game date)
/// than 415 − 10 × adaptability (`staff::ADAPTABILITY`), the settling counter `staff_state::INJURY_COUNTER` [inference:
/// raised by injuries; here a settling-in value] = (26, or 21 from 215 days) − a − random(a), a = adaptability × 0.5
/// (truncated); a person without a nation gets England's; below 275 days a foreigner
/// (`database_is_staff_foreign`(s, nation, 1)) adds random(5) + 5; it is capped by the current ability
/// (`player_sheet::CURRENT_ABILITY`) × 0.15 (unsigned byte against the signed cap); otherwise 0. A nation join date
/// (`staff::DATE_JOINED_NATION`) after the start date becomes it. `staff_state::B_INTERNATIONAL_PLAYED` = 0, bit 1 of
/// `staff_state::EXTRA_FLAGS` cleared, the five former clubs (−1, weight 0) and the opinion links (−1) emptied.
pub(crate) unsafe fn create_game_staff_states(states: usize) {
    let start = G_GAME_START_DATE;
    for i in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
        let st = states + i * staff_state::SIZE;
        let s = Staff(read_usize(G_STAFF_TABLE) + i * staff::SIZE);
        write_u32(st, 0xffffffff);
        write_u32(st + 4, 0xffffffff);
        write_u8(st + 8, 0);
        write_u8(st + staff_state::STATUS_FLAGS, read_u8(st + staff_state::STATUS_FLAGS) & 0x80);
        let after_start = |date: usize| {
            let (year, day) = (read_u16(start + 2), read_i16(start));
            year < read_u16(date + 2) || (year == read_u16(date + 2) && day < read_i16(date))
        };
        let joined = s.addr() + staff::DATE_JOINED_CLUB;
        if after_start(joined) {
            let mut tmp = [0u8; 8];
            exe::date_operator_3d(joined, tmp.as_mut_ptr() as usize, start);
        }
        let days = exe::date_days_since(G_CURRENT_DATE, read_u32(joined), read_u32(joined + 4));
        let adaptability = s.adaptability() as i32;
        let player = s.player().addr();
        if player == 0 || s.club_job().addr() == 0 || days >= 0x19f - 10 * adaptability {
            write_u8(st + staff_state::INJURY_COUNTER, 0);
        } else {
            let a = ftol(adaptability as f64 * read_f64(rdata::F64_0_5)) as u8;
            let r = exe::utils_random(a as i8 as i32) as u8;
            let base: u8 = if days < 0xd7 { 0x1a } else { 0x15 };
            write_u8(st + staff_state::INJURY_COUNTER, base.wrapping_sub(a).wrapping_sub(r));
            if s.nation().addr() == 0 {
                write_usize(s.addr() + staff::NATION, read_usize(G_NATION_TABLE) + read_u32(KEY_NATION_ENGLAND) as usize * nation::SIZE);
            }
            if days < 0x113 && exe::database_is_staff_foreign(s, s.nation().addr(), 1) != 0 {
                let extra = (exe::utils_random(5) as u8).wrapping_add(5);
                write_u8(st + staff_state::INJURY_COUNTER, read_u8(st + staff_state::INJURY_COUNTER).wrapping_add(extra));
            }
            let ability = read_i16(s.player().addr() + player_sheet::CURRENT_ABILITY) as i32;
            let cap = ftol(ability as f64 * read_f64(rdata::F64_0_15)) as i8;
            if read_u8(st + staff_state::INJURY_COUNTER) as i32 > cap as i32 {
                write_u8(st + staff_state::INJURY_COUNTER, cap as u8);
            }
        }
        let nation_date = s.addr() + staff::DATE_JOINED_NATION;
        if after_start(nation_date) {
            let mut tmp = [0u8; 8];
            exe::date_operator_3d(nation_date, tmp.as_mut_ptr() as usize, start);
        }
        write_u8(st + staff_state::B_INTERNATIONAL_PLAYED, 0);
        write_u8(st + staff_state::EXTRA_FLAGS, read_u8(st + staff_state::EXTRA_FLAGS) & 0xfd);
        for j in 0..5 {
            write_u32(st + staff_state::FORMER_CLUB_ID + 5 * j, 0xffffffff);
            write_u8(st + staff_state::FORMER_CLUB_WEIGHT + 5 * j, 0);
        }
        for j in 0..2 {
            for c in [staff_state::OPINION_LINK_STAFF_ID, 0x2b, 0x2f, 0x33, staff_state::OPINION_LINK_FLAG] {
                write_u32(st + c + 0x14 * j, 0xffffffff);
            }
        }
    }
}

/// The club pass of `create_game` (step 0x23), for each club: `club::OWN_STADIUM` = 0 for Brighton and Wimbledon (the
/// first 6 letters of `club::NAME`), 1 for the others; a reputation (`club::REPUTATION`) ≤ 0 becomes random(4000) + 1 as
/// a signed byte; attendance ≤ 0 becomes reputation² / 1000 for a national side (`database_is_national_team`), / 2000
/// otherwise, at least 250; minimum attendance ≤ 0: reputation² / 3350 (at least 750) or / 10000 (at least 75); maximum
/// attendance ≤ 0: reputation² / 500 (at least 3250) or / 1000 (at least 12500), plus random(7500); a training value
/// (`club::TRAINING`) outside 1..20 becomes reputation / 500 within 1..20.
pub(crate) unsafe fn create_game_clubs() {
    for i in 0..read_i32(G_CLUB_COUNT).max(0) {
        let club = club_at(i as isize);
        let name = club.0 + club::NAME;
        let shares = exe::crt_strncmp(name, 0xa813a0, 6) == 0 || exe::crt_strncmp(name, 0xa81394, 6) == 0; // "Brighton", "Wimbledon"
        club.set_own_stadium(if shares { 0 } else { 1 });
        if club.reputation() <= 0 {
            let r = (exe::utils_random(0xfa0) as u8).wrapping_add(1);
            club.set_reputation(r as i8 as i16);
        }
        if club.attendance() <= 0 {
            let national = exe::database_is_national_team(club) != 0;
            let rep = club.reputation() as i32;
            let square = rep.wrapping_mul(rep);
            let a = if national { square / 1000 } else { square / 2000 };
            club.set_attendance(a.max(0xfa));
        }
        if club.min_attendance() <= 0 {
            let national = exe::database_is_national_team(club) != 0;
            let rep = club.reputation() as i32;
            let square = rep.wrapping_mul(rep);
            let a = if national { (square / 0xd16).max(0x2ee) } else { (square / 10000).max(0x4b) };
            club.set_min_attendance(a);
        }
        if club.max_attendance() <= 0 {
            let national = exe::database_is_national_team(club) != 0;
            let rep = club.reputation() as i32;
            let square = rep.wrapping_mul(rep);
            let a = if national { (square / 500).max(0xcb2) } else { (square / 1000).max(0x30d4) };
            club.set_max_attendance(exe::utils_random(0x1d4c).wrapping_add(a));
        }
        let training = club.training();
        if training < 1 || training > 0x14 {
            let t = (club.reputation() as i32 / 500) as i8;
            club.set_training(t.clamp(1, 0x14));
        }
    }
}

/// The human manager row `k` (0..16) counted back from the end of the staff table: the 16 human managers are the
/// last 16 staff records.
pub(crate) unsafe fn human_manager_row(k: i32) -> Staff {
    Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(G_STAFF_COUNT) - 1 - k) as usize * staff::SIZE))
}

/// The current screen slot's manager (`G_SCREEN_RECORDS_MANAGER`, 0x300 bytes per slot, slot
/// `G_SCREEN_MANAGER_CURRENT_SLOT`).
pub(crate) unsafe fn current_screen_manager() -> Staff {
    Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize) as usize))
}

/// The competition record of a key: `G_CLUB_COMP_TABLE` + [key] × 0x6b.
pub(crate) unsafe fn key_comp_record(key: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32(key) as usize).wrapping_mul(0x6b))
}

/// The value of an `EntryYear` as the dword a constructor gets (the original's high word is stack leftover, here 0,
/// except for the 4 weeks date, read as a dword from its year on); the 16-bit fields take its low word.
pub(crate) unsafe fn entry_year(year: EntryYear, weeks: &[u8; 8]) -> u32 {
    match year {
        EntryYear::Current(d) => read_u16(G_CURRENT_DATE_YEAR).wrapping_add(d) as u32,
        EntryYear::Start(d) => read_u16(G_GAME_START_DATE_YEAR).wrapping_add(d) as u32,
        EntryYear::StartPlus4Weeks => read_u32(weeks.as_ptr() as usize + cm_date::YEAR),
        EntryYear::Fixed(y) => y as u32,
    }
}

/// The value an `EntryLink` writes, or None when it writes nothing.
pub(crate) unsafe fn entry_link(link: EntryLink) -> Option<usize> {
    match link {
        EntryLink::Untouched => None,
        EntryLink::Null => Some(0),
        EntryLink::Comp(key) => Some(key_comp_record(key)),
    }
}

/// The competition count an entry gets, or None (a continental entry without a selected nation).
pub(crate) unsafe fn key_nation_comp_count(entry: usize, spec: &KeyNationComps) -> Option<u32> {
    match spec.count {
        EntryCount::Fixed(n) => Some(n),
        EntryCount::Continental(n) => {
            (exe::key_nation_continent_has_selected_nation(read_usize(entry + KEY_NATION_RECORD_CONTINENT)) != 0).then_some(n)
        }
        EntryCount::ExtraDivision(small, large, _) => Some(if key_nation_extra_division(entry) { large } else { small }),
    }
}

/// Whether a run of extra-division competitions is created (tested once before the run, as the original).
pub(crate) unsafe fn key_nation_extra_guard(entry: usize, spec: &KeyNationComps) -> bool {
    match spec.count {
        EntryCount::ExtraDivision(_, large, ExtraGuard::CountIsLarger) => read_u32(entry + KEY_NATION_RECORD_COMP_COUNT) == large,
        EntryCount::ExtraDivision(_, _, ExtraGuard::Flag) => key_nation_extra_division(entry),
        _ => true,
    }
}

/// The 4 weeks date of an entry that uses it (`Date::add_weeks`(G_GAME_START_DATE, 4)), zeroes otherwise.
pub(crate) unsafe fn key_nation_weeks(spec: &KeyNationComps) -> [u8; 8] {
    let mut weeks = [0u8; 8];
    if spec.weeks {
        exe::date_add_weeks(G_GAME_START_DATE, &mut weeks, 4);
    }
    weeks
}

/// The day of Date::Date(day, month, 2001, −1) (the season end of every entry is a 2001 date).
pub(crate) unsafe fn key_nation_end_day(spec: &KeyNationComps) -> u16 {
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, spec.end_day.0, spec.end_day.1, 0x7d1, 0xffff_ffff);
    d[0]
}

/// Opens the box screen `build`/`input` with `flag` and, when it opens, its param 0 = 0.
pub(crate) unsafe fn setup_open_screen_with_flag(build: usize, input: usize, flag: u32) {
    if exe::screenmanager_open_screen(SCREEN_MANAGER, build, input, flag, 0usize, 0) != 0 {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, 0usize, 0);
    }
}

/// A row label of the Game Settings box: `label` (translated, with an empty argument) in column 0, silver, font 3.
pub(crate) unsafe fn settings_label(area: usize, row: u32, label: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(SETUP_TEXT, label, 0xad9c64usize);
    exe::screenmanager_add_area_text(SCREEN_MANAGER, area, 0, row, 1, 0, 1, 3, setup_colour(G_GUI_COLOUR_SILVER), SETUP_TEXT);
}

/// One choice of an on/off row (already in the shared text) in column `column`: when `bit` is not set in the options,
/// a button (id 0x2c with `bit` as value, the option colour) or, with `disabled`, the same as plain text; when set,
/// yellow text with style 0x221.
pub(crate) unsafe fn settings_toggle(area: usize, column: u32, row: u32, options: u32, bit: u32, disabled: bool) {
    let button: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let (option, yellow) = (setup_colour(G_SETTINGS_OPTION_COLOUR), setup_colour(G_GUI_COLOUR_YELLOW));
    if options & bit != 0 {
        exe::screenmanager_add_area_text(SCREEN_MANAGER, area, column, row, 0x221, yellow, 0xc, 2, yellow, SETUP_TEXT);
    } else if disabled {
        exe::screenmanager_add_area_text(SCREEN_MANAGER, area, column, row, 1, 0, 0x2c, 2, option, SETUP_TEXT);
    } else {
        button(SCREEN_MANAGER, area, column, row, 1, 0, 0xc, 2, option, SETUP_TEXT, 0x2c, bit);
    }
}

/// `settings_toggle` with the choice's text `label` translated first (with an empty argument).
pub(crate) unsafe fn settings_toggle_text(area: usize, column: u32, row: u32, options: u32, bit: u32, label: usize, disabled: bool) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(SETUP_TEXT, label, 0xad9c64usize);
    settings_toggle(area, column, row, options, bit, disabled);
}

/// The drop-down of a list row in column 2: read-only, an empty black cell; otherwise an empty button and the menu area
/// it opens (style 0x100030, green), which is returned (0 read-only: the original keeps a leftover there, never used).
pub(crate) unsafe fn settings_dropdown(area: usize, row: u32, read_only: bool) -> usize {
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let (black, pale) = (setup_colour(G_GUI_COLOUR_BLACK), setup_colour(G_GUI_COLOUR_PALE_GREY));
    if read_only {
        exe::screenmanager_add_area_text(SCREEN_MANAGER, area, 2, row, 0x30, black, 0xc, 1, pale, 0);
        return 0;
    }
    let b = exe::screenmanager_add_area_button(SCREEN_MANAGER, area, 2, row, 0x30, black, 0xc, 1, pale, 0, 0, 0);
    add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x100030, setup_colour(G_GUI_COLOUR_GREEN), b)
}

/// One item of a drop-down menu `menu`: `text`, style `style` (1 normal, 0x21 greyed), id `id` with value `value`.
pub(crate) unsafe fn settings_choice(menu: usize, text: usize, style: usize, id: usize, value: usize) {
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let (green, yellow, black) = (setup_colour(G_GUI_COLOUR_GREEN), setup_colour(G_GUI_COLOUR_YELLOW), setup_colour(G_GUI_COLOUR_BLACK));
    item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, style, 1, black, text, 0, id, value, menu);
}

/// The current choice of a list row in column 1: `name` after two spaces, yellow.
pub(crate) unsafe fn settings_current(area: usize, row: u32, name: &[u8; 0xc8]) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(SETUP_TEXT, 0x987ff0, name.as_ptr()); // "  %s"
    exe::screenmanager_add_area_text(SCREEN_MANAGER, area, 1, row, 1, 0, 1, 2, setup_colour(G_GUI_COLOUR_YELLOW), SETUP_TEXT);
}

/// Whether the entry's nation has its extra division (bit 2 of `nation::LEAGUE_SELECTED`).
pub(crate) unsafe fn key_nation_extra_division(entry: usize) -> bool {
    Nation(read_usize(entry + KEY_NATION_RECORD_NATION)).league_selected() & 4 != 0
}

/// The size of a human manager record (`human_manager_record`, 42 735 bytes) and the match day object
/// (`set_computer_controlled` tells it when it is set).
pub(crate) const HUMAN_MANAGER_RECORD_SIZE: usize = 0xa6ef;

pub(crate) const G_MATCH_DAY_OBJECT: usize = 0xb67a50;

/// The entry of `create_world_cup_qualifying_comps`.
pub(crate) const WORLD_CUP_QUALIFYING_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2ede, 0x2efb, 0xa81d04), // "create_world_cup_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Fixed(7),
    calloc: None,
    comps: &[
        (0x192, 0x92bf50, KEY_CLUB_COMP_FIFA_WORLD_CUP, EntryYear::Current(0), false), // WorldCup::WorldCup
        (0xbb, 0x90f210, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcAfricanCup::WcAfricanCup
        (0xfb, 0x911bf0, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcAsiaLeague::WcAsiaLeague
        (0xbd, 0x9149f0, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcConcacafCup::WcConcacafCup
        (0x102, 0x918200, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcEuropeLeague::WcEuropeLeague
        (0xf7, 0x91df90, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcOceaniaLeague::WcOceaniaLeague
        (0xf0, 0x9202b0, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, EntryYear::Current(0), false), // WcSouthAmericanLeague::WcSouthAmericanLeague
    ],
    end_day: (25, 6),
    season_year: EntryYear::Fixed(0x7cc),
    spans: 2,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_other_int_comps`.
pub(crate) const OTHER_INT_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2f22, 0x2f3e, 0xa81d3c), // "create_other_int_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Fixed(8),
    calloc: None,
    comps: &[
        (0xf6, 0x79f530, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, EntryYear::Current(0), false), // Olympics::Olympics
        (0x142, 0x57a890, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, EntryYear::Current(0), false), // EuroChamp::EuroChamp
        (0x102, 0x57ce10, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING, EntryYear::Current(0), false), // EuroChampQual::EuroChampQual
        (0xf4, 0x401000, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, EntryYear::Current(0), false), // AfricanNations::AfricanNations
        (0xf6, 0x5e0660, KEY_CLUB_COMP_COPA_AMERICA, EntryYear::Current(0), false), // GoldCup::GoldCup
        (0xf4, 0x7988b0, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, EntryYear::Current(0), false), // OceaniaNations::OceaniaNations
        (0xf4, 0x40e860, KEY_CLUB_COMP_ASIAN_CUP, EntryYear::Current(0), false), // AsiaNations::AsiaNations
        (0xf6, 0x58b310, KEY_CLUB_COMP_CONFEDERATIONS_CUP, EntryYear::Current(0), false), // FifaConfed::FifaConfed
    ],
    end_day: (15, 7),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 2,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_world_club_comps`.
pub(crate) const WORLD_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2f69, 0x2f88, 0xa81d74), // "create_world_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xf6, 0x929140, KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP, EntryYear::Current(0), false), // WorldClubChamp::WorldClubChamp
        (0xb2, 0x92b4b0, KEY_CLUB_COMP_WORLD_CLUB_CUP, EntryYear::Current(0), false), // WorldClubCup::WorldClubCup
        (0xb2, 0x632080, KEY_CLUB_COMP_INTER_AMERICAN_CUP, EntryYear::Current(0), false), // InterAmerCup::InterAmerCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_european_club_comps`.
pub(crate) const EUROPEAN_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2fac, 0x2fcd, 0xa81dac), // "create_european_club_comps() : unable to allocate memory"
    start: (23, 5, EntryYear::Current(0), 0xffff),
    end: (20, 4, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(4),
    calloc: None,
    comps: &[
        (0xb2, 0x5837c0, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, EntryYear::Current(0), false), // EuropeanCup::EuropeanCup
        (0xb3, 0x9022e0, KEY_CLUB_COMP_UEFA_CUP, EntryYear::Current(0), false), // UefaCup::UefaCup
        (0xb6, 0x632fb0, KEY_CLUB_COMP_INTER_TOTO_CUP, EntryYear::Current(0), false), // IntertotoCup::IntertotoCup
        (0xb2, 0x57a100, KEY_CLUB_COMP_EUROPEAN_SUPER_CUP, EntryYear::Current(0), false), // EurSuperCup::EurSuperCup
    ],
    end_day: (21, 5),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_north_american_club_comps`.
pub(crate) const NORTH_AMERICAN_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2ff9, 0x301d, 0xa81de8), // "create_north_american_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(2),
    calloc: None,
    comps: &[
        (0xb2, 0x4c0c40, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, EntryYear::Start(0), false), // ConChamp::ConChamp
        (0xf6, 0x4c16f0, KEY_CLUB_COMP_CONCACAF_MERCONORTE_CUP, EntryYear::Start(0), false), // ConMercCup::ConMercCup
    ],
    end_day: (21, 1),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_south_american_club_comps`.
pub(crate) const SOUTH_AMERICAN_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3045, 0x3065, 0xa81e28), // "create_south_american_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(2),
    calloc: Some((4, 2)),
    comps: &[
        (0xf6, 0x4c2f50, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, EntryYear::Current(0), false), // ConmebolLiber::ConmebolLiber
        (0xf6, 0x4c46c0, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_MERCOSUR, EntryYear::Current(0), false), // ConmebolMerc::ConmebolMerc
    ],
    end_day: (10, 0),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_asian_club_comps`.
pub(crate) const ASIAN_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x308d, 0x30ad, 0xa81e68), // "create_asian_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(3),
    calloc: None,
    comps: &[
        (0xba, 0x40aae0, KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP, EntryYear::Current(0), false), // AsiaClubChamp::AsiaClubChamp
        (0xba, 0x40cbe0, KEY_CLUB_COMP_ASIAN_CUP_WINNERS_CUP, EntryYear::Current(0), false), // AsiaCupWinner::AsiaCupWinner
        (0xb2, 0x410650, KEY_CLUB_COMP_ASIAN_SUPER_CUP, EntryYear::Current(0), false), // AsiaSuperCup::AsiaSuperCup
    ],
    end_day: (28, 7),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_oceania_club_comps`.
pub(crate) const OCEANIA_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x30d6, 0x30f6, 0xa81e68), // "create_asian_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(1),
    calloc: None,
    comps: &[
        (0xf6, 0x796fd0, KEY_CLUB_COMP_OCEANIA_CLUB_CHAMPIONSHIP, EntryYear::Current(0), false), // OceaniaClubChamp::OceaniaClubChamp
    ],
    end_day: (21, 6),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_argentine_comps`.
pub(crate) const ARGENTINE_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x311f, 0x313c, 0xa81ea0), // "create_argentine_comps() : unable to allocate memory"
    start: (23, 7, EntryYear::Current(0), 0x5),
    end: (19, 6, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(2),
    calloc: None,
    comps: &[
        (0xee, 0x404250, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION, EntryYear::Current(0), false), // ArgPrm::ArgPrm
        (0xf2, 0x406bf0, KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION, EntryYear::Current(0), false), // ArgSecond::ArgSecond
    ],
    end_day: (1, 7),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_australian_comps`.
pub(crate) const AUSTRALIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3160, 0x317d, 0xa81ed8), // "create_australian_comps() : unable to allocate memory"
    start: (2, 9, EntryYear::Current(0), 0x5),
    end: (10, 5, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(1),
    calloc: None,
    comps: &[
        (0xee, 0x410e90, KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE, EntryYear::Current(0), false), // AusNsl::AusNsl
    ],
    end_day: (20, 6),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_belgian_comps`.
pub(crate) const BELGIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x31a1, 0x31be, 0xa81f10), // "create_belgian_comps() : unable to allocate memory"
    start: (3, 7, EntryYear::Current(0), 0x5),
    end: (1, 5, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(5),
    calloc: None,
    comps: &[
        (0xee, 0x41dd40, KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, EntryYear::Current(0), false), // BelFirst::BelFirst
        (0xf9, 0x41f030, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION, EntryYear::Current(0), false), // BelSecond::BelSecond
        (0x104, 0x4216f0, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION, EntryYear::Current(0), false), // BelThird::BelThird
        (0xb2, 0x41d050, KEY_CLUB_COMP_BELGIAN_CUP, EntryYear::Current(0), false), // BelFaCup::BelFaCup
        (0xb2, 0x420ef0, KEY_CLUB_COMP_BELGIAN_SUPER_CUP, EntryYear::Current(0), false), // BelSuper::BelSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_BELGIAN_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_brazilian_comps`.
pub(crate) const BRAZILIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x31e6, 0x3205, 0xa81f44), // "create_norwegian_comps() : unable to allocate memory"
    start: (18, 0, EntryYear::StartPlus4Weeks, 0x6),
    end: (14, 11, EntryYear::StartPlus4Weeks, 0x6),
    count: EntryCount::Fixed(17),
    calloc: None,
    comps: &[
        (0xee, 0x429850, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, EntryYear::StartPlus4Weeks, false), // BraNatFirst::BraNatFirst
        (0xee, 0x42be40, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, EntryYear::StartPlus4Weeks, false), // BraNatSecond::BraNatSecond
        (0xf2, 0x42d2c0, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION, EntryYear::StartPlus4Weeks, false), // BraNatThird::BraNatThird
        (0xee, 0x42f2d0, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegBahia::BraRegBahia
        (0xee, 0x4304b0, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegCentral::BraRegCentral
        (0xee, 0x4317c0, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegGaucho::BraRegGaucho
        (0xee, 0x432b50, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegGoias::BraRegGoias
        (0xee, 0x433d50, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, EntryYear::StartPlus4Weeks, false), // BraRegMinasGerais::BraRegMinasGerais
        (0xee, 0x435340, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegNorth::BraRegNorth
        (0xee, 0x4363f0, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegNortheast::BraRegNortheast
        (0xee, 0x437400, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegParana::BraRegParana
        (0xee, 0x4388f0, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegPern::BraRegPern
        (0xee, 0x439a40, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, EntryYear::StartPlus4Weeks, false), // BraRegRio::BraRegRio
        (0xee, 0x43b130, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, EntryYear::StartPlus4Weeks, false), // BraRegSanta::BraRegSanta
        (0xee, 0x43c280, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, EntryYear::StartPlus4Weeks, false), // BraRegSp::BraRegSp
        (0xb2, 0x427430, KEY_CLUB_COMP_BRAZILIAN_CUP, EntryYear::StartPlus4Weeks, false), // BraCup::BraCup
        (0xee, 0x4259a0, KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP, EntryYear::StartPlus4Weeks, false), // BraChampCup::BraChampCup
    ],
    end_day: (12, 0),
    season_year: EntryYear::StartPlus4Weeks,
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: true,
};

/// The entry of `create_croatian_comps`.
pub(crate) const CROATIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3239, 0x3256, 0xa81f7c), // "create_croatian_comps() : unable to allocate memory"
    start: (24, 6, EntryYear::Current(0), 0x5),
    end: (14, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xef, 0x513e10, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, EntryYear::Current(0), false), // CroA1::CroA1
        (0xef, 0x515310, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, EntryYear::Current(0), false), // CroA2a::CroA2a
        (0xef, 0x516430, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, EntryYear::Current(0), false), // CroA2b::CroA2b
        (0xb2, 0x517480, KEY_CLUB_COMP_CROATIAN_CUP, EntryYear::Current(0), false), // CroCup::CroCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_danish_comps`.
pub(crate) const DANISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x327e, 0x329a, 0xa81fb0), // "create_danish_comps() : unable to allocate memory"
    start: (27, 6, EntryYear::Current(0), 0x6),
    end: (14, 5, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x5525e0, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, EntryYear::Current(0), false), // DenPrm::DenPrm
        (0xee, 0x5516f0, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, EntryYear::Current(0), false), // DenFirst::DenFirst
        (0xee, 0x553e80, KEY_CLUB_COMP_DANISH_SECOND_DIVISION, EntryYear::Current(0), false), // DenSecond::DenSecond
        (0xb2, 0x5509b0, KEY_CLUB_COMP_DANISH_CUP, EntryYear::Current(0), false), // DenCup::DenCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_english_comps`.
pub(crate) const ENGLISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x32c1, 0x32e0, 0xa81fe4), // "create_english_comps() : unable to allocate memory"
    start: (10, 7, EntryYear::Current(0), 0x5),
    end: (24, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::ExtraDivision(8, 10, ExtraGuard::CountIsLarger),
    calloc: None,
    comps: &[
        (0xee, 0x573380, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, EntryYear::Current(0), false), // EngPrm::EngPrm
        (0xee, 0x571680, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, EntryYear::Current(0), false), // EngFirst::EngFirst
        (0xee, 0x5754a0, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, EntryYear::Current(0), false), // EngSecond::EngSecond
        (0xee, 0x576dd0, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, EntryYear::Current(0), false), // EngThird::EngThird
        (0xee, 0x56db60, KEY_CLUB_COMP_ENGLISH_CONFERENCE, EntryYear::Current(0), true), // EngConf::EngConf
        (0xb5, 0x56f040, KEY_CLUB_COMP_ENGLISH_FA_CUP, EntryYear::Current(0), false), // EngFaCup::EngFaCup
        (0xb3, 0x56c2a0, KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP, EntryYear::Current(0), false), // EngCcCup::EngCcCup
        (0xba, 0x56aa30, KEY_CLUB_COMP_ENGLISH_AUTOGLASS_TROPHY, EntryYear::Current(0), false), // EngAutoCup::EngAutoCup
        (0xb2, 0x56d380, KEY_CLUB_COMP_ENGLISH_CHARITY_SHIELD, EntryYear::Current(0), false), // EngCharity::EngCharity
        (0xb2, 0x570c00, KEY_CLUB_COMP_ENGLISH_FA_TROPHY, EntryYear::Current(0), true), // EngFaTrophy::EngFaTrophy
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_ENGLISH_CHARITY_SHIELD),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_finnish_comps`.
pub(crate) const FINNISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3311, 0x332d, 0xa82018), // "create_finnish_comps() : unable to allocate memory"
    start: (29, 3, EntryYear::Start(0), 0x5),
    end: (12, 10, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xee, 0x595170, KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, EntryYear::Start(0), false), // FinPrm::FinPrm
        (0xfe, 0x592eb0, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION, EntryYear::Start(0), false), // FinFirst::FinFirst
        (0xb2, 0x592250, KEY_CLUB_COMP_FINNISH_CUP, EntryYear::Start(0), false), // FinCup::FinCup
    ],
    end_day: (27, 11),
    season_year: EntryYear::Start(0),
    spans: 0,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_french_comps`.
pub(crate) const FRENCH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3352, 0x336e, 0xa8204c), // "create_french_comps() : unable to allocate memory"
    start: (9, 7, EntryYear::Current(0), 0x5),
    end: (24, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(6),
    calloc: None,
    comps: &[
        (0xf6, 0x5bb160, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, EntryYear::Current(0), false), // FraFirst::FraFirst
        (0xee, 0x5be330, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION, EntryYear::Current(0), false), // FraSecond::FraSecond
        (0xee, 0x5bfb40, KEY_CLUB_COMP_FRENCH_NATIONAL, EntryYear::Current(0), false), // FraThird::FraThird
        (0xb2, 0x5ba130, KEY_CLUB_COMP_FRENCH_CUP, EntryYear::Current(0), false), // FraCup::FraCup
        (0xb2, 0x5bc980, KEY_CLUB_COMP_FRENCH_LEAGUE_CUP, EntryYear::Current(0), false), // FraLgeCup::FraLgeCup
        (0xb2, 0x5bf320, KEY_CLUB_COMP_FRENCH_CHAMPIONS_TROPHY, EntryYear::Current(0), false), // FraSuper::FraSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_FRENCH_CHAMPIONS_TROPHY),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_german_comps`.
pub(crate) const GERMAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3397, 0x33b6, 0xa82080), // "create_german_comps() : unable to allocate memory"
    start: (10, 7, EntryYear::Current(0), 0x5),
    end: (14, 5, EntryYear::Current(1), 0x5),
    count: EntryCount::ExtraDivision(4, 5, ExtraGuard::CountIsLarger),
    calloc: None,
    comps: &[
        (0xee, 0x5d99b0, KEY_CLUB_COMP_GERMAN_FIRST_DIVISION, EntryYear::Current(0), false), // GerFirst::GerFirst
        (0xee, 0x5de2e0, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION, EntryYear::Current(0), false), // GerSecond::GerSecond
        (0xee, 0x5dcbd0, KEY_CLUB_COMP_GERMAN_REGIONAL, EntryYear::Current(0), true), // GerRegional::GerRegional
        (0xb2, 0x5d8c20, KEY_CLUB_COMP_GERMAN_CUP, EntryYear::Current(0), false), // GerCup::GerCup
        (0xb2, 0x5dc0e0, KEY_CLUB_COMP_GERMAN_LEAGUE_CUP, EntryYear::Current(0), false), // GerLgeCup::GerLgeCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_greek_comps`.
pub(crate) const GREEK_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x33e1, 0x33fd, 0xa82080), // "create_german_comps() : unable to allocate memory"
    start: (15, 7, EntryYear::Current(0), 0x5),
    end: (21, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x5ea410, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, EntryYear::Current(0), false), // GrePrm::GrePrm
        (0xee, 0x5eb2b0, KEY_CLUB_COMP_GREEK_SECOND_DIVISION, EntryYear::Current(0), false), // GreSecond::GreSecond
        (0xf6, 0x5e88a0, KEY_CLUB_COMP_GREEK_CUP, EntryYear::Current(0), false), // GreCup::GreCup
        (0xb2, 0x5ec040, KEY_CLUB_COMP_GREEK_SUPER_CUP, EntryYear::Current(0), false), // GreSuper::GreSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_dutch_comps`.
pub(crate) const DUTCH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3423, 0x343f, 0xa820b4), // "create_dutch_comps() : unable to allocate memory"
    start: (16, 7, EntryYear::Current(0), 0x5),
    end: (7, 5, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x5f7610, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, EntryYear::Current(0), false), // HolPrm::HolPrm
        (0xf2, 0x5f5490, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION, EntryYear::Current(0), false), // HolFirst::HolFirst
        (0xf6, 0x5f3450, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP, EntryYear::Current(0), false), // HolCup::HolCup
        (0xb2, 0x5f8820, KEY_CLUB_COMP_DUTCH_SUPER_CUP, EntryYear::Current(0), false), // HolSuper::HolSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_irish_comps`.
pub(crate) const IRISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3466, 0x3482, 0xa820e8), // "create_irish_comps() : unable to allocate memory"
    start: (1, 7, EntryYear::Current(0), 0x6),
    end: (23, 3, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(8),
    calloc: None,
    comps: &[
        (0x120, 0x639bb0, KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, EntryYear::Current(0), false), // IrePrm::IrePrm
        (0xee, 0x634c40, KEY_CLUB_COMP_IRISH_FIRST_DIVISION, EntryYear::Current(0), false), // IreFirst::IreFirst
        (0xb2, 0x6340a0, KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP, EntryYear::Current(0), false), // IreChalCup::IreChalCup
        (0xf6, 0x636dd0, KEY_CLUB_COMP_IRISH_LEAGUE_CUP, EntryYear::Current(0), false), // IreLgeCup::IreLgeCup
        (0xee, 0x63acd0, KEY_CLUB_COMP_IRISH_SUPER_CUP, EntryYear::Current(0), false), // IreSuperCup::IreSuperCup
        (0xb2, 0x639280, KEY_CLUB_COMP_IRISH_PRESIDENTS_CUP, EntryYear::Current(0), false), // IrePresCup::IrePresCup
        (0xb2, 0x6387a0, KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_CUP, EntryYear::Current(0), false), // IreMunsterCup::IreMunsterCup
        (0xb2, 0x6364a0, KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_CUP, EntryYear::Current(0), false), // IreLeinsterCup::IreLeinsterCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_IRISH_SUPER_CUP),
    cup: EntryLink::Comp(KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP),
    weeks: false,
};

/// The entry of `create_italian_comps`.
pub(crate) const ITALIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x34af, 0x34ce, 0xa8211c), // "create_italian_comps() : unable to allocate memory"
    start: (6, 8, EntryYear::Current(0), 0x6),
    end: (8, 5, EntryYear::Current(1), 0x6),
    count: EntryCount::ExtraDivision(8, 11, ExtraGuard::Flag),
    calloc: None,
    comps: &[
        (0xef, 0x6406d0, KEY_CLUB_COMP_ITALIAN_SERIE_A, EntryYear::Current(0), false), // ItaSerA::ItaSerA
        (0xef, 0x645810, KEY_CLUB_COMP_ITALIAN_SERIE_B, EntryYear::Current(0), false), // ItaSerB::ItaSerB
        (0xee, 0x64aa70, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, EntryYear::Current(0), false), // ItaSerC1a::ItaSerC1a
        (0xee, 0x64ec20, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, EntryYear::Current(0), false), // ItaSerC1b::ItaSerC1b
        (0xee, 0x652e90, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, EntryYear::Current(0), true), // ItaSerC2a::ItaSerC2a
        (0xee, 0x657350, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, EntryYear::Current(0), true), // ItaSerC2b::ItaSerC2b
        (0xee, 0x65b4e0, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C, EntryYear::Current(0), true), // ItaSerC2c::ItaSerC2c
        (0xf6, 0x63e900, KEY_CLUB_COMP_ITALIAN_CUP, EntryYear::Current(0), false), // ItaCup::ItaCup
        (0xf6, 0x63cc40, KEY_CLUB_COMP_ITALIAN_SERIE_C_CUP, EntryYear::Current(0), false), // ItaCCup::ItaCCup
        (0xb2, 0x65f670, KEY_CLUB_COMP_ITALIAN_SUPER_CUP, EntryYear::Current(0), false), // ItaSuper::ItaSuper
        (0xb2, 0x63c5e0, KEY_CLUB_COMP_ITALIAN_C1_SUPER_CUP, EntryYear::Current(0), false), // ItaC1Super::ItaC1Super
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_ITALIAN_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_japanese_comps`.
pub(crate) const JAPANESE_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3502, 0x351e, 0xa82150), // "create_japanese_comps() : unable to allocate memory"
    start: (14, 2, EntryYear::Start(0), 0x5),
    end: (28, 10, EntryYear::Start(0), 0x5),
    count: EntryCount::Fixed(5),
    calloc: None,
    comps: &[
        (0xee, 0x662b30, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, EntryYear::Start(0), false), // JapJ1::JapJ1
        (0xee, 0x6644c0, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, EntryYear::Start(0), false), // JapJ2::JapJ2
        (0xb2, 0x661d30, KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, EntryYear::Start(0), false), // JapEmpCup::JapEmpCup
        (0xb2, 0x665380, KEY_CLUB_COMP_JAPANESE_YAMAZAKI_NABISCO_CUP, EntryYear::Start(0), false), // JapJCup::JapJCup
        (0xb2, 0x665d70, KEY_CLUB_COMP_JAPANESE_XEROX_SUPER_CUP, EntryYear::Start(0), false), // JapSuper::JapSuper
    ],
    end_day: (25, 0),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_northern_irish_comps`.
pub(crate) const NORTHERN_IRISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3546, 0x3562, 0xa82184), // "create_northern_irish_comps() : unable to allocate memory"
    start: (12, 7, EntryYear::Current(0), 0x5),
    end: (5, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(5),
    calloc: None,
    comps: &[
        (0xee, 0x78e070, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, EntryYear::Current(0), false), // NirPrm::NirPrm
        (0xee, 0x78b010, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, EntryYear::Current(0), false), // NirFirst::NirFirst
        (0xb2, 0x78a5b0, KEY_CLUB_COMP_NORTHERN_IRISH_CUP, EntryYear::Current(0), false), // NirCup::NirCup
        (0xf6, 0x78c780, KEY_CLUB_COMP_NORTHERN_IRISH_LEAGUE_CUP, EntryYear::Current(0), false), // NirLgeCup::NirLgeCup
        (0xb2, 0x789df0, KEY_CLUB_COMP_NORTHERN_IRISH_CHARITY_SHIELD, EntryYear::Current(0), false), // NirCharity::NirCharity
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_NORTHERN_IRISH_CHARITY_SHIELD),
    cup: EntryLink::Comp(KEY_CLUB_COMP_NORTHERN_IRISH_CUP),
    weeks: false,
};

/// The entry of `create_norwegian_comps`.
pub(crate) const NORWEGIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x358c, 0x35a8, 0xa81f44), // "create_norwegian_comps() : unable to allocate memory"
    start: (12, 3, EntryYear::Start(0), 0x6),
    end: (31, 9, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xee, 0x791500, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, EntryYear::Start(0), false), // NorPrm::NorPrm
        (0xee, 0x78fd40, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION, EntryYear::Start(0), false), // NorFirst::NorFirst
        (0xb2, 0x78f020, KEY_CLUB_COMP_NORWEGIAN_CUP, EntryYear::Start(0), false), // NorCup::NorCup
    ],
    end_day: (27, 10),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_polish_comps`.
pub(crate) const POLISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x35cf, 0x35eb, 0xa821c0), // "create_polish_comps() : unable to allocate memory"
    start: (17, 6, EntryYear::Current(0), 0x5),
    end: (27, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(5),
    calloc: None,
    comps: &[
        (0xf2, 0x7c8a80, KEY_CLUB_COMP_POLISH_FIRST_DIVISION, EntryYear::Current(0), false), // PolFirst::PolFirst
        (0xee, 0x7cb1d0, KEY_CLUB_COMP_POLISH_SECOND_DIVISION, EntryYear::Current(0), false), // PolSecond::PolSecond
        (0xb2, 0x7c7d80, KEY_CLUB_COMP_POLISH_FA_CUP, EntryYear::Current(0), false), // PolCup::PolCup
        (0xb2, 0x7ca4e0, KEY_CLUB_COMP_POLISH_LEAGUE_CUP, EntryYear::Current(0), false), // PolLgeCup::PolLgeCup
        (0xb2, 0x7cc920, KEY_CLUB_COMP_POLISH_SUPER_CUP, EntryYear::Current(0), false), // PolSuper::PolSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Untouched,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_portuguese_comps`.
pub(crate) const PORTUGUESE_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3612, 0x3631, 0xa821f4), // "create_portuguese_comps() : unable to allocate memory"
    start: (25, 7, EntryYear::Current(0), 0x6),
    end: (8, 5, EntryYear::Current(1), 0x6),
    count: EntryCount::ExtraDivision(4, 5, ExtraGuard::Flag),
    calloc: None,
    comps: &[
        (0xee, 0x7ce720, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, EntryYear::Current(0), false), // PorPrm::PorPrm
        (0xee, 0x7d0950, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION, EntryYear::Current(0), false), // PorSecond::PorSecond
        (0xee, 0x7d1920, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B, EntryYear::Current(0), true), // PorSecondB::PorSecondB
        (0xb2, 0x7cd980, KEY_CLUB_COMP_PORTUGUESE_CUP, EntryYear::Current(0), false), // PorCup::PorCup
        (0xb2, 0x7d2e40, KEY_CLUB_COMP_PORTUGUESE_SUPER_CUP, EntryYear::Current(0), false), // PorSuper::PorSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_PORTUGUESE_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_russian_comps`.
pub(crate) const RUSSIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x365a, 0x3679, 0xa8222c), // "create_russian_comps() : unable to allocate memory"
    start: (25, 2, EntryYear::Start(0), 0x5),
    end: (12, 10, EntryYear::Start(0), 0x6),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xef, 0x7eaa80, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, EntryYear::Start(0), false), // RusPrm::RusPrm
        (0xee, 0x7e9f30, KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION, EntryYear::Start(0), false), // RusFirst::RusFirst
        (0xb2, 0x7e9210, KEY_CLUB_COMP_RUSSIAN_CUP, EntryYear::Current(0), false), // RusCup::RusCup
    ],
    end_day: (27, 11),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_scottish_comps`.
pub(crate) const SCOTTISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x36a1, 0x36bd, 0xa82260), // "create_scottish_comps() : unable to allocate memory"
    start: (2, 7, EntryYear::Current(0), 0x5),
    end: (24, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(7),
    calloc: None,
    comps: &[
        (0xee, 0x7f0b80, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, EntryYear::Current(0), false), // ScoPrm::ScoPrm
        (0xee, 0x7edfc0, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, EntryYear::Current(0), false), // ScoFirst::ScoFirst
        (0xee, 0x7f3250, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, EntryYear::Current(0), false), // ScoSecond::ScoSecond
        (0xee, 0x7f41f0, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION, EntryYear::Current(0), false), // ScoThird::ScoThird
        (0xb2, 0x7ed310, KEY_CLUB_COMP_SCOTTISH_CUP, EntryYear::Current(0), false), // ScoFaCup::ScoFaCup
        (0xb3, 0x7ef9e0, KEY_CLUB_COMP_SCOTTISH_LEAGUE_CUP, EntryYear::Current(0), false), // ScoLgeCup::ScoLgeCup
        (0xb2, 0x7ec980, KEY_CLUB_COMP_SCOTTISH_LEAGUE_CHALLENGE_CUP, EntryYear::Current(0), false), // ScoChalCup::ScoChalCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_south_korean_comps`.
pub(crate) const SOUTH_KOREAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x36e9, 0x3705, 0xa82294), // "create_spanish_comps() : unable to allocate memory"
    start: (25, 2, EntryYear::Start(0), 0x6),
    end: (25, 10, EntryYear::Start(0), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x66ae70, KEY_CLUB_COMP_KOREAN_LEAGUE, EntryYear::Start(0), false), // KorLeague::KorLeague
        (0xb2, 0x66a460, KEY_CLUB_COMP_KOREAN_FA_CUP, EntryYear::Start(0), false), // KorFaCup::KorFaCup
        (0xf6, 0x66bf00, KEY_CLUB_COMP_KOREAN_LEAGUE_CUP, EntryYear::Start(0), false), // KorLeagueCup::KorLeagueCup
        (0xb2, 0x66d820, KEY_CLUB_COMP_KOREAN_SUPER_CUP, EntryYear::Start(0), false), // KorSuperCup::KorSuperCup
    ],
    end_day: (20, 0),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_KOREAN_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_spanish_comps`.
pub(crate) const SPANISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x372e, 0x374d, 0xa82294), // "create_spanish_comps() : unable to allocate memory"
    start: (1, 8, EntryYear::Current(0), 0x6),
    end: (14, 5, EntryYear::Current(1), 0x5),
    count: EntryCount::ExtraDivision(4, 5, ExtraGuard::CountIsLarger),
    calloc: None,
    comps: &[
        (0xf2, 0x84e130, KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, EntryYear::Current(0), false), // SpaFirst::SpaFirst
        (0xee, 0x8539a0, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION, EntryYear::Current(0), false), // SpaSecond::SpaSecond
        (0xee, 0x855f40, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B, EntryYear::Current(0), true), // SpaSecondB::SpaSecondB
        (0xb2, 0x84cf50, KEY_CLUB_COMP_SPANISH_CUP, EntryYear::Current(0), false), // SpaCup::SpaCup
        (0xb2, 0x8586b0, KEY_CLUB_COMP_SPANISH_SUPER_CUP, EntryYear::Current(0), false), // SpaSuper::SpaSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_SPANISH_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_swedish_comps`.
pub(crate) const SWEDISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x377d, 0x379f, 0xa822c8), // "create_swedish_comps() : unable to allocate memory"
    start: (6, 3, EntryYear::Start(0), 0x6),
    end: (26, 9, EntryYear::Start(0), 0x6),
    count: EntryCount::ExtraDivision(3, 4, ExtraGuard::CountIsLarger),
    calloc: None,
    comps: &[
        (0xee, 0x88fb10, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, EntryYear::Start(0), false), // SwePrm::SwePrm
        (0xee, 0x88e5e0, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION, EntryYear::Start(0), false), // SweFirst::SweFirst
        (0x11e, 0x8916f0, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION, EntryYear::Start(0), true), // SweSecond::SweSecond
        (0xb8, 0x88cbb0, KEY_CLUB_COMP_SWEDISH_CUP, EntryYear::Current(0), false), // SweCup::SweCup
    ],
    end_day: (27, 11),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_turkish_comps`.
pub(crate) const TURKISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x37c9, 0x37e5, 0xa82294), // "create_spanish_comps() : unable to allocate memory"
    start: (8, 7, EntryYear::Current(0), 0x6),
    end: (21, 4, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x8fcf60, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION, EntryYear::Current(0), false), // TurFirst::TurFirst
        (0xee, 0x8fe6f0, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, EntryYear::Current(0), false), // TurSecond::TurSecond
        (0xee, 0x8ff4c0, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B, EntryYear::Current(0), false), // TurSecondB::TurSecondB
        (0xb2, 0x8fc3b0, KEY_CLUB_COMP_TURKISH_FA_CUP, EntryYear::Current(0), false), // TurCup::TurCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_united_states_comps`.
pub(crate) const UNITED_STATES_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x380b, 0x3827, 0xa822fc), // "create_united_states_comps() : unable to allocate memory"
    start: (22, 2, EntryYear::Start(0), 0x5),
    end: (6, 0, EntryYear::Start(1), 0x2),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xee, 0x906f80, KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE, EntryYear::Start(0), false), // UsaMls::UsaMls
        (0xb2, 0x90aeb0, KEY_CLUB_COMP_US_OPEN_CUP, EntryYear::Start(0), false), // UsaOpenCup::UsaOpenCup
        (0xb2, 0x90a150, KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS, EntryYear::Start(0), false), // UsaMlsAllStars::UsaMlsAllStars
    ],
    end_day: (20, 1),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// The entry of `create_welsh_comps`.
pub(crate) const WELSH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x384c, 0x3868, 0xa82294), // "create_spanish_comps() : unable to allocate memory"
    start: (19, 7, EntryYear::Current(0), 0x5),
    end: (6, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x924e30, KEY_CLUB_COMP_WELSH_PREMIER_DIVISION, EntryYear::Current(0), false), // WelFirst::WelFirst
        (0xb2, 0x924420, KEY_CLUB_COMP_WELSH_CUP, EntryYear::Current(0), false), // WelCup::WelCup
        (0xf6, 0x925e90, KEY_CLUB_COMP_WELSH_LEAGUE_CUP, EntryYear::Current(0), false), // WelLgeCup::WelLgeCup
        (0xf6, 0x9277f0, KEY_CLUB_COMP_WELSH_PREMIER_CUP, EntryYear::Current(0), false), // WelPrmCup::WelPrmCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Untouched,
    cup: EntryLink::Untouched,
    weeks: false,
};

pub(crate) const FN_CRT_MKDIR: usize = 0x945bd2; // _mkdir(path) -> 0 on success [inference: CRT, returns nonzero on failure]

pub(crate) const FN_UTILS_SEED_RANDOM: usize = 0x90d0a0; // utils_seed_random(seed)

pub(crate) const FN_PLAYER_SETUP_CFG_LOAD: usize = 0x7a30c0; // player_setup_cfg_load(type_mask, report_errors)

pub(crate) const FN_SCREENMANAGER_SET_DEFAULT_SCREEN: usize = 0x7feb20; // thiscall(sm, build, input, flag, leave, exit, slots, count) ret 0x1c

pub(crate) const FN_GUI_ADD_MENU_BUTTONS: usize = 0x5ed300; // gui_add_menu_buttons(x1, y1, x2, y2, cols, rows, font, count, items, bold, sort)

pub(crate) const FN_OPEN_DISK_FULL_BOX: usize = 0x826a60; // open_disk_full_box(reason)

pub(crate) const FN_OPEN_NO_MEMORY_BOX: usize = 0x826ce0; // open_no_memory_box(reason)

pub(crate) const FN_SELECT_DEFAULT_LEAGUE_BY_LANGUAGE: usize = 0x827d50; // select_default_league_by_language() -> 1 or 0

pub(crate) const FN_KEY_NATION_EXTRA_DIVISION_NAME: usize = 0x669f50; // key_nation_extra_division_name(nation, buffer, size) -> 1 or 0

pub(crate) const FN_LOAD_GAME_DATA: usize = 0x82a9c0; // load_game_data(path) -> 1 or 0

pub(crate) const G_NETWORK_PLAY_ENABLED: usize = 0xa801c4; // g_network_play_enabled: 1, 0 with -nonetwork

pub(crate) const G_NETWORK_GAME_NAME: usize = 0xdd70dc; // g_network_game_name

pub(crate) const G_NETWORK_GAME_NAMED_FLAG: usize = 0xdd712c; // g_network_game_named_flag

pub(crate) const G_DEFAULT_SCREEN_SLOT_VALUES: usize = 0xdd7148; // g_default_screen_slot_values (5 dwords)

pub(crate) const G_PLAYER_SETUP_GERMAN_FLAG: usize = 0xb74348; // g_player_setup_german_flag (byte)

pub(crate) const G_USE_REAL_PLAYERS: usize = 0x9b5981; // g_use_real_players (byte)

pub(crate) const G_KEY_NATION_ORDER: usize = 0xb63d38; // the 35 sorted key nation indices that key_nation_sorted_order rewrites

pub(crate) const G_GUI_COLOUR_DARK_RED: usize = 0xaebdcc; // g_gui_colour_dark_red

pub(crate) const G_GUI_COLOUR_ORANGE: usize = 0xaebdd4; // g_gui_colour_orange

pub(crate) const SETUP_TITLE_TEXT: usize = 0xa80288; // "Championship Manager 2001/02"

pub(crate) const CONFIG_SETTINGS_SEEN: usize = 0x213; // nonzero once the game settings were confirmed [inference: 0 opens the settings box first]

pub(crate) const FN_SCREENMANAGER_GET_LOCAL_ADDRESSES: usize = 0x7fc7c0; // thiscall(sm, out) ret 4 -> count of 20-byte addresses

pub(crate) const FN_SCREENMANAGER_IS_PLAYER_DONE: usize = 0x7fff20; // thiscall(sm, player) ret 4 -> 0 or 1

pub(crate) const FN_SCREENMANAGER_COUNT_IDLE_CHANNELS: usize = 0x7fff70; // thiscall(sm) -> count (a short)

pub(crate) const FN_SCREENMANAGER_RUN_NETWORK_CLIENT: usize = 0x7fdc00; // thiscall(sm)

pub(crate) const FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY: usize = 0x5fdf40; // thiscall(HUMAN_MANAGER, staff) ret 4 -> char *

pub(crate) const NETWORK_JOIN_CALLBACK: usize = 0x838a80; // the player joined hook given to start_network_session

pub(crate) const NETWORK_LEAVE_CALLBACK: usize = 0x838c00; // the player left hook

pub(crate) const NETWORK_GAME_COUNT: usize = 0xc3a6a; // the Network object's count of games found (a short) [inference]

pub(crate) const NETWORK_GAME_LIST: usize = 0xc3a6c; // the Network object's list of games found (10 names of 50 bytes) [inference]

pub(crate) const G_NETWORK_GAME_LIST_COPY: usize = 0xdd6cd0; // g_network_game_list_copy (10 entries of 50 bytes)

pub(crate) const G_MANAGER_STATUS_CONFIRMED_FLAG: usize = 0xdd6ed0; // g_manager_status_confirmed_flag

pub(crate) const G_GUI_COLOUR_PALE_CYAN: usize = 0xaebde8; // g_gui_colour_pale_cyan

pub(crate) const DISPLAY_ITEM_TEXT: usize = 0xba9de; // the text of display item k (+0x18c x k; item +0x80) [inference]

pub(crate) const HUMAN_MANAGER_LOCKED_COUNT: usize = 0x8; // the HUMAN_MANAGER count Manager Status watches (param 2; nonzero: no Finish) [inference]

pub(crate) const HUMAN_MANAGER_WATCHED_COUNT: usize = 0xc; // the HUMAN_MANAGER count Manager Status watches (param 4) [inference]

pub(crate) const FN_NATION_MENU_SPLIT_CONSTRUCT: usize = 0x66a280; // thiscall(split, filtered_only, order, by_second_name) ret 0xc

pub(crate) const FN_CRT_TOUPPER: usize = 0x9471cd; // toupper(c)

pub(crate) const SETUP_NATIONALITY_COMPARE: usize = 0x839530; // the qsort comparer of the nationality list (by name)

pub(crate) const SETUP_CLUB_COMPARE: usize = 0x540080; // the qsort comparer of the team list (by name)

pub(crate) const G_TEXT_BUFFER_CHAR_2: usize = 0xde1f66; // g_text_buffer_char_2

pub(crate) const COMPETITION_ABBREVIATION: usize = 0x53; // the club competition's abbreviation text [inference: shown next to the club]

pub(crate) const CONTINENT_SHORT_NAME: usize = 0x1f; // a continent name text [inference: shown next to national sides]

/// The 33 major nations of the nationality filter (the key nation ids, in the original's order).
pub(crate) const SETUP_MAJOR_NATIONS: [usize; 33] = [KEY_NATION_ARGENTINA, KEY_NATION_AUSTRALIA, KEY_NATION_BELGIUM, KEY_NATION_BRAZIL,
    KEY_NATION_CANADA, KEY_NATION_CROATIA, KEY_NATION_DENMARK, KEY_NATION_ENGLAND, KEY_NATION_FINLAND, KEY_NATION_FRANCE,
    KEY_NATION_GERMANY, KEY_NATION_GREECE, KEY_NATION_HOLLAND, KEY_NATION_HONG_KONG, KEY_NATION_ICELAND, KEY_NATION_ISRAEL,
    KEY_NATION_ITALY, KEY_NATION_JAPAN, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_NORWAY, KEY_NATION_POLAND,
    KEY_NATION_PORTUGAL, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_RUSSIA, KEY_NATION_SCOTLAND, KEY_NATION_SINGAPORE,
    KEY_NATION_SOUTH_AFRICA, KEY_NATION_SOUTH_KOREA, KEY_NATION_SPAIN, KEY_NATION_SWEDEN, KEY_NATION_TURKEY,
    KEY_NATION_UNITED_STATES, KEY_NATION_WALES];

pub(crate) const FN_HUMAN_MANAGER_SET_COMPUTER_CONTROLLED: usize = 0x5fe140; // thiscall(HM, value, staff) ret 8

pub(crate) const FN_MATCH_DAY_IS_CLUB_PLAYING: usize = 0x6b59a0; // thiscall(MATCH_DAY, club, watched_only) ret 8

pub(crate) const FN_FOG_OF_WAR_ADD_MANAGER_MAP: usize = 0x5afd70; // thiscall(FOG_OF_WAR, staff) ret 4

pub(crate) const G_SCREEN_MANAGER_PLAYER_JOINED_CALLBACK: usize = 0xdd6bf0; // g_screen_manager_player_joined_callback

pub(crate) const G_SCREEN_MANAGER_PLAYER_LEFT_CALLBACK: usize = 0xdd6bf4; // g_screen_manager_player_left_callback

pub(crate) const SETUP_MASTER_PASSWORD: usize = 0xa8099c; // "sportsinteractive": accepted for any manager's password

pub(crate) const CLUB_HUMAN_MANAGER: usize = 0xcf; // the club's human manager (a staff record) [inference: named in the box]

pub(crate) const FN_CONFIG_COPY: usize = 0x5d7ad0; // Config::Config(this, source) thiscall ret 4

pub(crate) const FN_CONFIG_DESTROY: usize = 0x5d7b70; // Config::~Config thiscall

pub(crate) const FN_CONFIG_SET_DEFAULTS: usize = 0x5d7bc0; // Config::set_defaults thiscall -> 1

pub(crate) const FN_CONFIG_SET_CURRENCY: usize = 0x5d8270; // Config::set_currency(this, currency) thiscall ret 4

pub(crate) const FN_CONFIG_SET_FONT: usize = 0x5d8b50; // Config::set_font(this, font) thiscall ret 4

pub(crate) const FN_CONFIG_SET_AUTO_SAVE_INTERVAL: usize = 0x5d83e0; // Config::set_auto_save_interval(this, days) thiscall ret 4

pub(crate) const FN_CONFIG_GET_FONT_INDEX: usize = 0x5d8b40; // Config::get_font_index thiscall -> int

pub(crate) const FN_CASH_GET_CURRENCY_NAME: usize = 0x4415f0; // cash_get_currency_name(buffer, size, currency) -> 1 or 0

pub(crate) const FN_FONT_GET_MODE_NAME: usize = 0x5b2040; // font_get_mode_name(font, buffer, size)

pub(crate) const FN_FONT_GET_FILE_COUNT: usize = 0x5b1fc0; // font_get_file_count() -> count

pub(crate) const FN_IS_FONT_ALLOWED_FOR_LANGUAGE: usize = 0x5d8b80; // is_font_allowed_for_language(language, name) -> 1 or 0

pub(crate) const G_GUI_COLOUR_SILVER: usize = 0xae3154; // g_gui_colour_silver

pub(crate) const SETTINGS_LANGUAGE_NAMES: usize = 0x9eb290; // the 19 language names ("English"...)

pub(crate) const CONFIG_OPTIONS: usize = 0x4; // the Config's option bits

pub(crate) const CONFIG_LANGUAGE: usize = 0x8; // the Config's language

pub(crate) const CONFIG_CURRENCY: usize = 0xc; // the Config's currency (byte)

pub(crate) const CONFIG_AUTO_SAVE_DAYS: usize = 0xd; // the Config's auto save interval in days (short)

pub(crate) const CONFIG_SIZE: usize = 0x31b; // the Config object (operator new size of the settings copy)

pub(crate) const FN_CRT_GETDRIVE: usize = 0x9473a7; // _getdrive() -> drive number (1 = A)

pub(crate) const FN_CRT_LOCALTIME: usize = 0x946c40; // localtime(&time_t) -> struct tm *

pub(crate) const FN_HALL_OF_FAME_GET_TABLE_NAME: usize = 0x5f07c0; // hall_of_fame_get_table_name(table, buffer, size, short_form) -> 1 or 0

pub(crate) const FN_HALL_OF_FAME_SORT_LEAGUE_ORDER: usize = 0x5f10c0; // hall_of_fame_sort_league_order(name_style) -> u8 * (28 tables)

pub(crate) const FN_HALL_OF_FAME_GET_ENTRY: usize = 0x5ef130; // thiscall(HallOfFame, table, rank, entry *) ret 0xc -> 1 or 0

pub(crate) const FN_HALL_OF_FAME_EXPORT_TO_CSV: usize = 0x5f0430; // thiscall(HallOfFame, path) ret 4 -> 1 or 0

pub(crate) const FN_HALL_OF_FAME_IMPORT_FILE: usize = 0x5efd50; // thiscall(HallOfFame, path, check_only) ret 8 -> 1 or 0

pub(crate) const HALL_OF_FAME_TRY_LOAD_QUIET: usize = 0x5f1090; // hall_of_fame_try_load_quiet(path): the Merge In dialog's file check

pub(crate) const G_GUI_COLOUR_GREEN: usize = 0xae31aa; // g_gui_colour_green

pub(crate) const G_GUI_COLOUR_DARK_PURPLE: usize = 0xae3192; // g_gui_colour_dark_purple

pub(crate) const HALL_OF_FAME_ENTRY_NAME: usize = 0x0; // the manager's name (text)

pub(crate) const HALL_OF_FAME_ENTRY_CLUB: usize = 0x32; // the club's name (text) [inference: second column of the list]

pub(crate) const HALL_OF_FAME_ENTRY_DATE: usize = 0x64; // time_t of the entry (0: none)

pub(crate) const HALL_OF_FAME_ENTRY_POINTS: usize = 0x68; // the points (0: empty rank)

pub(crate) const FN_FRIENDLY_REMOVE_CLUB_FROM_AI_PLANS: usize = 0x5c64e0; // thiscall(Friendly, club) ret 4

pub(crate) const FN_DATABASE_TRIM_SENIOR_AND_B_TEAM_SQUADS: usize = 0x541e60; // cdecl()

pub(crate) const FN_VERIFY_BACKGROUND_CONFIG: usize = 0x41bc20; // cdecl()

pub(crate) const FN_VERIFY_HISTORY_FILES: usize = 0x5f2840; // cdecl()

pub(crate) const FN_VALIDATE_EURO_CFG: usize = 0x905d40; // cdecl()

pub(crate) const FN_DATABASE_BUILD_NATION_NAME_INDEX: usize = 0x539750; // cdecl()

pub(crate) const FN_DATABASE_PRUNE_COMP_HISTORY_TABLE: usize = 0x539ae0; // cdecl()

pub(crate) const FN_DATABASE_TRIM_STAFF_TABLES: usize = 0x524d70; // cdecl(database_size)

pub(crate) const FN_DATABASE_TRIM_CLUB_TABLE: usize = 0x5267c0; // cdecl(database_size)

pub(crate) const FN_FIX_DIVISION_CLUB_COUNT: usize = 0x839920; // cdecl(comp_id, wanted)

pub(crate) const FN_DATABASE_MERGE_B_TEAM_STAFF: usize = 0x544970; // cdecl()

pub(crate) const FN_SETUP_SET_GAME_START_DATE: usize = 0x830520; // cdecl() -> 1 or 0 (ignored)

pub(crate) const FN_DATABASE_PLAYER_INITIALISATION: usize = 0x535a80; // cdecl(game_date)

pub(crate) const FN_DATABASE_APPLY_FAKE_PLAYERS: usize = 0x532e00; // cdecl()

pub(crate) const FN_CONTRACT_MANAGER_INITIALISE_ALL_STAFF_CONTRACTS: usize = 0x4c7560; // thiscall(CONTRACT_MANAGER)

pub(crate) const FN_GAME_RUN_TO_START_DATE: usize = 0x5cc850; // cdecl()

pub(crate) const G_DATABASE_SIZE_OPTION: usize = 0xdd7114; // the database size of the new game (Config options & 7), byte

pub(crate) const MANAGER_MANAGER_CLUB_RECORDS: usize = 0x0; // the MANAGER_MANAGER's table of 0x49-byte club records [inference: indexed by club id]

/// The steps of `create_game` from 2 on, in the original's order, with the progress step of each (0x2c in all).
pub(crate) const CREATE_STEPS: [(u32, CreateStep); 0x2d] = [
    (2, CreateStep::Routine(FN_KEY_NATION_NUMBER_ACTIVE_ENTRIES, 0x1f6d)),
    (3, CreateStep::Object(0x609, 0x893dc0, G_TACTICS, 0x1f82, CreateArg::Zero)),
    (4, CreateStep::Object(0x10, 0x5fb430, G_HUMAN_MANAGER, 0x1f97, CreateArg::Zero)),
    (5, CreateStep::Object(0xe8, 0x7f6760, G_SCOUT_MANAGER, 0x1fac, CreateArg::Zero)),
    (6, CreateStep::Object(0x14, 0x785b20, G_NEWS, 0x1fc1, CreateArg::Zero)),
    (7, CreateStep::Object(0x3507, 0x5c1f80, G_FRIENDLY, 0x1fd6, CreateArg::Alone)),
    (8, CreateStep::Object(0x698, 0x903360, G_UEFA_SEEDING, 0x1feb, CreateArg::Alone)),
    (9, CreateStep::Object(0x75, 0x4c5fe0, G_CONMEBOL_SEEDING, 0x2000, CreateArg::Alone)),
    (0xa, CreateStep::Object(0x18, 0x868390, G_STAFF_RECORDS, 0x2015, CreateArg::Zero)),
    (0xb, CreateStep::Object(0x20, 0x729000, G_MATCH_STATS, 0x2037, CreateArg::Stats)),
    (0xc, CreateStep::Object(0x30, 0x444520, G_CLUB_RECORDS, 0x204c, CreateArg::Zero)),
    (0xd, CreateStep::Object(0x48, 0x7bfd50, G_PLAYER_STATS, 0x2061, CreateArg::Zero)),
    (0xe, CreateStep::Object(0x10, 0x442ab0, G_CLUB_HISTORY, 0x2076, CreateArg::Zero)),
    (0xf, CreateStep::Object(6, 0x5f97c0, G_HOST_COUNTRY, 0x208b, CreateArg::Alone)),
    (0x10, CreateStep::Object(0x2a, 0x5554a0, G_DISCIPLINE, 0x20a0, CreateArg::Zero)),
    (0x11, CreateStep::Object(0xe0, 0x413180, G_AWARD_MANAGER, 0x20b5, CreateArg::Zero)),
    (0x12, CreateStep::Object(0xc, 0x5a4fc0, G_FINE, 0x20c9, CreateArg::Zero)),
    (0x13, CreateStep::Object(0x10, 0x418440, G_AWOL, 0x20dd, CreateArg::Zero)),
    (0x14, CreateStep::Object(8, 0x5697e0, G_DISPUTE, 0x20f1, CreateArg::Zero)),
    (0x15, CreateStep::Object(0x1070, 0x794ad0, G_NOTES, 0x2104, CreateArg::Zero)),
    (0x16, CreateStep::Object(0x2c, 0x62a680, G_INJURY, 0x2119, CreateArg::Zero)),
    (0x17, CreateStep::Object(0x3a, 0x76a4e0, G_NATIONAL_TEAMS, 0x212e, CreateArg::Zero)),
    (0x18, CreateStep::Object(0x6a, 0x90d140, G_VIRTUAL_STAFF, 0x2143, CreateArg::Alone)),
    (0x19, CreateStep::Object(0x19, 0x85b670, G_STADIUM, 0x2158, CreateArg::Alone)),
    (0x1a, CreateStep::Object(0x1184, 0x5a9cf0, G_FIXTURE_MANAGER, 0x216d, CreateArg::Fixture)),
    (0x1b, CreateStep::Object(0xab, 0x7e5fc0, G_RESERVE_COMP, 0x2182, CreateArg::Zero)),
    (0x1b, CreateStep::Routine(FN_KEY_NATION_CREATE_ALL_COMPS, 0x2193)),
    (0x1c, CreateStep::Routine(FN_BUILD_COMP_OBJECT_TABLE, 0x21a7)),
    (0x1d, CreateStep::Object(2, 0x859aa0, G_SQUAD_MANAGER, 0x21bc, CreateArg::Zero)),
    (0x1e, CreateStep::Method(FN_RESERVECOMP_CREATE_SUB_COMPS, G_RESERVE_COMP)),
    (0x1f, CreateStep::Object(4, 0x59a370, G_FINANCE_TABLE, 0x21e1, CreateArg::Zero)),
    (0x20, CreateStep::Object(0x3c, 0x4c7040, G_CONTRACT_MANAGER, 0x21f6, CreateArg::Zero)),
    (0x21, CreateStep::Method(FN_FINANCETABLE_REFRESH_ALL_WAGE_BILLS, G_FINANCE_TABLE)),
    (0x22, CreateStep::Object(0x94a, 0x8bba30, G_TRANSFER_MANAGER, 0x221e, CreateArg::Zero)),
    (0x23, CreateStep::Clubs),
    (0x24, CreateStep::Object(0x60, 0x7a5380, G_PLAYER_REGEN, 0x2243, CreateArg::Zero)),
    (0x25, CreateStep::Object(0x1c, 0x68dbf0, G_MANAGER_MANAGER, 0x2258, CreateArg::Zero)),
    (0x26, CreateStep::Object(0x18, 0x72b1d0, G_MEDIA, 0x226d, CreateArg::Zero)),
    (0x27, CreateStep::Object(0x40, 0x5af630, G_FOG_OF_WAR, 0x2282, CreateArg::Zero)),
    (0x28, CreateStep::Object(0x10, 0x79b580, G_OFFICIALS_MANAGER, 0x2297, CreateArg::Zero)),
    (0x29, CreateStep::Method(FN_SQUAD_MANAGER_INIT_NEW_GAME, G_SQUAD_MANAGER)),
    (0x2a, CreateStep::Object(8, 0x84b940, G_SIMULATED_STATS, 0x22bc, CreateArg::Zero)),
    (0x2b, CreateStep::Object(0xc, 0x9226c0, G_WEATHER, 0x22d1, CreateArg::Alone)),
    (0x2c, CreateStep::Object(0x688f, 0x8b0ec0, G_TRAINING_MANAGER, 0x22e6, CreateArg::Zero)),
    (0x2c, CreateStep::Object(0xa, 0x7da190, G_PUNDIT_LIST, 0x22f8, CreateArg::Language)),
];

pub(crate) const FN_FRIENDLY_REGISTER_COMP_OBJECTS: usize = 0x5ca030; // Friendly::register_comp_objects thiscall(Friendly)

pub(crate) const FN_NETWORK_HOST_OR_JOIN_GAME: usize = 0x77a9f0; // Network::host_or_join_game thiscall(network, join, name) ret 8 -> int

pub(crate) const FN_DISPLAY_ALLOC_GUIO_SNAPSHOT_POOL: usize = 0x55f1c0; // display_alloc_guio_snapshot_pool stdcall(slots) ret 4

pub(crate) const FN_MATCH_DAY_DROP_VIEWER_AND_FINISH_MATCH: usize = 0x6b6970; // match_day_drop_viewer_and_finish_match()

pub(crate) const KEY_NATION_ENTRY_COUNT: usize = 35; // the rows of G_LEAGUE_TABLE

pub(crate) const KEY_NATION_RECORD_SIZE: usize = 0x48; // the size of a G_LEAGUE_TABLE row

pub(crate) const KEY_NATION_RECORD_NATION: usize = 0x0; // the nation of the entry (null: an international entry)

pub(crate) const KEY_NATION_RECORD_COMP_COUNT: usize = 0xc; // the number of competitions the entry created

pub(crate) const KEY_NATION_RECORD_COMPS: usize = 0x10; // the entry's array of competition objects (calloc)

pub(crate) const KEY_NATION_RECORD_START_YEAR: usize = 0x35; // 2 bytes: Date::Date year of the season start

pub(crate) const KEY_NATION_RECORD_START_WEEKDAY: usize = 0x37; // 2 bytes: Date::Date weekday of the season start

pub(crate) const KEY_NATION_RECORD_END_MONTH: usize = 0x43; // 1 byte: Date::Date month of the season end

pub(crate) const KEY_NATION_RECORD_END_YEAR: usize = 0x44; // 2 bytes: Date::Date year of the season end

pub(crate) const KEY_NATION_RECORD_END_WEEKDAY: usize = 0x46; // 2 bytes: Date::Date weekday of the season end

pub(crate) const HUMAN_MANAGER_COUNT_8: usize = 0x8; // a HUMAN_MANAGER count tested by the join hook [inference: humans already in the game]

/// The Setup.cpp source name the error box shows.
pub(crate) const SETUP_CPP_FILE: usize = 0xa801dc; // "E:\dev\CM3\cm3 00-01\cm3\code\Setup.cpp"

pub(crate) const FN_KEY_NATION_NUMBER_ACTIVE_ENTRIES: usize = 0x669140; // cdecl() -> 1

pub(crate) const FN_KEY_NATION_CREATE_ALL_COMPS: usize = 0x669180; // cdecl() -> int

pub(crate) const FN_BUILD_COMP_OBJECT_TABLE: usize = 0x8386f0; // cdecl() -> 1 or 0

pub(crate) const FN_FINANCETABLE_REFRESH_ALL_WAGE_BILLS: usize = 0x59cf50; // thiscall(FinanceTable)

pub(crate) const FN_SQUAD_MANAGER_INIT_NEW_GAME: usize = 0x859dc0; // thiscall(SQUAD_MANAGER)

/// One step of `create_game` after the start: `Routine(f, row)` a cdecl() that fails at Setup.cpp `row` when it returns
/// 0; `Method(f, global)` a thiscall on the subsystem in `global`; `Object(size, constructor, global, row, arg)`; the club
/// pass (`Clubs`).
#[derive(Clone, Copy)]
pub(crate) enum CreateStep {
    Routine(usize, u32),
    Method(usize, usize),
    Object(u32, usize, usize, u32, CreateArg),
    Clubs,
}

/// How a subsystem's constructor is called by `create_game` (thiscall, `this` = the `operator new`).
#[derive(Clone, Copy)]
pub(crate) enum CreateArg {
    /// (this, 0)
    Zero,
    /// (this)
    Alone,
    /// (this, 0, matches stored): Match_Stats
    Stats,
    /// (this, game year, 0, 1): FIXTURE_MANAGER (the original pushes a dword with stack leftover above the year; the
    /// constructor only reads the word)
    Fixture,
    /// (this, the names language byte): PunditList
    Language,
}

/// The Setup Game screen's answer to item `item`.
///
/// Param 0 set (first time): cleared, the game settings box opens (`open_game_settings_box`(0)) and −4. A "-load <file>"
/// on the command line (`setup_take_load_argument`) with a ".sav" or ".qst" extension loads it right away
/// (`G_PENDING_COMMAND_CODE` and `G_PENDING_COMMAND_ARG` zeroed, `exe::config_apply_and_save`, `setup_load_chosen_game`
/// with the box functions, fatal line 0x1b71): with no human manager (`count_human_managers`(0) = 0) Manager Status opens
/// with a new manager (`setup_open_manager_status_screen`, `setup_add_human_manager`) and −4; with network managers to
/// assign (`setup_assign_restored_network_managers`) Manager Status and −4; otherwise −5. Otherwise, a
/// `quick_start.cfg` in the game folder produces the quick starts (`setup_process_quick_start_file`; a file that does not
/// open: box 0x1bb2; a failure: box 0x378) and then runs `auto_setup.cfg` if it exists (box 0x37f when it fails; −5), or
/// ends the game (fatal cleanup, exit(0)). Without it, an `auto_setup.cfg` creates the automatic career
/// (`create_game_from_setup_file`("auto_setup.cfg", null); box 0x38f when it fails) and −5.
///
/// With a command waiting (`G_PENDING_COMMAND_DISPATCH_FLAG`): codes 4 (quick start) and 5 (saved game) from the file
/// dialog load the chosen file (`setup_load_chosen_game` freeing the name, the boxes opened here, fatal line 0x414);
/// without staff (box 0x39d8) or with no human manager among the last 16 staff (`staff::CLASSIFICATION` = 1) Manager
/// Status opens with its params 0..4 zeroed and a new manager is added (−4); otherwise the restored network managers
/// decide (−5 when none, else Manager Status with params 0..4 zeroed, −4). Code 6 deletes the chosen file (`remove`);
/// when it cannot, the "Unable To Delete" box with the path (−4). The name is freed.
///
/// Then the clicked id (`setup_clicked_id`): 1 Start New Game (`setup_start_new_game`), 2 Quick Start Game (the file
/// dialog on CM3_QSTART, ".qst", `check_quick_start_file`, code 4), 3 Restore Saved Game (the save folder, ".sav", the
/// save name, `check_saved_game_file`, code 5), 4 Delete Saved Game (mode 2, code 6), 5 Network Play (a network
/// session must open, `ScreenManager::start_network_session`: it is closed again and the Network Play screen opens;
/// otherwise the "Unable To Open Network" box), 6 Game Settings (`open_game_settings_box`(1)), 7 Hall Of Fame (params
/// 0..2 zeroed), 8 credits, 9 web sites, all −4; −3 returns 0; any other goes to the menu bar
/// (`exe::menubar_handle_command`, otherwise `exe::menubar_handle_object_link`).
/// Original: `setup_handle_setup_game_input` (0x81bd70, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_handle_setup_game_input(item: u32) -> i32 {
    covered!("tel11_1 setup game input");
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let settings: unsafe extern "C" fn(u32) = core::mem::transmute(FN_OPEN_GAME_SETTINGS_BOX);
    let assign: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_SETUP_ASSIGN_RESTORED_NETWORK_MANAGERS);
    let open_status: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_OPEN_MANAGER_STATUS_SCREEN);
    let add_manager: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_ADD_HUMAN_MANAGER);
    let new_game: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_START_NEW_GAME);
    let create: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CREATE_GAME_FROM_SETUP_FILE);
    let credits: unsafe extern "C" fn() = core::mem::transmute(FN_CREDITS_SCREEN_OPEN);
    let web_sites: unsafe extern "C" fn() = core::mem::transmute(FN_WEB_SITES_SCREEN_OPEN);
    let config = || read_usize(G_CONFIG);
    if exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0) != 0 {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, 0usize, 0);
        settings(0);
        return -4;
    }
    let p = exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa803bc); // "-load"
    let mut path = [0u8; 0x100];
    let mut ext = [0u8; 0x100];
    let mut load_mode = 0;
    if p != 0 {
        setup_take_load_argument(p, &mut path);
        exe::crt_strtok(path.as_ptr() as usize, 0usize, 0usize, core::ptr::null_mut(), ext.as_mut_ptr());
        if exe::crt_strstr(ext.as_ptr() as usize, 0xa5aab8) != 0 {
            load_mode = 5; // ".sav"
        } else if exe::crt_strstr(ext.as_ptr() as usize, 0xa803b4) != 0 {
            load_mode = 4; // ".qst"
        }
    }
    if load_mode != 0 {
        covered!("tel11_1 setup game -load");
        write_u32(G_PENDING_COMMAND_CODE, 0);
        exe::config_apply_and_save(config());
        write_u32(G_PENDING_COMMAND_ARG, 0);
        if let Some(r) = setup_load_chosen_game(path.as_ptr() as usize, load_mode, false, true, 0x1b71, true) {
            return r;
        }
        if exe::count_human_managers(0) == 0 {
            open_status();
            add_manager();
            return -4;
        }
        if assign() != 0 {
            open_status();
            return -4;
        }
        return -5;
    }
    if exe::utils_file_exists(0xa803a4) != 0 {
        // "quick_start.cfg": the quick start production, then the automatic career or the end
        covered!("tel11_1 setup game quick start file");
        let f = exe::crt_fopen(0xa803a4usize, 0x9899fc);
        let ok = if f == 0 {
            setup_warning(0x1bb2);
            false
        } else {
            let ok = setup_process_quick_start_file(f);
            exe::crt_fclose(f);
            ok
        };
        if !ok {
            setup_warning(0x378);
        }
        if exe::utils_file_exists(0xa80340) == 0 {
            setup_fatal_exit(None, 0);
        }
        if create(0xa80340, 0) == 0 {
            setup_warning(0x37f);
        }
        return -5;
    }
    if exe::utils_file_exists(0xa80340) != 0 {
        // "auto_setup.cfg": the automatic career
        covered!("tel11_1 setup game auto setup");
        if create(0xa80340, 0) == 0 {
            setup_warning(0x38f);
        }
        return -5;
    }
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 {
        let code = read_u32(G_PENDING_COMMAND_CODE);
        if code == 4 || code == 5 {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            exe::config_apply_and_save(config());
            let name = read_usize(G_PENDING_COMMAND_ARG);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if name != 0 {
                covered!("tel11_1 setup game load chosen");
                if let Some(r) = setup_load_chosen_game(name, code, true, false, 0x414, true) {
                    return r;
                }
                let staff = read_usize(G_STAFF_TABLE);
                let mut humans = 0;
                if staff == 0 {
                    setup_warning(0x39d8);
                } else {
                    let n = read_i32(G_STAFF_COUNT) as isize;
                    for k in 1..=16isize {
                        if read_u8((staff as isize + (n - k) * staff::SIZE as isize) as usize + staff::CLASSIFICATION) == 1 {
                            humans += 1;
                        }
                    }
                }
                if humans == 0 {
                    setup_open_screen(0x81ff10, 0x820950, 0, 5);
                    add_manager();
                    return -4;
                }
                if assign() == 0 {
                    return -5;
                }
                setup_open_screen(0x81ff10, 0x820950, 0, 5);
                return -4;
            }
        } else if code == 6 {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            exe::config_apply_and_save(config());
            let name = read_usize(G_PENDING_COMMAND_ARG);
            if name != 0 {
                write_u32(G_PENDING_COMMAND_ARG, 0);
                if exe::crt_remove(name as *const u8) != 0 {
                    covered!("tel11_1 setup game cannot delete");
                    let mut heading = [0u8; 0x100];
                    setup_confirm_message(&mut heading, 0xa7f710, 0xa8030c, name); // "Unable To Delete", "The saved game ..."
                    exe::crt_operator_delete(name);
                    return -4;
                }
                exe::crt_operator_delete(name);
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
    }
    let raw = item;
    let item = item as i16;
    let (pale, red) = (setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_RED));
    let mut title = [0u8; 0x100];
    let mut heading = [0u8; 0x100];
    let (t, h) = (title.as_mut_ptr() as usize, heading.as_mut_ptr() as usize);
    match setup_clicked_id(item) {
        1 => {
            new_game();
            -4
        }
        2 => {
            copy_text(G_QUICK_START_FILE_PATH as *mut u8, exe::utils_build_file_path(0xa80204, 0xad9c64) as *const u8);
            let hd = text(h, 0xa7fc84); // "Select File"
            let tt = text(t, 0xa80258); // "Quick Start Game"
            exe::open_file_dialog(tt, hd, 0, G_QUICK_START_FILE_PATH, 0xa803b4, 0, 0x839290, pale, red, 4, 0, 1, 0, 1);
            -4
        }
        3 => {
            let hd = text(t, 0xa7fc84);
            let tt = text(h, 0xa80244); // "Restore Saved Game"
            exe::open_file_dialog(tt, hd, 0, config() + CONFIG_SAVE_FOLDER, 0xa5aab8, config() + config::SAVE_NAME, 0x838e10, pale, red, 5, 0, 0, 0, 1);
            -4
        }
        4 => {
            let hd = text(t, 0xa7fc84);
            let tt = text(h, 0xa80230); // "Delete Saved Game"
            exe::open_file_dialog(tt, hd, 2, config() + CONFIG_SAVE_FOLDER, 0xa5aab8, 0, 0x838e10, pale, red, 6, 0, 0, 0, 1);
            -4
        }
        5 => {
            if exe::screenmanager_start_network_session(SCREEN_MANAGER, 0, 0) != 0 {
                exe::screenmanager_end_network_session(SCREEN_MANAGER);
                exe::screenmanager_open_screen(SCREEN_MANAGER, 0x81edd0usize, 0x81ef70usize, 0, 0usize, 0);
            } else {
                setup_confirm_message(&mut heading, 0xa802a8, 0xa802c0, 0); // "Unable To Open Network"
            }
            -4
        }
        6 => {
            settings(1);
            -4
        }
        7 => {
            setup_open_screen(0x825d10, 0x826690, 0, 3); // Hall Of Fame
            -4
        }
        8 => {
            credits();
            -4
        }
        9 => {
            web_sites();
            -4
        }
        0xfffd => 0,
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Comparer of `setup_handle_setup_game_input`: only the original runs (every answer opens a screen, a box or a file
/// dialog, loads or deletes a file, or creates the game); the copy is proved in Rust mode (the automatic career goes
/// through the "auto_setup.cfg" branch).
pub(crate) unsafe extern "C" fn compare_setup_handle_setup_game_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SETUP_HANDLE_SETUP_GAME_INPUT)(item)
}
