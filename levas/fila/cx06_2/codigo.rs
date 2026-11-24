// ---------- cx06_2: the European Championship qualifiers' own data in the saved game (euro_champ_qual.cpp) ----------

/// `fgetpos` of the CRT (cdecl: the stream and the position to fill, 8 bytes); 0 on success.
const FN_CRT_FGETPOS: usize = 0x946c1e;
/// `fsetpos` of the CRT (cdecl: the stream and the position to go back to, 8 bytes); 0 on success.
const FN_CRT_FSETPOS: usize = 0x946c06;

/// Reads the European Championship qualifiers' own data from the saved game: the count of stored group results, each
/// result (a fixture record, `fixture::SIZE` bytes) with its pointers rebuilt from the ids (the home and away clubs
/// `club_at`, the competition record `club_comp_record`, the stadium `G_STADIUM_TABLE` + `stadium::SIZE` × id or null for
/// −1) and added to the results list (+0x10, `exe::llist_add_tail`), then the hosts count (+0xf6, 1 byte) and the
/// qualified flag (+0xfb, 4 bytes). A short read, or a result with no home club, away club or competition (id −1), shows
/// the euro_champ_qual.cpp box (lines 0x64a, 0x652, 0x65a, 0x661, 0x669, 0x683, 0x68a) and returns 0; otherwise 1.
/// Original: `EuroChampQual::load_extra_data` (0x580130, euro_champ_qual.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_load_extra_data(this: usize, file: usize) -> u32 {
    let mut count = 0i32;
    if exe::crt_fread(&mut count as *mut i32 as usize, 4, 1, file) != 1 {
        show_error_at(0x9c4674, 0x64a); // euro_champ_qual.cpp
        return 0;
    }
    let mut result = [0u8; fixture::SIZE];
    let f = result.as_mut_ptr() as usize;
    for _ in 0..count.max(0) {
        if exe::crt_fread(f, fixture::SIZE as u32, 1, file) != 1 {
            show_error_at(0x9c4674, 0x652);
            return 0;
        }
        let home = read_i32(f + fixture::HOME_CLUB_ID);
        if home == -1 {
            show_error_at(0x9c4674, 0x65a);
            return 0;
        }
        write_usize(f + fixture::HOME_CLUB, club_at(home as isize).addr());
        let away = read_i32(f + fixture::AWAY_CLUB_ID);
        if away == -1 {
            show_error_at(0x9c4674, 0x661);
            return 0;
        }
        write_usize(f + fixture::AWAY_CLUB, club_at(away as isize).addr());
        let comp = read_i32(f + fixture::COMP_ID);
        if comp == -1 {
            show_error_at(0x9c4674, 0x669);
            return 0;
        }
        write_usize(f + fixture::COMPETITION, club_comp_record(comp as usize));
        let stadium = read_i32(f + fixture::STADIUM_ID);
        write_usize(f + fixture::STADIUM, if stadium == -1 { 0 } else { read_usize(G_STADIUM_TABLE).wrapping_add((stadium as usize).wrapping_mul(stadium::SIZE)) });
        covered!("cx06_2 qualifiers result loaded");
        exe::llist_add_tail(read_usize(this + 0x10), f, fixture::SIZE as u32); // the stored group results
    }
    covered!("cx06_2 qualifiers data loaded");
    if exe::crt_fread(this + euro_champ_qual::GROUP_FORMAT, 1, 1, file) != 1 {
        show_error_at(0x9c4674, 0x683);
        return 0;
    }
    if exe::crt_fread(this + 0xfb, 4, 1, file) != 1 { // the qualified flag
        show_error_at(0x9c4674, 0x68a);
        return 0;
    }
    1
}

/// Writes the European Championship qualifiers' own data to the saved game: a count placeholder (0), each stored group
/// result (the results list +0x10 from its head, `list_rewind` and `exe::llist_get_next`; `fixture::SIZE` bytes each,
/// unchecked), then the real count over the placeholder (the stream position saved and restored with fgetpos/fsetpos),
/// the hosts count (+0xf6, 1 byte) and the qualified flag (+0xfb, 4 bytes). A failed position or write shows the
/// euro_champ_qual.cpp box (lines 0x6a2, 0x6ac, 0x6bb, 0x6c2, 0x6c7, 0x6ce, 0x6d5, 0x6dc) and returns 0 (the original
/// leaves the box's eax); otherwise 1 (the last write's count).
/// Original: `EuroChampQual::save_extra_data` (0x580410, euro_champ_qual.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_save_extra_data(this: usize, file: usize) -> u32 {
    let getpos: unsafe extern "C" fn(usize, *mut i64) -> i32 = core::mem::transmute(FN_CRT_FGETPOS);
    let setpos: unsafe extern "C" fn(usize, *const i64) -> i32 = core::mem::transmute(FN_CRT_FSETPOS);
    let mut count = 0i32;
    let (mut count_at, mut end) = (0i64, 0i64);
    let fail = |row: u32| {
        show_error_at(0x9c4674, row); // euro_champ_qual.cpp
        0
    };
    if getpos(file, &mut count_at) != 0 {
        return fail(0x6a2);
    }
    if exe::crt_fwrite(&count as *const i32 as usize, 4, 1, file) != 1 {
        return fail(0x6ac);
    }
    let results = read_usize(this + 0x10); // the stored group results
    list_rewind(results);
    loop {
        let item = exe::llist_get_next(results);
        if item == 0 {
            break;
        }
        covered!("cx06_2 qualifiers result saved");
        exe::crt_fwrite(item, fixture::SIZE as u32, 1, file);
        count += 1;
    }
    if getpos(file, &mut end) != 0 {
        return fail(0x6bb);
    }
    if setpos(file, &count_at) != 0 {
        return fail(0x6c2);
    }
    if exe::crt_fwrite(&count as *const i32 as usize, 4, 1, file) != 1 {
        return fail(0x6c7);
    }
    if setpos(file, &end) != 0 {
        return fail(0x6ce);
    }
    covered!("cx06_2 qualifiers data saved");
    if exe::crt_fwrite(this + euro_champ_qual::GROUP_FORMAT, 1, 1, file) != 1 {
        return fail(0x6d5);
    }
    if exe::crt_fwrite(this + 0xfb, 4, 1, file) != 1 { // the qualified flag
        return fail(0x6dc);
    }
    1
}

/// Comparer for `euro_champ_qual_load_extra_data`: the reading adds to the results list (it cannot run twice), so only
/// the original runs; the plan: on success the stream moved by the count, the results (`fixture::SIZE` each) and the 5
/// trailing bytes, and the list grew by the count read.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_load_extra_data(this: usize, file: usize) -> u32 {
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
    exe::crt_fseek(file, at, 0);
    exe::crt_fread(&mut count as *mut i32 as usize, 4, 1, file);
    exe::crt_fseek(file, end, 0);
    let added = read_u16(results + llist::COUNT).wrapping_sub(before) as i32;
    let moved = end.wrapping_sub(at) as i32;
    SCOREBOARD.record(added == count && moved == 4 + count * fixture::SIZE as i32 + 5, || format!("euro_champ_qual_load_extra_data({:#x}): {} results read, {} added, file moved {}", this, count, added, moved));
    r
}

/// Comparer for `euro_champ_qual_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}
