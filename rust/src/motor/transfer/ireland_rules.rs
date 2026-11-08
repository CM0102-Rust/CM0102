use super::*;

/// Constructs the Ireland transfer rules (rule 13): one window, from 10 May to 31 January.
///
/// `IRELAND_RULES::IRELAND_RULES`; SEH frame only for the `new`, no destructor on the normal path. The Ireland transfer
/// rules (rule 13): the base (0x8e84a0), the vtable 0x96c190, 1 window from `IRELAND_TRANSFER_WINDOWS`. Returns the
/// object.
/// Original: `IRELAND_RULES::IRELAND_RULES` (0x63c210, ireland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x63c210 } (thiscall, ret 0x14, ireland_rules.cpp, `IRELAND_RULES::IRELAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Irlanda (regra 13): a base (0x8e84a0), a vtable 0x96c190, 1 janela(s) de `IRELAND_TRANSFER_WINDOWS`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqm_1 irlanda");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96c190, 1, &IRELAND_TRANSFER_WINDOWS, 0x9ea12c, 0x4e);
    this
}

/// Refuses a loan with five loans this season (0x1e) and between clubs of the same division (0x1d).
///
/// `IRELAND_RULES::can_loan_player(club, player, message, reason)`: five loans in the season (the 16-byte count at
/// [[0xde1f54] + 0x91c]): 0x1e; with a contract, the requesting club and the owner in the same division (+0x57, without
/// checking for null, as in the original): 0x1d, with the nation's name.
/// Original: `IRELAND_RULES::can_loan_player` (0x63c3d0, ireland_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x63c3d0 } (thiscall, ret 0x10, ireland_rules.cpp): `IRELAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: cinco empréstimos na temporada (a conta de 16 bytes em [[0xde1f54] + 0x91c]): 0x1e; com contrato, o
/// clube que pede e o dono na mesma divisão (+0x57, sem conferir o nulo, como a original): 0x1d, com o nome da nação.
pub(crate) unsafe extern "thiscall" fn ireland_rules_can_loan_player(this: usize, clube: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let count = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x91c).wrapping_add(read_usize(clube).wrapping_mul(16));
    if read_u8(count) as i8 >= 5 {
        crate::comparison::branch("cqe_6 irlanda cinco na temporada");
        if msg != 0 {
            let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let g = read_u8(clube + club::SHORT_GENDER_NAME) as i8 as i32;
            text(msg, 0x9c4534, g, g, clube + club::SHORT_NAME);
        }
        if !reason.is_null() {
            *reason = 0x1e;
        }
        return 0;
    }
    let owner = match loan_owner_club(player) {
        None => return 1,
        Some(d) => d,
    };
    if read_usize(clube + club::DIVISION) == read_usize(owner + club::DIVISION) {
        crate::comparison::branch("cqe_6 irlanda mesma divisao");
        return refuse_loan(msg, reason, 0x9ea168, read_usize(this + 0xa), 0x1d);
    }
    1
}
