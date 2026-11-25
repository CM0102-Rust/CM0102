use super::*;

/// Clears the person's dispute flag and removes her entry from the dispute list.
///
/// `DISPUTE::remove_dispute(person)`: without a person or without the flag table (+4), or with the person's flag at
/// zero: nothing. Otherwise it zeroes the flag and, with the list (+0), moves the cursor back to the start (+0xa = +2)
/// and searches, among the first `count` (+0, word), the entry with the person's id (`next_list_item`): if found, it removes it
/// (`manager_manager_remove`).
/// Original: `DISPUTE::remove_dispute` (0x56a090, dispute.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn remove_dispute(this: usize, p: usize) {
    if p == 0 || read_usize(this + 4) == 0 {
        return;
    }
    let m = (read_usize(this + 4) as isize + read_i32(p) as isize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m = 0;
    let list = read_usize(this);
    if list == 0 {
        return;
    }
    write_u32(list + llist::CURSOR, read_usize(list + 2) as u32);
    let mut k = 0i32;
    while (k as i16 as i32) < read_i16(read_usize(this)) as i32 {
        let in_the = exe::llist_get_next(read_usize(this));
        if in_the == 0 {
            return;
        }
        if read_i32(in_the) == read_i32(p) {
            covered!("cqg_7 left the contest");
            exe::llist_remove(read_usize(this), in_the);
            return;
        }
        k += 1;
    }
}

/// Reads the dispute object from the save's "dispute.dat": nothing (0) when the game data version (`G_GAME_DATA_VERSION`,
/// in absolute value) is below 0x3e046, without a save or without the item. The flag table length (dword; 0 closes
/// the item and answers 0), the table (`malloc` into +4; failing, the item closed and 0) and its bytes; the dispute
/// count (dword), a new list (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) in +0 and that
/// many 17-byte disputes (`LLIST::add_tail`). The item closed, 1. A short read closes the item and answers 0.
/// Original: `DISPUTE::load` (0x569910, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn dispute_load(this: usize, save: usize) -> u8 {
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < DISPUTE_DAT_VERSION || save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, DISPUTE_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx09c_3 load dispute.dat");
    let read = |z: usize| -> bool {
        let mut length = 0u32;
        if exe::zread(&mut length as *mut u32 as usize, 4, 1, z) != 1 || length == 0 {
            return false;
        }
        let flags = exe::crt_malloc(length);
        write_usize(this + 4, flags); // sx09c_3: the per-person dispute flags
        if flags == 0 || exe::zread(flags, 1, length, z) != length {
            return false;
        }
        let mut count = 0i32;
        if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
            return false;
        }
        let list = club_history_new_list();
        write_usize(this, list);
        let mut record = [0u8; 0x11];
        for _ in 0..count.max(0) {
            if exe::zread(record.as_mut_ptr() as usize, 0x11, 1, z) != 1 {
                return false;
            }
            exe::llist_add_tail(list, record.as_ptr() as usize, 0x11);
        }
        true
    };
    let ok = read(z);
    exe::zclose(&mut z);
    ok as u8
}

/// Comparer for `dispute_load`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_dispute_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_DISPUTE_LOAD)(this, save)
}

/// Writes the dispute object to the save's "dispute.dat": the staff count (`G_STAFF_COUNT`, the global itself), the flag
/// table (that many bytes), the dispute count (the list's i16 as a dword) and the 17-byte disputes (from the list's
/// head); the item closed, 1. Without a save (line 0xe9) or the item (0xf1): the dispute.cpp "Error" box and 0. A short
/// write or a missing node: the box (lines 0xf8, 0xff, 0x108, 0x111, 0x117), the item closed and 0.
/// Original: `DISPUTE::save` (0x569ae0, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn dispute_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_box(DISPUTE_CPP_PATH, 0xe9);
        return 0;
    }
    let mut z = exe::zopen(save, DISPUTE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(DISPUTE_CPP_PATH, 0xf1);
        return 0;
    }
    covered!("sx09c_3 save dispute.dat");
    let write = |z: usize| -> Result<(), u32> {
        if exe::zwrite(G_STAFF_COUNT, 4, 1, z) != 1 { // sx09c_3: the people count
            return Err(0xf8);
        }
        if exe::zwrite(read_usize(this + 4), 1, read_u32(G_STAFF_COUNT), z) != read_u32(G_STAFF_COUNT) { // sx09c_3: the flags
            return Err(0xff);
        }
        let list = Llist(read_usize(this));
        let n = list.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x108);
        }
        list.set_cursor(list.head());
        let mut k = 0i32;
        while k < list.count() as i32 {
            let record = exe::llist_get_next(list);
            if record == 0 {
                return Err(0x111);
            }
            if exe::zwrite(record, 0x11, 1, z) != 1 {
                return Err(0x117);
            }
            k += 1;
        }
        Ok(())
    };
    let r = write(z);
    if let Err(row) = r {
        show_error_box(DISPUTE_CPP_PATH, row);
    }
    exe::zclose(&mut z);
    r.is_ok() as u8
}

/// Comparer for `dispute_save`: it writes into the save container, so it cannot run twice; only the original runs.
/// The copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_dispute_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_DISPUTE_SAVE)(this, save)
}

/// The save item "dispute.dat".
pub(crate) const DISPUTE_DAT: usize = 0x9c384c;

/// The source path of dispute.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\dispute.cpp").
pub(crate) const DISPUTE_CPP_PATH: usize = 0x9c3858;

/// The game data version from which saves carry dispute.dat (older ones have none).
pub(crate) const DISPUTE_DAT_VERSION: i32 = 0x3e046;

/// Returns the person's dispute flag, dropping the dispute when it no longer stands.
///
/// `is_in_dispute(person, flag_only)`: no person, or an id at or above `G_STAFF_COUNT`: 0. A flag set while the FIFA
/// transfer rules are off (`G_FIFA_TRANSFER_RULES_IN_FORCE` 0): the dispute is dropped (`remove_dispute`) and 0. With
/// `flag_only`, the flag. Otherwise, with the flag set, the person's contract (`exe::contract_manager_get_contract`,
/// kept in `G_CONTRACT_LOOKUP_RESULT`): none, or its club's record address null, drops the dispute and gives 0; else
/// the flag.
/// Original: `DISPUTE::is_in_dispute` (0x569e30, dispute.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn is_in_dispute(this: usize, person: usize, flag_only: u32) -> u8 {
    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let flag = read_u8(dispute_flag(this, person));
    if flag != 0 && read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 0 {
        covered!("sx09b_5 dispute dropped without the FIFA rules");
        remove_dispute(this, person);
        return 0;
    }
    if flag_only != 0 || flag == 0 {
        return flag;
    }
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if c.is_null() || read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE)) == 0 {
        covered!("sx09b_5 dispute dropped without a contract");
        remove_dispute(this, person);
        return 0;
    }
    read_u8(dispute_flag(this, person))
}

/// Returns the club id of the person's dispute entry, or −1.
///
/// `get_dispute_club_id(person)`: no person, no flag table, the person's flag clear or no dispute list: −1.
/// Otherwise, from the start of the list (the cursor at the head), the first of the first `count` 17-byte entries with
/// the person's id (+0) gives its club id (+4); none: −1.
/// Original: `DISPUTE::get_dispute_club_id` (0x56a030, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_dispute_club_id(this: usize, person: usize) -> i32 {
    if person == 0 || read_usize(this + 4) == 0 || read_u8(dispute_flag(this, person)) == 0 || read_usize(this) == 0 {
        return -1;
    }
    let list = Llist(read_usize(this));
    list.set_cursor(list.head());
    let mut k = 0i32;
    while k < list.count() as i32 {
        let e = exe::llist_get_next(list);
        if e == 0 {
            return -1;
        }
        if read_i32(e) == read_i32(person) {
            covered!("sx09b_5 dispute club found");
            return read_i32(e + 4); // +4: the club id of the dispute
        }
        k += 1;
    }
    -1
}

/// Resolves every dispute whose hearing date has passed.
///
/// `process_hearings()` (daily): with the dispute list, from its head, for each of the first `count` 17-byte entries
/// (re-read after each step, so a removal skips the next one, as in the original) whose hearing date (+8) is before
/// today (`exe::date_operator_3c` with `G_CURRENT_DATE`) and whose person id (+0) is between 0 and `G_STAFF_COUNT` − 2:
/// when the person's `.club_job()` is the dispute's club (+4): the byte +0x10 set keeps him (the news with outcome 1);
/// otherwise he leaves on a free transfer (`exe::transfer_manager_queue_free_transfer` with 0 and −1), the club's
/// manager stops favouring him (`exe::staff_is_favoured_by` → `exe::staff_remove_favoured_staff`), the club stops
/// favouring him (`exe::club_favours_staff` → `FN_CLUB_REMOVE_FAVOURED_STAFF`), the manager and the club dislike him
/// (`exe::staff_add_disliked_staff`, `exe::club_add_disliked_staff`, both with 1) and the news with outcome 2
/// (`FN_DISPUTE_POST_NEWS`). Then (whatever the club) the person's flag goes to 0 and the entry leaves the list
/// (`exe::llist_remove`).
/// Original: `DISPUTE::process_hearings` (0x569ed0, dispute.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn process_hearings(this: usize) {
    if read_usize(this) == 0 {
        return;
    }
    let list = Llist(read_usize(this));
    list.set_cursor(list.head());
    if list.count() <= 0 {
        return;
    }
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_DISPUTE_POST_NEWS);
    let mut k = 0i32;
    loop {
        let e = exe::llist_get_next(list);
        if e == 0 {
            return;
        }
        let id = read_i32(e);
        if exe::date_operator_3c(e + 8, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0 && id > -1 && id < read_i32(G_STAFF_COUNT) - 1 {
            let person = staff_at(id as isize);
            let club = person.club_job();
            if !club.is_null() && read_i32(club.addr()) == read_i32(e + 4) {
                if read_u8(e + 0x10) != 0 { // +0x10: the player stays at his club
                    covered!("sx09b_5 hearing keeps the player");
                    post(this, person.addr(), club.addr(), 1, 0, 0);
                } else {
                    covered!("sx09b_5 hearing frees the player");
                    exe::transfer_manager_queue_free_transfer(read_usize(G_TRANSFER_MANAGER), person, 0, u32::MAX);
                    let manager = club.manager();
                    if !manager.is_null() {
                        if exe::staff_is_favoured_by(person, manager) != 0 {
                            exe::staff_remove_favoured_staff(person, manager);
                        }
                        if exe::club_favours_staff(person, club) != 0 {
                            let unfavour: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_CLUB_REMOVE_FAVOURED_STAFF);
                            unfavour(club.addr(), person.addr());
                        }
                        exe::staff_add_disliked_staff(person, manager, 1);
                    }
                    exe::club_add_disliked_staff(club, person, 1);
                    post(this, person.addr(), club.addr(), 2, 0, 0); // sx09b_5: the outcome of a freed player
                }
            }
            write_u8(read_usize(this + 4).wrapping_add(read_usize(e)), 0);
            exe::llist_remove(list, e);
        }
        k += 1;
        if k >= read_i16(read_usize(this)) as i32 {
            return;
        }
    }
}

/// Comparer for `is_in_dispute`: side by side over the contract lookup result and the flag returned; a dispute that
/// would be dropped (the list changes) runs the original only, and the flag must end at 0 with 0 returned.
pub(crate) unsafe extern "thiscall" fn compare_is_in_dispute(this: usize, person: usize, flag_only: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_IS_IN_DISPUTE);
    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) || read_u8(dispute_flag(this, person)) == 0 {
        let r = o(this, person, flag_only);
        SCOREBOARD.record(r == 0, || format!("is_in_dispute(person {:#x}) without a dispute returned {}", person, r));
        return r;
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 0 || flag_only == 0 {
        let flag = read_u8(dispute_flag(this, person));
        let r = o(this, person, flag_only);
        let after = read_u8(dispute_flag(this, person));
        SCOREBOARD.record((after == 0 && r == 0) || (r == flag && after == flag), || format!("is_in_dispute(person {:#x}, flag {}) returned {}, flag after {}", person, flag, r, after));
        return r;
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, person, flag_only), || is_in_dispute(this, person, flag_only));
    SCOREBOARD.record(same && r == n, || format!("is_in_dispute(person {:#x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `get_dispute_club_id`: side by side over the list cursor and the result.
pub(crate) unsafe extern "thiscall" fn compare_get_dispute_club_id(this: usize, person: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_GET_DISPUTE_CLUB_ID);
    let list = read_usize(this);
    let regions: &[(usize, usize)] = if list == 0 { &[] } else { &[(list + llist::CURSOR, 4)] };
    let (r, n, same, _, _) = crate::comparison::side_by_side_light(regions, || o(this, person), || get_dispute_club_id(this, person));
    SCOREBOARD.record(same && r == n, || format!("get_dispute_club_id(person {:#x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `process_hearings`: frees players and posts news, so only the original runs; the plan is checked
/// afterwards: the entries the walk resolves (due, with a valid person id; the walk stops early after removals, as in the
/// original) have the flag cleared and left the list.
pub(crate) unsafe extern "thiscall" fn compare_process_hearings(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_HEARINGS);
    let list = read_usize(this);
    if list == 0 || read_i16(list) <= 0 {
        return o(this);
    }
    // the copy's walk over the nodes, without changing anything: the ids of the entries it resolves
    let mut nodes = Vec::new();
    let mut node = read_usize(list + llist::HEAD);
    while node != 0 {
        nodes.push(read_usize(node + llist_node::DATA));
        node = read_usize(node + llist_node::NEXT);
    }
    let due = |e: usize| {
        let id = read_i32(e);
        exe::date_operator_3c(e + 8, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0 && id > -1 && id < read_i32(G_STAFF_COUNT) - 1
    };
    let (mut count, mut resolved) = (nodes.len() as i32, Vec::new());
    for (k, &e) in nodes.iter().enumerate() {
        if due(e) {
            resolved.push(read_usize(e));
            count -= 1;
        }
        if k as i32 + 1 >= count {
            break;
        }
    }
    o(this);
    let flags = read_usize(this + 4);
    let wrong: Vec<usize> = resolved.iter().copied().filter(|&id| read_u8(flags.wrapping_add(id)) != 0).collect();
    let left = if read_usize(this) == 0 { 0 } else { read_i16(read_usize(this)) as i32 };
    SCOREBOARD.record(wrong.is_empty() && left == count, || format!("process_hearings: resolved {:?}, flags still set {:?}, entries left {} (expected {})", resolved, wrong, left, count));
}

/// Posts news item 3049 (0xbe9) about a player's contract dispute with a club.
///
/// `dispute_post_news(person, club, outcome, a, b)` (the dispute object in ecx is not read; SEH frame for the local
/// news item, no destructor). Without a person, his `.club_job()`, his `.player()` or `club`: nothing. Slots 0-3 the
/// person and his name ids (`set_news_person_slots`), 4 the club's id, 5 his `.club_job()`'s id, 6 `a` and 7 `b`
/// (signed bytes), 8 the id of his club's nation (−1 without one), 9 `outcome` (signed byte: 1 kept at the club, 2
/// freed, from `process_hearings`). A player whose `.world_reputation()` is above 7500 is news to every human manager;
/// otherwise to the managers of his club's nation (when it has one) and, when his own `.nation()` is another, also to
/// those of his nation.
/// Original: `dispute_post_news` (0x56a3b0, dispute.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn dispute_post_news(_this: usize, person: usize, club: usize, outcome: u32, a: u32, b: u32) {
    let person = Staff(person);
    if person.is_null() || person.club_job().is_null() || person.player().is_null() || club == 0 {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbe9, 0);
    set_news_person_slots(n, 0, person);
    set_news_slot(n, 4, read_u32(club));
    let own = person.club_job();
    set_news_slot(n, 5, if own.is_null() { u32::MAX } else { read_u32(own.addr()) });
    set_news_slot(n, 6, a as i8 as i32 as u32);
    set_news_slot(n, 7, b as i8 as i32 as u32);
    let club_nation = if own.is_null() { 0 } else { own.nation().addr() };
    set_news_slot(n, 8, if club_nation == 0 { u32::MAX } else { read_u32(club_nation) });
    set_news_slot(n, 9, outcome as i8 as i32 as u32); // sx09b_6: the outcome
    if person.player().world_reputation() > 0x1d4c {
        covered!("sx09b_6 dispute news to every manager");
        dispute_news_send(n, None);
        return;
    }
    if club_nation != 0 {
        covered!("sx09b_6 dispute news to the club's nation");
        dispute_news_send(n, Some(club_nation));
    }
    if person.club_job().nation() != person.nation() {
        covered!("sx09b_6 dispute news to the player's nation");
        dispute_news_send(n, Some(person.nation().addr()));
    }
}

/// Comparer for `dispute_post_news`, as a dry run: the copy builds the news item without sending it, then the original
/// runs. Every dispute news item (type 0xbe9) that reached a human inbox must be the copy's, and with no send planned
/// none may arrive.
pub(crate) unsafe extern "thiscall" fn compare_dispute_post_news(this: usize, person: usize, club: usize, outcome: u32, a: u32, b: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_DISPUTE_POST_NEWS);
    let inboxes = human_inbox_marks();
    *DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner()) = (Vec::new(), 0);
    DISPUTE_NEWS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| dispute_post_news(this, person, club, outcome, a, b)));
    DISPUTE_NEWS_DRY_RUN.store(false, Relaxed);
    let (mut built, sends) = core::mem::take(&mut *DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner()));
    o(this, person, club, outcome, a, b);
    if !built.is_empty() {
        built[news_item::IMPORTANT] = 0;
    }
    let delivered = delivered_news_items(&inboxes, 0xbe9);
    let same = inboxes.is_empty() || if sends == 0 { delivered.is_empty() } else { delivered.iter().all(|d| *d == built) };
    SCOREBOARD.record(same, || format!("dispute_post_news(person {:#x}, outcome {}) {} sends; human items original {:x?} rust {:x?}", person, outcome as i8, sends, delivered, built));
}

/// The person's byte in the dispute flag table (the pointer at +4 of the dispute object, one byte per person id).
pub(crate) unsafe fn dispute_flag(this: usize, person: usize) -> usize {
    read_usize(this + 4).wrapping_add(read_usize(person)) // +4: the per-person dispute flags
}

/// Sends the dispute news item to every human manager (`nation` null: `exe::news_send_to_all_managers`) or to the
/// managers of a nation (`exe::news_send_to_nation_managers`); in the dry run, counts the send.
pub(crate) unsafe fn dispute_news_send(n: usize, nation: Option<usize>) {
    if DISPUTE_NEWS_DRY_RUN.load(Relaxed) {
        let mut built = DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner());
        built.0 = (0..news_item::ID).map(|k| read_u8(n + k)).collect();
        built.1 += 1;
        return;
    }
    match nation {
        None => {
            exe::news_send_to_all_managers(read_usize(G_NEWS), n);
        }
        Some(x) => {
            exe::news_send_to_nation_managers(read_usize(G_NEWS), n, x);
        }
    }
}

/// `dispute_post_news` (0x56a3b0, dispute.cpp; thiscall(dispute, person, club, outcome, a, b), ret 0x14).
pub(crate) const FN_DISPUTE_POST_NEWS: usize = 0x56a3b0;

/// `club_remove_favoured_staff` (0x547bd0; cdecl(club, person)): clears the person from the club's favoured slots.
pub(crate) const FN_CLUB_REMOVE_FAVOURED_STAFF: usize = 0x547bd0;

/// The comparer's dry run of `dispute_post_news`: the copy builds the news item and keeps it and the number of sends,
/// instead of sending it.
pub(crate) static DISPUTE_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// What the dry run kept: the news item's first `news_item::ID` bytes and how many sends it would make.
pub(crate) static DISPUTE_NEWS_BUILT: std::sync::Mutex<(Vec<u8>, usize)> = std::sync::Mutex::new((Vec::new(), 0));
