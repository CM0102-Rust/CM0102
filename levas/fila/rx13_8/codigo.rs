// ---------- rx13_8: the fate news texts of the three Brazilian national divisions (bra_nat_first.cpp, bra_nat_second.cpp, bra_nat_third.cpp) ----------

/// "{club} have qualified for the quarter final of the {league}." and "{club} qualify for quarter final".
const BRA_FIRST_QUARTER_FINAL_TEXT: usize = 0x98ab08;
const BRA_FIRST_QUARTER_FINAL_SHORT_TEXT: usize = 0x98aad8;
/// "{club} win {competition}" and "{club} lose in {competition} final" (the first and second divisions).
const BRA_WIN_TEXT: usize = 0x98aaa8;
const BRA_LOSE_FINAL_TEXT: usize = 0x98aa70;
/// "{club} have qualified for the second round of the {league}." and "{club} qualify for second round".
const BRA_THIRD_SECOND_ROUND_TEXT: usize = 0x98ac50;
const BRA_THIRD_SECOND_ROUND_SHORT_TEXT: usize = 0x98ac24;
/// "{club} win {competition}" (the third division's champion).
const BRA_THIRD_WIN_TEXT: usize = 0x98abe4;

/// The base fate text of a competition (`exe::comp_build_fate_news_text`) with the routine's own arguments, the stage
/// it passes, the 7th argument 0 and the 8th and 9th as given (some paths pass 0).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn fate_base_text(this: usize, club_id: u32, outcome: u32, stage: i32, a4: u32, a5: u32, a6: u32, a8: u32, news: u32, text: usize) {
    exe::comp_build_fate_news_text(this, club_id, outcome, stage, a4, a5 as usize, a6, 0, a8, news, text)
}

/// Writes a fate headline into `text`: `template` (`langlib_sprintf` into the shared buffer 0xde1f64) with the club's
/// gender (`club::SHORT_GENDER_NAME`, signed) twice, then, with `comp_words`, the competition's gender
/// (`competition::SHORT_GENDER_NAME`) twice, the club's `club::SHORT_NAME` and, with `comp_words`, the competition's name
/// at `comp_name` (its short name or its full name); without `comp_words` only the club's gender twice and its name.
/// The text set (`exe::text_set_string`), the club's link (0x7d5) and, with `comp_link`, the competition's link (0x7d0,
/// the same name).
pub(crate) unsafe fn fate_headline(text: usize, club_id: u32, comp: usize, comp_name: usize, template: usize, comp_words: bool, comp_link: bool) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    let name_of_comp = comp + comp_name;
    if comp_words {
        let cg = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        story_lang!(PRAISE_TEXT_BUFFER, template, g, g, cg, cg, name, name_of_comp);
    } else {
        story_lang!(PRAISE_TEXT_BUFFER, template, g, g, name);
    }
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, name as *const u8, 0x7d5, club);
    if comp_link {
        exe::text_add_link(text, name_of_comp as *const u8, 0x7d0, comp);
    }
}

/// The headline a Brazilian first or second division fate gets at stage 0 (the final) without a news object: the
/// title (1) or the final lost (2); None for the base text.
fn bra_final_headline(outcome: u8) -> Option<usize> {
    match outcome {
        1 => Some(BRA_WIN_TEXT),
        2 => Some(BRA_LOSE_FINAL_TEXT), // rx13_8: the final lost
        _ => None,
    }
}

/// The final (stage 0) of the Brazilian first and second divisions: with a news object the base text (stage 0); without
/// one, the title or the final lost (`bra_final_headline`, with the competition's words and both links), and any other
/// outcome the base text with the 8th and 9th arguments 0.
#[allow(clippy::too_many_arguments)]
unsafe fn bra_final_fate(this: usize, club_id: u32, outcome: u32, a4: u32, a5: u32, a6: u32, a8: u32, news: u32, text: usize) {
    if news != 0 {
        return fate_base_text(this, club_id, outcome, 0, a4, a5, a6, a8, news, text);
    }
    match bra_final_headline(outcome as u8) {
        Some(t) => {
            covered!("rx13_8 brazilian final headline");
            fate_headline(text, club_id, read_usize(this + 4), competition::SHORT_NAME, t, true, true)
        }
        None => fate_base_text(this, club_id, outcome, 0, a4, a5, a6, 0, 0, text),
    }
}

/// rule: the Brazilian first division news of a club's fate: over the season (stage 0xff) a club through to the
/// quarter finals gets "{club} have qualified for the quarter final of the {league}." (with a news object) or "{club}
/// qualify for quarter final"; in the final (stage 0) the winner "{club} win {league}" and the loser "{club} lose in
/// {league} final" (without a news object); every other fate, the base competition text
///
/// `BraNatFirst::build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28).
/// Stage (byte) 0xff: outcome (byte) 6, the headline (`fate_headline`: the long one with the competition's words when
/// `news`, the short one otherwise; both links in both); another outcome, the base text with stage −1. Stage 0:
/// `bra_final_fate`. Other stages: nothing.
/// Original: `BraNatFirst::build_fate_news_text` (0x42ac70, bra_nat_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn bra_nat_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    match stage as u8 {
        0xff if outcome as u8 == 6 => {
            covered!("rx13_8 brazilian quarter final headline");
            let long = news != 0;
            let t = if long { BRA_FIRST_QUARTER_FINAL_TEXT } else { BRA_FIRST_QUARTER_FINAL_SHORT_TEXT };
            fate_headline(text, club_id, read_usize(this + 4), competition::SHORT_NAME, t, long, true)
        }
        0xff => fate_base_text(this, club_id, outcome, -1, a4, a5, a6, a8, news, text),
        0 => bra_final_fate(this, club_id, outcome, a4, a5, a6, a8, news, text),
        _ => {}
    }
}

/// rule: the Brazilian second division news of a club's fate: in the final (stage 0) the winner "{club} win {league}"
/// and the loser "{club} lose in {league} final" (without a news object); every other fate, the base competition text
///
/// `BraNatSecond::build_fate_news_text(..)` (10 dwords, ret 0x28). Stage (byte) 0xff: the base text with stage −1.
/// Stage 0: `bra_final_fate`. Other stages: nothing.
/// Original: `BraNatSecond::build_fate_news_text` (0x42d0d0, bra_nat_second.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn bra_nat_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    match stage as u8 {
        0xff => fate_base_text(this, club_id, outcome, -1, a4, a5, a6, a8, news, text),
        0 => bra_final_fate(this, club_id, outcome, a4, a5, a6, a8, news, text),
        _ => {}
    }
}

/// rule: the Brazilian third division news of a club's fate: in the first round (stages below 7) a club through gets
/// "{club} have qualified for the second round of the {league}." (with a news object) or "{club} qualify for second
/// round"; the champion of the last stage (10) without a news object gets the title headline; every other fate up to
/// stage 10, the base competition text
///
/// `BraNatThird::build_fate_news_text(..)` (10 dwords, ret 0x28). Outcome (byte) 6 with the stage (signed byte) below 7:
/// the headline (`fate_headline`, the long one when `news`; both links). Otherwise, stage 7: the base text with stage
/// 7; stages below 10: the base text with the stage argument as passed; stage 10: without `news` and outcome 1, the
/// title (`bra_third_champion_headline`); else the base text with stage 10 and then, again, the shared buffer set as the
/// text with both links (`bra_third_relink`). Above 10: nothing.
/// Original: `BraNatThird::build_fate_news_text` (0x42ec60, bra_nat_third.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn bra_nat_third_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    if outcome as u8 == 6 && s < 7 {
        covered!("rx13_8 brazilian third second round headline");
        let long = news != 0;
        let t = if long { BRA_THIRD_SECOND_ROUND_TEXT } else { BRA_THIRD_SECOND_ROUND_SHORT_TEXT };
        return fate_headline(text, club_id, read_usize(this + 4), competition::SHORT_NAME, t, long, true);
    }
    match s {
        7 => fate_base_text(this, club_id, outcome, 7, a4, a5, a6, a8, news, text),
        s if s < 10 => fate_base_text(this, club_id, outcome, stage as i32, a4, a5, a6, a8, news, text),
        10 if news == 0 && outcome as u8 == 1 => {
            covered!("rx13_8 brazilian third champion");
            bra_third_champion_headline(text, club_id, read_usize(this + 4))
        }
        10 => {
            fate_base_text(this, club_id, outcome, 10, a4, a5, a6, a8, news, text);
            bra_third_relink(text, club_id, read_usize(this + 4)) // rx13_8: the base text set again
        }
        _ => {}
    }
}

/// The third division's title headline, as the original pushes it: the club's gender twice, the competition's gender
/// once, then the club's name, the competition's short name twice (the template wants four genders and two names, so
/// the club's name goes in as the fourth gender and the competition's name in the club's place: a defect of the
/// original); then the text and both links (`bra_third_relink`).
unsafe fn bra_third_champion_headline(text: usize, club_id: u32, comp: usize) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let cg = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    let comp_name = comp + competition::SHORT_NAME;
    story_lang!(PRAISE_TEXT_BUFFER, BRA_THIRD_WIN_TEXT, g, g, cg, name, comp_name, comp_name); // rx13_8: the original's arguments
    bra_third_relink(text, club_id, comp);
}

/// The end of the third division's stage 10: the shared buffer set as the text (`exe::text_set_string`) and the club's
/// (0x7d5) and the competition's (0x7d0) links.
unsafe fn bra_third_relink(text: usize, club_id: u32, comp: usize) {
    let club = club_at(club_id as isize);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// The comparer of a fate text routine: `text_only` (the path writes only the text: the headlines) side by side with
/// `compare_text_writer` (the original into the caller's Text, the copy into a local one); the paths through the base
/// competition text only the original (it may also write the news object, which takes its text once).
pub(crate) unsafe fn compare_fate_text(name: &'static str, text_only: bool, club_id: u32, outcome: u32, stage: u32, news: u32, text: usize,
    original: impl FnOnce(), copy: impl FnOnce(usize)) {
    if !text_only || text == 0 {
        return original();
    }
    compare_text_writer(text, original, copy, || format!("{}(club {}, outcome {}, stage {}, news {})", name, club_id, outcome as u8, stage as u8 as i8, news));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    let text_only = match stage as u8 {
        0xff => outcome as u8 == 6,
        0 => news == 0 && bra_final_headline(outcome as u8).is_some(),
        _ => false,
    };
    compare_fate_text("bra_nat_first_build_fate_news_text", text_only, club_id, outcome, stage, news, text,
        || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| bra_nat_first_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    let text_only = stage as u8 == 0 && news == 0 && bra_final_headline(outcome as u8).is_some();
    compare_fate_text("bra_nat_second_build_fate_news_text", text_only, club_id, outcome, stage, news, text,
        || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| bra_nat_second_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I2%%);
    let s = stage as u8 as i8;
    let text_only = (outcome as u8 == 6 && s < 7) || (s == 10 && news == 0 && outcome as u8 == 1);
    compare_fate_text("bra_nat_third_build_fate_news_text", text_only, club_id, outcome, stage, news, text,
        || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| bra_nat_third_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}
