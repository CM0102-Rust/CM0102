use super::*;

/// Puts a human manager's score in the calendar year table (0) of the hall of fame.
///
/// `HallOfFame::add_calendar_year_entry(staff, score)`. Without a person, the game's error box (line 0xff) and 0; a
/// person who is not one of the 16 human managers gives 0; otherwise `try_add_entry` with table 0.
/// Original: `HallOfFame::add_calendar_year_entry` (0x5ef270, hall_of_fame.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_calendar_year_entry(this: usize, staff: usize, score: u32) -> u32 {
    if staff == 0 {
        show_error_at(hall_of_fame_consts::HALL_OF_FAME_CPP, 0xff);
        return 0;
    }
    if !is_human_manager_row(staff) {
        return 0;
    }
    covered!("mwg_7 calendar year entry");
    hall_of_fame_try_add(this, 0, staff, score) // mwg7: the calendar year table
}

/// Comparer for `add_calendar_year_entry`.
pub(crate) unsafe extern "thiscall" fn compare_add_calendar_year_entry(this: usize, staff: usize, score: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ADD_CALENDAR_YEAR_ENTRY);
    compare_hall_of_fame_entry(this, staff, "add_calendar_year_entry", || o(this, staff, score), || add_calendar_year_entry(this, staff, score))
}

/// Puts a human manager's score in the hall of fame table of a league's nation.
///
/// `HallOfFame::add_league_entry(staff, nation_id, score)`. Without a person or with a nation id outside the nation
/// table (`G_NATION_COUNT`), the game's error box (line 0x11b) and 0; a person who is not one of the 16 human managers
/// gives 0. The table is the position (1..27) of the nation in `LEAGUE_NATIONS`, −1 for any other nation (which
/// `try_add_entry` refuses with its error box).
/// Original: `HallOfFame::add_league_entry` (0x5ef330, hall_of_fame.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_league_entry(this: usize, staff: usize, nation_id: i32, score: u32) -> u32 {
    if staff == 0 || nation_id < 0 || nation_id > read_i32(G_NATION_COUNT) - 1 {
        show_error_at(hall_of_fame_consts::HALL_OF_FAME_CPP, 0x11b);
        return 0;
    }
    if !is_human_manager_row(staff) {
        return 0;
    }
    covered!("mwg_7 league entry");
    let table = hall_of_fame_consts::LEAGUE_NATIONS
        .iter()
        .position(|&k| nation_id == read_i32(k))
        .map_or(0xffff_ffff, |i| i as u32 + 1); // mwg7: the league's table
    hall_of_fame_try_add(this, table, staff, score)
}

/// Comparer for `add_league_entry`.
pub(crate) unsafe extern "thiscall" fn compare_add_league_entry(this: usize, staff: usize, nation_id: i32, score: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(IDX_ADD_LEAGUE_ENTRY);
    compare_hall_of_fame_entry(this, staff, "add_league_entry", || o(this, staff, nation_id, score), || add_league_entry(this, staff, nation_id, score))
}

/// Puts a manager's score in one of the 28 hall of fame tables when it beats the lowest entry, and keeps the table
/// sorted.
///
/// `HallOfFame::try_add_entry(table, staff, score)` (the table is read as a byte). A table outside 0..27 or no person
/// shows the game's error box (line 0x13b) and gives 0. A score of 0, or not above the last (100th) entry's, gives 0.
/// The new entry: the manager's name (`exe::database_format_staff_name`, style 1, 50 characters), "club/nation" from
/// his club (`.club_job()`) and national team (`.national_job()`) names (`exe::club_get_national_team_name`), the one
/// he has, or "No Club/Nation"; the time stamp (time(0)), the score, the game's random seed (`G_GAME_RANDOM_SEED`) and
/// today. An entry among the first 99 with the same name, seed and date is a repeat: 0. Otherwise the new entry
/// replaces the last one, the table is sorted again (qsort with `hall_of_fame_compare_entries`: score high to low,
/// then older first) and 1 is returned.
/// Original: `HallOfFame::try_add_entry` (0x5ef5e0, hall_of_fame.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn try_add_entry(this: usize, table: u32, staff: usize, score: u32) -> u32 {
    use hall_of_fame_consts as hf;
    let mut today = [0u8; 8];
    exe::date_date_54a110(today.as_mut_ptr() as usize);
    let t = table as u8 as i8;
    if !(0..=0x1b).contains(&t) || staff == 0 {
        show_error_at(hf::HALL_OF_FAME_CPP, 0x13b);
        return 0;
    }
    let base = this + t as usize * hf::TABLE_SIZE;
    let last = base + (hf::ENTRIES - 1) * hf::ENTRY_SIZE;
    if score == 0 || score <= read_u32(last + hf::SCORE) {
        return 0;
    }
    covered!("mwg_7 entry beats the last one");
    let strncpy: unsafe extern "C" fn(*mut u8, *const u8, u32) -> usize = core::mem::transmute(hf::STRNCPY);
    let team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(hf::TIME_FN);
    let mut text = [0u8; 200];
    let mut name = [0u8; hf::NAME_LEN];
    let mut club_text = [0u8; hf::NAME_LEN];
    exe::database_format_staff_name(staff, text.as_mut_ptr() as usize, 200, 1);
    strncpy(name.as_mut_ptr(), text.as_ptr(), hf::NAME_LEN as u32);
    name[hf::NAME_LEN - 1] = 0;
    let person = Staff(staff);
    let mut club_name = [0u8; 200];
    let mut nation_name = [0u8; 200];
    if !person.club_job().is_null() {
        exe::club_get_national_team_name(person.club_job(), 0usize, club_name.as_mut_ptr() as usize, 200, 1, 1);
    }
    if !person.national_job().is_null() {
        exe::club_get_national_team_name(person.national_job(), 0usize, nation_name.as_mut_ptr() as usize, 200, 1, 1);
    }
    let (c, n) = (club_name.as_ptr() as usize, nation_name.as_ptr() as usize);
    let out = text.as_mut_ptr() as usize;
    match (club_name[0] != 0, nation_name[0] != 0) {
        (true, true) => {
            let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            format(out, hf::CLUB_AND_NATION, c, n);
        }
        (true, false) => {
            strncpy(text.as_mut_ptr(), club_name.as_ptr(), 200);
        }
        (false, true) => {
            strncpy(text.as_mut_ptr(), nation_name.as_ptr(), 200);
        }
        (false, false) => {
            let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            translate(out, hf::NO_CLUB_OR_NATION);
        }
    }
    strncpy(club_text.as_mut_ptr(), text.as_ptr(), hf::NAME_LEN as u32);
    club_text[hf::NAME_LEN - 1] = 0;
    let stamp = team(0);
    let seed = read_u32(G_GAME_RANDOM_SEED);
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(today.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, G_CURRENT_DATE);
    let same_name = |e: usize| {
        let mut k = 0;
        loop {
            let (a, b) = (read_u8(e + k), name[k]);
            if a != b {
                return false;
            }
            if a == 0 {
                return true;
            }
            k += 1;
        }
    };
    for i in 0..hf::ENTRIES - 1 {
        let e = base + i * hf::ENTRY_SIZE;
        if same_name(e) && read_u32(e + hf::SEED) == seed && read_u16(e + hf::DATE + 2) == u16::from_le_bytes([today[2], today[3]])
            && read_u16(e + hf::DATE) == u16::from_le_bytes([today[0], today[1]]) {
            covered!("mwg_7 repeated entry");
            return 0;
        }
    }
    core::ptr::copy_nonoverlapping(name.as_ptr(), last as *mut u8, hf::NAME_LEN);
    core::ptr::copy_nonoverlapping(club_text.as_ptr(), (last + hf::CLUB) as *mut u8, hf::NAME_LEN);
    write_u32(last + hf::TIME, stamp);
    write_u32(last + hf::SCORE, score); // mwg7: the entry's score
    write_u32(last + hf::SEED, seed);
    exe::date_operator_3d(last + hf::DATE, tmp.as_mut_ptr() as usize, today.as_ptr() as usize);
    exe::crt_qsort(base, hf::ENTRIES as u32, hf::ENTRY_SIZE as u32, hf::COMPARE_ENTRIES);
    1
}

/// Comparer for `try_add_entry`: side by side over the 28 tables.
pub(crate) unsafe extern "thiscall" fn compare_try_add_entry(this: usize, table: u32, staff: usize, score: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(IDX_TRY_ADD_ENTRY);
    let regions = [(this, hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, table, staff, score), || try_add_entry(this, table, staff, score));
    SCOREBOARD.record(equal && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("try_add_entry(table {}, staff {:x}, score {}) original={} rust={}, tables differ from byte {:#x}", table as u8 as i8, staff, score, r, n, k)
    });
    r
}

/// `try_add_entry` through the exe (the detour runs its comparer in comparison mode).
pub(crate) unsafe fn hall_of_fame_try_add(this: usize, table: u32, staff: usize, score: u32) -> u32 {
    exe::halloffame_try_add_entry(this, table, staff, score)
}

/// Whether the person is one of the 16 human managers (the last 16 staff rows).
pub(crate) unsafe fn is_human_manager_row(staff: usize) -> bool {
    read_i32(staff) >= read_i32(G_STAFF_COUNT) - 16
}

/// Comparer of the hall of fame entry routines: side by side over the 28 tables when the person is a human manager
/// (otherwise nothing is written and the return value is compared).
pub(crate) unsafe fn compare_hall_of_fame_entry(this: usize, staff: usize, what: &str, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    use hall_of_fame_consts as hf;
    if staff == 0 || !is_human_manager_row(staff) {
        let r = original();
        let n = copy();
        SCOREBOARD.record(r == n, || format!("{}(staff {:x}) original={} rust={}", what, staff, r, n));
        return r;
    }
    let regions = [(this, hf::TABLES * hf::TABLE_SIZE)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, copy);
    SCOREBOARD.record(equal && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("{}(staff {:x}) original={} rust={}, tables differ from byte {:#x}", what, staff, r, n, k)
    });
    r
}

/// Reads `hall_of_fame.bin` into the 28 hall of fame tables; returns 1, or 0 when the file is missing or bad.
///
/// `HallOfFame::load()`: opens the file ("rb"); missing: 0 with no box. Reads the 5-byte header (failing: the "Error"
/// box, hall_of_fame.cpp line 0x1c0), checks the magic dword 0x3abfb2 (0x1c8) and the version byte 1 (0x1d0); these
/// close the file and return 0. Then the tables (`hall_of_fame_consts`, 28 × 12000 = 0x52080 bytes, into `this`; 0x1d8),
/// the checksum dword (0x1e1) and the unscrambling with its check (`hall_of_fame_unscramble_and_check`; 0x1ea): any of
/// these failing shows the box, clears the whole table, closes the file and returns 0. Otherwise the file is closed and
/// it returns 1. Called once by the constructor `HallOfFame::HallOfFame` (0x5ef0b0), which writes a new file when it
/// fails.
/// Original: `HallOfFame::load` (0x5efaa0, hall_of_fame.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_load(this: usize) -> u32 {
    use hall_of_fame_consts as hf;
    const BYTES: u32 = (hf::TABLES * hf::TABLE_SIZE) as u32;
    let file = exe::crt_fopen(0x9ce450usize, 0x98f928usize); // "hall_of_fame.bin", "rb"
    if file == 0 {
        return 0;
    }
    let failed = |row: u32, clear: bool| {
        show_error_at(hf::HALL_OF_FAME_CPP, row);
        if clear {
            core::ptr::write_bytes(this as *mut u8, 0, BYTES as usize);
        }
        exe::crt_fclose(file);
        0
    };
    let mut header = [0u8; 5];
    if exe::crt_fread(header.as_mut_ptr() as usize, 5, 1, file) != 1 {
        return failed(0x1c0, false);
    }
    if read_u32(header.as_ptr() as usize) != 0x3abfb2 {
        return failed(0x1c8, false); // not a hall of fame file
    }
    if header[4] != 1 {
        return failed(0x1d0, false); // another version
    }
    if exe::crt_fread(this, BYTES, 1, file) != 1 {
        return failed(0x1d8, true);
    }
    let mut checksum = 0u32;
    if exe::crt_fread(&mut checksum as *mut u32 as usize, 4, 1, file) != 1 {
        return failed(0x1e1, true);
    }
    let check: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_UNSCRAMBLE_AND_CHECK);
    if check(this, BYTES as i32, checksum) == 0 {
        return failed(0x1ea, true); // sx12_7: a bad checksum
    }
    covered!("sx12_7 hall of fame loaded");
    exe::crt_fclose(file);
    1
}

/// Unscrambles a hall of fame buffer in place and tells whether its byte sum matches the checksum.
///
/// `hall_of_fame_unscramble_and_check(buffer, length, checksum)`: a null buffer or a negative length shows the "Error"
/// box (hall_of_fame.cpp line 0x39f) and returns 0. From the last byte down (i = length − 1 .. 0), it swaps the bytes
/// at key[i mod 480] mod length and key[(i + 1) mod 480] mod length (the key dwords at 0x9cdc88, signed remainders);
/// then XORs each byte i with the low byte of key[i mod 480] and adds the result to a sum. Returns 1 when the sum
/// equals the checksum, else 0.
/// Original: `hall_of_fame_unscramble_and_check` (0x5f0680, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_unscramble_and_check(buffer: usize, length: i32, checksum: u32) -> u32 {
    if buffer == 0 || length < 0 {
        show_error_at(hall_of_fame_consts::HALL_OF_FAME_CPP, 0x39f);
        return 0;
    }
    let key = |i: u32| read_i32(HALL_OF_FAME_KEY + 4 * (i % 0x1e0) as usize);
    let mut i = length - 1;
    while i >= 0 {
        let a = buffer + (key(i as u32) % length) as usize;
        let b = buffer + (key(i as u32 + 1) % length) as usize;
        let t = read_u8(a);
        write_u8(a, read_u8(b));
        write_u8(b, t);
        i -= 1;
    }
    let mut sum = 0u32;
    for i in 0..length as u32 {
        let v = read_u8(buffer + i as usize) ^ key(i) as u8;
        write_u8(buffer + i as usize, v); // sx12_7: the unscrambled byte
        sum = sum.wrapping_add(v as u32);
    }
    covered!("sx12_7 hall of fame unscrambled");
    (sum == checksum) as u32
}

/// Orders two hall of fame entries for qsort: the higher score (`hall_of_fame_consts::SCORE`, unsigned) first, then
/// the older time stamp (`hall_of_fame_consts::TIME`, signed) first; equal: 0. A null a gives 1 and a null b -1.
/// Original: `hall_of_fame_compare_entries` (0x5f0620, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_compare_entries(a: usize, b: usize) -> i32 {
    use hall_of_fame_consts as hf;
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (sa, sb) = (read_u32(a + hf::SCORE), read_u32(b + hf::SCORE));
    if sa < sb {
        return 1;
    }
    if sa > sb {
        return -1;
    }
    covered!("sx12_7 hall of fame equal scores");
    let (ta, tb) = (read_i32(a + hf::TIME), read_i32(b + hf::TIME));
    if tb < ta {
        1
    } else if ta >= tb {
        0
    } else {
        -1 // sx12_7: the older entry first
    }
}

/// Comparer for `hall_of_fame_load`: the original loads into the table; when it succeeds the copy loads the same file
/// into a buffer of its own and the two tables and returns are compared. A failed load (missing file or the "Error"
/// box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_load(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HALL_OF_FAME_LOAD)(this);
    if r == 0 {
        return r;
    }
    let bytes = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    let mut copy = vec![0u8; bytes];
    let n = hall_of_fame_load(copy.as_mut_ptr() as usize);
    let original = core::slice::from_raw_parts(this as *const u8, bytes);
    let first = original.iter().zip(copy.iter()).position(|(x, y)| x != y);
    SCOREBOARD.record(r == n && first.is_none(), || format!("HallOfFame::load original {} rust {}, first different byte {:?}", r, n, first));
    r
}

/// Comparer for `hall_of_fame_unscramble_and_check`: the buffer and the return, side by side. A null buffer or a
/// negative length (the "Error" box): only the original.
pub(crate) unsafe extern "C" fn compare_hall_of_fame_unscramble_and_check(buffer: usize, length: i32, checksum: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, u32) -> u32>(IDX_HALL_OF_FAME_UNSCRAMBLE_AND_CHECK);
    if buffer == 0 || length < 0 {
        return o(buffer, length, checksum);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(buffer, length as usize)], || o(buffer, length, checksum),
        || hall_of_fame_unscramble_and_check(buffer, length, checksum));
    SCOREBOARD.record(equal && r == n, || format!("hall_of_fame_unscramble_and_check({}) original {} rust {}: {:?}", length, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_compare_entries`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_hall_of_fame_compare_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_HALL_OF_FAME_COMPARE_ENTRIES)(a, b);
    let n = hall_of_fame_compare_entries(a, b);
    SCOREBOARD.record(r == n, || format!("hall_of_fame_compare_entries({:#x}, {:#x}) original {} rust {}", a, b, r, n));
    r
}

/// `hall_of_fame_unscramble_and_check` (cdecl), called through the exe so that the comparison sees it.
pub(crate) const FN_HALL_OF_FAME_UNSCRAMBLE_AND_CHECK: usize = 0x5f0680;

/// The 480 dwords of the hall of fame scrambling key.
pub(crate) const HALL_OF_FAME_KEY: usize = 0x9cdc88;
