// ---------- batch %%LEVA%%: loading a league's temporary file: League::load_from_temp_file (league.cpp) ----------

/// The club pointer at `p` (unaligned: a 0x3b-byte table row, a 0x0e-byte head-to-head entry or a 6-byte entrant book
/// place) rebuilt from the club id stored in the file: −1 gives null, otherwise [`G_CLUB_TABLE`] + `club::SIZE` × id
/// (the table read again each time, as the original).
unsafe fn league_load_club_from_id(p: usize) {
    let id = read_i32(p);
    write_usize(p, if id == -1 { 0 } else { read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE)) });
}

/// league.cpp's source path (0x9f15f4 "E:\dev\CM3\cm3 00-01\cm3\code\comp\league.cpp"), for the error boxes.
const LEAGUE_SOURCE: usize = 0x9f15f4;

/// Rule: a league's saved state comes back from its CM3_TEMP file in the order `League::save_to_temp_file` wrote it
/// (docs/MAPA_SAVE.en.md §6): the header fields, the round dates, the points-average table, the table with its clubs,
/// head-to-head records and position history, the stages and the entrant book.
///
/// `League::load_from_temp_file(by_id)`. The file name, written at +0x53: the competition's name
/// (`competition::NAME` of `league::COMPETITION`) with '/' and '.' made '_', then "%s_%d.tmp" with the record's id
/// (+0 of the record) when `by_id` (its low byte) is set, or "%s_%d.bak" with `league::YEAR` otherwise; '/' in the
/// result made '_'. Opened from CM3_TEMP ("rb"); without it (a new game), 0. Otherwise, with fread (`exe::crt_fread`,
/// cdecl), every short read showing the error box at its line and returning 0 with the file left open (as the
/// original): the 0x50-byte name again (line 0x1131); the statistics flag (4 bytes, unchecked): with it 0,
/// `league::COMP_STATS` = 0 (an old object is not destroyed); with it set, a new object (operator new 0x5ce,
/// `exe::new_comp_stats`(object, this); null when the allocation fails, the old one not destroyed) read by
/// `Comp_Stats::load` (0x4ab890, thiscall, ret 4, called even on null). The class's own data through the virtual +0x20
/// (thiscall, ret 4). Then `league::REQUIRED_CAPACITY` (0x1148), `league::REQUIRED_SEATS` (0x114d),
/// `league::PRIZE_MODE` (0x1152), `league::PRIZE_FUND` (0x1157), `league::COMPLETE` (0x115c),
/// `league::LAST_UPDATE_DATE` (8 bytes, 0x1161), `league::GAMES_PER_CLUB` (0x1167), `league::SQUAD_FLAGS` (0x116d),
/// `league::IMPORTANCE` (0x1173), `league::ENTRANTS_ARE_QUALIFIERS` (0x1179); below data version (`game_data_version`)
/// 0x3e013, one byte as `league::SEASON_CLOSED` (sign-extended; 0x118c) and three bytes skipped, from it on 4 bytes
/// (0x119a); `league::FIXED_STADIUM_ID` (0x11a2), `league::SUBSTITUTES_NAMED` (0x11a8),
/// `league::SUBSTITUTIONS_ALLOWED` (0x11ae), `league::NEXT_DRAW_DAY` (0x11b4), `league::NEXT_DRAW_YEAR_OFFSET`
/// (0x11ba); `league::ACTIVE` as `league::SEASON_CLOSED` (0x11ce, 0x11dc); `league::STOPPED` (0x11e5),
/// `league::LEGS` (0x11eb), `league::CLUB_COUNT` (0x11f1), `league::YEAR` (0x11f7), `league::COMP_TYPE` (0x11fe),
/// `league::CLUB_SIDES` (0x1205), `league::STAGE_ID` (0x120c); the competition id (0x1213): `league::COMPETITION` =
/// null for −1, otherwise [`G_CLUB_COMP_TABLE`] + 0x6b × id; `league::ROUND_COUNT` (0x1222), `league::WIN_POINTS`
/// (0x1229), `league::DRAW_POINTS` (0x1230), `league::MATCH_CATEGORY` (0x1237), `league::TIEBREAKS` (4 bytes,
/// 0x123e), the four band counts from `league::PROMOTED_COUNT` (4 bytes, 0x1245), `league::LEAGUE_FLAGS` (0x124c),
/// `league::ROUND_DRAW_RULES` (0x1253), `league::MATCH_RULES` (0x125a), the divisions above
/// `league::UPPER_DIVISION_ID` (0x1261) and below `league::LOWER_DIVISION_ID` (0x1268), `league::TIEBREAK_MATCHES`
/// (0x126f). The round dates `league::ROUND_DATES`, 0x41 bytes per round (out of memory: line 0x1276; short: 0x127c).
/// The points-average table pointer +0x18 (stored as the memory value) and its row count +0x34 (2 bytes, unchecked):
/// with both nonzero, a new table of 0x13 bytes per row read in (neither the allocation nor the read checked). The table
/// (`league_load_table`). `league::STAGE_COUNT`, `league::CURRENT_STAGE`, `league::STAGE_INDEX` (unchecked) and the
/// stages (`league_load_stages`). The entrant book: the old one (`league::ENTRANT_BOOK`) freed (`exe::crt_free`) and
/// cleared; capacity `league::ENTRANT_CAPACITY` and count `league::ENTRANT_COUNT` (2 bytes each, unchecked); with a
/// count, a new book of 6 bytes per place of the capacity (out of memory: line 0x136f), the capacity's places read
/// (unchecked) and the clubs of the first count places rebuilt from their ids (`league_load_club_from_id`). The file
/// closed (`exe::crt_fclose`) and 1. The old round-date and table arrays are not freed (as the original).
/// Original: `League::load_from_temp_file` (0x687b10, league.cpp, thiscall, ret 4).
unsafe extern "thiscall" fn league_load_from_temp_file(this: usize, by_id: u32) -> u32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut name = [0u8; 0x100];
    let n = name.as_mut_ptr() as usize;
    format(n, 0x991944, read_usize(this + league::COMPETITION) + competition::NAME); // "%s"
    let mut k = 0;
    while name[k] != 0 {
        if name[k] == b'/' || name[k] == b'.' {
            name[k] = b'_';
        }
        k += 1;
    }
    let file_name = this + 0x53;
    if by_id as u8 != 0 {
        format(file_name, 0x99b838, n, read_i32(read_usize(this + league::COMPETITION))); // "%s_%d.tmp"
    } else {
        format(file_name, 0x9f1624, n, read_i16_as_i32(this + league::YEAR)); // "%s_%d.bak"
    }
    let mut p = file_name;
    while read_u8(p) != 0 {
        if read_u8(p) == b'/' {
            write_u8(p, b'_');
        }
        p += 1;
    }
    let f = temp_file_open_for_read(file_name);
    if f == 0 {
        covered!("gra_29 league without the file");
        return 0;
    }
    let read = |a: usize, size: u32| exe::crt_fread(a, size, 1, f);
    if read(file_name, 0x50) != 1 {
        show_error_at(LEAGUE_SOURCE, 0x1131);
        return 0;
    }
    let mut has_stats = 0u32;
    read(&mut has_stats as *mut u32 as usize, 4);
    if has_stats == 0 {
        write_usize(this + league::COMP_STATS, 0);
    } else {
        covered!("gra_29 league statistics");
        let o = exe::crt_operator_new(0x5ce);
        let s = if o != 0 { exe::new_comp_stats(o, this) } else { 0 };
        write_usize(this + league::COMP_STATS, s);
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4ab890usize); // Comp_Stats::load
        load(s, f);
    }
    let class_data: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(this) + 0x20));
    class_data(this, f);
    for (a, size, line) in [
        (league::REQUIRED_CAPACITY, 2, 0x1148),
        (league::REQUIRED_SEATS, 2, 0x114d),
        (league::PRIZE_MODE, 1, 0x1152),
        (league::PRIZE_FUND, 4, 0x1157),
        (league::COMPLETE, 4, 0x115c),
        (league::LAST_UPDATE_DATE, 8, 0x1161),
        (league::GAMES_PER_CLUB, 2, 0x1167),
        (league::SQUAD_FLAGS, 1, 0x116d),
        (league::IMPORTANCE, 1, 0x1173),
        (league::ENTRANTS_ARE_QUALIFIERS, 1, 0x1179),
    ] {
        if read(this + a, size) != 1 {
            show_error_at(LEAGUE_SOURCE, line);
            return 0;
        }
    }
    let mut byte = 0u8;
    let b = &mut byte as *mut u8 as usize;
    if game_data_version() < 0x3e013 {
        if read(b, 1) != 1 {
            show_error_at(LEAGUE_SOURCE, 0x118c);
            return 0;
        }
        write_i32(this + league::SEASON_CLOSED, byte as i8 as i32);
        read(b, 1);
        read(b, 1);
        read(b, 1);
    } else if read(this + league::SEASON_CLOSED, 4) != 1 {
        show_error_at(LEAGUE_SOURCE, 0x119a);
        return 0;
    }
    for (a, size, line) in [
        (league::FIXED_STADIUM_ID, 4, 0x11a2),
        (league::SUBSTITUTES_NAMED, 1, 0x11a8),
        (league::SUBSTITUTIONS_ALLOWED, 1, 0x11ae),
        (league::NEXT_DRAW_DAY, 2, 0x11b4),
        (league::NEXT_DRAW_YEAR_OFFSET, 2, 0x11ba),
    ] {
        if read(this + a, size) != 1 {
            show_error_at(LEAGUE_SOURCE, line);
            return 0;
        }
    }
    if game_data_version() < 0x3e013 {
        if read(b, 1) != 1 {
            show_error_at(LEAGUE_SOURCE, 0x11ce);
            return 0;
        }
        write_i32(this + league::ACTIVE, byte as i8 as i32);
        read(b, 1);
        read(b, 1);
        read(b, 1);
    } else if read(this + league::ACTIVE, 4) != 1 {
        show_error_at(LEAGUE_SOURCE, 0x11dc);
        return 0;
    }
    for (a, size, line) in [
        (league::STOPPED, 1, 0x11e5),
        (league::LEGS, 2, 0x11eb),
        (league::CLUB_COUNT, 2, 0x11f1),
        (league::YEAR, 2, 0x11f7),
        (league::COMP_TYPE, 1, 0x11fe),
        (league::CLUB_SIDES, 1, 0x1205),
        (league::STAGE_ID, 2, 0x120c),
    ] {
        if read(this + a, size) != 1 {
            show_error_at(LEAGUE_SOURCE, line);
            return 0;
        }
    }
    let mut comp_id = 0i32;
    if read(&mut comp_id as *mut i32 as usize, 4) != 1 {
        show_error_at(LEAGUE_SOURCE, 0x1213);
        return 0;
    }
    write_usize(this + league::COMPETITION, if comp_id == -1 { 0 } else { read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b)) }); // gra29: the competition record
    for (a, size, line) in [
        (league::ROUND_COUNT, 2, 0x1222),
        (league::WIN_POINTS, 1, 0x1229),
        (league::DRAW_POINTS, 1, 0x1230),
        (league::MATCH_CATEGORY, 1, 0x1237),
        (league::TIEBREAKS, 4, 0x123e),
        (league::PROMOTED_COUNT, 4, 0x1245),
        (league::LEAGUE_FLAGS, 2, 0x124c),
        (league::ROUND_DRAW_RULES, 2, 0x1253),
        (league::MATCH_RULES, 2, 0x125a),
        (league::UPPER_DIVISION_ID, 4, 0x1261),
        (league::LOWER_DIVISION_ID, 4, 0x1268),
        (league::TIEBREAK_MATCHES, 4, 0x126f),
    ] {
        if read(this + a, size) != 1 {
            show_error_at(LEAGUE_SOURCE, line);
            return 0;
        }
    }
    let dates = exe::crt_malloc((read_i16_as_i32(this + league::ROUND_COUNT) as u32).wrapping_mul(0x41));
    write_usize(this + league::ROUND_DATES, dates);
    if dates == 0 {
        show_error_at(LEAGUE_SOURCE, 0x1276);
        return 0;
    }
    if exe::crt_fread(dates, 0x41, read_i16_as_i32(this + league::ROUND_COUNT) as u32, f) as i32 != read_i16_as_i32(this + league::ROUND_COUNT) {
        show_error_at(LEAGUE_SOURCE, 0x127c);
        return 0;
    }
    // the points-average table: +0x18 (the pointer as saved) and +0x34 (its rows)
    read(this + 0x18, 4);
    read(this + 0x34, 2);
    if read_usize(this + 0x18) != 0 && read_i16(this + 0x34) != 0 {
        let t = exe::crt_malloc((read_i16_as_i32(this + 0x34) as u32).wrapping_mul(0x13));
        write_usize(this + 0x18, t);
        exe::crt_fread(t, 0x13, read_i16_as_i32(this + 0x34) as u32, f);
    }
    if let Some(line) = league_load_table(this, f) {
        show_error_at(LEAGUE_SOURCE, line);
        return 0;
    }
    read(this + league::STAGE_COUNT, 4);
    read(this + league::CURRENT_STAGE, 4);
    read(this + league::STAGE_INDEX, 1);
    if let Some(line) = league_load_stages(this, f) {
        show_error_at(LEAGUE_SOURCE, line);
        return 0;
    }
    if read_usize(this + league::ENTRANT_BOOK) != 0 {
        exe::crt_free(read_usize(this + league::ENTRANT_BOOK));
        write_usize(this + league::ENTRANT_BOOK, 0);
    }
    read(this + league::ENTRANT_CAPACITY, 2);
    read(this + league::ENTRANT_COUNT, 2);
    if read_i16(this + league::ENTRANT_COUNT) != 0 {
        covered!("gra_29 league entrant book");
        let book = exe::crt_malloc((read_i16_as_i32(this + league::ENTRANT_CAPACITY) as u32).wrapping_mul(6));
        write_usize(this + league::ENTRANT_BOOK, book);
        if book == 0 {
            show_error_at(LEAGUE_SOURCE, 0x136f);
            return 0;
        }
        exe::crt_fread(book, 6, read_i16_as_i32(this + league::ENTRANT_CAPACITY) as u32, f);
        let mut i = 0;
        while i < read_i16_as_i32(this + league::ENTRANT_COUNT) {
            league_load_club_from_id(read_usize(this + league::ENTRANT_BOOK) + 6 * i as usize); // gra29: the club of an entrant book place
            i += 1;
        }
    }
    exe::crt_fclose(f);
    1
}

/// The table part of `league_load_from_temp_file`; `Some(line)` is the error box to show (the caller returns 0). With
/// no clubs (`league::CLUB_COUNT` 0) the table pointer `league::TABLE` is cleared and nothing is read. Otherwise a new
/// table of 0x3b bytes per club (out of memory: line 0x1294) read in (short: 0x129b); the head-to-head flag (4 bytes;
/// short: 0x12a3): for each row, without it the row's `league_table_row::H2H_ENTRIES` is cleared, with it a new array
/// of 0x0e bytes per opponent (n − 1 of them; out of memory: 0x12b1) read in (short: 0x12ba); the position-history
/// flag (4 bytes; short: 0x12c6): without it each row's `league_table_row::POSITION_HISTORY` is cleared; with it, a
/// `league::GAMES_PER_CLUB` of −1 becomes (n − 1) × `league::LEGS` (16 bits) when there are clubs, and each row gets a
/// new array of that many bytes (out of memory: 0x12db) read as one block (short: 0x12e4). Last, each row's club and
/// (when the FIRST row has head-to-head entries: the original tests row 0's pointer for every row) its n − 1
/// opponents' clubs are rebuilt from their ids (`league_load_club_from_id`).
unsafe fn league_load_table(this: usize, f: usize) -> Option<u32> {
    let clubs = || read_i16_as_i32(this + league::CLUB_COUNT);
    if read_i16(this + league::CLUB_COUNT) == 0 {
        write_usize(this + league::TABLE, 0);
        return None;
    }
    let t = exe::crt_malloc((clubs() as u32).wrapping_mul(league_table_row::SIZE as u32));
    write_usize(this + league::TABLE, t);
    if t == 0 {
        return Some(0x1294);
    }
    if exe::crt_fread(t, league_table_row::SIZE as u32, clubs() as u32, f) as i32 != clubs() {
        return Some(0x129b);
    }
    let row = |k: i32| read_usize(this + league::TABLE) + league_table_row::SIZE * k as usize;
    let mut has_h2h = 0u32;
    if exe::crt_fread(&mut has_h2h as *mut u32 as usize, 4, 1, f) != 1 {
        return Some(0x12a3);
    }
    let mut k = 0;
    while k < clubs() {
        if has_h2h == 0 {
            write_usize(row(k) + league_table_row::H2H_ENTRIES, 0);
        } else {
            let a = exe::crt_malloc((clubs() as u32).wrapping_mul(0xe).wrapping_sub(0xe));
            write_usize(row(k) + league_table_row::H2H_ENTRIES, a);
            let a = read_usize(row(k) + league_table_row::H2H_ENTRIES);
            if a == 0 {
                return Some(0x12b1);
            }
            if exe::crt_fread(a, 0xe, (clubs() - 1) as u32, f) as i32 != clubs() - 1 {
                return Some(0x12ba);
            }
        }
        k += 1;
    }
    let mut has_history = 0u32;
    if exe::crt_fread(&mut has_history as *mut u32 as usize, 4, 1, f) != 1 {
        return Some(0x12c6);
    }
    if has_history != 0 {
        covered!("gra_29 league position history");
        if read_i16(this + league::GAMES_PER_CLUB) == -1 && read_i16(this + league::CLUB_COUNT) > 0 {
            write_i16(this + league::GAMES_PER_CLUB, (read_i16(this + league::CLUB_COUNT).wrapping_sub(1)).wrapping_mul(read_i16(this + league::LEGS)));
        }
        let mut k = 0;
        while k < clubs() {
            let a = exe::crt_malloc(read_i16_as_i32(this + league::GAMES_PER_CLUB) as u32);
            write_usize(row(k) + league_table_row::POSITION_HISTORY, a);
            let a = read_usize(row(k) + league_table_row::POSITION_HISTORY);
            if a == 0 {
                return Some(0x12db);
            }
            if exe::crt_fread(a, read_i16_as_i32(this + league::GAMES_PER_CLUB) as u32, 1, f) != 1 {
                return Some(0x12e4);
            }
            k += 1;
        }
    } else {
        let mut k = 0;
        while k < clubs() {
            write_usize(row(k) + league_table_row::POSITION_HISTORY, 0);
            k += 1;
        }
    }
    let mut k = 0;
    while k < clubs() {
        league_load_club_from_id(row(k) + league_table_row::CLUB); // gra29: the club of a table row
        if read_usize(row(0) + league_table_row::H2H_ENTRIES) != 0 {
            let mut j = 0;
            while j < clubs() - 1 {
                league_load_club_from_id(read_usize(row(k) + league_table_row::H2H_ENTRIES) + 0xe * j as usize); // gra29: the opponent of a head-to-head entry
                j += 1;
            }
        }
        k += 1;
    }
    None
}

/// The stages part of `league_load_from_temp_file`; `Some(line)` is the error box to show (the caller returns 0).
/// Without a stage array (`league::STAGES` null) and with a positive `league::STAGE_COUNT`, a new array of 4 bytes per
/// stage (out of memory: line 0x1320); still without an array, nothing. An existing array is reused whatever its size.
/// Then, with `league::CURRENT_STAGE` not negative, for each stage k from 0 to `league::CURRENT_STAGE` (read again
/// after each one): its type (1 byte). Type 1, a cup stage: its team count and round count (2 bytes each), a list of
/// the round count's tie counts (2 bytes each, read one by one into a new list; out of memory: line 0x133c), the stage
/// made (operator new 0xb2, `Cup_Stage::Cup_Stage`(team count, round count, list) 0x5230e0, thiscall, ret 0xc; null
/// when the allocation fails) into slot k, the list freed (`exe::crt_free`) and the stage read
/// (`Cup_Stage::load_from_file` 0x523380, thiscall, ret 4). Type 2, a league stage: its team count and entry count (2
/// bytes each), the stage made (operator new 0xee, `League_Stage::League_Stage`(team count, entry count) 0x68b4f0,
/// thiscall, ret 8) into slot k and read (`League_Stage::load_from_file` 0x68b720, thiscall, ret 4). Another type:
/// nothing read for it. The type and the counts keep their last values when a read falls short (as the original's
/// locals).
unsafe fn league_load_stages(this: usize, f: usize) -> Option<u32> {
    if read_usize(this + league::STAGES) == 0 {
        let n = read_i32(this + league::STAGE_COUNT);
        if n > 0 {
            let a = exe::crt_malloc((n as u32).wrapping_mul(4));
            write_usize(this + league::STAGES, a);
            if a == 0 {
                return Some(0x1320);
            }
        }
        if read_usize(this + league::STAGES) == 0 {
            return None;
        }
    }
    if read_i32(this + league::CURRENT_STAGE) < 0 {
        return None;
    }
    let (mut kind, mut first, mut second) = (0u8, 0u16, 0u16);
    let mut k = 0i32;
    loop {
        exe::crt_fread(&mut kind as *mut u8 as usize, 1, 1, f);
        let slot = |k: i32| read_usize(this + league::STAGES) + 4 * k as usize;
        if kind == 1 {
            covered!("gra_29 league stage of a cup");
            exe::crt_fread(&mut first as *mut u16 as usize, 2, 1, f);
            exe::crt_fread(&mut second as *mut u16 as usize, 2, 1, f);
            let list = exe::crt_malloc(((second as i16 as i32) << 1) as u32);
            if list == 0 {
                return Some(0x133c);
            }
            let mut i = 0;
            while i < second as i16 as i32 {
                exe::crt_fread(list + 2 * i as usize, 2, 1, f);
                i += 1;
            }
            let o = exe::crt_operator_new(0xb2);
            let make: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> usize = core::mem::transmute(0x5230e0usize); // Cup_Stage::Cup_Stage
            let s = if o != 0 { make(o, first as u32, second as u32, list) } else { 0 };
            write_usize(slot(k), s);
            exe::crt_free(list);
            let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x523380usize); // Cup_Stage::load_from_file
            load(read_usize(slot(k)), f);
        } else if kind == 2 {
            covered!("gra_29 league stage of a league");
            exe::crt_fread(&mut first as *mut u16 as usize, 2, 1, f);
            exe::crt_fread(&mut second as *mut u16 as usize, 2, 1, f);
            let o = exe::crt_operator_new(0xee);
            let make: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x68b4f0usize); // League_Stage::League_Stage
            let s = if o != 0 { make(o, first as u32, second as u32) } else { 0 };
            write_usize(slot(k), s);
            let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68b720usize); // League_Stage::load_from_file
            load(read_usize(slot(k)), f);
        }
        k += 1;
        if k > read_i32(this + league::CURRENT_STAGE) {
            break;
        }
    }
    None
}

/// What `league_load_from_temp_file` left in the league, for the comparer: the league's base bytes (to +0xee) without
/// the pointers it makes (+8 statistics, +0xc stages, +0x14 book, +0x18 points-average table, +0xb1 table, +0xba round
/// dates), the average table, the round dates, each table row (with the club pointer, without its two array pointers),
/// its head-to-head entries and position history, the book, whether there are statistics and their three counts (+4,
/// +0x14, +0x21), and each stage's class (vtable) and loaded fields (+0x1c..+0x53).
unsafe fn league_load_state(this: usize) -> Vec<u8> {
    let bytes = |a: usize, n: usize| (0..n).map(move |k| read_u8(a + k));
    let mut v: Vec<u8> = (4..8).chain(0x10..0x14).chain(0x1c..0xb1).chain(0xb5..0xba).chain(0xbe..0xee).map(|k| read_u8(this + k)).collect();
    let rows = read_i16_as_i32(this + 0x34);
    if read_usize(this + 0x18) != 0 && rows > 0 {
        v.extend(bytes(read_usize(this + 0x18), 0x13 * rows as usize));
    }
    if read_usize(this + league::ROUND_DATES) != 0 {
        v.extend(bytes(read_usize(this + league::ROUND_DATES), 0x41 * read_i16_as_i32(this + league::ROUND_COUNT).max(0) as usize));
    }
    let clubs = read_i16_as_i32(this + league::CLUB_COUNT).max(0) as usize;
    let t = read_usize(this + league::TABLE);
    if t != 0 {
        for k in 0..clubs {
            let r = t + league_table_row::SIZE * k;
            v.extend(bytes(r, 5).chain(bytes(r + 0xd, league_table_row::SIZE - 0xd)));
            let h = read_usize(r + league_table_row::H2H_ENTRIES);
            if h != 0 {
                v.extend(bytes(h, 0xe * clubs.saturating_sub(1)));
            }
            let p = read_usize(r + league_table_row::POSITION_HISTORY);
            if p != 0 {
                v.extend(bytes(p, read_i16_as_i32(this + league::GAMES_PER_CLUB).max(0) as usize));
            }
        }
    }
    if read_i16(this + league::ENTRANT_COUNT) != 0 && read_usize(this + league::ENTRANT_BOOK) != 0 {
        v.extend(bytes(read_usize(this + league::ENTRANT_BOOK), 6 * read_i16_as_i32(this + league::ENTRANT_CAPACITY).max(0) as usize));
    }
    let s = read_usize(this + league::COMP_STATS);
    v.push((s != 0) as u8);
    if s != 0 {
        for o in [4usize, 0x14, 0x21] {
            v.extend_from_slice(&read_u32(s + o).to_le_bytes());
        }
    }
    let stages = read_usize(this + league::STAGES);
    if stages != 0 {
        for k in 0..=read_i32(this + league::CURRENT_STAGE).min(read_i32(this + league::STAGE_COUNT) - 1) {
            let st = read_usize(stages + 4 * k as usize);
            v.extend_from_slice(&st.min(1).to_le_bytes());
            if st != 0 {
                v.extend_from_slice(&read_u32(st).to_le_bytes());
                v.extend(bytes(st + 0x1c, 0x53 - 0x1c));
            }
        }
    }
    v
}

/// Comparer for `league_load_from_temp_file`, over the same league: the original first, then the copy, which reads
/// the same file again with the statistics, stage and entrant-book pointers set aside (so that it neither frees the
/// original's book nor writes into its stage array); the state left by each (`league_load_state`) and the return must
/// be equal. Then the copy's arrays, statistics (`Comp_Stats::~Comp_Stats` 0x49f450 and the operator delete) and
/// stages (their deleting destructor, vtable slot 0) are freed and the original's pointers put back. Without the file
/// both return 0 and nothing else is compared.
unsafe extern "thiscall" fn compare_league_load_from_temp_file(this: usize, by_id: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, by_id);
    if r == 0 {
        let c = league_load_from_temp_file(this, by_id);
        SCOREBOARD.record(c == 0, || format!("league_load_from_temp_file({:#x}, {}): original 0 copy {}", this, by_id, c));
        return r;
    }
    let original = league_load_state(this);
    const KEPT: [usize; 6] = [league::COMP_STATS, league::STAGES, league::ENTRANT_BOOK, 0x18, league::TABLE, league::ROUND_DATES];
    let kept: Vec<usize> = KEPT.iter().map(|&o| read_usize(this + o)).collect();
    for o in [league::COMP_STATS, league::STAGES, league::ENTRANT_BOOK] {
        write_usize(this + o, 0);
    }
    let c = league_load_from_temp_file(this, by_id);
    let copy = league_load_state(this);
    // the copy's allocations
    if read_usize(this + 0x18) != 0 && read_i16(this + 0x34) != 0 && read_usize(this + 0x18) != kept[3] {
        exe::crt_free(read_usize(this + 0x18));
    }
    let t = read_usize(this + league::TABLE);
    if t != 0 && t != kept[4] {
        for k in 0..read_i16_as_i32(this + league::CLUB_COUNT).max(0) as usize {
            for o in [league_table_row::H2H_ENTRIES, league_table_row::POSITION_HISTORY] {
                let a = read_usize(t + league_table_row::SIZE * k + o);
                if a != 0 {
                    exe::crt_free(a);
                }
            }
        }
        exe::crt_free(t);
    }
    for o in [league::ROUND_DATES, league::ENTRANT_BOOK] {
        let a = read_usize(this + o);
        if a != 0 && !kept.contains(&a) {
            exe::crt_free(a);
        }
    }
    let s = read_usize(this + league::COMP_STATS);
    if s != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x49f450usize); // Comp_Stats::~Comp_Stats
        destroy(s);
        exe::crt_operator_delete(s);
    }
    let stages = read_usize(this + league::STAGES);
    if stages != 0 {
        if read_i32(this + league::CURRENT_STAGE) >= 0 {
            for k in 0..=read_i32(this + league::CURRENT_STAGE) as usize {
                let st = read_usize(stages + 4 * k);
                if st != 0 {
                    let delete: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(st)));
                    delete(st, 1);
                }
            }
        }
        exe::crt_free(stages);
    }
    for (k, &o) in KEPT.iter().enumerate() {
        write_usize(this + o, kept[k]);
    }
    SCOREBOARD.record(r == c && copy == original, || format!("league_load_from_temp_file({:#x}, {}): original {} copy {}, state equal {}", this, by_id, r, c, copy == original));
    r
}
