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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
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
