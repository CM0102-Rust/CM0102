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
