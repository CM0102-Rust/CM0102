// ---------- rx09_21: the league stage made and read back from the saved game (league_stage.cpp) ----------

/// The source file of the league stages (league_stage.cpp), for the error boxes.
pub(crate) const LEAGUE_STAGE_SOURCE: usize = 0x9f1630;

/// The empty league stage the loader reads into (thiscall(club count, round count), ret 8): League::League
/// (`exe::new_league`), the vtable 0x96d0f0 and the table (`exe::crt_malloc` of 0x3b a club, the word argument; none:
/// the box at line 0xa6 and the end). With rounds, their dates (0x41 each; none: the box at line 0xaf); without, one
/// round dated today ([`G_CURRENT_DATE`] in the day word, [`G_CURRENT_DATE_YEAR`] − the year in the year offset word,
/// 0 in the time slice byte; none: the box at line 0xba). Returns this. (The original's SEH frame has no destructor on
/// the normal path.)
/// Original: `League_Stage::League_Stage` (0x68b4f0, league_stage.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_league_stage_for_load(this: usize, club_count: u32, round_count: u32) -> usize {
    let l = League(this);
    exe::new_league(l);
    write_usize(this, 0x96d0f0);
    let t = exe::crt_malloc((club_count as i16 as i32 * league_table_row::SIZE as i32) as u32);
    l.set_table(t);
    if t == 0 {
        show_error_at(LEAGUE_STAGE_SOURCE, 0xa6);
        return this;
    }
    covered!("rx09_21 league stage for loading");
    let rounds = round_count as i16;
    if rounds > 0 {
        let d = exe::crt_malloc((rounds as i32 * 0x41) as u32);
        l.set_round_dates(d);
        if d == 0 {
            show_error_at(LEAGUE_STAGE_SOURCE, 0xaf);
        }
        return this;
    }
    let d = exe::crt_malloc(0x41);
    l.set_round_dates(d);
    if d == 0 {
        show_error_at(LEAGUE_STAGE_SOURCE, 0xba);
        return this;
    }
    write_u16(d, read_u16(G_CURRENT_DATE));
    write_u16(d + 2, read_u16(G_CURRENT_DATE_YEAR).wrapping_sub(l.year() as u16)); // rx09_21: today's round, as an offset from the year
    write_u8(d + 4, 0);
    this
}

/// Comparer for `construct_league_stage_for_load`: only the original runs (it allocates); the vtable, the table and the
/// round dates made, and for a stage without rounds today's date in its single round.
pub(crate) unsafe extern "thiscall" fn compare_construct_league_stage_for_load(this: usize, club_count: u32, round_count: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I0%%)(this, club_count, round_count);
    let l = League(this);
    let d = l.round_dates();
    let today_ok = round_count as i16 > 0 || d == 0
        || (read_u16(d) == read_u16(G_CURRENT_DATE) && read_u16(d + 2) == read_u16(G_CURRENT_DATE_YEAR).wrapping_sub(l.year() as u16) && read_u8(d + 4) == 0);
    SCOREBOARD.record(r == this && read_usize(this) == 0x96d0f0 && l.table() != 0 && d != 0 && today_ok, || format!("construct_league_stage_for_load({:#x}, {}, {}): vtable {:#x} table {:#x} dates {:#x}",
        this, club_count as i16, round_count as i16, read_usize(this), l.table(), d));
    r
}

/// The fields of a league stage in the saved game, in the original's order up to the round draw rules: (offset, size,
/// line of the fatal box when short; 0 for the stage id, whose read is not checked). The competition (an id, −1 for
/// none) comes after the stage id and is read apart.
pub(crate) const LEAGUE_STAGE_SAVED_HEAD: [(usize, u32, u32); 13] = [
    (league::PRIZE_MODE, 1, 0xff), (league::PRIZE_FUND, 4, 0x105), (league::COMPLETE, 4, 0x10b), (league::GAMES_PER_CLUB, 2, 0x111),
    (league::IMPORTANCE, 1, 0x117), (league::SUBSTITUTES_NAMED, 1, 0x11d), (league::SUBSTITUTIONS_ALLOWED, 1, 0x123), (league::LEGS, 2, 0x129),
    (league::CLUB_COUNT, 2, 0x12f), (league::YEAR, 2, 0x135), (league::COMP_TYPE, 1, 0x13c), (league::CLUB_SIDES, 1, 0x143), (league::STAGE_ID, 2, 0),
];

/// The fields after the competition, in the original's order (offset, size, line of the fatal box).
pub(crate) const LEAGUE_STAGE_SAVED_RULES: [(usize, u32, u32); 12] = [
    (league::ROUND_COUNT, 2, 0x160), (league::WIN_POINTS, 1, 0x167), (league::DRAW_POINTS, 1, 0x16e), (league::MATCH_CATEGORY, 1, 0x175),
    (league::TIEBREAKS, 4, 0x17c), (league::PROMOTED_COUNT, 4, 0x183), (league::LEAGUE_FLAGS, 2, 0x18a), (league::ROUND_DRAW_RULES, 2, 0x191),
    (league::MATCH_RULES, 2, 0x198), (league::UPPER_DIVISION_ID, 4, 0x19f), (league::LOWER_DIVISION_ID, 4, 0x1a6), (league::STAGE_INDEX, 1, 0x1ad),
];

/// Reads a league stage from the open temporary file (thiscall(file), ret 4), field by field
/// (`LEAGUE_STAGE_SAVED_HEAD`, the competition id turned into its record in [`G_CLUB_COMP_TABLE`] (−1: none; short:
/// line 0x151), `LEAGUE_STAGE_SAVED_RULES`); a short read of a checked field shows its box and stops the game
/// (`fatal_error_at`). Then the round dates (0x41 bytes a round into the array the constructor made; short: line 0x1b4,
/// fatal); the 0x13-byte records at +0x18 with their count at +0x34 (both unchecked; when both are there, a new array
/// of the count's records, not checked, read unchecked). With no clubs, 1. Otherwise the table rows (short: line 0x1cc,
/// fatal), the head-to-head flag (short: 0x1d4, fatal): for each row, without it the row's
/// `league_table_row::H2H_ENTRIES` cleared, with it a new array of 0xe bytes per opponent (n − 1; none: the box at line
/// 0x1e2 and 0) read (short: the box at 0x1eb and 0). The position history flag (short: the box at 0x1f7 and 0); a
/// `league::GAMES_PER_CLUB` of −1 becomes (n − 1) × `league::LEGS` (16 bits); with the flag each row gets a new array of
/// that many bytes (none: 0x20c and 0) read as one block (short: 0x215 and 0), without it each row's
/// `league_table_row::POSITION_HISTORY` is cleared. Last, each row's club and (when the FIRST row has head-to-head
/// entries) its opponents' clubs are rebuilt from their ids (`league_load_club_from_id`). Returns 1.
/// Original: `League_Stage::load_from_file` (0x68b720, league_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn league_stage_load_from_file(this: usize, file: usize) -> u32 {
    let l = League(this);
    let read = |p: usize, size: u32, n: u32| exe::crt_fread(p, size, n, file);
    let read_or_die = |p: usize, size: u32, line: u32| {
        if read(p, size, 1) != 1 && line != 0 {
            fatal_error_at(LEAGUE_STAGE_SOURCE, line);
        }
    };
    for &(o, size, line) in &LEAGUE_STAGE_SAVED_HEAD {
        read_or_die(this + o, size, line);
    }
    let mut comp_id = 0i32;
    read_or_die(&mut comp_id as *mut i32 as usize, 4, 0x151);
    write_usize(this + league::COMPETITION, if comp_id == -1 { 0 } else { read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b)) }); // rx09_21: the competition record
    for &(o, size, line) in &LEAGUE_STAGE_SAVED_RULES {
        read_or_die(this + o, size, line);
    }
    let rounds = l.round_count() as i32;
    if read(l.round_dates(), 0x41, rounds as u32) as i32 != rounds {
        fatal_error_at(LEAGUE_STAGE_SOURCE, 0x1b4);
    }
    read(this + 0x18, 4, 1); // +0x18: the 0x13-byte records of the competition base
    read(this + 0x34, 2, 1); // +0x34: their count (word)
    if read_usize(this + 0x18) != 0 && read_u16(this + 0x34) != 0 {
        let n = read_i16(this + 0x34) as i32;
        let p = exe::crt_malloc((n * 0x13) as u32);
        write_usize(this + 0x18, p);
        read(p, 0x13, n as u32);
    }
    if l.club_count() == 0 {
        return 1;
    }
    covered!("rx09_21 league stage rows loaded");
    let clubs = || l.club_count() as i32;
    let row = |k: i32| l.table() + league_table_row::SIZE * k as usize;
    if read(l.table(), league_table_row::SIZE as u32, clubs() as u32) as i32 != clubs() {
        fatal_error_at(LEAGUE_STAGE_SOURCE, 0x1cc);
    }
    let mut has_h2h = 0u32;
    read_or_die(&mut has_h2h as *mut u32 as usize, 4, 0x1d4);
    let mut k = 0;
    while k < clubs() {
        if has_h2h == 0 {
            write_usize(row(k) + league_table_row::H2H_ENTRIES, 0);
        } else {
            let a = exe::crt_malloc((clubs() * 0xe - 0xe) as u32);
            write_usize(row(k) + league_table_row::H2H_ENTRIES, a);
            if a == 0 {
                show_error_at(LEAGUE_STAGE_SOURCE, 0x1e2);
                return 0;
            }
            if read(a, 0xe, (clubs() - 1) as u32) as i32 != clubs() - 1 {
                show_error_at(LEAGUE_STAGE_SOURCE, 0x1eb);
                return 0;
            }
        }
        k += 1;
    }
    let mut has_history = 0u32;
    if read(&mut has_history as *mut u32 as usize, 4, 1) != 1 {
        show_error_at(LEAGUE_STAGE_SOURCE, 0x1f7);
        return 0;
    }
    if l.games_per_club() == -1 {
        l.set_games_per_club(l.club_count().wrapping_sub(1).wrapping_mul(l.legs())); // rx09_21: the games per club worked out
    }
    let mut k = 0;
    while k < clubs() {
        if has_history == 0 {
            write_usize(row(k) + league_table_row::POSITION_HISTORY, 0);
        } else {
            let a = exe::crt_malloc(l.games_per_club() as i32 as u32);
            write_usize(row(k) + league_table_row::POSITION_HISTORY, a);
            if a == 0 {
                show_error_at(LEAGUE_STAGE_SOURCE, 0x20c);
                return 0;
            }
            if read(a, l.games_per_club() as i32 as u32, 1) != 1 {
                show_error_at(LEAGUE_STAGE_SOURCE, 0x215);
                return 0;
            }
        }
        k += 1;
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
    1
}

/// Comparer for `league_stage_load_from_file`: it reads the file and allocates, so only the original runs (the copy is
/// proven by the save proof, loading with and without it); the result must be 1 and the competition one of the
/// database's (or none).
pub(crate) unsafe extern "thiscall" fn compare_league_stage_load_from_file(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file);
    let comp = read_usize(this + league::COMPETITION);
    let base = read_usize(G_CLUB_COMP_TABLE);
    let comp_ok = comp == 0 || (comp >= base && (comp - base) % 0x6b == 0);
    SCOREBOARD.record(r == 1 && comp_ok, || format!("league_stage_load_from_file({:#x}) returned {} competition {:#x}", this, r, comp));
    r
}
