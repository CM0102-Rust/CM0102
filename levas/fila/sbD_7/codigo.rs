// ---------- sbD_7: the Belgian Second Division staff records and fate news (bel_second.cpp) ----------

/// The staff record an outcome of the Belgian Second Division adds (promotion with its weight).
#[derive(Clone, Copy, PartialEq, Debug)]
enum BelSecondRecord {
    None,
    Champions,
    Promotion(u32),
    Qualified,
    Relegation,
}

/// Which of the club's table rows get a status: none, every one of them, or only the first.
#[derive(Clone, Copy, PartialEq, Debug)]
enum BelSecondRows {
    None,
    Every(u8),
    First(u8),
}

/// What `bel_second_update_staff_records_for_status` does for an outcome: the status written on the club's table
/// rows, the staff record and how many times it is added, and the value returned.
struct BelSecondStatusPlan {
    rows: BelSecondRows,
    record: BelSecondRecord,
    times: usize,
    result: i32,
}

/// The table rows (`league::TABLE`, 0x3b bytes each) whose club is `club`, in order.
unsafe fn bel_second_rows_of(this: League, club: usize) -> impl Iterator<Item = usize> {
    (0..this.club_count().max(0) as usize).map(move |r| this.table() + r * 0x3b).filter(move |&row| read_usize(row) == club)
}

/// The plan of the Belgian Second Division status update (no side effects):
/// - main stage (byte 0xff): status 5 the champions record, 0 promotion (weight 0x64), 1 the qualified record, 3
///   relegation; 0 returned;
/// - stages 0 to 2 (and any other negative byte): −1;
/// - stage 3, the promotion play-off group: with status 0 every table row of the club gets row status 0 and a
///   promotion record (weight 0x32) each, 0 returned; with another status the first row of the club gets 0xff and −1
///   is returned (no row: 0);
/// - stage 4, the relegation play-off: only for clubs of this division (`club::DIVISION` = this competition), else −1;
///   status 1 with the match's +0x34 word at 2: the club's first row 0xfe; status 2 or 0xff: that row (if any) 3 and
///   the relegation record; 0 returned;
/// - stages above 4: 0.
unsafe fn bel_second_status_plan(this: League, club: usize, status: u8, stage: u8, fixture: usize) -> BelSecondStatusPlan {
    let plan = |rows: BelSecondRows, record: BelSecondRecord, times: usize, result: i32| BelSecondStatusPlan { rows, record, times, result };
    match stage {
        0xff => {
            let record = match status {
                5 => BelSecondRecord::Champions,
                0 => BelSecondRecord::Promotion(0x64),
                1 => BelSecondRecord::Qualified,
                3 => BelSecondRecord::Relegation,
                _ => BelSecondRecord::None,
            };
            plan(BelSecondRows::None, record, 1, 0)
        }
        s if (s as i8) < 3 => plan(BelSecondRows::None, BelSecondRecord::None, 0, -1),
        3 => {
            let n = bel_second_rows_of(this, club).count();
            match status {
                0 => plan(BelSecondRows::Every(0), BelSecondRecord::Promotion(0x32), n, 0),
                _ if n > 0 => plan(BelSecondRows::First(0xff), BelSecondRecord::None, 0, -1),
                _ => plan(BelSecondRows::None, BelSecondRecord::None, 0, 0),
            }
        }
        4 => {
            if Club(club).division().addr() != this.competition().addr() {
                return plan(BelSecondRows::None, BelSecondRecord::None, 0, -1);
            }
            match status {
                1 if read_u16(fixture + 0x34) == 2 => plan(BelSecondRows::First(0xfe), BelSecondRecord::None, 0, 0), // +0x34: the match's leg count [inference]
                2 | 0xff => plan(BelSecondRows::First(3), BelSecondRecord::Relegation, 1, 0),
                _ => plan(BelSecondRows::None, BelSecondRecord::None, 0, 0),
            }
        }
        _ => plan(BelSecondRows::None, BelSecondRecord::None, 0, 0),
    }
}

/// Whether the club's rows carry the statuses of the plan.
unsafe fn bel_second_rows_hold(this: League, club: usize, rows: BelSecondRows) -> bool {
    match rows {
        BelSecondRows::None => true,
        BelSecondRows::Every(s) => bel_second_rows_of(this, club).all(|row| LeagueTableRow(row).status() == s),
        BelSecondRows::First(s) => bel_second_rows_of(this, club).next().map_or(true, |row| LeagueTableRow(row).status() == s),
    }
}

/// Updates the staff records by a club's outcome in the Belgian Second Division (`bel_second_status_plan`): the records
/// on the Staff_Records object (`G_STAFF_RECORDS`, with this competition): promotion with its weight, the qualified
/// record (0, 0xa0, 0x2d); then the row statuses (`LeagueTableRow::set_status`). Returns the plan's value.
/// Original: `BelSecond::update_staff_records_for_status` (0x420790, bel_second.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn bel_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, fixture: usize, _a6: u32) -> i32 {
    let l = League(this);
    let p = bel_second_status_plan(l, club, status as u8, stage as u8, fixture);
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    for _ in 0..p.times {
        match p.record {
            BelSecondRecord::None => {}
            BelSecondRecord::Champions => exe::staff_records_add_league_champions_record(records, club, comp),
            BelSecondRecord::Promotion(weight) => {
                covered!("sbD_7 belgian second division promotion record");
                exe::staff_records_add_promotion_record(records, club, comp, weight);
            }
            BelSecondRecord::Qualified => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0, 0xa0, 0x2d),
            BelSecondRecord::Relegation => {
                covered!("sbD_7 belgian second division relegation record");
                exe::staff_records_add_relegation_record(records, club, comp);
            }
        }
    }
    match p.rows {
        BelSecondRows::None => {}
        BelSecondRows::Every(s) => bel_second_rows_of(l, club).for_each(|row| LeagueTableRow(row).set_status(s)),
        BelSecondRows::First(s) => {
            if let Some(row) = bel_second_rows_of(l, club).next() {
                LeagueTableRow(row).set_status(s);
            }
        }
    }
    p.result
}

/// A club's one-line fate text: `format` with the club's short-name gender twice and its short name, into the shared
/// buffer [`PRAISE_TEXT_BUFFER`], set into the Text, and the short name linked to the club (0x7d5).
pub(crate) unsafe fn club_fate_line(club_id: u32, format: usize, text: usize) {
    let club = club_at(club_id as i32 as isize);
    let g = club.short_gender_name() as i8 as i32;
    story_lang!(PRAISE_TEXT_BUFFER, format, g, g, club.addr() + club::SHORT_NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
}

/// A club's fate text naming the competition: `format` with the club's short-name gender twice, the competition's
/// name gender twice, the club's short name and the competition's name (the record at `league::COMPETITION`), into
/// [`PRAISE_TEXT_BUFFER`]; links: the club (0x7d5) and the competition (0x7d0).
pub(crate) unsafe fn club_competition_fate_line(this: League, club_id: u32, format: usize, text: usize) {
    let club = club_at(club_id as i32 as isize);
    let comp = this.competition().addr();
    let (cg, kg) = (club.short_gender_name() as i8 as i32, read_i8(comp + competition::GENDER_NAME) as i32);
    story_lang!(PRAISE_TEXT_BUFFER, format, cg, cg, kg, kg, club.addr() + club::SHORT_NAME, comp + competition::NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
}

/// Rule: the news of a club's fate in the Belgian Second Division: in the promotion play-off group (stage 3), promoted
/// ("{club} win promotion", or with `body` the line naming the division) or missing out ("{club} miss out on
/// promotion"); in the relegation play-off (stage 4), avoiding relegation (status 1) or relegated (2 or 0xff), with
/// `body` the cup tie's text; the main stage the competitions' common text.
///
/// `BelSecond::build_fate_news_text(club, outcome, stage, a4, fixture, a6, a7, a8, body, text)`:
/// - stage byte 0xff: Comp::build_fate_news_text (`exe::comp_build_fate_news_text`, the same arguments, stage −1);
/// - stage 3: outcome 0 with `body` 0x989c4c, without it 0x989c18; outcome 0xfe with `body` 0x989b90, without it
///   0x987528; any other outcome nothing (`club_competition_fate_line` with `body`, `club_fate_line` without);
/// - stage 4: with `body` the cup tie text (`exe::comp_util_write_cup_tie_result_news`(club, fixture, a8, outcome, a6,
///   text), outcome 1 passed as 1); without it, outcome 1 "{club} avoid relegation" (0x987594), 2 or 0xff "{club}
///   relegated" (0x987564), any other the cup win or defeat text (`exe::comp_util_write_cup_win_or_defeat_news`(club,
///   fixture, outcome, text));
/// - other stages: nothing.
/// Original: `BelSecond::build_fate_news_text` (0x4209c0, bel_second.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn bel_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, body: u32, text: usize) {
    let l = League(this);
    match stage as u8 {
        0xff => exe::comp_build_fate_news_text(this, club_id, outcome, -1, a4, fixture, a6, a7, a8, body, text),
        3 => {
            let (with_body, headline) = match outcome as u8 {
                0 => (0x989c4c, 0x989c18),    // "{club} win promotion"
                0xfe => (0x989b90, 0x987528), // "{club} miss out on promotion"
                _ => return,
            };
            covered!("sbD_7 belgian second division play-off group text");
            if body != 0 {
                club_competition_fate_line(l, club_id, with_body, text);
            } else {
                club_fate_line(club_id, headline, text);
            }
        }
        4 => {
            let o = outcome as u8;
            if body != 0 {
                let outcome = if o == 1 { 1 } else { outcome };
                return exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
            }
            covered!("sbD_7 belgian second division relegation play-off text");
            match o {
                1 => club_fate_line(club_id, 0x987594, text),        // "{club} avoid relegation"
                2 | 0xff => club_fate_line(club_id, 0x987564, text), // "{club} relegated"
                _ => exe::comp_util_write_cup_win_or_defeat_news(club_id, fixture as u32, outcome, text),
            }
        }
        _ => {}
    }
}

/// Comparer for `bel_second_update_staff_records_for_status`: only the original (it adds the records); the copy's
/// plan, made before, must give the same return and the same row statuses.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, fixture: usize, a6: u32) -> i32 {
    let p = bel_second_status_plan(League(this), club, status as u8, stage as u8, fixture);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> i32>(%%I0%%)(this, club, status, stage, a4, fixture, a6);
    let rows_ok = bel_second_rows_hold(League(this), club, p.rows);
    SCOREBOARD.record(r == p.result && rows_ok, || format!("bel_second_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {:#x}): original {} copy {} {:?} x{} rows {:?}", this, club, status as u8, stage as u8, r, p.result, p.record, p.times, p.rows));
    r
}

/// Comparer for `bel_second_build_fate_news_text`: the league texts (stage 3, and stage 4 without `body` for outcomes
/// 1, 2 and 0xff) side by side (`compare_text_writer`); the common text and the cup tie texts only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_bel_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I1%%);
    let own_text = match stage as u8 {
        3 => true,
        4 => body == 0 && matches!(outcome as u8, 1 | 2 | 0xff),
        _ => false,
    };
    if text == 0 || !own_text {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text),
        |t| bel_second_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, t),
        || format!("bel_second_build_fate_news_text(club {}, outcome {}, stage {}, body {})", club_id, outcome as u8, stage as u8 as i8, body));
}
