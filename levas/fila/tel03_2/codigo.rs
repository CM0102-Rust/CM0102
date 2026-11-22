// ---------- batch %%LEVA%%: the club history screen (club_screens.cpp, part 3): opening, building, input, leaving ----------

/// The club history screen's flag the builder raises on every build (0x98ded8; read elsewhere) [inference].
const CLUB_HISTORY_BUILT_FLAG: usize = 0x98ded8;
/// The club_screens.cpp source path the assert boxes print.
const CLUB_SCREENS_SOURCE: usize = 0x98e598;
/// The shared text buffer the screens format into (0xde1f64).
const HISTORY_TEXT: usize = 0xde1f64;

/// Shows the club_screens.cpp "Error" box at the given line (the game continues) and clears the left mouse flag: the
/// original's `_splitpath` of the source path, `utils_get_version_string`, "v%s %s.%s %d" and the message box.
unsafe fn club_history_error_box(line: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(CLUB_SCREENS_SOURCE, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let s = join(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    format(0xae24d0, 0x9870e8, s); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// `ScreenManager::add_area_text` (0x804100, thiscall, ret 0x24) of the club history screen: cell (`col`, `row`) of
/// area `area` with the two layout words `a4`/`a5`, the font `font`, the style `style`, the colour and the text (0 =
/// an empty cell).
unsafe fn club_history_cell(area: usize, col: u32, row: i32, a4: u32, a5: usize, font: u32, style: u32, colour: usize, text: usize) -> usize {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(G_SCREEN_MANAGER, area, col as usize, row as usize, a4 as usize, a5, font as usize, style as usize, colour, text)
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): draws a Text into area `area` from x1 to x2, starting at row
/// `*row` (which it moves on), with the font, the colour, the style and the last flag.
unsafe fn club_history_draw(text: *mut usize, area: usize, x1: i32, x2: i32, row: *mut i32, font: u32, colour: usize, style: u32) {
    let f: unsafe extern "thiscall" fn(*mut usize, usize, usize, usize, *mut i32, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4aeaf0usize);
    f(text, area, x1 as usize, x2 as usize, row, font as usize, colour, style as usize, 0);
}

/// `ScreenManager::set_area_scroll_row` (0x4567a0, thiscall, ret 8).
unsafe fn club_history_restore_scroll(area: usize, row: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4567a0usize);
    f(G_SCREEN_MANAGER, area, row);
}

/// The 36 parameters of the club history screen as `open_club_history_screen` leaves them (`prev` = the previous
/// screen's 60 params, read before opening, used with `restore`).
unsafe fn club_history_planned_params(club: usize, restore: u32, players_tab: u32, prev: &[usize; 0x3c]) -> [usize; 0x24] {
    let mut p = [0usize; 0x24];
    p[0] = club;
    p[1] = if players_tab != 0 { 8 } else { 3 };
    p[0x1d] = usize::MAX;
    p[2] = 1;
    p[7] = 1;
    p[0xa] = 3;
    p[0xd] = 7;
    p[0x10] = 0xc;
    p[0x13] = if players_tab != 0 { 0x20 } else { 0x10 };
    p[0x16] = 0x1c;
    p[0x20] = 1;
    p[0x23] = 1;
    if restore != 0 {
        for k in [1usize, 2, 7, 0xa, 0xd, 0x10, 0x13, 0x16] {
            p[k] = prev[k];
        }
    }
    p
}

/// Opens the club history screen for `club` (builder `club_history_screen_build`, handler `club_history_screen_input`,
/// leave `club_history_screen_leave`). The params: 0 the club, 1 the tab (3 Records, or 8 Players with
/// `players_tab`), 0x19..0x1c the history lists (none yet), 0x1d the landmark period −1, 2 the records view 1, 7, 0xa,
/// 0xd, 0x10, 0x13 and 0x16 the views of the positions, attendances, results, sequences, players and transfers tabs (1,
/// 3, 7, 0xc, 0x10 or 0x20 with `players_tab`, 0x1c), 0x20 the landmarks page 1, 0x23 the page count 1, the rest 0.
/// With `restore`, params 1, 2, 7, 0xa, 0xd, 0x10, 0x13 and 0x16 come back from the previous screen (its 60 params read
/// before opening).
/// Original: `open_club_history_screen` (0x469e70, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_club_history_screen(club: usize, restore: u32, players_tab: u32) {
    covered!("tel03_2 open club history");
    let mut prev = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in prev.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x46a1e0, 0x46dd20, 0, 0x46e560, 0) == 0 {
        return;
    }
    let p = club_history_planned_params(club, 0, players_tab, &prev);
    // the original's order of writes
    for k in [0usize, 1, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x20, 0x21, 0x22, 0x23] {
        set_screen_param(k as u32, p[k]);
    }
    if restore != 0 {
        for k in [1u32, 2, 7, 0xa, 0xd, 0x10, 0x13, 0x16] {
            set_screen_param(k, prev[k as usize]);
        }
    }
}

/// The Landmarks tab of the club history screen: the club's history (`Staff_Records::get_club_history`, mask 0), one
/// row per entry from the newest (the list walked from its tail): the date (`Date::get_short_date_string`) and the
/// entry's text (`Staff_Records::format_club_history_entry`, wrapped in the second column). More than 100 entries go in
/// pages of as many rows as fit in font 2 (param 0x23 the page count, the page buttons at the top, ids 0xd and 0xe;
/// param 0x20 the page); up to 100 all rows go in, scrolled, and the text column is narrower (0x2f6) when they do not
/// fit. Empty rows fill the rest; the scroll row comes back from param 0x21 and the area goes to param 0x22.
/// Original: part of `club_history_screen_build` (0x46a1e0, club_screens.cpp).
unsafe fn club_history_landmarks_tab(top: i32, bottom: i32) {
    let format_entry: unsafe extern "thiscall" fn(usize, usize, *mut usize) = core::mem::transmute(0x86f300usize);
    let history: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86df90usize);
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x54a650usize);
    let page_buttons: unsafe extern "C" fn(u32, i32, usize, i32, u32, u32, u32, u32) = core::mem::transmute(0x5ede20usize);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let weights: [u8; 2] = [3, 0x14];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let club = get_screen_param(0);
    let page = get_screen_param(0x20);
    let list = history(read_usize(G_STAFF_RECORDS), club, 0);
    let rows = ((bottom - top - 0x19) / exe::font_get_height(2)) & 0xff;
    let mut y1 = top;
    if list != 0 && read_i16(list + llist::COUNT) > 100 {
        covered!("tel03_2 landmarks pages");
        let count = read_i16(list + llist::COUNT) as i32;
        let pages = count / rows + (count % rows != 0) as i32;
        set_screen_param(0x23, pages as i16 as isize as usize);
        page_buttons(0x30c, top, page, pages, 0, 0xd, 0xe, 0);
        y1 = top + 0x19;
    }
    let rows = ((bottom - y1) / exe::font_get_height(2)) & 0xff;
    let area = add_area(current_human_display(), 0x6e, y1 as usize, 0x30c, bottom as usize, 2, weights.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    let mut r: i32 = 0;
    let dark_blue = colour_word(G_GUI_COLOUR_DARK_BLUE);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    if list != 0 {
        let count = read_i16(list + llist::COUNT) as i32;
        let x2 = if count > rows && count <= 100 { 0x2f6 } else { 0x30c }; // tel03_2 the landmark text column
        let x1 = (x2 * 3 - 0x14a) / 23 + 0x6e;
        let mut k = (page as i16 as i32 - 1) * rows;
        write_u32(list + llist::CURSOR, read_u32(list + llist::TAIL));
        for _ in 0..k.max(0) {
            exe::llist_get_prev(list);
        }
        while (r < rows || read_i16(list + llist::COUNT) < 100) && k < read_i16(list + llist::COUNT) as i32 {
            let e = exe::llist_get_prev(list);
            short_date(e, HISTORY_TEXT, 0x7d0, 0xff);
            club_history_cell(area, 0, r, 0x30, dark_blue, 0xc, 1, pale, HISTORY_TEXT);
            format_entry(read_usize(G_STAFF_RECORDS), e, t);
            club_history_cell(area, 1, r, 2, 0, 1, 2, yellow, 0);
            club_history_draw(t, area, x1, x2, &mut r, 2, yellow, 2);
            r += 1;
            k += 1;
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    while r < rows {
        for col in 0..3 {
            club_history_cell(area, col, r, 2, 0, 0xc, 2, pale, 0);
        }
        r += 1;
    }
    club_history_restore_scroll(area, get_screen_param(0x21));
    set_screen_param(0x22, area as i16 as isize as usize);
    exe::drop_text(t);
}

/// The Records tab of the club history screen: the View button (a popup of the two periods,
/// `club_records_get_period_label` 1 and 2, the chosen one, param 2, shown as the heading) and one row per record type
/// 1..0x20 that applies to the club in that period (`club_records_record_type_applies`): the type's label as a button
/// (link 5 with the type) and the record's text (`club_records_get_record_text`, wrapped). Empty rows fill the rest; the
/// scroll row comes back from param 3 (`Display::set_area_scroll_row`) and the area goes to param 4.
/// Original: part of `club_history_screen_build` (0x46a1e0, club_screens.cpp).
unsafe fn club_history_records_tab(top: i32, bottom: i32) {
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let area_button: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let add_area: DisplayListFn = core::mem::transmute(0x4567f0usize); // ScreenManager::add_area
    let popup: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x456850usize);
    let menu_item: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x415640usize);
    let period_label: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x448fa0usize);
    let type_label: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x4490b0usize);
    let applies: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x449590usize);
    let record_text: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut usize) -> u32 = core::mem::transmute(0x4496d0usize);
    let scroll: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x46ad90usize);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let weights: [u8; 2] = [1, 2];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let club = get_screen_param(0);
    // the view is a char (the callees read the low byte)
    let view = get_screen_param(2) as u8;
    lang(HISTORY_TEXT, 0x988004); // "View"
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let grey = colour_word(G_GUI_COLOUR_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    let b = button_item(G_SCREEN_MANAGER, 0x6e, top as usize, 0xeb, (top + 0x14) as usize, 0x30, grey, 0xc, 1, pale, HISTORY_TEXT, 0, 0);
    let menu = popup(G_SCREEN_MANAGER, b);
    let mut heading = [0u8; 0x7d0];
    let records = read_usize(G_CLUB_RECORDS);
    for k in 1..=2u32 {
        period_label(records, k, HISTORY_TEXT, 0x7d0);
        menu_item(G_SCREEN_MANAGER, menu, HISTORY_TEXT, 4, k);
        if k == view as i8 as i32 as u32 {
            let mut i = 0;
            loop {
                let c = read_u8(HISTORY_TEXT + i);
                heading[i] = c;
                if c == 0 {
                    break;
                }
                i += 1;
            }
        }
    }
    let y = top + 0x19;
    text_item(G_SCREEN_MANAGER, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, yellow, heading.as_ptr() as usize, 1);
    let y = y + 0x28;
    let rows = ((bottom - y) / exe::font_get_height(2)) as u8 as usize;
    let area = add_area(G_SCREEN_MANAGER, 0x6e, y as usize, 0x30c, bottom as usize, 2, weights.as_ptr() as usize, rows, 0, 1, 0, usize::MAX);
    let x1 = (weights[0] as i32 * 648) / (weights[0] as i32 + weights[1] as i32) + 0x6e;
    let mut r: i32 = 0;
    let mut label = [0u8; 0x7d0];
    for kind in 1..=0x20u32 {
        if applies(records, kind, club, view as u32) == 0 {
            continue;
        }
        type_label(records, kind, label.as_mut_ptr() as usize, 0x7d0);
        format(HISTORY_TEXT, 0x987ff0, label.as_ptr() as usize); // "  %s"
        area_button(G_SCREEN_MANAGER, area, 0, r as usize, 2, 0, 1, 2, pale, HISTORY_TEXT, 5, kind as usize);
        record_text(records, club, view as u32, kind, t);
        club_history_cell(area, 1, r, 2, 0, 1, 1, 0, 0);
        club_history_draw(t, area, x1 + 8, 0x2ee, &mut r, 1, yellow, 2);
        r += 1;
    }
    while r < rows as i32 {
        club_history_cell(area, 0, r, 2, 0, 1, 2, pale, 0);
        club_history_cell(area, 1, r, 2, 0, 1, 1, pale, 0);
        r += 1;
    }
    scroll(current_human_display(), area as u32, get_screen_param(3));
    set_screen_param(4, area as i16 as isize as usize);
    exe::drop_text(t);
}

/// Builds the club history screen of param 0 (with no club, the club_screens.cpp assert box, line 0x2436): the section
/// picture 8, the title "{club} History" in the club's kit colours (`database_get_club_kit_colours` 1), the menu bar,
/// the club's action button, the tab bar of 9 tabs (Competitions 1, Landmarks 2, Records 3, Positions 4, Attendances
/// 5, Results 6, Sequences 7, Players 8, Transfers 9; for a national team Positions and Transfers are greyed out with
/// no value, and an open one of them goes back to Competitions, param 1 = 1), then the open tab (param 1) from the top
/// and bottom the tab bar leaves: Competitions, Positions, Attendances, Results, Sequences, Players and Transfers by
/// their own routines, Landmarks and Records here; any other value an empty area. At the end the squad navigation menu
/// (`build_club_squad_navigation_menu` 1, 0xc) and the Back/Next buttons. Raises 0x98ded8.
///
/// Convention: cdecl, SEH frame (0x1ec0 bytes of locals: 25 menu items built and destroyed by the CRT vector
/// iterators). Split into named pieces (`club_history_landmarks_tab`, `club_history_records_tab`); the whole routine
/// is hooked.
/// Original: `club_history_screen_build` (0x46a1e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_screen_build() {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let section_picture: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(0x41c8e0usize);
    let action_button: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x482a40usize);
    let navigation: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x481ef0usize);
    let tab_panel = |f: usize, top: i32, bottom: i32| {
        let g: unsafe extern "C" fn(i32, i32) = core::mem::transmute(f);
        g(top, bottom)
    };
    let mut tabs: MatchTabs = [0; 0x14b4 / 4];
    construct_match_tabs(&mut tabs);
    write_u32(CLUB_HISTORY_BUILT_FLAG, 1);
    let club = get_screen_param(0);
    if club == 0 {
        covered!("tel03_2 club history with no club");
        club_history_error_box(0x2436);
        destruct_match_tabs(&mut tabs);
        return;
    }
    covered!("tel03_2 club history build");
    let mut tab = get_screen_param(1) as u8;
    section_picture(8, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
    lang(HISTORY_TEXT, 0x990ac8, gender, gender, club + club::SHORT_NAME); // "{}<%s - club name>{} History"
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, HISTORY_TEXT);
    exe::menubar_build(4, 0);
    action_button(club, back, fore);

    let titles: [usize; 9] = [0x98ed18, 0x990abc, 0x990ab4, 0x990aa8, 0x990a9c, 0x990a94, 0x990a88, 0x990a80, 0x98e880];
    for (k, &title) in titles.iter().enumerate() {
        lang(match_tab_title(&mut tabs, k), title);
        // the national teams have no positions and no transfers: greyed out (the word at +2), no mark and no value
        if (k == 3 || k == 8) && exe::database_is_national_team(club) != 0 {
            if tab == k as u8 + 1 {
                set_screen_param(1, 1);
                tab = 1;
            }
            let base = tabs.as_mut_ptr() as usize + k * 0xd4;
            write_u16(base + 2, read_u16(G_GUI_COLOUR_GREY));
            set_match_tab_flags(&mut tabs, k, 0, 0);
        } else {
            set_match_tab_flags(&mut tabs, k, 2, k as u32 + 1);
        }
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(tabs.as_mut_ptr() as usize, 9, tab as i8 as i32, &mut top, &mut bottom, 1);
    match tab {
        1 => tab_panel(0x46adc0, top, bottom),
        2 => club_history_landmarks_tab(top, bottom),
        3 => club_history_records_tab(top, bottom),
        4 => tab_panel(0x46c030, top, bottom),
        5 => tab_panel(0x46c510, top, bottom),
        6 => tab_panel(0x46c9e0, top, bottom),
        7 => tab_panel(0x46ceb0, top, bottom),
        8 => tab_panel(0x46d370, top, bottom),
        9 => tab_panel(0x46d860, top, bottom),
        _ => {
            let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
            add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
        }
    }
    navigation(club, back, fore, 1, 0xc, 0, 0, 0);
    let next = (exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0) as u32;
    let back_possible = (exe::screenmanager_is_back_unavailable(G_SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back_possible, next);
    destruct_match_tabs(&mut tabs);
}

/// One View popup of a club history tab (the sequences, players and transfers tabs): for a real widget, the area's
/// scroll row (param `area_param`) is saved into param `scroll_param` first; then, when the widget's action id (on the
/// current human's Display, or the injected one when it has none) is `id` and its value differs from param
/// `view_param`, the value goes there and the answer is −11 (rebuild); otherwise 0.
unsafe fn club_history_view_input(widget: u32, scroll_param: u32, area_param: u32, id: u16, view_param: u32) -> i32 {
    let w = widget as i16;
    let mut action = 0u16;
    if w != -1 {
        set_screen_param(scroll_param, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area_param)));
        action = read_u16((current_human_display() as isize + w as isize * 0x18c + display::ITEM_ID as isize) as usize);
    }
    if action == 0 {
        action = read_u16(G_SCREEN_MANAGER + 0x26278e);
    }
    if action != id {
        return 0;
    }
    let value = |w: i16| if w == -1 { 0 } else { read_usize((current_human_display() as isize + w as isize * 0x18c + display::ITEM_VALUE as isize) as usize) };
    if value(w) == get_screen_param(view_param) {
        return 0;
    }
    set_screen_param(view_param, value(w));
    -11
}

/// The View popup of the Sequences tab: action 9 sets the record type (param 0x10); scroll row in param 0x11 from the
/// area of param 0x12.
/// Original: `club_history_sequences_tab_input` (0x46e260, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_sequences_tab_input(widget: u32) -> i32 {
    club_history_view_input(widget, 0x11, 0x12, 9, 0x10)
}

/// The View popup of the Players tab: action 0xa sets the record type (param 0x13); scroll row in param 0x14 from the
/// area of param 0x15.
/// Original: `club_history_players_tab_input` (0x46e360, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_players_tab_input(widget: u32) -> i32 {
    club_history_view_input(widget, 0x14, 0x15, 0xa, 0x13)
}

/// The View popup of the Transfers tab: action 0xb sets the record type (param 0x16); scroll row in param 0x17 from
/// the area of param 0x18.
/// Original: `club_history_transfers_tab_input` (0x46e460, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_transfers_tab_input(widget: u32) -> i32 {
    club_history_view_input(widget, 0x17, 0x18, 0xb, 0x16)
}

/// One "pick a value" answer of the club history screen: when `action` is `id` and the widget's value differs from
/// param `param`, the value goes there (read twice, as in the original) and the answer is −11; otherwise None.
unsafe fn club_history_pick(widget: u32, action: i32, id: i32, param: u32) -> Option<i32> {
    if action != id || exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) == get_screen_param(param) {
        return None;
    }
    set_screen_param(param, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
    Some(-11)
}

/// `ScreenManager::get_widget_action_id` of the club history screen's widget.
unsafe fn club_history_action(widget: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x468b40usize);
    f(G_SCREEN_MANAGER, widget)
}

/// Answers a click on the club history screen. First by the open tab (param 1), each saving its area's scroll row
/// before (for a real widget): Competitions (scroll 0x1e from area 0x1f) action 3 (the item id on the current human's
/// Display) picks the period (param 0x1d); Landmarks (0x22 from 0x21) actions 0xd and 0xe page back and forth (param
/// 0x20, 1 to the page count of param 0x23); Records (3 from 4) action 4 picks the period (param 2), action 5 a record
/// type: `club_record_type_to_category` gives the tab (param 1) and the param of its view, which gets the type, the
/// next param 0; Positions (8 from 9) action 6 → param 7; Attendances (0xb from 0xc) action 7 → param 0xa; Results
/// (0xe from 0xf) action 8 → param 0xd; Sequences, Players and Transfers by their View popup routines. Then for every
/// tab: action 1 opens another club's history (the widget's club, restoring the views, −4); action 0xc opens the hall
/// of fame of the club's English name (`history_screen_open`, −4); action 2 changes the tab (param 1, −11); anything
/// else goes to the club screen actions, the menu bar commands and the object links, in that order.
/// Original: `club_history_screen_input` (0x46dd20, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_screen_input(widget: u32) -> i32 {
    let w = widget as i16;
    let club = get_screen_param(0);
    let tab = get_screen_param(1) as u8 as i8;
    let scroll = |save: u32, area: u32| {
        if w != -1 {
            set_screen_param(save, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area)));
        }
    };
    let answer = match tab {
        1 => {
            scroll(0x1e, 0x1f);
            let mut id = exe::display_get_widget_id(current_human_display(), widget);
            if id == 0 {
                id = read_u16(G_SCREEN_MANAGER + 0x26278e);
            }
            if id == 3 {
                club_history_pick(widget, 3, 3, 0x1d)
            } else {
                None
            }
        }
        2 => {
            scroll(0x22, 0x21);
            let page = get_screen_param(0x20) as i16 as i32;
            let pages = get_screen_param(0x23) as i16 as i32;
            let action = club_history_action(widget);
            if action == 0xd {
                set_screen_param(0x20, (page - 1).max(1) as usize);
                Some(-11)
            } else if action == 0xe {
                set_screen_param(0x20, (page + 1).min(pages) as usize); // tel03_2 the next page
                Some(-11)
            } else {
                None
            }
        }
        3 => {
            scroll(3, 4);
            let picked = club_history_pick(widget, club_history_action(widget), 4, 2);
            if picked.is_some() {
                picked
            } else if club_history_action(widget) == 5 {
                covered!("tel03_2 club history record type");
                let to_category: unsafe extern "C" fn(u32, *mut i32) -> u8 = core::mem::transmute(0x483920usize);
                let kind = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) as u8;
                // the original passes its argument slot as the out param: it starts with the widget
                let mut param: i32 = widget as i32;
                let category = to_category(kind as u32, &mut param);
                set_screen_param(1, category as i8 as isize as usize);
                set_screen_param(param as u32, kind as i8 as isize as usize);
                set_screen_param((param + 1) as u32, 0);
                Some(-11)
            } else {
                None
            }
        }
        4 => {
            scroll(8, 9);
            club_history_pick(widget, club_history_action(widget), 6, 7)
        }
        5 => {
            scroll(0xb, 0xc);
            club_history_pick(widget, club_history_action(widget), 7, 0xa)
        }
        6 => {
            scroll(0xe, 0xf);
            club_history_pick(widget, club_history_action(widget), 8, 0xd)
        }
        7 | 8 | 9 => {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(match tab {
                7 => 0x46e260usize,
                8 => 0x46e360usize,
                _ => 0x46e460usize,
            });
            let r = f(widget);
            if r != 0 {
                Some(r)
            } else {
                None
            }
        }
        _ => None,
    };
    if let Some(r) = answer {
        return r;
    }
    let action = club_history_action(widget);
    if action == 1 {
        covered!("tel03_2 club history of another club");
        let open: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x469e70usize);
        open(exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget), 1, 0);
        return -4;
    }
    if club_history_action(widget) == 0xc {
        let hall_of_fame: unsafe extern "C" fn(usize) = core::mem::transmute(0x5f11c0usize);
        hall_of_fame(exe::club_get_english_name(club));
        return -4;
    }
    if let Some(r) = club_history_pick(widget, club_history_action(widget), 2, 1) {
        return r;
    }
    let club_action: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x482d10usize);
    let r = club_action(widget);
    if r != 0 {
        return r;
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Frees what the club history screen holds when it closes: the history lists of params 0x19..0x1c (`LLIST::~LLIST`
/// and delete, each param back to 0) and the record array of param 5 (`free`; params 5 and 6 back to 0).
/// Original: `club_history_screen_leave` (0x46e560, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_screen_leave() {
    for k in 0x19..=0x1cu32 {
        if get_screen_param(k) != 0 {
            let list = get_screen_param(k);
            if list != 0 {
                exe::drop_llist(list);
                exe::crt_operator_delete(list);
            }
            set_screen_param(k, 0);
        }
    }
    if get_screen_param(5) != 0 {
        covered!("tel03_2 club history leave with records");
        exe::crt_free(get_screen_param(5));
        set_screen_param(5, 0);
        set_screen_param(6, 0);
    }
}

/// Comparer for `open_club_history_screen`: opening a screen does not run twice; only the original runs and the new
/// screen's 36 params are checked against the plan (`club_history_planned_params`), when it holds the club in param 0.
pub(crate) unsafe extern "C" fn compare_open_club_history_screen(club: usize, restore: u32, players_tab: u32) {
    let mut prev = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in prev.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let plan = club_history_planned_params(club, restore, players_tab, &prev);
    original_fn::<unsafe extern "C" fn(usize, u32, u32)>(%%I0%%)(club, restore, players_tab);
    if get_screen_param(0) == club && club != 0 {
        let mut got = [0usize; 0x24];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("club history screen (open {:#x}): params {:x?} x plan {:x?}", club, got, plan));
    }
}

/// Comparer for `club_history_screen_build`: both build side by side and are compared by the screen (the display list,
/// `compare_match_screen_build`) plus the built flag 0x98ded8. When the open tab still has to build the lists it keeps
/// in the params (Competitions: params 0x19..0x1c; the season tabs: the record array of param 5), only the original
/// runs: building twice would leave two lists and the params would differ by the pointer.
pub(crate) unsafe extern "C" fn compare_club_history_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let club = get_screen_param(0);
    let mut tab = get_screen_param(1) as u8;
    if club != 0 && (tab == 4 || tab == 9) && exe::database_is_national_team(club) != 0 {
        tab = 1;
    }
    let missing = |ks: &[u32]| ks.iter().any(|&k| get_screen_param(k) == 0);
    if club == 0 || (tab == 1 && missing(&[0x19, 0x1a, 0x1b, 0x1c])) || ((4..=9).contains(&tab) && missing(&[5])) {
        return o();
    }
    compare_match_screen_build("club history screen", &[(CLUB_HISTORY_BUILT_FLAG, 4)], || o(), || club_history_screen_build());
}

/// Whether a click on the club history screen only writes params (the tab answers that end in −11 without calling
/// another screen): only then both run side by side.
unsafe fn club_history_click_stays(widget: u32) -> bool {
    let w = widget as i16;
    if w == -1 {
        return false;
    }
    let action = club_history_action(widget);
    let data = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget);
    let differs = |p: u32| data != get_screen_param(p);
    let by_tab = match get_screen_param(1) as u8 {
        1 => {
            let mut id = exe::display_get_widget_id(current_human_display(), widget);
            if id == 0 {
                id = read_u16(G_SCREEN_MANAGER + 0x26278e);
            }
            id == 3 && differs(0x1d)
        }
        2 => action == 0xd || action == 0xe,
        3 => (action == 4 && differs(2)) || action == 5,
        4 => action == 6 && differs(7),
        5 => action == 7 && differs(0xa),
        6 => action == 8 && differs(0xd),
        7 => action == 9 && differs(0x10),
        8 => action == 0xa && differs(0x13),
        9 => action == 0xb && differs(0x16),
        _ => false,
    };
    by_tab || (action == 2 && differs(1))
}

/// Comparer for `club_history_screen_input`: the clicks that only write params run side by side (the params and the
/// answer, `compare_resposta`); the ones that open a screen or go to the menu bar run the original only.
pub(crate) unsafe extern "C" fn compare_club_history_screen_input(widget: u32) -> i32 {
    let stays = club_history_click_stays(widget);
    compare_resposta("club history screen input", widget, original_fn(%%I2%%), club_history_screen_input, !stays)
}

/// Comparer for `club_history_sequences_tab_input`: side by side over the params and the answer.
pub(crate) unsafe extern "C" fn compare_club_history_sequences_tab_input(widget: u32) -> i32 {
    compare_resposta("club history sequences tab input", widget, original_fn(%%I3%%), club_history_sequences_tab_input, false)
}

/// Comparer for `club_history_players_tab_input`: side by side over the params and the answer.
pub(crate) unsafe extern "C" fn compare_club_history_players_tab_input(widget: u32) -> i32 {
    compare_resposta("club history players tab input", widget, original_fn(%%I4%%), club_history_players_tab_input, false)
}

/// Comparer for `club_history_transfers_tab_input`: side by side over the params and the answer.
pub(crate) unsafe extern "C" fn compare_club_history_transfers_tab_input(widget: u32) -> i32 {
    compare_resposta("club history transfers tab input", widget, original_fn(%%I5%%), club_history_transfers_tab_input, false)
}

/// Comparer for `club_history_screen_leave`: freeing does not run twice; only the original runs and the params it
/// clears are checked afterwards (0x19..0x1c, and 5 and 6 when there was a record array).
pub(crate) unsafe extern "C" fn compare_club_history_screen_leave() {
    let records = get_screen_param(5) != 0;
    original_fn::<unsafe extern "C" fn()>(%%I6%%)();
    let mut left = [0usize; 6];
    for (i, k) in [0x19u32, 0x1a, 0x1b, 0x1c, 5, 6].iter().enumerate() {
        left[i] = get_screen_param(*k);
    }
    let cleared = left[..4].iter().all(|&v| v == 0) && (!records || (left[4] == 0 && left[5] == 0));
    SCOREBOARD.record(cleared, || format!("club history screen leave: params 0x19..0x1c, 5, 6 left {:x?}", left));
}
