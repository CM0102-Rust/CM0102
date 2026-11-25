// ---------- rx03_1: the T2K fixed-point helpers and the small sort (library_without_asserts, T2K util.c) ----------

/// A two-operand T2K helper: (a, b) -> result (cdecl).
pub(crate) type FixedOpFn = unsafe extern "C" fn(i32, i32) -> i32;

/// The product sign of the T2K helpers (−1 when exactly one operand is negative) and both magnitudes (the original's
/// `neg`, so `i32::MIN` stays 0x80000000).
fn sign_and_magnitudes(a: i32, b: i32) -> (i32, u32, u32) {
    let sign = if (a < 0) != (b < 0) { -1 } else { 1 };
    (sign, a.unsigned_abs(), b.unsigned_abs())
}

/// The quotient of two magnitudes as 16.16: the whole part shifted left 16, plus the remainder (and the divisor, by an
/// arithmetic shift, as the original) halved until it fits 16 bits, then divided. A zero divisor faults, as the
/// original's `div`.
fn unsigned_fixed_div(a: u32, b: u32) -> u32 {
    let whole = (a / b) << 16;
    let (mut rest, mut divisor) = (a % b, b);
    while rest > 0xffff {
        rest >>= 1;
        divisor = ((divisor as i32) >> 1) as u32;
    }
    ((rest << 16) / divisor).wrapping_add(whole)
}

/// Sorts `n` signed 16-bit values into ascending order with a bubble sort (whole passes until nothing moves; equal
/// neighbours stay). Nothing happens for `n` < 2.
/// Original: `sort_int16_array_ascending` (0x9446a0, library_without_asserts, T2K util.c, cdecl).
pub(crate) unsafe extern "C" fn sort_int16_array_ascending(values: usize, n: i32) {
    let last = n.wrapping_sub(1);
    loop {
        let mut moved = false;
        if last <= 0 {
            return;
        }
        for k in 0..last as usize {
            let (a, b) = (read_i16(values + 2 * k), read_i16(values + 2 * k + 2));
            if b < a {
                write_i16(values + 2 * k, b);
                moved = true;
                write_i16(values + 2 * k + 2, a);
            }
        }
        if !moved {
            return;
        }
    }
}

/// Multiplies two 16.16 numbers: the magnitudes' 16-bit halves crossed, `(lo·lo >> 16) + lo·hi + hi·lo + (hi·hi << 16)`
/// (truncated, no rounding; wraps on overflow), with the product sign.
/// Original: `fixed_mul` (0x9446e0, library_without_asserts, T2K util.c, cdecl).
pub(crate) unsafe extern "C" fn fixed_mul(a: i32, b: i32) -> i32 {
    let (sign, a, b) = sign_and_magnitudes(a, b);
    let (ah, al, bh, bl) = (a >> 16, a & 0xffff, b >> 16, b & 0xffff);
    let r = (al * bl >> 16).wrapping_add(bl * ah).wrapping_add(al * bh).wrapping_add((ah * bh) << 16);
    (r as i32).wrapping_mul(sign)
}

/// Multiplies two numbers and divides the product by 64 (26.6 fixed point), without a 64-bit intermediate:
/// `((|a|·b_hi + a_hi·b_lo) << 10) + (a_lo·b_lo >> 6)` (truncated; wraps), with the product sign.
/// Original: `f26dot6_mul` (0x944760, library_without_asserts, T2K util.c, cdecl).
pub(crate) unsafe extern "C" fn f26dot6_mul(a: i32, b: i32) -> i32 {
    let (sign, a, b) = sign_and_magnitudes(a, b);
    let (ah, al, bh, bl) = (a >> 16, a & 0xffff, b >> 16, b & 0xffff);
    let high = a.wrapping_mul(bh).wrapping_add(bl * ah);
    ((high << 10).wrapping_add(bl * al >> 6) as i32).wrapping_mul(sign)
}

/// Multiplies a 32-bit value by a 2.14 factor (only the low 16 bits of `b` are read) and shifts right by 14:
/// `(a_lo·|b| >> 14) + a_hi·|b|·4` (truncated; wraps), with the product sign.
/// Original: `f2dot14_mul` (0x9447e0, library_without_asserts, T2K util.c, cdecl).
pub(crate) unsafe extern "C" fn f2dot14_mul(a: i32, b: i32) -> i32 {
    let factor = b as i16;
    let sign = if (a < 0) != (factor < 0) { -1 } else { 1 };
    let (a, factor) = (a.unsigned_abs(), factor.unsigned_abs() as u32);
    let r = ((a & 0xffff) * factor >> 14).wrapping_add(((a >> 16) * factor).wrapping_mul(4));
    (r as i32).wrapping_mul(sign)
}

/// Divides two 16.16 numbers (truncated), with the quotient sign; a zero divisor faults, as the original's `div`.
/// Original: `fixed_div` (0x944840, library_without_asserts, T2K util.c, cdecl).
pub(crate) unsafe extern "C" fn fixed_div(a: i32, b: i32) -> i32 {
    let (sign, a, b) = sign_and_magnitudes(a, b);
    (unsigned_fixed_div(a, b) as i32).wrapping_mul(sign)
}

/// The length of the vector (x, y) in 16.16: with one component zero, the other's magnitude; else the first guess
/// `max + min/2` and three Newton steps `g = (g + x·(x/g) + y·(y/g) + 1) >> 1` with `fixed_mul` and `fixed_div`.
/// Original: `fixed_hypot` (0x9448b0, library_without_asserts, T2K util.c, cdecl).
pub(crate) unsafe extern "C" fn fixed_hypot(x: i32, y: i32) -> i32 {
    let (x, y) = (x.wrapping_abs(), y.wrapping_abs());
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }
    let mut g = if x > y { x.wrapping_add(y >> 1) } else { y.wrapping_add(x >> 1) };
    for _ in 0..3 {
        let sum = fixed_mul(x, fixed_div(x, g)).wrapping_add(fixed_mul(y, fixed_div(y, g)));
        g = g.wrapping_add(sum).wrapping_add(1) >> 1;
    }
    g
}

/// Comparers: value against value (pure); the sort side by side on the array.
pub(crate) unsafe extern "C" fn compare_sort_int16_array_ascending(values: usize, n: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32)>(%%I0%%);
    let bytes = if values != 0 && (2..=0x1000).contains(&n) { 2 * n as usize } else { 0 };
    let (_, _, same, _, _) = crate::comparison::side_by_side(&[(values, bytes)], || o(values, n), || sort_int16_array_ascending(values, n));
    SCOREBOARD.record(same, || format!("sort_int16_array_ascending(n = {}) differs", n));
}

unsafe fn compare_fixed_op(name: &str, original: FixedOpFn, copy: FixedOpFn, a: i32, b: i32) -> i32 {
    let r = original(a, b);
    let c = copy(a, b);
    SCOREBOARD.record(r == c, || format!("{}({:#x}, {:#x}) original={:#x} rust={:#x}", name, a, b, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_fixed_mul(a: i32, b: i32) -> i32 {
    compare_fixed_op("fixed_mul", original_fn::<FixedOpFn>(%%I1%%), fixed_mul, a, b)
}

pub(crate) unsafe extern "C" fn compare_f26dot6_mul(a: i32, b: i32) -> i32 {
    compare_fixed_op("f26dot6_mul", original_fn::<FixedOpFn>(%%I2%%), f26dot6_mul, a, b)
}

pub(crate) unsafe extern "C" fn compare_f2dot14_mul(a: i32, b: i32) -> i32 {
    compare_fixed_op("f2dot14_mul", original_fn::<FixedOpFn>(%%I3%%), f2dot14_mul, a, b)
}

pub(crate) unsafe extern "C" fn compare_fixed_div(a: i32, b: i32) -> i32 {
    compare_fixed_op("fixed_div", original_fn::<FixedOpFn>(%%I4%%), fixed_div, a, b)
}

pub(crate) unsafe extern "C" fn compare_fixed_hypot(x: i32, y: i32) -> i32 {
    compare_fixed_op("fixed_hypot", original_fn::<FixedOpFn>(%%I5%%), fixed_hypot, x, y)
}
