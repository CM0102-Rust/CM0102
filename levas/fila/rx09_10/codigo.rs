// ---------- rx09_10: the Swedish Second Division staff records, fate news text and club list (swe_second.cpp) ----------

/// The section of a Swedish Second Division club by the id of its reserve division (`SWEDISH_SECOND_SECTIONS`): the
/// North section is the league's own table, the others the stages 0 to 4. None when the id is none of the six.
pub(crate) unsafe fn swedish_second_section_of(l: League, reserve: usize) -> Option<League> {
    SWEDISH_SECOND_SECTIONS.iter().position(|&s| read_usize(reserve) == read_usize(s)).map(|k| if k == 0 { l } else { League(read_usize(l.stages() + 4 * (k - 1))) })
}

/// The plan of `swe_second_update_staff_records_for_status` for a club with `status` (its byte) at `stage` (signed):
/// - the sections (below 5): status 1 adds the qualified record; the result is 0;
/// - the promotion play-off (5): no reserve division, nothing and 0; the section by `swedish_second_section_of` (none:
///   the box at line 0x38b and −1), the club's row in it (League::find_club_row `exe::league_find_club_row`(section,
///   club, 0); none: −1); status 1 writes 0 in the row with the promotion record, any other 0xfe; the result 0;
/// - the relegation play-off (6): no reserve division or a division other than this league's record: −1; the section
///   (none: the box at line 0x3bb and −1), the row (none: −1); status 1 writes 0xfe, any other 3 with the relegation
///   record; the result 0;
/// - later stages: nothing and 0.
pub(crate) unsafe fn swedish_second_record_plan(l: League, club: Club, status: u8, stage: i8) -> CategoryBRecordPlan {
    let mut plan = CategoryBRecordPlan { error_line: None, row: None, record: None, result: 0 };
    if stage < 5 {
        if status == 1 {
            plan.record = Some(CategoryBRecord::Qualified);
        }
        return plan;
    }
    if stage > 6 {
        return plan;
    }
    let reserve = club.reserve_division().addr();
    if reserve == 0 {
        if stage == 6 {
            plan.result = u32::MAX;
        }
        return plan;
    }
    if stage == 6 && read_usize(club.addr() + club::DIVISION) != l.competition().addr() {
        plan.result = u32::MAX;
        return plan;
    }
    let Some(section) = swedish_second_section_of(l, reserve) else {
        plan.error_line = Some(if stage == 5 { 0x38b } else { 0x3bb });
        plan.result = u32::MAX;
        return plan;
    };
    let row = exe::league_find_club_row(section, club, 0);
    if row == 0 {
        plan.result = u32::MAX;
        return plan;
    }
    match (stage, status) {
        (5, 1) => {
            plan.row = Some((row, 0)); // rx09_10: the promotion play-off winner's row
            plan.record = Some(CategoryBRecord::Promotion);
        }
        (5, _) => plan.row = Some((row, 0xfe)),
        (_, 1) => plan.row = Some((row, 0xfe)),
        _ => {
            plan.row = Some((row, 3));
            plan.record = Some(CategoryBRecord::Relegation);
        }
    }
    plan
}

/// Updates the staff records by a club's status in the Swedish Second Division and marks its row in its section
/// (`swedish_second_record_plan`: the error box, the row status, then the record, with this league's competition).
/// The last three arguments are not read.
/// Original: `SweSecond::update_staff_records_for_status` (0x892ee0, swe_second.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn swe_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let plan = swedish_second_record_plan(l, Club(club), status as u8, stage as u8 as i8);
    if let Some(line) = plan.error_line {
        show_error_at(SWEDISH_SECOND_SOURCE, line);
    }
    if let Some((row, s)) = plan.row {
        covered!("rx09_10 swedish second row status");
        write_u8(row + league_table_row::STATUS, s);
    }
    let records = read_usize(G_STAFF_RECORDS);
    let comp = l.competition().addr();
    match plan.record {
        Some(CategoryBRecord::Qualified) => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x458, 0, 0x28),
        Some(CategoryBRecord::Promotion) => exe::staff_records_add_promotion_record(records, club, comp, 0x64),
        Some(CategoryBRecord::Relegation) => exe::staff_records_add_relegation_record(records, club, comp),
        None => {}
    }
    plan.result
}

/// Comparer for `swe_second_update_staff_records_for_status`: only the original runs (it adds records); its result and
/// the status of the planned row against the plan.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let plan = swedish_second_record_plan(League(this), Club(club), status as u8, stage as u8 as i8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%)(this, club, status, stage, a4, a5, a6);
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == plan.result && row_ok, || format!("swe_second_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={:#x} plan={:?}", this, club,
        status as u8, stage as u8 as i8, r, plan));
    r
}

/// True when `swe_second_build_fate_news_text` writes the text itself (not the common text nor the cup news).
pub(crate) fn swedish_second_writes_own_text(stage: i8, outcome: u8, news: u32) -> bool {
    match stage {
        5 => news == 0,
        6 => news == 0 && matches!(outcome, 1 | 2 | 0xff),
        _ => false,
    }
}

/// The fate news text of a Swedish Second Division club (thiscall, ret 0x28; same arguments as
/// Comp::build_fate_news_text):
/// - the sections (stage below 5): the common text (`exe::comp_build_fate_news_text`, a7 = 0, the rest as they came);
/// - the promotion play-off (5): with a news item the cup tie news (`exe::comp_util_write_cup_tie_result_news`(club, a5,
///   a8, outcome, a6, text)); without one, outcome 1 gives 0x989c18 "{club} win promotion" and any other 0x987528
///   "{club} miss out on promotion" (`play_off_club_text`);
/// - the relegation play-off (6): with a news item the cup tie news; without one, outcome 1 gives 0x987594 "{club}
///   avoid relegation", outcomes 2 and 0xff 0x987564 "{club} relegated", any other the cup win or defeat news
///   (`exe::comp_util_write_cup_win_or_defeat_news`(club, a5, outcome, text));
/// - later stages: nothing.
/// Original: `SweSecond::build_fate_news_text` (0x893160, swe_second.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn swe_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    if s < 5 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    }
    if s != 5 && s != 6 {
        return;
    }
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    if s == 5 {
        covered!("rx09_10 swedish second promotion text");
        return play_off_club_text(club_id, if outcome as u8 == 1 { 0x989c18 } else { 0x987528 }, text); // rx09_10: win / miss out on promotion
    }
    match outcome as u8 {
        1 => play_off_club_text(club_id, 0x987594, text), // "{club} avoid relegation"
        2 | 0xff => {
            covered!("rx09_10 swedish second relegated text");
            play_off_club_text(club_id, 0x987564, text); // "{club} relegated"
        }
        _ => exe::comp_util_write_cup_win_or_defeat_news(club_id, a5, outcome, text),
    }
}

/// Comparer for `swe_second_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the common text and the cup news only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_swe_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    let s = stage as u8 as i8;
    if !swedish_second_writes_own_text(s, outcome as u8, news) || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| swe_second_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("swe_second_build_fate_news_text(club {}, outcome {:#x}, stage {}, news {})", club_id, outcome as u8, s, news));
}

/// The clubs of the Swedish Second Division: 72 slots (the six sections of 12), filled by `grouped_league_club_array`
/// with the section stages 0..4, each copied for its own club count (only the North and the last section end up in the
/// list: the defect described there); out of memory: the box at line 0x464.
/// Original: `SweSecond::get_club_array` (0x893320, swe_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn swe_second_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    grouped_league_club_array(League(this), list, n, 0x48, 5, false, SWEDISH_SECOND_SOURCE, 0x464);
}

/// Comparer for `swe_second_get_club_array` (`compare_grouped_league_club_array`).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(%%I2%%);
    compare_grouped_league_club_array("swe_second_get_club_array", League(this), list, n, 0x48, 5, false, || o(this, list, n));
}
