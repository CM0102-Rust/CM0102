// ---------- batch %%LEVA%%: the Text class (1): setting and appending the string, adding a link (comp_text.cpp) ----------

/// "%s%s", the format of the appended string.
const COMP_TEXT_JOIN: usize = 0x99ec04;
/// A Text object (8 bytes): the string (+0, heap, operator new) and the link list (+4, an LLIST of 0xd0-byte links:
/// the phrase at +0, at most 200 bytes with its terminator, the link kind at +0xc8 and its data at +0xcc).
const TEXT_STRING: usize = 0;
const TEXT_LINKS: usize = 4;
const TEXT_LINK_SIZE: usize = 0xd0;
const TEXT_LINK_KIND: usize = 0xc8;
const TEXT_LINK_DATA: usize = 0xcc;
const TEXT_LINK_MAX_PHRASE: usize = 0xc8;

/// The length of a C string.
unsafe fn comp_text_len(s: usize) -> usize {
    core::ffi::CStr::from_ptr(s as *const core::ffi::c_char).to_bytes().len()
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

/// A local Text for the comparers: a copy of the string (operator new) and of the link list of `t`.
unsafe fn comp_text_clone(t: usize, with_links: bool) -> [usize; 2] {
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

/// Frees a comparer's local Text (its string and its list), as `Text::clear` does.
unsafe fn comp_text_free(c: &mut [usize; 2]) {
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
unsafe fn compare_text_method(name: &str, this: usize, with_links: bool, original: impl FnOnce() -> u32, rust: impl FnOnce(usize) -> u32) -> u32 {
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
