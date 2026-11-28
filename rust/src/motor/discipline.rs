use super::*;

/// The range (start, end) of the items of the discipline nation index for the nation of `time`.
///
/// Items are 8 bytes (+0 person id, +4 nation id) in the discipline nation index ([this+0xc], count +0x12, signed), for
/// the nation of `time` (+0x53), in index order: bsearch (`exe::crt_bsearch` with 0x55e5f0, cdecl) by the key (−1, nation), which
/// matches any person of the nation; then it walks back to the first equal item and forward to the last. 0 items if not
/// found.
pub(crate) unsafe fn nation_index_range(this: usize, team: Club) -> (usize, usize) {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let key = [u32::MAX, read_usize(team.nation().addr()) as u32];
    let base = read_usize(this + discipline_fields::NATION_INDEX);
    let mut e = bsearch(key.as_ptr(), base, read_i16(this + discipline_fields::NATION_INDEX_COUNT) as i32, 8, 0x55e5f0);
    if e == 0 {
        return (0, 0);
    }
    // the items equal the key by nation (+4): the key has −1 in the id
    let same = |x: usize| read_usize(x + nation_index_item::NATION_ID) as u32 == key[1];
    while e > base && same(e - 8) {
        e -= 8;
    }
    let mut end = e;
    while end <= base.wrapping_add((read_i16(this + discipline_fields::NATION_INDEX_COUNT) as i32 as usize).wrapping_mul(8)).wrapping_sub(8) && same(end) {
        end += 8;
    }
    (e, end)
}

/// In a national team match, the people of that team's nation in the nation index (binary search by nation) serve one
/// match of their ban under rule 0, one by one from a list of their ids (the count can release the ban and reorder the
/// index).
///
/// Convention: thiscall, ret 4, `discipline.cpp`, SEH frame for the local LLIST (`exe::new_llist`/`exe::drop_llist`).
/// `Discipline::serve_international_bans(team)`, called by the national team match (`0x7e8090`, in the ruling body of
/// the bans). The people of the team's nation in the nation index (`nation_index_range`) go into a list
/// (`exe::llist_add_tail` with the item and 4: a copy of the id, because the count can release the ban and reorder the
/// index) and, for each one, one match served under rule 0 (`exe::discipline_update_ban_progress` with 0 and 1). At the
/// end the list is destroyed.
/// Original: `Discipline::serve_international_bans` (0x557120, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn serve_international_bans(this: usize, team: usize) {
    let mut l = [0u8; 14];
    let list = Llist(l.as_mut_ptr() as usize);
    exe::new_llist(list);
    let (ini, fim) = nation_index_range(this, Club(team));
    if ini != 0 {
        let mut e = ini;
        while e < fim {
            exe::llist_add_tail(list, e, 4);
            e += 8;
        }
        // the cursor (+0xa) at the start (+2)
        list.set_cursor(list.head());
        let mut p = exe::llist_get_next(list);
        while p != 0 {
            covered!("cqb_9 match served");
            exe::discipline_update_ban_progress(this, read_usize(p) as u32, 0, 1); // cqb9: the match served under rule 0
            p = exe::llist_get_next(list);
        }
    }
    exe::drop_llist(list);
}

/// O plano do recurso: a entrada da pessoa no órgão da `regra` (`exe::rulingbody_find_or_add_discipline_entry` sem criar), a suspensão (20 bytes em
/// [this+4]), os valores novos de +0 (jogos) ou +2 (dias) e se ela é solta.
pub(crate) struct AppealPlan {
    pub(crate) body: usize,
    pub(crate) entry: usize,
    pub(crate) suspensao: usize,
    pub(crate) games: u8,
    pub(crate) days: i16,
    pub(crate) solta: bool,
}

/// Plan of a discipline appeal for person id under a rule: finds the entry and ban, computes reduced matches/days and
/// whether the ban is lifted.
pub(crate) unsafe fn appeal_plan(this: usize, id: u32, rule: u32, reduces: u32, minus_games: u32, minus_days: u32, new: u32, days_fixed: u32) -> AppealPlan {
    let orgao = read_usize(read_usize(this).wrapping_add((rule as i8 as isize as usize).wrapping_mul(4)));
    let entrada = exe::rulingbody_find_or_add_discipline_entry(orgao, id, 0);
    let mut r = AppealPlan { body: orgao, entry: entrada, suspensao: 0, games: 0, days: 0, solta: false };
    if entrada == 0 || read_i16(entrada + ruling_body_entry::BAN_INDEX) == -1 {
        return r;
    }
    let s = read_usize(this + discipline_fields::BANS).wrapping_add((read_i16(entrada + ruling_body_entry::BAN_INDEX) as i32 as usize).wrapping_mul(20));
    r.suspensao = s;
    r.games = read_u8(s);
    r.days = read_i16(s + ban_record::DAYS);
    if r.games != 0xff {
        if new as u8 != 0xff {
            r.games = r.games.wrapping_sub(new as u8);
            if (r.games as i8) < 0 {
                r.games = 0;
                r.solta = true;
            }
        } else if reduces as u8 != 0 {
            r.games = r.games.wrapping_sub(minus_games as u8);
            r.solta = r.games as i8 <= read_i8(s + ban_record::MATCHES_SERVED);
        } else {
            r.solta = true;
        }
    } else if days_fixed as u16 != 0xffff {
        r.days = r.days.wrapping_sub(minus_days as i16);
        if r.days <= 0 {
            r.days = 0;
            r.solta = true;
        }
    } else if reduces as u8 != 0 {
        r.days = r.days.wrapping_sub(minus_days as i16);
        r.solta = r.days <= 0;
    } else {
        r.solta = true;
    }
    r
}

/// Applies the result of a ban appeal: the ruling body drops the person's pending bans; a match ban loses matches (or
/// stops at the counter) and a timed ban loses days, and it is released when over or when the appeal lifts it.
///
/// `Discipline::apply_appeal_result(id, rule, reduces, minus_games, minus_days, new, days_fixed, _x)`, called by the
/// appeal result (`0x6abf99`, the human manager's news). With the person's entry in the ruling body
/// (`exe::rulingbody_find_or_add_discipline_entry` without creating) and a ban (`ruling_body_entry::BAN_INDEX`): the
/// body removes the person's pending bans (`exe::rulingbody_remove_pending_bans`); by matches (`+0` ≠ `0xff`): `new` ≠
/// `0xff` removes `new` matches (below 0: zero and release), otherwise `reduces` removes `minus_games` and releases if
/// it ends at or below the counter (`ban_record::MATCHES_SERVED`), otherwise releases; by days (`ban_record::DAYS`):
/// `days_fixed` ≠ −1 removes `minus_days` (down to 0: zero and release), otherwise `reduces` removes and releases at 0,
/// otherwise releases. Release: `exe::discipline_release_ban` with the index.
/// Original: `Discipline::apply_appeal_result` (0x557000, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn apply_appeal_result(this: usize, id: u32, rule: u32, reduces: u32, minus_games: u32, minus_days: u32, new: u32, days_fixed: u32, _x: u32) {
    let r = appeal_plan(this, id, rule, reduces, minus_games, minus_days, new, days_fixed);
    if r.suspensao == 0 {
        return;
    }
    covered!("cqb_9 appeal");
    exe::rulingbody_remove_pending_bans(r.body, id);
    let s = r.suspensao;
    write_u8(s, r.games);
    write_i16(s + ban_record::DAYS, r.days);
    if r.solta {
        // simplification: the original pushes the register with the index in the low word (the high part is leftover); 0x556d00 only reads
        // a palavra
        exe::discipline_release_ban(this, id, read_i16(r.entry + ruling_body_entry::BAN_INDEX) as u16 as u32);
    }
}

/// Rule (ruling body byte, 0xff none) of ban slot `vaga` in the person's 29-byte discipline record ([this+0x14]).
pub(crate) unsafe fn ban_slot_rule(this: usize, person: usize, slot: u32) -> u8 {
    let id = read_usize(person);
    read_u8(read_usize(this + discipline_fields::PERSON_RECORDS) + id.wrapping_mul(29) + (slot as i8 as isize as usize).wrapping_mul(5))
}

/// Returns the text of the ban in the person's slot, from the ruling body of its rule; with no rule in the slot, 0.
///
/// `Discipline::get_slot_text(person, slot, out, size)`, called by the person screen (`0x87618a`, with `0xc8`). A
/// person with id from `G_STAFF_COUNT` on: 0. The slot's rule (`ban_slot_rule`) `0xff`: 0; otherwise its ruling body's
/// text (`[[this] + 4 × rule]`, `exe::rulingbody_get_ban_text` with the person, the output and the size). Null person:
/// the original routine runs (it reads the id from 0).
/// Original: `Discipline::get_slot_text` (0x55d3c0, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_slot_text(this: usize, person: usize, slot: u32, out: usize, size: u32) -> u32 {
    if person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(IDX_GET_SLOT_TEXT)(this, person, slot, out, size);
    }
    if read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let rule = ban_slot_rule(this, person, slot);
    if rule == 0xff {
        return 0;
    }
    exe::rulingbody_get_ban_text(read_usize(read_usize(this).wrapping_add((rule as i8 as isize as usize).wrapping_mul(4))), person, out, size)
}

/// Ruling body of rule (signed byte): [[this] + 4 x rule].
pub(crate) unsafe fn ruling_body_of_rule(this: usize, rule: u8) -> usize {
    read_usize(read_usize(this).wrapping_add((rule as i8 as isize as usize).wrapping_mul(4)))
}

/// Person id's 29-byte discipline record in [this+0x14] (4 slots of 5 bytes, history, stored ban count).
pub(crate) unsafe fn discipline_record(this: usize, id: u32) -> usize {
    read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add((id as usize).wrapping_mul(29))
}

/// Does the ruling body apply to the club (by nation +0x17, else continent +0x1b, else always)?
pub(crate) unsafe fn ruling_body_covers_club(body: usize, club: Club) -> bool {
    let nation = read_usize(body + ruling_body::NATION_ID) as u32;
    if nation == 0xffff_ffff {
        let continent = read_usize(body + ruling_body::CONTINENT_ID) as u32;
        if continent == 0xffff_ffff {
            return true;
        }
        if club.is_null() || club.nation().addr() == 0 {
            return false;
        }
        let c = club.nation().continent();
        return c != 0 && read_usize(c) as u32 == continent;
    }
    !club.is_null() && club.nation().addr() != 0 && read_usize(club.nation().addr()) as u32 == nation
}

/// Releases the entry's ban (if +6 > -1) via release_ban with the original's dirty high bits.
pub(crate) unsafe fn release_entry_ban(this: usize, id: u32, e: usize) {
    let i = read_i16(e + ruling_body_entry::BAN_INDEX);
    if i > -1 {
        exe::discipline_release_ban(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
    }
}

/// Clears slot k of the person's discipline record (rule 0xff, index -1) and the entry (+0 = -1).
pub(crate) unsafe fn clear_discipline_slot(this: usize, id: u32, k: usize, e: usize) {
    let v = discipline_record(this, id).wrapping_add(k * 5);
    write_u8(v, 0xff);
    write_u32(v + ban_slot::VALUE, 0xffff_ffff);
    write_u32(e, 0xffff_ffff);
}

/// Adds a ban: when the person already has a ban under one of the 4 rules, extends the old one (matches or days) and
/// flags both; otherwise copies the new ban into the array's first free slot (or the end, growing by 100 up to
/// `0x7d00`), links each rule to it and, with the fourth rule, adds {id, nation} to the nation index.
///
/// `Discipline::add_ban(id, ban)`: if any of the 4 rules of the new ban (`ban_record::RULES`) already has a ban for the
/// person (the body's entry with `ruling_body_entry::BAN_INDEX` > −1), it extends the old one (matches `+0` added
/// together or, with either of the two by days (`0xff`), the days `ban_record::DAYS`) and flags both
/// (`ban_record::FLAGS` bit 0); otherwise (bit 0 of the new one cleared for each rule with no ban) it copies the new
/// one (`exe::banrecord_operator_3d`) into the first free slot of the array (`discipline_fields::BANS`; `+0` = `0xff`
/// and `ban_record::DAYS` = −1), or at the end (count `discipline_fields::BAN_COUNT`, capacity
/// `discipline_fields::BAN_CAPACITY`; grows by 100, up to `0x7d00`, malloc (`exe::crt_malloc`)/realloc
/// (`FN_CRT_REALLOC`), the new slots free), links each rule to the index (`exe::rulingbody_attach_ban`) and, with the
/// fourth rule (`ban_record::NATION_RULE`), puts {id, nation} into the nation index (`discipline_fields::NATION_INDEX`,
/// capacity `discipline_fields::NATION_INDEX_CAPACITY`, count `discipline_fields::NATION_INDEX_COUNT`; grows by 20;
/// qsort `0x55e5f0`). "Error" boxes (capacity at maximum, out of memory) via show_error_at.
/// Original: `Discipline::add_ban` (0x556940, discipline.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_ban(this: usize, id: u32, ban: usize) {
    for k in 0..4usize {
        let rule = read_u8(ban + ban_record::RULES + k);
        if rule == 0xff {
            continue;
        }
        let e = ruling_body_entry(ruling_body_of_rule(this, rule), id);
        if e != 0 && read_i16(e + ruling_body_entry::BAN_INDEX) > -1 {
            covered!("cpo_11 suspension extended");
            let s = read_usize(this + discipline_fields::BANS).wrapping_add((read_i16(e + ruling_body_entry::BAN_INDEX) as i32 as usize).wrapping_mul(20));
            let games = read_u8(s);
            let novos = read_u8(ban);
            if games != 0xff && novos != 0xff {
                write_u8(s, novos.wrapping_add(games));
            } else {
                write_u16(s + ban_record::DAYS, (read_i16(s + ban_record::DAYS) as u16).wrapping_add(read_i16(ban + ban_record::DAYS) as u16));
            }
            *((ban + ban_record::FLAGS) as *mut u8) |= 1;
            *((s + ban_record::FLAGS) as *mut u8) |= 1;
            return;
        }
        *((ban + ban_record::FLAGS) as *mut u8) &= 0xfe;
    }
    let n = read_i16(this + discipline_fields::BAN_COUNT);
    let mut slot: i16 = -1;
    for i in 0..n.max(0) {
        let s = read_usize(this + discipline_fields::BANS).wrapping_add(i as usize * 20);
        if read_u8(s) == 0xff && read_i16(s + ban_record::DAYS) == -1 {
            slot = i;
            exe::banrecord_operator_3d(s, ban);
            break;
        }
    }
    if slot == -1 {
        let count = read_i16(this + discipline_fields::BAN_COUNT);
        let cap = read_i16(this + discipline_fields::BAN_CAPACITY);
        if count < cap {
            write_u16(this + discipline_fields::BAN_COUNT, count.wrapping_add(1) as u16);
            exe::banrecord_operator_3d(read_usize(this + discipline_fields::BANS).wrapping_add((count as i32 as usize).wrapping_mul(20)), ban);
            slot = read_i16(this + discipline_fields::BAN_COUNT).wrapping_sub(1);
        } else {
            if cap >= 0x7d00 {
                show_error_at(0x9b8f18, 0x401);
                return;
            }
            let new = cap.wrapping_add(100);
            write_u16(this + discipline_fields::BAN_CAPACITY, new as u16);
            let size = (new as i32 as u32).wrapping_mul(20);
            let p = if read_usize(this + discipline_fields::BANS) == 0 {
                exe::crt_malloc(size)
            } else {
                let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
                realloc(read_usize(this + discipline_fields::BANS), size)
            };
            write_u32(this + discipline_fields::BANS, p as u32);
            if p == 0 {
                show_error_at(0x9b8f18, 0x410);
                return;
            }
            slot = read_i16(this + discipline_fields::BAN_COUNT);
            write_u16(this + discipline_fields::BAN_COUNT, slot.wrapping_add(1) as u16);
            exe::banrecord_operator_3d(p.wrapping_add((slot as i32 as usize).wrapping_mul(20)), ban);
            let mut i = read_i16(this + discipline_fields::BAN_COUNT);
            while i < read_i16(this + discipline_fields::BAN_CAPACITY) {
                let s = read_usize(this + discipline_fields::BANS).wrapping_add((i as i32 as usize).wrapping_mul(20));
                write_u8(s, 0xff);
                write_u16(s + ban_record::DAYS, 0xffff);
                i = i.wrapping_add(1);
            }
        }
    }
    covered!("cpo_11 new suspension");
    for k in 0..4usize {
        let rule = read_u8(ban + ban_record::RULES + k);
        if rule != 0xff {
            exe::rulingbody_attach_ban(ruling_body_of_rule(this, rule), id, slot as u16 as u32); // links the rule to the suspension
        }
    }
    if read_u8(ban + ban_record::NATION_RULE) == 0xff {
        return;
    }
    let cap = read_i16(this + discipline_fields::NATION_INDEX_CAPACITY);
    if cap == read_i16(this + discipline_fields::NATION_INDEX_COUNT) {
        let new = cap.wrapping_add(0x14);
        write_u16(this + discipline_fields::NATION_INDEX_CAPACITY, new as u16);
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realloc(read_usize(this + discipline_fields::NATION_INDEX), (new as i32 as u32).wrapping_mul(8));
        write_u32(this + discipline_fields::NATION_INDEX, p as u32);
        if p == 0 {
            show_error_at(0x9b8f18, 0x43e);
            return;
        }
    }
    let n = read_i16(this + discipline_fields::NATION_INDEX_COUNT);
    let item = read_usize(this + discipline_fields::NATION_INDEX).wrapping_add((n as i32 as usize).wrapping_mul(8));
    write_u32(item, id);
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    write_u32(item + nation_index_item::NATION_ID, read_usize(person.nation().addr()) as u32); // the nation
    write_u16(this + discipline_fields::NATION_INDEX_COUNT, n.wrapping_add(1) as u16);
    exe::crt_qsort(read_usize(this + discipline_fields::NATION_INDEX), (read_i16(this + discipline_fields::NATION_INDEX_COUNT) as i32) as u32, 8, 0x55e5f0);
}

/// on a club move, frees each of the person's 4 record slots whose ruling body (nation `ruling_body::NATION_ID` or
/// continent `ruling_body::CONTINENT_ID`, `ruling_body_covers_club`) does not cover the new club: releases the ban,
/// frees the slot and the entry; with held bans, shows the "Error" box
/// Original: `Discipline::clear_bans_invalid_for_club` (0x556270, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn clear_bans_invalid_for_club(this: usize, person: usize, _nada: u32, club: usize) {
    let id = read_usize(person) as u32;
    let r = discipline_record(this, id);
    for k in 0..4usize {
        let rule = read_u8(r + k * 5);
        if rule == 0xff {
            continue;
        }
        let body = ruling_body_of_rule(this, rule);
        if ruling_body_covers_club(body, Club(club)) {
            continue;
        }
        let e = ruling_body_entry(body, id);
        if e == 0 {
            continue;
        }
        covered!("cpo_11 transferred frees the slot");
        release_entry_ban(this, id, e);
        if read_u8(r + discipline_record::HELD_BAN_COUNT) != 0 {
            show_error_at(0x9b8f18, 0x239);
        }
        clear_discipline_slot(this, id, k, e);
    }
}

/// What `restore_held_bans` is going to do, read beforehand (the list of held bans loses and frees its items): the contract and
/// the club, the person's held items (the slot and the 5 bytes) and, after them, the slots 0..2 not restored that the
/// club does not cover.
pub(crate) struct ContractReturnPlan {
    pub(crate) contract: usize,
    pub(crate) club: usize,
    pub(crate) guardadas: Vec<(usize, [u8; 5])>,
    pub(crate) rest: u8,
    pub(crate) soltas: Vec<(usize, usize)>,
}

/// Read-only plan of `restore_held_bans`: the person's contract and club, stored bans restored from the list, and bans to release
/// for bodies not covering the club.
pub(crate) unsafe fn return_bans_plan(this: usize, person: usize) -> ContractReturnPlan {
    let id = read_usize(person) as u32;
    let r = discipline_record(this, id);
    let contrato = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    let clube = if contrato.is_null() { 0 } else { read_usize(G_CLUB_TABLE).wrapping_add(contrato.club().wrapping_mul(club::SIZE)) };
    let mut p = ContractReturnPlan { contract: contrato.addr(), club: clube, guardadas: Vec::new(), rest: read_u8(r + discipline_record::HELD_BAN_COUNT), soltas: Vec::new() };
    let list = Llist(this + discipline_fields::HELD_BANS);
    list.set_cursor((read_usize(this + discipline_fields::HELD_BANS_HEAD) as u32) as usize);
    let mut slots = [0u8; 20];
    core::ptr::copy_nonoverlapping(r as *const u8, slots.as_mut_ptr(), 20);
    let mut marks = [false; 4];
    let mut item = exe::llist_get_next(list);
    while item != 0 && (p.rest as i8) > 0 {
        if read_usize(item) as u32 == id {
            let k = (read_u8(item + held_ban::SLOT) as i8 as isize as usize) & 3;
            let mut b = [0u8; 5];
            core::ptr::copy_nonoverlapping((item + held_ban::SLOT_DATA) as *const u8, b.as_mut_ptr(), 5);
            slots[k * 5..k * 5 + 5].copy_from_slice(&b);
            marks[k] = true;
            p.guardadas.push((k, b));
            p.rest = p.rest.wrapping_sub(1);
        }
        item = exe::llist_get_next(list);
    }
    for k in 0..3usize {
        if marks[k] || slots[k * 5] == 0xff {
            continue;
        }
        let body = ruling_body_of_rule(this, slots[k * 5]);
        if !ruling_body_covers_club(body, Club(clube)) {
            let e = ruling_body_entry(body, id);
            if e != 0 {
                p.soltas.push((k, e));
            }
        }
    }
    p
}

/// When a loan ends, puts the person's held bans back into their slots (freeing the occupied slot first; the item
/// leaves the list) and frees slots 0..2 not restored whose ruling body does not cover the contract club.
///
/// `Discipline::restore_held_bans(person)`, at the end of a loan (`0x4d07e0`): the contract
/// (`exe::contract_manager_get_contract`, stored in `G_CONTRACT_LOOKUP_RESULT`) gives the club; for each held ban of
/// the person in list `discipline_fields::HELD_BANS` (id, slot `held_ban::SLOT`, the 5 bytes `held_ban::SLOT_DATA`),
/// while there are held bans (`discipline_record::HELD_BAN_COUNT` of the record, signed): the occupied slot is freed
/// first (the ban released, `release_entry_ban`), the held ban goes back to the slot, leaves the list
/// (`exe::llist_remove`, which frees it) and the count drops. Then the slots 0..2 not restored (never the fourth: the
/// original's `k != 3`) whose ruling body does not cover the club are freed.
/// Original: `Discipline::restore_held_bans` (0x556570, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn restore_held_bans(this: usize, person: usize) {
    let id = read_usize(person) as u32;
    let r = discipline_record(this, id);
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_u32(G_CONTRACT_LOOKUP_RESULT, contract.addr() as u32);
    let club = if contract.is_null() { 0 } else { read_usize(G_CLUB_TABLE).wrapping_add(contract.club().wrapping_mul(club::SIZE)) };
    let list = Llist(this + discipline_fields::HELD_BANS);
    list.set_cursor((read_usize(this + discipline_fields::HELD_BANS_HEAD) as u32) as usize);
    let mut marks = [0u32; 4];
    let mut item = exe::llist_get_next(list);
    while item != 0 && *((r + discipline_record::HELD_BAN_COUNT) as *const i8) > 0 {
        if read_usize(item) as u32 == id {
            let k = read_u8(item + held_ban::SLOT) as i8 as isize as usize;
            let rule = read_u8(r.wrapping_add(k.wrapping_mul(5)));
            if rule != 0xff {
                let e = ruling_body_entry(ruling_body_of_rule(this, rule), id);
                if e != 0 {
                    release_entry_ban(this, id, e);
                    clear_discipline_slot(this, id, k, e);
                }
            }
            covered!("cpo_11 stored one returned");
            marks[k & 3] = 1;
            let v = r.wrapping_add(k.wrapping_mul(5));
            write_u32(v, read_usize(item + held_ban::SLOT_DATA) as u32);
            write_u8(v + 4, read_u8(item + 9));
            exe::llist_remove(list, item); // libera o item
            write_u8(r + discipline_record::HELD_BAN_COUNT, read_u8(r + discipline_record::HELD_BAN_COUNT).wrapping_sub(1));
        }
        item = exe::llist_get_next(list);
    }
    for k in 0..4usize {
        let rule = read_u8(r + k * 5);
        if marks[k] != 0 || k == 3 || rule == 0xff {
            continue;
        }
        let body = ruling_body_of_rule(this, rule);
        if ruling_body_covers_club(body, Club(club)) {
            continue;
        }
        let e = ruling_body_entry(body, id);
        if e != 0 {
            covered!("cpo_11 loan end frees the slot");
            release_entry_ban(this, id, e);
            clear_discipline_slot(this, id, k, e);
        }
    }
}

/// O caminho da `add_ban`, lido antes: a suspensão velha estendida (e os valores novos dela), ou a vaga da nova e a
/// conta do vetor depois; `None` nas caixas (capacidade no máximo).
pub(crate) enum SuspensionPlan {
    Estender { s: usize, games: u8, days: i16 },
    Nova { slot: i16, count: i16 },
    Caixa,
}

/// Plan for adding a ban: extend an existing one for the same body, reuse a free ban slot, append, or error box when
/// full.
pub(crate) unsafe fn add_ban_plan(this: usize, id: u32, ban: usize) -> SuspensionPlan {
    for k in 0..4usize {
        let rule = read_u8(ban + ban_record::RULES + k);
        if rule == 0xff {
            continue;
        }
        let e = ruling_body_entry(ruling_body_of_rule(this, rule), id);
        if e != 0 && read_i16(e + ruling_body_entry::BAN_INDEX) > -1 {
            let s = read_usize(this + discipline_fields::BANS).wrapping_add((read_i16(e + ruling_body_entry::BAN_INDEX) as i32 as usize).wrapping_mul(20));
            let (jogos, novos) = (read_u8(s), read_u8(ban));
            return if jogos != 0xff && novos != 0xff {
                SuspensionPlan::Estender { s, games: novos.wrapping_add(jogos), days: read_i16(s + ban_record::DAYS) }
            } else {
                SuspensionPlan::Estender { s, games: jogos, days: read_i16(s + ban_record::DAYS).wrapping_add(read_i16(ban + ban_record::DAYS)) }
            };
        }
    }
    let n = read_i16(this + discipline_fields::BAN_COUNT);
    for i in 0..n.max(0) {
        let s = read_usize(this + discipline_fields::BANS).wrapping_add(i as usize * 20);
        if read_u8(s) == 0xff && read_i16(s + ban_record::DAYS) == -1 {
            return SuspensionPlan::Nova { slot: i, count: n };
        }
    }
    if n < read_i16(this + discipline_fields::BAN_CAPACITY) || read_i16(this + discipline_fields::BAN_CAPACITY) < 0x7d00 {
        return SuspensionPlan::Nova { slot: n, count: n.wrapping_add(1) };
    }
    SuspensionPlan::Caixa
}

/// Person id's 29-byte discipline record in [this+0x14].
pub(crate) unsafe fn person_discipline_record(this: usize, id: usize) -> usize {
    read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add(id.wrapping_mul(29))
}

/// Ruling body of rule (signed byte): [[this] + 4 x rule].
pub(crate) unsafe fn ruling_body_for_rule(this: usize, rule: u8) -> usize {
    read_usize(read_usize(this).wrapping_add((rule as i8 as isize as usize).wrapping_mul(4)))
}

/// The person's 14-byte entry in a ruling body (`exe::rulingbody_find_or_add_discipline_entry` without creating; +6 ban index).
pub(crate) unsafe fn ruling_body_entry(body: usize, id: u32) -> usize {
    exe::rulingbody_find_or_add_discipline_entry(body, id, 0)
}

/// Person's item (id, nation) in the discipline nation index by bsearch on his nation, then on his second nation; 0 if
/// absent.
pub(crate) unsafe fn find_in_nation_index(this: usize, id: u32) -> usize {
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    let mut key = [id, read_usize(person.nation().addr()) as u32];
    let e = exe::crt_bsearch((key.as_ptr()) as usize, read_usize(this + discipline_fields::NATION_INDEX), (read_i16(this + discipline_fields::NATION_INDEX_COUNT) as i32) as u32, 8, 0x55e5f0);
    if e != 0 {
        return e;
    }
    let second = read_usize(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)) + staff::SECOND_NATION);
    if second == 0 {
        return 0;
    }
    key[1] = read_usize(second) as u32;
    exe::crt_bsearch((key.as_ptr()) as usize, read_usize(this + discipline_fields::NATION_INDEX), (read_i16(this + discipline_fields::NATION_INDEX_COUNT) as i32) as u32, 8, 0x55e5f0)
}

/// O que a `update_ban_progress` faz com o jogo: a entrada no órgão, a suspensão (20 bytes em [this+4]), o contador novo de jogos
/// cumpridos (+1) e se a suspensão termina.
pub(crate) struct SuspensionMatchUpdate {
    pub(crate) entry: usize,
    pub(crate) suspensao: usize,
    pub(crate) counter: Option<u8>,
    pub(crate) cumprida: bool,
}

/// The read-only plan of a match for a ban (only reads; `exe::rulingbody_find_or_add_discipline_entry` without creating, and `exe::date_add_days` only writes its scratch
/// date): the person's suspension in the body of `regra`; reports completion.
///
/// By matches (+0 ≠ 0xff): from the start (day +4, year +6; today [0xae2c90]: day +0, year +2), with `conta`, +1 on the
/// counter (+1), and it ends on reaching +0.
///
/// By days (+0 = 0xff): it ends when today reaches start + days (+2) (`exe::date_add_days`).
pub(crate) unsafe fn serve_ban_match_plan(this: usize, id: u32, rule: u32, count: u32) -> SuspensionMatchUpdate {
    let mut j = SuspensionMatchUpdate { entry: 0, suspensao: 0, counter: None, cumprida: false };
    let e = ruling_body_entry(ruling_body_for_rule(this, rule as u8), id);
    if e == 0 || read_i16(e + ruling_body_entry::BAN_INDEX) == -1 {
        return j;
    }
    j.entry = e;
    let cinco = (read_i16(e + ruling_body_entry::BAN_INDEX) as i32 as u32).wrapping_mul(5); // the original’s lea (the high part goes in edx below)
    let s = read_usize(this + discipline_fields::BANS).wrapping_add((cinco as usize).wrapping_mul(4));
    j.suspensao = s;
    let today = read_usize(G_CURRENT_DATE) as u32;
    let (dia, ano) = (today as u16 as i16, (today >> 16) as u16);
    let games = read_u8(s);
    if games != 0xff {
        let (dia0, ano0) = (read_i16(s + ban_record::START_DATE), read_i16(s + 6) as u16);
        if ano < ano0 || (ano == ano0 && dia < dia0) { // has not started yet
            covered!("cpm_4 before the start");
            return j;
        }
        if count == 0 {
            return j;
        }
        covered!("cpm_4 match served");
        let n = read_u8(s + ban_record::MATCHES_SERVED).wrapping_add(1); // one more match served
        j.counter = Some(n);
        j.cumprida = n == games;
    } else {
        let mut end = [0u8; 8];
        // the original pushes edx with the days in dx and, in the high part, the index × 5 one (zero below 13108 suspensions)
        let f = exe::date_add_days(s + ban_record::START_DATE, (end.as_mut_ptr()) as usize, (cinco & 0xffff_0000) | read_i16(s + ban_record::DAYS) as u16 as u32);
        let (dia1, ano1) = (read_i16(f), read_i16(f + 2) as u16);
        if ano < ano1 || (ano == ano1 && dia < dia1) {
            return j;
        }
        j.cumprida = true;
    }
    j
}

/// the person's ban in the rule's ruling body: match bans, from the start date (`ban_record::START_DATE`/+6) and when
/// counting, +1 to the counter (`ban_record::MATCHES_SERVED`); on reaching the total (+0), or day bans once today
/// reaches start + days (`ban_record::DAYS`, `exe::date_add_days`), it goes to the history (`FN_FILE_LLIST_ADD_NODE`)
/// and is released (`exe::discipline_release_ban`)
/// Original: `Discipline::update_ban_progress` (0x556e90, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn update_ban_progress(this: usize, id: u32, rule: u32, count: u32) {
    let j = serve_ban_match_plan(this, id, rule, count);
    if let Some(n) = j.counter {
        write_u8(j.suspensao + 1, n);
    }
    if !j.cumprida {
        return;
    }
    let r = person_discipline_record(this, id as usize);
    let history: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let g = history(read_usize(this + discipline_fields::BAN_HISTORY), r + discipline_record::HISTORY_START, r + discipline_record::HISTORY_LAST, j.suspensao);
    // the index goes in ax; the high part of eax is that of 0x58e340’s return (0x556d00 only reads the word)
    exe::discipline_release_ban(this, id, (g as u32 & 0xffff_0000) | read_i16(j.entry + ruling_body_entry::BAN_INDEX) as u16 as u32);
}

/// Releases a ban: each ruling body of the ban's 4 rules forgets it; on the fourth rule, the person's item leaves the
/// nation index; the ban becomes free. Returns the ban.
///
/// `Discipline::release_ban(id, index)`: ban `index` (20 bytes in `discipline_fields::BANS`; the word is signed): for
/// each of the 4 rules (`ban_record::RULES`, `0xff` = none), the ruling body forgets it (`exe::rulingbody_detach_ban`);
/// on the fourth, the person's item leaves the nation index (the last one is moved into its place, count
/// `discipline_fields::NATION_INDEX_COUNT` − 1, qsort `exe::crt_qsort` with `0x55e5f0`; if not found, "Error" box and
/// it carries on). Finally the ban becomes free (`+0` = `0xff`, `ban_record::DAYS` = −1).
/// Original: `Discipline::release_ban` (0x556d00, discipline.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn release_ban(this: usize, id: u32, index: u32) -> usize {
    let s = read_usize(this + discipline_fields::BANS).wrapping_add((index as u16 as i16 as i32 as usize).wrapping_mul(20));
    for k in 0..4usize {
        let rule = read_u8(s + ban_record::RULES + k);
        if rule == 0xff {
            continue;
        }
        covered!("cpm_4 release from the body");
        exe::rulingbody_detach_ban(ruling_body_for_rule(this, rule), id); // the ruling body forgets the suspension
        if k != 3 {
            continue;
        }
        let e = find_in_nation_index(this, id);
        if e == 0 {
            show_error_at(0x9b8f18, 0x47d);
            continue;
        }
        let n = read_i16(this + discipline_fields::NATION_INDEX_COUNT).wrapping_sub(1);
        write_i16(this + discipline_fields::NATION_INDEX_COUNT, n);
        let last = read_usize(this + discipline_fields::NATION_INDEX).wrapping_add((n as i32 as usize).wrapping_mul(8));
        write_u32(e, read_usize(last) as u32);
        write_u32(e + nation_index_item::NATION_ID, read_usize(last + nation_index_item::NATION_ID) as u32);
        exe::crt_qsort(read_usize(this + discipline_fields::NATION_INDEX), (read_i16(this + discipline_fields::NATION_INDEX_COUNT) as i32) as u32, 8, 0x55e5f0);
    }
    write_u8(s, 0xff);
    write_u16(s + ban_record::DAYS, 0xffff);
    s
}

/// Clears the person's ban history: the last entry of the history is dropped (`discipline_record::HISTORY_LAST` of the
/// 29-byte record = −1, via `exe::file_llist_reset_last_offset`).
///
/// `Discipline::clear_ban_history(person)`: the person's ban history (29-byte record of the id in
/// `discipline_fields::PERSON_RECORDS`) loses its last entry (`exe::file_llist_reset_last_offset` on
/// `discipline_fields::BAN_HISTORY`: `discipline_record::HISTORY_LAST` = −1). Returns what
/// `exe::file_llist_reset_last_offset` returns. Null person or not a player (`.player()`): the original routine runs
/// (fatal box).
/// Original: `Discipline::clear_ban_history` (0x55e1d0, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_ban_history(this: usize, person: usize) -> usize {
    let person = Staff(person);
    if person.is_null() || person.player().is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLEAR_BAN_HISTORY)(this, person.addr()); // fatal box: original only
    }
    let r = person_discipline_record(this, read_usize(person.addr()));
    exe::file_llist_reset_last_offset(read_usize(this + discipline_fields::BAN_HISTORY), r + discipline_record::HISTORY_START, r + discipline_record::HISTORY_LAST)
}

/// Copies a ban record (`0x14` bytes): the fields, the date (`ban_record::START_DATE`) and bits 0..2 of
/// `ban_record::FLAGS`. Returns `this`.
///
/// `BanRecord::operator=`: copies the ban record from `source`: `+0` and `+1` (bytes), `ban_record::DAYS` (word), the
/// date at `ban_record::START_DATE` (`exe::date_operator_3d`, return value in a local), `+0xc..+0xf` (byte by byte),
/// `ban_record::REASON` (byte), `ban_record::REASON_AMOUNT` (word) and bits 0..2 of `ban_record::FLAGS` (the other bits
/// are kept).
/// Original: `BanRecord::operator=` (0x556c70, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_operator(this: usize, source: usize) -> usize {
    let b = |o: usize| write_u8(this + o, read_u8(source + o));
    let w = |o: usize| write_u16(this + o, read_u16(source + o));
    b(0);
    b(1);
    w(ban_record::DAYS);
    let mut local = [0u8; 8];
    exe::date_operator_3d(this + ban_record::START_DATE, (&mut local) as *mut [u8; 8] as usize, source + ban_record::START_DATE);
    for o in 0xc..0x10 {
        b(o);
    }
    b(ban_record::REASON);
    w(ban_record::REASON_AMOUNT);
    let m = (this + ban_record::FLAGS) as *mut u8;
    *m = (*m & !7) | (read_u8(source + ban_record::FLAGS) & 7); // the three marks
    this
}

// the original runs first and updates the day’s fitness (0x62c680, with a random roll); the copy, afterwards, finds the day already done
compare_value!(compare_get_match_condition, get_match_condition, IDX_GET_MATCH_CONDITION, (jogador: usize, em_porcento: u32) -> u16);

/// starts the new season of the competition's discipline rules (virtual method 0)
/// Original: `discipline_new_season` (0x556210, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn discipline_new_season(this: usize, id: *const i32) {
    let competition = League(*((*(G_COMP_OBJECT_TABLE as *const usize) as isize + 4 * *id as isize) as *const usize));
    if competition.is_null() {
        return;
    }
    let k = competition.discipline_rule() as i8;
    if k == -1 {
        return;
    }
    let rules = *((*(this as *const usize) as isize + 4 * k as isize) as *const usize);
    if rules != 0 {
        let metodo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(*(*(rules as *const usize) as *const usize));
        metodo(rules);
    }
}

/// the day of each of the 43 discipline rules (virtual method 1)
/// Original: `discipline_daily_update` (0x556240, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn discipline_daily_update(this: usize, today: usize) {
    for k in 0..43usize {
        let rules = read_usize(read_usize(this) + 4 * k);
        if rules != 0 {
            let metodo: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(rules) + ruling_body_vtable::DAILY_UPDATE));
            metodo(rules, today);
        }
    }
}

/// writes the type and value of one of the person's slots in the discipline table
/// Original: `discipline_set_slot` (0x55cb20, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn discipline_set_slot(this: usize, id: i32, slot: i8, kind: u8, value: u32) {
    let a = (read_usize(this + discipline_fields::PERSON_RECORDS) as isize + id as isize * 29 + slot as isize * 5) as usize;
    write_u8(a, kind);
    write_u32(a + ban_slot::VALUE, value);
}

// simplification: the first two only dispatch through the virtual table (effects outside): proven by the career

/// Tells whether the player is suspended: by the general ruling body (`exe::rulingbody_is_player_banned` with 1) or by
/// the competition's (`.discipline_rule()`, with 0); in the latter case, at a human-managed club, he leaves his slot
/// among the 20 at `club::TEAM_SELECTED`.
///
/// `Discipline::is_player_suspended(player, comp, extra, fixture)`: player outside the people (`G_STAFF_COUNT`) or a
/// non-existent competition (id in `*comp`, array `G_COMP_OBJECT_TABLE`): 0. Suspended by the general body (`[[this]]`,
/// `exe::rulingbody_is_player_banned` with 1): 1. Otherwise, by the competition's body (`.discipline_rule()`, ≠ −1,
/// with 0): 1, and if his club (`.club_job()`) is human-managed (`exe::human_manager_is_human_club` with 0, 1, 0, 0),
/// he leaves the slot he holds among the 20 at `club::TEAM_SELECTED`. Otherwise 0.
/// Original: `Discipline::is_player_suspended` (0x55d270, discipline.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn is_player_suspended(this: usize, player: usize, comp: *const i32, extra: u32, fixture: usize) -> u32 {
    let player = Staff(player);
    if !player.is_null() && read_usize(player.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let v = read_usize(G_COMP_OBJECT_TABLE);
    if v == 0 {
        return 0;
    }
    let c = League(read_usize(v.wrapping_add((*comp as usize).wrapping_mul(4))));
    if c.is_null() {
        return 0;
    }
    let tribunais = read_usize(this);
    if exe::rulingbody_is_player_banned(read_usize(tribunais), player, 1, extra, fixture) != 0 {
        return 1; // the general suspension
    }
    let t = c.discipline_rule() as i8;
    if t == -1 {
        return 0;
    }
    let b = read_usize(tribunais.wrapping_add((t as isize as usize).wrapping_mul(4)));
    if b == 0 || exe::rulingbody_is_player_banned(b, player, 0, extra, fixture) == 0 { // without the competition suspension
        return 0;
    }
    covered!("cpk_3 suspended by the competition");
    let club = player.club_job();
    if !club.is_null() {
        if exe::human_manager_is_human_club(club, 0, 1, (0) as u32, 0) != 0 {
            if let Some(s) = (0..20).map(|k| club.addr() + club::TEAM_SELECTED + k * 4).find(|&s| read_usize(s) == player.addr()) {
                write_usize(s, 0); // leaves the slot
            }
        }
    }
    1
}

/// Removes the person's items from the held-ban list (`discipline_fields::HELD_BANS`) while the
/// `discipline_record::HELD_BAN_COUNT` counter of the person's record is positive.
///
/// `Discipline::drop_held_bans(person)`: from the list of held bans (`discipline_fields::HELD_BANS`; the cursor goes
/// back to the start, `.set_cursor(.head())`), it removes (`list_remove`) each item of id `*person` while the counter
/// of the person's record (`discipline_record::HELD_BAN_COUNT`, signed) is positive, decrementing it for each item.
/// Original: `Discipline::drop_held_bans` (0x5567d0, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn drop_held_bans(this: usize, person: *const u32) {
    let l = Llist(this + discipline_fields::HELD_BANS);
    let n = stored_suspension_counter(this, *person as usize);
    l.set_cursor(l.head());
    let mut e = contract_manager_list_next(l.addr());
    while e != 0 && *n > 0 {
        if read_usize(e) as u32 == *person {
            list_remove(l, e);
            *n = (*n).wrapping_sub(1); // one fewer
        }
        e = contract_manager_list_next(l.addr());
    }
}

/// Holds bans for a loan: the bans (slots 0 to 2) that do not apply at the loan club go to the held list
/// (`discipline_fields::HELD_BANS`), the slot becomes −1 and the `discipline_record::HELD_BAN_COUNT` counter goes up.
///
/// `Discipline::hold_bans_for_loan(player)`: with the player's contract sheet
/// (`exe::contract_manager_get_loan_contract` on `G_CONTRACT_MANAGER`), each ban in his record (slots 0 to 2) that does
/// not apply at the loan club goes to the list of held bans (`discipline_fields::HELD_BANS`, `exe::llist_add_tail` with
/// 10 bytes: the id, the slot and its 5 bytes), the slot becomes −1 and the record's
/// `discipline_record::HELD_BAN_COUNT` counter goes up. No contract sheet: the original routine runs ("Error" box).
/// Original: `Discipline::hold_bans_for_loan` (0x5563f0, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hold_bans_for_loan(this: usize, player: usize) {
    let sheet = exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), (player as *const i32) as usize);
    if sheet == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HOLD_BANS_FOR_LOAN)(this, player); // message box: original only
    }
    let (reg, vagas) = suspensions_to_store(this, sheet, player);
    for k in vagas {
        let slot = reg + 5 * k as usize;
        let mut r = [0u8; 10];
        r[..4].copy_from_slice(&(read_usize(player) as u32).to_le_bytes());
        r[4] = k; // the place
        r[5..].copy_from_slice(core::slice::from_raw_parts(slot as *const u8, 5));
        (exe::llist_add_tail(this + discipline_fields::HELD_BANS, (r.as_ptr()) as *const u8 as usize, 10) as u32);
        write_u32(slot, u32::MAX);
        write_u8(slot + 4, 0xff);
        write_u8(reg + discipline_record::HELD_BAN_COUNT, read_u8(reg + discipline_record::HELD_BAN_COUNT).wrapping_add(1));
    }
}

/// Tells whether the person is one booking away from a ban: with a valid id, it asks the competition's ruling body;
/// otherwise 0.
///
/// `Discipline::is_one_booking_from_ban(person, competition)`: with a valid id, the competition (`G_COMP_OBJECT_TABLE`
/// + 4 × id) and its ruling body (`.discipline_rule()`, `0xff` if none; `[this]` + 4 × body), the question to the body
/// (`exe::rulingbody_check_one_booking_from_ban` with the id, 0, 0 and the competition's `.competition()`); otherwise
/// 0. Called by the club screen (`0x4849f0`).
/// Original: `Discipline::is_one_booking_from_ban` (0x55d360, discipline.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn is_one_booking_from_ban(this: usize, person: usize, competition: usize) -> u32 {
    if person != 0 && read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let comp = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(competition) as u32).wrapping_mul(4) as usize)));
    if comp.is_null() || person == 0 {
        return 0;
    }
    let body = comp.discipline_rule();
    if body == 0xff {
        return 0;
    }
    let o = read_usize(read_usize(this).wrapping_add((body as i8 as i32 as u32).wrapping_mul(4) as usize));
    if o == 0 {
        return 0;
    }
    exe::rulingbody_check_one_booking_from_ban(o, read_usize(person) as u32, 0, 0, comp.competition().addr() as u32)
}

/// the penalty at the table row and column, if the type matches
/// Original: `table_penalty` (0x55cae0, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn table_penalty(t: usize, row: i32, column: i8, kind: i8) -> i32 {
    let e = read_usize(t + discipline_fields::PERSON_RECORDS)
        .wrapping_add((row as usize).wrapping_mul(29))
        .wrapping_add((column as isize * 5) as usize);
    if read_i8(e) == kind {
        read_i32(e + ban_slot::VALUE)
    } else {
        -1
    }
}

/// Processes a match for discipline: the competition's ruling body judges it; without one, in friendlies, the league
/// ruling bodies of both clubs.
///
/// `Discipline::process_match(fixture, result)`: the ruling body of the match's competition (`+0`, id > −1;
/// `comp_tribunal`) judges it (`exe::rulingbody_process_match` with 1, 1, 1). Without one, in a friendly
/// (`exe::friendly_is_friendly_competition` on `G_FRIENDLY` with `.competition()`) or in competition
/// `KEY_CLUB_COMP_RESERVE`: the league (`club::DIVISION`) body of the home club (`+0x1c`, with 1, 0, 0) and that of the
/// away club (`+0x20`, with 0, 1, 0). Returns what the first body returns (the other paths, 0; nobody reads it).
/// Original: `Discipline::process_match` (0x55ca00, discipline.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn process_match(this: usize, fixture: usize, result: u32) -> u32 {
    let fixture = Fixture(fixture);
    let id = read_usize(fixture.addr()) as i32;
    if id > -1 {
        if let Some(b) = comp_tribunal(this, id) {
            return exe::rulingbody_process_match(b, fixture, result, 1, 1, 1) as u32;
        }
    }
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), fixture.competition().addr()) == 0 && read_usize(fixture.addr()) != read_usize(KEY_CLUB_COMP_RESERVE) {
        return 0;
    }
    for (lado, a, b) in [(0x1cusize, 1u32, 0u32), (0x20, 0, 1)] {
        let league = read_usize(read_usize(fixture.addr() + lado) + club::DIVISION);
        if league == 0 {
            continue;
        }
        if let Some(t) = comp_tribunal(this, read_usize(league) as i32) { // the tribunal of the club’s league
            (exe::rulingbody_process_match(t, fixture, result, a, b, 0) as u32);
        }
    }
    0
}

/// Removes a player from discipline: clears the ban history, releases the bans and frees the 4 slots, drops the held
/// bans from the list and leaves the 29-byte record free (`0xff`, count 0).
///
/// `Discipline::remove_player(id)`, called by the regen (`0x7abac0`): deletes the ban history
/// (`exe::discipline_clear_ban_history`), frees the 4 slots of the 29-byte record (`discipline_fields::PERSON_RECORDS`;
/// with an entry in the ruling body: the ban released, `exe::discipline_release_ban`, the slot free and entry `+0` =
/// −1), removes the person's held bans from list `discipline_fields::HELD_BANS` (`exe::llist_remove`, while count
/// `discipline_record::HELD_BAN_COUNT` > 0) and the record becomes `0xff` with count 0.
/// Original: `Discipline::remove_player` (0x556830, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_remove_player(this: usize, id: u32) {
    let r = read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add((id as usize).wrapping_mul(29));
    exe::discipline_clear_ban_history(this, read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    for k in 0..4usize {
        let rule = read_u8(r + k * 5);
        if rule == 0xff {
            continue;
        }
        let e = ruling_body_entry(ruling_body_of_rule(this, rule), id);
        if e == 0 {
            continue;
        }
        covered!("cpo_13 regen frees the slot");
        let i = read_i16(e + ruling_body_entry::BAN_INDEX);
        if i > -1 {
            // the index in ax; the high part of eax is that of the entry pointer (0x556d00 only reads the word)
            exe::discipline_release_ban(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
        }
        let v = read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add((id as usize).wrapping_mul(29) + k * 5);
        write_u8(v, 0xff);
        write_u32(v + ban_slot::VALUE, 0xffff_ffff);
        write_u32(e, 0xffff_ffff);
    }
    let list = Llist(this + discipline_fields::HELD_BANS);
    list.set_cursor((read_usize(this + discipline_fields::HELD_BANS_HEAD) as u32) as usize);
    let mut item = exe::llist_get_next(list);
    while item != 0 && *((r + discipline_record::HELD_BAN_COUNT) as *const i8) > 0 {
        if read_usize(item) as u32 == id {
            exe::llist_remove(list, item); // libera o item
            write_u8(r + discipline_record::HELD_BAN_COUNT, read_u8(r + discipline_record::HELD_BAN_COUNT).wrapping_sub(1));
        }
        item = exe::llist_get_next(list);
    }
    core::ptr::write_bytes(r as *mut u8, 0xff, 29);
    write_u8(r + discipline_record::HELD_BAN_COUNT, 0);
}

/// Returns the player's held ban in a slot, or 0 if none.
///
/// The held ban of player `id` in slot `slot`: it walks the list of held bans (`discipline_fields::HELD_BANS`, from the
/// start, `exe::llist_get_next`) while the player has any (byte `discipline_record::HELD_BAN_COUNT` of the 29-byte
/// record in `discipline_fields::PERSON_RECORDS`) and returns the one with the same player and slot (`held_ban::SLOT`).
/// Original: `Discipline::find_held_ban` (0x55cb60, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn find_held_ban(discipline: usize, id: i32, slot: i8) -> usize {
    let list = Llist(discipline + discipline_fields::HELD_BANS);
    let record = (read_usize(discipline + discipline_fields::PERSON_RECORDS) as isize + id as isize * 29) as usize;
    list.set_cursor(((list.head() as u32)) as usize);
    let mut p = exe::llist_get_next(list);
    while p != 0 {
        if read_i8(record + discipline_record::HELD_BAN_COUNT) <= 0 {
            return 0;
        }
        if read_i32(p) == id && *((p + held_ban::SLOT) as *const i8) == slot {
            return p;
        }
        p = exe::llist_get_next(list);
    }
    0
}

/// Writes the headline or the body of the news that a player was banned (types 0x7d2, and 0x7d3 through
/// `write_further_ban_news_text`).
///
/// `write_ban_news_text(news, full, out)`. The news item's slots: 0 the appeal made, 2 the count (matches; below 0 the
/// ban is in days), 3 the days, 4 and 5 the day and year the ban starts (year 0: no date), 6..9 the four rule sets
/// (`Discipline::get_scope_text`), 10 the kind (0 points, 1 yellow cards, 2 a sending-off, 3 misconduct, 4 yellow
/// cards in successive matches), 11 the number (points or cards; for kind 1, 1 = "a yellow card in" a match), 12 the
/// appeal pending, 14 "a further" ban; from slot 17 the person (`read_staff_name_ref_from_news`), then the match flag
/// and the match record (`read_match_record_from_news`, `BanMatch`). With a match, the competition is named when it is
/// not a reserve or minor competition (`exe::comp_is_reserve_or_minor_comp` of `G_RESERVE_COMP`) unless it is a league
/// division (`exe::comp_util_is_league_division`) in round 0x418, 0x419 or 0xa0 or with a stage word; the score link
/// holds when the fixture still matches (`exe::match_stats_fixture_entry_matches`). The person's links hold when his
/// names still match (`exe::database_staff_matches_names`).
///
/// The headline: "<name> banned for one match" (0x9c1230), "for <n> matches" (0x9c11e4) or, in days, "for <length>"
/// (0x9c1198, `format_duration_text`, 0x1e bytes), name in mode 5; the person's link.
///
/// The body (name in mode 1; the date text when there is a year; the score "<more>-<fewer>" with a match; the scope
/// text of form 4, whose gender is dropped):
/// - rule set 9 = 0 with an appeal pending, the assault on a referee: "banned for <days> days after his assault on a
///   referee in the <competition> match between" (0x9c1078, the competition's short name and gender) when named,
///   otherwise "in the match between" (0x9c0fa8); with a match and the club's goals ≥ 0 the sentence on how it ended
///   is appended (`ban_result_sentence`) with the score's link; then the person's and the two clubs' links (no
///   appeal note);
/// - otherwise the scope text again (form 5 for one match, else 6, its gender kept) and the template of the kind
///   (`BAN_*`: by the count, by "a further" ban, then by a start date, then plain), the arguments in the original's
///   order: the scope's gender twice, 0x64 for the date (twice for one match by points), the competition's and the
///   clubs' genders, the name, the count, the scope, the days, the date, then the number or the competition and the
///   clubs. A match kind not in days names the competition (long name) when it is named, with the sentence on how it
///   ended appended when there is a match; otherwise the club's result (won, drew, lost) picks the form with the
///   score. With the appeal made, "You have appealed..." (0x9b8f68) follows after a blank line. The links: the
///   person, the two clubs, the score.
///
/// Defects of the original (`DEFEITOS_DO_ORIGINAL`): without a match (the bans by points or by cards) the original
/// links the two clubs and the score of a match record it never read (uninitialised stack, `B+0x20..0x3f` and the
/// fixture flag `B+0xd8`): the copy leaves those three links out (its comparer drops them from the original's list);
/// a kind above 4 formats no template and prints the uninitialised buffer: the copy prints an empty text.
/// Original: `Discipline::write_ban_news_text` (0x5576e0, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_ban_news_text(this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let appealed = slot(0) as u8;
    slot(1);
    let count = slot(2) as u8 as i8;
    let days = slot(3) as u16 as i16 as i32;
    let start_day = slot(4);
    let start_year = slot(5);
    let rules = [slot(6) as u8, slot(7) as u8, slot(8) as u8, slot(9) as u8];
    let kind = slot(10) as u8;
    let number = slot(11) as u8 as i8 as i32;
    let pending = slot(12);
    slot(13);
    let further = slot(14) as u8;
    // the person (four ids and a birth date, from slot 17), then the match flag and the match record
    let mut person = [0u8; 0x18];
    person[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
    let mut m = BanMatch([0u8; 0x20]);
    m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
    let mut next: u8 = 17;
    exe::read_staff_name_ref_from_news(news, &mut next, person.as_mut_ptr());
    let pp = person.as_ptr() as usize;
    let has_match = slot(next as u32) != 0;
    next = next.wrapping_add(1);
    let mut named = false;
    if has_match {
        covered!("gra_16 ban with a match");
        exe::read_match_record_from_news(news, &mut next, m.0.as_mut_ptr());
        if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), m.comp()) == 0 {
            named = true;
        }
        if exe::comp_util_is_league_division(m.comp_index()) != 0 {
            let round = read_u16(m.at(0xc));
            if round == 0x418 || round == 0x419 || round == 0xa0 || read_u16(m.at(0xe)) != 0 {
                named = false;
            }
        }
    }
    let (first, second, common, id) = (read_u32(pp), read_u32(pp + 4), read_u32(pp + 8), read_u32(pp + 0xc));
    let staff = staff_at(id as i32 as isize);
    let person_ok = exe::database_staff_matches_names(staff, first, second, common, pp + 0x10) != 0;
    let fixture_ok = has_match
        && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), m.fixture() as i32, read_i32(m.at(0x11)),
            read_i32(m.at(0x15)), read_i32(m.at(0)), read_i32(m.at(2))) != 0;
    let mut name = [0u8; 0xc8];
    let np = name.as_ptr() as usize;
    if full == 0 {
        exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 5, id);
        if count > -1 {
            if count == 1 {
                fmt(NEWS_TEXT_BUFFER, 0x9c1230, np); // "<name> banned for one match"
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9c11e4, np, count as i32); // "<name> banned for <n> matches"
            }
        } else {
            let mut length = [0u8; 0x20];
            format_duration_text(days, length.as_mut_ptr() as usize, 0x1e, 0);
            fmt(NEWS_TEXT_BUFFER, 0x9c1198, np, length.as_ptr() as usize); // "<name> banned for <length>"
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if person_ok {
            exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
        }
        return;
    }
    covered!("gra_16 ban body");
    exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 1, id);
    let mut date_text = [0u8; 0x64];
    let dp = date_text.as_mut_ptr() as usize;
    if start_year as u16 != 0 {
        let mut date = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, start_day, start_year);
        exe::date_get_date_string_no_weekday(date.as_ptr() as usize, dp, 0x64, 0xff);
    }
    let mut score = [0u8; 0xc];
    let sp = score.as_mut_ptr() as usize;
    let (g1, g2) = m.goals();
    if has_match {
        fmt(sp, 0x9c116c, g1.max(g2) as i32, g1.min(g2) as i32); // "<more>-<fewer>"
    }
    let scope_text: unsafe extern "thiscall" fn(usize, *const u8, usize, u32, u32, u32) -> i8 = core::mem::transmute(FN_DISCIPLINE_GET_SCOPE_TEXT);
    let mut scope = [0u8; 0x64];
    let scp = scope.as_mut_ptr() as usize;
    scope_text(this, rules.as_ptr(), scp, 0x64, pending, 4);
    let mut text = [0u8; 0x2710];
    let tp = text.as_mut_ptr() as usize;
    let c1 = Club(m.club());
    let c2 = Club(m.opponent());
    let comp = m.comp();
    let link_clubs = |out: usize| {
        exe::text_add_link(out, (c1.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c1);
        exe::text_add_link(out, (c2.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c2);
    };
    if rules[3] == 0 && pending != 0 {
        covered!("gra_16 assault on a referee");
        let (gc1, gc2) = ((c1.short_gender_name() as i8) as i32, (c2.short_gender_name() as i8) as i32);
        let (n1, n2) = (c1.addr() + club::SHORT_NAME, c2.addr() + club::SHORT_NAME);
        if named {
            let gc = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
            fmt(tp, 0x9c1078, gc, gc, gc1, gc1, gc2, gc2, np, days, comp + competition::SHORT_NAME, n1, n2);
        } else {
            fmt(tp, 0x9c0fa8, gc1, gc1, gc2, gc2, np, days, n1, n2);
        }
        exe::text_set_string(out, tp as *const u8);
        if has_match && g1 > -1 {
            ban_result_sentence(tp, &m, sp);
            news_text_append(out, tp);
            if fixture_ok {
                exe::text_add_link(out, score.as_ptr(), 0x7d3, m.fixture() as usize);
            }
        }
        if person_ok {
            exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
        }
        if has_match {
            link_clubs(out);
        }
        return;
    }
    // the count: 0 = N matches, 1 = one match, 2 = days
    let (row, form) = if count < 0 { (2usize, 6) } else if count == 1 { (1, 5) } else { (0, 6) };
    let scope_gender = scope_text(this, rules.as_ptr(), scp, 0x64, pending, form);
    let variant = if further != 0 { 1 } else if start_year as u16 != 0 { 2 } else { 0 };
    let match_forms = match kind {
        1 if number == 1 => Some(&BAN_YELLOW_IN_MATCH),
        2 => Some(&BAN_SENT_OFF),
        3 => Some(&BAN_MISCONDUCT),
        _ => None,
    };
    let plain_forms = match kind {
        0 => Some(&BAN_POINTS),
        1 if number != 1 => Some(&BAN_YELLOWS),
        4 => Some(&BAN_SUCCESSIVE_YELLOWS),
        _ => None,
    };
    let competition_form = row != 2 && named;
    let template = if let Some(t) = plain_forms {
        Some(t[row][variant])
    } else if let Some(t) = match_forms {
        let result = if competition_form { 3 } else if g1 > g2 { 0 } else if g1 == g2 { 1 } else { 2 };
        Some(t[row][result][variant])
    } else {
        None
    };
    if let Some(template) = template {
        covered!("gra_16 ban template");
        let mut a = BanArgs::new();
        a.gender(scope_gender);
        if variant == 2 {
            a.push(0x64);
            if kind == 0 && row == 1 {
                a.push(0x64);
            }
        }
        if match_forms.is_some() {
            if competition_form {
                a.gender(read_i8(comp + competition::GENDER_NAME));
            }
            a.gender(c1.short_gender_name() as i8);
            a.gender(c2.short_gender_name() as i8);
        }
        a.push(np);
        if row == 0 {
            a.push(count as i32 as usize);
        }
        a.push(scp);
        if row == 2 {
            a.push(days as usize);
        }
        if variant == 2 {
            a.push(dp);
        }
        if match_forms.is_none() {
            a.push(number as usize);
        } else if competition_form {
            a.push(comp + competition::NAME);
            a.push(c1.addr() + club::SHORT_NAME);
            a.push(c2.addr() + club::SHORT_NAME);
        } else {
            a.push(c1.addr() + club::SHORT_NAME);
            a.push(sp);
            a.push(c2.addr() + club::SHORT_NAME);
        }
        let v = a.v;
        fmt(tp, template as usize, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]);
        // only the match forms name the competition and add how the match ended (the plain forms never do: the
        // original builds the sentence in the assault branch and the three match kinds, 0x557d19, 0x5584df, 0x559413,
        // 0x55a123; caught by the 8-league audit, 23/11/2026)
        if match_forms.is_some() && competition_form && has_match {
            covered!("gra_16 ban match result");
            let mut result = [0u8; 0x3e8];
            ban_result_sentence(result.as_mut_ptr() as usize, &m, sp);
            ban_strcat(tp, result.as_ptr() as usize);
        }
    }
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    if appealed != 0 {
        covered!("gra_16 ban appealed");
        let mut appeal = [0u8; 0x3d8];
        fmt(appeal.as_mut_ptr() as usize, 0x9b8f68); // "You have appealed against this decision..."
        crt_format(NEWS_TEXT_BUFFER, 0x991948, tp, appeal.as_ptr() as usize); // "%s\n\n%s"
    } else {
        crt_format(NEWS_TEXT_BUFFER, 0x991944, tp); // "%s"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if person_ok {
        exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
    }
    if has_match {
        link_clubs(out);
        if fixture_ok {
            exe::text_add_link(out, score.as_ptr(), 0x7d3, m.fixture() as usize);
        }
    }
}

/// Comparer for `write_ban_news_text`: the text side by side (`compare_text_writer`). The body of a ban without a match
/// is compared without the clubs' and the score's links, which the original takes from uninitialised stack.
pub(crate) unsafe extern "thiscall" fn compare_write_ban_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_BAN_NEWS_TEXT);
    let what = || format!("write_ban_news_text(news {:#x}, full {})", news, full);
    if full == 0 || exe::newsitem_get_slot(news, 23) != 0 {
        compare_text_writer(out, || o(this, news, full, out), |t| write_ban_news_text(this, news, full, t), what);
        return;
    }
    covered!("gra_16 ban without a match compared");
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, full, out),
        || crate::rng::without_counting(|| write_ban_news_text(this, news, full, lp)));
    let keep = |t: (Vec<u8>, Vec<(Vec<u8>, u32, u32)>)| (t.0, t.1.into_iter().filter(|l| l.1 != 0x7d5 && l.1 != 0x7d3).collect::<Vec<_>>());
    let (mo, mr) = (keep(text_contents(out)), keep(text_contents(lp)));
    exe::drop_text(local.as_mut_ptr());
    SCOREBOARD.record(same_rng && mo == mr, || format!("{}: original {:?} {:x?} rust {:?} {:x?}", what(),
        String::from_utf8_lossy(&mo.0), mo.1.len(), String::from_utf8_lossy(&mr.0), mr.1.len()));
}

/// Rule: the "Appeal" button of a ban news item opens the appeal confirm screen and copies the ban out of the news
/// item into its parameters.
///
/// A local `NewsItem` (`NewsItem::NewsItem`, 0, 0) and a staff name reference with its `Date` (`Date::Date` at +0x10)
/// are set up; the screen opens (`open_screen`: builder 0x55ea10, handler 0x55ed00, single instance, no leave/exit);
/// when it does not, nothing else. Otherwise the item `news_id` of `manager`'s inbox is fetched (`News::find_item_by_id`
/// on [0xb7431c], its answer not checked), its slots 0..14 read in order (`NewsItem::get_slot`), the slot index moved on
/// by two more, and the banned person read from the next slots (`read_staff_name_ref_from_news`, his staff id at +0xc).
/// Parameters: 0 slot 0 (signed byte), 1..3 the name ids (+0x33) of the person's first, second and common name records,
/// 4 his staff id, 5 slot 2 (signed byte), 6 slot 3 (signed word), 8 and 9 the low words of slots 4 and 5, 10..15
/// slots 6, 7, 8, 9, 10 and 11 (signed bytes), 0x10 slot 12, 0x11 the news id, 0x12 and 0x13 slots 13 and 14 (signed
/// bytes), 0x14 slot 1; parameter 7 is not written.
/// Original: `discipline_appeal_news_action` (0x55e620, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn discipline_appeal_news_action(manager: usize, news_id: u32) {
    let mut item = [0u8; 0xe0];
    let mut person = [0u8; 0x18];
    let mut slot: u8 = 0;
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    exe::date_date_54a110(person.as_mut_ptr() as usize + 0x10);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x55ea10usize, 0x55ed00usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel34_3 open appeal");
    exe::news_find_item_by_id(read_usize(0xb7431c), manager, news_id, (item.as_mut_ptr()) as *mut u8 as usize);
    let mut s = [0u32; 15];
    for v in s.iter_mut() {
        let k = slot;
        slot = slot.wrapping_add(1);
        *v = exe::newsitem_get_slot(item.as_ptr() as usize, k as u32);
    }
    slot = slot.wrapping_add(2);
    exe::read_staff_name_ref_from_news(item.as_ptr() as usize, &mut slot, person.as_mut_ptr());
    let staff = staff_at(read_u32(person.as_ptr() as usize + 0xc) as i32 as isize);
    let byte = |v: u32| v as u8 as i8 as isize as usize;
    set_screen_param(0, byte(s[0]));
    set_screen_param(1, read_usize(staff.first_name() + names::ID));
    set_screen_param(2, read_usize(staff.second_name() + names::ID));
    set_screen_param(3, read_usize(staff.common_name() + names::ID));
    set_screen_param(4, read_usize(staff.addr()));
    set_screen_param(5, byte(s[2]));
    set_screen_param(6, s[3] as u16 as i16 as isize as usize);
    set_screen_param(8, (s[4] & 0xffff) as usize);
    set_screen_param(9, (s[5] & 0xffff) as usize);
    for (k, v) in [(0xau32, s[6]), (0xb, s[7]), (0xc, s[8]), (0xd, s[9]), (0xe, s[10]), (0xf, s[11])] {
        set_screen_param(k, byte(v));
    }
    set_screen_param(0x10, s[12] as usize);
    set_screen_param(0x11, news_id as usize);
    set_screen_param(0x12, byte(s[13]));
    set_screen_param(0x13, byte(s[14]));
    set_screen_param(0x14, s[1] as usize);
}

/// Rule: the ban appeal confirm screen: the banned person's picture, the human manager's name in the club's colours,
/// "Please Confirm", the question (a game ban, a day ban or a plain ban) and the No and Yes buttons.
///
/// Parameters 1..3 are read and not used; the person = the staff record of parameter 4, the games = parameter 5
/// (signed byte), the days = parameter 6 (signed word); the club = his club (`staff::CLUB_JOB`) or, with none, the
/// first club of the table. The person's background (`background_set_staff_picture` 0x41c790, 0, 0); the menu bar
/// (4, 0); the title: the slot's human manager's name (2000 bytes) in the club's kit colours; the subtitle "Please
/// Confirm"; the person's name (500 bytes). Games > 0: the number as "%d" and, in Swedish (language 10) with one game,
/// the Swedish text (not translated, 0x9c36c0, with the name), otherwise "Appeal against the <n> game ban that has been
/// placed on <name>"; else days > 0: "... the <n> day ban ..."; else "Appeal against the ban that has been placed on
/// <name>" (the translated texts with their 100-char limits). The text in a text area (0x6e,0x91..0x30c,0x217); a
/// two-column area (weights 3/1) with "No" (id 1) and "Yes" (id 2).
/// Original: `discipline_appeal_dialog_build` (0x55ea10, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn discipline_appeal_dialog_build() {
    let crt: Tel34FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    let weights: [u8; 2] = [3, 1];
    let mut name = [0u8; 500];
    let mut number = [0u8; 500];
    covered!("tel34_3 appeal dialog");
    get_screen_param(1);
    get_screen_param(2);
    get_screen_param(3);
    let person = get_screen_param(4);
    let games = get_screen_param(5) as u8 as i8;
    let days = get_screen_param(6) as u16 as i16;
    let staff = staff_at(person as i32 as isize);
    let club = if staff.club_job().addr() != 0 { staff.club_job().addr() } else { read_usize(G_CLUB_TABLE) };
    (exe::background_set_staff_picture(staff, 0, 0) as u32);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::database_format_staff_name(tel26_slot_manager(), JOB_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, JOB_TEXT);
    tel34_lang_text(0x9909b8); // "Please Confirm"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::database_format_staff_name(staff, name.as_mut_ptr() as usize, 0x1f4, 1);
    let lang: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if games > 0 {
        crt(number.as_mut_ptr() as usize, 0x98806c, games as i32); // "%d"
        if exe::langlib_get_language() == 10 && games == 1 {
            covered!("tel34_3 appeal swedish one game");
            crt(JOB_TEXT, 0x9c36c0, name.as_ptr());
        } else {
            lang(JOB_TEXT, 0x9c364c, 100, 100, number.as_ptr(), name.as_ptr());
        }
    } else if days > 0 {
        crt(number.as_mut_ptr() as usize, 0x98806c, days as i32); // "%d"
        lang(JOB_TEXT, 0x9c35d8, 100, 100, number.as_ptr(), name.as_ptr());
    } else {
        lang(JOB_TEXT, 0x9c3584, 100, name.as_ptr());
    }
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    tel34_lang_text(0x98e8cc); // "No"
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    tel34_lang_text(0x98e8d0); // "Yes"
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
}

/// Rule: "No" (id 1) closes the appeal screen (-9); "Yes" (id 2) submits the appeal with the ban in the screen
/// parameters, marks the news item answered and closes (-9); other ids go to the menu bar.
///
/// The news id = parameter 0x11 (read first). Yes: the person = the staff record of parameter 4, the slot's human
/// manager; `MANAGER_MANAGER::submit_ban_appeal` (0x6ab800 on [0xb67a44], thiscall, 11 arguments) with the person,
/// parameter 0x12 (byte), then four dwords packed from the bytes of parameters 10..13 as the original's frame holds them
/// (10|11|12|13, 11|12|13, 12|13, 13; the bytes above them, stack leftovers in the original, zero here), parameter 0x13
/// (byte), parameter 5 (byte), parameter 6, the dword of the handler's own argument with its low byte replaced by
/// parameter 0xe, and parameter 0x14; then `News::set_item_slot` (manager, news id, slot 0, value 1, 0).
/// Original: `discipline_appeal_dialog_handle_click` (0x55ed00, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn discipline_appeal_dialog_handle_click(raw: u32) -> i32 {
    type SubmitBanAppeal = unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32, u32, usize, u32, usize) -> u8;
    let news_id = get_screen_param(0x11);
    match tel26_item_id(raw as i16) {
        1 => -9,
        2 => {
            covered!("tel34_3 appeal submitted");
            let person = staff_at(get_screen_param(4) as i32 as isize);
            let manager = tel26_slot_manager();
            let b = |k: u32| get_screen_param(k) as u8 as u32;
            let human = b(0x12);
            let (p10, p11, p12, p13) = (b(0xa), b(0xb), b(0xc), b(0xd));
            let p14 = b(0xe);
            let p19 = b(0x13);
            let p5 = b(5);
            let p6 = get_screen_param(6);
            let p20 = get_screen_param(0x14);
            let submit: SubmitBanAppeal = core::mem::transmute(FN_MANAGER_MANAGER_SUBMIT_BAN_APPEAL);
            submit(read_usize(0xb67a44), person.addr(), human, p10 | p11 << 8 | p12 << 16 | p13 << 24, p11 | p12 << 8 | p13 << 16, p12 | p13 << 8, p13,
                p19, p5, p6, (raw & 0xffff_ff00) | p14, p20);
            exe::news_set_item_slot(read_usize(0xb7431c), manager, news_id, 0, 1, 0);
            -9
        }
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Comparer for `discipline_appeal_news_action`.
pub(crate) unsafe extern "C" fn compare_discipline_appeal_news_action(manager: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_DISCIPLINE_APPEAL_NEWS_ACTION)(manager, news_id)
}

/// Comparer for `discipline_appeal_dialog_build`: the appeal screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_discipline_appeal_dialog_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DISCIPLINE_APPEAL_DIALOG_BUILD);
    compare_by_display_list_with("Ban appeal screen (discipline_appeal_dialog_build)", &[], || o(), || discipline_appeal_dialog_build());
}

/// Comparer for `discipline_appeal_dialog_handle_click`.
pub(crate) unsafe extern "C" fn compare_discipline_appeal_dialog_handle_click(raw: u32) -> i32 {
    // simplification: the appeal and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_DISCIPLINE_APPEAL_DIALOG_HANDLE_CLICK)(raw)
}

/// Allocates the 43-entry ruling body table and constructs one ruling body object per active rule set (0x23 bytes;
/// the English one 0x2b).
///
/// `Discipline::create_rule_sets(arg)`: the table (`exe::crt_calloc` of 0x2b dwords) in the discipline object's first
/// dword; failing, the error box (discipline.cpp line 0x161e) and nothing else. Then, in `RULE_SETS` order, each
/// ruling body whose condition holds: a nation's by its league being selected (the `nation::LEAGUE_SELECTED` byte of
/// the nation the key names), Oceania's by its continent (0x6699d0 on `G_CONTINENT_TABLE` + key × 0xc6), the last two
/// (the South Korean one also by the nation) with the constructor's argument 0 when the absolute data version
/// (`G_GAME_DATA_VERSION`) is below 0x3e044 / 0x3e047, otherwise `arg`; each one `exe::crt_operator_new(0x23)` (0x2b for slot 0x24, England) and
/// its constructor (null when the allocation fails), stored in its slot.
/// Original: `Discipline::create_rule_sets` (0x55d420, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_create_rule_sets(this: usize, arg: u32) {
    let t = exe::crt_calloc(0x2b, 4);
    write_usize(this, t);
    if t == 0 {
        show_error_at(0x9b8f18, 0x161e); // "E:\dev\CM3\cm3 00-01\cm3\code\discipline.cpp"
        return;
    }
    let selected = |key: usize| read_u8(read_usize(G_NATION_TABLE) + read_u32(key) as usize * 0x122 + nation::LEAGUE_SELECTED) != 0;
    let old = |v: i32| read_i32(G_GAME_DATA_VERSION).wrapping_abs() < v;
    for (slot, when, ctor) in RULE_SETS {
        let a = match when {
            RuleSetWhen::Always => arg,
            RuleSetWhen::Nation(k) => {
                if !selected(k) {
                    continue;
                }
                arg
            }
            RuleSetWhen::Continent(k) => {
                let active: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(CONTINENT_IS_ACTIVE);
                if active(read_usize(G_CONTINENT_TABLE) + read_u32(k) as usize * 0xc6) == 0 {
                    continue;
                }
                arg
            }
            RuleSetWhen::OlderThan(v) => if old(v) { 0 } else { arg },
            RuleSetWhen::NationOlderThan(k, v) => {
                if !selected(k) {
                    continue;
                }
                if old(v) { 0 } else { arg }
            }
        };
        if slot == 0x24 {
            covered!("sma_5 english rule set");
        }
        let a = if slot == 0x24 { a } else { a }; // sma5: the English rule set's argument (the mutation's anchor)
        // the English ruling body takes 0x2b bytes (0x55d733): RbEngland keeps a date at +0x23..+0x2a; with 0x23 its
        // constructor wrote 8 bytes past the block (24/11/2026; found by the reader of sx18)
        let o = exe::crt_operator_new(if slot == 0x24 { 0x2b } else { 0x23 });
        let v = if o != 0 {
            let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(ctor);
            f(o, a)
        } else {
            0
        };
        write_usize(read_usize(this) + slot, v);
    }
}

/// Comparer for `discipline_create_rule_sets`.
///
/// The ruling bodies are built once per game: only the original runs, and the table it leaves is checked: one object
/// exactly in the slots whose condition holds, none in the others.
pub(crate) unsafe extern "thiscall" fn compare_discipline_create_rule_sets(this: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DISCIPLINE_CREATE_RULE_SETS)(this, arg);
    let t = read_usize(this);
    if t == 0 {
        return;
    }
    let selected = |key: usize| read_u8(read_usize(G_NATION_TABLE) + read_u32(key) as usize * 0x122 + nation::LEAGUE_SELECTED) != 0;
    let mut wrong = Vec::new();
    for (slot, when, _) in RULE_SETS {
        let built = match when {
            RuleSetWhen::Nation(k) | RuleSetWhen::NationOlderThan(k, _) => selected(k),
            RuleSetWhen::Continent(_) => continue, // the continent test is not repeated here
            _ => true,
        };
        if built != (read_usize(t + slot) != 0) {
            wrong.push(slot);
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("discipline_create_rule_sets: slots {:x?} against the selected leagues", wrong));
}

/// `strcat(dst, src)` on two NUL-terminated buffers (the original's inline `repne scasb` / `rep movs`).
pub(crate) unsafe fn ban_strcat(dst: usize, src: usize) {
    let mut d = dst;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut s = src;
    loop {
        let c = read_u8(s);
        write_u8(d, c);
        if c == 0 {
            break;
        }
        d += 1;
        s += 1;
    }
}

/// The sentence on how the match ended, from the player's club's goals `g1` and the opponent's `g2` (`d` = g1 − g2,
/// `s` = g1 + g2): won by more than 5 "ran riot with a <score> victory" (0x9c0f50), by more than 3 "ran out easy
/// winners" (0x9c0efc), otherwise won "won a thrilling game" (s > 5, 0x9c0ea8) or "won the match" (0x9c0e5c); 0-0 "a
/// stalemate" (0x9c0da8); another draw "a thrilling draw" (s > 5, 0x9c0e1c) or "a draw" (0x9c0de4); lost by more
/// than 5 "a humiliating defeat" (0x9c0d30), by more than 3 "were easily beaten" (0x9c0cdc), otherwise "were beaten in
/// a cracking match" (s > 5, 0x9c0c7c) or "were beaten" (0x9c0c30). The club forms take the club's short name and
/// gender; the draws only the score.
pub(crate) unsafe fn ban_result_sentence(out: usize, m: &BanMatch, score: usize) {
    let fmt = news_text_format();
    let (g1, g2) = m.goals();
    let (g1, g2) = (g1 as i32, g2 as i32);
    let (d, s) = (g1 - g2, g1 + g2);
    let c = Club(m.club());
    let g = (c.short_gender_name() as i8) as i32;
    let name = c.addr() + club::SHORT_NAME;
    let team = if d > 5 {
        0x9c0f50
    } else if d > 3 {
        0x9c0efc
    } else if g1 > g2 {
        if s > 5 { 0x9c0ea8 } else { 0x9c0e5c }
    } else if g1 == g2 {
        let draw = if g1 == 0 { 0x9c0da8 } else if s > 5 { 0x9c0e1c } else { 0x9c0de4 };
        fmt(out, draw, score);
        return;
    } else if d < -5 {
        0x9c0d30
    } else if d < -3 {
        0x9c0cdc
    } else if s > 5 {
        0x9c0c7c
    } else {
        0x9c0c30
    };
    fmt(out, team, g, g, name, score);
}

/// `Discipline::get_scope_text(rules, out, size, appeal_pending, form)` (0x55ce40, discipline.cpp; thiscall, ret 0x14):
/// the scope of a ban ("European", "domestic"...) from the four rule-set bytes into `out`; returns the scope's
/// grammatical gender.
pub(crate) const FN_DISCIPLINE_GET_SCOPE_TEXT: usize = 0x55ce40;

/// The ban templates `[plain, a further ban, starting on a date]`, by the count of the ban: `[N matches, one match,
/// days]` (a count below 0 is a ban in days). Points: "after collecting <n> points".
pub(crate) const BAN_POINTS: [[u32; 3]; 3] = [
    [0x9c08e8, 0x9c098c, 0x9c080c],
    [0x9c0b08, 0x9c0b98, 0x9c0a3c],
    [0x9bb388, 0x9bb438, 0x9bb2a0],
];

/// "after collecting <n> yellow cards in successive matches".
pub(crate) const BAN_SUCCESSIVE_YELLOWS: [[u32; 3]; 3] = [
    [0x9be6a8, 0x9be768, 0x9be5b0],
    [0x9be918, 0x9be9c4, 0x9be834],
    [0x9ba49c, 0x9ba568, 0x9ba398],
];

/// "after collecting <n> yellow cards".
pub(crate) const BAN_YELLOWS: [[u32; 3]; 3] = [
    [0x9beb58, 0x9bec04, 0x9bea78],
    [0x9bed88, 0x9bee20, 0x9becb8], // gra16: one match
    [0x9ba728, 0x9ba7dc, 0x9ba63c],
];

/// The bans that come from a match, `[count][the player's club won, drew, lost, the competition named][form]`: the
/// result forms name the score ("<club>'s 2-1 win over <club>"), the competition form names the competition and the
/// two clubs (never for a ban in days). "after collecting a yellow card in".
pub(crate) const BAN_YELLOW_IN_MATCH: [[[u32; 3]; 4]; 3] = [
    [[0x9bfc50, 0x9bfd4c, 0x9bfb1c], [0x9beff0, 0x9bf0ec, 0x9beebc], [0x9bf620, 0x9bf71c, 0x9bf4ec], [0x9c0294, 0x9c03a8, 0x9c0148]],
    [[0x9bff70, 0x9c0058, 0x9bfe50], [0x9bf314, 0x9bf3fc, 0x9bf1f4], [0x9bf944, 0x9bfa2c, 0x9bf824], [0x9c0600, 0x9c0704, 0x9c04c8]],
    [[0x9bb088, 0x9bb190, 0x9baf4c], [0x9ba9dc, 0x9baae4, 0x9ba89c], [0x9bad34, 0x9bae3c, 0x9babf4], [0, 0, 0]],
];

/// "after being sent off in" (a ban in days: "after collecting a red card in").
pub(crate) const BAN_SENT_OFF: [[[u32; 3]; 4]; 3] = [
    [[0x9bda60, 0x9bdb50, 0x9bd938], [0x9bce78, 0x9bcf6c, 0x9bcd50], [0x9bd46c, 0x9bd560, 0x9bd344], [0x9be068, 0x9be174, 0x9bdf24]],
    [[0x9bdd60, 0x9bde40, 0x9bdc4c], [0x9bd180, 0x9bd260, 0x9bd068], [0x9bd774, 0x9bd854, 0x9bd65c], [0x9be3b8, 0x9be4b0, 0x9be288]],
    [[0x9ba188, 0x9ba28c, 0x9ba04c], [0x9b9ae8, 0x9b9bec, 0x9b99ac], [0x9b9e38, 0x9b9f3c, 0x9b9cfc], [0, 0, 0]],
];

/// "after his misconduct in".
pub(crate) const BAN_MISCONDUCT: [[[u32; 3]; 4]; 3] = [
    [[0x9bc200, 0x9bc2f0, 0x9bc0d8], [0x9bb618, 0x9bb70c, 0x9bb4f0], [0x9bbc0c, 0x9bbd00, 0x9bbae4], [0x9bc808, 0x9bc914, 0x9bc6c4]],
    [[0x9bc500, 0x9bc5e0, 0x9bc3ec], [0x9bb920, 0x9bba00, 0x9bb808], [0x9bbf14, 0x9bbff4, 0x9bbdfc], [0x9bcb58, 0x9bcc50, 0x9bca28]],
    [[0x9b97a8, 0x9b98a4, 0x9b9674], [0x9b9138, 0x9b9234, 0x9b9004], [0x9b9470, 0x9b956c, 0x9b933c], [0, 0, 0]],
];

/// The match record a ban news item carries (`read_match_record_from_news`, 0x1f bytes): the date (+0 day, +2 year),
/// the club competition (+8), the round and stage words (+0xc, +0xe), the player's club (+0x11) and its opponent
/// (+0x15), their goals (+0x19, +0x1a) and the fixture index (+0x1b).
pub(crate) struct BanMatch(pub(crate) [u8; 0x20]);

impl BanMatch {
    pub(crate) fn at(&self, k: usize) -> usize {
        self.0.as_ptr() as usize + k
    }
    pub(crate) unsafe fn comp_index(&self) -> i32 {
        read_i32(self.at(8))
    }
    pub(crate) unsafe fn club(&self) -> usize {
        club_at(read_i32(self.at(0x11)) as isize).addr()
    }
    pub(crate) unsafe fn opponent(&self) -> usize {
        club_at(read_i32(self.at(0x15)) as isize).addr()
    }
    pub(crate) unsafe fn goals(&self) -> (i8, i8) {
        (read_i8(self.at(0x19)), read_i8(self.at(0x1a)))
    }
    pub(crate) unsafe fn fixture(&self) -> u32 {
        read_u32(self.at(0x1b))
    }
    /// The club competition's record (`G_CLUB_COMP_TABLE`, 0x6b bytes; its names where `competition::` has them).
    pub(crate) unsafe fn comp(&self) -> usize {
        read_usize(G_CLUB_COMP_TABLE).wrapping_add((self.comp_index() as usize).wrapping_mul(0x6b))
    }
}

/// The date `Date::Date` (0x54a110) leaves before the readers overwrite it: day 0x28 of 1972 (0x7b4), flag 1.
pub(crate) const BAN_DEFAULT_DATE: [u8; 8] = [0x28, 0, 0xb4, 0x07, 1, 0, 0, 0];

/// The arguments of one template, in the order the original pushes them (langlib_sprintf is variadic: the unused
/// tail of the array is passed and never read).
pub(crate) struct BanArgs {
    pub(crate) v: [usize; 16],
    pub(crate) n: usize,
}

impl BanArgs {
    pub(crate) fn new() -> Self {
        BanArgs { v: [0; 16], n: 0 }
    }
    pub(crate) fn push(&mut self, x: usize) {
        self.v[self.n] = x;
        self.n += 1;
    }
    /// A gender byte, sign-extended as the original pushes it, twice.
    pub(crate) fn gender(&mut self, g: i8) {
        self.push(g as i32 as usize);
        self.push(g as i32 as usize);
    }
}

/// The 43 ruling bodies of `Discipline::create_rule_sets` in the original's order: the slot (byte offset in the
/// table), when it is built and its constructor (thiscall(object, argument), each 0x23 bytes).
pub(crate) const RULE_SETS: [(usize, RuleSetWhen, usize); 43] = {
    use RuleSetWhen::*;
    [(0x00, Always, 0x7df180), (0x04, Always, 0x7dd400), (0x08, Always, 0x7e29e0),
     (0x0c, Nation(KEY_NATION_ARGENTINA), 0x7db100),
     (0x10, Nation(KEY_NATION_BELGIUM), 0x7dbca0), (0x14, Nation(KEY_NATION_BELGIUM), 0x7db8f0),
     (0x18, Nation(KEY_NATION_BRAZIL), 0x7dc080), (0x1c, Nation(KEY_NATION_BRAZIL), 0x7dc3c0),
     (0x20, Nation(KEY_NATION_DENMARK), 0x7dcae0),
     (0x24, Nation(KEY_NATION_ENGLAND), 0x7dcf90),
     (0x28, Nation(KEY_NATION_FINLAND), 0x7dd990), (0x2c, Nation(KEY_NATION_FINLAND), 0x7dd660),
     (0x30, Nation(KEY_NATION_FRANCE), 0x7ddcc0),
     (0x34, Nation(KEY_NATION_GERMANY), 0x7de620), (0x38, Nation(KEY_NATION_GERMANY), 0x7de2e0),
     (0x3c, Nation(KEY_NATION_HOLLAND), 0x7ded90),
     (0x40, Nation(KEY_NATION_REPUBLIC_OF_IRELAND), 0x7df480),
     (0x44, Nation(KEY_NATION_ITALY), 0x7dfd70), (0x48, Nation(KEY_NATION_ITALY), 0x7df960),
     (0x4c, Nation(KEY_NATION_JAPAN), 0x7e0490), (0x50, Nation(KEY_NATION_JAPAN), 0x7e01b0),
     (0x54, Nation(KEY_NATION_NORWAY), 0x7e1150), (0x58, Nation(KEY_NATION_NORWAY), 0x7e0ef0),
     (0x5c, Nation(KEY_NATION_PORTUGAL), 0x7e1a60),
     (0x60, Nation(KEY_NATION_SCOTLAND), 0x7e2470), (0x64, Nation(KEY_NATION_SCOTLAND), 0x7e21a0),
     (0x68, Nation(KEY_NATION_SPAIN), 0x7e2f80), (0x6c, Nation(KEY_NATION_SPAIN), 0x7e2c30),
     (0x70, Nation(KEY_NATION_SWEDEN), 0x7e3650), (0x74, Nation(KEY_NATION_SWEDEN), 0x7e32d0),
     (0x78, Nation(KEY_NATION_UNITED_STATES), 0x7e40f0),
     (0x7c, Nation(KEY_NATION_POLAND), 0x7e1610),
     (0x80, Nation(KEY_NATION_GREECE), 0x7de960),
     (0x84, Nation(KEY_NATION_CROATIA), 0x7dc700),
     (0x88, Nation(KEY_NATION_AUSTRALIA), 0x7db5b0),
     (0x8c, Continent(KEY_CONTINENT_OCEANIA), 0x7e13d0),
     (0x90, Nation(KEY_NATION_NORTHERN_IRELAND), 0x7e0af0),
     (0x94, Nation(KEY_NATION_TURKEY), 0x7e3d60), (0x98, Nation(KEY_NATION_TURKEY), 0x7e3a10),
     (0x9c, Nation(KEY_NATION_RUSSIA), 0x7e1e00),
     (0xa0, Nation(KEY_NATION_WALES), 0x7e43c0),
     (0xa4, OlderThan(0x3e044), 0x7db360),
     (0xa8, NationOlderThan(KEY_NATION_SOUTH_KOREA, 0x3e047), 0x7e0730)]
};

/// The continent test of the Oceania rule set (0x6699d0, cdecl(continent) -> nonzero).
pub(crate) const CONTINENT_IS_ACTIVE: usize = 0x6699d0;

/// When a ruling body of the table is built.
#[derive(Clone, Copy)]
pub(crate) enum RuleSetWhen {
    /// always
    Always,
    /// when the league of the nation (the `KEY_NATION_*` global) is selected (`nation::LEAGUE_SELECTED`)
    Nation(usize),
    /// when the continent (the `KEY_CONTINENT_*` global) passes 0x6699d0
    Continent(usize),
    /// always, the constructor's argument 0 for data older than the version (`G_GAME_DATA_VERSION`, absolute value)
    OlderThan(i32),
    /// when the nation is selected, and the argument 0 for data older than the version
    NationOlderThan(usize, i32),
}

/// Posts news item 2002 (0x7d2) that a person got a ban, and picks who receives it.
///
/// `post_ban_news(person, ban, fixture)` (the Discipline object in ecx is not read; SEH frame for the local news item
/// and records, no destructor). First, with the transfer manager, the clubs it picks for the player's news
/// (`exe::transfer_manager_collect_player_news_recipients` with 0xda, up to 30; none without it). The slots, in order:
/// 0 = 0 (the appeal made, set later by the appeal); the fixture's `.referee()` (−1 without a fixture); the ban's
/// `.matches()` (signed byte) and `.days()`; the start date (day signed, year unsigned) when it is still to come (today
/// `G_CURRENT_DATE` before it: an earlier year, or the same year and an earlier day), otherwise 0 and 0; the four rules
/// (`ban_record::RULES`, signed bytes); `.reason()` (signed) and `.reason_amount()`; bit 1 of `.flags()` (valid in
/// every competition), bit 2 (extended by a pending ban) and bit 0 (added to an existing ban); today's day (signed) and
/// year. Then the person's reference (`exe::fill_staff_name_ref` into a record whose date starts as `Date::Date`,
/// written by `exe::write_staff_name_ref_to_news`). With a fixture: slot 1, and the person's club in the match, the
/// `.national_job()` when the home side is a national team (`exe::database_is_national_team`), otherwise the
/// `.club_job()`; when that club is neither the home nor the away club, the routine ends there and nothing is sent;
/// otherwise its match record (`exe::fill_match_record_from_fixture`, written by `exe::write_match_record_to_news`).
/// Without a fixture: slot 0. Who gets it: a ban of more than 50 days (`.days()`, signed; a ban by matches has −1)
/// goes to every human manager (`exe::news_send_to_all_managers`) when the player's `.world_reputation()` is above
/// 8000, otherwise to the clubs picked at the start; a shorter ban only to the person's club when it has a human
/// manager (`exe::news_club_has_human_manager`).
/// Original: `Discipline::post_ban_news` (0x557280, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_ban_news(_this: usize, person: usize, ban: usize, fixture: usize) {
    let person = Staff(person);
    let ban = BanRecord(ban);
    let fixture = Fixture(fixture);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let mut recipients = [0usize; 0x1e];
    let count = if tm != 0 { exe::transfer_manager_collect_player_news_recipients(tm, person, 0xda, recipients.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x7d2, 0);
    let mut name = [0u8; 0x18]; // the person's reference: four name ids and the birth date
    name[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
    exe::fill_staff_name_ref(person, name.as_mut_ptr() as usize);
    let mut k = 0u8;
    ban_news_slot(n, &mut k, 0); // sx09b_1: the appeal made
    ban_news_slot(n, &mut k, if fixture.is_null() { u32::MAX } else { fixture.referee() as i32 as u32 });
    ban_news_slot(n, &mut k, ban.matches() as i8 as i32 as u32); // sx09b_1: the matches
    ban_news_slot(n, &mut k, ban.days() as i32 as u32);
    let today_day = read_i16(G_CURRENT_DATE);
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    let start_day = ban.start_date();
    let start_year = read_u16(ban.addr() + ban_record::START_DATE + 2); // the start date's year
    if today_year < start_year || (today_year == start_year && today_day < start_day) { // sx09b_1: the start still to come
        covered!("sx09b_1 ban news with a later start");
        ban_news_slot(n, &mut k, start_day as i32 as u32);
        ban_news_slot(n, &mut k, start_year as u32);
    } else {
        ban_news_slot(n, &mut k, 0);
        ban_news_slot(n, &mut k, 0);
    }
    for j in 0..4 {
        ban_news_slot(n, &mut k, read_i8(ban.addr() + ban_record::RULES + j) as i32 as u32);
    }
    ban_news_slot(n, &mut k, ban.reason() as i8 as i32 as u32);
    ban_news_slot(n, &mut k, ban.reason_amount() as i32 as u32);
    let flags = ban.flags() as u32;
    ban_news_slot(n, &mut k, (flags >> 1) & 1);
    ban_news_slot(n, &mut k, (flags >> 2) & 1);
    ban_news_slot(n, &mut k, flags & 1);
    ban_news_slot(n, &mut k, today_day as i32 as u32);
    ban_news_slot(n, &mut k, today_year as u32);
    exe::write_staff_name_ref_to_news(name.as_mut_ptr() as usize, n, &mut k);
    if fixture.is_null() {
        ban_news_slot(n, &mut k, 0);
    } else {
        ban_news_slot(n, &mut k, 1);
        let mut m = BanMatch([0u8; 0x20]);
        m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
        let club = if exe::database_is_national_team(fixture.home_club()) != 0 { person.national_job() } else { person.club_job() };
        if club != fixture.home_club() && club != fixture.away_club() {
            covered!("sx09b_1 ban news of a club not in the match");
            return;
        }
        exe::fill_match_record_from_fixture(fixture, club, m.0.as_mut_ptr() as usize);
        exe::write_match_record_to_news(m.0.as_mut_ptr() as usize, n, &mut k);
    }
    if ban.days() > 0x32 {
        let sheet = person.player();
        if !sheet.is_null() && sheet.world_reputation() > 0x1f40 {
            covered!("sx09b_1 ban news to every manager");
            ban_news_send(n, Club(0));
        } else {
            covered!("sx09b_1 ban news to the picked clubs");
            for &c in recipients.iter().take(count.max(0) as usize) {
                ban_news_send(n, Club(c));
            }
        }
    } else {
        let club = person.club_job();
        if !club.is_null() && exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
            covered!("sx09b_1 ban news to the club");
            ban_news_send(n, club); // sx09b_1: the club gets it
        }
    }
}

/// Comparer for `post_ban_news`, as a dry run: the copy runs first without sending (it keeps each send), then the
/// original from the same random generator state. Compares the generator's state and the ban news items that reached
/// the human managers' inboxes with the copy's sends: one per club with a human manager, and for the send to every
/// manager one per human manager (the last 16 people with `.classification()` 1, as `News::send_to_all_managers`).
pub(crate) unsafe extern "thiscall" fn compare_post_ban_news(this: usize, person: usize, ban: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_POST_BAN_NEWS);
    let inboxes = human_inbox_marks();
    let rng = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).clear();
    BAN_NEWS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| post_ban_news(this, person, ban, fixture)));
    BAN_NEWS_DRY_RUN.store(false, Relaxed);
    let rng_rust = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let sent = core::mem::take(&mut *BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()));
    rng.write_back();
    o(this, person, ban, fixture);
    let rng_original = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let staff_count = read_i32(G_STAFF_COUNT);
    let humans = (0..0x10).filter(|&i| staff_at((staff_count - 0x10 + i) as isize).classification() == 1).count();
    let mut expected: Vec<Vec<u8>> = Vec::new();
    for (c, b) in &sent {
        let mut b = b.clone();
        b[news_item::IMPORTANT] = 0;
        let copies = if *c == 0 {
            humans
        } else {
            let m = Club(*c).manager();
            (!m.is_null() && read_i32(m.addr()) >= staff_count.wrapping_sub(0x10)) as usize
        };
        expected.extend(core::iter::repeat(b).take(copies));
    }
    expected.sort();
    let delivered = delivered_news_items(&inboxes, 0x7d2);
    let same = rng_original == rng_rust && (inboxes.is_empty() || delivered == expected);
    SCOREBOARD.record(same, || {
        format!(
            "post_ban_news(person {:#x}, ban {:#x}, fixture {:#x}) rng same {}; {} sends, human items original {:x?} rust {:x?}",
            person,
            ban,
            fixture,
            rng_original == rng_rust,
            sent.len(),
            delivered,
            expected
        )
    });
}

/// Orders two items of the discipline nation index (`discipline_fields::NATION_INDEX`, 8 bytes: +0 the person id,
/// `nation_index_item::NATION_ID`), for qsort and bsearch: by the nation id (the difference, when they differ), then
/// by the person id; a person id of −1 on either side (the key that `nation_index_range` searches with) matches any
/// person of the nation (0).
/// Original: `compare_nation_index_entries` (0x55e5f0, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_nation_index_entries(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (nation_a, nation_b) = (read_i32(a + nation_index_item::NATION_ID), read_i32(b + nation_index_item::NATION_ID));
    if nation_a != nation_b {
        return nation_a.wrapping_sub(nation_b);
    }
    let (id_a, id_b) = (read_i32(a), read_i32(b));
    if id_a == -1 || id_b == -1 { // sx09b_2: the key matches any person
        covered!("sx09b_2 nation index key");
        return 0;
    }
    id_a.wrapping_sub(id_b)
}

/// Writes which ruling bodies a ban covers and returns the grammatical form of the text (the `{}` codes of the
/// templates).
///
/// `get_scope_text(rules, out, size, everywhere, form)`: `rules` are the ban's four rules (`ban_record::RULES`, one per
/// person record slot; 0xff none). A `size` below 70 gives the "Error" box (line 0x1475) and returns 0xff. Three flags
/// start set: world-wide, continental and domestic; the first rule present (in slot order) is the one that speaks.
/// Slot 0 or slot 1 missing clears all three; slot 2 present clears domestic, missing clears world-wide and
/// continental. With slot 3 (the FIFA's) present: `everywhere` set gives "global" and returns 0xff; otherwise "world-wide"
/// when that flag survived, else the speaking rule's own text. Without slot 3: domestic and not continental gives
/// "domestic"; continental gives "continental"; neither, the speaking rule's own text (its vtable slot 0x10 with `out`,
/// `size` and `form`). The form returned is the speaking rule's: by its record slot (`ruling_body::RECORD_SLOT`) 1 →
/// 9, 3 → 1, 0 → 8 when the byte +0x20 is not 0xff (else 0); otherwise by its index (`ruling_body::RULE_SET_INDEX`)
/// 1 → 2, 2 → 6, 35 → 7, 41 → 3, any other → 0xff.
/// Original: `Discipline::get_scope_text` (0x55ce40, discipline.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_scope_text(this: usize, rules: usize, out: usize, size: i32, everywhere: u32, form: u32) -> u8 {
    if size < 0x46 {
        show_error_at(0x9b8f18, 0x1475);
        return 0xff;
    }
    let rule = |k: usize| read_u8(rules + k);
    let (mut world, mut continental, mut domestic) = (true, true, true);
    let mut speaker = 0xffu8;
    for k in 0..2 {
        if rule(k) == 0xff {
            (world, continental, domestic) = (false, false, false);
        } else if speaker == 0xff {
            speaker = rule(k);
        }
    }
    if rule(2) != 0xff {
        if speaker == 0xff {
            speaker = rule(2);
        }
        domestic = false;
    } else {
        (world, continental) = (false, false);
    }
    let body = |s: u8| read_usize(read_usize(this).wrapping_add((s as i8 as isize as usize).wrapping_mul(4)));
    let own_text = |s: u8| {
        let f: unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(body(s)) + 0x10)); // vtable +0x10: the rule's scope text
        f(body(s), out, size, form);
    };
    if rule(3) != 0xff {
        if everywhere != 0 {
            covered!("sx09b_3 global");
            story_lang!(out, 0x9c3478); // "global"
            return 0xff;
        }
        if speaker == 0xff {
            speaker = rule(3);
        }
        if world {
            covered!("sx09b_3 world-wide");
            story_lang!(out, 0x9c346c); // "world-wide"
        } else {
            own_text(speaker);
        }
    } else if domestic && !continental {
        covered!("sx09b_3 domestic");
        story_lang!(out, 0x9c3454); // "domestic"
    } else if continental {
        covered!("sx09b_3 continental");
        story_lang!(out, 0x9c3460); // "continental"
    } else {
        covered!("sx09b_3 the rule's own text");
        own_text(speaker);
    }
    let b = body(speaker);
    match read_u8(b + ruling_body::RECORD_SLOT) {
        1 => 9,
        0 => if read_u8(b + 0x20) != 0xff { 8 } else { 0 }, // sx09b_3: +0x20 of the ruling body (unnamed byte)
        3 => 1,
        _ => match read_u8(b + ruling_body::RULE_SET_INDEX) as i8 {
            1 => 2,
            2 => 6,
            35 => 7,
            41 => 3,
            _ => 0xff,
        },
    }
}

/// Comparer for `get_scope_text`: side by side over the output buffer and the form returned. A short buffer (the
/// "Error" box) runs the original only.
pub(crate) unsafe extern "thiscall" fn compare_get_scope_text(this: usize, rules: usize, out: usize, size: i32, everywhere: u32, form: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32) -> u8>(IDX_GET_SCOPE_TEXT);
    if size < 0x46 {
        return o(this, rules, out, size, everywhere, form);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, size as usize)], || o(this, rules, out, size, everywhere, form), || get_scope_text(this, rules, out, size, everywhere, form));
    SCOREBOARD.record(same && r == n, || {
        format!(
            "get_scope_text(rules {:02x?}, everywhere {}, form {}) original={} rust={} {:?}",
            (0..4).map(|k| read_u8(rules + k)).collect::<Vec<_>>(),
            everywhere,
            form,
            r,
            n,
            first_differences(&mo, &mr)
        )
    });
    r
}

/// Sends the ban news item to a club's manager (`exe::news_send_to_club_manager`), or to every human manager when
/// `club` is null (`exe::news_send_to_all_managers`); in the dry run, keeps it.
pub(crate) unsafe fn ban_news_send(n: usize, club: Club) {
    if BAN_NEWS_DRY_RUN.load(Relaxed) {
        let bytes = (0..news_item::ID).map(|k| read_u8(n + k)).collect();
        BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).push((club.addr(), bytes));
        return;
    }
    if club.is_null() {
        exe::news_send_to_all_managers(read_usize(G_NEWS), n);
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
}

/// Writes value `v` into slot `*k` of news item `n` and moves to the next slot (the original's byte counter).
pub(crate) unsafe fn ban_news_slot(n: usize, k: &mut u8, v: u32) {
    exe::newsitem_set_slot(n, *k as i8, v);
    *k = k.wrapping_add(1);
}

/// The human inboxes' ring positions (entry address, ring start, next slot), or none without the news object.
pub(crate) unsafe fn human_inbox_marks() -> Vec<(usize, i32, i32)> {
    let news = read_usize(G_NEWS);
    if news == 0 || read_usize(news + news_fields::HUMAN_INBOXES) == 0 {
        return Vec::new();
    }
    (0..0x10).map(|i| read_usize(news + news_fields::HUMAN_INBOXES) + i * 12).map(|e| (e, read_i32(e), read_i32(e + 4))).collect()
}

/// The news items of type `kind` that reached the human inboxes since `before` (`human_inbox_marks`): the first
/// `news_item::ID` bytes of each, without `news_item::IMPORTANT` (the delivery sets it), sorted.
pub(crate) unsafe fn delivered_news_items(before: &[(usize, i32, i32)], kind: u32) -> Vec<Vec<u8>> {
    let news = read_usize(G_NEWS);
    let mut out = Vec::new();
    for &(e, start, next) in before {
        let (start_now, next_now) = (read_i32(e), read_i32(e + 4));
        if start_now == -1 || (start_now == start && next_now == next) {
            continue;
        }
        let mut k = if start_now == start { next } else { start_now };
        while k != next_now {
            let r = read_usize(news).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
            if read_u32(r) == kind {
                let mut bytes: Vec<u8> = (0..news_item::ID).map(|j| read_u8(r + j)).collect();
                bytes[news_item::IMPORTANT] = 0;
                out.push(bytes);
            }
            k += 1;
            if k == start_now + 0x64 {
                k = start_now;
            }
        }
    }
    out.sort();
    out
}

/// The comparer's dry run of `post_ban_news`: the copy builds the news item and keeps each send instead of sending it.
pub(crate) static BAN_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// The sends the dry run kept: (the club, or 0 for every human manager; the news item's first `news_item::ID` bytes).
pub(crate) static BAN_NEWS_SENT: std::sync::Mutex<Vec<(usize, Vec<u8>)>> = std::sync::Mutex::new(Vec::new());

/// Takes the person out of the squad of `from` when he has a player record (`manager_manager_remove_player_from_squad`).
pub(crate) unsafe fn remove_player(this: usize, person: Staff, from: Club) {
    if !person.player().is_null() {
        manager_manager_remove_player_from_squad(this, person.addr(), from.addr());
    }
}

/// Reads the discipline object from the save's "discipline.dat". Without a save (line 0xda) or the item (0xe2): the
/// discipline.cpp "Error" box and 0. Then: the per-person records (`G_STAFF_COUNT` × 29 bytes, `malloc` into
/// `discipline_fields::PERSON_RECORDS`; lines 0xea, 0xf2); the rule sets (`discipline_create_rule_sets` with the address
/// of the open item: each rule set reads its entries and pending bans); the ban array (`BAN_COUNT`, `BAN_CAPACITY`,
/// `BANS`, 20 bytes each; lines 0xfd, 0x10a, 0x112) and the nation index (`NATION_INDEX_COUNT`, `NATION_INDEX_CAPACITY`,
/// `NATION_INDEX`, 8 bytes each; lines 0x11b, 0x128, 0x130), each allocated only with a count; those failures close the
/// item and answer 0. Then the held-ban count (i16; a short read shows the box of line 0x139 and counts 0) and that many 10-byte
/// held bans into the `HELD_BANS` list (`LLIST::add_tail`; a short read shows the box of line 0x143 and goes on with the
/// next one). The item closed, 1.
/// Original: `Discipline::load` (0x555860, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_load(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0xda);
        return 0;
    }
    let mut z = exe::zopen(save, DISCIPLINE_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0xe2);
        return 0;
    }
    covered!("sx09c_4 load discipline.dat");
    let tables = |z: &mut usize| -> Result<(), u32> {
        let records = exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(0x1d));
        write_usize(this + discipline_fields::PERSON_RECORDS, records);
        if records == 0 {
            return Err(0xea);
        }
        if exe::zread(records, 0x1d, read_u32(G_STAFF_COUNT), *z) != read_u32(G_STAFF_COUNT) {
            return Err(0xf2);
        }
        discipline_create_rule_sets(this, z as *mut usize as u32); // sx09c_4: the rule sets read their entries
        discipline_read_table(*z, this, discipline_fields::BAN_COUNT, discipline_fields::BAN_CAPACITY,
                              discipline_fields::BANS, 0x14, (0xfd, 0x10a, 0x112))?;
        discipline_read_table(*z, this, discipline_fields::NATION_INDEX_COUNT, discipline_fields::NATION_INDEX_CAPACITY,
                              discipline_fields::NATION_INDEX, 8, (0x11b, 0x128, 0x130))
    };
    if let Err(row) = tables(&mut z) {
        show_error_box(DISCIPLINE_CPP_PATH, row);
        exe::zclose(&mut z);
        return 0;
    }
    let mut held = 0i16;
    if exe::zread(&mut held as *mut i16 as usize, 2, 1, z) != 1 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x139);
        held = 0;
    }
    let mut record = [0u8; 0xa];
    let mut k = 0i16;
    while k < held {
        if exe::zread(record.as_mut_ptr() as usize, 0xa, 1, z) != 1 {
            show_error_box(DISCIPLINE_CPP_PATH, 0x143);
        } else {
            exe::llist_add_tail(this + discipline_fields::HELD_BANS, record.as_ptr() as usize, 0xa);
        }
        k += 1;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `discipline_load`: it reads the save item and allocates, so it cannot run twice; only the original
/// runs. The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_discipline_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_DISCIPLINE_LOAD)(this, save)
}

/// Writes the discipline object to the save's "discipline.dat": the per-person records (`G_STAFF_COUNT` × 29 bytes), each
/// existing rule set of the 43 (`RulingBody::save` with the address of the open item), the ban array (i16 count and
/// 20-byte bans), the nation index (i16 count and 8-byte items), the held-ban count (the list's i16) and the 10-byte
/// held bans from the list's head until it runs out. Without a save (line 0x163) or the item (0x16b): the
/// discipline.cpp "Error" box. A short write of the records or tables (lines 0x172, 0x185, 0x18f, 0x198, 0x1a2): the box
/// and the item closed. A short held-ban count (0x1ac) only shows the box; a short held ban (0x1b7) shows it and stops
/// the list. The item closed. The original returns nothing (eax is left over; the caller ignores it).
/// Original: `Discipline::save` (0x555dd0, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_save(this: usize, save: usize) {
    if save == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x163);
        return;
    }
    let mut z = exe::zopen(save, DISCIPLINE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x16b);
        return;
    }
    covered!("sx09c_4 save discipline.dat");
    let tables = |z: &mut usize| -> Result<(), u32> {
        let n = read_u32(G_STAFF_COUNT);
        if exe::zwrite(read_usize(this + discipline_fields::PERSON_RECORDS), 0x1d, n, *z) != read_u32(G_STAFF_COUNT) {
            return Err(0x172);
        }
        let save_body: unsafe extern "thiscall" fn(usize, *mut usize) = core::mem::transmute(FN_RULING_BODY_SAVE);
        for k in 0..0x2busize {
            let body = read_usize(read_usize(this + discipline_fields::RULING_BODIES) + 4 * k);
            if body != 0 {
                save_body(body, z as *mut usize); // sx09c_4: each rule set
            }
        }
        discipline_write_table(*z, this, discipline_fields::BAN_COUNT, discipline_fields::BANS, 0x14, (0x185, 0x18f))?;
        discipline_write_table(*z, this, discipline_fields::NATION_INDEX_COUNT, discipline_fields::NATION_INDEX, 8, (0x198, 0x1a2))
    };
    if let Err(row) = tables(&mut z) {
        show_error_box(DISCIPLINE_CPP_PATH, row);
        exe::zclose(&mut z);
        return;
    }
    let held_list = Llist(this + discipline_fields::HELD_BANS);
    let held = held_list.count() as i32; // the original stores the i16 in a dword and writes 2 bytes
    if exe::zwrite(&held as *const i32 as usize, 2, 1, z) != 1 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x1ac);
    }
    held_list.set_cursor(held_list.head());
    let mut record = exe::llist_get_next(held_list);
    while record != 0 {
        if exe::zwrite(record, 0xa, 1, z) != 1 {
            show_error_box(DISCIPLINE_CPP_PATH, 0x1b7);
            break;
        }
        record = exe::llist_get_next(held_list);
    }
    exe::zclose(&mut z);
}

/// Comparer for `discipline_save`: it writes into the save container, so it cannot run twice; only the original runs.
/// The copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_discipline_save(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_DISCIPLINE_SAVE)(this, save)
}

/// Reads one counted table of the discipline object from the open item: the i16 count into `count` (box `row_count`
/// when short); with a count, the capacity field takes it, the array (`malloc` of count × `size`) goes into `table`
/// and its records are read (boxes `row_alloc`, `row_read`). Returns the line of the failure.
pub(crate) unsafe fn discipline_read_table(z: usize, this: usize, count: usize, capacity: usize, table: usize, size: u32,
                                rows: (u32, u32, u32)) -> Result<(), u32> {
    if exe::zread(this + count, 2, 1, z) != 1 {
        return Err(rows.0);
    }
    let n = read_i16(this + count);
    if n == 0 {
        return Ok(());
    }
    write_i16(this + capacity, n);
    let array = exe::crt_malloc((n as i32 as u32).wrapping_mul(size));
    write_usize(this + table, array);
    if array == 0 {
        return Err(rows.1);
    }
    if exe::zread(array, size, n as i32 as u32, z) != read_i16(this + count) as i32 as u32 {
        return Err(rows.2);
    }
    Ok(())
}

/// Writes one counted table of the discipline object: the i16 count (line `rows.0` when short) and, with a count,
/// its records of `size` bytes (line `rows.1`).
pub(crate) unsafe fn discipline_write_table(z: usize, this: usize, count: usize, table: usize, size: u32, rows: (u32, u32)) -> Result<(), u32> {
    if exe::zwrite(this + count, 2, 1, z) != 1 {
        return Err(rows.0);
    }
    let n = read_i16(this + count);
    if n != 0 && exe::zwrite(read_usize(this + table), size, n as i32 as u32, z) != read_i16(this + count) as i32 as u32 {
        return Err(rows.1);
    }
    Ok(())
}

/// The save item "discipline.dat".
pub(crate) const DISCIPLINE_DAT: usize = 0x9b8f58;

/// The source path of discipline.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\discipline.cpp").
pub(crate) const DISCIPLINE_CPP_PATH: usize = 0x9b8f18;

/// `RulingBody::save` (0x7e7b80, ruling_body.cpp; thiscall(body, &z), ret 4): writes one rule set's entries and pending
/// bans to the open item.
pub(crate) const FN_RULING_BODY_SAVE: usize = 0x7e7b80;
