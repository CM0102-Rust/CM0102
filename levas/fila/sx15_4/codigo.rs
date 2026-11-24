// ---------- batch %%LEVA%%: the national call-up news text (national_teams.cpp) ----------

/// Up to three called-up players read from a call-up news item (slots 10.., four per player: id, first, second and
/// common name ids) with their names formatted, the repeated ones dropped.
pub(crate) struct CallUpPlayers {
    pub(crate) ids: [u32; 3],
    pub(crate) first: [u32; 3],
    pub(crate) second: [u32; 3],
    pub(crate) common: [u32; 3],
    pub(crate) names: [[u8; 200]; 3],
}

/// Reads the `count` (< 4) players of the call-up news: player k from slot 10 + 4k (id, first, second, common name
/// ids), his name formatted into `names[i]` (`exe::database_format_name_from_ids`, mode 1 in the body, 5 in the
/// headline). A player whose id repeats an earlier one is read but then overwritten by the next one, and still counts
/// as a read: the loop runs until `count` different players are kept, reading further slots. Returns the players and
/// the number of repeats.
pub(crate) unsafe fn read_call_up_players(news: usize, count: i8, body: bool) -> (CallUpPlayers, i8) {
    let mut p = CallUpPlayers { ids: [0; 3], first: [0; 3], second: [0; 3], common: [0; 3], names: [[0; 200]; 3] };
    let mut slot: u8 = 10;
    let mut repeats: i8 = 0;
    let mut i: i8 = 0;
    while i < count {
        let k = i as usize;
        p.ids[k] = exe::newsitem_get_slot(news, slot as u32);
        p.first[k] = exe::newsitem_get_slot(news, slot.wrapping_add(1) as u32);
        p.second[k] = exe::newsitem_get_slot(news, slot.wrapping_add(2) as u32);
        p.common[k] = exe::newsitem_get_slot(news, slot.wrapping_add(3) as u32);
        slot = slot.wrapping_add(4);
        let mode = if body { 1 } else { 5 };
        exe::database_format_name_from_ids(p.first[k], p.second[k], p.common[k], p.names[k].as_mut_ptr(), 200, mode, p.ids[k]);
        if (0..i).any(|j| p.ids[j as usize] == p.ids[k]) {
            covered!("sx15_4 repeated player");
            i -= 1; // the next player takes this place
            repeats += 1;
        }
        i += 1;
    }
    (p, repeats)
}

/// Links each called-up player's name (0x7d7) when the person still has those names
/// (`exe::database_staff_matches_names`).
pub(crate) unsafe fn link_call_up_players(out: usize, p: &CallUpPlayers, count: i8) {
    for k in 0..count.max(0) as usize {
        let person = staff_at(p.ids[k] as i32 as isize);
        if exe::database_staff_matches_names(person, p.first[k], p.second[k], p.common[k], 0usize) != 0 {
            exe::text_add_link(out, p.names[k].as_ptr(), 0x7d7, person);
        }
    }
}

/// The number of players in words with a capital first letter (`exe::utils_number_to_words`, 0x32 bytes, then
/// `exe::utils_char_to_upper` on the first byte).
pub(crate) unsafe fn players_in_words(count: i8) -> [u8; 0x34] {
    let mut words = [0u8; 0x34];
    exe::utils_number_to_words(count as i32, words.as_mut_ptr(), 0x32, 1);
    words[0] = exe::utils_char_to_upper(read_i32(words.as_ptr() as usize));
    words
}

/// The opponents of the call-up as one phrase in `out`: one club, its short name; two, "A and B" (0xa5c134), or one
/// name when they are the same club; three, "A, B and C" (0xa5c0f8), with the repeats folded into the two-club phrase
/// or one name. With three where the first two are the same club, the second slot takes the third club (the links
/// read it later). Other counts leave `out` as it is.
pub(crate) unsafe fn write_call_up_opponents(clubs: &mut [Club; 3], count: i8, out: &mut [u8; 300]) {
    let fmt = news_text_format();
    let gender = |c: Club| c.short_gender_name() as i8 as i32;
    let short = |c: Club| c.addr() + club::SHORT_NAME;
    let copy = |c: Club, out: &mut [u8; 300]| {
        let name = core::slice::from_raw_parts(short(c) as *const u8, 26);
        let n = name.iter().position(|&b| b == 0).unwrap_or(25) + 1;
        out[..n].copy_from_slice(&name[..n]);
    };
    let two = |a: Club, b: Club, out: &mut [u8; 300]| {
        fmt(out.as_mut_ptr() as usize, 0xa5c134, gender(a), gender(a), gender(b), gender(b), short(a), short(b));
    };
    match count {
        1 => copy(clubs[0], out),
        2 if clubs[0] == clubs[1] => copy(clubs[0], out),
        2 => two(clubs[0], clubs[1], out),
        3 if clubs[0] == clubs[1] => {
            clubs[1] = clubs[2];
            if clubs[0] != clubs[2] {
                two(clubs[0], clubs[1], out);
            } else {
                copy(clubs[0], out);
            }
        }
        3 if clubs[0] == clubs[2] => two(clubs[0], clubs[1], out),
        3 if clubs[1] == clubs[2] => two(clubs[1], clubs[2], out),
        3 => {
            covered!("sx15_4 three opponents");
            let (a, b, c) = (clubs[0], clubs[1], clubs[2]);
            fmt(out.as_mut_ptr() as usize, 0xa5c0f8, gender(a), gender(a), gender(b), gender(b), gender(c), gender(c), short(a), short(b), short(c));
        }
        _ => {}
    }
}

/// The call-up news without opponents (slot 2 set): the headline "<national team> name squad for <competition>"
/// (0xa5c15c) with the national team (long name) and competition links, or the body by the player count: one
/// (0xa5c444), two (0xa5c378), three (0xa5c280) by name, more (0xa5c1c0) with the count in words and the club; then the
/// player links (or the club's, 0x7d5, with four or more), the national team's short name (0x7d5) and the
/// competition (2000).
pub(crate) unsafe fn write_call_up_without_opponents(out: usize, body: bool, team: Club, club_: Club, comp: Competition, p: &CallUpPlayers, count: i8) {
    let fmt = news_text_format();
    let tg = team.gender_name() as i32;
    let cg = comp.short_gender_name() as i8 as i32;
    let (tname, cname) = (team.name_addr(), comp.short_name_addr());
    if !body {
        fmt(NEWS_TEXT_BUFFER, 0xa5c15c, tg, tg, cg, cg, tname, cname);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, tname as *const u8, 0x7d5, team);
        exe::text_add_link(out, cname as *const u8, 2000, comp);
        return;
    }
    let n = |k: usize| p.names[k].as_ptr() as usize;
    match count {
        1 => { fmt(NEWS_TEXT_BUFFER, 0xa5c444, tg, tg, cg, cg, n(0), tname, cname); }
        2 => { fmt(NEWS_TEXT_BUFFER, 0xa5c378, tg, tg, cg, cg, n(0), n(1), tname, cname); }
        3 => { fmt(NEWS_TEXT_BUFFER, 0xa5c280, tg, tg, cg, cg, n(0), n(1), n(2), tname, cname); }
        _ => {
            let words = players_in_words(count);
            let g = club_.short_gender_name() as i8 as i32;
            fmt(NEWS_TEXT_BUFFER, 0xa5c1c0, g, g, tg, tg, cg, cg, words.as_ptr() as usize, club_.addr() + club::SHORT_NAME, tname, cname);
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if count < 4 {
        link_call_up_players(out, p, count);
    } else {
        exe::text_add_link(out, (club_.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club_);
    }
    exe::text_add_link(out, (team.addr() + club::SHORT_NAME) as *const u8, 0x7d5, team);
    exe::text_add_link(out, cname as *const u8, 2000, comp);
}

/// The headline of the call-up news with opponents: one player "<name> selected by <national team>" (0xa5b710 for the
/// first team, 0xa5b6b4 for the second, 0xa5b654 when `exe::club_is_national_team` says the national team is not a
/// first or second side [inference]), with the player's link written before the text; more players "<club> players
/// called up by <national team>" (0xa5b5f0, 0xa5b588, 0xa5b51c the same way). Then the club's link (0x7d5) and the
/// national team's (0x7d5 first team, 0x7d6 second).
pub(crate) unsafe fn write_call_up_headline(out: usize, team: Club, club_: Club, second: i32, p: &CallUpPlayers, count: i8) {
    let fmt = news_text_format();
    let tg = team.short_gender_name() as i8 as i32;
    let tshort = team.addr() + club::SHORT_NAME;
    let special = exe::club_is_national_team(team) != 0;
    if count == 1 {
        let model = if special { 0xa5b654 } else if second == 0 { 0xa5b710 } else { 0xa5b6b4 };
        fmt(NEWS_TEXT_BUFFER, model, tg, tg, p.names[0].as_ptr() as usize, tshort);
        link_call_up_players(out, p, 1); // before the text, like the original
    } else {
        let model = if special { 0xa5b51c } else if second == 0 { 0xa5b5f0 } else { 0xa5b588 }; // sx15_4: the players headline
        let g = club_.short_gender_name() as i8 as i32;
        fmt(NEWS_TEXT_BUFFER, model, g, g, tg, tg, club_.addr() + club::SHORT_NAME, tshort);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, (club_.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club_);
    exe::text_add_link(out, tshort as *const u8, if second == 0 { 0x7d5 } else { 0x7d6 }, team);
}

/// The body of the call-up news with opponents, by the player count and the opponent count (`opponents`, slot 6):
/// - one player: one opponent 0xa5c050 (with the opponent's gender), up to three 0xa5bfa0, more 0xa5bf18 (without the
///   opponents);
/// - two: 0xa5be40, 0xa5bd60, 0xa5bca8; three: 0xa5bba4, 0xa5ba98, 0xa5b9b4, the same way;
/// - four or more (the count in words and the club): 0xa5b8e8, 0xa5b814, 0xa5b768.
/// `team_name` is the national team's name (`exe::club_get_national_team_name`), `versus` the opponents phrase.
/// Then the player links (or the club's), the national team's name (0x7d5, second team 0x7d6) and each opponent's.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn write_call_up_body(out: usize, team: Club, club_: Club, second: i32, p: &CallUpPlayers, count: i8,
    opponents: i8, clubs: &[Club; 3], team_name: &[u8; 100], versus: &[u8; 300]) {
    let fmt = news_text_format();
    let tg = team.short_gender_name() as i8 as i32;
    let og = clubs[0].short_gender_name() as i8 as i32;
    let (tn, vs) = (team_name.as_ptr() as usize, versus.as_ptr() as usize);
    let n = |k: usize| p.names[k].as_ptr() as usize;
    match count {
        1 if opponents == 1 => { fmt(NEWS_TEXT_BUFFER, 0xa5c050, tg, tg, og, og, n(0), tn, vs); }
        1 if opponents < 4 => { fmt(NEWS_TEXT_BUFFER, 0xa5bfa0, tg, tg, n(0), tn, vs); }
        1 => { fmt(NEWS_TEXT_BUFFER, 0xa5bf18, tg, tg, n(0), tn); }
        2 if opponents == 1 => { fmt(NEWS_TEXT_BUFFER, 0xa5be40, tg, tg, og, og, n(0), n(1), tn, vs); }
        2 if opponents < 4 => { fmt(NEWS_TEXT_BUFFER, 0xa5bd60, tg, tg, 100, n(0), n(1), tn, vs); }
        2 => { fmt(NEWS_TEXT_BUFFER, 0xa5bca8, tg, tg, n(0), n(1), tn); }
        3 if opponents == 1 => { fmt(NEWS_TEXT_BUFFER, 0xa5bba4, tg, tg, og, og, n(0), n(1), n(2), tn, vs); }
        3 if opponents < 4 => { fmt(NEWS_TEXT_BUFFER, 0xa5ba98, tg, tg, 100, n(0), n(1), n(2), tn, vs); }
        3 => { fmt(NEWS_TEXT_BUFFER, 0xa5b9b4, tg, tg, n(0), n(1), n(2), tn); }
        _ => {
            let words = players_in_words(count);
            let w = words.as_ptr() as usize;
            let g = club_.short_gender_name() as i8 as i32;
            let cshort = club_.addr() + club::SHORT_NAME;
            if opponents == 1 {
                fmt(NEWS_TEXT_BUFFER, 0xa5b8e8, g, g, tg, tg, og, og, w, cshort, tn, vs);
            } else if opponents < 4 {
                fmt(NEWS_TEXT_BUFFER, 0xa5b814, g, g, tg, tg, w, cshort, tn, vs); // sx15_4: many players, the opponents
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa5b768, g, g, tg, tg, w, cshort, tn);
            }
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if count < 4 {
        link_call_up_players(out, p, count);
    } else {
        exe::text_add_link(out, (club_.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club_);
    }
    let kind = if second == 0 { 0x7d5 } else { 0x7d6 };
    exe::text_add_link(out, team_name.as_ptr(), kind, team);
    if opponents < 4 {
        for c in clubs.iter().take(opponents.max(0) as usize) {
            exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, kind, *c);
        }
    }
}

/// Writes the headline or the body of the news (type 0x1d) that a club's players were called up by a national team.
///
/// The slots (`national_teams_post_call_up_news`): 0 the national team (club id), 1 the player's club, 2 no opponents,
/// 3 the competition, 4 the player count, 5 the second team, 6 the opponent count, 7..9 the opponents (club ids),
/// 10.. four per player (id and name ids). With fewer than four players their names are read
/// (`read_call_up_players`), repeats dropped from the count.
///
/// Without opponents (slot 2 set) the text names the competition (`write_call_up_without_opponents`). Otherwise the
/// national team's name is formatted (`exe::club_get_national_team_name`, 100 bytes, with the second-team flag), the
/// opponents read when there are one to three (`write_call_up_opponents`) and the headline
/// (`write_call_up_headline`) or the body (`write_call_up_body`) written.
///
/// Defect of the original: with opponents and none to read (slot 6 zero or negative), the bodies that name the
/// opponents (up to three) append a 300-byte stack buffer that was never written (stack garbage); the copy writes an
/// empty phrase and the comparer leaves that case to the original.
/// Original: `NationalTeams::write_call_up_news_text` (0x76e3b0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_call_up_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let team = club_at(slot(0) as i32 as isize);
    let club_ = club_at(slot(1) as i32 as isize);
    let no_opponents = slot(2) as u8 != 0;
    let comp = Competition(club_comp_record(slot(3) as usize));
    let mut count = slot(4) as u8 as i8;
    let second = slot(5) as u8 as i8 as i32;
    let body = body != 0;
    let mut players = CallUpPlayers { ids: [0; 3], first: [0; 3], second: [0; 3], common: [0; 3], names: [[0; 200]; 3] };
    if count < 4 {
        let (p, repeats) = read_call_up_players(news, count, body);
        players = p;
        count -= repeats;
    }
    if no_opponents {
        covered!("sx15_4 call-up without opponents");
        write_call_up_without_opponents(out, body, team, club_, comp, &players, count);
        return;
    }
    let mut team_name = [0u8; 100];
    exe::club_get_national_team_name(team, second as usize, team_name.as_mut_ptr() as usize, 100, 2, 1);
    let opponents = slot(6) as u8 as i8;
    let mut clubs = [Club(0); 3];
    if opponents < 4 {
        for k in 0..opponents.max(0) as usize {
            clubs[k] = club_at(slot(7 + k as u32) as i32 as isize);
        }
    }
    let mut versus = [0u8; 300]; // the original leaves it unwritten with no opponent (defect)
    write_call_up_opponents(&mut clubs, opponents, &mut versus);
    if !body {
        covered!("sx15_4 call-up headline");
        write_call_up_headline(out, team, club_, second, &players, count);
    } else {
        covered!("sx15_4 call-up body");
        write_call_up_body(out, team, club_, second, &players, count, opponents, &clubs, &team_name, &versus);
    }
}

/// Comparer for `write_call_up_news_text`: side by side into two Texts (`compare_text_writer`). A body with opponents
/// and none to read appends the original's unwritten stack buffer (see the defect): only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_write_call_up_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    if body != 0 && slot(2) as u8 == 0 && slot(6) as u8 as i8 <= 0 {
        covered!("sx15_4 unwritten opponents");
        return o(this, news, body, out);
    }
    compare_text_writer(out, || o(this, news, body, out), |t| write_call_up_news_text(this, news, body, t),
        || format!("write_call_up_news_text(team {}, club {}, players {}, opponents {}, body {})", slot(0) as i32, slot(1) as i32, slot(4) as u8 as i8, slot(6) as u8 as i8, body));
}
