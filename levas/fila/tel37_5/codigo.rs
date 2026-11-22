// ---------- batch %%LEVA%%: the display list reader: applying the screen manager's command packets to a Display (display.cpp) ----------

/// The Display field `Display::process_command_packet` writes that has no name elsewhere.
#[allow(dead_code)]
pub(crate) mod display_packet {
    /// i32: the pixel format the colours are converted to (the new screen packet's byte, sign-extended), passed to
    /// `Area::convert_colour_to_pixel_format` and `Guio::convert_colours_pixel_format` [inference: nonzero = 565].
    pub(crate) const COLOUR_FORMAT: usize = 0x12f559;
}

/// The print list of a Display (`display_life::MEDIA_LIST` points to it; 10 bytes, `calloc`ed by command 23): the
/// line pointers, the line count and the characters per line given to `print_text_lines`.
#[allow(dead_code)]
mod print_list {
    pub(crate) const LINES: usize = 0x0;
    pub(crate) const COUNT: usize = 0x4;
    pub(crate) const CHARS_PER_LINE: usize = 0x8;
}

/// The channel (player) of the screen being applied, a word (`display_send_keepalive` reads it).
const SCREEN_CHANNEL: usize = 0xae2914;

/// The `network.h` lines of the NetworkBuffer getters (the error box of a read without room).
const GET_CHAR: u32 = 0x8f;
const GET_SHORT: u32 = 0x90;
const GET_LONG: u32 = 0x92;
const GET_INT: u32 = 0x93;
const GET_UCHAR: u32 = 0x94;
const GET_USHORT: u32 = 0x95;
const GET_ULONG: u32 = 0x97;

/// The local variables of `Display::process_command_packet`, at the original's stack offsets (from the stack pointer
/// after the four saved registers; the first 0x10 bytes are those registers and stay unused). The reads write them
/// with the original's widths (a word read writes 2 bytes of a dword local, a byte read 1), the locals keep their
/// values between the items of a packet, and what the original leaves uninitialised starts at 0 here.
struct CommandFrame([u8; 0x2b08]);

/// The frame slots by the role they have in the item commands (2, 6 and 8: the `Guio` fields given to
/// `Display::add_item` / `Guio::init`) and in the area command (1).
mod frame {
    pub(crate) const FONT: usize = 0x15; // u8: the item font (Guio +0x76)
    pub(crate) const FLAG_BYTE: usize = 0x16; // u8: command 0 the colour format, command 1 whether weights follow, command 10 looping
    pub(crate) const BORDER: usize = 0x17; // u8: the item's fifth field (Guio +0x20) [inference]
    pub(crate) const OTHER_ITEM: usize = 0x18; // i16: command 2 the item copied, command 6 the drop target, command 8 the item
    pub(crate) const LENGTH: usize = 0x1c; // i16: a block length (commands 0, 1); u32: command 2 the field mask
    pub(crate) const COMMAND: usize = 0x20; // u8: the command; i16: command 1 the area owner, command 6 the item
    pub(crate) const COLOUR: usize = 0x24; // u16: the item colour (Guio +0x72); command 1 the area colour (dword)
    pub(crate) const X1: usize = 0x28;
    pub(crate) const Y2: usize = 0x2c;
    pub(crate) const Y1: usize = 0x30;
    pub(crate) const STYLE: usize = 0x34; // u32: the item style (Guio +0x38); command 1 the area marks
    pub(crate) const X2: usize = 0x38;
    pub(crate) const SLOT: usize = 0x3c; // u8: a sound slot
    pub(crate) const FLAGS: usize = 0x40; // u32: the item flags (Guio +0xc)
    pub(crate) const ROW_LENGTH: usize = 0x46; // i16: a second block length
    pub(crate) const TARGET: usize = 0x48; // u32: command 2 Guio +0x58, command 6 the text entry's caller buffer, commands 10 and 13 a value
    pub(crate) const FIELD_3C: usize = 0x4c; // u32: Guio +0x3c
    pub(crate) const AREA: usize = 0x50; // i16: the item's area; commands 15 and 16 an area
    pub(crate) const VALUE: usize = 0x54; // u32: the item value (Guio +0x48)
    pub(crate) const ID: usize = 0x58; // u32: the item id (Guio +8)
    pub(crate) const TEXT_COLOUR: usize = 0x5c; // u16: Guio +0x78
    pub(crate) const HIGHLIGHT_COLOUR: usize = 0x60; // u16: Guio +0x74
    pub(crate) const ROW: usize = 0x64; // i16: the item row (Guio +0x24)
    pub(crate) const SUBCOMMAND: usize = 0x6b; // u8: command 8
    pub(crate) const MAX_LENGTH: usize = 0x6c; // i16: the text entry length limit (Guio +0x40)
    pub(crate) const COLUMNS: usize = 0x70; // u8: command 1 the column count
    pub(crate) const STEP: usize = 0x78; // u32: command 7 the progress step; commands 15 and 16 a row
    pub(crate) const ROWS: usize = 0x7c; // u8: command 1 the row count
    pub(crate) const TOTAL: usize = 0xa0; // u32: command 7 the progress total
    pub(crate) const PAN: usize = 0xa8; // u8: command 10 the pan
    pub(crate) const DROP_STALE: usize = 0xb0; // u32: what command 6 takes for the first drop coordinate without room
    pub(crate) const DROP_RECT: usize = 0xb4; // u32 x3: command 6 the first three drop coordinates
    pub(crate) const COLUMN_WEIGHTS: usize = 0x2c4; // command 1
    pub(crate) const ROW_WEIGHTS: usize = 0x2e4; // command 1
    pub(crate) const TEXT: usize = 0x404; // the item text, a file name, a line
    pub(crate) const PICTURE: usize = 0x1a04; // command 0 the background picture
    // the outputs of Guio::get_fields (command 2's snapshot) that are not item slots
    pub(crate) const SNAP_X1: usize = 0x84;
    pub(crate) const SNAP_Y1: usize = 0x8c;
    pub(crate) const SNAP_X2: usize = 0xac;
    pub(crate) const SNAP_Y2: usize = 0x94;
    pub(crate) const SNAP_BORDER: usize = 0xa4;
    pub(crate) const SNAP_ROW: usize = 0x9c;
    pub(crate) const SNAP_FONT: usize = 0x82;
    pub(crate) const SNAP_TEXT: usize = 0x90;
    pub(crate) const SNAP_MAX_LENGTH: usize = 0x88;
    pub(crate) const SNAP_LAST: usize = 0xb0;
}

impl CommandFrame {
    fn new() -> CommandFrame {
        CommandFrame([0; 0x2b08])
    }
    fn addr(&mut self, off: usize) -> usize {
        self.0.as_mut_ptr() as usize + off
    }
    fn u8(&self, off: usize) -> u8 {
        self.0[off]
    }
    fn u16(&self, off: usize) -> u16 {
        u16::from_le_bytes([self.0[off], self.0[off + 1]])
    }
    fn i16(&self, off: usize) -> i16 {
        self.u16(off) as i16
    }
    /// The word sign-extended to a dword (the original's `movsx`).
    fn sx16(&self, off: usize) -> u32 {
        self.i16(off) as i32 as u32
    }
    /// The byte sign-extended to a dword.
    fn sx8(&self, off: usize) -> u32 {
        self.0[off] as i8 as i32 as u32
    }
    fn u32(&self, off: usize) -> u32 {
        u32::from_le_bytes([self.0[off], self.0[off + 1], self.0[off + 2], self.0[off + 3]])
    }
    fn set_u8(&mut self, off: usize, v: u8) {
        self.0[off] = v;
    }
    fn set_u16(&mut self, off: usize, v: u16) {
        self.0[off..off + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn set_u32(&mut self, off: usize, v: u32) {
        self.0[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }
    /// Copies a C string into the frame at `off` (the original's inline strcpy).
    unsafe fn set_text(&mut self, off: usize, text: usize) {
        let n = c_string_len(text) + 1;
        let n = n.min(self.0.len() - off); // ponytail: the original would run over its frame; the copy stops at its end
        core::ptr::copy(text as *const u8, self.0.as_mut_ptr().add(off), n);
    }
    /// The low word of `low` sign-extended from a byte over the high word of `high` (the original's `movsx r16, byte`
    /// into a register that held another dword: Guio::init reads only the low word).
    fn byte_over(&self, high: u32, off: usize) -> u32 {
        (high & 0xffff_0000) | (self.sx8(off) & 0xffff)
    }
}

/// A command packet reader: the NetworkBuffer reads of `Display::process_command_packet`, inlined or called
/// (`NetworkBuffer::get_*`, tel37_1: the same room check, box line and width). `quiet` (the comparer's plan, on a copy of
/// the buffer header) shows no box and only notes the failure.
struct PacketReader {
    buf: usize,
    quiet: bool,
    failed: bool,
}

impl PacketReader {
    fn new(buf: usize, quiet: bool) -> PacketReader {
        PacketReader { buf, quiet, failed: false }
    }
    /// Room for `n` bytes (else the box network.h at `line`): the address of the bytes and the position before.
    unsafe fn take(&mut self, n: u32, line: u32) -> Option<(usize, u32)> {
        let pos = read_u32(self.buf + network_buffer::POS);
        if pos.wrapping_add(n) > read_u32(self.buf + network_buffer::CAPACITY) {
            self.failed = true;
            if !self.quiet {
                tel26_error_box(NETWORK_H, line);
            }
            return None;
        }
        let at = network_buffer_cursor(self.buf);
        network_buffer_advance(self.buf, n);
        Some((at, pos))
    }
    unsafe fn byte_to(&mut self, out: usize, line: u32) {
        if let Some((at, _)) = self.take(1, line) {
            write_u8(out, read_u8(at));
        }
    }
    unsafe fn word_to(&mut self, out: usize, line: u32) {
        if let Some((at, _)) = self.take(2, line) {
            write_u16(out, read_u16(at));
        }
    }
    /// A word into a dword local: the high word is the position's (the original reads it into `ax` with the position
    /// in `eax`).
    unsafe fn word_dword_to(&mut self, out: usize, line: u32) {
        if let Some((at, pos)) = self.take(2, line) {
            write_u32(out, (pos & 0xffff_0000) | read_u16(at) as u32);
        }
    }
    unsafe fn dword_to(&mut self, out: usize, line: u32) {
        if let Some((at, _)) = self.take(4, line) {
            write_u32(out, read_u32(at));
        }
    }
    /// A dword, or `stale` without room (a register the original keeps).
    unsafe fn dword_or(&mut self, stale: u32, line: u32) -> u32 {
        match self.take(4, line) {
            Some((at, _)) => read_u32(at),
            None => stale,
        }
    }
    /// The position advances by 2 when there is room (a word nobody reads).
    unsafe fn skip_word(&mut self) {
        self.take(2, GET_SHORT);
    }
    /// The bytes at the position copied to `out` and the position advanced by `n` (sign-extended; no room check).
    unsafe fn copy_to(&mut self, out: usize, n: i16, room: usize) {
        let len = (n.max(0) as usize).min(room); // ponytail: the original copies a negative length as ~4 GB and crashes
        core::ptr::copy(network_buffer_cursor(self.buf) as *const u8, out as *mut u8, len);
        network_buffer_advance(self.buf, n as i32 as u32);
    }
    /// An inline sized block: the length into the frame slot `len` (`get_short`: without room the slot keeps its old
    /// value, which is the length copied), then the bytes into the frame at `out`.
    unsafe fn block_to_frame(&mut self, fr: &mut CommandFrame, len: usize, out: usize) {
        self.word_to(fr.addr(len), GET_SHORT);
        let room = fr.0.len() - out;
        self.copy_to(fr.addr(out), fr.i16(len), room);
    }
    /// `NetworkBuffer::get_sized_block` (tel37_1: without room for the length nothing is copied).
    unsafe fn sized_block(&mut self, fr: &mut CommandFrame, out: usize) {
        let mut n = 0i16;
        if let Some((at, _)) = self.take(2, GET_SHORT) {
            n = read_i16(at);
        }
        let room = fr.0.len() - out;
        self.copy_to(fr.addr(out), n, room);
    }
}

/// The fields of the new screen packet (command 0) that go straight to the Display.
#[derive(Clone, Copy)]
struct NewScreenFields {
    area_count: u16,
    item_count: u16,
    focus_flags: u8,
    focus_item: u16,
}

/// Reads the new screen packet: the background picture (an inline sized block into `frame::PICTURE`), the colour format
/// byte, the area and item counts, the focus flags and the focus item (each left as it was without room).
unsafe fn read_new_screen(r: &mut PacketReader, fr: &mut CommandFrame, f: &mut NewScreenFields) {
    r.block_to_frame(fr, frame::LENGTH, frame::PICTURE);
    r.byte_to(fr.addr(frame::FLAG_BYTE), GET_CHAR);
    r.word_to(&mut f.area_count as *mut u16 as usize, GET_SHORT);
    r.word_to(&mut f.item_count as *mut u16 as usize, GET_SHORT);
    r.byte_to(&mut f.focus_flags as *mut u8 as usize, GET_CHAR);
    r.word_to(&mut f.focus_item as *mut u16 as usize, GET_SHORT);
}

/// The Display's new screen fields as they are.
unsafe fn new_screen_fields(d: usize) -> NewScreenFields {
    NewScreenFields {
        area_count: read_u16(d + display_life::WORD_53B),
        item_count: read_u16(d + display_life::WORD_539),
        focus_flags: read_u8(d + display::FOCUS_FLAGS),
        focus_item: read_u16(d + display::FOCUS_ITEM),
    }
}

/// The answer to a screen section packet (commands 0, 1, 2): a network player that asked for a resend
/// (`display_list::RESEND_PENDING`) gets the status packet `status` (`display_send_status_packet` 0x568290); otherwise the
/// network send buffer is emptied and gets 5 and `reply` (`NetworkBuffer::put_char`, the box network.h 133 without room).
unsafe fn answer_screen_section(d: usize, session: usize, player: u32, status: u32, reply: u32) {
    if read_u32(d + display_list::RESEND_PENDING) != 0 {
        let send: unsafe extern "stdcall" fn(usize, u32, u32) = core::mem::transmute(0x568290usize);
        send(session, player, status);
        return;
    }
    write_u32(NET_SEND_BUFFER + network_buffer::POS, 0);
    network_buffer_put_char(NET_SEND_BUFFER, 5);
    network_buffer_put_char(NET_SEND_BUFFER, reply);
}

/// `display_localise_media_path(path, is_picture)` (0x5680f0, cdecl).
unsafe fn localise_media_path(path: usize, is_picture: u32) {
    let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x5680f0usize);
    f(path, is_picture)
}

/// `Display::draw_screen(-1, -1, -1, -1)` (0x55f300): the whole screen.
unsafe fn draw_whole_screen(d: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x55f300usize);
    f(d, u32::MAX, u32::MAX, u32::MAX, u32::MAX);
}

/// `Guio::convert_colours_pixel_format(item, format, which)` (0x5eef80): 1 the colour, 2 the highlight, 4 the text colour.
unsafe fn convert_item_colours(d: usize, item: usize, which: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x5eef80usize);
    f(item, read_u32(d + display_packet::COLOUR_FORMAT), which)
}

/// `Guio::init` (0x5ee150) on item `item` of the Display with its index (the whole dword) and the 18 fields.
unsafe fn init_item(d: usize, item: usize, index: u32, f: DisplayItemFields) {
    type GuioInit = unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let init: GuioInit = core::mem::transmute(0x5ee150usize);
    init(item, d + display_list::AREAS, d + display_list::ITEMS, index,
        f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12], f[13], f[14], f[15], f[16], f[17]);
}

/// The 18 item fields from the frame, in `Display::add_item`'s order: the font word over `font_high`, the area `area`.
unsafe fn item_fields(fr: &mut CommandFrame, font_high: u32, area: u32) -> DisplayItemFields {
    use frame as s;
    [fr.u32(s::FLAGS), fr.sx16(s::X1), fr.sx16(s::Y1), fr.sx16(s::X2), fr.sx16(s::Y2), fr.sx8(s::BORDER), fr.sx16(s::ROW), fr.u32(s::STYLE),
     fr.u32(s::COLOUR), fr.u32(s::HIGHLIGHT_COLOUR), fr.u32(s::FIELD_3C), fr.byte_over(font_high, s::FONT), fr.u32(s::TEXT_COLOUR),
     fr.addr(s::TEXT) as u32, fr.sx16(s::MAX_LENGTH), fr.u32(s::ID), fr.u32(s::VALUE), area]
}

/// Whether the Display's areas and items are the ones the screen packet announced (commands 8 and 9): with an announced
/// area count (`display_life::WORD_53B` ≠ 0) the area count and the item count must equal the announced ones.
unsafe fn screen_complete(d: usize) -> bool {
    let areas = read_u16(d + display_life::WORD_53B);
    areas == 0 || (read_u16(d + display_life::AREA_COUNT) == areas && read_u16(d + display_list::ITEM_COUNT) == read_u16(d + display_life::WORD_539))
}

/// Command 0, a new screen: the Display reset (releasing), the channel, the packet's picture, colour format, counts and
/// focus, the picture localised and made the screen manager's background, and the answer (status 2, reply 0).
///
/// `Display::reset(1)` (0x55f7c0) first; [0xae2914] = the player (word). The new screen fields are read
/// (`read_new_screen`) into `display_life::WORD_53B` (areas), `WORD_539` (items), `display::FOCUS_FLAGS` and
/// `FOCUS_ITEM`; `display_packet::COLOUR_FORMAT` = the colour byte sign-extended; `display_localise_media_path(picture, 1)`
/// and `ScreenManager::set_background_picture(picture)` (0x7feeb0); then `answer_screen_section` with status 2 and 0.
unsafe fn command_new_screen(d: usize, session: usize, player: u32, r: &mut PacketReader, fr: &mut CommandFrame) -> u16 {
    covered!("tel37_5 new screen");
    display_method1(0x55f7c0, d, 1);
    write_u16(SCREEN_CHANNEL, player as u16);
    let mut f = new_screen_fields(d);
    read_new_screen(r, fr, &mut f);
    write_u16(d + display_life::WORD_53B, f.area_count);
    write_u16(d + display_life::WORD_539, f.item_count);
    write_u8(d + display::FOCUS_FLAGS, f.focus_flags);
    write_u16(d + display::FOCUS_ITEM, f.focus_item);
    write_u32(d + display_packet::COLOUR_FORMAT, fr.sx8(frame::FLAG_BYTE));
    localise_media_path(fr.addr(frame::PICTURE), 1);
    display_method1(0x7feeb0, G_SCREEN_MANAGER, fr.addr(frame::PICTURE) as u32);
    answer_screen_section(d, session, player, 2, 0);
    0xffff
}

/// Reads one area of the area packet into the frame: x1, y1, x2, y2 (words into dwords), the column count
/// (`get_uchar`), whether column weights follow (then a block, else as many 1s as columns), the row count and its
/// weights the same way, the marks (dword), the colour (unsigned word into a dword) and the owner (word).
unsafe fn read_area(r: &mut PacketReader, fr: &mut CommandFrame) {
    use frame as s;
    for slot in [s::X1, s::Y1, s::X2, s::Y2] {
        r.word_dword_to(fr.addr(slot), GET_SHORT);
    }
    for (count, len, weights) in [(s::COLUMNS, s::LENGTH, s::COLUMN_WEIGHTS), (s::ROWS, s::ROW_LENGTH, s::ROW_WEIGHTS)] {
        r.byte_to(fr.addr(count), GET_UCHAR);
        r.byte_to(fr.addr(s::FLAG_BYTE), GET_CHAR);
        if fr.u8(s::FLAG_BYTE) != 0 {
            r.block_to_frame(fr, len, weights);
        } else {
            let n = fr.u8(count) as usize;
            fr.0[weights..weights + n].fill(1);
        }
    }
    r.dword_to(fr.addr(s::STYLE), GET_ULONG);
    r.word_dword_to(fr.addr(s::COLOUR), GET_USHORT);
    r.word_to(fr.addr(s::COMMAND), GET_SHORT);
}

/// Command 1, the screen's areas: with no keepalive sent, the announced areas (`display_life::WORD_53B`) are read and
/// added, then the answer (status 1, reply 2).
///
/// With `display_list::KEEPALIVE_SENT` set nothing happens. Otherwise [0xae2914] = the player and, for k from 0 while
/// k (a word) is below the announced count: the area is read (`read_area`), area k is released (`Area::release`
/// 0x4031a0), `Display::add_area` (0x55fba0, by address: the 1080 exe scales it) gets x1, y1, x2, y2 (sign-extended),
/// the column count, the column weights, the row count, the row weights, the marks, the colour and the owner (the
/// dwords as they are), and the area it returns gets `Area::convert_colour_to_pixel_format(display_packet::COLOUR_FORMAT)`
/// (0x4041d0). Then `answer_screen_section` with status 1 and 2.
unsafe fn command_add_areas(d: usize, session: usize, player: u32, r: &mut PacketReader, fr: &mut CommandFrame) -> u16 {
    use frame as s;
    if read_u32(d + display_list::KEEPALIVE_SENT) != 0 {
        return 0xffff;
    }
    covered!("tel37_5 areas");
    write_u16(SCREEN_CHANNEL, player as u16);
    let mut k: u32 = 0;
    if read_i16(d + display_life::WORD_53B) > 0 {
        loop {
            read_area(r, fr);
            display_method(0x4031a0, display_area_at(d, k as i16));
            let area = tel26_add_area(d, [fr.sx16(s::X1), fr.sx16(s::Y1), fr.sx16(s::X2), fr.sx16(s::Y2), fr.u32(s::COLUMNS), fr.addr(s::COLUMN_WEIGHTS) as u32,
                                          fr.u32(s::ROWS), fr.addr(s::ROW_WEIGHTS) as u32, fr.u32(s::STYLE), fr.u32(s::COLOUR), fr.u32(s::COMMAND)]);
            display_method1(0x4041d0, display_area_at(d, area as i16), read_u32(d + display_packet::COLOUR_FORMAT));
            k = k.wrapping_add(1);
            if k as i16 >= read_i16(d + display_life::WORD_53B) {
                break;
            }
        }
    }
    answer_screen_section(d, session, player, 1, 2);
    0xffff
}

/// Command 2's snapshot: item k of the snapshot pool's first slot gives its fields (`Guio::get_fields` 0x5ee4a0 into the
/// frame); the rectangle, the border, the row, the font and the length limit go to their slots, `frame::TARGET` = the
/// snapshot's +0x58 and the text is copied (empty for a null text).
unsafe fn read_item_snapshot(fr: &mut CommandFrame, k: u32) {
    use frame as s;
    type GetFields = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize);
    let snap = read_usize(DISPLAY_SNAPSHOT_POOL).wrapping_add((k as i16 as i32 * display_list::ITEM_SIZE as i32) as usize);
    let get: GetFields = core::mem::transmute(0x5ee4a0usize);
    get(snap, fr.addr(s::FLAGS), fr.addr(s::SNAP_X1), fr.addr(s::SNAP_Y1), fr.addr(s::SNAP_X2), fr.addr(s::SNAP_Y2), fr.addr(s::SNAP_BORDER),
        fr.addr(s::SNAP_ROW), fr.addr(s::STYLE), fr.addr(s::COLOUR), fr.addr(s::HIGHLIGHT_COLOUR), fr.addr(s::FIELD_3C), fr.addr(s::SNAP_FONT),
        fr.addr(s::TEXT_COLOUR), fr.addr(s::SNAP_TEXT), fr.addr(s::SNAP_MAX_LENGTH), fr.addr(s::ID), fr.addr(s::VALUE), fr.addr(s::AREA), fr.addr(s::SNAP_LAST));
    for (to, from) in [(s::X1, s::SNAP_X1), (s::Y1, s::SNAP_Y1), (s::X2, s::SNAP_X2), (s::Y2, s::SNAP_Y2), (s::ROW, s::SNAP_ROW), (s::MAX_LENGTH, s::SNAP_MAX_LENGTH)] {
        fr.set_u32(to, fr.u32(from));
    }
    fr.set_u8(s::BORDER, fr.u8(s::SNAP_BORDER));
    fr.set_u8(s::FONT, fr.u8(s::SNAP_FONT));
    fr.set_u32(s::TARGET, read_u32(snap + 0x58));
    let text = fr.u32(s::SNAP_TEXT) as usize;
    if text != 0 {
        fr.set_text(s::TEXT, text);
    } else {
        fr.set_u8(s::TEXT, 0);
    }
}

/// Command 2's fields of one item, by the mask: each bit present reads its field, an absent one clears it unless the
/// item comes from the snapshot (`from_snapshot`); returns whether the item copies another (bit 0x80000).
///
/// In order: 1 the flags, 2 the style, 4 the target (`get_int`), 8 / 0x10 / 0x20 / 0x40 x1, y1, x2, y2, 0x80 the
/// border, 0x100 the row, 0x200 the colour, 0x400 the highlight colour, 0x800 the +0x3c field, 0x1000 the font, 0x2000
/// the text colour (unsigned words), 0x4000 the text (an inline block; with the flag 0x400 it is a picture path,
/// localised), 0x8000 the length limit, 0x10000 the id (`get_long`), 0x20000 the value (`get_int`), 0x40000 the area.
/// The words fill only the low word of their dword slot; the clearing writes the whole slot. With 0x80000 the item
/// index to copy is read and that item's flags, style, colour, highlight colour, +0x3c, font and text colour replace
/// the frame's.
unsafe fn read_item_fields(d: usize, r: &mut PacketReader, fr: &mut CommandFrame, mask: u32, from_snapshot: bool) -> bool {
    use frame as s;
    let read = |r: &mut PacketReader, fr: &mut CommandFrame, bit: u32, slot: usize, size: u32, line: u32| {
        if mask & bit != 0 {
            match size {
                1 => r.byte_to(fr.addr(slot), line),
                2 => r.word_to(fr.addr(slot), line),
                _ => r.dword_to(fr.addr(slot), line),
            }
        } else if !from_snapshot {
            if size == 1 {
                fr.set_u8(slot, 0);
            } else {
                fr.set_u32(slot, 0);
            }
        }
    };
    read(r, fr, 0x1, s::FLAGS, 4, GET_ULONG);
    read(r, fr, 0x2, s::STYLE, 4, GET_ULONG);
    read(r, fr, 0x4, s::TARGET, 4, GET_INT);
    read(r, fr, 0x8, s::X1, 2, GET_SHORT);
    read(r, fr, 0x10, s::Y1, 2, GET_SHORT);
    read(r, fr, 0x20, s::X2, 2, GET_SHORT);
    read(r, fr, 0x40, s::Y2, 2, GET_SHORT);
    read(r, fr, 0x80, s::BORDER, 1, GET_CHAR);
    read(r, fr, 0x100, s::ROW, 2, GET_SHORT);
    read(r, fr, 0x200, s::COLOUR, 2, GET_USHORT);
    read(r, fr, 0x400, s::HIGHLIGHT_COLOUR, 2, GET_USHORT);
    read(r, fr, 0x800, s::FIELD_3C, 4, GET_ULONG);
    read(r, fr, 0x1000, s::FONT, 1, GET_CHAR);
    read(r, fr, 0x2000, s::TEXT_COLOUR, 2, GET_USHORT);
    if mask & 0x4000 != 0 {
        r.block_to_frame(fr, s::ROW_LENGTH, s::TEXT);
        if fr.u32(s::FLAGS) & 0x400 != 0 {
            localise_media_path(fr.addr(s::TEXT), 1);
        }
    } else if !from_snapshot {
        fr.set_u8(s::TEXT, 0);
    }
    read(r, fr, 0x8000, s::MAX_LENGTH, 2, GET_SHORT);
    read(r, fr, 0x10000, s::ID, 4, GET_LONG);
    read(r, fr, 0x20000, s::VALUE, 4, GET_INT);
    read(r, fr, 0x40000, s::AREA, 2, GET_SHORT);
    if mask & 0x80000 == 0 {
        return false;
    }
    r.word_to(fr.addr(s::OTHER_ITEM), GET_SHORT);
    let other = display_item_at(d, fr.i16(s::OTHER_ITEM));
    fr.set_u32(s::FLAGS, read_u32(other + display_item::FLAGS));
    fr.set_u32(s::STYLE, read_u32(other + display_item::STYLE));
    fr.set_u16(s::COLOUR, read_u16(other + 0x72));
    fr.set_u16(s::HIGHLIGHT_COLOUR, read_u16(other + 0x74));
    fr.set_u32(s::FIELD_3C, read_u32(other + 0x3c));
    fr.set_u8(s::FONT, read_u8(other + display_item::FONT));
    fr.set_u16(s::TEXT_COLOUR, read_u16(other + 0x78));
    true
}

/// Command 2, the screen's items: with no keepalive sent, the announced items (`display_life::WORD_539`) are read and
/// added, the snapshot pool's first slot takes the new item table, then the answer (status 3, reply 1).
///
/// With `display_list::KEEPALIVE_SENT` set nothing happens. Otherwise [0xae2914] = the player and, for k from 0 while
/// k (a word) is below the announced count: the field mask (dword, kept in the frame without room); with the bit
/// 0x100000 the item starts from item k of the snapshot (`read_item_snapshot`); the fields (`read_item_fields`);
/// `Display::add_item` (0x55f990, by address: the 1080 exe has its own) with the 18 fields (the font word over the length
/// limit's sign extension, the area as read); unless the item copies another, the colours read get
/// `Guio::convert_colours_pixel_format` (0x200: 1, 0x400: 2, 0x2000: 4); the new item's +0x54 = 0 and +0x58 = the
/// target. Then the snapshot pool's word +0x74040 = `display_list::ITEM_COUNT` and the item table (count × 0x18c bytes)
/// is copied to the pool; `answer_screen_section` with status 3 and 1.
unsafe fn command_add_items(d: usize, session: usize, player: u32, r: &mut PacketReader, fr: &mut CommandFrame) -> u16 {
    use frame as s;
    if read_u32(d + display_list::KEEPALIVE_SENT) != 0 {
        return 0xffff;
    }
    covered!("tel37_5 items");
    write_u16(SCREEN_CHANNEL, player as u16);
    let mut k: u32 = 0;
    if read_i16(d + display_life::WORD_539) > 0 {
        loop {
            r.dword_to(fr.addr(s::LENGTH), GET_ULONG);
            let mask = fr.u32(s::LENGTH);
            let from_snapshot = mask & 0x100000 != 0;
            if from_snapshot {
                read_item_snapshot(fr, k);
            }
            let copies = read_item_fields(d, r, fr, mask, from_snapshot);
            let (high, area) = (fr.sx16(s::MAX_LENGTH), fr.u32(s::AREA));
            let f = item_fields(fr, high, area);
            let item = display_item_at(d, tel26_add_item(d, f) as i16);
            if !copies {
                for (bit, which) in [(0x200, 1), (0x400, 2), (0x2000, 4)] {
                    if mask & bit != 0 {
                        convert_item_colours(d, item, which);
                    }
                }
            }
            write_u32(item + 0x54, 0);
            write_u32(item + 0x58, fr.u32(s::TARGET));
            k = k.wrapping_add(1);
            if k as i16 >= read_i16(d + display_life::WORD_539) {
                break;
            }
        }
    }
    let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
    write_u16(pool + 0x74040, read_u16(d + display_list::ITEM_COUNT));
    let size = (read_i16(d + display_list::ITEM_COUNT) as i32 * display_list::ITEM_SIZE as i32) as u32;
    core::ptr::copy((d + display_list::ITEMS) as *const u8, read_usize(DISPLAY_SNAPSHOT_POOL) as *mut u8, size as usize);
    answer_screen_section(d, session, player, 3, 1);
    0xffff
}

/// Reads command 6's item fields in the first form (inline: words into dwords except the item and the length limit).
unsafe fn read_built_item(r: &mut PacketReader, fr: &mut CommandFrame) {
    use frame as s;
    r.word_to(fr.addr(s::COMMAND), GET_SHORT);
    r.dword_to(fr.addr(s::FLAGS), GET_ULONG);
    for slot in [s::X1, s::Y1, s::X2, s::Y2] {
        r.word_dword_to(fr.addr(slot), GET_SHORT);
    }
    r.byte_to(fr.addr(s::BORDER), GET_CHAR);
    r.word_dword_to(fr.addr(s::ROW), GET_SHORT);
    r.dword_to(fr.addr(s::STYLE), GET_ULONG);
    r.word_dword_to(fr.addr(s::COLOUR), GET_USHORT);
    r.word_dword_to(fr.addr(s::HIGHLIGHT_COLOUR), GET_USHORT);
    r.dword_to(fr.addr(s::FIELD_3C), GET_ULONG);
    r.byte_to(fr.addr(s::FONT), GET_CHAR);
    r.word_dword_to(fr.addr(s::TEXT_COLOUR), GET_USHORT);
    r.block_to_frame(fr, s::ROW_LENGTH, s::TEXT);
    r.word_to(fr.addr(s::MAX_LENGTH), GET_SHORT);
    r.dword_to(fr.addr(s::ID), GET_LONG);
    r.dword_to(fr.addr(s::VALUE), GET_INT);
    r.word_dword_to(fr.addr(s::AREA), GET_SHORT);
    r.dword_to(fr.addr(s::TARGET), GET_INT);
}

/// Reads command 6's item fields in the second form (the NetworkBuffer getters called: words fill their low word only;
/// the text by `get_sized_block`).
unsafe fn read_dropped_item(r: &mut PacketReader, fr: &mut CommandFrame) {
    use frame as s;
    r.dword_to(fr.addr(s::FLAGS), GET_ULONG);
    r.word_dword_to(fr.addr(s::X1), GET_SHORT);
    for slot in [s::Y1, s::X2, s::Y2] {
        r.word_to(fr.addr(slot), GET_SHORT);
    }
    r.byte_to(fr.addr(s::BORDER), GET_CHAR);
    r.word_to(fr.addr(s::ROW), GET_SHORT);
    r.dword_to(fr.addr(s::STYLE), GET_ULONG);
    r.word_to(fr.addr(s::COLOUR), GET_USHORT);
    r.word_to(fr.addr(s::HIGHLIGHT_COLOUR), GET_USHORT);
    r.dword_to(fr.addr(s::FIELD_3C), GET_ULONG);
    r.byte_to(fr.addr(s::FONT), GET_CHAR);
    r.word_to(fr.addr(s::TEXT_COLOUR), GET_USHORT);
    r.sized_block(fr, s::TEXT);
    r.word_to(fr.addr(s::MAX_LENGTH), GET_SHORT);
    r.dword_to(fr.addr(s::ID), GET_LONG);
    r.dword_to(fr.addr(s::VALUE), GET_INT);
    r.word_to(fr.addr(s::AREA), GET_SHORT);
    r.dword_to(fr.addr(s::TARGET), GET_INT);
}

/// Command 6, an item rebuilt in place (a network player's drag and drop): item k is read and rebuilt; when the packet
/// names a drop target other than k, its drop rectangle is set and that item is rebuilt too. Returns k.
///
/// `read_built_item`, then `Guio::init` (0x5ee150) on item k with the 18 fields (the font word over the text pointer,
/// the area -1); item +0x54 = the target, +0x5c and +0x4c = 0, and a text entry (flag 8) with a target gets its text
/// copied there. The drop target (word): -1 returns k. Otherwise a word is skipped and four dwords read (without room
/// the first is `frame::DROP_STALE`, the others repeat the previous one); item k's `display_item::DROP_TARGET` = the target
/// and `DROP_RECT` = the four. A target equal to k returns k; otherwise `read_dropped_item`, `Guio::init` on the target
/// item (index the target dword, the font word over the length limit's sign extension, the area -1), its +0x54 = the
/// target buffer, +0x5c and +0x4c = 0 and `guio_copy_text_to_caller` (0x567b00). Returns k.
unsafe fn command_rebuild_item(d: usize, r: &mut PacketReader, fr: &mut CommandFrame) -> u16 {
    use frame as s;
    covered!("tel37_5 rebuild an item");
    read_built_item(r, fr);
    let item = display_item_at(d, fr.i16(s::COMMAND));
    let text_high = fr.addr(s::TEXT) as u32;
    init_item(d, item, fr.u32(s::COMMAND), item_fields(fr, text_high, u32::MAX));
    write_u32(item + 0x54, fr.u32(s::TARGET));
    write_u32(item + 0x5c, 0);
    write_u32(item + display_item::BACK_IMAGE, 0);
    if read_u8(item + display_item::FLAGS) & 8 != 0 && read_usize(item + 0x54) != 0 {
        tel26_copy_text(read_usize(item + 0x54) as *mut u8, (item + display_item::TEXT) as *const u8);
    }
    r.word_to(fr.addr(s::OTHER_ITEM), GET_SHORT);
    if fr.i16(s::OTHER_ITEM) == -1 {
        return fr.u16(s::COMMAND);
    }
    r.skip_word();
    let first = r.dword_or(fr.u32(s::DROP_STALE), GET_LONG);
    fr.set_u32(s::DROP_RECT, first);
    let second = r.dword_or(first, GET_LONG);
    fr.set_u32(s::DROP_RECT + 4, second);
    let third = r.dword_or(second, GET_LONG);
    fr.set_u32(s::DROP_RECT + 8, third);
    let fourth = r.dword_or(third, GET_LONG);
    write_u16(item + display_item::DROP_TARGET, fr.u16(s::OTHER_ITEM));
    for (i, v) in [first, second, third, fourth].into_iter().enumerate() {
        write_u32(item + display_item::DROP_RECT + i * 4, v);
    }
    if fr.u16(s::OTHER_ITEM) == fr.u16(s::COMMAND) {
        return fr.u16(s::COMMAND);
    }
    read_dropped_item(r, fr);
    let target = display_item_at(d, fr.i16(s::OTHER_ITEM));
    let high = fr.sx16(s::MAX_LENGTH);
    let f = item_fields(fr, high, u32::MAX);
    init_item(d, target, fr.u32(s::OTHER_ITEM), f);
    write_u32(target + 0x54, fr.u32(s::TARGET));
    write_u32(target + 0x5c, 0);
    write_u32(target + display_item::BACK_IMAGE, 0);
    display_method(0x567b00, target);
    fr.u16(s::COMMAND)
}

/// Reads command 8's item and subcommand and, when the screen is complete and the subcommand known, the subcommand's
/// field; returns the subcommand to apply.
unsafe fn read_item_change(d: usize, r: &mut PacketReader, fr: &mut CommandFrame) -> Option<i32> {
    use frame as s;
    r.word_to(fr.addr(s::OTHER_ITEM), GET_SHORT);
    r.byte_to(fr.addr(s::SUBCOMMAND), GET_CHAR);
    if !screen_complete(d) {
        return None;
    }
    let sub = fr.sx8(s::SUBCOMMAND) as i32;
    match sub {
        0 => r.dword_to(fr.addr(s::FLAGS), GET_ULONG),
        1 => {
            for slot in [s::X1, s::Y1, s::X2, s::Y2] {
                r.word_to(fr.addr(slot), GET_SHORT);
            }
        }
        2 => r.dword_to(fr.addr(s::STYLE), GET_ULONG),
        3 | 4 | 7 => r.word_to(fr.addr(s::COLOUR), GET_USHORT),
        5 => r.dword_to(fr.addr(s::FIELD_3C), GET_ULONG),
        6 => r.byte_to(fr.addr(s::FONT), GET_CHAR),
        8 => r.sized_block(fr, s::TEXT),
        9 => r.dword_to(fr.addr(s::ID), GET_LONG),
        10 => r.dword_to(fr.addr(s::VALUE), GET_INT),
        _ => return None,
    }
    Some(sub)
}

/// Command 8, one field of an item changed: the item (word) and the subcommand (byte); nothing when the screen is not
/// complete (`screen_complete`) or the subcommand is above 10.
///
/// 0 the flags (`display_item::FLAGS`); 1 the rectangle: `guio_set_fixed_rect` (0x567ac0) and `guio_set_rect` (0x567ae0)
/// with x1, y1, x2, y2; 2 the style; 3 the colour (+0x72), 4 the highlight colour (+0x74) and 7 the text colour (+0x78),
/// each converted (`Guio::convert_colours_pixel_format` 1, 2, 4); 5 the +0x3c field; 6 the font (the byte
/// sign-extended into `display_item::FONT`); 8 the text (`Guio::set_text_and_refresh`, not refreshed); 9 the id; 10 the
/// value (+0x48).
unsafe fn command_change_item(d: usize, r: &mut PacketReader, fr: &mut CommandFrame) -> u16 {
    use frame as s;
    let Some(sub) = read_item_change(d, r, fr) else { return 0xffff };
    covered!("tel37_5 change an item");
    let item = display_item_at(d, fr.i16(s::OTHER_ITEM));
    match sub {
        0 => write_u32(item + display_item::FLAGS, fr.u32(s::FLAGS)),
        1 => {
            type SetRect = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32);
            for f in [0x567ac0usize, 0x567ae0] {
                let set: SetRect = core::mem::transmute(f);
                set(item, fr.sx16(s::X1), fr.sx16(s::Y1), fr.sx16(s::X2), fr.sx16(s::Y2));
            }
        }
        2 => write_u32(item + display_item::STYLE, fr.u32(s::STYLE)),
        3 | 4 | 7 => {
            let (field, which) = match sub { 3 => (0x72, 1), 4 => (0x74, 2), _ => (0x78, 4) };
            write_u16(item + field, fr.u16(s::COLOUR));
            convert_item_colours(d, item, which);
        }
        5 => write_u32(item + 0x3c, fr.u32(s::FIELD_3C)),
        6 => write_u16(item + display_item::FONT, fr.sx8(s::FONT) as u16),
        8 => display_set_text(item, fr.addr(s::TEXT) as *const u8, 0),
        9 => write_u32(item + display_item::ID, fr.u32(s::ID)),
        _ => write_u32(item + 0x48, fr.u32(s::VALUE)),
    }
    0xffff
}

/// Reads the fields of the commands whose reads all come before their effects (5, 7, 10 to 16, 25): the plan of the
/// comparer reads them too.
unsafe fn read_plain_command(r: &mut PacketReader, fr: &mut CommandFrame, command: i8) {
    use frame as s;
    match command {
        5 => r.byte_to(fr.addr(s::COMMAND), GET_CHAR),
        7 => {
            r.dword_to(fr.addr(s::STEP), GET_LONG);
            r.dword_to(fr.addr(s::TOTAL), GET_LONG);
            r.sized_block(fr, s::TEXT);
        }
        10 => {
            r.dword_to(fr.addr(s::TARGET), GET_INT);
            r.byte_to(fr.addr(s::PAN), GET_CHAR);
            r.byte_to(fr.addr(s::FLAG_BYTE), GET_CHAR);
            r.sized_block(fr, s::TEXT);
        }
        11 | 14 => r.byte_to(fr.addr(s::SLOT), GET_CHAR),
        12 => {
            r.byte_to(fr.addr(s::SLOT), GET_CHAR);
            r.word_to(fr.addr(s::X1), GET_SHORT);
        }
        13 => {
            r.dword_to(fr.addr(s::TARGET), GET_INT);
            r.byte_to(fr.addr(s::SLOT), GET_CHAR);
        }
        15 | 16 => {
            r.word_to(fr.addr(s::AREA), GET_SHORT);
            r.dword_to(fr.addr(s::STEP), GET_LONG);
        }
        25 => r.sized_block(fr, s::TEXT),
        _ => {}
    }
}

/// Command 10, a sound a network player plays: the reply value, the pan, looping and the file; with a sound player the
/// file is localised and played (`SoundPlayer::play_file` 0x84cbc0) at volume 100 (`set_slot_volume` 0x84ccc0); the
/// slot (0xff without a player) goes back to the player: the send buffer gets 0xd, the value (`put_int`) and the slot,
/// sent by the session (`Network::send_packet` 0x77ac30); a failure shows the "Network Error" box (display.cpp 2864).
unsafe fn command_play_sound(session: usize, player: u32, fr: &mut CommandFrame) {
    use frame as s;
    fr.set_u8(s::SLOT, 0xff);
    if read_usize(SOUND_PLAYER) != 0 {
        localise_media_path(fr.addr(s::TEXT), 0);
        let play: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x84cbc0usize);
        let slot = play(read_usize(SOUND_PLAYER), fr.u32(s::PAN), fr.addr(s::TEXT), fr.sx8(s::FLAG_BYTE));
        fr.set_u8(s::SLOT, slot as u8);
        let volume: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x84ccc0usize);
        volume(read_usize(SOUND_PLAYER), fr.u32(s::SLOT), 0x64);
    }
    write_u32(NET_SEND_BUFFER + network_buffer::POS, 0);
    network_buffer_put_char(NET_SEND_BUFFER, 0xd);
    network_buffer_put_int(NET_SEND_BUFFER, fr.u32(s::TARGET));
    network_buffer_put_char(NET_SEND_BUFFER, fr.u32(s::SLOT));
    write_u16(SCREEN_CHANNEL, player as u16);
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x77ac30usize);
    if send(session, read_usize(NET_SEND_BUFFER + network_buffer::DATA), read_u32(NET_SEND_BUFFER + network_buffer::POS), player) == 0 {
        display_network_error_box(0xb30);
    }
}

/// The commands that only read a few fields and call one function (5, 7, 10 to 16), the input flags and queued items
/// (17 to 20) and the print list (21 to 26).
///
/// 5 a screen section for a network player (`Display::send_screen_section` 0x562370: the session, the player and the
/// section byte's dword); 7 the bottom progress box (`gui_draw_progress_box` 0x5e76f0: the text, the step, the total;
/// the Display's `display_life::DWORD_551` cleared first) and, with [0xb77c8c] set, the cursor mode 1; 10
/// `command_play_sound`; 11 a sound stopped (with a sound player: slot 0xff `SoundPlayer::stop_all`, else `stop_slot`);
/// 12 a slot's volume (`set_slot_volume`, the word sign-extended); 13 and 14 the screen manager's sound slot reported
/// playing (0x802cb0: the slot, the value as the byte's address) and stopped (0x802ce0); 15 a remote area selection
/// (`ScreenManager::set_area_selection`); 16 an area's scroll row (`Area::set_scroll_row`); 17 and 18 the hotkey and click
/// flags [0xb67a0c] / [0xb67a1c] cleared; 19 and 20 `display_list::HOTKEY_ITEM` / `CLICK_ITEM` read (a word); 21 printing
/// nothing (`print_text_lines(0, 0, 0)` 0x84c830); 22 printing the print list (its lines, count and characters per
/// line, the word over the high word of the packet's address); 23 a new print list (`calloc(1, 10)`, its characters per
/// line read); 24 the print list freed (each line, then the lines, the pointer cleared); 25 a line added (the text,
/// `malloc`ed and copied) and 26 a null line (a page break), the lines `realloc`ed one longer first.
unsafe fn command_simple(d: usize, session: usize, player: u32, buf: usize, r: &mut PacketReader, fr: &mut CommandFrame, command: i8) -> u16 {
    use frame as s;
    covered!("tel37_5 simple command");
    read_plain_command(r, fr, command);
    let sound = read_usize(SOUND_PLAYER);
    match command {
        5 => {
            let send: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x562370usize);
            send(d, session, player, fr.u32(s::COMMAND));
        }
        7 => {
            write_u32(d + display_life::DWORD_551, 0);
            let progress: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x5e76f0usize);
            progress(fr.addr(s::TEXT), fr.u32(s::STEP), fr.u32(s::TOTAL));
            if read_u32(0xb77c8c) != 0 {
                exe::set_cursor_mode(1);
            }
        }
        10 => command_play_sound(session, player, fr),
        11 if sound != 0 => {
            if fr.u8(s::SLOT) == 0xff {
                display_method(0x84cb10, sound);
            } else {
                display_method1(0x84cb70, sound, fr.u32(s::SLOT));
            }
        }
        12 if sound != 0 => {
            let volume: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x84ccc0usize);
            volume(sound, fr.u32(s::SLOT), fr.sx16(s::X1));
        }
        13 => {
            let playing: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x802cb0usize);
            playing(G_SCREEN_MANAGER, fr.u32(s::SLOT), fr.u32(s::TARGET));
        }
        14 => display_method1(0x802ce0, G_SCREEN_MANAGER, fr.u32(s::SLOT)),
        15 => {
            let select: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x5680b0usize);
            select(G_SCREEN_MANAGER, fr.u32(s::AREA), fr.u32(s::STEP));
        }
        16 => exe::area_set_scroll_row(display_area_at(d, fr.i16(s::AREA)), fr.u32(s::STEP) as usize),
        17 => write_u32(DISPLAY_HOTKEY_PENDING, 0),
        18 => write_u32(DISPLAY_CLICK_PENDING, 0),
        19 => r.word_to(d + display_list::HOTKEY_ITEM, GET_SHORT),
        20 => r.word_to(d + display_list::CLICK_ITEM, GET_SHORT),
        21 | 22 => {
            let print: unsafe extern "stdcall" fn(usize, u32, u32) = core::mem::transmute(0x84c830usize);
            if command == 21 {
                print(0, 0, 0);
            } else {
                let list = read_usize(d + display_life::MEDIA_LIST);
                if list != 0 {
                    let chars = (read_u32(buf + network_buffer::DATA) & 0xffff_0000) | read_u16(list + print_list::CHARS_PER_LINE) as u32;
                    print(read_usize(list + print_list::LINES), read_u32(list + print_list::COUNT), chars);
                }
            }
        }
        23 => {
            let list = exe::crt_calloc(1, 0xa);
            write_usize(d + display_life::MEDIA_LIST, list);
            if list != 0 {
                r.word_to(list + print_list::CHARS_PER_LINE, GET_SHORT);
            }
        }
        24 => free_print_list(d),
        25 | 26 => add_print_line(d, fr, command),
        _ => {}
    }
    0xffff
}

/// Command 24: frees each line of the print list, then its line array; the Display's pointer cleared.
unsafe fn free_print_list(d: usize) {
    let list = read_usize(d + display_life::MEDIA_LIST);
    if list == 0 {
        return;
    }
    if read_i32(list + print_list::COUNT) > 0 {
        let mut i: u32 = 0;
        loop {
            let list = read_usize(d + display_life::MEDIA_LIST);
            exe::crt_free(read_usize(read_usize(list + print_list::LINES) + (i as i16 as i32 as usize).wrapping_mul(4)));
            i = i.wrapping_add(1);
            if i as i16 as i32 >= read_i32(read_usize(d + display_life::MEDIA_LIST) + print_list::COUNT) {
                break;
            }
        }
    }
    exe::crt_free(read_usize(read_usize(d + display_life::MEDIA_LIST) + print_list::LINES));
    write_usize(d + display_life::MEDIA_LIST, 0);
}

/// Commands 25 and 26: the line array grows by one (`realloc` to count × 4 + 4; nothing on failure); 25 puts a copy of
/// the text (`malloc` of its length + 1; nothing more when it fails), 26 a null line; the count goes up.
unsafe fn add_print_line(d: usize, fr: &mut CommandFrame, command: i8) {
    let list = read_usize(d + display_life::MEDIA_LIST);
    if list == 0 {
        return;
    }
    let lines = exe::crt_realloc(read_usize(list + print_list::LINES), read_u32(list + print_list::COUNT).wrapping_mul(4).wrapping_add(4));
    if lines == 0 {
        return;
    }
    write_usize(read_usize(d + display_life::MEDIA_LIST) + print_list::LINES, lines);
    let list = read_usize(d + display_life::MEDIA_LIST);
    let slot = |l: usize| read_usize(l + print_list::LINES) + (read_u32(l + print_list::COUNT) as usize).wrapping_mul(4);
    if command == 0x19 {
        let text = fr.addr(frame::TEXT);
        let line = exe::crt_malloc(c_string_len(text) as u32 + 1);
        write_usize(slot(list), line);
        let list = read_usize(d + display_life::MEDIA_LIST);
        let line = read_usize(slot(list));
        if line == 0 {
            return;
        }
        write_u32(list + print_list::COUNT, read_u32(list + print_list::COUNT).wrapping_add(1));
        tel26_copy_text(line as *mut u8, text as *const u8);
    } else {
        write_usize(slot(list), 0);
        write_u32(list + print_list::COUNT, read_u32(list + print_list::COUNT).wrapping_add(1));
    }
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
pub(crate) unsafe extern "thiscall" fn display_process_command_packet(d: usize, session: usize, player: u32, buf: usize) -> u32 {
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
    result as u32
}

/// How the comparer checks a packet.
enum PacketCheck {
    /// Side by side over these regions (nothing drawn, sent, allocated or freed).
    SideBySide(Vec<(usize, usize)>),
    /// Only the original, checked against the copy's plan.
    Plan,
    /// Only the original (the network answer, an area with images to release, a truncated packet).
    OriginalOnly,
}

/// Chooses the check of a packet by its command (`buf` with room for it).
unsafe fn packet_check(d: usize, buf: usize, command: i8) -> PacketCheck {
    let data = read_usize(buf + network_buffer::DATA);
    let cap = read_u32(buf + network_buffer::CAPACITY);
    let mut reg = vec![(d, 0x12f550usize), (buf, 12), (SCREEN_CHANNEL, 2)];
    let quiet = read_u32(d + display_list::KEEPALIVE_SENT) != 0;
    let answer = |reg: &mut Vec<(usize, usize)>| {
        reg.push((NET_SEND_BUFFER, 12));
        let send = read_usize(NET_SEND_BUFFER + network_buffer::DATA);
        if send != 0 && read_u32(NET_SEND_BUFFER + network_buffer::CAPACITY) >= 2 {
            reg.push((send, 2));
        }
    };
    match command {
        1 | 2 if quiet => PacketCheck::SideBySide(reg),
        8 if cap < 4 || read_u8(data + 3) == 8 => PacketCheck::Plan,
        1 | 2 | 6 | 8 | 16 | 19 | 20 if !packet_reads_fit(d, buf, command) => PacketCheck::OriginalOnly,
        1 | 2 if read_u32(d + display_list::RESEND_PENDING) != 0 => PacketCheck::OriginalOnly,
        1 => {
            let count = read_i16(d + display_life::WORD_53B).max(0);
            let images = (0..count).any(|a| {
                let area = display_area_at(d, a);
                read_u32(area + 0xba6) != 0 || read_u32(area + 0x200) != 0
            });
            if images {
                return PacketCheck::OriginalOnly;
            }
            answer(&mut reg);
            reg.push((DISPLAY_PACKET_BUFFER, 12));
            let packet = read_usize(DISPLAY_PACKET_BUFFER + network_buffer::DATA);
            if packet != 0 {
                reg.push((packet, read_u32(DISPLAY_PACKET_BUFFER + network_buffer::CAPACITY) as usize));
            }
            reg.push((G_ACTIVE_AREA_SCROLLBAR, 4));
            PacketCheck::SideBySide(reg)
        }
        2 => {
            let pool = read_usize(DISPLAY_SNAPSHOT_POOL);
            if pool == 0 {
                return PacketCheck::OriginalOnly;
            }
            answer(&mut reg);
            reg.push((pool, 0x74042));
            PacketCheck::SideBySide(reg)
        }
        4 | 6 | 8 | 16 | 19 | 20 => PacketCheck::SideBySide(reg),
        17 | 18 => {
            reg.push((DISPLAY_HOTKEY_PENDING, 4));
            reg.push((DISPLAY_CLICK_PENDING, 4));
            PacketCheck::SideBySide(reg)
        }
        0 | 3 | 5 | 7 | 9..=15 | 21..=26 => PacketCheck::Plan,
        _ => PacketCheck::SideBySide(reg),
    }
}

/// Whether the reads of a packet the comparer runs twice all fit in the buffer (read by the copy's readers on a copy of
/// the buffer header, no box): a truncated packet's error boxes do not run twice.
unsafe fn packet_reads_fit(d: usize, buf: usize, command: i8) -> bool {
    use frame as s;
    let mut header = [read_u32(buf), 1u32, read_u32(buf + 8)];
    let mut r = PacketReader::new(header.as_mut_ptr() as usize, true);
    let mut fr = CommandFrame::new();
    match command {
        1 => {
            for _ in 0..read_i16(d + display_life::WORD_53B).max(0) {
                read_area(&mut r, &mut fr);
            }
        }
        2 => {
            for _ in 0..read_i16(d + display_life::WORD_539).max(0) {
                r.dword_to(fr.addr(s::LENGTH), GET_ULONG);
                let mask = fr.u32(s::LENGTH);
                read_item_fields(d, &mut r, &mut fr, mask, false);
            }
        }
        6 => {
            read_built_item(&mut r, &mut fr);
            r.word_to(fr.addr(s::OTHER_ITEM), GET_SHORT);
            if fr.i16(s::OTHER_ITEM) != -1 {
                r.skip_word();
                for _ in 0..4 {
                    r.dword_or(0, GET_LONG);
                }
                if fr.u16(s::OTHER_ITEM) != fr.u16(s::COMMAND) {
                    read_dropped_item(&mut r, &mut fr);
                }
            }
        }
        8 => {
            read_item_change(d, &mut r, &mut fr);
        }
        16 => read_plain_command(&mut r, &mut fr, 16),
        _ => r.skip_word(),
    }
    !r.failed
}

/// The copy's plan of a packet only the original applies: the buffer position after it, read by the copy's own readers
/// on a copy of the buffer header (no box), and the Display fields it must leave (command 0's counts, focus and colour
/// format; the print list of 23 to 26). None when a read would fail (the original shows the box).
unsafe fn packet_plan(d: usize, buf: usize, command: i8) -> Option<(u32, Vec<(usize, u32, usize)>)> {
    let mut header = [read_u32(buf), 1u32, read_u32(buf + 8)];
    let mut r = PacketReader::new(header.as_mut_ptr() as usize, true);
    let mut fr = CommandFrame::new();
    let mut fields = Vec::new();
    match command {
        0 => {
            let mut f = new_screen_fields(d);
            read_new_screen(&mut r, &mut fr, &mut f);
            fields = vec![(d + display_life::WORD_53B, f.area_count as u32, 2), (d + display_life::WORD_539, f.item_count as u32, 2),
                          (d + display::FOCUS_FLAGS, f.focus_flags as u32, 1), (d + display::FOCUS_ITEM, f.focus_item as u32, 2),
                          (d + display_packet::COLOUR_FORMAT, fr.sx8(frame::FLAG_BYTE), 4)];
        }
        8 => {
            read_item_change(d, &mut r, &mut fr);
        }
        23 => {
            let mut chars = 0u16;
            r.word_to(&mut chars as *mut u16 as usize, GET_SHORT);
            fields.push((usize::MAX, chars as u32, 23));
        }
        24 => fields.push((d + display_life::MEDIA_LIST, 0, 4)),
        _ => {
            read_plain_command(&mut r, &mut fr, command);
            let list = read_usize(d + display_life::MEDIA_LIST);
            if (command == 25 || command == 26) && list != 0 {
                fields.push((list + print_list::COUNT, read_u32(list + print_list::COUNT).wrapping_add(1), 4));
            }
        }
    }
    if r.failed {
        return None;
    }
    Some((header[1], fields))
}

/// Comparer for `display_process_command_packet`: by the command (`packet_check`), side by side over the whole Display,
/// the buffer header, the channel and what the command also writes (the send buffer's answer, the display packet buffer
/// of `add_area`, the snapshot pool, the input flags), with the result in `ax`; the commands that draw, send, play,
/// print, allocate or free run only in the original, checked against the copy's plan (`packet_plan`: the buffer
/// position and the Display fields).
pub(crate) unsafe extern "thiscall" fn compare_display_process_command_packet(d: usize, session: usize, player: u32, buf: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(%%I0%%);
    if read_u32(buf + network_buffer::CAPACITY) < 1 {
        // simplification: the box of an empty packet does not run twice
        return o(d, session, player, buf);
    }
    let command = read_u8(read_usize(buf + network_buffer::DATA)) as i8;
    match packet_check(d, buf, command) {
        PacketCheck::OriginalOnly => o(d, session, player, buf),
        PacketCheck::SideBySide(reg) => {
            let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(d, session, player, buf), || display_process_command_packet(d, session, player, buf));
            SCOREBOARD.record(same && r as u16 == n as u16, || format!("Display::process_command_packet command {} (screen packet of {} bytes): original={} rust={} differences {:?}",
                command, read_u32(buf + network_buffer::CAPACITY), r as u16 as i16, n as u16 as i16, first_differences(&mo, &mr)));
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
