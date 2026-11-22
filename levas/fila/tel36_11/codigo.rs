// ---------- batch %%LEVA%%: the tactics screens' 48-bit mask and bit index (tactics_screens.cpp) ----------

/// Rule: a tactics mask with a single bit among the low 48 gives its index (0..47); anything else -1.
///
/// `mask64_to_bit_index(low, high)`: the mask high:low compared against 1 << k for k = 0..47 in order.
/// Original: `mask64_to_bit_index` (0x8adbb0, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn mask64_to_bit_index(low: u32, high: u32) -> i8 {
    let mask = (high as u64) << 32 | low as u64;
    if mask.count_ones() == 1 && mask.trailing_zeros() < 0x30 {
        covered!("tel36_11 mask bit index");
        return mask.trailing_zeros() as i8;
    }
    -1
}

/// Rule: a tactics bit index gives its mask: 1 << k for 0..47, 0 otherwise.
///
/// `bit_index_to_mask64(k)`: the 64-bit mask in edx:eax for the char `k`.
/// Original: `bit_index_to_mask64` (0x8ade80, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn bit_index_to_mask64(k: u32) -> u64 {
    match k as u8 {
        b @ 0..=0x2f => 1u64 << b,
        _ => 0,
    }
}

/// Comparer for `mask64_to_bit_index`.
pub(crate) unsafe extern "C" fn compare_mask64_to_bit_index(low: u32, high: u32) -> i8 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32) -> i8>(%%I0%%)(low, high);
    let n = mask64_to_bit_index(low, high);
    SCOREBOARD.record(r == n, || format!("mask64_to_bit_index({:#x}, {:#x}): original={} rust={}", low, high, r, n));
    r
}

/// Comparer for `bit_index_to_mask64`.
pub(crate) unsafe extern "C" fn compare_bit_index_to_mask64(k: u32) -> u64 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u64>(%%I1%%)(k);
    let n = bit_index_to_mask64(k);
    SCOREBOARD.record(r == n, || format!("bit_index_to_mask64({}): original={:#x} rust={:#x}", k as u8, r, n));
    r
}
