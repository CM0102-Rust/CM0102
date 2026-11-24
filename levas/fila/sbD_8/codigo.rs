// ---------- sbD_8: the Argentine Second Division staff records and fate news (arg_second.cpp) ----------

/// What `arg_second_update_staff_records_for_status` writes on the table rows and returns (the records apart): up to
/// three rows with their new status.
struct ArgSecondStatusPlan {
    rows: [(usize, u8); 3],
    n: usize,
    result: i32,
}

/// The plan of the Argentine Second Division status update (no side effects; League::find_club_row
/// `exe::league_find_club_row`(league, club, 0) only looks):
/// - stages below 3 (signed): League::update_staff_records_for_status does it all; 0 returned;
/// - stage 3, the promotion zones: status 1 the club's row (if any) gets 0, −1 returned; any other status −1;
/// - stages 4 and 5, the promotion play-offs: a club with rows in the Premier Division (the object of
///   [`KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION`]) and both its stages gets 3 on the three with status 2, otherwise
///   0xfe, and the Premier Division's id is returned; any other club, on its row here (if any): status 2 → 0xfe,
///   otherwise 0; the Second Division's id returned;
/// - stages above 5: 0.
unsafe fn arg_second_status_plan(this: League, club: usize, status: u8, stage: i8) -> ArgSecondStatusPlan {
    let mut p = ArgSecondStatusPlan { rows: [(0, 0); 3], n: 0, result: 0 };
    match stage {
        s if s < 3 => {}
        3 => {
            if status == 1 {
                if let Some(row) = (0..this.club_count().max(0) as usize).map(|r| this.table() + r * 0x3b).find(|&row| read_usize(row) == club) {
                    p.rows[0] = (row, 0);
                    p.n = 1;
                }
            }
            p.result = -1;
        }
        4 | 5 => {
            let premier = League(comp_object_of_key(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION));
            let rows = [
                exe::league_find_club_row(premier, club, 0usize),
                exe::league_find_club_row(read_usize(premier.stages()), club, 0usize),
                exe::league_find_club_row(read_usize(premier.stages() + 4), club, 0usize),
            ];
            if rows.iter().all(|&r| r != 0) {
                let s = if status == 2 { 3 } else { 0xfe };
                p.rows = [(rows[0], s), (rows[1], s), (rows[2], s)];
                p.n = 3;
                p.result = read_i32(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION);
            } else {
                let row = exe::league_find_club_row(this, club, 0usize);
                if row != 0 {
                    p.rows[0] = (row, if status == 2 { 0xfe } else { 0 });
                    p.n = 1;
                }
                p.result = read_i32(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION);
            }
        }
        _ => {}
    }
    p
}

/// Updates the staff records by a club's outcome in the Argentine Second Division (`arg_second_status_plan` for the
/// rows and the value returned). Stages below 3: League::update_staff_records_for_status
/// (`exe::league_update_staff_records_for_status`(club, status, stage, a4, 0, 0)), 0. Stage 3: status 1 the promotion
/// record (weight 0x32), status 2 the qualified record (0x45b, 0x82, 0x28), on the Staff_Records object
/// (`G_STAFF_RECORDS`, with this competition). Stages 4 and 5: a Premier Division club with status 2 the relegation
/// record (with its own division); any other club, not with status 2 and with a row here, the promotion record
/// (weight 0x28).
/// Original: `ArgSecond::update_staff_records_for_status` (0x408cd0, arg_second.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn arg_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, _a5: u32, _a6: u32) -> i32 {
    let l = League(this);
    let s = stage as u8 as i8;
    if s < 3 {
        exe::league_update_staff_records_for_status(this, club, status, stage, a4, 0usize, 0);
        return 0;
    }
    let p = arg_second_status_plan(l, club, status as u8, s);
    for &(row, st) in &p.rows[..p.n] {
        LeagueTableRow(row).set_status(st);
    }
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match (s, status as u8) {
        (3, 1) => {
            covered!("sbD_8 argentine second division promotion zone record");
            exe::staff_records_add_promotion_record(records, club, comp, 0x32);
        }
        (3, 2) => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x45b, 0x82, 0x28),
        (4 | 5, st) => {
            covered!("sbD_8 argentine second division play-off record");
            if p.result == read_i32(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION) && p.n == 3 {
                if st == 2 {
                    exe::staff_records_add_relegation_record(records, club, read_usize(club + club::DIVISION));
                }
            } else if p.n == 1 && st != 2 {
                exe::staff_records_add_promotion_record(records, club, comp, 0x28);
            }
        }
        _ => {}
    }
    p.result
}

/// Sets the Text to the shared buffer [`PRAISE_TEXT_BUFFER`] and links the club's short name (0x7d5) and the
/// competition's (0x7d0, the record `comp`).
pub(crate) unsafe fn set_club_and_competition_links(club_id: u32, comp: usize, text: usize) {
    let club = club_at(club_id as i32 as isize);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// A fate line naming the club and a competition: `format` with the club's short-name gender twice, the
/// competition's short-name gender twice, the club's and the competition's short names (the record `comp`), into
/// [`PRAISE_TEXT_BUFFER`], with the links of both (`set_club_and_competition_links`).
pub(crate) unsafe fn club_and_competition_line(club_id: u32, comp: usize, format: usize, text: usize) {
    let club = club_at(club_id as i32 as isize);
    let (cg, kg) = (club.short_gender_name() as i8 as i32, read_i8(comp + competition::SHORT_GENDER_NAME) as i32);
    story_lang!(PRAISE_TEXT_BUFFER, format, cg, cg, kg, kg, club.addr() + club::SHORT_NAME, comp + competition::SHORT_NAME);
    set_club_and_competition_links(club_id, comp, text);
}

/// A club's one-line fate text that still links the competition: `format` with the club's short-name gender twice and
/// its short name, into [`PRAISE_TEXT_BUFFER`], with the links of the club and of the competition `comp`.
unsafe fn club_line_with_competition_link(club_id: u32, comp: usize, format: usize, text: usize) {
    let club = club_at(club_id as i32 as isize);
    let g = club.short_gender_name() as i8 as i32;
    story_lang!(PRAISE_TEXT_BUFFER, format, g, g, club.addr() + club::SHORT_NAME);
    set_club_and_competition_links(club_id, comp, text);
}

/// Rule: the news of a club's fate in the Argentine Second Division: in the zones (stages below 3) "{club} qualify for
/// Second Promotion" (outcome 6) or relegated (outcome 3, by the average when a7 is 1); in the promotion zones (stage 3)
/// promoted to the Premier Division (outcome 1) or "{club} qualify for Third Promotion" (2); in the play-offs (above
/// 3) avoiding relegation / relegated for a Premier Division club (a7 its id), promoted / missing out otherwise; with
/// `body` the full lines, the cup texts and the main stage through the competitions' common text.
///
/// `ArgSecond::build_fate_news_text(club, outcome, stage, a4, fixture, a6, a7, a8, body, text)`:
/// - stage byte 0xff: Comp::build_fate_news_text (`exe::comp_build_fate_news_text`, stage −1);
/// - stages below 3 (signed): outcome 6, with `body` 0x987880 (`club_and_competition_line`), without it 0x98783c;
///   outcome 3, with `body` 0x987420 (a7 = 1) or 0x9877b8, without it "{club} relegated" (0x987784); any other outcome
///   the Text set to the shared buffer as it is (defect of the original: the last text written there); always the
///   links of the club and of this competition;
/// - stage 3: outcome 1, with `body` 0x987704 naming the Premier Division (its record), without it "{club} win
///   promotion" (0x9876cc), links of the club and of the Premier Division; outcome 2, with `body` 0x98760c, without it
///   0x9875c8, links of the club and of this competition; any other outcome the common text with stage 3;
/// - stages above 3: with `body` the common text (the same arguments); without it, a7 the Premier Division's id:
///   outcome 1 "{club} avoid relegation" (0x987594), otherwise "{club} relegated" (0x987564); any other a7: outcome 1
///   "{club} win promotion" (0x9876cc), otherwise "{club} miss out on promotion" (0x987528); the club's link only.
/// Original: `ArgSecond::build_fate_news_text` (0x408ec0, arg_second.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn arg_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, body: u32, text: usize) {
    let own = League(this).competition().addr();
    let s = stage as u8 as i8;
    let o = outcome as u8;
    if s == -1 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, -1, a4, fixture, a6, a7, a8, body, text);
    }
    if s < 3 {
        covered!("sbD_8 argentine second division zone text");
        match (o, body != 0) {
            (6, true) => club_and_competition_line(club_id, own, 0x987880, text),
            (6, false) => club_line_with_competition_link(club_id, own, 0x98783c, text), // "{club} qualify for Second Promotion"
            (3, true) => club_and_competition_line(club_id, own, if a7 == 1 { 0x987420 } else { 0x9877b8 }, text),
            (3, false) => club_line_with_competition_link(club_id, own, 0x987784, text), // "{club} relegated"
            _ => set_club_and_competition_links(club_id, own, text),
        }
        return;
    }
    if s == 3 {
        let premier = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION).wrapping_mul(0x6b));
        covered!("sbD_8 argentine second division promotion zone text");
        match (o, body != 0) {
            (1, true) => club_and_competition_line(club_id, premier, 0x987704, text),
            (1, false) => club_line_with_competition_link(club_id, premier, 0x9876cc, text), // "{club} win promotion"
            (2, true) => club_and_competition_line(club_id, own, 0x98760c, text),
            (2, false) => club_line_with_competition_link(club_id, own, 0x9875c8, text), // "{club} qualify for Third Promotion"
            _ => exe::comp_build_fate_news_text(this, club_id, outcome, 3, a4, fixture, a6, a7, a8, body, text),
        }
        return;
    }
    if body != 0 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, a7, a8, body, text);
    }
    covered!("sbD_8 argentine second division play-off text");
    let format = match (a7 == read_u32(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION), o == 1) {
        (true, true) => 0x987594,   // "{club} avoid relegation"
        (true, false) => 0x987564,  // "{club} relegated"
        (false, true) => 0x9876cc,  // "{club} win promotion"
        (false, false) => 0x987528, // "{club} miss out on promotion"
    };
    club_fate_line(club_id, format, text);
}

/// Comparer for `arg_second_update_staff_records_for_status`: only the original (it adds the records); the copy's
/// plan, made before, must give the same return and the same row statuses.
pub(crate) unsafe extern "thiscall" fn compare_arg_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> i32 {
    let p = arg_second_status_plan(League(this), club, status as u8, stage as u8 as i8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> i32>(%%I0%%)(this, club, status, stage, a4, a5, a6);
    let rows_ok = p.rows[..p.n].iter().all(|&(row, s)| LeagueTableRow(row).status() == s);
    let expected = if (stage as u8 as i8) < 3 { 0 } else { p.result };
    SCOREBOARD.record(r == expected && rows_ok, || format!("arg_second_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {:#x}): original {} copy {} rows {:x?}", this, club, status as u8, stage as u8, r, expected, &p.rows[..p.n]));
    r
}

/// Comparer for `arg_second_build_fate_news_text`: the league's own texts side by side (`compare_text_writer`); the
/// common text only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_arg_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I1%%);
    let s = stage as u8 as i8;
    let own_text = match s {
        -1 => false,
        s if s < 3 => true,
        3 => matches!(outcome as u8, 1 | 2),
        _ => body == 0,
    };
    if text == 0 || !own_text {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text),
        |t| arg_second_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, t),
        || format!("arg_second_build_fate_news_text(club {}, outcome {}, stage {}, body {})", club_id, outcome as u8, s, body));
}
