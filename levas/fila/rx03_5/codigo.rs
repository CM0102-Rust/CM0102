// ---------- rx03_5: the T2K TrueType hinting context: the scale factors, the font and control programs and the glyph program (library_without_asserts, T2K truetype.c) ----------

/// The T2K TrueType hinting context (0x14c bytes, `tt_exec_create`): +0 the memory context, +4 and +8 the pixel sizes
/// (x, y), +0xc the control values in font units (i16) and +0x10 their count, +0x14 the glyph work buffer, +0x18 the
/// interpreter's global state (`TtGlobalState`: +0x20 its scaled control values, +0xa4 the opcode table, +0xa8 and
/// +0xac the function and instruction definitions, +0xb0 the scale function, +0xb4 and +0xb8 the control and font
/// programs, +0xbc..+0xcc the scale record), +0xe4 the units per em, +0xe6 the twilight point count, +0xe8 and +0xec the
/// program lengths, +0xf0 the twilight zone (+0xf4..+0x11c its arrays) and +0x120 the glyph zone.
pub(crate) mod tt_exec {
    pub(crate) const MEM: usize = 0x0;
    pub(crate) const X_PPEM: usize = 0x4;
    pub(crate) const Y_PPEM: usize = 0x8;
    pub(crate) const CVT_UNITS: usize = 0xc;
    pub(crate) const CVT_COUNT: usize = 0x10;
    pub(crate) const WORK: usize = 0x14;
    pub(crate) const GLOBAL: usize = 0x18;
    pub(crate) const SCALE_FN: usize = 0xb0;
    pub(crate) const PREP: usize = 0xb4;
    pub(crate) const FPGM: usize = 0xb8;
    pub(crate) const UNITS_PER_EM: usize = 0xe4;
    pub(crate) const TWILIGHT_POINTS: usize = 0xe6;
    pub(crate) const PREP_LENGTH: usize = 0xe8;
    pub(crate) const FPGM_LENGTH: usize = 0xec;
    pub(crate) const TWILIGHT: usize = 0xf0;
    pub(crate) const TWILIGHT_ARRAYS: usize = 0xf4;
    pub(crate) const GLYPH_ZONE: usize = 0x120;
}

/// The scale record that `tt_compute_scale_factor` fills (0x12 bytes): +0 the multiplier (i16), +2 the shift (i16),
/// +4 the divisor, +8 the rounding, +0xc the 16.16 ratio, +0x10 the mode (0 shift, 1 division, 2 ratio, i16).
#[derive(Clone, Copy, Default)]
#[repr(C, packed)]
pub(crate) struct TtScaleRecord {
    multiplier: i16,
    shift: i16,
    divisor: i32,
    rounding: i32,
    ratio: i32,
    mode: i16,
}

/// The interpreter of the exe (it stays there: it calls setjmp): (zones, first byte, end, global state, trace hook).
const FN_TT_INTERPRETER_RUN: usize = 0x937090;
/// The input stream release of the exe (it stays there: setjmp with a status pointer; none here).
const FN_INPUT_STREAM_FREE: usize = 0x941760;
/// The scale functions and the round function, stored in the global state by their exe addresses (the interpreter
/// calls them through the state; the detours lead to the Rust copies).
const EXE_TT_SCALE_BY_SHIFT: u32 = 0x941e80;
const EXE_TT_SCALE_BY_DIVISION: u32 = 0x941ea0;
const EXE_TT_SCALE_BY_FIXED_MUL: u32 = 0x941ef0;
const EXE_TT_ROUND_TO_GRID: u32 = 0x936f10;
/// The tags of the control value, control and font programs.
pub(crate) const TAG_CVT: u32 = 0x63767420;
const TAG_PREP: u32 = 0x70726570;
const TAG_FPGM: u32 = 0x6670676d;

/// `tt_interpreter_run` (0x937090, cdecl): runs `[start, end)` over the zones with the global state.
unsafe fn t2k_run_program(zones: usize, start: usize, end: usize, global: usize) {
    let f: unsafe extern "C" fn(usize, usize, usize, usize, usize) = core::mem::transmute(FN_TT_INTERPRETER_RUN);
    f(zones, start, end, global, 0)
}

/// `input_stream_free` (0x941760, cdecl) without a status pointer.
pub(crate) unsafe fn t2k_stream_close(s: usize) {
    let f: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_INPUT_STREAM_FREE);
    f(s, 0)
}

/// Scales a font unit value by shifting: `(value × multiplier + rounding) >> shift` (the record at +0xa8 of the global
/// state; the shift's low byte, masked by the CPU).
/// Original: `tt_scale_by_shift` (0x941e80, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_scale_by_shift(g: usize, value: i32) -> i32 {
    read_i32(g + 0xa8).wrapping_mul(value).wrapping_add(read_i32(g + 0xb0)) >> (read_u8(g + 0xb4) & 0x1f)
}

/// Scales a font unit value by division, symmetric about zero: `(value × multiplier + rounding) / divisor`, a negative
/// value giving `−((rounding − value × multiplier) / divisor)` (truncated divisions).
/// Original: `tt_scale_by_division` (0x941ea0, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_scale_by_division(g: usize, value: i32) -> i32 {
    let (m, r, d) = (read_i32(g + 0xa8), read_i32(g + 0xb0), read_i32(g + 0xac));
    if value > -1 {
        m.wrapping_mul(value).wrapping_add(r) / d
    } else {
        (r.wrapping_sub(m.wrapping_mul(value)) / d).wrapping_neg()
    }
}

/// Scales a font unit value by the 16.16 ratio (+0xa4 of the global state, `fixed_mul`).
/// Original: `tt_scale_by_fixed_mul` (0x941ef0, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_scale_by_fixed_mul(g: usize, value: i32) -> i32 {
    fixed_mul(value, read_i32(g + 0xa4))
}

/// Builds the scale record from the pixel size and the units per em: with `n = size << 6` and `d = units`, both are
/// halved while both are even; the ratio is `fixed_div(n, d)` and the mode 2. When `n` fits a signed 16-bit value,
/// the multiplier is `n`, the rounding `d / 2`, the divisor `d`, the shift the number of trailing zero bits of `d`,
/// and the mode 0 (shift) when `d` is a power of two, else 1 (division); otherwise only the ratio and the mode are
/// written.
/// Defect of the original: with both values zero the halving never ends.
/// Original: `tt_compute_scale_factor` (0x942200, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_compute_scale_factor(size: i32, units: i32, out: usize) {
    let (mut n, mut d) = (size.wrapping_shl(6), units);
    while (n | d) & 1 == 0 {
        d >>= 1;
        n >>= 1;
    }
    write_i16(out + 0x10, 2);
    write_i32(out + 0xc, fixed_div(n, d));
    if n > -0x8000 && n < 0x8000 {
        write_i16(out, n as i16);
        let mut shift = 0u32;
        if d & 1 == 0 {
            loop {
                shift += 1;
                if (d >> (shift & 0x1f)) & 1 != 0 {
                    break;
                }
            }
        }
        write_i16(out + 2, shift as i16);
        write_i32(out + 4, d);
        write_i32(out + 8, d >> 1);
        write_i16(out + 0x10, ((d >> (shift & 0x1f)) != 1) as i16);
    }
}

/// Frees a hinting context and its buffers (the opcode table, the control values, the definitions, the programs,
/// the work buffer, the twilight arrays); nothing for null.
/// Original: `tt_exec_destroy` (0x942140, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_exec_destroy(e: usize) {
    if e == 0 {
        return;
    }
    for field in [0xa4, 0x18, 0x1c, 0x20, 0xa8, 0xac, 0xb4, 0xb8, 0x14, 0xc, 0xf4] {
        lib_ctx_free(read_usize(e), read_usize(e + field));
    }
    lib_ctx_free(read_usize(e), e);
}

/// Creates the hinting context of a TrueType font: the opcode table (`tt_init_opcode_table`), the interpreter stack
/// (maxp stack size × 4), the storage (zeroed), the work buffer (9 × the larger point limit + 0x24), the control
/// values (cvt, read in font units, scaled later), the function and instruction definitions, the control (prep) and
/// font (fpgm) programs read from the font stream, the default graphics state (round to grid, minimum distance 1
/// pixel, cut-ins 17/16 and 9/16 pixel...), the twilight zone (maxp twilight points); then the font program runs.
/// Pixel sizes start at −1 and 0x7fffa55a, so the first `tt_exec_set_ppem` always runs.
/// Original: `tt_exec_create` (0x941830, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_exec_create(mem: usize, s: usize, font: usize) -> usize {
    let e = lib_ctx_alloc(mem, 0x14c);
    let g = e + tt_exec::GLOBAL;
    let maxp = read_usize(font + tt_font::MAXP);
    write_usize(e, mem);
    write_u16(e + tt_exec::UNITS_PER_EM, tt_font_get_units_per_em(font) as u16);
    write_i32(e + tt_exec::X_PPEM, -1);
    write_i32(e + tt_exec::Y_PPEM, 0x7fffa55a);
    write_usize(g + 0x8c, lib_ctx_alloc(mem, 0x4c8));
    tt_init_opcode_table(TtGlobalState(g));
    write_usize(g, lib_ctx_alloc(mem, (read_i16(maxp + 0x1c) as i32).wrapping_shl(2) as u32));
    write_usize(g + 4, lib_ctx_alloc(mem, (read_i16(maxp + 0x16) as i32).wrapping_shl(2) as u32));
    for k in 0..(read_i16(maxp + 0x16) as i32).max(0) as usize {
        write_u32(read_usize(g + 4) + 4 * k, 0);
    }
    let points = (read_i16(maxp + 0xa) as i32).max(read_i16(maxp + 0xe) as i32);
    write_usize(e + tt_exec::WORK, lib_ctx_alloc(read_usize(e), points.wrapping_mul(9).wrapping_add(0x24) as u32));
    write_u32(g + 8, 0);
    write_u32(e + tt_exec::CVT_UNITS, 0);
    write_u32(e + tt_exec::CVT_COUNT, 0);
    let cvt = tt_find_table_by_tag(font, TAG_CVT);
    if cvt != 0 {
        read_control_values(e, s, cvt);
    }
    write_usize(g + 0x90, lib_ctx_alloc(mem, (read_i16(maxp + 0x18) as i32).wrapping_shl(3) as u32));
    write_usize(g + 0x94, lib_ctx_alloc(mem, (read_i16(maxp + 0x1a) as i32).wrapping_shl(3) as u32));
    for (k, tag) in [TAG_PREP, TAG_FPGM].into_iter().enumerate() {
        write_u32(e + tt_exec::PREP + 4 * k, 0);
        let table = tt_find_table_by_tag(font, tag);
        if table != 0 {
            input_stream_seek(s, read_u32(table + 0xc));
            let length = read_u32(table + 0x10);
            write_u32(e + tt_exec::PREP_LENGTH + 4 * k, length);
            let program = lib_ctx_alloc(mem, length);
            write_usize(e + tt_exec::PREP + 4 * k, program);
            input_stream_read_bytes(s, program, read_i32(e + tt_exec::PREP_LENGTH + 4 * k));
        }
    }
    write_default_graphics_state(g + 0x24);
    init_twilight_zone(e, mem, read_i16(maxp + 0x14));
    let fpgm = read_usize(e + tt_exec::FPGM);
    if fpgm != 0 {
        copy_graphics_state(g + 0x24, g + 0x58);
        write_u8(g + 0xc9, 1);
        write_u8(g + 0xc8, 1);
        t2k_run_program(e + tt_exec::TWILIGHT_ARRAYS, fpgm, fpgm + read_usize(e + tt_exec::FPGM_LENGTH), g);
    }
    e
}

/// The control values of the cvt table: a stream over it, the scaled array (4 bytes each) and the font unit array
/// (2 bytes each, read big-endian), `length / 2` values.
unsafe fn read_control_values(e: usize, s: usize, cvt: usize) {
    let mem = read_usize(e);
    let table = t2k_stream_open(mem, s, read_u32(cvt + 0xc), read_u32(cvt + 0x10));
    let n = read_u32(cvt + 0x10) >> 1;
    write_usize(e + tt_exec::GLOBAL + 8, lib_ctx_alloc(mem, n.wrapping_mul(4)));
    let units = lib_ctx_alloc(mem, n.wrapping_mul(2));
    write_usize(e + tt_exec::CVT_UNITS, units);
    write_u32(e + tt_exec::CVT_COUNT, n);
    for k in 0..n as usize {
        write_i16(units + 2 * k, input_stream_read_int16(table));
    }
    t2k_stream_close(table);
}

/// Copies a graphics state (13 dwords, the default one at +0x24 of the global state and the running one at +0x58).
unsafe fn copy_graphics_state(from: usize, to: usize) {
    core::ptr::copy(from as *const u8, to as *mut u8, 0x34);
}

/// The default graphics state (13 dwords at +0x24 of the global state, copied to +0x58 before each program).
unsafe fn write_default_graphics_state(gs: usize) {
    write_u32(gs + 0x18, EXE_TT_ROUND_TO_GRID);
    write_u32(gs + 0x14, 0x40);
    write_u32(gs, 0x44);
    write_u32(gs + 4, 0);
    write_u16(gs + 0x30, 0);
    write_u8(gs + 0x32, 1);
    write_u16(gs + 0x2a, 9);
    write_u16(gs + 0x2c, 3);
    write_u16(gs + 0x2e, 0x80);
    write_u32(gs + 0xc, 0);
    write_u32(gs + 0x10, 0);
}

/// The twilight zone: `n` points in one block of 0x16 bytes each (four 32-bit arrays, two 16-bit, two 8-bit), one
/// contour from 0 to 0 (+0xf0 and +0xf2), with its start and end array pointers at +0x114 and +0x118.
unsafe fn init_twilight_zone(e: usize, mem: usize, n: i16) {
    let n = n as i32;
    write_u16(e + tt_exec::TWILIGHT_POINTS, n as u16);
    let mut at = lib_ctx_alloc(mem, n.wrapping_mul(0x16) as u32);
    let (wide, narrow) = ((n as usize).wrapping_mul(4), (n as usize).wrapping_mul(2));
    write_u16(e + 0xf0, 0);
    write_u16(e + 0xf2, 0);
    for (field, step) in [(0xf4, wide), (0xf8, wide), (0xfc, wide), (0x100, wide), (0x104, narrow), (0x108, narrow), (0x10c, n as usize)] {
        write_usize(e + field, at);
        at = at.wrapping_add(step);
    }
    write_u16(e + 0x110, 1);
    write_usize(e + 0x11c, at);
    write_u16(e + 0x112, 0);
    write_usize(e + 0x114, e + 0xf0);
    write_usize(e + 0x118, e + 0xf2);
}

/// Sets the pixel sizes of a hinting context (nothing for null or unchanged sizes): the stretch ratios of the smaller
/// axis (`fixed_div`; square pixels flagged), the pixel size per em, the scale record from the larger size and the
/// units per em, the control values scaled by its mode (shift, division or `fixed_mul`) with the matching scale
/// function stored by its exe address, the twilight zone cleared; then the control program (prep) runs on a copy of
/// the default graphics state, which is put back after.
/// Defect of the original: with a size whose `size << 6` does not fit 16 bits, the multiplier, shift, divisor and
/// rounding copied into the state are stack bytes that `tt_compute_scale_factor` did not write (zero here).
/// Original: `tt_exec_set_ppem` (0x941ba0, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_exec_set_ppem(e: usize, x_ppem: i32, y_ppem: i32) {
    if e == 0 || (read_i32(e + tt_exec::X_PPEM) == x_ppem && read_i32(e + tt_exec::Y_PPEM) == y_ppem) {
        return;
    }
    let g = e + tt_exec::GLOBAL;
    write_i32(e + tt_exec::X_PPEM, x_ppem);
    write_i32(e + tt_exec::Y_PPEM, y_ppem);
    write_i32(g + 0xb8, 0x10000);
    write_i32(g + 0xbc, 0x10000);
    if x_ppem == y_ppem {
        write_u8(g + 0xb6, 1);
        write_i16(g + 0xc, x_ppem as i16);
    } else {
        write_u8(g + 0xb6, 0);
        if x_ppem > y_ppem {
            write_i16(g + 0xc, x_ppem as i16);
            write_i32(g + 0xbc, fixed_div(y_ppem, x_ppem));
        } else {
            write_i16(g + 0xc, y_ppem as i16);
            write_i32(g + 0xb8, fixed_div(x_ppem, y_ppem));
        }
    }
    let ppem = read_u16(g + 0xc);
    write_u32(g + 0x14, 0);
    write_u16(g + 0xe, ppem);
    write_u32(g + 0x18, 0);
    write_u32(g + 0x1c, 0);
    write_u32(g + 0x10, (ppem as u32) << 16);
    write_u32(g + 0x20, 0);
    let mut rec = TtScaleRecord::default();
    tt_compute_scale_factor(ppem as i32, read_u16(e + tt_exec::UNITS_PER_EM) as i32, &mut rec as *mut TtScaleRecord as usize);
    scale_control_values(e, &rec);
    write_i32(g + 0xa8, rec.multiplier as i32);
    write_i16(g + 0xb4, rec.shift);
    write_i32(g + 0xac, rec.divisor);
    write_i32(g + 0xb0, rec.rounding);
    write_i32(g + 0xa4, rec.ratio);
    clear_twilight_zone(e);
    let prep = read_usize(g + 0x9c);
    if prep != 0 {
        write_u8(g + 0xc9, 0);
        copy_graphics_state(g + 0x24, g + 0x58);
        write_u8(g + 0xc8, 1);
        write_u16(g + 0xca, 0);
        t2k_run_program(e + tt_exec::TWILIGHT_ARRAYS, prep, prep + read_usize(e + tt_exec::PREP_LENGTH), g);
        copy_graphics_state(g + 0x58, g + 0x24);
    }
}

/// The control values scaled by the record's mode, and the matching scale function stored (by its exe address).
unsafe fn scale_control_values(e: usize, rec: &TtScaleRecord) {
    let g = e + tt_exec::GLOBAL;
    let (n, units, scaled) = (read_i32(e + tt_exec::CVT_COUNT), read_usize(e + tt_exec::CVT_UNITS), read_usize(g + 8));
    let (m, shift, d, r, ratio) = (rec.multiplier as i32, rec.shift as i32, rec.divisor, rec.rounding, rec.ratio);
    let value = |k: i32| read_i16(units + 2 * k as usize) as i32;
    match rec.mode {
        0 => {
            write_u32(g + 0x98, EXE_TT_SCALE_BY_SHIFT);
            for k in 0..n.max(0) {
                write_i32(scaled + 4 * k as usize, value(k).wrapping_mul(m).wrapping_add(r) >> (shift & 0x1f));
            }
        }
        1 => {
            write_u32(g + 0x98, EXE_TT_SCALE_BY_DIVISION);
            for k in 0..n.max(0) {
                let v = value(k);
                let s = if v > -1 { m.wrapping_mul(v).wrapping_add(r) / d } else { (r.wrapping_sub(m.wrapping_mul(v)) / d).wrapping_neg() };
                write_i32(scaled + 4 * k as usize, s);
            }
        }
        2 => {
            write_u32(g + 0x98, EXE_TT_SCALE_BY_FIXED_MUL);
            for k in 0..n.max(0) {
                write_i32(scaled + 4 * k as usize, fixed_mul(value(k), ratio));
            }
        }
        _ => {}
    }
}

/// The twilight zone's points cleared (the four coordinate arrays, the two 16-bit and the two 8-bit ones).
unsafe fn clear_twilight_zone(e: usize) {
    let n = read_i16(e + tt_exec::TWILIGHT_POINTS) as i32;
    for k in 0..n.max(0) as usize {
        for field in [0xf4, 0xf8, 0xfc, 0x100] {
            write_u32(read_usize(e + field) + 4 * k, 0);
        }
        write_u16(read_usize(e + 0x104) + 2 * k, 0);
        write_u16(read_usize(e + 0x108) + 2 * k, 0);
        write_u8(read_usize(e + 0x10c) + k, 0);
        write_u8(read_usize(e + 0x11c) + k, 0);
    }
}

/// Runs a glyph's instructions (nothing without a context, instructions or a positive length): the glyph zone points
/// at the glyph's arrays (the current coordinates copied into the work buffer, the touch flags cleared), the original
/// coordinates stretched by the pixel ratios when not square (`fixed_mul`, 16-bit), the first two phantom points
/// rounded to the pixel (x, keeping their distance rounded) and the next two (y); then the glyph program runs on a copy
/// of the graphics state.
/// Original: `tt_exec_hint_glyph` (0x941f10, library_without_asserts, T2K truetype.c, cdecl).
pub(crate) unsafe extern "C" fn tt_exec_hint_glyph(e: usize, glyph: usize) {
    if e == 0 || read_u32(glyph + 0x60) == 0 || read_i32(glyph + 0x64) <= 0 {
        return;
    }
    let n = read_i16(glyph + 0x36) as i32 + 4;
    let work = read_usize(e + tt_exec::WORK);
    let (old_x, old_y, flags) = (work, work + 4 * n as usize, work + 8 * n as usize);
    write_u32(e + 0x120, read_u32(glyph + 0x4c));
    write_u32(e + 0x124, read_u32(glyph + 0x50));
    write_usize(e + 0x128, old_x);
    write_usize(e + 0x12c, old_y);
    write_u32(e + 0x130, read_u32(glyph + 0x40));
    write_u32(e + 0x134, read_u32(glyph + 0x44));
    write_u32(e + 0x138, read_u32(glyph + 0x48));
    write_u16(e + 0x13c, read_u16(glyph + 0x34));
    write_u16(e + 0x13e, 0);
    write_u32(e + 0x140, read_u32(glyph + 0x38));
    write_u32(e + 0x144, read_u32(glyph + 0x3c));
    write_usize(e + 0x148, flags);
    for k in 0..n.max(0) as usize {
        write_u32(old_x + 4 * k, read_u32(read_usize(glyph + 0x4c) + 4 * k));
        write_u8(flags + k, 0);
    }
    for k in 0..n.max(0) as usize {
        write_u32(old_y + 4 * k, read_u32(read_usize(glyph + 0x50) + 4 * k));
    }
    stretch_originals(read_usize(e + 0x134), read_i32(e + tt_exec::GLOBAL + 0xbc), n);
    stretch_originals(read_usize(e + 0x130), read_i32(e + tt_exec::GLOBAL + 0xb8), n);
    let last = read_i16(glyph + 0x36) as usize;
    round_phantom_pair(read_usize(glyph + 0x4c) + 4 * last);
    round_phantom_pair(read_usize(glyph + 0x50) + 4 * last + 8);
    let g = e + tt_exec::GLOBAL;
    copy_graphics_state(g + 0x24, g + 0x58);
    write_u8(g + 0xc9, 0);
    write_u8(g + 0xc8, 0);
    let code = read_usize(glyph + 0x60);
    t2k_run_program(e + tt_exec::TWILIGHT_ARRAYS, code, code + read_usize(glyph + 0x64), g);
}

/// The original coordinates (16-bit) stretched by a ratio other than 1.0 (`fixed_mul`).
unsafe fn stretch_originals(values: usize, ratio: i32, n: i32) {
    if ratio == 0x10000 {
        return;
    }
    for k in 0..n.max(0) as usize {
        write_i16(values + 2 * k, fixed_mul(read_i16(values + 2 * k) as i32, ratio) as i16);
    }
}

/// Two phantom coordinates: the first rounded to the pixel, the second at the first plus their distance rounded.
unsafe fn round_phantom_pair(at: usize) {
    let (a, b) = (read_i32(at), read_i32(at + 4));
    write_i32(at, a.wrapping_add(0x20) & !0x3f);
    write_i32(at + 4, read_i32(at).wrapping_add(b.wrapping_sub(a).wrapping_add(0x20) & !0x3f));
}

/// Comparers: value against value for the scale functions; side by side on the record for the scale factor; only the
/// original for the rest (they allocate, free or run programs; proved on the emulator).
unsafe fn compare_tt_scaler(name: &str, original: unsafe extern "C" fn(usize, i32) -> i32, copy: unsafe extern "C" fn(usize, i32) -> i32, g: usize, v: i32) -> i32 {
    let (r, c) = (original(g, v), copy(g, v));
    SCOREBOARD.record(r == c, || format!("{}({}) original={} rust={}", name, v, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_scale_by_shift(g: usize, v: i32) -> i32 {
    compare_tt_scaler("tt_scale_by_shift", original_fn::<unsafe extern "C" fn(usize, i32) -> i32>(%%I0%%), tt_scale_by_shift, g, v)
}

pub(crate) unsafe extern "C" fn compare_tt_scale_by_division(g: usize, v: i32) -> i32 {
    compare_tt_scaler("tt_scale_by_division", original_fn::<unsafe extern "C" fn(usize, i32) -> i32>(%%I1%%), tt_scale_by_division, g, v)
}

pub(crate) unsafe extern "C" fn compare_tt_scale_by_fixed_mul(g: usize, v: i32) -> i32 {
    compare_tt_scaler("tt_scale_by_fixed_mul", original_fn::<unsafe extern "C" fn(usize, i32) -> i32>(%%I2%%), tt_scale_by_fixed_mul, g, v)
}

pub(crate) unsafe extern "C" fn compare_tt_compute_scale_factor(size: i32, units: i32, out: usize) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32, usize)>(%%I3%%);
    let (_, _, same, _, _) = crate::comparison::side_by_side_light(&[(out, 0x12)], || o(size, units, out), || tt_compute_scale_factor(size, units, out));
    SCOREBOARD.record(same, || format!("tt_compute_scale_factor({}, {}) differs", size, units));
}

pub(crate) unsafe extern "C" fn compare_tt_exec_destroy(e: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I4%%)(e)
}

pub(crate) unsafe extern "C" fn compare_tt_exec_create(mem: usize, s: usize, font: usize) -> usize {
    original_fn::<unsafe extern "C" fn(usize, usize, usize) -> usize>(%%I5%%)(mem, s, font)
}

pub(crate) unsafe extern "C" fn compare_tt_exec_set_ppem(e: usize, x_ppem: i32, y_ppem: i32) {
    original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I6%%)(e, x_ppem, y_ppem)
}

pub(crate) unsafe extern "C" fn compare_tt_exec_hint_glyph(e: usize, glyph: usize) {
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I7%%)(e, glyph)
}
