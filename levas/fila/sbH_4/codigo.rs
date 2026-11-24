// ---------- sbH_4: saving the friendlies (friendly.cpp) ----------

/// friendly.cpp, for the error boxes.
const FRIENDLY_SOURCE: usize = 0x9cd0a0;
/// "Friendly.tmp", the friendlies' file in CM3_TEMP.
const FRIENDLY_TEMP_FILE: usize = 0x9cd0f4;
/// The nation of a node of the scheduled slot list (`friendly::SCHEDULED_SLOT_LIST`), a pointer at +4.
const SCHEDULED_SLOTS_NATION: usize = 0x4;
/// The size of a node of the scheduled slot list (`friendly::SCHEDULED_SLOT_LIST`): the club +0, a nation +4, the slot
/// count +8 and ten slots of 0x19 bytes from +9.
const SCHEDULED_NODE_SIZE: u32 = 0x107;

/// A club pointer as the club id for the file (−1 for none), in place.
unsafe fn club_to_id(at: usize) {
    let club = read_usize(at);
    write_u32(at, if club != 0 { read_u32(club) } else { u32::MAX });
}

/// Back from the file: id −1 → none, otherwise the club of the id (`club_at`), in place.
unsafe fn id_to_club(at: usize) {
    let id = read_i32(at);
    write_usize(at, if id == -1 { 0 } else { club_at(id as isize).addr() });
}

/// The error box of line `line` and the file closed (`exe::crt_fclose`); false.
unsafe fn friendly_save_failed(f: usize, line: u32) -> bool {
    show_error_at(FRIENDLY_SOURCE, line);
    exe::crt_fclose(f);
    false
}

/// The items of an LLIST in order, its cursor rewound (`exe::llist_get_next`, one at a time as the original).
unsafe fn for_each_list_item(list: usize, mut each: impl FnMut(usize)) {
    Llist(list).set_cursor(Llist(list).head());
    loop {
        let item = exe::llist_get_next(list);
        if item == 0 {
            return;
        }
        each(item);
    }
}

/// One of the friendlies' lists with its count first: the count's place (`exe::crt_ftell`) and a 0 written there
/// (short: the box of line `lines.0`); with the list, its items (`write_item`, counted) and the count written over it
/// (`exe::crt_fseek`(place, 0); short: the box of line `lines.1`); then on to the end (`exe::crt_fseek`(0, 2)). False
/// after a box (the file closed).
unsafe fn save_counted_list(f: usize, list: usize, lines: (u32, u32), mut write_item: impl FnMut(usize)) -> bool {
    let place = exe::crt_ftell(f);
    let mut count = 0u32;
    if exe::crt_fwrite(&mut count as *mut u32 as usize, 4, 1, f) != 1 {
        return friendly_save_failed(f, lines.0);
    }
    if list != 0 {
        for_each_list_item(list, |item| {
            count += 1;
            write_item(item);
        });
        exe::crt_fseek(f, place, 0);
        if exe::crt_fwrite(&mut count as *mut u32 as usize, 4, 1, f) != 1 {
            return friendly_save_failed(f, lines.1);
        }
    }
    exe::crt_fseek(f, 0, 2);
    true
}

/// The 127 tournament slots (`friendly::TOURNAMENT_SLOTS`) written at once with their clubs as ids: a slot in use
/// (`tournament_slot::HOST`) gets its host and its four invited clubs (`tournament_slot::INVITED_CLUBS`) as ids, a free
/// one host −1 (its invited clubs as they are); after the write the clubs back and each slot's competition
/// (`tournament_slot::COMPETITION`) saved by its virtual `comp_vtable::SAVE_TO_TEMP_FILE`(1).
unsafe fn save_tournament_slots(this: Friendly, f: usize) {
    let slots = || (0..0x7f).map(|i| this.addr() + friendly::TOURNAMENT_SLOTS + i * 0x69);
    for slot in slots() {
        let in_use = read_usize(slot + tournament_slot::HOST) != 0;
        club_to_id(slot + tournament_slot::HOST);
        if in_use {
            for j in 0..4 {
                club_to_id(slot + tournament_slot::INVITED_CLUBS + 4 * j);
            }
        }
    }
    exe::crt_fwrite(this.addr() + friendly::TOURNAMENT_SLOTS, 0x69, 0x7f, f);
    for slot in slots() {
        id_to_club(slot + tournament_slot::HOST);
        if read_usize(slot + tournament_slot::HOST) == 0 {
            continue;
        }
        for j in 0..4 {
            id_to_club(slot + tournament_slot::INVITED_CLUBS + 4 * j);
        }
        let comp = read_usize(slot + tournament_slot::COMPETITION);
        if comp != 0 {
            let save: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(comp) + comp_vtable::SAVE_TO_TEMP_FILE));
            save(comp, 1);
        }
    }
}

/// The three clubs of a friendly match record (`friendly_match::REQUESTING_CLUB`, `HOME_CLUB`, `AWAY_CLUB`) as ids, or
/// back.
unsafe fn friendly_match_clubs(m: usize, convert: unsafe fn(usize)) {
    for field in [friendly_match::REQUESTING_CLUB, friendly_match::HOME_CLUB, friendly_match::AWAY_CLUB] {
        convert(m + field);
    }
}

/// A node of the scheduled slot list as ids (`to_ids`) or back: the club +0, the nation +4 (its id at +0; back,
/// [`G_NATION_TABLE`] + `nation::SIZE` × id; −1 none) and the three clubs of each slot (the count a signed byte at +8).
unsafe fn scheduled_node_ids(node: usize, to_ids: bool) {
    let nation = node + SCHEDULED_SLOTS_NATION;
    if to_ids {
        club_to_id(node);
        let p = read_usize(nation);
        write_u32(nation, if p != 0 { read_u32(p) } else { u32::MAX });
    } else {
        id_to_club(node);
        let id = read_i32(nation);
        write_usize(nation, if id == -1 { 0 } else { read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE)) });
    }
    let mut c: i8 = 0;
    while c < read_i8(node + SCHEDULED_SLOT_COUNT) {
        let slot = node + SCHEDULED_SLOTS + (c as isize as usize).wrapping_mul(SCHEDULED_SLOT_SIZE);
        friendly_match_clubs(slot, if to_ids { club_to_id } else { id_to_club });
        c += 1;
    }
}

/// Rule: saving a game writes the friendlies: the pre-season base date and the first tournament id, the AI friendly
/// plan, the 127 tournament slots (and each tournament's competition), the arranged friendlies, the pending
/// tournaments, the requests awaiting a reply, the scheduled slots, the last day of the international friendlies and,
/// per nation, its list of international friendlies; the clubs and nations go as their ids.
///
/// `Friendly::save_to_temp_file(unused)`: CM3_TEMP\Friendly.tmp (`open_temp_file_for_write`; none: the friendly.cpp box
/// of line 0x135b and nothing else). With fwrite (`exe::crt_fwrite`, cdecl): `friendly::PRESEASON_BASE_DATE` (8),
/// `friendly::FIRST_TOURNAMENT_COMP_ID` (2), `friendly::PLANNED_NEXT_INDEX`, `PLANNED_COUNT` and `PLANNED_CAPACITY` (4
/// each) and, with a count and the array, the plan (`friendly::PLANNED_FRIENDLIES`, 0x14 bytes each);
/// `save_tournament_slots`; the lists by `save_counted_list`: the friendlies (`friendly::FRIENDLY_MATCH_LIST`, 0x19
/// bytes each with its three clubs as ids; boxes 0x13b6 and 0x13ee), the pending tournaments (the slot's
/// `tournament_slot::SLOT_INDEX` byte; 0x1401 and 0x141c), the requests awaiting a reply (the record's date, 8 bytes,
/// then its requesting club as an id; 0x142f and 0x1457) and the scheduled slots (`SCHEDULED_NODE_SIZE` bytes as
/// `scheduled_node_ids`; 0x146a and 0x14c6), each record's ids turned back after it is written; then
/// `friendly::INTL_LAST_DAY_DONE` (2; box 0x14d2) and, for each of the `G_NATION_COUNT` nation lists
/// (`friendly::NATION_LISTS`), its count word (box 0x14de) and its items, 4 bytes each (box 0x14ea). The file closed
/// (`exe::crt_fclose`). A box closes the file and stops (the ids of the records already turned back stay right).
/// Original: `Friendly::save_to_temp_file` (0x5c9650, friendly.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn friendly_save_to_temp_file(this: usize, _unused: u32) {
    let this = Friendly(this);
    let a = this.addr();
    let f = open_temp_file_for_write(FRIENDLY_TEMP_FILE);
    if f == 0 {
        return show_error_at(FRIENDLY_SOURCE, 0x135b);
    }
    exe::crt_fwrite(a + friendly::PRESEASON_BASE_DATE, 8, 1, f);
    exe::crt_fwrite(a + friendly::FIRST_TOURNAMENT_COMP_ID, 2, 1, f);
    for field in [friendly::PLANNED_NEXT_INDEX, friendly::PLANNED_COUNT, friendly::PLANNED_CAPACITY] {
        exe::crt_fwrite(a + field, 4, 1, f);
    }
    if this.planned_count() != 0 && this.planned_friendlies() != 0 {
        exe::crt_fwrite(this.planned_friendlies(), 0x14, this.planned_count() as u32, f);
    }
    save_tournament_slots(this, f);
    let friendlies = save_counted_list(f, this.friendly_match_list().addr(), (0x13b6, 0x13ee), |m| {
        friendly_match_clubs(m, club_to_id);
        exe::crt_fwrite(m, 0x19, 1, f);
        friendly_match_clubs(m, id_to_club);
    });
    if !friendlies
        || !save_counted_list(f, this.pending_tournament_list().addr(), (0x1401, 0x141c), |item| {
            exe::crt_fwrite(read_usize(item) + tournament_slot::SLOT_INDEX, 1, 1, f);
        })
        || !save_counted_list(f, this.request_reply_list(), (0x142f, 0x1457), |item| {
            let m = read_usize(item);
            club_to_id(m + friendly_match::REQUESTING_CLUB);
            exe::crt_fwrite(m + friendly_match::DATE, 8, 1, f);
            exe::crt_fwrite(m + friendly_match::REQUESTING_CLUB, 4, 1, f);
            id_to_club(m + friendly_match::REQUESTING_CLUB);
        })
        || !save_counted_list(f, this.scheduled_slot_list(), (0x146a, 0x14c6), |node| {
            scheduled_node_ids(node, true);
            exe::crt_fwrite(node, SCHEDULED_NODE_SIZE, 1, f);
            scheduled_node_ids(node, false);
        })
    {
        return;
    }
    if exe::crt_fwrite(a + friendly::INTL_LAST_DAY_DONE, 2, 1, f) != 1 {
        friendly_save_failed(f, 0x14d2);
        return;
    }
    for i in 0..read_i32(G_NATION_COUNT).max(0) as usize {
        let list = read_usize(this.nation_lists() + 4 * i);
        let mut count = read_u16(list + llist::COUNT);
        if exe::crt_fwrite(&mut count as *mut u16 as usize, 2, 1, f) != 1 {
            friendly_save_failed(f, 0x14de);
            return;
        }
        let mut failed = false;
        Llist(list).set_cursor(Llist(list).head());
        loop {
            let item = exe::llist_get_next(list);
            if item == 0 {
                break;
            }
            if exe::crt_fwrite(item, 4, 1, f) != 1 {
                failed = true;
                break;
            }
        }
        if failed {
            friendly_save_failed(f, 0x14ea);
            return;
        }
    }
    covered!("sbH_4 friendlies saved");
    exe::crt_fclose(f);
}

/// Comparer for `friendly_save_to_temp_file`: only the original (it writes CM3_TEMP\Friendly.tmp, which cannot be
/// written twice); the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_friendly_save_to_temp_file(this: usize, unused: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, unused)
}

/// The constructors that rebuild a friendly tournament's competition from its competition record on loading: a
/// MiniLeague (operator new 0xf6) and a MiniCup (0xbc), thiscall(record), ret 4 [inference: the load constructors, one
/// argument, next to the ones of Friendly::process_tournament_replies].
const FN_MINI_LEAGUE_FROM_RECORD: usize = 0x766800;
const FN_MINI_CUP_FROM_RECORD: usize = 0x765e30;

/// Reads `count` items of `size` bytes into `at` (`exe::crt_fread`, cdecl); true when all `count` came.
unsafe fn read_items(at: usize, size: u32, count: u32, f: usize) -> bool {
    exe::crt_fread(at, size, count, f) == count
}

/// Frees one of the friendlies' lists (LLIST::~LLIST `exe::drop_llist` and `exe::crt_operator_delete`) and clears its
/// place.
unsafe fn free_friendly_list(at: usize) {
    let list = read_usize(at);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        write_usize(at, 0);
    }
}

/// The tournament slots back from the file: each host and, for a slot in use, its four invited clubs from their ids;
/// a slot whose saved competition pointer is not null gets its competition rebuilt from the competition record of its
/// id ([`G_CLUB_COMP_TABLE`] + 0x6b × (`friendly::FIRST_TOURNAMENT_COMP_ID` + slot)): kind 2 a MiniLeague, kind 1 a
/// MiniCup (`exe::crt_operator_new`; none: 0); any other kind keeps the stale pointer (a defect of the original).
unsafe fn load_tournament_slots(this: Friendly) {
    for i in 0..0x7f {
        let slot = this.addr() + friendly::TOURNAMENT_SLOTS + i * 0x69;
        id_to_club(slot + tournament_slot::HOST);
        if read_usize(slot + tournament_slot::HOST) == 0 {
            continue;
        }
        for j in 0..4 {
            id_to_club(slot + tournament_slot::INVITED_CLUBS + 4 * j);
        }
        if read_usize(slot + tournament_slot::COMPETITION) == 0 {
            continue;
        }
        let (size, build) = match read_u8(slot + tournament_slot::KIND) {
            2 => (0xf6, FN_MINI_LEAGUE_FROM_RECORD),
            1 => (0xbc, FN_MINI_CUP_FROM_RECORD),
            _ => continue,
        };
        let id = (this.first_tournament_comp_id() as i32).wrapping_add(i as i32);
        let record = read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b));
        let p = exe::crt_operator_new(size);
        let comp = if p != 0 {
            let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(build);
            construct(p, record)
        } else {
            0
        };
        write_usize(slot + tournament_slot::COMPETITION, comp);
    }
}

/// One of the friendlies' lists from the file: its count (4 bytes; short: the box of line `line` and false), the old
/// list freed (`free_friendly_list`) and, with a count, a new one (`alloc_list`) given `read_item` that many times.
unsafe fn load_counted_list(f: usize, at: usize, line: u32, mut read_item: impl FnMut(usize)) -> bool {
    let mut count = 0i32;
    if !read_items(&mut count as *mut i32 as usize, 4, 1, f) {
        show_error_at(FRIENDLY_SOURCE, line);
        return false;
    }
    free_friendly_list(at);
    if count != 0 {
        write_usize(at, alloc_list());
        for _ in 0..count.max(0) {
            read_item(read_usize(at));
        }
    }
    true
}

/// Rule: loading a saved game reads the friendlies back in the order they were saved, rebuilds each tournament's
/// competition and the lists, turning the ids back into clubs and nations.
///
/// `Friendly::load_from_temp_file(unused)` -> 1, or 0: CM3_TEMP\Friendly.tmp (`temp_file_open_for_read`; none: 0, no
/// box). With fread (`exe::crt_fread`, cdecl): `friendly::PRESEASON_BASE_DATE` (8), `FIRST_TOURNAMENT_COMP_ID` (2),
/// `PLANNED_NEXT_INDEX`, `PLANNED_COUNT`, `PLANNED_CAPACITY` (4 each, unchecked); with a capacity above 0, the plan
/// array (`exe::crt_malloc`(capacity × 0x14); none: the friendly.cpp box of line 0x121a and 0) and `PLANNED_COUNT`
/// entries of 0x14 bytes into it; the slots (0x7f of 0x69 bytes, `load_tournament_slots`); the lists by
/// `load_counted_list`: the friendlies (0x19 bytes each, the three clubs from their ids, LLIST::add_tail
/// `exe::llist_add_tail`; box 0x1257), the pending tournaments (a slot index byte: the slot if in use, else none, as a
/// 4-byte item; box 0x1287), the requests awaiting a reply (a date and a club id: the friendly match found by
/// Friendly::find_friendly_match `exe::friendly_find_friendly_match`(date, club), as a 4-byte item; box 0x12ad) and the
/// scheduled slots (`SCHEDULED_NODE_SIZE` bytes, `scheduled_node_ids` back; box 0x12db); `friendly::INTL_LAST_DAY_DONE`
/// (2; box 0x131b); for each of the `G_NATION_COUNT` nations a new list into `friendly::NATION_LISTS` (the old ones are
/// not freed), its count word (box 0x1329) and its 4-byte items (box 0x1334). The file closed (`exe::crt_fclose`) and 1.
/// Defects of the original: after the boxes of lines 0x121a, 0x1257, 0x1287, 0x12ad and 0x12db the file stays open (the
/// handle leaks; the last three boxes close it); the record reads inside the lists are not checked.
/// The original builds the 10 slots of its local scheduled node with a constructor (0x645800) and an empty destructor
/// (0x5a53c0); every byte of the node is read from the file before use, so the copy uses a plain buffer.
/// Original: `Friendly::load_from_temp_file` (0x5c8a00, friendly.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn friendly_load_from_temp_file(this: usize, _unused: u32) -> u32 {
    let this = Friendly(this);
    let a = this.addr();
    let f = temp_file_open_for_read(FRIENDLY_TEMP_FILE);
    if f == 0 {
        covered!("sbH_4 friendlies without the file");
        return 0;
    }
    read_items(a + friendly::PRESEASON_BASE_DATE, 8, 1, f);
    read_items(a + friendly::FIRST_TOURNAMENT_COMP_ID, 2, 1, f);
    for field in [friendly::PLANNED_NEXT_INDEX, friendly::PLANNED_COUNT, friendly::PLANNED_CAPACITY] {
        read_items(a + field, 4, 1, f);
    }
    let capacity = this.planned_capacity() as i32;
    if capacity > 0 {
        let plan = exe::crt_malloc((capacity as u32).wrapping_mul(0x14));
        this.set_planned_friendlies(plan);
        if plan == 0 {
            show_error_at(FRIENDLY_SOURCE, 0x121a);
            return 0;
        }
        read_items(plan, 0x14, this.planned_count() as u32, f);
    }
    read_items(a + friendly::TOURNAMENT_SLOTS, 0x69, 0x7f, f);
    load_tournament_slots(this);
    let mut m = [0u8; 0x19];
    let mut node = [0u8; SCHEDULED_NODE_SIZE as usize];
    let ok = load_counted_list(f, a + friendly::FRIENDLY_MATCH_LIST, 0x1257, |list| {
        read_items(m.as_mut_ptr() as usize, 0x19, 1, f);
        friendly_match_clubs(m.as_mut_ptr() as usize, id_to_club);
        exe::llist_add_tail(list, m.as_mut_ptr() as usize, 0x19);
    }) && load_counted_list(f, a + friendly::PENDING_TOURNAMENT_LIST, 0x1287, |list| {
        let mut index = 0i8;
        read_items(&mut index as *mut i8 as usize, 1, 1, f);
        let slot = a + friendly::TOURNAMENT_SLOTS + (index as isize as usize).wrapping_mul(0x69);
        let mut item = if read_usize(slot + tournament_slot::HOST) != 0 { slot } else { 0 };
        exe::llist_add_tail(list, &mut item as *mut usize as usize, 4);
    }) && load_counted_list(f, a + friendly::REQUEST_REPLY_LIST, 0x12ad, |list| {
        let (mut date, mut club) = ([0u8; 8], 0usize);
        read_items(date.as_mut_ptr() as usize, 8, 1, f);
        read_items(&mut club as *mut usize as usize, 4, 1, f);
        id_to_club(&mut club as *mut usize as usize);
        let mut item = exe::friendly_find_friendly_match(a, date.as_mut_ptr() as usize, club);
        exe::llist_add_tail(list, &mut item as *mut usize as usize, 4);
    }) && load_counted_list(f, a + friendly::SCHEDULED_SLOT_LIST, 0x12db, |list| {
        read_items(node.as_mut_ptr() as usize, SCHEDULED_NODE_SIZE, 1, f);
        scheduled_node_ids(node.as_mut_ptr() as usize, false);
        exe::llist_add_tail(list, node.as_mut_ptr() as usize, SCHEDULED_NODE_SIZE);
    });
    if !ok {
        return 0;
    }
    let failed = |line: u32| -> u32 {
        show_error_at(FRIENDLY_SOURCE, line);
        exe::crt_fclose(f);
        0
    };
    if !read_items(a + friendly::INTL_LAST_DAY_DONE, 2, 1, f) {
        return failed(0x131b);
    }
    for i in 0..read_i32(G_NATION_COUNT).max(0) as usize {
        write_usize(this.nation_lists() + 4 * i, alloc_list());
        let mut count = 0i16;
        if !read_items(&mut count as *mut i16 as usize, 2, 1, f) {
            return failed(0x1329);
        }
        for _ in 0..count.max(0) {
            let mut item = 0u32;
            if !read_items(&mut item as *mut u32 as usize, 4, 1, f) {
                return failed(0x1334);
            }
            exe::llist_add_tail(read_usize(this.nation_lists() + 4 * i), &mut item as *mut u32 as usize, 4);
        }
    }
    covered!("sbH_4 friendlies loaded");
    exe::crt_fclose(f);
    1
}

/// Comparer for `friendly_load_from_temp_file`: only the original (it frees and builds the lists and the tournaments'
/// competitions); the copy is proven by the load proof of the save.
pub(crate) unsafe extern "thiscall" fn compare_friendly_load_from_temp_file(this: usize, unused: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%)(this, unused)
}
