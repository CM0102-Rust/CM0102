// ---------- cx08_17: the Belgian Second Division period winner news and club progress (bel_second.cpp) ----------

/// Rule: a club that wins a third of the Belgian Second Division season gets the news (type 0x11) of its play-off
/// place, sent to the managers of the division.
///
/// `BelSecond::send_period_winner_news(club, period)`: a local NewsItem (`exe::new_newsitem`(0x11, 0)) with the club's id
/// (slot 0), the period (slot 1, its signed byte) and the competition's id (slot 2), sent by News::send_to_division_managers
/// (`exe::news_send_to_division_managers` on `G_NEWS`, with the competition record).
/// Original: `BelSecond::send_period_winner_news` (0x420ca0, bel_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bel_second_send_period_winner_news(this: usize, club: usize, period: u32) {
    let comp = League(this).competition().addr();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x11, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club));
    exe::newsitem_set_slot(n, 1, period as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 2, read_u32(comp));
    covered!("cx08_17 belgian period winner news");
    exe::news_send_to_division_managers(read_usize(G_NEWS), n, comp);
}

/// Comparer for `bel_second_send_period_winner_news`: only the original (it sends the news).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_send_period_winner_news(this: usize, club: usize, period: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, club, period)
}

/// Rule: the Belgian Second Division progress codes: in the league a period winner (marked 1 in the table) keeps its
/// codes; otherwise the best code 2 to 5 becomes 2 and the worst code 2 to 5 becomes 5; in the play-off group (stage 3)
/// each code goes up by one; elsewhere as they come.
///
/// `BelSecond::set_club_progress(club, stage, current, best, worst)`: the club's entry (`exe::comp_stats_find_club_entry`,
/// unchecked) and its row status in the league table (0xff without a row; the first row of the club, in order). At the
/// stage byte 0xff: status 1 writes nothing; otherwise best 2..=5 → 2 and worst 2..=5 → 5. At stage 3, each code + 1.
/// The bytes go to `comp_stats_club::CURRENT_POSITION`, `BEST_POSITION` and `WORST_POSITION`.
/// Original: `BelSecond::set_club_progress` (0x420d40, bel_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bel_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let l = League(this);
    let e = exe::comp_stats_find_club_entry(l.comp_stats(), club);
    let mut status = 0xffu8;
    let mut k: i16 = 0;
    while k < l.club_count() {
        let row = LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
        if row.club() as usize == club {
            status = row.status();
            break;
        }
        k += 1;
    }
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    match stage as u8 {
        0xff => {
            if status == 1 {
                covered!("cx08_17 belgian period winner keeps progress");
                return;
            }
            if (2..=5).contains(&b) {
                b = 2;
            }
            if (2..=5).contains(&w) {
                w = 5;
            }
        }
        3 => {
            covered!("cx08_17 belgian play-off progress");
            (c, b, w) = (c.wrapping_add(1), b.wrapping_add(1), w.wrapping_add(1));
        }
        _ => {}
    }
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `bel_second_set_club_progress`: side by side on the 3 bytes (`compare_group_championship_progress`).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_group_championship_progress("bel_second_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bel_second_set_club_progress(this, club, stage, current, best, worst));
}
