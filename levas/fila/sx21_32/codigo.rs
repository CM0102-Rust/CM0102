// ---------- sx21_32: the transfer deadline news text of France (france_rules.cpp) ----------

/// Writes the French transfer deadline story: the general deadlines of the first window and the restrictions of the
/// third, with a link to the national team; the headline is the base one.
///
/// `FRANCE_RULES::write_deadline_news_text(news, story, text)`, the vtable slot
/// `transfer_rules_vtable::WRITE_DEADLINE_NEWS_TEXT` of France. The news item's slots 1, 2 and 5 are read first
/// (`exe::newsitem_get_slot`). The headline (`story` 0): the base text (`transfer_rules_write_deadline_news_text`). The
/// story (`write_window_deadline_story` with the link, adjective form 2): window 0, state 3 0x98a148, 0 0x9ccee0, 1
/// 0x9cce30, 2 0x9ccd80; window 2, state 3 0x9ccf60 (all restrictions lifted), 0 0x9cce90, 1 0x9ccde0, 2 0x9ccd2c.
/// Another window: the text set from the shared buffer unchanged and the link on an adjective never written. A state
/// above 3: nothing.
/// Defect of the original: in another window the link text is an uninitialised stack buffer (the copy uses zeros). The
/// French deadline news skip window 1 (`france_rules_post_deadline_news`), so it does not happen in play.
/// Original: `FRANCE_RULES::write_deadline_news_text` (0x5c1b90, france_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn france_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    if story == 0 {
        exe::newsitem_get_slot(news, 1);
        exe::newsitem_get_slot(news, 2);
        exe::newsitem_get_slot(news, 5);
        transfer_rules_write_deadline_news_text(this, news, 0, text);
        return;
    }
    let written = write_window_deadline_story(news, text, true, |state, window| match (state, window) {
        (3, 0) => Some(Some((0x98a148, 2, false))),
        (3, 2) => Some(Some((0x9ccf60, 2, false))),
        (0, 0) => Some(Some((0x9ccee0, 2, false))),
        (0, 2) => Some(Some((0x9cce90, 2, false))),
        (1, 0) => Some(Some((0x9cce30, 2, false))), // sx21_32: restrictions from tomorrow
        (1, 2) => Some(Some((0x9ccde0, 2, false))),
        (2, 0) => Some(Some((0x9ccd80, 2, false))),
        (2, 2) => Some(Some((0x9ccd2c, 2, false))),
        (0..=3, _) => Some(None),
        _ => None,
    });
    if written {
        covered!("sx21_32 france deadline story");
    }
}

/// Comparer for `france_rules_write_deadline_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_france_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, news, story, text), |t| france_rules_write_deadline_news_text(this, news, story, t), || format!("france_rules_write_deadline_news_text(story {})", story));
}
