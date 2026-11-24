// ---------- sx21b_9: the virtual players of the club a human manager views (virtual_staff.cpp) ----------

/// Whether the club's squad holds a virtual player of its own (club or national job) before any virtual player of
/// another club: `Some(true)` some of its own and none foreign, `Some(false)` none at all, `None` a foreign one.
unsafe fn viewed_club_virtual_players(club: Club) -> Option<bool> {
    let mut own = false;
    for k in 0..50 {
        let s = club.squad(k);
        if s == 0 || read_i32(s) < read_i32(G_STAFF_COUNT) {
            continue;
        }
        let p = Staff(s);
        if p.club_job().addr() == club.addr() || p.national_job().addr() == club.addr() {
            own = true;
        } else {
            return None;
        }
    }
    Some(own)
}

/// Tops up the squad of the club that the current human manager views with virtual players, after moving the
/// previously viewed club's virtual players out.
///
/// `VIRTUAL_STAFF::fill_viewed_club_squad(club)`. The manager's slot (`current_viewed_club_slot`) already on this club
/// with virtual players of its own (and no foreign one): nothing. Otherwise the slot's previous club, if any, keeps its
/// virtual players elsewhere: in the block of a fixture it plays (`FIXTURE_LIST`, home or away side; `get_id_block`
/// with 1), or else of another viewed slot holding it (`get_id_block` with 0, 0), through `relocate_virtual_players`;
/// with neither it loses them (`remove_virtual_players`). Then, unless the new club already has virtual players of its
/// own and none foreign, its squad is topped up in the slot's block (`get_id_block` with 0, 0;
/// `top_up_club_squad(club, staff, player, 0, 0)`). The slot takes the club.
/// Original: `VIRTUAL_STAFF::fill_viewed_club_squad` (0x90d4f0, virtual_staff.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fill_viewed_club_squad(this: usize, club: usize) {
    use virtual_fixture_fns as vf;
    let slot = current_viewed_club_slot();
    let viewed = (this as isize + vf::VIEWED_CLUBS as isize + slot as isize * 4) as usize;
    let relocate: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(vf::RELOCATE_VIRTUAL_PLAYERS);
    if read_usize(viewed) == club && viewed_club_virtual_players(Club(club)) == Some(true) {
        return;
    }
    let old = read_usize(viewed);
    if old != 0 {
        let list = Llist(this + vf::FIXTURE_LIST);
        list.set_cursor(list.head());
        let mut node = exe::llist_get_next(list);
        while node != 0 {
            let f = Fixture(read_usize(node));
            if f.home_club().addr() == old || f.away_club().addr() == old {
                break;
            }
            node = exe::llist_get_next(list);
        }
        if node != 0 {
            covered!("sx21b_9 old club kept in its fixture's block");
            let home = (Fixture(read_usize(node)).home_club().addr() == old) as u32;
            let (staff_id, player_id) = virtual_id_block(this, read_u16(node + 4) as u32, 1, home);
            relocate(this, old, staff_id, player_id);
        } else if let Some(k) = (0..0x10i16).find(|&k| k != slot && read_usize(this + vf::VIEWED_CLUBS + k as usize * 4) == old) {
            covered!("sx21b_9 old club kept in another viewed block");
            let (staff_id, player_id) = virtual_id_block(this, k as u32, 0, 0);
            relocate(this, old, staff_id, player_id);
        } else {
            covered!("sx21b_9 old club loses its virtual players");
            exe::virtual_staff_remove_virtual_players(this, old, 0); // sx21b_9: nobody views it now
        }
    }
    if viewed_club_virtual_players(Club(club)) != Some(true) {
        covered!("sx21b_9 viewed club topped up");
        let (staff_id, player_id) = virtual_id_block(this, slot as i32 as u32, 0, 0);
        let top_up: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) = core::mem::transmute(vf::TOP_UP_CLUB_SQUAD);
        top_up(this, club, staff_id, player_id, 0, 0);
    }
    write_usize(viewed, club);
}

/// Comparer for `fill_viewed_club_squad`: it creates and moves virtual players (the generator, the records), so only
/// the original runs; then the manager's slot must hold the club.
pub(crate) unsafe extern "thiscall" fn compare_fill_viewed_club_squad(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, club);
    let at = read_usize((this as isize + virtual_fixture_fns::VIEWED_CLUBS as isize + current_viewed_club_slot() as isize * 4) as usize);
    SCOREBOARD.record(at == club, || format!("fill_viewed_club_squad (club screen): club {:#x}, the slot holds {:#x}", club, at));
}
