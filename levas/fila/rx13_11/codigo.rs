// ---------- rx13_11: the fate news text of the Finnish First Division (fin_first.cpp) ----------

/// "{club} have qualified for the promotion playoffs of the {league}." and "{club} qualify for playoffs" (the first
/// stage); "{club} avoid relegation", "{club} relegated" and "{club} win promotion" (the play-offs).
const FIN_PLAYOFFS_TEXT: usize = 0x9c81f0;
const FIN_PLAYOFFS_SHORT_TEXT: usize = 0x9c81c8;
const FIN_AVOID_RELEGATION_TEXT: usize = 0x987594;
const FIN_RELEGATED_TEXT: usize = 0x987564;
const FIN_PROMOTION_TEXT: usize = 0x989c18;

/// The headline a Finnish First Division fate gets, with whether it carries the competition's words and link, or None:
/// a club through (6) in the first stage (signed byte below 1): the play-off headline (the long one with the
/// competition's words with `news`, else the short one; both links); without `news`, in the relegation play-off (2)
/// outcome 1 "avoid relegation" and outcomes 2 and 0xff "relegated", and in the promotion play-off (3) outcome 1 "avoid
/// relegation" for the Premier Division's own club (`a7`, the club's division [inference], equal to
/// `KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`) and "win promotion" for the First Division's (only the club's words and
/// link).
unsafe fn fin_first_headline(outcome: u32, stage: u32, a7: u32, news: u32) -> Option<(usize, bool)> {
    let (o, s) = (outcome as u8, stage as u8 as i8);
    if o == 6 && s < 1 {
        return Some(if news != 0 { (FIN_PLAYOFFS_TEXT, true) } else { (FIN_PLAYOFFS_SHORT_TEXT, false) });
    }
    if news != 0 {
        return None;
    }
    match (s, o) {
        (2, 1) => Some((FIN_AVOID_RELEGATION_TEXT, false)),
        (2, 2) | (2, 0xff) => Some((FIN_RELEGATED_TEXT, false)),
        (3, 1) if a7 == read_u32(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) => Some((FIN_AVOID_RELEGATION_TEXT, false)), // rx13_11: the Premier club stays up
        (3, 1) => Some((FIN_PROMOTION_TEXT, false)),
        _ => None,
    }
}

/// rule: the Finnish First Division news of a club's fate: a club through to the promotion play-offs ("{club} have
/// qualified for the promotion playoffs of the {league}." with the full news, else "{club} qualify for playoffs");
/// in the relegation play-off, "{club} avoid relegation" or "{club} relegated"; in the promotion play-off against the
/// Premier Division's club, "{club} avoid relegation" for that club and "{club} win promotion" for the First
/// Division's; with the full news, the play-offs go through the generic tie result and the base text
///
/// `FinFirst::build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28).
/// The headlines of `fin_first_headline` (`fate_headline`; the play-off headline with both links, the others only the
/// club's). Otherwise: stage 1, the base text with stage 1; stage 2 with `news`, the tie result
/// (`exe::comp_util_write_cup_tie_result_news`(club_id, a5, a8, outcome, a6, text)); stage 3 with `news`, the base text
/// with stage 3; anything else, nothing.
/// Original: `FinFirst::build_fate_news_text` (0x594a10, fin_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn fin_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let comp = read_usize(this + 4);
    if let Some((t, comp_words)) = fin_first_headline(outcome, stage, a7, news) {
        covered!("rx13_11 finnish first headline");
        return fate_headline(text, club_id, comp, competition::SHORT_NAME, t, comp_words, t == FIN_PLAYOFFS_TEXT || t == FIN_PLAYOFFS_SHORT_TEXT);
    }
    match stage as u8 as i8 {
        1 => fate_base_text(this, club_id, outcome, 1, a4, a5, a6, a8, news, text),
        2 if news != 0 => exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text),
        3 if news != 0 => fate_base_text(this, club_id, outcome, 3, a4, a5, a6, a8, news, text),
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_fin_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_fate_text("fin_first_build_fate_news_text", fin_first_headline(outcome, stage, a7, news).is_some(), club_id, outcome, stage, news, text,
        || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| fin_first_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}
