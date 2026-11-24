// --- sx01_1: Media::write_team_media_story_news_text (0x74bbd0, media.cpp, 35,664 bytes), the whole routine in named pieces ---

/// The last piece `write_team_media_story_news_text` ran (for the comparer's message).
pub(crate) static TEAM_TEXT_PIECE: std::sync::Mutex<&'static str> = std::sync::Mutex::new("");
/// Set when the body used the story text the original never writes (kinds 0x50, 0x51 and 0x3d without a match).
pub(crate) static TEAM_TEXT_UNSET: AtomicBool = AtomicBool::new(false);

/// What `write_team_media_story_news_text` reads from the news item and prepares before writing (the original's frame).
pub(crate) struct TeamText {
    draw: i32,
    tone: i32,
    club_index: i32,
    confident: u32,
    cautious: u32,
    comp_index: i32,
    kind: i32,
    reply: i32,
    manager: i32,
    manager_ids: [u32; 3],
    person: i32,
    person_ids: [u32; 3],
    has_game: u32,
    record: [u8; 0x20],
    variant: i32,
    latest: u32,
    host_a: i32,
    host_b: i32,
    extra: u32,
    person_club: i32,
    person_job: i32,
    caps: u32,
    shootout: i32,
    pundit: i32,
    on_screen: bool,
    home: bool,
    home_england: bool,
    manager_name: [u8; 0x78],
    person_name: [u8; 0x78],
    person_short: [u8; 0x78],
    outlet: [u8; 0xc8],
    outlet_gender: u8,
    adjective: [u8; 0x78],
    score: [u8; 0x20],
    stage: [u8; 0xc8],
    money: [u8; 0x40],
    text: [u8; 0x1388],
    second: [u8; 0x400],
}

/// Writes a langlib template into the story text (`story_lang!` on `TeamText::text`).
macro_rules! team_text {
    ($s:expr, $tpl:expr $(, $a:expr)* $(,)?) => {{
        let t = $s.text.as_mut_ptr();
        story_lang!(t, $tpl $(, $a)*)
    }};
}

/// Writes a langlib template into the second paragraph (the reply, or a sentence appended to the story).
macro_rules! team_second {
    ($s:expr, $tpl:expr $(, $a:expr)* $(,)?) => {{
        let t = $s.second.as_mut_ptr();
        story_lang!(t, $tpl $(, $a)*)
    }};
}

impl TeamText {
    fn piece(&self, name: &'static str) {
        if let Ok(mut p) = TEAM_TEXT_PIECE.lock() {
            *p = name;
        }
    }
    unsafe fn club(&self) -> usize {
        club_at(self.club_index as isize).addr()
    }
    unsafe fn comp(&self) -> usize {
        read_usize(G_CLUB_COMP_TABLE).wrapping_add((self.comp_index as usize).wrapping_mul(NEWS_COMP_SIZE))
    }
    unsafe fn nation(&self) -> usize {
        read_usize(self.club() + club::NATION)
    }
    /// The club, competition or nation record's short name and its gender byte (the three share the layout).
    unsafe fn short(rec: usize) -> usize {
        rec + club::SHORT_NAME
    }
    unsafe fn short_gender(rec: usize) -> i32 {
        read_i8(rec + club::SHORT_GENDER_NAME) as i32
    }
    unsafe fn long(rec: usize) -> usize {
        rec + club::NAME
    }
    unsafe fn long_gender(rec: usize) -> i32 {
        read_i8(rec + club::GENDER_NAME) as i32
    }
    unsafe fn cs(&self) -> usize { Self::short(self.club()) }
    unsafe fn cg(&self) -> i32 { Self::short_gender(self.club()) }
    unsafe fn ks(&self) -> usize { Self::short(self.comp()) }
    unsafe fn kg(&self) -> i32 { Self::short_gender(self.comp()) }
    unsafe fn kn(&self) -> usize { Self::long(self.comp()) }
    unsafe fn kgl(&self) -> i32 { Self::long_gender(self.comp()) }
    unsafe fn ns(&self) -> usize { Self::short(self.nation()) }
    unsafe fn ng(&self) -> i32 { Self::short_gender(self.nation()) }
    /// The opponent of the story's match (the match record's opponent index).
    unsafe fn opponent(&self) -> usize {
        club_at(read_i32(self.record.as_ptr() as usize + club_match_record::OPPONENT_ID) as isize).addr()
    }
    unsafe fn on(&self) -> usize { Self::long(self.opponent()) }
    unsafe fn os(&self) -> usize { Self::short(self.opponent()) }
    unsafe fn og(&self) -> i32 { Self::short_gender(self.opponent()) }
    unsafe fn ogl(&self) -> i32 { Self::long_gender(self.opponent()) }
    /// The person's club (the club of his job, from the news).
    unsafe fn ps(&self) -> usize { Self::short(club_at(self.person_club as isize).addr()) }
    unsafe fn pg(&self) -> i32 { Self::short_gender(club_at(self.person_club as isize).addr()) }
    /// A host nation of the tournament (`G_NATION_TABLE` by index).
    unsafe fn host(&self, i: i32) -> usize {
        read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE))
    }
    unsafe fn as_(&self) -> usize { Self::short(self.host(self.host_a)) }
    unsafe fn ag(&self) -> i32 { Self::short_gender(self.host(self.host_a)) }
    unsafe fn bs(&self) -> usize { Self::short(self.host(self.host_b)) }
    unsafe fn bg(&self) -> i32 { Self::short_gender(self.host(self.host_b)) }
    /// The continent of the club's nation: its name (+4) and that name's gender byte (+0x1e).
    unsafe fn continent(&self) -> (usize, i32) {
        let c = read_usize(self.nation() + nation::CONTINENT);
        (c + 4, read_i8(c + 0x1e) as i32)
    }
    unsafe fn gf(&self) -> i32 {
        self.record[club_match_record::GOALS_FOR] as i8 as i32
    }
    unsafe fn ga(&self) -> i32 {
        self.record[club_match_record::GOALS_AGAINST] as i8 as i32
    }
    /// The score class of the story's match: a hammering (margin above 3), a thriller (more than 5 goals) or plain.
    unsafe fn score_class(&self) -> u8 {
        if self.gf() - self.ga() > 3 {
            0
        } else if self.gf() + self.ga() > 5 {
            1
        } else {
            2
        }
    }
    /// The score as "high-low" (`%d-%d` of the larger and the smaller goal count).
    unsafe fn score_high_low(&mut self) {
        let (a, b) = (self.gf(), self.ga());
        let (lo, hi) = (if a < b { a } else { b }, if a > b { a } else { b });
        let s = self.score.as_mut_ptr();
        story_crt!(s, 0x98c114, hi, lo); // "%d-%d"
    }
    /// The club's nationality adjective in form `form` (`nation_get_nationality_adjective`, 0x78).
    unsafe fn adjective(&mut self, form: u32) -> usize {
        exe::nation_get_nationality_adjective(self.nation(), self.adjective.as_mut_ptr() as usize, 0x78, form);
        self.adjective.as_ptr() as usize
    }
    /// The name and gender of the person's club job (`database_get_job_name`).
    unsafe fn job(&self) -> (usize, i32) {
        let mut g: i8 = 0;
        let n = exe::database_get_job_name(self.person_job as u32, 0, 0, &mut g);
        (n, g as i32)
    }
    /// The national team hosts the tournament (either host is the club's nation).
    unsafe fn hosting(&self) -> bool {
        let id = read_i32(self.nation());
        self.host_a == id || self.host_b == id
    }
    fn pname(&self) -> usize {
        self.person_name.as_ptr() as usize
    }
    fn pshort(&self) -> usize {
        self.person_short.as_ptr() as usize
    }
    fn mname(&self) -> usize {
        self.manager_name.as_ptr() as usize
    }
    fn outlet_name(&self) -> usize {
        self.outlet.as_ptr() as usize
    }
    fn om(&self) -> i32 {
        self.outlet_gender as i8 as i32
    }
    fn score_text(&self) -> usize {
        self.score.as_ptr() as usize
    }
}

/// Reads the news slots of a team media story (laid out by `post_team_media_story`): the draw, the tone, the club
/// and the competition, the kind, the manager's reply, the manager and the person with their name ids, the match
/// record from slot 0x14 (moving the index), then in order the variant, "last placed", the two hosts, the extra word,
/// the person's club and job, his caps, the shoot-out and the pundit (the reads go on in sequence even without a
/// match, as in the original). Slots 0, 3 and 8 are read or skipped as the original does.
/// Original: part of `Media::write_team_media_story_news_text` (0x74bbd0..0x74be48).
pub(crate) unsafe fn team_text_read(news: usize) -> TeamText {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut record = [0u8; 0x20];
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    slot(0);
    let draw = slot(1) as i32;
    let tone = slot(2) as i32;
    slot(3);
    let club_index = slot(4) as i32;
    let confident = slot(5);
    let cautious = slot(6);
    let comp_index = slot(7) as i32;
    let kind = slot(9) as i32;
    let reply = slot(0xa) as i32;
    let manager = slot(0xb) as i32;
    let manager_ids = [slot(0xc), slot(0xd), slot(0xe)];
    let person = slot(0xf) as i32;
    let person_ids = [slot(0x10), slot(0x11), slot(0x12)];
    let has_game = slot(0x13);
    let mut k: u8 = 0x14;
    if has_game != 0 {
        exe::read_match_record_from_news(news, &mut k, record.as_mut_ptr());
    }
    let mut next = || {
        let v = exe::newsitem_get_slot(news, k as i8 as i32 as u32);
        k = k.wrapping_add(1);
        v
    };
    let variant = next() as i32;
    let latest = next();
    let host_a = next() as i32;
    let host_b = next() as i32;
    let extra = next();
    let person_club = next() as i32;
    let person_job = next() as i32;
    let caps = next();
    let shootout = next() as i32;
    let pundit = next() as i32;
    TeamText {
        draw, tone, club_index, confident, cautious, comp_index, kind, reply, manager, manager_ids, person, person_ids,
        has_game, record, variant, latest, host_a, host_b, extra, person_club, person_job, caps, shootout, pundit,
        on_screen: false, home: false, home_england: false,
        manager_name: [0; 0x78], person_name: [0; 0x78], person_short: [0; 0x78], outlet: [0; 0xc8], outlet_gender: 0,
        adjective: [0; 0x78], score: [0; 0x20], stage: [0; 0xc8], money: [0; 0x40], text: [0; 0x1388], second: [0; 0x400],
    }
}

/// The names and the point of view: whether the manager of the story is the human on screen
/// (`G_SCREEN_RECORDS_MANAGER` of `G_SCREEN_MANAGER_CURRENT_SLOT`); the manager's name (style 1); the person's name
/// in styles 1 and 5, or "Dixie Dean" and "Dean" without a person; `home` when the screen manager's club job (or,
/// without one, his own nation) is of the club's nation, and `home_england` when that nation is England
/// (`KEY_NATION_ENGLAND`).
/// Original: part of `Media::write_team_media_story_news_text` (0x74be48..0x74c025).
pub(crate) unsafe fn team_text_names(s: &mut TeamText) {
    let screen = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    s.on_screen = staff_at(s.manager as isize).addr() == screen;
    let m = s.manager_ids;
    exe::database_format_name_from_ids(m[0], m[1], m[2], s.manager_name.as_mut_ptr(), 0x78, 1, s.manager as u32);
    if s.person > -1 {
        let p = s.person_ids;
        exe::database_format_name_from_ids(p[0], p[1], p[2], s.person_name.as_mut_ptr(), 0x78, 1, s.person as u32);
        exe::database_format_name_from_ids(p[0], p[1], p[2], s.person_short.as_mut_ptr(), 0x78, 5, s.person as u32);
    } else {
        copy_c_string(s.person_name.as_mut_ptr() as usize, 0xa53ad4); // "Dixie Dean"
        copy_c_string(s.person_short.as_mut_ptr() as usize, 0xa53acc); // "Dean"
    }
    if screen == 0 {
        return;
    }
    let screen = Staff(screen);
    let nation = if !screen.club_job().is_null() { screen.club_job().nation().addr() } else { screen.nation().addr() };
    if nation == s.nation() {
        s.home = true;
        s.home_england = read_u32(nation) == read_u32(KEY_NATION_ENGLAND);
    }
}

/// The outlet that runs the story: the pundit's name (`PunditList::get_pundit_name` of `G_PUNDIT_LIST`, with its
/// gender) when the club, the tone and the pundit are set and either the story is at home with the club having a
/// nation, or it is an English story about a national team whose kind and club index have the same parity; otherwise
/// by tone 1..6 and other: national or local newspaper, national or local radio station, magazine, sports paper,
/// website, plain at home or "{nationality} ..." abroad (adjective forms 0x16..0x1a); the magazine's gender is 1 with
/// language 8, the others 0.
/// Original: part of `Media::write_team_media_story_news_text` (0x74c025..0x74c342).
pub(crate) unsafe fn team_text_outlet(s: &mut TeamText) {
    if s.club_index != -1 && s.tone != -1 && s.pundit != -1 {
        let rec = s.club();
        let pundit = if read_usize(rec + club::NATION) != 0 && s.home {
            true
        } else if s.home_england {
            exe::database_is_national_team(rec) != 0 && s.kind % 2 == s.club_index % 2
        } else {
            false
        };
        if pundit {
            covered!("sx01_1 team text pundit");
            exe::punditlist_get_pundit_name(read_usize(G_PUNDIT_LIST), s.tone as u32, s.pundit as u32, s.outlet.as_mut_ptr() as usize, &mut s.outlet_gender);
            return;
        }
    }
    let out = s.outlet.as_mut_ptr();
    let (plain, form, foreign) = match s.tone {
        1 => (0xa242cc, 0x16, 0xa24294), // national newspaper
        2 => (0xa24284, 0x17, 0xa24250), // local newspaper
        3 => (0xa24238, 0x18, 0xa24208), // national radio station
        4 => (0xa241f4, 0x18, 0xa241bc), // local radio station
        5 => (0xa241b0, 0x19, 0xa24184), // magazine
        6 => (0xa24174, 0x1a, 0xa24144), // sports paper
        _ => (0xa2413c, 0x1a, 0xa24110), // website
    };
    if s.home {
        story_lang!(out, plain);
    } else {
        let adj = s.adjective(form);
        story_lang!(out, foreign, 0x64, adj);
    }
    s.outlet_gender = if s.tone == 5 { (exe::langlib_get_language() == 8) as u8 } else { 0 };
}

/// Kind 0x50: the club's unbeaten run has ended (needs the match; the variant is the run's length). By the draw: 0,
/// the club was beaten (inspired by the person when there is one); 1, "after going N games unbeaten" (with the
/// opponent's victory inspired by the person); other, the run ended at the hands of the opponent (with the person's
/// performance). Each by the score class (hammering, thriller, plain).
/// Original: part of `Media::write_team_media_story_news_text` (0x74c351..0x74ca63).
pub(crate) unsafe fn team_text_unbeaten_run_ended(s: &mut TeamText) {
    s.piece("unbeaten run ended (0x50)");
    covered!("sx01_1 team text unbeaten run ended");
    s.score_high_low();
    let (cg, cs, ogl, on, v, sc, pn) = (s.cg(), s.cs(), s.ogl(), s.on(), s.variant, s.score_text(), s.pname());
    let class = s.score_class();
    match s.draw {
        0 => {
            if s.person > -1 {
                let tpl = [0xa52e9c, 0xa52da4, 0xa52cc0][class as usize];
                team_text!(s, tpl, cg, cg, 0x64, ogl, ogl, cs, v, sc, pn, on);
            } else {
                let tpl = [0xa52c08, 0xa52b40, 0xa52a8c][class as usize];
                team_text!(s, tpl, cg, cg, ogl, ogl, cs, v, sc, on);
            }
        }
        1 => {
            if s.person > -1 {
                let tpl = [0xa53418, 0xa532dc, 0xa531ac][class as usize];
                let og = s.og();
                team_text!(s, tpl, cg, cg, ogl, ogl, og, og, v, cs, on, sc, on, pn);
            } else {
                let tpl = [0xa530f8, 0xa53038, 0xa52f84][class as usize];
                team_text!(s, tpl, cg, cg, ogl, ogl, v, cs, on, sc);
            }
        }
        _ => {
            if s.person > -1 {
                let tpl = [0xa539c8, 0xa538b0, 0xa537a0][class as usize];
                team_text!(s, tpl, cg, cg, ogl, ogl, cs, v, sc, on, pn);
            } else {
                let tpl = [0xa536d4, 0xa53608, 0xa53548][class as usize];
                team_text!(s, tpl, cg, cg, ogl, ogl, cs, v, sc, on);
            }
        }
    }
}

/// Kind 0x51: the club's run without a win has ended (needs the match). By the draw: 0, the run ended with the
/// victory (the man of the match looking forward); 1, the man of the match inspired the win; other, the club defeated
/// the opponent to halt the run (the person instrumental); each without the person when there is none.
/// Original: part of `Media::write_team_media_story_news_text` (0x74ca63..0x74ccde).
pub(crate) unsafe fn team_text_winless_run_ended(s: &mut TeamText) {
    s.piece("winless run ended (0x51)");
    covered!("sx01_1 team text winless run ended");
    s.score_high_low();
    let (cg, cs, ogl, on, os, v, sc, pn) = (s.cg(), s.cs(), s.ogl(), s.on(), s.os(), s.variant, s.score_text(), s.pname());
    let with = s.person > -1;
    match s.draw {
        0 if with => team_text!(s, 0xa5263c, cg, cg, ogl, ogl, cs, v, on, sc, pn),
        0 => team_text!(s, 0xa5258c, cg, cg, ogl, ogl, cs, v, on, sc),
        1 if with => team_text!(s, 0xa52814, cg, cg, ogl, ogl, pn, cs, sc, on, v),
        1 => team_text!(s, 0xa52764, cg, cg, ogl, ogl, cs, v, sc, on),
        _ if with => team_text!(s, 0xa52994, cg, cg, ogl, ogl, cs, os, sc, v, pn),
        _ => team_text!(s, 0xa528f0, cg, cg, ogl, ogl, cs, os, sc, v),
    }
}

/// The "club figure, former player or the media" paragraph many kinds share at draw 0: a person of a club with a job
/// (`expert`, his club, job name and name), a former player (`former`) or the fallback. Returns true when it wrote.
unsafe fn team_text_person(s: &mut TeamText, expert: impl FnOnce(&mut TeamText, usize, i32), former: impl FnOnce(&mut TeamText)) -> bool {
    if s.person <= -1 {
        return false;
    }
    if s.person_club > -1 && s.person_job > -1 {
        covered!("sx01_1 team text club figure");
        let (job, jg) = s.job();
        expert(s, job, jg);
        return true;
    }
    if s.caps != 0 {
        covered!("sx01_1 team text former player");
        former(s);
        return true;
    }
    false
}

/// Kind 0x3f: the club battles relegation. Draw 0: a club figure or former player predicts it will be hard to escape
/// relegation from the division, otherwise the media (at home) or the nationality's media; draw 1: desperate for points,
/// the media (or the nationality's media) say they need a miracle; other: a pundit (at home) or the nation's media say
/// their lack of quality makes them candidates for the drop.
/// Original: part of `Media::write_team_media_story_news_text` (0x74ccde..0x74d023).
pub(crate) unsafe fn team_text_relegation_battle(s: &mut TeamText) {
    s.piece("relegation battle (0x3f)");
    covered!("sx01_1 team text relegation battle");
    let (cg, cs, kg, ks) = (s.cg(), s.cs(), s.kg(), s.ks());
    match s.draw {
        0 => {
            if team_text_person(s, |s, job, jg| {
                let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                team_text!(s, 0xa520bc, pg, pg, jg, jg, cg, cg, kg, kg, ps, job, pn, cs, ks)
            }, |s| {
                let pn = s.pname();
                team_text!(s, 0xa51fe0, cg, cg, kg, kg, pn, cs, ks)
            }) {
                return;
            }
            if s.home {
                team_text!(s, 0xa51ef8, cg, cg, kg, kg, cs, ks);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa51df0, cg, cg, 0x64, 0x64, kg, kg, cs, adj, ks);
            }
        }
        1 => {
            if s.home {
                team_text!(s, 0xa522f4, cg, cg, kg, kg, cs, ks);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa521c0, cg, cg, kg, kg, 0x64, 0x64, cs, ks, adj);
            }
        }
        _ => {
            if s.home {
                team_text!(s, 0xa524d0, cg, cg, cs);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa52404, cg, cg, ng, ng, cs, ns);
            }
        }
    }
}

/// Kind 0x5f: the club pushes for promotion. Draw 0: a club figure or former player predicts promotion, otherwise
/// intense media speculation (or in the nation); draw 1: the media (or the nation's media) predict a successful bid;
/// other: the media (or the nationality's media) are convinced the good form continues to promotion.
/// Original: part of `Media::write_team_media_story_news_text` (0x74d023..0x74d34f).
pub(crate) unsafe fn team_text_promotion_push(s: &mut TeamText) {
    s.piece("promotion push (0x5f)");
    covered!("sx01_1 team text promotion push");
    let (cg, cs, kg, ks) = (s.cg(), s.cs(), s.kg(), s.ks());
    match s.draw {
        0 => {
            if team_text_person(s, |s, job, jg| {
                let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                team_text!(s, 0xa51988, pg, pg, jg, jg, cg, cg, kg, kg, ps, job, pn, cs, ks)
            }, |s| {
                let pn = s.pname();
                team_text!(s, 0xa518bc, cg, cg, kg, kg, pn, cs, ks)
            }) {
                return;
            }
            if s.home {
                team_text!(s, 0xa51804, cg, cg, cs);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa51730, ng, ng, cg, cg, ns, cs);
            }
        }
        1 => {
            if s.home {
                team_text!(s, 0xa51b50, cg, cg, kg, kg, cs, ks);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa51a78, ng, ng, cg, cg, kg, kg, ns, cs, ks);
            }
        }
        _ => {
            let (kgl, kn) = (s.kgl(), s.kn());
            if s.home {
                team_text!(s, 0xa51d10, cg, cg, kgl, kgl, cs, kn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa51c0c, 0x64, 0x64, cg, cg, kgl, kgl, adj, cs, kn);
            }
        }
    }
}

/// Kind 0x60: the club is climbing away from relegation. Draw 0: a club figure or former player predicts the escape,
/// otherwise intense media speculation (or in the nation); draw 1: the media (or the nation's media) predict a
/// successful escape; other: the media (or the nationality's media) are convinced the good form continues to safety.
/// Original: part of `Media::write_team_media_story_news_text` (0x74d34f..0x74d67e).
pub(crate) unsafe fn team_text_escaping_relegation(s: &mut TeamText) {
    s.piece("escaping relegation (0x60)");
    covered!("sx01_1 team text escaping relegation");
    let (cg, cs, kg, ks) = (s.cg(), s.cs(), s.kg(), s.ks());
    match s.draw {
        0 => {
            if team_text_person(s, |s, job, jg| {
                let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                team_text!(s, 0xa512a4, pg, pg, jg, jg, cg, cg, kg, kg, ps, job, pn, cs, ks)
            }, |s| {
                let pn = s.pname();
                team_text!(s, 0xa511c4, cg, cg, kg, kg, pn, cs, ks)
            }) {
                return;
            }
            if s.home {
                team_text!(s, 0xa51128, cg, cg, cs);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa51070, ng, ng, cg, cg, ns, cs);
            }
        }
        1 => {
            if s.home {
                team_text!(s, 0xa51484, cg, cg, kg, kg, cs, ks);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa513a8, ng, ng, cg, cg, kg, kg, ns, cs, ks);
            }
        }
        _ => {
            let (kgl, kn) = (s.kgl(), s.kn());
            if s.home {
                team_text!(s, 0xa5164c, cg, cg, kgl, kgl, cs, kn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa51544, 0x64, 0x64, cg, cg, kgl, kgl, adj, cs, kn);
            }
        }
    }
}

/// Kind 0x42: the club is not considered a contender (the reigning champions when "last placed" is set). Draw 0: a
/// club figure doubts their ability to retain or win the title, a former player writes off their chances, otherwise
/// several national (or nationality's) newspapers write them off; other draws: sections of the media (or the
/// nationality's media) do not consider them contenders.
/// Original: part of `Media::write_team_media_story_news_text` (0x74d67e..0x74db17).
pub(crate) unsafe fn team_text_not_contenders(s: &mut TeamText) {
    s.piece("not contenders (0x42)");
    covered!("sx01_1 team text not contenders");
    let (cg, cs, kg, ks, kgl, kn) = (s.cg(), s.cs(), s.kg(), s.ks(), s.kgl(), s.kn());
    let latest = s.latest != 0;
    if s.draw != 0 {
        if s.home {
            team_text!(s, if latest { 0xa50fac } else { 0xa50efc }, cg, cg, kgl, kgl, cs, kn);
        } else {
            let adj = s.adjective(0x1c);
            team_text!(s, if latest { 0xa50e18 } else { 0xa50d44 }, cg, cg, kgl, kgl, 0x64, 0x64, cs, kn, adj);
        }
        return;
    }
    if team_text_person(s, |s, job, jg| {
        let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
        team_text!(s, if latest { 0xa50c58 } else { 0xa50b70 }, pg, pg, jg, jg, cg, cg, kg, kg, ps, job, pn, cs, ks)
    }, |s| {
        let pn = s.pname();
        team_text!(s, 0xa50ab8, cg, cg, kg, kg, pn, cs, ks)
    }) {
        return;
    }
    if s.home {
        team_text!(s, if latest { 0xa50a0c } else { 0xa5095c }, cg, cg, kgl, kgl, cs, kn);
    } else {
        let adj = s.adjective(0x1b);
        team_text!(s, if latest { 0xa50898 } else { 0xa507cc }, 0x64, 0x64, cg, cg, kgl, kgl, adj, cs, kn);
    }
}

/// Kind 0x43: the club is called boring. Draw 0: a club figure calls them one of the least adventurous sides, a former
/// player the most boring, otherwise the media (or the nationality's media) write them off as a boring mid-table side
/// under the manager; draw 1: their performances written off as mediocre by the press (or the nationality's press);
/// other: a former manager of the club labels them dull (in the nation, abroad; the adjective is fetched there too).
/// Original: part of `Media::write_team_media_story_news_text` (0x74db17..0x74de5b).
pub(crate) unsafe fn team_text_boring(s: &mut TeamText) {
    s.piece("boring (0x43)");
    covered!("sx01_1 team text boring");
    let (cg, cs, kg, ks) = (s.cg(), s.cs(), s.kg(), s.ks());
    match s.draw {
        0 => {
            if team_text_person(s, |s, job, jg| {
                let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                team_text!(s, 0xa50330, pg, pg, jg, jg, cg, cg, kg, kg, ps, job, pn, cs, ks)
            }, |s| {
                let pn = s.pname();
                team_text!(s, 0xa50268, cg, cg, kg, kg, pn, cs, ks)
            }) {
                return;
            }
            let mn = s.mname();
            if s.home {
                team_text!(s, 0xa501c8, cg, cg, 0x64, cs, mn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa50108, cg, cg, 0x64, 0x64, 0x64, cs, adj, mn);
            }
        }
        1 => {
            let (kgl, kn) = (s.kgl(), s.kn());
            if s.home {
                team_text!(s, 0xa50528, cg, cg, kgl, kgl, cs, kn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa50430, cg, cg, 0x64, 0x64, kgl, kgl, cs, adj, kn);
            }
        }
        _ => {
            if s.home {
                team_text!(s, 0xa506f4, cg, cg, cs);
            } else {
                s.adjective(0x1c); // fetched but not used by the text
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa50600, ng, ng, cg, cg, ns, cs);
            }
        }
    }
}

/// Kind 0x44: the national team is not a contender for the tournament (the holders when "last placed" is set), with
/// the hosts: the club's nation hosting ("hosts", "on their own soil"), two hosts (`host_b` set) or one. Draw 0: a
/// club figure or a former international writes off their chances of retaining (holders) or winning/challenging,
/// otherwise several national newspapers; other draws: the media do not consider them contenders.
/// Original: part of `Media::write_team_media_story_news_text` (0x74de5b..0x74ecc3).
pub(crate) unsafe fn team_text_tournament_outsiders(s: &mut TeamText) {
    s.piece("tournament outsiders (0x44)");
    covered!("sx01_1 team text tournament outsiders");
    let (cg, cs, kgl, kn) = (s.cg(), s.cs(), s.kgl(), s.kn());
    let latest = s.latest != 0;
    let hosting = s.hosting();
    let two = s.host_b > -1;
    if s.draw != 0 {
        if hosting {
            team_text!(s, if latest { 0xa4fe70 } else { 0xa4fc1c }, cg, cg, kgl, kgl, cs, kn);
        } else if two {
            let (ag, as_, bg, bs) = (s.ag(), s.as_(), s.bg(), s.bs());
            team_text!(s, if latest { 0xa50004 } else { 0xa4fd80 }, cg, cg, kgl, kgl, ag, ag, bg, bg, cs, kn, as_, bs);
        } else {
            let (ag, as_) = (s.ag(), s.as_());
            team_text!(s, if latest { 0xa4ff2c } else { 0xa4fcc0 }, cg, cg, kgl, kgl, ag, ag, cs, kn, as_);
        }
        return;
    }
    // draw 0: (club figure, former international, newspapers) by retaining/winning and the hosts
    let (expert, former, papers) = match (latest, hosting, two) {
        (true, true, _) => (0xa4f5b4, 0xa4f4f4, 0xa4f44c),
        (true, false, true) => (0xa4fb00, 0xa4fa00, 0xa4f918),
        (true, false, false) => (0xa4f828, 0xa4f750, 0xa4f690),
        (false, true, _) => (0xa4edd8, 0xa4ed18, 0xa4ec68),
        (false, false, true) => (0xa4f334, 0xa4f234, 0xa4f148),
        (false, false, false) => (0xa4f050, 0xa4ef7c, 0xa4eeb8),
    };
    if hosting {
        if team_text_person(s, |s, job, jg| {
            let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
            team_text!(s, expert, pg, pg, jg, jg, cg, cg, kgl, kgl, ps, job, pn, cs, kn)
        }, |s| {
            let pn = s.pname();
            team_text!(s, former, cg, cg, kgl, kgl, pn, cs, kn)
        }) {
            return;
        }
        team_text!(s, papers, cg, cg, kgl, kgl, cs, kn);
    } else if two {
        let (ag, as_, bg, bs) = (s.ag(), s.as_(), s.bg(), s.bs());
        if team_text_person(s, |s, job, jg| {
            let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
            team_text!(s, expert, pg, pg, jg, jg, cg, cg, kgl, kgl, ag, ag, bg, bg, ps, job, pn, cs, kn, as_, bs)
        }, |s| {
            let pn = s.pname();
            team_text!(s, former, cg, cg, kgl, kgl, ag, ag, bg, bg, pn, cs, kn, as_, bs)
        }) {
            return;
        }
        team_text!(s, papers, cg, cg, kgl, kgl, ag, ag, bg, bg, cs, kn, as_, bs);
    } else {
        let (ag, as_) = (s.ag(), s.as_());
        if team_text_person(s, |s, job, jg| {
            let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
            team_text!(s, expert, pg, pg, jg, jg, cg, cg, kgl, kgl, ag, ag, ps, job, pn, cs, kn, as_)
        }, |s| {
            let pn = s.pname();
            team_text!(s, former, cg, cg, kgl, kgl, ag, ag, pn, cs, kn, as_)
        }) {
            return;
        }
        team_text!(s, papers, cg, cg, kgl, kgl, ag, ag, cs, kn, as_);
    }
}

/// Kind 0x5a: the national team is tipped for the tournament. Other draws: the campaign is set to end in glory
/// according to the outlet (holders: "again", the hosts "a combination of home advantage"; otherwise by the hosts:
/// the club's nation as host, two hosts or one); draw 1: touted as likely (holders, by the hosts) or potential winners;
/// draw 0: a club figure, a former international or the media claim an excellent chance of retaining (holders) or
/// winning, by the hosts.
/// Original: part of `Media::write_team_media_story_news_text` (0x74ecc3..0x74fd3a).
pub(crate) unsafe fn team_text_tournament_favourites(s: &mut TeamText) {
    s.piece("tournament favourites (0x5a)");
    covered!("sx01_1 team text tournament favourites");
    let (cg, cs, kg, ks, kgl, kn) = (s.cg(), s.cs(), s.kg(), s.ks(), s.kgl(), s.kn());
    let latest = s.latest != 0;
    let hosting = s.hosting();
    let two = s.host_b > -1;
    let (om, outlet) = (s.om(), s.outlet_name());
    match s.draw {
        0 => {
            // (club figure, former international, media) and whether the competition is named in full
            let (expert, former, media) = match (latest, hosting, two) {
                (true, true, _) => (0xa4da4c, 0xa4d984, 0xa4d8e0),
                (true, false, true) => (0xa4dff4, 0xa4dedc, 0xa4ddec),
                (true, false, false) => (0xa4dce4, 0xa4dbf4, 0xa4db2c),
                (false, true, _) => (0xa4d214, 0xa4d150, 0xa4d0ac),
                (false, false, true) => (0xa4d7b0, 0xa4d698, 0xa4d5ac),
                (false, false, false) => (0xa4d4a4, 0xa4d3b8, 0xa4d2f4),
            };
            if hosting {
                if team_text_person(s, |s, job, jg| {
                    let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                    team_text!(s, expert, pg, pg, jg, jg, cg, cg, kgl, kgl, ps, job, pn, cs, kn)
                }, |s| {
                    let pn = s.pname();
                    team_text!(s, former, cg, cg, kgl, kgl, pn, cs, kn)
                }) {
                    return;
                }
                team_text!(s, media, cg, cg, kg, kg, cs, ks);
            } else if two {
                let (ag, as_, bg, bs) = (s.ag(), s.as_(), s.bg(), s.bs());
                if team_text_person(s, |s, job, jg| {
                    let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                    team_text!(s, expert, pg, pg, jg, jg, cg, cg, kg, kg, ag, ag, bg, bg, ps, job, pn, cs, ks, as_, bs)
                }, |s| {
                    let pn = s.pname();
                    team_text!(s, former, cg, cg, kg, kg, ag, ag, bg, bg, pn, cs, ks, as_, bs)
                }) {
                    return;
                }
                team_text!(s, media, cg, cg, kg, kg, ag, ag, bg, bg, cs, ks, as_, bs);
            } else {
                let (ag, as_) = (s.ag(), s.as_());
                if team_text_person(s, |s, job, jg| {
                    let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                    team_text!(s, expert, pg, pg, jg, jg, cg, cg, kg, kg, ag, ag, ps, job, pn, cs, ks, as_)
                }, |s| {
                    let pn = s.pname();
                    team_text!(s, former, cg, cg, kg, kg, ag, ag, pn, cs, ks, as_)
                }) {
                    return;
                }
                team_text!(s, media, cg, cg, kg, kg, ag, ag, cs, ks, as_);
            }
        }
        1 => {
            if !latest {
                team_text!(s, if hosting { 0xa4e128 } else { 0xa4e1b8 }, cg, cg, kg, kg, cs, ks);
            } else if hosting {
                team_text!(s, 0xa4e23c, cg, cg, kg, kg, cg, cg, cs, ks, cs);
            } else if two {
                let (ag, as_, bg, bs) = (s.ag(), s.as_(), s.bg(), s.bs());
                team_text!(s, 0xa4e440, cg, cg, kg, kg, cg, cg, ag, ag, bg, bg, cs, ks, cs, as_, bs);
            } else {
                let (ag, as_) = (s.ag(), s.as_());
                team_text!(s, 0xa4e32c, cg, cg, kg, kg, cg, cg, ag, ag, cs, ks, cs, as_);
            }
        }
        _ => {
            if latest {
                team_text!(s, if hosting { 0xa4e9bc } else { 0xa4eb10 }, cg, cg, kgl, kgl, om, om, cg, cg, cs, kn, outlet, cs);
            } else if hosting {
                team_text!(s, 0xa4e57c, cg, cg, kgl, kgl, om, om, cs, kn, outlet);
            } else if two {
                let (ag, as_, bg, bs) = (s.ag(), s.as_(), s.bg(), s.bs());
                team_text!(s, 0xa4e824, cg, cg, kgl, kgl, ag, ag, bg, bg, om, om, cg, cg, cs, kn, as_, bs, outlet, cs);
            } else {
                let (ag, as_) = (s.ag(), s.as_());
                team_text!(s, 0xa4e6b4, cg, cg, kgl, kgl, ag, ag, om, om, cg, cg, cs, kn, as_, outlet, cs);
            }
        }
    }
}

/// Kind 0x5c: the club is tipped for its continental competition. Other draws: the outlet says the campaign will end
/// in glory and bring the trophy back to the nation ("yet again" for the holders); draw 1: touted as likely (holders)
/// or potential winners by the media (or the nationality's media); draw 0: a club figure, a former player or the
/// media (at home, or in the nation) say they are equipped to retain or win it. Then, with the match won, a sentence
/// on it is appended (`team_text_continental_match`).
/// Original: part of `Media::write_team_media_story_news_text` (0x74fd3a..0x750377).
pub(crate) unsafe fn team_text_continental_favourites(s: &mut TeamText) {
    s.piece("continental favourites (0x5c)");
    covered!("sx01_1 team text continental favourites");
    let (cg, cs, kg, ks) = (s.cg(), s.cs(), s.kg(), s.ks());
    let latest = s.latest != 0;
    match s.draw {
        0 => {
            if !team_text_person(s, |s, job, jg| {
                let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                team_text!(s, if latest { 0xa4ca14 } else { 0xa4c940 }, pg, pg, jg, jg, cg, cg, kg, kg, ps, job, pn, cs, ks)
            }, |s| {
                let pn = s.pname();
                team_text!(s, if latest { 0xa4c884 } else { 0xa4c7cc }, cg, cg, kg, kg, pn, cs, ks)
            }) {
                if s.home {
                    team_text!(s, if latest { 0xa4c730 } else { 0xa4c694 }, cg, cg, kg, kg, cs, ks);
                } else {
                    let (ng, ns) = (s.ng(), s.ns());
                    team_text!(s, if latest { 0xa4c5dc } else { 0xa4c524 }, ng, ng, cg, cg, kg, kg, ns, cs, ks);
                }
            }
        }
        1 => {
            if s.home {
                team_text!(s, if latest { 0xa4cd04 } else { 0xa4cc80 }, cg, cg, kg, kg, cs, ks);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, if latest { 0xa4cb94 } else { 0xa4caec }, cg, cg, kg, kg, 0x64, 0x64, cs, ks, adj);
            }
        }
        _ => {
            let (kgl, kn, ng, ns, om, outlet) = (s.kgl(), s.kn(), s.ng(), s.ns(), s.om(), s.outlet_name());
            team_text!(s, if latest { 0xa4cf38 } else { 0xa4cdd0 }, cg, cg, kgl, kgl, om, om, cg, cg, ng, ng, cs, kn, outlet, cs, ns);
        }
    }
    team_text_continental_match(s);
}

/// After a kind 0x5c story with a match the club won: the score as "for-against" and, appended to the story, the
/// person's comments came after the win (a person at draw 0), or by (club + draw) % 3: 0, the win in the competition
/// added weight to the claims; 1, the accuracy of the reports was confirmed; other, the win inspired such confidence;
/// each by the score class.
/// Original: part of `Media::write_team_media_story_news_text` (0x750377..0x7506f7).
pub(crate) unsafe fn team_text_continental_match(s: &mut TeamText) {
    if s.has_game == 0 || s.gf() <= s.ga() {
        return;
    }
    covered!("sx01_1 team text continental match");
    let sc = s.score.as_mut_ptr();
    story_crt!(sc, 0x98c114, s.gf(), s.ga()); // "%d-%d"
    let (ogl, on, sc, class) = (s.ogl(), s.on(), s.score_text(), s.score_class() as usize);
    if s.person > -1 && s.draw == 0 {
        let ps = s.pshort();
        team_second!(s, [0xa4c4a0, 0xa4c404, 0xa4c380][class], 0x64, ogl, ogl, ps, on, sc);
    } else {
        match (s.club_index.wrapping_add(s.draw)) % 3 {
            0 => team_second!(s, [0xa4bfd4, 0xa4bf3c, 0xa4beb0][class], ogl, ogl, sc, on),
            1 => team_second!(s, [0xa4c184, 0xa4c0ec, 0xa4c06c][class], ogl, ogl, on, sc),
            _ => team_second!(s, [0xa4c2fc, 0xa4c27c, 0xa4c204][class], ogl, ogl, sc, on),
        }
    }
    append_c_string(s.text.as_mut_ptr() as usize, s.second.as_ptr() as usize);
}

/// Kind 0x5b: the club spent big (the extra word is the amount, `Cash::get_display_amount` and `cash_format_money`).
/// Draw 0: after spending (on players such as the person) they are expected to retain (holders) or challenge for the
/// title; draw 1: the supporters expect immediate success (the manager bringing in players such as the person);
/// other: the media (or the nation's media; the adjective is fetched but not used) expect them to challenge on all
/// fronts.
/// Original: part of `Media::write_team_media_story_news_text` (0x7506f7..0x750ad1).
pub(crate) unsafe fn team_text_big_spenders(s: &mut TeamText) {
    s.piece("big spenders (0x5b)");
    covered!("sx01_1 team text big spenders");
    let cash = s.extra;
    let amount = exe::cash_get_display_amount(&cash, 6);
    exe::cash_format_money(s.money.as_mut_ptr() as usize, 0x32, amount, 6);
    let (cg, cs, money, pn) = (s.cg(), s.cs(), s.money.as_ptr() as usize, s.pname());
    let with = s.person > -1;
    match s.draw {
        0 => {
            let (kgl, kn) = (s.kgl(), s.kn());
            let latest = s.latest != 0;
            if with {
                team_text!(s, if latest { 0xa4b8d0 } else { 0xa4b730 }, cg, cg, kgl, kgl, money, pn, cs, kn);
            } else {
                team_text!(s, if latest { 0xa4b818 } else { 0xa4b674 }, cg, cg, kgl, kgl, money, cs, kn);
            }
        }
        1 => {
            let mn = s.mname();
            if with {
                team_text!(s, 0xa4ba4c, cg, cg, cs, mn, money, pn);
            } else {
                team_text!(s, 0xa4b9b8, cg, cg, cs, money);
            }
        }
        _ => {
            if s.home {
                if with {
                    team_text!(s, 0xa4bdd4, cg, cg, cs, money, pn);
                } else {
                    team_text!(s, 0xa4bd1c, cg, cg, cs, money);
                }
            } else {
                s.adjective(0x1c); // fetched but not used by the text
                let (ng, ns) = (s.ng(), s.ns());
                if with {
                    team_text!(s, 0xa4bc20, ng, ng, cg, cg, ns, cs, money, pn);
                } else {
                    team_text!(s, 0xa4bb4c, ng, ng, cg, cg, ns, cs, money);
                }
            }
        }
    }
}

/// Kind 0x52: the striker (the person) is starved of service. Draw 0: the club accused by the media (or the
/// nationality's media) of not creating chances for him; draw 1: a media pundit (or one in the nation) accuses the
/// midfield; other: a (nationality's) newspaper article says he will struggle for goals.
/// Original: part of `Media::write_team_media_story_news_text` (0x750ad1..0x750cb5).
pub(crate) unsafe fn team_text_striker_starved(s: &mut TeamText) {
    s.piece("striker starved (0x52)");
    covered!("sx01_1 team text striker starved");
    let (cg, cs, pn, psh) = (s.cg(), s.cs(), s.pname(), s.pshort());
    match s.draw {
        0 => {
            if s.home {
                team_text!(s, 0xa4b068, cg, cg, cs, pn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa4af98, cg, cg, 0x64, 0x64, cs, adj, pn);
            }
        }
        1 => {
            if s.home {
                team_text!(s, 0xa4b254, cg, cg, cs, pn, psh);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa4b114, ng, ng, cg, cg, ns, cs, pn, psh);
            }
        }
        _ => {
            if s.home {
                team_text!(s, 0xa4b508, cg, cg, pn, cs, psh);
            } else {
                let adj = s.adjective(0x1e);
                team_text!(s, 0xa4b378, cg, cg, 0x64, 0x64, pn, cs, adj, psh);
            }
        }
    }
}

/// Kind 0x3e: the club's slump continues. Draw 0: deep in the relegation mire, the media (or the nationality's media)
/// doubt they stay up; draw 1: a (nationality's) media pundit says it will take a miracle; other: a pundit says they
/// play as if relegation were a foregone conclusion.
/// Original: part of `Media::write_team_media_story_news_text` (0x750cb5..0x750ec2).
pub(crate) unsafe fn team_text_relegation_slump(s: &mut TeamText) {
    s.piece("relegation slump (0x3e)");
    covered!("sx01_1 team text relegation slump");
    let (cg, cs, kg, ks) = (s.cg(), s.cs(), s.kg(), s.ks());
    match s.draw {
        0 => {
            if s.home {
                team_text!(s, 0xa4abc4, cg, cg, kg, kg, cs, ks);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa4aa9c, cg, cg, kg, kg, 0x64, 0x64, cs, ks, adj);
            }
        }
        1 => {
            if s.home {
                team_text!(s, 0xa4adc8, cg, cg, kg, kg, cs, ks);
            } else {
                let adj = s.adjective(0x1f);
                team_text!(s, 0xa4accc, 0x64, 0x64, cg, cg, kg, kg, adj, cs, ks);
            }
        }
        _ => team_text!(s, 0xa4aea0, cg, cg, kg, kg, cs, ks),
    }
}

/// Kind 0x5e: the club keeps on winning. Draw 0: the press (or the nationality's press) run out of superlatives (for
/// the champions when "last placed"); draw 1: the supporters hope for the title or honours, phone-ins praise the
/// manager; draw 2: respected pundits (or the nation's pundits) predict a season of glory; other: the media (or the
/// nationality's media) say nothing can stop them in their bid for another (or a) successful season.
/// Original: part of `Media::write_team_media_story_news_text` (0x750ec2..0x7512a2).
pub(crate) unsafe fn team_text_winning_ways(s: &mut TeamText) {
    s.piece("winning ways (0x5e)");
    covered!("sx01_1 team text winning ways");
    let (cg, cs, kgl, kn) = (s.cg(), s.cs(), s.kgl(), s.kn());
    let latest = s.latest != 0;
    match s.draw {
        0 => {
            if s.home {
                team_text!(s, if latest { 0xa4a2d0 } else { 0xa4a20c }, cg, cg, kgl, kgl, cs, kn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, if latest { 0xa4a114 } else { 0xa4a02c }, 0x64, 0x64, cg, cg, kgl, kgl, adj, cs, kn);
            }
        }
        1 => {
            let mn = s.mname();
            team_text!(s, if latest { 0xa4a4ec } else { 0xa4a3ac }, cg, cg, kgl, kgl, 0x64, cs, kn, mn);
        }
        2 => {
            if s.home {
                team_text!(s, 0xa4a714, cg, cg, kgl, kgl, cs, kn);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa4a638, cg, cg, kgl, kgl, ng, ng, cs, kn, ns);
            }
        }
        _ => {
            if s.home {
                team_text!(s, if latest { 0xa4a9fc } else { 0xa4a960 }, cg, cg, cs);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, if latest { 0xa4a898 } else { 0xa4a7dc }, cg, cg, 0x64, 0x64, cs, adj);
            }
        }
    }
}

/// Kind 0x3c: the supporters are losing patience. Draw 0: murmurs of discontent on the terraces; draw 1: a supporters'
/// club spokesman says the fans are losing patience; other: an opinion poll in a local (or a nationality's) paper.
/// Original: part of `Media::write_team_media_story_news_text` (0x7512a2..0x7513a7).
pub(crate) unsafe fn team_text_losing_patience(s: &mut TeamText) {
    s.piece("losing patience (0x3c)");
    covered!("sx01_1 team text losing patience");
    let (cg, cs) = (s.cg(), s.cs());
    match s.draw {
        0 => team_text!(s, 0xa49c44, cg, cg, cs),
        1 => team_text!(s, 0xa49cec, cg, cg, cs),
        _ => {
            if s.home {
                team_text!(s, 0xa49f30, cg, cg, cs);
            } else {
                let adj = s.adjective(0x17);
                team_text!(s, 0xa49e14, cg, cg, 0x64, 0x64, cs, adj);
            }
        }
    }
}

/// Kind 0x3d: the supporters' fury at the match (needs the match; the score high-low). With a round code in the match
/// record (+0xc or +0xe), the round's label (`comp_util_format_comp_stage_round_label` of the record's competition,
/// 0xc8; "Friendly" and "Reserve" get a lower-case first letter) and its gender (`comp_util_get_label_gender`) go into
/// the sentence. By the draw: 0, boos and jeers towards the end; 1, the outlet reported an angry demonstration; 2,
/// furious fans according to the outlet; other, the supporters vented their feelings; each for a draw or a defeat.
/// After a draw, the shoot-out sentence is appended (`team_text_shootout`).
/// Original: part of `Media::write_team_media_story_news_text` (0x7513a7..0x751ce8).
pub(crate) unsafe fn team_text_supporters_fury(s: &mut TeamText) {
    s.piece("supporters fury (0x3d)");
    covered!("sx01_1 team text supporters fury");
    s.score_high_low();
    let rec = s.record.as_ptr() as usize;
    let (cg, cs, ogl, on, sc, om, outlet) = (s.cg(), s.cs(), s.ogl(), s.on(), s.score_text(), s.om(), s.outlet_name());
    let drawn = s.gf() == s.ga();
    if read_i16(rec + club_match_record::ROUND_PREFIX) != 0 || read_i16(rec + club_match_record::ROUND_NAME) != 0 {
        covered!("sx01_1 team text supporters fury round");
        // the record's +8 indexes the competition table here
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(rec + club_match_record::FIXTURE_ID) as usize).wrapping_mul(NEWS_COMP_SIZE));
        let (r1, r2) = (read_u32(rec + club_match_record::ROUND_PREFIX), read_u32(rec + club_match_record::ROUND_NAME));
        exe::comp_util_format_comp_stage_round_label(comp, r1, r2, 1, s.stage.as_mut_ptr(), 0xc8);
        let st = core::ffi::CStr::from_ptr(s.stage.as_ptr() as *const core::ffi::c_char).to_bytes();
        if st == b"Friendly" {
            s.stage[0] = b'f';
        } else if st == b"Reserve" {
            s.stage[0] = b'r';
        }
        let sg = exe::comp_util_get_label_gender(0, r1, r2, 1) as i8 as i32;
        let stage = s.stage.as_ptr() as usize;
        match s.draw {
            0 if drawn => team_text!(s, 0xa48ec4, 0x64, sg, sg, ogl, ogl, cg, cg, sc, stage, on, cs),
            0 => return team_text!(s, 0xa48dd4, 0x64, sg, sg, ogl, ogl, cg, cg, sc, stage, on, cs),
            1 if drawn => team_text!(s, 0xa490bc, om, om, cg, cg, sg, sg, ogl, ogl, outlet, cs, sc, stage, on),
            1 => return team_text!(s, 0xa48fa8, om, om, cg, cg, sg, sg, ogl, ogl, outlet, cs, sc, stage, on),
            2 if drawn => team_text!(s, 0xa492cc, cg, cg, sg, sg, ogl, ogl, om, om, cs, sc, stage, on, outlet),
            2 => return team_text!(s, 0xa491c4, cg, cg, sg, sg, ogl, ogl, om, om, cs, sc, stage, on, outlet),
            _ if drawn => team_text!(s, 0xa494b4, cg, cg, sg, sg, ogl, ogl, cs, sc, stage, on),
            _ => return team_text!(s, 0xa493c8, cg, cg, sg, sg, ogl, ogl, cs, sc, stage, on),
        }
    } else {
        match s.draw {
            0 if drawn => team_text!(s, 0xa49664, 0x64, ogl, ogl, cg, cg, sc, on, cs),
            0 => return team_text!(s, 0xa49594, 0x64, ogl, ogl, cg, cg, sc, on, cs),
            1 if drawn => team_text!(s, 0xa4981c, om, om, cg, cg, ogl, ogl, outlet, cs, sc, on),
            1 => return team_text!(s, 0xa49728, om, om, cg, cg, ogl, ogl, outlet, cs, sc, on),
            2 if drawn => team_text!(s, 0xa499e0, cg, cg, ogl, ogl, om, om, cs, sc, on, outlet),
            2 => return team_text!(s, 0xa49904, cg, cg, ogl, ogl, om, om, cs, sc, on, outlet),
            _ if drawn => team_text!(s, 0xa49b80, cg, cg, ogl, ogl, cs, sc, on),
            _ => return team_text!(s, 0xa49abc, cg, cg, ogl, ogl, cs, sc, on),
        }
    }
    team_text_shootout(s);
}

/// After a drawn kind 0x3d match: the shoot-out sentence appended to the story (1 won after a penalty shoot-out, 2 lost
/// on penalties, 3 won after a shoot-out, 4 lost on a shoot-out; nothing otherwise).
/// Original: part of `Media::write_team_media_story_news_text` (0x75148a..0x751c7c).
pub(crate) unsafe fn team_text_shootout(s: &mut TeamText) {
    let tpl = match s.shootout {
        1 => 0xa21e0c,
        2 => 0xa21684,
        3 => 0xa21dc0,
        4 => 0xa21d74,
        _ => return,
    };
    covered!("sx01_1 team text shootout");
    let (cg, cs) = (s.cg(), s.cs());
    team_second!(s, tpl, cg, cg, cs);
    append_c_string(s.text.as_mut_ptr() as usize, s.second.as_ptr() as usize);
}

/// Kind 0x63: the club's title credentials. Draw 0: intense media speculation (or in the nation) whether they can
/// keep their form and retain the division (holders) or mount a realistic title challenge; draw 1: the newspapers
/// want to know whether you (the manager on screen), the manager, or (abroad) the manager believes the club can live
/// up to the tag; other: media opinion is divided (or in the nation) whether the holders (or the club) can mount a
/// serious challenge.
/// Original: part of `Media::write_team_media_story_news_text` (0x751ce8..0x752015).
pub(crate) unsafe fn team_text_title_credentials(s: &mut TeamText) {
    s.piece("title credentials (0x63)");
    covered!("sx01_1 team text title credentials");
    let (cg, cs) = (s.cg(), s.cs());
    let latest = s.latest != 0;
    match s.draw {
        0 => {
            let (kgl, kn) = (s.kgl(), s.kn());
            if s.home {
                if latest {
                    team_text!(s, 0xa48804, cg, cg, kgl, kgl, cs, kn);
                } else {
                    team_text!(s, 0xa48768, cg, cg, cs);
                }
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                if latest {
                    team_text!(s, 0xa48688, ng, ng, cg, cg, kgl, kgl, ns, cs, kn);
                } else {
                    team_text!(s, 0xa485d0, ng, ng, cg, cg, ns, cs);
                }
            }
        }
        1 => {
            let mn = s.mname();
            if s.on_screen {
                team_text!(s, 0xa48a84, cg, cg, cs);
            } else if s.home {
                team_text!(s, 0xa489b8, cg, cg, cs, mn);
            } else {
                let adj = s.adjective(0x1b);
                team_text!(s, 0xa488c8, 0x64, 0x64, cg, cg, adj, cs, mn);
            }
        }
        _ => {
            if s.home {
                team_text!(s, if latest { 0xa48d30 } else { 0xa48c94 }, cg, cg, cs);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, if latest { 0xa48bd4 } else { 0xa48b1c }, ng, ng, cg, cg, ns, cs);
            }
        }
    }
}

/// Kind 0x61: the club as title outsiders. Draw 0: intense media speculation (or in the nation) about an unlikely
/// challenge; draw 1: the newspapers want to know whether you, the manager, or (abroad) the manager believes the
/// side has what it takes; other: the media (or the nationality's media) are dubious.
/// Original: part of `Media::write_team_media_story_news_text` (0x752015..0x752296).
pub(crate) unsafe fn team_text_title_outsiders(s: &mut TeamText) {
    s.piece("title outsiders (0x61)");
    covered!("sx01_1 team text title outsiders");
    let (cg, cs) = (s.cg(), s.cs());
    match s.draw {
        0 => {
            if s.home {
                team_text!(s, 0xa480ac, cg, cg, cs);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa47ff4, ng, ng, cg, cg, ns, cs);
            }
        }
        1 => {
            let mn = s.mname();
            if s.on_screen {
                team_text!(s, 0xa483bc, cg, cg, cs);
            } else if s.home {
                team_text!(s, 0xa482ec, cg, cg, cs, mn);
            } else {
                let adj = s.adjective(0x1b);
                team_text!(s, 0xa48148, 0x64, 0x64, cg, cg, adj, cs, mn);
            }
        }
        _ => {
            if s.home {
                team_text!(s, 0xa48528, cg, cg, cs);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, 0xa48460, 0x64, 0x64, cg, cg, adj, cs);
            }
        }
    }
}

/// Kind 0x5d: the club as title favourites (to retain it when "last placed"). Draw 0: a club figure predicts it, a
/// former player claims it, otherwise several national (or nationality's) newspapers tip them; other draws: they are
/// considered favourites (by many inside the nationality's media abroad).
/// Original: part of `Media::write_team_media_story_news_text` (0x752296..0x75276e).
pub(crate) unsafe fn team_text_title_favourites(s: &mut TeamText) {
    s.piece("title favourites (0x5d)");
    covered!("sx01_1 team text title favourites");
    let (cg, cs, kg, ks, kgl, kn) = (s.cg(), s.cs(), s.kg(), s.ks(), s.kgl(), s.kn());
    let latest = s.latest != 0;
    if s.draw != 0 {
        if s.home {
            team_text!(s, if latest { 0xa47f58 } else { 0xa47e9c }, cg, cg, kgl, kgl, cs, kn);
        } else {
            let adj = s.adjective(0x1c);
            team_text!(s, if latest { 0xa47dbc } else { 0xa47cdc }, cg, cg, 0x64, 0x64, kgl, kgl, cs, adj, kn);
        }
        return;
    }
    if team_text_person(s, |s, job, jg| {
        let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
        team_text!(s, if latest { 0xa47bf8 } else { 0xa47b0c }, pg, pg, jg, jg, cg, cg, kg, kg, ps, job, pn, cs, ks)
    }, |s| {
        let pn = s.pname();
        team_text!(s, if latest { 0xa47a48 } else { 0xa47980 }, cg, cg, kg, kg, pn, cs, ks)
    }) {
        return;
    }
    if s.home {
        team_text!(s, if latest { 0xa478e4 } else { 0xa4784c }, cg, cg, kgl, kgl, cs, kn);
    } else {
        let adj = s.adjective(0x1b);
        team_text!(s, if latest { 0xa47798 } else { 0xa476e8 }, 0x64, 0x64, cg, cg, kgl, kgl, adj, cs, kn);
    }
}

/// Kind 0x62: the club expected to last the pace. Draw 0: a club figure says they will retain (holders) or cannot be
/// stopped from winning the division, a former player claims they cannot be stopped, otherwise intense media
/// speculation (or in the nation) whether they can be stopped; draw 1: their (relentless) winning form convinces the
/// media (or the nationality's media); other: the media (or the nationality's media) are convinced they go on to
/// retain or win it.
/// Original: part of `Media::write_team_media_story_news_text` (0x75276e..0x752cf2).
pub(crate) unsafe fn team_text_title_pace(s: &mut TeamText) {
    s.piece("title pace (0x62)");
    covered!("sx01_1 team text title pace");
    let (cg, cs, kg, ks, kgl, kn) = (s.cg(), s.cs(), s.kg(), s.ks(), s.kgl(), s.kn());
    let latest = s.latest != 0;
    match s.draw {
        0 => {
            if team_text_person(s, |s, job, jg| {
                let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                team_text!(s, if latest { 0xa46ec8 } else { 0xa46dc8 }, pg, pg, jg, jg, cg, cg, kg, kg, ps, job, pn, cs, ks)
            }, |s| {
                let pn = s.pname();
                team_text!(s, 0xa46cec, cg, cg, kg, kg, pn, cs, ks)
            }) {
                return;
            }
            if s.home {
                team_text!(s, if latest { 0xa46c34 } else { 0xa46b7c }, cg, cg, cs);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, if latest { 0xa46aa8 } else { 0xa469d4 }, ng, ng, cg, cg, ns, cs);
            }
        }
        1 => {
            if s.home {
                team_text!(s, if latest { 0xa472a4 } else { 0xa471c4 }, cg, cg, kgl, kgl, cs, kn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, if latest { 0xa470c0 } else { 0xa46fc0 }, cg, cg, 0x64, 0x64, kgl, kgl, cs, adj, kn);
            }
        }
        _ => {
            if s.home {
                team_text!(s, if latest { 0xa47620 } else { 0xa4755c }, cg, cg, kgl, kgl, cs, kn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, if latest { 0xa47470 } else { 0xa47388 }, 0x64, 0x64, cg, cg, kgl, kgl, adj, cs, kn);
            }
        }
    }
}

/// Kind 0x40: the promotion bid stutters. Draw 0: a club figure says they are in danger of blowing their chances, a
/// former player claims it, otherwise intense media speculation; other draws: the media claim the bid for promotion
/// is stuttering to a halt.
/// Original: part of `Media::write_team_media_story_news_text` (0x752cf2..0x752e5c).
pub(crate) unsafe fn team_text_promotion_stutter(s: &mut TeamText) {
    s.piece("promotion stutter (0x40)");
    covered!("sx01_1 team text promotion stutter");
    let (cg, cs) = (s.cg(), s.cs());
    if s.draw != 0 {
        let (kgl, kn) = (s.kgl(), s.kn());
        team_text!(s, 0xa46910, cg, cg, kgl, kgl, cs, kn);
        return;
    }
    if team_text_person(s, |s, job, jg| {
        let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
        team_text!(s, 0xa46824, pg, pg, jg, jg, cg, cg, ps, job, pn, cs)
    }, |s| {
        let pn = s.pname();
        team_text!(s, 0xa46764, cg, cg, pn, cs)
    }) {
        return;
    }
    team_text!(s, 0xa466b8, cg, cg, cs);
}

/// Kind 0x41: the title bid stutters. Draw 0: a club figure says they are feeling the pressure, a former player
/// believes the players feel it, otherwise intense media speculation (or in the nation) that they may blow their
/// chances; draw 1: speculation (or in the nation) that they feel the pressure at the top; other: the media (or the
/// nationality's media) claim the bid to retain (holders) or for the title is stuttering.
/// Original: part of `Media::write_team_media_story_news_text` (0x752e5c..0x75321b).
pub(crate) unsafe fn team_text_title_stutter(s: &mut TeamText) {
    s.piece("title stutter (0x41)");
    covered!("sx01_1 team text title stutter");
    let (cg, cs, kgl, kn) = (s.cg(), s.cs(), s.kgl(), s.kn());
    let latest = s.latest != 0;
    match s.draw {
        0 => {
            if team_text_person(s, |s, job, jg| {
                let (pg, ps, pn) = (s.pg(), s.ps(), s.pname());
                team_text!(s, 0xa46078, pg, pg, jg, jg, cg, cg, ps, job, pn, cs)
            }, |s| {
                let (pn, psh) = (s.pname(), s.pshort());
                team_text!(s, 0xa45f68, cg, cg, pn, cs, psh)
            }) {
                return;
            }
            if s.home {
                team_text!(s, 0xa45ebc, cg, cg, cs);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa45df4, ng, ng, cg, cg, ns, cs);
            }
        }
        1 => {
            if s.home {
                team_text!(s, 0xa4628c, cg, cg, kgl, kgl, cs, kn);
            } else {
                let (ng, ns) = (s.ng(), s.ns());
                team_text!(s, 0xa46180, ng, ng, cg, cg, kgl, kgl, ns, cs, kn);
            }
        }
        _ => {
            if s.home {
                team_text!(s, if latest { 0xa465f8 } else { 0xa4653c }, cg, cg, kgl, kgl, cs, kn);
            } else {
                let adj = s.adjective(0x1c);
                team_text!(s, if latest { 0xa46458 } else { 0xa4637c }, 0x64, 0x64, cg, cg, kgl, kgl, adj, cs, kn);
            }
        }
    }
}

/// The story's first paragraph by kind; kinds 0x50, 0x51 and 0x3d without a match clear the shared text buffer
/// (0xde1f64) instead and leave the story text unwritten; an unknown kind gets the programmer's joke sentence.
/// Original: part of `Media::write_team_media_story_news_text` (0x74c351..0x753238).
pub(crate) unsafe fn team_text_body(s: &mut TeamText) {
    if matches!(s.kind, 0x50 | 0x51 | 0x3d) && s.has_game == 0 {
        covered!("sx01_1 team text no match");
        s.piece("no match");
        write_u8(BACKGROUND_TEXT, 0); // the shared text buffer, not the story's
        TEAM_TEXT_UNSET.store(true, Relaxed);
        return;
    }
    match s.kind {
        0x50 => team_text_unbeaten_run_ended(s),
        0x51 => team_text_winless_run_ended(s),
        0x3f => team_text_relegation_battle(s),
        0x5f => team_text_promotion_push(s),
        0x60 => team_text_escaping_relegation(s),
        0x42 => team_text_not_contenders(s),
        0x43 => team_text_boring(s),
        0x44 => team_text_tournament_outsiders(s),
        0x5a => team_text_tournament_favourites(s),
        0x5c => team_text_continental_favourites(s),
        0x5b => team_text_big_spenders(s),
        0x52 => team_text_striker_starved(s),
        0x3e => team_text_relegation_slump(s),
        0x5e => team_text_winning_ways(s),
        0x3c => team_text_losing_patience(s),
        0x3d => team_text_supporters_fury(s),
        0x63 => team_text_title_credentials(s),
        0x61 => team_text_title_outsiders(s),
        0x5d => team_text_title_favourites(s),
        0x62 => team_text_title_pace(s),
        0x40 => team_text_promotion_stutter(s),
        0x41 => team_text_title_stutter(s),
        _ => {
            s.piece("unknown kind");
            let (t, pn) = (s.text.as_mut_ptr(), s.pname());
            story_crt!(t, 0xa45d68, pn); // "A newspaper article questioned whether %s's ability to program ..."
        }
    }
}

/// The reply paragraph when the story's manager is the human on screen ("You have ..."): by the reply 1 (positive)
/// or 2 (negative) and the kind; reply 3, a supporters' club spokesman (club % 4 = 0) or spokeswoman for kind 0x3d with
/// club % 3 = 0, otherwise "You have decided to ignore this news report."; no reply, an empty paragraph.
/// Original: part of `Media::write_team_media_story_news_text` (0x753238..0x753de7, the on-screen half).
pub(crate) unsafe fn team_text_reply_yours(s: &mut TeamText) {
    let k = s.kind;
    let psh = s.pshort();
    match s.reply {
        1 => {
            covered!("sx01_1 team text your positive reply");
            let tpl = match k {
                0x3c => 0xa456a4,
                0x3d => 0xa4564c,
                0x5d => 0xa45608,
                0x63 => 0xa45590,
                0x62 => if s.latest != 0 { 0xa4554c } else { 0xa4550c },
                0x61 => 0xa454c4,
                0x40 | 0x41 => 0xa45480,
                0x3f => 0xa45420,
                0x5f => 0xa453d0,
                0x60 => 0xa4537c,
                0x42 => 0xa45334,
                0x43 => 0xa452f0,
                0x44 => 0xa452ac,
                0x5a => 0xa45264,
                0x5c => {
                    let (cn, cg) = s.continent();
                    team_second!(s, if s.latest != 0 { 0xa45200 } else { 0xa451a4 }, cg, cg, cn);
                    return;
                }
                0x5b => 0xa45154,
                0x52 => 0xa45110,
                _ => return team_second!(s, 0xad9c64),
            };
            team_second!(s, tpl);
        }
        2 => {
            covered!("sx01_1 team text your negative reply");
            let tpl = match k {
                0x3c => 0xa45c5c,
                0x3d => 0xa45bb4,
                0x5d => 0xa45b64,
                0x63 | 0x61 => 0xa45b20,
                0x62 => 0xa45ad4,
                0x40 => 0xa45a88,
                0x41 => 0xa45a34,
                0x3f => 0xa459e8,
                0x5f => 0xa459a0,
                0x60 => 0xa45964,
                0x42 => 0xa45924,
                0x43 => 0xa458f0,
                0x44 => 0xa458ac,
                0x5a => 0xa4586c,
                0x5c => {
                    let (cn, cg) = s.continent();
                    team_second!(s, if s.latest != 0 { 0xa45804 } else { 0xa457a4 }, cg, cg, cn);
                    return;
                }
                0x5b => 0xa4576c,
                0x52 => return team_second!(s, 0xa4570c, psh),
                _ => return team_second!(s, 0xad9c64),
            };
            team_second!(s, tpl);
        }
        3 => {
            covered!("sx01_1 team text your ignore reply");
            team_text_reply_ignore(s, 0xa1ba14); // "You have decided to ignore this news report."
        }
        _ => {
            let t = s.second.as_mut_ptr();
            story_crt!(t, 0xad9c64);
        }
    }
}

/// Reply 3 to a kind 0x3d story at a club with index % 3 = 0: the supporters' club spokesman (index % 4 = 0) or
/// spokeswoman; otherwise `other` (or, with 0, an empty paragraph by the C runtime).
unsafe fn team_text_reply_ignore(s: &mut TeamText, other: usize) {
    if s.kind == 0x3d && s.club_index % 3 == 0 {
        team_second!(s, if s.club_index % 4 == 0 { 0xa45cb8 } else { 0xa4823c });
    } else if other != 0 {
        team_second!(s, other);
    } else {
        let t = s.second.as_mut_ptr();
        story_crt!(t, 0xad9c64);
    }
}

/// The reply paragraph of a computer manager ("Manager {name} ..."): by the reply 1 (the story eases: unlucky,
/// talk is premature, still developing...) or 2 (the manager agrees or is confident) and the kind; some kinds pick by
/// the manager's confidence flags (slot 5, slot 6) or "last placed"; an unknown kind writes an empty paragraph (the C
/// runtime; langlib for reply 1 to kind 0x63 with neither flag); reply 3 as `team_text_reply_ignore`, without the
/// "ignore" sentence.
/// Original: part of `Media::write_team_media_story_news_text` (0x752149..0x753de7, the computer half).
pub(crate) unsafe fn team_text_reply_manager(s: &mut TeamText) {
    let k = s.kind;
    let (mn, psh) = (s.mname(), s.pshort());
    match s.reply {
        1 => {
            covered!("sx01_1 team text manager reply 1");
            let tpl = match k {
                0x3c => 0xa446d4,
                0x3d => 0xa44654,
                0x5d => 0xa445dc,
                0x63 => {
                    if s.confident != 0 {
                        0xa44520
                    } else if s.cautious != 0 {
                        0xa44480
                    } else {
                        return team_second!(s, 0xad9c64);
                    }
                }
                0x62 => 0xa44410,
                0x61 => 0xa443a4,
                0x40 => 0xa44334,
                0x41 => 0xa442b8,
                0x3f => 0xa44248,
                0x5f => 0xa441d8,
                0x60 => 0xa44178,
                0x42 => 0xa44114,
                0x43 => 0xa440bc,
                0x44 => 0xa44054,
                0x5a => 0xa43fe8,
                0x5c => {
                    let (cn, cg) = s.continent();
                    team_second!(s, if s.latest != 0 { 0xa43f54 } else { 0xa43ecc }, cg, cg, mn, cn);
                    return;
                }
                0x5b => 0xa43e70,
                0x52 => return team_second!(s, 0xa43df0, 0x64, mn, psh),
                _ => {
                    let t = s.second.as_mut_ptr();
                    story_crt!(t, 0xad9c64);
                    return;
                }
            };
            team_second!(s, tpl, mn);
        }
        2 => {
            covered!("sx01_1 team text manager reply 2");
            let tpl = match k {
                0x3c => 0xa45090,
                0x3d => 0xa44fc0,
                0x5d => 0xa44f50,
                0x63 => if s.confident != 0 { 0xa44ec8 } else { 0xa44e2c },
                0x62 => 0xa44ec8,
                0x61 => 0xa44dbc,
                0x40 | 0x41 => 0xa44d58,
                0x3f => 0xa44cc0,
                0x5f => 0xa44c44,
                0x60 => 0xa44bcc,
                0x42 => 0xa44b60,
                0x43 => 0xa44af4,
                0x44 => 0xa44a84,
                0x5a => if s.confident != 0 { 0xa44a08 } else { 0xa44978 },
                0x5c => {
                    let (cn, cg) = s.continent();
                    team_second!(s, if s.latest != 0 { 0xa448f4 } else { 0xa44874 }, cg, cg, mn, cn);
                    return;
                }
                0x5b => 0xa447f4,
                0x52 => return team_second!(s, 0xa4476c, mn, psh),
                _ => {
                    let t = s.second.as_mut_ptr();
                    story_crt!(t, 0xad9c64);
                    return;
                }
            };
            team_second!(s, tpl, mn);
        }
        3 => {
            covered!("sx01_1 team text manager reply 3");
            team_text_reply_ignore(s, 0);
        }
        _ => {
            let t = s.second.as_mut_ptr();
            story_crt!(t, 0xad9c64);
        }
    }
}

/// The links of the body: the person (0x7d7, his name when his ids match, `database_staff_matches_names`), the
/// manager (0x7d7), the match (0x7d3 on the score when the match statistics hold it, `Match_Stats::fixture_entry_matches`
/// of `G_MATCH_STATS`) and its opponent (0x7d5, long name), the person again (name and short name), the competition
/// (0x7d0, long and short name), the club (0x7d5, short and long name), the club's nation as its national team (0x7d5,
/// long and short name, `database_get_national_team`) and the ten nationality adjectives 0x16..0x1f of that nation.
/// Original: part of `Media::write_team_media_story_news_text` (0x753e25..0x754707).
pub(crate) unsafe fn team_text_links(s: &mut TeamText, out: usize) {
    let staff_matches = |p: usize, ids: [u32; 3]| exe::database_staff_matches_names(p, ids[0], ids[1], ids[2], 0usize) != 0;
    if s.person > -1 {
        let p = staff_at(s.person as isize).addr();
        if staff_matches(p, s.person_ids) {
            exe::text_add_link(out, s.person_name.as_ptr(), 0x7d7, p);
        }
    }
    let m = staff_at(s.manager as isize).addr();
    if staff_matches(m, s.manager_ids) {
        exe::text_add_link(out, s.manager_name.as_ptr(), 0x7d7, m);
    }
    if s.has_game != 0 {
        let rec = s.record.as_ptr() as usize;
        let slot = read_i32(rec + club_match_record::MATCH_STATS_SLOT);
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot, read_i32(rec + club_match_record::CLUB_ID),
            read_i32(rec + club_match_record::OPPONENT_ID), read_i32(rec), read_i32(rec + 2)) != 0
        {
            exe::text_add_link(out, s.score.as_ptr(), 0x7d3, slot as u32 as usize);
        }
        let o = s.opponent();
        exe::text_add_link(out, TeamText::long(o) as *const u8, 0x7d5, o);
    }
    if s.person > -1 {
        let p = staff_at(s.person as isize).addr();
        if staff_matches(p, s.person_ids) {
            exe::text_add_link(out, s.person_name.as_ptr(), 0x7d7, p);
            exe::text_add_link(out, s.person_short.as_ptr(), 0x7d7, p);
        }
    }
    let k = s.comp();
    exe::text_add_link(out, TeamText::long(k) as *const u8, 0x7d0, k);
    exe::text_add_link(out, TeamText::short(k) as *const u8, 0x7d0, k);
    let c = s.club();
    exe::text_add_link(out, TeamText::short(c) as *const u8, 0x7d5, c);
    exe::text_add_link(out, TeamText::long(c) as *const u8, 0x7d5, c);
    let team = exe::database_get_national_team(s.nation());
    exe::text_add_link(out, TeamText::long(s.nation()) as *const u8, 0x7d5, team);
    let team = exe::database_get_national_team(s.nation());
    exe::text_add_link(out, TeamText::short(s.nation()) as *const u8, 0x7d5, team);
    for form in 0x16..=0x1f {
        let adj = s.adjective(form);
        let team = exe::database_get_national_team(s.nation());
        exe::text_add_link(out, adj as *const u8, 0x7d5, team);
    }
}

/// The headline by kind ("{club} beaten", "{club} halt slide", "Boring {club}", ...): kind 0x5a by (club + 90 × draw)
/// % 2 (press or pundit), 0x5c by (club + draw) % 3 (make their mark on the continent, tipped for glory, fly the
/// flag), 0x63 by the draw, 0x5d and 0x41 with the person's short name when there is one, 0x42 and 0x44 at draw 0 with
/// the person's short name; then the club's short name is linked (0x7d5).
/// Original: part of `Media::write_team_media_story_news_text` (0x75434d..0x75471f).
pub(crate) unsafe fn team_text_headline(s: &mut TeamText, out: usize) {
    s.piece("headline");
    covered!("sx01_1 team text headline");
    let (cg, cs, psh) = (s.cg(), s.cs(), s.pshort());
    let club_only = |s: &mut TeamText, tpl: usize| team_text!(s, tpl, cg, cg, cs);
    match s.kind {
        0x50 => club_only(s, 0xa43dc4),
        0x51 => club_only(s, 0xa43d94),
        0x3f => club_only(s, 0xa43d64),
        0x5f => club_only(s, 0xa43d28),
        0x60 => club_only(s, 0xa43cf0),
        0x42 | 0x44 => {
            if s.draw == 0 && s.person > -1 {
                team_text!(s, 0xa43800, cg, cg, psh, cs);
            } else {
                club_only(s, 0xa437c4);
            }
        }
        0x43 => club_only(s, 0xa43cc4),
        0x5a => club_only(s, if s.club_index.wrapping_add(s.draw.wrapping_mul(90)) % 2 != 0 { 0xa43c94 } else { 0xa43c68 }),
        0x5c => match s.club_index.wrapping_add(s.draw) % 3 {
            0 => {
                let (cn, cng) = s.continent();
                team_text!(s, 0xa43ba4, cg, cg, cng, cng, cs, cn);
            }
            1 => club_only(s, 0xa43bfc),
            _ => club_only(s, 0xa43c34),
        },
        0x52 => team_text!(s, 0xa43b6c, psh),
        0x5b => club_only(s, 0xa43b30),
        0x3e => club_only(s, 0xa43afc),
        0x5e => club_only(s, 0xa43acc),
        0x3c => club_only(s, 0xa43a98),
        0x3d => club_only(s, 0xa43a64),
        0x63 => club_only(s, if s.draw != 0 { 0xa43a2c } else { 0xa439f8 }),
        0x5d => {
            if s.person != -1 {
                team_text!(s, 0xa43988, cg, cg, psh, cs);
            } else {
                club_only(s, 0xa439cc);
            }
        }
        0x62 => club_only(s, 0xa4395c),
        0x61 => club_only(s, 0xa43930),
        0x40 => club_only(s, 0xa43900),
        0x41 => {
            if s.person != -1 {
                team_text!(s, 0xa43888, cg, cg, psh, cs);
            } else {
                club_only(s, 0xa438d0);
            }
        }
        _ => club_only(s, 0xa43860),
    }
    exe::text_set_string(out, s.text.as_ptr());
    let c = s.club();
    exe::text_add_link(out, TeamText::short(c) as *const u8, 0x7d5, c); // sx01_1: the headline's club link
}

/// `Media::write_team_media_story_news_text(news, body, out)`: writes the headline or the body of a media story about
/// a club or national team (news 0xbdf, posted by `post_team_media_story`) into `out`, with the links.
///
/// The slots and the point of view (`team_text_read`, `team_text_names`), the outlet or pundit (`team_text_outlet`).
/// Body: the first paragraph by kind (one piece each, `team_text_body`), then the reply paragraph: "You have ..." when
/// the story's manager is the human on screen (`team_text_reply_yours`), otherwise "Manager ..." for the computer's
/// reply (`team_text_reply_manager`); the two are joined as "%s\n\n%s" in a frame buffer and set on `out`
/// (`exe::text_set_string`), then the links (`team_text_links`). Headline: `team_text_headline`.
///
/// Defects of the original: kinds 0x50, 0x51 and 0x3d without a match clear the shared buffer 0xde1f64 and leave the
/// story text unwritten, so the body joins whatever the stack held with the reply (the copy uses an empty text and the
/// comparer accepts any prefix there); several pieces fetch the nationality adjective and then do not use it (kept);
/// a person's job name is kept in frame slots that hold the pundit or the extra word (no effect after the outlet).
/// Original: `Media::write_team_media_story_news_text` (0x74bbd0, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_team_media_story_news_text(_this: usize, news: usize, body: u32, out: usize) {
    TEAM_TEXT_UNSET.store(false, Relaxed);
    let mut s = Box::new(team_text_read(news));
    team_text_names(&mut s);
    team_text_outlet(&mut s);
    if body == 0 {
        team_text_headline(&mut s, out);
        return;
    }
    covered!("sx01_1 team text body");
    team_text_body(&mut s);
    if s.on_screen {
        team_text_reply_yours(&mut s);
    } else {
        team_text_reply_manager(&mut s);
    }
    let mut joined = vec![0u8; 0x1600];
    story_crt!(joined.as_mut_ptr(), 0x991948, s.text.as_ptr(), s.second.as_ptr()); // "%s\n\n%s"
    exe::text_set_string(out, joined.as_ptr());
    team_text_links(&mut s, out);
}

/// Comparer for `write_team_media_story_news_text`: the original writes into `out`, the copy into a local Text side by
/// side (the generator's state, the copy's draws not counted); the strings, the links (text, type, data) and the
/// generator must match. When the copy hit the original's unwritten story text (kinds 0x50, 0x51, 0x3d without a
/// match), the original's string only has to end with the copy's ("\n\n" and the reply). The message names the piece.
pub(crate) unsafe extern "thiscall" fn compare_write_team_media_story_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_TEAM_MEDIA_STORY_NEWS_TEXT);
    let before = text_contents(out).1.len();
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, body, out),
        || crate::rng::without_counting(|| write_team_media_story_news_text(this, news, body, lp)));
    let (mut mo, mr) = (text_contents(out), text_contents(lp));
    exe::drop_text(local.as_mut_ptr());
    mo.1.drain(..before.min(mo.1.len()));
    let unset = TEAM_TEXT_UNSET.load(Relaxed);
    let text_same = mo.0 == mr.0 || (unset && mo.0.ends_with(&mr.0));
    let piece = TEAM_TEXT_PIECE.lock().map(|p| *p).unwrap_or("");
    let show = |t: &(Vec<u8>, Vec<(Vec<u8>, u32, u32)>)| {
        format!("{:?} {:x?}", String::from_utf8_lossy(&t.0), t.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>())
    };
    SCOREBOARD.record(same_rng && text_same && mo.1 == mr.1, || format!(
        "write_team_media_story_news_text(kind {}, body {}, piece {}, unset {}) original {} rust {}",
        exe::newsitem_get_slot(news, 9) as i32, body, piece, unset, show(&mo), show(&mr)));
}

// ---------- sx04_1: the month change of a nation's awards, the month award's new season and the month ratings order (nation_awards.cpp, month_award.cpp, month_ratings.cpp) ----------

/// A nation's awards object (`NationAwards`, nation_awards.cpp): +0 vtable, +4 award table, +8 award count (i16), +0xa
/// month ratings (`MONTH_RATINGS`, 0 when the nation has none).
pub(crate) mod nation_awards_obj {
    pub(crate) const MONTH_RATINGS: usize = 0xa;
    /// The virtual `on_month_change()` (thiscall, no arguments): the month awards of the nation.
    pub(crate) const ON_MONTH_CHANGE_SLOT: usize = 0xc;
}

/// The month award's slot count (i8): how many of the twelve 11-byte month slots are in use.
const MONTH_AWARD_SLOT_COUNT: usize = 0xa1;
/// One month slot of a month award (12 of them from `month_award_fields::SLOT_WINNER_ID`).
const MONTH_AWARD_SLOT_SIZE: usize = 11;

/// `MONTH_RATINGS::reset(month, year)` (0x767f60, thiscall, ret 8); the month and the year go as the original pushes
/// them, in whole dwords.
const FN_MONTH_RATINGS_RESET: usize = 0x767f60;

/// The month and the year (with the leap-year word above it, as the original pushes it) of tomorrow, when tomorrow is
/// in another month than the one the nation's month ratings cover; `None` when it is the same month or the nation has
/// no month ratings.
unsafe fn nation_awards_new_month(this: usize, today: usize) -> Option<(u32, u32)> {
    let mut tomorrow = [0u8; 8];
    exe::date_add_days(today, tomorrow.as_mut_ptr() as usize, 1);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    if ratings == 0 {
        return None;
    }
    let month = exe::date_get_month(tomorrow.as_ptr() as usize);
    let year = read_u16(tomorrow.as_ptr() as usize + cm_date::YEAR);
    // the ratings' year is signed, tomorrow's is not
    if read_u8(ratings + month_ratings_fields::MONTH) == month && read_i16(ratings + month_ratings_fields::YEAR) as i32 == year as i32 {
        return None;
    }
    Some((month as u32, read_u32(tomorrow.as_ptr() as usize + cm_date::YEAR)))
}

/// Closes a nation's month on its last day: when tomorrow (`today` + 1 day) falls in another month or year than the
/// one the nation's month ratings cover, it runs the nation's month awards (the virtual `on_month_change`, slot +0xc:
/// player, young player and manager of the month) and then empties the month ratings for tomorrow's month and year
/// (`MONTH_RATINGS::reset`, the ratings pointer read again after the awards). A nation without month ratings does
/// nothing. The base of every nation's `daily_update`, called at its end with the day being played.
/// Original: `NationAwards::daily_update` (0x768310, nation_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_awards_daily_update(this: usize, today: usize) {
    let Some((month, year)) = nation_awards_new_month(this, today) else {
        return;
    };
    covered!("sx04_1 month change");
    let on_month_change: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + nation_awards_obj::ON_MONTH_CHANGE_SLOT));
    on_month_change(this); // sx04_1: the month awards
    let reset: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_MONTH_RATINGS_RESET);
    reset(read_usize(this + nation_awards_obj::MONTH_RATINGS), month, year); // sx04_1: the new month's ratings
}

/// Comparer for `nation_awards_daily_update`.
///
/// The month awards and the reset cannot run twice: only the original runs. The copy's decision is worked out first
/// (pure: tomorrow's date and the ratings' month); then, on a month change, the ratings must be empty and cover
/// tomorrow's month and year; otherwise their month and year must be unchanged.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_daily_update(this: usize, today: usize) {
    let plan = nation_awards_new_month(this, today);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    let before = if ratings != 0 { (read_u8(ratings + month_ratings_fields::MONTH), read_i16(ratings + month_ratings_fields::YEAR)) } else { (0, 0) };
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NATION_AWARDS_DAILY_UPDATE)(this, today);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    if ratings == 0 {
        SCOREBOARD.record(plan.is_none(), || format!("nation_awards_daily_update(today {:#x}): no month ratings, rust planned {:?}", read_u32(today), plan));
        return;
    }
    let after = (read_u8(ratings + month_ratings_fields::MONTH), read_i16(ratings + month_ratings_fields::YEAR));
    let ok = match plan {
        Some((m, y)) => after == (m as u8, y as u16 as i16) && read_u32(ratings + 4) == 0,
        None => after == before,
    };
    SCOREBOARD.record(ok, || format!("nation_awards_daily_update(today {:#x}): month/year before {:?} after {:?}, rust planned {:?}", read_u32(today), before, after, plan));
}

/// Moves a month award on to the next season: unless the first month slot is already in `year` (or no slot is in
/// use), every used slot (`MONTH_AWARD_SLOT_COUNT`, read again at each step) goes one year on
/// (`month_award_fields::SLOT_YEAR` + 1) and loses its winner and club (`SLOT_WINNER_ID`, `SLOT_CLUB_ID` = −1). Called
/// by each nation's `daily_update` on the nation's season start day, with the current year.
/// Original: `MONTH_AWARD::start_new_season` (0x7672e0, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_new_season(this: usize, year: u16) {
    if read_i8(this + MONTH_AWARD_SLOT_COUNT) <= 0 || read_u16(this + month_award_fields::SLOT_YEAR) == year {
        return;
    }
    covered!("sx04_1 month slots moved on");
    let mut k = 0;
    while k < read_i8(this + MONTH_AWARD_SLOT_COUNT) as i32 {
        let slot = this + k as usize * MONTH_AWARD_SLOT_SIZE;
        write_u16(slot + month_award_fields::SLOT_YEAR, read_u16(slot + month_award_fields::SLOT_YEAR).wrapping_add(1)); // sx04_1: one year on
        write_i32(slot + month_award_fields::SLOT_WINNER_ID, -1);
        write_i32(slot + month_award_fields::SLOT_CLUB_ID, -1);
        k += 1;
    }
}

/// Comparer for `month_award_new_season`: side by side over the twelve month slots.
pub(crate) unsafe extern "thiscall" fn compare_month_award_new_season(this: usize, year: u16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u16)>(IDX_MONTH_AWARD_NEW_SEASON);
    let regions = [(this + month_award_fields::SLOT_WINNER_ID, 12 * MONTH_AWARD_SLOT_SIZE)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, year), || month_award_new_season(this, year));
    SCOREBOARD.record(same, || format!("month_award_new_season(year {}) original={:x?} rust={:x?}", year, mo, mr));
}

/// The qsort order of the month ratings (12-byte `month_rating_entry` records): the best average rating first, with
/// those of fewer than 3 matches after all the others. `a` with fewer than 3 matches: 1 when `b` has at least 3, else 0;
/// `b` with fewer than 3 (and `a` not): −1; otherwise the averages (the rating sum, i16 at +8, / `month_rating_entry::MATCHES`, signed)
/// decide: −1 when `a`'s is higher, 1 when lower, 0 when equal. Used by `find_best_player`.
/// Original: `month_ratings_compare_average` (0x768200, month_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn month_ratings_compare_average(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (ma, mb) = (read_i8(a + month_rating_entry::MATCHES), read_i8(b + month_rating_entry::MATCHES));
    if ma < 3 { // sx04_1: fewer than three matches go last
        return (mb >= 3) as i32;
    }
    if mb < 3 {
        return -1;
    }
    covered!("sx04_1 averages compared");
    let average = |e: usize, m: i8| read_i16(e + 8) as i32 as f64 / m as i32 as f64; // +8: the rating sum
    let (va, vb) = (average(a, ma), average(b, mb));
    if va > vb {
        -1
    } else if va < vb {
        1
    } else {
        0
    }
}

compare_order!(compare_month_ratings_compare_average, month_ratings_compare_average, IDX_MONTH_RATINGS_COMPARE_AVERAGE);

// ---------- sx04_2: a nation's yearly awards from the season: the player and the manager of the year and the team of the season (nation_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\nation_awards.cpp" (the error boxes).
const NATION_AWARDS_CPP: usize = 0xa5b26c;

/// `YEAR_AWARD::record_winners(record, add_to_list)` (0x9302f0, thiscall, ret 8): the winners record goes to the
/// history, the three winners' reputations rise and their staff records get the award.
const FN_YEAR_AWARD_RECORD_WINNERS: usize = 0x9302f0;
/// `TEAM_AWARD::clear_selection()` (0x8afd90, thiscall).
const FN_TEAM_AWARD_CLEAR_SELECTION: usize = 0x8afd90;
/// `TEAM_AWARD::add_candidate(person, club, f32 rating, 0)` (0x8afa40, thiscall, ret 0x10): nonzero when the team is full.
const FN_TEAM_AWARD_ADD_CANDIDATE: usize = 0x8afa40;
/// `TEAM_AWARD::set_season_dates(year)` (0x8aff50, thiscall, ret 4).
const FN_TEAM_AWARD_SET_SEASON_DATES: usize = 0x8aff50;
/// `TEAM_AWARD::record_winners()` (0x8affb0, thiscall): the team goes to the history, reputations rise, the selection
/// is cleared.
const FN_TEAM_AWARD_RECORD_WINNERS: usize = 0x8affb0;
/// `Staff_Records::get_manager_of_year_candidates(clubs, n)` (0x86fab0, thiscall, ret 8): sorts the clubs, keeps up to
/// three at the start and returns how many (al).
const FN_STAFF_RECORDS_GET_MANAGER_OF_YEAR_CANDIDATES: usize = 0x86fab0;

/// The year award winners record (`YEAR_AWARD::record_winners`): the year (word) and three 0x18-byte entries (first,
/// second and common name ids, staff id, a score, club id); the history file keeps it whole.
const YEAR_RECORD_SIZE: usize = 0x4a;
const YEAR_RECORD_ENTRY: usize = 0x18;
/// The entry's score dword (+0x10 of an entry): the statistic for the stat-2 awards, 0 for the others.
const YEAR_RECORD_SCORE: usize = 0x10;
/// A year award's offsets of its records in the year award history file (`YEAR_AWARD +0x23 first, +0x27 last`).
const YEAR_AWARD_HISTORY_FIRST: usize = 0x23;
const YEAR_AWARD_HISTORY_LAST: usize = 0x27;
/// A team award's selection: the 0x205 bytes from +0x4e (twenty 0x19-byte candidates, the season dates +0x242/+0x24a,
/// the count +0x252) the history file keeps, then the number of substitutes (+0x253), then its history offsets.
const TEAM_AWARD_RECORD: usize = 0x4e;
const TEAM_RECORD_SIZE: usize = 0x205;
const TEAM_AWARD_SUBSTITUTES_OFFSET: usize = 0x253;
const TEAM_AWARD_HISTORY_FIRST: usize = 0x254;
const TEAM_AWARD_HISTORY_LAST: usize = 0x258;

/// A row of the competition's sorted player table (`comp_stats_get_sorted_player_table`, 0x2d bytes): appearances
/// (+0xc), of them as a substitute (+0xd), the statistic of the stat-2 awards (+0xe) and the rating sum (i16, +0x1a).
const ROW_APPEARANCES: usize = 0xc;
const ROW_SUB_APPEARANCES: usize = 0xd;
const ROW_STAT: usize = 0xe;
const ROW_RATING_SUM: usize = 0x1a;

/// What the award routines do with their winners: record them in the game, or (a comparer's dry run) only keep them.
pub(crate) type YearAwardSink<'a> = &'a mut dyn FnMut(usize, &[u8; YEAR_RECORD_SIZE]);
pub(crate) type TeamAwardSink<'a> = &'a mut dyn FnMut(usize);

/// The game's sink: `YEAR_AWARD::record_winners(record, 1)`.
unsafe fn record_year_award(award: usize, record: &[u8; YEAR_RECORD_SIZE]) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_YEAR_AWARD_RECORD_WINNERS);
    f(award, record.as_ptr() as usize, 1);
}

/// The game's sink: `TEAM_AWARD::record_winners()`.
unsafe fn record_team_award(award: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEAM_AWARD_RECORD_WINNERS);
    f(award);
}

/// The league object of the game's competition table (`G_COMP_OBJECT_TABLE`) at `index`.
unsafe fn award_league(index: u32) -> League {
    League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(index as usize * 4)))
}

/// The league games a club plays: (club count − 1) × legs, as a 16-bit product.
unsafe fn award_league_games(league: League) -> i16 {
    league.club_count().wrapping_sub(1).wrapping_mul(league.legs())
}

/// A name record's id (`names::ID`), −1 without one.
unsafe fn award_name_id_of_name(name: usize) -> u32 {
    if name != 0 { read_u32(name + names::ID) } else { u32::MAX }
}

/// Writes entry `k` of a year award record: the person's three name ids and staff id, the club id; the score only
/// when given (the manager award leaves it as the stack had it).
unsafe fn year_record_entry(record: &mut [u8; YEAR_RECORD_SIZE], k: usize, person: Staff, score: Option<u32>, club_id: u32) {
    let e = 2 + k * YEAR_RECORD_ENTRY;
    let mut put = |off: usize, v: u32| record[e + off..e + off + 4].copy_from_slice(&v.to_le_bytes());
    put(0, award_name_id_of_name(person.first_name()));
    put(4, award_name_id_of_name(person.second_name()));
    put(8, award_name_id_of_name(person.common_name()));
    put(0xc, read_u32(person.addr()));
    if let Some(s) = score {
        put(YEAR_RECORD_SCORE, s);
    }
    put(0x14, club_id);
}

/// The yearly player award of a competition from its season statistics (see `nation_awards_award_player_of_year`),
/// with the winners record going to `sink`.
pub(crate) unsafe fn nation_awards_award_player_of_year_with(comp_index: u32, today: usize, award: usize, age: u32, positions: u32,
    stat: u32, nation: i32, club_nation: i32, not_nation: i32, home_nation: i32, sink: YearAwardSink) {
    let stat_full = stat;
    let stat = stat as u8;
    // the share of the season's games a candidate must have started, lowered by 0.1 a pass
    let mut share: f32 = if stat == 2 || (stat > 3 && stat <= 5) { f32::from_bits(0x3dcccccd) } else { f32::from_bits(0x3f28f5c3) }; // 0.1 or 0.66
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() == 1 {
        tel26_error_box(NATION_AWARDS_CPP, 0x2e1);
        return;
    }
    let games = award_league_games(league);
    let mut n = 0i32;
    let rows = exe::comp_stats_get_sorted_player_table(league.comp_stats(), stat_full, &mut n, 0x64, u32::MAX);
    if n == 0 {
        return;
    }
    let mut chosen = [Staff(0); 3];
    let mut scores = [0u32; 3];
    let mut found = 0usize;
    let mut older = 0i32; // over the age limit: counted across the passes, never reset
    while share as f64 > 0.0 {
        let least = ftol(games as i32 as f64 * share as f64) as i16 as i32;
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
            let starts = read_u8(row + ROW_APPEARANCES) as i32 - read_u8(row + ROW_SUB_APPEARANCES) as i32;
            if starts < least { // sx04_2: enough starts
                continue;
            }
            let own = person.nation();
            let club_of = person.club_job();
            let club_nation_id = || if club_of.is_null() || club_of.nation().is_null() { None } else { Some(read_i32(club_of.nation().addr())) };
            if nation != -1 && (own.is_null() || read_i32(own.addr()) != nation) {
                continue;
            }
            if club_nation != -1 && club_nation_id() != Some(club_nation) {
                continue;
            }
            if not_nation != -1 && (own.is_null() || read_i32(own.addr()) == not_nation) {
                continue;
            }
            if home_nation != -1 && (club_nation_id() != Some(home_nation) || own.is_null() || read_i32(own.addr()) != home_nation) {
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
            covered!("sx04_2 player of the year candidate");
            chosen[found] = person;
            scores[found] = if stat == 2 { read_u8(row + ROW_STAT) as u32 } else { 0 };
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
    for k in 0..3 {
        let club = chosen[k].club_job();
        year_record_entry(&mut record, k, chosen[k], Some(scores[k]), if club.is_null() { u32::MAX } else { read_u32(club.addr()) });
    }
    sink(award, &record);
}

/// Gives a yearly player award from a competition's season statistics: the three best of the competition's player
/// table for the award's statistic (`comp_stats_get_sorted_player_table`, at most 100) who are players, play one of the
/// `positions` (16 bits; 0 = any), have started (appearances − substitute appearances) at least the share of the
/// league's games ((club count − 1) × legs) and meet the filters (−1 = none): `nation` the person's nation,
/// `club_nation` his club's nation, `not_nation` not the person's nation, `home_nation` both. The share starts at 0.66
/// (0.1 for the statistics 2, 4 and 5) and drops by 0.1 a pass while it stays above 0, the chosen kept. With an `age`
/// (not 0xff), those older are counted and the younger only qualify after three were counted (a count never reset).
/// With all three: the record (the year of `today`, each winner's names, staff id, the statistic for award 2 or 0, the
/// club) goes to `YEAR_AWARD::record_winners`. A missing competition or a cup: the nation_awards.cpp 737 box.
/// Original: `nation_awards_award_player_of_year` (0x768cf0, nation_awards.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_player_of_year(_this: usize, comp_index: u32, today: usize, award: usize, age: u32,
    positions: u32, stat: u32, nation: i32, club_nation: i32, not_nation: i32, home_nation: i32) {
    nation_awards_award_player_of_year_with(comp_index, today, award, age, positions, stat, nation, club_nation, not_nation, home_nation,
        &mut |award, record| record_year_award(award, record));
}

/// The yearly manager award of a league (see `nation_awards_award_manager_of_year`), with the winners record going to
/// `sink`.
pub(crate) unsafe fn nation_awards_award_manager_of_year_with(today: usize, award: usize, comp_index: u32, sink: YearAwardSink) {
    let league = award_league(comp_index);
    if league.is_null() {
        tel26_error_box(NATION_AWARDS_CPP, 0x1d4);
        return;
    }
    if league.comp_type() != 2 {
        tel26_error_box(NATION_AWARDS_CPP, 0x1db);
        return;
    }
    // the league's clubs (virtual +0xa0: a block the caller frees, and its count)
    let (mut clubs, mut n) = (0usize, 0u32);
    let get_clubs: unsafe extern "thiscall" fn(usize, *mut usize, *mut u32) = core::mem::transmute(read_usize(read_usize(league.addr()) + 0xa0));
    get_clubs(league.addr(), &mut clubs, &mut n);
    let candidates: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_OF_YEAR_CANDIDATES);
    if candidates(read_usize(G_STAFF_RECORDS), clubs, n) != 3 { // sx04_2: three managers
        exe::crt_free(clubs);
        return;
    }
    covered!("sx04_2 manager of the year");
    let mut record = [0u8; YEAR_RECORD_SIZE];
    record[..2].copy_from_slice(&read_u16(today + cm_date::YEAR).to_le_bytes());
    for k in 0..3 {
        let club = Club(read_usize(clubs + k * 4));
        // defect of the original: the score dword is not written (whatever the stack held goes to the history)
        year_record_entry(&mut record, k, club.manager(), None, read_u32(club.addr()));
    }
    sink(award, &record);
    exe::crt_free(clubs);
}

/// Gives a league's yearly manager award: the league's clubs (its virtual +0xa0) go to
/// `Staff_Records::get_manager_of_year_candidates`, which sorts them by their manager's points and keeps those whose
/// manager qualifies; with exactly three, their managers (names, staff id, club id; the score left unwritten) go with
/// the year of `today` to `YEAR_AWARD::record_winners`. The club list is freed. Not a league: the nation_awards.cpp
/// 468 box (no competition) or 475 box (not a league). Defect of the original: the record's score dwords are never
/// written, so the history keeps 12 bytes of stack garbage per award.
/// Original: `nation_awards_award_manager_of_year` (0x768940, nation_awards.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_manager_of_year(_this: usize, today: usize, award: usize, comp_index: u32) {
    nation_awards_award_manager_of_year_with(today, award, comp_index, &mut |award, record| record_year_award(award, record));
}

/// The team of the season of a league (see `nation_awards_award_team_of_season`), with the final record going to
/// `sink`.
pub(crate) unsafe fn nation_awards_award_team_of_season_with(comp_index: u32, award: usize, sink: TeamAwardSink) {
    let mut share: f32 = f32::from_bits(0x3f28f5c3); // 0.66 of the league's games
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() == 1 {
        tel26_error_box(NATION_AWARDS_CPP, 0x658);
        return;
    }
    let games = if league.games_per_club() != -1 { league.games_per_club() } else { award_league_games(league) };
    let mut n = 0i32;
    let rows = exe::comp_stats_get_sorted_player_table(league.comp_stats(), 0x11, &mut n, -1i32 as u32, u32::MAX);
    if n == 0 {
        return;
    }
    // the substitutes the award names: those of its club competition, 3 without one
    let mut substitutes = 3u8;
    let table = read_usize(G_COMP_OBJECT_TABLE);
    let club_comp = read_usize(award + 9);
    if table != 0 && club_comp != 0 {
        let comp = League(read_usize(table + read_u32(club_comp) as usize * 4));
        if !comp.is_null() {
            substitutes = comp.substitutes_named();
        }
    }
    write_u8(award + TEAM_AWARD_SUBSTITUTES_OFFSET, substitutes);
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEAM_AWARD_CLEAR_SELECTION);
    clear(award);
    let add: unsafe extern "thiscall" fn(usize, usize, usize, f32, u32) -> u32 = core::mem::transmute(FN_TEAM_AWARD_ADD_CANDIDATE);
    'passes: loop {
        let least = ftol(games as i32 as f32 as f64 * share as f64) as i16 as i32;
        for k in 0..n.max(0) as usize {
            let row = rows + k * comp_stats_obj::PLAYER_ENTRY_SIZE;
            let person = staff_at(read_i32(row + comp_stats_player::STAFF_ID) as isize);
            let appearances = read_u8(row + ROW_APPEARANCES) as i32;
            let starts = appearances - read_u8(row + ROW_SUB_APPEARANCES) as i32;
            if starts <= least { // sx04_2: more starts than the share
                continue;
            }
            let rating = (read_i16(row + ROW_RATING_SUM) as i32 as f64 / appearances as f64) as f32;
            covered!("sx04_2 team of the season candidate");
            if add(award, person.addr(), person.club_job().addr(), rating, 0) != 0 {
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

/// Picks a league's team of the season: the award's substitutes (+0x253) become those its club competition names (3
/// without one) and the selection is cleared; then, for shares of the league's games from 0.66 down by 0.1 while above
/// 0 (the games per club, or (club count − 1) × legs when not worked out), each player of the league's rating table
/// (statistic 0x11) who started more games than that share is offered to the award with his average rating (rating
/// sum / appearances), until the award reports the team full. The table is freed, the season dates set for the
/// current year and the team recorded (`TEAM_AWARD::record_winners`: history, reputations). A missing competition or a
/// cup: the nation_awards.cpp 1624 box. No rated player: nothing (the selection is kept).
/// Original: `NationAwards::award_team_of_season` (0x7698f0, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_team_of_season(_this: usize, comp_index: u32, award: usize) {
    nation_awards_award_team_of_season_with(comp_index, award, &mut |award| record_team_award(award));
}

/// The newest record (`size` bytes) of a history file list between the offsets at `first` and `last`, if any.
unsafe fn newest_history_record(list: usize, first: usize, last: usize, size: usize) -> Option<Vec<u8>> {
    let nodes = exe::file_llist_get_nodes(list, read_u32(first), read_u32(last));
    if nodes == 0 {
        return None;
    }
    write_usize(nodes + llist::CURSOR, read_usize(nodes + llist::TAIL));
    let d = exe::llist_get_prev(nodes);
    let r = if d != 0 { Some((0..size).map(|i| read_u8(d + i)).collect()) } else { None };
    exe::llist_scalar_deleting_destructor(nodes, 1);
    r
}

/// Checks an award routine against its dry run: `planned` is the record the copy would give `record_winners` (none:
/// it would record nothing). The original runs; then either the award's history offsets did not move, or the newest
/// record of the history `list` is the planned one, except the `ignored` byte ranges.
unsafe fn compare_award_record(name: &str, list: usize, first: usize, last: usize, planned: Option<Vec<u8>>, ignored: &[(usize, usize)], original: impl FnOnce()) {
    let before = (read_u32(first), read_u32(last));
    original();
    let after = (read_u32(first), read_u32(last));
    let (ok, got) = match &planned {
        None => (after == before, None),
        Some(p) => {
            let got = newest_history_record(list, first, last, p.len());
            let same = got.as_ref().map_or(false, |g| (0..p.len()).all(|i| ignored.iter().any(|&(o, l)| i >= o && i < o + l) || g[i] == p[i]));
            (after != before && same, got)
        }
    };
    SCOREBOARD.record(ok, || format!("{}: history offsets {:x?} -> {:x?}, rust planned {:02x?}, original recorded {:02x?}", name, before, after, planned, got));
}

/// Comparer for `nation_awards_award_player_of_year`: the copy runs as a dry run (no record), then the original; its
/// newest year award history record must be the planned one. The error box: only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_player_of_year(this: usize, comp_index: u32, today: usize, award: usize, age: u32,
    positions: u32, stat: u32, nation: i32, club_nation: i32, not_nation: i32, home_nation: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, u32, u32, i32, i32, i32, i32)>(IDX_NATION_AWARDS_AWARD_PLAYER_OF_YEAR);
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() == 1 {
        return o(this, comp_index, today, award, age, positions, stat, nation, club_nation, not_nation, home_nation);
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_player_of_year_with(comp_index, today, award, age, positions, stat, nation, club_nation,
        not_nation, home_nation, &mut |_, r| planned = Some(r.to_vec())));
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_player_of_year", list, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned, &[],
        || o(this, comp_index, today, award, age, positions, stat, nation, club_nation, not_nation, home_nation));
}

/// Comparer for `nation_awards_award_manager_of_year`: as the player award's, without the score dwords the original
/// never writes. The error boxes: only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_manager_of_year(this: usize, today: usize, award: usize, comp_index: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_NATION_AWARDS_AWARD_MANAGER_OF_YEAR);
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() != 2 {
        return o(this, today, award, comp_index);
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_manager_of_year_with(today, award, comp_index, &mut |_, r| planned = Some(r.to_vec())));
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    let garbage: Vec<(usize, usize)> = (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect();
    compare_award_record("nation_awards_award_manager_of_year", list, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned, &garbage,
        || o(this, today, award, comp_index));
}

/// Comparer for `nation_awards_award_team_of_season`: the copy runs as a dry run on the award's selection (kept, then
/// put back), then the original; its newest team award history record must be the copy's selection. The error box:
/// only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_team_of_season(this: usize, comp_index: u32, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_NATION_AWARDS_AWARD_TEAM_OF_SEASON);
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() == 1 {
        return o(this, comp_index, award);
    }
    let selection = [(award + TEAM_AWARD_RECORD, TEAM_AWARD_SUBSTITUTES_OFFSET + 1 - TEAM_AWARD_RECORD)];
    let before = crate::comparison::MemorySnapshot::read(&selection);
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_team_of_season_with(comp_index, award,
        &mut |a| planned = Some((0..TEAM_RECORD_SIZE).map(|i| read_u8(a + TEAM_AWARD_RECORD + i)).collect::<Vec<u8>>())));
    before.write_back();
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::TEAM_AWARD_FILE);
    compare_award_record("nation_awards_award_team_of_season", list, award + TEAM_AWARD_HISTORY_FIRST, award + TEAM_AWARD_HISTORY_LAST, planned, &[],
        || o(this, comp_index, award));
}

/// `database_compare_staff_by_birth_date(a, b)`: qsort comparison of person pointers by date of birth, oldest first,
/// then by the style-4 name ("surname, first name").
///
/// The birth year (`date_of_birth_year()`, unsigned) decides first: `b`'s later, -1; earlier, 1. Then the day of the
/// year (`date_of_birth()`, signed) the same way. Same date: the two names (`exe::database_format_staff_name` into
/// 0x7d0 bytes, style 4) through the case-insensitive text comparison (`compare_text_nocase`). A null person counts as
/// the default `Date` (day 40 of 1972, `exe::date_date_54a110`) and the empty name. The community map calls the order
/// "youngest first"; the disassembly sorts the oldest first. Used by the squad numbers
/// (`SQUAD_MANAGER::allocate_squad_number` and the squad number list of squad_manager.cpp).
/// Original: `database_compare_staff_by_birth_date` (0x540130, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_compare_staff_by_birth_date(a: usize, b: usize) -> i32 {
    let birth = |p: usize| {
        let s = Staff(read_usize(p));
        if s.is_null() { (40i16, 1972u16) } else { (s.date_of_birth(), s.date_of_birth_year()) } // the default Date
    };
    let ((day_a, year_a), (day_b, year_b)) = (birth(a), birth(b));
    if year_b != year_a {
        return if year_b > year_a { -1 } else { 1 };
    }
    if day_b != day_a {
        return if day_b > day_a { -1 } else { 1 };
    }
    let mut name_a = [0u8; 0x7d0];
    let mut name_b = [0u8; 0x7d0];
    let (pa, pb) = (read_usize(a), read_usize(b));
    if pa != 0 {
        exe::database_format_staff_name(pa, name_a.as_mut_ptr() as usize, 0x7d0, 4); // "surname, first name"
    }
    if pb != 0 {
        exe::database_format_staff_name(pb, name_b.as_mut_ptr() as usize, 0x7d0, 4);
    }
    compare_text_nocase(name_a.as_ptr(), name_b.as_ptr())
}

/// Comparer for `database_compare_staff_by_birth_date`: the order of the original and of the copy.
pub(crate) unsafe extern "C" fn compare_database_compare_staff_by_birth_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_STAFF_BY_BIRTH_DATE)(a, b);
    let n = database_compare_staff_by_birth_date(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_staff_by_birth_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Writes the date the new transfer rules come into force (31 August 2001) into `date` (8 bytes, `Date::Date`
/// `exe::date_date_549f70`(date, 31, 7, 2001, -1), months counted from 0).
unsafe fn set_transfer_rule_change_date(date: usize) {
    exe::date_date_549f70(date as *mut [u16; 4], 0x1f, 7, 0x7d1, u32::MAX); // 31 August 2001
}

/// `init_transfer_rule_change_date()`: static initialiser of the exe (runs in the CRT start-up, before WinMain, with
/// the detours already installed by the DLL load): sets `G_TRANSFER_RULE_CHANGE_DATE` to 31 August 2001 and registers
/// its destructor (`destroy_transfer_rule_change_date` 0x524d00, an empty body) with `exe::crt_atexit`.
///
/// The date is read by the daily game loop (the "new transfer system" news), the contracts signed before the change
/// (contract_manager, staff_contracts) and the contract screen.
/// Original: `init_transfer_rule_change_date` (0x524cd0, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn init_transfer_rule_change_date() {
    set_transfer_rule_change_date(G_TRANSFER_RULE_CHANGE_DATE);
    exe::crt_atexit(0x524d00usize); // destroy_transfer_rule_change_date: an empty body
}

/// Comparer for `init_transfer_rule_change_date`: the original runs (it registers the destructor once); the copy writes
/// the date into a local and its 8 bytes are compared with the global's.
pub(crate) unsafe extern "C" fn compare_init_transfer_rule_change_date() {
    original_fn::<unsafe extern "C" fn()>(IDX_INIT_TRANSFER_RULE_CHANGE_DATE)();
    let mut copy = [0u16; 4];
    set_transfer_rule_change_date(copy.as_mut_ptr() as usize);
    let original: [u16; 4] = core::array::from_fn(|k| read_u16(G_TRANSFER_RULE_CHANGE_DATE + 2 * k));
    SCOREBOARD.record(copy == original, || format!("init_transfer_rule_change_date: original={:x?} rust={:x?}", original, copy));
}

/// `staff_add_disliked_staff(person, other, force)`: puts `other` among the person's three disliked staff (preferences
/// `staff_preferences()`: `preferences::STAFF_DISLIKED_STAFF1`..+0x30).
///
/// Nothing when `other` is the person, or with a null person, preferences or other. Already disliked: nothing; one of
/// the three favoured staff (`preferences::STAFF_FAVOURITE_STAFF1`..+0x24): stops being one and the routine stops;
/// otherwise the first free disliked slot; with no free slot, with `force`, a random slot (`random`, 3)
/// (`database_add_to_preference_list`). Used after the match (the media's ratings of a player, the fights and the
/// reserves' complaints), by the injuries and the transfers.
/// Original: `staff_add_disliked_staff` (0x547af0, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_add_disliked_staff(person: usize, other: usize, force: u32) {
    let person = Staff(person);
    if person.addr() == other || person.is_null() {
        return;
    }
    let pref = person.staff_preferences();
    if pref == 0 || other == 0 {
        return;
    }
    let disliked = pref + preferences::STAFF_DISLIKED_STAFF1;
    database_add_to_preference_list(disliked, pref + preferences::STAFF_FAVOURITE_STAFF1, other, list3_contains(disliked, other), force)
}

/// Comparer for `staff_add_disliked_staff`: the favoured and disliked staff (24 bytes) and the random generator, side by
/// side.
pub(crate) unsafe extern "C" fn compare_staff_add_disliked_staff(person: usize, other: usize, force: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_STAFF_ADD_DISLIKED_STAFF);
    let pref = if person != 0 { Staff(person).staff_preferences() } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + preferences::STAFF_FAVOURITE_STAFF1 } else { 0 });
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, other, force),
        || crate::rng::without_counting(|| staff_add_disliked_staff(person, other, force)));
    SCOREBOARD.record(same, || format!("staff_add_disliked_staff({:x}, {:x}, {}) {:x?} x {:x?}", person, other, force, mo, mr));
}

/// `staff_add_favoured_club(person, club, force)`: puts the club among the person's three favoured clubs (preferences
/// `staff_preferences()`: `preferences::STAFF_FAVOURITE_CLUBS1`..+0xc).
///
/// Null person, preferences or club: nothing. The club already favours the person (`club::FAV_STAFF1`..+0xa3):
/// nothing; the club is one of the person's disliked clubs (`preferences::STAFF_DISLIKED_CLUBS1`..+0x18): it stops
/// being one and the routine stops; otherwise the first free favoured slot (without checking whether it is already
/// among the favoured, as in the original); with no free slot, with `force`, a random slot (`random`, 3)
/// (`database_add_to_preference_list`). Used after the match (the player pleased with his club) and by the transfers.
/// Original: `staff_add_favoured_club` (0x547d90, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_add_favoured_club(person: usize, club: usize, force: u32) {
    let person = Staff(person);
    let club = Club(club);
    if person.is_null() {
        return;
    }
    let pref = person.staff_preferences();
    if pref == 0 || club.is_null() {
        return;
    }
    let club_favours_person = list3_contains(club.addr() + club::FAV_STAFF1, person.addr());
    database_add_to_preference_list(pref + preferences::STAFF_FAVOURITE_CLUBS1, pref + preferences::STAFF_DISLIKED_CLUBS1, club.addr(), club_favours_person, force)
}

/// Comparer for `staff_add_favoured_club`: the favoured and disliked clubs (24 bytes) and the random generator, side by
/// side.
pub(crate) unsafe extern "C" fn compare_staff_add_favoured_club(person: usize, club: usize, force: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_STAFF_ADD_FAVOURED_CLUB);
    let pref = if person != 0 { Staff(person).staff_preferences() } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + preferences::STAFF_FAVOURITE_CLUBS1 } else { 0 });
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, club, force),
        || crate::rng::without_counting(|| staff_add_favoured_club(person, club, force)));
    SCOREBOARD.record(same, || format!("staff_add_favoured_club({:x}, {:x}, {}) {:x?} x {:x?}", person, club, force, mo, mr));
}

/// The eight name tables of eng.lng in file order: (offset of the count in the header, offset of the row pointer in
/// the table object, row size). The counts are the 8 header dwords at +0..+0x1c; the rows go to +0x20..+0x3c.
const ENG_LNG_TABLES: [(usize, usize, u32); 8] = [
    (0x0, english_name_tables::CLUB_ROWS, 0x4f),        // clubs, 79 bytes
    (0x4, 0x24, 0xba),                                  // continents, 186 bytes
    (0x8, english_name_tables::NATION_COMP_ROWS, 0x53), // nation competitions, 83 bytes
    (0xc, english_name_tables::NATION_ROWS, 0x6d),      // nations, 109 bytes
    (0x18, english_name_tables::STADIUM_ROWS, 0x34),    // stadiums, 52 bytes
    (0x10, 0x30, 0x34),                                 // cities, 52 bytes
    (0x1c, english_name_tables::STAFF_COMP_ROWS, 0x4f), // staff competitions, 79 bytes
    (0x14, english_name_tables::CLUB_COMP_ROWS, 0x53),  // club competitions, 83 bytes
];

/// True when the eng.lng header counts match the loaded tables: continents, nations, cities, stadiums and staff
/// competitions one by one, and the club plus nation competitions against [`G_CLUB_COMP_COUNT`] − 127; the club
/// count (+0) is not checked.
unsafe fn eng_lng_counts_match(t: usize) -> bool {
    read_u32(t + 4) == read_u32(G_CONTINENT_COUNT)
        && read_u32(t + 0xc) == read_u32(G_NATION_COUNT)
        && read_u32(t + 0x10) == read_u32(G_CITY_COUNT)
        && read_u32(t + 8).wrapping_add(read_u32(t + 0x14)) == read_u32(G_CLUB_COMP_COUNT).wrapping_sub(0x7f)
        && read_u32(t + 0x18) == read_u32(G_STADIUM_COUNT)
        && read_u32(t + 0x1c) == read_u32(0xae2388) // staff competition count
}

/// `eng_lng_load_tables(table)` (thiscall): opens CM3_TEMP\eng.lng (`exe::utils_build_file_path`, `exe::crt_fopen`
/// "rb"); missing file: the Database.cpp error box (line 0x4dd2) and 0.
///
/// Reads the format marker (`exe::utils_read_file_format_marker`: 0xf1fffff1 = big-endian file) and the 8-dword header
/// into +0..+0x1c of the table, byte-swapping each dword when big-endian. Counts that do not match the loaded tables
/// (`eng_lng_counts_match`): closes the file, error box (line 0x4df1) and 0. Otherwise, for each table in file order
/// (`ENG_LNG_TABLES`), `exe::crt_malloc`(count × row size) into its row pointer and, when it came, `exe::crt_fread` of
/// the rows (the row data is never swapped); closes the file and returns 1. Called once, on the first use of an
/// English name (`nation_get_english_name` and its sisters build the 0x40-byte object with the row pointers at zero
/// and keep it in [`G_ENGLISH_NAME_TABLES`]).
/// Original: `eng_lng_load_tables` (0x544fc0, Database.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn eng_lng_load_tables(t: usize) -> u32 {
    let path = exe::utils_build_file_path(0x987da0usize, 0x9b632cusize); // "CM3_TEMP", "eng.lng"
    let file = exe::crt_fopen(path, 0x98f928usize); // "rb"
    if file == 0 {
        database_error_box(0x4dd2);
        return 0;
    }
    let big_endian = exe::utils_read_file_format_marker(file) == 0xf1fffff1;
    exe::crt_fread(t, 0x20, 1, file);
    if big_endian {
        for k in 0..8 {
            write_u32(t + 4 * k, read_u32(t + 4 * k).swap_bytes());
        }
    }
    if !eng_lng_counts_match(t) {
        exe::crt_fclose(file);
        database_error_box(0x4df1);
        return 0;
    }
    for (count, rows, size) in ENG_LNG_TABLES {
        let n = read_u32(t + count);
        let p = exe::crt_malloc(n.wrapping_mul(size));
        write_usize(t + rows, p);
        if p != 0 {
            exe::crt_fread(p, size, n, file);
        }
    }
    exe::crt_fclose(file);
    1
}

/// Comparer for `eng_lng_load_tables`: the original loads into the game's object; the copy loads into a local
/// 0x40-byte object (row pointers at zero, as the callers build it). The answer, the header and each table's rows
/// must match; then the copy's rows are freed (`eng_lng_free_tables`).
pub(crate) unsafe extern "thiscall" fn compare_eng_lng_load_tables(t: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_LNG_LOAD_TABLES)(t);
    let mut copy = [0u8; 0x40];
    let c = copy.as_mut_ptr() as usize;
    let n = eng_lng_load_tables(c);
    let rows_equal = |count: usize, rows: usize, size: u32| {
        let (po, pc) = (read_usize(t + rows), read_usize(c + rows));
        let len = read_u32(t + count).wrapping_mul(size) as usize;
        (po == 0) == (pc == 0)
            && (po == 0 || core::slice::from_raw_parts(po as *const u8, len) == core::slice::from_raw_parts(pc as *const u8, len))
    };
    // without the file nothing is read: the game's header is still the garbage of `new`
    let header_equal = r == 0 || (0..0x20).all(|k| read_u8(t + k) == copy[k]);
    let tables_equal = r == 0 || ENG_LNG_TABLES.iter().all(|&(count, rows, size)| rows_equal(count, rows, size));
    SCOREBOARD.record(r == n && header_equal && tables_equal,
        || format!("eng_lng_load_tables: original={} rust={} header {} tables {}", r, n, header_equal, tables_equal));
    eng_lng_free_tables(c);
    r
}

// The Shortlist_Manager constructor (shortlist_manager.cpp, thiscall(this, load_stream), ret 4, returns this) and the
// transfer manager's own load from the saved game (thiscall(this, stream), ret 4, non-zero when loaded).
const FN_SHORTLIST_MANAGER_CONSTRUCTOR: usize = 0x839a80;
const FN_TRANSFER_MANAGER_LOAD_FROM_DISK: usize = 0x8bc020;

/// The 27 national transfer rules objects after the European one, in the order the original builds them: (slot in
/// `transfer_manager_fields::TRANSFER_RULES`, rules id, constructor, nation key). Slots 0x10 and 0x11 are built in
/// the opposite order (Norway before Northern Ireland). Each constructor is thiscall(this, id, nation, −1, 1, 0), ret
/// 0x14.
const NATIONAL_TRANSFER_RULES: [(usize, u32, usize, usize); 27] = [
    (1, 0x1, 0x40a580, KEY_NATION_ARGENTINA),
    (2, 0x2, 0x412c80, KEY_NATION_AUSTRALIA),
    (3, 0x3, 0x424b90, KEY_NATION_BELGIUM),
    (4, 0x4, 0x43f0e0, KEY_NATION_BRAZIL),
    (5, 0x5, 0x518500, KEY_NATION_CROATIA),
    (6, 0x6, 0x555460, KEY_NATION_DENMARK),
    (7, 0x7, 0x579b30, KEY_NATION_ENGLAND),
    (8, 0x8, 0x5a9950, KEY_NATION_FINLAND),
    (9, 0x9, 0x5c1570, KEY_NATION_FRANCE),
    (10, 0xa, 0x5dff70, KEY_NATION_GERMANY),
    (11, 0xb, 0x5eceb0, KEY_NATION_GREECE),
    (12, 0xc, 0x5f9690, KEY_NATION_HOLLAND),
    (13, 0xd, 0x63c210, KEY_NATION_REPUBLIC_OF_IRELAND),
    (14, 0xe, 0x661380, KEY_NATION_ITALY),
    (15, 0xf, 0x666bd0, KEY_NATION_JAPAN),
    (17, 0x11, 0x7944f0, KEY_NATION_NORWAY),
    (16, 0x10, 0x7936d0, KEY_NATION_NORTHERN_IRELAND),
    (18, 0x12, 0x7cd830, KEY_NATION_POLAND),
    (19, 0x13, 0x7d3d80, KEY_NATION_PORTUGAL),
    (20, 0x14, 0x7ec770, KEY_NATION_RUSSIA),
    (21, 0x15, 0x7f6390, KEY_NATION_SCOTLAND),
    (22, 0x16, 0x859400, KEY_NATION_SPAIN),
    (23, 0x17, 0x893d80, KEY_NATION_SWEDEN),
    (24, 0x18, 0x9020a0, KEY_NATION_TURKEY),
    (25, 0x19, 0x90b940, KEY_NATION_UNITED_STATES),
    (26, 0x1a, 0x90f080, KEY_NATION_WALES),
    (27, 0x1b, 0x66e630, KEY_NATION_SOUTH_KOREA),
];

/// The European rules constructor (slot 0): thiscall(this, 0, −1, continent, 2, 0), ret 0x14.
const FN_EUROPEAN_TRANSFER_RULES_CONSTRUCTOR: usize = 0x583280;

/// Size of a transfer rules object (`operator new` of each one).
const TRANSFER_RULES_OBJECT_SIZE: u32 = 0x19;

/// The CD search of a constructor: among the drives (GetLogicalDriveStringsA, 200 bytes) the first CD drive
/// (GetDriveTypeA = 5, always asked) holding "<root>cm0102.exe" (`exe::file_find_first`; on the others the search is
/// closed, `exe::file_find_close`); none: the "Insert CD" box (langlib_sprintf of 0x987df4 and 0x987dc4, the box with
/// 1) and again, forever. A no-CD exe (as cm0102-1080) patched this copy of the check: the `jne` on the drive type at
/// `drive_type_jump` made nops and the found test at `found_test` (85 c0) made xor; the copy reads those bytes of the
/// running exe.
unsafe fn find_game_cd(drive_type_jump: usize, found_test: usize) {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8)); // GetLogicalDriveStringsA
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100)); // GetDriveTypeA
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(drive_type_jump) == 0x90;
    let always_found = read_u8(found_test) != 0x85;
    loop {
        // simplification: the buffer starts zeroed; the original would only read garbage with more than 200 bytes of drives
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    let nm = name.as_mut_ptr() as usize;
                    format(nm, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let f = found.as_mut_ptr() as usize;
                    if exe::file_find_first(nm, f, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(f);
                }
                // simplification: the original walks with an i8 (steps of 4): it would only pass 127 with more than 32 drives
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        let (mut title, mut msg) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(msg.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }
}

/// Builds the transfer manager, for a new game (`stream` 0) or from the saved game.
///
/// - The 150 day buckets at +0x18 (LLISTs of 14 bytes, `exe::crt_vector_constructor_iterator`), the date
/// `current_date()` (`exe::date_date_54a110`) and the inline lists `INSTALMENTS` and `FREE_TRANSFER_QUEUE`
/// (`exe::new_llist`).
/// - The CD check (`find_game_cd`, the no-CD patch read at 0x8bbaf2 and 0x8bbb34).
/// - The offers and negotiations (+0..+0x14), the past transfers, the next-sale percentages, both appearance money
/// arrays, the future transfers, the work permits (pointers, capacities and counts) and the loan negotiations zeroed;
/// `day_cursor()` 0; `current_date()` = a date built with 0, 0 (`exe::new_date`, `exe::date_operator_3d`);
/// `current_slice()` 0.
/// - The transfer rules (`setup_transfer_rules`).
/// - `person_flags()` = calloc(`G_STAFF_COUNT`, 8) and `season_loan_counts()` = calloc(`G_CLUB_COUNT`, 16); either
/// failing: the "Memory Allocation Error" box (lines 0xa3 and 0xab) and the construction stops there.
/// - `lowest_free_slot()` = −1.
/// - With a `stream`, the manager is read from the saved game (`FN_TRANSFER_MANAGER_LOAD_FROM_DISK`); when that
/// works, nothing else. Otherwise a new Shortlist_Manager (`exe::crt_operator_new` of 0xe0 bytes, its constructor with
/// 0) in `shortlist_manager()`; none: the box (line 0xc0) and stop; then the work permits (`init_work_permits`).
///
/// Returns this.
/// Original: `TRANSFER_MANAGER::TRANSFER_MANAGER` (0x8bba30, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_transfer_manager(this: usize, stream: usize) -> usize {
    let tm = TransferManagerFields(this);
    exe::crt_vector_constructor_iterator(this + 0x18, 0xe, 0x96, FN_LLIST_LLIST, FN_LLIST_DESTRUCTOR_LLIST); // the day buckets
    exe::date_date_54a110(this + transfer_manager_fields::CURRENT_DATE);
    exe::new_llist(this + transfer_manager_fields::INSTALMENTS);
    exe::new_llist(this + transfer_manager_fields::FREE_TRANSFER_QUEUE);
    find_game_cd(0x8bbaf2, 0x8bbb34);
    for field in [0x0usize, 0x8, 0x4, 0xc, 0x14, 0x10, // the offer and negotiation arrays
                  transfer_manager_fields::PAST_TRANSFERS, transfer_manager_fields::PAST_TRANSFER_COUNT, 0x866, // 0x866: past transfer capacity
                  transfer_manager_fields::NEXT_SALE_PERCENTAGES, transfer_manager_fields::NEXT_SALE_PERCENTAGE_COUNT, 0x884, // 0x884: their capacity
                  transfer_manager_fields::APPEARANCE_MONEY, transfer_manager_fields::APPEARANCE_MONEY_COUNT, 0x890, // 0x890: their capacity
                  transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY, transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY_COUNT, 0x89c, // 0x89c: their capacity
                  transfer_manager_fields::FUTURE_TRANSFERS, transfer_manager_fields::FUTURE_TRANSFER_COUNT, 0x924, // 0x924: their capacity
                  transfer_manager_fields::WORK_PERMITS, transfer_manager_fields::WORK_PERMIT_COUNT, 0x942, // 0x942: their capacity
                  transfer_manager_fields::LOAN_NEGOTIATIONS] {
        write_u32(this + field, 0);
    }
    tm.set_day_cursor(0);
    let (mut start, mut result) = ([0u8; 8], [0u8; 8]);
    let d = exe::new_date(start.as_mut_ptr() as usize, 0, 0);
    exe::date_operator_3d(this + transfer_manager_fields::CURRENT_DATE, result.as_mut_ptr() as usize, d);
    tm.set_current_slice(0);
    setup_transfer_rules(this);
    let flags = exe::crt_calloc(read_u32(G_STAFF_COUNT), 8);
    write_usize(this + transfer_manager_fields::PERSON_FLAGS, flags);
    if flags == 0 {
        show_out_of_memory_box(0xa3, 0xa8d4cc); // "TRANSFER_MANAGER::constructor() : cannot malloc staff information array"
        return this;
    }
    let loans = exe::crt_calloc(read_u32(G_CLUB_COUNT), 0x10);
    write_usize(this + transfer_manager_fields::SEASON_LOAN_COUNTS, loans);
    if loans == 0 {
        show_out_of_memory_box(0xab, 0xa8d484); // "TRANSFER_MANAGER::constructor() : cannot malloc club information array"
        return this;
    }
    write_i32(this + transfer_manager_fields::LOWEST_FREE_SLOT, -1);
    if stream != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_LOAD_FROM_DISK);
        if load(this, stream) != 0 {
            covered!("sx02b_1 loaded from the saved game");
            return this;
        }
    }
    covered!("sx02b_1 new transfer manager");
    let mut shortlist = exe::crt_operator_new(0xe0);
    if shortlist != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_CONSTRUCTOR);
        shortlist = construct(shortlist, 0);
    }
    write_usize(this + transfer_manager_fields::SHORTLIST_MANAGER, shortlist);
    if shortlist == 0 {
        show_out_of_memory_box(0xc0, 0xa8d440); // "TRANSFER_MANAGER::constructor() : cannot create shortlist manager"
        return this;
    }
    init_work_permits(this); // sx02b_1: the work permits of a new manager
    this
}

/// Comparer for `construct_transfer_manager`: only the original runs (it builds the rules objects and the shortlist
/// manager, which cannot be built twice).
pub(crate) unsafe extern "thiscall" fn compare_construct_transfer_manager(this: usize, stream: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_TRANSFER_MANAGER)(this, stream) // simplification: original only
}

/// The transfer window dates of the manager's rules objects, in rules order: for each of the 28 objects (a null one
/// skipped) and each of its windows (signed byte +8; 12-byte entries at the pointer +4), the opening date (entry +0)
/// and the closing one (entry +6).
unsafe fn collect_transfer_window_dates(this: usize) -> Vec<usize> {
    let mut dates = Vec::new();
    for k in 0..0x1c {
        let rules = transfer_rules_at(TransferManagerFields(this), k);
        if rules == 0 {
            continue;
        }
        // simplification: a negative count (never in the data) would lower the original's total and add no date
        for c in 0..read_i8(rules + 8).max(0) as usize { // +8: window count
            let window = read_usize(rules + 4) + c * 0xc; // +4: the window entries
            dates.push(window);
            dates.push(window + 6); // sx02b_1: the closing date
        }
    }
    dates
}

/// Builds the 28 transfer rules objects and the sorted table of all their window dates.
///
/// - Each object (`exe::crt_operator_new` of 0x19 bytes; when it fails the slot stays 0 and nothing is built): slot 0
/// the European rules (`FN_EUROPEAN_TRANSFER_RULES_CONSTRUCTOR` with 0, −1, `KEY_CONTINENT_EUROPE`, 2, 0), then the
/// 27 national ones of `NATIONAL_TRANSFER_RULES` (id, the nation, −1, 1, 0), in that order.
/// - `transfer_window_count()` = the sum of 2 × the window count (signed byte +8) of each object; `transfer_windows()`
/// = malloc of 4 × that (none: the "Memory Allocation Error" box, line 0x3bdf, and nothing more), filled with the
/// opening and closing date of each window (`collect_transfer_window_dates`) and sorted by month, then day
/// (`exe::crt_qsort` with 0x8e5590).
/// Original: `TRANSFER_MANAGER::setup_transfer_rules` (0x8d26b0, transfer_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn setup_transfer_rules(this: usize) {
    let slot = |k: usize| this + transfer_manager_fields::TRANSFER_RULES + 4 * k;
    let mut rules = exe::crt_operator_new(TRANSFER_RULES_OBJECT_SIZE);
    if rules != 0 {
        let construct: unsafe extern "thiscall" fn(usize, u32, i32, usize, u32, u32) -> usize = core::mem::transmute(FN_EUROPEAN_TRANSFER_RULES_CONSTRUCTOR);
        rules = construct(rules, 0, -1, read_usize(KEY_CONTINENT_EUROPE), 2, 0);
    }
    write_usize(slot(0), rules);
    for &(k, id, constructor, nation) in NATIONAL_TRANSFER_RULES.iter() {
        let mut rules = exe::crt_operator_new(TRANSFER_RULES_OBJECT_SIZE);
        if rules != 0 {
            let construct: unsafe extern "thiscall" fn(usize, u32, usize, i32, u32, u32) -> usize = core::mem::transmute(constructor);
            rules = construct(rules, id, read_usize(nation), -1, 1, 0); // sx02b_1: a national rules object
        }
        write_usize(slot(k), rules);
    }
    let dates = collect_transfer_window_dates(this);
    write_usize(this + transfer_manager_fields::TRANSFER_WINDOW_COUNT, dates.len());
    let table = exe::crt_malloc((dates.len() * 4) as u32);
    write_usize(this + transfer_manager_fields::TRANSFER_WINDOWS, table);
    if table == 0 {
        show_out_of_memory_box(0x3bdf, 0xa8f530); // "TRANSFER_MANAGER::setup_transfer_rules() : cannot malloc deadline dates"
        return;
    }
    core::ptr::copy_nonoverlapping(dates.as_ptr(), table as *mut usize, dates.len());
    exe::crt_qsort(table, dates.len() as u32, 4, 0x8e5590usize);
}

/// Comparer for `setup_transfer_rules`: only the original runs (it allocates the rules objects); then the window
/// table is rebuilt from its objects (`collect_transfer_window_dates`, sorted with the same qsort) and checked against
/// the original's, and each national object's nation (`transfer_rules::NATION_ID`, the nation's record: the base
/// constructor 0x8e84a0 stores `G_NATION_TABLE + id × nation::SIZE` for the id its key holds) against its key.
pub(crate) unsafe extern "thiscall" fn compare_setup_transfer_rules(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SETUP_TRANSFER_RULES)(this);
    let tm = TransferManagerFields(this);
    let mut plan = collect_transfer_window_dates(this);
    if !plan.is_empty() {
        exe::crt_qsort(plan.as_mut_ptr() as usize, plan.len() as u32, 4, 0x8e5590usize);
    }
    let n = tm.transfer_window_count();
    let same_table = n == plan.len() && tm.transfer_windows() != 0
        && core::slice::from_raw_parts(tm.transfer_windows() as *const usize, n) == plan.as_slice();
    let wrong_nation = NATIONAL_TRANSFER_RULES.iter().find(|&&(k, _, _, nation)| {
        let rules = transfer_rules_at(tm, k);
        rules != 0 && read_usize(rules + transfer_rules::NATION_ID) != read_usize(G_NATION_TABLE) + read_usize(nation) * nation::SIZE
    });
    SCOREBOARD.record(same_table && wrong_nation.is_none(), || format!(
        "setup_transfer_rules: {} window dates (plan {}), table equal {}, rules slot with another nation {:?}",
        n, plan.len(), same_table, wrong_nation.map(|r| r.0)));
}

/// How many past transfer records the manager has before `cutoff`: they are in date order, so the count of the leading
/// records dated before it (`transfer_manager_date_before`: year unsigned, then day signed).
unsafe fn count_past_transfers_before(this: TransferManagerFields, cutoff: usize) -> usize {
    (0..this.past_transfer_count())
        .take_while(|&i| transfer_manager_date_before(this.past_transfers() + i * PAST_TRANSFER_SIZE + past_transfer::DATE, cutoff))
        .count()
}

/// Moves the past transfers older than a year to the yearly files and shrinks the array.
///
/// - The cut-off is today (`G_CURRENT_DATE`) less 52 weeks (`exe::date_sub_weeks`). The records in front dated before
/// it (`count_past_transfers_before`) are appended (`exe::crt_fwrite` of 37 bytes) to `transfers_<year>.tmp` in
/// CM3_TEMP (the name sprintf'ed into 0xde1f64, `exe::utils_build_file_path`, `exe::crt_fopen` "ab"), one file open at
/// a time, closed when the year changes and at the end.
/// - A file that does not open: the "Error" box (line 0x1332); a write that fails: the "Error" box (line 0x133c).
/// - With records moved: a new array (malloc of 37 × (left + 100); none: the "Memory Allocation Error" box, line
/// 0x135d, and the array stays as it was) gets the records left, the old one is freed; count = left, capacity = left +
/// 100.
///
/// Defects of the original: after a file fails to open, the other records of that year count as moved without being
/// written anywhere (they are lost); a record whose write fails is not counted, so the array keeps as many of the
/// last written ones (they show twice later) and the failed one is lost; when the new array cannot be allocated, the
/// records already appended to the files also stay in memory (they show twice later).
/// Original: `TRANSFER_MANAGER::clear_old_past_transfers` (0x8c3eb0, transfer_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn clear_old_past_transfers(this: usize) {
    let tm = TransferManagerFields(this);
    covered!("sx02b_2 clear old transfers");
    let mut cutoff = [0u8; 8];
    exe::date_sub_weeks(G_CURRENT_DATE, &mut cutoff, 0x34);
    let old = count_past_transfers_before(tm, cutoff.as_ptr() as usize);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut file, mut year_open, mut moved) = (0usize, 0u16, 0usize);
    for i in 0..old {
        let r = tm.past_transfers() + i * PAST_TRANSFER_SIZE;
        let year = read_u16(r + past_transfer::DATE + cm_date::YEAR);
        if year != year_open {
            if file != 0 {
                exe::crt_fclose(file);
            }
            year_open = year;
            format(0xde1f64, 0xa8ea38, year as u32); // "transfers_%d.tmp"
            file = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0xde1f64), 0x9c79b0); // "CM3_TEMP", "ab"
            if file == 0 {
                show_error_box(0xa8d514, 0x1332);
                continue;
            }
            covered!("sx02b_2 yearly file");
        } else if file == 0 {
            moved += 1; // the original counts it without a file
            continue;
        }
        if exe::crt_fwrite(r, PAST_TRANSFER_SIZE as u32, 1, file) == 1 {
            moved += 1; // sx02b_2: a record moved to its year's file
        } else {
            show_error_box(0xa8d514, 0x133c);
        }
    }
    if file != 0 {
        exe::crt_fclose(file);
    }
    if moved == 0 {
        return;
    }
    let left = tm.past_transfer_count() - moved;
    let capacity = left + 100;
    let array = exe::crt_malloc((capacity * PAST_TRANSFER_SIZE) as u32);
    if array == 0 {
        show_out_of_memory_box(0x135d, 0xa8e9dc); // "TRANSFER_MANAGER::clear old_past_transfers() : could not malloc new past transfers array"
        return;
    }
    core::ptr::copy_nonoverlapping((tm.past_transfers() + moved * PAST_TRANSFER_SIZE) as *const u8, array as *mut u8, left * PAST_TRANSFER_SIZE);
    exe::crt_free(tm.past_transfers());
    write_usize(this + transfer_manager_fields::PAST_TRANSFERS, array);
    write_usize(this + transfer_manager_fields::PAST_TRANSFER_COUNT, left);
    write_usize(this + 0x866, capacity); // 0x866: past transfer capacity
}

/// Comparer for `clear_old_past_transfers`: only the original runs (it appends to the yearly files). The plan is made
/// beforehand (the records before the cut-off, `count_past_transfers_before`, and a copy of the array); afterwards the
/// count must have dropped by that much, the capacity be the count + 100 (when something moved) and the array hold the
/// records that were after them.
pub(crate) unsafe extern "thiscall" fn compare_clear_old_past_transfers(this: usize) {
    let tm = TransferManagerFields(this);
    let mut cutoff = [0u8; 8];
    exe::date_sub_weeks(G_CURRENT_DATE, &mut cutoff, 0x34);
    let old = count_past_transfers_before(tm, cutoff.as_ptr() as usize);
    let records = || {
        let n = tm.past_transfer_count();
        if n == 0 { Vec::new() } else { core::slice::from_raw_parts(tm.past_transfers() as *const u8, n * PAST_TRANSFER_SIZE).to_vec() }
    };
    let (n, before) = (tm.past_transfer_count(), records());
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CLEAR_OLD_PAST_TRANSFERS)(this);
    let (m, after) = (tm.past_transfer_count(), records());
    let same = m + old == n && (old == 0 || read_usize(this + 0x866) == m + 100) && after[..] == before[old * PAST_TRANSFER_SIZE..];
    SCOREBOARD.record(same, || format!("clear_old_past_transfers: {} records, {} left (plan {} moved)", n, m, old));
}

/// Cancels offer `o` because the manager of `club` left: status `offer_status::CANCELLED`, its contract offers closed
/// (`exe::transfer_manager_close_offer_contract_offers`), the bid markers cleared (the buyer's on the target and the
/// seller's, the club of the target's contract through `club_of_staff_contract`, on each exchange player:
/// `exe::shortlist_manager_clear_bid_markers_for_player`), the target's contract offer from the buyer removed
/// (`exe::contract_manager_remove_contract_offer`), `terms_state()` 0, each exchange player's contract offer for the
/// offer's slot removed (`exe::contract_manager_remove_contract_offer_for_transfer`) and his "terms offered" byte
/// cleared; due again in 10 days (`exe::transfer_manager_schedule_offer`) and the collapse news with reason 0xb
/// (`exe::transfer_manager_post_offer_collapsed_news`).
unsafe fn cancel_offer_of_departed_manager(this: TransferManagerFields, o: Offer, club: usize) {
    o.set_status(offer_status::CANCELLED); // sx02b_2: an offer cancelled
    exe::transfer_manager_close_offer_contract_offers(this, o);
    let target = transfer_manager_staff_by_id(o.target_staff_id());
    let seller = club_of_staff_contract(target);
    let buyer = transfer_manager_club_by_id(o.buyer_club_id());
    exe::shortlist_manager_clear_bid_markers_for_player(this.shortlist_manager(), buyer, target);
    let exchange = o.exchange_players();
    if !exchange.is_null() {
        exchange.set_cursor(exchange.head());
        loop {
            let x = exe::llist_get_next(exchange);
            if x == 0 {
                break;
            }
            exe::shortlist_manager_clear_bid_markers_for_player(this.shortlist_manager(), seller, transfer_manager_staff_by_id(read_usize(x)));
        }
    }
    exe::contract_manager_remove_contract_offer(read_usize(G_CONTRACT_MANAGER), target, buyer);
    o.set_terms_state(0);
    let exchange = o.exchange_players();
    if !exchange.is_null() {
        exchange.set_cursor(exchange.head());
        loop {
            let x = exe::llist_get_next(exchange);
            if x == 0 {
                break;
            }
            exe::contract_manager_remove_contract_offer_for_transfer(read_usize(G_CONTRACT_MANAGER), transfer_manager_staff_by_id(read_usize(x)), o.slot_index());
            write_u8(x + 4, 0); // exchange item +4: terms offered
        }
    }
    exe::transfer_manager_schedule_offer(this, o, 10, 0);
    exe::transfer_manager_post_offer_collapsed_news(this, o, club, 0xb);
}

/// Cancels the offers of a club whose manager leaves, and those for its players.
///
/// - A national team (`exe::database_is_national_team`): nothing.
/// - Each open offer (not `offer_is_closed`) the club made (`buyer_club_id()` = the club's id), by slot up to
/// `offer_capacity()`: unless it is a future transfer (`offer_status::FUTURE_TRANSFER`), it is cancelled
/// (`cancel_offer_of_departed_manager`).
/// - Then each person in the club's squad (`exe::database_collect_club_squad`, 50 slots; empty slots and ids from
/// `G_STAFF_COUNT` up skipped): his open offers (`exe::transfer_manager_get_offers_for_player` with 0) are appended to
/// one list, and every offer in that list that is not a future transfer nor a pre-contract (`pre_contract()`) is
/// cancelled.
///
/// Defect of the original: the list of the squad's offers is never emptied between players, so a player with open
/// offers makes the offers of the players before him go through again: cancelled once more, scheduled again and their
/// collapse news posted again.
/// Original: `TRANSFER_MANAGER::cancel_club_offers` (0x8c4950, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cancel_club_offers(this: usize, club: usize) {
    let tm = TransferManagerFields(this);
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let id = read_usize(club); // the club's id
    let mut found = 0;
    for k in 0..tm.offer_capacity() as i32 {
        let o = Offer(offer_at(this, k as usize));
        if !o.is_null() && !offer_is_closed(o) && o.buyer_club_id() == id {
            exe::llist_add_tail(l, &k as *const i32 as usize, 4);
            found += 1;
        }
    }
    if found != 0 {
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let o = Offer(offer_at(this, read_usize(e)));
            if o.status() != offer_status::FUTURE_TRANSFER {
                covered!("sx02b_2 the club's own offer");
                cancel_offer_of_departed_manager(tm, o, club); // sx02b_2: the club's own offer
            }
        }
    }
    exe::llist_clear(l);
    let mut squad = [0usize; 50];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 0);
    for &person in squad.iter() {
        if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if exe::transfer_manager_get_offers_for_player(this, person, l, 0) == 0 {
            continue;
        }
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let o = Offer(offer_at(this, read_usize(e)));
            if o.status() != offer_status::FUTURE_TRANSFER && o.pre_contract() == 0 {
                covered!("sx02b_2 an offer for a squad player");
                cancel_offer_of_departed_manager(tm, o, club); // sx02b_2: an offer for a squad player
            }
        }
    }
    exe::drop_llist(l);
}

/// Comparer for `cancel_club_offers`: only the original runs (it schedules offers, removes contract offers and posts
/// news, none of which can run twice).
pub(crate) unsafe extern "thiscall" fn compare_cancel_club_offers(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CANCEL_CLUB_OFFERS)(this, club) // simplification: original only
}

/// Orders two 13-byte next-sale percentage entries (`next_sale_percentages()`) for qsort and bsearch: by the person id
/// at +9, then by the club id at +1 (each as the difference of the two ints, wrapping).
/// Original: `transfer_manager_compare_next_sale_entries` (0x8e54d0, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_next_sale_entries(a: usize, b: usize) -> i32 {
    let d = read_i32(a + 9).wrapping_sub(read_i32(b + 9)); // sx02b_3: +9, the person id
    if d != 0 {
        return d;
    }
    read_i32(a + 1).wrapping_sub(read_i32(b + 1)) // +1: the club id
}

/// Comparer for `compare_next_sale_entries`: both run side by side (they only read).
pub(crate) unsafe extern "C" fn compare_compare_next_sale_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_NEXT_SALE_ENTRIES)(a, b);
    let n = compare_next_sale_entries(a, b);
    SCOREBOARD.record(r == n, || format!("compare_next_sale_entries original={} rust={}", r, n));
    r
}

/// Orders two transfer window dates through pointers to them (the table `transfer_windows()`): by the month byte
/// (`transfer_window::MONTH`), then the day byte (+3), each as the difference of the two signed bytes.
/// Original: `transfer_manager_compare_window_dates` (0x8e5590, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_window_dates(a: usize, b: usize) -> i32 {
    let (a, b) = (read_usize(a), read_usize(b));
    let (ma, mb) = (read_i8(a + transfer_window::MONTH), read_i8(b + transfer_window::MONTH));
    if ma != mb {
        return ma as i32 - mb as i32; // sx02b_3: by month
    }
    read_i8(a + 3) as i32 - read_i8(b + 3) as i32 // +3: the day
}

/// Comparer for `compare_window_dates`: both run side by side (they only read).
pub(crate) unsafe extern "C" fn compare_compare_window_dates(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_WINDOW_DATES)(a, b);
    let n = compare_window_dates(a, b);
    SCOREBOARD.record(r == n, || format!("compare_window_dates original={} rust={}", r, n));
    r
}

/// Orders two 14-byte work permit entries (`work_permits()`) by the person id at +0, with the empty key −1 last: both
/// −1, 0; only the first, 1; only the second, −1; otherwise the difference (wrapping).
/// Original: `transfer_manager_compare_work_permit_entries` (0x8e55c0, transfer_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_work_permit_entries(a: usize, b: usize) -> i32 {
    let (ka, kb) = (read_i32(a), read_i32(b));
    match (ka == -1, kb == -1) {
        (true, true) => 0,
        (true, false) => 1, // sx02b_3: the empty key last
        (false, true) => -1,
        (false, false) => ka.wrapping_sub(kb),
    }
}

/// Comparer for `compare_work_permit_entries`: both run side by side (they only read).
pub(crate) unsafe extern "C" fn compare_compare_work_permit_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_WORK_PERMIT_ENTRIES)(a, b);
    let n = compare_work_permit_entries(a, b);
    SCOREBOARD.record(r == n, || format!("compare_work_permit_entries original={} rust={}", r, n));
    r
}

// --- sx01c_1: Media::apply_story_effect_flags (0x736c00, media.cpp, 5,939 bytes), the whole routine in named pieces ---

/// The f32 constants of `apply_story_effect_flags` that have no name in `rdata_consts.rs` yet (cm0102.exe .rdata).
const STORY_F32_ONE_FIFTEENTH: usize = 0x96d888; // f32 = 1/15
const STORY_F32_ONE_150TH: usize = 0x96d884; // f32 = 1/150
const STORY_F32_30: usize = 0x96d880; // f32 = 30.0
/// `Media::queue_player_gratitude_news(player, club, kind, variant)` (0x732c80, media.cpp, thiscall, ret 0x10).
const FN_MEDIA_QUEUE_PLAYER_GRATITUDE_NEWS: usize = 0x732c80;
/// `Media::post_player_backing_request_news(player, club, kind, story)` (0x743370, media.cpp, thiscall, ret 0x10).
const FN_MEDIA_POST_PLAYER_BACKING_REQUEST_NEWS: usize = 0x743370;
/// `Media::post_player_criticism_reaction_news(player, club, topic, reply)` (0x743a50, media.cpp, thiscall, ret 0x10).
const FN_MEDIA_POST_PLAYER_CRITICISM_REACTION_NEWS: usize = 0x743a50;

/// What `apply_story_effect_flags` keeps in its frame while it walks the squad.
struct StoryEffects {
    this: usize,
    club: Club,
    kind: u32,
    flags: u32,
    weight: f32,
    player: Staff,
    /// the player's contract at the club
    contract: Contract,
    /// 0x13 when the story kind is 0x13 (a word, compared with 0, 0x13 and 0xa)
    story13: u16,
    /// the player may still apologise (cleared by a draw when he already answered a story)
    may_apologise: bool,
    /// set when the caller passed neither 0x80 nor 0x100 (the routine chose the reaction itself)
    chosen: bool,
    /// the player's form for the story (`exe::media_rate_player_form_for_story`)
    form: f32,
    /// what the manager expected of him; without a player, the bits of the response argument
    expected: f32,
}

impl StoryEffects {
    /// The form above the expectation by more than `margin`.
    fn above(&self, margin: f64) -> bool {
        (self.expected as f64 + margin) < self.form as f64
    }
    /// The form below the expectation by more than `margin`.
    fn below(&self, margin: f64) -> bool {
        (self.expected as f64 - margin) > self.form as f64
    }
}

/// The staff's confidence in the manager after a story: the club's `board_record::STAFF_CONFIDENCE` plus the change,
/// kept between 1 and 10000.
pub(crate) unsafe fn story_staff_confidence_after(club: Club, change: i16) -> i16 {
    read_i16(manager_manager_row(club) + board_record::STAFF_CONFIDENCE).wrapping_add(change).clamp(1, 10000)
}

/// The byte of the person's 79-byte state record that media stories raise (`staff_state::INJURY_COUNTER`).
unsafe fn story_unrest(p: Staff) -> StaffState {
    StaffState(staff_state_entry(p))
}

/// Raises the person's state byte by draw(n) (8 bits, wrapping).
unsafe fn story_raise_unrest(p: Staff, n: i32) {
    let s = story_unrest(p);
    s.set_injury_counter(s.injury_counter().wrapping_add(exe::utils_random(n) as u8));
}

/// Whether the player already answered a story in the media's log (`story_log_entry::PERSON_A` with
/// `story_log_entry::RESPONSE` 2).
unsafe fn story_already_answered(this: usize, player: Staff) -> bool {
    let log = read_usize(this + media_fields::STORY_LOG);
    (0..read_i32(this + media_fields::STORY_LOG_COUNT).max(0) as usize).any(|i| {
        let e = log + i * 0x17; // story_log_entry, 23 bytes
        read_u32(e + story_log_entry::PERSON_A) == read_u32(player.addr()) && read_u8(e + story_log_entry::RESPONSE) == 2
    })
}

/// The player's grievance of a strong or weak reaction: the contract flag 0x100 for a kind-0x13 story, otherwise the
/// event flags 0x400.
unsafe fn story_flag_grievance(s: &StoryEffects) {
    if s.story13 != 0 {
        exe::staff_contract_set_flags(s.contract, 0x100);
    } else {
        exe::staff_contract_apply_event_flags(s.contract, 0x400, 0);
    }
}

/// The player falls out with the manager: he dislikes him (`exe::staff_add_disliked_staff`) and his contract takes the
/// event flags 8.
unsafe fn story_fall_out(s: &StoryEffects, strong: u32) {
    exe::staff_add_disliked_staff(s.player, s.club.manager(), strong);
    exe::staff_contract_apply_event_flags(s.contract, 8, 0);
}

/// The player warms to the manager: he favours him (`exe::staff_add_favoured_staff`) and his contract loses the
/// grievances 8 and 0x2000.
unsafe fn story_make_up(s: &StoryEffects) {
    exe::staff_add_favoured_staff(s.player, s.club.manager(), 0);
    exe::staff_contract_clear_grievances(s.contract, 8);
    exe::staff_contract_clear_grievances(s.contract, 0x2000);
}

unsafe fn story_gratitude(s: &StoryEffects) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_MEDIA_QUEUE_PLAYER_GRATITUDE_NEWS);
    f(s.this, s.player.addr(), s.club.addr(), s.kind, 0)
}

unsafe fn story_backing_request(s: &StoryEffects, story: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_MEDIA_POST_PLAYER_BACKING_REQUEST_NEWS);
    f(s.this, s.player.addr(), s.club.addr(), story, s.kind)
}

unsafe fn story_criticism_reaction(s: &StoryEffects) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_MEDIA_POST_PLAYER_CRITICISM_REACTION_NEWS);
    f(s.this, s.player.addr(), s.club.addr(), s.kind as u16 as i16 as i32 as u32, 2)
}

/// The supporters' and board's confidence when the caller asks for it (flag 0x400): d = (form − expectation) × 225
/// (`ftol`); with flag 0x100 (backed) the supporters get d, d + 1000 for a player the club favours or d − 1000 for one it
/// dislikes, and the board d, d + 1000 × the chairman's weight for a player the chairman favours or d − 1000 × weight
/// for one he dislikes; with 0x80 (criticised) the signs turn (−d, −1000 − d, 1000 − d; the board −d, −1000 × weight
/// − d, 1000 × weight − d); with neither, h = |d/2| and the supporters get −h, −200 − h or 200 − h, the board −h,
/// −200 × weight − h or 200 × weight − h.
unsafe fn story_effects_confidence(s: &StoryEffects, chairman_weight: f64, response: u32) {
    covered!("sx01c_1 story effects confidence");
    let (club, player, kind) = (s.club, s.player, s.kind);
    let d = ftol((s.form as f64 - s.expected as f64) * read_f32(rdata::F32_225) as f64);
    let d16 = d as i16 as f64;
    let favoured = exe::club_favours_staff(player, club) != 0;
    let disliked = !favoured && exe::club_dislikes_staff(player, club) != 0;
    let board = if s.flags & 0x100 != 0 {
        let supporters = if favoured { d + 1000 } else if disliked { d - 1000 } else { d };
        player_story_supporters(s.this, club, player, kind, supporters, response);
        if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_1000) + d16)
        } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
            ftol(d16 - chairman_weight * read_f64(rdata::F64_1000))
        } else {
            d
        }
    } else if s.flags & 0x80 != 0 {
        let supporters = if favoured { -1000 - d } else if disliked { 1000 - d } else { d.wrapping_neg() };
        player_story_supporters(s.this, club, player, kind, supporters, response);
        if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_NEG_1000) - d16)
        } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_1000) - d16)
        } else {
            d.wrapping_neg()
        }
    } else {
        let h = (d as i16 as i32 / 2).abs();
        let supporters = if favoured { -200 - h } else if disliked { 200 - h } else { -h };
        player_story_supporters(s.this, club, player, kind, supporters, response);
        if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_NEG_200) - h as f64)
        } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_200) - h as f64)
        } else {
            -h
        }
    };
    player_story_board(s.this, club, player, kind, board, response);
}

/// The player's form and the manager's expectation (`form`, `expected`): the expectation is (`.pressure()` − 10) / 15 +
/// the contract's `.morale()` / 150, one less when the manager dislikes the player and one more when he favours him,
/// and one less when the player already answered a story (then a draw of his `.temperament()` below draw(6) stops
/// the apology).
unsafe fn story_effects_expectation(s: &mut StoryEffects, response: u32) {
    let (club, player) = (s.club, s.player);
    s.form = exe::media_rate_player_form_for_story(s.this, player, club, s.contract, s.kind, response, s.weight) as f32;
    s.expected = ((player.pressure() as i32 - 10) as f64 * read_f32(STORY_F32_ONE_FIFTEENTH) as f64
        + s.contract.morale() as f64 * read_f32(STORY_F32_ONE_150TH) as f64) as f32;
    if exe::staff_is_disliked_by(club.manager(), player) != 0 {
        s.expected = (s.expected as f64 - read_f32(rdata::F32_1) as f64) as f32;
    } else if exe::staff_is_favoured_by(club.manager(), player) != 0 {
        s.expected = (s.expected as f64 + read_f32(rdata::F32_1) as f64) as f32;
    }
    if story_already_answered(s.this, player) {
        s.expected = (s.expected as f64 - read_f32(rdata::F32_1) as f64) as f32;
        if exe::utils_random(player.temperament() as i32) < exe::utils_random(6) {
            s.may_apologise = false;
        }
    }
}

/// The squad member's morale by the flags (the first that is set): 1 raises it 5; 2 lowers it 5; 4 moves it draw(4) −
/// draw(4); 8: an ambitious member (draw(ambition) above draw(20)) without professionalism (draw below draw(20)) loses
/// 5, otherwise a professional one gains draw(5); 0x10: professional and ambitious, draw(5); 0x20: by the ambition +
/// the weight against draw(10) and the professionalism against the expectation + 30, −8 or draw(5); 0x200: the story's
/// player (when it is not the member) loses 8, the member's contract morale moves by the player's temperament × 3 −
/// 100, and on two zero draws the member dislikes the manager (event flags 8 on his contract). Then, with 0x40, a
/// member whose state byte is below 50 with a zero draw(professionalism) has it raised by draw(20).
unsafe fn story_effects_member_morale(s: &StoryEffects, m: Staff, member_contract: Contract) {
    let f = s.flags;
    let rnd = |n: i32| exe::utils_random(n);
    if f & 1 != 0 {
        adjust_morale(m.addr(), 5);
    } else if f & 2 != 0 {
        adjust_morale(m.addr(), -5);
    } else if f & 4 != 0 {
        let a = rnd(4);
        let b = rnd(4);
        exe::staff_adjust_morale(m, (a as u8).wrapping_sub(b as u8) as u32, 1); // the original pushes the byte difference
    } else if f & 8 != 0 {
        if rnd(m.ambition() as i32) > rnd(20) && rnd(m.professionalism() as i32) < rnd(20) {
            adjust_morale(m.addr(), -5);
        } else if rnd(m.professionalism() as i32) > rnd(20) {
            adjust_morale(m.addr(), rnd(5));
        }
    } else if f & 0x10 != 0 {
        if rnd(m.professionalism() as i32) > rnd(20) && rnd(m.ambition() as i32) > rnd(20) {
            adjust_morale(m.addr(), rnd(5));
        }
    } else if f & 0x20 != 0 {
        let threshold = || ftol(s.expected as f64 + read_f32(STORY_F32_30) as f64);
        if rnd(ftol(m.ambition() as f64 + s.weight as f64)) > rnd(10) && rnd(m.professionalism() as i32) < rnd(threshold()) {
            adjust_morale(m.addr(), -8);
        } else if rnd(m.professionalism() as i32) > rnd(threshold()) {
            adjust_morale(m.addr(), rnd(5));
        }
    } else if f & 0x200 != 0 && s.player != m {
        adjust_morale(s.player.addr(), -8);
        add_contract_morale(member_contract.addr(), s.player.temperament().wrapping_mul(3).wrapping_sub(100) as i32);
        if rnd(s.player.professionalism() as i32) == 0 && rnd(s.player.temperament() as i32) == 0 {
            exe::staff_add_disliked_staff(m, s.club.manager(), 0);
            exe::staff_contract_apply_event_flags(member_contract, 8, 0);
        }
    }
    if f & 0x40 != 0 && story_unrest(m).injury_counter() < 50 && rnd(m.professionalism() as i32) == 0 {
        story_raise_unrest(m, 20);
    }
}

/// Flag 0x80, the member is the story's player (criticised): he loses 5 of morale; in a kind-6 story a zero
/// draw(dirtiness) lowers his `.dirtiness()` (above 10 by 1 + draw(3), above 4 by 1); a zero draw(pressure) gives the
/// contract event 0x100000 on draw(pressure) below draw(15) (under 24) or draw(6); a player who has not answered a
/// story yet, hard-working and professional (draws above draw(20)), may gain consistency and work rate (up to 20) and
/// his state byte is cleared. Then by the form: well above the expectation (+0.5) the contract morale moves by the
/// temperament × 3 − 100, he falls out with the manager on a zero draw(professionalism) or else may answer the
/// criticism and takes the grievance 0x80 (contract flag for a kind-0x13 story, event 0x80000 otherwise), may ask for
/// backing, and for kinds 1, 0x1e and 3 a disloyal, unhappy foreigner may go absent (AWOL); well below (−0.5) he
/// apologises or answers the criticism; in between the contract morale moves by the temperament − 20, he may fall out
/// with the manager and ask for backing.
unsafe fn story_effects_player_criticised(s: &StoryEffects, member_contract: Contract) {
    covered!("sx01c_1 story effects player criticised");
    let (p, c, club) = (s.player, s.contract, s.club);
    let rnd = |n: i32| exe::utils_random(n);
    adjust_morale(p.addr(), -5);
    if s.kind as u16 == 6 && rnd(p.player().dirtiness() as i32) == 0 {
        let sheet = p.player();
        let d = sheet.dirtiness();
        if d > 10 {
            sheet.set_dirtiness(d.wrapping_add((0xffu8.wrapping_sub(rnd(3) as u8)) as i8));
        } else if d > 4 {
            sheet.set_dirtiness(d - 1);
        }
    }
    if rnd(p.pressure() as i32) == 0 {
        let hit = if (p.age() as i8) < 24 { rnd(p.pressure() as i32) < rnd(15) } else { rnd(p.pressure() as i32) < rnd(6) };
        if hit {
            exe::staff_contract_apply_event_flags(c, 0x100000, 0);
        }
    }
    if !story_already_answered(s.this, p) && rnd(p.player().work_rate() as i32) > rnd(20) && rnd(p.professionalism() as i32) > rnd(20) {
        let sheet = p.player();
        if (sheet.consistency() as i32) < rnd(30) {
            let v = sheet.consistency().wrapping_add(rnd(5) as i8);
            sheet.set_consistency(v.min(20));
        }
        let w = sheet.work_rate();
        if (w as i32) < rnd(30) {
            sheet.set_work_rate(w.wrapping_add(1).min(20));
        }
        story_unrest(p).set_injury_counter(0);
    }
    if s.above(read_f64(rdata::F64_0_5)) {
        covered!("sx01c_1 story effects player criticised above");
        add_contract_morale(c.addr(), p.temperament().wrapping_mul(3).wrapping_sub(100) as i32);
        if rnd(p.professionalism() as i32) == 0 {
            story_fall_out(s, 1);
        } else {
            if member_contract.grievances() == 0 && (p.player().player_morale() as i32) > rnd(30) + 10 && !s.chosen {
                story_criticism_reaction(s);
            }
            if s.story13 != 0 {
                exe::staff_contract_set_flags(c, 0x80);
            } else {
                exe::staff_contract_apply_event_flags(c, 0x80000, 0);
            }
        }
        if rnd(p.temperament() as i32) == 0 {
            let k = s.kind as u16;
            let strong = (k == 0x1e || k == 0x20) && (member_contract.grievances() & 0x40000 != 0 || rnd(p.pressure() as i32) == 0);
            if strong {
                story_backing_request(s, if s.chosen { 3 } else { 2 });
            } else if s.story13 == 0 && !s.chosen {
                story_backing_request(s, 0);
            }
            story_raise_unrest(p, 15);
        }
        let k = s.kind as u16;
        if (k == 1 || k == 0x1e || k == 3)
            && !s.chosen
            && exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) == 0
            && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
            && rnd(p.loyalty() as i32) == 0
            && rnd(p.pressure() as i32) == 0
            && exe::database_is_staff_foreign(p, club.nation(), 1) != 0
            && member_contract.morale() < -50
        {
            covered!("sx01c_1 story effects player goes awol");
            exe::awol_start_absence(read_usize(G_AWOL), p, 0, 0);
        }
    } else if s.below(read_f64(rdata::F64_0_5)) {
        covered!("sx01c_1 story effects player criticised below");
        if s.may_apologise && rnd(p.professionalism() as i32 + p.loyalty() as i32) > rnd(120) {
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(player_story_fns::QUEUE_PLAYER_APOLOGY_NEWS);
            f(s.this, p.addr(), club.addr(), s.kind);
        } else if member_contract.grievances() == 0 && (p.player().player_morale() as i32) > rnd(40) + 15 && s.story13 == 0 && !s.chosen {
            story_criticism_reaction(s);
        }
    } else {
        covered!("sx01c_1 story effects player criticised between");
        add_contract_morale(c.addr(), p.temperament().wrapping_sub(20) as i32);
        if rnd(p.loyalty() as i32 + p.professionalism() as i32) == 0 {
            exe::staff_add_disliked_staff(p, club.manager(), 0);
            if s.story13 != 0 {
                exe::staff_contract_set_flags(c, 0x80);
                return;
            }
            exe::staff_contract_apply_event_flags(c, 0x80000, 0);
        } else if s.story13 != 0 {
            return;
        }
        if rnd(p.temperament() as i32) == 0 && rnd(p.professionalism() as i32) == 0 {
            story_backing_request(s, s.chosen as u32);
            story_raise_unrest(p, 15);
        }
    }
}

/// Flag 0x80, a team-mate of the criticised player: one the player dislikes gains 5 of morale; with one he favours, by
/// the form (±1.5 around the expectation), the player may take the grievance (`story_flag_grievance`) and on zero draws
/// fall out with the manager; with any other, a respected player (leadership × 50 + reputation above 5250 and the
/// team-mate's) may take the grievance when the form is well above the expectation.
unsafe fn story_effects_team_mate_criticised(s: &StoryEffects, m: Staff) {
    let p = s.player;
    let rnd = |n: i32| exe::utils_random(n);
    let (above, below) = (s.above(read_f64(rdata::F64_1_5)), s.below(read_f64(rdata::F64_1_5)));
    if exe::staff_is_disliked_by(p, m) != 0 {
        adjust_morale(m.addr(), 5);
        return;
    }
    if exe::staff_is_favoured_by(p, m) == 0 {
        if above {
            let sheet = p.player();
            let standing = sheet.leadership() as i32 * 50 + sheet.current_reputation() as i32;
            if standing > 0x1482 && standing > m.player().current_reputation() as i32 && rnd(m.temperament() as i32) == 0 && rnd(m.ambition() as i32) != 0 {
                story_flag_grievance(s);
            }
        }
        return;
    }
    covered!("sx01c_1 story effects team mate criticised");
    let fall_out = if above {
        if rnd(m.loyalty() as i32) != 0 {
            story_flag_grievance(s);
        }
        rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32) == 0
    } else if below {
        if rnd(m.loyalty() as i32) > rnd(20) {
            story_flag_grievance(s);
        }
        rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32) == 0 && rnd(p.loyalty() as i32) == 0
    } else {
        if rnd(m.loyalty() as i32) > rnd(10) {
            story_flag_grievance(s);
        }
        rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32 + 5) == 0 && rnd(p.loyalty() as i32 + 10) != 0
    };
    if fall_out {
        story_fall_out(s, 0);
    }
}

/// Flag 0x100, the member is the story's player (backed): in a kind-0x13 story with the contract grievance 0x2000000,
/// a disloyal player (draw(loyalty) below 5) takes the event 0x8000000 unless he is professional (7+) and the injury
/// count reaches 4 (the original counts the player's own injury status once per filled squad slot). Otherwise he gains
/// 10 of morale; in a kind-6 story zero draws of his professionalism may lower his temperament and raise his
/// aggression and dirtiness (up to 20); the grievance 0x40000 may clear; his work rate may drop (and the state byte
/// rise) or his loyalty rise (and the state byte clear). By the form (±0.5): the state byte may rise, he may thank the
/// manager (`queue_player_gratitude_news`) and, ambitious and loyal, favour him (grievances 8 and 0x2000 cleared).
unsafe fn story_effects_player_backed(s: &StoryEffects, member_contract: Contract) {
    covered!("sx01c_1 story effects player backed");
    let (p, club) = (s.player, s.club);
    let rnd = |n: i32| exe::utils_random(n);
    if s.story13 != 0 && member_contract.grievances() & 0x2000000 != 0 {
        let mut injured: u8 = 0;
        for k in 0..50 {
            // defect of the original: the status asked is the story player's, not the squad member's
            if club.squad(k) != 0 && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
                injured = injured.wrapping_add(1);
            }
        }
        if (injured as i8) >= 4 && p.professionalism() >= 7 {
            return;
        }
        if rnd(p.loyalty() as i32) < 5 {
            exe::staff_contract_apply_event_flags(s.contract, 0x8000000, 0);
        }
        return;
    }
    adjust_morale(p.addr(), 10);
    if s.kind as u16 == 6 {
        if p.temperament() > 1 && rnd(p.professionalism() as i32) == 0 {
            p.set_temperament(p.temperament() - 1);
        }
        if p.player().aggression() < 20 && rnd(p.professionalism() as i32) == 0 {
            p.player().set_aggression(p.player().aggression() + 1);
        }
        if p.player().dirtiness() < 20 && rnd(p.professionalism() as i32) == 0 {
            p.player().set_dirtiness(p.player().dirtiness() + 1);
        }
    }
    if member_contract.grievances() & 0x40000 != 0 && rnd(p.pressure() as i32) > rnd(8) {
        exe::staff_contract_clear_grievances(member_contract, 0x40000);
    }
    if rnd(p.player().work_rate() as i32) == 0 && rnd(p.professionalism() as i32) < rnd(10) {
        if rnd(p.player().work_rate() as i32) != 0 {
            p.player().set_work_rate(p.player().work_rate() - 1);
        }
        story_raise_unrest(p, 15);
    } else if rnd(p.professionalism() as i32) > rnd(25) && rnd(p.player().work_rate() as i32) > rnd(40) {
        if story_unrest(p).injury_counter() != 0 {
            story_unrest(p).set_injury_counter(0);
        }
        let l = p.loyalty();
        if (l as i32) < rnd(30) {
            p.set_loyalty(l.wrapping_add(1).min(20));
        }
    }
    let raise_ok = |extra: i32| story_unrest(p).injury_counter() < 50 && rnd(p.professionalism() as i32 + extra) == 0;
    // the 0xa test never holds (defect of the original: story13 is 0 or 0x13), so the grievance 0x4000000 stays
    let thanks = |draw: i32, clear_0x4000000: bool| {
        if rnd(p.loyalty() as i32) > rnd(draw) {
            story_gratitude(s);
            if clear_0x4000000 && s.story13 == 0xa && member_contract.grievances() & 0x4000000 != 0 {
                exe::staff_contract_clear_grievances(member_contract, 0x4000000);
            }
        }
    };
    let make_up = |draw: i32| {
        if rnd(p.ambition() as i32) > rnd(draw) && rnd(p.loyalty() as i32) > rnd(draw) {
            story_make_up(s);
        }
    };
    if s.above(read_f64(rdata::F64_0_5)) {
        covered!("sx01c_1 story effects player backed above");
        if raise_ok(10) {
            story_raise_unrest(p, 20);
        }
        thanks(40, true);
        make_up(60);
    } else if s.below(read_f64(rdata::F64_0_5)) {
        if raise_ok(0) {
            story_raise_unrest(p, 30 - p.professionalism() as i32);
        }
        thanks(10, false);
        make_up(10);
    } else {
        if raise_ok(5) {
            story_raise_unrest(p, 20);
        }
        thanks(20, true);
        make_up(40);
    }
}

/// Flag 0x100, a team-mate of the backed player: for a kind-0x1e story zero draws make the player fall out with the
/// manager; a team-mate the player dislikes loses morale (5, 15 for 0x1e; 15 and 25 when the form is well below), one
/// he favours gains 5 (well above), 10 (well below) or loses 5 (0x1e).
unsafe fn story_effects_team_mate_backed(s: &StoryEffects, m: Staff) {
    let p = s.player;
    let rnd = |n: i32| exe::utils_random(n);
    let k1e = s.kind as u16 == 0x1e;
    let (disliked, favoured) = if s.above(read_f64(rdata::F64_1_5)) {
        if k1e && rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32 + 5) == 0 && rnd(p.loyalty() as i32 + 10) != 0 {
            story_fall_out(s, 0);
        }
        (if k1e { -15 } else { -5 }, if k1e { -5 } else { 5 })
    } else if s.below(read_f64(rdata::F64_1_5)) {
        if k1e && rnd(m.temperament() as i32 / 2) == 0 && rnd(m.professionalism() as i32 / 2) == 0 {
            story_fall_out(s, 0);
        }
        (if k1e { -25 } else { -15 }, if k1e { -5 } else { 10 })
    } else {
        if k1e && rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32) == 0 {
            story_fall_out(s, 0);
        }
        return;
    };
    if exe::staff_is_disliked_by(p, m) != 0 {
        adjust_morale(m.addr(), disliked);
    } else if exe::staff_is_favoured_by(p, m) != 0 {
        adjust_morale(m.addr(), favoured);
    }
}

/// Applies the effects of a media story's outcome to the club and the squad.
///
/// First the staff's confidence in the manager (`board_record::STAFF_CONFIDENCE` of the club's row in
/// MANAGER_MANAGER) moves by `confidence_change`, kept between 1 and 10000, and the chairman's weight is
/// (`.ambition()` − 10) × 0.05 + 1 (0.25 without a chairman). With a player: no contract at the club ends it; otherwise
/// his form (`exe::media_rate_player_form_for_story`) and the manager's expectation (`story_effects_expectation`); with
/// flag 0x400 the supporters' and board's confidence (`story_effects_confidence`); and when the caller passed neither
/// 0x80 nor 0x100 the routine marks the reaction as its own and treats the story as criticism (0x80) when the form beats
/// the expectation by more than 1.
///
/// Then each of the 50 squad members with a player sheet, an id below the staff count and a contract at the club, when
/// draw(contract `.happiness()` + 150) is not below draw(100): his morale by the flags
/// (`story_effects_member_morale`); with a player, flag 0x80 gives `story_effects_player_criticised` (the member is the
/// player) or `story_effects_team_mate_criticised`, and flag 0x100 `story_effects_player_backed` or
/// `story_effects_team_mate_backed`.
///
/// Without a player the expectation read by flag 0x20 is the bits of the response argument as a float (the original's
/// slot is never written). Defects of the original: the injured count of a backed kind-0x13 story asks the player's
/// own status once per filled slot; the thanks of a backed player test the story kind 0x13 against 0xa, never true.
/// Original: `Media::apply_story_effect_flags` (0x736c00, media.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn apply_story_effect_flags(this: usize, club: usize, kind: u32, flags: u32, weight: f32, response: u32, confidence_change: u32, player: usize) {
    covered!("sx01c_1 story effects");
    let club = Club(club);
    if !club.is_null() {
        let row = manager_manager_row(club);
        write_i16(row + board_record::STAFF_CONFIDENCE, story_staff_confidence_after(club, confidence_change as i16));
    }
    let chairman = club.chairman();
    let chairman_weight = if chairman.is_null() {
        0.25
    } else {
        (chairman.ambition() as i32 - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1)
    };
    let mut s = StoryEffects {
        this,
        club,
        kind,
        flags,
        weight,
        player: Staff(player),
        contract: Contract(0),
        story13: if kind as u16 == 0x13 { 0x13 } else { 0 },
        may_apologise: true,
        chosen: false,
        form: 0.0,
        expected: f32::from_bits(response),
    };
    if !s.player.is_null() {
        s.contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), s.player, club));
        if s.contract.is_null() {
            return;
        }
        story_effects_expectation(&mut s, response);
        if s.flags & 0x400 != 0 {
            story_effects_confidence(&s, chairman_weight, response);
        }
        if s.flags & 0x80 == 0 && s.flags & 0x100 == 0 {
            s.chosen = true;
            if s.above(read_f64(rdata::F64_1)) {
                covered!("sx01c_1 story effects chosen criticism");
                s.flags |= 0x80;
            }
        }
    }
    for k in 0..50 {
        let m = Staff(club.squad(k));
        if m.is_null() || m.player().is_null() || read_i32(m.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let mc = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), m, club));
        if mc.is_null() || exe::utils_random(mc.happiness() as i32 + 150) < exe::utils_random(100) {
            continue;
        }
        story_effects_member_morale(&s, m, mc);
        if s.player.is_null() {
            continue;
        }
        if s.flags & 0x80 != 0 {
            if m == s.player {
                story_effects_player_criticised(&s, mc);
            } else {
                story_effects_team_mate_criticised(&s, m);
            }
        } else if s.flags & 0x100 != 0 {
            if m == s.player {
                story_effects_player_backed(&s, mc);
            } else {
                story_effects_team_mate_backed(&s, m);
            }
        }
    }
}

/// Comparer for `apply_story_effect_flags`: the routine draws, posts news and changes confidences, relations and
/// morale, so it cannot run twice; only the original runs and the staff's confidence it leaves is checked against the
/// copy's first piece (`story_staff_confidence_after`). The rest is proven by the career and the mutations.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_apply_story_effect_flags(this: usize, club: usize, kind: u32, flags: u32, weight: f32, response: u32, confidence_change: u32, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, f32, u32, u32, usize)>(IDX_APPLY_STORY_EFFECT_FLAGS);
    if club == 0 {
        return o(this, club, kind, flags, weight, response, confidence_change, player);
    }
    let before = read_i16(manager_manager_row(Club(club)) + board_record::STAFF_CONFIDENCE);
    let expected = story_staff_confidence_after(Club(club), confidence_change as i16);
    o(this, club, kind, flags, weight, response, confidence_change, player);
    let after = read_i16(manager_manager_row(Club(club)) + board_record::STAFF_CONFIDENCE);
    SCOREBOARD.record(after == expected, || format!(
        "apply_story_effect_flags(club {:x}, kind {:#x}, flags {:#x}, change {}) staff confidence {} -> {} x original {}",
        club, kind, flags, confidence_change as i16, before, expected, after));
}

// --- sx01c_2: Media::queue_player_sent_off_story (0x759610, media.cpp) ---

/// Queues the press story about a player sent off (news item 0xbe6).
///
/// Nothing unless the player has a nation (`.nation()`), a club (`.club_job()`) whose `.manager()` is set and a
/// player sheet (`.player()`), and the other player has a club with a manager and a sheet. The local news item 0xbe6:
/// 0..3 the player and his names, 4..7 the other player, 8 draw(2), 9 the player's club id, 10 the other's club id,
/// 11 the player's stat 0xb (red cards, by the community map) (`exe::playerstats_get_stat`(sheet, 0xb, 2, 6.5), `ftol`), 12..15 the player's club
/// manager, 16..19 the other club's manager, 20 the AI manager's response (`exe::media_pick_ai_manager_response`(manager,
/// player, 0x82)) when the player's club is not human (`media_is_human_club`(club, 0, 1)), otherwise 0, then the match
/// record (`exe::fill_match_record_from_fixture` with the player's club, `exe::write_match_record_to_news`). A player
/// with a `.world_reputation()` above 4750 goes to every manager of his nation (`exe::news_send_to_nation_managers`),
/// any other to his club's manager (`exe::news_send_to_club_manager`).
/// Original: `Media::queue_player_sent_off_story` (0x759610, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn queue_player_sent_off_story(this: usize, player: usize, other: usize, fixture: usize) {
    let (player, other) = (Staff(player), Staff(other));
    let has_club_and_sheet = |p: Staff| !p.club_job().is_null() && !p.club_job().manager().is_null() && !p.player().is_null();
    if player.is_null() || player.nation().is_null() || !has_club_and_sheet(player) || other.is_null() || !has_club_and_sheet(other) {
        return;
    }
    covered!("sx01c_2 sent off story");
    let (club, other_club) = (player.club_job(), other.club_job());
    let (build, write) = news_item_fns();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    build(n, 0xbe6, 0);
    set_person_name_slots(write, n, 0, player.addr());
    set_person_name_slots(write, n, 4, other.addr());
    write(n, 8, exe::utils_random(2) as u32);
    write(n, 9, read_u32(club.addr()));
    write(n, 10, read_u32(other_club.addr())); // the original writes -1 without a club, which the test above rules out
    write(n, 11, ftol(exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), player.player(), 0xb, 2, 6.5)) as u32);
    set_person_name_slots(write, n, 12, club.manager().addr());
    set_person_name_slots(write, n, 16, other_club.manager().addr());
    let response = if media_is_human_club(club.addr(), 0, 1) == 0 {
        exe::media_pick_ai_manager_response(this, club.manager(), player, 0x82)
    } else {
        0
    };
    write(n, 20, response);
    let mut k: u8 = 21;
    let mut record = [0u8; 0x20]; // the 0x1f-byte match record
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    exe::fill_match_record_from_fixture(fixture, club, record.as_mut_ptr() as usize);
    exe::write_match_record_to_news(record.as_mut_ptr() as usize, n, &mut k);
    if player.player().world_reputation() > 0x128e {
        exe::news_send_to_nation_managers(read_usize(G_NEWS), n, player.nation());
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
}

/// Comparer for `queue_player_sent_off_story`: it draws and posts news, so it cannot run twice; only the original runs
/// (in the comparison mode the post-match gate of 0x759610 is installed first and keeps the address, see the item).
pub(crate) unsafe extern "thiscall" fn compare_queue_player_sent_off_story(this: usize, player: usize, other: usize, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_QUEUE_PLAYER_SENT_OFF_STORY)(this, player, other, fixture)
}

// ---------- batch 330: Date.cpp: setting a date, going back whole weeks and the gap between two dates (sx09_1) ----------

/// The game's date (`Date.cpp`, 8 bytes: day of the year, year, leap-year flag), the type of `crate::date`.
pub(crate) type GameDate = crate::date::Data;

/// Date.cpp's source path in the exe, for the error boxes of the date routines.
pub(crate) const DATE_CPP: usize = 0x9b8478;
/// The days before each month (12 `i16` from January, starting at -1): an ordinary year and a leap year.
pub(crate) const DAYS_BEFORE_MONTH: usize = 0x9b8448;
pub(crate) const DAYS_BEFORE_MONTH_LEAP: usize = 0x9b8460;

/// The local static result of `date_sub_weeks` (its guard byte and the empty destructor registered at exit).
const SUB_WEEKS_RESULT: usize = 0xae24c8;
const SUB_WEEKS_GUARD: usize = 0xae24aa;
const SUB_WEEKS_DESTRUCTOR: usize = 0x54cc20;
/// The local static result of `date_get_difference`.
const DIFFERENCE_RESULT: usize = 0xae2428;
const DIFFERENCE_GUARD: usize = 0xae249d;
const DIFFERENCE_DESTRUCTOR: usize = 0x54cda0;
/// The local static result of `Date::sub_days` (0x54c8e0), which `date_set` reaches through the weekday adjustment.
const SUB_DAYS_RESULT: usize = 0xae24c0;
const SUB_DAYS_GUARD: usize = 0xae249e;

/// The days before `month` (0..11) in an ordinary or a leap year.
pub(crate) unsafe fn days_before_month(leap: bool, month: usize) -> i16 {
    read_i16(if leap { DAYS_BEFORE_MONTH_LEAP } else { DAYS_BEFORE_MONTH } + 2 * month)
}

/// The month (0..11) of a date: the last month whose days before it are fewer than the date's day of the year; none
/// when the day is -1 or less (the tables start at -1).
pub(crate) unsafe fn date_month_index(d: &GameDate) -> Option<usize> {
    (0..12).rev().find(|&k| days_before_month(d.is_leap_year != 0, k) < d.day)
}

/// The local static result date of a Date routine, built on the first call as the original does: the guard bit set,
/// the default date (`crate::date::default_date`) and the empty destructor registered at exit.
pub(crate) unsafe fn date_local_static(value: usize, guard: usize, destructor: usize) -> *mut GameDate {
    if read_u8(guard) & 1 == 0 {
        write_u8(guard, read_u8(guard) | 1);
        crate::date::default_date(value as *mut GameDate);
        crate::crt::atexit(destructor);
    }
    value as *mut GameDate
}

/// Writes a date name into one of the shared name buffers and returns the buffer: the langlib template (its extra
/// argument is the empty string, which only the templates with a translator's comment read), or the empty string when
/// there is none.
pub(crate) unsafe fn write_date_name(buffer: usize, template: Option<usize>) -> usize {
    match template {
        Some(t) => story_lang!(buffer, t, EMPTY_STRING),
        None => copy_c_string(buffer, EMPTY_STRING),
    }
    buffer
}

/// Sets a date from the day of the month, the month and the year, and moves it to the nearest given weekday.
///
/// `Date::set(day, month, year, weekday)`: a day of the month outside 1..31 (read as `i16`) gives the error box
/// (Date.cpp, line 0x72) and becomes 1; a month outside 0..11 (read as a signed byte) gives the error box (line 0x79)
/// and becomes January. The year (16 bits) and its Gregorian leap flag (`crate::date::is_leap_year`) are stored, and
/// the day of the year is the days before the month (`days_before_month`) plus the day of the month. A weekday other
/// than 0xffff (16 bits) moves the date to the nearest one, three days either way (`exe::date_get_nearest_weekday` on
/// the date itself). Returns what the original leaves in eax: the weekday argument when it is 0xffff, otherwise the
/// adjustment's return (no caller reads it). The same as the constructor `Date::Date` (0x549f70) but for the error
/// lines.
/// Original: `Date::set` (0x54a130, Date.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn date_set(this: *mut GameDate, day: u32, month: u32, year: u32, weekday: u32) -> u32 {
    covered!("sx09_1 date set");
    let mut day = day as i16;
    if !(1..=31).contains(&day) {
        crate::crt::error_box(DATE_CPP, 0x72);
        day = 1;
    }
    let mut month = month as u8 as i8;
    if !(0..=11).contains(&month) {
        crate::crt::error_box(DATE_CPP, 0x79);
        month = 0;
    }
    let year = year as u16;
    let leap = crate::date::is_leap_year(year) != 0;
    (*this).year = year;
    (*this).is_leap_year = leap as i32;
    (*this).day = days_before_month(leap, month as usize).wrapping_add(day); // sx09_1: the day of the year
    if weekday as u16 == 0xffff {
        return weekday;
    }
    exe::date_get_nearest_weekday(this as usize, weekday, this as usize)
}

/// Comparer for `date_set`: the date and `Date::sub_days`' static side by side (the return is the adjustment's leftover
/// and is not compared). A bad day or month: only the original (the error box).
pub(crate) unsafe extern "thiscall" fn compare_date_set(this: *mut GameDate, day: u32, month: u32, year: u32, weekday: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(*mut GameDate, u32, u32, u32, u32) -> u32>(IDX_DATE_SET);
    if !(1..=31).contains(&(day as i16)) || !(0..=11).contains(&(month as u8 as i8)) {
        return o(this, day, month, year, weekday);
    }
    let regions = [(this as usize, 8), (SUB_DAYS_RESULT, 8), (SUB_DAYS_GUARD, 1)];
    let (r, _, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, day, month, year, weekday), || date_set(this, day, month, year, weekday));
    SCOREBOARD.record(equal, || format!("date_set({}, {}, {}, weekday {:#x}) original {:x?} rust {:x?}", day as i16, month as u8 as i8, year as u16, weekday as u16, mo, mr));
    r
}

/// Returns the date a number of whole weeks earlier.
///
/// `Date::sub_weeks(out, weeks)`: the local static result (`date_local_static`, built on the first call) takes the
/// date's day of the year minus 7 days per week (the count read as a byte); while the day is negative, the year goes
/// back by one and the day gains that year's length (366 in a leap year, else 365); the static gets the year's leap
/// flag and is copied to `out`, which is returned.
/// Original: `Date::sub_weeks` (0x54cab0, Date.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn date_sub_weeks(this: *const GameDate, out: *mut GameDate, weeks: u32) -> *mut GameDate {
    covered!("sx09_1 weeks back");
    let result = date_local_static(SUB_WEEKS_RESULT, SUB_WEEKS_GUARD, SUB_WEEKS_DESTRUCTOR);
    let mut year = (*this).year;
    let mut day = (*this).day.wrapping_sub((weeks as u8 as u16 * 7) as i16); // sx09_1: seven days a week
    while day < 0 {
        year = year.wrapping_sub(1);
        day = day.wrapping_add(if crate::date::is_leap_year(year) != 0 { 366 } else { 365 });
    }
    *result = GameDate { day, year, is_leap_year: crate::date::is_leap_year(year) };
    *out = *result;
    out
}

/// Comparer for `date_sub_weeks`: the result, the static and its guard side by side, and the return.
pub(crate) unsafe extern "thiscall" fn compare_date_sub_weeks(this: *const GameDate, out: *mut GameDate, weeks: u32) -> *mut GameDate {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, *mut GameDate, u32) -> *mut GameDate>(IDX_DATE_SUB_WEEKS);
    let date = *this;
    let regions = [(out as usize, 8), (SUB_WEEKS_RESULT, 8), (SUB_WEEKS_GUARD, 1)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, out, weeks), || date_sub_weeks(this, out, weeks));
    SCOREBOARD.record(equal && ro == rr, || format!("date_sub_weeks({:?}, {} weeks) original {:x?} rust {:x?}", date, weeks as u8, mo, mr));
    ro
}

/// Writes the gap between two dates as days and whole years.
///
/// `Date::get_difference(out, other)` (the other date by value): when the other date is later (a later year, or the
/// same year and a later day), the two swap places (the routine runs on the other date with this one). Otherwise,
/// in the local static result (`date_local_static`): when this day of the year is not before the other's, the days are
/// the difference of the days and the years the difference of the years; when it is before (only across a year), the
/// days gain the length of this date's year (366 in a leap year, else 365) and the years lose one. The static keeps
/// its leap flag (1 from the default date, never set here) and is copied to `out`, which is returned.
/// Original: `Date::get_difference` (0x54cc30, Date.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn date_get_difference(this: *const GameDate, out: *mut GameDate, other: GameDate) -> *mut GameDate {
    let result = date_local_static(DIFFERENCE_RESULT, DIFFERENCE_GUARD, DIFFERENCE_DESTRUCTOR);
    let date = *this;
    if other.year > date.year || other.year == date.year && other.day > date.day {
        return date_get_difference(&other, out, date);
    }
    let (days, years) = if date.day >= other.day {
        (date.day.wrapping_sub(other.day), date.year.wrapping_sub(other.year))
    } else {
        covered!("sx09_1 difference across a year");
        let length = if crate::date::is_leap_year(date.year) != 0 { 366 } else { 365 };
        (date.day.wrapping_sub(other.day).wrapping_add(length), date.year.wrapping_sub(other.year).wrapping_sub(1)) // sx09_1: a year less
    };
    (*result).day = days;
    (*result).year = years;
    *out = *result;
    out
}

/// Comparer for `date_get_difference`: the result, the static and its guard side by side, and the return.
pub(crate) unsafe extern "thiscall" fn compare_date_get_difference(this: *const GameDate, out: *mut GameDate, other: GameDate) -> *mut GameDate {
    let o = original_fn::<unsafe extern "thiscall" fn(*const GameDate, *mut GameDate, GameDate) -> *mut GameDate>(IDX_DATE_GET_DIFFERENCE);
    let date = *this;
    let regions = [(out as usize, 8), (DIFFERENCE_RESULT, 8), (DIFFERENCE_GUARD, 1)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, out, other), || date_get_difference(this, out, other));
    SCOREBOARD.record(equal && ro == rr, || format!("date_get_difference({:?}, {:?}) original {:x?} rust {:x?}", date, other, mo, mr));
    ro
}

// ---------- batch 330: Date.cpp: the day of the month, the month and the leap year (sx09_2) ----------

/// Returns the day of the month of a date (in ax; the caller reads the low word).
///
/// `Date::get_day_of_month()`: searching from December back, the first month whose days before it
/// (`days_before_month` in the date's year kind) are fewer than the day of the year: the day of the year minus those
/// days. None (a day of -1 or less): -1. The high word is what the original leaves in eax: the high word of the leap
/// flag, minus (when a month matched) the high word of the table pointer (0x9b), which the callers never read.
/// Original: `Date::get_day_of_month` (0x54cf70, Date.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn date_get_day_of_month(this: *const GameDate) -> u32 {
    let d = &*this;
    let high = d.is_leap_year as u32 & 0xffff_0000;
    match date_month_index(d) {
        Some(month) => {
            covered!("sx09_2 day of month");
            let before = days_before_month(d.is_leap_year != 0, month) as u16 as u32;
            (high | d.day as u16 as u32).wrapping_sub(0x009b_0000 | before) // sx09_2: the day of the month
        }
        None => high | 0xffff,
    }
}

/// Returns the month of a date, 0 for January to 11 for December, or -1 (all of eax) when none matches.
///
/// `Date::get_month()`: searching from December back, the first month whose days before it (`days_before_month`)
/// are fewer than the date's day of the year (`date_month_index`).
/// Original: `Date::get_month` (0x54cfb0, Date.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn date_get_month(this: *const GameDate) -> u32 {
    covered!("sx09_2 month");
    match date_month_index(&*this) {
        Some(month) => month as u32, // sx09_2: the month
        None => u32::MAX,
    }
}

/// Returns 1 when a year (the low 16 bits) is a Gregorian leap year and 0 otherwise (`crate::date::is_leap_year`: odd
/// or not divisible by 4, no; not divisible by 100, yes; otherwise only when divisible by 400).
/// Original: `date_is_leap_year` (0x54cfe0, Date.cpp, cdecl).
pub(crate) unsafe extern "C" fn date_is_leap_year(year: u32) -> u32 {
    covered!("sx09_2 leap year");
    crate::date::is_leap_year(year as u16) as u32 // sx09_2: the Gregorian rule
}

/// Comparer for `date_get_day_of_month`: the whole eax, side by side (the routine only reads).
pub(crate) unsafe extern "thiscall" fn compare_date_get_day_of_month(this: *const GameDate) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(*const GameDate) -> u32>(IDX_DATE_GET_DAY_OF_MONTH)(this);
    let n = date_get_day_of_month(this);
    SCOREBOARD.record(r == n, || format!("date_get_day_of_month({:?}) original {:#x} rust {:#x}", *this, r, n));
    r
}

/// Comparer for `date_get_month`: the return, side by side.
pub(crate) unsafe extern "thiscall" fn compare_date_get_month(this: *const GameDate) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(*const GameDate) -> u32>(IDX_DATE_GET_MONTH)(this);
    let n = date_get_month(this);
    SCOREBOARD.record(r == n, || format!("date_get_month({:?}) original {:#x} rust {:#x}", *this, r, n));
    r
}

/// Comparer for `date_is_leap_year`: the return, side by side.
pub(crate) unsafe extern "C" fn compare_date_is_leap_year(year: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_DATE_IS_LEAP_YEAR)(year);
    let n = date_is_leap_year(year);
    SCOREBOARD.record(r == n, || format!("date_is_leap_year({}) original {} rust {}", year as u16, r, n));
    r
}
