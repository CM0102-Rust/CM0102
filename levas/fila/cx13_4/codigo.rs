// ---------- cx13_4: the Polish First Division's staff records, fate news text and club progress (pol_first.cpp) ----------

/// The staff record of a Polish First Division club by its status at the end of a stage (no side effects): in the
/// group stages (signed `stage` below 1) status 1 (into the championship group) is a qualified record; in the final
/// stage (stage 1) status 5 is the title and 3 relegation. Returns the record (`None` for nothing) and the routine's
/// result: 0 when a record is added (and in the final stage for any other status), −1 otherwise.
pub(crate) fn pol_first_staff_record(status: u8, stage: i8) -> (Option<GroupStageRecord>, u32) {
    if stage < 1 {
        if status == 1 {
            return (Some(GroupStageRecord::Qualified), 0);
        }
        return (None, 0xffff_ffff);
    }
    if stage != 1 {
        return (None, 0);
    }
    match status {
        5 => (Some(GroupStageRecord::Champions), 0),
        3 => (Some(GroupStageRecord::Relegation), 0),
        _ => (None, 0xffff_ffff),
    }
}

/// A staff record of the end-of-stage statuses of the leagues with a championship group (Poland, Scotland).
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum GroupStageRecord {
    Champions,
    Relegation,
    Qualified,
}

/// Updates the staff records by a club's status in the Polish First Division (`pol_first_staff_record`): the title
/// (Staff_Records::add_league_champions_record `exe::staff_records_add_league_champions_record`(records, club, the
/// competition record)), relegation (`exe::staff_records_add_relegation_record`, the same arguments) or the qualified
/// record (`exe::staff_records_add_qualified_for_round_record`(records, club, competition, 0x44b, 0, 0x28)), into the
/// game's staff records (`G_STAFF_RECORDS`). The last three arguments are not read.
/// Original: `PolFirst::update_staff_records_for_status` (0x7ca2a0, pol_first.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn pol_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let comp = League(this).competition().addr();
    let records = read_usize(G_STAFF_RECORDS);
    let (record, result) = pol_first_staff_record(status as u8, stage as u8 as i8);
    match record {
        Some(GroupStageRecord::Champions) => exe::staff_records_add_league_champions_record(records, club, comp),
        Some(GroupStageRecord::Relegation) => exe::staff_records_add_relegation_record(records, club, comp),
        Some(GroupStageRecord::Qualified) => {
            covered!("cx13_4 polish championship group record");
            exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x44b, 0, 0x28); // cx13_4: into the championship group
        }
        None => {}
    }
    result
}

/// Comparer for `pol_first_update_staff_records_for_status`: only the original runs (it adds records); its result
/// against the plan (`pol_first_staff_record`).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (_, plan) = pol_first_staff_record(status as u8, stage as u8 as i8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%)(this, club, status, stage, a4, a5, a6);
    SCOREBOARD.record(r == plan, || format!("pol_first_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={} rust={}", this, club, status as u8, stage as u8 as i8, r as i32, plan as i32));
    r
}

/// The text of a club that goes into the championship group of a Polish or Scottish league (the shared buffer
/// `PRAISE_TEXT_BUFFER`): with `news`, 0x9b54d0 "{club} have qualified for the championship group stage of the
/// {competition}" (the club's short gender byte twice, the competition's twice, the club's and the competition's short
/// names); without, 0x9b549c "{club} qualify for championship group" (the club's gender byte twice and its short
/// name). The text is only written to the buffer: the Text object is not touched.
pub(crate) unsafe fn championship_group_text(this: League, club_id: u32, news: u32) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    if news != 0 {
        let comp = this.competition().addr();
        let cg = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        story_lang!(PRAISE_TEXT_BUFFER, 0x9b54d0, g, g, cg, cg, club.addr() + club::SHORT_NAME, comp + competition::SHORT_NAME);
    } else {
        story_lang!(PRAISE_TEXT_BUFFER, 0x9b549c, g, g, club.addr() + club::SHORT_NAME); // "qualify for championship group"
    }
}

/// The fate news text of a Polish First Division club: in the group stages (signed `stage` below 1) a club with
/// outcome 1 (into the championship group) gets `championship_group_text`; in the final stage (stage 1) the common
/// text (Comp::build_fate_news_text `exe::comp_build_fate_news_text` with the stage 1 and a7 = 0, the other arguments
/// as they came); any other stage, nothing.
/// Original: `PolFirst::build_fate_news_text` (0x7ca320, pol_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn pol_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    if s < 1 && outcome as u8 == 1 {
        covered!("cx13_4 polish championship group text");
        championship_group_text(League(this), club_id, news); // cx13_4: the championship group text
    }
    if s == 1 {
        exe::comp_build_fate_news_text(this, club_id, outcome, 1, a4, a5 as usize, a6, 0, a8, news, text);
    }
}

/// Comparer for `pol_first_build_fate_news_text`: in the group stages, side by side over the shared buffer
/// (`PRAISE_TEXT_BUFFER`, 0x200 bytes); in the final stage only the original (the common text writes the Text object,
/// which takes its text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_pol_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    if stage as u8 as i8 == 1 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(PRAISE_TEXT_BUFFER, 0x200)], || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        || pol_first_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text));
    SCOREBOARD.record(same, || format!("pol_first_build_fate_news_text(club {}, outcome {}, stage {}, news {}) original={:?} rust={:?}", club_id, outcome as u8, stage as u8 as i8, news, mo, mr));
}

/// The progress codes of a Polish First Division club in its statistics entry (Comp_Stats::find_club_entry
/// `exe::comp_stats_find_club_entry`, not checked for null). In the group stages (signed `stage` below 1) the two groups
/// of eight are folded into one ladder: the best code becomes 1 when it is below 5 (a championship group place within
/// reach) and 9 otherwise, the worst 8 when below 5 and 16 otherwise, and the current one doubles; in the final stage
/// they go as they came.
/// Original: `PolFirst::set_club_progress` (0x7ca3f0, pol_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn pol_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let e = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < 1 {
        covered!("cx13_4 polish group progress");
        e.set_best_position(if b < 5 { 1 } else { 9 }); // cx13_4: the championship group band
        e.set_current_position(c.wrapping_shl(1));
        e.set_worst_position(if w < 5 { 8 } else { 0x10 });
        return;
    }
    e.set_current_position(c);
    e.set_best_position(b);
    e.set_worst_position(w);
}

/// Comparer for `pol_first_set_club_progress`: side by side on the three codes of the club's entry
/// (`compare_club_stage_status`).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_club_stage_status("pol_first_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || pol_first_set_club_progress(this, club, stage, current, best, worst))
}
