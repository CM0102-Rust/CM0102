// ---------- rx13_17: the Portuguese Second Division B's exchange with the Third Division and its three zones (por_prm.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\leagues\por_prm.cpp" (the error boxes).
const POR_PRM_CPP: usize = 0xa78724;

/// Whether a club may go up to the Portuguese Second Division B: it has a division and a nation, the nation is Portugal
/// and the division is the Third Division or the generic lower division.
unsafe fn portuguese_second_b_candidate(c: usize) -> bool {
    let d = read_usize(c + club::DIVISION);
    let nation = read_usize(c + club::NATION);
    d != 0 && nation != 0 && read_u32(nation) == read_u32(KEY_NATION_PORTUGAL)
        && (read_u32(d) == read_u32(KEY_CLUB_COMP_PORTUGUESE_THIRD_DIVISION) || read_u32(d) == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION))
}

/// rule: at the Portuguese turnover with a played Second Division B, the clubs relegated from it and its two stages go
/// down to the Third Division, and as many Portuguese clubs of the Third Division or the lower division, drawn from the
/// strongest, go up into the Second Division B
///
/// A list of `G_CLUB_COUNT` × 4 bytes (`exe::crt_malloc`; out of memory, the box of por_prm.cpp line 0x2ee) takes, in
/// club table order, every `portuguese_second_b_candidate`; none: the list freed and the box of line 0x301. The Second
/// Division B ([`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`]) and its stages 0 and 1: every row with
/// `league_table_row::STATUS` 3 goes down (`exe::league_relegate_club_to_division`(this, club, the Third Division's
/// record, 1)), counted in a signed byte. More relegated than candidates: the box of line 0x320 (the list is not freed:
/// a leak of the original). Otherwise `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, relegated) and the first
/// `relegated` entries go up (`exe::league_move_club_to_division`(this, club, the Second Division B record, 1)); the
/// list is freed.
/// Original: `por_prm_exchange_second_b_with_lower` (0x7cff70, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_exchange_second_b_with_lower(this: usize) {
    let this = League(this);
    let second_b = League(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B));
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(POR_PRM_CPP, 0x2ee);
        return;
    }
    let mut n = 0u32;
    for i in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        if portuguese_second_b_candidate(c) {
            write_usize(list + n as usize * 4, c);
            n += 1;
        }
    }
    if n == 0 {
        exe::crt_free(list);
        show_error_at(POR_PRM_CPP, 0x301);
        return;
    }
    let mut relegated = 0i8;
    for k in -1i8..2 {
        let g = if k < 0 { second_b } else { League(read_usize(second_b.stages() + k as usize * 4)) };
        for r in 0..g.club_count().max(0) as usize {
            let row = g.table() + r * 0x3b;
            if read_u8(row + league_table_row::STATUS) == 3 {
                exe::league_relegate_club_to_division(this, read_usize(row), comp_record_of_key(KEY_CLUB_COMP_PORTUGUESE_THIRD_DIVISION), 1);
                relegated = relegated.wrapping_add(1);
            }
        }
    }
    if relegated as i16 > n as u16 as i16 {
        show_error_at(POR_PRM_CPP, 0x320); // the original returns without freeing the list
        return;
    }
    covered!("rx13_17 second division b exchange");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, relegated as i16 as u16 as u32);
    for k in 0..relegated.max(0) as usize {
        exe::league_move_club_to_division(this, read_usize(list + k * 4), comp_record_of_key(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B), 1); // rx13_17: one up per relegated
    }
    exe::crt_free(list);
}

/// rule: the Second Division B clubs (up to 60) are dealt into its three zones from south to north by their stadium
/// city: the 20 southernmost to the South zone, the next 20 to the Central zone and the rest to the North zone
///
/// In club table order (stopping at 60), the clubs whose division is the record of the North, Central or South zone or
/// of the Second Division B; `exe::crt_qsort`(list, n, 4, `CLUB_LATITUDE_COMPARE`); entry k's `club::RESERVE_DIVISION`
/// [the dword at +0x60: the zone] becomes the South zone's record (k < 20), the Central's (k < 40) or the North's.
/// Original: `por_prm_assign_second_b_zones` (0x7d0210, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_assign_second_b_zones(_this: usize) {
    let (north, central, south) = (comp_record_of_key(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH),
        comp_record_of_key(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL), comp_record_of_key(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH));
    let second_b = comp_record_of_key(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    let mut list = [0usize; 60];
    let mut n = 0usize;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) && n < 60 {
        let c = read_usize(G_CLUB_TABLE) + i as u16 as usize * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d == south || d == north || d == central || d == second_b {
            list[n] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    covered!("rx13_17 second division b zones");
    exe::crt_qsort(list.as_mut_ptr() as usize, n as u32, 4, CLUB_LATITUDE_COMPARE);
    for (k, &c) in list[..n].iter().enumerate() {
        let zone = if k < 0x14 { south } else if k < 0x28 { central } else { north }; // rx13_17: south to north
        write_usize(c + club::RESERVE_DIVISION, zone);
    }
}

/// Comparer for `por_prm_exchange_second_b_with_lower`: only the original runs (it draws in the sort and moves clubs);
/// afterwards no relegated row may still hold a club in the Second Division B.
pub(crate) unsafe extern "thiscall" fn compare_por_prm_exchange_second_b_with_lower(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let second_b = League(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B));
    let record = comp_record_of_key(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    let wrong: Vec<usize> = (-1i8..2).flat_map(|k| {
        let g = if k < 0 { second_b } else { League(read_usize(second_b.stages() + k as usize * 4)) };
        (0..g.club_count().max(0) as usize).map(move |r| g.table() + r * 0x3b)
    }).filter(|&row| read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) == record)
        .map(|row| read_usize(row)).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("por_prm_exchange_second_b_with_lower({:#x}) relegated clubs still in the Second Division B: {:x?}", this, wrong));
}

/// Comparer for `por_prm_assign_second_b_zones`: side by side over the +0x60 dword of the clubs it deals (the copy's
/// sort on a copy of the list gives the same order).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_assign_second_b_zones(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let records = [KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH,
        KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B].map(|k| comp_record_of_key(k));
    let regions: Vec<(usize, usize)> = (0..read_i32(G_CLUB_COUNT).max(0) as usize)
        .map(|i| read_usize(G_CLUB_TABLE) + i * club::SIZE)
        .filter(|&c| records.contains(&read_usize(c + club::DIVISION)))
        .take(60)
        .map(|c| (c + club::RESERVE_DIVISION, 4))
        .collect();
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || por_prm_assign_second_b_zones(this));
    SCOREBOARD.record(equal, || format!("por_prm_assign_second_b_zones {:?}", first_differences(&mo, &mr)));
}
