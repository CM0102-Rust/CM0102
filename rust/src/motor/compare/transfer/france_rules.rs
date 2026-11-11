use super::*;

// The comparers of the routines in ../transfer/france_rules.rs.

/// Comparer for `france_rules_init_transfer_windows`.
pub(crate) unsafe extern "thiscall" fn compare_france_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FRANCE_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("france_rules_init_transfer_windows", this, || o(this), |s| france_rules_init_transfer_windows(s));
}
