// --- sx17_2: the player search's result list (player_search.cpp): ~PLAYER_SEARCH (0x7ae4f0), search_result_copy
// (0x7af580), add_player (0x7af720), add_result (0x7af870), remove_own_squad (0x7b11b0), remove_player (0x7b1260),
// clear_results (0x7b14a0), purge_invalid_results (0x7b1520) and the order by person (0x7bce50) ---

/// The search result at index `k` of the search's sorted result array (`player_search_fields::RESULTS`).
unsafe fn player_search_result_at(this: usize, k: i32) -> usize {
    read_usize(this + player_search_fields::RESULTS).wrapping_add((k as usize).wrapping_mul(SCOUT_DAY_RESULT_SIZE))
}

/// Sorts the result array by person (`player_search_compare_results_by_person`), as every list routine does after a change.
unsafe fn player_search_sort_results(this: usize) {
    exe::crt_qsort(read_usize(this + player_search_fields::RESULTS), read_u32(this + player_search_fields::RESULT_COUNT),
                   SCOUT_DAY_RESULT_SIZE as u32, player_search_compare_results_by_person as *const () as usize);
}

/// After entries were removed: with entries left, they are sorted again; with none, the array is freed and its
/// pointer and capacity zeroed.
unsafe fn player_search_sort_or_free_results(this: usize) {
    if read_u32(this + player_search_fields::RESULT_COUNT) != 0 {
        player_search_sort_results(this);
    } else {
        exe::crt_free(read_usize(this + player_search_fields::RESULTS));
        write_usize(this + player_search_fields::RESULTS, 0);
        write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
    }
}

/// Removes the entry `k` by copying the last entry over it and counting one less (the order is restored afterwards).
unsafe fn player_search_drop_result(this: usize, entry: usize) {
    let count = this + player_search_fields::RESULT_COUNT;
    search_result_copy(entry, player_search_result_at(this, read_i32(count) - 1));
    write_i32(count, read_i32(count) - 1);
}

/// The search's scouted club record (+0x170) leaves the global list of club records and is deleted.
unsafe fn player_search_delete_club_record(this: usize, record: usize) {
    exe::llist_remove(G_SCOUT_CLUB_TABLE_LIST, record);
    exe::crt_operator_delete(record);
    write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, 0);
}

/// Destroys a player search: the result array is freed (pointer zeroed) and the scouted club record, if any, leaves
/// the global list of club records and is deleted. The capacity and count stay.
/// Original: `PLAYER_SEARCH::~PLAYER_SEARCH` (0x7ae4f0, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_player_search(this: usize) {
    let results = read_usize(this + player_search_fields::RESULTS);
    if results != 0 {
        exe::crt_free(results);
        write_usize(this + player_search_fields::RESULTS, 0);
    }
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    if record != 0 {
        covered!("sx17_2 drop the club record");
        player_search_delete_club_record(this, record);
    }
}

/// Comparer for `drop_player_search`: it frees, so it cannot run twice; only the original runs and the plan is
/// checked afterwards (no result array and no club record).
pub(crate) unsafe extern "thiscall" fn compare_drop_player_search(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let (results, record) = (read_usize(this + player_search_fields::RESULTS), read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD));
    SCOREBOARD.record(results == 0 && record == 0, || format!("drop_player_search({:#x}) results {:#x} record {:#x}", this, results, record));
}

/// Copies one search result (0x2d bytes) field by field: the name ref (+0..+0xc), the two dates (`Date::operator=`),
/// the flags, the two ratings (+0x21, +0x25) and the bytes +0x29..+0x2c. Returns the destination.
/// Original: `search_result_copy` (0x7af580, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_result_copy(this: usize, from: usize) -> usize {
    for d in [0, 4, 8, 0xc] {
        write_u32(this + d, read_u32(from + d));
    }
    let mut copy = [0u8; 8];
    exe::date_operator_3d(this + 0x10, copy.as_mut_ptr() as usize, from + 0x10);
    exe::date_operator_3d(this + search_result::LAST_UPDATE_DATE, copy.as_mut_ptr() as usize, from + search_result::LAST_UPDATE_DATE);
    write_u8(this + search_result::FLAGS, read_u8(from + search_result::FLAGS));
    write_u32(this + 0x21, read_u32(from + 0x21));
    write_u32(this + 0x25, read_u32(from + 0x25));
    for b in 0x29..=0x2c {
        write_u8(this + b, read_u8(from + b));
    }
    this
}

/// Comparer for `search_result_copy`: side by side over the destination entry and the return.
pub(crate) unsafe extern "thiscall" fn compare_search_result_copy(this: usize, from: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, SCOUT_DAY_RESULT_SIZE)], || o(this, from), || search_result_copy(this, from));
    SCOREBOARD.record(same && r == n, || format!("search_result_copy({:#x}, {:#x}) {:?}", this, from, first_differences(&mo, &mr)));
    r
}

/// Adds a player to the search, dated today: the entry gets his name ref (`fill_staff_name_ref`), today as the last
/// update, flags, ratings and the bytes +0x29 and +0x2a at 0, and as +0x2c the note rating of the owning manager's note
/// on him (`Notes::find_note`, byte +0x113; 0xff without a note), and goes in through `add_result`. Without a player:
/// the player_search.cpp "Error" box (line 0x369) and 0; otherwise what `add_result` answers.
/// Defect of the original: the byte +0x2b of the entry is never set (the stack's leftover goes into the list); the copy
/// writes 0 there and the comparer ignores it.
/// Original: `PLAYER_SEARCH::add_player` (0x7af720, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_add_player(this: usize, person: Staff) -> u32 {
    let mut entry = [0u8; SCOUT_DAY_RESULT_SIZE];
    let e = entry.as_mut_ptr() as usize;
    exe::date_date_54a110(e + 0x10);
    exe::date_date_54a110(e + search_result::LAST_UPDATE_DATE);
    if person.is_null() {
        show_error_box(0xa701b0, 0x369); // player_search.cpp
        return 0;
    }
    exe::fill_staff_name_ref(person, e);
    let mut copy = [0u8; 8];
    exe::date_operator_3d(e + search_result::LAST_UPDATE_DATE, copy.as_mut_ptr() as usize, G_CURRENT_DATE);
    let note = exe::notes_find_note(read_usize(G_NOTES), staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize), person) as usize;
    write_u8(e + 0x2c, if note != 0 { read_u8(note + 0x113) } else { 0xff }); // sx17_2: the note rating
    player_search_add_result(this, e)
}

/// Comparer for `player_search_add_player`: side by side over the result list (`player_search_compare_results`),
/// ignoring the byte +0x2b of the player's entry (the original's uninitialised stack byte); only the original runs
/// without a player (the box) or when the list grows (the array is reallocated).
pub(crate) unsafe extern "thiscall" fn compare_player_search_add_player(this: usize, person: Staff) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, Staff) -> u32>(%%I2%%);
    if person.is_null() {
        return o(this, person);
    }
    player_search_compare_results(this, || format!("player_search_add_player({:#x}, person {:#x})", this, person.addr()),
        &[], Some(read_i32(person.addr())), || o(this, person), || player_search_add_player(this, person),
        |(_, capacity, count)| read_i32(this + SCOUT_DAY_RESULT_CAPACITY) == capacity + 250 && read_i32(this + player_search_fields::RESULT_COUNT) == count + 1)
}

/// Adds or updates an entry of the sorted result list: without an entry, the player_search.cpp "Error" box (line
/// 0x394) and 0. When the person is already listed (binary search by person), his entry is overwritten
/// (`search_result_copy`). Otherwise, with the array full the capacity grows by 250 (`realloc`; when it fails: the box,
/// line 0x3b5, `game_fatal_cleanup` and `exit(-1)`); without a note of the owning manager on the person
/// (`Notes::find_note`) the given entry's byte +0x2c becomes 0xff; the entry is copied to the end and the list sorted.
/// Either way the report mark is cleared; answers 1.
/// Original: `PLAYER_SEARCH::add_result` (0x7af870, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_add_result(this: usize, entry: usize) -> u32 {
    if entry == 0 {
        show_error_box(0xa701b0, 0x394); // player_search.cpp
        return 0;
    }
    let count = this + player_search_fields::RESULT_COUNT;
    if read_u32(count) != 0 {
        let found = exe::crt_bsearch(entry, read_usize(this + player_search_fields::RESULTS), read_u32(count), SCOUT_DAY_RESULT_SIZE as u32,
                                     player_search_compare_results_by_person as *const () as usize);
        if found != 0 {
            covered!("sx17_2 update a result");
            search_result_copy(found, entry);
            write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
            return 1;
        }
    }
    let capacity = this + SCOUT_DAY_RESULT_CAPACITY;
    if read_u32(count) == read_u32(capacity) {
        covered!("sx17_2 grow the results");
        write_i32(capacity, read_i32(capacity) + 250); // sx17_2: the growth step
        let array = exe::crt_realloc(read_usize(this + player_search_fields::RESULTS), read_u32(capacity).wrapping_mul(SCOUT_DAY_RESULT_SIZE as u32));
        write_usize(this + player_search_fields::RESULTS, array);
        if array == 0 {
            show_error_box(0xa701b0, 0x3b5); // player_search.cpp
            exe::game_fatal_cleanup();
            exe::shared_noop_ret0_5a8f60();
            exe::crt_exit(-1);
        }
    }
    let owner = staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize);
    if exe::notes_find_note(read_usize(G_NOTES), owner, staff_at(read_i32(entry + 0xc) as isize)) == 0 {
        write_u8(entry + 0x2c, 0xff);
    }
    search_result_copy(player_search_result_at(this, read_i32(count)), entry);
    write_i32(count, read_i32(count) + 1);
    player_search_sort_results(this);
    write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
    1
}

/// Comparer for `player_search_add_result`: side by side over the result list and the given entry; only the original
/// runs without an entry (the box) or when the array is reallocated (then the capacity must have grown by 250 and the
/// count by one).
pub(crate) unsafe extern "thiscall" fn compare_player_search_add_result(this: usize, entry: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I3%%);
    if entry == 0 {
        return o(this, entry);
    }
    player_search_compare_results(this, || format!("player_search_add_result({:#x}, person {})", this, read_i32(entry + 0xc)),
        &[(entry, SCOUT_DAY_RESULT_SIZE)], None, || o(this, entry), || player_search_add_result(this, entry),
        |(_, capacity, count)| read_i32(this + SCOUT_DAY_RESULT_CAPACITY) == capacity + 250 && read_i32(this + player_search_fields::RESULT_COUNT) == count + 1)
}

/// Removes the own squad entries (flag bit 1 of `search_result::FLAGS`) from the result list, each replaced by the
/// last entry; then the list is sorted again or, empty, freed. An empty list stays as it is. Answers 1.
/// Original: `PLAYER_SEARCH::remove_own_squad` (0x7b11b0, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn player_search_remove_own_squad(this: usize) -> u32 {
    if read_u32(this + player_search_fields::RESULT_COUNT) == 0 {
        return 1;
    }
    let mut k = 0;
    while k < read_i32(this + player_search_fields::RESULT_COUNT) {
        let entry = player_search_result_at(this, k);
        if read_u8(entry + search_result::FLAGS) & 2 != 0 { // sx17_2: the own squad flag
            player_search_drop_result(this, entry);
            k -= 1;
        }
        k += 1;
    }
    player_search_sort_or_free_results(this);
    1
}

/// Comparer for `player_search_remove_own_squad`: side by side over the result list; when every entry is of the own
/// squad the array is freed, so only the original runs and the plan (no array, capacity and count 0) is checked.
pub(crate) unsafe extern "thiscall" fn compare_player_search_remove_own_squad(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I4%%);
    player_search_compare_results(this, || format!("player_search_remove_own_squad({:#x})", this), &[], None,
        || o(this), || player_search_remove_own_squad(this), |_| player_search_results_freed(this))
}

/// Removes a person from the result list: the entry found by binary search is replaced by the last one, then the list
/// is sorted again or, empty, freed, and the report mark is cleared; answers 1. Without a person, with an empty list or
/// when the person is not listed: the player_search.cpp "Error" box (lines 0x733, 0x73d, 0x747) and 0.
/// Original: `PLAYER_SEARCH::remove_player` (0x7b1260, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_remove_player(this: usize, person: Staff) -> u32 {
    let found = match player_search_find_result(this, person) {
        Ok(entry) => entry,
        Err(line) => {
            show_error_box(0xa701b0, line); // player_search.cpp
            return 0;
        }
    };
    player_search_drop_result(this, found);
    player_search_sort_or_free_results(this);
    write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
    1
}

/// The entry of `person` in the result list (binary search on the key `fill_staff_name_ref` builds), or the
/// "Error" box line `player_search_remove_player` shows: 0x733 without a person, 0x73d with an empty list, 0x747 when
/// the person is not listed.
unsafe fn player_search_find_result(this: usize, person: Staff) -> Result<usize, u32> {
    let mut key = [0u8; SCOUT_DAY_RESULT_SIZE];
    let k = key.as_mut_ptr() as usize;
    exe::date_date_54a110(k + 0x10);
    exe::date_date_54a110(k + search_result::LAST_UPDATE_DATE);
    if person.is_null() {
        return Err(0x733);
    }
    exe::fill_staff_name_ref(person, k);
    let count = read_u32(this + player_search_fields::RESULT_COUNT);
    if count == 0 {
        return Err(0x73d);
    }
    match exe::crt_bsearch(k, read_usize(this + player_search_fields::RESULTS), count, SCOUT_DAY_RESULT_SIZE as u32,
                           player_search_compare_results_by_person as *const () as usize) {
        0 => Err(0x747),
        entry => Ok(entry),
    }
}

/// Comparer for `player_search_remove_player`: side by side over the result list; only the original runs when it shows
/// the box (no person, empty list, not listed) and when the last entry goes (the array is freed; plan: no array,
/// capacity and count 0).
pub(crate) unsafe extern "thiscall" fn compare_player_search_remove_player(this: usize, person: Staff) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, Staff) -> u32>(%%I5%%);
    if player_search_find_result(this, person).is_err() {
        return o(this, person);
    }
    player_search_compare_results(this, || format!("player_search_remove_player({:#x}, person {:#x})", this, person.addr()), &[], None,
        || o(this, person), || player_search_remove_player(this, person), |_| player_search_results_freed(this))
}

/// Empties the result list: the array is freed (pointer zeroed), the capacity, count and report mark are zeroed. With
/// `drop_club` set and an assigned club (> −1) that has a scouted club record, the assignment is cleared (−1) and the
/// record leaves the global list of club records and is deleted. Answers 1.
/// Original: `PLAYER_SEARCH::clear_results` (0x7b14a0, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_clear_results(this: usize, drop_club: u32) -> u32 {
    let results = read_usize(this + player_search_fields::RESULTS);
    if results != 0 {
        exe::crt_free(results);
        write_usize(this + player_search_fields::RESULTS, 0);
    }
    write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
    write_u32(this + player_search_fields::RESULT_COUNT, 0);
    write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    if read_i32(this + player_search_fields::ASSIGNED_CLUB) > -1 && drop_club != 0 && record != 0 {
        covered!("sx17_2 clear the assigned club");
        write_i32(this + player_search_fields::ASSIGNED_CLUB, -1);
        player_search_delete_club_record(this, record);
    }
    1
}

/// Comparer for `player_search_clear_results`: it frees, so only the original runs, and the plan is checked: no array,
/// capacity, count and mark 0 and, when the club was to be dropped, no assigned club and no record.
pub(crate) unsafe extern "thiscall" fn compare_player_search_clear_results(this: usize, drop_club: u32) -> u32 {
    let drops = read_i32(this + player_search_fields::ASSIGNED_CLUB) > -1 && drop_club != 0
        && read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD) != 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I6%%)(this, drop_club);
    let cleared = player_search_results_freed(this) && read_u32(this + SCOUT_DAY_REPORT_MARK) == 0
        && (!drops || (read_i32(this + player_search_fields::ASSIGNED_CLUB) == -1 && read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD) == 0));
    SCOREBOARD.record(r == 1 && cleared, || format!("player_search_clear_results({:#x}, {}) returned {} cleared {}", this, drop_club, r, cleared));
    r
}

/// The person `person` still fits a result of this search: he matches the entry's names (`database_staff_matches_names`)
/// and, for a staff search (list type 3), is a non-player who is not a manager, assistant or coach (job for the club
/// 1..3), otherwise a player; and he is not retired or retiring (`player_regen_get_retirement_status` ≠ 2).
unsafe fn player_search_result_still_fits(this: usize, entry: usize, person: Staff) -> bool {
    if exe::database_staff_matches_names(person, read_u32(entry), read_u32(entry + 4), read_u32(entry + 8), entry + 0x10) == 0 {
        return false;
    }
    let staff_search = read_u8(this + player_search_fields::LIST_TYPE) == 3;
    if !staff_search && person.player().is_null() {
        return false;
    }
    if staff_search && (person.non_player().is_null() || matches!(person.job_for_club(), 1..=3)) {
        return false;
    }
    exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), person) != 2
}

/// Drops the results whose person is gone: for each entry (with `relink` set, its last update becomes today), the
/// person at its index (below the staff count − 16) must still fit it (`player_search_result_still_fits`). When not,
/// with `relink` the whole staff table is searched for the first person who fits the entry, and his index replaces the
/// entry's; otherwise (or when nobody fits) the entry is replaced by the last one. The list is then sorted again or,
/// empty, freed (an empty list from the start stays as it is). Answers 1.
/// Original: `PLAYER_SEARCH::purge_invalid_results` (0x7b1520, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_purge_invalid_results(this: usize, relink: u32) -> u32 {
    let count = this + player_search_fields::RESULT_COUNT;
    if read_i32(count) == 0 {
        return 1;
    }
    let mut k = 0;
    while k < read_i32(count) {
        let entry = player_search_result_at(this, k);
        if relink != 0 {
            let mut copy = [0u8; 8];
            exe::date_operator_3d(entry + search_result::LAST_UPDATE_DATE, copy.as_mut_ptr() as usize, G_CURRENT_DATE);
        }
        let id = read_i32(entry + 0xc);
        let person = staff_at(id as isize);
        let fits = id >= 0 && id < read_i32(G_STAFF_COUNT) - 0x10 && !person.is_null() && player_search_result_still_fits(this, entry, person);
        if !fits && !(relink != 0 && player_search_relink_result(this, entry)) {
            covered!("sx17_2 drop an invalid result");
            player_search_drop_result(this, entry); // sx17_2: the invalid entry goes
            k -= 1;
        }
        k += 1;
    }
    player_search_sort_or_free_results(this);
    1
}

/// Looks through the whole staff table for the first person who fits the entry (`player_search_result_still_fits`)
/// and gives the entry his index; false when nobody does.
unsafe fn player_search_relink_result(this: usize, entry: usize) -> bool {
    for id in 0..read_i32(G_STAFF_COUNT) {
        if player_search_result_still_fits(this, entry, staff_at(id as isize)) {
            covered!("sx17_2 relink a result");
            write_i32(entry + 0xc, id);
            return true;
        }
    }
    false
}

/// Comparer for `player_search_purge_invalid_results`: side by side over the result list; when every entry goes the
/// array is freed, so only the original's outcome is checked (no array, capacity and count 0).
pub(crate) unsafe extern "thiscall" fn compare_player_search_purge_invalid_results(this: usize, relink: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I7%%);
    player_search_compare_results(this, || format!("player_search_purge_invalid_results({:#x}, {})", this, relink), &[], None,
        || o(this, relink), || player_search_purge_invalid_results(this, relink), |_| player_search_results_freed(this))
}

/// The order of the result list: by person index (+0xc), a null entry after everything.
/// Original: `player_search_compare_results_by_person` (0x7bce50, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_compare_results_by_person(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    read_i32(a + 0xc).wrapping_sub(read_i32(b + 0xc))
}

/// Comparer for `player_search_compare_results_by_person`: both run, the answers must match.
pub(crate) unsafe extern "C" fn compare_player_search_compare_results_by_person(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I8%%)(a, b);
    let n = player_search_compare_results_by_person(a, b);
    SCOREBOARD.record(r == n, || format!("player_search_compare_results_by_person({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// The result list has no array, no capacity and no entries (what a routine that empties it leaves).
unsafe fn player_search_results_freed(this: usize) -> bool {
    read_usize(this + player_search_fields::RESULTS) == 0 && read_u32(this + SCOUT_DAY_RESULT_CAPACITY) == 0
        && read_u32(this + player_search_fields::RESULT_COUNT) == 0
}

/// The memory a result list routine changes: the array header (pointer, capacity, count), the report mark, the assigned
/// club, the club record pointer, the `extra` regions and, last, the array up to its capacity.
unsafe fn player_search_result_regions(this: usize, extra: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut regions = vec![(this + player_search_fields::RESULTS, 0xc), (this + SCOUT_DAY_REPORT_MARK, 4),
                           (this + player_search_fields::ASSIGNED_CLUB, 4), (this + player_search_fields::SCOUTED_CLUB_RECORD, 4)];
    regions.extend_from_slice(extra);
    let (array, capacity) = (read_usize(this + player_search_fields::RESULTS), read_i32(this + SCOUT_DAY_RESULT_CAPACITY));
    if array != 0 && capacity > 0 {
        regions.push((array, capacity as usize * SCOUT_DAY_RESULT_SIZE));
    }
    regions
}

/// Runs a result list routine side by side over `player_search_result_regions`, the original first. When the original
/// moved or freed the array (realloc, free), the copy cannot run on the same memory: only `plan` (given the array,
/// capacity and count before) is checked. `mask_person`: the byte +0x2b of that person's entry is ignored.
unsafe fn player_search_compare_results<R: PartialEq + core::fmt::Debug>(this: usize, label: impl FnOnce() -> String, extra: &[(usize, usize)],
        mask_person: Option<i32>, original: impl FnOnce() -> R, copy: impl FnOnce() -> R, plan: impl FnOnce((usize, i32, i32)) -> bool) -> R {
    let regions = player_search_result_regions(this, extra);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let header = (read_usize(this + player_search_fields::RESULTS), read_i32(this + SCOUT_DAY_RESULT_CAPACITY),
                  read_i32(this + player_search_fields::RESULT_COUNT));
    let r = original();
    if read_usize(this + player_search_fields::RESULTS) != header.0 {
        let ok = plan(header);
        SCOREBOARD.record(ok, || format!("{} (array moved: plan only) before {:x?}", label(), header));
        return r;
    }
    let mut after_original = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let n = crate::rng::pause_trace(copy);
    let mut after_copy = crate::comparison::MemorySnapshot::read(&regions);
    after_original.write_back();
    if let Some(person) = mask_person {
        for snapshot in [&mut after_original, &mut after_copy] {
            if let Some((address, array)) = snapshot.0.last_mut() {
                if *address != header.0 {
                    continue;
                }
                for entry in array.chunks_exact_mut(SCOUT_DAY_RESULT_SIZE) {
                    if i32::from_le_bytes([entry[0xc], entry[0xd], entry[0xe], entry[0xf]]) == person {
                        entry[0x2b] = 0;
                    }
                }
            }
        }
    }
    let same = after_original == after_copy && r == n;
    SCOREBOARD.record(same, || format!("{} original={:?} rust={:?} {:?}", label(), r, n, first_differences(&after_original, &after_copy)));
    r
}
