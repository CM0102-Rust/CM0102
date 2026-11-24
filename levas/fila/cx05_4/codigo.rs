// ---------- cx05_4: the Asian Cup results: a nation's fate text, the staff records and the progress codes (asia_nations.cpp) ----------

/// Writes the news text of a nation's fate in the Asian Cup.
///
/// In the group stages (stage below 2, signed): through (outcome 6) with `news` the shared group news
/// (`exe::comp_util_write_group_qualify_news`(club, a4, competition, 6, news, text), cdecl), without it "{nation}
/// qualify for quarter finals" (0x98737c, `group_qualify_text` of cx04_5); out (0xfe) the group news with −2; any other
/// outcome, nothing. In the knockout (stage 2): with `news` the tie result news
/// (`exe::comp_util_write_cup_tie_result_news`(club, fixture, a8, outcome, a6, text), cdecl), without it the Copa
/// América templates (`copa_america_fate_template`: win, runners-up, third, fourth, knocked out, any other "through to
/// next round") with the nation's and the competition's short names and links (`european_cup_fate_text`). From stage 3
/// on, nothing.
/// Original: `AsiaNations::build_fate_news_text` (0x40f130, asia_nations.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn asia_nations_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    let comp = League(this).competition().addr();
    let knockout = 2i8; // cx05_4: the Asian Cup knockout stage
    if s < knockout {
        if d == 6 {
            if news != 0 {
                exe::comp_util_write_group_qualify_news(club_id, a4, comp, 6, news, text);
            } else {
                covered!("cx05_4 asian cup quarter finals");
                group_qualify_text(club_id, 0x98737c, text); // "{nation} qualify for quarter finals"
            }
        } else if d == 0xfe {
            exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, news, text); // cx05_4: out in the groups
        }
        return;
    }
    if s != knockout {
        return;
    }
    if news != 0 {
        exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
    } else {
        covered!("cx05_4 asian cup knockout fate");
        european_cup_fate_text(text, club_id, copa_america_fate_template(d), read_u8(comp + competition::SHORT_GENDER_NAME), comp + competition::SHORT_NAME, comp);
    }
}

/// Comparer for `asia_nations_build_fate_news_text`: the text-only paths (through in the groups or any knockout fate,
/// without `news`) side by side into two Texts (`compare_text_writer`); the group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    let (s, d) = (stage as u8 as i8, outcome as u8);
    if text == 0 || news != 0 || !((s < 2 && d == 6) || s == 2) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| asia_nations_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("asia_nations_build_fate_news_text(club {}, outcome {}, stage {})", club_id, d, s));
}

/// rule: in the two Asian Cup group stages (stage below 2) a nation going through gets the qualification record
/// (weight 25) and one going out the failed-to-qualify record; in the knockout (stage 2) a nation going through gets
/// the qualification by round (weight 40 to 100), one going out the knocked-out record, and the winner, runner-up and
/// third place the final records; the fourth place gets nothing (`national_tournament_staff_records` with the knockout
/// at 2, the Copa América's body)
/// Original: `AsiaNations::update_staff_records_for_status` (0x40f5a0, asia_nations.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn asia_nations_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("cx05_4 asian cup staff records");
    let knockout = 2i8; // cx05_4: the stage of the Asian Cup knockout records
    national_tournament_staff_records(1, League(this), club, outcome, stage, game, a6, knockout)
}

/// Comparer for `asia_nations_update_staff_records_for_status`: the records go to the managers' history (they cannot
/// run twice); only the original, checking the return of 0.
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("asia_nations_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// Stores a nation's progress in the Asian Cup statistics (`group_championship_progress` of cx04_5): in the group
/// stages (below 2) current 1→1, 2→3, 3→5, 4→11; best 1, 2, 3→1, 4→9; worst 1, 2→5, 3→9, 4→11; afterwards as they
/// come.
/// Original: `AsiaNations::set_club_progress` (0x410560, asia_nations.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn asia_nations_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx05_4 asian cup progress");
    group_championship_progress(this, club, stage, current, best, worst, 2, [[1, 3, 5, 0xb], [1, 1, 1, 9], [5, 5, 9, 0xb]]); // cx05_4: the Asian Cup codes
}

/// Comparer for `asia_nations_set_club_progress`: side by side on the 3 bytes of the nation's entry
/// (`compare_group_championship_progress` of cx04_5).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_group_championship_progress("asia_nations_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || asia_nations_set_club_progress(this, club, stage, current, best, worst));
}
