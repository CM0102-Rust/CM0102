// --- sx01e_4: the media text writers (media.cpp): obscene gesture, sale unrest, sending off ---

/// The match of a media story, as `exe::read_match_record_from_news` reads it from the news slots (0x1f bytes, after
/// `exe::date_date_54a110`): +0 and +2 the date, +8 the competition index, +0x11 the first club's id, +0x15 the other
/// club's id, +0x19 and +0x1a the goals, +0x1b the match statistics index.
pub(crate) struct NewsMatch {
    record: [u8; 0x20],
}

impl NewsMatch {
    /// Reads the record from slot `k` on; `k` is left on the slot after it.
    unsafe fn read(news: usize, k: &mut u8) -> Self {
        let mut m = NewsMatch { record: [0; 0x20] };
        exe::date_date_54a110(m.record.as_mut_ptr() as usize);
        exe::read_match_record_from_news(news, k, m.record.as_mut_ptr());
        m
    }
    fn dword(&self, at: usize) -> i32 {
        i32::from_le_bytes([self.record[at], self.record[at + 1], self.record[at + 2], self.record[at + 3]])
    }
    fn goals(&self) -> (i8, i8) {
        (self.record[0x19] as i8, self.record[0x1a] as i8)
    }
    unsafe fn competition(&self) -> usize {
        read_usize(G_CLUB_COMP_TABLE).wrapping_add((self.dword(8) as usize).wrapping_mul(0x6b)) // the 0x6b-byte competition records
    }
    unsafe fn other_club(&self) -> Club {
        club_at(self.dword(0x15) as isize)
    }
    /// Links the score text to the match (0x7d3, the statistics index) when `Match_Stats::fixture_entry_matches` finds
    /// it.
    unsafe fn link_score(&self, t: &MediaText, score: usize) {
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), self.dword(0x1b), self.dword(0x11), self.dword(0x15), self.dword(0), self.dword(2)) != 0 {
            t.link(score, 0x7d3, self.dword(0x1b) as u32 as usize);
        }
    }
}

/// The penalty shoot-out sentence appended to a drawn match (`story_strcat` into the text): slot value 1 the club won
/// after a penalty shoot-out, 2 lost on penalties, 3 won after a shoot-out, 4 lost on a shoot-out (with the club's
/// short name); other values nothing.
unsafe fn append_shootout(t: &mut MediaText, result: u32, club: Club) {
    let tpl = match result {
        1 => 0xa21e0c,
        2 => 0xa21684,
        3 => 0xa21dc0,
        4 => 0xa21d74,
        _ => return,
    };
    covered!("sx01e_4 shoot-out sentence");
    let (g, short) = club_short_name(club);
    let mut line = [0u8; 0x200];
    story_lang!(line.as_mut_ptr(), tpl, g, g, short);
    story_strcat(t.buf(), line.as_ptr() as usize);
}

/// Writes the body of news item 0xbcb, a player filmed making obscene gestures to the fans (into the shared buffer
/// 0xde1f64): slots 0..3 the player (full and short names, 0x190), 4 his club, 5 the form of the story, the match
/// record from slot 6 and the shoot-out after it. The score is "%d-%d" with the higher goals first. With slot 5 set:
/// "During {club}'s {score} {competition} draw / win / defeat against {opponent}, {player} incited fury ..."; without
/// it "{player} has been captured on camera making obscene gestures to {opponent} fans during ..."; a draw gets the
/// shoot-out sentence (`append_shootout`). Links: the club's long and short names, the opponent's, the competition
/// (0x7d0), the score (`NewsMatch::link_score`) and the player's two names when they match.
unsafe fn obscene_gesture_body(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 1);
    let club = club_at(exe::newsitem_get_slot(news, 4) as i32 as isize);
    let filmed_at_match = exe::newsitem_get_slot(news, 5) != 0;
    let mut k: u8 = 6;
    let m = NewsMatch::read(news, &mut k);
    let shootout = exe::newsitem_get_slot(news, k as u32);
    let short = player.restyled(0x190, 5);
    let (g1, g2) = m.goals();
    let mut score = [0u8; 0x20];
    story_crt!(score.as_mut_ptr(), 0x98c114, g1.max(g2) as i32, g1.min(g2) as i32); // "%d-%d"
    let comp = m.competition();
    let other = m.other_club();
    let (pg, own) = club_short_name(club);
    let (og, other_long) = club_long_name(other);
    let cg = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
    let comp_short = comp + competition::SHORT_NAME;
    let (full, sc) = (player.ptr(), score.as_ptr() as usize);
    if filmed_at_match {
        covered!("sx01e_4 obscene gesture text during the match");
        let tpl = if g1 == g2 { 0xa55d48 } else if g1 > g2 { 0xa55be4 } else { 0xa55a7c }; // draw / win / defeat
        media_text!(t, tpl, pg, pg, cg, cg, og, og, own, sc, comp_short, other_long, full, short.ptr());
    } else if g1 == g2 {
        media_text!(t, 0xa55924, 0x64, pg, pg, og, og, cg, cg, own, full, other_long, sc, comp_short);
    } else {
        let tpl = if g1 > g2 { 0xa557cc } else { 0xa55674 }; // win / defeat
        media_text!(t, tpl, 0x64, 0x64, og, og, pg, pg, 0x64, cg, cg, full, other_long, own, sc, comp_short);
    }
    if g1 == g2 {
        append_shootout(t, shootout, club);
    }
    t.set();
    t.link(club.addr() + club::NAME, 0x7d5, club.addr());
    t.link_club_short(club);
    t.link(other_long, 0x7d5, other.addr());
    t.link_club_short(other);
    t.link(comp_short, 0x7d0, comp);
    m.link_score(t, sc);
    if player.matches() {
        player.link(t);
        short.link(t);
    }
}

/// Writes the headline of news item 0xbcb (into 0xde1f64): the player's short name (0x190, style 5); "caught on camera"
/// when (slot 5 + 1) × his id mod 7 is above 3, otherwise "banned for gesture"; the name links to him with type
/// 0x7da when the names match.
unsafe fn obscene_gesture_headline(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 5);
    let form = exe::newsitem_get_slot(news, 5) as i32;
    let tpl = if form.wrapping_add(1).wrapping_mul(player.id as i32) % 7 > 3 { 0xa55640 } else { 0xa55608 };
    media_text!(t, tpl, 0x64, player.ptr());
    t.set();
    if player.matches() {
        t.link(player.ptr(), 0x7da, staff_at(player.id as i32 as isize).addr());
    }
}

/// Writes the headline or body of news item 0xbcb, about a player filmed making obscene gestures to the fans, with the
/// score and any shoot-out (`obscene_gesture_body`, `obscene_gesture_headline`). Both go through the shared buffer
/// 0xde1f64, as the original.
/// Original: `Media::write_obscene_gesture_news_text` (0x7572a0, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_obscene_gesture_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::shared(out);
    if body != 0 {
        obscene_gesture_body(news, &mut t);
    } else {
        obscene_gesture_headline(news, &mut t);
    }
}

/// Comparer for `write_obscene_gesture_news_text` (`compare_media_text_writer`, the form in slot 5).
pub(crate) unsafe extern "thiscall" fn compare_write_obscene_gesture_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_media_text_writer("write_obscene_gesture_news_text", news, body, 5, out, || o(this, news, body, out), |t| write_obscene_gesture_news_text(this, news, body, t));
}

/// The media outlet of a story about `club` (its index, -1 none), written into `out` as the originals do before
/// choosing the text: with the club, the outlet type, the pundit, the club's nation and the human manager on screen at
/// home (his club job's nation, or his own nation without one, is the club's), the pundit's name
/// (`exe::punditlist_get_pundit_name`); otherwise by type the national or local newspaper, radio station, the
/// magazine, the sports paper or the website, plain at home or "{nationality} ..." abroad
/// (`exe::nation_get_nationality_adjective`). The gender byte: the pundit's, or 1 for the magazine in language 8.
unsafe fn media_story_outlet(club: Club, club_index: i32, outlet: i32, pundit: i32, out: &mut [u8; 0x78]) -> u8 {
    let human = Staff(read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300))));
    let nation = read_usize(club.addr() + club::NATION);
    let home = !human.is_null() && if !human.club_job().is_null() { human.club_job().nation().addr() == nation } else { nation == human.nation().addr() };
    if club_index != -1 && outlet != -1 && pundit != -1 && nation != 0 && home {
        let mut gender = 0u8;
        exe::punditlist_get_pundit_name(read_usize(G_PUNDIT_LIST), outlet as u32, pundit as u32, out.as_mut_ptr() as usize, &mut gender);
        return gender;
    }
    let (plain, form, abroad) = match outlet.wrapping_sub(1) {
        0 => (0xa242cc, 0x16, 0xa24294), // national newspaper
        1 => (0xa24284, 0x17, 0xa24250), // local newspaper
        2 => (0xa24238, 0x18, 0xa24208), // national radio station
        3 => (0xa241f4, 0x18, 0xa241bc), // local radio station
        4 => (0xa241b0, 0x19, 0xa24184), // magazine
        5 => (0xa24174, 0x1a, 0xa24144), // sports paper
        _ => (0xa2413c, 0x1a, 0xa24110), // website
    };
    if home {
        story_lang!(out.as_mut_ptr(), plain);
    } else {
        let mut adjective = [0u8; 0x78];
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x78, form);
        story_lang!(out.as_mut_ptr(), abroad, 0x64, adjective.as_ptr());
    }
    if outlet.wrapping_sub(1) == 4 { (exe::langlib_get_language() == 8) as u8 } else { 0 }
}

/// The people and clubs of news item 0xbe4 (a player unhappy at a team mate's sale): slots 0..3 the manager, 4..7 the
/// player, 8..11 the sold player (all full and short names, 0x190), 12 the selling club, 13 the buying club, 14 the
/// case, 15 the outlet type, 16 the pundit, 17 the form (2 the story, otherwise the tea lady).
struct SaleUnrest {
    manager: NewsName,
    manager_short: NewsName,
    player: NewsName,
    player_short: NewsName,
    sold: NewsName,
    sold_short: NewsName,
    seller: Club,
    buyer: Club,
    case: i32,
    story: bool,
}

/// Reads news item 0xbe4 and writes its outlet (`media_story_outlet`, which no template uses).
unsafe fn sale_unrest_read(news: usize) -> SaleUnrest {
    let s = |k: u32| exe::newsitem_get_slot(news, k) as i32;
    let player = NewsName::read(news, 4, 0x190, 1);
    let player_short = player.restyled(0x190, 5);
    let manager = NewsName::read(news, 0, 0x190, 1);
    let manager_short = manager.restyled(0x190, 5);
    let sold = NewsName::read(news, 8, 0x190, 1);
    let sold_short = sold.restyled(0x190, 5);
    let seller = club_at(s(12) as isize);
    let mut outlet = [0u8; 0x78];
    media_story_outlet(seller, s(12), s(15), s(16), &mut outlet);
    SaleUnrest { manager, manager_short, player, player_short, sold, sold_short, seller, buyer: club_at(s(13) as isize), case: s(14), story: s(17) == 2 }
}

/// Writes the body of news item 0xbe4: without the story form the tea lady's line ("Mary the tea-lady was upset by
/// the sale of Roy Keane ..."); otherwise by case: 0 several players declined to comment, 1 the manager's decision
/// sparked rumours of discontent, 2 the buyer's acquisition prompted the player to speak out against the manager,
/// other the player is unhappy that his former colleague was transferred. Links: the seller's and buyer's short and
/// long names and the player's two names when they match.
unsafe fn sale_unrest_body(u: &SaleUnrest, t: &mut MediaText) {
    let (sl, seller_long) = club_long_name(u.seller);
    let (ss, seller_short) = club_short_name(u.seller);
    let (bl, buyer_long) = club_long_name(u.buyer);
    let (bs, buyer_short) = club_short_name(u.buyer);
    if !u.story {
        covered!("sx01e_4 sale unrest text tea lady");
        story_crt!(t.buf(), 0xa5a304); // "Mary the tea-lady was upset by the sale of Roy Keane ..."
    } else {
        covered!("sx01e_4 sale unrest text body");
        match u.case {
            0 => media_text!(t, 0xa59e14, sl, sl, 0x64, bs, bs, seller_long, u.sold.ptr(), buyer_short), // "Several ... players have declined to comment ..."
            1 => media_text!(t, 0xa59f70, bl, bl, ss, ss, 0x64, u.manager.ptr(), u.sold.ptr(), buyer_long, seller_short), // "Manager ...{s} decision ..."
            2 => media_text!(t, 0xa5a080, bl, bl, ss, ss, 0x64, buyer_long, seller_short, u.sold.ptr(), u.player.ptr(), u.manager_short.ptr()),
            _ => media_text!(t, 0xa5a1ec, sl, sl, 0x64, 0x64, bs, bs, seller_long, u.player.ptr(), u.sold.ptr(), buyer_short), // "Worried ... star ..."
        }
    }
    t.set();
    t.link_club_short(u.seller);
    t.link(seller_long, 0x7d5, u.seller.addr());
    t.link_club_short(u.buyer);
    t.link(buyer_long, 0x7d5, u.buyer.addr());
    if u.player.matches() {
        u.player.link(t);
        u.player_short.link(t);
    }
}

/// Writes the headline of news item 0xbe4 (the player's name formatted again in style 5): without the story form "Mary
/// the tea lady celebrates victory"; cases 0 and 1 "{sold player} sale causes unrest", others "{player} worried by sale
/// of {sold player}". Links: the player's and the sold player's short names when they match.
unsafe fn sale_unrest_headline(news: usize, u: &SaleUnrest, t: &mut MediaText) {
    NewsName::read(news, 4, 0x190, 5);
    if !u.story {
        story_crt!(t.buf(), 0xa593a4); // "Mary the tea lady celebrates victory"
    } else if (0..=1).contains(&u.case) {
        media_text!(t, 0xa59dd4, u.sold_short.ptr());
    } else {
        media_text!(t, 0xa59d70, u.player_short.ptr(), u.sold_short.ptr());
    }
    t.set();
    if u.player.matches() {
        u.player_short.link(t);
    }
    if u.sold.matches() {
        u.sold_short.link(t);
    }
}

/// Writes the headline or body of news item 0xbe4, where a player is unhappy at a team mate's sale
/// (`sale_unrest_read`, `sale_unrest_body`, `sale_unrest_headline`).
/// Original: `media_write_sale_unrest_news_text` (0x75d210, media.cpp, thiscall, ret 0xc). The outlet it writes
/// first (the "named media outlet" of the map) is used by no template.
pub(crate) unsafe extern "thiscall" fn media_write_sale_unrest_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::new(out);
    let u = sale_unrest_read(news);
    if body != 0 {
        sale_unrest_body(&u, &mut t);
    } else {
        sale_unrest_headline(news, &u, &mut t);
    }
}

/// Comparer for `media_write_sale_unrest_news_text` (`compare_media_text_writer`, the case in slot 14).
pub(crate) unsafe extern "thiscall" fn compare_media_write_sale_unrest_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_media_text_writer("media_write_sale_unrest_news_text", news, body, 14, out, || o(this, news, body, out), |t| media_write_sale_unrest_news_text(this, news, body, t));
}

/// The story paragraph of news item 0xbe6 (a sending off), into `story`: slot 8 picks the form (set: "{player}" with
/// the clubs, the score, the opponent and his red cards in slot 11; clear: with the ordinal of slot 11
/// (`exe::utils_format_ordinal`), the other club's manager and his own), the result picks the text (0 draw, 1 win,
/// other defeat). The result and the score come from the match record: when the player's club is the record's first
/// club, a win is 1 and "g1-g2", a draw 0, a defeat 0xff and "g2-g1"; when it is the second club, its win is 1 with
/// "g1-g2", a draw 0 and its defeat also 0 with "g2-g1" (a defect of the original: a defeat away reads as a draw).
unsafe fn sent_off_story(news: usize, story: &mut [u8; 0x1400], names: &SentOffNames, own: Club, other: Club, m: &NewsMatch) {
    let red_cards = exe::newsitem_get_slot(news, 11) as i32;
    let mut ordinal = [0u8; 0x80];
    praise_strcpy(ordinal.as_mut_ptr() as usize, exe::utils_format_ordinal(red_cards, -1));
    let (g1, g2) = m.goals();
    let first = m.dword(0x11) == read_i32(own.addr());
    let (result, high, low): (u8, i8, i8) = if first {
        if g1 > g2 { (1, g1, g2) } else if g1 == g2 { (0, g1, g2) } else { (0xff, g2, g1) }
    } else if g1 > g2 {
        (0, g2, g1)
    } else if g1 == g2 {
        (0, g1, g2)
    } else {
        (1, g1, g2)
    };
    let mut score = [0u8; 0x20];
    story_lang!(score.as_mut_ptr(), 0x9c116c, high as i32, low as i32); // "<%d - goals 1>-<%d - goals 2>"
    let (og, own_long) = club_long_name(own);
    let (tg, other_long) = club_long_name(other);
    let (sc, full) = (score.as_ptr() as usize, names.player.ptr());
    let s = story.as_mut_ptr();
    if exe::newsitem_get_slot(news, 8) != 0 {
        let tpl = match result { 0 => 0xa576a0, 1 => 0xa57504, _ => 0xa5736c };
        story_lang!(s, tpl, 0x64, og, og, tg, tg, full, own_long, sc, other_long, names.player_short.ptr(), red_cards);
    } else {
        let tpl = match result { 0 => 0xa5719c, 1 => 0xa56fcc, _ => 0xa56dfc };
        story_lang!(s, tpl, og, og, tg, tg, full, ordinal.as_ptr(), own_long, sc, other_long, names.other_manager.ptr(), names.manager.ptr());
    }
}

/// The names of news item 0xbe6 (0x190 each): slots 0..3 the player (full and short), 4..7 the other player (full),
/// 12..15 the player's club manager (short), 16..19 the other club's manager (short).
struct SentOffNames {
    player: NewsName,
    player_short: NewsName,
    other_player: NewsName,
    manager: NewsName,
    other_manager: NewsName,
}

/// The reply paragraph of news item 0xbe6 (slot 20, the response), into `reply`. When the player himself is the human
/// manager on screen (the original compares the player, not his manager: a defect, the second person is never used
/// otherwise): 1 and 2 two answers in the second person, 3 "You have chosen to ignore this story"; otherwise the
/// manager's comment for 1 and 2, or 3 his club's short name and the manager. Another response leaves the reply
/// unwritten (`MEDIA_TEXT_UNSET`). False without a response (slot 20 = 0).
unsafe fn sent_off_reply(news: usize, reply: &mut [u8; 0x1400], names: &SentOffNames, own: Club) -> bool {
    let response = exe::newsitem_get_slot(news, 20) as i32;
    if response == 0 {
        return false;
    }
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    let r = reply.as_mut_ptr();
    let (full, manager) = (names.player.ptr(), names.manager.ptr());
    if staff_at(names.player.id as i32 as isize).addr() == human {
        covered!("sx01e_4 sending off text your reply");
        match response {
            1 => story_lang!(r, 0xa56cf8, 0x64, full),
            2 => story_lang!(r, 0xa56d68, 0x64, full),
            3 => story_lang!(r, 0xa56dd4), // "You have chosen to ignore this story"
            _ => MEDIA_TEXT_UNSET.store(true, Relaxed),
        }
    } else {
        match response {
            1 => story_lang!(r, 0xa56b14, 0x64, 0x64, manager, full),
            2 => story_lang!(r, 0xa56bb8, 0x64, 0x64, manager, full),
            3 => {
                let (g, short) = (own.gender_name() as i32, own.addr() + club::SHORT_NAME);
                story_lang!(r, 0xa56c58, g, g, short, manager);
            }
            _ => MEDIA_TEXT_UNSET.store(true, Relaxed),
        }
    }
    true
}

/// Writes the body of news item 0xbe6: slots 0..19 the people, 8 the form, 9 the player's club, 10 the other club, 11
/// his red cards, 20 the response, the match record from 21. The story (`sent_off_story`) and, with a response, the
/// reply (`sent_off_reply`) joined by "%s\n\n%s". Links: the other club's short and long names, the player's club's,
/// the player's two names, his club manager, the other player and the other manager, each when the names match.
unsafe fn sent_off_body(news: usize, t: &mut MediaText) {
    let names = SentOffNames {
        player: NewsName::read(news, 0, 0x190, 1),
        player_short: NewsName::read(news, 0, 0x190, 5),
        other_player: NewsName::read(news, 4, 0x190, 1),
        manager: NewsName::read(news, 12, 0x190, 5),
        other_manager: NewsName::read(news, 16, 0x190, 5),
    };
    let own = club_at(exe::newsitem_get_slot(news, 9) as i32 as isize);
    let other_id = exe::newsitem_get_slot(news, 10) as i32;
    let other = if other_id != -1 { club_at(other_id as isize) } else { Club(0) }; // the original keeps a stale register
    let mut k: u8 = 21;
    let m = NewsMatch::read(news, &mut k);
    covered!("sx01e_4 sending off text body");
    let mut story = [0u8; 0x1400];
    let mut reply = [0u8; 0x1400];
    sent_off_story(news, &mut story, &names, own, other, &m);
    if sent_off_reply(news, &mut reply, &names, own) {
        story_crt!(t.buf(), 0x991948, story.as_ptr(), reply.as_ptr()); // "%s\n\n%s"
    } else {
        story_crt!(t.buf(), 0x991944, story.as_ptr()); // "%s"
    }
    t.set();
    t.link_club_short(other);
    t.link(other.addr() + club::NAME, 0x7d5, other.addr());
    t.link_club_short(own);
    t.link(own.addr() + club::NAME, 0x7d5, own.addr());
    if names.player.matches() {
        names.player.link(t);
        names.player_short.link(t);
    }
    for n in [&names.manager, &names.other_player, &names.other_manager] {
        if n.matches() {
            n.link(t);
        }
    }
}

/// Writes the headline of news item 0xbe6 (the player's short name, 0x190, style 5): with slot 8 "{player} lashes out",
/// otherwise "{player} should be put on a leash"; the name links to the player when the names match.
unsafe fn sent_off_headline(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 5);
    covered!("sx01e_4 sending off text headline");
    let tpl = if exe::newsitem_get_slot(news, 8) != 0 { 0xa56ae4 } else { 0xa56aa4 };
    media_text!(t, tpl, player.ptr());
    t.set();
    if player.matches() {
        player.link(t);
    }
}

/// Writes the headline or body of news item 0xbe6, about a player's sending off, adding your reply or the manager's
/// comment (`sent_off_body`, `sent_off_headline`).
/// Original: `Media::write_player_sent_off_news_text` (0x759a10, media.cpp, thiscall, ret 0xc). Defects of the
/// original: a defeat of the player's club as the record's second club reads as a draw; the second-person reply is
/// chosen when the player (not his manager) is the human manager on screen; a response above 3 joins an unwritten
/// reply buffer.
pub(crate) unsafe extern "thiscall" fn write_player_sent_off_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::new(out);
    if body != 0 {
        sent_off_body(news, &mut t);
    } else {
        sent_off_headline(news, &mut t);
    }
}

/// Comparer for `write_player_sent_off_news_text` (`compare_media_text_writer`, the response in slot 20).
pub(crate) unsafe extern "thiscall" fn compare_write_player_sent_off_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_media_text_writer("write_player_sent_off_news_text", news, body, 20, out, || o(this, news, body, out), |t| write_player_sent_off_news_text(this, news, body, t));
}
