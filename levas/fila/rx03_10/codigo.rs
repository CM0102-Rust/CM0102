// ---------- rx03_10: the T2K glyph renderer: outline, composite glyphs, scaling, hinting and the bitmap (library_without_asserts, T2K t2k.c) ----------

/// The automatic hinter's outline fitting of the exe (rx01; the detour leads to the Rust copy):
/// (hinter, glyph view, mode, digit, curve order, unrounded, 4) -> error (−100: x only).
const FN_GLYPH_HINT_OUTLINE_AND_GET_ADVANCE: usize = 0x936510;
/// The renderer's errors.
const T2K_ERR_BAD_GREY_LEVEL: i32 = 0x2713;
const T2K_ERR_NO_GLYPH: i32 = 0x2715;
const T2K_ERR_TOO_MANY_POINTS: i32 = 0x2716;
/// The hinter's "fitted x only" result (y is then scaled plainly).
const HINT_X_ONLY: i32 = -100;

/// The glyph view the automatic hinter works on (`HintGlyph`, rx01_5): the counts and the glyph's arrays (the rest,
/// written by the hinter, starts zeroed here; the original's is stack memory).
#[repr(C)]
struct HintGlyphView {
    contours: i16,
    points: i16,
    starts: u32,
    ends: u32,
    oox: u32,
    ooy: u32,
    on_curve: u32,
    x: u32,
    y: u32,
    tail: [u32; 4],
}

/// Renders one glyph into the scaler (the work under `T2K_RenderGlyph`'s guard): the command bits decide the hinting
/// (0x80 TrueType instructions, kept only for a TrueType font at or above its smallest readable size; 0x281 the
/// automatic hinter; 0x200 its x only mode), 8 a glyph index instead of a character, 0x10 integer pixel offsets, 2 the
/// bitmap, 0x20 bounds only, 4 keep the glyph. The glyph is loaded (`tt_font_load_glyph`); a composite glyph renders
/// each component (recursively, with its scale or 2×2 matrix and its offset or point anchors, scaled to pixels when
/// they are an offset) and joins them (`glyph_outline_append_component`); the outline is scaled (`t2k_scale_coordinates`)
/// and hinted, its phantom points rounded and moved to the origin, the advances computed (transformed by the scaler's
/// matrix when it is not the identity), and with command bit 2 the scan converter builds the bitmap (dropout control
/// remembered per glyph in the scaler's bit sets).
/// Defect of the original: the column dropout is decided by the row dropout bit set (+0x10) too; the column bit set
/// (+0x14) is written but never read. The depth argument is counted up by each component of a composite glyph, so a
/// composite glyph at the top level skips the depth 0 step (its points are not moved to the origin nor its phantom
/// points rounded to whole pixels).
/// Original: `t2k_render_glyph_internal` (0x93e970, library_without_asserts, T2K t2k.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_render_glyph_internal(t: usize, code: i32, depth: i32, dx: i32, dy: i32, grey: i32, cmd: u32) {
    let mut cmd = cmd;
    let mut depth = depth;
    let mut tt_hint = cmd & 0x80;
    let mut digit = 0i32;
    let auto_hint;
    if tt_hint != 0 {
        let head = read_usize(read_usize(t + t2k_scaler::FONT) + tt_font::HEAD);
        if head == 0 || read_i16(head + 0x38) == 2000 || read_i16(head + 0x38) == 2001 {
            cmd |= 1;
            tt_hint = 0;
            auto_hint = cmd & 0x281 != 0;
        } else {
            cmd &= 0xfffe;
            let smallest = read_i16(head + 0x32) as i32;
            if read_i32(t + t2k_scaler::X_PPEM) < smallest || read_i32(t + t2k_scaler::Y_PPEM) < smallest {
                cmd &= 0xff7f;
                tt_hint = 0;
                auto_hint = cmd & 0x281 != 0;
            } else {
                auto_hint = false;
            }
        }
    } else {
        auto_hint = cmd & 0x281 != 0;
    }
    let mode: i16 = if cmd & 0x200 != 0 { 3 } else { 0 };
    if grey as u8 > 5 {
        lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), T2K_ERR_BAD_GREY_LEVEL);
    }
    let units = read_u16(t + t2k_scaler::UNITS_PER_EM);
    t2k_purge_memory_internal(t, 1);
    let font = read_usize(t + t2k_scaler::FONT);
    let (mut aw, mut ah) = (0u16, 0u16);
    let (awp, ahp) = (&mut aw as *mut u16 as usize, &mut ah as *mut u16 as usize);
    let mut g;
    if cmd & 8 != 0 {
        g = tt_font_load_glyph(font, code, tt_hint as i32, awp, ahp);
        digit = tt_font_is_digit_glyph(font, code);
        if depth == 0 {
            write_u16(t + 0xcc, code as u16);
        }
    } else {
        let index = tt_font_cmap_lookup(font, code as u32);
        g = tt_font_load_glyph(read_usize(t + t2k_scaler::FONT), index as i32, tt_hint as i32, awp, ahp);
        if depth == 0 {
            write_u16(t + 0xcc, read_u16(g + t2k_glyph::INDEX));
        }
        if (0x30..=0x39).contains(&code) {
            digit = 1;
        }
    }
    if g == 0 {
        lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), T2K_ERR_NO_GLYPH);
    }
    write_usize(t + t2k_scaler::GLYPH, g);
    let n;
    if read_i16(g + t2k_glyph::CONTOURS) < 0 {
        let (joined, all_flags, after) = render_components(t, g, depth, dx, dy, cmd, mode);
        g = joined;
        depth = after;
        n = read_i16(g + t2k_glyph::POINTS) as i32;
        let (oox, xs) = (read_usize(g + t2k_glyph::OOX), read_usize(g + t2k_glyph::X));
        if all_flags & 0x200 == 0 {
            write_i16(oox + 2 * n as usize, 0);
            let first = read_i16(oox + 2 * read_i16(read_usize(t + t2k_scaler::GLYPH) + t2k_glyph::POINTS) as usize);
            write_i16(oox + 2 * n as usize + 2, first.wrapping_add(aw as i16));
            let scaled = (read_i16(oox + 2 * n as usize + 2) as i32).wrapping_mul(read_i32(t + t2k_scaler::X_PPEM)).wrapping_mul(0x40)
                .wrapping_add((units >> 1) as i32) / units as i32;
            write_i32(xs + 4 * n as usize + 4, scaled);
            if cmd & 0x81 != 0 {
                write_i32(xs + 4 * n as usize + 4, read_i32(xs + 4 * n as usize + 4).wrapping_add(0x20) & !0x3f);
            }
            write_i32(xs + 4 * n as usize, 0);
        }
        if cmd & 0x80 != 0 || mode == 3 || (read_i32(t + t2k_scaler::HINT_MODE) == 2 && auto_hint) {
            let sx = fixed_div(units as i32, read_i32(t + t2k_scaler::X_PPEM).wrapping_shl(6));
            let sy = fixed_div(units as i32, read_i32(t + t2k_scaler::Y_PPEM).wrapping_shl(6));
            let ys = read_usize(g + t2k_glyph::Y);
            if n != -4 && n + 4 >= 0 {
                for k in 0..(n + 4) as usize {
                    let y = read_i32(ys + 4 * k);
                    write_i16(read_usize(g + t2k_glyph::OOX) + 2 * k, fixed_mul(read_i32(xs + 4 * k), sx) as i16);
                    write_i16(read_usize(g + t2k_glyph::OOY) + 2 * k, fixed_mul(y, sy) as i16);
                }
            }
            if tt_hint != 0 {
                tt_exec_hint_glyph(read_usize(read_usize(t + t2k_scaler::FONT) + tt_font::EXEC), g);
            }
            if auto_hint {
                t2k_set_up_hinting_for_size(t);
                let err = run_auto_hinter(t, g, n, mode, digit, grey);
                if err != 0 {
                    lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), err);
                }
            }
        }
    } else {
        n = read_i16(g + t2k_glyph::POINTS) as i32;
        scale_simple_glyph(t, g, n, tt_hint, auto_hint, cmd, dx, dy, mode, digit, grey);
    }
    finish_glyph(t, g, n, depth, cmd, aw, ah, tt_hint, grey);
}

/// The components of a composite glyph rendered and joined (see `t2k_render_glyph_internal`): returns the joined
/// glyph, the union of the component flags and the depth after the components (the original counts them in its
/// depth argument, which the end of the rendering then reads).
unsafe fn render_components(t: usize, g: usize, depth: i32, dx: i32, dy: i32, cmd: u32, mode: i16) -> (usize, u32, i32) {
    let mut depth = depth;
    let hints = read_usize(g + t2k_glyph::HINTS);
    let records = read_usize(g + t2k_glyph::COMPONENTS);
    let hint_length = read_u32(g + t2k_glyph::HINT_LENGTH);
    write_u32(g + t2k_glyph::COMPONENTS, 0);
    write_u32(g + t2k_glyph::HINTS, 0);
    write_u32(g + t2k_glyph::HINT_LENGTH, 0);
    let matrix = [read_i32(t + 8), read_i32(t + 0xc), read_i32(t + 0x10), read_i32(t + 0x14)];
    let (x_ppem, y_ppem) = (read_i32(t + t2k_scaler::X_PPEM), read_i32(t + t2k_scaler::Y_PPEM));
    let mut sub_cmd = ((cmd as u8 & 0x9d) | 0xc) as u32;
    if read_i32(t + t2k_scaler::HINT_MODE) == 2 || mode == 3 {
        sub_cmd &= 0xfdfe;
    }
    let mut joined = 0usize;
    let mut all_flags = 0u32;
    let mut p = records;
    loop {
        let flags = read_u16(p) as u32;
        all_flags |= flags;
        let glyph = read_u16(p + 2) as i32;
        p += 4;
        let w = read_i16(p) as i32;
        let (mut arg1, mut arg2);
        if flags & 1 != 0 {
            arg1 = w;
            arg2 = read_i16(p + 2) as i32;
            p += 4;
        } else {
            p += 2;
            if flags & 2 != 0 {
                arg2 = w as i8 as i32;
                arg1 = w >> 8;
            } else {
                arg2 = w & 0xff;
                arg1 = (w >> 8) & 0xff;
            }
        }
        if flags & 2 != 0 {
            let units = read_u16(t + t2k_scaler::UNITS_PER_EM) as i32;
            arg1 = (units >> 1).wrapping_add(arg1.wrapping_mul(read_i32(t + t2k_scaler::X_PPEM)).wrapping_shl(6)) / units;
            arg2 = (units >> 1).wrapping_add(arg2.wrapping_mul(read_i32(t + t2k_scaler::Y_PPEM)).wrapping_shl(6)) / units;
        }
        let word = |at: usize| (read_i16(at) as i32).wrapping_shl(2);
        let scale = if flags & 8 != 0 {
            let a = word(p);
            p += 2;
            Some([a, 0, 0, a])
        } else if flags & 0x40 != 0 {
            let (a, d) = (word(p), word(p + 2));
            p += 4;
            Some([a, 0, 0, d])
        } else if flags & 0x80 != 0 {
            let m = [word(p), word(p + 2), word(p + 4), word(p + 6)];
            p += 8;
            Some(m)
        } else {
            None
        };
        if let Some(m) = scale {
            set_scaler_matrix(t, m);
        }
        t2k_render_glyph_internal(t, glyph, depth, dx, dy, 0, sub_cmd);
        depth += 1;
        if scale.is_some() {
            set_scaler_matrix(t, matrix);
            write_i32(t + t2k_scaler::X_PPEM, x_ppem);
            write_i32(t + t2k_scaler::Y_PPEM, y_ppem);
        }
        glyph_outline_append_component(&mut joined as *mut usize as usize, read_usize(t + t2k_scaler::GLYPH), flags, arg1 as u32, arg2 as u32);
        if joined != read_usize(t + t2k_scaler::GLYPH) {
            glyph_outline_free(read_usize(t + t2k_scaler::GLYPH));
        }
        write_u32(t + t2k_scaler::GLYPH, 0);
        if flags & 0x20 == 0 {
            break;
        }
    }
    let g = joined;
    write_usize(t + t2k_scaler::GLYPH, g);
    let mem = read_usize(t + t2k_scaler::MEM);
    if read_usize(g + t2k_glyph::HINTS) == read_usize(mem + t2k_ctx::SCRATCH + 0xc) {
        write_u32(mem + t2k_ctx::SCRATCH_FREE + 0xc, 1);
    } else {
        lib_ctx_free(mem, read_usize(g + t2k_glyph::HINTS));
    }
    write_usize(g + t2k_glyph::HINTS, hints);
    write_u32(g + t2k_glyph::HINT_LENGTH, hint_length);
    lib_ctx_free(read_usize(t + t2k_scaler::MEM), records);
    (g, all_flags, depth)
}

/// The scaler's 2×2 matrix (16.16; +8..+0x14) and its identity flag (+0x1c).
unsafe fn set_scaler_matrix(t: usize, m: [i32; 4]) {
    for (k, v) in m.iter().enumerate() {
        write_i32(t + 8 + 4 * k, *v);
    }
    write_u32(t + 0x1c, (m[0] == 0x10000 && m[1] == 0 && m[2] == 0 && m[3] == m[0]) as u32);
}

/// The automatic hinter on the glyph (`glyph_hint_outline_and_get_advance`, through a view of its arrays); more than
/// 32000 points raise 0x2716.
unsafe fn run_auto_hinter(t: usize, g: usize, n: i32, mode: i16, digit: i32, grey: i32) -> i32 {
    if n > 32000 {
        lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), T2K_ERR_TOO_MANY_POINTS);
    }
    let mut view = HintGlyphView {
        contours: read_i16(g + t2k_glyph::CONTOURS),
        points: n as i16,
        starts: read_u32(g + t2k_glyph::STARTS),
        ends: read_u32(g + t2k_glyph::ENDS),
        oox: read_u32(g + t2k_glyph::OOX),
        ooy: read_u32(g + t2k_glyph::OOY),
        on_curve: read_u32(g + t2k_glyph::ON_CURVE),
        x: read_u32(g + t2k_glyph::X),
        y: read_u32(g + t2k_glyph::Y),
        tail: [0; 4],
    };
    let f: unsafe extern "C" fn(usize, usize, i32, i32, i32, i32, i32) -> i32 = core::mem::transmute(FN_GLYPH_HINT_OUTLINE_AND_GET_ADVANCE);
    f(read_usize(t + t2k_scaler::HINTER), &mut view as *mut HintGlyphView as usize, mode as i32, digit,
        read_i16(g + t2k_glyph::CURVE_ORDER) as i32, (grey as u8 != 0) as i32, 4)
}

/// A simple glyph's scaling and hinting (see `t2k_render_glyph_internal`).
#[allow(clippy::too_many_arguments)]
unsafe fn scale_simple_glyph(t: usize, g: usize, n: i32, tt_hint: u32, auto_hint: bool, cmd: u32, dx: i32, dy: i32, mode: i16, digit: i32, grey: i32) {
    let (xs, ys) = (read_usize(g + t2k_glyph::X), read_usize(g + t2k_glyph::Y));
    let scale_x = || t2k_scale_coordinates(read_usize(g + t2k_glyph::OOX), xs, n + 4, t + 0xfe);
    let scale_y = || t2k_scale_coordinates(read_usize(g + t2k_glyph::OOY), ys, n + 4, t + 0x110);
    if (tt_hint == 0 && !auto_hint) || cmd & 0x10 != 0 || n < 1 {
        scale_x();
        scale_y();
        if cmd & 0x10 != 0 {
            if dx as i8 != 0 {
                for k in 0..n.max(0) as usize {
                    write_i32(xs + 4 * k, read_i32(xs + 4 * k).wrapping_add(dx as i8 as i32));
                }
            }
            if dy as i8 != 0 {
                for k in 0..n.max(0) as usize {
                    write_i32(ys + 4 * k, read_i32(ys + 4 * k).wrapping_sub(dy as i8 as i32));
                }
            }
        }
        return;
    }
    if tt_hint != 0 {
        scale_x();
        scale_y();
        tt_exec_hint_glyph(read_usize(read_usize(t + t2k_scaler::FONT) + tt_font::EXEC), g);
    }
    if auto_hint {
        t2k_set_up_hinting_for_size(t);
        if n > 32000 {
            lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), T2K_ERR_TOO_MANY_POINTS);
        }
        scale_x();
        if mode != 3 {
            scale_y();
        }
        let err = run_auto_hinter(t, g, n, mode, digit, grey);
        if err == HINT_X_ONLY {
            scale_y();
        } else if err != 0 {
            lib_ctx_longjmp_error(read_usize(t + t2k_scaler::MEM), err);
        }
    }
}

/// The common end of the rendering (see `t2k_render_glyph_internal`): origin, advances, matrix, bitmap.
#[allow(clippy::too_many_arguments)]
unsafe fn finish_glyph(t: usize, g: usize, n: i32, depth: i32, cmd: u32, aw: u16, ah: u16, tt_hint: u32, grey: i32) {
    let (xs, ys) = (read_usize(g + t2k_glyph::X), read_usize(g + t2k_glyph::Y));
    let x = |k: i32| xs.wrapping_add((k as usize).wrapping_mul(4));
    let y = |k: i32| ys.wrapping_add((k as usize).wrapping_mul(4));
    write_i32(t + 0x9c, (aw as i32) << 16);
    write_u32(t + 0xa0, 0);
    write_u32(t + 0xb8, 0);
    write_i32(t + 0xbc, (ah as i32) << 16);
    if depth == 0 {
        for base in [xs, ys] {
            let at = |k: i32| base.wrapping_add((k as usize).wrapping_mul(4));
            let origin = read_i32(at(n));
            if origin != 0 && n + 4 > 0 {
                for k in 0..n + 4 {
                    write_i32(at(k), read_i32(at(k)).wrapping_sub(origin));
                }
            }
        }
        if cmd & 0x10 == 0 {
            let (right, left) = (read_i32(x(n + 1)), read_i32(x(n)));
            let lo = left.wrapping_add(0x20) & !0x3f;
            let hi = (right.wrapping_sub(left).wrapping_add(0x20) & !0x3f).wrapping_add(lo);
            write_i32(x(n), lo);
            write_i32(x(n + 1), hi);
            if cmd & 0x140 != 0 && n > 0 {
                let shift = hi.wrapping_sub(right).wrapping_sub(left).wrapping_add(lo) >> 1;
                if shift != 0 {
                    for k in 0..n {
                        write_i32(x(k), read_i32(x(k)).wrapping_add(shift));
                    }
                }
            }
        }
    }
    write_i32(t + 0x9c, fixed_mul(read_i32(t + 0x9c), read_i32(t + 0xf6)));
    write_i32(t + 0xbc, fixed_mul(read_i32(t + 0xbc), read_i32(t + 0xfa)));
    for k in n..n + 4 {
        write_i32(x(k), read_i32(x(k)).wrapping_shl(10));
        write_i32(y(k), read_i32(y(k)).wrapping_shl(10));
    }
    if read_u32(t + 0x1c) == 0 {
        transform_points(t, xs, ys, n);
    }
    write_i32(t + 0x94, read_i32(x(n + 1)).wrapping_sub(read_i32(x(n))));
    write_i32(t + 0x98, read_i32(y(n + 1)).wrapping_sub(read_i32(y(n))));
    write_i32(t + 0xb0, read_i32(x(n + 2)).wrapping_sub(read_i32(x(n + 3))));
    write_i32(t + 0xb4, read_i32(y(n + 2)).wrapping_sub(read_i32(y(n + 3))));
    for k in n..n + 4 {
        write_i32(x(k), read_i32(x(k)) >> 10);
        write_i32(y(k), read_i32(y(k)) >> 10);
    }
    for field in [0xde, 0xda, 0xd6, 0xce, 0xd2, 0xa4, 0xa8, 0xc0, 0xc4] {
        write_u32(t + field, 0);
    }
    write_u8(g + 0x70, 1);
    if read_u32(t + 0x1c) != 0 {
        let mut limit = 0xffu32;
        if tt_hint != 0 {
            let v = read_u32(read_usize(read_usize(t + t2k_scaler::FONT) + tt_font::EXEC) + 0x7c) & 0xff;
            if v != 0xff {
                limit = v;
            }
        }
        if (limit as i32) < read_i32(t + t2k_scaler::X_PPEM) {
            write_u8(g + 0x70, 0);
        }
    }
    if cmd & 2 != 0 && read_i16(g + t2k_glyph::POINTS) > 0 {
        render_bitmap(t, g, n, cmd, grey);
    }
    if cmd & 4 == 0 {
        t2k_purge_memory_internal(t, 0);
    }
}

/// The scaler's matrix applied to every point and to the advances (a diagonal matrix scales x and y apart).
unsafe fn transform_points(t: usize, xs: usize, ys: usize, n: i32) {
    let (a, b, c, d) = (read_i32(t + 8), read_i32(t + 0xc), read_i32(t + 0x10), read_i32(t + 0x14));
    for k in 0..(n + 4).max(0) as usize {
        let (xv, yv) = (read_i32(xs + 4 * k), read_i32(ys + 4 * k));
        if b == 0 && c == 0 {
            write_i32(xs + 4 * k, fixed_mul(a, xv));
            write_i32(ys + 4 * k, fixed_mul(d, yv));
        } else {
            write_i32(xs + 4 * k, fixed_mul(b, yv).wrapping_add(fixed_mul(a, xv)));
            write_i32(ys + 4 * k, fixed_mul(d, yv).wrapping_add(fixed_mul(c, xv)));
        }
    }
    let advance = read_i32(t + 0x9c);
    write_i32(t + 0x9c, fixed_mul(a, advance));
    write_i32(t + 0xa0, fixed_mul(c, advance));
    let vertical = read_i32(t + 0xbc);
    write_i32(t + 0xb8, fixed_mul(b, vertical));
    write_i32(t + 0xbc, fixed_mul(d, vertical));
}

/// The bitmap (command bit 2): the scan converter with the dropout flags (none at the glyph's size when the scaler's
/// bit set says this glyph needed none before), its bitmap and metrics taken by the scaler, the origin corrections.
unsafe fn render_bitmap(t: usize, g: usize, n: i32, cmd: u32, grey: i32) {
    let (xs, ys) = (read_usize(g + t2k_glyph::X), read_usize(g + t2k_glyph::Y));
    let mut sc = 0usize;
    let small = read_u8(t + t2k_scaler::SMALL) != 0 && cmd & 1 != 0 && read_i32(t + t2k_scaler::HINTED_X_PPEM) <= 0x18;
    if read_i32(g + 0xa) < 1 {
        let index = read_u16(t + 0xcc);
        let bitset = |field: usize| read_usize(read_usize(t + 0x18) + field) + (index >> 3) as usize;
        let mask = 1u8 << (index & 7);
        let mut column_dropout = read_u8(g + 0x70);
        let mut dropout = column_dropout;
        if grey as u8 == 0 && read_u32(t + 0x1c) != 0 && column_dropout != 0 {
            if index >= 0x800 || read_u8(bitset(0x10)) & mask != 0 {
                dropout = 0;
            }
            if index >= 0x800 || read_u8(bitset(0x10)) & mask != 0 {
                column_dropout = 0;
            }
        }
        sc = t2k_scan_converter_new(read_usize(t + t2k_scaler::MEM), read_i16(g + t2k_glyph::CONTOURS) as i32, read_usize(g + t2k_glyph::STARTS),
            read_usize(g + t2k_glyph::ENDS), xs, ys, read_usize(g + t2k_glyph::ON_CURVE), grey, read_i16(g + t2k_glyph::CURVE_ORDER) as i32,
            dropout as i32, column_dropout as i32, read_i32(t + 0x122));
        t2k_scan_convert_to_bitmap(sc, small as usize, (cmd & 0x20) as i32);
        write_i32(t + 0xce, read_i32(sc + 4).wrapping_sub(read_i32(sc)));
        write_i32(t + 0xd2, read_i32(sc + 0xc).wrapping_sub(read_i32(sc + 8)));
        write_i32(t + 0xa4, read_i32(sc + 0x10));
        write_i32(t + 0xa8, read_i32(sc + 0x14));
        write_i32(t + 0xc0, read_i32(sc + 0x10));
        write_i32(t + 0xc4, read_i32(sc + 0x14));
        write_i32(t + 0xd6, read_i32(sc + 0x18));
        if grey as u8 == 0 && read_u32(t + 0x1c) != 0 {
            if read_u32(sc + t2k_scan::ROW_DROPOUT_USED) == 0 && index < 0x800 {
                write_u8(bitset(0x10), read_u8(bitset(0x10)) | mask);
            }
            if read_u32(sc + t2k_scan::COLUMN_DROPOUT_USED) == 0 && index < 0x800 {
                write_u8(bitset(0x14), read_u8(bitset(0x14)) | mask);
            }
        }
    }
    let at = |a: usize, k: i32| read_i32(a + 4 * k as usize);
    write_i32(t + 0xc4, read_i32(t + 0xc4).wrapping_sub(at(xs, n + 2)));
    write_i32(t + 0xc0, read_i32(t + 0xc0).wrapping_sub(at(ys, n + 2)));
    if read_i32(g + 0xa) < 1 {
        write_u32(t + t2k_scaler::BITMAP, read_u32(sc + 0x1c));
        write_u32(sc + 0x1c, 0);
        write_u32(t + t2k_scaler::BITMAP_2, 0);
    }
    if at(xs, n) != 0 {
        write_i32(t + 0xa8, read_i32(t + 0xa8).wrapping_sub(at(xs, n)));
    }
    if at(ys, n) != 0 {
        write_i32(t + 0xa4, read_i32(t + 0xa4).wrapping_sub(at(ys, n)));
    }
    write_i32(t + 0xa8, read_i32(t + 0xa8) & !0x3f);
    write_i32(t + 0xa4, read_i32(t + 0xa4) & !0x3f);
    t2k_scan_converter_delete(sc);
}

/// Comparer: only the original (it loads, allocates, recurses and fills the bitmap; proved on the emulator by the
/// whole text path).
pub(crate) unsafe extern "C" fn compare_t2k_render_glyph_internal(t: usize, code: i32, depth: i32, dx: i32, dy: i32, grey: i32, cmd: u32) {
    original_fn::<unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32)>(%%I0%%)(t, code, depth, dx, dy, grey, cmd)
}
