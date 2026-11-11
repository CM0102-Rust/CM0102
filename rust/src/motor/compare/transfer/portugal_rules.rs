use super::*;

// The comparers of the routines in ../transfer/portugal_rules.rs.

/// Comparer for `portugal_rules_init_transfer_windows`.
pub(crate) unsafe extern "thiscall" fn compare_portugal_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PORTUGAL_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("portugal_rules_init_transfer_windows", this, || o(this), |s| portugal_rules_init_transfer_windows(s));
}
