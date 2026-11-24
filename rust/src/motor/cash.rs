use super::*;

/// CRT fread of one record of tam bytes; returns records read.
pub(crate) unsafe fn crt_fread_one(buf: usize, size: u32, f: usize) -> u32 {
    exe::crt_fread(buf, size, 1, f)
}

/// CRT fclose.
pub(crate) unsafe fn cash_crt_fclose(f: usize) {
    exe::crt_fclose(f);
}

/// Error box of formation.cpp at the given line.
pub(crate) unsafe fn formation_error_box(row: u32) {
    show_error_at(0x9cc3dc, row);
}

/// Byte swap of a dword.
pub(crate) fn swap_bytes_u32(v: u32) -> u32 {
    v.swap_bytes()
}

/// Failed read: box unless quiet, closes the file and returns 0.
pub(crate) unsafe fn fail_read(quiet: u32, row: u32, f: usize) -> u32 {
    if quiet == 0 {
        formation_error_box(row);
    }
    cash_crt_fclose(f);
    0
}

/// swaps the two bytes of each word of an array
/// Original: `swap_bytes_16_array` (0x43f2d0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn swap_bytes_16_array(p: usize, count: u32) {
    for k in 0..count as usize {
        let a = (p + 2 * k) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).swap_bytes());
    }
}

/// reverses the four bytes of each dword of an array
/// Original: `swap_bytes_32_array` (0x43f300, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn swap_bytes_32_array(p: usize, count: u32) {
    for k in 0..count as usize {
        write_u32(p + 4 * k, swap_bytes_u32(read_u32(p + 4 * k)));
    }
}

/// The step a money amount is rounded to on screen. Flag 4 (wages per year, transfer fees) uses the coarse ladder: up
/// to 10,000 in steps of 1,000, 30,000 of 2,000, 100,000 of 5,000, 250,000 of 10,000, 1M of 25,000, 5M of 100,000, 20M
/// of 250,000, 50M of 500,000, above that of 1M. Flag 8 (wages per week or month) the fine one, from steps of 1 up to
/// 10 to steps of 1M above 50M (with a jump from 500 to 5,000 at 100,000). Thresholds and steps are times the currency
/// scale and the step is truncated (`_ftol`); flag 4 wins over 8, and without either the step is 1,000.
/// Original: `cash_get_rounding_step` (0x4426e0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_get_rounding_step(amount: i64, flags: u32) -> i64 {
    let ladder: &[(f64, f64)] = if flags & 4 != 0 {
        &ROUNDING_FLAG_4
    } else if flags & 8 != 0 {
        &ROUNDING_FLAG_8
    } else {
        covered!("sx05_1 rounding step default");
        return 1000;
    };
    let scale = read_f64(CURRENCY_SCALE);
    // ponytail: the original compares the exact 64-bit integer (fild); amounts beyond 2^53 would round here
    let x = amount as f64;
    for &(threshold, step) in ladder {
        if !(x > scale * threshold) {
            covered!("sx05_1 rounding step ladder");
            return (scale * step) as i64;
        }
    }
    (scale * 1000000.0) as i64
}

/// The amount a cash value shows in the display currency: the stored value times the exchange rate, truncated, then
/// (unless flag 0x20 asks for the exact value) rounded to the nearest step of `cash_get_rounding_step` with the same
/// flags (half a step up, then down to the step, in signed 64-bit division).
/// Original: `Cash::get_display_amount` (0x43f460, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_get_display_amount(cash: usize, flags: u32) -> i64 {
    let amount = (read_i32(cash) as f64 * read_f64(CURRENCY_RATE)) as i64;
    if flags & 0x20 != 0 {
        return amount;
    }
    covered!("sx05_1 display amount rounded");
    let step = cash_get_rounding_step(amount, flags);
    (step / 2 + amount) / step * step
}

/// Writes a 64-bit money amount as text into `buf`: the currency prefix, the amount and the suffix. With flag 2 the
/// whole amount with the thousands separator (none in a four-digit amount when the currency says so); without it,
/// amounts of four digits or more are abbreviated to thousands, millions, billions or thousands of billions with the
/// unit text: the leading one to three digits, then up to two decimals after the decimal separator with trailing zeros
/// dropped. A currency without a billions unit writes billions as whole millions with the separator. Zero and negative
/// amounts are written "0"; spaces in the result become '|' (the screens' non-breaking space). Returns 1, or 0 with
/// the assertion box (line 0x2f3) when the buffer is null or shorter than 25.
/// Defect of the original: a single-digit amount is written "0" (the jump table starts at two digits and one digit
/// falls into the default case), and the buffer size is only checked against 25.
/// Original: `cash_format_money` (0x441c10, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_format_money(buf: usize, size: u32, amount: i64, flags: u32) -> u32 {
    if buf == 0 || (size as u16 as i16) < 0x19 {
        show_error_at(CASH_CPP, 0x2f3);
        return 0;
    }
    let (digits, n) = money_digits(amount);
    let d = &digits[..n];
    let mut out = Vec::with_capacity(0x60);
    push_c_string(&mut out, MONEY_VIEW_CURRENCY_SYMBOL);
    if n < 2 {
        covered!("sx05_1 money zero");
        out.push(b'0');
        push_c_string(&mut out, CURRENCY_SUFFIX);
    } else if n <= 3 || flags & 2 != 0 {
        covered!("sx05_1 money full");
        if n == 4 && read_u32(CURRENCY_NO_SEPARATOR_UNDER_10000) != 0 {
            out.extend_from_slice(d);
        } else {
            push_grouped(&mut out, d);
        }
        push_c_string(&mut out, CURRENCY_SUFFIX);
    } else if (10..=12).contains(&n) && read_u8(CURRENCY_UNIT_BILLIONS) == 0 {
        covered!("sx05_1 money whole millions");
        push_grouped(&mut out, &d[..n - 6]);
        push_c_string(&mut out, CURRENCY_UNIT_MILLIONS);
    } else {
        covered!("sx05_1 money abbreviated");
        let unit = [CURRENCY_UNIT_THOUSANDS, CURRENCY_UNIT_MILLIONS, CURRENCY_UNIT_BILLIONS, CURRENCY_UNIT_TRILLIONS][(n - 1) / 3 - 1];
        let whole = (n - 1) % 3 + 1;
        out.extend_from_slice(&d[..whole]);
        let decimals = if d[whole + 1] != b'0' { 2 } else if d[whole] != b'0' { 1 } else { 0 };
        if decimals > 0 {
            out.push(read_u8(CURRENCY_DECIMAL_SEPARATOR));
            out.extend_from_slice(&d[whole..whole + decimals]);
        }
        push_c_string(&mut out, unit);
    }
    out.push(0);
    core::ptr::copy_nonoverlapping(out.as_ptr(), buf as *mut u8, out.len());
    // the spaces up to the first terminator (a separator character 0 ends the text early)
    let mut k = 0;
    while read_u8(buf + k) != 0 {
        if read_u8(buf + k) == b' ' {
            write_u8(buf + k, b'|');
        }
        k += 1;
    }
    1
}

/// Comparer for `cash_get_rounding_step`: the step side by side.
pub(crate) unsafe extern "C" fn compare_cash_get_rounding_step(amount: i64, flags: u32) -> i64 {
    let r = original_fn::<unsafe extern "C" fn(i64, u32) -> i64>(IDX_CASH_GET_ROUNDING_STEP)(amount, flags);
    let v = cash_get_rounding_step(amount, flags);
    SCOREBOARD.record(r == v, || format!("cash_get_rounding_step({}, {:#x}) original={} rust={}", amount, flags, r, v));
    r
}

/// Comparer for `cash_format_money`: the text side by side (the first 0x80 bytes of the buffer, at most its size); a
/// bad buffer (the assertion box) only runs the original.
pub(crate) unsafe extern "C" fn compare_cash_format_money(buf: usize, size: u32, amount: i64, flags: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, i64, u32) -> u32>(IDX_CASH_FORMAT_MONEY);
    if buf == 0 || (size as u16 as i16) < 0x19 {
        return o(buf, size, amount, flags);
    }
    let len = (size as u16 as usize).min(0x80);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(buf, len)], || o(buf, size, amount, flags), || cash_format_money(buf, size, amount, flags));
    SCOREBOARD.record(equal && r == v, || format!("cash_format_money({}, {:#x}) original={:?} rust={:?}", amount, flags, first_differences(&a, &b), (r, v)));
    r
}

/// The decimal digits of `amount`, most significant first, at most the lowest 15; none for zero or less. As the
/// original builds them: digit by digit from the units until the digits written add up to the amount (so the top
/// zero digits never appear).
pub(crate) fn money_digits(amount: i64) -> ([u8; 16], usize) {
    let (mut digits, mut n, mut sum, mut power) = ([0u8; 16], 0usize, 0i64, 1i64);
    while sum < amount && n < 15 {
        let digit = amount % (power * 10) / power;
        digits.copy_within(0..n, 1);
        digits[0] = b'0' + digit as u8;
        sum += digit * power;
        power *= 10;
        n += 1;
    }
    (digits, n)
}

/// The digits with the thousands separator before each group of three from the right.
pub(crate) unsafe fn push_grouped(out: &mut Vec<u8>, digits: &[u8]) {
    for (k, &d) in digits.iter().enumerate() {
        if k > 0 && (digits.len() - k) % 3 == 0 {
            out.push(read_u8(CURRENCY_THOUSANDS_SEPARATOR));
        }
        out.push(d);
    }
}

/// The C string at `a`, without its terminator.
pub(crate) unsafe fn push_c_string(out: &mut Vec<u8>, a: usize) {
    out.extend_from_slice(core::slice::from_raw_parts(a as *const u8, c_string_len(a)));
}

/// The display currency's scale factor (f64): the rounding thresholds and steps are in pounds times this.
pub(crate) const CURRENCY_SCALE: usize = 0xad9cc0;

/// The display currency's exchange rate (f64): a stored amount times this is the amount shown.
pub(crate) const CURRENCY_RATE: usize = 0xad9cb8;

/// The thousands separator character of the display currency (byte).
pub(crate) const CURRENCY_THOUSANDS_SEPARATOR: usize = 0xad9ca2;

/// Nonzero when a four-digit amount is written without the thousands separator (dword).
pub(crate) const CURRENCY_NO_SEPARATOR_UNDER_10000: usize = 0xad9ca4;

/// The decimal separator character of the abbreviated amounts (byte).
pub(crate) const CURRENCY_DECIMAL_SEPARATOR: usize = 0xad9cc8;

/// The text after a full amount (the currency suffix).
pub(crate) const CURRENCY_SUFFIX: usize = 0xad9c70;

/// The unit texts of the abbreviated amounts: thousands, millions, billions (empty in some currencies) and the
/// thousands of billions.
pub(crate) const CURRENCY_UNIT_THOUSANDS: usize = 0xad9d00;

pub(crate) const CURRENCY_UNIT_MILLIONS: usize = 0xad9d68;

pub(crate) const CURRENCY_UNIT_BILLIONS: usize = 0xad9ccc;

pub(crate) const CURRENCY_UNIT_TRILLIONS: usize = 0xad9d34;

/// cash.cpp, for the assertion box.
pub(crate) const CASH_CPP: usize = 0x98b1fc;

/// The rounding ladders of `cash_get_rounding_step`: (threshold, step) in pounds, both times the currency scale; the
/// first threshold the amount does not exceed gives the step, and above the last one the step is 1,000,000.
pub(crate) const ROUNDING_FLAG_4: [(f64, f64); 8] = [
    (10000.0, 1000.0), (30000.0, 2000.0), (100000.0, 5000.0), (250000.0, 10000.0),
    (1000000.0, 25000.0), (5000000.0, 100000.0), (20000000.0, 250000.0), (50000000.0, 500000.0),
];

pub(crate) const ROUNDING_FLAG_8: [(f64, f64); 13] = [
    (10.0, 1.0), (100.0, 5.0), (250.0, 10.0), (1000.0, 25.0), (5000.0, 100.0), (20000.0, 250.0), (50000.0, 500.0),
    (100000.0, 5000.0), (250000.0, 10000.0), (1000000.0, 25000.0), (5000000.0, 100000.0), (20000000.0, 250000.0),
    (50000000.0, 500000.0),
];
