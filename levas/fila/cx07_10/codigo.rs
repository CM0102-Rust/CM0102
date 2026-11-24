// ---------- cx07_10: a league's temporary file (League::save_to_temp_file, league.cpp) ----------

/// Turns the club pointers at `first`, `first + stride`, ... (`count` of them) into their ids (−1 for none), in place.
unsafe fn club_pointers_to_ids(first: usize, stride: usize, count: i32) {
    for i in 0..count.max(0) as usize {
        let e = first + i * stride;
        let club = read_usize(e);
        write_u32(e, if club != 0 { read_u32(club) } else { u32::MAX });
    }
}

/// Turns the club ids at `first`, `first + stride`, ... (`count` of them) back into club pointers (null for −1), in
/// place.
unsafe fn club_ids_to_pointers(first: usize, stride: usize, count: i32) {
    for i in 0..count.max(0) as usize {
        let e = first + i * stride;
        let id = read_i32(e);
        write_usize(e, if id != -1 { club_at(id as isize).addr() } else { 0 });
    }
}

/// The table of a league in its temporary file: nothing without clubs. Otherwise each row's club and, when row 0 has
/// head-to-head entries, each row's `club_count − 1` entries' opponents (14 bytes each) become ids; the rows
/// (`league_table_row::SIZE` each) are written; then the head-to-head flag (4 bytes: whether row 0 has entries) and, with
/// it, each row's entries; then the position-history flag (whether row 0 has a history) and, with it,
/// `league::GAMES_PER_CLUB` bytes of each row's history (the count made `(club_count − 1) × legs` first when it is −1).
/// The ids become pointers again (only rows and, when row 0 has entries, the entries).
unsafe fn league_save_table(this: League, f: usize) {
    let n = this.club_count() as i32;
    if n <= 0 {
        return;
    }
    let t = this.table();
    let row = |i: i32| this.table() + i as usize * league_table_row::SIZE;
    let has_h2h = read_usize(t + league_table_row::H2H_ENTRIES) != 0; // the original checks row 0 for every row
    for i in 0..n {
        club_pointers_to_ids(row(i) + league_table_row::CLUB, 0, 1);
        if has_h2h {
            club_pointers_to_ids(read_usize(row(i) + league_table_row::H2H_ENTRIES), 0xe, n - 1);
        }
    }
    temp_write(f, this.table(), league_table_row::SIZE as u32, n as u32);
    let h2h_flag = has_h2h as u32;
    temp_write(f, &h2h_flag as *const u32 as usize, 4, 1);
    if h2h_flag != 0 {
        for i in 0..n {
            temp_write(f, read_usize(row(i) + league_table_row::H2H_ENTRIES), 0xe, (n - 1) as u32);
        }
    }
    let history_flag = (read_usize(this.table() + league_table_row::POSITION_HISTORY) != 0) as u32;
    temp_write(f, &history_flag as *const u32 as usize, 4, 1);
    if history_flag != 0 {
        covered!("cx07_10 league position history saved");
        if read_i16(this.addr() + league::GAMES_PER_CLUB) == -1 {
            write_i16(this.addr() + league::GAMES_PER_CLUB, ((n - 1) as i16).wrapping_mul(this.legs()));
        }
        for i in 0..n {
            temp_write(f, read_usize(row(i) + league_table_row::POSITION_HISTORY), 1, read_i16(this.addr() + league::GAMES_PER_CLUB) as i32 as u32);
        }
    }
    for i in 0..n {
        club_ids_to_pointers(row(i) + league_table_row::CLUB, 0, 1);
        if read_usize(this.table() + league_table_row::H2H_ENTRIES) != 0 {
            club_ids_to_pointers(read_usize(row(i) + league_table_row::H2H_ENTRIES), 0xe, n - 1);
        }
    }
}

/// Rule: a league's state goes to its CM3_TEMP file in the order `League::load_from_temp_file` reads it back
/// (docs/MAPA_SAVE.en.md §6): the name, the statistics, the class's own data, the header fields, the round dates, the
/// points-average table, the table with its head-to-head records and position history, the stages and the entrant book.
///
/// `League::save_to_temp_file(by_id)`. The file name at +0x53 (`cup_temp_file_name` of cx01_7: the competition's name
/// with '/' and '.' made '_', "%s_%d.tmp"): with `by_id` (its low byte) the record's id; otherwise, when the league is not
/// `league::ACTIVE`, nothing is written, and when it is, the year `.year()`. Opened in CM3_TEMP ("wb",
/// `open_temp_file_for_write`; failing: the box at line 0x13e7). Then, with fwrite (`temp_write`, cdecl): the 0x50-byte
/// name; whether there are statistics (4 bytes) and, with them, `Comp_Stats::save` (0x4ac4e0, thiscall, ret 4); the class's
/// own data (virtual +0x24, thiscall, ret 4); `league::REQUIRED_CAPACITY`, `league::REQUIRED_SEATS`, `league::PRIZE_MODE`,
/// `league::PRIZE_FUND`, `league::COMPLETE`, `league::LAST_UPDATE_DATE` (8 bytes), `league::GAMES_PER_CLUB`,
/// `league::SQUAD_FLAGS`, `league::IMPORTANCE`, `league::ENTRANTS_ARE_QUALIFIERS`, `league::SEASON_CLOSED`,
/// `league::FIXED_STADIUM_ID`, `league::SUBSTITUTES_NAMED`, `league::SUBSTITUTIONS_ALLOWED`, `league::NEXT_DRAW_DAY`,
/// `league::NEXT_DRAW_YEAR_OFFSET`, `league::ACTIVE`, `league::STOPPED`, `league::LEGS`, `league::CLUB_COUNT`,
/// `league::YEAR`, `league::COMP_TYPE`, `league::CLUB_SIDES`, `league::STAGE_ID`; the competition id (+0 of the record;
/// without a record the box at line 0x141e and −1); `league::ROUND_COUNT`, `league::WIN_POINTS`, `league::DRAW_POINTS`,
/// `league::MATCH_CATEGORY`, `league::TIEBREAKS` (4 bytes), the four band counts from `league::PROMOTED_COUNT` (4 bytes),
/// `league::LEAGUE_FLAGS`, `league::ROUND_DRAW_RULES`, `league::MATCH_RULES`, `league::UPPER_DIVISION_ID`,
/// `league::LOWER_DIVISION_ID`, `league::TIEBREAK_MATCHES`; the round dates (0x41 bytes each, `league::ROUND_COUNT` of
/// them); the points-average table pointer +0x18 (as it is) and its row count +0x34 and, with the pointer, its rows
/// (0x13 bytes each); the table (`league_save_table`); `league::STAGE_COUNT`, `league::CURRENT_STAGE`,
/// `league::STAGE_INDEX` and the stages (`cup_save_stages` of cx01_7: each stage's type and counts and its own
/// save_to_file); `league::ENTRANT_CAPACITY`, `league::ENTRANT_COUNT` and the entrant book with its clubs as ids
/// (`club_entries_to_ids` of cx01_7, the capacity's entries written, the ids made pointers again). The file closed.
/// Original: `League::save_to_temp_file` (0x6891c0, league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn league_save_to_temp_file(this: usize, by_id: u32) {
    let this = League(this);
    let has_stats = (this.comp_stats() != 0) as u32;
    let no_competition = u32::MAX;
    if by_id as u8 != 0 {
        cup_temp_file_name(this, read_i32(this.competition().addr()));
    } else {
        if this.active() == 0 {
            return;
        }
        cup_temp_file_name(this, this.year() as i32);
    }
    let name = this.addr() + 0x53;
    let f = open_temp_file_for_write(name);
    if f == 0 {
        return show_error_at(LEAGUE_SOURCE, 0x13e7);
    }
    covered!("cx07_10 league saved");
    temp_write(f, name, 0x50, 1);
    temp_write(f, &has_stats as *const u32 as usize, 4, 1);
    if has_stats != 0 {
        let save: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_STATS_SAVE);
        save(this.comp_stats(), f);
    }
    let class_data: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x24));
    class_data(this.addr(), f);
    let a = this.addr();
    for (field, size) in [(league::REQUIRED_CAPACITY, 2), (league::REQUIRED_SEATS, 2), (league::PRIZE_MODE, 1), (league::PRIZE_FUND, 4),
        (league::COMPLETE, 4), (league::LAST_UPDATE_DATE, 8), (league::GAMES_PER_CLUB, 2), (league::SQUAD_FLAGS, 1), (league::IMPORTANCE, 1),
        (league::ENTRANTS_ARE_QUALIFIERS, 1), (league::SEASON_CLOSED, 4), (league::FIXED_STADIUM_ID, 4), (league::SUBSTITUTES_NAMED, 1),
        (league::SUBSTITUTIONS_ALLOWED, 1), (league::NEXT_DRAW_DAY, 2), (league::NEXT_DRAW_YEAR_OFFSET, 2), (league::ACTIVE, 4), (league::STOPPED, 1),
        (league::LEGS, 2), (league::CLUB_COUNT, 2), (league::YEAR, 2), (league::COMP_TYPE, 1), (league::CLUB_SIDES, 1), (league::STAGE_ID, 2)] {
        temp_write(f, a + field, size, 1);
    }
    if !this.competition().is_null() {
        temp_write(f, this.competition().addr(), 4, 1);
    } else {
        show_error_at(LEAGUE_SOURCE, 0x141e);
        temp_write(f, &no_competition as *const u32 as usize, 4, 1);
    }
    for (field, size) in [(league::ROUND_COUNT, 2), (league::WIN_POINTS, 1), (league::DRAW_POINTS, 1), (league::MATCH_CATEGORY, 1), (league::TIEBREAKS, 4),
        (league::PROMOTED_COUNT, 4), (league::LEAGUE_FLAGS, 2), (league::ROUND_DRAW_RULES, 2), (league::MATCH_RULES, 2), (league::UPPER_DIVISION_ID, 4),
        (league::LOWER_DIVISION_ID, 4), (league::TIEBREAK_MATCHES, 4)] {
        temp_write(f, a + field, size, 1);
    }
    temp_write(f, this.round_dates(), 0x41, this.round_count() as i32 as u32);
    // the points-average table: its pointer +0x18 as it is, its row count +0x34 and its 0x13-byte rows
    temp_write(f, a + 0x18, 4, 1);
    temp_write(f, a + 0x34, 2, 1);
    let averages = read_usize(a + 0x18);
    if averages != 0 {
        temp_write(f, averages, 0x13, read_i16(a + 0x34) as i32 as u32);
    }
    league_save_table(this, f);
    temp_write(f, a + league::STAGE_COUNT, 4, 1);
    temp_write(f, a + league::CURRENT_STAGE, 4, 1);
    temp_write(f, a + league::STAGE_INDEX, 1, 1);
    cup_save_stages(this, f);
    temp_write(f, a + league::ENTRANT_CAPACITY, 2, 1);
    temp_write(f, a + league::ENTRANT_COUNT, 2, 1);
    let book = this.entrant_book();
    club_entries_to_ids(book, this.entrant_count());
    if book != 0 {
        temp_write(f, book, 6, this.entrant_capacity() as i32 as u32);
    }
    club_entries_from_ids(book, this.entrant_count());
    exe::crt_fclose(f);
}

/// Comparer for `league_save_to_temp_file`: the original writes the file, which is read back (`cup_temp_file_bytes` of
/// cx01_7); the copy writes it again and the two contents (and the name at +0x53) must match. The file is left as the
/// copy wrote it, the same bytes.
pub(crate) unsafe extern "thiscall" fn compare_league_save_to_temp_file(this: usize, by_id: u32) {
    let name = |p: usize| { let s = core::slice::from_raw_parts(p as *const u8, 0x50); s[..s.iter().position(|&c| c == 0).unwrap_or(0x50)].to_vec() };
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, by_id);
    let name_o = name(this + 0x53);
    let o = cup_temp_file_bytes(this);
    league_save_to_temp_file(this, by_id);
    let name_r = name(this + 0x53);
    let r = cup_temp_file_bytes(this);
    SCOREBOARD.record(name_o == name_r && o == r, || format!("league_save_to_temp_file({:#x}, {}): file {:?} {} bytes, copy {:?} {} bytes, first difference at {:?}", this, by_id as u8,
        String::from_utf8_lossy(&name_o), o.len(), String::from_utf8_lossy(&name_r), r.len(), o.iter().zip(r.iter()).position(|(a, b)| a != b)));
}
