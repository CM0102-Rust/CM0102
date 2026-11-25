// ---------- rx01_8: the T2K automatic hinter: the constraint list, the stem widths, the size and the hinter's life (library_without_asserts, T2K autogrid) ----------

/// The T2K memory manager (rx03 ports it; called here through its exe addresses).
const FN_LIB_MEM_ALLOC: usize = 0x9443a0; // (memory context, size) -> block
const FN_LIB_MEM_FREE: usize = 0x9445d0; // (memory context, block)

/// `lib_mem_alloc` (0x9443a0, cdecl): a block of `size` bytes from the library's memory context.
pub(crate) unsafe fn lib_mem_alloc(mem: usize, size: i32) -> usize {
    let f: unsafe extern "C" fn(usize, i32) -> usize = core::mem::transmute(FN_LIB_MEM_ALLOC);
    f(mem, size)
}

/// `lib_mem_free` (0x9445d0, cdecl): frees a block of the library's memory context.
pub(crate) unsafe fn lib_mem_free(mem: usize, block: usize) {
    let f: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_LIB_MEM_FREE);
    f(mem, block)
}

/// A hinter constraint (8 bytes, the list at +0x58): +0 the kind (1 a point on an alignment zone, 3 a stem), +1 the
/// axis class (2 across x, 1 across y, 0xff neither), +2 the side (1 forward, 0 backward; not written for kind 1),
/// +3 the priority (1 or 5; the list is sorted by it), +4 and +6 the two points (i16).
const CONSTRAINT_SIZE: usize = 8;

/// Builds the hinter's constraint list from the analysis: for each point with a stem partner (forward or backward), a
/// kind 1 record when it sits on an alignment zone (flag 0x40), a kind 3 record for the forward partner when the pair
/// is mutual, and one for the backward partner when mutual (a smooth point's backward partner only when its directions
/// are not almost the same); the backward records stop at the list's capacity (+0x54). Each record's axis class comes
/// from its side's direction at the second point (beyond ±0x3dd2 in x: 2; in y: 1; else 0xff); the list is bubble
/// sorted by priority and its length stored (16 bits) in `count`.
/// Original: `glyph_hint_build_sorted_constraint_list` (0x936060, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_build_sorted_constraint_list(h: GlyphHinter, list: usize, count: usize) {
    let capacity = read_i32(h.0 + 0x54);
    let mut n = 0i32;
    let record = |k: i32| list.wrapping_add((k as usize).wrapping_mul(CONSTRAINT_SIZE));
    let push = |n: &mut i32, kind: u8, side: Option<u8>, priority: u8, a: i32, b: i32| {
        let r = record(*n);
        write_u8(r, kind);
        if let Some(s) = side {
            write_u8(r + 2, s);
        }
        write_u8(r + 3, priority);
        write_i16(r + 4, a as i16);
        write_i16(r + 6, b as i16);
        *n += 1;
    };
    let mutual = |q: i32, p: i32| h.partner(true, q) == p || h.partner(false, q) == p;
    for c in 0..h.contour_count() as i32 {
        if n >= capacity {
            break;
        }
        let (first, last) = (h.contour_start(c), h.contour_end(c));
        if first >= last {
            continue;
        }
        for p in first..=last {
            let (forward, backward) = (h.partner(true, p), h.partner(false, p));
            if forward < 0 && backward < 0 {
                continue;
            }
            if read_u8(h.flags().wrapping_add((p as usize).wrapping_mul(2))) & 0x40 != 0 {
                push(&mut n, 1, None, 1, p, p);
            }
            if forward >= 0 && mutual(forward, p) {
                push(&mut n, 3, Some(1), 5, forward, p);
            }
            if backward >= 0 {
                let same_direction = backward == forward
                    && (h.backward_y(p).wrapping_mul(h.forward_y(p)).wrapping_add(h.backward_x(p).wrapping_mul(h.forward_x(p))) & !0x3fff) > 0xddb4000;
                if !same_direction && mutual(backward, p) {
                    push(&mut n, 3, Some(0), 5, backward, p);
                    if n >= capacity {
                        break;
                    }
                }
            }
        }
    }
    let beyond = |v: i32| v > 0x3dd2 || v < -0x3dd2;
    for k in 0..n {
        let r = record(k);
        let p = read_i16(r + 6) as i32;
        let (vx, vy) = if read_u8(r + 2) != 0 { (h.forward_x(p), h.forward_y(p)) } else { (h.backward_x(p), h.backward_y(p)) };
        write_u8(r + 1, if beyond(vx) { 2 } else if beyond(vy) { 1 } else { 0xff });
    }
    loop {
        let mut swapped = false;
        for k in 0..n - 1 {
            let (a, b) = (record(k), record(k + 1));
            if read_u8(a + 3) > read_u8(b + 3) {
                let (ra, rb) = (read_u64(a), read_u64(b));
                write_u64(a, rb);
                write_u64(b, ra);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    write_i32(count, n as i16 as i32);
}

/// Collects the widths of the glyph's stems (kind 3 constraints) up to a third of an em (16-bit): the axis class 1
/// ones measured in original x, class 2 in original y, into two new arrays of the library's memory (one entry to
/// spare each), given back with their counts. When an allocation fails both are freed, the outputs are empty and the
/// result is −1; otherwise 0.
/// Original: `glyph_hint_collect_stem_widths` (0x9362b0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_collect_stem_widths(h: GlyphHinter, g: HintGlyph, x_widths: usize, x_count: usize, y_widths: usize, y_count: usize) -> i32 {
    let limit = (h.units_per_em() as i32 / 3 + 1) as i16;
    let list = read_usize(h.0 + 0x58);
    let records = read_i32(h.0 + 0x50);
    // the width of record k on its axis, when it is a stem within the limit: (class, width)
    let width = |k: i32| -> Option<(u8, i16)> {
        let r = list.wrapping_add((k as usize).wrapping_mul(CONSTRAINT_SIZE));
        let class = read_u8(r + 1);
        if read_u8(r) != 3 || (class != 1 && class != 2) {
            return None;
        }
        let oo = if class == 1 { g.oox() } else { g.ooy() };
        let w = at_i16(oo, read_i16(r + 6) as i32).wrapping_sub(at_i16(oo, read_i16(r + 4) as i32)).wrapping_abs();
        (w <= limit).then_some((class, w))
    };
    let (mut nx, mut ny) = (0i32, 0i32);
    for k in 0..records {
        match width(k) {
            Some((1, _)) => nx += 1,
            Some(_) => ny += 1,
            None => {}
        }
    }
    let mem = read_usize(h.0 + 0x444);
    let mut xs = lib_mem_alloc(mem, nx * 2 + 2);
    let mut ys = lib_mem_alloc(mem, ny * 2 + 2);
    let (mut cx, mut cy, mut result) = (0i32, 0i32, 0);
    if xs == 0 || ys == 0 {
        result = -1;
        lib_mem_free(mem, xs);
        lib_mem_free(mem, ys);
        xs = 0;
        ys = 0;
    } else {
        for k in 0..read_i32(h.0 + 0x50) {
            match width(k) {
                Some((1, w)) => {
                    write_i16(xs + 2 * cx as usize, w);
                    cx += 1;
                }
                Some((_, w)) => {
                    write_i16(ys + 2 * cy as usize, w);
                    cy += 1;
                }
                None => {}
            }
        }
    }
    write_usize(x_widths, xs);
    write_i32(x_count, cx);
    write_usize(y_widths, ys);
    write_i32(y_count, cy);
    result
}

/// Stores the render mode and, when given, the font's hint parameters (27 dwords: the alignment zones and the stem
/// widths in font units) in a valid hinter and marks them new (+0x443); 0, or −1 for an invalid hinter.
/// Original: `glyph_hint_set_font_params` (0x9364c0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_set_font_params(h: GlyphHinter, params: usize, mode: i32) -> i32 {
    if !h.is_valid() {
        return -1;
    }
    write_i32(h.0 + 0x2a6, mode);
    if params != 0 {
        for k in 0..0x1b {
            write_u32(h.0 + 0x34e + 4 * k, read_u32(params + 4 * k));
        }
    }
    write_u8(h.0 + 0x443, 1);
    0
}

/// Hints a glyph's outline and keeps its advance: sets the unrounded mode from `unrounded`; in mode 0 first copies
/// the glyph's x and y (its points, `extra` more and the phantom points) into the hinter's scaled originals and keeps
/// the advance (the second phantom x minus the first) at +0x20; runs `glyph_hint_run_pass` in `mode` with `figure`;
/// then the hinted advance in whole pixels (rounded) at +0x24.
/// Original: `glyph_hint_outline_and_get_advance` (0x936510, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_outline_and_get_advance(h: GlyphHinter, g: HintGlyph, mode: i16, figure: i32, a5: i32, unrounded: i16, extra: i16) {
    write_i32(h.0 + 0x448, (unrounded != 0) as i32);
    let points = g.point_count() as i32;
    let advance = |coords: usize| at_i32(coords, points + 1).wrapping_sub(at_i32(coords, points));
    if mode == 0 {
        let n = extra as i32 + points;
        for k in 0..n {
            set_i32(h.x_scaled(), k, at_i32(g.x(), k));
        }
        write_i32(g.0 + 0x20, advance(g.x()));
        for k in 0..n {
            set_i32(h.y_scaled(), k, at_i32(g.y(), k));
        }
    }
    glyph_hint_run_pass(h, g, figure as i16, a5, mode, 0, 0, 0, 0);
    write_i32(g.0 + 0x24, advance(g.x()).wrapping_add(0x20) >> 6);
}

/// The hinter's work arrays: (field, block of the allocation, offset in it as a multiple of the point capacity).
const HINTER_ARRAYS: [(usize, usize, usize); 17] = [
    (0x3d0, 0, 0),
    (0x3c8, 1, 0), (0x3cc, 1, 4),
    (0x1c, 2, 0), (0x20, 2, 2), (0x24, 2, 4),
    (0x28, 3, 0),
    (0x2c, 4, 0), (0x30, 4, 2),
    (0x34, 5, 0), (0x38, 5, 2), (0x3c, 5, 4),
    (0x40, 6, 0), (0x44, 6, 4), (0x48, 6, 8), (0x4c, 6, 12),
    (0x58, 7, 0),
];
/// The size of each block as a multiple of the point capacity.
const HINTER_BLOCKS: [i32; 8] = [1, 8, 6, 2, 4, 6, 16, 16];
/// The fields that own a block (freed by `glyph_hint_destroy`, in this order).
const HINTER_OWNED: [usize; 8] = [0x3d0, 0x3c8, 0x1c, 0x28, 0x2c, 0x34, 0x40, 0x58];

/// Frees a valid hinter's blocks and the hinter itself.
unsafe fn free_hinter(h: GlyphHinter) {
    let mem = read_usize(h.0 + 0x444);
    for field in HINTER_OWNED {
        lib_mem_free(mem, read_usize(h.0 + field));
    }
    lib_mem_free(mem, h.0);
}

/// Creates a hinter for glyphs of up to `max_points` points (plus four phantom ones) and `units_per_em`: the context
/// (0x450 bytes) with its magic words, the default stem parameters (+0x3ba..+0x3c6: 2, 16, 16, 32, 0, 256, 0), mode 1,
/// the capacity, and its eight work blocks; 0 with the hinter in `out`. When the context cannot be had, −1 (and `out`
/// untouched); when a block cannot be had, everything is freed, `out` cleared and −1.
/// Original: `glyph_hint_create` (0x9365d0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_create(mem: usize, max_points: i32, units_per_em: i16, out: usize) -> i32 {
    let ctx = lib_mem_alloc(mem, GlyphHinter::SIZE as i32);
    if ctx == 0 {
        return -1;
    }
    let h = GlyphHinter(ctx);
    let n = max_points.wrapping_add(4);
    for (off, v) in [(0x3c2, 0), (0x3c6, 0), (0x3bc, 0x10), (0x3be, 0x10), (0x3ba, 2), (0x3c0, 0x20), (0x3c4, 0x100)] {
        write_u16(ctx + off, v);
    }
    write_usize(ctx + 0x444, mem);
    write_u32(ctx, HINTER_MAGIC);
    write_u32(ctx + 0x44c, HINTER_MAGIC_END);
    write_i32(ctx + 0x2a6, 1);
    write_u8(ctx + 0x442, 0);
    write_u8(ctx + 0x443, 0);
    write_i32(ctx + 0x34a, n);
    write_i16(ctx + 0x29c, units_per_em);
    let mut blocks = [0usize; 8];
    for (k, size) in HINTER_BLOCKS.iter().enumerate() {
        blocks[k] = lib_mem_alloc(mem, n.wrapping_mul(*size));
        for &(field, block, at) in HINTER_ARRAYS.iter().filter(|a| a.1 == k) {
            write_usize(ctx + field, blocks[block].wrapping_add((n as usize).wrapping_mul(at)));
        }
        if k == 6 {
            write_i32(ctx + 0x54, n.wrapping_mul(2)); // the constraint capacity
        }
    }
    let complete = [0x3d0, 0x3c8, 0x3cc, 0x1c, 0x28, 0x2c, 0x34, 0x40].iter().all(|&f| read_usize(ctx + f) != 0) && blocks[7] != 0;
    if complete {
        write_usize(out, ctx);
        return 0;
    }
    if h.is_valid() {
        free_hinter(h);
    }
    write_usize(out, 0);
    -1
}

/// Frees a hinter and its blocks: 0 (also for none), −1 when it is not valid.
/// Original: `glyph_hint_destroy` (0x936840, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_destroy(h: GlyphHinter) -> i32 {
    if h.0 == 0 {
        return 0;
    }
    if !h.is_valid() {
        return -1;
    }
    free_hinter(h);
    0
}

/// Sets a valid hinter's x and y pixels per em and rescales its zones and stem widths (`glyph_hint_scale_font_zones`
/// with the recomputation), its result; −1 for an invalid hinter.
/// Original: `glyph_hint_set_size` (0x9368e0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_set_size(h: GlyphHinter, x_ppem: i32, y_ppem: i32, small: usize) -> i32 {
    if !h.is_valid() {
        return -1;
    }
    write_i32(h.0 + 0x29e, x_ppem);
    write_i32(h.0 + 0x2a2, y_ppem);
    glyph_hint_scale_font_zones(h, 1, small)
}

/// `units` font units scaled to 26.6 at `ppem64` (pixels per em × 64), rounded to nearest.
fn scale_units(units: i32, ppem64: i32, upem: i32) -> i32 {
    units.wrapping_mul(ppem64).wrapping_add(upem >> 1) / upem
}

/// Scales the font's hint parameters for the current size. The 0x60 hint values (26.6) and their font-unit copies are
/// cleared; for each of the 10 alignment zones (from the last), its height (zone 2 takes zone 1's in mode 1 when they
/// lie within half a pixel) and overshoot are copied, and with `recompute` scaled: the height rounded to a pixel
/// (zone 3, the x height, 9 units higher in mode 1; zones 0-2 and 8 by the x height's rounding error), the overshoot
/// rounded down from +16 with its sign, added to the height. Then the four main heights (cap, x height, baseline,
/// descender) give the band scales and offsets that `glyph_hint_run_pass` mode 3 uses, and +0x346 marks heights out of
/// order. The 12 x and 12 y stem widths are copied and scaled (at least 0x21), and those within half a pixel of the
/// first one take its value. `small` gets whether the first x stem is under 1.5 pixels (never in the unrounded mode).
/// Original: `glyph_hint_scale_font_zones` (0x936930, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_scale_font_zones(h: GlyphHinter, recompute: i32, small: usize) -> i32 {
    let c = h.0;
    let xs = h.x_ppem() << 6;
    let ys = h.y_ppem() << 6;
    let upem = h.units_per_em() as i32;
    write_u8(c + 0x442, 1);
    let value = |k: usize| c + 0x5c + 4 * k;
    let units = |k: usize| c + 0x1dc + 2 * k;
    for k in 0..0x60 {
        write_i32(value(k), 0);
        write_i16(units(k), 0);
    }
    let zone = |k: usize, field: usize| read_i16(c + 0x34e + 6 * k + field) as i32;
    let mode1 = h.mode() == 1;
    let x_height = zone(3, 0);
    write_i16(units(6), x_height as i16);
    let scaled_x_height = scale_units(x_height, ys, upem);
    let rounded_x_height = round_to_pixel(if mode1 { scaled_x_height + 9 } else { scaled_x_height });
    write_i32(value(6), rounded_x_height);
    let x_height_error = rounded_x_height.wrapping_sub(scaled_x_height);
    for k in (0..10).rev() {
        let mut height = zone(k, 0);
        if k == 2 && mode1 && scale_units((zone(1, 0) - zone(2, 0)).wrapping_abs(), ys, upem) <= 0x20 {
            height = zone(1, 0);
        }
        write_i16(units(2 * k), height as i16);
        if recompute != 0 {
            let mut v = scale_units(height, ys, upem);
            if k == 3 {
                if mode1 {
                    v += 9;
                }
            } else if k < 3 || k == 8 {
                v = v.wrapping_add(x_height_error);
            }
            write_i32(value(2 * k), round_to_pixel(v));
        }
        let overshoot = zone(k, 4);
        write_i16(units(2 * k + 1), overshoot as i16);
        if recompute != 0 {
            let sign = if overshoot < 0 { -1 } else { 1 };
            let o = scale_units(overshoot.wrapping_abs(), ys, upem).wrapping_add(0x10) & !0x3f;
            write_i32(value(2 * k + 1), o.wrapping_mul(sign).wrapping_add(read_i32(value(2 * k))));
        }
    }
    set_height_bands(h, ys, upem);
    scale_stem_widths(h, 0x38a, 0x48, xs, upem, recompute);
    scale_stem_widths(h, 0x3a2, 0x54, ys, upem, recompute);
    let first_x_stem = read_i32(value(0x48));
    write_u8(small, (first_x_stem < 0x60) as u8);
    if h.unrounded() {
        write_u8(small, 0);
    }
    0
}

/// The four main heights (units 2, 6, 10, 14: cap, x height, baseline, descender) and the band table of
/// `glyph_hint_scale_font_zones`: for each, the height (+0x2da..), its exact scaling (+0x2aa..), its hinted value
/// (+0x2ae..) and their difference (+0x2b2..); the band scales (hinted span over the unit span, +0x31e..) and offsets
/// (+0x30a..), and +0x336.. the heights again; +0x346 set when they are out of order.
unsafe fn set_height_bands(h: GlyphHinter, ys: i32, upem: i32) {
    let c = h.0;
    for (k, unit_slot) in [2usize, 6, 10, 14].into_iter().enumerate() {
        let height = read_i16(c + 0x1dc + 2 * unit_slot);
        write_i16(c + 0x2da + 2 * k, height);
        let exact = (height as i32).wrapping_mul(ys) / upem;
        let hinted = read_i32(c + 0x5c + 4 * unit_slot);
        write_i32(c + 0x2aa + 12 * k, exact);
        write_i32(c + 0x2ae + 12 * k, hinted);
        write_i32(c + 0x2b2 + 12 * k, hinted.wrapping_sub(exact));
    }
    let height = |k: usize| read_i16(c + 0x2da + 2 * k) as i32;
    let hinted = |k: usize| read_i32(c + 0x2ae + 12 * k);
    write_i32(c + 0x2f6, upem);
    write_i32(c + 0x2e2, ys);
    write_i32(c + 0x30a, read_i32(c + 0x2b2));
    write_i32(c + 0x31a, read_i32(c + 0x2d6));
    write_i32(c + 0x2e6, hinted(0).wrapping_sub(hinted(1)));
    write_i32(c + 0x2f2, ys);
    write_i32(c + 0x306, upem);
    write_i32(c + 0x2fa, height(0) - height(1));
    write_i32(c + 0x2ea, hinted(1).wrapping_sub(hinted(2)));
    write_i32(c + 0x2fe, height(1) - height(2));
    write_i32(c + 0x2ee, hinted(2).wrapping_sub(hinted(3)));
    write_i32(c + 0x302, height(2) - height(3));
    for k in 0..5 {
        let scale = lib_fixed_div(read_i32(c + 0x2e2 + 4 * k), read_i32(c + 0x2f6 + 4 * k));
        write_i32(c + 0x31e + 4 * k, scale);
    }
    // the offsets of the three middle bands (between heights a and b): the average of the two hinted edges minus the
    // band's scaling of the two unit edges
    let middle = |band: usize, a: usize, b: usize, inner: usize| {
        let scaled = lib_fixed_mul(read_i32(c + 0x31e + 4 * band), height(a) + height(b));
        write_i32(c + 0x30a + 4 * band, (hinted(inner).wrapping_sub(scaled).wrapping_add(1).wrapping_add(hinted(if inner == a { b } else { a }))) >> 1);
    };
    middle(1, 1, 0, 1); // x height to cap height
    middle(2, 2, 1, 2); // baseline to x height
    middle(3, 2, 3, 2); // descender to baseline
    for k in 0..4 {
        write_i32(c + 0x336 + 4 * k, height(k));
    }
    let disordered = height(3) > height(2) || height(2) > height(1) || height(1) > height(0);
    write_i32(c + 0x346, disordered as i32);
}

/// Copies the 12 stem widths at `widths` into the unit slots from `first_slot` and, with `recompute`, scales the non-zero
/// ones (at least 0x21); then each later width within half a pixel of the first takes the first's value.
unsafe fn scale_stem_widths(h: GlyphHinter, widths: usize, first_slot: usize, ppem64: i32, upem: i32, recompute: i32) {
    let c = h.0;
    for k in 0..12 {
        let w = read_i16(c + widths + 2 * k);
        let slot = first_slot + k;
        write_i16(c + 0x1dc + 2 * slot, w);
        if recompute != 0 && w != 0 {
            write_i32(c + 0x5c + 4 * slot, scale_units(w as i32, ppem64, upem).max(0x21));
        }
    }
    let first = read_i32(c + 0x5c + 4 * first_slot);
    for k in 1..12 {
        let at = c + 0x5c + 4 * (first_slot + k);
        let v = read_i32(at);
        if v != 0 {
            write_i32(at, if first.wrapping_sub(v).wrapping_abs() <= 0x20 { first } else { v });
        }
    }
}

/// Comparers: only the original (they change the hinter or allocate; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_glyph_hint_build_sorted_constraint_list(h: GlyphHinter, list: usize, count: usize) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, usize, usize)>(%%I0%%)(h, list, count)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_collect_stem_widths(h: GlyphHinter, g: HintGlyph, xw: usize, xc: usize, yw: usize, yc: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, usize, usize, usize, usize) -> i32>(%%I1%%)(h, g, xw, xc, yw, yc)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_set_font_params(h: GlyphHinter, params: usize, mode: i32) -> i32 {
    original_fn::<unsafe extern "C" fn(GlyphHinter, usize, i32) -> i32>(%%I2%%)(h, params, mode)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_outline_and_get_advance(h: GlyphHinter, g: HintGlyph, mode: i16, figure: i32, a5: i32, unrounded: i16, extra: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32, i32, i16, i16)>(%%I3%%)(h, g, mode, figure, a5, unrounded, extra)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_create(mem: usize, max_points: i32, units_per_em: i16, out: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, i16, usize) -> i32>(%%I4%%)(mem, max_points, units_per_em, out)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_destroy(h: GlyphHinter) -> i32 {
    original_fn::<unsafe extern "C" fn(GlyphHinter) -> i32>(%%I5%%)(h)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_set_size(h: GlyphHinter, x_ppem: i32, y_ppem: i32, small: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(GlyphHinter, i32, i32, usize) -> i32>(%%I6%%)(h, x_ppem, y_ppem, small)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_scale_font_zones(h: GlyphHinter, recompute: i32, small: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(GlyphHinter, i32, usize) -> i32>(%%I7%%)(h, recompute, small)
}
