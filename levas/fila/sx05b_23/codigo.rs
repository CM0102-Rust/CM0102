// ---------- sx05b_23: the display currency's id and the 64-bit byte swap (cash.cpp) ----------

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
    let r = original_fn::<unsafe extern "C" fn() -> u8>(%%I0%%)();
    let v = cash_get_currency();
    SCOREBOARD.record(r == v, || format!("cash_get_currency original={} rust={}", r, v));
    r
}

/// Comparer for `swap_bytes_64_array`: side by side over the buffer.
pub(crate) unsafe extern "C" fn compare_swap_bytes_64_array(buf: usize, count: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(%%I1%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(buf, 8 * count as usize)], || o(buf, count), || swap_bytes_64_array(buf, count));
    SCOREBOARD.record(equal, || format!("swap_bytes_64_array({} words) {:?}", count, first_differences(&a, &b)));
}
