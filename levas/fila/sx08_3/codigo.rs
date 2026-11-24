/// The eight name tables of eng.lng in file order: (offset of the count in the header, offset of the row pointer in
/// the table object, row size). The counts are the 8 header dwords at +0..+0x1c; the rows go to +0x20..+0x3c.
const ENG_LNG_TABLES: [(usize, usize, u32); 8] = [
    (0x0, english_name_tables::CLUB_ROWS, 0x4f),        // clubs, 79 bytes
    (0x4, 0x24, 0xba),                                  // continents, 186 bytes
    (0x8, english_name_tables::NATION_COMP_ROWS, 0x53), // nation competitions, 83 bytes
    (0xc, english_name_tables::NATION_ROWS, 0x6d),      // nations, 109 bytes
    (0x18, english_name_tables::STADIUM_ROWS, 0x34),    // stadiums, 52 bytes
    (0x10, 0x30, 0x34),                                 // cities, 52 bytes
    (0x1c, english_name_tables::STAFF_COMP_ROWS, 0x4f), // staff competitions, 79 bytes
    (0x14, english_name_tables::CLUB_COMP_ROWS, 0x53),  // club competitions, 83 bytes
];

/// True when the eng.lng header counts match the loaded tables: continents, nations, cities, stadiums and staff
/// competitions one by one, and the club plus nation competitions against [`G_CLUB_COMP_COUNT`] − 127; the club
/// count (+0) is not checked.
unsafe fn eng_lng_counts_match(t: usize) -> bool {
    read_u32(t + 4) == read_u32(G_CONTINENT_COUNT)
        && read_u32(t + 0xc) == read_u32(G_NATION_COUNT)
        && read_u32(t + 0x10) == read_u32(G_CITY_COUNT)
        && read_u32(t + 8).wrapping_add(read_u32(t + 0x14)) == read_u32(G_CLUB_COMP_COUNT).wrapping_sub(0x7f)
        && read_u32(t + 0x18) == read_u32(G_STADIUM_COUNT)
        && read_u32(t + 0x1c) == read_u32(0xae2388) // staff competition count
}

/// `eng_lng_load_tables(table)` (thiscall): opens CM3_TEMP\eng.lng (`exe::utils_build_file_path`, `exe::crt_fopen`
/// "rb"); missing file: the Database.cpp error box (line 0x4dd2) and 0.
///
/// Reads the format marker (`exe::utils_read_file_format_marker`: 0xf1fffff1 = big-endian file) and the 8-dword header
/// into +0..+0x1c of the table, byte-swapping each dword when big-endian. Counts that do not match the loaded tables
/// (`eng_lng_counts_match`): closes the file, error box (line 0x4df1) and 0. Otherwise, for each table in file order
/// (`ENG_LNG_TABLES`), `exe::crt_malloc`(count × row size) into its row pointer and, when it came, `exe::crt_fread` of
/// the rows (the row data is never swapped); closes the file and returns 1. Called once, on the first use of an
/// English name (`nation_get_english_name` and its sisters build the 0x40-byte object with the row pointers at zero
/// and keep it in [`G_ENGLISH_NAME_TABLES`]).
/// Original: `eng_lng_load_tables` (0x544fc0, Database.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn eng_lng_load_tables(t: usize) -> u32 {
    let path = exe::utils_build_file_path(0x987da0usize, 0x9b632cusize); // "CM3_TEMP", "eng.lng"
    let file = exe::crt_fopen(path, 0x98f928usize); // "rb"
    if file == 0 {
        database_error_box(0x4dd2);
        return 0;
    }
    let big_endian = exe::utils_read_file_format_marker(file) == 0xf1fffff1;
    exe::crt_fread(t, 0x20, 1, file);
    if big_endian {
        for k in 0..8 {
            write_u32(t + 4 * k, read_u32(t + 4 * k).swap_bytes());
        }
    }
    if !eng_lng_counts_match(t) {
        exe::crt_fclose(file);
        database_error_box(0x4df1);
        return 0;
    }
    for (count, rows, size) in ENG_LNG_TABLES {
        let n = read_u32(t + count);
        let p = exe::crt_malloc(n.wrapping_mul(size));
        write_usize(t + rows, p);
        if p != 0 {
            exe::crt_fread(p, size, n, file);
        }
    }
    exe::crt_fclose(file);
    1
}

/// Comparer for `eng_lng_load_tables`: the original loads into the game's object; the copy loads into a local
/// 0x40-byte object (row pointers at zero, as the callers build it). The answer, the header and each table's rows
/// must match; then the copy's rows are freed (`eng_lng_free_tables`).
pub(crate) unsafe extern "thiscall" fn compare_eng_lng_load_tables(t: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(t);
    let mut copy = [0u8; 0x40];
    let c = copy.as_mut_ptr() as usize;
    let n = eng_lng_load_tables(c);
    let rows_equal = |count: usize, rows: usize, size: u32| {
        let (po, pc) = (read_usize(t + rows), read_usize(c + rows));
        let len = read_u32(t + count).wrapping_mul(size) as usize;
        (po == 0) == (pc == 0)
            && (po == 0 || core::slice::from_raw_parts(po as *const u8, len) == core::slice::from_raw_parts(pc as *const u8, len))
    };
    // without the file nothing is read: the game's header is still the garbage of `new`
    let header_equal = r == 0 || (0..0x20).all(|k| read_u8(t + k) == copy[k]);
    let tables_equal = r == 0 || ENG_LNG_TABLES.iter().all(|&(count, rows, size)| rows_equal(count, rows, size));
    SCOREBOARD.record(r == n && header_equal && tables_equal,
        || format!("eng_lng_load_tables: original={} rust={} header {} tables {}", r, n, header_equal, tables_equal));
    eng_lng_free_tables(c);
    r
}
