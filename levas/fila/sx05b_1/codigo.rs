// ---------- sx05b_1: the team award's winners, its club news and its season dates (award/team_award.cpp) ----------

/// `TEAM_AWARD::send_club_news()` (0x8b0140, thiscall), called by `team_award_record_winners` through the hook.
const FN_TEAM_AWARD_SEND_CLUB_NEWS: usize = 0x8b0140;

/// Records the team: the 0x205-byte selection record (from +0x4e) goes to the award manager's team history (between
/// the award's first and last offsets, `award_manager_add_team_award_record`); each of the eleven starters (the
/// substitutes are not credited) with a staff id gets the award: his reputations rise (`award_raise_winner_reputation`
/// with the award's national and world reputations and player flag) and his history gets the team of the year entry
/// (`Staff_Records::add_team_of_year_record` with his club, the award's staff competition, the period end year and the
/// season span). Then the human clubs get their news (`TEAM_AWARD::send_club_news`) and the selection is cleared
/// (`team_award_clear_selection`).
/// Defect of the original: a starter's club counts only when its id is above 0, so the players of club 0 get the award
/// without a club (no club in the reputation rise and −1 in the history entry).
/// Original: `TEAM_AWARD::record_winners` (0x8affb0, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_record_winners(this: usize) {
    use team_award_obj as ta;
    let t = TeamAward(this);
    award_manager_add_team_award_record(read_usize(G_AWARD_MANAGER), this + ta::RECORD, this + ta::HISTORY_FIRST, this + ta::HISTORY_LAST);
    for k in 0..11 {
        let id = t.slot_id(k);
        if id < 0 {
            continue;
        }
        covered!("sx05b_1 starter credited");
        let person = staff_at(id as isize);
        let club_id = read_i32(t.slot(k) + 0x10);
        let club = if club_id > 0 { club_at(club_id as isize).addr() } else { 0 };
        if !person.is_null() {
            exe::award_raise_winner_reputation(person, club, read_u16(this + award_fields::NATIONAL_REPUTATION) as u32,
                read_u16(this + award_fields::WORLD_REPUTATION) as u32, read_u32(this + award_fields::FOR_PLAYERS));
        }
        add_team_of_year_record(read_usize(G_STAFF_RECORDS), person.addr(), club, read_usize(this + award_base::STAFF_COMP),
            read_u16(this + ta::PERIOD_END + 2) as u32, read_u8(this + ta::SEASON_SPAN) as u32);
    }
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEAM_AWARD_SEND_CLUB_NEWS);
    news(this);
    team_award_clear_selection(this);
}

/// Tells each human club which of its players made the team: one type 0x1b news item, built once and reused. For each
/// slot whose club id is not −1 and whose club a human runs (`exe::human_manager_is_human_club`(club, 0, 1, 0, 0)):
/// every slot from this one on with the same club is counted (the first three remembered) and loses its club id (−1,
/// so the club is told once); the item gets the award's nation and index (slots 0, 1), the period start and end years
/// (2, 3), the count (4) and the club id (5) and, with one to three players, each player's staff id and three name ids
/// (6 onwards, four slots a player); then it goes to the club (`NewsPost::Club`). With more than three, the slots from 6
/// keep what an earlier club left (the text counts them instead).
/// Original: `TEAM_AWARD::send_club_news` (0x8b0140, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_send_club_news(this: usize) {
    team_award_send_club_news_with(this, &mut |item, to| news_post(item, to));
}

/// `team_award_send_club_news` with the posts going to `sink`.
pub(crate) unsafe fn team_award_send_club_news_with(this: usize, sink: NewsSink) {
    use team_award_obj as ta;
    let t = TeamAward(this);
    let mut item = [0u8; NEWS_ITEM_BYTES];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x1b, 0);
    let mut i = 0i8;
    while (i as i32) < t.slot_count() as i32 {
        let club_id = read_i32(t.slot(i as usize) + 0x10);
        let club = club_at(club_id as isize);
        if club_id > -1 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
            covered!("sx05b_1 human club told");
            let (mut count, mut picked) = (0i8, [0usize; 3]);
            for j in i as usize..t.slot_count().max(0) as usize {
                if read_i32(t.slot(j) + 0x10) == club_id {
                    if count < 3 {
                        picked[count as usize] = j;
                    }
                    count = count.wrapping_add(1);
                    write_i32(t.slot(j) + 0x10, -1);
                }
            }
            exe::newsitem_set_slot(p, 0, read_u32(this + award_base::NATION_ID));
            exe::newsitem_set_slot(p, 1, read_u32(this + award_base::INDEX));
            exe::newsitem_set_slot(p, 2, read_u16(this + ta::PERIOD_START + 2) as u32);
            exe::newsitem_set_slot(p, 3, read_u16(this + ta::PERIOD_END + 2) as u32);
            exe::newsitem_set_slot(p, 4, count as i32 as u32);
            exe::newsitem_set_slot(p, 5, club_id as u32);
            if count <= 3 {
                let mut s = 6i8;
                for &k in &picked[..count.max(0) as usize] {
                    for f in [0, 4, 8, 0xc] {
                        exe::newsitem_set_slot(p, s, read_u32(t.slot(k) + f));
                        s += 1;
                    }
                }
            }
            sink(p, NewsPost::Club(club.addr()));
        }
        i = i.wrapping_add(1);
    }
}

/// Sets the award period to whole years: the start to day −1 of the year minus the season span (+0x25, signed), the
/// end to day −1 of `year` (each built with `Date::Date(day, year)` and copied with `Date::operator=`).
/// Original: `TEAM_AWARD::set_season_dates` (0x8aff50, team_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn team_award_set_season_dates(this: usize, year: u32) {
    use team_award_obj as ta;
    let (mut date, mut out) = ([0u8; 8], [0u8; 8]);
    let start = (year as i32).wrapping_sub(read_i8(this + ta::SEASON_SPAN) as i32) as u32;
    exe::new_date(date.as_mut_ptr() as usize, u32::MAX, start);
    exe::date_operator_3d(this + ta::PERIOD_START, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    exe::new_date(date.as_mut_ptr() as usize, u32::MAX, year);
    exe::date_operator_3d(this + ta::PERIOD_END, out.as_mut_ptr() as usize, date.as_ptr() as usize);
}

/// Comparer for `team_award_record_winners`: the history, the reputations, the staff records and the news cannot be
/// written twice, so only the original runs; then the selection must be cleared and the history last offset moved.
pub(crate) unsafe extern "thiscall" fn compare_team_award_record_winners(this: usize) {
    use team_award_obj as ta;
    let t = TeamAward(this);
    let before = read_u32(this + ta::HISTORY_LAST);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let ok = t.filled() == 0 && (0..20).all(|k| t.slot_id(k) == -1) && read_u32(this + ta::HISTORY_LAST) != before;
    SCOREBOARD.record(ok, || format!("team_award_record_winners({:#x}): filled {} history last {:#x} -> {:#x}", this, t.filled(), before, read_u32(this + ta::HISTORY_LAST)));
}

/// Comparer for `team_award_send_club_news`: the original posts the news; the copy rehearses it (the same item built,
/// nothing posted) from the same state, side by side over the selection's club ids (the told clubs lose theirs) and
/// the random generator.
pub(crate) unsafe extern "thiscall" fn compare_team_award_send_club_news(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let mut regions = vec![(this + team_award_obj::RECORD, 20 * TEAM_AWARD_SLOT_SIZE)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let mut posts = Vec::new();
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&regions, || o(this),
        || crate::rng::without_counting(|| team_award_send_club_news_with(this, &mut |_, to| posts.push(to))));
    SCOREBOARD.record(equal, || format!("team_award_send_club_news({:#x}) rust posts {:x?} {:?}", this, posts, first_differences(&a, &b)));
}

/// Comparer for `team_award_set_season_dates`: side by side over the two period dates.
pub(crate) unsafe extern "thiscall" fn compare_team_award_set_season_dates(this: usize, year: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + team_award_obj::PERIOD_START, 16)], || o(this, year), || team_award_set_season_dates(this, year));
    SCOREBOARD.record(equal, || format!("team_award_set_season_dates(year {}) {:?}", year as u16, first_differences(&a, &b)));
}
