// ---------- sx21_3: the base transfer rules: the transfer deadline news and its text (transfer_offer.cpp) ----------

/// Posts the transfer deadline news item (0xfb2) of a window to the managers the rule set covers.
///
/// `TRANSFER_RULES::post_deadline_news(window, kind, year)`, the vtable slot `transfer_rules_vtable::POST_DEADLINE_NEWS`,
/// from the transfer manager's window news. The window's date in `year` (`exe::transfer_window_get_date`); a local news
/// item 0xfb2 (`news_item_fns`) with slot 0 the rule number (+9), 1 the kind (the deadline state: 0 passed, 1 today, 2
/// in a week, 3 lifted), 2 the window number (+1 of the window), 3 the date's day of the year, 4 its year, 5 the nation
/// id (-1 without a nation). Sent to the managers of the nation's clubs (`exe::news_send_to_nation_club_managers`),
/// else of the continent (`exe::news_send_to_continent_managers`, `transfer_rules::CONTINENT_ID`), else to all
/// managers (`exe::news_send_to_all_managers`).
/// Original: `TRANSFER_RULES::post_deadline_news` (0x8e9080, transfer_offer.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn transfer_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    covered!("sx21_3 deadline news");
    let mut date = [0u8; 8];
    exe::transfer_window_get_date(window, date.as_mut_ptr() as usize, year);
    let (build, set) = news_item_fns();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    build(n, 0xfb2, 0);
    set(n, 0, read_u8(this + 9) as i8 as u32);
    set(n, 1, kind as u8 as i8 as u32);
    set(n, 2, read_u8(window + 1) as i8 as u32);
    set(n, 3, read_i16(date.as_ptr() as usize) as u32);
    set(n, 4, read_u16(date.as_ptr() as usize + 2) as u32);
    let nation = read_usize(this + transfer_rules::NATION_ID);
    set(n, 5, if nation != 0 { read_u32(nation) } else { u32::MAX });
    send_rules_news(this, n);
}

/// Sends a rule set's news item: to the managers of its nation's clubs, else of its continent, else to all.
pub(crate) unsafe fn send_rules_news(this: usize, n: usize) {
    let nation = read_usize(this + transfer_rules::NATION_ID);
    let continent = read_usize(this + transfer_rules::CONTINENT_ID);
    if nation != 0 {
        exe::news_send_to_nation_club_managers(read_usize(G_NEWS), n, nation);
    } else if continent != 0 {
        exe::news_send_to_continent_managers(read_usize(G_NEWS), n, continent);
    } else {
        exe::news_send_to_all_managers(read_usize(G_NEWS), n);
    }
}

/// Comparer for `transfer_rules_post_deadline_news`: only the original (it posts the news).
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I0%%)(this, window, kind, year)
}

/// Writes into the shared buffer (0xde1f64) the format `fmt` with the nationality adjective (form 2, 100 bytes,
/// `exe::nation_get_nationality_adjective`) of the nation with index `nation` (`G_NATION_TABLE`).
pub(crate) unsafe fn format_with_nationality(fmt: usize, nation: u32) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut adjective = [0u8; 100];
    let record = read_usize(G_NATION_TABLE).wrapping_add((nation as usize).wrapping_mul(nation::SIZE));
    exe::nation_get_nationality_adjective(record, adjective.as_mut_ptr() as usize, 100, 2);
    text(BACKGROUND_TEXT, fmt, 100, 100, adjective.as_ptr());
}

/// Writes the transfer deadline headline, or with `story` the story naming the nation's clubs, into a news text.
///
/// `TRANSFER_RULES::write_deadline_news_text(news, story, text)`, the vtable slot
/// `transfer_rules_vtable::WRITE_DEADLINE_NEWS_TEXT`, and the base text of the countries' own versions. The deadline
/// state is the news item's slot 1 (`exe::newsitem_get_slot`). The story (`story` set): the format for the state (0
/// 0x98a020 passed, 1 0x989f20 today, 2 0x989e70 in a week, 3 0x98a148 lifted) with the nationality adjective of the
/// nation in slot 5 (`format_with_nationality`). The headline: "Transfer deadline" (0xa9e3a0), "Transfer deadline
/// today" (0xa9e388), "Transfer deadline in one week" (0xa9e368) or "Transfer deadline lifted" (0xa9e34c). Either goes
/// from the shared buffer into the text (`exe::text_set_string`); another state writes nothing. The rule set is not
/// read.
/// Original: `TRANSFER_RULES::write_deadline_news_text` (0x8e91b0, transfer_offer.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn transfer_rules_write_deadline_news_text(_this: usize, news: usize, story: u32, text: usize) {
    let state = exe::newsitem_get_slot(news, 1) as u8;
    if story != 0 {
        let nation = exe::newsitem_get_slot(news, 5);
        let fmt = match state {
            0 => 0x98a020,
            1 => 0x989f20,
            2 => 0x989e70, // sx21_3: the story a week before
            3 => 0x98a148,
            _ => return,
        };
        covered!("sx21_3 deadline story");
        format_with_nationality(fmt, nation);
    } else {
        let fmt = match state {
            0 => 0xa9e3a0,
            1 => 0xa9e388,
            2 => 0xa9e368, // sx21_3: the headline a week before
            3 => 0xa9e34c,
            _ => return,
        };
        covered!("sx21_3 deadline headline");
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        sprintf(BACKGROUND_TEXT, fmt);
    }
    exe::text_set_string(text, BACKGROUND_TEXT as *const u8);
}

/// Comparer for `transfer_rules_write_deadline_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(this, news, story, text), |t| transfer_rules_write_deadline_news_text(this, news, story, t), || format!("transfer_rules_write_deadline_news_text(rules {:#x}, story {})", this, story));
}
