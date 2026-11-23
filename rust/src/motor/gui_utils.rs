use super::*;

/// Adds a text area: an area of one column and `rows` rows with the marks and colour, and (with a text) the text with
/// its links placed in it. Returns 1.
///
/// The area: `Display::add_area` on the current Display (x1, y1, x2, y2, 1 column, `rows` rows, the marks, the colour,
/// no owner). Without a text that is all. Otherwise a local Text gets the text (`Text::set_string`) and each link
/// (`Text::add_link` with the link text and its two values, until a null link text) and is placed by
/// `gui_blocks_place_text`. The original reads as many links as the caller passed; the copy at most `GUI_TEXT_AREA_LINKS`.
/// The original also opens an exception frame for the Text; the copy does not.
/// Original: `gui_add_text_area` (0x5ed070, gui_utils.cpp, cdecl, variadic).
pub(crate) unsafe extern "C" fn gui_add_text_area(a: GuiTextAreaArgs) -> u32 {
    covered!("tel37_4 text area");
    let mut text = [0usize; 2];
    exe::new_text(text.as_mut_ptr(), 0usize);
    let area = gui_blocks_add_area([a.x1 as u32, a.y1 as u32, a.x2 as u32, a.y2 as u32, 1, 0, a.rows as u32, 0, a.marks, a.colour, u32::MAX]);
    if a.text != 0 {
        let t = text.as_mut_ptr() as usize;
        exe::text_set_string(t, a.text as *const u8);
        let mut k = 0;
        // ponytail: at most GUI_TEXT_AREA_LINKS links (the callers pass 0 to 2); widen the record if one passes more
        while k + 2 < a.links.len() && a.links[k] != 0 {
            exe::text_add_link(t, a.links[k] as usize as *const u8, a.links[k + 1], a.links[k + 2] as usize);
            k += 3;
        }
        gui_blocks_place_text(t, a.text, area, a.x1, a.x2, a.rows, a.font, a.text_colour);
    }
    exe::drop_text(text.as_mut_ptr());
    1
}

/// Adds a text area from a Text: like `gui_add_text_area`, with the caller's Text (its string for the width and the
/// '\n' test) instead of a text and links. Without a Text only the area. Returns 1.
/// Original: `gui_add_text_area_from_text` (0x5ed200, gui_utils.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn gui_add_text_area_from_text(x1: i32, y1: i32, x2: i32, y2: i32, marks: u32, colour: u32, rows: i32, font: u32,
    text_colour: u32, text: usize) -> u32 {
    covered!("tel37_4 text area from a Text");
    let area = gui_blocks_add_area([x1 as u32, y1 as u32, x2 as u32, y2 as u32, 1, 0, rows as u32, 0, marks, colour, u32::MAX]);
    if text != 0 {
        gui_blocks_place_text(text, read_usize(text), area, x1, x2, rows, font, text_colour);
    }
    1
}

/// Adds a block of menu buttons: a copy of the `n` button records (sorted by text when asked, the selection bits
/// following their buttons), an area of `columns` × `rows` cells and one button widget per record, the last row
/// centred. Returns 1, or 0 (the box gui_utils.cpp 193) without buttons.
///
/// No buttons (`n` <= 0 or a null array): the error box and 0. The records are copied into a new array; with `sort`,
/// a pass of exchanges puts them in text order (`strcmp` > 0 swaps, `utils_swap_bytes`), and each swap moves the bit
/// of `selected` from one position to the other when only one of the two is set. The area: `Display::add_area`
/// (x1, y1, x2, y2, `columns` columns, `rows` rows, marks 1, colour 0, no owner). For button k: at the start of the
/// last row when fewer than `columns` buttons are left, the buttons get a width of (x2 - 2 × columns - x1 + 3) /
/// columns and are centred ((x2 - (width + 2) × left - x1 + 3) / 2 from x1); a selected button (its bit in
/// `selected`) is a 0x822 widget (colour or `G_GUI_COLOUR_YELLOW`), the others 0x22 (colour or
/// `G_GUI_COLOUR_PALE_GREY`); the font is the record's or `font` for -1; the widget (`Display::add_item` type 2) goes
/// in its column and row of the area with `G_GUI_COLOUR_DARK_BLUE`, size 12, the text, the group and the id; with
/// the centred x the next button moves width + 2 to the right. The array is destroyed. The original also opens an
/// exception frame for the array; the copy does not.
/// Original: `gui_add_menu_buttons` (0x5ed300, gui_utils.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn gui_add_menu_buttons(x1: i32, y1: i32, x2: i32, y2: i32, columns: u32, rows: u32, font: u32, n: i32,
    buttons: usize, selected: u32, sort: u32) -> u32 {
    if n <= 0 || buttons == 0 {
        tel26_error_box(GUI_UTILS_CPP, 0xc1);
        return 0;
    }
    covered!("tel37_4 menu buttons");
    let mut selected = selected;
    let list = gui_blocks_new_buttons(n);
    for k in 0..n as usize {
        gui_blocks_copy_button(list + k * gui_blocks_button::SIZE, buttons + k * gui_blocks_button::SIZE);
    }
    // the dword the colours are built in: the original's local, last written by the sort (else not initialised)
    let mut colour_word = 0u32;
    if sort != 0 && n - 1 > 0 {
        let mut i = 0i32;
        while i < n - 1 {
            let a = list + i as usize * gui_blocks_button::SIZE;
            let mut j = i + 1;
            while j < n {
                let b = list + j as usize * gui_blocks_button::SIZE;
                if gui_blocks_text_after(a + gui_blocks_button::TEXT, b + gui_blocks_button::TEXT) {
                    exe::utils_swap_bytes((a) as usize as *mut i32, (b) as usize as *mut i32, gui_blocks_button::SIZE as u32);
                    let (bi, bj) = (1u32.wrapping_shl(i as u32), 1u32.wrapping_shl(j as u32));
                    if selected & bi != 0 && selected & bj == 0 {
                        selected = selected.wrapping_add(bj.wrapping_sub(bi));
                    } else if selected & bi == 0 && selected & bj != 0 {
                        selected = selected.wrapping_add(bi.wrapping_sub(bj));
                    }
                }
                j += 1;
            }
            i += 1;
        }
        colour_word = (list + n as usize * gui_blocks_button::SIZE) as u32;
    }
    let area = gui_blocks_add_area([x1 as u32, y1 as u32, x2 as u32, y2 as u32, columns, 0, rows, 0, 1, 0, u32::MAX]);
    let per_row = (columns & 0xff) as i32;
    let (mut column, mut row) = (0u32, 0u32);
    let (mut left_x, mut right_x, mut width) = (0i32, 0i32, area as i32);
    let mut remaining = n;
    for k in 0..n {
        let rec = list + k as usize * gui_blocks_button::SIZE;
        if k % per_row == 0 && remaining < per_row {
            width = x2.wrapping_sub(per_row * 2).wrapping_sub(x1).wrapping_add(3) / per_row;
            left_x = (x2.wrapping_sub((width + 2).wrapping_mul(remaining)).wrapping_sub(x1).wrapping_add(3) / 2).wrapping_add(x1);
            right_x = width + left_x - 1;
        }
        let colour = read_u16(rec + gui_blocks_button::COLOUR);
        let flags = if selected & 1u32.wrapping_shl(k as u32) != 0 {
            colour_word = if colour != 0 { gui_blocks_low_word(selected, colour) } else { gui_blocks_low_word(colour_word, read_u16(G_GUI_COLOUR_YELLOW)) };
            0x822
        } else {
            colour_word = if colour != 0 { gui_blocks_low_word(selected, colour) } else { gui_blocks_low_word(colour_word, read_u16(G_GUI_COLOUR_PALE_GREY)) };
            0x22
        };
        let font_word = read_u16(rec + gui_blocks_button::FONT);
        let font_value = if font_word == 0xffff { font } else { gui_blocks_low_word(selected, font_word) };
        let blue = read_u16(G_GUI_COLOUR_DARK_BLUE);
        tel26_add_item(gui_blocks_display(), [2, left_x as u32, 0, right_x as u32, 0, column, row, flags, gui_blocks_low_word(font_value, blue),
            gui_blocks_low_word(colour_word, blue), 0xc, font_value, colour_word, (rec + gui_blocks_button::TEXT) as u32, 0,
            read_u32(rec + gui_blocks_button::GROUP), read_u32(rec + gui_blocks_button::ID), area]);
        if left_x != 0 && right_x != 0 {
            left_x = left_x.wrapping_add(width).wrapping_add(2);
            right_x = width.wrapping_add(right_x).wrapping_add(2);
        }
        column += 1;
        if column == columns & 0xff {
            column = 0;
            row += 1;
        }
        remaining -= 1;
    }
    gui_blocks_delete_buttons(list);
    1
}

/// Adds a tab bar: up to 12 tabs on one or two rows of a strip 0x64..0x316 wide, the selected tab and its group
/// marked. Returns 1, or 0 (the boxes gui_utils.cpp 350, 358, 366).
///
/// The first call makes the tab colour (`graphics_rgb_to_pixel` of the immediates at 0x5ed7ad, 0x5ed7ab, 0x5ed7a7:
/// 0x20, 0, 0x60 in the 800 × 600 exe; the 1080 exe has its own). No tabs: box 350; more than 12: box 358; no top or
/// no bottom y pointer: box 366. The tabs are copied into a new array; the selected group is the group of the tab
/// whose id is `selected_id` (none: the original keeps the raw allocation pointer there). With more than 5 tabs the
/// first row takes (n + 1) / 2 (`odd_first`) or n / 2 of them, else all. Tab 0 opens the top row at `*top_y`: a panel
/// area (marks 2) and the tab area of `per_row` columns (marks 1), 0x23 high, and `*top_y` moves 0x2d down; the first
/// tab of the second row does the same above `*bottom_y` with the rest of the tabs, and `*bottom_y` moves 0x2d up.
/// Each tab is a widget in its column of the current tab area: the selected one 0x830 (colour or
/// `G_GUI_COLOUR_YELLOW`), one of the selected group 0x30 (colour or `G_GUI_COLOUR_PALE_GREY`), the others 0x30
/// (colour or 0xaebde8); type 0x4002 for a tab of another non-zero group, else 2; the font is the record's or 1 for
/// -1; with the tab colour, size 12, the text, the group and the id. The array is destroyed.
/// Original: `gui_add_tab_bar` (0x5ed780, gui_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_add_tab_bar(tabs: usize, n: i32, selected_id: u32, top_y: *mut i32, bottom_y: *mut i32, odd_first: u32) -> u32 {
    if read_u8(GUI_TAB_BAR_COLOUR_ONCE) & 1 == 0 {
        write_u8(GUI_TAB_BAR_COLOUR_ONCE, read_u8(GUI_TAB_BAR_COLOUR_ONCE) | 1);
        // push imm8 (sign-extended): red, green and blue of the tab colour, read from the running exe
        let red = core::ptr::read(0x5ed7ad as *const i8) as i32 as u32;
        let green = core::ptr::read(0x5ed7ab as *const i8) as i32 as u32;
        let blue = core::ptr::read(0x5ed7a7 as *const i8) as i32 as u32;
        write_u16(GUI_TAB_BAR_COLOUR, exe::graphics_rgb_to_pixel(red, green, blue, 0) as u16);
    }
    if tabs == 0 {
        tel26_error_box(GUI_UTILS_CPP, 0x15e);
        return 0;
    }
    if n > 0xc {
        tel26_error_box(GUI_UTILS_CPP, 0x166);
        return 0;
    }
    if top_y.is_null() || bottom_y.is_null() {
        tel26_error_box(GUI_UTILS_CPP, 0x16e);
        return 0;
    }
    covered!("tel37_4 tab bar");
    let raw_list = exe::crt_operator_new((n as u32).wrapping_mul(gui_blocks_button::SIZE as u32).wrapping_add(4));
    let list = if raw_list != 0 {
        write_i32(raw_list, n);
        exe::crt_vector_constructor_iterator(raw_list + 4, gui_blocks_button::SIZE as u32, (n) as i32 as u32, 0x4568f0, 0x5a53c0);
        raw_list + 4
    } else {
        0
    };
    let mut group = raw_list as u32;
    for k in 0..n.max(0) as usize {
        if read_u32(tabs + k * gui_blocks_button::SIZE + gui_blocks_button::ID) == selected_id {
            group = read_u32(tabs + k * gui_blocks_button::SIZE + gui_blocks_button::GROUP);
            break;
        }
    }
    for k in 0..n.max(0) as usize {
        gui_blocks_copy_button(list + k * gui_blocks_button::SIZE, tabs + k * gui_blocks_button::SIZE);
    }
    let per_row = if n > 5 { if odd_first as u8 != 0 { (n + 1) / 2 } else { n / 2 } } else { n };
    // the dword the colours are built in: the original's local, last holding the pass counter (3)
    let mut colour_word = 3u32;
    let mut area = group;
    let mut column = 0u32;
    for k in 0..n.max(0) {
        if k == 0 {
            let y = *top_y;
            gui_blocks_add_area([0x64, y as u32, 0x316, y.wrapping_add(0x23) as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
            let y = *top_y;
            area = gui_blocks_add_area([0x64, y as u32, 0x316, y.wrapping_add(0x23) as u32, per_row as u32, 0, 1, 0, 1, 0, u32::MAX]);
            *top_y = (*top_y).wrapping_add(0x2d);
            column = 0;
        } else if k == per_row {
            let y = *bottom_y;
            gui_blocks_add_area([0x64, y.wrapping_sub(0x23) as u32, 0x316, y as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
            let y = *bottom_y;
            // the column count is a byte (dl = n - per_row; Area::init and the packet read the byte)
            let rest = (n as u8).wrapping_sub(per_row as u8) as u32;
            area = gui_blocks_add_area([0x64, y.wrapping_sub(0x23) as u32, 0x316, y as u32, rest, 0, 1, 0, 1, 0, u32::MAX]);
            *bottom_y = (*bottom_y).wrapping_sub(0x2d);
            column = 0;
        }
        let rec = list + k as usize * gui_blocks_button::SIZE;
        let colour = read_u16(rec + gui_blocks_button::COLOUR);
        let tab_group = read_u32(rec + gui_blocks_button::GROUP);
        let flags = if read_u32(rec + gui_blocks_button::ID) == selected_id {
            colour_word = if colour != 0 { gui_blocks_low_word(selected_id, colour) } else { gui_blocks_low_word(colour_word, read_u16(G_GUI_COLOUR_YELLOW)) };
            0x800
        } else if tab_group == group {
            colour_word = if colour != 0 { gui_blocks_low_word(group, colour) } else { gui_blocks_low_word(colour_word, read_u16(G_GUI_COLOUR_PALE_GREY)) };
            0
        } else {
            colour_word = if colour != 0 { gui_blocks_low_word(group, colour) } else { gui_blocks_low_word(colour_word, read_u16(JOB_COLOUR_BDE8)) };
            0
        };
        let font_word = read_u16(rec + gui_blocks_button::FONT);
        let font = if font_word == 0xffff { 1 } else { gui_blocks_low_word(bottom_y as u32, font_word) };
        let kind = if tab_group != 0 && tab_group != group { 0x4002 } else { 2 };
        let tab_colour = read_u16(GUI_TAB_BAR_COLOUR);
        tel26_add_item(gui_blocks_display(), [kind, 0, 0, 0, 0, column, 0, flags | 0x30, gui_blocks_low_word(colour_word, tab_colour),
            gui_blocks_low_word(tab_group, tab_colour), 0xc, font, colour_word, (rec + gui_blocks_button::TEXT) as u32, 0, tab_group,
            read_u32(rec + gui_blocks_button::ID), area]);
        column += 1;
    }
    gui_blocks_delete_buttons(list);
    1
}

/// Comparer for `gui_add_text_area`: the display list side by side (one area, up to 64 widgets).
pub(crate) unsafe extern "C" fn compare_gui_add_text_area(a: GuiTextAreaArgs) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(GuiTextAreaArgs) -> u32>(IDX_GUI_ADD_TEXT_AREA);
    compare_gui_blocks(64, 1, &[], || format!("gui_add_text_area ({}, {})-({}, {}), {} rows", a.x1, a.y1, a.x2, a.y2, a.rows), || o(a), || gui_add_text_area(a))
}

/// Comparer for `gui_add_text_area_from_text`: only the original (the caller's Text may be changed by the layout,
/// so it does not run twice); afterwards the area count must have gone up by one or the overflow flag be set.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_gui_add_text_area_from_text(x1: i32, y1: i32, x2: i32, y2: i32, marks: u32, colour: u32, rows: i32, font: u32,
    text_colour: u32, text: usize) -> u32 {
    type F = unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, i32, u32, u32, usize) -> u32;
    let d = gui_blocks_display();
    let before = read_i16(d + 0x12e99e);
    let r = original_fn::<F>(IDX_GUI_ADD_TEXT_AREA_FROM_TEXT)(x1, y1, x2, y2, marks, colour, rows, font, text_colour, text);
    let (after, full) = (read_i16(d + 0x12e99e), read_u32(d + 0x12f54d));
    SCOREBOARD.record(r == 1 && (after == before.wrapping_add(1) || full != 0), || format!("gui_add_text_area_from_text ({}, {})-({}, {}): returned {} areas {} -> {}",
        x1, y1, x2, y2, r, before, after));
    r
}

/// Comparer for `gui_add_menu_buttons`: the display list side by side (one area, a widget per button).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_gui_add_menu_buttons(x1: i32, y1: i32, x2: i32, y2: i32, columns: u32, rows: u32, font: u32, n: i32,
    buttons: usize, selected: u32, sort: u32) -> u32 {
    type F = unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, i32, usize, u32, u32) -> u32;
    let o = original_fn::<F>(IDX_GUI_ADD_MENU_BUTTONS);
    if n <= 0 || buttons == 0 {
        return o(x1, y1, x2, y2, columns, rows, font, n, buttons, selected, sort);
    }
    compare_gui_blocks(n as usize, 1, &[], || format!("gui_add_menu_buttons ({} buttons, {} columns, sort {})", n, columns as u8, sort),
        || o(x1, y1, x2, y2, columns, rows, font, n, buttons, selected, sort),
        || gui_add_menu_buttons(x1, y1, x2, y2, columns, rows, font, n, buttons, selected, sort))
}

/// Comparer for `gui_add_tab_bar`: the display list (up to four areas, a widget per tab), both y pointers and the
/// tab colour side by side.
pub(crate) unsafe extern "C" fn compare_gui_add_tab_bar(tabs: usize, n: i32, selected_id: u32, top_y: *mut i32, bottom_y: *mut i32, odd_first: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, u32, *mut i32, *mut i32, u32) -> u32>(IDX_GUI_ADD_TAB_BAR);
    if tabs == 0 || n > 0xc || top_y.is_null() || bottom_y.is_null() || read_u8(GUI_TAB_BAR_COLOUR_ONCE) & 1 == 0 {
        // the error boxes and the first call (the colour is made once) only the original
        return o(tabs, n, selected_id, top_y, bottom_y, odd_first);
    }
    let extra = [(top_y as usize, 4), (bottom_y as usize, 4), (GUI_TAB_BAR_COLOUR_ONCE, 4)];
    compare_gui_blocks(n.max(0) as usize, 4, &extra, || format!("gui_add_tab_bar ({} tabs, selected {:#x})", n, selected_id),
        || o(tabs, n, selected_id, top_y, bottom_y, odd_first), || gui_add_tab_bar(tabs, n, selected_id, top_y, bottom_y, odd_first))
}

/// Rule: the Back and Next buttons are a two-cell area at the bottom of the screen; a button that cannot be used is
/// shown as grey text instead.
///
/// An area on the current slot's `Display` (`Display::add_area`: (100, 555)–(790, 590), 2 cells, the cell layout
/// bytes {3, 1} of a local frame, 1, 0, 1, 0, no owner). In cell 0, "Back" (translated, `FN_LANGLIB_SPRINTF`): with
/// `back` an area button (`ScreenManager::add_area_button`: flags 0x30, `G_GUI_COLOUR_GREY`, font 0xc, style 3,
/// `G_GUI_COLOUR_PALE_GREY`, id −2, data 0), otherwise an area text in font 0x2c. In cell 1 the same with "Next" and id
/// −3. Returns 1. The colours go as words (the original's high halves are leftovers; `Guio::init` keeps words).
/// Original: `gui_add_back_next_buttons` (0x5edcc0, gui_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_add_back_next_buttons(back: u32, next: u32) -> u32 {
    let add_area: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32, u32) -> u16 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_item: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> u16 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut frame = [0u8; 0x7d4];
    frame[..4].copy_from_slice(&[1, 1, 3, 1]);
    let layout = frame.as_ptr() as usize + 2;
    let text = frame.as_mut_ptr() as usize + 4;
    let area = add_area(screens_current_display(SCREEN_MANAGER), 0x64, 0x22b, 0x316, 0x24e, 2, layout as u32, 1, 0, 1, 0, u32::MAX);
    let grey = read_u16(G_GUI_COLOUR_GREY) as u32;
    let pale = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    for (cell, label, enabled, id) in [(0u32, 0x9880a8usize, back, -2i32), (1, 0x9880a0, next, -3)] {
        format(text, label);
        if enabled != 0 {
            button(SCREEN_MANAGER, area, cell, 0, 0x30, grey, 0xc, 3, pale, text, id as u32, 0); // tel38_3: the usable button
        } else {
            covered!("tel38_3 back or next greyed");
            text_item(SCREEN_MANAGER, area, cell, 0, 0x30, grey, 0x2c, 3, pale, text);
        }
    }
    1
}

/// Comparer for `gui_add_back_next_buttons`: side by side over the display list (one area, two items) and the return.
pub(crate) unsafe extern "C" fn compare_gui_add_back_next_buttons(back: u32, next: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(IDX_GUI_ADD_BACK_NEXT_BUTTONS);
    compare_gui_build("gui_add_back_next_buttons", 1, 2, || o(back, next), || gui_add_back_next_buttons(back, next))
}

/// Rule: the page buttons of a list, right to left from `right`: last page, next page, the "Pg page/max" text, previous
/// page and first page; the first and last buttons only when they have an id, and the buttons that lead nowhere carry
/// id 0.
///
/// Each button is 0x14 wide with 2 pixels between them (`gui_page_button`; the symbols 0x2f, 0x2b, 0x2a, 0x2e or the
/// texts ">>", ">", "<", "<<"). ">>" (when `last_id` ≠ 0) and ">" carry their ids while `page` < `max` (signed words),
/// "<" and "<<" (when `first_id` ≠ 0) while `page` > 1. The text: its width is that of "Pg max/max"
/// (`FN_LANGLIB_SPRINTF` into the scratch text, `exe::font_text_width` of font 2, + 10); then "Pg page/max", a text
/// item (`ScreenManager::add_text_item`: flags 2, colour 0, font 0xc, style 2, `G_GUI_COLOUR_YELLOW`) ending 2 pixels
/// left of ">"; "<" starts 2 pixels left of it and "<<" one button further. Returns the left edge of the "<<" slot
/// (whether or not it is added).
/// Original: `gui_add_page_buttons` (0x5ede20, gui_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_add_page_buttons(right: i32, y: i32, page: u32, max: u32, first_id: u32, prev_id: u32, next_id: u32,
    last_id: u32) -> i32 {
    let (page, max) = (page as i16, max as i16);
    let mut x = right;
    if last_id != 0 {
        gui_page_button(x, y, 0x2f, 0x98e7a8, if page >= max { 0 } else { last_id });
        x -= 0x16;
    }
    gui_page_button(x, y, 0x2b, 0x9cdc54, if page >= max { 0 } else { next_id }); // tel38_3: next page while not at the end
    x -= 0x16;
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    format(GUI_SCRATCH_TEXT, 0x9cdc30, max as i32, max as i32);
    let width = exe::font_text_width(2usize, GUI_SCRATCH_TEXT as *const u8) + 10;
    format(GUI_SCRATCH_TEXT, 0x9cdc30, page as i32, max as i32);
    let text_item: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, u32, u32, u32, u32, u32, usize, u32) -> u16 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let left = x - width;
    text_item(SCREEN_MANAGER, left, y, x, y + 0x14, 2, 0, 0xc, 2, read_u16(G_GUI_COLOUR_YELLOW) as u32, GUI_SCRATCH_TEXT, 1);
    x = left - 2;
    gui_page_button(x, y, 0x2a, 0x9cdc2c, if page <= 1 { 0 } else { prev_id });
    x -= 0x16;
    if first_id != 0 {
        covered!("tel38_3 first page button");
        gui_page_button(x, y, 0x2e, 0x98e7c4, if page <= 1 { 0 } else { first_id });
    }
    x - 0x14
}

/// Comparer for `gui_add_page_buttons`: side by side over the display list (up to five items) and the return.
pub(crate) unsafe extern "C" fn compare_gui_add_page_buttons(right: i32, y: i32, page: u32, max: u32, first_id: u32, prev_id: u32, next_id: u32,
    last_id: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i32, i32, u32, u32, u32, u32, u32, u32) -> i32>(IDX_GUI_ADD_PAGE_BUTTONS);
    compare_gui_build("gui_add_page_buttons", 0, 5, || o(right, y, page, max, first_id, prev_id, next_id, last_id) as u32,
        || gui_add_page_buttons(right, y, page, max, first_id, prev_id, next_id, last_id) as u32) as i32
}

/// The Display of the screen manager's current channel.
pub(crate) unsafe fn gui_blocks_display() -> usize {
    read_usize(screen_record(G_SCREEN_MANAGER) + 0x7fc)
}

/// `Display::add_area` (0x55fba0) by address on the current Display; returns eax (the area index in ax).
pub(crate) unsafe fn gui_blocks_add_area(f: [u32; 11]) -> u32 {
    tel26_add_area(gui_blocks_display(), f)
}

/// Puts a text into a text area's grid (the tail of `gui_add_text_area` and `gui_add_text_area_from_text`): a text
/// that fits in one line (its width in the font at most x2 - x1 - 9 and no '\n') goes centred on the middle row
/// ((rows - 1) / 2); otherwise from row 0 over the `rows` lines; both between x1 + 5 and x2 - 5.
pub(crate) unsafe fn gui_blocks_place_text(text: usize, string: usize, area: u32, x1: i32, x2: i32, rows: i32, font: u32, colour: u32) {
    if exe::font_text_width(font as usize, string as *const u8) <= x2.wrapping_sub(x1).wrapping_sub(9) && exe::at_946540((string) as usize as *const u8, 10) == 0 {
        let mut row = rows.wrapping_sub(1) / 2;
        gui_blocks_text_to_area(text, area, x1.wrapping_add(5), x2.wrapping_sub(5), &mut row, font, colour, 1, 0);
    } else {
        let mut row = 0i32;
        gui_blocks_text_to_area(text, area, x1.wrapping_add(5), x2.wrapping_sub(5), &mut row, font, colour, 0, rows as u32);
    }
}

/// A new array of `n` menu button records (operator new with the count before them, each built by the
/// `MenuButton` constructor through `vector_constructor_iterator`); 0 when the allocation fails.
pub(crate) unsafe fn gui_blocks_new_buttons(n: i32) -> usize {
    let raw = exe::crt_operator_new((n as u32).wrapping_mul(gui_blocks_button::SIZE as u32).wrapping_add(4));
    if raw == 0 {
        return 0;
    }
    write_i32(raw, n);
    exe::crt_vector_constructor_iterator(raw + 4, gui_blocks_button::SIZE as u32, (n) as i32 as u32, 0x4568f0, 0x5a53c0);
    raw + 4
}

/// Destroys an array from `gui_blocks_new_buttons` (when not null): the destructors and operator delete.
pub(crate) unsafe fn gui_blocks_delete_buttons(buttons: usize) {
    if buttons == 0 {
        return;
    }
    exe::crt_vector_destructor_iterator(buttons, gui_blocks_button::SIZE as u32, (read_i32(buttons - 4)) as i32 as u32, 0x5a53c0);
    exe::crt_operator_delete(buttons - 4);
}

/// Copies a 0xd4-byte button record.
pub(crate) unsafe fn gui_blocks_copy_button(to: usize, from: usize) {
    core::ptr::copy(from as *const u8, to as *mut u8, gui_blocks_button::SIZE);
}

/// The inline `strcmp(a, b) > 0` of the original (bytes compared unsigned).
pub(crate) unsafe fn gui_blocks_text_after(a: usize, b: usize) -> bool {
    let mut i = 0;
    loop {
        let (x, y) = (read_u8(a + i), read_u8(b + i));
        if x != y {
            return x > y;
        }
        if x == 0 {
            return false;
        }
        i += 1;
    }
}

/// The one-word value whose low word is `low` and whose high word is `high`'s (the original moves words into
/// dword registers that held something else).
pub(crate) fn gui_blocks_low_word(high: u32, low: u16) -> u32 {
    (high & 0xffff_0000) | low as u32
}

/// Side by side over the display list a builder changes (`gui_blocks_regions`, plus `extra`); near a table's end only
/// the original.
pub(crate) unsafe fn compare_gui_blocks(items: usize, areas: usize, extra: &[(usize, usize)], what: impl Fn() -> String, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let Some(mut reg) = gui_blocks_regions(items, areas) else {
        // simplification: near the end of the tables (error boxes) only the original
        return original();
    };
    reg.extend_from_slice(extra);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, original, rust);
    SCOREBOARD.record(same && r == n, || format!("{}: original={} rust={} differences {:?}", what(), r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of a gui builder: side by side over `gui_build_regions` and the return.
pub(crate) unsafe fn compare_gui_build(name: &str, areas: usize, items: usize, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let regions = gui_build_regions(areas, items);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}: return {:#x} x {:#x}, differences in {:?}", name, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// One page button of `gui_add_page_buttons`: 0x14 × 0x14 with its right edge at `right`, at `y`; with the symbol font
/// (`G_FONT_MODE` ≠ −1) the symbol `code` ("%c" into the scratch text, style 0), otherwise the text `plain` (style 1).
pub(crate) unsafe fn gui_page_button(right: i32, y: i32, code: u32, plain: usize, id: u32) {
    let button: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, u32, u32, u32, u32, u32, usize, u32, u32) -> u16 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let (text, style) = if read_i32(G_FONT_MODE) != -1 {
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        format(GUI_SCRATCH_TEXT, 0x99ec1c, code);
        (GUI_SCRATCH_TEXT, 0)
    } else {
        (plain, 1)
    };
    button(SCREEN_MANAGER, right - 0x14, y, right, y + 0x14, 0x30, read_u16(G_GUI_COLOUR_DARK_BLUE) as u32, 0xc, style,
        read_u16(G_GUI_COLOUR_PALE_GREY) as u32, text, id, 0);
}

/// `Text::add_to_area` (0x4aeaf0, thiscall, ret 0x20): the Text laid out as widgets of an area between x1 and x2 from
/// the row `*row`, with the font and colour; `centred` and the line count as the last two.
pub(crate) unsafe fn gui_blocks_text_to_area(text: usize, area: u32, x1: i32, x2: i32, row: *mut i32, font: u32, colour: u32, centred: u32, lines: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) = core::mem::transmute(FN_TEXT_DRAW_WRAPPED);
    f(text, area, x1, x2, row, font, colour, centred, lines)
}

/// The display list regions a builder may change on the current Display, for up to `items` new widgets and `areas`
/// new areas; `None` when that could reach a table's end (an error box).
pub(crate) unsafe fn gui_blocks_regions(items: usize, areas: usize) -> Option<Vec<(usize, usize)>> {
    let d = gui_blocks_display();
    let (count, area_count) = (read_i16(d + display_list::ITEM_COUNT), read_i16(d + 0x12e99e));
    let (item_order, area_order) = (read_i16(d + display_list::ITEM_ORDER_COUNT), read_i16(d + display_list::AREA_ORDER_COUNT));
    if d == 0 || count < 0 || area_count < 0 || item_order < 0 || area_order < 0 || count as usize + items >= 0x4af
        || area_count as usize + areas >= 0xf9 || item_order as usize + items >= 0x4b0 || area_order as usize + areas >= 0xfa {
        return None;
    }
    let pos = read_u32(DISPLAY_PACKET_BUFFER + network_buffer::POS);
    let room = read_u32(DISPLAY_PACKET_BUFFER + network_buffer::CAPACITY).saturating_sub(pos).min(0x400) as usize;
    Some(vec![(d + display_list::ITEM_COUNT, 2), (d + 0x12e99e, 2), (d + display_list::ITEM_ORDER_COUNT, 2), (d + display_list::AREA_ORDER_COUNT, 2),
              (d + display_list::OVERFLOW, 4), (d + 0x12f54d, 4), (DISPLAY_PACKET_BUFFER, 12), (network_buffer_cursor(DISPLAY_PACKET_BUFFER), room),
              (display_item_at(d, count), items * display_list::ITEM_SIZE), (display_area_at(d, area_count), areas * display_list::AREA_SIZE + 0xd9),
              (d + display_list::ITEM_ORDER + item_order as usize * 2, items * 2), (d + display_list::AREA_ORDER + area_order as usize * 2, areas * 2)])
}

/// The display-list writes of a gui builder: what `screens_forwarder_regions` (tel01_1) covers, the next `areas` area
/// slots and `items` item slots, both draw orders (counts and next slots) and the scratch text.
pub(crate) unsafe fn gui_build_regions(areas: usize, items: usize) -> Vec<(usize, usize)> {
    use display_list as l;
    let mut r = screens_forwarder_regions(SCREEN_MANAGER);
    r.push((GUI_SCRATCH_TEXT, 0x100));
    let d = screens_current_display(SCREEN_MANAGER);
    if d == 0 {
        return r;
    }
    let a = read_i16(d + l::AREA_ORDER_COUNT);
    let n = read_i16(d + l::ITEM_ORDER_COUNT);
    r.push((d + l::AREA_ORDER_COUNT, 4));
    if (0..250 - areas as i16).contains(&a) {
        r.push((d + l::AREA_ORDER + a as usize * 2, areas * 2));
    }
    if (0..0x4b0 - items as i16).contains(&n) {
        r.push((d + l::ITEM_ORDER + n as usize * 2, items * 2));
    }
    let ac = read_i16(d + 0x12e99e);
    if (0..0xf9 - areas as i16).contains(&ac) {
        r.push((display_area_at(d, ac), areas * l::AREA_SIZE));
    }
    let ic = read_i16(d + l::ITEM_COUNT);
    if (0..0x4af - items as i16).contains(&ic) {
        r.push((display_item_at(d, ic), items * l::ITEM_SIZE));
    }
    r
}

/// "E:\dev\CM3\cm3 00-01\si\code\gui_utils.cpp" (the error boxes).
pub(crate) const GUI_UTILS_CPP: usize = 0x9cdc00;

/// The tab bar's colour (u16) and the bit 0 of its once flag (the colour is made on the first call).
pub(crate) const GUI_TAB_BAR_COLOUR_ONCE: usize = 0xb0be34;

pub(crate) const GUI_TAB_BAR_COLOUR: usize = 0xb0be36;

/// The arguments of `gui_add_text_area` (cdecl, variadic): the fixed ten, then the links (text, two values) ending
/// in a null text. Passed by value, the record is the caller's argument area itself.
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct GuiTextAreaArgs {
    pub(crate) x1: i32,
    pub(crate) y1: i32,
    pub(crate) x2: i32,
    pub(crate) y2: i32,
    pub(crate) marks: u32,
    pub(crate) colour: u32,
    pub(crate) rows: i32,
    pub(crate) font: u32,
    pub(crate) text_colour: u32,
    pub(crate) text: usize,
    pub(crate) links: [u32; 3 * GUI_TEXT_AREA_LINKS + 1],
}

/// The game's scratch text (`0xde1f64`, 0x100 bytes used here) and the Display flag that lets a widget redraw at once
/// [inference].
pub(crate) const GUI_SCRATCH_TEXT: usize = 0xde1f64;

/// The most links `gui_add_text_area` reads after the text (the callers pass at most 2).
pub(crate) const GUI_TEXT_AREA_LINKS: usize = 8;
