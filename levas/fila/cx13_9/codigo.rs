// ---------- cx13_9: the Scottish First Division's staff records and fate news text, the Premier's split order (sco_first.cpp, sco_prm.cpp) ----------

/// The Scottish First Division and its 1999 play-off group with the Premier Division's play-off club.
pub(crate) const SCOTTISH_FIRST_PLAY_OFF: PlayOffDivision = PlayOffDivision {
    own_key: KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION,
    upper_key: KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION,
    upper_stage: None,
    winner: 0,
    qualified: (0xa0, 0, 0x28),
};

/// Updates the staff records by a club's status in the Scottish First Division and its play-off group
/// (`play_off_update_staff_records` with `SCOTTISH_FIRST_PLAY_OFF`: the season's play-off place with 0xa0, 0, 0x28; in
/// the play-off group, which only exists in 1999, status 0 is the winner and the Premier club is looked for in the
/// Premier's own table). The last three arguments are not read.
/// Original: `ScoFirst::update_staff_records_for_status` (0x7ef5d0, sco_first.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn sco_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    covered!("cx13_9 scottish first records");
    play_off_update_staff_records(this, club, status, stage, &SCOTTISH_FIRST_PLAY_OFF) // cx13_9: the Scottish First Division
}

/// Comparer for `sco_first_update_staff_records_for_status` (`compare_play_off_staff_records`).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    compare_play_off_staff_records("sco_first_update_staff_records_for_status", this, club, status, stage, &SCOTTISH_FIRST_PLAY_OFF, || o(this, club, status, stage, a4, a5, a6))
}

/// The play-off group text with the competition: `format` with the club's short gender byte twice, the competition's
/// gender byte twice, the club's short name and the competition's name, set in `text` with the club's (0x7d5) and the
/// competition's (0x7d0) links.
pub(crate) unsafe fn play_off_group_text(this: League, club_id: u32, format: usize, text: usize) {
    let club = club_at(club_id as isize);
    let comp = this.competition().addr();
    let g = club.short_gender_name() as i8 as i32;
    let cg = read_u8(comp + competition::GENDER_NAME) as i8 as i32;
    story_lang!(PRAISE_TEXT_BUFFER, format, g, g, cg, cg, club.addr() + club::SHORT_NAME, comp + competition::NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
}

/// The fate news text of a Scottish First Division club. Outside the play-off group (stage other than 0) the common
/// text (Comp::build_fate_news_text `exe::comp_build_fate_news_text`, every argument as it came). In the play-off group
/// (1999 only): outcome 0 (in the top two), with a news item 0xa7aea4 "{club} have finished in the top two places of
/// the {competition} playoff group" (`play_off_group_text`), without one 0x987594 "{club} avoid relegation" for a
/// Premier club (`a7` is [`KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION`]) and 0x989c18 "{club} win promotion" for the
/// others; any other outcome, with a news item 0xa7ae20 "{club} have finished bottom of the {competition} playoff
/// group", without one 0x989d78 "{club} relegated" for a Premier club and 0x989d40 "{club} miss out on promotion" for
/// the others (`play_off_club_text`).
/// Original: `ScoFirst::build_fate_news_text` (0x7ef7c0, sco_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn sco_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    if stage as u8 != 0 {
        covered!("cx13_9 scottish first common text");
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, a7, a8, news, text); // cx13_9: the common text
    }
    let premier = a7 == read_u32(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION);
    let top_two = outcome as u8 == 0;
    if news != 0 {
        return play_off_group_text(League(this), club_id, if top_two { 0xa7aea4 } else { 0xa7ae20 }, text);
    }
    let format = match (top_two, premier) {
        (true, true) => 0x987594,  // "avoid relegation"
        (true, false) => 0x989c18, // "win promotion"
        (false, true) => 0x989d78, // "relegated"
        (false, false) => 0x989d40, // "miss out on promotion"
    };
    play_off_club_text(club_id, format, text);
}

/// Comparer for `sco_first_build_fate_news_text`: in the play-off group side by side with `compare_text_writer`; the
/// common text only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_sco_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    if stage as u8 != 0 || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| sco_first_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("sco_first_build_fate_news_text(club {}, outcome {}, news {})", club_id, outcome as u8, news));
}

/// The order of the Scottish Premier Division table after the split: a row whose club was in the bottom six at the
/// split (its position after round 33, the byte 0x20 of its position history `league_table_row::POSITION_HISTORY`,
/// signed, 6 or more) ranks 2, the others 1, and the order is the difference: each half stays apart.
/// Original: `sco_prm_compare_split_half` (0x7f3220, sco_prm.cpp, cdecl).
pub(crate) unsafe extern "C" fn sco_prm_compare_split_half(a: *const usize, b: *const usize) -> i32 {
    covered!("cx13_9 scottish split order");
    let half = |row: *const usize| if read_u8(read_usize(row as usize + league_table_row::POSITION_HISTORY) + 0x20) as i8 >= 6 { 2 } else { 1 }; // cx13_9: the split's bottom six
    half(a) - half(b)
}

compare_order!(compare_sco_prm_compare_split_half, sco_prm_compare_split_half, %%I2%%);
