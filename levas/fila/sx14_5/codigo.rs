// ---------- batch %%LEVA%%: the news texts of a job offer and of the board's reaction to a release (manager_manager.cpp) ----------

/// The person's name from news slots `first`..`first + 2` (`exe::database_format_name_from_ids`, 400 bytes, `mode`).
unsafe fn manager_news_person_name(ids: [u32; 3], id: u32, mode: u32, buf: &mut [u8; 400]) {
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], buf.as_mut_ptr(), 0x190, mode, id);
}

/// Links the person's name (0x7da) when the news item's names still match him (`exe::database_staff_matches_names`).
unsafe fn manager_news_link_person(out: usize, name: &[u8; 400], id: u32, ids: [u32; 3]) {
    let p = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(p, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7da, p);
    }
}

/// Writes the headline or body of the news that a club offered a staff member or a veteran player its manager's
/// position (news 0x1775, posted by `post_job_offer_news`).
///
/// `news_job_offer_text(news, body, out)`: slot 0 the person, 1..3 his name ids. Headline, with slot 5 (the club) and
/// the name in mode 5: "<club> offer <name> the position of manager" (0xa05ac4, the club's `.gender_name()` with its
/// short name); then the person's link (0x7da) and the club's (0x7d5, on its name). Body, with slots 4 (his club), 5
/// (the club offering) and 6 (his job) and the name in mode 1: a job 0xb, 0xf or 0xd (a player) 0xa05900, else
/// 0xa059e8, with the offering club's and his club's genders and names; then the person's link and the two clubs'.
/// Defect of the original: the headline pairs the club's full-name gender with its short name.
/// Original: `news_job_offer_text` (0x6a0690, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_job_offer_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut name = [0u8; 400];
    if body == 0 {
        let (id, ids) = (slot(0), [slot(1), slot(2), slot(3)]);
        let club = club_at(slot(5) as i32 as isize);
        manager_news_person_name(ids, id, 5, &mut name);
        let g = club.gender_name() as i32;
        story_lang!(NEWS_TEXT_BUFFER, 0xa05ac4, g, g, club.addr() + club::SHORT_NAME, name.as_ptr()); // "<club> offer <name> the position of manager"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        manager_news_link_person(out, &name, id, ids);
        exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
        return;
    }
    covered!("sx14_5 job offer body");
    let (id, ids) = (slot(0), [slot(1), slot(2), slot(3)]);
    let (from, to, job) = (club_at(slot(4) as i32 as isize), club_at(slot(5) as i32 as isize), slot(6));
    manager_news_person_name(ids, id, 1, &mut name);
    let template = if job == 0xb || job == 0xf || job == 0xd { 0xa05900 } else { 0xa059e8 };
    let (gf, gt) = (from.gender_name() as i32, to.gender_name() as i32);
    story_lang!(NEWS_TEXT_BUFFER, template, gt, gt, gf, gf, to.name_addr(), from.name_addr(), name.as_ptr());
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    manager_news_link_person(out, &name, id, ids);
    exe::text_add_link(out, from.name_addr() as *const u8, 0x7d5, from);
    exe::text_add_link(out, to.name_addr() as *const u8, 0x7d5, to);
}

/// Writes the headline or body of the board's reaction to a released player or a blocked free transfer (news 0x1778).
///
/// `news_board_release_reaction_text(news, body, out)`: slots 0 the player, 1..3 his name ids, 4 the club, 5 the
/// reaction flags. Headline (name in mode 5, 100 as the gender): 0x800 "Board block <name> departure" (0xa06020), 0x400
/// "Board shocked by release of <name>" (0xa05fe0), 1 "Board upset by release of <name>" (0xa05fa4), else "Board
/// surprised at <name> departure" (0xa05f64); the player's link (0x7da). Body (name in mode 1): 0x800 with 0x200
/// 0xa05eac, else 0xa05e00 (100, the club's gender and name, the player's name); 0x400 0xa05d18; 1 0xa05c84; 0x200
/// 0xa05bb4; else 0xa05b28 (the club's gender and name, the player's name); the player's link and the club's (0x7d5).
/// Original: `news_board_release_reaction_text` (0x6a0ad0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_release_reaction_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut name = [0u8; 400];
    let (id, ids) = (slot(0), [slot(1), slot(2), slot(3)]);
    if body == 0 {
        covered!("sx14_5 release reaction headline");
        let flags = slot(5);
        manager_news_person_name(ids, id, 5, &mut name);
        let template = if flags & 0x800 != 0 {
            0xa06020 // "Board block <name> departure"
        } else if flags & 0x400 != 0 {
            0xa05fe0 // "Board shocked by release of <name>"
        } else if flags & 1 != 0 {
            0xa05fa4 // "Board upset by release of <name>"
        } else {
            0xa05f64 // "Board surprised at <name> departure"
        };
        story_lang!(NEWS_TEXT_BUFFER, template, 100, name.as_ptr());
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        manager_news_link_person(out, &name, id, ids);
        return;
    }
    covered!("sx14_5 release reaction body");
    let (club, flags) = (club_at(slot(4) as i32 as isize), slot(5));
    manager_news_person_name(ids, id, 1, &mut name);
    let g = club.gender_name() as i32;
    let n = name.as_ptr();
    if flags & 0x800 != 0 {
        let template = if flags & 0x200 != 0 { 0xa05eac } else { 0xa05e00 };
        story_lang!(NEWS_TEXT_BUFFER, template, 100, g, g, club.name_addr(), n);
    } else {
        let template = if flags & 0x400 != 0 {
            0xa05d18
        } else if flags & 1 != 0 {
            0xa05c84
        } else if flags & 0x200 != 0 {
            0xa05bb4
        } else {
            0xa05b28
        };
        story_lang!(NEWS_TEXT_BUFFER, template, g, g, club.name_addr(), n);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    manager_news_link_person(out, &name, id, ids);
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
}

/// Comparer for `news_job_offer_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_job_offer_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, body, out), |t| news_job_offer_text(this, news, body, t),
                        || format!("news_job_offer_text(news {:#x}, body {})", news, body));
}

/// Comparer for `news_board_release_reaction_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_release_reaction_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_release_reaction_text(this, news, body, t),
                        || format!("news_board_release_reaction_text(news {:#x}, body {})", news, body));
}
