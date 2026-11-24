// ---------- cx02_10: the Irish and Northern Irish League Cups' fate news (ire_lge_cup.cpp, nir_lge_cup.cpp) ----------

/// The fate news text of the cups built on a league with group stages (the Irish, Northern Irish and Korean League
/// Cups, the Serie C cup and the Greek and Dutch Cups; the Coppa Italia's is the same with its own copy), identical
/// apart from the knockout stage and the "through to" template. In the groups (stage, a signed byte, below `knockout`):
/// qualification (outcome 6) with a news object goes to the generic group news
/// (`exe::comp_util_write_group_qualify_news`(club id, a4, the competition record, 6, news, text)); without one, the
/// club (`club_at`) gets the line from `template` (`FN_LANGLIB_SPRINTF` into 0xde1f64 with its gender byte +0x52 twice
/// and its name +0x38) set into `text` with a link to the club (0x7d5); elimination (0xfe) goes to the generic group
/// news with −2; other outcomes, nothing. In the knockout stage: with a news object the cup tie result news
/// (`exe::comp_util_write_cup_tie_result_news`(club id, a5, a8, outcome, a6, text)), without one the win or defeat
/// news (`exe::comp_util_write_cup_win_or_defeat_news`(club id, a5, outcome, text)). Later stages: nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn group_cup_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a8: u32, news: u32, text: usize,
    knockout: i8, template: usize) {
    let (outcome_byte, stage) = (outcome as u8, stage as u8 as i8);
    let competition = League(this).competition().addr();
    if stage < knockout {
        if outcome_byte == 6 {
            if news != 0 {
                return exe::comp_util_write_group_qualify_news(club_id, a4, competition, 6, news, text);
            }
            covered!("cx02_10 group cup through to the next round");
            let club = club_at(club_id as isize);
            let gender = read_i8(club.addr() + 0x52) as i32; // the name's gender byte
            story_lang!(0xde1f64, template, gender, gender, club.addr() + 0x38);
            exe::text_set_string(text, 0xde1f64usize as *const u8);
            exe::text_add_link(text, (club.addr() + 0x38) as *const u8, 0x7d5, club.addr());
        } else if outcome_byte == 0xfe {
            exe::comp_util_write_group_qualify_news(club_id, a4, competition, -2, news, text);
        }
    } else if stage == knockout {
        if news != 0 {
            exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
        } else {
            exe::comp_util_write_cup_win_or_defeat_news(club_id, a5, outcome, text);
        }
    }
}

/// The original of a fate news routine, called with its ten arguments.
pub(crate) type FateNewsText = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize);

/// Comparer of the group cups' fate news: the "through to" line (a group qualifier without a news object) side by side
/// with `compare_text_writer` (the original into the caller's Text, the copy into a local one); the paths that pass
/// the news object or the text to the generic news, only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_group_cup_fate_news_text(name: &str, original: FateNewsText, copy: FateNewsText, this: usize, club_id: u32, outcome: u32, stage: u32,
    a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize, knockout: i8) {
    if (stage as u8 as i8) < knockout && outcome as u8 == 6 && news == 0 && text != 0 {
        return compare_text_writer(text, || original(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
            |t| copy(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
            || format!("{}(club {}, outcome {}, stage {})", name, club_id, outcome as u8, stage as u8 as i8));
    }
    original(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)
}

/// The Irish League Cup fate news: `group_cup_fate_news_text` with the knockout at stage 7 and "through to quarter
/// finals" (0x9a7e9c).
/// Original: `IreLgeCup::build_fate_news_text` (0x6374a0, ire_lge_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 7, 0x9a7e9c)
}

/// The Northern Irish League Cup fate news: `group_cup_fate_news_text` with the knockout at stage 3 and "through to
/// quarter finals" (0x9a7e9c).
/// Original: `NirLgeCup::build_fate_news_text` (0x78ceb0, nir_lge_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 3, 0x9a7e9c)
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    compare_group_cup_fate_news_text("ire_lge_cup_build_fate_news_text", original_fn::<FateNewsText>(%%I0%%), ire_lge_cup_build_fate_news_text,
        this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text, 7)
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    compare_group_cup_fate_news_text("nir_lge_cup_build_fate_news_text", original_fn::<FateNewsText>(%%I1%%), nir_lge_cup_build_fate_news_text,
        this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text, 3)
}
