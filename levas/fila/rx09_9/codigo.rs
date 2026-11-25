// ---------- rx09_9: the Swedish Second Division sections, promotion play-off and relegation play-off (swe_second.cpp) ----------

/// The key of Swedish Second Division section stage k (0 East Svealand, 1 West Svealand, 2 East Götaland, 3 West
/// Götaland, any other value South Götaland), as `swe_second_create_section_stage` picks it.
pub(crate) fn swedish_second_section_key(k: u8) -> usize {
    match k {
        0 => KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND,
        1 => KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND,
        2 => KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND,
        3 => KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND,
        _ => KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND,
    }
}

/// Builds Swedish Second Division section stage `section` (0 to 4, `swedish_second_section_key`) as stage `section`:
/// the league's club count read first; its round table (`stage_round_table`, the argument pushed as it came), a list of
/// that many clubs (`exe::crt_malloc` of 4 each; none: the box at line 0x272) with the section's clubs
/// (`swedish_second_section_clubs`); each club past the count shows the box at line 0x283 and loses its division
/// (`club::DIVISION` = 0, a defect of the original) and the scan goes on; fewer: the box at line 0x290. Then the
/// League_Stage (`category_b_league_stage`: the league's own places from `league::PROMOTED_COUNT`, importance and rule
/// bits) into stage slot `section` (signed byte); none: the box at line 0x29b. Otherwise the list and the round table
/// freed, each row of the new stage gets the B team mark (`mark_b_team_row`) and the current stage = `section`. On the
/// error paths the round table (and the list) are not freed (leaks of the original).
/// Original: `SweSecond::create_section_stage` (0x892670, swe_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn swe_second_create_section_stage(this: usize, section: u32) {
    let l = League(this);
    let n = l.club_count();
    let (table, rounds, extra) = stage_round_table(l, section);
    let key = swedish_second_section_key(section as u8);
    let list = exe::crt_malloc((n as i32 * 4) as u32);
    if list == 0 {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x272);
    }
    let mut count = 0i16;
    for c in swedish_second_section_clubs(l, key) {
        if count >= n {
            show_error_at(SWEDISH_SECOND_SOURCE, 0x283);
            write_usize(c.addr() + club::DIVISION, 0);
        } else {
            write_usize(list + 4 * count as usize, c.addr()); // rx09_9: the next club of the section
            count += 1;
        }
    }
    if count < n {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x290);
    }
    let slot = section as u8 as i8 as isize as usize;
    let places = &*((this + league::PROMOTED_COUNT) as *const [u8; 4]);
    let stage = category_b_league_stage(l, n as u16 as u32, list, table, rounds, extra, places, section, l.importance() as u32, l.league_flags() as u16 as u32);
    write_usize(l.stages() + 4 * slot, stage);
    if read_usize(l.stages() + 4 * slot) == 0 {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x29b);
    }
    covered!("rx09_9 swedish second section stage");
    exe::crt_free(list);
    exe::crt_free(table);
    for k in 0..(n as i32).max(0) as usize {
        mark_b_team_row(League(read_usize(l.stages() + 4 * slot)), k);
    }
    l.set_current_stage(slot);
}

/// Comparer for `swe_second_create_section_stage`: only the original runs (it allocates the stage); with a full
/// section in the plan, the stage's clubs (as a set) and the current stage against it.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_section_stage(this: usize, section: u32) {
    let l = League(this);
    let n = l.club_count().max(0) as usize;
    let plan: Vec<usize> = swedish_second_section_clubs(l, swedish_second_section_key(section as u8)).iter().take(n).map(|c| c.addr()).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, section);
    if plan.len() == n {
        compare_category_b_stage("swe_second_create_section_stage", l, section as u8 as i8 as isize as usize, &plan);
    }
}

/// The club of the first row of a table whose status (`league_table_row::STATUS`) is `status`, if any.
pub(crate) unsafe fn first_club_with_status(t: League, status: u8) -> Option<usize> {
    let mut i = 0i16;
    while i < t.club_count() {
        let row = t.table() + i as usize * league_table_row::SIZE;
        if read_u8(row + league_table_row::STATUS) == status {
            return Some(read_usize(row));
        }
        i += 1;
    }
    None
}

/// The club of each of the six sections (the North at −1 in the league's own table, the others in stages 0 to 4)
/// whose row has `status` (`first_club_with_status`), in section order; none: 0 (the original leaves stack leftovers
/// in that slot; never seen, every section has its winner and runner-up).
pub(crate) unsafe fn swedish_second_section_places(l: League, status: u8) -> [usize; 6] {
    let mut v = [0usize; 6];
    for (k, slot) in v.iter_mut().enumerate() {
        let t = if k == 0 { l } else { League(read_usize(l.stages() + 4 * (k - 1))) };
        if let Some(c) = first_club_with_status(t, status) {
            *slot = c;
        }
    }
    v
}

/// Builds the promotion play-off of the Swedish Second Division as stage 5: the six section winners (row status 1,
/// `swedish_second_section_places`) in a local list, the round table of stage 5 (`stage_round_table`), the Cup_Stage
/// (`league_cup_stage`: six clubs, stage 5, 1) into slot 5; none: the box at line 0x2e0 (the round table not freed).
/// Otherwise the round table freed, its round drawn (Cup::draw_round `exe::cup_draw_round`(stage, 0)) and the current
/// stage = 5.
/// Original: `SweSecond::create_promotion_play_off_stage` (0x892a10, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_create_promotion_play_off_stage(this: usize) {
    let l = League(this);
    let list = swedish_second_section_places(l, 1); // rx09_9: the six section winners
    let (table, rounds, extra) = stage_round_table(l, 5);
    let p = league_cup_stage(l, 6, list.as_ptr() as usize, table, rounds, extra, 5, 1, 0);
    write_usize(l.stages() + 4 * 5, p);
    if read_usize(l.stages() + 4 * 5) == 0 {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x2e0);
    }
    covered!("rx09_9 swedish second promotion play-off");
    exe::crt_free(table);
    exe::cup_draw_round(read_usize(l.stages() + 4 * 5), 0); // rx09_9: the promotion ties drawn
    l.set_current_stage(5);
}

/// Comparer for `swe_second_create_promotion_play_off_stage`: only the original runs (it allocates and draws); the
/// stage's clubs and the current stage against the planned winners.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_promotion_play_off_stage(this: usize) {
    let l = League(this);
    let plan = swedish_second_section_places(l, 1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    compare_league_cup_stage("swe_second_create_promotion_play_off_stage", l, 5, &plan);
}

/// The seed levels of the relegation play-off's 18 clubs (a local array of the original): 2 for the twelve lower
/// division clubs, 1 for the six section runners-up.
pub(crate) const SWEDISH_RELEGATION_PLAY_OFF_SEEDS: [u8; 18] = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1];

/// The Swedish Lower Division clubs in database order (at most 400): among the first [`G_CLUB_COUNT`] − 2 ×
/// [`G_NATION_COUNT`] clubs, those with a nation (`club::NATION`) and a division whose ids are Sweden's
/// ([`KEY_NATION_SWEDEN`]) and the Lower Division's ([`KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`]).
pub(crate) unsafe fn swedish_lower_division_clubs() -> Vec<usize> {
    let mut v = Vec::new();
    for c in domestic_clubs() {
        let nation = read_usize(c.addr() + club::NATION);
        let division = read_usize(c.addr() + club::DIVISION);
        if nation != 0 && division != 0 && read_usize(division) == read_usize(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION) && read_usize(nation) == read_usize(KEY_NATION_SWEDEN) {
            v.push(c.addr());
            if v.len() == 0x190 {
                break;
            }
        }
    }
    v
}

/// Builds the relegation play-off of the Swedish Second Division as stage 6: the Swedish Lower Division clubs
/// (`swedish_lower_division_clubs`, in a local array of 400); fewer than 24: the box at line 0x313 and nothing else.
/// Otherwise they are sorted by reputation and the best 72 shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list,
/// n, 0x18)); the ids of the first twelve are stored at +0xee (the clubs promoted at the turnover [inference]); the
/// next twelve and the six section runners-up (row status 2, `swedish_second_section_places`) make the stage's list
/// of 18 with the seed levels `SWEDISH_RELEGATION_PLAY_OFF_SEEDS`; the round table of stage 6, the Cup_Stage
/// (`league_cup_stage`: 18 clubs, stage 6, 1, the seeds) into slot 6; none: the box at line 0x33f (the round table not
/// freed). Otherwise its first round set up (Cup::set_up_round `exe::cup_set_up_round`(stage, 0)), the round table
/// freed and the current stage = 6.
/// Original: `SweSecond::create_relegation_play_off_stage` (0x892bb0, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_create_relegation_play_off_stage(this: usize) {
    let l = League(this);
    let lower = swedish_lower_division_clubs();
    if lower.len() < 0x18 {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x313);
    }
    covered!("rx09_9 swedish second relegation play-off");
    let mut all = [0usize; 0x190];
    all[..lower.len()].copy_from_slice(&lower);
    exe::comp_util_sort_and_shuffle_top_clubs(all.as_mut_ptr() as usize, lower.len() as u32, 0x18);
    for k in 0..12 {
        write_usize(this + 0xee + 4 * k, read_usize(all[k])); // +0xee: the twelve promoted club ids
    }
    let mut list = [0usize; 0x12];
    list[..12].copy_from_slice(&all[12..24]); // rx09_9: the next twelve play the runners-up
    list[12..].copy_from_slice(&swedish_second_section_places(l, 2));
    let (table, rounds, extra) = stage_round_table(l, 6);
    let p = league_cup_stage(l, 0x12, list.as_ptr() as usize, table, rounds, extra, 6, 1, SWEDISH_RELEGATION_PLAY_OFF_SEEDS.as_ptr() as usize);
    write_usize(l.stages() + 4 * 6, p);
    if read_usize(l.stages() + 4 * 6) == 0 {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x33f);
    }
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 6), 0);
    exe::crt_free(table);
    l.set_current_stage(6);
}

/// Comparer for `swe_second_create_relegation_play_off_stage`: only the original runs (it shuffles and allocates); with
/// enough lower division clubs, the stage holds the six runners-up and twelve lower division clubs, the twelve ids at
/// +0xee are lower division clubs outside the stage, and the current stage is 6.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_relegation_play_off_stage(this: usize) {
    let l = League(this);
    let lower = swedish_lower_division_clubs();
    let runners_up = swedish_second_section_places(l, 2);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    if lower.len() < 0x18 {
        return;
    }
    let stage = read_usize(l.stages() + 4 * 6);
    let clubs = if stage == 0 { Vec::new() } else { league_cup_stage_clubs(stage) };
    let ids: Vec<u32> = (0..12).map(|k| read_u32(this + 0xee + 4 * k)).collect();
    let in_stage_lower = clubs.iter().filter(|c| lower.contains(c)).count();
    let promoted_ok = ids.iter().all(|&id| lower.iter().any(|&c| read_u32(c) == id) && !clubs.iter().any(|&c| read_u32(c) == id));
    let same = l.current_stage() == 6 && clubs.len() == 0x12 && runners_up.iter().all(|c| clubs.contains(c)) && in_stage_lower == 12 && promoted_ok;
    SCOREBOARD.record(same, || format!("swe_second_create_relegation_play_off_stage({:#x}) current {} clubs {:x?} runners-up {:x?} promoted {:x?}", this, l.current_stage() as i32, clubs,
        runners_up, ids));
}
