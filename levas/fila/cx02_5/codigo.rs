// ---------- cx02_5: the Auto Windscreens Shield fate news (eng_auto_cup.cpp) ----------

/// Writes the news text of a club's fate in the Auto Windscreens Shield. With a news object (`news`), the cup tie
/// result news whatever the stage (`exe::comp_util_write_cup_tie_result_news`(club id, a5, a8, outcome, a6, text)).
/// Without one: a section winner (stage −1 or 0, outcome 1) gets the headline "{club} through to {competition} final"
/// (`FN_LANGLIB_SPRINTF` into the shared buffer 0xde1f64 with the template 0x9c41bc: the club's gender byte twice, the
/// competition's gender byte twice, the club's name and the competition's name), set into `text`
/// (`exe::text_set_string`) with a link to the club (0x7d5) and one to the competition (0x7d0); any other fate goes
/// to `Comp::build_fate_news_text` with the same arguments but a7 and the news object 0.
/// Original: `EngAutoCup::build_fate_news_text` (0x56b2b0, eng_auto_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    let section = stage as u8 == 0xff || stage as u8 == 0;
    if section && outcome as u8 == 1 {
        covered!("cx02_5 auto windscreens section winner");
        let competition = League(this).competition();
        let club = club_at(club_id as isize);
        let (club_gender, comp_gender) = (read_i8(club.addr() + 0x52) as i32, read_i8(competition.addr() + 0x52) as i32); // the names' gender bytes
        story_lang!(0xde1f64, 0x9c41bc, club_gender, club_gender, comp_gender, comp_gender, club.addr() + 0x38, competition.addr() + 0x38);
        exe::text_set_string(text, 0xde1f64usize as *const u8);
        let club = club_at(club_id as isize);
        exe::text_add_link(text, (club.addr() + 0x38) as *const u8, 0x7d5, club.addr());
        exe::text_add_link(text, (competition.addr() + 0x38) as *const u8, 0x7d0, competition.addr());
        return;
    }
    exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, 0, text);
}

/// Comparer for `eng_auto_cup_build_fate_news_text`: the section winner's headline (no news object) side by side with
/// `compare_text_writer` (the original into the caller's Text, the copy into a local one); the other paths, which pass
/// the news object or the text on to the generic news, only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    if news == 0 && (stage as u8 == 0xff || stage as u8 == 0) && outcome as u8 == 1 && text != 0 {
        return compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
            |t| eng_auto_cup_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
            || format!("eng_auto_cup_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
    }
    o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)
}
