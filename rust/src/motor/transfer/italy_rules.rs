use super::*;

/// Decides whether an Italian loan is allowed: refuses outside the first two windows (9) and, in the five listed
/// divisions, with four players on loan from other clubs (0x25).
///
/// `ITALY_RULES::can_loan_player(club, player, message, reason)`: outside the first two windows (0x8e8c40 with
/// `[this+4]` and `[this+4]` + 0xc): reason 9. A club of one of the five divisions (`[0x9cf578]` to `[0x9cf588]`) with
/// four players on loan from other clubs (ids below `[0xae2364]`, loan contract 0x4cf6d0) may not (0x25).
/// Original: `ITALY_RULES::can_loan_player` (0x661770, italy_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x661770 } (thiscall, ret 0x10, italy_rules.cpp): `ITALY_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora das duas primeiras janelas (0x8e8c40 com [this+4] e [this+4] + 0xc): motivo 9. Um clube de uma das
/// cinco divisões ([0x9cf578] a [0x9cf588]) com quatro jogadores emprestados de outros clubes (ids abaixo de
/// [0xae2364], contrato de empréstimo 0x4cf6d0) não pode (0x25).
pub(crate) unsafe extern "thiscall" fn italy_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    // 0x8e8c40: push janela; push 0xae2c90; ecx = this -> thiscall, ret 8
    if exe::transfer_window_contains_date(this, G_CURRENT_DATE, read_usize(this + 4)) == 0 && exe::transfer_window_contains_date(this, G_CURRENT_DATE, read_usize(this + 4) + 0xc) == 0 {
        crate::comparison::branch("cqe_6 italia fora da janela");
        return refuse_outside_window(this, msg, reason);
    }
    let comp = read_usize(club + club::DIVISION);
    if comp == 0 {
        return 1;
    }
    let id = read_usize(comp);
    if ![KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C].iter().any(|&g| read_usize(g) == id) {
        return 1;
    }
    // 0x4cf6d0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let mut n: i8 = 0;
    for k in 0..50usize {
        let p = read_usize(club + club::SQUAD + 4 * k);
        if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) == 0 {
            continue;
        }
        let cp = game_get_contract_and_store(p);
        let de = if cp != 0 { england_rules_club_of_contract(cp) } else { 0 };
        if de != club {
            n = n.wrapping_add(1);
        }
    }
    if n >= 4 {
        crate::comparison::branch("cqe_6 italia quatro emprestados");
        if msg != 0 {
            let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let g = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
            text(msg, 0x9ea9dc, g, g, club + club::SHORT_NAME);
        }
        if !reason.is_null() {
            *reason = 0x25;
        }
        return 0;
    }
    1
}

/// the Italian rule (14): three windows, from 1 July to 27 September (moved to the Thursday [inference]), from 2 to 31
/// January, and from 1 July to 30 April [inference: the free-agent one]
/// Original: `ITALY_RULES::init_transfer_windows` (0x6613f0, italy_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x6613f0 } (thiscall, ret 0, italy_rules.cpp, `ITALY_RULES::init_transfer_windows`; chamada pelo construtor das regras da Itália (regra 14), depois da base e da vtable): as
/// 3 janelas de transferência de `ITALY_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn italy_rules_init_transfer_windows(this: usize) {
    crate::comparison::branch("cqk_2 italia");
    set_transfer_windows(this, 3, &ITALY_TRANSFER_WINDOWS, 0x9ea9a4, 0x52);
}
