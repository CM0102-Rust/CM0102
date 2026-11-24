// ---------- cx11_10: the fate news text of the Serie C1 and C2 groups (ita_ser_c2a.cpp) ----------

/// "{club} win promotion" (langlib template with the club's short name and gender).
const WIN_PROMOTION_TEXT: usize = 0x989c18;
/// "{club} relegated".
const RELEGATED_TEXT: usize = 0x987564;

/// Writes the news text of a club's fate in a Serie C1 or C2 group (the five groups share it at virtual +0x48).
///
/// With a news body (`news`), the base text (`exe::comp_build_fate_news_text`) with a7 = 0. Without one: outcome 1 in
/// stage 0 (the promotion play-off won), "{club} win promotion"; outcome 0xff in stage 1 (the relegation play-out lost),
/// "{club} relegated"; both with the club's short name and gender byte (`group_qualify_text`: the shared buffer, the
/// Text and the club's link 0x7d5). Any other outcome or stage: the base text with a7 = 0 and no news. The outcome and
/// the stage are compared as bytes; the base text gets them as the caller passed them.
/// Original: `ItaSerC2a::build_fate_news_text` (0x654390, ita_ser_c2a.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn ita_ser_c_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    if news != 0 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, news, text);
    }
    covered!("cx11_10 serie c own fate text");
    match (outcome as u8, stage as u8) {
        (1, 0) => group_qualify_text(club_id, WIN_PROMOTION_TEXT, text), // cx11_10: the play-off winner goes up
        (0xff, 1) => group_qualify_text(club_id, RELEGATED_TEXT, text),
        _ => exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, 0, text),
    }
}

/// Comparer for `ita_ser_c_build_fate_news_text`: the two text-only paths side by side (`compare_text_writer`; with the
/// CM_TEXTO_NOTICIAS mode); the base text only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    let own = matches!((outcome as u8, stage as u8), (1, 0) | (0xff, 1));
    if news != 0 || text == 0 || !own {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| ita_ser_c_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("ita_ser_c_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}
