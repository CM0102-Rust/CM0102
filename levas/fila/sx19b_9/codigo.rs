// ---------- sx19b_9: the search edit session of the scout screens, second part (search_edit_session.cpp) ----------

/// `PLAYER_SEARCH` routines the session calls on its current search (thiscall): `set_password` (0x7ae7d0, ret 4),
/// `set_manager_name` (0x7ae780, ret 4), `start_scout_assignment` (0x7ae9c0, ret 0x14), `clear_scout_assignment`
/// (0x7aee80), `clear_results` (0x7b14a0, ret 4), `get_result_list` (0x7af160, ret 0xc), `load_from_file` (0x7b2bf0,
/// ret 8) and `save_to_file` (0x7b3660, ret 8).
const FN_SEARCH_SET_PASSWORD: usize = 0x7ae7d0;
const FN_SEARCH_SET_MANAGER_NAME: usize = 0x7ae780;
const FN_SEARCH_START_SCOUT_ASSIGNMENT: usize = 0x7ae9c0;
const FN_SEARCH_CLEAR_SCOUT_ASSIGNMENT: usize = 0x7aee80;
const FN_SEARCH_CLEAR_RESULTS: usize = 0x7b14a0;
const FN_SEARCH_GET_RESULT_LIST: usize = 0x7af160;
const FN_SEARCH_LOAD_FROM_FILE: usize = 0x7b2bf0;
const FN_SEARCH_SAVE_TO_FILE: usize = 0x7b3660;
/// `HUMAN_MANAGER::add_recent_file` (0x5fcb60): thiscall(`G_HUMAN_MANAGER`, file, 0, 2), ret 0xc.
const FN_SESSION_ADD_RECENT_FILE: usize = 0x5fcb60;

/// A thiscall exe routine on the session's current search with one argument (ret 4).
unsafe fn current_search_call(s: usize, f: usize, a: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(f);
    f(session_current_search(s), a)
}

/// The current search's index as a flag index.
unsafe fn current_index(s: usize) -> usize {
    read_i8(s) as isize as usize
}

/// A change of the current search: the undo snapshot, the change, the search and the session marked changed and, with
/// `stale`, the results not fresh. Returns 1.
unsafe fn change_current_search(s: usize, stale: bool, change: impl FnOnce()) -> u32 {
    session_take_undo(s);
    change();
    session_mark_changed(s, current_index(s));
    if stale {
        write_u32(s + SESSION_RESULTS_FRESH, 0);
    }
    1
}

/// Comparer of a change of the current search: only the original runs; checked afterwards: the change recorded (and,
/// with `stale`, the results not fresh).
unsafe fn compare_current_search_change(s: usize, r: u32, stale: bool, name: &str) {
    let ok = r == 1 && session_change_recorded(s, current_index(s)) && (!stale || read_u32(s + SESSION_RESULTS_FRESH) == 0);
    SCOREBOARD.record(ok, || format!("SearchEditSession::{} (search screen, search {}): answered {}", name, read_i8(s), r));
}

/// Takes an undo snapshot and writes the current manager's password into the current search when it has none, then
/// marks it changed.
///
/// `SearchEditSession::lock_current_search()` (thiscall), from the search screen (0x80f616, 0x80f661): without a
/// password (the first byte of +0x139), the undo snapshot, `PLAYER_SEARCH::set_password` with 0 (the manager's own) and
/// the search and the session changed. Returns 1.
/// Original: `SearchEditSession::lock_current_search` (0x804810, search_edit_session.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn lock_current_search(this: usize) -> u32 {
    if read_u8(session_current_search(this) + 0x139) == 0 { // the search's password
        change_current_search(this, false, || { current_search_call(this, FN_SEARCH_SET_PASSWORD, 0); });
    }
    1
}

/// Comparer for `lock_current_search`: only the original runs; checked afterwards: an unlocked search got its change
/// recorded.
pub(crate) unsafe extern "thiscall" fn compare_lock_current_search(this: usize) -> u32 {
    let unlocked = read_u8(session_current_search(this) + 0x139) == 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    if unlocked {
        compare_current_search_change(this, r, false, "lock_current_search");
    }
    r
}

/// Takes an undo snapshot, then points the current scout list at a nation, region, competition, youth or next
/// opposition target and marks it changed.
///
/// `SearchEditSession::set_search_scope(a, b, c, d, e)` (thiscall, ret 0x14), from the search screen (0x80ec4b,
/// 0x80ed86, 0x80edc9, 0x80edf8, 0x80ee37): the undo snapshot, `PLAYER_SEARCH::start_scout_assignment` with the five
/// arguments, the search and the session changed (the results stay as they are). Returns 1.
/// Original: `SearchEditSession::set_search_scope` (0x8048e0, search_edit_session.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn set_search_scope(this: usize, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    change_current_search(this, false, || {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_SEARCH_START_SCOUT_ASSIGNMENT);
        f(session_current_search(this), a, b, c, d, e); // sx19b_9: the scout's target
    })
}

/// Comparer for `set_search_scope`.
pub(crate) unsafe extern "thiscall" fn compare_set_search_scope(this: usize, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(%%I1%%)(this, a, b, c, d, e);
    compare_current_search_change(this, r, false, "set_search_scope");
    r
}

/// Takes an undo snapshot, recalls the scout of the current search by clearing its scout assignment and marks that
/// search as changed.
///
/// `SearchEditSession::reset_scout_fields()` (thiscall), from the search screen (0x80ee5d): the undo snapshot,
/// `PLAYER_SEARCH::clear_scout_assignment`, the search and the session changed. Returns 1.
/// Original: `SearchEditSession::reset_scout_fields` (0x8049a0, search_edit_session.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn reset_scout_fields(this: usize) -> u32 {
    change_current_search(this, false, || {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SEARCH_CLEAR_SCOUT_ASSIGNMENT);
        f(session_current_search(this));
    })
}

/// Comparer for `reset_scout_fields`.
pub(crate) unsafe extern "thiscall" fn compare_reset_scout_fields(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    compare_current_search_change(this, r, false, "reset_scout_fields");
    r
}

/// Returns the result list of the current search and refreshes it once through the player search code when the cache
/// flag is clear.
///
/// `SearchEditSession::get_current_results(&list, &outside_own_squad, order)` (thiscall, ret 0xc), from the search
/// screen (0x808e47): the list is the session's (`SESSION_RESULTS`); when the results are not fresh, the current
/// search fills it (`PLAYER_SEARCH::get_result_list` with the list, the session's flag `SESSION_OUTSIDE_OWN_SQUAD` and
/// the order), the count it saw goes to `SESSION_RESULTS_SEEN` and the results are fresh. The flag is handed out and
/// the count returned.
/// Original: `SearchEditSession::get_current_results` (0x804a40, search_edit_session.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn get_current_results(this: usize, list: *mut usize, outside_own_squad: *mut u32, order: u32) -> u32 {
    *list = this + SESSION_RESULTS;
    if read_u32(this + SESSION_RESULTS_FRESH) == 0 {
        covered!("sx19b_9 results refreshed");
        let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_SEARCH_GET_RESULT_LIST);
        write_u32(this + SESSION_RESULTS_SEEN, f(session_current_search(this), this + SESSION_RESULTS, this + SESSION_OUTSIDE_OWN_SQUAD, order));
        write_u32(this + SESSION_RESULTS_FRESH, 1);
    }
    *outside_own_squad = read_u32(this + SESSION_OUTSIDE_OWN_SQUAD);
    read_u32(this + SESSION_RESULTS_SEEN) // sx19b_9: the results seen
}

/// Comparer for `get_current_results`: with fresh results it only reads, so both run side by side; otherwise only the
/// original runs (the list is refilled once) and the results must be fresh afterwards.
pub(crate) unsafe extern "thiscall" fn compare_get_current_results(this: usize, list: *mut usize, outside_own_squad: *mut u32, order: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut u32, u32) -> u32>(%%I3%%);
    let fresh = read_u32(this + SESSION_RESULTS_FRESH) != 0;
    let ro = o(this, list, outside_own_squad, order);
    let (lo, fo) = (*list, *outside_own_squad);
    if fresh {
        let (mut lr, mut fr) = (0usize, 0u32);
        let rr = get_current_results(this, &mut lr, &mut fr, order);
        SCOREBOARD.record(ro == rr && lo == lr && fo == fr, || format!("SearchEditSession::get_current_results (search screen): original {} {:#x} {} rust {} {:#x} {}", ro, lo, fo, rr, lr, fr));
    } else {
        SCOREBOARD.record(read_u32(this + SESSION_RESULTS_FRESH) == 1 && lo == this + SESSION_RESULTS, || format!("SearchEditSession::get_current_results (search screen): fresh {} list {:#x}", read_u32(this + SESSION_RESULTS_FRESH), lo));
    }
    ro
}

/// Takes an undo snapshot, removes the player from the current search's result list, marks it changed and clears the
/// result cache.
///
/// `SearchEditSession::remove_player_from_current_search(person)` (thiscall, ret 4), from the search screen (0x8100b4):
/// the undo snapshot, `PLAYER_SEARCH::remove_player`, the search and the session changed, the results not fresh.
/// Returns 1.
/// Original: `SearchEditSession::remove_player_from_current_search` (0x804ab0, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_player_from_current_search(this: usize, person: usize) -> u32 {
    change_current_search(this, true, || { current_search_call(this, FN_PLAYER_SEARCH_REMOVE, person); })
}

/// Comparer for `remove_player_from_current_search`.
pub(crate) unsafe extern "thiscall" fn compare_remove_player_from_current_search(this: usize, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I4%%)(this, person);
    compare_current_search_change(this, r, true, "remove_player_from_current_search");
    r
}

/// Takes an undo snapshot, empties the result list of the current search, marks it changed and clears the result
/// cache.
///
/// `SearchEditSession::clear_current_results(relink)` (thiscall, ret 4), from the search screen (0x80fdd2): the undo
/// snapshot, `PLAYER_SEARCH::clear_results` with the argument, the search and the session changed, the results not
/// fresh. Returns 1.
/// Original: `SearchEditSession::clear_current_results` (0x804b60, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_current_results(this: usize, relink: u32) -> u32 {
    change_current_search(this, true, || { current_search_call(this, FN_SEARCH_CLEAR_RESULTS, relink as usize); })
}

/// Comparer for `clear_current_results`: as the other changes, plus no results left.
pub(crate) unsafe extern "thiscall" fn compare_clear_current_results(this: usize, relink: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I5%%)(this, relink);
    compare_current_search_change(this, r, true, "clear_current_results");
    let left = read_u32(session_current_search(this) + player_search_fields::RESULT_COUNT);
    SCOREBOARD.record(left == 0, || format!("SearchEditSession::clear_current_results (search screen): {} results left", left));
    r
}

/// Takes an undo snapshot, copies a 0x99-byte filter block into the current search, clears its cached count and marks
/// it changed.
///
/// `SearchEditSession::set_current_filters(filters)` (thiscall, ret 4), from the search filter screen (0x8186df): the
/// undo snapshot, the 0x99 bytes of `filters` onto the current search's filters (+0x1fa), its cached count (+0x2a4) 0,
/// the search and the session changed, the results not fresh.
/// Original: `SearchEditSession::set_current_filters` (0x804c10, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_current_filters(this: usize, filters: usize) {
    change_current_search(this, true, || {
        let s = session_current_search(this);
        core::ptr::copy(filters as *const u8, (s + 0x1fa) as *mut u8, 0x99); // sx19b_9: the filters (+0x1fa)
        write_u32(s + 0x2a4, 0); // the cached count
    });
}

/// Comparer for `set_current_filters`: as the other changes, plus the filters equal to the block.
pub(crate) unsafe extern "thiscall" fn compare_set_current_filters(this: usize, filters: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I6%%)(this, filters);
    compare_current_search_change(this, 1, true, "set_current_filters");
    let s = session_current_search(this);
    let same = (0..0x99).all(|k| read_u8(s + 0x1fa + k) == read_u8(filters + k)) && read_u32(s + 0x2a4) == 0;
    SCOREBOARD.record(same, || "SearchEditSession::set_current_filters (search filter screen): the filters differ from the block".to_string());
}

/// Takes an undo snapshot and loads a search file into the current search, adding it to recent files, or restores the
/// snapshot when loading fails.
///
/// `SearchEditSession::load_search_file(file)` (thiscall, ret 4), from the search screen (0x80f214, 0x80ff28,
/// 0x8123fd): the undo snapshot; `PLAYER_SEARCH::load_from_file` (file, 1) answering: the search and the session
/// changed, the results not fresh, the file in the recent files (`HUMAN_MANAGER::add_recent_file`) and 1. Failing:
/// everything back from the undo block (current, searches, flags, "anything changed"), no undo, the results not fresh,
/// and 0.
/// Original: `SearchEditSession::load_search_file` (0x804cd0, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_search_file(this: usize, file: usize) -> u32 {
    session_take_undo(this);
    let load: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SEARCH_LOAD_FROM_FILE);
    if load(session_current_search(this), file, 1) != 0 {
        session_mark_changed(this, current_index(this));
        write_u32(this + SESSION_RESULTS_FRESH, 0);
        let recent: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_SESSION_ADD_RECENT_FILE);
        recent(read_usize(G_HUMAN_MANAGER), file, 0, 2);
        return 1;
    }
    covered!("sx19b_9 load undone");
    write_u8(this, read_u8(this + SESSION_UNDO));
    for k in 0..10 {
        search_assign(session_search(this, SESSION_SEARCHES, k), session_search(this, SESSION_UNDO + 1, k));
        write_u32(this + SESSION_CHANGED + k * 4, read_u32(this + SESSION_UNDO + SESSION_CHANGED + k * 4));
    }
    write_u32(this + SESSION_ANY_CHANGED, read_u32(this + SESSION_UNDO + SESSION_ANY_CHANGED));
    write_u32(this + SESSION_HAS_UNDO, 0); // sx19b_9: the undo spent
    write_u32(this + SESSION_RESULTS_FRESH, 0);
    0
}

/// Comparer for `load_search_file`: it reads the file once, so only the original runs; checked afterwards: loaded, the
/// change recorded; failed, no undo left.
pub(crate) unsafe extern "thiscall" fn compare_load_search_file(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I7%%)(this, file);
    if r != 0 {
        compare_current_search_change(this, r, true, "load_search_file");
    } else {
        SCOREBOARD.record(read_u32(this + SESSION_HAS_UNDO) == 0, || "SearchEditSession::load_search_file (search screen): failed and the undo is still held".to_string());
    }
    r
}

/// Saves the current search to a file, with an optional temporary lock owner, then clears the undo flag and adds the
/// file to recent files.
///
/// `SearchEditSession::save_current_search(file, owner)` (thiscall, ret 8), from the search screen (0x80f626,
/// 0x8116d6): the manager's name on the search (`PLAYER_SEARCH::set_manager_name` with 0); with an owner, his password
/// set for the save (`set_password`) and taken off after it (0); `PLAYER_SEARCH::save_to_file` (file, 1). Failing: 0.
/// Saved: no undo, the file in the recent files, 1.
/// Original: `SearchEditSession::save_current_search` (0x804e10, search_edit_session.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn save_current_search(this: usize, file: usize, owner: usize) -> u32 {
    current_search_call(this, FN_SEARCH_SET_MANAGER_NAME, 0);
    let save: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SEARCH_SAVE_TO_FILE);
    let saved = if owner != 0 {
        current_search_call(this, FN_SEARCH_SET_PASSWORD, owner);
        let r = save(session_current_search(this), file, 1);
        current_search_call(this, FN_SEARCH_SET_PASSWORD, 0);
        r
    } else {
        save(session_current_search(this), file, 1)
    };
    if saved == 0 {
        return 0;
    }
    write_u32(this + SESSION_HAS_UNDO, 0);
    let recent: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_SESSION_ADD_RECENT_FILE);
    recent(read_usize(G_HUMAN_MANAGER), file, 0, 2);
    1
}

/// Comparer for `save_current_search`: it writes the file once, so only the original runs; checked afterwards: saved,
/// no undo left.
pub(crate) unsafe extern "thiscall" fn compare_save_current_search(this: usize, file: usize, owner: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I8%%)(this, file, owner);
    SCOREBOARD.record(r == 0 || read_u32(this + SESSION_HAS_UNDO) == 0, || "SearchEditSession::save_current_search (search screen): saved and the undo is still held".to_string());
    r
}
