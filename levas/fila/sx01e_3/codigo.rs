// --- sx01e_3: the media text writers (media.cpp): apology, backing request, criticism reaction, FA complaint ---

/// Set by a media text copy where the original hands the Text its frame buffer without writing a template into it (a
/// kind or variant with no case: the text is whatever the stack held). The copy writes an empty text there and
/// `compare_media_text_writer` accepts any text.
pub(crate) static MEDIA_TEXT_UNSET: AtomicBool = AtomicBool::new(false);

/// The media text a copy writes: its own buffer (the originals' frame buffers hold 0x400 to 0x4000 bytes) or, as some
/// originals do, the shared buffer `PRAISE_TEXT_BUFFER` (0xde1f64), and the Text.
pub(crate) struct MediaText {
    out: usize,
    shared: bool,
    text: [u8; 0x4000],
}

/// Writes a langlib template (`FN_LANGLIB_SPRINTF`) into a `MediaText`'s buffer.
macro_rules! media_text {
    ($t:expr, $tpl:expr $(, $a:expr)* $(,)?) => {
        story_lang!($t.buf(), $tpl $(, $a)*)
    };
}

impl MediaText {
    fn new(out: usize) -> Self {
        MediaText { out, shared: false, text: [0; 0x4000] }
    }
    /// A text formatted in the shared buffer 0xde1f64.
    fn shared(out: usize) -> Self {
        MediaText { out, shared: true, text: [0; 0x4000] }
    }
    fn buf(&mut self) -> usize {
        if self.shared { PRAISE_TEXT_BUFFER } else { self.text.as_mut_ptr() as usize }
    }
    /// The original copies its unwritten frame buffer: an empty text here (`MEDIA_TEXT_UNSET`).
    unsafe fn unset(&mut self) {
        MEDIA_TEXT_UNSET.store(true, Relaxed);
        write_u8(self.buf(), 0);
    }
    /// Puts the text into the Text (`exe::text_set_string`).
    unsafe fn set(&mut self) {
        let b = self.buf();
        exe::text_set_string(self.out, b as *const u8);
    }
    /// Links a text to a record (`exe::text_add_link`: 0x7d5 club, 0x7d7 person...).
    unsafe fn link(&self, text: usize, kind: u32, record: usize) {
        exe::text_add_link(self.out, text as *const u8, kind, record);
    }
    /// Links a club's short name to the club (0x7d5).
    unsafe fn link_club_short(&self, c: Club) {
        self.link(c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
    }
}

/// A person named in a media text: his staff id and three name ids from the news slots, and the name written by
/// `exe::database_format_name_from_ids` (style 1 the full name, 5 the short one) into a buffer of the original's size.
pub(crate) struct NewsName {
    id: u32,
    names: [u32; 3],
    text: [u8; 0x190],
}

impl NewsName {
    /// The person of slots k (id) and k + 1..k + 3 (names), formatted with the buffer size and style.
    unsafe fn read(news: usize, k: u32, size: u32, style: u32) -> Self {
        let s = |i: u32| exe::newsitem_get_slot(news, k + i);
        Self::new(s(0), [s(1), s(2), s(3)], size, style)
    }
    unsafe fn new(id: u32, names: [u32; 3], size: u32, style: u32) -> Self {
        let mut n = NewsName { id, names, text: [0; 0x190] };
        exe::database_format_name_from_ids(names[0], names[1], names[2], n.text.as_mut_ptr(), size, style, id);
        n
    }
    fn ptr(&self) -> usize {
        self.text.as_ptr() as usize
    }
    /// The same person in another style (the original formats a second buffer from the same slots).
    unsafe fn restyled(&self, size: u32, style: u32) -> Self {
        Self::new(self.id, self.names, size, style)
    }
    /// The person still bears the names (`exe::database_staff_matches_names`).
    unsafe fn matches(&self) -> bool {
        exe::database_staff_matches_names(staff_at(self.id as i32 as isize), self.names[0], self.names[1], self.names[2], 0usize) != 0
    }
    /// Links the name to the person (0x7d7).
    unsafe fn link(&self, t: &MediaText) {
        t.link(self.ptr(), 0x7d7, staff_at(self.id as i32 as isize).addr());
    }
}

/// A club's long name and its gender byte (`club::NAME`, `.gender_name()`).
unsafe fn club_long_name(c: Club) -> (i32, usize) {
    (c.gender_name() as i32, c.addr() + club::NAME)
}

/// A club's short name and its gender byte (`club::SHORT_NAME`, `.short_gender_name()`).
unsafe fn club_short_name(c: Club) -> (i32, usize) {
    (c.short_gender_name() as i8 as i32, c.addr() + club::SHORT_NAME)
}

/// The comparer of the media text writers of sx01e: `compare_text_writer` (the original into `out`, the copy into a
/// local Text, the generator restored and checked), the text not compared when the copy left it unset
/// (`MEDIA_TEXT_UNSET`); the message names the writer, the part and the slot `kind_slot`.
pub(crate) unsafe fn compare_media_text_writer(what: &str, news: usize, body: u32, kind_slot: u32, out: usize, original: impl FnOnce(), rust: impl FnOnce(usize)) {
    MEDIA_TEXT_UNSET.store(false, Relaxed);
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, original, || crate::rng::without_counting(|| rust(lp)));
    let (mo, mr) = (text_contents(out), text_contents(lp));
    exe::drop_text(local.as_mut_ptr());
    let unset = MEDIA_TEXT_UNSET.swap(false, Relaxed);
    let show = |t: &(Vec<u8>, Vec<(Vec<u8>, u32, u32)>)| {
        format!("{:?} {:x?}", String::from_utf8_lossy(&t.0), t.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>())
    };
    SCOREBOARD.record(same_rng && (unset || mo.0 == mr.0) && mo.1 == mr.1, || {
        format!("{}({}, slot {} = {}, unset {}): original {} rust {}", what, if body != 0 { "body" } else { "headline" }, kind_slot,
            exe::newsitem_get_slot(news, kind_slot) as i32, unset, show(&mo), show(&mr))
    });
}

/// Writes the body of news item 0xbdb, a player's apology (`write_player_apology_news_text`): slots 0..3 the player,
/// 4 the club, 5 the kind, 7 the variant; the player's full name (0x190, style 1). By kind: 0x25 he will fight for
/// his place; 0x78 sorry for his form, thanks for your defence; 0x11 his form fell short, your defence is
/// appreciated; 0, 2..5, 0xd, 0xe by variant (0 his form not up to the club's standards, with the club's long name; 1
/// a press report of his apology; 2 sorry, your criticism was fair; other an apology to the supporters); 1 embarrassed
/// about media quotes; 0x1e an apology to his team-mates; 0x20 an apology to the club's supporters over quotes, with
/// the club's short name; 6 his disciplinary record; 0x23 comments about you; 8 missing training; 0x13 his playing
/// position; 0xa petulance after a substitution; others (and kinds above 0x78) his unprofessional behaviour. The name
/// links to the player when the names match.
unsafe fn apology_body(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 1);
    let club = club_at(exe::newsitem_get_slot(news, 4) as i32 as isize);
    let kind = exe::newsitem_get_slot(news, 5);
    exe::newsitem_get_slot(news, 6);
    let variant = exe::newsitem_get_slot(news, 7) as i32;
    let p = player.ptr();
    covered!("sx01e_3 apology text body");
    match kind {
        0x25 => media_text!(t, 0xa18840, p), // "... is prepared to fight for his place in the team."
        0x78 => media_text!(t, 0xa1877c, p),
        0x11 => media_text!(t, 0xa18694, p),
        0 | 2..=5 | 0xd | 0xe => match variant {
            0 => {
                let (g, name) = club_long_name(club);
                media_text!(t, 0xa183c0, g, g, p, name); // "... not been up to the standards expected by {}<club>{} ..."
            }
            1 => media_text!(t, 0xa1848c, 0x64, p), // "A press report appeared today containing an apology by ..."
            2 => media_text!(t, 0xa18510, p),
            _ => media_text!(t, 0xa185d0, p), // "... has publicly apologised to club supporters ..."
        },
        1 => media_text!(t, 0xa18308, p),
        0x1e => media_text!(t, 0xa18228, p),
        0x20 => {
            let (g, name) = club_short_name(club);
            media_text!(t, 0xa18130, g, g, p, name); // "... publicly apologised to {}<club>{} supporters over quotes ..."
        }
        6 => media_text!(t, 0xa18078, p),
        0x23 => media_text!(t, 0xa17f8c, p),
        8 => media_text!(t, 0xa17f00, p),
        0x13 => media_text!(t, 0xa17e34, p),
        0xa => media_text!(t, 0xa17d98, p),
        _ => media_text!(t, 0xa17cd8, p), // "... letting the club down with his recent unprofessional behaviour ..."
    }
    t.set();
    if player.matches() {
        player.link(t);
    }
}

/// Writes the headline of news item 0xbdb: the player's short name (0x190, style 5); kind 0x25 "will fight for his
/// place", otherwise by the variant's parity "apologises" (odd) or "admits his mistake" (even); the name links to the
/// player when the names match.
unsafe fn apology_headline(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 5);
    let kind = exe::newsitem_get_slot(news, 5);
    let variant = exe::newsitem_get_slot(news, 7) as i32;
    covered!("sx01e_3 apology text headline");
    let tpl = if kind == 0x25 {
        0xa17c80 // "... indicates he will fight for his place in the team"
    } else if variant % 2 != 0 {
        0xa17c54 // "... apologises"
    } else {
        0xa17c20 // "... admits his mistake"
    };
    media_text!(t, tpl, player.ptr());
    t.set();
    if player.matches() {
        player.link(t);
    }
}

/// Writes the headline or body of news item 0xbdb, in which a player apologises for an outburst, his form, media
/// quotes or his behaviour (`apology_body`, `apology_headline`).
/// Original: `Media::write_player_apology_news_text` (0x72bd50, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_apology_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::new(out);
    if body != 0 {
        apology_body(news, &mut t);
    } else {
        apology_headline(news, &mut t);
    }
}

/// Comparer for `write_player_apology_news_text` (`compare_media_text_writer`, the kind in slot 5).
pub(crate) unsafe extern "thiscall" fn compare_write_player_apology_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_media_text_writer("write_player_apology_news_text", news, body, 5, out, || o(this, news, body, out), |t| write_player_apology_news_text(this, news, body, t));
}

/// Writes the body of news item 0xbcf, a player or his agent asking you for support against the media: slots 0..3 the
/// player (full name, 0x190, style 1), 4 the club, 5 the kind, 6 the media topic, 7 your reply, 8 the case, 9 the
/// "no longer needed" mark. Kind 2 is the player himself: case 3 the end of his career at the club; topic 2 he wants
/// public support; topic 3 misquoted (reply 2), upset at your answer to his honesty (reply 4) or doubts about his
/// future; other topics a training ground argument. Otherwise his agent: case 3 his player talks to other clubs (with
/// the mark) or wants your help to leave the club (short name); topic 2 or 3 more support (3: misquoted); reply 4 a
/// fax to accept the player's assessment or he moves on; reply 0 disbelief at your attack; others more support or he
/// moves on. The name links to the player when the names match.
unsafe fn backing_request_body(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 1);
    let club = club_at(exe::newsitem_get_slot(news, 4) as i32 as isize);
    let s = |k: u32| exe::newsitem_get_slot(news, k) as i32;
    let (kind, topic, reply, case, unwanted) = (s(5), s(6), s(7), s(8), s(9));
    let p = player.ptr();
    if kind == 2 {
        covered!("sx01e_3 backing request text from the player");
        if case == 3 {
            media_text!(t, 0xa3879c, p); // "... very disappointed by your public announcement of the end of his career ..."
        } else if topic == 2 {
            media_text!(t, 0xa386f0, 0x64, p);
        } else if topic == 3 {
            match reply {
                2 => media_text!(t, 0xa385d0, p), // "... upset that he has been misquoted by the media ..."
                4 => media_text!(t, 0xa384fc, p), // "... upset at your response to his honesty to the press ..."
                _ => media_text!(t, 0xa38408, p), // "... the media are casting doubts on his future at the club ..."
            }
        } else {
            media_text!(t, 0xa3834c, p); // "In a training ground argument ..."
        }
    } else {
        covered!("sx01e_3 backing request text from the agent");
        if case == 3 {
            let (g, name) = club_short_name(club);
            let tpl = if unwanted != 0 { 0xa38d18 } else { 0xa38c2c }; // "... talking to other clubs" / "... facilitate his player's move"
            media_text!(t, tpl, 0x64, g, g, p, name);
        } else if topic == 2 {
            media_text!(t, 0xa38ba4, 0x64, p);
        } else if topic == 3 {
            media_text!(t, 0xa38ad0, 0x64, p);
        } else if reply == 4 {
            media_text!(t, 0xa38a04, 0x64, p); // "... informed you in a fax ..."
        } else if reply == 0 {
            media_text!(t, 0xa38928, 0x64, p); // "... expressed disbelief at your vicious attack on him ..."
        } else {
            media_text!(t, 0xa3882c, 0x64, p, p);
        }
    }
    t.set();
    if player.matches() {
        player.link(t);
    }
}

/// Writes the headline of news item 0xbcf (the player's short name, 0x190, style 5): kind 2 "requests backing"
/// (topic 2), "responds to criticism" (reply 4) or "demands support"; otherwise the agent's "request", "responds to
/// criticism" or "Agent demands support"; the name links to the player when the names match.
unsafe fn backing_request_headline(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 5);
    let s = |k: u32| exe::newsitem_get_slot(news, k) as i32;
    let (kind, topic, reply) = (s(5), s(6), s(7));
    let p = player.ptr();
    match (kind == 2, topic == 2, reply == 4) {
        (true, true, _) => media_text!(t, 0xa38264, p), // "... requests backing"
        (true, false, true) => media_text!(t, 0xa3822c, p), // "... responds to criticism"
        (true, false, false) => media_text!(t, 0xa381f8, p), // "... demands support"
        (false, true, _) => media_text!(t, 0xa38318, 0x64, p), // "...{} agent request"
        (false, false, true) => media_text!(t, 0xa382d8, 0x64, p), // "...{} agent responds to criticism"
        (false, false, false) => media_text!(t, 0xa38298, 0x64, p), // "Agent demands support for ..."
    }
    t.set();
    if player.matches() {
        player.link(t);
    }
}

/// Writes the headline or body of news item 0xbcf, where a player or his agent asks the manager for support against
/// the media (`backing_request_body`, `backing_request_headline`).
/// Original: `Media::write_player_backing_request_news_text` (0x7435a0, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_backing_request_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::new(out);
    if body != 0 {
        backing_request_body(news, &mut t);
    } else {
        backing_request_headline(news, &mut t);
    }
}

/// Comparer for `write_player_backing_request_news_text` (`compare_media_text_writer`, the topic in slot 6).
pub(crate) unsafe extern "thiscall" fn compare_write_player_backing_request_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_media_text_writer("write_player_backing_request_news_text", news, body, 6, out, || o(this, news, body, out), |t| write_player_backing_request_news_text(this, news, body, t));
}

/// Writes the body of news item 0xbdc, a player answering his manager's criticism or the media attention: slots 0..3
/// the player (full and short names, 0x78), 4 the club (short name), 5 the case, 6 the variant, 9..12 the manager
/// (full and short names, 0x78). Case 0: variant 2 he acknowledged the criticism, otherwise he is unperturbed; case 1:
/// variant 1 or 3 unaffected (a player of his ability), 2 he is too important for the manager to act; case 2 (and any
/// case above 4): variant 1 or 3 unruffled (the fans will back him), 2 unconcerned (a loyal servant); case 3 he
/// dismissed the criticism (the manager will not keep his job), 4 likewise (he will outlast the manager). Cases 1 and
/// 2 with another variant leave the text unwritten (the original copies its uninitialised buffer, `MEDIA_TEXT_UNSET`).
/// Links: the player's two names and the manager's two names when the names match, the club's short name.
unsafe fn criticism_reaction_body(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x78, 1);
    let player_short = player.restyled(0x78, 5);
    let club = club_at(exe::newsitem_get_slot(news, 4) as i32 as isize);
    let case = exe::newsitem_get_slot(news, 5);
    let variant = exe::newsitem_get_slot(news, 6) as i32;
    exe::newsitem_get_slot(news, 7);
    let manager = NewsName::read(news, 9, 0x78, 1);
    let manager_short = manager.restyled(0x78, 5);
    let (g, name) = club_short_name(club);
    let (p, m, ms) = (player.ptr(), manager.ptr(), manager_short.ptr());
    covered!("sx01e_3 criticism reaction text body");
    match case {
        0 if variant == 2 => media_text!(t, 0xa392a0, g, g, 0x64, name, p, m, ms), // "... has acknowledged manager ...{s} criticism"
        0 => media_text!(t, 0xa393a4, g, g, name, p), // "... appears to be unperturbed by the media attention ..."
        3 => media_text!(t, 0xa395d8, g, g, 0x64, name, p, m, ms), // "... doesn't expect ... to keep his job ..."
        4 => media_text!(t, 0xa39498, g, g, 0x64, name, p, m, ms), // "... is likely to leave the club before he does."
        1 => match variant {
            1 | 3 => media_text!(t, 0xa39064, g, g, name, p), // "... a player of his own ability is not easily distracted ..."
            2 => media_text!(t, 0xa39158, g, g, 0x64, name, p, m, ms), // "... so important to the club ..."
            _ => t.unset(),
        },
        _ => match variant {
            1 | 3 => media_text!(t, 0xa38e40, g, g, name, p), // "... the fans will back him through any difficulties ..."
            2 => media_text!(t, 0xa38f40, g, g, 0x64, name, p, m), // "... a loyal servant of the club ..."
            _ => t.unset(),
        },
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
    t.link_club_short(club);
}

/// Writes the headline of news item 0xbdc (the player's short name, 0x190, style 5): "questions boss" with slot 7 set
/// and case 3 or 4, otherwise "quoted in media"; the name links to the player when the names match.
unsafe fn criticism_reaction_headline(news: usize, t: &mut MediaText) {
    let player = NewsName::read(news, 0, 0x190, 5);
    let case = exe::newsitem_get_slot(news, 5);
    let reply = exe::newsitem_get_slot(news, 7);
    let tpl = if reply != 0 && (case == 3 || case == 4) { 0xa38e10 } else { 0xa38ddc }; // "... questions boss" / "... quoted in media"
    media_text!(t, tpl, player.ptr());
    t.set();
    if player.matches() {
        player.link(t);
    }
}

/// Writes the headline or body of news item 0xbdc, where a player answers his manager's criticism or the media
/// attention he receives (`criticism_reaction_body`, `criticism_reaction_headline`).
/// Original: `Media::write_player_criticism_reaction_news_text` (0x743d30, media.cpp, thiscall, ret 0xc). Defect of
/// the original: cases 1 and 2 (and any case above 4) with a variant other than 1, 2 or 3 copy the uninitialised text
/// buffer into the Text.
pub(crate) unsafe extern "thiscall" fn write_player_criticism_reaction_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::new(out);
    if body != 0 {
        criticism_reaction_body(news, &mut t);
    } else {
        criticism_reaction_headline(news, &mut t);
    }
}

/// Comparer for `write_player_criticism_reaction_news_text` (`compare_media_text_writer`, the case in slot 5).
pub(crate) unsafe extern "thiscall" fn compare_write_player_criticism_reaction_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_media_text_writer("write_player_criticism_reaction_news_text", news, body, 5, out, || o(this, news, body, out), |t| write_player_criticism_reaction_news_text(this, news, body, t));
}

/// Writes the body of news item (6, 0x178e), a manager complaining to the FA: slots 0 the club (long and short names),
/// 1..4 the manager (full and short names, 0x190), 5 the case, 6..9 the player (full name), 10 the games of his ban
/// (-1 for a ban in days), 11 the days, 12 a nation (its adjective, `exe::nation_get_nationality_adjective`, fetched
/// and not used), 13 the matches of the touchline ban. Case 0 the manager stood by the player; 1 cap in hand to the
/// FA for the removal of his ban (games or days); 2 outspoken against the refereeing; 3 a refereeing conspiracy; 4 the
/// FA losing patience; 5 the FA fined the club 25000 (`exe::cash_get_display_amount`, `exe::cash_format_money`) and a
/// touchline ban; above 5 the text is unwritten (`MEDIA_TEXT_UNSET`). Links: the club's short and long names, the
/// player and the manager when the names match.
unsafe fn fa_complaint_body(news: usize, t: &mut MediaText, club: Club) {
    let manager = NewsName::read(news, 1, 0x190, 1);
    let manager_short = manager.restyled(0x190, 5);
    let player = NewsName::read(news, 6, 0x190, 1);
    let s = |k: u32| exe::newsitem_get_slot(news, k) as i32;
    let (case, games, days, matches) = (s(5), s(10), s(11), s(13));
    let (lg, long) = club_long_name(club);
    let (sg, short) = club_short_name(club);
    let (m, ms, p) = (manager.ptr(), manager_short.ptr(), player.ptr());
    covered!("sx01e_3 FA complaint text body");
    match case {
        0 => media_text!(t, 0xa56984, lg, lg, long, m, p, ms), // "... stood by ... today."
        1 if games != -1 => media_text!(t, 0xa567e8, lg, lg, 0x64, games, sg, sg, games, long, m, p, games, short, p, games),
        1 => media_text!(t, 0xa5665c, lg, lg, 0x64, sg, sg, long, m, p, days, short, p, days), // "... day ban."
        2 => media_text!(t, 0xa564c4, lg, lg, sg, sg, long, m, ms, short), // "Outspoken ... manager ..."
        3 => media_text!(t, 0xa56360, lg, lg, long, m, ms), // "... a refereeing conspiracy against his side."
        4 => media_text!(t, 0xa561e0, lg, lg, long, m, ms), // "Sources at the FA have said that they are losing patience ..."
        5 => {
            let fine: u32 = 0x61a8;
            let amount = exe::cash_get_display_amount(&fine, 6);
            let mut money = [0u8; 0x1f4];
            exe::cash_format_money(money.as_mut_ptr() as usize, 0x1f4, amount, 6);
            media_text!(t, 0xa56064, lg, lg, 0x64, matches, long, money.as_ptr(), m, ms, matches); // "The Football Association have fined ..."
        }
        _ => t.unset(),
    }
    t.set();
    t.link_club_short(club);
    t.link(long, 0x7d5, club.addr());
    if player.matches() {
        player.link(t);
    }
    if manager.matches() {
        manager.link(t);
    }
}

/// Writes the headline of news item (6, 0x178e) (the manager's and the player's short names, 0x190, style 5): 0 the
/// manager defends the player, 1 clutches at straws, 2 criticises refereeing, 3 attacks a referee conspiracy, 4 the FA
/// losing patience, 5 the FA fine the club (short name) over his outburst; above 5 unwritten (`MEDIA_TEXT_UNSET`).
/// Links: the manager when the names match, the club's short name.
unsafe fn fa_complaint_headline(news: usize, t: &mut MediaText, club: Club) {
    let manager = NewsName::read(news, 1, 0x190, 5);
    let player = NewsName::read(news, 6, 0x190, 5);
    let case = exe::newsitem_get_slot(news, 5);
    let m = manager.ptr();
    match case {
        0 => media_text!(t, 0xa56018, m, player.ptr()), // "... defends ..."
        1 => media_text!(t, 0xa55fe0, m), // "... clutches at straws"
        2 => media_text!(t, 0xa55f98, m), // "... criticises poor refereeing standards"
        3 => media_text!(t, 0xa55f58, m), // "... attacks referee conspiracy"
        4 => media_text!(t, 0xa55f0c, m), // "FA rumoured to be losing patience with ..."
        5 => {
            let (g, short) = club_short_name(club);
            media_text!(t, 0xa55eac, g, g, 0x64, short, m); // "FA fine {}<club>{} over ...{s} outburst"
        }
        _ => t.unset(),
    }
    t.set();
    if manager.matches() {
        manager.link(t);
    }
    t.link_club_short(club);
}

/// Writes the headline or body of news item (6, 0x178e), where a manager defends a player, attacks referees or annoys
/// the FA (`fa_complaint_body`, `fa_complaint_headline`). Before either, the nationality adjective of the nation in
/// slot 12 (-1 none) is written into a buffer that no template uses.
/// Original: `Media::write_manager_fa_complaint_news_text` (0x758fe0, media.cpp, thiscall, ret 0xc). Defect of the
/// original: a case above 5 copies the uninitialised text buffer into the Text.
pub(crate) unsafe extern "thiscall" fn write_manager_fa_complaint_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut t = MediaText::new(out);
    let club = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    let nation = exe::newsitem_get_slot(news, 12) as i32;
    let mut adjective = [0u8; 0x78];
    if nation != -1 {
        let n = read_usize(G_NATION_TABLE).wrapping_add((nation as usize).wrapping_mul(nation::SIZE));
        exe::nation_get_nationality_adjective(n, adjective.as_mut_ptr() as usize, 0x78, 9);
    }
    if body != 0 {
        fa_complaint_body(news, &mut t, club);
    } else {
        fa_complaint_headline(news, &mut t, club);
    }
}

/// Comparer for `write_manager_fa_complaint_news_text` (`compare_media_text_writer`, the case in slot 5).
pub(crate) unsafe extern "thiscall" fn compare_write_manager_fa_complaint_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I3%%);
    compare_media_text_writer("write_manager_fa_complaint_news_text", news, body, 5, out, || o(this, news, body, out), |t| write_manager_fa_complaint_news_text(this, news, body, t));
}
