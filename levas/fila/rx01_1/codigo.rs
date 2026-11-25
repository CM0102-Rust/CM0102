// ---------- rx01_1: the T2K TrueType interpreter: its states, the opcode table, the instruction loops and the zone callbacks (library_without_asserts, T2K fnt) ----------

/// The T2K library's fixed-point helpers (rx03 ports them; called here through their exe addresses).
const FN_LIB_FIXED_MUL: usize = 0x9446e0; // 16.16 × 16.16, rounded
const FN_LIB_F2DOT14_MUL: usize = 0x9447e0; // value × 2.14 factor, rounded
const FN_LIB_FIXED_DIV: usize = 0x944840; // 16.16 ÷ 16.16, rounded
const FN_LIB_FIXED_HYPOT: usize = 0x9448b0; // the length of (x, y)
/// The C runtime's longjmp (cdecl, never returns).
const FN_CRT_LONGJMP: usize = 0x94791c;

/// `fixed_mul` (0x9446e0, cdecl): a × b in 16.16, rounded.
pub(crate) unsafe fn lib_fixed_mul(a: i32, b: i32) -> i32 {
    let f: unsafe extern "C" fn(i32, i32) -> i32 = core::mem::transmute(FN_LIB_FIXED_MUL);
    f(a, b)
}

/// `f2dot14_mul` (0x9447e0, cdecl): a × b where b is a 2.14 factor (only its low 16 bits are read), rounded.
pub(crate) unsafe fn lib_f2dot14_mul(a: i32, b: i16) -> i32 {
    let f: unsafe extern "C" fn(i32, i32) -> i32 = core::mem::transmute(FN_LIB_F2DOT14_MUL);
    f(a, b as i32)
}

/// `fixed_div` (0x944840, cdecl): a ÷ b in 16.16, rounded.
pub(crate) unsafe fn lib_fixed_div(a: i32, b: i32) -> i32 {
    let f: unsafe extern "C" fn(i32, i32) -> i32 = core::mem::transmute(FN_LIB_FIXED_DIV);
    f(a, b)
}

/// `fixed_hypot` (0x9448b0, cdecl): the length of the vector (x, y).
pub(crate) unsafe fn lib_fixed_hypot(x: i32, y: i32) -> i32 {
    let f: unsafe extern "C" fn(i32, i32) -> i32 = core::mem::transmute(FN_LIB_FIXED_HYPOT);
    f(x, y)
}

/// The interpreter's error exit: longjmp to the state's jump buffer with `code` (never returns).
pub(crate) unsafe fn tt_raise(gs: TtLocalState, code: i32) -> ! {
    let f: unsafe extern "C" fn(usize, i32) -> ! = core::mem::transmute(FN_CRT_LONGJMP);
    f(gs.0 + 0x5a, code)
}

/// An interpreter callback: an opcode handler or the instruction loop's trace hook (cdecl, the local state only).
pub(crate) type TtOpcodeFn = unsafe extern "C" fn(TtLocalState);
/// The point mover of the state (`move_point`): (state, zone, point, distance along the freedom vector).
pub(crate) type TtMovePointFn = unsafe extern "C" fn(TtLocalState, TtZone, i32, i32);
/// The projectors of the state (`project`, `dual_project`): (state, dx, dy) -> the distance along the vector.
pub(crate) type TtProjectFn = unsafe extern "C" fn(TtLocalState, i32, i32) -> i32;
/// The value readers of the state (`get_cvt`): (state, index) -> the value.
pub(crate) type TtReadFn = unsafe extern "C" fn(TtLocalState, i32) -> i32;
/// The single width reader of the state (`get_single_width`): (state) -> the value.
pub(crate) type TtValueFn = unsafe extern "C" fn(TtLocalState) -> i32;
/// The instruction loop of the state (`interpreter`): (state, first byte, end).
pub(crate) type TtInterpreterFn = unsafe extern "C" fn(TtLocalState, usize, usize);

/// The interpreter's local graphics state (T2K `fnt_LocalGraphicStateType`, packed, 0xa0 bytes on the stack of
/// `tt_interpreter_run`, 0x937090). Vectors are 2.14 shorts (0x4000 = 1.0).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct TtLocalState(pub(crate) usize);

#[allow(dead_code)]
impl TtLocalState {
    /// +0 the next instruction byte.
    pub(crate) unsafe fn ins_ptr(self) -> usize { read_usize(self.0) }
    pub(crate) unsafe fn set_ins_ptr(self, v: usize) { write_usize(self.0, v) }
    /// +4 the opcode being run.
    pub(crate) unsafe fn opcode(self) -> u32 { read_u32(self.0 + 4) }
    pub(crate) unsafe fn set_opcode(self, v: u32) { write_u32(self.0 + 4, v) }
    /// +8, +0xc, +0x10 the zone pointers 0, 1 and 2 (twilight or glyph).
    pub(crate) unsafe fn zone(self, k: usize) -> TtZone { TtZone(read_usize(self.0 + 8 + 4 * k)) }
    /// +0x14 the projection vector.
    pub(crate) unsafe fn proj(self) -> (i16, i16) { (read_i16(self.0 + 0x14), read_i16(self.0 + 0x16)) }
    pub(crate) unsafe fn set_proj(self, x: i16, y: i16) { write_i16(self.0 + 0x14, x); write_i16(self.0 + 0x16, y) }
    /// +0x18 the freedom vector.
    pub(crate) unsafe fn free(self) -> (i16, i16) { (read_i16(self.0 + 0x18), read_i16(self.0 + 0x1a)) }
    pub(crate) unsafe fn set_free(self, x: i16, y: i16) { write_i16(self.0 + 0x18, x); write_i16(self.0 + 0x1a, y) }
    /// +0x1c the dual projection vector (of the original outline).
    pub(crate) unsafe fn dual(self) -> (i16, i16) { (read_i16(self.0 + 0x1c), read_i16(self.0 + 0x1e)) }
    pub(crate) unsafe fn set_dual(self, x: i16, y: i16) { write_i16(self.0 + 0x1c, x); write_i16(self.0 + 0x1e, y) }
    /// +0x20 the stack pointer (the next free 32-bit slot).
    pub(crate) unsafe fn stack_ptr(self) -> usize { read_usize(self.0 + 0x20) }
    pub(crate) unsafe fn set_stack_ptr(self, v: usize) { write_usize(self.0 + 0x20, v) }
    /// Pops the top of the stack.
    pub(crate) unsafe fn pop(self) -> i32 {
        let sp = self.stack_ptr() - 4;
        self.set_stack_ptr(sp);
        read_i32(sp)
    }
    /// Pushes a value.
    pub(crate) unsafe fn push(self, v: i32) {
        let sp = self.stack_ptr();
        write_i32(sp, v);
        self.set_stack_ptr(sp + 4);
    }
    /// +0x24 the zone array (zone 0 the twilight zone, zone 1 the glyph; `TtZone::SIZE` each).
    pub(crate) unsafe fn zones(self) -> usize { read_usize(self.0 + 0x24) }
    /// +0x28 the global graphics state.
    pub(crate) unsafe fn global(self) -> TtGlobalState { TtGlobalState(read_usize(self.0 + 0x28)) }
    /// +0x2c the trace hook (0 without one).
    pub(crate) unsafe fn trace(self) -> usize { read_usize(self.0 + 0x2c) }
    /// +0x30, +0x34, +0x38 the reference points 0, 1 and 2.
    pub(crate) unsafe fn set_reference_point(self, k: usize, v: i32) { write_i32(self.0 + 0x30 + 4 * k, v) }
    /// +0x3c the round state as a number, for the trace hook (0 half grid, 1 grid, 2 double grid, 3 down, 4 up,
    /// 5 off, 6 super, 7 super 45, −1 other).
    pub(crate) unsafe fn set_round_state_code(self, v: i16) { write_i16(self.0 + 0x3c, v) }
    /// +0x3e the loop counter (SLOOP value − 1).
    pub(crate) unsafe fn set_loop(self, v: i16) { write_i16(self.0 + 0x3e, v) }
    /// +0x40 the dot product of the projection and the freedom vectors (2.14).
    pub(crate) unsafe fn proj_dot_free(self) -> i16 { read_i16(self.0 + 0x40) }
    pub(crate) unsafe fn set_proj_dot_free(self, v: i16) { write_i16(self.0 + 0x40, v) }
    /// +0x42 the point mover, +0x46 the projector, +0x4a the dual projector, +0x4e the instruction loop,
    /// +0x52 the control value reader, +0x56 the single width reader (function addresses).
    pub(crate) unsafe fn set_move_point(self, f: usize) { write_usize(self.0 + 0x42, f) }
    pub(crate) unsafe fn project_fn(self) -> usize { read_usize(self.0 + 0x46) }
    pub(crate) unsafe fn set_project(self, f: usize) { write_usize(self.0 + 0x46, f) }
    pub(crate) unsafe fn set_dual_project(self, f: usize) { write_usize(self.0 + 0x4a, f) }
}

/// A zone of points (T2K `fnt_ElementType`, 0x2c bytes): the twilight zone or the glyph's outline. Coordinates are
/// 26.6 integers.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct TtZone(pub(crate) usize);

#[allow(dead_code)]
impl TtZone {
    pub(crate) const SIZE: usize = 0x2c;
    /// +0 the current x coordinates, +4 the current y coordinates (i32 arrays).
    pub(crate) unsafe fn x(self) -> usize { read_usize(self.0) }
    pub(crate) unsafe fn y(self) -> usize { read_usize(self.0 + 4) }
    /// +8 the original x coordinates, +0xc the original y coordinates (i32 arrays).
    pub(crate) unsafe fn ox(self) -> usize { read_usize(self.0 + 8) }
    pub(crate) unsafe fn oy(self) -> usize { read_usize(self.0 + 0xc) }
    /// +0x28 the per-point flags (u8 array; bit 0 touched in x, bit 1 touched in y).
    pub(crate) unsafe fn flags(self) -> usize { read_usize(self.0 + 0x28) }
    /// Adds `d` to point `p`'s current coordinate in `coords` (x or y) and sets `touched` in its flags.
    pub(crate) unsafe fn shift_and_touch(self, coords: usize, p: i32, d: i32, touched: u8) {
        let at = coords.wrapping_add((p as usize).wrapping_mul(4));
        write_i32(at, read_i32(at).wrapping_add(d));
        let f = self.flags().wrapping_add(p as usize);
        write_u8(f, read_u8(f) | touched);
    }
}

/// The interpreter's global graphics state (T2K `fnt_GlobalGraphicStateType`, packed), shared by the font program,
/// the control value program and the glyph programs of one font.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct TtGlobalState(pub(crate) usize);

#[allow(dead_code)]
impl TtGlobalState {
    /// +8 the control value table (i32, scaled).
    pub(crate) unsafe fn cvt(self) -> usize { read_usize(self.0 + 8) }
    /// +0x58 the control value cut-in, +0x5c the single width cut-in, +0x60 the single width (scaled), +0x6c the
    /// minimum distance (26.6).
    pub(crate) unsafe fn set_cvt_cut_in(self, v: i32) { write_i32(self.0 + 0x58, v) }
    pub(crate) unsafe fn set_single_width_cut_in(self, v: i32) { write_i32(self.0 + 0x5c, v) }
    pub(crate) unsafe fn single_width(self) -> i32 { read_i32(self.0 + 0x60) }
    pub(crate) unsafe fn set_single_width(self, v: i32) { write_i32(self.0 + 0x60, v) }
    pub(crate) unsafe fn set_min_distance(self, v: i32) { write_i32(self.0 + 0x6c, v) }
    /// +0x70 the round function (one of the `tt_round_*` addresses).
    pub(crate) unsafe fn round_fn(self) -> usize { read_usize(self.0 + 0x70) }
    /// +0x74 the super round period mask, +0x78 the super round 45 period (16.16), +0x7e the phase, +0x80 the
    /// threshold (26.6 shorts).
    pub(crate) unsafe fn period_mask(self) -> i32 { read_i32(self.0 + 0x74) }
    pub(crate) unsafe fn period_45(self) -> i32 { read_i32(self.0 + 0x78) }
    pub(crate) unsafe fn phase(self) -> i16 { read_i16(self.0 + 0x7e) }
    pub(crate) unsafe fn threshold(self) -> i16 { read_i16(self.0 + 0x80) }
    /// +0x88 the single width in font units (SSW).
    pub(crate) unsafe fn set_single_width_units(self, v: i16) { write_i16(self.0 + 0x88, v) }
    /// +0x8c the opcode table (256 handler addresses, then the angle tables at +0x400 and +0x4a0).
    pub(crate) unsafe fn opcode_table(self) -> usize { read_usize(self.0 + 0x8c) }
    /// +0x98 the font-unit scaler: (global state, value) -> 26.6.
    pub(crate) unsafe fn scale(self, v: i16) -> i32 {
        let f: unsafe extern "C" fn(TtGlobalState, i32) -> i32 = core::mem::transmute(read_usize(self.0 + 0x98));
        f(self, v as i32)
    }
    /// +0xb8 the x stretch and +0xbc the y stretch of non-square pixels (16.16).
    pub(crate) unsafe fn x_stretch(self) -> i32 { read_i32(self.0 + 0xb8) }
    pub(crate) unsafe fn y_stretch(self) -> i32 { read_i32(self.0 + 0xbc) }
    /// +0xc0 the angle points and +0xc4 the angle distances (inside the opcode table block).
    pub(crate) unsafe fn set_angle_tables(self, points: usize, distances: usize) {
        write_usize(self.0 + 0xc0, points);
        write_usize(self.0 + 0xc4, distances);
    }
}

/// The handler of each opcode (`tt_init_opcode_table`): 0x00-0x0f the vectors, 0x10-0x1f the graphics state, 0x20-0x2d
/// the stack and the functions, 0x2e-0x3f the point moves, 0x40-0x4f the pushes and the storage, 0x50-0x6f the
/// arithmetic and the conditions, 0x70-0x8e the deltas and the rest, 0x8f-0xaf the user opcodes, 0xb0-0xbf the short
/// pushes, 0xc0-0xdf MDRP and 0xe0-0xff MIRP.
const TT_OPCODE_HANDLERS: [u32; 256] = [
    0x937550, 0x937d90, 0x937dd0, 0x937dd0, 0x937f60, 0x937f60, 0x937fd0, 0x937fd0, // SVTCA y/x, SPVTCA, SFVTCA, SPVTL
    0x938280, 0x938280, 0x938360, 0x9383f0, 0x938470, 0x9384a0, 0x9384d0, 0x9384f0, // SFVTL, SPVFS, SFVFS, GPV, GFV, SFVTPV, ISECT
    0x938820, 0x938820, 0x938820, 0x938880, 0x938880, 0x938880, 0x938880, 0x938820, // SRP0-2, SZP0-2, SZPS, SLOOP
    0x938930, 0x938930, 0x938790, 0x93a600, 0x93a720, 0x9387b0, 0x9387d0, 0x9387f0, // RTG, RTHG, SMD, ELSE, JMPR, SCVTCI, SSWCI, SSW
    0x938bc0, 0x938820, 0x938be0, 0x938bf0, 0x938c10, 0x938c40, 0x938c60, 0x9397d0, // DUP, POP, CLEAR, SWAP, DEPTH, CINDEX, MINDEX, ALIGNPTS
    0x938b80, 0x93aff0, 0x93ad80, 0x93abf0, 0x93ac30, 0x937d70, 0x938cc0, 0x938cc0, // 0x28, UTP, LOOPCALL, CALL, FDEF, ENDF, MDAP
    0x938e20, 0x938e20, 0x939110, 0x939110, 0x9392a0, 0x9392a0, 0x939360, 0x939360, // IUP, SHP, SHC, SHZ
    0x9394a0, 0x939540, 0x939670, 0x939670, 0x939750, 0x938930, 0x938d40, 0x938d40, // SHPIX, IP, MSIRP, ALIGNRP, RTDG, MIAP
    0x939d50, 0x939de0, 0x939e30, 0x939e60, 0x939e90, 0x939f20, 0x939f50, 0x939f50, // NPUSHB, NPUSHW, WS, RS, WCVTP, RCVT, GC
    0x939fc0, 0x93a020, 0x93a020, 0x93a0d0, 0x93a190, 0x93a210, 0x93a220, 0x93a230, // SCFS, MD, MPPEM, MPS, FLIPON, FLIPOFF, DEBUG
    0x93a240, 0x93a240, 0x93a240, 0x93a240, 0x93a240, 0x93a240, 0x93a400, 0x93a400, // LT..NEQ, ODD, EVEN
    0x93a4d0, 0x93a710, 0x93a240, 0x93a240, 0x93a400, 0x93b070, 0x93b030, 0x93b050, // IF, EIF, AND, OR, NOT, DELTAP1, SDB, SDS
    0x93a240, 0x93a240, 0x93a240, 0x93a240, 0x93a400, 0x93a400, 0x93a400, 0x93a400, // ADD..MUL, ABS, NEG, FLOOR, CEILING
    0x93a7a0, 0x93a7a0, 0x93a7a0, 0x93a7a0, 0x93a7d0, 0x93a7d0, 0x93a7d0, 0x93a7d0, // ROUND, NROUND
    0x939ef0, 0x93b1f0, 0x93b220, 0x93b250, 0x93b2a0, 0x93b2d0, 0x9388d0, 0x938900, // WCVTF, DELTAP2-3, DELTAC1-3, SROUND, S45ROUND
    0x93a740, 0x93a770, 0x938930, 0x937d70, 0x938930, 0x938930, 0x939840, 0x939a00, // JROT, JROF, ROFF, 0x7b, RUTG, RDTG, SANGW, AA
    0x939860, 0x9398a0, 0x9398e0, 0x93adf0, 0x93adf0, 0x939920, 0x9380f0, 0x9380f0, // FLIPPT, FLIPRGON, FLIPRGOFF, 0x83, 0x84, SCANCTRL, SDPVTL
    0x93a1b0, 0x93ae60, 0x938ca0, 0x93a240, 0x93a240, 0x939950, 0x9399a0, 0x93adf0, // GETINFO, IDEF, ROLL, MAX, MIN, SCANTYPE, INSTCTRL
    0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, // user opcodes
    0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0,
    0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0,
    0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0, 0x93adf0,
    0x939d10, 0x939d10, 0x939d10, 0x939d10, 0x939d10, 0x939d10, 0x939d10, 0x939d10, // PUSHB
    0x939d90, 0x939d90, 0x939d90, 0x939d90, 0x939d90, 0x939d90, 0x939d90, 0x939d90, // PUSHW
    0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, // MDRP
    0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0,
    0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0,
    0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0, 0x93a7e0,
    0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, // MIRP
    0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960,
    0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960,
    0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960, 0x93a960,
];

/// The angle points of the table block (+0x400, 39 dwords: 0, then the unit vectors as 2.30 pairs) and the angle
/// distances (+0x4a0, 20 shorts), for SANGW and AA.
const TT_ANGLE_POINTS: [u32; 39] = [
    0, 0x40000000, 0xd2bec333, 0x2d413ccd, 0xe360da3a, 0x393e4b8b, 0xebc2ec9e, 0x3cb73a27, 0xdc7fcac0, 0x35404fe1,
    0xf07a4bdc, 0x3e16d092, 0xd999999a, 0x33333333, 0xf372d54f, 0x3ec1d573, 0xe83b229a, 0x3b6c2980, 0xdf128065,
    0x36e12a02, 0xf57a7cac, 0x3f2113fb, 0xd804ff10, 0x31f9c12c, 0xf6f2f3d7, 0x3f5b551f, 0xee6af813, 0x3d899bbd,
    0xe6ca06b6, 0x3ad345ad, 0xd7073803, 0x312a8996, 0xe03f4277, 0x37914baf, 0xf80fd09e, 0x3f817b11, 0xe987318b,
    0x3becd137, 0x3414358e,
];
const TT_ANGLE_DISTANCES: [u16; 20] =
    [64, 91, 143, 202, 231, 264, 320, 326, 345, 373, 389, 410, 453, 466, 487, 500, 516, 516, 547, 551];

/// Fills the opcode table of a global state: the handler of each of the 256 opcodes (`TT_OPCODE_HANDLERS`), then the
/// angle points at +0x400 and the angle distances at +0x4a0 of the same block, and points the state's angle table
/// fields at them. The last dword of the angle block (+0x49c) stays as it was.
/// Original: `tt_init_opcode_table` (0x9375d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_init_opcode_table(global: TtGlobalState) {
    let table = global.opcode_table();
    for (k, &h) in TT_OPCODE_HANDLERS.iter().enumerate() {
        write_u32(table + 4 * k, h);
    }
    let (points, distances) = (table + 0x400, table + 0x4a0);
    global.set_angle_tables(points, distances);
    for (k, &v) in TT_ANGLE_POINTS.iter().enumerate() {
        write_u32(points + 4 * k, v);
    }
    for (k, &v) in TT_ANGLE_DISTANCES.iter().enumerate() {
        write_u16(distances + 2 * k, v);
    }
}

/// Runs the instructions from `start` to `end`: each byte is the opcode, stored in the state, and its handler from
/// the global state's table runs (the table read once, at the start; a handler may move the instruction pointer). The
/// instruction pointer is restored at the end.
/// Original: `tt_run_ins_range` (0x937500, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_run_ins_range(gs: TtLocalState, start: usize, end: usize) {
    let table = gs.global().opcode_table();
    let saved = gs.ins_ptr();
    gs.set_ins_ptr(start);
    while gs.ins_ptr() < end {
        run_one_opcode(gs, table);
    }
    gs.set_ins_ptr(saved);
}

/// Fetches the opcode at the instruction pointer, stores it, steps past it and runs its handler.
unsafe fn run_one_opcode(gs: TtLocalState, table: usize) {
    let p = gs.ins_ptr();
    let op = read_u8(p) as usize;
    gs.set_opcode(op as u32);
    gs.set_ins_ptr(p + 1);
    let handler: TtOpcodeFn = core::mem::transmute(read_usize(table + 4 * op));
    handler(gs);
}

/// The round state as the number the trace hook reads.
unsafe fn round_state_code(round_fn: usize) -> i16 {
    match round_fn {
        0x936f10 => 1, // to grid
        0x936f40 => 0, // to half grid
        0x936e80 => 2, // to double grid
        0x936eb0 => 3, // down to grid
        0x936ee0 => 4, // up to grid
        0x936f80 => 5, // off
        0x936f90 => 6, // super
        0x936ff0 => 7, // super 45
        _ => -1,
    }
}

/// Runs the instructions from `start` to `end` like `tt_run_ins_range`, with the trace hook: before each opcode the
/// round state is stored as a number (`round_state_code`) and the hook runs; it may clear itself, which ends the run.
/// Without a hook nothing runs. The instruction pointer is restored at the end.
/// Original: `tt_run_ins_range_with_callback` (0x937410, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_run_ins_range_with_callback(gs: TtLocalState, start: usize, end: usize) {
    let saved = gs.ins_ptr();
    gs.set_ins_ptr(start);
    let global = gs.global();
    let table = global.opcode_table();
    if gs.trace() == 0 {
        return; // the instruction pointer stays at `start`
    }
    while gs.ins_ptr() < end {
        gs.set_round_state_code(round_state_code(global.round_fn()));
        let hook: TtOpcodeFn = core::mem::transmute(gs.trace());
        hook(gs);
        if gs.trace() == 0 {
            break;
        }
        run_one_opcode(gs, table);
    }
    gs.set_ins_ptr(saved);
}

/// The handler of the undefined opcodes (and ENDF outside a definition): the interpreter's error exit with 0x1101.
/// Original: `tt_op_invalid_opcode` (0x937d70, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_invalid_opcode(gs: TtLocalState) {
    tt_raise(gs, 0x1101)
}

/// Moves point `p` of `zone` by `d` in x and marks it touched in x (the point mover of the x axis).
/// Original: `tt_move_point_and_touch` (0x937260, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_move_point_and_touch(_gs: TtLocalState, zone: TtZone, p: i32, d: i32) {
    zone.shift_and_touch(zone.x(), p, d, 1);
}

/// Moves point `p` of `zone` by `d` in y and marks it touched in y (the point mover of the y axis).
/// Original: `tt_direct_move_y` (0x937590, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_direct_move_y(_gs: TtLocalState, zone: TtZone, p: i32, d: i32) {
    zone.shift_and_touch(zone.y(), p, d, 2);
}

/// The projection on the x axis: dx.
/// Original: `tt_project_x` (0x937290, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_project_x(_gs: TtLocalState, dx: i32, _dy: i32) -> i32 {
    dx
}

/// The projection on the y axis: dy.
/// Original: `tt_project_y` (0x9375c0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_project_y(_gs: TtLocalState, _dx: i32, dy: i32) -> i32 {
    dy
}

/// The projection of (dx, dy) on the projection vector: dx × x + dy × y (each by `f2dot14_mul`).
/// Original: `tt_project` (0x9380c0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_project(gs: TtLocalState, dx: i32, dy: i32) -> i32 {
    let (x, y) = gs.proj();
    lib_f2dot14_mul(dx, x).wrapping_add(lib_f2dot14_mul(dy, y))
}

/// The projection of (dx, dy) on the dual projection vector: dx × x + dy × y (each by `f2dot14_mul`).
/// Original: `tt_dual_project` (0x938250, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_dual_project(gs: TtLocalState, dx: i32, dy: i32) -> i32 {
    let (x, y) = gs.dual();
    lib_f2dot14_mul(dx, x).wrapping_add(lib_f2dot14_mul(dy, y))
}

/// Control value `n`, unscaled (square pixels).
/// Original: `tt_read_cvt` (0x9372a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_read_cvt(gs: TtLocalState, n: i32) -> i32 {
    read_i32(gs.global().cvt().wrapping_add((n as usize).wrapping_mul(4)))
}

/// The scaled single width value of the global state (square pixels).
/// Original: `tt_single_width` (0x937350, library_without_asserts, T2K fnt.c, cdecl; "tt_current_ppem" in the
/// first survey: it reads the single width, +0x60, which SSW writes).
pub(crate) unsafe extern "C" fn tt_single_width(gs: TtLocalState) -> i32 {
    gs.global().single_width()
}

/// Comparers: the opcode table side by side (the original fills the game's table, the copy a table of its own, and
/// both blocks compared); the pure readers and projectors value against value; the loops, the error exit and the point
/// movers only the original (they run handlers or change the points; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_init_opcode_table(global: TtGlobalState) {
    original_fn::<unsafe extern "C" fn(TtGlobalState)>(%%I0%%)(global);
    let mut block = [0u8; 0x4c8];
    let mut state = [0u8; 0xc8];
    let own = TtGlobalState(state.as_mut_ptr() as usize);
    write_usize(own.0 + 0x8c, block.as_mut_ptr() as usize);
    tt_init_opcode_table(own);
    let game = global.opcode_table();
    let same = (0..0x49c).all(|k| read_u8(game + k) == block[k]) && (0x4a0..0x4c8).all(|k| read_u8(game + k) == block[k]);
    SCOREBOARD.record(same, || "tt_init_opcode_table: table differs".to_string());
}

pub(crate) unsafe extern "C" fn compare_tt_run_ins_range(gs: TtLocalState, start: usize, end: usize) {
    original_fn::<TtInterpreterFn>(%%I1%%)(gs, start, end)
}

pub(crate) unsafe extern "C" fn compare_tt_run_ins_range_with_callback(gs: TtLocalState, start: usize, end: usize) {
    original_fn::<TtInterpreterFn>(%%I2%%)(gs, start, end)
}

pub(crate) unsafe extern "C" fn compare_tt_op_invalid_opcode(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_move_point_and_touch(gs: TtLocalState, zone: TtZone, p: i32, d: i32) {
    original_fn::<TtMovePointFn>(%%I4%%)(gs, zone, p, d)
}

pub(crate) unsafe extern "C" fn compare_tt_direct_move_y(gs: TtLocalState, zone: TtZone, p: i32, d: i32) {
    original_fn::<TtMovePointFn>(%%I5%%)(gs, zone, p, d)
}

/// A projector side by side.
unsafe fn compare_tt_projector(name: &str, original: TtProjectFn, copy: TtProjectFn, gs: TtLocalState, dx: i32, dy: i32) -> i32 {
    let r = original(gs, dx, dy);
    let c = copy(gs, dx, dy);
    SCOREBOARD.record(r == c, || format!("{}({}, {}) original={} rust={}", name, dx, dy, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_project_x(gs: TtLocalState, dx: i32, dy: i32) -> i32 {
    compare_tt_projector("tt_project_x", original_fn::<TtProjectFn>(%%I6%%), tt_project_x, gs, dx, dy)
}

pub(crate) unsafe extern "C" fn compare_tt_project_y(gs: TtLocalState, dx: i32, dy: i32) -> i32 {
    compare_tt_projector("tt_project_y", original_fn::<TtProjectFn>(%%I7%%), tt_project_y, gs, dx, dy)
}

pub(crate) unsafe extern "C" fn compare_tt_project(gs: TtLocalState, dx: i32, dy: i32) -> i32 {
    compare_tt_projector("tt_project", original_fn::<TtProjectFn>(%%I8%%), tt_project, gs, dx, dy)
}

pub(crate) unsafe extern "C" fn compare_tt_dual_project(gs: TtLocalState, dx: i32, dy: i32) -> i32 {
    compare_tt_projector("tt_dual_project", original_fn::<TtProjectFn>(%%I9%%), tt_dual_project, gs, dx, dy)
}

pub(crate) unsafe extern "C" fn compare_tt_read_cvt(gs: TtLocalState, n: i32) -> i32 {
    let r = original_fn::<TtReadFn>(%%I10%%)(gs, n);
    let c = tt_read_cvt(gs, n);
    SCOREBOARD.record(r == c, || format!("tt_read_cvt({}) original={} rust={}", n, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_single_width(gs: TtLocalState) -> i32 {
    let r = original_fn::<TtValueFn>(%%I11%%)(gs);
    let c = tt_single_width(gs);
    SCOREBOARD.record(r == c, || format!("tt_single_width original={} rust={}", r, c));
    r
}
