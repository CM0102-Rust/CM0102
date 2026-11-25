// ---------- rx02_4: the T2K TrueType interpreter: the word pushes, the storage, the control value table and the measuring opcodes (library_without_asserts, T2K fnt) ----------

/// Opcodes 0xB8-0xBF, PUSHW[n]: pushes the n + 1 big-endian words after the opcode, sign-extended. The font program
/// of the game's symbol.ttf uses PUSHW[0].
/// Original: `tt_op_pushw` (0x939d90, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_pushw(gs: TtLocalState) {
    let count = (gs.opcode() as u16).wrapping_sub(0xb8) as i16;
    tt_push_words(gs, count);
}

/// Opcode 0x41, NPUSHW: reads a count byte and pushes that many following big-endian words, sign-extended.
/// Original: `tt_op_npushw` (0x939de0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_npushw(gs: TtLocalState) {
    let ip = gs.ins_ptr();
    let n = read_u8(ip);
    gs.set_ins_ptr(ip + 1);
    tt_push_words(gs, (n as u16).wrapping_sub(1) as i16);
}

/// Pushes `last + 1` big-endian signed words from the instruction stream, none when `last` is negative.
unsafe fn tt_push_words(gs: TtLocalState, last: i16) {
    let mut ip = gs.ins_ptr();
    if last >= 0 {
        for _ in 0..=last {
            gs.push(i16::from_be_bytes([read_u8(ip), read_u8(ip + 1)]) as i32);
            ip += 2;
        }
    }
    gs.set_ins_ptr(ip);
}

/// Opcode 0x42, WS: pops a value and a storage index and writes the value there.
/// Original: `tt_op_ws` (0x939e30, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_ws(gs: TtLocalState) {
    let v = gs.pop();
    let k = gs.pop();
    set_i32(gs.global().storage(), k, v);
}

/// Opcode 0x43, RS: replaces the storage index on top of the stack with the value stored there.
/// Original: `tt_op_rs` (0x939e60, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_rs(gs: TtLocalState) {
    let k = gs.pop();
    gs.push(at_i32(gs.global().storage(), k));
}

/// Opcode 0x44, WCVTP: pops a value (in pixels) and a control value index and writes the value; when it is not 0 and
/// the control value reader then gives a different non-zero value r (the stretched reading), the entry becomes
/// `v × (v ÷ r)` (16.16) so that it reads back as v.
/// Original: `tt_op_wcvtp` (0x939e90, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_wcvtp(gs: TtLocalState) {
    let v = gs.pop();
    let k = gs.pop();
    set_i32(gs.global().cvt(), k, v);
    if v != 0 {
        let r = gs.read_cvt(k);
        if r != 0 && r != v {
            set_i32(gs.global().cvt(), k, lib_fixed_mul(v, lib_fixed_div(v, r)));
        }
    }
}

/// Opcode 0x70, WCVTF: pops a value in font units and a control value index and writes the value scaled to pixels
/// (the global state's scaler on the whole 32-bit value).
/// Original: `tt_op_wcvtf` (0x939ef0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_wcvtf(gs: TtLocalState) {
    let v = gs.pop();
    let k = gs.pop();
    let g = gs.global();
    let scaled = g.scale_distance(v);
    set_i32(g.cvt(), k, scaled);
}

/// Opcode 0x45, RCVT: replaces the control value index on top of the stack with the value the control value reader
/// gives.
/// Original: `tt_op_rcvt` (0x939f20, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_rcvt(gs: TtLocalState) {
    let k = gs.pop();
    let v = gs.read_cvt(k);
    gs.push(v);
}

/// Opcodes 0x46 and 0x47, GC: replaces a point of zone 2 on top of the stack with its coordinate along the projection
/// vector: current (0x46, projector) or original (0x47, dual projector).
/// Original: `tt_op_gc` (0x939f50, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_gc(gs: TtLocalState) {
    let p = gs.pop();
    let zone = gs.zone(2);
    let v = if gs.opcode() & 1 != 0 {
        let (x, y) = tt_original(zone, p);
        gs.dual_project(x, y)
    } else {
        let (x, y) = tt_current(zone, p);
        gs.project(x, y)
    };
    gs.push(v);
}

/// Opcode 0x48, SCFS: pops a coordinate and a point of zone 2 and moves the point along the freedom vector until its
/// coordinate along the projection vector is that value; in the twilight zone the original position follows.
/// Original: `tt_op_scfs` (0x939fc0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_scfs(gs: TtLocalState) {
    let c = gs.pop();
    let p = gs.pop();
    let zone = gs.zone(2);
    let (x, y) = tt_current(zone, p);
    let d = gs.project(x, y);
    gs.move_point(zone, p, c.wrapping_sub(d));
    if zone.0 == gs.zones() {
        set_i32(zone.ox(), p, at_i32(zone.x(), p));
        set_i32(zone.oy(), p, at_i32(zone.y(), p));
    }
}

/// Opcodes 0x49 and 0x4A, MD: pops two points (the top one in zone 1, the other in zone 0) and pushes the distance
/// from the first to the second along the projection vector: 0x49 the current positions, 0x4A the original positions
/// in font units along the dual projection vector, scaled to pixels.
/// Original: `tt_op_md` (0x93a020, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_md(gs: TtLocalState) {
    let g = gs.global();
    let a = gs.pop();
    let b = gs.pop();
    let (z0, z1) = (gs.zone(0), gs.zone(1));
    let d = if gs.opcode().wrapping_sub(1) & 1 != 0 {
        let ((bx, by), (ax, ay)) = (tt_original_units(z0, b), tt_original_units(z1, a));
        g.scale_distance(gs.dual_project(bx.wrapping_sub(ax), by.wrapping_sub(ay)))
    } else {
        tt_project_between(gs, z0, b, z1, a)
    };
    gs.push(d);
}

/// Opcode 0x4B, MPPEM: pushes the pixels per em; with non-square pixels it is stretched along the projection vector
/// (`fixed_mul` by the stretch), cut to 16 bits.
/// Original: `tt_op_mppem` (0x93a0d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_mppem(gs: TtLocalState) {
    let g = gs.global();
    let ppem = g.ppem() as i32;
    let v = if g.square_pixels() { ppem } else { lib_fixed_mul(ppem, projection_stretch(gs)) };
    gs.push(v & 0xffff);
}

/// Opcode 0x4C, MPS: pushes the point size.
/// Original: `tt_op_mps` (0x93a190, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_mps(gs: TtLocalState) {
    gs.push(gs.global().point_size() as i32);
}

/// Comparers: only the original (they change the interpreter's state; proved on the emulator). PUSHW runs in the game
/// (the font program of symbol.ttf).
pub(crate) unsafe extern "C" fn compare_tt_op_pushw(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I0%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_npushw(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_ws(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_rs(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_wcvtp(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I4%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_wcvtf(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I5%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_rcvt(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_gc(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I7%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_scfs(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I8%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_md(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I9%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_mppem(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I10%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_mps(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I11%%)(gs)
}
