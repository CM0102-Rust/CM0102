/// `database_compare_staff_by_birth_date(a, b)`: qsort comparison of person pointers by date of birth, oldest first,
/// then by the style-4 name ("surname, first name").
///
/// The birth year (`date_of_birth_year()`, unsigned) decides first: `b`'s later, -1; earlier, 1. Then the day of the
/// year (`date_of_birth()`, signed) the same way. Same date: the two names (`exe::database_format_staff_name` into
/// 0x7d0 bytes, style 4) through the case-insensitive text comparison (`compare_text_nocase`). A null person counts as
/// the default `Date` (day 40 of 1972, `exe::date_date_54a110`) and the empty name. The community map calls the order
/// "youngest first"; the disassembly sorts the oldest first. Used by the squad numbers
/// (`SQUAD_MANAGER::allocate_squad_number` and the squad number list of squad_manager.cpp).
/// Original: `database_compare_staff_by_birth_date` (0x540130, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_compare_staff_by_birth_date(a: usize, b: usize) -> i32 {
    let birth = |p: usize| {
        let s = Staff(read_usize(p));
        if s.is_null() { (40i16, 1972u16) } else { (s.date_of_birth(), s.date_of_birth_year()) } // the default Date
    };
    let ((day_a, year_a), (day_b, year_b)) = (birth(a), birth(b));
    if year_b != year_a {
        return if year_b > year_a { -1 } else { 1 };
    }
    if day_b != day_a {
        return if day_b > day_a { -1 } else { 1 };
    }
    let mut name_a = [0u8; 0x7d0];
    let mut name_b = [0u8; 0x7d0];
    let (pa, pb) = (read_usize(a), read_usize(b));
    if pa != 0 {
        exe::database_format_staff_name(pa, name_a.as_mut_ptr() as usize, 0x7d0, 4); // "surname, first name"
    }
    if pb != 0 {
        exe::database_format_staff_name(pb, name_b.as_mut_ptr() as usize, 0x7d0, 4);
    }
    compare_text_nocase(name_a.as_ptr(), name_b.as_ptr())
}

/// Comparer for `database_compare_staff_by_birth_date`: the order of the original and of the copy.
pub(crate) unsafe extern "C" fn compare_database_compare_staff_by_birth_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = database_compare_staff_by_birth_date(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_staff_by_birth_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Writes the date the new transfer rules come into force (31 August 2001) into `date` (8 bytes, `Date::Date`
/// `exe::date_date_549f70`(date, 31, 7, 2001, -1), months counted from 0).
unsafe fn set_transfer_rule_change_date(date: usize) {
    exe::date_date_549f70(date as *mut [u16; 4], 0x1f, 7, 0x7d1, u32::MAX); // 31 August 2001
}

/// `init_transfer_rule_change_date()`: static initialiser of the exe (runs in the CRT start-up, before WinMain, with
/// the detours already installed by the DLL load): sets `G_TRANSFER_RULE_CHANGE_DATE` to 31 August 2001 and registers
/// its destructor (`destroy_transfer_rule_change_date` 0x524d00, an empty body) with `exe::crt_atexit`.
///
/// The date is read by the daily game loop (the "new transfer system" news), the contracts signed before the change
/// (contract_manager, staff_contracts) and the contract screen.
/// Original: `init_transfer_rule_change_date` (0x524cd0, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn init_transfer_rule_change_date() {
    set_transfer_rule_change_date(G_TRANSFER_RULE_CHANGE_DATE);
    exe::crt_atexit(0x524d00usize); // destroy_transfer_rule_change_date: an empty body
}

/// Comparer for `init_transfer_rule_change_date`: the original runs (it registers the destructor once); the copy writes
/// the date into a local and its 8 bytes are compared with the global's.
pub(crate) unsafe extern "C" fn compare_init_transfer_rule_change_date() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)();
    let mut copy = [0u16; 4];
    set_transfer_rule_change_date(copy.as_mut_ptr() as usize);
    let original: [u16; 4] = core::array::from_fn(|k| read_u16(G_TRANSFER_RULE_CHANGE_DATE + 2 * k));
    SCOREBOARD.record(copy == original, || format!("init_transfer_rule_change_date: original={:x?} rust={:x?}", original, copy));
}
