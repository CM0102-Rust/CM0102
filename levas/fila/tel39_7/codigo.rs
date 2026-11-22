// ---------- batch %%LEVA%%: the Text class (2): the constructor, the destructor and clearing (comp_text.cpp) ----------

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
