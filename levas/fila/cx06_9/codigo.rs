// ---------- cx06_9: the Confederations Cup's fate news text (fifa_confed.cpp) ----------

/// The final-stage template of a nation's fate in the Confederations Cup: 1 "{nation} win {cup}", 2 "finish runners-up
/// in", 3 "finish third place in", 4 "finish fourth place in", 0xff "knocked out of", any other "through to next round
/// in".
pub(crate) fn confed_final_format(outcome: u8) -> usize {
    match outcome {
        1 => 0x98732c,
        2 => 0x9872c8,
        3 => 0x987264, // finish third place in
        4 => 0x987200,
        0xff => 0x98713c,
        _ => 0x987198,
    }
}

/// Writes the news text of a nation's fate in the Confederations Cup (`group_championship_fate_text`): in the groups
/// (stage below 1) through, "{nation} qualify for semi-finals" (0x9c74b4) or the group news; out, the group news; in the
/// knockout (stage 1) the cup tie news or, without it, `confed_final_format` with the nation's and the cup's short names
/// and links. Every outcome of the knockout has a template.
/// Original: `FifaConfed::build_fate_news_text` (0x58ba10, fifa_confed.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn fifa_confed_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    covered!("cx06_9 confed fate text");
    let finals = [(outcome as u8, confed_final_format(outcome as u8))];
    group_championship_fate_text(this, club_id, outcome, stage, a4, fixture, a6, a8, news, text, 1, None, 0x9c74b4, &finals);
}

/// Comparer for `fifa_confed_build_fate_news_text`: the text-only paths side by side (`compare_text_writer`); the group
/// and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    if !group_championship_text_only(outcome, stage, news, text, 1) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| fifa_confed_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("fifa_confed_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}
