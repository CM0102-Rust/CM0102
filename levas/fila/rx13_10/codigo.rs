// ---------- rx13_10: the fate news text of the English Third Division (eng_third.cpp) ----------

/// "{club} win promotion" (the play-off winner).
const ENG_THIRD_PROMOTION_TEXT: usize = 0x989c18;

/// Whether an English Third Division fate gets the promotion headline: outcome (byte) 1 in the play-off (stage byte 0)
/// without a news object.
fn eng_third_promotion_headline(outcome: u32, stage: u32, news: u32) -> bool {
    outcome as u8 == 1 && stage as u8 == 0 && news == 0
}

/// rule: the English Third Division news of a club's fate: the play-off winner, without the full news, gets "{club} win
/// promotion"; every other fate, the base competition text
///
/// `EngThird::build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28):
/// `eng_third_promotion_headline`, the headline (`fate_headline` with only the club's words and link); otherwise the
/// base text (`fate_base_text`) with the stage as passed.
/// Original: `EngThird::build_fate_news_text` (0x5785b0, eng_third.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn eng_third_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    if eng_third_promotion_headline(outcome, stage, news) {
        covered!("rx13_10 third division promotion headline");
        return fate_headline(text, club_id, read_usize(this + 4), competition::SHORT_NAME, ENG_THIRD_PROMOTION_TEXT, false, false);
    }
    fate_base_text(this, club_id, outcome, stage as i32, a4, a5, a6, a8, news, text) // rx13_10: the base text
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_eng_third_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_fate_text("eng_third_build_fate_news_text", eng_third_promotion_headline(outcome, stage, news), club_id, outcome, stage, news, text,
        || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| eng_third_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}
