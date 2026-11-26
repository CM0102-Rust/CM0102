// ---------- rx03_7: the T2K glyph: the compressed outlines, the empty glyph, the composite glyphs, the point arrays and the bold style (library_without_asserts, T2K glyph.c) ----------

/// The T2K glyph (`GlyphClass`, 0x73 bytes, usually from scratch slot 0): +0 the memory context, +4 the contour count
/// and +6 the point count of the arrays, +0x12 and +0x22 room for 8 contour starts and ends, +0x32 the curve order (2),
/// +0x34 the contour count (negative: a composite glyph), +0x36 the point count, +0x38 and +0x3c the contour starts and
/// ends, +0x40 and +0x44 the font unit coordinates (i16), +0x48 the on curve flags, +0x4c and +0x50 the scaled 26.6
/// coordinates, +0x54 the component records (composite), +0x58 their count, +0x5c their capacity, +0x60 the
/// instructions and +0x64 their length, +0x68..+0x6e the bounding box (x min, y min, x max, y max), +0x71 the glyph
/// index. Each point array has four phantom points after the outline (left and right side bearing, top and bottom).
pub(crate) mod t2k_glyph {
    pub(crate) const ARRAY_CONTOURS: usize = 0x4;
    pub(crate) const ARRAY_POINTS: usize = 0x6;
    pub(crate) const INLINE_STARTS: usize = 0x12;
    pub(crate) const INLINE_ENDS: usize = 0x22;
    pub(crate) const CURVE_ORDER: usize = 0x32;
    pub(crate) const CONTOURS: usize = 0x34;
    pub(crate) const POINTS: usize = 0x36;
    pub(crate) const STARTS: usize = 0x38;
    pub(crate) const ENDS: usize = 0x3c;
    pub(crate) const OOX: usize = 0x40;
    pub(crate) const OOY: usize = 0x44;
    pub(crate) const ON_CURVE: usize = 0x48;
    pub(crate) const X: usize = 0x4c;
    pub(crate) const Y: usize = 0x50;
    pub(crate) const COMPONENTS: usize = 0x54;
    pub(crate) const COMPONENT_COUNT: usize = 0x58;
    pub(crate) const COMPONENT_CAPACITY: usize = 0x5c;
    pub(crate) const HINTS: usize = 0x60;
    pub(crate) const HINT_LENGTH: usize = 0x64;
    pub(crate) const X_MIN: usize = 0x68;
    pub(crate) const Y_MIN: usize = 0x6a;
    pub(crate) const X_MAX: usize = 0x6c;
    pub(crate) const Y_MAX: usize = 0x6e;
    pub(crate) const INDEX: usize = 0x71;
    pub(crate) const SIZE: u32 = 0x73;
}

/// A composite glyph's component records: 0x400 words first (a 0x800-byte block), grown by half when within 10 words
/// of the end; each record is its flags, the glyph, the two arguments (one word each with flag 1, else one word for
/// both), the scale (flag 8: one word; 0x40: two; 0x80: four); records follow while flag 0x20 is set.
/// Returns the block, its word count and the last record's flags.
pub(crate) unsafe fn read_components(g: usize, s: usize) -> (usize, i32, u16) {
    write_i32(g + t2k_glyph::COMPONENT_CAPACITY, 0x400);
    let mut data = lib_ctx_alloc(read_usize(g), 0x800);
    let mut n = 0i32;
    let put = |data: usize, n: &mut i32, v: i16| {
        write_i16(data + 2 * *n as usize, v);
        *n += 1;
    };
    loop {
        let capacity = read_i32(g + t2k_glyph::COMPONENT_CAPACITY);
        if n >= capacity - 10 {
            let grown = capacity / 2 + capacity;
            write_i32(g + t2k_glyph::COMPONENT_CAPACITY, grown);
            data = lib_ctx_realloc(read_usize(g), data, grown.wrapping_mul(2) as u32);
        }
        let flags = input_stream_read_int16(s) as u16;
        put(data, &mut n, flags as i16);
        put(data, &mut n, input_stream_read_int16(s));
        if flags & 1 != 0 {
            put(data, &mut n, input_stream_read_int16(s));
        }
        put(data, &mut n, input_stream_read_int16(s));
        let scale_words = if flags & 8 != 0 {
            1
        } else if flags & 0x40 != 0 {
            2
        } else if flags & 0x80 != 0 {
            4
        } else {
            0
        };
        for _ in 0..scale_words {
            put(data, &mut n, input_stream_read_int16(s));
        }
        if flags & 0x20 == 0 {
            return (data, n, flags);
        }
    }
}

/// The four phantom points after `n` points of the arrays `xs` and `ys` (font units): (x min − lsb, 0), (that + aw,
/// 0), (their middle, y max + tsb), (the same x, that y − ah), with the glyph's x min and y max.
pub(crate) unsafe fn write_phantom_points(g: usize, xs: usize, ys: usize, n: i32, lsb: i32, aw: i32, tsb: i32, ah: i32) {
    let (xs, ys) = (xs.wrapping_add(2 * n as usize), ys.wrapping_add(2 * n as usize));
    write_i16(ys, 0);
    write_i16(xs, read_i16(g + t2k_glyph::X_MIN).wrapping_sub(lsb as i16));
    write_i16(ys + 2, 0);
    let right = read_i16(xs).wrapping_add(aw as i16);
    write_i16(xs + 2, right);
    let left = read_i16(xs);
    write_i16(ys + 4, read_i16(g + t2k_glyph::Y_MAX).wrapping_add(tsb as i16));
    let middle = ((left as i32 + right as i32) >> 1) as i16;
    write_i16(xs + 4, middle);
    write_i16(ys + 6, read_i16(ys + 4).wrapping_sub(ah as i16));
    write_i16(xs + 6, middle);
}

/// Decodes one T2K compressed point step: a big-endian word whose top two bits are the quadrant and the low 14 bits a
/// code (with zero and quadrant 0 or 1: an explicit 16-bit dx and dy follow, the on curve flag being quadrant 0);
/// codes below 0x898 give (code, 0), below 0x313c the pair (c / 102 + 1, c % 102 + 1) of `code − 0x898`, below 0x393c
/// with one more byte the pair (v / 724 + 1, v % 724 + 1), else with two more bytes (v / 10650, v % 10650). The
/// quadrant sets the signs and the order: (a, b), (−b, a), (−a, −b), (b, −a); dx is the first halved (arithmetic) and
/// its low bit clear means on curve.
/// Original: `glyph_outline_read_compressed_point_delta` (0x93bbd0, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_outline_read_compressed_point_delta(s: usize, dx: usize, dy: usize) -> u32 {
    let byte = || read_u8(stream_take(s, 1)) as u32;
    let word = byte() << 8 | byte();
    let (code, quadrant) = (word & 0x3fff, word >> 14);
    let (a, b) = if code < 0x898 {
        if code == 0 && quadrant <= 1 {
            let x = (byte() << 8 | byte()) as i16;
            let y = (byte() << 8 | byte()) as i16;
            write_i16(dx, x);
            write_i16(dy, y);
            return (quadrant == 0) as u32;
        }
        (code, 0)
    } else if code < 0x313c {
        let v = code - 0x898;
        (v / 0x66 + 1, v % 0x66 + 1)
    } else if code < 0x393c {
        let v = (code - 0x313c) << 8 | byte();
        (v / 0x2d4 + 1, v % 0x2d4 + 1)
    } else {
        let high = (code - 0x393c) << 8 | byte();
        let v = high << 8 | byte();
        (v / 0x299a, v % 0x299a)
    };
    let (first, second) = match quadrant {
        0 => (a, b),
        1 => (b.wrapping_neg(), a),
        2 => (a.wrapping_neg(), b.wrapping_neg()),
        _ => (b, a.wrapping_neg()),
    };
    write_i16(dx, ((first as i32) >> 1) as i16);
    write_i16(dy, second as i16);
    1 - (first & 1)
}

/// The fields every new glyph starts with (the arrays null, the curve order 2, no components or instructions).
unsafe fn glyph_clear_header(g: usize, mem: usize) {
    write_usize(g, mem);
    for field in [t2k_glyph::ENDS, t2k_glyph::STARTS, t2k_glyph::COMPONENTS, t2k_glyph::Y, t2k_glyph::X] {
        write_u32(g + field, 0);
    }
    write_u32(g + 0xa, 0);
    write_u32(g + 0xe, 0);
    write_i16(g + t2k_glyph::CURVE_ORDER, 2);
    write_i16(g + t2k_glyph::ARRAY_CONTOURS, 0);
    write_u32(g + t2k_glyph::ARRAY_POINTS, 0);
}

/// The point arrays of a glyph without outline points: the four phantom points only (a 0x34-byte block from scratch
/// slot 1).
unsafe fn glyph_phantom_only_arrays(g: usize) {
    let block = lib_ctx_acquire_scratch_buffer(read_usize(g), 0x34, 1);
    write_usize(g + t2k_glyph::X, block);
    write_usize(g + t2k_glyph::Y, block + 0x10);
    write_usize(g + t2k_glyph::OOX, block + 0x20);
    write_u32(g + t2k_glyph::ARRAY_POINTS, 0);
    write_usize(g + t2k_glyph::OOY, block + 0x28);
    write_usize(g + t2k_glyph::ON_CURVE, block + 0x30);
}

/// An empty glyph (scratch slot 0): no contours, the four phantom points from the advance and bearings: (0, 0),
/// (aw, 0), (0, tsb), (0, tsb − ah).
/// Original: `glyph_outline_new_empty` (0x93bd70, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_outline_new_empty(mem: usize, _lsb: i32, aw: i32, tsb: i32, ah: i32) -> usize {
    let g = lib_ctx_acquire_scratch_buffer(mem, t2k_glyph::SIZE, 0);
    glyph_clear_header(g, mem);
    write_u32(g + t2k_glyph::COMPONENT_CAPACITY, 0);
    write_i16(g + t2k_glyph::CONTOURS, 0);
    for field in [t2k_glyph::X_MIN, t2k_glyph::Y_MIN, t2k_glyph::X_MAX, t2k_glyph::Y_MAX] {
        write_i16(g + field, 0);
    }
    for field in [t2k_glyph::COMPONENT_COUNT, t2k_glyph::HINT_LENGTH, t2k_glyph::HINTS] {
        write_u32(g + field, 0);
    }
    glyph_phantom_only_arrays(g);
    let (xs, ys) = (read_usize(g + t2k_glyph::OOX), read_usize(g + t2k_glyph::OOY));
    write_i16(ys, 0);
    write_i16(xs, 0);
    write_i16(ys + 2, 0);
    write_i16(xs + 2, read_i16(xs).wrapping_add(aw as i16));
    write_i16(ys + 4, read_i16(g + t2k_glyph::Y_MAX).wrapping_add(tsb as i16));
    write_i16(xs + 4, 0);
    write_i16(ys + 6, read_i16(ys + 4).wrapping_sub(ah as i16));
    write_i16(xs + 6, 0);
    write_i16(g + t2k_glyph::POINTS, 0);
    g
}

/// Reads a T2K compressed glyph (the .t2k fonts; scratch slot 0): the contour count; a composite glyph keeps its
/// component records as `tt_glyph_decode_outline` does (no instructions) and only phantom points; a simple glyph reads
/// its contour ends as 7-bit varints (each a step from the next start) and its points as compressed steps
/// (`glyph_outline_read_compressed_point_delta`), keeping the x minimum and the y maximum (from 0x7fff and −0x7fff) as
/// the box's x min and y max; then the four phantom points as in `tt_glyph_decode_outline`.
/// Defect of the original: a glyph with no contours (and some bytes) writes its phantom points through null arrays.
/// Original: `glyph_outline_read_from_stream` (0x93be40, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_outline_read_from_stream(mem: usize, s: usize, _read_hints: i32, lsb: i32, aw: i32, tsb: i32, ah: i32, _extra: usize) -> usize {
    let g = lib_ctx_acquire_scratch_buffer(mem, t2k_glyph::SIZE, 0);
    glyph_clear_header(g, mem);
    let contours = input_stream_read_int16(s);
    write_i16(g + t2k_glyph::CONTOURS, contours);
    for field in [t2k_glyph::X_MIN, t2k_glyph::Y_MIN, t2k_glyph::X_MAX, t2k_glyph::Y_MAX] {
        write_i16(g + field, 0);
    }
    for field in [t2k_glyph::COMPONENT_COUNT, t2k_glyph::HINT_LENGTH, t2k_glyph::HINTS] {
        write_u32(g + field, 0);
    }
    let mut points = 0i32;
    if contours < 0 {
        let (data, n, _) = read_components(g, s);
        write_u32(g + t2k_glyph::HINT_LENGTH, 0);
        glyph_phantom_only_arrays(g);
        write_usize(g + t2k_glyph::COMPONENTS, data);
        write_i32(g + t2k_glyph::COMPONENT_COUNT, n);
    } else if contours > 0 {
        points = read_compressed_outline(g, s, contours);
    }
    let (xs, ys) = if contours == 0 { (0, 0) } else { (read_usize(g + t2k_glyph::OOX), read_usize(g + t2k_glyph::OOY)) };
    write_phantom_points(g, xs, ys, points, lsb, aw, tsb, ah);
    write_i16(g + t2k_glyph::POINTS, points as i16);
    g
}

/// A compressed simple outline (see `glyph_outline_read_from_stream`); returns the point count.
unsafe fn read_compressed_outline(g: usize, s: usize, contours: i16) -> i32 {
    if contours < 9 {
        write_usize(g + t2k_glyph::STARTS, g + t2k_glyph::INLINE_STARTS);
        write_usize(g + t2k_glyph::ENDS, g + t2k_glyph::INLINE_ENDS);
    } else {
        let block = lib_ctx_alloc(read_usize(g), (contours as i32).wrapping_shl(2) as u32);
        write_usize(g + t2k_glyph::STARTS, block);
        write_usize(g + t2k_glyph::ENDS, block + 2 * read_i16(g + t2k_glyph::CONTOURS) as usize);
    }
    let mut next = 0i16;
    let mut k = 0;
    while k < read_i16(g + t2k_glyph::CONTOURS) as i32 {
        write_i16(read_usize(g + t2k_glyph::STARTS) + 2 * k as usize, next);
        let step = input_stream_read_varint7(s) as i16;
        let end_at = read_usize(g + t2k_glyph::ENDS) + 2 * k as usize;
        write_i16(end_at, step.wrapping_add(next));
        next = read_i16(end_at).wrapping_add(1);
        k += 1;
    }
    let n = next as i32;
    write_u32(g + t2k_glyph::HINT_LENGTH, 0);
    glyph_outline_alloc_point_arrays(g, n);
    write_i16(g + t2k_glyph::ARRAY_CONTOURS, read_i16(g + t2k_glyph::CONTOURS));
    let (oox, ooy, on) = (read_usize(g + t2k_glyph::OOX), read_usize(g + t2k_glyph::OOY), read_usize(g + t2k_glyph::ON_CURVE));
    let (mut x, mut y) = (0i32, 0i32);
    let (mut x_min, mut y_max) = (0x7fffi16, -0x7fffi16);
    let (mut dx, mut dy) = (0i16, 0i16);
    for k in 0..n.max(0) as usize {
        let on_curve = glyph_outline_read_compressed_point_delta(s, &mut dx as *mut i16 as usize, &mut dy as *mut i16 as usize);
        write_u8(on + k, on_curve as u8);
        x = x.wrapping_add(dx as i32);
        y = y.wrapping_add(dy as i32);
        if (x as i16) < x_min {
            x_min = x as i16;
        }
        if y_max < y as i16 {
            y_max = y as i16;
        }
        write_i16(oox + 2 * k, x as i16);
        write_i16(ooy + 2 * k, y as i16);
    }
    write_i16(g + t2k_glyph::X_MIN, x_min);
    write_i16(g + t2k_glyph::Y_MAX, y_max);
    n
}

/// Appends a component glyph to a composite glyph being built (`*base`). The first component becomes the base, moved by
/// (arg1, arg2) when they are an offset (flag 2; rounded to the pixel with flag 4). Later ones go into new arrays
/// (a block for `n` points, not from the scratch slots, and one for the contours): the base's points, its first two
/// phantom points unless flag 0x200, the component's points moved by the offset (flag 2, rounded with flag 4) or by
/// the distance between base point arg1 and component point arg2, the contours (the component's shifted by the base's
/// point count); the base's old arrays are freed (or their scratch slot given back) and it takes the component's
/// curve order.
/// Original: `glyph_outline_append_component` (0x93c200, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_outline_append_component(base_at: usize, comp: usize, flags: u32, arg1: u32, arg2: u32) {
    let base = read_usize(base_at);
    if base == 0 {
        if flags & 2 != 0 {
            let (mut dx, mut dy) = (arg1, arg2);
            if flags & 4 != 0 {
                dx = dx.wrapping_add(0x20) & !0x3f;
                dy = dy.wrapping_add(0x20) & !0x3f;
            }
            if dx != 0 || dy != 0 {
                for k in 0..(read_i16(comp + t2k_glyph::POINTS) as i32).max(0) as usize {
                    let (xa, ya) = (read_usize(comp + t2k_glyph::X) + 4 * k, read_usize(comp + t2k_glyph::Y) + 4 * k);
                    write_u32(xa, read_u32(xa).wrapping_add(dx));
                    write_u32(ya, read_u32(ya).wrapping_add(dy));
                }
            }
        }
        write_usize(base_at, comp);
        return;
    }
    let mem = read_usize(base);
    let (bp, cp) = (read_i16(base + t2k_glyph::POINTS) as i32, read_i16(comp + t2k_glyph::POINTS) as i32);
    let n = cp + bp;
    let contours = read_i16(comp + t2k_glyph::CONTOURS) as i32 + read_i16(base + t2k_glyph::CONTOURS) as i32;
    let (nn, nc) = (n as usize, contours as usize);
    let xs = lib_ctx_alloc(mem, n.wrapping_mul(0xd).wrapping_add(0x34) as u32);
    let ys = xs + 0x10 + 4 * nn;
    let oox = ys + 0x10 + 4 * nn;
    let ooy = oox + 8 + 2 * nn;
    let on = ooy + 8 + 2 * nn;
    let starts = lib_ctx_alloc(mem, contours.wrapping_mul(4) as u32);
    let ends = starts + 2 * nc;
    write_i32(base + t2k_glyph::ARRAY_POINTS, n as i16 as i32);
    write_i16(base + t2k_glyph::ARRAY_CONTOURS, contours as i16);
    let (bx, by) = (read_usize(base + t2k_glyph::X), read_usize(base + t2k_glyph::Y));
    for k in 0..bp.max(0) as usize {
        write_u32(xs + 4 * k, read_u32(bx + 4 * k));
        write_u32(ys + 4 * k, read_u32(by + 4 * k));
        write_u8(on + k, read_u8(read_usize(base + t2k_glyph::ON_CURVE) + k));
    }
    if flags & 0x200 == 0 {
        write_u32(xs + 4 * nn, read_u32(bx + 4 * bp as usize));
        write_u32(ys + 4 * nn, read_u32(by + 4 * bp as usize));
        write_u32(xs + 4 * nn + 4, read_u32(bx + 4 * bp as usize + 4));
        write_u32(ys + 4 * nn + 4, read_u32(by + 4 * bp as usize + 4));
    }
    let (dx, dy) = if flags & 2 == 0 {
        (read_u32(bx + 4 * arg1 as usize).wrapping_sub(read_u32(read_usize(comp + t2k_glyph::X) + 4 * arg2 as usize)),
         read_u32(by + 4 * arg1 as usize).wrapping_sub(read_u32(read_usize(comp + t2k_glyph::Y) + 4 * arg2 as usize)))
    } else if flags & 4 != 0 {
        (arg1.wrapping_add(0x20) & !0x3f, arg2.wrapping_add(0x20) & !0x3f)
    } else {
        (arg1, arg2)
    };
    for (j, k) in (bp..n).enumerate() {
        write_u32(xs + 4 * k as usize, read_u32(read_usize(comp + t2k_glyph::X) + 4 * j).wrapping_add(dx));
        write_u32(ys + 4 * k as usize, read_u32(read_usize(comp + t2k_glyph::Y) + 4 * j).wrapping_add(dy));
        write_u8(on + k as usize, read_u8(read_usize(comp + t2k_glyph::ON_CURVE) + j));
    }
    let base_contours = read_i16(base + t2k_glyph::CONTOURS) as i32;
    for k in 0..base_contours.max(0) as usize {
        write_i16(starts + 2 * k, read_i16(read_usize(base + t2k_glyph::STARTS) + 2 * k));
        write_i16(ends + 2 * k, read_i16(read_usize(base + t2k_glyph::ENDS) + 2 * k));
    }
    for (j, k) in (base_contours..contours).enumerate() {
        let shift = read_i16(base + t2k_glyph::POINTS);
        write_i16(starts + 2 * k as usize, read_i16(read_usize(comp + t2k_glyph::STARTS) + 2 * j).wrapping_add(shift));
        write_i16(ends + 2 * k as usize, read_i16(read_usize(comp + t2k_glyph::ENDS) + 2 * j).wrapping_add(shift));
    }
    write_i16(base + t2k_glyph::POINTS, n as i16);
    write_i16(base + t2k_glyph::CONTOURS, contours as i16);
    release_point_arrays(base);
    if read_usize(base + t2k_glyph::STARTS) != base + t2k_glyph::INLINE_STARTS {
        lib_ctx_free(read_usize(base), read_usize(base + t2k_glyph::STARTS));
    }
    write_usize(base + t2k_glyph::X, xs);
    write_usize(base + t2k_glyph::Y, ys);
    write_usize(base + t2k_glyph::OOX, oox);
    write_usize(base + t2k_glyph::OOY, ooy);
    write_usize(base + t2k_glyph::ON_CURVE, on);
    write_usize(base + t2k_glyph::STARTS, starts);
    write_usize(base + t2k_glyph::ENDS, ends);
    write_u16(base + t2k_glyph::CURVE_ORDER, read_u16(comp + t2k_glyph::CURVE_ORDER));
}

/// The glyph's point block: scratch slot 1 given back when it is that slot's buffer, else freed.
unsafe fn release_point_arrays(g: usize) {
    let mem = read_usize(g);
    if read_usize(g + t2k_glyph::X) == read_usize(mem + t2k_ctx::SCRATCH + 4) {
        write_u32(mem + t2k_ctx::SCRATCH_FREE + 4, 1);
    } else {
        lib_ctx_free(mem, read_usize(g + t2k_glyph::X));
    }
}

/// Points the glyph's arrays into one block for `n` points (scratch slot 1, `n` × 0xd + 0x34 bytes): x and y (26.6,
/// with four phantom points), the font unit x and y (16-bit) and the on curve flags; the array point count is `n`.
/// Original: `glyph_outline_alloc_point_arrays` (0x93c590, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_outline_alloc_point_arrays(g: usize, n: i32) {
    let block = lib_ctx_acquire_scratch_buffer(read_usize(g), n.wrapping_mul(0xd).wrapping_add(0x34) as u32, 1);
    let nn = n as usize;
    write_usize(g + t2k_glyph::X, block);
    let ys = block + 0x10 + 4 * nn;
    write_usize(g + t2k_glyph::Y, ys);
    let oox = ys + 0x10 + 4 * nn;
    write_i32(g + t2k_glyph::ARRAY_POINTS, n);
    write_usize(g + t2k_glyph::OOX, oox);
    let ooy = oox + 8 + 2 * nn;
    write_usize(g + t2k_glyph::OOY, ooy);
    write_usize(g + t2k_glyph::ON_CURVE, ooy + 8 + 2 * nn);
}

/// Frees a glyph: its point block and its instructions (each given back to its scratch slot, 1 and 3, when it is that
/// slot's buffer), its contour arrays when not inline, its component records, then the glyph (slot 0 given back when
/// it is that slot's buffer). Nothing for null.
/// Original: `glyph_outline_free` (0x93c5e0, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_outline_free(g: usize) {
    if g == 0 {
        return;
    }
    release_point_arrays(g);
    if read_usize(g + t2k_glyph::STARTS) != g + t2k_glyph::INLINE_STARTS {
        lib_ctx_free(read_usize(g), read_usize(g + t2k_glyph::STARTS));
    }
    let mem = read_usize(g);
    if read_usize(g + t2k_glyph::HINTS) == read_usize(mem + t2k_ctx::SCRATCH + 0xc) {
        write_u32(mem + t2k_ctx::SCRATCH_FREE + 0xc, 1);
    } else {
        lib_ctx_free(mem, read_usize(g + t2k_glyph::HINTS));
    }
    lib_ctx_free(read_usize(g), read_usize(g + t2k_glyph::COMPONENTS));
    let mem = read_usize(g);
    if g == read_usize(mem + t2k_ctx::SCRATCH) {
        write_u32(mem + t2k_ctx::SCRATCH_FREE, 1);
    } else {
        lib_ctx_free(mem, g);
    }
}

/// Restores the max-heap order of an array of record pointers below node `node` (to `last`), keyed by `keys[record's
/// first i16]`: the node sinks to a leaf along the larger children, then climbs back while larger than its parent.
/// Original: `glyph_hint_heap_sift_down_by_coord` (0x93d780, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_heap_sift_down_by_coord(records: usize, node: i32, last: i32, keys: usize) {
    let key = |k: i32| read_i16(keys.wrapping_add(2 * read_i16(read_usize(records + 4 * k as usize)) as usize));
    let swap = |a: i32, b: i32| {
        let t = read_u32(records + 4 * a as usize);
        write_u32(records + 4 * a as usize, read_u32(records + 4 * b as usize));
        write_u32(records + 4 * b as usize, t);
    };
    let mut k = node;
    loop {
        let mut child = k * 2 + 1;
        if last < child {
            break;
        }
        if k * 2 + 2 <= last && key(child) < key(k * 2 + 2) {
            child = k * 2 + 2;
        }
        swap(k, child);
        k = child;
    }
    loop {
        let parent = (k - 1) >> 1;
        if node > parent || parent == k {
            return;
        }
        if key(k) < key(parent) {
            return;
        }
        swap(k, parent);
        k = parent;
    }
}

/// The bold style's stroke offset: `fixed_mul(strength, units/12) − units/12` (the units per em divided by 12, signed).
unsafe fn embolden_offset(units: i16, strength: usize) -> i32 {
    let twelfth = (units / 12) as i32;
    fixed_mul(read_i32(strength), twelfth).wrapping_sub(twelfth)
}

/// Widens every advance width of a metrics table (+0x10, +4 the count) by twice the bold offset.
/// Original: `font_embolden_advance_widths` (0x93d820, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn font_embolden_advance_widths(m: usize, _mem: usize, units: i32, strength: usize) {
    let d = embolden_offset(units as i16, strength) as i16;
    for k in 0..read_i32(m + 4).max(0) as usize {
        let at = read_usize(m + 0x10) + 2 * k;
        write_i16(at, read_i16(at).wrapping_add(d.wrapping_mul(2)));
    }
}

/// Makes a glyph's strokes heavier by the bold offset `d` (nothing without contours or with `d` = 0): each point is
/// moved off its two edges by `d` (by the sign of each edge's y step) and put at the meeting of the two moved edges
/// (their middle when parallel), kept within `|2d|` of where it was (the distance measured as max + min/2); then a
/// point whose move reversed its order against its predecessor takes the predecessor's coordinate (x and y apart);
/// then runs of points with equal original x are averaged (the run from the contour's end wraps to its start).
/// Original: `glyph_outline_embolden` (0x93d870, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_outline_embolden(g: usize, mem: usize, units: i32, strength: usize) {
    let full = embolden_offset(units as i16, strength);
    let d = full as i16;
    let mut limit = full.wrapping_mul(2);
    if (limit as i16) < 0 {
        limit = limit.wrapping_neg();
    }
    let contours = read_i16(g + t2k_glyph::CONTOURS) as i32;
    if contours <= 0 || d == 0 {
        return;
    }
    let n = read_i16(read_usize(g + t2k_glyph::ENDS) + 2 * (contours - 1) as usize) as i32 + 1;
    let old_x = lib_ctx_alloc(mem, n.wrapping_mul(4) as u32);
    let old_y = old_x + 2 * n as usize;
    for k in 0..n.max(0) as usize {
        write_i16(old_x + 2 * k, read_i16(read_usize(g + t2k_glyph::OOX) + 2 * k));
        write_i16(old_y + 2 * k, read_i16(read_usize(g + t2k_glyph::OOY) + 2 * k));
    }
    for c in 0..read_i16(g + t2k_glyph::CONTOURS) as usize {
        embolden_contour(g, c, d, limit);
    }
    for c in 0..read_i16(g + t2k_glyph::CONTOURS) as usize {
        undo_reversed_moves(g, c, old_x, old_y);
    }
    for c in 0..read_i16(g + t2k_glyph::CONTOURS) as usize {
        average_equal_runs(g, c, old_x);
    }
    lib_ctx_free(mem, old_x);
}

/// The first pass of `glyph_outline_embolden` over one contour.
unsafe fn embolden_contour(g: usize, c: usize, d: i16, limit: i32) {
    let (xs, ys) = (read_usize(g + t2k_glyph::OOX), read_usize(g + t2k_glyph::OOY));
    let start = read_i16(read_usize(g + t2k_glyph::STARTS) + 2 * c) as i32;
    let end = read_i16(read_usize(g + t2k_glyph::ENDS) + 2 * c) as i32;
    let at = |a: usize, k: i32| read_i16(a + 2 * k as usize);
    let (first_x, first_y) = (at(xs, start), at(ys, start));
    let (mut prev_x, mut prev_y) = (at(xs, end), at(ys, end));
    let (mut cur_x, mut cur_y) = (first_x, first_y);
    for k in start..=end {
        let (next_x, next_y) = if k + 1 <= end { (at(xs, k + 1), at(ys, k + 1)) } else { (first_x, first_y) };
        let rising = prev_y.wrapping_sub(cur_y) < 0;
        let in_x = if prev_y == cur_y || rising {
            if rising {
                prev_x = prev_x.wrapping_sub(d);
                cur_x.wrapping_sub(d)
            } else {
                cur_x
            }
        } else {
            prev_x = prev_x.wrapping_add(d);
            cur_x.wrapping_add(d)
        };
        let rising = cur_y.wrapping_sub(next_y) < 0;
        let (out_x, out_next_x) = if cur_y == next_y || rising {
            if rising {
                (cur_x.wrapping_sub(d), next_x.wrapping_sub(d))
            } else {
                (cur_x, next_x)
            }
        } else {
            (cur_x.wrapping_add(d), next_x.wrapping_add(d))
        };
        if in_x == out_x {
            write_i16(xs + 2 * k as usize, in_x);
        } else {
            meet_moved_edges(g, k, [prev_x, prev_y, in_x, cur_x, cur_y, out_x, out_next_x, next_y], limit);
        }
        prev_x = cur_x;
        cur_x = next_x;
        prev_y = cur_y;
        cur_y = next_y;
    }
}

/// Puts point `k` at the meeting of the moved incoming edge (prev_x', prev_y)→(in_x, cur_y) and the moved outgoing
/// edge (out_x, cur_y)→(out_next_x, next_y), then keeps it within `limit` of (cur_x, cur_y).
unsafe fn meet_moved_edges(g: usize, k: i32, v: [i16; 8], limit: i32) {
    let [prev_x, prev_y, in_x, cur_x, cur_y, out_x, out_next_x, next_y] = v.map(|a| a as i32);
    let (xa, ya) = (read_usize(g + t2k_glyph::OOX) + 2 * k as usize, read_usize(g + t2k_glyph::OOY) + 2 * k as usize);
    let in_dx = in_x.wrapping_sub(prev_x);
    let in_dy = cur_y.wrapping_sub(prev_y);
    let det = out_next_x.wrapping_sub(out_x).wrapping_mul(in_dy).wrapping_sub(next_y.wrapping_sub(cur_y).wrapping_mul(in_dx));
    if det == 0 {
        write_i16(xa, (in_x.wrapping_add(out_x) / 2) as i16);
        write_i16(ya, (cur_y.wrapping_mul(2) / 2) as i16);
    } else {
        let t = fixed_div(in_dy.wrapping_mul(in_dx).wrapping_sub(out_x.wrapping_sub(prev_x).wrapping_mul(in_dy)), det);
        write_i16(xa, (fixed_mul(out_next_x.wrapping_sub(out_x), t) as i16).wrapping_add(out_x as i16));
        write_i16(ya, (fixed_mul(next_y.wrapping_sub(cur_y), t) as i16).wrapping_add(cur_y as i16));
    }
    let dx = (read_i16(xa) as i32).wrapping_sub(cur_x);
    let dy = (read_i16(ya) as i32).wrapping_sub(cur_y);
    let (ax, ay) = (if dx < 1 { dx.wrapping_neg() } else { dx }, if dy < 1 { dy.wrapping_neg() } else { dy });
    let length = if ay < ax { (ay >> 1).wrapping_add(ax) } else { (ax >> 1).wrapping_add(ay) };
    if limit < length {
        write_i16(xa, (limit.wrapping_mul(dx) / length) as i16 + cur_x as i16);
        write_i16(ya, (limit.wrapping_mul(dy) / length) as i16 + cur_y as i16);
    }
}

/// The second pass: a point whose move reversed its order against the previous point (original difference times new
/// difference negative) takes the previous point's coordinate, x and y apart; the contour's start follows its end.
unsafe fn undo_reversed_moves(g: usize, c: usize, old_x: usize, old_y: usize) {
    let start = read_i16(read_usize(g + t2k_glyph::STARTS) + 2 * c) as i32;
    let end = read_i16(read_usize(g + t2k_glyph::ENDS) + 2 * c) as i32;
    let mut prev = end as usize;
    for k in start..=end {
        let k = k as usize;
        for (now, old) in [(read_usize(g + t2k_glyph::OOX), old_x), (read_usize(g + t2k_glyph::OOY), old_y)] {
            let was = (read_i16(old + 2 * k) as i32).wrapping_sub(read_i16(old + 2 * prev) as i32);
            let is = (read_i16(now + 2 * k) as i32).wrapping_sub(read_i16(now + 2 * prev) as i32);
            if was.wrapping_mul(is) < 0 {
                write_i16(now + 2 * k, read_i16(now + 2 * prev));
            }
        }
        prev = k;
    }
}

/// The third pass over one contour, as the original walks it: from the contour's end as the run's first point, points
/// with the same original x as the run's first are summed; at the first different one a run of more than one point is
/// averaged (written from the run's first point and backwards from the point before the different one), and the walk
/// restarts after the lowest point written.
unsafe fn average_equal_runs(g: usize, c: usize, old_x: usize) {
    let start = read_i16(read_usize(g + t2k_glyph::STARTS) + 2 * c) as i32;
    let end = read_i16(read_usize(g + t2k_glyph::ENDS) + 2 * c) as i32;
    let xs = read_usize(g + t2k_glyph::OOX);
    let mut first = end;
    let mut sum = read_i16(xs + 2 * end as usize) as i32;
    let mut count = 1i32;
    let mut k = start;
    while k <= end {
        if read_i16(old_x + 2 * k as usize) == read_i16(old_x + 2 * first as usize) {
            sum = sum.wrapping_add(read_i16(xs + 2 * k as usize) as i32);
            count += 1;
        } else {
            if count > 1 {
                let average = (sum / count) as i16;
                k -= 1;
                write_i16(xs + 2 * first as usize, average);
                write_i16(xs + 2 * k as usize, average);
                for _ in 0..count - 2 {
                    k -= 1;
                    write_i16(xs + 2 * k as usize, average);
                }
            }
            first = k;
            count = 1;
            sum = read_i16(xs + 2 * k as usize) as i32;
        }
        k += 1;
    }
}


/// Comparers: side by side for the pure ones (the point step on the stream position and its two outputs; the heap
/// order on the record array; the widened advances); only the original for the rest (they allocate or free; proved on
/// the emulator).
pub(crate) unsafe extern "C" fn compare_glyph_outline_read_compressed_point_delta(s: usize, dx: usize, dy: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I0%%);
    let (r, c, same, _, _) = crate::comparison::side_by_side_light(&[(s + t2k_stream::POS, 4), (dx, 2), (dy, 2)],
        || o(s, dx, dy), || glyph_outline_read_compressed_point_delta(s, dx, dy));
    SCOREBOARD.record(same && r == c, || format!("glyph_outline_read_compressed_point_delta: original={} rust={} memory equal={}", r, c, same));
    r
}

pub(crate) unsafe extern "C" fn compare_glyph_outline_new_empty(mem: usize, lsb: i32, aw: i32, tsb: i32, ah: i32) -> usize {
    original_fn::<unsafe extern "C" fn(usize, i32, i32, i32, i32) -> usize>(%%I1%%)(mem, lsb, aw, tsb, ah)
}

pub(crate) unsafe extern "C" fn compare_glyph_outline_read_from_stream(mem: usize, s: usize, hints: i32, lsb: i32, aw: i32, tsb: i32, ah: i32, extra: usize) -> usize {
    original_fn::<unsafe extern "C" fn(usize, usize, i32, i32, i32, i32, i32, usize) -> usize>(%%I2%%)(mem, s, hints, lsb, aw, tsb, ah, extra)
}

pub(crate) unsafe extern "C" fn compare_glyph_outline_append_component(base_at: usize, comp: usize, flags: u32, arg1: u32, arg2: u32) {
    original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(%%I3%%)(base_at, comp, flags, arg1, arg2)
}

pub(crate) unsafe extern "C" fn compare_glyph_outline_alloc_point_arrays(g: usize, n: i32) {
    original_fn::<unsafe extern "C" fn(usize, i32)>(%%I4%%)(g, n)
}

pub(crate) unsafe extern "C" fn compare_glyph_outline_free(g: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I5%%)(g)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_heap_sift_down_by_coord(records: usize, node: i32, last: i32, keys: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32, usize)>(%%I6%%);
    let bytes = if (0..0x4000).contains(&last) { 4 * (last as usize + 1) } else { 0 };
    let (_, _, same, _, _) = crate::comparison::side_by_side(&[(records, bytes)], || o(records, node, last, keys),
        || glyph_hint_heap_sift_down_by_coord(records, node, last, keys));
    SCOREBOARD.record(same, || format!("glyph_hint_heap_sift_down_by_coord({}, {}) differs", node, last));
}

pub(crate) unsafe extern "C" fn compare_font_embolden_advance_widths(m: usize, mem: usize, units: i32, strength: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32, usize)>(%%I7%%);
    let n = read_i32(m + 4);
    let bytes = if (0..0x10000).contains(&n) { 2 * n as usize } else { 0 };
    let (_, _, same, _, _) = crate::comparison::side_by_side(&[(read_usize(m + 0x10), bytes)], || o(m, mem, units, strength),
        || font_embolden_advance_widths(m, mem, units, strength));
    SCOREBOARD.record(same, || "font_embolden_advance_widths differs".to_string());
}

pub(crate) unsafe extern "C" fn compare_glyph_outline_embolden(g: usize, mem: usize, units: i32, strength: usize) {
    original_fn::<unsafe extern "C" fn(usize, usize, i32, usize)>(%%I8%%)(g, mem, units, strength)
}
