// ---------- sx05b_2: the team award news text and the week's dates (award/team_award.cpp) ----------

/// The award's staff competition record: its name (+4) and the gender byte of the name (+0x37, sign-extended, as the
/// texts pass it).
unsafe fn award_comp_name(award: usize) -> (usize, u32) {
    let comp = read_usize(award + award_base::STAFF_COMP);
    (comp + 4, read_i8(comp + 0x37) as i32 as u32)
}

/// Writes the headline or the body of the team award news (`team_award_send_club_news`'s item: slots 2 and 3 the
/// period years, 4 the count, 5 the club, 6.. the players' ids and name ids).
///
/// The headline: "<award name>" in the award's template, with the competition's link. The body: the period as "%d"
/// (one year) or "%d/%02d" (two, the second modulo 100); one player "<name> ... <award> ... <period>", two and three
/// players their own templates, any other count the count in words with a capital first letter, the club's short
/// name, the award and the period. Each named player (one to three) whose names still match the item gets his link
/// (0x7d7); with more than three, the club's (short name, 0x7d5); then the competition's (0x7d4). The players' names
/// are built (style 1, 0xc8 bytes each) before the headline test, as the original does.
/// Original: `TEAM_AWARD::write_news` (0x8b0370, team_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn team_award_write_news(this: usize, item: usize, text: usize, body: u32) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    let start = exe::newsitem_get_slot(item, 2);
    let end = exe::newsitem_get_slot(item, 3);
    let count = exe::newsitem_get_slot(item, 4) as u8 as i8;
    let club_id = exe::newsitem_get_slot(item, 5);
    let mut players = [([0u32; 4], [0u8; 0xc8]); 3];
    if count <= 3 {
        let mut s = 6i8;
        for (ids, name) in players.iter_mut().take(count.max(0) as usize) {
            for v in ids.iter_mut() {
                *v = exe::newsitem_get_slot(item, s as u8 as u32);
                s += 1;
            }
            exe::database_format_name_from_ids(ids[1], ids[2], ids[3], name.as_mut_ptr(), 0xc8, 1, ids[0]);
        }
    }
    let (comp_name, g) = award_comp_name(this);
    if body == 0 {
        fmt(buf, 0xa8c8b0, g, g, comp_name); // "{}<%s - Award name>{}"
        exe::text_set_string(text, buf as *const u8);
    } else {
        covered!("sx05b_2 team award body");
        let mut period = [0u8; 0x40];
        let pp = period.as_mut_ptr() as usize;
        if start as u16 != end as u16 {
            comp_crt_format()(pp, 0x988070, start & 0xffff, (end & 0xffff) % 100); // "%d/%02d"
        } else {
            comp_crt_format()(pp, 0x98806c, start & 0xffff); // "%d"
        }
        let name = |k: usize| players[k].1.as_ptr() as usize;
        match count {
            1 => fmt(buf, 0xa8cbcc, g, g, name(0), comp_name, pp),
            2 => fmt(buf, 0xa8cae0, g, g, name(0), name(1), comp_name, pp),
            3 => fmt(buf, 0xa8c9c8, g, g, name(0), name(1), name(2), comp_name, pp),
            _ => {
                covered!("sx05b_2 team award count in words");
                let mut words = [0u8; 0x34];
                exe::utils_number_to_words(count as i32, words.as_mut_ptr(), 0x32, 1);
                words[0] = exe::utils_char_to_upper(read_u32(words.as_ptr() as usize) as i32);
                let club = club_at(club_id as i32 as isize);
                let cg = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32 as u32;
                fmt(buf, 0xa8c8ec, cg, cg, g, g, words.as_ptr() as usize, club.addr() + club::SHORT_NAME, comp_name, pp)
            }
        };
        exe::text_set_string(text, buf as *const u8);
        if count > 3 {
            let club = club_at(club_id as i32 as isize);
            exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        } else {
            for (ids, name) in players.iter().take(count.max(0) as usize) {
                let person = staff_at(ids[0] as i32 as isize);
                if exe::database_staff_matches_names(person, ids[1], ids[2], ids[3], 0usize) != 0 {
                    exe::text_add_link(text, name.as_ptr(), 0x7d7, person);
                }
            }
        }
    }
    let comp = read_usize(this + award_base::STAFF_COMP);
    exe::text_add_link(text, (comp + 4) as *const u8, 0x7d4, comp);
}

/// Copies the three dates of a team award: the period start (+0x242) and end (+0x24a) and the update date (+0x1d),
/// each with `Date::operator=`.
/// Original: `TEAM_AWARD::set_dates` (0x8aff00, team_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn team_award_set_dates(this: usize, start: usize, end: usize, update: usize) {
    use team_award_obj as ta;
    let mut out = [0u8; 8];
    let o = out.as_mut_ptr() as usize;
    exe::date_operator_3d(this + ta::PERIOD_START, o, start);
    exe::date_operator_3d(this + ta::PERIOD_END, o, end);
    exe::date_operator_3d(this + ta::UPDATE_DATE, o, update);
}

/// Comparer for `team_award_write_news`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_team_award_write_news(this: usize, item: usize, text: usize, body: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%);
    compare_text_writer(text, || o(this, item, text, body), |t| team_award_write_news(this, item, t, body),
        || format!("team_award_write_news(count {}, body {})", exe::newsitem_get_slot(item, 4) as u8 as i8, body));
}

/// Comparer for `team_award_set_dates`: side by side over the three dates.
pub(crate) unsafe extern "thiscall" fn compare_team_award_set_dates(this: usize, start: usize, end: usize, update: usize) {
    use team_award_obj as ta;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I1%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + ta::PERIOD_START, 16), (this + ta::UPDATE_DATE, 8)],
        || o(this, start, end, update), || team_award_set_dates(this, start, end, update));
    SCOREBOARD.record(equal, || format!("team_award_set_dates({:#x}) {:?}", this, first_differences(&a, &b)));
}
