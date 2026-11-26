// ---------- rx03_12: the T2K automatic hinter: fitting the outline on both axes (library_without_asserts, T2K autogrid) ----------

/// One edge of the outline fitting (14 bytes, in the block `glyph_hint_fit_outline_both_axes` allocates): +0 the point,
/// +2 its stem partner (−1 none), +4 its alignment-zone slot (−1 none, `NO_HINT_SLOT` for a plain glyph), +6 its group
/// (neighbouring edges closer than the group gap share one), +8 the group of its partner (−1 none; read back as an
/// edge index), +0xa the edge that points back to it (−1 none), +0xc its flags (`EDGE_*`).
#[derive(Clone, Copy)]
struct FitEdge(usize);

const FIT_EDGE_SIZE: i32 = 14;
/// Edge flags: merged into a neighbour of the same group, placed on the grid, an alignment-zone candidate.
const EDGE_MERGED: u16 = 1;
const EDGE_PLACED: u16 = 2;
const EDGE_ZONE: u16 = 4;
/// The constraint kind of a stem (the hinter's constraint list at +0x58).
const CONSTRAINT_STEM: u8 = 3;

impl FitEdge {
    unsafe fn point(self) -> i32 { read_i16(self.0) as i32 }
    unsafe fn partner(self) -> i32 { read_i16(self.0 + 2) as i32 }
    unsafe fn slot(self) -> i16 { read_i16(self.0 + 4) }
    unsafe fn set_slot(self, v: i16) { write_i16(self.0 + 4, v) }
    unsafe fn group(self) -> i16 { read_i16(self.0 + 6) }
    unsafe fn set_group(self, v: i16) { write_i16(self.0 + 6, v) }
    unsafe fn partner_group(self) -> i16 { read_i16(self.0 + 8) }
    unsafe fn set_partner_group(self, v: i16) { write_i16(self.0 + 8, v) }
    unsafe fn link(self) -> i16 { read_i16(self.0 + 0xa) }
    unsafe fn set_link(self, v: i16) { write_i16(self.0 + 0xa, v) }
    unsafe fn flags(self) -> u16 { read_u16(self.0 + 0xc) }
    unsafe fn has(self, f: u16) -> bool { self.flags() & f != 0 }
    unsafe fn add(self, f: u16) { write_u16(self.0 + 0xc, self.flags() | f) }
}

/// The sorted edge list: an array of pointers to `FitEdge`s (indexes as the original reads them, unchecked).
#[derive(Clone, Copy)]
struct FitEdges(usize);

impl FitEdges {
    unsafe fn at(self, k: i32) -> FitEdge { FitEdge(read_usize(self.0.wrapping_add((k as usize).wrapping_mul(4)))) }
    unsafe fn set(self, k: i32, e: FitEdge) { write_usize(self.0.wrapping_add((k as usize).wrapping_mul(4)), e.0) }
}

/// What one pass of the fitting works on: the x pass (first) or the y pass.
struct FitAxis {
    x: bool,
    /// The original coordinates along the axis and across it (the hinter's +0x12/+0x16).
    along: usize,
    across: usize,
    /// The per-point hint flag that marks a point placed on this axis, the one left on a merged edge, the one of an
    /// extremum candidate and the one of an edge point.
    placed: u16,
    merged: u16,
    candidate: u16,
    edge: u16,
    /// The axis class of the stem constraints of this pass.
    class: u8,
}

/// Fits a glyph to the pixel grid after the hinter's analysis, first along x and then along y. Each pass:
/// - builds the edges: one per stem constraint of the axis (kind 3; the partner kept only when the stem is wider along
///   the axis than an eighth of its length across it), one per zone point (flag 0x40, y pass only, a zone candidate),
///   one per edge point (flag 0x5000 in x, 0x2800 in y), the lowest and the highest extremum candidate (flag 4 in x, 8
///   in y; zone candidates when the glyph is plain) and, in the x pass of a mode 1 non-figure glyph, the two phantom
///   points after the last point;
/// - sorts them by coordinate (heap sort), groups neighbours closer than units per em / 1000 + 1, gives the zone
///   candidates their nearest alignment zone (or `NO_HINT_SLOT` for a plain glyph) and merges edges of one group into
///   one (the merged one leaves the axis' merged flag on its point);
/// - places the first edge (snapped to its zone, or rounded), then each edge in order: from its placed partner by the
///   stem widths, or between the edges before it; the zone edges refine the scale search, which restarts the pass from
///   the last zone edge while it runs; the unplaced partner is placed from it and the edges between are interpolated;
/// - in the x pass of a mode 1 non-figure glyph, adjusts the stem between the extrema and the phantom points;
/// - interpolates the untouched candidate points that lie between two consecutive edges.
///
/// Then the untouched points of both axes are interpolated. In the first pass, a glyph that is not a figure or is not
/// in mode 1 first gets its first phantom point rounded and the advance set from it. In mode 2 a fixed polynomial of the
/// contour count, the point count and the candidate count decides whether the glyph is plain (no zones).
/// The original also calls the empty hook 0x5a8f60 at each pass and the two maximum stem width functions without using
/// their results; the copy leaves them out.
/// Defects of the original: the partner group (+8) is read back as an edge index (`edges[group]`), not a group; an
/// edge's +0xa link stays as the allocator left it unless the compaction or the linking writes it.
/// Original: `glyph_hint_fit_outline_both_axes` (0x93c680, library_without_asserts, T2K autogrid, cdecl: glyph,
/// hinter).
pub(crate) unsafe extern "C" fn glyph_hint_fit_outline_both_axes(g: HintGlyph, h: GlyphHinter) -> i32 {
    let tenth = read_i16(h.0 + 0x29c) as i32 / 100;
    let group_gap = tenth / 10 + 1;
    let near_gap = tenth * 3 + 1;
    let groups = read_usize(h.0 + 0x3c);
    let last = h.contour_end(h.contour_count() as i32 - 1);
    let capacity = last * 2 + 0x12;
    // the scale search's state of the fitting (+0x3f8..+0x3fc, +0x43e, +0x440)
    write_i16(h.0 + 0x3fc, 0);
    write_i16(h.0 + 0x3fa, 0);
    write_i16(h.0 + 0x3f8, 0);
    write_i16(h.0 + 0x43e, 1);
    write_i16(h.0 + 0x440, 0);
    let mem = read_usize(h.0 + 0x444);
    let records = lib_mem_alloc(mem, capacity.wrapping_mul(FIT_EDGE_SIZE));
    let edges = FitEdges(lib_mem_alloc(mem, capacity.wrapping_mul(4)));
    let mut plain = false;
    for pass in 0..2 {
        covered!("rx03_12 fit pass");
        let axis = start_fit_axis(h, g, pass == 0, last);
        let (n, low, high) = collect_fit_edges(h, g, &axis, records, edges, last, &mut plain);
        sort_fit_edges(edges, n, axis.along);
        group_fit_edges(h, &axis, edges, n, groups, last, group_gap, plain);
        merge_fit_edges(&axis, edges, n);
        let m = compact_fit_edges(h, &axis, edges, n);
        if m <= 0 {
            continue;
        }
        place_fit_edges(h, g, &axis, edges, m, near_gap);
        if axis.x && h.is_figure() == 0 && h.mode() == 1 {
            glyph_hint_adjust_x_stem_edges(h, g, last + 1, low, high, last + 2);
        }
        interpolate_between_fit_edges(h, g, &axis, edges, m, last);
    }
    glyph_hint_interpolate_untouched_x(h, g);
    glyph_hint_interpolate_untouched_y(h, g);
    lib_mem_free(mem, records);
    lib_mem_free(mem, edges.0);
    0
}

/// The start of a pass: the axis' arrays and flags; in the x pass of a figure or non mode 1 glyph, the first phantom
/// point rounded and the advance set from it.
unsafe fn start_fit_axis(h: GlyphHinter, g: HintGlyph, x: bool, last: i32) -> FitAxis {
    if x {
        if h.is_figure() != 0 || h.mode() != 1 {
            let phantom = (last + 1) as i16 as i32;
            glyph_hint_round_x_to_pixel_if_flag(h, g, 1, phantom);
            glyph_hint_set_x_distance_rounded(h, g, -1, 0, 0, 1, 0x47, 0x72, phantom, (last + 2) as i16 as i32);
        }
        FitAxis { x, along: h.oox(), across: h.ooy(), placed: 0x10, merged: 1, candidate: 4, edge: 0x5000, class: 1 }
    } else {
        FitAxis { x, along: h.ooy(), across: h.oox(), placed: 0x20, merged: 2, candidate: 8, edge: 0x2800, class: 2 }
    }
}

/// A new edge at `records[*n]`, listed at `edges[*n]`: its point, partner and flags (the rest is filled later).
unsafe fn push_fit_edge(records: usize, edges: FitEdges, n: &mut i32, point: i32, partner: i32, flags: u16) -> FitEdge {
    let e = FitEdge(records.wrapping_add((*n as usize).wrapping_mul(FIT_EDGE_SIZE as usize)));
    edges.set(*n, e);
    write_i16(e.0, point as i16);
    write_i16(e.0 + 2, partner as i16);
    write_u16(e.0 + 0xc, flags);
    *n += 1;
    e
}

/// The edges of a pass (see `glyph_hint_fit_outline_both_axes`); returns their count and the points of the lowest and
/// the highest extremum candidate. In the x pass of a mode 2 glyph it also decides `plain`.
unsafe fn collect_fit_edges(h: GlyphHinter, g: HintGlyph, axis: &FitAxis, records: usize, edges: FitEdges, last: i32, plain: &mut bool) -> (i32, i32, i32) {
    let (along, across) = (axis.along, axis.across);
    let mut n = 0;
    let constraints = read_usize(h.0 + 0x58);
    for k in 0..read_i32(h.0 + 0x50) {
        let c = constraints.wrapping_add((k as usize).wrapping_mul(8));
        if read_u8(c) != CONSTRAINT_STEM || read_u8(c + 1) != axis.class {
            continue;
        }
        let (a, b) = (read_i16(c + 4) as i32, read_i16(c + 6) as i32);
        let width = (at_i16(along, a) as i32 - at_i16(along, b) as i32).abs();
        let length = (at_i16(across, a) as i32 - at_i16(across, b) as i32).abs();
        push_fit_edge(records, edges, &mut n, a, if width <= length >> 3 { -1 } else { b }, 0);
    }
    let flags = h.flags();
    let (mut low, mut low_at, mut high, mut high_at) = (at_i16(along, 0) as i32, 0, at_i16(along, 0) as i32, 0);
    let mut candidates: u32 = 0;
    for p in 0..=last {
        let f = at_i16(flags, p) as u16;
        if !axis.x && f & 0x40 != 0 {
            push_fit_edge(records, edges, &mut n, p, -1, EDGE_ZONE);
        }
        if f & axis.candidate != 0 {
            candidates = candidates.wrapping_add(1);
            let v = at_i16(along, p) as i32;
            if v > high {
                high = v;
                high_at = p;
            } else if v < low {
                low = v;
                low_at = p;
            }
            if f & axis.edge != 0 {
                push_fit_edge(records, edges, &mut n, p, -1, 0);
            }
        }
    }
    if axis.x && h.mode() == 2 {
        *plain = plain_glyph_score(read_i16(g.0) as i32, last, candidates & 0x3f) > 0;
    }
    let zone = if *plain { EDGE_ZONE } else { 0 };
    push_fit_edge(records, edges, &mut n, low_at, -1, zone);
    push_fit_edge(records, edges, &mut n, high_at, -1, zone);
    if axis.x && h.is_figure() == 0 && h.mode() == 1 {
        push_fit_edge(records, edges, &mut n, last + 1, -1, 0);
        push_fit_edge(records, edges, &mut n, last + 2, -1, 0);
    }
    (n, low_at, high_at)
}

/// The mode 2 test of a plain glyph: a fixed cubic polynomial (10-bit fixed point) of the contour count (at most 16),
/// the last point (at most 0x12f) and the candidate count (6 bits), above 0 for a plain glyph; small glyphs (under 4
/// contours, 15 candidates and 0x57 points, fewer candidates than five per contour) with a score below 0x14d are not.
fn plain_glyph_score(contours: i32, last: i32, candidates: u32) -> i32 {
    let contours = contours.min(0x10);
    let points = last.min(0x12f);
    let a = contours.wrapping_mul(0x162).wrapping_sub(0x4e9);
    let b = points.wrapping_mul(0x12).wrapping_sub(0x55c);
    let c = (candidates as i32).wrapping_mul(0x51).wrapping_sub(0x4f6);
    let m = |x: i32, y: i32| x.wrapping_mul(y) >> 10;
    let (ab, aa, cc, bb) = (m(b, a), m(a, a), m(c, c), m(b, b));
    let mut s = m(m(c, b), 0x172);
    s = s.wrapping_sub(m(m(ab, c), 0x4e));
    s = s.wrapping_add(m(m(cc, c), 0x57));
    s = s.wrapping_add(m(m(c, a), 100));
    s = s.wrapping_add(m(m(aa, a), 0x39));
    s = s.wrapping_sub(m(cc, 0x2ad));
    s = s.wrapping_sub(m(aa, 0x133));
    s = s.wrapping_sub(m(bb, 0x8c));
    s = s.wrapping_add(bb.wrapping_mul(b) >> 0x12);
    s = s.wrapping_add(m(c, 0x3ac));
    s = s.wrapping_add(m(a, 0x16c));
    s = s.wrapping_sub(b >> 3);
    let score = (ab >> 4).wrapping_add(s).wrapping_add(0x237);
    if score > 0 && score < 0x14d && contours <= 3 && candidates <= 0xe && points <= 0x56 && contours * 5 > candidates as i32 {
        0
    } else {
        score
    }
}

/// Heap sort of the edge list by the coordinate of each edge's point along the axis (ascending).
unsafe fn sort_fit_edges(edges: FitEdges, n: i32, along: usize) {
    let mut k = (n - 2) / 2;
    while k >= 0 {
        glyph_hint_heap_sift_down_by_coord(edges.0, k, n - 1, along);
        k -= 1;
    }
    let mut end = n - 1;
    while end > 0 {
        let top = edges.at(end);
        edges.set(end, edges.at(0));
        edges.set(0, top);
        end -= 1;
        glyph_hint_heap_sift_down_by_coord(edges.0, 0, end, along);
    }
}

/// The groups: the per-point group array (the hinter's +0x3c, −1 for the points and the two phantom points first)
/// and each edge's group (a new one when the gap to the edge before is larger than `group_gap`); then each edge's
/// partner group and the zone slot of the zone candidates (`NO_HINT_SLOT` for a plain glyph).
unsafe fn group_fit_edges(h: GlyphHinter, axis: &FitAxis, edges: FitEdges, n: i32, groups: usize, last: i32, group_gap: i32, plain: bool) {
    for p in 0..last + 3 {
        write_i16(groups.wrapping_add((p as usize).wrapping_mul(2)), -1);
    }
    let set_group = |e: FitEdge, v: i16| {
        e.set_group(v);
        write_i16(groups.wrapping_add((e.point() as usize).wrapping_mul(2)), v);
    };
    let mut group: i16 = 0;
    set_group(edges.at(0), 0);
    for k in 1..n {
        let (prev, e) = (edges.at(k - 1), edges.at(k));
        if (at_i16(axis.along, e.point()) as i32 - at_i16(axis.along, prev.point()) as i32).abs() > group_gap {
            group += 1;
        }
        set_group(e, group);
    }
    for k in 0..n {
        let e = edges.at(k);
        let partner = e.partner();
        e.set_partner_group(if partner < 0 { -1 } else { at_i16(groups, partner) });
        if !e.has(EDGE_ZONE) {
            e.set_slot(-1);
        } else if plain {
            e.set_slot(NO_HINT_SLOT);
        } else {
            e.set_slot(glyph_hint_find_nearest_alignment_zone(h, e.point()));
        }
    }
}

/// Merges neighbouring edges of one group: the kept edge takes the other's flags, its zone slot and partner group
/// when it has none, and the other is marked merged. Which one is kept depends on whether the two points share the
/// coordinate along the axis, on which is higher across it (the new edge's point against the old one's partner) and
/// on the first and last edge of the list.
unsafe fn merge_fit_edges(axis: &FitAxis, edges: FitEdges, n: i32) {
    let absorb = |keep: FitEdge, gone: FitEdge| {
        keep.add(gone.flags());
        if keep.slot() < 0 {
            keep.set_slot(gone.slot());
        }
        if gone.partner_group() >= 0 && keep.partner_group() < 0 {
            keep.set_partner_group(gone.partner_group());
        }
        gone.add(EDGE_MERGED);
    };
    let mut j = 0;
    for i in 1..n {
        let (old, new) = (edges.at(j), edges.at(i));
        if old.group() != new.group() {
            j = i;
            continue;
        }
        let higher = at_i16(axis.across, new.point()) > at_i16(axis.across, old.partner());
        let keep_old = if at_i16(axis.along, new.point()) == at_i16(axis.along, old.partner()) {
            higher
        } else if higher {
            i != n - 1
        } else {
            j == 0 || j == n - 1
        };
        if keep_old {
            covered!("rx03_12 merge into old");
            absorb(old, new);
        } else {
            absorb(new, old);
            j = i;
        }
    }
}

/// Drops the merged edges (their points get the axis' merged flag) and links each edge's partner edge back to it;
/// returns the count left.
unsafe fn compact_fit_edges(h: GlyphHinter, axis: &FitAxis, edges: FitEdges, n: i32) -> i32 {
    let flags = h.flags();
    let mut m = 0;
    for k in 0..n {
        let e = edges.at(k);
        if e.has(EDGE_MERGED) {
            let f = flags.wrapping_add((e.point() as usize).wrapping_mul(2));
            write_u16(f, read_u16(f) | axis.merged);
        } else {
            e.set_link(-1);
            edges.set(m, e);
            m += 1;
        }
    }
    for k in 0..m {
        let pg = edges.at(k).partner_group();
        if pg >= 0 {
            let target = edges.at(pg as i32);
            if target.partner_group() as i32 != k {
                target.set_link(k as i16);
            }
        }
    }
    m
}

/// Marks point `p` placed on the axis.
unsafe fn mark_fit_point(h: GlyphHinter, axis: &FitAxis, p: i32) {
    let f = h.flags().wrapping_add((p as usize).wrapping_mul(2));
    write_u16(f, read_u16(f) | axis.placed);
}

/// Rounds point `p` to a pixel along the axis.
unsafe fn round_fit_point(h: GlyphHinter, g: HintGlyph, axis: &FitAxis, p: i32) {
    if axis.x {
        glyph_hint_round_x_to_pixel_if_flag(h, g, 1, p);
    } else {
        glyph_hint_round_y_to_pixel_if_flag(h, g, 1, p);
    }
}

/// Interpolates point `mid` between points `a` and `b` along the axis.
unsafe fn interpolate_fit_point(h: GlyphHinter, g: HintGlyph, axis: &FitAxis, a: i32, mid: i32, b: i32) {
    if axis.x {
        glyph_hint_interpolate_x_between_points(h, g, a, mid, b);
    } else {
        glyph_hint_interpolate_y_between_points(h, g, a, mid, b);
    }
}

/// Places the edges in order (see `glyph_hint_fit_outline_both_axes`). `zone_at` is the last edge placed on a zone
/// (the restart point of the scale search), `reach` the furthest edge placed from a partner.
unsafe fn place_fit_edges(h: GlyphHinter, g: HintGlyph, axis: &FitAxis, edges: FitEdges, m: i32, near_gap: i32) {
    let (x, other) = (axis.x as i16, (!axis.x) as i16);
    let first = edges.at(0);
    if first.slot() >= 0 {
        if axis.x {
            glyph_hint_snap_x_toward_target(h, g, 1, first.point(), first.slot());
        } else {
            glyph_hint_snap_y_toward_target(h, g, 1, first.point(), first.slot());
        }
    } else {
        round_fit_point(h, g, axis, first.point());
    }
    mark_fit_point(h, axis, first.point());
    first.add(EDGE_PLACED);
    glyph_hint_reset_scale_search(h);
    let (mut zone_at, mut reach) = (0, 0);
    let mut i = 0;
    while i < m {
        let e = edges.at(i);
        let p = e.point();
        // the placed edge to measure from (the larger of the partner group and the link) and the unplaced partner
        let (mut anchor, mut partner) = (-1, -1);
        let pg = e.partner_group() as i32;
        if pg >= 0 {
            if edges.at(pg).has(EDGE_PLACED) { anchor = pg } else { partner = pg }
        }
        let link = e.link() as i32;
        if link >= 0 {
            if edges.at(link).has(EDGE_PLACED) {
                if anchor < 0 || anchor <= link {
                    anchor = link;
                }
            } else if partner < 0 || partner >= link {
                partner = link;
            }
        }
        if !e.has(EDGE_PLACED) {
            if anchor >= 0 {
                glyph_hint_set_distance_by_stem_width(h, g, axis.along, x, other, 0x40, 1, 0x42, 0x6c, edges.at(anchor).point(), p);
            } else {
                let reference = fit_reference_point(axis, edges, i, p, near_gap);
                glyph_hint_place_point_between_two_refs(h, g, x, other as i32, edges.at(zone_at).point() as i16, reference as i16, p as i16);
            }
            glyph_hint_raise_to_max_of_pair(h, g, x, edges.at(i - 1).point() as i16, p as i16);
            if reach > i {
                glyph_hint_lower_to_min_of_pair(h, g, x, edges.at(reach).point() as i16, p as i16);
            }
            mark_fit_point(h, axis, p);
            edges.at(i).add(EDGE_PLACED);
        }
        let slot = edges.at(i).slot();
        if slot >= 0 && i != zone_at {
            glyph_hint_refine_scale_step(h, g, x, slot, p as i16);
            if read_i32(h.0 + 0x432) != 0 {
                // the scale search goes on: back to the last zone edge, the edges after it unplaced
                covered!("rx03_12 scale search restart");
                reach = zone_at;
                i = m - 1;
                if i > zone_at {
                    for k in (zone_at + 1..=i).rev() {
                        let r = edges.at(k).0 + 0xc;
                        write_u16(r, read_u16(r) & !EDGE_PLACED);
                    }
                    i = zone_at;
                }
                continue;
            }
            if read_i32(h.0 + 0x42e) != 0 {
                let (a, b) = (edges.at(zone_at).point(), p);
                for k in zone_at + 1..i {
                    let q = edges.at(k).point();
                    interpolate_fit_point(h, g, axis, a, q, b);
                    if read_i32(h.0 + 0x448) == 0 {
                        round_fit_point(h, g, axis, q);
                    }
                }
            }
            zone_at = i;
        }
        if partner >= 0 && !edges.at(partner).has(EDGE_PLACED) {
            place_fit_partner(h, g, axis, edges, i, p, partner, zone_at, &mut reach);
        }
        i += 1;
    }
}

/// The reference point of an edge with no placed anchor: the edge before it, or the one before that when the two
/// last edges are far apart along the axis (beyond `near_gap`), the gap across is more than twice the gap along, and
/// the edge two before is within an eighth of the gap across.
unsafe fn fit_reference_point(axis: &FitAxis, edges: FitEdges, i: i32, p: i32, near_gap: i32) -> i32 {
    let prev = edges.at(i - 1).point();
    if i <= 1 {
        return prev;
    }
    let prev2 = edges.at(i - 2).point();
    let along = (at_i16(axis.along, p) as i32 - at_i16(axis.along, prev) as i32).abs();
    let across = (at_i16(axis.across, p) as i32 - at_i16(axis.across, prev) as i32).abs();
    let across2 = (at_i16(axis.across, p) as i32 - at_i16(axis.across, prev2) as i32).abs();
    if along > near_gap && across > along * 2 && across2 < across / 8 {
        prev2
    } else {
        prev
    }
}

/// Places the unplaced partner edge of edge `i` from it by the stem widths; when the partner is further than any
/// placed so far, the zone slots of the edges between it and the last zone edge (from the last one with a slot) are
/// dropped, otherwise the partner is clamped against the furthest one; then the unplaced edges between `i` and the
/// partner with no partner group nor link are interpolated and rounded.
unsafe fn place_fit_partner(h: GlyphHinter, g: HintGlyph, axis: &FitAxis, edges: FitEdges, i: i32, p: i32, partner: i32, zone_at: i32, reach: &mut i32) {
    let (x, other) = (axis.x as i16, (!axis.x) as i16);
    let q = edges.at(partner).point();
    glyph_hint_set_distance_by_stem_width(h, g, axis.along, x, other, 0x40, 1, 0x42, 0x6c, p, q);
    edges.at(partner).add(EDGE_PLACED);
    mark_fit_point(h, axis, q);
    if partner > *reach {
        *reach = partner;
        let mut k = partner;
        while k > zone_at && edges.at(k).slot() < 0 {
            k -= 1;
        }
        for j in (zone_at + 1..k).rev() {
            edges.at(j).set_slot(-1);
        }
    } else {
        glyph_hint_clamp_point_and_shift_companion(h, g, x, edges.at(*reach).point() as i16, q as i16, p as i16);
    }
    let a = edges.at(i).point();
    let b = edges.at(partner).point();
    for k in i + 1..partner {
        let e = edges.at(k);
        let r = e.point();
        if !e.has(EDGE_PLACED) && e.partner_group() < 0 && e.link() < 0 {
            interpolate_fit_point(h, g, axis, a, r, b);
            round_fit_point(h, g, axis, r);
            e.add(EDGE_PLACED);
            mark_fit_point(h, axis, r);
        }
    }
}

/// The candidate points (merged flag of the axis) not yet placed that lie between two consecutive edges along the axis
/// are interpolated between them, unless a contour neighbour already placed sits at the same coordinate.
unsafe fn interpolate_between_fit_edges(h: GlyphHinter, g: HintGlyph, axis: &FitAxis, edges: FitEdges, m: i32, last: i32) {
    let flags = h.flags();
    let along = |p: i32| at_i16(axis.along, p);
    let placed_at = |p: i32, v: i16| at_i16(flags, p) as u16 & axis.placed != 0 && v == along(p);
    let mut prev = edges.at(0).point();
    for k in 1..m {
        let cur = edges.at(k).point();
        for p in 0..=last {
            let f = at_i16(flags, p) as u16;
            let v = along(p);
            if f & axis.merged == 0 || f & axis.placed != 0 || v < along(prev) || v > along(cur) {
                continue;
            }
            if placed_at(h.prev(p), v) || placed_at(h.next(p), v) {
                continue;
            }
            interpolate_fit_point(h, g, axis, prev, p, cur);
            let fp = flags.wrapping_add((p as usize).wrapping_mul(2));
            write_u16(fp, read_u16(fp) | axis.placed);
        }
        prev = cur;
    }
}

/// Comparer: only the original (the fitting changes the glyph and the hinter; it does not run in the game, which never
/// grid-fits; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_glyph_hint_fit_outline_both_axes(g: HintGlyph, h: GlyphHinter) -> i32 {
    original_fn::<unsafe extern "C" fn(HintGlyph, GlyphHinter) -> i32>(%%I0%%)(g, h)
}
