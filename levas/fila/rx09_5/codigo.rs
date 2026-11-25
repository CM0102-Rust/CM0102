// ---------- rx09_5: the Spanish Second Division B fate news text, club list and predicted positions (spa_second_b.cpp) ----------

/// The text of a club that missed promotion from a play-off group (with a news item): the stage's name
/// (`exe::comp_util_get_stage_name` of `stage_code` into a local of 0x64 bytes) and its gender byte
/// (`exe::comp_util_get_stage_name_gender`), then 0xa829ac "{club} have failed to win promotion from {group} in the
/// {competition}." with the club's short gender byte twice, the stage's twice and the competition's (the long name's,
/// `competition::GENDER_NAME`) twice, the club's short name, the stage name and the competition's name
/// (`competition::NAME`) into the shared buffer (`PRAISE_TEXT_BUFFER`); the buffer set in `text` with the club's link
/// (0x7d5) and the competition's (0x7d0).
pub(crate) unsafe fn failed_promotion_from_group_text(club_id: u32, comp: usize, stage_code: u32, text: usize) {
    let mut name = [0u8; 0x64];
    exe::comp_util_get_stage_name(name.as_mut_ptr(), 0x64, stage_code, 0);
    let sg = exe::comp_util_get_stage_name_gender(stage_code, 0) as i8 as i32;
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let cg = read_u8(comp + competition::GENDER_NAME) as i8 as i32;
    let team = club.addr() + club::SHORT_NAME;
    story_lang!(PRAISE_TEXT_BUFFER, 0xa829ac, g, g, sg, sg, cg, cg, team, name.as_ptr(), comp + competition::NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, team as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
}

/// True when `spa_second_b_build_fate_news_text` writes the text itself (not the common text nor the cup tie news).
pub(crate) fn segunda_b_writes_own_text(stage: i8, outcome: u8, news: u32) -> bool {
    match stage {
        3..=6 => outcome == 0xfe,
        7 | 8 => news == 0 && (outcome == 1 || outcome == 2),
        _ => false,
    }
}

/// The fate news text of a Spanish Second Division B club (thiscall, ret 0x28; same arguments as
/// Comp::build_fate_news_text):
/// - the groups (stage below 3): the common text (`exe::comp_build_fate_news_text`, a7 = 0, the rest as they came);
/// - the promotion play-off groups (3 to 6): outcome 0 the common text with outcome 0; outcome 0xfe, with a news item,
///   `failed_promotion_from_group_text` (the stage code a4) and, without one, 0xa82970 "{club} fail to win promotion"
///   (`play_off_club_text`); any other outcome nothing;
/// - the relegation play-off (7) and its final (8): with a news item the cup tie news
///   (`exe::comp_util_write_cup_tie_result_news`(club, a5, a8, outcome, a6, text)); without one, outcome 1 gives
///   0x987594 "{club} avoid relegation", outcome 2 0xa82934 "{club} slip nearer relegation" at 7 and 0x987564 "{club}
///   relegated" at 8; any other nothing;
/// - later stages: nothing.
/// Original: `SpaSecondB::build_fate_news_text` (0x857e20, spa_second_b.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn spa_second_b_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    let common = |outcome: u32| exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    if s < 3 {
        return common(outcome);
    }
    if s < 7 {
        match outcome as u8 {
            0 => common(0),
            0xfe if news != 0 => {
                covered!("rx09_5 segunda b failed promotion news");
                failed_promotion_from_group_text(club_id, League(this).competition().addr(), a4, text);
            }
            0xfe => {
                covered!("rx09_5 segunda b failed promotion text");
                play_off_club_text(club_id, 0xa82970, text); // rx09_5: "{club} fail to win promotion"
            }
            _ => {}
        }
        return;
    }
    if s != 7 && s != 8 {
        return;
    }
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    match outcome as u8 {
        1 => play_off_club_text(club_id, 0x987594, text), // "{club} avoid relegation"
        2 => {
            covered!("rx09_5 segunda b relegation text");
            play_off_club_text(club_id, if s == 7 { 0xa82934 } else { 0x987564 }, text); // rx09_5: slip nearer relegation / relegated
        }
        _ => {}
    }
}

/// Comparer for `spa_second_b_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the common text and the cup tie news only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    let s = stage as u8 as i8;
    if !segunda_b_writes_own_text(s, outcome as u8, news) || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| spa_second_b_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("spa_second_b_build_fate_news_text(club {}, outcome {:#x}, stage {}, news {})", club_id, outcome as u8, s, news));
}

/// The clubs of the Spanish Second Division B: 80 slots (the four groups of 20), filled by `grouped_league_club_array`
/// with the group stages 0..2, each copied for the league's own club count (only group 1 and the last group stage end
/// up in the list: the defect described there); out of memory: the box at line 0x4f2.
/// Original: `SpaSecondB::get_club_array` (0x8581a0, spa_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn spa_second_b_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    grouped_league_club_array(League(this), list, n, 0x50, 3, true, SEGUNDA_B_SOURCE, 0x4f2);
}

/// Comparer for `spa_second_b_get_club_array` (`compare_grouped_league_club_array`).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(%%I1%%);
    compare_grouped_league_club_array("spa_second_b_get_club_array", League(this), list, n, 0x50, 3, true, || o(this, list, n));
}

/// The predicted positions of the Spanish Second Division B: each of the four groups (group 1 in the league's table, the
/// others in the group stages 0..2) sorted by strength, and the club in place k gets 4k + 1 whatever its group
/// (`grouped_league_predicted_positions`; a list of 4 × `league::CLUB_COUNT` bytes, boxes at lines 0x519 and 0x521).
/// Original: `SpaSecondB::set_predicted_positions` (0x8582f0, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_set_predicted_positions(this: usize) {
    let l = League(this);
    let size = (l.club_count() as i32 * 4) as u32;
    grouped_league_predicted_positions(l, 3, size, SEGUNDA_B_SOURCE, (0x519, 0x521), |_, k| k.wrapping_mul(4).wrapping_add(1)); // rx09_5: 4k + 1
}

/// Comparer for `spa_second_b_set_predicted_positions` (`compare_grouped_predicted_positions`).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_grouped_predicted_positions("spa_second_b_set_predicted_positions", League(this), 3, || o(this), || spa_second_b_set_predicted_positions(this));
}
