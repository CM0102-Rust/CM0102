// rx04_2: langlib_sprintf, the game's text formatter (Langlib.cpp).

/// The most argument dwords a caller passes `langlib_sprintf` after the format (0x748782 and others: add esp, 0x68,
/// so 26 dwords with the output and the format).
pub(crate) const LANGLIB_ARG_WORDS: usize = 24;

/// The arguments of `langlib_sprintf` after the format (cdecl, variadic: every argument one dword). Passed by value,
/// the record is the caller's argument area itself (as `GuiTextAreaArgs`), so its address is the original's va_list.
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct LanglibArgs {
    pub(crate) words: [u32; LANGLIB_ARG_WORDS],
}

/// The C runtime's vsprintf (cdecl): the output, the format and the argument list.
pub(crate) const FN_CRT_VSPRINTF: usize = 0x9470ce;

/// "{" and "[" (the marks the second pass looks for in an English text).
pub(crate) const LANGLIB_OPEN_BRACE: usize = 0x9ed914;
pub(crate) const LANGLIB_OPEN_BRACKET: usize = 0x9ed910;

/// The gender byte of the grammar marks of an English text ("none").
const LANGLIB_NO_GENDER: u32 = 0x64;

/// The C runtime's vsprintf.
pub(crate) unsafe fn crt_vsprintf(out: usize, format: usize, args: usize) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_CRT_VSPRINTF);
    f(out, format, args)
}

/// Writes a message like sprintf, translated into the game language. Returns the output after the format's leading
/// spaces.
///
/// The leading spaces of the format go to the output as they are. With a game language other than English
/// (`G_LANGLIB_LANGUAGE` not -1 nor 0) and a non-empty translation of the rest (`langlib_lookup_translation`), the
/// translation is written (`langlib_write_translation`); otherwise the English text (`langlib_write_english`). The
/// entry the lookup filled is freed.
///
/// The caller's arguments are, in order, one gender byte (in a dword) for each "{" and "[" of the English text, then
/// the vsprintf arguments. In the English text "<...>" is a note for the translators: from "<" to the first "%" and
/// from the next space to ">" are left out (so "<the %s >" keeps "%s"), "{}" and "[]" are left out, and any other
/// "{mark}" stays to be applied to the word it touches.
/// Original: `langlib_sprintf` (0x66f4e0, Langlib.cpp, cdecl, variadic).
pub(crate) unsafe extern "C" fn langlib_sprintf(out: usize, format: usize, args: LanglibArgs) -> usize {
    let argp = core::ptr::addr_of!(args) as usize;
    let mut n = 0;
    while read_u8(format + n) == b' ' {
        write_u8(out + n, b' ');
        n += 1;
    }
    let (text, dest) = (format + n, out + n);
    // the entry the lookup fills: key, argument format, translation, the 10 piece numbers
    let mut entry = [0usize; LANGLIB_ENTRY_SIZE / 4];
    let e = entry.as_mut_ptr() as usize;
    let language = read_i32(G_LANGLIB_LANGUAGE);
    if language != -1 && language != 0 && langlib_lookup_translation(e, text, 0) != 0
        && read_u8(read_usize(e + LANGLIB_ENTRY_TRANSLATION)) != 0 {
        covered!("rx04_2 translated");
        langlib_write_translation(dest, text, argp, e);
    } else {
        covered!("rx04_2 english");
        langlib_write_english(dest, text, argp);
    }
    for slot in [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT, LANGLIB_ENTRY_TRANSLATION] {
        if read_usize(e + slot) != 0 {
            exe::crt_operator_delete(read_usize(e + slot));
        }
    }
    dest
}

/// The translation: the gender pairs from the caller's first arguments (`langlib_read_genders`), the argument
/// pieces (`langlib_split_pieces`), then the translated template (`langlib_expand_template`).
unsafe fn langlib_write_translation(dest: usize, text: usize, argp: usize, e: usize) {
    let mark_only = read_u32(G_LANGLIB_MARK_ONLY) != 0;
    // two gender bytes per argument piece, 0xff 0xff for "none"
    let mut genders = [0xffu8; 0x14];
    let g = genders.as_mut_ptr() as usize;
    let rest = langlib_read_genders(text, argp, mark_only, g);
    langlib_split_pieces(read_usize(e + LANGLIB_ENTRY_ARG_FORMAT), rest, mark_only);
    langlib_expand_template(dest, read_usize(e + LANGLIB_ENTRY_TRANSLATION), e + LANGLIB_ENTRY_ORDER, g);
}

/// The gender bytes of the English text's marks: each "<" opens the next pair; each "{" or "[" takes the next
/// argument's low byte (0 with `mark_only`, which reads no argument) into the first byte of the current pair, or into
/// the second byte of the previous pair when it comes right after a ">" (and a "<" was seen). Returns where the
/// vsprintf arguments start.
unsafe fn langlib_read_genders(text: usize, argp: usize, mark_only: bool, genders: usize) -> usize {
    let (mut pair, mut opens, mut p) = (0usize, 0, argp);
    let mut j = 0;
    while read_u8(text + j) != 0 {
        let c = read_u8(text + j);
        if c == b'<' {
            opens += 1;
            pair += 2;
        } else if c == b'{' || c == b'[' {
            let slot = if j > 0 && read_u8(text + j - 1) == b'>' && opens > 0 { pair - 1 } else { pair };
            let v = if mark_only {
                0
            } else {
                p += 4;
                read_u8(p - 4)
            };
            write_u8(genders + slot, v);
        }
        j += 1;
    }
    p
}

/// The argument pieces: vsprintf of the entry's argument format into `G_LANGLIB_LINE` (with `mark_only`, ten " ~"
/// instead), then the line cut at each "~", the pieces' starts in `G_LANGLIB_PIECES`.
unsafe fn langlib_split_pieces(arg_format: usize, args: usize, mark_only: bool) {
    if mark_only {
        for k in 0..10 {
            write_u8(G_LANGLIB_LINE + 2 * k, b' ');
            write_u8(G_LANGLIB_LINE + 2 * k + 1, b'~');
        }
        write_u8(G_LANGLIB_LINE + 20, 0);
    } else {
        crt_vsprintf(G_LANGLIB_LINE, arg_format, args);
    }
    write_usize(G_LANGLIB_PIECES, G_LANGLIB_LINE);
    if read_u8(G_LANGLIB_LINE) == 0 {
        return;
    }
    let (mut p, mut slot) = (G_LANGLIB_LINE, G_LANGLIB_PIECES + 4);
    loop {
        if read_u8(p) == b'~' {
            write_u8(p, 0);
            write_usize(slot, p + 1);
            slot += 4;
        }
        p += 1;
        if read_u8(p) == 0 {
            break;
        }
    }
}

/// Copies the C string at `s` to `dest + len`; returns the new length.
unsafe fn langlib_append(dest: usize, mut len: usize, s: usize) -> usize {
    // a null `s` (the grammar's answer to a text it rejects) faults here as in the original
    let mut q = s;
    while read_u8(q) != 0 {
        write_u8(dest + len, read_u8(q));
        len += 1;
        q += 1;
    }
    len
}

/// Copies the mark that starts at `t + i` (its first character) into `to` up to the "}" or the end of the text, and
/// terminates it; returns the position of the "}" (or of the end).
unsafe fn langlib_copy_mark(t: usize, mut i: usize, to: usize) -> usize {
    let mut c = 0;
    let mut al = read_u8(t + i);
    while al != b'}' && al != 0 {
        write_u8(to + c, al);
        al = read_u8(t + i + 1);
        c += 1;
        i += 1;
    }
    write_u8(to + c, 0);
    i
}

/// Writes the translated template: each 0x08 is the piece its number (the entry's next number) names. A piece whose
/// gender pair is 0xff 0xff goes as it is; otherwise through `langlib_apply_grammar_modifier` with the mark before
/// it ("{upper}" before the 0x08, kept until the next space, `G_LANGLIB_MODIFIER_BEFORE`), the mark right after it
/// ("\b{s}", `G_LANGLIB_MODIFIER_AFTER`) and its two gender bytes. "%%" becomes "%".
/// Defect of the original: a mark without its "}" stops at the end of the text and the scan goes on past it
/// (Spanish, Italian, Dutch, Norwegian and Swedish templates in language.ldb have one); the copy reads the same.
unsafe fn langlib_expand_template(dest: usize, t: usize, order: usize, genders: usize) {
    write_u8(G_LANGLIB_MODIFIER_BEFORE, 0);
    let mut len = 0;
    if read_u8(t) != 0 {
        let (mut order, mut prev, mut i) = (order, b' ', 0usize);
        loop {
            if read_u8(t + i) == b' ' {
                write_u8(G_LANGLIB_MODIFIER_BEFORE, 0);
            }
            if read_u8(t + i) == b'{' {
                i = langlib_copy_mark(t, i + 1, G_LANGLIB_MODIFIER_BEFORE) + 1;
            }
            if read_u8(t + i) == 8 {
                let k = read_i32(order) as isize;
                let (g0, g1) = (read_u8((genders as isize + 2 * k) as usize), read_u8((genders as isize + 2 * k + 1) as usize));
                let piece = read_usize((G_LANGLIB_PIECES as isize + 4 * k) as usize);
                let s = if g0 == 0xff && g1 == 0xff {
                    piece
                } else {
                    covered!("rx04_2 piece through the grammar");
                    write_u8(G_LANGLIB_MODIFIER_AFTER, 0);
                    if read_u8(t + i + 1) == b'{' {
                        i = langlib_copy_mark(t, i + 2, G_LANGLIB_MODIFIER_AFTER);
                    }
                    exe::langlib_apply_grammar_modifier(piece, G_LANGLIB_MODIFIER_BEFORE as *const u8, G_LANGLIB_MODIFIER_AFTER as *const u8,
                        g0 as u32, g1 as u32)
                };
                len = langlib_append(dest, len, s);
                order += 4;
                i += 1;
                if read_u8(t + i) == b' ' {
                    write_u8(G_LANGLIB_MODIFIER_BEFORE, 0);
                }
            }
            let c = read_u8(t + i);
            if c == 0 {
                break;
            }
            if c == b'%' && prev == b'%' {
                prev = b' ';
            } else {
                prev = c;
                write_u8(dest + len, c);
                len += 1;
            }
            i += 1;
            if read_u8(t + i) == 0 {
                break;
            }
        }
    }
    write_u8(dest + len, 0);
}

/// The English text: the format cleaned into `G_LANGLIB_LINE` (`langlib_clean_format`: the translators' notes, "{}"
/// and "[]" out, one argument skipped per "{" or "["), then vsprintf into the output with the arguments after the
/// genders. With `mark_only` nothing is written. When the result has a "{" or "[" (in the format or in an argument)
/// and is under 0x3e8 bytes, the marks are applied (`langlib_apply_marks`).
unsafe fn langlib_write_english(dest: usize, text: usize, argp: usize) {
    let mark_only = read_u32(G_LANGLIB_MARK_ONLY) != 0;
    let args = langlib_clean_format(text, if mark_only { 0 } else { argp }, mark_only);
    if mark_only {
        return;
    }
    crt_vsprintf(dest, G_LANGLIB_LINE, args);
    if exe::crt_strstr(dest, LANGLIB_OPEN_BRACE) == 0 && exe::crt_strstr(dest, LANGLIB_OPEN_BRACKET) == 0 {
        return;
    }
    if c_string_len(dest) >= 0x3e8 {
        return;
    }
    covered!("rx04_2 english with marks");
    langlib_apply_marks(dest);
}

/// The format of an English text without the translators' notes and the empty marks (see `langlib_sprintf`), into
/// `G_LANGLIB_LINE`; every "{" or "[" skips one argument. Returns where the vsprintf arguments start.
unsafe fn langlib_clean_format(text: usize, mut args: usize, mark_only: bool) -> usize {
    let mut n = 0;
    let mut in_note = false;
    let mut j = 0usize;
    // a "{"/"[" consumes a gender argument; true when it or its closing partner is dropped
    let open_or_close_dropped = |j: usize, c: u8, args: &mut usize| -> bool {
        if c == b'{' || c == b'[' {
            if !mark_only {
                *args += 4;
            }
            let d = read_u8(text + j + 1);
            if d == b'}' || d == b']' {
                return true;
            }
        }
        if c == b'}' || c == b']' {
            if j == 0 {
                return true;
            }
            let d = read_u8(text + j - 1);
            if d == b'{' || d == b'[' {
                return true;
            }
        }
        false
    };
    if read_u8(text) != 0 {
        loop {
            'next: {
                let mut c = read_u8(text + j);
                if open_or_close_dropped(j, c, &mut args) {
                    break 'next;
                }
                if c == b'<' {
                    while c != 0 {
                        c = read_u8(text + j + 1);
                        j += 1;
                        if c == b'%' {
                            break;
                        }
                    }
                    in_note = true;
                }
                c = read_u8(text + j);
                if c == b' ' && in_note {
                    while c != 0 {
                        c = read_u8(text + j + 1);
                        j += 1;
                        if c == b'>' {
                            break;
                        }
                    }
                    c = read_u8(text + j + 1);
                    in_note = false;
                    j += 1;
                    if open_or_close_dropped(j, c, &mut args) {
                        break 'next;
                    }
                }
                write_u8(G_LANGLIB_LINE + n, c);
                n += 1;
                if read_u8(text + j) == 0 {
                    write_u8(G_LANGLIB_LINE + n, 0);
                    return args;
                }
            }
            j += 1;
            if read_u8(text + j) == 0 {
                break;
            }
        }
    }
    write_u8(G_LANGLIB_LINE + n, 0);
    args
}

/// The locals of the marks pass, in the original's stack order (a mark at the start of the text reads the byte
/// before the text, the last of `before`).
#[repr(C)]
struct LanglibMarksFrame {
    after: [u8; 0x64],
    word: [u8; 0x64],
    before: [u8; 0x64],
    text: [u8; 0x3e8],
}

/// Applies the marks of an English result through `langlib_apply_grammar_modifier` (genders 0x64 0x64, "none"),
/// rewriting the output from a copy of it. A mark touching the word before it ("word{s}") takes that word back from
/// the output, from two characters before its start (the character and the space before it, which the grammar
/// needs to contract articles), with the mark as the "after" mark. A mark at the start or after a space or line break
/// ("{upper}word", maybe "{upper}word{s}") takes the word after it, up to a space or mark, with the first mark as the
/// "before" mark and a following one as the "after" mark.
/// Defect of the original: a mark without its "}" is copied past the end of the text.
unsafe fn langlib_apply_marks(dest: usize) {
    let mut f = LanglibMarksFrame { after: [0; 0x64], word: [0; 0x64], before: [0; 0x64], text: [0; 0x3e8] };
    let (after, word, before, l) = (f.after.as_mut_ptr() as usize, f.word.as_mut_ptr() as usize, f.before.as_mut_ptr() as usize,
        f.text.as_mut_ptr() as usize);
    copy_c_string(l, dest);
    let (mut i, mut out) = (0usize, 0usize);
    while read_u8(l + i) != 0 {
        let c = read_u8(l + i);
        if c != b'{' && c != b'[' {
            write_u8(dest + out, c);
            out += 1;
            i += 1;
            continue;
        }
        write_u8(before, 0);
        write_u8(after, 0);
        let p = if i != 0 { read_u8(l + i - 1) } else { b' ' };
        if p != b' ' && p != b'\n' {
            covered!("rx04_2 mark after a word");
            let mut k = i + 1;
            i = langlib_copy_closed_mark(l, k, after) + 1;
            while k != 0 && out != 0 {
                let a = read_u8(l + k);
                if a == b' ' || a == b'\n' {
                    break;
                }
                k -= 1;
                out -= 1;
            }
            // the word from l[k - 1] up to the mark (l[-1] is the last byte of `before`)
            let mut q = l + k - 1;
            let mut n = 0;
            let mut a = read_u8(q);
            while a != b'{' && a != b'[' {
                write_u8(word + n, read_u8(q));
                a = read_u8(q + 1);
                n += 1;
                q += 1;
            }
            write_u8(word + n, 0);
        } else {
            covered!("rx04_2 mark before a word");
            i = langlib_copy_closed_mark(l, i + 1, before) + 1;
            let mut n = 0;
            let mut a = read_u8(l + i);
            while a != 0 && a != b' ' && a != b'{' && a != b'[' {
                i += 1;
                write_u8(word + n, a);
                n += 1;
                a = read_u8(l + i);
            }
            write_u8(word + n, 0);
            if a == b'{' || a == b'[' {
                i = langlib_copy_closed_mark(l, i + 1, after) + 1;
            }
        }
        let s = exe::langlib_apply_grammar_modifier(word, before as *const u8, after as *const u8, LANGLIB_NO_GENDER, LANGLIB_NO_GENDER);
        out = langlib_append(dest, out, s);
    }
    write_u8(dest + out, 0);
}

/// Copies the mark that starts at `t + i` into `to` up to its "}" (no stop at the end of the text, as in the
/// original) and terminates it; returns the position of the "}".
unsafe fn langlib_copy_closed_mark(t: usize, mut i: usize, to: usize) -> usize {
    let mut c = 0;
    let mut al = read_u8(t + i);
    while al != b'}' {
        write_u8(to + c, al);
        al = read_u8(t + i + 1);
        c += 1;
        i += 1;
    }
    write_u8(to + c, 0);
    i
}

/// The globals `langlib_sprintf` writes besides the output: the line and its pieces, the two marks.
const LANGLIB_SPRINTF_STATE: [(usize, usize); 4] = [(G_LANGLIB_PIECES, 0x28), (G_LANGLIB_LINE, 0x828), (G_LANGLIB_MODIFIER_BEFORE, 0x828),
    (G_LANGLIB_MODIFIER_AFTER, 0x100)];

/// Side by side: the copy first, into a buffer of its own (an argument may be the caller's output buffer), then the
/// original into the caller's; the texts, the returned position (from the start of each buffer) and the line, pieces
/// and marks each leaves must be equal.
pub(crate) unsafe extern "C" fn compare_langlib_sprintf(out: usize, format: usize, args: LanglibArgs) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, LanglibArgs) -> usize>(%%I0%%);
    let mut mine = vec![0u8; 0x2000];
    let m = mine.as_mut_ptr() as usize;
    let b = langlib_sprintf(m, format, args) - m;
    let state_rust = crate::comparison::MemorySnapshot::read(&LANGLIB_SPRINTF_STATE);
    let r = o(out, format, args);
    let a = r - out;
    let state_original = crate::comparison::MemorySnapshot::read(&LANGLIB_SPRINTF_STATE);
    let (ta, tb) = (langlib_text_of(out), langlib_text_of(m));
    SCOREBOARD.record(a == b && ta == tb && state_original == state_rust, || format!("langlib_sprintf({:?}, language {}): original {:?} rust {:?}{}",
        String::from_utf8_lossy(&langlib_text_of(format)), read_i32(G_LANGLIB_LANGUAGE), String::from_utf8_lossy(&ta), String::from_utf8_lossy(&tb),
        if state_original == state_rust { "" } else { " (line, pieces or marks differ)" }));
    r
}

/// The C string at `a` (up to 0x2000 bytes), for the comparers.
pub(crate) unsafe fn langlib_text_of(a: usize) -> Vec<u8> {
    let n = (0..0x2000).find(|&k| read_u8(a + k) == 0).unwrap_or(0x2000);
    core::slice::from_raw_parts(a as *const u8, n).to_vec()
}
