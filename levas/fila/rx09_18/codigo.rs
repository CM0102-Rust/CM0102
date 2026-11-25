// ---------- rx09_18: the Asian World Cup qualifying third round, play-off and predicted positions (wc_asia_league.cpp) ----------

/// `WcEuropeLeague` 0x91def0 (thiscall, ret 0): the European nation drawn for the Asian play-off [inference: the first
/// team of the European runners-up stage, +0x100, that is not in the play-off stage, +0x101].
const FN_WC_EUROPE_LEAGUE_ASIAN_PLAY_OFF_NATION: usize = 0x91def0;

/// The two sides of the Asian third round tie: from each second round group (stages 9 and 10), the third-placed team
/// (table row 2) with no exempt nation, else the runner-up (row 1).
pub(crate) unsafe fn wc_asia_third_round_sides(this: usize) -> [usize; 2] {
    let l = League(this);
    let row = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 2 } else { 1 }; // rx09_18: the third-placed sides with no exempt nation
    [9usize, 10].map(|s| read_usize(exe::league_get_table_row(read_usize(l.stages() + 4 * s), row)))
}

/// Builds the Asian third round tie as stage 11: a list of two (`exe::crt_malloc` of 8; none: the box at line 0x552)
/// with `wc_asia_third_round_sides`, the round table of stage 11 (`stage_round_table`), the Cup_Stage (`league_cup_stage`:
/// two teams, stage 11, 4) into slot 11; the list and the round table freed; none: the box at line 0x572. Otherwise its
/// round set up (Cup::set_up_round `exe::cup_set_up_round`(stage, 0)) and the current stage = 11.
/// Original: `WcAsiaLeague::create_third_round_tie` (0x913b60, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_third_round_tie(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x552);
    }
    let sides = wc_asia_third_round_sides(this);
    write_usize(list, sides[0]);
    write_usize(list + 4, sides[1]);
    let (table, rounds, extra) = stage_round_table(l, 0xb);
    let p = league_cup_stage(l, 2, list, table, rounds, extra, 0xb, 4, 0);
    write_usize(l.stages() + 4 * 0xb, p);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(l.stages() + 4 * 0xb) == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x572);
    }
    covered!("rx09_18 asian third round tie");
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 0xb), 0);
    l.set_current_stage(0xb);
}

/// Comparer for `wc_asia_league_create_third_round_tie`: only the original runs (it allocates); the tie's teams and the
/// current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_third_round_tie(this: usize) {
    let plan = wc_asia_third_round_sides(this);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    compare_league_cup_stage("wc_asia_league_create_third_round_tie", League(this), 0xb, &plan);
}

/// The Asian side of the play-off: the first entry of the third round tie's team table (`cup::TEAM_TABLE`, the count
/// re-read) whose status is 2 (lost) with one exempt nation, else 1 (won); none: None (the original leaves the slot as
/// malloc returned it).
pub(crate) unsafe fn wc_asia_play_off_side(this: usize) -> Option<usize> {
    let tie = Cup(read_usize(League(this).stages() + 4 * 0xb));
    let mut i = 0i16;
    while i < tie.team_count() {
        let want = if read_u8(this + WC_ASIA_EXEMPT) == 1 { 2 } else { 1 }; // rx09_18: the third round winner goes on
        let e = tie.team_table() + 6 * i as usize;
        if read_u8(e + cup_team_entry::STATUS) == want {
            return Some(read_usize(e + cup_team_entry::CLUB));
        }
        i += 1;
    }
    None
}

/// The European nation of the play-off (WcEuropeLeague 0x91def0 on the European section's object:
/// `KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION` in [`G_COMP_OBJECT_TABLE`]).
pub(crate) unsafe fn wc_asia_play_off_european_side() -> usize {
    let europe = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION).wrapping_mul(4)));
    let pick: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_WC_EUROPE_LEAGUE_ASIAN_PLAY_OFF_NATION);
    pick(europe)
}

/// Builds the Asian play-off as stage 12: a list of two (`exe::crt_malloc` of 8; none: the box at line 0x591) with the
/// Asian side (`wc_asia_play_off_side`; when none, the slot is left as malloc returned it) and the European one
/// (`wc_asia_play_off_european_side`); the round table of stage 12, the Cup_Stage (`league_cup_stage`: two teams, stage
/// 12, 4) into slot 12; the list and the round table freed; none: the box at line 0x5be. Otherwise its round set up
/// (`exe::cup_set_up_round`(stage, 0)) and the current stage = 12.
/// Original: `WcAsiaLeague::create_playoff_tie` (0x913d50, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_playoff_tie(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x591);
    }
    if let Some(side) = wc_asia_play_off_side(this) {
        write_usize(list, side);
    }
    write_usize(list + 4, wc_asia_play_off_european_side());
    let (table, rounds, extra) = stage_round_table(l, 0xc);
    let p = league_cup_stage(l, 2, list, table, rounds, extra, 0xc, 4, 0);
    write_usize(l.stages() + 4 * 0xc, p);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(l.stages() + 4 * 0xc) == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x5be);
    }
    covered!("rx09_18 asian play-off");
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 0xc), 0);
    l.set_current_stage(0xc);
}

/// Comparer for `wc_asia_league_create_playoff_tie`: only the original runs (it allocates); with an Asian side, the
/// tie's teams and the current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_playoff_tie(this: usize) {
    let side = wc_asia_play_off_side(this);
    let europe = wc_asia_play_off_european_side();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    if let Some(side) = side {
        compare_league_cup_stage("wc_asia_league_create_playoff_tie", League(this), 0xc, &[side, europe]);
    }
}

/// The predicted place of the Asian entrant in place k (by reputation), by the exempt count: none exempt, 1 for the top
/// four, then 5, 6, 7 (to the eighth), 9 (to the tenth), 11 (to the twentieth), 21 (to the thirtieth), 31; one or two,
/// 1 for the top two, then 3, 4, 5 (to the sixth), 7 (to the eighth), 9, 11, 21, 31; three, 1, 2, 3 (to the fourth), 5,
/// 7, 9, 11, 21, 31; any other count, no place.
pub(crate) fn wc_asia_predicted_place(exempt: i8, k: i16) -> Option<u8> {
    let bands: &[(i16, u8)] = match exempt {
        0 => &[(4, 1), (5, 5), (6, 6), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        1 | 2 => &[(2, 1), (3, 3), (4, 4), (6, 5), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        3 => &[(1, 1), (2, 2), (4, 3), (6, 5), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        _ => return None,
    };
    Some(bands.iter().find(|&&(limit, _)| k < limit).map_or(0x1f, |&(_, p)| p)) // rx09_18: the reputation bands
}

/// The predicted positions of the Asian qualifying: without statistics, the box at line 0x6bb and nothing else. The
/// entrants (`wc_turnover_consts::ENTRANT_LIST`, `WC_ASIA_ENTRANT_COUNT`) sorted by reputation (qsort `exe::crt_qsort`
/// with 0x4b07d0); the Asian national teams released from the continent's competitions
/// (`exe::nationalteams_release_competitions_of_continent`(the Asian continent record, 0) on [`G_NATIONAL_TEAMS`]);
/// then each entrant k gets its predicted place (`wc_asia_predicted_place` with the exempt count read again each time;
/// `exe::comp_stats_set_club_final_position`) and this competition as its nation's current one
/// (`exe::nationalteams_set_current_competition`(the entrant's nation, the record)).
/// Original: `WcAsiaLeague::set_predicted_positions` (0x9144d0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x6bb);
    }
    covered!("rx09_18 asian predicted positions");
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    exe::crt_qsort(list, read_u32(this + WC_ASIA_ENTRANT_COUNT), 4, 0x4b07d0);
    let continent = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_ASIA).wrapping_mul(0xc6));
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), continent, 0);
    let mut k = 0i16;
    while (k as i32) < read_i32(this + WC_ASIA_ENTRANT_COUNT) {
        let team = read_usize(list + 4 * k as usize);
        if let Some(p) = wc_asia_predicted_place(read_u8(this + WC_ASIA_EXEMPT) as i8, k) {
            exe::comp_stats_set_club_final_position(l.comp_stats(), team, p as u32);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), read_usize(team + club::NATION), l.competition().addr());
        k = k.wrapping_add(1);
    }
}

/// Comparer for `wc_asia_league_set_predicted_positions`: only the original runs (it sorts the list and moves the
/// national teams); then each entrant's predicted place byte (`comp_stats_club::FINAL_POSITION`) against the plan, over
/// the sorted list.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_set_predicted_positions(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let l = League(this);
    let stats = l.comp_stats();
    if stats == 0 {
        return;
    }
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let exempt = read_u8(this + WC_ASIA_EXEMPT) as i8;
    let mut bad = Vec::new();
    for k in 0..read_i32(this + WC_ASIA_ENTRANT_COUNT).max(0) as i16 {
        let e = exe::comp_stats_find_club_entry(stats, read_usize(list + 4 * k as usize));
        if let (Some(p), true) = (wc_asia_predicted_place(exempt, k), e != 0) {
            if read_u8(e + comp_stats_club::FINAL_POSITION) != p {
                bad.push((k, p, read_u8(e + comp_stats_club::FINAL_POSITION)));
            }
        }
    }
    SCOREBOARD.record(bad.is_empty(), || format!("wc_asia_league_set_predicted_positions({:#x}) exempt {}: (place, plan, original) {:?}", this, exempt, bad));
}
