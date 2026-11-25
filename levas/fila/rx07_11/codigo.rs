// ---------- rx07_11: the text of the news that a player wants a new contract (contract_manager.cpp) ----------

/// The body templates of the wants-new-contract news by the squad status (slot 6: 1, 2, 3, any other), when he asks
/// himself (slot 11 clear; the name only) and when his agent asks (slot 11 set; 100 and the name).
pub(crate) const NEW_CONTRACT_TEXTS: [[usize; 4]; 2] = [
    [0x9ae690, 0x9ae740, 0x9ae7f4, 0x9ae8a8],
    [0x9ae934, 0x9aea00, 0x9aead0, 0x9aeba0],
];

/// Whether the inbox item of news `news` (the same item, the contract terms in slots 8 to 10) must be marked as no
/// longer current: the person's main contract no longer has the wage (slot 10) and end date (slots 8 day, 9 year) the
/// news named, or it does and the club (slot 0) has an offer for him (`contract_manager_fields::CONTRACT_OFFERS`,
/// walked with the list cursor). With no offer list, nothing.
pub(crate) unsafe fn new_contract_news_outdated(this: usize, news: usize, person: u32, club: u32) -> bool {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut end = [0u32; 2];
    let (year, day) = (slot(9), slot(8));
    exe::new_date(end.as_mut_ptr() as usize, day, year);
    let wage = slot(10);
    let s = read_usize(G_STAFF_TABLE).wrapping_add((person as usize).wrapping_mul(staff::SIZE));
    let id = if s == 0 { -1 } else { read_i32(s) };
    let mut c = 0usize;
    if s != 0 && id < read_i32(G_STAFF_COUNT) {
        let i = read_i32(contract_manager_staff_state_record(id as usize));
        if i >= 0 {
            c = (i as usize).wrapping_mul(80).wrapping_add(read_usize(read_usize(G_CONTRACT_MANAGER)));
        }
    }
    let end_day = read_u16(end.as_ptr() as usize);
    let end_year = read_u16(end.as_ptr() as usize + 2);
    if c == 0 || read_u32(c + contract::WAGE) != wage || read_u16(c + contract::CONTRACT_EXPIRES_YEAR) != end_year
        || read_u16(c + contract::CONTRACT_EXPIRES) != end_day {
        return true;
    }
    let list = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if list == 0 {
        return false;
    }
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut e = exe::llist_get_next(list);
    while e != 0 {
        if read_i32(e) == id && read_u32(e + contract_offer::CLUB_ID) == club {
            return true;
        }
        e = exe::llist_get_next(list);
    }
    false
}

/// Rule: the text of news item 0xbc2 (a player or his agent asks for a better contract): slots 0 the club, 2 the
/// person, 3 to 5 his names, 6 the squad status, 7 whether the item is already outdated, 8 to 10 the contract end and
/// wage it named, 11 whether his agent asks. The body: unless slot 7 is set, an outdated item
/// (`new_contract_news_outdated`) gets slot 7 set to 1 in the human manager's inbox (`exe::news_set_item_slot` with
/// the item's id); the name (mode 1) in the template by the status and who asks (`NEW_CONTRACT_TEXTS`), the club
/// linked (name). The headline (name mode 5): "<name> wants new contract" (0x9ae660). Then the person's link (0x7d7)
/// when the names still match.
/// Original: `CONTRACT_MANAGER::write_wants_new_contract_news_text` (0x4da190, contract_manager.cpp, thiscall, ret
/// 0xc).
pub(crate) unsafe extern "thiscall" fn write_wants_new_contract_news_text(this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, person, s3, s4, s5, status) = (slot(0), slot(2), slot(3), slot(4), slot(5), slot(6));
    let outdated = slot(7) as u8;
    let agent = slot(11) as u8;
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full != 0 {
        if outdated == 0 && new_contract_news_outdated(this, news, person, club) {
            covered!("rx07_11 outdated news marked");
            let manager = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
            exe::news_set_item_slot(read_usize(G_NEWS), manager, read_usize(news + 0xd6), 7, 1, news as u32); // +0xd6: the item's id
        }
        news_text_person_name(s3, s4, s5, &mut name, 1, person);
        let k = match status { 1 => 0, 2 => 1, 3 => 2, _ => 3 };
        if agent != 0 {
            fmt(NEWS_TEXT_BUFFER, NEW_CONTRACT_TEXTS[1][k], 100, np);
        } else {
            fmt(NEWS_TEXT_BUFFER, NEW_CONTRACT_TEXTS[0][k], np);
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        let c = club_at(club as i32 as isize);
        exe::text_add_link(out, (c.addr() + club::NAME) as *const u8, 0x7d5, c);
    } else {
        news_text_person_name(s3, s4, s5, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0x9ae660, np); // "<name> wants new contract"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
    covered!("rx07_11 new contract text");
    news_text_link_person(out, &name, person, s3, s4, s5, 0x7d7);
}

/// Comparer for `write_wants_new_contract_news_text`: the text side by side (`compare_text_writer`); the original runs
/// first, so its mark on the inbox item (slot 7, the same value) makes the copy skip it.
pub(crate) unsafe extern "thiscall" fn compare_write_wants_new_contract_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_wants_new_contract_news_text(this, news, full, t),
        || format!("write_wants_new_contract_news_text(news {:#x}, full {})", news, full));
}
