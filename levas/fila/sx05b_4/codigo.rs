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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%);
    compare_text_writer(text, || o(this, item, text, body), |t| year_award_write_news(this, item, t, body),
        || format!("year_award_write_news(draw {}, body {})", exe::newsitem_get_slot(item, 0x13) as u8 as i8, body));
}
