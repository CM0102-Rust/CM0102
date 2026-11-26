// ---------- rx03_3: the T2K input stream: big-endian reads and the read position (library_without_asserts, T2K t2kstrm.c) ----------

/// The T2K input stream (`InputStream`, 29 bytes, packed): +0 the data, +4 the read position, +8 the end offset,
/// +0xc the start offset, +0x10 the kind (1 owns its data, 2 a slice of a scratch buffer, 3 the caller's buffer),
/// +0x11 the memory context, +0x15 and +0x19 zero.
pub(crate) mod t2k_stream {
    pub(crate) const DATA: usize = 0x0;
    pub(crate) const POS: usize = 0x4;
    pub(crate) const END: usize = 0x8;
    pub(crate) const START: usize = 0xc;
}

/// The address of the next byte of the stream, and the position moved on by `n`.
pub(crate) unsafe fn stream_take(s: usize, n: u32) -> usize {
    let pos = read_u32(s + t2k_stream::POS);
    write_u32(s + t2k_stream::POS, pos.wrapping_add(n));
    read_usize(s + t2k_stream::DATA).wrapping_add(pos as usize)
}

/// Reads a big-endian 32-bit value and moves the position on by 4.
/// Original: `input_stream_read_int32` (0x941580, library_without_asserts, T2K t2kstrm.c, cdecl).
pub(crate) unsafe extern "C" fn input_stream_read_int32(s: usize) -> u32 {
    u32::from_be(read_u32(stream_take(s, 4)))
}

/// Reads a big-endian 16-bit value and moves the position on by 2 (the original leaves the data pointer's high half in
/// the upper bits of eax; callers read ax).
/// Original: `input_stream_read_int16` (0x9415c0, library_without_asserts, T2K t2kstrm.c, cdecl).
pub(crate) unsafe extern "C" fn input_stream_read_int16(s: usize) -> i16 {
    i16::from_be(read_i16(stream_take(s, 2)))
}

/// Copies `n` bytes to `out` and moves the position on by `n`; nothing for `n` <= 0.
/// Original: `input_stream_read_bytes` (0x9415e0, library_without_asserts, T2K t2kstrm.c, cdecl).
pub(crate) unsafe extern "C" fn input_stream_read_bytes(s: usize, out: usize, n: i32) {
    if n > 0 {
        let from = stream_take(s, n as u32);
        core::ptr::copy(from as *const u8, out as *mut u8, n as usize);
    }
}

/// The stream's size: the end offset minus the start offset.
/// Original: `input_stream_get_size` (0x941620, library_without_asserts, T2K t2kstrm.c, cdecl).
pub(crate) unsafe extern "C" fn input_stream_get_size(s: usize) -> i32 {
    read_i32(s + t2k_stream::END).wrapping_sub(read_i32(s + t2k_stream::START))
}

/// Moves the read position back to the start offset.
/// Original: `input_stream_rewind` (0x941730, library_without_asserts, T2K t2kstrm.c, cdecl).
pub(crate) unsafe extern "C" fn input_stream_rewind(s: usize) {
    write_u32(s + t2k_stream::POS, read_u32(s + t2k_stream::START));
}

/// Sets the read position to the start offset plus `offset`.
/// Original: `input_stream_seek` (0x941740, library_without_asserts, T2K t2kstrm.c, cdecl).
pub(crate) unsafe extern "C" fn input_stream_seek(s: usize, offset: u32) {
    write_u32(s + t2k_stream::POS, read_u32(s + t2k_stream::START).wrapping_add(offset));
}

/// Reads an unsigned value of seven bits a byte, low group first, while the byte's top bit is set (the shift wraps
/// at 32, as x86 `shl`).
/// Original: `input_stream_read_varint7` (0x9417f0, library_without_asserts, T2K t2kstrm.c, cdecl).
pub(crate) unsafe extern "C" fn input_stream_read_varint7(s: usize) -> u32 {
    let (mut value, mut shift) = (0u32, 0u32);
    loop {
        let byte = read_u8(stream_take(s, 1));
        value |= ((byte & 0x7f) as u32).wrapping_shl(shift);
        shift = shift.wrapping_add(7);
        if byte & 0x80 == 0 {
            return value;
        }
    }
}

/// Comparers: side by side on the read position (and the bytes read out); the value against the value.
unsafe fn compare_stream_read<R: PartialEq + core::fmt::Debug>(name: &str, s: usize, extra: (usize, usize), original: impl FnOnce() -> R, copy: impl FnOnce() -> R) -> R {
    let (r, c, same, _, _) = crate::comparison::side_by_side_light(&[(s + t2k_stream::POS, 4), extra], original, copy);
    SCOREBOARD.record(same && r == c, || format!("{}: original={:?} rust={:?} position equal={}", name, r, c, same));
    r
}

pub(crate) unsafe extern "C" fn compare_input_stream_read_int32(s: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%);
    compare_stream_read("input_stream_read_int32", s, (s, 0), || o(s), || input_stream_read_int32(s))
}

pub(crate) unsafe extern "C" fn compare_input_stream_read_int16(s: usize) -> i16 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> i16>(%%I1%%);
    compare_stream_read("input_stream_read_int16", s, (s, 0), || o(s), || input_stream_read_int16(s))
}

pub(crate) unsafe extern "C" fn compare_input_stream_read_bytes(s: usize, out: usize, n: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32)>(%%I2%%);
    let bytes = if n > 0 && n <= 0x100000 { n as usize } else { 0 };
    compare_stream_read("input_stream_read_bytes", s, (out, bytes), || o(s, out, n), || input_stream_read_bytes(s, out, n))
}

pub(crate) unsafe extern "C" fn compare_input_stream_get_size(s: usize) -> i32 {
    let (r, c) = (original_fn::<unsafe extern "C" fn(usize) -> i32>(%%I3%%)(s), input_stream_get_size(s));
    SCOREBOARD.record(r == c, || format!("input_stream_get_size original={} rust={}", r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_input_stream_rewind(s: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(%%I4%%);
    compare_stream_read("input_stream_rewind", s, (s, 0), || o(s), || input_stream_rewind(s))
}

pub(crate) unsafe extern "C" fn compare_input_stream_seek(s: usize, offset: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(%%I5%%);
    compare_stream_read("input_stream_seek", s, (s, 0), || o(s, offset), || input_stream_seek(s, offset))
}

pub(crate) unsafe extern "C" fn compare_input_stream_read_varint7(s: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I6%%);
    compare_stream_read("input_stream_read_varint7", s, (s, 0), || o(s), || input_stream_read_varint7(s))
}
