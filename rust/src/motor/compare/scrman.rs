use super::*;

// The comparers of the routines in ../scrman.rs.

/// Comparer for `is_forward_unavailable`.
pub(crate) unsafe extern "thiscall" fn compare_is_forward_unavailable(manager: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IS_FORWARD_UNAVAILABLE)(manager);
    let n = is_forward_unavailable(manager);
    SCOREBOARD.record(r == n, || format!("is_forward_unavailable original={} rust={}", r, n));
    r
}
