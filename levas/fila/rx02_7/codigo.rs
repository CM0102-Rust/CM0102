// ---------- rx02_7: the T2K TrueType interpreter: the DELTA opcodes; the T2K automatic hinter: the font's hint parameters, read or measured (library_without_asserts, T2K fnt and autogrid) ----------

/// The T2K library's routines this item calls (rx03 ports them; called here through their exe addresses).
const FN_LIB_READ_INT16: usize = 0x9415c0; // input_stream_read_int16: (stream) -> big-endian i16
const FN_LIB_MEDIAN_CHAR_EXTREME_Y: usize = 0x93ba30; // glyph_hint_median_char_extreme_y: (font, letters, top) -> i16
const FN_LIB_CMAP_LOOKUP: usize = 0x942a40; // tt_font_cmap_lookup: (font, character) -> glyph index
const FN_LIB_LOAD_GLYPH: usize = 0x942350; // tt_font_load_glyph: (font, index, 0, &a, &b) -> glyph
const FN_LIB_GLYPH_COUNT: usize = 0x942330; // tt_font_get_glyph_count: (font) -> count
const FN_LIB_GLYPH_FREE: usize = 0x93c5e0; // glyph_outline_free: (glyph)
const FN_LIB_FREE_CMAP: usize = 0x943ec0; // tt_font_free_cmap: (font)
const FN_LIB_SORT_I16: usize = 0x9446a0; // sort_int16_array_ascending: (array, count)
/// `glyph_hint_collect_stem_widths_checked` (rx01_7), called through its exe address like the original (the detour
/// takes it to the copy).
const FN_GLYPH_HINT_COLLECT_STEM_WIDTHS_CHECKED: usize = 0x934b10;
/// `tt_delta_cvt_add`, the DELTAC callback (its exe address: the scan calls it through the pointer).
pub(crate) const TT_DELTA_CVT_ADD: usize = 0x93b280;

/// `input_stream_read_int16` (0x9415c0, cdecl): the next big-endian 16-bit value of a memory stream.
pub(crate) unsafe fn lib_read_int16(stream: usize) -> i16 {
    let f: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_LIB_READ_INT16);
    f(stream)
}

/// `sort_int16_array_ascending` (0x9446a0, cdecl): sorts `count` i16 values in place.
pub(crate) unsafe fn lib_sort_i16(array: usize, count: i32) {
    let f: unsafe extern "C" fn(usize, i32) = core::mem::transmute(FN_LIB_SORT_I16);
    f(array, count)
}

/// The DELTAP and DELTAC scan: pops a count n and n (argument, target) pairs (the deepest pair first in the stack).
/// The pixels per em (stretched along the projection vector with non-square pixels) minus `base` (16 bits) picks the
/// size; only sizes 0-15 above the base apply. Each pair whose argument's high bits (`argument & !0xf`) are that size
/// × 16 calls `callback(state, zone 0, target, step)`, the step being the argument's low 4 bits as −8..−1, 1..8
/// pixels (×64, shifted right by `shift`). The pairs are found by a halving search for the size, then scanned in
/// order until an argument for a larger size.
/// Original: `tt_delta_scan_apply` (0x93b0a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_delta_scan_apply(gs: TtLocalState, callback: usize, base: i32, shift: i32) {
    let g = gs.global();
    let words = gs.pop().wrapping_mul(2);
    gs.set_stack_ptr(gs.stack_ptr().wrapping_sub((words as usize).wrapping_mul(4)));
    let mut ppem = g.ppem() as i32;
    if !g.square_pixels() {
        ppem = lib_fixed_mul(ppem, projection_stretch(gs));
    }
    let size = ppem.wrapping_sub(base as i16 as i32);
    if !(0..16).contains(&size) {
        return;
    }
    let key = size << 4;
    let pairs = gs.stack_ptr();
    let mut k = 0;
    let mut step = (words >> 1) & !1;
    while step > 2 {
        if at_i32(pairs, step + k) & !0xf < key {
            k += step;
        }
        step = (step >> 1) & !1;
    }
    while k < words {
        let argument = at_i32(gs.stack_ptr(), k);
        let class = argument & !0xf;
        if class == key {
            let low = argument & 0xf;
            let pixels = low - if low < 8 { 8 } else { 7 };
            let target = at_i32(gs.stack_ptr(), k + 1);
            let f: TtMovePointFn = core::mem::transmute(callback);
            f(gs, gs.zone(0), target, (pixels << 6).wrapping_shr(shift as u32));
        } else if class > key {
            return;
        }
        k += 2;
    }
}

/// Opcode 0x71, DELTAP2: the point exceptions for the sizes from the delta base + 16.
/// Original: `tt_op_deltap2` (0x93b1f0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_deltap2(gs: TtLocalState) {
    let g = gs.global();
    tt_delta_scan_apply(gs, gs.move_point_fn(), g.delta_base().wrapping_add(16) as i32, g.delta_shift() as i32);
}

/// Opcode 0x72, DELTAP3: the point exceptions for the sizes from the delta base + 32.
/// Original: `tt_op_deltap3` (0x93b220, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_deltap3(gs: TtLocalState) {
    let g = gs.global();
    tt_delta_scan_apply(gs, gs.move_point_fn(), g.delta_base().wrapping_add(32) as i32, g.delta_shift() as i32);
}

/// Opcode 0x73, DELTAC1: the control value exceptions for the sizes from the delta base (`tt_delta_cvt_add`).
/// Original: `tt_op_deltac1` (0x93b250, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_deltac1(gs: TtLocalState) {
    let g = gs.global();
    tt_delta_scan_apply(gs, TT_DELTA_CVT_ADD, g.delta_base() as i32, g.delta_shift() as i32);
}

/// The DELTAC callback: adds `step` to control value `n` (the zone is not used).
/// Original: `tt_delta_cvt_add` (0x93b280, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_delta_cvt_add(gs: TtLocalState, _zone: TtZone, n: i32, step: i32) {
    let cvt = gs.global().cvt();
    set_i32(cvt, n, at_i32(cvt, n).wrapping_add(step));
}

/// Opcode 0x74, DELTAC2: the control value exceptions for the sizes from the delta base + 16.
/// Original: `tt_op_deltac2` (0x93b2a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_deltac2(gs: TtLocalState) {
    let g = gs.global();
    tt_delta_scan_apply(gs, TT_DELTA_CVT_ADD, g.delta_base().wrapping_add(16) as i32, g.delta_shift() as i32);
}

/// Opcode 0x75, DELTAC3: the control value exceptions for the sizes from the delta base + 32.
/// Original: `tt_op_deltac3` (0x93b2d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_deltac3(gs: TtLocalState) {
    let g = gs.global();
    tt_delta_scan_apply(gs, TT_DELTA_CVT_ADD, g.delta_base().wrapping_add(32) as i32, g.delta_shift() as i32);
}

/// Reads the font's stored hint parameters (the T2KG table) from `stream` into `params` (the 27 dwords the hinter
/// keeps at +0x34e): the 10 alignment zones as (flat height, flat + overshoot, overshoot), the stored pair being the
/// flat height and the overshoot, then the 12 x and the 12 y stem widths, read in pairs (x, y).
/// Original: `glyph_hint_read_font_params` (0x93b300, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_read_font_params(params: usize, stream: usize) {
    lib_read_int16(stream); // the table's version, not kept
    for k in 0..10 {
        let zone = params + 6 * k;
        let flat = lib_read_int16(stream);
        write_i16(zone, flat);
        let overshoot = lib_read_int16(stream);
        write_i16(zone + 4, overshoot);
        write_i16(zone + 2, flat.wrapping_add(overshoot));
    }
    for k in 0..12 {
        write_i16(params + 0x3c + 2 * k, lib_read_int16(stream));
        write_i16(params + 0x54 + 2 * k, lib_read_int16(stream));
    }
}

/// The letters measured for each alignment zone value: (the value's byte offset in the parameters, the letters, 1 for
/// their top extreme or 0 for their bottom). The zones are 0 the ascenders, 1 the capitals' top, 2 the figures' top,
/// 3 the x height, 4 the capitals' base, 5 the lower case base, 6 the figures' base, 7 the descenders, 8 the
/// brackets' top and 9 their bottom; in each, the flat letters, then the round ones.
const GLYPH_HINT_ZONE_LETTERS: [(usize, &[u8], i32); 20] = [
    (0x00, b"bdhkl\0", 1),
    (0x02, b"f\0", 1),
    (0x06, b"EFHIL\0", 1),
    (0x08, b"CGOQ\0", 1),
    (0x0c, b"7\0", 1),
    (0x0e, b"089\0", 1),
    (0x12, b"z\0", 1),
    (0x14, b"eco\0", 1),
    (0x18, b"AFHILTZ\0", 0),
    (0x1a, b"CJOSU\0", 0),
    (0x1e, b"r\0", 0),
    (0x20, b"ceos\0", 0),
    (0x24, b"1247\0", 0),
    (0x26, b"035689\0", 0),
    (0x2a, b"pq\0", 0),
    (0x2c, b"g\0", 0),
    (0x30, b"[]\0", 1),
    (0x32, b"{}()\0", 1),
    (0x36, b"[]\0", 0),
    (0x38, b"{}()\0", 0),
];

/// The size of a 16-bit value (−32768 stays itself, as with the original's 16-bit negation).
fn abs_i16(v: i16) -> i16 {
    if v < 0 { v.wrapping_neg() } else { v }
}

/// The median x and y stem widths of glyph `index` of the font, from the hinter's analysis in mode 2 (the widths
/// sorted, the middle one of each); None when the glyph has no contour. The glyph and the width lists are freed.
unsafe fn glyph_hint_glyph_stem_medians(font: usize, hinter: GlyphHinter, index: i32) -> Option<(i16, i16)> {
    let (mut a, mut b) = (0i16, 0i16);
    let load: unsafe extern "C" fn(usize, i32, i32, usize, usize) -> usize = core::mem::transmute(FN_LIB_LOAD_GLYPH);
    let glyph = load(font, index, 0, &mut a as *mut i16 as usize, &mut b as *mut i16 as usize);
    let contours = read_i16(glyph + 0x34); // the glyph's contour count, then +0x36 its point count, +0x38.. its arrays
    let mut medians = None;
    if contours > 0 {
        // the glyph as the hinter reads it (`HintGlyph`): the counts and the arrays, no hinted coordinates
        let mut outline = [0u32; 10];
        let view = outline.as_mut_ptr() as usize;
        write_i16(view, contours);
        write_i16(view + 2, read_i16(glyph + 0x36));
        for k in 0..5 {
            write_u32(view + 4 + 4 * k, read_u32(glyph + 0x38 + 4 * k));
        }
        let (mut x_widths, mut x_count, mut y_widths, mut y_count) = (0usize, 0i32, 0usize, 0i32);
        let collect: unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32, usize, usize, usize, usize) -> i32 =
            core::mem::transmute(FN_GLYPH_HINT_COLLECT_STEM_WIDTHS_CHECKED);
        collect(
            hinter,
            HintGlyph(view),
            0,
            2,
            &mut x_widths as *mut usize as usize,
            &mut x_count as *mut i32 as usize,
            &mut y_widths as *mut usize as usize,
            &mut y_count as *mut i32 as usize,
        );
        lib_sort_i16(x_widths, x_count);
        lib_sort_i16(y_widths, y_count);
        medians = Some((at_i16(x_widths, x_count >> 1), at_i16(y_widths, y_count >> 1)));
        let memory = read_usize(font + 0x5c); // the font's memory context
        lib_mem_free(memory, x_widths);
        lib_mem_free(read_usize(font + 0x5c), y_widths);
    }
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_LIB_GLYPH_FREE);
    free(glyph);
    medians
}

/// Measures the font's hint parameters when it has no T2KG table, into `params` (the layout of
/// `glyph_hint_read_font_params`):
/// - each alignment zone's flat and round heights are the medians of the extremes of its letters
///   (`GLYPH_HINT_ZONE_LETTERS`), the overshoot their difference, kept non-negative for the top zones (0-3, 8) and
///   non-positive for the bottom ones (4-7, 9);
/// - the overshoot pairs (capitals 1/4, x height 3/5, figures 2/6, brackets 8/9) are made symmetric, first with the
///   median m of the seven overshoot sizes (1, 3, 2, 4, 5, 6, 7), else with the pair's average, when both are within
///   2 units of it; the ascender's overshoot becomes m and the descender's −m on the same test;
/// - the round heights are rebuilt (flat + overshoot) and the 24 stem widths cleared;
/// - the stem widths: when `sample_glyphs` is 0, from the letter "o" alone; otherwise (or when "o" has no contour)
///   from up to 32 glyphs, one per 1/32 of the font from its first 1/64 (the first of five glyphs in a row with
///   contours), the medians of their median widths; x and y become their smaller one when they differ by at most 1
///   unit.
/// The font's character map is freed at the end.
/// Defects of the original: with fewer than 32 glyphs and no contour in the first five, the sampling never ends (the
/// step is 0); with no sample the widths are read from uninitialised stack (the copy reads zeros).
/// Original: `glyph_hint_measure_font_params` (0x93b370, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_measure_font_params(font: usize, hinter: GlyphHinter, params: usize, sample_glyphs: i32) {
    let median_extreme: unsafe extern "C" fn(usize, usize, i32) -> i16 = core::mem::transmute(FN_LIB_MEDIAN_CHAR_EXTREME_Y);
    for &(at, letters, top) in GLYPH_HINT_ZONE_LETTERS.iter() {
        write_i16(params + at, median_extreme(font, letters.as_ptr() as usize, top));
    }
    measure_zone_overshoots(params);
    for k in 0..12 {
        write_i16(params + 0x3c + 2 * k, 0);
        write_i16(params + 0x54 + 2 * k, 0);
    }
    measure_stem_widths(font, hinter, params, sample_glyphs);
    let free_cmap: unsafe extern "C" fn(usize) = core::mem::transmute(FN_LIB_FREE_CMAP);
    free_cmap(font);
}

/// The alignment zones' overshoots of `glyph_hint_measure_font_params`, and the round heights rebuilt from them.
unsafe fn measure_zone_overshoots(params: usize) {
    let overshoot = |k: usize| params + 6 * k + 4;
    for k in 0..10 {
        write_i16(overshoot(k), read_i16(params + 6 * k + 2).wrapping_sub(read_i16(params + 6 * k)));
    }
    for k in [0, 1, 2, 3, 8] {
        if read_i16(overshoot(k)) < 0 {
            write_i16(overshoot(k), 0);
        }
    }
    for k in [4, 5, 6, 7, 9] {
        if read_i16(overshoot(k)) > 0 {
            write_i16(overshoot(k), 0);
        }
    }
    let mut sizes = [1, 3, 2, 4, 5, 6, 7].map(|k| {
        let v = read_i16(overshoot(k));
        if k < 4 { v } else { v.wrapping_neg() }
    });
    lib_sort_i16(sizes.as_mut_ptr() as usize, 7);
    let median = sizes[3];
    for (top, bottom) in [(1, 4), (3, 5), (2, 6), (8, 9)] {
        let (up, down) = (read_i16(overshoot(top)), read_i16(overshoot(bottom)));
        let average = ((up as i32 - down as i32 + 1) >> 1) as i16;
        for v in [median, average] {
            if abs_i16(v.wrapping_sub(up)) < 3 && abs_i16(down.wrapping_add(v)) < 3 {
                write_i16(overshoot(top), v);
                write_i16(overshoot(bottom), v.wrapping_neg());
                break;
            }
        }
    }
    if abs_i16(median.wrapping_sub(read_i16(overshoot(0)))) < 3 {
        write_i16(overshoot(0), median);
    }
    if abs_i16(median.wrapping_add(read_i16(overshoot(7)))) < 3 {
        write_i16(overshoot(7), median.wrapping_neg());
    }
    for k in 0..10 {
        write_i16(params + 6 * k + 2, read_i16(params + 6 * k).wrapping_add(read_i16(overshoot(k))));
    }
}

/// The stem widths of `glyph_hint_measure_font_params` (first x and y width, at +0x3c and +0x54).
unsafe fn measure_stem_widths(font: usize, hinter: GlyphHinter, params: usize, sample_glyphs: i32) {
    if sample_glyphs == 0 {
        let lookup: unsafe extern "C" fn(usize, i32) -> u32 = core::mem::transmute(FN_LIB_CMAP_LOOKUP);
        let index = lookup(font, b'o' as i32) & 0xffff;
        if let Some((x, y)) = glyph_hint_glyph_stem_medians(font, hinter, index as i32) {
            write_i16(params + 0x3c, x);
            write_i16(params + 0x54, y);
            return;
        }
    }
    let count_fn: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_LIB_GLYPH_COUNT);
    let count = count_fn(font);
    let (mut xs, mut ys) = ([0i16; 32], [0i16; 32]);
    let mut n = 0usize;
    let mut first = count / 64;
    while first < count && n < 32 {
        let mut k = 0;
        loop {
            if let Some((x, y)) = glyph_hint_glyph_stem_medians(font, hinter, k + first) {
                xs[n] = x;
                ys[n] = y;
                n += 1;
                break;
            }
            if k >= 4 {
                break;
            }
            k += 1;
        }
        first += count / 32;
    }
    lib_sort_i16(xs.as_mut_ptr() as usize, n as i32);
    lib_sort_i16(ys.as_mut_ptr() as usize, n as i32);
    let (x, y) = (xs[n >> 1], ys[n >> 1]);
    write_i16(params + 0x3c, x);
    write_i16(params + 0x54, y);
    if (x.max(y) as i32) - (x.min(y) as i32) <= 1 {
        write_i16(params + 0x3c, x.min(y));
        write_i16(params + 0x54, x.min(y));
    }
}

/// Comparers: only the original (they change the interpreter's state or read a font; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_delta_scan_apply(gs: TtLocalState, callback: usize, base: i32, shift: i32) {
    original_fn::<unsafe extern "C" fn(TtLocalState, usize, i32, i32)>(%%I0%%)(gs, callback, base, shift)
}

pub(crate) unsafe extern "C" fn compare_tt_op_deltap2(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_deltap3(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_deltac1(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_delta_cvt_add(gs: TtLocalState, zone: TtZone, n: i32, step: i32) {
    original_fn::<TtMovePointFn>(%%I4%%)(gs, zone, n, step)
}

pub(crate) unsafe extern "C" fn compare_tt_op_deltac2(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I5%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_deltac3(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_read_font_params(params: usize, stream: usize) {
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I7%%)(params, stream)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_measure_font_params(font: usize, hinter: GlyphHinter, params: usize, sample_glyphs: i32) {
    original_fn::<unsafe extern "C" fn(usize, GlyphHinter, usize, i32)>(%%I8%%)(font, hinter, params, sample_glyphs)
}
