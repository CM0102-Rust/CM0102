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
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEAM_AWARD_RECORD_WINNERS)(this);
    let ok = t.filled() == 0 && (0..20).all(|k| t.slot_id(k) == -1) && read_u32(this + ta::HISTORY_LAST) != before;
    SCOREBOARD.record(ok, || format!("team_award_record_winners({:#x}): filled {} history last {:#x} -> {:#x}", this, t.filled(), before, read_u32(this + ta::HISTORY_LAST)));
}

/// Comparer for `team_award_send_club_news`: the original posts the news; the copy rehearses it (the same item built,
/// nothing posted) from the same state, side by side over the selection's club ids (the told clubs lose theirs) and
/// the random generator.
pub(crate) unsafe extern "thiscall" fn compare_team_award_send_club_news(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEAM_AWARD_SEND_CLUB_NEWS);
    let mut regions = vec![(this + team_award_obj::RECORD, 20 * TEAM_AWARD_SLOT_SIZE)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let mut posts = Vec::new();
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&regions, || o(this),
        || crate::rng::without_counting(|| team_award_send_club_news_with(this, &mut |_, to| posts.push(to))));
    SCOREBOARD.record(equal, || format!("team_award_send_club_news({:#x}) rust posts {:x?} {:?}", this, posts, first_differences(&a, &b)));
}

/// Comparer for `team_award_set_season_dates`: side by side over the two period dates.
pub(crate) unsafe extern "thiscall" fn compare_team_award_set_season_dates(this: usize, year: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_TEAM_AWARD_SET_SEASON_DATES);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + team_award_obj::PERIOD_START, 16)], || o(this, year), || team_award_set_season_dates(this, year));
    SCOREBOARD.record(equal, || format!("team_award_set_season_dates(year {}) {:?}", year as u16, first_differences(&a, &b)));
}

// ---------- sx05b_2: the team award news text and the week's dates (award/team_award.cpp) ----------

/// The award's staff competition record: its name (+4) and the gender byte of the name (+0x37, sign-extended, as the
/// texts pass it).
unsafe fn award_comp_name(award: usize) -> (usize, u32) {
    let comp = read_usize(award + award_base::STAFF_COMP);
    (comp + 4, read_i8(comp + 0x37) as i32 as u32)
}

/// Writes the headline or the body of the team award news (`team_award_send_club_news`'s item: slots 2 and 3 the
/// period years, 4 the count, 5 the club, 6.. the players' ids and name ids).
///
/// The headline: "<award name>" in the award's template, with the competition's link. The body: the period as "%d"
/// (one year) or "%d/%02d" (two, the second modulo 100); one player "<name> ... <award> ... <period>", two and three
/// players their own templates, any other count the count in words with a capital first letter, the club's short
/// name, the award and the period. Each named player (one to three) whose names still match the item gets his link
/// (0x7d7); with more than three, the club's (short name, 0x7d5); then the competition's (0x7d4). The players' names
/// are built (style 1, 0xc8 bytes each) before the headline test, as the original does.
/// Original: `TEAM_AWARD::write_news` (0x8b0370, team_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn team_award_write_news(this: usize, item: usize, text: usize, body: u32) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    let start = exe::newsitem_get_slot(item, 2);
    let end = exe::newsitem_get_slot(item, 3);
    let count = exe::newsitem_get_slot(item, 4) as u8 as i8;
    let club_id = exe::newsitem_get_slot(item, 5);
    let mut players = [([0u32; 4], [0u8; 0xc8]); 3];
    if count <= 3 {
        let mut s = 6i8;
        for (ids, name) in players.iter_mut().take(count.max(0) as usize) {
            for v in ids.iter_mut() {
                *v = exe::newsitem_get_slot(item, s as u8 as u32);
                s += 1;
            }
            exe::database_format_name_from_ids(ids[1], ids[2], ids[3], name.as_mut_ptr(), 0xc8, 1, ids[0]);
        }
    }
    let (comp_name, g) = award_comp_name(this);
    if body == 0 {
        fmt(buf, 0xa8c8b0, g, g, comp_name); // "{}<%s - Award name>{}"
        exe::text_set_string(text, buf as *const u8);
    } else {
        covered!("sx05b_2 team award body");
        let mut period = [0u8; 0x40];
        let pp = period.as_mut_ptr() as usize;
        if start as u16 != end as u16 {
            comp_crt_format()(pp, 0x988070, start & 0xffff, (end & 0xffff) % 100); // "%d/%02d"
        } else {
            comp_crt_format()(pp, 0x98806c, start & 0xffff); // "%d"
        }
        let name = |k: usize| players[k].1.as_ptr() as usize;
        match count {
            1 => fmt(buf, 0xa8cbcc, g, g, name(0), comp_name, pp),
            2 => fmt(buf, 0xa8cae0, g, g, name(0), name(1), comp_name, pp),
            3 => fmt(buf, 0xa8c9c8, g, g, name(0), name(1), name(2), comp_name, pp),
            _ => {
                covered!("sx05b_2 team award count in words");
                let mut words = [0u8; 0x34];
                exe::utils_number_to_words(count as i32, words.as_mut_ptr(), 0x32, 1);
                words[0] = exe::utils_char_to_upper(read_u32(words.as_ptr() as usize) as i32);
                let club = club_at(club_id as i32 as isize);
                let cg = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32 as u32;
                fmt(buf, 0xa8c8ec, cg, cg, g, g, words.as_ptr() as usize, club.addr() + club::SHORT_NAME, comp_name, pp)
            }
        };
        exe::text_set_string(text, buf as *const u8);
        if count > 3 {
            let club = club_at(club_id as i32 as isize);
            exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        } else {
            for (ids, name) in players.iter().take(count.max(0) as usize) {
                let person = staff_at(ids[0] as i32 as isize);
                if exe::database_staff_matches_names(person, ids[1], ids[2], ids[3], 0usize) != 0 {
                    exe::text_add_link(text, name.as_ptr(), 0x7d7, person);
                }
            }
        }
    }
    let comp = read_usize(this + award_base::STAFF_COMP);
    exe::text_add_link(text, (comp + 4) as *const u8, 0x7d4, comp);
}

/// Copies the three dates of a team award: the period start (+0x242) and end (+0x24a) and the update date (+0x1d),
/// each with `Date::operator=`.
/// Original: `TEAM_AWARD::set_dates` (0x8aff00, team_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn team_award_set_dates(this: usize, start: usize, end: usize, update: usize) {
    use team_award_obj as ta;
    let mut out = [0u8; 8];
    let o = out.as_mut_ptr() as usize;
    exe::date_operator_3d(this + ta::PERIOD_START, o, start);
    exe::date_operator_3d(this + ta::PERIOD_END, o, end);
    exe::date_operator_3d(this + ta::UPDATE_DATE, o, update);
}

/// Comparer for `team_award_write_news`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_team_award_write_news(this: usize, item: usize, text: usize, body: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_TEAM_AWARD_WRITE_NEWS);
    compare_text_writer(text, || o(this, item, text, body), |t| team_award_write_news(this, item, t, body),
        || format!("team_award_write_news(count {}, body {})", exe::newsitem_get_slot(item, 4) as u8 as i8, body));
}

/// Comparer for `team_award_set_dates`: side by side over the three dates.
pub(crate) unsafe extern "thiscall" fn compare_team_award_set_dates(this: usize, start: usize, end: usize, update: usize) {
    use team_award_obj as ta;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_TEAM_AWARD_SET_DATES);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + ta::PERIOD_START, 16), (this + ta::UPDATE_DATE, 8)],
        || o(this, start, end, update), || team_award_set_dates(this, start, end, update));
    SCOREBOARD.record(equal, || format!("team_award_set_dates({:#x}) {:?}", this, first_differences(&a, &b)));
}

// ---------- sx05b_3: the year award's winners and their news (award/year_award.cpp) ----------

/// `YEAR_AWARD::post_winner_news(record)` (0x9304f0, thiscall, ret 4), called by `year_award_record_winners` through the
/// hook.
const FN_YEAR_AWARD_POST_WINNER_NEWS: usize = 0x9304f0;

/// A year award winners record (`YEAR_AWARD::record_winners`'s argument, 0x4a bytes): the year word (+0) and three
/// 0x18-byte entries from +2 (first, second and common name ids, staff id, score, club id).
pub(crate) mod year_record {
    pub(crate) const YEAR: usize = 0x0;
    pub(crate) const ENTRY: usize = 0x2;
    pub(crate) const ENTRY_SIZE: usize = 0x18;
    pub(crate) const STAFF_ID: usize = 0xc;
    pub(crate) const CLUB_ID: usize = 0x14;
}

/// Records a year award: the record goes to the award manager's year history (between the award's first and last
/// offsets, `award_manager_add_year_award_record`); with `post`, the winners' news (`YEAR_AWARD::post_winner_news`).
/// Then each of the three places with a staff id: the winner's reputations rise (`award_raise_winner_reputation` with
/// the award's national and world reputations, three quarters of them for the second, half for the third, truncated,
/// and the player flag) and his history gets the year award entry (`Staff_Records::add_year_award_record` with his
/// club or none, the award's staff competition, the record's year, the award's kind byte and the place 1 to 3).
/// Original: `YEAR_AWARD::record_winners` (0x9302f0, year_award.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn year_award_record_winners(this: usize, record: usize, post: u32) {
    use year_award_obj as ya;
    award_manager_add_year_award_record(read_usize(G_AWARD_MANAGER), record, this + ya::HISTORY_FIRST, this + ya::HISTORY_LAST);
    if post != 0 {
        let news: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_YEAR_AWARD_POST_WINNER_NEWS);
        news(this, record);
    }
    let national = read_i16(this + award_fields::NATIONAL_REPUTATION);
    let world = read_i16(this + award_fields::WORLD_REPUTATION);
    for place in 1..=3u32 {
        let e = record + year_record::ENTRY + (place as usize - 1) * year_record::ENTRY_SIZE;
        let id = read_i32(e + year_record::STAFF_ID);
        if id <= -1 {
            continue;
        }
        covered!("sx05b_3 winner credited");
        let person = staff_at(id as isize);
        let club_id = read_i32(e + year_record::CLUB_ID);
        let club = if club_id > -1 { club_at(club_id as isize).addr() } else { 0 };
        if !person.is_null() {
            // the second gets three quarters of the award's reputations, the third half (`_ftol` truncates)
            let (n, w) = match place {
                1 => (national as u16 as u32, world as u16 as u32),
                2 => (ftol(national as f64 * read_f64(rdata::F64_0_75)) as u32, ftol(world as f64 * read_f64(rdata::F64_0_75)) as u32),
                _ => (ftol(national as f64 * read_f64(rdata::F64_0_5)) as u32, ftol(world as f64 * read_f64(rdata::F64_0_5)) as u32),
            };
            exe::award_raise_winner_reputation(person, club, n, w, read_u32(this + award_fields::FOR_PLAYERS));
        }
        add_year_award_record(read_usize(G_STAFF_RECORDS), person.addr(), club, read_usize(this + award_base::STAFF_COMP),
            read_u16(record + year_record::YEAR) as u32, read_u8(this + ya::KIND) as u32, place);
    }
}

/// Posts the year award's winners news (type 0x1b). Slots: 0 and 1 the award's nation and index, 2 the year, 3..7 the
/// winner's three name ids, staff id and club id, 8..0xc the second's, 0xd the constant 0x420e9, 0xe..0x12 the
/// third's, 0x13 a draw of 3 (the template), 0x14 the winner's non-player sheet pointer (as a flag) and, for a winner
/// with a player sheet, 0x15 his position mask and 0x16 his strength flags at the record's club.
///
/// The clubs interested in the winner (`transfer_manager_collect_player_news_recipients`(winner, 0xda, up to 30), only
/// with the transfer manager and a player sheet) are collected first. Then: an award of a club competition goes to
/// that competition's division and to each interested club outside it; else, with a staff competition, one of a nation
/// (+0x57) goes to that nation and to each interested club of another nation (all of them without the competition),
/// one of a continent (+0x53) to that continent and to each interested club outside it (no nation, or its continent
/// another); any other award, and an award without a competition, goes to every manager.
/// Original: `YEAR_AWARD::post_winner_news` (0x9304f0, year_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn year_award_post_winner_news(this: usize, record: usize) {
    year_award_post_winner_news_with(this, record, &mut |item, to| news_post(item, to));
}

/// `year_award_post_winner_news` with the posts going to `sink`.
pub(crate) unsafe fn year_award_post_winner_news_with(this: usize, record: usize, sink: NewsSink) {
    let mut item = [0u8; NEWS_ITEM_BYTES];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x1b, 0);
    let slot = |k: i8, v: u32| {
        exe::newsitem_set_slot(p, k, v);
    };
    let field = |off: usize| read_u32(record + off);
    slot(0, read_u32(this + award_base::NATION_ID));
    slot(1, read_u32(this + award_base::INDEX));
    slot(2, read_u16(record + year_record::YEAR) as u32);
    for (k, off) in [(3, 2), (4, 6), (5, 0xa), (6, 0xe), (7, 0x16), (8, 0x1a), (9, 0x1e), (10, 0x22), (11, 0x26), (12, 0x2e)] {
        slot(k, field(off));
    }
    slot(0xd, 0x420e9);
    for (k, off) in [(0xe, 0x32), (0xf, 0x36), (0x10, 0x3a), (0x11, 0x3e), (0x12, 0x46)] {
        slot(k, field(off));
    }
    slot(0x13, exe::utils_random(3) as u32);
    let winner = staff_at(read_i32(record + 0xe) as isize);
    slot(0x14, read_u32(winner.addr() + staff::NON_PLAYER));
    if !winner.player().is_null() {
        covered!("sx05b_3 winner with a player sheet");
        slot(0x15, exe::formation_get_position_mask(winner.player()) as i16 as i32 as u32);
        slot(0x16, exe::staff_get_player_strength_flags(winner, club_at(read_i32(record + 0x16) as isize)));
    }
    let mut interested = [0usize; 30];
    let mut n = 0i8;
    let tm = read_usize(G_TRANSFER_MANAGER);
    if tm != 0 && !winner.player().is_null() {
        n = exe::transfer_manager_collect_player_news_recipients(tm, winner, 0xda, interested.as_mut_ptr(), 0x1e) as i8;
    }
    let interested = &interested[..n.max(0) as usize];
    let club_comp = read_usize(this + award_base::CLUB_COMP);
    let staff_comp = read_usize(this + award_base::STAFF_COMP);
    if club_comp != 0 {
        covered!("sx05b_3 news to the division");
        sink(p, NewsPost::Division(club_comp));
        for &c in interested {
            if read_usize(c + club::DIVISION) != club_comp {
                sink(p, NewsPost::Club(c));
            }
        }
    } else if staff_comp != 0 && read_usize(staff_comp + 0x57) != 0 { // +0x57: the staff competition's nation
        covered!("sx05b_3 news to the nation");
        sink(p, NewsPost::Nation(read_usize(staff_comp + 0x57)));
        for &c in interested {
            let comp = read_usize(this + award_base::STAFF_COMP);
            if comp == 0 || read_usize(c + club::NATION) != read_usize(comp + 0x57) {
                sink(p, NewsPost::Club(c));
            }
        }
    } else if staff_comp != 0 && read_usize(staff_comp + 0x53) != 0 { // +0x53: the staff competition's continent
        sink(p, NewsPost::Continent(read_usize(staff_comp + 0x53)));
        for &c in interested {
            let comp = read_usize(this + award_base::STAFF_COMP);
            let nation = read_usize(c + club::NATION);
            if comp == 0 || nation == 0 || read_usize(nation + nation::CONTINENT) != read_usize(comp + 0x53) {
                sink(p, NewsPost::Club(c));
            }
        }
    } else {
        sink(p, NewsPost::All);
    }
}

/// Comparer for `year_award_record_winners`: the history, the reputations and the staff records (with their draws)
/// cannot be written twice, so only the original runs; the history last offset must have moved.
pub(crate) unsafe extern "thiscall" fn compare_year_award_record_winners(this: usize, record: usize, post: u32) {
    let before = read_u32(this + year_award_obj::HISTORY_LAST);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_YEAR_AWARD_RECORD_WINNERS)(this, record, post);
    let after = read_u32(this + year_award_obj::HISTORY_LAST);
    SCOREBOARD.record(after != before, || format!("year_award_record_winners({:#x}, post {}): history last {:#x} -> {:#x}", this, post, before, after));
}

/// Comparer for `year_award_post_winner_news`: the original posts; the copy rehearses it (`rehearse_news`) and its one
/// draw (the template) must be the original's first and the only one it makes.
pub(crate) unsafe extern "thiscall" fn compare_year_award_post_winner_news(this: usize, record: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_YEAR_AWARD_POST_WINNER_NEWS);
    let mut posts = Vec::new();
    let (ok, a, b) = rehearse_news(|| o(this, record), || year_award_post_winner_news_with(this, record, &mut |_, to| posts.push(to)));
    SCOREBOARD.record(ok && b.len() == 1 && !posts.is_empty(), || format!("year_award_post_winner_news({:#x}) draws original {:?} rust {:?}, rust posts {:x?}", this, a, b, posts));
}

// ---------- sx05b_4: the year award news text (award/year_award.cpp) ----------

/// The slot 0xd value of a current year award news item (`year_award_post_winner_news`); older items carry another
/// value and get the plain texts, no position text and no third winner.
const YEAR_NEWS_CURRENT: u32 = 0x420e9;

/// What the year award news text reads from its item (the slots of `year_award_post_winner_news`) and builds before
/// choosing a template: the three winners (staff id and three name ids) and their club ids, whether the item is
/// current, the template draw, the non-player flag, the winner's position mask and strength flags, the names (style
/// 1, and the winner's in style 5), the winner's position text with its gender byte, and the period.
struct YearNews {
    winners: [[u32; 4]; 3],
    clubs: [i32; 3],
    current: bool,
    draw: i8,
    non_player: u32,
    mask: u32,
    flags: u32,
    names: [[u8; 0xc8]; 3],
    winner_short: [u8; 0xc8],
    position: [u8; 0xc8],
    position_gender: u8,
    period: [u8; 0x14],
}

impl YearNews {
    /// Reads the item's slots 2 to 0x16 in order, then, as the original does before any template: the winner's
    /// position text (`exe::write_position_name`(mask, text, 0xc8, 1), its gender byte; only for a current item whose
    /// winner has no non-player sheet), the names (`exe::database_format_name_from_ids`, 0xc8 bytes: the winner in styles 1 and 5, the
    /// second, and the third only in a current item with a third), and the period from the year: "%d/%02d" (the year
    /// less the award's kind byte, then the year modulo 100) when the kind byte is not 0, else "%d".
    unsafe fn read(this: usize, item: usize) -> YearNews {
        let slot = |k: u32| exe::newsitem_get_slot(item, k);
        let year = slot(2);
        let mut n = YearNews {
            winners: [[0; 4]; 3], clubs: [0; 3], current: false, draw: 0, non_player: 0, mask: 0, flags: 0,
            names: [[0; 0xc8]; 3], winner_short: [0; 0xc8], position: [0; 0xc8], position_gender: 0, period: [0; 0x14],
        };
        for (k, first) in [3u32, 8, 0xe].into_iter().enumerate() {
            if k == 2 {
                n.current = slot(0xd) == YEAR_NEWS_CURRENT;
            }
            let names = [slot(first), slot(first + 1), slot(first + 2)];
            n.winners[k] = [slot(first + 3), names[0], names[1], names[2]];
            n.clubs[k] = slot(first + 4) as i32;
        }
        n.draw = slot(0x13) as u8 as i8;
        n.non_player = slot(0x14);
        n.mask = slot(0x15);
        n.flags = slot(0x16);
        if n.non_player == 0 && n.current {
            n.position_gender = exe::write_position_name(n.mask, n.position.as_mut_ptr() as usize, 0xc8, 1);
        }
        let name = |w: [u32; 4], out: &mut [u8; 0xc8], style: u32| exe::database_format_name_from_ids(w[1], w[2], w[3], out.as_mut_ptr(), 0xc8, style, w[0]);
        let [w1, w2, w3] = n.winners;
        name(w1, &mut n.names[0], 1);
        name(w1, &mut n.winner_short, 5);
        name(w2, &mut n.names[1], 1);
        if n.current && w3[0] as i32 > -1 {
            name(w3, &mut n.names[2], 1);
        }
        let pp = n.period.as_mut_ptr() as usize;
        let kind = read_i8(this + year_award_obj::KIND) as i32;
        if kind != 0 {
            comp_crt_format()(pp, 0x988070, (year & 0xffff) as i32 - kind, (year & 0xffff) % 100); // "%d/%02d"
        } else {
            comp_crt_format()(pp, 0x98806c, year & 0xffff); // "%d"
        }
        n
    }

    fn name(&self, k: usize) -> usize {
        self.names[k].as_ptr() as usize
    }

    /// The winner's description in place of the position text (`exe::write_player_description`(id, mask, flags, text,
    /// 0xc8, the position gender, 1)) and its gender byte, sign-extended; the club-less templates use it.
    unsafe fn describe_winner(&mut self) -> u32 {
        let g = exe::write_player_description(self.winners[0][0] as i32, self.mask, self.flags, self.position.as_mut_ptr() as usize, 0xc8, self.position_gender as u32, 1);
        g as i8 as i32 as u32
    }
}

/// Writes the body of the year award news into the shared buffer: the plain template (the winner, the award, the
/// period, the second) for an old item, a winner whose first name id is a multiple of 10, a winner with a non-player
/// sheet or the French most unpopular player award; otherwise by the third winner (present or not), the draw (0, 1, other) and the
/// winner's club (present or not) one of twelve templates, the club-less ones with the winner's description
/// (`describe_winner`) and the others with his club's short name and position text.
unsafe fn year_news_body(this: usize, n: &mut YearNews) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    let comp = read_usize(this + award_base::STAFF_COMP);
    let (cn, cg) = (comp + 4, read_i8(comp + 0x37) as i32 as u32);
    let (n1, n2, n3, short) = (n.name(0), n.name(1), n.name(2), n.winner_short.as_ptr() as usize);
    let (period, pos) = (n.period.as_ptr() as usize, n.position.as_ptr() as usize);
    let pg = n.position_gender as i8 as i32 as u32;
    if !n.current || n.winners[0][1] as i32 % 10 == 0 || n.non_player != 0 || read_u32(comp) == read_u32(KEY_STAFF_COMP_FRENCH_MOST_UNPOPULAR_PLAYER_OF_THE_YEAR) {
        fmt(buf, 0xad5b68, cg, cg, n1, cn, period, n2);
        return;
    }
    covered!("sx05b_4 year award special body");
    let club = (n.clubs[0] > -1).then(|| club_at(n.clubs[0] as isize).addr());
    let (kn, kg) = club.map_or((0, 0), |c| (c + club::SHORT_NAME, read_i8(c + club::SHORT_GENDER_NAME) as i32 as u32));
    let three = n.winners[2][0] as i32 > -1;
    match (three, n.draw, club.is_some()) {
        (true, 0, true) => fmt(buf, 0xad63c8, kg, kg, cg, cg, kn, n1, cn, period, n2, n3),
        (true, 0, false) => {
            let g = n.describe_winner();
            fmt(buf, 0xad627c, g, g, cg, cg, pos, n1, cn, period, n2, n3)
        }
        (true, 1, true) => fmt(buf, 0xad6658, kg, kg, pg, pg, cg, cg, kn, pos, n1, cn, short, n2, n3),
        (true, 1, false) => {
            let g = n.describe_winner();
            fmt(buf, 0xad6508, cg, cg, g, g, n1, cn, pos, n2, n3)
        }
        (true, _, true) => fmt(buf, 0xad68e8, kg, kg, pg, pg, cg, cg, kn, pos, n1, cn, n2, n3),
        (true, _, false) => fmt(buf, 0xad67dc, cg, cg, n1, cn, n2, n3),
        (false, 0, true) => fmt(buf, 0xad5d5c, kg, kg, cg, cg, kn, n1, cn, period, n2),
        (false, 0, false) => {
            let g = n.describe_winner();
            fmt(buf, 0xad5c4c, g, g, cg, cg, pos, n1, cn, period, n2)
        }
        (false, 1, true) => fmt(buf, 0xad5f6c, kg, kg, pg, pg, cg, cg, kn, pos, n1, cn, short, n2),
        (false, 1, false) => {
            let g = n.describe_winner();
            fmt(buf, 0xad5e60, cg, cg, g, g, n1, cn, pos, n2)
        }
        (false, _, true) => {
            let g = n.describe_winner();
            fmt(buf, 0xad6170, kg, kg, g, g, cg, cg, kn, pos, n1, cn, n2)
        }
        (false, _, false) => fmt(buf, 0xad60a8, cg, cg, n1, cn, n2),
    };
}

/// Writes the headline of the year award news into the shared buffer: "<award> award decided" (the award's short
/// name) for the French most unpopular player award; otherwise one of five by (the second's staff id / the draw (at
/// least 1) + the winner's staff id) modulo 6: 0 "<winner> takes top award", 1 "<winner> named as <award>", 2 "Major
/// award for <winner>", 3 "<winner> scoops big prize", 4 "<winner> wins <period> award" (the winner's name in style
/// 5); 5 or a negative remainder "award decided".
unsafe fn year_news_headline(this: usize, n: &YearNews) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    let comp = read_usize(this + award_base::STAFF_COMP);
    let (short_name, sg) = (comp + 0x38, read_i8(comp + 0x52) as i32 as u32);
    let short = n.winner_short.as_ptr() as usize;
    let pick = if read_u32(comp) == read_u32(KEY_STAFF_COMP_FRENCH_MOST_UNPOPULAR_PLAYER_OF_THE_YEAR) {
        -1
    } else {
        let div = if n.draw >= 1 { n.draw as i32 } else { 1 };
        ((n.winners[1][0] as i32) / div).wrapping_add(n.winners[0][0] as i32) % 6
    };
    match pick {
        0 => fmt(buf, 0xad5b38, short),
        1 => fmt(buf, 0xad5adc, sg, sg, short, short_name),
        2 => fmt(buf, 0xad5aac, short),
        3 => fmt(buf, 0xad5a7c, short),
        4 => fmt(buf, 0xad5a28, short, n.period.as_ptr() as usize),
        _ => fmt(buf, 0xad59e4, sg, sg, short_name), // "{}<%s - Award name>{} award decided"
    };
}

/// The links of the year award news: the winners' clubs (short name, 0x7d5; the third's only in a current item), each
/// winner whose names still match the item (`exe::database_staff_matches_names`; 0x7d7: the winner's both names, the
/// third only in a current item with a third) and the award's competition (its name and its short name, 0x7d4).
unsafe fn year_news_links(this: usize, n: &YearNews, text: usize) {
    for (k, &c) in n.clubs.iter().enumerate() {
        if c > -1 && (k < 2 || n.current) {
            let club = club_at(c as isize);
            exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        }
    }
    let matches = |w: [u32; 4]| {
        let person = staff_at(w[0] as i32 as isize);
        (exe::database_staff_matches_names(person, w[1], w[2], w[3], 0usize) != 0).then_some(person)
    };
    if let Some(p) = matches(n.winners[0]) {
        exe::text_add_link(text, n.names[0].as_ptr(), 0x7d7, p);
        exe::text_add_link(text, n.winner_short.as_ptr(), 0x7d7, p);
    }
    if let Some(p) = matches(n.winners[1]) {
        exe::text_add_link(text, n.names[1].as_ptr(), 0x7d7, p);
    }
    if n.current && n.winners[2][0] as i32 > -1 {
        if let Some(p) = matches(n.winners[2]) {
            exe::text_add_link(text, n.names[2].as_ptr(), 0x7d7, p);
        }
    }
    let comp = read_usize(this + award_base::STAFF_COMP);
    exe::text_add_link(text, (comp + 4) as *const u8, 0x7d4, comp);
    exe::text_add_link(text, (comp + 0x38) as *const u8, 0x7d4, comp);
}

/// Writes the headline or the body of the year award news (`year_award_post_winner_news`'s item): everything is read
/// and built first (`YearNews::read`), then the headline (`year_news_headline`) or the body (`year_news_body`) goes
/// into the Text and the links follow (`year_news_links`).
/// Original: `YEAR_AWARD::write_news` (0x930880, year_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn year_award_write_news(this: usize, item: usize, text: usize, body: u32) {
    let mut n = YearNews::read(this, item);
    if body != 0 {
        year_news_body(this, &mut n);
    } else {
        covered!("sx05b_4 year award headline");
        year_news_headline(this, &n);
    }
    exe::text_set_string(text, NEWS_TEXT_BUFFER as *const u8);
    year_news_links(this, &n, text);
}

/// Comparer for `year_award_write_news`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_year_award_write_news(this: usize, item: usize, text: usize, body: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_YEAR_AWARD_WRITE_NEWS);
    compare_text_writer(text, || o(this, item, text, body), |t| year_award_write_news(this, item, t, body),
        || format!("year_award_write_news(draw {}, body {})", exe::newsitem_get_slot(item, 0x13) as u8 as i8, body));
}

// ---------- sx05b_20: the World Cup awards (award/world_cup_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\world_cup_awards.cpp" (the error box).
const WORLD_CUP_AWARDS_CPP: usize = 0xad58c4;
/// `WorldCupAwards::init_awards(file)` (0x92fc80, thiscall, ret 4), through its hook.
const FN_WORLD_CUP_AWARDS_INIT: usize = 0x92fc80;
/// `nation_awards_award_tournament_player(comp, today, award, max age, positions, stat)` (0x7690c0, ret 0x18) and
/// `NationAwards::award_tournament_team(comp, award)` (0x769af0, thiscall, ret 8): the best three players and the team
/// of a competition's statistics, crediting the national teams.
const FN_NATION_AWARDS_AWARD_TOURNAMENT_PLAYER: usize = 0x7690c0;
const FN_NATION_AWARDS_AWARD_TOURNAMENT_TEAM: usize = 0x769af0;

/// The World Cup awards' class: the vtable 0x9713a4, no nation record, nation id 0, type 4 (international).
const WORLD_CUP_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x9713a4, nation_key: None, nation_id: 0, kind: 4, month_ratings: None };

/// Builds the World Cup awards (`nation_awards_construct` with `WORLD_CUP_AWARDS`): the saved awards with a file, else
/// `WorldCupAwards::init_awards(file)`. Returns this.
/// Original: `WorldCupAwards::WorldCupAwards` (0x92fc10, world_cup_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &WORLD_CUP_AWARDS, file, core::mem::transmute(FN_WORLD_CUP_AWARDS_INIT))
}

/// Builds the two World Cup awards (`nation_awards_build`, the error box at line 0x5d), both of the FIFA World Cup: the
/// golden boot (a year award, national and world reputations 9000 and 7000, option 1) and the dream team (a team award,
/// 8500 and 6750); kind 0, no scope.
/// Original: `WorldCupAwards::init_awards` (0x92fc80, world_cup_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let cup = read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    let specs = [
        AwardSpec::Year([n, 0, read_u32(KEY_STAFF_COMP_WORLD_CUP_GOLDEN_BOOT), 9000, 7000, cup, 0, 0, 1, f, 1]),
        AwardSpec::Team([n, 1, read_u32(KEY_STAFF_COMP_WORLD_CUP_DREAM_TEAM), 8500, 6750, cup, 0, 0, f, 1]),
    ];
    covered!("sx05b_20 world cup awards built");
    nation_awards_build(this, &specs, WORLD_CUP_AWARDS_CPP, 0x5d);
}

/// The World Cup awards' day: in a World Cup year ((year − 1998) divisible by 4), on 13 July (`Date::Date(13, 6, year,
/// 0)`, compared by day and year), the golden boot (the competition's best three by statistic 2) and the dream team.
/// There is no base daily update (the World Cup awards have no month awards).
/// Original: `WorldCupAwards::daily_update` (0x92fdf0, world_cup_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_awards_daily_update(this: usize, today: usize) {
    let year = read_u16(today + cm_date::YEAR);
    if (year as i32 - 0x7ce) % 4 != 0 {
        return;
    }
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 0xd, 6, year as u32, 0);
    if d[1] != year || d[0] != read_u16(today) {
        return;
    }
    covered!("sx05b_20 world cup awards day");
    let cup = read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    let player: unsafe extern "thiscall" fn(usize, u32, usize, usize, i32, u32, u32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TOURNAMENT_PLAYER);
    player(this, cup, today, nation_award(this, 0), -1, 0, 2);
    let team: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TOURNAMENT_TEAM);
    team(this, read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP), nation_award(this, 1));
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_WORLD_CUP_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &WORLD_CUP_AWARDS, 2, "world_cup_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_WORLD_CUP_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "world_cup_awards_init");
}

/// Comparer for `world_cup_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_world_cup_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_WORLD_CUP_AWARDS_DAILY_UPDATE)(this, today)
}

// ---------- sx04b_4: the year awards given from the year ratings' candidates and from a shortlist (nation_awards.cpp) ----------

/// `YEAR_RATINGS::collect_candidates(out, max, continent, positions, mode, excluded list)` (0x931d50, year_ratings.cpp,
/// thiscall, ret 0x18): how many were collected (ax).
const FN_YEAR_RATINGS_COLLECT_CANDIDATES: usize = 0x931d50;
/// `AWARD_SHORTLIST::pick_winners(out, count)` (0x417c50, award_shortlist.cpp, thiscall, ret 8): how many were
/// written (ax).
const FN_AWARD_SHORTLIST_PICK_WINNERS: usize = 0x417c50;

/// The top three of the year ratings' candidates (see `nation_awards_award_top_three_from_ratings`), the record going
/// to `sink`; the winners are added to `list` only when `add_to_list`.
pub(crate) unsafe fn nation_awards_award_top_three_with(today: usize, award: usize, ratings: usize, continent: u32, positions: u32, mode: u32, list: usize,
    add_to_list: bool, sink: YearAwardSink) {
    let buf = exe::crt_malloc(0xc);
    if buf == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0x153);
        return;
    }
    if ratings == 0 {
        // defect of the original: the block is not freed
        tel26_error_box(NATION_AWARDS_CPP, 0x15a);
        return;
    }
    let collect: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, usize) -> u16 = core::mem::transmute(FN_YEAR_RATINGS_COLLECT_CANDIDATES);
    if (collect(ratings, buf, 3, continent, positions, mode, list) as i16) < 3 { // sx04b_4: three candidates
        exe::crt_free(buf);
        return;
    }
    covered!("sx04b_4 top three from the ratings");
    let mut people = [Staff(0); 3];
    for (k, person) in people.iter_mut().enumerate() {
        *person = staff_at(read_i32(read_usize(buf + 4 * k)) as isize);
        if list != 0 && add_to_list {
            let mut local = person.addr();
            exe::llist_add_tail(list, &mut local as *mut usize as usize, 4);
        }
    }
    sink(award, &year_record_of(today, people));
    exe::crt_free(buf);
}

/// Gives a year award to the three best candidates of a year ratings table: `YEAR_RATINGS::collect_candidates` (3, of
/// the `continent`, the `positions`, by the sort `mode`, leaving out those already in `list`) into a 12-byte block.
/// Fewer than three: nothing. Each winner is added to `list` when there is one (`LLIST::add_tail`, the staff record
/// pointer); the three (names, staff id, club id or −1) go with the year of `today` to `YEAR_AWARD::record_winners`.
/// The block is freed. Out of memory: the nation_awards.cpp 339 box; no ratings: the 346 box. Defects of the
/// original: with no ratings the block is not freed; the record's score dwords are never written.
/// Original: `nation_awards_award_top_three_from_ratings` (0x768720, nation_awards.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_top_three_from_ratings(_this: usize, today: usize, award: usize, ratings: usize,
    continent: u32, positions: u32, mode: u32, list: usize) {
    nation_awards_award_top_three_with(today, award, ratings, continent, positions, mode, list, true, &mut |award, record| record_year_award(award, record));
}

/// The three winners of a year award's shortlist (see `nation_awards_award_from_shortlist`), the record going to
/// `sink`.
pub(crate) unsafe fn nation_awards_award_from_shortlist_with(today: usize, award: usize, sink: YearAwardSink) {
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    if shortlist == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0x24c);
        return;
    }
    let buf = exe::crt_malloc(0xc);
    if buf == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0x254);
        return;
    }
    let pick: unsafe extern "thiscall" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_AWARD_SHORTLIST_PICK_WINNERS);
    let picked = pick(shortlist, buf, 3) as i16;
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AWARD_SHORTLIST_CLEAR);
    clear(shortlist);
    if picked < 3 { // sx04b_4: three winners
        exe::crt_free(buf);
        return;
    }
    covered!("sx04b_4 winners from the shortlist");
    let people = [0, 1, 2].map(|k| Staff(read_usize(buf + 4 * k)));
    sink(award, &year_record_of(today, people));
    exe::crt_free(buf);
}

/// Decides a year award from its shortlist (`AWARD_SHORTLIST` of the award): `AWARD_SHORTLIST::pick_winners` (the
/// computer clubs' votes) writes up to three staff records into a 12-byte block, and the shortlist is cleared. Fewer
/// than three: nothing. The three (names, staff id, club id or −1) go with the year of `today` to
/// `YEAR_AWARD::record_winners`; the block is freed. No shortlist: the nation_awards.cpp 588 box; out of memory: the 596
/// box. Defect of the original: the record's score dwords are never written.
/// Original: `nation_awards_award_from_shortlist` (0x768b20, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_from_shortlist(_this: usize, today: usize, award: usize) {
    nation_awards_award_from_shortlist_with(today, award, &mut |award, record| record_year_award(award, record));
}

/// The year award record's byte ranges the original never writes: each winner's score.
fn year_record_unwritten() -> Vec<(usize, usize)> {
    (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect()
}

/// Comparer for `nation_awards_award_top_three_from_ratings`: the copy runs as a dry run (no record, nothing added to
/// the list), then the original; its newest year award history record must be the planned one, without the scores.
/// No ratings (the error box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_top_three_from_ratings(this: usize, today: usize, award: usize, ratings: usize,
    continent: u32, positions: u32, mode: u32, list: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, usize)>(IDX_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS);
    if ratings == 0 {
        return o(this, today, award, ratings, continent, positions, mode, list);
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_top_three_with(today, award, ratings, continent, positions, mode, list, false, &mut |_, r| planned = Some(r.to_vec())));
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_top_three_from_ratings", history, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned,
        &year_record_unwritten(), || o(this, today, award, ratings, continent, positions, mode, list));
}

/// Comparer for `nation_awards_award_from_shortlist`: the votes draw and the shortlist is cleared, so the shortlist (its
/// 0x40 bytes and its entries) and the random generator are kept; the copy runs as a dry run (no record), they are put
/// back and the original runs; its newest year award history record must be the planned one, without the scores. No
/// shortlist (the error box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_from_shortlist(this: usize, today: usize, award: usize) {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_NATION_AWARDS_AWARD_FROM_SHORTLIST);
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    if shortlist == 0 {
        return o(this, today, award);
    }
    let mut regions = vec![(shortlist, 0x40), (read_usize(shortlist + asl::ENTRIES), read_i16(shortlist + asl::CAPACITY).max(0) as usize * asl::ENTRY_SIZE)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let mut planned = None;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| nation_awards_award_from_shortlist_with(today, award, &mut |_, r| planned = Some(r.to_vec()))));
    before.write_back();
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_from_shortlist", history, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned,
        &year_record_unwritten(), || o(this, today, award));
}

// ---------- sx04b_5: a tournament's player of the year and team of the tournament (nation_awards.cpp) ----------

/// Whether a competition is international (`exe::key_nation_index_of_comp` of its record gives 0 or 1): its awards
/// credit each player's national team instead of his club.
unsafe fn tournament_is_international(comp: League) -> bool {
    let kind = exe::key_nation_index_of_comp(comp.competition());
    kind == 1 || kind == 0
}

/// The fewest starts a tournament award candidate needs: the share of 3 games, cut by `_ftol` to 16 bits.
unsafe fn tournament_least_starts(share: f32) -> i32 {
    ftol(share as f64 * read_f32(rdata::F32_3) as f64) as i16 as i32
}

/// A tournament's player of the year (see `nation_awards_award_tournament_player`), the record going to `sink`.
pub(crate) unsafe fn nation_awards_award_tournament_player_with(comp_index: u32, today: usize, award: usize, age: u32, positions: u32, stat: u32,
    sink: YearAwardSink) {
    let comp = award_league(comp_index);
    if comp.is_null() {
        return;
    }
    let mut share: f32 = f32::from_bits(0x3f28f5c3); // 0.66 of three games
    let international = tournament_is_international(comp);
    let mut n = 0i32;
    let rows = exe::comp_stats_get_sorted_player_table(comp.comp_stats(), stat, &mut n, 0x64, u32::MAX);
    if n == 0 {
        return;
    }
    let mut chosen = [Staff(0); 3];
    let mut scores = [None; 3];
    let mut found = 0usize;
    let mut older = 0i32; // over the age limit: counted across the passes, never reset
    while share as f64 > 0.0 {
        let least = tournament_least_starts(share);
        for k in 0..n.max(0) as usize {
            let row = rows + k * comp_stats_obj::PLAYER_ENTRY_SIZE;
            let person = staff_at(read_i32(row + comp_stats_player::STAFF_ID) as isize);
            if person.player().is_null() {
                continue;
            }
            if positions as u16 != 0 && exe::staff_get_natural_position_mask(person) as u16 & positions as u16 == 0 {
                continue;
            }
            if chosen[..found].contains(&person) {
                continue;
            }
            if (read_u8(row + ROW_APPEARANCES) as i32 - read_u8(row + ROW_SUB_APPEARANCES) as i32) < least { // sx04b_5: enough starts
                continue;
            }
            if age as u8 != 0xff {
                if read_i8(person.addr() + staff::AGE) > age as u8 as i8 {
                    older += 1;
                    continue;
                }
                if older < 3 {
                    continue;
                }
            }
            covered!("sx04b_5 tournament player candidate");
            chosen[found] = person;
            if stat as u8 == 2 {
                scores[found] = Some(read_u8(row + ROW_STAT) as u32);
            }
            found += 1;
            if found == 3 {
                break;
            }
        }
        share = (share as f64 - read_f64(rdata::F64_0_1)) as f32;
        if found == 3 {
            break;
        }
    }
    exe::crt_free(rows);
    if found != 3 {
        return;
    }
    let mut record = [0u8; YEAR_RECORD_SIZE];
    record[..2].copy_from_slice(&read_u16(today + cm_date::YEAR).to_le_bytes());
    for (k, &person) in chosen.iter().enumerate() {
        let club_id = if international {
            let nation = person.nation();
            if nation.is_null() { u32::MAX } else { read_u32(exe::database_get_national_team(nation)) }
        } else {
            let club = person.club_job();
            if club.is_null() { u32::MAX } else { read_u32(club.addr()) }
        };
        year_record_entry(&mut record, k, person, scores[k], club_id);
    }
    sink(award, &record);
}

/// Gives a tournament's player award from its statistics: the first three of the competition's player table for the
/// award's statistic (`comp_stats_get_sorted_player_table`, at most 100) who are players, play one of the `positions`
/// (16 bits; 0 = any) and started at least the share of three games (0.66, down by 0.1 a pass while above 0, the
/// chosen kept). With an `age` (not 0xff), those older are counted and the younger only qualify after three were
/// counted. With all three: the record (the year of `today`, each winner's names, staff id, the statistic for award
/// statistic 2, and his national team's id in an international competition, else his club's, or −1) goes to
/// `YEAR_AWARD::record_winners`. No competition: nothing. Defect of the original: for the other statistics the score
/// dwords are never written (the history keeps stack garbage).
/// Original: `nation_awards_award_tournament_player` (0x7690c0, nation_awards.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_tournament_player(_this: usize, comp_index: u32, today: usize, award: usize, age: u32,
    positions: u32, stat: u32) {
    nation_awards_award_tournament_player_with(comp_index, today, award, age, positions, stat, &mut |award, record| record_year_award(award, record));
}

/// A tournament's team (see `nation_awards_award_tournament_team`), the final record going to `sink`.
pub(crate) unsafe fn nation_awards_award_tournament_team_with(comp_index: u32, award: usize, sink: TeamAwardSink) {
    let comp = award_league(comp_index);
    if comp.is_null() {
        return;
    }
    let mut share: f32 = f32::from_bits(0x3f28f5c3); // 0.66 of three games
    let international = tournament_is_international(comp);
    let mut n = 0i32;
    let rows = exe::comp_stats_get_sorted_player_table(comp.comp_stats(), 0x11, &mut n, u32::MAX, u32::MAX);
    if n == 0 {
        return;
    }
    // the substitutes the award names: those of its club competition, 3 without one
    let mut substitutes = 3u8;
    let table = read_usize(G_COMP_OBJECT_TABLE);
    let club_comp = read_usize(award + award_base::CLUB_COMP);
    if table != 0 && club_comp != 0 {
        let named = League(read_usize(table + read_u32(club_comp) as usize * 4));
        if !named.is_null() {
            substitutes = named.substitutes_named();
        }
    }
    write_u8(award + team_award_obj::SUBSTITUTES, substitutes);
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEAM_AWARD_CLEAR_SELECTION);
    clear(award);
    let add: unsafe extern "thiscall" fn(usize, usize, usize, f32, u32) -> u32 = core::mem::transmute(FN_TEAM_AWARD_ADD_CANDIDATE);
    'passes: loop {
        let least = tournament_least_starts(share);
        for k in 0..n.max(0) as usize {
            let row = rows + k * comp_stats_obj::PLAYER_ENTRY_SIZE;
            let person = staff_at(read_i32(row + comp_stats_player::STAFF_ID) as isize);
            let appearances = read_u8(row + ROW_APPEARANCES) as i32;
            if appearances - read_u8(row + ROW_SUB_APPEARANCES) as i32 <= least { // sx04b_5: more starts than the share
                continue;
            }
            let side = if international { person.national_job() } else { person.club_job() };
            let rating = (read_i16(row + ROW_RATING_SUM) as i32 as f64 / appearances as f64) as f32;
            covered!("sx04b_5 tournament team candidate");
            if add(award, person.addr(), side.addr(), rating, 0) != 0 {
                break 'passes;
            }
        }
        share = (share as f64 - read_f64(rdata::F64_0_1)) as f32;
        if share as f64 <= 0.0 {
            break;
        }
    }
    exe::crt_free(rows);
    let dates: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_TEAM_AWARD_SET_SEASON_DATES);
    dates(award, read_u16(G_CURRENT_DATE + cm_date::YEAR) as u32);
    sink(award);
}

/// Picks a tournament's team: the award's substitutes (`team_award_obj::SUBSTITUTES`) become those its club
/// competition names (3 without one) and the selection is cleared; then, for shares of three games from 0.66 down by
/// 0.1 while above 0, each player of the competition's rating table (statistic 0x11) who started more games than that
/// share is offered to the award (`TEAM_AWARD::add_candidate`) with his national team in an international
/// competition, else his club, and his average rating (rating sum / appearances), until the award reports the team
/// full. The table is freed, the season dates set for the current year and the team recorded
/// (`TEAM_AWARD::record_winners`: history, reputations). No competition or no rated player: nothing.
/// Original: `NationAwards::award_tournament_team` (0x769af0, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_tournament_team(_this: usize, comp_index: u32, award: usize) {
    nation_awards_award_tournament_team_with(comp_index, award, &mut |award| record_team_award(award));
}

/// Comparer for `nation_awards_award_tournament_player`: the copy runs as a dry run (no record), then the original; its
/// newest year award history record must be the planned one (the scores only for statistic 2: the others are never
/// written).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_tournament_player(this: usize, comp_index: u32, today: usize, award: usize, age: u32,
    positions: u32, stat: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, u32, u32)>(IDX_NATION_AWARDS_AWARD_TOURNAMENT_PLAYER);
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_tournament_player_with(comp_index, today, award, age, positions, stat, &mut |_, r| planned = Some(r.to_vec())));
    let unwritten: Vec<(usize, usize)> = if stat as u8 == 2 { Vec::new() } else { (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect() };
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_tournament_player", history, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned,
        &unwritten, || o(this, comp_index, today, award, age, positions, stat));
}

/// Comparer for `nation_awards_award_tournament_team`: the copy runs as a dry run on the award's selection (kept, then
/// put back), then the original; its newest team award history record must be the copy's selection.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_tournament_team(this: usize, comp_index: u32, award: usize) {
    use team_award_obj as ta;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_NATION_AWARDS_AWARD_TOURNAMENT_TEAM);
    let selection = [(award + ta::RECORD, ta::SUBSTITUTES + 1 - ta::RECORD)];
    let before = crate::comparison::MemorySnapshot::read(&selection);
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_tournament_team_with(comp_index, award,
        &mut |a| planned = Some((0..ta::RECORD_SIZE as usize).map(|i| read_u8(a + ta::RECORD + i)).collect::<Vec<u8>>())));
    before.write_back();
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::TEAM_AWARD_FILE);
    compare_award_record("nation_awards_award_tournament_team", history, award + ta::HISTORY_FIRST, award + ta::HISTORY_LAST, planned, &[],
        || o(this, comp_index, award));
}

// ---------- cx16_1: the CONCACAF World Cup qualifying: its constructor, the predicted positions and the knockout team table (wc_concacaf_cup.cpp) ----------

/// The CONCACAF section's vtable (WcConcacafCup).
pub(crate) const WC_CONCACAF_CUP_VTABLE: usize = 0x970c24;

/// The CONCACAF World Cup qualifying at career creation.
///
/// Cup::Cup (`exe::new_cup`), the vtable, the year `year` (word), `cup::CURRENT_ROUND` 0, the main stage, no current
/// stage (−1), 7 stages, the record (`league::COMPETITION`), `cup::CLUB_SIDES` 4, no section above (−1) and the Asian
/// section below (the competition id of [`KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION`]), 7 substitutes named and 3
/// allowed, discipline 0 and importance bias 0xa. If the saved state loads (Cup::load_from_disk `exe::cup_load_from_disk`
/// (this, 1): push 1; mov ecx -> thiscall, ret 4), returns this.
///
/// Otherwise the campaign year (`qualifying_campaign_year`: a year one past the four-year cycle goes back one, then up
/// to the cycle), the stage vector (`exe::crt_malloc` of 4 × 7, cdecl), `cup::RUN_BY_STAGES` 0; the entrants
/// (WcConcacafCup::select_entrants 0x916250; 0: the error box of line 0x5e and this); the statistics (operator new of
/// 0x5ce and Comp_Stats::Comp_Stats into `league::COMP_STATS`; null: the box of line 0x66 and this); the predicted
/// positions (WcConcacafCup::rank_teams 0x917db0) and the knockout team table (WcConcacafCup::build_team_table 0x915d40;
/// 0: the box of line 0x70 and this), all thiscall, ret 0, called directly; the round table of stage −1 (virtual
/// `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0), thiscall, ret 0x10) into
/// `cup::ROUND_TABLE`. A campaign that started before the game's year (`cup::YEAR` below `year`) stays dormant
/// (`cup::SEASON_SET_UP` 0, a dword); otherwise the two Central American groups
/// (WcConcacafCup::create_central_american_groups 0x916cd0). Last the tie tree (Cup::map_fixture_tree
/// `exe::cup_map_fixture_tree`; 0: the box of line 0x8a). Returns this. The SEH frame covers only the statistics' new.
/// Original: `WcConcacafCup::WcConcacafCup` (0x9149f0, wc_concacaf_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_concacaf_cup(this: usize, year: u32, record: usize) -> usize {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    exe::new_cup(cup);
    write_usize(this, WC_CONCACAF_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(stage_index::MAIN);
    cup.set_current_stage(usize::MAX);
    cup.set_stage_count(7);
    write_usize(this + league::COMPETITION, record);
    cup.set_club_sides(4);
    write_u32(this + league::UPPER_DIVISION_ID, u32::MAX);
    write_u32(this + league::LOWER_DIVISION_ID, read_u32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION));
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(0);
    cup.set_importance_bias(0xa);
    if exe::cup_load_from_disk(cup, 1) != 0 {
        return this;
    }
    covered!("cx16_1 concacaf qualifying");
    cup.set_year(qualifying_campaign_year(cup.year())); // cx16_1: the campaign of the four-year cycle
    let n = cup.stage_count() as i32;
    cup.set_stages(if n > 0 { exe::crt_malloc((n as u32) << 2) } else { 0 });
    cup.set_run_by_stages(0);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_COLLECT_ENTRANTS, this) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x5e);
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, cup) } else { 0 };
    write_usize(this + league::COMP_STATS, stats);
    if stats == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x66);
        return this;
    }
    call_section_routine(FN_WC_CONCACAF_CUP_RANK_TEAMS, this);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_SET_UP_SEEDS, this) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x70);
        return this;
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    cup.set_round_table(create_schedule(this, u32::MAX, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0));
    if cup.year() < year as i16 {
        cup.set_season_set_up(0); // cx16_1: a campaign begun before the game stays dormant
    } else {
        call_section_routine(wt::FN_WC_CONCACAF_CUP_CREATE_CENTRAL_AMERICAN_GROUPS, this);
    }
    if exe::cup_map_fixture_tree(cup) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x8a);
    }
    this
}

/// Comparer for `construct_wc_concacaf_cup`: it allocates, reads the save and builds the stages, so only the original
/// runs; then the plan: this returned, the vtable, the record, the campaign year, the Asian section below, 7 stages and,
/// for a new campaign begun before the game's year, a dormant cup (`cup::SEASON_SET_UP` 0).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_concacaf_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_CONCACAF_CUP);
    let loaded = league_will_load_saved_state(record);
    let r = o(this, year, record);
    let cup = Cup(this);
    let expected = qualifying_campaign_year(year as i16);
    let dormant = loaded || expected >= year as i16 || cup.season_set_up() == 0;
    let ok = r == this && read_usize(this) == WC_CONCACAF_CUP_VTABLE && read_usize(this + league::COMPETITION) == record
        && built_year_ok(cup.year() as u16, expected as u16) && read_u32(this + league::LOWER_DIVISION_ID) == read_u32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION)
        && cup.stage_count() == 7 && dormant;
    SCOREBOARD.record(ok, || format!("construct_wc_concacaf_cup({:#x}, {}, {:#x}): returned {:#x}, year {} expected {}, vtable {:#x}, +0x45 {}", this, year as i16, record, r, cup.year(), expected, read_usize(this), cup.season_set_up()));
    r
}

/// WcConcacafCup::rank_teams (thiscall, ret 0), called by the constructor.
pub(crate) const FN_WC_CONCACAF_CUP_RANK_TEAMS: usize = 0x917db0;

/// The predicted positions of the CONCACAF entrants in reputation order, for `n` entrants of which `exempt` are exempt
/// (+0xba): the first 3 − exempt get 1; then, from that place up to the sixth, the place itself plus one (a byte); the
/// seventh to ninth get 7, the tenth to twelfth 10, and from the thirteenth up to n − exempt, 13.
pub(crate) fn concacaf_predicted_positions(n: i32, exempt: i32) -> Vec<u32> {
    let seeded = 3 - exempt;
    let mut v: Vec<u32> = (0..seeded.max(0)).map(|_| 1).collect();
    let mut j = seeded as i16;
    while j < 6 {
        v.push((j as u8).wrapping_add(1) as u32);
        j = j.wrapping_add(1);
    }
    v.extend((6..12).map(|s| if s < 9 { 7 } else { 10 })); // cx16_1: the middle bands
    v.extend((12..n - exempt).map(|_| 13));
    v
}

/// rule: the CONCACAF qualifying predicts the places by reputation: the three best (fewer by the exempt hosts and
/// holder) first, then 4, 5 and 6, the next three 7, the next three 10 and the rest 13; every entrant's national team
/// becomes busy with this competition
///
/// Without statistics (`league::COMP_STATS`), the error box of line 0x7cf; a copy of the entrant list (+0xb2, the count
/// +0xb6 dword; `exe::crt_malloc` of 4 × count, cdecl; out of memory, the box of line 0x7d7) sorted by reputation
/// (`predicted_by_reputation_sort`: qsort with 0x4b07d0). The North American national teams are released
/// (`exe::nationalteams_release_competitions_of_continent`([`G_NATIONAL_TEAMS`], the continent of
/// [`KEY_CONTINENT_NORTH_AMERICA`] in [`G_CONTINENT_TABLE`], 0xc6 bytes each, 0): push 0; push; mov ecx -> thiscall, ret
/// 8); the places of `concacaf_predicted_positions`(count, exempt +0xba, signed byte) go in order to the sorted copy
/// (`exe::comp_stats_set_club_final_position`, thiscall, ret 8; the original pushes the places of the second band with
/// stack garbage in the high bytes: the callee stores only the byte); then the first count − exempt of the copy that
/// are there and have a nation become busy with the record (`exe::nationalteams_set_current_competition`, thiscall,
/// ret 8); the copy freed (`exe::crt_free`).
/// Original: `WcConcacafCup::rank_teams` (0x917db0, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_rank_teams(this: usize) {
    use wc_cup_turnover_consts as wt;
    let stats = read_usize(this + league::COMP_STATS);
    if stats == 0 {
        return show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x7cf);
    }
    let n = read_u32(this + 0xb6); // the entrant count
    let v = exe::crt_malloc(n << 2);
    if v == 0 {
        return show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x7d7);
    }
    core::ptr::copy_nonoverlapping(read_usize(this + wt::CUP_ENTRANT_LIST) as *const u8, v as *mut u8, (n << 2) as usize);
    predicted_by_reputation_sort(v, n);
    let national_teams = read_usize(G_NATIONAL_TEAMS);
    exe::nationalteams_release_competitions_of_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_NORTH_AMERICA).wrapping_mul(0xc6)), 0);
    let exempt = read_u8(this + 0xba) as i8 as i32; // the exempt North Americans (hosts and holder)
    covered!("cx16_1 concacaf predicted positions");
    for (k, p) in concacaf_predicted_positions(n as i32, exempt).into_iter().enumerate() {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + k * 4), p);
    }
    for k in 0..(n as i32 - exempt).max(0) as usize {
        let team = Club(read_usize(v + k * 4));
        if !team.is_null() && !team.nation().is_null() {
            exe::nationalteams_set_current_competition(national_teams, team.nation().addr(), read_usize(this + league::COMPETITION));
        }
    }
    exe::crt_free(v);
}

/// What the predicted positions leave in the game for each entrant of the list: its place in the statistics and its
/// national team's current competition.
pub(crate) unsafe fn concacaf_entrant_marks(this: usize) -> Vec<(usize, u8, usize)> {
    let (stats, list, n) = (read_usize(this + league::COMP_STATS), read_usize(this + 0xb2), read_u32(this + 0xb6) as i32);
    if stats == 0 || list == 0 || !(0..=64).contains(&n) {
        return Vec::new();
    }
    (0..n as usize)
        .map(|k| {
            let team = Club(read_usize(list + k * 4));
            if team.is_null() {
                return (0, 0, 0);
            }
            let e = exe::comp_stats_find_club_entry(stats, team);
            let place = if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 };
            let busy = if !team.nation().is_null() { exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation().addr()) } else { 0 };
            (team.addr(), place, busy)
        })
        .collect()
}

/// Comparer for `wc_concacaf_cup_rank_teams`: the original runs, then the copy again on the state it left (the places
/// and the busy marks are written again the same way); each entrant's place and its national team's competition must
/// not change.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_rank_teams(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_CONCACAF_CUP_RANK_TEAMS)(this);
    let a = concacaf_entrant_marks(this);
    if a.is_empty() {
        return;
    }
    wc_concacaf_cup_rank_teams(this);
    let b = concacaf_entrant_marks(this);
    SCOREBOARD.record(a == b, || format!("wc_concacaf_cup_rank_teams({:#x}): original {:x?} rust {:x?}", this, a, b));
}

/// rule: the CONCACAF qualifying knockout holds the first 24 Caribbean entrants, fewer by the exempt Caribbean nations
///
/// `cup::TEAM_COUNT` = 0x18 − the exempt Caribbean count (+0xbb, signed byte; a word); `cup::TEAM_TABLE` = a table of
/// that many 6-byte rows (`exe::crt_malloc`, cdecl; out of memory, the error box of line 0x350 and −1); row i gets the
/// entrant of slot i of the list (+0xb2) and its seed and status bytes (+4, +5) 0. Returns 1.
/// Original: `WcConcacafCup::build_team_table` (0x915d40, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_build_team_table(this: usize) -> i32 {
    let cup = Cup(this);
    cup.set_team_count((0x18i16).wrapping_sub(read_u8(this + 0xbb) as i8 as i16)); // cx16_1: 24 Caribbean places
    let t = exe::crt_malloc((cup.team_count() as i32 as u32).wrapping_mul(6));
    cup.set_team_table(t);
    if t == 0 {
        show_error_at(wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP, 0x350);
        return -1;
    }
    covered!("cx16_1 concacaf team table");
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let mut i = 0i16;
    while i < cup.team_count() {
        let row = t + i as usize * 6;
        write_usize(row, read_usize(list + i as usize * 4));
        write_u8(row + 4, 0);
        write_u8(row + 5, 0);
        i += 1;
    }
    1
}

/// Comparer for `wc_concacaf_cup_build_team_table`: the original builds its table; the copy then builds its own, which
/// must have the same count and rows; the copy's table is freed and the original's put back.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_build_team_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_WC_CONCACAF_CUP_BUILD_TEAM_TABLE)(this);
    let cup = Cup(this);
    let (t, n) = (cup.team_table(), cup.team_count());
    if o != 1 || t == 0 || !(0..=64).contains(&n) {
        return o;
    }
    let rows = |t: usize, n: i16| core::slice::from_raw_parts(t as *const u8, n as usize * 6).to_vec();
    let a = rows(t, n);
    let r = wc_concacaf_cup_build_team_table(this);
    let (t2, n2) = (cup.team_table(), cup.team_count());
    let b = if t2 != 0 && n2 == n { rows(t2, n2) } else { Vec::new() };
    if t2 != 0 && t2 != t {
        exe::crt_free(t2);
    }
    cup.set_team_table(t);
    cup.set_team_count(n);
    SCOREBOARD.record(r == o && a == b, || format!("wc_concacaf_cup_build_team_table({:#x}): {} teams, original {:x?} rust {:x?}", this, n, a, b));
    o
}

// ---------- cx16_3: the CONCACAF World Cup qualifying entrants, Central American groups and day (wc_concacaf_cup.cpp) ----------

/// The 35 national teams of the 2000 CONCACAF campaign (the 2002 World Cup qualifying), by nation key, in the list's
/// order: the 25 Caribbean slots, then the 10 Central and North American ones.
pub(crate) const WC_CONCACAF_2000_ENTRANTS: [usize; 35] = [
    KEY_NATION_CUBA, KEY_NATION_CAYMAN_ISLANDS, KEY_NATION_SAINT_LUCIA, KEY_NATION_SURINAM, KEY_NATION_ARUBA, KEY_NATION_PUERTO_RICO,
    KEY_NATION_BARBADOS, KEY_NATION_GRENADA, KEY_NATION_ST_VINCENT_THE_GRENADINES, KEY_NATION_US_VIRGIN_ISLANDS, KEY_NATION_ST_KITTS_NEVIS,
    KEY_NATION_TURKS_AND_CAICOS_ISLANDS, KEY_NATION_GUYANA, KEY_NATION_ANTIGUA_BARBUDA, KEY_NATION_BRITISH_VIRGIN_ISLANDS, KEY_NATION_BERMUDA,
    KEY_NATION_TRINIDAD_TOBAGO, KEY_NATION_NETHERLANDS_ANTILLES, KEY_NATION_DOMINICAN_REPUBLIC, KEY_NATION_MONTSERRAT, KEY_NATION_DOMINICA,
    KEY_NATION_HAITI, KEY_NATION_ANGUILLA, KEY_NATION_THE_BAHAMAS, KEY_NATION_JAMAICA, KEY_NATION_HONDURAS, KEY_NATION_EL_SALVADOR,
    KEY_NATION_GUATEMALA, KEY_NATION_PANAMA, KEY_NATION_BELIZE, KEY_NATION_NICARAGUA, KEY_NATION_CANADA, KEY_NATION_COSTA_RICA,
    KEY_NATION_MEXICO, KEY_NATION_UNITED_STATES,
];

/// The nation record of id `id` (`G_NATION_TABLE`, `nation::SIZE` bytes each).
pub(crate) unsafe fn nation_of_id(id: i32) -> Nation {
    Nation(read_usize(G_NATION_TABLE).wrapping_add((id as isize as usize).wrapping_mul(nation::SIZE)))
}

/// Counts an exempt nation into the CONCACAF section when it is North American: +0xba, and +0xbb when its region byte
/// is 6 (the Caribbean [inference]) or +0xbc otherwise.
pub(crate) unsafe fn concacaf_count_exempt(this: usize, nation: Nation) {
    let c = nation.continent();
    if c == 0 || read_usize(c) != read_usize(KEY_CONTINENT_NORTH_AMERICA) {
        return;
    }
    write_u8(this + 0xba, read_u8(this + 0xba).wrapping_add(1));
    let k = if nation.region() == 6 { 0xbb } else { 0xbc };
    write_u8(this + k, read_u8(this + k).wrapping_add(1));
}

/// rule: the CONCACAF qualifying has 25 Caribbean places and 10 Central and North American ones, less the World Cup
/// hosts and holder from North America; the 2000 campaign has its fixed 35 national teams; later campaigns take the
/// North American national teams in database order (and those without a continent), each list sorted
///
/// The exempt counts +0xba, +0xbb and +0xbc start at 0. The hosts of the World Cup two years on (HostCountry::find_hosts
/// `exe::hostcountry_find_hosts`([`G_HOST_COUNTRY`], [`KEY_CLUB_COMP_FIFA_WORLD_CUP`], `cup::YEAR` + 2, &h0, &h1, 1): 5
/// pushes; mov ecx -> thiscall, ret 0x14; without [`G_HOST_COUNTRY`], the error box of line 0x467 and no hosts), up to
/// the first −1, and the last World Cup winner (`exe::comp_util_get_latest_placed_club`(its record, 0), cdecl) when its
/// nation is neither host: each North American one is counted (`concacaf_count_exempt`). The list: +0xb6 = 35 and +0xb2
/// = `exe::crt_calloc`(35, 4) (cdecl; null, the box of line 0x4aa and 0).
///
/// Campaign 2000 (`cup::YEAR`): the national teams (`exe::database_get_national_team`, cdecl) of
/// `WC_CONCACAF_2000_ENTRANTS` in order, and 1. Otherwise, over the nations (`G_NATION_COUNT`), those of North America
/// or without a continent whose id is not a host's and whose national team is not the holder: region 6 into the next of
/// the first 25 slots, region 7 into the next of the last 10, until the Caribbean count reaches 25 − (+0xbb) and the other
/// 10 − (+0xbc). Short of either, 0; full, each part sorted (qsort `exe::crt_qsort`(part, count, 4, 0x4b0810), cdecl)
/// and 1.
/// Original: `WcConcacafCup::select_entrants` (0x916250, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_select_entrants(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    write_u8(this + 0xba, 0); // the exempt North Americans
    write_u8(this + 0xbb, 0); // of them, the Caribbean ones
    write_u8(this + 0xbc, 0); // and the others
    let mut hosts = [-1i32; 2];
    let host_country = read_usize(G_HOST_COUNTRY);
    if host_country == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x467);
    } else {
        let (a, b) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(host_country, read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), (cup.year() as u16).wrapping_add(2) as u32, &mut a[0], &mut b[0], 1) as i8;
        for k in 0..n.max(0) as usize {
            let h = hosts.get(k).copied().unwrap_or(-1); // ponytail: find_hosts gives at most the two slots
            if h == -1 {
                break;
            }
            concacaf_count_exempt(this, nation_of_id(h));
        }
    }
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let champion = exe::comp_util_get_latest_placed_club(world_cup, 0);
    if champion != 0 && !Club(champion).nation().is_null() {
        let nation = Club(champion).nation();
        let id = read_u32(nation.addr()) as i32;
        if id != hosts[0] && id != hosts[1] {
            concacaf_count_exempt(this, nation);
        }
    }
    write_u32(this + 0xb6, 0x23); // the 35 places
    let list = exe::crt_calloc(0x23, 4);
    write_usize(this + wt::CUP_ENTRANT_LIST, list);
    if list == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x4aa);
        return 0;
    }
    if cup.year() == 0x7d0 {
        covered!("cx16_3 concacaf 2000 entrants");
        for (k, &key) in WC_CONCACAF_2000_ENTRANTS.iter().enumerate() {
            write_usize(list + k * 4, exe::database_get_national_team(nation_of_id(read_u32(key) as i32)));
        }
        return 1;
    }
    let caribbean_places = 0x19 - read_u8(this + 0xbb) as i8 as i32;
    let other_places = 0xa - read_u8(this + 0xbc) as i8 as i32;
    let (mut caribbean, mut others) = (0i32, 0i32);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = nation_of_id(i as i32);
        let c = nation.continent();
        if c == 0 || read_usize(c) == read_usize(KEY_CONTINENT_NORTH_AMERICA) {
            let team = exe::database_get_national_team(nation);
            let id = read_u32(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != champion {
                let region = nation.region();
                if region == 6 && caribbean < 0x19 {
                    write_usize(list + caribbean as usize * 4, team);
                    caribbean += 1;
                } else if region == 7 && others < 0xa {
                    write_usize(list + 0x64 + others as usize * 4, team);
                    others += 1;
                }
                if caribbean == caribbean_places && others == other_places {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if caribbean != caribbean_places || others != other_places {
        return 0;
    }
    covered!("cx16_3 concacaf entrants");
    exe::crt_qsort(list, caribbean as u32, 4, 0x4b0810);
    exe::crt_qsort(list + 0x64, others as u32, 4, 0x4b0810); // cx16_3: the Central and North American part sorted
    1
}

/// What the entrants routine leaves: its result, the three exempt counts, the count and the 35 slots.
pub(crate) unsafe fn concacaf_entrants_state(this: usize, r: u32) -> (u32, [u8; 3], u32, Vec<usize>) {
    let list = read_usize(this + 0xb2);
    let slots = if list != 0 { (0..0x23).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    (r, [read_u8(this + 0xba), read_u8(this + 0xbb), read_u8(this + 0xbc)], read_u32(this + 0xb6), slots)
}

/// Comparer for `wc_concacaf_cup_select_entrants`: the original makes its list; the copy then makes its own, which must
/// match it with the same result and counts; the copy's list is freed and the original's state put back.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_select_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_CONCACAF_CUP_SELECT_ENTRANTS)(this);
    let list = read_usize(this + 0xb2);
    let a = concacaf_entrants_state(this, r);
    let r2 = wc_concacaf_cup_select_entrants(this);
    let b = concacaf_entrants_state(this, r2);
    let list2 = read_usize(this + 0xb2);
    if list2 != 0 && list2 != list {
        exe::crt_free(list2);
    }
    write_usize(this + 0xb2, list);
    write_u8(this + 0xba, a.1[0]);
    write_u8(this + 0xbb, a.1[1]);
    write_u8(this + 0xbc, a.1[2]);
    write_u32(this + 0xb6, a.2);
    SCOREBOARD.record(a == b, || format!("wc_concacaf_cup_select_entrants({:#x}): original {:x?} rust {:x?}", this, a, b));
    r
}

/// One Central American group: its round table (virtual `comp_vtable::CREATE_SCHEDULE`(stage, &rounds, &label, 0):
/// thiscall, ret 0x10) and the League_Stage (`exe::crt_operator_new` of 0xee and League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the section, `n`, the teams, 2, the record, the round table, the
/// rounds, 3 and 1 points, category 0xa, the tie-breaks {1, 2, 3, 0}, the places {1, 1, 0, 0}, the year, `index`, the
/// label, importance 0xf, 4, 0, flags 0x28, −1, 0, 2; the original pushes the rounds and the label with garbage in the
/// high bytes, which the constructor does not read: the copy passes clean words). Returns (stage or 0, round table).
pub(crate) unsafe fn concacaf_group_stage(this: usize, stage: u32, n: u32, teams: usize, index: u32) -> (usize, usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let tiebreaks: [u8; 4] = [1, 2, 3, 0];
    let places: [u8; 4] = [1, 1, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = create_schedule(this, stage, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        construct(p, this, n, teams, 2, read_usize(this + league::COMPETITION), table, rounds as u16 as u32, 3, 1, 0xa, tiebreaks.as_ptr() as usize, places.as_ptr() as usize,
            Cup(this).year() as u16 as u32, index, label as u16 as u32, 0xf, 4, 0, 0x28, -1, 0, 2)
    } else {
        0
    };
    (s, table)
}

/// Moves the entrants of the given slots of the list (+0xb2) into `teams`, leaving the slots empty.
pub(crate) unsafe fn take_concacaf_slots(this: usize, slots: &[usize], teams: usize) {
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    for (k, &s) in slots.iter().enumerate() {
        write_usize(teams + k * 4, read_usize(list + s * 4));
        write_usize(list + s * 4, 0);
    }
}

/// The list slots of the two Central American groups: the first takes slots 25 and 27 (and 29 with fewer than two
/// exempt non-Caribbean nations), the second 26 and 28 (and 30 with none).
pub(crate) fn concacaf_group_slots(others: i8) -> ([usize; 3], usize, [usize; 3], usize) {
    let n1 = 2 + (others < 2) as usize;
    let n2 = 2 + (others < 1) as usize;
    ([25, 27, 29], n1, [26, 28, 30], n2) // cx16_3: the alternate slots of the two groups
}

/// rule: the two Central American groups of the CONCACAF qualifying take the seeded slots in turn: the first slots 25,
/// 27 and 29, the second 26, 28 and 30 (three nations each, or two when the exempt nations take places), 3 points a win
/// and 1 a draw
///
/// A team array (`exe::crt_malloc` of 0xc, cdecl; null, the error box of line 0x572). The first group (stage 0): its
/// slots (`concacaf_group_slots` by +0xbc, signed byte) moved into the array (`take_concacaf_slots`) and the group
/// built (`concacaf_group_stage` with index 0) into `cup::STAGES`[0] (null: the box of line 0x591, ending), its round
/// table freed (`exe::crt_free`) and its draw news sent (`exe::league_send_group_draw_news`). The second (stage 1) the
/// same with index 1 into `cup::STAGES`[1] (null: the box of line 0x5b3). Then the team array freed,
/// `cup::CURRENT_STAGE` = 1 and the second group's draw news sent again. Defect of the original: the second group's
/// draw news goes out twice. Returns 1 (the original the eax of the news call, unread by the constructor; the turnover
/// passes it on), 0 after a box.
/// Original: `WcConcacafCup::create_central_american_groups` (0x916cd0, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_create_central_american_groups(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    let teams = exe::crt_malloc(0xc);
    if teams == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x572);
        return 0;
    }
    covered!("cx16_3 central american groups");
    let (first, n1, second, n2) = concacaf_group_slots(read_u8(this + 0xbc) as i8);
    take_concacaf_slots(this, &first[..n1], teams);
    let (s, table) = concacaf_group_stage(this, 0, n1 as u32, teams, 0);
    write_usize(cup.stages(), s);
    if s == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x591);
        return 0;
    }
    exe::crt_free(table);
    exe::league_send_group_draw_news(s);
    take_concacaf_slots(this, &second[..n2], teams);
    let (s, table) = concacaf_group_stage(this, 1, n2 as u32, teams, 1);
    write_usize(cup.stages() + 4, s);
    if s == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x5b3);
        return 0;
    }
    exe::crt_free(table);
    exe::league_send_group_draw_news(s);
    exe::crt_free(teams);
    cup.set_current_stage(1);
    exe::league_send_group_draw_news(s);
    1
}

/// Comparer for `wc_concacaf_cup_create_central_american_groups`: it builds the stages and sends news, so only the
/// original runs; then the plan: the current stage 1, the moved slots empty and each group's table with the entrants
/// of its slots (taken before).
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_create_central_american_groups(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_CONCACAF_CUP_CREATE_CENTRAL_AMERICAN_GROUPS);
    let list = read_usize(this + 0xb2);
    let stages = Cup(this).stages();
    if list == 0 || stages == 0 {
        return o(this);
    }
    let (first, n1, second, n2) = concacaf_group_slots(read_u8(this + 0xbc) as i8);
    let plan = |slots: &[usize]| {
        let mut v: Vec<usize> = slots.iter().map(|&s| read_usize(list + s * 4)).collect();
        v.sort_unstable();
        v
    };
    let (p1, p2) = (plan(&first[..n1]), plan(&second[..n2]));
    let r = o(this);
    let teams_of = |s: usize| {
        let g = League(s);
        let mut v: Vec<usize> = if !g.is_null() && g.table() != 0 { (0..g.club_count().max(0) as usize).map(|k| read_usize(g.table() + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        v.sort_unstable();
        v
    };
    let (g1, g2) = (teams_of(read_usize(stages)), teams_of(read_usize(stages + 4)));
    let empty = first[..n1].iter().chain(second[..n2].iter()).all(|&s| read_usize(list + s * 4) == 0);
    let ok = Cup(this).current_stage() == 1 && empty && g1 == p1 && g2 == p2;
    SCOREBOARD.record(ok, || format!("wc_concacaf_cup_create_central_american_groups({:#x}): groups {:x?} {:x?} plan {:x?} {:x?}, stage {}", this, g1, g2, p1, p2, Cup(this).current_stage() as i32));
    r
}

/// Whether `today` (the date's day and year words) is 10 January of the CONCACAF campaign year (`cup::YEAR`; the
/// date built by Date::Date `exe::date_date_549f70`(&date, 10, 0, year, −1): 4 pushes; lea ecx -> thiscall, ret 0x10).
pub(crate) unsafe fn concacaf_busy_day(this: usize, today: usize) -> bool {
    let mut date = [0u16; 4];
    exe::date_date_549f70(&mut date, 0xa, 0, Cup(this).year() as u16 as u32, u32::MAX);
    read_u16(today + 2) == date[1] && read_u16(today) == date[0]
}

/// rule: on 10 January of the campaign year, with the full update, every CONCACAF entrant and every team already in a
/// stage becomes busy with the qualifying; then the cup's day
///
/// On that day (`concacaf_busy_day`) with `full`: the North American national teams released
/// (`exe::nationalteams_release_competitions_of_continent`, the continent of [`KEY_CONTINENT_NORTH_AMERICA`]); each
/// non-null entrant of the list (+0xb2, the count +0xb6) busy with the record
/// (`exe::nationalteams_set_current_competition`([`G_NATIONAL_TEAMS`], its nation, `league::COMPETITION`), thiscall,
/// ret 8; the nation not checked); then, when `cup::CURRENT_STAGE` is −1 or more, for each stage index from −1 to the
/// current one: −1 the cup's own team table (`cup::TEAM_TABLE`, 6-byte rows, `cup::TEAM_COUNT`), the others the table of
/// that stage (`league::TABLE`, `league_table_row::SIZE`-byte rows, `league::CLUB_COUNT`), each team's nation busy too.
/// Then Cup::update_for_date (`exe::cup_update_for_date`(this, today, full), thiscall, ret 8), whose eax it returns.
/// Original: `WcConcacafCup::update_for_date` (0x915e40, wc_concacaf_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let cup = Cup(this);
    if concacaf_busy_day(this, today) && full != 0 { // cx16_3: the busy day
        covered!("cx16_3 concacaf busy day");
        let national_teams = read_usize(G_NATIONAL_TEAMS);
        let record = read_usize(this + league::COMPETITION);
        exe::nationalteams_release_competitions_of_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_NORTH_AMERICA).wrapping_mul(0xc6)), 0);
        let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
        let mut k = 0i16;
        while (k as i32) < read_u32(this + 0xb6) as i32 {
            let team = Club(read_usize(list + k as usize * 4));
            if !team.is_null() {
                exe::nationalteams_set_current_competition(national_teams, team.nation().addr(), record);
            }
            k = k.wrapping_add(1);
        }
        let current = cup.current_stage() as i32;
        let mut s = -1i8;
        while current >= -1 && (s as i32) <= current {
            if s == -1 {
                for i in 0..cup.team_count().max(0) as usize {
                    exe::nationalteams_set_current_competition(national_teams, Club(read_usize(cup.team_table() + i * 6)).nation().addr(), record);
                }
            } else {
                let g = League(read_usize(cup.stages() + s as usize * 4));
                for i in 0..g.club_count().max(0) as usize {
                    exe::nationalteams_set_current_competition(national_teams, Club(read_usize(g.table() + i * league_table_row::SIZE)).nation().addr(), record);
                }
            }
            s = s.wrapping_add(1);
        }
    }
    exe::cup_update_for_date(cup, today, full)
}

/// Comparer for `wc_concacaf_cup_update_for_date`: the cup's day cannot run twice; only the original, and on the busy
/// day with the full update every entrant's national team must be busy with the qualifying.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let busy = full != 0 && concacaf_busy_day(this, today);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_CONCACAF_CUP_UPDATE_FOR_DATE)(this, today, full);
    if busy {
        let record = read_usize(this + league::COMPETITION);
        let list = read_usize(this + 0xb2);
        let n = read_u32(this + 0xb6) as i32;
        let free: Vec<usize> = (0..n.clamp(0, 64) as usize)
            .map(|k| Club(read_usize(list + k * 4)))
            .filter(|t| !t.is_null() && !t.nation().is_null() && exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), t.nation().addr()) != record)
            .map(|t| t.addr())
            .collect();
        SCOREBOARD.record(free.is_empty(), || format!("wc_concacaf_cup_update_for_date({:#x}): entrants not busy {:x?}", this, free));
    }
    r
}

// ---------- cx16_4: the European World Cup qualifying constructor (wc_europe_league.cpp) ----------

/// The European section's vtable (WcEuropeLeague).
pub(crate) const WC_EUROPE_LEAGUE_VTABLE: usize = 0x970cc4;

/// WcEuropeLeague routines the constructor calls directly (thiscall, ret 0): the rules and the 2002 campaign.
pub(crate) const FN_WC_EUROPE_LEAGUE_SET_UP_RULES: usize = 0x9191e0;
pub(crate) const FN_WC_EUROPE_LEAGUE_LOAD_2002_CAMPAIGN: usize = 0x91b760;

/// The European campaign year of a game year: the four-year cycle (`qualifying_campaign_year`), never before 2004.
pub(crate) fn wc_europe_campaign_year(year: i16) -> i16 {
    qualifying_campaign_year(year).max(0x7d4) // cx16_4: the European campaign, from 2004
}

/// The European World Cup qualifying at career creation.
///
/// League::League (`exe::new_league`), the vtable, the year `year` (word), the record (`league::COMPETITION`),
/// `league::CLUB_SIDES` 4, the African section above (the competition id of
/// [`KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION`]) and none below (−1), the dwords +0xf7, +0xee, +0xf2 and +0xfb
/// 0, one exempt nation (`WC_EUROPE_EXEMPT` 1), discipline 0; the campaign year (`wc_europe_campaign_year`); a new
/// list (`exe::crt_operator_new(0xe)` and LLIST::LLIST `exe::new_llist`, or 0) in `wc_cup_turnover_consts::EUROPE_LIST`
/// and importance 0xf. If the saved state loads (League::load_from_temp_file `exe::league_load_from_temp_file`(this, 1)),
/// returns this.
///
/// Otherwise `league::LEAGUE_FLAGS` 0x24; the entry pool (WcEuropeLeague::build_entry_pool 0x919e30; its result not
/// read); the stage count by the exempt count (0 → 11, 1 or 2 → 10, 3 → 9, any other kept); the main stage, no current
/// stage (−1), the stage vector (`exe::crt_malloc` of 4 × count when positive, else null), `league::ACTIVE` 0 (dword);
/// the rules (WcEuropeLeague::set_up_rules 0x9191e0; 0: the error box of line 0x7f and this); the statistics (operator
/// new of 0x5ce and Comp_Stats::Comp_Stats into `league::COMP_STATS`; null: the box of line 0x87 and this). A campaign
/// year not after the game's (never: the campaign is at least 2004 and the game starts in 2001) loads the 2002 campaign
/// (WcEuropeLeague::load_2002_campaign 0x91b760). Returns this. The SEH frame covers only the two operator new.
/// Original: `WcEuropeLeague::WcEuropeLeague` (0x918200, wc_europe_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_europe_league(this: usize, year: u32, record: usize) -> usize {
    use wc_cup_turnover_consts as wt;
    let league = League(this);
    exe::new_league(league);
    write_usize(this, WC_EUROPE_LEAGUE_VTABLE);
    league.set_year(year as i16);
    write_usize(this + league::COMPETITION, record);
    league.set_club_sides(4);
    league.set_upper_division_id(read_u32(KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION) as i32);
    league.set_lower_division_id(usize::MAX);
    write_u32(this + 0xf7, 0); // the drawn groups [inference]
    write_u32(this + 0xee, 0); // the entry pool list
    write_u32(this + 0xf2, 0); // its count
    write_u32(this + wt::EUROPE_FB, 0);
    write_u8(this + WC_EUROPE_EXEMPT, 1);
    league.set_discipline_rule(0);
    league.set_year(wc_europe_campaign_year(year as i16));
    let p = exe::crt_operator_new(0xe);
    write_usize(this + wt::EUROPE_LIST, if p != 0 { exe::new_llist(p) } else { 0 });
    league.set_importance(0xf);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cx16_4 european qualifying");
    league.set_league_flags(0x24);
    call_section_routine(wt::FN_WC_EUROPE_LEAGUE_BUILD_ENTRY_POOL, this);
    match read_u8(this + WC_EUROPE_EXEMPT) {
        0 => league.set_stage_count(0xb),
        1 | 2 => league.set_stage_count(0xa),
        3 => league.set_stage_count(9),
        _ => {}
    }
    league.set_stage_index(stage_index::MAIN);
    league.set_current_stage(usize::MAX);
    let n = league.stage_count() as i32;
    league.set_stages(if n > 0 { exe::crt_malloc((n as u32) << 2) } else { 0 });
    league.set_active(0);
    if call_section_routine(FN_WC_EUROPE_LEAGUE_SET_UP_RULES, this) == 0 {
        show_error_at(wt::WC_EUROPE_LEAGUE_CPP, 0x7f);
        return this;
    }
    if !qualifying_section_stats(league, wt::WC_EUROPE_LEAGUE_CPP, 0x87) {
        return this;
    }
    if year as i16 >= league.year() {
        call_section_routine(FN_WC_EUROPE_LEAGUE_LOAD_2002_CAMPAIGN, this);
    }
    this
}

/// Comparer for `construct_wc_europe_league`: it allocates, reads the save and builds the pool, so only the original
/// runs; then the plan: this returned, the vtable, the record, the campaign year (`wc_europe_campaign_year`), the
/// African section above and none below.
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_europe_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_EUROPE_LEAGUE);
    let r = o(this, year, record);
    let league = League(this);
    let expected = wc_europe_campaign_year(year as i16);
    let ok = r == this && read_usize(this) == WC_EUROPE_LEAGUE_VTABLE && league.competition().addr() == record && built_year_ok(league.year() as u16, expected as u16)
        && league.upper_division_id() as u32 == read_u32(KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION) && league.lower_division_id() as u32 == u32::MAX;
    SCOREBOARD.record(ok, || format!("construct_wc_europe_league({:#x}, {}, {:#x}): returned {:#x}, year {} expected {}, vtable {:#x}", this, year as i16, record, r, league.year(), expected, read_usize(this)));
    r
}
