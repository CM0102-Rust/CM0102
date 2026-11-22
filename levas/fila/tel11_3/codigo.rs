// ---------- batch %%LEVA%%: the Setup screens, part 3: Enter Name, Select Nationality, Select Team (Setup.cpp) ----------

// (uses the Setup helpers of tel11_1 and FN_HUMAN_MANAGER_GET_PASSWORD(_ENTRY) of tel11_2)
const FN_NATION_MENU_SPLIT_CONSTRUCT: usize = 0x66a280; // thiscall(split, filtered_only, order, by_second_name) ret 0xc
const FN_NATION_MENU_SPLIT_PART_COUNT: usize = 0x66a350; // thiscall(split) -> al (1 or 2)
const FN_NATION_MENU_SPLIT_FORMAT_HEADER: usize = 0x66a360; // thiscall(split, buffer, size, part) ret 0xc
const FN_NATION_MENU_SPLIT_IN_SECOND_PART: usize = 0x66a430; // thiscall(split, nation) ret 4 -> al
const FN_CRT_TOUPPER: usize = 0x9471cd; // toupper(c)
const SETUP_NATIONALITY_COMPARE: usize = 0x839530; // the qsort comparer of the nationality list (by name)
const SETUP_CLUB_COMPARE: usize = 0x540080; // the qsort comparer of the team list (by name)
const G_SCREEN_MANAGER_SLOT_RECORD_COUNTS: usize = 0xb77c5a; // g_screen_manager_slot_record_counts (16 words)
const G_TEXT_BUFFER_CHAR_2: usize = 0xde1f66; // g_text_buffer_char_2
const COMPETITION_ABBREVIATION: usize = 0x53; // the club competition's abbreviation text [inference: shown next to the club]
const CONTINENT_SHORT_NAME: usize = 0x1f; // a continent name text [inference: shown next to national sides]

/// The 33 major nations of the nationality filter (the key nation ids, in the original's order).
const SETUP_MAJOR_NATIONS: [usize; 33] = [KEY_NATION_ARGENTINA, KEY_NATION_AUSTRALIA, KEY_NATION_BELGIUM, KEY_NATION_BRAZIL,
    KEY_NATION_CANADA, KEY_NATION_CROATIA, KEY_NATION_DENMARK, KEY_NATION_ENGLAND, KEY_NATION_FINLAND, KEY_NATION_FRANCE,
    KEY_NATION_GERMANY, KEY_NATION_GREECE, KEY_NATION_HOLLAND, KEY_NATION_HONG_KONG, KEY_NATION_ICELAND, KEY_NATION_ISRAEL,
    KEY_NATION_ITALY, KEY_NATION_JAPAN, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_NORWAY, KEY_NATION_POLAND,
    KEY_NATION_PORTUGAL, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_RUSSIA, KEY_NATION_SCOTLAND, KEY_NATION_SINGAPORE,
    KEY_NATION_SOUTH_AFRICA, KEY_NATION_SOUTH_KOREA, KEY_NATION_SPAIN, KEY_NATION_SWEDEN, KEY_NATION_TURKEY,
    KEY_NATION_UNITED_STATES, KEY_NATION_WALES];

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
    let confirm: unsafe extern "C" fn(usize, usize, usize, u32, usize, usize, u32) = core::mem::transmute(FN_CONFIRM_PUSH);
    let mut buffer = [0u8; 0x100];
    let h = text(buffer.as_mut_ptr() as usize, heading);
    confirm(G_SETUP_SCREEN_TITLE, h, SETUP_TEXT, 0, text_colour, colour, code);
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
    let manager = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
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
    item(current_human_display(), 0x80008 | focused(0), 0, 0, 0, 0, 2, 0, 1, 0, 0, 1, 3, pale, read_usize(manager + staff::FIRST_NAME), 0x1a, 0x4d, 0, a);
    let a = row(0x117, 0xa80710, pale); // "   Second Name"
    item(current_human_display(), 0x80008 | focused(1), 0, 0, 0, 0, 2, 0, 1, 0, 0, 1, 3, pale, read_usize(manager + staff::SECOND_NAME), 0x1a, 0x4e, 0, a);
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
    let password: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD);
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
            if read_u8(password(hm(), 0)) != 0 {
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
            let manager = read_usize(manager_slot());
            let (first, second) = (read_u8(read_usize(manager + staff::FIRST_NAME)), read_u8(read_usize(manager + staff::SECOND_NAME)));
            let (pale, dark_red) = (setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_DARK_RED));
            if first != 0 && second != 0 {
                if read_u8(password(hm(), 0)) != 0 && setup_texts_differ(password(hm(), 0), typed(hm(), 0)) {
                    write_u8(password(hm(), 0), 0);
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
    let manager = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
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
        if n == read_usize(manager + staff::NATION) {
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
            let manager = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
            write_usize(manager + staff::NATION, setup_clicked_value(item16));
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
    covered!("tel11_3 build select team");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let order_of: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_KEY_NATION_SORTED_ORDER);
    let split_new: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(FN_NATION_MENU_SPLIT_CONSTRUCT);
    let split_parts: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_NATION_MENU_SPLIT_PART_COUNT);
    let split_header: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_NATION_MENU_SPLIT_FORMAT_HEADER);
    let split_part_of: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_NATION_MENU_SPLIT_IN_SECOND_PART);
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
        let order = order_of(0);
        let mut split = [0u8; 0x40];
        let sp = split.as_mut_ptr() as usize;
        split_new(sp, 1, order, 0);
        let parts = split_parts(sp) as i8;
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
                split_header(sp, SETUP_TEXT, 0x7d0, part as u8 as u32);
                let header = item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, SETUP_TEXT, 0, 0, 0, popup);
                let sub = area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, green, header);
                for k in 0..0x23 {
                    let n = selected(k);
                    if n != 0 && split_part_of(sp, n) == part as u8 {
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
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    compare_by_display_list_with("Enter Name screen", &[], || o(), || setup_build_enter_name_screen());
}

/// Comparer of `setup_handle_enter_name_input`: the password entry (0x1a) runs side by side over the screen's params
/// and the typed password; Cancel, Next and the menu bar only in the original (they free records, open boxes and
/// screens).
pub(crate) unsafe extern "C" fn compare_setup_handle_enter_name_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
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
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    compare_by_display_list_with("Select Nationality screen", &[], || o(), || setup_build_select_nationality_screen());
}

/// Comparer of `setup_handle_select_nationality_input`: the filter and nation buttons (0x1c, 0x1d, 0x1e) run side by
/// side over the screen's params and the current human's nation; Next and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_setup_handle_select_nationality_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I3%%);
    if !matches!(setup_clicked_id(item as i16), 0x1c | 0x1d | 0x1e) {
        return o(item);
    }
    let manager = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    let mut regions = vec![(SCREEN_MANAGER, 0x3840)];
    regions.extend(screen_param_regions());
    if manager != 0 {
        regions.push((manager + staff::NATION, 4));
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || setup_handle_select_nationality_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Select Nationality answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `setup_build_select_team_screen`: side by side over the display list (the club vector is born and freed
/// inside); with the club vector's allocation the heap differs between the two runs, which the comparison does not see.
pub(crate) unsafe extern "C" fn compare_setup_build_select_team_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    compare_by_display_list_with("Select Team screen", &[], || o(), || setup_build_select_team_screen());
}

/// Comparer of `select_team_screen_handle_input`: the nation chooser (0x1f, with a different nation) runs side by side
/// over the screen's params; Next and the menu bar (a club opens the take control box) only in the original.
pub(crate) unsafe extern "C" fn compare_select_team_screen_handle_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%);
    if setup_clicked_id(item as i16) != 0x1f || setup_clicked_value(item as i16) == exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0) {
        return o(item);
    }
    let mut regions = vec![(SCREEN_MANAGER, 0x3840)];
    regions.extend(screen_param_regions());
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || select_team_screen_handle_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Select Team answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}
