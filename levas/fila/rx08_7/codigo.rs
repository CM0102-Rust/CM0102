// ---------- rx08_7: the European World Cup qualifying's own data in the saved game (wc_europe_league.cpp) ----------

/// The European qualifying's error box at `row` (wc_europe_league.cpp); 0 for the caller to return.
unsafe fn wc_europe_league_file_error(row: u32) -> u32 {
    show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, row);
    0
}

/// Rebuilds the pointers of a saved fixture record from its ids: the home and away clubs (`club_at`), the competition
/// record (`club_comp_record`) and the stadium (`G_STADIUM_TABLE` + `stadium::SIZE` × id, or null for −1). A home, away
/// or competition id of −1 gives the box row that rejects it (0x6c4, 0x6cb, 0x6d3).
unsafe fn wc_europe_league_rebuild_fixture(f: usize) -> Result<(), u32> {
    let home = read_i32(f + fixture::HOME_CLUB_ID);
    if home == -1 {
        return Err(0x6c4);
    }
    write_usize(f + fixture::HOME_CLUB, club_at(home as isize).addr());
    let away = read_i32(f + fixture::AWAY_CLUB_ID);
    if away == -1 {
        return Err(0x6cb);
    }
    write_usize(f + fixture::AWAY_CLUB, club_at(away as isize).addr());
    let comp = read_i32(f + fixture::COMP_ID);
    if comp == -1 {
        return Err(0x6d3);
    }
    write_usize(f + fixture::COMPETITION, club_comp_record(comp as usize));
    let stadium = read_i32(f + fixture::STADIUM_ID);
    write_usize(f + fixture::STADIUM, if stadium == -1 { 0 } else { read_usize(G_STADIUM_TABLE).wrapping_add((stadium as usize).wrapping_mul(stadium::SIZE)) });
    Ok(())
}

/// Reads the European qualifying's own data from the saved game (`exe::crt_fread`, cdecl).
///
/// The exempt count (`WC_EUROPE_EXEMPT`, a byte; short: the error box of line 0x693 and 0), which sets the format
/// bytes (`WC_EUROPE_LAST_GROUP` and the next two: 8, 9, 10 with none exempt; 7, 8, 9 with one or two; 6, 7, 8 with
/// more); the count of the stored group results (short: 0x6b4) and each result (a fixture record, `fixture::SIZE` bytes;
/// short: 0x6bc) with its pointers rebuilt (`wc_europe_league_rebuild_fixture`; a missing id: its box and 0) and added
/// to the results list (+0x10, `exe::llist_add_tail`); the done flag (+0xfb, 4 bytes; short: 0x6ed); the pool count
/// (+0xf2, 4 bytes; short: 0x6f5): zero ends here with 1; otherwise the pool (`exe::crt_malloc` of 4 × count into +0xee;
/// none: 0x700), its indices (short: 0x708) turned back into clubs (`club_indices_to_pointers`). Returns 1.
/// Original: `WcEuropeLeague::load_extra_data` (0x91a9f0, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + WC_EUROPE_EXEMPT, 1, 1, file) != 1 {
        return wc_europe_league_file_error(0x693);
    }
    let format: [u8; 3] = match read_u8(this + WC_EUROPE_EXEMPT) {
        0 => [8, 9, 0xa],
        1 | 2 => [7, 8, 9],
        _ => [6, 7, 8],
    };
    for (k, v) in format.into_iter().enumerate() {
        write_u8(this + WC_EUROPE_LAST_GROUP + k, v);
    }
    let mut count = 0i32;
    if exe::crt_fread(&mut count as *mut i32 as usize, 4, 1, file) != 1 {
        return wc_europe_league_file_error(0x6b4);
    }
    let mut result = [0u8; fixture::SIZE];
    let f = result.as_mut_ptr() as usize;
    for _ in 0..count.max(0) {
        if exe::crt_fread(f, fixture::SIZE as u32, 1, file) != 1 {
            return wc_europe_league_file_error(0x6bc);
        }
        if let Err(row) = wc_europe_league_rebuild_fixture(f) {
            return wc_europe_league_file_error(row);
        }
        covered!("rx08_7 european result loaded");
        exe::llist_add_tail(read_usize(this + 0x10), f, fixture::SIZE as u32); // rx08_7: the stored group results
    }
    if exe::crt_fread(this + 0xfb, 4, 1, file) != 1 {
        return wc_europe_league_file_error(0x6ed);
    }
    if exe::crt_fread(this + 0xf2, 4, 1, file) != 1 {
        return wc_europe_league_file_error(0x6f5);
    }
    covered!("rx08_7 european data loaded");
    let n = read_u32(this + 0xf2);
    if n == 0 {
        return 1;
    }
    let pool = exe::crt_malloc(n.wrapping_mul(4));
    write_usize(this + 0xee, pool);
    if pool == 0 {
        return wc_europe_league_file_error(0x700);
    }
    if exe::crt_fread(pool, 4, n, file) != n {
        return wc_europe_league_file_error(0x708);
    }
    club_indices_to_pointers(pool, n as i32);
    1
}

/// Writes the European qualifying's own data to the saved game, in the order the load reads it: the exempt count
/// (unchecked), a count placeholder (0) whose position is kept (`FN_CRT_FGETPOS`), each stored group result (the results
/// list +0x10 from its head, `list_rewind` and `exe::llist_get_next`; `fixture::SIZE` bytes each, unchecked), then the
/// real count over the placeholder (`FN_CRT_FSETPOS` there and back), the done flag (+0xfb), the pool count (+0xf2,
/// unchecked) and the pool as club indices (`club_pointers_to_indices`, written unchecked, then back,
/// `club_indices_to_pointers`). A failed position or checked write shows the wc_europe_league.cpp box (lines 0x733,
/// 0x73d, 0x74c, 0x753, 0x758, 0x75f, 0x766) and returns 0 (the original leaves the box's eax). Returns the pool count.
/// Original: `WcEuropeLeague::save_extra_data` (0x91ae80, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_save_extra_data(this: usize, file: usize) -> u32 {
    let getpos: unsafe extern "C" fn(usize, *mut i64) -> i32 = core::mem::transmute(FN_CRT_FGETPOS);
    let setpos: unsafe extern "C" fn(usize, *const i64) -> i32 = core::mem::transmute(FN_CRT_FSETPOS);
    let mut count = 0i32;
    let (mut count_at, mut end) = (0i64, 0i64);
    exe::crt_fwrite(this + WC_EUROPE_EXEMPT, 1, 1, file);
    if getpos(file, &mut count_at) != 0 {
        return wc_europe_league_file_error(0x733);
    }
    if exe::crt_fwrite(&count as *const i32 as usize, 4, 1, file) != 1 {
        return wc_europe_league_file_error(0x73d);
    }
    let results = read_usize(this + 0x10);
    list_rewind(results);
    loop {
        let item = exe::llist_get_next(results);
        if item == 0 {
            break;
        }
        covered!("rx08_7 european result saved");
        exe::crt_fwrite(item, fixture::SIZE as u32, 1, file);
        count += 1; // rx08_7: one more stored result
    }
    if getpos(file, &mut end) != 0 {
        return wc_europe_league_file_error(0x74c);
    }
    if setpos(file, &count_at) != 0 {
        return wc_europe_league_file_error(0x753);
    }
    if exe::crt_fwrite(&count as *const i32 as usize, 4, 1, file) != 1 {
        return wc_europe_league_file_error(0x758);
    }
    if setpos(file, &end) != 0 {
        return wc_europe_league_file_error(0x75f);
    }
    if exe::crt_fwrite(this + 0xfb, 4, 1, file) != 1 {
        return wc_europe_league_file_error(0x766);
    }
    covered!("rx08_7 european data saved");
    exe::crt_fwrite(this + 0xf2, 4, 1, file);
    let (pool, n) = (read_usize(this + 0xee), read_u32(this + 0xf2));
    club_pointers_to_indices(pool, n as i32);
    exe::crt_fwrite(pool, 4, n, file);
    club_indices_to_pointers(pool, n as i32);
    n
}

/// Comparer for `wc_europe_league_load_extra_data`: the reading adds to the results list and allocates the pool (it
/// cannot run twice), so only the original runs; the plan: on success the stream moved by the exempt byte, the count,
/// the results, the flag, the pool count and the pool, the list grew by the count read and the format bytes follow the
/// exempt count.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    let results = read_usize(this + 0x10);
    let at = exe::crt_ftell(file);
    let before = if results != 0 { read_u16(results + llist::COUNT) } else { 0 };
    let r = o(this, file);
    if r != 1 || results == 0 {
        return r;
    }
    let end = exe::crt_ftell(file);
    let mut count = 0i32;
    exe::crt_fseek(file, at + 1, 0);
    exe::crt_fread(&mut count as *mut i32 as usize, 4, 1, file);
    exe::crt_fseek(file, end, 0);
    let added = read_u16(results + llist::COUNT).wrapping_sub(before) as i32;
    let moved = end.wrapping_sub(at);
    let expected = 1 + 4 + count * fixture::SIZE as i32 + 4 + 4 + read_i32(this + 0xf2) * 4;
    let format = match read_u8(this + WC_EUROPE_EXEMPT) { 0 => 8u8, 1 | 2 => 7, _ => 6 };
    let ok = added == count && moved == expected && read_u8(this + WC_EUROPE_LAST_GROUP) == format;
    SCOREBOARD.record(ok, || format!("wc_europe_league_load_extra_data({:#x}): {} results read, {} added, file moved {} (expected {})", this, count, added, moved, expected));
    r
}

/// Comparer for `wc_europe_league_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}
