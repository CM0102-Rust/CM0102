// --- sx09b_2: the discipline nation index order and the clubs' fine counts (discipline.cpp, Fine.cpp) ---

/// Orders two items of the discipline nation index (`discipline_fields::NATION_INDEX`, 8 bytes: +0 the person id,
/// `nation_index_item::NATION_ID`), for qsort and bsearch: by the nation id (the difference, when they differ), then
/// by the person id; a person id of −1 on either side (the key that `nation_index_range` searches with) matches any
/// person of the nation (0).
/// Original: `compare_nation_index_entries` (0x55e5f0, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_nation_index_entries(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (nation_a, nation_b) = (read_i32(a + nation_index_item::NATION_ID), read_i32(b + nation_index_item::NATION_ID));
    if nation_a != nation_b {
        return nation_a.wrapping_sub(nation_b);
    }
    let (id_a, id_b) = (read_i32(a), read_i32(b));
    if id_a == -1 || id_b == -1 { // sx09b_2: the key matches any person
        covered!("sx09b_2 nation index key");
        return 0;
    }
    id_a.wrapping_sub(id_b)
}

/// The club's fine count: the byte of the club (by its id) in the Fine object's per-club table (the pointer at +8).
unsafe fn club_fine_count(this: usize, club: usize) -> usize {
    read_usize(this + 8).wrapping_add(read_usize(club)) // +8: the per-club fine count table
}

/// Halves the club's count of player fines (at the new season): the signed byte × 0.5 (`rdata::F64_0_5`), truncated
/// (`ftol`). A null club: nothing.
/// Original: `Fine::halve_club_fine_count` (0x5a5930, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn halve_club_fine_count(this: usize, club: usize) {
    if club == 0 {
        return;
    }
    let e = club_fine_count(this, club);
    write_u8(e, ftol(read_i8(e) as f64 * read_f64(rdata::F64_0_5)) as u8);
}

/// Clears the club's count of player fines (when its manager leaves). A null club: nothing.
/// Original: `Fine::clear_club_fine_count` (0x5a5960, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_club_fine_count(this: usize, club: usize) {
    if club == 0 {
        return;
    }
    write_u8(club_fine_count(this, club), 0);
}

compare_order!(compare_compare_nation_index_entries, compare_nation_index_entries, %%I0%%);

/// Comparer for `halve_club_fine_count`: side by side over the club's byte.
pub(crate) unsafe extern "thiscall" fn compare_halve_club_fine_count(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    if club == 0 {
        return o(this, club);
    }
    let e = club_fine_count(this, club);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e, 1)], || o(this, club), || halve_club_fine_count(this, club));
    SCOREBOARD.record(same, || format!("halve_club_fine_count(club {:#x}) original={:x?} rust={:x?}", club, mo, mr));
}

/// Comparer for `clear_club_fine_count`: side by side over the club's byte.
pub(crate) unsafe extern "thiscall" fn compare_clear_club_fine_count(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    if club == 0 {
        return o(this, club);
    }
    let e = club_fine_count(this, club);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e, 1)], || o(this, club), || clear_club_fine_count(this, club));
    SCOREBOARD.record(same, || format!("clear_club_fine_count(club {:#x}) original={:x?} rust={:x?}", club, mo, mr));
}
