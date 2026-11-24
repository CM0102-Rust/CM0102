// ---------- cx02_22: the Dutch Cup progress codes and fate news (hol_cup.cpp) ----------

/// The Dutch Cup: nineteen groups (stages 0 to 18); the group places 1 and 2 become 17 (the best, 1; the worst 27 and
/// 47), 3 and 4 become 47 and 77.
pub(crate) const DUTCH_CUP_PROGRESS: GroupCupProgress = GroupCupProgress { knockout: 0x13, current: &[17, 17, 47, 77], best: &[1, 1, 47, 77], worst: &[27, 47, 47, 77] };

/// The Dutch Cup progress codes (`set_group_cup_club_progress` with `DUTCH_CUP_PROGRESS`).
/// Original: `HolCup::set_club_progress` (0x5f5390, hol_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn hol_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, &DUTCH_CUP_PROGRESS)
}

/// The Dutch Cup fate news: `group_cup_fate_news_text` with the knockout at stage 19 and "through to second round"
/// (0x9a7e24).
/// Original: `HolCup::build_fate_news_text` (0x5f3c50, hol_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn hol_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 0x13, 0x9a7e24)
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_club_progress("hol_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || hol_cup_set_club_progress(this, club, stage, current, best, worst));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    compare_group_cup_fate_news_text("hol_cup_build_fate_news_text", original_fn::<FateNewsText>(%%I1%%), hol_cup_build_fate_news_text,
        this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text, 0x13)
}
