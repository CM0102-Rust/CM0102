// ---------- sbE_5: the Eerste Divisie play-off groups, staff records and fate news text (hol_first.cpp) ----------

/// Rule: the Eerste Divisie promotion play-off: the period winners (rows already marked 1) are completed to six with the
/// best-placed clubs not yet decided (each marked, announced and given its progress), joined by the Eredivisie's two
/// play-off clubs (status 2); the eight play in two home-and-away groups of four (stages 4 and 5), the first and the
/// last of the eight swapping groups.
///
/// `HolFirst::create_play_off_groups()`: the rows with `league_table_row::STATUS` 1 are counted; while fewer than 6,
/// each row in table order with status 0xff gets 1, the status announced (virtual `comp_vtable::ANNOUNCE_TEAM_STATUS`
/// (club, 1, `league::STAGE_INDEX`, 0, 0)) and its progress set (`belgian_progress_virtual`). The list (`exe::crt_malloc`
/// of 0x20; none: the hol_first.cpp (0x9cee08) box at line 0x2a9 and nothing else) with the first six clubs with status
/// 1, then the Eredivisie's (`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) clubs with status 2 in table order
/// (League::get_table_row, no bound on the row) until there are eight; list[0] and list[7] swapped. For stage 4 and then
/// stage 5: the round table (virtual `comp_vtable::CREATE_SCHEDULE`(stage, &rounds, &label, 0)); `league::RELEGATED_COUNT`
/// = 0 (before the first group); a League_Stage (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100,
/// thiscall, ret 0x58: this league, 4 clubs, list[0..4] or list[4..8], 2 legs, the competition, the table and its
/// rounds, the win, draw and category bytes, the tie-breaks (4, 1, 2, 0), the places (this league's four counts from `league::PROMOTED_COUNT`), the year,
/// the stage, the label, importance 0x14, 1, 0, rule bits 0x22, −1, prize fund 0 and prize mode 2) into stage slot 4 or
/// 5; the first round table freed after its stage, then the list and the second round table. With both stages
/// `league::CURRENT_STAGE` = 5, otherwise the box of line 0x2e6.
/// Original: `HolFirst::create_play_off_groups` (0x5f6720, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_create_play_off_groups(this: usize) {
    let l = League(this);
    let row = |k: i32| LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
    let mut qualified = (0..l.club_count() as i32).filter(|&k| row(k).status() == 1).count() as i32;
    let mut k = 0i32;
    while k < l.club_count() as i32 && qualified < 6 {
        let r = row(k);
        if r.status() == 0xff {
            r.set_status(1);
            qualified += 1;
            let announce: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::ANNOUNCE_TEAM_STATUS));
            announce(this, r.club() as usize, 1, l.stage_index() as u32, 0, 0);
            belgian_progress_virtual(l, r.club() as usize);
        }
        k += 1;
    }
    let list = exe::crt_malloc(0x20);
    if list == 0 {
        return show_error_at(0x9cee08, 0x2a9);
    }
    covered!("sbE_5 eerste divisie play-off groups");
    let (mut n, mut k) = (0usize, 0i32);
    while k < l.club_count() as i32 && n < 6 {
        if row(k).status() == 1 {
            write_usize(list + 4 * n, row(k).club() as usize);
            n += 1;
        }
        k += 1;
    }
    let premier = comp_by_global(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION);
    let mut k = 0i32;
    while n < 8 {
        if read_u8(exe::league_get_table_row(premier, k) + league_table_row::STATUS) == 2 { // sbE_5: the Eredivisie's play-off clubs
            write_usize(list + 4 * n, read_usize(exe::league_get_table_row(premier, k)));
            n += 1;
        }
        k += 1;
    }
    let (first, last) = (read_usize(list), read_usize(list + 0x1c));
    write_usize(list, last);
    write_usize(list + 0x1c, first);
    let tiebreaks: [u8; 4] = [4, 1, 2, 0];
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let mut tables = [0usize; 2];
    for (g, stage) in [4u32, 5].into_iter().enumerate() {
        let (mut rounds, mut label) = (0i16, 0i16);
        tables[g] = schedule(this, stage, &mut rounds, &mut label, 0);
        if g == 0 {
            write_u8(this + league::RELEGATED_COUNT, 0);
        }
        let p = exe::crt_operator_new(0xee);
        let s = if p != 0 {
            let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
            ctor(p, this, 4, list + 0x10 * g, 2, l.competition().addr(), tables[g], rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
                l.match_category() as u32, tiebreaks.as_ptr() as usize, this + league::PROMOTED_COUNT, l.year() as u16 as u32, stage, label as u16 as u32, 0x14, 1, 0, 0x22, -1, 0, 2) // the places: the four counts from PROMOTED_COUNT
        } else {
            0
        };
        write_usize(l.stages() + 4 * stage as usize, s);
        if g == 0 {
            exe::crt_free(tables[0]);
        }
    }
    exe::crt_free(list);
    exe::crt_free(tables[1]);
    if read_usize(l.stages() + 0x10) != 0 && read_usize(l.stages() + 0x14) != 0 {
        l.set_current_stage(5);
    } else {
        show_error_at(0x9cee08, 0x2e6);
    }
}

/// Comparer for `hol_first_create_play_off_groups`: only the original (it marks, announces and builds the stages);
/// afterwards stage slots 4 and 5 must hold stages of 4 clubs and 5 be the current stage.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_create_play_off_groups(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let (a, b) = (read_usize(l.stages() + 0x10), read_usize(l.stages() + 0x14));
    let ok = a != 0 && b != 0 && League(a).club_count() == 4 && League(b).club_count() == 4 && l.current_stage() as i32 == 5;
    SCOREBOARD.record(ok, || format!("hol_first_create_play_off_groups({:#x}): stages {:#x} {:#x}, current {}", this, a, b, l.current_stage() as i32));
}

/// What the Eerste Divisie's staff records routine does (no side effects): the record, the row whose status it writes
/// with the status, and the result.
pub(crate) struct DutchRecordPlan {
    record: Option<PlayOffRecord>,
    row: Option<(usize, u8)>,
    result: u32,
}

/// The plan of `hol_first_update_staff_records_for_status`. Season (stage 0xff), by status: 5 the title, 0 promotion
/// (weight 0x64), 1 the play-off place (the qualified record with 0xa0, 0, 0x28), 3 relegation; the result 0. The
/// periods (stage below 4, signed): status 1 the play-off place; the result −1. The play-off groups (stages 4 and 5): an
/// Eerste Divisie club (its division the id of `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION`, read without a null check) found in
/// this table: status 0 promotion (weight 0x32) with the row's status 0 and the result the Eerste Divisie's id, status
/// 0xfe the row's status 0xfe and −1; otherwise (not found, another status or another club) the Eredivisie
/// (`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) table: status 0 the row's status 0xfe (stays up), status 0xfe relegation
/// with the row's status 3, both with the result the Eredivisie's id; not found: 0. Other stages: 0.
/// Defect of the original [inference]: the relegation record of an Eredivisie club names the Eerste Divisie's competition
/// (this league's `league::COMPETITION`), not the Eredivisie's.
pub(crate) unsafe fn dutch_record_plan(this: League, club: Club, status: u8, stage: u8) -> DutchRecordPlan {
    let none = DutchRecordPlan { record: None, row: None, result: 0 };
    if stage == 0xff {
        let record = match status {
            5 => PlayOffRecord::Champions,
            0 => PlayOffRecord::Promotion(0x64),
            1 => PlayOffRecord::Qualified,
            3 => PlayOffRecord::Relegation,
            _ => return none,
        };
        return DutchRecordPlan { record: Some(record), ..none };
    }
    if (stage as i8) < 4 {
        return DutchRecordPlan { record: (status == 1).then_some(PlayOffRecord::Qualified), row: None, result: 0xffff_ffff };
    }
    if stage != 4 && stage != 5 {
        return none;
    }
    if read_u32(club.division().addr()) == read_u32(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) {
        if let Some(row) = table_row_of_club(this.table(), this.club_count(), club.addr()).filter(|_| status == 0 || status == 0xfe) {
            return if status == 0 {
                DutchRecordPlan { record: Some(PlayOffRecord::Promotion(0x32)), row: Some((row, 0)), result: read_u32(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) }
            } else {
                DutchRecordPlan { record: None, row: Some((row, 0xfe)), result: 0xffff_ffff }
            };
        }
    }
    let premier = League(comp_by_global(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION));
    match table_row_of_club(premier.table(), premier.club_count(), club.addr()) {
        Some(row) if status == 0 => DutchRecordPlan { record: None, row: Some((row, 0xfe)), result: read_u32(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) },
        Some(row) if status == 0xfe => DutchRecordPlan { record: Some(PlayOffRecord::Relegation), row: Some((row, 3)), result: read_u32(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) },
        _ => none,
    }
}

/// Rule: the Eerste Divisie's staff records: in the season the title, promotion, the play-off place and relegation; a
/// period winner gets the play-off place; in the play-off groups the Eerste Divisie club that wins its group goes up
/// (promotion record) and the Eredivisie club that does not win its group goes down (relegation record), the table
/// statuses telling who stays where.
///
/// `dutch_record_plan` carried out: the records into `G_STAFF_RECORDS` (Staff_Records::add_league_champions_record,
/// add_promotion_record with the weight, add_qualified_for_round_record with 0xa0, 0, 0x28, add_relegation_record), all
/// with this league's competition; then the row's status. Returns the plan's result. The last three arguments are not
/// read.
/// Original: `HolFirst::update_staff_records_for_status` (0x5f6ee0, hol_first.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn hol_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let plan = dutch_record_plan(l, Club(club), status as u8, stage as u8);
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match plan.record {
        Some(PlayOffRecord::Champions) => exe::staff_records_add_league_champions_record(records, club, comp),
        Some(PlayOffRecord::Promotion(weight)) => exe::staff_records_add_promotion_record(records, club, comp, weight),
        Some(PlayOffRecord::Qualified) => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0xa0, 0, 0x28),
        Some(PlayOffRecord::Relegation) => exe::staff_records_add_relegation_record(records, club, comp),
        None => {}
    }
    if let Some((row, s)) = plan.row {
        covered!("sbE_5 eerste divisie play-off status");
        write_u8(row + league_table_row::STATUS, s); // sbE_5: the play-off group's verdict
    }
    plan.result
}

/// Comparer for `hol_first_update_staff_records_for_status`: only the original runs (it adds records); its result and
/// the status of the planned row against the plan.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I1%%);
    let plan = dutch_record_plan(League(this), Club(club), status as u8, stage as u8);
    let r = o(this, club, status, stage, a4, a5, a6);
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == plan.result && row_ok, || format!("hol_first_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={:#x} rust={:#x} row {:?}", this, club, status as u8, stage as u8, r, plan.result, plan.row));
    r
}

/// The text of an Eerste Divisie play-off group result into the shared buffer (`PRAISE_TEXT_BUFFER`): with the
/// competition (`format` takes the club's short gender byte twice, the competition's twice, the club's short name and the
/// competition's short name) or without it (the club's gender twice and its name).
unsafe fn dutch_play_off_text(club: Club, comp: usize, format: usize, with_comp: bool) {
    let g = club.short_gender_name() as i8 as i32;
    if with_comp {
        let cg = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
        story_lang!(PRAISE_TEXT_BUFFER, format, g, g, cg, cg, club.addr() + club::SHORT_NAME, comp + competition::SHORT_NAME);
    } else {
        story_lang!(PRAISE_TEXT_BUFFER, format, g, g, club.addr() + club::SHORT_NAME);
    }
}

/// Rule: the fate news text of an Eerste Divisie club in the play-off groups (stages 4 and 5): the group winner (outcome
/// 0) of the Eerste Divisie "win promotion" (with a news item, "have won promotion through the {competition} playoffs"),
/// of the Eredivisie "avoid relegation" (with a news item, "have won their {competition} playoff group"); a club that
/// does not win (outcome 0xfe) "relegated" (with a news item, "have failed to win their {competition} playoff group");
/// any other outcome, nothing. The other stages get the common text.
///
/// Outside stages 4 and 5: Comp::build_fate_news_text (`exe::comp_build_fate_news_text`, a7 = 0, the rest as they came).
/// Otherwise the Eerste Divisie is told apart by a7 (the club's division id) against
/// `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION`, the news item by a9; the texts 0x9cef28, 0x9cef4c, 0x9ceeb8, 0x9ceedc, 0x9cee44
/// and 0x9cee64 into the shared buffer (`dutch_play_off_text`), then the buffer set in `text` (`exe::text_set_string`)
/// with the club's link (0x7d5) and the competition's link (0x7d0, this league's `league::COMPETITION`), both always.
/// Original: `HolFirst::build_fate_news_text` (0x5f7110, hol_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn hol_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    if stage as u8 != 4 && stage as u8 != 5 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    }
    let club = club_at(club_id as isize);
    let comp = League(this).competition().addr();
    let first = a7 == read_u32(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION);
    let with_comp = news != 0;
    let format = match (outcome as u8, first, with_comp) {
        (0, true, true) => 0x9cef4c,   // "... have won promotion through the ... playoffs"
        (0, true, false) => 0x9cef28,  // "... win promotion"
        (0, false, true) => 0x9ceedc,  // "... have won their ... playoff group"
        (0, false, false) => 0x9ceeb8, // "... avoid relegation"
        (0xfe, _, true) => 0x9cee64,   // "... have failed to win their ... playoff group"
        (0xfe, _, false) => 0x9cee44,  // "... relegated"
        _ => return,
    };
    covered!("sbE_5 eerste divisie play-off text");
    dutch_play_off_text(club, comp, format, with_comp);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// Comparer for `hol_first_build_fate_news_text`: in the play-off groups, side by side with `compare_text_writer` (the
/// original into the caller's Text, the copy into a local one); in the other stages only the original (the common text
/// writes the news object).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_hol_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I2%%);
    let play_off = (stage as u8 == 4 || stage as u8 == 5) && (outcome as u8 == 0 || outcome as u8 == 0xfe);
    if play_off && text != 0 {
        return compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
            |t| hol_first_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
            || format!("hol_first_build_fate_news_text(club {}, outcome {}, stage {}, news {})", club_id, outcome as u8, stage as u8, news != 0));
    }
    o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)
}
