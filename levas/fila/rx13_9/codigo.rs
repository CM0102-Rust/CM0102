// ---------- rx13_9: the fate news texts of the Gaucho and Rio state leagues and of the MLS (bra_reg_gaucho.cpp, bra_reg_rio.cpp, usa_mls.cpp) ----------

/// "{club} have qualified for the {league} semi finals.", "{club} qualify for semi finals" and "{club} have won the
/// {league} championship." (the Gaucho and Rio state leagues).
const STATE_SEMI_FINAL_TEXT: usize = 0x98ad98;
const STATE_SEMI_FINAL_SHORT_TEXT: usize = 0x98ad5c;
const STATE_CHAMPIONSHIP_TEXT: usize = 0x98acec;
/// "{club} qualify for quarter finals", "{club} qualify for {league} final" and "{club} win {league}" (the MLS).
const MLS_QUARTER_FINAL_TEXT: usize = 0xaa2064;
const MLS_FINAL_TEXT: usize = 0xaa2020;
const MLS_WIN_TEXT: usize = 0xaa1fec;

/// The fate news text of the Gaucho and Rio state leagues (the same body, read instruction by instruction; only the
/// final stage differs: 0 for the Gaucho, 2 for Rio). The competition goes by its full name (`competition::NAME`) with
/// its short name's gender.
///
/// Stage (byte) 0xff: outcome (byte) 6, the semi final headline (`fate_headline`: the long one with the competition's
/// words when `news`, the short one otherwise; both links); another outcome, the base text with stage −1. The final
/// stage: with `news` the base text with that stage; without, outcome 1 the championship headline (with the
/// competition's words, only the club's link), another outcome the base text with that stage and the 9th argument 0.
/// Other stages: nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn state_league_fate_news(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a8: u32, news: u32, text: usize,
    final_stage: u8) {
    let comp = read_usize(this + 4);
    let s = stage as u8;
    if s == 0xff {
        if outcome as u8 != 6 {
            return fate_base_text(this, club_id, outcome, -1, a4, a5, a6, a8, news, text);
        }
        covered!("rx13_9 state league semi final headline");
        let long = news != 0;
        let t = if long { STATE_SEMI_FINAL_TEXT } else { STATE_SEMI_FINAL_SHORT_TEXT };
        return fate_headline(text, club_id, comp, competition::NAME, t, long, true);
    }
    if s != final_stage {
        return;
    }
    if news != 0 {
        return fate_base_text(this, club_id, outcome, final_stage as i32, a4, a5, a6, a8, news, text);
    }
    if outcome as u8 == 1 {
        covered!("rx13_9 state league champion headline");
        return fate_headline(text, club_id, comp, competition::NAME, STATE_CHAMPIONSHIP_TEXT, true, false); // rx13_9: only the club's link
    }
    fate_base_text(this, club_id, outcome, final_stage as i32, a4, a5, a6, a8, 0, text)
}

/// Whether a state league fate takes only the text (the headlines), for the comparer.
fn state_league_text_only(stage: u32, outcome: u32, news: u32, final_stage: u8) -> bool {
    let s = stage as u8;
    (s == 0xff && outcome as u8 == 6) || (s == final_stage && news == 0 && outcome as u8 == 1)
}

/// rule: the Gaucho state league news of a club's fate: over the season a club through to the semi finals ("{club}
/// have qualified for the {league} semi finals." with the full news, or the short headline); in the final (stage 0)
/// the champion without the full news, "{club} have won the {league} championship."; every other fate, the base
/// competition text (`state_league_fate_news` with the final stage 0).
/// Original: `BraRegGaucho::build_fate_news_text` (0x432770, bra_reg_gaucho.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    state_league_fate_news(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 0) // rx13_9: the Gaucho final is stage 0
}

/// rule: the Rio state league news of a club's fate, as the Gaucho's with the final at stage 2
/// (`state_league_fate_news` with the final stage 2).
/// Original: `BraRegRio::build_fate_news_text` (0x43ab20, bra_reg_rio.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    state_league_fate_news(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 2) // rx13_9: the Rio final is stage 2
}

/// The MLS headline a fate gets without a news object, or None: before the play-offs (stage below 2, signed) a club
/// through (6) to the quarter finals; in the semi finals (2) the final reached and in the final (3) the title (outcome
/// 1).
fn mls_headline(stage: u32, outcome: u32) -> Option<usize> {
    match (stage as u8 as i8, outcome as u8) {
        (s, 6) if s < 2 => Some(MLS_QUARTER_FINAL_TEXT),
        (2, 1) => Some(MLS_FINAL_TEXT),
        (3, 1) => Some(MLS_WIN_TEXT),
        _ => None,
    }
}

/// rule: the MLS news of a club's fate: before the play-offs a club through to the quarter finals (the generic group
/// sentence with the full news, else "{club} qualify for quarter finals"); in the semi finals and the final, the tie
/// result with the full news, else "{club} qualify for {league} final" and "{club} win {league}" for the winner and the
/// generic win or defeat sentence for the others
///
/// `UsaMls::build_fate_news_text(..)` (10 dwords, ret 0x28). Stage (signed byte) below 2 with outcome (byte) 6: with
/// `news`, `exe::comp_util_write_group_qualify_news`(club_id, a4, the competition, 6, news, text); without, the short
/// headline with the club's link whose target is the club's name, not the club (a defect of the original:
/// `mls_quarter_final_headline`). Stages 2 and 3: with `news`, `exe::comp_util_write_cup_tie_result_news`(club_id, a5,
/// a8, outcome, a6, text); without, outcome 1 the headline (`fate_headline` with the competition's words and both
/// links), else `exe::comp_util_write_cup_win_or_defeat_news`(club_id, a5, outcome, text). Other cases: nothing.
/// Original: `UsaMls::build_fate_news_text` (0x908360, usa_mls.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn usa_mls_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let comp = read_usize(this + 4);
    let s = stage as u8 as i8;
    if s < 2 && outcome as u8 == 6 {
        if news != 0 {
            return exe::comp_util_write_group_qualify_news(club_id, a4, comp, 6, news, text);
        }
        covered!("rx13_9 mls quarter final headline");
        return mls_quarter_final_headline(text, club_id);
    }
    if s != 2 && s != 3 {
        return;
    }
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    match mls_headline(stage, outcome) {
        Some(t) => {
            covered!("rx13_9 mls final headline");
            fate_headline(text, club_id, comp, competition::SHORT_NAME, t, true, true)
        }
        None => exe::comp_util_write_cup_win_or_defeat_news(club_id, a5, outcome, text),
    }
}

/// The MLS quarter final headline (`MLS_QUARTER_FINAL_TEXT` with the club's gender twice and its short name) set as the
/// text, with the club's link (0x7d5) whose target is the club's short name address, as the original pushes it.
unsafe fn mls_quarter_final_headline(text: usize, club_id: u32) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    story_lang!(PRAISE_TEXT_BUFFER, MLS_QUARTER_FINAL_TEXT, g, g, name);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, name as *const u8, 0x7d5, name); // rx13_9: the name as the link's target
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_gaucho_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_fate_text("bra_reg_gaucho_build_fate_news_text", state_league_text_only(stage, outcome, news, 0), club_id, outcome, stage, news, text,
        || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| bra_reg_gaucho_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_rio_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    compare_fate_text("bra_reg_rio_build_fate_news_text", state_league_text_only(stage, outcome, news, 2), club_id, outcome, stage, news, text,
        || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| bra_reg_rio_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I2%%);
    let text_only = news == 0 && mls_headline(stage, outcome).is_some();
    compare_fate_text("usa_mls_build_fate_news_text", text_only, club_id, outcome, stage, news, text,
        || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| usa_mls_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}
