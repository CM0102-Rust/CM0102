// ---------- cx05_20: the friendlies screens: a tournament by index, the human club's open dates request and its tournament invitations (friendly.cpp) ----------

/// The club of the current human manager (`G_SCREEN_RECORDS_MANAGER` + 0x300 × `G_SCREEN_MANAGER_CURRENT_SLOT`, its
/// `.club_job()`), or null.
unsafe fn screen_manager_club() -> usize {
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300))));
    if manager.is_null() {
        return 0;
    }
    manager.club_job().addr()
}

/// The tournament slot `index` (signed byte) of the friendlies (`friendly::TOURNAMENT_SLOTS`, 0x69 bytes each) when it
/// has a host (+0) and, if `date` is given, its start date equals it (Date::operator== `exe::date_operator_3d_3d` with
/// the date's two dwords); otherwise 0.
/// Original: `Friendly::get_tournament` (0x5c89a0, friendly.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn friendly_get_tournament(this: usize, index: u32, date: usize) -> usize {
    let slot = this + friendly::TOURNAMENT_SLOTS + (index as u8 as i8 as isize as usize).wrapping_mul(0x69);
    if date != 0 && exe::date_operator_3d_3d(slot + tournament_slot::START_DATE, read_u32(date), read_u32(date + 4)) == 0 {
        return 0;
    }
    if read_usize(slot + tournament_slot::HOST) == 0 { // cx05_20: a free slot is no tournament
        return 0;
    }
    slot
}

/// Comparer for `friendly_get_tournament`: both run (they only read) and return the same.
pub(crate) unsafe extern "thiscall" fn compare_friendly_get_tournament(this: usize, index: u32, date: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, index, date);
    let n = friendly_get_tournament(this, index, date);
    SCOREBOARD.record(r == n, || format!("friendly_get_tournament({}) original={:#x} rust={:#x}", index as u8 as i8, r, n));
    r
}

/// The friendly dates request (`friendly::SCHEDULED_SLOT_LIST`, 0x107-byte records) of the current human manager's
/// club that still waits for opponents (+0x103 not 0); otherwise 0. The list's cursor goes back to its head first.
/// Original: `Friendly::find_human_club_open_dates_request` (0x5c6920, friendly.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn find_human_club_open_dates_request(this: usize) -> usize {
    let list = Friendly(this).scheduled_slot_list();
    if list == 0 {
        return 0;
    }
    let club = screen_manager_club();
    if club == 0 {
        return 0;
    }
    let l = Llist(list);
    l.set_cursor(l.head());
    loop {
        let request = exe::llist_get_next(Friendly(this).scheduled_slot_list());
        if request == 0 {
            return 0;
        }
        if read_usize(request) == club && read_usize(request + 0x103) != 0 { // cx05_20: the request still open
            return request;
        }
    }
}

/// Comparer for `find_human_club_open_dates_request`: both run (they only read; the list cursor ends where it ends)
/// and return the same.
pub(crate) unsafe extern "thiscall" fn compare_find_human_club_open_dates_request(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%)(this);
    let n = find_human_club_open_dates_request(this);
    SCOREBOARD.record(r == n, || format!("find_human_club_open_dates_request original={:#x} rust={:#x}", r, n));
    r
}

/// `Friendly::send_tournament_invitations(tournament)` (thiscall, ret 4).
const FN_FRIENDLY_SEND_TOURNAMENT_INVITATIONS: usize = 0x5c5310;

/// Adds (`add` not 0) or removes `club` among the four invited clubs of the tournament the current human manager's
/// club hosts and has in use (the first such slot; none: the box at line 0xd95 and nothing more). Adding: a club
/// already invited gives the box at line 0xda2 and nothing more; otherwise it goes into the first empty place, marked
/// pending (`tournament_slot::INVITE_PENDING`), or nowhere when the four are taken. Removing: its place is emptied.
/// Then the invitations go out (`Friendly::send_tournament_invitations`).
/// Original: `Friendly::set_tournament_invite` (0x5c6990, friendly.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_tournament_invite(this: usize, club: usize, add: u32) {
    let host = screen_manager_club();
    let tournament = if host == 0 { 0 } else {
        (0..0x7fusize).map(|i| this + friendly::TOURNAMENT_SLOTS + i * 0x69)
            .find(|&t| read_usize(t + tournament_slot::HOST) != 0 && read_usize(t + tournament_slot::HOST) == host && read_usize(t + tournament_slot::IN_USE) != 0)
            .unwrap_or(0)
    };
    if tournament == 0 {
        return show_error_at(0x9cd0a0, 0xd95);
    }
    let invited = |k: usize| tournament + tournament_slot::INVITED_CLUBS + k * 4;
    if add != 0 {
        let mut free = -1i8;
        for k in 0..4usize {
            let c = read_usize(invited(k));
            if c == club {
                return show_error_at(0x9cd0a0, 0xda2);
            }
            if free == -1 && c == 0 {
                free = k as i8; // cx05_20: the first empty place
            }
        }
        if free != -1 {
            covered!("cx05_20 tournament invite added");
            write_usize(invited(free as usize), club);
            write_u8(tournament + tournament_slot::INVITE_PENDING + free as usize, 1);
        }
    } else if let Some(k) = (0..4usize).find(|&k| read_usize(invited(k)) == club) {
        write_usize(invited(k), 0);
    }
    let send: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FRIENDLY_SEND_TOURNAMENT_INVITATIONS);
    send(this, tournament);
}

/// Comparer for `set_tournament_invite`: the invitations go out as news (they cannot run twice); only the original.
pub(crate) unsafe extern "thiscall" fn compare_set_tournament_invite(this: usize, club: usize, add: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, club, add)
}

/// The size of a friendly dates request (`friendly::SCHEDULED_SLOT_LIST` items): the club, a dword, the count, ten
/// date slots of 0x19 bytes from +9 and the open flag at +0x103.
const FRIENDLY_DATES_REQUEST_SIZE: u32 = 0x107;

/// Builds a request of `count` open friendly dates for `club` and appends a copy to the dates requests list.
///
/// A request (`exe::crt_operator_new`(0x107); its ten date slots constructed by the C runtime's vector constructor
/// with FixedFixture::FixedFixture; out of memory: the box at line 0xed0 and 0): the club, `value` at +4, the count at
/// +8, +0x103 = 0; date slot k (k below `count`, signed) gets date k of `dates` (8 bytes each, Date::operator=), +8 and
/// +0x10 cleared, the club at +0xc and +0x14 and the byte +0x18 = 1. Without the list (`friendly::SCHEDULED_SLOT_LIST`)
/// a new one (`alloc_empty_list`; out of memory: the box at line 0xeeb, the request destroyed and 0). A copy of the
/// 0x107 bytes goes to the end of the list (`exe::llist_add_tail`; none: the box at line 0xef5, the request destroyed
/// and 0); the request is destroyed (the slots' empty destructor through the C runtime's vector destructor, then the
/// operator delete) and the list's copy is returned.
/// Original: `Friendly::add_friendly_dates_request` (0x5c71f0, friendly.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn add_friendly_dates_request(this: usize, club: usize, value: u32, count: u32, dates: usize) -> usize {
    let request = exe::crt_operator_new(FRIENDLY_DATES_REQUEST_SIZE);
    if request == 0 {
        show_error_at(0x9cd0a0, 0xed0);
        return 0;
    }
    exe::crt_vector_constructor_iterator(request + 9, 0x19, 0xa, FN_FIXEDFIXTURE_FIXEDFIXTURE, 0x5a53c0);
    let destroy_request = || {
        exe::crt_vector_destructor_iterator(request + 9, 0x19, 0xa, 0x5a53c0);
        exe::crt_operator_delete(request);
    };
    let n = count as u8 as i8;
    write_usize(request, club);
    write_u32(request + 4, value);
    write_u8(request + 8, n as u8);
    write_u32(request + 0x103, 0); // cx05_20: the request starts closed
    for k in 0..n.max(0) as usize {
        let slot = request + 9 + k * 0x19;
        let mut copy = [0u32; 2];
        exe::date_operator_3d(slot, copy.as_mut_ptr() as usize, dates + k * 8);
        write_u32(slot + 8, 0);
        write_usize(slot + 0xc, club);
        write_u32(slot + 0x10, 0);
        write_usize(slot + 0x14, club);
        write_u8(slot + 0x18, 1);
    }
    if Friendly(this).scheduled_slot_list() == 0 {
        let list = alloc_empty_list();
        write_usize(this + friendly::SCHEDULED_SLOT_LIST, list);
        if list == 0 {
            show_error_at(0x9cd0a0, 0xeeb);
            destroy_request();
            return 0;
        }
    }
    let node = exe::llist_add_tail(Friendly(this).scheduled_slot_list(), request, FRIENDLY_DATES_REQUEST_SIZE);
    if node == 0 {
        show_error_at(0x9cd0a0, 0xef5);
        destroy_request();
        return 0;
    }
    covered!("cx05_20 dates request added");
    destroy_request();
    node
}

/// Comparer for `add_friendly_dates_request`: it allocates and appends to the list (it cannot run twice); only the
/// original, then the plan: the copy in the list with the club, the count and the request closed.
pub(crate) unsafe extern "thiscall" fn compare_add_friendly_dates_request(this: usize, club: usize, value: u32, count: u32, dates: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) -> usize>(%%I3%%)(this, club, value, count, dates);
    if r != 0 {
        let ok = read_usize(r) == club && read_u32(r + 4) == value && read_u8(r + 8) == count as u8 && read_u32(r + 0x103) == 0;
        SCOREBOARD.record(ok, || format!("add_friendly_dates_request({:#x}): the request in the list does not match", club));
    }
    r
}

/// Cancels a friendly dates request: without the request or the dates requests list, the box at line 0x112c and
/// nothing more. For each of its dates (+8 of them, signed): the club's fixtures on that day
/// (`exe::fixture_manager_find_fixtures` into a local list: the day, the requesting club, any competition): each
/// unplayed friendly (`fixture::HOME_GOALS` 0xff, competition [`KEY_CLUB_COMP_FRIENDLY`]) not on today's date leaves
/// the calendar (`exe::fixture_manager_remove_fixture`); the club's friendly on that date
/// (`exe::friendly_find_friendly_match`) is removed (`exe::friendly_remove_friendly_match`(match, 0)). Then the
/// request leaves the list (`exe::llist_remove`).
/// Original: `Friendly::cancel_friendly_dates_request` (0x5c85d0, friendly.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cancel_friendly_dates_request(this: usize, request: usize) {
    if request == 0 || Friendly(this).scheduled_slot_list() == 0 {
        return show_error_at(0x9cd0a0, 0x112c);
    }
    let mut k = 0i8;
    while k < read_i8(request + 8) {
        let slot = request + 9 + k as usize * 0x19;
        let date = [read_u32(slot), read_u32(slot + 4)];
        let d = date.as_ptr() as usize;
        let mut list = [0u8; 0x10];
        let l = list.as_mut_ptr() as usize;
        exe::new_llist(l);
        let club_id = read_u32(read_usize(request));
        // the original pushes the dword at +2 of the date (the year and the low word of the leap flag): the callee reads the word
        exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), date[0], read_u32(d + 2), date[0], read_u32(d + 2), club_id, l, u32::MAX, (-2i32) as u32, u32::MAX);
        let ll = Llist(l);
        ll.set_cursor(ll.head());
        loop {
            let j = exe::llist_get_next(l);
            if j == 0 {
                break;
            }
            let today = read_u16(j + fixture::DAY) as i32 == read_i16(G_CURRENT_DATE) as i32 && read_u16(j + fixture::YEAR) == read_u16(G_CURRENT_DATE_YEAR);
            if !today && read_u8(j + fixture::HOME_GOALS) == 0xff && read_u32(j) == read_u32(KEY_CLUB_COMP_FRIENDLY) { // cx05_20: the unplayed friendlies
                exe::fixture_manager_remove_fixture(read_usize(G_FIXTURE_MANAGER), j);
            }
        }
        let friendly_match = exe::friendly_find_friendly_match(this, d, read_usize(request));
        if friendly_match != 0 {
            exe::friendly_remove_friendly_match(this, friendly_match, 0);
        }
        exe::drop_llist(l);
        k = k.wrapping_add(1);
    }
    covered!("cx05_20 dates request cancelled");
    exe::llist_remove(Friendly(this).scheduled_slot_list(), request);
}

/// Comparer for `cancel_friendly_dates_request`: the fixtures and the friendlies leave the calendar (it cannot run
/// twice); only the original, then the plan: the request is no longer in the list.
pub(crate) unsafe extern "thiscall" fn compare_cancel_friendly_dates_request(this: usize, request: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I4%%)(this, request);
    let list = Friendly(this).scheduled_slot_list();
    if list != 0 && request != 0 {
        let mut node = read_usize(list + 2);
        let mut found = false;
        while node != 0 {
            found |= read_usize(node) == request;
            node = read_usize(node + 6);
        }
        SCOREBOARD.record(!found, || format!("cancel_friendly_dates_request({:#x}): the request is still in the list", request));
    }
}
