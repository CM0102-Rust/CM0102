// ---------- batch %%LEVA%%: the club history lists of the club screens: league seasons, cup runs, their competitions and the current league record (club_history.cpp) ----------

/// The club history store as the club screens read it (`G_CLUB_HISTORY`): +0 the league history file list, +4 the cup
/// history file list, +8 the per-club records (16 bytes, the club id the index: +0/+4 the first and last file offsets
/// of the club's league seasons, +8/+0xc those of its cup runs).
pub(crate) mod club_history_lists {
    pub(crate) const LEAGUE_LIST: usize = 0x0;
    pub(crate) const CUP_LIST: usize = 0x4;
    pub(crate) const RECORDS: usize = 0x8;
    pub(crate) const RECORD_SIZE: usize = 0x10;
    pub(crate) const LEAGUE_FIRST: usize = 0x0;
    pub(crate) const LEAGUE_LAST: usize = 0x4;
    pub(crate) const CUP_FIRST: usize = 0x8;
    pub(crate) const CUP_LAST: usize = 0xc;
    /// A league season record (+0 comp id, +4 year, +6 position, +7 the 14 bytes of the table totals) and a cup run
    /// record (+0 comp id, +4 year, +6 status, +7 stage label, +9 round code).
    pub(crate) const LEAGUE_ENTRY_SIZE: u32 = 0x15;
    pub(crate) const CUP_ENTRY_SIZE: u32 = 0xb;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\club_history.cpp" (the error boxes).
    pub(crate) const CLUB_HISTORY_CPP: usize = 0x98b644;
    /// The qsort comparison of the competition ids (`compare_int_ids`).
    pub(crate) const COMPARE_INT_IDS: usize = 0x444500;
}

/// A new empty list (`operator new(0xe)` and `LLIST::LLIST`), or 0 when the allocation fails.
unsafe fn club_history_new_list() -> usize {
    let p = exe::crt_operator_new(0xe);
    if p != 0 { exe::new_llist(p) } else { 0 }
}

/// The stored records of a club's file list from the newest to the oldest, appended to `list` (created on the first
/// one); returns the list.
unsafe fn club_history_append_stored(mut list: usize, file_list: usize, first: usize, last: usize, size: u32) -> usize {
    let nodes = exe::file_llist_get_nodes(file_list, read_u32(first), read_u32(last));
    if nodes == 0 {
        return list;
    }
    write_usize(nodes + llist::CURSOR, read_usize(nodes + llist::TAIL));
    loop {
        let data = exe::llist_get_prev(nodes);
        if data == 0 {
            break;
        }
        if list == 0 {
            list = club_history_new_list();
        }
        exe::llist_add_tail(list, data, size);
    }
    exe::drop_llist(nodes);
    exe::crt_operator_delete(nodes);
    list
}

/// Builds the list of a club's league seasons, newest first, with the current season first when it is not over.
///
/// `Club_History::get_league_history(club)`: no club: the club_history.cpp 0x184 box and 0. The current record
/// (`club_history_get_current_league_record` with no stage and no reserve league): found and its league not complete,
/// a new list gets it. Then the club's stored seasons (`FILE_LLIST::get_nodes` of the league list with the club's
/// offsets), from the newest (the tail) back, each appended (21 bytes; the list created on the first one); the node
/// list destroyed. Returns the list or 0.
/// Original: `Club_History::get_league_history` (0x443380, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_get_league_history(this: usize, club: usize) -> usize {
    use club_history_lists as ch;
    if club == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x184);
        return 0;
    }
    let mut list = 0usize;
    let mut rec = [0u8; 0x15];
    // simplification: the original's flag is a stack slot it never sets; a record found in a stage the club has not
    // played yet leaves it unwritten (there the copy takes 0: the record is shown)
    let mut complete = 0i32;
    if club_history_get_current_league_record(club, rec.as_mut_ptr(), &mut complete, core::ptr::null_mut(), 0) != 0
        && complete == 0 // tel42_3: the season not over
    {
        covered!("tel42_3 current league season");
        list = club_history_new_list();
        exe::llist_add_tail(list, rec.as_ptr() as usize, ch::LEAGUE_ENTRY_SIZE);
    }
    let r = read_usize(this + ch::RECORDS) + read_u32(club) as usize * ch::RECORD_SIZE;
    club_history_append_stored(list, read_usize(this + ch::LEAGUE_LIST), r + ch::LEAGUE_FIRST, r + ch::LEAGUE_LAST, ch::LEAGUE_ENTRY_SIZE)
}

/// Builds the list of a club's cup runs: its progress in each running cup first, then the stored runs, newest first.
///
/// `Club_History::get_cup_history(club)`: no club: the club_history.cpp 0x1d0 box and 0. Each competition object
/// (`G_COMP_OBJECT_TABLE`, `G_COMP_OBJECT_COUNT`) that is not a league (virtual +0xc), not a league division
/// (`comp_util_is_league_division`) and not a friendly competition (`Friendly::is_friendly_competition`): a cup record
/// (competition id, year, status 0, stage -1, round -1) filled by `Comp::find_club_progress`; with a stage reached
/// (≥ 0) it is appended (11 bytes; the list created on the first one). Then the club's stored runs (the cup list with
/// the club's +8/+0xc offsets) from the newest back. Returns the list or 0.
/// Original: `Club_History::get_cup_history` (0x443520, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_get_cup_history(this: usize, club: usize) -> usize {
    use club_history_lists as ch;
    if club == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x1d0);
        return 0;
    }
    let mut list = 0usize;
    for i in 0..read_i32(G_COMP_OBJECT_COUNT).max(0) as usize {
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE) + i * 4);
        if comp == 0 {
            continue;
        }
        let is_league: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(read_usize(read_usize(comp) + 0xc));
        if is_league(comp) != 0 {
            continue;
        }
        let competition = read_usize(comp + league::COMPETITION);
        if exe::comp_util_is_league_division(read_u32(competition) as i32) != 0
            || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), competition) != 0
        {
            continue;
        }
        let mut rec = [0u8; 0xb];
        rec[0..4].copy_from_slice(&read_u32(competition).to_le_bytes());
        rec[4..6].copy_from_slice(&read_u16(comp + league::YEAR).to_le_bytes());
        rec[7..0xb].fill(0xff);
        let p = rec.as_mut_ptr();
        find_club_progress(comp, club, p.add(7) as *mut i16, p.add(9) as *mut i16, p.add(6));
        if i16::from_le_bytes([rec[7], rec[8]]) > -1 { // tel42_3: a stage reached
            covered!("tel42_3 running cup progress");
            if list == 0 {
                list = club_history_new_list();
            }
            exe::llist_add_tail(list, rec.as_ptr() as usize, ch::CUP_ENTRY_SIZE);
        }
    }
    let r = read_usize(this + ch::RECORDS) + read_u32(club) as usize * ch::RECORD_SIZE;
    club_history_append_stored(list, read_usize(this + ch::CUP_LIST), r + ch::CUP_FIRST, r + ch::CUP_LAST, ch::CUP_ENTRY_SIZE)
}

/// The distinct competition ids of a history list (the first dword of each record), sorted with `compare_int_ids`, as
/// a new list of ints; the two `operator new` failures give the boxes at the lines given.
unsafe fn club_history_comp_ids(history: usize, lines: [u32; 3]) -> usize {
    use club_history_lists as ch;
    if history == 0 {
        return 0;
    }
    let count = read_u32(G_CLUB_COMP_COUNT);
    let ids = exe::crt_operator_new(count.wrapping_mul(4));
    if ids == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, lines[0]);
        return 0;
    }
    let seen = exe::crt_operator_new(count.wrapping_mul(4));
    if seen == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, lines[1]);
        exe::crt_operator_delete(ids);
        return 0;
    }
    for k in 0..read_i32(G_CLUB_COMP_COUNT).max(0) as usize {
        write_u32(seen + k * 4, 0);
    }
    let mut n = 0usize;
    write_usize(history + llist::CURSOR, read_usize(history + llist::HEAD));
    loop {
        let data = exe::llist_get_next(history);
        if data == 0 {
            break;
        }
        let id = read_u32(data) as usize;
        if read_u32(seen + id * 4) == 0 { // tel42_3: each competition once
            write_u32(ids + n * 4, id as u32);
            n += 1;
            write_u32(seen + id * 4, 1);
        }
    }
    exe::crt_operator_delete(seen);
    // the original's comparison never orders (1 for any two different ids): the order is whatever qsort leaves
    exe::crt_qsort(ids, n as u32, 4, ch::COMPARE_INT_IDS);
    let mut list = 0usize;
    for k in 0..n {
        if list == 0 {
            list = club_history_new_list();
            if list == 0 {
                show_error_at(ch::CLUB_HISTORY_CPP, lines[2]);
                exe::crt_operator_delete(ids);
                return 0;
            }
        }
        exe::llist_add_tail(list, ids + k * 4, 4);
    }
    exe::crt_operator_delete(ids);
    list
}

/// Collects the distinct competition ids of a club's league history list, "sorted", as a new list of ints.
///
/// `Club_History::get_league_comp_ids(history)`: no list: 0. Two tables of `G_CLUB_COMP_COUNT` ints (`operator new`;
/// none: the club_history.cpp 0x238 / 0x243 box and 0), the seen marks zeroed; the list walked from its head
/// (`LLIST::get_next`), each competition id (the record's first dword) not seen yet kept and marked; the marks freed;
/// the ids sorted (`qsort` with `compare_int_ids`, which returns 1 for any two different ids: no real order); each one
/// appended (4 bytes) to a new list (none: the 0x26e box and 0). The table freed. Returns the list or 0.
/// Original: `Club_History::get_league_comp_ids` (0x443770, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_get_league_comp_ids(_this: usize, history: usize) -> usize {
    club_history_comp_ids(history, [0x238, 0x243, 0x26e])
}

/// Collects the distinct competition ids of a club's cup history list, "sorted", as a new list of ints.
///
/// `Club_History::get_cup_comp_ids(history)`: as `club_history_get_league_comp_ids`, with the boxes at lines 0x298,
/// 0x2a3 and 0x2ce.
/// Original: `Club_History::get_cup_comp_ids` (0x443a20, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_get_cup_comp_ids(_this: usize, history: usize) -> usize {
    club_history_comp_ids(history, [0x298, 0x2a3, 0x2ce])
}

/// Finds a club in the current table of its division and fills a league history record with its position and totals.
///
/// `club_history_get_current_league_record(club, rec, complete, stage, reserve)`: `*stage` = 0 when given; no club: the
/// club_history.cpp 0x36a box and 0. The division is the club's (`club::DIVISION`), or its reserve division
/// (`club::RESERVE_DIVISION`) for a Brazilian club (`club::NATION` = Brazil) with `reserve` set while today
/// (`G_CURRENT_DATE`, the day only) is after 1 February and before 20 June (the state championships). No division or
/// no competition object for it (`G_COMP_OBJECT_TABLE`): 0. From the league's current stage (the low byte of
/// `league::CURRENT_STAGE`; -1 the league itself, always so in the Argentine premier division) back to -1: a league
/// stage (`league::COMP_TYPE` = 2) with more rounds (`league::ROUND_COUNT`) than 7 (0 in the Polish first division)
/// gets `league::TABLE_COLUMN` = 0 and is sorted (`League::sort_table`, 0); the club's row there, in the US Major
/// League, on the league itself or after a game played (`league_table_row::PLAYED`), fills the record at once (the
/// division's id, the stage's year, the row's position and the 14 bytes of the totals from +0xd), `*complete` = the
/// league's virtual +0x10 (1, 1) (the season over), `*stage` = the stage, and returns 1; a row with no game yet keeps
/// the stage and the row and the search goes on. At the end, with a kept stage: the record from it (with the club's
/// `club::DIVISION` id), `*stage`, 1 (`*complete` not written); otherwise 0.
/// Original: `club_history_get_current_league_record` (0x444190, club_history.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_get_current_league_record(club: usize, rec: *mut u8, complete: *mut i32,
                                                                       stage_out: *mut usize, reserve: u32) -> u32 {
    use club_history_lists as ch;
    use league as l;
    if !stage_out.is_null() {
        *stage_out = 0;
    }
    if club == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x36a);
        return 0;
    }
    let nation = read_usize(club + club::NATION);
    let mut division = read_usize(club + club::DIVISION);
    if nation != 0 && read_u32(nation) == read_u32(KEY_NATION_BRAZIL) && reserve != 0 {
        let (mut from, mut to) = ([0u16; 4], [0u16; 4]);
        let from_day = read_u16(exe::date_date_549f70(&mut from, 1, 1, 0x7d1, u32::MAX)) as i16;
        let today = read_u16(G_CURRENT_DATE) as i16;
        if today > from_day && today < read_u16(exe::date_date_549f70(&mut to, 0x14, 5, 0x7d1, u32::MAX)) as i16 {
            covered!("tel42_3 Brazilian state league");
            division = read_usize(club + club::RESERVE_DIVISION);
        }
    }
    if division == 0 {
        return 0;
    }
    let league = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4);
    if league == 0 {
        return 0;
    }
    let mut current = read_u8(league + l::CURRENT_STAGE) as i8;
    let mut min_rounds = 7i16;
    let competition = read_usize(league + l::COMPETITION);
    if competition != 0 {
        if read_u32(competition) == read_u32(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION) {
            current = -1;
        }
        if read_u32(competition) == read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) {
            min_rounds = 0;
        }
    }
    let fill = |stage: usize, k: usize, id: u32| {
        let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
        write_u32(rec as usize, id);
        write_u16(rec as usize + 4, read_u16(stage + l::YEAR));
        write_u8(rec as usize + 6, read_u8(row + league_table_row::POSITION)); // tel42_3: the position
        for i in 0..14 {
            write_u8(rec as usize + 7 + i, read_u8(row + 0xd + i));
        }
    };
    let (mut kept, mut kept_row) = (0usize, 0usize);
    while current >= -1 {
        let stage = if current == -1 { league } else { read_usize(read_usize(league + l::STAGES) + current as isize as usize * 4) };
        if read_u8(stage + l::COMP_TYPE) == 2 && read_i16(stage + l::ROUND_COUNT) > min_rounds {
            write_u32(stage + l::TABLE_COLUMN, 0);
            exe::league_sort_table(stage, 0usize);
            for k in 0..read_i16(stage + l::CLUB_COUNT).max(0) as usize {
                let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
                if read_usize(row + league_table_row::CLUB) != club {
                    continue;
                }
                if read_u32(competition) == read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) || current == -1
                    || read_u8(row + league_table_row::PLAYED) != 0
                {
                    covered!("tel42_3 current league row");
                    fill(stage, k, read_u32(division));
                    let over: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(league) + 0x10));
                    *complete = over(league, 1, 1) as i8 as i32;
                    if !stage_out.is_null() {
                        *stage_out = stage;
                    }
                    return 1;
                }
                kept = stage;
                kept_row = k;
                break;
            }
        }
        current -= 1;
    }
    if kept == 0 {
        return 0;
    }
    covered!("tel42_3 league row before a game");
    fill(kept, kept_row, read_u32(read_usize(club + club::DIVISION)));
    if !stage_out.is_null() {
        *stage_out = kept;
    }
    1
}

/// The qsort comparison of two competition ids that never orders them: 0 when equal, 1 otherwise (either way round).
///
/// `compare_int_ids(a, b)`: 1 when *a > *b, otherwise 1 when *a < *b and 0 when equal (signed).
/// Original: `compare_int_ids` (0x444500, club_history.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_int_ids(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (read_i32(a as usize), read_i32(b as usize));
    (x != y) as i32 // tel42_3: never an order
}

/// The contents of a list of records of `size` bytes, from the head (the cursor moved as `LLIST::get_next` does).
unsafe fn club_history_list_contents(list: usize, size: usize) -> Vec<Vec<u8>> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let cursor = read_usize(list + llist::CURSOR);
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    loop {
        let d = exe::llist_get_next(list);
        if d == 0 {
            break;
        }
        v.push((0..size).map(|i| read_u8(d + i)).collect());
    }
    write_usize(list + llist::CURSOR, cursor);
    v
}

/// The side-by-side comparison of the routines that return a new list: the original's list is the game's; the copy's
/// is compared by content (`club_history_list_contents`) and destroyed. `regions` are the other memory compared.
unsafe fn club_history_compare_list(name: &str, size: usize, regions: &[(usize, usize)], original: impl FnOnce() -> usize,
                                    rust: impl FnOnce() -> usize) -> usize {
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(regions, original, rust);
    let (lo, lr) = (club_history_list_contents(ro, size), club_history_list_contents(rr, size));
    if rr != 0 {
        exe::llist_scalar_deleting_destructor(rr, 1);
    }
    SCOREBOARD.record(equal && lo == lr && (ro == 0) == (rr == 0), || format!("{} (club history screen): original {:02x?} rust {:02x?}, differences in {:?}",
        name, lo, lr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `club_history_get_league_history`: side by side by the list contents (the sorts it asks for give the
/// same tables twice).
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_league_history(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    if club == 0 {
        return o(this, club);
    }
    club_history_compare_list("club_history_get_league_history", 0x15, &[], || o(this, club), || club_history_get_league_history(this, club))
}

/// Comparer for `club_history_get_cup_history`: side by side by the list contents.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_cup_history(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    if club == 0 {
        return o(this, club);
    }
    club_history_compare_list("club_history_get_cup_history", 0xb, &[], || o(this, club), || club_history_get_cup_history(this, club))
}

/// Comparer for `club_history_get_league_comp_ids`: side by side by the list contents and the history list's cursor.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_league_comp_ids(this: usize, history: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    if history == 0 {
        return o(this, history);
    }
    club_history_compare_list("club_history_get_league_comp_ids", 4, &[(history, 0xe)], || o(this, history), || club_history_get_league_comp_ids(this, history))
}

/// Comparer for `club_history_get_cup_comp_ids`: side by side by the list contents and the history list's cursor.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_cup_comp_ids(this: usize, history: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I3%%);
    if history == 0 {
        return o(this, history);
    }
    club_history_compare_list("club_history_get_cup_comp_ids", 4, &[(history, 0xe)], || o(this, history), || club_history_get_cup_comp_ids(this, history))
}

/// Comparer for `club_history_get_current_league_record`: side by side over the record and the two answers given (the
/// sorts give the same tables twice). Without a club: only the original (the box).
pub(crate) unsafe extern "C" fn compare_club_history_get_current_league_record(club: usize, rec: *mut u8, complete: *mut i32,
                                                                                stage_out: *mut usize, reserve: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u8, *mut i32, *mut usize, u32) -> u32>(%%I4%%);
    if club == 0 {
        return o(club, rec, complete, stage_out, reserve);
    }
    let mut regions = vec![(rec as usize, 0x15)];
    if !complete.is_null() {
        regions.push((complete as usize, 4));
    }
    if !stage_out.is_null() {
        regions.push((stage_out as usize, 4));
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(club, rec, complete, stage_out, reserve),
        || club_history_get_current_league_record(club, rec, complete, stage_out, reserve));
    SCOREBOARD.record(equal && ro == rr, || format!("club_history_get_current_league_record(club {:#x}, reserve {}) original={} rust={}, differences in {:?}",
        club, reserve, ro, rr, first_differences(&mo, &mr)));
    ro
}

compare_order!(compare_compare_int_ids, compare_int_ids, %%I5%%);
