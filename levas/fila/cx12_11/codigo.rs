// ---------- cx12_11: the Norwegian First Division staff records, fate news text and club progress (nor_first.cpp) ----------

/// The Norwegian First Division and Premier Division (`FirstDivisionPlayOff`): the relegation of a Premier club that loses
/// the play-off names the Norwegian Premier Division; the league band starts at 3 and a Premier club's play-off codes 1
/// and 2 become 11 and 14.
pub(crate) const NORWEGIAN_PLAY_OFF_DIVISIONS: FirstDivisionPlayOff = FirstDivisionPlayOff {
    premier: KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION,
    first: KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION,
    relegated_from: KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION,
    band: 3, // cx12_11: the Norwegian play-off band
    premier_codes: [0xb, 0xe],
};

/// Updates the staff records by a club's outcome in the Norwegian First Division and its play-off against the Premier
/// Division (`first_division_update_staff_records` with `NORWEGIAN_PLAY_OFF_DIVISIONS`).
/// Original: `NorFirst::update_staff_records_for_status` (0x791080, nor_first.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn nor_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    covered!("cx12_11 norwegian first division records");
    first_division_update_staff_records(this, club, status, stage, &NORWEGIAN_PLAY_OFF_DIVISIONS)
}

/// Writes the news text of a club's fate in the Norwegian First Division and its play-off (`first_division_fate_news_text`
/// with `NORWEGIAN_PLAY_OFF_DIVISIONS`).
/// Original: `NorFirst::build_fate_news_text` (0x791270, nor_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn nor_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, division: u32, a8: u32, news: u32, text: usize) {
    first_division_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, division, a8, news, text, &NORWEGIAN_PLAY_OFF_DIVISIONS)
}

/// The club's progress codes in the Norwegian First Division (`first_division_set_club_progress` with
/// `NORWEGIAN_PLAY_OFF_DIVISIONS`: the league band 3, best 3 → 2 and worst 3 → 4; a Premier club in the play-off 1 → 11
/// and 2 → 14, in the Premier's statistics).
/// Original: `NorFirst::set_club_progress` (0x7913f0, nor_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn nor_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx12_11 norwegian first division progress");
    first_division_set_club_progress(this, club, stage, current, best, worst, &NORWEGIAN_PLAY_OFF_DIVISIONS)
}

/// Comparer for `nor_first_update_staff_records_for_status` (`compare_first_division_update_staff_records`).
pub(crate) unsafe extern "thiscall" fn compare_nor_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    compare_first_division_update_staff_records("nor_first_update_staff_records_for_status", this, club, status, stage, &NORWEGIAN_PLAY_OFF_DIVISIONS, || o(this, club, status, stage, a4, a5, a6))
}

/// Comparer for `nor_first_build_fate_news_text` (`compare_first_division_fate_news_text`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_nor_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, division: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I1%%);
    compare_first_division_fate_news_text("nor_first_build_fate_news_text", this, club_id, outcome, stage, a4, fixture, a6, division, a8, news, text, &NORWEGIAN_PLAY_OFF_DIVISIONS,
        |t| o(this, club_id, outcome, stage, a4, fixture, a6, division, a8, news, t));
}

/// Comparer for `nor_first_set_club_progress` (`compare_first_division_club_progress`).
pub(crate) unsafe extern "thiscall" fn compare_nor_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_first_division_club_progress("nor_first_set_club_progress", this, club, stage, &NORWEGIAN_PLAY_OFF_DIVISIONS, || o(this, club, stage, current, best, worst),
        || nor_first_set_club_progress(this, club, stage, current, best, worst));
}
