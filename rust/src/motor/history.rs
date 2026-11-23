use super::*;

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

/// Comparer for `history_load_his_file`: the copy reads first, its result is kept as a deep copy and freed, then the
/// original reads from the same state and both are compared by content.
///
/// The routine allocates the sections, paragraphs and pictures it leaves in the screen parameters (and parameter 1
/// takes a copy of the title): the copy's blocks are freed after the digest (`his_file_digest`) and the parameter
/// block, the two static buffers and the pushed-back character are put back before the original runs. A file that
/// does not open: only the original (the box).
pub(crate) unsafe extern "C" fn compare_history_load_his_file(name: usize, last: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> i32>(IDX_HISTORY_LOAD_HIS_FILE);
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
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_HISTORY_HIS_FILE_EXISTS);
    let regions = [(HISTORY_FILE_TEXT, his_file::TEXT_SIZE), (his_file::PUSHBACK, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(name), || history_his_file_exists(name));
    SCOREBOARD.record(equal && ro == rr, || format!("history_his_file_exists (history menus) original={} rust={}, differences in {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `history_get_his_file_category`: side by side over the name buffer and the pushed-back character.
pub(crate) unsafe extern "C" fn compare_history_get_his_file_category(name: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> i32>(IDX_HISTORY_GET_HIS_FILE_CATEGORY);
    let regions = [(HISTORY_FILE_TEXT, his_file::TEXT_SIZE), (his_file::PUSHBACK, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(name), || history_get_his_file_category(name));
    SCOREBOARD.record(equal && ro == rr, || format!("history_get_his_file_category (written history screen) original={} rust={}, differences in {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `history_get_his_file_title`: side by side over the two static buffers and the pushed-back character.
pub(crate) unsafe extern "C" fn compare_history_get_his_file_title(name: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_HISTORY_GET_HIS_FILE_TITLE);
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
    original_fn::<unsafe extern "C" fn()>(IDX_VERIFY_HISTORY_FILES)()
}

/// Rule: the history screen opens on a .his file with the default frame (0x91, 0x4b)-(0x2e9, 0x20f), the text column
/// 0x115 from the left, the picture 0x101 × 0x14f and the pictures under "cm3_history".
///
/// `history_screen_open(name)`: `open_screen` with the builder 0x5f1360, the handler 0x5f1ba0, single instance, the
/// leave routine 0x5f1d60 and 0; when it opens, the parameters: 0 = -1 (no file list), 1..7 = 0, 8..0xb = -1, 0xc =
/// a copy of the name (`set_screen_param` with copy 1), 0xd = -1, 0xe..0x12 = 0x91, 0x4b, 0x2e9, 0x20f, 0x115, 0x13 =
/// "cm3_history", 0x14 = 0x101 and 0x15 = 0x14f.
/// Original: `history_screen_open` (0x5f11c0, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_screen_open(name: usize) {
    covered!("tel35_2 open history");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5f1360usize, 0x5f1ba0usize, 1, 0x5f1d60usize, 0) == 0 {
        return;
    }
    set_screen_param(0, usize::MAX);
    for k in 1..8 {
        set_screen_param(k, 0);
    }
    for k in 8..0xc {
        set_screen_param(k, usize::MAX);
    }
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xc, name, 1);
    for (k, v) in [(0xd, usize::MAX), (0xe, 0x91), (0xf, 0x4b), (0x10, 0x2e9), (0x11, 0x20f), (0x12, 0x115), (0x13, 0x9cec18),
        (0x14, 0x101), (0x15, 0x14f)] {
        set_screen_param(k, v);
    }
}

/// Rule: the history screen shows a .his file: its title, the record chooser (when the file has more than one), the
/// record's picture and its text word-wrapped to the text column, with an Ok button.
///
/// `history_screen_draw()`: the frame (parameters 0xe..0x11), the text column's left offset (0x12), the file list
/// (0xc) and its count (0xd); the text width = the frame's width − the offset − 0x34. The background picture 0. With a
/// file index (parameter 0) neither in the list nor -1 with no list (-1): the History.cpp 180 box, nothing more. With no
/// file loaded (parameter 1 empty): `history_load_his_file` of "<name>.his" (no list) or of the list's entry, 0; when it
/// fails nothing is built. The full-screen black area (799 × 599; 1919 × 1079 in the 1080 exe, read from it), the frame
/// area in the colour (0, 0x30, 0x70), the title (parameter 1, font 4) from 10 inside the frame, 0x2d high. The record
/// (parameter 6 of the records in parameter 2, `HISTORY_RECORD_SIZE` each): its first entry's picture index written to
/// parameter 8; the picture's frame (yellow) and the picture item 0x400 (the picture of that index under parameter
/// 0x13, or the default picture (parameter 7) when the index is -1 or its file does not exist; an index below -1 is
/// taken without the check); with more than one record (parameter 3), the record's name (font 3) at the text column,
/// the chooser button and its popup area (0x200030, colour (0, 0x80, 0xff)) with one item per record (id 2, data k),
/// and the text 0x2c lower. Two frame areas of the text column (8 and 0x62) and the text area (1, grey) 10 inside,
/// written to parameter 9 (the picture item to 0xa). Each entry's text is cut into lines that fit the width (font 2;
/// a line ends at the last space before the width, the following spaces skipped; the text is cut in place and put
/// back), one item per line with the entry's picture index as data (the first line's item written to parameter 0xb
/// when it is -1), and a blank item after each entry. Last the "Ok" button (id 1).
/// Original: `history_screen_draw` (0x5f1360, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_screen_draw() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let x1 = get_screen_param(0xe) as i32;
    let y1 = get_screen_param(0xf) as i32;
    let x2 = get_screen_param(0x10) as i32;
    let y2 = get_screen_param(0x11) as i32;
    let indent = get_screen_param(0x12) as i32;
    let files = get_screen_param(0xc);
    let count = get_screen_param(0xd) as i32;
    let width = x2 - indent - x1 - 0x34;
    exe::background_set_picture(0usize);
    let p0 = get_screen_param(0) as i32;
    if !((p0 >= 0 && p0 < count) || (p0 == -1 && count == -1)) {
        tel26_error_box(HISTORY_CPP, 0xb4);
        return;
    }
    if get_screen_param(1) == 0 {
        let loaded = if get_screen_param(0) as i32 == -1 {
            format(HISTORY_FILE_TEXT, 0x9cec50, get_screen_param(0xc));
            exe::history_load_his_file(HISTORY_FILE_TEXT, 0)
        } else {
            exe::history_load_his_file(read_usize(files.wrapping_add(get_screen_param(0).wrapping_mul(4))), 0)
        };
        if loaded == 0 {
            return;
        }
    }
    covered!("tel35_2 draw history");
    let black = tel35_colour(G_GUI_COLOUR_BLACK);
    let pale = tel35_colour(G_GUI_COLOUR_PALE_GREY);
    let yellow = tel35_colour(G_GUI_COLOUR_YELLOW);
    tel26_add_area(current_human_display(), [0, 0, exe_immediate(0x5f14d0 as *const i32) as u32, exe_immediate(0x5f14c8 as *const i32) as u32,
        1, 0, 1, 0, 2, black, u32::MAX]);
    let colour = exe::graphics_rgb_to_pixel(0, 0x30, 0x70, 0);
    tel26_add_area(current_human_display(), [x1 as u32, y1 as u32, x2 as u32, y2 as u32, 1, 0, 1, 0, 0x30, colour, u32::MAX]);
    let (left, right, top) = (x1 + 0xa, x2 - 0xa, y1 + 0xa);
    tel35_text_item([left as u32, top as u32, right as u32, (top + 0x2d) as u32, 0x30, black, 0xc, 4, pale, get_screen_param(1) as u32, 1]);
    let mut y = top + 0x37;
    let records = get_screen_param(2);
    let record = records.wrapping_add((get_screen_param(6) as u32).wrapping_mul(HISTORY_RECORD_SIZE as u32) as usize);
    let first = read_i32(read_usize(record) + 4);
    set_screen_param(8, first as usize);
    let pic_x2 = get_screen_param(0x14) as i32 + x1 + 0xc;
    let pic_y2 = get_screen_param(0x15) as i32 + y + 2;
    tel35_text_item([left as u32, y as u32, pic_x2 as u32, pic_y2 as u32, 0x61, colour, 1, 2, yellow, HISTORY_EMPTY as u32, 1]);
    let picture = if first == -1 {
        get_screen_param(7)
    } else if first < -1 {
        history_picture_name(first)
    } else if exe::utils_file_exists(exe::utils_build_file_path(get_screen_param(0x13), history_picture_name(first))) != 0 {
        history_picture_name(first)
    } else {
        get_screen_param(7)
    };
    let path = exe::utils_build_file_path(get_screen_param(0x13), picture);
    let picture_item = tel26_add_item(current_human_display(), [0x400, (x1 + 0xc) as u32, (y + 2) as u32, pic_x2 as u32, pic_y2 as u32, 0, 0, 1,
        black, black, 0xc, 2, pale, path as u32, 1, 0, 0, u32::MAX]);
    if get_screen_param(3) as i32 > 1 {
        covered!("tel35_2 history record chooser");
        let x = indent + x1;
        tel35_text_item([x as u32, y as u32, right as u32, (y + 0x22) as u32, 0x32, colour, 0xc, 3, yellow, (record + 8) as u32, 1]);
        let popup_colour = exe::graphics_rgb_to_pixel(0, 0x80, 0xff, 0);
        let button = tel26_add_button([(x + 5) as u32, (y + 5) as u32, (x + 0x14) as u32, (y + 0x19) as u32, 0x20, black, 0xc, 2, yellow,
            HISTORY_EMPTY as u32, 0, 0]);
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x200030, popup_colour, button]);
        let mut k = 0i32;
        while k < get_screen_param(3) as i32 {
            let name = records.wrapping_add(8).wrapping_add((k as u32).wrapping_mul(HISTORY_RECORD_SIZE as u32) as usize);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x90, popup_colour, yellow, 1, 1, black, name as u32, 0, 2, k as u32, popup]);
            k += 1;
        }
        y += 0x2c;
    }
    let x = x1 + indent;
    let bottom = y2 - 0x32;
    tel26_add_area(current_human_display(), [x as u32, y as u32, right as u32, bottom as u32, 1, 0, 0xf, 0, 8, colour, u32::MAX]);
    tel26_add_area(current_human_display(), [x as u32, y as u32, right as u32, bottom as u32, 1, 0, 0xf, 0, 0x62, colour, u32::MAX]);
    let area = tel26_add_area(current_human_display(), [(x + 0xa) as u32, (y + 0xa) as u32, right as u32, (y2 - 0x3c) as u32, 1, 0, 0xf, 0, 1,
        tel35_colour(G_GUI_COLOUR_GREY), u32::MAX]);
    set_screen_param(9, area as i16 as isize as usize);
    set_screen_param(0xa, picture_item as i16 as isize as usize);
    let mut row = 0u32;
    let mut k = 0usize;
    while (k as i32) < read_i32(record + 4) {
        let mut s = read_usize(read_usize(record) + k * 8) as *mut u8;
        if *s != 0 {
            let mut c = *s;
            loop {
                let mut i = 0isize;
                let mut b = c;
                loop {
                    *s.offset(i) = b;
                    b = *s.offset(i + 1);
                    i += 1;
                    *s.offset(i) = 0;
                    if !(exe::font_text_width(2usize, s) < width && b != 0) {
                        break;
                    }
                }
                *s.offset(i) = b;
                while b != b' ' && b != 0 {
                    i -= 1;
                    b = *s.offset(i);
                }
                b = *s.offset(i);
                *s.offset(i) = 0;
                let value = read_u32(read_usize(record) + k * 8 + 4);
                let line = tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 0, row, 1, black, 0, 1, 2, pale, s as u32, 0, 0, value, area]);
                row += 1;
                if get_screen_param(0xb) as i32 == -1 {
                    set_screen_param(0xb, line as i16 as isize as usize);
                }
                *s.offset(i) = b;
                if b == b' ' {
                    loop {
                        let a = *s.offset(i + 1);
                        i += 1;
                        if a != b' ' {
                            break;
                        }
                    }
                }
                let a = *s.offset(i);
                s = s.offset(i);
                if a == 0 {
                    break;
                }
                c = a;
            }
        }
        let value = read_u32(read_usize(record) + k * 8 + 4);
        tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 0, row, 1, black, 0, 1, 2, pale, HISTORY_EMPTY as u32, 0, 0, value, area]);
        row += 1;
        k += 1;
    }
    text(HISTORY_FILE_TEXT, 0x990f7c);
    tel26_add_button([left as u32, (y2 - 0x28) as u32, right as u32, (y2 - 0xa) as u32, 0x30, tel35_colour(G_GUI_COLOUR_GREY), 0xc, 4, pale,
        HISTORY_FILE_TEXT as u32, 1, 0]);
}

/// Rule: scrolling the history text shows the picture of the first visible line's entry; Ok closes, a record of the
/// chooser shows that record.
///
/// `history_screen_handle_event(item)`: with no file loaded (parameter 1) -9. The first visible widget of the text
/// (the scroll row of the text area, parameter 9, + the first line's widget, parameter 0xb, + 7): its data (0 for
/// -1) differing from parameter 8 becomes parameter 8, and the picture item (parameter 0xa) gets that picture's path
/// (under parameter 0x13; the default picture, parameter 7, for -1); redraw. Then the item: -1 gives 0; id 1 (Ok) -9;
/// id 2 (a record) writes its data to parameter 6 and gives -11; others 0.
/// Original: `history_screen_handle_event` (0x5f1ba0, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_screen_handle_event(raw: u32) -> i32 {
    if get_screen_param(1) == 0 {
        return -9;
    }
    let scroll = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(9)) as u32;
    let shown = scroll.wrapping_add(get_screen_param(0xb) as u32).wrapping_add(7) as i16;
    if tel26_item_value(shown) != get_screen_param(8) as u32 {
        covered!("tel35_2 history picture follows");
        set_screen_param(8, tel26_item_value(shown) as usize);
        let path = exe::utils_build_file_path(get_screen_param(0x13), history_picture_name(get_screen_param(8) as i32));
        exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, get_screen_param(0xa) as u32, path);
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
    }
    let item = raw as i16;
    if item == -1 {
        return 0;
    }
    match tel26_item_id(item) {
        1 => -9,
        2 => {
            set_screen_param(6, tel26_item_value(item) as usize);
            -11
        }
        _ => 0,
    }
}

/// Rule: leaving the history screen frees the loaded file: the picture table, and each record's entry texts, entry
/// list and the record table.
///
/// `history_screen_free()`: parameter 1 = 0; the picture table (parameter 4) freed and cleared when there is one;
/// parameter 7 = 0; with a record table (parameter 2): for each of the parameter 3 records whose entry list is not
/// null, every non-null entry text freed, then the list; then the table, and parameters 2 and 3 = 0.
/// Original: `history_screen_free` (0x5f1d60, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_screen_free() {
    set_screen_param(1, 0);
    if get_screen_param(4) != 0 {
        exe::crt_free(get_screen_param(4));
        set_screen_param(4, 0);
    }
    set_screen_param(7, 0);
    let records = get_screen_param(2);
    let count = get_screen_param(3) as i32;
    if records == 0 {
        return;
    }
    covered!("tel35_2 history free");
    for r in 0..count.max(0) as usize {
        let record = records + r * HISTORY_RECORD_SIZE;
        if read_usize(record) == 0 {
            continue;
        }
        let mut j = 0usize;
        while (j as i32) < read_i32(record + 4) {
            let t = read_usize(read_usize(record) + j * 8);
            if t != 0 {
                exe::crt_free(t);
            }
            j += 1;
        }
        exe::crt_free(read_usize(record));
    }
    exe::crt_free(records);
    set_screen_param(2, 0);
    set_screen_param(3, 0);
}

/// Rule: the written history screen opens on a tab with no file list yet.
///
/// `history_written_screen_open(tab)`: `open_screen` with the builder 0x5f2c00, the handler 0x5f31c0, not single, the
/// leave routine 0x5f33d0 and 0; parameter 0 = the tab, 1..4 = 0.
/// Original: `history_written_screen_open` (0x5f2b80, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_written_screen_open(tab: usize) {
    covered!("tel35_2 open written history");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5f2c00usize, 0x5f31c0usize, 0, 0x5f33d0usize, 0) == 0 {
        return;
    }
    set_screen_param(0, tab);
    for k in 1..5 {
        set_screen_param(k, 0);
    }
}

/// Rule: the written history screen lists, under the tabs Nations, Competitions, Leagues, Clubs and Players, the
/// titles of the .his files of the chosen tab's category.
///
/// `history_written_screen_draw()`: ten tab records (0xd4 bytes, built by the runtime's vector constructor with
/// `MenuItem::MenuItem`); the "CM3_HISTORY" folder; the tab (parameter 0), the list (1) and its count (2). The random
/// background picture, the title "History" (dark blue, pale grey), the menu bar (4, 0) and the tab bar of five tabs
/// (Nations 1, Competitions 2, Leagues 3, Clubs 0, Players 4, all kind 1) from 0x50 to 0x221. With no list: a block of
/// 100 file entries (`HISTORY_FILE_ENTRY_SIZE`, `operator new`; null: the History.cpp 1235 box and nothing more); every
/// "*.his" of the folder (`file_find_first`/`file_find_next`) whose name without its extension has the tab's category
/// (`history_get_his_file_category`) and a title (`history_get_his_file_title`) is copied (name 0x104, title 0x12c,
/// each cut at its last byte); a 101st shows the History.cpp 1270 box and the screen stops there (the search left open,
/// the block not kept); the entries sorted by title (qsort with 0x5f3410); parameter 1 = the block and 2 = the count;
/// the search closed. The list area (0x6e, top)-(0x30c, bottom) with (bottom − top − 16) / the height of font 3 rows,
/// one button per entry (" <title>", id 2, the entry as data), scrolled to parameter 3 and written to parameter 4; the
/// Back and Next buttons.
/// Original: `history_written_screen_draw` (0x5f2c00, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_written_screen_draw() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut tabs = [0u32; 0xd4 * 10 / 4];
    let t = tabs.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(t, 0xd4, 0xa, 0x4568f0usize, 0x5a53c0usize);
    let mut folder = [0u8; 0x104];
    tel26_copy_text(folder.as_mut_ptr(), exe::utils_build_file_path(0x9cedb8usize, HISTORY_EMPTY) as *const u8);
    let tab = get_screen_param(0);
    let mut list = get_screen_param(1);
    let mut count = get_screen_param(2) as i32;
    exe::background_set_random_picture();
    text(JOB_TEXT, 0x9914d4, HISTORY_EMPTY);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, tel35_colour(G_GUI_COLOUR_DARK_BLUE), tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT);
    exe::menubar_build(4, 0);
    for (k, (label, link)) in [(0x9c9f64u32, 1u32), (0x98ed18, 2), (0x9cedcc, 3), (0x9cedc4, 0), (0x990a80, 4)].into_iter().enumerate() {
        let entry = t + k * 0xd4;
        text(entry + 4, label as usize);
        write_u32(entry + 0xcc, 1);
        write_u32(entry + 0xd0, link);
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(t, 5, tab as i32, &mut top, &mut bottom, 1);
    if list == 0 {
        covered!("tel35_2 written history file search");
        let block = exe::crt_operator_new(0xdac0);
        count = 0;
        if block == 0 {
            tel26_error_box(HISTORY_CPP, 0x4d3);
            exe::crt_vector_destructor_iterator(t, 0xd4, 0xa, 0x5a53c0usize);
            return;
        }
        let mut pattern = [0u8; 0x104];
        let mut found = [0u8; 0x114];
        exe::crt_makepath(pattern.as_mut_ptr(), 0usize, folder.as_ptr() as usize, 0x989ad8usize, 0x9cedb0usize);
        if exe::file_find_first(pattern.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 {
            let mut entry = block;
            loop {
                tel26_copy_text(pattern.as_mut_ptr(), found.as_ptr().add(0x10));
                let mut i = tel35_c_string_len(pattern.as_ptr()) as isize - 1;
                while i >= 0 {
                    if pattern[i as usize] == b'.' {
                        pattern[i as usize] = 0;
                        break;
                    }
                    i -= 1;
                }
                if tel35_cdecl1(0x5f2580, pattern.as_ptr() as usize) as u32 == tab as u32 {
                    let title = tel35_cdecl1(0x5f2700, pattern.as_ptr() as usize);
                    if title != 0 {
                        if count >= 0x64 {
                            tel26_error_box(HISTORY_CPP, 0x4f6);
                            exe::crt_vector_destructor_iterator(t, 0xd4, 0xa, 0x5a53c0usize);
                            return;
                        }
                        tel35_strncpy(entry, pattern.as_ptr() as usize, 0x104);
                        write_u8(entry + 0x103, 0);
                        tel35_strncpy(entry + 0x104, title, 0x12c);
                        write_u8(entry + 0x104 + 0x12b, 0);
                        count += 1;
                        entry += HISTORY_FILE_ENTRY_SIZE;
                    }
                }
                if exe::file_find_next(found.as_mut_ptr()) != 0 {
                    break;
                }
            }
            if count != 0 {
                exe::crt_qsort(block, count as u32, HISTORY_FILE_ENTRY_SIZE as u32, 0x5f3410usize);
            }
        }
        set_screen_param(1, block);
        set_screen_param(2, count as usize);
        exe::file_find_close(found.as_ptr() as usize);
        list = block;
    }
    let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as u32;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 1, 0, rows, 0, 2, 0, u32::MAX]);
    for k in 0..count.max(0) as usize {
        let entry = list + k * HISTORY_FILE_ENTRY_SIZE;
        format(JOB_TEXT, 0x987ff0, entry + 0x104);
        tel26_area_button([area, 0, k as u32, 1, 0, 1, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 2, entry as u32]);
    }
    let scroll = get_screen_param(3);
    let d = current_human_display();
    if area as i16 != -1 {
        exe::area_set_scroll_row(d.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(0xbf1)), scroll);
    }
    set_screen_param(4, area as i16 as isize as usize);
    tel26_back_next();
    exe::crt_vector_destructor_iterator(t, 0xd4, 0xa, 0x5a53c0usize);
}

/// Rule: on the written history screen a tab click changes the category and drops the list; an entry opens its
/// history screen.
///
/// `history_written_screen_handle_click(item)`: with an item, the list's scroll row (area of parameter 4) is kept in
/// parameter 3. Id 1 (a tab) with a tab other than parameter 0: parameter 0 = that tab, the list (parameter 1) deleted
/// (`operator delete`) and parameters 1 and 2 = 0, -11. Id 2 (an entry): its file name (0x104 bytes, cut at its last
/// byte) opens the history screen (`history_screen_open`), -11. Otherwise (the same tab included) the menu bar's
/// command, then its object link.
/// Original: `history_written_screen_handle_click` (0x5f31c0, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_written_screen_handle_click(raw: u32) -> i32 {
    let tab = get_screen_param(0) as u32;
    let list = get_screen_param(1);
    let item = raw as i16;
    if item != -1 {
        set_screen_param(3, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(4)));
    }
    let id = tel26_item_id(item);
    if id == 1 && tel26_item_value(item) != tab {
        covered!("tel35_2 written history tab");
        set_screen_param(0, tel26_item_value(item) as usize);
        if list != 0 {
            exe::crt_operator_delete(list);
        }
        set_screen_param(1, 0);
        set_screen_param(2, 0);
        return -11;
    }
    if id == 2 {
        covered!("tel35_2 written history entry");
        let mut name = [0u8; 0x104];
        tel35_strncpy(name.as_mut_ptr() as usize, tel26_item_value(item) as usize, 0x104);
        name[0x103] = 0;
        tel35_cdecl1(0x5f11c0, name.as_ptr() as usize);
        return -11;
    }
    manager_menu_bar(raw)
}

/// Rule: leaving the written history screen deletes the file list.
///
/// `history_written_screen_close()`: the list (parameter 1) deleted (`operator delete`) when there is one; parameters
/// 1 and 2 = 0.
/// Original: `history_written_screen_close` (0x5f33d0, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_written_screen_close() {
    let list = get_screen_param(1);
    if list != 0 {
        exe::crt_operator_delete(list);
    }
    set_screen_param(1, 0);
    set_screen_param(2, 0);
}

/// Comparer for `history_screen_open`.
pub(crate) unsafe extern "C" fn compare_history_screen_open(name: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(IDX_HISTORY_SCREEN_OPEN)(name)
}

/// Comparer for `history_screen_draw`: side by side over the display list, the file name buffer and the parameters;
/// loading the file does not run twice (only the original runs then).
pub(crate) unsafe extern "C" fn compare_history_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_HISTORY_SCREEN_DRAW);
    if get_screen_param(1) == 0 {
        // simplification: loading the .his file allocates the records; the first build only in the original
        return o();
    }
    compare_by_display_list_with("History screen (history_screen_draw)", &[(HISTORY_FILE_TEXT, 0x104)], || o(), || history_screen_draw());
}

/// Comparer for `history_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_history_screen_handle_event(item: u32) -> i32 {
    // simplification: the redraw does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_HISTORY_SCREEN_HANDLE_EVENT)(item)
}

/// Comparer for `history_screen_free`.
pub(crate) unsafe extern "C" fn compare_history_screen_free() {
    // simplification: freeing does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_HISTORY_SCREEN_FREE)()
}

/// Comparer for `history_written_screen_open`.
pub(crate) unsafe extern "C" fn compare_history_written_screen_open(tab: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(IDX_HISTORY_WRITTEN_SCREEN_OPEN)(tab)
}

/// Comparer for `history_written_screen_draw`: side by side over the display list when the list exists (the file
/// search allocates it: the first build only in the original).
pub(crate) unsafe extern "C" fn compare_history_written_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_HISTORY_WRITTEN_SCREEN_DRAW);
    if get_screen_param(1) == 0 {
        // simplification: the file search allocates the list; the first build only in the original
        return o();
    }
    compare_by_display_list_with("Written history screen (history_written_screen_draw)", &[], || o(), || history_written_screen_draw());
}

/// Comparer for `history_written_screen_handle_click`.
pub(crate) unsafe extern "C" fn compare_history_written_screen_handle_click(item: u32) -> i32 {
    // simplification: opening a screen and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_HISTORY_WRITTEN_SCREEN_HANDLE_CLICK)(item)
}

/// Comparer for `history_written_screen_close`.
pub(crate) unsafe extern "C" fn compare_history_written_screen_close() {
    // simplification: deleting does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_HISTORY_WRITTEN_SCREEN_CLOSE)()
}

/// The next character of a .his file: the pushed-back one (`his_file::PUSHBACK`, then -1) or `fgetc`.
pub(crate) unsafe fn his_file_getc(file: usize) -> i32 {
    let c = read_i32(his_file::PUSHBACK);
    if c != -1 {
        write_i32(his_file::PUSHBACK, -1);
        return c;
    }
    exe::at_9469e0(file)
}

/// The end-of-file flag of a C runtime FILE (`_flag` +0xc, `_IOEOF` 0x10).
pub(crate) unsafe fn his_file_at_end(file: usize) -> bool {
    read_u8(file + 0xc) & 0x10 != 0
}

/// Opens "cm3_history\<name>.his" for reading ("%s.his" written into `HISTORY_FILE_TEXT`); 0 when missing.
pub(crate) unsafe fn his_file_open(name: usize) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(HISTORY_FILE_TEXT, his_file::NAME_FORMAT, name);
    exe::crt_fopen(exe::utils_build_file_path(his_file::FOLDER, HISTORY_FILE_TEXT), his_file::READ_MODE)
}

/// The value of a "{tag value}" after the tag: the spaces skipped, then up to '}' or the end, into `out` (closed).
pub(crate) unsafe fn his_file_tag_value(file: usize, out: usize) {
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

/// The verification of one .his file name (without its extension) against the database, by category: the message
/// format of the failure, or 0 when it matches.
pub(crate) unsafe fn verify_history_file(name: *const u8) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    match history_get_his_file_category(name as usize) {
        0 | 1 => {
            // a NATION file is checked against the club names too
            for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
                let club = club_at(k as isize).addr();
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
                let person = staff_at(k as isize).addr();
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

/// A deep copy of what `history_load_his_file` leaves in the screen parameters: the return, the title text, each
/// section's name and paragraphs (text and picture) and the picture names.
pub(crate) unsafe fn his_file_digest(ret: i32) -> Vec<u8> {
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

/// `ScreenManager::add_text_item(x1, y1, x2, y2, style, colour, flags, font, text colour, text, 1)` (0x803fd0, thiscall,
/// ret 0x2c).
pub(crate) unsafe fn tel35_text_item(f: [u32; 11]) -> u32 {
    type TextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: TextItem = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// The C runtime's strncpy (0x9466d0, cdecl).
pub(crate) unsafe fn tel35_strncpy(dst: usize, src: usize, n: u32) {
    exe::at_9466d0(dst, (src) as usize as *const u8, n);
}

/// The picture of the history record's selected entry: the picture name (`HISTORY_PICTURE_NAME_SIZE` bytes each from
/// parameter 4) of `index`, or the default picture (parameter 7) for -1.
pub(crate) unsafe fn history_picture_name(index: i32) -> usize {
    if index == -1 {
        get_screen_param(7)
    } else {
        get_screen_param(4).wrapping_add((index as u32).wrapping_mul(HISTORY_PICTURE_NAME_SIZE as u32) as usize)
    }
}

/// The length of a C string (the original's repne scasb).
pub(crate) unsafe fn tel35_c_string_len(s: *const u8) -> usize {
    let mut n = 0;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}

/// The History.cpp path for the error box.
pub(crate) const HISTORY_CPP: usize = 0x9cec24;

/// The history screens' file name buffer (0xb5dedc, also the "Ok" text).
pub(crate) const HISTORY_FILE_TEXT: usize = 0xb5dedc;

/// The empty text (0xad9c64).
pub(crate) const HISTORY_EMPTY: usize = 0xad9c64;

/// The size of a history record (an entry list: +0 the entries, 8 bytes each (text, picture), +4 their count, +8 the
/// record's name) and of a written history file entry (+0 the file name, 0x104 bytes; +0x104 its title, 0x12c bytes).
pub(crate) const HISTORY_RECORD_SIZE: usize = 0x6c;

pub(crate) const HISTORY_FILE_ENTRY_SIZE: usize = 0x230;

/// The size of a picture name in the history's picture table (screen parameter 4).
pub(crate) const HISTORY_PICTURE_NAME_SIZE: usize = 0x104;
