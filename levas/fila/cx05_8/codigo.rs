// ---------- cx05_8: the European Championship groups and predicted positions (euro_champ.cpp) ----------

/// The predicted final places of the 16 European Championship finalists, by reputation.
pub(crate) const EURO_CHAMP_POSITIONS: [u32; 16] = [1, 2, 3, 3, 5, 5, 5, 5, 9, 9, 9, 9, 13, 13, 13, 13];

/// The national team of a draw-list entry (5 bytes: the team's id, then its group from 1): the club record of that id.
unsafe fn drawn_entry_team(entry: usize) -> usize {
    club_at(read_u32(entry) as isize).addr()
}

/// Group 1 of the European Championship in the cup's own table: 4 teams (`.set_club_count()`), the table
/// (`exe::crt_malloc`(0xec); out of memory: the box at line 0x208 and 0) and, over the 16 entries of the draw list
/// `list` while fewer than 4 rows are filled, each entry of group 1 in the next row (`exe::league_init_table_row`(this,
/// row, team, 0, 0)); the group draw news (`exe::league_send_group_draw_news`) and 1.
/// Original: `EuroChamp::create_group_table` (0x57b530, euro_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_create_group_table(this: usize, list: usize) -> u32 {
    let league = League(this);
    league.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    league.set_table(t);
    if t == 0 {
        show_error_at(0x9c4638, 0x208);
        return 0;
    }
    covered!("cx05_8 euro group 1");
    let mut row = 0i16;
    let mut k = 0i16;
    while row < 4 {
        let entry = list + k as usize * 5;
        if read_u8(entry + 4) == 1 { // cx05_8: the entries of group 1
            // simplification: the row goes in a dword whose high bytes are register leftovers; 0x687430 reads only the byte
            exe::league_init_table_row(league, row as u8 as u32, drawn_entry_team(entry), 0, 0);
            row += 1;
        }
        k += 1;
        if k >= 0x10 {
            break;
        }
    }
    exe::league_send_group_draw_news(league);
    1
}

/// Comparer for `euro_champ_create_group_table`: only the original (the table and the rows allocate, the news goes
/// out); then the plan: 1, four teams and, in order, the entries of group 1 in rows 0 to 3.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_create_group_table(this: usize, list: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    if list == 0 {
        return o(this, list);
    }
    let expected: Vec<usize> = (0..16usize).map(|k| list + k * 5).filter(|&e| read_u8(e + 4) == 1).take(4).map(|e| drawn_entry_team(e)).collect();
    let r = o(this, list);
    let league = League(this);
    let t = league.table();
    let read: Vec<usize> = if t != 0 { (0..expected.len()).map(|i| read_usize(t + i * league_table_row::SIZE)).collect() } else { Vec::new() };
    let same = r == 1 && league.club_count() == 4 && read == expected;
    SCOREBOARD.record(same, || format!("euro_champ_create_group_table({:#x}) returned {} rows {:x?} expected {:x?}", this, r, read, expected));
    r
}

/// Group g + 2 (g = 0 to 2: groups 2 to 4) of the European Championship as a League_Stage: a vector of `.club_count()`
/// (`exe::crt_malloc`; out of memory: the box at line 0x2b8 and 0) with the entries of the draw list `list` whose group
/// is g + 2 (over the 16 entries while fewer than 4 are found), then `nations_group_stage` of cx05_3 with
/// `.club_count()` teams (no stage: the box at line 0x2d2).
/// Original: `EuroChamp::create_group_stage` (0x57b910, euro_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn euro_champ_create_group_stage(this: usize, g: i32, list: usize) -> u32 {
    let league = League(this);
    let teams = exe::crt_malloc(((league.club_count() as i32) << 2) as u32);
    if teams == 0 {
        show_error_at(0x9c4638, 0x2b8);
        return 0;
    }
    covered!("cx05_8 euro group stage");
    let group = g + 2; // cx05_8: the stage's group in the draw list
    let mut filled = 0i16;
    let mut k = 0i16;
    while filled < 4 {
        let entry = list + k as usize * 5;
        if read_u8(entry + 4) as i8 as i32 == group {
            write_usize(teams + filled as usize * 4, drawn_entry_team(entry));
            filled += 1;
        }
        k += 1;
        if k >= 0x10 {
            break;
        }
    }
    nations_group_stage(league, g, league.club_count() as u16 as u32, teams, 0x9c4638, 0x2d2)
}

/// Comparer for `euro_champ_create_group_stage`: only the original (the stage allocates, the news goes out); then,
/// when it returns 1, the plan: the stage at [+0xc][g], +0x30 = g, four teams, the cup's year and the entries of group
/// g + 2 in its table (in any order: the constructor sorts the table).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_create_group_stage(this: usize, g: i32, list: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(%%I1%%);
    if list == 0 || !(0..3).contains(&g) {
        return o(this, g, list);
    }
    let mut plan: Vec<usize> = (0..16usize).map(|k| list + k * 5).filter(|&e| read_u8(e + 4) as i8 as i32 == g + 2).take(4).map(|e| drawn_entry_team(e)).collect();
    let r = o(this, g, list);
    if r == 1 {
        let league = League(this);
        let f = League(read_usize(league.stages() + g as usize * 4));
        let t = if !f.is_null() { f.table() } else { 0 };
        let mut read: Vec<usize> = if t != 0 { (0..plan.len()).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        read.sort_unstable();
        plan.sort_unstable();
        let ok = !f.is_null() && league.current_stage() as i32 == g && f.year() == league.year() && read == plan;
        SCOREBOARD.record(ok, || format!("euro_champ_create_group_stage({}): the stage does not match the plan ({:x?} read {:x?})", g, plan, read));
    }
    r
}

/// The predicted positions of the European Championship: without statistics the box at line 0x4e7; a vector of the
/// entrants (`exe::crt_malloc`(4 × `.entrant_count()`); out of memory: the box at line 0x4ef); the European national
/// teams released from their competitions (`exe::nationalteams_release_competitions_of_continent`(national teams, the
/// continent record of [`KEY_CONTINENT_EUROPE`], 1)); the entrants (the teams of the entrant book) sorted by reputation
/// (`predicted_by_reputation_sort`) get 1, 2, two 3rds, four 5ths, four 9ths and four 13ths
/// (`predicted_by_reputation_write`); each entrant's nation gets the European Championship as its current competition
/// (`exe::nationalteams_set_current_competition`(national teams, nation, the competition record)); the vector is freed.
/// Original: `EuroChamp::set_predicted_positions` (0x57c9f0, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_set_predicted_positions(this: usize) {
    let league = League(this);
    if league.comp_stats() == 0 {
        return show_error_at(0x9c4638, 0x4e7);
    }
    let v = exe::crt_malloc(((league.entrant_count() as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0x9c4638, 0x4ef);
    }
    let europe = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_EUROPE).wrapping_mul(0xc6));
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), europe, 1);
    let mut k = 0i16;
    while k < league.entrant_count() {
        write_usize(v + k as usize * 4, read_usize(league.entrant_book() + k as usize * 6));
        k += 1;
    }
    predicted_by_reputation_sort(v, league.entrant_count() as i32 as u32);
    covered!("cx05_8 euro predicted");
    predicted_by_reputation_write(this, v, &EURO_CHAMP_POSITIONS);
    let mut k = 0i16;
    while k < league.entrant_count() {
        let team = Club(read_usize(v + k as usize * 4));
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation().addr(), league.competition().addr()); // cx05_8: the finalists' current competition
        k += 1;
    }
    exe::crt_free(v);
}

/// Comparer for `euro_champ_set_predicted_positions`: only the original (the statistics add the entries, the national
/// teams change competition), then each entrant's position in the statistics by reputation
/// (`predicted_by_reputation_check`).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let league = League(this);
    let (book, n) = (league.entrant_book(), league.entrant_count());
    let teams: Vec<usize> = if book != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| read_usize(book + k * 6)).collect() } else { Vec::new() };
    o(this);
    predicted_by_reputation_check("euro_champ_set_predicted_positions", this, &teams, &EURO_CHAMP_POSITIONS);
}
