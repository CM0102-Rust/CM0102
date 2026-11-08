use super::*;

// The comparers of the routines in ../transfer/england_rules.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer/england_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_england_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_england_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_ENGLAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_england_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_england_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Comparer for `construct_usa_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_usa_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_USA_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_usa_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_usa_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Comparer for `australia_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_australia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_AUSTRALIA_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("australia_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        australia_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Comparer for `england_rules_can_loan_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_england_rules_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    england_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_ENGLAND_RULES_CAN_LOAN_PLAYER), england_rules_can_loan_player, "england_rules_can_loan_player", this, clube, jogador, msg, motivo)
}

/// Comparador: lado a lado no byte da conta do clube da oferta.
pub(crate) unsafe extern "thiscall" fn comparar_record_loan(this: usize, oferta: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RECORD_LOAN);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(oferta + 8).wrapping_mul(club::SIZE));
    let regioes = [(season_loans_entry(clube), 1usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || record_loan(this, oferta));
    PLACAR.registrar(igual, || format!("record_loan(oferta {:#x}) original={:x?} rust={:x?}", oferta, mo, mr));
}

/// Comparer for `england_rules_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na tabela das contas inteira ([0xae235c] clubes × 16 bytes) só no dia em que ela muda (20
/// de junho); nos outros dias as duas rodam sem gravar nada e só a original fica valendo.
pub(crate) unsafe extern "thiscall" fn comparar_england_rules_daily_update(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ENGLAND_RULES_DAILY_UPDATE);
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    let r = data_de(d.as_mut_ptr() as usize, 0x14, 5, 0x7ce, -1);
    if read_i16(data) != read_i16(r) {
        o(this, data);
        return;
    }
    let n = (read_usize(G_CLUB_COUNT) as i32).max(0) as usize;
    let regioes = [(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x91c), n * 16)];
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this, data), || england_rules_daily_update(this, data));
    PLACAR.registrar(igual, || format!("england_rules_daily_update(regras {:#x}) tabela diverge", this));
}

/// Comparer for `england_rules_get_loan_period` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nas quatro saídas e em [0xae1080] (a regra base pode consultar contratos).
pub(crate) unsafe extern "thiscall" fn comparar_england_rules_get_loan_period(this: usize, clube: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_ENGLAND_RULES_GET_LOAN_PERIOD);
    let regioes = [(ini_mes as usize, 1usize), (ini_ano as usize, 2), (fim_mes as usize, 1), (fim_ano as usize, 2), (G_CONTRACT_LOOKUP_RESULT, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador), || england_rules_get_loan_period(this, clube, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador));
    PLACAR.registrar(igual, || format!("england_rules_get_loan_period(regras {:#x}, clube {:#x}, jogador {:#x}) original={:x?} rust={:x?}", this, clube, jogador, mo, mr));
}
