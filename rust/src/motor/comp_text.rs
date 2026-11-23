use super::*;

/// Constructs a Text as a copy of another: its string and each of its links.
///
/// `Text::Text(this, other)` (the other by pointer, may be null): string and link list null; with another Text, its
/// string (when there is one) through `Text::set_string`, then, with a link list, the list read from its head (the
/// other's cursor moves) and each link added through `Text::add_link(phrase, kind, data)`. Returns `this`.
/// Original: `Text::Text` (0x4ae5c0, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_construct_copy(this: usize, other: usize) -> usize {
    write_usize(this + TEXT_STRING, 0);
    write_usize(this + TEXT_LINKS, 0);
    if other == 0 {
        return this;
    }
    covered!("tel39_2 copy a text");
    let s = read_usize(other + TEXT_STRING);
    if s != 0 {
        text_set_string(this, s);
    }
    if read_usize(other + TEXT_LINKS) == 0 {
        return this;
    }
    list_rewind(read_usize(other + TEXT_LINKS));
    loop {
        let e = exe::llist_get_next(read_usize(other + TEXT_LINKS));
        if e == 0 {
            break;
        }
        // tel39_2: each link with its kind and data
        text_add_link(this, e, read_u32(e + TEXT_LINK_KIND), read_u32(e + TEXT_LINK_DATA));
    }
    this
}

/// Comparer for `text_construct_copy`: the copy builds a local Text, the original the real one; string and links
/// equal.
pub(crate) unsafe extern "thiscall" fn compare_text_construct_copy(this: usize, other: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TEXT_CONSTRUCT_COPY);
    let mut local = [0usize; 2];
    text_construct_copy(local.as_mut_ptr() as usize, other);
    let r = o(this, other);
    let (mo, mr) = (text_contents(this), text_contents(local.as_ptr() as usize));
    comp_text_free(&mut local);
    SCOREBOARD.record(r == this && mo == mr, || format!("Text::Text (copy): original {:?} {:x?}, rust {:?} {:x?}", String::from_utf8_lossy(&mo.0), mo.1,
        String::from_utf8_lossy(&mr.0), mr.1));
    r
}

/// Replaces the string of a Text with a new copy of `source`.
///
/// `Text::set_string(this, source)`: a null source shows the comp_text.cpp 139 box and gives 0. Otherwise the old
/// string is freed (operator delete), the new one allocated (operator new, the length + 1) and stored even when null
/// (then the 151 box and 0); the source is copied with its terminator and 1 comes back.
/// Original: `Text::set_string` (0x4ae660, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_set_string(this: usize, source: usize) -> u32 {
    if source == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0x8b);
        return 0;
    }
    covered!("tel39_6 set the string");
    let old = read_usize(this + TEXT_STRING);
    if old != 0 {
        exe::crt_operator_delete(old);
    }
    // tel39_6: the length with the terminator
    let n = comp_text_len(source) + 1;
    let p = exe::crt_operator_new(n as u32);
    write_usize(this + TEXT_STRING, p);
    if p == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0x97);
        return 0;
    }
    core::ptr::copy_nonoverlapping(source as *const u8, p as *mut u8, n);
    1
}

/// Appends `source` to the string of a Text, or sets it when the Text has none.
///
/// `Text::append_string(this, source)`: a null source shows the comp_text.cpp 177 box and gives 0. With no string,
/// `Text::set_string(source)`. Otherwise a new block of both lengths + 1 (operator new) is stored even when null (then
/// the 192 box and 0, and the old string is lost: a leak of the original), the two are joined into it (`sprintf`
/// "%s%s"), the old string is freed and 1 comes back.
/// Original: `Text::append_string` (0x4ae770, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_append_string(this: usize, source: usize) -> u32 {
    if source == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0xb1);
        return 0;
    }
    let old = read_usize(this + TEXT_STRING);
    if old == 0 {
        return text_set_string(this, source);
    }
    covered!("tel39_6 append to the string");
    let p = exe::crt_operator_new((comp_text_len(old) + comp_text_len(source) + 1) as u32);
    write_usize(this + TEXT_STRING, p);
    if p == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0xc0);
        return 0;
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    // tel39_6: the old string, then the new one
    format(p, COMP_TEXT_JOIN, old, source);
    exe::crt_operator_delete(old);
    1
}

/// Adds a hyperlink to a Text: a phrase of the text with a link kind and its data.
///
/// `Text::add_link(this, phrase, kind, data)`: a null phrase shows the comp_text.cpp 227 box and a phrase of 200 bytes
/// or more (with the terminator past 200) the 235 box, both giving 0. Without a link list one is made (operator new
/// 0xe, `LLIST::LLIST`; a failed allocation stores null and shows the 247 box, 0). The list is read from its head: a
/// link with the same phrase (strcmp) gives 1 when its kind and data are the same, else 0, and nothing is added.
/// Otherwise a 0xd0-byte link (the phrase with its terminator, the kind at +0xc8, the data at +0xcc; the bytes past
/// the terminator are stack garbage in the original, zero here) is appended (`LLIST::add_tail`) and 1 comes back.
/// Original: `Text::add_link` (0x4ae8a0, comp_text.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn text_add_link(this: usize, phrase: usize, kind: u32, data: u32) -> u32 {
    if phrase == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0xe3);
        return 0;
    }
    let n = comp_text_len(phrase) + 1;
    if n > TEXT_LINK_MAX_PHRASE {
        tel26_error_box(COMP_TEXT_CPP, 0xeb);
        return 0;
    }
    if read_usize(this + TEXT_LINKS) == 0 {
        let l = exe::crt_operator_new(0xe);
        let l = if l != 0 { exe::new_llist(l) } else { 0 };
        write_usize(this + TEXT_LINKS, l);
        if l == 0 {
            tel26_error_box(COMP_TEXT_CPP, 0xf7);
            return 0;
        }
    }
    let list = read_usize(this + TEXT_LINKS);
    list_rewind(list);
    let wanted = core::slice::from_raw_parts(phrase as *const u8, n);
    loop {
        let e = exe::llist_get_next(read_usize(this + TEXT_LINKS));
        if e == 0 {
            break;
        }
        if core::slice::from_raw_parts(e as *const u8, n) == wanted {
            covered!("tel39_6 link already there");
            // tel39_6: the same phrase linked to the same object
            return (read_u32(e + TEXT_LINK_KIND) == kind && read_u32(e + TEXT_LINK_DATA) == data) as u32;
        }
    }
    covered!("tel39_6 add a link");
    let mut link = [0u8; TEXT_LINK_SIZE];
    link[..n].copy_from_slice(wanted);
    link[TEXT_LINK_KIND..TEXT_LINK_KIND + 4].copy_from_slice(&kind.to_le_bytes());
    link[TEXT_LINK_DATA..TEXT_LINK_DATA + 4].copy_from_slice(&data.to_le_bytes());
    exe::llist_add_tail(read_usize(this + TEXT_LINKS), link.as_ptr() as usize, TEXT_LINK_SIZE as u32);
    1
}

/// Comparer for `text_set_string`: side by side on a local Text (a null source, which shows the box, only the
/// original).
pub(crate) unsafe extern "thiscall" fn compare_text_set_string(this: usize, source: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TEXT_SET_STRING);
    if source == 0 {
        return o(this, source);
    }
    compare_text_method("Text::set_string", this, false, || o(this, source), |t| text_set_string(t, source))
}

/// Comparer for `text_append_string`: side by side on a local copy of the Text (a null source only the original).
pub(crate) unsafe extern "thiscall" fn compare_text_append_string(this: usize, source: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TEXT_APPEND_STRING);
    if source == 0 {
        return o(this, source);
    }
    compare_text_method("Text::append_string", this, false, || o(this, source), |t| text_append_string(t, source))
}

/// Comparer for `text_add_link`: side by side on a local copy of the Text and its links (a null or too long phrase,
/// which shows a box, only the original).
pub(crate) unsafe extern "thiscall" fn compare_text_add_link(this: usize, phrase: usize, kind: u32, data: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_TEXT_ADD_LINK);
    if phrase == 0 || comp_text_len(phrase) + 1 > TEXT_LINK_MAX_PHRASE {
        return o(this, phrase, kind, data);
    }
    compare_text_method("Text::add_link", this, true, || o(this, phrase, kind, data), |t| text_add_link(t, phrase, kind, data))
}

/// Constructs a Text with no string and no links, then copies the initial string when one is given.
///
/// `Text::Text(this, source)`: string and link list null; a non-null source goes to `Text::set_string`. Returns
/// `this`.
/// Original: `Text::Text` (0x4ae590, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_construct(this: usize, source: usize) -> usize {
    covered!("tel39_7 construct a text");
    write_usize(this + TEXT_STRING, 0);
    write_usize(this + TEXT_LINKS, 0);
    if source != 0 {
        text_set_string(this, source);
    }
    this
}

/// Destroys a Text: frees the string and the link list (the pointers are left as they were).
///
/// `Text::~Text(this)`: the string with operator delete; the list with `LLIST::~LLIST` and operator delete.
/// Original: `Text::~Text` (0x4ae630, comp_text.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn text_destruct(this: usize) {
    covered!("tel39_7 destroy a text");
    let s = read_usize(this + TEXT_STRING);
    if s != 0 {
        exe::crt_operator_delete(s);
    }
    let l = read_usize(this + TEXT_LINKS);
    if l != 0 {
        exe::drop_llist(l);
        exe::crt_operator_delete(l);
    }
}

/// Empties a Text so it can be reused: frees the string and the link list and sets both pointers to null.
///
/// `Text::clear(this)`: the string with operator delete, then null; the list with `LLIST::~LLIST` and operator
/// delete, then null. Returns 1.
/// Original: `Text::clear` (0x4afa40, comp_text.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn text_clear(this: usize) -> u32 {
    covered!("tel39_7 clear a text");
    let s = read_usize(this + TEXT_STRING);
    if s != 0 {
        exe::crt_operator_delete(s);
        write_usize(this + TEXT_STRING, 0);
    }
    let l = read_usize(this + TEXT_LINKS);
    if l != 0 {
        exe::drop_llist(l);
        exe::crt_operator_delete(l);
        // tel39_7: the list pointer cleared
        write_usize(this + TEXT_LINKS, 0);
    }
    1
}

/// Comparer for `text_construct`: the copy builds a local Text, the original the real one; string and links equal.
pub(crate) unsafe extern "thiscall" fn compare_text_construct(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TEXT_CONSTRUCT);
    let mut local = [0usize; 2];
    text_construct(local.as_mut_ptr() as usize, source);
    let r = o(this, source);
    let (mo, mr) = (text_contents(this), text_contents(local.as_ptr() as usize));
    comp_text_free(&mut local);
    SCOREBOARD.record(r == this && mo == mr, || format!("Text::Text: original {:?}, rust {:?}", String::from_utf8_lossy(&mo.0), String::from_utf8_lossy(&mr.0)));
    r
}

/// Comparer for `text_destruct`: only the original runs (freeing does not run twice; nothing is left to check).
pub(crate) unsafe extern "thiscall" fn compare_text_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEXT_DESTRUCT)(this)
}

/// Comparer for `text_clear`: only the original runs (freeing does not run twice); then both pointers must be null
/// and the answer 1, the outcome the copy guarantees.
pub(crate) unsafe extern "thiscall" fn compare_text_clear(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TEXT_CLEAR)(this);
    let (s, l) = (read_usize(this + TEXT_STRING), read_usize(this + TEXT_LINKS));
    SCOREBOARD.record(r == 1 && s == 0 && l == 0, || format!("Text::clear({:#x}): answer {}, string {:#x}, links {:#x}", this, r, s, l));
    r
}

/// Draws a Text object wrapped between two columns: the lines go into the display list of the current human's
/// screen as text items, its links as link items over the words, and the row moves past the lines.
///
/// `text` is the Text object: +0 the C string, +4 the list (LLIST) of its links or null. A null string returns 0.
/// When the string is no wider (`font_text_width`) than `x2 - x1 + 1` and holds no line feed it is one line;
/// otherwise the spaces right before every '?' become '|' (so a question mark never starts a line) and the string is
/// broken, character by character, into `WRAPPED_LINES`: a line feed becomes a space and closes the line; a
/// six-character colour code `#xxxx#` goes in whole; when the line gets wider than the columns it is cut at its last
/// space (the word after it goes to the next line) or before its last colour code (after a code that is not the last
/// thing on the line). With `max_lines` (the low byte; 0 = no limit) and more lines than that, the right column moves
/// 22 pixels left and the text is broken once more (once only). Every line becomes a text item (type 1) of the
/// current human's `Display` in area `area`, font `font`, colour `colour`, at row `*row + k`, from `x1` to `x2` or,
/// with flag 1, centred on its own width; with flag 2 only the first line is added. Each link (+0xc8 and +0xcc are the
/// item's link fields) is looked for in the string, every occurrence, and becomes link items (type 2) over the
/// characters it covers, one per line it spans. Then `*row` moves to the last line (to the first with flag 2).
/// Returns 1 (also with no lines, without moving the row).
///
/// Defects of the original: it takes the string's length before testing it for null (a null string crashes);
/// the carried-over word buffer is not cleared before its first use, so the first line cut with no space or colour
/// code to cut at starts the next line with stack leftovers (the copy starts it empty); more than 50 lines run over
/// the left and right tables on its stack (the copy hands those texts to the original). A link whose characters are
/// not found in the lines shows the "comp_text.cpp 569" or "570" error box.
/// Original: `Text::draw_wrapped` (0x4aeaf0, comp_text.cpp, thiscall, ret 0x20).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn text_draw_wrapped(text: usize, area: u32, x1: i32, x2: i32, row: usize, font: u32, colour: u32,
    flags: u32, max_lines: u32) -> u32 {
    let s = read_usize(text);
    if s == 0 {
        return 0;
    }
    let len = c_string_len(s) as i32;
    let width = |p: usize| exe::font_text_width((font & 0xffff) as usize, p as *const u8);
    let mut x2 = x2;
    let mut pass = 1;
    let mut lines: i32;
    let mut carry = [0u8; 0x200];
    let carry_at = carry.as_mut_ptr() as usize;
    let mut carry_pending = false;
    loop {
        lines = 0;
        if width(s) <= x2 - x1 + 1 && !(0..c_string_len(s)).any(|k| read_u8(s + k) == b'\n') {
            covered!("tel31_4 one line");
            if c_string_len(s) != 0 {
                copy_c_string(wrapped_line(0), s);
                lines = 1;
            }
            break;
        }
        covered!("tel31_4 wrap");
        let mut k = c_string_len(s) as i32 - 1;
        while k >= 0 {
            if read_u8(s + k as usize) == b'?' {
                while k > 0 && read_u8(s + k as usize - 1) == b' ' {
                    write_u8(s + k as usize - 1, b'|');
                    k -= 1;
                }
            }
            k -= 1;
        }
        let mut cur = 0usize;
        write_u8(wrapped_line(0), 0);
        let mut i = 0i32;
        while i < len {
            let c = read_u8(s + i as usize);
            i += 1;
            let mut piece = [0u8; 8];
            if c == b'\n' {
                piece[0] = b' ';
                carry_pending = true;
                write_u8(carry_at, 0);
            } else if c == b'#' && i < len - 4 && read_u8(s + i as usize + 4) == b'#' {
                piece[0] = c;
                for n in 0..5 {
                    piece[1 + n] = read_u8(s + i as usize + n);
                }
                i += 5;
            } else {
                piece[0] = c;
            }
            let line = wrapped_line(cur);
            append_c_string(line, piece.as_ptr() as usize);
            if width(line) > x2 - x1 + 1 {
                let mut searching = true;
                let mut first = true;
                let mut j = c_string_len(line) as i32 - 1;
                while j > 0 && searching {
                    if read_u8(line + j as usize) == b'#' && j > 5 && read_u8(line + j as usize - 5) == b'#' {
                        covered!("tel31_4 cut at a colour code");
                        j = if first { j - 5 } else { j + 1 };
                        copy_c_string(carry_at, line + j as usize);
                        write_u8(line + j as usize, 0);
                        searching = false;
                    } else if read_u8(line + j as usize) == b' ' {
                        copy_c_string(carry_at, line + j as usize + 1);
                        write_u8(line + j as usize + 1, 0);
                        searching = false;
                    }
                    first = false;
                    j -= 1;
                }
                carry_pending = true;
            }
            if carry_pending {
                cur += 1;
                copy_c_string(wrapped_line(cur), carry_at);
                carry_pending = false;
                write_u8(carry_at, 0);
            }
        }
        lines = cur as i32 + (read_u8(wrapped_line(cur)) != 0) as i32;
        let max = (max_lines & 0xff) as i32;
        if max > 0 && lines > max && pass == 1 {
            covered!("tel31_4 narrower second pass");
            x2 -= 0x16;
            pass = 2;
            continue;
        }
        break;
    }
    if lines == 0 {
        return 1;
    }
    if lines as usize > WRAPPED_MAX_LINES {
        // the original runs over its own stack here: the text goes to it (it breaks the string the same way again)
        covered!("tel31_4 over 50 lines");
        type DrawWrapped = unsafe extern "thiscall" fn(usize, u32, i32, i32, usize, u32, u32, u32, u32) -> u32;
        return original_fn::<DrawWrapped>(IDX_TEXT_DRAW_WRAPPED)(text, area, x1, x2, row, font, colour, flags, max_lines);
    }
    // the original pushes the 16-bit arguments with whatever the register held in its high half
    let high = |v: i32| v as u32 & 0xffff_0000;
    let (area, font, colour) = (area & 0xffff, font & 0xffff, colour & 0xffff);
    let mut left = [0i32; WRAPPED_MAX_LINES];
    let mut right = [0i32; WRAPPED_MAX_LINES];
    for k in 0..lines as usize {
        if flags & 1 != 0 {
            let w = width(wrapped_line(k));
            left[k] = x1 + (x2 - x1 + 1 - w) / 2;
            right[k] = left[k] + w - 1;
        } else {
            left[k] = x1;
            right[k] = x2;
        }
        if k == 0 || flags & 2 == 0 {
            tel26_add_item(current_human_display(), [1, left[k] as u32, 0, right[k] as u32, 0, 0, read_i32(row).wrapping_add(k as i32) as u32, 1, 0, 0, 1,
                font | high(right[k]), colour | high(left[k]), wrapped_line(k) as u32, 0, 0, 0, area | high(right[k])]);
        }
    }
    if read_usize(text + 4) != 0 {
        let list = read_usize(text + 4);
        write_u32(list + 0xa, read_u32(list + 2));
        'links: loop {
            let link = exe::llist_get_next(read_usize(text + 4));
            if link == 0 {
                break;
            }
            covered!("tel31_4 link");
            let mut pos = 0i32;
            loop {
                if (pos + c_string_len(link) as i32 - 1) as u32 > c_string_len(read_usize(text)) as u32 {
                    continue 'links;
                }
                let hit = exe::crt_strstr(read_usize(text) + pos as usize, link);
                if hit == 0 {
                    continue 'links;
                }
                let mut start = hit.wrapping_sub(read_usize(text)) as i32;
                let mut end = start + c_string_len(link) as i32 - 1;
                pos = start + 1;
                let (mut a, mut b, mut acc) = (-1i32, -1i32, 0i32);
                for k in 0..lines {
                    let n = c_string_len(wrapped_line(k as usize)) as i32;
                    if start >= acc && start < acc + n && a == -1 {
                        start -= acc;
                        a = k;
                    }
                    if end >= acc && end < acc + n && b == -1 {
                        end -= acc;
                        b = k;
                        break;
                    }
                    acc += n;
                }
                if a == -1 {
                    tel26_error_box(COMP_TEXT_CPP, 0x239);
                    continue 'links;
                }
                let mut cut = [0u8; 0x200];
                let cut_at = cut.as_mut_ptr() as usize;
                copy_c_string(cut_at, wrapped_line(a as usize));
                write_u8(cut_at + start as usize, 0);
                let from = left[a as usize] + width_through_call(0x4af58c as *const i32, 0x4af590, font, cut_at);
                if b == -1 {
                    tel26_error_box(COMP_TEXT_CPP, 0x246);
                    continue 'links;
                }
                copy_c_string(cut_at, wrapped_line(b as usize));
                write_u8(cut_at + end as usize + 1, 0);
                let to = left[b as usize] + width_through_call(0x4af670 as *const i32, 0x4af674, font, cut_at);
                let (field_c8, field_cc) = (read_u32(link + 0xc8), read_u32(link + 0xcc));
                let add = |x1: i32, x2: i32, k: i32, hi: u32| {
                    tel26_add_item(current_human_display(), [2, x1 as u32, 0, x2 as u32, 0, 0, read_i32(row).wrapping_add(k) as u32, 1, 0, 0, 1,
                        font | hi, 0, 0, 0, field_c8, field_cc, area | hi]);
                };
                if a == b {
                    if a == 0 || flags & 2 == 0 {
                        add(from, to, a, field_c8 & 0xffff_0000);
                    }
                } else {
                    covered!("tel31_4 link over lines");
                    if a == 0 || flags & 2 == 0 {
                        add(from, right[a as usize], a, high(read_i32(row).wrapping_add(a)));
                    }
                    for k in a + 1..b {
                        if k == 0 || flags & 2 == 0 {
                            add(left[k as usize], right[k as usize], k, high(right[k as usize]));
                        }
                    }
                    if b == 0 || flags & 2 == 0 {
                        add(left[b as usize], to, b, high(read_i32(row).wrapping_add(b)));
                    }
                }
            }
        }
    }
    let shown = if flags & 2 == 0 { lines } else { lines.min(1) };
    write_i32(row, read_i32(row).wrapping_add(shown - 1));
    1
}

/// Comparer for `text_draw_wrapped`: side by side over the string (the '|' it writes), the wrapped lines, the row,
/// the counts and the new slots of the display list (at most 64 items, or the area the items go into) and the link
/// list cursor, and the return. Only the original runs with no string or no display, and when the free item order
/// could fill up (the display.cpp 746 box).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_text_draw_wrapped(text: usize, area: u32, x1: i32, x2: i32, row: usize, font: u32, colour: u32,
    flags: u32, max_lines: u32) -> u32 {
    type DrawWrapped = unsafe extern "thiscall" fn(usize, u32, i32, i32, usize, u32, u32, u32, u32) -> u32;
    let o = original_fn::<DrawWrapped>(IDX_TEXT_DRAW_WRAPPED);
    let run = || o(text, area, x1, x2, row, font, colour, flags, max_lines);
    let s = read_usize(text);
    let d = current_human_display();
    if s == 0 || d == 0 || row == 0 {
        // simplification: the null string crashes the original; no screen to add to
        return run();
    }
    let count = read_i16(d + display_list::ITEM_COUNT);
    let order = read_i16(d + display_list::ITEM_ORDER_COUNT);
    let area16 = area as i16;
    // ponytail: at most 64 new items compared; a text with more still compares the counts
    let slots = (0x4af - count as i32).clamp(0, 64) as usize;
    if count < 0 || order < 0 || (area16 == -1 && order as usize + slots >= 0x4b0) {
        // simplification: the error box does not run twice
        return run();
    }
    let len = c_string_len(s);
    let mut reg = vec![(s, len + 1), (row, 4), (WRAPPED_LINES, (len + 2).min(WRAPPED_MAX_LINES + 1) * WRAPPED_LINE_SIZE),
        (d + display_list::ITEM_COUNT, 2), (d + display_list::ITEM_ORDER_COUNT, 2), (d + display_list::OVERFLOW, 4)];
    if slots > 0 {
        reg.push((display_item_at(d, count), slots * display_list::ITEM_SIZE));
    }
    if area16 == -1 {
        reg.push((d + display_list::ITEM_ORDER + order as usize * 2, slots * 2));
    } else {
        reg.push((display_area_at(d, area16), display_list::AREA_SIZE));
    }
    if read_usize(text + 4) != 0 {
        reg.push((read_usize(text + 4) + 0xa, 4));
    }
    let shown: String = (0..len.min(40)).map(|k| read_u8(s + k) as char).collect();
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, run, || text_draw_wrapped(text, area, x1, x2, row, font, colour, flags, max_lines));
    SCOREBOARD.record(same && r == n, || format!("Text::draw_wrapped(\"{}\", {}..{}, flags {:#x}): original={} rust={} differences {:?}", shown, x1, x2, flags & 0xffff,
        r, n, first_differences(&mo, &mr)));
    r
}

/// The length of a C string.
pub(crate) unsafe fn comp_text_len(s: usize) -> usize {
    core::ffi::CStr::from_ptr(s as *const core::ffi::c_char).to_bytes().len()
}

/// Frees a comparer's local Text (its string and its list), as `Text::clear` does.
pub(crate) unsafe fn comp_text_free(c: &mut [usize; 2]) {
    if c[0] != 0 {
        exe::crt_operator_delete(c[0]);
    }
    if c[1] != 0 {
        exe::drop_llist(c[1]);
        exe::crt_operator_delete(c[1]);
    }
    *c = [0, 0];
}

/// The comparison of a Text method: the copy runs first on a local clone of the Text (so it reads the source before
/// the original frees anything), then the original on the Text; the string, the links and the answers must match.
pub(crate) unsafe fn compare_text_method(name: &str, this: usize, with_links: bool, original: impl FnOnce() -> u32, rust: impl FnOnce(usize) -> u32) -> u32 {
    let mut local = comp_text_clone(this, with_links);
    let n = rust(local.as_mut_ptr() as usize);
    let r = original();
    let (mo, mr) = (text_contents(this), text_contents(local.as_ptr() as usize));
    let same = if with_links { mo == mr } else { mo.0 == mr.0 };
    comp_text_free(&mut local);
    SCOREBOARD.record(same && r == n, || format!("{}: original {} {:?} {:x?}, rust {} {:?} {:x?}", name, r, String::from_utf8_lossy(&mo.0), mo.1,
        n, String::from_utf8_lossy(&mr.0), mr.1));
    r
}

/// Length of the C string at `s`.
pub(crate) unsafe fn c_string_len(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// Line `k` of the wrapped lines table.
pub(crate) fn wrapped_line(k: usize) -> usize {
    WRAPPED_LINES + k * WRAPPED_LINE_SIZE
}

/// Width in pixels of the C string `s` in `font`, through the call the original makes at a site: `rel32` is the
/// displacement of that call and `next` the address after it. The 1920 × 1080 exe sends the two calls that measure
/// where a link starts and ends (0x4af58b, 0x4af66f) to a stub of its own that scales the width; the copy follows
/// whatever the running exe calls.
pub(crate) unsafe fn width_through_call(rel32: *const i32, next: usize, font: u32, s: usize) -> i32 {
    let target = next.wrapping_add(exe_immediate(rel32) as usize);
    let f: unsafe extern "C" fn(u32, usize) -> i32 = core::mem::transmute(target);
    f(font, s)
}

/// A local Text for the comparers: a copy of the string (operator new) and of the link list of `t`.
pub(crate) unsafe fn comp_text_clone(t: usize, with_links: bool) -> [usize; 2] {
    let mut c = [0usize; 2];
    let s = read_usize(t + TEXT_STRING);
    if s != 0 {
        let n = comp_text_len(s) + 1;
        c[0] = exe::crt_operator_new(n as u32);
        core::ptr::copy_nonoverlapping(s as *const u8, c[0] as *mut u8, n);
    }
    let l = read_usize(t + TEXT_LINKS);
    if with_links && l != 0 {
        c[1] = exe::new_llist(exe::crt_operator_new(0xe));
        exe::llist_copy_from(c[1], l);
    }
    c
}

/// "%s%s", the format of the appended string.
pub(crate) const COMP_TEXT_JOIN: usize = 0x99ec04;

/// A Text object (8 bytes): the string (+0, heap, operator new) and the link list (+4, an LLIST of 0xd0-byte links:
/// the phrase at +0, at most 200 bytes with its terminator, the link kind at +0xc8 and its data at +0xcc).
pub(crate) const TEXT_STRING: usize = 0;

pub(crate) const TEXT_LINKS: usize = 4;

pub(crate) const TEXT_LINK_SIZE: usize = 0xd0;

pub(crate) const TEXT_LINK_KIND: usize = 0xc8;

pub(crate) const TEXT_LINK_DATA: usize = 0xcc;

pub(crate) const TEXT_LINK_MAX_PHRASE: usize = 0xc8;

/// The lines `Text::draw_wrapped` breaks a text into: a global table of 501-byte C strings.
pub(crate) const WRAPPED_LINES: usize = 0xadae1c;

pub(crate) const WRAPPED_LINE_SIZE: usize = 0x1f5;

/// Lines the original's left and right tables hold (two 50-int arrays on its stack).
pub(crate) const WRAPPED_MAX_LINES: usize = 50;

/// The comp_text.cpp path of the error box.
pub(crate) const COMP_TEXT_CPP: usize = 0x99ebd8;
