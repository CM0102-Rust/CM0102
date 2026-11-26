use super::*;

/// Builds a year award: the award base (with the file and the player flag as its last two arguments), type 1, the
/// kind (`kind`) and option (`option`) bytes, no shortlist and no history (−1, −1). With a file,
/// `YEAR_AWARD::load_from_disk(scope, file)`; when there is none or the load fails, a scope gets a new shortlist
/// (`construct_award_shortlist`(scope, no file) in 0x40 bytes of `exe::crt_operator_new`; 0 when out of memory).
/// Returns this.
/// Original: `YEAR_AWARD::YEAR_AWARD` (0x92fe80, year_award.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn year_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, kind: u32, scope: usize, option: u32, file: usize, for_players: u32) -> usize {
    use year_award_obj as ya;
    award_construct_base(this, [nation, index, staff_comp, national_rep, world_rep, club_comp, file as u32, for_players]);
    write_usize(this, ya::VTABLE);
    write_usize(this + ya::SHORTLIST, 0);
    write_u8(this + ya::KIND, kind as u8);
    write_u8(this + ya::OPTION, option as u8);
    write_i32(this + ya::HISTORY_FIRST, -1);
    write_i32(this + ya::HISTORY_LAST, -1);
    write_u8(this + award_base::TYPE, 1);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_YEAR_AWARD_LOAD_FROM_DISK);
        if load(this, scope, file) != 0 {
            return this;
        }
    }
    if scope != 0 {
        covered!("sx05b_5 year award shortlist");
        let p = exe::crt_operator_new(0x40);
        write_usize(this + ya::SHORTLIST, if p != 0 { construct_award_shortlist(p, scope, 0) } else { 0 });
    }
    this
}

pub(crate) unsafe extern "thiscall" fn compare_year_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, kind: u32, scope: usize, option: u32, file: usize, for_players: u32) -> usize {
    use year_award_obj as ya;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, u32, usize, u32) -> usize>(IDX_YEAR_AWARD_CONSTRUCT);
    let r = o(this, nation, index, staff_comp, national_rep, world_rep, club_comp, kind, scope, option, file, for_players);
    compare_award_built(r, this, file, || {
        read_usize(this) == ya::VTABLE && read_u8(this + ya::KIND) == kind as u8 && read_u8(this + ya::OPTION) == option as u8
            && read_i32(this + ya::HISTORY_FIRST) == -1 && read_i32(this + ya::HISTORY_LAST) == -1 && read_u8(this + award_base::TYPE) == 1
            && (read_usize(this + ya::SHORTLIST) != 0) == (scope != 0)
    }, || format!("year_award_construct(nation {}, index {}, kind {}, scope {:#x}): vtable {:#x} shortlist {:#x}", nation, index, kind, scope,
        read_usize(this), read_usize(this + ya::SHORTLIST)))
}

/// `YEAR_AWARD::load_from_disk(scope, file)` (0x92fff0, thiscall, ret 8): 1 when read.
pub(crate) const FN_YEAR_AWARD_LOAD_FROM_DISK: usize = 0x92fff0;

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

/// Comparer for `year_award_record_winners`: the history, the reputations and the staff records (with their draws)
/// cannot be written twice, so only the original runs; the history last offset must have moved.
pub(crate) unsafe extern "thiscall" fn compare_year_award_record_winners(this: usize, record: usize, post: u32) {
    let before = read_u32(this + year_award_obj::HISTORY_LAST);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_YEAR_AWARD_RECORD_WINNERS)(this, record, post);
    let after = read_u32(this + year_award_obj::HISTORY_LAST);
    SCOREBOARD.record(after != before, || format!("year_award_record_winners({:#x}, post {}): history last {:#x} -> {:#x}", this, post, before, after));
}

/// Comparer for `year_award_post_winner_news`: the original posts; the copy rehearses it (`rehearse_news`) and its
/// draws (the template, and the text's with the news texts mode) must be the original's first ones.
pub(crate) unsafe extern "thiscall" fn compare_year_award_post_winner_news(this: usize, record: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_YEAR_AWARD_POST_WINNER_NEWS);
    let mut posts = Vec::new();
    let (ok, a, b) = rehearse_news(|| o(this, record), || year_award_post_winner_news_with(this, record, &mut |_, to| posts.push(to)));
    SCOREBOARD.record(ok && !b.is_empty() && !posts.is_empty(), || format!("year_award_post_winner_news({:#x}) draws original {:?} rust {:?}, rust posts {:x?}", this, a, b, posts));
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
    slot(0x14, winner.non_player().addr() as u32);
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
            let nation = Nation(read_usize(c + club::NATION));
            if comp == 0 || nation.is_null() || nation.continent() != read_usize(comp + 0x53) {
                sink(p, NewsPost::Club(c));
            }
        }
    } else {
        sink(p, NewsPost::All);
    }
}

/// Writes the body of the year award news into the shared buffer: the plain template (the winner, the award, the
/// period, the second) for an old item, a winner whose first name id is a multiple of 10, a winner with a non-player
/// sheet or the French most unpopular player award; otherwise by the third winner (present or not), the draw (0, 1, other) and the
/// winner's club (present or not) one of twelve templates, the club-less ones with the winner's description
/// (`describe_winner`) and the others with his club's short name and position text.
pub(crate) unsafe fn year_news_body(this: usize, n: &mut YearNews) {
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
pub(crate) unsafe fn year_news_headline(this: usize, n: &YearNews) {
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
pub(crate) unsafe fn year_news_links(this: usize, n: &YearNews, text: usize) {
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

/// `YEAR_AWARD::post_winner_news(record)` (0x9304f0, thiscall, ret 4), called by `year_award_record_winners` through the
/// hook.
pub(crate) const FN_YEAR_AWARD_POST_WINNER_NEWS: usize = 0x9304f0;

/// What the year award news text reads from its item (the slots of `year_award_post_winner_news`) and builds before
/// choosing a template: the three winners (staff id and three name ids) and their club ids, whether the item is
/// current, the template draw, the non-player flag, the winner's position mask and strength flags, the names (style
/// 1, and the winner's in style 5), the winner's position text with its gender byte, and the period.
pub(crate) struct YearNews {
    pub(crate) winners: [[u32; 4]; 3],
    pub(crate) clubs: [i32; 3],
    pub(crate) current: bool,
    pub(crate) draw: i8,
    pub(crate) non_player: u32,
    pub(crate) mask: u32,
    pub(crate) flags: u32,
    pub(crate) names: [[u8; 0xc8]; 3],
    pub(crate) winner_short: [u8; 0xc8],
    pub(crate) position: [u8; 0xc8],
    pub(crate) position_gender: u8,
    pub(crate) period: [u8; 0x14],
}

impl YearNews {
    /// Reads the item's slots 2 to 0x16 in order, then, as the original does before any template: the winner's
    /// position text (`exe::write_position_name`(mask, text, 0xc8, 1), its gender byte; only for a current item whose
    /// winner has no non-player sheet), the names (`exe::database_format_name_from_ids`, 0xc8 bytes: the winner in styles 1 and 5, the
    /// second, and the third only in a current item with a third), and the period from the year: "%d/%02d" (the year
    /// less the award's kind byte, then the year modulo 100) when the kind byte is not 0, else "%d".
    pub(crate) unsafe fn read(this: usize, item: usize) -> YearNews {
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

    pub(crate) fn name(&self, k: usize) -> usize {
        self.names[k].as_ptr() as usize
    }

    /// The winner's description in place of the position text (`exe::write_player_description`(id, mask, flags, text,
    /// 0xc8, the position gender, 1)) and its gender byte, sign-extended; the club-less templates use it.
    pub(crate) unsafe fn describe_winner(&mut self) -> u32 {
        let g = exe::write_player_description(self.winners[0][0] as i32, self.mask, self.flags, self.position.as_mut_ptr() as usize, 0xc8, self.position_gender as u32, 1);
        g as i8 as i32 as u32
    }
}

/// The slot 0xd value of a current year award news item (`year_award_post_winner_news`); older items carry another
/// value and get the plain texts, no position text and no third winner.
pub(crate) const YEAR_NEWS_CURRENT: u32 = 0x420e9;
