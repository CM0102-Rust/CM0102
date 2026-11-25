// ---------- rx02_6: the T2K TrueType interpreter: NROUND, MDRP, MIRP, the function and instruction definitions, UTP, SDB, SDS and DELTAP1 (library_without_asserts, T2K fnt) ----------

/// Opcodes 0x6C-0x6F, NROUND: leaves the top value as it is (no engine compensation in T2K; the original only stores
/// the stack pointer back unchanged).
/// Original: `tt_op_nround` (0x93a7d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_nround(_gs: TtLocalState) {}

/// The single width test of MDRP and MIRP: with a single width cut-in, a distance whose size is within the cut-in of
/// the single width becomes the single width, with the distance's sign.
unsafe fn tt_apply_single_width(gs: TtLocalState, d: i32) -> i32 {
    if gs.global().single_width_cut_in() == 0 {
        return d;
    }
    let width = gs.read_single_width();
    let cut_in = gs.global().single_width_cut_in();
    let snap = |size: i32| if size.wrapping_sub(width).wrapping_abs() < cut_in { width } else { size };
    if d >= 0 { snap(d) } else { snap(d.wrapping_neg()).wrapping_neg() }
}

/// The minimum distance of MDRP and MIRP: `d` kept at least `min` away from 0 on the side of `sign_of`.
fn tt_keep_min_distance(min: i32, sign_of: i32, d: i32) -> i32 {
    if sign_of >= 0 {
        d.max(min)
    } else {
        d.min(min.wrapping_neg())
    }
}

/// Opcodes 0xC0-0xDF, MDRP[abcde]: pops a point of zone 1 and moves it, along the freedom vector, so that its
/// distance from reference point 0 (zone 0) along the projection vector is their original distance: from the
/// original positions (dual projection) when either zone is the twilight zone, else from the font units scaled to
/// pixels. The distance goes through the single width test; bit 2 rounds it, bit 3 keeps the minimum distance (on
/// the side of the distance before rounding). Reference point 1 becomes reference point 0, 2 the point, and with bit
/// 4 reference point 0 the point too.
/// Original: `tt_op_mdrp` (0x93a7e0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_mdrp(gs: TtLocalState) {
    let g = gs.global();
    let rp0 = gs.reference_point(0);
    let z1 = gs.zone(1);
    let p = gs.pop();
    let z0 = gs.zone(0);
    let zones = gs.zones();
    let mut d = if z0.0 == zones || z1.0 == zones {
        tt_dual_project_between(gs, z1, p, z0, rp0)
    } else {
        let ((px, py), (qx, qy)) = (tt_original_units(z1, p), tt_original_units(z0, rp0));
        g.scale_distance(gs.dual_project(px.wrapping_sub(qx), py.wrapping_sub(qy)))
    };
    d = tt_apply_single_width(gs, d);
    let measured = d;
    if gs.opcode() & 4 != 0 {
        d = tt_round(gs, d);
    }
    if gs.opcode() & 8 != 0 {
        d = tt_keep_min_distance(g.min_distance(), measured, d);
    }
    let now = tt_project_between(gs, z1, p, z0, rp0);
    gs.move_point(z1, p, d.wrapping_sub(now));
    gs.set_reference_point(1, rp0);
    gs.set_reference_point(2, p);
    if gs.opcode() & 0x10 != 0 {
        gs.set_reference_point(0, p);
    }
}

/// The distance from point `q` of `zq` to point `p` of `zp` along `f` (one of the state's projectors), read straight
/// from the coordinates when `f` is the x or the y projector (`coords` +0 current, +8 original), as MIRP does.
unsafe fn tt_mirp_distance(gs: TtLocalState, f: usize, coords: usize, zp: TtZone, p: i32, zq: TtZone, q: i32) -> i32 {
    let at = |z: TtZone, axis: usize, k: i32| at_i32(read_usize(z.0 + coords + 4 * axis), k);
    match f {
        TT_PROJECT_X => at(zp, 0, p).wrapping_sub(at(zq, 0, q)),
        TT_PROJECT_Y => at(zp, 1, p).wrapping_sub(at(zq, 1, q)),
        _ => {
            let project: TtProjectFn = core::mem::transmute(f);
            project(gs, at(zp, 0, p).wrapping_sub(at(zq, 0, q)), at(zp, 1, p).wrapping_sub(at(zq, 1, q)))
        }
    }
}

/// Opcodes 0xE0-0xFF, MIRP[abcde]: pops a control value index and a point of zone 1; reference point 1 becomes
/// reference point 0, 2 the point, and with bit 4 reference point 0 the point too. The control value (through the
/// single width test) is the distance; in the twilight zone the point is first put at it from reference point 0
/// (zone 0) along the projection vector (original and current). With auto flip the control value takes the sign of
/// the original distance; bit 2 replaces it by the original distance when they differ by more than the control value
/// cut-in, and rounds it; bit 3 keeps the minimum distance (on the side of the original distance). The point moves,
/// along the freedom vector, to that distance from reference point 0.
/// Original: `tt_op_mirp` (0x93a960, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_mirp(gs: TtLocalState) {
    let z0 = gs.zone(0);
    let g = gs.global();
    let op = gs.opcode();
    let z1 = gs.zone(1);
    let n = gs.pop();
    let mut v = gs.read_cvt(n);
    v = tt_apply_single_width(gs, v);
    let p = gs.pop();
    let rp0 = gs.reference_point(0);
    gs.set_reference_point(1, rp0);
    gs.set_reference_point(2, p);
    if op & 0x10 != 0 {
        gs.set_reference_point(0, p);
    }
    if z1.0 == gs.zones() {
        set_i32(z1.ox(), p, lib_f2dot14_mul(v, gs.proj().0).wrapping_add(at_i32(z0.ox(), rp0)));
        set_i32(z1.x(), p, at_i32(z1.ox(), p));
        set_i32(z1.oy(), p, lib_f2dot14_mul(v, gs.proj().1).wrapping_add(at_i32(z0.oy(), rp0)));
        set_i32(z1.y(), p, at_i32(z1.oy(), p));
    }
    let original = tt_mirp_distance(gs, gs.dual_project_fn(), 8, z1, p, z0, rp0);
    let now = tt_mirp_distance(gs, gs.project_fn(), 0, z1, p, z0, rp0);
    if g.auto_flip() != 0 && (original ^ v) < 0 {
        v = v.wrapping_neg();
    }
    if op & 4 != 0 {
        if v.wrapping_sub(original).wrapping_abs() > g.cvt_cut_in() {
            v = original;
        }
        v = tt_round(gs, v);
    }
    if op & 8 != 0 {
        v = tt_keep_min_distance(g.min_distance(), original, v);
    }
    gs.move_point(z1, p, v.wrapping_sub(now));
}

/// The instructions of a definition (8 bytes: +0 the start inside its program, +4 the length): from the program's
/// first byte + the start, for the length.
unsafe fn tt_definition_range(g: TtGlobalState, def: usize, program: usize) -> (usize, usize) {
    let start = g.program_base(program).wrapping_add(read_i32(def) as usize);
    (start, start.wrapping_add(read_u16(def + 4) as usize))
}

/// Function definition `n` (+0 start, +4 length, +6 program as u16).
unsafe fn tt_function_def(g: TtGlobalState, n: i32) -> usize {
    g.function_defs().wrapping_add((n as usize).wrapping_mul(8))
}

/// Opcode 0x2B, CALL: pops a function number and runs its instructions.
/// Original: `tt_op_call` (0x93abf0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_call(gs: TtLocalState) {
    let g = gs.global();
    let def = tt_function_def(g, gs.pop());
    let (start, end) = tt_definition_range(g, def, read_u16(def + 6) as usize);
    gs.run_instructions(start, end);
}

/// Steps over a definition's body up to its ENDF (skipping push data) and returns the body's length from `start`
/// (16 bits).
unsafe fn tt_skip_to_endf(gs: TtLocalState, start: usize) -> u16 {
    let mut op = tt_fetch_opcode(gs);
    while op != 0x2d {
        tt_skip_push_data(gs, op);
        op = tt_fetch_opcode(gs);
    }
    (gs.ins_ptr() as u16).wrapping_sub(start as u16).wrapping_sub(1)
}

/// Opcode 0x2C, FDEF: pops a function number and records the definition: the running program, the body's start
/// inside it and its length (up to ENDF, which the instruction pointer passes). The font program of the game's
/// symbol.ttf uses FDEF.
/// Original: `tt_op_fdef` (0x93ac30, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_fdef(gs: TtLocalState) {
    let g = gs.global();
    let def = tt_function_def(g, gs.pop());
    let program = g.current_program();
    write_u16(def + 6, program as u16);
    let start = gs.ins_ptr();
    write_i32(def, start.wrapping_sub(g.program_base(program as usize)) as i32);
    let length = tt_skip_to_endf(gs, start);
    write_u16(def + 4, length);
}

/// Opcode 0x2A, LOOPCALL: pops a function number and a count, and runs the function that many times (the count cut
/// to 16 bits; none when it is not positive).
/// Original: `tt_op_loopcall` (0x93ad80, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_loopcall(gs: TtLocalState) {
    let g = gs.global();
    let def = tt_function_def(g, gs.pop());
    let (start, end) = tt_definition_range(g, def, read_u16(def + 6) as usize);
    let last = (gs.pop() as u16).wrapping_sub(1) as i16;
    let run = gs.interpreter_fn();
    if last >= 0 {
        for _ in 0..=last {
            run(gs, start, end);
        }
    }
}

/// The instruction definition of opcode `op` (+7), searched in order, or 0.
unsafe fn tt_find_instruction_def(g: TtGlobalState, op: u8) -> usize {
    let mut def = g.instruction_defs();
    let mut left = g.instruction_def_count().wrapping_sub(1);
    while left >= 0 {
        if read_u8(def + 7) == op {
            return def;
        }
        def += 8;
        left = left.wrapping_sub(1);
    }
    0
}

/// Opcodes 0x83, 0x84 and 0x8F-0xAF (no TrueType meaning): runs the instruction definition of the opcode (IDEF), or
/// leaves the program with error 0x1101 when there is none.
/// Original: `tt_op_call_user_defined` (0x93adf0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_call_user_defined(gs: TtLocalState) {
    let g = gs.global();
    let def = tt_find_instruction_def(g, gs.opcode() as u8);
    if def == 0 {
        tt_raise(gs, 0x1101);
    }
    let (start, end) = tt_definition_range(g, def, read_u8(def + 6) as usize);
    gs.run_instructions(start, end);
}

/// Opcode 0x89, IDEF: pops an opcode and records its instruction definition (reusing the opcode's entry, else a new
/// one at the end): the running program, the opcode, the body's start inside the program and its length (up to ENDF,
/// which the instruction pointer passes).
/// Original: `tt_op_idef` (0x93ae60, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_idef(gs: TtLocalState) {
    let op = gs.pop() as u8;
    let g = gs.global();
    let mut def = tt_find_instruction_def(g, op);
    let start = gs.ins_ptr();
    let program = g.current_program();
    if def == 0 {
        let count = g.instruction_def_count();
        def = g.instruction_defs().wrapping_add((count as isize as usize).wrapping_mul(8));
        g.set_instruction_def_count(count.wrapping_add(1));
    }
    write_u8(def + 6, program);
    write_u8(def + 7, op);
    write_i32(def, start.wrapping_sub(g.program_base(program as usize)) as i32);
    let length = tt_skip_to_endf(gs, start);
    write_u16(def + 4, length);
}

/// Opcode 0x29, UTP: pops a point of zone 0 and clears its touched bit on each axis the freedom vector moves.
/// Original: `tt_op_utp` (0x93aff0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_utp(gs: TtLocalState) {
    let p = gs.pop();
    let a = gs.zone(0).flags().wrapping_add(p as usize);
    let (fx, fy) = gs.free();
    if fx != 0 {
        write_u8(a, read_u8(a) & 0xfe);
    }
    if fy != 0 {
        write_u8(a, read_u8(a) & 0xfd);
    }
}

/// Opcode 0x5E, SDB: pops the delta base (16 bits).
/// Original: `tt_op_sdb` (0x93b030, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sdb(gs: TtLocalState) {
    let v = gs.pop();
    gs.global().set_delta_base(v as i16);
}

/// Opcode 0x5F, SDS: pops the delta shift (16 bits).
/// Original: `tt_op_sds` (0x93b050, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sds(gs: TtLocalState) {
    let v = gs.pop();
    gs.global().set_delta_shift(v as i16);
}

/// Opcode 0x5D, DELTAP1: the point exceptions for the sizes from the delta base (`tt_delta_scan_apply` with the
/// state's point mover).
/// Original: `tt_op_deltap1` (0x93b070, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_deltap1(gs: TtLocalState) {
    let g = gs.global();
    tt_delta_scan_apply(gs, gs.move_point_fn(), g.delta_base() as i32, g.delta_shift() as i32);
}

/// Comparers: only the original (they change the interpreter's state; proved on the emulator). FDEF runs in the game
/// (the font program of symbol.ttf).
pub(crate) unsafe extern "C" fn compare_tt_op_nround(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I0%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_mdrp(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_mirp(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_call(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_fdef(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I4%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_loopcall(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I5%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_call_user_defined(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_idef(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I7%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_utp(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I8%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sdb(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I9%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sds(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I10%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_deltap1(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I11%%)(gs)
}
