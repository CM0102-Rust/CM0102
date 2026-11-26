use super::*;

/// Rule: the Polish turnover, led by the First Division: the season reopened; promotion and relegation along the chain;
/// the four best clubs of the lowest Polish division (the best twelve shuffled) up into the Second Division and its
/// bottom clubs down (`pol_first_promote_lower_division_clubs`); the First Division rebuilt for the next year (rules,
/// table, fixtures, order); its second group stage built again; the Second Division turned over; the predicted
/// positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1) (push 1 ->
/// thiscall, ret 4) and `pol_first_promote_lower_division_clubs` (direct, ecx = this); the table PolFirst::create_table
/// (0x7c9450) and the boxes of pol_first.cpp (0xa78598) at 0x1e7, 0x1ee, 0x1f4, 0x1fb; after the order,
/// PolFirst::create_second_group_stage (0x7c9900, thiscall, ret 0, not ported) and the Second Division's start_new_season
/// (virtual `comp_vtable::START_NEW_SEASON` on the object of `KEY_CLUB_COMP_POLISH_SECOND_DIVISION`, called without a
/// null check). The stages are not destroyed and the current stage is not reset (as the original). After a box: 0.
/// Original: `PolFirst::start_new_season` (0x7c9740, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
        call_on_league(0x7ca030, this);
    }, 0x7c9450, 0xa78598, [0x1e7, 0x1ee, 0x1f4, 0x1fb], || {
        covered!("mwd_29 polish first division turnover");
        call_on_league(0x7c9900, this);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Rule: at the Polish turnover, the four best Polish clubs of the lowest division (`KEY_CLUB_COMP_POLISH_LOWER_DIVISION`,
/// sorted by strength with the best twelve shuffled) go up to the Second Division, and the Second Division's bottom
/// `league::RELEGATED_COUNT` clubs go down to the lowest division; with fewer than 4 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box of pol_first.cpp at line 0x2f8 and
/// return) with the clubs whose nation (`club::NATION`) is Poland (`KEY_NATION_POLAND`) and whose division
/// (`club::DIVISION`) is the lowest one, in database order (a 16-bit index); each of them also gets
/// `club::LAST_DIVISION` = its division. With at least 4: `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4)
/// (cdecl), the first four moved to the Second Division (League::move_club_to_division(this, club, the Second
/// Division's record, 1)); then the rows from `club_count − relegated_count` (a word and a signed byte of the Second
/// Division) up to the last one go down (League::get_table_row, then League::relegate_club_to_division(this, club, the
/// lowest division's record, 1)). The list is freed.
/// Original: `pol_first_promote_lower_division_clubs` (0x7ca030, pol_first.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_promote_lower_division_clubs(this: usize) {
    let second = League(comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0xa78598, 0x2f8);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let nation = read_usize(club + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_POLAND) {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION) {
                write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                n = n.wrapping_add(1);
                write_usize(club + club::LAST_DIVISION, read_usize(club + club::DIVISION));
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 4 {
        covered!("mwd_29 polish lowest division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd29: the best twelve shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), 1);
        }
        let last = second.club_count() as i32;
        let mut i = last - (second.relegated_count() as i8) as i32;
        while i < last {
            let record = comp_record_of(KEY_CLUB_COMP_POLISH_LOWER_DIVISION);
            let club = read_usize(exe::league_get_table_row(second, i));
            exe::league_relegate_club_to_division(this, club, record, 1); // mwd29: the bottom clubs down
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `pol_first_start_new_season`: only the original runs (it draws and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("pol_first_start_new_season", League(this), || o(this))
}

/// Comparer for `pol_first_promote_lower_division_clubs`: only the original runs (it draws); with at least 4
/// candidates, the Second Division's bottom `relegated_count` clubs must end in the lowest division.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_promote_lower_division_clubs(this: usize) {
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let (nation, d) = (read_usize(c + club::NATION), read_usize(c + club::DIVISION));
        nation != 0 && read_usize(nation) == read_usize(KEY_NATION_POLAND) && d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION)
    }).count();
    let l = League(comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
    let mut planned: Vec<usize> = Vec::new();
    if candidates >= 4 && l.table() != 0 {
        let last = l.club_count() as i32;
        for i in (last - (l.relegated_count() as i8) as i32).max(0)..last {
            planned.push(read_usize(l.table() + i as usize * league_table_row::SIZE));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_POL_FIRST_PROMOTE_LOWER_DIVISION_CLUBS)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_POLISH_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("pol_first_promote_lower_division_clubs({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}

/// Rule: the next stage of the Polish First Division: past the last stage, the error box; otherwise the current stage
/// goes up by one; leaving stage 0 (reaching 1) the final stage is built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of pol_first.cpp at line
/// 0x24f and nothing else. Otherwise the current stage + 1; when the old one was 0, PolFirst::create_final_stage
/// (0x7c9b60, thiscall, ret 0, not ported).
/// Original: `PolFirst::advance_stage` (0x7c9ad0, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xa78598, 0x24f);
    }
    l.set_current_stage((stage + 1) as usize);
    covered!("mwd_31 polish first division next stage");
    if stage == 0 {
        call_on_league(0x7c9b60, l);
    }
}

/// Comparer for `pol_first_advance_stage` (`compare_advance_stage_building`: stage 1 builds).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_POL_FIRST_ADVANCE_STAGE);
    compare_advance_stage_building("pol_first_advance_stage", League(this), |s| s == 1, || o(this), || pol_first_advance_stage(this))
}

/// Rule: the Polish First Division: 16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal
/// difference, goals for; 4 in the promotion band, nobody down directly; the Second Division below (`pol_first_rules`).
/// Original: `PolFirst::set_up_rules` (0x7c9690, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_set_up_rules(this: usize) -> u32 {
    covered!("mwd_28 polish first division rules");
    eng_prm_apply_league_rules(League(this), &pol_first_rules())
}

/// Comparer for `pol_first_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("pol_first_set_up_rules", League(this), pol_first_rules(), || o(this), || pol_first_set_up_rules(this))
}

/// Side by side on the current stage of a league's advance_stage when it builds nothing (`builds`(new stage) false and
/// not at the last stage); otherwise only the original (a stage is built once).
pub(crate) unsafe fn compare_advance_stage_building(name: &str, this: League, builds: impl Fn(i32) -> bool, original: impl FnOnce(), copy: impl FnOnce()) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) || builds(stage + 1) {
        return original();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + league::CURRENT_STAGE, 4)], original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}) original={:x?} rust={:x?}", name, this.addr(), mo, mr));
}

/// The rules of the Polish First Division (16 clubs in two groups, then a championship and a relegation group): 2 legs,
/// 3 points a win and 1 a draw, league matches, +0xab = +0xdb = 0, national clubs; tie-breaks 4, 1, 2 (head to head,
/// goal difference, goals for; +0xc8 not written); 4 places in the promotion band (+0xbe), none to the play-offs, none
/// down directly (+0xc1 = 0: the relegation goes through the final stage); nothing above, the Second Division below;
/// +0x52 = 0; 7 substitutes, 3 substitutions; the rule bits +0xd9 = 8 (a word).
pub(crate) unsafe fn pol_first_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2),
        (0xbe, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x52, 1, 0),
        (0x20, 4, read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION) as u32), (0x49, 1, 7), (0x4a, 1, 3), (0xd9, 2, 8)]
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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_POL_FIRST_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    SCOREBOARD.record(r == plan, || format!("pol_first_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={} rust={}", this, club, status as u8, stage as u8 as i8, r as i32, plan as i32));
    r
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_POL_FIRST_BUILD_FATE_NEWS_TEXT);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_POL_FIRST_SET_CLUB_PROGRESS);
    compare_club_stage_status("pol_first_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || pol_first_set_club_progress(this, club, stage, current, best, worst))
}

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

/// A staff record of the end-of-stage statuses of the leagues with a championship group (Poland, Scotland).
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum GroupStageRecord {
    Champions,
    Relegation,
    Qualified,
}
