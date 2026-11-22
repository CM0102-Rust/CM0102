// ---------- batch %%LEVA%%: the history screens (History.cpp) ----------

/// The History.cpp path for the error box.
const HISTORY_CPP: usize = 0x9cec24;
/// The history screens' file name buffer (0xb5dedc, also the "Ok" text).
const HISTORY_FILE_TEXT: usize = 0xb5dedc;
/// The empty text (0xad9c64).
const HISTORY_EMPTY: usize = 0xad9c64;
/// The size of a history record (an entry list: +0 the entries, 8 bytes each (text, picture), +4 their count, +8 the
/// record's name) and of a written history file entry (+0 the file name, 0x104 bytes; +0x104 its title, 0x12c bytes).
const HISTORY_RECORD_SIZE: usize = 0x6c;
const HISTORY_FILE_ENTRY_SIZE: usize = 0x230;
/// The size of a picture name in the history's picture table (screen parameter 4).
const HISTORY_PICTURE_NAME_SIZE: usize = 0x104;

/// `ScreenManager::add_text_item(x1, y1, x2, y2, style, colour, flags, font, text colour, text, 1)` (0x803fd0, thiscall,
/// ret 0x2c).
pub(crate) unsafe fn tel35_text_item(f: [u32; 11]) -> u32 {
    type TextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: TextItem = core::mem::transmute(0x803fd0usize);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// The C runtime's strncpy (0x9466d0, cdecl).
pub(crate) unsafe fn tel35_strncpy(dst: usize, src: usize, n: u32) {
    let f: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x9466d0usize);
    f(dst, src, n);
}

/// A cdecl routine of the exe with one argument, returning eax.
pub(crate) unsafe fn tel35_cdecl1(f: usize, a: usize) -> usize {
    let g: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(f);
    g(a)
}

/// The picture of the history record's selected entry: the picture name (`HISTORY_PICTURE_NAME_SIZE` bytes each from
/// parameter 4) of `index`, or the default picture (parameter 7) for -1.
unsafe fn history_picture_name(index: i32) -> usize {
    if index == -1 {
        get_screen_param(7)
    } else {
        get_screen_param(4).wrapping_add((index as u32).wrapping_mul(HISTORY_PICTURE_NAME_SIZE as u32) as usize)
    }
}

/// Rule: the history screen opens on a .his file with the default frame (0x91, 0x4b)-(0x2e9, 0x20f), the text column
/// 0x115 from the left, the picture 0x101 × 0x14f and the pictures under "cm3_history".
///
/// `history_screen_open(name)`: `open_screen` with the builder 0x5f1360, the handler 0x5f1ba0, single instance, the
/// leave routine 0x5f1d60 and 0; when it opens, the parameters: 0 = -1 (no file list), 1..7 = 0, 8..0xb = -1, 0xc =
/// a copy of the name (`set_screen_param` with copy 1), 0xd = -1, 0xe..0x12 = 0x91, 0x4b, 0x2e9, 0x20f, 0x115, 0x13 =
/// "cm3_history", 0x14 = 0x101 and 0x15 = 0x14f.
/// Original: `history_screen_open` (0x5f11c0, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_screen_open(name: usize) {
    covered!("tel35_2 open history");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5f1360usize, 0x5f1ba0usize, 1, 0x5f1d60usize, 0) == 0 {
        return;
    }
    set_screen_param(0, usize::MAX);
    for k in 1..8 {
        set_screen_param(k, 0);
    }
    for k in 8..0xc {
        set_screen_param(k, usize::MAX);
    }
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xc, name, 1);
    for (k, v) in [(0xd, usize::MAX), (0xe, 0x91), (0xf, 0x4b), (0x10, 0x2e9), (0x11, 0x20f), (0x12, 0x115), (0x13, 0x9cec18),
        (0x14, 0x101), (0x15, 0x14f)] {
        set_screen_param(k, v);
    }
}

/// Rule: the history screen shows a .his file: its title, the record chooser (when the file has more than one), the
/// record's picture and its text word-wrapped to the text column, with an Ok button.
///
/// `history_screen_draw()`: the frame (parameters 0xe..0x11), the text column's left offset (0x12), the file list
/// (0xc) and its count (0xd); the text width = the frame's width − the offset − 0x34. The background picture 0. With a
/// file index (parameter 0) neither in the list nor -1 with no list (-1): the History.cpp 180 box, nothing more. With no
/// file loaded (parameter 1 empty): `history_load_his_file` of "<name>.his" (no list) or of the list's entry, 0; when it
/// fails nothing is built. The full-screen black area (799 × 599; 1919 × 1079 in the 1080 exe, read from it), the frame
/// area in the colour (0, 0x30, 0x70), the title (parameter 1, font 4) from 10 inside the frame, 0x2d high. The record
/// (parameter 6 of the records in parameter 2, `HISTORY_RECORD_SIZE` each): its first entry's picture index written to
/// parameter 8; the picture's frame (yellow) and the picture item 0x400 (the picture of that index under parameter
/// 0x13, or the default picture (parameter 7) when the index is -1 or its file does not exist; an index below -1 is
/// taken without the check); with more than one record (parameter 3), the record's name (font 3) at the text column,
/// the chooser button and its popup area (0x200030, colour (0, 0x80, 0xff)) with one item per record (id 2, data k),
/// and the text 0x2c lower. Two frame areas of the text column (8 and 0x62) and the text area (1, grey) 10 inside,
/// written to parameter 9 (the picture item to 0xa). Each entry's text is cut into lines that fit the width (font 2;
/// a line ends at the last space before the width, the following spaces skipped; the text is cut in place and put
/// back), one item per line with the entry's picture index as data (the first line's item written to parameter 0xb
/// when it is -1), and a blank item after each entry. Last the "Ok" button (id 1).
/// Original: `history_screen_draw` (0x5f1360, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_screen_draw() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let load: unsafe extern "C" fn(usize, u32) -> i32 = core::mem::transmute(0x5f1e50usize);
    let x1 = get_screen_param(0xe) as i32;
    let y1 = get_screen_param(0xf) as i32;
    let x2 = get_screen_param(0x10) as i32;
    let y2 = get_screen_param(0x11) as i32;
    let indent = get_screen_param(0x12) as i32;
    let files = get_screen_param(0xc);
    let count = get_screen_param(0xd) as i32;
    let width = x2 - indent - x1 - 0x34;
    exe::background_set_picture(0usize);
    let p0 = get_screen_param(0) as i32;
    if !((p0 >= 0 && p0 < count) || (p0 == -1 && count == -1)) {
        tel26_error_box(HISTORY_CPP, 0xb4);
        return;
    }
    if get_screen_param(1) == 0 {
        let loaded = if get_screen_param(0) as i32 == -1 {
            format(HISTORY_FILE_TEXT, 0x9cec50, get_screen_param(0xc));
            load(HISTORY_FILE_TEXT, 0)
        } else {
            load(read_usize(files.wrapping_add(get_screen_param(0).wrapping_mul(4))), 0)
        };
        if loaded == 0 {
            return;
        }
    }
    covered!("tel35_2 draw history");
    let black = tel35_colour(G_GUI_COLOUR_BLACK);
    let pale = tel35_colour(G_GUI_COLOUR_PALE_GREY);
    let yellow = tel35_colour(G_GUI_COLOUR_YELLOW);
    tel26_add_area(current_human_display(), [0, 0, exe_immediate(0x5f14d0 as *const i32) as u32, exe_immediate(0x5f14c8 as *const i32) as u32,
        1, 0, 1, 0, 2, black, u32::MAX]);
    let colour = exe::graphics_rgb_to_pixel(0, 0x30, 0x70, 0);
    tel26_add_area(current_human_display(), [x1 as u32, y1 as u32, x2 as u32, y2 as u32, 1, 0, 1, 0, 0x30, colour, u32::MAX]);
    let (left, right, top) = (x1 + 0xa, x2 - 0xa, y1 + 0xa);
    tel35_text_item([left as u32, top as u32, right as u32, (top + 0x2d) as u32, 0x30, black, 0xc, 4, pale, get_screen_param(1) as u32, 1]);
    let mut y = top + 0x37;
    let records = get_screen_param(2);
    let record = records.wrapping_add((get_screen_param(6) as u32).wrapping_mul(HISTORY_RECORD_SIZE as u32) as usize);
    let first = read_i32(read_usize(record) + 4);
    set_screen_param(8, first as usize);
    let pic_x2 = get_screen_param(0x14) as i32 + x1 + 0xc;
    let pic_y2 = get_screen_param(0x15) as i32 + y + 2;
    tel35_text_item([left as u32, y as u32, pic_x2 as u32, pic_y2 as u32, 0x61, colour, 1, 2, yellow, HISTORY_EMPTY as u32, 1]);
    let picture = if first == -1 {
        get_screen_param(7)
    } else if first < -1 {
        history_picture_name(first)
    } else if exe::utils_file_exists(exe::utils_build_file_path(get_screen_param(0x13), history_picture_name(first))) != 0 {
        history_picture_name(first)
    } else {
        get_screen_param(7)
    };
    let path = exe::utils_build_file_path(get_screen_param(0x13), picture);
    let picture_item = tel26_add_item(current_human_display(), [0x400, (x1 + 0xc) as u32, (y + 2) as u32, pic_x2 as u32, pic_y2 as u32, 0, 0, 1,
        black, black, 0xc, 2, pale, path as u32, 1, 0, 0, u32::MAX]);
    if get_screen_param(3) as i32 > 1 {
        covered!("tel35_2 history record chooser");
        let x = indent + x1;
        tel35_text_item([x as u32, y as u32, right as u32, (y + 0x22) as u32, 0x32, colour, 0xc, 3, yellow, (record + 8) as u32, 1]);
        let popup_colour = exe::graphics_rgb_to_pixel(0, 0x80, 0xff, 0);
        let button = tel26_add_button([(x + 5) as u32, (y + 5) as u32, (x + 0x14) as u32, (y + 0x19) as u32, 0x20, black, 0xc, 2, yellow,
            HISTORY_EMPTY as u32, 0, 0]);
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x200030, popup_colour, button]);
        let mut k = 0i32;
        while k < get_screen_param(3) as i32 {
            let name = records.wrapping_add(8).wrapping_add((k as u32).wrapping_mul(HISTORY_RECORD_SIZE as u32) as usize);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x90, popup_colour, yellow, 1, 1, black, name as u32, 0, 2, k as u32, popup]);
            k += 1;
        }
        y += 0x2c;
    }
    let x = x1 + indent;
    let bottom = y2 - 0x32;
    tel26_add_area(current_human_display(), [x as u32, y as u32, right as u32, bottom as u32, 1, 0, 0xf, 0, 8, colour, u32::MAX]);
    tel26_add_area(current_human_display(), [x as u32, y as u32, right as u32, bottom as u32, 1, 0, 0xf, 0, 0x62, colour, u32::MAX]);
    let area = tel26_add_area(current_human_display(), [(x + 0xa) as u32, (y + 0xa) as u32, right as u32, (y2 - 0x3c) as u32, 1, 0, 0xf, 0, 1,
        tel35_colour(G_GUI_COLOUR_GREY), u32::MAX]);
    set_screen_param(9, area as i16 as isize as usize);
    set_screen_param(0xa, picture_item as i16 as isize as usize);
    let mut row = 0u32;
    let mut k = 0usize;
    while (k as i32) < read_i32(record + 4) {
        let mut s = read_usize(read_usize(record) + k * 8) as *mut u8;
        if *s != 0 {
            let mut c = *s;
            loop {
                let mut i = 0isize;
                let mut b = c;
                loop {
                    *s.offset(i) = b;
                    b = *s.offset(i + 1);
                    i += 1;
                    *s.offset(i) = 0;
                    if !(exe::font_text_width(2usize, s) < width && b != 0) {
                        break;
                    }
                }
                *s.offset(i) = b;
                while b != b' ' && b != 0 {
                    i -= 1;
                    b = *s.offset(i);
                }
                b = *s.offset(i);
                *s.offset(i) = 0;
                let value = read_u32(read_usize(record) + k * 8 + 4);
                let line = tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 0, row, 1, black, 0, 1, 2, pale, s as u32, 0, 0, value, area]);
                row += 1;
                if get_screen_param(0xb) as i32 == -1 {
                    set_screen_param(0xb, line as i16 as isize as usize);
                }
                *s.offset(i) = b;
                if b == b' ' {
                    loop {
                        let a = *s.offset(i + 1);
                        i += 1;
                        if a != b' ' {
                            break;
                        }
                    }
                }
                let a = *s.offset(i);
                s = s.offset(i);
                if a == 0 {
                    break;
                }
                c = a;
            }
        }
        let value = read_u32(read_usize(record) + k * 8 + 4);
        tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 0, row, 1, black, 0, 1, 2, pale, HISTORY_EMPTY as u32, 0, 0, value, area]);
        row += 1;
        k += 1;
    }
    text(HISTORY_FILE_TEXT, 0x990f7c);
    tel26_add_button([left as u32, (y2 - 0x28) as u32, right as u32, (y2 - 0xa) as u32, 0x30, tel35_colour(G_GUI_COLOUR_GREY), 0xc, 4, pale,
        HISTORY_FILE_TEXT as u32, 1, 0]);
}

/// Rule: scrolling the history text shows the picture of the first visible line's entry; Ok closes, a record of the
/// chooser shows that record.
///
/// `history_screen_handle_event(item)`: with no file loaded (parameter 1) -9. The first visible widget of the text
/// (the scroll row of the text area, parameter 9, + the first line's widget, parameter 0xb, + 7): its data (0 for
/// -1) differing from parameter 8 becomes parameter 8, and the picture item (parameter 0xa) gets that picture's path
/// (under parameter 0x13; the default picture, parameter 7, for -1); redraw. Then the item: -1 gives 0; id 1 (Ok) -9;
/// id 2 (a record) writes its data to parameter 6 and gives -11; others 0.
/// Original: `history_screen_handle_event` (0x5f1ba0, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_screen_handle_event(raw: u32) -> i32 {
    if get_screen_param(1) == 0 {
        return -9;
    }
    let scroll = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(9)) as u32;
    let shown = scroll.wrapping_add(get_screen_param(0xb) as u32).wrapping_add(7) as i16;
    if tel26_item_value(shown) != get_screen_param(8) as u32 {
        covered!("tel35_2 history picture follows");
        set_screen_param(8, tel26_item_value(shown) as usize);
        let path = exe::utils_build_file_path(get_screen_param(0x13), history_picture_name(get_screen_param(8) as i32));
        exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, get_screen_param(0xa) as u32, path);
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
    }
    let item = raw as i16;
    if item == -1 {
        return 0;
    }
    match tel26_item_id(item) {
        1 => -9,
        2 => {
            set_screen_param(6, tel26_item_value(item) as usize);
            -11
        }
        _ => 0,
    }
}

/// Rule: leaving the history screen frees the loaded file: the picture table, and each record's entry texts, entry
/// list and the record table.
///
/// `history_screen_free()`: parameter 1 = 0; the picture table (parameter 4) freed and cleared when there is one;
/// parameter 7 = 0; with a record table (parameter 2): for each of the parameter 3 records whose entry list is not
/// null, every non-null entry text freed, then the list; then the table, and parameters 2 and 3 = 0.
/// Original: `history_screen_free` (0x5f1d60, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_screen_free() {
    set_screen_param(1, 0);
    if get_screen_param(4) != 0 {
        exe::crt_free(get_screen_param(4));
        set_screen_param(4, 0);
    }
    set_screen_param(7, 0);
    let records = get_screen_param(2);
    let count = get_screen_param(3) as i32;
    if records == 0 {
        return;
    }
    covered!("tel35_2 history free");
    for r in 0..count.max(0) as usize {
        let record = records + r * HISTORY_RECORD_SIZE;
        if read_usize(record) == 0 {
            continue;
        }
        let mut j = 0usize;
        while (j as i32) < read_i32(record + 4) {
            let t = read_usize(read_usize(record) + j * 8);
            if t != 0 {
                exe::crt_free(t);
            }
            j += 1;
        }
        exe::crt_free(read_usize(record));
    }
    exe::crt_free(records);
    set_screen_param(2, 0);
    set_screen_param(3, 0);
}

/// Rule: the written history screen opens on a tab with no file list yet.
///
/// `history_written_screen_open(tab)`: `open_screen` with the builder 0x5f2c00, the handler 0x5f31c0, not single, the
/// leave routine 0x5f33d0 and 0; parameter 0 = the tab, 1..4 = 0.
/// Original: `history_written_screen_open` (0x5f2b80, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_written_screen_open(tab: usize) {
    covered!("tel35_2 open written history");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5f2c00usize, 0x5f31c0usize, 0, 0x5f33d0usize, 0) == 0 {
        return;
    }
    set_screen_param(0, tab);
    for k in 1..5 {
        set_screen_param(k, 0);
    }
}

/// Rule: the written history screen lists, under the tabs Nations, Competitions, Leagues, Clubs and Players, the
/// titles of the .his files of the chosen tab's category.
///
/// `history_written_screen_draw()`: ten tab records (0xd4 bytes, built by the runtime's vector constructor with
/// `MenuItem::MenuItem`); the "CM3_HISTORY" folder; the tab (parameter 0), the list (1) and its count (2). The random
/// background picture, the title "History" (dark blue, pale grey), the menu bar (4, 0) and the tab bar of five tabs
/// (Nations 1, Competitions 2, Leagues 3, Clubs 0, Players 4, all kind 1) from 0x50 to 0x221. With no list: a block of
/// 100 file entries (`HISTORY_FILE_ENTRY_SIZE`, `operator new`; null: the History.cpp 1235 box and nothing more); every
/// "*.his" of the folder (`file_find_first`/`file_find_next`) whose name without its extension has the tab's category
/// (`history_get_his_file_category`) and a title (`history_get_his_file_title`) is copied (name 0x104, title 0x12c,
/// each cut at its last byte); a 101st shows the History.cpp 1270 box and the screen stops there (the search left open,
/// the block not kept); the entries sorted by title (qsort with 0x5f3410); parameter 1 = the block and 2 = the count;
/// the search closed. The list area (0x6e, top)-(0x30c, bottom) with (bottom − top − 16) / the height of font 3 rows,
/// one button per entry (" <title>", id 2, the entry as data), scrolled to parameter 3 and written to parameter 4; the
/// Back and Next buttons.
/// Original: `history_written_screen_draw` (0x5f2c00, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_written_screen_draw() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut tabs = [0u32; 0xd4 * 10 / 4];
    let t = tabs.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(t, 0xd4, 0xa, 0x4568f0usize, 0x5a53c0usize);
    let mut folder = [0u8; 0x104];
    tel26_copy_text(folder.as_mut_ptr(), exe::utils_build_file_path(0x9cedb8usize, HISTORY_EMPTY) as *const u8);
    let tab = get_screen_param(0);
    let mut list = get_screen_param(1);
    let mut count = get_screen_param(2) as i32;
    exe::background_set_random_picture();
    text(JOB_TEXT, 0x9914d4, HISTORY_EMPTY);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, tel35_colour(G_GUI_COLOUR_DARK_BLUE), tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT);
    exe::menubar_build(4, 0);
    for (k, (label, link)) in [(0x9c9f64u32, 1u32), (0x98ed18, 2), (0x9cedcc, 3), (0x9cedc4, 0), (0x990a80, 4)].into_iter().enumerate() {
        let entry = t + k * 0xd4;
        text(entry + 4, label as usize);
        write_u32(entry + 0xcc, 1);
        write_u32(entry + 0xd0, link);
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(t, 5, tab as i32, &mut top, &mut bottom, 1);
    if list == 0 {
        covered!("tel35_2 written history file search");
        let block = exe::crt_operator_new(0xdac0);
        count = 0;
        if block == 0 {
            tel26_error_box(HISTORY_CPP, 0x4d3);
            exe::crt_vector_destructor_iterator(t, 0xd4, 0xa, 0x5a53c0usize);
            return;
        }
        let mut pattern = [0u8; 0x104];
        let mut found = [0u8; 0x114];
        exe::crt_makepath(pattern.as_mut_ptr(), 0usize, folder.as_ptr() as usize, 0x989ad8usize, 0x9cedb0usize);
        if exe::file_find_first(pattern.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 {
            let mut entry = block;
            loop {
                tel26_copy_text(pattern.as_mut_ptr(), found.as_ptr().add(0x10));
                let mut i = tel35_c_string_len(pattern.as_ptr()) as isize - 1;
                while i >= 0 {
                    if pattern[i as usize] == b'.' {
                        pattern[i as usize] = 0;
                        break;
                    }
                    i -= 1;
                }
                if tel35_cdecl1(0x5f2580, pattern.as_ptr() as usize) as u32 == tab as u32 {
                    let title = tel35_cdecl1(0x5f2700, pattern.as_ptr() as usize);
                    if title != 0 {
                        if count >= 0x64 {
                            tel26_error_box(HISTORY_CPP, 0x4f6);
                            exe::crt_vector_destructor_iterator(t, 0xd4, 0xa, 0x5a53c0usize);
                            return;
                        }
                        tel35_strncpy(entry, pattern.as_ptr() as usize, 0x104);
                        write_u8(entry + 0x103, 0);
                        tel35_strncpy(entry + 0x104, title, 0x12c);
                        write_u8(entry + 0x104 + 0x12b, 0);
                        count += 1;
                        entry += HISTORY_FILE_ENTRY_SIZE;
                    }
                }
                if exe::file_find_next(found.as_mut_ptr()) != 0 {
                    break;
                }
            }
            if count != 0 {
                exe::crt_qsort(block, count as u32, HISTORY_FILE_ENTRY_SIZE as u32, 0x5f3410usize);
            }
        }
        set_screen_param(1, block);
        set_screen_param(2, count as usize);
        exe::file_find_close(found.as_ptr() as usize);
        list = block;
    }
    let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as u32;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 1, 0, rows, 0, 2, 0, u32::MAX]);
    for k in 0..count.max(0) as usize {
        let entry = list + k * HISTORY_FILE_ENTRY_SIZE;
        format(JOB_TEXT, 0x987ff0, entry + 0x104);
        tel26_area_button([area, 0, k as u32, 1, 0, 1, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 2, entry as u32]);
    }
    let scroll = get_screen_param(3);
    let d = current_human_display();
    if area as i16 != -1 {
        exe::area_set_scroll_row(d.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(0xbf1)), scroll);
    }
    set_screen_param(4, area as i16 as isize as usize);
    tel26_back_next();
    exe::crt_vector_destructor_iterator(t, 0xd4, 0xa, 0x5a53c0usize);
}

/// The length of a C string (the original's repne scasb).
pub(crate) unsafe fn tel35_c_string_len(s: *const u8) -> usize {
    let mut n = 0;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}

/// Rule: on the written history screen a tab click changes the category and drops the list; an entry opens its
/// history screen.
///
/// `history_written_screen_handle_click(item)`: with an item, the list's scroll row (area of parameter 4) is kept in
/// parameter 3. Id 1 (a tab) with a tab other than parameter 0: parameter 0 = that tab, the list (parameter 1) deleted
/// (`operator delete`) and parameters 1 and 2 = 0, -11. Id 2 (an entry): its file name (0x104 bytes, cut at its last
/// byte) opens the history screen (`history_screen_open`), -11. Otherwise (the same tab included) the menu bar's
/// command, then its object link.
/// Original: `history_written_screen_handle_click` (0x5f31c0, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_written_screen_handle_click(raw: u32) -> i32 {
    let tab = get_screen_param(0) as u32;
    let list = get_screen_param(1);
    let item = raw as i16;
    if item != -1 {
        set_screen_param(3, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(4)));
    }
    let id = tel26_item_id(item);
    if id == 1 && tel26_item_value(item) != tab {
        covered!("tel35_2 written history tab");
        set_screen_param(0, tel26_item_value(item) as usize);
        if list != 0 {
            exe::crt_operator_delete(list);
        }
        set_screen_param(1, 0);
        set_screen_param(2, 0);
        return -11;
    }
    if id == 2 {
        covered!("tel35_2 written history entry");
        let mut name = [0u8; 0x104];
        tel35_strncpy(name.as_mut_ptr() as usize, tel26_item_value(item) as usize, 0x104);
        name[0x103] = 0;
        tel35_cdecl1(0x5f11c0, name.as_ptr() as usize);
        return -11;
    }
    manager_menu_bar(raw)
}

/// Rule: leaving the written history screen deletes the file list.
///
/// `history_written_screen_close()`: the list (parameter 1) deleted (`operator delete`) when there is one; parameters
/// 1 and 2 = 0.
/// Original: `history_written_screen_close` (0x5f33d0, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_written_screen_close() {
    let list = get_screen_param(1);
    if list != 0 {
        exe::crt_operator_delete(list);
    }
    set_screen_param(1, 0);
    set_screen_param(2, 0);
}

/// Comparer for `history_screen_open`.
pub(crate) unsafe extern "C" fn compare_history_screen_open(name: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(name)
}

/// Comparer for `history_screen_draw`: side by side over the display list, the file name buffer and the parameters;
/// loading the file does not run twice (only the original runs then).
pub(crate) unsafe extern "C" fn compare_history_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    if get_screen_param(1) == 0 {
        // simplification: loading the .his file allocates the records; the first build only in the original
        return o();
    }
    compare_by_display_list_with("History screen (history_screen_draw)", &[(HISTORY_FILE_TEXT, 0x104)], || o(), || history_screen_draw());
}

/// Comparer for `history_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_history_screen_handle_event(item: u32) -> i32 {
    // simplification: the redraw does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}

/// Comparer for `history_screen_free`.
pub(crate) unsafe extern "C" fn compare_history_screen_free() {
    // simplification: freeing does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer for `history_written_screen_open`.
pub(crate) unsafe extern "C" fn compare_history_written_screen_open(tab: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I4%%)(tab)
}

/// Comparer for `history_written_screen_draw`: side by side over the display list when the list exists (the file
/// search allocates it: the first build only in the original).
pub(crate) unsafe extern "C" fn compare_history_written_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I5%%);
    if get_screen_param(1) == 0 {
        // simplification: the file search allocates the list; the first build only in the original
        return o();
    }
    compare_by_display_list_with("Written history screen (history_written_screen_draw)", &[], || o(), || history_written_screen_draw());
}

/// Comparer for `history_written_screen_handle_click`.
pub(crate) unsafe extern "C" fn compare_history_written_screen_handle_click(item: u32) -> i32 {
    // simplification: opening a screen and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%)(item)
}

/// Comparer for `history_written_screen_close`.
pub(crate) unsafe extern "C" fn compare_history_written_screen_close() {
    // simplification: deleting does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I7%%)()
}
