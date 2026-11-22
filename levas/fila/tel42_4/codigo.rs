// ---------- batch %%LEVA%%: the written history files (.his): reading one into the history screen, existence, category, title, the -verifyhist check and the order by title (History.cpp) ----------

/// Constants of the .his readers (History.cpp).
pub(crate) mod his_file {
    /// The character pushed back by the readers (-1 = none), read again before the file.
    pub(crate) const PUSHBACK: usize = 0x9cebe0;
    /// The static title buffer `history_get_his_file_title` returns.
    pub(crate) const TITLE_TEXT: usize = 0xb5e008;
    /// The compared size of the two static buffers (the name / title buffer and the title buffer).
    pub(crate) const TEXT_SIZE: usize = 0x12c;
    /// The five category names ("CLUB", "NATION", "COMP", "LEAGUE", "PLAYER"; pointers from 0x9cebc8).
    pub(crate) const CATEGORY_NAMES: usize = 0x9cebc8;
    pub(crate) const CATEGORY_COUNT: usize = 5;
    /// "cm3_history", "CM3_HISTORY", "%s.his", "%s", "r", "*" and ".his".
    pub(crate) const FOLDER: usize = 0x9cec18;
    pub(crate) const FOLDER_KEY: usize = 0x9cedb8;
    pub(crate) const NAME_FORMAT: usize = 0x9cec50;
    pub(crate) const TOKEN_FORMAT: usize = 0x991944;
    pub(crate) const READ_MODE: usize = 0x9899fc;
    pub(crate) const ANY_NAME: usize = 0x989ad8;
    pub(crate) const EXTENSION: usize = 0x9cedb0;
    /// The tags: "title:", "section:", "pic:", "cat:", "category:".
    pub(crate) const TITLE_TAG: usize = 0x9cec6c;
    pub(crate) const SECTION_TAG: usize = 0x9cec60;
    pub(crate) const PIC_TAG: usize = 0x9cec58;
    pub(crate) const CAT_TAG: usize = 0x9cec8c;
    pub(crate) const CATEGORY_TAG: usize = 0x9cec80;
    /// The history screen's default background ("wcup.hsr", parameter 7).
    pub(crate) const DEFAULT_PICTURE: usize = 0x9cec74;
    /// A section's paragraph: +0 the text (allocated), +4 the picture index (-1 none).
    pub(crate) const PARAGRAPH_SIZE: usize = 8;
    /// `fscanf` (C runtime, cdecl).
    pub(crate) const CRT_FSCANF: usize = 0x9470b8;
}

/// The next character of a .his file: the pushed-back one (`his_file::PUSHBACK`, then -1) or `fgetc`.
unsafe fn his_file_getc(file: usize) -> i32 {
    let c = read_i32(his_file::PUSHBACK);
    if c != -1 {
        write_i32(his_file::PUSHBACK, -1);
        return c;
    }
    exe::at_9469e0(file)
}

/// The end-of-file flag of a C runtime FILE (`_flag` +0xc, `_IOEOF` 0x10).
unsafe fn his_file_at_end(file: usize) -> bool {
    read_u8(file + 0xc) & 0x10 != 0
}

/// Opens "cm3_history\<name>.his" for reading ("%s.his" written into `HISTORY_FILE_TEXT`); 0 when missing.
unsafe fn his_file_open(name: usize) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(HISTORY_FILE_TEXT, his_file::NAME_FORMAT, name);
    exe::crt_fopen(exe::utils_build_file_path(his_file::FOLDER, HISTORY_FILE_TEXT), his_file::READ_MODE)
}

/// The value of a "{tag value}" after the tag: the spaces skipped, then up to '}' or the end, into `out` (closed).
unsafe fn his_file_tag_value(file: usize, out: usize) {
    let mut c = his_file_getc(file);
    while c == b' ' as i32 {
        c = his_file_getc(file);
    }
    let mut i = 0usize;
    write_u8(out, 0);
    while c != b'}' as i32 && c != -1 {
        write_u8(out + i, c as u8);
        i += 1;
        c = his_file_getc(file);
    }
    write_u8(out + i, 0);
}

/// Parses a .his history file into the history screen's title, sections and pictures.
///
/// `history_load_his_file(name, last)`: parameter 7 empty gets "wcup.hsr". The file (parameter 0x13's folder and the
/// name, `utils_build_file_path`, "r") missing: the History.cpp 0x1fe box and 0. Until the end of the file, each
/// character read (the pushed-back one first, `his_file::PUSHBACK`):
/// - '{' and a word (`fscanf` "%s"; tags compared without case, `stricmp`): "title:" the value (spaces skipped, up to
///   '}') into `HISTORY_FILE_TEXT`, parameter 1 a copy of it; "section:" a new section (the table grows by
///   `HISTORY_RECORD_SIZE`, `realloc`; none: the 0x22f box and 0) with no paragraph, its name at +8 up to '}', the
///   current picture back to -1; "pic:" the value without spaces into a new picture name (`HISTORY_PICTURE_NAME_SIZE`
///   each, `realloc`; none: 0x269), which becomes the current picture; any other word: skipped up to '}';
/// - any other character inside a section (before the first one it is lost): that character is lost, then the
///   paragraphs: blanks (space, tab, CR, LF) skipped, '{' or the end pushed back to end the text; otherwise a new
///   paragraph (+0 grows by 8, `realloc`; none: 0x29b) with the current picture, whose text grows by one byte per
///   character (`realloc`; none: 0x2af): a single line break becomes one space (the next line's leading blanks
///   skipped, its first character pushed back), two line breaks or the end close the paragraph (the 0 stored).
/// The file closed; parameters 2 = the sections, 3 = their count, 4 = the pictures, 5 = their count, 6 = the last
/// section when `last` is set, else 0. Returns whether parameter 1 (the title) is set. The error returns leave the file
/// open.
/// Original: `history_load_his_file` (0x5f1e50, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_load_his_file(name: usize, last: u32) -> i32 {
    let fscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(his_file::CRT_FSCANF);
    let (mut section, mut sections, mut pictures, mut picture_count, mut picture) = (-1i32, 0usize, 0usize, 0u32, -1i32);
    if get_screen_param(7) == 0 {
        set_screen_param(7, his_file::DEFAULT_PICTURE);
    }
    let file = exe::crt_fopen(exe::utils_build_file_path(get_screen_param(0x13), name), his_file::READ_MODE);
    if file == 0 {
        show_error_at(HISTORY_CPP, 0x1fe);
        return 0;
    }
    covered!("tel42_4 read a .his file");
    // the original's word buffer is 32 bytes in a 0x238-byte frame: a longer word runs into the error box buffers
    let mut token = [0u8; 0x220];
    let tok = token.as_mut_ptr() as usize;
    while !his_file_at_end(file) {
        let mut c = his_file_getc(file);
        if c == -1 {
            continue;
        }
        if c == b'{' as i32 {
            fscanf(file, his_file::TOKEN_FORMAT, tok);
            if exe::crt_stricmp(tok, his_file::TITLE_TAG) == 0 {
                his_file_tag_value(file, HISTORY_FILE_TEXT);
                exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, HISTORY_FILE_TEXT, 1);
            } else if exe::crt_stricmp(tok, his_file::SECTION_TAG) == 0 {
                section += 1;
                picture = -1;
                sections = exe::crt_realloc(sections, (section as u32 + 1).wrapping_mul(HISTORY_RECORD_SIZE as u32));
                if sections == 0 {
                    show_error_at(HISTORY_CPP, 0x22f);
                    return 0;
                }
                let s = sections + section as usize * HISTORY_RECORD_SIZE;
                write_usize(s, 0);
                write_u32(s + 4, 0);
                // the name has no bound (0x64 bytes before the next section)
                let mut i = 0usize;
                while c != -1 {
                    c = his_file_getc(file);
                    if c != b'}' as i32 && c != -1 {
                        write_u8(s + 8 + i, c as u8);
                        i += 1;
                    } else {
                        write_u8(s + 8 + i, 0);
                    }
                    if c == b'}' as i32 {
                        break;
                    }
                }
            } else if exe::crt_stricmp(tok, his_file::PIC_TAG) == 0 {
                let mut i = 0usize;
                while c != -1 {
                    c = his_file_getc(file);
                    if c == b'}' as i32 || c == -1 {
                        token[i] = 0;
                    } else if c != b' ' as i32 {
                        token[i] = c as u8;
                        i += 1;
                    }
                    if c == b'}' as i32 {
                        break;
                    }
                }
                pictures = exe::crt_realloc(pictures, (picture_count + 1).wrapping_mul(HISTORY_PICTURE_NAME_SIZE as u32));
                if pictures == 0 {
                    show_error_at(HISTORY_CPP, 0x269);
                    return 0;
                }
                tel26_copy_text((pictures + picture_count as usize * HISTORY_PICTURE_NAME_SIZE) as *mut u8, token.as_ptr());
                picture = picture_count as i32; // tel42_4: the current picture
                picture_count += 1;
            } else {
                while c != -1 {
                    c = his_file_getc(file);
                    if c == b'}' as i32 {
                        break;
                    }
                }
            }
        } else if section != -1 {
            // the character that brought us here is lost; then the paragraphs up to '{' or the end
            loop {
                c = his_file_getc(file);
                while matches!(c, 0x20 | 0x9 | 0xa | 0xd) {
                    c = his_file_getc(file);
                }
                if c == b'{' as i32 || c == -1 {
                    write_i32(his_file::PUSHBACK, c);
                    break;
                }
                covered!("tel42_4 .his paragraph");
                let s = sections + section as usize * HISTORY_RECORD_SIZE;
                let n = read_u32(s + 4) as usize;
                let paragraphs = exe::crt_realloc(read_usize(s), (n as u32).wrapping_mul(8).wrapping_add(8));
                if paragraphs == 0 {
                    show_error_at(HISTORY_CPP, 0x29b);
                    return 0;
                }
                write_usize(s, paragraphs);
                let p = paragraphs + n * his_file::PARAGRAPH_SIZE;
                write_usize(p, 0);
                write_i32(p + 4, picture);
                let mut len = 0usize;
                loop {
                    let text = exe::crt_realloc(read_usize(read_usize(s) + n * his_file::PARAGRAPH_SIZE), len as u32 + 1);
                    if text == 0 {
                        show_error_at(HISTORY_CPP, 0x2af);
                        return 0;
                    }
                    write_usize(read_usize(s) + n * his_file::PARAGRAPH_SIZE, text);
                    write_u8(text + len, c as u8);
                    len += 1;
                    if c == 0 {
                        break;
                    }
                    let mut line_break = false;
                    loop {
                        let d = his_file_getc(file);
                        c = if d == -1 {
                            0
                        } else if d == b'\n' as i32 {
                            if !line_break {
                                line_break = true;
                                continue;
                            }
                            0 // tel42_4: two line breaks end the paragraph
                        } else {
                            d
                        };
                        if line_break && matches!(c, 0x20 | 0x9 | 0xd) {
                            continue;
                        }
                        break;
                    }
                    if line_break && c != 0 {
                        write_i32(his_file::PUSHBACK, c);
                        c = b' ' as i32;
                    }
                }
                write_u32(s + 4, n as u32 + 1);
            }
        }
    }
    exe::crt_fclose(file);
    let count = section + 1;
    set_screen_param(2, sections);
    set_screen_param(3, count as usize);
    set_screen_param(4, pictures);
    set_screen_param(5, picture_count as usize);
    set_screen_param(6, if last != 0 { (count - 1) as usize } else { 0 });
    (get_screen_param(1) != 0) as i32
}

/// Whether a named .his file exists in cm3_history, for a single-player game on a local screen only.
///
/// `history_his_file_exists(name)`: the game mode (`G_CONFIG` +8) not 0 or 1, or the current screen slot going over
/// the network (`screen_manager::NETWORK_SLOT`): 0. Otherwise "cm3_history\<name>.his" opened for reading (the name
/// written into `HISTORY_FILE_TEXT`): open, it is closed and 1; missing, 0.
/// Original: `history_his_file_exists` (0x5f2500, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_his_file_exists(name: usize) -> u32 {
    if !matches!(read_u32(read_usize(G_CONFIG) + 8), 0 | 1) {
        return 0;
    }
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize;
    if read_u16(G_SCREEN_MANAGER + screen_manager::NETWORK_SLOT + slot.wrapping_mul(0x300)) != 0 {
        return 0;
    }
    let file = his_file_open(name);
    if file == 0 {
        return 0;
    }
    covered!("tel42_4 .his file exists");
    exe::crt_fclose(file);
    1 // tel42_4: the file is there
}

/// Reads the category of a .his file and returns its index in the category names, or -1.
///
/// `history_get_his_file_category(name)`: "cm3_history\<name>.his" (`his_file_open`) missing: -1. `HISTORY_FILE_TEXT`
/// emptied; until the end of the file, a '{' followed by the word "cat:" or "category:" (without case) gives the value
/// (spaces skipped, up to '}') into `HISTORY_FILE_TEXT` and stops; any other character goes on. The file closed. The
/// text compared without case to "CLUB", "NATION", "COMP", "LEAGUE", "PLAYER": the index of the first equal, else -1
/// (a file with no category compares the empty text).
/// Original: `history_get_his_file_category` (0x5f2580, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_get_his_file_category(name: usize) -> i32 {
    let fscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(his_file::CRT_FSCANF);
    let file = his_file_open(name);
    if file == 0 {
        return -1;
    }
    write_u8(HISTORY_FILE_TEXT, 0);
    // the original's word buffer is 32 bytes at the bottom of its frame: a longer word would run over its stack
    let mut word = [0u8; 0x200];
    while !his_file_at_end(file) {
        let c = his_file_getc(file);
        if c == b'{' as i32 {
            fscanf(file, his_file::TOKEN_FORMAT, word.as_mut_ptr());
            if exe::crt_stricmp(word.as_ptr() as usize, his_file::CAT_TAG) == 0
                || exe::crt_stricmp(word.as_ptr() as usize, his_file::CATEGORY_TAG) == 0
            {
                covered!("tel42_4 .his category");
                his_file_tag_value(file, HISTORY_FILE_TEXT);
                break;
            }
        }
    }
    exe::crt_fclose(file);
    (0..his_file::CATEGORY_COUNT)
        .find(|&k| exe::crt_stricmp(read_usize(his_file::CATEGORY_NAMES + k * 4), HISTORY_FILE_TEXT) == 0) // tel42_4: the category index
        .map_or(-1, |k| k as i32)
}

/// Reads the title of a named .his file into a static buffer and returns it, or 0 when the file has none.
///
/// `history_get_his_file_title(name)`: `his_file::TITLE_TEXT` emptied; "cm3_history\<name>.his" missing: 0. Until the
/// end of the file, a '{' followed by the word "title:" (without case) gives the value (spaces skipped, up to '}') into
/// `his_file::TITLE_TEXT`; the file closed and the buffer returned. No title: the file closed and 0.
/// Original: `history_get_his_file_title` (0x5f2700, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_get_his_file_title(name: usize) -> usize {
    let fscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(his_file::CRT_FSCANF);
    write_u8(his_file::TITLE_TEXT, 0);
    let file = his_file_open(name);
    if file == 0 {
        return 0;
    }
    // the original's word buffer is 32 bytes at the bottom of its frame: a longer word would run over its stack
    let mut word = [0u8; 0x200];
    while !his_file_at_end(file) {
        let c = his_file_getc(file);
        if c == b'{' as i32 {
            fscanf(file, his_file::TOKEN_FORMAT, word.as_mut_ptr());
            if exe::crt_stricmp(word.as_ptr() as usize, his_file::TITLE_TAG) == 0 {
                covered!("tel42_4 .his title");
                his_file_tag_value(file, his_file::TITLE_TEXT);
                exe::crt_fclose(file);
                return his_file::TITLE_TEXT; // tel42_4: the title
            }
        }
    }
    exe::crt_fclose(file);
    0
}

/// The verification of one .his file name (without its extension) against the database, by category: the message
/// format of the failure, or 0 when it matches.
unsafe fn verify_history_file(name: *const u8) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    match history_get_his_file_category(name as usize) {
        0 | 1 => {
            // a NATION file is checked against the club names too
            for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
                let club = read_usize(G_CLUB_TABLE) + k * 0x245;
                if exe::utils_string_compare(name, exe::club_get_english_name(club) as *const u8, 1, 0) == 0 {
                    return 0;
                }
            }
            0x9ced58 // "verify_history_files() : file %s has no matching nation or club"
        }
        2 | 3 => {
            for k in 0..read_i32(G_CLUB_COMP_COUNT).max(0) as usize {
                let comp = read_usize(G_CLUB_COMP_TABLE) + k * 0x6b;
                if exe::utils_string_compare(name, comp_get_english_name(comp) as *const u8, 1, 0) == 0 {
                    return 0;
                }
            }
            0x9ced18 // "verify_history_files() : file %s has no matching competition"
        }
        4 => {
            for k in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
                let person = read_usize(G_STAFF_TABLE) + k * 0x6e;
                let second = match read_usize(person + 8) { 0 => his_file::ANY_NAME, p => p };
                let first = match read_usize(person + 4) { 0 => his_file::ANY_NAME, p => p };
                let birth = person + 0x10;
                let year = read_u16(birth + 2) as u32;
                let month = exe::date_get_month(birth) as i8 as i32 + 1;
                let day = exe::date_get_day_of_month(birth) as i16 as i32;
                // "%s %s %d.%d.%d": the two names and the date of birth
                format(JOB_TEXT, 0x9ced08, first, second, day, month, year);
                if exe::utils_string_compare(name, JOB_TEXT as *const u8, 1, 0) == 0 {
                    return 0;
                }
            }
            0x9cecd0 // "verify_history_files() : file %s has no matching staff"
        }
        _ => 0x9cec94, // "verify_history_files() : file %s has an invalid category"
    }
}

/// Checks every .his file in CM3_HISTORY against the database and reports each one with no match (the -verifyhist
/// option of the command line).
///
/// `verify_history_files()`: the folder (`utils_build_file_path` "CM3_HISTORY"), the pattern "*.his" (`makepath`). A
/// first pass only counts the files (`file_find_first`/`file_find_next`); the second, for each file: the network
/// status message "Verifying Histories..." with the file's number and the count (`send_network_status_message`, -1);
/// the name without its last extension; by its category (`history_get_his_file_category`): CLUB and NATION against
/// the clubs' English names (`club_get_english_name`), COMP and LEAGUE against the competitions'
/// (`comp_get_english_name`), PLAYER against "<first> <second> <day>.<month>.<year>" of each person (a missing name is
/// "*"; the date of birth +0x10), compared by `utils_string_compare` (1, 0); no match, or another category: the
/// message (with the full file name) in an "Error" box. Each search closed (`file_find_close`).
/// Original: `verify_history_files` (0x5f2840, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn verify_history_files() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut folder = [0u8; 0x104];
    tel26_copy_text(folder.as_mut_ptr(), exe::utils_build_file_path(his_file::FOLDER_KEY, HISTORY_EMPTY) as *const u8);
    let mut total = 0u32;
    for pass in 1..=2 {
        let mut pattern = [0u8; 0x104];
        let mut found = [0u8; 0x114];
        exe::crt_makepath(pattern.as_mut_ptr(), 0usize, folder.as_ptr() as usize, his_file::ANY_NAME, his_file::EXTENSION);
        let mut count = 0u32;
        if exe::file_find_first(pattern.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 {
            let mut number = 1i32;
            loop {
                if pass == 2 {
                    covered!("tel42_4 verify a .his file");
                    text(JOB_TEXT, 0x9ced98); // "Verifying Histories..."
                    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, JOB_TEXT, number, total, -1);
                    let mut name = [0u8; 0x104];
                    tel26_copy_text(name.as_mut_ptr(), found.as_ptr().add(0x10));
                    if let Some(dot) = name[..tel35_c_string_len(name.as_ptr())].iter().rposition(|&b| b == b'.') {
                        name[dot] = 0;
                    }
                    let message = verify_history_file(name.as_ptr());
                    if message != 0 { // tel42_4: a file with no match
                        format(JOB_TEXT, message, found.as_ptr().add(0x10));
                        exe::gui_show_message_box(0x9870e0, JOB_TEXT, 0);
                    }
                }
                count += 1;
                number += 1;
                if exe::file_find_next(found.as_mut_ptr()) != 0 {
                    break;
                }
            }
        }
        exe::file_find_close(found.as_ptr() as usize);
        total = count;
    }
}

/// Compares two history file entries by their titles for qsort, a missing entry last.
///
/// `history_compare_entries_by_title(a, b)`: no a: 1; no b: -1; otherwise `utils_string_compare` of the titles (+0x104,
/// 1, 0).
/// Original: `history_compare_entries_by_title` (0x5f3410, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_compare_entries_by_title(a: *const usize, b: *const usize) -> i32 {
    if a.is_null() {
        return 1;
    }
    if b.is_null() {
        return -1;
    }
    exe::utils_string_compare((a as usize + 0x104) as *const u8, (b as usize + 0x104) as *const u8, 1, 0) // tel42_4: by title
}

/// A deep copy of what `history_load_his_file` leaves in the screen parameters: the return, the title text, each
/// section's name and paragraphs (text and picture) and the picture names.
unsafe fn his_file_digest(ret: i32) -> Vec<u8> {
    let mut v = ret.to_le_bytes().to_vec();
    let text = |v: &mut Vec<u8>, p: usize| {
        if p != 0 {
            let n = tel35_c_string_len(p as *const u8);
            v.extend((0..n).map(|i| read_u8(p + i)));
        }
        v.push(0);
    };
    text(&mut v, get_screen_param(1));
    let (sections, count) = (get_screen_param(2), get_screen_param(3) as i32);
    v.extend(count.to_le_bytes());
    for k in 0..count.max(0) as usize {
        let s = sections + k * HISTORY_RECORD_SIZE;
        text(&mut v, s + 8);
        let n = read_u32(s + 4) as usize;
        v.extend((n as u32).to_le_bytes());
        for j in 0..n {
            let p = read_usize(s) + j * his_file::PARAGRAPH_SIZE;
            text(&mut v, read_usize(p));
            v.extend(read_u32(p + 4).to_le_bytes());
        }
    }
    let (pictures, pc) = (get_screen_param(4), get_screen_param(5));
    v.extend((pc as u32).to_le_bytes());
    for k in 0..pc {
        text(&mut v, pictures + k * HISTORY_PICTURE_NAME_SIZE);
    }
    v.extend((get_screen_param(6) as u32).to_le_bytes());
    v
}

/// Comparer for `history_load_his_file`: the copy reads first, its result is kept as a deep copy and freed, then the
/// original reads from the same state and both are compared by content.
///
/// The routine allocates the sections, paragraphs and pictures it leaves in the screen parameters (and parameter 1
/// takes a copy of the title): the copy's blocks are freed after the digest (`his_file_digest`) and the parameter
/// block, the two static buffers and the pushed-back character are put back before the original runs. A file that
/// does not open: only the original (the box).
pub(crate) unsafe extern "C" fn compare_history_load_his_file(name: usize, last: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> i32>(%%I0%%);
    // a value owned by one of the parameters it sets (1..7: the slot's flag at +0x18 + 8n) would be freed by the copy's
    // set and again by the original's; so would a title already in parameter 1: only the original then (the history
    // screen calls it with parameter 1 empty)
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize;
    let block = read_usize(G_SCREEN_MANAGER + screen_manager::PARAM_BLOCK + slot.wrapping_mul(0x300));
    if block == 0 || get_screen_param(1) != 0 || (1..=7).any(|n| read_u32(block + 0x18 + 8 * n) != 0) {
        return o(name, last);
    }
    let probe = exe::crt_fopen(exe::utils_build_file_path(get_screen_param(0x13), name), his_file::READ_MODE);
    if probe == 0 {
        return o(name, last);
    }
    exe::crt_fclose(probe);
    let mut regions = screen_param_regions();
    regions.push((HISTORY_FILE_TEXT, his_file::TEXT_SIZE));
    regions.push((his_file::PUSHBACK, 4));
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let rr = history_load_his_file(name, last);
    let mine = his_file_digest(rr);
    // free what the copy allocated: the title copy (when it is a new one), the texts, the paragraphs, the tables
    let title = get_screen_param(1);
    if title != 0 {
        exe::crt_operator_delete(title);
    }
    let (sections, count) = (get_screen_param(2), get_screen_param(3) as i32);
    for k in 0..count.max(0) as usize {
        let s = sections + k * HISTORY_RECORD_SIZE;
        for j in 0..read_u32(s + 4) as usize {
            exe::crt_free(read_usize(read_usize(s) + j * his_file::PARAGRAPH_SIZE));
        }
        exe::crt_free(read_usize(s));
    }
    exe::crt_free(sections);
    exe::crt_free(get_screen_param(4));
    before.write_back();
    let ro = o(name, last);
    let theirs = his_file_digest(ro);
    SCOREBOARD.record(mine == theirs, || format!("history_load_his_file (history screen): the original read {} bytes of content, the copy {}; first difference at {:?}",
        theirs.len(), mine.len(), theirs.iter().zip(mine.iter()).position(|(a, b)| a != b)));
    ro
}

/// Comparer for `history_his_file_exists`: side by side over the name buffer and the pushed-back character.
pub(crate) unsafe extern "C" fn compare_history_his_file_exists(name: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%);
    let regions = [(HISTORY_FILE_TEXT, his_file::TEXT_SIZE), (his_file::PUSHBACK, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(name), || history_his_file_exists(name));
    SCOREBOARD.record(equal && ro == rr, || format!("history_his_file_exists (history menus) original={} rust={}, differences in {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `history_get_his_file_category`: side by side over the name buffer and the pushed-back character.
pub(crate) unsafe extern "C" fn compare_history_get_his_file_category(name: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> i32>(%%I2%%);
    let regions = [(HISTORY_FILE_TEXT, his_file::TEXT_SIZE), (his_file::PUSHBACK, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(name), || history_get_his_file_category(name));
    SCOREBOARD.record(equal && ro == rr, || format!("history_get_his_file_category (written history screen) original={} rust={}, differences in {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `history_get_his_file_title`: side by side over the two static buffers and the pushed-back character.
pub(crate) unsafe extern "C" fn compare_history_get_his_file_title(name: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I3%%);
    let regions = [(HISTORY_FILE_TEXT, his_file::TEXT_SIZE), (his_file::TITLE_TEXT, his_file::TEXT_SIZE), (his_file::PUSHBACK, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(name), || history_get_his_file_title(name));
    SCOREBOARD.record(equal && ro == rr, || format!("history_get_his_file_title (written history screen) original={:#x} rust={:#x}, differences in {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `verify_history_files`.
///
/// It sends network status messages and shows a box per file with no match: nothing that shows runs twice. Only the
/// original runs; the copy is proven in Rust mode by starting a new game with -verifyhist (the same boxes as the
/// original's).
pub(crate) unsafe extern "C" fn compare_verify_history_files() {
    original_fn::<unsafe extern "C" fn()>(%%I4%%)()
}

compare_order!(compare_history_compare_entries_by_title, history_compare_entries_by_title, %%I5%%);
