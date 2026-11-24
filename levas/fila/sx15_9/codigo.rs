// ---------- batch %%LEVA%%: the national teams object written to the saved game and destroyed (national_teams.cpp) ----------

/// `FIFA_RANKINGS::save(save)` (thiscall, ret 4): the rankings' own item of the saved game.
const FN_FIFA_RANKINGS_SAVE: usize = 0x58dae0;
/// `FIFA_RANKINGS::~FIFA_RANKINGS()` (thiscall, ret 0).
const FN_FIFA_RANKINGS_DESTRUCTOR: usize = 0x58cf20;

/// One reserve pool written to the saved game: its size (4 bytes from `this + count_field`), then the table at
/// `this + pool_field` with each person turned into his id (none: −1), written as size dwords and turned back into
/// persons (−1: none). False after the national_teams.cpp box of the failing step (`lines`: the size, the ids), with
/// the table left as ids when the ids failed.
unsafe fn save_reserve_pool(this: usize, count_field: usize, pool_field: usize, z: usize, lines: [u32; 2]) -> bool {
    if exe::zwrite(this + count_field, 4, 1, z) != 1 {
        show_error_at(NATIONAL_TEAMS_CPP, lines[0]);
        return false;
    }
    let pool = read_usize(this + pool_field);
    for k in 0..read_i32(this + count_field).max(0) as usize {
        let person = read_usize(pool + k * 4);
        write_i32(pool + k * 4, if person != 0 { read_i32(person) } else { -1 }); // the person's id (+0)
    }
    let count = read_u32(this + count_field);
    if exe::zwrite(read_usize(this + pool_field), 4, count, z) != count {
        show_error_at(NATIONAL_TEAMS_CPP, lines[1]);
        return false;
    }
    let pool = read_usize(this + pool_field);
    for k in 0..(count as i32).max(0) as usize {
        let id = read_i32(pool + k * 4);
        write_usize(pool + k * 4, if id != -1 { staff_at(id as isize).addr() } else { 0 });
    }
    true
}

/// Writes the national teams object into the saved game.
///
/// `NationalTeams::save_dat(save)`: the "national_teams.dat" item (`zopen` with 1, 0x16, 4); then each step checked, a
/// failure showing the national_teams.cpp box of its line and returning with the item left open:
/// - the nations' records (`NATION_RECORDS`, nations × 0x6e bytes; open: line 0x2046, write: 0x204e);
/// - the senior reserve pool (size +0x14, table +0xc; lines 0x2055, 0x206e) and the under-21 one (size +0x18, table
///   +0x10; lines 0x2086, 0x209f), as person ids (`save_reserve_pool`);
/// - the call-up count (2 bytes: the list's count, 0 without a list; line 0x20bf) and, with a list, every 0x12-byte
///   call-up from its head (`LLIST::get_next`, the cursor reset before; line 0x20cd);
/// - the club call-up records (`CLUB_CALL_UP_RECORDS`, 6 bytes per club without the national sides; line 0x20d6) and,
///   for each record with entries (+5), its vector (+0) of that many 9-byte entries (line 0x20e3);
/// - the next competitions (`NEXT_COMPETITIONS`, 4 bytes per nation; line 0x20ec) and the Olympic marks
///   (`OLYMPIC_NATION_MARKS`, 1 byte per nation; line 0x20f3).
/// The item closed, then the FIFA rankings write their own item (`FIFA_RANKINGS::save(save)`).
/// Original: `NationalTeams::save_dat` (0x776a20, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nationalteams_save_dat(this: usize, save: usize) {
    use national_teams_fields as f;
    let clubs = club_call_up_record_count();
    let mut z = exe::zopen(save, NATIONAL_TEAMS_DAT, 1, 0x16, 4);
    let fail = |line: u32| show_error_at(NATIONAL_TEAMS_CPP, line);
    if z == 0 {
        return fail(0x2046);
    }
    covered!("sx15_9 save national teams");
    if exe::zwrite(read_usize(this + f::NATION_RECORDS), 0x6e, read_u32(G_NATION_COUNT), z) != read_u32(G_NATION_COUNT) {
        return fail(0x204e);
    }
    // the two reserve pools: sizes at +0x14 and +0x18, tables at RESERVE_POOLS (+0xc) and +0x10
    if !save_reserve_pool(this, 0x14, f::RESERVE_POOLS, z, [0x2055, 0x206e])
        || !save_reserve_pool(this, 0x18, f::RESERVE_POOLS + 4, z, [0x2086, 0x209f])
    {
        return;
    }
    let mut count = 0u32;
    let list = read_usize(this + f::CALL_UP_LIST);
    if list != 0 {
        count = read_u16(list + llist::COUNT) as u32;
        write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    }
    if exe::zwrite(&mut count as *mut u32 as usize, 2, 1, z) != 1 {
        return fail(0x20bf);
    }
    let list = read_usize(this + f::CALL_UP_LIST);
    if list != 0 {
        write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
        let mut call_up = exe::llist_get_next(read_usize(this + f::CALL_UP_LIST));
        while call_up != 0 {
            if exe::zwrite(call_up, 0x12, 1, z) != 1 {
                return fail(0x20cd);
            }
            call_up = exe::llist_get_next(read_usize(this + f::CALL_UP_LIST));
        }
    }
    if exe::zwrite(read_usize(this + f::CLUB_CALL_UP_RECORDS), 6, clubs as u32, z) != clubs as u32 {
        return fail(0x20d6);
    }
    for k in 0..clubs.max(0) as usize {
        let rec = read_usize(this + f::CLUB_CALL_UP_RECORDS) + k * 6;
        let entries = read_i8(rec + 5) as i32;
        if entries != 0 && exe::zwrite(read_usize(rec), 9, entries as u32, z) != entries as u32 {
            return fail(0x20e3);
        }
    }
    if exe::zwrite(read_usize(this + f::NEXT_COMPETITIONS), 4, read_u32(G_NATION_COUNT), z) != read_u32(G_NATION_COUNT) {
        return fail(0x20ec);
    }
    let marks = read_u32(G_NATION_COUNT); // sx15_9: the Olympic marks, one byte per nation
    if exe::zwrite(read_usize(this + f::OLYMPIC_NATION_MARKS), 1, marks, z) != marks {
        return fail(0x20f3);
    }
    exe::zclose(&mut z);
    let rankings_save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIFA_RANKINGS_SAVE);
    rankings_save(read_usize(this + f::FIFA_RANKING), save);
}

/// Comparer for `nationalteams_save_dat`: it writes the save's item (it cannot run twice), so only the original runs;
/// the copy is proven by the save proof (the .qst written with and without the batch).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_save_dat(this: usize, save: usize) {
    // ponytail: writes the container once; the save proof compares the .qst
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, save)
}

/// Frees what the national teams object owns, without clearing the fields.
///
/// `NationalTeams::~NationalTeams()`: the FIFA rankings (`FIFA_RANKING`: destructor and `delete`), the call-up list
/// (`CALL_UP_LIST`: `LLIST::~LLIST` and `delete`), then `free` of the nation records, the next competitions and the two
/// reserve pools; each club call-up record's vector and then the records (`CLUB_CALL_UP_RECORDS`, the club count read
/// again at each step); the club marks (`CLUB_MARKS`) and the Olympic marks; last, the inline new qualifiers list
/// (`NEW_QUALIFIERS`, `LLIST::~LLIST`). Run by free_game_data (0x8301ca) when a game is closed (another one loaded,
/// back to the menu).
/// Original: `NationalTeams::~NationalTeams` (0x76a710, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn nationalteams_destructor(this: usize) {
    use national_teams_fields as f;
    let rankings = read_usize(this + f::FIFA_RANKING);
    if rankings != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FIFA_RANKINGS_DESTRUCTOR);
        destroy(rankings);
        exe::crt_operator_delete(rankings);
    }
    let list = read_usize(this + f::CALL_UP_LIST);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    for field in [f::NATION_RECORDS, f::NEXT_COMPETITIONS, f::RESERVE_POOLS, f::RESERVE_POOLS + 4] {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
        }
    }
    if read_usize(this + f::CLUB_CALL_UP_RECORDS) != 0 {
        covered!("sx15_9 national teams destroyed");
        let mut k = 0;
        while k < club_call_up_record_count() {
            let vector = read_usize(read_usize(this + f::CLUB_CALL_UP_RECORDS) + k as usize * 6);
            if vector != 0 {
                exe::crt_free(vector);
            }
            k += 1;
        }
        exe::crt_free(read_usize(this + f::CLUB_CALL_UP_RECORDS));
    }
    for field in [f::CLUB_MARKS, f::OLYMPIC_NATION_MARKS] {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
        }
    }
    exe::drop_llist(this + f::NEW_QUALIFIERS);
}

/// Comparer for `nationalteams_destructor`: it frees the tables (it cannot run twice), so only the original runs; the
/// copy runs in the save proof when the holiday save is loaded over a game (free_game_data destroys the object first).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_destructor(this: usize) {
    // ponytail: frees once; nothing to compare after it
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}
