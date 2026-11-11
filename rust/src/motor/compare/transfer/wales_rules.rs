use super::*;

// The comparers of the routines in ../transfer/wales_rules.rs.

/// Comparer for `construct_wales_rules`.
pub(crate) unsafe extern "thiscall" fn compare_construct_wales_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_WALES_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_wales_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_wales_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
