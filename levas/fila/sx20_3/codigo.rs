// --- sx20_3: a person's name against a text (staff_name_equals 0x84b8b0) and two orders of the shortlist manager:
// the open slots first (0x83b690) and the id pairs by their second id (0x844740) (shortlist_manager.cpp) ---

/// Whether the name of `person` (`database_format_staff_name` into a local of 500 bytes, style 1) is exactly `text`;
/// false without a person.
/// Original: `staff_name_equals` (0x84b8b0, shortlist_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_name_equals(person: usize, text: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    covered!("sx20_3 staff name compared");
    let mut name = [0u8; 0x1f4];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x1f4, 1); // sx20_3: the name style
    guio_same_c_string(name.as_ptr() as usize, text) as u32 // sx20_3: the names match
}

/// The qsort order of a club's shortlist slots: a slot with a target (`shortlist_slot::TARGET_STAFF_ID`) or an offer
/// (`shortlist_slot::OFFER_REF`) above -1 goes after one without; two open slots by price
/// (`shortlist_slot::PRICE`), highest first; equal prices answer 1 (the order is never 0).
/// Original: `shortlist_compare_slots_open_first` (0x83b690, shortlist_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn shortlist_compare_slots_open_first(a: usize, b: usize) -> i32 {
    if read_i32(a + shortlist_slot::TARGET_STAFF_ID) > -1 {
        return 1;
    }
    if read_i32(b + shortlist_slot::TARGET_STAFF_ID) > -1 {
        return -1;
    }
    if read_i32(a + shortlist_slot::OFFER_REF) > -1 {
        return 1;
    }
    if read_i32(b + shortlist_slot::OFFER_REF) > -1 {
        return -1;
    }
    covered!("sx20_3 open slots by price");
    let by_price = read_i32(b + shortlist_slot::PRICE).wrapping_sub(read_i32(a + shortlist_slot::PRICE)); // sx20_3: highest price first
    if by_price != 0 { by_price } else { 1 }
}

/// The bsearch / qsort order of the shortlist manager's id pairs (two dwords: +0 the first id, +4 the second): a
/// null pair before everything; by the second id; with the same second id, a pair whose first id is -1 (on either
/// side) matches (0), otherwise by the first id.
/// Original: `compare_id_pair_keys_by_second_id` (0x844740, shortlist_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_id_pair_keys_by_second_id(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    if read_i32(a + 4) != read_i32(b + 4) {
        return read_i32(a + 4).wrapping_sub(read_i32(b + 4));
    }
    covered!("sx20_3 same second id");
    if read_i32(a) == -1 || read_i32(b) == -1 { // sx20_3: -1 matches any first id
        return 0;
    }
    read_i32(a).wrapping_sub(read_i32(b))
}

/// Runs a two-argument cdecl routine that only reads side by side: the answers must match.
unsafe fn sx20_compare_pure(name: &str, a: usize, b: usize, original: unsafe extern "C" fn(usize, usize) -> i32,
                            copy: unsafe extern "C" fn(usize, usize) -> i32) -> i32 {
    let r = original(a, b);
    let n = copy(a, b);
    SCOREBOARD.record(r == n, || format!("{}({:#x}, {:#x}) original={} rust={}", name, a, b, r, n));
    r
}

/// Comparer for `staff_name_equals`.
pub(crate) unsafe extern "C" fn compare_staff_name_equals(person: usize, text: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I0%%)(person, text);
    let n = staff_name_equals(person, text);
    SCOREBOARD.record(r == n, || format!("staff_name_equals({:#x}, {:?}) original={} rust={}", person,
        if text == 0 { Vec::new() } else { c_string_bytes(text) }, r, n));
    r
}

/// Comparer for `shortlist_compare_slots_open_first`.
pub(crate) unsafe extern "C" fn compare_shortlist_compare_slots_open_first(a: usize, b: usize) -> i32 {
    sx20_compare_pure("shortlist_compare_slots_open_first", a, b,
        original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%), shortlist_compare_slots_open_first)
}

/// Comparer for `compare_id_pair_keys_by_second_id`.
pub(crate) unsafe extern "C" fn compare_compare_id_pair_keys_by_second_id(a: usize, b: usize) -> i32 {
    sx20_compare_pure("compare_id_pair_keys_by_second_id", a, b,
        original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%), compare_id_pair_keys_by_second_id)
}
