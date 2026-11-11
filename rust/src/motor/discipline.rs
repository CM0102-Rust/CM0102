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

