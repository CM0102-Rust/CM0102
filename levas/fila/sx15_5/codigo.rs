// ---------- batch %%LEVA%%: the news of the nations that qualified for a finals (national_teams.cpp) ----------

/// Takes from the new qualifiers list (`national_teams_fields::NEW_QUALIFIERS`, 8-byte items: national team, finals)
/// the national teams of the first finals still in it, clearing their items, into `teams` (club ids); slot 0 of
/// `news` gets the finals' id. Returns the finals (0 when none is left) and the count.
pub(crate) unsafe fn take_qualifiers_of_one_finals(list: Llist, news: usize, teams: &mut [u32; 256]) -> (usize, i8) {
    let mut finals = 0usize;
    let mut count: i8 = 0;
    list.set_cursor(list.head());
    let mut item = exe::llist_get_next(list);
    while item != 0 {
        if read_usize(item) != 0 {
            if finals == 0 {
                finals = read_usize(item + 4);
                exe::newsitem_set_slot(news, 0, read_u32(finals));
                count = 0;
            }
            if read_usize(item + 4) == finals {
                teams[count as u8 as usize] = read_u32(read_usize(item));
                count = count.wrapping_add(1);
                write_usize(item, 0); // taken
            }
        }
        item = exe::llist_get_next(list);
    }
    (finals, count)
}

/// Orders the qualifiers by reputation (`.reputation()` as a signed word), highest first, by swapping pairs (i, j)
/// with i < j < count − 1: the last one never moves (defect of the original: with two qualifiers nothing is sorted).
pub(crate) unsafe fn sort_qualifiers_by_reputation(teams: &mut [u32; 256], count: i8) {
    let n = count as i32;
    let mut i: i8 = 0;
    while (i as i32) < n - 2 {
        let mut j = i + 1;
        while (j as i32) < n - 1 {
            let (a, b) = (teams[i as u8 as usize], teams[j as u8 as usize]);
            if club_at(a as i32 as isize).reputation() < club_at(b as i32 as isize).reputation() {
                teams[i as u8 as usize] = b;
                teams[j as u8 as usize] = a;
            }
            j += 1;
        }
        i += 1;
    }
}

/// Posts one news item (type 0x2c) per finals with the nations that qualified for it and empties the list.
///
/// With new qualifiers (`national_teams_fields::NEW_QUALIFIERS`, an LLIST of national team and finals), one local news
/// item is built (`exe::new_newsitem`, 0x2c) and, while the list has a finals not yet taken
/// (`take_qualifiers_of_one_finals`): slot 0 the finals' id, 1 the count, 2 `utils_random(4)` (the headline wording),
/// 3 `utils_random(2)` (the body wording), 4 and 5 the next hosts of that finals (`exe::hostcountry_find_next_hosts`,
/// 0 and −1 when none), 6 the competition object's year (`cup::YEAR` in `G_COMP_OBJECT_TABLE`) and from 7 the
/// national teams (club ids), ordered by reputation (`sort_qualifiers_by_reputation`). The European Championship
/// (`KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`) goes to the managers of Europe (`exe::news_send_to_continent_managers`,
/// the `KEY_CONTINENT_EUROPE` record), any other finals to every manager (`exe::news_send_to_all_managers`). The same
/// news item carries over between finals (slots past the count keep the earlier ones). Finally the list is cleared.
/// Original: `NationalTeams::post_qualified_nations_news` (0x7779e0, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_qualified_nations_news(this: usize) {
    let list = Llist(this + national_teams_fields::NEW_QUALIFIERS);
    if read_i16(list.addr()) == 0 {
        return;
    }
    let mut news = [0u32; 0x38];
    let nt = news.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0x2c, 0);
    let mut teams = [0u32; 256];
    loop {
        let (finals, count) = take_qualifiers_of_one_finals(list, nt, &mut teams);
        if finals == 0 {
            break;
        }
        covered!("sx15_5 qualified nations news");
        exe::newsitem_set_slot(nt, 1, count as i32 as u32);
        exe::newsitem_set_slot(nt, 2, exe::utils_random(4) as u32);
        exe::newsitem_set_slot(nt, 3, exe::utils_random(2) as u32); // sx15_5: the body wording
        let (mut host, mut co_host) = (0u32, u32::MAX);
        exe::hostcountry_find_next_hosts(read_usize(G_HOST_COUNTRY), read_u32(finals), &mut host, &mut co_host);
        exe::newsitem_set_slot(nt, 4, host);
        exe::newsitem_set_slot(nt, 5, co_host);
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(finals).wrapping_mul(4)));
        exe::newsitem_set_slot(nt, 6, read_i16(comp + cup::YEAR) as i32 as u32);
        sort_qualifiers_by_reputation(&mut teams, count);
        let mut k: i8 = 0;
        while k < count {
            exe::newsitem_set_slot(nt, k.wrapping_add(7), teams[k as u8 as usize]);
            k += 1;
        }
        if read_u32(finals) == read_u32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) {
            let europe = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_EUROPE).wrapping_mul(0xc6));
            exe::news_send_to_continent_managers(read_usize(G_NEWS), nt, europe);
        } else {
            exe::news_send_to_all_managers(read_usize(G_NEWS), nt);
        }
    }
    exe::llist_clear(list);
}

/// Comparer for `post_qualified_nations_news`: it draws, sends news and empties the list (it cannot run twice), so only
/// the original runs; the plan checked afterwards is the empty list.
pub(crate) unsafe extern "thiscall" fn compare_post_qualified_nations_news(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let left = read_i16(this + national_teams_fields::NEW_QUALIFIERS);
    SCOREBOARD.record(left == 0, || format!("post_qualified_nations_news: {} qualifiers left in the list", left));
}

/// The slots of a qualified nations news item, as the writer reads them.
pub(crate) struct QualifiedNations {
    pub(crate) comp: Competition,
    pub(crate) count: i32,
    pub(crate) headline_kind: i32,
    pub(crate) body_kind: i32,
    pub(crate) host: i32,
    pub(crate) co_host: i32,
    pub(crate) year: i32,
    pub(crate) teams: Vec<Club>,
}

/// The national teams after the first one as one phrase in `out`: one 0xa641a8, two 0xa64164, three 0xa64100 by
/// name; with four the last three (0xa64100), five the last four (0xa6407c), more the 2nd to 6th (0xa63fdc) (the first
/// one is named on its own in those bodies). Nothing for a count below 1.
pub(crate) unsafe fn qualified_teams_phrase(q: &QualifiedNations, out: &mut [u8; 1000]) {
    let fmt = news_text_format();
    let o = out.as_mut_ptr() as usize;
    let g = |k: usize| q.teams[k].short_gender_name() as i8 as i32;
    let n = |k: usize| q.teams[k].addr() + club::SHORT_NAME;
    match q.count {
        1 => { fmt(o, 0xa641a8, g(0), g(0), n(0)); }
        2 => { fmt(o, 0xa64164, g(0), g(0), g(1), g(1), n(0), n(1)); }
        3 => { fmt(o, 0xa64100, g(0), g(0), g(1), g(1), g(2), g(2), n(0), n(1), n(2)); }
        4 => { fmt(o, 0xa64100, g(1), g(1), g(2), g(2), g(3), g(3), n(1), n(2), n(3)); }
        5 => { fmt(o, 0xa6407c, g(1), g(1), g(2), g(2), g(3), g(3), g(4), g(4), n(1), n(2), n(3), n(4)); }
        c if c > 5 => { fmt(o, 0xa63fdc, g(1), g(1), g(2), g(2), g(3), g(3), g(4), g(4), g(5), g(5), n(1), n(2), n(3), n(4), n(5)); }
        _ => {}
    }
}

/// The headline by the drawn wording (slot 2): 0 by the count (one 0xa6425c, two 0xa64208, more 0xa641c8 with the
/// count in words), 1 the competition (0xa642a0), 2 with a single host its name (0xa642e8), else 0xa64348, 3 0xa64384.
pub(crate) unsafe fn write_qualified_nations_headline(q: &QualifiedNations, words: &[u8; 0x78]) {
    let fmt = news_text_format();
    let g = |k: usize| q.teams[k].short_gender_name() as i8 as i32;
    let n = |k: usize| q.teams[k].addr() + club::SHORT_NAME;
    let host = Nation(nation_by_id(q.host as usize));
    match q.headline_kind {
        0 if q.count == 1 => { fmt(NEWS_TEXT_BUFFER, 0xa6425c, g(0), g(0), n(0), q.year); }
        0 if q.count == 2 => { fmt(NEWS_TEXT_BUFFER, 0xa64208, g(0), g(0), g(1), g(1), n(0), n(1)); }
        0 => { fmt(NEWS_TEXT_BUFFER, 0xa641c8, words.as_ptr() as usize, q.year); }
        1 => {
            let cg = q.comp.short_gender_name() as i8 as i32;
            fmt(NEWS_TEXT_BUFFER, 0xa642a0, cg, cg, q.comp.short_name_addr(), q.year);
        }
        2 if q.co_host < 0 => {
            let hg = host.short_gender_name() as i8 as i32;
            fmt(NEWS_TEXT_BUFFER, 0xa642e8, g(0), g(0), hg, hg, n(0), host.addr() + nation::SHORT_NAME, q.year);
        }
        2 => { fmt(NEWS_TEXT_BUFFER, 0xa64348, g(0), g(0), n(0), q.year); }
        _ => { fmt(NEWS_TEXT_BUFFER, 0xa64384, g(0), g(0), n(0), q.year); }
    }
}

/// The body by the drawn wording (slot 3, 0 or else) and the count (more than six, four to six, one, two or three),
/// each with one host or two (slot 5 ≥ 0): the first team by name, the competition (long name, gender +0x37), the
/// hosts, the year and the phrase of the others (`qualified_teams_phrase`); wording 0 with many teams also the count in
/// words. Models (one host / two): wording 0: 0xa6357c/0xa63694, 0xa6332c/0xa63440, 0xa631cc/0xa63268,
/// 0xa63034/0xa630ec; wording 1: 0xa63d88/0xa63e9c, 0xa63b40/0xa63c50, 0xa639a8/0xa63a60, 0xa637d4/0xa638a8.
pub(crate) unsafe fn write_qualified_nations_body(q: &QualifiedNations, words: &[u8; 0x78], others: &[u8; 1000]) {
    let fmt = news_text_format();
    let (t0g, t0n) = (q.teams[0].short_gender_name() as i8 as i32, q.teams[0].addr() + club::SHORT_NAME);
    let cg = q.comp.gender_name() as i8 as i32;
    let cn = q.comp.name_addr();
    let host = Nation(nation_by_id(q.host as usize));
    let co_host = Nation(nation_by_id(q.co_host as usize));
    let (hg, hn) = (host.short_gender_name() as i8 as i32, host.addr() + nation::SHORT_NAME);
    let (h2g, h2n) = (co_host.short_gender_name() as i8 as i32, co_host.addr() + nation::SHORT_NAME);
    let (w, l, y) = (words.as_ptr() as usize, others.as_ptr() as usize, q.year);
    let two_hosts = q.co_host >= 0;
    let many = |one: usize, two: usize| {
        if q.body_kind == 0 {
            if two_hosts {
                fmt(NEWS_TEXT_BUFFER, two, 100, t0g, t0g, cg, cg, hg, hg, h2g, h2g, 100, w, t0n, y, cn, hn, h2n, l);
            } else {
                fmt(NEWS_TEXT_BUFFER, one, 100, t0g, t0g, cg, cg, hg, hg, 100, w, t0n, y, cn, hn, l);
            }
        } else if two_hosts {
            fmt(NEWS_TEXT_BUFFER, two, t0g, t0g, cg, cg, hg, hg, h2g, h2g, 100, t0n, y, cn, hn, h2n, l);
        } else {
            fmt(NEWS_TEXT_BUFFER, one, t0g, t0g, cg, cg, hg, hg, 100, t0n, y, cn, hn, l);
        }
    };
    let (zero, one) = (q.body_kind == 0, q.body_kind != 0);
    match q.count {
        c if c > 6 && zero => many(0xa6357c, 0xa63694),
        c if c > 6 => many(0xa63d88, 0xa63e9c),
        c if c > 3 && zero => many(0xa6332c, 0xa63440),
        c if c > 3 => many(0xa63b40, 0xa63c50),
        1 => {
            let (m1, m2) = if one { (0xa639a8, 0xa63a60) } else { (0xa631cc, 0xa63268) };
            if two_hosts {
                fmt(NEWS_TEXT_BUFFER, m2, t0g, t0g, cg, cg, hg, hg, h2g, h2g, t0n, y, cn, hn, h2n);
            } else {
                fmt(NEWS_TEXT_BUFFER, m1, t0g, t0g, cg, cg, hg, hg, t0n, y, cn, hn);
            }
        }
        _ => {
            let (m1, m2) = if one { (0xa637d4, 0xa638a8) } else { (0xa63034, 0xa630ec) }; // sx15_5: two or three teams
            if two_hosts {
                fmt(NEWS_TEXT_BUFFER, m2, 100, cg, cg, hg, hg, h2g, h2g, l, y, cn, hn, h2n);
            } else {
                fmt(NEWS_TEXT_BUFFER, m1, 100, cg, cg, hg, hg, l, y, cn, hn);
            }
        }
    }
}

/// Writes the headline or the body of the news (type 0x2c) listing the nations that qualified for a finals.
///
/// Reads the slots (`post_qualified_nations_news`): 0 the finals (club competition id), 1 the count, 2 and 3 the drawn
/// wordings, 4 and 5 the hosts (nation ids, −1 none), 6 the year and 7.. the national teams (club ids, by reputation),
/// and the count in words (`exe::utils_number_to_words`, 0x78 bytes). The headline: `write_qualified_nations_headline`;
/// the body: the others' phrase (`qualified_teams_phrase`) and `write_qualified_nations_body`. Then the text and the
/// links: each team's long and short name (0x7d5), each host's short name to its national team (0x7d5,
/// `exe::database_get_national_team`) and the competition's long and short name (2000).
/// Original: `NationalTeams::write_qualified_nations_text` (0x777ca0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_qualified_nations_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let count = slot(1) as i32;
    let mut q = QualifiedNations {
        comp: Competition(club_comp_record(slot(0) as usize)),
        count,
        headline_kind: slot(2) as i32,
        body_kind: slot(3) as i32,
        host: slot(4) as i32,
        co_host: slot(5) as i32,
        year: slot(6) as i32,
        teams: Vec::new(),
    };
    let mut k: i8 = 7;
    while (k as i32) < count + 7 {
        q.teams.push(club_at(slot(k as u8 as u32) as i32 as isize));
        k = k.wrapping_add(1);
    }
    // ponytail: the poster always sends at least one team; with none the original would read stale stack, the copy club 0
    q.teams.resize(q.teams.len().max(1), club_at(0));
    let mut words = [0u8; 0x78];
    exe::utils_number_to_words(count, words.as_mut_ptr(), 0x78, 0);
    if body == 0 {
        covered!("sx15_5 qualified nations headline");
        write_qualified_nations_headline(&q, &words);
    } else {
        covered!("sx15_5 qualified nations body");
        let mut others = [0u8; 1000];
        qualified_teams_phrase(&q, &mut others);
        write_qualified_nations_body(&q, &words, &others);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    let mut k: i8 = 0;
    while (k as i32) < count {
        let t = q.teams[k as u8 as usize];
        exe::text_add_link(out, t.name_addr() as *const u8, 0x7d5, t);
        exe::text_add_link(out, (t.addr() + club::SHORT_NAME) as *const u8, 0x7d5, t);
        k = k.wrapping_add(1);
    }
    for id in [q.host, q.co_host] {
        if id > -1 {
            let nation = nation_by_id(id as usize);
            exe::text_add_link(out, (nation + nation::SHORT_NAME) as *const u8, 0x7d5, exe::database_get_national_team(nation));
        }
    }
    exe::text_add_link(out, q.comp.name_addr() as *const u8, 2000, q.comp); // sx15_5: the competition links
    exe::text_add_link(out, q.comp.short_name_addr() as *const u8, 2000, q.comp);
}

/// Comparer for `write_qualified_nations_text`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_qualified_nations_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    compare_text_writer(out, || o(this, news, body, out), |t| write_qualified_nations_text(this, news, body, t),
        || format!("write_qualified_nations_text(finals {}, count {}, kinds {} {}, hosts {} {}, body {})", slot(0) as i32, slot(1) as i32, slot(2) as i32, slot(3) as i32, slot(4) as i32, slot(5) as i32, body));
}
