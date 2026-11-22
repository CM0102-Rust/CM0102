// ---------- batch %%LEVA%%: the Setup screens, part 1: the front end, Setup Game, Select League(s), Select Start Season (Setup.cpp) ----------

// The exe functions these Setup screens call that have no wrapper yet (cdecl unless noted; checked at the call sites).
const FN_UTILS_IS_DIRECTORY: usize = 0x90bf90; // utils_is_directory(path) -> int
const FN_CRT_MKDIR: usize = 0x945bd2; // _mkdir(path) -> 0 on success [inference: CRT, returns nonzero on failure]
const FN_UTILS_SEED_RANDOM: usize = 0x90d0a0; // utils_seed_random(seed)
const FN_PLAYER_SETUP_CFG_LOAD: usize = 0x7a30c0; // player_setup_cfg_load(type_mask, report_errors)
const FN_SCREENMANAGER_SET_DEFAULT_SCREEN: usize = 0x7feb20; // thiscall(sm, build, input, flag, leave, exit, slots, count) ret 0x1c
const FN_GUI_ADD_MENU_BUTTONS: usize = 0x5ed300; // gui_add_menu_buttons(x1, y1, x2, y2, cols, rows, font, count, items, bold, sort)
const FN_OPEN_GAME_SETTINGS_BOX: usize = 0x823890; // open_game_settings_box(origin)
const FN_OPEN_DISK_FULL_BOX: usize = 0x826a60; // open_disk_full_box(reason)
const FN_OPEN_NO_MEMORY_BOX: usize = 0x826ce0; // open_no_memory_box(reason)
const FN_COUNT_HUMAN_MANAGERS: usize = 0x838d10; // count_human_managers(connected_only) -> int
const FN_SETUP_ASSIGN_RESTORED_NETWORK_MANAGERS: usize = 0x8387e0; // -> 1 when Manager Status must open
const FN_SETUP_OPEN_MANAGER_STATUS_SCREEN: usize = 0x81fea0; // setup_open_manager_status_screen()
const FN_SETUP_ADD_HUMAN_MANAGER: usize = 0x820d40; // setup_add_human_manager()
const FN_SETUP_START_NEW_GAME: usize = 0x81ce00; // setup_start_new_game()
const FN_CONFIRM_PUSH: usize = 0x591840; // confirm_push(title, heading, text, buttons, text_colour, colour, code)
const FN_OPEN_FILE_DIALOG: usize = 0x58f500; // open_file_dialog(title, heading, mode, dir, ext, name, on_ok, colour_a, colour_b, a10..a14)
const FN_CREDITS_SCREEN_OPEN: usize = 0x4e5920; // credits_screen_open()
const FN_WEB_SITES_SCREEN_OPEN: usize = 0x5134c0; // web_sites_screen_open()
const FN_SELECT_DEFAULT_LEAGUE_BY_LANGUAGE: usize = 0x827d50; // select_default_league_by_language() -> 1 or 0
const FN_KEY_NATION_SORTED_ORDER: usize = 0x66a0c0; // key_nation_sorted_order(by_second_name) -> u8 * (the static 35 indices at 0xb63d38)
const FN_KEY_NATION_EXTRA_DIVISION_NAME: usize = 0x669f50; // key_nation_extra_division_name(nation, buffer, size) -> 1 or 0
const FN_CREATE_GAME_FROM_SETUP_FILE: usize = 0x827110; // create_game_from_setup_file(cfg, qst) -> 1 or 0
const FN_LOAD_GAME_DATA: usize = 0x82a9c0; // load_game_data(path) -> 1 or 0

// The Setup screens' globals that have no name yet (community data map names in the comments).
const G_NETWORK_PLAY_ENABLED: usize = 0xa801c4; // g_network_play_enabled: 1, 0 with -nonetwork
const G_COMMAND_LINE_TEXT: usize = 0xae2c98; // g_command_line_text
const G_SETUP_SCREEN_TITLE: usize = 0xdd70a8; // g_setup_screen_title ("Championship Manager 2001/02")
const G_NETWORK_GAME_NAME: usize = 0xdd70dc; // g_network_game_name
const G_NETWORK_SETUP_HOST_FLAG: usize = 0xdd7128; // g_network_setup_host_flag
const G_NETWORK_GAME_NAMED_FLAG: usize = 0xdd712c; // g_network_game_named_flag
const G_DEFAULT_SCREEN_SLOT_VALUES: usize = 0xdd7148; // g_default_screen_slot_values (5 dwords)
const G_QUICK_START_FILE_PATH: usize = 0xdd6f9c; // g_quick_start_file_path
const G_PENDING_COMMAND_DISPATCH_FLAG: usize = 0xb77c84; // g_pending_command_dispatch_flag
const G_PLAYER_SETUP_GERMAN_FLAG: usize = 0xb74348; // g_player_setup_german_flag (byte)
const G_USE_REAL_PLAYERS: usize = 0x9b5981; // g_use_real_players (byte)
const G_KEY_NATION_ORDER: usize = 0xb63d38; // the 35 sorted key nation indices that key_nation_sorted_order rewrites
const G_GUI_COLOUR_DARK_RED: usize = 0xaebdcc; // g_gui_colour_dark_red
const G_GUI_COLOUR_ORANGE: usize = 0xaebdd4; // g_gui_colour_orange
const SETUP_TITLE_TEXT: usize = 0xa80288; // "Championship Manager 2001/02"
const CONFIG_SAVE_FOLDER: usize = 0xf; // the Config's save folder text [inference: passed as the folder of the restore and delete dialogs]
const CONFIG_SETTINGS_SEEN: usize = 0x213; // nonzero once the game settings were confirmed [inference: 0 opens the settings box first]
const KEY_NATION_RECORD_SETUP_FUNC: usize = 0x8; // the setup function of a league entry (null: the league cannot be selected)
const SETUP_TEXT: usize = 0xde1f64; // the shared text buffer

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
    let confirm: unsafe extern "C" fn(usize, usize, usize, u32, usize, usize, u32) = core::mem::transmute(FN_CONFIRM_PUSH);
    setup_format_title();
    let (dark_red, pale) = (setup_colour(G_GUI_COLOUR_DARK_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
    let b = text(SETUP_TEXT, body, arg);
    let h = text(heading_buffer.as_mut_ptr() as usize, heading);
    confirm(G_SETUP_SCREEN_TITLE, h, b, 0, pale, dark_red, 0);
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
    let is_directory: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_UTILS_IS_DIRECTORY);
    let mkdir: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_MKDIR);
    let sscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let seed_random: unsafe extern "C" fn(i32) = core::mem::transmute(FN_UTILS_SEED_RANDOM);
    let player_setup: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
    let set_default: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_DEFAULT_SCREEN);
    if is_directory(exe::utils_get_folder_for_key(0xa80204)) == 0 && mkdir(exe::utils_get_folder_for_key(0xa80204)) != 0 {
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
        let (x2, y2) = (core::ptr::read_unaligned(0x81bd17 as *const i32), core::ptr::read_unaligned(0x81bd12 as *const i32));
        area(current_human_display(), 0, 0, x2 as usize, y2 as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
    }
    destruct_match_tabs(&mut items);
}

/// The token after "-load" on the command line, as the original reads it: the "-load" is marked used (its '-' becomes
/// '+'), the rest of the word and the spaces are skipped, then the path is copied, between quotes or up to the next
/// space, into `out`.
unsafe fn setup_take_load_argument(p: usize, out: &mut [u8; 0x100]) {
    write_u8(p, b'+');
    let mut p = p;
    let mut c = b'+';
    while c != b' ' {
        c = read_u8(p + 1);
        p += 1;
        if c == 0 {
            break;
        }
    }
    let mut c = read_u8(p);
    while c != 0 && c == b' ' {
        c = read_u8(p + 1);
        p += 1;
    }
    let (end, mut q) = if read_u8(p) == b'"' {
        c = read_u8(p + 1);
        p += 2;
        (b'"', 0usize)
    } else {
        c = read_u8(p);
        p += 1;
        (b' ', 0usize)
    };
    out[q] = c;
    q += 1;
    if c == 0 {
        return;
    }
    loop {
        if c == end {
            out[q - 1] = 0;
            return;
        }
        c = read_u8(p);
        if q >= out.len() {
            return; // simplification: the original copies past its 256-byte buffer
        }
        out[q] = c;
        q += 1;
        p += 1;
        if c == 0 {
            return;
        }
    }
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

/// The quick start production of `quick_start.cfg` (`f` open for reading): for each `AUTO_SETUP=` line, each file the
/// pattern finds whose quick start (CM3_QSTART\<name>.qst) does not exist yet is created from it
/// (`create_game_from_setup_file`(file, name)); a failure writes the message into the shared text, shows the error box
/// (0x1bdd) and stops the search. Returns whether everything went well (the original's `ebp`).
unsafe fn setup_process_quick_start_file(f: usize) -> bool {
    let strncmp: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_947120);
    let makepath: unsafe extern "C" fn(*mut u8, usize, usize, usize, usize) = core::mem::transmute(FN_CRT_MAKEPATH);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let create: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CREATE_GAME_FROM_SETUP_FILE);
    let mut ok = true;
    let mut line = [0u8; 0x400];
    let mut find = [0u8; 0x140];
    let (mut drive, mut dir, mut cfg, mut name, mut qst) = ([0u8; 4], [0u8; 0x104], [0u8; 0x104], [0u8; 0x100], [0u8; 0x104]);
    while exe::utils_read_config_line(f, line.as_mut_ptr()) != 0 {
        exe::utils_trim_control_chars(line.as_mut_ptr());
        if strncmp(line.as_ptr() as usize, 0xa80398, 0xb) != 0 {
            continue; // not "AUTO_SETUP="
        }
        let pattern = line.as_ptr() as usize + 0xb;
        let fd = find.as_mut_ptr() as usize;
        if exe::file_find_first(pattern, fd, 0) != 0 {
            continue;
        }
        loop {
            exe::crt_strtok(pattern, drive.as_mut_ptr() as usize, dir.as_mut_ptr() as usize, core::ptr::null_mut(), core::ptr::null_mut());
            makepath(cfg.as_mut_ptr(), drive.as_ptr() as usize, dir.as_ptr() as usize, fd + 0x10, 0);
            exe::crt_strtok(fd + 0x10, 0usize, 0usize, name.as_mut_ptr(), core::ptr::null_mut());
            makepath(qst.as_mut_ptr(), 0, 0, name.as_ptr() as usize, 0xa803b4); // ".qst"
            let full = exe::utils_build_file_path(0xa80204, qst.as_ptr() as usize); // "CM3_QSTART"
            copy_text(qst.as_mut_ptr(), full as *const u8);
            if exe::utils_file_exists(qst.as_ptr() as usize) == 0 && create(cfg.as_ptr() as usize, name.as_ptr() as usize) == 0 {
                format(SETUP_TEXT, 0xa80350, cfg.as_ptr());
                ok = false;
                setup_warning(0x1bdd);
                break;
            }
            if exe::file_find_next(find.as_mut_ptr()) != 0 {
                break;
            }
        }
        exe::file_find_close(fd);
    }
    ok
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
    let count_humans: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_COUNT_HUMAN_MANAGERS);
    let assign: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_SETUP_ASSIGN_RESTORED_NETWORK_MANAGERS);
    let open_status: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_OPEN_MANAGER_STATUS_SCREEN);
    let add_manager: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_ADD_HUMAN_MANAGER);
    let new_game: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_START_NEW_GAME);
    let create: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CREATE_GAME_FROM_SETUP_FILE);
    let dialog: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_OPEN_FILE_DIALOG);
    let start_network: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7ff500usize);
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
        if count_humans(0) == 0 {
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
            dialog(tt, hd, 0, G_QUICK_START_FILE_PATH, 0xa803b4, 0, 0x839290, pale, red, 4, 0, 1, 0, 1);
            -4
        }
        3 => {
            let hd = text(t, 0xa7fc84);
            let tt = text(h, 0xa80244); // "Restore Saved Game"
            dialog(tt, hd, 0, config() + CONFIG_SAVE_FOLDER, 0xa5aab8, config() + config::SAVE_NAME, 0x838e10, pale, red, 5, 0, 0, 0, 1);
            -4
        }
        4 => {
            let hd = text(t, 0xa7fc84);
            let tt = text(h, 0xa80230); // "Delete Saved Game"
            dialog(tt, hd, 2, config() + CONFIG_SAVE_FOLDER, 0xa5aab8, 0, 0x838e10, pale, red, 6, 0, 0, 0, 1);
            -4
        }
        5 => {
            if start_network(SCREEN_MANAGER, 0, 0) != 0 {
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

/// Free memory in sevenths of a megabyte, at least 1: the original's limit on the leagues loaded (`os_get_free_memory_mb`
/// / 7, signed, 0 becoming 1).
pub(crate) unsafe fn setup_league_memory_limit() -> i32 {
    let m = exe::os_get_free_memory_mb() / 7;
    if m == 0 { 1 } else { m }
}

/// The league entry (0x48 bytes in `G_LEAGUE_TABLE`) at position `k` of the key nation order `order`.
unsafe fn setup_league_entry(order: usize, k: usize) -> usize {
    G_LEAGUE_TABLE + (read_i8(order + k) as isize * 0x48) as usize
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
    let order_of: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_KEY_NATION_SORTED_ORDER);
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
    let rows = core::ptr::read_unaligned(0x81d6dd as *const i8) as usize;
    let list = area(current_human_display(), 0x6e, 0xaa, 0x30c, 0x217, 7, list_columns.as_mut_ptr() as usize, rows, 0, 2, 0, usize::MAX);
    let order = order_of(0);
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
                        let s = (staff as isize + (count - j - 1) * staff::SIZE as isize) as usize;
                        if read_u8(s + staff::CLASSIFICATION) != 0 {
                            let club = read_usize(s + staff::CLUB_JOB);
                            if club != 0 && read_usize(club + club::NATION) == n {
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

/// The memory check before one more league is loaded: unless the command line says "-dontcheckram" or the memory
/// check is off (`G_MEMORY_CHECK_ENABLED`), with as many leagues to load (`G_NATIONS_TO_LOAD_COUNT`) as the memory limit
/// (`setup_league_memory_limit`) the "Not enough memory" box opens (`exe::gui_show_message_box`) and it says no.
unsafe fn setup_room_for_one_more_league() -> bool {
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
unsafe fn setup_pick_main_nation(chosen: impl Fn(u8) -> bool) {
    write_usize(G_MAIN_NATION, 0);
    let mut e = G_LEAGUE_TABLE;
    while e < G_KEY_NATION_ACTIVE_COUNT && read_usize(G_MAIN_NATION) == 0 {
        let n = read_usize(e);
        if n != 0 && chosen(read_u8(n + nation::LEAGUE_SELECTED)) {
            write_usize(G_MAIN_NATION, n);
        }
        e += 0x48;
    }
}

/// Adds `d` to the counter at `a`.
pub(crate) unsafe fn setup_count(a: usize, d: i32) {
    write_i32(a, read_i32(a).wrapping_add(d));
}

/// Opens the box screen `build`/`input` with `flag` and, when it opens, its param 0 = 0.
pub(crate) unsafe fn setup_open_screen_with_flag(build: usize, input: usize, flag: u32) {
    if exe::screenmanager_open_screen(SCREEN_MANAGER, build, input, flag, 0usize, 0) != 0 {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, 0usize, 0);
    }
}

/// Creates the game chosen on Select League(s) or Select Start Season (`G_NEW_GAME_SETUP_DONE_FLAG` = 1,
/// `exe::create_game`(0)). A failure opens the disk full box (game in progress) or the no memory box (session started),
/// each with param 0 = 0, or is fatal (`fatal_row`). Then, unless it is a network game (`G_NETWORK_GAME_NAMED_FLAG`:
/// `setup_create_or_join_network_game`(0, `G_NETWORK_GAME_NAME`), whose nonzero answer is returned), Manager Status
/// opens with params 0..4 zeroed and a new manager is added; −4.
unsafe fn setup_create_chosen_game(fatal_row: u32) -> i32 {
    let network: unsafe extern "C" fn(u32, usize) -> i32 = core::mem::transmute(0x8388e0usize);
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
        let r = network(0, G_NETWORK_GAME_NAME);
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
    let order_of: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_KEY_NATION_SORTED_ORDER);
    let menu: unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, usize, u32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_MENU_BUTTONS);
    let mut items = [0u32; 0x23 * 0xd4 / 4];
    let base = items.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(base, 0xd4, 0x23, 0x4568f0usize, 0x5a53c0usize);
    exe::background_set_random_picture();
    setup_screen_header(0xa804ac); // "Select Start Season"
    let order = order_of(1);
    let mut count = 0usize;
    let mut bold = 0u32; // simplification: uninitialised in the original when the main nation has no item
    for k in 0..0x23 {
        let e = setup_league_entry(order, k);
        let n = read_usize(e);
        if n == 0 || read_u8(n + nation::LEAGUE_SELECTED) & 2 == 0 {
            continue;
        }
        let item = base + count * 0xd4;
        let year = read_u16(e + key_nation_record::START_DATE_YEAR) as i32;
        if read_u32(e + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
            format(item + 4, 0xa8049c, n + 0x38, year % 100, (year + 1) % 10); // "%s %02d/%02d"
        } else {
            format(item + 4, 0xa80494, n + 0x38, year); // "%s %d"
        }
        write_u32(item + 0xcc, 0xf);
        write_usize(item + 0xd0, n);
        if n == read_usize(G_MAIN_NATION) {
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
    original_fn::<unsafe extern "C" fn() -> i32>(%%I0%%)()
}

/// The regions a Setup screen's answer changes when it opens nothing: the screen manager and the current screen's
/// params (the list's scroll row), the league flags of every nation, the three league counters, the two options and the
/// Setup flags.
unsafe fn setup_answer_regions() -> Vec<(usize, usize)> {
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
unsafe fn compare_setup_answer(screen: &str, item: u32, original: impl FnOnce() -> i32, rust: impl FnOnce() -> i32) -> i32 {
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&setup_answer_regions(), original, rust);
    SCOREBOARD.record(equal && r == n, || format!("{} answer to item {}: original={} rust={} differences at {:?}", screen, item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `setup_build_setup_game_screen`: side by side over the display list (`compare_by_display_list_with`,
/// which also restores the generator for the random background), with `G_NETWORK_SETUP_HOST_FLAG`.
pub(crate) unsafe extern "C" fn compare_setup_build_setup_game_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Setup Game screen", &[(G_NETWORK_SETUP_HOST_FLAG, 4)], || o(), || setup_build_setup_game_screen());
}

/// Comparer of `setup_handle_setup_game_input`: only the original runs (every answer opens a screen, a box or a file
/// dialog, loads or deletes a file, or creates the game); the copy is proved in Rust mode (the automatic career goes
/// through the "auto_setup.cfg" branch).
pub(crate) unsafe extern "C" fn compare_setup_handle_setup_game_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}

/// Comparer of `setup_start_new_game`: only the original runs (it loads the database and opens Select League(s)).
pub(crate) unsafe extern "C" fn compare_setup_start_new_game() {
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer of `setup_build_select_leagues_screen`: side by side over the display list, with the counter of selected
/// leagues it recounts, `G_NEW_GAME_SETUP_DONE_FLAG` and the key nation order that `key_nation_sorted_order` rewrites.
pub(crate) unsafe extern "C" fn compare_setup_build_select_leagues_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    let extra = [(G_SELECTED_NATION_COUNT, 4), (G_NEW_GAME_SETUP_DONE_FLAG, 4), (G_KEY_NATION_ORDER, 0x23)];
    compare_by_display_list_with("Select League(s) screen", &extra, || o(), || setup_build_select_leagues_screen());
}

/// Comparer of `setup_handle_select_leagues_input`: the answers that only change the league flags, the counters and the
/// options (Select All, De-Select All, the four option buttons, and the league buttons when no memory box will open)
/// run side by side (`compare_setup_answer`); next (−3), the memory box and the menu bar run only in the original.
pub(crate) unsafe extern "C" fn compare_setup_handle_select_leagues_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%);
    if read_usize(G_NATION_TABLE) == 0 {
        return o(item);
    }
    let id = setup_clicked_id(item as i16);
    let pure = match id {
        0xa | 0xb | 0x39 | 0x3a | 0x3b | 0x3c => true,
        0xc | 0xd | 0xe | 0x38 => {
            let n = setup_clicked_value(item as i16);
            // simplification: the memory box (a modal box) does not run twice: original only when it would open
            n != 0 && (read_u8(n + nation::LEAGUE_SELECTED) != 0
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
    let o = original_fn::<unsafe extern "C" fn()>(%%I6%%);
    compare_by_display_list_with("Select Start Season screen", &[(G_KEY_NATION_ORDER, 0x23)], || o(), || setup_build_select_start_season_screen());
}

/// Comparer of `setup_handle_select_start_season_input`: the season button (id 0xf) runs side by side over the main
/// nation; next (it creates the game) and the menu bar run only in the original.
pub(crate) unsafe extern "C" fn compare_setup_handle_select_start_season_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I7%%);
    if setup_clicked_id(item as i16) != 0xf {
        return o(item);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(G_MAIN_NATION, 4)], || o(item), || setup_handle_select_start_season_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Select Start Season answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}
