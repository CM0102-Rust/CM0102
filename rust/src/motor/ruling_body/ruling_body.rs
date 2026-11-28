use super::*;

/// Matches of ban by foul code (9..12) and ruling body scale (+6); None when no ban.
pub(crate) fn ban_matches(code: i32, e: i8) -> Option<u8> {
    let x = e as i32;
    Some(match code {
        9 if e < 3 => return None,
        9 if e < 8 => 1,
        9 => 2 + (e >= 0xf) as u8,
        10 if e < 3 => return None,
        10 if e < 5 => 1,
        10 if e < 9 => 2,
        10 if e < 0xd => 3,
        10 => 4 + (e >= 0x11) as u8,
        11 => ((x * 8) / 20 + 4) as u8,
        12 => ((x * 28) / 20 + 12) as u8,
        _ => return None,
    })
}

/// Days of ban (same steps in weeks) by foul code and scale; None when no ban.
pub(crate) fn ban_days(code: i32, e: i8) -> Option<u16> {
    let x = e as i32;
    Some(match code {
        9 if e < 3 => return None,
        9 if e < 8 => 7,
        9 => if e >= 0xf { 21 } else { 14 },
        10 if e < 3 => return None,
        10 if e < 5 => 7,
        10 if e < 9 => 14,
        10 if e < 0xd => 21,
        10 => if e >= 0x11 { 35 } else { 28 },
        11 => (((x * 8) / 20 + 4) * 7) as u16,
        12 => (((x * 28) / 20 + 12) * 7) as u16,
        _ => return None,
    })
}

/// Builds the red card ban: the matches or days from the offence code (9 to 12) and the body's scale, today's date and
/// the player; 0 when there is no ban. Ret 0xc: `make_points_ban(r, j, by_games)` (record, player, by matches), from
/// each country's bodies (rb_*.cpp, when processing a match's red card): the offence code (`first_card_offence()` with
/// `yellow_cards()` = 0, `second_card_offence()` with `yellow_cards()` = 1; another type: 0) gives, through the body's
/// scale (+6), the matches (`by_games` ≠ 0; the days stay 0xffff) or the days (the matches stay 0xff) of the ban. In
/// the record: +0 matches, +1 = 0, +2 days, +4 today's date (`G_CURRENT_DATE`, `exe::date_operator_3d`),
/// `ban_record::REASON` = 3, `pending_ban::PERSON_ID` the player's id (+0), +0x6f = (+0x48 of the player == 0). Returns
/// 1; with no ban, 0 without writing anything.
/// Original: `RulingBody::make_points_ban` (0x7e8e10, ruling_body.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn make_points_ban(this: usize, r: usize, j: usize, by_games: u32) -> u32 {
    let j = MatchRecordRow(j);
    let code = match j.yellow_cards() {
        0 => j.first_card_offence() as i32, // cpu10: the foul of the first type
        1 => j.second_card_offence() as i32,
        _ => return 0,
    };
    let escala = read_i8(this + 6);
    let (jogos, dias) = if by_games != 0 {
        match ban_matches(code, escala) {
            Some(n) => (n, 0xffffu16),
            None => return 0,
        }
    } else {
        match ban_days(code, escala) {
            Some(n) => (0xffu8, n),
            None => return 0,
        }
    };
    covered!("cpu_10 punishment");
    write_u8(r, jogos); // cpu10: the suspension matches
    write_u8(r + 1, 0);
    write_u16(r + 2, dias);
    let mut today = [0u32; 2];
    exe::date_operator_3d(r + 4, today.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(r + ban_record::REASON, 3);
    write_u32(r + pending_ban::PERSON_ID, read_usize(j.addr()) as u32);
    write_u8(r + 0x6f, (read_u8(j.addr() + 0x48) == 0) as u8);
    1 // cpu10: the punishment carried out
}

/// At the new season, releases bans 2 (or 4) years old, frees the entries without a ban, clears the bookings of the
/// others and compacts the table. Ret 0: `RulingBody::start_new_season()`: for each occupied disciplinary entry (14
/// bytes in `ruling_body::ENTRIES`, count `ruling_body::ENTRY_COUNT`; id −1 is a free slot): with a ban (+6, signed
/// index into the 20-byte discipline table `[[G_DISCIPLINE]+4]`) 2 or more years old (4 if the body has neither
/// `ruling_body::NATION_ID` nor `ruling_body::CONTINENT_ID`; the ban year in +6 against today's `G_CURRENT_DATE_YEAR`),
/// releases the ban (`release_ban`) and frees the entry; without a ban, frees the entry; otherwise zeroes the bookings
/// (+4, +5, +8 and the low 6 bits of `ruling_body_entry::SEASON_FLAGS`). Freeing: the player's stored slot in the body
/// (`find_held_ban` through slot +4) with the same +5 gets −1 in +6; without it, writes the slot
/// (`discipline_set_slot`, type +5, −1); the entry's id becomes −1. At the end compacts the entries (`compact_entries`,
/// in this batch).
/// Original: `RulingBody::start_new_season` (0x7e7ed0, ruling_body.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn ruling_body_start_new_season(this: usize) {
    covered!("cpu_10 new season");
    let mut e = read_usize(this + ruling_body::ENTRIES);
    let mut i = 0i32;
    while i < read_i32(this + ruling_body::ENTRY_COUNT) {
        let id = read_i32(e);
        if id != -1 {
            let s = read_i16(e + 6);
            let free = if s == -1 {
                true
            } else {
                let ban = read_usize(read_usize(G_DISCIPLINE) + 4).wrapping_add((s as i32 as usize).wrapping_mul(20));
                let years = if read_i32(this + ruling_body::NATION_ID) == -1 && read_i32(this + ruling_body::CONTINENT_ID) == -1 { 4 } else { 2 };
                let year = read_u16(ban + 6) as i32;
                if read_u16(G_CURRENT_DATE_YEAR) as i32 - year >= years {
                    covered!("cpu_10 old suspension");
                    // simplification: the original pushes the index with the high 16 bits of the previous edx; 0x556d00 reads only the word.
                    exe::discipline_release_ban(read_usize(G_DISCIPLINE), id as u32, s as u16 as u32);
                    true
                } else {
                    false
                }
            };
            if free {
                let id = read_i32(e);
                let g = exe::discipline_find_held_ban(read_usize(G_DISCIPLINE), id, read_i8(this + 4));
                if g != 0 && *((g + 5) as *const u8) == read_u8(this + 5) {
                    write_u32(g + 6, 0xffff_ffff);
                } else {
                    exe::discipline_set_slot(read_usize(G_DISCIPLINE), read_i32(e), read_i8(this + 4), read_u8(this + 5), 0xffff_ffff);
                }
                write_u32(e, 0xffff_ffff);
            } else {
                let f = read_u8(e + ruling_body_entry::SEASON_FLAGS);
                write_u8(e + 4, 0);
                write_u8(e + 5, 0);
                write_u16(e + 8, 0);
                write_u8(e + ruling_body_entry::SEASON_FLAGS, f & 0xc0);
            }
        }
        i += 1;
        e += 0xe;
    }
    exe::rulingbody_compact_entries(this);
}

/// Comparador: a original solta suspensões e realoca (não pode rodar duas vezes): só ela roda.
// simplification: the copy carries the ”cpu_10 nova temporada” counter to prove in Rust mode; the compaction (compared) checks the end.

/// Removes the free entries from the body's discipline table, fixes the stored index of each moved player and
/// reallocates with 100 spare rows. Ret 0: `RulingBody::compact_entries()`: removes the free entries (id −1) from the
/// 14-byte table `ruling_body::ENTRIES`, bringing the occupied ones to the front; each one that moves fixes the
/// player's stored index (`find_held_ban` through slot +4: with the same +5, the new index in +6; without it,
/// `discipline_set_slot` with type +5 and the index). Count `ruling_body::ENTRY_COUNT` = n, capacity
/// `ruling_body::ENTRY_CAPACITY` = n + 100 and reallocates (`exe::crt_realloc`) with (n + 100) × 14 bytes; out of
/// memory, the box (line 0x594) and continues.
/// Original: `RulingBody::compact_entries` (0x7e9050, ruling_body.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn compact_entries(this: usize) {
    let mut target = read_usize(this + ruling_body::ENTRIES);
    let mut source = target;
    let mut n = 0u32;
    let mut i = 0i32;
    while i < read_i32(this + ruling_body::ENTRY_COUNT) {
        if read_i32(source) != -1 {
            if source != target {
                core::ptr::copy(source as *const u8, target as *mut u8, 0xe);
                let g = exe::discipline_find_held_ban(read_usize(G_DISCIPLINE), read_i32(source), read_i8(this + 4));
                if g != 0 && *((g + 5) as *const u8) == read_u8(this + 5) {
                    write_u32(g + 6, n);
                } else {
                    exe::discipline_set_slot(read_usize(G_DISCIPLINE), read_i32(source), read_i8(this + 4), read_u8(this + 5), n);
                }
            }
            target += 0xe;
            n += 1;
        }
        i += 1;
        source += 0xe;
    }
    write_u32(this + ruling_body::ENTRY_COUNT, n);
    write_u32(this + ruling_body::ENTRY_CAPACITY, n + 100);
    let p = exe::crt_realloc(read_usize(this + ruling_body::ENTRIES), (n + 100) * 14);
    write_u32(this + ruling_body::ENTRIES, p as u32);
    if p == 0 {
        show_error_at(0xa7a858, 0x594);
    }
}

/// The base constructor of every ruling body: its vtable, a new list, no entries; from a saved game, its saved state.
///
/// [this] = `RULING_BODY_VTABLE`; `RULING_BODY_LIST` = a new LLIST (`exe::crt_operator_new(0xe)` and LLIST::LLIST
/// `exe::new_llist`, or 0 out of memory); no entries (`ruling_body::ENTRIES`, `ENTRY_CAPACITY` and `ENTRY_COUNT` = 0).
/// With a saved game (`file`), RulingBody::load(file) (`FN_RULING_BODY_LOAD`, thiscall, ret 4): 0 shows the error box
/// (line 0x36). Returns this (the original builds the list under an SEH frame; nothing here throws).
/// Original: `RulingBody::RulingBody` (0x7e7760, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_ruling_body(this: usize, file: usize) -> usize {
    write_u32(this, RULING_BODY_VTABLE);
    let p = exe::crt_operator_new(0xe);
    let list = if p != 0 { exe::new_llist(p) } else { 0 };
    write_usize(this + RULING_BODY_LIST, list);
    covered!("mwg_17 ruling body built");
    write_u32(this + ruling_body::ENTRIES, 0);
    write_u32(this + ruling_body::ENTRY_CAPACITY, 0);
    write_u32(this + ruling_body::ENTRY_COUNT, 0);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_RULING_BODY_LOAD);
        if load(this, file) == 0 {
            show_error_at(RULING_BODY_CPP, 0x36);
        }
    }
    this
}

/// Comparer for `construct_ruling_body`.
pub(crate) unsafe extern "thiscall" fn compare_construct_ruling_body(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_RULING_BODY)(this, file);
    let ok = file != 0 || (r == this && read_u32(this) == RULING_BODY_VTABLE && read_usize(this + RULING_BODY_LIST) != 0
        && read_u32(this + ruling_body::ENTRIES) == 0 && read_u32(this + ruling_body::ENTRY_COUNT) == 0);
    SCOREBOARD.record(ok, || format!("construct_ruling_body({:#x}): vtable {:#x} list {:#x}", this, read_u32(this), read_usize(this + RULING_BODY_LIST)));
    r
}

/// RulingBody's own vtable, RulingBody::load (thiscall, ret 4) and the ruling body's list (+7, a 0xe-byte LLIST).
pub(crate) const RULING_BODY_VTABLE: u32 = 0x96f104;

pub(crate) const FN_RULING_BODY_LOAD: usize = 0x7e78a0;

pub(crate) const RULING_BODY_LIST: usize = 0x7;

/// "E:\dev\CM3\cm3 00-01\cm3\code\ruling_body\ruling_body.cpp" (the error box).
pub(crate) const RULING_BODY_CPP: usize = 0xa7a858;

/// Reads the discipline entry array and the pending ban list from a saved game, then the body's own extra data.
///
/// `RulingBody::load(file)` (file = the save item, its stream at +0), from the base constructor with a save
/// (`construct_ruling_body`): the entry count (`ruling_body::ENTRY_COUNT`, 4 bytes); with entries, the capacity
/// (`ENTRY_CAPACITY`) = the count and a new array (`exe::crt_malloc`, 14 bytes each; out of memory: the box at line
/// 0x6c) in `ENTRIES`, read whole; then the number of pending bans (16 bits) and each 0x70-byte node, whose fixture copy
/// (+0x20) gets its competition, stadium and both clubs back as pointers from their ids (`G_CLUB_COMP_TABLE`,
/// `G_STADIUM_TABLE`, `G_CLUB_TABLE`) before it goes to the end of the body's list (`exe::llist_add_tail`, copied).
/// Last, the body's `load_extra_data` (vtable +0x18): returns 1 when it answers. A short read: the ruling_body.cpp
/// "Error" box with the field's line (0x5f, 0x74, 0x7e, 0x8b) and 0.
/// Original: `RulingBody::load` (0x7e78a0, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_load(this: usize, file: usize) -> u32 {
    let stream = read_usize(file);
    if exe::zread(this + ruling_body::ENTRY_COUNT, 4, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0x5f);
        return 0;
    }
    let count = read_u32(this + ruling_body::ENTRY_COUNT);
    if count != 0 {
        write_u32(this + ruling_body::ENTRY_CAPACITY, count);
        let entries = exe::crt_malloc(count.wrapping_mul(14));
        write_usize(this + ruling_body::ENTRIES, entries);
        if entries == 0 {
            show_error_at(RULING_BODY_CPP, 0x6c);
            return 0;
        }
        if exe::zread(entries, 14, count, stream) != read_u32(this + ruling_body::ENTRY_COUNT) {
            show_error_at(RULING_BODY_CPP, 0x74);
            return 0;
        }
    }
    // simplification: the original constructs the node's two dates (+4, +0x14) first; the read overwrites them
    let mut node = [0u8; PENDING_BAN_NODE_SIZE as usize];
    let mut pending = 0i16;
    if exe::zread(&mut pending as *mut i16 as usize, 2, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0x7e);
        return 0;
    }
    let mut i = 0i16;
    while i < pending {
        if exe::zread(node.as_mut_ptr() as usize, PENDING_BAN_NODE_SIZE, 1, stream) != 1 {
            show_error_at(RULING_BODY_CPP, 0x8b);
            return 0;
        }
        covered!("sx19b_3 pending ban loaded");
        let f = Fixture(node.as_mut_ptr() as usize + PENDING_BAN_FIXTURE);
        // the club competition records are 0x6b bytes
        f.set_competition(Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((f.comp_id() as usize).wrapping_mul(0x6b))));
        f.set_stadium(read_usize(G_STADIUM_TABLE).wrapping_add(f.stadium_id().wrapping_mul(stadium::SIZE)));
        f.set_home_club(Club(read_usize(G_CLUB_TABLE).wrapping_add(f.home_club_id().wrapping_mul(club::SIZE)))); // sx19b_3: the home club relinked
        f.set_away_club(Club(read_usize(G_CLUB_TABLE).wrapping_add(f.away_club_id().wrapping_mul(club::SIZE))));
        exe::llist_add_tail(read_usize(this + RULING_BODY_LIST), node.as_ptr() as usize, PENDING_BAN_NODE_SIZE);
        i += 1;
    }
    (ruling_body_extra_data(this, 0x18, file) != 0) as u32
}

/// Comparer for `ruling_body_load`: it reads the save stream, so only the original runs; checked afterwards: the
/// capacity equals the count when there are entries.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_load(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RULING_BODY_LOAD)(this, file);
    let (n, c) = (read_u32(this + ruling_body::ENTRY_COUNT), read_u32(this + ruling_body::ENTRY_CAPACITY));
    SCOREBOARD.record(r == 0 || n == 0 || n == c, || format!("ruling_body_load({:#x}): {} entries, capacity {}", this, n, c));
    r
}

/// Writes the discipline entry array and the pending ban list to a saved game, then the body's own extra data.
///
/// `RulingBody::save(file)`, from `Discipline::save` (each of the 43 bodies): the entry count
/// (`ruling_body::ENTRY_COUNT`, 4 bytes) and, with entries, the array (`ENTRIES`, 14 bytes each); the number of
/// pending bans (the list's 16-bit count, 0 without a list) and, with any, each node of the list (0x70 bytes, from the
/// head). Last, the body's `save_extra_data` (vtable +0x1c), whose answer is returned. A short write: the
/// ruling_body.cpp "Error" box with the field's line (0xae, 0xb8, 0xc7, 0xd5) and nothing more (the caller ignores
/// the answer; the copy returns 0).
/// Original: `RulingBody::save` (0x7e7b80, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_save(this: usize, file: usize) -> u32 {
    let stream = read_usize(file);
    if exe::zwrite(this + ruling_body::ENTRY_COUNT, 4, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0xae);
        return 0;
    }
    let count = read_u32(this + ruling_body::ENTRY_COUNT);
    if count != 0 && exe::zwrite(read_usize(this + ruling_body::ENTRIES), 14, count, stream) != read_u32(this + ruling_body::ENTRY_COUNT) {
        show_error_at(RULING_BODY_CPP, 0xb8);
        return 0;
    }
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    let pending: u32 = if list.is_null() { 0 } else { list.count() as u16 as u32 }; // sx19b_3: the pending ban count
    if exe::zwrite(&pending as *const u32 as usize, 2, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0xc7);
        return 0;
    }
    if pending as u16 != 0 {
        list.set_cursor(list.head());
        let mut node = exe::llist_get_next(list);
        while node != 0 {
            covered!("sx19b_3 pending ban saved");
            if exe::zwrite(node, PENDING_BAN_NODE_SIZE, 1, stream) != 1 {
                show_error_at(RULING_BODY_CPP, 0xd5);
                return 0;
            }
            node = exe::llist_get_next(list);
        }
    }
    ruling_body_extra_data(this, 0x1c, file)
}

/// Comparer for `ruling_body_save`: it writes the save stream, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_save(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RULING_BODY_SAVE)(this, file)
}

/// The ruling body's virtual slot `slot` (thiscall(body, save item), ret 4): +0x18 `load_extra_data`, +0x1c
/// `save_extra_data`.
pub(crate) unsafe fn ruling_body_extra_data(this: usize, slot: usize, file: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + slot));
    f(this, file)
}

/// The pending ban node of a ruling body's list (0x70 bytes): its fixture copy at +0x20.
pub(crate) const PENDING_BAN_NODE_SIZE: u32 = 0x70;

pub(crate) const PENDING_BAN_FIXTURE: usize = 0x20;

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_FIND_OR_ADD_DISCIPLINE_ENTRY);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, i16)>(IDX_ATTACH_BAN);
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
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DETACH_BAN)(this, id);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
    SCOREBOARD.record(e == 0 || read_i16(e + ruling_body_entry::BAN_INDEX) == -1, || format!("detach_ban(player {}): entry {:#x} ban {}", id, e,
        if e != 0 { read_i16(e + ruling_body_entry::BAN_INDEX) } else { 0 }));
}

/// Whether this ruling body's active ban on the player covers a match.
///
/// No entry in the body (`ruling_body_entry_index`): no. A body without entries is the error box of line 0x330 and no.
/// First the player's bans move on (`Discipline::update_ban_progress` with the body's rule set and 0 matches); then
/// the entry's ban (none: no). With `flagged_only`, a ban without flag bit 1 does not count. A ban that has not started
/// yet (today before its start) only counts against a later match (`ban_covers_later_match`; no fixture: no). A
/// started ban by days covers any match without a fixture, and a fixture's match when the days from the ban's start
/// to the match are fewer than the ban's; a ban by matches covers when the matches served plus `extra` are at most its
/// matches.
/// Original: `RulingBody::is_player_banned` (0x7e85b0, ruling_body.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn is_player_banned(this: usize, person: usize, flagged_only: u32, extra: i16, fixture: usize) -> u32 {
    let d = read_usize(G_DISCIPLINE);
    let id = read_u32(person);
    let k = ruling_body_entry_index(this, id);
    if k == -1 {
        return 0;
    }
    if read_usize(this + ruling_body::ENTRIES) == 0 {
        show_error_at(RULING_BODY_CPP, 0x330);
        return 0;
    }
    exe::discipline_update_ban_progress(d, id, read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 0);
    let b = read_i16(ruling_body_entry_at(this, k) + ruling_body_entry::BAN_INDEX);
    if b == -1 {
        return 0;
    }
    let ban = read_usize(d + discipline_fields::BANS).wrapping_add((b as isize * 20) as usize);
    if flagged_only != 0 && read_u8(ban + ban_record::FLAGS) & 2 == 0 {
        return 0;
    }
    if ban_date_before(G_CURRENT_DATE, ban + ban_record::START_DATE) {
        covered!("sx19_18 ban not started");
        return (fixture != 0 && ban_covers_later_match(ban, Staff(person), Fixture(fixture))) as u32;
    }
    if read_i16(ban + ban_record::DAYS) != -1 {
        if fixture == 0 {
            return 1;
        }
        covered!("sx19_18 ban by days");
        let mut md = [0u8; 8];
        exe::new_date(md.as_mut_ptr() as usize, Fixture(fixture).day() as u32, Fixture(fixture).year() as u32);
        let days = exe::date_days_since(md.as_ptr() as usize, read_u32(ban + ban_record::START_DATE), read_u32(ban + ban_record::START_DATE + 4));
        return (days < read_i16(ban + ban_record::DAYS) as i32) as u32;
    }
    covered!("sx19_18 ban by matches");
    (read_i8(ban + ban_record::MATCHES_SERVED) as i32 + extra as i32 <= read_i8(ban + ban_record::MATCHES) as i32) as u32 // sx19_18: matches still to serve
}

/// Clears the bookings of every player of the club in this body (all players with a null club); entries without a
/// player (−1) are skipped.
/// Original: `RulingBody::reset_bookings_for_club` (0x7e9180, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn reset_bookings_for_club(this: usize, club: usize) {
    let mut k = 0;
    while k < read_i32(this + ruling_body::ENTRY_COUNT) {
        let e = ruling_body_entry_at(this, k);
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id != -1 && (club == 0 || read_usize(staff_at(id as isize).addr() + staff::CLUB_JOB) == club) {
            covered!("sx19_18 bookings cleared");
            write_u8(e + ruling_body_entry::YELLOW_CARDS, 0); // sx19_18: the club's bookings cleared
        }
        k += 1;
    }
}

/// Removes every pending ban of the player from this body's list (`LLIST::remove` on the nodes whose person is his).
/// Original: `RulingBody::remove_pending_bans` (0x7e91d0, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_pending_bans(this: usize, id: u32) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if read_u32(node + pending_ban::PERSON_ID) == id { // sx19_18: the player's pending ban
            covered!("sx19_18 pending ban removed");
            exe::llist_remove(list, node);
        }
        node = exe::llist_get_next(list);
    }
}

/// Comparer of `is_player_banned`: the original runs first (it moves the player's bans on); the copy then asks the same
/// question (moving them on again changes nothing) and the answers are compared.
pub(crate) unsafe extern "thiscall" fn compare_is_player_banned(this: usize, person: usize, flagged_only: u32, extra: i16, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i16, usize) -> u32>(IDX_IS_PLAYER_BANNED)(this, person, flagged_only, extra, fixture);
    let n = is_player_banned(this, person, flagged_only, extra, fixture);
    SCOREBOARD.record(r == n, || format!("is_player_banned(player {}, {}, {}, fixture {:#x}): original {} rust {}", read_u32(person), flagged_only, extra, fixture, r, n));
    r
}

/// Comparer of `reset_bookings_for_club`: the copy clears a copy of the entries, the original runs, the bytes are
/// compared.
pub(crate) unsafe extern "thiscall" fn compare_reset_bookings_for_club(this: usize, club: usize) {
    let entries = read_usize(this + ruling_body::ENTRIES);
    let n = read_i32(this + ruling_body::ENTRY_COUNT).max(0) as usize;
    let mut copy = core::slice::from_raw_parts(entries as *const u8, n * RULING_BODY_ENTRY_SIZE).to_vec();
    for k in 0..n {
        let e = copy.as_mut_ptr() as usize + k * RULING_BODY_ENTRY_SIZE;
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id != -1 && (club == 0 || read_usize(staff_at(id as isize).addr() + staff::CLUB_JOB) == club) {
            write_u8(e + ruling_body_entry::YELLOW_CARDS, 0);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RESET_BOOKINGS_FOR_CLUB)(this, club);
    let got = core::slice::from_raw_parts(entries as *const u8, n * RULING_BODY_ENTRY_SIZE);
    SCOREBOARD.record(got == &copy[..], || format!("reset_bookings_for_club({:#x}, club {:#x}): {} entries differ", this, club, n));
}

/// Comparer of `remove_pending_bans`: only the original runs; afterwards no node of the body's list is the player's.
pub(crate) unsafe extern "thiscall" fn compare_remove_pending_bans(this: usize, id: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_REMOVE_PENDING_BANS)(this, id);
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    let mut left = 0;
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if read_u32(node + pending_ban::PERSON_ID) == id {
            left += 1;
        }
        node = exe::llist_get_next(list);
    }
    SCOREBOARD.record(left == 0, || format!("remove_pending_bans(player {}): {} nodes left", id, left));
}

/// Applies the body's pending bans whose start date has come.
///
/// From the start of the body's list, until the first node that starts after `date`: the node's ban record takes the
/// node's start date (+0x14 into +4) and the player's entry is looked up (not added). Without an entry the node is just
/// removed. Otherwise the player's bans move on (`Discipline::update_ban_progress` with the body's rule set and 0
/// matches); without a ban in the body, the node's ban is added as it is (flag bit 0 cleared, +0x6f = 1,
/// `Discipline::add_ban`); with one, the node's matches (or days, for a ban by days) go onto it (replacing a 0xff or −1)
/// with flag bit 2, and the node gets flag bit 0. The ban news is posted (the player, the node and the node's fixture at
/// +0x20) and the node removed; each removal restarts the walk from the head.
/// Original: `RulingBody::daily_update` (0x7e7d50, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_daily_update(this: usize, date: usize) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    let d = read_usize(G_DISCIPLINE);
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if pending_ban_after(node, date) {
            return;
        }
        covered!("sx19_19 pending ban due");
        copy_record_date(node + ban_record::START_DATE, node + 0x14); // the pending ban starts on its date
        let id = read_u32(node + pending_ban::PERSON_ID);
        let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
        if e != 0 {
            exe::discipline_update_ban_progress(d, id, read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 0);
            let b = read_i16(e + ruling_body_entry::BAN_INDEX);
            if b == -1 {
                write_u8(node + ban_record::FLAGS, read_u8(node + ban_record::FLAGS) & 0xfe);
                write_u8(node + 0x6f, 1); // the pending ban's byte +0x6f
                add_ban(d, id, node);
            } else {
                covered!("sx19_19 pending ban merged");
                write_u8(node + ban_record::FLAGS, read_u8(node + ban_record::FLAGS) | 1);
                let ban = read_usize(d + discipline_fields::BANS).wrapping_add((b as isize * 20) as usize);
                let matches = read_u8(node + ban_record::MATCHES);
                if matches != 0xff {
                    let old = read_u8(ban + ban_record::MATCHES);
                    write_u8(ban + ban_record::MATCHES, if old == 0xff { matches } else { old.wrapping_add(matches) }); // sx19_19: the pending matches added
                } else {
                    let (old, days) = (read_i16(ban + ban_record::DAYS), read_i16(node + ban_record::DAYS));
                    write_i16(ban + ban_record::DAYS, if old == -1 { days } else { old.wrapping_add(days) });
                }
                write_u8(ban + ban_record::FLAGS, read_u8(ban + ban_record::FLAGS) | 4);
            }
            let post: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_POST_BAN_NEWS);
            post(d, staff_at(id as isize).addr(), node, node + 0x20);
        }
        exe::llist_remove(list, node);
        list.set_cursor(list.head());
        node = exe::llist_get_next(list);
    }
}

/// Comparer of the day: the bans and their news cannot be handed out twice, so only the original runs; checked
/// afterwards: the nodes due that day (the leading ones not after the date) left the list and the others stayed.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_daily_update(this: usize, date: usize) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    let (mut due, mut total) = (0u16, 0u16);
    if !list.is_null() {
        total = list.count() as u16;
        list.set_cursor(list.head());
        let mut node = exe::llist_get_next(list);
        while node != 0 && !pending_ban_after(node, date) {
            due += 1;
            node = exe::llist_get_next(list);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RULING_BODY_DAILY_UPDATE)(this, date);
    let left = if list.is_null() { 0 } else { list.count() as u16 };
    SCOREBOARD.record(left == total - due, || format!("ruling_body_daily_update({:#x}): {} nodes left of {} ({} due)", this, left, total, due));
}

/// The address of entry `k` of the body's entries.
pub(crate) unsafe fn ruling_body_entry_at(this: usize, k: i32) -> usize {
    read_usize(this + ruling_body::ENTRIES).wrapping_add((k as usize).wrapping_mul(RULING_BODY_ENTRY_SIZE))
}

/// The player's index in the body's entries, from his discipline record (`table_penalty` on the body's record slot and
/// rule set); −1 when he has none.
pub(crate) unsafe fn ruling_body_entry_index(this: usize, id: u32) -> i32 {
    table_penalty(read_usize(G_DISCIPLINE), id as i32, read_u8(this + ruling_body::RECORD_SLOT) as i8, read_u8(this + ruling_body::RULE_SET_INDEX) as i8)
}

/// Whether date `a` (day i16 +0, year u16 +2) comes before date `b`: the year unsigned, then the day signed.
pub(crate) unsafe fn ban_date_before(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya < yb || (ya == yb && read_i16(a) < read_i16(b))
}

/// A ban that has not started yet against a match: the player's club in the fixture (home when the fixture's home club
/// is his club or his national team, otherwise away) and that side's mark pick the club's fixtures from the ban's start
/// to the match (`FIXTURE_MANAGER::find_fixtures` into a local list, any competition and kind, the side filter = the mark
/// is set); the unplayed ones (home goals 0xff) of a competition under the same discipline rule set as the fixture's are
/// counted. Covered when the ban's matches are at least that count; a ban starting after the match covers nothing.
pub(crate) unsafe fn ban_covers_later_match(ban: usize, person: Staff, fixture: Fixture) -> bool {
    let mut l = [0u8; 14];
    let list = Llist(l.as_mut_ptr() as usize);
    exe::new_llist(list);
    let mut md = [0u8; 8];
    exe::new_date(md.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    let m = md.as_ptr() as usize;
    if ban_date_before(m, ban + ban_record::START_DATE) {
        exe::drop_llist(list);
        return false;
    }
    let home = fixture.home_club().addr();
    let (club, mark) = if person.club_job().addr() == home || person.national_job().addr() == home {
        (home, fixture.home_side_mark())
    } else {
        (fixture.away_club().addr(), fixture.away_side_mark() as u8)
    };
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u16(ban + ban_record::START_DATE) as u32,
        read_u16(ban + ban_record::START_DATE + 2) as u32, read_u16(m) as u32, read_u16(m + 2) as u32, read_u32(club), list, u32::MAX,
        (-2i32) as u32, (mark != 0) as u32);
    let rule_of = |comp: i32| League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((comp as usize).wrapping_mul(4)))).discipline_rule();
    list.set_cursor(list.head());
    let mut count = 0i16;
    let mut f = exe::llist_get_next(list);
    while f != 0 {
        let other = Fixture(f);
        if other.home_goals() as u8 == 0xff && other.comp_id() >= 0 && fixture.comp_id() >= 0 {
            let r = rule_of(other.comp_id());
            if r != 0xff && r == rule_of(fixture.comp_id()) { // sx19_18: a match of the same rule set still to play
                count += 1;
            }
        }
        f = exe::llist_get_next(list);
    }
    exe::drop_llist(list);
    read_i8(ban + ban_record::MATCHES) as i16 >= count
}

/// Whether a pending ban's start (node +0x14: day i16, year u16) is after `date`: the list is in date order, so the
/// day's work stops at the first such node.
pub(crate) unsafe fn pending_ban_after(node: usize, date: usize) -> bool {
    let (yn, yd) = (read_u16(node + 0x16), read_u16(date + 2));
    yd < yn || (yd == yn && read_i16(date) < read_i16(node + 0x14))
}

/// The size of a ruling body's disciplinary entry (`ruling_body_entry`).
pub(crate) const RULING_BODY_ENTRY_SIZE: usize = 0xe;
