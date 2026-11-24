/// Orders two 13-byte next-sale percentage entries (`next_sale_percentages()`) for qsort and bsearch: by the person id
/// at +9, then by the club id at +1 (each as the difference of the two ints, wrapping).
/// Original: `transfer_manager_compare_next_sale_entries` (0x8e54d0, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_next_sale_entries(a: usize, b: usize) -> i32 {
    let d = read_i32(a + 9).wrapping_sub(read_i32(b + 9)); // sx02b_3: +9, the person id
    if d != 0 {
        return d;
    }
    read_i32(a + 1).wrapping_sub(read_i32(b + 1)) // +1: the club id
}

/// Comparer for `compare_next_sale_entries`: both run side by side (they only read).
pub(crate) unsafe extern "C" fn compare_compare_next_sale_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = compare_next_sale_entries(a, b);
    SCOREBOARD.record(r == n, || format!("compare_next_sale_entries original={} rust={}", r, n));
    r
}

/// Orders two transfer window dates through pointers to them (the table `transfer_windows()`): by the month byte
/// (`transfer_window::MONTH`), then the day byte (+3), each as the difference of the two signed bytes.
/// Original: `transfer_manager_compare_window_dates` (0x8e5590, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_window_dates(a: usize, b: usize) -> i32 {
    let (a, b) = (read_usize(a), read_usize(b));
    let (ma, mb) = (read_i8(a + transfer_window::MONTH), read_i8(b + transfer_window::MONTH));
    if ma != mb {
        return ma as i32 - mb as i32; // sx02b_3: by month
    }
    read_i8(a + 3) as i32 - read_i8(b + 3) as i32 // +3: the day
}

/// Comparer for `compare_window_dates`: both run side by side (they only read).
pub(crate) unsafe extern "C" fn compare_compare_window_dates(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = compare_window_dates(a, b);
    SCOREBOARD.record(r == n, || format!("compare_window_dates original={} rust={}", r, n));
    r
}

/// Orders two 14-byte work permit entries (`work_permits()`) by the person id at +0, with the empty key −1 last: both
/// −1, 0; only the first, 1; only the second, −1; otherwise the difference (wrapping).
/// Original: `transfer_manager_compare_work_permit_entries` (0x8e55c0, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_work_permit_entries(a: usize, b: usize) -> i32 {
    let (ka, kb) = (read_i32(a), read_i32(b));
    match (ka == -1, kb == -1) {
        (true, true) => 0,
        (true, false) => 1, // sx02b_3: the empty key last
        (false, true) => -1,
        (false, false) => ka.wrapping_sub(kb),
    }
}

/// Comparer for `compare_work_permit_entries`: both run side by side (they only read).
pub(crate) unsafe extern "C" fn compare_compare_work_permit_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = compare_work_permit_entries(a, b);
    SCOREBOARD.record(r == n, || format!("compare_work_permit_entries original={} rust={}", r, n));
    r
}
