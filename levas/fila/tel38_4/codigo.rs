// ---------- batch %%LEVA%%: the screen manager's widget packets: setting a widget's flags, style, colours, text flags, font, text, id and data on the current screen, locally or over the network (scrman.cpp) ----------

/// The Display that echoes a remote manager's packets while his own Display blocks them [inference: +0x132ea5 of the
/// screen manager, after the local Display].
const SCRMAN_ECHO_DISPLAY: usize = 0x132ea5;

/// Appends `size` bytes (1, 2 or 4) of `value` to the screen manager's network command buffer (`screen_history::NET_BUFFER`,
/// the position `NET_BUFFER_POS`, the capacity `NET_BUFFER_CAPACITY`), as network.h's inline writers do: no room
/// (position + size above the capacity, unsigned) gives the box `network.h <line>` and nothing written.
unsafe fn scrman_net_put(sm: usize, value: u32, size: u32, line: u32) {
    use screen_history as h;
    let pos = read_u32(sm + h::NET_BUFFER_POS);
    if pos.wrapping_add(size) > read_u32(sm + h::NET_BUFFER_CAPACITY) {
        show_error_at(NETWORK_H, line);
        return;
    }
    let at = read_usize(sm + h::NET_BUFFER) + pos as usize;
    match size {
        1 => write_u8(at, value as u8),
        2 => write_u16(at, value as u16),
        _ => write_u32(at, value),
    }
    write_u32(sm + h::NET_BUFFER_POS, pos + size);
}

/// Starts a widget packet: the buffer position to 0, the command 8, the widget (word) and the sub-command (byte).
unsafe fn scrman_widget_packet(sm: usize, widget: u32, sub: u8) {
    write_u32(sm + screen_history::NET_BUFFER_POS, 0);
    scrman_net_put(sm, 8, 1, 0x85); // tel38_4: the widget command
    scrman_net_put(sm, widget, 2, 0x86);
    scrman_net_put(sm, sub as u32, 1, 0x85);
}

/// Delivers the packet in the buffer for the current record: a local one (`screen_history::NETWORK_PLAYER` = 0) has it
/// applied to the local Display (`Display::process_command_packet` 0x564080, thiscall, with the session and 0); a remote
/// one whose Display blocks it (`DISPLAY_REDRAW_BLOCKED`) has it applied to the echo Display; otherwise it is sent to
/// his player (`Network::send_packet` 0x77ac30, thiscall; failing: the box `scrman.cpp <line>`).
unsafe fn scrman_deliver_packet(sm: usize, line: u32) {
    use screen_history as h;
    let process: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x564080usize);
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x77ac30usize);
    let rec = screen_record(sm);
    let player = read_u16(rec + h::NETWORK_PLAYER);
    if player == 0 {
        process(sm + h::LOCAL_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
        return;
    }
    covered!("tel38_4 widget packet for a remote manager");
    if read_u32(read_usize(rec + screen_manager::DISPLAY) + DISPLAY_REDRAW_BLOCKED) != 0 {
        process(sm + SCRMAN_ECHO_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
        return;
    }
    if send(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), read_u32(sm + h::NET_BUFFER_POS), player as u32) == 0 {
        show_error_at(SCRMAN_CPP, line);
    }
}

/// The body of the fixed-size widget setters: nothing for widget −1 (unless `always`); the packet (command 8, the
/// widget, `sub`, then `value` in `size` bytes with the box line `put_line`) and its delivery (`send_line`).
unsafe fn scrman_set_widget(sm: usize, widget: u32, sub: u8, value: u32, size: u32, put_line: u32, send_line: u32, always: bool) {
    if !always && widget as u16 == 0xffff {
        return;
    }
    scrman_widget_packet(sm, widget, sub);
    scrman_net_put(sm, value, size, put_line);
    scrman_deliver_packet(sm, send_line);
}

/// The packet a widget setter leaves in the buffer: command 8, the widget word, the sub-command and the value bytes.
fn scrman_expected_packet(widget: u32, sub: u8, value: &[u8]) -> Vec<u8> {
    let mut p = vec![8, widget as u8, (widget >> 8) as u8, sub];
    p.extend_from_slice(value);
    p
}

// simplification: the packet is applied to the Display (it may redraw) or sent: it does not run twice. Original only;
// the plan: the packet in the buffer, when there was room for it.
/// The comparer of a widget setter.
unsafe fn compare_scrman_set_widget(name: &str, sm: usize, widget: u32, skip: bool, expected: Vec<u8>, original: impl FnOnce()) {
    use screen_history as h;
    let room = read_u32(sm + h::NET_BUFFER_CAPACITY) as usize >= expected.len();
    original();
    if skip || !room {
        return;
    }
    let pos = read_u32(sm + h::NET_BUFFER_POS) as usize;
    let got = core::slice::from_raw_parts(read_usize(sm + h::NET_BUFFER) as *const u8, pos.min(expected.len()));
    SCOREBOARD.record(pos == expected.len() && got == &expected[..], || format!("{}(widget {}): packet {:x?}, expected {:x?}", name, widget as i16, got, expected));
}

/// Rule: a widget of the current screen gets its flags (sub-command 0, a dword).
///
/// `scrman_set_widget` with widget −1 doing nothing, the dword written with the box line `network.h 141`, the send
/// failure `scrman.cpp 2606`.
/// Original: `ScreenManager::set_widget_flags` (0x800580, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_flags(sm: usize, widget: u32, flags: u32) {
    scrman_set_widget(sm, widget, 0, flags, 4, 0x8d, 0xa2e, false); // tel38_4: the flags sub-command
}

/// Comparer for `screen_manager_set_widget_flags`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_flags(sm: usize, widget: u32, flags: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I0%%);
    compare_scrman_set_widget("screen_manager_set_widget_flags", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 0, &flags.to_le_bytes()),
        || o(sm, widget, flags));
}

/// Rule: a widget of the current screen gets its style (sub-command 2, a dword; box line `network.h 141`, send failure
/// `scrman.cpp 2679`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_style` (0x800d30, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_style(sm: usize, widget: u32, style: u32) {
    scrman_set_widget(sm, widget, 2, style, 4, 0x8d, 0xa77, false);
}

/// Comparer for `screen_manager_set_widget_style`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_style(sm: usize, widget: u32, style: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I1%%);
    compare_scrman_set_widget("screen_manager_set_widget_style", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 2, &style.to_le_bytes()),
        || o(sm, widget, style));
}

/// Rule: a widget of the current screen gets its colour (sub-command 3, a word; box line `network.h 139`, send failure
/// `scrman.cpp 2714`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_colour` (0x801040, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_colour(sm: usize, widget: u32, colour: u32) {
    scrman_set_widget(sm, widget, 3, colour, 2, 0x8b, 0xa9a, false);
}

/// Comparer for `screen_manager_set_widget_colour`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_colour(sm: usize, widget: u32, colour: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I2%%);
    compare_scrman_set_widget("screen_manager_set_widget_colour", sm, widget, widget as u16 == 0xffff,
        scrman_expected_packet(widget, 3, &(colour as u16).to_le_bytes()), || o(sm, widget, colour));
}

/// Rule: a widget of the current screen gets its text flags (sub-command 5, a dword; box line `network.h 141`, send
/// failure `scrman.cpp 2784`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_text_flags` (0x801350, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_text_flags(sm: usize, widget: u32, flags: u32) {
    scrman_set_widget(sm, widget, 5, flags, 4, 0x8d, 0xae0, false);
}

/// Comparer for `screen_manager_set_widget_text_flags`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_text_flags(sm: usize, widget: u32, flags: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I3%%);
    compare_scrman_set_widget("screen_manager_set_widget_text_flags", sm, widget, widget as u16 == 0xffff,
        scrman_expected_packet(widget, 5, &flags.to_le_bytes()), || o(sm, widget, flags));
}

/// Rule: a widget of the current screen gets its font (sub-command 6, a byte; box line `network.h 133`, send failure
/// `scrman.cpp 2817`). Unlike the other setters, widget −1 is not skipped: the packet goes out with it.
/// Original: `ScreenManager::set_widget_font` (0x801660, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_font(sm: usize, widget: u32, font: u32) {
    scrman_set_widget(sm, widget, 6, font, 1, 0x85, 0xb01, true); // tel38_4: the font has no −1 check
}

/// Comparer for `screen_manager_set_widget_font`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_font(sm: usize, widget: u32, font: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I4%%);
    compare_scrman_set_widget("screen_manager_set_widget_font", sm, widget, false, scrman_expected_packet(widget, 6, &[font as u8]), || o(sm, widget, font));
}

/// Rule: a widget of the current screen gets its text colour (sub-command 7, a word; box line `network.h 139`, send
/// failure `scrman.cpp 2852`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_text_colour` (0x801960, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_text_colour(sm: usize, widget: u32, colour: u32) {
    scrman_set_widget(sm, widget, 7, colour, 2, 0x8b, 0xb24, false);
}

/// Comparer for `screen_manager_set_widget_text_colour`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_text_colour(sm: usize, widget: u32, colour: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I5%%);
    compare_scrman_set_widget("screen_manager_set_widget_text_colour", sm, widget, widget as u16 == 0xffff,
        scrman_expected_packet(widget, 7, &(colour as u16).to_le_bytes()), || o(sm, widget, colour));
}

/// Rule: a widget of the current screen gets a new text (sub-command 8, the string with its terminator, after its
/// length as a word); nothing for widget −1.
///
/// The packet as `scrman_widget_packet`; the length (strlen + 1, a word, box line `network.h 134`); then the string
/// with its terminator is copied after it and the position advances, with no room check (defect of the original: a
/// long text runs past the buffer). Delivery with the send failure `scrman.cpp 2887`.
/// Original: `ScreenManager::set_widget_text` (0x801c70, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_text(sm: usize, widget: u32, text: usize) {
    use screen_history as h;
    if widget as u16 == 0xffff {
        return;
    }
    scrman_widget_packet(sm, widget, 8);
    let len = c_string_len(text) + 1;
    scrman_net_put(sm, len as u32, 2, 0x86);
    let pos = read_u32(sm + h::NET_BUFFER_POS);
    core::ptr::copy_nonoverlapping(text as *const u8, (read_usize(sm + h::NET_BUFFER) + pos as usize) as *mut u8, len); // tel38_4: the text copied unchecked
    write_u32(sm + h::NET_BUFFER_POS, pos.wrapping_add(len as u16 as i16 as u32));
    scrman_deliver_packet(sm, 0xb47);
}

/// Comparer for `screen_manager_set_widget_text`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_text(sm: usize, widget: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(%%I6%%);
    let skip = widget as u16 == 0xffff || text == 0;
    let mut value = Vec::new();
    if !skip {
        let len = c_string_len(text) + 1;
        value.extend_from_slice(&(len as u16).to_le_bytes());
        value.extend_from_slice(core::slice::from_raw_parts(text as *const u8, len));
    }
    compare_scrman_set_widget("screen_manager_set_widget_text", sm, widget, skip, scrman_expected_packet(widget, 8, &value), || o(sm, widget, text));
}

/// Rule: a widget of the current screen gets its id (sub-command 9, a dword; box line `network.h 136`, send failure
/// `scrman.cpp 2922`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_id` (0x801fc0, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_id(sm: usize, widget: u32, id: u32) {
    scrman_set_widget(sm, widget, 9, id, 4, 0x88, 0xb6a, false);
}

/// Comparer for `screen_manager_set_widget_id`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_id(sm: usize, widget: u32, id: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I7%%);
    compare_scrman_set_widget("screen_manager_set_widget_id", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 9, &id.to_le_bytes()),
        || o(sm, widget, id));
}

/// Rule: a widget of the current screen gets its data (sub-command 0xa, a dword; box line `network.h 137`, send failure
/// `scrman.cpp 2957`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_data` (0x8022d0, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_data(sm: usize, widget: u32, data: u32) {
    scrman_set_widget(sm, widget, 0xa, data, 4, 0x89, 0xb8d, false);
}

/// Comparer for `screen_manager_set_widget_data`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_data(sm: usize, widget: u32, data: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I8%%);
    compare_scrman_set_widget("screen_manager_set_widget_data", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 0xa, &data.to_le_bytes()),
        || o(sm, widget, data));
}
