// ---------- batch %%LEVA%%: the Display's life: the two command buffers, building and destroying a Display, resetting it, adding an area and the network status packet (display.cpp) ----------

/// More fields of the Display object (display.cpp) used by its constructor, destructor, reset and add_area.
#[allow(dead_code)]
pub(crate) mod display_life {
    /// i16: areas in the table (at most 249 are added).
    pub(crate) const AREA_COUNT: usize = 0x12e99e;
    /// u8 and i16 cleared by the constructor and the reset [inference: the pressed key state].
    pub(crate) const KEY_STATE: usize = 0x12f4fe;
    pub(crate) const KEY_ITEM: usize = 0x12f4ff;
    /// i32 x2: the rest of the drag outline (`display_list::DRAG_RECT` is 4 dwords from 0x12f511).
    pub(crate) const DRAG_RECT_END: usize = 0x12f51d;
    /// Pointer to the media list: {pointer to the entries, count} [inference]; the destructor frees each entry.
    pub(crate) const MEDIA_LIST: usize = 0x12f529;
    /// i16 x2 cleared by the constructor.
    pub(crate) const WORD_539: usize = 0x12f539;
    pub(crate) const WORD_53B: usize = 0x12f53b;
    /// u32: the constructor's argument [inference: the channel].
    pub(crate) const CHANNEL: usize = 0x12f541;
    /// u32: an area did not fit (`Display::add_area`).
    pub(crate) const AREA_OVERFLOW: usize = 0x12f54d;
    /// u32 cleared by the reset.
    pub(crate) const DWORD_551: usize = 0x12f551;
}

/// The shared image of the Displays and how many Displays hold it [inference]: freed by the destructor of the last.
const DISPLAY_SHARED_IMAGE: usize = 0xae2a24;
const DISPLAY_SHARED_IMAGE_USERS: usize = 0xae291c;

/// Builds a NetworkBuffer (`NetworkBuffer::NetworkBuffer` 0x77b630, thiscall, capacity) and registers its freeing
/// function with `atexit`.
unsafe fn display_build_buffer(buf: usize, capacity: u32, free: usize) {
    let build: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x77b630usize);
    build(buf, capacity);
    exe::crt_atexit(free);
}

/// Frees a NetworkBuffer (`NetworkBuffer::~NetworkBuffer` 0x77b6e0, thiscall).
unsafe fn display_free_buffer(buf: usize) {
    let free: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x77b6e0usize);
    free(buf)
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

/// The fields the constructor writes, with their size (for the comparer).
const DISPLAY_BUILT_FIELDS: [(usize, usize); 19] = [
    (display_life::CHANNEL, 4), (display_list::BACKGROUND_IMAGE, 4), (display_list::DRAG_IMAGE, 4), (display_list::RESEND_PENDING, 4),
    (display_list::KEEPALIVE_SENT, 4), (display_list::FROZEN, 4), (display_list::FADE_IN, 4), (display_list::LAID_OUT, 4),
    (display_list::INPUT_MODE, 4), (display_list::BACKGROUND_NAME, 1), (display_life::MEDIA_LIST, 4), (display_life::AREA_COUNT, 2),
    (display_list::ITEM_COUNT, 2), (display_life::WORD_539, 2), (display_life::WORD_53B, 2), (display_life::KEY_ITEM, 2),
    (display_life::KEY_STATE, 1), (display_list::HOTKEY_ITEM, 2), (display_list::CLICK_ITEM, 2)];

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

/// Frees an image field of the Display when set (`gui_free_image`) and clears it.
unsafe fn display_free_image_field(field: usize) {
    let image = read_usize(field);
    if image != 0 {
        exe::gui_free_image(image);
        write_usize(field, 0);
    }
}

/// Releases every area (`Area::release` 0x4031a0) and frees every widget's images (`Guio::free_images` 0x5eef00).
unsafe fn display_release_all(d: usize) {
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
    let destroy: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    destroy(d + display_list::ITEMS, display_list::ITEM_SIZE as u32, 0x4b0, 0x55f1b0);
    destroy(d + display_list::AREAS, display_list::AREA_SIZE as u32, 0xfa, 0x55f1a0);
}

/// The Display fields `Display::reset` writes, with their size (the area and widget tables and the buffers apart).
fn display_reset_fields(d: usize) -> Vec<(usize, usize)> {
    [
        (display_life::KEY_ITEM, 2), (display_life::KEY_STATE, 1), (display_list::INPUT_MODE, 4), (display_list::AREA_ORDER_COUNT, 2),
        (display_list::ITEM_ORDER_COUNT, 2), (display_list::ITEM_COUNT, 2), (display_life::AREA_COUNT, 2), (display_list::OPEN_POPUP_AREA, 2),
        (display_list::SCROLL_AREA, 2), (display_list::DRAG_ITEM, 2), (display_list::LAID_OUT, 4), (display_list::DRAG_MOUSE_X, 4),
        (display_list::DRAG_MOUSE_Y, 4), (display_list::DRAG_RECT, 0x10), (display_life::DRAG_RECT_END, 4), (display_list::OVERFLOW, 4),
        (display_life::AREA_OVERFLOW, 4), (display_list::NEXT_TEXT_FOCUS, 2), (display_life::DWORD_551, 4), (display_list::KEEPALIVE_SENT, 4),
        (display_list::SCREEN_CLEARED, 4), (display_list::DRAG_IMAGE, 4)].iter().map(|&(f, n)| (d + f, n)).collect()
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

/// Puts a word or a byte into the display packet buffer inline (as the original inlines it): with room for `size`
/// (else the box network.h at `line`) the value is written at the position, which advances.
unsafe fn display_packet_put(value: u32, size: u32, line: u32) {
    if network_buffer_room(DISPLAY_PACKET_BUFFER, size, line) {
        let at = network_buffer_cursor(DISPLAY_PACKET_BUFFER);
        if size == 2 {
            write_u16(at, value as u16);
        } else {
            write_u8(at, value as u8);
        }
        network_buffer_advance(DISPLAY_PACKET_BUFFER, size);
    }
}

/// The bytes `Display::add_area` puts in the display packet buffer.
pub(crate) fn display_add_area_packet_size(columns: u32, column_weights: usize, rows: u32, row_weights: usize) -> u32 {
    let mut n = 8 + 1 + 1 + 1 + 1 + 4 + 2 + 2;
    if column_weights != 0 {
        n += 2 + (columns & 0xff);
    }
    if row_weights != 0 {
        n += 2 + (rows & 0xff);
    }
    n
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
        let block: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x569540usize);
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
    let init: Init = core::mem::transmute(0x402ac0usize);
    let index = (read_i16(d + display_life::AREA_COUNT) as u16 as u32).wrapping_sub(1);
    init(area, d + display_list::AREAS, d + display_list::ITEMS, index, x1, y1, x2, y2, columns, column_weights, rows, row_weights, marks, colour, owner);
    display_method1(0x5603e0, d, read_i16(d + display_life::AREA_COUNT).wrapping_sub(1) as u16 as u32);
    read_i16(d + display_life::AREA_COUNT).wrapping_sub(1)
}

/// The "Network Error" box of a display.cpp line: the version line in 0xae24d0, the title and the text through
/// `langlib_sprintf` (0xae28bc, 0xae26c4 with the format 0x9c37b0) and the message box.
unsafe fn display_network_error_box(line: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(DISPLAY_CPP, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    // like tel26_error_box: the three arguments of the version call are the rest of the format
    format(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line);
    text(0xae28bc, 0x9c3814);
    text(0xae26c4, 0x9c37b0, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
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
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x77ac30usize);
    if send(session, read_usize(NET_SEND_BUFFER + network_buffer::DATA), read_u32(NET_SEND_BUFFER + network_buffer::POS), player) == 0 {
        display_network_error_box(0xc52);
    }
}

/// Comparer for `init_net_send_buffer`.
pub(crate) unsafe extern "C" fn compare_init_net_send_buffer() {
    // simplification: the buffer is built once (before WinMain); proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `free_net_send_buffer`.
pub(crate) unsafe extern "C" fn compare_free_net_send_buffer() {
    // simplification: the buffer is freed once (at exit)
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// Comparer for `init_display_packet_buffer`.
pub(crate) unsafe extern "C" fn compare_init_display_packet_buffer() {
    // simplification: the buffer is built once (before WinMain); proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

/// Comparer for `free_display_packet_buffer`.
pub(crate) unsafe extern "C" fn compare_free_display_packet_buffer() {
    // simplification: the buffer is freed once (at exit)
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer for `display_build`: the constructor's fields and the return side by side.
pub(crate) unsafe extern "thiscall" fn compare_display_build(d: usize, channel: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%);
    let reg: Vec<(usize, usize)> = DISPLAY_BUILT_FIELDS.iter().map(|&(f, n)| (d + f, n)).collect();
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(d, channel), || display_build(d, channel));
    SCOREBOARD.record(same && r == n, || format!("Display::Display({}): {:?}", channel, first_differences(&mo, &mr)));
    r
}

/// Comparer for `display_destroy`.
pub(crate) unsafe extern "thiscall" fn compare_display_destroy(d: usize) {
    // simplification: the frees and the table destructors do not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(d)
}

/// Comparer for `display_reset`: side by side (the fields and both buffers' headers and the first packet byte) when
/// nothing is released or freed and the packet buffer has room; otherwise only the original and the same fields are
/// checked afterwards against the plan.
pub(crate) unsafe extern "thiscall" fn compare_display_reset(d: usize, release: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I6%%);
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

/// Comparer for `display_add_area`: the packet bytes, the counts, the overflow flag, the new area (and the 0xd9 bytes
/// after it that Area::init may write) and the area order side by side; without room for the whole packet or with
/// the area order full only the original (the error boxes).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_display_add_area(d: usize, x1: u32, y1: u32, x2: u32, y2: u32, columns: u32, column_weights: usize,
    rows: u32, row_weights: usize, marks: u32, colour: u32, owner: u32) -> i16 {
    type AddArea = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, usize, u32, u32, u32) -> i16;
    let o = original_fn::<AddArea>(%%I7%%);
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

/// Comparer for `display_send_status_packet`.
pub(crate) unsafe extern "stdcall" fn compare_display_send_status_packet(session: usize, player: u32, status: u32) {
    // simplification: the packet is sent over the network once; proven in Rust mode
    original_fn::<unsafe extern "stdcall" fn(usize, u32, u32)>(%%I8%%)(session, player, status)
}
