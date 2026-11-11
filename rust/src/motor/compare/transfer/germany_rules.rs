use super::*;

// The comparers of the routines in ../transfer/germany_rules.rs.

/// Comparer for `germany_rules_init_transfer_windows`.
pub(crate) unsafe extern "thiscall" fn compare_germany_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GERMANY_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("germany_rules_init_transfer_windows", this, || o(this), |s| germany_rules_init_transfer_windows(s));
}
