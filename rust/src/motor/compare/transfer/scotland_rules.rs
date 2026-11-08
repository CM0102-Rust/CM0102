use super::*;

// The comparers of the routines in ../transfer/scotland_rules.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer/scotland_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `scotland_rules_can_loan_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_scotland_rules_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    scotland_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_SCOTLAND_RULES_CAN_LOAN_PLAYER), scotland_rules_can_loan_player, "scotland_rules_can_loan_player", this, clube, jogador, msg, motivo)
}

/// Comparer for `scotland_rules_get_loan_period` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nas quatro saídas e em [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_scotland_rules_get_loan_period(this: usize, a1: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_SCOTLAND_RULES_GET_LOAN_PERIOD);
    let regioes = [(ini_mes as usize, 1usize), (ini_ano as usize, 2), (fim_mes as usize, 1), (fim_ano as usize, 2), (G_CONTRACT_LOOKUP_RESULT, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, a1, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador), || scotland_rules_get_loan_period(this, a1, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador));
    PLACAR.registrar(igual, || format!("scotland_rules_get_loan_period(regras {:#x}, jogador {:#x}) original={:x?} rust={:x?}", this, jogador, mo, mr));
}

/// Comparer for `scotland_rules_can_loan_player_661770` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_scotland_rules_can_loan_player_661770(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    scotland_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_SCOTLAND_RULES_CAN_LOAN_PLAYER_661770), scotland_rules_can_loan_player_661770, "scotland_rules_can_loan_player_661770", this, clube, jogador, msg, motivo)
}

/// Comparer for `scotland_rules_can_loan_player_63c3d0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_scotland_rules_can_loan_player_63c3d0(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    scotland_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_SCOTLAND_RULES_CAN_LOAN_PLAYER_63C3D0), scotland_rules_can_loan_player_63c3d0, "scotland_rules_can_loan_player_63c3d0", this, clube, jogador, msg, motivo)
}

/// Comparer for `construct_scotland_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_scotland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_SCOTLAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_scotland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_scotland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
