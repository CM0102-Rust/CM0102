use super::*;

/// Constructs the Wales transfer rules (rule 26): one window, from 7 May (moved to the Sunday [inference]) to 20 March.
///
/// `WALES_RULES::WALES_RULES`; SEH frame only for the `new`, no destructor on the normal path. The Wales transfer rules
/// (rule 26): the base (`exe::new_transfer_rules`), the vtable 0x970a9c, 1 window from `WALES_TRANSFER_WINDOWS`. Returns the object.
/// Original: `WALES_RULES::WALES_RULES` (0x90f080, wales_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_wales_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_2 wales");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x970a9c, 1, &WALES_TRANSFER_WINDOWS, 0xad4404, 0x4e);
    this
}

/// Returns the loan period of the rules whose loan lasts exactly to the season end: start and end both in the season end
/// month of the club's nation.
///
/// `WALES_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of 11 nations (Australia, Belgium, Croatia, France, Germany, Greece,
/// Holland, Republic of Ireland, Korea, Turkey and Wales), from `transfer_manager_get_loan_period` with the borrowing
/// club's nation. As the base period: the start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 1 of
/// this year (`G_CURRENT_DATE_YEAR`), past December 12 months back and a year on; the end month is the season end month
/// of the club's nation (`exe::contract_manager_season_end_month`: push club; mov ecx, `[G_CONTRACT_MANAGER]` ->
/// thiscall, ret 4); the start month (signed) below it: the end year is the start year, otherwise today's year + 1.
/// Then the start becomes the end (month and year): start = end, which `transfer_manager_can_loan_player` accepts at
/// once (without asking the owner's `CAN_END_LOAN`). `a2` and `player` are not read.
/// Original: `WALES_RULES::get_loan_period` (0x90f1a0, wales_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn wales_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(1);
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end;
    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) };
    covered!("mkc_3 wales period");
    *start_month = *end_month; // mkc3: Wales, the start is the end
    *start_year = *end_year;
}

pub(crate) unsafe extern "thiscall" fn compare_wales_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_WALES_RULES_GET_LOAN_PERIOD);
    nation_rules_compare_loan_period(o, wales_rules_get_loan_period, "wales_rules_get_loan_period", this, club, a2, start_month, start_year, end_month, end_year, player)
}
