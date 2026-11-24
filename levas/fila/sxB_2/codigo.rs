// --- sxB_2: the media texts of a player who wants first team football and of a player's outburst (media.cpp) ---

/// The media is local when `nation` is the nation of the human manager on screen: his club's nation, or his own
/// nation without a club; never without a human.
unsafe fn media_is_local(nation: usize) -> bool {
    let human = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    if human.is_null() {
        return false;
    }
    let job = human.club_job();
    if !job.is_null() { job.nation().addr() == nation } else { nation == human.nation().addr() }
}

/// The name of the media outlet a story quotes into `out`, returning its grammar mark: the pundit's name and mark
/// (`exe::punditlist_get_pundit_name`) when `pundit` is given; otherwise by the outlet type (1 national newspaper, 2
/// local newspaper, 3 national radio station, 4 local radio station, 5 magazine, 6 sports paper, other website), the
/// plain name when the media is local, else "<nationality> ..." with the adjective of `nation` (forms 0x16..0x1a,
/// `exe::nation_get_nationality_adjective`, 0x78 bytes); the mark is 1 for a magazine in language 8, else 0.
unsafe fn media_outlet_name(outlet: i32, pundit: Option<i32>, nation: usize, local: bool, out: usize) -> u8 {
    let mut mark = 0u8;
    if let Some(p) = pundit {
        exe::punditlist_get_pundit_name(read_usize(G_PUNDIT_LIST), outlet as u32, p as u32, out, &mut mark);
        return mark;
    }
    let (plain, form, foreign) = match outlet {
        1 => (0xa242cc, 0x16, 0xa24294), // national newspaper
        2 => (0xa24284, 0x17, 0xa24250), // local newspaper
        3 => (0xa24238, 0x18, 0xa24208), // national radio station
        4 => (0xa241f4, 0x18, 0xa241bc), // local radio station
        5 => (0xa241b0, 0x19, 0xa24184), // magazine
        6 => (0xa24174, 0x1a, 0xa24144), // sports paper
        _ => (0xa2413c, 0x1a, 0xa24110), // website
    };
    if local {
        story_lang!(out, plain);
    } else {
        let mut adjective = [0u8; 0x78];
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x78, form);
        story_lang!(out, foreign, 0x64, adjective.as_ptr());
    }
    if outlet == 5 { (exe::langlib_get_language() == 8) as u8 } else { 0 }
}

/// What the text writer of news item 0xbe8 (a player wants first team football) reads and prepares for both parts:
/// slots 0..3 the player (full and short names, 0x190), 4..7 the manager (short name), 8 the club, 9 the story flags,
/// 10 the variant, 11 the outlet type, 12 the pundit, 14 the player's position (`exe::write_position_name`, 200
/// bytes, its gender), 15 the case, 16 and 17 two ratings, 18 your reply, 19 the reassurance, 20 a promise made, 21
/// "surplus to requirements"; slot 13 is read and dropped. The outlet (`media_outlet_name`) uses the pundit when the
/// club, the outlet and the pundit are set, the club has a nation and the media is local.
struct WantsFirstTeam {
    player: NewsName,
    player_short: NewsName,
    manager: NewsName,
    club: Club,
    s: [i32; 22],
    position: [u8; 200],
    position_gender: u8,
    outlet: [u8; 0x1f4],
    outlet_gender: u8,
}

impl WantsFirstTeam {
    unsafe fn read(news: usize) -> Self {
        let s: [i32; 22] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32) as i32);
        let club = club_at(s[8] as isize);
        let player = NewsName::new(s[0] as u32, [s[1] as u32, s[2] as u32, s[3] as u32], 0x190, 1);
        let player_short = player.restyled(0x190, 5);
        let manager = NewsName::new(s[4] as u32, [s[5] as u32, s[6] as u32, s[7] as u32], 0x190, 5);
        let mut w = WantsFirstTeam { player, player_short, manager, club, s, position: [0; 200], position_gender: 0, outlet: [0; 0x1f4], outlet_gender: 0 };
        w.position_gender = exe::write_position_name(s[14] as u32, w.position.as_mut_ptr() as usize, 200, 1);
        let nation = club.nation().addr();
        let local = media_is_local(nation);
        let pundit = (s[8] != -1 && s[11] != -1 && s[12] != -1 && nation != 0 && local).then_some(s[12]);
        w.outlet_gender = media_outlet_name(s[11], pundit, nation, local, w.outlet.as_mut_ptr() as usize);
        w
    }
}

/// The story paragraph of case 0 into `para` (slot 9 bit 1, bit 4, else the variant of slot 10): the goalkeeper's
/// jersey competition (0xa58308), unease at the competitive atmosphere at the stadium (0xa581b0, the club's stadium
/// `club::STADIUM`), or restless as a squad player (variant 0 0xa57c24, 1 0xa57dd4, other 0xa57f98, with the position).
unsafe fn wants_first_team_story(w: &WantsFirstTeam, para: usize) {
    let c = w.club;
    let (cs, short) = club_short_name(c);
    let stadium = c.stadium();
    let (sg, ground) = (stadium.gender_name() as i8 as i32, stadium.addr() + stadium::NAME);
    let og = w.outlet_gender as i8 as i32;
    let pg = w.position_gender as i8 as i32;
    let (outlet, p, pos, ms) = (w.outlet.as_ptr(), w.player.ptr(), w.position.as_ptr(), w.manager.ptr());
    let flags = w.s[9] as u8;
    if flags & 1 != 0 {
        story_lang!(para, 0xa58308, og, og, cs, cs, outlet, p, short); // "... the first team goalkeeper's jersey ..."
    } else if flags & 4 != 0 {
        story_lang!(para, 0xa581b0, cs, cs, sg, sg, og, og, short, p, ground, outlet); // "... unease at the furiously competitive atmosphere ..."
    } else {
        match w.s[10] {
            0 => story_lang!(para, 0xa57c24, 0x64, sg, sg, cs, cs, og, og, p, ground, short, outlet, ms),
            1 => story_lang!(para, 0xa57dd4, og, og, cs, cs, pg, pg, sg, sg, outlet, short, pos, p, ground, ms),
            _ => story_lang!(para, 0xa57f98, og, og, cs, cs, pg, pg, sg, sg, cs, cs, outlet, short, pos, p, ground, short, ms, p),
        }
    }
}

/// Your reply (slot 18, 1..3) into `reply`, false when none was written: when the player is the human manager on
/// screen (the original compares the player, not the manager), "You have indicated that ..." (1 part of your plans
/// 0xa57adc, 2 surplus 0xa57bc8 or learn to handle the pressure 0xa57b3c by slot 21, 3 you ignored the story
/// 0xa56dd4); otherwise the manager's (1 0xa57928, 2 0xa57a60 or 0xa579b0, 3 the press officer 0xa56c58).
unsafe fn wants_first_team_reply(w: &WantsFirstTeam, reply: usize) -> bool {
    let human = read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300));
    let (p, ms) = (w.player.ptr(), w.manager.ptr());
    let surplus = w.s[21] != 0;
    if staff_at(w.s[0] as isize).addr() == human {
        match w.s[18] {
            1 => story_lang!(reply, 0xa57adc, 0x64, p),
            2 => story_lang!(reply, if surplus { 0xa57bc8 } else { 0xa57b3c }, p),
            3 => story_lang!(reply, 0xa56dd4), // "You have chosen to ignore this story"
            _ => return false,
        }
    } else {
        match w.s[18] {
            1 => story_lang!(reply, 0xa57928, 0x64, 0x64, ms, p),
            2 => story_lang!(reply, if surplus { 0xa57a60 } else { 0xa579b0 }, 0x64, 0x64, ms, p),
            3 => {
                let g = w.club.gender_name() as i8 as i32;
                story_lang!(reply, 0xa56c58, g, g, w.club.addr() + club::SHORT_NAME, ms); // the press officer
            }
            _ => return false,
        }
    }
    true
}

/// The body of news item 0xbe8 by the case (slot 15): 2 your empty promises (with a promise and a reassurance 1..3
/// dismissed 0xa586fc; a rating above 25 unconvinced by your sincerity 0xa58688; else 0xa585c8); 1 your reassurance
/// (0xa58520 first with a reassurance of 1 or 2, then by the second rating: below 21 0xa58490, above 34 0xa58414, else
/// 0xa58520); 0 the story (`wants_first_team_story`) and, with a reply, your reply (`wants_first_team_reply`) after a
/// blank line; other cases leave the text unwritten. Links: the club's short and long names, the player's two names
/// and the manager when the names match.
unsafe fn wants_first_team_body(w: &WantsFirstTeam, t: &mut MediaText) {
    let (lg, long) = club_long_name(w.club);
    let p = w.player.ptr();
    covered!("sxB_2 wants first team body");
    match w.s[15] {
        2 => {
            if w.s[20] != 0 && matches!(w.s[19], 1 | 2 | 3) {
                story_lang!(t.buf(), 0xa586fc, lg, lg, p, long); // "... angrily dismissed your empty promises ..."
            } else if w.s[16] > 0x19 {
                story_lang!(t.buf(), 0xa58688, p);
            } else {
                story_lang!(t.buf(), 0xa585c8, lg, lg, p, long);
            }
        }
        1 => {
            if matches!(w.s[19], 1 | 2) {
                story_lang!(t.buf(), 0xa58520, lg, lg, p, long); // overwritten below, as in the original
            }
            if w.s[17] < 0x15 {
                story_lang!(t.buf(), 0xa58490, p);
            } else if w.s[17] > 0x22 {
                story_lang!(t.buf(), 0xa58414, p);
            } else {
                story_lang!(t.buf(), 0xa58520, lg, lg, p, long);
            }
        }
        0 => {
            let mut para = [0u8; 0x1388];
            let mut reply = [0u8; 0x1388];
            wants_first_team_story(w, para.as_mut_ptr() as usize);
            if w.s[18] == 0 {
                story_crt!(t.buf(), 0x991944, para.as_ptr()); // "%s"
            } else if wants_first_team_reply(w, reply.as_mut_ptr() as usize) {
                story_crt!(t.buf(), 0x991948, para.as_ptr(), reply.as_ptr()); // "%s\n\n%s"
            } else {
                t.unset(); // the original joins its unwritten reply buffer
            }
        }
        _ => t.unset(),
    }
    t.set();
    t.link_club_short(w.club);
    t.link(long, 0x7d5, w.club.addr());
    if w.player.matches() {
        w.player.link(t);
        w.player_short.link(t);
    }
    if w.manager.matches() {
        w.manager.link(t);
    }
}

/// The headline of news item 0xbe8 (the player's short name, 0x190, style 5): case 0 "desperate to play" (variant
/// set) or "wants first team football", 1 "thanks his manager", 2 "unconvinced by his manager"; other cases leave it
/// unwritten. The name links to the player when the names match.
unsafe fn wants_first_team_headline(w: &WantsFirstTeam, t: &mut MediaText) {
    let short = w.player.restyled(0x190, 5);
    let tpl = match w.s[15] {
        0 if w.s[10] != 0 => 0xa57878, // "... desperate to play"
        0 => 0xa57838, // "... wants first team football"
        1 => 0xa578b0, // "... thanks his manager"
        2 => 0xa578e8, // "... unconvinced by his manager"
        _ => 0,
    };
    covered!("sxB_2 wants first team headline");
    if tpl != 0 {
        story_lang!(t.buf(), tpl, short.ptr());
    } else {
        t.unset();
    }
    t.set();
    if short.matches() {
        short.link(t);
    }
}

/// Writes the headline or body of news item 0xbe8, where a player wants first team football, with your reply and his
/// reaction (`WantsFirstTeam`, `wants_first_team_body`, `wants_first_team_headline`).
/// Original: `Media::write_wants_first_team_news_text` (0x75a940, media.cpp, thiscall, ret 0xc). Defects of the
/// original: a case other than 0..2, or case 0 with a reply other than 1..3, hands the Text a buffer it never wrote;
/// your reply compares the player, not the manager, with the human manager.
pub(crate) unsafe extern "thiscall" fn write_wants_first_team_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let w = WantsFirstTeam::read(news);
    let mut t = MediaText::new(out);
    if body != 0 {
        wants_first_team_body(&w, &mut t);
    } else {
        wants_first_team_headline(&w, &mut t);
    }
}

/// Comparer for `write_wants_first_team_news_text` (`compare_media_text_writer`, the case in slot 15).
pub(crate) unsafe extern "thiscall" fn compare_write_wants_first_team_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_media_text_writer("write_wants_first_team_news_text", news, body, 15, out, || o(this, news, body, out), |t| write_wants_first_team_news_text(this, news, body, t));
}

/// What the text writer of news item 0xbd9 (a player's outburst) reads and prepares: slots 0..3 the player, 4 the
/// club, 5 the captain, 6 the kind (0x52 the lack of service), 7 the media story, 8 the assistant manager's report, 10
/// your reply, 11..14 the assistant (id and names), 15 the match flag; with a match its record
/// (`read_match_record_from_news`, the slot index moving on) and one more slot; then the outlet type and the pundit
/// (slot 9 is skipped). The media nation is the club's, or, for the lack of service and a draw (`exe::utils_random(2)`)
/// when the player's nation has a reputation above 5000, the player's. The outlet uses the pundit when the club has a
/// nation, the outlet and the pundit are set and the media is local (for the media nation); the club's manager is
/// `club::MANAGER` (0 none).
struct Outburst {
    player: [u32; 4],
    club: Club,
    captain: u8,
    kind: i32,
    media: u32,
    assistant_report: u32,
    reply: u32,
    assistant: [i32; 4],
    has_match: bool,
    m: BanMatch,
    outlet: [u8; 0x1f4],
    outlet_gender: u8,
    manager: usize,
    human: usize,
}

impl Outburst {
    unsafe fn read(news: usize) -> Self {
        let slot = |k: u8| exe::newsitem_get_slot(news, k as u32);
        let player = [slot(0), slot(1), slot(2), slot(3)];
        let club_id = slot(4) as i32;
        let captain = slot(5) as u8;
        let kind = slot(6) as i32;
        let media = slot(7);
        let assistant_report = slot(8);
        let reply = slot(10);
        let assistant = [slot(11) as i32, slot(12) as i32, slot(13) as i32, slot(14) as i32];
        let has_match = slot(15) != 0;
        let mut m = BanMatch([0; 0x20]);
        m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
        let mut next: u8 = 16;
        if has_match {
            exe::read_match_record_from_news(news, &mut next, m.0.as_mut_ptr());
            slot(next);
            next = next.wrapping_add(1);
        }
        let outlet_type = slot(next) as i32;
        let pundit = slot(next.wrapping_add(1)) as i32;
        let club = club_at(club_id as isize);
        let human = read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300));
        let mut nation = club.nation().addr();
        if kind == 0x52 && exe::utils_random(2) != 0 {
            let own = staff_at(player[0] as i32 as isize).nation();
            if own.reputation() > 0x1388 {
                nation = own.addr();
            }
        }
        let local = media_is_local(nation);
        let mut o = Outburst { player, club, captain, kind, media, assistant_report, reply, assistant, has_match, m, outlet: [0; 0x1f4], outlet_gender: 0, manager: 0, human };
        let pundit = (club.nation().addr() != 0 && outlet_type != -1 && pundit != -1 && local).then_some(pundit);
        o.outlet_gender = media_outlet_name(outlet_type, pundit, nation, local, o.outlet.as_mut_ptr() as usize);
        o.manager = club.manager().addr();
        o
    }
    unsafe fn name(&self, style: u32) -> NewsName {
        NewsName::new(self.player[0], [self.player[1], self.player[2], self.player[3]], 0x78, style)
    }
    /// The assistant reports and the human manages the club.
    fn report_to_human(&self) -> bool {
        self.assistant_report != 0 && self.human == self.manager
    }
}

/// The assistant manager's report on the player (slot 8 with an assistant id): as captain (strip him of the
/// captaincy) or not, his own words (slot 7: "Your assistant manager blames ...") or "Your assistant <name> believes
/// ...", with a match by the club's result (draw, win, defeat: the score and the opponent's long name), without one
/// "in the last match".
unsafe fn outburst_assistant_report(o: &Outburst, para: usize, p: usize, assistant: usize, score: usize) {
    let (g1, g2) = o.m.goals();
    let result = if g1 == g2 { 0 } else if g1 > g2 { 1 } else { 2 };
    let opponent = club_at(read_i32(o.m.at(0x15)) as isize);
    let (og, opp) = club_long_name(opponent);
    // [blames: draw, win, defeat, last match], [believes: draw, win, defeat, last match]
    let forms: [[usize; 4]; 2] = if o.captain != 0 {
        [[0xa27004, 0xa26f18, 0xa26e2c, 0xa26d84], [0xa26c6c, 0xa26b54, 0xa26a38, 0xa26954]]
    } else {
        [[0xa268a8, 0xa267fc, 0xa2674c, 0xa266d8], [0xa265fc, 0xa26520, 0xa26444, 0xa263a0]]
    };
    if o.media != 0 {
        if o.has_match {
            story_lang!(para, forms[0][result], 0x64, og, og, p, score, opp);
        } else {
            story_lang!(para, forms[0][3], 0x64, p);
        }
    } else if o.has_match {
        story_lang!(para, forms[1][result], og, og, assistant, p, score, opp);
    } else {
        story_lang!(para, forms[1][3], assistant, p);
    }
}

/// The body of news item 0xbd9 (the player's full name twice, 0x78, style 1; the score "%d-%d" of the match, more
/// first): the lack of service (with the media story 0xa271e0, else 0xa270f0); the assistant's report
/// (`outburst_assistant_report`, the assistant's full name, 0x190); else unhappy with his team-mates (captain: media
/// story 0xa262bc, else 0xa261d4; not captain 0xa260f8, else 0xa2601c). Then your reply, or his manager's when the
/// human does not manage the club (slot 10: 1 the player may have a point, 2 dealt with internally, other the empty
/// text 0xad9c64), after a blank line. Links: the player's two names when the names match, the assistant when set and
/// his names match, the club's short name, the club's nation (`exe::database_get_national_team`: its name, short name
/// and nationality adjectives of forms 0x16..0x1f), and with a match the score when the fixture matches and the
/// opponent's long name.
unsafe fn outburst_body(o: &Outburst, t: &mut MediaText) {
    let name = o.name(1);
    let name2 = o.name(1);
    let mut score = [0u8; 0x14];
    let sp = score.as_mut_ptr() as usize;
    if o.has_match {
        let (g1, g2) = o.m.goals();
        story_crt!(sp, 0x98c114, g1.max(g2) as i32, g1.min(g2) as i32); // "%d-%d"
    }
    let (cs, short) = club_short_name(o.club);
    let og = o.outlet_gender as i8 as i32;
    let (p, outlet) = (name.ptr(), o.outlet.as_ptr());
    let mut assistant = [0u8; 0x190];
    let mut para = [0u8; 0x1388];
    let pp = para.as_mut_ptr() as usize;
    let mut reply = [0u8; 0x1f4];
    covered!("sxB_2 outburst body");
    if o.kind == 0x52 {
        if o.media != 0 {
            story_lang!(pp, 0xa271e0, og, og, cs, cs, outlet, short, p, name2.ptr());
        } else {
            story_lang!(pp, 0xa270f0, cs, cs, og, og, short, p, outlet);
        }
    } else if o.assistant_report != 0 && o.assistant[0] > -1 {
        let [id, a, b, c] = o.assistant;
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, assistant.as_mut_ptr(), 0x190, 1, id as u32);
        outburst_assistant_report(o, pp, p, assistant.as_ptr() as usize, sp);
    } else {
        let tpl = match (o.captain != 0, o.media != 0) {
            (true, true) => 0xa262bc,
            (true, false) => 0xa261d4,
            (false, true) => 0xa260f8,
            (false, false) => 0xa2601c,
        };
        if o.media != 0 {
            story_lang!(pp, tpl, og, og, cs, cs, outlet, short, p);
        } else {
            story_lang!(pp, tpl, cs, cs, og, og, short, p, outlet);
        }
    }
    let rp = reply.as_mut_ptr() as usize;
    let mine = o.human == o.manager;
    match o.reply {
        1 => story_lang!(rp, if mine { 0xa25f7c } else { 0xa25eec }), // "... the player may have a point."
        2 => story_lang!(rp, if mine { 0xa25fc0 } else { 0xa25f28 }), // "... dealt with internally by the club."
        _ => story_crt!(rp, 0xad9c64),
    }
    story_crt!(t.buf(), 0x991948, pp, rp); // "%s\n\n%s"
    t.set();
    if name.matches() {
        name.link(t);
        name2.link(t);
    }
    if o.assistant[0] > -1 {
        let [id, a, b, c] = o.assistant;
        if exe::database_staff_matches_names(staff_at(id as isize), a as u32, b as u32, c as u32, 0usize) != 0 {
            t.link(assistant.as_ptr() as usize, 0x7d7, staff_at(id as isize).addr());
        }
    }
    t.link_club_short(o.club);
    let nation = o.club.nation();
    t.link(nation.addr() + 4, 0x7d5, exe::database_get_national_team(nation)); // the nation's name
    t.link(nation.addr() + nation::SHORT_NAME, 0x7d5, exe::database_get_national_team(nation));
    let mut adjective = [0u8; 0x78];
    for form in 0x16..=0x1f {
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x78, form);
        t.link(adjective.as_ptr() as usize, 0x7d5, exe::database_get_national_team(nation));
    }
    if o.has_match {
        let m = &o.m;
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), m.fixture() as i32, read_i32(m.at(0x11)),
            read_i32(m.at(0x15)), read_i32(m.at(0)), read_i32(m.at(2))) != 0 {
            t.link(sp, 0x7d3, m.fixture() as usize);
        }
        let opponent = club_at(read_i32(m.at(0x15)) as isize);
        t.link(opponent.addr() + club::NAME, 0x7d5, opponent.addr());
    }
}

/// The headline of news item 0xbd9 (the player's short name, 0x78, style 5): the lack of service "blasts lack of
/// service"; the assistant's report "Feedback from Assistant Manager"; a media story when not captain "blast at club";
/// else "disheartened". When the assistant reports to the human who manages the club, no link; otherwise, when the
/// player's names match, the player's and the club's short name's links.
unsafe fn outburst_headline(o: &Outburst, t: &mut MediaText) {
    let name = o.name(5);
    covered!("sxB_2 outburst headline");
    if o.kind == 0x52 {
        story_lang!(t.buf(), 0xa273d4, name.ptr()); // "... blasts lack of service"
    } else if o.assistant_report != 0 && o.assistant[0] > -1 {
        story_lang!(t.buf(), 0xa273b4); // "Feedback from Assistant Manager"
    } else if o.media != 0 && o.captain == 0 {
        story_lang!(t.buf(), 0xa27380, 0x64, name.ptr()); // "...{s} blast at club"
    } else {
        story_lang!(t.buf(), 0xa27350, name.ptr()); // "... disheartened"
    }
    t.set();
    if o.report_to_human() {
        return;
    }
    if name.matches() {
        name.link(t);
        t.link_club_short(o.club);
    }
}

/// Writes the headline or body of news item 0xbd9: a player's outburst at the club or his team-mates, or the assistant
/// manager's report on his bad attitude (`Outburst`, `outburst_body`, `outburst_headline`).
/// Original: `Media::write_player_outburst_news_text` (0x733fc0, media.cpp, thiscall, ret 0xc). Defect of the
/// original: with an assistant id but no assistant report, the body links the assistant's name buffer it never
/// wrote (the copy links an empty name).
pub(crate) unsafe extern "thiscall" fn write_player_outburst_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let o = Outburst::read(news);
    let mut t = MediaText::new(out);
    if body != 0 {
        outburst_body(&o, &mut t);
    } else {
        outburst_headline(&o, &mut t);
    }
}

/// Comparer for `write_player_outburst_news_text` (`compare_media_text_writer`, the kind in slot 6).
pub(crate) unsafe extern "thiscall" fn compare_write_player_outburst_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_media_text_writer("write_player_outburst_news_text", news, body, 6, out, || o(this, news, body, out), |t| write_player_outburst_news_text(this, news, body, t));
}
