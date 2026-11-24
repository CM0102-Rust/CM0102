// --- sxB_4: the news texts of a job rumour, the FA's expectations and a job offer to the human (manager_manager.cpp) ---

/// Writes the headline or body of the rumour that a staff member is linked with or interested in a club's job (news
/// item (6, 0x1776)).
///
/// `news_job_rumour_text(news, body, out)`: slots 0..3 the person, 4 his job at his club (0 none), 5 his club, 6 the
/// club with the job, 7 the wording (a signed word). Headline (the short name, 0x190, style 5): "linked with <club>
/// job" 0xa058ac (the club's short gender and name); the person's link (0x7da) when his names match and the club's
/// long name (0x7d5). Body (the full name, style 1), with a job the job's name (`database_get_job_name`, its gender
/// written only in language 4: elsewhere the gender byte is the original's uninitialised stack byte, 0 in the copy)
/// and his club: wording 0 an approach (his own club: offered promotion 0xa05408, else approached 0xa0534c; without a
/// job: at his own club 0xa05408 with the job name of 0, else 0xa052d4), 1 considering taking over (0xa05548,
/// 0xa054b8), 2 linked with the manager's job (0xa05690, 0xa0561c), other interested in the manager's position
/// (0xa057d8, 0xa0574c). Links: the person (0x7da) when his names match, his club's long name when it is in the text
/// (`exe::crt_strstr`) and the club with the job's.
/// Original: `news_job_rumour_text` (0x69fef0, manager_manager.cpp, stdcall, ret 0xc). Defect of the original: outside
/// language 4 the job's gender byte is never written (uninitialised stack).
pub(crate) unsafe extern "stdcall" fn news_job_rumour_text(news: usize, body: u32, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let s = |k: u32| exe::newsitem_get_slot(news, k);
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    if body == 0 {
        let (id, names) = (s(0), [s(1), s(2), s(3)]);
        let target = club_at(s(6) as i32 as isize);
        exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 5, id);
        covered!("sxB_4 job rumour headline");
        let g = target.short_gender_name() as i8 as i32;
        story_lang!(buf, 0xa058ac, g, g, nm, target.addr() + club::SHORT_NAME); // "... linked with {}<club>{} job"
        exe::text_set_string(out, buf as *const u8);
        if exe::database_staff_matches_names(staff_at(id as i32 as isize), names[0], names[1], names[2], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7da, staff_at(id as i32 as isize));
        }
        exe::text_add_link(out, (target.addr() + club::NAME) as *const u8, 0x7d5, target);
        return;
    }
    let (id, names) = (s(0), [s(1), s(2), s(3)]);
    let job = s(4) as u8;
    let own = s(5) as i32;
    let target_id = s(6) as i32;
    let wording = s(7) as i16;
    exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 1, id);
    covered!("sxB_4 job rumour body");
    let (current, target) = (club_at(own as isize), club_at(target_id as isize));
    let (cg, cur) = (current.gender_name() as i8 as i32, current.addr() + club::NAME);
    let (tg, tgt) = (target.gender_name() as i8 as i32, target.addr() + club::NAME);
    let mut gender: i8 = 0;
    // the four job forms: (with a job, without one), the job forms take his club, the job, the name and the club
    let pair = match wording {
        0 => None,
        1 => Some((0xa05548, 0xa054b8)), // "... considering taking over the {}<club>{} manager's position."
        2 => Some((0xa05690, 0xa0561c)), // "... has been linked with the {}<club>{} manager's job."
        _ => Some((0xa057d8, 0xa0574c)), // "... interested in taking the {}<club>{} manager's position."
    };
    match pair {
        Some((with_job, plain)) => {
            if job != 0 {
                let title = exe::database_get_job_name(job as u32, 0, 0, &mut gender);
                let jg = gender as i32;
                story_lang!(buf, with_job, cg, cg, jg, jg, tg, tg, cur, title, nm, tgt);
            } else {
                story_lang!(buf, plain, tg, tg, nm, tgt);
            }
        }
        None => {
            if own == target_id {
                let title = exe::database_get_job_name(job as u32, 0, 0, &mut gender);
                let jg = gender as i32;
                story_lang!(buf, 0xa05408, cg, cg, jg, jg, cur, title, nm); // "... offered promotion to the position of manager."
            } else if job != 0 {
                let title = exe::database_get_job_name(job as u32, 0, 0, &mut gender);
                let jg = gender as i32;
                story_lang!(buf, 0xa0534c, cg, cg, jg, jg, tg, tg, cur, title, nm, tgt); // "... approached by {}<club>{}."
            } else {
                story_lang!(buf, 0xa052d4, 0x64, 0x64, tg, tg, nm, tgt);
            }
        }
    }
    exe::text_set_string(out, buf as *const u8);
    if exe::database_staff_matches_names(staff_at(id as i32 as isize), names[0], names[1], names[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7da, staff_at(id as i32 as isize));
    }
    if exe::crt_strstr(buf, cur) != 0 {
        exe::text_add_link(out, cur as *const u8, 0x7d5, current);
    }
    exe::text_add_link(out, tgt as *const u8, 0x7d5, target);
}

/// Comparer for `news_job_rumour_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_job_rumour_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(news, body, out), |t| news_job_rumour_text(news, body, t),
        || format!("news_job_rumour_text(body {}, wording {})", body, exe::newsitem_get_slot(news, 7) as i16));
}

/// The competition record of club competition `id` (`G_CLUB_COMP_TABLE`, 0x6b bytes each), no bounds check.
unsafe fn club_comp_at(id: i32) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b))
}

/// Writes the headline or body of the football association's expectations for its national team (news item
/// (6, 0x177e)).
///
/// `news_fa_expectations_text(news, body, out)`: slot 0 the national team, 1 the expectation (a signed word), 2 the
/// competition (−1 the friendlies). Headline: "Football Association announce expectations" (0xa0aaec). Body: the
/// nation's adjective (`exe::nation_get_nationality_adjective`, 0x1f4 bytes, form 9; only with a nation) and, by the
/// expectation: −4 qualify for the second phase (hope, 0xa0a6ec), −3 qualify from (hope, 0xa0a790), −2 qualify for
/// the competition its object names (slot 0x78 of the competition object in `G_COMP_OBJECT_TABLE`, when there is one
/// and it names one: 0xa0a820) or qualify from (expect, 0xa0a8b0), −1 win (expect, 0xa0a940), 0x14 the second round
/// (0xa0a654), 0x78 the quarter-finals (0xa0a5b8), 0x82 and 0x8c the semi-finals (0xa0a51c), 0x96 win (hope,
/// 0xa0a494); 0 and the others: without a competition the friendlies (groom the stars of the future 0xa0aa5c when
/// `exe::club_is_national_team`, else uphold the country's honour 0xa0a9c8), else perform well (0xa0a400). The
/// competition forms take its short gender and short name. Links: the adjective to the club (0x7d5) and the
/// competition's short name (0x7d0), always (competition −1 included).
/// Original: `news_fa_expectations_text` (0x6a4550, manager_manager.cpp, stdcall, ret 0xc). Defects of the original:
/// without a nation the adjective buffer is never written; the competition link reads competition −1's record.
pub(crate) unsafe extern "stdcall" fn news_fa_expectations_text(news: usize, body: u32, out: usize) {
    let mut text = [0u8; 0x1388];
    let tp = text.as_mut_ptr() as usize;
    let club = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    if body == 0 {
        covered!("sxB_4 FA expectations headline");
        story_lang!(tp, 0xa0aaec); // "Football Association announce expectations"
        exe::text_set_string(out, tp as *const u8);
        return;
    }
    let expectation = exe::newsitem_get_slot(news, 1) as i16;
    let comp_id = exe::newsitem_get_slot(news, 2) as i32;
    let mut adjective = [0u8; 0x1f4];
    let adj = adjective.as_mut_ptr() as usize;
    if club.nation().addr() != 0 {
        exe::nation_get_nationality_adjective(club.nation(), adj, 0x1f4, 9);
    }
    covered!("sxB_4 FA expectations body");
    let comp = club_comp_at(comp_id);
    let comp_text = |tpl: usize, c: usize| {
        let g = read_i8(c + competition::SHORT_GENDER_NAME) as i32;
        story_lang!(tp, tpl, 0x64, 0x64, g, g, adj, c + competition::SHORT_NAME);
    };
    match expectation {
        -4 => comp_text(0xa0a6ec, comp), // "... hope the team will qualify for the second phase of ..."
        -3 => comp_text(0xa0a790, comp), // "... hope the team will qualify from ..."
        -2 => {
            let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + comp_id as usize * 4);
            // slot 0x78 of the competition's object: the competition it qualifies for [inference]
            let target = if object != 0 {
                let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(read_usize(read_usize(object) + 0x78));
                f(object)
            } else {
                0
            };
            if target != 0 {
                comp_text(0xa0a820, target); // "... expect the team to qualify for ..."
            } else {
                comp_text(0xa0a8b0, comp); // "... expect the team to qualify from ..."
            }
        }
        -1 => comp_text(0xa0a940, comp), // "... expect the team to win ..."
        0x14 => comp_text(0xa0a654, comp), // "... the second round of ..."
        0x78 => comp_text(0xa0a5b8, comp), // "... the quarter-finals of ..."
        0x82 | 0x8c => comp_text(0xa0a51c, comp), // "... the semi-finals of ..."
        0x96 => comp_text(0xa0a494, comp), // "... hope the team will win ..."
        _ => {
            if comp_id == -1 {
                let tpl = if exe::club_is_national_team(club) != 0 { 0xa0aa5c } else { 0xa0a9c8 };
                story_lang!(tp, tpl, 0x64, 0x64, adj); // the friendlies
            } else {
                comp_text(0xa0a400, comp); // "... hope the team will perform well in ..."
            }
        }
    }
    exe::text_set_string(out, tp as *const u8);
    exe::text_add_link(out, adj as *const u8, 0x7d5, club);
    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// Comparer for `news_fa_expectations_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_fa_expectations_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(news, body, out), |t| news_fa_expectations_text(news, body, t),
        || format!("news_fa_expectations_text(body {}, expectation {})", body, exe::newsitem_get_slot(news, 1) as i16));
}

/// The body of a national team's job offer (`news_job_offer_to_human_text`) into `tp`: the FA offers the manager's job
/// (`exe::club_is_national_team` false: `fa`) or the Under 21 team's (`u21`, with the club's long gender and name),
/// with the nation's adjective, or the club's long name in its place when the club has no nation.
unsafe fn job_offer_national(tp: usize, club: Club, adj: usize, fa: usize, u21: usize) {
    let long = club.addr() + club::NAME;
    let place = if club.nation().addr() != 0 { adj } else { long };
    if exe::club_is_national_team(club) == 0 {
        story_lang!(tp, fa, 0x64, 0x64, place);
    } else {
        let g = club.gender_name() as i8 as i32;
        story_lang!(tp, u21, 0x64, 0x64, g, g, place, long);
    }
}

/// Writes the headline or body of a club's or football association's offer of its manager's job to the human
/// (news item (6, 0x1774)).
///
/// `MANAGER_MANAGER::news_job_offer_to_human_text(news, body, out)`: slot 0 the club, 1 the state (0 offered, 1
/// accepted, other declined), 2 the transfer funds, 3 the expectation flags, 4 the funds wording. Headline: "<club>
/// offer you the position of manager" (0xa0b600, the short gender and name). Body: the nation's adjective
/// (`exe::nation_get_nationality_adjective`, 100 bytes, form 9, of the club's nation even when it is none). A national
/// team (`exe::database_is_national_team`): the FA's offer (`job_offer_national`: offered 0xa0b570 / 0xa0b4a8,
/// accepted 0xa0af8c / 0xa0aea0, declined 0xa0ad80 / 0xa0ac94). A club, offered: the board's expectations
/// (`build_board_expectation_text` through the exe, 0x1388 bytes) and the funds: above `reputation² × 0.05 + 500000`
/// the amount (`exe::cash_get_display_amount` and `exe::cash_format_money`, unit 4) with "a transfer fund of"
/// (0xa0b374, wording set) or "willing to make" (0xa0b228); above 50000 "a small amount" (0xa0b104); else none
/// (0xa0b040); accepted 0xa0ae34; declined 0xa0ac28. The club's long name links to the club (0x7d5) before the text
/// is set, in both parts.
/// Original: `MANAGER_MANAGER::news_job_offer_to_human_text` (0x6a5120, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_job_offer_to_human_text(this: usize, news: usize, body: u32, out: usize) {
    let mut text = [0u8; 0x1388];
    let tp = text.as_mut_ptr() as usize;
    let club = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    let long = club.addr() + club::NAME;
    if body == 0 {
        covered!("sxB_4 job offer headline");
        let g = club.short_gender_name() as i8 as i32;
        story_lang!(tp, 0xa0b600, g, g, club.addr() + club::SHORT_NAME); // "{}<club>{} offer you the position of manager"
        exe::text_add_link(out, long as *const u8, 0x7d5, club);
        exe::text_set_string(out, tp as *const u8);
        return;
    }
    let state = exe::newsitem_get_slot(news, 1) as u8;
    let funds = exe::newsitem_get_slot(news, 2) as i32;
    let flags = exe::newsitem_get_slot(news, 3);
    let wording = exe::newsitem_get_slot(news, 4);
    let mut adjective = [0u8; 0x64];
    let adj = adjective.as_mut_ptr() as usize;
    exe::nation_get_nationality_adjective(club.nation(), adj, 0x64, 9);
    covered!("sxB_4 job offer body");
    let lg = club.gender_name() as i8 as i32;
    if exe::database_is_national_team(club) != 0 {
        let (fa, u21) = match state {
            0 => (0xa0b570, 0xa0b4a8),
            1 => (0xa0af8c, 0xa0aea0),
            _ => (0xa0ad80, 0xa0ac94),
        };
        job_offer_national(tp, club, adj, fa, u21);
    } else {
        match state {
            0 => {
                let mut expectations = [0u8; 0x1388];
                let ep = expectations.as_mut_ptr() as usize;
                let build: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(board_text_fns::BUILD_BOARD_EXPECTATION_TEXT);
                build(this, club.addr(), flags, ep, 0x1388);
                let rep = club.reputation() as i16 as i32;
                let threshold = (rep * rep) as f64 * read_f64(rdata::F64_0_05) + 500000.0;
                if threshold < funds as f64 {
                    let value = funds as u32;
                    let amount = exe::cash_get_display_amount(&value, 4);
                    let mut money = [0u8; 0x3e8];
                    exe::cash_format_money(money.as_mut_ptr() as usize, 0x3e8, amount, 4);
                    story_lang!(tp, if wording != 0 { 0xa0b374 } else { 0xa0b228 }, lg, lg, long, ep, money.as_ptr());
                } else {
                    story_lang!(tp, if funds > 0xc350 { 0xa0b104 } else { 0xa0b040 }, lg, lg, long, ep);
                }
            }
            1 => story_lang!(tp, 0xa0ae34, lg, lg, long), // "... You have accepted this job offer."
            _ => story_lang!(tp, 0xa0ac28, lg, lg, long), // "... You have declined this job offer."
        }
    }
    exe::text_add_link(out, long as *const u8, 0x7d5, club);
    exe::text_set_string(out, tp as *const u8);
}

/// Comparer for `news_job_offer_to_human_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_job_offer_to_human_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, body, out), |t| news_job_offer_to_human_text(this, news, body, t),
        || format!("news_job_offer_to_human_text(body {}, state {})", body, exe::newsitem_get_slot(news, 1) as u8));
}
