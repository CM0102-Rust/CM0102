use super::*;

// ---------- 273a leva: as regras de transferência da Espanha, da Itália e da França: as janelas de transferência (france_rules.cpp, italy_rules.cpp, spain_rules.cpp) ----------



/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
pub(crate) unsafe fn cqk2_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk1_janelas(this, n, t, arquivo, linha);
}


/// As janelas da Espanha: de 17 de junho a 20 de agosto e de 15 de dezembro a 31 de janeiro.
pub(crate) const CQK2_ESPANHA: [[u8; 6]; 4] = [
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
/// 2 janelas de transferência de `CQK2_ESPANHA`.
pub(crate) unsafe extern "thiscall" fn janelas_espanha(this: usize) {
    crate::comparacao::ramo("cqk_2 espanha");
    cqk1_janelas(this, 2, &CQK2_ESPANHA, 0xa82ac8, 0x54);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_janelas_espanha(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_JANELAS_ESPANHA);
    cqk1_comparar("janelas_espanha", this, || o(this), |s| janelas_espanha(s));
}

/// As janelas da Itália: de 1º de julho a 27 de setembro (andando para a quinta-feira [inferência]), de 2 a 31 de janeiro, e a terceira, de 1º de julho a 30 de abril [inferência: a dos jogadores sem contrato].
pub(crate) const CQK2_ITALIA: [[u8; 6]; 6] = [
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
/// 3 janelas de transferência de `CQK2_ITALIA`.
pub(crate) unsafe extern "thiscall" fn janelas_italia(this: usize) {
    crate::comparacao::ramo("cqk_2 italia");
    cqk1_janelas(this, 3, &CQK2_ITALIA, 0x9ea9a4, 0x52);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_janelas_italia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_JANELAS_ITALIA);
    cqk1_comparar("janelas_italia", this, || o(this), |s| janelas_italia(s));
}

/// As janelas da França: de 30 de maio a 31 de agosto, de 1º de setembro a 20 de dezembro e de 21 de dezembro a 10 de janeiro (com os dias da semana para os quais as datas andam [inferência]).
pub(crate) const CQK2_FRANCA: [[u8; 6]; 6] = [
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
/// 3 janelas de transferência de `CQK2_FRANCA`.
pub(crate) unsafe extern "thiscall" fn janelas_franca(this: usize) {
    crate::comparacao::ramo("cqk_2 franca");
    cqk1_janelas(this, 3, &CQK2_FRANCA, 0x9cccf4, 0x4f);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_janelas_franca(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_JANELAS_FRANCA);
    cqk1_comparar("janelas_franca", this, || o(this), |s| janelas_franca(s));
}

/// refuses outside the window (9) and between clubs of the same division after five league games of the player (0x1f)
/// Original: `SPAIN_RULES::can_loan_player` (0x859630, spain_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x859630 } (thiscall, ret 0x10, spain_rules.cpp): `SPAIN_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora da janela (0x8e8e50): 9; com contrato, entre clubes da mesma divisão (+0x57) depois de mais de
/// [0x968990] jogos do jogador na liga (0x7c1e30 da ficha +0x61, tipo 2, 1, 6,5): 0x1f.
pub(crate) unsafe extern "thiscall" fn cqe6_pode_emprestar_espanha(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x8e8e50: push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8
    let janela: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_FIND_OPEN_WINDOW);
    if janela(this, 0xae2c90, 0) == 0 {
        crate::comparacao::ramo("cqe_6 espanha fora da janela");
        return cqe6_fora_da_janela(this, msg, motivo);
    }
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    if read_usize(clube + club::DIVISION) == read_usize(dono + 0x57) {
        // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall, ret 0x10 (devolve no st0)
        let jogos: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
        let v = jogos(read_usize(G_PLAYER_STATS), read_usize(jogador + 0x61), 2, 1, 6.5);
        if v > f32_em(0x968990) as f64 {
            crate::comparacao::ramo("cqe_6 espanha mesma divisao");
            return cqe6_recusa(msg, motivo, 0xa82b00, read_usize(this + 0xa), 0x1f);
        }
    }
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_espanha(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_CQE6_PODE_EMPRESTAR_ESPANHA), cqe6_pode_emprestar_espanha, "cqe6_pode_emprestar_espanha", this, clube, jogador, msg, motivo)
}
