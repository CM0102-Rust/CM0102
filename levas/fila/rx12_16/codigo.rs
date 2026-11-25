// ---------- rx12_16: Serie C2 and Serie D at the Italian turnover: the exchange and the regrouping (ita_ser_a.cpp) ----------

/// With Serie C2: the nine best Italian Serie D clubs go up to Serie C2 A and the clubs relegated from any C2 group go
/// down to Serie D (the C2 regrouping that follows rebalances the three groups).
///
/// C2 A, C2 B and C2 C are the objects of [`KEY_CLUB_COMP_ITALIAN_SERIE_C2_A`], `_B` and `_C`. A list of `G_CLUB_COUNT`
/// × 4 bytes (`exe::crt_malloc`; out of memory, the box of ita_ser_a.cpp line 0x46b and it returns) takes, in club
/// table order, every club with a division and a nation whose division has the id of Serie D and whose nation is Italy
/// (the count is a word). With fewer than nine, nothing moves. Otherwise the list is sorted by reputation and shuffled
/// in nines (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 9), cdecl) and its first nine go up to C2 A
/// (`exe::league_move_club_to_division`(this, club, the C2 A record, 1)). Then for each row i below C2 A's
/// `club_count()` (read once; C2 B and C2 C are assumed the same size): a C2 A, then a C2 B, then a C2 C row with
/// `league_table_row::STATUS` 3 sends its club down to Serie D (`exe::league_relegate_club_to_division`(this, club, the
/// Serie D record, 1)). The list is freed.
/// Original: `ItaSerA::exchange_serie_d_clubs` (0x6421c0, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_exchange_serie_d_clubs(this: usize) {
    let this = League(this);
    let groups = [comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_B), comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_C)];
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9ea2a8, 0x46b);
        return;
    }
    let mut n = 0i16;
    let mut k = 0i32;
    while k < read_i32(G_CLUB_COUNT) {
        let c = Club(read_usize(G_CLUB_TABLE) + k as usize * club::SIZE);
        let (d, nation) = (c.division().addr(), c.nation().addr());
        if d != 0 && nation != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_ITALIAN_SERIE_D) && read_u32(nation) == read_u32(KEY_NATION_ITALY) {
            write_usize(list + n as usize * 4, c.addr());
            n = n.wrapping_add(1);
        }
        k += 1;
    }
    if n >= 9 {
        covered!("rx12_16 serie d up to c2");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 9); // rx12_16: the Serie D clubs by reputation, shuffled in nines
        for t in 0..9 { // rx12_16: the nine promoted
            exe::league_move_club_to_division(this, read_usize(list + t * 4), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), 1);
        }
        let rows = League(groups[0]).club_count() as i32;
        for i in 0..rows.max(0) {
            for &group in &groups {
                if read_u8(exe::league_get_table_row(group, i) + league_table_row::STATUS) == 3 {
                    exe::league_relegate_club_to_division(this, read_usize(exe::league_get_table_row(group, i)), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_D), 1);
                }
            }
        }
    }
    exe::crt_free(list);
}

/// Comparer for `ita_ser_a_exchange_serie_d_clubs`: only the original runs (it draws and moves clubs); afterwards every
/// club in a C2 row marked relegated (status 3) must be in Serie D.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_exchange_serie_d_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let serie_d = comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_D);
    let mut wrong = Vec::new();
    for key in [KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C] {
        let g = League(comp_by_global(key));
        for i in 0..g.club_count().max(0) as i32 {
            let row = exe::league_get_table_row(g, i);
            if read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != serie_d {
                wrong.push(read_usize(row));
            }
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("ita_ser_a_exchange_serie_d_clubs({:#x}) relegated clubs not in Serie D: {:x?}", this, wrong));
}

/// The clubs of the three Serie C2 groups (their division is the C2 A, B or C record), in club table order, at most 54
/// (the count is a signed byte, checked before each club; the index compared as a word with `G_CLUB_COUNT`).
unsafe fn serie_c2_clubs(records: [usize; 3]) -> Vec<usize> {
    let mut list = Vec::new();
    let mut k: i16 = 0;
    while (k as i32) < read_i32(G_CLUB_COUNT) && list.len() < 0x36 {
        let c = read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE;
        if records.contains(&read_usize(c + club::DIVISION)) {
            list.push(c);
        }
        k = k.wrapping_add(1);
    }
    list
}

/// The Serie C2 regrouping: the clubs of the three C2 groups (`serie_c2_clubs`, at most 54) sorted from north to south
/// (`exe::crt_qsort`(list, n, 4, 0x53fd40), the latitude comparer) and split by eighteens: the first 18 into C2 C, the
/// next 18 into C2 B and the rest into C2 A (`club::DIVISION` the group's record; no news, unlike the C1 regrouping).
/// Original: `ita_ser_a_regroup_serie_c2_clubs` (0x642790, ita_ser_a.cpp, called with ecx = the Serie A object and no
/// arguments, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_regroup_serie_c2_clubs(_this: usize) {
    let records = [comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_B), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_C)];
    let mut list = serie_c2_clubs(records);
    covered!("rx12_16 serie c2 regrouping");
    exe::crt_qsort(list.as_mut_ptr() as usize, list.len() as u32, 4, 0x53fd40usize); // rx12_16: north to south
    for (k, &c) in list.iter().enumerate() {
        let group = if k < 0x12 { records[2] } else if k < 0x24 { records[1] } else { records[0] }; // rx12_16: C2 C, C2 B, C2 A
        write_usize(c + club::DIVISION, group);
    }
}

/// Comparer for `ita_ser_a_regroup_serie_c2_clubs`: only the original runs; the plan, made before from the same sort
/// over a copy of the list, is the group of each club.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_regroup_serie_c2_clubs(this: usize) {
    let records = [comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_B), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_C)];
    let mut list = serie_c2_clubs(records);
    exe::crt_qsort(list.as_mut_ptr() as usize, list.len() as u32, 4, 0x53fd40usize);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let wrong: Vec<usize> = list.iter().enumerate()
        .filter(|&(k, &c)| read_usize(c + club::DIVISION) != if k < 0x12 { records[2] } else if k < 0x24 { records[1] } else { records[0] })
        .map(|(_, &c)| c).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("ita_ser_a_regroup_serie_c2_clubs: clubs in the wrong group {:x?}", wrong));
}
