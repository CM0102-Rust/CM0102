// r7_01_1: the grammar modifier (Langlib.cpp): the entry, the English and Portuguese rules and the languages without
// rules; the other languages (French, German, Italian, Spanish, Danish, Dutch, Norwegian, Swedish) still run the original.

/// The grammar modifier's output (0x800 bytes): zeroed on entry, returned by every call that ends normally.
pub(crate) const G_LANGLIB_GRAMMAR_OUT: usize = 0xb64804;

/// The gender byte that means "none" (the English second pass of `langlib_sprintf`, the competition names).
const GRAMMAR_GENDER_NONE: i8 = 100;

/// The marks shared by the languages (strings of the exe, compared with strcmp).
const GRAMMAR_MARK_UPPER: usize = 0x9f15ec; // "upper"
const GRAMMAR_MARK_LOWER: usize = 0x9f15c0; // "lower"
const GRAMMAR_MARK_S: usize = 0x99c368; // "s"
/// " ": the article or ending of a gender the rules do not know.
const GRAMMAR_SPACE: usize = 0x9f1308;

/// The English marks and their texts.
const ENGLISH_MARK_S_UPPER: usize = 0x9f15e8; // "S"
const ENGLISH_MARK_AN: usize = 0x9f15dc; // "an"
const ENGLISH_MARK_AN_UPPER: usize = 0x9f15d0; // "An"
const ENGLISH_APOSTROPHE: usize = 0x9f15e0; // "'"
const ENGLISH_APOSTROPHE_S: usize = 0x9f15e4; // "'s"
const ENGLISH_AN: usize = 0x9f15d8; // "an "
const ENGLISH_A: usize = 0x9f15d4; // "a "
const ENGLISH_AN_UPPER: usize = 0x9f15cc; // "An "
const ENGLISH_A_UPPER: usize = 0x9f15c8; // "A "

/// One Portuguese rule: the marks that select it and the text for each gender byte, 0 masculine, 1 feminine,
/// 2 masculine plural, 3 feminine plural and 4 (the form without article).
struct PortugueseRule {
    marks: &'static [usize],
    forms: [usize; 5],
}

/// The Portuguese marks before the text: the article or preposition goes before it. A gender byte out of 0..4 gives
/// " ", except for "do"/"di" with `GRAMMAR_GENDER_NONE`, which gives nothing (`PORTUGUESE_OF`).
const PORTUGUESE_BEFORE: [PortugueseRule; 12] = [
    // "no": "no ", "na ", "nos ", "nas ", "em "
    PortugueseRule { marks: &[0x9eee10], forms: [0x9eee08, 0x9eee04, 0x9eedfc, 0x9eedf4, 0x9eee0c] },
    // "No": "No ", "Na ", "Nos ", "Nas ", "Em "
    PortugueseRule { marks: &[0x98e8cc], forms: [0x9eedec, 0x9eede8, 0x9eede0, 0x9eedd8, 0x9eedf0] },
    // "o", "a": "o ", "a ", "os ", "as ", ""
    PortugueseRule { marks: &[0x9eedd4, 0x99c35c], forms: [0x9eecc4, 0x9f15d4, 0x9eecc0, 0x9eecbc, STR_EMPTY] },
    // "O": "O ", "A ", "Os ", "As ", ""
    PortugueseRule { marks: &[0x9eedd0], forms: [0x9eedcc, 0x9f15c8, 0x9eedc8, 0x9eedc4, STR_EMPTY] },
    // "do", "di": "do ", "da ", "dos ", "das ", "de "
    PortugueseRule { marks: &PORTUGUESE_OF, forms: [0x9eecd0, 0x9f0e58, 0x9eecc8, 0x9f07b0, 0x9f14c0] },
    // "Do": "Do ", "Da ", "Dos ", "Das ", "De "
    PortugueseRule { marks: &[0x9eedb8], forms: [0x9eedb0, 0x9eedac, 0x9eeda4, 0x9f07cc, 0x9eedb4] },
    // "DO": "DO ", "DA ", "DOS ", "DAS ", "DE "
    PortugueseRule { marks: &[0x9eeda0], forms: [0x9eed98, 0x9eed94, 0x9eed8c, 0x9eed84, 0x9eed9c] },
    // "ao": "ao ", "à ", "aos ", "às ", "a "
    PortugueseRule { marks: &[0x9eed80], forms: [0x9eed7c, 0x9f1360, 0x9eed74, 0x9eed70, 0x9f15d4] },
    // "pelo": "pelo ", "pela ", "pelos ", "pelas ", "por "
    PortugueseRule { marks: &[0x9eed68], forms: [0x9eed58, 0x9eed50, 0x9eed48, 0x9eed40, 0x9eed60] },
    // "um": "um ", "uma ", "umos ", "umas ", ""
    PortugueseRule { marks: &[0x9eed3c], forms: [0x9eed38, 0x9eed30, 0x9eed28, 0x9eed20, STR_EMPTY] },
    // "Um", "Uma": "Um ", "Uma ", "Umos ", "Umas ", ""
    PortugueseRule { marks: &[0x9eed1c, 0x9eed18], forms: [0x9eecec, 0x9eece4, 0x9eecdc, 0x9eecd4, STR_EMPTY] },
    // "deste": "desta ", "destes ", "destas ", "deste ", "deste " (the original's table is one gender late)
    PortugueseRule { marks: &[0x9eed10], forms: [0x9eed00, 0x9eecf8, 0x9eecf0, 0x9eed08, 0x9eed08] },
];

/// "do" and "di": the rule where `GRAMMAR_GENDER_NONE` gives no preposition.
const PORTUGUESE_OF: [usize; 2] = [0x9eedc0, 0x9eedbc];

/// The Portuguese marks after the text: the participle goes after it, agreeing with the gender (4 as 0); a gender
/// byte out of 0..4 gives " ".
const PORTUGUESE_AFTER: [PortugueseRule; 8] = [
    // "promovido": " promovido", " promovida", " promovidos", " promovidas"
    PortugueseRule { marks: &[0x9eecb0], forms: [0x9eeca4, 0x9eec98, 0x9eec8c, 0x9eec80, 0x9eeca4] },
    // "relegado": " relegado", " relegada", " relegados", " relegadas"
    PortugueseRule { marks: &[0x9eec74], forms: [0x9eec68, 0x9eec5c, 0x9eec50, 0x9eec44, 0x9eec68] },
    // "foi promovido": " foi promovido", " foi promovida", " foram promovidos", " foram promovidas"
    PortugueseRule { marks: &[0x9eec34], forms: [0x9eec24, 0x9eec14, 0x9eec00, 0x9eebec, 0x9eec24] },
    // "despromovido": " despromovido", " despromovida", " despromovidos", " despromovidas"
    PortugueseRule { marks: &[0x9eebdc], forms: [0x9eebcc, 0x9eebbc, 0x9eebac, 0x9eeb9c, 0x9eebcc] },
    // "eliminado": " eliminado", " eliminada", " eliminados", " eliminadas"
    PortugueseRule { marks: &[0x9eeb90], forms: [0x9eeb84, 0x9eeb78, 0x9eeb6c, 0x9eeb60, 0x9eeb84] },
    // "foi despromovido": " foi despromovido", " foi despromovida", " foram despromovidos", " foram despromovidas"
    PortugueseRule { marks: &[0x9eeb4c], forms: [0x9eeb38, 0x9eeb24, 0x9eeb0c, 0x9eeaf4, 0x9eeb38] },
    // "foi premiado": " foi premiado", " foi premiada", " foram premiados", " foram premiadas"
    PortugueseRule { marks: &[0x9eeae4], forms: [0x9eead4, 0x9eeac4, 0x9eeab0, 0x9eea9c, 0x9eead4] },
    // "está qualificado": " está qualificado", " está qualificada", " estão qualificados", " estão qualificadas"
    PortugueseRule { marks: &[0x9eea88], forms: [0x9eea74, 0x9eea60, 0x9eea4c, 0x9eea38, 0x9eea74] },
];

/// The original's signature (cdecl): the text, the mark before, the mark after and the two gender bytes (in dwords).
type GrammarModifierFn = unsafe extern "C" fn(usize, usize, usize, u32, u32) -> usize;

/// A C string of any length, without its terminator.
unsafe fn grammar_string(s: usize) -> Vec<u8> {
    core::slice::from_raw_parts(s as *const u8, c_string_len(s)).to_vec()
}

/// The first rule of `rules` that `mark` selects.
unsafe fn portuguese_rule(rules: &'static [PortugueseRule], mark: usize) -> Option<&'static PortugueseRule> {
    rules.iter().find(|r| r.marks.iter().any(|&m| c_string_bytes_eq(mark, m)))
}

/// The form of `rule` for a gender byte: 0..4 index the forms, anything else gives " ".
unsafe fn portuguese_form(rule: &PortugueseRule, gender: i8) -> Vec<u8> {
    match gender {
        0..=4 => grammar_string(rule.forms[gender as usize]),
        _ => grammar_string(GRAMMAR_SPACE),
    }
}

/// The English rules: "upper" puts the first letter in upper case; the mark after "s" or "S" adds the possessive
/// ("'" after a final "s" or "S", otherwise "'s"); the mark before "an" or "An" gives the article by the first letter
/// ("an " or "An " before a vowel, otherwise "a " or "A "). Returns false (the caller returns null) for a possessive
/// of an empty text.
unsafe fn grammar_english(before: usize, after: usize, text: &mut Vec<u8>, prefix: &mut Vec<u8>, suffix: &mut Vec<u8>) -> bool {
    if c_string_bytes_eq(before, GRAMMAR_MARK_UPPER) {
        if let Some(c) = text.first_mut() {
            *c = exe::at_9471cd(*c as i8 as i32) as u8;
        }
    }
    if c_string_bytes_eq(after, GRAMMAR_MARK_S) || c_string_bytes_eq(after, ENGLISH_MARK_S_UPPER) {
        let Some(&last) = text.last() else {
            covered!("r7_01_1 english possessive of nothing");
            return false;
        };
        covered!("r7_01_1 english possessive");
        // the original writes them with sprintf, as formats without "%"
        *suffix = grammar_string(if last == b'S' || last == b's' { ENGLISH_APOSTROPHE } else { ENGLISH_APOSTROPHE_S });
    }
    let vowel = matches!(text.first(), Some(b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u'));
    if c_string_bytes_eq(before, ENGLISH_MARK_AN) {
        covered!("r7_01_1 english article");
        *prefix = grammar_string(if vowel { ENGLISH_AN } else { ENGLISH_A });
    } else if c_string_bytes_eq(before, ENGLISH_MARK_AN_UPPER) {
        covered!("r7_01_1 english article");
        *prefix = grammar_string(if vowel { ENGLISH_AN_UPPER } else { ENGLISH_A_UPPER });
    }
    true
}

/// The Portuguese rules. Before the text: "upper" puts the first letter in upper case, "lower" the whole text in lower
/// case, and the marks of `PORTUGUESE_BEFORE` the article or preposition by the first gender byte; any other mark
/// that is not empty is erased in the caller's string. After the text: "s" adds nothing, the marks of
/// `PORTUGUESE_AFTER` the participle by the second gender byte, and any other mark that is not empty is erased.
unsafe fn grammar_portuguese(before: usize, after: usize, genders: (i8, i8), text: &mut [u8], prefix: &mut Vec<u8>, suffix: &mut Vec<u8>) {
    if c_string_bytes_eq(before, GRAMMAR_MARK_UPPER) {
        covered!("r7_01_1 portuguese upper");
        if let Some(c) = text.first_mut() {
            *c = exe::at_9471cd(*c as i8 as i32) as u8;
        }
    } else if c_string_bytes_eq(before, GRAMMAR_MARK_LOWER) {
        covered!("r7_01_1 portuguese lower");
        for c in text.iter_mut() {
            *c = exe::at_9465fc(*c as i8 as i32) as u8;
        }
    } else if let Some(rule) = portuguese_rule(&PORTUGUESE_BEFORE, before) {
        covered!("r7_01_1 portuguese article");
        *prefix = if genders.0 == GRAMMAR_GENDER_NONE && rule.marks.contains(&PORTUGUESE_OF[0]) {
            Vec::new()
        } else {
            portuguese_form(rule, genders.0)
        };
    } else if read_u8(before) != 0 {
        covered!("r7_01_1 portuguese unknown mark before");
        write_u8(before, 0);
    }
    if c_string_bytes_eq(after, GRAMMAR_MARK_S) {
        suffix.clear();
    } else if let Some(rule) = portuguese_rule(&PORTUGUESE_AFTER, after) {
        covered!("r7_01_1 portuguese participle");
        *suffix = portuguese_form(rule, genders.1);
    } else if read_u8(after) != 0 {
        covered!("r7_01_1 portuguese unknown mark after");
        write_u8(after, 0);
    }
}

/// The grammar modifier writing to `out`: the prefix, the text and the suffix the rules of the language give.
/// Returns `out`, `STR_EMPTY` for a null text, or 0 when the English possessive is asked of an empty text.
unsafe fn grammar_modifier_into(out: usize, language: i32, text: usize, before: usize, after: usize, genders: (i8, i8)) -> usize {
    write_u8(out, 0);
    if text == 0 {
        return STR_EMPTY;
    }
    let mut body = grammar_string(text);
    let (mut prefix, mut suffix) = (Vec::new(), Vec::new());
    match language {
        0 => {
            if !grammar_english(before, after, &mut body, &mut prefix, &mut suffix) {
                return 0;
            }
        }
        9 => grammar_portuguese(before, after, genders, &mut body, &mut prefix, &mut suffix),
        _ => covered!("r7_01_1 language without rules"),
    }
    let mut at = out;
    for part in [&prefix, &body, &suffix] {
        for &c in part.iter() {
            write_u8(at, c);
            at += 1;
        }
    }
    write_u8(at, 0);
    out
}

/// Whether the original still handles the language: French (2), German (3), Italian (4), Spanish (5), Danish (6),
/// Dutch (7), Norwegian (8) and Swedish (10) are not ported yet.
fn grammar_language_in_original(language: i32) -> bool {
    matches!(language, 2..=8 | 10)
}

/// Applies a grammar mark to a text fragment for the game language (`G_LANGLIB_LANGUAGE`) and returns the result in
/// `G_LANGLIB_GRAMMAR_OUT`: the prefix, the text and the suffix, joined.
///
/// The output is emptied first. A null text returns `STR_EMPTY`. English (0): "upper" puts the first letter in upper
/// case, "s"/"S" after adds "'" (after a final "s" or "S") or "'s", and "an"/"An" before give "an "/"An " before a
/// vowel and "a "/"A " otherwise; the possessive of an empty text returns null (`langlib_sprintf` then faults).
/// Portuguese (9): "upper" and "lower" change the case; the article or preposition of the mark before ("no", "o",
/// "do", "ao", "pelo", "um", "deste"...) and the participle of the mark after ("promovido", "eliminado"...) agree with
/// the two gender bytes (0 masculine, 1 feminine, 2 and 3 plural, 4 without article; anything else gives " ", and
/// "do" with gender 100 gives nothing); a mark it does not know is erased in the caller's string. Any language without
/// rules (1, or none loaded) returns the text as it is. The other languages run the original.
///
/// Defects of the original: the null check comes after the text is copied, so a null text faults before it; the
/// "deste" table is one gender late ("desta " for a masculine, "deste " only for the feminine plural and 4).
/// Original: `langlib_apply_grammar_modifier` (0x6701f0, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_apply_grammar_modifier(text: usize, before: usize, after: usize, gender: u32, after_gender: u32) -> usize {
    let language = read_i32(G_LANGLIB_LANGUAGE);
    if grammar_language_in_original(language) {
        covered!("r7_01_1 other language");
        return original_fn::<GrammarModifierFn>(%%I0%%)(text, before, after, gender, after_gender);
    }
    grammar_modifier_into(G_LANGLIB_GRAMMAR_OUT, language, text, before, after, (gender as u8 as i8, after_gender as u8 as i8))
}

/// Side by side: the copy first, on copies of the two marks (the rules may erase them) and into a buffer of its own;
/// then the original on the caller's. The answer (output, `STR_EMPTY` or null), the text written and the marks left
/// must be equal. A language the original still handles, or a null text, runs the original only.
pub(crate) unsafe extern "C" fn compare_langlib_apply_grammar_modifier(text: usize, before: usize, after: usize, gender: u32, after_gender: u32) -> usize {
    let o = original_fn::<GrammarModifierFn>(%%I0%%);
    let language = read_i32(G_LANGLIB_LANGUAGE);
    if grammar_language_in_original(language) || text == 0 {
        return o(text, before, after, gender, after_gender);
    }
    let input = grammar_string(text);
    let (mut mine_before, mut mine_after) = (grammar_string(before), grammar_string(after));
    let (marks_before, marks_after) = (mine_before.clone(), mine_after.clone());
    mine_before.push(0);
    mine_after.push(0);
    // the longest prefix and suffix are under 0x20 bytes
    let mut mine = vec![0u8; input.len() + 0x100];
    let r = grammar_modifier_into(mine.as_mut_ptr() as usize, language, text, mine_before.as_mut_ptr() as usize,
        mine_after.as_mut_ptr() as usize, (gender as u8 as i8, after_gender as u8 as i8));
    let ro = o(text, before, after, gender, after_gender);
    let kind = |v: usize, own: usize| if v == 0 { 0 } else if v == STR_EMPTY { 1 } else if v == own { 2 } else { 3 };
    let (ta, tb) = (grammar_string(G_LANGLIB_GRAMMAR_OUT), grammar_string(mine.as_ptr() as usize));
    let (ba, bb) = (grammar_string(before), grammar_string(mine_before.as_ptr() as usize));
    let (aa, ab) = (grammar_string(after), grammar_string(mine_after.as_ptr() as usize));
    let same = kind(ro, G_LANGLIB_GRAMMAR_OUT) == kind(r, mine.as_ptr() as usize) && ta == tb && ba == bb && aa == ab;
    SCOREBOARD.record(same, || format!("langlib_apply_grammar_modifier(language {}, {:?}, {:?}, {:?}, {}, {}): original {:#x} {:?} marks {:?} {:?}; rust {:#x} {:?} marks {:?} {:?}",
        language, String::from_utf8_lossy(&input), String::from_utf8_lossy(&marks_before), String::from_utf8_lossy(&marks_after),
        gender as u8 as i8, after_gender as u8 as i8, ro, String::from_utf8_lossy(&ta), String::from_utf8_lossy(&ba),
        String::from_utf8_lossy(&aa), r, String::from_utf8_lossy(&tb), String::from_utf8_lossy(&bb), String::from_utf8_lossy(&ab)));
    ro
}
