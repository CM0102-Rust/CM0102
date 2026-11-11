use super::*;

// The comparers of the routines in ../transfer/argentina_rules.rs.

/// Comparer for `argentina_rules_init_transfer_windows`.
pub(crate) unsafe extern "thiscall" fn compare_argentina_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ARGENTINA_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("argentina_rules_init_transfer_windows", this, || o(this), |s| argentina_rules_init_transfer_windows(s));
}

