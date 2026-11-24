// ---------- sx21_31: the transfer deadline news texts of Argentina, Germany and Belgium (argentina_rules.cpp, germany_rules.cpp, belgium_rules.cpp) ----------

/// Writes the Argentine deadline story when the second window lifts the restrictions, else the base text.
///
/// `ARGENTINA_RULES::write_deadline_news_text(news, story, text)`, the vtable slot
/// `transfer_rules_vtable::WRITE_DEADLINE_NEWS_TEXT` of Argentina. The news item's slots 1 (the deadline state), 2 (the
/// window number) and 5 (the nation) (`exe::newsitem_get_slot`). The story (`story` set) of state 3 (lifted) in window
/// 1: the format 0x98799c (Argentine clubs may buy again, two signings at most before February) with the nationality
/// adjective (`format_with_nationality`) into the text (`exe::text_set_string`). Otherwise the base text
/// (`transfer_rules_write_deadline_news_text`).
/// Original: `ARGENTINA_RULES::write_deadline_news_text` (0x40aa30, argentina_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn argentina_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let state = exe::newsitem_get_slot(news, 1) as u8;
    let window = exe::newsitem_get_slot(news, 2) as u8;
    let nation = exe::newsitem_get_slot(news, 5);
    if state == 3 && window == 1 && story != 0 { // sx21_31: the second window opens
        covered!("sx21_31 argentina second window story");
        format_with_nationality(0x98799c, nation);
        exe::text_set_string(text, BACKGROUND_TEXT as *const u8);
        return;
    }
    transfer_rules_write_deadline_news_text(this, news, story, text);
}

/// Comparer for `argentina_rules_write_deadline_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_argentina_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, news, story, text), |t| argentina_rules_write_deadline_news_text(this, news, story, t), || format!("argentina_rules_write_deadline_news_text(story {})", story));
}

/// What a German or Belgian deadline story writes for a (state, window): `None` leaves the text alone (a state above
/// 3); `Some(None)` sets the text from the shared buffer as it is (a window without a format of its own: whatever the
/// buffer last held); `Some(Some((fmt, form, twice)))` formats it first, with the nationality adjective in form `form`
/// (once, or `twice`).
type DeadlineStory = Option<Option<(usize, u32, bool)>>;

/// Writes a German or Belgian deadline story: `pick` gives the format of the news item's state (slot 1) and window
/// number (slot 2); the adjective is the one of the nation in slot 5 (`exe::nation_get_nationality_adjective`, 100
/// bytes); through the shared buffer (0xde1f64) into the text (`exe::text_set_string`); with `link`, a link (type
/// 0x7d5) on the adjective to the nation's national team (`exe::database_get_national_team`, `exe::text_add_link`).
pub(crate) unsafe fn write_window_deadline_story(news: usize, text: usize, link: bool, pick: impl Fn(u8, u8) -> DeadlineStory) -> bool {
    let state = exe::newsitem_get_slot(news, 1) as u8;
    let window = exe::newsitem_get_slot(news, 2) as u8;
    let nation = exe::newsitem_get_slot(news, 5);
    let Some(story) = pick(state, window) else {
        return false;
    };
    let mut adjective = [0u8; 100];
    let record = read_usize(G_NATION_TABLE).wrapping_add((nation as usize).wrapping_mul(nation::SIZE));
    if let Some((fmt, form, twice)) = story {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        exe::nation_get_nationality_adjective(record, adjective.as_mut_ptr() as usize, 100, form);
        if twice {
            sprintf(BACKGROUND_TEXT, fmt, 100, 100, 100, 100, adjective.as_ptr(), adjective.as_ptr());
        } else {
            sprintf(BACKGROUND_TEXT, fmt, 100, 100, adjective.as_ptr());
        }
    }
    exe::text_set_string(text, BACKGROUND_TEXT as *const u8);
    if link {
        let team = exe::database_get_national_team(record);
        exe::text_add_link(text, adjective.as_ptr(), 0x7d5, team);
    }
    true
}

/// Writes the German transfer deadline story: the lower league and inter-club deadlines of the two windows; the
/// headline is the base one.
///
/// `GERMANY_RULES::write_deadline_news_text(news, story, text)`, the vtable slot
/// `transfer_rules_vtable::WRITE_DEADLINE_NEWS_TEXT` of Germany. The news item's slots 1, 2 and 5 are read first
/// (`exe::newsitem_get_slot`). The headline (`story` 0): the base text (`transfer_rules_write_deadline_news_text`). The
/// story (`write_window_deadline_story`): state 3, 0x98a148 (clubs may buy again). State 0: window 0, 0x98a0c4 with the
/// adjective twice; window 1, 0x98a070 (no more transfers between the nation's clubs). State 1: window 0, 0x989fc8
/// (adjective form 0xe, last day for lower league transfers); window 1, 0x989f70. State 2: window 0, 0x989ec4 (form
/// 0xe); window 1, 0x989e70. The forms are 2 where not said. Another window: the text is set from the shared buffer
/// unchanged. A state above 3: nothing.
/// Defect of the original: a window other than 0 and 1 sets the text from whatever the shared buffer last held.
/// Original: `GERMANY_RULES::write_deadline_news_text` (0x5e0420, germany_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn germany_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    if story == 0 {
        exe::newsitem_get_slot(news, 1);
        exe::newsitem_get_slot(news, 2);
        exe::newsitem_get_slot(news, 5);
        transfer_rules_write_deadline_news_text(this, news, 0, text);
        return;
    }
    let written = write_window_deadline_story(news, text, false, |state, window| match (state, window) {
        (3, _) => Some(Some((0x98a148, 2, false))),
        (0, 0) => Some(Some((0x98a0c4, 2, true))),
        (0, 1) => Some(Some((0x98a070, 2, false))),
        (1, 0) => Some(Some((0x989fc8, 0xe, false))), // sx21_31: the German lower league deadline today
        (1, 1) => Some(Some((0x989f70, 2, false))),
        (2, 0) => Some(Some((0x989ec4, 0xe, false))),
        (2, 1) => Some(Some((0x989e70, 2, false))),
        (0..=2, _) => Some(None),
        _ => None,
    });
    if written {
        covered!("sx21_31 germany deadline story");
    }
}

/// Comparer for `germany_rules_write_deadline_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_germany_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(this, news, story, text), |t| germany_rules_write_deadline_news_text(this, news, story, t), || format!("germany_rules_write_deadline_news_text(story {})", story));
}

/// Writes the Belgian transfer deadline story: the lower league, inter-club and general deadlines of the three windows;
/// the headline is the base one.
///
/// `BELGIUM_RULES::write_deadline_news_text(news, story, text)`, the vtable slot
/// `transfer_rules_vtable::WRITE_DEADLINE_NEWS_TEXT` of Belgium. As Germany's (`write_window_deadline_story`), with the
/// third window: state 0, window 2, 0x98a020 (the deadline has passed); state 1, window 2, 0x989f20 (today); state 2,
/// windows 1 and 2, 0x989e70 (next week). Another window: the text set from the shared buffer unchanged; a state above
/// 3: nothing.
/// Defect of the original: as Germany's, a window without a format sets the text from the stale shared buffer.
/// Original: `BELGIUM_RULES::write_deadline_news_text` (0x4250b0, belgium_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn belgium_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    if story == 0 {
        exe::newsitem_get_slot(news, 1);
        exe::newsitem_get_slot(news, 2);
        exe::newsitem_get_slot(news, 5);
        transfer_rules_write_deadline_news_text(this, news, 0, text);
        return;
    }
    let written = write_window_deadline_story(news, text, false, |state, window| match (state, window) {
        (3, _) => Some(Some((0x98a148, 2, false))),
        (0, 0) => Some(Some((0x98a0c4, 2, true))),
        (0, 1) => Some(Some((0x98a070, 2, false))),
        (0, 2) => Some(Some((0x98a020, 2, false))),
        (1, 0) => Some(Some((0x989fc8, 0xe, false))),
        (1, 1) => Some(Some((0x989f70, 2, false))),
        (1, 2) => Some(Some((0x989f20, 2, false))), // sx21_31: the Belgian deadline today
        (2, 0) => Some(Some((0x989ec4, 0xe, false))),
        (2, 1) | (2, 2) => Some(Some((0x989e70, 2, false))),
        (0..=2, _) => Some(None),
        _ => None,
    });
    if written {
        covered!("sx21_31 belgium deadline story");
    }
}

/// Comparer for `belgium_rules_write_deadline_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_belgium_rules_write_deadline_news_text(this: usize, news: usize, story: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(text, || o(this, news, story, text), |t| belgium_rules_write_deadline_news_text(this, news, story, t), || format!("belgium_rules_write_deadline_news_text(story {})", story));
}
