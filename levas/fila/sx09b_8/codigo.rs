// --- sx09b_8: the text of the board's warning about fining players too often (Fine.cpp) ---

/// Writes the headline or the body of news item 6020 (0x1784): the board's warning about a manager who fines his
/// players too often.
///
/// `write_board_fine_warning_news_text(news, full, out)`: the headline "The board make concerns known" (0x9cba54); the
/// body by the news item's slot 0 (`exe::newsitem_get_slot`): set, the first warning ("The board are concerned that
/// your frequent fining of players will damage the squads morale...", 0x9cbb3c), clear, the last one ("...we may be
/// forced to look for a new manager", 0x9cba74). Formatted into `NEWS_TEXT_BUFFER` and set as the text of `out`
/// (`exe::text_set_string`).
/// Original: `write_board_fine_warning_news_text` (0x5a7ed0, Fine.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_board_fine_warning_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let template = if full == 0 {
        0x9cba54 // "The board make concerns known"
    } else if exe::newsitem_get_slot(news, 0) != 0 { // sx09b_8: the first warning
        covered!("sx09b_8 board fine warning, the first");
        0x9cbb3c
    } else {
        covered!("sx09b_8 board fine warning, the last");
        0x9cba74
    };
    story_lang!(NEWS_TEXT_BUFFER, template);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
}

/// Comparer for `write_board_fine_warning_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_board_fine_warning_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let what = || format!("write_board_fine_warning_news_text(news {:#x}, full {})", news, full);
    compare_text_writer(out, || o(this, news, full, out), |t| write_board_fine_warning_news_text(this, news, full, t), what);
}
