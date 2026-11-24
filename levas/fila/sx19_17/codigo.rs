// ---------- sx19_17: a ruling body's disciplinary entries: finding or adding a player's, attaching and detaching his ban (ruling_body/ruling_body.cpp) ----------

/// The size of a ruling body's disciplinary entry (`ruling_body_entry`).
const RULING_BODY_ENTRY_SIZE: usize = 0xe;

/// The address of entry `k` of the body's entries.
unsafe fn ruling_body_entry_at(this: usize, k: i32) -> usize {
    read_usize(this + ruling_body::ENTRIES).wrapping_add((k as usize).wrapping_mul(RULING_BODY_ENTRY_SIZE))
}

/// The player's index in the body's entries, from his discipline record (`table_penalty` on the body's record slot and
/// rule set); −1 when he has none.
unsafe fn ruling_body_entry_index(this: usize, id: u32) -> i32 {
    table_penalty(read_usize(G_DISCIPLINE), id as i32, read_u8(this + ruling_body::RECORD_SLOT) as i8, read_u8(this + ruling_body::RULE_SET_INDEX) as i8)
}

/// Returns the player's disciplinary entry in this ruling body, adding one when he has none and `create` is set.
///
/// The index comes from the player's discipline record (`ruling_body_entry_index`). Without one and without `create`:
/// 0. With `create`: a held ban of the body's rule set in the record slot (`Discipline::find_held_ban`) is the error
/// box of line 0x295 (and goes on); a full array grows by 100 entries (`realloc`, or `malloc` the first time; failing,
/// the error box of line 0x2b0 and 0); the new entry at the end is zeroed with the player's id and no ban (−1), and its
/// index goes into the player's record (`Discipline::set_slot`). An index outside the array clears the record's slot
/// (−1) and gives 0; otherwise the entry's address.
/// Original: `RulingBody::find_or_add_discipline_entry` (0x7e82e0, ruling_body.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn find_or_add_discipline_entry(this: usize, id: u32, create: u32) -> usize {
    let d = read_usize(G_DISCIPLINE);
    let (slot, rule) = (read_u8(this + ruling_body::RECORD_SLOT) as i8, read_u8(this + ruling_body::RULE_SET_INDEX));
    let mut k = ruling_body_entry_index(this, id);
    if k == -1 {
        if create == 0 {
            return 0;
        }
        covered!("sx19_17 entry added");
        let held = exe::discipline_find_held_ban(d, id as i32, slot);
        if held != 0 && read_u8(held + 5) == rule { // the held ban's rule set
            show_error_at(RULING_BODY_CPP, 0x295);
        }
        let count = read_i32(this + ruling_body::ENTRY_COUNT);
        if count == read_i32(this + ruling_body::ENTRY_CAPACITY) {
            let capacity = count + 100;
            write_i32(this + ruling_body::ENTRY_CAPACITY, capacity);
            let old = read_usize(this + ruling_body::ENTRIES);
            let size = (capacity as u32).wrapping_mul(RULING_BODY_ENTRY_SIZE as u32);
            let p = if old != 0 { exe::crt_realloc(old, size) } else { exe::crt_malloc(size) };
            write_usize(this + ruling_body::ENTRIES, p);
            if p == 0 {
                show_error_at(RULING_BODY_CPP, 0x2b0);
                return 0;
            }
        }
        let e = ruling_body_entry_at(this, count);
        core::ptr::write_bytes(e as *mut u8, 0, RULING_BODY_ENTRY_SIZE);
        write_i16(e + ruling_body_entry::BAN_INDEX, -1);
        write_u32(e + ruling_body_entry::PERSON_ID, id); // sx19_17: the new entry's player
        k = count;
        write_i32(this + ruling_body::ENTRY_COUNT, count + 1);
        exe::discipline_set_slot(d, id as i32, slot, rule, k as u32);
    }
    if k < read_i32(this + ruling_body::ENTRY_CAPACITY) && k >= 0 {
        return ruling_body_entry_at(this, k);
    }
    exe::discipline_set_slot(d, id as i32, slot, rule, u32::MAX);
    0
}

/// Links ban `ban` of the discipline's ban array to the player's entry (found or added). When the entry already points
/// to a ban, what is left of the old one goes onto the new one (its days when positive, otherwise its matches when
/// positive) and the old one is released (`Discipline::release_ban`).
/// Original: `RulingBody::attach_ban` (0x7e8510, ruling_body.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn attach_ban(this: usize, id: u32, ban: i16) {
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    if e == 0 {
        return;
    }
    let old = read_i16(e + ruling_body_entry::BAN_INDEX);
    if old != -1 {
        covered!("sx19_17 ban merged");
        let d = read_usize(G_DISCIPLINE);
        let bans = read_usize(d + discipline_fields::BANS);
        let (o, n) = (bans.wrapping_add((old as isize * 20) as usize), bans.wrapping_add((ban as isize * 20) as usize));
        let days = read_i16(o + ban_record::DAYS);
        if days > 0 {
            write_i16(n + ban_record::DAYS, read_i16(n + ban_record::DAYS).wrapping_add(days));
        } else if read_i8(o + ban_record::MATCHES) > 0 { // sx19_17: the old ban's matches go onto the new one
            write_u8(n + ban_record::MATCHES, read_u8(n + ban_record::MATCHES).wrapping_add(read_u8(o + ban_record::MATCHES)));
        }
        exe::discipline_release_ban(d, id, old as u16 as u32);
    }
    write_i16(e + ruling_body_entry::BAN_INDEX, ban);
}

/// Unlinks the player's ban from his entry (−1), when he has an entry; called when the ban is released.
/// Original: `RulingBody::detach_ban` (0x7e8590, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn detach_ban(this: usize, id: u32) {
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
    if e != 0 {
        covered!("sx19_17 ban detached");
        write_i16(e + ruling_body_entry::BAN_INDEX, -1); // sx19_17: the entry without a ban
    }
}

/// Comparer of `find_or_add_discipline_entry`. A lookup (no `create`, or the player has an index) runs both and
/// compares the addresses. Adding cannot happen twice: only the original runs, and the new entry is checked (the
/// player's id, no ban, zero counts, one more entry).
pub(crate) unsafe extern "thiscall" fn compare_find_or_add_discipline_entry(this: usize, id: u32, create: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I0%%);
    if create == 0 || ruling_body_entry_index(this, id) != -1 {
        let r = o(this, id, create);
        let n = find_or_add_discipline_entry(this, id, create);
        SCOREBOARD.record(r == n, || format!("find_or_add_discipline_entry(player {}, {}): original {:#x} rust {:#x}", id, create, r, n));
        return r;
    }
    let before = read_i32(this + ruling_body::ENTRY_COUNT);
    let r = o(this, id, create);
    let ok = r == 0 || (read_u32(r + ruling_body_entry::PERSON_ID) == id && read_i16(r + ruling_body_entry::BAN_INDEX) == -1
        && read_u16(r + ruling_body_entry::YELLOW_CARDS) == 0 && read_u32(r + ruling_body_entry::DISCIPLINE_POINTS) == 0
        && read_i32(this + ruling_body::ENTRY_COUNT) == before + 1 && ruling_body_entry_index(this, id) == before);
    SCOREBOARD.record(ok, || format!("find_or_add_discipline_entry(player {}): new entry {:#x}, count {} (before {})", id, r,
        read_i32(this + ruling_body::ENTRY_COUNT), before));
    r
}

/// Comparer of `attach_ban`: the old ban is released only once, so only the original runs; checked afterwards: the
/// entry points to the new ban and the new ban carries the old one's days or matches.
pub(crate) unsafe extern "thiscall" fn compare_attach_ban(this: usize, id: u32, ban: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, i16)>(%%I1%%);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    if e == 0 {
        return o(this, id, ban);
    }
    let bans = read_usize(read_usize(G_DISCIPLINE) + discipline_fields::BANS);
    let at = |k: i16| bans.wrapping_add((k as isize * 20) as usize);
    let old = read_i16(e + ruling_body_entry::BAN_INDEX);
    let n = at(ban);
    let (mut days, mut matches) = (read_i16(n + ban_record::DAYS), read_u8(n + ban_record::MATCHES));
    if old != -1 {
        let o = at(old);
        if read_i16(o + ban_record::DAYS) > 0 {
            days = days.wrapping_add(read_i16(o + ban_record::DAYS));
        } else if read_i8(o + ban_record::MATCHES) > 0 {
            matches = matches.wrapping_add(read_u8(o + ban_record::MATCHES));
        }
    }
    o(this, id, ban);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
    let ok = e != 0 && read_i16(e + ruling_body_entry::BAN_INDEX) == ban && read_i16(n + ban_record::DAYS) == days
        && read_u8(n + ban_record::MATCHES) == matches;
    SCOREBOARD.record(ok, || format!("attach_ban(player {}, ban {}): old {}, days {} matches {} (expected {} {})", id, ban, old,
        read_i16(n + ban_record::DAYS), read_u8(n + ban_record::MATCHES), days, matches));
}

/// Comparer of `detach_ban`: only the original runs; afterwards the player's entry, when there is one, has no ban.
pub(crate) unsafe extern "thiscall" fn compare_detach_ban(this: usize, id: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%)(this, id);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
    SCOREBOARD.record(e == 0 || read_i16(e + ruling_body_entry::BAN_INDEX) == -1, || format!("detach_ban(player {}): entry {:#x} ban {}", id, e,
        if e != 0 { read_i16(e + ruling_body_entry::BAN_INDEX) } else { 0 }));
}
