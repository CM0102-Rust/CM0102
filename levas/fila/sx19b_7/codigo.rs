// ---------- sx19b_7: the scout requests of the human manager and the report orders (scout_manager.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\scout_manager.cpp" (the error box).
const SCOUT_MANAGER_SOURCE: usize = 0xa7b094;
/// `PLAYER_SEARCH::clear_results` (0x7b14a0): thiscall(search, relink), ret 4.
const FN_PLAYER_SEARCH_CLEAR_RESULTS: usize = 0x7b14a0;
/// The bsearch/qsort order of the scout requests (`scout_manager_compare_report_keys`).
const SCOUT_REQUEST_ORDER: usize = 0x7fc270;

/// Returns a status code for asking scouts to watch a player: 3 already requested, 0 covered by one of the manager's
/// searches, 1 possible, 4 out of the club's range, 2 not possible.
///
/// `SCOUT_MANAGER::get_scout_request_status(person, club)` (thiscall, ret 8), from the player screens (0x889880,
/// 0x88b043, always with club 0): without a club, the club job of the human whose turn it is (none: the box at line
/// 0x66b and 2). A request of the club for the person (`has_scout_request`, any date): 3. The nation looked at is the
/// one of the person's club (club job +0x53) or else his own (+0x1a); none: 2. Each of the manager's 10 searches
/// (`scout_record`): one with a scout (`get_scout`) and of type 4 counts; one whose scout covers the nation
/// (`scout_covers_nation`): 0. No counted search: 2; otherwise 1 when the club can scout the nation
/// (`can_club_scout_nation`), else 4. No person: the original (box at line 0x65d, 2).
/// Defect of the original: with a club given, the manager passed to `scout_record` is a stack dword never written (the
/// callers always pass 0; the copy uses the current human).
/// Original: `SCOUT_MANAGER::get_scout_request_status` (0x7f8ff0, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_scout_request_status(this: usize, person: usize, club: usize) -> u8 {
    let manager = scout_manager_current_human_manager();
    let club = if club != 0 { club } else if manager != 0 { read_usize(manager + staff::CLUB_JOB) } else { 0 };
    if person == 0 || club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I0%%)(this, person, 0); // the box
    }
    if has_scout_request(this, person, club, 0) != 0 {
        return 3;
    }
    let job = Staff(person).club_job();
    let mut nation = if job.is_null() { 0 } else { read_usize(job.addr() + 0x53) }; // the club's nation
    if nation == 0 {
        nation = read_usize(person + 0x1a); // the person's nation
    }
    if nation == 0 {
        return 2;
    }
    let mut scouting = 0;
    for k in 0..10i8 {
        let s = scout_record(this, k, manager as *const i32);
        if get_scout(s) != 0 && read_u8(s + player_search_fields::LIST_TYPE) == 4 {
            scouting += 1;
        }
        if scout_covers_nation(s, nation) != 0 {
            covered!("sx19b_7 covered by a search");
            return 0;
        }
    }
    if scouting == 0 {
        return 2;
    }
    if can_club_scout_nation(this, nation, club) != 0 { 1 } else { 4 } // sx19b_7: in range
}

/// Comparer for `get_scout_request_status`: side by side.
pub(crate) unsafe extern "thiscall" fn compare_get_scout_request_status(this: usize, person: usize, club: usize) -> u8 {
    let ro = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I0%%)(this, person, club);
    let manager = scout_manager_current_human_manager();
    if person != 0 && club == 0 && manager != 0 && read_usize(manager + staff::CLUB_JOB) != 0 {
        let rr = get_scout_request_status(this, person, club);
        SCOREBOARD.record(ro == rr, || format!("SCOUT_MANAGER::get_scout_request_status (player screen, person {}): original {} rust {}", read_i32(person), ro, rr));
    }
    ro
}

/// The fatal end of an allocation that failed: the scout_manager.cpp "Error" box, the fatal cleanup and `exit(-1)`.
unsafe fn scout_requests_out_of_memory(line: u32) {
    show_error_at(SCOUT_MANAGER_SOURCE, line);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
}

/// Appends a scout request for a player by a club with today's date and no reports, growing the array in steps of ten.
///
/// `SCOUT_MANAGER::add_scout_request(&person_id, &club_id)` (thiscall, ret 8), from the player screen (0x88c419, club
/// 0): without a club, the club job of the human whose turn it is. Without an array, a new one of 10 requests
/// (`exe::crt_malloc` of 0xaa bytes; failing: the box at line 0x6f2 and `exit(-1)`), capacity +0x60 = 10; a full array
/// grows by 10 (`exe::crt_realloc`; failing: the box at line 0x6fe and `exit(-1)`). The new 17-byte request: the
/// player id (`scout_request::PLAYER_ID`), the club id (`CLUB_ID`), today (`DATE`, `Date::operator=` from
/// `G_CURRENT_DATE`), no reports (`REPORT_COUNT`); the count goes up and the array is sorted (`exe::crt_qsort` with
/// `SCOUT_REQUEST_ORDER`). No person (line 0x6da) or no club (0x6e8): the original ("Error" box).
/// Original: `SCOUT_MANAGER::add_scout_request` (0x7f9290, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_scout_request(this: usize, person: *const u32, club: *const u32) {
    let club = if !club.is_null() {
        club
    } else {
        let m = scout_manager_current_human_manager();
        read_usize(m + staff::CLUB_JOB) as *const u32
    };
    if person.is_null() || club.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(%%I1%%)(this, person, core::ptr::null()); // the box
    }
    let requests = this + scout_manager_fields::SCOUT_REQUESTS;
    let capacity = this + 0x60; // the scout request capacity
    let count = this + scout_manager_fields::SCOUT_REQUEST_COUNT;
    if read_usize(requests) == 0 {
        let p = exe::crt_malloc(0xaa);
        write_usize(requests, p);
        if p == 0 {
            scout_requests_out_of_memory(0x6f2);
        }
        write_u32(capacity, 10);
    }
    if read_u32(capacity) == read_u32(count) {
        let p = exe::crt_realloc(read_usize(requests), (read_u32(capacity) + 10).wrapping_mul(0x11));
        write_usize(requests, p);
        if p == 0 {
            scout_requests_out_of_memory(0x6fe);
        }
        write_u32(capacity, read_u32(capacity) + 10);
    }
    let r = read_usize(requests) + read_u32(count) as usize * 0x11;
    write_u32(r + scout_request::PLAYER_ID, *person);
    write_u32(r + scout_request::CLUB_ID, *club);
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(r + scout_request::DATE, tmp.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(r + scout_request::REPORT_COUNT, 0); // sx19b_7: no reports yet
    let n = read_u32(count) + 1;
    write_u32(count, n);
    exe::crt_qsort(read_usize(requests), n, 0x11, SCOUT_REQUEST_ORDER);
}

/// Comparer for `add_scout_request`: the request can be added once, so only the original runs; checked afterwards: one
/// request more, and the pair is there with no reports.
pub(crate) unsafe extern "thiscall" fn compare_add_scout_request(this: usize, person: *const u32, club: *const u32) {
    let before = read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT);
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(%%I1%%)(this, person, club);
    let m = scout_manager_current_human_manager();
    let c = if !club.is_null() { club } else if m != 0 { read_usize(m + staff::CLUB_JOB) as *const u32 } else { core::ptr::null() };
    if !person.is_null() && !c.is_null() {
        let key = [*c, *person];
        let n = read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT);
        let r = exe::crt_bsearch(key.as_ptr() as usize, read_usize(this + scout_manager_fields::SCOUT_REQUESTS), n, 0x11, SCOUT_REQUEST_ORDER);
        SCOREBOARD.record(n == before + 1 && r != 0, || format!("SCOUT_MANAGER::add_scout_request (player screen, person {}): {} requests (before {}), found {:#x}", *person, n, before, r));
    }
}

/// Marks one scout report made on a request by resetting its date and raising its report count.
///
/// `SCOUT_MANAGER::record_scout_report(&person_id, &club_id)` (thiscall, ret 8), from the scouting of a search
/// (0x7b64a6, PLAYER_SEARCH): without a club, the club job of the human whose turn it is. The request of the pair
/// (`exe::crt_bsearch` with `SCOUT_REQUEST_ORDER`; none: nothing) gets the date 27/04/1969 (`Date::Date(0x1b, 4, 0x7b1,
/// −1)`, the "no date" of the scouting [inference]) and one report more (`scout_request::REPORT_COUNT`). No person
/// (line 0x71a) or no club (0x728): the original ("Error" box).
/// Original: `SCOUT_MANAGER::record_scout_report` (0x7f9520, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn record_scout_report(this: usize, person: *const u32, club: *const u32) {
    let club = if !club.is_null() {
        club
    } else {
        let m = scout_manager_current_human_manager();
        read_usize(m + staff::CLUB_JOB) as *const u32
    };
    if person.is_null() || club.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(%%I2%%)(this, person, core::ptr::null()); // the box
    }
    let key = [*club, *person];
    let r = exe::crt_bsearch(key.as_ptr() as usize, read_usize(this + scout_manager_fields::SCOUT_REQUESTS),
        read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT), 0x11, SCOUT_REQUEST_ORDER);
    if r == 0 {
        return;
    }
    covered!("sx19b_7 scout report counted");
    let (mut date, mut tmp) = ([0u16; 4], [0u8; 8]);
    let d = exe::date_date_549f70(&mut date, 0x1b, 4, 0x7b1, u32::MAX);
    exe::date_operator_3d(r + scout_request::DATE, tmp.as_mut_ptr() as usize, d);
    write_u8(r + scout_request::REPORT_COUNT, read_u8(r + scout_request::REPORT_COUNT).wrapping_add(1)); // sx19b_7: one report more
}

/// Comparer for `record_scout_report`: only the original runs; checked afterwards: the request (when there is one) has
/// one report more.
pub(crate) unsafe extern "thiscall" fn compare_record_scout_report(this: usize, person: *const u32, club: *const u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(%%I2%%);
    let m = scout_manager_current_human_manager();
    let c = if !club.is_null() { club } else if m != 0 { read_usize(m + staff::CLUB_JOB) as *const u32 } else { core::ptr::null() };
    if person.is_null() || c.is_null() {
        return o(this, person, club);
    }
    let key = [*c, *person];
    let r = exe::crt_bsearch(key.as_ptr() as usize, read_usize(this + scout_manager_fields::SCOUT_REQUESTS),
        read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT), 0x11, SCOUT_REQUEST_ORDER);
    let before = if r != 0 { read_u8(r + scout_request::REPORT_COUNT) } else { 0 };
    o(this, person, club);
    if r != 0 {
        let after = read_u8(r + scout_request::REPORT_COUNT);
        SCOREBOARD.record(after == before.wrapping_add(1), || format!("SCOUT_MANAGER::record_scout_report (person {}): reports {} -> {}", *person, before, after));
    }
}

/// Orders two scout request keys (qsort/bsearch): a null first is −1, a null second 1; then by the player id (+4) and
/// then the club id (+0), by subtraction.
/// Original: `scout_manager_compare_report_keys` (0x7fc270, scout_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn scout_manager_compare_report_keys(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (pa, pb) = (read_i32(a + scout_request::PLAYER_ID), read_i32(b + scout_request::PLAYER_ID));
    if pa != pb {
        return pa.wrapping_sub(pb); // sx19b_7: by the player first
    }
    read_i32(a + scout_request::CLUB_ID).wrapping_sub(read_i32(b + scout_request::CLUB_ID))
}

/// Comparer for `scout_manager_compare_report_keys`: side by side.
pub(crate) unsafe extern "C" fn compare_scout_manager_compare_report_keys(a: usize, b: usize) -> i32 {
    let ro = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I3%%)(a, b);
    let rr = scout_manager_compare_report_keys(a, b);
    SCOREBOARD.record(ro == rr, || format!("scout_manager_compare_report_keys: original {} rust {}", ro, rr));
    ro
}

/// Orders two 0x67-byte scout report records (qsort): a null first is −1, a null second 1; a record without a report
/// (`scout_report::REPORT_MODE` = 0xff) first (−1 when the first is one, else 1 when the second is); then by the player
/// (`PLAYER_ID`), the club (`CLUB_ID`), the current rating (`CURRENT_RATING`) and the potential rating
/// (`POTENTIAL_RATING`, signed bytes), by subtraction.
/// Original: `scout_manager_compare_report_records` (0x7fc2a0, scout_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn scout_manager_compare_report_records(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    if read_u8(a + scout_report::REPORT_MODE) == 0xff {
        return -1;
    }
    if read_u8(b + scout_report::REPORT_MODE) == 0xff {
        return 1;
    }
    for f in [scout_report::PLAYER_ID, scout_report::CLUB_ID] {
        let (x, y) = (read_i32(a + f), read_i32(b + f));
        if x != y {
            return x.wrapping_sub(y);
        }
    }
    let (x, y) = (read_i8(a + scout_report::CURRENT_RATING), read_i8(b + scout_report::CURRENT_RATING));
    if x != y {
        return x as i32 - y as i32;
    }
    read_i8(a + scout_report::POTENTIAL_RATING) as i32 - read_i8(b + scout_report::POTENTIAL_RATING) as i32
}

/// Comparer for `scout_manager_compare_report_records`: side by side.
pub(crate) unsafe extern "C" fn compare_scout_manager_compare_report_records(a: usize, b: usize) -> i32 {
    let ro = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I4%%)(a, b);
    let rr = scout_manager_compare_report_records(a, b);
    SCOREBOARD.record(ro == rr, || format!("scout_manager_compare_report_records: original {} rust {}", ro, rr));
    ro
}

/// Clears all ten player searches that belong to one human manager when that manager leaves the game.
///
/// `SCOUT_MANAGER::clear_manager_lists(manager)` (thiscall, ret 4), from a human manager leaving (0x6b2ce2): each of
/// his 10 searches (`scout_record`) has its results cleared (`PLAYER_SEARCH::clear_results` with 1). The searches are
/// not checked for null.
/// Original: `SCOUT_MANAGER::clear_manager_lists` (0x7fc310, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_manager_lists(this: usize, manager: usize) {
    let clear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_PLAYER_SEARCH_CLEAR_RESULTS);
    for k in 0..10i8 {
        clear(scout_record(this, k, manager as *const i32), 1);
    }
}

/// Comparer for `clear_manager_lists`: it empties the lists once, so only the original runs; checked afterwards: the
/// ten searches have no results.
pub(crate) unsafe extern "thiscall" fn compare_clear_manager_lists(this: usize, manager: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I5%%)(this, manager);
    let left: Vec<u32> = (0..10i8).map(|k| scout_record(this, k, manager as *const i32)).filter(|&s| s != 0)
        .map(|s| read_u32(s + player_search_fields::RESULT_COUNT)).collect();
    SCOREBOARD.record(left.iter().all(|&n| n == 0), || format!("SCOUT_MANAGER::clear_manager_lists (manager leaving): results left {:?}", left));
}
