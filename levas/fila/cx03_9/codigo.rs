// ---------- cx03_9: the fate news of the two Welsh league cups and the Welsh League Cup progress (wel_lge_cup.cpp, wel_prm_cup.cpp) ----------

/// The text "{club} through to ..." of a club going through a group: the text (`langlib_sprintf`(0xde1f64, `format`, the
/// club's `club::SHORT_GENDER_NAME` byte twice, its `club::SHORT_NAME`): 5 pushes; add esp, 0x14 -> cdecl) set in
/// `text` (`exe::text_set_string` with 0xde1f64) with the club link (`exe::text_add_link`(text, short name, 0x7d5,
/// club)).
pub(crate) unsafe fn club_through_text(text: usize, club_id: u32, format: usize) {
    let club = club_at(club_id as isize);
    let b = club.short_gender_name() as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    let sprintf: unsafe extern "C" fn(usize, usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    sprintf(0xde1f64, format, b, b, name);
    exe::text_set_string(text, 0xde1f64usize as *const u8);
    exe::text_add_link(text, name as *const u8, 0x7d5, club); // the club link
}

/// The news text of a club's fate in a cup played in groups then a knockout stage: the same body in WelLgeCup,
/// WelPrmCup, ConMercCup, ConmebolLiber and ConmebolMerc (checked instruction by instruction: only the knockout stage
/// and the "through" text change), and in ItaCup with stage 7.
///
/// `build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28). Stage (a
/// signed byte) below `knockout`: outcome 6 with `news`, the generic group sentence
/// (`exe::comp_util_write_group_qualify_news`(club_id, a4, the competition `+4`, 6, news, text), cdecl); 6 without
/// `news`, the `through` text (`club_through_text`); 0xfe, the group sentence with −2; other outcomes nothing. At the
/// knockout stage: with `news`, the tie result (`exe::comp_util_write_cup_tie_result_news`(club_id, a5, a8, outcome, a6,
/// text)); without, the win or defeat (`exe::comp_util_write_cup_win_or_defeat_news`(club_id, a5, outcome, text)), both
/// cdecl. Above it: nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn write_group_cup_fate_news(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a8: u32, news: u32, text: usize,
    knockout: i8, through: usize, branch: &'static str) {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < knockout {
        if d == 6 {
            if news != 0 {
                return exe::comp_util_write_group_qualify_news(club_id, a4, comp, 6, news, text);
            }
            covered!(branch);
            club_through_text(text, club_id, through);
        } else if d == 0xfe {
            exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, news, text);
        }
    } else if f == knockout {
        if news != 0 {
            exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
        } else {
            exe::comp_util_write_cup_win_or_defeat_news(club_id, a5, outcome, text);
        }
    }
}

/// Comparer of `write_group_cup_fate_news`: the "through" text alone (a club going through the groups without a news
/// object) side by side with `compare_text_writer` (the original into the caller's Text, the copy into a local one);
/// the paths that pass the news object on to the generic cup news only the original (they may also write the news
/// object, which takes its text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_group_cup_fate_news(name: &'static str, knockout: i8, club_id: u32, outcome: u32, stage: u32, news: u32, text: usize,
    original: impl FnOnce(), copy: impl FnOnce(usize)) {
    if (stage as u8 as i8) < knockout && outcome as u8 == 6 && news == 0 && text != 0 {
        return compare_text_writer(text, original, copy, || format!("{}(club {}, outcome {}, stage {})", name, club_id, outcome as u8, stage as u8 as i8));
    }
    original()
}

/// rule: the Welsh League Cup news of a club's fate: in the groups (stages below 5) qualification and elimination
/// through the generic group sentence, or "{club} through to quarter finals" for a club going through with no news
/// object; in the knockout (stage 5) the tie result (`write_group_cup_fate_news`).
/// Original: `WelLgeCup::build_fate_news_text` (0x9265e0, wel_lge_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    write_group_cup_fate_news(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 5, 0x9a7e9c, "cx03_9 welsh league cup through to quarter finals") // cx03_9: the Welsh League Cup knockout
}

/// rule: the Welsh Premier Cup news of a club's fate: as the Welsh League Cup's, with the knockout at stage 1
/// (`write_group_cup_fate_news`, "{club} through to quarter finals").
/// Original: `WelPrmCup::build_fate_news_text` (0x927f40, wel_prm_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    write_group_cup_fate_news(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 1, 0x9a7e9c, "cx03_9 welsh premier cup through to quarter finals") // cx03_9: the Welsh Premier Cup knockout
}

/// Stores a club's three progress codes in its statistics entry (`exe::comp_stats_find_club_entry` on `league::COMP_STATS`,
/// thiscall, ret 4, the entry not checked): `comp_stats_club::CURRENT_POSITION`, `BEST_POSITION` and `WORST_POSITION`.
pub(crate) unsafe fn store_club_progress(this: usize, club: usize, current: u8, best: u8, worst: u8) {
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    entry.set_best_position(best);
    entry.set_current_position(current);
    entry.set_worst_position(worst);
}

/// rule: in the Welsh League Cup groups (stages below 5) the group places become cup progress codes: places 1 and 2
/// give 1 and place 3 gives 9 for the current and best codes, and for the worst place 1 gives 5 and places 2 and 3 give
/// 13; other codes (0 and above 3) stay; from the knockout on the codes stay as they come.
///
/// `set_club_progress(club, stage, current, best, worst)` (virtual `comp_vtable::SET_CLUB_PROGRESS`, ret 0x14), the
/// codes are bytes (`store_club_progress`).
/// Original: `WelLgeCup::set_club_progress` (0x927760, wel_lge_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < 5 {
        covered!("cx03_9 welsh league cup group progress");
        let place = |code: u8| match code {
            1 | 2 => 1,
            3 => 9,
            _ => code,
        };
        current = place(current);
        best = place(best);
        worst = match worst {
            1 => 5,
            2 | 3 => 0xd,
            _ => worst,
        };
    }
    store_club_progress(this, club, current, best, worst);
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_group_cup_fate_news("wel_lge_cup_build_fate_news_text", 5, club_id, outcome, stage, news, text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| wel_lge_cup_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    compare_group_cup_fate_news("wel_prm_cup_build_fate_news_text", 1, club_id, outcome, stage, news, text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| wel_prm_cup_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}

pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_club_progress("wel_lge_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || wel_lge_cup_set_club_progress(this, club, stage, current, best, worst));
}
