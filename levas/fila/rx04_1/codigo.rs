// rx04_1: the language library's state (Langlib.cpp) and the lookup of a translation.

/// The current game language (Langlib.cpp, i32): -1 before `langlib_setup` and after `langlib_shutdown`, 0 English
/// (no table), otherwise the language id of language.ldb (2 French, 3 German, 4 Italian, 9 Portuguese...).
pub(crate) const G_LANGLIB_LANGUAGE: usize = 0x9eb2dc;
/// `langlib_setup`'s flag (u32): 1 loads the whole language into the table, 0 makes every lookup read the file.
pub(crate) const G_LANGLIB_LOAD_IN_MEMORY: usize = 0xb647b0;
/// The number of texts in language.ldb (u32), one table entry each.
pub(crate) const G_LANGLIB_ENTRY_COUNT: usize = 0xb647b4;
/// The ids of the languages language.ldb has (i32 each), `G_LANGLIB_AVAILABLE_COUNT` of them.
pub(crate) const G_LANGLIB_AVAILABLE: usize = 0xb64764;
pub(crate) const G_LANGLIB_AVAILABLE_COUNT: usize = 0xb66170;
/// The number of languages in language.ldb (i32), English included.
pub(crate) const G_LANGLIB_FILE_LANGUAGE_COUNT: usize = 0xb6616c;
/// The position of the current language in language.ldb's list (i32, -1 none): its block in each text record.
pub(crate) const G_LANGLIB_FILE_LANGUAGE_INDEX: usize = 0xb65868;
/// The path of language.ldb (`langlib_setup`).
pub(crate) const G_LANGLIB_DATABASE_PATH: usize = 0xb6586c;
/// The library's last error message.
pub(crate) const G_LANGLIB_LAST_ERROR: usize = 0xb65004;
/// The table of the loaded language: `G_LANGLIB_ENTRY_COUNT` entries of `LANGLIB_ENTRY_SIZE` bytes, sorted by key,
/// with the count in the dword before it (a C++ array).
pub(crate) const G_LANGLIB_TABLE: usize = 0xb679f0;
/// Debug flag (u32): when set, `langlib_sprintf` puts "~" marks instead of the arguments [inference: never set].
pub(crate) const G_LANGLIB_MARK_ONLY: usize = 0xb679f4;
/// The scratch line (0x828 bytes): a record read from language.ldb, the argument pieces of `langlib_sprintf`.
pub(crate) const G_LANGLIB_LINE: usize = 0xb6619c;
/// The 10 pointers to the argument pieces in `G_LANGLIB_LINE` (split at the "~").
pub(crate) const G_LANGLIB_PIECES: usize = 0xb66174;
/// The grammar mark before an argument ("{upper}"), kept until the next space.
pub(crate) const G_LANGLIB_MODIFIER_BEFORE: usize = 0xb669c4;
/// The grammar mark after an argument ("\b{s}").
pub(crate) const G_LANGLIB_MODIFIER_AFTER: usize = 0xb671ec;
/// "E:\dev\CM3\cm3 00-01\si\code\Langlib.cpp" (the error boxes).
pub(crate) const LANGLIB_CPP: usize = 0x9ed840;
/// "rb".
pub(crate) const LANGLIB_READ_MODE: usize = 0x98f928;
/// "Unable to open database file".
pub(crate) const LANGLIB_UNABLE_TO_OPEN: usize = 0x9ed8dc;

/// One text of the language table (0x34 bytes): the English key (no spaces, braces or "\n" escapes), the argument
/// format (the vsprintf format that splits the arguments into pieces, "~" between them), the translation (the pieces
/// are the 0x08 bytes) and the piece number of each 0x08, in order (10 i32).
pub(crate) const LANGLIB_ENTRY_SIZE: usize = 0x34;
pub(crate) const LANGLIB_ENTRY_KEY: usize = 0;
pub(crate) const LANGLIB_ENTRY_ARG_FORMAT: usize = 4;
pub(crate) const LANGLIB_ENTRY_TRANSLATION: usize = 8;
pub(crate) const LANGLIB_ENTRY_ORDER: usize = 0xc;
/// The size of a text and of a translation in a language.ldb record.
pub(crate) const LANGLIB_RECORD_TEXT: u32 = 0x800;
/// The size of the argument format in a language.ldb record.
pub(crate) const LANGLIB_RECORD_ARG_FORMAT: u32 = 0x1e;

/// Replaces the string at `slot` by a copy of `src`, as the library does: the old one deleted first, then the new one
/// (`operator new` of its length + 1) stored and filled.
pub(crate) unsafe fn langlib_replace_string(slot: usize, src: usize) {
    let old = read_usize(slot);
    if old != 0 {
        exe::crt_operator_delete(old);
    }
    let p = exe::crt_operator_new(c_string_len(src) as u32 + 1);
    write_usize(slot, p);
    copy_c_string(p, src);
}

/// Reads `n` bytes of language.ldb into the scratch line.
pub(crate) unsafe fn langlib_read_line(file: usize, n: u32) {
    exe::crt_fread(G_LANGLIB_LINE, 1, n, file);
}

/// Skips the record of one other language: its translation and its 10 piece numbers.
pub(crate) unsafe fn langlib_skip_language(file: usize) {
    langlib_read_line(file, LANGLIB_RECORD_TEXT);
    exe::crt_fread(G_LANGLIB_LINE, 4, 10, file);
}

/// The error box of Langlib.cpp `row` and the message kept as the last error.
pub(crate) unsafe fn langlib_fail(row: u32, message: usize) {
    show_error_box(LANGLIB_CPP, row);
    copy_c_string(G_LANGLIB_LAST_ERROR, message);
}

/// Returns the current game language (`G_LANGLIB_LANGUAGE`).
/// Original: `langlib_get_language` (0x66f4d0, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_get_language() -> i32 {
    covered!("rx04_1 get language");
    read_i32(G_LANGLIB_LANGUAGE)
}

pub(crate) unsafe extern "C" fn compare_langlib_get_language() -> i32 {
    let a = original_fn::<unsafe extern "C" fn() -> i32>(%%I0%%)();
    let b = langlib_get_language();
    SCOREBOARD.record(a == b, || format!("langlib_get_language: original {} rust {}", a, b));
    a
}

/// Compares the strings two pointers point to, as strcmp (-1, 0 or 1): the order of the language table (qsort and
/// bsearch by the key).
/// Original: `langlib_compare_string_ptrs` (0x682060, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_compare_string_ptrs(a: *const usize, b: *const usize) -> i32 {
    covered!("rx04_1 compare keys");
    let (x, y) = (*a, *b);
    let mut i = 0;
    loop {
        let (c, d) = (read_u8(x + i), read_u8(y + i));
        if c != d {
            return if c < d { -1 } else { 1 };
        }
        if c == 0 {
            return 0;
        }
        i += 1;
    }
}

compare_order!(compare_langlib_compare_string_ptrs, langlib_compare_string_ptrs, %%I1%%);

/// The key of a text as the table keeps it: without the braces and the spaces, the "\n" escape as a line break and
/// the '\"' escape as a quote (other backslashes stay).
unsafe fn langlib_normalise_key(text: usize, key: usize) {
    let (mut i, mut n) = (0, 0);
    while read_u8(text + i) != 0 {
        let c = read_u8(text + i);
        match c {
            b'{' | b'}' | b' ' => {}
            b'\\' if read_u8(text + i + 1) == b'n' => {
                write_u8(key + n, b'\n');
                n += 1;
                i += 1;
            }
            b'\\' if read_u8(text + i + 1) == b'"' => {
                write_u8(key + n, b'"');
                n += 1;
                i += 1;
            }
            _ => {
                write_u8(key + n, c);
                n += 1;
            }
        }
        i += 1;
    }
    // the original then trims trailing spaces, which the loop above never keeps
    write_u8(key + n, 0);
}

/// Finds the translation of an English text in the current language and copies it into `entry` (a table entry whose
/// three strings are replaced, and its 10 piece numbers); `index`, when not null, gets the entry's position in the
/// table. Returns 1 when found, else 0.
///
/// The text's key: without braces and spaces, "\n" a line break and '\"' a quote. With the whole language in memory
/// (`G_LANGLIB_LOAD_IN_MEMORY` 1, what the game sets up) a binary search of the table by key (`crt_bsearch` with
/// `langlib_compare_string_ptrs`). Otherwise language.ldb is read text by text: the key read, without spaces and with
/// "\n" a line break, compared with the text's; on a match the argument format, the other languages' blocks skipped,
/// the current language's translation and piece numbers; else the rest of the record skipped. Without the file, the
/// error box of line 0xda6 and "Unable to open database file" as the last error.
/// Defects of the original: the file search keeps the braces and the '\"' of the keys it reads, so texts with them
/// are never found that way, and it does not swap the piece numbers of a big-endian file (`langlib_change_language`
/// does); the game only uses the table (`langlib_setup` is always called with 1).
/// Original: `langlib_lookup_translation` (0x66fbb0, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_lookup_translation(entry: usize, text: usize, index: usize) -> u32 {
    let mut key = [0u8; 0x800];
    let k = key.as_mut_ptr() as usize;
    langlib_normalise_key(text, k);
    if read_u32(G_LANGLIB_LOAD_IN_MEMORY) == 1 {
        langlib_lookup_in_table(entry, k, index)
    } else {
        langlib_lookup_in_file(entry, k)
    }
}

/// The binary search of the table: the key goes in a search entry of its own (its second string, which is what the
/// search compares through), found entries are copied.
unsafe fn langlib_lookup_in_table(entry: usize, key: usize, index: usize) -> u32 {
    covered!("rx04_1 lookup in table");
    let copy = exe::crt_operator_new(c_string_len(key) as u32 + 1);
    copy_c_string(copy, key);
    let table = read_usize(G_LANGLIB_TABLE);
    let found = exe::crt_bsearch(&copy as *const usize as usize, table, read_u32(G_LANGLIB_ENTRY_COUNT), LANGLIB_ENTRY_SIZE as u32,
        LANGLIB_KEY_ORDER);
    let mut r = 0;
    if found != 0 {
        covered!("rx04_1 lookup found");
        langlib_copy_entry(entry, found);
        if index != 0 {
            write_u32(index, ((found - table) / LANGLIB_ENTRY_SIZE) as u32);
        }
        r = 1;
    }
    exe::crt_operator_delete(copy);
    r
}

/// Copies a table entry: its three strings (each replaced in `entry`) and its 10 piece numbers.
pub(crate) unsafe fn langlib_copy_entry(entry: usize, from: usize) {
    for slot in [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT, LANGLIB_ENTRY_TRANSLATION] {
        langlib_replace_string(entry + slot, read_usize(from + slot));
    }
    for k in 0..10 {
        write_u32(entry + LANGLIB_ENTRY_ORDER + 4 * k, read_u32(from + LANGLIB_ENTRY_ORDER + 4 * k));
    }
}

/// The search in language.ldb, record by record (see `langlib_lookup_translation`).
unsafe fn langlib_lookup_in_file(entry: usize, key: usize) -> u32 {
    covered!("rx04_1 lookup in file");
    let file = exe::crt_fopen(G_LANGLIB_DATABASE_PATH, LANGLIB_READ_MODE);
    if file == 0 {
        langlib_fail(0xda6, LANGLIB_UNABLE_TO_OPEN);
        return 0;
    }
    // past the header: the version, the two counts and the language ids
    exe::crt_fseek(file, read_i32(G_LANGLIB_FILE_LANGUAGE_COUNT) * 4 + 0xc, 0);
    let mut found = 0;
    let mut i = 0;
    while i < read_i32(G_LANGLIB_ENTRY_COUNT) && found == 0 {
        langlib_read_line(file, LANGLIB_RECORD_TEXT);
        langlib_replace_string(entry + LANGLIB_ENTRY_KEY, G_LANGLIB_LINE);
        let mut read_key = [0u8; 0x800];
        let r = read_key.as_mut_ptr() as usize;
        let s = read_usize(entry + LANGLIB_ENTRY_KEY);
        let (mut j, mut n) = (0, 0);
        while read_u8(s + j) != 0 {
            let c = read_u8(s + j);
            if c != b' ' {
                if c == b'\\' && read_u8(s + j + 1) == b'n' {
                    write_u8(r + n, b'\n');
                    j += 1;
                } else {
                    write_u8(r + n, c);
                }
                n += 1;
            }
            j += 1;
        }
        write_u8(r + n, 0);
        if c_string_bytes_eq(r, key) {
            langlib_read_line(file, LANGLIB_RECORD_ARG_FORMAT);
            langlib_replace_string(entry + LANGLIB_ENTRY_ARG_FORMAT, G_LANGLIB_LINE);
            let mut l = 1;
            while l < read_i32(G_LANGLIB_FILE_LANGUAGE_INDEX) {
                langlib_skip_language(file);
                l += 1;
            }
            langlib_read_line(file, LANGLIB_RECORD_TEXT);
            langlib_replace_string(entry + LANGLIB_ENTRY_TRANSLATION, G_LANGLIB_LINE);
            for k in 0..10 {
                exe::crt_fread(entry + LANGLIB_ENTRY_ORDER + 4 * k, 4, 1, file);
            }
            let mut l = read_i32(G_LANGLIB_FILE_LANGUAGE_INDEX) + 1;
            while l < read_i32(G_LANGLIB_FILE_LANGUAGE_COUNT) {
                langlib_skip_language(file);
                l += 1;
            }
            found = 1;
        } else {
            // the rest of the record: the argument format and the other languages' blocks
            let n = read_i32(G_LANGLIB_FILE_LANGUAGE_COUNT);
            exe::crt_fseek(file, n.wrapping_mul(0x828).wrapping_sub(0x80a), 1);
        }
        i += 1;
    }
    exe::crt_fclose(file);
    found
}

/// Whether two C strings are equal (strcmp == 0).
pub(crate) unsafe fn c_string_bytes_eq(a: usize, b: usize) -> bool {
    let mut i = 0;
    loop {
        let c = read_u8(a + i);
        if c != read_u8(b + i) {
            return false;
        }
        if c == 0 {
            return true;
        }
        i += 1;
    }
}

/// The three strings and the 10 numbers of an entry, for the comparers.
pub(crate) unsafe fn langlib_entry_contents(e: usize) -> (Vec<Vec<u8>>, Vec<u32>) {
    let s = [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT, LANGLIB_ENTRY_TRANSLATION].iter()
        .map(|&o| { let p = read_usize(e + o); if p == 0 { Vec::new() } else { c_string_bytes(p) } }).collect();
    (s, (0..10).map(|k| read_u32(e + LANGLIB_ENTRY_ORDER + 4 * k)).collect())
}

/// Side by side: the original into the caller's entry, the copy into an entry of its own (empty strings at first,
/// deleted afterwards); the result, the position and the entry's strings and numbers must be equal. Only the table
/// search: the file search reads language.ldb once per text, the copy too.
pub(crate) unsafe extern "C" fn compare_langlib_lookup_translation(entry: usize, text: usize, index: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I2%%);
    if read_u32(G_LANGLIB_LOAD_IN_MEMORY) != 1 {
        return o(entry, text, index);
    }
    let mut mine = [0usize; LANGLIB_ENTRY_SIZE / 4];
    let m = mine.as_mut_ptr() as usize;
    let mut my_index = u32::MAX;
    let b = langlib_lookup_translation(m, text, if index != 0 { &mut my_index as *mut u32 as usize } else { 0 });
    let a = o(entry, text, index);
    let their_index = if index != 0 { read_u32(index) } else { u32::MAX };
    let (ea, eb) = (langlib_entry_contents(entry), langlib_entry_contents(m));
    let same = a == b && (a == 0 || (their_index == my_index && ea == eb));
    SCOREBOARD.record(same, || format!("langlib_lookup_translation({:?}): original {} at {} {:?} rust {} at {} {:?}",
        String::from_utf8_lossy(&c_string_bytes(text)), a, their_index, ea, b, my_index, eb));
    for slot in [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT, LANGLIB_ENTRY_TRANSLATION] {
        if read_usize(m + slot) != 0 {
            exe::crt_operator_delete(read_usize(m + slot));
        }
    }
    a
}

/// `langlib_compare_string_ptrs` (0x682060), the order of the language table, given to the C runtime by address.
pub(crate) const LANGLIB_KEY_ORDER: usize = 0x682060;
