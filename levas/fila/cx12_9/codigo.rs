// ---------- cx12_9: the Northern Irish First Division staff records, fate news text and club progress (nir_first.cpp) ----------

/// The divisions of a First Division whose champion plays the Premier Division's second last for a place (Northern
/// Ireland, Norway): the database keys of the Premier and the First Division, the key whose competition record names
/// the relegation of a Premier club that loses the play-off, and the progress codes of the play-off (a Premier club's
/// best, worst and current code 1 and 2 become `premier_codes`; the league band starts at `band`).
pub(crate) struct FirstDivisionPlayOff {
    pub(crate) premier: usize,
    pub(crate) first: usize,
    pub(crate) relegated_from: usize,
    pub(crate) band: u8,
    pub(crate) premier_codes: [u8; 2],
}

/// The Northern Irish First Division and Premier Division. Defect of the original: the relegation record of a Premier
/// club that loses the play-off names the Republic of Ireland's Premier Division (`KEY_CLUB_COMP_IRISH_PREMIER_DIVISION`)
/// instead of the Northern Irish one.
pub(crate) const NORTHERN_IRISH_PLAY_OFF_DIVISIONS: FirstDivisionPlayOff = FirstDivisionPlayOff {
    premier: KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION,
    first: KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION,
    relegated_from: KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, // cx12_9: the original's key (the Republic's)
    band: 2,
    premier_codes: [8, 0xa],
};

/// The staff record a First Division outcome adds.
#[derive(Clone, Copy, PartialEq, Debug)]
enum FirstDivisionRecord {
    None,
    Champions,
    /// Promotion with the given weight (0x64 for the title race, 0x32 through the play-off).
    Promotion(u32),
    /// A play-off place (Staff_Records::add_qualified_for_round_record with 0, 0xa0, 0x28).
    PlayOffPlace,
    /// Relegation from the given competition record.
    Relegation(usize),
}

/// What the First Division status update does: the record, the row (0 for none) and its new status, and the value
/// returned.
#[derive(Clone, Copy, Debug)]
struct FirstDivisionPlan {
    record: FirstDivisionRecord,
    row: usize,
    status: Option<u8>,
    result: u32,
}

/// The club's row in a league table (`league::CLUB_COUNT` rows, signed), or 0.
unsafe fn first_division_table_row(l: League, club: usize) -> usize {
    (0..l.club_count().max(0) as usize).map(|k| l.table() + k * league_table_row::SIZE).find(|&r| LeagueTableRow(r).club() as usize == club).unwrap_or(0)
}

/// The plan of the First Division status update (no side effects); the status and the stage are bytes.
///
/// Stage 0xff (the league): status 5 is the title, 0 promotion (weight 0x64), 1 a play-off place, 3 relegation from this
/// division; 0. Any other stage (the play-off): a First Division club (its `club::DIVISION` id is the First's) in this
/// table (none: 0) goes up with status 1 (promotion weight 0x32, row status 0) or stays (row 0xfe), returning the First
/// Division's id; any other club, in the Premier Division's table (none: 0), stays up with status 1 (row 0xfe) or goes
/// down (relegation from the `relegated_from` record, row status 3), returning the Premier Division's id.
unsafe fn first_division_status_plan(l: League, club: usize, status: u8, stage: u8, d: &FirstDivisionPlayOff) -> FirstDivisionPlan {
    let plan = |record, row, status, result| FirstDivisionPlan { record, row, status, result };
    if stage == 0xff {
        let record = match status {
            5 => FirstDivisionRecord::Champions,
            0 => FirstDivisionRecord::Promotion(0x64),
            1 => FirstDivisionRecord::PlayOffPlace,
            3 => FirstDivisionRecord::Relegation(l.competition().addr()),
            _ => FirstDivisionRecord::None,
        };
        return plan(record, 0, None, 0);
    }
    let first_id = read_u32(d.first);
    if read_u32(Club(club).division().addr()) == first_id {
        let row = first_division_table_row(l, club);
        if row == 0 {
            return plan(FirstDivisionRecord::None, 0, None, 0);
        }
        return if status == 1 {
            plan(FirstDivisionRecord::Promotion(0x32), row, Some(0), first_id)
        } else {
            plan(FirstDivisionRecord::None, row, Some(0xfe), first_id)
        };
    }
    let premier_id = read_u32(d.premier);
    let row = first_division_table_row(League(comp_object_of_key(d.premier)), club);
    if row == 0 {
        return plan(FirstDivisionRecord::None, 0, None, 0);
    }
    if status == 1 {
        plan(FirstDivisionRecord::None, row, Some(0xfe), premier_id)
    } else {
        let record = read_usize(G_CLUB_COMP_TABLE) + read_usize(d.relegated_from) * 0x6b; // the competition record
        plan(FirstDivisionRecord::Relegation(record), row, Some(3), premier_id) // cx12_9: the Premier club goes down
    }
}

/// The body of the First Division status updates (`first_division_status_plan`): the record on the Staff_Records object
/// (`G_STAFF_RECORDS`) with this competition unless named, then the row status; returns the plan's value.
pub(crate) unsafe fn first_division_update_staff_records(this: usize, club: usize, status: u32, stage: u32, d: &FirstDivisionPlayOff) -> u32 {
    let l = League(this);
    let p = first_division_status_plan(l, club, status as u8, stage as u8, d);
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match p.record {
        FirstDivisionRecord::None => {}
        FirstDivisionRecord::Champions => exe::staff_records_add_league_champions_record(records, club, comp),
        FirstDivisionRecord::Promotion(weight) => {
            covered!("cx12_9 first division promotion");
            exe::staff_records_add_promotion_record(records, club, comp, weight);
        }
        FirstDivisionRecord::PlayOffPlace => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0, 0xa0, 0x28),
        FirstDivisionRecord::Relegation(from) => {
            covered!("cx12_9 relegation record");
            exe::staff_records_add_relegation_record(records, club, from);
        }
    }
    if let Some(s) = p.status {
        LeagueTableRow(p.row).set_status(s);
    }
    p.result
}

/// The comparer of the First Division status updates: only the original (it adds the records); the plan, made before,
/// must give the same return and row status.
pub(crate) unsafe fn compare_first_division_update_staff_records(name: &str, this: usize, club: usize, status: u32, stage: u32, d: &FirstDivisionPlayOff, original: impl FnOnce() -> u32) -> u32 {
    let p = first_division_status_plan(League(this), club, status as u8, stage as u8, d);
    let r = original();
    let row_ok = p.status.map_or(true, |s| LeagueTableRow(p.row).status() == s);
    SCOREBOARD.record(r == p.result && row_ok, || format!("{}({:#x}, {:#x}, status {}, stage {}): original {:#x} copy {:?}", name, this, club, status as u8, stage as u8, r, p));
    r
}

/// Updates the staff records by a club's outcome in the Northern Irish First Division and its play-off against the
/// Premier Division (`first_division_update_staff_records` with `NORTHERN_IRISH_PLAY_OFF_DIVISIONS`).
/// Original: `NirFirst::update_staff_records_for_status` (0x78c300, nir_first.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn nir_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    first_division_update_staff_records(this, club, status, stage, &NORTHERN_IRISH_PLAY_OFF_DIVISIONS)
}

/// "{club} avoid relegation", "{club} win promotion", "{club} relegated".
const PLAY_OFF_AVOID_RELEGATION_TEXT: usize = 0x987594;
const PLAY_OFF_WIN_PROMOTION_TEXT: usize = 0x989c18;
const PLAY_OFF_RELEGATED_TEXT: usize = 0x989d78;

/// The play-off text a fate gets (`first_division_fate_news_text`), or None for the base text.
fn first_division_fate_template(outcome: u8, stage: u8, premier_club: bool) -> Option<usize> {
    match (outcome, stage, premier_club) {
        (1, 0, true) => Some(PLAY_OFF_AVOID_RELEGATION_TEXT),
        (1, 0, false) => Some(PLAY_OFF_WIN_PROMOTION_TEXT), // cx12_9: the First Division club goes up
        (_, _, true) => Some(PLAY_OFF_RELEGATED_TEXT),
        _ => None,
    }
}

/// The body of the First Division fate news texts. With a news item (`news`) the base text first
/// (`exe::comp_build_fate_news_text` with a7 = 0), and then, without returning: when `division` (the 7th argument, the
/// status update's result) is the Premier Division's id, outcome 1 in the play-off (stage 0) "{club} avoid relegation"
/// and any other "{club} relegated"; otherwise outcome 1 at stage 0 "{club} win promotion" (`group_qualify_text`: the
/// club's gender twice and short name, set as the text with the club's link) and anything else the base text again.
///
/// Defect of the original: with a news item and a fate of none of the play-off texts, the base text runs twice; with
/// a play-off text, the base text is written first and then replaced.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn first_division_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, division: u32, a8: u32, news: u32, text: usize, d: &FirstDivisionPlayOff) {
    if news != 0 {
        exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, news, text);
    }
    match first_division_fate_template(outcome as u8, stage as u8, division == read_u32(d.premier)) {
        Some(template) => {
            covered!("cx12_9 play-off fate text");
            group_qualify_text(club_id, template, text)
        }
        None => exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, news, text),
    }
}

/// The comparer of the First Division fate texts: without a news item, on the play-off texts, side by side
/// (`compare_text_writer`); the other paths only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_first_division_fate_news_text(name: &str, this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, division: u32, a8: u32, news: u32, text: usize,
        d: &FirstDivisionPlayOff, original: impl Fn(usize)) {
    if news != 0 || text == 0 || first_division_fate_template(outcome as u8, stage as u8, division == read_u32(d.premier)).is_none() {
        return original(text);
    }
    compare_text_writer(text, || original(text), |t| first_division_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, division, a8, news, t, d),
        || format!("{}(club {}, outcome {}, stage {}, division {})", name, club_id, outcome as u8, stage as u8, division));
}

/// Writes the news text of a club's fate in the Northern Irish First Division and its play-off
/// (`first_division_fate_news_text` with `NORTHERN_IRISH_PLAY_OFF_DIVISIONS`).
/// Original: `NirFirst::build_fate_news_text` (0x78c4f0, nir_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn nir_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, division: u32, a8: u32, news: u32, text: usize) {
    first_division_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, division, a8, news, text, &NORTHERN_IRISH_PLAY_OFF_DIVISIONS)
}

/// The body of the First Division club progress: the club's entry in this competition's statistics
/// (`exe::comp_stats_find_club_entry`, unchecked) gets the current, best and worst codes. Stage 0xff (the league):
/// best `band` becomes `band` − 1 and worst `band` becomes `band` + 1. Stage 0 (the play-off): a Premier Division club
/// (its `club::DIVISION` id is the Premier's) writes into its entry in the Premier Division's statistics, each code 1
/// becoming `premier_codes[0]` and 2 `premier_codes[1]`; a First Division club doubles its three codes. Other stages:
/// the codes as they come.
pub(crate) unsafe fn first_division_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, d: &FirstDivisionPlayOff) {
    let mut e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    match stage as u8 {
        0xff => {
            if b == d.band {
                b = d.band - 1;
            }
            if w == d.band {
                w = d.band + 1;
            }
        }
        0 if read_u32(Club(club).division().addr()) == read_u32(d.premier) => {
            covered!("cx12_9 premier club play-off progress");
            e = exe::comp_stats_find_club_entry(League(comp_object_of_key(d.premier)).comp_stats(), club);
            let map = |v: u8| match v {
                1 => d.premier_codes[0],
                2 => d.premier_codes[1],
                _ => v,
            };
            (c, b, w) = (map(c), map(b), map(w));
        }
        0 => {
            covered!("cx12_9 first division club play-off progress");
            (c, b, w) = (c << 1, b << 1, w << 1); // cx12_9: the First Division club doubles its codes
        }
        _ => {}
    }
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// The club's progress codes in the Northern Irish First Division (`first_division_set_club_progress` with
/// `NORTHERN_IRISH_PLAY_OFF_DIVISIONS`: the league band 2, best 2 → 1 and worst 2 → 3; a Premier club in the play-off 1 → 8 and
/// 2 → 10, in the Premier's statistics).
/// Original: `NirFirst::set_club_progress` (0x78c670, nir_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn nir_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    first_division_set_club_progress(this, club, stage, current, best, worst, &NORTHERN_IRISH_PLAY_OFF_DIVISIONS)
}

/// The comparer of the First Division club progress: side by side on the 3 bytes of the entry written (the Premier
/// Division's for a Premier club in the play-off).
pub(crate) unsafe fn compare_first_division_club_progress(name: &str, this: usize, club: usize, stage: u32, d: &FirstDivisionPlayOff, original: impl FnOnce(), copy: impl FnOnce()) {
    let premier_club = stage as u8 == 0 && read_u32(Club(club).division().addr()) == read_u32(d.premier);
    let stats = if premier_club { League(comp_object_of_key(d.premier)).comp_stats() } else { League(this).comp_stats() };
    let e = exe::comp_stats_find_club_entry(stats, club);
    if e == 0 {
        return original();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}, {:#x}, stage {}) original={:x?} rust={:x?}", name, this, club, stage as u8, mo, mr));
}

/// Comparer for `nir_first_update_staff_records_for_status` (`compare_first_division_update_staff_records`).
pub(crate) unsafe extern "thiscall" fn compare_nir_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    compare_first_division_update_staff_records("nir_first_update_staff_records_for_status", this, club, status, stage, &NORTHERN_IRISH_PLAY_OFF_DIVISIONS, || o(this, club, status, stage, a4, a5, a6))
}

/// Comparer for `nir_first_build_fate_news_text` (`compare_first_division_fate_news_text`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_nir_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, division: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I1%%);
    compare_first_division_fate_news_text("nir_first_build_fate_news_text", this, club_id, outcome, stage, a4, fixture, a6, division, a8, news, text, &NORTHERN_IRISH_PLAY_OFF_DIVISIONS,
        |t| o(this, club_id, outcome, stage, a4, fixture, a6, division, a8, news, t));
}

/// Comparer for `nir_first_set_club_progress` (`compare_first_division_club_progress`).
pub(crate) unsafe extern "thiscall" fn compare_nir_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_first_division_club_progress("nir_first_set_club_progress", this, club, stage, &NORTHERN_IRISH_PLAY_OFF_DIVISIONS, || o(this, club, stage, current, best, worst),
        || nir_first_set_club_progress(this, club, stage, current, best, worst));
}
