// ---------- batch %%LEVA%%: the screen manager's display list helpers: the screen title and subtitle, the text and button items, the area items, an area's scroll row, the scrollbar keys and the record and flag counts (scrman.cpp) ----------

/// Comparer of an item helper: side by side over `scrman_item_regions` and the item index (the low word: `Display::add_item`
/// returns a short).
unsafe fn compare_scrman_item(name: &str, sm: usize, area: u32, original: impl FnOnce() -> u16, rust: impl FnOnce() -> u16) -> u16 {
    let regions = scrman_item_regions(sm, area);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}: index {:#x} x {:#x}, differences in {:?}", name, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// `Display::add_item` of the screen manager's current slot with the 18 fields; the item index (a short) comes back.
unsafe fn scrman_add_item(sm: usize, f: [u32; 18]) -> u16 {
    let add: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    add(screens_current_display(sm), f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12], f[13], f[14], f[15],
        f[16], f[17]) as u16
}

/// Rule: a screen's title is a text item across the top of the screen, in font 0xc, style 7, centred.
///
/// `Display::add_item` of the current slot's `Display` with type 1, the rectangle (100, 10)–(790, 70), two zeros, the
/// text flags 0x30, `colour` (the background colour, field 8), 0, the font 0xc, the style 7, `text_colour`, `text`,
/// three zeros and no area (−1). The item index comes back.
/// Original: `ScreenManager::add_screen_title` (0x803f30, scrman.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_screen_title(sm: usize, colour: u32, text_colour: u32, text: u32) -> u16 {
    covered!("tel38_1 screen title");
    scrman_add_item(sm, [1, 0x64, 0xa, 0x316, 0x46, 0, 0, 0x30, colour, 0, 0xc, 7, text_colour, text, 0, 0, 0, u32::MAX]) // tel38_1: the title rectangle
}

/// Comparer for `screen_manager_add_screen_title`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_screen_title(sm: usize, colour: u32, text_colour: u32, text: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u16>(%%I0%%);
    compare_scrman_item("screen_manager_add_screen_title", sm, u32::MAX, || o(sm, colour, text_colour, text),
        || screen_manager_add_screen_title(sm, colour, text_colour, text))
}

/// Rule: a screen's subtitle is a text item under the title, in font 0xc, style 6, in the yellow text colour.
///
/// `Display::add_item` of the current slot's `Display` with type 1, the rectangle (100, 80)–(790, 125), two zeros, the
/// text flags 2, two zeros, the font 0xc, the style 6, the yellow colour (the word `G_GUI_COLOUR_YELLOW`), `text`, three
/// zeros and no area (−1). The second argument is not read. The item index comes back.
/// Original: `ScreenManager::add_screen_subtitle` (0x803f80, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_screen_subtitle(sm: usize, text: u32, _unused: u32) -> u16 {
    covered!("tel38_1 screen subtitle");
    // simplification: the original pushes the colour in dx, whose high word is the caller's edx; Guio::init keeps a word
    let yellow = read_u16(G_GUI_COLOUR_YELLOW) as u32;
    scrman_add_item(sm, [1, 0x64, 0x50, 0x316, 0x7d, 0, 0, 2, 0, 0, 0xc, 6, yellow, text, 0, 0, 0, u32::MAX]) // tel38_1: the subtitle style
}

/// Comparer for `screen_manager_add_screen_subtitle`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_screen_subtitle(sm: usize, text: u32, unused: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u16>(%%I1%%);
    compare_scrman_item("screen_manager_add_screen_subtitle", sm, u32::MAX, || o(sm, text, unused), || screen_manager_add_screen_subtitle(sm, text, unused))
}

/// Rule: a text item is a type-1 item outside any area, with no id and no data.
///
/// `ScreenManager::add_text_item(x1, y1, x2, y2, flags, colour, font, style, text_colour, text, _)`: `Display::add_item`
/// of the current slot's `Display` with type 1, the rectangle, two zeros, the flags, the colour, 0, the font, the
/// style, the text colour, the text, three zeros and no area (−1). The eleventh argument is not read. The item index
/// comes back.
/// Original: `ScreenManager::add_text_item` (0x803fd0, scrman.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_text_item(sm: usize, x1: u32, y1: u32, x2: u32, y2: u32, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32, _unused: u32) -> u16 {
    scrman_add_item(sm, [1, x1, y1, x2, y2, 0, 0, flags, colour, 0, font, style, text_colour, text, 0, 0, 0, u32::MAX]) // tel38_1: a text item
}

/// Comparer for `screen_manager_add_text_item`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_text_item(sm: usize, x1: u32, y1: u32, x2: u32, y2: u32, flags: u32, colour: u32,
    font: u32, style: u32, text_colour: u32, text: u32, unused: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(%%I2%%);
    compare_scrman_item("screen_manager_add_text_item", sm, u32::MAX,
        || o(sm, x1, y1, x2, y2, flags, colour, font, style, text_colour, text, unused),
        || screen_manager_add_text_item(sm, x1, y1, x2, y2, flags, colour, font, style, text_colour, text, unused))
}

/// Rule: a button item is a type-2 item outside any area whose highlight colours are its own colour, carrying its id
/// and data.
///
/// `ScreenManager::add_button_item(x1, y1, x2, y2, flags, colour, font, style, text_colour, text, id, data)`:
/// `Display::add_item` of the current slot's `Display` with type 2, the rectangle, two zeros, the flags, the colour
/// twice (fields 8 and 9), the font, the style, the text colour, the text, 0, the id, the data and no area (−1). The
/// item index comes back.
/// Original: `ScreenManager::add_button_item` (0x804030, scrman.cpp, thiscall, ret 0x30).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_button_item(sm: usize, x1: u32, y1: u32, x2: u32, y2: u32, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32, id: u32, data: u32) -> u16 {
    scrman_add_item(sm, [2, x1, y1, x2, y2, 0, 0, flags, colour, colour, font, style, text_colour, text, 0, id, data, u32::MAX]) // tel38_1: a button item
}

/// Comparer for `screen_manager_add_button_item`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_button_item(sm: usize, x1: u32, y1: u32, x2: u32, y2: u32, flags: u32, colour: u32,
    font: u32, style: u32, text_colour: u32, text: u32, id: u32, data: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(%%I3%%);
    compare_scrman_item("screen_manager_add_button_item", sm, u32::MAX,
        || o(sm, x1, y1, x2, y2, flags, colour, font, style, text_colour, text, id, data),
        || screen_manager_add_button_item(sm, x1, y1, x2, y2, flags, colour, font, style, text_colour, text, id, data))
}

/// Rule: an area button is a type-2 item placed by its area (no rectangle), with its own colour as the highlight.
///
/// `ScreenManager::add_area_button(area, w, h, flags, colour, font, style, text_colour, text, id, data)`:
/// `Display::add_item` of the current slot's `Display` with type 2, four zeros (the area lays it out), the width and
/// height (fields 5 and 6), the flags, the colour twice, the font, the style, the text colour, the text, 0, the id, the
/// data and the area. The item index comes back.
/// Original: `ScreenManager::add_area_button` (0x8040a0, scrman.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_area_button(sm: usize, area: u32, w: u32, h: u32, flags: u32, colour: u32, font: u32, style: u32,
    text_colour: u32, text: u32, id: u32, data: u32) -> u16 {
    scrman_add_item(sm, [2, 0, 0, 0, 0, w, h, flags, colour, colour, font, style, text_colour, text, 0, id, data, area]) // tel38_1: an area button
}

/// Comparer for `screen_manager_add_area_button`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_area_button(sm: usize, area: u32, w: u32, h: u32, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32, id: u32, data: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(%%I4%%);
    compare_scrman_item("screen_manager_add_area_button", sm, area,
        || o(sm, area, w, h, flags, colour, font, style, text_colour, text, id, data),
        || screen_manager_add_area_button(sm, area, w, h, flags, colour, font, style, text_colour, text, id, data))
}

/// Rule: an area text is a type-1 item placed by its area, with no id and no data.
///
/// `ScreenManager::add_area_text(area, w, h, flags, colour, font, style, text_colour, text)`: `Display::add_item` of
/// the current slot's `Display` with type 1, four zeros, the width and height, the flags, the colour, 0, the font, the
/// style, the text colour, the text, three zeros and the area. The item index comes back.
/// Original: `ScreenManager::add_area_text` (0x804100, scrman.cpp, thiscall, ret 0x24).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_area_text(sm: usize, area: u32, w: u32, h: u32, flags: u32, colour: u32, font: u32, style: u32,
    text_colour: u32, text: u32) -> u16 {
    scrman_add_item(sm, [1, 0, 0, 0, 0, w, h, flags, colour, 0, font, style, text_colour, text, 0, 0, 0, area]) // tel38_1: an area text
}

/// Comparer for `screen_manager_add_area_text`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_area_text(sm: usize, area: u32, w: u32, h: u32, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(%%I5%%);
    compare_scrman_item("screen_manager_add_area_text", sm, area, || o(sm, area, w, h, flags, colour, font, style, text_colour, text),
        || screen_manager_add_area_text(sm, area, w, h, flags, colour, font, style, text_colour, text))
}

/// Rule: like `screen_manager_add_area_text`, with the height read through a pointer.
///
/// `ScreenManager::add_area_text_by_ref(area, w, h_ptr, flags, colour, font, style, text_colour, text, _, _)`: the same
/// item as `add_area_text`, field 6 being the dword at `h_ptr`. The last two arguments are not read. The item index
/// comes back.
/// Original: `ScreenManager::add_area_text_by_ref` (0x804160, scrman.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_area_text_by_ref(sm: usize, area: u32, w: u32, h_ptr: usize, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32, _unused1: u32, _unused2: u32) -> u16 {
    scrman_add_item(sm, [1, 0, 0, 0, 0, w, read_u32(h_ptr), flags, colour, 0, font, style, text_colour, text, 0, 0, 0, area])
}

/// Comparer for `screen_manager_add_area_text_by_ref`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_area_text_by_ref(sm: usize, area: u32, w: u32, h_ptr: usize, flags: u32, colour: u32,
    font: u32, style: u32, text_colour: u32, text: u32, unused1: u32, unused2: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(%%I6%%);
    compare_scrman_item("screen_manager_add_area_text_by_ref", sm, area,
        || o(sm, area, w, h_ptr, flags, colour, font, style, text_colour, text, unused1, unused2),
        || screen_manager_add_area_text_by_ref(sm, area, w, h_ptr, flags, colour, font, style, text_colour, text, unused1, unused2))
}

/// Rule: the row an area of the current screen shows; for a remote manager, the row he last reported for that area.
///
/// The record of the current slot (`screen_record`, tel29_1): when it is local (`screen_history::NETWORK_PLAYER` = 0),
/// area −1 gives 0 and any other area (its low word, signed) of the slot's `Display` (`screen_manager::DISPLAY`) gives
/// `Area::get_shown_row` (0x4032e0, thiscall). When it is remote, the reported row (`SCREEN_KEY_DATA`) when the area is
/// the reported one (`SCREEN_KEY`), otherwise 0.
/// Original: `ScreenManager::get_area_scroll_row` (0x803320, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_area_scroll_row(sm: usize, area: u32) -> u32 {
    let rec = screen_record(sm);
    if read_u16(rec + screen_history::NETWORK_PLAYER) == 0 {
        if area as u16 == 0xffff {
            return 0;
        }
        let shown_row: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4032e0usize);
        return shown_row(display_area_at(read_usize(rec + screen_manager::DISPLAY), area as i16));
    }
    covered!("tel38_1 remote scroll row");
    if area as u16 == read_u16(rec + screen_history::SCREEN_KEY) {
        return read_u32(rec + screen_history::SCREEN_KEY_DATA);
    }
    0
}

compare_value!(compare_screen_manager_get_area_scroll_row, screen_manager_get_area_scroll_row, %%I7%%, (area: u32) -> u32);

/// The mouse globals `Display::press_scrollbar_key` (0x5621f0) writes: the y (0xb67a14), the x (0xb67a20), the left button
/// (`G_MOUSE_LEFT_BUTTON_DOWN`) and the key (0xb67a3c).
const SCROLLBAR_KEY_MOUSE: usize = 0xb67a14;

/// Rule: a scrollbar key of the current screen is pressed as a click on the open scrollbar.
///
/// `Display::press_scrollbar_key(key, held)` (0x5621f0, thiscall, ret 8) of the current slot's `Display`, with both
/// arguments: with an open scrollbar (`G_ACTIVE_AREA_SCROLLBAR`) it fakes a left click on the arrow or the track the key
/// names (0..3).
/// Original: `ScreenManager::press_scrollbar_key` (0x803390, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_press_scrollbar_key(sm: usize, key: u32, held: u32) {
    covered!("tel38_1 scrollbar key");
    let press: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x5621f0usize);
    press(screens_current_display(sm), key, held);
}

/// Comparer for `screen_manager_press_scrollbar_key`: side by side over the mouse globals.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_press_scrollbar_key(sm: usize, key: u32, held: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I8%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(SCROLLBAR_KEY_MOUSE, 0x2c)], || o(sm, key, held),
        || screen_manager_press_scrollbar_key(sm, key, held));
    SCOREBOARD.record(equal, || format!("screen_manager_press_scrollbar_key({}, {}): differences in {:?}", key, held, first_differences(&mo, &mr)));
}

/// Rule: how many manager records there are in all channels.
///
/// The sum of the 16 channel record counts (`screen_history::CHANNEL_RECORD_COUNTS`), as a 16-bit word (the original
/// keeps garbage in the high word; the callers read the short).
/// Original: `ScreenManager::count_all_records` (0x8033c0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_count_all_records(sm: usize) -> u16 {
    (0..16).fold(0u16, |s, c| s.wrapping_add(read_u16(sm + screen_history::CHANNEL_RECORD_COUNTS + 2 * c))) // tel38_1: all 16 channels
}

compare_value!(compare_screen_manager_count_all_records, screen_manager_count_all_records, %%I9%%, () -> u16);

/// Rule: how many single-instance screens are open in all records of all channels.
///
/// The sum of the flagged screen counts (`screen_history::FLAGGED_COUNT`) of the 16 records of each of the 16 channels,
/// as a 16-bit word (the callers read the short).
/// Original: `ScreenManager::count_flagged_screens` (0x803d50, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_count_flagged_screens(sm: usize) -> u16 {
    (0..256).fold(0u16, |s, k| s.wrapping_add(read_u16(sm + screen_history::FLAGGED_COUNT + k * screen_history::RECORD_SIZE)))
}

compare_value!(compare_screen_manager_count_flagged_screens, screen_manager_count_flagged_screens, %%I10%%, () -> u16);
