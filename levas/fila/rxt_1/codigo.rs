// --- rxt_1: the text of a scout search news item (player_search.cpp) ---

/// `player_search_write_opposition_report_text` (0x7be670): the opposition report the club scouting body hands over.
pub(crate) const FN_PLAYER_SEARCH_WRITE_OPPOSITION_REPORT_TEXT: usize = 0x7be670;

/// How many best-player entries of 0x1a bytes the original's frame holds (0x158 bytes); the sender writes at most 2.
const SEARCH_NEWS_PLAYERS: usize = 13;

/// The slots of a scout search news item (0x138a), in the order `scout_day_final_report` (and the progress news)
/// writes them and `write_search_update_news_text` reads them.
struct SearchUpdateNews {
    /// slot 0: the search's slot of the scout manager (`exe::scout_manager_get_search`).
    search: i8,
    /// the scout's name ref (three name ids, his id) and the birth date behind it.
    scout: [u8; 0x18],
    /// players found so far.
    found: i32,
    /// the search is over.
    completed: u32,
    /// the assignment: nation id, region, competition id, youth search, club scouting (the opposition flag).
    nation: i32,
    region: i8,
    comp: i32,
    youth: u32,
    club_scouting: u32,
    /// the scouted club's id and how many best players follow.
    target_club: i32,
    shown: i32,
    /// club scouting only: the best players (7 slots each: +0 the rating, +4 the player's id, +8..+0x13 his name
    /// ids, +0x14 the flags, +0x18 the current ability as a word).
    players: [[u8; 0x1a]; SEARCH_NEWS_PLAYERS],
    /// club scouting only: the eleven answers about the scouted club at +0, +1, +2, +3, +4, +5, +6, +7, +9, +0xa,
    /// +0xb and its club state at +0xc (+8 is never written; the report does not read it).
    answers: [u8; 0xd],
    /// club scouting only: the manager's club id and the squad strength band between the clubs.
    own_club: i32,
    band: u8,
}

impl SearchUpdateNews {
    unsafe fn read(news: usize) -> Self {
        let mut idx: u8 = 1;
        let next = |idx: &mut u8| {
            let k = *idx;
            *idx = idx.wrapping_add(1);
            exe::newsitem_get_slot(news, k as u32)
        };
        let search = exe::newsitem_get_slot(news, 0) as i8;
        let mut scout = [0u8; 0x18];
        scout[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
        exe::read_staff_name_ref_from_news(news, &mut idx, scout.as_mut_ptr());
        let found = next(&mut idx) as i32;
        let completed = next(&mut idx);
        let nation = next(&mut idx) as i32;
        let region = next(&mut idx) as i8;
        let comp = next(&mut idx) as i32;
        let youth = next(&mut idx);
        let club_scouting = next(&mut idx);
        let target_club = next(&mut idx) as i32;
        let shown = next(&mut idx) as i32;
        let mut r = SearchUpdateNews { search, scout, found, completed, nation, region, comp, youth, club_scouting, target_club, shown,
            players: [[0; 0x1a]; SEARCH_NEWS_PLAYERS], answers: [0; 0xd], own_club: 0, band: 0 };
        if club_scouting == 0 {
            return r;
        }
        // ponytail: the original reads `shown` entries into a 13-entry frame array; the sender writes at most 2
        for k in 0..shown.clamp(0, SEARCH_NEWS_PLAYERS as i32) as usize {
            let e = r.players[k].as_mut_ptr() as usize;
            for off in [0usize, 4, 8, 0xc, 0x10, 0x14] {
                write_u32(e + off, next(&mut idx));
            }
            write_u16(e + 0x18, next(&mut idx) as u16);
        }
        for off in [1usize, 0, 2, 0xa, 5, 6, 7, 9, 3, 4, 0xb] { // rxt_1: where each answer goes
            r.answers[off] = next(&mut idx) as u8;
        }
        r.own_club = next(&mut idx) as i32;
        next(&mut idx); // the scouted club's reputation: not used
        r.band = next(&mut idx) as u8;
        r.answers[0xc] = next(&mut idx) as u8;
        r
    }

    fn scout_id(&self) -> i32 {
        i32::from_le_bytes([self.scout[0xc], self.scout[0xd], self.scout[0xe], self.scout[0xf]])
    }

    /// The scout's name in full (style 1) into a 0xc8-byte buffer.
    unsafe fn scout_name(&self, buf: &mut [u8; 0xc8]) {
        let p = self.scout.as_ptr() as usize;
        exe::database_format_name_from_ids(read_u32(p), read_u32(p + 4), read_u32(p + 8), buf.as_mut_ptr(), 0xc8, 1, read_u32(p + 0xc));
    }

    /// The scout's id is set and his names still match.
    unsafe fn scout_matches(&self) -> bool {
        let p = self.scout.as_ptr() as usize;
        self.scout_id() > -1 && exe::database_staff_matches_names(staff_at(self.scout_id() as isize), read_u32(p), read_u32(p + 4), read_u32(p + 8), p + 0x10) != 0
    }
}

/// The wording of a search sentence by the assignment: [completed: none, one, several; in progress: none, one,
/// several].
const NATION_SEARCH_TEXTS: [u32; 6] = [0xa70fb4, 0xa70f4c, 0xa70ed8, 0xa70e74, 0xa70e10, 0xa70da4];
const REGION_SEARCH_TEXTS: [u32; 6] = [0xa70d30, 0xa70cc8, 0xa70c54, 0xa70bf0, 0xa70b8c, 0xa70b20];
const COMP_SEARCH_TEXTS: [u32; 6] = [0xa70aac, 0xa70a44, 0xa709d0, 0xa7096c, 0xa70908, 0xa7089c];
const YOUTH_SEARCH_TEXTS: [u32; 6] = [0xa70828, 0xa707bc, 0xa70754, 0xa706f8, 0xa7069c, 0xa7063c];

/// The search sentence into `NEWS_TEXT_BUFFER`, set as the text of `out`: "Scout <scout> has (completed his search
/// and) found no / one / <n> suitable players in his <place> search". `gender` is passed twice before the names (100 for none); a completed
/// sentence names the place before the number, one in progress after it.
unsafe fn search_sentence(out: usize, texts: &[u32; 6], completed: bool, found: i32, gender: Option<i32>, scout: usize, place: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let k = match found { 0 => 0, 1 => 1, _ => 2 } + if completed { 0 } else { 3 }; // rxt_1: none, one, several
    let tpl = texts[k];
    match (gender, k) {
        (Some(g), 2) => story_lang!(buf, tpl, g, g, scout, place, found),
        (Some(g), 5) => story_lang!(buf, tpl, g, g, scout, found, place),
        (Some(g), _) => story_lang!(buf, tpl, g, g, scout, place),
        (None, 2) => story_lang!(buf, tpl, scout, place, found),
        (None, 5) => story_lang!(buf, tpl, scout, found, place),
        (None, _) => story_lang!(buf, tpl, scout, place),
    }
    exe::text_set_string(out, buf as *const u8);
}


/// Writes the headline or body of a scout search news item (0x138a): how many suitable players the scout has found
/// in his nation, region, competition or youth search, or his report on a scouted club.
///
/// `player_search_write_search_update_news_text(news, body, out)` reads the slots (`SearchUpdateNews`) and clears
/// `out`. The assignment decides the wording, in this order: a nation (its nationality adjective, form 1), a region
/// (`exe::get_region_name`, form 1), a competition (its name), a youth search ("youth search"), club scouting (the
/// scouted club's short name); none of them: the box of line 0x1570 (body) or 0x15f0 (headline) and 0.
/// - Headline: "<place> search completed" / "<place> search update" ("Youth search completed" / "Youth scout update",
///   "<club> scouting completed").
/// - Body: the search sentence (`search_sentence`) with the scout's full name; for club scouting the opposition
///   report (`FN_PLAYER_SEARCH_WRITE_OPPOSITION_REPORT_TEXT`) instead, then a link (0x7d7) for each of the scouted
///   club's best players the report names, when his names still match (an id below 0 or above the person count: the
///   box of line 0x1540).
/// - Links (`link_search`): in the body the scout's full name (0x7d7) when his names match; for a nation, region or
///   competition headline and every body, the place's text to the search (0x7e4) when the search's scout is the
///   news item's scout.
/// Without the news item or the Text: the box of line 0x13c5 and 0. Otherwise 1.
/// Original: `player_search_write_search_update_news_text` (0x7b81f0, player_search.cpp, cdecl). Defect of the
/// original: a best player's id equal to the person count passes the check (one past the table).
pub(crate) unsafe extern "C" fn write_search_update_news_text(news: usize, body: u32, out: usize) -> u32 {
    if news == 0 || out == 0 {
        show_error_at(0xa701b0, 0x13c5);
        return 0;
    }
    let r = SearchUpdateNews::read(news);
    exe::text_clear(out as *mut usize);
    let buf = NEWS_TEXT_BUFFER;
    let mut place = [0u8; 0xc8];
    let pl = place.as_mut_ptr() as usize;
    let done = r.completed != 0;
    if body == 0 {
        covered!("rxt_1 search headline");
        if r.nation > -1 {
            let n = read_usize(G_NATION_TABLE).wrapping_add((r.nation as usize).wrapping_mul(nation::SIZE));
            exe::nation_get_nationality_adjective(n, pl, 0xc8, 1);
            let g = read_i8(n + nation::GENDER_NAME) as i32;
            story_lang!(buf, if done { 0xa70614 } else { 0xa705ec }, g, g, pl);
        } else if r.region > -1 {
            exe::get_region_name(r.region as u32, pl, 0xc8, 1);
            story_lang!(buf, if done { 0xa705c4 } else { 0xa7059c }, 0x64, 0x64, pl);
        } else if r.comp > -1 {
            let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((r.comp as usize).wrapping_mul(0x6b));
            copy_c_string(pl, c + competition::NAME);
            let g = read_i8(c + competition::GENDER_NAME) as i32;
            story_lang!(buf, if done { 0xa70574 } else { 0xa7054c }, g, g, pl);
        } else {
            if r.youth != 0 {
                story_lang!(buf, if done { 0xa70534 } else { 0xa70520 }); // "Youth search completed" / "Youth scout update"
            } else if r.club_scouting != 0 {
                let c = club_at(r.target_club as isize);
                let g = read_i8(c.addr() + club::SHORT_GENDER_NAME) as i32;
                story_lang!(buf, 0xa704f8, g, g, c.addr() + club::SHORT_NAME); // "<club> scouting completed"
            } else {
                show_error_at(0xa701b0, 0x15f0);
                return 0;
            }
            exe::text_set_string(out, buf as *const u8);
            return 1;
        }
        exe::text_set_string(out, buf as *const u8);
        if r.scout_matches() {
            link_search(&r, pl, out);
        }
        return 1;
    }
    covered!("rxt_1 search body");
    let mut name = [0u8; 0xc8];
    r.scout_name(&mut name);
    let sc = name.as_ptr() as usize;
    if r.nation > -1 {
        let n = read_usize(G_NATION_TABLE).wrapping_add((r.nation as usize).wrapping_mul(nation::SIZE));
        exe::nation_get_nationality_adjective(n, pl, 0xc8, 1);
        search_sentence(out, &NATION_SEARCH_TEXTS, done, r.found, Some(read_i8(n + nation::GENDER_NAME) as i32), sc, pl);
    } else if r.region > -1 {
        exe::get_region_name(r.region as u32, pl, 0xc8, 1);
        search_sentence(out, &REGION_SEARCH_TEXTS, done, r.found, Some(0x64), sc, pl);
    } else if r.comp > -1 {
        let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((r.comp as usize).wrapping_mul(0x6b));
        copy_c_string(pl, c + competition::NAME);
        search_sentence(out, &COMP_SEARCH_TEXTS, done, r.found, Some(read_i8(c + competition::GENDER_NAME) as i32), sc, pl);
    } else if r.youth != 0 {
        story_lang!(pl, 0xa7088c); // "youth search"
        search_sentence(out, &YOUTH_SEARCH_TEXTS, done, r.found, None, sc, pl);
    } else if r.club_scouting != 0 {
        covered!("rxt_1 club scouting report");
        copy_c_string(pl, club_at(r.target_club as isize).addr() + club::SHORT_NAME);
        club_scouting_report(&r, out);
    } else {
        show_error_at(0xa701b0, 0x1570);
        return 0;
    }
    if r.scout_matches() {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, staff_at(r.scout_id() as isize));
        link_search(&r, pl, out);
    }
    1
}

/// The club scouting body: the opposition report into `NEWS_TEXT_BUFFER`, set as the text, then a link (0x7d7) for
/// each best player the report names (`ids`, `n` of them), when his names still match.
unsafe fn club_scouting_report(r: &SearchUpdateNews, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let mut ids = [0i32; 0x85];
    let mut n: i8 = 0;
    let f: unsafe extern "C" fn(usize, Club, Club, u32, *const u8, i32, *const u8, *mut i32, *mut i8) =
        core::mem::transmute(FN_PLAYER_SEARCH_WRITE_OPPOSITION_REPORT_TEXT);
    f(buf, club_at(r.own_club as isize), club_at(r.target_club as isize), r.band as u32, r.answers.as_ptr(), r.shown,
        r.players.as_ptr() as *const u8, ids.as_mut_ptr(), &mut n);
    exe::text_set_string(out, buf as *const u8);
    let mut name = [0u8; 0xc8];
    for k in 0..r.shown.clamp(0, SEARCH_NEWS_PLAYERS as i32) as usize {
        let e = r.players[k].as_ptr() as usize;
        let id = read_i32(e + 4);
        for &named in ids.iter().take(n.max(0) as usize) {
            if id < 0 || id > read_i32(G_STAFF_COUNT) {
                show_error_at(0xa701b0, 0x1540);
                continue;
            }
            if id != named {
                continue;
            }
            let p = staff_at(id as isize);
            exe::database_format_name_from_ids(read_u32(e + 8), read_u32(e + 0xc), read_u32(e + 0x10), name.as_mut_ptr(), 0xc8, 1, id as u32);
            if exe::database_staff_matches_names(p, read_u32(e + 8), read_u32(e + 0xc), read_u32(e + 0x10), 0usize) != 0 {
                exe::text_add_link(out, name.as_ptr(), 0x7d7, p);
            }
        }
    }
}

/// The place's text links to the search (0x7e4, the search's slot) when the scout the search names (+0x180, while
/// his names +0x174..+0x17c and date +0x184 still match) is the news item's scout.
unsafe fn link_search(r: &SearchUpdateNews, place: usize, out: usize) {
    let s = exe::scout_manager_get_search(read_usize(0xb74448), r.search as i32 as u32, 0usize);
    let id = read_i32(s + 0x180);
    let mut scout = Staff(0);
    if id > -1 {
        let p = staff_at(id as isize);
        if exe::database_staff_matches_names(p, read_u32(s + 0x174), read_u32(s + 0x178), read_u32(s + 0x17c), s + 0x184) != 0 {
            scout = p;
        }
    }
    if scout == staff_at(r.scout_id() as isize) {
        exe::text_add_link(out, place as *const u8, 0x7e4, r.search as i32 as usize); // rxt_1: the place to the search
    }
}

/// Comparer for `write_search_update_news_text` (`compare_text_writer`); the answers must agree too.
pub(crate) unsafe extern "C" fn compare_write_search_update_news_text(news: usize, body: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I0%%);
    let (mut ro, mut rr) = (0, 0);
    compare_text_writer(out, || ro = o(news, body, out), |t| rr = write_search_update_news_text(news, body, t),
        || format!("write_search_update_news_text(body {}, search {})", body, exe::newsitem_get_slot(news, 0) as i8));
    SCOREBOARD.record(ro == rr, || format!("write_search_update_news_text(body {}): original {} rust {}", body, ro, rr));
    ro
}
