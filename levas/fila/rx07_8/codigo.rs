// ---------- rx07_8: the news that a player seeks assurances; the texts of the cancelled offer and trial (contract_manager.cpp) ----------

/// Rule: a player seeks assurances about his future: news item 0xbc1 to his contract club's manager, unless he is
/// queued for a free transfer (`exe::transfer_manager_is_queued_for_free_transfer`), has offers
/// (`exe::transfer_manager_get_offers_for_player`) or is offered in an exchange
/// (`exe::transfer_manager_get_exchange_offers_for_player`), or his main contract (`main_contract_lookup`) is of type
/// 5. Slots: 0 the club, 2 to 5 the person and his names, 6 and 7 zero, 8 and 9 the contract's transfer status, 10
/// zero, 11 the person's age. Without a contract: the "Error" box (line 0x2c36).
/// Original: `CONTRACT_MANAGER::post_seeks_assurances_news` (0x4d9840, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn post_seeks_assurances_news(_this: usize, contract: usize) {
    let contract = Contract(contract);
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2c36);
    }
    let club = club_at(contract.club() as isize);
    let p = staff_at(read_i32(contract.addr()) as isize);
    let tm = read_usize(G_TRANSFER_MANAGER);
    if exe::transfer_manager_is_queued_for_free_transfer(tm, p) != 0 || exe::transfer_manager_get_offers_for_player(tm, p, 0, 0) != 0
        || exe::transfer_manager_get_exchange_offers_for_player(tm, p, 0) != 0 {
        return;
    }
    let main = main_contract_lookup(p);
    if main != 0 && read_u8(main + contract::CONTRACT_TYPE) & 0x3f == 5 {
        return;
    }
    covered!("rx07_8 seeks assurances news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbc1, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club.addr()));
    set_news_staff_slots(n, p);
    exe::newsitem_set_slot(n, 6, 0);
    exe::newsitem_set_slot(n, 7, 0);
    let status = contract.transfer_status() as i8 as i32 as u32;
    exe::newsitem_set_slot(n, 8, status);
    exe::newsitem_set_slot(n, 9, status);
    exe::newsitem_set_slot(n, 10, 0);
    exe::newsitem_set_slot(n, 11, read_i8(p.addr() + staff::AGE) as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Comparer for `post_seeks_assurances_news`: only the original runs (the news goes to the inbox).
pub(crate) unsafe extern "thiscall" fn compare_post_seeks_assurances_news(this: usize, contract: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, contract)
}

/// The person and the club of a news item with the person in slot 0, his names in slots 1 to 3 and the club in slot 4:
/// (person id, first, second, common, club id).
pub(crate) unsafe fn news_person_club_slots(news: usize) -> (u32, u32, u32, u32, u32) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    (slot(0), slot(1), slot(2), slot(3), slot(4))
}

/// Rule: the text of news item 0xbc4 (a contract offer cancelled for lack of squad space). The body: 0x9aa484 with the
/// club's short-name gender twice, its short name and the person's name (mode 1), linked to the club (short name,
/// 0x7d5); the headline: "<name>{} contract offer cancelled" (0x9aa448) with 100 and the name (mode 5). Then the
/// person's link (0x7d7) when the names still match.
/// Original: `CONTRACT_MANAGER::write_offer_cancelled_no_space_news_text` (0x4d65c0, contract_manager.cpp, thiscall,
/// ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_offer_cancelled_no_space_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let (person, s1, s2, s3, club) = news_person_club_slots(news);
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full != 0 {
        news_text_person_name(s1, s2, s3, &mut name, 1, person);
        let c = club_at(club as i32 as isize);
        let gender = read_i8(c.addr() + club::SHORT_GENDER_NAME) as i32;
        fmt(NEWS_TEXT_BUFFER, 0x9aa484, gender, gender, c.addr() + club::SHORT_NAME, np);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c);
    } else {
        news_text_person_name(s1, s2, s3, &mut name, 5, person); // rx07_8: the headline name
        fmt(NEWS_TEXT_BUFFER, 0x9aa448, 100, np); // "<name>{} contract offer cancelled"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
    covered!("rx07_8 offer cancelled text");
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
}

/// Comparer for `write_offer_cancelled_no_space_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_offer_cancelled_no_space_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_cancelled_no_space_news_text(this, news, full, t),
        || format!("write_offer_cancelled_no_space_news_text(news {:#x}, full {})", news, full));
}

/// Rule: the text of news item 0xbbc (a trial cancelled for lack of squad space). The person's name (mode 1). The body:
/// 0x9af07c with 100, the club's gender four times, the name and the club's name twice, linked to the club (name,
/// 0x7d5); the headline: "<name>{} trial cancelled" (0x9af040) with 100 and the name. Then the person's link (0x7d7)
/// when the names still match.
/// Original: `CONTRACT_MANAGER::write_trial_cancelled_news_text` (0x4daec0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_trial_cancelled_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let (person, s1, s2, s3, club) = news_person_club_slots(news);
    let mut name = [0u8; 0x12c];
    news_text_person_name(s1, s2, s3, &mut name, 1, person);
    let np = name.as_ptr() as usize;
    if full != 0 {
        let c = club_at(club as i32 as isize);
        let gender = read_i8(c.addr() + club::GENDER_NAME) as i32;
        let club_name = c.addr() + club::NAME;
        fmt(NEWS_TEXT_BUFFER, 0x9af07c, 100, gender, gender, gender, gender, np, club_name, club_name);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, club_name as *const u8, 0x7d5, c);
    } else {
        fmt(NEWS_TEXT_BUFFER, 0x9af040, 100, np); // "<name>{} trial cancelled"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
    covered!("rx07_8 trial cancelled text");
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
}

/// Comparer for `write_trial_cancelled_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_trial_cancelled_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_trial_cancelled_news_text(this, news, full, t),
        || format!("write_trial_cancelled_news_text(news {:#x}, full {})", news, full));
}
