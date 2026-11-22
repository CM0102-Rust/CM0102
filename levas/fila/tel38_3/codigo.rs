// ---------- batch %%LEVA%%: the Back/Next and page buttons and the widget object: its fields, highlight, text, images and colour format (gui_utils.cpp, Guio.cpp) ----------

/// More widget (`Guio`) fields than `display_item` (tel26_1) names: the item's own index, its three colours (words),
/// the text entry's target buffer and the cached text image.
mod guio_fields {
    pub(crate) const ENTRY_TARGET: usize = 0x54;
    pub(crate) const TEXT_IMAGE: usize = 0x5c;
    pub(crate) const INDEX: usize = 0x70;
    pub(crate) const COLOUR: usize = 0x72;
    pub(crate) const HIGHLIGHT_COLOUR: usize = 0x74;
    pub(crate) const TEXT_COLOUR: usize = 0x78;
}

/// The shared text image's name (a widget whose text is this name shares the image and only counts it down) and its
/// count (word) [inference].
const GUIO_SHARED_IMAGE_NAME: usize = 0xae2920;
const GUIO_SHARED_IMAGE_COUNT: usize = 0xae291c;
/// The game's scratch text (`0xde1f64`, 0x100 bytes used here) and the Display flag that lets a widget redraw at once
/// [inference].
const GUI_SCRATCH_TEXT: usize = 0xde1f64;
const DISPLAY_LIVE_REDRAW: usize = 0x12f545;

/// `restore_background(x, y, image)` (0x5e3fd0, cdecl): puts back the screen under a widget.
unsafe fn gui_restore_background(x: u32, y: u32, image: usize) {
    let f: unsafe extern "C" fn(u32, u32, usize) = core::mem::transmute(0x5e3fd0usize);
    f(x, y, image);
}

/// `update_screen_rect(x1, y1, x2, y2)` (0x5e2eb0, cdecl): shows a rectangle of the back buffer.
unsafe fn gui_update_rect(item: usize) {
    use display_item as d;
    let f: unsafe extern "C" fn(u32, u32, u32, u32) = core::mem::transmute(0x5e2eb0usize);
    f(read_u32(item + d::X1), read_u32(item + d::Y1), read_u32(item + d::X2), read_u32(item + d::Y2));
}

/// The display-list writes of a gui builder: what `screens_forwarder_regions` (tel01_1) covers, the next `areas` area
/// slots and `items` item slots, both draw orders (counts and next slots) and the scratch text.
unsafe fn gui_build_regions(areas: usize, items: usize) -> Vec<(usize, usize)> {
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

/// Comparer of a gui builder: side by side over `gui_build_regions` and the return.
unsafe fn compare_gui_build(name: &str, areas: usize, items: usize, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let regions = gui_build_regions(areas, items);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}: return {:#x} x {:#x}, differences in {:?}", name, ro, rr, first_differences(&mo, &mr)));
    ro
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
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(%%I0%%);
    compare_gui_build("gui_add_back_next_buttons", 1, 2, || o(back, next), || gui_add_back_next_buttons(back, next))
}

/// One page button of `gui_add_page_buttons`: 0x14 × 0x14 with its right edge at `right`, at `y`; with the symbol font
/// (`G_FONT_MODE` ≠ −1) the symbol `code` ("%c" into the scratch text, style 0), otherwise the text `plain` (style 1).
unsafe fn gui_page_button(right: i32, y: i32, code: u32, plain: usize, id: u32) {
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
    let o = original_fn::<unsafe extern "C" fn(i32, i32, u32, u32, u32, u32, u32, u32) -> i32>(%%I1%%);
    compare_gui_build("gui_add_page_buttons", 0, 5, || o(right, y, page, max, first_id, prev_id, next_id, last_id) as u32,
        || gui_add_page_buttons(right, y, page, max, first_id, prev_id, next_id, last_id) as u32) as i32
}

/// Rule: a widget's fields copied out to the caller's 19 pointers.
///
/// In order: the flags, x1, y1, x2, y2, +0x20, the row (`display_item::ROW`), the style (dwords); the colour and the
/// highlight colour (words), +0x3c (dword), the font and the text colour (words), the text (`TEXT`, a pointer; past the
/// five-space indent, +0x85, when the item is in an area whose +0x20e is not −1), the length limit, the id, +0x48
/// (dwords), the area (word) and the text entry target (dword).
/// Original: `Guio::get_fields` (0x5ee4a0, Guio.cpp, thiscall, ret 0x4c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn guio_get_fields(item: usize, flags: usize, x1: usize, y1: usize, x2: usize, y2: usize, f20: usize, row: usize,
    style: usize, colour: usize, highlight: usize, f3c: usize, font: usize, text_colour: usize, text: usize, max_length: usize, id: usize,
    f48: usize, area: usize, target: usize) {
    use display_item as d;
    use guio_fields as g;
    for (out, field) in [(flags, d::FLAGS), (x1, d::X1), (y1, d::Y1), (x2, d::X2), (y2, d::Y2), (f20, 0x20), (row, d::ROW), (style, d::STYLE)] {
        write_u32(out, read_u32(item + field));
    }
    write_u16(colour, read_u16(item + g::COLOUR));
    write_u16(highlight, read_u16(item + g::HIGHLIGHT_COLOUR));
    write_u32(f3c, read_u32(item + 0x3c));
    write_u16(font, read_u16(item + d::FONT));
    write_u16(text_colour, read_u16(item + g::TEXT_COLOUR));
    write_usize(text, item + d::TEXT);
    write_u32(max_length, read_u32(item + d::MAX_LENGTH));
    write_u32(id, read_u32(item + d::ID));
    write_u32(f48, read_u32(item + 0x48));
    write_u16(area, read_u16(item + d::AREA));
    write_u32(target, read_u32(item + g::ENTRY_TARGET));
    let a = read_i16(area);
    if a != -1 && read_i16(guio_area(item, a) + 0x20e) != -1 { // tel38_3: an area item's text skips the indent
        write_usize(text, item + d::TEXT + 5);
    }
}

/// Comparer for `guio_get_fields`: side by side over the 19 outputs.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_guio_get_fields(item: usize, flags: usize, x1: usize, y1: usize, x2: usize, y2: usize, f20: usize, row: usize,
    style: usize, colour: usize, highlight: usize, f3c: usize, font: usize, text_colour: usize, text: usize, max_length: usize, id: usize,
    f48: usize, area: usize, target: usize) {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize,
        usize, usize, usize, usize);
    let o = original_fn::<F>(%%I2%%);
    let outs = [(flags, 4), (x1, 4), (y1, 4), (x2, 4), (y2, 4), (f20, 4), (row, 4), (style, 4), (colour, 2), (highlight, 2), (f3c, 4), (font, 2),
        (text_colour, 2), (text, 4), (max_length, 4), (id, 4), (f48, 4), (area, 2), (target, 4)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&outs,
        || o(item, flags, x1, y1, x2, y2, f20, row, style, colour, highlight, f3c, font, text_colour, text, max_length, id, f48, area, target),
        || guio_get_fields(item, flags, x1, y1, x2, y2, f20, row, style, colour, highlight, f3c, font, text_colour, text, max_length, id, f48, area, target));
    SCOREBOARD.record(equal, || format!("guio_get_fields({:#x}): differences in {:?}", item, first_differences(&mo, &mr)));
}

/// Area `a` of a widget's Display (the widget keeps the area table's address at +0).
unsafe fn guio_area(item: usize, a: i16) -> usize {
    (read_usize(item) as isize + a as isize * display_list::AREA_SIZE as isize) as usize
}

/// The redraw shared by the highlight routines and `guio_set_text_and_refresh`: the saved background back
/// (`gui_restore_background`, when there is one), `Guio::draw(item, 0)` and the rectangle shown.
unsafe fn guio_redraw(item: usize) {
    use display_item as d;
    let back = read_usize(item + d::BACK_IMAGE);
    if back != 0 {
        gui_restore_background(read_u32(item + d::X1), read_u32(item + d::Y1), back);
    }
    exe::guio_draw(item, 0);
    gui_update_rect(item);
}

/// Whether a highlight change redraws the widget: a style with 0x30 (the bits that show the highlight), unless the
/// widget has flag 4 and the same colour and highlight colour.
unsafe fn guio_highlight_shows(item: usize) -> bool {
    use guio_fields as g;
    if read_u8(item + display_item::STYLE) & 0x30 == 0 {
        return false;
    }
    !(read_u32(item + display_item::FLAGS) & 4 != 0 && read_u16(item + g::COLOUR) == read_u16(item + g::HIGHLIGHT_COLOUR))
}

/// Rule: highlighting a widget marks it, redraws it when the highlight shows, and makes it its area's highlighted item.
///
/// `display_item::HIGHLIGHTED` = 1 and `HOVER_TIME` = the tick (0xb67a08); when `guio_highlight_shows`, `guio_redraw`;
/// in an area (`AREA` ≠ −1) the area's `display_area::HIGHLIGHTED_ITEM` = the widget's index.
/// Original: `Guio::highlight_on` (0x5eec20, Guio.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn guio_highlight_on(item: usize) {
    use display_item as d;
    write_u32(item + d::HIGHLIGHTED, 1);
    write_u32(item + d::HOVER_TIME, read_u32(0xb67a08));
    if guio_highlight_shows(item) {
        guio_redraw(item);
    }
    let area = read_i16(item + d::AREA);
    if area != -1 {
        write_u16(guio_area(item, area) + display_area::HIGHLIGHTED_ITEM, read_u16(item + guio_fields::INDEX)); // tel38_3: the area's highlight
    }
}

// simplification: draws on the screen: it does not run twice. Original only; the plan: the marks and the area's
// highlighted item.
/// Comparer for `guio_highlight_on`.
pub(crate) unsafe extern "thiscall" fn compare_guio_highlight_on(item: usize) {
    use display_item as d;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(item);
    let area = read_i16(item + d::AREA);
    let ok = read_u32(item + d::HIGHLIGHTED) == 1 && read_u32(item + d::HOVER_TIME) == read_u32(0xb67a08)
        && (area == -1 || read_u16(guio_area(item, area) + display_area::HIGHLIGHTED_ITEM) == read_u16(item + guio_fields::INDEX));
    SCOREBOARD.record(ok, || format!("guio_highlight_on({:#x}): highlighted {} area {}", item, read_u32(item + d::HIGHLIGHTED), area));
}

/// Rule: taking the highlight off a widget unmarks it, redraws it when the highlight shows, and clears its area's
/// highlighted item when it was this one.
///
/// `display_item::HIGHLIGHTED` = 0 and `HOVER_TIME` = the tick; when `guio_highlight_shows`, `guio_redraw`; in an area
/// whose `display_area::HIGHLIGHTED_ITEM` is the widget's index, it goes to −1.
/// Original: `Guio::highlight_off` (0x5eecb0, Guio.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn guio_highlight_off(item: usize) {
    use display_item as d;
    write_u32(item + d::HIGHLIGHTED, 0);
    write_u32(item + d::HOVER_TIME, read_u32(0xb67a08));
    if guio_highlight_shows(item) {
        guio_redraw(item);
    }
    let area = read_i16(item + d::AREA);
    if area != -1 {
        let h = guio_area(item, area) + display_area::HIGHLIGHTED_ITEM;
        if read_u16(h) == read_u16(item + guio_fields::INDEX) {
            write_u16(h, 0xffff);
        }
    }
}

// simplification: draws on the screen: it does not run twice. Original only; the plan: the marks and, when the area
// highlighted this widget, no highlighted item.
/// Comparer for `guio_highlight_off`.
pub(crate) unsafe extern "thiscall" fn compare_guio_highlight_off(item: usize) {
    use display_item as d;
    let area = read_i16(item + d::AREA);
    let was = area != -1 && read_u16(guio_area(item, area) + display_area::HIGHLIGHTED_ITEM) == read_u16(item + guio_fields::INDEX);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(item);
    let ok = read_u32(item + d::HIGHLIGHTED) == 0 && read_u32(item + d::HOVER_TIME) == read_u32(0xb67a08)
        && (!was || read_i16(guio_area(item, area) + display_area::HIGHLIGHTED_ITEM) == -1);
    SCOREBOARD.record(ok, || format!("guio_highlight_off({:#x}): highlighted {} area {}", item, read_u32(item + d::HIGHLIGHTED), area));
}

/// Whether two C strings are equal (the original's inline strcmp).
unsafe fn guio_same_c_string(a: usize, b: usize) -> bool {
    let mut i = 0;
    loop {
        let c = read_u8(a + i);
        if c != read_u8(b + i) {
            return false;
        }
        if c == 0 {
            return true;
        }
        i += 1;
    }
}

/// Frees a widget's cached text image (`guio_fields::TEXT_IMAGE`): when its text is the shared image's name, only the
/// shared count goes down; otherwise `exe::gui_free_image`. The pointer goes to 0.
unsafe fn guio_drop_text_image(item: usize) {
    let image = read_usize(item + guio_fields::TEXT_IMAGE);
    if guio_same_c_string(GUIO_SHARED_IMAGE_NAME, item + display_item::TEXT) {
        write_i16(GUIO_SHARED_IMAGE_COUNT, read_i16(GUIO_SHARED_IMAGE_COUNT).wrapping_sub(1));
    } else {
        exe::gui_free_image(image);
    }
    write_usize(item + guio_fields::TEXT_IMAGE, 0);
}

/// Rule: a widget gets a new text; with `refresh` and a live display it is redrawn at once.
///
/// A null text: the original routine runs (error box `Guio.cpp 559`). A widget with flag 0x400 and a cached text image
/// drops it (`guio_drop_text_image`). The text is copied to `display_item::TEXT`; a text entry (flag 8) with a target
/// copies it there too. With `refresh` and the current slot's `Display` live (`DISPLAY_LIVE_REDRAW`): the saved
/// background is put back and freed (`BACK_IMAGE` = 0), `Guio::draw(item, 0)` and the rectangle shown.
/// Original: `Guio::set_text_and_refresh` (0x5eed50, Guio.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn guio_set_text_and_refresh(item: usize, text: usize, refresh: u32) {
    use display_item as d;
    if text == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I5%%)(item, text, refresh);
    }
    if read_u32(item + d::FLAGS) & 0x400 != 0 && read_usize(item + guio_fields::TEXT_IMAGE) != 0 {
        guio_drop_text_image(item);
    }
    copy_c_string(item + d::TEXT, text);
    if read_u32(item + d::FLAGS) & 8 != 0 && read_usize(item + guio_fields::ENTRY_TARGET) != 0 {
        copy_c_string(read_usize(item + guio_fields::ENTRY_TARGET), item + d::TEXT); // tel38_3: the entry's target follows
    }
    if read_u32(screens_current_display(SCREEN_MANAGER) + DISPLAY_LIVE_REDRAW) == 0 || refresh == 0 {
        return;
    }
    covered!("tel38_3 text redrawn at once");
    let back = read_usize(item + d::BACK_IMAGE);
    if back != 0 {
        gui_restore_background(read_u32(item + d::X1), read_u32(item + d::Y1), back);
        exe::gui_free_image(read_usize(item + d::BACK_IMAGE));
        write_usize(item + d::BACK_IMAGE, 0);
    }
    exe::guio_draw(item, 0);
    gui_update_rect(item);
}

// simplification: frees images and draws: it does not run twice. Original only; the plan: the text and, for a text
// entry, its target.
/// Comparer for `guio_set_text_and_refresh`.
pub(crate) unsafe extern "thiscall" fn compare_guio_set_text_and_refresh(item: usize, text: usize, refresh: u32) {
    use display_item as d;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I5%%);
    if text == 0 {
        return o(item, text, refresh);
    }
    let expected = core::slice::from_raw_parts(text as *const u8, c_string_len(text)).to_vec();
    o(item, text, refresh);
    let same = |p: usize| c_string_len(p) == expected.len() && core::slice::from_raw_parts(p as *const u8, expected.len()) == &expected[..];
    let target = read_usize(item + guio_fields::ENTRY_TARGET);
    let ok = same(item + d::TEXT) && (read_u32(item + d::FLAGS) & 8 == 0 || target == 0 || same(target));
    SCOREBOARD.record(ok, || format!("guio_set_text_and_refresh({:#x}): text {:?}", item, String::from_utf8_lossy(&expected)));
}

/// Rule: a widget's images are freed: the saved background (`display_item::BACK_IMAGE`, `exe::gui_free_image`) and the
/// cached text image (`guio_drop_text_image`), each pointer going to 0.
/// Original: `Guio::free_images` (0x5eef00, Guio.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn guio_free_images(item: usize) {
    let back = read_usize(item + display_item::BACK_IMAGE);
    if back != 0 {
        exe::gui_free_image(back);
        write_usize(item + display_item::BACK_IMAGE, 0);
    }
    if read_usize(item + guio_fields::TEXT_IMAGE) != 0 {
        covered!("tel38_3 text image freed");
        guio_drop_text_image(item);
    }
}

// simplification: frees the images: it does not run twice. Original only; the plan: both pointers null.
/// Comparer for `guio_free_images`.
pub(crate) unsafe extern "thiscall" fn compare_guio_free_images(item: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(item);
    let (back, image) = (read_usize(item + display_item::BACK_IMAGE), read_usize(item + guio_fields::TEXT_IMAGE));
    SCOREBOARD.record(back == 0 && image == 0, || format!("guio_free_images({:#x}): back {:#x} image {:#x}", item, back, image));
}

/// Rule: a widget's colours chosen by `which` (1 colour, 2 highlight colour, 4 text colour) are converted from the
/// 5-5-5 or 5-6-5 pixel format (`to_565`) to the screen's.
///
/// The source format: masks 0xf800, 0x7e0, 0x1f with `to_565`, otherwise 0x7c00, 0x3e0, 0x1f (at +0x10 of a local
/// format). Each chosen colour is first split into its three channels (0x5e49f0, cdecl), all into the same three
/// locals; then each chosen colour gets `exe::graphics_rgb_to_pixel` of those locals in the screen format. Defect of the
/// original: with more than one colour chosen, every one of them gets the last one split (the text colour, else the
/// highlight colour). With the "no format" flag (0xaebe0c) the split writes nothing and the conversion gives 0.
/// Original: `Guio::convert_colours_pixel_format` (0x5eef80, Guio.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn guio_convert_colours_pixel_format(item: usize, to_565: u32, which: u32) {
    use guio_fields as g;
    let split: unsafe extern "C" fn(u32, *mut u32, *mut u32, *mut u32, *const u32) = core::mem::transmute(0x5e49f0usize);
    let mut format = [0u32; 7];
    format[4..].copy_from_slice(&if to_565 != 0 { [0xf800, 0x7e0, 0x1f] } else { [0x7c00, 0x3e0, 0x1f] });
    // simplification: the original's channel locals start as leftovers; they only matter with the "no format" flag,
    // where the conversion gives 0 anyway
    let (mut c0, mut c1, mut c2) = (0u32, 0u32, 0u32);
    let chosen = [(1u32, g::COLOUR), (2, g::HIGHLIGHT_COLOUR), (4, g::TEXT_COLOUR)];
    for &(bit, field) in &chosen {
        if which & bit != 0 {
            split(read_u16(item + field) as u32, &mut c0, &mut c1, &mut c2, format.as_ptr()); // tel38_3: every split into the same channels
        }
    }
    for &(bit, field) in &chosen {
        if which & bit != 0 {
            write_u16(item + field, exe::graphics_rgb_to_pixel(c0, c1, c2, 0) as u16);
        }
    }
}

/// Comparer for `guio_convert_colours_pixel_format`: side by side over the three colours.
pub(crate) unsafe extern "thiscall" fn compare_guio_convert_colours_pixel_format(item: usize, to_565: u32, which: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I7%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(item + guio_fields::COLOUR, 8)], || o(item, to_565, which),
        || guio_convert_colours_pixel_format(item, to_565, which));
    SCOREBOARD.record(equal, || format!("guio_convert_colours_pixel_format({:#x}, {}, {}): differences in {:?}", item, to_565, which, first_differences(&mo, &mr)));
}
