// ---------- batch %%LEVA%%: the Display's network side: a text entry's text back to its caller and a remote player's click sent to the server (display.cpp) ----------

/// The NetworkBuffer writers this file calls by address (thiscall, ret 4): put_long (0x5694a0, box line `network.h 136`;
/// not ported).
const FN_NETWORKBUFFER_PUT_LONG: usize = 0x5694a0;
/// A widget's dword the click packets send as its last field (`Guio::get_fields` gives +0x54, the entry target, but
/// the packets send +0x58) [inference].
const GUIO_CLICK_TAIL: usize = 0x58;

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

/// The fields `Guio::get_fields` (0x5ee4a0, thiscall, 19 out pointers, ret 0x4c) gives of a widget, in its order: the
/// flags, the rectangle, the grid column and row, the style, the two colours (words), the third colour, the font and the
/// text colour (words), the text pointer (the text, or 5 bytes in for an item of a popup area), the length limit, the id,
/// the data, the area (word) and the entry target.
#[derive(Default)]
struct GuioFields {
    flags: u32,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    column: u32,
    row: u32,
    style: u32,
    colour: u16,
    highlight: u16,
    colour_c: u32,
    font: u16,
    text_colour: u16,
    text: usize,
    max_length: u32,
    id: u32,
    data: u32,
    area: u16,
    target: u32,
}

/// `Guio::get_fields` (0x5ee4a0, by its address) on `item`.
unsafe fn click_guio_fields(item: usize) -> GuioFields {
    type Get = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u16, *mut u16,
        *mut u32, *mut u16, *mut u16, *mut usize, *mut u32, *mut u32, *mut u32, *mut u16, *mut u32);
    let get: Get = core::mem::transmute(0x5ee4a0usize);
    let mut f = GuioFields::default();
    get(item, &mut f.flags, &mut f.x1, &mut f.y1, &mut f.x2, &mut f.y2, &mut f.column, &mut f.row, &mut f.style, &mut f.colour, &mut f.highlight,
        &mut f.colour_c, &mut f.font, &mut f.text_colour, &mut f.text, &mut f.max_length, &mut f.id, &mut f.data, &mut f.area, &mut f.target);
    f
}

/// The widget's text as the click packet copies it (the text with its terminator; empty with no text).
unsafe fn guio_text_copy(text: usize) -> Vec<u8> {
    if text == 0 {
        return vec![0];
    }
    let n = c_string_len(text) + 1;
    core::slice::from_raw_parts(text as *const u8, n).to_vec()
}

/// One inline write into the network send buffer (`NET_SEND_BUFFER`): with room for `size` bytes it is written and
/// the position advances; without, the box `network.h <line>`.
unsafe fn send_buffer_put(value: u32, size: u32, line: u32) {
    if !network_buffer_room(NET_SEND_BUFFER, size, line) {
        return;
    }
    let at = network_buffer_cursor(NET_SEND_BUFFER);
    match size {
        1 => write_u8(at, value as u8),
        2 => write_u16(at, value as u16),
        _ => write_u32(at, value),
    }
    network_buffer_advance(NET_SEND_BUFFER, size);
}

/// The "Network Error" box of display.cpp at `line` (as `display_send_keepalive`'s): the version string, then the box
/// with the game's title and message texts (no mouse reset).
unsafe fn display_network_error(line: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(DISPLAY_CPP, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    format(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line);
    text(0xae28bc, 0x9c3814);
    text(0xae26c4, 0x9c37b0, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bcusize, 0xae26c4usize, 0);
}

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

/// Comparer for `guio_copy_text_to_caller`: side by side over the caller's buffer (the text's length with its
/// terminator).
pub(crate) unsafe extern "thiscall" fn compare_guio_copy_text_to_caller(item: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let target = read_usize(item + guio_fields::ENTRY_TARGET);
    if target == 0 || read_u8(item + display_item::FLAGS) & 8 == 0 {
        return o(item);
    }
    let reg = [(target, c_string_len(item + display_item::TEXT) + 1)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(item), || guio_copy_text_to_caller(item));
    SCOREBOARD.record(same, || format!("guio_copy_text_to_caller(item {:#x}): differences {:?}", item, first_differences(&mo, &mr)));
}

// simplification: the packet is sent to the server: it does not run twice. Original only; the plan: the packet starts
// with command 6 and the widget, and a sent click clears the Display's DWORD_551.
/// Comparer for `display_send_widget_click`.
pub(crate) unsafe extern "thiscall" fn compare_display_send_widget_click(d: usize, session: usize, widget: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(d, session, widget);
    let data = read_usize(NET_SEND_BUFFER);
    let head_ok = data == 0 || read_u32(NET_SEND_BUFFER + network_buffer::POS) < 3 || (read_u8(data) == 6 && read_u16(data + 1) == widget as u16);
    SCOREBOARD.record(head_ok && (r == 0 || read_u32(d + display_life::DWORD_551) == 0), || format!("send_widget_click(widget {}): returned {}, packet head {:x?}",
        widget as i16, r, if data == 0 { &[][..] } else { core::slice::from_raw_parts(data as *const u8, 3) }));
    r
}
