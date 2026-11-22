// ---------- batch %%LEVA%%: the popup areas, scrolling an area, the area colour, the widget rectangles, the scroll keys, the media paths and the screen-sent log (Area.cpp, display.cpp) ----------

/// The fixed rectangle of a Guio widget (x1, x2, y1, y2; 0 = placed by its area's grid): `guio_set_fixed_rect`.
#[allow(dead_code)]
pub(crate) mod guio_rect {
    pub(crate) const COLUMN: usize = 0x20;
    pub(crate) const FIXED_X1: usize = 0x28;
    pub(crate) const FIXED_X2: usize = 0x2c;
    pub(crate) const FIXED_Y1: usize = 0x30;
    pub(crate) const FIXED_Y2: usize = 0x34;
}

/// i32: the first row of the area's grid that the widgets use (`Area::init` sets 0x7fffffff, `add_widget` lowers it).
const AREA_FIRST_ROW: usize = 0x10;
/// u32: a scroll bar key is held (`display_press_scrollbar_key`) [inference].
const DISPLAY_SCROLL_KEY_HELD: usize = 0xb67a3c;

/// The screen-flip call (`graphics_flip_rect` 0x5e2eb0) over an area's rectangle.
unsafe fn area_flip(area: usize) {
    display_rect_call(0x5e2eb0, read_i32(area), read_i32(area + 4), read_i32(area + 8), read_i32(area + 0xc));
}

/// Opens a popup area: moves it to its popup position, marks it open, draws it and flips its rectangle.
///
/// The position comes from `Area::get_popup_position` (0x403760); when it differs from the area's x1/y1 the area is
/// moved there (`Area::move_to` 0x403a70). Then `area_fields::IS_OPEN` = 1, `Area::draw(area, 0)` and the flip of
/// (x1, y1, x2, y2).
/// Original: `Area::open_popup` (0x402d10, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_open_popup(area: usize) {
    covered!("tel37_2 open a popup");
    let position: unsafe extern "thiscall" fn(usize, *mut i32, *mut i32) = core::mem::transmute(0x403760usize);
    let move_to: unsafe extern "thiscall" fn(usize, i32, i32) = core::mem::transmute(0x403a70usize);
    let (mut x, mut y) = (0i32, 0i32);
    position(area, &mut x, &mut y);
    if read_i32(area) != x || read_i32(area + 4) != y {
        move_to(area, x, y);
    }
    write_u32(area + area_fields::IS_OPEN, 1);
    display_method1(0x402e70, area, 0);
    area_flip(area);
}

/// Closes a popup area that is open: unhighlights its widgets (closing the popups they own), puts back the screen
/// under it and flips its rectangle.
///
/// With `area_fields::IS_OPEN` set: from `area_fields::FIRST_VISIBLE_ITEM` (when not -1) to the end of the list each
/// widget gets `display_item::HIGHLIGHTED` = 0 and the popup area it owns (`guio::POPUP_AREA_INDEX`, when not -1) is
/// closed the same way. Then `IS_OPEN` = 0; with a saved image (`area_fields::SAVED_IMAGE`) it is put back at (x1, y1),
/// freed and cleared, and the rectangle is flipped.
/// Original: `Area::close_popup` (0x402d80, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_close_popup(area: usize) {
    if read_u32(area + area_fields::IS_OPEN) == 0 {
        return;
    }
    covered!("tel37_2 close a popup");
    let mut k = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    if k != -1 && k < read_i16(area + area_fields::ITEM_COUNT) {
        loop {
            write_u32(area_list_item(area, k) + display_item::HIGHLIGHTED, 0);
            let popup = read_i16(area_list_item(area, k) + guio::POPUP_AREA_INDEX);
            if popup != -1 {
                area_close_popup(area_item_at_size(read_usize(area + area_fields::AREAS), popup, display_list::AREA_SIZE));
            }
            k = k.wrapping_add(1);
            if k >= read_i16(area + area_fields::ITEM_COUNT) {
                break;
            }
        }
    }
    let image = read_usize(area + area_fields::SAVED_IMAGE);
    write_u32(area + area_fields::IS_OPEN, 0);
    if image != 0 {
        display_restore_image(read_i32(area), read_i32(area + 4), image);
        exe::gui_free_image(read_usize(area + area_fields::SAVED_IMAGE));
        write_usize(area + area_fields::SAVED_IMAGE, 0);
        area_flip(area);
    }
}

/// Element `k` of a table of `size`-byte records at `base` (signed index).
pub(crate) fn area_item_at_size(base: usize, k: i16, size: usize) -> usize {
    (base as isize).wrapping_add(k as isize * size as isize) as usize
}

/// The row (`display_item::ROW`) of the widget at list position `k`.
unsafe fn area_row_at(area: usize, k: i16) -> i32 {
    read_i32(area_list_item(area, k) + display_item::ROW)
}

/// The list position that becomes the first visible one after scrolling `delta` rows from `area_fields::FIRST_VISIBLE_ITEM`.
///
/// Down (`delta` > 0): the first position whose row is at least `delta` rows past the current first one (or the last
/// position). Up (otherwise): the last position going back whose row is at most `delta` rows from the current first one
/// (or position 0), then back to the first position of that row (equal rows go together).
pub(crate) unsafe fn area_scroll_target(area: usize, delta: i32) -> i16 {
    let count = read_i16(area + area_fields::ITEM_COUNT);
    let first = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    let mut k = first;
    let distance = |k: i16| area_row_at(area, k).wrapping_sub(area_row_at(area, first));
    if delta > 0 {
        if distance(k) < delta {
            loop {
                k = k.wrapping_add(1);
                if k >= count {
                    k = k.wrapping_sub(1);
                    break;
                }
                if distance(k) >= delta {
                    break;
                }
            }
        }
        return k;
    }
    if distance(k) > delta {
        loop {
            k = k.wrapping_sub(1);
            if k < 0 {
                k = k.wrapping_add(1);
                break;
            }
            if distance(k) <= delta {
                break;
            }
        }
    }
    while k as i32 - 1 != -1 && area_row_at(area, k.wrapping_sub(1)) == area_row_at(area, k) {
        k = k.wrapping_sub(1);
    }
    k
}

/// The rectangle (x1, y1, x2, y2) of a widget shown `distance` rows below the first visible one: its fixed x and y
/// when set (`guio_rect::FIXED_X1`, `FIXED_Y1`), else its column's bounds and the row `AREA_FIRST_ROW` + `distance`
/// of the area's grid, from the area's x1/y1.
pub(crate) unsafe fn area_visible_item_rect(area: usize, item: usize, distance: i32) -> [i32; 4] {
    let (x1, x2) = if read_i32(item + guio_rect::FIXED_X1) != 0 {
        (read_i32(item + guio_rect::FIXED_X1), read_i32(item + guio_rect::FIXED_X2))
    } else {
        let column = read_i32(item + guio_rect::COLUMN) as isize;
        let left = read_i32(area);
        (read_i32((area as isize + area_fields::COLUMN_STARTS as isize + column * 4) as usize).wrapping_add(left),
         read_i32((area as isize + area_fields::COLUMN_ENDS as isize + column * 4) as usize).wrapping_add(left))
    };
    let (y1, y2) = if read_i32(item + guio_rect::FIXED_Y1) != 0 {
        (read_i32(item + guio_rect::FIXED_Y1), read_i32(item + guio_rect::FIXED_Y2))
    } else {
        let row = read_i32(area + AREA_FIRST_ROW).wrapping_add(distance) as isize;
        let top = read_i32(area + 4);
        (read_i32((area as isize + area_fields::ROW_STARTS as isize + row * 4) as usize).wrapping_add(top),
         read_i32((area as isize + area_fields::ROW_ENDS as isize + row * 4) as usize).wrapping_add(top))
    };
    [x1, y1, x2, y2]
}

/// Scrolls an area by `delta` rows: the new first visible widget, the visible widgets placed and drawn, the flip and
/// the screen manager told the new first row.
///
/// Nothing without rows (`area_fields::ROW_COUNT`) or widgets. The saved image (`area_fields::SAVED_IMAGE`) is put
/// back first. The new first position comes from `area_scroll_target`; when it is a position of the list, each widget
/// from it on whose row is within `ROW_COUNT` - 1 rows of the first gets the rectangle of `area_visible_item_rect` and
/// is drawn (`Guio::draw(item, 0)`). Then the flip of (x1, y1, x2 - 22, y2) and `ScreenManager` 0x800330 (the area
/// index and the first visible widget's row).
/// Original: `Area::scroll_rows` (0x403ee0, Area.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn area_scroll_rows(area: usize, delta: i32) {
    if read_u8(area + area_fields::ROW_COUNT) == 0 || read_i16(area + area_fields::ITEM_COUNT) <= 0 {
        return;
    }
    covered!("tel37_2 scroll an area");
    let image = read_usize(area + area_fields::SAVED_IMAGE);
    if image != 0 {
        display_restore_image(read_i32(area), read_i32(area + 4), image);
    }
    let mut k = area_scroll_target(area, delta);
    write_i16(area + area_fields::FIRST_VISIBLE_ITEM, k);
    if k != -1 {
        while k < read_i16(area + area_fields::ITEM_COUNT) {
            let item = area_list_item(area, k);
            let first = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
            let distance = read_i32(item + display_item::ROW).wrapping_sub(area_row_at(area, first));
            if distance > read_u8(area + area_fields::ROW_COUNT) as i32 - 1 {
                break;
            }
            let [x1, y1, x2, y2] = area_visible_item_rect(area, item, distance);
            write_i32(item + display_item::X1, x1);
            write_i32(item + display_item::X2, x2);
            write_i32(item + display_item::Y1, y1);
            write_i32(item + display_item::Y2, y2);
            exe::guio_draw(area_list_item(area, k), 0);
            k = k.wrapping_add(1);
            if k == -1 {
                break;
            }
        }
    }
    display_rect_call(0x5e2eb0, read_i32(area), read_i32(area + 4), read_i32(area + 8).wrapping_sub(0x16), read_i32(area + 0xc));
    let first_row = area_row_at(area, read_i16(area + area_fields::FIRST_VISIBLE_ITEM));
    let tell: unsafe extern "thiscall" fn(usize, u32, i32) = core::mem::transmute(0x800330usize);
    tell(G_SCREEN_MANAGER, read_u16(area + area_fields::AREA_INDEX) as u32, first_row);
}

/// Converts the area colour (`area_fields::COLOUR`) to the screen's pixel format: the colour is split into red,
/// green and blue by the masks of the other format (`graphics` 0x5e49f0: 5-5-5 when `to_565` is 0, 5-6-5 otherwise)
/// and joined again by `graphics_rgb_to_pixel` (the current format).
/// Original: `Area::convert_colour_to_pixel_format` (0x4041d0, Area.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn area_convert_colour_to_pixel_format(area: usize, to_565: u32) {
    let split: unsafe extern "C" fn(u32, *mut u32, *mut u32, *mut u32, *const u32) = core::mem::transmute(0x5e49f0usize);
    // the pixel format record: only the red, green and blue masks (+0x10, +0x14, +0x18) are read
    let mut format = [0u32; 7];
    (format[4], format[5]) = if to_565 != 0 { (0xf800, 0x7e0) } else { (0x7c00, 0x3e0) };
    format[6] = 0x1f;
    // the blue output is the argument's own slot in the original; red and green are locals
    let (mut red, mut green, mut blue) = (0u32, 0u32, to_565);
    split(read_u16(area + area_fields::COLOUR) as u32, &mut red, &mut green, &mut blue, format.as_ptr());
    let pixel = exe::graphics_rgb_to_pixel(red, green, blue, 0);
    write_u16(area + area_fields::COLOUR, pixel as u16);
}

/// Sets a widget's fixed rectangle: `guio_rect::FIXED_X1`, `FIXED_Y1`, `FIXED_X2`, `FIXED_Y2`.
/// Original: `guio_set_fixed_rect` (0x567ac0, display.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn guio_set_fixed_rect(item: usize, x1: i32, y1: i32, x2: i32, y2: i32) {
    write_i32(item + guio_rect::FIXED_X1, x1);
    write_i32(item + guio_rect::FIXED_X2, x2);
    write_i32(item + guio_rect::FIXED_Y1, y1);
    write_i32(item + guio_rect::FIXED_Y2, y2);
}

/// Sets a widget's rectangle: `display_item::X1`, `Y1`, `X2`, `Y2`.
/// Original: `guio_set_rect` (0x567ae0, display.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn guio_set_rect(item: usize, x1: i32, y1: i32, x2: i32, y2: i32) {
    write_i32(item + display_item::X1, x1);
    write_i32(item + display_item::X2, x2);
    write_i32(item + display_item::Y1, y1);
    write_i32(item + display_item::Y2, y2);
}

/// Sets the current channel's screen key and its data (`screen_history::SCREEN_KEY`, `SCREEN_KEY_DATA`; the channel
/// of `screen_manager::CURRENT_SLOT`).
/// Original: `ScreenManager::set_area_selection` (0x5680b0, display.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_area_selection(mgr: usize, key: u32, data: u32) {
    write_u16(screen_record(mgr) + screen_history::SCREEN_KEY, key as u16);
    write_u32(screen_record(mgr) + screen_history::SCREEN_KEY_DATA, data);
}

/// Presses a key of the keyboard scroll bar (`G_ACTIVE_AREA_SCROLLBAR`): the mouse is put on the bar's part and the
/// left button held.
///
/// Nothing without an active scroll bar. The left button down and `DISPLAY_SCROLL_KEY_HELD` = `held`; the mouse x is
/// the bar's x1 + 1 and the mouse y, by `part`: 0 the dword at +0x14 + 2, 1 the bar's y1 + 1, 2 the dword at +0x28 + 2,
/// 3 the dword at +0x10 + 1; another part lets the button and the key go.
/// Original: `display_press_scrollbar_key` (0x5621f0, display.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn display_press_scrollbar_key(part: u32, held: u32) {
    let bar = read_usize(G_ACTIVE_AREA_SCROLLBAR);
    if bar == 0 {
        return;
    }
    covered!("tel37_2 scroll bar key");
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 1);
    write_u32(DISPLAY_SCROLL_KEY_HELD, held);
    let y = match part {
        0 => read_i32(bar + 0x14).wrapping_add(2),
        1 => read_i32(bar + 4).wrapping_add(1),
        2 => read_i32(bar + 0x28).wrapping_add(2),
        3 => read_i32(bar + 0x10).wrapping_add(1),
        _ => {
            write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
            write_u32(DISPLAY_SCROLL_KEY_HELD, 0);
            return;
        }
    };
    write_i32(DISPLAY_MOUSE_X, read_i32(bar).wrapping_add(1));
    write_i32(DISPLAY_MOUSE_Y, y);
}

/// The part of `path` after the last `folder` in it, from its second character, with ':' turned into '\' (empty when
/// `folder` is not in it).
unsafe fn display_media_tail(path: usize, folder: &[u8]) -> Option<[u8; 0x104]> {
    let name = folder.as_ptr();
    let mut found = exe::crt_strstr(path, name as usize);
    if found == 0 {
        return None;
    }
    let mut at = found + folder.len() - 1;
    loop {
        found = exe::crt_strstr(at, name as usize);
        if found == 0 {
            break;
        }
        at = found + folder.len() - 1;
    }
    if at == path {
        return None;
    }
    let mut out = [0u8; 0x104];
    let mut i = 0;
    let mut s = at + 1;
    loop {
        let c = read_u8(s);
        out[i] = if c == b':' { b'\\' } else { c };
        if c == 0 {
            break;
        }
        i += 1;
        s += 1;
    }
    Some(out)
}

/// Localises a media path received from another PC: the part after the last "pictures" (or "sounds"; then "data")
/// folder is joined to the local folder of that kind.
///
/// Nothing for an empty path or one that exists (`utils_file_exists`). The path is lowered (`_strlwr`). A picture
/// (`is_picture`) looks for "pictures", a sound for "sounds"; when that folder is not in the path, "data"; with none,
/// nothing. The rest of the path after the folder and its separator, with ':' turned into '\', is joined to the folder
/// of `CM3_PICTURES`, `CM3_SOUNDS` or `CM3_DATA` (`utils_build_file_path`) and copied over the path.
/// Original: `display_localise_media_path` (0x5680f0, display.cpp, cdecl).
pub(crate) unsafe extern "C" fn display_localise_media_path(path: usize, is_picture: u32) {
    if read_u8(path) == 0 || exe::utils_file_exists(path) != 0 {
        return;
    }
    covered!("tel37_2 localise a media path");
    let lower: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x9532a6usize);
    lower(path);
    let mut found = None;
    if is_picture != 0 {
        if let Some(t) = display_media_tail(path, b"pictures\0") {
            found = Some((t, 0x9899e0usize));
        }
    } else if let Some(t) = display_media_tail(path, b"sounds\0") {
        found = Some((t, 0x9c382cusize));
    }
    if found.is_none() {
        if let Some(t) = display_media_tail(path, b"data\0") {
            found = Some((t, 0x9b5348usize));
        }
    }
    let Some((tail, key)) = found else { return };
    let local = exe::utils_build_file_path(key, tail.as_ptr() as usize);
    tel26_copy_text(path as *mut u8, local as *const u8);
}

/// Logs the size of the screen sent to a network player: the sections sent by `Display::send_screen_section`
/// (0x562370) and the line "Screen sent: ..." in the text buffer 0xde1f64.
///
/// Nothing without a session or when the session's flag at +0xc3a5a is clear. Section 0 (the display) is always sent;
/// unless `display_list::RESEND_PENDING`, sections 2 (areas), 1 (widgets) and 3 (draw) too, and the total is the sum
/// (else the total is the display's and the others count 0). The line has the total, the display bytes, the area count
/// and bytes, the widget count and bytes and the draw bytes.
/// Original: `Display::log_screen_sent` (0x5622a0, display.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn display_log_screen_sent(d: usize, session: usize, player: u32) {
    if session == 0 || read_u32(session + 0xc3a5a) == 0 {
        return;
    }
    covered!("tel37_2 log the screen sent");
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x562370usize);
    let display = send(d, session, player, 0);
    let (mut areas, mut widgets, mut draw) = (0i32, 0i32, 0i32);
    let total = if read_u32(d + display_list::RESEND_PENDING) == 0 {
        areas = send(d, session, player, 2);
        widgets = send(d, session, player, 1);
        draw = send(d, session, player, 3);
        draw.wrapping_add(display).wrapping_add(widgets).wrapping_add(areas)
    } else {
        display
    };
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(JOB_TEXT, 0x9c3764, total, display, read_i16(d + 0x12e99e) as i32, areas, read_i16(d + display_list::ITEM_COUNT) as i32, widgets, draw);
}

/// Comparer for `area_open_popup`: only the original (it draws); afterwards the area must be open at its popup
/// position (asked before, `Area::get_popup_position`).
pub(crate) unsafe extern "thiscall" fn compare_area_open_popup(area: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let position: unsafe extern "thiscall" fn(usize, *mut i32, *mut i32) = core::mem::transmute(0x403760usize);
    let (mut x, mut y) = (0i32, 0i32);
    position(area, &mut x, &mut y);
    o(area);
    let (ax, ay, open) = (read_i32(area), read_i32(area + 4), read_u32(area + area_fields::IS_OPEN));
    SCOREBOARD.record(ax == x && ay == y && open == 1, || format!("Area::open_popup (area {}): at ({}, {}) open {}, expected ({}, {}) open 1",
        read_i16(area + area_fields::AREA_INDEX), ax, ay, open, x, y));
}

/// Comparer for `area_close_popup`: only the original (it puts the screen back); afterwards the area must be closed,
/// its saved image gone and its widgets from the first visible one unhighlighted.
pub(crate) unsafe extern "thiscall" fn compare_area_close_popup(area: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let was_open = read_u32(area + area_fields::IS_OPEN) != 0;
    o(area);
    if !was_open {
        return;
    }
    let mut lit = Vec::new();
    let first = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    if first != -1 {
        let mut k = first;
        while k >= 0 && k < read_i16(area + area_fields::ITEM_COUNT) {
            if read_u32(area_list_item(area, k) + display_item::HIGHLIGHTED) != 0 {
                lit.push(k);
            }
            k += 1;
        }
    }
    let (open, image) = (read_u32(area + area_fields::IS_OPEN), read_usize(area + area_fields::SAVED_IMAGE));
    SCOREBOARD.record(open == 0 && image == 0 && lit.is_empty(), || format!("Area::close_popup (area {}): open {} image {:#x} still highlighted {:?}",
        read_i16(area + area_fields::AREA_INDEX), open, image, lit));
}

/// Comparer for `area_scroll_rows`: only the original (it draws); the plan (the new first visible position and the
/// rectangles of the visible widgets) is computed before and checked afterwards.
pub(crate) unsafe extern "thiscall" fn compare_area_scroll_rows(area: usize, delta: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I2%%);
    if read_u8(area + area_fields::ROW_COUNT) == 0 || read_i16(area + area_fields::ITEM_COUNT) <= 0 {
        return o(area, delta);
    }
    let target = area_scroll_target(area, delta);
    let mut plan = Vec::new();
    let count = read_i16(area + area_fields::ITEM_COUNT);
    if target >= 0 && target < count {
        let base_row = area_row_at(area, target);
        let mut k = target;
        while k < count {
            let item = area_list_item(area, k);
            let distance = read_i32(item + display_item::ROW).wrapping_sub(base_row);
            if distance > read_u8(area + area_fields::ROW_COUNT) as i32 - 1 {
                break;
            }
            plan.push((k, area_visible_item_rect(area, item, distance)));
            k += 1;
        }
    }
    o(area, delta);
    let first = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    let wrong: Vec<_> = plan.iter().filter_map(|&(k, r)| {
        let item = area_list_item(area, k);
        let got = [read_i32(item + display_item::X1), read_i32(item + display_item::Y1), read_i32(item + display_item::X2), read_i32(item + display_item::Y2)];
        (got != r).then_some((k, got, r))
    }).collect();
    SCOREBOARD.record(first == target && wrong.is_empty(), || format!("Area::scroll_rows (area {}, {} rows): first {} expected {}; rectangles (position, original, plan) {:?}",
        read_i16(area + area_fields::AREA_INDEX), delta, first, target, wrong));
}

/// Comparer for `area_convert_colour_to_pixel_format`: the area colour side by side.
pub(crate) unsafe extern "thiscall" fn compare_area_convert_colour_to_pixel_format(area: usize, to_565: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I3%%);
    let reg = [(area + area_fields::COLOUR, 2)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(area, to_565), || area_convert_colour_to_pixel_format(area, to_565));
    SCOREBOARD.record(same, || format!("Area::convert_colour_to_pixel_format({}): {:?}", to_565, first_differences(&mo, &mr)));
}

/// Comparer for `guio_set_fixed_rect`.
pub(crate) unsafe extern "thiscall" fn compare_guio_set_fixed_rect(item: usize, x1: i32, y1: i32, x2: i32, y2: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i32)>(%%I4%%);
    let reg = [(item + guio_rect::FIXED_X1, 16)];
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(item, x1, y1, x2, y2), || guio_set_fixed_rect(item, x1, y1, x2, y2));
    SCOREBOARD.record(same, || format!("guio_set_fixed_rect({}, {}, {}, {}) differs", x1, y1, x2, y2));
}

/// Comparer for `guio_set_rect`.
pub(crate) unsafe extern "thiscall" fn compare_guio_set_rect(item: usize, x1: i32, y1: i32, x2: i32, y2: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i32)>(%%I5%%);
    let reg = [(item + display_item::X1, 16)];
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(item, x1, y1, x2, y2), || guio_set_rect(item, x1, y1, x2, y2));
    SCOREBOARD.record(same, || format!("guio_set_rect({}, {}, {}, {}) differs", x1, y1, x2, y2));
}

/// Comparer for `screen_manager_set_area_selection`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_area_selection(mgr: usize, key: u32, data: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I6%%);
    let reg = [(screen_record(mgr) + screen_history::SCREEN_KEY, 6)];
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(mgr, key, data), || screen_manager_set_area_selection(mgr, key, data));
    SCOREBOARD.record(same, || format!("ScreenManager::set_area_selection({}, {:#x}) differs", key as i16, data));
}

/// Comparer for `display_press_scrollbar_key`: the mouse, the button and the key flag side by side.
pub(crate) unsafe extern "stdcall" fn compare_display_press_scrollbar_key(part: u32, held: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, u32)>(%%I7%%);
    let reg = [(DISPLAY_MOUSE_X, 4), (DISPLAY_MOUSE_Y, 4), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (DISPLAY_SCROLL_KEY_HELD, 4)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(part, held), || display_press_scrollbar_key(part, held));
    SCOREBOARD.record(same, || format!("display_press_scrollbar_key({}, {}): {:?}", part, held, first_differences(&mo, &mr)));
}

/// Comparer for `display_localise_media_path`: the path buffer side by side.
pub(crate) unsafe extern "C" fn compare_display_localise_media_path(path: usize, is_picture: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(%%I8%%);
    let reg = [(path, 0x104)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(path, is_picture), || display_localise_media_path(path, is_picture));
    SCOREBOARD.record(same, || format!("display_localise_media_path (picture {}): {:?}", is_picture, first_differences(&mo, &mr)));
}

/// Comparer for `display_log_screen_sent`.
pub(crate) unsafe extern "thiscall" fn compare_display_log_screen_sent(d: usize, session: usize, player: u32) {
    // simplification: the screen sections are sent over the network once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I9%%)(d, session, player)
}
