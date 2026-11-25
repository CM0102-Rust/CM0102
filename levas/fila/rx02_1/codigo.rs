// ---------- rx02_1: the T2K TrueType interpreter: more state fields, SZP, the round state opcodes and the stack opcodes (library_without_asserts, T2K fnt) ----------

/// The round functions the round state opcodes install (their exe addresses, rx01_2: the global state calls them
/// through its round function field).
pub(crate) const TT_ROUND_TO_GRID: usize = 0x936f10;
pub(crate) const TT_ROUND_TO_HALF_GRID: usize = 0x936f40;
pub(crate) const TT_ROUND_TO_DOUBLE_GRID: usize = 0x936e80;
pub(crate) const TT_ROUND_DOWN_TO_GRID: usize = 0x936eb0;
pub(crate) const TT_ROUND_UP_TO_GRID: usize = 0x936ee0;
pub(crate) const TT_ROUND_OFF: usize = 0x936f80;
pub(crate) const TT_SUPER_ROUND: usize = 0x936f90;
pub(crate) const TT_SUPER_ROUND_45: usize = 0x936ff0;

#[allow(dead_code)]
impl TtLocalState {
    /// +0x30, +0x34, +0x38 the reference points 0, 1 and 2.
    pub(crate) unsafe fn reference_point(self, k: usize) -> i32 { read_i32(self.0 + 0x30 + 4 * k) }
    /// Points zone pointer `k` (0, 1 or 2) at `zone`.
    pub(crate) unsafe fn set_zone(self, k: usize, zone: usize) { write_usize(self.0 + 8 + 4 * k, zone) }
    /// +0x3e the loop counter (SLOOP value − 1).
    pub(crate) unsafe fn loop_counter(self) -> i16 { read_i16(self.0 + 0x3e) }
    /// +0x42 the point mover: moves point `p` of `zone` by `d` along the freedom vector.
    pub(crate) unsafe fn move_point_fn(self) -> usize { read_usize(self.0 + 0x42) }
    pub(crate) unsafe fn move_point(self, zone: TtZone, p: i32, d: i32) {
        let f: TtMovePointFn = core::mem::transmute(self.move_point_fn());
        f(self, zone, p, d)
    }
    /// +0x46 the projector: (dx, dy) measured along the projection vector.
    pub(crate) unsafe fn project(self, dx: i32, dy: i32) -> i32 {
        let f: TtProjectFn = core::mem::transmute(self.project_fn());
        f(self, dx, dy)
    }
    /// +0x4a the dual projector: (dx, dy) measured along the dual projection vector.
    pub(crate) unsafe fn dual_project_fn(self) -> usize { read_usize(self.0 + 0x4a) }
    pub(crate) unsafe fn dual_project(self, dx: i32, dy: i32) -> i32 {
        let f: TtProjectFn = core::mem::transmute(self.dual_project_fn());
        f(self, dx, dy)
    }
    /// +0x4e the instruction loop: runs the instructions from `start` to `end`.
    pub(crate) unsafe fn interpreter_fn(self) -> TtInterpreterFn { core::mem::transmute(read_usize(self.0 + 0x4e)) }
    pub(crate) unsafe fn run_instructions(self, start: usize, end: usize) { self.interpreter_fn()(self, start, end) }
    /// +0x52 the control value reader: control value `n` in pixels.
    pub(crate) unsafe fn read_cvt(self, n: i32) -> i32 {
        let f: TtReadFn = core::mem::transmute(read_usize(self.0 + 0x52));
        f(self, n)
    }
    /// +0x56 the single width reader.
    pub(crate) unsafe fn read_single_width(self) -> i32 {
        let f: TtValueFn = core::mem::transmute(read_usize(self.0 + 0x56));
        f(self)
    }
}

#[allow(dead_code)]
impl TtZone {
    /// +0x10 the original x and +0x14 the original y in font units (i16 arrays).
    pub(crate) unsafe fn oox(self) -> usize { read_usize(self.0 + 0x10) }
    pub(crate) unsafe fn ooy(self) -> usize { read_usize(self.0 + 0x14) }
    /// +0x18 the per-point outline flags (u8 array; bit 0 on the curve).
    pub(crate) unsafe fn on_curve(self) -> usize { read_usize(self.0 + 0x18) }
    /// +0x1c the number of contours.
    pub(crate) unsafe fn contour_count(self) -> i16 { read_i16(self.0 + 0x1c) }
    /// +0x20 the first and +0x24 the last point of each contour (i16 arrays).
    pub(crate) unsafe fn contour_starts(self) -> usize { read_usize(self.0 + 0x20) }
    pub(crate) unsafe fn contour_ends(self) -> usize { read_usize(self.0 + 0x24) }
}

#[allow(dead_code)]
impl TtGlobalState {
    /// +0 the base of the interpreter stack, +4 the storage area (i32 arrays).
    pub(crate) unsafe fn stack_base(self) -> usize { read_usize(self.0) }
    pub(crate) unsafe fn storage(self) -> usize { read_usize(self.0 + 4) }
    /// +0xc the pixels per em and +0xe the point size of the current size.
    pub(crate) unsafe fn ppem(self) -> u16 { read_u16(self.0 + 0xc) }
    pub(crate) unsafe fn point_size(self) -> u16 { read_u16(self.0 + 0xe) }
    /// +0x58 the control value cut-in, +0x5c the single width cut-in, +0x6c the minimum distance.
    pub(crate) unsafe fn cvt_cut_in(self) -> i32 { read_i32(self.0 + 0x58) }
    pub(crate) unsafe fn single_width_cut_in(self) -> i32 { read_i32(self.0 + 0x5c) }
    pub(crate) unsafe fn min_distance(self) -> i32 { read_i32(self.0 + 0x6c) }
    /// +0x64 the scan control word (SCANCTRL low half, SCANTYPE high half), +0x68 the instruction control flags.
    pub(crate) unsafe fn scan_control(self) -> u32 { read_u32(self.0 + 0x64) }
    pub(crate) unsafe fn set_scan_control(self, v: u32) { write_u32(self.0 + 0x64, v) }
    pub(crate) unsafe fn instruct_control(self) -> u32 { read_u32(self.0 + 0x68) }
    pub(crate) unsafe fn set_instruct_control(self, v: u32) { write_u32(self.0 + 0x68, v) }
    pub(crate) unsafe fn set_round_fn(self, f: usize) { write_usize(self.0 + 0x70, f) }
    /// +0x74 the super round period mask, +0x78 the super round 45 period, +0x7c the period, +0x7e the phase, +0x80
    /// the threshold.
    pub(crate) unsafe fn set_period_mask(self, v: i32) { write_i32(self.0 + 0x74, v) }
    pub(crate) unsafe fn set_period_45(self, v: i32) { write_i32(self.0 + 0x78, v) }
    pub(crate) unsafe fn period(self) -> i16 { read_i16(self.0 + 0x7c) }
    pub(crate) unsafe fn set_period(self, v: i16) { write_i16(self.0 + 0x7c, v) }
    pub(crate) unsafe fn set_phase(self, v: i16) { write_i16(self.0 + 0x7e, v) }
    pub(crate) unsafe fn set_threshold(self, v: i16) { write_i16(self.0 + 0x80, v) }
    /// +0x82 the delta base, +0x84 the delta shift, +0x86 the angle weight (SDB, SDS, SANGW).
    pub(crate) unsafe fn delta_base(self) -> i16 { read_i16(self.0 + 0x82) }
    pub(crate) unsafe fn set_delta_base(self, v: i16) { write_i16(self.0 + 0x82, v) }
    pub(crate) unsafe fn delta_shift(self) -> i16 { read_i16(self.0 + 0x84) }
    pub(crate) unsafe fn set_delta_shift(self, v: i16) { write_i16(self.0 + 0x84, v) }
    pub(crate) unsafe fn angle_weight(self) -> i16 { read_i16(self.0 + 0x86) }
    pub(crate) unsafe fn set_angle_weight(self, v: i16) { write_i16(self.0 + 0x86, v) }
    /// +0x8a the auto flip flag (FLIPON, FLIPOFF).
    pub(crate) unsafe fn auto_flip(self) -> u8 { read_u8(self.0 + 0x8a) }
    pub(crate) unsafe fn set_auto_flip(self, v: u8) { write_u8(self.0 + 0x8a, v) }
    /// +0x90 the function definitions and +0x94 the instruction definitions (8 bytes each: +0 the start inside its
    /// program, +4 the length (u16), +6 the program (u16 for a function, u8 for an instruction, then +7 its opcode)).
    pub(crate) unsafe fn function_defs(self) -> usize { read_usize(self.0 + 0x90) }
    pub(crate) unsafe fn instruction_defs(self) -> usize { read_usize(self.0 + 0x94) }
    /// +0x9c the first byte of each program (0 the font program, 1 the control value program, ...).
    pub(crate) unsafe fn program_base(self, k: usize) -> usize { read_usize(self.0 + 0x9c + 4 * k) }
    /// +0xb6 square pixels (no stretch), +0xb7 the GETINFO flags (bit 0 rotated, bit 1 stretched).
    pub(crate) unsafe fn square_pixels(self) -> bool { read_u8(self.0 + 0xb6) != 0 }
    pub(crate) unsafe fn info_flags(self) -> u8 { read_u8(self.0 + 0xb7) }
    /// +0xc0 the angle points and +0xc4 the angle distances (AA).
    pub(crate) unsafe fn angle_points(self) -> usize { read_usize(self.0 + 0xc0) }
    pub(crate) unsafe fn angle_distances(self) -> usize { read_usize(self.0 + 0xc4) }
    /// +0xc8 INSTCTRL allowed (set while the control value program runs), +0xc9 the program running (the index of
    /// `program_base`), +0xca the number of instruction definitions.
    pub(crate) unsafe fn instruct_control_allowed(self) -> bool { read_u8(self.0 + 0xc8) != 0 }
    pub(crate) unsafe fn current_program(self) -> u8 { read_u8(self.0 + 0xc9) }
    pub(crate) unsafe fn instruction_def_count(self) -> i16 { read_i16(self.0 + 0xca) }
    pub(crate) unsafe fn set_instruction_def_count(self, v: i16) { write_i16(self.0 + 0xca, v) }
    /// The font-unit scaler (+0x98) on a full 32-bit value: (global state, value) -> 26.6.
    pub(crate) unsafe fn scale_distance(self, v: i32) -> i32 {
        let f: unsafe extern "C" fn(TtGlobalState, i32) -> i32 = core::mem::transmute(read_usize(self.0 + 0x98));
        f(self, v)
    }
}

/// `v` rounded by the state's current round function, without engine compensation.
pub(crate) unsafe fn tt_round(gs: TtLocalState, v: i32) -> i32 {
    let f: TtRoundFn = core::mem::transmute(gs.global().round_fn());
    f(gs, v, 0)
}

/// Opcodes 0x13-0x16, SZP0, SZP1, SZP2 and SZPS: pops a zone number and points zone pointer 0, 1 or 2, or all three
/// (SZPS), at that zone record (`TtZone::SIZE` bytes each from the zone array). Any other opcode routed here only pops.
/// Original: `tt_op_szp` (0x938880, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_szp(gs: TtLocalState) {
    let n = gs.pop();
    let zone = gs.zones().wrapping_add((n as usize).wrapping_mul(TtZone::SIZE));
    match gs.opcode().wrapping_sub(0x13) {
        k @ 0..=2 => gs.set_zone(k as usize, zone),
        3 => (0..3).for_each(|k| gs.set_zone(k, zone)),
        _ => {}
    }
}

/// Opcode 0x76, SROUND: pops the super round byte, decodes it for a grid period and installs `tt_super_round`.
/// Original: `tt_op_sround` (0x9388d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_sround(gs: TtLocalState) {
    let v = gs.pop();
    tt_set_super_round_params(gs, v, 1);
    gs.global().set_round_fn(TT_SUPER_ROUND);
}

/// Opcode 0x77, S45ROUND: pops the super round byte, decodes it for a 45° period and installs `tt_super_round_45`.
/// Original: `tt_op_s45round` (0x938900, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_s45round(gs: TtLocalState) {
    let v = gs.pop();
    tt_set_super_round_params(gs, v, 0);
    gs.global().set_round_fn(TT_SUPER_ROUND_45);
}

/// Opcodes RTG (0x18), RTHG (0x19), RTDG (0x3d), ROFF (0x7a), RUTG (0x7c) and RDTG (0x7d): install the matching round
/// function; any other opcode routed here changes nothing.
/// Original: `tt_op_set_round_mode` (0x938930, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_set_round_mode(gs: TtLocalState) {
    let f = match gs.opcode() {
        0x18 => TT_ROUND_TO_GRID,
        0x19 => TT_ROUND_TO_HALF_GRID,
        0x3d => TT_ROUND_TO_DOUBLE_GRID,
        0x7a => TT_ROUND_OFF,
        0x7c => TT_ROUND_UP_TO_GRID,
        0x7d => TT_ROUND_DOWN_TO_GRID,
        _ => return,
    };
    gs.global().set_round_fn(f);
}

/// Decodes a super round byte `v` into the global state's period, phase and threshold:
/// - the period from bits 6-7: on the grid (`grid` non-zero) 32, 64 or 128 (half, one or two pixels; 999 for the
///   reserved value), with the period mask `!(period − 1)`; at 45° the period in 16.16 (√2/2, √2 or 2√2 pixels, 999
///   for the reserved value) kept at +0x78 and its whole 26.6 part as the period;
/// - the phase from bits 4-5: 0, a quarter, a half or three quarters of the period (rounded);
/// - the threshold from bits 0-3: the period − 1 for 0, else `((n − 4) × period + 4) / 8`.
/// Original: `tt_set_super_round_params` (0x938a10, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_set_super_round_params(gs: TtLocalState, v: i32, grid: i32) {
    let g = gs.global();
    let selector = v & 0xc0;
    if grid != 0 {
        let period: i16 = match selector {
            0 => 0x20,
            0x40 => 0x40,
            0x80 => 0x80,
            _ => 0x3e7,
        };
        g.set_period(period);
        g.set_period_mask(!(period as i32 - 1));
    } else {
        let period_45 = match selector {
            0 => 0x16a09e66,
            0x40 => 0x2d413ccd,
            0x80 => 0x5a82799a,
            _ => 0x3e7,
        };
        g.set_period_45(period_45);
        g.set_period((period_45.wrapping_add(0x800000) >> 24) as i16);
    }
    let period = g.period() as i32;
    let phase = match v & 0x30 {
        0 => 0,
        0x10 => (period + 2) >> 2,
        0x20 => (period + 1) >> 1,
        _ => (3 * period + 2) >> 2,
    };
    g.set_phase(phase as i16);
    let n = v & 0xf;
    let threshold = if n == 0 { g.period().wrapping_sub(1) } else { ((n - 4) * period + 4 >> 3) as i16 };
    g.set_threshold(threshold);
}

/// Opcode 0x28 (T2K's own): pushes the glyph's unhinted advance width, the distance between its two phantom points
/// (the two original x coordinates after the last contour's end point).
/// Original: `tt_op_push_advance_width` (0x938b80, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_push_advance_width(gs: TtLocalState) {
    let glyph = TtZone(gs.zones() + TtZone::SIZE);
    let last = at_i16(glyph.contour_ends(), glyph.contour_count() as i32 - 1) as i32;
    let ox = glyph.ox();
    gs.push(at_i32(ox, last + 2).wrapping_sub(at_i32(ox, last + 1)));
}

/// Opcode 0x20, DUP: pushes a copy of the top value.
/// Original: `tt_op_dup` (0x938bc0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_dup(gs: TtLocalState) {
    gs.push(read_i32(gs.stack_ptr() - 4));
}

/// Opcode 0x22, CLEAR: empties the stack (the pointer back at its base).
/// Original: `tt_op_clear` (0x938be0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_clear(gs: TtLocalState) {
    gs.set_stack_ptr(gs.global().stack_base());
}

/// Opcode 0x23, SWAP: swaps the top two values.
/// Original: `tt_op_swap` (0x938bf0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_swap(gs: TtLocalState) {
    let sp = gs.stack_ptr();
    let (top, below) = (read_i32(sp - 4), read_i32(sp - 8));
    write_i32(sp - 8, top);
    write_i32(sp - 4, below);
}

/// Opcode 0x24, DEPTH: pushes the number of values on the stack.
/// Original: `tt_op_depth` (0x938c10, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_depth(gs: TtLocalState) {
    let depth = (gs.stack_ptr().wrapping_sub(gs.global().stack_base()) as i32) >> 2;
    gs.push(depth);
}

/// Opcode 0x25, CINDEX: replaces the top value n with a copy of the value n places below it.
/// Original: `tt_op_cindex` (0x938c40, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_cindex(gs: TtLocalState) {
    let top = gs.stack_ptr() - 4;
    let n = read_i32(top);
    write_i32(top, read_i32(top.wrapping_sub((n as usize).wrapping_mul(4))));
}

/// Opcode 0x26, MINDEX: pops n and moves the value n places below it to the top, the values above it sliding down
/// one place (n = 0 leaves the stack as it was).
/// Original: `tt_op_mindex` (0x938c60, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_mindex(gs: TtLocalState) {
    let mut top = gs.stack_ptr() - 4;
    let n = read_i32(top);
    let mut at = top.wrapping_sub((n as usize).wrapping_mul(4));
    let v = read_i32(at);
    if n != 0 {
        for _ in 0..n as u32 {
            write_i32(at, read_i32(at + 4));
            at += 4;
        }
        top -= 4;
    }
    write_i32(top, v);
    gs.set_stack_ptr(top + 4);
}

/// Comparers: only the original (they change the interpreter's state; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_op_szp(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I0%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_sround(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_s45round(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_set_round_mode(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_set_super_round_params(gs: TtLocalState, v: i32, grid: i32) {
    original_fn::<unsafe extern "C" fn(TtLocalState, i32, i32)>(%%I4%%)(gs, v, grid)
}

pub(crate) unsafe extern "C" fn compare_tt_op_push_advance_width(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I5%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_dup(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_clear(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I7%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_swap(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I8%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_depth(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I9%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_cindex(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I10%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_mindex(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I11%%)(gs)
}
