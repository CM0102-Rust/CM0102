// ---------- rx11_1: the news texts of a nation's fate in the African and CONCACAF World Cup qualifying (wc_african_cup.cpp, wc_concacaf_cup.cpp) ----------

/// The fate news templates of the African qualifying (langlib, `.rdata`).
pub(crate) mod wc_african_fate_texts {
    /// "{}<%s - Team Name (e.g. Scotland)>{} qualify for Final"
    pub(crate) const QUALIFY_FOR_FINAL: usize = 0xad45fc;
    /// "{}<%s - Team Name (e.g. Scotland)>{} qualify for {}<%s - World Cup>{}" (from a group)
    pub(crate) const GROUP_QUALIFY_FOR_WORLD_CUP: usize = 0xad45b4;
    /// "{}<%s - Team Name (e.g. South Africa)>{} qualify for 2nd round in qualifiers"
    pub(crate) const QUALIFY_FOR_SECOND_ROUND: usize = 0xad4564;
    /// "{}<%s - Team Name (e.g. South Africa)>{} knocked out in 1st round qualifier"
    pub(crate) const KNOCKED_OUT_IN_FIRST_ROUND: usize = 0xad4518;
    /// "{}<%s - Team Name (e.g. South Africa)>{} qualify for {}<%s - World Cup>{}" (from a tie)
    pub(crate) const TIE_QUALIFY_FOR_WORLD_CUP: usize = 0xad44cc;
    /// "{}<%s - Team Name (e.g. South Africa)>{} knocked out of {}<%s - World Cup>{}"
    pub(crate) const KNOCKED_OUT_OF_WORLD_CUP: usize = 0xad447c;
}

/// The fate news templates of the CONCACAF qualifying (langlib, `.rdata`).
pub(crate) mod wc_concacaf_fate_texts {
    /// "{}<%s - Team Name(e.g.Canada)>{} qualify for Semi Finals"
    pub(crate) const QUALIFY_FOR_SEMI_FINALS: usize = 0xad49e4;
    /// "{}<%s - Team Name(e.g.Canada)>{} have qualified from the semi final stage of the {}<%s - Competition Name>{}."
    pub(crate) const QUALIFIED_FROM_SEMI_FINAL_STAGE: usize = 0xad4974;
    /// "{}<%s - Team Name(e.g.Canada)>{} qualify for final"
    pub(crate) const QUALIFY_FOR_FINAL: usize = 0xad4940;
    /// "{}<%s - Team Name(e.g.Canada)>{} have qualified from the final stage of the {}<%s - Competition Name>{}."
    pub(crate) const QUALIFIED_FROM_FINAL_STAGE: usize = 0xad48d4;
    /// "{}<%s - Team Name(e.g.Canada)>{} qualify for {}<%s - World Cup>{}"
    pub(crate) const QUALIFY_FOR_WORLD_CUP: usize = 0xad4890;
}

/// The club of a club index in [`G_CLUB_TABLE`] (`club::SIZE` bytes each).
pub(crate) unsafe fn club_of_index(index: u32) -> Club {
    Club(read_usize(G_CLUB_TABLE).wrapping_add((index as usize).wrapping_mul(club::SIZE)))
}

/// One line of a qualifying fate news: `template` (`FN_LANGLIB_SPRINTF`, cdecl, into `NEWS_TEXT_BUFFER`) with the
/// club's short gender twice and, with `other` (a gender byte's address and a name), that gender twice, then the club's
/// short name and the other name; set as the text (`exe::text_set_string`). The caller adds the links.
pub(crate) unsafe fn qualifying_fate_line(text: usize, template: usize, club: Club, other: Option<(usize, usize)>) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let g = club.short_gender_name() as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    match other {
        Some((gender_at, other_name)) => {
            let og = read_u8(gender_at) as i8 as i32;
            sprintf(NEWS_TEXT_BUFFER, template, g, g, og, og, name, other_name);
        }
        None => {
            sprintf(NEWS_TEXT_BUFFER, template, g, g, name);
        }
    }
    exe::text_set_string(text, NEWS_TEXT_BUFFER as *const u8);
}

/// The club's link in a fate news (its short name, kind 0x7d5).
pub(crate) unsafe fn link_fate_club(text: usize, club: Club) {
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
}

/// A competition record's link in a fate news (its short name, kind 0x7d0).
pub(crate) unsafe fn link_fate_competition(text: usize, record: usize) {
    exe::text_add_link(text, (record + competition::SHORT_NAME) as *const u8, 0x7d0, record);
}

/// rule: the African qualifying fate news: in the groups (stage codes 0x407 to 0x40b and the final group 0x472), the
/// shared group sentence, or, in short, "qualify for Final" (fate 6) and "qualify for the World Cup" (fate 7); in the
/// first round (stage −1) and in the ties (code 0), the shared tie sentence, or, in short, "qualify for 2nd round" or
/// "knocked out in 1st round" in the first round and "qualify for" or "knocked out of" the World Cup in the ties
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48; the
/// club in [`G_CLUB_TABLE`] by index, the fate byte, the stage byte (signed), the stage code (a signed word), four
/// values passed on, `full_sentence` and the Text).
///
/// Stage other than −1 with a code: a code outside 0x407..=0x40b and 0x472 writes nothing. With `full_sentence`, the
/// shared group sentence (`exe::comp_util_write_group_qualify_news`(club_index, code, the competition record
/// `league::COMPETITION`, the fate dword, full_sentence, text), cdecl). Otherwise fate 6: "qualify for Final" with the
/// club's link; fate 7: "qualify for the World Cup" (the World Cup record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`]: its short
/// gender and short name) with the club's link and the World Cup's; any other fate, the shared group sentence with 0
/// for `full_sentence`.
///
/// Stage −1 or code 0: with `full_sentence`, the shared tie sentence (`exe::comp_util_write_cup_tie_result_news`(
/// club_index, a5, a8, the fate dword, a6, text), cdecl). Otherwise in the first round (stage −1) fate 1 "qualify for
/// 2nd round in qualifiers", any other "knocked out in 1st round qualifier" (the club only); in a tie fate 1 "qualify
/// for", any other "knocked out of" the World Cup (its gender and short name); then the World Cup's link and the
/// club's, in that order.
/// Original: `WcAfricanCup::build_fate_news_text` (0x90fe00, wc_african_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_african_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use wc_african_fate_texts as t;
    let _ = a7;
    let comp = League(this).competition().addr();
    let club = club_of_index(club_index);
    let world_cup = key_comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    let wc_name = Some((world_cup + competition::SHORT_GENDER_NAME, world_cup + competition::SHORT_NAME));
    let first_round = stage as u8 as i8 == -1;
    let raw_code = code;
    let code = code as u16 as i16;
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    if !first_round && code != 0 {
        if !((0x407..=0x40b).contains(&code) || code == 0x472) {
            return;
        }
        covered!("rx11_1 african group fate");
        if full_sentence != 0 {
            return exe::comp_util_write_group_qualify_news(club_index, raw_code, comp, fate as i32, full_sentence, text);
        }
        match fate as u8 {
            6 => {
                qualifying_fate_line(text, t::QUALIFY_FOR_FINAL, club, None);
                link_fate_club(text, club);
            }
            7 => {
                qualifying_fate_line(text, t::GROUP_QUALIFY_FOR_WORLD_CUP, club, wc_name);
                link_fate_club(text, club); // rx11_1: the club's link first in a group
                link_fate_competition(text, world_cup);
            }
            _ => exe::comp_util_write_group_qualify_news(club_index, raw_code, comp, fate as i32, 0, text),
        }
        return;
    }
    covered!("rx11_1 african tie fate");
    if full_sentence != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
    }
    let through = fate as u8 == 1;
    if first_round {
        qualifying_fate_line(text, if through { t::QUALIFY_FOR_SECOND_ROUND } else { t::KNOCKED_OUT_IN_FIRST_ROUND }, club, None);
    } else {
        qualifying_fate_line(text, if through { t::TIE_QUALIFY_FOR_WORLD_CUP } else { t::KNOCKED_OUT_OF_WORLD_CUP }, club, wc_name);
    }
    link_fate_competition(text, world_cup); // rx11_1: the World Cup's link first in a tie
    link_fate_club(text, club);
}

/// rule: the CONCACAF qualifying fate news: in the Caribbean knockout (code 0x40f) and the play-off (stage 2), the
/// shared tie sentence, or, in short, "qualify for Semi Finals" for a winner and the shared win-or-defeat sentence
/// otherwise; in the semi-final groups (codes 0x40c to 0x40e), "have qualified from the semi final stage" (or, in
/// short, "qualify for final") for a qualifier (6) and the group sentence for one going out (−2); in the final group
/// (code 0x96), "have qualified from the final stage" (or, in short, "qualify for the World Cup") for a qualifier (7)
/// and the group sentence for one going out (−2)
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48), as
/// the African one. Code 0x40f or stage byte 2: with `full_sentence`, `exe::comp_util_write_cup_tie_result_news`(
/// club_index, a5, a8, the fate dword, a6, text); otherwise fate 1 "qualify for Semi Finals" with the club's link, any
/// other `exe::comp_util_write_cup_win_or_defeat_news`(club_index, a5, the fate dword, text) (cdecl).
///
/// Codes 0x40c..=0x40e (signed): fate 6, with `full_sentence` "have qualified from the semi final stage of the" the
/// competition (its long name `competition::NAME` with the short gender byte), the club's link and the competition's
/// (its short name); without, "qualify for final" with the club's link. Fate 0xfe: the shared group sentence
/// (`exe::comp_util_write_group_qualify_news`(club_index, code, the competition record, −2, full_sentence, text)). Code
/// 0x96: fate 7, with `full_sentence` "have qualified from the final stage of the" competition and the two links;
/// without, "qualify for" the World Cup (the record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`]: short gender and short name)
/// with the club's link and the World Cup's; fate 0xfe, the group sentence with code 0x96. Any other code or fate,
/// nothing.
/// Original: `WcConcacafCup::build_fate_news_text` (0x9159b0, wc_concacaf_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use wc_concacaf_fate_texts as t;
    let _ = a7;
    let comp = League(this).competition().addr();
    let club = club_of_index(club_index);
    let raw_code = code;
    let code = code as u16 as i16;
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    if code == 0x40f || stage as u8 == 2 {
        covered!("rx11_1 concacaf tie fate");
        if full_sentence != 0 {
            return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
        }
        if fate as u8 == 1 {
            qualifying_fate_line(text, t::QUALIFY_FOR_SEMI_FINALS, club, None);
            return link_fate_club(text, club);
        }
        return exe::comp_util_write_cup_win_or_defeat_news(club_index, a5, fate, text);
    }
    let (qualified, from_stage, in_short) = if (0x40c..=0x40e).contains(&code) {
        (6, t::QUALIFIED_FROM_SEMI_FINAL_STAGE, t::QUALIFY_FOR_FINAL)
    } else if code == 0x96 {
        (7, t::QUALIFIED_FROM_FINAL_STAGE, t::QUALIFY_FOR_WORLD_CUP)
    } else {
        return;
    };
    covered!("rx11_1 concacaf group fate");
    match fate as u8 {
        f if f == qualified => {
            if full_sentence != 0 {
                qualifying_fate_line(text, from_stage, club, Some((comp + competition::SHORT_GENDER_NAME, comp + competition::NAME)));
                link_fate_club(text, club);
                link_fate_competition(text, comp);
            } else if code == 0x96 {
                let world_cup = key_comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP);
                qualifying_fate_line(text, in_short, club, Some((world_cup + competition::SHORT_GENDER_NAME, world_cup + competition::SHORT_NAME)));
                link_fate_club(text, club);
                link_fate_competition(text, world_cup); // rx11_1: the World Cup from the final group
            } else {
                qualifying_fate_line(text, in_short, club, None);
                link_fate_club(text, club);
            }
        }
        0xfe => exe::comp_util_write_group_qualify_news(club_index, if code == 0x96 { 0x96 } else { raw_code }, comp, -2, full_sentence, text),
        _ => {}
    }
}

/// Comparers of the two fate texts: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| wc_african_cup_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("wc_african_cup_build_fate_news_text(club {}, fate {}, stage {}, code {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, code as u16, full_sentence));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| wc_concacaf_cup_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("wc_concacaf_cup_build_fate_news_text(club {}, fate {}, stage {}, code {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, code as u16, full_sentence));
}
