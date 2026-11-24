// ---------- batch %%LEVA%%: the network buffer's dword and sized block writers and the widget animation (network.h) ----------

/// The animation timers of `Display::animate_widgets`: the time (`utils_get_time_ms`) the marquee texts last turned
/// and the time the blinking colours last swapped.
const ANIMATE_TEXT_TIME: usize = 0xae2a28;
const ANIMATE_COLOUR_TIME: usize = 0xae2a2c;
/// `display_item::FLAGS` bits: the text turns like a marquee (0x800), the colour and text colour swap (0x1000).
const ANIMATED_TEXT: u32 = 0x800;
const ANIMATED_COLOURS: u32 = 0x1000;
/// The marquee text's marker between its head and its rest.
const MARQUEE_MARK: u8 = 1;
/// "%s%c%s" (.rdata).
const FORMAT_MARQUEE: usize = 0x98b4c8;

/// Puts a dword: with room for 4 (else the box network.h 136) it is written at the position, which advances by 4.
/// Original: `NetworkBuffer::put_long` (0x5694a0, network.h, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_long(buf: usize, value: u32) {
    covered!("tel46_1 put a long");
    if network_buffer_room(buf, 4, 0x88) {
        write_u32(network_buffer_cursor(buf), value); // tel46_1: the long
        network_buffer_advance(buf, 4);
    }
}

/// Puts a sized block: its length as a word, then the bytes.
///
/// With room for 2 (else the box network.h 134) the low word of `length` is written at the position, which advances by
/// 2; then `length` bytes (sign-extended from the word) of `data` are copied at the position, which advances by them.
/// Defects of the original: the bytes are not checked against the capacity, and after the box (no room for the length)
/// they are still copied, at the position that did not advance.
/// Original: `NetworkBuffer::put_sized_block` (0x569540, network.h, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_sized_block(buf: usize, data: usize, length: u32) {
    covered!("tel46_1 put a sized block");
    network_buffer_put_short(buf, length);
    let n = length as i16 as i32;
    core::ptr::copy(data as *const u8, network_buffer_cursor(buf) as *mut u8, n as usize); // tel46_1: the block's bytes
    network_buffer_advance(buf, n as u32); // tel46_1: past the block
}

/// The time `animate_widgets` reads: `utils_get_time_ms`. In the simulation (`simulation.rs`, `fixed_clock`) each
/// calling routine of the exe has a clock of its own, chosen by the return address; a query from the DLL would advance
/// the clock of the last caller (0x81b890) and shift the match day waits (the 318th's stall), so the copy reads this
/// routine's clock without advancing it (`clock_peek`): in the simulation the copy's widgets do not animate.
unsafe fn animation_clock() -> u32 {
    crate::simulation::clock_peek(0x569610).map(|v| v as u32).unwrap_or_else(|| exe::utils_get_time_ms())
}

/// Whether the animation timer at `timer` is due: unless `now` is in [timer, timer + period) (unsigned, the sum
/// wrapping), the timer restarts at `now` and it is due.
unsafe fn animation_due(timer: usize, now: u32, period: u32) -> bool {
    let t = read_u32(timer);
    if now >= t && now < t.wrapping_add(period) {
        return false;
    }
    write_u32(timer, now); // tel46_1: the timer restarts
    true
}

/// The marquee text turned: "head\x01rest" gives "rest\x01head" (with no marker, "\x01" before the whole text).
pub(crate) fn marquee_turned(text: &[u8]) -> Vec<u8> {
    let cut = text.iter().position(|&b| b == MARQUEE_MARK).unwrap_or(text.len());
    let rest = if cut < text.len() { &text[cut + 1..] } else { &[][..] };
    let mut out = rest.to_vec();
    out.push(MARQUEE_MARK);
    out.extend_from_slice(&text[..cut]);
    out
}

/// Turns a marquee widget's text: the marker (the first byte 1 before the terminator) becomes a terminator, the rest
/// and the head are joined around a new marker (`crt_sprintf("%s%c%s", rest, 1, head)`) and, when that differs from the
/// head, the widget takes it (`Guio::set_text_and_refresh`, no refresh) and its rectangle is redrawn
/// (`Display::draw_screen` on `d`).
unsafe fn turn_marquee(d: usize, item: usize) {
    let text = item + display_item::TEXT;
    let mut k = 0;
    while !matches!(read_u8(text + k), 0 | MARQUEE_MARK) {
        k += 1;
    }
    if read_u8(text + k) == MARQUEE_MARK {
        write_u8(text + k, 0);
        k += 1;
    }
    // the original's buffer is 0x104 bytes on its stack; a longer text overflows it there
    let mut turned = [0u8; 0x210];
    let format: Tel28FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    format(turned.as_mut_ptr() as usize, FORMAT_MARQUEE, text + k, MARQUEE_MARK as u32, text);
    if c_string_bytes(text) != c_string_bytes(turned.as_ptr() as usize) {
        exe::guio_set_text_and_refresh(item, turned.as_ptr(), 0);
        exe::display_draw_screen(d, read_u32(item + display_item::X1), read_u32(item + display_item::Y1), read_u32(item + display_item::X2),
            read_u32(item + display_item::Y2));
    }
}

/// Swaps a blinking widget's colour and text colour and, when the current slot's Display is laid out
/// (`display_list::LAID_OUT`), redraws the widget's rectangle on `d`.
unsafe fn swap_blink_colours(d: usize, item: usize) {
    let colour = read_u16(item + guio_fields::COLOUR);
    write_u16(item + guio_fields::COLOUR, read_u16(item + guio_fields::TEXT_COLOUR)); // tel46_1: the colours swap
    write_u16(item + guio_fields::TEXT_COLOUR, colour);
    let current = read_usize(screen_record(G_SCREEN_MANAGER) + screen_manager::DISPLAY);
    if read_u32(current + display_list::LAID_OUT) != 0 {
        exe::display_draw_screen(d, read_u32(item + display_item::X1), read_u32(item + display_item::Y1), read_u32(item + display_item::X2),
            read_u32(item + display_item::Y2));
    }
}

/// Rule: every second the marquee widgets turn their text, and every 100 ms the blinking widgets swap their colours.
///
/// The time (`animation_clock`) restarts the text timer (`ANIMATE_TEXT_TIME`, 1000 ms) and the colour timer
/// (`ANIMATE_COLOUR_TIME`, 100 ms) when due (`animation_due`); with neither due nothing happens. Otherwise each widget
/// of the Display (`display_list::ITEM_COUNT`, read anew each turn): with `ANIMATED_TEXT` and the text timer due its
/// text turns (`turn_marquee`); with `ANIMATED_COLOURS` and the colour timer due its colours swap
/// (`swap_blink_colours`).
/// Original: `Display::animate_widgets` (0x569610, network.h, thiscall).
pub(crate) unsafe extern "thiscall" fn display_animate_widgets(d: usize) {
    let now = animation_clock();
    let text_due = animation_due(ANIMATE_TEXT_TIME, now, 1000);
    let colours_due = animation_due(ANIMATE_COLOUR_TIME, now, 100);
    if !text_due && !colours_due {
        return;
    }
    covered!("tel46_1 animate the widgets");
    let mut k: i32 = 0;
    while k < read_i16(d + display_list::ITEM_COUNT) as i32 {
        let item = display_item_at(d, k as i16);
        let flags = read_u32(item + display_item::FLAGS);
        if flags & ANIMATED_TEXT != 0 && text_due {
            covered!("tel46_1 a marquee turns");
            turn_marquee(d, item);
        }
        if flags & ANIMATED_COLOURS != 0 && colours_due {
            covered!("tel46_1 a widget blinks");
            swap_blink_colours(d, item);
        }
        k += 1;
    }
}

/// Comparer for `network_buffer_put_long`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_long(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
    compare_network_buffer(buf, 4, (network_buffer_cursor(buf), 4), "put_long", || o(buf, value), || network_buffer_put_long(buf, value));
}

/// Comparer for `network_buffer_put_sized_block`: side by side over the buffer header, the length and the bytes; a
/// negative length or a block past the capacity only the original (it writes past the buffer).
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_sized_block(buf: usize, data: usize, length: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%);
    let n = length as i16 as i32;
    if n < 0 {
        // simplification: a negative length copies past the buffer
        return o(buf, data, length);
    }
    let size = 2 + n as u32;
    compare_network_buffer(buf, size, (network_buffer_cursor(buf), size as usize), "put_sized_block", || o(buf, data, length),
        || network_buffer_put_sized_block(buf, data, length));
}

/// Comparer for `display_animate_widgets`: only the original (it draws); the plan afterwards: a timer that changed was
/// due, and then each marquee widget holds its text turned (`marquee_turned` of the text before) and each blinking
/// widget its colours swapped; a timer not due leaves its widgets as they were.
pub(crate) unsafe extern "thiscall" fn compare_display_animate_widgets(d: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let (text_time, colour_time) = (read_u32(ANIMATE_TEXT_TIME), read_u32(ANIMATE_COLOUR_TIME));
    let n = read_i16(d + display_list::ITEM_COUNT).max(0);
    let before: Vec<(u32, Vec<u8>, u16, u16)> = (0..n)
        .map(|k| {
            let item = display_item_at(d, k);
            (read_u32(item + display_item::FLAGS), c_string_bytes(item + display_item::TEXT), read_u16(item + guio_fields::COLOUR),
                read_u16(item + guio_fields::TEXT_COLOUR))
        })
        .collect();
    o(d);
    let (text_due, colours_due) = (read_u32(ANIMATE_TEXT_TIME) != text_time, read_u32(ANIMATE_COLOUR_TIME) != colour_time);
    let mut wrong = Vec::new();
    for (k, (flags, text, colour, text_colour)) in before.iter().enumerate() {
        let item = display_item_at(d, k as i16);
        let want_text = if flags & ANIMATED_TEXT != 0 && text_due { marquee_turned(text) } else { text.clone() };
        let want_colours = if flags & ANIMATED_COLOURS != 0 && colours_due { (*text_colour, *colour) } else { (*colour, *text_colour) };
        let got_colours = (read_u16(item + guio_fields::COLOUR), read_u16(item + guio_fields::TEXT_COLOUR));
        if c_string_bytes(item + display_item::TEXT) != want_text || got_colours != want_colours {
            wrong.push(k);
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("animate_widgets (text due {}, colours due {}): widgets {:?} differ from the plan", text_due,
        colours_due, &wrong[..wrong.len().min(8)]));
}
