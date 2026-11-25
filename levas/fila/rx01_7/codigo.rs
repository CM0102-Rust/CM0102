// ---------- rx01_7: the T2K automatic hinter: the analysis pass (contour links, directions, extrema, stem edges) (library_without_asserts, T2K autogrid) ----------

/// `glyph_hint_fit_outline_both_axes` (0x93c680, rx03): the fitting of a glyph after the analysis (cdecl: glyph, hinter).
const FN_GLYPH_HINT_FIT_OUTLINE_BOTH_AXES: usize = 0x93c680;

/// Flag word `p` of the hinter's per-point flags.
unsafe fn hint_flags(h: GlyphHinter, p: i32) -> u16 {
    read_u16(h.flags().wrapping_add((p as usize).wrapping_mul(2)))
}

unsafe fn set_hint_flags(h: GlyphHinter, p: i32, v: u16) {
    write_u16(h.flags().wrapping_add((p as usize).wrapping_mul(2)), v)
}

/// The analysis pass for the stem widths (mode 2 of `glyph_hint_run_pass`), when the hinter is valid; −1 otherwise.
/// Original: `glyph_hint_collect_stem_widths_checked` (0x934b10, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_collect_stem_widths_checked(h: GlyphHinter, g: HintGlyph, figure: i16, a4: i32, x_widths: usize, x_count: usize, y_widths: usize, y_count: usize) -> i32 {
    if !h.is_valid() {
        return -1;
    }
    glyph_hint_run_pass(h, g, figure, a4, 2, x_widths, x_count, y_widths, y_count)
}

/// Runs the hinter on a glyph. The glyph's contours and original points are copied into the hinter (+4 to +0x16) and
/// `figure` kept at +0x1a.
/// - Mode 3 (the vertical scaling only): with the zones out of order (+0x346) −100; otherwise each original y
///   (points and the four phantom points) goes through the scale and offset of its band (above the cap height, between
///   it and the x height, between the x height and the baseline, below the baseline, below the descender) into the
///   hinted y; 0.
/// - Other modes: the contour links (next and previous, a one-point contour links to itself), the directions
///   (`glyph_hint_outline_compute_point_tangents`), the extrema (`glyph_hint_outline_flag_extrema_points`) and the stem
///   edges (`glyph_hint_outline_pair_stem_edges`); each candidate point (flag 2) that falls on an alignment zone gets
///   flag 0x40; the constraint list is built. Then mode 0 fits the outline (`glyph_hint_fit_outline_both_axes`, its
///   result), mode 2 collects the stem widths (its result), mode 1 returns 0 and any other −1.
/// Original: `glyph_hint_run_pass` (0x934b60, library_without_asserts, T2K autogrid, cdecl; the 4th argument is not
/// read).
pub(crate) unsafe extern "C" fn glyph_hint_run_pass(h: GlyphHinter, g: HintGlyph, figure: i16, _a4: i32, mode: i16, x_widths: usize, x_count: usize, y_widths: usize, y_count: usize) -> i32 {
    write_i16(h.0 + 0x1a, figure);
    write_i16(h.0 + 4, read_i16(g.0));
    write_usize(h.0 + 6, read_usize(g.0 + 4));
    write_usize(h.0 + 0xa, read_usize(g.0 + 8));
    write_usize(h.0 + 0xe, read_usize(g.0 + 0x14));
    write_usize(h.0 + 0x12, g.oox());
    write_usize(h.0 + 0x16, g.ooy());
    if mode == 3 {
        if read_i32(h.0 + 0x346) != 0 {
            return -100;
        }
        scale_vertically(h, g);
        return 0;
    }
    write_i32(h.0 + 0x50, 0);
    link_contours(h);
    glyph_hint_outline_compute_point_tangents(h);
    glyph_hint_outline_flag_extrema_points(h);
    glyph_hint_outline_pair_stem_edges(h);
    for c in 0..h.contour_count() as i32 {
        let (first, last) = (h.contour_start(c), h.contour_end(c));
        if first < last {
            for p in first..=last {
                let f = h.flags().wrapping_add((p as usize).wrapping_mul(2));
                if read_u8(f) & 2 != 0 && glyph_hint_find_nearest_alignment_zone(h, p) >= 0 {
                    write_u8(f, read_u8(f) | 0x40);
                }
            }
        }
    }
    glyph_hint_build_sorted_constraint_list(h, read_usize(h.0 + 0x58), h.0 + 0x50);
    match mode {
        0 => {
            let fit: unsafe extern "C" fn(HintGlyph, GlyphHinter) -> i32 = core::mem::transmute(FN_GLYPH_HINT_FIT_OUTLINE_BOTH_AXES);
            fit(g, h)
        }
        2 => glyph_hint_collect_stem_widths(h, g, x_widths, x_count, y_widths, y_count),
        1 => 0,
        _ => -1,
    }
}

/// Mode 3 of `glyph_hint_run_pass`: every original y (with the phantom points) through the scale (+0x31e) and offset
/// (+0x30a) of its height band, by the zone heights at +0x2da..+0x2e0.
unsafe fn scale_vertically(h: GlyphHinter, g: HintGlyph) {
    let (h0, h1, h2, h3) = (read_i16(h.0 + 0x2da) as i32, read_i16(h.0 + 0x2dc) as i32, read_i16(h.0 + 0x2de) as i32, read_i16(h.0 + 0x2e0) as i32);
    let (oo, out) = (g.ooy(), g.y());
    for p in 0..g.point_count() as i32 + 4 {
        let v = at_i16(oo, p) as i32;
        let band: usize = if v < h1 {
            if v < h2 {
                if v > h3 { 3 } else { 4 }
            } else {
                2
            }
        } else if v < h0 {
            1
        } else {
            0
        };
        let scaled = lib_fixed_mul(read_i32(h.0 + 0x31e + 4 * band), v);
        set_i32(out, p, scaled.wrapping_add(read_i32(h.0 + 0x30a + 4 * band)));
    }
}

/// The next and previous links of every contour (+0x1c, +0x20): a closed ring from its first to its last point; a
/// contour of one point (or reversed) links its first point to itself.
unsafe fn link_contours(h: GlyphHinter) {
    let (next, prev) = (read_usize(h.0 + 0x1c), read_usize(h.0 + 0x20));
    let set = |arr: usize, p: i32, v: i32| write_i16(arr.wrapping_add((p as usize).wrapping_mul(2)), v as i16);
    for c in 0..h.contour_count() as i32 {
        let (first, last) = (h.contour_start(c), h.contour_end(c));
        if first < last {
            let mut before = last;
            for p in first..=last {
                set(prev, p, before);
                set(next, p, p + 1);
                before = p;
            }
            set(next, last, first);
        } else {
            set(prev, first, first);
            set(next, first, first);
        }
    }
}

/// Computes, for every point of every contour, the unit direction toward the next point (+0x40/+0x44) and from the
/// previous one (+0x48/+0x4c), and the point's position for the stem search (+0x2c/+0x30). An on-curve point uses its
/// neighbours directly (its own position; the backward direction copies the forward one when either axis has no
/// length); an off-curve point uses the midpoints toward off-curve neighbours (16-bit, rounded up), and both of its
/// directions are the chord between the two half-way points, its position their middle.
/// Original: `glyph_hint_outline_compute_point_tangents` (0x934e00, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_outline_compute_point_tangents(h: GlyphHinter) {
    let (oox, ooy) = (h.oox(), h.ooy());
    let half16 = |a: i16, b: i16| ((a.wrapping_add(b)).wrapping_add(1)) >> 1;
    for c in 0..h.contour_count() as i32 {
        let (first, last) = (h.contour_start(c), h.contour_end(c));
        if first >= last {
            continue;
        }
        let (mut px, mut py) = (at_i16(oox, last), at_i16(ooy, last));
        let mut previous = last;
        for p in first..=last {
            let n = h.next(p);
            let (x, y) = (at_i16(oox, p), at_i16(ooy, p));
            let (mut nx, mut ny) = (at_i16(oox, n), at_i16(ooy, n));
            let (mut fx, mut fy, mut bx, mut by, mx, my);
            if h.on_curve(p) {
                (fx, fy) = (nx as i32 - x as i32, ny as i32 - y as i32);
                (bx, by) = (x as i32 - px as i32, y as i32 - py as i32);
                mx = x;
                my = y;
            } else {
                let (mut qx, mut qy) = (px, py);
                if !h.on_curve(n) {
                    nx = half16(nx, x);
                    ny = half16(ny, y);
                }
                if !h.on_curve(previous) {
                    qx = half16(qx, x);
                    qy = half16(qy, y);
                }
                let a = ((nx as i32 + x as i32 + 1) >> 1) as i16 as i32;
                let b = ((qx as i32 + x as i32 + 1) >> 1) as i16 as i32;
                let cy = ((ny as i32 + y as i32 + 1) >> 1) as i16 as i32;
                let dy = ((qy as i32 + y as i32 + 1) >> 1) as i16 as i32;
                (fx, fy) = (a - b, cy - dy);
                (bx, by) = (fx, fy);
                mx = ((a + b + 1) >> 1) as i16;
                my = ((cy + dy + 1) >> 1) as i16;
            }
            fixed_normalise_vector_2dot14(&mut fx, &mut fy);
            set_i32(read_usize(h.0 + 0x40), p, fx);
            set_i32(read_usize(h.0 + 0x44), p, fy);
            let degenerate = (bx == 0 && fx == 0) || (by == 0 && fy == 0);
            if !h.on_curve(p) || degenerate {
                set_i32(read_usize(h.0 + 0x48), p, h.forward_x(p));
                set_i32(read_usize(h.0 + 0x4c), p, h.forward_y(p));
            } else {
                fixed_normalise_vector_2dot14(&mut bx, &mut by);
                set_i32(read_usize(h.0 + 0x48), p, bx);
                set_i32(read_usize(h.0 + 0x4c), p, by);
            }
            write_i16(read_usize(h.0 + 0x2c).wrapping_add((p as usize).wrapping_mul(2)), mx);
            write_i16(read_usize(h.0 + 0x30).wrapping_add((p as usize).wrapping_mul(2)), my);
            previous = p;
            px = x;
            py = y;
        }
    }
}

/// Normalises the vector (*x, *y) in place to unit length in 2.14: an axis vector becomes ±0x4000 on its axis;
/// otherwise both parts are first scaled by an estimate of the length (the larger magnitude plus half the smaller),
/// then divided by the integer square root (Newton steps from 0x4000) of their squared length.
/// Original: `fixed_normalise_vector_2dot14` (0x935080, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn fixed_normalise_vector_2dot14(x: &mut i32, y: &mut i32) {
    let unit = |v: i32| if v < 0 { -0x4000 } else { 0x4000 };
    if *x == 0 {
        *y = unit(*y);
        return;
    }
    if *y == 0 {
        *x = unit(*x);
        return;
    }
    let (mut ax, mut ay) = (x.wrapping_abs(), y.wrapping_abs());
    if ax > ay {
        ay >>= 1;
    } else {
        ax >>= 1;
    }
    let estimate = ax.wrapping_add(ay);
    let nx = (*x << 14) / estimate;
    let ny = (*y << 14) / estimate;
    let squared = ny.wrapping_mul(ny).wrapping_add(nx.wrapping_mul(nx));
    let mut root = 0x4000;
    loop {
        let next = (squared / root).wrapping_add(root).wrapping_add(1) >> 1;
        if next == root {
            break;
        }
        root = next;
    }
    *x = (nx << 14) / root;
    *y = (ny << 14) / root;
}

/// The curvature at point `p` between `before` and `after` (original coordinates): the cross product of the two
/// chords divided by a 16-bit length estimate of their sum, plus one.
unsafe fn turn_at(h: GlyphHinter, before: i32, p: i32, after: i32) -> i32 {
    let (oox, ooy) = (h.oox(), h.ooy());
    let (x, y) = (at_i16(oox, p) as i32, at_i16(ooy, p) as i32);
    let (dx_in, dy_in) = (x - at_i16(oox, before) as i32, y - at_i16(ooy, before) as i32);
    let (dx_out, dy_out) = (at_i16(oox, after) as i32 - x, at_i16(ooy, after) as i32 - y);
    let cross = dy_out.wrapping_mul(dx_in).wrapping_sub(dx_out.wrapping_mul(dy_in));
    let sx = (dx_in.wrapping_add(dx_out) as i16).wrapping_abs();
    let sy = (dy_in.wrapping_add(dy_out) as i16).wrapping_abs();
    let (small, big) = if sx > sy { (sy, sx) } else { (sx, sy) };
    let length = ((small as i32 >> 1) + big as i32) as i16 as i32;
    cross / (length + 1)
}

/// Marks the local extrema of one axis along a contour (flag `flag`): the first point of each run of equal values
/// that is above both neighbouring runs, or below both (the previous run found walking back at most 0x81 points).
unsafe fn flag_axis_extrema(h: GlyphHinter, first: i32, coords: usize, flag: u8) {
    let value = |p: i32| at_i16(coords, p);
    let start = value(first);
    let mut q = h.prev(first);
    let mut budget = 0x80;
    while value(q) == start && q != first {
        if budget <= 0 {
            break;
        }
        budget -= 1;
        q = h.prev(q);
    }
    let mut before = value(q);
    let (mut cur, mut cur_value) = (first, start);
    loop {
        let mut n = h.next(cur);
        while value(n) == cur_value && n != cur {
            n = h.next(n);
        }
        let after = value(n);
        if (cur_value > after && cur_value > before) || (cur_value < after && cur_value < before) {
            let f = h.flags().wrapping_add((cur as usize).wrapping_mul(2));
            write_u8(f, read_u8(f) | flag);
        }
        if n <= cur {
            break;
        }
        before = cur_value;
        cur = n;
        cur_value = after;
    }
}

/// Moves an extremum flag (`flag`, 4 in x or 8 in y) from an off-curve point to an on-curve neighbour within
/// `tolerance` on that axis (`coords`), the next one first.
unsafe fn shift_extremum_to_on_curve(h: GlyphHinter, p: i32, coords: usize, flag: u16, tolerance: i32) {
    let flags = hint_flags(h, p);
    if flags & flag == 0 {
        return;
    }
    let near = |q: i32| h.on_curve(q) && (at_i16(coords, p) as i32 - at_i16(coords, q) as i32).wrapping_abs() <= tolerance;
    let (next, prev) = (h.next(p), h.prev(p));
    if near(next) {
        set_hint_flags(h, p, flags & !flag);
        let f = h.flags().wrapping_add((next as usize).wrapping_mul(2));
        write_u8(f, read_u8(f) | flag as u8);
    }
    if near(prev) {
        set_hint_flags(h, p, hint_flags(h, p) & !flag);
        let f = h.flags().wrapping_add((prev as usize).wrapping_mul(2));
        write_u8(f, read_u8(f) | flag as u8);
    }
}

/// Flags 0x200 (x) or 0x400 (y) on an extremum (flag 4 or 8) that sits on a nearly flat stretch of off-curve points:
/// an on-curve point needs both neighbours off-curve and closer than `limit` on that axis; an off-curve point one
/// such neighbour.
unsafe fn flag_flat_extremum(h: GlyphHinter, p: i32, coords: usize, extremum: u16, mark: u16, limit: i32) {
    let flags = hint_flags(h, p);
    if flags & extremum == 0 {
        return;
    }
    let close = |q: i32| !h.on_curve(q) && (at_i16(coords, q) as i32 - at_i16(coords, p) as i32).wrapping_abs() < limit;
    let (next, prev) = (h.next(p), h.prev(p));
    let flat = if h.on_curve(p) { close(next) && close(prev) } else { close(next) || close(prev) };
    if flat {
        set_hint_flags(h, p, flags | mark);
    }
}

/// Walks a contour from `start` (where the first clear turn was found) all the way round and marks with 0x80 the point
/// half-way between two clear turns of opposite sign (|turn| above `limit`), when none of the six points involved is
/// on a flat stretch (0x100).
unsafe fn flag_inflections(h: GlyphHinter, start: i32, before: i32, after: i32, turn: i32, limit: i32) {
    let sign = |v: i32| if v < 0 { -1 } else { 1 };
    let (mut last_prev, mut last_cur, mut last_next) = (before, start, after);
    let (mut last_turn, mut last_sign) = (turn, sign(turn));
    let mut p = start;
    loop {
        let (prev, next) = (h.prev(p), h.next(p));
        let t = turn_at(h, prev, p, next);
        if t != 0 && (t < -limit || t > limit) {
            let s = sign(t);
            if s != last_sign && (last_turn < -limit || last_turn > limit) {
                let flat = |a: i32, b: i32, c: i32| (hint_flags(h, a) | hint_flags(h, b) | hint_flags(h, c)) & 0x100 != 0;
                if !flat(last_next, last_cur, last_prev) && !flat(next, prev, p) {
                    let mut steps = 0;
                    let mut k = last_prev;
                    while k != next {
                        steps += 1;
                        k = h.next(k);
                    }
                    let mut middle = last_prev;
                    for _ in 0..(steps + 1) / 2 {
                        middle = h.next(middle);
                    }
                    let f = hint_flags(h, middle);
                    if f & 0x100 == 0 {
                        set_hint_flags(h, middle, f | 0x80);
                    }
                }
            }
            last_prev = prev;
            last_cur = p;
            last_next = next;
            last_turn = t;
            last_sign = s;
        }
        if hint_flags(h, p) & 0x100 != 0 {
            last_turn = 0;
        }
        p = h.next(p);
        if p == start {
            break;
        }
    }
}

/// The first point of a contour, walking from its first point with the walk's previous point, whose turn is clear
/// (beyond `limit`, skipping flat points) or, failing that, not zero: (point, its walk predecessor, its next, turn).
unsafe fn first_turn(h: GlyphHinter, first: i32, last: i32, limit: i32) -> Option<(i32, i32, i32, i32)> {
    let mut before = h.prev(first);
    let mut turn = 0;
    for p in first..=last {
        let next = h.next(p);
        if hint_flags(h, p) & 0x100 == 0 {
            turn = turn_at(h, before, p, next);
            if turn < -limit || turn > limit {
                if turn != 0 {
                    return Some((p, before, next, turn));
                }
                break; // only with a negative limit: the original falls to the second search
            }
        }
        before = p;
    }
    if turn != 0 {
        return None; // the last turn was weak but not zero: the original keeps it and finds no start
    }
    let mut before = h.prev(first);
    for p in first..=last {
        let next = h.next(p);
        let turn = turn_at(h, before, p, next);
        if turn != 0 {
            return Some((p, before, next, turn));
        }
        before = p;
    }
    None
}

/// Analyses the outline's shape into the per-point flags: x extrema (4) and y extrema (8), moved from off-curve points
/// to near on-curve neighbours (within 1/1000 em); 0x100 where the forward and backward directions agree (a smooth
/// point), 0x800/0x1000 where the forward direction runs along x/y and 0x2000/0x4000 the backward one; 0x200/0x400 on
/// flat extrema (within 1/128 em); 0x80 half-way between turns of opposite sign (curvature beyond 1/256 em); then the
/// x/y candidates (1 and 2) from the extrema and the flat points, the two phantom points after the last point marked
/// 1, and both ends of each long on-curve segment (at least 1/10 em) marked by its direction (1 along x within ±0x2cb,
/// 2 along y, 3 slanted).
/// Original: `glyph_hint_outline_flag_extrema_points` (0x935140, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_outline_flag_extrema_points(h: GlyphHinter) {
    let upem = h.units_per_em() as i32;
    let tolerance = upem / 100 / 10 + 1;
    let last = h.contour_end(h.contour_count() as i32 - 1);
    for p in (0..=last + 4).rev() {
        set_hint_flags(h, p, 0);
    }
    for c in 0..h.contour_count() as i32 {
        let (first, end) = (h.contour_start(c), h.contour_end(c));
        if end > first {
            flag_axis_extrema(h, first, h.oox(), 4);
            flag_axis_extrema(h, first, h.ooy(), 8);
        }
    }
    for p in (0..=last).rev() {
        if !h.on_curve(p) {
            shift_extremum_to_on_curve(h, p, h.oox(), 4, tolerance);
            shift_extremum_to_on_curve(h, p, h.ooy(), 8, tolerance);
        }
    }
    let beyond = |v: i32| v > 0x3fc2 || v < -0x3fc2;
    for p in (0..=last).rev() {
        let hi = h.flags().wrapping_add((p as usize).wrapping_mul(2)) + 1;
        let mut bits = 0u8;
        let dot = h.backward_y(p).wrapping_mul(h.forward_y(p)).wrapping_add(h.backward_x(p).wrapping_mul(h.forward_x(p))) & !0x3fff;
        if dot <= 0xff08000 {
            bits |= 1;
        }
        if beyond(h.forward_x(p)) {
            bits |= 8;
        } else if beyond(h.forward_y(p)) {
            bits |= 0x10;
        }
        if beyond(h.backward_x(p)) {
            bits |= 0x20;
        } else if beyond(h.backward_y(p)) {
            bits |= 0x40;
        }
        write_u8(hi, read_u8(hi) | bits);
    }
    let flat_limit = (upem >> 7) + 1;
    for p in (0..=last).rev() {
        if hint_flags(h, p) & 0x100 == 0 {
            flag_flat_extremum(h, p, h.oox(), 4, 0x200, flat_limit);
            flag_flat_extremum(h, p, h.ooy(), 8, 0x400, flat_limit);
        }
    }
    let turn_limit = if upem / 256 == 0 { 1 } else { upem / 256 };
    for c in 0..h.contour_count() as i32 {
        let (first, end) = (h.contour_start(c), h.contour_end(c));
        if end > first + 1 {
            if let Some((p, before, after, turn)) = first_turn(h, first, end, turn_limit) {
                flag_inflections(h, p, before, after, turn, turn_limit);
            }
        }
    }
    for p in (0..=last + 4).rev() {
        let f = hint_flags(h, p);
        let f = if f & 0x104 != 0 { f | 1 } else { f };
        set_hint_flags(h, p, f);
        if f & 0x108 != 0 {
            set_hint_flags(h, p, f | 2);
        }
    }
    set_hint_flags(h, last + 1, hint_flags(h, last + 1) | 1);
    set_hint_flags(h, last + 2, hint_flags(h, last + 2) | 1);
    let long = upem / 10;
    let along = |v: i32| (-0x2cb..=0x2cb).contains(&v);
    for p in (0..=last).rev() {
        let n = h.next(p);
        if !(h.on_curve(p) && h.on_curve(n)) {
            continue;
        }
        let dy = (at_i16(h.ooy(), p).wrapping_sub(at_i16(h.ooy(), n))).wrapping_abs();
        let dx = (at_i16(h.oox(), p).wrapping_sub(at_i16(h.oox(), n))).wrapping_abs();
        let length = if dx > dy { (dy >> 1).wrapping_add(dx) } else { (dx >> 1).wrapping_add(dy) };
        if (length as i32) < long {
            continue;
        }
        let bits = if along(h.forward_x(p)) { 1 } else if along(h.forward_y(p)) { 2 } else { 3 };
        for q in [p, n] {
            let f = h.flags().wrapping_add((q as usize).wrapping_mul(2));
            write_u8(f, read_u8(f) | bits);
        }
    }
}

/// Pairs the edge points (an x/y candidate, 1, 2 or 0x80, with a straight direction, 0x7800, not alone on its contour)
/// that face each other across a stem: for each, among the others with a compatible direction class, the nearest
/// (distance estimate plus the offset across its forward direction) that `glyph_hint_stem_edge_faces_point` accepts
/// becomes its forward partner (+0x34), and likewise along its backward direction (+0x38; a smooth point copies the
/// forward partner).
/// Original: `glyph_hint_outline_pair_stem_edges` (0x935cb0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_outline_pair_stem_edges(h: GlyphHinter) {
    let last = h.contour_end(h.contour_count() as i32 - 1);
    let list = h.edge_list();
    let set16 = |arr: usize, p: i32, v: i16| write_i16(arr.wrapping_add((p as usize).wrapping_mul(2)), v);
    let mut count = 0i32;
    for p in 0..=last {
        set16(read_usize(h.0 + 0x34), p, -1);
        set16(read_usize(h.0 + 0x38), p, -1);
        let f = hint_flags(h, p);
        if f & 0x83 != 0 && f & 0x7800 != 0 && h.next(p) != p {
            count += 1;
            set16(list, count - 1, p as i16);
        }
    }
    let mut a: i16 = 0;
    while (a as i32) < count {
        let p = at_i16(list, a as i32) as i32;
        let fp = hint_flags(h, p);
        let (fx, fy, bx, by) = (h.forward_x(p), h.forward_y(p), h.backward_x(p), h.backward_y(p));
        let smooth = bx == fx && by == fy;
        let (mut best_forward, mut best_backward) = (0x7fff, 0x7fff);
        let mut b: i16 = 0;
        while (b as i32) < count {
            let q = at_i16(list, b as i32) as i32;
            b = b.wrapping_add(1);
            if q == p {
                continue;
            }
            let fq = hint_flags(h, q);
            let (q_x_class, q_y_class) = (fq & 0x2800 != 0, fq & 0x5000 != 0);
            if !((q_x_class && fp & 0x2800 != 0) || (q_y_class && fp & 0x5000 != 0)) {
                continue;
            }
            let dx = h.mid_x(q) - h.mid_x(p);
            let dy = h.mid_y(q) - h.mid_y(p);
            let (ax, ay) = (dx.wrapping_abs(), dy.wrapping_abs());
            let distance = if ax > ay { (ay >> 2) + (ax >> 1) } else { (ay >> 1) + (ax >> 2) };
            if distance < best_forward && ((q_x_class && fp & 0x800 != 0) || (q_y_class && fp & 0x1000 != 0)) {
                let score = ((dy.wrapping_mul(fx).wrapping_sub(dx.wrapping_mul(fy))) >> 14).wrapping_abs() + distance;
                if score < best_forward && glyph_hint_stem_edge_faces_point(h, p, q, fx, fy) != 0 {
                    set16(read_usize(h.0 + 0x34), p, q as i16);
                    best_forward = score;
                }
            }
            if !smooth && distance < best_backward && ((q_x_class && fp & 0x2000 != 0) || (q_y_class && fp & 0x4000 != 0)) {
                let score = ((dy.wrapping_mul(bx).wrapping_sub(dx.wrapping_mul(by))) >> 14).wrapping_abs() + distance;
                if score < best_backward && glyph_hint_stem_edge_faces_point(h, p, q, bx, by) != 0 {
                    set16(read_usize(h.0 + 0x38), p, q as i16);
                    best_backward = score;
                }
            }
        }
        if smooth {
            set16(read_usize(h.0 + 0x38), p, h.partner(true, p) as i16);
        }
        a = a.wrapping_add(1);
    }
}

/// Whether point `q` faces point `p` across a stem along the direction (vx, vy): q lies to the left of the direction
/// from p, and one of q's directions turns the other way while pointing against (vx, vy) (dot product at most
/// −0xf850000 after cutting the 14 low bits).
/// Original: `glyph_hint_stem_edge_faces_point` (0x935f90, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_stem_edge_faces_point(h: GlyphHinter, p: i32, q: i32, vx: i32, vy: i32) -> i32 {
    let dx = at_i16(h.oox(), q) as i32 - at_i16(h.oox(), p) as i32;
    let dy = at_i16(h.ooy(), q) as i32 - at_i16(h.ooy(), p) as i32;
    if dx.wrapping_mul(vy).wrapping_sub(dy.wrapping_mul(vx)) <= 0 {
        return 0;
    }
    let opposed = |ux: i32, uy: i32| {
        uy.wrapping_mul(dx).wrapping_sub(ux.wrapping_mul(dy)) < 0 && (ux.wrapping_mul(vx).wrapping_add(uy.wrapping_mul(vy)) & !0x3fff) <= -0xf850000
    };
    if opposed(h.forward_x(q), h.forward_y(q)) || opposed(h.backward_x(q), h.backward_y(q)) {
        1
    } else {
        0
    }
}

/// Comparers: the facing test value against value; the rest only the original (they change the hinter; proved on the
/// emulator).
pub(crate) unsafe extern "C" fn compare_glyph_hint_collect_stem_widths_checked(h: GlyphHinter, g: HintGlyph, figure: i16, a4: i32, xw: usize, xc: usize, yw: usize, yc: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32, usize, usize, usize, usize) -> i32>(%%I0%%)(h, g, figure, a4, xw, xc, yw, yc)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_run_pass(h: GlyphHinter, g: HintGlyph, figure: i16, a4: i32, mode: i16, xw: usize, xc: usize, yw: usize, yc: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32, i16, usize, usize, usize, usize) -> i32>(%%I1%%)(h, g, figure, a4, mode, xw, xc, yw, yc)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_outline_compute_point_tangents(h: GlyphHinter) {
    original_fn::<unsafe extern "C" fn(GlyphHinter)>(%%I2%%)(h)
}

pub(crate) unsafe extern "C" fn compare_fixed_normalise_vector_2dot14(x: &mut i32, y: &mut i32) {
    let (mut cx, mut cy) = (*x, *y);
    original_fn::<unsafe extern "C" fn(&mut i32, &mut i32)>(%%I3%%)(x, y);
    fixed_normalise_vector_2dot14(&mut cx, &mut cy);
    SCOREBOARD.record((cx, cy) == (*x, *y), || format!("fixed_normalise_vector_2dot14 original=({}, {}) rust=({}, {})", x, y, cx, cy));
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_outline_flag_extrema_points(h: GlyphHinter) {
    original_fn::<unsafe extern "C" fn(GlyphHinter)>(%%I4%%)(h)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_outline_pair_stem_edges(h: GlyphHinter) {
    original_fn::<unsafe extern "C" fn(GlyphHinter)>(%%I5%%)(h)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_stem_edge_faces_point(h: GlyphHinter, p: i32, q: i32, vx: i32, vy: i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(GlyphHinter, i32, i32, i32, i32) -> i32>(%%I6%%)(h, p, q, vx, vy);
    let c = glyph_hint_stem_edge_faces_point(h, p, q, vx, vy);
    SCOREBOARD.record(r == c, || format!("glyph_hint_stem_edge_faces_point({}, {}, {}, {}) original={} rust={}", p, q, vx, vy, r, c));
    r
}
