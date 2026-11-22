// ---------- batch %%LEVA%%: loading a cup's temporary file: Cup::load_from_disk (Cup.cpp) ----------

/// The club pointer of a 6-byte cup team entry (`p`, unaligned) rebuilt from the club id stored in the file: −1 gives
/// null, otherwise [`G_CLUB_TABLE`] + `club::SIZE` × id (the table read again each time, as the original).
unsafe fn cup_load_club_from_id(p: usize) {
    let id = read_i32(p);
    write_usize(p, if id == -1 { 0 } else { read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE)) });
}

/// Destroys a competition's statistics object: `Comp_Stats::~Comp_Stats` (0x49f450, thiscall, ret 0) and the
/// operator delete (`exe::crt_operator_delete`, cdecl).
unsafe fn cup_load_delete_comp_stats(s: usize) {
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x49f450usize); // Comp_Stats::~Comp_Stats
    destroy(s);
    exe::crt_operator_delete(s);
}

/// Rule: a cup's saved state comes back from its CM3_TEMP file in the order `Cup::save_cup_to_disk` wrote it
/// (docs/MAPA_SAVE.en.md §6): the header fields, the points-average table, the competition, the rounds, the team table
/// with the clubs turned from ids into pointers, the ties of each round, the stages and the entrant book.
///
/// `Cup::load_from_disk(by_id)`. The file name, written at +0x53: the competition's name (`competition::NAME` of the
/// record +4) with '/' and '.' made '_', then "%s_%d.tmp" with the record's id (+0 of the record) when `by_id` (its low
/// byte) is set, or with the cup's `cup::YEAR` otherwise; '/' in the result made '_'. Opened from CM3_TEMP ("rb");
/// without it (a new game), 0. Otherwise, with fread (`exe::crt_fread`, cdecl): the 0x50-byte name again (short: the
/// error box at line 0x12c7, returning 0 with the file left open); the statistics flag (4 bytes): with it 0, the old
/// statistics object (+8), when there, is destroyed but the pointer is left as it was (a defect of the original: +8
/// dangles); with it set, the old one destroyed and a new one made (operator new 0x5ce, `exe::new_comp_stats`(object,
/// this); null when the allocation fails) into +8 and read (`Comp_Stats::load` 0x4ab890, thiscall, ret 4, called even
/// on null). The class's own data through the virtual +0x20 (thiscall, ret 4). `cup::PRIZE_FUND`, +0x52, the
/// importance bias +0x51 and +0x4b (unchecked). Below data version (`game_data_version`) 0x3e013, one byte as the
/// season-closed flag (`cup::SEASON_CLOSED`, sign-extended; short: the box at line 0x12f7) and three bytes skipped;
/// from it on, `cup::SEASON_CLOSED` as 4 bytes (the box at line 0x1305). `cup::VENUE_STADIUM_ID` (unchecked); the
/// substitutes named and the substitutions allowed (short: the box at lines 0x130e and 0x1314, after which the game
/// stops: `fatal_error_at`). `cup::SEASON_SET_UP` as `cup::SEASON_CLOSED` (the boxes at lines 0x1327 and 0x1335,
/// returning 0). Then, each short stopping the game: `cup::RUN_BY_STAGES` (0x133d), `cup::ROUND_COUNT` (0x1343),
/// `cup::TEAM_COUNT` (0x1349), `cup::YEAR` (0x134f), `cup::CURRENT_ROUND` (0x1355), `cup::NEXT_DRAW_ROUND` (0x135b;
/// here only a 0 return counts as short), `cup::CLUB_SIDES` (0x1361), `cup::STAGE_LABEL` (0x1368), the divisions above
/// +0x1c (0x136f) and below +0x20 (0x1376). The points-average table pointer +0x18 (stored in the file as the memory
/// value) and its row count +0x34 (2 bytes): with both nonzero, a new table of 0x13 bytes per row (out of memory: the
/// "Memory Allocation Error" box at line 0x1385, the game stops) read in (unchecked). The competition id (short: the
/// memory box at line 0x138f): +4 = null for −1, otherwise [`G_CLUB_COMP_TABLE`] + 0x6b × id. The round table
/// `cup::ROUND_TABLE`, 0x68 bytes per round (memory box at line 0x139f; short read: the box at line 0x13a6, the game
/// stops). The team table `cup::TEAM_TABLE`, 6 bytes per team (the boxes at lines 0x13ae and 0x13b6, the game stops),
/// each club rebuilt from its id (`cup_load_club_from_id`). The tie tables `cup::TIE_TABLES`: one pointer per round
/// (unchecked), then for each round an array of 0x35 bytes per tie (`cup_round::TIES` of the round; null: the box at
/// line 0x13cf, the game stops) and, round by round, its ties read as one block (not exactly one block read, which
/// includes a round without ties: the box at line 0x13d9, the game stops). `cup::STAGE_COUNT`, `cup::CURRENT_STAGE`,
/// `cup::STAGE_INDEX` (unchecked) and the stages (`cup_load_stages`). The entrant book: capacity
/// `cup::ENTRANT_CAPACITY` and count `cup::ENTRANT_COUNT` (2 bytes each, unchecked); with a count, a new book of 6
/// bytes per place of the capacity (memory box at line 0x1431, the game stops), the capacity's places read and the
/// clubs of the first count places rebuilt from their ids. The file closed (`exe::crt_fclose`) and 1. The old round,
/// team, tie and book arrays are not freed (as the original).
/// Original: `Cup::load_from_disk` (0x51fc00, Cup.cpp, thiscall, ret 4).
unsafe extern "thiscall" fn cup_load_from_disk(this: usize, by_id: u32) -> u32 {
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
    let number = if by_id as u8 != 0 { read_i32(read_usize(this + league::COMPETITION)) } else { read_i16_as_i32(this + cup::YEAR) };
    format(file_name, 0x99b838, n, number); // "%s_%d.tmp"
    let mut p = file_name;
    while read_u8(p) != 0 {
        if read_u8(p) == b'/' {
            write_u8(p, b'_');
        }
        p += 1;
    }
    let f = temp_file_open_for_read(file_name);
    if f == 0 {
        covered!("gra_28 cup without the file");
        return 0;
    }
    let read = |a: usize, size: u32| exe::crt_fread(a, size, 1, f);
    if read(file_name, 0x50) != 1 {
        show_error_at(CUP_SOURCE, 0x12c7);
        return 0;
    }
    let mut has_stats = 0u32;
    read(&mut has_stats as *mut u32 as usize, 4);
    let old = read_usize(this + league::COMP_STATS);
    if has_stats == 0 {
        if old != 0 {
            cup_load_delete_comp_stats(old);
        }
    } else {
        covered!("gra_28 cup statistics");
        if old != 0 {
            cup_load_delete_comp_stats(old);
        }
        let o = exe::crt_operator_new(0x5ce);
        let s = if o != 0 { exe::new_comp_stats(o, this) } else { 0 };
        write_usize(this + league::COMP_STATS, s);
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4ab890usize); // Comp_Stats::load
        load(s, f);
    }
    let class_data: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(this) + 0x20));
    class_data(this, f);
    read(this + cup::PRIZE_FUND, 4);
    read(this + 0x52, 1);
    read(this + cup::IMPORTANCE_BIAS, 1);
    read(this + 0x4b, 1);
    let mut byte = 0u8;
    let b = &mut byte as *mut u8 as usize;
    if game_data_version() < 0x3e013 {
        if read(b, 1) != 1 {
            show_error_at(CUP_SOURCE, 0x12f7);
            return 0;
        }
        write_i32(this + cup::SEASON_CLOSED, byte as i8 as i32);
        read(b, 1);
        read(b, 1);
        read(b, 1);
    } else if read(this + cup::SEASON_CLOSED, 4) != 1 {
        show_error_at(CUP_SOURCE, 0x1305);
        return 0;
    }
    read(this + cup::VENUE_STADIUM_ID, 4);
    if read(this + cup::SUBSTITUTES_NAMED, 1) != 1 {
        fatal_error_at(CUP_SOURCE, 0x130e);
    }
    if read(this + cup::SUBSTITUTIONS_ALLOWED, 1) != 1 {
        fatal_error_at(CUP_SOURCE, 0x1314);
    }
    if game_data_version() < 0x3e013 {
        if read(b, 1) != 1 {
            show_error_at(CUP_SOURCE, 0x1327);
            return 0;
        }
        write_i32(this + cup::SEASON_SET_UP, byte as i8 as i32);
        read(b, 1);
        read(b, 1);
        read(b, 1);
    } else if read(this + cup::SEASON_SET_UP, 4) != 1 {
        show_error_at(CUP_SOURCE, 0x1335);
        return 0;
    }
    for (a, size, line) in [(cup::RUN_BY_STAGES, 1, 0x133d), (cup::ROUND_COUNT, 2, 0x1343), (cup::TEAM_COUNT, 2, 0x1349), (cup::YEAR, 2, 0x134f), (cup::CURRENT_ROUND, 1, 0x1355)] {
        if read(this + a, size) != 1 {
            fatal_error_at(CUP_SOURCE, line);
        }
    }
    if read(this + cup::NEXT_DRAW_ROUND, 1) == 0 {
        fatal_error_at(CUP_SOURCE, 0x135b);
    }
    for (a, size, line) in [(cup::CLUB_SIDES, 1, 0x1361), (cup::STAGE_LABEL, 2, 0x1368), (0x1c, 4, 0x136f), (0x20, 4, 0x1376)] {
        if read(this + a, size) != 1 {
            fatal_error_at(CUP_SOURCE, line);
        }
    }
    // the points-average table: +0x18 (the pointer as saved) and +0x34 (its rows)
    read(this + 0x18, 4);
    read(this + 0x34, 2);
    if read_usize(this + 0x18) != 0 && read_i16(this + 0x34) != 0 {
        let rows = read_i16_as_i32(this + 0x34);
        let t = exe::crt_malloc((rows as u32).wrapping_mul(0x13));
        write_usize(this + 0x18, t);
        if t == 0 {
            fatal_memory_error_box(0x1385, 0x9b58a8);
        }
        exe::crt_fread(t, 0x13, read_i16_as_i32(this + 0x34) as u32, f);
    }
    let mut comp_id = 0i32;
    if read(&mut comp_id as *mut i32 as usize, 4) != 1 {
        fatal_memory_error_box(0x138f, 0x9b5874);
    }
    write_usize(this + league::COMPETITION, if comp_id == -1 { 0 } else { read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b)) }); // gra28: the competition record
    let rounds = exe::crt_malloc((read_i16_as_i32(this + cup::ROUND_COUNT) as u32).wrapping_mul(cup_round::SIZE as u32));
    write_usize(this + cup::ROUND_TABLE, rounds);
    if rounds == 0 {
        fatal_memory_error_box(0x139f, 0x9b5834);
    }
    if exe::crt_fread(rounds, cup_round::SIZE as u32, read_i16_as_i32(this + cup::ROUND_COUNT) as u32, f) as i32 != read_i16_as_i32(this + cup::ROUND_COUNT) {
        fatal_error_at(CUP_SOURCE, 0x13a6);
    }
    let teams = exe::crt_malloc((read_i16_as_i32(this + cup::TEAM_COUNT) as u32).wrapping_mul(6));
    write_usize(this + cup::TEAM_TABLE, teams);
    if teams == 0 {
        fatal_error_at(CUP_SOURCE, 0x13ae);
    }
    if exe::crt_fread(teams, 6, read_i16_as_i32(this + cup::TEAM_COUNT) as u32, f) as i32 != read_i16_as_i32(this + cup::TEAM_COUNT) {
        fatal_error_at(CUP_SOURCE, 0x13b6);
    }
    let mut i = 0;
    while i < read_i16_as_i32(this + cup::TEAM_COUNT) {
        cup_load_club_from_id(read_usize(this + cup::TEAM_TABLE) + 6 * i as usize); // gra28: the club of a cup team
        i += 1;
    }
    write_usize(this + cup::TIE_TABLES, exe::crt_malloc(((read_i16_as_i32(this + cup::ROUND_COUNT)) << 2) as u32));
    let ties_of = |r: i32| read_i16_as_i32(read_usize(this + cup::ROUND_TABLE) + cup_round::SIZE * r as usize + cup_round::TIES);
    let mut r = 0;
    while r < read_i16_as_i32(this + cup::ROUND_COUNT) {
        let a = exe::crt_malloc((ties_of(r) as u32).wrapping_mul(0x35));
        write_usize(read_usize(this + cup::TIE_TABLES) + 4 * r as usize, a);
        if read_usize(read_usize(this + cup::TIE_TABLES) + 4 * r as usize) == 0 {
            fatal_error_at(CUP_SOURCE, 0x13cf);
        }
        r += 1;
    }
    let mut r = 0;
    while r < read_i16_as_i32(this + cup::ROUND_COUNT) {
        if exe::crt_fread(read_usize(read_usize(this + cup::TIE_TABLES) + 4 * r as usize), (ties_of(r) as u32).wrapping_mul(0x35), 1, f) != 1 {
            fatal_error_at(CUP_SOURCE, 0x13d9);
        }
        r += 1;
    }
    read(this + cup::STAGE_COUNT, 4);
    read(this + cup::CURRENT_STAGE, 4);
    read(this + cup::STAGE_INDEX, 1);
    cup_load_stages(this, f);
    read(this + cup::ENTRANT_CAPACITY, 2);
    read(this + cup::ENTRANT_COUNT, 2);
    if read_i16(this + cup::ENTRANT_COUNT) != 0 {
        covered!("gra_28 cup entrant book");
        let book = exe::crt_malloc((read_i16_as_i32(this + cup::ENTRANT_CAPACITY) as u32).wrapping_mul(6));
        write_usize(this + cup::ENTRANT_BOOK, book);
        if book == 0 {
            fatal_memory_error_box(0x1431, 0x9b57c0);
        }
        exe::crt_fread(book, 6, read_i16_as_i32(this + cup::ENTRANT_CAPACITY) as u32, f);
        let mut i = 0;
        while i < read_i16_as_i32(this + cup::ENTRANT_COUNT) {
            cup_load_club_from_id(read_usize(this + cup::ENTRANT_BOOK) + 6 * i as usize); // gra28: the club of an entrant book place
            i += 1;
        }
    }
    exe::crt_fclose(f);
    1
}

/// Cup.cpp's source path (0x9b55ac "E:\dev\CM3\cm3 00-01\cm3\code\comp\Cup.cpp"), for the error boxes.
const CUP_SOURCE: usize = 0x9b55ac;

/// The stages part of `cup_load_from_disk`: without a stage array (`cup::STAGES` null) and with a positive
/// `cup::STAGE_COUNT`, a new array of 4 bytes per stage (out of memory: the "Memory Allocation Error" box at line
/// 0x13e9, the game stops); still without an array, nothing. An existing array is reused whatever its size. Then, with
/// `cup::CURRENT_STAGE` not negative, for each stage k from 0 to `cup::CURRENT_STAGE` (read again after each one): its
/// type (1 byte). Type 1, a cup stage: its team count and round count (2 bytes each), a list of the round count's tie
/// counts (2 bytes each, read one by one into a new list; out of memory: the memory box at line 0x1405, the game
/// stops), the stage made (operator new 0xb2, `Cup_Stage::Cup_Stage`(team count, round count, list) 0x5230e0,
/// thiscall, ret 0xc; null when the allocation fails) into slot k, the list freed (`exe::crt_free`) and the stage read
/// (`Cup_Stage::load_from_file` 0x523380, thiscall, ret 4). Type 2, a league stage: its team count and entry count (2
/// bytes each), the stage made (operator new 0xee, `League_Stage::League_Stage`(team count, entry count) 0x68b4f0,
/// thiscall, ret 8) into slot k and read (`League_Stage::load_from_file` 0x68b720, thiscall, ret 4). Another type:
/// nothing read for it. The type and the counts keep their last values when a read falls short (as the original's
/// locals).
unsafe fn cup_load_stages(this: usize, f: usize) {
    if read_usize(this + cup::STAGES) == 0 {
        let n = read_i32(this + cup::STAGE_COUNT);
        if n > 0 {
            let a = exe::crt_malloc((n as u32).wrapping_mul(4));
            write_usize(this + cup::STAGES, a);
            if a == 0 {
                fatal_memory_error_box(0x13e9, 0x9b57fc);
            }
        }
        if read_usize(this + cup::STAGES) == 0 {
            return;
        }
    }
    if read_i32(this + cup::CURRENT_STAGE) < 0 {
        return;
    }
    let (mut kind, mut first, mut second) = (0u8, 0u16, 0u16);
    let mut k = 0i32;
    loop {
        exe::crt_fread(&mut kind as *mut u8 as usize, 1, 1, f);
        let slot = |k: i32| read_usize(this + cup::STAGES) + 4 * k as usize;
        if kind == 1 {
            covered!("gra_28 cup stage of a cup");
            exe::crt_fread(&mut first as *mut u16 as usize, 2, 1, f);
            exe::crt_fread(&mut second as *mut u16 as usize, 2, 1, f);
            let list = exe::crt_malloc(((second as i16 as i32) << 1) as u32);
            if list == 0 {
                fatal_memory_error_box(0x1405, 0x9b5784);
            }
            let mut i = 0;
            while i < second as i16 as i32 {
                exe::crt_fread(list + 2 * i as usize, 2, 1, f);
                i += 1;
            }
            let o = exe::crt_operator_new(0xb2);
            let make: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> usize = core::mem::transmute(0x5230e0usize); // Cup_Stage::Cup_Stage
            let s = if o != 0 { make(o, first as u32, second as u32, list) } else { 0 }; // gra28: the cup stage's team and round counts
            write_usize(slot(k), s);
            exe::crt_free(list);
            let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x523380usize); // Cup_Stage::load_from_file
            load(read_usize(slot(k)), f);
        } else if kind == 2 {
            covered!("gra_28 cup stage of a league");
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
        if k > read_i32(this + cup::CURRENT_STAGE) {
            break;
        }
    }
}

/// What `cup_load_from_disk` left in the cup, for the comparer: the cup's bytes without the pointers it makes (+8
/// statistics, +0xc stages, +0x14 book, +0x18 points-average table, +0xa3 rounds, +0xa7 teams, +0xad ties), the
/// average table, the rounds, the team table (with the club pointers), each round's ties, the book, whether there are
/// statistics and their three counts (+4, +0x14, +0x21), and each stage's class (vtable) and loaded fields
/// (+0x1c..+0x53).
unsafe fn cup_load_state(this: usize) -> Vec<u8> {
    let bytes = |a: usize, n: usize| (0..n).map(move |k| read_u8(a + k));
    let mut v: Vec<u8> = (4..8).chain(0x10..0x14).chain(0x1c..0xa3).chain(0xb1..0xb2).map(|k| read_u8(this + k)).collect();
    let rows = read_i16_as_i32(this + 0x34);
    if read_usize(this + 0x18) != 0 && rows > 0 {
        v.extend(bytes(read_usize(this + 0x18), 0x13 * rows as usize));
    }
    let rounds = read_i16_as_i32(this + cup::ROUND_COUNT).max(0) as usize;
    if read_usize(this + cup::ROUND_TABLE) != 0 {
        v.extend(bytes(read_usize(this + cup::ROUND_TABLE), cup_round::SIZE * rounds));
        if read_usize(this + cup::TIE_TABLES) != 0 {
            for r in 0..rounds {
                let t = read_usize(read_usize(this + cup::TIE_TABLES) + 4 * r);
                let n = read_i16_as_i32(read_usize(this + cup::ROUND_TABLE) + cup_round::SIZE * r + cup_round::TIES).max(0) as usize;
                if t != 0 {
                    v.extend(bytes(t, 0x35 * n));
                }
            }
        }
    }
    if read_usize(this + cup::TEAM_TABLE) != 0 {
        v.extend(bytes(read_usize(this + cup::TEAM_TABLE), 6 * read_i16_as_i32(this + cup::TEAM_COUNT).max(0) as usize));
    }
    if read_i16(this + cup::ENTRANT_COUNT) != 0 && read_usize(this + cup::ENTRANT_BOOK) != 0 {
        v.extend(bytes(read_usize(this + cup::ENTRANT_BOOK), 6 * read_i16_as_i32(this + cup::ENTRANT_CAPACITY).max(0) as usize));
    }
    let s = read_usize(this + league::COMP_STATS);
    v.push((s != 0) as u8);
    if s != 0 {
        for o in [4usize, 0x14, 0x21] {
            v.extend_from_slice(&read_u32(s + o).to_le_bytes());
        }
    }
    let stages = read_usize(this + cup::STAGES);
    if stages != 0 {
        for k in 0..=read_i32(this + cup::CURRENT_STAGE).min(read_i32(this + cup::STAGE_COUNT) - 1) {
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

/// Comparer for `cup_load_from_disk`, over the same cup: the original first, then the copy, which reads the same
/// file again with the statistics and stage pointers set aside (so that it neither destroys the original's
/// statistics nor writes into its stage array); the state left by each (`cup_load_state`) and the return must be
/// equal. Then the copy's arrays, statistics and stages are freed (the stages by their deleting destructor, vtable
/// slot 0) and the original's pointers put back. Without the file both return 0 and nothing else is compared.
unsafe extern "thiscall" fn compare_cup_load_from_disk(this: usize, by_id: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, by_id);
    if r == 0 {
        let c = cup_load_from_disk(this, by_id);
        SCOREBOARD.record(c == 0, || format!("cup_load_from_disk({:#x}, {}): original 0 copy {}", this, by_id, c));
        return r;
    }
    let original = cup_load_state(this);
    const KEPT: [usize; 7] = [0x8, cup::STAGES, cup::ENTRANT_BOOK, 0x18, cup::ROUND_TABLE, cup::TEAM_TABLE, cup::TIE_TABLES];
    let kept: Vec<usize> = KEPT.iter().map(|&o| read_usize(this + o)).collect();
    write_usize(this + league::COMP_STATS, 0);
    write_usize(this + cup::STAGES, 0);
    let c = cup_load_from_disk(this, by_id);
    let copy = cup_load_state(this);
    // the copy's allocations
    if read_usize(this + 0x18) != 0 && read_i16(this + 0x34) != 0 && read_usize(this + 0x18) != kept[3] {
        exe::crt_free(read_usize(this + 0x18));
    }
    if read_usize(this + cup::TIE_TABLES) != kept[6] {
        for k in 0..read_i16_as_i32(this + cup::ROUND_COUNT).max(0) as usize {
            exe::crt_free(read_usize(read_usize(this + cup::TIE_TABLES) + 4 * k));
        }
        exe::crt_free(read_usize(this + cup::TIE_TABLES));
    }
    for (o, k) in [(cup::ROUND_TABLE, 4), (cup::TEAM_TABLE, 5), (cup::ENTRANT_BOOK, 2)] {
        if read_usize(this + o) != kept[k] {
            exe::crt_free(read_usize(this + o));
        }
    }
    if read_usize(this + league::COMP_STATS) != 0 {
        cup_load_delete_comp_stats(read_usize(this + league::COMP_STATS));
    }
    let stages = read_usize(this + cup::STAGES);
    if stages != 0 {
        if read_i32(this + cup::CURRENT_STAGE) >= 0 {
            for k in 0..=read_i32(this + cup::CURRENT_STAGE) as usize {
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
    SCOREBOARD.record(r == c && copy == original, || format!("cup_load_from_disk({:#x}, {}): original {} copy {}, state equal {}", this, by_id, r, c, copy == original));
    r
}
