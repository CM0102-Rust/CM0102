// ---------- batch %%LEVA%%: the human manager's nicknames for people (human_manager.cpp; sx12b_9) ----------

/// One nickname entry (30 bytes): the person id (+0) and the nickname text (+4, up to 26 characters).
const NICKNAME_SIZE: usize = 0x1e;
const NICKNAME_MAX: usize = 0x1a;
/// `compare_int_keys`, the order of the nickname list (by person id), called through the exe by qsort and bsearch.
const FN_COMPARE_INT_KEYS: usize = 0x601a10;

/// The human record's nickname entry for `person`, by binary search (`bsearch` with `compare_int_keys`; the key is the
/// id and a 0 byte), or 0.
unsafe fn find_nickname(record: usize, person: usize) -> usize {
    let key = [read_u32(person), 0u32];
    exe::crt_bsearch(key.as_ptr() as usize, read_usize(record + human_manager_record::NICKNAMES),
        read_u32(record + human_manager_record::NICKNAME_COUNT), NICKNAME_SIZE as u32, FN_COMPARE_INT_KEYS)
}

/// Whether a nickname text is too long (over 26 characters; a null text passes the check).
unsafe fn nickname_too_long(text: usize) -> bool {
    text != 0 && core::ffi::CStr::from_ptr(text as *const core::ffi::c_char).to_bytes().len() > NICKNAME_MAX
}

/// Adds the nickname a human manager gives a person to his sorted list.
///
/// `HUMAN_MANAGER::add_nickname(person, text, manager)`: a null person: nothing. No manager (null: the current human)
/// or out of range: the "Error" box (human_manager.cpp lines 0x9b3, 0x9bf); a text over 26 characters: 0x9c7. Without
/// a list, one of 10 entries (`malloc` of 300; none stops the game, line 0x9d4); a full list grows by 10 (`realloc`;
/// none stops the game, 0x9e0). The entry (the person's id and the text, `strcpy`) goes at the end and the list is
/// sorted again (qsort with `compare_int_keys`). A person who already has a nickname gets a second entry.
/// Original: `HUMAN_MANAGER::add_nickname` (0x600090, human_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_nickname(this: usize, person: usize, text: usize, manager: usize) {
    if person == 0 {
        return;
    }
    let Some(record) = human_record_or_error(this, manager, 0x9b3, 0x9bf) else {
        return;
    };
    if nickname_too_long(text) {
        show_error_at(0x9cf11c, 0x9c7); // "...\human_manager.cpp"
        return;
    }
    if read_usize(record + human_manager_record::NICKNAMES) == 0 {
        let list = exe::crt_malloc(10 * NICKNAME_SIZE as u32);
        write_usize(record + human_manager_record::NICKNAMES, list);
        if list == 0 {
            fatal_error_at(0x9cf11c, 0x9d4);
        }
        write_u32(record + 0xa6e7, 10); // the nickname capacity
    }
    if read_u32(record + 0xa6e7) == read_u32(record + human_manager_record::NICKNAME_COUNT) {
        let list = exe::crt_realloc(read_usize(record + human_manager_record::NICKNAMES),
            read_u32(record + 0xa6e7).wrapping_add(10).wrapping_mul(NICKNAME_SIZE as u32));
        write_usize(record + human_manager_record::NICKNAMES, list);
        if list == 0 {
            fatal_error_at(0x9cf11c, 0x9e0);
        }
        write_u32(record + 0xa6e7, read_u32(record + 0xa6e7).wrapping_add(10));
    }
    let n = read_u32(record + human_manager_record::NICKNAME_COUNT) as usize;
    let entry = read_usize(record + human_manager_record::NICKNAMES) + n * NICKNAME_SIZE;
    write_u32(entry, read_u32(person));
    match_screens_c_strcpy((entry + 4) as *mut u8, text as *const u8); // sx12b_9: the nickname text
    write_u32(record + human_manager_record::NICKNAME_COUNT, n as u32 + 1);
    exe::crt_qsort(read_usize(record + human_manager_record::NICKNAMES), n as u32 + 1, NICKNAME_SIZE as u32, FN_COMPARE_INT_KEYS);
    covered!("sx12b_9 nickname added");
}

/// Removes a person's nickname from a human manager's list: the entries after it move up one place, the last one gets
/// id -1 and an empty text, the count goes down and the list is sorted again. A null person, no list or no nickname for
/// the person: nothing. No manager (null: the current human) or out of range: the "Error" box (lines 0xa02, 0xa0e).
/// Original: `HUMAN_MANAGER::remove_nickname` (0x6003d0, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn remove_nickname(this: usize, person: usize, manager: usize) {
    if person == 0 {
        return;
    }
    let Some(record) = human_record_or_error(this, manager, 0xa02, 0xa0e) else {
        return;
    };
    let list = read_usize(record + human_manager_record::NICKNAMES);
    if list == 0 {
        return;
    }
    let found = find_nickname(record, person);
    if found == 0 {
        return;
    }
    let count = || read_u32(record + human_manager_record::NICKNAME_COUNT) as i32;
    let mut k = ((found.wrapping_sub(list)) / NICKNAME_SIZE) as i32;
    while k < count() - 1 {
        let e = read_usize(record + human_manager_record::NICKNAMES) + k as usize * NICKNAME_SIZE;
        core::ptr::copy((e + NICKNAME_SIZE) as *const u8, e as *mut u8, NICKNAME_SIZE); // sx12b_9: the next one moves up
        k += 1;
    }
    let end = read_usize(record + human_manager_record::NICKNAMES) + count() as usize * NICKNAME_SIZE;
    write_u8(end - NICKNAME_SIZE + 4, 0);
    write_u32(end - NICKNAME_SIZE, u32::MAX);
    write_u32(record + human_manager_record::NICKNAME_COUNT, (count() - 1) as u32);
    exe::crt_qsort(read_usize(record + human_manager_record::NICKNAMES), count() as u32, NICKNAME_SIZE as u32, FN_COMPARE_INT_KEYS);
    covered!("sx12b_9 nickname removed");
}

/// Returns the nickname a human manager gave a person (the text in his list), or 0: a null person, no list or no
/// nickname. No manager (null: the current human) or out of range: the "Error" box (lines 0xa4b, 0xa57) and 0.
/// Original: `HUMAN_MANAGER::get_nickname` (0x6005e0, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_nickname(this: usize, person: usize, manager: usize) -> usize {
    if person == 0 {
        return 0;
    }
    let Some(record) = human_record_or_error(this, manager, 0xa4b, 0xa57) else {
        return 0;
    };
    if read_usize(record + human_manager_record::NICKNAMES) == 0 {
        return 0;
    }
    let found = find_nickname(record, person);
    if found == 0 { 0 } else { found + 4 } // sx12b_9: the nickname text
}

/// Replaces the text of a person's nickname in a human manager's list (`strcpy`); a null person, no list or no
/// nickname: nothing. No manager (null: the current human) or out of range: the "Error" box (lines 0xaca, 0xad6); a
/// text over 26 characters: 0xade.
/// Original: `HUMAN_MANAGER::set_nickname` (0x600870, human_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn set_nickname(this: usize, person: usize, text: usize, manager: usize) {
    if person == 0 {
        return;
    }
    let Some(record) = human_record_or_error(this, manager, 0xaca, 0xad6) else {
        return;
    };
    if nickname_too_long(text) {
        show_error_at(0x9cf11c, 0xade);
        return;
    }
    if read_usize(record + human_manager_record::NICKNAMES) == 0 {
        return;
    }
    let found = find_nickname(record, person);
    if found != 0 {
        match_screens_c_strcpy((found + 4) as *mut u8, text as *const u8);
        covered!("sx12b_9 nickname changed");
    }
}

/// Orders two records by their leading int key for qsort and bsearch: a null a gives -1, a null b 1, equal keys 0,
/// otherwise a's key minus b's (32-bit).
/// Original: `compare_int_keys` (0x601a10, human_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_int_keys(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    read_i32(a).wrapping_sub(read_i32(b)) // sx12b_9: the key difference
}

/// The nickname list of the human record a nickname routine works on, as regions for a side by side comparison: the
/// record's list fields and the list itself up to its capacity (with room for a growth of 10); None after the "Error"
/// box.
unsafe fn nickname_regions(this: usize, manager: usize) -> Option<Vec<(usize, usize)>> {
    if !human_record_found(this, manager) {
        return None;
    }
    let m = manager_or_current_human(manager);
    let record = human_manager_record_at(this, human_manager_human_manager_index(this, m)?);
    let mut regions = vec![(record + human_manager_record::NICKNAMES, 12)];
    let list = read_usize(record + human_manager_record::NICKNAMES);
    if list != 0 {
        regions.push((list, read_u32(record + 0xa6e7) as usize * NICKNAME_SIZE));
    }
    Some(regions)
}

/// Comparer for `add_nickname`: it allocates or grows the list, so only the original runs; the entry it added is then
/// checked (the person's id and the text).
pub(crate) unsafe extern "thiscall" fn compare_add_nickname(this: usize, person: usize, text: usize, manager: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%)(this, person, text, manager);
    if person == 0 || text == 0 || nickname_too_long(text) || !human_record_found(this, manager) {
        return;
    }
    let m = manager_or_current_human(manager);
    let Some(k) = human_manager_human_manager_index(this, m) else { return };
    let record = human_manager_record_at(this, k);
    let found = find_nickname(record, person);
    let ok = found != 0 && core::ffi::CStr::from_ptr((found + 4) as *const core::ffi::c_char) == core::ffi::CStr::from_ptr(text as *const core::ffi::c_char);
    SCOREBOARD.record(ok, || format!("HUMAN_MANAGER::add_nickname (staff screen nickname): the entry for {:#x} not found as given", read_u32(person)));
}

/// Comparer for `remove_nickname`: the list fields and the list, side by side; a box: only the original.
pub(crate) unsafe extern "thiscall" fn compare_remove_nickname(this: usize, person: usize, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    let Some(regions) = nickname_regions(this, manager) else {
        return o(this, person, manager);
    };
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, person, manager), || remove_nickname(this, person, manager));
    SCOREBOARD.record(equal, || format!("HUMAN_MANAGER::remove_nickname (staff screen nickname): {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `get_nickname`: the answer (it only reads); a box: only the original.
pub(crate) unsafe extern "thiscall" fn compare_get_nickname(this: usize, person: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I2%%);
    if person == 0 || !human_record_found(this, manager) {
        return o(this, person, manager);
    }
    let r = o(this, person, manager);
    let n = get_nickname(this, person, manager);
    SCOREBOARD.record(r == n, || format!("HUMAN_MANAGER::get_nickname (names) original {:#x} rust {:#x}", r, n));
    r
}

/// Comparer for `set_nickname`: the list fields and the list, side by side; a box: only the original.
pub(crate) unsafe extern "thiscall" fn compare_set_nickname(this: usize, person: usize, text: usize, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I3%%);
    let regions = if person != 0 && !nickname_too_long(text) { nickname_regions(this, manager) } else { None };
    let Some(regions) = regions else {
        return o(this, person, text, manager);
    };
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, person, text, manager), || set_nickname(this, person, text, manager));
    SCOREBOARD.record(equal, || format!("HUMAN_MANAGER::set_nickname (staff screen nickname): {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `compare_int_keys`: the answer (it only reads).
pub(crate) unsafe extern "C" fn compare_compare_int_keys(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I4%%)(a, b);
    let n = compare_int_keys(a, b);
    SCOREBOARD.record(r == n, || format!("compare_int_keys original {} rust {}", r, n));
    r
}
