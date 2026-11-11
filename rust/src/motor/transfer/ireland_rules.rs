use super::*;

/// Constructs the Ireland transfer rules (rule 13): one window, from 10 May to 31 January.
///
/// `IRELAND_RULES::IRELAND_RULES`; SEH frame only for the `new`, no destructor on the normal path. The Ireland transfer
/// rules (rule 13): the base (`exe::new_transfer_rules`), the vtable 0x96c190, 1 window from `IRELAND_TRANSFER_WINDOWS`. Returns the
/// object.
/// Original: `IRELAND_RULES::IRELAND_RULES` (0x63c210, ireland_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_1 ireland");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96c190, 1, &IRELAND_TRANSFER_WINDOWS, 0x9ea12c, 0x4e);
    this
}

/// Refuses a loan with five loans this season (0x1e) and between clubs of the same division (0x1d).
///
/// `IRELAND_RULES::can_loan_player(club, player, message, reason)`: five loans in the season (the 16-byte count at
/// [[`G_TRANSFER_MANAGER`] + `transfer_manager_fields::SEASON_LOAN_COUNTS`]): 0x1e; with a contract
/// (`loan_owner_club`), the requesting club and the owner in the same division (`division()`, without checking for
/// null, as in the original): 0x1d, with the nation's name (`refuse_loan`).
/// Original: `IRELAND_RULES::can_loan_player` (0x63c3d0, ireland_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ireland_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let club = Club(club);
    let count = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SEASON_LOAN_COUNTS).wrapping_add(read_usize(club.addr()).wrapping_mul(16));
    if read_u8(count) as i8 >= 5 {
        covered!("cqe_6 ireland five in the season");
        if msg != 0 {
            let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let g = club.short_gender_name() as i8 as i32;
            text(msg, 0x9c4534, g, g, club.addr() + club::SHORT_NAME);
        }
        if !reason.is_null() {
            *reason = 0x1e;
        }
        return 0;
    }
    let owner = Club(match loan_owner_club(player) {
        None => return 1,
        Some(d) => d,
    });
    if club.division() == owner.division() {
        covered!("cqe_6 ireland same division");
        return refuse_loan(msg, reason, 0x9ea168, read_usize(this + transfer_rules::NATION_ID), 0x1d);
    }
    1
}
