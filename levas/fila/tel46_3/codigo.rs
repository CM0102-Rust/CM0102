// ---------- batch %%LEVA%%: sending one section of the screen to a network player: the header, the widgets with their changes, the area display packets and the scroll rows (display.cpp) ----------

/// The sections `Display::send_screen_section` sends (the first byte of each packet).
const SECTION_HEADER: u8 = 0;
const SECTION_WIDGETS: u8 = 1;
const SECTION_AREAS: u8 = 2;
const SECTION_SCROLL_ROWS: u8 = 3;
/// The command of one area's scroll row packet.
const PACKET_AREA_SCROLL_ROW: u8 = 0x10;
/// The green mask of the screen's pixel format [inference]: 0x7e0 is 5-6-5, which the header sends as 1.
const SCREEN_GREEN_MASK: usize = 0xae316c;
const GREEN_MASK_565: u32 = 0x7e0;
/// One channel's slot of the widget snapshot pool (`DISPLAY_SNAPSHOT_POOL`): 1200 widgets and their count (a word).
const SNAPSHOT_SLOT_SIZE: i32 = 0x74042;
const SNAPSHOT_COUNT: usize = 0x74040;
/// The widget field mask: the item is coded as changes from the snapshot (`MASK_DELTA`), or copies the look of an
/// earlier item (`MASK_SAME_AS`, then the bits of that look are dropped: `MASK_LOOK`).
const MASK_DELTA: u32 = 0x100000;
const MASK_SAME_AS: u32 = 0x80000;
const MASK_LOOK: u32 = 0x3e03;
const MASK_TEXT: u32 = 0x4000;
/// How far back an item's look is searched for (the 9 items before it) and the bytes the full size drops then.
const SAME_AS_REACH: i32 = 10;
const SAME_AS_SAVING: u32 = 0x11;
/// The widget's dword after its colours (`Guio::get_fields`' third colour; no name yet).
const GUIO_COLOUR_C: usize = 0x3c;

/// One widget field the packet can carry: its mask bit, the bytes it takes, the `NetworkBuffer` writer and how to read
/// it from the `Guio::get_fields` result.
struct WidgetField {
    bit: u32,
    size: u32,
    put: unsafe extern "thiscall" fn(usize, u32),
    get: fn(&GuioFields) -> u32,
}

/// The widget fields in packet order (the text, 0x4000, goes between the text colour and the length limit and is
/// written apart; the "same as" index, 0x80000, last).
const WIDGET_FIELDS: [WidgetField; 18] = [
    WidgetField { bit: 1, size: 4, put: network_buffer_put_ulong, get: |f| f.flags },
    WidgetField { bit: 2, size: 4, put: network_buffer_put_ulong, get: |f| f.style },
    WidgetField { bit: 4, size: 4, put: network_buffer_put_int, get: |f| f.target },
    WidgetField { bit: 8, size: 2, put: network_buffer_put_short, get: |f| f.x1 },
    WidgetField { bit: 0x10, size: 2, put: network_buffer_put_short, get: |f| f.y1 },
    WidgetField { bit: 0x20, size: 2, put: network_buffer_put_short, get: |f| f.x2 },
    WidgetField { bit: 0x40, size: 2, put: network_buffer_put_short, get: |f| f.y2 },
    WidgetField { bit: 0x80, size: 1, put: network_buffer_put_char, get: |f| f.column },
    WidgetField { bit: 0x100, size: 2, put: network_buffer_put_short, get: |f| f.row },
    WidgetField { bit: 0x200, size: 2, put: network_buffer_put_ushort, get: |f| f.colour as u32 },
    WidgetField { bit: 0x400, size: 2, put: network_buffer_put_ushort, get: |f| f.highlight as u32 },
    WidgetField { bit: 0x800, size: 4, put: network_buffer_put_ulong, get: |f| f.colour_c },
    WidgetField { bit: 0x1000, size: 1, put: network_buffer_put_char, get: |f| f.font as u32 },
    WidgetField { bit: 0x2000, size: 2, put: network_buffer_put_ushort, get: |f| f.text_colour as u32 },
    WidgetField { bit: 0x8000, size: 2, put: network_buffer_put_short, get: |f| f.max_length },
    WidgetField { bit: 0x10000, size: 4, put: network_buffer_put_long, get: |f| f.id },
    WidgetField { bit: 0x20000, size: 4, put: network_buffer_put_int, get: |f| f.data },
    WidgetField { bit: 0x40000, size: 2, put: network_buffer_put_short, get: |f| f.area as u32 },
];

/// The words of `GuioFields` compare as words; the others as dwords (`get` gives the word zero-extended, so a plain
/// comparison is the same).
fn widget_field_differs(field: &WidgetField, a: &GuioFields, b: &GuioFields) -> bool {
    (field.get)(a) != (field.get)(b)
}

/// The snapshot slot of the current channel (`G_SCREEN_MANAGER_CURRENT_SLOT`) in the pool, in 32 bits as the original.
unsafe fn snapshot_slot(pool: usize) -> usize {
    pool.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32).wrapping_mul(SNAPSHOT_SLOT_SIZE) as u32 as usize)
}

/// A text into the buffer as the original's inline copies: its length with the terminator by
/// `NetworkBuffer::put_short` (box network.h 134), then the bytes and the terminator with no room check, the position
/// advanced by them (the length sign-extended from its word).
unsafe fn put_text_unchecked(b: usize, text: usize) {
    let len = c_string_len(text) as u32 + 1;
    network_buffer_put_short(b, len);
    let n = len as i16 as i32;
    core::ptr::copy(text as *const u8, network_buffer_cursor(b) as *mut u8, n as usize);
    network_buffer_advance(b, n as u32);
}

/// Sends the buffer `b` (its bytes and position) to `player` (`Network::send_packet`); failing, the "Network Error"
/// box of display.cpp at `line` and false.
unsafe fn send_section_packet(session: usize, b: usize, player: u32, line: u32) -> bool {
    if exe::network_send_packet(session, read_usize(b + network_buffer::DATA), read_u32(b + network_buffer::POS), player) == 0 {
        display_network_error(line);
        return false;
    }
    true
}

/// Section 0's body: the background picture (`ScreenManager::get_background_picture`, `put_text_unchecked`), the colour
/// format (1 when `SCREEN_GREEN_MASK` is `GREEN_MASK_565`), the area count and the widget count (words), the focus flags
/// (byte) and the focus item (word).
unsafe fn put_screen_header(d: usize, b: usize) {
    put_text_unchecked(b, exe::screenmanager_get_background_picture(G_SCREEN_MANAGER));
    network_buffer_put_char(b, (read_u32(SCREEN_GREEN_MASK) == GREEN_MASK_565) as u32); // tel46_3: the colour format
    network_buffer_put_short(b, read_u16(d + display_life::AREA_COUNT) as u32);
    network_buffer_put_short(b, read_u16(d + display_list::ITEM_COUNT) as u32);
    network_buffer_put_char(b, read_u8(d + display::FOCUS_FLAGS) as u32);
    network_buffer_put_short(b, read_u16(d + display::FOCUS_ITEM) as u32);
}

/// The full mask of a widget and its size: each field that is not zero (the text when it is not empty, its length
/// without the terminator).
unsafe fn full_widget_mask(f: &GuioFields) -> (u32, u32) {
    let (mut mask, mut size) = (0u32, 0u32);
    for field in WIDGET_FIELDS.iter() {
        if (field.get)(f) != 0 {
            mask |= field.bit;
            size = size.wrapping_add(field.size);
        }
    }
    if f.text != 0 && read_u8(f.text) != 0 {
        mask |= MASK_TEXT;
        size = size.wrapping_add(c_string_len(f.text) as u32);
    }
    (mask, size)
}

/// The change mask of a widget against its snapshot `s` and its size: `MASK_DELTA` and each field that differs (the
/// text only when both texts exist and differ, its length without the terminator).
unsafe fn delta_widget_mask(f: &GuioFields, s: &GuioFields) -> (u32, u32) {
    let (mut mask, mut size) = (MASK_DELTA, 0u32);
    for field in WIDGET_FIELDS.iter() {
        if widget_field_differs(field, f, s) {
            mask |= field.bit;
            size = size.wrapping_add(field.size);
        }
    }
    if f.text != 0 && s.text != 0 && c_string_bytes(f.text) != c_string_bytes(s.text) {
        mask |= MASK_TEXT;
        size = size.wrapping_add(c_string_len(f.text) as u32);
    }
    (mask, size)
}

/// The earlier item (k − 1 down to k − 9, stopping at 0) whose look (flags, style, colour, highlight colour, third
/// colour, font and text colour, read from the item table) equals the widget's; −1 when none.
unsafe fn same_look_item(d: usize, k: i16, f: &GuioFields) -> i16 {
    let mut j = k.wrapping_sub(1);
    while j >= 0 && (j as i32) > k as i32 - SAME_AS_REACH {
        let item = display_item_at(d, j);
        if f.flags == read_u32(item + display_item::FLAGS) && f.style == read_u32(item + display_item::STYLE)
            && f.colour == read_u16(item + guio_fields::COLOUR) && f.highlight == read_u16(item + guio_fields::HIGHLIGHT_COLOUR)
            && f.colour_c == read_u32(item + GUIO_COLOUR_C) && f.font == read_u16(item + display_item::FONT)
            && f.text_colour == read_u16(item + guio_fields::TEXT_COLOUR)
        {
            return j;
        }
        j -= 1;
    }
    -1
}

/// One widget into the widget section: its fields (`Guio::get_fields`, by address) coded with the smaller mask.
///
/// The full mask (`full_widget_mask`); with a widget among the 9 before it with the same look (`same_look_item`) the
/// look bits are dropped (`MASK_LOOK`), `MASK_SAME_AS` added and the size lowered by `SAME_AS_SAVING` (17, though the
/// look takes up to 19 bytes; the size only chooses). When the Display has a channel (`display_life::CHANNEL`), there is
/// a snapshot pool and the current slot's snapshot has this widget (k below its count), the change mask against it
/// (`delta_widget_mask`) is used when its size is below the full one (unsigned). The mask (`put_ulong`), then each
/// field of the mask in `WIDGET_FIELDS` order, the text (`put_text_unchecked`) after the text colour, and with
/// `MASK_SAME_AS` the earlier item (`put_short`).
unsafe fn put_widget(d: usize, b: usize, k: i16) {
    let f = click_guio_fields(display_item_at(d, k));
    let mut delta = (0u32, 0u32);
    let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
    if read_u32(d + display_life::CHANNEL) != 0 && pool != 0 {
        let slot = snapshot_slot(pool);
        if k < read_i16(slot + SNAPSHOT_COUNT) {
            covered!("tel46_3 a widget against its snapshot");
            let s = click_guio_fields(slot.wrapping_add((k as i32 * display_list::ITEM_SIZE as i32) as usize));
            delta = delta_widget_mask(&f, &s);
        }
    }
    let (mut full, mut full_size) = full_widget_mask(&f);
    let same = same_look_item(d, k, &f);
    if same != -1 {
        covered!("tel46_3 a widget looks like an earlier one");
        full = (full & !MASK_LOOK) | MASK_SAME_AS;
        full_size = full_size.wrapping_sub(SAME_AS_SAVING);
    }
    let mask = if delta.0 != 0 && delta.1 < full_size { delta.0 } else { full }; // tel46_3: the smaller mask
    network_buffer_put_ulong(b, mask);
    for field in WIDGET_FIELDS.iter() {
        if field.bit == 0x8000 && mask & MASK_TEXT != 0 {
            put_text_unchecked(b, f.text);
        }
        if mask & field.bit != 0 {
            (field.put)(b, (field.get)(&f));
        }
    }
    if mask & MASK_SAME_AS != 0 {
        network_buffer_put_short(b, same as u16 as u32);
    }
}

/// Section 1's body: every widget of the Display (`display_list::ITEM_COUNT`, read anew each turn), `put_widget`.
unsafe fn put_widgets(d: usize, b: usize) {
    let mut k: i16 = 0;
    while k < read_i16(d + display_list::ITEM_COUNT) {
        put_widget(d, b, k);
        k = k.wrapping_add(1);
    }
}

/// After the widget section is sent, the current slot's snapshot takes the Display's widgets: when the Display has a
/// channel and there is a pool, the slot's count = `display_list::ITEM_COUNT` and the item table (count × 0x18c bytes)
/// is copied there.
unsafe fn save_widget_snapshot(d: usize) {
    let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
    if read_u32(d + display_life::CHANNEL) == 0 || pool == 0 {
        return;
    }
    let count = read_i16(d + display_list::ITEM_COUNT);
    write_i16(snapshot_slot(pool) + SNAPSHOT_COUNT, count);
    let size = (count as i32 * display_list::ITEM_SIZE as i32) as usize;
    core::ptr::copy((d + display_list::ITEMS) as *const u8, snapshot_slot(read_usize(DISPLAY_SNAPSHOT_POOL)) as *mut u8, size); // tel46_3: the snapshot
}

/// Section 3: for each area (`display_life::AREA_COUNT`) with a pending scroll row (`area_scroll::PENDING_ROW` ≠ −1)
/// a packet (position 0): `PACKET_AREA_SCROLL_ROW` (box 133), the area (word, 134), the row (`put_long`), sent
/// (failing: the box display.cpp 2287 and 0); the bytes of the packets sent add up. Then the section's end packet
/// (position 0, the byte 3) sent (failing: the box display.cpp 2304 and 0). Returns the sum plus the last packet.
unsafe fn send_scroll_rows(d: usize, session: usize, player: u32) -> i32 {
    let b = NET_SEND_BUFFER;
    let mut total: u32 = 0;
    let mut a: i16 = 0;
    while a < read_i16(d + display_life::AREA_COUNT) {
        let row = read_u32(display_area_at(d, a) + area_scroll::PENDING_ROW);
        if row as i32 != -1 {
            covered!("tel46_3 an area's scroll row");
            write_u32(b + network_buffer::POS, 0);
            network_buffer_put_char(b, PACKET_AREA_SCROLL_ROW as u32);
            network_buffer_put_short(b, a as u16 as u32);
            network_buffer_put_long(b, row);
            if !send_section_packet(session, b, player, 0x8ef) {
                return 0;
            }
            total = total.wrapping_add(read_u32(b + network_buffer::POS));
        }
        a = a.wrapping_add(1);
    }
    write_u32(b + network_buffer::POS, 0);
    network_buffer_put_char(b, SECTION_SCROLL_ROWS as u32);
    if !send_section_packet(session, b, player, 0x900) {
        return 0;
    }
    total.wrapping_add(read_u32(b + network_buffer::POS)) as i32
}

/// Rule: one section of the screen goes to a network player: the header, the widgets (each as changes from what the
/// player last got, or as a copy of an earlier widget's look, whichever is shorter), the area display packets, or the
/// pending scroll rows. Returns the bytes sent, 0 when a send failed.
///
/// The network send buffer (`NET_SEND_BUFFER`) starts at position 0 with the section byte (box network.h 133).
/// Section 0 (`SECTION_HEADER`): `put_screen_header`, sent (failing: box display.cpp 2086 and 0); returns the position.
/// Section 2 (`SECTION_AREAS`): the display packet buffer (`DISPLAY_PACKET_BUFFER`) is sent as it is (failing: box
/// display.cpp 2101 and 0); returns its position. Section 1 (`SECTION_WIDGETS`): `put_widgets`, sent (failing: box
/// display.cpp 2252 and 0), then `save_widget_snapshot`; returns the position. Section 3 (`SECTION_SCROLL_ROWS`):
/// `send_scroll_rows`. Another section sends nothing. Defect of the original: for another section it returns a stack
/// dword it never wrote; the copy returns 0.
/// Original: `Display::send_screen_section` (0x562370, display.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn display_send_screen_section(d: usize, session: usize, player: u32, section: u32) -> i32 {
    covered!("tel46_3 send a screen section");
    let b = NET_SEND_BUFFER;
    let section = section as u8;
    write_u32(b + network_buffer::POS, 0);
    network_buffer_put_char(b, section as u32); // tel46_3: the section byte
    match section {
        SECTION_HEADER => {
            put_screen_header(d, b);
            if !send_section_packet(session, b, player, 0x826) {
                return 0;
            }
            read_u32(b + network_buffer::POS) as i32
        }
        SECTION_AREAS => {
            if !send_section_packet(session, DISPLAY_PACKET_BUFFER, player, 0x835) {
                return 0;
            }
            read_u32(DISPLAY_PACKET_BUFFER + network_buffer::POS) as i32
        }
        SECTION_WIDGETS => {
            put_widgets(d, b);
            if !send_section_packet(session, b, player, 0x8cc) {
                return 0;
            }
            save_widget_snapshot(d);
            read_u32(b + network_buffer::POS) as i32
        }
        SECTION_SCROLL_ROWS => send_scroll_rows(d, session, player),
        _ => 0,
    }
}

/// The bytes a section's body builder writes, built into a scratch NetworkBuffer large enough that no room check fails
/// (1 MB): the section byte first. None when the body passes the real buffer's capacity (the original shows a box
/// there and the plan stops).
unsafe fn scratch_section(section: u8, build: impl FnOnce(usize)) -> Option<Vec<u8>> {
    let mut bytes = vec![0u8; 0x100000];
    let mut header = [bytes.as_mut_ptr() as u32, 0, 0x80000];
    let b = header.as_mut_ptr() as usize;
    network_buffer_put_char(b, section as u32);
    build(b);
    let n = header[1] as usize;
    if n > read_u32(NET_SEND_BUFFER + network_buffer::CAPACITY) as usize {
        return None;
    }
    bytes.truncate(n);
    Some(bytes)
}

/// Comparer for `display_send_screen_section`: only the original (it sends packets); the plan is built first, from the
/// state before. Sections 0 and 1: the copy's packet (`scratch_section` with `put_screen_header` / `put_widgets`) must be
/// what the original left in the send buffer and its length the result; section 1 also leaves the snapshot with the
/// widget count. Section 2: the result the display packet buffer's position. Section 3: the result the 7 bytes of each
/// pending scroll row plus 1. Any other: the section byte alone in the buffer. A result of 0 (a send failed) checks
/// nothing more.
pub(crate) unsafe extern "thiscall" fn compare_display_send_screen_section(d: usize, session: usize, player: u32, section: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32>(%%I0%%);
    let s = section as u8;
    let packet = match s {
        SECTION_HEADER => scratch_section(s, |b| put_screen_header(d, b)),
        SECTION_WIDGETS => scratch_section(s, |b| put_widgets(d, b)),
        _ => None,
    };
    let rows = (0..read_i16(d + display_life::AREA_COUNT).max(0)).filter(|&a| read_i32(display_area_at(d, a) + area_scroll::PENDING_ROW) != -1).count();
    let r = o(d, session, player, section);
    let pos = read_u32(NET_SEND_BUFFER + network_buffer::POS) as usize;
    let data = read_usize(NET_SEND_BUFFER + network_buffer::DATA);
    let got = if data == 0 { Vec::new() } else { core::slice::from_raw_parts(data as *const u8, pos).to_vec() };
    let ok = match s {
        _ if r == 0 && s <= SECTION_SCROLL_ROWS => true,
        SECTION_HEADER | SECTION_WIDGETS => match &packet {
            None => true,
            Some(p) => {
                let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
                let snapshot_ok = s != SECTION_WIDGETS || read_u32(d + display_life::CHANNEL) == 0 || pool == 0
                    || read_i16(snapshot_slot(pool) + SNAPSHOT_COUNT) == read_i16(d + display_list::ITEM_COUNT);
                &got == p && r as usize == p.len() && snapshot_ok
            }
        },
        SECTION_AREAS => r as u32 == read_u32(DISPLAY_PACKET_BUFFER + network_buffer::POS),
        SECTION_SCROLL_ROWS => r as usize == rows * 7 + 1,
        _ => got.len() <= 1,
    };
    SCOREBOARD.record(ok, || format!("Display::send_screen_section(section {}): returned {}, packet {:x?}, plan {:x?} ({} scroll rows)", s, r,
        &got[..got.len().min(48)], packet.as_ref().map(|p| &p[..p.len().min(48)]), rows));
    r
}
