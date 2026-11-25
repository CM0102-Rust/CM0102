// ---------- batch %%LEVA%%: the award shortlist news text (award_manager.cpp) ----------

/// The award shortlist news' texts: the headline "<award> shortlist decided", the body "You have been invited to vote
/// in this year's <award> award.", and the vote line "You voted for <club> <player>." (or without a club).
pub(crate) const SHORTLIST_DECIDED: usize = 0x987f64;
pub(crate) const SHORTLIST_VOTE_INVITE: usize = 0x987ef4;
pub(crate) const SHORTLIST_VOTED_WITH_CLUB: usize = 0x987e94;
pub(crate) const SHORTLIST_VOTED: usize = 0x987e5c;

/// The year award's shortlist (+0x1f) [inference: read only here and by the shortlist news].
pub(crate) const YEAR_AWARD_SHORTLIST: usize = 0x1f;

/// Writes the headline or the body of an award shortlist news item, and the human's vote when he has cast one.
///
/// `AWARD_MANAGER::write_shortlist_news(news, out, body)`. Slots: 0 the nation awards, 1 the award, 2/3 the
/// announcement day and year, 4 whether the vote is already in the item, 5 the voted club (-1 none), 6 the voted person,
/// 7..9 his name ids. The award is [[`NATION_AWARDS`][slot 0] + 4][slot 1] and its competition record is at +5.
/// The headline (`body` 0): "<award> shortlist decided" with the competition's short name; the body: "You have been
/// invited to vote in this year's <award> award." with its full name; either goes into the shared buffer `JOB_TEXT` and
/// becomes the text (`Text::set_string`).
/// Slot 4 at 0: when the award has a shortlist (`YEAR_AWARD_SHORTLIST`) announced on the item's date (slots 2/3 as a
/// date against `award_shortlist_fields::DATE`) and the screen slot's manager (`G_SCREEN_RECORDS_MANAGER`) has voted in
/// it (`AWARD_SHORTLIST::get_vote_status` ≥ 0), the vote is written into the stored item for him
/// (`News::set_item_slot` with the item's id: 4 = 1, 5 the entry's club, 6..9 the person's reference by
/// `fill_staff_name_ref`: id, then the three name ids) and used below; otherwise the award's two links (short name
/// and name, link 0x7d4 to the competition) end the text. Slot 4 set: the vote comes from slots 5..9.
/// With a vote, only the body goes on: the person's name (`database_format_name_from_ids`, 300 bytes, style 1), the
/// line "You voted for <club> <player>." (the club's short name) or "You voted for <player>." appended
/// (`Text::append_string`), the person's link (0x7d7) when the names still match him (`database_staff_matches_names`)
/// and the club's link (0x7d5). The headline with a vote has no links.
/// Original: `AWARD_MANAGER::write_shortlist_news` (0x414420, award_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_shortlist_news(this: usize, news: usize, out: usize, body: u32) {
    let fmt = news_text_format();
    let s: [u32; 5] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let awards = read_usize(read_usize(this + award_manager_consts::NATION_AWARDS) + 4 * s[0] as usize);
    let award = read_usize(read_usize(awards + 4) + 4 * s[1] as usize);
    let comp = read_usize(award + 5);
    if body == 0 {
        let g = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
        fmt(JOB_TEXT, SHORTLIST_DECIDED, g, g, comp + competition::SHORT_NAME);
    } else {
        let g = read_i8(comp + competition::GENDER_NAME) as i32;
        fmt(JOB_TEXT, SHORTLIST_VOTE_INVITE, g, g, comp + competition::NAME);
    }
    exe::text_set_string(out, JOB_TEXT as *const u8);
    // the vote: (club id, person id, the three name ids)
    let vote = if s[4] != 0 {
        let v: [u32; 5] = core::array::from_fn(|k| exe::newsitem_get_slot(news, 5 + k as u32));
        Some((v[0], v[1], [v[2], v[3], v[4]]))
    } else {
        shortlist_vote_into_news(news, read_usize(award + YEAR_AWARD_SHORTLIST), s[2], s[3])
    };
    let Some((club, person, names)) = vote else {
        covered!("rxf_4 shortlist news without a vote");
        exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d4, comp);
        exe::text_add_link(out, (comp + competition::NAME) as *const u8, 0x7d4, comp);
        return;
    };
    if body == 0 {
        return;
    }
    covered!("rxf_4 shortlist news with the vote");
    let mut name = [0u8; 0x12c];
    exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x12c, 1, person);
    let c = club_at(club as i32 as isize);
    if club as i32 != -1 {
        let g = read_i8(c.addr() + club::SHORT_GENDER_NAME) as i32;
        fmt(JOB_TEXT, SHORTLIST_VOTED_WITH_CLUB, g, g, 0x64, c.addr() + club::SHORT_NAME, name.as_ptr()); // rxf_4: the vote line
    } else {
        fmt(JOB_TEXT, SHORTLIST_VOTED, name.as_ptr());
    }
    exe::text_append_string(out, JOB_TEXT);
    let p = staff_at(person as i32 as isize);
    if exe::database_staff_matches_names(p, names[0], names[1], names[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, p);
    }
    if club as i32 != -1 {
        exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c);
    }
}

/// The screen slot manager's vote in an award's shortlist, written into his stored copy of the news item: with a
/// shortlist announced on the item's day and year and a vote cast (`AWARD_SHORTLIST::get_vote_status` ≥ 0), the voted
/// entry's club and person (`fill_staff_name_ref`: name ids, then the id) go into slots 4..9 of the stored item
/// (`News::set_item_slot`: 4 = 1, 5 the club, 6 the id, 7..9 the names) and come back as (club, id, names); otherwise
/// None.
pub(crate) unsafe fn shortlist_vote_into_news(news: usize, shortlist: usize, day: u32, year: u32) -> Option<(u32, u32, [u32; 3])> {
    if shortlist == 0 {
        return None;
    }
    let mut d = [0u16; 4];
    exe::new_date(d.as_mut_ptr() as usize, day, year);
    if d[1] != read_u16(shortlist + award_shortlist_fields::DATE + 2) || d[0] != read_u16(shortlist + award_shortlist_fields::DATE) {
        return None;
    }
    let slot_manager = || read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    let k = exe::award_shortlist_get_vote_status(shortlist, slot_manager()) as i16;
    if k < 0 {
        return None;
    }
    let entry = read_usize(shortlist + award_shortlist_fields::ENTRIES) + k as usize * award_shortlist_fields::ENTRY_SIZE;
    let club = read_u32(entry + award_shortlist_fields::ENTRY_CLUB_ID);
    let mut r = [0u32; 4]; // the name reference: the three name ids, then the id
    exe::fill_staff_name_ref(staff_at(read_i32(entry) as isize), r.as_mut_ptr() as usize);
    let manager = slot_manager();
    let id = read_u32(news + news_item::ID);
    for (slot, value) in [(4, 1), (5, club), (6, r[3]), (7, r[0]), (8, r[1]), (9, r[2])] {
        exe::news_set_item_slot(read_usize(G_NEWS), manager, id as usize, slot, value, 0);
    }
    Some((club, r[3], [r[0], r[1], r[2]]))
}

/// Comparer for `write_shortlist_news`: side by side into two Texts (`compare_text_writer`). The vote written into the
/// stored item is the same both times.
pub(crate) unsafe extern "thiscall" fn compare_write_shortlist_news(this: usize, news: usize, out: usize, body: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%);
    compare_text_writer(out, || o(this, news, out, body), |t| write_shortlist_news(this, news, t, body),
        || format!("write_shortlist_news(news {:#x}, body {})", news, body));
}
