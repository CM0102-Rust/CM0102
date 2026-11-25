// ---------- rx02_3: the T2K TrueType interpreter: ALIGNRP, ALIGNPTS, the outline flag opcodes, the scan and instruction control, AA and the byte pushes (library_without_asserts, T2K fnt) ----------

/// Opcode 0x3C, ALIGNRP: pops loop-count points of zone 1 and moves each onto reference point 0 (zone 0) along the
/// projection vector (reference point 0's position read once, before the moves); the loop resets to 1.
/// Original: `tt_op_alignrp` (0x939750, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_alignrp(gs: TtLocalState) {
    let z1 = gs.zone(1);
    let (x0, y0) = tt_current(gs.zone(0), gs.reference_point(0));
    if gs.loop_counter() >= 0 {
        loop {
            let p = gs.pop();
            let (x, y) = tt_current(z1, p);
            let d = gs.project(x.wrapping_sub(x0), y.wrapping_sub(y0));
            gs.move_point(z1, p, d.wrapping_neg());
            gs.set_loop(gs.loop_counter().wrapping_sub(1));
            if gs.loop_counter() < 0 {
                break;
            }
        }
    }
    gs.set_loop(0);
}

/// Opcode 0x27, ALIGNPTS: pops two points and moves both by half their distance along the projection vector so they
/// meet. Defect of the original: the distance is measured with the first popped point in zone 0 and the second in
/// zone 1, but the second is then moved in zone 0 and the first in zone 1 (the same points only when both zone
/// pointers are equal).
/// Original: `tt_op_alignpts` (0x9397d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_alignpts(gs: TtLocalState) {
    let p1 = gs.pop();
    let p2 = gs.pop();
    let d = tt_project_between(gs, gs.zone(0), p1, gs.zone(1), p2);
    let half = d >> 1;
    gs.move_point(gs.zone(0), p2, half);
    gs.move_point(gs.zone(1), p1, half.wrapping_sub(d));
}

/// Opcode 0x7E, SANGW: pops the angle weight (16 bits).
/// Original: `tt_op_sangw` (0x939840, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sangw(gs: TtLocalState) {
    let v = gs.pop();
    gs.global().set_angle_weight(v as i16);
}

/// Opcode 0x80, FLIPPT: pops loop-count points of zone 0 and flips each one's on-curve bit; the loop resets to 1.
/// Original: `tt_op_flippt` (0x939860, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_flippt(gs: TtLocalState) {
    let on_curve = gs.zone(0).on_curve();
    let count = gs.loop_counter();
    if count >= 0 {
        for _ in 0..=count {
            let a = on_curve.wrapping_add(gs.pop() as usize);
            write_u8(a, read_u8(a) ^ 1);
        }
    }
    gs.set_loop(0);
}

/// FLIPRGON and FLIPRGOFF: pops the last and the first point of a range of zone 0 and applies `flip` to the outline
/// flags of each point in it (none when the range, cut to 16 bits, is negative).
unsafe fn tt_flip_range(gs: TtLocalState, flip: impl Fn(u8) -> u8) {
    let on_curve = gs.zone(0).on_curve();
    let last = gs.pop();
    let first = gs.pop();
    let count = last.wrapping_sub(first) as i16;
    if count >= 0 {
        for k in 0..=count as i32 {
            let a = on_curve.wrapping_add(first.wrapping_add(k) as usize);
            write_u8(a, flip(read_u8(a)));
        }
    }
}

/// Opcode 0x81, FLIPRGON: pops a range of points of zone 0 and puts them on the curve.
/// Original: `tt_op_fliprgon` (0x9398a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_fliprgon(gs: TtLocalState) {
    tt_flip_range(gs, |f| f | 1);
}

/// Opcode 0x82, FLIPRGOFF: pops a range of points of zone 0 and takes them off the curve.
/// Original: `tt_op_fliprgoff` (0x9398e0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_fliprgoff(gs: TtLocalState) {
    tt_flip_range(gs, |f| f & 0xfe);
}

/// Opcode 0x85, SCANCTRL: pops a value into the low half of the scan control word (OR'ed in whole: a value above
/// 0xffff also sets bits of the high half, as in the original).
/// Original: `tt_op_scanctrl` (0x939920, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_scanctrl(gs: TtLocalState) {
    let v = gs.pop();
    let g = gs.global();
    g.set_scan_control(g.scan_control() & 0xffff0000 | v as u32);
}

/// Opcode 0x8D, SCANTYPE: pops the scan type 0, 1 or 2 into the high half of the scan control word; other values
/// change nothing.
/// Original: `tt_op_scantype` (0x939950, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_scantype(gs: TtLocalState) {
    let v = gs.pop();
    let g = gs.global();
    let low = g.scan_control() & 0xffff;
    match v {
        0 => g.set_scan_control(low),
        1 => g.set_scan_control(low | 0x10000),
        2 => g.set_scan_control(low | 0x20000),
        _ => {}
    }
}

/// Opcode 0x8E, INSTCTRL: pops a selector and a value; only while the control value program runs, selector 1 or 2
/// sets instruction control bit 0 or 1 from the same bit of the value.
/// Original: `tt_op_instctrl` (0x9399a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_instctrl(gs: TtLocalState) {
    let selector = gs.pop();
    let v = gs.pop() as u32;
    let g = gs.global();
    if g.instruct_control_allowed() && (selector == 1 || selector == 2) {
        let flags = g.instruct_control();
        g.set_instruct_control(flags ^ (flags ^ v) & selector as u32);
    }
}

/// Opcode 0x7F, AA (T2K's version): pops a point and moves it, along the freedom vector, by its distance to reference
/// point 0 measured along the best of the 20 angle table vectors:
/// - the offset from reference point 0 (zone 0) to the point (zone 1) is folded into the first octant (its signs and
///   the swap of its axes kept) and its unit normal taken;
/// - each table vector, in order of its cost (the angle distance table, increasing), is scored by its cost + the
///   angle weight × |the offset along it| / 64, the scan stopping at the first cost not below the best score; a
///   vector further than 0xc9107b7 from the normal (never, at 2.14) is skipped;
/// - the winner, unfolded, becomes the projection vector for the move (with its projection·freedom product), the
///   distance being from the point (zone 0) to reference point 0 (zone 1) along it; without a winner the point is
///   only touched (a move of 0);
/// - the projection vector and the product are restored at the end.
/// Defects of the original: the offset is taken with the point in zone 1 and reference point 0 in zone 0, the
/// distance with the point in zone 0 and reference point 0 in zone 1, and the move is on zone 1 (the same only when
/// both zone pointers are equal); and the angle points are read as 16-bit values at a 4-byte stride, so with the
/// game's table (`tt_init_opcode_table`) the first vector is (0, 0), costs 64 and always wins: AA only touches the
/// point.
/// Original: `tt_op_aa` (0x939a00, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_aa(gs: TtLocalState) {
    let g = gs.global();
    let p = gs.pop();
    let (saved_proj, saved_dot) = (gs.proj(), gs.proj_dot_free());
    let rp0 = gs.reference_point(0);
    let ((px, py), (rx, ry)) = (tt_current(gs.zone(1), p), tt_current(gs.zone(0), rp0));
    let (mut dx, mut dy) = (px.wrapping_sub(rx), py.wrapping_sub(ry));
    let negative_y = dy < 0;
    if negative_y {
        dy = dy.wrapping_neg();
    }
    let negative_x = dx < 0;
    if negative_x {
        dx = dx.wrapping_neg();
    }
    let swapped = dy > dx;
    let (big, small) = if swapped { (dy, dx) } else { (dx, dy) };
    let (a, b) = (small.wrapping_neg() << 10, big << 10);
    let length = lib_fixed_hypot(a, b);
    let normal_y = (lib_fixed_div(a, length) >> 2) as i16 as i32;
    let normal_x = (lib_fixed_div(b, length) >> 2) as i16 as i32;
    let (points, costs) = (g.angle_points(), g.angle_distances());
    let mut best = 0x280;
    let mut winner: i32 = -1;
    for k in 0..20 {
        let cost = at_i16(costs, k) as i32;
        if cost >= best {
            break;
        }
        let (vx, vy) = (read_i16(points + 8 * k as usize), read_i16(points + 8 * k as usize + 4));
        gs.set_proj(vx, vy);
        let ex = (vx as i32).wrapping_sub(normal_y).wrapping_abs();
        let ey = (vy as i32).wrapping_sub(normal_x).wrapping_abs();
        let off = if ex > ey { (ey >> 1).wrapping_add(ex) } else { (ex >> 1).wrapping_add(ey) };
        if off > 0xc9107b7 {
            continue;
        }
        let along = lib_f2dot14_mul(small, vy).wrapping_add(lib_f2dot14_mul(big, vx)).wrapping_abs();
        let score = ((g.angle_weight() as i32).wrapping_mul(along) >> 6).wrapping_add(cost);
        if score < best {
            best = score;
            winner = k;
        }
    }
    let mut d = 0;
    if winner >= 0 {
        let entry = g.angle_points() + 8 * winner as usize;
        let (mut vx, mut vy) = (read_i16(entry), read_i16(entry + 4));
        if swapped {
            core::mem::swap(&mut vx, &mut vy);
        }
        if negative_x {
            vx = vx.wrapping_neg();
        }
        if negative_y {
            vy = vy.wrapping_neg();
        }
        gs.set_proj(vx, vy);
        tt_set_proj_dot_free(gs, tt_dot_proj_free(gs));
        let ((qx, qy), (sx, sy)) = (tt_current(gs.zone(1), rp0), tt_current(gs.zone(0), p));
        let (vx, vy) = gs.proj();
        d = lib_f2dot14_mul(qx.wrapping_sub(sx), vx).wrapping_add(lib_f2dot14_mul(qy.wrapping_sub(sy), vy));
    }
    tt_direct_move(gs, gs.zone(1), p, d);
    gs.set_proj(saved_proj.0, saved_proj.1);
    gs.set_proj_dot_free(saved_dot);
}

/// Opcodes 0xB0-0xB7, PUSHB[n]: pushes the n + 1 bytes after the opcode.
/// Original: `tt_op_pushb` (0x939d10, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_pushb(gs: TtLocalState) {
    let count = (gs.opcode() as u16).wrapping_sub(0xb0) as i16;
    tt_push_bytes(gs, count);
}

/// Opcode 0x40, NPUSHB: reads a count byte and pushes that many following bytes.
/// Original: `tt_op_npushb` (0x939d50, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_npushb(gs: TtLocalState) {
    let ip = gs.ins_ptr();
    let n = read_u8(ip);
    gs.set_ins_ptr(ip + 1);
    tt_push_bytes(gs, (n as u16).wrapping_sub(1) as i16);
}

/// Pushes `last + 1` bytes from the instruction stream (unsigned), none when `last` is negative.
unsafe fn tt_push_bytes(gs: TtLocalState, last: i16) {
    let mut ip = gs.ins_ptr();
    if last >= 0 {
        for _ in 0..=last {
            gs.push(read_u8(ip) as i32);
            ip += 1;
        }
    }
    gs.set_ins_ptr(ip);
}

/// Comparers: only the original (they change the interpreter's state; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_op_alignrp(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I0%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_alignpts(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sangw(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_flippt(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_fliprgon(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I4%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_fliprgoff(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I5%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_scanctrl(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_scantype(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I7%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_instctrl(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I8%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_aa(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I9%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_pushb(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I10%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_npushb(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I11%%)(gs)
}
