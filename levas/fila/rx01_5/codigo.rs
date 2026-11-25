// ---------- rx01_5: the T2K automatic hinter: its context, the glyph it hints and the point operations (library_without_asserts, T2K autogrid) ----------

/// The T2K automatic hinter's context (`ag_DataType`, packed, 0x450 bytes, made by `glyph_hint_create`): the glyph
/// outline being hinted (copied pointers), the per-point work arrays, the font's alignment zones and stem widths in
/// font units and scaled to 26.6, and the size. Its two magic words mark it valid.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct GlyphHinter(pub(crate) usize);

/// The magic words of a hinter context (+0 and +0x44c).
pub(crate) const HINTER_MAGIC: u32 = 0xa5a0f5a5;
pub(crate) const HINTER_MAGIC_END: u32 = 0x0fa55af0;
/// The "no slot" mark of the hint value slots.
pub(crate) const NO_HINT_SLOT: i16 = 9999;

#[allow(dead_code)]
impl GlyphHinter {
    pub(crate) const SIZE: usize = 0x450;
    /// Both magic words in place (and not null).
    pub(crate) unsafe fn is_valid(self) -> bool {
        self.0 != 0 && read_u32(self.0) == HINTER_MAGIC && read_u32(self.0 + 0x44c) == HINTER_MAGIC_END
    }
    /// +4 the contour count, +6 the contour start points, +0xa the contour end points (i16 arrays).
    pub(crate) unsafe fn contour_count(self) -> i16 { read_i16(self.0 + 4) }
    pub(crate) unsafe fn contour_start(self, c: i32) -> i32 { at_i16(read_usize(self.0 + 6), c) as i32 }
    pub(crate) unsafe fn contour_end(self, c: i32) -> i32 { at_i16(read_usize(self.0 + 0xa), c) as i32 }
    /// +0xe the on-curve flags (u8 array).
    pub(crate) unsafe fn on_curve(self, p: i32) -> bool { read_u8(read_usize(self.0 + 0xe).wrapping_add(p as usize)) != 0 }
    /// +0x12 the original x, +0x16 the original y (i16 arrays, font units).
    pub(crate) unsafe fn oox(self) -> usize { read_usize(self.0 + 0x12) }
    pub(crate) unsafe fn ooy(self) -> usize { read_usize(self.0 + 0x16) }
    /// +0x1a set for a figure glyph (the figure zones 2 and 6 apply only then) [inference].
    pub(crate) unsafe fn is_figure(self) -> i16 { read_i16(self.0 + 0x1a) }
    /// +0x1c the next point and +0x20 the previous point on the contour (i16 arrays).
    pub(crate) unsafe fn next(self, p: i32) -> i32 { at_i16(read_usize(self.0 + 0x1c), p) as i32 }
    pub(crate) unsafe fn prev(self, p: i32) -> i32 { at_i16(read_usize(self.0 + 0x20), p) as i32 }
    /// +0x24 the edge point list, +0x2c/+0x30 the per-point midpoints (i16), +0x34/+0x38 the stem partner found along
    /// the forward and the backward direction (i16, −1 for none).
    pub(crate) unsafe fn edge_list(self) -> usize { read_usize(self.0 + 0x24) }
    pub(crate) unsafe fn mid_x(self, p: i32) -> i32 { at_i16(read_usize(self.0 + 0x2c), p) as i32 }
    pub(crate) unsafe fn mid_y(self, p: i32) -> i32 { at_i16(read_usize(self.0 + 0x30), p) as i32 }
    pub(crate) unsafe fn partner(self, forward: bool, p: i32) -> i32 { at_i16(read_usize(self.0 + if forward { 0x34 } else { 0x38 }), p) as i32 }
    /// +0x28 the per-point hint flags (u16 array).
    pub(crate) unsafe fn flags(self) -> usize { read_usize(self.0 + 0x28) }
    /// +0x40/+0x44 the unit direction from each point toward the next, +0x48/+0x4c from the previous point toward it
    /// (i32 arrays, 2.14; the same vector for an off-curve point).
    pub(crate) unsafe fn forward_x(self, p: i32) -> i32 { at_i32(read_usize(self.0 + 0x40), p) }
    pub(crate) unsafe fn forward_y(self, p: i32) -> i32 { at_i32(read_usize(self.0 + 0x44), p) }
    pub(crate) unsafe fn backward_x(self, p: i32) -> i32 { at_i32(read_usize(self.0 + 0x48), p) }
    pub(crate) unsafe fn backward_y(self, p: i32) -> i32 { at_i32(read_usize(self.0 + 0x4c), p) }
    /// +0x5c the hint values in 26.6 (i32 × 0x60): the alignment zones by pairs, the x stem widths from slot 0x48, the
    /// y stem widths from slot 0x54.
    pub(crate) unsafe fn hint_value(self, slot: i16) -> i32 { read_i32(self.0.wrapping_add(0x5c).wrapping_add((slot as isize as usize).wrapping_mul(4))) }
    /// +0x29c the units per em.
    pub(crate) unsafe fn units_per_em(self) -> i16 { read_i16(self.0 + 0x29c) }
    /// +0x29e the x and +0x2a2 the y pixels per em.
    pub(crate) unsafe fn x_ppem(self) -> i32 { read_i32(self.0 + 0x29e) }
    pub(crate) unsafe fn y_ppem(self) -> i32 { read_i32(self.0 + 0x2a2) }
    /// +0x2a6 the render mode (1 and 2 seen) [inference].
    pub(crate) unsafe fn mode(self) -> i32 { read_i32(self.0 + 0x2a6) }
    /// +0x3c8 the original x and +0x3cc the original y scaled to 26.6 (i32 arrays).
    pub(crate) unsafe fn x_scaled(self) -> usize { read_usize(self.0 + 0x3c8) }
    pub(crate) unsafe fn y_scaled(self) -> usize { read_usize(self.0 + 0x3cc) }
    /// +0x41e/+0x422/+0x426 the scale search's upper bound, lower bound and factor (0x1000 = 1.0), +0x42a its step
    /// count, +0x42e the last miss, +0x432 set while it searches.
    pub(crate) unsafe fn scale_factor(self) -> i32 { read_i32(self.0 + 0x426) }
    pub(crate) unsafe fn scale_steps(self) -> i32 { read_i32(self.0 + 0x42a) }
    /// +0x448 set when the advance is not rounded (the grey mode of `glyph_hint_outline_and_get_advance`) [inference].
    pub(crate) unsafe fn unrounded(self) -> bool { read_i32(self.0 + 0x448) != 0 }
}

/// The glyph a hinter works on (`ag_ElementType`): +0 the contour count, +2 the point count (i16), +4 the contour
/// starts, +8 the contour ends (i16 arrays), +0xc the original x and +0x10 the original y (i16), +0x14 the on-curve
/// flags (u8), +0x18 the hinted x and +0x1c the hinted y (i32, 26.6), +0x20 the advance and +0x24 the hinted advance.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct HintGlyph(pub(crate) usize);

#[allow(dead_code)]
impl HintGlyph {
    pub(crate) unsafe fn point_count(self) -> i16 { read_i16(self.0 + 2) }
    pub(crate) unsafe fn oox(self) -> usize { read_usize(self.0 + 0xc) }
    pub(crate) unsafe fn ooy(self) -> usize { read_usize(self.0 + 0x10) }
    pub(crate) unsafe fn x(self) -> usize { read_usize(self.0 + 0x18) }
    pub(crate) unsafe fn y(self) -> usize { read_usize(self.0 + 0x1c) }
    /// The hinted coordinates of one axis.
    pub(crate) unsafe fn coords(self, x_axis: bool) -> usize { if x_axis { self.x() } else { self.y() } }
}

/// Element `k` of an i16 array.
pub(crate) unsafe fn at_i16(arr: usize, k: i32) -> i16 {
    read_i16(arr.wrapping_add((k as usize).wrapping_mul(2)))
}

/// Element `k` of an i32 array.
pub(crate) unsafe fn at_i32(arr: usize, k: i32) -> i32 {
    read_i32(arr.wrapping_add((k as usize).wrapping_mul(4)))
}

/// Sets element `k` of an i32 array.
pub(crate) unsafe fn set_i32(arr: usize, k: i32, v: i32) {
    write_i32(arr.wrapping_add((k as usize).wrapping_mul(4)), v)
}

/// `v` moved toward `target` by at most `step` (never past it).
pub(crate) fn approach(v: i32, target: i32, step: i32) -> i32 {
    if v > target {
        target.max(v.wrapping_sub(step))
    } else {
        target.min(v.wrapping_add(step))
    }
}

/// Rounds a 26.6 value to the nearest whole pixel.
pub(crate) fn round_to_pixel(v: i32) -> i32 {
    v.wrapping_add(0x20) & !0x3f
}

/// The scale search back at its start: factor 0x1000, bounds 0x800 and 0x2000, no step.
/// Original: `glyph_hint_reset_scale_search` (0x9340a0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_reset_scale_search(h: GlyphHinter) {
    write_i32(h.0 + 0x426, 0x1000);
    write_i32(h.0 + 0x422, 0x800);
    write_i32(h.0 + 0x41e, 0x2000);
    write_i32(h.0 + 0x42a, 0);
}

/// One step of the scale search for point `p` on one axis: its scaled original coordinate, moved toward hint value
/// `slot` by at most 0x60 (none with `NO_HINT_SLOT`), is rounded to a pixel and compared with the hinted coordinate.
/// A miss (kept at +0x42e) under 14 steps halves the factor's interval toward it (from the second step), counts the
/// step and marks the search running; a hit, or the 14th step, stores the rounded value in the point and resets the
/// search.
/// Original: `glyph_hint_refine_scale_step` (0x9340d0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_refine_scale_step(h: GlyphHinter, g: HintGlyph, x_axis: i16, slot: i16, p: i16) {
    let x_axis = x_axis != 0;
    let coords = g.coords(x_axis);
    let scaled = if x_axis { h.x_scaled() } else { h.y_scaled() };
    let p = p as i32;
    let mut target = at_i32(scaled, p);
    if slot != NO_HINT_SLOT {
        target = approach(target, h.hint_value(slot), 0x60);
    }
    let rounded = round_to_pixel(target);
    let miss = at_i32(coords, p).wrapping_sub(rounded);
    write_i32(h.0 + 0x42e, miss);
    let steps = h.scale_steps();
    if miss != 0 && steps < 0xe {
        if steps != 0 {
            let factor = h.scale_factor();
            let other = if miss > 0 {
                write_i32(h.0 + 0x41e, factor);
                read_i32(h.0 + 0x422)
            } else {
                write_i32(h.0 + 0x422, factor);
                read_i32(h.0 + 0x41e)
            };
            write_i32(h.0 + 0x426, other.wrapping_add(factor) / 2);
        }
        write_i32(h.0 + 0x432, 1);
        write_i32(h.0 + 0x42a, steps + 1);
        return;
    }
    set_i32(coords, p, rounded);
    glyph_hint_reset_scale_search(h);
    write_i32(h.0 + 0x432, 0);
}

/// Raises point `to`'s hinted coordinate on one axis to point `from`'s when that is larger.
/// Original: `glyph_hint_raise_to_max_of_pair` (0x9341e0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_raise_to_max_of_pair(_h: GlyphHinter, g: HintGlyph, x_axis: i16, from: i16, to: i16) {
    let coords = g.coords(x_axis != 0);
    let v = at_i32(coords, from as i32);
    if at_i32(coords, to as i32) < v {
        set_i32(coords, to as i32, v);
    }
}

/// Lowers point `to`'s hinted coordinate on one axis to point `from`'s when that is smaller.
/// Original: `glyph_hint_lower_to_min_of_pair` (0x934220, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_lower_to_min_of_pair(_h: GlyphHinter, g: HintGlyph, x_axis: i16, from: i16, to: i16) {
    let coords = g.coords(x_axis != 0);
    let v = at_i32(coords, from as i32);
    if at_i32(coords, to as i32) > v {
        set_i32(coords, to as i32, v);
    }
}

/// When point `p` passed the reference point `limit` on one axis, moves it back by the excess (measured on the low
/// 16 bits) and moves the companion point by the same amount.
/// Original: `glyph_hint_clamp_point_and_shift_companion` (0x934260, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_clamp_point_and_shift_companion(_h: GlyphHinter, g: HintGlyph, x_axis: i16, limit: i16, p: i16, companion: i16) {
    let coords = g.coords(x_axis != 0);
    let (v, l) = (at_i32(coords, p as i32), at_i32(coords, limit as i32));
    if v > l {
        let excess = (v as i16).wrapping_sub(l as i16) as i32;
        set_i32(coords, p as i32, v.wrapping_sub(excess));
        set_i32(coords, companion as i32, at_i32(coords, companion as i32).wrapping_sub(excess));
    }
}

/// The scaled distance (26.6) from `from` to `to` in the original coordinates `oo` at `ppem` pixels per em, rounded:
/// its magnitude and its sign (1 or −1).
unsafe fn scaled_distance(h: GlyphHinter, oo: usize, ppem: i32, from: i32, to: i32) -> (i32, i32) {
    let upem = h.units_per_em() as i32;
    let d = (at_i16(oo, to) as i32).wrapping_sub(at_i16(oo, from) as i32).wrapping_mul(ppem).wrapping_mul(64).wrapping_add(upem >> 1) / upem;
    if d < 0 {
        (d.wrapping_neg(), -1)
    } else {
        (d, 1)
    }
}

/// Places point `to`'s hinted x at point `from`'s plus their scaled original distance with its sign. Normally the
/// distance moves toward the stem width of `slot` (when ≥ 0) by at most 42 and is rounded to a whole pixel, at least
/// one pixel with `min_one_pixel`; in the unrounded mode a distance under half a pixel is doubled (with
/// `min_one_pixel`, at most half a pixel) and nothing is rounded.
/// Original: `glyph_hint_set_x_distance_rounded` (0x9342c0, library_without_asserts, T2K autogrid, cdecl; the 4th and
/// 6th to 8th arguments are not read).
pub(crate) unsafe extern "C" fn glyph_hint_set_x_distance_rounded(h: GlyphHinter, g: HintGlyph, slot: i16, _a4: i32, min_one_pixel: i16, _a6: i32, _a7: i32, _a8: i32, from: i32, to: i32) {
    let (mut d, sign) = scaled_distance(h, h.oox(), h.x_ppem(), from, to);
    if h.unrounded() {
        if d < 0x20 && min_one_pixel != 0 {
            d = (d * 2).min(0x20);
        }
    } else {
        if slot >= 0 {
            d = approach(d, h.hint_value(slot), 0x2a);
        }
        d = round_to_pixel(d);
        if d == 0 && min_one_pixel != 0 {
            d = 0x40;
        }
    }
    set_i32(g.x(), to, sign.wrapping_mul(d).wrapping_add(at_i32(g.x(), from)));
}

/// Places point `to`'s hinted y at point `from`'s plus their scaled original distance with its sign: moved toward the
/// stem width of `slot` (when ≥ 0) by at most 42, then normally rounded to a whole pixel (at least one with
/// `min_one_pixel`); in the unrounded mode only a distance of 0x24 or more outside mode 2 is rounded, and anything else
/// keeps the distance as measured (without the stem width).
/// Original: `glyph_hint_set_y_distance_rounded` (0x934380, library_without_asserts, T2K autogrid, cdecl; the 4th and
/// 6th to 8th arguments are not read).
pub(crate) unsafe extern "C" fn glyph_hint_set_y_distance_rounded(h: GlyphHinter, g: HintGlyph, slot: i16, _a4: i32, min_one_pixel: i16, _a6: i32, _a7: i32, _a8: i32, from: i32, to: i32) {
    let (measured, sign) = scaled_distance(h, h.ooy(), h.y_ppem(), from, to);
    let snapped = if slot >= 0 { approach(measured, h.hint_value(slot), 0x2a) } else { measured };
    let d = if h.unrounded() {
        if snapped >= 0x24 && h.mode() != 2 { round_to_pixel(snapped) } else { measured }
    } else {
        let r = round_to_pixel(snapped);
        if r == 0 && min_one_pixel != 0 { 0x40 } else { r }
    };
    set_i32(g.y(), to, sign.wrapping_mul(d).wrapping_add(at_i32(g.y(), from)));
}

/// Rounds point `p`'s hinted x to the nearest whole pixel when `apply` is set.
/// Original: `glyph_hint_round_x_to_pixel_if_flag` (0x934440, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_round_x_to_pixel_if_flag(_h: GlyphHinter, g: HintGlyph, apply: i16, p: i32) {
    if apply != 0 {
        set_i32(g.x(), p, round_to_pixel(at_i32(g.x(), p)));
    }
}

/// Rounds point `p`'s hinted y to the nearest whole pixel when `apply` is set.
/// Original: `glyph_hint_round_y_to_pixel_if_flag` (0x934460, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_round_y_to_pixel_if_flag(_h: GlyphHinter, g: HintGlyph, apply: i16, p: i32) {
    if apply != 0 {
        set_i32(g.y(), p, round_to_pixel(at_i32(g.y(), p)));
    }
}

/// Point `p`'s hinted coordinate in `coords`, moved toward hint value `slot` by at most 0x60 (none with
/// `NO_HINT_SLOT`) and rounded to a whole pixel.
unsafe fn snap_toward_slot(h: GlyphHinter, coords: usize, p: i32, slot: i16) {
    let mut v = at_i32(coords, p);
    if slot != NO_HINT_SLOT {
        v = approach(v, h.hint_value(slot), 0x60);
    }
    set_i32(coords, p, round_to_pixel(v));
}

/// Snaps point `p`'s hinted x toward hint value `slot` (at most 0x60) and to a pixel, when `apply` is set.
/// Original: `glyph_hint_snap_x_toward_target` (0x934480, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_snap_x_toward_target(h: GlyphHinter, g: HintGlyph, apply: i16, p: i32, slot: i16) {
    if apply != 0 {
        snap_toward_slot(h, g.x(), p, slot);
    }
}

/// Snaps point `p`'s hinted y toward hint value `slot` (at most 0x60) and to a pixel, when `apply` is set.
/// Original: `glyph_hint_snap_y_toward_target` (0x9344e0, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_snap_y_toward_target(h: GlyphHinter, g: HintGlyph, apply: i16, p: i32, slot: i16) {
    if apply != 0 {
        snap_toward_slot(h, g.y(), p, slot);
    }
}

/// Point `mid`'s hinted coordinate by linear interpolation between `a` and `b`, the original coordinates `oo` giving
/// the ratio (rounded to nearest, the division truncated).
unsafe fn interpolate_between(oo: usize, coords: usize, a: i32, mid: i32, b: i32) {
    let o = at_i16(oo, a) as i32;
    let span = (at_i16(oo, b) as i32).wrapping_sub(o);
    let base = at_i32(coords, a);
    let part = (at_i16(oo, mid) as i32).wrapping_sub(o).wrapping_mul(at_i32(coords, b).wrapping_sub(base)).wrapping_add(span >> 1);
    set_i32(coords, mid, (part / span).wrapping_add(base));
}

/// Sets point `mid`'s hinted x between `a` and `b` in the proportion of their original x.
/// Original: `glyph_hint_interpolate_x_between_points` (0x934540, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_interpolate_x_between_points(_h: GlyphHinter, g: HintGlyph, a: i32, mid: i32, b: i32) {
    interpolate_between(g.oox(), g.x(), a, mid, b);
}

/// Sets point `mid`'s hinted y between `a` and `b` in the proportion of their original y.
/// Original: `glyph_hint_interpolate_y_between_points` (0x934590, library_without_asserts, T2K autogrid, cdecl).
pub(crate) unsafe extern "C" fn glyph_hint_interpolate_y_between_points(_h: GlyphHinter, g: HintGlyph, a: i32, mid: i32, b: i32) {
    interpolate_between(g.ooy(), g.y(), a, mid, b);
}

/// Comparers: only the original (they change the hinter or the glyph; the hinter never runs in the game, which draws
/// without grid fitting; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_glyph_hint_reset_scale_search(h: GlyphHinter) {
    original_fn::<unsafe extern "C" fn(GlyphHinter)>(%%I0%%)(h)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_refine_scale_step(h: GlyphHinter, g: HintGlyph, x_axis: i16, slot: i16, p: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i16, i16)>(%%I1%%)(h, g, x_axis, slot, p)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_raise_to_max_of_pair(h: GlyphHinter, g: HintGlyph, x_axis: i16, from: i16, to: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i16, i16)>(%%I2%%)(h, g, x_axis, from, to)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_lower_to_min_of_pair(h: GlyphHinter, g: HintGlyph, x_axis: i16, from: i16, to: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i16, i16)>(%%I3%%)(h, g, x_axis, from, to)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_clamp_point_and_shift_companion(h: GlyphHinter, g: HintGlyph, x_axis: i16, limit: i16, p: i16, companion: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i16, i16, i16)>(%%I4%%)(h, g, x_axis, limit, p, companion)
}

/// The signature of the two distance setters.
pub(crate) type GlyphHintDistanceFn = unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32, i16, i32, i32, i32, i32, i32);

pub(crate) unsafe extern "C" fn compare_glyph_hint_set_x_distance_rounded(h: GlyphHinter, g: HintGlyph, slot: i16, a4: i32, m: i16, a6: i32, a7: i32, a8: i32, from: i32, to: i32) {
    original_fn::<GlyphHintDistanceFn>(%%I5%%)(h, g, slot, a4, m, a6, a7, a8, from, to)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_set_y_distance_rounded(h: GlyphHinter, g: HintGlyph, slot: i16, a4: i32, m: i16, a6: i32, a7: i32, a8: i32, from: i32, to: i32) {
    original_fn::<GlyphHintDistanceFn>(%%I6%%)(h, g, slot, a4, m, a6, a7, a8, from, to)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_round_x_to_pixel_if_flag(h: GlyphHinter, g: HintGlyph, apply: i16, p: i32) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32)>(%%I7%%)(h, g, apply, p)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_round_y_to_pixel_if_flag(h: GlyphHinter, g: HintGlyph, apply: i16, p: i32) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32)>(%%I8%%)(h, g, apply, p)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_snap_x_toward_target(h: GlyphHinter, g: HintGlyph, apply: i16, p: i32, slot: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32, i16)>(%%I9%%)(h, g, apply, p, slot)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_snap_y_toward_target(h: GlyphHinter, g: HintGlyph, apply: i16, p: i32, slot: i16) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i16, i32, i16)>(%%I10%%)(h, g, apply, p, slot)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_interpolate_x_between_points(h: GlyphHinter, g: HintGlyph, a: i32, mid: i32, b: i32) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i32, i32, i32)>(%%I11%%)(h, g, a, mid, b)
}

pub(crate) unsafe extern "C" fn compare_glyph_hint_interpolate_y_between_points(h: GlyphHinter, g: HintGlyph, a: i32, mid: i32, b: i32) {
    original_fn::<unsafe extern "C" fn(GlyphHinter, HintGlyph, i32, i32, i32)>(%%I12%%)(h, g, a, mid, b)
}
