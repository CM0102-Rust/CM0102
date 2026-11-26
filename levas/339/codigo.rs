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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_POL_SECOND_UPDATE_STAFF_RECORDS_FOR_STATUS);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_POL_SECOND_BUILD_FATE_NEWS_TEXT);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_POL_SECOND_SET_CLUB_PROGRESS);
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

// ---------- cx14_3: the Turkish category B club list, predicted positions and progress codes (tur_second_b.cpp) ----------

/// The source file of the Turkish second division category B (tur_second_b.cpp), for the error boxes.
pub(crate) const TURKISH_CATEGORY_B_SOURCE: usize = 0xaa18b8;

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

// ---------- cx14_4: the Turkish category B staff records, fate news text and final positions (tur_second_b.cpp) ----------

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

/// The first phase group of a category B club, by the id of its reserve division (`club::RESERVE_DIVISION`): group 1
/// ([`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1`]) is the league's own table, groups 2 to 5 the group stages 0..3.
/// None when the id is none of the five.
pub(crate) unsafe fn category_b_group_of(this: League, division: usize) -> Option<League> {
    let id = read_usize(division);
    TURKISH_CATEGORY_B_GROUPS.iter().position(|&g| id == read_usize(g)).map(|k| if k == 0 { this } else { League(read_usize(this.stages() + 4 * (k - 1))) })
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
    exe::text_add_link(text, team as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
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

// ---------- cx14_6: the Turkish category B group stages, promotion group and classification groups (tur_second_b.cpp) ----------

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

/// The clubs of first phase group g (−1 to 3) that did not go through (rows of status other than 1), in row order;
/// the first eight and how many more.
pub(crate) unsafe fn category_b_classification_clubs(l: League, g: i8) -> (Vec<usize>, usize) {
    let out: Vec<usize> = category_b_first_phase_rows(l).into_iter().filter(|(k, r)| *k == g && r.status() != 1).map(|(_, r)| read_usize(r.addr())).collect();
    let more = out.len().saturating_sub(8);
    (out.into_iter().take(8).collect(), more)
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

/// Copies the totals of a first phase row into a classification row (`rep movsd` of ten dwords and a word from
/// +0xd) and clears the played counts (`league_table_row::PLAYED`, `HOME_PLAYED`, `AWAY_PLAYED`).
pub(crate) unsafe fn carry_first_phase_record(from: usize, to: usize) {
    core::ptr::copy_nonoverlapping((from + 0xd) as *const u8, (to + 0xd) as *mut u8, 0x2a);
    write_u8(to + league_table_row::PLAYED, 0);
    write_u8(to + league_table_row::HOME_PLAYED, 0);
    write_u8(to + league_table_row::AWAY_PLAYED, 0);
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

// ---------- cx01_9: the order of clubs by their last division (comp_util.cpp, used by the Turkish cup) ----------

/// qsort order of two club pointers by their last season: a club without a last division goes after one with it (two
/// without are equal); between two with one, the division of higher reputation first, then the higher competition id,
/// then the better (lower) last position, by the differences of those values.
/// Original: `compare_clubs_by_last_division` (0x4c0a40, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_clubs_by_last_division(a: *const usize, b: *const usize) -> i32 {
    let (ca, cb) = (Club(read_usize(a as usize)), Club(read_usize(b as usize)));
    let (da, db) = (ca.last_division(), cb.last_division());
    match (da.is_null(), db.is_null()) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => -1,
        (false, false) => {
            let by_reputation = db.reputation() as i32 - da.reputation() as i32;
            if by_reputation != 0 {
                return by_reputation;
            }
            let by_id = read_u32(db.addr()).wrapping_sub(read_u32(da.addr())) as i32;
            if by_id != 0 {
                return by_id;
            }
            covered!("cx01_9 same last division");
            ca.last_position() as i8 as i32 - cb.last_position() as i8 as i32
        }
    }
}

compare_order!(compare_compare_clubs_by_last_division, compare_clubs_by_last_division, IDX_COMPARE_CLUBS_BY_LAST_DIVISION);

// ---------- sbF_2: the Greek Cup schedule, first group table and group stages (gre_cup.cpp) ----------

/// The Greek Cup groups: six matchdays from 11 August to 8 September (11, 18, 22, 25 and 29 August and 8 September;
/// weekday 2 with code 2, or weekday 6 with code 1).
pub(crate) const GREEK_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 6] = [(0xb, 7, 2, 2), (0x12, 7, 2, 2), (0x16, 7, 6, 1), (0x19, 7, 2, 2), (0x1d, 7, 6, 1), (8, 8, 2, 2)]; // sbF2: the Greek Cup group matchdays

/// The Greek Cup knockout: the 1st round drawn on 16 September and played on 15 December (12 clubs, 6 ties, all new),
/// the 2nd drawn 16 December and played 13 January (16 clubs, 8 ties, 10 new from entry 12), the quarter-finals drawn
/// 14 January and played over two legs 14 days apart from 24 February, the semi-finals drawn 8 March and played over
/// two legs from 5 April, the final drawn 20 April and played on 10 May at a neutral ground [inference: venue 4];
/// single-match tie rules 3; category 4 up to the quarter-finals and 6 after; no prize per round (−1).
pub(crate) const GREEK_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x10, 8, 0, 3), (0xf, 11, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0xc, 6, 0xc, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // sbF2: the Greek Cup 1st round
    cql10r((0x10, 11, 0, 3), (0xd, 0, 1, 2, 2, 0), 0x1e, (0, 3, 0), 4, (0x10, 8, 0xa, 0xc), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xe, 0, 1, 3), (0x18, 1, 1, 2, 2, 0), 0x78, (0, 0, 3), 4, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((8, 2, 1, 3), (5, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x14, 3, 1, 3), (0xa, 4, 1, 2, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Rule: the Greek Cup's eleven groups of four (the cup's own group and stages 0 to 9) play six matchdays from 11
/// August to 8 September, each group a league stage (is_league = 1, label 0x3e9 + stage); the knockout (stage 10) has
/// 22 clubs: 12 in the 1st round (December), 10 more in the 2nd (January), then two-legged quarter- and semi-finals
/// and the final on 10 May.
///
/// `create_schedule` with the knockout at stage 10, the label 0x3e9, `GREEK_CUP_GROUP_ROUNDS`,
/// `GREEK_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xc3 and 0xee of gre_cup.cpp (checked by emulation of the
/// original for every stage −128..127 in an odd and an even year: the same calls and the same 16 fields per round as
/// ItaCup::create_schedule). Above stage 10: 0.
/// Original: `GreCup::create_schedule` (0x5e8bf0, gre_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 0xa, 0x3e9, &GREEK_CUP_GROUP_ROUNDS, &GREEK_CUP_KNOCKOUT_ROUNDS, 0x9cdab0, 0xc3, 0xee,
        "sbF_2 greek cup groups", "sbF_2 greek cup knockout rounds")
}

/// Comparer for `gre_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GRE_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("gre_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| gre_cup_create_schedule(this, stage, r, e, l), 0xa)
}

/// The Greek Cup entrant pool: 44 clubs in eleven columns of four (entry `group + 11 × k` is club k of group `group`;
/// group 0 is the cup's own group, group g + 1 the group stage g). The pool is read, never cleared: each column is used
/// once.
pub(crate) const GREEK_CUP_GROUPS: usize = 0xb;

/// Pool entry `k` of Greek Cup group `group` (the entrant vector `ita_cup_fields::ENTRANT_CLUBS`, column `group`).
pub(crate) unsafe fn greek_cup_pool_club(this: League, group: usize, k: usize) -> usize {
    read_usize(cup_pool_entry(this, group + k * GREEK_CUP_GROUPS))
}

/// Rule: the Greek Cup's own group (the cup is the league of its first group) takes the four clubs of column 0 of the
/// entrant pool; the group draw news follows.
///
/// `league::CLUB_COUNT` = 4; the table (`exe::crt_malloc` of 0xec; out of memory, the box at line 0x1ab of gre_cup.cpp
/// and 0); row k gets pool entry 11 × k (`exe::league_init_table_row`(this, k, club, 0, 0); the original pushes the
/// dword of the local byte with stack garbage above it, the routine reads only the byte; the index is a signed byte
/// compared with the word `league::CLUB_COUNT`); unlike the Welsh and Irish cups the pool entry is not cleared. Then
/// the group draw news (League::send_group_draw_news, thiscall, ret 0). Returns 1.
/// Original: `GreCup::create_table` (0x5e9260, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9cdab0, 0x1ab);
        return 0;
    }
    covered!("sbF_2 greek cup first group");
    let mut k: i8 = 0;
    while (k as i16) < this.club_count() {
        exe::league_init_table_row(this, k as u8 as u32, greek_cup_pool_club(this, 0, k as usize), 0, 0);
        k = k.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Comparer for `gre_cup_create_table`: the plan (column 0 of the pool), then only the original (`init_table_row`
/// allocates and the news goes to the managers); returns 1, four clubs, the table rows in pool order and the pool
/// untouched.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_CUP_CREATE_TABLE);
    let t = League(this);
    let plan: Vec<usize> = (0..4).map(|k| greek_cup_pool_club(t, 0, k)).collect();
    let r = o(this);
    let found: Vec<usize> = if t.table() != 0 { (0..4).map(|k| read_usize(t.table() + k * league_table_row::SIZE)).collect() } else { Vec::new() };
    let ok = r == 1 && t.club_count() == 4 && found == plan && (0..4).all(|k| greek_cup_pool_club(t, 0, k) == plan[k]);
    SCOREBOARD.record(ok, || format!("gre_cup_create_table({:#x}) returned {}; plan {:x?} table {:x?}", this, r, plan, found));
    r
}

/// How a cup played as a league builds one of its group stages (`build_cup_group_stage`): the club count, the first
/// local tie-break, the 4th and the 18th arguments of League_Stage::League_Stage (1 or 2, and 0 or 0x82 [inference:
/// the places through and the stage's round name]), the source file and the two error-box lines.
pub(crate) struct CupGroupStage {
    pub(crate) clubs: u32,
    pub(crate) tiebreak: u8,
    pub(crate) arg4: u32,
    pub(crate) arg18: u32,
    pub(crate) file: usize,
    pub(crate) rows: (u32, u32),
}

/// Builds group stage `g` of a cup played as a league, in the original routine's order (the body of
/// WelPrmCup::create_group_stage, GreCup::create_group_stage, HolCup::create_group_stage and
/// KorLeagueCup::create_group_b_stage, compared instruction by instruction: only the values of `CupGroupStage` and how
/// the clubs leave the entrant pool change).
///
/// The round table of the stage (virtual `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0)); the club list
/// (`exe::crt_malloc` of 4 × clubs; null: the box of the first line and 0) with `take`(k) for k = 0.. clubs − 1 (the
/// club of slot k, taken from the pool by the caller's rule); a League_Stage (`exe::crt_operator_new`(0xee) and
/// League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this cup as the parent, the club count, the list, `arg4`,
/// `league::COMPETITION`, the round table and its count, the win, draw and category bytes, the tie-breaks
/// (`league::TIEBREAKS`), a local tie-break list (`tiebreak`, 0, 0, 0), the year, g, the label, the importance, 1,
/// `arg18`, the league flags, −1, 0, 2) into stage slot g (0 without memory); the list and the round table freed; no
/// stage: the box of the second line and 0; otherwise the group draw news of the stage (League::send_group_draw_news),
/// the current stage = g and 1. The original pushes the round count, the label, the year and the flags as dwords with
/// stale high bytes; League_Stage::League_Stage reads the words and bytes.
pub(crate) unsafe fn build_cup_group_stage(this: League, g: u32, p: &CupGroupStage, mut take: impl FnMut(usize) -> usize, ramo: &'static str) -> u32 {
    let tiebreaks: [u8; 4] = [p.tiebreak, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(this.addr(), g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * p.clubs);
    if clubs == 0 {
        show_error_at(p.file, p.rows.0);
        return 0;
    }
    covered!(ramo);
    for k in 0..p.clubs as usize {
        write_usize(clubs + 4 * k, take(k));
    }
    let n = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if n != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        s = ctor(n, this.addr(), p.clubs, clubs, p.arg4, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32, this.draw_points() as u32,
            this.match_category() as u32, this.addr() + league::TIEBREAKS, tiebreaks.as_ptr() as usize, this.year() as u16 as u32, g, label as u16 as u32,
            this.importance() as u32, 1, p.arg18, (this.league_flags() as u16) as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(p.file, p.rows.1);
        return 0;
    }
    exe::league_send_group_draw_news(s);
    this.set_current_stage(g as usize);
    1
}

/// The address of entrant pool entry `i` (`ita_cup_fields::ENTRANT_CLUBS`, a vector of club pointers).
pub(crate) unsafe fn cup_pool_entry(this: League, i: usize) -> usize {
    read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS) + 4 * i
}

/// The Greek Cup group stages: four clubs, the local tie-break 2, 1 as the 4th argument, the boxes at lines 0x271 and
/// 0x286 of gre_cup.cpp.
pub(crate) const GREEK_CUP_GROUP_STAGE: CupGroupStage = CupGroupStage { clubs: 4, tiebreak: 2, arg4: 1, arg18: 0, file: 0x9cdab0, rows: (0x271, 0x286) };

/// Rule: Greek Cup group stage g (0 to 9) is a league stage of the four clubs of pool column g + 1 (they stay in the
/// pool), played over the cup's group schedule; the group draw news follows and the current stage becomes g.
///
/// `build_cup_group_stage` with `GREEK_CUP_GROUP_STAGE`, club k from pool entry g + 1 + 11 × k, not cleared (against
/// WelPrmCup::create_group_stage the pool stride is 11, the entries are not cleared and the 4th argument of
/// League_Stage::League_Stage is 1).
/// Original: `GreCup::create_group_stage` (0x5e9650, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let this = League(this);
    build_cup_group_stage(this, g, &GREEK_CUP_GROUP_STAGE, |k| read_usize(cup_pool_entry(this, g as usize + 1 + k * GREEK_CUP_GROUPS)), // sbF2: the Greek group columns
        "sbF_2 greek cup group stage")
}

/// Comparer for `gre_cup_create_group_stage`: only the original (it builds a stage with its fixtures and sends the
/// news); then the return 1, the stage in slot g with four clubs and the cup's year, the current stage g, the stage
/// table holding pool column g + 1 (in any order: the constructor sorts it) and the pool untouched.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_GRE_CUP_CREATE_GROUP_STAGE);
    let t = League(this);
    let mut plan: Vec<usize> = (0..4).map(|k| greek_cup_pool_club(t, g as usize + 1, k)).collect();
    let r = o(this, g);
    let f = League(read_usize(t.stages() + 4 * g as usize));
    let mut ok = r == 1 && !f.is_null() && t.current_stage() as u32 == g && f.club_count() == 4 && f.year() == t.year()
        && (0..4).all(|k| greek_cup_pool_club(t, g as usize + 1, k) == plan[k]);
    if ok && f.table() != 0 {
        let mut found: Vec<usize> = (0..4).map(|k| read_usize(f.table() + k * league_table_row::SIZE)).collect();
        found.sort_unstable();
        plan.sort_unstable();
        ok = found == plan;
    }
    SCOREBOARD.record(ok, || format!("gre_cup_create_group_stage({:#x}, {}) returned {}, stage {:#x}, plan {:x?}", this, g, r, f.addr(), plan));
    r
}

// ---------- sbF_3: the Greek Cup constructor and knockout stage (gre_cup.cpp) ----------

/// Rule: the Greek Cup is a cup played as a league with eleven groups of four (the cup's own group and ten group
/// stages, one stage slot more for the knockout), with 7 substitutes on the bench and 3 substitutions under the Greek
/// discipline rules; with nothing saved, its 44 entrants, the statistics, the predicted positions, the rules, the first
/// group, its fixtures and the ten group stages are made.
///
/// Over League::League (`exe::new_league`, thiscall, ret 0): `league::COMPETITION` = the record, the vtable 0x96b4b4,
/// `league::YEAR`, 3 substitutions allowed, 7 substitutes named, `discipline_rule::GREECE`, importance 0xc. If the saved
/// state loads (`exe::league_load_from_temp_file`(this, 1) ≠ 0) it returns this. Otherwise, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, 11 stages
/// and their array (`exe::crt_malloc` of 0x2c, unchecked); the entrants (GreCup::collect_entrants 0x5e9a80, thiscall,
/// ret 0, unchecked); the statistics (`ita_cups_create_comp_stats`; without them the box at line 0x56); the predicted
/// positions (GreCup::set_predicted_positions 0x5ea060, unchecked); the rules (GreCup::set_up_rules 0x5e9350; 0: line
/// 0x60); the first group's table (GreCup::create_table 0x5e9260; 0: line 0x67); the fixtures
/// (`exe::league_create_fixtures`; 0: line 0x6d); the order (`exe::league_sort_table`(this, 0); 0: line 0x74); the
/// stages of groups 0 to 9 (GreCup::create_group_stage 0x5e9650, push g; mov ecx -> thiscall, ret 4). Each box returns
/// this (the same body as `construct_irish_league_cup`, checked instruction by instruction against
/// NirLgeCup::NirLgeCup: only the vtable, the substitutes, the discipline rule, the stage count, the routines, the group
/// count and the box lines change). Returns this.
/// Original: `GreCup::GreCup` (0x5e88a0, gre_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), GREEK_CUP_VTABLE);
    this.set_year(year as u16 as i16);
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(7);
    this.set_discipline_rule(discipline_rule::GREECE);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("sbF_3 greek cup new");
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(0xb); // sbF3: the ten groups and the knockout
    this.set_stages(exe::crt_malloc(0x2c));
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
        f(this.addr())
    };
    call(0x5e9a80); // GreCup::collect_entrants
    if !ita_cups_create_comp_stats(this) {
        show_error_at(0x9cdab0, 0x56);
        return this.addr();
    }
    call(0x5ea060); // GreCup::set_predicted_positions
    if call(0x5e9350) == 0 {
        // GreCup::set_up_rules
        show_error_at(0x9cdab0, 0x60);
        return this.addr();
    }
    if call(0x5e9260) == 0 {
        // GreCup::create_table
        show_error_at(0x9cdab0, 0x67);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9cdab0, 0x6d);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(0x9cdab0, 0x74);
        return this.addr();
    }
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x5e9650usize); // GreCup::create_group_stage
    for g in 0..0xa {
        group(this.addr(), g);
    }
    this.addr()
}

/// The Greek Cup vtable.
pub(crate) const GREEK_CUP_VTABLE: usize = 0x96b4b4;

/// Only the original (it allocates, draws and reads the save file), checking the plan (`ita_cups_check_constructor_plan`:
/// this, the vtable, the record, the year and, on a new cup, the fixed fields and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GRE_CUP)(this, year, record);
    let fields = [(league::STAGE_COUNT, 4u8, 0xb), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 7), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
        (league::DISCIPLINE_RULE, 1, discipline_rule::GREECE as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];
    let error = ita_cups_check_constructor_plan(this, r, year, record, GREEK_CUP_VTABLE, &fields, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_gre_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Rule: the Greek Cup knockout (stage 10) has 22 clubs: the eleven group runners-up in group order and the eleven
/// group winners shuffled (the round table: twelve clubs in the 1st round, ten more in the 2nd).
///
/// The club vector (`exe::crt_malloc` of 0x58; out of memory, the box at line 0x2a5 of gre_cup.cpp and nothing more);
/// slots 0 to 10 the second row of each group's table (the cup's own table `league::TABLE` + one row, then stages 0 to
/// 9 by League::get_table_row(stage, 1), thiscall, ret 4), slots 11 to 21 the first rows in the same order; slots 11
/// to 21 shuffled (`exe::comp_util_shuffle_pointer_array`(vector + 0x2c, 11), cdecl); then `create_knockout_stage`
/// (22 clubs, stage 10, no seeds, the box at line 0x2d3): the round table of stage 10, the Cup_Stage into stage slot
/// 10, the vector and the round table freed and the first round drawn.
/// Original: `GreCup::create_knockout_stage` (0x5e9850, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let teams = exe::crt_malloc(0x58);
    if teams == 0 {
        return show_error_at(0x9cdab0, 0x2a5);
    }
    covered!("sbF_3 greek cup knockout");
    for (half, row) in [(0usize, 1i32), (GREEK_CUP_GROUPS, 0)] {
        // sbF3: the runners-up first, then the winners
        write_usize(teams + 4 * half, read_usize(this.table() + row as usize * league_table_row::SIZE));
        for g in 0..0xausize {
            let stage = read_usize(this.stages() + 4 * g);
            write_usize(teams + 4 * (half + g + 1), read_usize(exe::league_get_table_row(stage, row)));
        }
    }
    exe::comp_util_shuffle_pointer_array(teams + 4 * GREEK_CUP_GROUPS, GREEK_CUP_GROUPS as u32);
    create_knockout_stage(this, 0x16, teams, 0xa, core::ptr::null(), 0x9cdab0, 0x2d3);
}

/// Comparer for `gre_cup_create_knockout_stage`: only the original (it builds the stage and draws its first round);
/// then the stage in slot 10 with 22 clubs, the eleven runners-up and the eleven winners (as a set: the winners are
/// shuffled).
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GRE_CUP_CREATE_KNOCKOUT_STAGE);
    let t = League(this);
    let group = |g: usize, row: i32| if g == 0 { read_usize(t.table() + row as usize * league_table_row::SIZE) } else { read_usize(exe::league_get_table_row(read_usize(t.stages() + 4 * (g - 1)), row)) };
    let mut plan: Vec<usize> = (0..GREEK_CUP_GROUPS).flat_map(|g| [group(g, 1), group(g, 0)]).collect();
    o(this);
    let s = Cup(read_usize(t.stages() + 4 * 0xa));
    let mut ok = !s.is_null() && s.team_count() == 0x16 && s.team_table() != 0;
    if ok {
        let mut teams: Vec<usize> = (0..0x16).map(|k| read_usize(s.team_table() + k * 6)).collect();
        teams.sort_unstable();
        plan.sort_unstable();
        ok = teams == plan;
    }
    SCOREBOARD.record(ok, || format!("gre_cup_create_knockout_stage({:#x}) stage {:#x} plan {:x?}", this, s.addr(), plan));
}

// ---------- cx02_18: the Greek Cup rules, turnover and predicted positions (gre_cup.cpp) ----------

/// The Greek Cup groups: 3 points for a win and 1 for a draw, category 4 (national cup), the tie-breaks 4, 1, 2 (the
/// head-to-head first [inference]), two places through per group, a 600,000 prize fund.
pub(crate) const GREEK_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [4, 1, 2, 0], slots: 2, prize: Some(0x927c0), bits: None };

/// The Greek Cup rules: `set_cup_league_rules` with `GREEK_CUP_GROUP_RULES` and one meeting (checked instruction by
/// instruction against NirLgeCup::set_up_rules: only the tie-break order and the prize change), then the round table.
/// Returns 1.
/// Original: `GreCup::set_up_rules` (0x5e9350, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_set_up_rules(this: usize) -> u32 {
    covered!("cx02_18 greek cup rules");
    set_cup_league_rules(League(this), &GREEK_CUP_GROUP_RULES, 1)
}

/// The Greek Cup turnover: `start_irish_league_cup_season` with GreCup::collect_entrants (0x5e9a80), GreCup::create_table
/// (0x5e9260), GreCup::create_group_stage (0x5e9650) for the ten groups, the year up by one and the boxes at lines
/// 0x21c, 0x223, 0x229 and 0x230 of gre_cup.cpp (checked instruction by instruction against
/// NirLgeCup::start_new_season: only the routines, the group count and the lines change).
/// Original: `GreCup::start_new_season` (0x5e93f0, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_start_new_season(this: usize) -> u32 {
    covered!("cx02_18 greek cup turnover");
    start_irish_league_cup_season(League(this), 0x5e9a80, 0x5e9260, 0x5e9650, 10, 0x9cdab0, [0x21c, 0x223, 0x229, 0x230], 1)
}

/// The Greek Cup predicted positions: `italian_cup_predicted_positions` with the knockout at stage 10, bands of 11 and
/// the boxes at lines 0x3f0 and 0x3f8 of gre_cup.cpp (checked instruction by instruction against
/// ItaCup::set_predicted_positions: only the file, the lines, the stage and the band change).
/// Original: `GreCup::set_predicted_positions` (0x5ea060, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9cdab0, (0x3f0, 0x3f8), 0xa, 0xb, "cx02_18 greek cup predicted")
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("gre_cup_set_up_rules", League(this), &GREEK_CUP_GROUP_RULES, r);
    r
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("gre_cup_start_new_season", League(this), || o(this))
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GRE_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("gre_cup_set_predicted_positions", this, gre_cup_set_predicted_positions, || o(this));
}

// ---------- cx02_25: the Greek Cup next stage (gre_cup.cpp) ----------

/// The Greek Cup moves on to its next stage: past the last stage the box at line 0x24a of gre_cup.cpp and nothing
/// else; otherwise the current stage goes up by one and, on reaching stage 10, the knockout stage is built
/// (GreCup::create_knockout_stage 0x5e9850) (`welsh_cup_advance_stage`, checked instruction by instruction against
/// WelPrmCup::advance_stage: only the file, the line and the stage change).
/// Original: `GreCup::advance_stage` (0x5e95c0, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0x9cdab0, 0x24a, 10, 0x5e9850)
}

/// Comparer for `gre_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage stays
/// at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GRE_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("gre_cup_advance_stage", League(this), || o(this))
}

// ---------- cx03_7: the Russian Cup's day (rus_cup.cpp) ----------

/// Whether today is Date(`day`, `month`, today's year) (months from 0, any weekday: `make_date_dword`).
unsafe fn is_today_on(today: usize, day: u32, month: u32) -> bool {
    // simplification: the original pushes today's year with garbage in the high 16 bits; Date::Date reads the word
    is_date_today(make_date_dword(day, month, read_i16(today + 2) as u16 as u32, 0xffff_ffff), today)
}

/// Whether a finished cup turns over today: `stage` set, the cup and its chain complete (virtual +0xc,
/// `Comp::all_stages_complete`, thiscall, ret 0, in al) and today on Date(`day`, `month`).
unsafe fn cup_turnover_due(this: Cup, today: usize, stage: u32, day: u32, month: u32) -> bool {
    if stage == 0 {
        return false;
    }
    let complete: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0xc)); // Comp::all_stages_complete
    complete(this.addr()) != 0 && is_today_on(today, day, month)
}

/// The daily virtual (`comp_vtable::UPDATE_FOR_DATE`) of the cups that start their next edition on a fixed day once the
/// current one is over: when due (`cup_turnover_due`) the season turnover (virtual `comp_vtable::START_NEW_SEASON`,
/// thiscall, ret 0); then, every day, the cup's day (`exe::cup_update_for_date`, the same arguments), whose result it
/// returns.
pub(crate) unsafe fn update_cup_with_turnover_on(this: Cup, today: usize, stage: u32, day: u32, month: u32, branch: &'static str) -> u32 {
    if cup_turnover_due(this, today, stage, day, month) {
        covered!(branch);
        let turn: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::START_NEW_SEASON));
        turn(this.addr());
    }
    exe::cup_update_for_date(this, today, stage)
}

/// Comparer of `update_cup_with_turnover_on`: only the original runs (the turnover frees, allocates and draws; the
/// cup's day plays); the plan: when the turnover was due, the cup year goes up by one, otherwise it stays.
pub(crate) unsafe fn compare_update_cup_with_turnover_on(name: &str, this: Cup, today: usize, stage: u32, day: u32, month: u32, original: impl FnOnce() -> u32) -> u32 {
    let due = cup_turnover_due(this, today, stage, day, month);
    let before = this.year();
    let r = original();
    let after = this.year();
    let expected = if due { before.wrapping_add(1) } else { before };
    SCOREBOARD.record(after == expected, || format!("{}({:#x}, stage {}) due {} year {} -> {}", name, this.addr(), stage, due, before, after));
    r
}

/// rule: the Russian Cup starts its next edition on 1 June (month 5 counted from 0) once the current one is over; then
/// the cup's day (`update_cup_with_turnover_on`).
/// Original: `RusCup::update_for_date` (0x7e9ec0, rus_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rus_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    update_cup_with_turnover_on(Cup(this), today, stage, 1, 5, "cx03_7 russian cup next edition") // cx03_7: the Russian Cup on 1 June
}

pub(crate) unsafe extern "thiscall" fn compare_rus_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_RUS_CUP_UPDATE_FOR_DATE);
    compare_update_cup_with_turnover_on("rus_cup_update_for_date", Cup(this), today, stage, 1, 5, || o(this, today, stage))
}

// ---------- cx09_18: the Croatian Second Division north group: the stage advance, the tie-break play-off and the day (cro_a2a.cpp) ----------

/// The cro_a2a.cpp source path, for the error boxes.
const CRO_A2A_SOURCE: usize = 0x9b53f8;

/// Rule: the Croatian Second Division north group's next stage, as the south group's (line 0x1e9 of cro_a2a.cpp, the
/// play-off `cro_a2a_create_tiebreak_playoff_stage`, through its detour).
/// Original: `CroA2a::advance_stage` (0x516060, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_advance_stage(this: usize) {
    covered!("cx09_18 croatian second division north next stage");
    state_league_advance_stage(League(this), CRO_A2A_SOURCE, 0x1e9, 0x5160f0);
}

/// Rule: the tie-break play-off at the end of the Croatian Second Division north group, as the south group's (line
/// 0x23a of cro_a2a.cpp; `create_croatian_tiebreak_playoff`).
/// Original: `CroA2a::create_tiebreak_playoff_stage` (0x5160f0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_create_tiebreak_playoff_stage(this: usize) {
    create_croatian_tiebreak_playoff(League(this), CRO_A2A_SOURCE, 0x23a);
}

/// Whether two neighbouring rows of a Croatian table call for a tie-break play-off: the same points
/// (`league_table_row::POINTS`) unless both are relegated (`league_table_row::STATUS` 3).
unsafe fn croatian_tie_needs_playoff(upper: LeagueTableRow, lower: LeagueTableRow) -> bool {
    upper.points() == lower.points() && (upper.status() != 3 || lower.status() != 3)
}

/// The Croatian Second Division north group's day: the league day (`exe::league_update_for_date`(today, full)), whose
/// byte is returned. At the end of the season (`league::COMPLETE`), with no stage yet (`league::STAGE_COUNT` = 0),
/// with `full` and with `league::TIEBREAK_MATCHES`: `league::SEASON_CLOSED` = 0 and, from the top, the first pair of
/// neighbouring rows that needs a play-off (`croatian_tie_needs_playoff`) makes one stage (`league::STAGE_COUNT` = 1,
/// `league::STAGES` = malloc of 4) and the next stage (virtual `comp_vtable::ADVANCE_STAGE`); with none,
/// `league::COMPLETE` = 0, `league::TIEBREAK_MATCHES` = 0 and virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`.
/// Original: `CroA2a::update_for_date` (0x516360, cro_a2a.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn cro_a2a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let r = exe::league_update_for_date(this, today, full) as u8 as u32;
    if l.complete() == 0 || l.stage_count() != 0 || full == 0 || l.tiebreak_matches() == 0 {
        return r;
    }
    l.set_season_closed(0);
    let row = |k: i32| LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
    let mut i: i16 = 0;
    while (i as i32) < l.club_count() as i32 - 1 {
        if croatian_tie_needs_playoff(row(i as i32), row(i as i32 + 1)) {
            covered!("cx09_18 croatian north tie-break");
            l.set_stage_count(1);
            l.set_stages(exe::crt_malloc(4));
            bra_nat_first_call_virtual_no_args(this, comp_vtable::ADVANCE_STAGE);
            return r;
        }
        i = i.wrapping_add(1);
    }
    l.set_complete(0);
    l.set_tiebreak_matches(0);
    let status: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
    status(this);
    r
}

/// Comparer for `cro_a2a_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A2A_ADVANCE_STAGE);
    ita_ser_c1a_compare_advance_stage("cro_a2a_advance_stage", League(this), || o(this), || cro_a2a_advance_stage(this))
}

/// Comparer for `cro_a2a_create_tiebreak_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_tiebreak_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A2A_CREATE_TIEBREAK_PLAYOFF_STAGE);
    compare_croatian_tiebreak_playoff("cro_a2a_create_tiebreak_playoff_stage", League(this), || o(this))
}

/// Comparer for `cro_a2a_update_for_date`: only the original (the league day cannot run twice); in the main stage with
/// `full`, a stage created means a tied pair needing a play-off, and the season end is never left pending.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let stages = l.stage_count();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CRO_A2A_UPDATE_FOR_DATE)(this, today, full);
    if full != 0 && stages == 0 {
        let row = |k: i32| LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
        let tie = (0..l.club_count() as i32 - 1).any(|k| croatian_tie_needs_playoff(row(k), row(k + 1)));
        let ok = if l.stage_count() != 0 { tie && l.stages() != 0 } else { !(l.complete() != 0 && l.tiebreak_matches() != 0) };
        SCOREBOARD.record(ok, || format!("cro_a2a_update_for_date({:#x}): stages {} -> {}, tie {}, complete {}, tie-break {}", this, stages, l.stage_count(), tie, l.complete(), l.tiebreak_matches()));
    }
    r
}

// ---------- cx09_19: the Croatian promotion and relegation chain and the regrouping of the second division (cro_a1.cpp) ----------

/// The Croatian chain, run by the First Division: the final positions of the First Division and of both second division
/// groups (virtual `league_vtable::RECORD_FINAL_POSITIONS`, First, north, south), then the exchanges First–north and
/// First–south (`exe::league_promote_and_relegate`(upper, lower, 1, 1, −1, −1) on this league). The argument is not
/// read.
/// Original: `CroA1::process_promotion_relegation_chain` (0x5150f0, cro_a1.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cro_a1_process_promotion_relegation_chain(this: usize, _arg: u32) {
    let first = comp_by_global(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4);
    for key in [KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        let league = comp_by_global(key);
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(league) + league_vtable::RECORD_FINAL_POSITIONS));
        f(league);
    }
    covered!("cx09_19 croatian chain");
    for key in [KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        exe::league_promote_and_relegate(this, first, comp_by_global(key), 1, 1, -1, -1);
    }
}

/// The 32 Croatian second division clubs, ordered by the latitude of their stadium city: the clubs (index 0 to
/// `G_CLUB_COUNT`, read once) whose `club::DIVISION` is the north or the south group record (up to 32, in club order),
/// sorted by qsort with Database's latitude order (0x53fd40). Returns the list (32 slots) and the count.
unsafe fn croatian_second_division_by_latitude() -> ([usize; 32], usize) {
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let mut list = [0usize; 32];
    let mut n = 0usize;
    let count = read_i32(G_CLUB_COUNT);
    let mut i = 0i32;
    while i < count {
        let club = club_at(i as isize);
        let division = club.division().addr();
        if division == north || division == south {
            list[n] = club.addr();
            n += 1;
            if n == 32 {
                break;
            }
        }
        i += 1;
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as u32, 4, 0x53fd40usize);
    (list, n)
}

/// Regroups the Croatian second division by latitude (`croatian_second_division_by_latitude`): the first 16 clubs go
/// to the north group and the other 16 to the south group (`club::DIVISION`); a club that changes group and has a human
/// manager (`exe::news_club_has_human_manager` on `G_NEWS`) gets a local news item (`exe::new_newsitem`(0x13, 0): slot 0
/// the club's id, slot 1 the new group's id) sent to its manager (`exe::news_send_to_club_manager`). The original walks
/// all 32 slots even with fewer clubs, reading its uninitialised stack (a defect; the copy's empty slots are 0 and
/// would fault the same way).
/// Original: `cro_a1_realign_second_division_groups` (0x515190, cro_a1.cpp, cdecl).
pub(crate) unsafe extern "C" fn cro_a1_realign_second_division_groups() {
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let (list, _) = croatian_second_division_by_latitude();
    covered!("cx09_19 croatian second division regrouped");
    for (k, &club) in list.iter().enumerate() {
        let group = if k < 16 { north } else { south };
        if read_usize(club + club::DIVISION) != group && exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
            let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
            let n = local.as_mut_ptr() as usize;
            exe::new_newsitem(n, 0x13, 0);
            exe::newsitem_set_slot(n, 0, read_u32(club));
            exe::newsitem_set_slot(n, 1, read_u32(group));
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
        }
        write_usize(club + club::DIVISION, group);
    }
}

/// Comparer for `cro_a1_process_promotion_relegation_chain`: only the original (it moves the clubs).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_process_promotion_relegation_chain(this: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CRO_A1_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, arg)
}

/// Comparer for `cro_a1_realign_second_division_groups`: only the original (it sends news); the copy's order, made
/// before, must match the groups the original gave.
pub(crate) unsafe extern "C" fn compare_cro_a1_realign_second_division_groups() {
    let (list, n) = croatian_second_division_by_latitude();
    original_fn::<unsafe extern "C" fn()>(IDX_CRO_A1_REALIGN_SECOND_DIVISION_GROUPS)();
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let wrong: Vec<(usize, usize)> = list[..n].iter().enumerate()
        .filter(|&(k, &c)| read_usize(c + club::DIVISION) != if k < 16 { north } else { south })
        .map(|(k, &c)| (k, c)).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("cro_a1_realign_second_division_groups: {} clubs, (slot, club) in the wrong group {:x?}", n, wrong));
}

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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_POL_FIRST_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
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
