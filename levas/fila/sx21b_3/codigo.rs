// ---------- sx21b_3: the virtual players of a viewed club: removing them, their id block and moving them (virtual_staff.cpp) ----------

/// `Staff::operator=` (thiscall(source), the whole 0x6e-byte person record, the dates through `Date::operator=`).
const FN_STAFF_ASSIGN: usize = 0x5335f0;

/// The viewed club slot of the human manager whose screens are open: 16 (the spare slot) without one, otherwise the
/// low word of `G_STAFF_COUNT` minus the manager's id minus 1 (as a signed word).
pub(crate) unsafe fn current_viewed_club_slot() -> i16 {
    let manager = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
    if manager == 0 {
        0x10
    } else {
        read_u16(G_STAFF_COUNT).wrapping_sub(read_u16(manager)).wrapping_sub(1) as i16
    }
}

/// Clears the current human manager's viewed club slot and takes every virtual player out of a club.
///
/// `VIRTUAL_STAFF::remove_virtual_players(club, unused)`: the slot of `current_viewed_club_slot` in `VIEWED_CLUBS` is
/// cleared; then `drop_club_virtual_players(club)` (each squad person with a staff id at or above `G_STAFF_COUNT` taken
/// out of the team selection twice and his slot emptied).
/// Original: `VIRTUAL_STAFF::remove_virtual_players` (0x90d860, virtual_staff.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn remove_virtual_players(this: usize, club: usize, _unused: u32) {
    let slot = current_viewed_club_slot() as isize;
    write_u32((this as isize + virtual_fixture_fns::VIEWED_CLUBS as isize + slot * 4) as usize, 0); // sx21b_3: the slot is freed
    covered!("sx21b_3 virtual players removed");
    drop_club_virtual_players(Club(club));
}

/// Comparer for `remove_virtual_players`: side by side over the 17 viewed club slots and the club's 50 squad slots
/// (the team selection, taken out by the original first, is left as the original leaves it).
pub(crate) unsafe extern "thiscall" fn compare_remove_virtual_players(this: usize, club: usize, unused: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let regions = [(this + virtual_fixture_fns::VIEWED_CLUBS, 0x44usize), (club + club::SQUAD, 200)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, unused), || remove_virtual_players(this, club, unused));
    SCOREBOARD.record(equal, || format!("remove_virtual_players(club {:#x}): differences in {:?}", club, first_differences(&mo, &mr)));
}

/// The first staff and player ids of the block of 25 virtual records of a fixture side or of a viewed club slot.
///
/// `VIRTUAL_STAFF::get_id_block(slot, is_fixture, is_home, &staff_id, &player_id)`, from `G_STAFF_COUNT` and
/// `G_PLAYER_DATA_COUNT`, with `slot` read as a signed word. A fixture's home side: slot × 50, or 800 for slot -2 and
/// 850 for -3; its away side: slot × 50 + 25, or 825 and 875; other negative slots leave the ids as they are. A viewed
/// club slot: (slot + 36) × 25; a negative one shows the virtual_staff.cpp line 0x5e6 box.
/// Original: `VIRTUAL_STAFF::get_id_block` (0x90e760, virtual_staff.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_id_block(_this: usize, slot: u32, is_fixture: u32, is_home: u32, staff_id: *mut u32, player_id: *mut u32) {
    let slot = slot as i16;
    let offset = if is_fixture != 0 {
        let away = if is_home != 0 { 0 } else { 0x19 };
        if slot >= 0 {
            covered!("sx21b_3 fixture block");
            (slot as i32).wrapping_mul(50) + away // sx21b_3: 50 records a fixture
        } else if slot == -2 {
            if is_home != 0 { 0x320 } else { 0x339 }
        } else if slot == -3 {
            if is_home != 0 { 0x352 } else { 0x36b }
        } else {
            return;
        }
    } else {
        if slot < 0 {
            show_error_at(virtual_fixture_fns::VIRTUAL_STAFF_CPP, 0x5e6);
            return;
        }
        covered!("sx21b_3 viewed club block");
        (slot as i32 + 0x24).wrapping_mul(25)
    };
    *staff_id = read_u32(G_STAFF_COUNT).wrapping_add(offset as u32);
    *player_id = read_u32(G_PLAYER_DATA_COUNT).wrapping_add(offset as u32);
}

/// Comparer for `get_id_block`: the two ids side by side (a viewed club's negative slot, which shows the box: only the
/// original).
pub(crate) unsafe extern "thiscall" fn compare_get_id_block(this: usize, slot: u32, is_fixture: u32, is_home: u32, staff_id: *mut u32, player_id: *mut u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u32, *mut u32)>(%%I1%%);
    if is_fixture == 0 && (slot as i16) < 0 {
        return o(this, slot, is_fixture, is_home, staff_id, player_id);
    }
    let regions = [(staff_id as usize, 4usize), (player_id as usize, 4)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, slot, is_fixture, is_home, staff_id, player_id),
        || get_id_block(this, slot, is_fixture, is_home, staff_id, player_id));
    SCOREBOARD.record(equal, || format!("get_id_block(slot {:#x}, fixture {}, home {}) original={:x?} rust={:x?}", slot, is_fixture, is_home, mo, mr));
}

/// Moves a club's virtual players into the block of records that starts at the given ids, unless they already sit in
/// that block.
///
/// `VIRTUAL_STAFF::relocate_virtual_players(club, staff_id, player_id)`. Over the club's 50 squad slots, each virtual
/// player (staff id at or above `G_STAFF_COUNT`): one whose id is under `staff_id + 25` ends the move at once (the
/// block is already in place). Otherwise he leaves the team selection (twice), his person record is copied
/// (`Staff::operator=`) into the record of `staff_id` (which gets that id), his 0x46-byte player sheet into the sheet of
/// `player_id` (which gets that id and becomes the record's `staff::PLAYER`), the squad slot points to the new record,
/// and both ids move on by one.
/// Original: `VIRTUAL_STAFF::relocate_virtual_players` (0x90e9a0, virtual_staff.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn relocate_virtual_players(_this: usize, club: usize, staff_id: u32, player_id: u32) {
    let club = Club(club);
    let assign: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_ASSIGN);
    let (mut staff_id, mut player_id) = (staff_id as i32, player_id);
    for i in 0..50 {
        let s = club.squad(i);
        if s == 0 || read_i32(s) < read_i32(G_STAFF_COUNT) {
            continue;
        }
        if read_i32(s).wrapping_sub(staff_id) < 0x19 { // sx21b_3: already in the block
            return;
        }
        covered!("sx21b_3 virtual player moved");
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, s, 1, 1);
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, club.squad(i), 0, 1);
        let record = read_usize(G_STAFF_TABLE).wrapping_add((staff_id as usize).wrapping_mul(staff::SIZE));
        assign(record, club.squad(i));
        write_i32(record, staff_id);
        club.set_squad(i, record);
        let sheet = read_usize(G_PLAYER_DATA_TABLE).wrapping_add((player_id as usize).wrapping_mul(0x46));
        core::ptr::copy_nonoverlapping(read_usize(record + staff::PLAYER) as *const u8, sheet as *mut u8, 0x46);
        write_u32(sheet, player_id);
        staff_id = staff_id.wrapping_add(1);
        write_usize(record + staff::PLAYER, sheet);
        player_id = player_id.wrapping_add(1);
    }
}

/// Comparer for `relocate_virtual_players`: side by side over the club's squad slots and the staff and player records
/// of the block, as many as the club has virtual players (the team selection is left as the original leaves it).
pub(crate) unsafe extern "thiscall" fn compare_relocate_virtual_players(this: usize, club: usize, staff_id: u32, player_id: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I2%%);
    let n = (0..50).filter(|&i| {
        let s = Club(club).squad(i);
        s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT)
    }).count();
    let regions = [
        (club + club::SQUAD, 200usize),
        (read_usize(G_STAFF_TABLE) + staff_id as usize * staff::SIZE, n * staff::SIZE),
        (read_usize(G_PLAYER_DATA_TABLE) + player_id as usize * 0x46, n * 0x46),
    ];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, staff_id, player_id),
        || relocate_virtual_players(this, club, staff_id, player_id));
    SCOREBOARD.record(equal, || format!("relocate_virtual_players(club {:#x}, staff {}, player {}): differences in {:?}", club, staff_id, player_id, first_differences(&mo, &mr)));
}
