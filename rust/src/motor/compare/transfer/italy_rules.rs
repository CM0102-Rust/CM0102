use super::*;

// The comparers of the routines in ../transfer/italy_rules.rs.

/// Comparer for `italy_rules_can_loan_player`.
pub(crate) unsafe extern "thiscall" fn compare_italy_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    england_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_ITALY_RULES_CAN_LOAN_PLAYER), italy_rules_can_loan_player, "italy_rules_can_loan_player", this, club, player, msg, reason)
}

/// Comparer for `italy_rules_init_transfer_windows`.
pub(crate) unsafe extern "thiscall" fn compare_italy_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITALY_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("italy_rules_init_transfer_windows", this, || o(this), |s| italy_rules_init_transfer_windows(s));
}
