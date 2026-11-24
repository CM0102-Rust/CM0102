// ---------- cx13_8: the Scottish Premier Division's staff records, fate news text and club progress (sco_prm.cpp) ----------

/// The staff record of a Scottish Premier Division club by its status (no side effects): only in the main stage
/// (stage 0xff), 5 the title, 1 the qualified record (into the championship group of the split), 3 relegation. The
/// routine always returns 0.
pub(crate) fn sco_prm_staff_record(status: u8, stage: u8) -> Option<GroupStageRecord> {
    if stage != 0xff {
        return None;
    }
    match status {
        5 => Some(GroupStageRecord::Champions),
        1 => Some(GroupStageRecord::Qualified), // cx13_8: into the championship group
        3 => Some(GroupStageRecord::Relegation),
        _ => None,
    }
}

/// Updates the staff records by a club's status in the Scottish Premier Division (`sco_prm_staff_record`), into the
/// game's staff records (`G_STAFF_RECORDS`) with the competition record: the title
/// (`exe::staff_records_add_league_champions_record`), the qualified record
/// (`exe::staff_records_add_qualified_for_round_record` with 0x44b, 0, 0x28) or relegation
/// (`exe::staff_records_add_relegation_record`). Returns 0; the last three arguments are not read.
/// Original: `ScoPrm::update_staff_records_for_status` (0x7f3010, sco_prm.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn sco_prm_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let comp = League(this).competition().addr();
    let records = read_usize(G_STAFF_RECORDS);
    match sco_prm_staff_record(status as u8, stage as u8) {
        Some(GroupStageRecord::Champions) => exe::staff_records_add_league_champions_record(records, club, comp),
        Some(GroupStageRecord::Qualified) => {
            covered!("cx13_8 scottish championship group record");
            exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x44b, 0, 0x28);
        }
        Some(GroupStageRecord::Relegation) => exe::staff_records_add_relegation_record(records, club, comp),
        None => {}
    }
    0
}

/// Comparer for `sco_prm_update_staff_records_for_status`: only the original runs (it adds records); its result must be 0.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%)(this, club, status, stage, a4, a5, a6);
    SCOREBOARD.record(r == 0, || format!("sco_prm_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={}", this, club, status as u8, stage as u8 as i8, r as i32));
    r
}

/// The fate news text of a Scottish Premier Division club: in the main stage (0xff) outcome 1 (into the championship
/// group of the split) gets `championship_group_text` (only the shared buffer); otherwise, in stages −1, 0 and 1, the
/// common text (Comp::build_fate_news_text `exe::comp_build_fate_news_text` with the stage and a7 = 0, the rest as
/// they came); other stages nothing.
/// Original: `ScoPrm::build_fate_news_text` (0x7f3080, sco_prm.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn sco_prm_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    if s == -1 && outcome as u8 == 1 {
        covered!("cx13_8 scottish championship group text");
        return championship_group_text(League(this), club_id, news); // cx13_8: the split text
    }
    if (-1..=1).contains(&s) {
        exe::comp_build_fate_news_text(this, club_id, outcome, s as i32, a4, a5 as usize, a6, 0, a8, news, text);
    }
}

/// Comparer for `sco_prm_build_fate_news_text`: on the split text side by side over the shared buffer
/// (`PRAISE_TEXT_BUFFER`, 0x200 bytes); through the common text only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    if !(stage as u8 == 0xff && outcome as u8 == 1) {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(PRAISE_TEXT_BUFFER, 0x200)], || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        || sco_prm_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text));
    SCOREBOARD.record(same, || format!("sco_prm_build_fate_news_text(club {}, news {}) original={:?} rust={:?}", club_id, news, mo, mr));
}

/// The progress codes of a Scottish Premier Division club, into its statistics entry (`exe::comp_stats_find_club_entry`,
/// not checked for null), in the order current, best, worst: in 1999 a best of 10 becomes 9; otherwise as they came.
/// The 1999 branch also tests the worst against 10 but writes it unchanged either way [inference: a mapping of the
/// worst that was meant and never written].
/// Original: `ScoPrm::set_club_progress` (0x7f31c0, sco_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn sco_prm_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    let l = League(this);
    let e = CompStatsClub(exe::comp_stats_find_club_entry(l.comp_stats(), club));
    covered!("cx13_8 scottish progress");
    let mut b = best as u8;
    if l.year() == 0x7cf && b == 0xa {
        b = 9;
    }
    e.set_current_position(current as u8); // cx13_8: the current code
    e.set_best_position(b);
    e.set_worst_position(worst as u8);
}

/// Comparer for `sco_prm_set_club_progress`: side by side on the three codes (`compare_club_stage_status`).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_club_stage_status("sco_prm_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || sco_prm_set_club_progress(this, club, stage, current, best, worst))
}
