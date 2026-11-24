// ---------- sx21_37: the language character table: upper and lower case and the name comparison (utils.cpp) ----------

/// The language character table: three bytes a character, +0 the sort key, +1 the upper-case form, +2 the lower-case
/// form.
const CHARACTER_TABLE: usize = 0xaa2154;

/// The upper-case form of a character from the language table (+1 of its 3-byte entry); control codes (below 0x20) are
/// returned as they came.
///
/// `utils_char_to_upper(c)`: the character byte below 0x20, returned unchanged (in al); otherwise the table's
/// upper-case byte.
/// Original: `utils_char_to_upper` (0x90c2d0, utils.cpp, cdecl, al).
pub(crate) unsafe extern "C" fn utils_char_to_upper(c: u32) -> u32 {
    let b = c as u8;
    if b < 0x20 {
        return c;
    }
    covered!("sx21_37 upper case");
    read_u8(CHARACTER_TABLE + b as usize * 3 + 1) as u32 // sx21_37: the upper-case column
}

/// Comparer for `utils_char_to_upper`: the character (al) of both.
pub(crate) unsafe extern "C" fn compare_utils_char_to_upper(c: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I0%%)(c);
    let m = utils_char_to_upper(c);
    SCOREBOARD.record(r as u8 == m as u8, || format!("utils_char_to_upper({:#x}) original={:#x} rust={:#x}", c as u8, r as u8, m as u8));
    r
}

/// The lower-case form of a character from the language table (+2 of its 3-byte entry); control codes (below 0x20) are
/// returned as they came.
///
/// `utils_char_to_lower(c)`: the character byte below 0x20, returned unchanged (in al); otherwise the table's
/// lower-case byte.
/// Original: `utils_char_to_lower` (0x90c2f0, utils.cpp, cdecl, al).
pub(crate) unsafe extern "C" fn utils_char_to_lower(c: u32) -> u32 {
    let b = c as u8;
    if b < 0x20 {
        return c;
    }
    covered!("sx21_37 lower case");
    read_u8(CHARACTER_TABLE + b as usize * 3 + 2) as u32 // sx21_37: the lower-case column
}

/// Comparer for `utils_char_to_lower`: the character (al) of both.
pub(crate) unsafe extern "C" fn compare_utils_char_to_lower(c: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I1%%)(c);
    let m = utils_char_to_lower(c);
    SCOREBOARD.record(r as u8 == m as u8, || format!("utils_char_to_lower({:#x}) original={:#x} rust={:#x}", c as u8, r as u8, m as u8));
    r
}

/// Compares two strings for name sorting: -1, 0 or 1, by the language's sort keys, optionally ignoring case and up to
/// a length.
///
/// `utils_string_compare(a, b, case_sensitive, length)`. A missing `a`: -1 when `b` exists, else 0; a missing `b`: 1.
/// Character by character while both go on: the bytes, or with the sort keys (+0 of the language table) unless the
/// language (`exe::langlib_get_language`) is 0xa; without `case_sensitive`, both through the lower-case column (+2) of
/// the table. The first difference gives -1 (a's below, unsigned) or 1. With a `length` above 0, reaching it gives 0.
/// When one string ends: `a` going on gives 1, `b` going on -1, both ending 0.
/// Original: `utils_string_compare` (0x90c1d0, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_string_compare(a: *const u8, b: *const u8, case_sensitive: u32, length: u32) -> i32 {
    let keys = exe::langlib_get_language() != 0xa;
    if a.is_null() {
        return if b.is_null() { 0 } else { -1 };
    }
    if b.is_null() {
        return 1;
    }
    let mut i = 0usize;
    if read_u8(a as usize) != 0 {
        loop {
            let (mut x, mut y) = (read_u8(a as usize + i), read_u8(b as usize + i));
            if y == 0 {
                break;
            }
            if keys {
                y = read_u8(CHARACTER_TABLE + y as usize * 3);
                x = read_u8(CHARACTER_TABLE + x as usize * 3);
            }
            if case_sensitive == 0 {
                x = read_u8(CHARACTER_TABLE + x as usize * 3 + 2);
                y = read_u8(CHARACTER_TABLE + y as usize * 3 + 2);
            }
            if x != y {
                covered!("sx21_37 strings differ");
                return if x < y { -1 } else { 1 }; // sx21_37: the first difference
            }
            if length > 0 && i as u32 == length - 1 {
                return 0;
            }
            i += 1;
            if read_u8(a as usize + i) == 0 {
                break;
            }
        }
    }
    if read_u8(a as usize + i) != 0 {
        1
    } else if read_u8(b as usize + i) != 0 {
        -1
    } else {
        0
    }
}

/// Comparer for `utils_string_compare`: the answer of both.
pub(crate) unsafe extern "C" fn compare_utils_string_compare(a: *const u8, b: *const u8, case_sensitive: u32, length: u32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const u8, *const u8, u32, u32) -> i32>(%%I2%%)(a, b, case_sensitive, length);
    let m = utils_string_compare(a, b, case_sensitive, length);
    SCOREBOARD.record(r == m, || format!("utils_string_compare(case {}, length {}) original={} rust={}", case_sensitive, length, r, m));
    r
}
