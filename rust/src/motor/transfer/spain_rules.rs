use super::*;



/// Constructor of a nation's transfer rules: TRANSFER_RULES base, the nation's vtable and its windows.
///
/// [pt] O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
pub(crate) unsafe fn spain_rules_construct_nation_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    set_transfer_windows(this, n, t, arquivo, linha);
}


/// As janelas da Espanha: de 17 de junho a 20 de agosto e de 15 de dezembro a 31 de janeiro.
pub(crate) const SPAIN_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x16, 0x00, 0x03, 0x11, 0x05, 0x01],
    [0x16, 0x00, 0x04, 0x14, 0x07, 0x00], // cqk2: a Espanha fecha em agosto
    [0x16, 0x01, 0xff, 0x0f, 0x0b, 0x01],
    [0x16, 0x01, 0xff, 0x1f, 0x00, 0x00],
];

/// the Spanish rule (22): two windows, from 17 June (moved to the Wednesday [inference]) to 20 August (moved to the
/// Thursday [inference]) and from 15 December to 31 January
/// Original: `SPAIN_RULES::init_transfer_windows` (0x859470, spain_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x859470 } (thiscall, ret 0, spain_rules.cpp, `SPAIN_RULES::init_transfer_windows`; chamada pelo construtor das regras da Espanha (regra 22), depois da base e da vtable): as
/// 2 janelas de transferência de `SPAIN_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn spain_rules_init_transfer_windows(this: usize) {
    crate::comparacao::ramo("cqk_2 espanha");
    set_transfer_windows(this, 2, &SPAIN_TRANSFER_WINDOWS, 0xa82ac8, 0x54);
}

/// Comparer for `spain_rules_init_transfer_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPAIN_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("spain_rules_init_transfer_windows", this, || o(this), |s| spain_rules_init_transfer_windows(s));
}

/// As janelas da Itália: de 1º de julho a 27 de setembro (andando para a quinta-feira [inferência]), de 2 a 31 de janeiro, e a terceira, de 1º de julho a 30 de abril [inferência: a dos jogadores sem contrato].
pub(crate) const ITALY_TRANSFER_WINDOWS: [[u8; 6]; 6] = [
    [0x0e, 0x00, 0xff, 0x01, 0x06, 0x01],
    [0x0e, 0x00, 0x04, 0x1b, 0x09, 0x00], // cqk2: a Itália fecha em setembro
    [0x0e, 0x01, 0xff, 0x02, 0x00, 0x01],
    [0x0e, 0x01, 0xff, 0x1f, 0x00, 0x00],
    [0x0e, 0x02, 0xff, 0x01, 0x06, 0x01],
    [0x0e, 0x02, 0xff, 0x1e, 0x03, 0x00],
];

/// the Italian rule (14): three windows, from 1 July to 27 September (moved to the Thursday [inference]), from 2 to 31
/// January, and from 1 July to 30 April [inference: the free-agent one]
/// Original: `ITALY_RULES::init_transfer_windows` (0x6613f0, italy_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x6613f0 } (thiscall, ret 0, italy_rules.cpp, `ITALY_RULES::init_transfer_windows`; chamada pelo construtor das regras da Itália (regra 14), depois da base e da vtable): as
/// 3 janelas de transferência de `ITALY_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn spain_rules_init_transfer_windows_6613f0(this: usize) {
    crate::comparacao::ramo("cqk_2 italia");
    set_transfer_windows(this, 3, &ITALY_TRANSFER_WINDOWS, 0x9ea9a4, 0x52);
}

/// Comparer for `spain_rules_init_transfer_windows_6613f0` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows_6613f0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPAIN_RULES_INIT_TRANSFER_WINDOWS_6613F0);
    compare_transfer_rules_ctor("spain_rules_init_transfer_windows_6613f0", this, || o(this), |s| spain_rules_init_transfer_windows_6613f0(s));
}

/// As janelas da França: de 30 de maio a 31 de agosto, de 1º de setembro a 20 de dezembro e de 21 de dezembro a 10 de janeiro (com os dias da semana para os quais as datas andam [inferência]).
pub(crate) const FRANCE_TRANSFER_WINDOWS: [[u8; 6]; 6] = [
    [0x09, 0x00, 0x06, 0x1e, 0x04, 0x01],
    [0x09, 0x00, 0x03, 0x1f, 0x07, 0x00], // cqk2: a França fecha em agosto
    [0x09, 0x01, 0x04, 0x01, 0x08, 0x01],
    [0x09, 0x01, 0x04, 0x14, 0x0b, 0x00],
    [0x09, 0x02, 0x05, 0x15, 0x0b, 0x01],
    [0x09, 0x02, 0x04, 0x0a, 0x00, 0x00],
];

/// the French rule (9): three windows, from 30 May to 31 August, from 1 September to 20 December and from 21 December
/// to 10 January (the dates move to weekdays [inference])
/// Original: `FRANCE_RULES::init_transfer_windows` (0x5c15e0, france_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x5c15e0 } (thiscall, ret 0, france_rules.cpp, `FRANCE_RULES::init_transfer_windows`; chamada pelo construtor das regras da França (regra 9), depois da base e da vtable): as
/// 3 janelas de transferência de `FRANCE_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn spain_rules_init_transfer_windows_5c15e0(this: usize) {
    crate::comparacao::ramo("cqk_2 franca");
    set_transfer_windows(this, 3, &FRANCE_TRANSFER_WINDOWS, 0x9cccf4, 0x4f);
}

/// Comparer for `spain_rules_init_transfer_windows_5c15e0` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows_5c15e0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPAIN_RULES_INIT_TRANSFER_WINDOWS_5C15E0);
    compare_transfer_rules_ctor("spain_rules_init_transfer_windows_5c15e0", this, || o(this), |s| spain_rules_init_transfer_windows_5c15e0(s));
}

/// refuses outside the window (9) and between clubs of the same division after five league games of the player (0x1f)
/// Original: `SPAIN_RULES::can_loan_player` (0x859630, spain_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x859630 } (thiscall, ret 0x10, spain_rules.cpp): `SPAIN_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora da janela (0x8e8e50): 9; com contrato, entre clubes da mesma divisão (+0x57) depois de mais de
/// [0x968990] jogos do jogador na liga (0x7c1e30 da ficha +0x61, tipo 2, 1, 6,5): 0x1f.
pub(crate) unsafe extern "thiscall" fn spain_rules_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x8e8e50: push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8
    let janela: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_FIND_OPEN_WINDOW);
    if janela(this, G_CURRENT_DATE, 0) == 0 {
        crate::comparacao::ramo("cqe_6 espanha fora da janela");
        return refuse_outside_window(this, msg, motivo);
    }
    let dono = match loan_owner_club(jogador) {
        None => return 1,
        Some(d) => d,
    };
    if read_usize(clube + club::DIVISION) == read_usize(dono + 0x57) {
        // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall, ret 0x10 (devolve no st0)
        let jogos: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
        let v = jogos(read_usize(G_PLAYER_STATS), read_usize(jogador + 0x61), 2, 1, 6.5);
        if v > read_f32_at(0x968990) as f64 { // 0x968990 = 5
            crate::comparacao::ramo("cqe_6 espanha mesma divisao");
            return refuse_loan(msg, motivo, 0xa82b00, read_usize(this + 0xa), 0x1f);
        }
    }
    1
}

/// Comparer for `spain_rules_can_loan_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spain_rules_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    scotland_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_SPAIN_RULES_CAN_LOAN_PLAYER), spain_rules_can_loan_player, "spain_rules_can_loan_player", this, clube, jogador, msg, motivo)
}
