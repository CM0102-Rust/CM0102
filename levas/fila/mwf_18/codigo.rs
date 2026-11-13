// ---------- mwf_18: the Conference exchange when the Conference is not played (eng_prm.cpp) ----------

/// Without a played Conference: its strongest club goes up to the Third Division if its ground meets the Third
/// Division's requirements (and the Third Division's relegated clubs go down to the Conference); otherwise the Third
/// Division's bottom club is reprieved and the Conference club gets the news.
///
/// The Third Division is the object of [`KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION`] (`G_COMP_OBJECT_TABLE`). A list of
/// `G_CLUB_COUNT` × 4 bytes (`exe::crt_malloc`; out of memory, the box of eng_prm.cpp line 0x2a9 and it returns) takes,
/// in club table order (the index compared as a word), every club whose division has the id of
/// [`KEY_CLUB_COMP_ENGLISH_CONFERENCE`]. With none: the list is freed and the box of line 0x2be (sic: no Conference
/// clubs is an error). Otherwise the list is sorted by reputation (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
/// 1): no shuffle) and the first club's ground is checked (`exe::financetable_get_club_finance`(`G_FINANCE_TABLE`,
/// club), then Finance::upgrade_stadium_for_league `exe::finance_upgrade_stadium_for_league`(finance, the Third
/// Division's `league::REQUIRED_SEATS`, `league::REQUIRED_CAPACITY` (signed words), its competition id, 1)).
///
/// Failing: the Third Division's last row (`.club_count()` − 1) gets `league_table_row::STATUS` 0xfe (stays up) and the
/// news (Comp::send_promotion_relegation_override_news 0x48ece0, thiscall(Third, the Conference record, club, 0, 2), ret
/// 0x10). Passing: the club goes up (League::move_club_to_division `exe::league_move_club_to_division`, thiscall(this,
/// club, the Third Division record, 1), ret 0xc) and every Third Division row with status 3 sends its club down to the
/// Conference (League::relegate_club_to_division 0x6831a0, thiscall(this, club, the Conference record, 1), ret 0xc).
/// The list is freed.
/// Original: `EngPrm::exchange_conference_clubs_unplayed` (0x574e60, eng_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_prm_exchange_conference_clubs_unplayed(this: usize) {
    let this = League(this);
    let third = League(comp_by_global(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION));
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9c43b0, 0x2a9);
        return;
    }
    let mut n = 0i16;
    let mut k = 0i16;
    if read_i32(G_CLUB_COUNT) > 0 {
        loop {
            let c = read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE;
            let d = read_usize(c + club::DIVISION);
            if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
                write_usize(list + n as usize * 4, c);
                n = n.wrapping_add(1);
            }
            k = k.wrapping_add(1);
            if k as i32 >= read_i32(G_CLUB_COUNT) {
                break;
            }
        }
    }
    if n == 0 {
        exe::crt_free(list);
        show_error_at(0x9c43b0, 0x2be);
        return;
    }
    covered!("mwf_18 conference exchange");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 1);
    let best = read_usize(list);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), best);
    let seats = read_i16(third.addr() + league::REQUIRED_SEATS) as i32;
    let capacity = read_i16(third.addr() + league::REQUIRED_CAPACITY) as i32;
    let ground_ok = exe::finance_upgrade_stadium_for_league(finance, seats, capacity, read_usize(third.competition().addr()), 1) != 0; // mwf18: the Conference club's ground
    if !ground_ok {
        covered!("mwf_18 conference club refused");
        write_u8(exe::league_get_table_row(third, third.club_count() as i32 - 1) + league_table_row::STATUS, 0xfe);
        let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x48ece0usize);
        news(third.addr(), comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE), best, 0, 2);
        exe::crt_free(list);
        return;
    }
    covered!("mwf_18 conference club up");
    exe::league_move_club_to_division(this, best, comp_record_of_key(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION), 1);
    let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
    let mut i = 0i32;
    while i < third.club_count() as i32 {
        if read_u8(exe::league_get_table_row(third, i) + league_table_row::STATUS) == 3 {
            relegate(this.addr(), read_usize(exe::league_get_table_row(third, i)), comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE), 1);
        }
        i += 1;
    }
    exe::crt_free(list);
}

/// Comparer for `eng_prm_exchange_conference_clubs_unplayed`: only the original runs (it may upgrade the stadium, move
/// clubs and send news); afterwards every club in a Third Division row marked relegated (status 3) must be in the
/// Conference, unless the Conference club was refused (the last row then has status 0xfe).
pub(crate) unsafe extern "thiscall" fn compare_eng_prm_exchange_conference_clubs_unplayed(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let third = League(comp_by_global(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION));
    let conference = comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE);
    let last = third.club_count() as i32 - 1;
    if last >= 0 && read_u8(exe::league_get_table_row(third, last) + league_table_row::STATUS) == 0xfe {
        return; // the refused case: nothing moved
    }
    let wrong: Vec<usize> = (0..third.club_count().max(0) as i32)
        .map(|i| exe::league_get_table_row(third, i))
        .filter(|&row| read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != conference)
        .map(|row| read_usize(row))
        .collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("eng_prm_exchange_conference_clubs_unplayed({:#x}) relegated clubs not in the Conference: {:x?}", this, wrong));
}
