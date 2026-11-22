// ---------- batch %%LEVA%%: the interface areas (2): releasing, adding a widget, the scroll row, the scroll bar, the popup position and moving (Area.cpp) ----------

/// The area fields of the rows and of the scroll bar (an area is 0xbf1 bytes; see `area_fields`): the first and last
/// widget rows, the pending scroll row; the scroll bar rectangle (x1, y1, x2, y2), the track between the arrow buttons
/// (top, bottom), the hidden rows, the scroll row, the thumb (height, top, bottom), the saved image behind the bar, the
/// pressed arrow (1 up, 2 down) and whether the area has a bar.
pub(crate) mod area_scroll {
    pub(crate) const FIRST_ROW: usize = 0x10;
    pub(crate) const LAST_ROW: usize = 0x14;
    pub(crate) const PENDING_ROW: usize = 0x1fc;
    pub(crate) const BAR_X1: usize = 0xb7a;
    pub(crate) const BAR_Y1: usize = 0xb7e;
    pub(crate) const BAR_X2: usize = 0xb82;
    pub(crate) const BAR_Y2: usize = 0xb86;
    pub(crate) const TRACK_TOP: usize = 0xb8a;
    pub(crate) const TRACK_BOTTOM: usize = 0xb8e;
    pub(crate) const HIDDEN_ROWS: usize = 0xb92;
    pub(crate) const SCROLL_ROW: usize = 0xb96;
    pub(crate) const THUMB_HEIGHT: usize = 0xb9a;
    pub(crate) const THUMB_TOP: usize = 0xb9e;
    pub(crate) const THUMB_BOTTOM: usize = 0xba2;
    pub(crate) const BAR_IMAGE: usize = 0xba6;
    pub(crate) const PRESSED_ARROW: usize = 0xbaa;
    pub(crate) const HAS_SCROLLBAR: usize = 0xbed;
    /// A widget's row (+0x24) and screen rectangle (+0x10..+0x1c); its popup area index (+0x7a) [inference].
    pub(crate) const WIDGET_ROW: usize = 0x24;
    pub(crate) const WIDGET_X1: usize = 0x10;
    pub(crate) const WIDGET_Y1: usize = 0x14;
    pub(crate) const WIDGET_X2: usize = 0x18;
    pub(crate) const WIDGET_Y2: usize = 0x1c;
    pub(crate) const WIDGET_AREA: usize = 0x7a;
    /// Area flags of a popup: opens under or over its widget (0x100), to the left (0x100000), to the right even past the
    /// middle of the screen (0x200000), ending at the widget's bottom (0x400000), at the widget's top even when it does
    /// not fit (0x800000) [inference from Area::get_popup_position].
    pub(crate) const POPUP_BESIDE: u32 = 0x100;
    pub(crate) const POPUP_LEFT: u32 = 0x100000;
    pub(crate) const POPUP_RIGHT_ANYWAY: u32 = 0x200000;
    pub(crate) const POPUP_ABOVE: u32 = 0x400000;
    pub(crate) const POPUP_KEEP_DOWN: u32 = 0x800000;
    /// Area size and the up and down arrow glyphs (7 × 4: width byte, height byte, the mask).
    pub(crate) const AREA_SIZE: usize = 0xbf1;
    pub(crate) const GLYPH_UP: usize = 0x9cd6bc;
    pub(crate) const GLYPH_DOWN: usize = 0x9cd6dc;
}

/// The screen limits the 1920 × 1080 exe rewrites in these routines, read from the exe itself: the 600 height of the
/// popup tests (`cmp reg, imm` at 0x40384b, 0x403922, 0x4039b5) and the 0x31f / 0x257 right and bottom edges of
/// Area::move_to (0x403b00 and 0x403bdc). The 400 middle of the popup tests is not rewritten.
pub(crate) unsafe fn popup_screen_height(k: usize) -> i32 {
    let imm = match k { 0 => 0x40384d as *const i32, 1 => 0x403924 as *const i32, _ => 0x4039b7 as *const i32 };
    imm.read_unaligned()
}

/// A widget of the area's widget table (`area_fields::ITEMS`, 0x18c bytes each).
pub(crate) unsafe fn area_widget(area: usize, k: i16) -> usize {
    area_item_at(read_usize(area + area_fields::ITEMS), k)
}

/// Frees an area's saved images: the one behind the scroll bar and the one behind the area (`exe::gui_free_image`,
/// each pointer cleared), and clears the keyboard scroll bar ([`G_ACTIVE_AREA_SCROLLBAR`]) when it is this area's.
/// Original: `Area::release` (0x4031a0, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_release(area: usize) {
    covered!("tel29_2 area released");
    if read_usize(area + area_scroll::BAR_IMAGE) != 0 {
        exe::gui_free_image(read_usize(area + area_scroll::BAR_IMAGE));
        write_usize(area + area_scroll::BAR_IMAGE, 0);
    }
    if read_usize(area + area_fields::SAVED_IMAGE) != 0 {
        exe::gui_free_image(read_usize(area + area_fields::SAVED_IMAGE));
        write_usize(area + area_fields::SAVED_IMAGE, 0);
    }
    if read_usize(G_ACTIVE_AREA_SCROLLBAR) == area + area_fields::SCROLLBAR {
        write_usize(G_ACTIVE_AREA_SCROLLBAR, 0);
    }
}

// simplification: frees the images: it does not run twice. Original only; the plan: both pointers cleared and the
// keyboard scroll bar no longer this area's.
/// Comparer for `area_release`.
pub(crate) unsafe extern "thiscall" fn compare_area_release(area: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(area);
    let ok = read_usize(area + area_scroll::BAR_IMAGE) == 0 && read_usize(area + area_fields::SAVED_IMAGE) == 0
        && read_usize(G_ACTIVE_AREA_SCROLLBAR) != area + area_fields::SCROLLBAR;
    SCOREBOARD.record(ok, || format!("area_release {:#x}: images {:#x} {:#x}", area, read_usize(area + area_scroll::BAR_IMAGE), read_usize(area + area_fields::SAVED_IMAGE)));
}

/// Adds a widget to the area's list (`area_fields::ITEM_LIST`), kept sorted by the widget's row, and widens the area's
/// first and last rows.
///
/// Nothing for widget −1, with 1200 widgets already (`area_fields::ITEM_COUNT`, signed) or in a grid-only area
/// (`area_fields_flags::GRID_ONLY`). The count goes up; the first row (`area_scroll::FIRST_ROW`) drops to the widget's
/// row when lower, the last (`LAST_ROW`) rises to it when higher; the widget goes in after the last entry whose row is
/// not above its own (insertion from the end: equal rows keep their order).
/// Original: `Area::add_widget` (0x403200, Area.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn area_add_widget(area: usize, widget: i16) {
    use area_scroll as s;
    let count = read_i16(area + area_fields::ITEM_COUNT);
    if widget == -1 || count >= 1200 || read_u32(area + area_fields::FLAGS) & area_fields_flags::GRID_ONLY != 0 {
        return;
    }
    covered!("tel29_2 widget added");
    let total = count.wrapping_add(1);
    let row = read_i32(area_widget(area, widget) + s::WIDGET_ROW);
    write_i16(area + area_fields::ITEM_COUNT, total);
    if row < read_i32(area + s::FIRST_ROW) {
        write_i32(area + s::FIRST_ROW, row);
    }
    if row > read_i32(area + s::LAST_ROW) {
        write_i32(area + s::LAST_ROW, row);
    }
    let list = |k: i16| (area as isize + area_fields::ITEM_LIST as isize + k as isize * 2) as usize;
    let mut k = total.wrapping_sub(2);
    while k >= 0 {
        let other = read_i16(list(k));
        if row >= read_i32(area_widget(area, other) + s::WIDGET_ROW) { // tel29_2: sorted by row, stable
            write_i16(list(k + 1), widget);
            return;
        }
        write_i16(list(k + 1), other);
        k -= 1;
    }
    write_i16(list(0), widget);
}

/// Comparer for `area_add_widget`: side by side over the area.
pub(crate) unsafe extern "thiscall" fn compare_area_add_widget(area: usize, widget: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i16)>(%%I1%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(area, area_scroll::AREA_SIZE)], || o(area, widget), || area_add_widget(area, widget));
    SCOREBOARD.record(equal, || format!("area_add_widget {:#x}({}): differences em {:?}", area, widget, first_differences(&mo, &mr)));
}

/// The row of the first visible widget (`area_fields::FIRST_VISIBLE_ITEM` in the list) when the area has a scroll bar
/// (`area_scroll::HAS_SCROLLBAR`), 0 when it has none.
/// Original: `Area::get_shown_row` (0x4032e0, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_get_shown_row(area: usize) -> u32 {
    if read_u32(area + area_scroll::HAS_SCROLLBAR) == 0 {
        return 0;
    }
    read_u32(area_list_item(area, read_i16(area + area_fields::FIRST_VISIBLE_ITEM)) + area_scroll::WIDGET_ROW)
}

compare_value!(compare_area_get_shown_row, area_get_shown_row, %%I2%%, () -> u32);

/// Stores a requested scroll row (`area_scroll::PENDING_ROW`): at most the last row less the visible rows plus one
/// (`LAST_ROW` − `area_fields::ROW_COUNT` + 1), and at least 0.
/// Original: `Area::set_scroll_row` (0x403320, Area.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn area_set_scroll_row(area: usize, row: i32) {
    let top = read_i32(area + area_scroll::LAST_ROW).wrapping_sub(read_u8(area + area_fields::ROW_COUNT) as i32).wrapping_add(1);
    let row = if row < top { row } else { top };
    write_i32(area + area_scroll::PENDING_ROW, if row > 0 { row } else { 0 }); // tel29_2: never above the first row
}

/// Comparer for `area_set_scroll_row`: side by side over the pending row.
pub(crate) unsafe extern "thiscall" fn compare_area_set_scroll_row(area: usize, row: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I3%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(area + area_scroll::PENDING_ROW, 4)], || o(area, row), || area_set_scroll_row(area, row));
    SCOREBOARD.record(equal, || format!("area_set_scroll_row({}) original={:x?} rust={:x?}", row, mo, mr));
}

/// Sets up the area's scroll bar inside margins `mx`, `my`: the bar, its track and thumb, a pending scroll row applied,
/// and the bar made the keyboard's.
///
/// The bar: x2 = area x2 − `mx`, x1 = x2 − 0x13, y1 = area y1 + `my`, y2 = area y2 − `my`; the track 0x14 inside the
/// arrow buttons. A pending row (`area_scroll::PENDING_ROW` ≠ −1) is clamped to 0..last row, the first visible widget
/// becomes the first whose row is not below it, and it is the scroll row (otherwise 0). Hidden rows = last row − visible
/// rows + 1; the thumb height = track × visible / (hidden + visible), at least 10; its top = track top + (track −
/// thumb + 1) × scroll row / hidden rows, its bottom = top + height − 1 (divisions truncating). The bar's saved image
/// pointer and the pressed arrow are cleared (the image is not freed), the area has a bar, and the keyboard scroll bar
/// ([`G_ACTIVE_AREA_SCROLLBAR`]) is this one.
/// Original: `Area::setup_scrollbar` (0x403600, Area.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn area_setup_scrollbar(area: usize, mx: i32, my: i32) {
    use area_scroll as s;
    covered!("tel29_2 scroll bar set up");
    let x2 = read_i32(area + 8).wrapping_sub(mx);
    write_i32(area + s::BAR_X2, x2);
    write_i32(area + s::BAR_X1, x2.wrapping_sub(0x13));
    let y1 = read_i32(area + 4).wrapping_add(my);
    let y2 = read_i32(area + 0xc).wrapping_sub(my);
    write_i32(area + s::BAR_Y2, y2);
    let top = y1.wrapping_add(0x14);
    let bottom = y2.wrapping_sub(0x14);
    write_i32(area + s::BAR_Y1, y1);
    write_i32(area + s::TRACK_TOP, top);
    write_i32(area + s::TRACK_BOTTOM, bottom);
    if read_i32(area + s::PENDING_ROW) != -1 {
        if read_i32(area + s::PENDING_ROW) < 0 {
            write_i32(area + s::PENDING_ROW, 0);
        }
        if read_i32(area + s::PENDING_ROW) > read_i32(area + s::LAST_ROW) {
            write_i32(area + s::PENDING_ROW, read_i32(area + s::LAST_ROW));
        }
        let mut i: i16 = 0;
        while i < read_i16(area + area_fields::ITEM_COUNT) && read_i32(area_list_item(area, i) + s::WIDGET_ROW) < read_i32(area + s::PENDING_ROW) {
            i += 1;
        }
        write_i16(area + area_fields::FIRST_VISIBLE_ITEM, i);
        write_i32(area + s::SCROLL_ROW, read_i32(area + s::PENDING_ROW));
    } else {
        write_i32(area + s::SCROLL_ROW, 0);
    }
    let rows = read_u8(area + area_fields::ROW_COUNT) as i32;
    let hidden = read_i32(area + s::LAST_ROW).wrapping_sub(rows).wrapping_add(1);
    write_i32(area + s::HIDDEN_ROWS, hidden);
    let thumb = bottom.wrapping_sub(top).wrapping_add(1).wrapping_mul(rows).wrapping_div(hidden.wrapping_add(rows));
    let thumb = if thumb > 10 { thumb } else { 10 }; // tel29_2: the thumb at least 10 pixels
    write_i32(area + s::THUMB_HEIGHT, thumb);
    write_u32(area + s::BAR_IMAGE, 0);
    write_u8(area + s::PRESSED_ARROW, 0);
    write_u32(area + s::HAS_SCROLLBAR, 1);
    let thumb_top = bottom.wrapping_sub(top).wrapping_sub(thumb).wrapping_add(1).wrapping_mul(read_i32(area + s::SCROLL_ROW)).wrapping_div(hidden).wrapping_add(top);
    write_i32(area + s::THUMB_TOP, thumb_top);
    write_i32(area + s::THUMB_BOTTOM, thumb_top.wrapping_add(thumb).wrapping_sub(1));
    write_usize(G_ACTIVE_AREA_SCROLLBAR, area + area_fields::SCROLLBAR);
}

/// Comparer for `area_setup_scrollbar`: side by side over the area and the keyboard scroll bar.
pub(crate) unsafe extern "thiscall" fn compare_area_setup_scrollbar(area: usize, mx: i32, my: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(%%I4%%);
    let regions = [(area, area_scroll::AREA_SIZE), (G_ACTIVE_AREA_SCROLLBAR, 4)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(area, mx, my), || area_setup_scrollbar(area, mx, my));
    SCOREBOARD.record(equal, || format!("area_setup_scrollbar {:#x}({}, {}): differences em {:?}", area, mx, my, first_differences(&mo, &mr)));
}

/// Where a popup area opens (its top left corner in `*out_x`, `*out_y`), beside or below the widget it belongs to,
/// kept inside the screen.
///
/// The owner widget (`area_fields::OWNER_ITEM`) is w. When w's own area (`area_scroll::WIDGET_AREA`) is itself a popup
/// that does not open to the side (flag `POPUP_BESIDE`): vertically at w's top when the area fits below it (its height
/// + w's y1 below the screen height 600), otherwise ending at w's bottom (at least 0); horizontally right of w (x2 + 3)
/// when the parent popup's widget (x2) is left of the parent area (x1), otherwise left of w (w x1 − width − 3). With
/// `POPUP_BESIDE` on this area: vertically under w (y2 + 3) when it fits, otherwise above it (w y1 − height − 3);
/// horizontally at w's x1 when w's x1 ≤ 400, otherwise ending at w's x2. Otherwise: vertically at w's y1 when (the
/// height fits below it or `POPUP_KEEP_DOWN` is set) and `POPUP_ABOVE` is not set, else ending at w's y2 (at least 0);
/// horizontally right of w (x2 + 3) when (w's x1 ≤ 400 or `POPUP_RIGHT_ANYWAY`) and not `POPUP_LEFT`, else left of w
/// (w x1 − width − 3). The 600 is read from the exe (the 1080
/// exe rewrites it); the 400 is fixed in both. Without an owner widget: the original routine runs (error box `Area.cpp
/// 718`).
/// Original: `Area::get_popup_position` (0x403760, Area.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn area_get_popup_position(area: usize, out_x: *mut i32, out_y: *mut i32) {
    use area_scroll as s;
    let owner = read_i16(area + area_fields::OWNER_ITEM);
    if owner == -1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, *mut i32, *mut i32)>(%%I5%%)(area, out_x, out_y);
    }
    let w = area_widget(area, owner);
    let (x1, y1, x2, y2) = (read_i32(area), read_i32(area + 4), read_i32(area + 8), read_i32(area + 0xc));
    let height = y2.wrapping_sub(y1);
    let clamp0 = |v: i32| if v > 0 { v } else { 0 };
    let parent_index = read_i16(w + s::WIDGET_AREA);
    let parent = if parent_index != -1 { read_usize(area + area_fields::AREAS).wrapping_add((parent_index as isize * s::AREA_SIZE as isize) as usize) } else { 0 };
    if parent != 0 && read_i16(parent + area_fields::OWNER_ITEM) != -1 && read_u32(parent + area_fields::FLAGS) & s::POPUP_BESIDE == 0 {
        covered!("tel29_2 popup from a popup");
        let wy1 = read_i32(w + s::WIDGET_Y1);
        *out_y = if height.wrapping_add(wy1) < popup_screen_height(0) { wy1 } else { clamp0(read_i32(w + s::WIDGET_Y2).wrapping_sub(y2).wrapping_add(y1)) };
        let grand = area_widget(area, read_i16(parent + area_fields::OWNER_ITEM));
        *out_x = if read_i32(grand + s::WIDGET_X2) < read_i32(parent) {
            read_i32(w + s::WIDGET_X2).wrapping_add(3)
        } else {
            read_i32(w + s::WIDGET_X1).wrapping_sub(x2).wrapping_add(x1).wrapping_sub(3)
        };
        return;
    }
    let flags = read_u32(area + area_fields::FLAGS);
    if flags & s::POPUP_BESIDE != 0 {
        covered!("tel29_2 popup beside");
        let wy2 = read_i32(w + s::WIDGET_Y2);
        *out_y = if wy2.wrapping_sub(y1).wrapping_add(y2).wrapping_add(3) < popup_screen_height(1) {
            wy2.wrapping_add(3)
        } else {
            read_i32(w + s::WIDGET_Y1).wrapping_sub(y2).wrapping_add(y1).wrapping_sub(3)
        };
        let wx1 = read_i32(w + s::WIDGET_X1);
        *out_x = if wx1 <= 400 { wx1 } else { read_i32(w + s::WIDGET_X2).wrapping_sub(x2).wrapping_add(x1) };
        return;
    }
    covered!("tel29_2 popup below");
    let wy1 = read_i32(w + s::WIDGET_Y1);
    let fits = height.wrapping_add(wy1) < popup_screen_height(2);
    *out_y = if (fits || flags & s::POPUP_KEEP_DOWN != 0) && flags & s::POPUP_ABOVE == 0 {
        wy1
    } else {
        clamp0(read_i32(w + s::WIDGET_Y2).wrapping_sub(y2).wrapping_add(y1))
    };
    let wx1 = read_i32(w + s::WIDGET_X1);
    let flags = read_u32(area + area_fields::FLAGS);
    *out_x = if (wx1 <= 400 || flags & s::POPUP_RIGHT_ANYWAY != 0) && flags & s::POPUP_LEFT == 0 { // tel29_2: right of the widget
        read_i32(w + s::WIDGET_X2).wrapping_add(3)
    } else {
        wx1.wrapping_sub(x2).wrapping_add(x1).wrapping_sub(3)
    };
}

/// Comparer for `area_get_popup_position`: side by side over the two outputs.
pub(crate) unsafe extern "thiscall" fn compare_area_get_popup_position(area: usize, out_x: *mut i32, out_y: *mut i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut i32, *mut i32)>(%%I5%%);
    if read_i16(area + area_fields::OWNER_ITEM) == -1 {
        return o(area, out_x, out_y);
    }
    let regions = [(out_x as usize, 4), (out_y as usize, 4)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(area, out_x, out_y), || area_get_popup_position(area, out_x, out_y));
    SCOREBOARD.record(equal, || format!("area_get_popup_position {:#x}: original={:x?} rust={:x?}", area, mo, mr));
}

/// Moves the area's rectangle so its top left corner is at (`x`, `y`).
///
/// When the move would take it off the screen (x < 0, y < 0, or past the right or bottom edge, 0x31f and 0x257 in the
/// 800 × 600 exe, read from the exe since the 1080 exe rewrites them): the original routine runs (error boxes `Area.cpp
/// 823`, `828`, `833`, `838`, and the move clamped to the screen). Otherwise all four corners shift by the difference
/// (nothing written when it is zero).
/// Original: `Area::move_to` (0x403a70, Area.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn area_move_to(area: usize, x: i32, y: i32) {
    let dx = x.wrapping_sub(read_i32(area));
    let dy = y.wrapping_sub(read_i32(area + 4));
    if x < 0 || read_i32(area + 8).wrapping_add(dx) > (0x403b02 as *const i32).read_unaligned() || y < 0 || read_i32(area + 0xc).wrapping_add(dy) > (0x403bde as *const i32).read_unaligned() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(%%I6%%)(area, x, y);
    }
    if dx == 0 && dy == 0 {
        return;
    }
    covered!("tel29_2 area moved");
    write_i32(area, read_i32(area).wrapping_add(dx));
    write_i32(area + 4, read_i32(area + 4).wrapping_add(dy));
    write_i32(area + 8, read_i32(area + 8).wrapping_add(dx));
    write_i32(area + 0xc, read_i32(area + 0xc).wrapping_add(dy)); // tel29_2: the bottom moves with the top
}

/// Comparer for `area_move_to`: side by side over the rectangle (off the screen, only the original: the boxes).
pub(crate) unsafe extern "thiscall" fn compare_area_move_to(area: usize, x: i32, y: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(%%I6%%);
    let dx = x.wrapping_sub(read_i32(area));
    let dy = y.wrapping_sub(read_i32(area + 4));
    if x < 0 || read_i32(area + 8).wrapping_add(dx) > (0x403b02 as *const i32).read_unaligned() || y < 0 || read_i32(area + 0xc).wrapping_add(dy) > (0x403bde as *const i32).read_unaligned() {
        return o(area, x, y);
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(area, 16)], || o(area, x, y), || area_move_to(area, x, y));
    SCOREBOARD.record(equal, || format!("area_move_to {:#x}({}, {}): original={:x?} rust={:x?}", area, x, y, mo, mr));
}

/// Draws the area's scroll bar: the up button with its arrow, the darkened track, the thumb and the down button with
/// its arrow, after saving the screen behind the bar.
///
/// Without a bar (`area_scroll::HAS_SCROLLBAR`): the original routine runs (error box `Area.cpp 870`). The image behind
/// the bar is saved once (`exe::gui_save_screen_image`, `BAR_IMAGE`). The up button: a panel from the bar's top to the
/// track top − 1 (`exe::gui_draw_panel`, style 0x70 and the arrow 2 pixels in when the up arrow is pressed,
/// `PRESSED_ARROW` = 1, otherwise 0x30), colour [`G_GUI_COLOUR_GREY`]; its arrow (`GLYPH_UP`, gui_draw_glyph 0x5e3ce0,
/// colour [`G_GUI_COLOUR_BLACK`]) centred in the 20-pixel button. The track (style 2, black), the thumb (`THUMB_TOP`..
/// `THUMB_BOTTOM`, style 0x30, grey); the down button from the track bottom + 1 to the bar's bottom (pressed = 2), its
/// arrow (`GLYPH_DOWN`, centred with the up arrow's width and height, as the original does).
/// Original: `Area::draw_scrollbar` (0x403c80, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_draw_scrollbar(area: usize) {
    use area_scroll as s;
    if read_u32(area + s::HAS_SCROLLBAR) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(area);
    }
    covered!("tel29_2 scroll bar drawn");
    let glyph: unsafe extern "C" fn(i32, i32, u32, usize) = core::mem::transmute(0x5e3ce0usize);
    let r = |f: usize| read_i32(area + f);
    if read_usize(area + s::BAR_IMAGE) == 0 {
        write_usize(area + s::BAR_IMAGE, exe::gui_save_screen_image(r(s::BAR_X1), r(s::BAR_Y1), r(s::BAR_X2), r(s::BAR_Y2), 0usize));
    }
    // simplification: the original pushes the colours from 16-bit loads into registers with leftover high bits; here
    // zero-extended
    let grey = read_u16(G_GUI_COLOUR_GREY) as u32;
    let black = read_u16(G_GUI_COLOUR_BLACK) as u32;
    let centre = |size_byte: usize| (0x14 - read_u8(size_byte) as i32) / 2;
    let (off, style) = if read_u8(area + s::PRESSED_ARROW) == 1 { (2, 0x70) } else { (0, 0x30) };
    exe::gui_draw_panel(r(s::BAR_X1), r(s::BAR_Y1), r(s::BAR_X2), r(s::TRACK_TOP) - 1, style, grey, 0);
    glyph(centre(s::GLYPH_UP) + r(s::BAR_X1) + off, centre(s::GLYPH_UP + 1) + r(s::BAR_Y1) + off, black, s::GLYPH_UP);
    exe::gui_draw_panel(r(s::BAR_X1), r(s::TRACK_TOP), r(s::BAR_X2), r(s::TRACK_BOTTOM), 2, black, 0);
    exe::gui_draw_panel(r(s::BAR_X1), r(s::THUMB_TOP), r(s::BAR_X2), r(s::THUMB_BOTTOM), 0x30, grey, 0);
    let (off, style) = if read_u8(area + s::PRESSED_ARROW) == 2 { (2, 0x70) } else { (0, 0x30) };
    exe::gui_draw_panel(r(s::BAR_X1), r(s::TRACK_BOTTOM) + 1, r(s::BAR_X2), r(s::BAR_Y2), style, grey, 0);
    glyph(centre(s::GLYPH_UP) + r(s::BAR_X1) + off, centre(s::GLYPH_UP + 1) + r(s::TRACK_BOTTOM) + off + 1, black, s::GLYPH_DOWN);
}

// simplification: draws on the screen and saves the image behind the bar (allocates): it does not run twice. Original
// only; the copy is proven in Lucas's game in Rust mode (counter "tel29_2 scroll bar drawn" in ddraw.log).
/// Comparer for `area_draw_scrollbar`.
pub(crate) unsafe extern "thiscall" fn compare_area_draw_scrollbar(area: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(area)
}
