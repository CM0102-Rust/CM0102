/// Orders two 37-byte past transfer rows by their fee (int at +0x18), the highest first; with equal fees a row of
/// type 2 (byte at +0x1c) goes after the other, otherwise they tie.
///
/// `transfer_manager_compare_past_transfers_by_fee(a, b)`, the qsort comparer of the transfer screens (0x8f5957).
/// Original: `transfer_manager_compare_past_transfers_by_fee` (0x8e54f0, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_manager_compare_past_transfers_by_fee(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (fa, fb) = (read_i32(a + 0x18), read_i32(b + 0x18)); // the fees
    if fa > fb {
        return -1;
    }
    if fa < fb {
        return 1;
    }
    let (ta, tb) = (read_u8(a + 0x1c), read_u8(b + 0x1c)); // the types
    if ta == tb {
        0
    } else if ta == 2 {
        1
    } else if tb == 2 {
        -1
    } else {
        0
    }
}

/// Orders two pointers to past transfer records by their date (+0x1d), the earliest first (`exe::date_operator_3c`
/// then `exe::date_operator_3e`).
///
/// `transfer_manager_compare_past_transfers_by_date(a, b)`, the qsort comparer of the club's transfer history screen
/// (club_screens, 0x455fa0).
/// Original: `transfer_manager_compare_past_transfers_by_date` (0x8e5540, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_manager_compare_past_transfers_by_date(a: *const usize, b: *const usize) -> i32 {
    let (ra, rb) = (read_usize(a as usize), read_usize(b as usize));
    let other = (read_u32(rb + 0x1d), read_u32(rb + 0x21)); // the record's date, by value
    if exe::date_operator_3c(ra + 0x1d, other.0, other.1) != 0 {
        return -1;
    }
    (exe::date_operator_3e(ra + 0x1d, other.0, other.1) != 0) as i32
}

/// Orders two 27-byte rows of a club's future transfer list: by the kind (signed byte at +0x19, ascending), then the
/// date at +0 (the earliest first), then the person's full name (the staff id at +8, `database_compare_staff_by_full_name`).
///
/// `transfer_manager_compare_future_transfer_rows(a, b)`, the qsort comparer of `get_club_future_transfer_list`
/// (0x8ccb80).
/// Original: `transfer_manager_compare_future_transfer_rows` (0x8e55f0, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_manager_compare_future_transfer_rows(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let kind = read_i8(a + 0x19) as i32 - read_i8(b + 0x19) as i32;
    if kind != 0 {
        return kind;
    }
    let other = (read_u32(b), read_u32(b + 4)); // b's date, by value
    if exe::date_operator_3c(a, other.0, other.1) != 0 {
        return -1;
    }
    if exe::date_operator_3e(a, other.0, other.1) != 0 {
        return 1;
    }
    covered!("rxs1_8 future transfer rows by name");
    let mut pa = staff_at(read_i32(a + 8) as isize).addr(); // rxs1_8: the people by name
    let mut pb = staff_at(read_i32(b + 8) as isize).addr();
    database_compare_staff_by_full_name(&mut pa as *mut usize as usize, &mut pb as *mut usize as usize)
}

compare_order!(compare_transfer_manager_compare_past_transfers_by_fee, transfer_manager_compare_past_transfers_by_fee, %%I0%%);
compare_order!(compare_transfer_manager_compare_past_transfers_by_date, transfer_manager_compare_past_transfers_by_date, %%I1%%);
compare_order!(compare_transfer_manager_compare_future_transfer_rows, transfer_manager_compare_future_transfer_rows, %%I2%%);
