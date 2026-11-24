// ---------- batch %%LEVA%%: the TrueType font list: the FONTS folder, the font modes and the font names (font code of fog_of_war.cpp) ----------

/// The font folder and the font file table (the FONTS folder found under SI_DATA or CM3_DATA, scanned once).
#[allow(dead_code)]
mod font_files {
    /// char[0x104]: the FONTS folder (empty when neither data folder has one).
    pub(crate) const FOLDER: usize = 0xae2a64;
    /// u32: 1 once the folder was scanned and at least one font file was listed.
    pub(crate) const SCANNED: usize = 0xae2c68;
    /// Pointer to the font file table (`ENTRY_SIZE` bytes each), allocated by the scan.
    pub(crate) const TABLE: usize = 0xae2c6c;
    /// u16: the number of entries in the table (the scan stops at `MAX_FILES`).
    pub(crate) const COUNT: usize = 0xae2c70;
    pub(crate) const MAX_FILES: u16 = 100;
    /// An entry: the file name (`_strdup`), the full font name from the 'name' table (`_strdup`, 0 when none) and the
    /// face index inside the file (always 0: collections are not walked).
    pub(crate) const ENTRY_FILE_NAME: usize = 0x0;
    pub(crate) const ENTRY_FULL_NAME: usize = 0x4;
    pub(crate) const ENTRY_FACE_INDEX: usize = 0x8;
    pub(crate) const ENTRY_SIZE: usize = 0xa;
    /// "SI_DATA", "CM3_DATA" and "FONTS" in the exe.
    pub(crate) const SI_DATA: usize = 0x9c3718;
    pub(crate) const CM3_DATA: usize = 0x9b5348;
    pub(crate) const FONTS: usize = 0x9cc2c8;
    /// "*.tt?", ".ttf", ".ttc", ".t2k" and "rb" in the exe.
    pub(crate) const PATTERN: usize = 0x9cc360;
    pub(crate) const EXT_TTF: usize = 0x9cc2e0;
    pub(crate) const EXT_TTC: usize = 0x9cc2d8;
    pub(crate) const EXT_T2K: usize = 0x9cc2d0;
    pub(crate) const READ_BINARY: usize = 0x98f928;
    /// The langlib formats of the two built-in modes and their (empty) argument, and the buffer they are written to.
    pub(crate) const FUTURISTIC: usize = 0x9cc318;
    pub(crate) const TRADITIONAL: usize = 0x9cc33c;
    pub(crate) const EMPTY_TEXT: usize = 0xad9c64;
    pub(crate) const MODE_NAME_BUFFER: usize = 0xde1f64;
    /// The font modes below the file indices: -2 Futuristic (the game's own T2K fonts), -1 Traditional (bitmap fonts).
    pub(crate) const MODE_FUTURISTIC: i32 = -2;
    pub(crate) const MODE_TRADITIONAL: i32 = -1;
    /// The 'name' table tag and the full font name id.
    pub(crate) const TAG_NAME: u32 = 0x6e616d65;
    pub(crate) const NAME_ID_FULL_NAME: i16 = 4;
}

/// The FONTS folder lookup that `FONT::FONT`, `font_set_mode`, `font_get_file_count` and `font_get_mode_name` each do
/// inline: unless already scanned, the folder is SI_DATA\FONTS, else CM3_DATA\FONTS (`utils_build_file_path`), copied
/// into `font_files::FOLDER` (0x104 bytes, the last one zeroed) and scanned (`font_scan_folder`, through the exe); a
/// scan that lists a file marks the folder scanned. Without either folder the path is emptied and nothing is scanned
/// (the next call looks again).
pub(crate) unsafe fn font_find_folder_once() {
    use font_files as ff;
    if read_u32(ff::SCANNED) != 0 {
        return;
    }
    let mut path = exe::utils_build_file_path(ff::SI_DATA, ff::FONTS);
    if path == 0 {
        path = exe::utils_build_file_path(ff::CM3_DATA, ff::FONTS);
    }
    if path == 0 {
        write_u8(ff::FOLDER, 0);
        return;
    }
    exe::at_9466d0(ff::FOLDER, path as *const u8, 0x104);
    write_u8(ff::FOLDER + 0x103, 0);
    let scan: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_FONT_SCAN_FOLDER);
    if scan() != 0 {
        write_u32(ff::SCANNED, 1);
    }
}

/// font_scan_folder (cdecl, no arguments): lists the TrueType files of the FONTS folder.
pub(crate) const FN_FONT_SCAN_FOLDER: usize = 0x5b2170;

/// Sets the font mode used for the interface text.
///
/// After the folder lookup (`font_find_folder_once`), stores `mode` in `G_FONT_MODE` when it is below the number of
/// font files. Defect of the original: the test meant -2, -1 or a file index, but every negative mode passes (a mode
/// below -2 is compared with the file count too).
/// Original: `font_set_mode` (0x5b1f30, fog_of_war.cpp, cdecl).
pub(crate) unsafe extern "C" fn font_set_mode(mode: i32) {
    font_find_folder_once();
    if mode < read_u16(font_files::COUNT) as i32 {
        covered!("sx10_12 font mode set");
        write_i32(G_FONT_MODE, mode);
    }
}

/// Returns the number of font files found in the FONTS folder (after the folder lookup).
/// Original: `font_get_file_count` (0x5b1fc0, fog_of_war.cpp, cdecl).
pub(crate) unsafe extern "C" fn font_get_file_count() -> u32 {
    font_find_folder_once();
    read_u16(font_files::COUNT) as u32
}

/// Writes a font mode's display name into `buffer` (`size` bytes, always terminated).
///
/// After the folder lookup: -2 the langlib text "Futuristic", -1 "Traditional" (both formatted into the shared buffer
/// 0xde1f64 first); a file index below the count, with the table allocated, the entry's full font name, or its file
/// name when the file has none; anything else an empty string. The copy is `strncpy` of `size` bytes, then the last byte
/// zeroed.
/// Original: `font_get_mode_name` (0x5b2040, fog_of_war.cpp, cdecl).
pub(crate) unsafe extern "C" fn font_get_mode_name(mode: i32, buffer: usize, size: i32) {
    use font_files as ff;
    font_find_folder_once();
    let name = if mode == ff::MODE_FUTURISTIC || mode == ff::MODE_TRADITIONAL {
        let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
        langlib(ff::MODE_NAME_BUFFER, if mode == ff::MODE_FUTURISTIC { ff::FUTURISTIC } else { ff::TRADITIONAL }, ff::EMPTY_TEXT);
        ff::MODE_NAME_BUFFER
    } else {
        let table = read_usize(ff::TABLE);
        if table == 0 || mode < 0 || mode >= read_u16(ff::COUNT) as i32 {
            write_u8(buffer, 0);
            return;
        }
        let entry = table + mode as usize * ff::ENTRY_SIZE;
        covered!("sx10_12 font file name");
        let full = read_usize(entry + ff::ENTRY_FULL_NAME);
        if full != 0 { full } else { read_usize(entry + ff::ENTRY_FILE_NAME) }
    };
    exe::at_9466d0(buffer, name as *const u8, size as u32);
    write_u8(buffer.wrapping_add(size as usize).wrapping_sub(1), 0);
}

/// Whether a file extension is one of the font files the game reads: ".ttf", ".ttc" or ".t2k" (`stricmp`).
pub(crate) unsafe fn font_is_font_extension(ext: &[u8]) -> bool {
    use font_files as ff;
    [ff::EXT_TTF, ff::EXT_TTC, ff::EXT_T2K].iter().any(|&e| exe::crt_stricmp(ext.as_ptr() as usize, e) == 0)
}

/// Lists the TrueType files of the FONTS folder in the font file table; returns 1 when at least one was listed.
///
/// Every file matching "*.tt?" in `font_files::FOLDER` (`file_find_first`/`file_find_next`, at most 100 files counted)
/// whose extension is .ttf, .ttc or .t2k (`font_is_font_extension`; .t2k never matches the pattern) and that opens
/// ("rb") gets an entry: its file name (`_strdup`), face index 0 and the full font name read from the file
/// (`font_read_full_name`, 0 when none). The entries are gathered in a local table of 100 and copied into a new table
/// (`operator new`, `font_files::TABLE`) at the end; no file listed returns 0 and allocates nothing.
///
/// The original loops a second time on each listed file with face index 1 and stops at the index test (collections are
/// never walked): that pass only rebuilds a local path and is left out.
///
/// Defects of the original: the count (`font_files::COUNT`) is not reset, so a second scan after a failed allocation
/// would append after entries the local table never filled; the duplicated file name leaks for files skipped by the
/// extension or that fail to open.
/// Original: `font_scan_folder` (0x5b2170, fog_of_war.cpp, cdecl).
pub(crate) unsafe extern "C" fn font_scan_folder() -> u32 {
    use font_files as ff;
    let mut pattern = [0u8; 0x104];
    exe::crt_makepath(pattern.as_mut_ptr(), 0usize, ff::FOLDER, ff::PATTERN, 0usize);
    let mut find = [0u8; 0x114];
    let mut table = [0u8; ff::MAX_FILES as usize * ff::ENTRY_SIZE];
    if exe::file_find_first(pattern.as_ptr() as usize, find.as_mut_ptr() as usize, 0) == 0 {
        loop {
            font_scan_file(find.as_ptr() as usize + file_find_state::FILE_NAME, &mut table);
            if read_u16(ff::COUNT) == ff::MAX_FILES || exe::file_find_next(find.as_mut_ptr()) != 0 {
                break;
            }
        }
    }
    exe::file_find_close(find.as_mut_ptr() as usize);
    let count = read_u16(ff::COUNT) as usize;
    if count == 0 {
        return 0;
    }
    let bytes = count * ff::ENTRY_SIZE;
    let copy = exe::crt_operator_new(bytes as u32);
    write_usize(ff::TABLE, copy);
    if copy == 0 {
        return 0;
    }
    core::ptr::copy_nonoverlapping(table.as_ptr(), copy as *mut u8, bytes);
    covered!("sx10_12 font folder scanned");
    1
}

/// One file found by `font_scan_folder`: the entry at the current count of the local table, when the file is a font
/// file that opens; the count goes up by one.
unsafe fn font_scan_file(found_name: usize, table: &mut [u8]) {
    use font_files as ff;
    let name = exe::at_952f5b(found_name as *const u8);
    if name == 0 {
        return;
    }
    let mut path = [0u8; 0x104];
    let mut ext = [0u8; 0x100];
    exe::crt_makepath(path.as_mut_ptr(), 0usize, ff::FOLDER, name, 0usize);
    exe::crt_strtok(path.as_ptr() as usize, 0usize, 0usize, core::ptr::null_mut(), ext.as_mut_ptr());
    if !font_is_font_extension(&ext) {
        return;
    }
    let mut file = exe::crt_fopen(path.as_ptr() as usize, ff::READ_BINARY);
    if file == 0 {
        return;
    }
    let count = read_u16(ff::COUNT);
    let entry = table.as_mut_ptr() as usize + count as usize * ff::ENTRY_SIZE;
    write_usize(entry + ff::ENTRY_FILE_NAME, name);
    write_u16(entry + ff::ENTRY_FACE_INDEX, 0);
    write_usize(entry + ff::ENTRY_FULL_NAME, font_read_full_name(&mut file));
    exe::crt_fclose(file);
    write_u16(ff::COUNT, count.wrapping_add(1)); // sx10_12: one more font file
}

/// The full font name of an open TrueType file, `_strdup`ed, or 0.
///
/// Reads the offset table at the start of the file (12 bytes; the number of tables is the big-endian word at +4; none
/// gives 0), then each 16-byte table record (`font_file_read_at` at 12 + 16 × k, converted by
/// `ttf_swap_table_record`) until the 'name' table: its header (6 bytes at the table's offset, `ttf_swap_name_table_header`)
/// gives the record count and the string storage (table offset + string offset); each 12-byte name record
/// (`ttf_swap_name_record`) is read with `ttf_read_name_string` for name id 4 into a 255-byte buffer, and the first
/// non-empty one is the name. A record or table that cannot be read is skipped.
unsafe fn font_read_full_name(file: &mut usize) -> usize {
    use font_files as ff;
    let fp = &mut *file as *mut usize as usize;
    let mut head = [0u8; 12];
    if exe::crt_fseek(*file, 0, 0) != 0 || exe::crt_fread(head.as_mut_ptr() as usize, 12, 1, *file) != 1 {
        return 0;
    }
    let tables = u16::from_be_bytes([head[4], head[5]]);
    let mut record = [0u32; 4];
    let mut header = [0u16; 3];
    let mut name_record = [0u16; 6];
    let mut text = [0u8; 0x100];
    for k in 0..tables as u32 {
        if font_file_read_at(fp, 12 + 16 * k, record.as_mut_ptr() as usize, 16) == 0
            || ttf_swap_table_record(record.as_mut_ptr() as usize) == 0
            || record[0] != ff::TAG_NAME
        {
            continue;
        }
        let table_offset = record[2];
        if font_file_read_at(fp, table_offset, header.as_mut_ptr() as usize, 6) == 0
            || ttf_swap_name_table_header(header.as_mut_ptr() as usize) == 0
        {
            continue;
        }
        let storage = (header[2] as u32).wrapping_add(table_offset);
        for r in 0..header[1] as u32 {
            if font_file_read_at(fp, table_offset + 6 + 12 * r, name_record.as_mut_ptr() as usize, 12) == 0
                || ttf_swap_name_record(name_record.as_mut_ptr() as usize) == 0
            {
                continue;
            }
            if ttf_read_name_string(fp, name_record.as_ptr() as usize, ff::NAME_ID_FULL_NAME as i32, storage, text.as_mut_ptr() as usize, 0xff) != 0 {
                covered!("sx10_12 full font name");
                return exe::at_952f5b(text.as_ptr());
            }
        }
    }
    0
}

/// Converts the four big-endian dwords of a TrueType table directory record (tag, checksum, offset, length) to host
/// order in place; returns 1.
/// Original: `ttf_swap_table_record` (0x5b25a0, fog_of_war.cpp, thiscall on the record).
pub(crate) unsafe extern "thiscall" fn ttf_swap_table_record(record: usize) -> u32 {
    for k in 0..4 {
        write_u32(record + 4 * k, read_u32(record + 4 * k).swap_bytes());
    }
    1
}

/// Converts the three big-endian words of a TrueType 'name' table header (format, count, string offset) to host order;
/// returns 1.
/// Original: `ttf_swap_name_table_header` (0x5b2650, fog_of_war.cpp, thiscall on the header).
pub(crate) unsafe extern "thiscall" fn ttf_swap_name_table_header(header: usize) -> u32 {
    for k in 0..3 {
        write_u16(header + 2 * k, read_u16(header + 2 * k).swap_bytes());
    }
    1
}

/// Converts the six big-endian words of a TrueType name record (platform, encoding, language, name id, length,
/// offset) to host order; returns 1.
/// Original: `ttf_swap_name_record` (0x5b2680, fog_of_war.cpp, thiscall on the record).
pub(crate) unsafe extern "thiscall" fn ttf_swap_name_record(record: usize) -> u32 {
    for k in 0..6 {
        write_u16(record + 2 * k, read_u16(record + 2 * k).swap_bytes());
    }
    1
}

/// Reads the string of a TrueType name record into `buffer` when the record has the wanted name id.
///
/// `file` points to the open `FILE *`. Another name id (word +6), a length (word +8) that leaves nothing after capping it
/// at `size` - 1, no file, a failed seek to storage + record offset (word +0xa) or a short read: 0. Otherwise the
/// string is terminated and the result is 1 when it does not start with a zero byte (the UTF-16 names of the Windows
/// platform start with one and are rejected).
/// Original: `ttf_read_name_string` (0x5b26e0, fog_of_war.cpp, cdecl).
pub(crate) unsafe extern "C" fn ttf_read_name_string(file: usize, record: usize, name_id: i32, storage: u32, buffer: usize, size: i32) -> u32 {
    if read_u16(record + 6) as i32 != name_id as i16 as i32 {
        return 0;
    }
    let length = (read_u16(record + 8) as i32).min(size.wrapping_sub(1));
    if length <= 0 {
        return 0;
    }
    let f = read_usize(file);
    if f == 0
        || exe::crt_fseek(f, (read_u16(record + 0xa) as u32).wrapping_add(storage) as i32, 0) != 0
        || exe::crt_fread(buffer, length as u32, 1, f) != 1
    {
        return 0;
    }
    write_u8(buffer + length as usize, 0);
    (read_u8(buffer) != 0) as u32
}

/// Seeks the open font file (`*file`) to `offset` and reads one block of `size` bytes into `buffer`; 1 when the block
/// was read, 0 without a file, on a failed seek or a short read.
/// Original: `font_file_read_at` (0x5b2770, fog_of_war.cpp, cdecl).
pub(crate) unsafe extern "C" fn font_file_read_at(file: usize, offset: u32, buffer: usize, size: u32) -> u32 {
    let f = read_usize(file);
    if f == 0 || exe::crt_fseek(f, offset as i32, 0) != 0 {
        return 0;
    }
    (exe::crt_fread(buffer, size, 1, f) == 1) as u32
}

/// Comparer for `font_set_mode`: before the folder was scanned only the original runs (the scan allocates the table
/// and duplicates the names); afterwards side by side on the font mode.
pub(crate) unsafe extern "C" fn compare_font_set_mode(mode: i32) {
    let original = original_fn::<unsafe extern "C" fn(i32)>(%%I0%%);
    if read_u32(font_files::SCANNED) == 0 {
        return original(mode);
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(G_FONT_MODE, 4)], || original(mode), || font_set_mode(mode));
    SCOREBOARD.record(equal, || format!("font_set_mode({}): {:?}", mode, first_differences(&mo, &mr)));
}

/// Comparer for `font_get_file_count` (only the original before the scan, as `compare_font_set_mode`).
pub(crate) unsafe extern "C" fn compare_font_get_file_count() -> u32 {
    let original = original_fn::<unsafe extern "C" fn() -> u32>(%%I1%%);
    if read_u32(font_files::SCANNED) == 0 {
        return original();
    }
    let (o, r) = (original(), font_get_file_count());
    SCOREBOARD.record(o == r, || format!("font_get_file_count: original {} copy {}", o, r));
    o
}

/// Comparer for `font_get_mode_name`: side by side on the buffer (only the original before the scan or with an
/// unusual size).
pub(crate) unsafe extern "C" fn compare_font_get_mode_name(mode: i32, buffer: usize, size: i32) {
    let original = original_fn::<unsafe extern "C" fn(i32, usize, i32)>(%%I2%%);
    if read_u32(font_files::SCANNED) == 0 || !(1..=0x1000).contains(&size) {
        return original(mode, buffer, size);
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(buffer, size as usize)], || original(mode, buffer, size),
        || font_get_mode_name(mode, buffer, size));
    SCOREBOARD.record(equal, || format!("font_get_mode_name({}) (options screen font name): {:?}", mode, first_differences(&mo, &mr)));
}

/// Comparer for `font_scan_folder`: it allocates the table and duplicates the names; only the original runs (its
/// helpers are compared on their own as it calls them). The copy is proven in Rust mode (the font list in the
/// preferences).
pub(crate) unsafe extern "C" fn compare_font_scan_folder() -> u32 {
    original_fn::<unsafe extern "C" fn() -> u32>(%%I3%%)()
}

/// Comparer for `ttf_swap_table_record`: side by side on the 16 bytes.
pub(crate) unsafe extern "thiscall" fn compare_ttf_swap_table_record(record: usize) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I4%%);
    let (o, r, equal, mo, mr) = crate::comparison::side_by_side(&[(record, 16)], || original(record), || ttf_swap_table_record(record));
    SCOREBOARD.record(equal && o == r, || format!("ttf_swap_table_record: {:?}", first_differences(&mo, &mr)));
    o
}

/// Comparer for `ttf_swap_name_table_header`: side by side on the 6 bytes.
pub(crate) unsafe extern "thiscall" fn compare_ttf_swap_name_table_header(header: usize) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I5%%);
    let (o, r, equal, mo, mr) = crate::comparison::side_by_side(&[(header, 6)], || original(header), || ttf_swap_name_table_header(header));
    SCOREBOARD.record(equal && o == r, || format!("ttf_swap_name_table_header: {:?}", first_differences(&mo, &mr)));
    o
}

/// Comparer for `ttf_swap_name_record`: side by side on the 12 bytes.
pub(crate) unsafe extern "thiscall" fn compare_ttf_swap_name_record(record: usize) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I6%%);
    let (o, r, equal, mo, mr) = crate::comparison::side_by_side(&[(record, 12)], || original(record), || ttf_swap_name_record(record));
    SCOREBOARD.record(equal && o == r, || format!("ttf_swap_name_record: {:?}", first_differences(&mo, &mr)));
    o
}

/// Comparer for `ttf_read_name_string`: side by side on the buffer (both seek to the same absolute offset, so the
/// file is left where the original left it); only the original with an unusual size.
pub(crate) unsafe extern "C" fn compare_ttf_read_name_string(file: usize, record: usize, name_id: i32, storage: u32, buffer: usize, size: i32) -> u32 {
    let original = original_fn::<unsafe extern "C" fn(usize, usize, i32, u32, usize, i32) -> u32>(%%I7%%);
    if !(1..=0x1000).contains(&size) {
        return original(file, record, name_id, storage, buffer, size);
    }
    let (o, r, equal, mo, mr) = crate::comparison::side_by_side(&[(buffer, size as usize)],
        || original(file, record, name_id, storage, buffer, size), || ttf_read_name_string(file, record, name_id, storage, buffer, size));
    SCOREBOARD.record(equal && o == r, || format!("ttf_read_name_string(id {}): original {} copy {} {:?}", read_u16(record + 6), o, r, first_differences(&mo, &mr)));
    o
}

/// Comparer for `font_file_read_at`: side by side on the block (an absolute seek each time).
pub(crate) unsafe extern "C" fn compare_font_file_read_at(file: usize, offset: u32, buffer: usize, size: u32) -> u32 {
    let original = original_fn::<unsafe extern "C" fn(usize, u32, usize, u32) -> u32>(%%I8%%);
    if !(1..=0x1000).contains(&size) {
        return original(file, offset, buffer, size);
    }
    let (o, r, equal, mo, mr) = crate::comparison::side_by_side(&[(buffer, size as usize)],
        || original(file, offset, buffer, size), || font_file_read_at(file, offset, buffer, size));
    SCOREBOARD.record(equal && o == r, || format!("font_file_read_at({:#x}, {}): original {} copy {} {:?}", offset, size, o, r, first_differences(&mo, &mr)));
    o
}
