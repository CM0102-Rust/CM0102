// rx04_3: loading a language from language.ldb, the library's setup and shutdown (Langlib.cpp).

/// "Setup not yet called".
pub(crate) const LANGLIB_SETUP_NOT_CALLED: usize = 0x9ed828;
/// "File is not a database file".
const LANGLIB_NOT_A_DATABASE: usize = 0x9ed8c0;
/// "Requested language not found in database".
const LANGLIB_LANGUAGE_NOT_FOUND: usize = 0x9ed894;
/// "Unable to allocate memory for language".
const LANGLIB_NO_MEMORY: usize = 0x9ed86c;
/// "Changing Language" (the progress message).
const LANGLIB_CHANGING_LANGUAGE: usize = 0x9ed8fc;
/// The mark language.ldb starts with when it was written big-endian (`utils_read_file_format_marker`).
const LANGLIB_SWAPPED_MARKER: u32 = 0xf1fffff1;
/// The import slot of GetKeyboardLayout (user32).
const IAT_GET_KEYBOARD_LAYOUT: usize = 0x9671d8;
/// `LANGLIB_ENTRY::LANGLIB_ENTRY` and `LANGLIB_ENTRY::~LANGLIB_ENTRY`, given to the C runtime's array helpers.
pub(crate) const LANGLIB_ENTRY_NEW: usize = 0x66f4c0;
pub(crate) const LANGLIB_ENTRY_DROP: usize = 0x66ea10;

/// Reads one dword of language.ldb, byte-swapped when the file is big-endian.
unsafe fn langlib_read_word(file: usize, swap: bool, to: usize) {
    exe::crt_fread(to, 4, 1, file);
    if swap {
        write_u32(to, read_u32(to).swap_bytes());
    }
}

/// Destroys the language table (each entry's strings, then the array) and clears `G_LANGLIB_TABLE`. Returns what
/// `operator delete` leaves in eax, or 0 without a table (`langlib_shutdown` returns it).
unsafe fn langlib_free_table() -> u32 {
    let table = read_usize(G_LANGLIB_TABLE);
    if table == 0 {
        return 0;
    }
    exe::crt_vector_destructor_iterator(table, LANGLIB_ENTRY_SIZE as u32, read_u32(table - 4), LANGLIB_ENTRY_DROP);
    let delete: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let r = delete(table - 4);
    write_usize(G_LANGLIB_TABLE, 0);
    r
}

/// Loads a language from language.ldb into the table, with progress messages when `progress` is set. Returns 1, or
/// 0 on an error.
///
/// Before `langlib_setup` (language -1): the error box of line 0x1ce, "Setup not yet called", 0. Otherwise the old
/// table is destroyed and the language set to -1 while the file is read. The file: a marker (a big-endian file
/// has its dwords swapped), the version (1 or 2, else the box of line 0x1ef, "File is not a database file", 0), the
/// number of texts, the number of languages and their ids; the ids are the available languages
/// (`G_LANGLIB_AVAILABLE`), except 0 and 1 in a version 2 file, and the requested one's position is kept
/// (`G_LANGLIB_FILE_LANGUAGE_INDEX`). Not found: "Requested language not found in database", and the first
/// available language is loaded instead (0 when there is none). Found: it becomes the game language; English (0) or
/// a library that reads the file at each lookup stop here (1). Otherwise a table of one entry per text
/// (`LANGLIB_ENTRY` each, built by the C runtime's array helper; without memory the box of line 0x235, "Unable to
/// allocate memory for language", language -1, 0) is filled record by record: the English key (without its braces,
/// then without spaces and with "\n" as a line break), the argument format, the requested language's translation and
/// 10 piece numbers (the other languages' blocks are skipped). With `progress`, `ScreenManager::
/// send_network_status_message` with "Changing Language" before the first text and after each 250th and the last.
/// The table is sorted by key (`langlib_compare_string_ptrs`). Without the file: the box of line 0x1df, "Unable to
/// open database file", 0.
/// Original: `langlib_change_language` (0x66eb40, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_change_language(language: i32, progress: u32) -> u32 {
    if read_i32(G_LANGLIB_LANGUAGE) == -1 {
        langlib_fail(0x1ce, LANGLIB_SETUP_NOT_CALLED);
        return 0;
    }
    let mut message = [0u8; 0xc8];
    let msg = message.as_mut_ptr() as usize;
    if progress != 0 {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
        sprintf(msg, LANGLIB_CHANGING_LANGUAGE);
    }
    langlib_free_table();
    write_i32(G_LANGLIB_LANGUAGE, -1);
    let file = exe::crt_fopen(G_LANGLIB_DATABASE_PATH, LANGLIB_READ_MODE);
    if file == 0 {
        langlib_fail(0x1df, LANGLIB_UNABLE_TO_OPEN);
        return 0;
    }
    let swap = exe::utils_read_file_format_marker(file) == LANGLIB_SWAPPED_MARKER;
    let mut version = 0u32;
    langlib_read_word(file, swap, &mut version as *mut u32 as usize);
    if version != 1 && version != 2 {
        langlib_fail(0x1ef, LANGLIB_NOT_A_DATABASE);
        exe::crt_fclose(file);
        return 0;
    }
    exe::crt_fread(G_LANGLIB_ENTRY_COUNT, 4, 1, file);
    exe::crt_fread(G_LANGLIB_FILE_LANGUAGE_COUNT, 4, 1, file);
    if swap {
        write_u32(G_LANGLIB_ENTRY_COUNT, read_u32(G_LANGLIB_ENTRY_COUNT).swap_bytes());
        write_u32(G_LANGLIB_FILE_LANGUAGE_COUNT, read_u32(G_LANGLIB_FILE_LANGUAGE_COUNT).swap_bytes());
    }
    write_i32(G_LANGLIB_FILE_LANGUAGE_INDEX, -1);
    write_u32(G_LANGLIB_AVAILABLE_COUNT, 0);
    let mut i = 0;
    while i < read_i32(G_LANGLIB_FILE_LANGUAGE_COUNT) {
        let mut id = 0i32;
        langlib_read_word(file, swap, &mut id as *mut i32 as usize);
        let count = read_u32(G_LANGLIB_AVAILABLE_COUNT) as usize;
        write_i32(G_LANGLIB_AVAILABLE + 4 * count, id);
        if !(version == 2 && (id == 0 || id == 1)) {
            if id == language {
                write_i32(G_LANGLIB_FILE_LANGUAGE_INDEX, i);
            }
            write_u32(G_LANGLIB_AVAILABLE_COUNT, count as u32 + 1);
        }
        i += 1;
    }
    if read_i32(G_LANGLIB_FILE_LANGUAGE_INDEX) == -1 {
        covered!("rx04_3 language not in the file");
        copy_c_string(G_LANGLIB_LAST_ERROR, LANGLIB_LANGUAGE_NOT_FOUND);
        exe::crt_fclose(file);
        if read_u32(G_LANGLIB_AVAILABLE_COUNT) == 0 {
            return 0;
        }
        let first = read_i32(G_LANGLIB_AVAILABLE);
        write_i32(G_LANGLIB_LANGUAGE, first);
        return langlib_change_language(first, progress);
    }
    write_i32(G_LANGLIB_LANGUAGE, language);
    if read_u32(G_LANGLIB_LOAD_IN_MEMORY) != 1 || language == 0 {
        covered!("rx04_3 language without a table");
        exe::crt_fclose(file);
        return 1;
    }
    covered!("rx04_3 language table");
    if !langlib_new_table() {
        langlib_fail(0x235, LANGLIB_NO_MEMORY);
        langlib_free_table();
        write_i32(G_LANGLIB_LANGUAGE, -1);
        exe::crt_fclose(file);
        return 0;
    }
    let count = read_u32(G_LANGLIB_ENTRY_COUNT);
    if progress != 0 {
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, msg, 0, count, -1);
    }
    let mut k = 0i32;
    while k < read_i32(G_LANGLIB_ENTRY_COUNT) {
        langlib_read_entry(file, read_usize(G_LANGLIB_TABLE) + k as usize * LANGLIB_ENTRY_SIZE, swap);
        let count = read_i32(G_LANGLIB_ENTRY_COUNT);
        if progress != 0 && (k % 250 == 0 || k == count - 1) {
            exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, msg, k + 1, count as u32, -1);
        }
        k += 1;
    }
    exe::crt_qsort(read_usize(G_LANGLIB_TABLE), read_u32(G_LANGLIB_ENTRY_COUNT), LANGLIB_ENTRY_SIZE as u32, LANGLIB_KEY_ORDER);
    exe::crt_fclose(file);
    1
}

/// A new table of `G_LANGLIB_ENTRY_COUNT` empty entries (the count in the dword before it) in `G_LANGLIB_TABLE`;
/// false without memory (the table then 0).
unsafe fn langlib_new_table() -> bool {
    let n = read_u32(G_LANGLIB_ENTRY_COUNT);
    let block = exe::crt_operator_new(n.wrapping_mul(LANGLIB_ENTRY_SIZE as u32).wrapping_add(4));
    let table = if block != 0 {
        write_u32(block, n);
        exe::crt_vector_constructor_iterator(block + 4, LANGLIB_ENTRY_SIZE as u32, n, LANGLIB_ENTRY_NEW, LANGLIB_ENTRY_DROP);
        block + 4
    } else {
        0
    };
    write_usize(G_LANGLIB_TABLE, table);
    table != 0
}

/// Reads one text record into a table entry: the key (its braces taken out), the argument format, the requested
/// language's translation and piece numbers (the others skipped), then the key without spaces and with "\n" as a
/// line break.
unsafe fn langlib_read_entry(file: usize, e: usize, swap: bool) {
    langlib_read_line(file, LANGLIB_RECORD_TEXT);
    langlib_replace_string(e + LANGLIB_ENTRY_KEY, G_LANGLIB_LINE);
    langlib_read_line(file, LANGLIB_RECORD_ARG_FORMAT);
    langlib_replace_string(e + LANGLIB_ENTRY_ARG_FORMAT, G_LANGLIB_LINE);
    let key = read_usize(e + LANGLIB_ENTRY_KEY);
    let (mut r, mut w) = (0, 0);
    while read_u8(key + r) != 0 {
        let c = read_u8(key + r);
        if c != b'{' && c != b'}' {
            write_u8(key + w, c);
            w += 1;
        }
        r += 1;
    }
    write_u8(key + w, 0);
    let mut l = 1;
    while l < read_i32(G_LANGLIB_FILE_LANGUAGE_INDEX) {
        langlib_skip_language(file);
        l += 1;
    }
    langlib_read_line(file, LANGLIB_RECORD_TEXT);
    langlib_replace_string(e + LANGLIB_ENTRY_TRANSLATION, G_LANGLIB_LINE);
    for k in 0..10 {
        langlib_read_word(file, swap, e + LANGLIB_ENTRY_ORDER + 4 * k);
    }
    let mut l = read_i32(G_LANGLIB_FILE_LANGUAGE_INDEX) + 1;
    while l < read_i32(G_LANGLIB_FILE_LANGUAGE_COUNT) {
        langlib_skip_language(file);
        l += 1;
    }
    let (mut j, mut n) = (0, 0);
    while read_u8(key + j) != 0 {
        let c = read_u8(key + j);
        if c != b' ' {
            if c == b'\\' && read_u8(key + j + 1) == b'n' {
                write_u8(G_LANGLIB_LINE + n, b'\n');
                j += 1;
            } else {
                write_u8(G_LANGLIB_LINE + n, c);
            }
            n += 1;
        }
        j += 1;
    }
    write_u8(G_LANGLIB_LINE + n, 0);
    langlib_replace_string(e + LANGLIB_ENTRY_KEY, G_LANGLIB_LINE);
}

/// The library's state after a load, as one number: the language, its position, the available languages, the
/// counts, the last error and every table entry (strings and numbers, in order).
unsafe fn langlib_state_digest() -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    let mut eat = |bytes: &[u8]| {
        for &b in bytes {
            h = (h ^ b as u64).wrapping_mul(0x100000001b3);
        }
        h = (h ^ 0xff).wrapping_mul(0x100000001b3);
    };
    for g in [G_LANGLIB_LANGUAGE, G_LANGLIB_FILE_LANGUAGE_INDEX, G_LANGLIB_AVAILABLE_COUNT, G_LANGLIB_ENTRY_COUNT, G_LANGLIB_FILE_LANGUAGE_COUNT,
        G_LANGLIB_LOAD_IN_MEMORY] {
        eat(&read_u32(g).to_le_bytes());
    }
    for k in 0..(read_u32(G_LANGLIB_AVAILABLE_COUNT) as usize).min(16) {
        eat(&read_u32(G_LANGLIB_AVAILABLE + 4 * k).to_le_bytes());
    }
    eat(&c_string_bytes(G_LANGLIB_LAST_ERROR));
    eat(&c_string_bytes(G_LANGLIB_DATABASE_PATH));
    let table = read_usize(G_LANGLIB_TABLE);
    if table != 0 {
        for k in 0..read_u32(G_LANGLIB_ENTRY_COUNT) as usize {
            let (s, n) = langlib_entry_contents(table + k * LANGLIB_ENTRY_SIZE);
            for t in &s {
                eat(t);
            }
            for v in n {
                eat(&v.to_le_bytes());
            }
        }
    }
    h
}

/// Side by side, one after the other: the copy loads first, then the language as it was is restored and the
/// original loads (destroying the copy's table); the result and the whole state (`langlib_state_digest`) must be
/// equal.
pub(crate) unsafe extern "C" fn compare_langlib_change_language(language: i32, progress: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32, u32) -> u32>(%%I0%%);
    let before = read_i32(G_LANGLIB_LANGUAGE);
    let b = langlib_change_language(language, progress);
    let (db, lb) = (langlib_state_digest(), read_i32(G_LANGLIB_LANGUAGE));
    write_i32(G_LANGLIB_LANGUAGE, before);
    let a = o(language, progress);
    let (da, la) = (langlib_state_digest(), read_i32(G_LANGLIB_LANGUAGE));
    SCOREBOARD.record(a == b && da == db, || format!("langlib_change_language({}, {}): original {} language {} state {:#x} rust {} language {} state {:#x}",
        language, progress, a, la, da, b, lb, db));
    a
}

/// The game language a Windows keyboard layout suggests (its primary language and, for English, Spanish and
/// Portuguese, the sublanguage): US English 1, other English 0, French 2, German 3, Italian 4, Spanish 5 (Mexican 18),
/// Danish 6, Dutch 7, Norwegian 8, Portuguese 9 (Brazilian 17), Swedish 10, Japanese 11, Korean 12, Polish 13,
/// Russian 14; any other 0.
fn langlib_language_of_keyboard(layout: u32) -> i32 {
    let sub = layout & 0xfc00;
    match layout & 0x3ff {
        0x06 => 6,
        0x07 => 3,
        0x09 => (sub == 0x400) as i32,
        0x0a => if sub == 0x800 { 18 } else { 5 },
        0x0c => 2,
        0x10 => 4,
        0x11 => 11,
        0x12 => 12,
        0x13 => 7,
        0x14 => 8,
        0x15 => 13,
        0x16 => if sub == 0x400 { 17 } else { 9 },
        0x19 => 14,
        0x1d => 10,
        _ => 0,
    }
}

/// Sets up the language library: the path of language.ldb and whether the whole language is loaded into memory
/// (`load_in_memory` 1, what the game passes), then the language the keyboard layout suggests
/// (`langlib_language_of_keyboard`) is set and loaded without progress messages. Returns the load's result.
/// Original: `langlib_setup` (0x66e790, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_setup(path: usize, load_in_memory: u32) -> u32 {
    covered!("rx04_3 setup");
    write_u32(G_LANGLIB_LOAD_IN_MEMORY, load_in_memory);
    copy_c_string(G_LANGLIB_DATABASE_PATH, path);
    let keyboard: unsafe extern "system" fn(u32) -> usize = core::mem::transmute(read_usize(IAT_GET_KEYBOARD_LAYOUT));
    let language = langlib_language_of_keyboard(keyboard(0) as u32 & 0xffff);
    write_i32(G_LANGLIB_LANGUAGE, language);
    langlib_change_language(language, 0)
}

/// Side by side, one after the other, as `compare_langlib_change_language`: the copy, the language restored, the
/// original; the result and the state must be equal.
pub(crate) unsafe extern "C" fn compare_langlib_setup(path: usize, load_in_memory: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(%%I1%%);
    let before = read_i32(G_LANGLIB_LANGUAGE);
    let b = langlib_setup(path, load_in_memory);
    let db = langlib_state_digest();
    write_i32(G_LANGLIB_LANGUAGE, before);
    let a = o(path, load_in_memory);
    let da = langlib_state_digest();
    SCOREBOARD.record(a == b && da == db, || format!("langlib_setup: original {} state {:#x} rust {} state {:#x}", a, da, b, db));
    a
}

/// Shuts the language library down: the table destroyed and the language set to -1. Returns what the table's
/// `operator delete` left in eax, 0 without a table (the caller passes it on).
/// Original: `langlib_shutdown` (0x66e9c0, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_shutdown() -> u32 {
    covered!("rx04_3 shutdown");
    let r = langlib_free_table();
    write_i32(G_LANGLIB_LANGUAGE, -1);
    r
}

/// The copy shuts down (it cannot run after the original: the table would be gone), which must leave no table and
/// language -1; then the original runs on what is left. Returns the copy's result.
pub(crate) unsafe extern "C" fn compare_langlib_shutdown() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(%%I2%%);
    let r = langlib_shutdown();
    let (table, language) = (read_usize(G_LANGLIB_TABLE), read_i32(G_LANGLIB_LANGUAGE));
    SCOREBOARD.record(table == 0 && language == -1, || format!("langlib_shutdown: table {:#x} language {}", table, language));
    o();
    r
}
