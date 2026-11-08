use super::*;

// The comparers of the routines in ../transfer/spain_rules.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer/spain_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spain_rules_init_transfer_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPAIN_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("spain_rules_init_transfer_windows", this, || o(this), |s| spain_rules_init_transfer_windows(s));
}

/// Comparer for `spain_rules_init_transfer_windows_6613f0` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows_6613f0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPAIN_RULES_INIT_TRANSFER_WINDOWS_6613F0);
    compare_transfer_rules_ctor("spain_rules_init_transfer_windows_6613f0", this, || o(this), |s| spain_rules_init_transfer_windows_6613f0(s));
}

/// Comparer for `spain_rules_init_transfer_windows_5c15e0` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows_5c15e0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPAIN_RULES_INIT_TRANSFER_WINDOWS_5C15E0);
    compare_transfer_rules_ctor("spain_rules_init_transfer_windows_5c15e0", this, || o(this), |s| spain_rules_init_transfer_windows_5c15e0(s));
}

/// Comparer for `spain_rules_can_loan_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spain_rules_can_loan_player(this: usize, clube: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    scotland_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_SPAIN_RULES_CAN_LOAN_PLAYER), spain_rules_can_loan_player, "spain_rules_can_loan_player", this, clube, player, msg, reason)
}
