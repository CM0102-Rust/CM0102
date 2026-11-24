// ---------- cx03_14: the fate news of the three American continental cups (con_merc_cup.cpp, conmebol_liber.cpp, conmebol_merc.cpp) ----------

/// rule: the Copa Merconorte news of a club's fate: in the groups (stages below 3) qualification and elimination
/// through the generic group sentence, or "{club} through to semi finals" for a club going through with no news object;
/// in the knockout (stage 3) the tie result (`write_group_cup_fate_news`).
/// Original: `ConMercCup::build_fate_news_text` (0x4c1cc0, con_merc_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn con_merc_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    write_group_cup_fate_news(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 3, 0x9a7dac, "cx03_14 merconorte through to semi finals") // cx03_14: the knockout at stage 3
}

/// rule: the Copa Libertadores news of a club's fate: in the groups (stages below 7) as the Merconorte's,
/// with "{club} through to second round"; in the knockout (stage 7) the tie result (`write_group_cup_fate_news`).
/// Original: `ConmebolLiber::build_fate_news_text` (0x4c3680, conmebol_liber.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn conmebol_liber_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    write_group_cup_fate_news(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 7, 0x9a7e24, "cx03_14 libertadores through to second round") // cx03_14: the knockout at stage 7
}

/// rule: the Copa Mercosur news of a club's fate: in the groups (stages below 4) as the Merconorte's, with
/// "{club} through to quarter finals"; in the knockout (stage 4) the tie result (`write_group_cup_fate_news`).
/// Original: `ConmebolMerc::build_fate_news_text` (0x4c4d40, conmebol_merc.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn conmebol_merc_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    write_group_cup_fate_news(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 4, 0x9a7e9c, "cx03_14 mercosur through to quarter finals") // cx03_14: the knockout at stage 4
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_group_cup_fate_news("con_merc_cup_build_fate_news_text", 3, club_id, outcome, stage, news, text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| con_merc_cup_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    compare_group_cup_fate_news("conmebol_liber_build_fate_news_text", 7, club_id, outcome, stage, news, text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| conmebol_liber_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I2%%);
    compare_group_cup_fate_news("conmebol_merc_build_fate_news_text", 4, club_id, outcome, stage, news, text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| conmebol_merc_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t));
}
