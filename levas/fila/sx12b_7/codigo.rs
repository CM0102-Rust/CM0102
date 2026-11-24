// ---------- batch %%LEVA%%: the hall of fame screen: entries, import, export and the league names (hall_of_fame.cpp; sx12b_7) ----------

/// `HallOfFameEntry::operator=`, `HallOfFame::merge_entry`, `HallOfFame::import_file`,
/// `hall_of_fame_compare_league_names` and `hall_of_fame_unscramble_and_check`, called through the exe so that their
/// own detours run.
const FN_HALL_OF_FAME_ENTRY_ASSIGN: usize = 0x5ef200;
const FN_HALL_OF_FAME_MERGE_ENTRY: usize = 0x5ef970;
const FN_HALL_OF_FAME_IMPORT_FILE: usize = 0x5efd50;
const FN_HALL_OF_FAME_COMPARE_LEAGUE_NAMES: usize = 0x5f1100;
const FN_HALL_OF_FAME_GET_TABLE_NAME: usize = 0x5f07c0;
/// The league order of the hall of fame menu (28 table indices) and the name form it was sorted by.
const HALL_OF_FAME_LEAGUE_ORDER: usize = 0xb5debc;
const HALL_OF_FAME_LEAGUE_ORDER_FORM: usize = 0xb5deb8;
/// The hall of fame import buffer (2800 entries, 0x52080 bytes).
const HALL_OF_FAME_IMPORT: usize = 0xb0be38;

/// The two names of each hall of fame table: the short menu form (with the menu comment, filled with 0xad9c64) and the
/// long form. Table 0 is the calendar year; 1..27 the leagues, in the order of `hall_of_fame_consts::LEAGUE_NATIONS`.
const HALL_OF_FAME_TABLE_NAMES: [(usize, usize); 28] = [
    (0x9cebbc, 0x9ceba8), // "Jan - Dec", "January - December"
    (0x9ceb7c, 0x9ceb68), (0x9ceb3c, 0x9ceb28), (0x9ceafc, 0x9ceaec), (0x9ceac0, 0x9ceaac), // Argentina .. Brazil
    (0x9cea80, 0x9cea70), (0x9cea44, 0x9cea34), (0x9cea08, 0x9ce9f8), (0x9ce9cc, 0x9ce9bc), // Croatia .. Finland
    (0x9ce990, 0x9ce980), (0x9ce954, 0x9ce944), (0x9ce918, 0x9ce908), (0x9ce8dc, 0x9ce8cc), // France .. Holland
    (0x9ce894, 0x9ce884), (0x9ce85c, 0x9ce84c), (0x9ce824, 0x9ce814), (0x9ce7e0, 0x9ce7c8), // Ireland .. Northern Ireland
    (0x9ce79c, 0x9ce788), (0x9ce75c, 0x9ce74c), (0x9ce720, 0x9ce70c), (0x9ce6e0, 0x9ce6d0), // Norway .. Russia
    (0x9ce6a4, 0x9ce694), (0x9ce66c, 0x9ce65c), (0x9ce630, 0x9ce620), (0x9ce5f4, 0x9ce5e4), // Scotland .. Turkey
    (0x9ce5b4, 0x9ce59c), (0x9ce574, 0x9ce564), (0x9ce534, 0x9ce524), // United States, Wales, South Korea
];

/// Copies one hall of fame entry, chosen by table (0..27) and rank (0..99), into `out` (`HallOfFameEntry::operator=`);
/// returns 1. A table or rank out of range: the "Error" box (hall_of_fame.cpp line 0xe7) and 0.
/// Original: `HallOfFame::get_entry` (0x5ef130, hall_of_fame.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_get_entry(this: usize, table: i8, rank: i8, out: usize) -> u32 {
    use hall_of_fame_consts as hf;
    if !(0..hf::TABLES as i8).contains(&table) || !(0..hf::ENTRIES as i8).contains(&rank) {
        show_error_at(hf::HALL_OF_FAME_CPP, 0xe7);
        return 0;
    }
    let assign: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HALL_OF_FAME_ENTRY_ASSIGN);
    assign(out, this + (table as usize * hf::ENTRIES + rank as usize) * hf::ENTRY_SIZE); // sx12b_7: the chosen entry
    1
}

/// Copies a hall of fame entry: the manager name (50 bytes), the club or nation (50 bytes), the time stamp, the score,
/// the game's random seed and the date (`Date::operator=`). Returns this.
/// Original: `HallOfFameEntry::operator=` (0x5ef200, hall_of_fame.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_entry_assign(this: usize, from: usize) -> usize {
    use hall_of_fame_consts as hf;
    core::ptr::copy(from as *const u8, this as *mut u8, hf::NAME_LEN);
    core::ptr::copy((from + hf::CLUB) as *const u8, (this + hf::CLUB) as *mut u8, hf::NAME_LEN);
    for field in [hf::TIME, hf::SCORE, hf::SEED] {
        write_u32(this + field, read_u32(from + field));
    }
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(this + hf::DATE, tmp.as_mut_ptr() as usize, from + hf::DATE);
    covered!("sx12b_7 hall of fame entry copied");
    this
}

/// Adds an imported entry to a hall of fame table; returns 1 when it went in.
///
/// `HallOfFame::merge_entry(table, entry)`: a table out of 0..27 or a null entry: the "Error" box (line 0x18f) and 0.
/// An identical entry among the table's first 99 (all 120 bytes): 0. An entry with no score, or a score not above the
/// table's last (100th) entry (unsigned): 0. Otherwise it replaces the last entry (`HallOfFameEntry::operator=`) and
/// the table is sorted again (qsort with `hall_of_fame_compare_entries`); 1.
/// Original: `HallOfFame::merge_entry` (0x5ef970, hall_of_fame.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_merge_entry(this: usize, table: i8, entry: usize) -> u32 {
    use hall_of_fame_consts as hf;
    if !(0..hf::TABLES as i8).contains(&table) || entry == 0 {
        show_error_at(hf::HALL_OF_FAME_CPP, 0x18f);
        return 0;
    }
    let t = this + table as usize * hf::TABLE_SIZE;
    let new = core::slice::from_raw_parts(entry as *const u8, hf::ENTRY_SIZE);
    if (0..hf::ENTRIES - 1).any(|k| core::slice::from_raw_parts((t + k * hf::ENTRY_SIZE) as *const u8, hf::ENTRY_SIZE) == new) {
        return 0; // already in the table
    }
    let last = t + (hf::ENTRIES - 1) * hf::ENTRY_SIZE;
    let score = read_u32(entry + hf::SCORE);
    if score == 0 || score <= read_u32(last + hf::SCORE) {
        return 0; // sx12b_7: not above the last entry
    }
    let assign: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HALL_OF_FAME_ENTRY_ASSIGN);
    assign(last, entry);
    exe::crt_qsort(t, hf::ENTRIES as u32, hf::ENTRY_SIZE as u32, hf::COMPARE_ENTRIES);
    covered!("sx12b_7 imported entry merged");
    1
}

/// Reads a hall of fame file into the import buffer and checks it; unless `check_only`, merges its entries into the
/// tables. Returns 1, or 0 when the file is missing or bad.
///
/// `HallOfFame::import_file(path, check_only)`: a null path, the file not opened ("rb"), a short header, a magic other
/// than 0x3abfb2, a version other than 1, short tables (0x52080 bytes into `HALL_OF_FAME_IMPORT`), a short checksum or
/// a checksum that does not match after unscrambling (`hall_of_fame_unscramble_and_check`) give 0, with the "Error"
/// box (lines 0x20f, 0x218, 0x220, 0x229, 0x232, 0x23b, 0x244, 0x24d) only without `check_only`; the file is closed
/// when it was opened. Otherwise, without `check_only`, each imported entry with a time stamp above 0 (signed) is
/// merged into its table (`HallOfFame::merge_entry`, the 28 tables of 100 in order); the file closed, 1.
/// Original: `HallOfFame::import_file` (0x5efd50, hall_of_fame.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_import_file(this: usize, path: usize, check_only: u32) -> u32 {
    use hall_of_fame_consts as hf;
    let reject = |row: u32| {
        if check_only == 0 {
            show_error_at(hf::HALL_OF_FAME_CPP, row);
        }
        0
    };
    if path == 0 {
        return reject(0x20f);
    }
    let file = exe::crt_fopen(path, 0x98f928usize); // "rb"
    if file == 0 {
        return reject(0x218);
    }
    let failed = |row: u32| {
        reject(row);
        exe::crt_fclose(file);
        0
    };
    let mut header = [0u8; 5];
    if exe::crt_fread(header.as_mut_ptr() as usize, 5, 1, file) != 1 {
        return failed(0x220);
    }
    if read_u32(header.as_ptr() as usize) != 0x3abfb2 {
        return failed(0x229);
    }
    if header[4] != 1 {
        return failed(0x232);
    }
    const BYTES: usize = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    if exe::crt_fread(HALL_OF_FAME_IMPORT, BYTES as u32, 1, file) != 1 {
        return failed(0x23b);
    }
    let mut checksum = 0u32;
    if exe::crt_fread(&mut checksum as *mut u32 as usize, 4, 1, file) != 1 {
        return failed(0x244);
    }
    let check: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(0x5f0680usize); // hall_of_fame_unscramble_and_check
    if check(HALL_OF_FAME_IMPORT, BYTES as i32, checksum) == 0 {
        return failed(0x24d);
    }
    if check_only == 0 {
        covered!("sx12b_7 hall of fame file imported");
        let merge: unsafe extern "thiscall" fn(usize, i8, usize) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_MERGE_ENTRY);
        for table in 0..hf::TABLES {
            for k in 0..hf::ENTRIES {
                let entry = HALL_OF_FAME_IMPORT + (table * hf::ENTRIES + k) * hf::ENTRY_SIZE;
                if read_i32(entry + hf::TIME) > 0 {
                    merge(this, table as i8, entry); // sx12b_7: each dated entry merged
                }
            }
        }
    }
    exe::crt_fclose(file);
    1
}

/// Writes the 28 hall of fame tables to a text file; returns 1, or 0 when the file cannot be created.
///
/// `HallOfFame::export_to_csv(path)`: the file opened "w" (failing: the "Error" box, line 0x2d4, and 0). The
/// translated "Hall of Fame" heading ("%s\n\n"); for each table its long name (`hall_of_fame_get_table_name`) as a
/// heading, then each entry with a score (rank 1..100): "%s,%s,%s,%s,%d\n" with the rank as an ordinal
/// (`utils_format_ordinal`), the manager, the club or nation, the date of its time stamp (`localtime`, `Date::set` and
/// the short weekday date string) and the score; a blank line after each table. The file closed, 1.
/// Original: `HallOfFame::export_to_csv` (0x5f0430, hall_of_fame.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_export_to_csv(this: usize, path: usize) -> u32 {
    use hall_of_fame_consts as hf;
    let mut date = [0u8; 8];
    exe::date_date_54a110(date.as_mut_ptr() as usize);
    let file = exe::crt_fopen(path, 0x99c370usize); // "w"
    if file == 0 {
        show_error_at(hf::HALL_OF_FAME_CPP, 0x2d4);
        return 0;
    }
    let fprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_9464F0);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let name: unsafe extern "C" fn(i8, usize, i32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_GET_TABLE_NAME);
    let mut line = [0u8; 0xc8];
    text(line.as_mut_ptr() as usize, 0x9ce514); // "Hall of Fame"
    fprintf(file, 0x9ce50c, line.as_ptr()); // "%s\n\n"
    for table in 0..hf::TABLES {
        name(table as i8, line.as_mut_ptr() as usize, 0xc8, 0);
        fprintf(file, 0x9ce50c, line.as_ptr());
        for rank in 1..=hf::ENTRIES {
            let entry = this + table * hf::TABLE_SIZE + (rank - 1) * hf::ENTRY_SIZE;
            let score = read_u32(entry + hf::SCORE);
            if score == 0 {
                continue;
            }
            let mut stamp = read_u32(entry + hf::TIME);
            let tm = exe::at_946c40(&mut stamp as *mut u32 as usize); // localtime
            exe::date_set(date.as_mut_ptr() as usize, read_u16(tm + 0xc) as u32, read_u8(tm + 0x10) as u32,
                read_u16(tm + 0x14).wrapping_add(1900) as u32, -1); // tm_mday, tm_mon, tm_year
            exe::date_get_short_weekday_date_string(date.as_mut_ptr(), line.as_mut_ptr(), 0xc8, 0xff);
            let ordinal = exe::utils_format_ordinal(rank as i32, -1);
            fprintf(file, 0x9ce4fc, ordinal, entry, entry + hf::CLUB, line.as_ptr(), score); // sx12b_7: one entry line
        }
        fprintf(file, 0x99cae4); // "\n"
    }
    exe::crt_fclose(file);
    1
}

/// Checks a hall of fame file for the import dialog: with the hall of fame (`[0xb5ded8]`) present,
/// `HallOfFame::import_file(path, 1)` (check only, no box) answers 1 when the file is good; otherwise (or with no hall
/// of fame) 2.
/// Original: `hall_of_fame_try_load_quiet` (0x5f1090, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_try_load_quiet(path: usize) -> u16 {
    let hall_of_fame = read_usize(0xb5ded8);
    if hall_of_fame == 0 {
        return 2;
    }
    let import: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_IMPORT_FILE);
    if import(hall_of_fame, path, 1) != 0 { 1 } else { 2 } // sx12b_7: the file accepted
}

/// Fills the menu order of the 28 hall of fame tables (`HALL_OF_FAME_LEAGUE_ORDER`, bytes 0..27), keeps the name form
/// (`form`, as `hall_of_fame_get_table_name` takes it) and sorts the order by the tables' names (qsort with
/// `hall_of_fame_compare_league_names`); returns the order.
/// Original: `hall_of_fame_sort_league_order` (0x5f10c0, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_sort_league_order(form: u32) -> usize {
    for k in 0..0x1cusize {
        write_u8(HALL_OF_FAME_LEAGUE_ORDER + k, k as u8);
    }
    write_u32(HALL_OF_FAME_LEAGUE_ORDER_FORM, form);
    exe::crt_qsort(HALL_OF_FAME_LEAGUE_ORDER, 0x1c, 1, FN_HALL_OF_FAME_COMPARE_LEAGUE_NAMES); // sx12b_7: by name
    HALL_OF_FAME_LEAGUE_ORDER
}

/// Orders two hall of fame table indices (byte pointers) by their names for the menu sort: a null a goes after (1), a
/// null b before (-1); table 0 (the calendar year) goes before any league; otherwise the names in the kept form
/// (`hall_of_fame_get_table_name`, 200-byte buffers) compared by `utils_string_compare` (with 1, 0).
/// Original: `hall_of_fame_compare_league_names` (0x5f1100, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_compare_league_names(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ta, tb) = (read_u8(a), read_u8(b));
    if ta == 0 && tb != 0 {
        return -1; // sx12b_7: the calendar year first
    }
    if ta != 0 && tb == 0 {
        return 1;
    }
    let name: unsafe extern "C" fn(i8, usize, i32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_GET_TABLE_NAME);
    let (mut na, mut nb) = ([0u8; 0xc8], [0u8; 0xc8]);
    name(ta as i8, na.as_mut_ptr() as usize, 0xc8, read_u32(HALL_OF_FAME_LEAGUE_ORDER_FORM));
    name(tb as i8, nb.as_mut_ptr() as usize, 0xc8, read_u32(HALL_OF_FAME_LEAGUE_ORDER_FORM));
    exe::utils_string_compare(na.as_ptr(), nb.as_ptr(), 1, 0)
}

/// Writes the name of a hall of fame table into `out`: the short menu form when `short` is set (the league's nation
/// with the menu comment; "Jan - Dec" for table 0), otherwise the long form ("English League"; "January - December"),
/// translated (`langlib_sprintf`; `size` is checked, not used). Returns 1; a table out of 0..27, a null `out` or a
/// `size` below 50: the "Error" box (line 0x3e6) and 0.
/// Original: `hall_of_fame_get_table_name` (0x5f07c0, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_get_table_name(table: i8, out: usize, size: i32, short: u32) -> u32 {
    if !(0..0x1c).contains(&table) || out == 0 || size < 0x32 {
        show_error_at(hall_of_fame_consts::HALL_OF_FAME_CPP, 0x3e6);
        return 0;
    }
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (menu, long) = HALL_OF_FAME_TABLE_NAMES[table as usize];
    if short == 0 {
        text(out, long);
    } else if table == 0 {
        text(out, menu);
    } else {
        text(out, menu, 0xad9c64usize); // sx12b_7: the menu form
    }
    1
}

/// Whether a hall of fame table and rank (or table alone) are in range, as the routines check before their box.
fn hall_of_fame_table_ok(table: i8) -> bool {
    (0..hall_of_fame_consts::TABLES as i8).contains(&table)
}

/// Comparer for `hall_of_fame_get_entry`: the copied entry and the return, side by side; out of range (the box): only
/// the original.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_get_entry(this: usize, table: i8, rank: i8, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u32>(%%I0%%);
    if !hall_of_fame_table_ok(table) || !(0..hall_of_fame_consts::ENTRIES as i8).contains(&rank) {
        return o(this, table, rank, out);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(out, hall_of_fame_consts::ENTRY_SIZE)], || o(this, table, rank, out),
        || hall_of_fame_get_entry(this, table, rank, out));
    SCOREBOARD.record(equal && r == n, || format!("HallOfFame::get_entry({}, {}) (hall of fame screen): {:?}", table, rank, first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_entry_assign`: the destination entry and the return, side by side.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_entry_assign(this: usize, from: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(this, hall_of_fame_consts::ENTRY_SIZE)], || o(this, from),
        || hall_of_fame_entry_assign(this, from));
    SCOREBOARD.record(equal && r == n, || format!("HallOfFameEntry::operator= (hall of fame screen): {:?}", first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_merge_entry`: the table and the return, side by side; a bad table or entry (the box):
/// only the original.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_merge_entry(this: usize, table: i8, entry: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize) -> u32>(%%I2%%);
    if !hall_of_fame_table_ok(table) || entry == 0 {
        return o(this, table, entry);
    }
    let t = this + table as usize * hall_of_fame_consts::TABLE_SIZE;
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(t, hall_of_fame_consts::TABLE_SIZE)], || o(this, table, entry),
        || hall_of_fame_merge_entry(this, table, entry));
    SCOREBOARD.record(equal && r == n, || format!("HallOfFame::merge_entry({}) (hall of fame import) original {} rust {}: {:?}", table, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_import_file`: the original first checks the file (check only: no box, only the import
/// buffer is written); a bad file: only the original with the real arguments (its box once). A good one: the tables,
/// the import buffer and the return, side by side (the file is read twice).
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_import_file(this: usize, path: usize, check_only: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I3%%);
    if o(this, path, 1) == 0 {
        return o(this, path, check_only);
    }
    let bytes = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, bytes), (HALL_OF_FAME_IMPORT, bytes)],
        || o(this, path, check_only), || hall_of_fame_import_file(this, path, check_only));
    SCOREBOARD.record(equal && r == n, || format!("HallOfFame::import_file(check only {}) (hall of fame screen) original {} rust {}: {:?}",
        check_only, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_export_to_csv`: the original writes the file; the copy writes it again (same path,
/// same tables): the two files and the returns are compared. When the original fails (the box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_export_to_csv(this: usize, path: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I4%%)(this, path);
    if r == 0 {
        return r;
    }
    let name = std::ffi::CStr::from_ptr(path as *const core::ffi::c_char).to_string_lossy().into_owned();
    let written = std::fs::read(&name).unwrap_or_default();
    let n = hall_of_fame_export_to_csv(this, path);
    let again = std::fs::read(&name).unwrap_or_default();
    let first = written.iter().zip(again.iter()).position(|(x, y)| x != y);
    SCOREBOARD.record(r == n && written == again, || format!("HallOfFame::export_to_csv({}) original {} rust {}: sizes {} {}, first different byte {:?}",
        name, r, n, written.len(), again.len(), first));
    r
}

/// Comparer for `hall_of_fame_try_load_quiet`: the import buffer and the answer, side by side.
pub(crate) unsafe extern "C" fn compare_hall_of_fame_try_load_quiet(path: usize) -> u16 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u16>(%%I5%%);
    let bytes = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(HALL_OF_FAME_IMPORT, bytes)], || o(path), || hall_of_fame_try_load_quiet(path));
    SCOREBOARD.record(equal && r == n, || format!("hall_of_fame_try_load_quiet (import dialog) original {} rust {}: {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_sort_league_order`: the order, its form and the return, side by side.
pub(crate) unsafe extern "C" fn compare_hall_of_fame_sort_league_order(form: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(%%I6%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(HALL_OF_FAME_LEAGUE_ORDER_FORM, 4 + 0x1c)], || o(form),
        || hall_of_fame_sort_league_order(form));
    SCOREBOARD.record(equal && r == n, || format!("hall_of_fame_sort_league_order({}) (hall of fame menu): {:?}", form, first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_compare_league_names`: the answer (the routine only reads).
pub(crate) unsafe extern "C" fn compare_hall_of_fame_compare_league_names(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I7%%)(a, b);
    let n = hall_of_fame_compare_league_names(a, b);
    SCOREBOARD.record(r == n, || format!("hall_of_fame_compare_league_names (hall of fame menu) original {} rust {}", r, n));
    r
}

/// Comparer for `hall_of_fame_get_table_name`: the text written and the return, side by side; bad arguments (the
/// box): only the original.
pub(crate) unsafe extern "C" fn compare_hall_of_fame_get_table_name(table: i8, out: usize, size: i32, short: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i8, usize, i32, u32) -> u32>(%%I8%%);
    if !hall_of_fame_table_ok(table) || out == 0 || size < 0x32 {
        return o(table, out, size, short);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(out, (size as usize).min(0xc8))], || o(table, out, size, short),
        || hall_of_fame_get_table_name(table, out, size, short));
    SCOREBOARD.record(equal && r == n, || format!("hall_of_fame_get_table_name({}, short {}) (hall of fame screen): {:?}", table, short, first_differences(&mo, &mr)));
    r
}
