// --- sx09b_5: the contract disputes before the Player Status Committee (dispute.cpp) ---

/// `dispute_post_news` (0x56a3b0, dispute.cpp; thiscall(dispute, person, club, outcome, a, b), ret 0x14).
const FN_DISPUTE_POST_NEWS: usize = 0x56a3b0;
/// `club_remove_favoured_staff` (0x547bd0; cdecl(club, person)): clears the person from the club's favoured slots.
const FN_CLUB_REMOVE_FAVOURED_STAFF: usize = 0x547bd0;

/// The person's byte in the dispute flag table (the pointer at +4 of the dispute object, one byte per person id).
unsafe fn dispute_flag(this: usize, person: usize) -> usize {
    read_usize(this + 4).wrapping_add(read_usize(person)) // +4: the per-person dispute flags
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
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
