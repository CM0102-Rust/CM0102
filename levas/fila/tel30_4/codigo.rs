// ---------- batch %%LEVA%%: the board request screen ("Meet with Board") (belgium_rules.cpp) ----------

/// The screen's build and input functions (passed to `ScreenManager::open_screen`).
pub(crate) const FN_BOARD_REQUEST_SCREEN_DRAW: usize = 0x4253f0;
pub(crate) const FN_BOARD_REQUEST_SCREEN_HANDLE_EVENT: usize = 0x4257f0;
/// The texts "Meet with Board", "  Specify the details of requests from board:", "Cancel" and "Confirm".
const BOARD_REQUEST_TITLE: usize = 0x98a1d8;
const BOARD_REQUEST_INTRO: usize = 0x98a1a8;
const BOARD_REQUEST_CANCEL: usize = 0x98a1a0;
const BOARD_REQUEST_CONFIRM: usize = 0x98a198;
/// u32: the game is a network game.
const BOARD_REQUEST_NETWORK_GAME: usize = 0xb77c84;
/// The item ids of the screen: a request (its value is the request), Cancel and Confirm.
const BOARD_REQUEST_ITEM_REQUEST: u16 = 0x64;
const BOARD_REQUEST_ITEM_CANCEL: u16 = 8;
const BOARD_REQUEST_ITEM_CONFIRM: u16 = 9;

/// Opens the board request screen of a club (`club` points to the club's id): `ScreenManager::open_screen` with
/// `FN_BOARD_REQUEST_SCREEN_DRAW` and `FN_BOARD_REQUEST_SCREEN_HANDLE_EVENT`, single instance, the empty 0x5a8f60 as
/// the leave function; when it opens, parameter 0 = the club id, 1 (the chosen request) and 2 (its button) = 0.
/// Original: `board_request_screen_open` (0x425390, belgium_rules.cpp, cdecl).
pub(crate) unsafe extern "C" fn board_request_screen_open(club: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, FN_BOARD_REQUEST_SCREEN_DRAW, FN_BOARD_REQUEST_SCREEN_HANDLE_EVENT, 1, FN_SHARED_NOOP_RET0_5A8F60, 0) == 0 {
        return;
    }
    covered!("tel30_4 board request screen opened");
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, read_usize(club), 0);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, 0usize, 0);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 2, 0usize, 0);
}

/// The club of the screen: parameter 0 is its id in the club table.
unsafe fn board_request_club() -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(get_screen_param(0).wrapping_mul(club::SIZE))
}

/// Builds the board request screen of the club in parameter 0 (nothing without a chairman at +0xcf).
///
/// Parameter 1 is the chosen request; parameter 2 is read too (unused). The boardroom picture
/// (`background_set_section_picture(7, 0)`); the title is the chairman's name (`database_format_staff_name`, style 1)
/// in the club's kit colours (kit 1, background then foreground); the subtitle "Meet with Board". An area
/// (0x6e, 0x8c)-(0x30c, 0x221) with the 7 column bytes [2, 0x14, 2, 0xa, 2, 4, 3] and 0x195 / font_get_height(3)
/// rows holds the wrapped introduction ("  Specify the details of requests from board:", font 3, pale grey) from row 0;
/// a second area over the same rectangle (2 columns [2, 0x11]) takes, from two rows below the text, one button per
/// request 1 to 5 (`get_board_request_option_text` on the manager manager, 2000 bytes): an unavailable one
/// (`is_board_request_available`) grey with no id, an available one pale grey (yellow when chosen) with id 0x64 and
/// the request as value; the first button's index goes to parameter 2. Then the menu bar (`menubar_build(4, 0)`) and
/// an area (0x64, 0x22b)-(0x316, 0x24e) with the columns [3, 1], one row and the club's kit background: "Cancel"
/// (id 8) and "Confirm" (id 9, a plain text with no request chosen). The colour words are passed zero-extended (the
/// original pushes registers with only their low half loaded).
/// Original: `board_request_screen_draw` (0x4253f0, belgium_rules.cpp, cdecl).
pub(crate) unsafe extern "C" fn board_request_screen_draw() {
    let mut text = [0usize; 2];
    exe::new_text(text.as_mut_ptr(), 0usize);
    let mut row: i32 = 0;
    let club = board_request_club();
    let chairman = read_usize(club + 0xcf);
    if chairman != 0 {
        covered!("tel30_4 board request screen built");
        let chosen = get_screen_param(1) as u8;
        get_screen_param(2);
        background_set_section_picture(7, 0);
        let (mut foreground, mut background) = (0u32, 0u32);
        exe::database_get_club_kit_colours(club, 1, &mut foreground, &mut background);
        let mut name = [0u8; 500];
        exe::database_format_staff_name(chairman, name.as_mut_ptr() as usize, 0x1f4, 1);
        exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, background & 0xffff, foreground & 0xffff, name.as_ptr() as usize);
        let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
        langlib(JOB_TEXT, BOARD_REQUEST_TITLE);
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        let rows = (0x195 / exe::font_get_height(3)) as u8 as u32;
        let columns: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
        let buttons: [u8; 2] = [2, 0x11];
        let footer: [u8; 2] = [3, 1];
        let intro = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 7, columns.as_ptr() as u32, rows, 0, 2, 0, 0xffff_ffff]);
        langlib(JOB_TEXT, BOARD_REQUEST_INTRO);
        exe::text_set_string(text.as_mut_ptr() as usize, JOB_TEXT as *const u8);
        let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x4aeaf0usize);
        draw(text.as_mut_ptr() as usize, intro, 0x6e, 0x30c, &mut row, 3, read_u16(G_GUI_COLOUR_PALE_GREY) as u32, 0, 0);
        let list = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, buttons.as_ptr() as u32, rows, 0, 1, 0, 0xffff_ffff]);
        row += 2;
        let mm = read_usize(G_MANAGER_MANAGER);
        let option_text: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x6a8770usize);
        let available: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x6ae180usize);
        let mut request: u8 = 1;
        loop {
            option_text(mm, request as u32, JOB_TEXT, 0x7d0);
            if request == 6 {
                break;
            }
            let (colour, id) = if available(mm, club, request as u32) == 0 {
                (read_u16(G_GUI_COLOUR_GREY) as u32, 0)
            } else if chosen == request {
                (read_u16(G_GUI_COLOUR_YELLOW) as u32, BOARD_REQUEST_ITEM_REQUEST as u32)
            } else {
                (read_u16(G_GUI_COLOUR_PALE_GREY) as u32, BOARD_REQUEST_ITEM_REQUEST as u32)
            };
            let button = tel26_area_button([list, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, id, request as i8 as i32 as u32]);
            if request == 1 {
                set_screen_param(2, button as i16 as i32 as usize);
            }
            row += 1;
            request += 1;
            if request > 6 {
                break;
            }
        }
        exe::menubar_build(4, 0);
        let bar = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, footer.as_ptr() as u32, 1, 0, 1, background, 0xffff_ffff]);
        langlib(JOB_TEXT, BOARD_REQUEST_CANCEL);
        let (grey, pale) = (read_u16(G_GUI_COLOUR_GREY) as u32, read_u16(G_GUI_COLOUR_PALE_GREY) as u32);
        tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, BOARD_REQUEST_ITEM_CANCEL as u32, 0]);
        langlib(JOB_TEXT, BOARD_REQUEST_CONFIRM);
        if chosen == 0 {
            tel26_area_text([bar, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
        } else {
            tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, BOARD_REQUEST_ITEM_CONFIRM as u32, 0]);
        }
    }
    exe::drop_text(text.as_mut_ptr());
}

/// The id of an item of the board request screen (`display_item_id`; none, -1, gives the current command id).
unsafe fn board_request_item_id(item: i16) -> u16 {
    tel26_item_id(item)
}

/// Answers a click on the board request screen.
///
/// Parameters 0 (the club), 1 (the chosen request) and 2 are read. In a network game the pending command argument
/// is cleared and -9 returned. A request button (id 0x64) puts its value (the request) in parameter 1 and returns -11
/// (redraw); Confirm (id 9) with a request chosen passes it to `MANAGER_MANAGER::handle_board_request(club,
/// request)`, and Confirm or Cancel (id 8) return -9 (back); any other item goes to the menu bar
/// (`menubar_handle_command`, then `menubar_handle_object_link` when it returns 0).
/// Original: `board_request_screen_handle_event` (0x4257f0, belgium_rules.cpp, cdecl).
pub(crate) unsafe extern "C" fn board_request_screen_handle_event(item: u32) -> i32 {
    let club = board_request_club();
    let chosen = get_screen_param(1);
    get_screen_param(2);
    if read_u32(BOARD_REQUEST_NETWORK_GAME) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let raw = item;
    let item = item as i16;
    let id = board_request_item_id(item);
    if id == BOARD_REQUEST_ITEM_REQUEST {
        set_screen_param(1, tel26_item_value(item) as usize);
        return -11;
    }
    if id == BOARD_REQUEST_ITEM_CONFIRM {
        if chosen != 0 {
            covered!("tel30_4 board request confirmed");
            let handle: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a8810usize);
            handle(read_usize(G_MANAGER_MANAGER), club, chosen as u32);
        }
        return -9;
    }
    if id == BOARD_REQUEST_ITEM_CANCEL {
        return -9;
    }
    let r = exe::menubar_handle_command(raw);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(raw)
}

// simplification: opens a screen (the history allocates a node): it does not run twice. Original only; the plan: the
// new current screen is this one, with the club in parameter 0 and no request chosen.
/// Comparer for `board_request_screen_open`.
pub(crate) unsafe extern "C" fn compare_board_request_screen_open(club: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(club);
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node == 0 || read_usize(node) != FN_BOARD_REQUEST_SCREEN_DRAW {
        return;
    }
    let (p0, p1, p2) = (get_screen_param(0), get_screen_param(1), get_screen_param(2));
    SCOREBOARD.record(p0 == read_usize(club) && p1 == 0 && p2 == 0, || format!("board_request_screen_open (Meet with Board): params {:#x} {} {}", p0, p1, p2));
}

/// Comparer for `board_request_screen_draw`: through the display list (`compare_by_display_list_with`), plus the
/// screen's picture path (the boardroom picture rolls the generator, which the list comparer restores).
pub(crate) unsafe extern "C" fn compare_board_request_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let mut extra = Vec::new();
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node != 0 {
        extra.push((node + screen_node_fields::PICTURE, screen_node_fields::PICTURE_SIZE));
    }
    compare_by_display_list_with("board_request_screen_draw (Meet with Board)", &extra, || o(), || board_request_screen_draw());
}

/// Comparer for `board_request_screen_handle_event`: a request button and Cancel side by side over the parameters
/// and the answer (`compare_resposta`); Confirm (the board's answer), the menu bar and a network game the original
/// only.
pub(crate) unsafe extern "C" fn compare_board_request_screen_handle_event(item: u32) -> i32 {
    let id = board_request_item_id(item as i16);
    let stays = read_u32(BOARD_REQUEST_NETWORK_GAME) == 0 && (id == BOARD_REQUEST_ITEM_REQUEST || id == BOARD_REQUEST_ITEM_CANCEL);
    compare_resposta("board_request_screen_handle_event (Meet with Board)", item, original_fn(%%I2%%), board_request_screen_handle_event, !stays)
}
