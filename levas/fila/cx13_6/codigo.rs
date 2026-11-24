// ---------- cx13_6: the Polish Second Division's staff records, fate news text and club progress (pol_second.cpp) ----------

/// The club competition record of the competition whose index is at `key` (`G_CLUB_COMP_TABLE`, 0x6b bytes each).
pub(crate) unsafe fn club_competition_of_key(key: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(0x6b))
}

/// The row of `club` in a table of `n` rows (signed word), or None.
pub(crate) unsafe fn table_row_of_club(table: usize, n: i16, club: usize) -> Option<usize> {
    (0..n.max(0) as usize).map(|k| table + k * league_table_row::SIZE).find(|&row| read_usize(row) == club)
}

/// A lower division whose clubs play a promotion play-off against clubs of the division above (the Polish Second
/// Division against the First, the Scottish First against the Premier).
pub(crate) struct PlayOffDivision {
    /// the key of the lower division (its own clubs)
    pub(crate) own_key: usize,
    /// the key of the division above
    pub(crate) upper_key: usize,
    /// the stage of the division above whose table holds its play-off clubs (None: its own table)
    pub(crate) upper_stage: Option<usize>,
    /// the status of a play-off winner
    pub(crate) winner: u8,
    /// the last three arguments of the season's play-off place record (Staff_Records::add_qualified_for_round_record)
    pub(crate) qualified: (u32, u32, u32),
}

/// The Polish Second Division and its play-off against the First Division's final stage.
pub(crate) const POLISH_SECOND_PLAY_OFF: PlayOffDivision = PlayOffDivision {
    own_key: KEY_CLUB_COMP_POLISH_SECOND_DIVISION,
    upper_key: KEY_CLUB_COMP_POLISH_FIRST_DIVISION,
    upper_stage: Some(1),
    winner: 1,
    qualified: (0, 0xa0, 0x28),
};

/// What the staff records routine of a play-off division does (no side effects): the record and the competition
/// record it names, the row whose status it writes with the status, and the result.
pub(crate) struct PlayOffRecordPlan {
    record: Option<(PlayOffRecord, usize)>,
    row: Option<(usize, u8)>,
    result: u32,
}

/// A staff record of a play-off division's statuses.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum PlayOffRecord {
    Champions,
    Promotion(u32),
    Qualified,
    Relegation,
}

/// The plan of the staff records routine of a play-off division `d`. In the season (stage 0xff), by status: 5 the
/// title, 0 promotion (weight 0x64), 1 the play-off place (the qualified record with `d.qualified`), 3 relegation, all in
/// the division's own competition; the result 0. In the play-off (any other stage), for a club of the division (its
/// division is `d.own_key`) found in its table: the winner (`d.winner`), promotion with weight 0x32 and the row's
/// status 0; otherwise the status 0xfe. For a club of the division above found in the upper table (`d.upper_stage` of
/// the upper division, or its own table): the winner, the status 0xfe (stays up); otherwise relegation from the upper
/// division's competition and the status 3. The play-off result is the value of the club's own key (`own_key` or
/// `upper_key`); a club not found gives 0 and nothing else.
pub(crate) unsafe fn play_off_record_plan(this: League, club: Club, status: u8, stage: u8, d: &PlayOffDivision) -> PlayOffRecordPlan {
    let comp = this.competition().addr();
    let none = PlayOffRecordPlan { record: None, row: None, result: 0 };
    if stage == 0xff {
        let record = match status {
            5 => PlayOffRecord::Champions,
            0 => PlayOffRecord::Promotion(0x64),
            1 => PlayOffRecord::Qualified,
            3 => PlayOffRecord::Relegation,
            _ => return none,
        };
        return PlayOffRecordPlan { record: Some((record, comp)), ..none };
    }
    let upper = League(comp_by_global(d.upper_key));
    let upper = match d.upper_stage {
        Some(k) => League(read_usize(upper.stages() + 4 * k)),
        None => upper,
    };
    if read_usize(club.division().addr()) == read_usize(d.own_key) {
        let Some(row) = table_row_of_club(this.table(), this.club_count() as i16, club.addr()) else { return none };
        let result = read_u32(d.own_key);
        if status == d.winner {
            return PlayOffRecordPlan { record: Some((PlayOffRecord::Promotion(0x32), comp)), row: Some((row, 0)), result };
        }
        return PlayOffRecordPlan { record: None, row: Some((row, 0xfe)), result };
    }
    let Some(row) = table_row_of_club(upper.table(), upper.club_count() as i16, club.addr()) else { return none };
    let result = read_u32(d.upper_key);
    if status == d.winner {
        return PlayOffRecordPlan { record: None, row: Some((row, 0xfe)), result }; // cx13_6: the upper division club stays up
    }
    PlayOffRecordPlan { record: Some((PlayOffRecord::Relegation, club_competition_of_key(d.upper_key))), row: Some((row, 3)), result }
}

/// Carries out a play-off division's staff records plan into the game's staff records (`G_STAFF_RECORDS`): the title
/// (`exe::staff_records_add_league_champions_record`), promotion (`exe::staff_records_add_promotion_record` with the
/// weight), the play-off place (`exe::staff_records_add_qualified_for_round_record` with `d.qualified`) or relegation
/// (`exe::staff_records_add_relegation_record`); then the row's status (`league_table_row::STATUS`). Returns the result.
pub(crate) unsafe fn play_off_update_staff_records(this: usize, club: usize, status: u32, stage: u32, d: &PlayOffDivision) -> u32 {
    let plan = play_off_record_plan(League(this), Club(club), status as u8, stage as u8, d);
    let records = read_usize(G_STAFF_RECORDS);
    if let Some((record, comp)) = plan.record {
        match record {
            PlayOffRecord::Champions => exe::staff_records_add_league_champions_record(records, club, comp),
            PlayOffRecord::Promotion(weight) => exe::staff_records_add_promotion_record(records, club, comp, weight),
            PlayOffRecord::Qualified => exe::staff_records_add_qualified_for_round_record(records, club, comp, d.qualified.0, d.qualified.1, d.qualified.2),
            PlayOffRecord::Relegation => exe::staff_records_add_relegation_record(records, club, comp),
        }
    }
    if let Some((row, s)) = plan.row {
        covered!("cx13_6 play-off status");
        write_u8(row + league_table_row::STATUS, s);
    }
    plan.result
}

/// Comparer of a play-off division's staff records routine: only the original runs (it adds records); its result and
/// the status of the planned row against the plan.
pub(crate) unsafe fn compare_play_off_staff_records(name: &str, this: usize, club: usize, status: u32, stage: u32, d: &PlayOffDivision, original: impl FnOnce() -> u32) -> u32 {
    let plan = play_off_record_plan(League(this), Club(club), status as u8, stage as u8, d);
    let r = original();
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == plan.result && row_ok, || format!("{}({:#x}, {:#x}, status {}, stage {}) original={:#x} rust={:#x} row {:?}", name, this, club, status as u8, stage as u8, r, plan.result, plan.row));
    r
}

/// Updates the staff records by a club's status in the Polish Second Division and its promotion play-off against the
/// First Division's final stage (`play_off_update_staff_records` with `POLISH_SECOND_PLAY_OFF`: the season's play-off
/// place with 0, 0xa0, 0x28, the play-off winner's status 1). The last three arguments are not read.
/// Original: `PolSecond::update_staff_records_for_status` (0x7cc490, pol_second.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn pol_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    play_off_update_staff_records(this, club, status, stage, &POLISH_SECOND_PLAY_OFF)
}

/// Comparer for `pol_second_update_staff_records_for_status` (`compare_play_off_staff_records`).
pub(crate) unsafe extern "thiscall" fn compare_pol_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    compare_play_off_staff_records("pol_second_update_staff_records_for_status", this, club, status, stage, &POLISH_SECOND_PLAY_OFF, || o(this, club, status, stage, a4, a5, a6))
}

/// The play-off text of a Polish club (or of a Scottish one, `format`): the club's short gender byte twice and its
/// short name into the shared buffer (`PRAISE_TEXT_BUFFER`), the buffer set in `text` (`exe::text_set_string`) with the
/// club's link (`exe::text_add_link`, 0x7d5).
pub(crate) unsafe fn play_off_club_text(club_id: u32, format: usize, text: usize) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    story_lang!(PRAISE_TEXT_BUFFER, format, g, g, club.addr() + club::SHORT_NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
}

/// The fate news text of a Polish Second Division club. With a news item (`news`) the common text first
/// (Comp::build_fate_news_text `exe::comp_build_fate_news_text`, a7 = 0, the rest as they came), and then, without
/// returning, the play-off text below. Outcome 1 in the play-off (stage 0): a First Division club (`a7`, its division,
/// is [`KEY_CLUB_COMP_POLISH_FIRST_DIVISION`]) gets 0x987594 "{club} avoid relegation", a Second Division one 0x989c18
/// "{club} win promotion" (`play_off_club_text`). Otherwise a First Division club gets 0x989d78 "{club} relegated",
/// and any other club the common text (again, when there was a news item).
/// Original: `PolSecond::build_fate_news_text` (0x7cc680, pol_second.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn pol_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    if news != 0 {
        exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    }
    let first = a7 == read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION);
    if outcome as u8 == 1 && stage as u8 == 0 {
        covered!("cx13_6 polish play-off text");
        return play_off_club_text(club_id, if first { 0x987594 } else { 0x989c18 }, text); // cx13_6: avoid relegation / win promotion
    }
    if first {
        return play_off_club_text(club_id, 0x989d78, text); // "relegated"
    }
    exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
}

/// Comparer for `pol_second_build_fate_news_text`: without a news item, on the play-off texts, side by side with
/// `compare_text_writer` (the original into the caller's Text, the copy into a local one); on the paths through the
/// common text only the original (it writes the news object and the Text, which takes its text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_pol_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    let play_off = outcome as u8 == 1 && stage as u8 == 0;
    if news == 0 && text != 0 && (play_off || a7 == read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION)) {
        return compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
            |t| pol_second_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
            || format!("pol_second_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
    }
    o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)
}

/// The Polish play-off progress code of a First Division club: 1 becomes 12 and 2 becomes 16; any other as it is.
pub(crate) fn polish_play_off_code(v: u8) -> u8 {
    match v {
        1 => 0xc,
        2 => 0x10,
        _ => v,
    }
}

/// The progress codes of a Polish Second Division club, into its statistics entry (`exe::comp_stats_find_club_entry`,
/// not checked for null). In the season (stage 0xff): a best of 3 or 4 becomes 2 and a worst of 3 becomes 5. In the
/// play-off (stage 0): a First Division club's codes go into the First Division's own entry with 1 → 12 and 2 → 16; a
/// Second Division club's are doubled. Any other stage, as they came. The bytes go in the order best, current, worst.
/// Defect of the original: the worst of 4 should also become 5, but the second test reads the best already mapped
/// (never 4), so a worst of 4 stays 4 [inference].
/// Original: `PolSecond::set_club_progress` (0x7cc800, pol_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn pol_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let club = Club(club);
    let mut e = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    match stage as u8 {
        0xff => {
            covered!("cx13_6 polish season progress");
            if b == 3 || b == 4 {
                b = 2; // cx13_6: the play-off band
            }
            if w == 3 || b == 4 {
                w = 5;
            }
        }
        0 => {
            covered!("cx13_6 polish play-off progress");
            if read_usize(club.division().addr()) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) {
                e = CompStatsClub(exe::comp_stats_find_club_entry(League(comp_by_global(KEY_CLUB_COMP_POLISH_FIRST_DIVISION)).comp_stats(), club));
                b = polish_play_off_code(b);
                w = polish_play_off_code(w);
                c = polish_play_off_code(c);
            } else {
                b = b.wrapping_shl(1);
                w = w.wrapping_shl(1);
                c = c.wrapping_shl(1);
            }
        }
        _ => {}
    }
    e.set_best_position(b);
    e.set_current_position(c);
    e.set_worst_position(w);
}

/// Comparer for `pol_second_set_club_progress`: side by side on the three codes of the entry written (the First
/// Division's for one of its clubs in the play-off; only the original without an entry).
pub(crate) unsafe extern "thiscall" fn compare_pol_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let c = Club(club);
    let first = stage as u8 == 0 && c.division().addr() != 0 && read_usize(c.division().addr()) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION);
    let stats = if first { League(comp_by_global(KEY_CLUB_COMP_POLISH_FIRST_DIVISION)).comp_stats() } else { League(this).comp_stats() };
    let e = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 || League(this).comp_stats() == 0 || c.division().addr() == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst), || pol_second_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("pol_second_set_club_progress({:#x}, {:#x}, stage {}, {} {} {}) original={:x?} rust={:x?}", this, club, stage as u8 as i8, current as u8, best as u8, worst as u8, mo, mr));
}
