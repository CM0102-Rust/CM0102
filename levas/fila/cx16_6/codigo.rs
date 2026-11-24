// ---------- cx16_6: the CONCACAF and Oceania World Cup qualifying entrant lists in the saved game (wc_concacaf_cup.cpp, wc_oceania_league.cpp) ----------

/// Turns a saved list of `n` club indices (−1 for none) back into club pointers (`club_at`; −1 becomes null).
pub(crate) unsafe fn club_indices_to_pointers(list: usize, n: i32) {
    for k in 0..n.max(0) as usize {
        let v = read_i32(list + k * 4);
        write_usize(list + k * 4, if v != -1 { club_at(v as isize).addr() } else { 0 }); // cx16_6: the saved index back to the club
    }
}

/// Turns a list of `n` club pointers into their indices (the club's id, `club::ID`, +0; null becomes −1), as the saved
/// game stores them.
pub(crate) unsafe fn club_pointers_to_indices(list: usize, n: i32) {
    for k in 0..n.max(0) as usize {
        let c = read_usize(list + k * 4);
        write_u32(list + k * 4, if c != 0 { read_u32(c) } else { u32::MAX });
    }
}

/// Reads the CONCACAF qualifying's own data from the saved game (`exe::crt_fread`, cdecl): the three exempt counts
/// (+0xba, +0xbb, +0xbc, a byte each; short of 3, the error box of line 0x75a and 0); the entrant count (+0xb6, a dword;
/// short, the box of line 0x762 and 0); a new list of 35 slots (`exe::crt_calloc`(35, 4); null, the box of line 0x76a
/// and 0) in +0xb2; count indices into it (short, the box of line 0x772 and 0), turned back into clubs
/// (`club_indices_to_pointers`). Returns 1. The list has 35 slots whatever the count read (always 35 in a saved game).
/// Original: `WcConcacafCup::load_extra_data` (0x917aa0, wc_concacaf_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let cpp = wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP;
    let read = exe::crt_fread(this + 0xba, 1, 1, file) + exe::crt_fread(this + 0xbb, 1, 1, file) + exe::crt_fread(this + 0xbc, 1, 1, file);
    if read != 3 {
        show_error_at(cpp, 0x75a);
        return 0;
    }
    if exe::crt_fread(this + 0xb6, 4, 1, file) != 1 {
        show_error_at(cpp, 0x762);
        return 0;
    }
    let list = exe::crt_calloc(0x23, 4);
    write_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST, list);
    if list == 0 {
        show_error_at(cpp, 0x76a);
        return 0;
    }
    let n = read_u32(this + 0xb6);
    if exe::crt_fread(list, 4, n, file) != n {
        show_error_at(cpp, 0x772);
        return 0;
    }
    covered!("cx16_6 concacaf qualifying loaded");
    club_indices_to_pointers(list, n as i32);
    1
}

/// Writes the CONCACAF qualifying's own data to the saved game (`exe::crt_fwrite`, cdecl, unchecked), in the order the
/// load reads it: the three exempt counts, the entrant count, and the list as club indices (`club_pointers_to_indices`,
/// then back, `club_indices_to_pointers`). Returns the count.
/// Original: `WcConcacafCup::save_extra_data` (0x917cd0, wc_concacaf_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx16_6 concacaf qualifying saved");
    for k in [0xba, 0xbb, 0xbc] {
        exe::crt_fwrite(this + k, 1, 1, file); // cx16_6: the three exempt counts
    }
    exe::crt_fwrite(this + 0xb6, 4, 1, file);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let n = read_u32(this + 0xb6);
    club_pointers_to_indices(list, n as i32);
    exe::crt_fwrite(list, 4, n, file);
    club_indices_to_pointers(list, n as i32);
    n
}

/// Reads the Oceania qualifying's own data from the saved game (`exe::crt_fread`, cdecl): the exempt count
/// (`wc_oceania_league_fields::EXEMPT_COUNT`, a byte; short, the error box of line 0x4b8 and 0), the pool count (+0xf3,
/// a dword; short, the box of line 0x4c0 and 0), a new pool of that many slots (`exe::crt_malloc`; null, the box of line
/// 0x4c8 and 0) in `wc_turnover_consts::ENTRANT_LIST`, the indices (short, the box of line 0x4d0 and 0) turned back
/// into clubs (`club_indices_to_pointers`). Returns 1.
/// Original: `WcOceaniaLeague::load_extra_data` (0x91fd20, wc_oceania_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_load_extra_data(this: usize, file: usize) -> u32 {
    let cpp = wc_turnover_consts::WC_OCEANIA_LEAGUE_CPP;
    if exe::crt_fread(this + wc_oceania_league_fields::EXEMPT_COUNT, 1, 1, file) != 1 {
        show_error_at(cpp, 0x4b8);
        return 0;
    }
    if exe::crt_fread(this + 0xf3, 4, 1, file) != 1 {
        show_error_at(cpp, 0x4c0);
        return 0;
    }
    let n = read_u32(this + 0xf3);
    let pool = exe::crt_malloc(n << 2);
    write_usize(this + wc_turnover_consts::ENTRANT_LIST, pool);
    if pool == 0 {
        show_error_at(cpp, 0x4c8);
        return 0;
    }
    if exe::crt_fread(pool, 4, n, file) != n {
        show_error_at(cpp, 0x4d0);
        return 0;
    }
    covered!("cx16_6 oceania qualifying loaded");
    club_indices_to_pointers(pool, n as i32);
    1
}

/// Comparer of the qualifying sections' list readers: the original reads first; the file goes back to where it was
/// and the copy reads the same bytes again into a list of its own; the returns, the `fields` bytes, the list's entries
/// (`count` of them, a dword at `count_at`) and the file positions must match. The copy's list is freed and the
/// original's state put back.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_qualifying_list_load(name: &str, this: usize, file: usize, fields: &[(usize, usize)], list_at: usize, count_at: usize, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let snapshot = |r: u32| {
        let bytes: Vec<u8> = fields.iter().flat_map(|&(o, n)| core::slice::from_raw_parts((this + o) as *const u8, n).to_vec()).collect();
        let (list, n) = (read_usize(this + list_at), read_u32(this + count_at) as i32);
        let entries: Vec<usize> = if list != 0 && r == 1 { (0..n.clamp(0, 256) as usize).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
        (r, bytes, entries)
    };
    let at = exe::crt_ftell(file);
    let r = original();
    let end_o = exe::crt_ftell(file);
    let a = snapshot(r);
    let list = read_usize(this + list_at);
    exe::crt_fseek(file, at, 0);
    let c = copy();
    let end_c = exe::crt_ftell(file);
    let b = snapshot(c);
    let list2 = read_usize(this + list_at);
    if list2 != 0 && list2 != list {
        exe::crt_free(list2);
    }
    write_usize(this + list_at, list);
    let mut k = 0;
    for &(o, n) in fields {
        core::ptr::copy_nonoverlapping(a.1.as_ptr().add(k), (this + o) as *mut u8, n);
        k += n;
    }
    exe::crt_fseek(file, end_o, 0);
    SCOREBOARD.record(a == b && end_o == end_c, || format!("{}({:#x}): original {:x?} at {}, copy {:x?} at {} (from {})", name, this, a, end_o, b, end_c, at));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_qualifying_list_load("wc_concacaf_cup_load_extra_data", this, file, &[(0xb6, 4), (0xba, 3)], 0xb2, 0xb6, || o(this, file), || wc_concacaf_cup_load_extra_data(this, file))
}

/// Comparer for `wc_concacaf_cup_save_extra_data`: it writes the saved game (it cannot run twice), so only the
/// original runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    compare_qualifying_list_load("wc_oceania_league_load_extra_data", this, file, &[(wc_oceania_league_fields::EXEMPT_COUNT, 1), (0xf3, 4)], wc_turnover_consts::ENTRANT_LIST, 0xf3,
        || o(this, file), || wc_oceania_league_load_extra_data(this, file))
}
