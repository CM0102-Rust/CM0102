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
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, record, post);
    let after = read_u32(this + year_award_obj::HISTORY_LAST);
    SCOREBOARD.record(after != before, || format!("year_award_record_winners({:#x}, post {}): history last {:#x} -> {:#x}", this, post, before, after));
}

/// Comparer for `year_award_post_winner_news`: the original posts; the copy rehearses it (`rehearse_news`) and its one
/// draw (the template) must be the original's first and the only one it makes.
pub(crate) unsafe extern "thiscall" fn compare_year_award_post_winner_news(this: usize, record: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let mut posts = Vec::new();
    let (ok, a, b) = rehearse_news(|| o(this, record), || year_award_post_winner_news_with(this, record, &mut |_, to| posts.push(to)));
    SCOREBOARD.record(ok && b.len() == 1 && !posts.is_empty(), || format!("year_award_post_winner_news({:#x}) draws original {:?} rust {:?}, rust posts {:x?}", this, a, b, posts));
}
