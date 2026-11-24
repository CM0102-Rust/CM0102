// ---------- cx12_7: the J1 League staff records by outcome, fate news text and club progress (jap_j1.cpp) ----------

/// The staff record a J1 League outcome adds.
#[derive(Clone, Copy, PartialEq, Debug)]
enum J1Record {
    None,
    /// A stage won (Staff_Records::add_qualified_for_round_record with 0, 0x96, 0x28).
    StageWinner,
    Champions,
    /// The championship final lost (Staff_Records::add_knocked_out_record with 0, 0x96, 0x50).
    FinalLoser,
}

/// What `jap_j1_update_staff_records_for_status` does for an outcome: the record, the status of the club's row in the
/// main table (`row`, 0 when the club has none), whether the club's rows in the two stage tables also get status 5, the
/// new `league::STAGE_COUNT`, whether the caller's flag is set, and the value returned.
#[derive(Clone, Copy, Debug)]
struct J1StatusPlan {
    record: J1Record,
    row: usize,
    status: Option<u8>,
    stage_rows: bool,
    stage_count: Option<usize>,
    flag: bool,
    result: u32,
}

/// The plan of the J1 League status update (no side effects). The stage and the status are bytes.
///
/// Stage 0 (the first stage): status 6 (stage won) is the stage winner record, the flag and the club's main row status
/// 6; 0. Stage 1 (the second stage), whatever the status: when the club also topped the first stage (row 0 of the first
/// stage's table), it is champion with no final: its main row and its rows in both stage tables get status 5, the
/// champions record, `league::STAGE_COUNT` = 2 and the flag; 1. Otherwise the stage winner record, the flag and the
/// main row gets the status passed; 2. Stage 2 (the championship final): status 1 (final won) is the champions record,
/// the same status 5 rows and the flag; 0; status 2 (final lost) is the final loser record; 0. Anything else: 0.
unsafe fn j1_status_plan(l: League, club: usize, status: u8, stage: u8) -> J1StatusPlan {
    let row = find_club_row(l.addr(), club, 0);
    let plan = |record, status: Option<u8>, stage_rows, stage_count, flag, result| J1StatusPlan { record, row, status, stage_rows, stage_count, flag, result };
    let nothing = plan(J1Record::None, None, false, None, false, 0);
    match stage {
        0 if status == 6 => plan(J1Record::StageWinner, Some(6), false, None, true, 0),
        1 => {
            let first_stage = League(read_usize(l.stages()));
            if read_usize(first_stage.table()) == club { // cx12_7: the first stage winner too
                plan(J1Record::Champions, Some(5), true, Some(2), true, 1)
            } else {
                plan(J1Record::StageWinner, Some(status), false, None, true, 2)
            }
        }
        2 if status == 1 => plan(J1Record::Champions, Some(5), true, None, true, 0),
        2 if status == 2 => plan(J1Record::FinalLoser, None, false, None, false, 0),
        _ => nothing,
    }
}

/// Updates the staff records by a club's outcome in the J1 League (`j1_status_plan`): the club's rows first, then the
/// record on the Staff_Records object (`G_STAFF_RECORDS`) with this competition, `league::STAGE_COUNT` and the byte
/// `flag` points to (set to 1). The rows of the two stage tables are scanned with the main table's club count. Returns
/// the plan's value.
/// Original: `JapJ1::update_staff_records_for_status` (0x663db0, jap_j1.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn jap_j1_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, flag: usize, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let p = j1_status_plan(l, club, status as u8, stage as u8);
    if let (Some(s), true) = (p.status, p.row != 0) {
        LeagueTableRow(p.row).set_status(s);
    }
    if p.stage_rows {
        covered!("cx12_7 j1 champions rows");
        for k in 0..l.club_count().max(0) as usize {
            for stage_league in [read_usize(l.stages()), read_usize(l.stages() + 4)] {
                let row = LeagueTableRow(League(stage_league).table() + k * league_table_row::SIZE);
                if row.club() as usize == club {
                    row.set_status(5);
                }
            }
        }
    }
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match p.record {
        J1Record::None => {}
        J1Record::StageWinner => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0, 0x96, 0x28),
        J1Record::Champions => exe::staff_records_add_league_champions_record(records, club, comp),
        J1Record::FinalLoser => exe::staff_records_add_knocked_out_record(records, club, comp, 0, 0x96, 0x50),
    }
    if let Some(n) = p.stage_count {
        l.set_stage_count(n);
    }
    if p.flag {
        write_u8(flag, 1);
    }
    p.result
}

/// "{club} win first stage", "{club} win second stage": the short texts, with the club only.
const WIN_FIRST_STAGE_TEXT: usize = 0x9eae2c;
const WIN_SECOND_STAGE_TEXT: usize = 0x9eacc0;
/// "{club} have won the first stage of the {league}.", "{club} have won the {league} by finishing top of both
/// stages.", "{club} have qualified for the final of the {league} having won the second stage of the competition.":
/// the long texts.
const WON_FIRST_STAGE_TEXT: usize = 0x9eae64;
const WON_BOTH_STAGES_TEXT: usize = 0x9eada8;
const REACHED_FINAL_TEXT: usize = 0x9eacfc;
/// "{club} win {league}", "{club} lose {league} final".
const WIN_LEAGUE_TEXT: usize = 0x9eac64;
const LOSE_FINAL_TEXT: usize = 0x9eac00;

/// Writes a J1 fate text into `text`: `template` (langlib_sprintf into the shared buffer) with the club's gender
/// (`club::SHORT_GENDER_NAME`, signed) twice, then, with `with_league`, the competition's gender twice, then the club's
/// short name and, with `with_league`, the competition's short name; set as the text, with the club's link (0x7d5) and
/// the competition's (0x7d0) in both forms.
unsafe fn j1_fate_text(club_id: u32, comp: usize, template: usize, with_league: bool, text: usize) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    if with_league {
        let cg = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        story_lang!(PRAISE_TEXT_BUFFER, template, g, g, cg, cg, name, comp + competition::SHORT_NAME);
    } else {
        story_lang!(PRAISE_TEXT_BUFFER, template, g, g, name);
    }
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, name as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// The text (template, with the league) a J1 fate gets, or None for the base text or nothing (see
/// `jap_j1_build_fate_news_text`).
fn j1_fate_template(stage: u8, outcome: u8, update_result: u32, news: bool) -> Option<(usize, bool)> {
    match (stage, outcome) {
        (0, 6) if news => Some((WON_FIRST_STAGE_TEXT, true)),
        (0, 6) => Some((WIN_FIRST_STAGE_TEXT, false)),
        (1, 6) => match (update_result, news) {
            (1, true) => Some((WON_BOTH_STAGES_TEXT, true)),
            (1, false) => Some((WIN_LEAGUE_TEXT, true)),
            (2, true) => Some((REACHED_FINAL_TEXT, true)),
            (2, false) => Some((WIN_SECOND_STAGE_TEXT, false)),
            _ => None,
        },
        (2, 1) if !news => Some((WIN_LEAGUE_TEXT, true)), // cx12_7: the final won
        (2, _) if !news => Some((LOSE_FINAL_TEXT, true)),
        _ => None,
    }
}

/// Writes the news text of a club's fate in the J1 League. Stage 0xff (the whole season): the base text
/// (`exe::comp_build_fate_news_text`) with stage −1 and a7 = 0. Stage 0, outcome 6: the first stage won (the long text
/// with `news`, else the short one). Stage 1, outcome 6, by the status update's result (`update_result`, the 7th
/// argument): 1, top of both stages (the long text with `news`, else "win {league}"); 2, the final reached (the long text
/// with `news`, else "win second stage"). Stage 2 with `news`: the base text with the outcome (1 when it is 1), stage 2
/// and a7 = 0; without it, outcome 1 "win {league}", any other "lose {league} final". Everything else writes nothing.
/// The stage and the outcome are compared as bytes.
/// Original: `JapJ1::build_fate_news_text` (0x6640a0, jap_j1.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn jap_j1_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, update_result: u32, a8: u32, news: u32, text: usize) {
    let comp = League(this).competition().addr();
    let (s, o) = (stage as u8, outcome as u8);
    if s == 0xff {
        return exe::comp_build_fate_news_text(this, club_id, outcome, -1, a4, fixture, a6, 0, a8, news, text);
    }
    if s == 2 && news != 0 {
        let outcome = if o == 1 { 1 } else { outcome };
        return exe::comp_build_fate_news_text(this, club_id, outcome, 2, a4, fixture, a6, 0, a8, news, text);
    }
    if let Some((template, with_league)) = j1_fate_template(s, o, update_result, news != 0) {
        covered!("cx12_7 j1 own fate text");
        j1_fate_text(club_id, comp, template, with_league, text);
    }
}

/// The club's place forecast in the J1 League: its entry in the competition statistics (`exe::comp_stats_find_club_entry`,
/// unchecked) gets the current, best and worst codes. At the stage byte 0xff (the whole season), a club whose main table
/// row has status 1 writes nothing; otherwise best 2..=5 becomes 2 and worst 2..=5 becomes 5. Other stages write the
/// codes as they come.
///
/// Defect of the original: the row status is kept in the stack slot of the `club` argument (its low byte); without a
/// row in the table that byte is the low byte of the club pointer, so a club with no row whose pointer ends in 0x01
/// writes nothing. The copy reproduces it.
/// Original: `JapJ1::set_club_progress` (0x664400, jap_j1.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn jap_j1_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let l = League(this);
    let e = exe::comp_stats_find_club_entry(l.comp_stats(), club);
    let row = find_club_row(this, club, 0);
    let status = if row != 0 { LeagueTableRow(row).status() } else { club as u8 };
    let (c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if stage as u8 == 0xff {
        if status == 1 {
            return;
        }
        covered!("cx12_7 j1 season progress");
        if (2..=5).contains(&b) {
            b = 2;
        }
        if (2..=5).contains(&w) { // cx12_7: the worst code of the band
            w = 5;
        }
    }
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `jap_j1_update_staff_records_for_status`: only the original (it adds the records); the copy's plan,
/// made before, must give the same return, the same main row status, the stage count and the flag.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, flag: usize, a5: u32, a6: u32) -> u32 {
    let l = League(this);
    let p = j1_status_plan(l, club, status as u8, stage as u8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(%%I0%%)(this, club, status, stage, flag, a5, a6);
    let row_ok = match (p.status, p.row) {
        (Some(s), row) if row != 0 => LeagueTableRow(row).status() == s,
        _ => true,
    };
    let count_ok = p.stage_count.map_or(true, |n| l.stage_count() == n);
    let flag_ok = !p.flag || read_u8(flag) == 1;
    SCOREBOARD.record(r == p.result && row_ok && count_ok && flag_ok, || format!("jap_j1_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}): original {} copy {:?}", this, club, status as u8, stage as u8, r, p));
    r
}

/// Comparer for `jap_j1_build_fate_news_text`: the text-only paths side by side (`compare_text_writer`; with the
/// CM_TEXTO_NOTICIAS mode); the base text only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, update_result: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I1%%);
    if text == 0 || j1_fate_template(stage as u8, outcome as u8, update_result, news != 0).is_none() {
        return o(this, club_id, outcome, stage, a4, fixture, a6, update_result, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, update_result, a8, news, text),
        |t| jap_j1_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, update_result, a8, news, t),
        || format!("jap_j1_build_fate_news_text(club {}, outcome {}, stage {}, result {}, news {})", club_id, outcome as u8, stage as u8 as i8, update_result, news));
}

/// Comparer for `jap_j1_set_club_progress`: side by side on the 3 bytes of the club's entry (`compare_c1_club_progress`).
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_c1_club_progress("jap_j1_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || jap_j1_set_club_progress(this, club, stage, current, best, worst))
}
