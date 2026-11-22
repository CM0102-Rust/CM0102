// ---------- batch %%LEVA%%: the competition history screen (comp_screens.cpp, part 2): the Attendances, Results, Sequences, Teams and Players tabs, the screen's clicks and leaving it ----------

/// The history screen's season records list (param 9; entries of `HISTORY_SEASON_RECORD_SIZE` bytes: the season word
/// at +0, the record at +2) and its count (param 0xa), shared by the list tabs and built on the first one shown
/// (`Comp_Stats::build_season_records_list`) [inference].
const HISTORY_RECORDS_LIST: u32 = 9;
const HISTORY_RECORDS_COUNT: u32 = 0xa;
/// A season record of that list (0x26b bytes) [inference].
const HISTORY_SEASON_RECORD_SIZE: usize = 0x26b;
/// The last Comp_Stats record type (1..=0x16).
const HISTORY_LAST_RECORD_TYPE: u32 = 0x16;
/// The column weights of a list tab (2 columns: the season and the record).
const HISTORY_LIST_WEIGHTS: [u8; 2] = [1, 8];

/// `Comp_Stats::is_record_type_shown` (0x4a54c0, thiscall, ret 4): whether the competition keeps records of `kind`.
unsafe fn comp_stats_record_shown(stats: usize, kind: u32) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a54c0usize);
    f(stats, kind) != 0
}

/// `comp_stats_get_record_title` (0x4a5120, thiscall, ret 0xc): the title of record type `kind` into `buf`.
unsafe fn comp_stats_record_title(stats: usize, kind: u32, buf: usize, size: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x4a5120usize);
    f(stats, kind, buf, size)
}

/// `Comp_Stats::build_season_records_list` (0x4ab0c0, thiscall, ret 4): a new heap list of the competition's season
/// records, its count into `count`.
unsafe fn comp_stats_build_season_records(stats: usize, count: &mut i32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, *mut i32) -> usize = core::mem::transmute(0x4ab0c0usize);
    f(stats, count)
}

/// `comp_stats_format_record_text` (0x4a5690, thiscall, ret 0xc): the text of a season record for record type `kind`
/// (its low byte) into the Text object.
unsafe fn comp_stats_season_record_text(stats: usize, record: usize, kind: u32, text: *mut usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize) = core::mem::transmute(0x4a5690usize);
    f(stats, record, kind, text)
}

/// `key_nation_format_comp_season` (0x6697c0, cdecl): the season label of `season` for the competition record.
unsafe fn comp_season_label(season: u32, comp: usize, buf: usize, size: u32) {
    let f: unsafe extern "C" fn(u32, usize, usize, u32) = core::mem::transmute(0x6697c0usize);
    f(season, comp, buf, size)
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): the Text into an area from x1 to x2 starting at `row` (moved
/// to its last row), in a font, colour and style.
unsafe fn history_draw_wrapped(text: *mut usize, area: u32, x1: i32, x2: i32, row: &mut i32, font: u32, colour: u32, style: u32) {
    let f: unsafe extern "thiscall" fn(*mut usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4aeaf0usize);
    f(text, area, x1, x2, row, font, colour, style, 0);
}

/// One of the history screen's list tabs (`tab` 3..7: Attendances, Results, Sequences, Teams, Players), the five
/// routines being the same code with the tab's numbers.
///
/// The tab's view (the record type shown) is param `0xb + 3 × (tab − 3)`, its scroll row the next param and its area
/// the one after. A "View" button (x 0x6e..0xeb, 0x14 high) with its popup (flags 0x130, [0xadae0c] = 1) lists the
/// record types 1..0x16 of this tab (`comp_history_tab_for_record`): the ones the competition keeps
/// (`Comp_Stats::is_record_type_shown`) as " <title>" items (id `tab + 2`, the type as value; the view ticked, flags
/// 0x2002, and its title without the leading space kept as the heading), the others greyed (flags 0x21, no id). The
/// heading in yellow font 3 (0x23 high) 0x19 below the top; 0x28 further down a 2-column area (weights 1, 8) of as many
/// rows as fit in font 2. The season records list (params 9 and 0xa) is built on the first use
/// (`Comp_Stats::build_season_records_list`); every record with a season gets a row: the season label
/// (`key_nation_format_comp_season`, dark blue, font 0xc) and the record's text for the view
/// (`comp_stats_format_record_text`) wrapped from x 0xbe to 0x2ee in yellow font 2. The rows left are filled with
/// empty yellow cells; the scroll row comes back and the area goes to its param.
/// Kept from the original: with no type ticked the heading is stack garbage there (empty here); the row count goes
/// as its byte (the original's dword carries the popup area's high bytes; Display keeps the byte).
unsafe fn history_list_tab(object: usize, top: i32, bottom: i32, tab: u32) {
    let view_param = 0xb + 3 * (tab - 3);
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut list = get_screen_param(HISTORY_RECORDS_LIST);
    let mut count = get_screen_param(HISTORY_RECORDS_COUNT) as i32;
    let view = get_screen_param(view_param) as u8;
    comp_lang_format()(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1,
                                   job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0, 0]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    write_u32(COMP_SCREEN_DATE_BUTTONS_FLAG, 1);
    let stats = read_usize(object + league::COMP_STATS);
    let mut heading = [0u8; 0x7d0];
    let mut title = [0u8; 0xc8];
    for kind in 1..=HISTORY_LAST_RECORD_TYPE {
        if comp_history_tab_for_record(kind, core::ptr::null_mut()) as u32 != tab {
            continue;
        }
        let (c31aa, yellow, black) = (job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_BLACK));
        if comp_stats_record_shown(stats, kind) {
            comp_stats_record_title(stats, kind, title.as_mut_ptr() as usize, 0xc8);
            comp_crt_format()(JOB_TEXT, 0x98ed14, title.as_ptr()); // " %s"
            let ticked = kind as u8 == view;
            let flags = if ticked { 0x2002 } else { 2 };
            tel26_add_item(current_human_display(), [flags, 0, 0, 0, 0, 0, 0, 0x10, c31aa, yellow, 1, 1, black, JOB_TEXT as u32, 0, tab + 2, kind, popup]);
            if ticked {
                tel26_copy_text(heading.as_mut_ptr(), (JOB_TEXT + 1) as *const u8);
            }
        } else {
            comp_stats_record_title(stats, kind, JOB_TEXT, 0x7d0);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, yellow, 0x21, 1, black, JOB_TEXT as u32, 0, 0, 0, popup]);
        }
    }
    let top = top + 0x19;
    comp_text_item([0x6e, top as u32, 0x30c, (top + 0x23) as u32, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), heading.as_ptr() as u32, 1]);
    let top = top + 0x28;
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 2, HISTORY_LIST_WEIGHTS.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    if list == 0 {
        covered!("tel23_1 history records list built");
        list = comp_stats_build_season_records(stats, &mut count);
        set_screen_param(HISTORY_RECORDS_LIST, list);
        set_screen_param(HISTORY_RECORDS_COUNT, count as usize);
    }
    for k in 0..count.max(0) as usize {
        let record = list + k * HISTORY_SEASON_RECORD_SIZE;
        let season = read_u16(record);
        if season == 0 {
            continue;
        }
        comp_season_label(season as u32, read_usize(object + league::COMPETITION), JOB_TEXT, 0x7d0);
        tel26_area_text([area, 0, row as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
        comp_stats_season_record_text(stats, record + 2, view as u32, t);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, 0, 0]);
        history_draw_wrapped(t, area, 0xbe, 0x2ee, &mut row, 2, job_colour(G_GUI_COLOUR_YELLOW), 2); // tel23_1 the record text's x
        row += 1;
    }
    while row < rows as i32 {
        tel26_area_text([area, 0, row as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_YELLOW), 0]);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_YELLOW), 0]);
        row += 1;
    }
    let scroll = get_screen_param(view_param + 1);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), scroll);
    }
    set_screen_param(view_param + 2, area as i16 as isize as usize);
    exe::drop_text(t);
}

/// Rule: the history screen's Attendances tab (tab 3): the attendance records season by season (`history_list_tab`
/// with view param 0xb, popup id 5, scroll param 0xc, area param 0xd).
/// Original: `comp_screens_draw_history_attendances_tab` (0x499900, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_attendances_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 3)
}

/// Rule: the history screen's Results tab (tab 4): the result records season by season (`history_list_tab` with view
/// param 0xe, popup id 6, scroll param 0xf, area param 0x10).
/// Original: `comp_screens_draw_history_results_tab` (0x499e10, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_results_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 4)
}

/// Rule: the history screen's Sequences tab (tab 5): the sequence records season by season (`history_list_tab` with
/// view param 0x11, popup id 7, scroll param 0x12, area param 0x13).
/// Original: `comp_screens_draw_history_sequences_tab` (0x49a320, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_sequences_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 5)
}

/// Rule: the history screen's Teams tab (tab 6): the team records season by season (`history_list_tab` with view
/// param 0x14, popup id 8, scroll param 0x15, area param 0x16).
/// Original: `comp_screens_draw_history_teams_tab` (0x49a830, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_teams_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 6)
}

/// Rule: the history screen's Players tab (tab 7): the player records season by season (`history_list_tab` with view
/// param 0x17, popup id 9, scroll param 0x18, area param 0x19).
/// Original: `comp_history_draw_players_tab` (0x49ad40, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_draw_players_tab(object: usize, top: i32, bottom: i32) {
    history_list_tab(object, top, bottom, 7)
}

/// `history_screen_open` (0x5f11c0, cdecl): the written history screen of a competition by its English name.
unsafe fn history_screen_open_by_name(name: usize) {
    let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x5f11c0usize);
    f(name)
}

/// The view param a list tab's popup id picks (ids 5..8: tabs 3..6, params 0xb, 0xe, 0x11, 0x14).
fn history_view_param(tab: i8) -> u32 {
    0xb + 3 * (tab as u32 - 3)
}

/// Rule: the history screen's clicks.
///
/// The object is param 0 and the tab param 1. With a widget, the open tab's list keeps its scroll row (Winners: param
/// 4 from the area in 5; Records 7/8; the list tabs their scroll and area params). Records: id 3 (a View period other
/// than param 6) writes param 6 and answers −11; id 4 (a record title) opens the tab of its type
/// (`comp_history_tab_for_record`: param 1 = the tab, its view param = the type, the param after it = 0; −11), unless
/// the type has no tab. The list tabs: their popup id (tab + 2) with a type other than the view writes it (−11). The
/// Players tab: `comp_history_players_tab_handle_click` first (nonzero answers). Then, for every tab: id 1 opens the
/// history screen of the widget's competition (restoring the views; −4), id 0xa the written history
/// (`history_screen_open` of the competition's English name; −4), id 2 a tab other than param 1 (−11); otherwise the
/// menu bar (`menubar_handle_command`, then `menubar_handle_object_link`).
/// Defect of the original: a record title whose type is outside 1..0x16 shows the error box and opens tab 3 with the
/// view written to the param numbered by the widget (the out slot is the widget's own argument).
/// Original: `comp_history_screen_handle_event` (0x49b250, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_screen_handle_event(widget: u32) -> i32 {
    let object = get_screen_param(0);
    let tab = get_screen_param(1) as u8 as i8;
    let w = widget as i16;
    match tab {
        1 => {
            if w != -1 {
                comp_click_keep_scroll(4, 5);
            }
        }
        2 => {
            if w != -1 {
                comp_click_keep_scroll(7, 8);
            }
            if comp_click_action_id(widget) == 3 && exe::display_get_widget_data(current_human_display(), widget) != get_screen_param(6) {
                covered!("tel23_1 history records period");
                set_screen_param(6, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
                return -11;
            }
            if comp_click_action_id(widget) == 4 {
                let kind = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) as u8;
                let mut out = widget;
                let picked = comp_history_tab_for_record(kind as u32, &mut out) as i8;
                if picked > -1 {
                    covered!("tel23_1 history record title");
                    set_screen_param(1, picked as isize as usize);
                    set_screen_param(out, kind as i8 as isize as usize);
                    set_screen_param(out.wrapping_add(1), 0);
                    return -11;
                }
            }
        }
        3..=6 => {
            let view = history_view_param(tab);
            if w != -1 {
                comp_click_keep_scroll(view + 1, view + 2);
            }
            if comp_click_action_id(widget) == tab as i32 + 2 && exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) != get_screen_param(view) {
                covered!("tel23_1 history list view");
                set_screen_param(view, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
                return -11;
            }
        }
        7 => {
            let r = comp_history_players_tab_handle_click(widget);
            if r != 0 {
                return r;
            }
        }
        _ => {}
    }
    match comp_click_action_id(widget) {
        1 => {
            let comp = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget);
            let open: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x498780usize);
            open(comp, 1);
            -4
        }
        0xa => {
            history_screen_open_by_name(comp_get_english_name(read_usize(object + league::COMPETITION)));
            -4
        }
        2 if exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) != get_screen_param(1) => {
            set_screen_param(1, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
            -11
        }
        _ => comp_click_menubar(widget),
    }
}

/// Rule: the Players tab's clicks: with a widget the list keeps its scroll row (param 0x18 from the area in 0x19); id
/// 9 (the View popup) with a type other than param 0x17 writes it and answers −11; otherwise 0.
/// Original: `comp_history_players_tab_handle_click` (0x49b6e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_players_tab_handle_click(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(0x18, 0x19);
    }
    if tel26_item_id(w) != 9 || tel26_item_value(w) as usize == get_screen_param(0x17) {
        return 0;
    }
    covered!("tel23_1 history players view");
    set_screen_param(0x17, tel26_item_value(w) as usize);
    -11
}

/// Rule: leaving the history screen frees the season records list (param 9, crt free) and clears params 9 and 0xa.
/// Original: `comp_history_screen_close` (0x49b7e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_screen_close() {
    comp_free_block_param(HISTORY_RECORDS_LIST);
}

/// Rule: the history tab of a Comp_Stats record type and the param that holds that tab's view.
///
/// Type 1 has none (−1, nothing written); 2..5 the Teams tab (6, param 0x14); 6..8 Attendances (3, param 0xb); 9 and 10
/// Results (4, param 0xe); 11..14 Sequences (5, param 0x11); 15..22 Players (7, param 0x17). The param is written to
/// `out` when it is given. Any other type shows the comp_screens.cpp error box (line 0x232b) and answers 3 without
/// writing.
/// Original: `comp_history_tab_for_record` (0x49eb80, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_history_tab_for_record(kind: u32, out: *mut u32) -> u8 {
    let (tab, param) = match kind as u8 as i8 {
        1 => return 0xff,
        2..=5 => (6u8, 0x14u32),
        6..=8 => (3, 0xb),
        9 | 10 => (4, 0xe),
        11..=14 => (5, 0x11),
        15..=22 => (7, 0x17),
        _ => {
            comp_screens_error_box(0x232b);
            return 3;
        }
    };
    if !out.is_null() {
        *out = param;
    }
    tab
}

/// The comparer of a history list tab: side by side over the display list (plus [0xadae0c]) once the season records
/// list exists; its first build (it allocates the list into params 9 and 0xa) only the original.
unsafe fn compare_history_list_tab(name: &str, object: usize, original: impl FnOnce(), rust: impl FnOnce()) {
    if object == 0 || get_screen_param(HISTORY_RECORDS_LIST) == 0 {
        // simplification: the allocation does not run twice; proven in Rust mode
        return original();
    }
    compare_match_screen_build(name, &[(COMP_SCREEN_DATE_BUTTONS_FLAG, 4)], original, rust);
}

/// Comparer for `comp_screens_draw_history_attendances_tab`.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_attendances_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I0%%);
    compare_history_list_tab("comp history attendances tab", object, || o(object, top, bottom), || comp_screens_draw_history_attendances_tab(object, top, bottom));
}

/// Comparer for `comp_screens_draw_history_results_tab`.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_results_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I1%%);
    compare_history_list_tab("comp history results tab", object, || o(object, top, bottom), || comp_screens_draw_history_results_tab(object, top, bottom));
}

/// Comparer for `comp_screens_draw_history_sequences_tab`.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_sequences_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I2%%);
    compare_history_list_tab("comp history sequences tab", object, || o(object, top, bottom), || comp_screens_draw_history_sequences_tab(object, top, bottom));
}

/// Comparer for `comp_screens_draw_history_teams_tab`.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_teams_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I3%%);
    compare_history_list_tab("comp history teams tab", object, || o(object, top, bottom), || comp_screens_draw_history_teams_tab(object, top, bottom));
}

/// Comparer for `comp_history_draw_players_tab`.
pub(crate) unsafe extern "C" fn compare_comp_history_draw_players_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I4%%);
    compare_history_list_tab("comp history players tab", object, || o(object, top, bottom), || comp_history_draw_players_tab(object, top, bottom));
}

/// Whether a history screen click only reads and writes params (the view, tab and scroll params): not opening a
/// screen, not the error box of a type without a tab, not the menu bar.
unsafe fn history_click_is_pure(widget: u32) -> bool {
    let w = widget as i16;
    let tab = get_screen_param(1) as u8 as i8;
    let id = comp_click_action_id(widget);
    let data = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget);
    match tab {
        2 => {
            if id == 3 && exe::display_get_widget_data(current_human_display(), widget) != get_screen_param(6) {
                return true;
            }
            if id == 4 {
                let kind = data as u8 as i8;
                if !(1..=HISTORY_LAST_RECORD_TYPE as i8).contains(&kind) {
                    return false;
                }
                if kind != 1 {
                    return true;
                }
            }
        }
        3..=6 if id == tab as i32 + 2 && data != get_screen_param(history_view_param(tab)) => return true,
        7 if tel26_item_id(w) == 9 && tel26_item_value(w) as usize != get_screen_param(0x17) => return true,
        _ => {}
    }
    id == 2 && data != get_screen_param(1)
}

/// Comparer for `comp_history_screen_handle_event`: side by side over the screen manager and the params
/// (`comp_click_compare`) when the click only writes params; opening a screen and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_comp_history_screen_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%);
    let pure = history_click_is_pure(widget);
    comp_click_compare("history handle_event", widget, pure, || o(widget), || comp_history_screen_handle_event(widget))
}

/// Comparer for `comp_history_players_tab_handle_click`: side by side (it only writes params).
pub(crate) unsafe extern "C" fn compare_comp_history_players_tab_handle_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%);
    comp_click_compare("history players_tab_handle_click", widget, true, || o(widget), || comp_history_players_tab_handle_click(widget))
}

/// Comparer for `comp_history_screen_close`: only the original (it frees the list), the params checked afterwards
/// (9 and 0xa cleared when there was a list, else untouched).
pub(crate) unsafe extern "C" fn compare_comp_history_screen_close() {
    let before = (get_screen_param(HISTORY_RECORDS_LIST), get_screen_param(HISTORY_RECORDS_COUNT));
    original_fn::<unsafe extern "C" fn()>(%%I7%%)();
    let plan = if before.0 != 0 { (0, 0) } else { before };
    let got = (get_screen_param(HISTORY_RECORDS_LIST), get_screen_param(HISTORY_RECORDS_COUNT));
    SCOREBOARD.record(got == plan, || format!("comp history screen (close): params 9/0xa {:x?} x plan {:x?}", got, plan));
}

/// Comparer for `comp_history_tab_for_record`: both side by side by value (the answer and the param written), a type
/// outside 1..0x16 (the error box) only the original.
pub(crate) unsafe extern "C" fn compare_comp_history_tab_for_record(kind: u32, out: *mut u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut u32) -> u8>(%%I8%%);
    if !(1..=HISTORY_LAST_RECORD_TYPE as i8).contains(&(kind as u8 as i8)) {
        return o(kind, out);
    }
    let start = if out.is_null() { 0 } else { *out };
    let (mut out_o, mut out_r) = (start, start);
    let ro = o(kind, if out.is_null() { core::ptr::null_mut() } else { &mut out_o });
    let rr = comp_history_tab_for_record(kind, if out.is_null() { core::ptr::null_mut() } else { &mut out_r });
    SCOREBOARD.record(ro == rr && out_o == out_r, || format!("comp_history_tab_for_record({}): {} {:#x} x {} {:#x}", kind as u8, ro, out_o, rr, out_r));
    if !out.is_null() {
        *out = out_o;
    }
    ro
}
