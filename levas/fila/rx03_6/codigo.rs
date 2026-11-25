// ---------- rx03_6: the T2K TrueType font: the character map, the glyph loading and the font tables (library_without_asserts, T2K truetype.c) ----------

// The character map (0x2c bytes): +0 the memory context, +4 the version, +6 the subtable count, +8 the subtable
// records (8 bytes: platform, encoding, offset), +0xc the cmap data and +0x10 its size, +0x14 the chosen subtable,
// +0x16 its format (0, 4 or 6), +0x18 the glyph indexes of the digits 0..9 (0xffff for none).

/// The error of a character map in another format.
const T2K_ERR_BAD_CMAP_FORMAT: i32 = 0x2725;
/// The head table's glyph data format of the T2K compressed glyphs (.t2k fonts).
const GLYPH_FORMAT_T2K: i16 = 2000;

/// A big-endian 16-bit value at `at`.
unsafe fn be16(at: usize) -> u16 {
    u16::from_be(read_u16(at))
}

/// The glyph index of a character in a cmap subtable of format 0 (bytes, codes below 0x100), 6 (a dense range) or 4
/// (segments: the first end code not below the character, then a delta or an index through the range offset);
/// 0 when not mapped. `table` points at the subtable's length field (+2 of its start).
unsafe fn cmap_glyph_index(format: i16, table: usize, c: u16) -> u16 {
    match format {
        0 => {
            if c < 0x100 {
                read_u8(table + 4 + c as usize) as u16
            } else {
                0
            }
        }
        6 => {
            let k = c.wrapping_sub(be16(table + 4));
            if k < be16(table + 6) {
                be16(table + 8 + 2 * k as usize)
            } else {
                0
            }
        }
        4 => {
            let seg2 = be16(table + 4) as usize;
            let mut end = table + 0xc;
            while be16(end) < c {
                end += 2;
            }
            let start = end + 2 + seg2;
            if be16(start) > c {
                return 0;
            }
            let delta = start + seg2;
            let range = delta + seg2;
            let offset = be16(range);
            if offset == 0 {
                return c.wrapping_add(be16(delta));
            }
            let k = c.wrapping_sub(be16(start)) as usize;
            be16(range + 2 * k + offset as usize).wrapping_add(be16(delta))
        }
        _ => 0,
    }
}

/// The chosen subtable of a character map, at its length field.
unsafe fn cmap_subtable(c: usize) -> usize {
    let record = read_usize(read_usize(c + 8) + 4 * read_i16(c + 0x14) as usize);
    read_usize(record + 4).wrapping_add(read_usize(c + 0xc)).wrapping_add(2)
}

/// Loads the character map on first use (`tt_cmap_load` over the cmap table, with the platform and encoding asked
/// for at +0x30 and +0x32).
unsafe fn tt_font_cmap(font: usize) -> usize {
    if read_u32(font + tt_font::CMAP) == 0 {
        let entry = tt_find_table_by_tag(font, TAG_CMAP);
        let s = t2k_stream_open(read_usize(font + tt_font::MEM), read_usize(font + tt_font::STREAM), read_u32(entry + 0xc), read_u32(entry + 0x10));
        let c = tt_cmap_load(read_usize(font + tt_font::MEM), read_i16(font + tt_font::CMAP_PLATFORM) as i32,
            read_u16(font + tt_font::CMAP_ENCODING) as i32, s);
        write_usize(font + tt_font::CMAP, c);
        t2k_stream_close(s);
    }
    read_usize(font + tt_font::CMAP)
}

/// Maps a character code to a glyph index (the low 16 bits of `c`) through the character map, loaded on first use
/// (callers read the low 16 bits of the result).
/// Original: `tt_font_cmap_lookup` (0x942a40, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_cmap_lookup(font: usize, c: u32) -> u16 {
    let cmap = tt_font_cmap(font);
    cmap_glyph_index(read_i16(cmap + 0x16), cmap_subtable(cmap), c as u16)
}

/// Parses a cmap table: the version, the subtable records (platform, encoding, 32-bit offset kept to 16 bits), the
/// whole table copied in a block; then the subtable chosen in four passes: the platform and encoding asked for (−1
/// matching any) among formats 0, 4 and 6, else the first of format 0, else of format 6, else of format 4 (the first
/// subtable when none matches); another format raises 0x2725. Then the glyph indexes of the digits '0'..'9' are cached
/// (0xffff when unmapped).
/// Original: `tt_cmap_load` (0x942c00, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_cmap_load(mem: usize, platform: i32, encoding: i32, s: usize) -> usize {
    let (platform, encoding) = (platform as i16, encoding as i16);
    let c = lib_ctx_alloc(mem, 0x2c);
    write_usize(c, mem);
    write_i16(c + 4, input_stream_read_int16(s));
    let n = input_stream_read_int16(s);
    write_i16(c + 6, n);
    write_usize(c + 8, lib_ctx_alloc(mem, (n as i32).wrapping_shl(2) as u32));
    let mut k = 0;
    while k < read_i16(c + 6) as i32 {
        let record = lib_ctx_alloc(mem, 8);
        write_usize(read_usize(c + 8) + 4 * k as usize, record);
        write_i16(record, input_stream_read_int16(s));
        write_i16(record + 2, input_stream_read_int16(s));
        write_u32(record + 4, input_stream_read_int32(s) & 0xffff);
        k += 1;
    }
    input_stream_rewind(s);
    let size = input_stream_get_size(s);
    write_i32(c + 0x10, size);
    let data = lib_ctx_alloc(mem, size as u32);
    write_usize(c + 0xc, data);
    input_stream_read_bytes(s, data, read_i32(c + 0x10));
    write_i16(c + 0x14, 0);
    choose_cmap_subtable(c, platform, encoding);
    let format = be16(cmap_subtable(c) - 2);
    write_i16(c + 0x16, format as i16);
    if format != 0 && format != 6 && format != 4 {
        lib_ctx_longjmp_error(mem, T2K_ERR_BAD_CMAP_FORMAT);
    }
    for digit in 0..10u16 {
        let g = cmap_glyph_index(read_i16(c + 0x16), cmap_subtable(c), b'0' as u16 + digit);
        write_u16(c + 0x18 + 2 * digit as usize, if g == 0 { 0xffff } else { g });
    }
    c
}

/// The four passes over the subtables (see `tt_cmap_load`); the chosen index goes to +0x14.
unsafe fn choose_cmap_subtable(c: usize, platform: i16, encoding: i16) {
    for pass in 0..4 {
        for k in 0..(read_i16(c + 6) as i32).max(0) {
            let record = read_usize(read_usize(c + 8) + 4 * k as usize);
            let format = be16(read_usize(record + 4).wrapping_add(read_usize(c + 0xc))) as i16;
            let found = match pass {
                0 => (format == 0 || format == 4 || format == 6) && platform_matches(record, platform, encoding),
                1 => format == 0,
                2 => format == 6,
                _ => format == 4,
            };
            if found {
                write_i16(c + 0x14, k as i16);
                return;
            }
        }
    }
}

/// The record's platform and encoding against the ones asked for: both equal, or the unequal one asked as −1.
unsafe fn platform_matches(record: usize, platform: i16, encoding: i16) -> bool {
    let (p, e) = (read_i16(record), read_i16(record + 2));
    if p == platform {
        e == encoding || encoding == -1
    } else {
        e == encoding && platform == -1
    }
}

/// Whether a glyph index is one of the ten digit glyphs (the character map loaded first if needed).
/// Original: `tt_font_is_digit_glyph` (0x942f50, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_is_digit_glyph(font: usize, glyph: i32) -> i32 {
    let cmap = tt_font_cmap(font);
    (0..10).any(|k| read_i16(cmap + 0x18 + 2 * k) == glyph as i16) as i32
}

/// Loads glyph `index`: with a glyf table, loca, horizontal or vertical metrics and the index below the glyph count,
/// the advance and side bearing (hmtx, else 0x800 and 0), the vertical advance and top bearing (vmtx, else the hhea
/// line height and a tenth of it, else 0), then the outline from the glyf bytes (T2K compressed when head's glyph data
/// format is 2000, else TrueType) and the font's style function on it, or an empty glyph when the glyph has no bytes;
/// otherwise an empty glyph with no metrics. The advances go to `*aw` and `*ah`; the glyph keeps its index (+0x71).
/// Original: `tt_font_load_glyph` (0x942350, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_load_glyph(font: usize, index: i32, read_hints: i32, aw: usize, ah: usize) -> usize {
    write_u16(aw, 0);
    write_u16(ah, 0);
    let mem = read_usize(font + tt_font::MEM);
    let glyf = tt_find_table_by_tag(font, TAG_GLYF);
    let (loca, hmtx, vmtx) = (read_usize(font + tt_font::LOCA), read_usize(font + tt_font::HMTX), read_usize(font + tt_font::VMTX));
    if glyf == 0 || loca == 0 || (hmtx == 0 && vmtx == 0) || index < 0 || index >= read_i32(font + 0x64) {
        let g = glyph_outline_new_empty(mem, 0, 0, 0, 0);
        write_i16(g + t2k_glyph::INDEX, index as i16);
        return g;
    }
    let offsets = read_usize(loca + 4);
    let (from, to) = (read_u32(offsets + 4 * index as usize), read_u32(offsets + 4 * index as usize + 4));
    let (advance, bearing) = if hmtx != 0 {
        (read_u16(read_usize(hmtx + 0x10) + 2 * index as usize), read_u16(read_usize(hmtx + 0xc) + 2 * index as usize))
    } else {
        (0x800, 0)
    };
    let (mut height, mut top) = (0u16, 0u32);
    if vmtx != 0 {
        height = read_u16(read_usize(vmtx + 0x10) + 2 * index as usize);
        top = read_u16(read_usize(vmtx + 0xc) + 2 * index as usize) as u32;
    } else {
        let hhea = read_usize(font + tt_font::HHEA);
        if hhea != 0 {
            height = read_u16(hhea + 0xc).wrapping_sub(read_u16(hhea + 0xa)).wrapping_add(read_u16(hhea + 8));
            top = (height / 10) as u32;
        }
    }
    let g = if to > from {
        let s = t2k_stream_open_scratch(mem, read_usize(font + tt_font::STREAM), read_u32(glyf + 0xc).wrapping_add(from), to - from);
        let g = if read_i16(read_usize(font + tt_font::HEAD) + 0x38) == GLYPH_FORMAT_T2K {
            glyph_outline_read_from_stream(mem, s, read_hints, bearing as i32, advance as i32, top as i32, height as i32, read_usize(font + 0x60))
        } else {
            tt_glyph_decode_outline(mem, s, read_hints, bearing as i32, advance as i32, top as i32, height as i32)
        };
        t2k_stream_close(s);
        let style = read_usize(font + 0x3c);
        if style != 0 {
            let f: unsafe extern "C" fn(usize, usize, u32, usize) = core::mem::transmute(style);
            f(g, mem, tt_font_get_units_per_em(font), font + tt_font::STYLE_PARAMS);
        }
        g
    } else {
        glyph_outline_new_empty(mem, bearing as i32, advance as i32, top as i32, height as i32)
    };
    write_u16(aw, advance);
    write_u16(ah, height);
    write_i16(g + t2k_glyph::INDEX, index as i16);
    g
}

/// `input_stream_new` (0x941630, cdecl) with the record from scratch slot 2 (the glyph's bytes).
unsafe fn t2k_stream_open_scratch(mem: usize, source: usize, offset: u32, length: u32) -> usize {
    let f: unsafe extern "C" fn(usize, usize, u32, u32, i32, usize) -> usize = core::mem::transmute(FN_INPUT_STREAM_NEW);
    f(mem, source, offset, length, 2, 0)
}

/// Decodes one TrueType glyf record into a glyph (scratch slot 0): the contour count and the bounding box; a simple
/// glyph reads its contour ends (inline room up to 8 contours), its instructions (kept in scratch slot 3 when
/// `read_hints`, else skipped), its flags (with repeats, cut at the point count) and its x and y deltas (short or
/// word, by the flags), the flags then kept to their on curve bit; a composite glyph keeps its component records
/// (16-bit words: flags, glyph, the arguments by size, the scale, xy or 2×2 by flags) in a growing block, then its
/// instructions when flag 0x100 is set. The four phantom points follow the outline: x min − side bearing, plus the
/// advance; the middle of those two and y max + top bearing, minus the vertical advance. A glyph with no contours
/// returns with only the point count set (no arrays).
/// Original: `tt_glyph_decode_outline` (0x942560, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_glyph_decode_outline(mem: usize, s: usize, read_hints: i32, lsb: i32, aw: i32, tsb: i32, ah: i32) -> usize {
    let g = lib_ctx_acquire_scratch_buffer(mem, t2k_glyph::SIZE, 0);
    write_usize(g, mem);
    for field in [t2k_glyph::ENDS, t2k_glyph::STARTS, t2k_glyph::COMPONENTS, t2k_glyph::Y, t2k_glyph::X] {
        write_u32(g + field, 0);
    }
    let contours = input_stream_read_int16(s);
    write_i16(g + t2k_glyph::CONTOURS, contours);
    for field in [t2k_glyph::X_MIN, t2k_glyph::Y_MIN, t2k_glyph::X_MAX, t2k_glyph::Y_MAX] {
        write_i16(g + field, input_stream_read_int16(s));
    }
    for field in [t2k_glyph::COMPONENT_COUNT, t2k_glyph::HINT_LENGTH, t2k_glyph::HINTS, 0xa, 0xe] {
        write_u32(g + field, 0);
    }
    write_i16(g + t2k_glyph::CURVE_ORDER, 2);
    write_i16(g + t2k_glyph::ARRAY_CONTOURS, 0);
    write_u32(g + t2k_glyph::ARRAY_POINTS, 0);
    let points;
    if contours < 0 {
        let (data, n, last_flags) = read_components(g, s);
        write_u32(g + t2k_glyph::HINT_LENGTH, 0);
        if last_flags & 0x100 != 0 {
            read_instructions(g, s, read_hints);
        }
        glyph_outline_alloc_point_arrays(g, 0);
        write_usize(g + t2k_glyph::COMPONENTS, data);
        write_i32(g + t2k_glyph::COMPONENT_COUNT, n);
        points = 0;
    } else if contours == 0 {
        write_i16(g + t2k_glyph::POINTS, 0);
        return g;
    } else {
        points = read_simple_outline(g, s, contours as i32, read_hints);
    }
    write_phantom_points(g, read_usize(g + t2k_glyph::OOX), read_usize(g + t2k_glyph::OOY), points, lsb, aw, tsb, ah);
    write_i16(g + t2k_glyph::POINTS, points as i16);
    g
}

/// The glyph's instructions: their length (16-bit), kept in scratch slot 3 when `read_hints` and positive, else the
/// stream skips them (the length then 0).
unsafe fn read_instructions(g: usize, s: usize, read_hints: i32) {
    let length = input_stream_read_int16(s) as i32;
    write_i32(g + t2k_glyph::HINT_LENGTH, length);
    if read_hints as u8 == 0 {
        write_u32(s + t2k_stream::POS, read_u32(s + t2k_stream::POS).wrapping_add(length as u32));
        write_i32(g + t2k_glyph::HINT_LENGTH, 0);
    } else if length > 0 {
        let code = lib_ctx_acquire_scratch_buffer(read_usize(g), length as u32, 3);
        write_usize(g + t2k_glyph::HINTS, code);
        input_stream_read_bytes(s, code, read_i32(g + t2k_glyph::HINT_LENGTH));
    }
}

/// A simple glyph's outline (see `tt_glyph_decode_outline`); returns the point count.
unsafe fn read_simple_outline(g: usize, s: usize, contours: i32, read_hints: i32) -> i32 {
    if contours < 9 {
        write_usize(g + t2k_glyph::STARTS, g + t2k_glyph::INLINE_STARTS);
        write_usize(g + t2k_glyph::ENDS, g + t2k_glyph::INLINE_ENDS);
    } else {
        let block = lib_ctx_alloc(read_usize(g), contours.wrapping_mul(4) as u32);
        write_usize(g + t2k_glyph::STARTS, block);
        write_usize(g + t2k_glyph::ENDS, block + 2 * contours as usize);
    }
    let mut next = 0i16;
    for k in 0..contours as usize {
        write_i16(read_usize(g + t2k_glyph::STARTS) + 2 * k, next);
        let end = input_stream_read_int16(s);
        write_i16(read_usize(g + t2k_glyph::ENDS) + 2 * k, end);
        next = end.wrapping_add(1);
    }
    let points = next as i32;
    read_instructions(g, s, read_hints);
    glyph_outline_alloc_point_arrays(g, points);
    write_i16(g + t2k_glyph::ARRAY_CONTOURS, contours as i16);
    write_i32(g + t2k_glyph::ARRAY_POINTS, points);
    let flags = read_usize(g + t2k_glyph::ON_CURVE);
    let take = || {
        let at = read_usize(s + t2k_stream::DATA).wrapping_add(read_u32(s + t2k_stream::POS) as usize);
        write_u32(s + t2k_stream::POS, read_u32(s + t2k_stream::POS).wrapping_add(1));
        read_u8(at)
    };
    let mut k = 0;
    while k < points {
        let f = take();
        write_u8(flags + k as usize, f);
        k += 1;
        if f & 8 != 0 {
            let mut repeat = take() as i32;
            while repeat > 0 {
                repeat -= 1;
                if k >= points {
                    break;
                }
                write_u8(flags + k as usize, f);
                k += 1;
            }
        }
    }
    let word = || {
        let hi = take() as u16;
        let lo = take() as u16;
        (hi << 8 | lo) as i16
    };
    let mut x = 0i16;
    for k in 0..points as usize {
        let f = read_u8(flags + k);
        if f & 2 != 0 {
            let d = take() as i16;
            x = if f & 0x10 != 0 { x.wrapping_add(d) } else { x.wrapping_sub(d) };
        } else if f & 0x10 == 0 {
            x = x.wrapping_add(word());
        }
        write_i16(read_usize(g + t2k_glyph::OOX) + 2 * k, x);
    }
    let mut y = 0i16;
    for k in 0..points as usize {
        let f = read_u8(flags + k);
        write_u8(flags + k, f & 1);
        if f & 4 != 0 {
            let d = take() as i16;
            y = if f & 0x20 != 0 { y.wrapping_add(d) } else { y.wrapping_sub(d) };
        } else if f & 0x20 == 0 {
            y = y.wrapping_add(word());
        }
        write_i16(read_usize(g + t2k_glyph::OOY) + 2 * k, y);
    }
    points
}

/// The median over a string (at most 32 characters, to its zero byte) of each glyph's lowest y (`highest` = 0) or
/// highest y (else) over its contours of more than one point (a composite glyph is replaced by its first component);
/// glyphs with contours and points count (an empty pass leaves 0x7fff or −0x8000); 0 when none.
/// Original: `glyph_hint_median_char_extreme_y` (0x93ba30, library_without_asserts, T2K glyph.c, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_median_char_extreme_y(font: usize, text: usize, highest: i32) -> i16 {
    let mut values = [0i16; 32];
    let mut n = 0usize;
    let (mut aw, mut ah) = (0u16, 0u16);
    for k in 0..32 {
        let c = read_u8(text + k);
        if c == 0 {
            break;
        }
        let index = tt_font_cmap_lookup(font, c as u32);
        let mut g = tt_font_load_glyph(font, index as i32, 0, &mut ah as *mut u16 as usize, &mut aw as *mut u16 as usize);
        if read_i16(g + t2k_glyph::CONTOURS) < 0 {
            let first = read_u16(read_usize(g + t2k_glyph::COMPONENTS) + 2);
            glyph_outline_free(g);
            g = tt_font_load_glyph(font, first as i32, 0, &mut ah as *mut u16 as usize, &mut aw as *mut u16 as usize);
        }
        let contours = read_i16(g + t2k_glyph::CONTOURS);
        if contours != 0 && read_i16(g + t2k_glyph::POINTS) > 0 {
            values[n] = contour_extreme_y(g, contours, highest as i16 != 0);
            n += 1;
        }
        glyph_outline_free(g);
    }
    if n == 0 {
        return 0;
    }
    sort_int16_array_ascending(values.as_mut_ptr() as usize, n as i32);
    values[n >> 1]
}

/// The lowest (or highest) font unit y over the contours whose start is below their end.
unsafe fn contour_extreme_y(g: usize, contours: i16, highest: bool) -> i16 {
    let mut best = if highest { -0x8000i16 } else { 0x7fff };
    for c in 0..(contours as i32).max(0) as usize {
        let start = read_i16(read_usize(g + t2k_glyph::STARTS) + 2 * c) as i32;
        let end = read_i16(read_usize(g + t2k_glyph::ENDS) + 2 * c) as i32;
        if start < end {
            for k in start..=end {
                let y = read_i16(read_usize(g + t2k_glyph::OOY) + 2 * k as usize);
                if (highest && best < y) || (!highest && y < best) {
                    best = y;
                }
            }
        }
    }
    best
}

//// Reads the font's tables from the font file stream (TrueType collection entry `ttc_index` when the file starts with
/// 'ttcf'), first freeing the tables of an earlier read: the directory (its entries), head, hhea, vhea, maxp, loca
/// (16-bit offsets doubled or 32-bit, by head's index format), hmtx and vmtx (with their headers; the horizontal
/// metrics then go through the style's metrics function), kern (not with an 'fvar' table), the hinting context
/// (`tt_exec_create`) and post's underline position and thickness (0 without post).
/// Original: `tt_font_load_tables` (0x9430e0, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_load_tables(font: usize, s: usize, ttc_index: i32) {
    free_font_tables(font);
    let mem = read_usize(font + tt_font::MEM);
    let mut ttc = 0;
    if input_stream_read_int32(s) == TAG_TTCF {
        ttc = lib_ctx_alloc(mem, 0x10);
        write_usize(ttc, mem);
        write_u32(ttc + 4, input_stream_read_int32(s));
        let n = input_stream_read_int32(s);
        write_u32(ttc + 8, n);
        write_usize(ttc + 0xc, lib_ctx_alloc(mem, n.wrapping_mul(4)));
        let mut k = 0u32;
        while k < read_u32(ttc + 8) {
            write_u32(read_usize(ttc + 0xc) + 4 * k as usize, input_stream_read_int32(s));
            k += 1;
        }
    }
    input_stream_rewind(s);
    write_usize(font + tt_font::COLLECTION, ttc);
    if ttc != 0 {
        input_stream_seek(s, read_u32(read_usize(ttc + 0xc) + 4 * ttc_index as usize));
    }
    read_table_directory(font, s);
    let head = tt_find_table_by_tag(font, TAG_HEAD);
    if head != 0 {
        let t = t2k_stream_open(read_usize(font + tt_font::MEM), s, read_u32(head + 0xc), read_u32(head + 0x10));
        let h = lib_ctx_alloc(read_usize(font + tt_font::MEM), 0x3a);
        write_usize(h, read_usize(font + tt_font::MEM));
        read_fields(t, h + 4, &[4, 4, 4, 4, 2, 2, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
        write_usize(font + tt_font::HEAD, h);
        t2k_stream_close(t);
    }
    for (tag, field) in [(TAG_HHEA, tt_font::HHEA), (TAG_VHEA, tt_font::VHEA)] {
        let e = tt_find_table_by_tag(font, tag);
        if e != 0 {
            let t = t2k_stream_open(read_usize(font + tt_font::MEM), s, read_u32(e + 0xc), read_u32(e + 0x10));
            write_usize(font + field, tt_read_hhea_or_vhea(read_usize(font + tt_font::MEM), t));
            t2k_stream_close(t);
        }
    }
    let e = tt_find_table_by_tag(font, TAG_MAXP);
    let t = t2k_stream_open(read_usize(font + tt_font::MEM), s, read_u32(e + 0xc), read_u32(e + 0x10));
    let m = lib_ctx_alloc(read_usize(font + tt_font::MEM), 0x24);
    write_usize(m, read_usize(font + tt_font::MEM));
    read_fields(t, m + 4, &[4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
    write_usize(font + tt_font::MAXP, m);
    t2k_stream_close(t);
    read_loca(font, s);
    read_metrics(font, s, TAG_HMTX, tt_font::HHEA, tt_font::HMTX);
    read_metrics(font, s, TAG_VMTX, tt_font::VHEA, tt_font::VMTX);
    let mut kern = tt_find_table_by_tag(font, TAG_KERN);
    if kern != 0 && tt_find_table_by_tag(font, TAG_FVAR) != 0 {
        kern = 0;
    }
    write_u32(font + tt_font::KERN, 0);
    if kern != 0 {
        let t = t2k_stream_open(read_usize(font + tt_font::MEM), s, read_u32(kern + 0xc), read_u32(kern + 0x10));
        write_usize(font + tt_font::KERN, tt_read_kern_table(read_usize(font + tt_font::MEM), t));
        t2k_stream_close(t);
    }
    write_usize(font + tt_font::EXEC, tt_exec_create(read_usize(font + tt_font::MEM), s, font));
    let post = tt_find_table_by_tag(font, TAG_POST);
    if post == 0 {
        write_u16(font + tt_font::UNDERLINE_POSITION, 0);
        write_u16(font + tt_font::UNDERLINE_THICKNESS, 0);
        return;
    }
    input_stream_seek(s, read_u32(post + 0xc).wrapping_add(8));
    write_i16(font + tt_font::UNDERLINE_POSITION, input_stream_read_int16(s));
    write_i16(font + tt_font::UNDERLINE_THICKNESS, input_stream_read_int16(s));
}

/// Big-endian fields of 4 or 2 bytes read one after another into `at`.
unsafe fn read_fields(s: usize, mut at: usize, sizes: &[usize]) {
    for &size in sizes {
        if size == 4 {
            write_u32(at, input_stream_read_int32(s));
        } else {
            write_i16(at, input_stream_read_int16(s));
        }
        at += size;
    }
}

/// The table directory (a 0x14-byte block: the version, the table count and the search fields, the entries).
unsafe fn read_table_directory(font: usize, s: usize) {
    let mem = read_usize(font + tt_font::MEM);
    let d = lib_ctx_alloc(mem, 0x14);
    write_usize(d, mem);
    read_fields(s, d + 4, &[4, 2, 2, 2, 2]);
    write_usize(d + 0x10, lib_ctx_alloc(mem, (read_i16(d + 8) as i32).wrapping_shl(2) as u32));
    let mut k = 0;
    while k < read_i16(d + 8) as i32 {
        write_usize(read_usize(d + 0x10) + 4 * k as usize, tt_read_table_directory_entry(mem, s));
        k += 1;
    }
    write_usize(font + tt_font::DIRECTORY, d);
}

/// The loca table (a 0xe-byte block: +4 the offsets, +8 their count = the table length >> (format + 1), +0xc the
/// format): 32-bit offsets for format 1, 16-bit ones doubled for format 0.
unsafe fn read_loca(font: usize, s: usize) {
    let e = tt_find_table_by_tag(font, TAG_LOCA);
    if e == 0 {
        return;
    }
    let t = t2k_stream_open(read_usize(font + tt_font::MEM), s, read_u32(e + 0xc), read_u32(e + 0x10));
    let mem = read_usize(font + tt_font::MEM);
    let format = read_i16(read_usize(font + tt_font::HEAD) + 0x36);
    let l = lib_ctx_alloc(mem, 0xe);
    let n = read_i32(e + 0x10) >> ((format as u32).wrapping_add(1) & 0x1f);
    write_usize(l, mem);
    write_i16(l + 0xc, format);
    write_i32(l + 8, n);
    write_usize(l + 4, lib_ctx_alloc(mem, n.wrapping_mul(4) as u32));
    for k in 0..n.max(0) as usize {
        match format {
            1 => write_u32(read_usize(l + 4) + 4 * k, input_stream_read_int32(t)),
            0 => write_u32(read_usize(l + 4) + 4 * k, (input_stream_read_int16(t) as u16 as u32) << 1),
            _ => break,
        }
    }
    write_usize(font + tt_font::LOCA, l);
    t2k_stream_close(t);
}

/// A metrics table (hmtx or vmtx) when it and its header exist: the glyph count (`tt_font_get_glyph_count`) and the
/// header's metrics count; the horizontal one then goes through the style's metrics function (+0x40), with the units
/// per em and the style parameters.
unsafe fn read_metrics(font: usize, s: usize, tag: u32, header: usize, field: usize) {
    let e = tt_find_table_by_tag(font, tag);
    if e == 0 || read_u32(font + header) == 0 {
        return;
    }
    let t = t2k_stream_open(read_usize(font + tt_font::MEM), s, read_u32(e + 0xc), read_u32(e + 0x10));
    let glyphs = tt_font_get_glyph_count(font);
    let metrics = read_u16(read_usize(font + header) + 0x26) as i32;
    write_usize(font + field, tt_read_hmtx_or_vmtx(read_usize(font + tt_font::MEM), t, glyphs, metrics));
    t2k_stream_close(t);
    let style = read_usize(font + tt_font::STYLE_METRICS_FN);
    if field == tt_font::HMTX && style != 0 {
        let f: unsafe extern "C" fn(usize, usize, u32, usize) = core::mem::transmute(style);
        f(read_usize(font + tt_font::HMTX), read_usize(font + tt_font::MEM), tt_font_get_units_per_em(font), font + tt_font::STYLE_PARAMS);
    }
}

/// The tables of an earlier read freed (see `tt_font_load_tables`).
unsafe fn free_font_tables(font: usize) {
    let free = |block: usize, field: usize| lib_ctx_free(read_usize(block), read_usize(block + field));
    let ttc = read_usize(font + tt_font::COLLECTION);
    if ttc != 0 {
        free(ttc, 0xc);
        lib_ctx_free(read_usize(ttc), ttc);
    }
    let d = read_usize(font + tt_font::DIRECTORY);
    if d != 0 {
        let mut k = 0;
        while k < read_i16(d + 8) as i32 {
            let e = read_usize(read_usize(d + 0x10) + 4 * k as usize);
            lib_ctx_free(read_usize(e), e);
            k += 1;
        }
        free(d, 0x10);
        lib_ctx_free(read_usize(d), d);
    }
    for field in [tt_font::HEAD, tt_font::HHEA, tt_font::VHEA] {
        let t = read_usize(font + field);
        if t != 0 {
            lib_ctx_free(read_usize(t), t);
        }
        write_u32(font + field, 0);
    }
    for field in [tt_font::HMTX, tt_font::VMTX] {
        let m = read_usize(font + field);
        if m != 0 {
            free(m, 0xc);
            free(m, 0x10);
            lib_ctx_free(read_usize(m), m);
        }
        write_u32(font + field, 0);
    }
    let maxp = read_usize(font + tt_font::MAXP);
    if maxp != 0 {
        lib_ctx_free(read_usize(maxp), maxp);
    }
    let loca = read_usize(font + tt_font::LOCA);
    if loca != 0 {
        free(loca, 4);
        lib_ctx_free(read_usize(loca), loca);
    }
    write_u32(font + tt_font::LOCA, 0);
    let kern = read_usize(font + tt_font::KERN);
    if kern != 0 {
        let mut k = 0;
        while k < read_i32(kern + 6) {
            let sub = read_usize(read_usize(kern + 0xa) + 4 * k as usize);
            if read_i16(sub + 4) == 0 {
                tt_kern_free_format0_table(read_usize(sub + 0xc));
            }
            lib_ctx_free(read_usize(sub), sub);
            k += 1;
        }
        free(kern, 0xa);
        lib_ctx_free(read_usize(kern), kern);
    }
}

// Comparers: value against value for the lookups (the low 16 bits; the character map is loaded first by the
/// original); only the original for the loaders (they allocate; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_font_cmap_lookup(font: usize, c: u32) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u16>(%%I0%%)(font, c);
    let copy = tt_font_cmap_lookup(font, c);
    SCOREBOARD.record(r == copy, || format!("tt_font_cmap_lookup({:#x}) original={} rust={}", c, r, copy));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_cmap_load(mem: usize, platform: i32, encoding: i32, s: usize) -> usize {
    original_fn::<unsafe extern "C" fn(usize, i32, i32, usize) -> usize>(%%I1%%)(mem, platform, encoding, s)
}

pub(crate) unsafe extern "C" fn compare_tt_font_is_digit_glyph(font: usize, glyph: i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, i32) -> i32>(%%I2%%)(font, glyph);
    let copy = tt_font_is_digit_glyph(font, glyph);
    SCOREBOARD.record(r == copy, || format!("tt_font_is_digit_glyph({}) original={} rust={}", glyph, r, copy));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_font_load_glyph(font: usize, index: i32, read_hints: i32, aw: usize, ah: usize) -> usize {
    original_fn::<unsafe extern "C" fn(usize, i32, i32, usize, usize) -> usize>(%%I3%%)(font, index, read_hints, aw, ah)
}

pub(crate) unsafe extern "C" fn compare_tt_glyph_decode_outline(mem: usize, s: usize, read_hints: i32, lsb: i32, aw: i32, tsb: i32, ah: i32) -> usize {
    original_fn::<unsafe extern "C" fn(usize, usize, i32, i32, i32, i32, i32) -> usize>(%%I4%%)(mem, s, read_hints, lsb, aw, tsb, ah)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_median_char_extreme_y(font: usize, text: usize, highest: i32) -> i16 {
    original_fn::<unsafe extern "C" fn(usize, usize, i32) -> i16>(%%I5%%)(font, text, highest)
}

pub(crate) unsafe extern "C" fn compare_tt_font_load_tables(font: usize, s: usize, ttc_index: i32) {
    original_fn::<unsafe extern "C" fn(usize, usize, i32)>(%%I6%%)(font, s, ttc_index)
}
