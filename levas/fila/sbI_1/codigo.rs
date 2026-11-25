// ---------- sbI_1: the UEFA seeding object, the euro.cfg club order and the euro.cfg check ----------

/// Builds the UEFA seeding object, for a new game or from UefaSeeding.tmp.
///
/// - The CD check (`find_game_cd`, the no-CD patch read at 0x9033a2 and 0x9033e4).
/// - `intertoto_volunteers()` 0, `intertoto_volunteer_count()` 0 and the 50 national cup finals (`cup_finals_addr()`,
/// 600 bytes) all −1.
/// - When `uefa_seeding_load_from_temp_file` reads the temporary file, nothing else.
/// - Otherwise `season_year()` 0 and the nation table rebuilt: each nation of `G_NATION_TABLE` (in table order) whose
/// continent is Europe (`KEY_CONTINENT_EUROPE`) gets the next entry (`seeding_nation_entry`): its id, the nation, the
/// coefficient 0, the Champions League, UEFA Cup and Intertoto places 0, the season points 0 and the clubs in the cups
/// 0; `nation_count()` counts them.
///
/// Returns this.
///
/// Defect of the original: the table has room for 60 entries (up to `uefa_seeding::NATION_COUNT`) and the count is
/// a signed byte, with no bound: a database with more European nations writes over the count and what follows.
/// Original: `UefaSeeding::UefaSeeding` (0x903360, uefa_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_uefa_seeding(this: usize) -> usize {
    let seeding = UefaSeeding(this);
    find_game_cd(0x9033a2, 0x9033e4);
    seeding.set_intertoto_volunteers(0);
    seeding.set_intertoto_volunteer_count(0);
    for k in 0..0x96 {
        write_i32(seeding.cup_finals_addr() + 4 * k, -1);
    }
    if uefa_seeding_load_from_temp_file(this) != 0 {
        covered!("sbI_1 seeding from the temporary file");
        return this;
    }
    covered!("sbI_1 new seeding table");
    seeding.set_season_year(0);
    seeding.set_nation_count(0);
    for nation in european_nations() {
        let e = seeding_nation_entry(this, seeding.nation_count() as i32);
        write_u32(e, read_u32(nation.addr())); // +0: the nation id
        write_usize(e + 4, nation.addr()); // +4: the nation
        write_u32(e + 8, 0); // +8: the coefficient (float)
        write_u8(e + uefa_seeding_nation::CHAMPIONS_LEAGUE_PLACES, 0);
        write_u8(e + uefa_seeding_nation::UEFA_CUP_PLACES, 0);
        write_u8(e + uefa_seeding_nation::INTERTOTO_PLACES, 0);
        write_i16(e + uefa_seeding_nation::SEASON_POINTS, 0);
        write_u8(e + uefa_seeding_nation::CLUBS_IN_CUPS, 0);
        seeding.set_nation_count(seeding.nation_count().wrapping_add(1)); // sbI_1: one more European nation
    }
    this
}

/// The nations of `G_NATION_TABLE` whose continent is Europe (`KEY_CONTINENT_EUROPE`), in table order.
pub(crate) unsafe fn european_nations() -> Vec<Nation> {
    (0..read_i32(G_NATION_COUNT).max(0) as usize)
        .map(|i| Nation(read_usize(G_NATION_TABLE) + i * nation::SIZE))
        .filter(|n| n.continent() != 0 && read_u32(n.continent()) == read_u32(KEY_CONTINENT_EUROPE))
        .collect()
}

/// Comparer for `new_uefa_seeding`: only the original runs (it waits for the CD and reads the temporary file). The 50
/// cup finals must be empty when the table is new (`season_year()` 0, which the saved games never have) and the nation
/// table must hold the European nations in order, with everything else 0.
pub(crate) unsafe extern "thiscall" fn compare_new_uefa_seeding(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this);
    let seeding = UefaSeeding(this);
    if seeding.season_year() != 0 {
        SCOREBOARD.record(r == this, || format!("new_uefa_seeding (from the file): returned {:#x}, this {:#x}", r, this));
        return r;
    }
    let plan = european_nations();
    let finals_empty = (0..0x96).all(|k| read_i32(seeding.cup_finals_addr() + 4 * k) == -1);
    let wrong = (0..plan.len()).find(|&k| {
        let e = seeding_nation_entry(this, k as i32);
        read_u32(e) != read_u32(plan[k].addr()) || read_usize(e + 4) != plan[k].addr()
            || core::slice::from_raw_parts((e + 8) as *const u8, 10).iter().any(|&b| b != 0)
    });
    SCOREBOARD.record(r == this && finals_empty && seeding.nation_count() as usize == plan.len() && wrong.is_none()
        && seeding.intertoto_volunteers() == 0 && seeding.intertoto_volunteer_count() == 0,
        || format!("new_uefa_seeding: {} nations (plan {}), cup finals empty {}, first wrong entry {:?}, volunteers {:#x}/{}",
            seeding.nation_count(), plan.len(), finals_empty, wrong, seeding.intertoto_volunteers(), seeding.intertoto_volunteer_count()));
    r
}

pub(crate) const FN_NATION_GET_ENGLISH_NAME_SBI: usize = 0x5453f0; // nation_get_english_name cdecl(nation) -> text

/// The English name of a nation (`nation_get_english_name`).
pub(crate) unsafe fn english_nation_name(nation: usize) -> usize {
    let name: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_NATION_GET_ENGLISH_NAME_SBI);
    name(nation)
}

/// Puts the clubs listed in euro.cfg at the front of their nation's block in the European clubs table.
///
/// - `table` holds `count` (short) 12-byte entries (`european_club_entry`: +0 the club, +4 the nation id), sorted by
/// nation. CM3_DATA\euro.cfg ("r") is read line by line (`exe::utils_read_config_line` into the text buffer 0xde1f64);
/// without the file, nothing.
/// - A header "*<nation>": the first seeding entry (`seeding_nation_entry`, up to `nation_count()`) whose nation's
/// English name is in the line (`exe::crt_strstr`). None, or a null nation: the next line is read and taken as a header.
/// - The nation's first entry in the table: `exe::crt_bsearch` with the key (nation id at +4, 0xff at +8) and the game's
/// comparer 0x905ce0, then back while the entry before has the same nation id. Not found: the next line is read and
/// taken as a header.
/// - Each following line up to the next header: from the cursor (the nation's first entry at the start), the first
/// entry of the nation (the scan stops at an entry of another nation, `club::NATION`) whose club's English name is in
/// the line is swapped with the cursor's entry and the cursor moves on one.
/// - The file is closed when the lines end.
///
/// Defects of the original: a line that is not a header where a header is expected (the first line, or the line after
/// a header whose nation is not found) makes it spin forever on that line; the table's last entry is never looked at.
/// Original: `UefaSeeding::apply_euro_cfg_order` (0x9059e0, uefa_seeding.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn apply_euro_cfg_order(this: usize, table: usize, count: u32) {
    let seeding = UefaSeeding(this);
    let n = count as i16 as i32;
    let last = (table as isize + (n as isize - 1) * 12) as usize;
    let f = exe::crt_fopen(exe::utils_build_file_path(0x9b5348, 0xaa1a64), 0x9899fc); // "CM3_DATA", "euro.cfg", "r"
    if f == 0 {
        return;
    }
    let line = 0xde1f64usize; // the shared text buffer
    let read = || exe::utils_read_config_line(f, line as *mut u8) != 0;
    if !read() {
        exe::crt_fclose(f);
        return;
    }
    'header: loop {
        if read_u8(line) != b'*' {
            covered!("sbI_1 euro.cfg stuck");
            loop {
                core::hint::spin_loop(); // the original tests the same line forever
            }
        }
        let mut found = 0usize;
        let mut k = 0i8;
        while k < seeding.nation_count() {
            let e = seeding_nation_entry(this, k as i32);
            if exe::crt_strstr(line, english_nation_name(read_usize(e + 4))) != 0 {
                found = e;
                break;
            }
            k += 1;
        }
        let nation = if found != 0 { read_usize(found + 4) } else { 0 };
        let mut first = 0usize;
        if nation != 0 {
            let mut key = [0u8; 12];
            key[4..8].copy_from_slice(&read_u32(nation).to_le_bytes()); // the nation id
            key[8] = 0xff;
            first = exe::crt_bsearch(key.as_ptr() as usize, table, n as u32, 12, 0x905ce0usize);
        }
        if first == 0 {
            if !read() {
                break;
            }
            continue 'header;
        }
        covered!("sbI_1 euro.cfg nation");
        let id = read_u32(nation);
        while first > table && read_u32(first - 12 + 4) == id {
            first -= 12;
        }
        let mut cursor = first;
        loop {
            if !read() {
                break 'header;
            }
            if read_u8(line) == b'*' {
                continue 'header;
            }
            let mut q = cursor;
            while q < last {
                if exe::crt_strstr(line, exe::club_get_english_name(read_usize(q))) != 0 {
                    covered!("sbI_1 euro.cfg club moved up");
                    for b in 0..12 {
                        let (x, y) = (read_u8(cursor + b), read_u8(q + b));
                        write_u8(cursor + b, y); // sbI_1: the listed club to the cursor
                        write_u8(q + b, x);
                    }
                    cursor += 12;
                    break;
                }
                q += 12;
                if Club(read_usize(q)).nation().addr() != nation {
                    break;
                }
            }
        }
    }
    exe::crt_fclose(f);
}

/// Comparer for `apply_euro_cfg_order`: both run side by side on the table (they read the same file).
pub(crate) unsafe extern "thiscall" fn compare_apply_euro_cfg_order(this: usize, table: usize, count: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%);
    let n = (count as i16 as i32).max(0) as usize;
    let regions = [(table, n * 12)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, table, count), || apply_euro_cfg_order(this, table, count));
    SCOREBOARD.record(equal, || format!("apply_euro_cfg_order({} entries) original={:x?} rust={:x?}", n, mo, mr));
}

/// Walks euro.cfg as `validate_euro_cfg` does and calls `report` with the format and the line of each box it shows;
/// returns false where the original spins forever. The line buffer is local.
///
/// - CM3_DATA\euro.cfg ("r"; none: nothing), line by line (`exe::utils_read_config_line`).
/// - A header "*<nation>": the first nation of `G_NATION_TABLE` whose English name is in the line (`exe::crt_strstr`);
/// none: the box "Euro config, nation %s not found" (0xaa1a90) and the next line is taken as a header.
/// - The lines up to the next header: the first club of `G_CLUB_TABLE` whose English name is in the line; the box
/// "Euro config, club %s not found" (0xaa1a70) only with no clubs at all.
///
/// Defects of the original: a line that is not a header where a header is expected (the first one, or the line after
/// a nation not found) makes it spin forever on that line; a club not found shows no box (the pointer it tests is
/// the club count × the record size, not 0).
pub(crate) unsafe fn scan_euro_cfg(report: &mut dyn FnMut(usize, usize)) -> bool {
    let f = exe::crt_fopen(exe::utils_build_file_path(0x9b5348, 0xaa1a64), 0x9899fc); // "CM3_DATA", "euro.cfg", "r"
    if f == 0 {
        return true;
    }
    // simplification: a bigger line buffer; the original's 200 bytes would overflow its frame with a longer line
    let mut buffer = [0u8; 0x800];
    let line = buffer.as_mut_ptr() as usize;
    let read = || exe::utils_read_config_line(f, line as *mut u8) != 0;
    let mut more = read();
    let mut ends = true;
    'header: while more {
        if read_u8(line) != b'*' {
            ends = false;
            break;
        }
        let nation = (0..read_i32(G_NATION_COUNT).max(0) as usize)
            .map(|i| read_usize(G_NATION_TABLE) + i * nation::SIZE)
            .find(|&p| exe::crt_strstr(line, english_nation_name(p)) != 0)
            .unwrap_or(0);
        if nation == 0 {
            covered!("sbI_1 euro.cfg unknown nation");
            report(0xaa1a90, line); // "Euro config, nation %s not found"
            more = read();
            continue;
        }
        loop {
            if !read() {
                break 'header;
            }
            if read_u8(line) == b'*' {
                continue 'header;
            }
            let clubs = read_i32(G_CLUB_COUNT);
            let known = (0..clubs.max(0) as isize).any(|i| exe::crt_strstr(line, exe::club_get_english_name(club_at(i))) != 0);
            if !known && clubs <= 0 {
                report(0xaa1a70, line); // "Euro config, club %s not found"
            }
        }
    }
    exe::crt_fclose(f);
    ends
}

/// Checks euro.cfg against the database ("-verifyeurocfg" on the command line, at game creation): for each "*<nation>"
/// header whose nation is not found, the "Error" box "Euro config, nation %s not found" (formatted into 0xde1f64); for
/// the club lines, the box "Euro config, club %s not found" only when there are no clubs (`scan_euro_cfg`).
///
/// Defects of the original: see `scan_euro_cfg` (it spins forever on a line that is not a header where one is expected,
/// and never reports an unknown club).
/// Original: `validate_euro_cfg` (0x905d40, uefa_seeding.cpp [inference: between the UefaSeeding routines], cdecl).
pub(crate) unsafe extern "C" fn validate_euro_cfg() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let ends = scan_euro_cfg(&mut |text, line| {
        format(0xde1f64, text, line);
        exe::gui_show_message_box(0x9870e0, 0xde1f64usize, 0); // "Error"
    });
    if !ends {
        covered!("sbI_1 euro.cfg check stuck");
        loop {
            core::hint::spin_loop(); // the original tests the same line forever
        }
    }
}

/// Comparer for `validate_euro_cfg`: only the original runs (it shows the boxes). Each box formats its text into the
/// text buffer 0xde1f64 first, so its first byte is cleared before and, afterwards, the buffer must still be empty when
/// the copy (without boxes) finds nothing, or hold the copy's last text.
pub(crate) unsafe extern "C" fn compare_validate_euro_cfg() {
    let first = read_u8(0xde1f64);
    write_u8(0xde1f64, 0);
    original_fn::<unsafe extern "C" fn()>(%%I2%%)();
    let shown = c_string_bytes(0xde1f64);
    if shown.is_empty() {
        write_u8(0xde1f64, first);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut last = Vec::new();
    let mut boxes = 0;
    let ends = scan_euro_cfg(&mut |text, line| {
        let mut b = [0u8; 0x900];
        format(b.as_mut_ptr() as usize, text, line);
        last = c_string_bytes(b.as_ptr() as usize);
        boxes += 1;
    });
    SCOREBOARD.record(ends && shown == last, || format!("validate_euro_cfg: original's last box {:?}, the copy's {} boxes, last {:?}, ends {}",
        String::from_utf8_lossy(&shown), boxes, String::from_utf8_lossy(&last), ends));
}
