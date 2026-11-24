// ---------- sx21b_11: numbers in texts: ordinals, numbers in words and a line break (utils.cpp) ----------

/// The shared buffer `utils_format_ordinal` returns (0x64 bytes, up to the random table's end pointer).
const G_ORDINAL_BUFFER: usize = 0xde2734;

/// The words for zero to twenty, lower case and capitalised (the language file translates them).
const NUMBER_WORDS: [[usize; 21]; 2] = [
    [0xad4374, 0xad4370, 0xad436c, 0xad4364, 0xad435c, 0xad4354, 0xad4350, 0xad4348, 0xad4340, 0xad4338, 0xad4334,
     0xad432c, 0xad4324, 0xad4318, 0xad430c, 0xad4304, 0xad42fc, 0xad42f0, 0xad42e4, 0xad42d8, 0xad42d0],
    [0xad42c8, 0xad42c4, 0xad42c0, 0xad42b8, 0xad42b0, 0xad42a8, 0xad42a4, 0xad429c, 0xad4294, 0xad428c, 0xad4288,
     0xad4280, 0xad4278, 0xad426c, 0xad4260, 0xad4258, 0xad4250, 0xad4244, 0xad4238, 0xad422c, 0xad4224],
];

/// Formats a number as an ordinal in the shared buffer and returns it.
///
/// `utils_format_ordinal(n, gender)`: 0 shows the box (line 0x3c6) and gives "". In French (language 2): 1 gives "1er"
/// ("1re" for gender 2), other numbers "%dème" (plain "%d" for gender 1). Languages 4, 7, 9 and 0xb with gender 1, and
/// 0xa and 0xb with gender 3, give plain digits. Otherwise the translated English suffix: "th" from 10 to 20 (of each
/// hundred), else "st", "nd" or "rd" for a last digit 1, 2 or 3 and "th" for the rest.
/// Original: `utils_format_ordinal` (0x90c310, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_format_ordinal(n: u32, gender: u8) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let language: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let buf = G_ORDINAL_BUFFER;
    if n < 1 {
        show_error_at(0xad4134, 0x3c6);
        match_screens_c_strcpy(buf as *mut u8, 0xad9c64 as *const u8);
        return buf;
    }
    if language() == 2 {
        covered!("sx21b_11 French ordinal");
        match (gender, n) {
            (1, 1) => { format(buf, 0xad4210); } // "1er"
            (1, _) => { format(buf, 0x98806c, n); } // "%d"
            (2, 1) => { format(buf, 0xad421c); } // "1re"
            (_, 1) => { format(buf, 0xad4210); }
            _ => { format(buf, 0xad4214, n); } // "%dème"
        }
        return buf;
    }
    let lang = language();
    if (matches!(lang, 4 | 9 | 7 | 0xb) && gender == 1) || (matches!(lang, 0xa | 0xb) && gender == 3) {
        format(buf, 0x98806c, n); // "%d"
        return buf;
    }
    covered!("sx21b_11 English ordinal");
    let (hundreds, last) = (n % 100, n % 10);
    let suffix = if hundreds > 9 && hundreds < 0x15 {
        0xad41d0 // "th": 10 to 20
    } else {
        match last {
            1 => 0xad4200, // sx21b_11: "st"
            2 => 0xad41f0, // "nd"
            3 => 0xad41e0, // "rd"
            _ => 0xad41d0, // "th"
        }
    };
    text(buf, suffix, n);
    buf
}

/// Comparer for `utils_format_ordinal`: the shared buffer side by side (0 shows the box: only the original).
pub(crate) unsafe extern "C" fn compare_utils_format_ordinal(n: u32, gender: u8) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32, u8) -> usize>(%%I0%%);
    if n < 1 {
        return o(n, gender);
    }
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(G_ORDINAL_BUFFER, 0x64usize)], || o(n, gender), || utils_format_ordinal(n, gender));
    SCOREBOARD.record(equal && r == c, || format!("utils_format_ordinal({}, {}): differences in {:?}", n, gender, first_differences(&mo, &mr)));
    r
}

/// Writes a number from zero to twenty as a word (lower case or capitalised, translated) and any other as digits; 1.
/// No buffer (line 0x45d) or a size under 50 (line 0x464): the box and 0.
/// Original: `utils_number_to_words` (0x90c520, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_number_to_words(n: u32, buffer: usize, size: i16, lower: u32) -> u32 {
    if buffer == 0 {
        show_error_at(0xad4134, 0x45d);
        return 0;
    }
    if size < 0x32 {
        show_error_at(0xad4134, 0x464);
        return 0;
    }
    if n <= 0x14 { // sx21b_11: words up to twenty
        covered!("sx21b_11 number in words");
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        text(buffer, NUMBER_WORDS[(lower == 0) as usize][n as usize]);
    } else {
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        format(buffer, 0x98806c, n); // "%d"
    }
    1
}

/// Comparer for `utils_number_to_words`: the text (up to 0x100 bytes of the buffer) and the value side by side (the box
/// cases: only the original).
pub(crate) unsafe extern "C" fn compare_utils_number_to_words(n: u32, buffer: usize, size: i16, lower: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, i16, u32) -> u32>(%%I1%%);
    if buffer == 0 || size < 0x32 {
        return o(n, buffer, size, lower);
    }
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(buffer, (size as usize).min(0x100))], || o(n, buffer, size, lower), || utils_number_to_words(n, buffer, size, lower));
    SCOREBOARD.record(equal && r == c, || format!("utils_number_to_words({}, lower {}): differences in {:?}", n, lower, first_differences(&mo, &mr)));
    r
}

/// Replaces the space nearest the middle of a text with a line break: from the middle (half the length) it looks one
/// step left (when `left`, above position 0) and one step right (when `right`, before the end) at a time; 1 when it
/// broke the line, 0 for no text, an empty one or no space.
/// Original: `utils_break_line_near_middle` (0x90cb20, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_break_line_near_middle(text: usize, left: u32, right: u32) -> u32 {
    if text == 0 {
        return 0;
    }
    let n = network_strlen(text) as i32;
    if n == 0 {
        return 0;
    }
    let (mut l, mut r) = (n / 2, n / 2);
    while l > 0 || r < n {
        if left != 0 && l > 0 && read_u8(text + l as usize) == b' ' {
            covered!("sx21b_11 line broken to the left");
            write_u8(text + l as usize, b'\n');
            return 1;
        }
        if right != 0 && r < n && read_u8(text + r as usize) == b' ' {
            covered!("sx21b_11 line broken to the right");
            write_u8(text + r as usize, b'\n'); // sx21b_11: the break to the right
            return 1;
        }
        l -= 1;
        r += 1;
    }
    0
}

/// Comparer for `utils_break_line_near_middle`: the text and the value side by side.
pub(crate) unsafe extern "C" fn compare_utils_break_line_near_middle(text: usize, left: u32, right: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I2%%);
    if text == 0 {
        return o(text, left, right);
    }
    let n = network_strlen(text) + 1;
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(text, n)], || o(text, left, right), || utils_break_line_near_middle(text, left, right));
    SCOREBOARD.record(equal && r == c, || format!("utils_break_line_near_middle(left {}, right {}): original {} rust {}, differences in {:?}", left, right, r, c, first_differences(&mo, &mr)));
    r
}
