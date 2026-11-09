use super::*;

/// As janelas da Espanha: de 17 de junho a 20 de agosto e de 15 de dezembro a 31 de janeiro.
pub(crate) const SPAIN_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x16, 0x00, 0x03, 0x11, 0x05, 0x01],
    [0x16, 0x00, 0x04, 0x14, 0x07, 0x00], // cqk2: Spain closes in August
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
    crate::comparison::branch("cqk_2 spain");
    set_transfer_windows(this, 2, &SPAIN_TRANSFER_WINDOWS, 0xa82ac8, 0x54);
}

/// Italy's transfer windows: from 1 July to 27 September (moved to the Thursday [inference]), from 2 to 31 January, and
/// a third one from 1 July to 30 April [inference: the one for players without a contract].
///
/// [pt] As janelas da Itália: de 1º de julho a 27 de setembro (andando para a quinta-feira [inferência]), de 2 a 31 de janeiro, e a terceira, de 1º de julho a 30 de abril [inferência: a dos jogadores sem contrato].
pub(crate) const ITALY_TRANSFER_WINDOWS: [[u8; 6]; 6] = [
    [0x0e, 0x00, 0xff, 0x01, 0x06, 0x01],
    [0x0e, 0x00, 0x04, 0x1b, 0x09, 0x00], // cqk2: Italy closes in September
    [0x0e, 0x01, 0xff, 0x02, 0x00, 0x01],
    [0x0e, 0x01, 0xff, 0x1f, 0x00, 0x00],
    [0x0e, 0x02, 0xff, 0x01, 0x06, 0x01],
    [0x0e, 0x02, 0xff, 0x1e, 0x03, 0x00],
];

/// France's transfer windows: from 30 May to 31 August, from 1 September to 20 December and from 21 December to 10
/// January (with the weekdays the dates are moved to [inference]).
///
/// [pt] As janelas da França: de 30 de maio a 31 de agosto, de 1º de setembro a 20 de dezembro e de 21 de dezembro a 10 de janeiro (com os dias da semana para os quais as datas andam [inferência]).
pub(crate) const FRANCE_TRANSFER_WINDOWS: [[u8; 6]; 6] = [
    [0x09, 0x00, 0x06, 0x1e, 0x04, 0x01],
    [0x09, 0x00, 0x03, 0x1f, 0x07, 0x00], // cqk2: France closes in August
    [0x09, 0x01, 0x04, 0x01, 0x08, 0x01],
    [0x09, 0x01, 0x04, 0x14, 0x0b, 0x00],
    [0x09, 0x02, 0x05, 0x15, 0x0b, 0x01],
    [0x09, 0x02, 0x04, 0x0a, 0x00, 0x00],
];

/// Decides whether a Spanish loan is allowed: refuses outside the window (9) and between clubs of the same division
/// after five league games of the player (0x1f).
///
/// `SPAIN_RULES::can_loan_player(club, player, message, reason)`: outside the window (`exe::transfer_rules_find_open_window`): 9; with a contract,
/// between clubs of the same division (+0x57) after more than `[0x968990]` league matches of the player (`get_stat` of
/// record +0x61, type 2, 1, 6.5): 0x1f.
/// Original: `SPAIN_RULES::can_loan_player` (0x859630, spain_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x859630 } (thiscall, ret 0x10, spain_rules.cpp): `SPAIN_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora da janela (0x8e8e50): 9; com contrato, entre clubes da mesma divisão (+0x57) depois de mais de
/// [0x968990] jogos do jogador na liga (0x7c1e30 da ficha +0x61, tipo 2, 1, 6,5): 0x1f.
pub(crate) unsafe extern "thiscall" fn spain_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    // 0x8e8e50: push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8
    let window: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_FIND_OPEN_WINDOW);
    if window(this, G_CURRENT_DATE, 0) == 0 {
        crate::comparison::branch("cqe_6 spain outside the window");
        return refuse_outside_window(this, msg, reason);
    }
    let owner = match loan_owner_club(player) {
        None => return 1,
        Some(d) => d,
    };
    if read_usize(club + club::DIVISION) == read_usize(owner + club::DIVISION) {
        // 0x7c1e30: push 6,5f; push 1; push 2; push record; mov ecx, [0xb74350] -> thiscall, ret 0x10 (returns in st0)
        let games: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
        let v = games(read_usize(G_PLAYER_STATS), read_usize(player + staff::PLAYER), 2, 1, 6.5);
        if v > read_f32(rdata::F32_5) as f64 {
            crate::comparison::branch("cqe_6 spain same division");
            return refuse_loan(msg, reason, 0xa82b00, read_usize(this + 0xa), 0x1f);
        }
    }
    1
}

