/// The C string at `p` (up to its NUL).
unsafe fn lng_c_str<'a>(p: usize) -> &'a [u8] {
    core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes()
}

/// `strncpy(dst, src, n)` of the game's CRT (pads with NULs up to `n`).
unsafe fn lng_strncpy(dst: usize, src: usize, n: u32) {
    exe::at_9466d0(dst, src as *const u8, n);
}

/// A gender byte of the names: a value below 1 (signed, the "none" 0xff included) becomes 0.
unsafe fn lng_clamp_gender(p: usize) {
    if read_i8(p) < 1 {
        write_u8(p, 0);
    }
}

/// The name, gender and short name of a record laid out like `TComp` (also `TClub`, `TNation`, `TStaffComp`: NAME +4,
/// GENDER_NAME +0x37, SHORT_NAME +0x38, its terminator +0x51, SHORT_GENDER_NAME +0x52) from a .lng row (name +0, gender
/// +0x33, short name +0x34, short gender +0x4e): an empty name keeps the old one; the gender is always taken; an empty
/// short name becomes the first 26 bytes of the name (terminated at +0x51) with the name's gender.
unsafe fn lng_apply_named_row(rec: usize, row: usize) {
    if read_u8(row) != 0 {
        lng_strncpy(rec + competition::NAME, row, 0x33);
    }
    write_u8(rec + competition::GENDER_NAME, read_u8(row + 0x33));
    if read_u8(row + 0x34) == 0 {
        lng_strncpy(rec + competition::SHORT_NAME, rec + competition::NAME, 0x1a);
        write_u8(rec + 0x51, 0); // the short name's last byte
        write_u8(rec + competition::SHORT_GENDER_NAME, read_u8(rec + competition::GENDER_NAME));
    } else {
        lng_strncpy(rec + competition::SHORT_NAME, row + 0x34, 0x1a);
        write_u8(rec + competition::SHORT_GENDER_NAME, read_u8(row + 0x4e));
    }
}

/// A competition from a .lng row of 83 bytes: the names (`lng_apply_named_row`), the three-letter name (+0x53 from row
/// +0x4f, when not empty) and both genders clamped.
unsafe fn lng_apply_comp_row(comp: usize, row: usize) {
    lng_apply_named_row(comp, row);
    if read_u8(row + 0x4f) != 0 {
        lng_strncpy(comp + 0x53, row + 0x4f, 4); // three-letter name
    }
    lng_clamp_gender(comp + competition::GENDER_NAME);
    lng_clamp_gender(comp + competition::SHORT_GENDER_NAME);
}

/// A club from a .lng row of 79 bytes (`i` = its row in the club table). When the row has a name but no short name, a
/// club whose short name equals its name gets the new name as short name too, if it is shorter than 26 bytes. Then the
/// name (when not empty) and the gender; a short name in the row is taken with its gender; without one, the national
/// teams (the last 2 × nations rows) get the first 26 bytes of the name with the name's gender, and the other clubs keep
/// theirs and get the name's gender only when the short name's first letter equals the name's and the row's short
/// gender is 0xff ("none"), the row's short gender otherwise. Spanish names (language 5) turn a "none" gender into 1 and
/// a "none" short gender into the gender; the other languages clamp both.
unsafe fn lng_apply_club_row(language: u32, i: i32, club: Club, row: usize) {
    let c = club.addr();
    if read_u8(row + 0x34) == 0 && read_u8(row) != 0 && lng_c_str(c + club::NAME) == lng_c_str(c + club::SHORT_NAME) && lng_c_str(row).len() < 0x1a {
        covered!("sx08b_1 club short name follows the name");
        lng_strncpy(c + club::SHORT_NAME, row, 0x1a);
    }
    if read_u8(row) != 0 {
        lng_strncpy(c + club::NAME, row, 0x33);
    }
    write_u8(c + club::GENDER_NAME, read_u8(row + 0x33));
    if read_u8(row + 0x34) != 0 {
        lng_strncpy(c + club::SHORT_NAME, row + 0x34, 0x1a);
        write_u8(c + club::SHORT_GENDER_NAME, read_u8(row + 0x4e));
    } else if i >= read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
        covered!("sx08b_1 national team short name from the name");
        lng_strncpy(c + club::SHORT_NAME, c + club::NAME, 0x1a);
        write_u8(c + 0x51, 0); // the short name's last byte
        write_u8(c + club::SHORT_GENDER_NAME, read_u8(c + club::GENDER_NAME));
    } else if read_u8(c + club::SHORT_NAME) == read_u8(c + club::NAME) && read_u8(row + 0x4e) == 0xff {
        write_u8(c + club::SHORT_GENDER_NAME, read_u8(c + club::GENDER_NAME));
    } else {
        write_u8(c + club::SHORT_GENDER_NAME, read_u8(row + 0x4e));
    }
    if language == 5 {
        covered!("sx08b_1 spanish club genders");
        if read_u8(c + club::GENDER_NAME) == 0xff {
            write_u8(c + club::GENDER_NAME, 1);
        }
        if read_u8(c + club::SHORT_GENDER_NAME) == 0xff {
            write_u8(c + club::SHORT_GENDER_NAME, read_u8(c + club::GENDER_NAME));
        }
    } else {
        lng_clamp_gender(c + club::GENDER_NAME);
        lng_clamp_gender(c + club::SHORT_GENDER_NAME);
    }
}

/// Reads `n` rows of `size` bytes from the .lng file and hands each (index, row) to `apply`: all at once into a
/// `malloc` block, freed after; when `malloc` fails, one row at a time into a stack buffer (the original repeats each
/// loop for that case).
unsafe fn lng_for_each_row(file: usize, n: i32, size: u32, mut apply: impl FnMut(i32, usize)) {
    let rows = exe::crt_malloc((n as u32).wrapping_mul(size));
    if rows == 0 {
        covered!("sx08b_1 rows one by one");
        let mut row = [0u8; 0xba];
        for k in 0..n {
            exe::crt_fread(row.as_mut_ptr() as usize, size, 1, file);
            apply(k, row.as_ptr() as usize);
        }
        return;
    }
    exe::crt_fread(rows, size, n as u32, file);
    for k in 0..n {
        apply(k, rows + k as usize * size as usize);
    }
    exe::crt_free(rows);
}

/// The club names (79-byte rows). With the English-name index ([`G_CLUB_ENGLISH_NAME_INDEX_TABLE`], built by the
/// database cuts: club row → row of the file), club i takes file row index[i]: the whole block read, every club of the
/// table; without the block, one `fseek` per club (to 0x20 + index[i] × 79) for the file's count of clubs, then to the
/// end of the club rows. Without the index, club k takes file row k.
unsafe fn lng_load_club_names(file: usize, n: i32, language: u32) {
    let index = read_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE);
    if index == 0 {
        lng_for_each_row(file, n, 0x4f, |k, row| lng_apply_club_row(language, k, club_at(k as isize), row));
        return;
    }
    covered!("sx08b_1 club names through the index");
    let file_row = |i: i32| read_i32(index + 4 * i as usize);
    let rows = exe::crt_malloc((n as u32).wrapping_mul(0x4f));
    if rows != 0 {
        exe::crt_fread(rows, 0x4f, n as u32, file);
        let mut i = 0;
        while i < read_i32(G_CLUB_COUNT) {
            lng_apply_club_row(language, i, club_at(i as isize), rows.wrapping_add((file_row(i) as usize).wrapping_mul(0x4f)));
            i += 1;
        }
        exe::crt_free(rows);
        return;
    }
    let mut row = [0u8; 0x4f];
    for i in 0..n {
        exe::crt_fseek(file, file_row(i).wrapping_mul(0x4f).wrapping_add(0x20), 0);
        exe::crt_fread(row.as_mut_ptr() as usize, 0x4f, 1, file);
        lng_apply_club_row(language, i, club_at(i as isize), row.as_ptr() as usize);
    }
    exe::crt_fseek(file, n.wrapping_mul(0x4f).wrapping_add(0x20), 0);
}

/// A continent from a 186-byte row: name (+4, 26 bytes) when not empty, gender (+0x1e), the second name (+0x23, 26),
/// the three-letter name (+0x1f, 4), the long name (+0x3d, 101), its gender (+0xa2), another name (+0xa3, 26) and its
/// gender (+0xbd), each string only when not empty; the first gender clamped.
unsafe fn lng_apply_continent_row(c: usize, row: usize) {
    if read_u8(row) != 0 {
        lng_strncpy(c + 4, row, 0x1a); // name
    }
    write_u8(c + 0x1e, read_u8(row + 0x1a)); // name gender
    if read_u8(row + 0x1f) != 0 {
        lng_strncpy(c + 0x23, row + 0x1f, 0x1a); // continental name
    }
    if read_u8(row + 0x1b) != 0 {
        lng_strncpy(c + 0x1f, row + 0x1b, 4); // three-letter name
    }
    if read_u8(row + 0x39) != 0 {
        lng_strncpy(c + 0x3d, row + 0x39, 0x65); // federation name
    }
    write_u8(c + 0xa2, read_u8(row + 0x9e)); // federation name gender
    if read_u8(row + 0x9f) != 0 {
        lng_strncpy(c + 0xa3, row + 0x9f, 0x1a); // federation short name
    }
    write_u8(c + 0xbd, read_u8(row + 0xb9)); // federation short name gender
    lng_clamp_gender(c + 0x1e);
}

/// A nation from a 109-byte row: the names (`lng_apply_named_row`), the three-letter name (+0x53 from row +0x4f) and
/// the nationality (+0x57 from row +0x53), each when not empty; both genders clamped.
unsafe fn lng_apply_nation_row(nat: Nation, row: usize) {
    let n = nat.addr();
    lng_apply_named_row(n, row);
    if read_u8(row + 0x4f) != 0 {
        lng_strncpy(n + nation::THREE_LETTER_NAME, row + 0x4f, 4);
    }
    if read_u8(row + 0x53) != 0 {
        lng_strncpy(n + nation::NATIONALITY, row + 0x53, 0x1a);
    }
    lng_clamp_gender(n + nation::GENDER_NAME);
    lng_clamp_gender(n + nation::SHORT_GENDER_NAME);
}

/// Loads the translated names of the database from the language's .lng in CM3_TEMP.
///
/// Nothing without the nation table or when the names are already in that language ([`G_DATABASE_NAMES_LANGUAGE`]).
/// English (0, 1, 11, 12): eng.lng, and the English-name tables object ([`G_ENGLISH_NAME_TABLES`], kept for the other
/// languages) is freed; the bar has 16 steps when `two_part_bar` (the first half of a switch), 8 otherwise. Any other
/// language first goes back to English (a call to itself with (0, 1)) and then reads its own file in the second half
/// of the bar (steps 9..16 of 16, without starting it again). An unknown language: the Database.cpp error box (line
/// 0x1e37), then the French names. The bar text is "Re-loading Database"; it starts at step 0 except in the second half.
///
/// The file missing: error box (line 0x1e48) and nothing else. Then the format marker and the 8-dword header (byte-
/// swapped in a big-endian file); counts that do not match the loaded tables (the check of `eng_lng_load_tables`): the
/// file is closed, error box (line 0x1e68), the language stays. Otherwise, in file order and a bar step after each:
/// clubs (`lng_load_club_names`), continents, the nation competitions (written to the club competition table before
/// its 127 reserve rows), nations, stadiums (name 51 bytes and clamped gender), cities (name 26 bytes, gender at +0x1e),
/// staff competitions (names only), club competitions; each table read whole into a `malloc` block or, failing that,
/// row by row. The file is closed, the last step shown, the language set, and the pundit list ([`G_PUNDIT_LIST`]), when
/// there is one, rebuilt in the new language (destroyed, deleted and a new one of 10 bytes built; 0 when `new` fails).
///
/// Defects of the original: an unknown language loads the French names after the error box; when the club row block
/// cannot be allocated with the English-name index, the loop runs over the file's count of clubs, not the table's.
/// Original: `database_load_language_names` (0x5339a0, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_load_language_names(language: u32, two_part_bar: u32) {
    if read_usize(G_NATION_TABLE) == 0 || read_u32(G_DATABASE_NAMES_LANGUAGE) == language {
        return;
    }
    covered!("sx08b_1 names reloaded");
    let mut path = [0u8; 0x104];
    let bar: u8;
    if matches!(language, 0 | 1 | 11 | 12) {
        copy_c_string(path.as_mut_ptr() as usize, exe::utils_build_file_path(0x987da0usize, LANGUAGE_FILES[0])); // "CM3_TEMP"
        bar = (two_part_bar != 0) as u8;
        let t = read_usize(G_ENGLISH_NAME_TABLES);
        if t != 0 {
            covered!("sx08b_1 english name tables freed");
            eng_lng_free_tables(t);
            exe::crt_operator_delete(t);
            write_usize(G_ENGLISH_NAME_TABLES, 0);
        }
    } else {
        let file = match LANGUAGE_FILES.get(language as usize) {
            Some(&f) => f,
            None => {
                covered!("sx08b_1 unknown language");
                database_error_box(0x1e37);
                LANGUAGE_FILES[2] // fra.lng
            }
        };
        database_load_language_names(0, 1); // back to the English names first
        copy_c_string(path.as_mut_ptr() as usize, exe::utils_build_file_path(0x987da0usize, file)); // "CM3_TEMP"
        bar = 2;
    }
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut text = [0u8; 200];
    langlib(text.as_mut_ptr() as usize, 0x9b62e8usize); // "Re-loading Database"
    let total: u32 = if bar != 0 { 16 } else { 8 };
    let first: i32 = if bar >= 2 { 8 } else { 0 };
    let step = |k: i32| exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, first + k, total, -1);
    if bar < 2 {
        step(0);
    }
    let file = exe::crt_fopen(path.as_ptr() as usize, 0x98f928usize); // "rb"
    if file == 0 {
        database_error_box(0x1e48);
        return;
    }
    let big_endian = exe::utils_read_file_format_marker(file) == 0xf1fffff1;
    let mut header = [0u32; 8];
    exe::crt_fread(header.as_mut_ptr() as usize, 0x20, 1, file);
    if big_endian {
        for v in header.iter_mut() {
            *v = v.swap_bytes();
        }
    }
    if !eng_lng_counts_match(header.as_ptr() as usize) {
        exe::crt_fclose(file);
        database_error_box(0x1e68);
        return;
    }
    let count = |k: usize| header[k] as i32;
    lng_load_club_names(file, count(0), language);
    step(1);
    lng_for_each_row(file, count(1), 0xba, |k, row| lng_apply_continent_row(read_usize(G_CONTINENT_TABLE) + k as usize * 0xc6, row));
    step(2);
    // the nation competitions sit right before the 127 reserve rows of the club competition table
    let nation_comps = count(2);
    lng_for_each_row(file, nation_comps, 0x53, |k, row| {
        let id = k - nation_comps - 0x7f + read_i32(G_CLUB_COMP_COUNT);
        lng_apply_comp_row(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b)), row);
    });
    step(3);
    lng_for_each_row(file, count(3), 0x6d, |k, row| lng_apply_nation_row(Nation(read_usize(G_NATION_TABLE) + k as usize * nation::SIZE), row));
    step(4);
    lng_for_each_row(file, count(6), 0x34, |k, row| {
        let s = read_usize(G_STADIUM_TABLE) + k as usize * stadium::SIZE;
        if read_u8(row) != 0 {
            lng_strncpy(s + stadium::NAME, row, 0x33);
        }
        write_u8(s + stadium::GENDER_NAME, read_u8(row + 0x33));
        lng_clamp_gender(s + stadium::GENDER_NAME);
    });
    step(5);
    lng_for_each_row(file, count(4), 0x34, |k, row| {
        let c = read_usize(G_CITY_TABLE) + k as usize * 0x38; // city record, 56 bytes
        if read_u8(row) != 0 {
            lng_strncpy(c + 4, row, 0x1a); // name
        }
        write_u8(c + city::GENDER_NAME, read_u8(row + 0x33));
        lng_clamp_gender(c + city::GENDER_NAME);
    });
    step(6);
    lng_for_each_row(file, count(7), 0x4f, |k, row| {
        let s = read_usize(G_STAFF_COMP_TABLE) + k as usize * 0x65; // staff competition record, 101 bytes
        lng_apply_named_row(s, row);
        lng_clamp_gender(s + competition::GENDER_NAME);
        lng_clamp_gender(s + competition::SHORT_GENDER_NAME);
    });
    step(7);
    lng_for_each_row(file, count(5), 0x53, |k, row| lng_apply_comp_row(read_usize(G_CLUB_COMP_TABLE) + k as usize * 0x6b, row));
    exe::crt_fclose(file);
    step(8);
    let pundits = read_usize(G_PUNDIT_LIST);
    write_u32(G_DATABASE_NAMES_LANGUAGE, language);
    if pundits == 0 {
        return;
    }
    covered!("sx08b_1 pundit list rebuilt");
    pundit_list_destruct(pundits);
    exe::crt_operator_delete(pundits);
    let p = exe::crt_operator_new(10);
    write_usize(G_PUNDIT_LIST, if p != 0 { pundit_list_construct(p, language) } else { 0 });
}

/// The tables `database_load_language_names` rewrites: (table pointer, row count, record size).
const LNG_NAME_TABLES: [(usize, usize, usize); 7] = [
    (G_CLUB_TABLE, G_CLUB_COUNT, club::SIZE),
    (G_CONTINENT_TABLE, G_CONTINENT_COUNT, 0xc6),
    (G_NATION_TABLE, G_NATION_COUNT, nation::SIZE),
    (G_STADIUM_TABLE, G_STADIUM_COUNT, stadium::SIZE),
    (G_CITY_TABLE, G_CITY_COUNT, 0x38),
    (G_STAFF_COMP_TABLE, 0xae2388, 0x65), // staff competition count
    (G_CLUB_COMP_TABLE, G_CLUB_COMP_COUNT, 0x6b),
];

/// A copy of the bytes of the seven name tables (empty for a table not allocated).
unsafe fn lng_snapshot() -> Vec<Vec<u8>> {
    LNG_NAME_TABLES.iter().map(|&(table, count, size)| {
        let (t, n) = (read_usize(table), read_i32(count).max(0) as usize);
        if t == 0 { Vec::new() } else { core::slice::from_raw_parts(t as *const u8, n * size).to_vec() }
    }).collect()
}

/// Puts the bytes of `lng_snapshot` back.
unsafe fn lng_restore(s: &[Vec<u8>]) {
    for (&(table, _, _), bytes) in LNG_NAME_TABLES.iter().zip(s) {
        let t = read_usize(table);
        if t != 0 {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), t as *mut u8, bytes.len());
        }
    }
}

/// Comparer for `database_load_language_names`, side by side on the seven name tables: the original runs (its call to
/// itself for English passes through this comparer again and leaves the original's result), then the tables and the
/// names language go back to before and the copy runs from the same state (the English tables object the original
/// freed stays 0; the pundit list the original built is rebuilt by the copy). The tables, the language, the English
/// tables object and the pundit list must match; the game goes on with the original's tables.
pub(crate) unsafe extern "C" fn compare_database_load_language_names(language: u32, two_part_bar: u32) {
    let before = lng_snapshot();
    let language_before = read_u32(G_DATABASE_NAMES_LANGUAGE);
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I0%%)(language, two_part_bar);
    let original = lng_snapshot();
    let (language_original, english_original, pundits_original) =
        (read_u32(G_DATABASE_NAMES_LANGUAGE), read_usize(G_ENGLISH_NAME_TABLES), read_usize(G_PUNDIT_LIST) != 0);
    lng_restore(&before);
    write_u32(G_DATABASE_NAMES_LANGUAGE, language_before);
    database_load_language_names(language, two_part_bar);
    let copy = lng_snapshot();
    let tables_equal = original == copy;
    let language_copy = read_u32(G_DATABASE_NAMES_LANGUAGE);
    SCOREBOARD.record(tables_equal && language_original == language_copy
            && (english_original == 0) == (read_usize(G_ENGLISH_NAME_TABLES) == 0)
            && pundits_original == (read_usize(G_PUNDIT_LIST) != 0),
        || format!("database_load_language_names({}, {}): tables equal {} (first differing table {:?}) language {} / {}",
            language, two_part_bar, tables_equal, original.iter().zip(&copy).position(|(a, b)| a != b), language_original, language_copy));
    lng_restore(&original);
    write_u32(G_DATABASE_NAMES_LANGUAGE, language_original);
}
