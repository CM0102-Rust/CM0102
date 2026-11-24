// ---------- batch %%LEVA%%: the national squad news of a human manager (national_teams.cpp) ----------

/// Builds news item 0x1e, the request that the human manager select a national squad, and delivers it to the team's
/// manager.
///
/// `national_teams_post_squad_request_news(team, variant, game)`, from `select_squad` when the squad is the human's to
/// pick. Slots: 0 the team id, 1 `variant` (the second-team flag of the national team name), 2 = 0, 3 = 1 when the
/// game's competition is one of the nine finals (`is_major_finals`), 4 the number of opponents, 5 the competition id,
/// 6 the game's day and 7 its year; with fewer than 4 opponents, 8.. their ids. Outside the finals the opponents come
/// from `exe::national_teams_collect_fixture_opponents` (the match opponent and the next 13 days' ones); in the finals
/// there is no list and the count is 0. Delivered (`exe::news_deliver_to_manager`) to the team's `.manager()`, or,
/// when it has none, to the manager of its nation's first team.
///
/// Defects of the original: the opponents list is allocated by the collector and never freed (a leak of one LLIST per
/// request); the competition id is read from the game's competition without the null check made just before (a game
/// without a competition would crash).
/// Original: `national_teams_post_squad_request_news` (0x76f1a0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn national_teams_post_squad_request_news(this: usize, team: usize, variant: u32, game: usize) {
    let team = Club(team);
    let game = Fixture(game);
    covered!("sx15_7 squad request news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1e, 0);
    let comp = game.competition();
    let finals = !comp.is_null() && is_major_finals(read_u32(comp.addr()));
    let (list, count) = if finals {
        (0, 0i16)
    } else {
        let list = exe::national_teams_collect_fixture_opponents(this, team, game);
        (list, read_i16(list)) // the list's item count
    };
    exe::newsitem_set_slot(n, 0, read_u32(team.addr()));
    exe::newsitem_set_slot(n, 1, variant);
    exe::newsitem_set_slot(n, 2, 0);
    exe::newsitem_set_slot(n, 3, finals as u32);
    exe::newsitem_set_slot(n, 4, count as i32 as u32);
    exe::newsitem_set_slot(n, 5, read_u32(game.competition().addr())); // sx15_7: the competition id
    exe::newsitem_set_slot(n, 6, game.day() as u32);
    exe::newsitem_set_slot(n, 7, game.year() as u32);
    if count < 4 && list != 0 && !finals {
        write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
        let mut k: u8 = 8;
        let mut item = exe::llist_get_next(list);
        while item != 0 {
            exe::newsitem_set_slot(n, k as i8, read_u32(read_usize(item)));
            k = k.wrapping_add(1);
            item = exe::llist_get_next(list);
        }
    }
    let mut manager = team.manager().addr();
    if manager == 0 {
        manager = Club(exe::database_get_national_team(read_usize(team.addr() + club::NATION))).manager().addr();
    }
    exe::news_deliver_to_manager(read_usize(G_NEWS), n, manager, 0);
}

/// Comparer for `national_teams_post_squad_request_news`: only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_national_teams_post_squad_request_news(this: usize, team: usize, variant: u32, game: usize) {
    // ponytail: the item goes into the human's inbox (allocates) and the opponents list is allocated: it cannot run
    // twice; the opponents are checked by the collector's own comparer, the text by write_squad_request_news_text
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%)(this, team, variant, game)
}

/// The team link of the national squad news: 0x7d6 for a second team (`variant` ≠ 0), else 0x7d5.
fn squad_link_kind(variant: u32) -> u32 {
    if variant != 0 { 0x7d6 } else { 0x7d5 }
}

/// The opponents of a squad request, as in "against Holland and Belgium": one name; two different ones joined by
/// "and" (0xa5c134); three: "a, b and c" (0xa5c0f8) when all differ, otherwise the two that differ, each club with its
/// `.short_gender_name()` twice and its short name. Other counts leave `buf` as it was.
///
/// Defect of the original: with three opponents where the second and the third are the same club and the first is
/// another, it writes "b and c" (the repeated club twice) and drops the first.
unsafe fn squad_request_opponents_text(opponents: &[Club; 3], count: i16, buf: usize) {
    let fmt = news_text_format();
    let two = |a: Club, b: Club| {
        let (ga, gb) = (a.short_gender_name() as i8 as i32, b.short_gender_name() as i8 as i32);
        fmt(buf, 0xa5c134, ga, ga, gb, gb, a.addr() + club::SHORT_NAME, b.addr() + club::SHORT_NAME);
    };
    let one = |a: Club| copy_c_string(buf, a.addr() + club::SHORT_NAME);
    let [a, b, c] = *opponents;
    match count {
        1 => one(a),
        2 if a != b => two(a, b),
        2 => one(a),
        3 if a == b && a != c => two(a, c),
        3 if a == b => one(a),
        3 if a == c => two(a, b),
        3 if b == c => two(b, c), // the original's slip: the first opponent is dropped
        3 => {
            let (ga, gb, gc) = (a.short_gender_name() as i8 as i32, b.short_gender_name() as i8 as i32, c.short_gender_name() as i8 as i32);
            fmt(buf, 0xa5c0f8, ga, ga, gb, gb, gc, gc, a.addr() + club::SHORT_NAME, b.addr() + club::SHORT_NAME, c.addr() + club::SHORT_NAME);
        }
        _ => {}
    }
}

/// Writes the headline or the body of news item 0x1e, the request that the human manager select a national squad.
///
/// The team is club `slot 0`, named as a national team (`exe::club_get_national_team_name`, 0x64 bytes, with `slot 1`
/// as the second-team flag). The headline: "<team> squad to be decided" (0xa5c4e0, the team's `.short_gender_name()`
/// twice) with the team link. The body, by the slots of `national_teams_post_squad_request_news` (3 the finals flag,
/// 4 the opponent count, 5 the competition id, 8.. the opponents): in the finals "select a <team> squad for the
/// <competition> finals" (0xa5c6b4, with the team's `.gender_name()` and the competition's, and its name); one
/// opponent "for the match against <opponent>" (0xa5c624); two or three "for the matches against <list>"
/// (0xa5c58c, `squad_request_opponents_text`); four or more "for the forthcoming matches" (0xa5c51c). Then the links:
/// each opponent (1 to 3), the team (0x7d6 for a second team, else 0x7d5, the opponents too) and, in the finals, the
/// competition (0x7d0).
///
/// Defects of the original: with no opponent outside the finals (not produced by the poster) the body joins a buffer
/// never written (stack leftover); the copy uses an empty one. The finals body takes the team's long-name gender
/// (`.gender_name()`, +0x37) for the national team name, the other bodies the short one (+0x52).
/// Original: `NationalTeams::write_squad_request_news_text` (0x76f370, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_squad_request_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut squad = [0u8; 0x64];
    let sq = squad.as_mut_ptr() as usize;
    if body == 0 {
        let team = club_at(slot(0) as i32 as isize);
        let variant = slot(1);
        exe::club_get_national_team_name(team, variant as usize, sq, 0x64, 2, 1);
        let g = team.short_gender_name() as i8 as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa5c4e0, g, g, sq);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, sq as *const u8, squad_link_kind(variant), team);
        return;
    }
    covered!("sx15_7 squad request text");
    let team = club_at(slot(0) as i32 as isize);
    let variant = slot(1);
    let finals = slot(3);
    let count = slot(4) as i16;
    let comp_id = slot(5);
    let mut opponents = [Club(0); 3];
    if count < 4 {
        for k in 0..count.max(0) as usize {
            opponents[k] = club_at(slot(8 + k as u32) as i32 as isize);
        }
    }
    let mut list = [0u8; 0x12c]; // the original's is never cleared (see the header)
    let lp = list.as_mut_ptr() as usize;
    squad_request_opponents_text(&opponents, count, lp);
    exe::club_get_national_team_name(team, variant as usize, sq, 0x64, 2, 1);
    let comp = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b)));
    let g = team.short_gender_name() as i8 as i32;
    if finals != 0 {
        let (gt, gc) = (team.gender_name() as i32, comp.gender_name() as i8 as i32);
        fmt(NEWS_TEXT_BUFFER, 0xa5c6b4, gt, gt, gc, gc, sq, comp.name_addr());
    } else if count == 1 {
        let go = opponents[0].short_gender_name() as i8 as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa5c624, g, g, go, go, sq, lp);
    } else if count < 4 {
        fmt(NEWS_TEXT_BUFFER, 0xa5c58c, g, g, sq, lp);
    } else {
        fmt(NEWS_TEXT_BUFFER, 0xa5c51c, g, g, sq);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if count < 4 {
        for o in opponents.iter().take(count.max(0) as usize) {
            exe::text_add_link(out, (o.addr() + club::SHORT_NAME) as *const u8, squad_link_kind(variant), *o); // sx15_7: the opponent links
        }
    }
    exe::text_add_link(out, sq as *const u8, squad_link_kind(variant), team);
    if finals != 0 {
        exe::text_add_link(out, comp.name_addr() as *const u8, 0x7d0, comp);
    }
}

/// Comparer for `write_squad_request_news_text`: side by side into two Texts (`compare_text_writer`); a body without
/// opponents outside the finals reads the original's uncleared buffer, so only the original runs then.
pub(crate) unsafe extern "thiscall" fn compare_write_squad_request_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    if body != 0 && exe::newsitem_get_slot(news, 3) == 0 && exe::newsitem_get_slot(news, 4) as i16 <= 0 {
        covered!("sx15_7 squad request without opponents");
        return o(this, news, body, out);
    }
    compare_text_writer(out, || o(this, news, body, out), |t| write_squad_request_news_text(this, news, body, t),
        || format!("write_squad_request_news_text(news {:#x}, body {})", news, body));
}

/// The national squad news of a player (items 0x20, 0x21 and 0x22): slots 0..3 the player and his name ids, 4 the
/// national team, 5 the second-team flag, 6 (when the two body templates differ) the injury severity.
///
/// The body: the player's name (mode 1) and the team's national name (`exe::club_get_national_team_name`, 0xc8
/// bytes, slot 5 as the second-team flag) in `body_templates` = (illness, injury): the illness one when slot 6 is 0,
/// the injury one otherwise (the same template twice: slot 6 is not read); the team's `.short_gender_name()` twice
/// first. The headline: the player's name (mode 5) and the team's short name in `headline_template`. Both set the text,
/// link the player (0x7d7) when the item's names still match him, and link the team's name (0x7d6 for a second team,
/// else 0x7d5).
unsafe fn write_player_squad_news_text(news: usize, body: u32, out: usize, body_templates: (usize, usize), headline_template: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (id, first, second, common) = (slot(0), slot(1), slot(2), slot(3));
    let team = club_at(slot(4) as i32 as isize);
    let g = team.short_gender_name() as i8 as i32;
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if body != 0 {
        let variant = slot(5);
        let (illness, injury) = body_templates;
        let template = if illness == injury || slot(6) as u8 == 0 { illness } else { injury };
        let mut squad = [0u8; 0xc8];
        let sq = squad.as_mut_ptr() as usize;
        exe::club_get_national_team_name(team, variant as usize, sq, 0xc8, 2, 1);
        news_text_person_name(first, second, common, &mut name, 1, id);
        fmt(NEWS_TEXT_BUFFER, template, g, g, np, sq);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, id, first, second, common, 0x7d7);
        exe::text_add_link(out, sq as *const u8, squad_link_kind(variant), team);
    } else {
        let variant = slot(5);
        news_text_person_name(first, second, common, &mut name, 5, id);
        fmt(NEWS_TEXT_BUFFER, headline_template, g, g, np, team.addr() + club::SHORT_NAME);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, id, first, second, common, 0x7d7);
        exe::text_add_link(out, (team.addr() + club::SHORT_NAME) as *const u8, squad_link_kind(variant), team);
    }
}

/// Writes the headline or the body of news item 0x20, a player removed from a national squad: the body "removed from
/// the <squad> squad because of illness" (0xa5c80c, slot 6 = 0) or "because of injury" (0xa5c790); the headline
/// "<player> removed from <team> squad" (0xa5c73c); see `write_player_squad_news_text`.
/// Original: `NationalTeams::write_player_withdrawn_news_text` (0x76fa40, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_withdrawn_news_text(_this: usize, news: usize, body: u32, out: usize) {
    covered!("sx15_7 withdrawn text");
    write_player_squad_news_text(news, body, out, (0xa5c80c, 0xa5c790), 0xa5c73c); // sx15_7: illness, injury
}

/// Comparer for `write_player_withdrawn_news_text`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_player_withdrawn_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, body, out), |t| write_player_withdrawn_news_text(this, news, body, t),
        || format!("write_player_withdrawn_news_text(news {:#x}, body {})", news, body));
}

/// Writes the headline or the body of news item 0x21, a player called up to a national squad: the body "has been
/// called up to the <squad> squad" (0xa5c8dc); the headline "<player> brought in to <team> squad" (0xa5c888); see
/// `write_player_squad_news_text`.
/// Original: `NationalTeams::write_player_called_up_news_text` (0x76ff40, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_called_up_news_text(_this: usize, news: usize, body: u32, out: usize) {
    covered!("sx15_7 called up text");
    write_player_squad_news_text(news, body, out, (0xa5c8dc, 0xa5c8dc), 0xa5c888); // sx15_7: called up, brought in
}

/// Comparer for `write_player_called_up_news_text`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_player_called_up_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I3%%);
    compare_text_writer(out, || o(this, news, body, out), |t| write_player_called_up_news_text(this, news, body, t),
        || format!("write_player_called_up_news_text(news {:#x}, body {})", news, body));
}

/// Writes the headline or the body of news item 0x22, a squad player out injured or ill whom the human national
/// manager must replace: the body "removed ... because of illness. A replacement player must be called up instead."
/// (0xa5c9f0, slot 6 = 0) or "because of injury..." (0xa5c944); the headline "<player> removed from <team> squad"
/// (0xa5c73c); see `write_player_squad_news_text`.
/// Original: `NationalTeams::write_replacement_needed_news_text` (0x7703f0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_replacement_needed_news_text(_this: usize, news: usize, body: u32, out: usize) {
    covered!("sx15_7 replacement text");
    write_player_squad_news_text(news, body, out, (0xa5c9f0, 0xa5c944), 0xa5c73c);
}

/// Comparer for `write_replacement_needed_news_text`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_replacement_needed_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I4%%);
    compare_text_writer(out, || o(this, news, body, out), |t| write_replacement_needed_news_text(this, news, body, t),
        || format!("write_replacement_needed_news_text(news {:#x}, body {})", news, body));
}
