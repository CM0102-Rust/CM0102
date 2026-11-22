// ---------- batch %%LEVA%%: the Setup screens, part 2: Network Play, the network game name and list, Manager Status (Setup.cpp) ----------

// (uses the Setup helpers of tel11_1: setup_clicked_id, setup_clicked_value, setup_colour, setup_screen_header,
// setup_fatal_exit, setup_open_screen, setup_open_screen_with_flag, setup_load_chosen_game, the FN_ and G_ names)
const FN_SCREENMANAGER_START_NETWORK_SESSION: usize = 0x7ff500; // thiscall(sm, on_join, on_leave) ret 8 -> success
const FN_SCREENMANAGER_GET_LOCAL_ADDRESSES: usize = 0x7fc7c0; // thiscall(sm, out) ret 4 -> count of 20-byte addresses
const FN_SCREENMANAGER_IS_PLAYER_DONE: usize = 0x7fff20; // thiscall(sm, player) ret 4 -> 0 or 1
const FN_SCREENMANAGER_COUNT_IDLE_CHANNELS: usize = 0x7fff70; // thiscall(sm) -> count (a short)
const FN_SCREENMANAGER_RUN_NETWORK_CLIENT: usize = 0x7fdc00; // thiscall(sm)
const FN_SCREENMANAGER_DISCONNECT_NETWORK: usize = 0x7fc690; // thiscall(sm)
const FN_TCPIP_SEND_DATAGRAM: usize = 0x8aec90; // thiscall(net, data, length, port, 0, 0, 0) ret 0x18
const FN_SETUP_CREATE_OR_JOIN_NETWORK_GAME: usize = 0x8388e0; // (join, game_name) -> 0 on success
const FN_HUMAN_MANAGER_GET_PASSWORD: usize = 0x5fde40; // thiscall(HUMAN_MANAGER, staff) ret 4 -> char *
const FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY: usize = 0x5fdf40; // thiscall(HUMAN_MANAGER, staff) ret 4 -> char *
const NETWORK_JOIN_CALLBACK: usize = 0x838a80; // the player joined hook given to start_network_session
const NETWORK_LEAVE_CALLBACK: usize = 0x838c00; // the player left hook
const NETWORK_GAME_COUNT: usize = 0xc3a6a; // the Network object's count of games found (a short) [inference]
const NETWORK_GAME_LIST: usize = 0xc3a6c; // the Network object's list of games found (10 names of 50 bytes) [inference]
const G_SCREEN_MANAGER_NETWORK: usize = 0xb77c90; // g_screen_manager_network: the Network object, null without a session
const G_NETWORK_GAME_LIST_COPY: usize = 0xdd6cd0; // g_network_game_list_copy (10 entries of 50 bytes)
const G_MANAGER_STATUS_CONFIRMED_FLAG: usize = 0xdd6ed0; // g_manager_status_confirmed_flag
const G_GUI_COLOUR_PALE_CYAN: usize = 0xaebde8; // g_gui_colour_pale_cyan
const DISPLAY_ITEM_TEXT: usize = 0xba9de; // the text of display item k (+0x18c x k; item +0x80) [inference]
const HUMAN_MANAGER_LOCKED_COUNT: usize = 0x8; // the HUMAN_MANAGER count Manager Status watches (param 2; nonzero: no Finish) [inference]
const HUMAN_MANAGER_WATCHED_COUNT: usize = 0xc; // the HUMAN_MANAGER count Manager Status watches (param 4) [inference]

/// Sends the network game list request ("CM3 LIST SERVER REQ", with its terminator, port 0x1a0b) on the network
/// object `net`, after zeroing its count of games found.
unsafe fn setup_request_network_games(net: usize) {
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
        let s = (staff as isize + (count - k) * staff::SIZE as isize) as usize;
        if read_u8(s + staff::CLASSIFICATION) == 1 && (!connected || exe::screenmanager_is_manager_done(SCREEN_MANAGER, s as u32) != 0) {
            n += 1;
        }
    }
    n
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
    let start: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_START_NETWORK_SESSION);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    write_u32(G_NETWORK_SETUP_HOST_FLAG, 0);
    write_u32(G_NETWORK_GAME_NAMED_FLAG, 0);
    if read_usize(G_SCREEN_MANAGER_NETWORK) == 0 {
        start(SCREEN_MANAGER, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK);
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
    let network: unsafe extern "C" fn(u32, usize) -> i32 = core::mem::transmute(FN_SETUP_CREATE_OR_JOIN_NETWORK_GAME);
    let assign: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_SETUP_ASSIGN_RESTORED_NETWORK_MANAGERS);
    let add_manager: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_ADD_HUMAN_MANAGER);
    let new_game: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_START_NEW_GAME);
    let dialog: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_OPEN_FILE_DIALOG);
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
                    let r = network(0, G_NETWORK_GAME_NAME);
                    if r != 0 {
                        return r;
                    }
                    setup_open_screen(0x81ff10, 0x820950, 0, 5);
                    add_manager();
                    return -4;
                }
                let restored = assign();
                let r = network(0, G_NETWORK_GAME_NAME);
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
                dialog(t, h, 0, G_QUICK_START_FILE_PATH, 0xa803b4, 0, 0x839290, pale, red, 4, 0, 1, 0, 1);
            } else {
                let config = read_usize(G_CONFIG);
                let t = text(title.as_mut_ptr() as usize, 0xa804d4); // "Restore Network Game"
                dialog(t, h, 0, config + CONFIG_SAVE_FOLDER, 0xa5aab8, config + config::SAVE_NAME, 0x838e10, pale, red, 5, 0, 0, 0, 1);
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
    let start: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_START_NETWORK_SESSION);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let screen_button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let sm = SCREEN_MANAGER;
    let mut columns: [u8; 2] = [1, 3];
    if read_usize(G_SCREEN_MANAGER_NETWORK) == 0 {
        start(sm, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK);
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
    let network: unsafe extern "C" fn(u32, usize) -> i32 = core::mem::transmute(FN_SETUP_CREATE_OR_JOIN_NETWORK_GAME);
    let run_client: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SCREENMANAGER_RUN_NETWORK_CLIENT);
    let disconnect: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SCREENMANAGER_DISCONNECT_NETWORK);
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
            let r = network(1, G_NETWORK_GAME_NAME);
            if r != 0 {
                let net = read_usize(G_SCREEN_MANAGER_NETWORK);
                if net != 0 {
                    setup_request_network_games(net);
                }
                return r;
            }
            run_client(sm);
            disconnect(sm);
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
    let password: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD);
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
            let s = (staff as isize + (count - k) * staff::SIZE as isize) as usize;
            if read_u8(s + staff::CLASSIFICATION) != 1 {
                continue;
            }
            let mut name = [0u8; 0xc8];
            exe::database_format_staff_name(s, name.as_mut_ptr() as usize, 0xc8, 1);
            format(SETUP_TEXT, 0x987ff0, name.as_ptr()); // "  %s"
            label(sm, list, 0, row, 1, 0, 1, 3, yellow, SETUP_TEXT);
            let (national, club) = (read_usize(s + staff::NATIONAL_JOB), read_usize(s + staff::CLUB_JOB));
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
                if read_u8(password(hm(), s)) != 0 {
                    may_finish = false;
                }
                if exe::screenmanager_find_manager_channel(sm, s) == 0xffff {
                    link(row, 0x18, s);
                } else if national != 0 {
                    link(row, 0x7d5, national);
                } else if club != 0 {
                    link(row, 0x7d5, club);
                }
            } else if exe::human_manager_is_computer_controlled(hm(), s) != 0 {
                let locked = read_u8(password(hm(), s)) != 0;
                text(SETUP_TEXT, if locked { 0xa805a4 } else { 0xa80598 }); // "Not Playing (locked)" / "Not Playing"
                label(sm, list, 2, row, 1, 0, 1, 3, grey, SETUP_TEXT);
                link(row, 0x18, s);
            } else {
                let playing = if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
                    player_done(sm, exe::screenmanager_find_manager_channel(sm, s) as u32) == 0
                } else {
                    exe::screenmanager_is_manager_done(sm, s as u32) == 0
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
    let password: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD);
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
        if read_u8(password(hm(), s)) != 0 {
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
                let r = (staff as isize + (count - k) * staff::SIZE as isize) as usize;
                if read_u8(r + staff::CLASSIFICATION) == 0 {
                    s = r;
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
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    compare_by_display_list_with("Network Play screen", &[], || o(), || setup_build_network_play_screen());
}

/// Comparer of `setup_handle_network_play_input`: only the original runs (every answer opens a screen or goes to the
/// menu bar).
pub(crate) unsafe extern "C" fn compare_setup_handle_network_play_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%)(item)
}

/// Comparer of `setup_build_enter_network_game_name_screen`: side by side over the display list (with the two network
/// flags) when the network object already exists; without it only the original runs (it starts the network session).
pub(crate) unsafe extern "C" fn compare_setup_build_enter_network_game_name_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    if read_usize(G_SCREEN_MANAGER_NETWORK) == 0 {
        return o();
    }
    compare_by_display_list_with("Enter Network Game Name screen", &[(G_NETWORK_SETUP_HOST_FLAG, 8)], || o(), || setup_build_enter_network_game_name_screen());
}

/// Comparer of `setup_handle_enter_network_game_name_input`: only the original runs (the answers load files, create the
/// network game, open screens and dialogs or end the session).
pub(crate) unsafe extern "C" fn compare_setup_handle_enter_network_game_name_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I3%%)(item)
}

/// Comparer of `setup_build_select_network_game_screen`: side by side over the display list and the copy of the game
/// list when the network object exists; without it only the original runs (it starts the session and sends the request).
pub(crate) unsafe extern "C" fn compare_setup_build_select_network_game_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    if read_usize(G_SCREEN_MANAGER_NETWORK) == 0 {
        return o();
    }
    compare_by_display_list_with("Select Network Game screen", &[(G_NETWORK_GAME_LIST_COPY, 500)], || o(), || setup_build_select_network_game_screen());
}

/// Comparer of `setup_handle_select_network_game_input`: the name entry and a game of the list (ids 0x16 and 0x17, with
/// an item) run side by side over the game name; Refresh, next, back and the menu bar only in the original (network).
pub(crate) unsafe extern "C" fn compare_setup_handle_select_network_game_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%);
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
    original_fn::<unsafe extern "C" fn()>(%%I6%%)()
}

/// Comparer of `setup_build_manager_status_screen`: side by side over the display list and the screen's params; without
/// staff only the original runs (the error box does not run twice).
pub(crate) unsafe extern "C" fn compare_setup_build_manager_status_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    if read_usize(G_STAFF_TABLE) == 0 {
        return o();
    }
    compare_by_display_list_with("Manager Status screen", &[], || o(), || setup_build_manager_status_screen());
}

/// Comparer of `setup_handle_manager_status_input`: back and Finish (ids -2 and 0x19, which also answer -11 when the
/// screen is out of date) run side by side over the Finish flag; a manager's row and the menu bar only in the original
/// (they open screens); without staff only the original (error box).
pub(crate) unsafe extern "C" fn compare_setup_handle_manager_status_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I8%%);
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
    original_fn::<unsafe extern "C" fn()>(%%I9%%)()
}
