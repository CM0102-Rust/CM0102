// ---------- rx03_9: the T2K scaler: the hinting set up, the memory purge, the coordinate scaling and the five-tap filter (library_without_asserts, T2K t2k.c) ----------

/// The T2K scaler (`T2K`, 0x5ed bytes, `t2k_scaler_new`; the game's `FONT` keeps it at +0x14): +4 the memory context,
/// +0xa4 and +0xa8 the advance (16.16), +0xc4 and +0xc8 the left bearing and the current glyph, +0xce the bitmap width,
/// +0xd2 its height, +0xd6 its row bytes, +0xda the bitmap, +0xde the second bitmap, +0xe6 and +0xea the pixel sizes,
/// +0x126 and +0x12a the sizes the hinting was set up for, +0x12e the small size flag, +0x12f the hinting mode, +0x137
/// the units per em, +0x139 the TrueType font, +0x13d the automatic hinter.
pub(crate) mod t2k_scaler {
    pub(crate) const MEM: usize = 0x4;
    pub(crate) const ADVANCE_X: usize = 0xa8;
    pub(crate) const LEFT: usize = 0xc4;
    pub(crate) const GLYPH: usize = 0xc8;
    pub(crate) const WIDTH: usize = 0xce;
    pub(crate) const HEIGHT: usize = 0xd2;
    pub(crate) const ROW_BYTES: usize = 0xd6;
    pub(crate) const BITMAP: usize = 0xda;
    pub(crate) const BITMAP_2: usize = 0xde;
    pub(crate) const X_PPEM: usize = 0xe6;
    pub(crate) const Y_PPEM: usize = 0xea;
    pub(crate) const HINTED_X_PPEM: usize = 0x126;
    pub(crate) const HINTED_Y_PPEM: usize = 0x12a;
    pub(crate) const SMALL: usize = 0x12e;
    pub(crate) const HINT_MODE: usize = 0x12f;
    pub(crate) const UNITS_PER_EM: usize = 0x137;
    pub(crate) const FONT: usize = 0x139;
    pub(crate) const HINTER: usize = 0x13d;
}

/// The automatic hinter's routines of the exe (rx01 and rx02; the detours lead to the Rust copies).
const FN_GLYPH_HINT_SET_FONT_PARAMS: usize = 0x9364c0;
const FN_GLYPH_HINT_CREATE: usize = 0x9365d0;
const FN_GLYPH_HINT_DESTROY: usize = 0x936840;
const FN_GLYPH_HINT_SET_SIZE: usize = 0x9368e0;
const FN_GLYPH_HINT_READ_FONT_PARAMS: usize = 0x93b300;
const FN_GLYPH_HINT_MEASURE_FONT_PARAMS: usize = 0x93b370;
/// The tag of the T2K hinter parameters table.
const TAG_T2KG: u32 = 0x54324b47;

/// The hinter of the scaler: created on first use (the larger point limit, the units per em), with its mode (2 for a
/// font of 1000 glyphs or more, else 1) and the font parameters (read from the font's T2KG table when the font has no
/// style function, else measured; kept in a 0x6c-byte block of the font, +0x38, once); then, when the pixel sizes
/// changed, the hinter's size and the TrueType context's size (its control program runs). Hinter errors longjmp.
/// Original: `t2k_set_up_hinting_for_size` (0x93e5b0, library_without_asserts, T2K t2k.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_set_up_hinting_for_size(t: usize) {
    let mem = read_usize(t + t2k_scaler::MEM);
    let font = read_usize(t + t2k_scaler::FONT);
    if read_u32(t + t2k_scaler::HINTER) == 0 {
        let create: unsafe extern "C" fn(usize, i32, i16, usize) -> i32 = core::mem::transmute(FN_GLYPH_HINT_CREATE);
        let err = create(mem, tt_font_get_max_glyph_points(font), read_i16(t + t2k_scaler::UNITS_PER_EM), t + t2k_scaler::HINTER);
        if err != 0 {
            lib_ctx_longjmp_error(mem, err);
        }
        let mode = (tt_font_get_glyph_count(font) > 999) as u32 + 1;
        write_u32(t + t2k_scaler::HINT_MODE, mode);
        if read_u32(font + 0x38) == 0 {
            load_hinter_font_params(t, font);
        }
        let set: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_GLYPH_HINT_SET_FONT_PARAMS);
        let err = set(read_usize(t + t2k_scaler::HINTER), read_usize(read_usize(t + t2k_scaler::FONT) + 0x38), read_u32(t + t2k_scaler::HINT_MODE));
        if err != 0 {
            lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), err);
        }
    }
    let (x, y) = (read_i32(t + t2k_scaler::X_PPEM), read_i32(t + t2k_scaler::Y_PPEM));
    if read_i32(t + t2k_scaler::HINTED_X_PPEM) != x || read_i32(t + t2k_scaler::HINTED_Y_PPEM) != y {
        let size: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_GLYPH_HINT_SET_SIZE);
        let err = size(read_usize(t + t2k_scaler::HINTER), x, y, t + t2k_scaler::SMALL);
        if err != 0 {
            lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), err);
        }
        let exec = read_usize(read_usize(t + t2k_scaler::FONT) + tt_font::EXEC);
        if exec != 0 {
            write_u8(t + t2k_scaler::SMALL, 0);
            tt_exec_set_ppem(exec, read_i32(t + t2k_scaler::X_PPEM), read_i32(t + t2k_scaler::Y_PPEM));
        }
        write_i32(t + t2k_scaler::HINTED_X_PPEM, read_i32(t + t2k_scaler::X_PPEM));
        write_i32(t + t2k_scaler::HINTED_Y_PPEM, read_i32(t + t2k_scaler::Y_PPEM));
    }
}

/// The hinter's font parameters (27 dwords) read from the T2KG table or measured, then kept in a block of the font.
unsafe fn load_hinter_font_params(t: usize, font: usize) {
    let mut params = [0u32; 27];
    let table = if read_u32(font + 0x3c) == 0 { tt_open_table_stream(font, TAG_T2KG) } else { 0 };
    if table != 0 {
        let read: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_GLYPH_HINT_READ_FONT_PARAMS);
        read(params.as_mut_ptr() as usize, table);
        t2k_stream_close(table);
    } else {
        let measure: unsafe extern "C" fn(usize, usize, usize, i32) = core::mem::transmute(FN_GLYPH_HINT_MEASURE_FONT_PARAMS);
        measure(read_usize(t + t2k_scaler::FONT), read_usize(t + t2k_scaler::HINTER), params.as_mut_ptr() as usize,
            (read_u32(t + t2k_scaler::HINT_MODE) == 2) as i32);
    }
    let block = lib_ctx_alloc(read_usize(t + t2k_scaler::MEM), 0x6c);
    write_usize(read_usize(t + t2k_scaler::FONT) + 0x38, block);
    for (k, v) in params.iter().enumerate() {
        write_u32(read_usize(read_usize(t + t2k_scaler::FONT) + 0x38) + 4 * k, *v);
    }
}

/// Frees the scaler's glyph memory by level: the current glyph always; from level 1 the bitmap (scratch slot 5 given
/// back when it is that slot's buffer) and the second bitmap; from level 100 also slot 5's buffer when larger than
/// 0x100 bytes; from level 2 the hinter (its sizes forgotten; its error longjmps).
/// Original: `t2k_purge_memory_internal` (0x93e7d0, library_without_asserts, T2K t2k.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_purge_memory_internal(t: usize, level: i32) {
    glyph_outline_free(read_usize(t + t2k_scaler::GLYPH));
    write_u32(t + t2k_scaler::GLYPH, 0);
    if level <= 0 {
        return;
    }
    let mem = read_usize(t + t2k_scaler::MEM);
    let slot5 = mem + t2k_ctx::SCRATCH + 0x14;
    if read_usize(t + t2k_scaler::BITMAP) == read_usize(slot5) {
        write_u32(mem + t2k_ctx::SCRATCH_FREE + 0x14, 1);
    } else {
        lib_ctx_free(mem, read_usize(t + t2k_scaler::BITMAP));
    }
    write_u32(t + t2k_scaler::BITMAP, 0);
    let mem = read_usize(t + t2k_scaler::MEM);
    if level > 99 && read_u32(mem + t2k_ctx::SCRATCH_SIZE + 0x14) > 0x100 {
        lib_ctx_free(mem, read_usize(mem + t2k_ctx::SCRATCH + 0x14));
        write_u32(read_usize(t + t2k_scaler::MEM) + t2k_ctx::SCRATCH + 0x14, 0);
        write_u32(read_usize(t + t2k_scaler::MEM) + t2k_ctx::SCRATCH_SIZE + 0x14, 0);
        write_u32(read_usize(t + t2k_scaler::MEM) + t2k_ctx::SCRATCH_FREE + 0x14, 1);
    }
    if read_u32(t + t2k_scaler::BITMAP_2) != 0 {
        lib_ctx_free(read_usize(t + t2k_scaler::MEM), read_usize(t + t2k_scaler::BITMAP_2));
        write_u32(t + t2k_scaler::BITMAP_2, 0);
    }
    if level > 1 {
        let destroy: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_GLYPH_HINT_DESTROY);
        let err = destroy(read_usize(t + t2k_scaler::HINTER));
        write_u32(t + t2k_scaler::HINTER, 0);
        write_i32(t + t2k_scaler::HINTED_Y_PPEM, -1);
        write_i32(t + t2k_scaler::HINTED_X_PPEM, -1);
        if err != 0 {
            lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), err);
        }
    }
}

/// Scales `n` font unit coordinates (16-bit) to 26.6 pixels (32-bit) with a scale record (`TtScaleRecord`): mode 0
/// `(v × multiplier + rounding) >> shift`; mode 1 `(v × multiplier + rounding) / divisor`, a negative product giving
/// `−((rounding − product) / divisor)`; mode 2 `fixed_mul(ratio, v)`.
/// Original: `t2k_scale_coordinates` (0x93f9a0, library_without_asserts, T2K t2k.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_scale_coordinates(from: usize, to: usize, n: i32, rec: usize) {
    let (mode, rounding, multiplier) = (read_i16(rec + 0x10), read_i32(rec + 8), read_i16(rec) as i32);
    let count = n.max(0) as usize;
    match mode {
        0 => {
            let shift = (read_i16(rec + 2) as u32) & 0x1f;
            for k in 0..count {
                write_i32(to + 4 * k, (read_i16(from + 2 * k) as i32).wrapping_mul(multiplier).wrapping_add(rounding) >> shift);
            }
        }
        1 => {
            let divisor = read_i32(rec + 4);
            for k in 0..count {
                let v = (read_i16(from + 2 * k) as i32).wrapping_mul(multiplier);
                let s = if v < 0 { (rounding.wrapping_sub(v) / divisor).wrapping_neg() } else { v.wrapping_add(rounding) / divisor };
                write_i32(to + 4 * k, s);
            }
        }
        2 => {
            let ratio = read_i32(rec + 0xc);
            for k in 0..count {
                write_i32(to + 4 * k, fixed_mul(ratio, read_i16(from + 2 * k) as i32));
            }
        }
        _ => {}
    }
}

/// The extra pixel filter (command bit 0x100): the bitmap is widened by four columns (the left bearing and the
/// advance moved back by two pixels, 0x80 in 26.6) into a new zeroed block, and each grey value v spreads over five
/// bytes with a = v>>3 − v>>6 and b = (v+1)>>1 − (v+15)>>5 − (v+4)>>3 − v>>6 as (a, b − a, b, b − a, a), in 8-bit
/// arithmetic; the old bitmap is freed (or scratch slot 5 given back).
/// Original: `t2k_apply_five_tap_bitmap_filter` (0x93fb70, library_without_asserts, T2K t2k.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_apply_five_tap_bitmap_filter(t: usize) {
    let (width, old_row, height) = (read_i32(t + t2k_scaler::WIDTH), read_i32(t + t2k_scaler::ROW_BYTES), read_i32(t + t2k_scaler::HEIGHT));
    let row = width.wrapping_add(4);
    write_i32(t + t2k_scaler::ROW_BYTES, row);
    write_i32(t + t2k_scaler::WIDTH, row);
    let size = row.wrapping_mul(height);
    write_i32(t + t2k_scaler::LEFT, read_i32(t + t2k_scaler::LEFT).wrapping_sub(0x80));
    write_i32(t + t2k_scaler::ADVANCE_X, read_i32(t + t2k_scaler::ADVANCE_X).wrapping_sub(0x80));
    let out = lib_ctx_alloc(read_usize(t + t2k_scaler::MEM), size as u32);
    if size > 0 {
        core::ptr::write_bytes(out as *mut u8, 0, size as usize);
    }
    let mut src = read_usize(t + t2k_scaler::BITMAP);
    let mut dst = out;
    for _ in 0..height.max(0) {
        for k in 0..width.max(0) as usize {
            let v = read_u8(src + k) as i32;
            if v != 0 {
                let c = (v >> 6) as u8;
                let a = ((v >> 3) as u8).wrapping_sub(c);
                let b = ((((v + 1) >> 1) as u8).wrapping_sub(((v + 0xf) >> 5) as u8).wrapping_sub(((v + 4) >> 3) as u8)).wrapping_sub(c);
                for (j, add) in [a, b.wrapping_sub(a), b, b.wrapping_sub(a), a].into_iter().enumerate() {
                    write_u8(dst + k + j, read_u8(dst + k + j).wrapping_add(add));
                }
            }
        }
        src = src.wrapping_add(old_row as usize);
        dst = dst.wrapping_add(row as usize);
    }
    let mem = read_usize(t + t2k_scaler::MEM);
    if read_usize(t + t2k_scaler::BITMAP) != read_usize(mem + t2k_ctx::SCRATCH + 0x14) {
        lib_ctx_free(mem, read_usize(t + t2k_scaler::BITMAP));
    } else {
        write_u32(mem + t2k_ctx::SCRATCH_FREE + 0x14, 1);
    }
    write_usize(t + t2k_scaler::BITMAP, out);
}

/// Comparers: side by side on the output for the scaling (pure); only the original for the rest (they allocate,
/// free or set up the hinter; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_t2k_set_up_hinting_for_size(t: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(t)
}

pub(crate) unsafe extern "C" fn compare_t2k_purge_memory_internal(t: usize, level: i32) {
    original_fn::<unsafe extern "C" fn(usize, i32)>(%%I1%%)(t, level)
}

pub(crate) unsafe extern "C" fn compare_t2k_scale_coordinates(from: usize, to: usize, n: i32, rec: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32, usize)>(%%I2%%);
    let bytes = if (0..0x10000).contains(&n) { 4 * n as usize } else { 0 };
    let (_, _, same, _, _) = crate::comparison::side_by_side(&[(to, bytes)], || o(from, to, n, rec), || t2k_scale_coordinates(from, to, n, rec));
    SCOREBOARD.record(same, || format!("t2k_scale_coordinates(n = {}, mode {}) differs", n, read_i16(rec + 0x10)));
}

pub(crate) unsafe extern "C" fn compare_t2k_apply_five_tap_bitmap_filter(t: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I3%%)(t)
}
