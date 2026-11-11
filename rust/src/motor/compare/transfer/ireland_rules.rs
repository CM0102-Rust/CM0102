use super::*;

// The comparers of the routines in ../transfer/ireland_rules.rs.

/// Comparer for `construct_ireland_rules`.
pub(crate) unsafe extern "thiscall" fn compare_construct_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_IRELAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_ireland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_ireland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Comparer for `ireland_rules_can_loan_player`.
pub(crate) unsafe extern "thiscall" fn compare_ireland_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    england_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_IRELAND_RULES_CAN_LOAN_PLAYER), ireland_rules_can_loan_player, "ireland_rules_can_loan_player", this, club, player, msg, reason)
}
