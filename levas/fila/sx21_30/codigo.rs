// ---------- sx21_30: the transfer deadline news texts of Italy and Norway (italy_rules.cpp, norway_rules.cpp) ----------

/// Writes a country's deadline story into a news text: the format `fmt` with the nationality adjective (form `form`,
/// 100 bytes, `exe::nation_get_nationality_adjective`) of the nation with index `nation` (`G_NATION_TABLE`) through the
/// shared buffer (0xde1f64) into the text (`exe::text_set_string`), then a link (type 0x7d5) on the adjective to that
/// nation's national team (`exe::database_get_national_team`, `exe::text_add_link`).
pub(crate) unsafe fn write_country_deadline_story(text: usize, fmt: usize, nation: u32, form: u32) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut adjective = [0u8; 100];
    let record = read_usize(G_NATION_TABLE).wrapping_add((nation as usize).wrapping_mul(nation::SIZE));
    exe::nation_get_nationality_adjective(record, adjective.as_mut_ptr() as usize, 100, form);
    sprintf(BACKGROUND_TEXT, fmt, 100, 100, adjective.as_ptr());
    exe::text_set_string(text, BACKGROUND_TEXT as *const u8);
    let team = exe::database_get_national_team(record);
    exe::text_add_link(text, adjective.as_ptr(), 0x7d5, team);
}

/// Writes the Italian transfer deadline story, with the free transfer wording for the third window; the headline is the
/// base one.
///
/// `ITALY_RULES::write_deadline_news_text(news, story, text)`, the vtable slot
/// `transfer_rules_vtable::WRITE_DEADLINE_NEWS_TEXT` of Italy. The news item's slots 1 (the deadline state), 2 (the
/// window number) and 5 (the nation) (`exe::newsitem_get_slot`). The headline (`story` 0): the base text
/// (`transfer_rules_write_deadline_news_text`). The story (`write_country_deadline_story`, adjective form 2): state 3,
/// clubs may buy again (0x98a148); state 0, the deadline has passed (0x9cce90, or the free transfer deadline 0x9eab38
/// from window 2); state 1, today (0x9eaae8, or 0x9eaa94); state 2, next week (0x9ccd2c, or 0x9eaa3c); another state
/// writes nothing.
/// Original: `ITALY_RULES::write_deadline_news_text` (0x661b40, italy_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn italy_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let state = exe::newsitem_get_slot(news, 1) as u8;
    let window = exe::newsitem_get_slot(news, 2) as u8 as i8;
    let nation = exe::newsitem_get_slot(news, 5);
    if story == 0 {
        transfer_rules_write_deadline_news_text(this, news, 0, text);
        return;
    }
    let free = window >= 2;
    let fmt = match state {
        3 => 0x98a148,
        0 => if free { 0x9eab38 } else { 0x9cce90 },
        1 => if free { 0x9eaa94 } else { 0x9eaae8 }, // sx21_30: the Italian deadline today
        2 => if free { 0x9eaa3c } else { 0x9ccd2c },
        _ => return,
    };
    covered!("sx21_30 italy deadline story");
    write_country_deadline_story(text, fmt, nation, 2);
}

/// Comparer for `italy_rules_write_deadline_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_italy_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, news, story, text), |t| italy_rules_write_deadline_news_text(this, news, story, t), || format!("italy_rules_write_deadline_news_text(story {})", story));
}

/// Writes the Norwegian transfer deadline story for the passed, today and next week states; the headline and the other
/// states are the base text.
///
/// `NORWAY_RULES::write_deadline_news_text(news, story, text)`, the vtable slot
/// `transfer_rules_vtable::WRITE_DEADLINE_NEWS_TEXT` of Norway. The news item's slots 1 (the deadline state) and 5 (the
/// nation) (`exe::newsitem_get_slot`). The headline (`story` 0): the base text (`transfer_rules_write_deadline_news_text`).
/// The story (`write_country_deadline_story`): state 0, 0xa69974 (adjective form 2); state 1, 0xa698f8 (form 2); state
/// 2, 0xa698a8 (form 0xe); another state, the base text.
/// Original: `NORWAY_RULES::write_deadline_news_text` (0x7948e0, norway_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn norway_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let state = exe::newsitem_get_slot(news, 1) as u8;
    let nation = exe::newsitem_get_slot(news, 5);
    if story == 0 {
        transfer_rules_write_deadline_news_text(this, news, 0, text);
        return;
    }
    let (fmt, form) = match state {
        0 => (0xa69974, 2),
        1 => (0xa698f8, 2),
        2 => (0xa698a8, 0xe), // sx21_30: the Norwegian deadline next week
        _ => {
            transfer_rules_write_deadline_news_text(this, news, story, text);
            return;
        }
    };
    covered!("sx21_30 norway deadline story");
    write_country_deadline_story(text, fmt, nation, form);
}

/// Comparer for `norway_rules_write_deadline_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_norway_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(this, news, story, text), |t| norway_rules_write_deadline_news_text(this, news, story, t), || format!("norway_rules_write_deadline_news_text(story {})", story));
}
