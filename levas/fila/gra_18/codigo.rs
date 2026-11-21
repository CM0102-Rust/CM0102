// ---------- batch %%LEVA%%: the news text of a broken club record (club_records.cpp) ----------

/// The constructors the original runs over its local copy of the broken record (0x38b bytes, the `club_record`
/// layout), in its order: the match records (0x645800: only their Date), the four run records, the tallies and the
/// counts, the goals-in-a-match and rating records, the youngest and oldest player records, the two transfer fee
/// records and the fans' player of the year. All thiscall, returning `this`.
const RECORD_NEWS_CONSTRUCTORS: [(usize, usize); 27] = [
    (0x12, 0x645800), (0x35, 0x645800), (0x58, 0x645800), (0x7d, 0x645800), (0x9c, 0x645800), (0xbb, 0x645800),
    (0xda, 0x645800), (0xf9, 0x645800),
    (0x118, 0x450ad0), (0x12c, 0x450b20), (0x140, 0x450b70), (0x154, 0x450bc0),
    (0x168, 0x450c60), (0x183, 0x450c60), (0x19e, 0x450c10), (0x1d6, 0x450c60), (0x1f1, 0x453c50), (0x210, 0x450c60),
    (0x22b, 0x450d30), (0x247, 0x450d30), (0x261, 0x450d30), (0x27b, 0x450c70), (0x2ba, 0x450cd0), (0x2f9, 0x450d30),
    (0x313, 0x4485d0), (0x33c, 0x448620), (0x371, 0x450d40),
];

/// `read_run_record_from_news(item, &slot, out)` (0x7e5570, record_utils.cpp; cdecl): the inverse of 0x7e5420.
const FN_READ_RUN_RECORD_FROM_NEWS: usize = 0x7e5570;
/// `database_format_years_between_dates(end, start, text, size)` (0x540340, Database.cpp; cdecl).
const FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES: usize = 0x540340;
/// The empty string the original sets the Text to first (0xad9c64).
const EMPTY_STRING: usize = 0xad9c64;

/// A match record inside the local record copy (`read_match_record_from_news`, 0x1f bytes): the date (+0 day, +2
/// year), the club competition (+8), the round and stage words (+0xc, +0xe), the venue (+0x10: 1 at home, 2 away),
/// the club (+0x11) and its opponent (+0x15), their goals (+0x19, +0x1a) and the fixture index (+0x1b).
#[derive(Clone, Copy)]
struct RecordMatch(usize);

impl RecordMatch {
    unsafe fn comp_index(self) -> i32 {
        read_i32(self.0 + 8)
    }
    unsafe fn comp(self) -> usize {
        read_usize(G_CLUB_COMP_TABLE).wrapping_add((self.comp_index() as usize).wrapping_mul(0x6b))
    }
    unsafe fn venue(self) -> u8 {
        read_u8(self.0 + 0x10)
    }
    unsafe fn opponent(self) -> Club {
        club_at(read_i32(self.0 + 0x15) as isize)
    }
    unsafe fn goals(self) -> (i8, i8) {
        (read_i8(self.0 + 0x19), read_i8(self.0 + 0x1a))
    }
    /// The round label (`exe::comp_util_format_comp_stage_round_label`, form 2, 200 bytes) and its gender
    /// (`exe::comp_util_get_label_gender`).
    unsafe fn round_label(self, label: &mut [u8; 0xc8]) -> i32 {
        let (round, stage) = (read_u32(self.0 + 0xc), read_u32(self.0 + 0xe));
        exe::comp_util_format_comp_stage_round_label(self.comp(), round, stage, 2, label.as_mut_ptr(), 0xc8);
        exe::comp_util_get_label_gender(self.comp() as u32, round, stage, 2) as i8 as i32
    }
    /// The score's link (0x7d3) when the fixture still matches (`exe::match_stats_fixture_entry_matches`).
    unsafe fn link_score(self, out: usize, score: &[u8]) {
        let (day, year) = (read_u16(self.0) as i32, read_u16(self.0 + 2) as i32);
        let fixture = read_i32(self.0 + 0x1b);
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), fixture, read_i32(self.0 + 0x11), read_i32(self.0 + 0x15), day, year) != 0 {
            exe::text_add_link(out, score.as_ptr(), 0x7d3, fixture as u32 as usize);
        }
    }
}

/// A record holder's name reference (0x18 bytes: three name ids, the person's id at +0xc, his birth date at +0x10).
#[derive(Clone, Copy)]
struct RecordHolder(usize);

impl RecordHolder {
    /// The holder's name (`exe::database_format_name_from_ids`, mode 1, 200 bytes).
    unsafe fn name(self, buf: &mut [u8; 0xc8]) {
        exe::database_format_name_from_ids(read_u32(self.0), read_u32(self.0 + 4), read_u32(self.0 + 8), buf.as_mut_ptr(), 0xc8, 1, read_u32(self.0 + 0xc));
    }
    /// The person's link (0x7d7) when his names still match (`exe::database_staff_matches_names`).
    unsafe fn link(self, out: usize, name: &[u8; 0xc8]) {
        let staff = staff_at(read_i32(self.0 + 0xc) as isize);
        if exe::database_staff_matches_names(staff, read_u32(self.0), read_u32(self.0 + 4), read_u32(self.0 + 8), self.0 + 0x10) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, staff);
        }
    }
}

/// A sum of money as the game shows it (`Cash::format`, 0x450aa0: `exe::cash_get_display_amount` with `flags`, then
/// `exe::cash_format_money` into 200 bytes).
unsafe fn record_money_text(value: u32, flags: u32, buf: &mut [u8; 0xc8]) {
    let cash = [value];
    exe::cash_format_money(buf.as_mut_ptr() as usize, 0xc8, exe::cash_get_display_amount(cash.as_ptr(), flags), flags);
}

/// The club's link (0x7d5), under the given name.
unsafe fn link_record_club(out: usize, name: usize, club: Club) {
    exe::text_add_link(out, name as *const u8, 0x7d5, club); // gra18: the club's link
}

/// Reads the broken record of type `kind` from the news item's slots into the local record copy `r`: the inverse of
/// `club_records_write_record_news_fields` (`record_news_fields`): bytes, words and dwords stored at their offsets, the
/// rating (type 20) as the slot × 0.01 in a float, the match, holder and run records by their readers; types 25 and 26
/// read the day and the year of the date at +0x27b / +0x2ba into a Date (`exe::new_date`, then
/// `exe::date_operator_3d`). Returns false for a type outside 1..=32.
unsafe fn read_record_news_fields(news: usize, kind: u8, r: usize, k: &mut u8) -> bool {
    use RecordNewsField::*;
    let fields = record_news_fields(kind);
    if fields.is_empty() {
        return false;
    }
    let slot = |k: &mut u8| {
        let v = exe::newsitem_get_slot(news, *k as u32);
        *k = k.wrapping_add(1);
        v
    };
    let mut rest = fields;
    if kind == 25 || kind == 26 {
        covered!("gra_18 record date");
        let base = if kind == 25 { 0x27b } else { 0x2ba };
        let (day, year) = (slot(k), slot(k));
        let mut date = [0u8; 8];
        let mut scratch = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, day, year);
        exe::date_operator_3d(r + base, scratch.as_mut_ptr() as usize, date.as_ptr() as usize);
        rest = &fields[2..];
    }
    let run: unsafe extern "C" fn(usize, *mut u8, usize) -> u32 = core::mem::transmute(FN_READ_RUN_RECORD_FROM_NEWS);
    for f in rest {
        match *f {
            Byte(o) | SByte(o) => write_u8(r + o, slot(k) as u8),
            Word(o) | SWord(o) => write_u16(r + o, slot(k) as u16),
            Dword(o) => write_u32(r + o, slot(k)),
            Percent(o) => write_f32(r + o, (slot(k) as i32 as f64 * read_f64(rdata::F64_0_01)) as f32),
            Match(o) => {
                exe::read_match_record_from_news(news, k, (r + o) as *mut u8);
            }
            Holder(o) => {
                exe::read_staff_name_ref_from_news(news, k, (r + o) as *mut u8);
            }
            Run(o) => {
                run(news, k, r + o);
            }
        }
    }
    true
}

/// Writes the headline or the body of the news that a club broke one of its records (news type 0x3e8, from
/// `send_record_news`).
///
/// `write_record_news_text(news, full, out)`. Without the Text, the error box (club_records.cpp, line 0xee7) and 0;
/// the Text is cleared and set empty; without the news item, the box (0xef3) and 0. The local record copy is
/// constructed as the original does (`RECORD_NEWS_CONSTRUCTORS`); slot 0 is the club id, slot 1 the record type, then
/// the type's fields (`read_record_news_fields`; an unknown type: the box at line 0x1b02, and on). A club id outside
/// the club table: the box (0xefe) and 0.
///
/// The headline (`full` = 0): type 32 "<club> <Fans Player of the Year> decided" (0x98de00, the award name 0x98de58),
/// with the club's link and the award's link (0x7e6, to the club); otherwise "New <club> record" for a national side
/// (`exe::database_is_national_team`, 0x98dde0) or "New <club> club record" (0x98ddb8), with the club's link. 1.
///
/// The body, by type, each only when its record holds (otherwise an empty text), with the club's short name and gender
/// (`club::SHORT_NAME`, `club::SHORT_GENDER_NAME`) and its link:
/// - 1, 2: the final league position (the ordinal of position + 1, `exe::utils_format_ordinal`) in the competition
///   (its short name and gender), record high (0x98dd28) or low (0x98dc98), when the competition is set; the
///   competition's link (0x7d0);
/// - 3, 5: the attendance at home to the opponent in the round (the label of form 2 and its gender), record high
///   (0x98dbf4) or low (0x98da98); 4: the gate receipts (`record_money_text`, flags 0x22, 0x98db3c); the opponent's
///   link and the round label's link (0x7d0, to the competition); 6: the average attendance (0x98da2c);
/// - 7, 8, 9: the record win, league win and defeat: the score "%d-%d" (the C sprintf; the club's goals first, in the
///   defeat the opponent's first), the template by the venue (at home, away, against); 10, 11: the highest scoring
///   game and league game, the score with the higher count first, the template by the result (win, defeat, draw);
///   the links of the opponent, the round and the score (0x7d3, when the fixture still matches);
/// - 12, 13: the runs of wins and defeats (more than one game) between the start and the end dates
///   (`exe::date_get_long_date_string_no_year`); 14, 15: unbeaten and without winning, from the start date;
/// - 16, 17: a player's goals and league goals in the season; 18: the goals in one match (with the match: venue,
///   opponent, round); 19: assists; 20: the average rating (above −1.0, "%4.2f" of the float) with the games; 21: man
///   of the match awards; 22: yellow and red cards; 23, 24: league appearances and goals in his club career; 25, 26:
///   the youngest and oldest appearance (the age by `database_format_years_between_dates` from the birth date to
///   the match, the template by the venue); 27: the most caps for the nation; 28, 29: the record purchase and sale
///   (the fee with flags 6, the other club, "part-exchange" when the flag is set); 32: the fans' player of the year
///   (the club's long name and gender, "Player of the Year" 0x98c380 with its link 0x7e6). The player's name (mode 1)
///   and his link (0x7d7) when his names still match;
/// - 30, 31: the season's transfer spending and income (flags 6).
///
/// An unknown type: the box (line 0x14a7) and 0; otherwise 1.
/// Original: `club_records_write_record_news_text` (0x44c650, club_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_record_news_text(_this: usize, news: usize, full: u32, out: usize) -> u32 {
    use record_news_fns::CLUB_RECORDS_CPP as FILE;
    let fmt = news_text_format();
    if out == 0 {
        show_error_at(FILE, 0xee7);
        return 0;
    }
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4afa40usize); // Text::clear
    clear(out);
    exe::text_set_string(out, EMPTY_STRING as *const u8);
    if news == 0 {
        show_error_at(FILE, 0xef3);
        return 0;
    }
    covered!("gra_18 record news");
    let mut record = [0u8; 0x390];
    let r = record.as_mut_ptr() as usize;
    for (o, f) in RECORD_NEWS_CONSTRUCTORS {
        let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
        construct(r + o);
    }
    let mut k: u8 = 1;
    let id = exe::newsitem_get_slot(news, 0) as i32;
    let kind = exe::newsitem_get_slot(news, k as u32) as u8;
    k += 1;
    let known = read_record_news_fields(news, kind, r, &mut k);
    if !known {
        show_error_at(FILE, 0x1b02);
    }
    if id < 0 || id > read_i32(G_CLUB_COUNT) - 1 {
        show_error_at(FILE, 0xefe);
        return 0;
    }
    let club = club_at(id as isize);
    let name = club.addr() + club::SHORT_NAME;
    let gender = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32;
    if full == 0 {
        if kind == 0x20 {
            let mut award = [0u8; 0x100];
            fmt(award.as_mut_ptr() as usize, 0x98de58); // "Fans Player of the Year"
            fmt(NEWS_TEXT_BUFFER, 0x98de00, gender, gender, name, award.as_ptr() as usize);
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            link_record_club(out, name, club);
            exe::text_add_link(out, award.as_ptr(), 0x7e6, club);
        } else {
            let headline = if exe::database_is_national_team(club) != 0 { 0x98dde0 } else { 0x98ddb8 };
            fmt(NEWS_TEXT_BUFFER, headline, gender, gender, name);
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            link_record_club(out, name, club);
        }
        return 1;
    }
    if !known {
        show_error_at(FILE, 0x14a7);
        return 0;
    }
    covered!("gra_18 record news body");
    let mut text = [0u8; 1000];
    let tp = text.as_mut_ptr() as usize;
    let set = |out: usize| {
        exe::text_set_string(out, tp as *const u8);
    };
    let (mut b1, mut b2, mut b3) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let mut score = [0u8; 20];
    let sp = score.as_mut_ptr() as usize;
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let comp_at = |idx: i32| read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
    match kind {
        // the final league position, high and low
        1 | 2 => {
            let (pos, idx) = if kind == 1 { (read_u8(r), read_i32(r + 1)) } else { (read_u8(r + 7), read_i32(r + 8)) };
            if idx > -1 {
                let ordinal = exe::utils_format_ordinal(pos as i32 + 1, -1); // ponytail: the original copies it into 20 bytes first
                let comp = comp_at(idx);
                let cg = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
                let template = if kind == 1 { 0x98dd28 } else { 0x98dc98 };
                fmt(tp, template, gender, gender, cg, cg, name, ordinal, comp + competition::SHORT_NAME);
                set(out);
                link_record_club(out, name, club);
                exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
            }
        }
        // the attendance high and low and the gate receipts, at home to the opponent in a round
        3 | 4 | 5 => {
            let (value_at, m) = match kind { 3 => (0xe, 0x12), 4 => (0x31, 0x35), _ => (0x54, 0x58) };
            let value = read_i32(r + value_at);
            if value > -1 {
                let m = RecordMatch(r + m);
                let opp = m.opponent();
                let og = read_i8(opp.addr() + club::SHORT_GENDER_NAME) as i32;
                let rg = m.round_label(&mut b1);
                let oname = opp.addr() + club::SHORT_NAME;
                if kind == 4 {
                    record_money_text(value as u32, 0x22, &mut b2);
                    fmt(tp, 0x98db3c, gender, gender, og, og, rg, rg, name, b2.as_ptr() as usize, oname, b1.as_ptr() as usize);
                } else {
                    let template = if kind == 3 { 0x98dbf4 } else { 0x98da98 };
                    fmt(tp, template, gender, gender, og, og, rg, rg, name, value, oname, b1.as_ptr() as usize);
                }
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b1.as_ptr(), 0x7d0, m.comp());
            }
        }
        // the average attendance
        6 => {
            let value = read_i32(r + 0x77);
            if value > -1 {
                fmt(tp, 0x98da2c, gender, gender, name, value);
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the record win, league win and defeat, the highest scoring game and league game
        7..=11 => {
            let m = RecordMatch(r + [0x7d, 0x9c, 0xbb, 0xda, 0xf9][kind as usize - 7]);
            if m.comp_index() > -1 {
                let (g1, g2) = m.goals();
                let (a, b) = match kind {
                    9 => (g2, g1),
                    10 | 11 => (g1.max(g2), g1.min(g2)),
                    _ => (g1, g2),
                };
                crt_format(sp, 0x98c114, a as i32, b as i32); // "%d-%d"
                let opp = m.opponent();
                let og = read_i8(opp.addr() + club::SHORT_GENDER_NAME) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b1);
                let by_venue = |home: usize, away: usize, against: usize| match m.venue() {
                    1 => home,
                    2 => away,
                    _ => against,
                };
                let by_result = |win: usize, defeat: usize, draw: usize| if g1 > g2 { win } else if g1 < g2 { defeat } else { draw };
                let template = match kind {
                    7 => by_venue(0x98d998, 0x98d908, 0x98d878),
                    8 => by_venue(0x98d7dc, 0x98d740, 0x98d6a4),
                    9 => by_venue(0x98d608, 0x98d570, 0x98d4d8),
                    10 => by_result(0x98d438, 0x98d394, 0x98d2f8),
                    _ => by_result(0x98d250, 0x98d1a4, 0x98d0fc),
                };
                fmt(tp, template, gender, gender, og, og, rg, rg, name, sp, oname, b1.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b1.as_ptr(), 0x7d0, m.comp());
                m.link_score(out, &score);
            }
        }
        // the runs: won, lost (start and end dates), unbeaten, without winning (start date)
        12..=15 => {
            let run = r + [0x118, 0x12c, 0x140, 0x154][kind as usize - 12];
            let games = read_i32(run);
            if games > 1 {
                let start = run + run_record::START_DATE;
                if kind <= 13 {
                    exe::date_get_long_date_string_no_year(start, b1.as_mut_ptr() as usize, 0xc8, 0xff);
                    exe::date_get_long_date_string_no_year(run + run_record::END_DATE, b2.as_mut_ptr() as usize, 0xc8, 0xff);
                    let template = if kind == 12 { 0x98d074 } else { 0x98cfec };
                    fmt(tp, template, gender, gender, 0x64, 0x64, name, games, b1.as_ptr() as usize, b2.as_ptr() as usize);
                } else {
                    exe::date_get_long_date_string_no_year(start, b1.as_mut_ptr() as usize, 0xc8, 0xff);
                    let template = if kind == 14 { 0x98cf80 } else { 0x98cf0c };
                    fmt(tp, template, gender, gender, 0x64, name, games, b1.as_ptr() as usize);
                }
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the player records with a count: goals, league goals, assists, man of the match, cards, career
        16 | 17 | 19 | 21 | 22 | 23 | 24 | 27 => {
            let (count, holder) = match kind {
                16 => (read_i8(r + 0x168) as i32, 0x169),
                17 => (read_i8(r + 0x183) as i32, 0x184),
                19 => (read_i8(r + 0x1d6) as i32, 0x1d7),
                21 => (read_i8(r + 0x210) as i32, 0x211),
                22 => (read_i8(r + 0x22b) as i32, 0x22d),
                23 => (read_i16(r + 0x247) as i32, 0x249),
                24 => (read_i16(r + 0x261) as i32, 0x263),
                _ => (read_i16(r + 0x2f9) as i32, 0x2fb),
            };
            if count > -1 {
                let h = RecordHolder(r + holder);
                h.name(&mut b1);
                let player = b1.as_ptr() as usize;
                match kind {
                    16 => fmt(tp, 0x98ceac, 0x64, gender, gender, player, count, name),
                    17 => fmt(tp, 0x98ce44, 0x64, gender, gender, player, count, name),
                    19 => fmt(tp, 0x98cbb4, 0x64, gender, gender, count, player, name),
                    21 => fmt(tp, 0x98ca88, 0x64, gender, gender, count, player, name),
                    22 => fmt(tp, 0x98c9fc, 0x64, gender, gender, player, count, read_i8(r + 0x22c) as i32, name),
                    23 => fmt(tp, 0x98c980, 0x64, gender, gender, count, player, name),
                    24 => fmt(tp, 0x98c90c, 0x64, gender, gender, count, player, name),
                    _ => fmt(tp, 0x98c668, gender, gender, player, name, count),
                };
                set(out);
                link_record_club(out, name, club);
                h.link(out, &b1);
            }
        }
        // the goals in one match
        18 => {
            let goals = read_i8(r + 0x19e) as i32;
            if goals > -1 {
                let h = RecordHolder(r + 0x19f);
                h.name(&mut b1);
                let m = RecordMatch(r + 0x1b7);
                let opp = m.opponent();
                let og = read_i8(opp.addr() + club::SHORT_GENDER_NAME) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b2);
                let template = match m.venue() { 1 => 0x98cd8c, 2 => 0x98ccd8, _ => 0x98cc24 };
                fmt(tp, template, 0x64, gender, gender, og, og, rg, rg, goals, b1.as_ptr() as usize, name, oname, b2.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b2.as_ptr(), 0x7d0, m.comp());
                h.link(out, &b1);
            }
        }
        // the average rating
        20 => {
            let rating = read_f32(r + club_record::SEASON_RATING);
            if rating as f64 > read_f64(rdata::F64_NEG_1) {
                let h = RecordHolder(r + 0x1f5);
                h.name(&mut b1);
                fmt(tp, 0x98cb10, 0x64, gender, gender, rating as f64, b1.as_ptr() as usize, read_i8(r + club_record::SEASON_RATING_GAMES) as i32, name);
                set(out);
                link_record_club(out, name, club);
                h.link(out, &b1);
            }
        }
        // the youngest and the oldest appearance
        25 | 26 => {
            let base = if kind == 25 { 0x27b } else { 0x2ba };
            let m = RecordMatch(r + base + 0x20);
            if m.comp_index() > -1 {
                let h = RecordHolder(r + base + 8);
                h.name(&mut b1);
                let years: unsafe extern "C" fn(usize, usize, usize, i32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES);
                years(m.0, r + base, sp, 5);
                let opp = m.opponent();
                let og = read_i8(opp.addr() + club::SHORT_GENDER_NAME) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b2);
                let template = match m.venue() { 1 => 0x98c850, 2 => 0x98c798, _ => 0x98c6e0 };
                fmt(tp, template, 0x64, gender, gender, og, og, rg, rg, b1.as_ptr() as usize, sp, name, oname, b2.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b2.as_ptr(), 0x7d0, m.comp());
                h.link(out, &b1);
            }
        }
        // the record purchase and sale
        28 | 29 => {
            let base = if kind == 28 { club_record::FEE_PAID } else { club_record::FEE_RECEIVED };
            let fee = read_i32(r + base);
            if fee > -1 {
                let h = RecordHolder(r + base + 4);
                h.name(&mut b1);
                let other = club_at(read_i32(r + base + 0x1c) as isize);
                let xg = read_i8(other.addr() + club::SHORT_GENDER_NAME) as i32;
                let xname = other.addr() + club::SHORT_NAME;
                record_money_text(fee as u32, 6, &mut b2);
                let exchange = read_u8(r + base + 0x28) != 0;
                let (money, player) = (b2.as_ptr() as usize, b1.as_ptr() as usize);
                if kind == 28 {
                    let template = if exchange { 0x98c5d8 } else { 0x98c554 };
                    fmt(tp, template, 0x64, xg, xg, gender, gender, money, player, xname, name);
                } else {
                    let template = if exchange { 0x98c4c8 } else { 0x98c448 };
                    fmt(tp, template, 0x64, gender, gender, xg, xg, money, player, name, xname);
                }
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, xname, other);
                h.link(out, &b1);
            }
        }
        // the season's transfer spending and income
        30 | 31 => {
            let value = read_i32(r + if kind == 30 { club_record::TRANSFER_SPENDING } else { club_record::TRANSFER_INCOME });
            if value > -1 {
                record_money_text(value as u32, 6, &mut b1);
                fmt(tp, if kind == 30 { 0x98c3f0 } else { 0x98c394 }, gender, gender, b1.as_ptr() as usize, name);
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the fans' player of the year
        _ => {
            let h = RecordHolder(r + club_record::FANS_PLAYER);
            if read_i32(h.0 + 0xc) > -1 {
                h.name(&mut b1);
                let long_name = club.addr() + club::NAME;
                let lg = read_i8(club.addr() + club::GENDER_NAME) as i32;
                fmt(b3.as_mut_ptr() as usize, 0x98c380); // "Player of the Year"
                fmt(tp, 0x98c2f0, lg, lg, 0x64, b1.as_ptr() as usize, long_name, b3.as_ptr() as usize);
                set(out);
                link_record_club(out, long_name, club);
                exe::text_add_link(out, b3.as_ptr(), 0x7e6, club);
                h.link(out, &b1);
            }
        }
    }
    1
}

/// Comparer for `write_record_news_text`: the text side by side (`compare_text_writer`) and the return. Without the
/// Text or the news item, with a club id outside the table or an unknown type (the error boxes), only the original.
pub(crate) unsafe extern "thiscall" fn compare_write_record_news_text(this: usize, news: usize, full: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(%%I0%%);
    if out == 0 || news == 0 {
        return o(this, news, full, out);
    }
    let id = exe::newsitem_get_slot(news, 0) as i32;
    let kind = exe::newsitem_get_slot(news, 1) as u8;
    if id < 0 || id > read_i32(G_CLUB_COUNT) - 1 || record_news_fields(kind).is_empty() {
        return o(this, news, full, out);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(out, || ro.set(o(this, news, full, out)), |t| rr.set(write_record_news_text(this, news, full, t)),
        || format!("write_record_news_text(type {}, full {})", kind, full));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("write_record_news_text(type {}) returned {} x {}", kind, ro.get(), rr.get()));
    ro.get()
}
