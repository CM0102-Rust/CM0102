// ---------- rx01_6: the T2K automatic hinter: the alignment zones, the stem widths and the interpolation of untouched points (library_without_asserts, T2K autogrid) ----------

/// Which alignment zone point `p` sits on: a zone edge code (2 × zone for the zone's first height, + 1 for its second
/// height) or −1.
///
/// A point whose in and out tangents both point up (x parts ≤ 0x3f07 in 2.14) cannot take a top zone (a zone height
/// above a quarter em), one whose tangents both point down cannot take a bottom zone, unless it is a local extreme
/// (flag 8) above (or below) both its neighbours; with both, none. The second heights are tried only for a point on the
/// curve whose on-curve neighbours, farther than 1/16 em, do not run flat (tangent x part beyond ±0x3fc2). Zones 2
/// and 6 apply only to a figure glyph, the others only to the rest. The nearest height within 1/66 em wins (the first
/// exact one ends the search).
/// Original: `glyph_hint_find_nearest_alignment_zone` (0x933b20, library_without_asserts, T2K autogrid, cdecl; the
/// result in ax).
pub(crate) unsafe extern "C" fn glyph_hint_find_nearest_alignment_zone(h: GlyphHinter, p: i32) -> i16 {
    let tin = h.forward_x(p);
    let tout = h.backward_x(p);
    let mut no_top = tin <= 0x3f07 && tout <= 0x3f07;
    let mut no_bottom = tin >= -0x3f07 && tout >= -0x3f07;
    let (next, prev) = (h.next(p), h.prev(p));
    let y = at_i16(h.ooy(), p) as i32;
    let extreme = read_u8(h.flags().wrapping_add((p as usize).wrapping_mul(2))) & 8 != 0;
    let (y_prev, y_next) = (at_i16(h.ooy(), prev) as i32, at_i16(h.ooy(), next) as i32);
    if no_top && extreme && y > y_prev && y > y_next {
        no_top = false;
    }
    if no_bottom && extreme && y < y_prev && y < y_next {
        no_bottom = false;
    }
    if no_top && no_bottom {
        return -1;
    }
    let upem = h.units_per_em() as i32;
    let mut best_distance = upem / 66;
    let mut best: i32 = -1;
    let second_heights = !h.on_curve(p) || allows_second_heights(h, p, next, prev, tin);
    let quarter_em = (upem / 4) as i16 as i32;
    let figure = h.is_figure() as i32;
    for k in 0..10i32 {
        let zone = h.0 + 0x34e + 6 * k as usize;
        let first = read_i16(zone) as i32;
        let excluded = if first > quarter_em { no_top } else { no_bottom };
        if excluded {
            continue;
        }
        let figure_zone = k == 2 || k == 6;
        if (figure != 0) != figure_zone {
            continue;
        }
        let d = first.wrapping_sub(y).wrapping_abs();
        if d < best_distance {
            best = 2 * k;
            best_distance = d;
            if d == 0 {
                break;
            }
        }
        if second_heights {
            let d = y.wrapping_sub(read_i16(zone + 2) as i32).wrapping_abs();
            if d < best_distance {
                best = 2 * k + 1;
                best_distance = d;
                if d == 0 {
                    break;
                }
            }
        }
    }
    best as i16
}

/// Whether an on-curve point may take a zone's second height: not when an on-curve neighbour lies farther than 1/16
/// em in x while the tangent on that side runs flat (its x part beyond ±0x3fc2: the in tangent toward the next point,
/// the out tangent toward the previous one).
unsafe fn allows_second_heights(h: GlyphHinter, p: i32, next: i32, prev: i32, tin: i32) -> bool {
    let sixteenth = h.units_per_em() as i32 / 16;
    let far = |q: i32| (at_i16(h.oox(), p).wrapping_sub(at_i16(h.oox(), q))).wrapping_abs() > sixteenth as i16;
    let flat = |t: i32| t > 0x3fc2 || t < -0x3fc2;
    if h.on_curve(next) && far(next) && flat(tin) {
        return false;
    }
    !(h.on_curve(prev) && far(prev) && flat(h.backward_x(p)))
}

/// Sets the hinted distance between points `from` and `to` from their original coordinates `oo`: when a stem width
/// slot matches the distance (`glyph_hint_find_nearest_stem_width_index` with `x_axis` and `y_allowed`) it is set by
/// `glyph_hint_set_distance_by_axis` with that slot; otherwise by the axis's distance setter with no slot and at least
/// one pixel, passing its other arguments through.
/// Original: `glyph_hint_set_distance_by_stem_width` (0x933da0, library_without_asserts, T2K autogrid, cdecl; the
/// original also picks a function by the distance's sign, but both choices are `glyph_hint_set_distance_by_axis`).
pub(crate) unsafe extern "C" fn glyph_hint_set_distance_by_stem_width(h: GlyphHinter, g: HintGlyph, oo: usize, x_axis: i16, y_allowed: i16, a6: i32, a7: i32, a8: i32, a9: i32, from: i32, to: i32) {
    let d = (at_i16(oo, to) as i32).wrapping_sub(at_i16(oo, from) as i32).wrapping_abs();
    let slot = glyph_hint_find_nearest_stem_width_index(h, x_axis, y_allowed, 0, d);
    if slot as i16 >= 0 {
        return glyph_hint_set_distance_by_axis(h, g, slot, from, to, x_axis);
    }
    let set: GlyphHintDistanceFn = if x_axis != 0 { glyph_hint_set_x_distance_rounded } else { glyph_hint_set_y_distance_rounded };
    set(h, g, -1, 1, a6 as i16, a7, a8, a9, from, to);
}

/// Places point `p` on one axis between the reference points `a` and `b`: each reference's hinted coordinate plus the
/// scaled original distance to `p` (times the scale search factor, /4096 rounded, then /64 truncated) gives two
/// candidates; outside a scale search the one from `a` moves one pixel toward the one from `b` (not past it); in mode 2
/// the point goes to their average, otherwise a third of the way from `b` to `a`; then rounded to a whole pixel.
/// Original: `glyph_hint_place_point_between_two_refs` (0x933e50, library_without_asserts, T2K autogrid, cdecl; the
/// 4th argument is not read).
pub(crate) unsafe extern "C" fn glyph_hint_place_point_between_two_refs(h: GlyphHinter, g: HintGlyph, x_axis: i16, _a4: i32, a: i16, b: i16, p: i16) {
    let x_axis = x_axis != 0;
    let (ppem, oo, coords) = if x_axis { (h.x_ppem(), h.oox(), g.x()) } else { (h.y_ppem(), h.ooy(), g.y()) };
    let ppem64 = ppem << 6;
    let upem = h.units_per_em() as i32;
    let origin = at_i16(oo, p as i32) as i32;
    let candidate = |r: i16| {
        let d = origin.wrapping_sub(at_i16(oo, r as i32) as i32).wrapping_mul(ppem64).wrapping_add(upem >> 1) / upem;
        let scaled = d.wrapping_mul(h.scale_factor()).wrapping_add(0x20) >> 6;
        at_i32(coords, r as i32).wrapping_add(scaled / 64)
    };
    let from_b = candidate(b);
    let mut from_a = candidate(a);
    if h.scale_steps() == 0 {
        from_a = approach(from_a, from_b, 0x40);
    }
    let placed = if h.mode() == 2 {
        from_a.wrapping_add(from_b).wrapping_add(1) / 2
    } else {
        from_a.wrapping_add(from_b.wrapping_mul(2)).wrapping_add(1) / 3
    };
    set_i32(coords, p as i32, round_to_pixel(placed));
}

/// Adjusts an x stem between edges `left` (with its partner `left_end`) and `right` (with `right_start`): `right` is
/// rounded to a pixel; when the stem spans at least three pixels and both edge widths (scaled originals) exceed −8,
/// the hinted edge widths are compared with the original ones: much wider than the original (by 0x39 or more) moves
/// `left` one pixel right; narrower moves `right` one pixel right; in between, `left` moves one pixel right with
/// `right` when the right edge is narrow and the left one wide enough.
/// Original: `glyph_hint_adjust_x_stem_edges` (0x933fa0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_adjust_x_stem_edges(h: GlyphHinter, g: HintGlyph, left: i32, left_end: i32, right_start: i32, right: i32) {
    let (xs, x) = (h.x_scaled(), g.x());
    let mut r = round_to_pixel(at_i32(x, right));
    let l = at_i32(x, left);
    let left_width = at_i32(xs, left_end).wrapping_sub(at_i32(xs, left));
    let right_width = at_i32(xs, right).wrapping_sub(at_i32(xs, right_start));
    if r.wrapping_sub(l) >= 0xc0 && left_width > -8 && right_width > -8 {
        let widths = right_width.wrapping_add(left_width);
        let hinted_left = at_i32(x, left_end).wrapping_sub(l);
        let hinted_right = r.wrapping_sub(at_i32(x, right_start));
        let hinted = hinted_right.wrapping_add(hinted_left);
        let narrow_right = hinted_right < 0x20;
        if hinted >= widths.wrapping_add(if narrow_right { 0x19 } else { 0 }).wrapping_sub(0x20) {
            if hinted > left_width.wrapping_add(right_width).wrapping_add(0x39) {
                set_i32(x, left, at_i32(x, left).wrapping_add(0x40));
                set_i32(x, right, r);
                return;
            }
            let limit = right_width.wrapping_sub(if narrow_right { 0 } else { 0x19 });
            if hinted_right > limit || hinted_left <= left_width.wrapping_add(0x19) {
                set_i32(x, right, r);
                return;
            }
            set_i32(x, left, at_i32(x, left).wrapping_add(0x40));
        }
        r = r.wrapping_add(0x40);
    }
    set_i32(x, right, r);
}

/// Moves the untouched points of each contour of `g` on one axis between the touched ones (flag `touched`): between
/// two consecutive touched points, each point between them keeps its proportional place by the scaled original
/// coordinates `scaled` (a plain shift when the two had the same original; the ratio by `fixed_div`/`fixed_mul` when
/// the hinted span exceeds ±0x2000).
unsafe fn interpolate_untouched(h: GlyphHinter, g: HintGlyph, touched: u8, scaled: usize, coords: usize) {
    let is_touched = |p: i32| read_u8(h.flags().wrapping_add((p as usize).wrapping_mul(2))) & touched != 0;
    for c in 0..read_i16(g.0) as i32 {
        let first = at_i16(read_usize(g.0 + 4), c) as i32;
        let last = at_i16(read_usize(g.0 + 8), c) as i32;
        if last <= first {
            continue;
        }
        let Some(start) = (first..=last).find(|&p| is_touched(p)) else { continue };
        let mut p0 = start;
        loop {
            let mut q = h.next(p0);
            while !is_touched(q) {
                q = h.next(q);
            }
            let (s0, c0) = (at_i32(scaled, p0), at_i32(coords, p0));
            let span = s0.wrapping_sub(at_i32(scaled, q));
            let between = |f: &mut dyn FnMut(i32)| {
                let mut k = h.next(p0);
                while k != q {
                    f(k);
                    k = h.next(k);
                }
            };
            if span == 0 {
                let shift = c0.wrapping_sub(s0);
                between(&mut |k| set_i32(coords, k, at_i32(coords, k).wrapping_add(shift)));
            } else {
                let moved = c0.wrapping_sub(at_i32(coords, q));
                if (-0x2000..=0x2000).contains(&moved) {
                    between(&mut |k| set_i32(coords, k, (at_i32(scaled, k).wrapping_sub(s0).wrapping_mul(moved) / span).wrapping_add(c0)));
                } else {
                    let ratio = lib_fixed_div(moved, span);
                    between(&mut |k| set_i32(coords, k, lib_fixed_mul(at_i32(scaled, k).wrapping_sub(s0), ratio).wrapping_add(c0)));
                }
            }
            p0 = q;
            if q == start {
                break;
            }
        }
    }
}

/// The interpolation of the untouched points in x (touched flag 0x10, the scaled original x).
/// Original: `glyph_hint_interpolate_untouched_x` (0x9345e0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_interpolate_untouched_x(h: GlyphHinter, g: HintGlyph) {
    interpolate_untouched(h, g, 0x10, h.x_scaled(), g.x());
}

/// The interpolation of the untouched points in y (touched flag 0x20, the scaled original y).
/// Original: `glyph_hint_interpolate_untouched_y` (0x9347a0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_interpolate_untouched_y(h: GlyphHinter, g: HintGlyph) {
    interpolate_untouched(h, g, 0x20, h.y_scaled(), g.y());
}

/// Sets the distance from `from` to `to` with stem width `slot` through the axis's distance setter (x when `x_axis`),
/// at least one pixel.
/// Original: `glyph_hint_set_distance_by_axis` (0x934960, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_set_distance_by_axis(h: GlyphHinter, g: HintGlyph, slot: i32, from: i32, to: i32, x_axis: i16) {
    let set: GlyphHintDistanceFn = if x_axis != 0 { glyph_hint_set_x_distance_rounded } else { glyph_hint_set_y_distance_rounded };
    set(h, g, slot as i16, 0, 1, 1, 0x47, 0x72, from, to);
}

/// The stem width slot nearest `distance` among the 12 x widths (slots 0x48..) or, when `x_axis` is clear and
/// `y_allowed` set, the 12 y widths (slots 0x54..): a width w counts when `distance` lies in [w − w/2, w + w/2] (16-bit
/// halves); the smallest |w − distance| (16-bit) wins, the first on ties. −1 when none.
/// Original: `glyph_hint_find_nearest_stem_width_index` (0x9349a0, library_without_asserts, T2K autogrid, cdecl; the
/// 4th argument is not read).
pub(crate) unsafe extern "C" fn glyph_hint_find_nearest_stem_width_index(h: GlyphHinter, x_axis: i16, y_allowed: i16, _a4: i32, distance: i32) -> i32 {
    let (widths, first_slot) = if x_axis != 0 {
        (h.0 + 0x38a, 0x48)
    } else if y_allowed != 0 {
        (h.0 + 0x3a2, 0x54)
    } else {
        return -1;
    };
    let mut best = -1;
    let mut best_diff = 0x7fff;
    for k in 0..12 {
        let w = read_i16(widths + 2 * k as usize);
        if w == 0 {
            continue;
        }
        let half = w >> 1;
        let diff = (w.wrapping_sub(distance as i16) as i32).wrapping_abs();
        if diff < best_diff && distance >= w.wrapping_sub(half) as i32 && distance <= w.wrapping_add(half) as i32 {
            best_diff = diff;
            best = k;
        }
    }
    if best < 0 {
        -1
    } else {
        best + first_slot
    }
}

/// The largest of the 12 x stem widths (0 when none is positive).
/// Original: `glyph_hint_max_x_stem_width` (0x934ab0, library_without_asserts, T2K autogrid, cdecl; the result in ax).
pub(crate) unsafe extern "C" fn glyph_hint_max_x_stem_width(h: GlyphHinter) -> i16 {
    (0..12).map(|k| read_i16(h.0 + 0x38a + 2 * k)).fold(0, i16::max)
}

/// The largest of the 12 y stem widths (0 when none is positive).
/// Original: `glyph_hint_max_y_stem_width` (0x934ae0, library_without_asserts, T2K autogrid, cdecl; the result in ax).
pub(crate) unsafe extern "C" fn glyph_hint_max_y_stem_width(h: GlyphHinter) -> i16 {
    (0..12).map(|k| read_i16(h.0 + 0x3a2 + 2 * k)).fold(0, i16::max)
}

/// Comparers: the pure ones (the zone, the stem width searches) value against value, the low 16 bits where the
/// original returns in ax; the rest only the original (they change the glyph; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_glyph_hint_find_nearest_alignment_zone(h: GlyphHinter, p: i32) -> i16 {
    let r = original_fn::<unsafe extern "C" fn(GlyphHinter, i32) -> i16>(%%I0%%)(h, p);
    let c = glyph_hint_find_nearest_alignment_zone(h, p);
    SCOREBOARD.record(r == c, || format!("glyph_hint_find_nearest_alignment_zone({}) original={} rust={}", p, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_set_distance_by_stem_width(h: GlyphHinter, g: HintGlyph, oo: usize, x_axis: i16, y_allowed: i16, a6: i32, a7: i32, a8: i32, a9: i32, from: i32, to: i32) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, usize, i16, i16, i32, i32, i32, i32, i32, i32)>(%%I1%%)(h, g, oo, x_axis, y_allowed, a6, a7, a8, a9, from, to)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_place_point_between_two_refs(h: GlyphHinter, g: HintGlyph, x_axis: i16, a4: i32, a: i16, b: i16, p: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32, i16, i16, i16)>(%%I2%%)(h, g, x_axis, a4, a, b, p)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_adjust_x_stem_edges(h: GlyphHinter, g: HintGlyph, left: i32, left_end: i32, right_start: i32, right: i32) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i32, i32, i32, i32)>(%%I3%%)(h, g, left, left_end, right_start, right)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_interpolate_untouched_x(h: GlyphHinter, g: HintGlyph) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph)>(%%I4%%)(h, g)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_interpolate_untouched_y(h: GlyphHinter, g: HintGlyph) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph)>(%%I5%%)(h, g)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_set_distance_by_axis(h: GlyphHinter, g: HintGlyph, slot: i32, from: i32, to: i32, x_axis: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i32, i32, i32, i16)>(%%I6%%)(h, g, slot, from, to, x_axis)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_find_nearest_stem_width_index(h: GlyphHinter, x_axis: i16, y_allowed: i16, a4: i32, distance: i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(GlyphHinter, i16, i16, i32, i32) -> i32>(%%I7%%)(h, x_axis, y_allowed, a4, distance);
    let c = glyph_hint_find_nearest_stem_width_index(h, x_axis, y_allowed, a4, distance);
    SCOREBOARD.record(r == c, || format!("glyph_hint_find_nearest_stem_width_index({}, {}, {}) original={} rust={}", x_axis, y_allowed, distance, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_max_x_stem_width(h: GlyphHinter) -> i16 {
    let r = original_fn::<unsafe extern "C" fn(GlyphHinter) -> i16>(%%I8%%)(h);
    let c = glyph_hint_max_x_stem_width(h);
    SCOREBOARD.record(r == c, || format!("glyph_hint_max_x_stem_width original={} rust={}", r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_max_y_stem_width(h: GlyphHinter) -> i16 {
    let r = original_fn::<unsafe extern "C" fn(GlyphHinter) -> i16>(%%I9%%)(h);
    let c = glyph_hint_max_y_stem_width(h);
    SCOREBOARD.record(r == c, || format!("glyph_hint_max_y_stem_width original={} rust={}", r, c));
    r
}
