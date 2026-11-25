// ---------- rx01_3: the T2K TrueType interpreter: the vector opcodes and the point mover along the freedom vector (library_without_asserts, T2K fnt) ----------

/// The point movers and the projectors the vector opcodes install (their exe addresses: the state calls them through
/// its function fields).
pub(crate) const TT_MOVE_X: usize = 0x937260; // tt_move_point_and_touch
pub(crate) const TT_MOVE_Y: usize = 0x937590; // tt_direct_move_y
pub(crate) const TT_MOVE_FREE: usize = 0x937e50; // tt_direct_move
pub(crate) const TT_PROJECT_X: usize = 0x937290; // tt_project_x
pub(crate) const TT_PROJECT_Y: usize = 0x9375c0; // tt_project_y
pub(crate) const TT_PROJECT: usize = 0x9380c0; // tt_project
pub(crate) const TT_DUAL_PROJECT: usize = 0x938250; // tt_dual_project
/// 1.0 in 2.14.
pub(crate) const TT_ONE: i16 = 0x4000;

/// Stores the dot product of the projection and freedom vectors; a magnitude under 1/16 (0x400) becomes ±1.0 with
/// the sign kept, so the moves never divide by almost nothing.
pub(crate) unsafe fn tt_set_proj_dot_free(gs: TtLocalState, v: i16) {
    let v = if v > -0x400 && v < 0x400 { if v >= 0 { TT_ONE } else { -TT_ONE } } else { v };
    gs.set_proj_dot_free(v);
}

/// The dot product of the projection and freedom vectors as the opcodes compute it: each product rounded to 2.14,
/// the sum cut to 16 bits.
pub(crate) unsafe fn tt_dot_proj_free(gs: TtLocalState) -> i16 {
    let ((px, py), (fx, fy)) = (gs.proj(), gs.free());
    let term = |a: i16, b: i16| (a as i32 * b as i32 + 0x2000) >> 14;
    term(fy, py).wrapping_add(term(fx, px)) as i16
}

/// Both vectors on one axis with the matching mover and projectors (SVTCA).
unsafe fn set_both_vectors_to_axis(gs: TtLocalState, x_axis: bool) {
    let (x, y) = if x_axis { (TT_ONE, 0) } else { (0, TT_ONE) };
    gs.set_proj(x, y);
    gs.set_free(x, y);
    gs.set_move_point(if x_axis { TT_MOVE_X } else { TT_MOVE_Y });
    let project = if x_axis { TT_PROJECT_X } else { TT_PROJECT_Y };
    gs.set_project(project);
    gs.set_dual_project(project);
    gs.set_proj_dot_free(TT_ONE);
}

/// Opcode 0x00, SVTCA[y]: the projection and freedom vectors along y, with the y mover and projectors.
/// Original: `tt_op_svtca_y` (0x937550, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_svtca_y(gs: TtLocalState) {
    set_both_vectors_to_axis(gs, false);
}

/// Opcode 0x01, SVTCA[x]: the projection and freedom vectors along x, with the x mover and projectors.
/// Original: `tt_op_svtca_x` (0x937d90, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_svtca_x(gs: TtLocalState) {
    set_both_vectors_to_axis(gs, true);
}

/// Opcodes 0x02 and 0x03, SPVTCA: the projection vector along y (0x02) or x (0x03) with the axis projector; the dot
/// product becomes the freedom vector's matching part (`tt_set_proj_dot_free`), the dual projector the projector and
/// the mover the general one.
/// Original: `tt_op_spvtca` (0x937dd0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_spvtca(gs: TtLocalState) {
    let (fx, fy) = gs.free();
    let (project, dot) = if gs.opcode() & 1 != 0 {
        gs.set_proj(TT_ONE, 0);
        (TT_PROJECT_X, fx)
    } else {
        gs.set_proj(0, TT_ONE);
        (TT_PROJECT_Y, fy)
    };
    gs.set_project(project);
    tt_set_proj_dot_free(gs, dot);
    gs.set_move_point(TT_MOVE_FREE);
    gs.set_dual_project(project);
}

/// Moves point `p` of `zone` by the distance `d` (measured along the projection vector) along the freedom vector:
/// each axis with a non-zero freedom part moves by `d × part` (the part divided by the dot product first when that is
/// not 1.0) and is marked touched.
/// Original: `tt_direct_move` (0x937e50, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_direct_move(gs: TtLocalState, zone: TtZone, p: i32, d: i32) {
    let dot = gs.proj_dot_free();
    let (fx, fy) = gs.free();
    let along = |part: i16| if dot == TT_ONE { lib_f2dot14_mul(d, part) } else { lib_fixed_mul(d, lib_fixed_div(part as i32, dot as i32)) };
    if fx != 0 {
        zone.shift_and_touch(zone.x(), p, along(fx), 1);
    }
    if fy != 0 {
        zone.shift_and_touch(zone.y(), p, along(fy), 2);
    }
}

/// Opcodes 0x04 and 0x05, SFVTCA: the freedom vector along y (0x04) or x (0x05); the dot product becomes the
/// projection vector's matching part and the mover the general one.
/// Original: `tt_op_sfvtca` (0x937f60, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sfvtca(gs: TtLocalState) {
    let (px, py) = gs.proj();
    let dot = if gs.opcode() & 1 != 0 {
        gs.set_free(TT_ONE, 0);
        px
    } else {
        gs.set_free(0, TT_ONE);
        py
    };
    tt_set_proj_dot_free(gs, dot);
    gs.set_move_point(TT_MOVE_FREE);
}

/// The unit vector (2.14) from point `to` of `zone_to` to point `from` of `zone_from`, read from the coordinate
/// arrays at +`coords` (+0 current, +8 original): the differences ×1024, divided by their length.
unsafe fn unit_vector_between(zone_from: TtZone, from: i32, zone_to: TtZone, to: i32, coords: usize) -> (i16, i16) {
    let at = |z: TtZone, axis: usize, p: i32| read_i32(read_usize(z.0 + coords + 4 * axis).wrapping_add((p as usize).wrapping_mul(4)));
    let dx = at(zone_from, 0, from).wrapping_sub(at(zone_to, 0, to)) << 10;
    let dy = at(zone_from, 1, from).wrapping_sub(at(zone_to, 1, to)) << 10;
    let length = lib_fixed_hypot(dx, dy);
    ((lib_fixed_div(dx, length) >> 2) as i16, (lib_fixed_div(dy, length) >> 2) as i16)
}

/// Opcodes 0x06 and 0x07, SPVTL: pops two points and sets the projection vector along the line from the top one
/// (zone 2) to the deeper one (zone 1), turned 90° counter-clockwise for 0x07; then the dot product, the general mover
/// and the general projector as the dual projector too.
/// Original: `tt_op_spvtl` (0x937fd0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_spvtl(gs: TtLocalState) {
    let top = gs.pop();
    let deeper = gs.pop();
    let (x, y) = unit_vector_between(gs.zone(1), deeper, gs.zone(2), top, 0);
    if gs.opcode() & 1 != 0 {
        gs.set_proj(y.wrapping_neg(), x);
    } else {
        gs.set_proj(x, y);
    }
    tt_set_proj_dot_free(gs, tt_dot_proj_free(gs));
    gs.set_move_point(TT_MOVE_FREE);
    gs.set_project(TT_PROJECT);
    gs.set_dual_project(TT_PROJECT);
}

/// Opcodes 0x86 and 0x87, SDPVTL: like SPVTL for the projection vector (current coordinates), and the dual projection
/// vector the same way from the original coordinates; both turned 90° for 0x87; the dot product, the general mover,
/// the projector and the dual projector.
/// Original: `tt_op_sdpvtl` (0x9380f0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sdpvtl(gs: TtLocalState) {
    let top = gs.pop();
    let deeper = gs.pop();
    let (x, y) = unit_vector_between(gs.zone(1), deeper, gs.zone(2), top, 0);
    gs.set_proj(x, y);
    let (dx, dy) = unit_vector_between(gs.zone(1), deeper, gs.zone(2), top, 8);
    gs.set_dual(dx, dy);
    if gs.opcode() & 1 != 0 {
        gs.set_proj(y.wrapping_neg(), x);
        gs.set_dual(dy.wrapping_neg(), dx);
    }
    tt_set_proj_dot_free(gs, tt_dot_proj_free(gs));
    gs.set_move_point(TT_MOVE_FREE);
    gs.set_project(TT_PROJECT);
    gs.set_dual_project(TT_DUAL_PROJECT);
}

/// Opcodes 0x08 and 0x09, SFVTL: pops two points and sets the freedom vector along their line (as SPVTL), turned 90°
/// for 0x09; the dot product and the general mover.
/// Original: `tt_op_sfvtl` (0x938280, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sfvtl(gs: TtLocalState) {
    let top = gs.pop();
    let deeper = gs.pop();
    let (x, y) = unit_vector_between(gs.zone(1), deeper, gs.zone(2), top, 0);
    if gs.opcode() & 1 != 0 {
        gs.set_free(y.wrapping_neg(), x);
    } else {
        gs.set_free(x, y);
    }
    tt_set_proj_dot_free(gs, tt_dot_proj_free(gs));
    gs.set_move_point(TT_MOVE_FREE);
}

/// Opcode 0x0A, SPVFS: pops y, then x (16 bits each) as the projection vector; the dot product, the general mover
/// and the general projector as the dual projector too.
/// Original: `tt_op_spvfs` (0x938360, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_spvfs(gs: TtLocalState) {
    let y = gs.pop() as i16;
    let x = gs.pop() as i16;
    gs.set_proj(x, y);
    tt_set_proj_dot_free(gs, tt_dot_proj_free(gs));
    gs.set_move_point(TT_MOVE_FREE);
    gs.set_project(TT_PROJECT);
    gs.set_dual_project(TT_PROJECT);
}

/// Opcode 0x0B, SFVFS: pops y, then x (16 bits each) as the freedom vector; the dot product and the general mover.
/// Original: `tt_op_sfvfs` (0x9383f0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sfvfs(gs: TtLocalState) {
    let y = gs.pop() as i16;
    let x = gs.pop() as i16;
    gs.set_free(x, y);
    tt_set_proj_dot_free(gs, tt_dot_proj_free(gs));
    gs.set_move_point(TT_MOVE_FREE);
}

/// Opcode 0x0C, GPV: pushes the projection vector's x, then its y.
/// Original: `tt_op_gpv` (0x938470, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_gpv(gs: TtLocalState) {
    let (x, y) = gs.proj();
    gs.push(x as i32);
    gs.push(y as i32);
}

/// Opcode 0x0D, GFV: pushes the freedom vector's x, then its y.
/// Original: `tt_op_gfv` (0x9384a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_gfv(gs: TtLocalState) {
    let (x, y) = gs.free();
    gs.push(x as i32);
    gs.push(y as i32);
}

/// Comparers: only the original (they change the interpreter's state; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_op_svtca_y(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I0%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_svtca_x(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_spvtca(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_direct_move(gs: TtLocalState, zone: TtZone, p: i32, d: i32) {
    original_fn::<TtMovePointFn>(%%I3%%)(gs, zone, p, d)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sfvtca(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I4%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_spvtl(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I5%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sdpvtl(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sfvtl(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I7%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_spvfs(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I8%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sfvfs(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I9%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_gpv(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I10%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_gfv(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I11%%)(gs)
}
