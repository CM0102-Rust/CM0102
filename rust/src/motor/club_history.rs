use super::*;

/// Opens an item named nome in the save archive (FN_ZOPEN with 1, 0x16, 4).
pub(crate) unsafe fn zip_open_item(save: usize, name: usize) -> usize {
    exe::zopen(save, name, 1, 0x16, 4)
}

/// Writes count items of size bytes to an open archive item (FN_ZWRITE).
pub(crate) unsafe fn zip_write(data: usize, size: u32, how_many: u32, z: usize) -> u32 {
    exe::zwrite(data, size, how_many, z)
}

/// Closes an open archive item (FN_ZCLOSE).
pub(crate) unsafe fn zip_close(z: &mut usize) {
    exe::zclose(z);
}

/// writes the count (`club_history_fields::RECORD_COUNT`) and the 0x10-byte records from [+8] into the "club
/// history.dat" item and returns 1; no container the "Error" box and 0; on open or write error the box and the game
/// stops
/// Original: `Club_History::save` (0x443f50, club_history.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn club_history_save(this: usize, save: usize) -> u32 {
    const FONTE: usize = 0x98b644;
    covered!("cpx_7 save club history");
    if save == 0 {
        show_error_at(FONTE, 0x329);
        return 0;
    }
    let mut z = zip_open_item(save, 0x98b68c);
    if z == 0 {
        fatal_error_at(FONTE, 0x331);
    }
    if zip_write(this + club_history_fields::RECORD_COUNT, 4, 1, z) != 1 {
        fatal_error_at(FONTE, 0x338);
    }
    if zip_write(read_usize(this + 8), 0x10, read_usize(this + club_history_fields::RECORD_COUNT) as u32, z) != read_usize(this + club_history_fields::RECORD_COUNT) as u32 { // cpx7: the history records
        fatal_error_at(FONTE, 0x340);
    }
    zip_close(&mut z);
    1
}

/// Shows the officials_manager.cpp error box at the line, closes the item and returns 0.
pub(crate) unsafe fn officials_save_error(row: u32, z: &mut usize) -> u8 {
    show_error_at(0xa6a22c, row);
    zip_close(z);
    0
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
        let comp = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + i * 4));
        if comp.is_null() {
            continue;
        }
        let is_league: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(read_usize(read_usize(comp.addr()) + 0xc));
        if is_league(comp.addr()) != 0 {
            continue;
        }
        let competition = comp.competition().addr();
        if exe::comp_util_is_league_division(read_u32(competition) as i32) != 0
            || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), competition) != 0
        {
            continue;
        }
        let mut rec = [0u8; 0xb];
        rec[0..4].copy_from_slice(&read_u32(competition).to_le_bytes());
        rec[4..6].copy_from_slice(&(comp.year() as u16).to_le_bytes());
        rec[7..0xb].fill(0xff);
        let p = rec.as_mut_ptr();
        find_club_progress(comp.addr(), club, p.add(7) as *mut i16, p.add(9) as *mut i16, p.add(6));
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
    let club = Club(club);
    use club_history_lists as ch;
    use league as l;
    if !stage_out.is_null() {
        *stage_out = 0;
    }
    if club.is_null() {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x36a);
        return 0;
    }
    let nation = club.nation().addr();
    let mut division = club.division().addr();
    if nation != 0 && read_u32(nation) == read_u32(KEY_NATION_BRAZIL) && reserve != 0 {
        let (mut from, mut to) = ([0u16; 4], [0u16; 4]);
        let from_day = read_u16(exe::date_date_549f70(&mut from, 1, 1, 0x7d1, u32::MAX)) as i16;
        let today = read_u16(G_CURRENT_DATE) as i16;
        if today > from_day && today < read_u16(exe::date_date_549f70(&mut to, 0x14, 5, 0x7d1, u32::MAX)) as i16 {
            covered!("tel42_3 Brazilian state league");
            division = club.reserve_division().addr();
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
                if read_usize(row + league_table_row::CLUB) != club.addr() {
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
    fill(kept, kept_row, read_u32(club.division().addr()));
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

/// Comparer for `club_history_get_league_history`: side by side by the list contents (the sorts it asks for give the
/// same tables twice).
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_league_history(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_GET_LEAGUE_HISTORY);
    if club == 0 {
        return o(this, club);
    }
    club_history_compare_list("club_history_get_league_history", 0x15, &[], || o(this, club), || club_history_get_league_history(this, club))
}

/// Comparer for `club_history_get_cup_history`: side by side by the list contents.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_cup_history(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_GET_CUP_HISTORY);
    if club == 0 {
        return o(this, club);
    }
    club_history_compare_list("club_history_get_cup_history", 0xb, &[], || o(this, club), || club_history_get_cup_history(this, club))
}

/// Comparer for `club_history_get_league_comp_ids`: side by side by the list contents and the history list's cursor.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_league_comp_ids(this: usize, history: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_GET_LEAGUE_COMP_IDS);
    if history == 0 {
        return o(this, history);
    }
    club_history_compare_list("club_history_get_league_comp_ids", 4, &[(history, 0xe)], || o(this, history), || club_history_get_league_comp_ids(this, history))
}

/// Comparer for `club_history_get_cup_comp_ids`: side by side by the list contents and the history list's cursor.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_cup_comp_ids(this: usize, history: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_GET_CUP_COMP_IDS);
    if history == 0 {
        return o(this, history);
    }
    club_history_compare_list("club_history_get_cup_comp_ids", 4, &[(history, 0xe)], || o(this, history), || club_history_get_cup_comp_ids(this, history))
}

/// Comparer for `club_history_get_current_league_record`: side by side over the record and the two answers given (the
/// sorts give the same tables twice). Without a club: only the original (the box).
pub(crate) unsafe extern "C" fn compare_club_history_get_current_league_record(club: usize, rec: *mut u8, complete: *mut i32,
                                                                                stage_out: *mut usize, reserve: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u8, *mut i32, *mut usize, u32) -> u32>(IDX_CLUB_HISTORY_GET_CURRENT_LEAGUE_RECORD);
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

/// Builds the club history store: the CD check, the league and cup history file lists, then the per-club records from
/// the saved game or new ones.
///
/// `Club_History::Club_History(save)`: the CD search (`club_history_find_cd`). The league history list (`LEAGUE_LIST`,
/// "CM3_TEMP\club league history.tmp", records of `LEAGUE_ENTRY_SIZE` bytes) and the cup history list (`CUP_LIST`,
/// "CM3_TEMP\club cup history.tmp", `CUP_ENTRY_SIZE` bytes); either one missing stops the game (club_history.cpp
/// 0x41 / 0x49). With a save, `club_history_load` reads the records and, when it answers, that is all. Otherwise the
/// count (`club_history_fields::RECORD_COUNT`) is the club count (`G_CLUB_COUNT`), the records (`RECORDS`, 16 bytes
/// each) are allocated (`malloc`; none stops the game, line 0x5e) and every offset becomes -1 (no season, no cup run).
/// Returns the object.
/// Original: `Club_History::Club_History` (0x442ab0, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_constructor(this: usize, save: usize) -> usize {
    use club_history_consts as ch;
    club_history_find_cd();
    let league = club_history_new_file_list(ch::LEAGUE_TMP, ch::LEAGUE_ENTRY_SIZE);
    write_usize(this + ch::LEAGUE_LIST, league);
    if league == 0 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x41);
    }
    let cup = club_history_new_file_list(ch::CUP_TMP, ch::CUP_ENTRY_SIZE);
    write_usize(this + ch::CUP_LIST, cup);
    if cup == 0 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x49);
    }
    if save != 0 && club_history_load(this, save) != 0 {
        covered!("tel42_1 club history from the save");
        return this;
    }
    covered!("tel42_1 new club history");
    let count = read_u32(G_CLUB_COUNT);
    write_u32(this + club_history_fields::RECORD_COUNT, count);
    let records = exe::crt_malloc(count << 4);
    write_usize(this + ch::RECORDS, records);
    if records == 0 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x5e);
    }
    for k in 0..read_i32(this + club_history_fields::RECORD_COUNT).max(0) as usize {
        let r = records + k * ch::RECORD_SIZE;
        for field in [ch::LEAGUE_FIRST, ch::LEAGUE_LAST, ch::CUP_FIRST, ch::CUP_LAST] {
            write_u32(r + field, u32::MAX); // tel42_1: no season yet
        }
    }
    this
}

/// Comparer for `club_history_constructor`.
///
/// It searches the CD, builds the two file lists and allocates (or loads) the records: it cannot run twice. Only the
/// original runs; the copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_club_history_constructor(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_CONSTRUCTOR)(this, save)
}

/// Destroys the club history store: both history file lists and the per-club records.
///
/// `Club_History::~Club_History()`: each file list (`LEAGUE_LIST`, `CUP_LIST`) present is destroyed
/// (`FILE_LLIST::~FILE_LLIST`, then `operator delete`) and zeroed; the records (`RECORDS`) present are freed (`free`)
/// and the pointer and the count (`club_history_fields::RECORD_COUNT`) zeroed.
/// Original: `Club_History::~Club_History` (0x442e20, club_history.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn club_history_destructor(this: usize) {
    use club_history_consts as ch;
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(ch::FILE_LLIST_DESTRUCTOR);
    for field in [ch::LEAGUE_LIST, ch::CUP_LIST] {
        let list = read_usize(this + field);
        if list != 0 {
            destroy(list);
            exe::crt_operator_delete(list);
            write_usize(this + field, 0);
        }
    }
    let records = read_usize(this + ch::RECORDS);
    if records != 0 {
        exe::crt_free(records);
        write_usize(this + ch::RECORDS, 0);
        write_u32(this + club_history_fields::RECORD_COUNT, 0);
    }
}

/// Comparer for `club_history_destructor`.
///
/// It frees the lists and the records: it cannot run twice. Only the original runs; the copy is proven by loading the
/// holiday save (free_game_data destroys the store first).
pub(crate) unsafe extern "thiscall" fn compare_club_history_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CLUB_HISTORY_DESTRUCTOR)(this)
}

/// Reads the club history records from the saved game.
///
/// `Club_History::load(save)`: no save: the club_history.cpp 0x2ee box and 0. The "club history.dat" item of the save
/// (`zopen` with 0, 0x16, 4; missing stops the game, line 0x2f6); the count (`club_history_fields::RECORD_COUNT`, 4
/// bytes; short read stops the game, 0x2fd); the records (`RECORDS`, `malloc` of count × 16; none stops the game,
/// 0x306) read in one go (count records of 16 bytes; fewer stops the game, 0x30e); the item closed. Returns 1.
/// Original: `Club_History::load` (0x443cd0, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_load(this: usize, save: usize) -> u32 {
    use club_history_consts as ch;
    if save == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x2ee);
        return 0;
    }
    let mut z = exe::zopen(save, ch::SAVE_ITEM, 0, 0x16, 4);
    if z == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x2f6);
        return 0;
    }
    covered!("tel42_1 load club history");
    let count = this + club_history_fields::RECORD_COUNT;
    if exe::zread(count, 4, 1, z) != 1 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x2fd);
    }
    let records = exe::crt_malloc(read_u32(count) << 4);
    write_usize(this + ch::RECORDS, records);
    if records == 0 {
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x306);
    }
    if exe::zread(records, ch::RECORD_SIZE as u32, read_u32(count), z) != read_u32(count) { // tel42_1: every record read
        fatal_error_at(ch::CLUB_HISTORY_CPP, 0x30e);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `club_history_load`.
///
/// It allocates the records and reads the save item: it cannot run twice. Only the original runs; the copy is proven by
/// loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_club_history_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLUB_HISTORY_LOAD)(this, save)
}

/// Adds the finished season of a competition to each club's history.
///
/// `Club_History::record_comp_season(comp)`: no competition: the club_history.cpp 0xab box and 0; a stage instead of
/// the competition itself (`league::STAGE_INDEX` ≠ 0xff): the 0xb3 box and 0; a friendly competition
/// (`Friendly::is_friendly_competition`): 0.
///
/// A league division (`comp_util_is_league_division` of the competition id): from the competition itself, each stage
/// whose table is recorded: with more than 7 rounds (`league::ROUND_COUNT`), except in the Polish first division, where
/// only its stage 1 is. The table is sorted (`League::sort_table`, 0) and each row gives a league record (21 bytes:
/// competition id, the competition's year, the row's position, then the 14 bytes of the totals from row +0xd: played,
/// won, drawn, lost, goals for and against, points; in the Polish first division played + 14) appended to the club's
/// league seasons (`FILE_LLIST::add_node` on the league list with the club's first and last offsets). Then the next
/// stage (`Comp::get_stage`, index + 1) while it exists (index below `league::CURRENT_STAGE`) and is a league
/// (`league::COMP_TYPE` = 2). Returns 1.
///
/// A cup: not set up for the season (`cup::SEASON_SET_UP` = 0): 0. A mark per club (`calloc`; none: the 0xfd box and
/// 0). From the current stage (`cup::CURRENT_STAGE`) back to -1 (the cup itself): in a league stage each table row's
/// club gets (competition id, year, the row's status `league_table_row::STATUS`, the stage id `league::STAGE_ID`,
/// -1); in a knockout stage the rounds from the last back, each tie's home and away team (index ≠ -1, the team table's
/// club) gets (id, year, the team's outcome +5, the stage label `cup::STAGE_LABEL`, the round code
/// `cup_round::ROUND_CODE`). Only the first record of each club (the furthest it got) goes to its cup runs (the cup
/// list, the club's +8/+0xc offsets). The marks freed; returns 1.
/// Original: `Club_History::record_comp_season` (0x442e80, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_record_comp_season(this: usize, comp: usize) -> u32 {
    club_history_record_comp_season_with(this, comp, &mut |stage| { exe::league_sort_table(stage, 0usize); },
        &mut |list, first, last, rec| { exe::file_llist_add_node(list, first, last, rec.as_ptr()); })
}

/// Comparer for `club_history_record_comp_season`: the copy plans, the original runs, the plan is checked in the files.
///
/// The records go into the history files (`FILE_LLIST::add_node` appends and moves the club's offsets): the routine
/// cannot run twice. The copy runs first as a dry run: it sorts the tables as the original does (`League::sort_table`
/// is a total order ending on the club id and stores the positions in that order: sorting again changes nothing) and
/// keeps each record it would append, with its club offsets; the original runs afterwards from the same state. Then,
/// for each club offset pair of the plan, the club's nodes are read back from the file (`FILE_LLIST::get_nodes`): the
/// newest ones must be the planned records, in order, and the returns equal. Without a competition or on a stage: only
/// the original (the error boxes).
pub(crate) unsafe extern "thiscall" fn compare_club_history_record_comp_season(this: usize, comp: usize) -> u32 {
    let comp = League(comp);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLUB_HISTORY_RECORD_COMP_SEASON);
    if comp.is_null() || comp.stage_index() != 0xff {
        return o(this, comp.addr());
    }
    // (list, first offset address, last offset address, record)
    let mut plan: Vec<(usize, usize, usize, Vec<u8>)> = Vec::new();
    let rr = club_history_record_comp_season_with(this, comp.addr(), &mut |stage| { exe::league_sort_table(stage, 0usize); },
        &mut |list, first, last, rec| plan.push((list, first, last, rec.to_vec())));
    let ro = o(this, comp.addr());
    let mut wrong: Vec<String> = Vec::new();
    let mut done: Vec<usize> = Vec::new();
    for (list, first, last, _) in &plan {
        if done.contains(first) {
            continue;
        }
        done.push(*first);
        let wanted: Vec<&Vec<u8>> = plan.iter().filter(|p| p.1 == *first).map(|p| &p.3).collect();
        let nodes = exe::file_llist_get_nodes(*list, read_u32(*first), read_u32(*last));
        let mut got: Vec<Vec<u8>> = Vec::new();
        if nodes != 0 {
            write_usize(nodes + llist::CURSOR, read_usize(nodes + llist::TAIL));
            while got.len() < wanted.len() {
                let d = exe::llist_get_prev(nodes);
                if d == 0 {
                    break;
                }
                got.push((0..wanted[0].len()).map(|i| read_u8(d + i)).collect());
            }
            exe::llist_scalar_deleting_destructor(nodes, 1);
        }
        got.reverse();
        if got.len() != wanted.len() || got.iter().zip(wanted.iter()).any(|(g, w)| g != *w) {
            wrong.push(format!("club offsets {:#x}: planned {:02x?} in the file {:02x?}", first, wanted, got));
        }
    }
    SCOREBOARD.record(ro == rr && wrong.is_empty(), || format!("club_history_record_comp_season(comp {:#x}) original={} rust={} {} appends planned: {}",
        comp.addr(), ro, rr, plan.len(), wrong.join("; ")));
    ro
}

/// A new empty list (`operator new(0xe)` and `LLIST::LLIST`), or 0 when the allocation fails.
pub(crate) unsafe fn club_history_new_list() -> usize {
    let p = exe::crt_operator_new(0xe);
    if p != 0 { exe::new_llist(p) } else { 0 }
}

/// The stored records of a club's file list from the newest to the oldest, appended to `list` (created on the first
/// one); returns the list.
pub(crate) unsafe fn club_history_append_stored(mut list: usize, file_list: usize, first: usize, last: usize, size: u32) -> usize {
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

/// The distinct competition ids of a history list (the first dword of each record), sorted with `compare_int_ids`, as
/// a new list of ints; the two `operator new` failures give the boxes at the lines given.
pub(crate) unsafe fn club_history_comp_ids(history: usize, lines: [u32; 3]) -> usize {
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

/// The side-by-side comparison of the routines that return a new list: the original's list is the game's; the copy's
/// is compared by content (`club_history_list_contents`) and destroyed. `regions` are the other memory compared.
pub(crate) unsafe fn club_history_compare_list(name: &str, size: usize, regions: &[(usize, usize)], original: impl FnOnce() -> usize,
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

/// The CD search at the start of the club history constructor: each CD drive (`GetLogicalDriveStringsA`,
/// `GetDriveTypeA` = 5) is looked at for "<root>cm0102.exe" (`file_find_first`) until one has it, with the
/// "Insert CD" box (OK) in between. A no-CD exe patched in this routine (as `cm0102-1080`: the drive type's `jne` at
/// 0x442b0c made `nop`, the found test at 0x442b4f made `xor`) is honoured by reading those bytes.
pub(crate) unsafe fn club_history_find_cd() {
    use club_history_consts as ch;
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = (ch::CD_DRIVE_TYPE_JUMP as *const u8).read() == 0x90;
    let always_found = (ch::CD_FOUND_TEST as *const u8).read() != 0x85;
    loop {
        // simplification: the buffer starts zeroed (as check_game_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        covered!("tel42_1 club history insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// A history file list in the temporary folder: `operator new(8)` and `FILE_LLIST::FILE_LLIST("CM3_TEMP\<name>",
/// record size)`; a failed allocation gives 0.
pub(crate) unsafe fn club_history_new_file_list(name: usize, record_size: u32) -> usize {
    let construct: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(club_history_consts::FILE_LLIST_CONSTRUCTOR);
    let list = exe::crt_operator_new(8);
    if list == 0 {
        return 0;
    }
    construct(list, exe::utils_build_file_path(club_history_consts::TEMP_FOLDER, name), record_size)
}

/// The season of a finished competition in each club's history, with the file list append and the table sort given
/// from outside (the copy appends and sorts; the comparer's dry run records the appends and sorts).
///
/// See `club_history_record_comp_season`.
pub(crate) unsafe fn club_history_record_comp_season_with(this: usize, comp: usize, sort: &mut dyn FnMut(usize),
                                               add: &mut dyn FnMut(usize, usize, usize, &[u8])) -> u32 {
    use club_history_season as ch;
    use league as l;
    if comp == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xab);
        return 0;
    }
    if read_u8(comp + l::STAGE_INDEX) != 0xff {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xb3);
        return 0;
    }
    let competition = read_usize(comp + l::COMPETITION);
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), competition) != 0 {
        return 0;
    }
    let comp_id = read_u32(competition);
    let year = read_u16(comp + l::YEAR);
    let records = read_usize(this + ch::RECORDS);
    let polish = read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION);
    if exe::comp_util_is_league_division(comp_id as i32) != 0 {
        // a league: the table of the league and of each following stage that is a league
        let mut stage = comp;
        loop {
            let recorded = if read_i16(stage + l::ROUND_COUNT) > 7 && comp_id != polish {
                true
            } else {
                comp_id == polish && read_u8(stage + l::STAGE_INDEX) == 1 // tel42_2: the Polish second stage
            };
            if recorded {
                covered!("tel42_2 league table into the history");
                sort(stage);
                let mut rec = [0u8; 0x15];
                rec[0..4].copy_from_slice(&comp_id.to_le_bytes());
                rec[4..6].copy_from_slice(&year.to_le_bytes());
                for k in 0..read_i16(stage + l::CLUB_COUNT).max(0) as usize {
                    let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
                    let club = records + read_u32(read_usize(row + league_table_row::CLUB)) as usize * ch::RECORD_SIZE;
                    rec[6] = read_u8(row + league_table_row::POSITION); // tel42_2: the final position
                    for (i, b) in rec[7..0x15].iter_mut().enumerate() {
                        *b = read_u8(row + 0xd + i);
                    }
                    if read_u32(read_usize(stage + l::COMPETITION)) == polish {
                        // the Polish second stage counts the 14 games of the first one
                        covered!("tel42_2 Polish games played");
                        rec[0xb] = rec[0xb].wrapping_add(0xe);
                    }
                    add(read_usize(this + ch::LEAGUE_LIST), club + ch::LEAGUE_FIRST, club + ch::LEAGUE_LAST, &rec);
                }
            }
            let index = read_i8(stage + l::STAGE_INDEX) as i32;
            if index >= read_i32(comp + l::CURRENT_STAGE) {
                return 1;
            }
            let next = exe::comp_get_stage(comp, index + 1);
            if read_u8(next + l::COMP_TYPE) != 2 {
                return 1;
            }
            stage = next;
        }
    }
    if read_u32(comp + cup::SEASON_SET_UP) == 0 {
        return 0;
    }
    let marks = exe::crt_calloc(read_u32(G_CLUB_COUNT), 4);
    if marks == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xfd);
        return 0;
    }
    covered!("tel42_2 cup runs into the history");
    let mut rec = [0u8; 0xb];
    rec[0..4].copy_from_slice(&comp_id.to_le_bytes());
    rec[4..6].copy_from_slice(&year.to_le_bytes());
    // one record per club, from the last stage back to the competition itself (-1): a club keeps the furthest one
    let mut add_club = |club: usize, status: u8, label: u16, round: u16| {
        let id = read_u32(club) as usize;
        if read_u32(marks + id * 4) != 0 {
            return;
        }
        rec[6] = status;
        rec[7..9].copy_from_slice(&label.to_le_bytes());
        rec[9..0xb].copy_from_slice(&round.to_le_bytes());
        let r = records + id * ch::RECORD_SIZE;
        add(read_usize(this + ch::CUP_LIST), r + ch::CUP_FIRST, r + ch::CUP_LAST, &rec);
        write_u32(marks + id * 4, 1);
    };
    let mut s = read_i32(comp + cup::CURRENT_STAGE);
    while s >= -1 {
        let stage = exe::comp_get_stage(comp, s);
        if read_u8(stage + l::COMP_TYPE) == 2 {
            covered!("tel42_2 cup group stage");
            for k in 0..read_i16(stage + l::CLUB_COUNT).max(0) as usize {
                let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
                add_club(read_usize(row + league_table_row::CLUB), read_u8(row + league_table_row::STATUS),
                         read_u16(stage + l::STAGE_ID), 0xffff);
            }
        } else {
            let mut r = read_i16(stage + cup::ROUND_COUNT) as i32 - 1;
            while r >= 0 {
                let round = read_usize(stage + cup::ROUND_TABLE) + r as usize * cup_round::SIZE;
                let ties = read_usize(read_usize(stage + cup::TIE_TABLES) + r as usize * 4);
                for t in 0..read_i16(round + cup_round::TIES).max(0) as usize {
                    let tie = ties + t * 0x35;
                    for side in [cup_tie::HOME_TEAM, cup_tie::AWAY_TEAM] {
                        let team = read_i16(tie + side);
                        if team == -1 {
                            continue;
                        }
                        let entry = read_usize(stage + cup::TEAM_TABLE) + team as isize as usize * 6;
                        // tel42_2: the team's outcome and the round it reached
                        add_club(read_usize(entry), read_u8(entry + 5), read_u16(stage + cup::STAGE_LABEL),
                                 read_u16(round + cup_round::ROUND_CODE));
                    }
                }
                r -= 1;
            }
        }
        s -= 1;
    }
    exe::crt_free(marks);
    1
}

/// The contents of a list of records of `size` bytes, from the head (the cursor moved as `LLIST::get_next` does).
pub(crate) unsafe fn club_history_list_contents(list: usize, size: usize) -> Vec<Vec<u8>> {
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
