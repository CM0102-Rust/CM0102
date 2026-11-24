// --- sx17_5: filling the player search's result list (player_search.cpp): get_result_list (0x7af160),
// collect_known_people (0x7afa70) and add_own_squad (0x7b0e40); uses the result list routines of sx17_2 ---

/// `SearchFilters::player_matches_filter` (0x807450, search_filters.cpp, thiscall, ret 4): whether a search result
/// passes every active filter.
const FN_SEARCH_FILTERS_PLAYER_MATCHES_FILTER: usize = 0x807450;
/// The two flag masks of a SearchFilters record at its defaults (`SearchFilters::set_defaults`): no filter is active.
const SEARCH_FILTERS_DEFAULT_MASKS: (u32, u32) = (0x14140081, 0x44aaa884);
/// The SearchFilters record of the search (0x99 bytes): +0 and +4 the flag masks.
const PLAYER_SEARCH_FILTERS: usize = 0x1fa;
/// Flag mask bit of the SearchFilters (+4): leave the own club's players out.
const FILTER_EXCLUDE_OWN_CLUB: u32 = 0x4000000;
/// Flag mask bits of the SearchFilters (+4): leave out the reserve players / the first team players.
const FILTER_EXCLUDE_RESERVES: u32 = 0x10000000;
const FILTER_EXCLUDE_FIRST_TEAM: u32 = 0x20000000;
/// The search screen's orders that need a byte filled first: by status code (+0x2b) and by note rating (+0x2c).
const SEARCH_ORDER_BY_STATUS_CODE: usize = 0x819870;
const SEARCH_ORDER_BY_NOTE_RATING: usize = 0x8189f0;

/// The owning manager's note rating on `person` (`Notes::find_note`, byte +0x113), 0xff without a note.
unsafe fn player_search_note_rating(this: usize, person: Staff) -> u8 {
    let note = exe::notes_find_note(read_usize(G_NOTES), staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize), person) as usize;
    if note != 0 { read_u8(note + 0x113) } else { 0xff }
}

/// Appends `entry` to the result array without sorting, growing it by 250 when full (`realloc`; when it fails: the
/// player_search.cpp "Error" box at `line`, `game_fatal_cleanup` and `exit(-1)`).
unsafe fn player_search_append_result(this: usize, entry: usize, line: u32) {
    let (count, capacity) = (this + player_search_fields::RESULT_COUNT, this + SCOUT_DAY_RESULT_CAPACITY);
    if read_u32(count) == read_u32(capacity) {
        write_i32(capacity, read_i32(capacity) + 250);
        let array = exe::crt_realloc(read_usize(this + player_search_fields::RESULTS), read_u32(capacity).wrapping_mul(SCOUT_DAY_RESULT_SIZE as u32));
        write_usize(this + player_search_fields::RESULTS, array);
        if array == 0 {
            show_error_box(0xa701b0, line); // player_search.cpp
            exe::game_fatal_cleanup();
            exe::shared_noop_ret0_5a8f60();
            exe::crt_exit(-1);
        }
    }
    search_result_copy(read_usize(this + player_search_fields::RESULTS) + read_u32(count) as usize * SCOUT_DAY_RESULT_SIZE, entry);
    write_i32(count, read_i32(count) + 1);
}

/// A new result for `person` built on `entry`: his name ref, today as the last update, the flags, the ratings and the
/// bytes +0x29/+0x2a; the byte +0x2b is left as it was (the original's stack leftover) and +0x2c is the note rating.
unsafe fn player_search_fill_result(this: usize, entry: usize, person: Staff, flags: u8, rating: i32) {
    exe::fill_staff_name_ref(person, entry);
    let mut copy = [0u8; 8];
    exe::date_operator_3d(entry + search_result::LAST_UPDATE_DATE, copy.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(entry + search_result::FLAGS, flags);
    write_i32(entry + 0x21, rating);
    write_u32(entry + 0x25, 0);
    write_u8(entry + 0x29, 0);
    write_u8(entry + 0x2a, 0);
    write_u8(entry + 0x2c, player_search_note_rating(this, person));
}

/// Adds the players of the owning manager's club squad (`database_collect_club_squad`, 50 slots) to the result list as
/// own squad entries (flag 2), unless the filters leave the own club out (`FILTER_EXCLUDE_OWN_CLUB`; then 0) or the
/// manager has no club (0). With a reserve squad (`database_club_has_reserve_squad`) the filters may leave out the
/// reserve (playing squad bit 2) or the first team (bit 1); otherwise, with a linked B team (`club_get_linked_b_team`),
/// the players contracted to the B team or to the club. Each entry is built by `player_search_fill_result` and, when the
/// person is not listed yet (binary search), appended (the box line 0x6e0 when the array cannot grow) and the list
/// sorted. Answers 1.
/// Defect of the original: the entry's byte +0x2b is never set (the stack leftover goes into the list).
/// Original: `PLAYER_SEARCH::add_own_squad` (0x7b0e40, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn player_search_add_own_squad(this: usize) -> u32 {
    let mut entry = [0u8; SCOUT_DAY_RESULT_SIZE];
    let e = entry.as_mut_ptr() as usize;
    exe::date_date_54a110(e + 0x10);
    exe::date_date_54a110(e + search_result::LAST_UPDATE_DATE);
    if read_u32(this + PLAYER_SEARCH_FILTERS + 4) & FILTER_EXCLUDE_OWN_CLUB != 0 {
        return 0;
    }
    let club = staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize).club_job();
    if club.is_null() {
        return 0;
    }
    let b_team = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
    let mut squad = [0usize; 0x32];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 0);
    for &slot in squad.iter() {
        if slot == 0 {
            continue;
        }
        let player = Staff(slot);
        let masks = read_u32(this + PLAYER_SEARCH_FILTERS + 4);
        let excluded = if exe::database_club_has_reserve_squad(club) != 0 {
            (masks & FILTER_EXCLUDE_RESERVES != 0 && player.playing_squad() & 2 != 0)
                || (masks & FILTER_EXCLUDE_FIRST_TEAM != 0 && player.playing_squad() & 1 != 0)
        } else if b_team != 0 {
            (masks & FILTER_EXCLUDE_RESERVES != 0 && player.club_job().addr() == b_team)
                || (masks & FILTER_EXCLUDE_FIRST_TEAM != 0 && player.club_job() == club)
        } else {
            false
        };
        if excluded {
            continue;
        }
        player_search_fill_result(this, e, player, 2, 0); // the own squad flag
        let count = read_u32(this + player_search_fields::RESULT_COUNT);
        if count != 0 && exe::crt_bsearch(e, read_usize(this + player_search_fields::RESULTS), count, SCOUT_DAY_RESULT_SIZE as u32,
                                          player_search_compare_results_by_person as *const () as usize) != 0 {
            continue;
        }
        covered!("sx17_5 add an own squad player");
        player_search_append_result(this, e, 0x6e0);
        player_search_sort_results(this);
    }
    1
}

/// Comparer for `player_search_add_own_squad`: side by side over the result list (`player_search_compare_results`),
/// ignoring the byte +0x2b of the own squad entries (the original's stack leftover); when the array grows (realloc)
/// only the original runs and the capacity must have grown by steps of 250.
pub(crate) unsafe extern "thiscall" fn compare_player_search_add_own_squad(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    player_search_compare_results(this, || format!("player_search_add_own_squad({:#x})", this), &[], |e| e[0x20] & 2 != 0,
        || o(this), || player_search_add_own_squad(this), |(_, capacity, _)| {
            let now = read_i32(this + SCOUT_DAY_RESULT_CAPACITY);
            now >= read_i32(this + player_search_fields::RESULT_COUNT) && (now - capacity) % 250 == 0
        })
}

/// Clears the slots of the scouted club record whose player is gone from the club: a slot keeps its player while the
/// person is outside the staff table or still has a job at the record's club; otherwise its id becomes −1 and its
/// match count and history ring are zeroed.
unsafe fn player_search_forget_departed(record: usize) {
    for k in 0..0x32 {
        let slot = record + 4 + k * 0x1a2;
        let id = read_i32(slot);
        if id == -1 {
            continue;
        }
        let person = staff_at(id as isize);
        if !person.is_null() && read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let club = person.club_job();
        if !club.is_null() && read_i32(club.addr()) == read_i32(record) {
            continue;
        }
        write_i32(slot, -1);
        write_u16(slot + 4, 0);
        write_u8(slot + scouted_player_slot::HISTORY_START, 0);
        write_u8(slot + scouted_player_slot::HISTORY_COUNT, 0);
    }
}

/// Copies the results that pass the filters into `copy` (the entries removed by flag bit 0 skipped; with the filters
/// at their defaults every entry passes, otherwise `SearchFilters::player_matches_filter`); answers (how many were
/// copied, how many were looked at), and sets `outside_own_squad` when one of those is not of the own squad.
unsafe fn player_search_filter_results(this: usize, copy: usize, outside_own_squad: *mut u32) -> (i32, i32) {
    let filters = this + PLAYER_SEARCH_FILTERS;
    let at_defaults = (read_u32(filters), read_u32(filters + 4)) == SEARCH_FILTERS_DEFAULT_MASKS;
    let matches: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SEARCH_FILTERS_PLAYER_MATCHES_FILTER);
    let (mut copied, mut seen) = (0, 0);
    let mut k = 0;
    while k < read_i32(this + player_search_fields::RESULT_COUNT) {
        let entry = read_usize(this + player_search_fields::RESULTS) + k as usize * SCOUT_DAY_RESULT_SIZE;
        k += 1;
        if read_u8(entry + search_result::FLAGS) & 1 != 0 {
            continue;
        }
        if at_defaults || matches(filters, entry) != 0 {
            search_result_copy(copy + copied as usize * SCOUT_DAY_RESULT_SIZE, entry);
            copied += 1;
        }
        seen += 1;
        if read_u8(entry + search_result::FLAGS) & 2 == 0 {
            *outside_own_squad = 1; // sx17_5: someone outside the own squad
        }
    }
    (copied, seen)
}

/// Fills the byte an order of the search screen sorts on: the status code (`get_player_status_code`) into +0x2b for
/// `SEARCH_ORDER_BY_STATUS_CODE`, the note rating into +0x2c for `SEARCH_ORDER_BY_NOTE_RATING`.
unsafe fn player_search_prepare_order(this: usize, copy: usize, n: i32, order: usize) {
    for k in 0..n as usize {
        let entry = copy + k * SCOUT_DAY_RESULT_SIZE;
        let person = staff_at(read_i32(entry + 0xc) as isize);
        if order == SEARCH_ORDER_BY_STATUS_CODE {
            write_u8(entry + 0x2b, exe::get_player_status_code(person, 0usize, 0usize));
        } else if order == SEARCH_ORDER_BY_NOTE_RATING {
            write_u8(entry + 0x2c, player_search_note_rating(this, person));
        }
    }
}

/// Refreshes the search results and hands them to the screen: without a list, the player_search.cpp "Error" box
/// (line 0x29a) and 0. The list is emptied (`LLIST::clear`). A quick or staff search (types 2, 3) collects the known
/// people again when it is empty or its first result was not updated today. With an assigned club and its scouted
/// club record, the departed players are dropped from the record (`player_search_forget_departed`) and the result
/// list emptied. The own squad is added (`add_own_squad`) and, with the club record, each recorded player
/// (`add_player`). With results: the invalid ones dropped (`purge_invalid_results`), those that pass the filters copied
/// (`player_search_filter_results`; `outside_own_squad` set when one is not of the own squad), filled for the order and
/// sorted with `order` when given, and each appended to `list` (0x2d bytes); a failed copy buffer shows the box (line
/// 0x2dc) and exits. Last, the own squad entries leave the result list (`remove_own_squad`). Answers how many results
/// were looked at.
/// Original: `PLAYER_SEARCH::get_result_list` (0x7af160, player_search.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn player_search_get_result_list(this: usize, list: usize, outside_own_squad: *mut u32, order: usize) -> u32 {
    if list == 0 {
        show_error_box(0xa701b0, 0x29a); // player_search.cpp
        return 0;
    }
    exe::llist_clear(list);
    if matches!(read_u8(this + player_search_fields::LIST_TYPE), 2 | 3) {
        let results = read_usize(this + player_search_fields::RESULTS);
        if read_u32(this + player_search_fields::RESULT_COUNT) == 0
            || exe::date_operator_3d_3d(results + search_result::LAST_UPDATE_DATE, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) == 0 {
            covered!("sx17_5 collect the known people again");
            player_search_collect_known_people(this);
        }
    }
    let watches_club = || read_i32(this + player_search_fields::ASSIGNED_CLUB) > -1 && read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD) != 0;
    if watches_club() {
        player_search_forget_departed(read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD));
        let results = read_usize(this + player_search_fields::RESULTS);
        if results != 0 {
            exe::crt_free(results);
            write_usize(this + player_search_fields::RESULTS, 0);
        }
        write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
        write_u32(this + player_search_fields::RESULT_COUNT, 0);
        write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
    }
    player_search_add_own_squad(this);
    *outside_own_squad = 0;
    if watches_club() {
        let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
        for k in 0..0x32 {
            let id = read_i32(record + 4 + k * 0x1a2);
            if id != -1 {
                player_search_add_player(this, staff_at(id as isize));
            }
        }
    }
    let mut seen = 0;
    let count = read_u32(this + player_search_fields::RESULT_COUNT);
    if count != 0 {
        let copy = exe::crt_malloc(count.wrapping_mul(SCOUT_DAY_RESULT_SIZE as u32));
        if copy == 0 {
            show_error_box(0xa701b0, 0x2dc); // player_search.cpp
            exe::game_fatal_cleanup();
            exe::shared_noop_ret0_5a8f60();
            exe::crt_exit(-1);
        }
        exe::player_search_purge_invalid_results(this, 0);
        let (n, looked_at) = player_search_filter_results(this, copy, outside_own_squad);
        seen = looked_at;
        if n > 0 {
            if order != 0 {
                player_search_prepare_order(this, copy, n, order);
                exe::crt_qsort(copy, n as u32, SCOUT_DAY_RESULT_SIZE as u32, order);
            }
            for k in 0..n as usize {
                exe::llist_add_tail(list, copy + k * SCOUT_DAY_RESULT_SIZE, SCOUT_DAY_RESULT_SIZE as u32);
            }
        }
        exe::crt_free(copy);
    }
    player_search_remove_own_squad(this);
    seen as u32
}

/// Comparer for `player_search_get_result_list`: it fills the screen's list (allocates), may collect the people again
/// and frees, so only the original runs, and the plan is checked: the list holds at most the results looked at and no
/// own squad entry is left in the result list.
pub(crate) unsafe extern "thiscall" fn compare_player_search_get_result_list(this: usize, list: usize, outside_own_squad: *mut u32, order: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32, usize) -> u32>(%%I0%%)(this, list, outside_own_squad, order);
    if list == 0 {
        return r;
    }
    let listed = Llist(list).count() as i32;
    let own_left = (0..read_i32(this + player_search_fields::RESULT_COUNT)).any(|k| {
        read_u8(read_usize(this + player_search_fields::RESULTS) + k as usize * SCOUT_DAY_RESULT_SIZE + search_result::FLAGS) & 2 != 0
    });
    SCOREBOARD.record(listed <= r as i32 && !own_left, || format!("player_search_get_result_list({:#x}) returned {} listed {} own squad left {}", this, r, listed, own_left));
    r
}

/// The nation a manager looks from: his club's nation, else his national side's nation (when he has one), else his own.
unsafe fn player_search_viewing_nation(owner: Staff) -> usize {
    let club = owner.club_job();
    if !club.is_null() && !club.nation().is_null() {
        return club.nation().addr();
    }
    if !owner.national_job().is_null() {
        return owner.national_job().nation().addr();
    }
    owner.nation().addr()
}

/// The nation a person belongs to for the continent test: his contract club's nation, else his national side's (unless
/// his national job is 0xb), else his own.
unsafe fn player_search_person_nation(person: Staff, club: usize) -> usize {
    if club != 0 && !Club(club).nation().is_null() {
        return Club(club).nation().addr();
    }
    if !person.national_job().is_null() && person.job_for_nation() != 0xb {
        return person.national_job().nation().addr();
    }
    person.nation().addr()
}

/// Whether a quick search (any type but 3) knows the player: a reputation the manager sees of at least 4000
/// (`database_get_staff_reputation_for_viewer`), a scout request of the manager's club (`has_scout_request`), or, from
/// the manager's continent (`nation::ACTUAL_REGION`), a player who is transfer or loan listed or eligible for the
/// manager's national side.
unsafe fn player_search_knows_player(owner: Staff, own_club: Club, view: usize, person: Staff, club: usize) -> Option<bool> {
    if exe::database_get_staff_reputation_for_viewer(person, owner, 0) >= 4000 { // sx17_5: the famous players
        return Some(true);
    }
    if exe::scout_manager_has_scout_request(read_usize(G_SCOUT_MANAGER), person, own_club, 0) != 0 {
        return Some(true);
    }
    let nation = player_search_person_nation(person, club);
    if nation == 0 || Nation(nation).actual_region() != Nation(view).actual_region() {
        return None;
    }
    exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person); // looked up again and dropped
    Some(exe::staff_is_transfer_listed_or_requested(person, 0usize) != 0 || exe::staff_is_loan_listed(person, 0usize) != 0
        || exe::national_teams_player_is_eligible(read_usize(G_NATIONAL_TEAMS), person, owner) != 0)
}

/// Rebuilds the result list with the people the owning manager knows: no manager, the player_search.cpp "Error" box
/// (line 0x3eb) and 0; no nation to look from (`player_search_viewing_nation`), the box (line 0x3fd) and 0. The list is
/// emptied; every person outside the manager's club (by contract, `player_search_contract_club`) and not listed yet
/// (`contains_player`) is taken when a staff search (type 3) finds a non-player, or another search a player the manager
/// knows (`player_search_knows_player`): the entry (`player_search_fill_result`) has as rating the reputation the
/// manager sees of a non-player (0 for others) and is appended (the box line 0x470 when the array cannot grow). The
/// list is sorted; then the results of the manager's other lists (`SCOUT_MANAGER::get_edit_session`: 10 searches from
/// +1, types other than 2 and 3) outside his club go in through `add_result`, and the list is sorted again. Answers 1.
/// Defect of the original: the entries' byte +0x2b is never set (the stack leftover goes into the list).
/// Original: `PLAYER_SEARCH::collect_known_people` (0x7afa70, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn player_search_collect_known_people(this: usize) -> u32 {
    let mut entry = [0u8; SCOUT_DAY_RESULT_SIZE];
    let e = entry.as_mut_ptr() as usize;
    exe::date_date_54a110(e + 0x10);
    exe::date_date_54a110(e + search_result::LAST_UPDATE_DATE);
    let owner = staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize);
    if owner.is_null() {
        show_error_box(0xa701b0, 0x3eb); // player_search.cpp
        return 0;
    }
    let own_club = owner.club_job();
    let view = player_search_viewing_nation(owner);
    if view == 0 {
        show_error_box(0xa701b0, 0x3fd); // player_search.cpp
        return 0;
    }
    let results = read_usize(this + player_search_fields::RESULTS);
    if results != 0 {
        exe::crt_free(results);
        write_usize(this + player_search_fields::RESULTS, 0);
    }
    write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
    write_u32(this + player_search_fields::RESULT_COUNT, 0);
    write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
    let staff_search = read_u8(this + player_search_fields::LIST_TYPE) == 3;
    for id in 0..read_i32(G_STAFF_COUNT) {
        let person = staff_at(id as isize);
        let club = player_search_contract_club(person);
        if !own_club.is_null() && club == own_club.addr() {
            continue;
        }
        let known = if staff_search {
            if person.non_player().is_null() { None } else { Some(true) }
        } else if person.player().is_null() {
            None
        } else {
            player_search_knows_player(owner, own_club, view, person, club)
        };
        let Some(known) = known else { continue };
        if contains_player(this, person.addr()) != 0 || !known {
            continue;
        }
        let rating = if person.non_player().is_null() { 0 } else { exe::database_get_staff_reputation_for_viewer(person, owner, 1) as i32 };
        player_search_fill_result(this, e, person, 0, rating);
        player_search_append_result(this, e, 0x470);
    }
    player_search_sort_results(this);
    let session = get_edit_session(read_usize(G_SCOUT_MANAGER), owner.addr());
    for k in 0..10 {
        let list = session + 1 + k * 0x2b0;
        if matches!(read_u8(list + player_search_fields::LIST_TYPE), 2 | 3) {
            continue;
        }
        let mut j = 0;
        while j < read_i32(list + player_search_fields::RESULT_COUNT) {
            let entry = read_usize(list + player_search_fields::RESULTS) + j as usize * SCOUT_DAY_RESULT_SIZE;
            let club = player_search_contract_club(staff_at(read_i32(entry + 0xc) as isize));
            if own_club.is_null() || club != own_club.addr() {
                player_search_add_result(this, entry);
            }
            j += 1;
        }
    }
    if read_u32(this + player_search_fields::RESULT_COUNT) != 0 {
        player_search_sort_results(this);
    }
    1
}

/// Comparer for `player_search_collect_known_people`: the original builds its list first; then the copy builds its own
/// from the same empty start (the original already freed the old array), the two lists are compared (the byte +0x2b
/// left aside: the original leaves it unset), the copy's array is freed and the original's put back. Without a manager
/// or a nation to look from only the original runs (the box).
pub(crate) unsafe extern "thiscall" fn compare_player_search_collect_known_people(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let owner = staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize);
    if owner.is_null() || player_search_viewing_nation(owner) == 0 {
        return o(this);
    }
    let r = o(this);
    let header = crate::comparison::MemorySnapshot::read(&[(this + player_search_fields::RESULTS, 0xc), (this + SCOUT_DAY_REPORT_MARK, 4)]);
    let list = |this: usize| -> Vec<u8> {
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(0) as usize * SCOUT_DAY_RESULT_SIZE;
        let mut v = core::slice::from_raw_parts(read_usize(this + player_search_fields::RESULTS) as *const u8, n).to_vec();
        v.chunks_exact_mut(SCOUT_DAY_RESULT_SIZE).for_each(|e| e[0x2b] = 0);
        v
    };
    let original_list = list(this);
    write_usize(this + player_search_fields::RESULTS, 0);
    write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
    write_u32(this + player_search_fields::RESULT_COUNT, 0);
    let n = crate::rng::pause_trace(|| player_search_collect_known_people(this));
    let copy_list = list(this);
    let capacity = read_u32(this + SCOUT_DAY_RESULT_CAPACITY);
    exe::crt_free(read_usize(this + player_search_fields::RESULTS));
    header.write_back();
    let same = r == n && original_list == copy_list && capacity == read_u32(this + SCOUT_DAY_RESULT_CAPACITY);
    SCOREBOARD.record(same, || {
        let k = original_list.iter().zip(copy_list.iter()).position(|(a, b)| a != b);
        format!("player_search_collect_known_people({:#x}) original={} rust={} lengths {}/{} first difference at {:?}", this, r, n,
                original_list.len(), copy_list.len(), k)
    });
    r
}
