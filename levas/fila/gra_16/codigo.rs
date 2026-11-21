// ---------- batch %%LEVA%%: the news text of a ban (discipline.cpp) ----------

/// `Discipline::get_scope_text(rules, out, size, appeal_pending, form)` (0x55ce40, discipline.cpp; thiscall, ret 0x14):
/// the scope of a ban ("European", "domestic"...) from the four rule-set bytes into `out`; returns the scope's
/// grammatical gender.
const FN_DISCIPLINE_GET_SCOPE_TEXT: usize = 0x55ce40;
/// `Date::get_date_string_no_weekday(date, out, size, time_of_day)` (0x54c3b0, Date.cpp; thiscall, ret 0xc).
const FN_DATE_GET_DATE_STRING_NO_WEEKDAY: usize = 0x54c3b0;

/// The ban templates `[plain, a further ban, starting on a date]`, by the count of the ban: `[N matches, one match,
/// days]` (a count below 0 is a ban in days). Points: "after collecting <n> points".
const BAN_POINTS: [[u32; 3]; 3] = [
    [0x9c08e8, 0x9c098c, 0x9c080c],
    [0x9c0b08, 0x9c0b98, 0x9c0a3c],
    [0x9bb388, 0x9bb438, 0x9bb2a0],
];
/// "after collecting <n> yellow cards in successive matches".
const BAN_SUCCESSIVE_YELLOWS: [[u32; 3]; 3] = [
    [0x9be6a8, 0x9be768, 0x9be5b0],
    [0x9be918, 0x9be9c4, 0x9be834],
    [0x9ba49c, 0x9ba568, 0x9ba398],
];
/// "after collecting <n> yellow cards".
const BAN_YELLOWS: [[u32; 3]; 3] = [
    [0x9beb58, 0x9bec04, 0x9bea78],
    [0x9bed88, 0x9bee20, 0x9becb8], // gra16: one match
    [0x9ba728, 0x9ba7dc, 0x9ba63c],
];
/// The bans that come from a match, `[count][the player's club won, drew, lost, the competition named][form]`: the
/// result forms name the score ("<club>'s 2-1 win over <club>"), the competition form names the competition and the
/// two clubs (never for a ban in days). "after collecting a yellow card in".
const BAN_YELLOW_IN_MATCH: [[[u32; 3]; 4]; 3] = [
    [[0x9bfc50, 0x9bfd4c, 0x9bfb1c], [0x9beff0, 0x9bf0ec, 0x9beebc], [0x9bf620, 0x9bf71c, 0x9bf4ec], [0x9c0294, 0x9c03a8, 0x9c0148]],
    [[0x9bff70, 0x9c0058, 0x9bfe50], [0x9bf314, 0x9bf3fc, 0x9bf1f4], [0x9bf944, 0x9bfa2c, 0x9bf824], [0x9c0600, 0x9c0704, 0x9c04c8]],
    [[0x9bb088, 0x9bb190, 0x9baf4c], [0x9ba9dc, 0x9baae4, 0x9ba89c], [0x9bad34, 0x9bae3c, 0x9babf4], [0, 0, 0]],
];
/// "after being sent off in" (a ban in days: "after collecting a red card in").
const BAN_SENT_OFF: [[[u32; 3]; 4]; 3] = [
    [[0x9bda60, 0x9bdb50, 0x9bd938], [0x9bce78, 0x9bcf6c, 0x9bcd50], [0x9bd46c, 0x9bd560, 0x9bd344], [0x9be068, 0x9be174, 0x9bdf24]],
    [[0x9bdd60, 0x9bde40, 0x9bdc4c], [0x9bd180, 0x9bd260, 0x9bd068], [0x9bd774, 0x9bd854, 0x9bd65c], [0x9be3b8, 0x9be4b0, 0x9be288]],
    [[0x9ba188, 0x9ba28c, 0x9ba04c], [0x9b9ae8, 0x9b9bec, 0x9b99ac], [0x9b9e38, 0x9b9f3c, 0x9b9cfc], [0, 0, 0]],
];
/// "after his misconduct in".
const BAN_MISCONDUCT: [[[u32; 3]; 4]; 3] = [
    [[0x9bc200, 0x9bc2f0, 0x9bc0d8], [0x9bb618, 0x9bb70c, 0x9bb4f0], [0x9bbc0c, 0x9bbd00, 0x9bbae4], [0x9bc808, 0x9bc914, 0x9bc6c4]],
    [[0x9bc500, 0x9bc5e0, 0x9bc3ec], [0x9bb920, 0x9bba00, 0x9bb808], [0x9bbf14, 0x9bbff4, 0x9bbdfc], [0x9bcb58, 0x9bcc50, 0x9bca28]],
    [[0x9b97a8, 0x9b98a4, 0x9b9674], [0x9b9138, 0x9b9234, 0x9b9004], [0x9b9470, 0x9b956c, 0x9b933c], [0, 0, 0]],
];

/// The match record a ban news item carries (`read_match_record_from_news`, 0x1f bytes): the date (+0 day, +2 year),
/// the club competition (+8), the round and stage words (+0xc, +0xe), the player's club (+0x11) and its opponent
/// (+0x15), their goals (+0x19, +0x1a) and the fixture index (+0x1b).
struct BanMatch([u8; 0x20]);

impl BanMatch {
    fn at(&self, k: usize) -> usize {
        self.0.as_ptr() as usize + k
    }
    unsafe fn comp_index(&self) -> i32 {
        read_i32(self.at(8))
    }
    unsafe fn club(&self) -> usize {
        club_at(read_i32(self.at(0x11)) as isize).addr()
    }
    unsafe fn opponent(&self) -> usize {
        club_at(read_i32(self.at(0x15)) as isize).addr()
    }
    unsafe fn goals(&self) -> (i8, i8) {
        (read_i8(self.at(0x19)), read_i8(self.at(0x1a)))
    }
    unsafe fn fixture(&self) -> u32 {
        read_u32(self.at(0x1b))
    }
    /// The club competition's record (`G_CLUB_COMP_TABLE`, 0x6b bytes; its names where `competition::` has them).
    unsafe fn comp(&self) -> usize {
        read_usize(G_CLUB_COMP_TABLE).wrapping_add((self.comp_index() as usize).wrapping_mul(0x6b))
    }
}

/// The date `Date::Date` (0x54a110) leaves before the readers overwrite it: day 0x28 of 1972 (0x7b4), flag 1.
const BAN_DEFAULT_DATE: [u8; 8] = [0x28, 0, 0xb4, 0x07, 1, 0, 0, 0];

/// The arguments of one template, in the order the original pushes them (langlib_sprintf is variadic: the unused
/// tail of the array is passed and never read).
struct BanArgs {
    v: [usize; 16],
    n: usize,
}

impl BanArgs {
    fn new() -> Self {
        BanArgs { v: [0; 16], n: 0 }
    }
    fn push(&mut self, x: usize) {
        self.v[self.n] = x;
        self.n += 1;
    }
    /// A gender byte, sign-extended as the original pushes it, twice.
    fn gender(&mut self, g: i8) {
        self.push(g as i32 as usize);
        self.push(g as i32 as usize);
    }
}

/// `strcat(dst, src)` on two NUL-terminated buffers (the original's inline `repne scasb` / `rep movs`).
unsafe fn ban_strcat(dst: usize, src: usize) {
    let mut d = dst;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut s = src;
    loop {
        let c = read_u8(s);
        write_u8(d, c);
        if c == 0 {
            break;
        }
        d += 1;
        s += 1;
    }
}

/// The sentence on how the match ended, from the player's club's goals `g1` and the opponent's `g2` (`d` = g1 − g2,
/// `s` = g1 + g2): won by more than 5 "ran riot with a <score> victory" (0x9c0f50), by more than 3 "ran out easy
/// winners" (0x9c0efc), otherwise won "won a thrilling game" (s > 5, 0x9c0ea8) or "won the match" (0x9c0e5c); 0-0 "a
/// stalemate" (0x9c0da8); another draw "a thrilling draw" (s > 5, 0x9c0e1c) or "a draw" (0x9c0de4); lost by more
/// than 5 "a humiliating defeat" (0x9c0d30), by more than 3 "were easily beaten" (0x9c0cdc), otherwise "were beaten in
/// a cracking match" (s > 5, 0x9c0c7c) or "were beaten" (0x9c0c30). The club forms take the club's short name and
/// gender; the draws only the score.
unsafe fn ban_result_sentence(out: usize, m: &BanMatch, score: usize) {
    let fmt = news_text_format();
    let (g1, g2) = m.goals();
    let (g1, g2) = (g1 as i32, g2 as i32);
    let (d, s) = (g1 - g2, g1 + g2);
    let c = m.club();
    let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
    let name = c + club::SHORT_NAME;
    let team = if d > 5 {
        0x9c0f50
    } else if d > 3 {
        0x9c0efc
    } else if g1 > g2 {
        if s > 5 { 0x9c0ea8 } else { 0x9c0e5c }
    } else if g1 == g2 {
        let draw = if g1 == 0 { 0x9c0da8 } else if s > 5 { 0x9c0e1c } else { 0x9c0de4 };
        fmt(out, draw, score);
        return;
    } else if d < -5 {
        0x9c0d30
    } else if d < -3 {
        0x9c0cdc
    } else if s > 5 {
        0x9c0c7c
    } else {
        0x9c0c30
    };
    fmt(out, team, g, g, name, score);
}

/// Writes the headline or the body of the news that a player was banned (types 0x7d2, and 0x7d3 through
/// `write_further_ban_news_text`).
///
/// `write_ban_news_text(news, full, out)`. The news item's slots: 0 the appeal made, 2 the count (matches; below 0 the
/// ban is in days), 3 the days, 4 and 5 the day and year the ban starts (year 0: no date), 6..9 the four rule sets
/// (`Discipline::get_scope_text`), 10 the kind (0 points, 1 yellow cards, 2 a sending-off, 3 misconduct, 4 yellow
/// cards in successive matches), 11 the number (points or cards; for kind 1, 1 = "a yellow card in" a match), 12 the
/// appeal pending, 14 "a further" ban; from slot 17 the person (`read_staff_name_ref_from_news`), then the match flag
/// and the match record (`read_match_record_from_news`, `BanMatch`). With a match, the competition is named when it is
/// not a reserve or minor competition (`exe::comp_is_reserve_or_minor_comp` of `G_RESERVE_COMP`) unless it is a league
/// division (`exe::comp_util_is_league_division`) in round 0x418, 0x419 or 0xa0 or with a stage word; the score link
/// holds when the fixture still matches (`exe::match_stats_fixture_entry_matches`). The person's links hold when his
/// names still match (`exe::database_staff_matches_names`).
///
/// The headline: "<name> banned for one match" (0x9c1230), "for <n> matches" (0x9c11e4) or, in days, "for <length>"
/// (0x9c1198, `format_duration_text`, 0x1e bytes), name in mode 5; the person's link.
///
/// The body (name in mode 1; the date text when there is a year; the score "<more>-<fewer>" with a match; the scope
/// text of form 4, whose gender is dropped):
/// - rule set 9 = 0 with an appeal pending, the assault on a referee: "banned for <days> days after his assault on a
///   referee in the <competition> match between" (0x9c1078, the competition's short name and gender) when named,
///   otherwise "in the match between" (0x9c0fa8); with a match and the club's goals ≥ 0 the sentence on how it ended
///   is appended (`ban_result_sentence`) with the score's link; then the person's and the two clubs' links (no
///   appeal note);
/// - otherwise the scope text again (form 5 for one match, else 6, its gender kept) and the template of the kind
///   (`BAN_*`: by the count, by "a further" ban, then by a start date, then plain), the arguments in the original's
///   order: the scope's gender twice, 0x64 for the date (twice for one match by points), the competition's and the
///   clubs' genders, the name, the count, the scope, the days, the date, then the number or the competition and the
///   clubs. A match kind not in days names the competition (long name) when it is named, with the sentence on how it
///   ended appended when there is a match; otherwise the club's result (won, drew, lost) picks the form with the
///   score. With the appeal made, "You have appealed..." (0x9b8f68) follows after a blank line. The links: the
///   person, the two clubs, the score.
///
/// Defects of the original (`DEFEITOS_DO_ORIGINAL`): without a match (the bans by points or by cards) the original
/// links the two clubs and the score of a match record it never read (uninitialised stack, `B+0x20..0x3f` and the
/// fixture flag `B+0xd8`): the copy leaves those three links out (its comparer drops them from the original's list);
/// a kind above 4 formats no template and prints the uninitialised buffer: the copy prints an empty text.
/// Original: `Discipline::write_ban_news_text` (0x5576e0, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_ban_news_text(this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let appealed = slot(0) as u8;
    slot(1);
    let count = slot(2) as u8 as i8;
    let days = slot(3) as u16 as i16 as i32;
    let start_day = slot(4);
    let start_year = slot(5);
    let rules = [slot(6) as u8, slot(7) as u8, slot(8) as u8, slot(9) as u8];
    let kind = slot(10) as u8;
    let number = slot(11) as u8 as i8 as i32;
    let pending = slot(12);
    slot(13);
    let further = slot(14) as u8;
    // the person (four ids and a birth date, from slot 17), then the match flag and the match record
    let mut person = [0u8; 0x18];
    person[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
    let mut m = BanMatch([0u8; 0x20]);
    m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
    let mut next: u8 = 17;
    exe::read_staff_name_ref_from_news(news, &mut next, person.as_mut_ptr());
    let pp = person.as_ptr() as usize;
    let has_match = slot(next as u32) != 0;
    next = next.wrapping_add(1);
    let mut named = false;
    if has_match {
        covered!("gra_16 ban with a match");
        exe::read_match_record_from_news(news, &mut next, m.0.as_mut_ptr());
        if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), m.comp()) == 0 {
            named = true;
        }
        if exe::comp_util_is_league_division(m.comp_index()) != 0 {
            let round = read_u16(m.at(0xc));
            if round == 0x418 || round == 0x419 || round == 0xa0 || read_u16(m.at(0xe)) != 0 {
                named = false;
            }
        }
    }
    let (first, second, common, id) = (read_u32(pp), read_u32(pp + 4), read_u32(pp + 8), read_u32(pp + 0xc));
    let staff = staff_at(id as i32 as isize);
    let person_ok = exe::database_staff_matches_names(staff, first, second, common, pp + 0x10) != 0;
    let fixture_ok = has_match
        && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), m.fixture() as i32, read_i32(m.at(0x11)),
            read_i32(m.at(0x15)), read_i32(m.at(0)), read_i32(m.at(2))) != 0;
    let mut name = [0u8; 0xc8];
    let np = name.as_ptr() as usize;
    if full == 0 {
        exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 5, id);
        if count > -1 {
            if count == 1 {
                fmt(NEWS_TEXT_BUFFER, 0x9c1230, np); // "<name> banned for one match"
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9c11e4, np, count as i32); // "<name> banned for <n> matches"
            }
        } else {
            let mut length = [0u8; 0x20];
            format_duration_text(days, length.as_mut_ptr() as usize, 0x1e, 0);
            fmt(NEWS_TEXT_BUFFER, 0x9c1198, np, length.as_ptr() as usize); // "<name> banned for <length>"
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if person_ok {
            exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
        }
        return;
    }
    covered!("gra_16 ban body");
    exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 1, id);
    let mut date_text = [0u8; 0x64];
    let dp = date_text.as_mut_ptr() as usize;
    if start_year as u16 != 0 {
        let mut date = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, start_day, start_year);
        let date_string: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_GET_DATE_STRING_NO_WEEKDAY);
        date_string(date.as_ptr() as usize, dp, 0x64, 0xff);
    }
    let mut score = [0u8; 0xc];
    let sp = score.as_mut_ptr() as usize;
    let (g1, g2) = m.goals();
    if has_match {
        fmt(sp, 0x9c116c, g1.max(g2) as i32, g1.min(g2) as i32); // "<more>-<fewer>"
    }
    let scope_text: unsafe extern "thiscall" fn(usize, *const u8, usize, u32, u32, u32) -> i8 = core::mem::transmute(FN_DISCIPLINE_GET_SCOPE_TEXT);
    let mut scope = [0u8; 0x64];
    let scp = scope.as_mut_ptr() as usize;
    scope_text(this, rules.as_ptr(), scp, 0x64, pending, 4);
    let mut text = [0u8; 0x2710];
    let tp = text.as_mut_ptr() as usize;
    let c1 = m.club();
    let c2 = m.opponent();
    let comp = m.comp();
    let link_clubs = |out: usize| {
        exe::text_add_link(out, (c1 + club::SHORT_NAME) as *const u8, 0x7d5, c1);
        exe::text_add_link(out, (c2 + club::SHORT_NAME) as *const u8, 0x7d5, c2);
    };
    if rules[3] == 0 && pending != 0 {
        covered!("gra_16 assault on a referee");
        let (gc1, gc2) = (read_i8(c1 + club::SHORT_GENDER_NAME) as i32, read_i8(c2 + club::SHORT_GENDER_NAME) as i32);
        let (n1, n2) = (c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        if named {
            let gc = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
            fmt(tp, 0x9c1078, gc, gc, gc1, gc1, gc2, gc2, np, days, comp + competition::SHORT_NAME, n1, n2);
        } else {
            fmt(tp, 0x9c0fa8, gc1, gc1, gc2, gc2, np, days, n1, n2);
        }
        exe::text_set_string(out, tp as *const u8);
        if has_match && g1 > -1 {
            ban_result_sentence(tp, &m, sp);
            news_text_append(out, tp);
            if fixture_ok {
                exe::text_add_link(out, score.as_ptr(), 0x7d3, m.fixture() as usize);
            }
        }
        if person_ok {
            exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
        }
        if has_match {
            link_clubs(out);
        }
        return;
    }
    // the count: 0 = N matches, 1 = one match, 2 = days
    let (row, form) = if count < 0 { (2usize, 6) } else if count == 1 { (1, 5) } else { (0, 6) };
    let scope_gender = scope_text(this, rules.as_ptr(), scp, 0x64, pending, form);
    let variant = if further != 0 { 1 } else if start_year as u16 != 0 { 2 } else { 0 };
    let match_forms = match kind {
        1 if number == 1 => Some(&BAN_YELLOW_IN_MATCH),
        2 => Some(&BAN_SENT_OFF),
        3 => Some(&BAN_MISCONDUCT),
        _ => None,
    };
    let plain_forms = match kind {
        0 => Some(&BAN_POINTS),
        1 if number != 1 => Some(&BAN_YELLOWS),
        4 => Some(&BAN_SUCCESSIVE_YELLOWS),
        _ => None,
    };
    let competition_form = row != 2 && named;
    let template = if let Some(t) = plain_forms {
        Some(t[row][variant])
    } else if let Some(t) = match_forms {
        let result = if competition_form { 3 } else if g1 > g2 { 0 } else if g1 == g2 { 1 } else { 2 };
        Some(t[row][result][variant])
    } else {
        None
    };
    if let Some(template) = template {
        covered!("gra_16 ban template");
        let mut a = BanArgs::new();
        a.gender(scope_gender);
        if variant == 2 {
            a.push(0x64);
            if kind == 0 && row == 1 {
                a.push(0x64);
            }
        }
        if match_forms.is_some() {
            if competition_form {
                a.gender(read_i8(comp + competition::GENDER_NAME));
            }
            a.gender(read_i8(c1 + club::SHORT_GENDER_NAME));
            a.gender(read_i8(c2 + club::SHORT_GENDER_NAME));
        }
        a.push(np);
        if row == 0 {
            a.push(count as i32 as usize);
        }
        a.push(scp);
        if row == 2 {
            a.push(days as usize);
        }
        if variant == 2 {
            a.push(dp);
        }
        if match_forms.is_none() {
            a.push(number as usize);
        } else if competition_form {
            a.push(comp + competition::NAME);
            a.push(c1 + club::SHORT_NAME);
            a.push(c2 + club::SHORT_NAME);
        } else {
            a.push(c1 + club::SHORT_NAME);
            a.push(sp);
            a.push(c2 + club::SHORT_NAME);
        }
        let v = a.v;
        fmt(tp, template as usize, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]);
        if competition_form && has_match {
            covered!("gra_16 ban match result");
            let mut result = [0u8; 0x3e8];
            ban_result_sentence(result.as_mut_ptr() as usize, &m, sp);
            ban_strcat(tp, result.as_ptr() as usize);
        }
    }
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    if appealed != 0 {
        covered!("gra_16 ban appealed");
        let mut appeal = [0u8; 0x3d8];
        fmt(appeal.as_mut_ptr() as usize, 0x9b8f68); // "You have appealed against this decision..."
        crt_format(NEWS_TEXT_BUFFER, 0x991948, tp, appeal.as_ptr() as usize); // "%s\n\n%s"
    } else {
        crt_format(NEWS_TEXT_BUFFER, 0x991944, tp); // "%s"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if person_ok {
        exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
    }
    if has_match {
        link_clubs(out);
        if fixture_ok {
            exe::text_add_link(out, score.as_ptr(), 0x7d3, m.fixture() as usize);
        }
    }
}

/// Comparer for `write_ban_news_text`: the text side by side (`compare_text_writer`). The body of a ban without a match
/// is compared without the clubs' and the score's links, which the original takes from uninitialised stack.
pub(crate) unsafe extern "thiscall" fn compare_write_ban_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let what = || format!("write_ban_news_text(news {:#x}, full {})", news, full);
    if full == 0 || exe::newsitem_get_slot(news, 23) != 0 {
        compare_text_writer(out, || o(this, news, full, out), |t| write_ban_news_text(this, news, full, t), what);
        return;
    }
    covered!("gra_16 ban without a match compared");
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, full, out),
        || crate::rng::without_counting(|| write_ban_news_text(this, news, full, lp)));
    let keep = |t: (Vec<u8>, Vec<(Vec<u8>, u32, u32)>)| (t.0, t.1.into_iter().filter(|l| l.1 != 0x7d5 && l.1 != 0x7d3).collect::<Vec<_>>());
    let (mo, mr) = (keep(text_contents(out)), keep(text_contents(lp)));
    exe::drop_text(local.as_mut_ptr());
    SCOREBOARD.record(same_rng && mo == mr, || format!("{}: original {:?} {:x?} rust {:?} {:x?}", what(),
        String::from_utf8_lossy(&mo.0), mo.1.len(), String::from_utf8_lossy(&mr.0), mr.1.len()));
}
