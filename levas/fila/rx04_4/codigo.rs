// rx04_4: whether a language is available, and the language table entry's constructor and destructor (Langlib.cpp).

/// Whether language.ldb has the language `id` (`G_LANGLIB_AVAILABLE`): 1 or 0. Before `langlib_setup` (language -1)
/// the error box of line 0x1a6, "Setup not yet called" and 0.
/// Original: `langlib_is_language_available` (0x66ea50, Langlib.cpp, cdecl).
pub(crate) unsafe extern "C" fn langlib_is_language_available(id: i32) -> u32 {
    if read_i32(G_LANGLIB_LANGUAGE) == -1 {
        langlib_fail(0x1a6, LANGLIB_SETUP_NOT_CALLED);
        return 0;
    }
    covered!("rx04_4 language available");
    let n = read_i32(G_LANGLIB_AVAILABLE_COUNT);
    (0..n.max(0) as usize).any(|k| read_i32(G_LANGLIB_AVAILABLE + 4 * k) == id) as u32
}

/// Side by side (it only reads), except before the setup (the error box): then only the original.
pub(crate) unsafe extern "C" fn compare_langlib_is_language_available(id: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32) -> u32>(%%I0%%);
    if read_i32(G_LANGLIB_LANGUAGE) == -1 {
        return o(id);
    }
    let a = o(id);
    let b = langlib_is_language_available(id);
    SCOREBOARD.record(a == b, || format!("langlib_is_language_available({}): original {} rust {}", id, a, b));
    a
}

/// Builds an empty language table entry: its three strings null (the 10 piece numbers are left as they are).
/// Returns the entry.
/// Original: `LANGLIB_ENTRY::LANGLIB_ENTRY` (0x66f4c0, Langlib.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn langlib_entry_new(this: usize) -> usize {
    covered!("rx04_4 entry built");
    for slot in [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT, LANGLIB_ENTRY_TRANSLATION] {
        write_usize(this + slot, 0);
    }
    this
}

/// Side by side: the copy on an entry of its own filled with 0xcd, the original on the caller's; the three strings
/// must both be null and the rest of the copy's entry untouched.
pub(crate) unsafe extern "thiscall" fn compare_langlib_entry_new(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let mut mine = [0xcdu8; LANGLIB_ENTRY_SIZE];
    let m = mine.as_mut_ptr() as usize;
    let b = langlib_entry_new(m);
    let a = o(this);
    let ok = a == this && b == m && (0..12).all(|k| read_u8(this + k) == mine[k]) && mine[12..].iter().all(|&v| v == 0xcd);
    SCOREBOARD.record(ok, || format!("LANGLIB_ENTRY::LANGLIB_ENTRY: original {:x?} rust {:x?}", (0..12).map(|k| read_u8(this + k)).collect::<Vec<_>>(),
        mine));
    a
}

/// Destroys a language table entry: deletes its key, its argument format and its translation (the ones not null),
/// in that order.
/// Original: `LANGLIB_ENTRY::~LANGLIB_ENTRY` (0x66ea10, Langlib.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn langlib_entry_drop(this: usize) {
    covered!("rx04_4 entry destroyed");
    for slot in [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT, LANGLIB_ENTRY_TRANSLATION] {
        let p = read_usize(this + slot);
        if p != 0 {
            exe::crt_operator_delete(p);
        }
    }
}

/// A destructor cannot run twice on the same strings: the copy destroys an entry of its own holding copies of the
/// three strings, then the original destroys the caller's (nothing to compare: what they do is free).
pub(crate) unsafe extern "thiscall" fn compare_langlib_entry_drop(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let mut mine = [0usize; LANGLIB_ENTRY_SIZE / 4];
    let m = mine.as_mut_ptr() as usize;
    for slot in [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT, LANGLIB_ENTRY_TRANSLATION] {
        if read_usize(this + slot) != 0 {
            langlib_replace_string(m + slot, read_usize(this + slot));
        }
    }
    langlib_entry_drop(m);
    o(this);
}
