// --- r7_03_1: the scout's report news text on one player (player_search.cpp) ---

/// `player_search_write_traits_report_text` (0x7bdee0): the strengths and weaknesses paragraph of a report.
pub(crate) const FN_PLAYER_SEARCH_WRITE_TRAITS_REPORT_TEXT: usize = 0x7bdee0;

/// One verdict of a report from a watched match: the templates [win, draw, defeat] of the player's club, without and
/// with the match's stage.
type WatchedVerdict = [[u32; 3]; 2];

const WATCHED_EXCELLENT_SIGNING: WatchedVerdict = [[0xa72100, 0xa72038, 0xa71f6c], [0xa71e94, 0xa71dbc, 0xa71ce0]];
const WATCHED_GOOD_SIGNING: WatchedVerdict = [[0xa74f28, 0xa72510, 0xa72448], [0xa72374, 0xa722a4, 0xa721cc]];
const WATCHED_GOOD_PROSPECT: WatchedVerdict = [[0xa74e58, 0xa74854, 0xa74780], [0xa746a0, 0xa745c4, 0xa744e4]];
const WATCHED_DECENT_PLAYER: WatchedVerdict = [[0xa74424, 0xa74364, 0xa742a0], [0xa741d0, 0xa74104, 0xa74030]];
const WATCHED_EXCELLENT_PROSPECT: WatchedVerdict = [[0xa74d84, 0xa74cb0, 0xa74bd8], [0xa74aec, 0xa74a08, 0xa74920]];
const WATCHED_MAY_STRUGGLE: WatchedVerdict = [[0xa73f4c, 0xa73e68, 0xa73d80], [0xa73c8c, 0xa73b9c, 0xa73aa4]];
const WATCHED_SOME_POTENTIAL: WatchedVerdict = [[0xa739e0, 0xa73920, 0xa73858], [0xa73784, 0xa736b4, 0xa735e0]];
const WATCHED_UNLIKELY_TO_IMPROVE: WatchedVerdict = [[0xa72f80, 0xa72eb0, 0xa72ddc], [0xa72cfc, 0xa72c20, 0xa72b3c]];
const WATCHED_LITTLE_TO_OFFER: WatchedVerdict = [[0xa734fc, 0xa73418, 0xa73330], [0xa7323c, 0xa73148, 0xa73050]];
const WATCHED_NOT_RECOMMENDED: WatchedVerdict = [[0xa72a5c, 0xa72980, 0xa728a0], [0xa727b0, 0xa726c4, 0xa725d4]];

/// One verdict of a report without a match: the templates [of <club>, without a club].
type PlainVerdict = [u32; 2];

const PLAIN_EXCELLENT_SIGNING: PlainVerdict = [0xa71c60, 0xa71bf0];
const PLAIN_EXCELLENT_PROSPECT: PlainVerdict = [0xa71b60, 0xa71ae0];
const PLAIN_GOOD_SIGNING: PlainVerdict = [0xa71a64, 0xa719fc];
const PLAIN_GOOD_PROSPECT: PlainVerdict = [0xa71970, 0xa718f8];
const PLAIN_POSSIBLE_GOOD_SIGNING: PlainVerdict = [0xa71874, 0xa71804];
const PLAIN_SOME_POTENTIAL: PlainVerdict = [0xa7177c, 0xa71708];
const PLAIN_USEFUL_ADDITION: PlainVerdict = [0xa7167c, 0xa71600];
const PLAIN_USEFUL_IN_THE_FUTURE: PlainVerdict = [0xa71564, 0xa714dc];
const PLAIN_UNLIKELY_TO_IMPROVE: PlainVerdict = [0xa71454, 0xa713e0];
const PLAIN_NOT_GOOD_ENOUGH: PlainVerdict = [0xa71354, 0xa712d8];

/// The verdict of a report from a watched match by the scout's ability and potential ratings (0..5; any other value
/// counts as none of them).
fn watched_verdict(ability: i8, potential: i8) -> &'static WatchedVerdict {
    match (ability, potential) {
        (5, _) | (4, 5) => &WATCHED_EXCELLENT_SIGNING,
        (4, _) | (3, 5) => &WATCHED_GOOD_SIGNING,
        (3, 4) => &WATCHED_GOOD_PROSPECT,
        (3, _) => &WATCHED_DECENT_PLAYER,
        (_, 5) => &WATCHED_EXCELLENT_PROSPECT,
        (_, 4) => &WATCHED_GOOD_PROSPECT,
        (2, _) => &WATCHED_MAY_STRUGGLE,
        (_, 3) => &WATCHED_SOME_POTENTIAL,
        (1, _) => &WATCHED_UNLIKELY_TO_IMPROVE,
        (0, 2) => &WATCHED_LITTLE_TO_OFFER,
        _ => &WATCHED_NOT_RECOMMENDED,
    }
}

/// The verdict of a report without a match by the same ratings.
fn plain_verdict(ability: i8, potential: i8) -> &'static PlainVerdict {
    match (ability, potential) {
        (5, _) => &PLAIN_EXCELLENT_SIGNING,
        (_, 5) => &PLAIN_EXCELLENT_PROSPECT,
        (4, _) => &PLAIN_GOOD_SIGNING,
        (_, 4) => &PLAIN_GOOD_PROSPECT,
        (3, _) => &PLAIN_POSSIBLE_GOOD_SIGNING,
        (_, 3) => &PLAIN_SOME_POTENTIAL,
        (2, _) => &PLAIN_USEFUL_ADDITION,
        (_, 2) => &PLAIN_USEFUL_IN_THE_FUTURE,
        (1, _) => &PLAIN_UNLIKELY_TO_IMPROVE,
        _ => &PLAIN_NOT_GOOD_ENOUGH,
    }
}

/// A staff name reference as `read_staff_name_ref_from_news` leaves it: the three name ids, the staff id (+0xc) and
/// the date (+0x10).
struct ReportNameRef([u8; 0x18]);

impl ReportNameRef {
    unsafe fn read(news: usize, idx: &mut u8) -> Self {
        let mut r = ReportNameRef([0; 0x18]);
        exe::date_date_54a110(r.at(0x10));
        exe::read_staff_name_ref_from_news(news, idx, r.0.as_mut_ptr());
        r
    }

    fn at(&self, k: usize) -> usize {
        self.0.as_ptr() as usize + k
    }

    unsafe fn id(&self) -> i32 {
        read_i32(self.at(0xc))
    }

    /// The name in form `form` (1 full, 5 short) into a 0xc8-byte buffer.
    unsafe fn format(&self, buf: &mut [u8; 0xc8], form: u32) {
        exe::database_format_name_from_ids(read_u32(self.at(0)), read_u32(self.at(4)), read_u32(self.at(8)), buf.as_mut_ptr(), 0xc8, form, self.id() as u32);
    }

    /// The id is set and the person's names and date still match.
    unsafe fn matches(&self) -> bool {
        self.id() > -1
            && exe::database_staff_matches_names(staff_at(self.id() as isize), read_u32(self.at(0)), read_u32(self.at(4)), read_u32(self.at(8)), self.at(0x10)) != 0
    }
}

/// The slots of a scout report news item (0x138b), in the order `write_player_report_news_text` reads them.
struct PlayerReportNews {
    /// slot 0: the search's slot of the scout manager.
    search: i8,
    scout: ReportNameRef,
    age: i8,
    /// the position mask (`exe::write_position_name`).
    position: u32,
    player: ReportNameRef,
    /// the player's club id (-1: none).
    club: i32,
    /// the scout's ratings, 0..5: ability now and potential.
    ability: i8,
    potential: i8,
    /// the search the player came from: nation id, region, competition id, youth search.
    nation: i32,
    region: i8,
    comp: i32,
    youth: u32,
    /// the scout watched him in a match (`record` follows).
    watched: u32,
    /// that match was a reserve match.
    reserve: u32,
    /// a report the manager asked for on this player ("Scout report on ...").
    direct: u32,
    /// the work permit paragraph: 1 the outlook, 2 the passport that frees him from it, otherwise none.
    permit: i8,
    permit_easy: u32,
    /// the strengths and weaknesses the scout noticed (`FN_PLAYER_SEARCH_WRITE_TRAITS_REPORT_TEXT`).
    strengths: u32,
    weaknesses: u32,
    /// the match record (`exe::read_match_record_from_news`, 0x1f bytes; `RecordMatch`).
    record: [u8; 0x20],
    /// the slot after it is 0x420e9: the position is described by his strengths (`qualities`).
    describe: bool,
    qualities: u32,
}

impl PlayerReportNews {
    unsafe fn read(news: usize) -> Self {
        let mut idx: u8 = 1;
        let next = |idx: &mut u8| {
            let k = *idx;
            *idx = idx.wrapping_add(1);
            exe::newsitem_get_slot(news, k as u32)
        };
        let search = exe::newsitem_get_slot(news, 0) as i8;
        let scout = ReportNameRef::read(news, &mut idx);
        let age = next(&mut idx) as i8;
        let position = next(&mut idx);
        let player = ReportNameRef::read(news, &mut idx);
        let club = next(&mut idx) as i32;
        let ability = next(&mut idx) as i8;
        let potential = next(&mut idx) as i8;
        let nation = next(&mut idx) as i32;
        let region = next(&mut idx) as i8;
        let comp = next(&mut idx) as i32;
        let youth = next(&mut idx);
        let watched = next(&mut idx);
        let reserve = next(&mut idx);
        let direct = next(&mut idx);
        let permit = next(&mut idx) as i8;
        let permit_easy = next(&mut idx);
        let strengths = next(&mut idx);
        let weaknesses = next(&mut idx);
        let mut record = [0u8; 0x20];
        exe::date_date_54a110(record.as_mut_ptr() as usize);
        if watched != 0 {
            exe::read_match_record_from_news(news, &mut idx, record.as_mut_ptr());
        }
        let describe = next(&mut idx) == 0x420e9;
        let qualities = next(&mut idx);
        PlayerReportNews { search, scout, age, position, player, club, ability, potential, nation, region, comp, youth, watched, reserve,
            direct, permit, permit_easy, strengths, weaknesses, record, describe, qualities }
    }

    fn rec(&self) -> RecordMatch {
        RecordMatch(self.record.as_ptr() as usize)
    }

    /// The match's stage into `stage` and its gender: "reserve" (gender 100) for a reserve match; otherwise, when the
    /// record names a round or a stage (+0xc, +0xe), the label (form 1, no competition) with "Friendly" lowered;
    /// otherwise none.
    unsafe fn stage(&self, stage: &mut [u8; 0xc8]) -> Option<i32> {
        let s = stage.as_mut_ptr() as usize;
        let rec = self.rec().0;
        if self.reserve != 0 {
            story_lang!(s, 0xa74bd0); // "reserve"
            return Some(0x64);
        }
        if read_u16(rec + 0xc) == 0 && read_u16(rec + 0xe) == 0 {
            return None;
        }
        let (round, part) = (read_u32(rec + 0xc), read_u32(rec + 0xe));
        exe::comp_util_format_comp_stage_round_label(0usize, round, part, 1, stage.as_mut_ptr(), 0xc8);
        if praise_same_text(s, INTL_FRIENDLY) {
            write_u8(s, b'f'); // "Friendly"
        }
        Some(exe::comp_util_get_label_gender(0, round, part, 1) as i8 as i32)
    }
}

/// The names and texts both report bodies share.
struct ReportTexts {
    /// the scout's and the player's names, full (form 1) and short (form 5).
    scout: [u8; 0xc8],
    scout_short: [u8; 0xc8],
    player: [u8; 0xc8],
    player_short: [u8; 0xc8],
    /// the position (`exe::write_position_name`, or `exe::write_player_description`) and its gender.
    position: [u8; 0xc8],
    position_gender: i32,
    /// the verdict sentence.
    text: [u8; 0x7d0],
}

impl ReportTexts {
    fn p(b: &[u8]) -> usize {
        b.as_ptr() as usize
    }
}

/// Writes the headline or the body of a scout's report on one player (news item 0x138b).
///
/// `player_search_write_player_report_news_text(news, body, out)` reads the slots (`PlayerReportNews`), clears `out`
/// and names the player (forms 5 and 1).
/// - Headline (`report_headline`): "Scout report on <player>" for a report the manager asked for; otherwise
///   "Recommended player in <place> search" by the search (nation, region, competition), "Recommended youth player"
///   or "Recommended player".
/// - Body: the scout's name (forms 1 and 5), the position (`exe::write_position_name`; when the slot after the record
///   is 0x420e9 and the ability is 3..5, `exe::write_player_description` instead, with its gender). Then the verdict
///   by the ability and potential ratings: from a watched match (`watched_report`) or without one (`plain_report`).
///   Then the work permit paragraph (`work_permit_paragraph`), appended when not empty; the links to the scout and
///   the player (0x7d7, both forms each) when their names still match.
/// Without the news item or the Text: the box of line 0x161b and 0. Otherwise 1.
/// Original: `player_search_write_player_report_news_text` (0x7b95c0, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_player_report_news_text(news: usize, body: u32, out: usize) -> u32 {
    if news == 0 || out == 0 {
        show_error_at(0xa701b0, 0x161b);
        return 0;
    }
    let r = PlayerReportNews::read(news);
    exe::text_clear(out as *mut usize);
    let mut t = ReportTexts { scout: [0; 0xc8], scout_short: [0; 0xc8], player: [0; 0xc8], player_short: [0; 0xc8], position: [0; 0xc8],
        position_gender: 0, text: [0; 0x7d0] };
    r.player.format(&mut t.player_short, 5);
    r.player.format(&mut t.player, 1);
    if body == 0 {
        covered!("r7_03_1 report headline");
        report_headline(&r, &t, out);
        return 1;
    }
    r.scout.format(&mut t.scout, 1);
    r.scout.format(&mut t.scout_short, 5);
    let mut g = exe::write_position_name(r.position, ReportTexts::p(&t.position), 0xc8, 1);
    if r.describe && matches!(r.ability, 3..=5) {
        g = exe::write_player_description(r.player.id(), r.position, r.qualities, ReportTexts::p(&t.position), 0xc8, g as u32, 1);
    }
    t.position_gender = g as i8 as i32;
    if r.watched != 0 {
        covered!("r7_03_1 watched report");
        watched_report(&r, &mut t, out);
    } else {
        covered!("r7_03_1 plain report");
        plain_report(&r, &mut t, out);
    }
    work_permit_paragraph(&r, &t, out);
    if r.scout.matches() {
        let p = staff_at(r.scout.id() as isize);
        exe::text_add_link(out, t.scout.as_ptr(), 0x7d7, p);
        exe::text_add_link(out, t.scout_short.as_ptr(), 0x7d7, p);
    }
    if r.player.matches() {
        let p = staff_at(r.player.id() as isize);
        exe::text_add_link(out, t.player.as_ptr(), 0x7d7, p); // r7_03_1: the player's links
        exe::text_add_link(out, t.player_short.as_ptr(), 0x7d7, p);
    }
    1
}

/// The verdict after a watched match, set as the text: "Scout <scout> ... <age> year old <position> <player> ...,
/// after watching him in <club>'s <score> (<stage>) win against / draw with / defeat against <opponent>." The
/// score is "<higher>-<lower>"; win, draw or defeat by the player's club's goals (+0x19) against the opponent's
/// (+0x1a). Links: the stage to the record's competition (0x7d0), the player's club (0x7d5), the score to the
/// fixture (0x7d3, `RecordMatch::link_score`) and the opponent (0x7d5).
unsafe fn watched_report(r: &PlayerReportNews, t: &mut ReportTexts, out: usize) {
    let rec = r.rec();
    let (goals, against) = rec.goals();
    let mut score = [0u8; 0xc8];
    story_crt!(score.as_mut_ptr(), 0x98c114, goals.max(against) as i32, goals.min(against) as i32); // "%d-%d"
    let k = if goals > against { 0 } else if goals == against { 1 } else { 2 }; // r7_03_1: win, draw, defeat
    let team = club_at(read_i32(rec.0 + 0x11) as isize);
    let opponent = rec.opponent();
    let verdict = watched_verdict(r.ability, r.potential);
    let (pg, tg, og) = (t.position_gender, team.gender_name() as i32, opponent.gender_name() as i32);
    let (sc, ps, pl, sp) = (ReportTexts::p(&t.scout), ReportTexts::p(&t.position), ReportTexts::p(&t.player), score.as_ptr() as usize);
    let buf = t.text.as_mut_ptr() as usize;
    let mut stage = [0u8; 0xc8];
    let staged = r.stage(&mut stage);
    match staged {
        None => story_lang!(buf, verdict[0][k], pg, pg, tg, tg, og, og, sc, r.age as i32, ps, pl, team.name_addr(), sp, opponent.name_addr()),
        Some(sg) => story_lang!(buf, verdict[1][k], pg, pg, tg, tg, sg, og, og, sc, r.age as i32, ps, pl, team.name_addr(), sp, stage.as_ptr() as usize,
                                opponent.name_addr()),
    }
    exe::text_set_string(out, t.text.as_ptr());
    if staged.is_some() {
        exe::text_add_link(out, stage.as_ptr(), 0x7d0, rec.comp());
    }
    exe::text_add_link(out, team.name_addr() as *const u8, 0x7d5, team);
    rec.link_score(out, &score);
    exe::text_add_link(out, opponent.name_addr() as *const u8, 0x7d5, opponent);
}

/// The verdict without a match: "Scout <scout> ... <age> year old <position> <player> (of <club>) ...". When the
/// club is not -1 and a strength or weakness is set (low words), the traits paragraph
/// (`FN_PLAYER_SEARCH_WRITE_TRAITS_REPORT_TEXT`, with the club's reputation and the short names) follows after a
/// blank line. The whole goes through the game's text buffer into `out`, and the club (when set) is linked (0x7d5).
unsafe fn plain_report(r: &PlayerReportNews, t: &mut ReportTexts, out: usize) {
    let verdict = plain_verdict(r.ability, r.potential);
    let pg = t.position_gender;
    let (sc, ps, pl) = (ReportTexts::p(&t.scout), ReportTexts::p(&t.position), ReportTexts::p(&t.player));
    let buf = t.text.as_mut_ptr() as usize;
    let club = club_at(r.club as isize);
    if r.club > -1 {
        let cg = club.gender_name() as i32;
        story_lang!(buf, verdict[0], pg, pg, cg, cg, sc, r.age as i32, ps, pl, club.name_addr());
    } else {
        story_lang!(buf, verdict[1], pg, pg, sc, r.age as i32, ps, pl);
    }
    let mut traits = [0u8; 0x400];
    if r.club != -1 && (r.strengths as u16 != 0 || r.weaknesses as u16 != 0) {
        let f: unsafe extern "C" fn(u32, u32, u32, u32, usize, usize, usize) = core::mem::transmute(FN_PLAYER_SEARCH_WRITE_TRAITS_REPORT_TEXT);
        f(r.strengths, r.weaknesses, read_u16(club.addr() + club::REPUTATION) as u32, r.age as u8 as u32, traits.as_mut_ptr() as usize,
          ReportTexts::p(&t.scout_short), ReportTexts::p(&t.player_short));
    }
    if traits[0] != 0 {
        story_crt!(NEWS_TEXT_BUFFER, 0x991948, buf, traits.as_ptr() as usize); // "%s\n\n%s"
    } else {
        story_crt!(NEWS_TEXT_BUFFER, 0x991944, buf); // "%s"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if r.club > -1 {
        exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
    }
}

/// The work permit paragraph into the game's text buffer, appended to `out` when not empty. 1: "no problem getting a
/// work permit" when the slot says so; otherwise "some problems" for a player rated 4 or 5 (ability or potential),
/// "difficulty" for the others. 2: "<player> would not require a work permit because he holds a(n) <nation>
/// passport", by his second nation ("British" for England, Wales, Scotland and Northern Ireland; the adjective, form
/// 0x15, otherwise). Any other value: no paragraph (the buffer is emptied).
unsafe fn work_permit_paragraph(r: &PlayerReportNews, t: &ReportTexts, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    match r.permit {
        1 => {
            let tpl = if r.permit_easy != 0 {
                0xa711d0 // "no problem"
            } else if matches!(r.ability, 4 | 5) || matches!(r.potential, 4 | 5) {
                0xa71124 // "some problems"
            } else {
                0xa71180 // "difficulty"
            };
            story_lang!(buf, tpl);
        }
        2 => {
            covered!("r7_03_1 passport");
            let nation = staff_at(r.player.id() as isize).second_nation();
            let id = read_u32(nation.addr());
            let mut adjective = [0u8; 0xc8];
            if [KEY_NATION_ENGLAND, KEY_NATION_WALES, KEY_NATION_SCOTLAND, KEY_NATION_NORTHERN_IRELAND].iter().any(|&k| id == read_u32(k)) {
                story_lang!(adjective.as_mut_ptr(), 0xa712ac, EMPTY_STRING); // "British"
            } else {
                exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0xc8, 0x15);
            }
            story_lang!(buf, 0xa71224, 0x64, 0x64, ReportTexts::p(&t.player_short), adjective.as_ptr() as usize);
        }
        _ => {
            write_u8(buf, 0);
            return;
        }
    }
    if read_u8(buf) != 0 {
        exe::text_append_string(out, buf);
    }
}

/// The headline into the game's text buffer, set as the text of `out`: "Scout report on <player>" (short name) for a
/// report the manager asked for; otherwise by the search: "Recommended player in <nationality> search" (the
/// adjective, form 1), "... <region> search" (gender 100), "... <competition> search", "Recommended youth player" or
/// "Recommended player". For a nation, region or competition, when the scout's names still match, the place's text
/// links to the search (0x7e4) if the search's scout is this scout (`link_report_search`).
unsafe fn report_headline(r: &PlayerReportNews, t: &ReportTexts, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let mut place = [0u8; 0xc8];
    let pl = place.as_mut_ptr() as usize;
    if r.direct != 0 {
        story_lang!(buf, 0xa710f4, ReportTexts::p(&t.player_short)); // "Scout report on <player>"
    } else if r.nation > -1 {
        let n = Nation(read_usize(G_NATION_TABLE).wrapping_add((r.nation as usize).wrapping_mul(nation::SIZE)));
        exe::nation_get_nationality_adjective(n, pl, 0xc8, 1);
        let g = n.gender_name() as i8 as i32;
        story_lang!(buf, 0xa710c0, g, g, pl);
    } else if r.region > -1 {
        exe::get_region_name(r.region as u32, pl, 0xc8, 1);
        story_lang!(buf, 0xa7108c, 0x64, 0x64, pl);
    } else if r.comp > -1 {
        let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((r.comp as usize).wrapping_mul(0x6b));
        copy_c_string(pl, c + competition::NAME);
        let g = read_i8(c + competition::GENDER_NAME) as i32;
        story_lang!(buf, 0xa71058, g, g, pl);
    } else {
        story_lang!(buf, if r.youth != 0 { 0xa7103c } else { 0xa71028 }); // "Recommended (youth) player"
    }
    exe::text_set_string(out, buf as *const u8);
    if r.direct == 0 && (r.nation > -1 || r.region > -1 || r.comp > -1) && r.scout.matches() {
        link_report_search(r, pl, out);
    }
}

/// The place's text links to the search (0x7e4, the search's slot) when the scout the search names (+0x180, while
/// his names +0x174..+0x17c and date +0x184 still match) is the report's scout.
unsafe fn link_report_search(r: &PlayerReportNews, place: usize, out: usize) {
    let s = exe::scout_manager_get_search(read_usize(G_SCOUT_MANAGER), r.search as u32, 0usize);
    let id = read_i32(s + 0x180);
    let mut scout = Staff(0);
    if id > -1 {
        let p = staff_at(id as isize);
        if exe::database_staff_matches_names(p, read_u32(s + 0x174), read_u32(s + 0x178), read_u32(s + 0x17c), s + 0x184) != 0 {
            scout = p;
        }
    }
    if scout == staff_at(r.scout.id() as isize) {
        exe::text_add_link(out, place as *const u8, 0x7e4, r.search as i32 as usize);
    }
}

/// Comparer for `write_player_report_news_text` (`compare_text_writer`); the answers must agree too.
pub(crate) unsafe extern "C" fn compare_write_player_report_news_text(news: usize, body: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I0%%);
    let (mut ro, mut rr) = (0, 0);
    compare_text_writer(out, || ro = o(news, body, out), |t| rr = write_player_report_news_text(news, body, t),
        || format!("write_player_report_news_text(body {}, player {})", body, exe::newsitem_get_slot(news, 12) as i32));
    SCOREBOARD.record(ro == rr, || format!("write_player_report_news_text(body {}): original {} rust {}", body, ro, rr));
    ro
}
