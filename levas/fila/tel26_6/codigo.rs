// ---------- batch %%LEVA%%: the menu bar (Menubar.cpp) ----------

/// The menu bar records: one 0xf6-byte record per human manager (16), by the manager's position among the last 16
/// staff records (`[manager] − G_STAFF_COUNT + 16`).
const MENUBAR_RECORDS: usize = 0xb73390;
const MENUBAR_RECORD_SIZE: usize = 0xf6;
/// The text buffer every menu text is written into.
const MENUBAR_TEXT: usize = 0xde1f64;
/// The empty string.
const MENUBAR_EMPTY: usize = 0xad9c64;
/// The menu entries' background colour (u16).
const MENUBAR_MENU_COLOUR: usize = 0xae31aa;
/// The highlighted heading colour (u16).
const MENUBAR_HEADING_COLOUR: usize = 0xaebde8;
/// time() of the last menu bar build.
const MENUBAR_BUILD_TIME: usize = 0xb74310;
/// The menu handler and its two command ids, written on every build [inference].
const MENUBAR_HANDLER: usize = 0xb77c3e;
/// Nonzero in a network game (screen manager + 0x3840) [inference].
const MENUBAR_NETWORK_GAME: usize = 0xb77c90;
/// One dword per human manager: a chat message is waiting [inference].
const MENUBAR_CHAT_PENDING: usize = 0xb6b7d0;
/// Per-slot fields of the screen manager (+0x300 x slot): the slot's manager (Staff), a u16 that is nonzero while
/// the slot cannot resign, retire or go on holiday [inference], and the time() the "Continue Game" countdown started
/// (−1: none).
const MENUBAR_SLOT_MANAGER: usize = 0x7d6;
const MENUBAR_SLOT_LOCKED: usize = 0x7e6;
const MENUBAR_SLOT_CONTINUE_TIME: usize = 0x800;

/// The fields of a menu bar record.
#[allow(dead_code)]
mod menubar_record {
    pub(crate) const MODE: usize = 0; // 1 byte: the menu built last (1 game, 2 team, 3 match)
    pub(crate) const CONTEXT: usize = 1; // 4 bytes: the second argument of the last build
    pub(crate) const UPDATING: usize = 5; // 1 byte: 1 while the match day is being processed
    pub(crate) const REFRESH_TIME: usize = 6; // 4 bytes: utils_get_time_ms + 1000 of the first build
    pub(crate) const HEADER_WIDGET: usize = 0xa; // 2 bytes: the header text widget
    pub(crate) const HEADER_TEXT: usize = 0xc; // the header text (date or match)
    pub(crate) const CONTINUE_WIDGET: usize = 0xd4; // 2 bytes: the "Continue Game" item
    pub(crate) const CONTINUE_STATE: usize = 0xd6; // 1 byte: 1 continue, 2 disabled, 3 latest scores
    pub(crate) const STATUS_WIDGET: usize = 0xd7; // 2 bytes: the status item at the bottom
    pub(crate) const STATUS_KIND: usize = 0xd9; // 1 byte: 0 none, 1 news, 2 chat, 3 waiting players
    pub(crate) const WAITING: usize = 0xda; // 4 bytes: players waiting
    pub(crate) const TOTAL: usize = 0xde; // 4 bytes: players in the game
    pub(crate) const PAUSED_MATCH: usize = 0xe2; // 4 bytes: the match of the match menu
    pub(crate) const HOLIDAY_PENDING: usize = 0xee; // 4 bytes: the countdown started (zeroed on every build)
    pub(crate) const COUNTDOWN: usize = 0xf2; // 4 bytes: seconds since the countdown started
}

type MenubarSprintf = unsafe extern "C" fn(usize, usize, ...) -> i32;
type MenubarThis0 = unsafe extern "thiscall" fn(usize) -> u32;
type MenubarThis1 = unsafe extern "thiscall" fn(usize, usize) -> u32;
type MenubarThis2 = unsafe extern "thiscall" fn(usize, usize, usize) -> u32;
type MenubarThis3 = unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32;
type MenubarThis4 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) -> u32;
type MenubarCdecl0 = unsafe extern "C" fn() -> usize;
type MenubarCdecl1 = unsafe extern "C" fn(usize) -> usize;
type MenubarCdecl3 = unsafe extern "C" fn(usize, usize, usize) -> usize;

/// The address of a per-slot field of the screen manager for the current slot.
unsafe fn menubar_slot(offset: usize) -> usize {
    (SCREEN_MANAGER as isize + offset as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize
}

/// `langlib_sprintf` into the text buffer, with no arguments.
unsafe fn menubar_text(format: usize) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(MENUBAR_TEXT, format);
}

/// `Display::add_item` on the current human's Display.
unsafe fn menubar_item(a: [usize; 18]) -> usize {
    let f: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    f(current_human_display(), a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15], a[16], a[17])
}

/// `Display::add_area` on the current human's Display.
unsafe fn menubar_area(a: [usize; 11]) -> usize {
    let f: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    f(current_human_display(), a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10])
}

/// `ScreenManager::add_area_text` of the text buffer in column 0; returns the widget (AX).
unsafe fn menubar_area_text(area: usize, row: i32, style: usize, flags: usize, font: usize, colour: usize) -> u16 {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(SCREEN_MANAGER, area, 0, row as usize, style, 0, flags, font, colour, MENUBAR_TEXT) as u16
}

/// `ScreenManager::add_area_button` with data 0.
#[allow(clippy::too_many_arguments)]
unsafe fn menubar_area_button(area: usize, column: usize, row: i32, flags: usize, font: usize, colour: usize, text: usize, id: usize) {
    let f: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    f(SCREEN_MANAGER, area, column, row as usize, 0x1021, 0, flags, font, colour, text, id, 0);
}

/// A menu heading or a top-level item of the bar (`kind` 0x82 opens a menu, 2 an item, 1 an inactive item).
#[allow(clippy::too_many_arguments)]
unsafe fn menubar_heading(kind: usize, row: i32, style: usize, flags: usize, font: usize, colour: usize, text: usize, id: usize, area: usize) -> usize {
    menubar_item([kind, 0, 0, 0, 0, 0, row as usize, style, 0, 0, flags, font, colour, text, 0, id, 0, area])
}

/// The menu area under a heading item.
unsafe fn menubar_submenu(item: usize) -> usize {
    menubar_area([0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(MENUBAR_MENU_COLOUR), item])
}

/// An entry of a menu: `flags` 1 enabled, 0x21 greyed out.
unsafe fn menubar_entry(text: usize, flags: usize, id: usize, data: usize, area: usize) -> usize {
    menubar_item([2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(MENUBAR_MENU_COLOUR), colour_word(G_GUI_COLOUR_YELLOW), flags, 1,
        colour_word(G_GUI_COLOUR_BLACK), text, 0, id, data, area])
}

/// The entry of the text buffer: enabled with `id`, or greyed out with no id.
unsafe fn menubar_entry_if(enabled: bool, id: usize, area: usize) {
    if enabled {
        menubar_entry(MENUBAR_TEXT, 1, id, 0, area);
    } else {
        menubar_entry(MENUBAR_TEXT, 0x21, 0, 0, area);
    }
}

/// A separator line in a menu.
unsafe fn menubar_separator(area: usize) {
    menubar_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, colour_word(MENUBAR_MENU_COLOUR), colour_word(G_GUI_COLOUR_YELLOW), 1, 1,
        colour_word(G_GUI_COLOUR_BLACK), 0, 0, 0, 0, area]);
}

/// The `is_current_manager_being_created` (0x838de0) test.
unsafe fn menubar_being_created() -> bool {
    core::mem::transmute::<usize, MenubarCdecl0>(0x838de0)() != 0
}

/// `count_human_managers` (0x838d10).
unsafe fn menubar_count_humans(connected_only: usize) -> i32 {
    core::mem::transmute::<usize, MenubarCdecl1>(0x838d10)(connected_only) as i32
}

/// `ScreenManager::count_flagged_screens` (0x803d50) is zero (AX).
unsafe fn menubar_no_flagged_screens() -> bool {
    core::mem::transmute::<usize, MenubarThis0>(0x803d50)(SCREEN_MANAGER) as u16 == 0
}

/// "Add Manager" can be chosen: a free human manager row (0x838c60), no manager being created and the human
/// manager object's +8 clear.
unsafe fn menubar_can_add_manager() -> bool {
    core::mem::transmute::<usize, MenubarCdecl0>(0x838c60)() != 0 && !menubar_being_created() && read_u32(read_usize(G_HUMAN_MANAGER) + 8) == 0
}

/// "Chat" is offered: a network game and a manager in the slot with a first or second name [inference: in memory the
/// name fields hold pointers to the strings].
unsafe fn menubar_can_chat() -> bool {
    if read_u32(MENUBAR_NETWORK_GAME) == 0 {
        return false;
    }
    let s = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
    s != 0 && (read_u8(read_usize(s + staff::FIRST_NAME)) != 0 || read_u8(read_usize(s + staff::SECOND_NAME)) != 0)
}

/// The "<<<" and ">>>" (or the symbol font's 0x28 and 0x29, `G_FONT_MODE` ≠ −1) buttons in their own area, ids −2 and −3.
unsafe fn menubar_arrows(row: i32) {
    let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
    let arrows = menubar_area([5, 10, 0x55, 0x24e, 2, 0, 0xd, 0, 1, 0, usize::MAX]);
    for (column, symbol, text, id) in [(0usize, 0x28usize, 0xa16444usize, -2isize), (1, 0x29, 0xa16440, -3)] {
        if read_i32(G_FONT_MODE) != -1 {
            format(MENUBAR_TEXT, 0x99ec1c, symbol);
            menubar_area_button(arrows, column, row, 0xc, 0, colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, id as usize);
        } else {
            menubar_area_button(arrows, column, row, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), text, id as usize);
        }
    }
}

/// Builds the game menu bar for the current human manager: the background, then the menu of `mode` (1 the game
/// menu, 2 the team menu, 3 the match menu) and the status item, and keeps the record of what was built.
///
/// It stores time() in `MENUBAR_BUILD_TIME` and the menu handler (0x823930, 0x423, 0x424). With the interactive flag
/// (`G_INTERACTIVE_MODE_FLAG`) clear, no manager in the slot or a manager outside the last 16 staff records, there is no
/// record and the game menu is built. Otherwise `menubar_record::HOLIDAY_PENDING` is zeroed and mode 4 (redraw) becomes
/// the record's mode, or 2 when the record has none or the manager can go neither back nor forward
/// (`exe::screenmanager_is_back_unavailable`(1) and `exe::screenmanager_is_forward_unavailable`).
///
/// The background area (0, 0, 0x59, 599: the height is read from the exe, 1079 in the 1080 copy) with the picture
/// "game.mbr" of CM3_DATA, and the bar's area (5, 10, 0x55, 0x24e).
///
/// Game menu (`menubar_build_game_menu`): the version, the arrows, "Add Manager", "Chat", "Restart Game" and "Exit
/// Game". Team menu (`menubar_build_team_menu`): the date (or the match, or the match day percentage), the arrows,
/// "Continue Game", the manager's menu, Competitions, Nations & Clubs, Find, the change player menu (0x7612c0) and
/// Game Options. Match menu (`menubar_build_match_menu`, the record's paused match): the minute, the arrows, "Paused",
/// both teams' tactics and the commentary speed. Then, with a record, the status item (`menubar_build_status`).
///
/// Finally, with a manager in the slot on holiday (`exe::human_manager_is_on_holiday`), the holiday stamp is set to
/// `exe::utils_get_time_ms`; and a record with no refresh time gets `exe::utils_get_time_ms` + 1000.
/// Original: menubar_build (0x75db00, Menubar.cpp, cdecl)
pub(crate) unsafe extern "C" fn menubar_build(mode: u32, context: i32) {
    let mut mode = mode as u8;
    let staff = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
    exe::at_946ebb(MENUBAR_BUILD_TIME as *mut i32);
    write_u32(MENUBAR_HANDLER, 0x823930);
    write_u32(MENUBAR_HANDLER + 4, 0x423);
    write_u32(MENUBAR_HANDLER + 8, 0x424);
    let mut rec = 0usize;
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        covered!("tel26_6 menu bar outside the interactive mode");
        mode = 1;
    } else {
        let s = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
        let k = if s == 0 { -1 } else { read_i32(s).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) };
        if (0..0x10).contains(&k) {
            rec = MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE;
        }
        if rec == 0 {
            covered!("tel26_6 menu bar without a record");
            mode = 1;
        } else {
            write_u32(rec + menubar_record::HOLIDAY_PENDING, 0);
            if mode == 4 {
                covered!("tel26_6 menu bar redraw");
                mode = if read_u8(rec + menubar_record::MODE) == 0
                    || (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 1) != 0 && exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) != 0)
                {
                    2
                } else {
                    read_u8(rec + menubar_record::MODE)
                };
            }
        }
    }
    // 1080: the background's height is the one in the exe (0x257 in the original, 0x437 in the 1080 copy)
    let height = core::ptr::read_unaligned(0x75dc11 as *const i32);
    let background = menubar_area([0, 0, 0x59, height as usize, 1, 0, 1, 0, 1, 0, usize::MAX]);
    let picture = exe::utils_build_file_path(0x9b5348usize, 0xa5a7ccusize);
    menubar_item([0x400, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0xc, 1, 0, picture, 0, 0, 0, background]);
    let area = menubar_area([5, 10, 0x55, 0x24e, 1, 0, 0xd, 0, 1, 0, usize::MAX]);
    match mode {
        1 => menubar_build_game_menu(area, rec, context),
        2 => menubar_build_team_menu(area, rec, staff, context),
        3 => {
            let m = read_usize(rec + menubar_record::PAUSED_MATCH);
            if m != 0 {
                menubar_build_match_menu(area, rec, m, context);
            }
        }
        _ => covered!("tel26_6 menu bar with no menu"),
    }
    if rec != 0 {
        menubar_build_status(rec, mode);
    }
    if read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)) != 0 && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), 0usize) != 0 {
        covered!("tel26_6 menu bar holiday stamp");
        let now = exe::utils_get_time_ms();
        exe::human_manager_set_holiday_stamp(read_usize(G_HUMAN_MANAGER), now, 0usize);
    }
    if rec != 0 && read_u32(rec + menubar_record::REFRESH_TIME) == 0 {
        write_u32(rec + menubar_record::REFRESH_TIME, exe::utils_get_time_ms().wrapping_add(1000));
    }
}

/// The game menu (mode 1): the version ("Version <%s>"), the arrows, "Add Manager" (a button 0x3fb with game data
/// loaded, `G_GAME_DATA_LOADED_FLAG`, and `menubar_can_add_manager`, else inactive text), "Chat" (0x421, with
/// `menubar_can_chat`), "Restart Game" (0x42f, outside a network slot, `screen_manager::NETWORK_SLOT`, and with no flagged
/// screen) and "Exit Game" (0x402, with no flagged screen); then, with a record, mode 1 and the context in it.
unsafe fn menubar_build_game_menu(area: usize, rec: usize, context: i32) {
    covered!("tel26_6 game menu");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut row = 0i32;
    text(MENUBAR_TEXT, 0xa5a7b4, exe::utils_get_version_string());
    menubar_area_text(area, row, 0x1061, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW));
    row += 1;
    menubar_arrows(row);
    row += 1;
    menubar_text(0xa5a7a8);
    if read_u32(G_GAME_DATA_LOADED_FLAG) != 0 && menubar_can_add_manager() {
        menubar_area_button(area, 0, row, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0x3fb);
    } else {
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY));
    }
    if menubar_can_chat() {
        row += 1;
        menubar_text(0xa5a7a0);
        menubar_area_button(area, 0, row, 0xc, 1, colour_word(MENUBAR_HEADING_COLOUR), MENUBAR_TEXT, 0x421);
    }
    row += 1;
    menubar_text(0xa5a790);
    if read_u16(menubar_slot(screen_manager::NETWORK_SLOT)) == 0 && menubar_no_flagged_screens() {
        menubar_area_button(area, 0, row, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, 0x42f);
    } else {
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_YELLOW));
    }
    row += 1;
    menubar_text(0xa5a784);
    if menubar_no_flagged_screens() {
        menubar_area_button(area, 0, row, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, 0x402);
    } else {
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_YELLOW));
    }
    if rec != 0 {
        write_u8(rec + menubar_record::MODE, 1);
        write_i32(rec + menubar_record::CONTEXT, context);
    }
}

/// The team menu (mode 2).
///
/// The header (kept in `menubar_record::HEADER_WIDGET` and copied to `menubar_record::HEADER_TEXT`): while the record is
/// updating and the match day (`G_MATCH_DAY`) is below 100 % (done +0x25 × 100 / total +0x21, a signed byte) with
/// `G_MATCH_DAY_PLAYING_FLAG` set, "Updating <%d>%"; otherwise, updating or with the config flag 0x10000 and a paused
/// match, "home v away"; otherwise the date (`Date::get_weekday_numeric_date_string` of `G_CURRENT_DATE` with
/// `G_CURRENT_DAY_SLICE`), in the font read from the exe (1, or 2 in the 1080 copy). Then the arrows.
///
/// The continue item (`menubar_record::CONTINUE_WIDGET` and `CONTINUE_STATE`): with a match day whose +0x34 stage is
/// neither 2 nor 8 and no paused match, "Latest Scores" (0x418, state 3); with a manager being created, a current
/// screen flagged without an exit function or a context, "Continue Game" inactive (state 2); a manager done
/// (`exe::screenmanager_is_manager_done`): "Waiting For Players" (0x3e8, state 1); otherwise "Continue Game" (0x3e8,
/// state 1), where a state-3 record restarts the countdown time and, in a network game with a countdown, no paused match
/// and the manager not on holiday, the seconds since it (`menubar_record::COUNTDOWN`) below 5 show the seconds left in
/// red, font 4; `menubar_record::HOLIDAY_PENDING` = 1 and `ScreenManager::send_clear_click`. Every state but the last
/// one clears the countdown time (−1).
///
/// Then the manager's menu (`menubar_manager_menu`), Competitions (`menubar_competitions_menu`), Nations & Clubs and
/// Find (`menubar_nations_and_find_menus`), the change player menu (0x7612c0, the others disabled with no paused
/// match), Game Options (`menubar_game_options_menu`) and mode 2 with the context in the record.
unsafe fn menubar_build_team_menu(area: usize, rec: usize, staff: usize, context: i32) {
    covered!("tel26_6 team menu");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
    let m = read_usize(rec + menubar_record::PAUSED_MATCH);
    let mut row = 0i32;
    let updating = read_u8(rec + menubar_record::UPDATING);
    let md = read_usize(G_MATCH_DAY);
    let percentage = |md: usize| {
        let total = read_i32(md + 0x21);
        if total == 0 { 0 } else { read_i32(md + 0x25).wrapping_mul(100).wrapping_div(total) as i8 }
    };
    let font;
    if updating == 1 && md != 0 && (read_i32(md + 0x21) == 0 || percentage(md) < 100) && read_u32(G_MATCH_DAY_PLAYING_FLAG) != 0 {
        covered!("tel26_6 team menu updating");
        text(MENUBAR_TEXT, 0xa5a764, percentage(md) as i32);
        font = 1;
    } else {
        if (updating == 1 || read_u32(read_usize(G_CONFIG) + 4) & 0x10000 != 0) && m != 0 {
            covered!("tel26_6 team menu match header");
            let away = read_usize(m + 0x24);
            let home = read_usize(m + 0x20);
            let a = read_i8(away + club::SHORT_GENDER_NAME) as i32;
            let h = read_i8(home + club::SHORT_GENDER_NAME) as i32;
            text(MENUBAR_TEXT, 0xa5a738, h, h, a, a, home + club::SHORT_NAME, away + club::SHORT_NAME);
        } else {
            let date: MenubarThis3 = core::mem::transmute(0x54a950usize);
            date(G_CURRENT_DATE, MENUBAR_TEXT, 0x7d0, read_u8(G_CURRENT_DAY_SLICE) as usize);
        }
        // 1080: the header font is the one in the exe (1 in the original, 2 in the 1080 copy)
        font = core::ptr::read_unaligned(0x75e16c as *const u8) as i8 as isize as usize;
    }
    let w = menubar_area_text(area, row, 0x1061, 0xc, font, colour_word(G_GUI_COLOUR_YELLOW));
    write_u16(rec + menubar_record::HEADER_WIDGET, w);
    let mut k = 0;
    loop {
        let c = read_u8(MENUBAR_TEXT + k);
        write_u8(rec + menubar_record::HEADER_TEXT + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
    row += 1;
    menubar_arrows(row);
    row += 1;
    let md = read_usize(G_MATCH_DAY);
    let latest = md != 0 && {
        let stage = read_usize(md + 0x34);
        stage != 0 && read_u8(stage + 0x67) != 2 && read_u8(stage + 0x67) != 8
    } && m == 0;
    if latest {
        covered!("tel26_6 team menu latest scores");
        menubar_text(0xa5a728);
        let w = menubar_heading(2, row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0x418, area);
        write_u16(rec + menubar_record::CONTINUE_WIDGET, w as u16);
        write_u8(rec + menubar_record::CONTINUE_STATE, 3);
        write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
    } else if menubar_being_created()
        || (core::mem::transmute::<usize, MenubarThis0>(0x7fed30)(SCREEN_MANAGER) != 0 && core::mem::transmute::<usize, MenubarThis0>(0x7fedd0)(SCREEN_MANAGER) == 0)
        || context != 0
    {
        covered!("tel26_6 team menu continue disabled");
        menubar_text(0xa16d24);
        let w = menubar_heading(1, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
        write_u16(rec + menubar_record::CONTINUE_WIDGET, w as u16);
        write_u8(rec + menubar_record::CONTINUE_STATE, 2);
        write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
    } else if exe::screenmanager_is_manager_done(SCREEN_MANAGER, read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)) as u32) != 0 {
        covered!("tel26_6 team menu waiting for players");
        menubar_text(0xa16d34);
        let w = menubar_heading(2, row, 0x1061, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0x3e8, area);
        write_u16(rec + menubar_record::CONTINUE_WIDGET, w as u16);
        write_u8(rec + menubar_record::CONTINUE_STATE, 1);
        write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
    } else {
        covered!("tel26_6 team menu continue");
        let mut font = 1usize;
        let mut colour = colour_word(G_GUI_COLOUR_PALE_GREY);
        menubar_text(0xa16d24);
        if read_u8(rec + menubar_record::CONTINUE_STATE) == 3 {
            exe::at_946ebb(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME) as *mut i32);
        }
        if read_u32(MENUBAR_NETWORK_GAME) != 0
            && read_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME)) != -1
            && m == 0
            && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), 0usize) == 0
        {
            covered!("tel26_6 team menu continue countdown");
            let mut now = 0i32;
            exe::at_946ebb(&mut now);
            let d = now.wrapping_sub(read_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME)));
            write_i32(rec + menubar_record::COUNTDOWN, d);
            if d < 5 {
                format(MENUBAR_TEXT, 0x98806c, 5i32.wrapping_sub(d));
                font = 4;
                colour = colour_word(G_GUI_COLOUR_RED);
            }
            write_u32(rec + menubar_record::HOLIDAY_PENDING, 1);
            core::mem::transmute::<usize, MenubarThis0>(0x800000)(SCREEN_MANAGER);
        }
        let w = menubar_heading(2, row, 0x1021, 0xc, font, colour, MENUBAR_TEXT, 0x3e8, area);
        write_u16(rec + menubar_record::CONTINUE_WIDGET, w as u16);
        write_u8(rec + menubar_record::CONTINUE_STATE, 1);
    }
    row += 1;
    menubar_manager_menu(area, row, staff, rec, m);
    row += 1;
    menubar_competitions_menu(area, row);
    menubar_nations_and_find_menus(area, &mut row);
    let change_player: MenubarCdecl3 = core::mem::transmute(0x7612c0usize);
    change_player(area, &mut row as *mut i32 as usize, (m == 0) as usize);
    row += 1;
    menubar_game_options_menu(area, row);
    write_u8(rec + menubar_record::MODE, 2);
    write_i32(rec + menubar_record::CONTEXT, context);
}

/// The manager's menu of the team menu.
///
/// With no human manager (`count_human_managers`(0)), no manager or a manager being created: "Manager Options" as
/// inactive text. Otherwise a menu headed by the manager's name (`exe::database_format_staff_name`, broken near the
/// middle): with a national job (`staff::NATIONAL_JOB`), unless it is a national team club
/// (`exe::club_is_national_team`), its squad (0x7d5) and, with a reserve squad, its 'B' squad (0x7d6); the U21 squad
/// (U23 when the nation's flag in `G_NATIONAL_TEAMS` + 0x36 is set; the team of `exe::database_get_national_u21_team`);
/// with the senior team only or not all teams available (`HUMAN_MANAGER::controls_all_national_teams`), "Control Senior
/// Team Only"/"Control All Teams" (0x433); "FA Confidence" (0x420), "Resign from Nation" (0x41d, greyed out in a locked
/// slot or with a paused match) and a separator. With a club job (`staff::CLUB_JOB`): its squad, its reserves (0x7d6)
/// and "Control Senior Team Only"/"Control Reserve Team" (0x425) when it has a reserve squad, the linked B team's squad
/// (`exe::club_get_linked_b_team`), "Board Confidence" (0x41f), "Resign from Club" (0x41c, same rule) and a separator.
/// Then News (0x3e9), Player & Staff Search (0x3eb), Compare two chosen players (0x434), Manager Stats (0x42e), Job
/// Information (0x41e), Transfers (0x415), History (0x3ec); not on holiday: "Go on Holiday" (0x3ef, same rule) and the
/// pending click on a hidden item 0x42d when the countdown started; on holiday: the pending click on a hidden item
/// 0x3f0 when the screen manager has at most one active record (else none); with more than one human manager, "Send
/// Abuse To" with each other human manager (staff classification 1 among the last 16 staff records, 0x414 with the
/// person) and "All" (0x414); a separator and "Retire" (0x3f2, same rule).
unsafe fn menubar_manager_menu(area: usize, row: i32, staff: usize, rec: usize, m: usize) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if menubar_count_humans(0) == 0 || staff == 0 || menubar_being_created() {
        covered!("tel26_6 manager menu inactive");
        menubar_text(0xa5a558);
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(MENUBAR_HEADING_COLOUR));
        return;
    }
    covered!("tel26_6 manager menu");
    exe::database_format_staff_name(staff, MENUBAR_TEXT, 0x7d0, 1);
    exe::utils_break_line_near_middle(MENUBAR_TEXT as *mut u8, 1, 1);
    let heading = menubar_heading(0x82, row, 0x1021, 0xc, 1, colour_word(MENUBAR_HEADING_COLOUR), MENUBAR_TEXT, 0, area);
    let menu = menubar_submenu(heading);
    let hm = || read_usize(G_HUMAN_MANAGER);
    let free = || read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0 && m == 0;
    let squad = |format: usize, team: usize| {
        let g = read_i8(team + club::SHORT_GENDER_NAME) as i32;
        text(MENUBAR_TEXT, format, g, g, team + club::SHORT_NAME);
    };
    let national = read_usize(staff + staff::NATIONAL_JOB);
    if national != 0 {
        covered!("tel26_6 manager menu national job");
        if exe::club_is_national_team(national) == 0 {
            squad(0xa5a708, national);
            menubar_entry(MENUBAR_TEXT, 1, 0x7d5, national, menu);
            if exe::database_club_has_reserve_squad(national) != 0 {
                squad(0x9b76e8, national);
                menubar_entry(MENUBAR_TEXT, 1, 0x7d6, national, menu);
            }
        }
        let nation = read_usize(national + club::NATION);
        let u23 = read_u8(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x36) + read_usize(nation)) != 0;
        squad(if u23 { 0xa5a6e4 } else { 0xa5a6c0 }, national);
        let u21 = exe::database_get_national_u21_team(read_usize(national + club::NATION));
        menubar_entry(MENUBAR_TEXT, 1, 0x7d5, u21, menu);
        let all_teams: MenubarThis2 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
        let mut not_available = 0u32;
        if all_teams(hm(), 0, &mut not_available as *mut u32 as usize) != 0 || not_available == 0 {
            let only = all_teams(hm(), 0, 0) != 0;
            menubar_text(if only { 0xa5a6a4 } else { 0xa5a690 });
            menubar_entry(MENUBAR_TEXT, 1, 0x433, 0, menu);
        }
        menubar_text(0xa5a680);
        menubar_entry(MENUBAR_TEXT, 1, 0x420, 0, menu);
        menubar_text(0xa5a66c);
        menubar_entry_if(free(), 0x41d, menu);
        menubar_separator(menu);
    }
    let club = read_usize(staff + staff::CLUB_JOB);
    if club != 0 {
        covered!("tel26_6 manager menu club job");
        squad(0xa5a650, club);
        menubar_entry(MENUBAR_TEXT, 1, 0x7d5, club, menu);
        if exe::database_club_has_reserve_squad(club) != 0 {
            squad(0x990728, club);
            menubar_entry(MENUBAR_TEXT, 1, 0x7d6, club, menu);
            let only = exe::human_manager_controls_reserve_team(hm(), 0usize) != 0;
            menubar_text(if only { 0xa5a6a4 } else { 0xa5a638 });
            menubar_entry(MENUBAR_TEXT, 1, 0x425, 0, menu);
        }
        let b = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
        if b != 0 {
            squad(0xa5a650, b);
            menubar_entry(MENUBAR_TEXT, 1, 0x7d5, b, menu);
        }
        menubar_text(0xa1549c);
        menubar_entry(MENUBAR_TEXT, 1, 0x41f, 0, menu);
        menubar_text(0xa5a624);
        menubar_entry_if(free(), 0x41c, menu);
        menubar_separator(menu);
    }
    for (format, id) in [(0xa5a61cusize, 0x3e9usize), (0xa5a604, 0x3eb), (0xa5a5e8, 0x434), (0xa15324, 0x42e), (0xa15424, 0x41e), (0x98e880, 0x415)] {
        menubar_text(format);
        menubar_entry(MENUBAR_TEXT, 1, id, 0, menu);
    }
    text(MENUBAR_TEXT, 0xa5a5b8, MENUBAR_EMPTY);
    menubar_entry(MENUBAR_TEXT, 1, 0x3ec, 0, menu);
    let hidden = |id: usize| {
        menubar_item([1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_EMPTY, 0, id, 0, usize::MAX])
    };
    let pending = if exe::human_manager_is_on_holiday(hm(), 0usize) == 0 {
        menubar_text(0xa5a5a8);
        menubar_entry_if(free(), 0x3ef, menu);
        if read_u32(rec + menubar_record::HOLIDAY_PENDING) != 0 {
            covered!("tel26_6 manager menu holiday countdown click");
            hidden(0x42d)
        } else {
            usize::MAX
        }
    } else if core::mem::transmute::<usize, MenubarThis0>(0x7feb70)(SCREEN_MANAGER) as i16 > 1 {
        usize::MAX
    } else {
        covered!("tel26_6 manager menu on holiday click");
        hidden(0x3f0)
    };
    core::mem::transmute::<usize, MenubarThis1>(0x800160)(SCREEN_MANAGER, pending);
    if menubar_count_humans(0) > 1 {
        covered!("tel26_6 manager menu send abuse");
        menubar_text(0xa5a598);
        let item = menubar_entry(MENUBAR_TEXT, 1, 0, 0, menu);
        let abuse = menubar_submenu(item);
        for i in 0..0x10isize {
            let e = (read_usize(G_STAFF_TABLE) as isize + (read_i32(G_STAFF_COUNT) as isize - i - 1) * 0x6e) as usize;
            if read_u8(e + staff::CLASSIFICATION) == 1 && e != read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)) {
                let mut name = [0u8; 200];
                exe::database_format_staff_name(e, name.as_mut_ptr() as usize, 200, 1);
                menubar_entry(name.as_ptr() as usize, 1, 0x414, e, abuse);
            }
        }
        text(MENUBAR_TEXT, 0xa5a570, MENUBAR_EMPTY);
        menubar_entry(MENUBAR_TEXT, 1, 0x414, 0, abuse);
    }
    menubar_separator(menu);
    menubar_text(0xa5a568);
    menubar_entry_if(free(), 0x3f2, menu);
}

/// The Competitions menu: one submenu per entry of the nation order (`key_nation_sorted_order`(0), 35 entries) whose
/// `G_LEAGUE_TABLE` row has a competition list (+0x10).
///
/// Row 0: "World Cup" (a menu entry); 1: "International"; 2: "Worldwide Club"; a continent (+4): its adjective
/// (`exe::get_continent_adjective`); a nation (+0) with its +0x11c byte set: its name, under the part header
/// (`NATION_MENU_SPLIT::format_header`) when the list is split in two parts and the nation starts another part. A
/// row with neither shows the error box (Menubar.cpp line 0xd3f) and is skipped. Each submenu lists the competitions
/// (`+0xc` of them, `ScreenManager::add_menu_item` with link 0x7d0 and the competition) and, for a nation with awards
/// (`AWARD_MANAGER::find_nation_awards`), a separator and "Awards" (0x7d4). Rows 0 and 1 add a separator, their awards
/// and "FIFA Rankings" (0x3f3); row 3 a separator, its awards and "UEFA Coefficients" (0x40c).
unsafe fn menubar_competitions_menu(area: usize, row: i32) {
    covered!("tel26_6 competitions menu");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
    let add_menu_item: MenubarThis4 = core::mem::transmute(0x415640usize);
    let add_menu_area: MenubarThis1 = core::mem::transmute(0x415600usize);
    let separator: MenubarThis1 = core::mem::transmute(0x761260usize);
    let part_count: MenubarThis0 = core::mem::transmute(0x66a350usize);
    let in_second_part: MenubarThis1 = core::mem::transmute(0x66a430usize);
    let find_nation_awards: MenubarThis1 = core::mem::transmute(0x414340usize);
    menubar_text(0x98ed18);
    let heading = menubar_heading(0x82, row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
    let comps = menubar_submenu(heading);
    let mut last = 0xffu8;
    let mut part = 0usize;
    let order = core::mem::transmute::<usize, MenubarCdecl1>(0x66a0c0)(0);
    let mut split = [0u32; 4];
    let s = split.as_mut_ptr() as usize;
    core::mem::transmute::<usize, MenubarThis3>(0x66a280)(s, 0, order, 0);
    let awards_of = |w: usize| read_u32(read_usize(read_usize(w + 4)) + 5) as usize;
    for i in 0..0x23usize {
        let c = read_i8(order + i);
        let entry = (G_LEAGUE_TABLE as isize + c as isize * 0x48) as usize;
        if read_usize(entry + 0x10) == 0 {
            continue;
        }
        let item = match c {
            0 => {
                menubar_text(0xa16284);
                menubar_entry(MENUBAR_TEXT, 1, 0, 0, comps)
            }
            1 => {
                text(MENUBAR_TEXT, 0xa16248, MENUBAR_EMPTY);
                add_menu_item(SCREEN_MANAGER, comps, MENUBAR_TEXT, 0, 0) as usize
            }
            2 => {
                text(MENUBAR_TEXT, 0xa16210, MENUBAR_EMPTY);
                add_menu_item(SCREEN_MANAGER, comps, MENUBAR_TEXT, 0, 0) as usize
            }
            _ if read_usize(entry + 4) != 0 => {
                let mut adjective = [0u8; 100];
                exe::get_continent_adjective(read_usize(entry + 4), adjective.as_mut_ptr(), 100, 0x14);
                add_menu_item(SCREEN_MANAGER, comps, adjective.as_ptr() as usize, 0, 0) as usize
            }
            _ => {
                let nation = read_usize(entry);
                if nation == 0 {
                    covered!("tel26_6 competitions menu error box");
                    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
                    exe::crt_strtok(0xa5a51cusize, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
                    format(0xae24d0, 0x9870e8, exe::utils_get_version_string(), name.as_ptr(), ext.as_ptr(), 0xd3f);
                    exe::gui_show_message_box(0x9870e0usize, 0xae24d0usize, 0);
                    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
                    continue;
                }
                if read_u8(nation + 0x11c) == 0 {
                    continue;
                }
                if part_count(s) as u8 as i8 > 1 && in_second_part(s, nation) as u8 != last {
                    covered!("tel26_6 competitions menu split header");
                    last = in_second_part(s, nation) as u8;
                    core::mem::transmute::<usize, MenubarThis3>(0x66a360)(s, MENUBAR_TEXT, 0x7d0, last as usize);
                    let header = add_menu_item(SCREEN_MANAGER, comps, MENUBAR_TEXT, 0, 0) as usize;
                    part = add_menu_area(SCREEN_MANAGER, header) as usize;
                } else if part_count(s) as u8 == 1 {
                    part = comps;
                }
                add_menu_item(SCREEN_MANAGER, part, nation + 4, 0, 0) as usize
            }
        };
        let menu = add_menu_area(SCREEN_MANAGER, item) as usize;
        if read_usize(entry + 0x10) != 0 {
            let mut k = 0;
            while k < read_i32(entry + 0xc) {
                let e = read_usize(read_usize(entry + 0x10) + k as usize * 4);
                if e != 0 {
                    let comp = read_usize(e + 4);
                    add_menu_item(SCREEN_MANAGER, menu, comp + 0x38, 0x7d0, comp);
                }
                k += 1;
            }
            let nation = read_usize(entry);
            if nation != 0 {
                let awards = find_nation_awards(read_usize(G_AWARD_MANAGER), nation) as usize;
                if awards != 0 {
                    separator(SCREEN_MANAGER, menu);
                    menubar_text(0x99c0a8);
                    add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x7d4, awards_of(awards));
                }
            }
        }
        let c = read_u8(order + i);
        if c == 0 || c == 1 {
            separator(SCREEN_MANAGER, menu);
            let lists = read_usize(read_usize(G_AWARD_MANAGER) + 8);
            let w = if read_u8(order + i) == 0 { read_usize(lists) } else { read_usize(lists + 4) };
            menubar_text(0x99c0a8);
            add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x7d4, awards_of(w));
            menubar_text(0xa5a548);
            add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x3f3, 0);
        }
        if read_u8(order + i) == 3 {
            separator(SCREEN_MANAGER, menu);
            let w = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8) + 8);
            menubar_text(0x99c0a8);
            add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x7d4, awards_of(w));
            menubar_text(0x99c924);
            add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x40c, 0);
        }
    }
}

/// The Nations & Clubs menu (Nations 0x3f4, Under-21s 0x3f5, Major Clubs 0x3f6 and Other 0x3f9 listed by continent,
/// 0x765010; League 0x3f7 and Non-League 0x3f8 by league nation, 0x7651e0) and the Find menu (Nation, Club,
/// Non-Player, Player: 0x3fa with 1..4; with the "cm3_history" folder, a config +8 of 0 or 1 and outside a network
/// slot, a separator and "Written History" 0x429). `row` moves to the Find heading.
unsafe fn menubar_nations_and_find_menus(area: usize, row: &mut i32) {
    covered!("tel26_6 nations and find menus");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    *row += 1;
    menubar_text(0xa5a50c);
    let heading = menubar_heading(0x82, *row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
    let menu = menubar_submenu(heading);
    for (format, comment, id, list) in [
        (0x9c9f64usize, false, 0x3f4usize, 0x765010usize),
        (0x9c9f58, false, 0x3f5, 0x765010),
        (0x9c9f4c, false, 0x3f6, 0x765010),
        (0x990f4c, false, 0x3f7, 0x7651e0),
        (0x9c9f40, false, 0x3f8, 0x7651e0),
        (0x9c9f1c, true, 0x3f9, 0x765010),
    ] {
        if comment {
            text(MENUBAR_TEXT, format, MENUBAR_EMPTY);
        } else {
            menubar_text(format);
        }
        let item = menubar_entry(MENUBAR_TEXT, 1, 0, 0, menu);
        let sub = menubar_submenu(item);
        core::mem::transmute::<usize, MenubarCdecl3>(list)(sub, id, 0);
    }
    *row += 1;
    menubar_text(0xa5a504);
    let heading = menubar_heading(0x82, *row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
    let find = menubar_submenu(heading);
    for (format, data) in [(0x9ca010usize, 1usize), (0x9ca08c, 2), (0x9ca080, 3)] {
        menubar_text(format);
        menubar_entry(MENUBAR_TEXT, 1, 0x3fa, data, find);
    }
    text(MENUBAR_TEXT, 0xa5a4e0, MENUBAR_EMPTY);
    menubar_entry(MENUBAR_TEXT, 1, 0x3fa, 4, find);
    let folder = exe::utils_build_file_path(0x9cec18usize, MENUBAR_EMPTY);
    if core::mem::transmute::<usize, MenubarCdecl1>(0x90bf90)(folder) != 0
        && matches!(read_u32(read_usize(G_CONFIG) + 8), 0 | 1)
        && read_u16(menubar_slot(screen_manager::NETWORK_SLOT)) == 0
    {
        covered!("tel26_6 find menu written history");
        menubar_separator(find);
        menubar_text(0xa5a4d0);
        menubar_entry(MENUBAR_TEXT, 1, 0x429, 0, find);
    }
}

/// The Game Options menu: Save Game (0x3fe, outside a network slot, with no manager being created and no flagged
/// screen), Add Manager (0x3fb, `menubar_can_add_manager`), Manager Status (0x3fc), Chat (0x421, `menubar_can_chat`),
/// Game Settings (0x423), Selected Leagues (0x431), Hall of Fame (0x42a), Game Credits (0x42b), Web Sites (0x42c),
/// Restart Game (0x42f, outside a network slot and with no flagged screen) and, in a network slot, "Exit Network Game"
/// (0x402, greyed out in a locked slot), otherwise "Exit Game" (0x402, with no flagged screen).
unsafe fn menubar_game_options_menu(area: usize, row: i32) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    menubar_text(0xa5a4c0);
    let heading = menubar_heading(0x82, row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, 0, area);
    let menu = menubar_submenu(heading);
    let local = || read_u16(menubar_slot(screen_manager::NETWORK_SLOT)) == 0;
    menubar_text(0xa5a4b4);
    menubar_entry_if(local() && !menubar_being_created() && menubar_no_flagged_screens(), 0x3fe, menu);
    text(MENUBAR_TEXT, 0xa5a48c, MENUBAR_EMPTY);
    menubar_entry_if(menubar_can_add_manager(), 0x3fb, menu);
    menubar_text(0xa5a47c);
    menubar_entry(MENUBAR_TEXT, 1, 0x3fc, 0, menu);
    if menubar_can_chat() {
        menubar_text(0xa5a7a0);
        menubar_entry(MENUBAR_TEXT, 1, 0x421, 0, menu);
    }
    for (format, id) in [(0xa5a46cusize, 0x423usize), (0xa5a458, 0x431), (0x9ce514, 0x42a), (0x9b52fc, 0x42b), (0x9b53b4, 0x42c)] {
        menubar_text(format);
        menubar_entry(MENUBAR_TEXT, 1, id, 0, menu);
    }
    text(MENUBAR_TEXT, 0xa5a42c, MENUBAR_EMPTY);
    menubar_entry_if(local() && menubar_no_flagged_screens(), 0x42f, menu);
    if !local() {
        covered!("tel26_6 game options network exit");
        menubar_text(0xa5a418);
        menubar_entry_if(read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0, 0x402, menu);
    } else {
        text(MENUBAR_TEXT, 0xa5a3f0, MENUBAR_EMPTY);
        menubar_entry_if(menubar_no_flagged_screens(), 0x402, menu);
    }
}

/// The match menu (mode 3) of paused match `m`: the minute (`m` +0x5f, +0x4d) in font 5, the arrows, "Paused", "<team>
/// Tactics" of the home and away teams and "Commentary Speed" as inactive text; mode 3 and the context in the record.
unsafe fn menubar_build_match_menu(area: usize, rec: usize, m: usize, context: i32) {
    covered!("tel26_6 match menu");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
    let mut row = 0i32;
    format(MENUBAR_TEXT, 0x98806c, read_i16(read_usize(m + 0x5f) + 0x4d) as i32);
    menubar_area_text(area, row, 0x1061, 0xc, 5, colour_word(G_GUI_COLOUR_YELLOW));
    row += 1;
    menubar_arrows(row);
    row += 1;
    menubar_text(0xa16d54);
    menubar_heading(1, row, 0x1061, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
    row += 1;
    for side in [0x20usize, 0x24] {
        let team = read_usize(m + side);
        let g = read_i8(team + club::SHORT_GENDER_NAME) as i32;
        text(MENUBAR_TEXT, 0xa16428, g, g, team + club::SHORT_NAME);
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY));
        row += 1;
    }
    menubar_text(0xa16414);
    menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY));
    write_u8(rec + menubar_record::MODE, 3);
    write_i32(rec + menubar_record::CONTEXT, context);
}

/// The status item at the bottom of the bar (`menubar_record::STATUS_WIDGET` and `STATUS_KIND`), outside the match menu:
/// in an unlocked slot with unread news (`exe::news_count_unread`), "You Have News" in red (0x3e9, kind 1); with a chat
/// message waiting for the manager (`MENUBAR_CHAT_PENDING`), "Chat Message" in red (0x421, kind 2); in an unlocked slot
/// with players waiting (`count_human_managers`(1)), "Player Waiting" (2 humans) or "<waiting> <total> Players" in
/// yellow (0x3fc, kind 3, the counts in `menubar_record::WAITING` and `TOTAL`); otherwise an empty item (kind 0). The
/// item kind is 0x802 with the config flag 0x20000, else 2. The counts are zeroed in the other cases.
unsafe fn menubar_build_status(rec: usize, mode: u8) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let kind = || if read_u32(read_usize(G_CONFIG) + 4) & 0x20000 == 0 { 2 } else { 0x802 };
    let status = |colour: usize, text: usize, id: usize| {
        menubar_item([kind(), 10, 0x226, 0x50, 0x24e, 0, 0, 1, colour_word(G_GUI_COLOUR_BLACK), colour_word(G_GUI_COLOUR_BLACK), 0xc, 1, colour,
            text, 0, id, 0, usize::MAX]) as u16
    };
    let mut kind_built = 0u8;
    let mut widget = None;
    if read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0
        && read_usize(G_NEWS) != 0
        && exe::news_count_unread(read_usize(G_NEWS), read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)), 0) > 0
        && mode != 3
    {
        covered!("tel26_6 status news");
        menubar_text(0xa5a3e0);
        widget = Some(status(colour_word(G_GUI_COLOUR_RED), MENUBAR_TEXT, 0x3e9));
        kind_built = 1;
    }
    if widget.is_none() {
        let s = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
        let chat = s != 0 && {
            let k = read_u16(s).wrapping_sub(read_u16(G_STAFF_COUNT)).wrapping_add(0x10) as i16;
            read_u32((MENUBAR_CHAT_PENDING as isize + k as isize * 4) as usize) != 0
        } && mode != 3;
        if chat {
            covered!("tel26_6 status chat");
            menubar_text(0xa5a3d0);
            widget = Some(status(colour_word(G_GUI_COLOUR_RED), MENUBAR_TEXT, 0x421));
            kind_built = 2;
        } else if read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0 {
            let waiting = menubar_count_humans(1);
            if waiting > 0 && mode != 3 {
                covered!("tel26_6 status players waiting");
                let total = menubar_count_humans(0);
                if total == 2 {
                    menubar_text(0xa5a3c0);
                } else {
                    text(MENUBAR_TEXT, 0xa5a394, waiting, total);
                }
                let w = status(colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, 0x3fc);
                write_u16(rec + menubar_record::STATUS_WIDGET, w);
                write_u8(rec + menubar_record::STATUS_KIND, 3);
                write_i32(rec + menubar_record::WAITING, waiting);
                write_i32(rec + menubar_record::TOTAL, total);
                return;
            }
        }
    }
    let w = match widget {
        Some(w) => w,
        None => status(colour_word(G_GUI_COLOUR_BLACK), MENUBAR_EMPTY, 0),
    };
    write_u16(rec + menubar_record::STATUS_WIDGET, w);
    write_u8(rec + menubar_record::STATUS_KIND, kind_built);
    write_i32(rec + menubar_record::WAITING, 0);
    write_i32(rec + menubar_record::TOTAL, 0);
}

/// Comparer for `menubar_build`.
///
/// The bar is compared side by side over the display list (the screen manager, the current Display, the parameter
/// block, the text buffer, the generator) plus the human's menu bar record, without its refresh time
/// (`menubar_record::REFRESH_TIME`, `utils_get_time_ms` + 1000 in each run). The build time (time()) is not compared.
pub(crate) unsafe extern "C" fn compare_menubar_build(mode: u32, context: i32) {
    let o = original_fn::<unsafe extern "C" fn(u32, i32)>(IDX_MENUBAR_BUILD);
    if read_u32(MENUBAR_NETWORK_GAME) != 0 {
        // simplification: in a network game the countdown sends a packet (ScreenManager::send_clear_click) and the
        // times are compared across a second boundary; only the original runs
        return o(mode, context);
    }
    let mut extra = vec![(MENUBAR_TEXT, 0x800)];
    let s = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
    if s != 0 {
        let k = read_i32(s).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
        if (0..0x10).contains(&k) {
            let rec = MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE;
            extra.push((rec, menubar_record::REFRESH_TIME));
            extra.push((rec + menubar_record::HEADER_WIDGET, MENUBAR_RECORD_SIZE - menubar_record::HEADER_WIDGET));
        }
    }
    // simplification: a competition row with neither nation nor continent shows the error box in both runs
    compare_by_display_list_with("menubar_build", &extra, || o(mode, context), || menubar_build(mode, context));
}
