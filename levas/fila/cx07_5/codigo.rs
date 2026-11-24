// ---------- cx07_5: the Olympic football results: a nation's fate text, the staff records and the progress codes (olympics.cpp) ----------

/// The Olympic football knockout stage (quarter-finals to final) after its four groups.
const OLYMPICS_KNOCKOUT: i8 = 3;

/// Writes the news text of a nation's fate in the Olympic football (`nations_tournament_fate_text` of cx07_3): in the
/// four groups (stage below 3) through "{nation} qualify for quarter finals" (0x98737c) or the group news, out the
/// group news; in the knockout (stage 3) the tie news or the Copa América templates.
/// Original: `Olympics::build_fate_news_text` (0x79fea0, olympics.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn olympics_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    covered!("cx07_5 olympics fate text");
    let knockout = OLYMPICS_KNOCKOUT; // cx07_5: the stage of the Olympic knockout texts
    nations_tournament_fate_text(this, club_id, outcome, stage, a4, fixture, a6, a8, news, text, knockout, 0x98737c);
}

/// Comparer for `olympics_build_fate_news_text`: the text-only paths side by side into two Texts
/// (`compare_text_writer`); the group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_olympics_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    if !nations_fate_text_only(outcome, stage, news, text, OLYMPICS_KNOCKOUT) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| olympics_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("olympics_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

/// rule: in the four Olympic football groups (stage below 3) a nation going through gets the qualification record
/// (weight 25) and one going out the failed-to-qualify record; in the knockout (stage 3) a nation going through gets
/// the qualification by round (weight 40 to 100 over three rounds), one going out the knocked-out record, and the
/// winner and runner-up the final records; third and fourth places get nothing (`nations_tournament_staff_records` of
/// cx07_3)
/// Original: `Olympics::update_staff_records_for_status` (0x7a0300, olympics.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn olympics_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("cx07_5 olympics staff records");
    let knockout = OLYMPICS_KNOCKOUT; // cx07_5: the stage of the Olympic knockout records
    nations_tournament_staff_records(League(this), club, outcome, stage, game, a6, knockout, 0x78, 3, &[1, 2])
}

/// Comparer for `olympics_update_staff_records_for_status`: the records go to the managers' history (they cannot run
/// twice); only the original, checking the return of 0.
pub(crate) unsafe extern "thiscall" fn compare_olympics_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("olympics_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// Stores a nation's progress in the Olympic football statistics (`group_championship_progress` of cx04_5): in the
/// groups (stage below 3) current 1→5, 2, 3→9, 4→13; best 1, 2→1, 3→9, 4→13; worst as current; afterwards as they
/// come.
/// Original: `Olympics::set_club_progress` (0x7a15d0, olympics.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn olympics_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx07_5 olympics progress");
    group_championship_progress(this, club, stage, current, best, worst, 3, [[5, 9, 9, 0xd], [1, 1, 9, 0xd], [5, 9, 9, 0xd]]); // cx07_5: the Olympic codes
}

/// Comparer for `olympics_set_club_progress`: side by side on the 3 bytes of the nation's entry
/// (`compare_group_championship_progress` of cx04_5).
pub(crate) unsafe extern "thiscall" fn compare_olympics_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_group_championship_progress("olympics_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || olympics_set_club_progress(this, club, stage, current, best, worst));
}
