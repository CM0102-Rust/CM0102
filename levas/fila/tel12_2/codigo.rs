// ---------- batch %%LEVA%%: the Setup screens, part 6: Hall of Fame and the disk full and no memory boxes (Setup.cpp) ----------

// (uses the Setup helpers of tel11_1 and setup_box_question of tel11_4)
const FN_CRT_GETDRIVE: usize = 0x9473a7; // _getdrive() -> drive number (1 = A)
const FN_CRT_LOCALTIME: usize = 0x946c40; // localtime(&time_t) -> struct tm *
const FN_HALL_OF_FAME_GET_TABLE_NAME: usize = 0x5f07c0; // hall_of_fame_get_table_name(table, buffer, size, short_form) -> 1 or 0
const FN_HALL_OF_FAME_SORT_LEAGUE_ORDER: usize = 0x5f10c0; // hall_of_fame_sort_league_order(name_style) -> u8 * (28 tables)
const FN_HALL_OF_FAME_GET_ENTRY: usize = 0x5ef130; // thiscall(HallOfFame, table, rank, entry *) ret 0xc -> 1 or 0
const FN_HALL_OF_FAME_EXPORT_TO_CSV: usize = 0x5f0430; // thiscall(HallOfFame, path) ret 4 -> 1 or 0
const FN_HALL_OF_FAME_IMPORT_FILE: usize = 0x5efd50; // thiscall(HallOfFame, path, check_only) ret 8 -> 1 or 0
const HALL_OF_FAME_TRY_LOAD_QUIET: usize = 0x5f1090; // hall_of_fame_try_load_quiet(path): the Merge In dialog's file check
const G_GUI_COLOUR_GREEN: usize = 0xae31aa; // g_gui_colour_green
const G_GUI_COLOUR_DARK_PURPLE: usize = 0xae3192; // g_gui_colour_dark_purple
const HALL_OF_FAME_ENTRY_NAME: usize = 0x0; // the manager's name (text)
const HALL_OF_FAME_ENTRY_CLUB: usize = 0x32; // the club's name (text) [inference: second column of the list]
const HALL_OF_FAME_ENTRY_DATE: usize = 0x64; // time_t of the entry (0: none)
const HALL_OF_FAME_ENTRY_POINTS: usize = 0x68; // the points (0: empty rank)

/// Opens the Hall of Fame screen (`hall_of_fame_screen_build`, `hall_of_fame_screen_handle_input`) with its params 0, 1
/// and 2 (the table shown, the list's scroll row and the list's area) zeroed.
/// Original: `open_hall_of_fame_screen` (0x825cc0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_hall_of_fame_screen() {
    covered!("tel12_2 open hall of fame");
    setup_open_screen(0x825d10, 0x826690, 0, 3);
}

/// One button of the Hall of Fame's View menu: table `table`'s name (`hall_of_fame_get_table_name`, long form) after a
/// space in the shared text; the table shown (`shown`) is a plain item (type 0x2002, no id), the others id 0x2e with the
/// table as value; green, yellow and black, in the menu area `area`.
unsafe fn hall_of_fame_table_button(table: u8, shown: u8, area: usize) {
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
unsafe fn hall_of_fame_submenu(label: usize, area: usize) -> usize {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    text(SETUP_TEXT, label);
    let (green, yellow, black) = (setup_colour(G_GUI_COLOUR_GREEN), setup_colour(G_GUI_COLOUR_YELLOW), setup_colour(G_GUI_COLOUR_BLACK));
    let id = item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, 0, 0, area);
    add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, green, id)
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
    let set_scroll_row: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AREA_SET_SCROLL_ROW);
    let button: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, usize, usize, u32, u32, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let cell: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
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
        cell(SCREEN_MANAGER, list, 0, row, 0x30, dark_blue, 0xc, 1, pale, ordinal);
        if points != 0 {
            format(SETUP_TEXT, 0x987ff0, e + HALL_OF_FAME_ENTRY_NAME); // "  %s"
            cell(SCREEN_MANAGER, list, 1, row, 1, 0, 1, 2, pale, SETUP_TEXT);
            cell(SCREEN_MANAGER, list, 2, row, 1, 0, 1, 2, yellow, e + HALL_OF_FAME_ENTRY_CLUB);
            let when = read_i32(e + HALL_OF_FAME_ENTRY_DATE);
            if when > 0 {
                let tm = localtime(&when);
                let year = read_u16(tm + 0x14).wrapping_add(0x76c);
                exe::date_set(date.as_mut_ptr() as usize, read_u16(tm + 0xc) as u32, read_u8(tm + 0x10) as u32, year as u32, -1);
                let mut buffer = [0u8; 0xc8];
                date_text(date.as_mut_ptr() as usize, buffer.as_mut_ptr() as usize, 0xc8, 0xff);
                cell(SCREEN_MANAGER, list, 3, row, 1, 0, 1, 1, orange, buffer.as_ptr() as usize);
            }
            format(SETUP_TEXT, 0x98c11c, points); // "%ld"
            cell(SCREEN_MANAGER, list, 4, row, 0x30, purple, 0xc, 1, pale, SETUP_TEXT);
        } else {
            cell(SCREEN_MANAGER, list, 4, row, 0x30, purple, 0xc, 1, pale, 0xa80e64); // "------"
        }
        row += 1;
    }
    let scroll = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 1);
    let list = list as i16;
    if list != -1 {
        set_scroll_row(current_human_display() + 4 + list as isize as usize * 0xbf1, scroll);
    }
    exe::screenmanager_set_screen_param(SCREEN_MANAGER, 2, list as isize as usize, 0);
    let next = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, next);
}

/// Shows the Setup "Error" box at `row` after a Hall of Fame file failed, freeing the dialog's path first.
unsafe fn hall_of_fame_file_error(path: usize, row: u32) {
    exe::crt_operator_delete(path);
    setup_warning(row);
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
    let scroll_row: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_GET_AREA_SCROLL_ROW);
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
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 1, scroll_row(SCREEN_MANAGER, area), 0);
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

/// Opens a Setup message box (`build`/`input`) with the interactive flag and `reason` in param 0.
unsafe fn setup_open_reason_box(build: usize, input: usize, reason: usize) {
    if exe::screenmanager_open_screen(SCREEN_MANAGER, build, input, read_u32(G_INTERACTIVE_MODE_FLAG), 0usize, 0) != 0 {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, reason, 0);
    }
}

/// Opens the disk full box (`disk_full_box_build`, `disk_full_box_handle_input`) with `reason` in param 0.
/// Original: `open_disk_full_box` (0x826a60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_disk_full_box(reason: usize) {
    covered!("tel12_2 open disk full");
    setup_open_reason_box(0x826aa0, 0x826c20, reason);
}

/// The common part of the disk full and no memory boxes: a random background, the menu bar (mode 4), the Setup title in
/// red with pale grey text and the subtitle `subtitle` (translated).
unsafe fn setup_reason_box_header(subtitle: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    exe::background_set_random_picture();
    exe::menubar_build(4, 0);
    setup_format_title();
    exe::screenmanager_add_screen_title(SCREEN_MANAGER, setup_colour(G_GUI_COLOUR_RED) as u32, setup_colour(G_GUI_COLOUR_PALE_GREY) as u32, G_SETUP_SCREEN_TITLE);
    text(SETUP_TEXT, subtitle);
    exe::screenmanager_add_screen_subtitle(SCREEN_MANAGER, SETUP_TEXT, 0);
}

/// The Ok button of the disk full and no memory boxes (0x64, 0x22b to 0x316, 0x24e; grey, font 4, pale grey), id `id`.
unsafe fn setup_reason_box_ok(id: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    text(SETUP_TEXT, 0x990f7c); // "Ok"
    let (grey, pale) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY));
    item(current_human_display(), 2, 0x64, 0x22b, 0x316, 0x24e, 0, 0, 0x30, grey, grey, 0xc, 4, pale, SETUP_TEXT, 0, id, 0, usize::MAX);
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

/// The answer of the disk full (`ok` 0x33) and no memory (0x34) boxes: Ok closes the box (−9 when the screen manager's
/// current screen has a flag, −5 otherwise); back (−2) gives 0; anything else goes to the menu bar.
unsafe fn setup_reason_box_answer(item: u32, ok: u16) -> i32 {
    let flag: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7fed30usize); // ScreenManager::get_current_screen_flag
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
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer of `hall_of_fame_screen_build`: side by side over the display list, with the screen params (the list's
/// area goes into param 2).
pub(crate) unsafe extern "C" fn compare_hall_of_fame_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let extra = screen_param_regions();
    compare_by_display_list_with("Hall of Fame screen", &extra, || o(), || hall_of_fame_screen_build());
}

/// Comparer of `hall_of_fame_screen_handle_input`: a click with no file command waiting that only picks a table or
/// goes nowhere (id 0x2e) runs side by side over the screen manager and the screen params; the file dialogs, the
/// waiting commands (they write files) and the menu bar run only in the original.
pub(crate) unsafe extern "C" fn compare_hall_of_fame_screen_handle_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
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
    original_fn::<unsafe extern "C" fn(usize)>(%%I3%%)(reason)
}

/// Comparer of `disk_full_box_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_disk_full_box_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    compare_by_display_list_with("Disk Full box", &[], || o(), || disk_full_box_build());
}

/// Comparer of `disk_full_box_handle_input`: only the original runs (Ok closes the box, the menu bar opens screens).
pub(crate) unsafe extern "C" fn compare_disk_full_box_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%)(item)
}

/// Comparer of `open_no_memory_box`: only the original runs (it pushes a screen).
pub(crate) unsafe extern "C" fn compare_open_no_memory_box(reason: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I6%%)(reason)
}

/// Comparer of `no_memory_box_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_no_memory_box_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    compare_by_display_list_with("No Memory box", &[], || o(), || no_memory_box_build());
}

/// Comparer of `no_memory_box_handle_input`: only the original runs (Ok closes the box, the menu bar opens screens).
pub(crate) unsafe extern "C" fn compare_no_memory_box_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I8%%)(item)
}
