// ---------- cx04_4: the Intertoto Cup's fate news text (intertoto_cup.cpp) ----------

/// Writes the news text of a club's fate in the Intertoto Cup. Outcome 1 (it goes through): with `news`, the cup tie
/// result news (`exe::comp_util_write_cup_tie_result_news`(club_id, fixture, a8, 1, a6, text)); without, the text
/// 0x9e9e54 ("{club} qualify for {competition}") with the UEFA Cup's short name and its link
/// (`european_cup_fate_text`, the UEFA Cup's record by `KEY_CLUB_COMP_UEFA_CUP`). Any other outcome: the base
/// competition's text (`exe::comp_build_fate_news_text` with the same arguments and 0 for a7).
/// Original: `IntertotoCup::build_fate_news_text` (0x633af0, intertoto_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn intertoto_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    if outcome as u8 != 1 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, news, text);
    }
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, 1, a6, text);
    }
    covered!("cx04_4 intertoto qualify text");
    let uefa = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_UEFA_CUP).wrapping_mul(NEWS_COMP_SIZE));
    european_cup_fate_text(text, club_id, 0x9e9e54, read_u8(uefa + 0x52), uefa + 0x38, uefa);
}

/// Comparer for `intertoto_cup_build_fate_news_text`: the path that only writes the text (outcome 1 without `news`)
/// side by side with `compare_text_writer`; the others (the base text and the cup tie news) only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    if outcome as u8 != 1 || news != 0 || text == 0 {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| intertoto_cup_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("intertoto_cup_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}
