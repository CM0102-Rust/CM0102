// ---------- rx01_4: the T2K TrueType interpreter: SFVTPV, ISECT and the graphics state setters (library_without_asserts, T2K fnt) ----------

/// Opcode 0x0E, SFVTPV: the freedom vector becomes the projection vector; the dot product 1.0 and the general mover.
/// Original: `tt_op_sfvtpv` (0x9384d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sfvtpv(gs: TtLocalState) {
    let (x, y) = gs.proj();
    gs.set_proj_dot_free(TT_ONE);
    gs.set_free(x, y);
    gs.set_move_point(TT_MOVE_FREE);
}

/// Point `p` of `zone`'s current coordinates.
unsafe fn tt_point(zone: TtZone, p: i32) -> (i32, i32) {
    let at = |coords: usize| read_i32(coords.wrapping_add((p as usize).wrapping_mul(4)));
    (at(zone.x()), at(zone.y()))
}

/// Opcode 0x0F, ISECT: pops a1, a0 (zone 0), b1, b0 (zone 1) and the point p (zone 2), marks p touched in both axes
/// and puts it where line a0-a1 crosses line b0-b1:
/// - a horizontal a with a vertical b, or a vertical a with a horizontal b: at (b0.x, a0.y) or (a0.x, b0.y);
/// - otherwise b0 + t × (b1 − b0), t = num ÷ den from the slope of a (the steeper axis divided by the other, `fixed_div`);
///   with |num| < |den| by `fixed_mul(·, num ÷ den)`, else by `fixed_div(·, den ÷ num)` (b0 itself when num is 0);
/// - parallel lines (den 0): the average of a0 + (a1 − a0)/2 and b0 + (b1 − b0)/2 (arithmetic halvings).
/// Original: `tt_op_isect` (0x9384f0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_isect(gs: TtLocalState) {
    let sp = gs.stack_ptr();
    let at = |k: usize| read_i32(sp - 4 * k);
    let (a1, a0, b1, b0, p) = (at(1), at(2), at(3), at(4), at(5));
    let (za, zb, zp) = (gs.zone(0), gs.zone(1), gs.zone(2));
    let ((ax0, ay0), (ax1, ay1)) = (tt_point(za, a0), tt_point(za, a1));
    let ((bx0, by0), (bx1, by1)) = (tt_point(zb, b0), tt_point(zb, b1));
    let (dxa, dya) = (ax1.wrapping_sub(ax0), ay1.wrapping_sub(ay0));
    let (dxb, dyb) = (bx1.wrapping_sub(bx0), by1.wrapping_sub(by0));
    gs.set_stack_ptr(sp - 20);
    let flags = zp.flags().wrapping_add(p as usize);
    write_u8(flags, read_u8(flags) | 3);
    let put = |x: i32, y: i32| {
        let idx = (p as usize).wrapping_mul(4);
        write_i32(zp.x().wrapping_add(idx), x);
        write_i32(zp.y().wrapping_add(idx), y);
    };
    // the numerator and the denominator of t along b
    let (num, den) = if dya == 0 {
        if dxb == 0 {
            return put(bx0, ay0);
        }
        (by0.wrapping_sub(ay0), dyb.wrapping_neg())
    } else if dxa == 0 {
        if dyb == 0 {
            return put(ax0, by0);
        }
        (bx0.wrapping_sub(ax0), dxb.wrapping_neg())
    } else if dxa.wrapping_abs() > dya.wrapping_abs() {
        let slope = lib_fixed_div(dya, dxa);
        (by0.wrapping_sub(ay0).wrapping_sub(lib_fixed_mul(bx0.wrapping_sub(ax0), slope)), lib_fixed_mul(dxb, slope).wrapping_sub(dyb))
    } else {
        let slope = lib_fixed_div(dxa, dya);
        (lib_fixed_mul(by0.wrapping_sub(ay0), slope).wrapping_add(ax0.wrapping_sub(bx0)), dxb.wrapping_sub(lib_fixed_mul(dyb, slope)))
    };
    if den == 0 {
        covered!("rx01_4 isect parallel");
        let mid = |a0: i32, da: i32, b0: i32, db: i32| (da >> 1).wrapping_add(b0.wrapping_add(db >> 1)).wrapping_add(a0) >> 1;
        return put(mid(ax0, dxa, bx0, dxb), mid(ay0, dya, by0, dyb));
    }
    if num.wrapping_abs() < den.wrapping_abs() {
        let t = lib_fixed_div(num, den);
        put(bx0.wrapping_add(lib_fixed_mul(dxb, t)), by0.wrapping_add(lib_fixed_mul(dyb, t)));
    } else if num == 0 {
        put(bx0, by0);
    } else {
        let inverse = lib_fixed_div(den, num);
        put(bx0.wrapping_add(lib_fixed_div(dxb, inverse)), by0.wrapping_add(lib_fixed_div(dyb, inverse)));
    }
}

/// Opcode 0x1A, SMD: pops the minimum distance.
/// Original: `tt_op_smd` (0x938790, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_smd(gs: TtLocalState) {
    let v = gs.pop();
    gs.global().set_min_distance(v);
}

/// Opcode 0x1D, SCVTCI: pops the control value cut-in.
/// Original: `tt_op_scvtci` (0x9387b0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_scvtci(gs: TtLocalState) {
    let v = gs.pop();
    gs.global().set_cvt_cut_in(v);
}

/// Opcode 0x1E, SSWCI: pops the single width cut-in.
/// Original: `tt_op_sswci` (0x9387d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sswci(gs: TtLocalState) {
    let v = gs.pop();
    gs.global().set_single_width_cut_in(v);
}

/// Opcode 0x1F, SSW: pops the single width in font units (16 bits), keeps it and stores it scaled by the global
/// state's scaler.
/// Original: `tt_op_ssw` (0x9387f0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_ssw(gs: TtLocalState) {
    let v = gs.pop() as i16;
    let g = gs.global();
    g.set_single_width_units(v);
    g.set_single_width(g.scale(v));
}

/// Opcodes 0x10-0x12 (SRP0-2), 0x17 (SLOOP) and 0x21 (POP): pops a value into reference point 0, 1 or 2, or the loop
/// counter (value − 1); POP and any other opcode routed here store nothing.
/// Original: `tt_op_srp_sloop_pop` (0x938820, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_srp_sloop_pop(gs: TtLocalState) {
    let v = gs.pop();
    match gs.opcode().wrapping_sub(0x10) {
        k @ 0..=2 => gs.set_reference_point(k as usize, v),
        7 => gs.set_loop(v.wrapping_sub(1) as i16),
        _ => {}
    }
}

/// Comparers: only the original (they change the interpreter's state; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_op_sfvtpv(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I0%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_isect(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_smd(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_scvtci(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sswci(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I4%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_ssw(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I5%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_srp_sloop_pop(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}
