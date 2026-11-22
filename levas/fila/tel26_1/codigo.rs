// ---------- batch %%LEVA%%: the display list base (display.cpp) ----------

/// The Display object (display.cpp, 0x12f550 bytes, one per screen slot): offsets used by the display.cpp routines.
#[allow(dead_code)]
mod display_list {
    /// u32: input mode of `Display::poll_input` (0 hit test, 1 popup hover, 2 scroll thumb drag, 3 widget drag).
    pub(crate) const INPUT_MODE: usize = 0x0;
    /// The Area table (Area.cpp objects, `AREA_SIZE` bytes each).
    pub(crate) const AREAS: usize = 0x4;
    pub(crate) const AREA_SIZE: usize = 0xbf1;
    /// The Guio item table (`ITEM_SIZE` bytes each).
    pub(crate) const ITEMS: usize = 0xba95e;
    pub(crate) const ITEM_SIZE: usize = 0x18c;
    /// i16: items in the table (at most 1199).
    pub(crate) const ITEM_COUNT: usize = 0x12e9a0;
    /// i16[250]: the areas in draw order.
    pub(crate) const AREA_ORDER: usize = 0x12e9a2;
    /// i16[1200]: the items that belong to no area, in draw order.
    pub(crate) const ITEM_ORDER: usize = 0x12eb96;
    pub(crate) const AREA_ORDER_COUNT: usize = 0x12f4f6;
    pub(crate) const ITEM_ORDER_COUNT: usize = 0x12f4f8;
    /// u32: the screen was darkened once (`clear_screen_once`); later draws darken it again.
    pub(crate) const SCREEN_CLEARED: usize = 0x12f4fa;
    /// i16: item queued by a hotkey / by a click (returned by `poll_input` while its flag is up).
    pub(crate) const HOTKEY_ITEM: usize = 0x12f501;
    pub(crate) const CLICK_ITEM: usize = 0x12f503;
    /// u32: the network buffer is to be resent instead of a status packet [inference].
    pub(crate) const RESEND_PENDING: usize = 0x12f505;
    /// i32 x2: the mouse when the drag started or moved last.
    pub(crate) const DRAG_MOUSE_X: usize = 0x12f509;
    pub(crate) const DRAG_MOUSE_Y: usize = 0x12f50d;
    /// i32 x4: the outline of the dragged widget.
    pub(crate) const DRAG_RECT: usize = 0x12f511;
    /// Image saved under the drag outline.
    pub(crate) const DRAG_IMAGE: usize = 0x12f521;
    /// Image of the screen background picture.
    pub(crate) const BACKGROUND_IMAGE: usize = 0x12f525;
    /// i16: the popup area opened by the hit test.
    pub(crate) const OPEN_POPUP_AREA: usize = 0x12f52d;
    /// i16: the area whose scroll thumb is being dragged.
    pub(crate) const SCROLL_AREA: usize = 0x12f52f;
    /// i16: the widget being dragged.
    pub(crate) const DRAG_ITEM: usize = 0x12f531;
    /// i16: the text entry left with Tab; the next text entry takes the focus.
    pub(crate) const NEXT_TEXT_FOCUS: usize = 0x12f533;
    /// u32: fade the next draw in.
    pub(crate) const FADE_IN: usize = 0x12f535;
    /// u32: `draw_screen` does nothing.
    pub(crate) const FROZEN: usize = 0x12f53d;
    /// u32: the areas were laid out (the first draw lays them out).
    pub(crate) const LAID_OUT: usize = 0x12f545;
    /// u32: an item did not fit in the table.
    pub(crate) const OVERFLOW: usize = 0x12f549;
    /// u32: a status packet was sent by `send_keepalive`.
    pub(crate) const KEEPALIVE_SENT: usize = 0x12f555;
    /// char[]: file name of the background picture on screen.
    pub(crate) const BACKGROUND_NAME: usize = 0x12f55d;
}

/// The Guio item (0x18c bytes) fields used by the display.cpp routines.
#[allow(dead_code)]
mod display_item {
    pub(crate) const ID: usize = 0x8;
    /// u32: 2 clickable, 8 text entry, 0x20 Tab does not pass the focus, 0x40 draggable, 0x100 right button,
    /// 0x200 not a drop target, 0x8000 ':' and '\' allowed, 0x10000 '?' allowed.
    pub(crate) const FLAGS: usize = 0xc;
    pub(crate) const X1: usize = 0x10;
    pub(crate) const Y1: usize = 0x14;
    pub(crate) const X2: usize = 0x18;
    pub(crate) const Y2: usize = 0x1c;
    pub(crate) const ROW: usize = 0x24;
    /// u32: bit 0x10000 = round outline when dragged.
    pub(crate) const STYLE: usize = 0x38;
    /// i32: text entry length limit; negative = the first key clears the text.
    pub(crate) const MAX_LENGTH: usize = 0x40;
    /// i32: text entry cursor, -1 when not editing.
    pub(crate) const CURSOR: usize = 0x44;
    pub(crate) const BACK_IMAGE: usize = 0x4c;
    /// u32: tick of the last highlight change.
    pub(crate) const HOVER_TIME: usize = 0x50;
    /// i32 x4: the drop rectangle.
    pub(crate) const DROP_RECT: usize = 0x60;
    pub(crate) const FONT: usize = 0x76;
    /// i16: the area the item is in.
    pub(crate) const AREA: usize = 0x7a;
    /// i16: the item dropped on (written by `drag_widget`).
    pub(crate) const DROP_TARGET: usize = 0x7e;
    pub(crate) const TEXT: usize = 0x80;
    pub(crate) const CHANGED: usize = 0x180;
    pub(crate) const HIGHLIGHTED: usize = 0x188;
}

/// Area fields the display.cpp routines read that have no name in `area_fields`.
#[allow(dead_code)]
mod display_area {
    /// i16: the item that is highlighted in a popup area.
    pub(crate) const HIGHLIGHTED_ITEM: usize = 0x210;
    /// i16: cursor of the visible items walk (starts at `area_fields::FIRST_VISIBLE_ITEM`).
    pub(crate) const WALK: usize = 0xb78;
    /// u32: the area has a scroll bar.
    pub(crate) const HAS_SCROLLBAR: usize = 0xbed;
}

/// Globals of the input and the screen.
const DISPLAY_MOUSE_X: usize = 0xb67a20;
const DISPLAY_MOUSE_Y: usize = 0xb67a14;
const DISPLAY_RIGHT_BUTTON: usize = 0xb67a38;
const DISPLAY_KEY_CODE: usize = 0xb67a04;
const DISPLAY_KEY_CHAR: usize = 0xb67a24;
const DISPLAY_TICKS: usize = 0xb67a08;
const DISPLAY_HOTKEY_PENDING: usize = 0xb67a0c;
const DISPLAY_CLICK_PENDING: usize = 0xb67a1c;
const DISPLAY_CURSOR_MODE: usize = 0x9c36e8;
const DISPLAY_SNAPSHOT_POOL: usize = 0xae2918;
/// The display.cpp and manager_screens.cpp paths for the error box.
const DISPLAY_CPP: usize = 0x9c36ec;
const MANAGER_SCREENS_CPP: usize = 0xa15138;

/// The "Error" box of a source file at a line ("v<version> <file>.<ext> <line>") and the left button let go.
pub(crate) unsafe fn tel26_error_box(file: usize, line: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(file, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    // the original leaves the three arguments of the version call on the stack: they are the rest of the format
    format(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// Item `k` of the Display `d`.
pub(crate) fn display_item_at(d: usize, k: i16) -> usize {
    (d as isize + display_list::ITEMS as isize + k as isize * display_list::ITEM_SIZE as isize) as usize
}

/// Area `a` of the Display `d`.
pub(crate) fn display_area_at(d: usize, a: i16) -> usize {
    (d as isize + display_list::AREAS as isize + a as isize * display_list::AREA_SIZE as isize) as usize
}

/// The next visible item of an area's walk (`display_area::WALK`): none at the end of the list, at a -1 entry or
/// past the visible rows (the row distance to the first visible item above `area_fields::ROW_COUNT` - 1).
pub(crate) unsafe fn display_area_next_visible(d: usize, area: usize) -> Option<i16> {
    let walk = read_i16(area + display_area::WALK);
    if walk >= read_i16(area + area_fields::ITEM_COUNT) {
        return None;
    }
    let k = read_i16((area as isize + area_fields::ITEM_LIST as isize + walk as isize * 2) as usize);
    write_i16(area + display_area::WALK, walk.wrapping_add(1));
    if k == -1 {
        return None;
    }
    let first_at = read_i16(area + area_fields::FIRST_VISIBLE_ITEM) as isize;
    let first = read_i16((area as isize + area_fields::ITEM_LIST as isize + first_at * 2) as usize);
    let distance = read_i32(display_item_at(d, k) + display_item::ROW).wrapping_sub(read_i32(display_item_at(d, first) + display_item::ROW));
    if distance > read_u8(area + area_fields::ROW_COUNT) as i32 - 1 {
        return None;
    }
    Some(k)
}

/// The mouse is inside the item's rectangle.
pub(crate) unsafe fn display_mouse_on(item: usize) -> bool {
    let (x, y) = (read_i32(DISPLAY_MOUSE_X), read_i32(DISPLAY_MOUSE_Y));
    x >= read_i32(item + display_item::X1) && x <= read_i32(item + display_item::X2)
        && y >= read_i32(item + display_item::Y1) && y <= read_i32(item + display_item::Y2)
}

/// A cdecl graphics call with a rectangle (`gui_darken_rect` 0x5e42e0, `graphics_flip_rect` 0x5e2eb0).
pub(crate) unsafe fn display_rect_call(f: usize, x1: i32, y1: i32, x2: i32, y2: i32) {
    let g: unsafe extern "C" fn(i32, i32, i32, i32) = core::mem::transmute(f);
    g(x1, y1, x2, y2)
}

/// A thiscall without arguments (Area and Guio methods).
pub(crate) unsafe fn display_method(f: usize, this: usize) {
    let g: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    g(this)
}

/// A thiscall with one argument.
pub(crate) unsafe fn display_method1(f: usize, this: usize, a: u32) {
    let g: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
    g(this, a)
}

/// `gui_restore_screen_image(x, y, image)` (0x5e3fd0).
pub(crate) unsafe fn display_restore_image(x: i32, y: i32, image: usize) {
    let g: unsafe extern "C" fn(i32, i32, usize) = core::mem::transmute(0x5e3fd0usize);
    g(x, y, image)
}

/// An immediate of an instruction of the original, read from the running exe: the screen width and height - 1
/// (799 and 599; 1919 and 1079 in the 1080 exe, which patches them).
pub(crate) unsafe fn exe_immediate(immediate: *const i32) -> i32 {
    core::ptr::read_unaligned(immediate)
}

/// Allocates the global widget snapshot pool, one 0x74042-byte slot per channel; fails with the display.cpp 195
/// box and the game exits.
///
/// With `G` = [0xae2918] already set nothing happens; otherwise `crt_calloc(0x74042, slots)` (the slot count
/// sign-extended from 16 bits) goes to [0xae2918]. A null allocation shows the "Error" box of display.cpp line 0xc3,
/// clears the left button, runs `game_fatal_cleanup`, the empty 0x5a8f60 and `exit(-1)`.
/// Original: `display_alloc_guio_snapshot_pool` (0x55f1c0, display.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn display_alloc_guio_snapshot_pool(slots: u32) {
    covered!("tel26_1 snapshot pool");
    if read_usize(DISPLAY_SNAPSHOT_POOL) != 0 {
        return;
    }
    let pool = exe::crt_calloc(0x74042, slots as i16 as i32 as u32);
    write_usize(DISPLAY_SNAPSHOT_POOL, pool);
    if pool != 0 {
        return;
    }
    tel26_error_box(DISPLAY_CPP, 0xc3);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
}

/// Frees the widget snapshot pool ([0xae2918]) when there is one and clears the pointer.
/// Original: `display_free_guio_snapshot_pool` (0x55f270, display.cpp, cdecl).
pub(crate) unsafe extern "C" fn display_free_guio_snapshot_pool() {
    let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
    if pool != 0 {
        exe::crt_free(pool);
        write_usize(DISPLAY_SNAPSHOT_POOL, 0);
    }
}

/// Clears the saved widget count (the word at +0x74040) of slot `channel` of the snapshot pool, when there is a pool.
/// The slot address is computed in 32 bits like the original (channel × 0x3a021 × 2).
/// Original: `display_clear_guio_snapshot` (0x55f290, display.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn display_clear_guio_snapshot(channel: u32) {
    let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
    if pool != 0 {
        let offset = channel.wrapping_mul(0x3a021).wrapping_mul(2);
        write_u16(pool.wrapping_add(offset as usize) + 0x74040, 0);
    }
}

/// Darkens and flips the whole screen the first time: with `display_list::SCREEN_CLEARED` still 0 it is set to 1,
/// then `gui_darken_rect` and `graphics_flip_rect` over (0, 0, width - 1, height - 1) (799 × 599, read from the
/// exe's immediates: 1919 × 1079 in the 1080 exe).
/// Original: `Display::clear_screen_once` (0x55f2c0, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_clear_screen_once(d: usize) {
    if read_u32(d + display_list::SCREEN_CLEARED) != 0 {
        return;
    }
    covered!("tel26_1 clear the screen once");
    let (w, h) = (exe_immediate(0x55f2d0 as *const i32), exe_immediate(0x55f2cb as *const i32));
    write_u32(d + display_list::SCREEN_CLEARED, 1);
    display_rect_call(0x5e42e0, 0, 0, w, h);
    display_rect_call(0x5e2eb0, 0, 0, exe_immediate(0x55f2ed as *const i32), exe_immediate(0x55f2e8 as *const i32));
}

/// Draws the screen: the background picture, the areas, the free widgets and the popups, then flips the rectangle.
///
/// Returns 1. With `display_list::FROZEN` nothing else happens. The picture name comes from
/// `ScreenManager::get_background_picture` (0x7fee80) and the key char [0xb67a24] is cleared; with [0xdd6bda] set
/// and a non-empty name, the name buffer is overwritten with the fixed name at 0x9c3730. When the name differs
/// (stricmp) from `display_list::BACKGROUND_NAME`: an empty name saves the whole screen as the background image
/// (`gui_save_screen_image`, reusing the old one) and empties the stored name; otherwise the name is stored and
/// loaded (`gui_load_rgn_picture` 0x5e40d0, reusing the old image), falling back to SI_DATA\DEFAULT_PIC.RGN; a new
/// image replaces (and frees) the old one. Then the background image is put back on the screen (0, 0); the first
/// draw (`display_list::LAID_OUT` 0) lays out every area (`Area::layout`); the open ordinary areas are drawn
/// (`Area::draw`, 1), then the free widgets (`Guio::draw`, 1), then the open popup areas; a cleared screen
/// (`SCREEN_CLEARED`) is darkened again when [0xb77c8c] is set; `FADE_IN` fades the screen in (20 steps) and is
/// cleared; the flip covers (x1, y1, x2, y2) when none is -1, else the whole screen; `LAID_OUT` = 1. The screen
/// edges are read from the exe (the 1080 exe has its own).
/// Original: `Display::draw_screen` (0x55f300, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_draw_screen(d: usize, x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    let get_picture: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x7fee80usize);
    let load_picture: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x5e40d0usize);
    let fade: unsafe extern "C" fn(i32, i32, i32, i32, i32) = core::mem::transmute(0x5e30e0usize);
    let picture = get_picture(G_SCREEN_MANAGER);
    if read_u32(d + display_list::FROZEN) != 0 {
        return 1;
    }
    covered!("tel26_1 draw the screen");
    let fixed = read_u32(0xdd6bda);
    write_u8(DISPLAY_KEY_CHAR, 0);
    if fixed != 0 && read_u8(picture) != 0 {
        tel26_copy_text(picture as *mut u8, 0x9c3730 as *const u8);
    }
    let name = d + display_list::BACKGROUND_NAME;
    if exe::crt_stricmp(name, picture) != 0 {
        if read_u8(picture) == 0 {
            let image = exe::gui_save_screen_image(0, 0, exe_immediate(0x55f393 as *const i32), exe_immediate(0x55f38e as *const i32), read_usize(d + display_list::BACKGROUND_IMAGE));
            write_usize(d + display_list::BACKGROUND_IMAGE, image);
            write_u8(name, 0);
        } else {
            tel26_copy_text(name as *mut u8, picture as *const u8);
            let mut image = load_picture(name, read_usize(d + display_list::BACKGROUND_IMAGE));
            if image == 0 {
                let old = read_usize(d + display_list::BACKGROUND_IMAGE);
                image = load_picture(exe::utils_build_file_path(0x9c3718usize, 0x9c3720usize), old);
            }
            let old = read_usize(d + display_list::BACKGROUND_IMAGE);
            if image != 0 && image != old {
                if old != 0 {
                    exe::gui_free_image(old);
                }
                write_usize(d + display_list::BACKGROUND_IMAGE, image);
            }
        }
    }
    let image = read_usize(d + display_list::BACKGROUND_IMAGE);
    if image != 0 {
        display_restore_image(0, 0, image);
    }
    let areas = |s: i16| display_area_at(d, read_i16(d + display_list::AREA_ORDER + s as usize * 2));
    if read_u32(d + display_list::LAID_OUT) == 0 {
        let mut s = 0i16;
        while s < read_i16(d + display_list::AREA_ORDER_COUNT) {
            display_method(0x403350, areas(s));
            s += 1;
        }
    }
    for popups in [false, true] {
        if popups {
            let mut s = 0i16;
            while s < read_i16(d + display_list::ITEM_ORDER_COUNT) {
                exe::guio_draw(display_item_at(d, read_i16(d + display_list::ITEM_ORDER + s as usize * 2)), 1);
                s += 1;
            }
        }
        let mut s = 0i16;
        while s < read_i16(d + display_list::AREA_ORDER_COUNT) {
            let a = areas(s);
            if read_u32(a + area_fields::IS_OPEN) != 0 && (read_i16(a + area_fields::OWNER_ITEM) != -1) == popups {
                display_method1(0x402e70, a, 1);
            }
            s += 1;
        }
    }
    if read_u32(d + display_list::SCREEN_CLEARED) != 0 && read_u32(0xb77c8c) != 0 {
        display_rect_call(0x5e42e0, 0, 0, exe_immediate(0x55f571 as *const i32), exe_immediate(0x55f56c as *const i32));
    }
    if read_u32(d + display_list::FADE_IN) != 0 {
        write_u32(d + display_list::FADE_IN, 0);
        fade(0, 0, exe_immediate(0x55f593 as *const i32), exe_immediate(0x55f58e as *const i32), 0x14);
    }
    if x1 as u16 != 0xffff && x2 as u16 != 0xffff && y1 as u16 != 0xffff && y2 as u16 != 0xffff {
        display_rect_call(0x5e2eb0, x1 as i16 as i32, y1 as i16 as i32, x2 as i16 as i32, y2 as i16 as i32);
    } else {
        display_rect_call(0x5e2eb0, 0, 0, exe_immediate(0x55f5f1 as *const i32), exe_immediate(0x55f5ec as *const i32));
    }
    write_u32(d + display_list::LAID_OUT, 1);
    1
}

/// Copies a C string with its terminator (the original's inline strcpy).
pub(crate) unsafe fn tel26_copy_text(dst: *mut u8, src: *const u8) {
    let mut i = 0;
    loop {
        let b = *src.add(i);
        *dst.add(i) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Returns the widget chosen by the input: a focus hotkey, a queued hotkey or click, else the handler of the input
/// mode; -1 when nothing.
///
/// The cursor mode [0x9c36e8] starts at 0. A focus item (`display::FOCUS_ITEM` ≠ -1) with a focus key
/// (`display::FOCUS_FLAGS` ≠ 0) equal to the key char [0xb67a24] wins: cursor 0 (`set_cursor_mode`), the key char
/// cleared, the focus item returned. With the hotkey flag [0xb67a0c] up: a queued `HOTKEY_ITEM` is returned (cursor
/// 0, flag down, item back to -1), otherwise the flag goes down; the same for the click flag [0xb67a1c] and
/// `CLICK_ITEM`. Then `Display::animate_widgets` (0x569610) and the handler of `INPUT_MODE` (0 hit_test_widgets,
/// 1 track_popup_hover, 2 drag_scroll_thumb, 3 drag_widget, called by address), the cursor mode set from
/// [0x9c36e8] and the handler's widget returned. Another mode: the display.cpp 513 box and 0.
/// Original: `Display::poll_input` (0x55f620, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_poll_input(d: usize) -> i16 {
    covered!("tel26_1 poll the input");
    write_u32(DISPLAY_CURSOR_MODE, 0);
    let focus = read_i16(d + display::FOCUS_ITEM);
    let key = read_u8(d + display::FOCUS_FLAGS);
    if focus != -1 && key != 0 && read_u8(DISPLAY_KEY_CHAR) == key {
        exe::set_cursor_mode(0);
        write_u8(DISPLAY_KEY_CHAR, 0);
        return read_i16(d + display::FOCUS_ITEM);
    }
    for (flag, field) in [(DISPLAY_HOTKEY_PENDING, display_list::HOTKEY_ITEM), (DISPLAY_CLICK_PENDING, display_list::CLICK_ITEM)] {
        if read_u32(flag) != 0 {
            if read_i16(d + field) != -1 {
                exe::set_cursor_mode(0);
                write_u32(flag, 0);
                let item = read_i16(d + field);
                write_i16(d + field, -1);
                return item;
            }
            write_u32(flag, 0);
        }
    }
    display_method(0x569610, d);
    let handler = match read_u32(d + display_list::INPUT_MODE) {
        0 => 0x560490usize,
        1 => 0x560c30,
        2 => 0x561050,
        3 => 0x561210,
        _ => {
            tel26_error_box(DISPLAY_CPP, 0x201);
            return 0;
        }
    };
    let f: unsafe extern "thiscall" fn(usize) -> i16 = core::mem::transmute(handler);
    let item = f(d);
    exe::set_cursor_mode(read_u32(DISPLAY_CURSOR_MODE));
    item
}

/// The 18 item fields of `Display::add_item`, as the caller pushes them (dwords).
pub(crate) type DisplayItemFields = [u32; 18];

/// Appends a widget to the item table and puts it in its area's list or in the free widget order.
///
/// With `display_list::ITEM_COUNT` at 1199 or more: `OVERFLOW` = 1 and -1. Otherwise the count goes up, the new
/// slot is zeroed (0x18c bytes) and `Guio::init` (0x5ee150, thiscall) fills it with the areas (+4), the item table
/// (+0xba95e), its index and the 18 fields in order. With an area (field 17 as i16 ≠ -1) `Area::add_item` (0x403200)
/// takes the index; otherwise the index goes to `ITEM_ORDER` (up to 1200: past that the display.cpp 746 box).
/// Returns the index.
/// Original: `Display::add_item` (0x55f990, display.cpp, thiscall).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn display_add_item(d: usize, f0: u32, f1: u32, f2: u32, f3: u32, f4: u32, f5: u32, f6: u32, f7: u32, f8: u32,
    f9: u32, f10: u32, f11: u32, f12: u32, f13: u32, f14: u32, f15: u32, f16: u32, f17: u32) -> i16 {
    display_add_item_fields(d, [f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16, f17])
}

/// The body of `display_add_item` with the fields in an array.
pub(crate) unsafe fn display_add_item_fields(d: usize, f: DisplayItemFields) -> i16 {
    type GuioInit = unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let count = read_i16(d + display_list::ITEM_COUNT);
    if count >= 0x4af {
        write_u32(d + display_list::OVERFLOW, 1);
        return -1;
    }
    covered!("tel26_1 add an item");
    let item = display_item_at(d, count);
    write_i16(d + display_list::ITEM_COUNT, count + 1);
    core::ptr::write_bytes(item as *mut u8, 0, 0x63 * 4);
    let init: GuioInit = core::mem::transmute(0x5ee150usize);
    init(item, d + display_list::AREAS, d + display_list::ITEMS, count as u16 as u32,
        f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12], f[13], f[14], f[15], f[16], f[17]);
    let index = read_i16(d + display_list::ITEM_COUNT) - 1;
    let area = f[17] as i16;
    if area != -1 {
        display_method1(0x403200, display_area_at(d, area), index as u16 as u32);
        return read_i16(d + display_list::ITEM_COUNT) - 1;
    }
    if index != -1 {
        let n = read_i16(d + display_list::ITEM_ORDER_COUNT);
        if n >= 0x4b0 {
            tel26_error_box(DISPLAY_CPP, 0x2ea);
            return read_i16(d + display_list::ITEM_COUNT) - 1;
        }
        write_i16(d + display_list::ITEM_ORDER + n as usize * 2, index);
        write_i16(d + display_list::ITEM_ORDER_COUNT, n + 1);
    }
    read_i16(d + display_list::ITEM_COUNT) - 1
}

/// Appends an area index to the area draw order (`display_list::AREA_ORDER`); -1 is ignored and a 251st area shows
/// the display.cpp 770 box instead.
/// Original: `Display::push_area_order` (0x5603e0, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_push_area_order(d: usize, area: u32) {
    let area = area as i16;
    if area == -1 {
        return;
    }
    let n = read_i16(d + display_list::AREA_ORDER_COUNT);
    if n >= 0xfa {
        tel26_error_box(DISPLAY_CPP, 0x302);
        return;
    }
    write_i16(d + display_list::AREA_ORDER + n as usize * 2, area);
    write_i16(d + display_list::AREA_ORDER_COUNT, n + 1);
}

/// `Display::add_item` (0x55f990) by address on the Display `d`: the 18 fields; returns eax.
pub(crate) unsafe fn tel26_add_item(d: usize, f: DisplayItemFields) -> u32 {
    type AddItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddItem = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    g(d, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12], f[13], f[14], f[15], f[16], f[17])
}

/// `Display::add_area` (0x55fba0) by address on the Display `d`: x1, y1, x2, y2, columns, column weights, rows, and
/// four more fields; returns eax (the area index in ax).
pub(crate) unsafe fn tel26_add_area(d: usize, f: [u32; 11]) -> u32 {
    type AddArea = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddArea = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    g(d, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// `ScreenManager::add_button_item` (0x804030) on the screen manager: 12 fields; returns eax (the item).
pub(crate) unsafe fn tel26_add_button(f: [u32; 12]) -> u32 {
    type AddButton = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddButton = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11])
}

/// `ScreenManager::add_area_button` (0x8040a0): area, column, row, five style fields, colour, text, id, value.
pub(crate) unsafe fn tel26_area_button(f: [u32; 11]) -> u32 {
    type AreaButton = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AreaButton = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// `ScreenManager::add_area_text` (0x804100): area, column, row, five style fields, text.
pub(crate) unsafe fn tel26_area_text(f: [u32; 9]) -> u32 {
    type AreaText = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AreaText = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8])
}

/// `gui_add_text_area` (0x5ed070, cdecl): x1, y1, x2, y2, two colours, lines, font, colour, text, 0.
pub(crate) unsafe fn tel26_text_area(f: [u32; 11]) {
    let g: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x5ed070usize);
    g(f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// The Back and Next buttons of a screen: `gui_add_back_next_buttons(back available, next available)`, the next
/// asked first (`is_forward_unavailable`, then `is_back_unavailable(0)`), like the originals.
pub(crate) unsafe fn tel26_back_next() {
    let next = (exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(G_SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, next);
}

/// The id of item `item` of the current human's display (0 and -1 become the default command id [0xdd6bde]).
pub(crate) unsafe fn tel26_item_id(item: i16) -> u16 {
    if item != -1 { display_item_id(item) } else { read_u16(G_CURRENT_COMMAND_ID) }
}

/// The value of item `item` of the current human's display (0 for -1).
pub(crate) unsafe fn tel26_item_value(item: i16) -> u32 {
    if item == -1 {
        return 0;
    }
    read_u32((current_human_display() as isize + item as isize * display_list::ITEM_SIZE as isize + display::ITEM_VALUE as isize) as usize)
}

/// The human manager of the current screen slot.
pub(crate) unsafe fn tel26_slot_manager() -> Staff {
    Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize))
}

/// Frees the list of screen parameter 1 when there is one and clears parameters 1 and 2.
pub(crate) unsafe fn tel26_free_list() {
    let list = get_screen_param(1);
    if list != 0 {
        exe::crt_free(list);
        set_screen_param(1, 0);
        set_screen_param(2, 0);
    }
}

/// The nation id of a nation (+0).
pub(crate) unsafe fn tel26_nation_id(n: Nation) -> usize {
    read_u32(n.addr()) as usize
}

/// Text buffer of the screens (0xde1f64).
pub(crate) const JOB_TEXT: usize = 0xde1f64;
/// Colours of the screen items (words).
pub(crate) const JOB_COLOUR_317E: usize = 0xae317e;
pub(crate) const JOB_COLOUR_3184: usize = 0xae3184;
pub(crate) const JOB_COLOUR_3192: usize = 0xae3192;
pub(crate) const JOB_COLOUR_31AA: usize = 0xae31aa;
pub(crate) const JOB_COLOUR_BDD4: usize = 0xaebdd4;
pub(crate) const JOB_COLOUR_BDE8: usize = 0xaebde8;
pub(crate) const JOB_COLOUR_BDEA: usize = 0xaebdea;
pub(crate) const JOB_COLOUR_BDEC: usize = 0xaebdec;
pub(crate) const JOB_COLOUR_BE04: usize = 0xaebe04;

/// A colour word zero-extended (the original pushes the register with only its low half loaded).
pub(crate) unsafe fn job_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

/// Comparer for `display_alloc_guio_snapshot_pool`.
pub(crate) unsafe extern "stdcall" fn compare_display_alloc_guio_snapshot_pool(slots: u32) {
    // simplification: the allocation (and the exit) do not run twice; proven in Rust mode
    original_fn::<unsafe extern "stdcall" fn(u32)>(%%I0%%)(slots)
}

/// Comparer for `display_free_guio_snapshot_pool`.
pub(crate) unsafe extern "C" fn compare_display_free_guio_snapshot_pool() {
    // simplification: the free does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// Comparer for `display_clear_guio_snapshot`: the slot's count word side by side.
pub(crate) unsafe extern "stdcall" fn compare_display_clear_guio_snapshot(channel: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(u32)>(%%I2%%);
    let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
    let mut reg = Vec::new();
    if pool != 0 {
        reg.push((pool.wrapping_add(channel.wrapping_mul(0x3a021).wrapping_mul(2) as usize) + 0x74040, 2));
    }
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(channel), || display_clear_guio_snapshot(channel));
    SCOREBOARD.record(same, || format!("display_clear_guio_snapshot({}) differs", channel));
}

/// Comparer for `display_clear_screen_once`: side by side once the screen is cleared (nothing happens); the first
/// time only the original (it draws).
pub(crate) unsafe extern "thiscall" fn compare_display_clear_screen_once(d: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%);
    if read_u32(d + display_list::SCREEN_CLEARED) == 0 {
        // simplification: the darkening is drawn once; proven in Rust mode
        return o(d);
    }
    let reg = [(d + display_list::SCREEN_CLEARED, 4)];
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(d), || display_clear_screen_once(d));
    SCOREBOARD.record(same, || "display_clear_screen_once differs".to_string());
}

/// Comparer for `display_draw_screen`.
pub(crate) unsafe extern "thiscall" fn compare_display_draw_screen(d: usize, x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    // simplification: drawing, loading the picture and flipping do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(%%I4%%)(d, x1, y1, x2, y2)
}

/// Comparer for `display_poll_input`.
pub(crate) unsafe extern "thiscall" fn compare_display_poll_input(d: usize) -> i16 {
    // simplification: the input (highlights, popups, cursor) does not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(%%I5%%)(d)
}

/// Comparer for `display_add_item`: the new slot, the counts, the free order entry, the overflow flag and the area
/// side by side (the display list as data); the error box (the free order full) only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_display_add_item(d: usize, f0: u32, f1: u32, f2: u32, f3: u32, f4: u32, f5: u32, f6: u32, f7: u32,
    f8: u32, f9: u32, f10: u32, f11: u32, f12: u32, f13: u32, f14: u32, f15: u32, f16: u32, f17: u32) -> i16 {
    type AddItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> i16;
    let o = original_fn::<AddItem>(%%I6%%);
    let run = || o(d, f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16, f17);
    let count = read_i16(d + display_list::ITEM_COUNT);
    let order = read_i16(d + display_list::ITEM_ORDER_COUNT);
    let area = f17 as i16;
    if count < 0 || (count < 0x4af && area == -1 && order >= 0x4b0) {
        // simplification: the error box does not run twice
        return run();
    }
    let mut reg = vec![(d + display_list::ITEM_COUNT, 2), (d + display_list::ITEM_ORDER_COUNT, 2), (d + display_list::OVERFLOW, 4)];
    if count < 0x4af {
        reg.push((display_item_at(d, count), display_list::ITEM_SIZE));
        if area == -1 && order >= 0 {
            reg.push((d + display_list::ITEM_ORDER + order as usize * 2, 2));
        } else if area != -1 {
            reg.push((display_area_at(d, area), display_list::AREA_SIZE));
        }
    }
    let f = [f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16, f17];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, run, || display_add_item_fields(d, f));
    SCOREBOARD.record(same && r == n, || format!("display list add_item (type {:#x}, area {}): original={} rust={} differences {:?}", f0, area, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `display_push_area_order`.
pub(crate) unsafe extern "thiscall" fn compare_display_push_area_order(d: usize, area: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I7%%);
    let n = read_i16(d + display_list::AREA_ORDER_COUNT);
    if area as i16 != -1 && (n >= 0xfa || n < 0) {
        // simplification: the error box does not run twice
        return o(d, area);
    }
    let mut reg = vec![(d + display_list::AREA_ORDER_COUNT, 2)];
    if area as i16 != -1 {
        reg.push((d + display_list::AREA_ORDER + n as usize * 2, 2));
    }
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(d, area), || display_push_area_order(d, area));
    SCOREBOARD.record(same, || format!("display list push_area_order({}) differs", area as i16));
}
