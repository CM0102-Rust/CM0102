// ---------- rx03_4: the T2K TrueType font: the table directory, the metrics headers, the kerning and the small getters (library_without_asserts, T2K truetype.c) ----------

/// The T2K TrueType font (`sfntClass`, 0x68 bytes; the game's `FONT` keeps it through the scaler, +0x139): +0 the table
/// directory, +4 the hinting context (`tt_exec_create`), +8 the TrueType collection header, +0xc head, +0x10 maxp,
/// +0x14 loca, +0x18 hhea, +0x1c vhea, +0x20 hmtx, +0x24 vmtx, +0x28 the character map (loaded on first use), +0x2c
/// kern, +0x30 and +0x32 the cmap platform and encoding asked for (i16), +0x34 and +0x36 the underline position and
/// thickness (post, i16), +0x3c and +0x40 the style functions (outline and metrics), +0x44 their four parameters,
/// +0x54 the font file stream, +0x5c the memory context, +0x64 the glyph count.
pub(crate) mod tt_font {
    pub(crate) const DIRECTORY: usize = 0x0;
    pub(crate) const EXEC: usize = 0x4;
    pub(crate) const COLLECTION: usize = 0x8;
    pub(crate) const HEAD: usize = 0xc;
    pub(crate) const MAXP: usize = 0x10;
    pub(crate) const LOCA: usize = 0x14;
    pub(crate) const HHEA: usize = 0x18;
    pub(crate) const VHEA: usize = 0x1c;
    pub(crate) const HMTX: usize = 0x20;
    pub(crate) const VMTX: usize = 0x24;
    pub(crate) const CMAP: usize = 0x28;
    pub(crate) const KERN: usize = 0x2c;
    pub(crate) const CMAP_PLATFORM: usize = 0x30;
    pub(crate) const CMAP_ENCODING: usize = 0x32;
    pub(crate) const UNDERLINE_POSITION: usize = 0x34;
    pub(crate) const UNDERLINE_THICKNESS: usize = 0x36;
    pub(crate) const STYLE_METRICS_FN: usize = 0x40;
    pub(crate) const STYLE_PARAMS: usize = 0x44;
    pub(crate) const STREAM: usize = 0x54;
    pub(crate) const MEM: usize = 0x5c;
}

/// The TrueType table tags, read as big-endian integers.
pub(crate) const TAG_CMAP: u32 = 0x636d6170;
pub(crate) const TAG_HEAD: u32 = 0x68656164;
pub(crate) const TAG_HHEA: u32 = 0x68686561;
pub(crate) const TAG_VHEA: u32 = 0x76686561;
pub(crate) const TAG_MAXP: u32 = 0x6d617870;
pub(crate) const TAG_LOCA: u32 = 0x6c6f6361;
pub(crate) const TAG_HMTX: u32 = 0x686d7478;
pub(crate) const TAG_VMTX: u32 = 0x766d7478;
pub(crate) const TAG_KERN: u32 = 0x6b65726e;
pub(crate) const TAG_FVAR: u32 = 0x66766172;
pub(crate) const TAG_POST: u32 = 0x706f7374;
pub(crate) const TAG_GLYF: u32 = 0x676c7966;
pub(crate) const TAG_TTCF: u32 = 0x74746366;
/// The units per em of a font without a head table.
const DEFAULT_UNITS_PER_EM: u32 = 0x800;

/// The input stream constructor of the exe (it stays there: it calls setjmp when given a status pointer).
const FN_INPUT_STREAM_NEW: usize = 0x941630;

/// `input_stream_new` (0x941630, cdecl): a stream over `length` bytes at `offset` of the data of `source` (+0); with
/// `scratch`, the record comes from scratch slot 2; no status pointer here, so no setjmp.
pub(crate) unsafe fn t2k_stream_open(mem: usize, source: usize, offset: u32, length: u32) -> usize {
    let f: unsafe extern "C" fn(usize, usize, u32, u32, i32, usize) -> usize = core::mem::transmute(FN_INPUT_STREAM_NEW);
    f(mem, source, offset, length, 0, 0)
}

/// Finds a table by its tag in the font's directory (+8 the count, i16; +0x10 the entries: +4 the tag, +0xc the
/// offset, +0x10 the length); null when absent.
/// Original: `tt_find_table_by_tag` (0x942290, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_find_table_by_tag(font: usize, tag: u32) -> usize {
    let dir = read_usize(font + tt_font::DIRECTORY);
    let (n, entries) = (read_i16(dir + 8) as i32, read_usize(dir + 0x10));
    for k in 0..n.max(0) as usize {
        let entry = read_usize(entries + 4 * k);
        if read_u32(entry + 4) == tag {
            return entry;
        }
    }
    0
}

/// Opens an input stream over a table of the font file (`input_stream_new` with the table's offset and length);
/// null when the font has no such table.
/// Original: `tt_open_table_stream` (0x9422d0, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_open_table_stream(font: usize, tag: u32) -> usize {
    let entry = tt_find_table_by_tag(font, tag);
    if entry == 0 {
        return 0;
    }
    t2k_stream_open(read_usize(font + tt_font::MEM), read_usize(font + tt_font::STREAM), read_u32(entry + 0xc), read_u32(entry + 0x10))
}

/// The glyph count: maxp's count (+8, u16), cut to the loca entry count minus one (+8 of loca) when that count is not
/// larger.
/// Original: `tt_font_get_glyph_count` (0x942330, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_get_glyph_count(font: usize) -> i32 {
    let count = read_u16(read_usize(font + tt_font::MAXP) + 8) as i32;
    let loca = read_usize(font + tt_font::LOCA);
    if loca != 0 {
        let entries = read_i32(loca + 8);
        if entries <= count {
            return entries.wrapping_sub(1);
        }
    }
    count
}

/// Reads one 16-byte table directory entry (a 0x14-byte block): +0 the memory context, then the tag, the checksum,
/// the offset and the length, all big-endian 32-bit.
/// Original: `tt_read_table_directory_entry` (0x943b60, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_read_table_directory_entry(mem: usize, s: usize) -> usize {
    let e = lib_ctx_alloc(mem, 0x14);
    write_usize(e, mem);
    for k in 1..5 {
        write_u32(e + 4 * k, input_stream_read_int32(s));
    }
    e
}

/// Reads a horizontal or vertical metrics header (a 0x28-byte block): +0 the memory context, +4 the version (32-bit),
/// then sixteen 16-bit fields from +8 (ascender, descender, line gap, maximum advance, ..., +0x26 the metrics count).
/// Original: `tt_read_hhea_or_vhea` (0x943bb0, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_read_hhea_or_vhea(mem: usize, s: usize) -> usize {
    let h = lib_ctx_alloc(mem, 0x28);
    write_usize(h, mem);
    write_u32(h + 4, input_stream_read_int32(s));
    for k in 0..16 {
        write_i16(h + 8 + 2 * k, input_stream_read_int16(s));
    }
    h
}

/// Reads a metrics table (a 0x14-byte block): +0 the memory context, +4 the glyph count, +8 the metrics count, +0xc
/// the side bearings and +0x10 the advances (2 × glyph count bytes each); `metrics` pairs (advance, bearing), then the
/// remaining glyphs take the last advance and read only their bearing.
/// Defect of the original: with no metrics pair the "last advance" is read from before the advances block (its guard
/// header).
/// Original: `tt_read_hmtx_or_vmtx` (0x943c80, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_read_hmtx_or_vmtx(mem: usize, s: usize, glyphs: i32, metrics: i32) -> usize {
    let m = lib_ctx_alloc(mem, 0x14);
    write_usize(m, mem);
    write_i32(m + 4, glyphs);
    write_i32(m + 8, metrics);
    write_usize(m + 0xc, lib_ctx_alloc(mem, glyphs.wrapping_mul(2) as u32));
    write_usize(m + 0x10, lib_ctx_alloc(mem, glyphs.wrapping_mul(2) as u32));
    let (bearings, advances) = (read_usize(m + 0xc), read_usize(m + 0x10));
    let mut k = 0;
    while k < metrics {
        write_i16(advances + 2 * k as usize, input_stream_read_int16(s));
        write_i16(bearings + 2 * k as usize, input_stream_read_int16(s));
        k += 1;
    }
    let last = read_i16(advances.wrapping_add(2 * k as usize).wrapping_sub(2));
    while k < glyphs {
        write_i16(advances + 2 * k as usize, last);
        write_i16(bearings + 2 * k as usize, input_stream_read_int16(s));
        k += 1;
    }
    m
}

/// Frees a kern format 0 subtable (its pair array, +0xc, then the block); nothing for null.
/// Original: `tt_kern_free_format0_table` (0x943960, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_kern_free_format0_table(t: usize) {
    if t != 0 {
        lib_ctx_free(read_usize(t), read_usize(t + 0xc));
        lib_ctx_free(read_usize(t), t);
    }
}

/// Reads a kern table (a 0xe-byte block: +0 the memory context, +4 the version, +6 the subtable count, +0xa the
/// subtable pointers). The Microsoft header is a 16-bit version and count; version 1 with count 0 is Apple's 32-bit
/// version, and the count is then read as 32 bits. Each subtable (0x10 bytes: +0 the context, +4 the format, +6 the
/// length, +0xa the coverage, +0xc the format 0 data): Apple's has a 32-bit length, the coverage and a skipped tuple
/// index, the format being the coverage's low byte; Microsoft's has the version (as the format), a 16-bit length and
/// the coverage. A format 0 subtable with a positive length reads its pair count, search range, entry selector and
/// range shift (0x10-byte block) and its pairs (6 bytes each: the 32-bit glyph pair and the 16-bit value).
/// Original: `tt_read_kern_table` (0x943990, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_read_kern_table(mem: usize, s: usize) -> usize {
    let k = lib_ctx_alloc(mem, 0xe);
    write_usize(k, mem);
    write_i16(k + 4, input_stream_read_int16(s));
    write_i32(k + 6, input_stream_read_int16(s) as i32);
    let mut apple = false;
    if read_i16(k + 4) == 1 && read_i32(k + 6) == 0 {
        write_u32(k + 6, input_stream_read_int32(s));
        apple = true;
    }
    write_usize(k + 0xa, lib_ctx_alloc(mem, read_i32(k + 6).wrapping_mul(4) as u32));
    let mut i = 0;
    while i < read_i32(k + 6) {
        let sub = lib_ctx_alloc(mem, 0x10);
        write_usize(sub, mem);
        if apple {
            write_u32(sub + 6, input_stream_read_int32(s));
            write_i16(sub + 0xa, input_stream_read_int16(s));
            input_stream_read_int16(s);
            write_u16(sub + 4, read_u8(sub + 0xa) as u16);
        } else {
            write_i16(sub + 4, input_stream_read_int16(s));
            write_u32(sub + 6, input_stream_read_int16(s) as u16 as u32);
            write_i16(sub + 0xa, input_stream_read_int16(s));
        }
        write_u32(sub + 0xc, 0);
        if read_i16(sub + 4) == 0 && read_i32(sub + 6) > 0 {
            write_usize(sub + 0xc, read_kern_format0(mem, s));
        }
        write_usize(read_usize(k + 0xa) + 4 * i as usize, sub);
        i += 1;
    }
    k
}

/// A kern format 0 subtable body: the counts (16-bit each) and the pairs.
unsafe fn read_kern_format0(mem: usize, s: usize) -> usize {
    let t = lib_ctx_alloc(mem, 0x10);
    write_usize(t, mem);
    for k in 0..4 {
        write_i16(t + 4 + 2 * k, input_stream_read_int16(s));
    }
    write_usize(t + 0xc, lib_ctx_alloc(mem, read_u16(t + 4) as u32 * 6));
    let mut j = 0u32;
    while j < read_u16(t + 4) as u32 {
        let pair = read_usize(t + 0xc) + 6 * j as usize;
        write_u32(pair, input_stream_read_int32(s));
        write_i16(read_usize(t + 0xc) + 6 * j as usize + 4, input_stream_read_int16(s));
        j += 1;
    }
    t
}

/// The units per em (head +0x16, u16), or 0x800 without a head table.
/// Original: `tt_font_get_units_per_em` (0x943d30, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_get_units_per_em(font: usize) -> u32 {
    let head = read_usize(font + tt_font::HEAD);
    if head != 0 {
        read_u16(head + 0x16) as u32
    } else {
        DEFAULT_UNITS_PER_EM
    }
}

/// The larger of maxp's simple glyph point limit (+0xa) and composite point limit (+0xe), signed.
/// Original: `tt_font_get_max_glyph_points` (0x943d50, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_get_max_glyph_points(font: usize) -> i32 {
    let maxp = read_usize(font + tt_font::MAXP);
    read_i16(maxp + 0xa).max(read_i16(maxp + 0xe)) as i32
}

/// Fills the horizontal and vertical line metrics records (+0 present, +4..+0xa ascender, descender, line gap and
/// maximum advance, +0xc and +0x10 the caret slope (rise, run) doubled while both stay inside ±0x10000, at most 16
/// times, +0x14 and +0x16 the underline position and thickness, only in the horizontal one) from hhea and vhea.
/// Original: `tt_font_get_line_metrics` (0x943d70, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_get_line_metrics(font: usize, h: usize, v: usize) {
    write_u32(v, 0);
    write_u32(h, 0);
    write_u16(h + 0x14, 0);
    write_u16(h + 0x16, 0);
    write_u16(v + 0x14, 0);
    write_u16(v + 0x16, 0);
    write_i16(h + 0x14, read_i16(font + tt_font::UNDERLINE_POSITION));
    write_i16(h + 0x16, read_i16(font + tt_font::UNDERLINE_THICKNESS));
    fill_line_metrics(read_usize(font + tt_font::HHEA), h);
    fill_line_metrics(read_usize(font + tt_font::VHEA), v);
}

/// One line metrics record from a metrics header (nothing when the header is absent).
unsafe fn fill_line_metrics(header: usize, out: usize) {
    if header == 0 {
        return;
    }
    write_u32(out, 1);
    for k in 0..4 {
        write_i16(out + 4 + 2 * k, read_i16(header + 8 + 2 * k));
    }
    write_i32(out + 0xc, read_i16(header + 0x18) as i32);
    write_i32(out + 0x10, read_i16(header + 0x16) as i32);
    let inside = |v: i32| v < 0x10000 && v > -0x10000;
    let mut k = 0;
    while inside(read_i32(out + 0xc)) && inside(read_i32(out + 0x10)) {
        write_i32(out + 0xc, read_i32(out + 0xc).wrapping_mul(2));
        write_i32(out + 0x10, read_i32(out + 0x10).wrapping_mul(2));
        k += 1;
        if k >= 16 {
            break;
        }
    }
}

/// Frees the character map (+0x28: its subtable records, their array, the cmap data and the block) and clears the
/// field.
/// Original: `tt_font_free_cmap` (0x943ec0, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_font_free_cmap(font: usize) {
    let c = read_usize(font + tt_font::CMAP);
    if c != 0 {
        let mut k = 0;
        while k < read_i16(c + 6) as i32 {
            lib_ctx_free(read_usize(c), read_usize(read_usize(c + 8) + 4 * k as usize));
            k += 1;
        }
        lib_ctx_free(read_usize(c), read_usize(c + 8));
        lib_ctx_free(read_usize(c), read_usize(c + 0xc));
        lib_ctx_free(read_usize(c), c);
    }
    write_u32(font + tt_font::CMAP, 0);
}

/// Comparers: value against value for the pure getters; side by side on the two records for the line metrics; only
/// the original for the readers and the frees (they allocate or free; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_find_table_by_tag(font: usize, tag: u32) -> usize {
    let (r, c) = (original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I0%%)(font, tag), tt_find_table_by_tag(font, tag));
    SCOREBOARD.record(r == c, || format!("tt_find_table_by_tag({:#x}) original={:#x} rust={:#x}", tag, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_open_table_stream(font: usize, tag: u32) -> usize {
    original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I1%%)(font, tag)
}

pub(crate) unsafe extern "C" fn compare_tt_font_get_glyph_count(font: usize) -> i32 {
    let (r, c) = (original_fn::<unsafe extern "C" fn(usize) -> i32>(%%I2%%)(font), tt_font_get_glyph_count(font));
    SCOREBOARD.record(r == c, || format!("tt_font_get_glyph_count original={} rust={}", r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_read_table_directory_entry(mem: usize, s: usize) -> usize {
    original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I3%%)(mem, s)
}

pub(crate) unsafe extern "C" fn compare_tt_read_hhea_or_vhea(mem: usize, s: usize) -> usize {
    original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I4%%)(mem, s)
}

pub(crate) unsafe extern "C" fn compare_tt_read_hmtx_or_vmtx(mem: usize, s: usize, glyphs: i32, metrics: i32) -> usize {
    original_fn::<unsafe extern "C" fn(usize, usize, i32, i32) -> usize>(%%I5%%)(mem, s, glyphs, metrics)
}

pub(crate) unsafe extern "C" fn compare_tt_kern_free_format0_table(t: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I6%%)(t)
}

pub(crate) unsafe extern "C" fn compare_tt_read_kern_table(mem: usize, s: usize) -> usize {
    original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I7%%)(mem, s)
}

pub(crate) unsafe extern "C" fn compare_tt_font_get_units_per_em(font: usize) -> u32 {
    let (r, c) = (original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I8%%)(font), tt_font_get_units_per_em(font));
    SCOREBOARD.record(r & 0xffff == c, || format!("tt_font_get_units_per_em original={} rust={}", r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_font_get_max_glyph_points(font: usize) -> i32 {
    let (r, c) = (original_fn::<unsafe extern "C" fn(usize) -> i32>(%%I9%%)(font), tt_font_get_max_glyph_points(font));
    SCOREBOARD.record(r == c, || format!("tt_font_get_max_glyph_points original={} rust={}", r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_font_get_line_metrics(font: usize, h: usize, v: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize)>(%%I10%%);
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&[(h, 0x18), (v, 0x18)], || o(font, h, v), || tt_font_get_line_metrics(font, h, v));
    SCOREBOARD.record(same, || "tt_font_get_line_metrics differs".to_string());
}

pub(crate) unsafe extern "C" fn compare_tt_font_free_cmap(font: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I11%%)(font)
}
