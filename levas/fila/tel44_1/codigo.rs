// ---------- batch %%LEVA%%: the news text of a broken competition record (comp_screens.cpp) ----------

/// The constructors the original runs over its local copy of the broken record (the `Comp_Season_Records` layout,
/// `comp_stats_obj::SEASON_RECORDS_SIZE` bytes), in its order: the attendance and result match records (0x645800,
/// FixedFixture::FixedFixture: only their Date), the four run records (`run_record_construct`), the two holders, the
/// goals-in-a-match record, the tallies, the rating record, the count record and the youngest and oldest player
/// records. All thiscall, returning `this`; they only write default Dates.
const COMP_RECORD_NEWS_CONSTRUCTORS: [(usize, usize); 17] = [
    (0x29, 0x645800), (0x4c, 0x645800), (0x75, 0x645800), (0x94, 0x645800),
    (0xb3, 0x448670), (0xcb, 0x448670), (0xe3, 0x448670), (0xfb, 0x448670),
    (0x114, 0x450d40), (0x133, 0x450d40), (0x14b, 0x645800), (0x16a, 0x450c60), (0x189, 0x453c50), (0x1ac, 0x450c60),
    (0x1cb, 0x450d30), (0x1eb, 0x49f390), (0x22a, 0x49f3f0),
];

/// The fields of each competition record kind (1..=22) as the news item carries them, in the original's order (the
/// inverse of `comp_stats_write_record_to_news`); kinds 21 and 22 start with the day and the year of the youngest or
/// oldest player's birth date.
pub(crate) fn comp_record_news_fields(kind: i8) -> &'static [RecordNewsField] {
    use RecordNewsField::*;
    match kind {
        1 => &[Byte(0), Dword(1)],
        2 => &[SWord(5), Dword(7), Word(0xb)],
        3 => &[SWord(0xd), Dword(0xf), Word(0x13)],
        4 => &[SWord(0x15), Dword(0x17), Word(0x1b)],
        5 => &[SByte(0x1d), SByte(0x1e), Dword(0x1f), Word(0x23)],
        6 => &[Dword(0x25), Match(0x29)],
        7 => &[Dword(0x48), Match(0x4c)],
        8 => &[Dword(0x6b), Dword(0x6f), Word(0x73)],
        9 => &[Match(0x75)],
        10 => &[Match(0x94)],
        11 => &[Run(0xb3), Dword(0xc7)],
        12 => &[Run(0xcb), Dword(0xdf)],
        13 => &[Run(0xe3), Dword(0xf7)],
        14 => &[Run(0xfb), Dword(0x10f)],
        15 => &[SByte(0x113), Holder(0x114), Dword(0x12c), Word(0x130)],
        16 => &[SByte(0x132), Holder(0x133), Match(0x14b)],
        17 => &[SByte(0x16a), Holder(0x16b), Dword(0x183), Word(0x187)],
        18 => &[Percent(0x189), Holder(0x18d), SByte(0x1a5), Dword(0x1a6), Word(0x1aa)],
        19 => &[SByte(0x1ac), Holder(0x1ad), Dword(0x1c5), Word(0x1c9)],
        20 => &[SByte(0x1cb), SByte(0x1cc), Holder(0x1cd), Dword(0x1e5), Word(0x1e9)],
        21 => &[SWord(0x1eb), Word(0x1ed), Holder(0x1f3), Match(0x20b)],
        22 => &[SWord(0x22a), Word(0x22c), Holder(0x232), Match(0x24a)],
        _ => &[],
    }
}

/// Reads the broken record of `kind` from the news item's slots (from slot 2 on) into the local record copy `r`:
/// bytes, words and dwords stored at their offsets, the average rating (kind 18) as the slot × 0.01 in a float, the
/// match, holder and run records by their readers (`exe::read_match_record_from_news`,
/// `exe::read_staff_name_ref_from_news`, `read_run_record_from_news`, each moving the slot on); kinds 21 and 22 read the
/// day and the year into a Date (`exe::new_date`, then `exe::date_operator_3d` into the record). Returns false for a
/// kind outside 1..=22.
pub(crate) unsafe fn read_comp_record_news_fields(news: usize, kind: i8, r: usize) -> bool {
    use RecordNewsField::*;
    let fields = comp_record_news_fields(kind);
    if fields.is_empty() {
        return false;
    }
    let mut k: u8 = 2;
    let slot = |k: &mut u8| {
        let v = exe::newsitem_get_slot(news, *k as u32);
        *k = k.wrapping_add(1);
        v
    };
    let mut rest = fields;
    if kind == 21 || kind == 22 {
        let (day, year) = (slot(&mut k), slot(&mut k));
        let (mut date, mut copy) = ([0u8; 8], [0u8; 8]);
        exe::new_date(date.as_mut_ptr() as usize, day, year);
        // the birth date of the youngest or oldest player's record
        exe::date_operator_3d(r + if kind == 21 { 0x1eb } else { 0x22a }, copy.as_mut_ptr() as usize, date.as_ptr() as usize);
        rest = &fields[2..];
    }
    let run: unsafe extern "C" fn(usize, *mut u8, usize) -> u32 = core::mem::transmute(FN_READ_RUN_RECORD_FROM_NEWS);
    for f in rest {
        match *f {
            Byte(o) | SByte(o) => write_u8(r + o, slot(&mut k) as u8),
            Word(o) | SWord(o) => write_u16(r + o, slot(&mut k) as u16),
            Dword(o) => write_u32(r + o, slot(&mut k)),
            Percent(o) => write_f32(r + o, (slot(&mut k) as i32 as f64 * read_f64(rdata::F64_0_01)) as f32),
            Match(o) => {
                exe::read_match_record_from_news(news, &mut k, (r + o) as *mut u8);
            }
            Holder(o) => {
                exe::read_staff_name_ref_from_news(news, &mut k, (r + o) as *mut u8);
            }
            Run(o) => {
                run(news, &mut k, r + o);
            }
        }
    }
    true
}

/// A club of a competition record: its short name copied (`club::SHORT_NAME`), the gender of that name
/// (`club::SHORT_GENDER_NAME`, sign-extended for the `{}` of the formats) and the club, from the club id at `id_at`.
struct CompRecordTeam {
    club: usize,
    name: [u8; 0xc8],
    gender: i32,
}

impl CompRecordTeam {
    unsafe fn at(id_at: usize) -> Self {
        let club = record_club(id_at, 0);
        let mut name = [0u8; 0xc8];
        record_club_name(club, &mut name);
        CompRecordTeam { club, name, gender: record_gender(club) } // tel44_1: the team's gender
    }
    fn name(&self) -> *const u8 {
        self.name.as_ptr()
    }
    /// The club's link (0x7d5) under its short name.
    unsafe fn link(&self, out: usize) {
        exe::text_add_link(out, self.name.as_ptr(), LINK_CLUB, self.club); // tel44_1: the club's link
    }
}

/// The round label of a record's match `m` (`exe::comp_util_format_stage_round_name` of the round and stage words at
/// +0xc and +0xe, 200 bytes, mode 0) and its gender (`exe::comp_util_get_label_gender` with no competition, the two
/// dwords at +0xc and +0xe and `form`).
unsafe fn comp_record_round_label(m: usize, form: u32, label: &mut [u8; 0xc8]) -> i32 {
    exe::comp_util_format_stage_round_name(read_i16(m + club_match_record::ROUND_PREFIX) as i32, read_i16(m + club_match_record::ROUND_NAME) as i32,
        label.as_mut_ptr() as usize, 0xc8, 0);
    exe::comp_util_get_label_gender(0, read_u32(m + club_match_record::ROUND_PREFIX), read_u32(m + club_match_record::ROUND_NAME), form) as i8 as i32
}

/// The template by the venue of a record's match (+0x10: 1 at home, 2 away, otherwise neutral).
unsafe fn comp_record_by_venue(m: usize, templates: [usize; 3]) -> usize {
    match read_u8(m + club_match_record::VENUE) {
        1 => templates[0],
        2 => templates[1],
        _ => templates[2],
    }
}

/// The two clubs of a record's match `m` (the club +0x11 and its opponent +0x15) and its round label (`form`).
struct CompRecordMatch {
    m: usize,
    home: CompRecordTeam,
    away: CompRecordTeam,
    label: [u8; 0xc8],
    label_gender: i32,
}

impl CompRecordMatch {
    unsafe fn at(m: usize, form: u32) -> Self {
        let home = CompRecordTeam::at(m + club_match_record::CLUB_ID);
        let away = CompRecordTeam::at(m + club_match_record::OPPONENT_ID);
        let mut label = [0u8; 0xc8];
        let label_gender = comp_record_round_label(m, form, &mut label);
        CompRecordMatch { m, home, away, label, label_gender }
    }
    fn has_label(&self) -> bool {
        self.label[0] != 0
    }
    /// The links of both clubs, then the round's (0x7d0, to the match's club competition) when there is a label.
    unsafe fn link(&self, out: usize) {
        self.home.link(out);
        self.away.link(out);
        if self.has_label() {
            record_stage_link(out, self.m, &self.label);
        }
    }
}

/// Kinds 1 to 5 and 8, a team record: most titles (or trophies, outside a league division), points, goals scored and
/// conceded, cards and the average attendance; empty (no text) when the count is below 0.
unsafe fn comp_record_team_text(stats: usize, out: usize, r: usize, kind: i8, tp: usize) -> bool {
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let team = |o: usize| CompRecordTeam::at(r + o);
    let t = match kind {
        1 => {
            let wins = read_i8(r);
            if wins <= -1 {
                return false;
            }
            let t = team(1);
            let comp_id = read_i32(read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION));
            let template = if exe::comp_util_is_league_division(comp_id) != 0 { 0x99eba0 } else { 0x99eb68 };
            f(tp as *mut u8, template, t.gender, t.gender, t.name(), wins as i32);
            t
        }
        2..=4 => {
            let at = [5usize, 0xd, 0x15][kind as usize - 2];
            let value = read_i16(r + at);
            if value <= -1 {
                return false;
            }
            let t = team(at + 2);
            f(tp as *mut u8, [0x99eb24usize, 0x99eae0, 0x99ea9c][kind as usize - 2], t.gender, t.gender, value as i32, t.name());
            t
        }
        5 => {
            let yellow = read_i8(r + 0x1d);
            if yellow <= -1 {
                return false;
            }
            let t = team(0x1f);
            f(tp as *mut u8, 0x99ea38, t.gender, t.gender, t.name(), yellow as i32, read_i8(r + 0x1e) as i32);
            t
        }
        _ => {
            let average = read_i32(r + 0x6b);
            if average <= -1 {
                return false;
            }
            let t = team(0x6f);
            f(tp as *mut u8, 0x99e7cc, t.gender, t.gender, t.name(), average);
            t
        }
    };
    exe::text_set_string(out, tp as *const u8);
    t.link(out);
    true
}

/// Kinds 6 and 7, the attendance at a match (`at`: the attendance, the match record after it), with the round label
/// (form 1) when there is one.
unsafe fn comp_record_attendance_text(out: usize, r: usize, at: usize, templates: (usize, usize), tp: usize) {
    let attendance = read_i32(r + at);
    if attendance <= -1 {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let x = CompRecordMatch::at(r + at + 4, 1);
    let (hg, ag) = (x.home.gender, x.away.gender);
    if x.has_label() {
        f(tp as *mut u8, templates.0, hg, hg, ag, ag, x.label_gender, attendance, x.home.name(), x.away.name(), x.label.as_ptr());
    } else {
        f(tp as *mut u8, templates.1, hg, hg, ag, ag, attendance, x.home.name(), x.away.name());
    }
    exe::text_set_string(out, tp as *const u8);
    x.link(out);
}

/// Kinds 9 and 10, a result: the record win (the score as it is, the template by the venue) and the highest scoring
/// game (the higher count first, the winner named first, the template by the result). Round label form 0. The genders
/// always follow the record's club and its opponent, also when the opponent is named first (as the original does).
unsafe fn comp_record_result_text(out: usize, r: usize, kind: i8, tp: usize) {
    let m = RecordMatch(r + if kind == 9 { 0x75 } else { 0x94 });
    if m.comp_index() <= -1 {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let crt: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (g1, g2) = m.goals();
    let mut score = [0u8; 0x14];
    if kind == 9 {
        crt(score.as_mut_ptr(), 0x98c114, g1 as i32, g2 as i32); // "%d-%d"
    } else {
        crt(score.as_mut_ptr(), 0x98c114, g1.max(g2) as i32, g1.min(g2) as i32);
    }
    let x = CompRecordMatch::at(m.0, 0);
    let (hg, ag, lg) = (x.home.gender, x.away.gender, x.label_gender);
    let (home, away, sp) = (x.home.name(), x.away.name(), score.as_ptr());
    let (first, second) = if kind == 10 && g1 < g2 { (away, home) } else { (home, away) };
    if x.has_label() {
        let template = match kind {
            9 => comp_record_by_venue(m.0, [0x99e748, 0x99e6c8, 0x99e648]),
            _ => if g1 == g2 { 0x99e414 } else { 0x99e4a0 },
        };
        f(tp as *mut u8, template, hg, hg, ag, ag, lg, first, sp, second, x.label.as_ptr());
    } else {
        let template = match kind {
            9 => comp_record_by_venue(m.0, [0x99e5e8, 0x99e58c, 0x99e530]),
            _ => if g1 == g2 { 0x99e344 } else { 0x99e3ac },
        };
        f(tp as *mut u8, template, hg, hg, ag, ag, first, sp, second);
    }
    exe::text_set_string(out, tp as *const u8);
    x.link(out);
    m.link_score(out, &score);
}

/// Kinds 11 to 14, a run (more than one game): the runs of wins and defeats between the start and the end dates, the
/// unbeaten and winless runs from the start date (`exe::date_get_long_date_string_no_year`); the club's link.
unsafe fn comp_record_run_text(out: usize, r: usize, kind: i8, tp: usize) {
    let run = r + [0xb3usize, 0xcb, 0xe3, 0xfb][kind as usize - 11];
    let games = read_i32(run);
    if games <= -1 || games <= 1 {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let t = CompRecordTeam::at(run + 0x14);
    let (mut start, mut end) = ([0u8; 0xc8], [0u8; 0xc8]);
    exe::date_get_long_date_string_no_year(run + run_record::START_DATE, start.as_mut_ptr() as usize, 0xc8, 0xff);
    if kind <= 12 {
        exe::date_get_long_date_string_no_year(run + run_record::END_DATE, end.as_mut_ptr() as usize, 0xc8, 0xff);
        let template = if kind == 11 { 0x99e2c4 } else { 0x99e240 };
        f(tp as *mut u8, template, t.gender, t.gender, 0x64, 0x64, t.name(), games, start.as_ptr(), end.as_ptr());
    } else {
        let template = if kind == 13 { 0x99e1d8 } else { 0x99e168 };
        f(tp as *mut u8, template, t.gender, t.gender, 0x64, t.name(), games, start.as_ptr());
    }
    exe::text_set_string(out, tp as *const u8);
    t.link(out);
}

/// Kinds 15 and 17 to 20, a player's season record: goals, assists, the average rating (above −1.0, "%4.2f", with the
/// games), man of the match awards, yellow and red cards; the holder's name (style 1), the club's link and the
/// player's when his names still match.
unsafe fn comp_record_player_text(out: usize, r: usize, kind: i8, tp: usize) {
    let (holder, club_at) = match kind {
        15 => (0x114usize, 0x12cusize),
        17 => (0x16b, 0x183),
        18 => (0x18d, 0x1a6),
        19 => (0x1ad, 0x1c5),
        _ => (0x1cd, 0x1e5),
    };
    // 15, 17, 19: the count byte just before the holder; 20: the yellow cards
    let count = read_i8(r + holder - 1);
    let rating = read_f32(r + 0x189);
    let holds = match kind {
        18 => rating as f64 > read_f64(rdata::F64_NEG_1),
        20 => read_i8(r + 0x1cb) > -1,
        _ => count > -1,
    };
    if !holds {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut player = [0u8; 0xc8];
    record_holder_name(r + holder, &mut player, 1);
    let t = CompRecordTeam::at(r + club_at);
    let (g, p) = (t.gender, player.as_ptr());
    match kind {
        15 => f(tp as *mut u8, 0x99e110, g, g, 0x64, count as i32, t.name(), p),
        17 => f(tp as *mut u8, 0x99dd40, g, g, 0x64, count as i32, t.name(), p),
        18 => f(tp as *mut u8, 0x99dcb0, g, g, 0x64, rating as f64, t.name(), p, read_i8(r + 0x1a5) as i32),
        19 => f(tp as *mut u8, 0x99dc3c, g, g, count as i32, t.name(), p),
        _ => f(tp as *mut u8, 0x99dbb4, g, g, read_i8(r + 0x1cb) as i32, read_i8(r + 0x1cc) as i32, t.name(), p),
    };
    exe::text_set_string(out, tp as *const u8);
    t.link(out);
    record_holder_link(out, r + holder, &player);
}

/// Kinds 16, 21 and 22, a player's record in one match: the most goals (the count first) and the youngest and oldest
/// player (the age in years at the match, `database_format_years_between_dates` from the birth date to the match date,
/// 5 bytes); the template by the venue, with the round label (form 0) when there is one; the links of both clubs, the
/// round and the player.
unsafe fn comp_record_player_match_text(out: usize, r: usize, kind: i8, tp: usize) {
    let (base, holder, at) = match kind {
        16 => (0x132usize, 0x133usize, 0x14busize),
        21 => (0x1eb, 0x1f3, 0x20b),
        _ => (0x22a, 0x232, 0x24a),
    };
    let holds = if kind == 16 { read_i8(r + base) > -1 } else { RecordMatch(r + at).comp_index() > -1 };
    if !holds {
        return;
    }
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut player = [0u8; 0xc8];
    record_holder_name(r + holder, &mut player, 1);
    let mut years = [0u8; 8];
    if kind != 16 {
        let between: unsafe extern "C" fn(usize, usize, usize, i32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES);
        between(r + at, r + base, years.as_mut_ptr() as usize, 5);
    }
    let x = CompRecordMatch::at(r + at, 0);
    let (hg, ag, lg) = (x.home.gender, x.away.gender, x.label_gender);
    let (home, away, p, y) = (x.home.name(), x.away.name(), player.as_ptr(), years.as_ptr());
    let goals = read_i8(r + base) as i32;
    match (kind == 16, x.has_label()) {
        (true, true) => f(tp as *mut u8, comp_record_by_venue(r + at, [0x99e068, 0x99dfc4, 0x99df20]), 0x64, hg, hg, ag, ag, lg, goals, p, home, away, x.label.as_ptr()),
        (true, false) => f(tp as *mut u8, comp_record_by_venue(r + at, [0x99de9c, 0x99de1c, 0x99dd9c]), 0x64, hg, hg, ag, ag, goals, p, home, away),
        (false, true) => f(tp as *mut u8, comp_record_by_venue(r + at, [0x99db08, 0x99da60, 0x99d9b8]), 0x64, hg, hg, ag, ag, lg, p, y, home, away, x.label.as_ptr()),
        (false, false) => f(tp as *mut u8, comp_record_by_venue(r + at, [0x99d930, 0x99d8ac, 0x99d828]), 0x64, hg, hg, ag, ag, p, y, home, away),
    };
    exe::text_set_string(out, tp as *const u8);
    x.link(out);
    record_holder_link(out, r + holder, &player);
}

/// Writes the body of the news that a competition broke one of its records (news type 0x3e9, from
/// `comp_stats_send_record_news`, written by `news_club_record` through the competition's statistics object).
///
/// `comp_stats_write_record_news_text(news, out)`. Without the Text, the comp_stats.CPP error box (line 0x1383) and 0;
/// the Text is cleared and set empty; without the news item, the box (0x138f) and 0. The local record copy is
/// constructed as the original does (`COMP_RECORD_NEWS_CONSTRUCTORS`); slot 0 (the competition id) is read and not
/// used, slot 1 is the record kind, then the kind's fields (`read_comp_record_news_fields`; an unknown kind: the box at
/// line 0x1ce2, then the box at line 0x16fe and 0).
///
/// The text, by kind, each only when its record holds (otherwise the empty text), with the clubs' short names and
/// genders and their links (0x7d5):
/// - 1 most titles ("titles" in a league division, `exe::comp_util_is_league_division` of the competition, otherwise
///   "trophies"), 2 points, 3 goals scored, 4 goals conceded, 5 yellow and red cards, 8 the average attendance;
/// - 6 and 7 the attendances, with both clubs and the round;
/// - 9 the record win ("%d-%d", the template by the venue) and 10 the highest scoring game (the higher count first,
///   the winner named first, win or draw); the score's link (0x7d3) when the fixture still matches;
/// - 11 to 14 the runs (more than one game): wins and defeats with the start and end dates, unbeaten and winless from
///   the start date;
/// - 15 goals, 17 assists, 18 the average rating with the games, 19 man of the match, 20 cards, with the player;
/// - 16 the most goals in one match, 21 and 22 the youngest and oldest player (the age at the match), with both clubs,
///   the round and the player.
/// The round label (`comp_util_format_stage_round_name`, mode 0) is linked to the match's club competition (0x7d0)
/// when not empty; the player (0x7d7) when his names still match. Returns 1.
///
/// Defects of the original: in kind 10 the genders stay the record club's first also when the opponent is named first
/// (the winner); the original's check that its local record copy is not null (line 0x1bdf) can never fail and is left
/// out.
/// Original: `comp_stats_write_record_news_text` (0x4a7f50, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_write_record_news_text(stats: usize, news: usize, out: usize) -> u32 {
    if out == 0 {
        show_error_box(COMP_STATS_CPP, 0x1383);
        return 0;
    }
    exe::text_clear(out as *mut usize);
    exe::text_set_string(out, EMPTY_STRING as *const u8);
    if news == 0 {
        show_error_box(COMP_STATS_CPP, 0x138f);
        return 0;
    }
    covered!("tel44_1 comp record news text");
    let mut record = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    let r = record.as_mut_ptr() as usize;
    for (o, f) in COMP_RECORD_NEWS_CONSTRUCTORS {
        let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
        construct(r + o);
    }
    exe::newsitem_get_slot(news, 0);
    let kind = exe::newsitem_get_slot(news, 1) as u8 as i8; // tel44_1: slot 1 holds the kind
    if !read_comp_record_news_fields(news, kind, r) {
        show_error_box(COMP_STATS_CPP, 0x1ce2);
        show_error_box(COMP_STATS_CPP, 0x16fe);
        return 0;
    }
    let mut text = [0u8; 1000];
    let tp = text.as_mut_ptr() as usize;
    match kind {
        1..=5 | 8 => {
            comp_record_team_text(stats, out, r, kind, tp);
        }
        6 => comp_record_attendance_text(out, r, 0x25, (0x99e9a4, 0x99e92c), tp),
        7 => comp_record_attendance_text(out, r, 0x48, (0x99e898, 0x99e824), tp),
        9 | 10 => comp_record_result_text(out, r, kind, tp),
        11..=14 => comp_record_run_text(out, r, kind, tp),
        16 | 21 | 22 => comp_record_player_match_text(out, r, kind, tp),
        _ => comp_record_player_text(out, r, kind, tp),
    }
    1
}

/// Comparer for `comp_stats_write_record_news_text`: the text side by side (`compare_text_writer`: the text, the links
/// and the random generator) and the return. Without the Text or the news item, or with an unknown kind (the error
/// boxes), only the original.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_write_record_news_text(stats: usize, news: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_COMP_STATS_WRITE_RECORD_NEWS_TEXT);
    if out == 0 || news == 0 {
        return o(stats, news, out);
    }
    let kind = exe::newsitem_get_slot(news, 1) as u8 as i8;
    if comp_record_news_fields(kind).is_empty() {
        return o(stats, news, out);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(out, || ro.set(o(stats, news, out)), |t| rr.set(comp_stats_write_record_news_text(stats, news, t)),
        || format!("comp_stats_write_record_news_text(kind {})", kind));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("comp_stats_write_record_news_text(kind {}) returned {} x {}", kind, ro.get(), rr.get()));
    ro.get()
}
