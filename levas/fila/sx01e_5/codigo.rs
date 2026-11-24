// --- sx01e_5: the media text writers (media.cpp): signing celebration, trophy praise, gratitude ---

/// The name of a club's stadium record (`club::STADIUM`: +4 the name, +0x37 its gender byte).
unsafe fn club_stadium_name(c: Club) -> (i32, usize) {
    let stadium = read_usize(c.addr() + club::STADIUM);
    (read_i8(stadium + 0x37) as i32, stadium + 4)
}

/// What news item 0xbe5 (a new signing welcomed) holds: slots 0..3 the player (full and short, 0x190), 4 the club, 5
/// the form, 6 the outlet type, 7 the pundit, 8 his position, 9 who welcomes him (1 the fans, 2 a squad spokesman,
/// other the tea lady); with 2, slots 11..14 the spokesman (full name), otherwise the name "Jake Vaughan"; then, from
/// slot 11 or 15, the mark of a named person, the person and his names (short).
struct SigningText {
    player: NewsName,
    player_short: NewsName,
    club: Club,
    form: u32,
    who: i32,
    spokesman_id: i32,
    spokesman: NewsName,
    named: u32,
    person: NewsName,
    position: [u8; 0xc8],
    position_gender: i32,
    outlet: [u8; 0x78],
    outlet_gender: i32,
}

/// Reads news item 0xbe5 and writes the names, the position (`exe::write_position_name`) and the outlet
/// (`media_story_outlet`).
unsafe fn signing_text_read(news: usize) -> SigningText {
    let s = |k: u32| exe::newsitem_get_slot(news, k);
    let club_index = s(4) as i32;
    let who = s(9) as i32;
    let (spokesman_id, spokesman, k) = if who == 2 {
        (s(11) as i32, NewsName::read(news, 11, 0x190, 1), 15)
    } else {
        let mut n = NewsName { id: 0, names: [0; 3], text: [0; 0x190] };
        story_crt!(n.text.as_mut_ptr(), 0xa592c8); // "Jake Vaughan"
        (0, n, 11)
    };
    let named = s(k);
    let person = NewsName::new(s(k + 1), [s(k + 2), s(k + 3), s(k + 4)], 0x190, 5);
    let player = NewsName::read(news, 0, 0x190, 1);
    let player_short = player.restyled(0x190, 5);
    let mut t = SigningText {
        player, player_short, club: club_at(club_index as isize), form: s(5), who, spokesman_id, spokesman, named, person,
        position: [0; 0xc8], position_gender: 0, outlet: [0; 0x78], outlet_gender: 0,
    };
    t.position_gender = exe::write_position_name(s(8), t.position.as_mut_ptr() as usize, 0xc8, 1) as i8 as i32;
    t.outlet_gender = media_story_outlet(t.club, club_index, s(6) as i32, s(7) as i32, &mut t.outlet) as i8 as i32;
    t
}

/// Writes the body of news item 0xbe5: the squad (2) with or without a named spokesman and by the form, the stadium's
/// name in the texts; the fans (1) with the named person, or by the form with the position (and the outlet); others
/// the tea lady ("Mary the tea lady was pleased by signing of %s ..."). Links: the club's short and long names, the
/// spokesman (2) and the player's two names when they match.
unsafe fn signing_body(s: &SigningText, t: &mut MediaText) {
    let (lg, long) = club_long_name(s.club);
    let (sg, short) = club_short_name(s.club);
    let (p, spokes, person) = (s.player.ptr(), s.spokesman.ptr(), s.person.ptr());
    match s.who {
        2 => {
            covered!("sx01e_5 signing text squad");
            let (stg, stadium) = club_stadium_name(s.club);
            match (s.named != 0, s.form != 0) {
                (true, true) => media_text!(t, 0xa59144, lg, lg, 0x64, stg, stg, 0x64, long, spokes, stadium, p),
                (true, false) => media_text!(t, 0xa58fe4, lg, lg, 0x64, stg, stg, 0x64, sg, sg, long, spokes, stadium, p, short),
                (false, true) => media_text!(t, 0xa58df0, lg, lg, 0x64, stg, stg, 0x64, sg, sg, long, p, stadium, spokes, short),
                (false, false) => media_text!(t, 0xa58d0c, lg, lg, 0x64, 0x64, long, p, spokes),
            }
        }
        1 => {
            covered!("sx01e_5 signing text fans");
            let (pg, og) = (s.position_gender, s.outlet_gender);
            let pos = s.position.as_ptr() as usize;
            if s.named != 0 {
                media_text!(t, 0xa58b9c, lg, lg, 0x64, 0x64, long, person, p);
            } else if s.form != 0 {
                media_text!(t, 0xa58a78, lg, lg, 0x64, pg, pg, 0x64, sg, sg, long, person, pos, p, short);
            } else {
                media_text!(t, 0xa588cc, lg, lg, pg, pg, 0x64, og, og, 0x64, sg, sg, long, pos, p, s.outlet.as_ptr(), person, short);
            }
        }
        _ => story_crt!(t.buf(), 0xa5922c, p), // "Mary the tea lady was pleased by signing of %s ..."
    }
    t.set();
    t.link_club_short(s.club);
    t.link(long, 0x7d5, s.club.addr());
    if s.who == 2 && s.spokesman.matches() {
        s.spokesman.link(t);
    }
    if s.player.matches() {
        s.player.link(t);
        s.player_short.link(t);
    }
}

/// Writes the headline of news item 0xbe5 (the player's short name, and the spokesman's short name when there is one):
/// the fans celebrate (1), the players celebrate or the spokesman celebrates (2), or the tea lady is pleased. Links:
/// the spokesman (2) and the player when the names match.
unsafe fn signing_headline(s: &mut SigningText, t: &mut MediaText) {
    let player = s.player.restyled(0x190, 5);
    if s.who == 2 && s.spokesman_id != -1 {
        s.spokesman = s.spokesman.restyled(0x190, 5);
    }
    let p = player.ptr();
    match s.who {
        1 => media_text!(t, 0xa587b0, 0x64, p), // "Fans celebrate signing of ..."
        2 if s.spokesman_id == -1 => media_text!(t, 0xa5885c, 0x64, p), // "Players celebrate signing of ..."
        2 => media_text!(t, 0xa587f4, 0x64, s.spokesman.ptr(), p), // "... celebrates signing ..."
        _ => story_crt!(t.buf(), 0xa588a0, p), // "Mary the tea lady pleased by signing of %s"
    }
    t.set();
    if s.who == 2 && s.spokesman.matches() {
        s.spokesman.link(t);
    }
    if player.matches() {
        player.link(t);
    }
}

/// Writes the headline or body of news item 0xbe5, where the fans, a squad spokesman or the tea lady welcome a new
/// signing (`signing_text_read`, `signing_body`, `signing_headline`). The outlet is written and used only by the
/// fans' text without a named person and a form.
/// Original: `Media::write_signing_celebration_news_text` (0x75b9c0, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_signing_celebration_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::new(out);
    let mut s = signing_text_read(news);
    if body != 0 {
        signing_body(&s, &mut t);
    } else {
        signing_headline(&mut s, &mut t);
    }
}

/// Comparer for `write_signing_celebration_news_text` (`compare_media_text_writer`, who welcomes him in slot 9).
pub(crate) unsafe extern "thiscall" fn compare_write_signing_celebration_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_media_text_writer("write_signing_celebration_news_text", news, body, 9, out, || o(this, news, body, out), |t| write_signing_celebration_news_text(this, news, body, t));
}

/// What news item (2, 0x2e) (a trophy won) holds: slots 0..3 the manager (full and short, 0x190), 4 the club, 5 the
/// competition index (records of 0x6b bytes in `G_CLUB_COMP_TABLE`; its word +0x69 [inference: its standing]), 7
/// and 8 the outlet type and pundit, 9 who praises (1 the fans, 3 the board, other the tea lady), 10 and 13 two ratings
/// [inference: his reputation before and now], 11 a mark, 12 his days at the club.
struct TrophyText {
    manager: NewsName,
    manager_short: NewsName,
    club: Club,
    comp: usize,
    who: i32,
    before: i32,
    mark: u32,
    days: i32,
    now: i32,
}

impl TrophyText {
    unsafe fn standing(&self) -> i32 {
        read_i16(self.comp + 0x69) as i32
    }
}

/// Reads news item (2, 0x2e) and writes the outlet (`media_story_outlet`, used by no template).
unsafe fn trophy_text_read(news: usize) -> TrophyText {
    let s = |k: u32| exe::newsitem_get_slot(news, k) as i32;
    let manager = NewsName::read(news, 0, 0x190, 1);
    let manager_short = manager.restyled(0x190, 5);
    let club = club_at(s(4) as isize);
    let mut outlet = [0u8; 0x78];
    media_story_outlet(club, s(4), s(7), s(8), &mut outlet);
    TrophyText {
        manager, manager_short, club,
        comp: read_usize(G_CLUB_COMP_TABLE).wrapping_add((s(5) as usize).wrapping_mul(0x6b)),
        who: s(9), before: s(10), mark: s(11) as u32, days: s(12), now: s(13),
    }
}

/// The board's second paragraph: the template and its arguments (0 none, 1 the manager's short name, 2 the club's
/// short name with its gender and the manager's short name); `None`: no second paragraph. With the mark: a first
/// season (under 365 days), a remarkable feat (now − 1500 below 500 × the standing), a rise (now below 7250, before
/// above 7250) or more of the same (now above 7250, before below 6750). Without it: the board's private praise (before
/// below 4750), a first season, a remarkable feat, a rise (now below 5250 or 7250, before above 7250), more of the
/// same (standing above 10, now above 7250, before below 6750) or "they expect" (now below 7250, before below 6750).
unsafe fn trophy_board_second(s: &TrophyText) -> Option<(usize, u8)> {
    let feat = s.now - 1500 < s.standing() * 500;
    let more_of_the_same = (0xa59a08, 0); // "They expect him to continue this success next season."
    if s.mark != 0 {
        if s.days < 365 {
            Some((0xa59b78, 1))
        } else if feat {
            Some((0xa59abc, 2))
        } else if s.now < 0x1c52 && s.before > 0x1c52 {
            Some((0xa59a40, 1))
        } else if s.now > 0x1c52 && s.before < 0x1a5e {
            Some(more_of_the_same)
        } else {
            None
        }
    } else if s.before < 0x128e {
        Some((0xa59874, 0))
    } else if s.days < 365 {
        Some((0xa59b78, 1))
    } else if feat {
        Some((0xa597bc, 2))
    } else if s.now < 0x1482 && s.before > 0x1c52 {
        Some((0xa59748, 1))
    } else if s.now < 0x1c52 && s.before > 0x1c52 {
        Some((0xa59a40, 1))
    } else if s.standing() > 10 && s.now > 0x1c52 && s.before < 0x1a5e {
        Some(more_of_the_same)
    } else if s.now < 0x1c52 && s.before < 0x1a5e {
        Some((0xa596c4, 1))
    } else {
        None
    }
}

/// Writes the body of news item (2, 0x2e): the board (3) or the supporters (1) praise the manager after the title or
/// cup, with a second paragraph by his ratings and time at the club, joined by "%s\n\n%s"; others the tea lady.
/// Links: the club's short and long names, the manager's two names when they match and the competition's name (0x7d0).
unsafe fn trophy_body(s: &TrophyText, t: &mut MediaText) {
    let (lg, long) = club_long_name(s.club);
    let (sg, short) = club_short_name(s.club);
    let (kg, comp_short) = (read_i8(s.comp + competition::SHORT_GENDER_NAME) as i32, s.comp + competition::SHORT_NAME);
    let ms = s.manager_short.ptr();
    let mut head = [0u8; 0x1400];
    let mut second = [0u8; 0x1400];
    let (h, x) = (head.as_mut_ptr(), second.as_mut_ptr());
    let joined = match s.who {
        3 => {
            covered!("sx01e_5 trophy text board");
            let tpl = if s.mark != 0 { 0xa59c18 } else if s.before < 0x128e { 0xa5997c } else { 0xa598e8 };
            story_lang!(h, tpl, lg, lg, kg, kg, long, comp_short);
            let paragraph = trophy_board_second(s);
            match paragraph {
                Some((tpl, 0)) => story_lang!(x, tpl),
                Some((tpl, 1)) => story_lang!(x, tpl, ms),
                Some((tpl, _)) => story_lang!(x, tpl, sg, sg, short, ms),
                None => {}
            }
            paragraph.is_some()
        }
        1 => {
            covered!("sx01e_5 trophy text supporters");
            let tpl = if s.mark != 0 { 0xa59604 } else { 0xa5955c };
            story_lang!(h, tpl, lg, lg, 0x64, kg, kg, long, s.manager.ptr(), comp_short);
            if s.standing() > 10 && s.now < 0x1a5e && s.before > 0x1f40 {
                story_lang!(x, 0xa594e0, ms);
                true
            } else if s.now < 0x1c52 && s.before < 0x1a5e {
                story_lang!(x, 0xa59444, 0x64, ms);
                true
            } else if s.standing() > 10 && s.now < 0x1c52 && s.now > 0x1482 {
                story_lang!(x, 0xa593cc, 0x64, ms);
                true
            } else {
                false
            }
        }
        _ => {
            story_crt!(t.buf(), 0xa59cd0); // "Mary the tea lady was pleased by the team winning ..."
            t.set();
            trophy_body_links(s, t, long);
            return;
        }
    };
    if joined {
        story_crt!(t.buf(), 0x991948, h, x); // "%s\n\n%s"
    } else {
        story_crt!(t.buf(), 0x991944, h); // "%s"
    }
    t.set();
    trophy_body_links(s, t, long);
}

/// The links of the trophy body: the club's short and long names, the manager's two names when they match and the
/// competition's long name (0x7d0).
unsafe fn trophy_body_links(s: &TrophyText, t: &MediaText, long: usize) {
    t.link_club_short(s.club);
    t.link(long, 0x7d5, s.club.addr());
    if s.manager.matches() {
        s.manager.link(t);
        s.manager_short.link(t);
    }
    t.link(s.comp + competition::NAME, 0x7d0, s.comp);
}

/// Writes the headline of news item (2, 0x2e) (the manager's short name formatted again): the fans celebrate winning
/// the competition (1), the board is pleased (before below 5250) or delighted (3), or the tea lady celebrates. Links:
/// the manager when the names match, the competition's short name (0x7d0).
unsafe fn trophy_headline(s: &TrophyText, t: &mut MediaText) {
    let manager = s.manager.restyled(0x190, 5);
    covered!("sx01e_5 trophy text headline");
    let (kg, comp_short) = (read_i8(s.comp + competition::SHORT_GENDER_NAME) as i32, s.comp + competition::SHORT_NAME);
    match s.who {
        1 => media_text!(t, 0xa592d8, kg, kg, comp_short), // "Fans celebrate winning the ..."
        3 => media_text!(t, if s.before < 0x1482 { 0xa59360 } else { 0xa5931c }, kg, kg, comp_short), // "Board pleased / delighted by ..."
        _ => story_crt!(t.buf(), 0xa593a4), // "Mary the tea lady celebrates victory"
    }
    t.set();
    if manager.matches() {
        manager.link(t);
    }
    t.link(comp_short, 0x7d0, s.comp);
}

/// Writes the headline or body of news item (2, 0x2e), where the board, the fans or the tea lady praise the manager
/// after a league title or cup win (`trophy_text_read`, `trophy_body`, `trophy_headline`).
/// Original: `Media::write_trophy_praise_news_text` (0x75c5a0, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_trophy_praise_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::new(out);
    let s = trophy_text_read(news);
    if body != 0 {
        trophy_body(&s, &mut t);
    } else {
        trophy_headline(&s, &mut t);
    }
}

/// Comparer for `write_trophy_praise_news_text` (`compare_media_text_writer`, who praises in slot 9).
pub(crate) unsafe extern "thiscall" fn compare_write_trophy_praise_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_media_text_writer("write_trophy_praise_news_text", news, body, 9, out, || o(this, news, body, out), |t| write_trophy_praise_news_text(this, news, body, t));
}

/// Writes the body of news item 0xbda (into the shared buffer 0xde1f64), a player thanking the manager: slots 0..3 the
/// player (full and short), 4 the club, 5 the topic, 6 a mark, 7 the variant, 8..11 the manager (full and short), 12
/// 0x420e9 when slots 13..16 hold the assistant (full name; empty without one). By topic and variant (the
/// original's two jump tables): private meetings, the assistant's report, your dismissal of press criticism, the
/// club's youngster or starlet, the manager's response... Links: the player's and the manager's two names when they
/// match, the club's short and long names.
unsafe fn gratitude_body(news: usize, t: &mut MediaText) {
    let s = |k: u32| exe::newsitem_get_slot(news, k);
    let player = NewsName::read(news, 0, 0x190, 1);
    let player_short = player.restyled(0x190, 5);
    let manager = NewsName::read(news, 8, 0x190, 1);
    let manager_short = manager.restyled(0x190, 5);
    let club = club_at(s(4) as i32 as isize);
    let (topic, mark, variant) = (s(5), s(6), s(7));
    let assistant_id = if s(12) == 0x420e9 { s(13) as i32 } else { -1 };
    let mut assistant = NewsName { id: 0, names: [0; 3], text: [0; 0x190] };
    assistant.text[0] = read_u8(0xad9c64); // the empty string
    if assistant_id > -1 {
        assistant = NewsName::new(assistant_id as u32, [s(14), s(15), s(16)], 0x190, 1);
    }
    let (g, short) = club_short_name(club);
    let (p, m, ms, a, ps) = (player.ptr(), manager.ptr(), manager_short.ptr(), assistant.ptr(), player_short.ptr());
    covered!("sx01e_5 gratitude text body");
    match topic {
        0x82 => media_text!(t, 0xa25de8, p), // "After training today ..."
        0x13 => media_text!(t, 0xa25d30, p),
        5 | 7 => match variant {
            0 => media_text!(t, 0xa2596c, p, m),
            1 => media_text!(t, 0xa25a78, g, g, short, p, m), // "... youngster ..."
            2 => media_text!(t, 0xa25b40, g, g, p, short, m), // "Press speculation that ..."
            _ => media_text!(t, 0xa25c58, g, g, 0x64, short, p, m), // "... starlet ..."
        },
        9 => match variant {
            0 => media_text!(t, 0xa25730, p),
            1 => media_text!(t, 0xa257cc, p), // "Your rebuttal of press criticism of your new ..."
            _ => media_text!(t, 0xa258a0, p),
        },
        2 if variant != 0 => media_text!(t, 0xa2568c, g, g, p, short),
        2 => media_text!(t, 0xa2558c, 0x64, g, g, p, m, short),
        4 if mark == 1 => media_text!(t, 0xa254cc, p), // "In a private meeting ..."
        4 => match variant {
            0 => media_text!(t, 0xa251e0, 0x64, 0x64, m, p, ms), // "...{s} response ..."
            1 => media_text!(t, 0xa252f4, p, m),
            _ => media_text!(t, 0xa253b0, 0x64, p, m, ms), // "Press speculation that ..."
        },
        0x23 => media_text!(t, 0xa25120, p),
        0 | 6 | 0xd | 0xe | 0x11 if mark == 1 => media_text!(t, 0xa25060, p),
        0 | 6 | 0xd | 0xe | 0x11 => match variant {
            0 if assistant_id > -1 => media_text!(t, 0xa24f80, a, p), // "Your assistant ..."
            0 => media_text!(t, 0xa24eb4, p), // "A member of your coaching staff reported that ..."
            1 if assistant_id > -1 => media_text!(t, 0xa24ddc, a, p),
            1 => media_text!(t, 0xa24d1c, p),
            2 => media_text!(t, 0xa24c5c, 0x64, p), // "Your dismissal of press criticism of ..."
            3 => media_text!(t, 0xa24b90, 0x64, p, m),
            _ => media_text!(t, 0xa24ad0, p),
        },
        0x21 => match variant {
            0 => media_text!(t, 0xa248c0, 0x64, p, m),
            1 => media_text!(t, 0xa24950, 0x64, m, p), // "In the wake of ..."
            _ => media_text!(t, 0xa24a60, p),
        },
        0xc | 0x12 | 0x22 => media_text!(t, 0xa24a60, p),
        3 => media_text!(t, 0xa247ec, p),
        0x25 => media_text!(t, 0xa24750, p),
        0x1e if variant == 0 => media_text!(t, 0xa244d8, 0x64, g, g, m, p, short, ps),
        0x1e if variant == 1 => media_text!(t, 0xa24638, p, m, ps), // "... is pleased ..."
        0x28..=0x34 => media_text!(t, 0xa24474, p),
        _ => media_text!(t, 0xa243f8, p), // "In a private meeting ..."
    }
    t.set();
    if player.matches() {
        player.link(t);
        player_short.link(t);
    }
    if manager.matches() {
        manager.link(t);
        manager_short.link(t);
    }
    if club.addr() != 0 {
        t.link_club_short(club);
        t.link(club.addr() + club::NAME, 0x7d5, club.addr());
    }
}

/// Writes the headline of news item 0xbda (into 0xde1f64; the player's short name): topic 0x13 he accepts your
/// decision; 0, 6, 0xd, 0xe, 0x11 and 0x23 with the mark he reacts to the manager, with variant 0 or 1 the "Report from
/// assistant manager"; 0x12, 0x21 and 0x22 he acknowledges support; otherwise he thanks the manager. The name links to
/// him when the names match.
unsafe fn gratitude_headline(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 5);
    let (topic, mark, variant) = (exe::newsitem_get_slot(news, 5), exe::newsitem_get_slot(news, 6), exe::newsitem_get_slot(news, 7));
    let p = player.ptr();
    covered!("sx01e_5 gratitude text headline");
    match topic {
        0x13 => media_text!(t, 0xa243bc, p), // "... accepts your decision"
        0 | 6 | 0xd | 0xe | 0x11 | 0x23 if mark == 1 => media_text!(t, 0xa24384, p), // "... reacts to manager"
        0 | 6 | 0xd | 0xe | 0x11 | 0x23 if variant <= 1 => media_text!(t, 0xa24364), // "Report from assistant manager"
        0x12 | 0x21 | 0x22 => media_text!(t, 0xa24328, p), // "... acknowledges support"
        _ => media_text!(t, 0xa242f0, p), // "... thanks manager"
    }
    t.set();
    if player.matches() {
        player.link(t);
    }
}

/// Writes the headline or body of news item 0xbda, in which a player thanks the manager for his support against press
/// criticism (`gratitude_body`, `gratitude_headline`), through the shared buffer 0xde1f64 as the original.
/// Original: `Media::write_player_gratitude_news_text` (0x733000, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_gratitude_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::shared(out);
    if body != 0 {
        gratitude_body(news, &mut t);
    } else {
        gratitude_headline(news, &mut t);
    }
}

/// Comparer for `write_player_gratitude_news_text` (`compare_media_text_writer`, the topic in slot 5).
pub(crate) unsafe extern "thiscall" fn compare_write_player_gratitude_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_media_text_writer("write_player_gratitude_news_text", news, body, 5, out, || o(this, news, body, out), |t| write_player_gratitude_news_text(this, news, body, t));
}
