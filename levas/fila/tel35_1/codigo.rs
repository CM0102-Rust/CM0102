// ---------- batch %%LEVA%%: the Discipline Player screen (Fine.cpp) ----------

/// The screen manager's network flag (`[0xb77c84]`): set, the screens of a network game answer every widget with -9.
const TEL35_NETWORK_FLAG: usize = 0xb77c84;
/// Fine.cpp's discipline texts: "Discipline Player", "Official Warning", "<%d - Fine period> Week Fine" and the
/// language-10 variants (a plain "%d" week text and a fixed one-week text).
const DISCIPLINE_TITLE: usize = 0x9cc0e8;
const DISCIPLINE_WARNING: usize = 0x9cc060;
const DISCIPLINE_WEEK_FINE: usize = 0x9cc01c;
const DISCIPLINE_WEEK_FINE_L10: usize = 0x9cc03c;
const DISCIPLINE_ONE_WEEK_L10: usize = 0x9cc050;

/// `Text::draw_wrapped(slot, left, right, &line, font, colour, style, flags)` (0x4aeaf0, thiscall, ret 0x20): lays a
/// Text's words out as area rows from `line` on, advancing it.
pub(crate) unsafe fn tel35_text_draw_wrapped(text: usize, area: u32, left: i32, right: i32, line: *mut i32, font: u32, colour: u32, style: u32, flags: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x4aeaf0usize);
    f(text, area, left, right, line, font, colour, style, flags)
}

/// A colour word zero-extended (the original pushes a register with only its low half loaded).
pub(crate) unsafe fn tel35_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

/// The person of screen parameter 0 (a staff id): the staff table entry.
unsafe fn discipline_person() -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((get_screen_param(0) as u32).wrapping_mul(staff::SIZE as u32) as usize)
}

/// The fine period's text into the screens' text buffer: 1 "Official Warning", otherwise "<n> Week Fine" with
/// n = period − 1 (langlib).
unsafe fn discipline_period_text(period: i32) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if period == 1 {
        text(JOB_TEXT, DISCIPLINE_WARNING);
    } else {
        text(JOB_TEXT, DISCIPLINE_WEEK_FINE, period - 1);
    }
}

/// Rule: the Discipline Player screen opens for a person with the fine period at 1 (an official warning) and the reason
/// at 0.
///
/// `open_discipline_player_screen(&person)`: `open_screen` with the builder 0x5a83e0, the handler 0x5a8990, single
/// instance, the no-op leave routine 0x5a8f60 and 0; when it opens, parameter 0 = the person's id (the first dword of
/// the record the argument points to), 1 = 1 (the period) and 3 = 0 (the reason).
/// Original: `open_discipline_player_screen` (0x5a8380, Fine.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_discipline_player_screen(person: usize) {
    covered!("tel35_1 open discipline");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5a83e0usize, 0x5a8990usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    set_screen_param(0, read_u32(person) as usize);
    set_screen_param(1, 1);
    set_screen_param(3, 0);
}

/// Rule: the Discipline Player screen shows the person's name in his club's colours, the sentence with a link to the
/// person, the period chooser ("-", the period, "+"; "-" greyed at a warning, "+" greyed at two weeks), the five
/// reasons (the chosen one in yellow) and Cancel / Confirm.
///
/// `draw_discipline_player_screen()`: the person of parameter 0 (`staff::SIZE` × id into the staff table); with no club
/// (`staff::CLUB_JOB`) nothing is built. Otherwise the period (parameter 1) and, as the reason, the low byte of
/// parameter 2 (the original reads 2, where this routine keeps the "-" widget, not 3 where the handler keeps the
/// reason: a defect kept). The person's background picture (`background_set_staff_picture`, 0, 0); the title: his name
/// (`database_format_staff_name`, 500 bytes, style 1) in the club's kit colours (kit 1); the subtitle "Discipline
/// Player". Area 1 (0x6e, 0x8c)-(0x30c, 0x221), seven columns (2, 20, 2, 10, 2, 4, 3), rows = 0x195 / the height of
/// font 3, kind 2: the sentence with the person's name (a Text with the link 0x7d7 to the person) drawn wrapped from
/// row 0 (font 3, pale grey); two rows below, "Disciplinary action:" (column 1), "-" in column 2 (a grey text at period
/// ≤ 1, otherwise a button id 2), its widget kept in parameter 2, the period's text in column 3 ("Official Warning", or
/// "<n> Week Fine"; in language 10 a fixed text for period 2 and a plain "%d" one otherwise), "+" in column 4 (a grey
/// text at period ≥ 3, otherwise a button id 1); two rows below "Reason For Disciplinary Action:". Area 2, the same
/// rectangle, two columns (2, 17), kind 1: from two rows below, the five reasons (`Fine::reason_text` k, 2000 bytes)
/// as buttons id 3 with data k, the chosen one yellow and the others grey. The menu bar (4, 0) and the button area
/// (0x64, 0x22b)-(0x316, 0x24e) (columns 3, 1; the club's background colour) with "Cancel" (id 4) and "Confirm" (id 5).
/// Original: `draw_discipline_player_screen` (0x5a83e0, Fine.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_discipline_player_screen() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let reason_text: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x5a73a0usize);
    let columns: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
    let buttons: [u8; 4] = [3, 1, 2, 0x11];
    let mut tx = [0u32; 8];
    let t = tx.as_mut_ptr() as usize;
    exe::new_text(tx.as_mut_ptr() as *mut usize, 0usize);
    let person = discipline_person();
    let club = read_usize(person + staff::CLUB_JOB);
    if club == 0 {
        exe::drop_text(tx.as_mut_ptr() as *mut usize);
        return;
    }
    covered!("tel35_1 draw discipline");
    let period = get_screen_param(1) as u8 as i8 as i32;
    let reason = get_screen_param(2) as u8;
    let background: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c790usize);
    background(person, 0, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    let mut name = [0u8; 500];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x1f4, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back & 0xffff, fore & 0xffff, name.as_ptr() as usize);
    text(JOB_TEXT, DISCIPLINE_TITLE);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x195 / exe::font_get_height(3)) as u8 as u32;
    let d = current_human_display();
    let area = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 7, columns.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    text(JOB_TEXT, 0x9cc08c, 0x64, name.as_ptr());
    exe::text_set_string(t, JOB_TEXT as *const u8);
    exe::text_add_link(t, name.as_ptr(), 0x7d7, person);
    let mut line = 0i32;
    tel35_text_draw_wrapped(t, area, 0x6e, 0x30c, &mut line, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), 0, 0);
    let mut row = line + 2;
    let pale = tel35_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = tel35_colour(G_GUI_COLOUR_GREY);
    let blue = tel35_colour(G_GUI_COLOUR_DARK_BLUE);
    text(JOB_TEXT, 0x9cc074);
    tel26_area_button([area, 1, row as u32, 1, 0, 1, 3, pale, JOB_TEXT as u32, 0, 0]);
    let minus = if period <= 1 {
        tel26_area_text([area, 2, row as u32, 0x30, blue, 0xc, 3, grey, 0x98ba0c])
    } else {
        tel26_area_button([area, 2, row as u32, 0x30, blue, 0xc, 3, pale, 0x98ba0c, 2, 0])
    };
    set_screen_param(2, minus as i16 as isize as usize);
    if period == 1 {
        text(JOB_TEXT, DISCIPLINE_WARNING);
    } else if exe::langlib_get_language() == 0xa {
        if period == 2 {
            tel26_copy_text(JOB_TEXT as *mut u8, DISCIPLINE_ONE_WEEK_L10 as *const u8);
        } else {
            format(JOB_TEXT, DISCIPLINE_WEEK_FINE_L10, period - 1);
        }
    } else {
        text(JOB_TEXT, DISCIPLINE_WEEK_FINE, period - 1);
    }
    tel26_area_text([area, 3, row as u32, 1, tel35_colour(G_GUI_COLOUR_BLACK), 0xc, 3, pale, JOB_TEXT as u32]);
    if period >= 3 {
        tel26_area_text([area, 4, row as u32, 0x30, blue, 0xc, 3, grey, 0x9afe7c]);
    } else {
        tel26_area_button([area, 4, row as u32, 0x30, blue, 0xc, 3, pale, 0x9afe7c, 1, 0]);
    }
    row += 2;
    text(JOB_TEXT, 0x9cbffc);
    tel26_area_button([area, 1, row as u32, 1, 0, 1, 3, pale, JOB_TEXT as u32, 0, 0]);
    row += 2;
    let reasons = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 2, buttons.as_ptr().wrapping_add(2) as u32, rows, 0, 1, 0, u32::MAX]);
    for k in 0..5u8 {
        reason_text(read_usize(G_FINE), k as u32, JOB_TEXT, 0x7d0, 0);
        let colour = if reason != k { grey } else { tel35_colour(G_GUI_COLOUR_YELLOW) };
        tel26_area_button([reasons, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 3, k as u32]);
        row += 1;
    }
    exe::menubar_build(4, 0);
    let d = current_human_display();
    let bar = tel26_add_area(d, [0x64, 0x22b, 0x316, 0x24e, 2, buttons.as_ptr() as u32, 1, 0, 1, back & 0xffff, u32::MAX]);
    text(JOB_TEXT, 0x98a1a0);
    tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 4, 0]);
    text(JOB_TEXT, 0x98a198);
    tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 5, 0]);
    exe::drop_text(tx.as_mut_ptr() as *mut usize);
}

/// Rule: on the Discipline Player screen "-" and "+" move the period between a warning and a two-week fine (greying
/// the button at the end), a reason click colours it yellow, Confirm fines the person and closes, Cancel closes.
///
/// `discipline_player_screen_input(item)`: in a network game (`[0xb77c84]`) the pending command argument is cleared and
/// -9 comes back. The widget's id (`tel26_item_id`): 2 ("-"): the period (parameter 1) goes down while above 1 and is
/// written back; the "-" widget (parameter 2) gets flags 6, grey and data 0 when the period reaches 1; the period text
/// (the next widget) is rewritten; "+" (the one after) gets flags 2, pale grey and id 1; redraw, 0. 1 ("+"): the period
/// goes up while below 3; "-" gets flags 2, pale grey and id 2; the text; "+" gets flags 6, grey and data 0 when it
/// reaches 3; redraw, 0. 3 (a reason): the widget's data k; the five reason widgets (parameter 2 + 4 ...) recoloured,
/// yellow the chosen one (k above 3 selects the fifth) and grey the others; parameter 3 = k; redraw, 0. 5 (Confirm):
/// `Fine::fine_player(person, period, reason)`, -9. 4 (Cancel): -9. Otherwise the menu bar's command, then its object
/// link.
/// Original: `discipline_player_screen_input` (0x5a8990, Fine.cpp, cdecl).
pub(crate) unsafe extern "C" fn discipline_player_screen_input(raw: u32) -> i32 {
    let person = discipline_person();
    let mut period = get_screen_param(1) as i32;
    let reason = get_screen_param(3);
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let item = raw as i16;
    let sm = G_SCREEN_MANAGER;
    match tel26_item_id(item) {
        2 => {
            covered!("tel35_1 discipline minus");
            if period > 1 {
                period -= 1;
            }
            set_screen_param(1, period as usize);
            let mut w = get_screen_param(2) as u32;
            if period <= 1 {
                exe::screenmanager_set_widget_flags(sm, w as usize, 6);
                exe::screenmanager_set_widget_text_colour(sm, w, tel35_colour(G_GUI_COLOUR_GREY));
                exe::screenmanager_set_widget_data(sm, w, 0usize);
            }
            w = w.wrapping_add(1);
            discipline_period_text(period);
            exe::screenmanager_set_widget_text(sm, w, JOB_TEXT);
            w = w.wrapping_add(1);
            exe::screenmanager_set_widget_flags(sm, w as usize, 2);
            exe::screenmanager_set_widget_text_colour(sm, w, tel35_colour(G_GUI_COLOUR_PALE_GREY));
            exe::screenmanager_set_widget_id(sm, w, 1);
        }
        1 => {
            covered!("tel35_1 discipline plus");
            if period < 3 {
                period += 1;
            }
            set_screen_param(1, period as usize);
            let mut w = get_screen_param(2) as u32;
            exe::screenmanager_set_widget_flags(sm, w as usize, 2);
            exe::screenmanager_set_widget_text_colour(sm, w, tel35_colour(G_GUI_COLOUR_PALE_GREY));
            exe::screenmanager_set_widget_id(sm, w, 2);
            w = w.wrapping_add(1);
            discipline_period_text(period);
            exe::screenmanager_set_widget_text(sm, w, JOB_TEXT);
            w = w.wrapping_add(1);
            if period >= 3 {
                exe::screenmanager_set_widget_flags(sm, w as usize, 6);
                exe::screenmanager_set_widget_text_colour(sm, w, tel35_colour(G_GUI_COLOUR_GREY));
                exe::screenmanager_set_widget_data(sm, w, 0usize);
            }
        }
        3 => {
            covered!("tel35_1 discipline reason");
            let k = tel26_item_value(item);
            let chosen = k.min(4);
            let first = (get_screen_param(2) as u32).wrapping_add(4);
            for j in 0..5u32 {
                let colour = if j == chosen { G_GUI_COLOUR_YELLOW } else { G_GUI_COLOUR_GREY };
                exe::screenmanager_set_widget_text_colour(sm, first.wrapping_add(j), tel35_colour(colour));
            }
            set_screen_param(3, k as usize);
        }
        5 => {
            covered!("tel35_1 discipline confirm");
            exe::fine_fine_player(read_usize(G_FINE), person, period as u32, reason as u32);
            return -9;
        }
        4 => return -9,
        _ => return manager_menu_bar(raw),
    }
    exe::screenmanager_redraw_screen(sm, 0, 0);
    0
}

/// Comparer for `open_discipline_player_screen`.
pub(crate) unsafe extern "C" fn compare_open_discipline_player_screen(person: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(person)
}

/// Comparer for `draw_discipline_player_screen`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_discipline_player_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Discipline Player screen (draw_discipline_player_screen)", &[], || o(), || draw_discipline_player_screen());
}

/// Comparer for `discipline_player_screen_input`.
pub(crate) unsafe extern "C" fn compare_discipline_player_screen_input(item: u32) -> i32 {
    // simplification: the fine, the redraw and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}
