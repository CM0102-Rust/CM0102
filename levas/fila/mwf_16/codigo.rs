// ---------- mwf_16: the Italian promotion and relegation, the Serie C2 exchange and the Serie C1 regrouping (ita_ser_a.cpp) ----------

/// The Italian promotion and relegation chain: the final positions of Serie A, B, C1 A and C1 B; four up and four down
/// between Serie A and B, the swaps between Serie B and each C1 group; when Serie C2 is loaded, its three groups'
/// positions and the swaps between each C1 group and each C2 group.
///
/// The final positions (virtual `league_vtable::RECORD_FINAL_POSITIONS`, thiscall, ret 0) of the objects of
/// [`KEY_CLUB_COMP_ITALIAN_SERIE_A`], [`KEY_CLUB_COMP_ITALIAN_SERIE_B`], [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_A`] and
/// [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_B`] (`G_COMP_OBJECT_TABLE`, none checked for null); League::promote_and_relegate
/// (`exe::league_promote_and_relegate`, thiscall(this, upper, lower, a, b, c, d), ret 0x18) for A and B with (1, 1, 4,
/// 4), B and C1 A and B and C1 B with (1, 1, −1, −1). When the Serie C2 A object exists: the final positions of C2 A,
/// C2 B and C2 C, then C1 A with C2 A (1, 1, −1, −1), C1 B with C2 A (0, 1, −1, −1), C1 A with C2 B (1, 1), C1 B with
/// C2 B (0, 1), C1 A with C2 C (1, 1) and C1 B with C2 C (0, 1); the last one's eax is returned. Without C2 A it returns
/// the competition object table's address (the eax the original leaves; the caller does not read it).
/// Original: `ItaSerA::process_promotion_relegation_chain` (0x642410, ita_ser_a.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (a, b, c1a, c1b) = (KEY_CLUB_COMP_ITALIAN_SERIE_A, KEY_CLUB_COMP_ITALIAN_SERIE_B, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B);
    covered!("mwf_16 italian chain");
    for k in [a, b, c1a, c1b] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), league_vtable::RECORD_FINAL_POSITIONS);
    }
    exe::league_promote_and_relegate(this, comp_by_global(a), comp_by_global(b), 1, 1, 4, 4); // mwf16: four up and four down between A and B
    exe::league_promote_and_relegate(this, comp_by_global(b), comp_by_global(c1a), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(b), comp_by_global(c1b), 1, 1, -1, -1);
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    covered!("mwf_16 serie c2 chain");
    let (c2a, c2b, c2c) = (KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C);
    for k in [c2a, c2b, c2c] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), league_vtable::RECORD_FINAL_POSITIONS);
    }
    let mut r = 0;
    for lower in [c2a, c2b, c2c] {
        exe::league_promote_and_relegate(this, comp_by_global(c1a), comp_by_global(lower), 1, 1, -1, -1);
        r = exe::league_promote_and_relegate(this, comp_by_global(c1b), comp_by_global(lower), 0, 1, -1, -1);
    }
    r
}

/// Comparer for `ita_ser_a_process_promotion_relegation_chain`: only the original (it moves the clubs between the
/// divisions, as the Brazilian chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, pairs)
}

/// Without Serie C2: the six best Serie C2 clubs go up to Serie C1 A and the clubs relegated from either C1 group go
/// down to Serie C2 A (the C1 regrouping that follows rebalances the two groups).
///
/// C1 A and C1 B are the objects of [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_A`] and [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_B`]. A
/// list of `G_CLUB_COUNT` × 4 bytes (`exe::crt_malloc`; out of memory, the box of ita_ser_a.cpp line 0x419 and it
/// returns) takes, in club table order, every club whose division has the id of C2 A, C2 B or C2 C (the count is a
/// word). With fewer than six, nothing moves. Otherwise the list is sorted by reputation and shuffled in sixes
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 6), cdecl) and its first six go up to C1 A
/// (League::move_club_to_division `exe::league_move_club_to_division`, thiscall(this, club, the C1 A record, 1), ret
/// 0xc). Then for each row i below C1 A's `.club_count()` (read once; C1 B is assumed the same size): a C1 A row with
/// `league_table_row::STATUS` 3 and then a C1 B row with status 3 send their clubs down to C2 A
/// (League::relegate_club_to_division 0x6831a0, thiscall(this, club, the C2 A record, 1), ret 0xc). The list is freed.
/// Original: `ItaSerA::exchange_serie_c2_clubs_unplayed` (0x641fc0, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_exchange_serie_c2_clubs_unplayed(this: usize) {
    let this = League(this);
    let (c1a, c1b) = (League(comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A)), League(comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B)));
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9ea2a8, 0x419);
        return;
    }
    let c2 = [KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C];
    let mut n = 0i16;
    let mut k = 0i32;
    while k < read_i32(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + k as usize * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && c2.iter().any(|&g| read_u32(d) == read_u32(g)) {
            write_usize(list + n as usize * 4, c);
            n = n.wrapping_add(1);
        }
        k += 1;
    }
    if n >= 6 {
        covered!("mwf_16 serie c2 up to c1");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 6); // mwf16: the C2 clubs by reputation, shuffled in sixes
        for t in 0..6 {
            exe::league_move_club_to_division(this, read_usize(list + t * 4), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), 1);
        }
        let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
        let rows = c1a.club_count() as i32;
        for i in 0..rows.max(0) {
            for group in [c1a, c1b] {
                if read_u8(exe::league_get_table_row(group, i) + league_table_row::STATUS) == 3 {
                    relegate(this.addr(), read_usize(exe::league_get_table_row(group, i)), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), 1);
                }
            }
        }
    }
    exe::crt_free(list);
}

/// Comparer for `ita_ser_a_exchange_serie_c2_clubs_unplayed`: only the original runs (it draws and moves clubs);
/// afterwards every club in a C1 A or C1 B row marked relegated (status 3) must be in Serie C2 A.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_exchange_serie_c2_clubs_unplayed(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let c2a = comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A);
    let mut wrong = Vec::new();
    for key in [KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B] {
        let g = League(comp_by_global(key));
        for i in 0..g.club_count().max(0) as i32 {
            let row = exe::league_get_table_row(g, i);
            if read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != c2a {
                wrong.push(read_usize(row));
            }
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("ita_ser_a_exchange_serie_c2_clubs_unplayed({:#x}) relegated clubs not in C2 A: {:x?}", this, wrong));
}

/// The Serie C1 regrouping: the 36 clubs of the two C1 groups sorted from north to south (by their stadium's city) and
/// split, the first 18 into C1 B and the other 18 into C1 A; the human manager of a club that changes group gets the
/// news.
///
/// In club table order (the index compared as a word with `G_CLUB_COUNT`, read once), the clubs whose division is the
/// C1 A or the C1 B record (`comp_record_of_key`) enter a local list of 36 (the count is a signed byte; it stops at
/// 36). The list is sorted (`exe::crt_qsort`(list, n, 4, 0x53fd40), the latitude comparer). Then the 36 entries (always
/// 36: with fewer clubs the original reads stack garbage, a defect [inference]): entry k < 18 goes to C1 B, the others
/// to C1 A; a club that was not already there and whose manager is human (`exe::news_club_has_human_manager`, thiscall
/// (`G_NEWS`, club)) gets the news item 0x13 (NewsItem::NewsItem `exe::new_newsitem`(item, 0x13, 0); slot 0 the club's
/// id, slot 1 the group's id; News::send_to_club_manager `exe::news_send_to_club_manager`(`G_NEWS`, item, club)); every
/// club's `club::DIVISION` becomes its group's record.
/// Original: `ita_ser_a_regroup_serie_c1_clubs` (0x642630, ita_ser_a.cpp, called with ecx = the Serie A object and no
/// arguments, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_regroup_serie_c1_clubs(_this: usize) {
    let (a, b) = (comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let count = read_i32(G_CLUB_COUNT);
    let mut list = [0usize; 36];
    let mut n = 0i8;
    let mut k = 0i16;
    if count > 0 {
        loop {
            let c = read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE;
            let d = read_usize(c + club::DIVISION);
            if d == a || d == b {
                list[n as usize] = c;
                n += 1;
                if n == 36 {
                    break;
                }
            }
            k = k.wrapping_add(1);
            if k as i32 >= count {
                break;
            }
        }
    }
    covered!("mwf_16 serie c1 regrouping");
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x53fd40usize); // mwf16: north to south
    for (k, &c) in list.iter().enumerate() {
        if c == 0 {
            continue; // simplification: an empty entry (fewer than 36 clubs) is skipped; the original reads stack garbage
        }
        let group = if k < 18 { b } else { a };
        if read_usize(c + club::DIVISION) != group && exe::news_club_has_human_manager(read_usize(G_NEWS), c) != 0 {
            let mut news = [0u32; 0x38];
            let nt = news.as_mut_ptr() as usize;
            exe::new_newsitem(nt, 0x13, 0);
            exe::newsitem_set_slot(nt, 0, read_u32(c));
            exe::newsitem_set_slot(nt, 1, read_u32(group));
            exe::news_send_to_club_manager(read_usize(G_NEWS), nt, c);
        }
        write_usize(c + club::DIVISION, group);
    }
}

/// Comparer for `ita_ser_a_regroup_serie_c1_clubs`: only the original runs (it sends news); the plan, made before from
/// the same sort over a copy of the list, is the group of each of the 36 clubs.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_regroup_serie_c1_clubs(this: usize) {
    let (a, b) = (comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let mut list: Vec<usize> = (0..read_i32(G_CLUB_COUNT).max(0) as usize)
        .map(|k| read_usize(G_CLUB_TABLE) + k * club::SIZE)
        .filter(|&c| { let d = read_usize(c + club::DIVISION); d == a || d == b })
        .take(36)
        .collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    if list.len() != 36 {
        return; // simplification: with fewer than 36 the original reads stack garbage; nothing to check
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, 36, 4, 0x53fd40usize);
    let wrong: Vec<usize> = list.iter().enumerate().filter(|&(k, &c)| read_usize(c + club::DIVISION) != if k < 18 { b } else { a }).map(|(_, &c)| c).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("ita_ser_a_regroup_serie_c1_clubs: clubs in the wrong group {:x?}", wrong));
}
