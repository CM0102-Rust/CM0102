use super::*;

/// the Russia rule (20): two windows, from 15 December to 24 March (moved to the Thursday [inference]); from 28 June
/// (moved to the Tuesday [inference]) to 4 August
/// Original: `RUSSIA_RULES::init_transfer_windows` (0x7ec7e0, russia_rules.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn russia_rules_init_transfer_windows(this: usize) {
    covered!("cqn_2 russia");
    set_transfer_windows(this, 2, &RUSSIA_TRANSFER_WINDOWS, 0xa7ad38, 0x4d);
}

/// Returns the loan period of the rules whose loan starts three months ahead: from today's month + 3 to the season end
/// month of the club's nation.
///
/// `RUSSIA_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of Brazil, Poland and Russia, from `transfer_manager_get_loan_period` with
/// the borrowing club's nation. The start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 3 of this year
/// (`G_CURRENT_DATE_YEAR`), past December 12 months back and a year on; the end month is the season end month of the
/// club's nation (`exe::contract_manager_season_end_month`); the start month (signed) below it: the end year is the start
/// year, otherwise today's year + 1. A defect of the original: when the start wraps into next year (October to
/// December) and the season ends in an earlier month (January or February), the end lands before the start (the
/// Northern Irish copy of this routine clamps it). `a2` and `player` are not read.
/// Original: `RUSSIA_RULES::get_loan_period` (0x7ec8e0, russia_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn russia_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(3); // mkc3: Russia, three months ahead
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end;
    covered!("mkc_3 russia period");
    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) };
}

pub(crate) unsafe extern "thiscall" fn compare_russia_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_RUSSIA_RULES_GET_LOAN_PERIOD);
    nation_rules_compare_loan_period(o, russia_rules_get_loan_period, "russia_rules_get_loan_period", this, club, a2, start_month, start_year, end_month, end_year, player)
}

/// Side-by-side comparer of a nation's `get_loan_period`: the four outputs.
pub(crate) unsafe fn nation_rules_compare_loan_period(o: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize), r: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize), name: &str, this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let regions = [(start_month as usize, 1usize), (start_year as usize, 2), (end_month as usize, 1), (end_year as usize, 2)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, a2, start_month, start_year, end_month, end_year, player), || r(this, club, a2, start_month, start_year, end_month, end_year, player));
    SCOREBOARD.record(equal, || format!("{}(rules {:#x}, club {:#x}) original={:x?} rust={:x?}", name, this, club, mo, mr));
}
