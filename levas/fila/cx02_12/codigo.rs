// ---------- cx02_12: the Serie C cup fate news and the Italian cups' progress codes (ita_c_cup.cpp, ita_cup.cpp) ----------

/// The Serie C cup fate news: `group_cup_fate_news_text` with the knockout at stage 15 and "through to second round"
/// (0x9a7e24).
/// Original: `ItaCCup::build_fate_news_text` (0x63d4c0, ita_c_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn ita_c_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 15, 0x9a7e24)
}

/// The Serie C cup: fifteen groups (stages 0 to 14); the group places 1 to 4 become 1, 33, 59 and 75 (the worst place
/// 1 becomes 17 and 4 becomes 71).
pub(crate) const SERIE_C_CUP_PROGRESS: GroupCupProgress = GroupCupProgress { knockout: 15, current: &[1, 33, 59, 75], best: &[1, 33, 59, 75], worst: &[17, 33, 59, 71] };

/// The Coppa Italia: seven groups (stages 0 to 6); the group places 1 to 4 become 1, 25, 33 and 41 (the worst place 1
/// becomes 17).
pub(crate) const COPPA_ITALIA_PROGRESS: GroupCupProgress = GroupCupProgress { knockout: 7, current: &[1, 25, 33, 41], best: &[1, 25, 33, 41], worst: &[17, 25, 33, 41] };

/// The Serie C cup progress codes (`set_group_cup_club_progress` with `SERIE_C_CUP_PROGRESS`).
/// Original: `ItaCCup::set_club_progress` (0x63e7f0, ita_c_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, &SERIE_C_CUP_PROGRESS)
}

/// The Coppa Italia progress codes (`set_group_cup_club_progress` with `COPPA_ITALIA_PROGRESS`).
/// Original: `ItaCup::set_club_progress` (0x640480, ita_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ita_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, &COPPA_ITALIA_PROGRESS)
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    compare_group_cup_fate_news_text("ita_c_cup_build_fate_news_text", original_fn::<FateNewsText>(%%I0%%), ita_c_cup_build_fate_news_text,
        this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text, 15)
}

pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_club_progress("ita_c_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || ita_c_cup_set_club_progress(this, club, stage, current, best, worst));
}

pub(crate) unsafe extern "thiscall" fn compare_ita_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_club_progress("ita_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || ita_cup_set_club_progress(this, club, stage, current, best, worst));
}
