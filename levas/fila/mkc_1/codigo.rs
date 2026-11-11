// ---------- %%LEVA%%a leva: the base loan period, the open window and the window test (transfer_rules.cpp) ----------

/// True if date `(day, year)` is before the date at `b` (year compared unsigned first, then the day signed), as the
/// window and loan routines compare their 8-byte dates.
unsafe fn window_date_before(day: i16, year: u16, b: usize) -> bool {
    let by = read_i16(b + cm_date::YEAR) as u16;
    year < by || (year == by && day < read_i16(b))
}

/// Returns the default loan period of a nation's rules: from next month to the season end month of the club's nation,
/// this year if that month is still ahead of the start, otherwise next year.
///
/// `TRANSFER_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of the nations without their own (and called by
/// `england_rules_get_loan_period` after 19 June for players under 24), from `transfer_manager_get_loan_period`. The
/// start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 1 of this year (`G_CURRENT_DATE_YEAR`); past
/// December, January of the next year. The end month is the season end month of the club's nation
/// (`exe::contract_manager_season_end_month`: push club; mov ecx, `[G_CONTRACT_MANAGER]` -> thiscall, ret 4). Start
/// month below it: the end year is the start year; otherwise today's year + 1 (so a loan starting in the season end
/// month itself runs for a whole year). `a2` and `player` are not read.
/// Original: `TRANSFER_RULES::get_loan_period` (0x8e8bd0, transfer_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn transfer_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(1);
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = 0;
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end; // mkc1: the season end month
    covered!("mkc_1 base loan period");
    if *start_month < end {
        *end_year = *start_year;
    } else {
        *end_year = read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1);
    }
}

unsafe extern "thiscall" fn compare_transfer_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(%%I0%%);
    let regions = [(start_month as usize, 1usize), (start_year as usize, 2), (end_month as usize, 1), (end_year as usize, 2)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, a2, start_month, start_year, end_month, end_year, player), || transfer_rules_get_loan_period(this, club, a2, start_month, start_year, end_month, end_year, player));
    SCOREBOARD.record(equal, || format!("transfer_rules_get_loan_period(rules {:#x}, club {:#x}) original={:x?} rust={:x?}", this, club, mo, mr));
}

/// Tells whether the date falls in a transfer window of the rules and gives that window: the first window (12 bytes
/// each, the table at +4, the count byte at +8) that contains the date (`exe::transfer_window_contains_date`); found:
/// the window in `out` (if given) and 1. None found: 0 when the rules have windows; rules without any window (Denmark,
/// for example) are always open: `out` = 0 and 1. The count is read again after each window.
///
/// `TRANSFER_RULES::find_open_window(date, out)`, from the nations' `can_loan_player`/`can_sign_player`/
/// `can_make_offer`/`can_end_loan` (England, Australia, Norway, Turkey, Finland, France, Ireland, Spain, Japan,
/// Portugal, Germany, Belgium, Argentina) and from `find_next_window_open_date` (each day of the next year).
/// Original: `TRANSFER_RULES::find_open_window` (0x8e8e50, transfer_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn transfer_rules_find_open_window(this: usize, date: usize, out: u32) -> u32 {
    let mut k: i8 = 0;
    let mut count = read_u8(this + 8) as i8;
    while k < count {
        let window = read_usize(this + 4) + k as i32 as usize * 12;
        if exe::transfer_window_contains_date(this, date, window) != 0 {
            if out != 0 {
                write_usize(out as usize, window);
            }
            return 1;
        }
        count = read_u8(this + 8) as i8;
        k += 1;
    }
    if count != 0 {
        covered!("mkc_1 no open window");
        return 0; // mkc1: windows, none open
    }
    if out != 0 {
        write_usize(out as usize, 0);
    }
    1
}

unsafe extern "thiscall" fn compare_transfer_rules_find_open_window(this: usize, date: usize, out: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    let regions: Vec<(usize, usize)> = if out != 0 { vec![(out as usize, 4)] } else { Vec::new() };
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, date, out), || transfer_rules_find_open_window(this, date, out));
    SCOREBOARD.record(equal && ro == rr, || format!("transfer_rules_find_open_window(rules {:#x}, date {:#x}) original={} {:x?} rust={} {:x?}", this, date, ro, mo, rr, mr));
    ro
}

/// Tells whether a date falls in a transfer window: the window's two 6-byte dates (+0 opens, +6 closes: weekday +2, day
/// +3, month `transfer_window::MONTH`) taken in the date's year (`exe::transfer_window_get_date`: push year; push out;
/// mov ecx, window date -> thiscall, ret 8). A window that wraps the new year (opening month above the closing month,
/// as England's June to March) is open from the opening to the end of the year and from the start of the year to the
/// closing: the opening before the date + 1 day (`exe::date_add_days`) or the closing after the date − 1 day
/// (`exe::date_sub_days`), both ends included. Otherwise the opening before the date + 1 and the closing after the date
/// − 1. The rules (`this`) are not read.
/// Original: `transfer_window_contains_date` (0x8e8c40, transfer_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn transfer_window_contains_date(_this: usize, date: usize, window: usize) -> u32 {
    let year = read_i16(date + cm_date::YEAR) as u16 as u32;
    let (mut t1, mut t2) = ([0u8; 8], [0u8; 8]);
    let open = exe::transfer_window_get_date(window, t1.as_mut_ptr() as usize, year);
    let (od, oy) = (read_i16(open), read_i16(open + cm_date::YEAR) as u16);
    let after = exe::date_add_days(date, t2.as_mut_ptr() as usize, 1);
    let opened = window_date_before(od, oy, after);
    if read_u8(window + transfer_window::MONTH) as i8 > read_u8(window + 6 + transfer_window::MONTH) as i8 {
        if opened {
            return 1;
        }
        covered!("mkc_1 wrapped window end");
        // mkc1: the wrapped window, from the start of the year to the closing
    } else if !opened {
        return 0;
    }
    let close = exe::transfer_window_get_date(window + 6, t1.as_mut_ptr() as usize, year);
    let (cd, cy) = (read_i16(close), read_i16(close + cm_date::YEAR) as u16);
    let before = exe::date_sub_days(date, t2.as_mut_ptr() as usize, 1);
    let (bd, by) = (read_i16(before), read_i16(before + cm_date::YEAR) as u16);
    (cy > by || (cy == by && cd > bd)) as u32
}

compare_value!(compare_transfer_window_contains_date, transfer_window_contains_date, %%I2%%, (date: usize, window: usize) -> u32);
