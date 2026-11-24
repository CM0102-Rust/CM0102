// ---------- cx02_20: the Greek Cup fate news (gre_cup.cpp) ----------

/// The Greek Cup fate news: `group_cup_fate_news_text` with the knockout at stage 10 and "through to second round"
/// (0x9a7e24).
/// Original: `GreCup::build_fate_news_text` (0x5e9140, gre_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn gre_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 10, 0x9a7e24)
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    compare_group_cup_fate_news_text("gre_cup_build_fate_news_text", original_fn::<FateNewsText>(%%I0%%), gre_cup_build_fate_news_text,
        this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text, 10)
}
