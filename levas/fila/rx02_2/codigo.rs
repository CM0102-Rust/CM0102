// ---------- rx02_2: the T2K TrueType interpreter: ROLL and the point moving opcodes MDAP, MIAP, IUP, SHP, SHC, SHZ, SHPIX, IP and MSIRP (library_without_asserts, T2K fnt) ----------

/// Opcode 0x8A, ROLL: the third value from the top moves to the top, the two above it one place down.
/// Original: `tt_op_roll` (0x938ca0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_roll(gs: TtLocalState) {
    let sp = gs.stack_ptr();
    let (a, b, c) = (read_i32(sp - 4), read_i32(sp - 8), read_i32(sp - 12));
    write_i32(sp - 8, a);
    write_i32(sp - 12, b);
    write_i32(sp - 4, c);
}

/// Point `p`'s current coordinates in `zone`.
pub(crate) unsafe fn tt_current(zone: TtZone, p: i32) -> (i32, i32) {
    (at_i32(zone.x(), p), at_i32(zone.y(), p))
}

/// Point `p`'s original (scaled) coordinates in `zone`.
pub(crate) unsafe fn tt_original(zone: TtZone, p: i32) -> (i32, i32) {
    (at_i32(zone.ox(), p), at_i32(zone.oy(), p))
}

/// Point `p`'s original coordinates in font units in `zone`.
pub(crate) unsafe fn tt_original_units(zone: TtZone, p: i32) -> (i32, i32) {
    (at_i16(zone.oox(), p) as i32, at_i16(zone.ooy(), p) as i32)
}

/// The current distance from point `q` of `zq` to point `p` of `zp` along the projection vector.
pub(crate) unsafe fn tt_project_between(gs: TtLocalState, zp: TtZone, p: i32, zq: TtZone, q: i32) -> i32 {
    let ((px, py), (qx, qy)) = (tt_current(zp, p), tt_current(zq, q));
    gs.project(px.wrapping_sub(qx), py.wrapping_sub(qy))
}

/// The original (scaled) distance from point `q` of `zq` to point `p` of `zp` along the dual projection vector.
pub(crate) unsafe fn tt_dual_project_between(gs: TtLocalState, zp: TtZone, p: i32, zq: TtZone, q: i32) -> i32 {
    let ((px, py), (qx, qy)) = (tt_original(zp, p), tt_original(zq, q));
    gs.dual_project(px.wrapping_sub(qx), py.wrapping_sub(qy))
}

/// Opcodes 0x2E and 0x2F, MDAP: pops a point of zone 0 and makes it reference points 0 and 1; with bit 0 (MDAP[r])
/// it moves the point to its rounded position along the projection vector, else it only touches it (a move of 0).
/// Original: `tt_op_mdap` (0x938cc0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_mdap(gs: TtLocalState) {
    let zone = gs.zone(0);
    let p = gs.pop();
    gs.set_reference_point(1, p);
    gs.set_reference_point(0, p);
    if gs.opcode() & 1 != 0 {
        let (x, y) = tt_current(zone, p);
        let d = gs.project(x, y);
        let rounded = tt_round(gs, d);
        gs.move_point(zone, p, rounded.wrapping_sub(d));
    } else {
        gs.move_point(zone, p, 0);
    }
}

/// Opcodes 0x3E and 0x3F, MIAP: pops a control value index and a point of zone 0, which becomes reference points 0
/// and 1. In the twilight zone the point is first put at the control value along the projection vector (original and
/// current). Then it moves to the control value along the projection vector; with bit 0 (MIAP[r]) the control value
/// is replaced by the current position when they differ by more than the control value cut-in, and rounded.
/// Original: `tt_op_miap` (0x938d40, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_miap(gs: TtLocalState) {
    let g = gs.global();
    let zone = gs.zone(0);
    let n = gs.pop();
    let mut v = gs.read_cvt(n);
    let p = gs.pop();
    gs.set_reference_point(1, p);
    gs.set_reference_point(0, p);
    if zone.0 == gs.zones() {
        set_i32(zone.ox(), p, lib_f2dot14_mul(v, gs.proj().0));
        set_i32(zone.x(), p, at_i32(zone.ox(), p));
        set_i32(zone.oy(), p, lib_f2dot14_mul(v, gs.proj().1));
        set_i32(zone.y(), p, at_i32(zone.oy(), p));
    }
    let (x, y) = tt_current(zone, p);
    let d = gs.project(x, y);
    if gs.opcode() & 1 != 0 {
        if v.wrapping_sub(d).wrapping_abs() > g.cvt_cut_in() {
            v = d;
        }
        v = tt_round(gs, v);
    }
    gs.move_point(zone, p, v.wrapping_sub(d));
}

/// Opcodes 0x30 and 0x31, IUP: in each contour of zone 2, the points not touched along the axis (y for 0x30, x for
/// 0x31) between two touched ones follow them. With `lo` and `hi` the touched pair ordered by their font-unit
/// coordinate, each untouched point at or below `lo`'s original coordinate moves as `lo` moved, at or above `hi`'s as
/// `hi` moved, and between them it is interpolated by its font-unit coordinate: with small spans (under 0x8000 units
/// and 0x8000 26.6) by `(u − u_lo) × span / units`, rounded to the nearest, otherwise by a 16.16 ratio. Points
/// between a pair at the same font-unit coordinate all move as `lo`. A contour without a touched point is left alone.
/// In the small span case a point at or below `lo` that is also below `hi` moves as `lo`, else as `hi` (the original's
/// order of the tests).
/// Original: `tt_op_iup` (0x938e20, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_iup(gs: TtLocalState) {
    let zone = gs.zone(2);
    let flags = zone.flags();
    let (cur, org, units, mask) = if gs.opcode() & 1 != 0 {
        (zone.x(), zone.ox(), zone.oox(), 1u8)
    } else {
        (zone.y(), zone.oy(), zone.ooy(), 2u8)
    };
    let touched = |p: i32| read_u8(flags.wrapping_add(p as usize)) & mask != 0;
    let mut c: i16 = 0;
    while c < zone.contour_count() {
        let start = at_i16(zone.contour_starts(), c as i32) as i32;
        let end = at_i16(zone.contour_ends(), c as i32) as i32;
        c += 1;
        let next = |p: i32| if p == end { start } else { p + 1 };
        let mut first = start;
        while !touched(first) && first <= end {
            first += 1;
        }
        if first > end {
            continue;
        }
        let mut p = first;
        loop {
            let a = p;
            p = next(p);
            if p == first {
                break;
            }
            if touched(p) {
                continue;
            }
            let mut b = next(p);
            while !touched(b) {
                b = next(b);
            }
            iup_interpolate_run(cur, org, units, p, b, a, next);
            p = b;
            if p == first {
                break;
            }
        }
    }
}

/// IUP over the untouched points from `from` up to the touched point `b` (excluded), between the touched points `a`
/// and `b` (see `tt_op_iup`).
unsafe fn iup_interpolate_run(cur: usize, org: usize, units: usize, from: i32, b: i32, a: i32, next: impl Fn(i32) -> i32) {
    let (lo, hi) = if at_i16(units, a) < at_i16(units, b) { (a, b) } else { (b, a) };
    let (cur_lo, org_lo, u_lo) = (at_i32(cur, lo), at_i32(org, lo), at_i16(units, lo) as i32);
    let delta_lo = cur_lo.wrapping_sub(org_lo);
    let span_units = (at_i16(units, hi) as i32).wrapping_sub(u_lo);
    let mut p = from;
    if span_units == 0 {
        while p != b {
            set_i32(cur, p, at_i32(cur, p).wrapping_add(delta_lo));
            p = next(p);
        }
        return;
    }
    let (cur_hi, org_hi) = (at_i32(cur, hi), at_i32(org, hi));
    let delta_hi = cur_hi.wrapping_sub(org_hi);
    let span = cur_hi.wrapping_sub(cur_lo);
    if span_units < 0x8000 && span < 0x8000 {
        let half = span_units >> 1;
        while p != b {
            let o = at_i32(org, p);
            let v = if o <= org_lo {
                o.wrapping_add(if o < org_hi { delta_lo } else { delta_hi })
            } else if o >= org_hi {
                o.wrapping_add(delta_hi)
            } else {
                let u = (at_i16(units, p) as i32).wrapping_sub(u_lo);
                u.wrapping_mul(span).wrapping_add(half).wrapping_div(span_units).wrapping_add(cur_lo)
            };
            set_i32(cur, p, v);
            p = next(p);
        }
    } else {
        let mut ratio = None;
        while p != b {
            let o = at_i32(org, p);
            let v = if o <= org_lo {
                o.wrapping_add(delta_lo)
            } else if o >= org_hi {
                o.wrapping_add(delta_hi)
            } else {
                covered!("rx02_2 iup ratio");
                let r = *ratio.get_or_insert_with(|| lib_fixed_div(span, span_units));
                lib_fixed_mul((at_i16(units, p) as i32).wrapping_sub(u_lo), r).wrapping_add(cur_lo)
            };
            set_i32(cur, p, v);
            p = next(p);
        }
    }
}

/// How far reference point 1 (zone 0, opcode bit 0 set) or reference point 2 (zone 1) moved from its original
/// position, measured along the projection vector and split along the freedom vector: the x part is written to `dx`
/// only when the freedom vector's x is non-zero, the y part to `dy` likewise (each the distance × the part ÷ the
/// projection·freedom product, or × the part when that is 1.0); the point goes to `rp`. Returns the zone.
/// Original: `tt_compute_rp_displacement` (0x9391a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_compute_rp_displacement(gs: TtLocalState, dx: usize, dy: usize, rp: usize) -> TtZone {
    let (p, zone) = if gs.opcode() & 1 != 0 { (gs.reference_point(1), gs.zone(0)) } else { (gs.reference_point(2), gs.zone(1)) };
    let ((x, y), (ox, oy)) = (tt_current(zone, p), tt_original(zone, p));
    let d = gs.project(x.wrapping_sub(ox), y.wrapping_sub(oy));
    let along = |part: i16| {
        let dot = gs.proj_dot_free();
        if dot == TT_ONE { lib_f2dot14_mul(d, part) } else { lib_fixed_mul(d, lib_fixed_div(part as i32, dot as i32)) }
    };
    let (fx, fy) = gs.free();
    if fx != 0 {
        write_i32(dx, along(fx));
    }
    if fy != 0 {
        write_i32(dy, along(fy));
    }
    write_i32(rp, p);
    zone
}

/// Opcodes 0x32 and 0x33, SHP: pops loop-count points of zone 2 and shifts each by the reference point's displacement
/// (`tt_compute_rp_displacement`), marking it touched on each axis the freedom vector moves; the loop resets to 1.
/// Original: `tt_op_shp` (0x939110, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_shp(gs: TtLocalState) {
    let (mut dx, mut dy, mut rp) = (0i32, 0i32, 0i32);
    tt_compute_rp_displacement(gs, &mut dx as *mut i32 as usize, &mut dy as *mut i32 as usize, &mut rp as *mut i32 as usize);
    let zone = gs.zone(2);
    let count = gs.loop_counter();
    if count >= 0 {
        for _ in 0..=count {
            let p = gs.pop();
            let (fx, fy) = gs.free();
            if fx != 0 {
                zone.shift_and_touch(zone.x(), p, dx, 1);
            }
            if fy != 0 {
                zone.shift_and_touch(zone.y(), p, dy, 2);
            }
        }
    }
    gs.set_loop(0);
}

/// Opcodes 0x34 and 0x35, SHC: pops a contour of zone 2 and shifts its points by the reference point's displacement,
/// skipping the reference point itself; an axis whose shift is 0 is neither moved nor touched. A contour number past
/// the last one, or a contour with its end before its start, changes nothing.
/// Original: `tt_op_shc` (0x9392a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_shc(gs: TtLocalState) {
    let (mut dx, mut dy, mut rp) = (0i32, 0i32, 0i32);
    let rp_zone = tt_compute_rp_displacement(gs, &mut dx as *mut i32 as usize, &mut dy as *mut i32 as usize, &mut rp as *mut i32 as usize);
    let (fx, fy) = gs.free();
    let sx = if fx != 0 { dx } else { 0 };
    let sy = if fy != 0 { dy } else { 0 };
    let c = gs.pop();
    let zone = gs.zone(2);
    if c >= zone.contour_count() as i32 {
        return;
    }
    let start = at_i16(zone.contour_starts(), c);
    let last = at_i16(zone.contour_ends(), c).wrapping_sub(start);
    if last < 0 {
        return;
    }
    let mut p = start as i32;
    for _ in 0..=last {
        if p != rp || rp_zone != zone {
            if sx != 0 {
                zone.shift_and_touch(zone.x(), p, sx, 1);
            }
            if sy != 0 {
                zone.shift_and_touch(zone.y(), p, sy, 2);
            }
        }
        p += 1;
    }
}

/// Opcodes 0x36 and 0x37, SHZ: pops a zone and shifts all its points (from the first contour's start to the last
/// contour's end) by the reference point's displacement, skipping the reference point when it lies in that zone.
/// Defect of the original: the points are not marked touched along x; along y, the points after the reference point
/// get the y bit (and the x bit when x moved too), those before it only the x bit when x moved.
/// Original: `tt_op_shz` (0x939360, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_shz(gs: TtLocalState) {
    let (mut dx, mut dy, mut rp) = (0i32, 0i32, 0i32);
    let rp_zone = tt_compute_rp_displacement(gs, &mut dx as *mut i32 as usize, &mut dy as *mut i32 as usize, &mut rp as *mut i32 as usize);
    let n = gs.pop();
    let zone = TtZone(gs.zones().wrapping_add((n as usize).wrapping_mul(TtZone::SIZE)));
    let last = at_i16(zone.contour_ends(), zone.contour_count() as i32 - 1) as i32;
    let first = read_i16(zone.contour_starts()) as i32;
    let skip = if rp_zone == zone { rp } else { -1 };
    let mut moved_x = 0u8;
    if gs.free().0 != 0 {
        shz_shift_runs(zone.x(), None, first, skip, last, dx, (0, 0));
        moved_x = 1;
    }
    if gs.free().1 != 0 {
        shz_shift_runs(zone.y(), Some(zone.flags()), first, skip, last, dy, (moved_x, moved_x | 2));
    }
}

/// SHZ's two runs over one axis: from `first` up to the skipped point (excluded), then from the point after it up to
/// `last` (from `first` itself when nothing is skipped, `skip` = −1); the counts are cut to 16 bits, and a negative
/// count skips its run. With `flags`, each point of the first run gets `marks.0` and of the second `marks.1`.
unsafe fn shz_shift_runs(coords: usize, flags: Option<usize>, first: i32, skip: i32, last: i32, d: i32, marks: (u8, u8)) {
    let shift = |k: i32, mark: u8| {
        set_i32(coords, k, at_i32(coords, k).wrapping_add(d));
        if let Some(f) = flags {
            let a = f.wrapping_add(k as usize);
            write_u8(a, read_u8(a) | mark);
        }
    };
    let mut k = first;
    let before = skip.wrapping_sub(first).wrapping_sub(1) as i16;
    if before >= 0 {
        for _ in 0..=before {
            shift(k, marks.0);
            k = k.wrapping_add(1);
        }
    }
    let after = if skip == -1 {
        last.wrapping_sub(first) as i16
    } else {
        k = k.wrapping_add(1);
        last.wrapping_sub(skip).wrapping_sub(1) as i16
    };
    if after >= 0 {
        for _ in 0..=after {
            shift(k, marks.1);
            k = k.wrapping_add(1);
        }
    }
}

/// Opcode 0x38, SHPIX: pops a distance and loop-count points of zone 2, and shifts each point by the distance along
/// the freedom vector, marking it touched on each axis the vector moves; the loop resets to 1.
/// Original: `tt_op_shpix` (0x9394a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_shpix(gs: TtLocalState) {
    let d = gs.pop();
    let (fx, fy) = gs.free();
    let sx = if fx != 0 { lib_f2dot14_mul(d, fx) } else { 0 };
    let sy = if fy != 0 { lib_f2dot14_mul(d, fy) } else { 0 };
    let zone = gs.zone(2);
    let count = gs.loop_counter();
    if count >= 0 {
        for _ in 0..=count {
            let p = gs.pop();
            if gs.free().0 != 0 {
                zone.shift_and_touch(zone.x(), p, sx, 1);
            }
            if gs.free().1 != 0 {
                zone.shift_and_touch(zone.y(), p, sy, 2);
            }
        }
    }
    gs.set_loop(0);
}

/// Opcode 0x39, IP: pops loop-count points of zone 2 and keeps each, along the projection vector, in the same
/// proportion between reference points 1 (zone 0) and 2 (zone 1) as in the original outline: its original distance
/// from reference point 1 (dual projection) × (the current span ÷ the original span, or 1.0 when the original span is
/// 0), moved to by the point mover; the loop resets to 1.
/// Original: `tt_op_ip` (0x939540, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_ip(gs: TtLocalState) {
    let (z0, z1, z2) = (gs.zone(0), gs.zone(1), gs.zone(2));
    let (rp1, rp2) = (gs.reference_point(1), gs.reference_point(2));
    let span = tt_project_between(gs, z1, rp2, z0, rp1);
    let original_span = tt_dual_project_between(gs, z1, rp2, z0, rp1);
    let ratio = if original_span != 0 { lib_fixed_div(span, original_span) } else { 0x10000 };
    let count = gs.loop_counter();
    if count >= 0 {
        for _ in 0..=count {
            let p = gs.pop();
            let target = lib_fixed_mul(tt_dual_project_between(gs, z2, p, z0, rp1), ratio);
            let now = tt_project_between(gs, z2, p, z0, rp1);
            gs.move_point(z2, p, target.wrapping_sub(now));
        }
    }
    gs.set_loop(0);
}

/// Opcodes 0x3A and 0x3B, MSIRP: pops a distance and a point of zone 1 and moves the point to that distance from
/// reference point 0 (zone 0) along the projection vector; in the twilight zone the point is first put there in the
/// original outline too (and the current position copied from it). Reference point 1 becomes reference point 0, 2
/// the point, and with bit 0 (MSIRP[a]) reference point 0 the point too.
/// Original: `tt_op_msirp` (0x939670, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_msirp(gs: TtLocalState) {
    let d = gs.pop();
    let p = gs.pop();
    let (z0, z1) = (gs.zone(0), gs.zone(1));
    let rp0 = gs.reference_point(0);
    if z1.0 == gs.zones() {
        set_i32(z1.ox(), p, lib_f2dot14_mul(d, gs.proj().0).wrapping_add(at_i32(z0.ox(), rp0)));
        set_i32(z1.oy(), p, lib_f2dot14_mul(d, gs.proj().1).wrapping_add(at_i32(z0.oy(), rp0)));
        set_i32(z1.x(), p, at_i32(z1.ox(), p));
        set_i32(z1.y(), p, at_i32(z1.oy(), p));
    }
    let now = tt_project_between(gs, z1, p, z0, rp0);
    gs.move_point(z1, p, d.wrapping_sub(now));
    gs.set_reference_point(1, rp0);
    gs.set_reference_point(2, p);
    if gs.opcode() & 1 != 0 {
        gs.set_reference_point(0, p);
    }
}

/// Comparers: only the original (they change the interpreter's state; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_op_roll(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I0%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_mdap(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_miap(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_iup(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_shp(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I4%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_compute_rp_displacement(gs: TtLocalState, dx: usize, dy: usize, rp: usize) -> TtZone {
    original_fn::<unsafe extern "C" fn(TtLocalState, usize, usize, usize) -> TtZone>(%%I5%%)(gs, dx, dy, rp)
}

pub(crate) unsafe extern "C" fn compare_tt_op_shc(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_shz(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I7%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_shpix(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I8%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_ip(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I9%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_msirp(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I10%%)(gs)
}
