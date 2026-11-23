use super::*;

/// Rule: a text entry widget with a caller's buffer gets its text copied back there.
///
/// With flag 8 (`display_item::FLAGS`, a text entry) and a target (`guio_fields::ENTRY_TARGET`), the text
/// (`display_item::TEXT`) is copied there with its terminator.
/// Original: `guio_copy_text_to_caller` (0x567b00, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn guio_copy_text_to_caller(item: usize) {
    let target = read_usize(item + guio_fields::ENTRY_TARGET);
    if read_u8(item + display_item::FLAGS) & 8 != 0 && target != 0 {
        tel26_copy_text(target as *mut u8, (item + display_item::TEXT) as *const u8); // tel40_3: the entry's text back
    }
}

/// Comparer for `guio_copy_text_to_caller`: side by side over the caller's buffer (the text's length with its
/// terminator).
pub(crate) unsafe extern "thiscall" fn compare_guio_copy_text_to_caller(item: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GUIO_COPY_TEXT_TO_CALLER);
    let target = read_usize(item + guio_fields::ENTRY_TARGET);
    if target == 0 || read_u8(item + display_item::FLAGS) & 8 == 0 {
        return o(item);
    }
    let reg = [(target, c_string_len(item + display_item::TEXT) + 1)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(item), || guio_copy_text_to_caller(item));
    SCOREBOARD.record(same, || format!("guio_copy_text_to_caller(item {:#x}): differences {:?}", item, first_differences(&mo, &mr)));
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
    let picture = exe::screenmanager_get_background_picture(G_SCREEN_MANAGER);
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
            let mut image = exe::gui_load_rgn_picture(name, read_usize(d + display_list::BACKGROUND_IMAGE));
            if image == 0 {
                let old = read_usize(d + display_list::BACKGROUND_IMAGE);
                image = exe::gui_load_rgn_picture(exe::utils_build_file_path(0x9c3718usize, 0x9c3720usize), old);
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
        exe::graphics_fade_rect(0, 0, exe_immediate(0x55f593 as *const i32), exe_immediate(0x55f58e as *const i32), 0x14);
    }
    if x1 as u16 != 0xffff && x2 as u16 != 0xffff && y1 as u16 != 0xffff && y2 as u16 != 0xffff {
        display_rect_call(0x5e2eb0, x1 as i16 as i32, y1 as i16 as i32, x2 as i16 as i32, y2 as i16 as i32);
    } else {
        display_rect_call(0x5e2eb0, 0, 0, exe_immediate(0x55f5f1 as *const i32), exe_immediate(0x55f5ec as *const i32));
    }
    write_u32(d + display_list::LAID_OUT, 1);
    1
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

/// Comparer for `display_alloc_guio_snapshot_pool`.
pub(crate) unsafe extern "stdcall" fn compare_display_alloc_guio_snapshot_pool(slots: u32) {
    // simplification: the allocation (and the exit) do not run twice; proven in Rust mode
    original_fn::<unsafe extern "stdcall" fn(u32)>(IDX_DISPLAY_ALLOC_GUIO_SNAPSHOT_POOL)(slots)
}

/// Comparer for `display_free_guio_snapshot_pool`.
pub(crate) unsafe extern "C" fn compare_display_free_guio_snapshot_pool() {
    // simplification: the free does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_DISPLAY_FREE_GUIO_SNAPSHOT_POOL)()
}

/// Comparer for `display_clear_guio_snapshot`: the slot's count word side by side.
pub(crate) unsafe extern "stdcall" fn compare_display_clear_guio_snapshot(channel: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(u32)>(IDX_DISPLAY_CLEAR_GUIO_SNAPSHOT);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DISPLAY_CLEAR_SCREEN_ONCE);
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
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(IDX_DISPLAY_DRAW_SCREEN)(d, x1, y1, x2, y2)
}

/// Comparer for `display_poll_input`.
pub(crate) unsafe extern "thiscall" fn compare_display_poll_input(d: usize) -> i16 {
    // simplification: the input (highlights, popups, cursor) does not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(IDX_DISPLAY_POLL_INPUT)(d)
}

/// Comparer for `display_add_item`: the new slot, the counts, the free order entry, the overflow flag and the area
/// side by side (the display list as data); the error box (the free order full) only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_display_add_item(d: usize, f0: u32, f1: u32, f2: u32, f3: u32, f4: u32, f5: u32, f6: u32, f7: u32,
    f8: u32, f9: u32, f10: u32, f11: u32, f12: u32, f13: u32, f14: u32, f15: u32, f16: u32, f17: u32) -> i16 {
    type AddItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> i16;
    let o = original_fn::<AddItem>(IDX_DISPLAY_ADD_ITEM);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DISPLAY_PUSH_AREA_ORDER);
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

/// Input mode 0: tests the mouse against the widgets, highlighting, opening popups, starting a drag, passing keys
/// to the text entries and clicks to the scroll bars. Returns the clicked widget or -1.
///
/// Two passes; the first also does the clicks. In each pass: every free widget (`display_list::ITEM_ORDER`) —
/// clickable (flag 2) in the first pass: `display_hit_widget` — and, for a text entry (flag 8), the key
/// (`Display::handle_text_entry_key`); then every area: an open ordinary one walks its visible items the same way,
/// and an area with a scroll bar gets the click in the first pass (`Display::hit_test_area_scrollbar`). When a key
/// changes a free text entry, the key also goes to the free entries from that one on (it again, the defect of the
/// original) and, in the first pass, to all of them, and that entry is returned; when it changes an area entry,
/// the key goes to the entries of the areas from that area on, and that entry is returned. At the end, a result with
/// a popup area opens it (`Area::open_popup`; mode 1, `OPEN_POPUP_AREA`, left button let go) and, without an id,
/// gives -1; a draggable result (flag 0x40) starts the drag (mode 3, the mouse, `DRAG_ITEM`, its rectangle as the
/// outline, no image) and gives -1.
/// Original: `Display::hit_test_widgets` (0x560490, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_hit_test_widgets(d: usize) -> i16 {
    covered!("tel26_2 hit test");
    let mut result: i16 = -1;
    let mut pass: i16 = 1;
    while pass <= 2 {
        let mut b = 0i16;
        while b < read_i16(d + display_list::ITEM_ORDER_COUNT) {
            let k = read_i16(d + display_list::ITEM_ORDER + b as usize * 2);
            let item = display_item_at(d, k);
            if read_u32(item + display_item::FLAGS) & 2 != 0 && pass == 1 {
                display_hit_widget(item, k, &mut result);
            }
            let k = read_i16(d + display_list::ITEM_ORDER + b as usize * 2);
            if read_u8(display_item_at(d, k) + display_item::FLAGS) & 8 != 0 && display_text_key(d, k) != 0 {
                covered!("tel26_2 hit test free text entry");
                let changed = read_i16(d + display_list::ITEM_ORDER + b as usize * 2);
                display_text_keys_free(d, b);
                if pass == 1 {
                    display_text_keys_free(d, 0);
                }
                return changed;
            }
            b += 1;
        }
        let mut s = 0i16;
        while s < read_i16(d + display_list::AREA_ORDER_COUNT) {
            let a = display_area_at(d, read_i16(d + display_list::AREA_ORDER + s as usize * 2));
            if read_u32(a + area_fields::IS_OPEN) != 0 && read_i16(a + area_fields::OWNER_ITEM) == -1 {
                write_i16(a + display_area::WALK, read_i16(a + area_fields::FIRST_VISIBLE_ITEM));
                while let Some(k) = display_area_next_visible(d, a) {
                    let item = display_item_at(d, k);
                    if read_u32(item + display_item::FLAGS) & 2 != 0 && pass == 1 {
                        display_hit_widget(item, k, &mut result);
                    }
                    if read_u8(item + display_item::FLAGS) & 8 != 0 && display_text_key(d, k) != 0 {
                        covered!("tel26_2 hit test area text entry");
                        display_text_keys_in_areas(d, &mut s);
                        if pass == 1 {
                            // s is the area count by now: the original's second walk does nothing
                            display_text_keys_in_areas(d, &mut s);
                        }
                        return k;
                    }
                }
            }
            let index = read_i16(d + display_list::AREA_ORDER + s as usize * 2);
            if read_u32(display_area_at(d, index) + display_area::HAS_SCROLLBAR) != 0 && pass == 1 {
                display_method1(0x561f10, d, index as u16 as u32);
            }
            s += 1;
        }
        pass += 1;
    }
    if result == -1 {
        return -1;
    }
    let item = display_item_at(d, result);
    let popup = read_i16(item + guio::POPUP_AREA_INDEX);
    if popup != -1 {
        display_method(DISPLAY_OPEN_POPUP, display_area_at(d, popup));
        write_u32(d + display_list::INPUT_MODE, 1);
        write_i16(d + display_list::OPEN_POPUP_AREA, read_i16(item + guio::POPUP_AREA_INDEX));
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
        if read_u32(item + display_item::ID) == 0 {
            return -1;
        }
    }
    if read_u8(item + display_item::FLAGS) & 0x40 != 0 {
        write_u32(d + display_list::INPUT_MODE, 3);
        write_i32(d + display_list::DRAG_MOUSE_X, read_i32(DISPLAY_MOUSE_X));
        write_i32(d + display_list::DRAG_MOUSE_Y, read_i32(DISPLAY_MOUSE_Y));
        write_i16(d + display_list::DRAG_ITEM, result);
        for k in 0..4 {
            write_i32(d + display_list::DRAG_RECT + 4 * k, read_i32(item + display_item::X1 + 4 * k));
        }
        write_usize(d + display_list::DRAG_IMAGE, 0);
        return -1;
    }
    result
}

/// Input mode 1: an open popup menu follows the mouse. Returns the item clicked in it, or -1.
///
/// Two passes over the visible clickable items of every open popup area: under the mouse, a dark item lights up
/// (the area's highlighted item goes dark first) and so does the item that owns the popup (the highlighted item
/// of the owner's area goes dark first); with the left button down (read at entry) the item becomes the result;
/// an item with an id or a popup sets the hand cursor. Away from the mouse a highlighted item without a popup goes
/// dark. Then, 300 ticks after its last change ([0xb67a08] − +0x50, unsigned), a dark item closes its open popup in
/// the first pass and a highlighted one opens its closed popup in the second. At the end, a result without a popup,
/// or the left button down with no result or a popup result, closes the popup (`OPEN_POPUP_AREA`), goes back to
/// mode 0 and lets the left button go.
/// Original: `Display::track_popup_hover` (0x560c30, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_track_popup_hover(d: usize) -> i16 {
    covered!("tel26_2 popup hover");
    let left = read_u32(G_MOUSE_LEFT_BUTTON_DOWN);
    let mut result: i16 = -1;
    let mut pass: i16 = 1;
    while pass <= 2 {
        let mut s = 0i16;
        while s < read_i16(d + display_list::AREA_ORDER_COUNT) {
            let a = display_area_at(d, read_i16(d + display_list::AREA_ORDER + s as usize * 2));
            if read_u32(a + area_fields::IS_OPEN) != 0 && read_i16(a + area_fields::OWNER_ITEM) != -1 {
                write_i16(a + display_area::WALK, read_i16(a + area_fields::FIRST_VISIBLE_ITEM));
                while let Some(k) = display_area_next_visible(d, a) {
                    let item = display_item_at(d, k);
                    if read_u8(item + display_item::FLAGS) & 2 == 0 {
                        continue;
                    }
                    if display_mouse_on(item) {
                        if read_u32(item + display_item::HIGHLIGHTED) == 0 {
                            let lit = read_i16(a + display_area::HIGHLIGHTED_ITEM);
                            if lit != -1 && read_u32(display_item_at(d, lit) + display_item::HIGHLIGHTED) != 0 {
                                display_method(DISPLAY_HIGHLIGHT_OFF, display_item_at(d, lit));
                            }
                            display_method(DISPLAY_HIGHLIGHT_ON, item);
                            let owner = display_item_at(d, read_i16(a + area_fields::OWNER_ITEM));
                            if read_u32(owner + display_item::HIGHLIGHTED) == 0 {
                                let owner_area = display_area_at(d, read_i16(owner + display_item::AREA));
                                let lit = read_i16(owner_area + display_area::HIGHLIGHTED_ITEM);
                                if lit != -1 && read_u32(display_item_at(d, lit) + display_item::HIGHLIGHTED) != 0 {
                                    display_method(DISPLAY_HIGHLIGHT_OFF, display_item_at(d, lit));
                                }
                                display_method(DISPLAY_HIGHLIGHT_ON, display_item_at(d, read_i16(a + area_fields::OWNER_ITEM)));
                            }
                        }
                        if left != 0 {
                            result = k;
                        }
                        if read_u32(item + display_item::ID) != 0 || read_i16(item + guio::POPUP_AREA_INDEX) != -1 {
                            write_u32(DISPLAY_CURSOR_MODE, 2);
                        }
                    } else if read_u32(item + display_item::HIGHLIGHTED) != 0 && read_i16(item + guio::POPUP_AREA_INDEX) == -1 {
                        display_method(DISPLAY_HIGHLIGHT_OFF, item);
                    }
                    let waited = |item: usize| read_u32(DISPLAY_TICKS).wrapping_sub(read_u32(item + display_item::HOVER_TIME)) >= 300;
                    let popup_open = |item: usize| -> Option<bool> {
                        let p = read_i16(item + guio::POPUP_AREA_INDEX);
                        if p == -1 { None } else { Some(read_u32(display_area_at(d, p) + area_fields::IS_OPEN) != 0) }
                    };
                    if read_u32(item + display_item::HIGHLIGHTED) == 0 && waited(item) && popup_open(item) == Some(true) && pass == 1 {
                        display_method(DISPLAY_CLOSE_POPUP, display_area_at(d, read_i16(item + guio::POPUP_AREA_INDEX)));
                    }
                    if read_u32(item + display_item::HIGHLIGHTED) != 0 && waited(item) && popup_open(item) == Some(false) && pass == 2 {
                        display_method(DISPLAY_OPEN_POPUP, display_area_at(d, read_i16(item + guio::POPUP_AREA_INDEX)));
                    }
                }
            }
            s += 1;
        }
        pass += 1;
    }
    let plain = result != -1 && read_i16(display_item_at(d, result) + guio::POPUP_AREA_INDEX) == -1;
    if plain || left != 0 {
        display_method(DISPLAY_CLOSE_POPUP, display_area_at(d, read_i16(d + display_list::OPEN_POPUP_AREA)));
        write_u32(d + display_list::INPUT_MODE, 0);
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
    }
    result
}

/// Input mode 2: the dragged scroll thumb follows the mouse. Returns -1.
///
/// With the left button up: back to mode 0. When the mouse moved vertically since `DRAG_MOUSE_Y`, the move is
/// clamped to the track (scroll bar +0x10..+0x14) and the thumb (+0x24/+0x28) moves; the image under the scroll bar
/// is put back and freed, the bar redrawn (`Area::draw_scrollbar`) and flipped, the position becomes (thumb top −
/// track top) × rows (+0x18) / (track bottom − thumb size (+0x20) − track top + 1) and the area scrolls by the
/// change (`Area::scroll_rows`). The mouse is stored.
/// Original: `Display::drag_scroll_thumb` (0x561050, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_drag_scroll_thumb(d: usize) -> i16 {
    let y = read_i32(DISPLAY_MOUSE_Y);
    let mut delta = y.wrapping_sub(read_i32(d + display_list::DRAG_MOUSE_Y));
    if read_u32(G_MOUSE_LEFT_BUTTON_DOWN) == 0 {
        write_u32(d + display_list::INPUT_MODE, 0);
        return -1;
    }
    if delta == 0 {
        return -1;
    }
    covered!("tel26_2 drag the scroll thumb");
    // ponytail: the original's null check of the scroll bar (an address inside the Display) is dead and left out
    let area = display_area_at(d, read_i16(d + display_list::SCROLL_AREA));
    let bar = area + area_fields::SCROLLBAR;
    let at = |o: usize| read_i32(bar + o);
    if at(0x24).wrapping_add(delta) < at(0x10) {
        delta = at(0x10).wrapping_sub(at(0x24));
    }
    if at(0x28).wrapping_add(delta) > at(0x14) {
        delta = at(0x14).wrapping_sub(at(0x28));
    }
    if delta != 0 {
        write_i32(bar + 0x24, at(0x24).wrapping_add(delta));
        write_i32(bar + 0x28, at(0x28).wrapping_add(delta));
        if read_usize(bar + 0x2c) != 0 {
            display_restore_image(at(0), at(4), read_usize(bar + 0x2c));
            exe::gui_free_image(read_usize(bar + 0x2c));
            write_usize(bar + 0x2c, 0);
        }
        exe::area_draw_scrollbar(area);
        display_rect_call(0x5e2eb0, at(0), at(4), at(8), at(0xc));
        let span = at(0x14).wrapping_sub(at(0x20)).wrapping_sub(at(0x10)).wrapping_add(1);
        let position = at(0x24).wrapping_sub(at(0x10)).wrapping_mul(at(0x18)) / span;
        let old = at(0x1c);
        write_i32(bar + 0x1c, position);
        if position != old {
            exe::area_scroll_rows(area, position.wrapping_sub(old));
        }
    }
    write_i32(d + display_list::DRAG_MOUSE_Y, y);
    -1
}

/// Input mode 3: a widget's outline follows the mouse; on release the widget under the drop point is recorded.
///
/// With its button down (right for flag 0x100): when the mouse moved (or there is no outline image yet) the move is
/// clamped to the screen (0..width − 1, 0..height − 1, read from the exe), the old outline image is put back,
/// flipped and freed, the outline moves, the screen under it is saved and the outline drawn in colour [0xae31a8]
/// (`gui_draw_circle` for style 0x10000, else `graphics_draw_rect`) and flipped; the mouse is stored. On release:
/// the outline image is put back, flipped and freed, the dragged widget's drop target (+0x7e) is cleared and the
/// first clickable droppable widget (flag 2 without 0x200) under the mouse, free or visible in an open ordinary area,
/// becomes it, with the outline as its drop rectangle (+0x60); back to mode 0 and the dragged widget is returned.
/// Returns -1 while dragging.
/// Original: `Display::drag_widget` (0x561210, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_drag_widget(d: usize) -> i16 {
    let (x, y) = (read_i32(DISPLAY_MOUSE_X), read_i32(DISPLAY_MOUSE_Y));
    let mut dx = x.wrapping_sub(read_i32(d + display_list::DRAG_MOUSE_X));
    let mut dy = y.wrapping_sub(read_i32(d + display_list::DRAG_MOUSE_Y));
    let dragged = display_item_at(d, read_i16(d + display_list::DRAG_ITEM));
    let r = |k: usize| read_i32(d + display_list::DRAG_RECT + 4 * k);
    let image = || read_usize(d + display_list::DRAG_IMAGE);
    let put_back = || {
        display_restore_image(r(0), r(1), image());
        display_rect_call(0x5e2eb0, r(0), r(1), r(2), r(3));
        exe::gui_free_image(image());
        write_usize(d + display_list::DRAG_IMAGE, 0);
    };
    if display_item_button(read_u32(dragged + display_item::FLAGS)) != 0 {
        if dy == 0 && dx == 0 && image() != 0 {
            return -1;
        }
        covered!("tel26_2 drag a widget");
        if r(0).wrapping_add(dx) < 0 {
            dx = r(0).wrapping_neg();
        }
        if r(2).wrapping_add(dx) > exe_immediate(0x5612a2 as *const i32) {
            dx = exe_immediate(0x5612a9 as *const i32).wrapping_sub(r(2));
        }
        if r(1).wrapping_add(dy) < 0 {
            dy = r(1).wrapping_neg();
        }
        if r(3).wrapping_add(dy) > exe_immediate(0x5612cb as *const i32) {
            dy = exe_immediate(0x5612d2 as *const i32).wrapping_sub(r(3));
        }
        if !(dx == 0 && dy == 0 && image() != 0) {
            if image() != 0 {
                put_back();
            }
            for (k, delta) in [(0, dx), (1, dy), (2, dx), (3, dy)] {
                write_i32(d + display_list::DRAG_RECT + 4 * k, r(k).wrapping_add(delta));
            }
            write_usize(d + display_list::DRAG_IMAGE, exe::gui_save_screen_image(r(0), r(1), r(2), r(3), 0usize));
            let colour = colour_word(0xae31a8) as u32;
            if read_u32(display_item_at(d, read_i16(d + display_list::DRAG_ITEM)) + display_item::STYLE) & 0x10000 != 0 {
                exe::gui_draw_circle(r(0), r(1), r(2), r(3), 1, colour);
            } else {
                exe::graphics_draw_rect(r(0), r(1), r(2), r(3), 1, colour);
            }
            display_rect_call(0x5e2eb0, r(0), r(1), r(2), r(3));
        }
        write_i32(d + display_list::DRAG_MOUSE_X, x);
        write_i32(d + display_list::DRAG_MOUSE_Y, y);
        return -1;
    }
    covered!("tel26_2 drop a widget");
    if image() != 0 {
        put_back();
    }
    let target = |d: usize| display_item_at(d, read_i16(d + display_list::DRAG_ITEM)) + display_item::DROP_TARGET;
    write_i16(target(d), -1);
    let offer = |k: i16| {
        let item = display_item_at(d, k);
        let flags = read_u32(item + display_item::FLAGS);
        if flags & 2 != 0 && flags & 0x200 == 0
            && x >= read_i32(item + display_item::X1) && x <= read_i32(item + display_item::X2)
            && y >= read_i32(item + display_item::Y1) && y <= read_i32(item + display_item::Y2)
            && read_i16(target(d)) == -1 {
            write_i16(target(d), k);
            let dragged = display_item_at(d, read_i16(d + display_list::DRAG_ITEM));
            for j in 0..4 {
                write_i32(dragged + display_item::DROP_RECT + 4 * j, r(j));
            }
        }
    };
    let mut j = 0i16;
    while j < read_i16(d + display_list::ITEM_ORDER_COUNT) {
        offer(read_i16(d + display_list::ITEM_ORDER + j as usize * 2));
        j += 1;
    }
    let mut s = 0i16;
    while s < read_i16(d + display_list::AREA_ORDER_COUNT) {
        let a = display_area_at(d, read_i16(d + display_list::AREA_ORDER + s as usize * 2));
        if read_u32(a + area_fields::IS_OPEN) != 0 && read_i16(a + area_fields::OWNER_ITEM) == -1 {
            write_i16(a + display_area::WALK, read_i16(a + area_fields::FIRST_VISIBLE_ITEM));
            while let Some(k) = display_area_next_visible(d, a) {
                offer(k);
            }
        }
        s += 1;
    }
    let dropped = read_i16(d + display_list::DRAG_ITEM);
    write_u32(d + display_list::INPUT_MODE, 0);
    dropped
}

/// Applies the current key to text entry `k`. Returns 1 when leaving the entry with its text changed, else 0.
///
/// A widget that is not a text entry (flag 8) shows the display.cpp 1518 box and gives 0. The text (+0x80) is
/// copied to a local buffer; the mouse over the entry sets the hand cursor. With the left button down: on the entry
/// and not editing (+0x44 = −1) the editing starts (cursor at the end, changed flag cleared, the image behind put
/// back, the text refreshed, key code and char cleared, 0); away from the entry while editing it stops (cursor −1,
/// image back, refresh, keys cleared) and the changed flag (+0x180) is returned. Tab (9) or Enter (13) while editing
/// stops it the same way (the entry redrawn and flipped), and, unless flag 0x20, `NEXT_TEXT_FOCUS` = `k`. A
/// different entry not editing takes the focus left by Tab (editing starts, the focus mark cleared). While
/// editing: the first key clears a text whose length limit (+0x40) is negative (the limit made positive, cursor 0,
/// changed, redraw, 0, the keys kept); Left (0x25) and Right (0x27) move the cursor; Backspace (8) and Delete (0x2e)
/// remove a char; another char (≥ 0x20, not one of " * / < > | { } [ ] ~, ':' and '\' only with flag 0x8000, '?'
/// only with flag 0x10000) under the limit is inserted at the cursor when the text with it still fits the entry's
/// width − 10 (`font_text_width` in the entry's font). Each change refreshes the text (image behind put back) and
/// clears the keys.
/// Original: `Display::handle_text_entry_key` (0x5617c0, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_handle_text_entry_key(d: usize, k: u32) -> u32 {
    let format: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let k = k as i16;
    let item = display_item_at(d, k);
    if read_u8(item + display_item::FLAGS) & 8 == 0 {
        tel26_error_box(DISPLAY_CPP, 0x5ee);
        return 0;
    }
    covered!("tel26_2 text entry key");
    let mut text = [0u8; 0x400];
    tel26_copy_text(text.as_mut_ptr(), (item + display_item::TEXT) as *const u8);
    let len = |t: &[u8; 0x400]| t.iter().position(|&c| c == 0).unwrap_or(0) as i32;
    let cursor = || read_i32(item + display_item::CURSOR);
    let put_back = || {
        let image = read_usize(item + display_item::BACK_IMAGE);
        if image != 0 {
            display_restore_image(read_i32(item + display_item::X1), read_i32(item + display_item::Y1), image);
        }
    };
    let clear_keys = || {
        write_u32(DISPLAY_KEY_CODE, 0);
        write_u8(DISPLAY_KEY_CHAR, 0);
    };
    let changed = || (read_u32(item + display_item::CHANGED) != 0) as u32;
    let inside = display_mouse_on(item);
    if inside {
        write_u32(DISPLAY_CURSOR_MODE, 2);
    }
    if read_u32(G_MOUSE_LEFT_BUTTON_DOWN) != 0 {
        if inside {
            if cursor() == -1 {
                write_i32(item + display_item::CURSOR, len(&text));
                write_u32(item + display_item::CHANGED, 0);
                put_back();
                display_set_text(item, text.as_ptr(), 1);
                clear_keys();
                return 0;
            }
        } else if cursor() > -1 {
            write_i32(item + display_item::CURSOR, -1);
            put_back();
            display_set_text(item, text.as_ptr(), 1);
            clear_keys();
            return changed();
        }
    }
    let key = read_u32(DISPLAY_KEY_CODE);
    if (key == 9 || key == 0xd) && cursor() > -1 {
        write_i32(item + display_item::CURSOR, -1);
        put_back();
        exe::guio_draw(item, 0);
        display_rect_call(0x5e2eb0, read_i32(item + display_item::X1), read_i32(item + display_item::Y1),
            read_i32(item + display_item::X2), read_i32(item + display_item::Y2));
        if read_u8(item + display_item::FLAGS) & 0x20 == 0 {
            write_i16(d + display_list::NEXT_TEXT_FOCUS, k);
        }
        clear_keys();
        display_set_text(item, text.as_ptr(), 1);
        return changed();
    }
    let focus = read_i16(d + display_list::NEXT_TEXT_FOCUS);
    if focus != -1 && k != focus && cursor() == -1 {
        write_i32(item + display_item::CURSOR, len(&text));
        write_u32(item + display_item::CHANGED, 0);
        put_back();
        display_set_text(item, text.as_ptr(), 1);
        write_i16(d + display_list::NEXT_TEXT_FOCUS, -1);
        clear_keys();
        return 0;
    }
    let mut at = cursor();
    if at > -1 && key != 0 && read_i32(item + display_item::MAX_LENGTH) < 0 {
        display_set_text(item, 0xad9c64 as *const u8, 0);
        write_i32(item + display_item::CURSOR, 0);
        write_u32(item + display_item::CHANGED, 1);
        write_i32(item + display_item::MAX_LENGTH, read_i32(item + display_item::MAX_LENGTH).wrapping_abs());
        put_back();
        exe::guio_draw(item, 0);
        display_rect_call(0x5e2eb0, read_i32(item + display_item::X1), read_i32(item + display_item::Y1),
            read_i32(item + display_item::X2), read_i32(item + display_item::Y2));
        return 0;
    }
    let refresh = |text: &[u8; 0x400]| {
        put_back();
        display_set_text(item, text.as_ptr(), 1);
        clear_keys();
        0
    };
    if at > 0 && key == 0x25 {
        write_i32(item + display_item::CURSOR, at - 1);
        return refresh(&text);
    }
    if at > -1 && at < len(&text) && key == 0x27 {
        write_i32(item + display_item::CURSOR, at + 1);
        return refresh(&text);
    }
    if at > 0 && key == 8 {
        let mut i = at;
        while i <= len(&text) {
            text[i as usize - 1] = text[i as usize];
            i += 1;
        }
        write_i32(item + display_item::CURSOR, at - 1);
        write_u32(item + display_item::CHANGED, 1);
        return refresh(&text);
    }
    if at <= -1 {
        return 0;
    }
    if at < len(&text) && key == 0x2e {
        while at <= len(&text) - 1 {
            text[at as usize] = text[at as usize + 1];
            at += 1;
        }
        write_u32(item + display_item::CHANGED, 1);
        return refresh(&text);
    }
    let n = len(&text);
    if n >= read_i32(item + display_item::MAX_LENGTH) {
        return 0;
    }
    let c = read_u8(DISPLAY_KEY_CHAR);
    let flags = read_u32(item + display_item::FLAGS);
    let accepted = match c {
        0..=0x1f | b'"' | b'*' | b'/' | b'<' | b'>' | b'|' | b'{' | b'}' | b'[' | b']' | b'~' => false,
        b':' | b'\\' => flags & 0x8000 != 0,
        b'?' => flags & 0x10000 != 0,
        _ => true,
    };
    if !accepted {
        return 0;
    }
    let mut wider = [0u8; 0x410];
    format(wider.as_mut_ptr(), 0x9c375c, text.as_ptr(), c as i8 as i32);
    let width = exe::font_text_width(read_u16(item + display_item::FONT) as usize, wider.as_ptr());
    if width > read_i32(item + display_item::X2).wrapping_sub(read_i32(item + display_item::X1)).wrapping_sub(10) {
        clear_keys();
        return 0;
    }
    let p = cursor();
    if p < n {
        let mut i = n - 1;
        while i >= p {
            text[i as usize + 1] = text[i as usize];
            i -= 1;
        }
    }
    text[p as usize] = read_u8(DISPLAY_KEY_CHAR);
    write_i32(item + display_item::CURSOR, p + 1);
    write_u32(item + display_item::CHANGED, 1);
    text[n as usize + 1] = 0;
    refresh(&text)
}

/// Tests a click on an area's scroll bar: the arrows scroll a row, the track a page, the thumb starts the drag.
/// Returns 1 when the position changed.
///
/// The scroll bar (`area_fields::SCROLLBAR`) is copied and its pressed part (+0x30) cleared. With the left button
/// down and the mouse between the bar's x1 and x2: above the track (y1..track top − 1) the position (+0x1c) goes
/// back a row (not below 0) and part 1; below the track (track bottom + 1..y2) forward a row (not past the rows,
/// +0x18) and part 2; in the track above the thumb back a page (`area_fields::ROW_COUNT`), below it forward a page;
/// on the thumb mode 2 starts (`SCROLL_AREA` = the area, `DRAG_MOUSE_Y` = the mouse). A new position moves the
/// thumb (top = (track bottom − thumb size − track top + 1) × position / rows + track top), puts back and frees the
/// image under the bar, redraws (`Area::draw_scrollbar`) and flips it and scrolls the area by the change; otherwise a
/// new pressed part only redraws the bar the same way.
/// Original: `Display::hit_test_area_scrollbar` (0x561f10, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_hit_test_area_scrollbar(d: usize, index: u32) -> u32 {
    covered!("tel26_2 scroll bar click");
    let area = display_area_at(d, index as i16);
    let bar = area + area_fields::SCROLLBAR;
    // ponytail: the original's null check of the scroll bar (an address inside the Display) is dead and left out
    let at = |o: usize| read_i32(bar + o);
    let saved_position = at(0x1c);
    let saved_part = read_u8(bar + 0x30);
    write_u8(bar + 0x30, 0);
    if read_u32(G_MOUSE_LEFT_BUTTON_DOWN) != 0 {
        let (x, y) = (read_i32(DISPLAY_MOUSE_X), read_i32(DISPLAY_MOUSE_Y));
        let rows = read_u8(area + area_fields::ROW_COUNT) as i32;
        if x >= at(0) && x <= at(8) {
            if y >= at(4) && y <= at(0x10) - 1 {
                write_i32(bar + 0x1c, (at(0x1c) - 1).max(0));
                write_u8(bar + 0x30, 1);
            } else if y >= at(0x14) + 1 && y <= at(0xc) {
                write_i32(bar + 0x1c, (at(0x1c) + 1).min(at(0x18)));
                write_u8(bar + 0x30, 2);
            } else if y >= at(0x10) && y <= at(0x24) - 1 {
                write_i32(bar + 0x1c, (at(0x1c) - rows).max(0));
            } else if y >= at(0x28) + 1 && y <= at(0x14) {
                write_i32(bar + 0x1c, (at(0x1c) + rows).min(at(0x18)));
            } else if y >= at(0x24) && y <= at(0x28) {
                write_u32(d + display_list::INPUT_MODE, 2);
                write_i16(d + display_list::SCROLL_AREA, index as i16);
                write_i32(d + display_list::DRAG_MOUSE_Y, read_i32(DISPLAY_MOUSE_Y));
            }
        }
    }
    let redraw = || {
        if read_usize(bar + 0x2c) != 0 {
            display_restore_image(at(0), at(4), read_usize(bar + 0x2c));
            exe::gui_free_image(read_usize(bar + 0x2c));
            write_usize(bar + 0x2c, 0);
        }
        exe::area_draw_scrollbar(area);
        display_rect_call(0x5e2eb0, at(0), at(4), at(8), at(0xc));
    };
    let position = at(0x1c);
    if position != saved_position {
        let top = at(0x14).wrapping_sub(at(0x20)).wrapping_sub(at(0x10)).wrapping_add(1).wrapping_mul(position) / at(0x18) + at(0x10);
        write_i32(bar + 0x24, top);
        write_i32(bar + 0x28, top + at(0x20) - 1);
        redraw();
        exe::area_scroll_rows(area, at(0x1c).wrapping_sub(saved_position));
        return 1;
    }
    if read_u8(bar + 0x30) != saved_part {
        redraw();
    }
    0
}

/// Keeps a network session alive: a status 0 packet (`display_send_status_packet` 0x568290, stdcall) with
/// `KEEPALIVE_SENT` = 1, or, with `RESEND_PENDING`, the pending buffer ([0xae28f8], [0xae28fc] bytes) sent again
/// (`Network::send_packet`); a failed send shows the "Network Error" box (display.cpp 3185).
/// Original: `Display::send_keepalive` (0x568470, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_send_keepalive(d: usize, session: usize) {
    covered!("tel26_2 network keepalive");
    let slot = read_u16(0xae2914) as u32;
    if read_u32(d + display_list::RESEND_PENDING) == 0 {
        write_u32(d + display_list::KEEPALIVE_SENT, 1);
        exe::display_send_status_packet(session, slot, 0);
        return;
    }
    if exe::network_send_packet(session, read_usize(0xae28f8), read_u32(0xae28fc), slot) != 0 {
        return;
    }
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(DISPLAY_CPP, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), 0xc71);
    format(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), 0xc71u32);
    text(0xae28bc, 0x9c3814);
    text(0xae26c4, 0x9c37b0, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bcusize, 0xae26c4usize, 0);
}

/// Comparer for `display_hit_test_widgets`.
pub(crate) unsafe extern "thiscall" fn compare_display_hit_test_widgets(d: usize) -> i16 {
    // simplification: highlights, popups, keys and scrolling do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(IDX_DISPLAY_HIT_TEST_WIDGETS)(d)
}

/// Comparer for `display_track_popup_hover`.
pub(crate) unsafe extern "thiscall" fn compare_display_track_popup_hover(d: usize) -> i16 {
    // simplification: highlights and popups do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(IDX_DISPLAY_TRACK_POPUP_HOVER)(d)
}

/// Comparer for `display_drag_scroll_thumb`.
pub(crate) unsafe extern "thiscall" fn compare_display_drag_scroll_thumb(d: usize) -> i16 {
    // simplification: drawing and scrolling do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(IDX_DISPLAY_DRAG_SCROLL_THUMB)(d)
}

/// Comparer for `display_drag_widget`.
pub(crate) unsafe extern "thiscall" fn compare_display_drag_widget(d: usize) -> i16 {
    // simplification: the outline images do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(IDX_DISPLAY_DRAG_WIDGET)(d)
}

/// Comparer for `display_handle_text_entry_key`.
pub(crate) unsafe extern "thiscall" fn compare_display_handle_text_entry_key(d: usize, k: u32) -> u32 {
    // simplification: the key is consumed and the entry redrawn once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_DISPLAY_HANDLE_TEXT_ENTRY_KEY)(d, k)
}

/// Comparer for `display_hit_test_area_scrollbar`.
pub(crate) unsafe extern "thiscall" fn compare_display_hit_test_area_scrollbar(d: usize, index: u32) -> u32 {
    // simplification: drawing and scrolling do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_DISPLAY_HIT_TEST_AREA_SCROLLBAR)(d, index)
}

/// Comparer for `display_send_keepalive`.
pub(crate) unsafe extern "thiscall" fn compare_display_send_keepalive(d: usize, session: usize) {
    // simplification: a network packet is sent once
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_DISPLAY_SEND_KEEPALIVE)(d, session)
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
    let display = exe::display_send_screen_section(d, session, player, 0);
    let (mut areas, mut widgets, mut draw) = (0i32, 0i32, 0i32);
    let total = if read_u32(d + display_list::RESEND_PENDING) == 0 {
        areas = exe::display_send_screen_section(d, session, player, 2);
        widgets = exe::display_send_screen_section(d, session, player, 1);
        draw = exe::display_send_screen_section(d, session, player, 3);
        draw.wrapping_add(display).wrapping_add(widgets).wrapping_add(areas)
    } else {
        display
    };
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(JOB_TEXT, 0x9c3764, total, display, read_i16(d + 0x12e99e) as i32, areas, read_i16(d + display_list::ITEM_COUNT) as i32, widgets, draw);
}

/// Comparer for `guio_set_fixed_rect`.
pub(crate) unsafe extern "thiscall" fn compare_guio_set_fixed_rect(item: usize, x1: i32, y1: i32, x2: i32, y2: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i32)>(IDX_GUIO_SET_FIXED_RECT);
    let reg = [(item + guio_rect::FIXED_X1, 16)];
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(item, x1, y1, x2, y2), || guio_set_fixed_rect(item, x1, y1, x2, y2));
    SCOREBOARD.record(same, || format!("guio_set_fixed_rect({}, {}, {}, {}) differs", x1, y1, x2, y2));
}

/// Comparer for `guio_set_rect`.
pub(crate) unsafe extern "thiscall" fn compare_guio_set_rect(item: usize, x1: i32, y1: i32, x2: i32, y2: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i32)>(IDX_GUIO_SET_RECT);
    let reg = [(item + display_item::X1, 16)];
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(item, x1, y1, x2, y2), || guio_set_rect(item, x1, y1, x2, y2));
    SCOREBOARD.record(same, || format!("guio_set_rect({}, {}, {}, {}) differs", x1, y1, x2, y2));
}

/// Comparer for `display_press_scrollbar_key`: the mouse, the button and the key flag side by side.
pub(crate) unsafe extern "stdcall" fn compare_display_press_scrollbar_key(part: u32, held: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, u32)>(IDX_DISPLAY_PRESS_SCROLLBAR_KEY);
    let reg = [(DISPLAY_MOUSE_X, 4), (DISPLAY_MOUSE_Y, 4), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (DISPLAY_SCROLL_KEY_HELD, 4)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(part, held), || display_press_scrollbar_key(part, held));
    SCOREBOARD.record(same, || format!("display_press_scrollbar_key({}, {}): {:?}", part, held, first_differences(&mo, &mr)));
}

/// Comparer for `display_log_screen_sent`.
pub(crate) unsafe extern "thiscall" fn compare_display_log_screen_sent(d: usize, session: usize, player: u32) {
    // simplification: the screen sections are sent over the network once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_DISPLAY_LOG_SCREEN_SENT)(d, session, player)
}

/// Builds the network send buffer (`NET_SEND_BUFFER`, 50000 bytes) and registers `free_net_send_buffer` with
/// `atexit` (a static initialiser of the exe, before WinMain).
/// Original: `init_net_send_buffer` (0x55eef0, display.cpp, cdecl).
pub(crate) unsafe extern "C" fn init_net_send_buffer() {
    display_build_buffer(NET_SEND_BUFFER, 0xc350, 0x55ef10);
}

/// Frees the network send buffer (at exit).
/// Original: `free_net_send_buffer` (0x55ef10, display.cpp, cdecl).
pub(crate) unsafe extern "C" fn free_net_send_buffer() {
    display_free_buffer(NET_SEND_BUFFER)
}

/// Builds the display packet buffer (`DISPLAY_PACKET_BUFFER`, 3072 bytes) and registers `free_display_packet_buffer`
/// with `atexit` (a static initialiser of the exe, before WinMain).
/// Original: `init_display_packet_buffer` (0x55ef20, display.cpp, cdecl).
pub(crate) unsafe extern "C" fn init_display_packet_buffer() {
    display_build_buffer(DISPLAY_PACKET_BUFFER, 0xc00, 0x55ef40);
}

/// Frees the display packet buffer (at exit).
/// Original: `free_display_packet_buffer` (0x55ef40, display.cpp, cdecl).
pub(crate) unsafe extern "C" fn free_display_packet_buffer() {
    display_free_buffer(DISPLAY_PACKET_BUFFER)
}

/// Builds a Display: the channel, no images, no media list, no areas or items, nothing pending, no key or queued
/// item (-1), and the background name set to "\x01" so that the first draw loads the picture. Returns the Display.
/// Original: `Display::Display` (0x55ef50, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn display_build(d: usize, channel: u32) -> usize {
    write_u32(d + display_life::CHANNEL, channel);
    for f in [display_list::BACKGROUND_IMAGE, display_list::DRAG_IMAGE, display_list::RESEND_PENDING, display_list::KEEPALIVE_SENT,
              display_list::FROZEN, display_list::FADE_IN, display_list::LAID_OUT, display_list::INPUT_MODE] {
        write_u32(d + f, 0);
    }
    write_u8(d + display_list::BACKGROUND_NAME, 1);
    write_u32(d + display_life::MEDIA_LIST, 0);
    for f in [display_life::AREA_COUNT, display_list::ITEM_COUNT, display_life::WORD_539, display_life::WORD_53B] {
        write_i16(d + f, 0);
    }
    write_i16(d + display_life::KEY_ITEM, -1);
    write_u8(d + display_life::KEY_STATE, 0);
    write_i16(d + display_list::HOTKEY_ITEM, -1);
    write_i16(d + display_list::CLICK_ITEM, -1);
    d
}

/// Destroys a Display: frees the media list, the widget snapshot pool, the areas and the widgets' images, the drag
/// and background images and (when no Display holds it any more) the shared image, then runs the destructors of the
/// widget table (1200) and of the area table (250).
///
/// The media list (`display_life::MEDIA_LIST`): each of its entries, then its entry array, is freed (`free`) and the
/// pointer cleared (the 8-byte holder itself is not freed). The snapshot pool ([0xae2918]) is freed and cleared. Every
/// area is released and every widget's images freed; the area and widget counts go to 0. `DRAG_IMAGE` is freed (the
/// original checks it twice) and `BACKGROUND_IMAGE`. The shared image (`DISPLAY_SHARED_IMAGE`) is freed when its user
/// count (`DISPLAY_SHARED_IMAGE_USERS`, signed word) is 0 or less, and both are cleared. Then
/// `vector_destructor_iterator` over the widgets (0x18c bytes, `thunk_guio_free_images` 0x55f1b0) and the areas (0xbf1
/// bytes, `thunk_area_release` 0x55f1a0). The original also opens an exception frame for the two tables; the copy does
/// not (no exception is thrown there).
/// Original: `Display::~Display` (0x55efe0, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_destroy(d: usize) {
    covered!("tel37_3 destroy a Display");
    let list = read_usize(d + display_life::MEDIA_LIST);
    if list != 0 {
        let mut k = 0i16;
        while (k as i32) < read_i32(read_usize(d + display_life::MEDIA_LIST) + 4) {
            exe::crt_free(read_usize(read_usize(read_usize(d + display_life::MEDIA_LIST)) + k as i32 as usize * 4));
            k = k.wrapping_add(1);
        }
        exe::crt_free(read_usize(read_usize(d + display_life::MEDIA_LIST)));
        write_usize(d + display_life::MEDIA_LIST, 0);
    }
    let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
    if pool != 0 {
        exe::crt_free(pool);
        write_usize(DISPLAY_SNAPSHOT_POOL, 0);
    }
    display_release_all(d);
    write_i16(d + display_life::AREA_COUNT, 0);
    write_i16(d + display_list::ITEM_COUNT, 0);
    display_free_image_field(d + display_list::DRAG_IMAGE);
    display_free_image_field(d + display_list::DRAG_IMAGE);
    display_free_image_field(d + display_list::BACKGROUND_IMAGE);
    if read_usize(DISPLAY_SHARED_IMAGE) != 0 && read_i16(DISPLAY_SHARED_IMAGE_USERS) <= 0 {
        exe::gui_free_image(read_usize(DISPLAY_SHARED_IMAGE));
        write_usize(DISPLAY_SHARED_IMAGE, 0);
        write_i16(DISPLAY_SHARED_IMAGE_USERS, 0);
    }
    exe::crt_vector_destructor_iterator(d + display_list::ITEMS, display_list::ITEM_SIZE as u32, 0x4b0, 0x55f1b0);
    exe::crt_vector_destructor_iterator(d + display_list::AREAS, display_list::AREA_SIZE as u32, 0xfa, 0x55f1a0);
}

/// Comparer for `init_net_send_buffer`.
pub(crate) unsafe extern "C" fn compare_init_net_send_buffer() {
    // simplification: the buffer is built once (before WinMain); proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_INIT_NET_SEND_BUFFER)()
}

/// Comparer for `free_net_send_buffer`.
pub(crate) unsafe extern "C" fn compare_free_net_send_buffer() {
    // simplification: the buffer is freed once (at exit)
    original_fn::<unsafe extern "C" fn()>(IDX_FREE_NET_SEND_BUFFER)()
}

/// Comparer for `init_display_packet_buffer`.
pub(crate) unsafe extern "C" fn compare_init_display_packet_buffer() {
    // simplification: the buffer is built once (before WinMain); proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_INIT_DISPLAY_PACKET_BUFFER)()
}

/// Comparer for `free_display_packet_buffer`.
pub(crate) unsafe extern "C" fn compare_free_display_packet_buffer() {
    // simplification: the buffer is freed once (at exit)
    original_fn::<unsafe extern "C" fn()>(IDX_FREE_DISPLAY_PACKET_BUFFER)()
}

/// Comparer for `display_build`: the constructor's fields and the return side by side.
pub(crate) unsafe extern "thiscall" fn compare_display_build(d: usize, channel: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_DISPLAY_BUILD);
    let reg: Vec<(usize, usize)> = DISPLAY_BUILT_FIELDS.iter().map(|&(f, n)| (d + f, n)).collect();
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(d, channel), || display_build(d, channel));
    SCOREBOARD.record(same && r == n, || format!("Display::Display({}): {:?}", channel, first_differences(&mo, &mr)));
    r
}

/// Comparer for `display_destroy`.
pub(crate) unsafe extern "thiscall" fn compare_display_destroy(d: usize) {
    // simplification: the frees and the table destructors do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DISPLAY_DESTROY)(d)
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
    exe::gui_restore_screen_image(x, y, image)
}

/// An immediate of an instruction of the original, read from the running exe: the screen width and height - 1
/// (799 and 599; 1919 and 1079 in the 1080 exe, which patches them).
pub(crate) unsafe fn exe_immediate(immediate: *const i32) -> i32 {
    core::ptr::read_unaligned(immediate)
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
    let init: GuioInit = core::mem::transmute(FN_GUIO_INIT);
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

/// `Display::handle_text_entry_key(d, k)` by address (0x5617c0): 1 when the text changed.
pub(crate) unsafe fn display_text_key(d: usize, k: i16) -> u32 {
    exe::display_handle_text_entry_key(d, k as u16 as u32)
}

/// `Guio::set_text_and_refresh(item, text, refresh)` (0x5eed50).
pub(crate) unsafe fn display_set_text(item: usize, text: *const u8, refresh: u32) {
    exe::guio_set_text_and_refresh(item, text, refresh)
}

/// The left or the right button, by the item's 0x100 flag.
pub(crate) unsafe fn display_item_button(flags: u32) -> u32 {
    read_u32(if flags & 0x100 != 0 { DISPLAY_RIGHT_BUTTON } else { G_MOUSE_LEFT_BUTTON_DOWN })
}

/// One clickable widget in the first pass of the hit test: pressed on it highlights it (if not yet) and makes it
/// the result; otherwise a highlighted one goes dark. Pressed or merely under the mouse, a widget with an id or a
/// popup sets the hand cursor (2).
pub(crate) unsafe fn display_hit_widget(item: usize, k: i16, result: &mut i16) {
    let inside = display_mouse_on(item);
    let button = display_item_button(read_u32(item + display_item::FLAGS));
    if inside && button != 0 {
        if read_u32(item + display_item::HIGHLIGHTED) == 0 {
            display_method(DISPLAY_HIGHLIGHT_ON, item);
        }
        *result = k;
    } else if read_u32(item + display_item::HIGHLIGHTED) != 0 {
        display_method(DISPLAY_HIGHLIGHT_OFF, item);
    }
    if inside && (read_u32(item + display_item::ID) != 0 || read_i16(item + guio::POPUP_AREA_INDEX) != -1) {
        write_u32(DISPLAY_CURSOR_MODE, 2);
    }
}

/// The key goes to every text entry among the visible items of the open ordinary areas, from area position `*s`
/// to the end (`*s` ends at the area count).
pub(crate) unsafe fn display_text_keys_in_areas(d: usize, s: &mut i16) {
    while *s < read_i16(d + display_list::AREA_ORDER_COUNT) {
        let a = display_area_at(d, read_i16(d + display_list::AREA_ORDER + *s as usize * 2));
        if read_u32(a + area_fields::IS_OPEN) != 0 && read_i16(a + area_fields::OWNER_ITEM) == -1 {
            write_i16(a + display_area::WALK, read_i16(a + area_fields::FIRST_VISIBLE_ITEM));
            while let Some(k) = display_area_next_visible(d, a) {
                if read_u8(display_item_at(d, k) + display_item::FLAGS) & 8 != 0 {
                    display_text_key(d, k);
                }
            }
        }
        *s += 1;
    }
}

/// The key goes to every free text entry from order position `from` to the end.
pub(crate) unsafe fn display_text_keys_free(d: usize, from: i16) {
    let mut j = from;
    while j < read_i16(d + display_list::ITEM_ORDER_COUNT) {
        let k = read_i16(d + display_list::ITEM_ORDER + j as usize * 2);
        if read_u8(display_item_at(d, k) + display_item::FLAGS) & 8 != 0 {
            display_text_key(d, k);
        }
        j += 1;
    }
}

/// Builds a NetworkBuffer (`NetworkBuffer::NetworkBuffer` 0x77b630, thiscall, capacity) and registers its freeing
/// function with `atexit`.
pub(crate) unsafe fn display_build_buffer(buf: usize, capacity: u32, free: usize) {
    exe::new_networkbuffer(buf, capacity);
    exe::crt_atexit(free);
}

/// Frees a NetworkBuffer (`NetworkBuffer::~NetworkBuffer` 0x77b6e0, thiscall).
pub(crate) unsafe fn display_free_buffer(buf: usize) {
    exe::drop_networkbuffer(buf)
}

/// Frees an image field of the Display when set (`gui_free_image`) and clears it.
pub(crate) unsafe fn display_free_image_field(field: usize) {
    let image = read_usize(field);
    if image != 0 {
        exe::gui_free_image(image);
        write_usize(field, 0);
    }
}

/// Releases every area (`Area::release` 0x4031a0) and frees every widget's images (`Guio::free_images` 0x5eef00).
pub(crate) unsafe fn display_release_all(d: usize) {
    let mut a = 0i16;
    while a < read_i16(d + display_life::AREA_COUNT) {
        display_method(0x4031a0, display_area_at(d, a));
        a += 1;
    }
    let mut k = 0i16;
    while k < read_i16(d + display_list::ITEM_COUNT) {
        display_method(0x5eef00, display_item_at(d, k));
        k += 1;
    }
}

/// Globals of the input and the screen.
pub(crate) const DISPLAY_MOUSE_X: usize = 0xb67a20;

pub(crate) const DISPLAY_MOUSE_Y: usize = 0xb67a14;

pub(crate) const DISPLAY_RIGHT_BUTTON: usize = 0xb67a38;

pub(crate) const DISPLAY_KEY_CODE: usize = 0xb67a04;

pub(crate) const DISPLAY_CURSOR_MODE: usize = 0x9c36e8;

/// `Guio::highlight_on` (0x5eec20) / `Guio::highlight_off` (0x5eecb0).
pub(crate) const DISPLAY_HIGHLIGHT_ON: usize = 0x5eec20;

pub(crate) const DISPLAY_HIGHLIGHT_OFF: usize = 0x5eecb0;

/// `Area::open_popup` (0x402d10) / `Area::close_popup` (0x402d80).
pub(crate) const DISPLAY_OPEN_POPUP: usize = 0x402d10;

pub(crate) const DISPLAY_CLOSE_POPUP: usize = 0x402d80;

/// u32: a scroll bar key is held (`display_press_scrollbar_key`) [inference].
pub(crate) const DISPLAY_SCROLL_KEY_HELD: usize = 0xb67a3c;

/// The shared image of the Displays and how many Displays hold it [inference]: freed by the destructor of the last.
pub(crate) const DISPLAY_SHARED_IMAGE: usize = 0xae2a24;

pub(crate) const DISPLAY_SHARED_IMAGE_USERS: usize = 0xae291c;

/// The fields the constructor writes, with their size (for the comparer).
pub(crate) const DISPLAY_BUILT_FIELDS: [(usize, usize); 19] = [
    (display_life::CHANNEL, 4), (display_list::BACKGROUND_IMAGE, 4), (display_list::DRAG_IMAGE, 4), (display_list::RESEND_PENDING, 4),
    (display_list::KEEPALIVE_SENT, 4), (display_list::FROZEN, 4), (display_list::FADE_IN, 4), (display_list::LAID_OUT, 4),
    (display_list::INPUT_MODE, 4), (display_list::BACKGROUND_NAME, 1), (display_life::MEDIA_LIST, 4), (display_life::AREA_COUNT, 2),
    (display_list::ITEM_COUNT, 2), (display_life::WORD_539, 2), (display_life::WORD_53B, 2), (display_life::KEY_ITEM, 2),
    (display_life::KEY_STATE, 1), (display_list::HOTKEY_ITEM, 2), (display_list::CLICK_ITEM, 2)];

/// Rule: a remote player's click is sent to the server: command 6 with the widget and all its current fields, and,
/// when the widget was dropped on another widget, the drop rectangle and that widget's fields too. Returns whether it
/// was sent.
///
/// Into `NET_SEND_BUFFER` (position 0), inline writes (box `network.h <line>` without room): command 6 (133), the widget
/// (134); `Guio::get_fields` of the widget (`display_item_at`); the flags (141), x1, y1, x2, y2 (134 each), the column's
/// low byte (133), the row (134), the style (141), the colour and highlight colour (139 each), the third colour (141), the
/// font's low byte (133), the text colour (139), the text (its length with the terminator by `NetworkBuffer::put_short`,
/// then the bytes with no room check), the length limit (134), the id (136), the data (137); then through the
/// NetworkBuffer methods: the area (`put_short`), the widget's `GUIO_CLICK_TAIL` dword (`put_int`) and the drop target
/// (`display_item::DROP_TARGET`, `put_short`). With a drop target: 16 (`put_short`) and the four dwords of
/// `display_item::DROP_RECT` (`put_long` 0x5694a0); a target other than the widget adds its fields: flags (`put_ulong`),
/// x1, y1, x2, y2 (`put_short`), column (`put_char`), row (`put_short`), style (`put_ulong`), colours (`put_ushort`),
/// third colour (`put_ulong`), font (`put_char`), text colour (`put_ushort`), the text block (`put_sized_block` with the
/// length with the terminator), the length limit (`put_short`), the id (`put_long`), the data (`put_int`), the area
/// (`put_short`) and its tail dword (`put_int`). Then `Network::send_packet(session, buffer, position, SCREEN_CHANNEL)`:
/// failing, the "Network Error" box (display.cpp 3322) and 0; otherwise the Display's `display_life::DWORD_551` = 0 and 1.
/// Original: `Display::send_widget_click` (0x568560, display.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn display_send_widget_click(d: usize, session: usize, widget: u32) -> u32 {
    covered!("tel40_3 a remote click sent");
    let b = NET_SEND_BUFFER;
    write_u32(b + network_buffer::POS, 0);
    send_buffer_put(6, 1, 0x85); // tel40_3: the click command
    send_buffer_put(widget, 2, 0x86);
    let item = display_item_at(d, widget as i16);
    let f = click_guio_fields(item);
    let tail = read_u32(item + GUIO_CLICK_TAIL);
    let text = guio_text_copy(f.text);
    send_buffer_put(f.flags, 4, 0x8d);
    for v in [f.x1, f.y1, f.x2, f.y2] {
        send_buffer_put(v, 2, 0x86);
    }
    send_buffer_put(f.column & 0xff, 1, 0x85);
    send_buffer_put(f.row, 2, 0x86);
    send_buffer_put(f.style, 4, 0x8d);
    send_buffer_put(f.colour as u32, 2, 0x8b);
    send_buffer_put(f.highlight as u32, 2, 0x8b);
    send_buffer_put(f.colour_c, 4, 0x8d);
    send_buffer_put(f.font as u32 & 0xff, 1, 0x85);
    send_buffer_put(f.text_colour as u32, 2, 0x8b);
    let len = text.len() as u32;
    network_buffer_put_short(b, len);
    let n = len as i16 as i32 as usize;
    core::ptr::copy(text.as_ptr(), network_buffer_cursor(b) as *mut u8, n);
    network_buffer_advance(b, len as i16 as u32);
    send_buffer_put(f.max_length, 2, 0x86);
    send_buffer_put(f.id, 4, 0x88);
    send_buffer_put(f.data, 4, 0x89);
    network_buffer_put_short(b, f.area as u32);
    network_buffer_put_int(b, tail);
    let target = read_u16(item + display_item::DROP_TARGET);
    network_buffer_put_short(b, target as u32);
    if target != 0xffff {
        covered!("tel40_3 a remote drop sent");
        let put_long: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_NETWORKBUFFER_PUT_LONG);
        let rect = [0, 4, 8, 12].map(|k| read_u32(item + display_item::DROP_RECT + k));
        network_buffer_put_short(b, 0x10);
        for v in rect {
            put_long(b, v);
        }
        if target != widget as u16 {
            let other = display_item_at(d, target as i16);
            let g = click_guio_fields(other);
            let tail = read_u32(other + GUIO_CLICK_TAIL);
            let text = guio_text_copy(g.text);
            network_buffer_put_ulong(b, g.flags);
            for v in [g.x1, g.y1, g.x2, g.y2] {
                network_buffer_put_short(b, v);
            }
            network_buffer_put_char(b, g.column);
            network_buffer_put_short(b, g.row);
            network_buffer_put_ulong(b, g.style);
            network_buffer_put_ushort(b, g.colour as u32);
            network_buffer_put_ushort(b, g.highlight as u32);
            network_buffer_put_ulong(b, g.colour_c);
            network_buffer_put_char(b, g.font as u32);
            network_buffer_put_ushort(b, g.text_colour as u32);
            exe::networkbuffer_put_sized_block(b, text.as_ptr() as usize, text.len() as u32);
            network_buffer_put_short(b, g.max_length);
            put_long(b, g.id);
            network_buffer_put_int(b, g.data);
            network_buffer_put_short(b, g.area as u32);
            network_buffer_put_int(b, tail);
        }
    }
    if exe::network_send_packet(session, read_usize(b), read_u32(b + network_buffer::POS), read_u16(SCREEN_CHANNEL) as u32) == 0 {
        display_network_error(0xcfa);
        return 0;
    }
    write_u32(d + display_life::DWORD_551, 0);
    1
}

/// Comparer for `display_send_widget_click`.
pub(crate) unsafe extern "thiscall" fn compare_display_send_widget_click(d: usize, session: usize, widget: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_DISPLAY_SEND_WIDGET_CLICK)(d, session, widget);
    let data = read_usize(NET_SEND_BUFFER);
    let head_ok = data == 0 || read_u32(NET_SEND_BUFFER + network_buffer::POS) < 3 || (read_u8(data) == 6 && read_u16(data + 1) == widget as u16);
    SCOREBOARD.record(head_ok && (r == 0 || read_u32(d + display_life::DWORD_551) == 0), || format!("send_widget_click(widget {}): returned {}, packet head {:x?}",
        widget as i16, r, if data == 0 { &[][..] } else { core::slice::from_raw_parts(data as *const u8, 3) }));
    r
}

/// Puts one byte: with room for 1 (else the box network.h 133) it is written at the position, which advances by 1.
/// Original: `NetworkBuffer::put_char` (0x560140, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_char(buf: usize, value: u32) {
    covered!("tel37_1 put a char");
    if network_buffer_room(buf, 1, 0x85) {
        write_u8(network_buffer_cursor(buf), value as u8);
        network_buffer_advance(buf, 1);
    }
}

/// Comparer for `network_buffer_put_char`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_char(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NETWORK_BUFFER_PUT_CHAR);
    compare_network_buffer(buf, 1, (network_buffer_cursor(buf), 1), "put_char", || o(buf, value), || network_buffer_put_char(buf, value));
}

/// Puts a signed word: with room for 2 (else the box network.h 134) it is written at the position, which advances by 2.
/// Original: `NetworkBuffer::put_short` (0x5601e0, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_short(buf: usize, value: u32) {
    if network_buffer_room(buf, 2, 0x86) {
        write_u16(network_buffer_cursor(buf), value as u16);
        network_buffer_advance(buf, 2);
    }
}

/// Comparer for `network_buffer_put_short`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_short(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NETWORK_BUFFER_PUT_SHORT);
    compare_network_buffer(buf, 2, (network_buffer_cursor(buf), 2), "put_short", || o(buf, value), || network_buffer_put_short(buf, value));
}

/// Puts an unsigned word: with room for 2 (else the box network.h 139) it is written at the position, which advances
/// by 2.
/// Original: `NetworkBuffer::put_ushort` (0x560290, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_ushort(buf: usize, value: u32) {
    if network_buffer_room(buf, 2, 0x8b) {
        write_u16(network_buffer_cursor(buf), value as u16);
        network_buffer_advance(buf, 2);
    }
}

/// Comparer for `network_buffer_put_ushort`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_ushort(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NETWORK_BUFFER_PUT_USHORT);
    compare_network_buffer(buf, 2, (network_buffer_cursor(buf), 2), "put_ushort", || o(buf, value), || network_buffer_put_ushort(buf, value));
}

/// Puts an unsigned dword: with room for 4 (else the box network.h 141) it is written at the position, which advances
/// by 4.
/// Original: `NetworkBuffer::put_ulong` (0x560340, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_ulong(buf: usize, value: u32) {
    if network_buffer_room(buf, 4, 0x8d) {
        write_u32(network_buffer_cursor(buf), value);
        network_buffer_advance(buf, 4);
    }
}

/// Comparer for `network_buffer_put_ulong`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_ulong(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NETWORK_BUFFER_PUT_ULONG);
    compare_network_buffer(buf, 4, (network_buffer_cursor(buf), 4), "put_ulong", || o(buf, value), || network_buffer_put_ulong(buf, value));
}

/// Puts a signed dword: with room for 4 (else the box network.h 137) it is written at the position, which advances
/// by 4.
/// Original: `NetworkBuffer::put_int` (0x567b40, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_int(buf: usize, value: u32) {
    if network_buffer_room(buf, 4, 0x89) {
        write_u32(network_buffer_cursor(buf), value);
        network_buffer_advance(buf, 4);
    }
}

/// Comparer for `network_buffer_put_int`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_int(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NETWORK_BUFFER_PUT_INT);
    compare_network_buffer(buf, 4, (network_buffer_cursor(buf), 4), "put_int", || o(buf, value), || network_buffer_put_int(buf, value));
}

/// Gets one byte into `out`: with room for 1 (else the box network.h 143, `out` untouched) the byte at the position,
/// which advances by 1.
/// Original: `NetworkBuffer::get_char` (0x567be0, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_char(buf: usize, out: usize) {
    covered!("tel37_1 get a char");
    if network_buffer_room(buf, 1, 0x8f) {
        write_u8(out, read_u8(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 1);
    }
}

/// Comparer for `network_buffer_get_char`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_char(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NETWORK_BUFFER_GET_CHAR);
    compare_network_buffer(buf, 1, (out, 1), "get_char", || o(buf, out), || network_buffer_get_char(buf, out));
}

/// Gets a signed word into `out`: with room for 2 (else the box network.h 144) the word at the position, which
/// advances by 2.
/// Original: `NetworkBuffer::get_short` (0x567c80, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_short(buf: usize, out: usize) {
    if network_buffer_room(buf, 2, 0x90) {
        write_u16(out, read_u16(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 2);
    }
}

/// Comparer for `network_buffer_get_short`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_short(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NETWORK_BUFFER_GET_SHORT);
    compare_network_buffer(buf, 2, (out, 2), "get_short", || o(buf, out), || network_buffer_get_short(buf, out));
}

/// Gets a signed dword into `out`: with room for 4 (else the box network.h 146) the dword at the position, which
/// advances by 4.
/// Original: `NetworkBuffer::get_long` (0x567d30, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_long(buf: usize, out: usize) {
    if network_buffer_room(buf, 4, 0x92) {
        write_u32(out, read_u32(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 4);
    }
}

/// Comparer for `network_buffer_get_long`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_long(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NETWORK_BUFFER_GET_LONG);
    compare_network_buffer(buf, 4, (out, 4), "get_long", || o(buf, out), || network_buffer_get_long(buf, out));
}

/// Gets a signed dword (int) into `out`: with room for 4 (else the box network.h 147) the dword at the position, which
/// advances by 4.
/// Original: `NetworkBuffer::get_int` (0x567de0, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_int(buf: usize, out: usize) {
    if network_buffer_room(buf, 4, 0x93) {
        write_u32(out, read_u32(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 4);
    }
}

/// Comparer for `network_buffer_get_int`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_int(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NETWORK_BUFFER_GET_INT);
    compare_network_buffer(buf, 4, (out, 4), "get_int", || o(buf, out), || network_buffer_get_int(buf, out));
}

/// Gets an unsigned word into `out`: with room for 2 (else the box network.h 149) the word at the position, which
/// advances by 2.
/// Original: `NetworkBuffer::get_ushort` (0x567e90, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_ushort(buf: usize, out: usize) {
    if network_buffer_room(buf, 2, 0x95) {
        write_u16(out, read_u16(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 2);
    }
}

/// Comparer for `network_buffer_get_ushort`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_ushort(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NETWORK_BUFFER_GET_USHORT);
    compare_network_buffer(buf, 2, (out, 2), "get_ushort", || o(buf, out), || network_buffer_get_ushort(buf, out));
}

/// Gets an unsigned dword into `out`: with room for 4 (else the box network.h 151) the dword at the position, which
/// advances by 4.
/// Original: `NetworkBuffer::get_ulong` (0x567f40, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_ulong(buf: usize, out: usize) {
    if network_buffer_room(buf, 4, 0x97) {
        write_u32(out, read_u32(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 4);
    }
}

/// Comparer for `network_buffer_get_ulong`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_ulong(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NETWORK_BUFFER_GET_ULONG);
    compare_network_buffer(buf, 4, (out, 4), "get_ulong", || o(buf, out), || network_buffer_get_ulong(buf, out));
}

/// Gets a sized block into `out`: a signed word length (room for 2, else the box network.h 144), then that many bytes
/// copied from the position to `out`; the position advances by 2 and by the length.
///
/// Defect of the original: after the box (no room for the length) the length is an uninitialised local of the frame
/// and the copy still runs with it; the copy uses 0 there (nothing copied, the position does not move).
/// Original: `NetworkBuffer::get_sized_block` (0x567ff0, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_sized_block(buf: usize, out: usize) {
    covered!("tel37_1 get a sized block");
    let mut length = 0i16;
    if network_buffer_room(buf, 2, 0x90) {
        length = read_i16(network_buffer_cursor(buf));
        network_buffer_advance(buf, 2);
    }
    let n = length as i32;
    core::ptr::copy(network_buffer_cursor(buf) as *const u8, out as *mut u8, n as u32 as usize);
    network_buffer_advance(buf, n as u32);
}

/// Comparer for `network_buffer_get_sized_block`: the header and the block written to `out` (the length read from
/// the buffer) side by side; no room for the length only the original.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_sized_block(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NETWORK_BUFFER_GET_SIZED_BLOCK);
    if read_u32(buf + network_buffer::POS).wrapping_add(2) > read_u32(buf + network_buffer::CAPACITY) {
        // simplification: the error box (and the copy of an uninitialised length) does not run twice
        return o(buf, out);
    }
    let n = read_i16(network_buffer_cursor(buf)).max(0) as usize;
    compare_network_buffer(buf, 2, (out, n), "get_sized_block", || o(buf, out), || network_buffer_get_sized_block(buf, out));
}

/// Sets the current channel's screen key and its data (`screen_history::SCREEN_KEY`, `SCREEN_KEY_DATA`; the channel
/// of `screen_manager::CURRENT_SLOT`).
/// Original: `ScreenManager::set_area_selection` (0x5680b0, display.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_area_selection(mgr: usize, key: u32, data: u32) {
    write_u16(screen_record(mgr) + screen_history::SCREEN_KEY, key as u16);
    write_u32(screen_record(mgr) + screen_history::SCREEN_KEY_DATA, data);
}

/// Comparer for `screen_manager_set_area_selection`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_area_selection(mgr: usize, key: u32, data: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_AREA_SELECTION);
    let reg = [(screen_record(mgr) + screen_history::SCREEN_KEY, 6)];
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&reg, || o(mgr, key, data), || screen_manager_set_area_selection(mgr, key, data));
    SCOREBOARD.record(same, || format!("ScreenManager::set_area_selection({}, {:#x}) differs", key as i16, data));
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
    exe::at_9532a6(path);
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

/// Comparer for `display_localise_media_path`: the path buffer side by side.
pub(crate) unsafe extern "C" fn compare_display_localise_media_path(path: usize, is_picture: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_DISPLAY_LOCALISE_MEDIA_PATH);
    let reg = [(path, 0x104)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(path, is_picture), || display_localise_media_path(path, is_picture));
    SCOREBOARD.record(same, || format!("display_localise_media_path (picture {}): {:?}", is_picture, first_differences(&mo, &mr)));
}

/// Resets a Display for a new screen: (with `release`, unless a keepalive was sent) the areas released and the
/// widgets' images freed, both command buffers emptied, command 2 (new screen) put in the display packet buffer and
/// the display list emptied.
///
/// `display_life::KEY_ITEM` = -1 and `KEY_STATE` = 0 first. With `display_list::KEEPALIVE_SENT` clear and `release`,
/// every area is released and every widget's images freed. The send buffer and the display packet buffer positions go
/// to 0 and the byte 2 is put in the packet buffer (no room: the box network.h 133). Then the input mode, the area and
/// widget order counts, the widget and area counts, the drag mouse and outline, both overflow flags, the keepalive and
/// the darkened flag go to 0, the open popup, the scrolled area, the dragged widget and the next text focus to -1;
/// the drag image is freed and `LAID_OUT` cleared.
/// Original: `Display::reset` (0x55f7c0, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn display_reset(d: usize, release: u32) {
    covered!("tel37_3 reset the Display");
    write_i16(d + display_life::KEY_ITEM, -1);
    write_u8(d + display_life::KEY_STATE, 0);
    if read_u32(d + display_list::KEEPALIVE_SENT) == 0 && release != 0 {
        display_release_all(d);
    }
    write_u32(NET_SEND_BUFFER + network_buffer::POS, 0);
    write_u32(DISPLAY_PACKET_BUFFER + network_buffer::POS, 0);
    if network_buffer_room(DISPLAY_PACKET_BUFFER, 1, 0x85) {
        write_u8(network_buffer_cursor(DISPLAY_PACKET_BUFFER), 2);
        network_buffer_advance(DISPLAY_PACKET_BUFFER, 1);
    }
    write_u32(d + display_list::INPUT_MODE, 0);
    for f in [display_list::AREA_ORDER_COUNT, display_list::ITEM_ORDER_COUNT, display_list::ITEM_COUNT, display_life::AREA_COUNT] {
        write_i16(d + f, 0);
    }
    for f in [display_list::OPEN_POPUP_AREA, display_list::SCROLL_AREA, display_list::DRAG_ITEM] {
        write_i16(d + f, -1);
    }
    write_u32(d + display_list::LAID_OUT, 0);
    for f in [display_list::DRAG_MOUSE_X, display_list::DRAG_MOUSE_Y, display_list::DRAG_RECT, display_list::DRAG_RECT + 4,
              display_list::DRAG_RECT + 8, display_life::DRAG_RECT_END, display_list::OVERFLOW, display_life::AREA_OVERFLOW] {
        write_u32(d + f, 0);
    }
    write_i16(d + display_list::NEXT_TEXT_FOCUS, -1);
    write_u32(d + display_life::DWORD_551, 0);
    write_u32(d + display_list::KEEPALIVE_SENT, 0);
    write_u32(d + display_list::SCREEN_CLEARED, 0);
    display_free_image_field(d + display_list::DRAG_IMAGE);
    write_u32(d + display_list::LAID_OUT, 0);
}

/// Comparer for `display_reset`: side by side (the fields and both buffers' headers and the first packet byte) when
/// nothing is released or freed and the packet buffer has room; otherwise only the original and the same fields are
/// checked afterwards against the plan.
pub(crate) unsafe extern "thiscall" fn compare_display_reset(d: usize, release: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DISPLAY_RESET);
    let releases = read_u32(d + display_list::KEEPALIVE_SENT) == 0 && release != 0
        && (read_i16(d + display_life::AREA_COUNT) > 0 || read_i16(d + display_list::ITEM_COUNT) > 0);
    let room = read_u32(DISPLAY_PACKET_BUFFER + network_buffer::CAPACITY) >= 1;
    let mut reg = display_reset_fields(d);
    reg.extend([(NET_SEND_BUFFER + network_buffer::POS, 4), (DISPLAY_PACKET_BUFFER, 12)]);
    if room {
        reg.push((read_usize(DISPLAY_PACKET_BUFFER + network_buffer::DATA), 1));
    }
    if !releases && room && read_usize(d + display_list::DRAG_IMAGE) == 0 {
        let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(d, release), || display_reset(d, release));
        SCOREBOARD.record(same, || format!("Display::reset({}): {:?}", release, first_differences(&mo, &mr)));
        return;
    }
    // simplification: the releases and frees do not run twice; the plan is checked afterwards
    o(d, release);
    let minus = [display_list::OPEN_POPUP_AREA, display_list::SCROLL_AREA, display_list::DRAG_ITEM, display_list::NEXT_TEXT_FOCUS, display_life::KEY_ITEM];
    let wrong: Vec<usize> = display_reset_fields(d).iter().filter(|&&(a, n)| {
        let f = a.wrapping_sub(d);
        let want: u32 = if minus.contains(&f) { 0xffff } else { 0 };
        let got = match n { 1 => read_u8(a) as u32, 2 => read_u16(a) as u32, 4 => read_u32(a), _ => (0..n).map(|i| read_u8(a + i) as u32).sum() };
        got != want
    }).map(|&(a, _)| a.wrapping_sub(d)).collect();
    let pos = read_u32(DISPLAY_PACKET_BUFFER + network_buffer::POS);
    let first = if room { read_u8(read_usize(DISPLAY_PACKET_BUFFER + network_buffer::DATA)) } else { 2 };
    let ok = wrong.is_empty() && read_u32(NET_SEND_BUFFER + network_buffer::POS) == 0 && pos == room as u32 && first == 2;
    SCOREBOARD.record(ok, || format!("Display::reset({}) after the original: fields off the plan {:x?}, packet position {} first byte {}", release, wrong, pos, first));
}

/// Adds an area to the Display: its command into the display packet buffer, then (up to 249 areas) a cleared area
/// initialised by `Area::init` and put in the draw order. Returns the area index, or -1 when the table is full.
///
/// The command: x1, y1, x2, y2 (words), the column count (byte), whether there are column weights (byte) and then
/// their count (word, `put_short`) and bytes (copied without a room check), the row count (byte), whether there are
/// row weights (`put_char`) and then the row weights block (0x569540: count word and bytes), the marks (`put_ulong`),
/// the colour (`put_ushort`) and the owner widget (`put_short`); each inline put without room shows the network.h
/// box (134 for the words, 138 for the counts, 133 for the flag) and is skipped. With 249 areas or more
/// `display_life::AREA_OVERFLOW` = 1 and -1 is returned. Otherwise the area at the count is zeroed (0xbf1 bytes), the
/// count goes up, `Area::init` (0x402ac0) gets the area table, the widget table, the index and the eleven arguments, and
/// `Display::push_area_order` (0x5603e0) the index.
/// Original: `Display::add_area` (0x55fba0, display.cpp, thiscall, ret 0x2c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn display_add_area(d: usize, x1: u32, y1: u32, x2: u32, y2: u32, columns: u32, column_weights: usize,
    rows: u32, row_weights: usize, marks: u32, colour: u32, owner: u32) -> i16 {
    covered!("tel37_3 add an area");
    for v in [x1, y1, x2, y2] {
        display_packet_put(v, 2, 0x86);
    }
    display_packet_put(columns, 1, 0x8a);
    display_packet_put((column_weights != 0) as u32, 1, 0x85);
    if column_weights != 0 {
        let n = columns & 0xff;
        network_buffer_put_short(DISPLAY_PACKET_BUFFER, n);
        core::ptr::copy(column_weights as *const u8, network_buffer_cursor(DISPLAY_PACKET_BUFFER) as *mut u8, n as usize);
        network_buffer_advance(DISPLAY_PACKET_BUFFER, n);
    }
    display_packet_put(rows, 1, 0x8a);
    network_buffer_put_char(DISPLAY_PACKET_BUFFER, (row_weights != 0) as u32);
    if row_weights != 0 {
        let block: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_NETWORKBUFFER_PUT_SIZED_BLOCK);
        block(DISPLAY_PACKET_BUFFER, row_weights, rows & 0xff);
    }
    network_buffer_put_ulong(DISPLAY_PACKET_BUFFER, marks);
    network_buffer_put_ushort(DISPLAY_PACKET_BUFFER, colour);
    network_buffer_put_short(DISPLAY_PACKET_BUFFER, owner);
    let count = read_i16(d + display_life::AREA_COUNT);
    if count >= 0xf9 {
        write_u32(d + display_life::AREA_OVERFLOW, 1);
        return -1;
    }
    write_i16(d + display_life::AREA_COUNT, count.wrapping_add(1));
    let area = display_area_at(d, count);
    core::ptr::write_bytes(area as *mut u8, 0, display_list::AREA_SIZE);
    type Init = unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32, u32, usize, u32, usize, u32, u32, u32);
    let init: Init = core::mem::transmute(FN_AREA_INIT);
    let index = (read_i16(d + display_life::AREA_COUNT) as u16 as u32).wrapping_sub(1);
    init(area, d + display_list::AREAS, d + display_list::ITEMS, index, x1, y1, x2, y2, columns, column_weights, rows, row_weights, marks, colour, owner);
    display_method1(0x5603e0, d, read_i16(d + display_life::AREA_COUNT).wrapping_sub(1) as u16 as u32);
    read_i16(d + display_life::AREA_COUNT).wrapping_sub(1)
}

/// Comparer for `display_add_area`: the packet bytes, the counts, the overflow flag, the new area (and the 0xd9 bytes
/// after it that Area::init may write) and the area order side by side; without room for the whole packet or with
/// the area order full only the original (the error boxes).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_display_add_area(d: usize, x1: u32, y1: u32, x2: u32, y2: u32, columns: u32, column_weights: usize,
    rows: u32, row_weights: usize, marks: u32, colour: u32, owner: u32) -> i16 {
    type AddArea = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, usize, u32, u32, u32) -> i16;
    let o = original_fn::<AddArea>(IDX_DISPLAY_ADD_AREA);
    let run = || o(d, x1, y1, x2, y2, columns, column_weights, rows, row_weights, marks, colour, owner);
    let size = display_add_area_packet_size(columns, column_weights, rows, row_weights);
    let pos = read_u32(DISPLAY_PACKET_BUFFER + network_buffer::POS);
    let count = read_i16(d + display_life::AREA_COUNT);
    let order = read_i16(d + display_list::AREA_ORDER_COUNT);
    if pos.wrapping_add(size) > read_u32(DISPLAY_PACKET_BUFFER + network_buffer::CAPACITY) || count < 0 || (count < 0xf9 && !(0..0xfa).contains(&order)) {
        // simplification: the error boxes do not run twice
        return run();
    }
    let mut reg = vec![(DISPLAY_PACKET_BUFFER, 12), (network_buffer_cursor(DISPLAY_PACKET_BUFFER), size as usize),
                       (d + display_life::AREA_COUNT, 2), (d + display_life::AREA_OVERFLOW, 4), (d + display_list::AREA_ORDER_COUNT, 2)];
    if count < 0xf9 {
        reg.push((display_area_at(d, count), display_list::AREA_SIZE + 0xd9));
        reg.push((d + display_list::AREA_ORDER + order as usize * 2, 2));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, run, || display_add_area(d, x1, y1, x2, y2, columns, column_weights, rows, row_weights, marks, colour, owner));
    SCOREBOARD.record(same && r == n, || format!("display list add_area ({}, {})-({}, {}), {} columns, {} rows: original={} rust={} differences {:?}",
        x1 as i16, y1 as i16, x2 as i16, y2 as i16, columns as u8, rows as u8, r, n, first_differences(&mo, &mr)));
    r
}

/// Sends a status packet to a network player: command 5 and the status byte in the network send buffer (emptied
/// first; no room: the box network.h 133), sent by the session (0x77ac30: bytes, length, player); a failed send shows
/// the "Network Error" box (display.cpp 3154).
/// Original: `display_send_status_packet` (0x568290, display.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn display_send_status_packet(session: usize, player: u32, status: u32) {
    covered!("tel37_3 status packet");
    write_u32(NET_SEND_BUFFER + network_buffer::POS, 0);
    for v in [5, status] {
        if network_buffer_room(NET_SEND_BUFFER, 1, 0x85) {
            write_u8(network_buffer_cursor(NET_SEND_BUFFER), v as u8);
            network_buffer_advance(NET_SEND_BUFFER, 1);
        }
    }
    if (exe::network_send_packet(session, read_usize(NET_SEND_BUFFER + network_buffer::DATA), read_u32(NET_SEND_BUFFER + network_buffer::POS), player) as i32) == 0 {
        display_network_error_box(0xc52);
    }
}

/// Comparer for `display_send_status_packet`.
pub(crate) unsafe extern "stdcall" fn compare_display_send_status_packet(session: usize, player: u32, status: u32) {
    // simplification: the packet is sent over the network once; proven in Rust mode
    original_fn::<unsafe extern "stdcall" fn(usize, u32, u32)>(IDX_DISPLAY_SEND_STATUS_PACKET)(session, player, status)
}

/// Applies a command packet of the screen manager to a Display: the first byte is the command and the rest its fields.
/// Returns -1 (in `ax`), -2 for command 4 and the item index for command 6.
///
/// The buffer position goes to 0 and the command byte is read (without room: the box network.h 143 and the copy takes
/// 0 for the original's uninitialised byte). Then: 0 `command_new_screen`; 1 `command_add_areas`; 2
/// `command_add_items`; 3 the whole screen drawn (`Display::draw_screen`, with `display_life::DWORD_551` = 1 first) unless
/// a keepalive was sent; 4 -2; 6 `command_rebuild_item`; 8 `command_change_item`; 9 when the screen is complete:
/// `DWORD_551` = 1, `display_list::SCREEN_CLEARED` = 0 and, once laid out (`LAID_OUT`), the whole screen drawn; 5, 7 and 10
/// to 26 `command_simple`; any other command nothing. It runs on every screen, also in a local game: the screen
/// manager writes each screen as packets and applies them to its local Display here.
/// Original: `Display::process_command_packet` (0x564080, display.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn display_process_command_packet(d: usize, session: usize, player: u32, buf: usize) -> u16 {
    let mut fr = CommandFrame::new();
    let mut r = PacketReader::new(buf, false);
    write_u32(buf + network_buffer::POS, 0);
    r.byte_to(fr.addr(frame::COMMAND), GET_CHAR);
    let command = fr.u8(frame::COMMAND) as i8;
    let result = match command {
        0 => command_new_screen(d, session, player, &mut r, &mut fr),
        1 => command_add_areas(d, session, player, &mut r, &mut fr),
        2 => command_add_items(d, session, player, &mut r, &mut fr),
        3 => {
            if read_u32(d + display_list::KEEPALIVE_SENT) == 0 {
                write_u32(d + display_life::DWORD_551, 1);
                draw_whole_screen(d);
            }
            0xffff
        }
        4 => 0xfffe,
        6 => command_rebuild_item(d, &mut r, &mut fr),
        8 => command_change_item(d, &mut r, &mut fr),
        9 => {
            if screen_complete(d) {
                write_u32(d + display_life::DWORD_551, 1);
                write_u32(d + display_list::SCREEN_CLEARED, 0);
                if read_u32(d + display_list::LAID_OUT) != 0 {
                    draw_whole_screen(d);
                }
            }
            0xffff
        }
        5 | 7 | 10..=26 => command_simple(d, session, player, buf, &mut r, &mut fr, command),
        _ => 0xffff,
    };
    result
}

/// Comparer for `display_process_command_packet`: by the command (`packet_check`), side by side over the whole Display,
/// the buffer header, the channel and what the command also writes (the send buffer's answer, the display packet buffer
/// of `add_area`, the snapshot pool, the input flags), with the result in `ax`; the commands that draw, send, play,
/// print, allocate or free run only in the original, checked against the copy's plan (`packet_plan`: the buffer
/// position and the Display fields).
pub(crate) unsafe extern "thiscall" fn compare_display_process_command_packet(d: usize, session: usize, player: u32, buf: usize) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u16>(IDX_DISPLAY_PROCESS_COMMAND_PACKET);
    if read_u32(buf + network_buffer::CAPACITY) < 1 {
        // simplification: the box of an empty packet does not run twice
        return o(d, session, player, buf);
    }
    let command = read_u8(read_usize(buf + network_buffer::DATA)) as i8;
    match packet_check(d, buf, command) {
        PacketCheck::OriginalOnly => o(d, session, player, buf),
        PacketCheck::SideBySide(reg) => {
            let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(d, session, player, buf), || display_process_command_packet(d, session, player, buf));
            SCOREBOARD.record(same && r == n, || format!("Display::process_command_packet command {} (screen packet of {} bytes): original={} rust={} differences {:?}",
                command, read_u32(buf + network_buffer::CAPACITY), r as i16, n as i16, first_differences(&mo, &mr)));
            r
        }
        PacketCheck::Plan => {
            let Some((pos, fields)) = packet_plan(d, buf, command) else {
                // simplification: a truncated packet's box does not run twice
                return o(d, session, player, buf);
            };
            let r = o(d, session, player, buf);
            let list = read_usize(d + display_life::MEDIA_LIST);
            let wrong: Vec<(usize, u32, u32)> = fields.iter().filter_map(|&(a, want, n)| {
                let got = match n {
                    1 => read_u8(a) as u32,
                    2 => read_u16(a) as u32,
                    23 if list == 0 => return None,
                    23 => read_u16(list + print_list::CHARS_PER_LINE) as u32,
                    _ => read_u32(a),
                };
                (got != want).then_some((a.wrapping_sub(d), want, got))
            }).collect();
            let at = read_u32(buf + network_buffer::POS);
            SCOREBOARD.record(at == pos && wrong.is_empty(), || format!("Display::process_command_packet command {} after the original: position {} (plan {}), fields off the plan (offset, plan, got) {:x?}",
                command, at, pos, wrong));
            r
        }
    }
}
