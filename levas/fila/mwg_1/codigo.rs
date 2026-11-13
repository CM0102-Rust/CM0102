// ---------- batch %%LEVA%%: virtual players for a match: topping up and releasing the squads (virtual_staff.cpp) ----------

/// Exe addresses the fixture side of the virtual staff uses that have no name in exe_fns.rs.
pub(crate) mod virtual_fixture_fns {
    pub(crate) const GET_ID_BLOCK: usize = 0x90e760; // VIRTUAL_STAFF::get_id_block
    pub(crate) const RELOCATE_VIRTUAL_PLAYERS: usize = 0x90e9a0; // VIRTUAL_STAFF::relocate_virtual_players
    pub(crate) const TOP_UP_CLUB_SQUAD: usize = 0x90dae0; // VIRTUAL_STAFF::top_up_club_squad
    pub(crate) const VIRTUAL_STAFF_CPP: usize = 0xad439c; // "E:\dev\CM3\cm3 00-01\cm3\code\virtual_staff.cpp"
    /// The fixture list of the VIRTUAL_STAFF object: an inline LLIST of 6-byte nodes (fixture pointer, id block slot).
    pub(crate) const FIXTURE_LIST: usize = 0x5c;
    /// The 17 viewed club slots of the VIRTUAL_STAFF object (one per human manager slot and one spare).
    pub(crate) const VIEWED_CLUBS: usize = 0x18;
    /// The fewest players a side may have once topped up before the game reports an error and starts again.
    pub(crate) const MIN_SQUAD_AFTER_TOP_UP: u8 = 15;
}

/// The first staff and player ids of the block of 25 virtual records of a fixture side or of a viewed club slot
/// (`VIRTUAL_STAFF::get_id_block`, still the exe's): `slot` is passed as the original's whole register.
unsafe fn virtual_id_block(this: usize, slot: u32, is_fixture: u32, is_home: u32) -> (u32, u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u32, *mut u32) = core::mem::transmute(virtual_fixture_fns::GET_ID_BLOCK);
    let (mut staff_id, mut player_id) = (0u32, 0u32);
    f(this, slot, is_fixture, is_home, &mut staff_id, &mut player_id);
    (staff_id, player_id)
}

/// The fixture's node in the VIRTUAL_STAFF fixture list, or 0: the cursor goes back to the head first.
unsafe fn virtual_fixture_node(this: usize, fixture: usize) -> usize {
    let list = Llist(this + virtual_fixture_fns::FIXTURE_LIST);
    list.set_cursor(list.head());
    loop {
        let node = exe::llist_get_next(list);
        if node == 0 || read_usize(node) == fixture {
            return node;
        }
    }
}

/// Takes every virtual player (staff id at or above the staff count) out of the club's 50 squad slots, and out of
/// its team selection twice (`exe::tactics_remove_player_from_selection` with 1 and then 0).
unsafe fn drop_club_virtual_players(club: Club) {
    for i in 0..50 {
        let s = club.squad(i);
        if s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT) {
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, s, 1, 1);
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, s, 0, 1);
            club.set_squad(i, 0);
        }
    }
}

/// Puts a fixture on the virtual staff list and tops up both sides' squads with virtual players.
///
/// `VIRTUAL_STAFF::fill_fixture_squads(fixture, slot, no_check)`. A fixture already on the list (`FIXTURE_LIST`)
/// returns at once. The slot of the id block: when `slot` is -1, a human side gets the slot of its manager (the
/// staff count's low word minus the manager's id (`exe::database_get_club_manager`) minus 1, the home side first,
/// `exe::human_manager_is_human_club` with 0, 1), otherwise -2. Each side's block (`get_id_block` with 1 and home
/// 1 / away 0) feeds `top_up_club_squad` (club, the block's staff and player ids, the fixture, and 1 when the side's
/// `fixture::HOME_SIDE_MARK` / `AWAY_SIDE_MARK` is 0); then the fixture enters the list with its slot. Unless
/// `no_check`, a side left with fewer than 15 players in its squad shows the game's error box (line 0xc3), the
/// fixture is released (`release_fixture`) and filled again with the same slot and no check.
/// Original: `VIRTUAL_STAFF::fill_fixture_squads` (0x90d2f0, virtual_staff.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn fill_fixture_squads(this: usize, fixture: usize, slot: u32, no_check: u8) {
    use virtual_fixture_fns as vf;
    if virtual_fixture_node(this, fixture) != 0 {
        return;
    }
    let home = Club(read_usize(fixture + fixture::HOME_CLUB));
    let away = Club(read_usize(fixture + fixture::AWAY_CLUB));
    let mut slot = slot;
    if slot as i16 == -1 {
        // the original rewrites the low word (staff count - manager id) and then decrements the whole register
        let human_slot = |club: Club, slot: u32| {
            let id = read_u16(exe::database_get_club_manager(club));
            ((slot & 0xffff_0000) | read_u16(G_STAFF_COUNT).wrapping_sub(id) as u32).wrapping_sub(1)
        };
        if exe::human_manager_is_human_club(home, 0, 1, 0, 0) != 0 {
            covered!("mwg_1 human home side");
            slot = human_slot(home, slot);
        } else if exe::human_manager_is_human_club(away, 0, 1, 0, 0) != 0 {
            slot = human_slot(away, slot);
        } else {
            slot = 0xffff_fffe;
        }
    }
    let top_up: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) = core::mem::transmute(vf::TOP_UP_CLUB_SQUAD);
    let (staff_id, player_id) = virtual_id_block(this, slot, 1, 1);
    top_up(this, home.addr(), staff_id, player_id, fixture, (read_u8(fixture + fixture::HOME_SIDE_MARK) == 0) as u32);
    let (staff_id, player_id) = virtual_id_block(this, slot, 1, 0);
    covered!("mwg_1 both sides topped up");
    top_up(this, away.addr(), staff_id, player_id, fixture, (read_u8(fixture + fixture::AWAY_SIDE_MARK) == 0) as u32); // mwg1: the away side's top-up
    let mut node = [0u8; 6];
    node[..4].copy_from_slice(&(fixture as u32).to_le_bytes());
    node[4..].copy_from_slice(&(slot as u16).to_le_bytes());
    exe::llist_add_tail(this + vf::FIXTURE_LIST, node.as_mut_ptr() as usize, 6);
    if no_check != 0 {
        return;
    }
    let filled = |club: Club| (0..50).filter(|&i| club.squad(i) != 0).count() as u8;
    if filled(home) < vf::MIN_SQUAD_AFTER_TOP_UP || filled(away) < vf::MIN_SQUAD_AFTER_TOP_UP {
        covered!("mwg_1 short squad after the top-up");
        show_error_at(vf::VIRTUAL_STAFF_CPP, 0xc3);
        exe::virtual_staff_release_fixture(this, fixture);
        exe::virtual_staff_fill_fixture_squads(this, fixture, slot as i32, 1);
    }
}

/// Comparer for `fill_fixture_squads`.
///
/// The fixture goes on a list (a new node) and the virtual players are created with the generator: it cannot run
/// twice. Only the original runs, and what it guarantees is checked: the fixture is on the list afterwards.
pub(crate) unsafe extern "thiscall" fn compare_fill_fixture_squads(this: usize, fixture: usize, slot: u32, no_check: u8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u8)>(%%I0%%)(this, fixture, slot, no_check);
    let listed = virtual_fixture_node(this, fixture) != 0;
    SCOREBOARD.record(listed, || format!("fill_fixture_squads(fixture {:x}, slot {:#x}, no_check {}): the fixture is not on the list", fixture, slot, no_check));
}

/// Takes a finished fixture's virtual players away and the fixture off the virtual staff list.
///
/// `VIRTUAL_STAFF::release_fixture(fixture)`. Over the 17 viewed club slots (`VIEWED_CLUBS`), the first slot holding
/// the home club (and then, in another slot, the away club) keeps that club's virtual players: they move to the slot's
/// block (`get_id_block` with 0, 0; `relocate_virtual_players`). A side without a slot loses them: every squad slot
/// whose person has a staff id at or above the staff count is emptied and the person taken out of the team selection
/// (`exe::tactics_remove_player_from_selection` with 1, then 0). Last, the fixture's node leaves the list
/// (`exe::llist_remove`).
/// Original: `VIRTUAL_STAFF::release_fixture` (0x90d6b0, virtual_staff.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn release_fixture(this: usize, fixture: usize) {
    use virtual_fixture_fns as vf;
    let home = read_usize(fixture + fixture::HOME_CLUB);
    let away = read_usize(fixture + fixture::AWAY_CLUB);
    let relocate: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(vf::RELOCATE_VIRTUAL_PLAYERS);
    let (mut home_kept, mut away_kept) = (false, false);
    for k in 0..17u32 {
        let viewed = read_usize(this + vf::VIEWED_CLUBS + k as usize * 4);
        let keep = if !home_kept && viewed == home {
            home_kept = true;
            true
        } else if !away_kept && viewed == away {
            away_kept = true;
            true
        } else {
            false
        };
        if keep {
            covered!("mwg_1 viewed club keeps its virtual players");
            let (staff_id, player_id) = virtual_id_block(this, k, 0, 0);
            relocate(this, viewed, staff_id, player_id);
        }
    }
    if !home_kept {
        covered!("mwg_1 side without a viewed slot");
        drop_club_virtual_players(Club(home)); // mwg1: the home side's virtual players leave
    }
    if !away_kept {
        drop_club_virtual_players(Club(away));
    }
    let node = virtual_fixture_node(this, fixture);
    if node != 0 {
        exe::llist_remove(this + vf::FIXTURE_LIST, node);
    }
}

/// Comparer for `release_fixture`.
///
/// The node leaves the list (freed): it cannot run twice. Only the original runs, and what it guarantees is checked:
/// the fixture is no longer on the list.
pub(crate) unsafe extern "thiscall" fn compare_release_fixture(this: usize, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, fixture);
    let listed = virtual_fixture_node(this, fixture) != 0;
    SCOREBOARD.record(!listed, || format!("release_fixture(fixture {:x}): the fixture is still on the list", fixture));
}

/// The position group a virtual player fills, from the player's position ratings (15 or more counts).
///
/// `VIRTUAL_STAFF::get_player_position_group(staff)`: no player data gives 0xff; a goalkeeper (`.goalkeeper()` ≥ 15)
/// 0; a defender (`.defender()` or `.sweeper()` ≥ 15) 1 on the left (`.left_side()`), else 2 on the right
/// (`.right_side()`), else 3 (centre); a midfielder (`.defensive_midfielder()`, `.midfielder()` or
/// `.attacking_midfielder()`) 4 on the left, else 6 on the right, else 5; an attacker (`.attacker()`) 7; anyone else
/// 0xff.
/// Original: `VIRTUAL_STAFF::get_player_position_group` (0x90e020, virtual_staff.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_player_position_group(_this: usize, staff: usize) -> u8 {
    const GOOD: i8 = 15;
    let p = Staff(staff).player();
    if p.is_null() {
        return 0xff;
    }
    if p.goalkeeper() >= GOOD {
        return 0;
    }
    if p.defender() >= GOOD || p.sweeper() >= GOOD {
        covered!("mwg_1 defender");
        return if p.left_side() >= GOOD { 1 } else if p.right_side() >= GOOD { 2 } else { 3 }; // mwg1: the defenders' side
    }
    if p.defensive_midfielder() >= GOOD || p.midfielder() >= GOOD || p.attacking_midfielder() >= GOOD {
        return if p.left_side() >= GOOD { 4 } else if p.right_side() >= GOOD { 6 } else { 5 };
    }
    if p.attacker() >= GOOD { 7 } else { 0xff }
}

compare_value!(compare_get_player_position_group, get_player_position_group, %%I2%%, (staff: usize) -> u8);
