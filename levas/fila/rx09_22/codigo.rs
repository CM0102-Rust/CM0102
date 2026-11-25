// ---------- rx09_22: the league stage written to the saved game (league_stage.cpp) ----------

/// Writes a league stage to the open temporary file (thiscall(file), ret 4; every write unchecked). Without round dates
/// or a table: the box at line 0x259 and nothing written. Otherwise the fields in the loader's order
/// (`LEAGUE_STAGE_SAVED_HEAD`), the competition's id (the first dword of its record; none: the box at line 0x276 and −1),
/// `LEAGUE_STAGE_SAVED_RULES`, the round dates (0x41 bytes a round), the dword at +0x18 and the word at +0x34, and the
/// 0x13-byte records at +0x18 (their count, when there are records). With clubs: each row's club pointer becomes the
/// club's id (−1 for none) and, when the FIRST row has head-to-head entries, each row's opponents too; the rows; the
/// head-to-head flag (1 when the first row has entries) and each row's n − 1 entries of 0xe bytes; the position history
/// flag (1 when the first row has a history: a `league::GAMES_PER_CLUB` of −1 becomes (n − 1) × `league::LEGS` first)
/// with each row's history of that many bytes; then the ids back to club pointers (`league_load_club_from_id`) for the
/// rows and (with the first row's entries) the opponents.
/// Original: `League_Stage::save_to_file` (0x68ca40, league_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn league_stage_save_to_file(this: usize, file: usize) {
    let l = League(this);
    if l.round_dates() == 0 || l.table() == 0 {
        return show_error_at(LEAGUE_STAGE_SOURCE, 0x259);
    }
    covered!("rx09_22 league stage saved");
    let write = |p: usize, size: u32, n: u32| {
        exe::crt_fwrite(p, size, n, file);
    };
    for &(o, size, _) in &LEAGUE_STAGE_SAVED_HEAD {
        write(this + o, size, 1);
    }
    let comp = read_usize(this + league::COMPETITION);
    if comp != 0 {
        write(comp, 4, 1);
    } else {
        show_error_at(LEAGUE_STAGE_SOURCE, 0x276);
        let none = -1i32;
        write(&none as *const i32 as usize, 4, 1);
    }
    for &(o, size, _) in &LEAGUE_STAGE_SAVED_RULES {
        write(this + o, size, 1);
    }
    write(l.round_dates(), 0x41, l.round_count() as i32 as u32); // rx09_22: every round's date
    write(this + 0x18, 4, 1); // +0x18: the 0x13-byte records of the competition base
    write(this + 0x34, 2, 1); // +0x34: their count (word)
    let records = read_usize(this + 0x18);
    if records != 0 {
        write(records, 0x13, read_i16(this + 0x34) as i32 as u32);
    }
    let clubs = || l.club_count() as i32;
    if clubs() <= 0 {
        return;
    }
    let row = |k: i32| l.table() + league_table_row::SIZE * k as usize;
    let to_id = |p: usize| {
        let club = read_usize(p);
        write_i32(p, if club != 0 { read_i32(club) } else { -1 });
    };
    let mut has_h2h = 0u32;
    let mut k = 0;
    while k < clubs() {
        to_id(row(k) + league_table_row::CLUB);
        if read_usize(row(0) + league_table_row::H2H_ENTRIES) != 0 {
            has_h2h = 1;
            let mut j = 0;
            while j < clubs() - 1 {
                to_id(read_usize(row(k) + league_table_row::H2H_ENTRIES) + 0xe * j as usize);
                j += 1;
            }
        }
        k += 1;
    }
    write(l.table(), league_table_row::SIZE as u32, clubs() as u32);
    write(&has_h2h as *const u32 as usize, 4, 1);
    if has_h2h != 0 {
        let mut k = 0;
        while k < clubs() {
            write(read_usize(row(k) + league_table_row::H2H_ENTRIES), 0xe, (clubs() - 1) as u32);
            k += 1;
        }
    }
    let mut has_history = 0u32;
    if read_usize(row(0) + league_table_row::POSITION_HISTORY) != 0 {
        if l.games_per_club() == -1 {
            l.set_games_per_club(l.club_count().wrapping_sub(1).wrapping_mul(l.legs()));
        }
        has_history = 1;
        write(&has_history as *const u32 as usize, 4, 1);
        let mut k = 0;
        while k < clubs() {
            write(read_usize(row(k) + league_table_row::POSITION_HISTORY), l.games_per_club() as i32 as u32, 1);
            k += 1;
        }
    } else {
        write(&has_history as *const u32 as usize, 4, 1);
    }
    let mut k = 0;
    while k < clubs() {
        league_load_club_from_id(row(k) + league_table_row::CLUB);
        if read_usize(row(0) + league_table_row::H2H_ENTRIES) != 0 {
            let mut j = 0;
            while j < clubs() - 1 {
                league_load_club_from_id(read_usize(row(k) + league_table_row::H2H_ENTRIES) + 0xe * j as usize);
                j += 1;
            }
        }
        k += 1;
    }
}

/// Comparer for `league_stage_save_to_file`: it writes the saved game (it cannot run twice), so only the original runs;
/// the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_league_stage_save_to_file(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, file)
}
