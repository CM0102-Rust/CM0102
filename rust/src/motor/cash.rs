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

/// Sets the display currency `id` (0 to 25, signed): its exchange rate (`CURRENCY_RATE`), round rate
/// (`CURRENCY_ROUND_RATE`) and scale (`CURRENCY_SCALE`), its prefix (`CURRENCY_PREFIX`) and suffix (`CURRENCY_SUFFIX`),
/// the unit texts (`CURRENCY_UNIT_THOUSANDS`, `_MILLIONS`, `_BILLIONS`, `_TRILLIONS`), the separators and the
/// no-separator flag, and the id itself (`CURRENCY_ID`); 1. Any other id: the "Error" box (line 0x265) and 0, nothing
/// changed.
/// Original: `cash_set_currency` (0x43f7f0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_set_currency(id: u32) -> u32 {
    let k = id as u8 as i8 as i32;
    if !(0..26).contains(&k) {
        show_error_at(CASH_CPP, 0x265);
        return 0;
    }
    let c = &CURRENCIES[k as usize];
    covered!("sx05b_21 currency set");
    write_u64(CURRENCY_RATE, c.rate);
    write_u64(CURRENCY_ROUND_RATE, c.round_rate);
    write_u64(CURRENCY_SCALE, c.scale);
    tel26_copy_text(CURRENCY_PREFIX as *mut u8, c.prefix as *const u8);
    tel26_copy_text(CURRENCY_SUFFIX as *mut u8, c.suffix as *const u8);
    for (dst, &src) in [CURRENCY_UNIT_THOUSANDS, CURRENCY_UNIT_MILLIONS, CURRENCY_UNIT_BILLIONS, CURRENCY_UNIT_TRILLIONS].iter().zip(c.units.iter()) {
        tel26_copy_text(*dst as *mut u8, src as *const u8);
    }
    write_u8(CURRENCY_THOUSANDS_SEPARATOR, c.thousands);
    write_u8(CURRENCY_DECIMAL_SEPARATOR, c.decimal);
    write_u32(CURRENCY_NO_SEPARATOR_UNDER_10000, c.no_separator_under_10000);
    write_u8(CURRENCY_ID, id as u8);
    1
}

/// Comparer for `cash_set_currency`: side by side over the currency globals (0xad9c70 to the round rate, and the id).
pub(crate) unsafe extern "C" fn compare_cash_set_currency(id: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_CASH_SET_CURRENCY);
    let regions = [(CURRENCY_SUFFIX, CURRENCY_ROUND_RATE + 8 - CURRENCY_SUFFIX), (CURRENCY_ID, 1)];
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&regions, || o(id), || cash_set_currency(id));
    SCOREBOARD.record(equal && r == v, || format!("cash_set_currency({}) original={} rust={} {:?}", id as u8 as i8, r, v, first_differences(&a, &b)));
    r
}

/// Raises a cash value one step: its display amount (`cash_get_display_amount` with `flags`: rounded to the nearest
/// step unless flag 0x20) plus the rounding step of the next amount (`cash_get_rounding_step`(amount + 1, flags)), back
/// in pounds (`cash_from_display`); stored and returned.
/// Original: `Cash::step_up` (0x43f4e0, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_step_up(cash: usize, flags: u32) -> u32 {
    let amount = cash_get_display_amount(cash, flags);
    let v = cash_from_display(amount.wrapping_add(cash_get_rounding_step(amount.wrapping_add(1), flags)));
    covered!("sx05b_22 cash stepped up");
    write_u32(cash, v);
    v
}

/// Lowers a cash value one step: its display amount (as `cash_step_up`) minus the rounding step of that amount, back
/// in pounds; stored and returned.
/// Original: `Cash::step_down` (0x43f580, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_step_down(cash: usize, flags: u32) -> u32 {
    let amount = cash_get_display_amount(cash, flags);
    let v = cash_from_display(amount.wrapping_sub(cash_get_rounding_step(amount, flags)));
    covered!("sx05b_22 cash stepped down");
    write_u32(cash, v);
    v
}

/// Comparer for `cash_step_up`: side by side over the cash value and the result.
pub(crate) unsafe extern "thiscall" fn compare_cash_step_up(cash: usize, flags: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_CASH_STEP_UP);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, flags), || cash_step_up(cash, flags));
    SCOREBOARD.record(equal && r == v, || format!("cash_step_up(flags {:#x}) original={} rust={} {:?}", flags, r, v, first_differences(&a, &b)));
    r
}

/// Comparer for `cash_step_down`: side by side over the cash value and the result.
pub(crate) unsafe extern "thiscall" fn compare_cash_step_down(cash: usize, flags: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_CASH_STEP_DOWN);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, flags), || cash_step_down(cash, flags));
    SCOREBOARD.record(equal && r == v, || format!("cash_step_down(flags {:#x}) original={} rust={} {:?}", flags, r, v, first_differences(&a, &b)));
    r
}

/// The id of the display currency (`CURRENCY_ID`).
/// Original: `cash_get_currency` (0x4415e0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_get_currency() -> u8 {
    covered!("sx05b_23 currency read");
    read_u8(CURRENCY_ID)
}

/// Reverses the byte order of each of the `count` 64-bit words of `buf`, in place (the original builds each swap from
/// 64-bit shifts, `__allshl` and `__allshr`: the same as a full byte reversal). Used by `Formation::load_from_disk` for
/// tactics files of the other byte order.
/// Original: `swap_bytes_64_array` (0x43f350, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn swap_bytes_64_array(buf: usize, count: u32) {
    for k in 0..count as usize {
        let p = buf + 8 * k;
        write_u64(p, read_u64(p).swap_bytes());
    }
}

/// Comparer for `cash_get_currency`: the value side by side.
pub(crate) unsafe extern "C" fn compare_cash_get_currency() -> u8 {
    let r = original_fn::<unsafe extern "C" fn() -> u8>(IDX_CASH_GET_CURRENCY)();
    let v = cash_get_currency();
    SCOREBOARD.record(r == v, || format!("cash_get_currency original={} rust={}", r, v));
    r
}

/// Comparer for `swap_bytes_64_array`: side by side over the buffer.
pub(crate) unsafe extern "C" fn compare_swap_bytes_64_array(buf: usize, count: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_SWAP_BYTES_64_ARRAY);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(buf, 8 * count as usize)], || o(buf, count), || swap_bytes_64_array(buf, count));
    SCOREBOARD.record(equal, || format!("swap_bytes_64_array({} words) {:?}", count, first_differences(&a, &b)));
}

/// A display currency amount back to a stored cash value: divided by the exchange rate (`CURRENCY_RATE`), truncated to
/// 64 bits (`_ftol`) and kept in its low dword.
pub(crate) unsafe fn cash_from_display(amount: i64) -> u32 {
    (amount as f64 / read_f64(CURRENCY_RATE)) as i64 as u32
}

/// The display currency's prefix text (before the amount) and the round exchange rate (f64) the preset amounts use;
/// the id of the display currency (byte).
pub(crate) const CURRENCY_PREFIX: usize = 0xad9ca8;

pub(crate) const CURRENCY_ROUND_RATE: usize = 0xad9da0;

pub(crate) const CURRENCY_ID: usize = 0x98b140;

/// The 26 currencies by id, as the original's jump table sets them.
pub(crate) const CURRENCIES: [Currency; 26] = [
    Currency { rate: 0x3ff0000000000000, round_rate: 0x3ff0000000000000, scale: 0x3ff0000000000000, prefix: 0x98b254, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 0 pound
    Currency { rate: 0x3ff7d70a3d70a3d7, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 1 peso
    Currency { rate: 0x4003eb851eb851ec, round_rate: 0x4004000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 2 Australian dollar
    Currency { rate: 0x405010a3d70a3d71, round_rate: 0x4049000000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b3e8, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3e0, 0x98b3d8, 0x98b3d0, 0x98b3c8] }, // 3 Belgian franc
    Currency { rate: 0x4005d70a3d70a3d7, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3c4, suffix: 0x98b3c0, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 4 real
    Currency { rate: 0x40283851eb851eb8, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b3bc, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b3b8, 0x98b3b0, 0x98b3ac, 0x98b3a8] }, // 5 kuna
    Currency { rate: 0x4027c28f5c28f5c3, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b3a4, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b39c, 0x98b394, 0x98b38c, 0x98b384] }, // 6 Danish krone
    Currency { rate: 0x3ff8cccccccccccd, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b380, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 7 euro
    Currency { rate: 0x4022f0a3d70a3d71, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b37c, thousands: 0x2c, no_separator_under_10000: 0x1, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 8 markka
    Currency { rate: 0x4024e66666666666, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b378, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b374, 0x98b370, 0x98b3e8, 0x98b36c] }, // 9 French franc
    Currency { rate: 0x4008e147ae147ae1, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b368, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b360, 0x98b358, 0x98b350, 0x98b348] }, // 10 Deutschmark
    Currency { rate: 0x400c147ae147ae14, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0x98b344, suffix: 0x98b340, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 11 guilder
    Currency { rate: 0x4080b70a3d70a3d7, round_rate: 0x4081300000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b33c, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b334, 0x98b330, 0x98b3f0, 0x98b3ec] }, // 12 drachma
    Currency { rate: 0x3ff4000000000000, round_rate: 0x3ff0000000000000, scale: 0x3ff0000000000000, prefix: 0x98b328, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b324, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 13 Irish pound
    Currency { rate: 0x40a817dc28f5c28f, round_rate: 0x40a7700000000000, scale: 0x408f400000000000, prefix: 0x98b31c, suffix: 0xad9c64, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3f8, 0x98b318, 0x98b3f4, 0x98b3ec] }, // 14 lira
    Currency { rate: 0x4063be147ae147ae, round_rate: 0x4062c00000000000, scale: 0x4059000000000000, prefix: 0x98b2dc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 15 yen
    Currency { rate: 0x402a28f5c28f5c29, round_rate: 0x402e000000000000, scale: 0x4024000000000000, prefix: 0x98b2d8, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3b8, 0x98b2d4, 0x98b3ac, 0x98b3a8] }, // 16 Norwegian krone
    Currency { rate: 0x401a5c28f5c28f5c, round_rate: 0x4014000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b2d0, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b2c8, 0x98b2c0, 0x98b3ac, 0x98b3a8] }, // 17 zloty
    Currency { rate: 0x3fd460807357e671, round_rate: 0x3fd0000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b2bc, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b2b8, 0x98b2b4, 0x98b2b0, 0x98b2ac] }, // 18 escudo
    Currency { rate: 0x4044dae147ae147b, round_rate: 0x4049000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b2a8, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b2a0, 0x98b294, 0x98b3ac, 0x98b3a8] }, // 19 ruble
    Currency { rate: 0x4070887ae147ae14, round_rate: 0x406f400000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b28c, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b284, 0x98b27c, 0xad9c64, 0x98b274] }, // 20 peseta
    Currency { rate: 0x4030000000000000, round_rate: 0x402e000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b270, thousands: 0x20, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b268, 0x98b260, 0x98b258, 0x98b268] }, // 21 Swedish krona
    Currency { rate: 0x4003ae147ae147ae, round_rate: 0x4004000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b24c, thousands: 0x2c, no_separator_under_10000: 0x1, decimal: 0x2e, units: [0x98b3b8, 0x98b244, 0x98b3ac, 0x98b3a8] }, // 22 Swiss franc
    Currency { rate: 0x412c77273d70a3d7, round_rate: 0x412e848000000000, scale: 0x412e848000000000, prefix: 0xad9c64, suffix: 0x98b240, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3b8, 0x98b238, 0x98b230, 0x98b224] }, // 23 Turkish lira
    Currency { rate: 0x3ff9eb851eb851ec, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 24 US dollar
    Currency { rate: 0x409dc00000000000, round_rate: 0x409f400000000000, scale: 0x409f400000000000, prefix: 0xad9c64, suffix: 0x98b310, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b308, 0x98b2f8, 0x98b2e8, 0x98b2e0] }, // 25 won
];
