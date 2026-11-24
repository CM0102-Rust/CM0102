// --- sbB_2: the text of a scout report news item (scout_manager.cpp) ---

/// The slots of a scout report news item (0x138e), in the order `send_scout_report_news` writes them and
/// `write_scout_report_news_text` reads them.
struct ScoutReportNews {
    /// slot 0: how many scouts report (up to 3 from the sender; the copy reads at most 10, the original's array).
    count: i32,
    /// slot 1: `scout_report::REPORT_MODE` (2: the player impressed).
    mode: i32,
    /// the player's name ref (four ids, `read_staff_name_ref_from_news`) and the birth date behind it.
    person: [u8; 0x18],
    age: i8,
    position: u32,
    /// the club the report is for (`scout_report::CLUB_ID`).
    club_id: i32,
    /// the match record (`read_match_record_from_news`) and whether there was a match.
    game: BanMatch,
    has_match: u32,
    /// `scout_report::LEAGUE_MATCH`: the match was a reserve one ("reserve" as its stage).
    reserve_match: u32,
    /// `scout_report::ELIGIBILITY_NOTE`: 1 the work permit, 2 the passport.
    eligibility: i8,
    work_permit: u32,
    /// each scout: his name ref (three name ids and his id).
    scouts: [[u32; 4]; 10],
    /// each scout's two verdicts: the report value / 8 and % 8 (signed).
    verdict: [i8; 10],
    other_verdict: [i8; 10],
    /// the slot after the scouts is not the 0x420e9 marker (an older news item).
    unmarked: bool,
    strengths: u32,
}

impl ScoutReportNews {
    unsafe fn read(news: usize) -> Self {
        let mut idx: u8 = 0;
        let next = |idx: &mut u8| {
            let k = *idx;
            *idx = idx.wrapping_add(1);
            exe::newsitem_get_slot(news, k as u32)
        };
        let count = next(&mut idx) as i32;
        let mode = next(&mut idx) as i32;
        let mut person = [0u8; 0x18];
        person[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
        exe::read_staff_name_ref_from_news(news, &mut idx, person.as_mut_ptr());
        let age = next(&mut idx) as i8;
        let position = next(&mut idx);
        let club_id = next(&mut idx) as i32;
        let mut game = BanMatch([0u8; 0x20]);
        game.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
        exe::read_match_record_from_news(news, &mut idx, game.0.as_mut_ptr());
        let has_match = next(&mut idx);
        let reserve_match = next(&mut idx);
        let eligibility = next(&mut idx) as i8;
        let work_permit = next(&mut idx);
        let mut r = ScoutReportNews { count, mode, person, age, position, club_id, game, has_match, reserve_match, eligibility,
            work_permit, scouts: [[0; 4]; 10], verdict: [0; 10], other_verdict: [0; 10], unmarked: false, strengths: 0 };
        // ponytail: the original's arrays hold 10 scouts; the sender writes at most 3
        for k in 0..count.clamp(0, 10) as usize {
            let id = next(&mut idx);
            r.scouts[k] = [next(&mut idx), next(&mut idx), next(&mut idx), id];
            let v = next(&mut idx) as i32;
            r.verdict[k] = (v / 8) as i8;
            r.other_verdict[k] = (v % 8) as i8;
        }
        r.unmarked = next(&mut idx) != 0x420e9;
        r.strengths = next(&mut idx);
        r
    }

    unsafe fn player(&self) -> Staff {
        staff_at(read_i32(self.person.as_ptr() as usize + 0xc) as isize)
    }

    /// The player's name (style 1 full, 5 short) into a 0xc8-byte buffer.
    unsafe fn player_name(&self, buf: &mut [u8; 0xc8], style: u32) {
        let p = self.person.as_ptr() as usize;
        exe::database_format_name_from_ids(read_u32(p), read_u32(p + 4), read_u32(p + 8), buf.as_mut_ptr(), 0xc8, style, read_u32(p + 0xc));
    }

    /// The player's id is set and his names still match.
    unsafe fn player_matches(&self) -> bool {
        let p = self.person.as_ptr() as usize;
        read_i32(p + 0xc) > -1 && exe::database_staff_matches_names(self.player(), read_u32(p), read_u32(p + 4), read_u32(p + 8), p + 0x10) != 0
    }

    /// Scout `k`'s name (style 1 full, 5 short) into a 0xc8-byte buffer.
    unsafe fn scout_name(&self, k: usize, buf: usize, style: u32) {
        let s = self.scouts[k];
        exe::database_format_name_from_ids(s[0], s[1], s[2], buf as *mut u8, 0xc8, style, s[3]);
    }

    /// The report's club (`club_id`).
    unsafe fn club(&self) -> Club {
        club_at(self.club_id as isize)
    }
}

/// The opinion of one scout of a category (`scout_category` 0..9) in a multi-scout report; category 10 is the "not
/// good enough" one.
const SCOUT_OPINION_TEXTS: [u32; 10] = [0xa7c4cc, 0xa7c458, 0xa7c3f8, 0xa7c3b0, 0xa7c358, 0xa7c308, 0xa7c2b8, 0xa7c258, 0xa7c1e8, 0xa7c18c];
/// The opinion of several scouts of a category (0..9): (two scouts, more than two).
const SCOUTS_OPINION_TEXTS: [(u32, u32); 10] = [(0xa7bc0c, 0xa7bc78), (0xa7bae0, 0xa7bb70), (0xa7b9dc, 0xa7ba58), (0xa7b904, 0xa7b968),
    (0xa7b80c, 0xa7b880), (0xa7b730, 0xa7b798), (0xa7b64c, 0xa7b6b8), (0xa7b548, 0xa7b5c4), (0xa7b41c, 0xa7b4a8), (0xa7b310, 0xa7b390)];

/// The list of 2, 3, 4, 5, 6 scout names ("A and B", "A, B and C", ...), and of 7 for any other number.
const SCOUT_NAME_LISTS: [u32; 5] = [0xa7c0c0, 0xa7c04c, 0xa7bfb4, 0xa7bef4, 0xa7be04];
const SCOUT_NAME_LIST_7: u32 = 0xa7bcf0;

/// The watch sentence by the result (win, draw, defeat) of a match without a stage: [one scout, several][mode 2 or not].
const SCOUT_MATCH_TEXTS: [[[u32; 3]; 2]; 2] = [
    [[0xa7dae4, 0xa7d7d8, 0xa7d4c0], [0xa7da34, 0xa7d728, 0xa7d404]], // "<player> was spotted / watched by scout <scout> during ..."
    [[0xa7d954, 0xa7d648, 0xa7d318], [0xa7d888, 0xa7d57c, 0xa7d240]], // "<three> of your scouts were impressed with / watched ..."
];

/// The same with the match's stage: [one scout, several][mode 2 or not].
const SCOUT_STAGE_MATCH_TEXTS: [[[u32; 3]; 2]; 2] = [
    [[0xa7d180, 0xa7ce34, 0xa7cae0], [0xa7d0c0, 0xa7cd74, 0xa7ca14]],
    [[0xa7cfd0, 0xa7cc88, 0xa7c918], [0xa7cef4, 0xa7cbac, 0xa7c830]],
];

/// Writes the headline or body of a scout report news item (0x138e): who watched the player and where, what each
/// scout thinks of him and whether he can play here.
///
/// `SCOUT_MANAGER::write_scout_report_news_text(news, body, out)` reads the slots (`ScoutReportNews`). The player's
/// name in full (style 1) and short (style 5), always. Headline (short name): one scout "<player> impresses scout"
/// (0xa7dd2c, mode 2) or "Scout report on <player>" (0xa710f4); otherwise "Scouts impressed by" (0xa7dcf8, mode 2)
/// or "Scouts watch" (0xa7dccc); the player's link (0x7d7) when his names match. Body, into `NEWS_TEXT_BUFFER`:
/// - the position's name (`exe::write_position_name`, its gender); for a news item with the 0x420e9 marker and the
///   first scout's verdicts 3..5, the player's description instead (`exe::write_player_description`);
/// - the watch sentence (`scout_watch_sentence`), set as the text; with a stage, the stage's link (0x7d0) to the
///   competition;
/// - the opinions, appended (`lone_scout_opinion`, `scouts_opinions`);
/// - the eligibility, appended when written (`scout_eligibility_note`);
/// - the links: the player in full and short (0x7d7) when his names match; with a match the first club (0x7d5), the
///   score (0x7d3) when the fixture still matches (`exe::match_stats_fixture_entry_matches`) and the second club;
///   each scout in full and short when his names match.
/// Original: `SCOUT_MANAGER::write_scout_report_news_text` (0x7f9d10, scout_manager.cpp, thiscall, ret 0xc; `this` is not
/// used). Defect of the original: with several scouts, only the first check of each scout's category (verdict 5)
/// reads his own verdicts; the others read the first scout's.
pub(crate) unsafe extern "thiscall" fn write_scout_report_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let r = ScoutReportNews::read(news);
    let buf = NEWS_TEXT_BUFFER;
    let (mut full, mut short) = ([0u8; 0xc8], [0u8; 0xc8]);
    r.player_name(&mut full, 1);
    r.player_name(&mut short, 5);
    let sp = short.as_ptr() as usize;
    if body == 0 {
        covered!("sbB_2 scout report headline");
        let tpl = match (r.count == 1, r.mode == 2) {
            (true, true) => 0xa7dd2c,   // "<player> impresses scout"
            (true, false) => 0xa710f4,  // "Scout report on <player>"
            (false, true) => 0xa7dcf8,  // "Scouts impressed by <player>"
            (false, false) => 0xa7dccc, // "Scouts watch <player>"
        };
        story_lang!(buf, tpl, sp);
        exe::text_set_string(out, buf as *const u8);
        if r.player_matches() {
            exe::text_add_link(out, short.as_ptr(), 0x7d7, r.player());
        }
        return;
    }
    covered!("sbB_2 scout report body");
    let mut position = [0u8; 0xc8];
    let pp = position.as_mut_ptr() as usize;
    let mut gender = exe::write_position_name(r.position, pp, 0xc8, 1);
    if !r.unmarked && ((3..=5).contains(&r.verdict[0]) || (3..=5).contains(&r.other_verdict[0])) {
        // the original passes a dword whose low byte is the gender and the rest the position mask's upper bytes
        gender = exe::write_player_description(read_i32(r.person.as_ptr() as usize + 0xc), r.position, r.strengths, pp, 0xc8,
            (r.position & 0xffff_ff00) | gender as u32, 1);
    }
    let words = ScoutWatchWords { full: full.as_ptr() as usize, position: pp, gender: gender as i8 as i32 };
    let mut score = [0u8; 0xc8];
    scout_watch_sentence(&r, &words, &mut score, out);
    if r.count == 1 {
        lone_scout_opinion(&r, out);
    } else {
        scouts_opinions(&r, sp, out);
    }
    scout_eligibility_note(&r, sp, out);
    if r.player_matches() {
        exe::text_add_link(out, full.as_ptr(), 0x7d7, r.player());
        exe::text_add_link(out, short.as_ptr(), 0x7d7, r.player());
    }
    if r.has_match != 0 {
        let (c1, c2) = (Club(r.game.club()), Club(r.game.opponent()));
        exe::text_add_link(out, (c1.addr() + club::NAME) as *const u8, 0x7d5, c1);
        let g = r.game.0.as_ptr() as usize;
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), r.game.fixture() as i32, read_i32(g + 0x11), read_i32(g + 0x15),
            read_i32(g), read_i32(g + 2)) != 0 {
            exe::text_add_link(out, score.as_ptr(), 0x7d3, r.game.fixture() as usize);
        }
        exe::text_add_link(out, (c2.addr() + club::NAME) as *const u8, 0x7d5, c2);
    }
    let (mut s_short, mut s_full) = ([0u8; 0xc8], [0u8; 0xc8]);
    for k in 0..r.count.clamp(0, 10) as usize {
        r.scout_name(k, s_short.as_mut_ptr() as usize, 5);
        r.scout_name(k, s_full.as_mut_ptr() as usize, 1);
        let s = r.scouts[k];
        let person = staff_at(s[3] as i32 as isize);
        if (s[3] as i32) > -1 && exe::database_staff_matches_names(person, s[0], s[1], s[2], 0usize) != 0 {
            exe::text_add_link(out, s_full.as_ptr(), 0x7d7, person);
            exe::text_add_link(out, s_short.as_ptr(), 0x7d7, person);
        }
    }
}

/// The player's full name, his position's text and its gender, for the watch sentence.
struct ScoutWatchWords {
    full: usize,
    position: usize,
    gender: i32,
}

/// The first paragraph of the body, set as the text: who watched the player, and the match (its score "<more>-<fewer>"
/// into `score`, for the link).
///
/// Without a match: "<scout> has been watching <age> year old <position> <player> during recent weeks." (0xa7dc34) or
/// "<three> of your scouts have been watching ..." (0xa7db94, the count in words, `exe::utils_number_to_words`). With a
/// match, the score and the result for the first club (win, draw, defeat; goals compared signed): without a round,
/// a stage or a reserve flag `SCOUT_MATCH_TEXTS`; otherwise the stage's name (reserve: "reserve" with gender 0x64; else
/// `exe::comp_util_format_comp_stage_round_label`, "Friendly" lower-cased, its gender
/// `exe::comp_util_get_label_gender`) and `SCOUT_STAGE_MATCH_TEXTS`, then the stage's link (0x7d0) to the competition.
/// The clubs by their gender and full name; one scout by his full name, several by the count in words with the age,
/// the position and its gender. The draws with a stage take the stage's gender twice, except the several-scouts mode 2
/// one.
unsafe fn scout_watch_sentence(r: &ScoutReportNews, w: &ScoutWatchWords, score: &mut [u8; 0xc8], out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let one = r.count == 1;
    let mut scout = [0u8; 0xc8];
    let mut count_words = [0u8; 0x32];
    let (sc, cw) = (scout.as_mut_ptr() as usize, count_words.as_mut_ptr() as usize);
    let who = |scout_name: bool| {
        if scout_name {
            r.scout_name(0, sc, 1);
        } else {
            exe::utils_number_to_words(r.count, cw as *mut u8, 0x32, 0);
        }
    };
    let (pg, age) = (w.gender, r.age as i32);
    if r.has_match == 0 {
        covered!("sbB_2 no match");
        if one {
            who(true);
            story_lang!(buf, 0xa7dc34, pg, pg, sc, age, w.position, w.full);
        } else {
            who(false);
            story_lang!(buf, 0xa7db94, pg, pg, cw, age, w.position, w.full);
        }
        exe::text_set_string(out, buf as *const u8);
        return;
    }
    let g = r.game.0.as_ptr() as usize;
    let staged = read_u16(g + 0xc) != 0 || read_u16(g + 0xe) != 0 || r.reserve_match != 0;
    let scp = score.as_mut_ptr() as usize;
    let (goals1, goals2) = r.game.goals();
    story_crt!(scp, 0x98c114, goals1.max(goals2) as i32, goals1.min(goals2) as i32); // "%d-%d"
    let (c1, c2) = (Club(r.game.club()), Club(r.game.opponent()));
    let (g1, g2) = (c1.gender_name() as i32, c2.gender_name() as i32);
    let (n1, n2) = (c1.addr() + club::NAME, c2.addr() + club::NAME);
    let result = if goals1 > goals2 { 0 } else if goals1 == goals2 { 1 } else { 2 };
    let mode = (r.mode != 2) as usize;
    if !staged {
        covered!("sbB_2 match");
        if one {
            who(true);
            story_lang!(buf, SCOUT_MATCH_TEXTS[0][mode][result], 0x64, 0x64, g1, g1, g2, g2, w.full, sc, n1, scp, n2);
        } else {
            who(false);
            story_lang!(buf, SCOUT_MATCH_TEXTS[1][mode][result], pg, pg, g1, g1, g2, g2, cw, age, w.position, w.full, n1, scp, n2);
        }
        exe::text_set_string(out, buf as *const u8);
        return;
    }
    covered!("sbB_2 match with a stage");
    let mut stage = [0u8; 0xc8];
    let stp = stage.as_mut_ptr() as usize;
    let sg = if r.reserve_match != 0 {
        story_lang!(stp, 0xa74bd0); // "reserve"
        0x64
    } else {
        let (round, phase) = (read_u32(g + 0xc), read_u32(g + 0xe));
        exe::comp_util_format_comp_stage_round_label(0usize, round, phase, 1, stage.as_mut_ptr(), 0xc8);
        if stage[..9] == *b"Friendly\0" {
            stage[0] = b'f'; // "Friendly" -> "friendly"
        }
        exe::comp_util_get_label_gender(0, round, phase, 1) as i8 as i32
    };
    let tpl = SCOUT_STAGE_MATCH_TEXTS[!one as usize][mode][result];
    if one {
        who(true);
        if result == 1 {
            story_lang!(buf, tpl, 0x64, 0x64, g1, g1, sg, sg, g2, g2, w.full, sc, n1, scp, stp, n2);
        } else {
            story_lang!(buf, tpl, 0x64, 0x64, g1, g1, sg, g2, g2, w.full, sc, n1, scp, stp, n2);
        }
    } else {
        who(false);
        if result == 1 && mode == 1 {
            story_lang!(buf, tpl, pg, pg, g1, g1, sg, sg, g2, g2, cw, age, w.position, w.full, n1, scp, stp, n2);
        } else {
            story_lang!(buf, tpl, pg, pg, g1, g1, sg, g2, g2, cw, age, w.position, w.full, n1, scp, stp, n2);
        }
    }
    exe::text_set_string(out, buf as *const u8);
    exe::text_add_link(out, stage.as_ptr(), 0x7d0, r.game.comp());
}

/// The opinion of a lone scout, appended, by his two verdicts (a = value / 8, c = value % 8), checked in this order: a 5
/// "recommended as an excellent signing"; c 5 (a 3 or more, or below 3); a 4; c 4; a 3; c 3; then with his short name a
/// 2, c 2, a 1; else "not good enough to play for <club>" with the report club's short gender and short name.
unsafe fn lone_scout_opinion(r: &ScoutReportNews, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let mut scout = [0u8; 0xc8];
    let sc = scout.as_mut_ptr() as usize;
    r.scout_name(0, sc, 5);
    let (a, c) = (r.verdict[0], r.other_verdict[0]);
    covered!("sbB_2 lone scout opinion");
    let tpl = if a == 5 {
        0xa7c800 // "He is recommended as an excellent signing."
    } else if c == 5 {
        if a >= 3 { 0xa7c7ac } else { 0xa7c76c }
    } else if a == 4 {
        0xa7c744
    } else if c == 4 {
        0xa7c70c
    } else if a == 3 {
        0xa7c6dc
    } else if c == 3 {
        0xa7c6ac
    } else {
        let tpl = if a == 2 {
            0xa7c654 // "<scout> feels the player may be a useful addition to your squad."
        } else if c == 2 {
            0xa7c5ec
        } else if a == 1 {
            0xa7c598
        } else {
            let club = r.club();
            let g = club.short_gender_name() as i8 as i32;
            story_lang!(buf, 0xa7c51c, g, g, sc, club.addr() + club::SHORT_NAME); // "<scout> does not believe ... <club>"
            exe::text_append_string(out, buf);
            return;
        };
        story_lang!(buf, tpl, sc);
        exe::text_append_string(out, buf);
        return;
    };
    story_lang!(buf, tpl);
    exe::text_append_string(out, buf);
}

/// The category of each scout of a multi-scout report: 0 his verdict 5, then by the FIRST scout's verdicts (the
/// original's defect) 1 a ≥ 3 and c 5, 2 a < 3 and c 5, 3 a 4, 4 c 4, 5 a 3, 6 c 3, 7 a 2, 8 c 2, 9 a 1, 10 else.
unsafe fn scout_category(r: &ScoutReportNews, k: usize) -> u8 {
    if r.verdict[k] == 5 {
        return 0;
    }
    let (a, c) = (r.verdict[0], r.other_verdict[0]);
    match () {
        _ if a >= 3 && c == 5 => 1,
        _ if a < 3 && c == 5 => 2,
        _ if a == 4 => 3,
        _ if c == 4 => 4,
        _ if a == 3 => 5,
        _ if c == 3 => 6,
        _ if a == 2 => 7,
        _ if c == 2 => 8,
        _ if a == 1 => 9,
        _ => 10,
    }
}

/// The opinions of several scouts, one paragraph per category in order (`scout_category`), appended: one scout of
/// the category by his full name (`SCOUT_OPINION_TEXTS`; category 10 "not good enough to play for <club>", the report
/// club's short gender and short name); several by the list of their full names (`SCOUT_NAME_LISTS`) and
/// `SCOUTS_OPINION_TEXTS` (2 or more than 2; category 10: 0xa7b190 with 0x64 twice, 0xa7b254, with the player's short
/// name and the club). The lone scout's buffer is kept between categories, as the original's.
unsafe fn scouts_opinions(r: &ScoutReportNews, player: usize, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let mut lone = [0u8; 0xc8];
    let scout = lone.as_mut_ptr() as usize;
    let n = r.count.clamp(0, 10) as usize;
    let mut category = [0u8; 10];
    let mut per = [0i8; 11];
    for k in 0..n {
        category[k] = scout_category(r, k);
        per[category[k] as usize] += 1;
    }
    let club = r.club();
    let (cg, cn) = (club.short_gender_name() as i8 as i32, club.addr() + club::SHORT_NAME);
    for cat in 0..11u8 {
        let many = per[cat as usize];
        if many == 1 {
            if let Some(k) = (0..n).find(|&k| category[k] == cat) {
                r.scout_name(k, scout, 1);
            }
            if cat < 10 {
                story_lang!(buf, SCOUT_OPINION_TEXTS[cat as usize], scout);
            } else {
                story_lang!(buf, 0xa7c10c, cg, cg, scout, cn); // "<scout> does not believe ... <club>"
            }
        } else if many > 1 {
            covered!("sbB_2 scouts opinions");
            let mut names = [[0u8; 0xc8]; 10];
            let mut j = 0;
            for k in (0..n).filter(|&k| category[k] == cat) {
                r.scout_name(k, names[j].as_mut_ptr() as usize, 1);
                j += 1;
            }
            let nm = |j: usize| names[j].as_ptr() as usize;
            let mut list = [0u8; 0x3e8];
            let lp = list.as_mut_ptr() as usize;
            match many {
                2 => story_lang!(lp, SCOUT_NAME_LISTS[0], nm(0), nm(1)),
                3 => story_lang!(lp, SCOUT_NAME_LISTS[1], nm(0), nm(1), nm(2)),
                4 => story_lang!(lp, SCOUT_NAME_LISTS[2], nm(0), nm(1), nm(2), nm(3)),
                5 => story_lang!(lp, SCOUT_NAME_LISTS[3], nm(0), nm(1), nm(2), nm(3), nm(4)),
                6 => story_lang!(lp, SCOUT_NAME_LISTS[4], nm(0), nm(1), nm(2), nm(3), nm(4), nm(5)),
                _ => story_lang!(lp, SCOUT_NAME_LIST_7, nm(0), nm(1), nm(2), nm(3), nm(4), nm(5), nm(6)),
            }
            if cat < 10 {
                let (two, more) = SCOUTS_OPINION_TEXTS[cat as usize];
                story_lang!(buf, if many > 2 { more } else { two }, lp);
            } else if many > 2 {
                story_lang!(buf, 0xa7b254, cg, cg, lp, player, cn);
            } else {
                story_lang!(buf, 0xa7b190, 0x64, 0x64, cg, cg, lp, player, cn);
            }
        } else {
            continue;
        }
        exe::text_append_string(out, buf);
    }
}

/// The eligibility paragraph, appended when written: 1 the work permit (granted or not; one scout without names,
/// several with the player's short name when granted); 2 the passport of the player's second nation ("British" for
/// England, Wales, Scotland and Northern Ireland, else its adjective, form 0x15) with his short name; else none.
unsafe fn scout_eligibility_note(r: &ScoutReportNews, player: usize, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    match r.eligibility {
        1 => {
            covered!("sbB_2 work permit");
            match (r.work_permit != 0, r.count == 1) {
                (true, true) => story_lang!(buf, 0xa711d0),
                (true, false) => story_lang!(buf, 0xa7b124, player),
                (false, true) => story_lang!(buf, 0xa71180),
                (false, false) => story_lang!(buf, 0xa7b0d8),
            }
        }
        2 => {
            covered!("sbB_2 passport");
            let nation = r.player().second_nation();
            let key = read_u32(nation.addr());
            let mut adjective = [0u8; 0xc8];
            let adj = adjective.as_mut_ptr() as usize;
            if [KEY_NATION_ENGLAND, KEY_NATION_WALES, KEY_NATION_SCOTLAND, KEY_NATION_NORTHERN_IRELAND].iter().any(|&k| key == read_u32(k)) {
                story_lang!(adj, 0xa712ac, 0xad9c64); // "British"
            } else {
                exe::nation_get_nationality_adjective(nation, adj, 0xc8, 0x15);
            }
            story_lang!(buf, 0xa71224, 0x64, 0x64, player, adj);
        }
        _ => write_u8(buf, 0),
    }
    if read_u8(buf) != 0 {
        exe::text_append_string(out, buf);
    }
}

/// Comparer for `write_scout_report_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_scout_report_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, body, out), |t| write_scout_report_news_text(this, news, body, t),
        || format!("write_scout_report_news_text(body {}, scouts {}, mode {})", body, exe::newsitem_get_slot(news, 0) as i32,
            exe::newsitem_get_slot(news, 1) as i32));
}
