// --- sx09c_9: the text of the further ban news (discipline.cpp) ---

/// The templates of a further ban by points, yellow cards and yellow cards in successive matches: `[one match, N
/// matches, a ban in days]`.
const FURTHER_BAN_POINTS: [u32; 3] = [0x9c31e0, 0x9c3130, 0x9c1c40];
const FURTHER_BAN_YELLOWS: [u32; 3] = [0x9c2a98, 0x9c29e0, 0x9c18c0];
const FURTHER_BAN_SUCCESSIVE_YELLOWS: [u32; 3] = [0x9c2928, 0x9c285c, 0x9c1808];

/// The templates of a further ban for a match (a yellow card in it, a sending-off, misconduct): `[one match, N matches,
/// days]` × `[the club's win, defeat, draw]`.
const FURTHER_BAN_YELLOW_IN_MATCH: [[u32; 3]; 3] =
    [[0x9c303c, 0x9c2e40, 0x9c2c44], [0x9c2f34, 0x9c2d38, 0x9c2b3c], [0x9c1b4c, 0x9c1a58, 0x9c1964]];
const FURTHER_BAN_SENT_OFF: [[u32; 3]; 3] =
    [[0x9c2770, 0x9c2588, 0x9c239c], [0x9c2674, 0x9c2488, 0x9c229c], [0x9c1718, 0x9c1624, 0x9c1530]];
const FURTHER_BAN_MISCONDUCT: [[u32; 3]; 3] =
    [[0x9c21b0, 0x9c1fc8, 0x9c1ddc], [0x9c20b4, 0x9c1ec8, 0x9c1cdc], [0x9c1444, 0x9c1358, 0x9c126c]];

/// The template sentence of a further ban (the body without the appeal note) into `tp`: the count picks the row (1
/// one match, ≥ 0 N matches, below 0 days) and the kind the template (`FURTHER_BAN_*`: 0 points, 1 yellow cards, or
/// with number 1 a yellow card in a match, 2 a sending-off, 3 misconduct, 4 yellow cards in successive matches; a match
/// form by the club's result). The scope text (`Discipline::get_scope_text` of the four rule sets, appeal pending 0,
/// form 5 for one match, else 6) is only formatted for a ban in matches; a ban in days passes 0x64 first and, in place
/// of the scope's gender, the appeal byte (slot 0), with the scope buffer never written (the copy's is empty). Then
/// the clubs' short genders (match forms), the name, the count (N matches), the scope, and the number (plain forms)
/// or the club, the score and the opponent (match forms). Returns false when the kind has no template (the original
/// leaves the buffer unwritten).
unsafe fn further_ban_sentence(this: usize, b: &FurtherBan, np: usize, sp: usize, tp: usize) -> bool {
    let fmt = news_text_format();
    let scope_text: unsafe extern "thiscall" fn(usize, *const u8, usize, u32, u32, u32) -> i8 = core::mem::transmute(FN_DISCIPLINE_GET_SCOPE_TEXT);
    let mut scope = [0u8; 0x64];
    let scp = scope.as_mut_ptr() as usize;
    let row = if b.count < 0 { 2 } else if b.count == 1 { 0 } else { 1 };
    let mut a = BanArgs::new();
    if row == 2 {
        a.push(0x64);
        a.gender(b.appealed as i8);
    } else {
        a.gender(scope_text(this, b.rules.as_ptr(), scp, 0x64, 0, if row == 0 { 5 } else { 6 }));
    }
    let (g1, g2) = b.m.goals();
    let result = if g1 > g2 { 0 } else if g1 < g2 { 1 } else { 2 };
    let (template, match_form) = match b.kind {
        0 => (FURTHER_BAN_POINTS[row], false),
        1 if b.number == 1 => (FURTHER_BAN_YELLOW_IN_MATCH[row][result], true),
        1 => (FURTHER_BAN_YELLOWS[row], false),
        4 => (FURTHER_BAN_SUCCESSIVE_YELLOWS[row], false),
        2 => (FURTHER_BAN_SENT_OFF[row][result], true),
        3 => (FURTHER_BAN_MISCONDUCT[row][result], true),
        _ => return false,
    };
    let (c1, c2) = (Club(b.m.club()), Club(b.m.opponent()));
    if match_form {
        a.gender(c1.short_gender_name() as i8);
        a.gender(c2.short_gender_name() as i8);
    }
    a.push(np);
    if row == 1 {
        a.push(b.count as i32 as usize);
    }
    a.push(scp);
    if match_form {
        a.push(c1.addr() + club::SHORT_NAME);
        a.push(sp);
        a.push(c2.addr() + club::SHORT_NAME);
    } else {
        a.push(b.number as i32 as usize);
    }
    covered!("sx09c_9 further ban template");
    let v = a.v;
    fmt(tp, template as usize, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12]);
    true
}

/// The slots of a further ban news item (0x7d3) that its writer reads: 0 the appeal made, 2 the count (matches; below
/// 0 a ban in days), 3 the days, 6..9 the rule sets, 10 the kind, 11 the number, 12 the appeal pending; slots 1 and 13
/// are read and dropped, 4, 5 and 14..16 skipped; from slot 17 the person (`read_staff_name_ref_from_news`), then,
/// past the match flag (not read), the match record (`read_match_record_from_news`, always read).
struct FurtherBan {
    appealed: u8,
    count: i8,
    days: i16,
    rules: [u8; 4],
    kind: u8,
    number: i8,
    pending: u32,
    person: [u8; 0x18],
    m: BanMatch,
}

impl FurtherBan {
    unsafe fn read(news: usize) -> Self {
        let slot = |k: u32| exe::newsitem_get_slot(news, k);
        let appealed = slot(0) as u8;
        slot(1);
        let count = slot(2) as u8 as i8;
        let days = slot(3) as u16 as i16;
        let rules = [slot(6) as u8, slot(7) as u8, slot(8) as u8, slot(9) as u8];
        let kind = slot(10) as u8;
        let number = slot(11) as u8 as i8;
        let pending = slot(12);
        slot(13);
        let mut b = FurtherBan { appealed, count, days, rules, kind, number, pending, person: [0; 0x18], m: BanMatch([0; 0x20]) };
        b.person[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
        b.m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
        let mut next: u8 = 17;
        exe::read_staff_name_ref_from_news(news, &mut next, b.person.as_mut_ptr());
        next = next.wrapping_add(1);
        exe::read_match_record_from_news(news, &mut next, b.m.0.as_mut_ptr());
        b
    }
    /// The person's name ids and staff id (four dwords of the name reference).
    unsafe fn ids(&self) -> (u32, u32, u32, u32) {
        let p = self.person.as_ptr() as usize;
        (read_u32(p), read_u32(p + 4), read_u32(p + 8), read_u32(p + 0xc))
    }
}

/// Writes the headline or the body of the news that a player got a further ban (news item 0x7d3).
///
/// `Discipline::write_further_ban_news_text(news, body, out)`: the slots in `FurtherBan`. With an appeal pending (slot
/// 12) the main ban writer writes it (`write_ban_news_text`). The person's link holds when his names still match
/// (`exe::database_staff_matches_names`, with the date of the name reference), the score's when the fixture still
/// matches (`exe::match_stats_fixture_entry_matches`). Headline (the name in mode 5, 0xc8 bytes): "banned for one
/// match" 0x9c1230, "for <n> matches" 0x9c11e4 or, in days, "for <length>" 0x9c1198 (`format_duration_text`, 0x1e
/// bytes), and the person's link (0x7d9). Body (the name in mode 1): the score "<more>-<fewer>" (0x9c116c), the
/// sentence of `further_ban_sentence`, "You have appealed..." (0x9b8f68) after a blank line when the appeal was made,
/// into the shared buffer (`FN_CRT_SPRINTF`, "%s\n\n%s" or "%s"); the links of the person, the two clubs' short names
/// (0x7d5, always: the match record is always read) and the score (0x7d3).
/// Original: `Discipline::write_further_ban_news_text` (0x55b8b0, discipline.cpp, thiscall, ret 0xc). Defects of the
/// original: a further ban in days formats the template with the scope buffer it never wrote (stack garbage) and the
/// appeal byte as the scope's gender; a kind above 4 hands over the template buffer it never wrote. The copy writes
/// an empty scope, or an empty sentence, and its comparer leaves the text out in those cases.
pub(crate) unsafe extern "thiscall" fn write_further_ban_news_text(this: usize, news: usize, body: u32, out: usize) {
    let b = FurtherBan::read(news);
    if b.pending != 0 {
        covered!("sx09c_9 further ban with an appeal pending");
        write_ban_news_text(this, news, body, out);
        return;
    }
    let fmt = news_text_format();
    let (first, second, common, id) = b.ids();
    let staff = staff_at(id as i32 as isize);
    let person_ok = exe::database_staff_matches_names(staff, first, second, common, b.person.as_ptr() as usize + 0x10) != 0;
    let m = &b.m;
    let fixture_ok = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), m.fixture() as i32, read_i32(m.at(0x11)),
        read_i32(m.at(0x15)), read_i32(m.at(0)), read_i32(m.at(2))) != 0;
    let mut name = [0u8; 0xc8];
    let np = name.as_ptr() as usize;
    if body == 0 {
        exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 5, id);
        covered!("sx09c_9 further ban headline");
        if b.count > -1 {
            if b.count == 1 {
                fmt(NEWS_TEXT_BUFFER, 0x9c1230, np); // "<name> banned for one match"
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9c11e4, np, b.count as i32); // "<name> banned for <n> matches"
            }
        } else {
            let mut length = [0u8; 0x20];
            format_duration_text(b.days as i32, length.as_mut_ptr() as usize, 0x1e, 0);
            fmt(NEWS_TEXT_BUFFER, 0x9c1198, np, length.as_ptr() as usize); // "<name> banned for <length>"
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if person_ok {
            exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
        }
        return;
    }
    exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 1, id);
    covered!("sx09c_9 further ban body");
    let mut score = [0u8; 0xc];
    let sp = score.as_mut_ptr() as usize;
    let (g1, g2) = m.goals();
    fmt(sp, 0x9c116c, g1.max(g2) as i32, g1.min(g2) as i32); // "<more>-<fewer>"
    let mut text = [0u8; 0x3e8];
    let tp = text.as_mut_ptr() as usize;
    further_ban_sentence(this, &b, np, sp, tp);
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    if b.appealed != 0 {
        let mut appeal = [0u8; 0x1f4];
        fmt(appeal.as_mut_ptr() as usize, 0x9b8f68); // "You have appealed against this decision..."
        crt_format(NEWS_TEXT_BUFFER, 0x991948, tp, appeal.as_ptr() as usize); // "%s\n\n%s"
    } else {
        crt_format(NEWS_TEXT_BUFFER, 0x991944, tp); // "%s"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if person_ok {
        exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
    }
    let (c1, c2) = (Club(m.club()), Club(m.opponent()));
    exe::text_add_link(out, (c1.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c1);
    exe::text_add_link(out, (c2.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c2);
    if fixture_ok {
        exe::text_add_link(out, score.as_ptr(), 0x7d3, m.fixture() as usize);
    }
}

/// Comparer for `write_further_ban_news_text`: the text side by side (`compare_text_writer`). With an appeal pending
/// only the original runs (it calls the main ban writer, whose comparer checks it). The body of a ban in days or of a
/// kind above 4 is compared without its text (the original formats stack garbage there): only the links and the
/// generator.
pub(crate) unsafe extern "thiscall" fn compare_write_further_ban_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    if slot(12) != 0 {
        o(this, news, body, out);
        return;
    }
    let what = || format!("write_further_ban_news_text(body {}, count {}, kind {})", body, slot(2) as u8 as i8, slot(10) as u8);
    if body == 0 || ((slot(2) as u8 as i8) >= 0 && slot(10) as u8 <= 4) {
        compare_text_writer(out, || o(this, news, body, out), |t| write_further_ban_news_text(this, news, body, t), what);
        return;
    }
    covered!("sx09c_9 further ban garbage text compared");
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, body, out),
        || crate::rng::without_counting(|| write_further_ban_news_text(this, news, body, lp)));
    let (mo, mr) = (text_contents(out).1, text_contents(lp).1);
    exe::drop_text(local.as_mut_ptr());
    SCOREBOARD.record(same_rng && mo == mr, || format!("{}: links original {:x?} rust {:x?}", what(), mo, mr));
}
