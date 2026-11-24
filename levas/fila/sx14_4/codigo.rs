// ---------- batch %%LEVA%%: the news texts of the board memos and the FA replies (manager_manager.cpp) ----------

/// The nation with id `id` (`G_NATION_TABLE`, `nation::SIZE` bytes each).
unsafe fn manager_news_nation(id: u32) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE))
}

/// The nationality adjective of `nation` into `buf` (`exe::nation_get_nationality_adjective`, form 9).
unsafe fn manager_news_adjective(nation: usize, buf: &mut [u8], size: u32) {
    exe::nation_get_nationality_adjective(nation, buf.as_mut_ptr() as usize, size, 9);
}

/// Writes the headline or body of a club's reply to the human manager's job application (news 0x1773).
///
/// `news_job_application_reply_text(news, body, out)`: slot 0 the club, slot 1 dismissed (else acknowledged). Headline
/// "<club> dismiss / acknowledge your job application" (0xa04f48 / 0xa04f04), body 0xa04e70 / 0xa04df4, both with the
/// club's `.gender_name()` and name, written into a local buffer; the link to the club's name (0x7d5) goes in before
/// the text (`exe::text_set_string`).
/// Original: `news_job_application_reply_text` (0x69c6b0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_job_application_reply_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut text = [0u8; 5000];
    let club = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    let dismissed = exe::newsitem_get_slot(news, 1) as u8 != 0;
    let template = match (body != 0, dismissed) {
        (false, true) => 0xa04f48,  // "<club> dismiss your job application"
        (false, false) => 0xa04f04, // "<club> acknowledge your job application"
        (true, true) => 0xa04e70,
        (true, false) => 0xa04df4,
    };
    covered!("sx14_4 application reply");
    let g = club.gender_name() as i32;
    story_lang!(text.as_mut_ptr(), template, g, g, club.name_addr());
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
    exe::text_set_string(out, text.as_ptr());
}

/// Writes the headline or body of the board's memo about the manager's interest in another club's job (news 0x1390).
///
/// `news_board_interest_memo_text(news, body, out)`: slot 1 the other club; the body by (slot 0 + slot 2) mod 3
/// (0xa04fa4, 0xa050b0, else 0xa051a0, with the club's `.short_gender_name()` and short name); the headline "Memo from
/// board received" (0xa04f88); the link to the club's short name (0x7d5).
/// Original: `news_board_interest_memo_text` (0x69c910, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_interest_memo_text(_this: usize, news: usize, body: u32, out: usize) {
    let (a, club, b) = (exe::newsitem_get_slot(news, 0) as i32, exe::newsitem_get_slot(news, 1), exe::newsitem_get_slot(news, 2) as i32);
    let club = club_at(club as i32 as isize);
    if body != 0 {
        covered!("sx14_4 memo body");
        let template = match b.wrapping_add(a) % 3 {
            0 => 0xa04fa4,
            1 => 0xa050b0,
            _ => 0xa051a0,
        };
        let g = club.short_gender_name() as i8 as i32;
        story_lang!(NEWS_TEXT_BUFFER, template, g, g, club.addr() + club::SHORT_NAME);
    } else {
        story_lang!(NEWS_TEXT_BUFFER, 0xa04f88); // "Memo from board received"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
}

/// Writes the headline or body of the FA's verdict on the national team manager in a competition (news 0x1781).
///
/// `news_fa_performance_assessment_text(news, body, out)`: slot 0 the national team, 1 the competition, 2 the grade.
/// The nationality of the team's `.nation()` ("Sedgefordian" without one); the headline "<nationality> FA performance
/// assessment" (0xa0ba48, 100 as the genders); the body by the grade (3 0xa0b9a0, 2 0xa0b8f8, 0 0xa0b850, −1 0xa0b7a0,
/// −2 0xa0b6f8, else 0xa0b648) with the competition's gender and name; the links to the national team (0x7d5, on the
/// nationality) and to the competition's name (0x7d0).
/// Original: `news_fa_performance_assessment_text` (0x6a5b40, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_fa_performance_assessment_text(_this: usize, news: usize, body: u32, out: usize) {
    let team = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((exe::newsitem_get_slot(news, 1) as usize).wrapping_mul(0x6b));
    let grade = exe::newsitem_get_slot(news, 2) as u8;
    let mut adjective = [0u8; 100];
    if team.nation().is_null() {
        story_crt!(adjective.as_mut_ptr(), 0x991944, 0xa09ba0); // "%s", "Sedgefordian"
    } else {
        manager_news_adjective(team.nation().addr(), &mut adjective, 100);
    }
    let mut text = [0u8; 500];
    if body == 0 {
        story_lang!(text.as_mut_ptr(), 0xa0ba48, 100, 100, adjective.as_ptr()); // "<nationality> FA performance assessment"
    } else {
        covered!("sx14_4 FA assessment body");
        let template = match grade {
            3 => 0xa0b9a0,
            2 => 0xa0b8f8,
            0 => 0xa0b850,
            0xff => 0xa0b7a0,
            0xfe => 0xa0b6f8,
            _ => 0xa0b648,
        };
        let g = read_i8(comp + competition::GENDER_NAME) as i32;
        story_lang!(text.as_mut_ptr(), template, 100, 100, g, g, adjective.as_ptr(), comp + competition::NAME);
    }
    exe::text_set_string(out, text.as_ptr());
    exe::text_add_link(out, adjective.as_ptr(), 0x7d5, team);
    exe::text_add_link(out, (comp + competition::NAME) as *const u8, 0x7d0, comp);
}

/// Writes the headline or body of the news that supporters protested against the manager (news 0x1782).
///
/// `news_supporter_protest_text(news, body, out)`: the headline "Supporter Protest" (0xa0bb3c); the body from slot 0
/// (the club) and slots 1..4 (the manager's id and name ids, `exe::database_format_name_from_ids`, 400 bytes, mode 1):
/// 0xa0ba88 with the club's `.gender_name()`, 100, its name and the manager's name; the links to the club (0x7d5) and
/// to the manager (0x7db) when the names still match him (`exe::database_staff_matches_names`).
/// Original: `news_supporter_protest_text` (0x6a5e30, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_supporter_protest_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut text = [0u8; 500];
    if body == 0 {
        story_lang!(text.as_mut_ptr(), 0xa0bb3c); // "Supporter Protest"
        exe::text_set_string(out, text.as_ptr());
        return;
    }
    covered!("sx14_4 protest body");
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, id, first, second, common) = (slot(0), slot(1), slot(2), slot(3), slot(4));
    let mut name = [0u8; 400];
    exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0x190, 1, id);
    let club = club_at(club as i32 as isize);
    let g = club.gender_name() as i32;
    story_lang!(text.as_mut_ptr(), 0xa0ba88, g, g, 100, club.name_addr(), name.as_ptr());
    exe::text_set_string(out, text.as_ptr());
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
    let manager = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(manager, first, second, common, 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7db, manager);
    }
}

/// Writes the headline or body of the news that the FA rescheduled a match after an appeal (news 0x29).
///
/// `news_match_rescheduled_text(news, body, out)`: the headline "Match rescheduled after appeal" (0xa11660, after
/// reading slots 0..2); the body from slots 0 and 1 (the two clubs) and 4 (the nation, a byte): 0xa11574 with 100 twice,
/// the clubs' `.gender_name()`, the nationality (120 bytes) and the two names; the links to the first club, to the
/// nation's national team (`exe::database_get_national_team`, on the nationality) and to the second club (0x7d5).
/// Original: `news_match_rescheduled_text` (0x6aa910, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_match_rescheduled_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    if body == 0 {
        slot(0);
        slot(1);
        slot(2);
        story_lang!(NEWS_TEXT_BUFFER, 0xa11660); // "Match rescheduled after appeal"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        return;
    }
    covered!("sx14_4 rescheduled body");
    let (home, away) = (slot(0), slot(1));
    slot(2);
    let nation = manager_news_nation(slot(4) as i8 as i32 as u32);
    slot(5);
    let mut adjective = [0u8; 0x78];
    manager_news_adjective(nation, &mut adjective, 0x78);
    let (home, away) = (club_at(home as i32 as isize), club_at(away as i32 as isize));
    let (gh, ga) = (home.gender_name() as i32, away.gender_name() as i32);
    story_lang!(NEWS_TEXT_BUFFER, 0xa11574, 100, 100, gh, gh, ga, ga, adjective.as_ptr(), home.name_addr(), away.name_addr());
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, home.name_addr() as *const u8, 0x7d5, home);
    exe::text_add_link(out, adjective.as_ptr(), 0x7d5, exe::database_get_national_team(nation));
    exe::text_add_link(out, away.name_addr() as *const u8, 0x7d5, away);
}

/// Writes the headline or body of the FA's reply to a referee complaint (request 1) or a postponement request
/// (request 2) (news 0x1789).
///
/// `news_fa_request_reply_text(news, body, out)`: slots 0 the club, 1 the request, 2 the outcome, 3 the nation, 4 the
/// referee's standing. The headline "FA reaction to request for match postponement" (request 2, 0xa11680) or "... to
/// complaint about referee" (0xa116b0). The body with the nationality (120 bytes, 100 as the genders), by the outcome:
/// 0xc 0xa11ea4, 4 0xa11dcc, 0xa 0xa11cb0; otherwise a referee complaint: outcome 0 0xa116d8, else standing below 5
/// 0xa11998, below 6 0xa11888, else 0xa117b0; a postponement: outcome 0 0xa11a64, 6 0xa11b18, else 0xa11bc4; any other
/// request "The <nationality> Football Association indicate that they will gladly give you sod all" (0xa11c64, C
/// sprintf). The link to the club (0x7d5).
/// Original: `news_fa_request_reply_text` (0x6aab70, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_fa_request_reply_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let club = club_at(slot(0) as i32 as isize);
    let request = slot(1) as i8;
    if body == 0 {
        let template = if request == 2 {
            0xa11680 // "FA reaction to request for match postponement"
        } else {
            0xa116b0 // "FA reaction to complaint about referee"
        };
        story_lang!(NEWS_TEXT_BUFFER, template);
    } else {
        covered!("sx14_4 FA reply body");
        let outcome = slot(2) as u8;
        let nation = manager_news_nation(slot(3));
        let standing = slot(4) as i32;
        let mut adjective = [0u8; 0x78];
        manager_news_adjective(nation, &mut adjective, 0x78);
        let template = match (outcome, request) {
            (0xc, _) => Some(0xa11ea4),
            (4, _) => Some(0xa11dcc),
            (0xa, _) => Some(0xa11cb0),
            (0, 1) => Some(0xa116d8),
            (_, 1) if standing < 5 => Some(0xa11998),
            (_, 1) if standing < 6 => Some(0xa11888),
            (_, 1) => Some(0xa117b0),
            (0, 2) => Some(0xa11a64),
            (6, 2) => Some(0xa11b18),
            (_, 2) => Some(0xa11bc4),
            _ => None,
        };
        match template {
            Some(t) => story_lang!(NEWS_TEXT_BUFFER, t, 100, 100, adjective.as_ptr()),
            None => story_crt!(NEWS_TEXT_BUFFER, 0xa11c64, adjective.as_ptr()), // "... will gladly give you sod all"
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
}

/// Writes the headline or body of the FA's reaction to a complaint about a referee (news 0x2b).
///
/// `news_fa_referee_complaint_text(news, body, out)`: the headline "FA react to ref complaint" (0xa11fbc); the body
/// from slot 0 (the nation), 1 and 2 (the second names, `G_SECOND_NAME_TABLE`, the referee had and has) and 3 (the
/// complaining manager): with the nationality (120 bytes), 0xa12148 (100 twice) when the manager is the one of the
/// current screen (`G_SCREEN_RECORDS_MANAGER`), else 0xa11fd8 (100 three times), with the two names. No links.
/// Original: `news_fa_referee_complaint_text` (0x6aadb0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_fa_referee_complaint_text(_this: usize, news: usize, body: u32, out: usize) {
    if body == 0 {
        story_lang!(NEWS_TEXT_BUFFER, 0xa11fbc); // "FA react to ref complaint"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        return;
    }
    covered!("sx14_4 referee complaint body");
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (nation, old, new, manager) = (slot(0), slot(1), slot(2), slot(3));
    let name = |id: u32| read_usize(G_SECOND_NAME_TABLE).wrapping_add((id as usize).wrapping_mul(0x3c));
    let (old, new) = (name(old), name(new));
    let manager = staff_at(manager as i32 as isize);
    let mut adjective = [0u8; 0x78];
    manager_news_adjective(manager_news_nation(nation), &mut adjective, 0x78);
    let current = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    if manager.addr() == current {
        story_lang!(NEWS_TEXT_BUFFER, 0xa12148, 100, 100, adjective.as_ptr(), old, new);
    } else {
        story_lang!(NEWS_TEXT_BUFFER, 0xa11fd8, 100, 100, 100, adjective.as_ptr(), old, new);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
}

/// Writes the headline or body of the FA's decision on a player's ban appeal (news 0x178a).
///
/// `news_ban_appeal_result_text(news, body, out)`: slots 0..3 the player (id and name ids; the name by
/// `exe::database_format_staff_name`, 500 bytes, mode 1), 4 upheld, 5 the nation, 6 and 7 the games taken off the two
/// bans (−1 none). Headline: upheld with no games given "FA revoke <name>'s ban" (0xa127c0), upheld "FA reduce ..."
/// (0xa12790), else "FA dismiss ban appeal" (0xa12778). Body, with the nation's gender, 100, the nationality (120
/// bytes) and the name: upheld with the first count 1 0xa12c84, another count 0xa12b88 (with it); else with the second
/// count 1 0xa12ab4, another 0xa12994 (with −1, not the count); neither 0xa128c0; dismissed 0xa127f0. The link to the
/// player (0x7da) when the names still match him.
///
/// Defect of the original: the reduced second ban quotes −1 (the first count) instead of its own count.
/// Original: `news_ban_appeal_result_text` (0x6ac190, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_ban_appeal_result_text(_this: usize, news: usize, body: u32, out: usize) {
    let s: [u32; 8] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    exe::newsitem_get_slot(news, 8);
    let [id, first, second, common, upheld, nation, first_ban, second_ban] = s;
    let (upheld, first_ban, second_ban) = (upheld as u8 != 0, first_ban as i32, second_ban as i32);
    let nation = manager_news_nation(nation);
    let player = staff_at(id as i32 as isize);
    let mut name = [0u8; 500];
    exe::database_format_staff_name(player, name.as_mut_ptr() as usize, 0x1f4, 1);
    let n = name.as_ptr();
    if body != 0 {
        covered!("sx14_4 appeal body");
        let mut adjective = [0u8; 0x78];
        manager_news_adjective(nation, &mut adjective, 0x78);
        let a = adjective.as_ptr();
        let g = read_i8(nation + nation::GENDER_NAME) as i32;
        match (upheld, first_ban, second_ban) {
            (false, _, _) => story_lang!(NEWS_TEXT_BUFFER, 0xa127f0, g, g, 100, a, n),
            (true, 1, _) => story_lang!(NEWS_TEXT_BUFFER, 0xa12c84, g, g, 100, a, n),
            (true, -1, -1) => story_lang!(NEWS_TEXT_BUFFER, 0xa128c0, g, g, 100, a, n),
            (true, -1, 1) => story_lang!(NEWS_TEXT_BUFFER, 0xa12ab4, g, g, 100, a, n),
            (true, -1, _) => story_lang!(NEWS_TEXT_BUFFER, 0xa12994, g, g, 100, a, n, -1i32),
            (true, games, _) => story_lang!(NEWS_TEXT_BUFFER, 0xa12b88, g, g, 100, a, n, games),
        }
    } else if !upheld {
        story_lang!(NEWS_TEXT_BUFFER, 0xa12778); // "FA dismiss ban appeal"
    } else if first_ban == -1 && second_ban == -1 {
        story_lang!(NEWS_TEXT_BUFFER, 0xa127c0, 100, n); // "FA revoke <name>'s ban"
    } else {
        story_lang!(NEWS_TEXT_BUFFER, 0xa12790, 100, n); // "FA reduce <name>'s ban"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if exe::database_staff_matches_names(player, first, second, common, 0usize) != 0 {
        exe::text_add_link(out, n, 0x7da, player);
    }
}

/// The comparer of the board and FA news texts: the text side by side (`compare_text_writer`).
unsafe fn compare_manager_news_text(index: usize, name: &str, rust: unsafe extern "thiscall" fn(usize, usize, u32, usize), this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(index);
    compare_text_writer(out, || o(this, news, body, out), |t| rust(this, news, body, t), || format!("{}(news {:#x}, body {})", name, news, body));
}

/// Comparer for `news_job_application_reply_text`.
pub(crate) unsafe extern "thiscall" fn compare_news_job_application_reply_text(this: usize, news: usize, body: u32, out: usize) {
    compare_manager_news_text(%%I0%%, "news_job_application_reply_text", news_job_application_reply_text, this, news, body, out);
}

/// Comparer for `news_board_interest_memo_text`.
pub(crate) unsafe extern "thiscall" fn compare_news_board_interest_memo_text(this: usize, news: usize, body: u32, out: usize) {
    compare_manager_news_text(%%I1%%, "news_board_interest_memo_text", news_board_interest_memo_text, this, news, body, out);
}

/// Comparer for `news_fa_performance_assessment_text`.
pub(crate) unsafe extern "thiscall" fn compare_news_fa_performance_assessment_text(this: usize, news: usize, body: u32, out: usize) {
    compare_manager_news_text(%%I2%%, "news_fa_performance_assessment_text", news_fa_performance_assessment_text, this, news, body, out);
}

/// Comparer for `news_supporter_protest_text`.
pub(crate) unsafe extern "thiscall" fn compare_news_supporter_protest_text(this: usize, news: usize, body: u32, out: usize) {
    compare_manager_news_text(%%I3%%, "news_supporter_protest_text", news_supporter_protest_text, this, news, body, out);
}

/// Comparer for `news_match_rescheduled_text`.
pub(crate) unsafe extern "thiscall" fn compare_news_match_rescheduled_text(this: usize, news: usize, body: u32, out: usize) {
    compare_manager_news_text(%%I4%%, "news_match_rescheduled_text", news_match_rescheduled_text, this, news, body, out);
}

/// Comparer for `news_fa_request_reply_text`.
pub(crate) unsafe extern "thiscall" fn compare_news_fa_request_reply_text(this: usize, news: usize, body: u32, out: usize) {
    compare_manager_news_text(%%I5%%, "news_fa_request_reply_text", news_fa_request_reply_text, this, news, body, out);
}

/// Comparer for `news_fa_referee_complaint_text`.
pub(crate) unsafe extern "thiscall" fn compare_news_fa_referee_complaint_text(this: usize, news: usize, body: u32, out: usize) {
    compare_manager_news_text(%%I6%%, "news_fa_referee_complaint_text", news_fa_referee_complaint_text, this, news, body, out);
}

/// Comparer for `news_ban_appeal_result_text`.
pub(crate) unsafe extern "thiscall" fn compare_news_ban_appeal_result_text(this: usize, news: usize, body: u32, out: usize) {
    compare_manager_news_text(%%I7%%, "news_ban_appeal_result_text", news_ban_appeal_result_text, this, news, body, out);
}
