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

/// Tells whether a loan with an Irish club can end (sent back or recalled): only on a date inside an Irish transfer window.
///
/// `IRELAND_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN` of
/// the Republic of Ireland, from `can_end_loan` (the loan screens, the AI loan request) and from
/// `transfer_manager_can_loan_player` (each month of the loan of a player owned by an Irish club, flag 1). Without a
/// date, today (`G_CURRENT_DATE`). An Irish window open on it (`exe::transfer_rules_find_open_window`, push 0; push
/// date; ecx = this; the one window, 10 May to 31 January): 1. Otherwise 0 and, with a message, the Irish adjective
/// (`[G_NATION_TABLE] + [KEY_NATION_REPUBLIC_OF_IRELAND] · nation::SIZE`, form 10) in "Players cannot be sent back
/// until after the <adjective> transfer deadline has passed." (0x9cd030) when `club` is the player's club
/// (`club_job()`), otherwise "Players cannot be recalled from loan after the <adjective> transfer deadline has passed."
/// (0x9ccfbc). `flag` is not read. The same code as `FRANCE_RULES::can_end_loan` (0x5c1ed0) with the Irish key.
/// Original: `IRELAND_RULES::can_end_loan` (0x63c530, ireland_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ireland_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, _flag: u32) -> u32 {
    let date = if date == 0 { G_CURRENT_DATE } else { date };
    if exe::transfer_rules_find_open_window(this, date, 0) != 0 {
        return 1; // mkc6: Ireland, the window open
    }
    covered!("mkc_6 ireland window closed");
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut adjective = [0u8; 100];
        let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_REPUBLIC_OF_IRELAND).wrapping_mul(nation::SIZE));
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 100, 10);
        let format = if Staff(player).club_job().addr() == club { 0x9cd030 } else { 0x9ccfbc };
        text(msg, format, 100, 100, adjective.as_ptr());
    }
    0
}

pub(crate) unsafe extern "thiscall" fn compare_ireland_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_IRELAND_RULES_CAN_END_LOAN);
    nation_rules_compare_can_end_loan(o, ireland_rules_can_end_loan, "ireland_rules_can_end_loan", this, club, player, msg, date, flag)
}

/// Adds 1 to the season loan count of the borrowing club when it is Irish (the count that `ireland_rules_can_loan_player`
/// limits to five; `england_rules_daily_update`, also the Irish rules' daily method, zeroes it on 20 June).
///
/// `IRELAND_RULES::record_loan(offer)`, the vtable slot `transfer_rules_vtable::RECORD_LOAN` of the Republic of Ireland,
/// from the loan signing with the borrowing club's nation. The club of the offer (id at +8, in `G_CLUB_TABLE`) with a
/// nation whose id is `KEY_NATION_REPUBLIC_OF_IRELAND`: byte 0 of its 16-byte row (`season_loans_entry`) + 1. The same
/// code as `ENGLAND_RULES::record_loan` (0x579fd0) with the Irish key.
/// Original: `IRELAND_RULES::record_loan` (0x63c4e0, ireland_rules.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ireland_rules_record_loan(_this: usize, offer: usize) {
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 8).wrapping_mul(club::SIZE)));
    let nation = club.nation().addr();
    if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) {
        covered!("mkc_6 ireland loan counted");
        let p = season_loans_entry(club);
        write_u8(p, read_u8(p).wrapping_add(1)); // mkc6: Ireland, the season count
    }
}

pub(crate) unsafe extern "thiscall" fn compare_ireland_rules_record_loan(this: usize, offer: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_IRELAND_RULES_RECORD_LOAN);
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 8).wrapping_mul(club::SIZE));
    let regions = [(season_loans_entry(Club(club)), 1usize)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, offer), || ireland_rules_record_loan(this, offer));
    SCOREBOARD.record(equal, || format!("ireland_rules_record_loan(offer {:#x}) original={:x?} rust={:x?}", offer, mo, mr));
}
