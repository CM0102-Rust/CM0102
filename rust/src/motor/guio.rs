use super::*;

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
    let o = original_fn::<F>(IDX_GUIO_GET_FIELDS);
    let outs = [(flags, 4), (x1, 4), (y1, 4), (x2, 4), (y2, 4), (f20, 4), (row, 4), (style, 4), (colour, 2), (highlight, 2), (f3c, 4), (font, 2),
        (text_colour, 2), (text, 4), (max_length, 4), (id, 4), (f48, 4), (area, 2), (target, 4)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&outs,
        || o(item, flags, x1, y1, x2, y2, f20, row, style, colour, highlight, f3c, font, text_colour, text, max_length, id, f48, area, target),
        || guio_get_fields(item, flags, x1, y1, x2, y2, f20, row, style, colour, highlight, f3c, font, text_colour, text, max_length, id, f48, area, target));
    SCOREBOARD.record(equal, || format!("guio_get_fields({:#x}): differences in {:?}", item, first_differences(&mo, &mr)));
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

/// Comparer for `guio_highlight_on`.
pub(crate) unsafe extern "thiscall" fn compare_guio_highlight_on(item: usize) {
    use display_item as d;
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GUIO_HIGHLIGHT_ON)(item);
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

/// Comparer for `guio_highlight_off`.
pub(crate) unsafe extern "thiscall" fn compare_guio_highlight_off(item: usize) {
    use display_item as d;
    let area = read_i16(item + d::AREA);
    let was = area != -1 && read_u16(guio_area(item, area) + display_area::HIGHLIGHTED_ITEM) == read_u16(item + guio_fields::INDEX);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GUIO_HIGHLIGHT_OFF)(item);
    let ok = read_u32(item + d::HIGHLIGHTED) == 0 && read_u32(item + d::HOVER_TIME) == read_u32(0xb67a08)
        && (!was || read_i16(guio_area(item, area) + display_area::HIGHLIGHTED_ITEM) == -1);
    SCOREBOARD.record(ok, || format!("guio_highlight_off({:#x}): highlighted {} area {}", item, read_u32(item + d::HIGHLIGHTED), area));
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
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_GUIO_SET_TEXT_AND_REFRESH)(item, text, refresh);
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

/// Comparer for `guio_set_text_and_refresh`.
pub(crate) unsafe extern "thiscall" fn compare_guio_set_text_and_refresh(item: usize, text: usize, refresh: u32) {
    use display_item as d;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_GUIO_SET_TEXT_AND_REFRESH);
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

/// Comparer for `guio_free_images`.
pub(crate) unsafe extern "thiscall" fn compare_guio_free_images(item: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GUIO_FREE_IMAGES)(item);
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
    let split: unsafe extern "C" fn(u32, *mut u32, *mut u32, *mut u32, *const u32) = core::mem::transmute(FN_GUI_COLOUR_TO_RGB);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_GUIO_CONVERT_COLOURS_PIXEL_FORMAT);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(item + guio_fields::COLOUR, 8)], || o(item, to_565, which),
        || guio_convert_colours_pixel_format(item, to_565, which));
    SCOREBOARD.record(equal, || format!("guio_convert_colours_pixel_format({:#x}, {}, {}): differences in {:?}", item, to_565, which, first_differences(&mo, &mr)));
}

/// Rule: a new widget gets its fields: its rectangle put in order and kept inside the screen, and, in an area that
/// indents (`guio_area_fields::INDENT_MODE` ≠ −1), its text indented, its column and row computed and the area grown
/// to fit it.
///
/// `AREAS`, `ITEMS`, `ODD_ROW` = 0. A rectangle with x1 > x2 (y1 > y2), unless style 0x400 without 0x4000, has the two
/// swapped; x1 and y1 below 0 go to 0, x2 and y2 at or above the screen's last column and row (799 and 599, read from
/// the exe: 1919 and 1079 in the 1080 exe) go to it. In an indenting area: the text ("" for null) is written as
/// "     %s"; the area's `X2` grows to the text width (`exe::font_text_width` + 15, at least 125); the column is 0 with
/// one column, otherwise the column after that of the area's last item (walking its list with `display_area::WALK`)
/// modulo the columns; the row is the area's `ROWS_ADDED` (odd: `ODD_ROW` = 1), which goes up; the area's `Y2` grows by
/// the font height + 6. Otherwise the text is copied as it is (no length check) and the row is field 6. Then: `INDEX`,
/// the flags, the rectangle (twice: also in `HOME_*`), the column, the row, the style, the colours (words), field 10,
/// the font, the length limit, the id, field 16, the area, `CURSOR` = −1, `CHANGED` = 0, `BACK_IMAGE` = 0, `POPUP_AREA`
/// = −1, `HIGHLIGHTED` = 0, `HOVER_TIME` = the tick, `DROP_TARGET` = −1, `TEXT_IMAGE` = 0, `ENTRY_TARGET` = the text
/// for a text entry (flag 8) with one, else 0; a text entry with flag 0x10 has its cursor at the end of the text.
/// Original: `Guio::init` (0x5ee150, Guio.cpp, thiscall, ret 0x54).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn guio_init(item: usize, areas: usize, items: usize, index: u32, flags: u32, x1: i32, y1: i32, x2: i32, y2: i32,
    column: i32, row: i32, style: u32, colour: u32, highlight: u32, field_10: u32, font: u32, text_colour: u32, text: usize, max_length: u32,
    id: u32, field_16: u32, area: u32) {
    use display_item as d;
    use guio_area_fields as af;
    use guio_fields as g;
    use guio_more as m;
    let (mut x1, mut y1, mut x2, mut y2, mut column, mut row) = (x1, y1, x2, y2, column, row);
    write_usize(item + m::AREAS, areas);
    write_usize(item + m::ITEMS, items);
    write_u32(item + m::ODD_ROW, 0);
    let swaps = style & 0x400 == 0 || style & 0x4000 != 0;
    if x1 > x2 && swaps {
        core::mem::swap(&mut x1, &mut x2);
    }
    if y1 > y2 && swaps {
        core::mem::swap(&mut y1, &mut y2);
    }
    x1 = x1.max(0);
    y1 = y1.max(0);
    let (last_x, last_y) = (exe_immediate(0x5ee1f6 as *const i32), exe_immediate(0x5ee20b as *const i32));
    if x2 >= last_x {
        x2 = last_x; // tel38_7: kept inside the screen
    }
    if y2 >= last_y {
        y2 = last_y;
    }
    let a = area as i16;
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    if a != -1 && read_i16(guio_area_of(item, a) + af::INDENT_MODE) != -1 {
        covered!("tel38_7 area item");
        let ar = guio_area_of(item, a);
        format(item + d::TEXT, GUIO_INDENT_FORMAT, if text == 0 { STR_EMPTY } else { text });
        let width = (exe::font_text_width(font as usize, (item + d::TEXT) as *const u8) + 0xf).max(0x7d);
        if width > read_i32(ar + af::X2) {
            write_i32(ar + af::X2, width);
        }
        let columns = read_u8(ar + af::COLUMNS);
        if columns == 1 {
            column = 0;
        } else {
            let mut last = 0i32;
            write_u16(ar + display_area::WALK, 0);
            loop {
                let w = read_i16(ar + display_area::WALK);
                if w >= read_i16(ar + af::ITEM_LIST_COUNT) {
                    break;
                }
                let k = read_i16(ar + af::ITEM_LIST + w as usize * 2);
                write_i16(ar + display_area::WALK, w + 1);
                if k == -1 {
                    break;
                }
                last = read_i32((items as isize + k as isize * display_list::ITEM_SIZE as isize) as usize + m::COLUMN);
            }
            column = (last + 1) % columns as i32; // tel38_7: the column after the last item's
        }
        let added = read_u8(ar + af::ROWS_ADDED);
        let height = exe::font_get_height(font as i32);
        write_i32(ar + af::Y2, height + read_i32(ar + af::Y2) + 6);
        if added & 1 != 0 { // tel38_7: the odd rows
            write_u32(item + m::ODD_ROW, 1);
        }
        write_u8(ar + af::ROWS_ADDED, added.wrapping_add(1));
        row = added as i32;
    } else {
        copy_c_string(item + d::TEXT, if text == 0 { STR_EMPTY } else { text });
    }
    write_u16(item + g::INDEX, index as u16);
    write_u32(item + d::FLAGS, flags);
    write_i32(item + d::X1, x1);
    write_i32(item + m::HOME_X1, x1);
    write_i32(item + d::Y1, y1);
    write_i32(item + m::HOME_Y1, y1);
    write_u16(item + g::COLOUR, colour as u16);
    write_i32(item + d::Y2, y2);
    write_i32(item + m::HOME_Y2, y2);
    write_u32(item + d::STYLE, style);
    write_u16(item + d::FONT, font as u16);
    write_i32(item + d::X2, x2);
    write_i32(item + m::HOME_X2, x2);
    write_i32(item + m::COLUMN, column);
    write_u16(item + g::HIGHLIGHT_COLOUR, highlight as u16);
    write_u32(item + m::FIELD_10, field_10);
    write_u32(item + d::ID, id);
    write_u16(item + g::TEXT_COLOUR, text_colour as u16);
    write_u32(item + d::MAX_LENGTH, max_length);
    write_u32(item + m::FIELD_16, field_16);
    write_i32(item + d::ROW, row);
    write_i32(item + d::CURSOR, -1);
    write_u32(item + d::CHANGED, 0);
    write_usize(item + d::BACK_IMAGE, 0);
    write_u16(item + d::AREA, area as u16);
    write_i16(item + m::POPUP_AREA, -1);
    write_u32(item + d::HIGHLIGHTED, 0);
    write_u32(item + d::HOVER_TIME, read_u32(DISPLAY_TICKS));
    write_i16(item + d::DROP_TARGET, -1);
    write_usize(item + g::TEXT_IMAGE, 0);
    let entry = flags & 8 != 0;
    write_usize(item + g::ENTRY_TARGET, if text != 0 && entry { text } else { 0 });
    if entry && flags & 0x10 != 0 {
        write_i32(item + d::CURSOR, c_string_len(item + d::TEXT) as i32);
    }
}

/// Comparer for `guio_init`: side by side over the widget and, for an area item, its area.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_guio_init(item: usize, areas: usize, items: usize, index: u32, flags: u32, x1: i32, y1: i32, x2: i32,
    y2: i32, column: i32, row: i32, style: u32, colour: u32, highlight: u32, field_10: u32, font: u32, text_colour: u32, text: usize,
    max_length: u32, id: u32, field_16: u32, area: u32) {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, i32, i32, i32, i32, i32, i32, u32, u32, u32, u32, u32, u32, usize, u32, u32,
        u32, u32);
    let o = original_fn::<F>(IDX_GUIO_INIT);
    let mut regions = vec![(item, display_list::ITEM_SIZE)];
    let a = area as i16;
    if (0..0xf9).contains(&a) {
        regions.push(((areas as isize + a as isize * display_list::AREA_SIZE as isize) as usize, display_list::AREA_SIZE));
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions,
        || o(item, areas, items, index, flags, x1, y1, x2, y2, column, row, style, colour, highlight, field_10, font, text_colour, text, max_length, id,
            field_16, area),
        || guio_init(item, areas, items, index, flags, x1, y1, x2, y2, column, row, style, colour, highlight, field_10, font, text_colour, text,
            max_length, id, field_16, area));
    SCOREBOARD.record(equal, || format!("guio_init(item {}, area {}): differences in {:?}", index as i16, a, first_differences(&mo, &mr)));
}

/// Rule: draws a widget: its panel, its picture, the arrow towards the popup it opens, its marks and its text, pressed
/// down 2 pixels when highlighted with style 0x20 (or with style 0x40).
///
/// An odd area row (`guio_more::ODD_ROW`) darkens its colour once (`gui_scale_colour` 0x5e4890, 110%, the flag back to
/// 0). With `refresh` the saved background is freed; with no style 0x10 and none saved, the screen under the widget is
/// saved (`exe::gui_save_screen_image`). A picture widget (flag 0x400) with no image: the shared image when its text
/// is the shared name and there is one (the count goes up), otherwise the picture is loaded (`gui_load_rgn_picture`
/// 0x5e40d0) and, when nobody shares the current one (count 0), it becomes the shared image (the old one freed, the
/// name copied, the count 1). The image is put on the screen. The panel (`exe::gui_draw_panel`: the rectangle, the
/// style, the colour, the text colour; highlighted: the highlight colour and, with style 0x20 and no flag 4, style
/// 0x40; style 0x40 presses it 2 pixels right and down). With a popup area and no flag 0x80: the popup's position
/// (`Area::get_popup_position`); its bottom above the widget: the up arrow; its top below the widget: the down
/// arrow; else its right edge left of the widget: the left arrow; else its left edge right of it: the right arrow
/// (`guio_popup_arrow`, colour `guio_arrow_colour`). Flag 0x2000: a glyph at the left edge. Flags 0x4000, 0x20000,
/// 0x40000: the marks '+', ',' and '-' (`guio_mark`). A widget without flag 0x400: with flag 0x800 the text is cut at
/// its first byte 1 while drawn; the text box (`gui_draw_text_box`: the rectangle pressed, field 10, the font, the
/// text colour, the text, the cursor), then the byte 1 comes back.
/// Original: `Guio::draw` (0x5ee590, Guio.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn guio_draw(item: usize, refresh: u32) {
    use display_item as d;
    use guio_fields as g;
    use guio_more as m;
    if read_u32(item + m::ODD_ROW) != 0 {
        write_u32(item + m::ODD_ROW, 0); // tel38_7: the darkening only once
        write_u16(item + g::COLOUR, exe::gui_scale_colour(read_u16(item + g::COLOUR) as u32, 0x6e, 0)); // tel38_7: an odd row darkened
    }
    if read_usize(item + d::BACK_IMAGE) != 0 && refresh != 0 {
        exe::gui_free_image(read_usize(item + d::BACK_IMAGE));
        write_usize(item + d::BACK_IMAGE, 0);
    }
    if read_u8(item + d::STYLE) & 0x10 == 0 && read_usize(item + d::BACK_IMAGE) == 0 {
        let image = exe::gui_save_screen_image(read_i32(item + d::X1), read_i32(item + d::Y1), read_i32(item + d::X2), read_i32(item + d::Y2), 0usize);
        write_usize(item + d::BACK_IMAGE, image);
    }
    if read_u32(item + d::FLAGS) & 0x400 != 0 && read_usize(item + g::TEXT_IMAGE) == 0 {
        covered!("tel38_7 picture widget");
        if guio_same_c_string(GUIO_SHARED_IMAGE_NAME, item + d::TEXT) && read_usize(GUIO_SHARED_IMAGE) != 0 {
            write_usize(item + g::TEXT_IMAGE, read_usize(GUIO_SHARED_IMAGE));
            write_i16(GUIO_SHARED_IMAGE_COUNT, read_i16(GUIO_SHARED_IMAGE_COUNT).wrapping_add(1));
        } else {
            let image = exe::gui_load_rgn_picture(item + d::TEXT, 0);
            write_usize(item + g::TEXT_IMAGE, image);
            if image != 0 && read_i16(GUIO_SHARED_IMAGE_COUNT) == 0 {
                let old = read_usize(GUIO_SHARED_IMAGE);
                if old != 0 {
                    exe::gui_free_image(old);
                }
                write_usize(GUIO_SHARED_IMAGE, read_usize(item + g::TEXT_IMAGE));
                copy_c_string(GUIO_SHARED_IMAGE_NAME, item + d::TEXT);
                write_u16(GUIO_SHARED_IMAGE_COUNT, 1);
            }
        }
    }
    let image = read_usize(item + g::TEXT_IMAGE);
    if image != 0 {
        gui_restore_background(read_u32(item + d::X1), read_u32(item + d::Y1), image);
    }
    let mut style = read_u32(item + d::STYLE);
    let mut colour = read_u16(item + g::COLOUR);
    if read_u32(item + d::HIGHLIGHTED) != 0 {
        colour = read_u16(item + g::HIGHLIGHT_COLOUR);
        if style & 0x20 != 0 && read_u32(item + d::FLAGS) & 4 == 0 {
            style |= 0x40; // tel38_7: a highlighted button pressed
        }
    }
    let (dx, dy) = if style & 0x40 != 0 { (2, 2) } else { (0, 0) };
    exe::gui_draw_panel(read_i32(item + d::X1), read_i32(item + d::Y1), read_i32(item + d::X2), read_i32(item + d::Y2), style, colour as u32,
        read_u16(item + g::TEXT_COLOUR) as u32);
    let flags = read_u32(item + d::FLAGS);
    let popup = read_i16(item + m::POPUP_AREA);
    if popup != -1 && flags & 0x80 == 0 {
        use guio_area_fields as af;
        let ar = guio_area_of(item, popup);
        let (mut px, mut py) = (0i32, 0i32);
        exe::area_get_popup_position(ar, &mut px, &mut py);
        let arrow = guio_arrow_colour(item);
        let ar = guio_area_of(item, read_i16(item + m::POPUP_AREA));
        if read_i32(ar + af::Y2) - read_i32(ar + af::Y1) + py < read_i32(item + d::Y1) {
            guio_popup_arrow(item, GLYPH_POPUP_ABOVE, false, dx, dy, arrow);
        } else if py > read_i32(item + d::Y2) {
            guio_popup_arrow(item, GLYPH_POPUP_BELOW, false, dx, dy, arrow);
        } else if read_i32(ar + af::X2) - read_i32(ar + af::X1) + px < read_i32(item + d::X1) {
            guio_popup_arrow(item, GLYPH_POPUP_LEFT, true, dx, dy, arrow);
        } else if px > read_i32(item + d::X2) {
            guio_popup_arrow(item, GLYPH_POPUP_RIGHT, false, dx, dy, arrow);
        }
    }
    if flags & 0x2000 != 0 {
        let h = read_u8(GLYPH_FLAG_2000 + 1) as i32;
        guio_glyph(read_i32(item + d::X1) + dx + 4, guio_centre(read_i32(item + d::Y1), read_i32(item + d::Y2), h) + dy, guio_arrow_colour(item),
            GLYPH_FLAG_2000);
    }
    if flags & 0x4000 != 0 {
        guio_mark(item, 0x2b, GLYPH_FLAG_4000, read_u16(GUIO_MARK_COLOUR) as u32, dx, dy);
    }
    if read_u32(item + d::FLAGS) & 0x20000 != 0 {
        guio_mark(item, 0x2c, GLYPH_FLAG_20000, read_u16(GUIO_MARK_COLOUR) as u32, dx, dy);
    }
    if read_u32(item + d::FLAGS) & 0x40000 != 0 {
        guio_mark(item, 0x2d, GLYPH_FLAG_40000, read_u16(G_GUI_COLOUR_RED) as u32, dx, dy);
    }
    let flags = read_u32(item + d::FLAGS);
    if flags & 0x400 != 0 {
        return;
    }
    let mut cut = None;
    if flags & 0x800 != 0 {
        let mut k = 0;
        while read_u8(item + d::TEXT + k) != 1 && read_u8(item + d::TEXT + k) != 0 {
            k += 1;
        }
        if read_u8(item + d::TEXT + k) == 1 {
            write_u8(item + d::TEXT + k, 0);
            cut = Some(k);
        }
    }
    guio_text_box(read_i32(item + d::X1) + dx, read_i32(item + d::Y1) + dy, read_i32(item + d::X2) + dx, read_i32(item + d::Y2) + dy,
        read_u32(item + m::FIELD_10), read_u16(item + d::FONT) as u32, read_u16(item + g::TEXT_COLOUR) as u32, item + d::TEXT, read_i32(item + d::CURSOR));
    if let Some(k) = cut {
        write_u8(item + d::TEXT + k, 1);
    }
}

/// Comparer for `guio_draw`.
pub(crate) unsafe extern "thiscall" fn compare_guio_draw(item: usize, refresh: u32) {
    let text = core::slice::from_raw_parts((item + display_item::TEXT) as *const u8, 0x100).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_GUIO_DRAW)(item, refresh);
    let after = core::slice::from_raw_parts((item + display_item::TEXT) as *const u8, 0x100);
    let odd = read_u32(item + guio_more::ODD_ROW);
    SCOREBOARD.record(odd == 0 && after == &text[..], || format!("guio_draw({:#x}): odd row {} text changed {}", item, odd, after != &text[..]));
}

/// `restore_background(x, y, image)` (0x5e3fd0, cdecl): puts back the screen under a widget.
pub(crate) unsafe fn gui_restore_background(x: u32, y: u32, image: usize) {
    exe::gui_restore_screen_image((x) as u32 as i32, (y) as u32 as i32, image);
}

/// `update_screen_rect(x1, y1, x2, y2)` (0x5e2eb0, cdecl): shows a rectangle of the back buffer.
pub(crate) unsafe fn gui_update_rect(item: usize) {
    use display_item as d;
    exe::graphics_flip_rect(read_u32(item + d::X1), read_u32(item + d::Y1), read_u32(item + d::X2), read_u32(item + d::Y2));
}

/// Area `a` of a widget's Display (the widget keeps the area table's address at +0).
pub(crate) unsafe fn guio_area(item: usize, a: i16) -> usize {
    (read_usize(item) as isize + a as isize * display_list::AREA_SIZE as isize) as usize
}

/// The redraw shared by the highlight routines and `guio_set_text_and_refresh`: the saved background back
/// (`gui_restore_background`, when there is one), `Guio::draw(item, 0)` and the rectangle shown.
pub(crate) unsafe fn guio_redraw(item: usize) {
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
pub(crate) unsafe fn guio_highlight_shows(item: usize) -> bool {
    use guio_fields as g;
    if read_u8(item + display_item::STYLE) & 0x30 == 0 {
        return false;
    }
    !(read_u32(item + display_item::FLAGS) & 4 != 0 && read_u16(item + g::COLOUR) == read_u16(item + g::HIGHLIGHT_COLOUR))
}

/// Whether two C strings are equal (the original's inline strcmp).
pub(crate) unsafe fn guio_same_c_string(a: usize, b: usize) -> bool {
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
pub(crate) unsafe fn guio_drop_text_image(item: usize) {
    let image = read_usize(item + guio_fields::TEXT_IMAGE);
    if guio_same_c_string(GUIO_SHARED_IMAGE_NAME, item + display_item::TEXT) {
        write_i16(GUIO_SHARED_IMAGE_COUNT, read_i16(GUIO_SHARED_IMAGE_COUNT).wrapping_sub(1));
    } else {
        exe::gui_free_image(image);
    }
    write_usize(item + guio_fields::TEXT_IMAGE, 0);
}

/// Area `a` of a widget's area table (`guio_more::AREAS`).
pub(crate) unsafe fn guio_area_of(item: usize, a: i16) -> usize {
    (read_usize(item + guio_more::AREAS) as isize + a as isize * display_list::AREA_SIZE as isize) as usize
}

/// `gui_draw_glyph(x, y, colour, glyph)` (0x5e3ce0, cdecl): a one-colour glyph on the back buffer.
pub(crate) unsafe fn guio_glyph(x: i32, y: i32, colour: u32, glyph: usize) {
    exe::gui_draw_glyph(x, y, colour, glyph);
}

/// `gui_draw_text_box(x1, y1, x2, y2, flags, font, colour, text, caret)` (0x5e6d80, cdecl): word-wrapped text.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn guio_text_box(x1: i32, y1: i32, x2: i32, y2: i32, flags: u32, font: u32, colour: u32, text: usize, caret: i32) {
    exe::gui_draw_text_box(x1, y1, x2, y2, flags, font, colour, text, caret);
}

/// Where a glyph of size `size` is centred between `lo` and `hi` (the original's halving rounds toward zero).
pub(crate) fn guio_centre(lo: i32, hi: i32, size: i32) -> i32 {
    (hi - size - lo + 1) / 2 + lo
}

/// A popup arrow at the widget's right edge (`at_left`: its left edge) when it has a text, otherwise centred; centred
/// in height; `dx`, `dy` = the pressed offset.
pub(crate) unsafe fn guio_popup_arrow(item: usize, glyph: usize, at_left: bool, dx: i32, dy: i32, colour: u32) {
    use display_item as d;
    let (w, h) = (read_u8(glyph) as i32, read_u8(glyph + 1) as i32);
    let (x1, x2) = (read_i32(item + d::X1), read_i32(item + d::X2));
    let x = if c_string_len(item + d::TEXT) != 0 {
        if at_left { x1 + dx + 4 } else { x2 - w + dx - 4 }
    } else {
        guio_centre(x1, x2, w) + dx
    };
    guio_glyph(x, guio_centre(read_i32(item + d::Y1), read_i32(item + d::Y2), h) + dy, colour, glyph);
}

/// One of the marks of flags 0x4000, 0x20000 and 0x40000: with the symbol font (`G_FONT_MODE` ≠ −1) the symbol `code`
/// as a text box (flags 0x48, font 0, the text colour) over the widget less 4 at the right; otherwise `glyph` at the
/// right edge in `colour`.
pub(crate) unsafe fn guio_mark(item: usize, code: u32, glyph: usize, colour: u32, dx: i32, dy: i32) {
    use display_item as d;
    let (x1, y1, x2, y2) = (read_i32(item + d::X1), read_i32(item + d::Y1), read_i32(item + d::X2), read_i32(item + d::Y2));
    if read_i32(G_FONT_MODE) != -1 {
        let mut symbol = [0u8; 4];
        let format: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        format(symbol.as_mut_ptr(), 0x99ec1c, code);
        guio_text_box(x1 + dx, y1 + dy, x2 + dx - 4, y2 + dy, 0x48, 0, read_u16(item + guio_fields::TEXT_COLOUR) as u32, symbol.as_ptr() as usize, -1);
    } else {
        let (w, h) = (read_u8(glyph) as i32, read_u8(glyph + 1) as i32);
        guio_glyph(x2 - w + dx - 4, guio_centre(y1, y2, h) + dy, colour, glyph);
    }
}

/// The arrow colour: `G_GUI_COLOUR_BLACK`, or `G_GUI_COLOUR_PALE_GREY` on a widget of that colour.
pub(crate) unsafe fn guio_arrow_colour(item: usize) -> u32 {
    let black = read_u16(G_GUI_COLOUR_BLACK);
    (if read_u16(item + guio_fields::COLOUR) == black { read_u16(G_GUI_COLOUR_PALE_GREY) } else { black }) as u32
}

/// The shared text image's name (a widget whose text is this name shares the image and only counts it down) and its
/// count (word) [inference].
pub(crate) const GUIO_SHARED_IMAGE_NAME: usize = 0xae2920;

pub(crate) const GUIO_SHARED_IMAGE_COUNT: usize = 0xae291c;

pub(crate) const DISPLAY_LIVE_REDRAW: usize = 0x12f545;

/// The shared text image (`Guio::draw` loads a picture named by a widget's text once and lends it to the next widget
/// with the same text).
pub(crate) const GUIO_SHARED_IMAGE: usize = 0xae2a24;

/// "     %s" (the five-space indent of an area item's text).
pub(crate) const GUIO_INDENT_FORMAT: usize = 0x9cdc58;

/// The popup arrows and the marks a widget can carry: each glyph starts with its width and height (bytes).
pub(crate) const GLYPH_POPUP_ABOVE: usize = 0x9cd6bc;

pub(crate) const GLYPH_POPUP_BELOW: usize = 0x9cd6dc;

pub(crate) const GLYPH_POPUP_LEFT: usize = 0x9cd67c;

pub(crate) const GLYPH_POPUP_RIGHT: usize = 0x9cd69c;

pub(crate) const GLYPH_FLAG_2000: usize = 0x9cd6fc;

pub(crate) const GLYPH_FLAG_4000: usize = 0x9cd730;

pub(crate) const GLYPH_FLAG_20000: usize = 0x9cd774;

pub(crate) const GLYPH_FLAG_40000: usize = 0x9cd7bc;

/// The "draw nothing" colour for the arrows (`G_GUI_COLOUR_BLACK`) and the colour of the marks drawn as glyphs.
pub(crate) const GUIO_MARK_COLOUR: usize = 0xaebdd4;
