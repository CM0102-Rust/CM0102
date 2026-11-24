// ---------- cx10_15: the Dutch First Division's progress codes and period winner news ----------

/// The Dutch Premier Division's (Eredivisie) statistics entry of a club, when the club's division is it; else none.
unsafe fn hol_premier_entry(club: usize) -> Option<usize> {
    let id = read_i32(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION);
    if read_i32(read_usize(club + club::DIVISION)) != id {
        return None;
    }
    let premier = read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * id as usize);
    Some(exe::comp_stats_find_club_entry(League(premier).comp_stats(), club))
}

/// A play-off code of a Premier Division club: 1 becomes 15 and 2 to 4 become 18; 0 and above 4 stay.
fn hol_premier_play_off_code(c: u8) -> u8 {
    match c {
        1 => 0xf,
        2..=4 => 0x12,
        _ => c,
    }
}

/// Stores a club's three progress codes, by the stage.
///
/// The club's table status (`league_table_row::STATUS` of its row in `league::TABLE`, −1 without a row) is looked up
/// first. In the league (stage −1): a period winner (status 1) is skipped; the best code 2 to 7 becomes 2 and, when the
/// worst code is 2 to 7, it becomes 7. In the play-off groups (stages 4 and 5): a club of the Premier Division
/// (`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) has its three codes stored in the Premier Division's statistics
/// (`hol_premier_play_off_code`); a First Division club has each code raised by 1. Other stages store the codes as
/// they come. The entries come from `exe::comp_stats_find_club_entry` (unchecked); the bytes are
/// `comp_stats_club::CURRENT_POSITION`, `BEST_POSITION` and `WORST_POSITION`.
/// Original: `HolFirst::set_club_progress` (0x5f73d0, hol_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn hol_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let l = League(this);
    let mut entry = exe::comp_stats_find_club_entry(l.comp_stats(), club);
    let mut status = 0xffu8;
    for k in 0..l.club_count() {
        let row = l.table() + k as usize * league_table_row::SIZE;
        if read_usize(row + league_table_row::CLUB) == club {
            status = read_u8(row + league_table_row::STATUS);
            break;
        }
    }
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    match stage as u8 {
        0xff => {
            if status == 1 {
                covered!("cx10_15 dutch period winner skipped");
                return;
            }
            if (2..=7).contains(&best) {
                best = 2;
            }
            if (2..=7).contains(&worst) {
                worst = 7;
            }
        }
        4 | 5 => match hol_premier_entry(club) {
            Some(premier) => {
                covered!("cx10_15 dutch premier club in the play-off");
                entry = premier;
                current = hol_premier_play_off_code(current);
                best = hol_premier_play_off_code(best);
                worst = hol_premier_play_off_code(worst);
            }
            None => {
                current = current.wrapping_add(1);
                best = best.wrapping_add(1);
                worst = worst.wrapping_add(1);
            }
        },
        _ => {}
    }
    let e = CompStatsClub(entry);
    e.set_current_position(current);
    e.set_best_position(best);
    e.set_worst_position(worst);
}

/// Sends the news 0x12 that a club won a period of the First Division (slots: 0 the club id, 1 `period` as a signed
/// byte, 2 the competition id) to the managers of the division (`exe::news_send_to_division_managers` with
/// `league::COMPETITION`).
///
/// SEH frame only for the local news item, no destructor on the normal path. `exe::new_newsitem` (lea ecx, local;
/// push 0; push 0x12 -> thiscall, ret 8), `exe::newsitem_set_slot` (thiscall, ret 8), the sender (ecx = `[G_NEWS]`;
/// push comp; push &local -> thiscall, ret 8).
/// Original: `HolFirst::add_period_winner_news` (0x5f7330, hol_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn hol_first_add_period_winner_news(this: usize, club: usize, period: u32) {
    covered!("cx10_15 dutch period winner news");
    let comp = League(this).competition();
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x12, 0);
    exe::newsitem_set_slot(p, 0, read_u32(club));
    exe::newsitem_set_slot(p, 1, period as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(p, 2, read_u32(comp.addr()));
    exe::news_send_to_division_managers(read_usize(G_NEWS), p, comp);
}

/// Comparer of the progress codes: side by side over the three progress bytes of the club's entry in this league's
/// statistics and, for a Premier Division club, in the Premier Division's (the play-off stages write there).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let mut regions = vec![(entry + comp_stats_club::CURRENT_POSITION, 3)];
    if let Some(premier) = hol_premier_entry(club).filter(|&e| e != 0) {
        regions.push((premier + comp_stats_club::CURRENT_POSITION, 3));
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, stage, current, best, worst),
        || hol_first_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("hol_first_set_club_progress({:#x}, stage {}): original {:x?} copy {:x?}", club, stage as u8 as i8, mo, mr));
}

/// Comparer of the period winner news: it sends news (it cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_add_period_winner_news(this: usize, club: usize, period: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%)(this, club, period)
}
