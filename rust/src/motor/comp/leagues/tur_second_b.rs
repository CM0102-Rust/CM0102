use super::*;

/// Rule: the Turkish second division category B turnover (called by the First Division's): the season reopened, last
/// season's stages destroyed, the next year; the rules, the table, the order and the fixtures (in this order); the
/// four group stages built again; the predicted positions and the officials (no board expectations).
///
/// `league::SEASON_CLOSED` = 0; League::free_table (`exe::league_free_table`(0)); the round dates freed and cleared;
/// with statistics, `exe::comp_stats_archive_season_and_reset`(1); with a current stage ≥ 0, the stages 0..=current
/// (re-read on each step) destroyed by their virtual 0 with 1; the year + 1 and the current stage −1. Then, each with
/// the box of tur_second_b.cpp (0xaa18b8) at its line and 0 returned when it returns 0: the rules (virtual
/// `comp_vtable::SET_UP_RULES`, 0x233), TurSecondB::create_table (0x9002c0, thiscall, ret 0; 0x23a), League::sort_table
/// (`exe::league_sort_table`(0), 0x241) and League::create_fixtures (0x247). Then TurSecondB::create_group_stage(i)
/// (0x900780, push i -> thiscall, ret 4) for i = 0..3, the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and OfficialsManager::renew_officials on `G_OFFICIALS_MANAGER` with the
/// record, whose eax is returned.
/// Original: `TurSecondB::start_new_season` (0x900500, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(1));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let file = 0xaa18b8;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x233);
        return 0;
    }
    if exe::tursecondb_create_table(this) == 0 {
        show_error_at(file, 0x23a);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x241);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x247);
        return 0;
    }
    covered!("mwd_31 turkish category b turnover");
    for i in 0..4 {
        exe::tursecondb_create_group_stage(this, i);
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Rule: the next stage of the Turkish second division category B: past the last stage, the error box; otherwise the
/// current stage goes up by one; on reaching stage 4 the promotion group and the classification groups are built, on
/// reaching stage 10 the play-off.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of tur_second_b.cpp at line
/// 0x265 and nothing else. Otherwise the current stage + 1; at 4, TurSecondB::create_promotion_group (0x900ab0) and
/// TurSecondB::create_classification_groups (0x900d00); at 10, TurSecondB::create_playoff_stage (0x901040) (all
/// thiscall, ret 0, not ported).
/// Original: `TurSecondB::advance_stage` (0x9006c0, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xaa18b8, 0x265);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("mwd_31 turkish category b next stage");
    if stage == 4 { // mwd31: the promotion and classification groups after the four groups
        call_on_league(0x900ab0, l);
        call_on_league(0x900d00, l);
    } else if stage == 10 {
        call_on_league(0x901040, l);
    }
}

/// Comparer for `tur_second_b_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_B_START_NEW_SEASON);
    compare_state_league_new_season("tur_second_b_start_new_season", League(this), || o(this))
}

/// Comparer for `tur_second_b_advance_stage` (`compare_advance_stage_building`: stages 4 and 10 build).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TUR_SECOND_B_ADVANCE_STAGE);
    compare_advance_stage_building("tur_second_b_advance_stage", League(this), |s| s == 4 || s == 10, || o(this), || tur_second_b_advance_stage(this))
}

/// Rule: the Turkish second division category B: 3 points a win and 1 a draw; tie-breaks goal difference, goals for; 2
/// to the play-offs; category A above (`tur_second_b_rules`).
/// Original: `TurSecondB::set_up_rules` (0x900460, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_set_up_rules(this: usize) -> u32 {
    covered!("mwd_28 turkish category b rules");
    eng_prm_apply_league_rules(League(this), &tur_second_b_rules())
}

/// Comparer for `tur_second_b_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_B_SET_UP_RULES);
    eng_prm_compare_league_rules("tur_second_b_set_up_rules", League(this), tur_second_b_rules(), || o(this), || tur_second_b_set_up_rules(this))
}

/// The rules of the Turkish second division category B (five regional groups, then promotion and classification
/// groups): 2 legs, 3 points a win and 1 a draw, league matches, national clubs; tie-breaks 1, 2 (goal difference,
/// goals for; +0xc7 = 0 ends); none up directly, 2 to the play-offs (+0xbf), none down; category A above, nothing
/// below; 7 substitutes, 3 substitutions; +0xd9 = 2 (a word). +0xab, +0xdb, +0xc8 and +0x52 are not written.
pub(crate) unsafe fn tur_second_b_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 0), (0xbf, 1, 2),
        (0xc0, 1, 0), (0xc1, 1, 0), (0x4a, 1, 3), (0xd9, 2, 2), (0x1c, 4, read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A) as u32),
        (0x20, 4, 0xffff_ffff), (0x49, 1, 7)]
}

/// The clubs of the Turkish second division category B: 50 slots (the five groups of ten), filled by
/// `grouped_league_club_array` with the group stages 0..3 (only the first group and the last group stage end up in
/// the list, the defect described there).
/// Original: `TurSecondB::get_club_array` (0x901800, tur_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn tur_second_b_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    grouped_league_club_array(League(this), list, n, 0x32, 4, false, TURKISH_CATEGORY_B_SOURCE, 0x4c5);
}

/// Comparer for `tur_second_b_get_club_array` (`compare_grouped_league_club_array`).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_TUR_SECOND_B_GET_CLUB_ARRAY);
    compare_grouped_league_club_array("tur_second_b_get_club_array", League(this), list, n, 0x32, 4, false, || o(this, list, n));
}

/// The predicted positions of the Turkish second division category B: each of the five groups (the first group in
/// the league's table, the other four in the group stages 0..3) sorted by strength, and the club in place k (0 to 9)
/// gets 5k + 1 whatever its group (`grouped_league_predicted_positions`; a list of 0xc8 bytes, boxes at lines 0x4ec and
/// 0x4f4).
/// Original: `TurSecondB::set_predicted_positions` (0x901950, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_set_predicted_positions(this: usize) {
    grouped_league_predicted_positions(League(this), 4, 0xc8, TURKISH_CATEGORY_B_SOURCE, (0x4ec, 0x4f4), |_, k| k.wrapping_mul(5).wrapping_add(1)); // cx14_3: 5k + 1
}

/// Comparer for `tur_second_b_set_predicted_positions` (`compare_grouped_predicted_positions`).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TUR_SECOND_B_SET_PREDICTED_POSITIONS);
    compare_grouped_predicted_positions("tur_second_b_set_predicted_positions", League(this), 4, || o(this), || tur_second_b_set_predicted_positions(this));
}

/// Stores a category B club's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the season codes by `turkish_category_b_progress`.
/// Original: `TurSecondB::set_club_progress` (0x901ac0, tur_second_b.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn tur_second_b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx14_3 turkish category b progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = turkish_category_b_progress(stage as u8 as i8, current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_worst_position(w);
    entry.set_current_position(c);
}

/// Comparer for `tur_second_b_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_TUR_SECOND_B_SET_CLUB_PROGRESS);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || tur_second_b_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("tur_second_b_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}

/// Updates the staff records by a club's status in the Turkish second division category B and marks its row in its
/// first phase group (`category_b_record_plan`: the error box, the row status, then the record; the competition is
/// this league's, `league::COMPETITION`). The last three arguments are not read.
/// Original: `TurSecondB::update_staff_records_for_status` (0x9012d0, tur_second_b.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn tur_second_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let plan = category_b_record_plan(l, Club(club), status as u8, stage as u8 as i8);
    if let Some(line) = plan.error_line {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, line);
    }
    if let Some((row, s)) = plan.row {
        covered!("cx14_4 category b row status");
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

/// Comparer for `tur_second_b_update_staff_records_for_status`: only the original runs (it adds records); its result
/// and the status of the planned row against the plan.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let plan = category_b_record_plan(League(this), Club(club), status as u8, stage as u8 as i8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_TUR_SECOND_B_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == plan.result && row_ok, || format!("tur_second_b_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={:#x} plan={:?}", this, club,
        status as u8, stage as u8 as i8, r, plan));
    r
}

/// The fate news text of a category B club (thiscall, ret 0x28; same arguments as Comp::build_fate_news_text).
/// - promotion group (stage 4): with a news item the common text (`exe::comp_build_fate_news_text` with the stage
///   pushed as the constant 4 and a7 = 0); without one, outcome 0 or 5 gives 0x989c18 "{club} win promotion"
///   (`play_off_club_text`) and any other outcome nothing;
/// - classification groups (5 to 9): outcome 1 gives, with a news item, 0xaa191c "{club} have qualified for the
///   playoff of the {competition}" and, without one, 0xaa18f4 "{club} qualify for playoff", both with the club's and
///   the competition's links (`club_and_competition_short_text`); outcome 3 the common text (a7 = 0); any other
///   nothing;
/// - first phase (below 4) and the play-off and later (10 on): the common text (a7 = 0, the rest as they came).
/// Original: `TurSecondB::build_fate_news_text` (0x9015f0, tur_second_b.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn tur_second_b_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    let common = |stage: u32| exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    if s == 4 {
        if news != 0 {
            return common(4);
        }
        if outcome as u8 == 0 || outcome as u8 == 5 {
            covered!("cx14_4 category b promotion text");
            play_off_club_text(club_id, 0x989c18, text); // "{club} win promotion"
        }
    } else if (5..10).contains(&s) {
        match outcome as u8 {
            1 => {
                covered!("cx14_4 category b play-off text");
                let comp = League(this).competition().addr();
                club_and_competition_short_text(club_id, comp, if news != 0 { 0xaa191c } else { 0xaa18f4 }, news != 0, text); // cx14_4: qualified for the play-off
            }
            3 => common(stage),
            _ => {}
        }
    } else {
        common(stage);
    }
}

/// Comparer for `tur_second_b_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the common text only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_TUR_SECOND_B_BUILD_FATE_NEWS_TEXT);
    let s = stage as u8 as i8;
    let own = (s == 4 && news == 0 && (outcome as u8 == 0 || outcome as u8 == 5)) || ((5..10).contains(&s) && outcome as u8 == 1);
    if !own || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| tur_second_b_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("tur_second_b_build_fate_news_text(club {}, outcome {}, stage {}, news {})", club_id, outcome as u8, s, news));
}

/// Records the final positions of the Turkish second division category B: each club of `category_b_final_places`
/// gets this league's competition as its last division (`club::LAST_DIVISION`) and its place
/// (`.set_last_position()`), with no check of the club's division.
/// Original: `TurSecondB::record_final_positions` (0x901ba0, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_record_final_positions(this: usize) {
    covered!("cx14_4 category b final positions");
    let l = League(this);
    for (c, p) in category_b_final_places(l) {
        write_usize(c.addr() + club::LAST_DIVISION, l.competition().addr());
        c.set_last_position(p);
    }
}

/// Comparer for `tur_second_b_record_final_positions`: side by side over the last division and the last position of
/// every club it records.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_record_final_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TUR_SECOND_B_RECORD_FINAL_POSITIONS);
    let regions: Vec<(usize, usize)> = category_b_final_places(League(this)).iter().map(|(c, _)| (c.addr() + club::LAST_DIVISION, 5)).collect();
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || tur_second_b_record_final_positions(this));
    SCOREBOARD.record(same, || format!("tur_second_b_record_final_positions({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Builds category B group `group` + 2 as stage `group` (0 to 3): its round table (`stage_round_table`), a list of 10
/// (`exe::crt_malloc` of 0x28; none: the box at line 0x299) with `category_b_group_clubs`; each club past the tenth
/// shows the box at line 0x2aa and loses its division (`club::DIVISION` = 0, a defect of the original), and the scan
/// goes on. Fewer than ten: the box at line 0x2b7. Then the League_Stage (`category_b_league_stage`: places 0 up, 2 to
/// the play-offs, 0, 0; this league's importance and rule bits `league::LEAGUE_FLAGS`) into stage slot `group` (signed
/// byte); none: the box at line 0x2c2. Otherwise the list and the round table freed and the current stage = `group`. On
/// the error paths the round table (and the list) are not freed (leaks of the original).
/// Original: `TurSecondB::create_group_stage` (0x900780, tur_second_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_group_stage(this: usize, group: u32) {
    let l = League(this);
    let group = group as u8;
    let (table, rounds, extra) = stage_round_table(l, group as u32);
    let list = exe::crt_malloc(0x28);
    if list == 0 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x299);
    }
    let (first, past) = category_b_group_clubs(l, group);
    for (k, &club) in first.iter().enumerate() {
        write_usize(list + 4 * k, club); // cx14_6: the k-th club of the group
    }
    for &club in &past {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x2aa);
        write_usize(club + club::DIVISION, 0);
    }
    if first.len() < 10 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x2b7);
    }
    let slot = group as i8 as isize as usize;
    let stage = category_b_league_stage(l, 10, list, table, rounds, extra, &[0, 2, 0, 0], group as u32, l.importance() as u32, l.league_flags() as u16 as u32); // cx14_6: 2 to the play-offs
    write_usize(l.stages() + 4 * slot, stage);
    if read_usize(l.stages() + 4 * slot) == 0 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x2c2);
    }
    covered!("cx14_6 category b group stage");
    exe::crt_free(list);
    exe::crt_free(table);
    l.set_current_stage(slot);
}

/// Comparer for `tur_second_b_create_group_stage`: only the original runs (it allocates the stage); with ten clubs in
/// the plan, the stage's clubs and the current stage against it.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_group_stage(this: usize, group: u32) {
    let l = League(this);
    let (plan, _) = category_b_group_clubs(l, group as u8);
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_TUR_SECOND_B_CREATE_GROUP_STAGE)(this, group);
    if plan.len() == 10 {
        compare_category_b_stage("tur_second_b_create_group_stage", l, group as u8 as i8 as isize as usize, &plan);
    }
}

/// Builds the promotion group of category B as stage 4: the clubs of `category_b_promotion_clubs` in a local list of
/// ten (each one past the tenth: the box at line 0x2f2); its round table (`stage_round_table`(4)); the League_Stage
/// (`category_b_league_stage`: places 2 up, 3 to the play-offs, 0, 0; importance 0x14 and rule bits 3) into slot 4;
/// none: the box at line 0x300 (the round table not freed). Otherwise the round table freed and the current stage = 4.
/// With fewer than ten clubs through the original passes stack leftovers in the empty slots and the copy zeros
/// (never seen: the five groups send two each).
/// Original: `TurSecondB::create_promotion_group` (0x900ab0, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_promotion_group(this: usize) {
    let l = League(this);
    let (clubs, more) = category_b_promotion_clubs(l);
    let mut list = [0usize; 10];
    list[..clubs.len()].copy_from_slice(&clubs);
    for _ in 0..more {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x2f2);
    }
    let (table, rounds, extra) = stage_round_table(l, 4);
    let stage = category_b_league_stage(l, 10, list.as_ptr() as usize, table, rounds, extra, &[2, 3, 0, 0], 4, 0x14, 3); // cx14_6: 2 up, 3 to the play-offs
    write_usize(l.stages() + 4 * 4, stage);
    if read_usize(l.stages() + 4 * 4) == 0 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x300);
    }
    covered!("cx14_6 category b promotion group");
    exe::crt_free(table);
    l.set_current_stage(4);
}

/// Comparer for `tur_second_b_create_promotion_group`: only the original runs; with ten clubs through, the stage's
/// clubs and the current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_promotion_group(this: usize) {
    let l = League(this);
    let (plan, _) = category_b_promotion_clubs(l);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TUR_SECOND_B_CREATE_PROMOTION_GROUP)(this);
    if plan.len() == 10 {
        compare_category_b_stage("tur_second_b_create_promotion_group", l, 4, &plan);
    }
}

/// Builds the five classification groups of category B, stages 5 to 9, one for each first phase group g = −1 to 3
/// (stage g + 6): the clubs of `category_b_classification_clubs` in a local list of eight (each one past the eighth:
/// the box at line 0x333), its round table, the League_Stage (`category_b_league_stage` with eight clubs: places 0, 1
/// to the play-offs, 0, 2 down; importance 0x14 and rule bits 2) into its slot. None: the box at line 0x340 and the
/// remaining groups are not built (the round table not freed). Otherwise the round table freed, the current stage =
/// the new one and each of the eight new rows takes the first phase record of its club (the 42 bytes of the totals,
/// `league_table_row::PLAYED` − 4 to +0x36, from the club's row in its first phase group, looked for in row order)
/// with the played counts (overall, home and away) cleared. With fewer than eight clubs the original passes stack
/// leftovers in the empty slots and the copy zeros (never seen: each group of ten keeps eight).
/// Original: `TurSecondB::create_classification_groups` (0x900d00, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_classification_groups(this: usize) {
    let l = League(this);
    for g in -1i8..4 {
        let slot = (g + 6) as usize;
        let group = if g == -1 { l } else { League(read_usize(l.stages() + 4 * g as usize)) };
        let (clubs, more) = category_b_classification_clubs(l, g);
        let mut list = [0usize; 8];
        list[..clubs.len()].copy_from_slice(&clubs);
        for _ in 0..more {
            show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x333);
        }
        let (table, rounds, extra) = stage_round_table(l, slot as u32);
        let stage = category_b_league_stage(l, 8, list.as_ptr() as usize, table, rounds, extra, &[0, 1, 0, 2], slot as u32, 0x14, 2); // cx14_6: 1 to the play-offs, 2 down
        write_usize(l.stages() + 4 * slot, stage);
        if read_usize(l.stages() + 4 * slot) == 0 {
            return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x340);
        }
        exe::crt_free(table);
        l.set_current_stage(slot);
        covered!("cx14_6 category b classification group");
        let stage = League(stage);
        for k in 0..8 {
            let row = stage.table() + k * league_table_row::SIZE;
            let club = read_usize(row);
            let mut j = 0i16;
            while j < group.club_count() {
                let from = group.table() + j as usize * league_table_row::SIZE;
                if read_usize(from) == club {
                    carry_first_phase_record(from, row); // cx14_6: the first phase record carried over
                    break;
                }
                j += 1;
            }
        }
    }
}

/// Comparer for `tur_second_b_create_classification_groups`: only the original runs (it allocates the stages); each
/// built stage's clubs against the plan (as a set) and each of its rows against the club's first phase row (the 42
/// bytes from +0xd, the three played counts cleared).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_classification_groups(this: usize) {
    let l = League(this);
    let plans: Vec<(Vec<usize>, Vec<Vec<u8>>)> = (-1i8..4).map(|g| {
        let (clubs, _) = category_b_classification_clubs(l, g);
        let records = clubs.iter().map(|&c| {
            let row = category_b_first_phase_rows(l).into_iter().find(|(k, r)| *k == g && read_usize(r.addr()) == c).map(|(_, r)| r.addr()).unwrap_or(0);
            let mut b = if row != 0 { core::slice::from_raw_parts((row + 0xd) as *const u8, 0x2a).to_vec() } else { Vec::new() };
            if !b.is_empty() {
                b[4] = 0;
                b[0x12] = 0;
                b[0x20] = 0;
            }
            b
        }).collect();
        (clubs, records)
    }).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TUR_SECOND_B_CREATE_CLASSIFICATION_GROUPS)(this);
    for (k, (clubs, records)) in plans.iter().enumerate() {
        let stage = League(read_usize(l.stages() + 4 * (k + 5)));
        if stage.is_null() || clubs.len() < 8 {
            continue;
        }
        let mut found = category_b_stage_clubs(stage);
        let rows_ok = (0..8).all(|r| {
            let row = stage.table() + r * league_table_row::SIZE;
            let i = clubs.iter().position(|&c| c == read_usize(row));
            i.is_some_and(|i| core::slice::from_raw_parts((row + 0xd) as *const u8, 0x2a) == &records[i][..])
        });
        let mut want = clubs.clone();
        found.sort_unstable();
        want.sort_unstable();
        SCOREBOARD.record(found == want && rows_ok, || format!("tur_second_b_create_classification_groups({:#x}) stage {}: plan {:x?} found {:x?} rows {}", this, k + 5, want, found, rows_ok));
    }
}

/// The body of `get_club_array` of a league split into groups (thiscall(&list, &count), ret 8): the count = `total`
/// (written first), a list of `total` club pointers (`exe::crt_malloc`; out of memory: the box at `line` of `file`,
/// the count 0 and nothing else), filled as `grouped_league_club_slots` says.
pub(crate) unsafe fn grouped_league_club_array(this: League, list: *mut usize, n: *mut i16, total: i16, groups: usize, bound_by_own: bool, file: usize, line: u32) {
    *n = total;
    let v = exe::crt_malloc((total as i32 as u32).wrapping_mul(4));
    *list = v;
    if v == 0 {
        show_error_at(file, line);
        *n = 0;
        return;
    }
    covered!("cx14_3 grouped club list");
    for (k, club) in grouped_league_club_slots(this, groups, bound_by_own) {
        write_usize(v + 4 * k, club);
    }
}

/// The comparer of a grouped `get_club_array`: only the original runs (it allocates the list the caller frees); the
/// count and every slot the copy would write (the last write of each slot) must match.
pub(crate) unsafe fn compare_grouped_league_club_array(name: &str, this: League, list: *mut usize, n: *mut i16, total: i16, groups: usize, bound_by_own: bool, original: impl FnOnce()) {
    original();
    if *list == 0 {
        return;
    }
    let mut expected: Vec<Option<usize>> = vec![None; total.max(0) as usize];
    for (k, club) in grouped_league_club_slots(this, groups, bound_by_own) {
        if k < expected.len() {
            expected[k] = Some(club);
        }
    }
    let got: Vec<Option<usize>> = expected.iter().enumerate().map(|(k, e)| e.map(|_| read_usize(*list + 4 * k))).collect();
    SCOREBOARD.record(*n == total && got == expected, || format!("{}({:#x}) count {}: original {:x?} rust {:x?}", name, this.addr(), *n, got, expected));
}

/// The body of `set_predicted_positions` of a league split into groups (thiscall, ret 0): without statistics, the box
/// at `lines.0` and nothing else; otherwise a list of `list_size` bytes (`exe::crt_malloc`; out of memory: the box at
/// `lines.1`). For each group g = −1 (the league's own table) to `groups` − 1 (`league::STAGES`[g]): its clubs copied
/// into the list, sorted by strength (qsort `exe::crt_qsort` with 0x4b07d0, cdecl) and the club in place k gets the
/// predicted place `place(g, k)` in the statistics (`exe::comp_stats_set_club_final_position`, which keeps a byte).
/// The list is freed at the end.
pub(crate) unsafe fn grouped_league_predicted_positions(this: League, groups: i8, list_size: u32, file: usize, lines: (u32, u32), place: impl Fn(i8, u8) -> u8) {
    if this.comp_stats() == 0 {
        show_error_at(file, lines.0);
        return;
    }
    let list = exe::crt_malloc(list_size);
    if list == 0 {
        show_error_at(file, lines.1);
        return;
    }
    covered!("cx14_3 grouped predicted positions");
    for g in -1..groups {
        let t = if g == -1 { this } else { League(read_usize(this.stages() + 4 * g as usize)) };
        let mut i = 0i16;
        while i < t.club_count() {
            write_usize(list + 4 * i as usize, aus_nsl_table_row_club(t, i as usize));
            i += 1;
        }
        exe::crt_qsort(list, t.club_count() as i32 as u32, 4, 0x4b07d0);
        let mut k = 0i16;
        while k < t.club_count() {
            exe::comp_stats_set_club_final_position(this.comp_stats(), read_usize(list + 4 * k as usize), place(g, k as u8) as u32);
            k += 1;
        }
    }
    exe::crt_free(list);
}

/// The comparer of a grouped `set_predicted_positions`: side by side over the predicted place byte
/// (`comp_stats_club::FINAL_POSITION`) of every club of every group (only the original without statistics or when a
/// club has no entry yet).
pub(crate) unsafe fn compare_grouped_predicted_positions(name: &str, this: League, groups: i8, original: impl FnOnce(), copy: impl FnOnce()) {
    let stats = this.comp_stats();
    if stats == 0 {
        return original();
    }
    let mut regions = Vec::new();
    for g in -1..groups {
        let t = if g == -1 { this } else { League(read_usize(this.stages() + 4 * g as usize)) };
        for i in 0..t.club_count().max(0) as usize {
            let e = exe::comp_stats_find_club_entry(stats, aus_nsl_table_row_club(t, i));
            if e == 0 {
                return original();
            }
            regions.push((e + comp_stats_club::FINAL_POSITION, 1));
        }
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&regions, original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}) original={:x?} rust={:x?}", name, this.addr(), mo, mr));
}

/// The three progress codes (current, best, worst) of a category B club at a stage, as the original turns them into
/// the season codes (byte arithmetic, "×5 + 1" written `scale`):
/// - first phase (stage below 4, signed: the league's own group −1 and the group stages 0..3): current
///   `scale(5 × current − 4)`, best 1, worst `scale(7)` when the worst code is below 3, else `scale(0x2e)`;
/// - promotion group (4): current `scale(current)` from 3 up, else `scale(1)`; best 1; worst `scale(1)` for 1 and 2,
///   `scale(5)` for 3 to 5, else `scale(worst)`;
/// - classification groups (5 to 9): `scale(current)`, 1, `scale(worst)`;
/// - play-off (10): each code 1 stays 1, any other becomes 0xb;
/// - later stages: the codes as they came.
pub(crate) fn turkish_category_b_progress(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let scale = |v: u8| v.wrapping_mul(5).wrapping_add(1);
    if stage < 4 {
        (scale(current.wrapping_mul(5).wrapping_sub(4)), 1, scale(if worst < 3 { 7 } else { 0x2e }))
    } else if stage == 4 {
        let worst = match worst {
            1 | 2 => 1,
            3..=5 => 5,
            w => w,
        };
        (scale(if current >= 3 { current } else { 1 }), 1, scale(worst))
    } else if stage < 10 {
        (scale(current), 1, scale(worst))
    } else if stage == 10 {
        let play_off = |v: u8| if v == 1 { 1 } else { 0xb }; // cx14_3: the play-off codes
        (play_off(current), play_off(best), play_off(worst))
    } else {
        (current, best, worst)
    }
}

/// The plan of `tur_second_b_update_staff_records_for_status` for a club with `status` at `stage` (signed):
/// - first phase (stage below 4): status 1 adds the qualified record; the result is 0;
/// - promotion group (4), classification groups (5 to 9) and play-off (10): no reserve division, nothing and 0; the
///   group by `category_b_group_of` (none: the box at line 0x3d4, 0x400 or 0x42a and −1), the club's row in it
///   (League::find_club_row `exe::league_find_club_row`(group, club, 0); none: −1). Then in the promotion group status
///   0 or 5 is written in the row with the promotion record (0) and any other status gives −1; in a classification
///   group status 3 is written with the relegation record; in the play-off status 1 writes 0 in the row with the
///   promotion record; otherwise nothing and 0;
/// - later stages: nothing and 0.
pub(crate) unsafe fn category_b_record_plan(this: League, club: Club, status: u8, stage: i8) -> CategoryBRecordPlan {
    let mut plan = CategoryBRecordPlan { error_line: None, row: None, record: None, result: 0 };
    if stage < 4 {
        if status == 1 {
            plan.record = Some(CategoryBRecord::Qualified);
        }
        return plan;
    }
    if stage > 10 {
        return plan;
    }
    let division = club.reserve_division().addr();
    if division == 0 {
        return plan;
    }
    let Some(group) = category_b_group_of(this, division) else {
        plan.error_line = Some(match stage {
            4 => 0x3d4,
            10 => 0x42a,
            _ => 0x400,
        });
        plan.result = u32::MAX;
        return plan;
    };
    let row = exe::league_find_club_row(group, club, 0);
    if row == 0 {
        plan.result = u32::MAX;
        return plan;
    }
    match stage {
        4 if status == 0 || status == 5 => {
            plan.row = Some((row, status));
            plan.record = Some(CategoryBRecord::Promotion);
        }
        4 => plan.result = u32::MAX,
        10 if status == 1 => {
            plan.row = Some((row, 0));
            plan.record = Some(CategoryBRecord::Promotion);
        }
        5..=9 if status == 3 => {
            plan.row = Some((row, 3)); // cx14_4: the classification group club goes down
            plan.record = Some(CategoryBRecord::Relegation);
        }
        _ => {}
    }
    plan
}

/// The text of a club and a competition: `format` with the club's short gender byte twice, then (when `with_comp`)
/// the competition's short gender byte twice, the club's short name and (when `with_comp`) the competition's short
/// name into the shared buffer (`PRAISE_TEXT_BUFFER`), the buffer set in `text` with the club's link (0x7d5) and the
/// competition's (0x7d0).
pub(crate) unsafe fn club_and_competition_short_text(club_id: u32, comp: usize, format: usize, with_comp: bool, text: usize) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let team = club.addr() + club::SHORT_NAME;
    if with_comp {
        let cg = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        story_lang!(PRAISE_TEXT_BUFFER, format, g, g, cg, cg, team, comp + competition::SHORT_NAME);
    } else {
        story_lang!(PRAISE_TEXT_BUFFER, format, g, g, team);
    }
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, team as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// The clubs whose final position the category B records, with the place: the first two of each first phase group
/// (the league's own table, then the group stages 0..3) get places 1 and 2, the clubs of each classification group
/// (stages 5 to 9) places 3 upwards in table order (a byte counter). The promotion group and the play-off record
/// nothing.
pub(crate) unsafe fn category_b_final_places(this: League) -> Vec<(Club, u8)> {
    let mut v = Vec::new();
    for g in -1i32..4 {
        let t = if g == -1 { this } else { League(read_usize(this.stages() + 4 * g as usize)) };
        for p in 1..=2u8 {
            v.push((Club(aus_nsl_table_row_club(t, (p - 1) as usize)), p));
        }
    }
    for g in 5..10usize {
        let t = League(read_usize(this.stages() + 4 * g));
        let mut k = 0i8;
        while (k as i16) < t.club_count() {
            v.push((Club(aus_nsl_table_row_club(t, k as usize)), (k as u8).wrapping_add(3))); // cx14_4: places from 3
            k = k.wrapping_add(1);
        }
    }
    v
}

/// The round table of stage `stage` of a league (virtual `comp_vtable::CREATE_SCHEDULE`(stage, &rounds, &extra, 0):
/// push 0; push &extra; push &rounds; push stage -> thiscall, ret 0x10), with the rounds and the extra.
pub(crate) unsafe fn stage_round_table(this: League, stage: u32) -> (usize, i16, i16) {
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = schedule(this.addr(), stage, &mut rounds, &mut extra, 0);
    (t, rounds, extra)
}

/// A League_Stage of category B (operator new `exe::crt_operator_new` of 0xee; League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58; 0 when out of memory): this league, `n` clubs from `list`, two
/// legs, the competition, the round table with its rounds, the win, draw and category bytes and the tie-breaks of this
/// league, the four place counts `places` (up, to the play-offs, to the relegation play-off, down: a local array in
/// the original), the year, the stage, the extra as its label, `importance`, 1, 0, the rule bits `flags`, −1, 0, 2.
/// (The original pushes the rounds and the extra as dwords with stack leftovers above the word; the constructor reads
/// the words.)
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn category_b_league_stage(l: League, n: u32, list: usize, table: usize, rounds: i16, extra: i16, places: &[u8; 4], stage: u32, importance: u32, flags: u32) -> usize {
    let p = exe::crt_operator_new(0xee);
    if p == 0 {
        return 0;
    }
    let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    ctor(p, l.addr(), n, list, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32, l.match_category() as u32,
        l.addr() + league::TIEBREAKS, places.as_ptr() as usize, l.year() as u16 as u32, stage, extra as u16 as u32, importance, 1, 0, flags, -1, 0, 2)
}

/// The clubs of category B group `group` (0 to 3: groups 2 to 5, [`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G2`] to
/// `_G5`; any other value is group 5), in database order among the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`]:
/// those with a division and a reserve division, the reserve division's id the group's and the division this league's
/// competition. Returns the first ten and the clubs past the tenth.
pub(crate) unsafe fn category_b_group_clubs(l: League, group: u8) -> (Vec<usize>, Vec<usize>) {
    let key = match group {
        0 => KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G2,
        1 => KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G3,
        2 => KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G4,
        _ => KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G5,
    };
    let (mut first, mut extra) = (Vec::new(), Vec::new());
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
        let division = read_usize(club + club::DIVISION);
        let reserve = read_usize(club + club::RESERVE_DIVISION);
        if division != 0 && reserve != 0 && read_usize(reserve) == read_usize(key) && division == l.competition().addr() {
            if first.len() < 10 { first.push(club) } else { extra.push(club) }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    (first, extra)
}

/// The clubs of a stage's table, in row order.
pub(crate) unsafe fn category_b_stage_clubs(stage: League) -> Vec<usize> {
    (0..stage.club_count().max(0) as usize).map(|i| aus_nsl_table_row_club(stage, i)).collect()
}

/// Checks a built category B stage against the planned clubs (as a set: the stage may order its rows) and the
/// current stage.
pub(crate) unsafe fn compare_category_b_stage(name: &str, l: League, slot: usize, plan: &[usize]) {
    let stage = League(read_usize(l.stages() + 4 * slot));
    let mut found = if stage.is_null() { Vec::new() } else { category_b_stage_clubs(stage) };
    let mut want = plan.to_vec();
    found.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(found == want && l.current_stage() == slot, || format!("{}({:#x}) stage {} current {}: plan {:x?} found {:x?}", name, l.addr(), slot, l.current_stage() as i32, want, found));
}

/// The rows of the five first phase groups (the league's own table, g = −1, then the group stages 0..3), in order.
pub(crate) unsafe fn category_b_first_phase_rows(l: League) -> Vec<(i8, LeagueTableRow)> {
    let mut v = Vec::new();
    for g in -1i8..4 {
        let t = if g == -1 { l } else { League(read_usize(l.stages() + 4 * g as usize)) };
        let mut i = 0i16;
        while i < t.club_count() {
            v.push((g, LeagueTableRow(t.table() + i as usize * league_table_row::SIZE)));
            i += 1;
        }
    }
    v
}

/// The promotion group's clubs: the clubs of the rows of status 1 (through) in the five first phase groups, in order;
/// the first ten and how many more.
pub(crate) unsafe fn category_b_promotion_clubs(l: League) -> (Vec<usize>, usize) {
    let through: Vec<usize> = category_b_first_phase_rows(l).into_iter().filter(|(_, r)| r.status() == 1).map(|(_, r)| read_usize(r.addr())).collect();
    let more = through.len().saturating_sub(10);
    (through.into_iter().take(10).collect(), more)
}

/// The clubs of first phase group g (−1 to 3) that did not go through (rows of status other than 1), in row order;
/// the first eight and how many more.
pub(crate) unsafe fn category_b_classification_clubs(l: League, g: i8) -> (Vec<usize>, usize) {
    let out: Vec<usize> = category_b_first_phase_rows(l).into_iter().filter(|(k, r)| *k == g && r.status() != 1).map(|(_, r)| read_usize(r.addr())).collect();
    let more = out.len().saturating_sub(8);
    (out.into_iter().take(8).collect(), more)
}

/// Copies the totals of a first phase row into a classification row (`rep movsd` of ten dwords and a word from
/// +0xd) and clears the played counts (`league_table_row::PLAYED`, `HOME_PLAYED`, `AWAY_PLAYED`).
pub(crate) unsafe fn carry_first_phase_record(from: usize, to: usize) {
    core::ptr::copy_nonoverlapping((from + 0xd) as *const u8, (to + 0xd) as *mut u8, 0x2a);
    write_u8(to + league_table_row::PLAYED, 0);
    write_u8(to + league_table_row::HOME_PLAYED, 0);
    write_u8(to + league_table_row::AWAY_PLAYED, 0);
}

/// Where `get_club_array` of a league split into groups writes each club, in the original's order: the league's own
/// table at 0.., then each of the `groups` group stages (`league::STAGES`[0..groups]) at `club_count`.. again.
///
/// Defect of the original: the group stages are not offset by the groups before them, so every group overwrites the
/// same slots (`club_count` to 2 × `club_count` − 1) and only the last group's clubs stay there; the rest of the list
/// (allocated for every club of every group) is left as malloc returned it. With `bound_by_own`, each group is copied
/// for the league's own club count instead of the group's (the Spanish Second Division B; the groups have the same
/// size).
pub(crate) unsafe fn grouped_league_club_slots(this: League, groups: usize, bound_by_own: bool) -> Vec<(usize, usize)> {
    let mut slots = Vec::new();
    let mut i = 0i16;
    while i < this.club_count() {
        slots.push((i as usize, aus_nsl_table_row_club(this, i as usize)));
        i += 1;
    }
    for g in 0..groups {
        let stage = League(read_usize(this.stages() + 4 * g));
        let mut i = 0i16;
        while i < if bound_by_own { this.club_count() } else { stage.club_count() } {
            let k = this.club_count() as i32 + i as i32; // cx14_3: the slot of a group club (no group offset)
            slots.push((k as usize, aus_nsl_table_row_club(stage, i as usize)));
            i += 1;
        }
    }
    slots
}

/// The first phase group of a category B club, by the id of its reserve division (`club::RESERVE_DIVISION`): group 1
/// ([`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1`]) is the league's own table, groups 2 to 5 the group stages 0..3.
/// None when the id is none of the five.
pub(crate) unsafe fn category_b_group_of(this: League, division: usize) -> Option<League> {
    let id = read_usize(division);
    TURKISH_CATEGORY_B_GROUPS.iter().position(|&g| id == read_usize(g)).map(|k| if k == 0 { this } else { League(read_usize(this.stages() + 4 * (k - 1))) })
}

/// The source file of the Turkish second division category B (tur_second_b.cpp), for the error boxes.
pub(crate) const TURKISH_CATEGORY_B_SOURCE: usize = 0xaa18b8;

/// What `tur_second_b_update_staff_records_for_status` does for a club, worked out without side effects: the error
/// box line (a club whose group is none of the five), the row of its first phase group and the status written there,
/// the record added and the result.
#[derive(Debug)]
pub(crate) struct CategoryBRecordPlan {
    pub(crate) error_line: Option<u32>,
    pub(crate) row: Option<(usize, u8)>,
    pub(crate) record: Option<CategoryBRecord>,
    pub(crate) result: u32,
}

/// The staff record a category B status adds (`tur_second_b_update_staff_records_for_status`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CategoryBRecord {
    /// Staff_Records::add_qualified_for_round_record with 0x458, 0, 0x28 (through from a first phase group).
    Qualified,
    /// Staff_Records::add_promotion_record with 0x64.
    Promotion,
    /// Staff_Records::add_relegation_record.
    Relegation,
}
