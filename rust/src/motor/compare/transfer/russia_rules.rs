use super::*;

// The comparers of the routines in ../transfer/russia_rules.rs.

/// Comparer for `russia_rules_init_transfer_windows`.
pub(crate) unsafe extern "thiscall" fn compare_russia_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RUSSIA_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("russia_rules_init_transfer_windows", this, || o(this), |s| russia_rules_init_transfer_windows(s));
}
