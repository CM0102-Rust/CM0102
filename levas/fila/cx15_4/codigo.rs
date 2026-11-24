// ---------- cx15_4: the neutral venue orders by nation and by continent (stadium.cpp) ----------

/// qsort order of the 16-byte neutral venues by nation id (+0, the dword difference), then by stadium capacity, the
/// largest first (`stadium_compare_venues_by_capacity`). The order `build_venue_list` leaves (`stadium_obj::VENUE_SORT` 1) and that
/// `find_available_venue` sorts back to before looking in a nation.
/// Original: `stadium_compare_venues_by_nation` (0x85e2f0, stadium.cpp, cdecl).
pub(crate) unsafe extern "C" fn stadium_compare_venues_by_nation(a: usize, b: usize) -> i32 {
    let nation = read_u32(a).wrapping_sub(read_u32(b)) as i32; // the venue's nation id
    if nation != 0 {
        return nation;
    }
    covered!("cx15_4 venues of one nation by capacity");
    stadium_compare_venues_by_capacity(a, b)
}

/// qsort order of the 16-byte neutral venues by continent id (+4, the dword difference), then by stadium capacity, the
/// largest first (`stadium_compare_venues_by_capacity`). `find_available_venue` sorts the list this way (`stadium_obj::VENUE_SORT` 2)
/// before looking in a continent.
/// Original: `stadium_compare_venues_by_continent` (0x85e320, stadium.cpp, cdecl).
pub(crate) unsafe extern "C" fn stadium_compare_venues_by_continent(a: usize, b: usize) -> i32 {
    let continent = read_u32(a + 4).wrapping_sub(read_u32(b + 4)) as i32; // the venue's continent id
    if continent != 0 {
        return continent;
    }
    covered!("cx15_4 venues of one continent by capacity");
    stadium_compare_venues_by_capacity(a, b)
}

/// Comparers of the two orders: both run on the same pair and the results must be equal.
pub(crate) unsafe extern "C" fn compare_stadium_compare_venues_by_nation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = stadium_compare_venues_by_nation(a, b);
    SCOREBOARD.record(r == n, || format!("stadium_compare_venues_by_nation({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

pub(crate) unsafe extern "C" fn compare_stadium_compare_venues_by_continent(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = stadium_compare_venues_by_continent(a, b);
    SCOREBOARD.record(r == n, || format!("stadium_compare_venues_by_continent({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
