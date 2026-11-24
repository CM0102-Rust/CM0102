// ---------- sx19b_8: the search edit session of the scout screens, first part (search_edit_session.cpp) ----------

/// The search edit session (0x3638 bytes): the current search (signed byte +0) and the 10 working searches from +1
/// (0x2b0 bytes each), their changed flags (+0x1ae1, 10 dwords) and the "anything changed" flag (+0x1b09); the undo
/// copy of all that from +0x1b0d (current, searches +0x1b0e, flags +0x35ee, anything +0x3616) and whether it holds one
/// (+0x361a); the result list (LLIST +0x361e), the count of results seen (+0x362c), the "outside the own squad" flag
/// (+0x3630) and whether the results are fresh (+0x3634).
const SESSION_SEARCHES: usize = 1;
const SESSION_CHANGED: usize = 0x1ae1;
const SESSION_ANY_CHANGED: usize = 0x1b09;
const SESSION_UNDO: usize = 0x1b0d;
const SESSION_HAS_UNDO: usize = 0x361a;
const SESSION_RESULTS: usize = 0x361e;
const SESSION_RESULTS_SEEN: usize = 0x362c;
const SESSION_OUTSIDE_OWN_SQUAD: usize = 0x3630;
const SESSION_RESULTS_FRESH: usize = 0x3634;

/// `PLAYER_SEARCH::operator=` (0x7b7690): thiscall(destination, source), ret 4.
const FN_PLAYER_SEARCH_ASSIGN: usize = 0x7b7690;
/// `PLAYER_SEARCH::add_player` (0x7af720) and `remove_player` (0x7b1260): thiscall(search, person), ret 4.
const FN_PLAYER_SEARCH_ADD: usize = 0x7af720;
const FN_PLAYER_SEARCH_REMOVE: usize = 0x7b1260;
/// `PLAYER_SEARCH::PLAYER_SEARCH` (0x7ae390) and `~PLAYER_SEARCH` (0x7ae4f0), for the vector constructor.
const FN_SEARCH_CONSTRUCT: usize = 0x7ae390;
const FN_SEARCH_DESTRUCT: usize = 0x7ae4f0;

/// Search k of a session block (the working one at `SESSION_SEARCHES`, the undo one at `SESSION_UNDO` + 1).
fn session_search(s: usize, block: usize, k: usize) -> usize {
    s + block + k * 0x2b0
}

/// The session's current search (the working search of the signed byte at +0).
unsafe fn session_current_search(s: usize) -> usize {
    s.wrapping_add(SESSION_SEARCHES).wrapping_add((read_i8(s) as isize as usize).wrapping_mul(0x2b0))
}

/// One search copied onto another (`PLAYER_SEARCH::operator=`).
unsafe fn search_assign(destination: usize, source: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_PLAYER_SEARCH_ASSIGN);
    f(destination, source);
}

/// The scout manager's search k of the current human manager (`scout_record` of `G_SCOUT_MANAGER`).
unsafe fn session_saved_search(k: i8) -> usize {
    scout_record(read_usize(G_SCOUT_MANAGER), k, core::ptr::null())
}

/// Takes the undo snapshot: the current search, the 10 searches, their changed flags and the "anything changed" flag
/// copied to the undo block, which now holds one (`SESSION_HAS_UNDO` = 1).
pub(crate) unsafe fn session_take_undo(s: usize) {
    write_u8(s + SESSION_UNDO, read_u8(s));
    for k in 0..10 {
        search_assign(session_search(s, SESSION_UNDO + 1, k), session_search(s, SESSION_SEARCHES, k));
        write_u32(s + SESSION_UNDO + SESSION_CHANGED + k * 4, read_u32(s + SESSION_CHANGED + k * 4));
    }
    write_u32(s + SESSION_UNDO + SESSION_ANY_CHANGED, read_u32(s + SESSION_ANY_CHANGED));
    write_u32(s + SESSION_HAS_UNDO, 1);
}

/// Marks search k (default: the current one) as changed, and the session too.
pub(crate) unsafe fn session_mark_changed(s: usize, k: usize) {
    write_u32(s + SESSION_CHANGED + k * 4, 1);
    write_u32(s + SESSION_ANY_CHANGED, 1);
}

/// The first of the session's 10 searches of type 1 (the short list), as its index.
unsafe fn session_short_list(s: usize) -> Option<usize> {
    (0..10).find(|&k| read_u8(session_search(s, SESSION_SEARCHES, k) + player_search_fields::LIST_TYPE) == 1)
}

/// Whether a change left search k marked changed and the undo snapshot taken (for the comparers).
pub(crate) unsafe fn session_change_recorded(s: usize, k: usize) -> bool {
    read_u32(s + SESSION_CHANGED + k * 4) == 1 && read_u32(s + SESSION_ANY_CHANGED) == 1 && read_u32(s + SESSION_HAS_UNDO) == 1
}

/// Builds a search edit session for the current human manager, copying his ten saved searches from the scout manager.
///
/// `SearchEditSession::SearchEditSession(list)` (thiscall, ret 4), from the search screen (0x8085a0): the two blocks of
/// 10 searches built (`exe::crt_vector_constructor_iterator` with `PLAYER_SEARCH::PLAYER_SEARCH`), the result list
/// (`LLIST::LLIST`); the current search is `list` (0..9) or else the manager's selected list (`get_selected_list` of
/// `G_SCOUT_MANAGER`); each working search k copied from the manager's search k (`session_saved_search`) and not
/// changed; nothing changed, no undo, the results not fresh. Returns the session.
/// Original: `SearchEditSession::SearchEditSession` (0x8041c0, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_search_edit_session(this: usize, list: u32) -> usize {
    exe::crt_vector_constructor_iterator(this + SESSION_SEARCHES, 0x2b0, 10, FN_SEARCH_CONSTRUCT, FN_SEARCH_DESTRUCT);
    exe::crt_vector_constructor_iterator(this + SESSION_UNDO + 1, 0x2b0, 10, FN_SEARCH_CONSTRUCT, FN_SEARCH_DESTRUCT);
    exe::new_llist(this + SESSION_RESULTS);
    let k = list as i8;
    write_i8(this, if (0..10).contains(&k) { k } else { get_selected_list(read_usize(G_SCOUT_MANAGER), 0) as i8 }); // sx19b_8: the current search
    for k in 0..10 {
        search_assign(session_search(this, SESSION_SEARCHES, k), session_saved_search(k as i8));
        write_u32(this + SESSION_CHANGED + k * 4, 0);
    }
    write_u32(this + SESSION_ANY_CHANGED, 0);
    write_u32(this + SESSION_HAS_UNDO, 0);
    write_u32(this + SESSION_RESULTS_FRESH, 0);
    this
}

/// Comparer for `new_search_edit_session`: it builds the searches (allocating), so only the original runs; checked
/// afterwards: the current search, no changes, no undo, the results not fresh.
pub(crate) unsafe extern "thiscall" fn compare_new_search_edit_session(this: usize, list: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, list);
    let k = list as i8;
    let current = read_i8(this);
    let ok = r == this && (!(0..10).contains(&k) || current == k) && (0..10).all(|i| read_u32(this + SESSION_CHANGED + i * 4) == 0)
        && read_u32(this + SESSION_ANY_CHANGED) == 0 && read_u32(this + SESSION_HAS_UNDO) == 0 && read_u32(this + SESSION_RESULTS_FRESH) == 0;
    SCOREBOARD.record(ok, || format!("SearchEditSession::SearchEditSession (search screen, list {}): current {}", k, current));
    r
}

/// Saves the working criteria as an undo copy, then reloads the current search from the scout manager and recomputes
/// the changed flag.
///
/// `SearchEditSession::revert_current_slot()` (thiscall), from the search screen (0x80fda4, 0x811ef7): the undo snapshot
/// (`session_take_undo`); the current search copied back from the manager's search (`session_saved_search`) and not
/// changed; "anything changed" again from the 10 flags; the results not fresh. Returns 1.
/// Original: `SearchEditSession::revert_current_slot` (0x8042e0, search_edit_session.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn revert_current_slot(this: usize) -> u32 {
    session_take_undo(this);
    let current = read_i8(this);
    search_assign(session_current_search(this), session_saved_search(current));
    write_u32(this.wrapping_add(SESSION_CHANGED).wrapping_add((current as isize as usize).wrapping_mul(4)), 0);
    let any = (0..10).any(|k| read_u32(this + SESSION_CHANGED + k * 4) != 0);
    write_u32(this + SESSION_ANY_CHANGED, any as u32); // sx19b_8: anything still changed
    write_u32(this + SESSION_RESULTS_FRESH, 0);
    1
}

/// Comparer for `revert_current_slot`: only the original runs; checked afterwards: the current search not changed, the
/// undo taken, the results not fresh.
pub(crate) unsafe extern "thiscall" fn compare_revert_current_slot(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    let k = read_i8(this) as isize as usize;
    let ok = r == 1 && read_u32(this.wrapping_add(SESSION_CHANGED).wrapping_add(k.wrapping_mul(4))) == 0
        && read_u32(this + SESSION_HAS_UNDO) == 1 && read_u32(this + SESSION_RESULTS_FRESH) == 0;
    SCOREBOARD.record(ok, || format!("SearchEditSession::revert_current_slot (search screen, search {})", k as i8));
    r
}

/// Copies the assigned scout of every saved scout list into the matching working and undo search of the session.
///
/// `SearchEditSession::refresh_scout_assignments()` (thiscall), from the scout manager (0x7f7403, `sync_scout_lists`):
/// for each of the manager's 10 searches of type 4 (scouts), its scout (`get_scout`) goes to working search k and undo
/// search k (`PLAYER_SEARCH::set_scout`). Returns 1.
/// Original: `SearchEditSession::refresh_scout_assignments` (0x8043c0, search_edit_session.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn refresh_scout_assignments(this: usize) -> u32 {
    for k in 0..10usize {
        if read_u8(session_saved_search(k as i8) + player_search_fields::LIST_TYPE) == 4 {
            covered!("sx19b_8 scout refreshed");
            set_scout(session_search(this, SESSION_SEARCHES, k), get_scout(session_saved_search(k as i8)));
            set_scout(session_search(this, SESSION_UNDO + 1, k), get_scout(session_saved_search(k as i8)));
        }
    }
    1
}

/// The scouts of the 10 working and undo searches (for the comparer).
unsafe fn session_scouts(s: usize) -> Vec<u32> {
    (0..10).flat_map(|k| [read_u32(session_search(s, SESSION_SEARCHES, k) + player_search_fields::SCOUT_ID),
        read_u32(session_search(s, SESSION_UNDO + 1, k) + player_search_fields::SCOUT_ID)]).collect()
}

/// Comparer for `refresh_scout_assignments`: the original runs, then the copy runs again on the same session (setting
/// the same scouts again changes nothing) and the 20 scouts must stay as the original left them.
pub(crate) unsafe extern "thiscall" fn compare_refresh_scout_assignments(this: usize) -> u32 {
    let ro = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let after = session_scouts(this);
    let rr = refresh_scout_assignments(this);
    let again = session_scouts(this);
    SCOREBOARD.record(ro == rr && after == again, || format!("SearchEditSession::refresh_scout_assignments (scout lists): original {:?} rust {:?}", after, again));
    ro
}

/// Stores the current list index as the manager's selected list, then writes all ten working searches back into the
/// scout manager.
///
/// `SearchEditSession::save_all_slots()` (thiscall), from the search screen (0x812dda, 0x8131d5): `set_selected_list`
/// with the current search, and each working search k copied onto the manager's search k. Returns 1.
/// Original: `SearchEditSession::save_all_slots` (0x804450, search_edit_session.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn save_all_slots(this: usize) -> u32 {
    set_selected_list(read_usize(G_SCOUT_MANAGER), read_u8(this) as u32, 0);
    for k in 0..10usize {
        search_assign(session_saved_search(k as i8), session_search(this, SESSION_SEARCHES, k)); // sx19b_8: back to the manager
    }
    1
}

/// Comparer for `save_all_slots`: only the original runs (the searches are copied once); checked afterwards: the
/// manager's selected list is the current search.
pub(crate) unsafe extern "thiscall" fn compare_save_all_slots(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I3%%)(this);
    let selected = get_selected_list(read_usize(G_SCOUT_MANAGER), 0);
    SCOREBOARD.record(r == 1 && selected == read_u8(this), || format!("SearchEditSession::save_all_slots (search screen): selected {} current {}", selected, read_u8(this)));
    r
}

/// Selects one of the ten searches as current and clears the result cache flag, or raises an assert for an index
/// outside 0 to 9.
///
/// `SearchEditSession::set_current_search(k)` (thiscall, ret 4), from the search screen (0x81056c, 0x810586): k (a
/// signed byte) in 0..9 becomes the current search and the results are not fresh; 1. Otherwise the original ("Error"
/// box, 0).
/// Original: `SearchEditSession::set_current_search` (0x8044a0, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_current_search(this: usize, k: u32) -> u32 {
    let k8 = k as i8;
    if !(0..10).contains(&k8) {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I4%%)(this, k); // the box
    }
    write_i8(this, k8);
    write_u32(this + SESSION_RESULTS_FRESH, 0);
    1
}

/// Comparer for `set_current_search`: only the original runs; checked afterwards: the current search and the results
/// not fresh.
pub(crate) unsafe extern "thiscall" fn compare_set_current_search(this: usize, k: u32) -> u32 {
    let ro = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I4%%)(this, k);
    if (0..10).contains(&(k as i8)) {
        let ok = ro == 1 && read_i8(this) == k as i8 && read_u32(this + SESSION_RESULTS_FRESH) == 0;
        SCOREBOARD.record(ok, || format!("SearchEditSession::set_current_search (search screen, search {}): current {}", k as i8, read_i8(this)));
    }
    ro
}

/// Finds the search slot of type short list and asks it whether the player is on the list.
///
/// `SearchEditSession::is_player_on_short_list(person)` (thiscall, ret 4), from the scout manager (0x7f7585,
/// `is_player_shortlisted`) and the screens (0x80d7c3): the first working search of type 1 answers
/// (`contains_player`); none: 0.
/// Original: `SearchEditSession::is_player_on_short_list` (0x804540, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn is_player_on_short_list(this: usize, person: usize) -> u32 {
    match session_short_list(this) {
        Some(k) => contains_player(session_search(this, SESSION_SEARCHES, k), person),
        None => 0,
    }
}

/// Comparer for `is_player_on_short_list`: side by side.
pub(crate) unsafe extern "thiscall" fn compare_is_player_on_short_list(this: usize, person: usize) -> u32 {
    let ro = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I5%%)(this, person);
    let rr = is_player_on_short_list(this, person);
    SCOREBOARD.record(ro == rr, || format!("SearchEditSession::is_player_on_short_list (person {}): original {} rust {}", read_i32(person), ro, rr));
    ro
}

/// Takes an undo snapshot, adds the player to the short list slot and marks that slot as changed.
///
/// `SearchEditSession::add_player_to_short_list(person)` (thiscall, ret 4), from the scout manager (0x7f76a8,
/// `add_to_shortlist`): the first working search of type 1 (none: 0): the undo snapshot, the player added
/// (`PLAYER_SEARCH::add_player`, the answer ignored), that search and the session changed, the results not fresh; 1.
/// Original: `SearchEditSession::add_player_to_short_list` (0x804590, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_player_to_short_list(this: usize, person: usize) -> u32 {
    short_list_change(this, person, FN_PLAYER_SEARCH_ADD)
}

/// The short list change shared by `add_player_to_short_list` and `remove_player_from_short_list`.
unsafe fn short_list_change(s: usize, person: usize, f: usize) -> u32 {
    let Some(k) = session_short_list(s) else {
        return 0;
    };
    session_take_undo(s);
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(f);
    f(session_search(s, SESSION_SEARCHES, k), person);
    session_mark_changed(s, k); // sx19b_8: the short list changed
    write_u32(s + SESSION_RESULTS_FRESH, 0);
    1
}

/// Comparer of a short list change: only the original runs; checked afterwards: the answer is whether there is a short
/// list, the change recorded and the player on (or off) the list.
unsafe fn compare_short_list_change(this: usize, person: usize, r: u32, on: u32, name: &str) {
    let ok = match session_short_list(this) {
        Some(k) => r == 1 && session_change_recorded(this, k) && is_player_on_short_list(this, person) == on,
        None => r == 0,
    };
    SCOREBOARD.record(ok, || format!("SearchEditSession::{} (person {}): answered {}", name, read_i32(person), r));
}

/// Comparer for `add_player_to_short_list`.
pub(crate) unsafe extern "thiscall" fn compare_add_player_to_short_list(this: usize, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I6%%)(this, person);
    compare_short_list_change(this, person, r, 1, "add_player_to_short_list");
    r
}

/// Takes an undo snapshot, removes the player from the short list slot and marks that slot as changed, or returns 0
/// when no short list slot exists.
///
/// `SearchEditSession::remove_player_from_short_list(person)` (thiscall, ret 4), from the scout manager (0x7f7ad8,
/// `remove_from_shortlist`): as `add_player_to_short_list`, with `PLAYER_SEARCH::remove_player`.
/// Original: `SearchEditSession::remove_player_from_short_list` (0x804670, search_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_player_from_short_list(this: usize, person: usize) -> u32 {
    short_list_change(this, person, FN_PLAYER_SEARCH_REMOVE)
}

/// Comparer for `remove_player_from_short_list`.
pub(crate) unsafe extern "thiscall" fn compare_remove_player_from_short_list(this: usize, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I7%%)(this, person);
    compare_short_list_change(this, person, r, 0, "remove_player_from_short_list");
    r
}

/// Takes an undo snapshot and clears the lock password of the current search when one is set, then marks the search
/// as changed.
///
/// `SearchEditSession::unlock_current_search()` (thiscall), from the search screen (0x80f60f, 0x80f65a): with a
/// password (the first byte of the current search's +0x139), the undo snapshot, the password emptied, the current search
/// and the session changed. Returns 1.
/// Original: `SearchEditSession::unlock_current_search` (0x804750, search_edit_session.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn unlock_current_search(this: usize) -> u32 {
    if read_u8(session_current_search(this) + 0x139) != 0 { // the search's password
        session_take_undo(this);
        write_u8(session_current_search(this) + 0x139, 0);
        session_mark_changed(this, read_i8(this) as isize as usize);
    }
    1
}

/// Comparer for `unlock_current_search`: only the original runs; checked afterwards: no password left.
pub(crate) unsafe extern "thiscall" fn compare_unlock_current_search(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I8%%)(this);
    let left = read_u8(session_current_search(this) + 0x139);
    SCOREBOARD.record(r == 1 && left == 0, || format!("SearchEditSession::unlock_current_search (search screen): password byte {:#x}", left));
    r
}
