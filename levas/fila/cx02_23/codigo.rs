// ---------- cx02_23: the Korean League Cup predicted positions, progress codes and fate news (kor_league_cup.cpp) ----------

/// The Korean League Cup predicted positions: `italian_cup_predicted_positions` with the knockout at stage 1, bands of
/// 2 and the boxes at lines 0x38d and 0x395 of kor_league_cup.cpp (checked instruction by instruction against
/// WelPrmCup::set_predicted_positions: only the file and the lines change).
/// Original: `KorLeagueCup::set_predicted_positions` (0x66d3a0, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9eb15c, (0x38d, 0x395), 1, 2, "cx02_23 korean league cup predicted")
}

/// The Korean League Cup: two groups (stage 0); the group places 1 to 5 become 1, 1 (the worst, 3), 5, 7 and 9.
pub(crate) const KOREAN_LEAGUE_CUP_PROGRESS: GroupCupProgress = GroupCupProgress { knockout: 1, current: &[1, 1, 5, 7, 9], best: &[1, 1, 5, 7, 9], worst: &[3, 3, 5, 7, 9] };

/// The Korean League Cup progress codes (`set_group_cup_club_progress` with `KOREAN_LEAGUE_CUP_PROGRESS`).
/// Original: `KorLeagueCup::set_club_progress` (0x66d650, kor_league_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, &KOREAN_LEAGUE_CUP_PROGRESS)
}

/// The Korean League Cup fate news: `group_cup_fate_news_text` with the knockout at stage 1 and "through to semi
/// finals" (0x9a7dac).
/// Original: `KorLeagueCup::build_fate_news_text` (0x66c650, kor_league_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn kor_league_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 1, 0x9a7dac)
}

pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_italian_cup_predicted("kor_league_cup_set_predicted_positions", this, kor_league_cup_set_predicted_positions, || o(this));
}

pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_club_progress("kor_league_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || kor_league_cup_set_club_progress(this, club, stage, current, best, worst));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    compare_group_cup_fate_news_text("kor_league_cup_build_fate_news_text", original_fn::<FateNewsText>(%%I2%%), kor_league_cup_build_fate_news_text,
        this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text, 1)
}
