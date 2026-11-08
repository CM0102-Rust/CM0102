use super::*;

// The comparers of the routines in ../transfer/england_rules.rs.
// [pt] Os comparadores das rotinas de ../transfer/england_rules.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_england_rules`.
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_england_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_ENGLAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_england_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_england_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Comparer for `england_rules_can_loan_player`.
pub(crate) unsafe extern "thiscall" fn compare_england_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    england_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_ENGLAND_RULES_CAN_LOAN_PLAYER), england_rules_can_loan_player, "england_rules_can_loan_player", this, club, player, msg, reason)
}

/// Comparador: lado a lado no byte da conta do clube da oferta.
pub(crate) unsafe extern "thiscall" fn compare_record_loan(this: usize, offer: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RECORD_LOAN);
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 8).wrapping_mul(club::SIZE));
    let regions = [(season_loans_entry(club), 1usize)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, offer), || record_loan(this, offer));
    SCOREBOARD.record(igual, || format!("record_loan(oferta {:#x}) original={:x?} rust={:x?}", offer, mo, mr));
}

/// Comparer for `england_rules_daily_update`.
///
/// [pt] Comparador: lado a lado na tabela das contas inteira ([0xae235c] clubes × 16 bytes) só no dia em que ela muda (20
/// de junho); nos outros dias as duas rodam sem gravar nada e só a original fica valendo.
pub(crate) unsafe extern "thiscall" fn compare_england_rules_daily_update(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ENGLAND_RULES_DAILY_UPDATE);
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    let r = data_de(d.as_mut_ptr() as usize, 0x14, 5, 0x7ce, -1);
    if read_i16(data) != read_i16(r) {
        o(this, data);
        return;
    }
    let n = (read_usize(G_CLUB_COUNT) as i32).max(0) as usize;
    let regions = [(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x91c), n * 16)];
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&regions, || o(this, data), || england_rules_daily_update(this, data));
    SCOREBOARD.record(igual, || format!("england_rules_daily_update(regras {:#x}) tabela diverge", this));
}

/// Comparer for `england_rules_get_loan_period`.
///
/// [pt] Comparador: lado a lado nas quatro saídas e em [0xae1080] (a regra base pode consultar contratos).
pub(crate) unsafe extern "thiscall" fn compare_england_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_ENGLAND_RULES_GET_LOAN_PERIOD);
    let regions = [(start_month as usize, 1usize), (start_year as usize, 2), (end_month as usize, 1), (end_year as usize, 2), (G_CONTRACT_LOOKUP_RESULT, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, a2, start_month, start_year, end_month, end_year, player), || england_rules_get_loan_period(this, club, a2, start_month, start_year, end_month, end_year, player));
    SCOREBOARD.record(igual, || format!("england_rules_get_loan_period(regras {:#x}, clube {:#x}, jogador {:#x}) original={:x?} rust={:x?}", this, club, player, mo, mr));
}
