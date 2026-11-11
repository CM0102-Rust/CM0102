use super::*;

// The comparers of the routines in ../transfer/greece_rules.rs.

/// Comparer for `greece_rules_init_transfer_windows`.
pub(crate) unsafe extern "thiscall" fn compare_greece_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GREECE_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("greece_rules_init_transfer_windows", this, || o(this), |s| greece_rules_init_transfer_windows(s));
}

