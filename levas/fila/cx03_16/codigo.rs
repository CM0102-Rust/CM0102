// ---------- cx03_16: the progress codes of the two Asian club cups and the Asian Cup Winners' Cup fate news (asia_club_champ.cpp, asia_cup_winner.cpp) ----------

/// rule: the Asian Club Championship progress codes: in the zone rounds (stages below 1) places 1, 2 and 3 become the
/// cup places 1, 9 and 17 for the current and best codes, and 7, 9 and 17 for the worst; in the two groups (stages 1
/// and 2) the best code gives 1 for places 1 and 2, 5 for 3 and 7 for 4, the worst 4, 9 and 17 for places 1 to 3, the
/// current 1, 9 and 17; a 4th place in the worst or the current code keeps that code and makes the best 7; from the
/// semi-finals on the codes stay as they come. Other codes stay.
///
/// `set_club_progress(club, stage, current, best, worst)` (virtual `comp_vtable::SET_CLUB_PROGRESS`, ret 0x14; the
/// jump tables 0x40cb64, 0x40cb74 and 0x40cb84 read); the codes are bytes (`store_club_progress`).
/// Original: `AsiaClubChamp::set_club_progress` (0x40ca20, asia_club_champ.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    let stage = stage as u8 as i8;
    if stage < 1 {
        covered!("cx03_16 asian club championship zone progress");
        best = remap_group_code(best, [1, 9, 0x11, 4]);
        worst = remap_group_code(worst, [7, 9, 0x11, 4]);
        current = remap_group_code(current, [1, 9, 0x11, 4]);
    } else if stage < 3 {
        covered!("cx03_16 asian club championship group progress");
        best = remap_group_code(best, [1, 1, 5, 7]);
        if worst == 4 {
            best = 7;
        } else {
            worst = remap_group_code(worst, [4, 9, 0x11, 4]);
        }
        if current == 4 {
            best = 7;
        } else {
            current = remap_group_code(current, [1, 9, 0x11, 4]);
        }
    }
    store_club_progress(this, club, current, best, worst);
}

/// rule: in the Asian Cup Winners' Cup zone rounds (stages below 1) places 1, 2, 3 and 4 become the cup places 1, 9, 17
/// and 17 for the current and best codes, and 7, 9, 17 and 17 for the worst; from the next stage on the codes stay as
/// they come (`store_club_progress`; the jump tables 0x40e778, 0x40e788 and 0x40e798 read).
/// Original: `AsiaCupWinner::set_club_progress` (0x40e6b0, asia_cup_winner.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < 1 {
        covered!("cx03_16 asian cup winners zone progress");
        best = remap_group_code(best, [1, 9, 0x11, 0x11]);
        worst = remap_group_code(worst, [7, 9, 0x11, 0x11]);
        current = remap_group_code(current, [1, 9, 0x11, 0x11]);
    }
    store_club_progress(this, club, current, best, worst);
}

/// rule: the Asian Cup Winners' Cup news of a club's fate: in the zone rounds (stages below 1) the cup tie result; from
/// the next stage on the generic competition news.
///
/// `build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28). Stage (a
/// signed byte) below 1: `exe::comp_util_write_cup_tie_result_news`(club_id, a5, a8, outcome, a6, text) (cdecl);
/// otherwise Comp::build_fate_news_text (`exe::comp_build_fate_news_text`, thiscall, ret 0x28) with the same arguments
/// but a7 cleared.
/// Original: `AsiaCupWinner::build_fate_news_text` (0x40d460, asia_cup_winner.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    if (stage as u8 as i8) < 1 {
        covered!("cx03_16 asian cup winners zone news");
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
}

pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_club_progress("asia_club_champ_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || asia_club_champ_set_club_progress(this, club, stage, current, best, worst));
}

pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_club_progress("asia_cup_winner_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || asia_cup_winner_set_club_progress(this, club, stage, current, best, worst));
}

/// Comparer for `asia_cup_winner_build_fate_news_text`: in the zone rounds the tie result (a text writer alone,
/// 0x4b0b80) side by side with `compare_text_writer`; the generic competition news (which may also write the news
/// object) only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I2%%);
    if (stage as u8 as i8) < 1 && text != 0 {
        return compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
            |t| asia_cup_winner_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
            || format!("asia_cup_winner_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
    }
    o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)
}
