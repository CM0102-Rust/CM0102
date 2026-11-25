// ---------- rx02_5: the T2K TrueType interpreter: GETINFO, the flags, the arithmetic, IF and ELSE, the jumps and ROUND (library_without_asserts, T2K fnt) ----------

/// The T2K library's 26.6 multiply (rx03 ports it; called here through its exe address).
const FN_LIB_F26DOT6_MUL: usize = 0x944760; // a × b ÷ 64, rounded

/// `f26dot6_mul` (0x944760, cdecl): a × b in 26.6.
pub(crate) unsafe fn lib_f26dot6_mul(a: i32, b: i32) -> i32 {
    let f: unsafe extern "C" fn(i32, i32) -> i32 = core::mem::transmute(FN_LIB_F26DOT6_MUL);
    f(a, b)
}

/// Opcode 0x88, GETINFO: pops a selector and pushes the information word: 1 (the version) for bit 0, 0x100 for bit 1
/// when the glyph is rotated, 0x200 for bit 2 when it is stretched (the global state's flags).
/// Original: `tt_op_getinfo` (0x93a1b0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_getinfo(gs: TtLocalState) {
    let selector = gs.pop();
    let flags = gs.global().info_flags();
    let mut info = 0;
    if selector & 1 != 0 {
        info = 1;
    }
    if selector & 2 != 0 && flags & 1 != 0 {
        info |= 0x100;
    }
    if selector & 4 != 0 && flags & 2 != 0 {
        info |= 0x200;
    }
    gs.push(info);
}

/// Opcode 0x4D, FLIPON: sets the auto flip flag.
/// Original: `tt_op_flipon` (0x93a210, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_flipon(gs: TtLocalState) {
    gs.global().set_auto_flip(1);
}

/// Opcode 0x4E, FLIPOFF: clears the auto flip flag.
/// Original: `tt_op_flipoff` (0x93a220, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_flipoff(gs: TtLocalState) {
    gs.global().set_auto_flip(0);
}

/// Opcode 0x4F, DEBUG: pops a value and ignores it.
/// Original: `tt_op_debug` (0x93a230, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_debug(gs: TtLocalState) {
    gs.set_stack_ptr(gs.stack_ptr().wrapping_sub(4));
}

/// The two-operand opcodes: pops b, then a, and pushes LT, LTEQ, GT, GTEQ, EQ, NEQ (0x50-0x55, 1 or 0), AND, OR (0x5A,
/// 0x5B), ADD, SUB (0x60, 0x61), DIV (0x62, `(a × 64) / b`, truncated), MUL (0x63, `f26dot6_mul`), MAX, MIN (0x8B,
/// 0x8C) of a and b; any other opcode routed here only pops both. Defect of the original: DIV by 0 is not checked
/// (the processor's divide error ends the game; the copy ends it too).
/// Original: `tt_op_binary_arith_compare` (0x93a240, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_binary_arith_compare(gs: TtLocalState) {
    let b = gs.pop();
    let a = gs.pop();
    let v = match gs.opcode() {
        0x50 => (a < b) as i32,
        0x51 => (a <= b) as i32,
        0x52 => (a > b) as i32,
        0x53 => (a >= b) as i32,
        0x54 => (a == b) as i32,
        0x55 => (a != b) as i32,
        0x5a => (a != 0 && b != 0) as i32,
        0x5b => (a != 0 || b != 0) as i32,
        0x60 => a.wrapping_add(b),
        0x61 => a.wrapping_sub(b),
        0x62 => (a << 6).wrapping_div(b),
        0x63 => lib_f26dot6_mul(a, b),
        0x8b => a.max(b),
        0x8c => a.min(b),
        _ => return,
    };
    gs.push(v);
}

/// The one-operand opcodes on the top value: ODD and EVEN (0x56, 0x57: 1 when the value rounded to the grid is an odd
/// or even number of pixels), NOT (0x5C), ABS (0x64), NEG (0x65), FLOOR (0x66) and CEILING (0x67) to a whole pixel;
/// any other opcode routed here only pops it.
/// Original: `tt_op_unary_math` (0x93a400, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_unary_math(gs: TtLocalState) {
    let v = gs.pop();
    let r = match gs.opcode() {
        op @ (0x56 | 0x57) => {
            let pixels = tt_round_to_grid(gs, v, 0) >> 6;
            !(if op == 0x56 { pixels + 1 } else { pixels }) & 1
        }
        0x5c => (v == 0) as i32,
        0x64 => if v > 0 { v } else { v.wrapping_neg() },
        0x65 => v.wrapping_neg(),
        0x66 => v & !0x3f,
        0x67 => v.wrapping_add(0x3f) & !0x3f,
        _ => return,
    };
    gs.push(r);
}

/// Reads the opcode at the instruction pointer, stores it in the state and steps past it.
pub(crate) unsafe fn tt_fetch_opcode(gs: TtLocalState) -> u32 {
    let ip = gs.ins_ptr();
    let op = read_u8(ip) as u32;
    gs.set_opcode(op);
    gs.set_ins_ptr(ip + 1);
    op
}

/// Steps the instruction pointer over the inline data of the push opcode `op` just fetched (NPUSHB, NPUSHW, PUSHB,
/// PUSHW); other opcodes have none.
pub(crate) unsafe fn tt_skip_push_data(gs: TtLocalState, op: u32) {
    let ip = gs.ins_ptr();
    let data = match op {
        0x40 => read_u8(ip) as usize + 1,
        0x41 => 2 * read_u8(ip) as usize + 1,
        0xb0..=0xb7 => op as usize - 0xaf,
        0xb8..=0xbf => 2 * (op as usize - 0xb7),
        _ => 0,
    };
    gs.set_ins_ptr(ip + data);
}

/// Opcode 0x58, IF: pops a condition; when it is 0, skips the instructions (and push data) up to the ELSE of the
/// same level, which stays skipped too, or to its EIF, counting the nested IF and EIF.
/// Original: `tt_op_if` (0x93a4d0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_if(gs: TtLocalState) {
    if gs.pop() != 0 {
        return;
    }
    let mut level = 1;
    while level != 0 {
        match tt_fetch_opcode(gs) {
            0x1b => {
                if level == 1 {
                    return;
                }
            }
            0x58 => level += 1,
            0x59 => level -= 1,
            op => tt_skip_push_data(gs, op),
        }
    }
}

/// Opcode 0x1B, ELSE (reached after the IF part ran): skips the instructions (and push data) up to the EIF of the same
/// level, counting the nested IF and EIF.
/// Original: `tt_op_else` (0x93a600, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_else(gs: TtLocalState) {
    let mut level = 1;
    while level != 0 {
        match tt_fetch_opcode(gs) {
            0x58 => level += 1,
            0x59 => level -= 1,
            op => tt_skip_push_data(gs, op),
        }
    }
}

/// Moves the instruction pointer by `offset` from the jump opcode itself (the pointer is already past it).
unsafe fn tt_jump(gs: TtLocalState, offset: i32) {
    gs.set_ins_ptr(gs.ins_ptr().wrapping_add(offset.wrapping_sub(1) as usize));
}

/// Opcode 0x1C, JMPR: pops an offset and jumps by it.
/// Original: `tt_op_jmpr` (0x93a720, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_jmpr(gs: TtLocalState) {
    let offset = gs.pop();
    tt_jump(gs, offset);
}

/// Opcode 0x78, JROT: pops a condition and an offset, and jumps by the offset when the condition is not 0.
/// Original: `tt_op_jrot` (0x93a740, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_jrot(gs: TtLocalState) {
    let condition = gs.pop();
    let offset = gs.pop();
    if condition != 0 {
        tt_jump(gs, offset);
    }
}

/// Opcode 0x79, JROF: pops a condition and an offset, and jumps by the offset when the condition is 0.
/// Original: `tt_op_jrof` (0x93a770, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_jrof(gs: TtLocalState) {
    let condition = gs.pop();
    let offset = gs.pop();
    if condition == 0 {
        tt_jump(gs, offset);
    }
}

/// Opcodes 0x68-0x6B, ROUND: replaces the top value with it rounded by the current round function (no engine
/// compensation).
/// Original: `tt_op_round` (0x93a7a0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_op_round(gs: TtLocalState) {
    let v = gs.pop();
    let r = tt_round(gs, v);
    gs.push(r);
}

/// Comparers: only the original (they change the interpreter's state; proved on the emulator).
pub(crate) unsafe extern "C" fn compare_tt_op_getinfo(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I0%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_flipon(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I1%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_flipoff(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I2%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_debug(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I3%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_binary_arith_compare(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I4%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_unary_math(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I5%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_if(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I6%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_else(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I7%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_jmpr(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I8%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_jrot(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I9%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_jrof(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I10%%)(gs)
}

pub(crate) unsafe extern "C" fn compare_tt_op_round(gs: TtLocalState) {
    original_fn::<TtOpcodeFn>(%%I11%%)(gs)
}
