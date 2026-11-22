// ---------- batch %%LEVA%%: the network buffer: putting and getting chars, words, dwords and sized blocks (display.cpp, network.h) ----------

/// The NetworkBuffer (network.h, inlined in display.cpp): the command packets the screen manager writes and the
/// Display reads (`Display::process_command_packet`), also in a local game. Two global ones: the network send buffer
/// (0xae28f8) and the display packet buffer (0xae2908).
#[allow(dead_code)]
pub(crate) mod network_buffer {
    /// Pointer to the bytes.
    pub(crate) const DATA: usize = 0x0;
    /// u32: the read or write position.
    pub(crate) const POS: usize = 0x4;
    /// u32: the size of the bytes.
    pub(crate) const CAPACITY: usize = 0x8;
}

/// The network send buffer and the display packet buffer (NetworkBuffer objects).
pub(crate) const NET_SEND_BUFFER: usize = 0xae28f8;
pub(crate) const DISPLAY_PACKET_BUFFER: usize = 0xae2908;

/// Whether `size` more bytes fit at the position (`POS + size <= CAPACITY`, unsigned); otherwise the "Error" box of
/// network.h at `line` (the left button let go) and false.
pub(crate) unsafe fn network_buffer_room(buf: usize, size: u32, line: u32) -> bool {
    if read_u32(buf + network_buffer::POS).wrapping_add(size) > read_u32(buf + network_buffer::CAPACITY) {
        tel26_error_box(NETWORK_H, line);
        return false;
    }
    true
}

/// The address of the position in the buffer.
pub(crate) unsafe fn network_buffer_cursor(buf: usize) -> usize {
    read_usize(buf + network_buffer::DATA).wrapping_add(read_u32(buf + network_buffer::POS) as usize)
}

/// Advances the position by `n`.
pub(crate) unsafe fn network_buffer_advance(buf: usize, n: u32) {
    write_u32(buf + network_buffer::POS, read_u32(buf + network_buffer::POS).wrapping_add(n));
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

/// Puts a signed word: with room for 2 (else the box network.h 134) it is written at the position, which advances by 2.
/// Original: `NetworkBuffer::put_short` (0x5601e0, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_short(buf: usize, value: u32) {
    if network_buffer_room(buf, 2, 0x86) {
        write_u16(network_buffer_cursor(buf), value as u16);
        network_buffer_advance(buf, 2);
    }
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

/// Puts an unsigned dword: with room for 4 (else the box network.h 141) it is written at the position, which advances
/// by 4.
/// Original: `NetworkBuffer::put_ulong` (0x560340, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_put_ulong(buf: usize, value: u32) {
    if network_buffer_room(buf, 4, 0x8d) {
        write_u32(network_buffer_cursor(buf), value);
        network_buffer_advance(buf, 4);
    }
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

/// Gets a signed word into `out`: with room for 2 (else the box network.h 144) the word at the position, which
/// advances by 2.
/// Original: `NetworkBuffer::get_short` (0x567c80, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_short(buf: usize, out: usize) {
    if network_buffer_room(buf, 2, 0x90) {
        write_u16(out, read_u16(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 2);
    }
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

/// Gets a signed dword (int) into `out`: with room for 4 (else the box network.h 147) the dword at the position, which
/// advances by 4.
/// Original: `NetworkBuffer::get_int` (0x567de0, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_int(buf: usize, out: usize) {
    if network_buffer_room(buf, 4, 0x93) {
        write_u32(out, read_u32(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 4);
    }
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

/// Gets an unsigned dword into `out`: with room for 4 (else the box network.h 151) the dword at the position, which
/// advances by 4.
/// Original: `NetworkBuffer::get_ulong` (0x567f40, display.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_get_ulong(buf: usize, out: usize) {
    if network_buffer_room(buf, 4, 0x97) {
        write_u32(out, read_u32(network_buffer_cursor(buf)));
        network_buffer_advance(buf, 4);
    }
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

/// Side by side over a network buffer call: the buffer header and `bytes` (address, size) as data; no room only the
/// original (the error box).
pub(crate) unsafe fn compare_network_buffer(buf: usize, size: u32, bytes: (usize, usize), what: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    if read_u32(buf + network_buffer::POS).wrapping_add(size) > read_u32(buf + network_buffer::CAPACITY) {
        // simplification: the error box does not run twice
        return original();
    }
    let reg = [(buf, 12), bytes];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, original, rust);
    SCOREBOARD.record(same, || format!("network buffer {} (buffer {:#x}): differences {:?}", what, buf, first_differences(&mo, &mr)));
}

/// Comparer for `network_buffer_put_char`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_char(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
    compare_network_buffer(buf, 1, (network_buffer_cursor(buf), 1), "put_char", || o(buf, value), || network_buffer_put_char(buf, value));
}

/// Comparer for `network_buffer_put_short`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_short(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    compare_network_buffer(buf, 2, (network_buffer_cursor(buf), 2), "put_short", || o(buf, value), || network_buffer_put_short(buf, value));
}

/// Comparer for `network_buffer_put_ushort`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_ushort(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%);
    compare_network_buffer(buf, 2, (network_buffer_cursor(buf), 2), "put_ushort", || o(buf, value), || network_buffer_put_ushort(buf, value));
}

/// Comparer for `network_buffer_put_ulong`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_ulong(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I3%%);
    compare_network_buffer(buf, 4, (network_buffer_cursor(buf), 4), "put_ulong", || o(buf, value), || network_buffer_put_ulong(buf, value));
}

/// Comparer for `network_buffer_put_int`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_put_int(buf: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I4%%);
    compare_network_buffer(buf, 4, (network_buffer_cursor(buf), 4), "put_int", || o(buf, value), || network_buffer_put_int(buf, value));
}

/// Comparer for `network_buffer_get_char`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_char(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I5%%);
    compare_network_buffer(buf, 1, (out, 1), "get_char", || o(buf, out), || network_buffer_get_char(buf, out));
}

/// Comparer for `network_buffer_get_short`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_short(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I6%%);
    compare_network_buffer(buf, 2, (out, 2), "get_short", || o(buf, out), || network_buffer_get_short(buf, out));
}

/// Comparer for `network_buffer_get_long`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_long(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I7%%);
    compare_network_buffer(buf, 4, (out, 4), "get_long", || o(buf, out), || network_buffer_get_long(buf, out));
}

/// Comparer for `network_buffer_get_int`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_int(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I8%%);
    compare_network_buffer(buf, 4, (out, 4), "get_int", || o(buf, out), || network_buffer_get_int(buf, out));
}

/// Comparer for `network_buffer_get_ushort`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_ushort(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I9%%);
    compare_network_buffer(buf, 2, (out, 2), "get_ushort", || o(buf, out), || network_buffer_get_ushort(buf, out));
}

/// Comparer for `network_buffer_get_ulong`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_ulong(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I10%%);
    compare_network_buffer(buf, 4, (out, 4), "get_ulong", || o(buf, out), || network_buffer_get_ulong(buf, out));
}

/// Comparer for `network_buffer_get_sized_block`: the header and the block written to `out` (the length read from
/// the buffer) side by side; no room for the length only the original.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_get_sized_block(buf: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I11%%);
    if read_u32(buf + network_buffer::POS).wrapping_add(2) > read_u32(buf + network_buffer::CAPACITY) {
        // simplification: the error box (and the copy of an uninitialised length) does not run twice
        return o(buf, out);
    }
    let n = read_i16(network_buffer_cursor(buf)).max(0) as usize;
    compare_network_buffer(buf, 2, (out, n), "get_sized_block", || o(buf, out), || network_buffer_get_sized_block(buf, out));
}
