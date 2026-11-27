use super::*;

/// Builds the Swedish Second Division at game creation (`construct_grouped_league` with `SWEDISH_SECOND`): the North
/// section in the league's own table, the other five as stages 0 to 4, then the predicted positions.
/// Original: `SweSecond::SweSecond` (0x8916f0, swe_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_swe_second(this: usize, year: u32, record: usize) -> usize {
    construct_grouped_league(this, year, record, &SWEDISH_SECOND, "rx09_7 new swedish second division")
}

/// Comparer for `construct_swe_second` (`compare_league_with_stages_ctor` with the shape of `SWEDISH_SECOND`): only the
/// original runs (it allocates the league, the stages and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SWE_SECOND);
    compare_league_with_stages_ctor("construct_swe_second", League(this), year, record, &SWEDISH_SECOND.shape, || o(this, year, record))
}

/// Rule: the Swedish Second Division: 3 points a win and 1 a draw; tie-breaks goal difference, goals for; each section
/// winner to the promotion play-off, each runner-up to the relegation play-off, the last two down; the First Division
/// above (`swedish_second_rules`, then the main round table into `league::ROUND_DATES`).
/// Original: `SweSecond::set_up_rules` (0x892380, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_set_up_rules(this: usize) -> u32 {
    covered!("rx09_7 swedish second division rules");
    eng_prm_apply_league_rules(League(this), &swedish_second_rules())
}

/// Comparer for `swe_second_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("swe_second_set_up_rules", League(this), swedish_second_rules(), || o(this), || swe_second_set_up_rules(this))
}

/// The round tables of the Swedish Second Division by stage (the signed byte of the argument): stages up to 4 (the
/// North section at −1, the other five at 0 to 4) the 22 section rounds, extra = stage + 0x46d written only for stages
/// −1 to 4 (line 0xb9); stage 5 the promotion play-off, extra 0x438 (line 0x12b); stage 6 the relegation play-off,
/// extra 0x452 (line 0x153); later stages 0 with nothing written.
/// Original: `SweSecond::create_schedule` (0x891a60, swe_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    let s = stage as u8 as i8;
    if s < 5 {
        let code = if s >= -1 { Some((s as i16).wrapping_add(0x46d)) } else { None };
        league_round_table(l, rounds, extra, league, &SWEDISH_SECOND_SECTION_ROUNDS, code, SWEDISH_SECOND_SOURCE, 0xb9, "rx09_7 swedish second section rounds")
    } else if s == 5 {
        cup_round_table(l, rounds, extra, league, &[SWEDISH_SECOND_PROMOTION_PLAY_OFF], 0x438, SWEDISH_SECOND_SOURCE, 0x12b, "rx09_7 swedish second promotion play-off round")
    } else if s == 6 {
        cup_round_table(l, rounds, extra, league, &SWEDISH_SECOND_RELEGATION_PLAY_OFF, 0x452, SWEDISH_SECOND_SOURCE, 0x153, "rx09_7 swedish second relegation play-off rounds")
    } else {
        0
    }
}

/// Comparer for `swe_second_create_schedule` (`compare_league_or_cup_schedule`; cup rounds at stages 5 and 6).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SWE_SECOND_CREATE_SCHEDULE);
    let cup = matches!(stage as u8 as i8, 5 | 6);
    compare_league_or_cup_schedule("swe_second_create_schedule", stage, cup, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| swe_second_create_schedule(this, stage, r, e, l))
}

/// Builds the table of the North section of the Swedish Second Division: `club_count` = 12, the table
/// (`exe::crt_malloc` of 0x2c4; out of memory: the box at line 0x198 and 0) and, for each club of
/// `swedish_second_section_clubs` (the North), a row (League::init_table_row `exe::league_init_table_row`(this, k, club,
/// 0, 0)) and the B team mark (`mark_b_team_row`). A 13th club shows the box at line 0x1a9, loses its division
/// (`club::DIVISION` = 0) and ends the scan. Returns 1 when the table is full, else 0.
/// Original: `SweSecond::create_table` (0x8921b0, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0xc);
    let t = exe::crt_malloc(0x2c4);
    l.set_table(t);
    if t == 0 {
        show_error_at(SWEDISH_SECOND_SOURCE, 0x198);
        return 0;
    }
    covered!("rx09_8 swedish second north table");
    let mut k: u8 = 0;
    for c in swedish_second_section_clubs(l, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH) {
        if (k as i8 as i16) >= l.club_count() {
            show_error_at(SWEDISH_SECOND_SOURCE, 0x1a9);
            write_usize(c.addr() + club::DIVISION, 0);
            break;
        }
        exe::league_init_table_row(l, k as u32, c, 0, 0); // rx09_8: the row of the k-th North club
        mark_b_team_row(l, k as i8 as isize as usize);
        k = k.wrapping_add(1);
    }
    ((k as i8 as i16) == l.club_count()) as u32
}

/// Comparer for `swe_second_create_table`: only the original runs (it allocates the table); the club count, the result,
/// the clubs of the rows and the B team marks against the North clubs, planned beforehand.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_table(this: usize) -> u32 {
    let l = League(this);
    let plan: Vec<usize> = swedish_second_section_clubs(l, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH).iter().take(12).map(|c| c.addr()).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_SECOND_CREATE_TABLE)(this);
    let found: Vec<usize> = if l.table() != 0 { (0..plan.len()).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    let same = r == (plan.len() == 12) as u32 && l.club_count() == 12 && found == plan;
    SCOREBOARD.record(same, || format!("swe_second_create_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}

/// Rule: the Swedish Second Division turnover: last season's stages destroyed, the next year, the rules, the North
/// table, the order and the fixtures, then the other five sections, the predicted positions and the officials
/// (`grouped_league_new_season` with SweSecond::create_table 0x8921b0 and SweSecond::create_section_stage 0x892670,
/// boxes at lines 0x20c, 0x213, 0x21a and 0x220).
/// Original: `SweSecond::start_new_season` (0x892410, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_start_new_season(this: usize) -> u32 {
    grouped_league_new_season(League(this), 0x8921b0, 0x892670, 5, SWEDISH_SECOND_SOURCE, [0x20c, 0x213, 0x21a, 0x220], "rx09_8 swedish second turnover")
}

/// Comparer for `swe_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("swe_second_start_new_season", League(this), || o(this))
}

/// Rule: the next stage of the Swedish Second Division: past the last stage, the error box; otherwise the current stage
/// goes up by one; on reaching stage 5 (the sections are over) the promotion play-off and the relegation play-off are
/// built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box at line 0x23e and nothing
/// else. Otherwise the current stage + 1; at 5, SweSecond::create_promotion_play_off_stage (0x892a10) and
/// SweSecond::create_relegation_play_off_stage (0x892bb0) (thiscall, ret 0).
/// Original: `SweSecond::advance_stage` (0x8925d0, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x23e);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("rx09_8 swedish second next stage");
    if stage == 5 {
        call_on_league(0x892a10, l);
        call_on_league(0x892bb0, l);
    }
}

/// Comparer for `swe_second_advance_stage` (`compare_advance_stage_building`: stage 5 builds).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_SECOND_ADVANCE_STAGE);
    compare_advance_stage_building("swe_second_advance_stage", League(this), |s| s == 5, || o(this), || swe_second_advance_stage(this))
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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_SWE_SECOND_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == plan.result && row_ok, || format!("swe_second_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={:#x} plan={:?}", this, club,
        status as u8, stage as u8 as i8, r, plan));
    r
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_SWE_SECOND_BUILD_FATE_NEWS_TEXT);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_SWE_SECOND_GET_CLUB_ARRAY);
    compare_grouped_league_club_array("swe_second_get_club_array", League(this), list, n, 0x48, 5, false, || o(this, list, n));
}

/// The predicted positions of the Swedish Second Division: each of the six sections (the North in the league's table,
/// the others in the section stages 0..4) sorted by strength, and the club in place k gets 6k + 1 whatever its section
/// (`grouped_league_predicted_positions`; a list of 0x30 bytes, boxes at lines 0x48b and 0x493).
/// Original: `SweSecond::set_predicted_positions` (0x893470, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_set_predicted_positions(this: usize) {
    grouped_league_predicted_positions(League(this), 5, 0x30, SWEDISH_SECOND_SOURCE, (0x48b, 0x493), |_, k| k.wrapping_mul(6).wrapping_add(1)); // rx09_11: 6k + 1
}

/// Comparer for `swe_second_set_predicted_positions` (`compare_grouped_predicted_positions`).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_SECOND_SET_PREDICTED_POSITIONS);
    compare_grouped_predicted_positions("swe_second_set_predicted_positions", League(this), 5, || o(this), || swe_second_set_predicted_positions(this));
}

/// Stores a Swedish Second Division club's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the season codes by `swedish_second_progress`.
/// Original: `SweSecond::set_club_progress` (0x8935e0, swe_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn swe_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx09_11 swedish second progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = swedish_second_progress(stage as u8 as i8, current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_current_position(c);
    entry.set_worst_position(w);
}

/// Comparer for `swe_second_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_SWE_SECOND_SET_CLUB_PROGRESS);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || swe_second_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("swe_second_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}

/// The rules of the Swedish Second Division (six regional sections of 12): 2 legs, 3 points a win and 1 a draw, league
/// matches, national clubs; tie-breaks 1, 2 (goal difference, goals for; +0xc7 = 0 ends); none up directly, 1 to the
/// promotion play-off, 1 to the relegation play-off, 2 down; the First Division above, nothing below; the rule bits
/// 0x42 (a word). The substitutes, the substitutions, +0xab, +0xdb and +0x52 are not written.
pub(crate) unsafe fn swedish_second_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
        (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0), (league::PROMOTED_COUNT, 1, 0),
        (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2), // rx09_7: two go down
        (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, 0xffff_ffff),
        (league::LEAGUE_FLAGS, 2, 0x42)]
}

/// The clubs of a Swedish Second Division section, in database order: those with a division (`club::DIVISION`) that is
/// this league's record and a reserve division (`club::RESERVE_DIVISION`, here the section) whose id is the section's
/// (`section`, a key).
pub(crate) unsafe fn swedish_second_section_clubs(l: League, section: usize) -> Vec<Club> {
    domestic_clubs().into_iter().filter(|c| {
        let division = c.division().addr();
        let reserve = c.reserve_division().addr();
        division != 0 && reserve != 0 && read_usize(reserve) == read_usize(section) && division == l.competition().addr()
    }).collect()
}

/// Marks a B team in row k of a section's table: a club linked to a senior club (`exe::club_get_linked_b_team`(club,
/// &senior, 1) non-zero with the senior flag 0) gets the row status 0xfc (barred from promotion).
pub(crate) unsafe fn mark_b_team_row(t: League, k: usize) {
    let row = t.table() + k * league_table_row::SIZE;
    let mut senior = 0u32;
    if exe::club_get_linked_b_team(read_usize(row), &mut senior, 1) != 0 && senior == 0 {
        write_u8(row + league_table_row::STATUS, 0xfc); // rx09_8: a B team cannot go up
    }
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
    if stage == 6 && club.division() != l.competition() {
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

/// True when `swe_second_build_fate_news_text` writes the text itself (not the common text nor the cup news).
pub(crate) fn swedish_second_writes_own_text(stage: i8, outcome: u8, news: u32) -> bool {
    match stage {
        5 => news == 0,
        6 => news == 0 && matches!(outcome, 1 | 2 | 0xff),
        _ => false,
    }
}

/// The three progress codes (current, best, worst) of a Swedish Second Division club at a stage (byte arithmetic):
/// - the sections (stage below 5): each code × 5 − 4;
/// - the relegation play-off (6): each code 1, 7 or 13 becomes 0x37, 0x3d or 0x43, any other stays;
/// - the promotion play-off (5) and later stages: the codes as they came.
pub(crate) fn swedish_second_progress(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage < 5 {
        let scale = |v: u8| v.wrapping_mul(5).wrapping_sub(4);
        return (scale(current), scale(best), scale(worst));
    }
    if stage != 6 {
        return (current, best, worst);
    }
    let play_off = |v: u8| match v {
        1 => 0x37,
        7 => 0x3d, // rx09_11: the relegation play-off codes
        13 => 0x43,
        v => v,
    };
    (play_off(current), play_off(best), play_off(worst))
}

/// The section of a Swedish Second Division club by the id of its reserve division (`SWEDISH_SECOND_SECTIONS`): the
/// North section is the league's own table, the others the stages 0 to 4. None when the id is none of the six.
pub(crate) unsafe fn swedish_second_section_of(l: League, reserve: usize) -> Option<League> {
    SWEDISH_SECOND_SECTIONS.iter().position(|&s| read_usize(reserve) == read_usize(s)).map(|k| if k == 0 { l } else { League(read_usize(l.stages() + 4 * (k - 1))) })
}

/// The Swedish Second Division: vtable 0x9702a0, the Swedish discipline rule 0x1c, seven stages after the North
/// section (the five other sections, the promotion play-off and the relegation play-off); the rules written inline
/// (no box; SweSecond::set_up_rules 0x892380 is not called), SweSecond::create_table (0x8921b0, line 0x50), sort
/// 0x57, statistics 0x5f, fixtures 0x68; SweSecond::create_section_stage (0x892670) for the five other sections and
/// SweSecond::set_predicted_positions (0x893470).
pub(crate) const SWEDISH_SECOND: GroupedLeagueCtor = GroupedLeagueCtor {
    shape: LeagueWithStages {
        division: LeagueDivision { vtabela: 0x9702a0, discipline: 0x1c, rules: 0x892380, table: 0x8921b0, file: SWEDISH_SECOND_SOURCE, rows: [0, 0x50, 0x68, 0x57, 0x5f], low: false },
        stages: 7, // rx09_7: the seven stages of the Swedish Second Division
        finish: None,
    },
    inline_rules: Some(swedish_second_rules),
    group_stage: 0x892670,
    groups: 5,
    predicted: 0x893470,
};

/// The Swedish Second Division sections: 22 rounds on Sundays (weekday 6) from 6 April to 8 November, the summer break
/// from 7 June to 19 July and a Saturday round on 31 October; no TV slots.
pub(crate) const SWEDISH_SECOND_SECTION_ROUNDS: [SpaSecondScheduleRound; 22] = [
    (6, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 6/abr (rx09_7: the first section round)
    (13, 3, 0, 6, 1, &[(-1, -1, -1)]), // 1: 13/abr
    (20, 3, 0, 6, 1, &[(-1, -1, -1)]), // 2: 20/abr
    (27, 3, 0, 6, 1, &[(-1, -1, -1)]), // 3: 27/abr
    (4, 4, 0, 6, 1, &[(-1, -1, -1)]), // 4: 4/mai
    (11, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 11/mai
    (18, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 18/mai
    (7, 5, 0, 6, 1, &[(-1, -1, -1)]), // 7: 7/jun
    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 8: 19/jul
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 9: 2/ago
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 10: 9/ago
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 11: 16/ago
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 12: 23/ago
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 13: 30/ago
    (6, 8, 0, 6, 1, &[(-1, -1, -1)]), // 14: 6/set
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 15: 13/set
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 16: 20/set
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 17: 4/out
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 18: 18/out
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 19: 25/out
    (31, 9, 0, 5, 1, &[(-1, -1, -1)]), // 20: 31/out
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 21: 8/nov
];

/// The promotion play-off of the Swedish Second Division (stage 5): drawn on 9 November (weekday 0), three ties of two
/// legs 7 days apart from 15 November (weekday 6) between the six section winners, code 0xa0, category 5, two-leg
/// tie-break 3, no prizes.
pub(crate) const SWEDISH_SECOND_PROMOTION_PLAY_OFF: EngCcCupCupRound = cql10r((9, 10, 0, 0), (0xf, 10, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (6, 3, 6, 0), (0, 2, 7), (0, 0, 0));

/// The relegation play-off of the Swedish Second Division (stage 6), two rounds: drawn on 9 November, six ties of two
/// legs from 15 November between twelve lower division clubs (code 0xa); then drawn on 23 November, six ties of two
/// legs from 29 November where the six section runners-up come in (code 0x14, tie-break 0x400); category 5, two-leg
/// tie-break 3, 7 days between the legs, no prizes.
pub(crate) const SWEDISH_SECOND_RELEGATION_PLAY_OFF: [EngCcCupCupRound; 2] = [
    cql10r((9, 10, 0, 0), (0xf, 10, 0, 6, 1, 0), 0xa, (0, 0, 3), 5, (0xc, 6, 0xc, 0), (0, 2, 7), (0, 0, 0)),
    cql10r((0x17, 10, 0, 0), (0x1d, 10, 0, 6, 1, 0), 0x14, (0, 0x400, 3), 5, (0xc, 6, 6, 0xc), (0, 2, 7), (0, 0, 0)), // rx09_7: the runners-up come in
];

/// The ids of the six Swedish Second Division sections, in stage order: the North section (the league's own table),
/// then East Svealand, West Svealand, East Götaland, West Götaland and South Götaland (stages 0 to 4).
pub(crate) const SWEDISH_SECOND_SECTIONS: [usize; 6] = [KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND];

/// The source file of the Swedish Second Division (swe_second.cpp), for the error boxes.
pub(crate) const SWEDISH_SECOND_SOURCE: usize = 0xa8b5b0;
