// ---------- sx21b_7: the training manager on the training screens: schedules, coaches and position training (training_manager.cpp) ----------

/// "...\training_manager.cpp", for the error box.
const TRAINING_MANAGER_FILE: usize = 0xa8ccd0;
/// The INJURY object (`Injury::set_training_load` and `get_player_injury_status` work on it).
const G_INJURY: usize = 0xb63d28;
/// `TRAINING_MANAGER::update_position_training` (thiscall(player), ret 4; this item).
const FN_UPDATE_POSITION_TRAINING: usize = 0x8b42d0;
/// `TRAINING_MANAGER::set_position_training_rate` (thiscall(item, start), ret 8; this item).
const FN_SET_POSITION_TRAINING_RATE: usize = 0x8b4450;
/// `TRAINING_MANAGER::add_position_training` (thiscall(item), ret 4; this item).
const FN_ADD_POSITION_TRAINING: usize = 0x8b49b0;
/// `training_compare_entries_by_staff_id` (cdecl, for `qsort` and `bsearch`; this item).
const FN_COMPARE_TRAINING_ENTRIES: usize = 0x8b4e00;

/// A player's training entry (6 bytes, by his player sheet id) in `PLAYER_ENTRIES`: +0 the schedule slot, +5 flags.
unsafe fn training_entry(this: usize, p: Staff) -> usize {
    read_usize(this + training_manager_fields::PLAYER_ENTRIES).wrapping_add((read_i32(p.player().addr()) as usize).wrapping_mul(6))
}

/// Moves a player to another schedule slot, then refreshes his training load and position training and drops the
/// position training items that no longer train.
///
/// `TRAINING_MANAGER::set_player_schedule(player, slot)`: no player or player sheet: the box (line 0x2eb). A slot other
/// than his entry's first catches his training up (`exe::training_manager_update_player_training(.., player, 0)`) and
/// takes the slot. Then the load of his schedule (`get_player_schedule`, `get_training_load`) goes to
/// `Injury::set_training_load`, and `update_position_training`. Last, each position training item with a rate of 0 is
/// replaced by the last item and the count drops (the moved item is not looked at again); when the count changed the
/// items are sorted again (`qsort` by staff id).
/// Original: `TRAINING_MANAGER::set_player_schedule` (0x8b2250, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_player_schedule(this: usize, player: usize, slot: u8) {
    use training_manager_fields as tm;
    let p = Staff(player);
    if player == 0 || p.player().addr() == 0 {
        show_error_at(TRAINING_MANAGER_FILE, 0x2eb);
        return;
    }
    let entry = training_entry(this, p);
    if slot != read_u8(entry) {
        covered!("sx21b_7 player moved to another schedule");
        exe::training_manager_update_player_training(this, player, 0);
        write_u8(entry, slot);
    }
    let load = exe::training_schedule_get_training_load(exe::training_manager_get_player_schedule(this, player));
    exe::injury_set_training_load(read_usize(G_INJURY), player, load as u32);
    let update: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_UPDATE_POSITION_TRAINING);
    update(this, player);
    let start = read_i32(this + tm::POSITION_TRAINING_COUNT);
    let mut i = 0i32;
    while i < read_i32(this + tm::POSITION_TRAINING_COUNT) {
        let items = read_usize(this + tm::POSITION_TRAINING_ITEMS);
        let item = items + i as usize * 8;
        if read_u8(item + 7) == 0 { // sx21b_7: an item that no longer trains
            let last = items + (read_i32(this + tm::POSITION_TRAINING_COUNT) as usize).wrapping_sub(1) * 8;
            write_u32(item, read_u32(last));
            write_u32(item + 4, read_u32(last + 4));
            write_i32(this + tm::POSITION_TRAINING_COUNT, read_i32(this + tm::POSITION_TRAINING_COUNT) - 1);
        }
        i += 1;
    }
    let count = read_i32(this + tm::POSITION_TRAINING_COUNT);
    if start != count {
        exe::crt_qsort(read_usize(this + tm::POSITION_TRAINING_ITEMS), count as u32, 8, FN_COMPARE_TRAINING_ENTRIES);
    }
}

/// Comparer for `set_player_schedule`: it catches the player's training up and changes the items, so only the
/// original runs; then the player's entry must hold the slot.
pub(crate) unsafe extern "thiscall" fn compare_set_player_schedule(this: usize, player: usize, slot: u8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u8)>(%%I0%%)(this, player, slot);
    if player != 0 && Staff(player).player().addr() != 0 {
        let now = read_u8(training_entry(this, Staff(player)));
        SCOREBOARD.record(now == slot, || format!("set_player_schedule (training screen): player {:#x} slot {} but the entry holds {}", player, slot, now));
    }
}

/// Copies an edited 54-byte coach assignment table into a human manager's slot and refreshes it against the club's
/// coaches.
///
/// `TRAINING_MANAGER::set_coach_assignments(manager, table)`: the manager's table (`get_coach_assignments`); none: the
/// box (line 0x31a). The 54 bytes copied in, then `update_coach_assignments(manager's club job, manager)`.
/// Original: `TRAINING_MANAGER::set_coach_assignments` (0x8b23b0, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_coach_assignments(this: usize, manager: usize, table: usize) {
    let slot = exe::training_manager_get_coach_assignments(this, manager);
    if slot == 0 {
        show_error_at(TRAINING_MANAGER_FILE, 0x31a);
        return;
    }
    covered!("sx21b_7 coach assignments set");
    core::ptr::copy_nonoverlapping(table as *const u8, slot as *mut u8, 0x36); // sx21b_7: the 54-byte table
    exe::training_manager_update_coach_assignments(this, Staff(manager).club_job().addr(), manager as u32);
}

/// Comparer for `set_coach_assignments`: it refreshes the table against the coaches (other routines), so only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_set_coach_assignments(this: usize, manager: usize, table: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, manager, table)
}

/// Fills a 50-entry list (5 bytes each: the person, his schedule slot) with the club's squad players, including those
/// of its linked B team.
///
/// `TRAINING_MANAGER::collect_squad_schedules(club, out)`: no club: the box (line 0x5c9). The squad
/// (`exe::database_collect_club_squad(club, .., 0)`) and the B team (`exe::club_get_linked_b_team(club, 0, 1)`); for
/// each of the 50: a real person (staff id below `G_STAFF_COUNT`) of the club or of the B team (`club_job()`) gets his
/// pointer and his entry's slot; one without a player sheet shows the box (line 0x5df); every other entry gets a null
/// person (its slot byte left as it was).
/// Original: `TRAINING_MANAGER::collect_squad_schedules` (0x8b3410, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn collect_squad_schedules(this: usize, club: usize, out: usize) {
    if club == 0 {
        show_error_at(TRAINING_MANAGER_FILE, 0x5c9);
        return;
    }
    let b_team = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
    let mut squad = [0usize; 50];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 0);
    for (k, &s) in squad.iter().enumerate() {
        let e = out + k * 5;
        write_u32(e, 0);
        if s == 0 || read_i32(s) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let p = Staff(s);
        if p.player().addr() == 0 {
            show_error_at(TRAINING_MANAGER_FILE, 0x5df);
            continue;
        }
        let job = p.club_job().addr();
        if job == club || (b_team != 0 && job == b_team) {
            covered!("sx21b_7 squad player listed");
            write_usize(e, s);
            write_u8(e + 4, read_u8(training_entry(this, p))); // sx21b_7: his schedule slot
        }
    }
}

/// Comparer for `collect_squad_schedules`: the 250-byte list side by side.
pub(crate) unsafe extern "thiscall" fn compare_collect_squad_schedules(this: usize, club: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    if club == 0 {
        return o(this, club, out);
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(out, 250usize)], || o(this, club, out), || collect_squad_schedules(this, club, out));
    SCOREBOARD.record(equal, || format!("collect_squad_schedules (training screen): club {:#x}, differences in {:?}", club, first_differences(&mo, &mr)));
}

/// Copies a human manager's 54-byte coach assignment table into a buffer for editing.
///
/// `TRAINING_MANAGER::get_coach_assignments_copy(manager, out)`: the manager's table (`get_coach_assignments`) copied
/// to `out`; none: the box (line 0x604).
/// Original: `TRAINING_MANAGER::get_coach_assignments_copy` (0x8b35a0, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_coach_assignments_copy(this: usize, manager: usize, out: usize) {
    let slot = exe::training_manager_get_coach_assignments(this, manager);
    if slot == 0 {
        show_error_at(TRAINING_MANAGER_FILE, 0x604);
        return;
    }
    covered!("sx21b_7 coach assignments copied");
    core::ptr::copy_nonoverlapping(slot as *const u8, out as *mut u8, 0x36);
}

/// Comparer for `get_coach_assignments_copy`: the 54 bytes side by side.
pub(crate) unsafe extern "thiscall" fn compare_get_coach_assignments_copy(this: usize, manager: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I3%%);
    let human = manager != 0 && (0..0x10).contains(&read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1));
    if !human {
        return o(this, manager, out);
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(out, 0x36usize)], || o(this, manager, out), || get_coach_assignments_copy(this, manager, out));
    SCOREBOARD.record(equal, || format!("get_coach_assignments_copy (training screen): manager {:#x}, differences in {:?}", manager, first_differences(&mo, &mr)));
}

/// One pass of `update_position_training` over the player's items for the bits `mask` of his schedule's training word:
/// the item of exactly those bits restarts when stopped (rate below 0); any other item with some of those bits stops
/// when training (rate above 0); with none matching and bits other than `alone`, a new item (rate set as started) is
/// added.
unsafe fn update_position_training_pass(this: usize, player: Staff, first: usize, count: i32, wanted: u16, mask: u16, alone: u16) {
    let rate: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SET_POSITION_TRAINING_RATE);
    let add: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_ADD_POSITION_TRAINING);
    let mut found = false;
    for k in 0..count.max(0) as usize {
        let item = first + k * 8;
        let m = read_u16(item + 4);
        if m == wanted {
            found = true;
            if read_i8(item + 7) < 0 {
                rate(this, item, 1);
            }
        } else if m & mask != 0 && read_i8(item + 7) > 0 {
            covered!("sx21b_7 position training stopped");
            rate(this, item, 0); // sx21b_7: the other position stops
        }
    }
    if !found && wanted != 0 && wanted != alone {
        covered!("sx21b_7 position training started");
        let mut item = [0u8; 8];
        item[..4].copy_from_slice(&read_u32(player.addr()).to_le_bytes());
        item[4..6].copy_from_slice(&wanted.to_le_bytes());
        item[6] = 1;
        rate(this, item.as_mut_ptr() as usize, 1);
        add(this, item.as_mut_ptr() as usize);
    }
}

/// Starts or stops a player's position and side training items so that they match his schedule.
///
/// `TRAINING_MANAGER::update_position_training(player)`: his items (`find_position_training`) and his schedule's
/// training word (+0x16 of `get_player_schedule`); one pass for the position bits (0x207f, 0x2000 alone adds
/// nothing) and one for the side bits (0x4a80, 0x4000 alone adds nothing), as in `update_position_training_pass`.
/// Original: `TRAINING_MANAGER::update_position_training` (0x8b42d0, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn update_position_training(this: usize, player: usize) {
    let mut first = 0usize;
    let count = exe::training_manager_find_position_training(this, player, &mut first);
    let schedule = exe::training_manager_get_player_schedule(this, player);
    update_position_training_pass(this, Staff(player), first, count, read_u16(schedule + 0x16) & 0x207f, 0x207f, 0x2000);
    update_position_training_pass(this, Staff(player), first, count, read_u16(schedule + 0x16) & 0x4a80, 0x4a80, 0x4000);
}

/// Comparer for `update_position_training`: it adds and rates items (other routines, reallocations), so only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_update_position_training(this: usize, player: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I4%%)(this, player)
}

/// The learning rate byte (+7) of a position training item: how fast the player learns the position (`start`) or
/// forgets it (negative).
///
/// `TRAINING_MANAGER::set_position_training_rate(item, start)`: a person without a player sheet: the box (line 0x80a)
/// and rate 0. The difficulty from the item's position mask and the player's (`exe::formation_get_position_mask`):
/// goalkeeper either way 0; 0x40: 20; 0x20: 100 from a 0x50 player, else 30; 0x10: 80 from 0x40, 130 from 0x38, else
/// 40; 8: 50 from 0x40, 70 from 0x20, 130 from 0x18, else 50; 4: 40 from 0x40 or 0x20, 60 from 0x10, 100 from 8, else
/// 120; 2: 30 from 0x40, 50 from 0x20, 60 from 0x10, 120 from 8, else 130; any other: 60. The age adds 50 under 17, 40
/// under 19, 30 under 22, -20 under 23 and -60 from 23; the aptitude (`get_position_aptitude`) twice, less 20. The
/// total (a word) above 150, 120, 90, 60, 40 and 20 gives 5, 7, 10, 20, 30 and 40, else 80 when starting; -80, -40,
/// -30, -20, -10 and -7, else -5 when stopping.
/// Original: `TRAINING_MANAGER::set_position_training_rate` (0x8b4450, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_position_training_rate(this: usize, item: usize, start: u32) {
    let person = staff_at(read_i32(item) as isize);
    let sheet = person.player();
    if sheet.addr() == 0 {
        show_error_at(TRAINING_MANAGER_FILE, 0x80a);
        write_u8(item + 7, 0);
        return;
    }
    let m = read_u16(item + 4);
    let player_mask = exe::formation_get_position_mask(sheet);
    let pm = player_mask as u8;
    let difficulty: i32 = if m == 1 || player_mask == 1 {
        0
    } else {
        match m {
            0x40 => 0x14,
            0x20 => if pm & 0x50 != 0 { 0x64 } else { 0x1e },
            0x10 => if pm & 0x40 != 0 { 0x50 } else if pm & 0x38 != 0 { 0x82 } else { 0x28 },
            8 => if pm & 0x40 != 0 { 0x32 } else if pm & 0x20 != 0 { 0x46 } else if pm & 0x18 != 0 { 0x82 } else { 0x32 },
            4 => if pm & 0x60 != 0 { 0x28 } else if pm & 0x10 != 0 { 0x3c } else if pm & 8 != 0 { 0x64 } else { 0x78 },
            2 => if pm & 0x40 != 0 { 0x1e } else if pm & 0x20 != 0 { 0x32 } else if pm & 0x10 != 0 { 0x3c } else if pm & 8 != 0 { 0x78 } else { 0x82 },
            _ => 0x3c,
        }
    };
    let age = person.age() as i8;
    let by_age = if age < 0x11 { 0x32 } else if age < 0x13 { 0x28 } else if age < 0x16 { 0x1e } else if age < 0x17 { -0x14 } else { -0x3c };
    let aptitude = exe::training_manager_get_position_aptitude(this, person.addr(), m as u32);
    let total = ((difficulty + by_age) as i16).wrapping_add((aptitude as i16).wrapping_mul(2)).wrapping_sub(0x14);
    const STEPS: [i16; 6] = [0x96, 0x78, 0x5a, 0x3c, 0x28, 0x14];
    let (rates, last): ([u8; 6], u8) = if start != 0 {
        covered!("sx21b_7 position training rate");
        ([5, 7, 0xa, 0x14, 0x1e, 0x28], 0x50) // sx21b_7: the fastest learner gets 5
    } else {
        ([0xb0, 0xd8, 0xe2, 0xec, 0xf6, 0xf9], 0xfb)
    };
    let rate = STEPS.iter().position(|&s| total > s).map_or(last, |k| rates[k]);
    write_u8(item + 7, rate);
}

/// Comparer for `set_position_training_rate`: the item's 8 bytes side by side.
pub(crate) unsafe extern "thiscall" fn compare_set_position_training_rate(this: usize, item: usize, start: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I5%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(item, 8usize)], || o(this, item, start), || set_position_training_rate(this, item, start));
    SCOREBOARD.record(equal, || format!("set_position_training_rate (training screen): start {}, original {:x?} rust {:x?}", start, mo, mr));
}

/// Adds a position training item to the sorted list unless the player can already play that position.
///
/// `TRAINING_MANAGER::add_position_training(item)`: a mask the player already has
/// (`exe::formation_get_position_mask`) adds nothing. A full list (`POSITION_TRAINING_COUNT` equal to the capacity at
/// +0x6529) grows by 10 (`malloc` or `realloc`; none: the box, line 0x973). The item is appended, the list sorted
/// (`qsort` by staff id) and bit 0x10 of the player's entry byte +5 set.
/// Original: `TRAINING_MANAGER::add_position_training` (0x8b49b0, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_position_training(this: usize, item: usize) {
    use training_manager_fields as tm;
    let person = staff_at(read_i32(item) as isize);
    if read_u16(item + 4) & exe::formation_get_position_mask(person.player()) != 0 {
        return;
    }
    if read_u32(this + 0x6529) == read_u32(this + tm::POSITION_TRAINING_COUNT) {
        let capacity = read_u32(this + 0x6529).wrapping_add(0xa); // sx21b_7: ten more items
        write_u32(this + 0x6529, capacity);
        let old = read_usize(this + tm::POSITION_TRAINING_ITEMS);
        let items = if old == 0 { exe::crt_malloc(capacity.wrapping_mul(8)) } else { exe::crt_realloc(old, capacity.wrapping_mul(8)) };
        write_usize(this + tm::POSITION_TRAINING_ITEMS, items);
        if items == 0 {
            show_error_at(TRAINING_MANAGER_FILE, 0x973);
            return;
        }
    }
    covered!("sx21b_7 position training added");
    let count = read_u32(this + tm::POSITION_TRAINING_COUNT) as usize;
    let items = read_usize(this + tm::POSITION_TRAINING_ITEMS);
    write_u32(items + count * 8, read_u32(item));
    write_u32(items + count * 8 + 4, read_u32(item + 4));
    write_u32(this + tm::POSITION_TRAINING_COUNT, count as u32 + 1);
    exe::crt_qsort(items, count as u32 + 1, 8, FN_COMPARE_TRAINING_ENTRIES);
    let entry = training_entry(this, staff_at(read_i32(item) as isize));
    write_u8(entry + 5, read_u8(entry + 5) | 0x10);
}

/// Comparer for `add_position_training`: it grows the list (reallocation), so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_add_position_training(this: usize, item: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I6%%)(this, item)
}

/// Compares two position training items (or entries keyed alike) by the staff id in their first dword, for `qsort`
/// and `bsearch`: the difference of the two ids.
/// Original: `training_compare_entries_by_staff_id` (0x8b4e00, training_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn training_compare_entries_by_staff_id(a: usize, b: usize) -> i32 {
    read_i32(a).wrapping_sub(read_i32(b)) // sx21b_7: by staff id
}

/// Comparer for `training_compare_entries_by_staff_id`: the value.
pub(crate) unsafe extern "C" fn compare_training_compare_entries_by_staff_id(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I7%%)(a, b);
    let c = training_compare_entries_by_staff_id(a, b);
    SCOREBOARD.record(r == c, || format!("training_compare_entries_by_staff_id original={} rust={}", r, c));
    r
}

/// Sets one of a schedule's five category levels and clears its saved flag.
///
/// `TRAINING_SCHEDULE::set_category_level(category, level)`: the byte at +4 + category (signed) takes the level; the
/// dword at +0x93 (saved to a file) becomes 0.
/// Original: `TRAINING_SCHEDULE::set_category_level` (0x8b4f30, training_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_category_level(this: usize, category: i8, level: u8) {
    covered!("sx21b_7 category level set");
    write_u8((this as isize + 4 + category as isize) as usize, level);
    write_u32(this + 0x93, 0); // sx21b_7: no longer as saved
}

/// Comparer for `set_category_level`: the 0x9b-byte schedule side by side.
pub(crate) unsafe extern "thiscall" fn compare_set_category_level(this: usize, category: i8, level: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u8)>(%%I8%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x9busize)], || o(this, category, level), || set_category_level(this, category, level));
    SCOREBOARD.record(equal, || format!("set_category_level (training screen): category {}, level {}, differences in {:?}", category, level, first_differences(&mo, &mr)));
}
