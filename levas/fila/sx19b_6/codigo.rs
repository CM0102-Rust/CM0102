// ---------- sx19b_6: the scout manager for the screens: shortlist, compare players, scope, duty competitions (scout_manager.cpp) ----------

/// `SearchEditSession::add_player_to_short_list` (0x804590) and `remove_player_from_short_list` (0x804670):
/// thiscall(session, person), ret 4.
const FN_SESSION_ADD_TO_SHORT_LIST: usize = 0x804590;
const FN_SESSION_REMOVE_FROM_SHORT_LIST: usize = 0x804670;
/// `PLAYER_SEARCH::add_player` (0x7af720), `remove_player` (0x7b1260) and `set_assigned_club` (0x7b3d50):
/// thiscall(search, argument), ret 4.
const FN_SEARCH_ADD_PLAYER: usize = 0x7af720;
const FN_SEARCH_REMOVE_PLAYER: usize = 0x7b1260;
const FN_SEARCH_SET_ASSIGNED_CLUB: usize = 0x7b3d50;

/// A thiscall(object, argument) ret 4 exe routine by its address.
unsafe fn scout_call_1(f: usize, this: usize, a: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(f);
    f(this, a)
}

/// The slot (0..15) of a human manager record (its id byte − `G_STAFF_COUNT` byte + 16, signed), when in range.
unsafe fn human_slot(manager: usize) -> Option<usize> {
    let i = read_u8(manager).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if (0..=0xf).contains(&i) { Some(i as usize) } else { None }
}

/// Puts the player on (or takes him off) the human manager's shortlist: through the open edit session, or on the first
/// of his 10 searches of type 1. Shared by `add_to_shortlist` and `remove_from_shortlist`.
unsafe fn shortlist_change(this: usize, person: usize, manager: usize, session_fn: usize, search_fn: usize) -> Option<u32> {
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    if person == 0 || m == 0 {
        return None;
    }
    let slot = human_slot(m)?;
    let session = read_usize(this + scout_manager_fields::EDIT_SESSIONS + slot * 4);
    if session != 0 {
        return Some(scout_call_1(session_fn, session, person));
    }
    // simplification: the original passes the index in a stack dword with only the low byte written; scout_record
    // reads that byte
    for k in 0..10i8 {
        let s = scout_record(this, k, m as *const i32);
        if s != 0 && read_u8(s + player_search_fields::LIST_TYPE) == 1 {
            covered!("sx19b_6 short list search");
            scout_call_1(search_fn, s, person);
            return Some(1);
        }
    }
    Some(0)
}

/// Adds a player to a human manager's shortlist, through the open edit session when one exists.
///
/// `SCOUT_MANAGER::add_to_shortlist(person, manager)` (thiscall, ret 8), from the player screens (0x8898e5,
/// 0x88b0a8, 0x8be9ad, 0x8f49a9...). Without a manager, the human whose turn it is (`scout_manager_current_human_manager`).
/// With his edit session open (`scout_manager_fields::EDIT_SESSIONS` + 4 × slot), its answer
/// (`SearchEditSession::add_player_to_short_list`); otherwise the first of his 10 searches (`scout_record`) of type 1
/// (`player_search_fields::LIST_TYPE`, the short list) takes the player (`PLAYER_SEARCH::add_player`) and 1 is
/// returned; none: 0. No person (line 0x273), no manager (0x27d) or a slot outside 0..15 (0x287): the original
/// ("Error" box, 0).
/// Original: `SCOUT_MANAGER::add_to_shortlist` (0x7f75f0, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_to_shortlist(this: usize, person: usize, manager: usize) -> u32 {
    match shortlist_change(this, person, manager, FN_SESSION_ADD_TO_SHORT_LIST, FN_SEARCH_ADD_PLAYER) {
        Some(r) => r,
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, person, manager), // the box
    }
}

/// Comparer for `add_to_shortlist`: the player can be added once, so only the original runs; checked afterwards: a
/// player it answered 1 for is on the shortlist (`is_player_shortlisted`).
pub(crate) unsafe extern "thiscall" fn compare_add_to_shortlist(this: usize, person: usize, manager: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, person, manager);
    if r == 1 {
        let on = is_player_shortlisted(this, person, manager);
        SCOREBOARD.record(on == 1, || format!("SCOUT_MANAGER::add_to_shortlist (player screen, person {}): answered 1, on the shortlist {}", read_i32(person), on));
    }
    r
}

/// Removes a player from a human manager's shortlist, through the open edit session when one exists.
///
/// `SCOUT_MANAGER::remove_from_shortlist(person, manager)` (thiscall, ret 8), from the player screens (0x8c9d11,
/// 0x8f4c24). As `add_to_shortlist`, with `SearchEditSession::remove_player_from_short_list` and
/// `PLAYER_SEARCH::remove_player`; the boxes at lines 0x31e, 0x328 and 0x332.
/// Original: `SCOUT_MANAGER::remove_from_shortlist` (0x7f7a20, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn remove_from_shortlist(this: usize, person: usize, manager: usize) -> u32 {
    match shortlist_change(this, person, manager, FN_SESSION_REMOVE_FROM_SHORT_LIST, FN_SEARCH_REMOVE_PLAYER) {
        Some(r) => r,
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I3%%)(this, person, manager), // the box
    }
}

/// Comparer for `remove_from_shortlist`: only the original runs; checked afterwards: a player it answered 1 for is
/// off the shortlist.
pub(crate) unsafe extern "thiscall" fn compare_remove_from_shortlist(this: usize, person: usize, manager: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I3%%)(this, person, manager);
    if r == 1 {
        let on = is_player_shortlisted(this, person, manager);
        SCOREBOARD.record(on == 0, || format!("SCOUT_MANAGER::remove_from_shortlist (player screen, person {}): answered 1, on the shortlist {}", read_i32(person), on));
    }
    r
}

/// The compare players 1 and 2 of human manager slot i after storing `person` as player `which` (1 or 2): when he
/// already holds the other place the two swap; otherwise he takes place `which` (any value but 1 means place 2).
fn compare_players_after(p1: usize, p2: usize, person: usize, which: i8) -> (usize, usize) {
    if (person == p1 && which == 2) || (person == p2 && which == 1) {
        (p2, p1)
    } else if which == 1 {
        (person, p2)
    } else {
        (p1, person)
    }
}

/// Stores a player as compare player 1 or 2 of a human manager, swapping the two when he already holds the other place.
///
/// `SCOUT_MANAGER::set_compare_player(person, manager, which)` (thiscall, ret 0xc), from the player comparison screen
/// (0x8f45c3). Without a manager, the human whose turn it is. Compare player 1 at +0x68 and 2 at +0xa8 (+ 4 × slot):
/// see `compare_players_after`. Returns 1. No person (line 0x2b7), no manager (0x2c1) or a slot outside 0..15
/// (0x2cb): the original ("Error" box, 0).
/// Original: `SCOUT_MANAGER::set_compare_player` (0x7f7780, scout_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn set_compare_player(this: usize, person: usize, manager: usize, which: u32) -> u32 {
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    let slot = if person != 0 && m != 0 { human_slot(m) } else { None };
    let Some(i) = slot else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I1%%)(this, person, manager, which); // the box
    };
    let (a, b) = (this + 0x68 + i * 4, this + 0xa8 + i * 4); // compare players 1 and 2 of the slot
    let (p1, p2) = compare_players_after(read_usize(a), read_usize(b), person, which as i8);
    write_usize(a, p1); // sx19b_6: compare player 1
    write_usize(b, p2);
    1
}

/// Comparer for `set_compare_player`: the expected pair is planned first, then only the original runs and both places
/// are checked.
pub(crate) unsafe extern "thiscall" fn compare_set_compare_player(this: usize, person: usize, manager: usize, which: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I1%%);
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    let Some(i) = (if person != 0 && m != 0 { human_slot(m) } else { None }) else {
        return o(this, person, manager, which);
    };
    let (a, b) = (this + 0x68 + i * 4, this + 0xa8 + i * 4);
    let plan = compare_players_after(read_usize(a), read_usize(b), person, which as i8);
    let r = o(this, person, manager, which);
    let got = (read_usize(a), read_usize(b));
    SCOREBOARD.record(r == 1 && got == plan, || format!("SCOUT_MANAGER::set_compare_player (player comparison screen, place {}): original {:#x?} expected {:#x?}", which as i8, got, plan));
    r
}

/// Returns compare player 1 or 2 of the current human manager.
///
/// `SCOUT_MANAGER::get_compare_player(which)` (thiscall, ret 4), from the player comparison screens (0x87065e,
/// 0x8765af, 0x879071, 0x888f20, 0x88c520...): the human whose turn it is; `which` 1: +0x68, otherwise +0xa8 (+ 4 ×
/// slot). No manager (line 0x2fa) or a slot outside 0..15 (0x304): the original ("Error" box, 0).
/// Original: `SCOUT_MANAGER::get_compare_player` (0x7f7920, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_compare_player(this: usize, which: u32) -> usize {
    let m = scout_manager_current_human_manager();
    let Some(i) = (if m != 0 { human_slot(m) } else { None }) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, which); // the box
    };
    read_usize(this + if which as i8 == 1 { 0x68 } else { 0xa8 } + i * 4)
}

/// Comparer for `get_compare_player`: side by side.
pub(crate) unsafe extern "thiscall" fn compare_get_compare_player(this: usize, which: u32) -> usize {
    let ro = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, which);
    let m = scout_manager_current_human_manager();
    if m != 0 && human_slot(m).is_some() {
        let rr = get_compare_player(this, which);
        SCOREBOARD.record(ro == rr, || format!("SCOUT_MANAGER::get_compare_player (player comparison screen, place {}): original {:#x} rust {:#x}", which as i8, ro, rr));
    }
    ro
}

/// Scopes the current search of the open edit session to a club when the search has no club yet.
///
/// `SCOUT_MANAGER::scope_current_search_to_club(&club)` (thiscall, ret 4), from the club screen's scout command
/// (0x482d72, the answer unused). The human whose turn it is (none: nothing); his edit session (none: nothing); its
/// current search (the signed byte at +0; the 0x2b0-byte searches from +1) with no club (+0x190 =
/// `player_search_fields::ASSIGNED_CLUB` = −2) takes the club (`PLAYER_SEARCH::set_assigned_club`). No club pointer
/// (line 0x415) or a slot outside 0..15 (0x427): the original ("Error" box).
/// Original: `SCOUT_MANAGER::scope_current_search_to_club` (0x7f82f0, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn scope_current_search_to_club(this: usize, club: *const u32) {
    let m = scout_manager_current_human_manager();
    if !club.is_null() && m == 0 {
        return;
    }
    let slot = if club.is_null() { None } else { human_slot(m) };
    let Some(i) = slot else {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(%%I4%%)(this, club); // the box
    };
    let session = read_usize(this + scout_manager_fields::EDIT_SESSIONS + i * 4);
    if session == 0 {
        return;
    }
    let search = search_record(session + 1, read_i8(session) as i32);
    if read_i32(search + player_search_fields::ASSIGNED_CLUB) == -2 {
        covered!("sx19b_6 search scoped to club");
        scout_call_1(FN_SEARCH_SET_ASSIGNED_CLUB, search, *club as usize);
    }
}

/// Comparer for `scope_current_search_to_club`: it changes the search once, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_scope_current_search_to_club(this: usize, club: *const u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(%%I4%%)(this, club);
}

/// The 8 bytes of a date at `a`.
unsafe fn date_bytes(a: usize) -> [u8; 8] {
    core::array::from_fn(|k| read_u8(a + k))
}

/// Whether date `d` (day i16, year u16) is after today (`G_CURRENT_DATE`); with `before`, whether it is before today.
unsafe fn date_vs_today(d: [u8; 8], before: bool) -> bool {
    let (today_day, today_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
    let (day, year) = (i16::from_le_bytes([d[0], d[1]]), u16::from_le_bytes([d[2], d[3]]));
    if before {
        year < today_year || (year == today_year && day < today_day)
    } else {
        year > today_year || (year == today_year && day > today_day)
    }
}

/// Whether a national team duty competition is near today: its first match date less 7 days is not after today and
/// its last match date less 7 days is not before today (`exe::date_sub_days`, compared with `G_CURRENT_DATE`). The
/// match dates come from `Comp::build_match_date_list` (20-byte entries, the date at +0xc), freed afterwards.
unsafe fn duty_competition_is_current(comp: usize) -> bool {
    let mut count = 0i32;
    let dates = exe::comp_build_match_date_list(comp, &mut count);
    let mut tmp = [0u8; 8];
    let first = date_bytes(exe::new_date(tmp.as_mut_ptr() as usize, read_u16(dates + 0xc) as u32, read_u16(dates + 0xe) as u32));
    let last_entry = dates.wrapping_add((count as usize).wrapping_mul(20));
    let last = date_bytes(exe::new_date(tmp.as_mut_ptr() as usize, read_u16(last_entry - 8) as u32, read_u16(last_entry - 6) as u32));
    exe::crt_free(dates);
    if date_vs_today(date_bytes(exe::date_sub_days(first.as_ptr() as usize, tmp.as_mut_ptr() as usize, 7)), false) {
        return false;
    }
    !date_vs_today(date_bytes(exe::date_sub_days(last.as_ptr() as usize, tmp.as_mut_ptr() as usize, 7)), true) // sx19b_6: still current
}

/// Fills a list with national team duty competitions and drops each one whose match dates do not fall near today.
///
/// `SCOUT_MANAGER::collect_current_duty_comps(list)` (thiscall, ret 4; this is not read), from the search screen
/// (0x80925b): the list emptied (`exe::llist_clear`) and filled by the national teams
/// (`NationalTeams::collect_duty_competitions` of `G_NATIONAL_TEAMS`); walking it from the head, a node (a competition
/// id) is removed (`exe::llist_remove`) when the id is negative, the competition object (`G_COMP_OBJECT_TABLE`) is
/// missing or `duty_competition_is_current` says no. Returns 1. No list: the original ("Error" box, line 0x176, 0).
/// Original: `SCOUT_MANAGER::collect_current_duty_comps` (0x7f7010, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn collect_current_duty_comps(this: usize, list: usize) -> u32 {
    if list == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I5%%)(this, list); // the box
    }
    exe::llist_clear(list);
    collect_duty_competitions(read_usize(G_NATIONAL_TEAMS), list);
    let l = Llist(list);
    l.set_cursor(l.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        let id = read_i32(node);
        let comp = if id > -1 { read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4) } else { 0 };
        if comp == 0 || !duty_competition_is_current(comp) {
            exe::llist_remove(list, node);
        }
        node = exe::llist_get_next(list);
    }
    1
}

/// The competition ids of a list, in order (walked with the list's cursor from the head).
unsafe fn llist_ids(list: usize) -> Vec<i32> {
    let l = Llist(list);
    let mut v = Vec::new();
    l.set_cursor(l.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        v.push(read_i32(node));
        node = exe::llist_get_next(list);
    }
    v
}

/// Comparer for `collect_current_duty_comps`: the original fills the screen's list and the copy a local one (its nodes
/// freed afterwards); the competition ids in order.
pub(crate) unsafe extern "thiscall" fn compare_collect_current_duty_comps(this: usize, list: usize) -> u32 {
    let ro = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I5%%)(this, list);
    if list != 0 {
        let mut local = [0u8; 14];
        let lp = local.as_mut_ptr() as usize;
        exe::new_llist(lp);
        let rr = collect_current_duty_comps(this, lp);
        let (io, ir) = (llist_ids(list), llist_ids(lp));
        exe::drop_llist(lp);
        SCOREBOARD.record(ro == rr && io == ir, || format!("SCOUT_MANAGER::collect_current_duty_comps (search screen): original {:?} rust {:?}", io, ir));
    }
    ro
}
