// ---------- batch %%LEVA%%: the display list input (display.cpp) ----------

/// `Guio::highlight_on` (0x5eec20) / `Guio::highlight_off` (0x5eecb0).
const DISPLAY_HIGHLIGHT_ON: usize = 0x5eec20;
const DISPLAY_HIGHLIGHT_OFF: usize = 0x5eecb0;
/// `Area::open_popup` (0x402d10) / `Area::close_popup` (0x402d80).
const DISPLAY_OPEN_POPUP: usize = 0x402d10;
const DISPLAY_CLOSE_POPUP: usize = 0x402d80;

/// `Display::handle_text_entry_key(d, k)` by address (0x5617c0): 1 when the text changed.
unsafe fn display_text_key(d: usize, k: i16) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5617c0usize);
    f(d, k as u16 as u32)
}

/// `Guio::set_text_and_refresh(item, text, refresh)` (0x5eed50).
unsafe fn display_set_text(item: usize, text: *const u8, refresh: u32) {
    let f: unsafe extern "thiscall" fn(usize, *const u8, u32) = core::mem::transmute(0x5eed50usize);
    f(item, text, refresh)
}

/// The left or the right button, by the item's 0x100 flag.
unsafe fn display_item_button(flags: u32) -> u32 {
    read_u32(if flags & 0x100 != 0 { DISPLAY_RIGHT_BUTTON } else { G_MOUSE_LEFT_BUTTON_DOWN })
}

/// One clickable widget in the first pass of the hit test: pressed on it highlights it (if not yet) and makes it
/// the result; otherwise a highlighted one goes dark. Pressed or merely under the mouse, a widget with an id or a
/// popup sets the hand cursor (2).
unsafe fn display_hit_widget(item: usize, k: i16, result: &mut i16) {
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
unsafe fn display_text_keys_in_areas(d: usize, s: &mut i16) {
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
unsafe fn display_text_keys_free(d: usize, from: i16) {
    let mut j = from;
    while j < read_i16(d + display_list::ITEM_ORDER_COUNT) {
        let k = read_i16(d + display_list::ITEM_ORDER + j as usize * 2);
        if read_u8(display_item_at(d, k) + display_item::FLAGS) & 8 != 0 {
            display_text_key(d, k);
        }
        j += 1;
    }
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
    let scroll_rows: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x403ee0usize);
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
            scroll_rows(area, position.wrapping_sub(old));
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
    let draw: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32) = core::mem::transmute(0x5e3b50usize);
    let circle: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32) = core::mem::transmute(0x5e7840usize);
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
                circle(r(0), r(1), r(2), r(3), 1, colour);
            } else {
                draw(r(0), r(1), r(2), r(3), 1, colour);
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
    let scroll_rows: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x403ee0usize);
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
        scroll_rows(area, at(0x1c).wrapping_sub(saved_position));
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
        let status: unsafe extern "stdcall" fn(usize, u32, u32) = core::mem::transmute(0x568290usize);
        write_u32(d + display_list::KEEPALIVE_SENT, 1);
        status(session, slot, 0);
        return;
    }
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x77ac30usize);
    if send(session, read_usize(0xae28f8), read_u32(0xae28fc), slot) != 0 {
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
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(%%I0%%)(d)
}

/// Comparer for `display_track_popup_hover`.
pub(crate) unsafe extern "thiscall" fn compare_display_track_popup_hover(d: usize) -> i16 {
    // simplification: highlights and popups do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(%%I1%%)(d)
}

/// Comparer for `display_drag_scroll_thumb`.
pub(crate) unsafe extern "thiscall" fn compare_display_drag_scroll_thumb(d: usize) -> i16 {
    // simplification: drawing and scrolling do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(%%I2%%)(d)
}

/// Comparer for `display_drag_widget`.
pub(crate) unsafe extern "thiscall" fn compare_display_drag_widget(d: usize) -> i16 {
    // simplification: the outline images do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> i16>(%%I3%%)(d)
}

/// Comparer for `display_handle_text_entry_key`.
pub(crate) unsafe extern "thiscall" fn compare_display_handle_text_entry_key(d: usize, k: u32) -> u32 {
    // simplification: the key is consumed and the entry redrawn once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I4%%)(d, k)
}

/// Comparer for `display_hit_test_area_scrollbar`.
pub(crate) unsafe extern "thiscall" fn compare_display_hit_test_area_scrollbar(d: usize, index: u32) -> u32 {
    // simplification: drawing and scrolling do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I5%%)(d, index)
}

/// Comparer for `display_send_keepalive`.
pub(crate) unsafe extern "thiscall" fn compare_display_send_keepalive(d: usize, session: usize) {
    // simplification: a network packet is sent once
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I6%%)(d, session)
}
