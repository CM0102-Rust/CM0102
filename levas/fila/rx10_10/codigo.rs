// ---------- rx10_10: the award manager's vote slots and history lists (award_manager.cpp) ----------

/// The human managers' vote slots of the AWARD_MANAGER: 17 slots of 12 bytes from +0x1c (the player, then the two
/// award indices), one per human manager by staff id (the humans are the last staff records).
pub(crate) const AWARD_VOTE_SLOTS: usize = 0x1c;
pub(crate) const AWARD_VOTE_SLOT_SIZE: usize = 0xc;

/// The vote slot index of a manager: its staff id minus the staff count plus 16, when it is from 0 to 16 (the last
/// 16 staff records, and one past them).
pub(crate) unsafe fn award_vote_slot_index(manager: usize) -> Option<usize> {
    let k = (read_u32(manager) as i32).wrapping_sub(read_u32(G_STAFF_COUNT) as i32).wrapping_add(0x10);
    if (0..=0x10).contains(&k) { Some(k as usize) } else { None }
}

/// Rule: a human manager's pending vote is the player he picked on the nominations screen and the award's indices,
/// kept in his own slot until he casts it.
///
/// `AWARD_MANAGER::set_pending_vote(manager, player, first, second)`: the slot of `award_vote_slot_index`(manager)
/// gets the player, `first` and `second` (three dwords); a manager outside the slots: the award_manager.cpp box at line
/// 0x3bc and nothing written.
/// Original: `AWARD_MANAGER::set_pending_vote` (0x4148e0, award_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn award_manager_set_pending_vote(this: usize, manager: usize, player: usize, first: u32, second: u32) {
    let Some(k) = award_vote_slot_index(manager) else {
        return show_error_at(0x987e24, 0x3bc);
    };
    covered!("rx10_10 pending vote set");
    let slot = this + AWARD_VOTE_SLOTS + k * AWARD_VOTE_SLOT_SIZE;
    write_usize(slot, player);
    write_u32(slot + 4, first);
    write_u32(slot + 8, second);
}

/// Comparer for `award_manager_set_pending_vote`: the original (outside the slots only the original, with its box);
/// then the slot must hold the three arguments.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_set_pending_vote(this: usize, manager: usize, player: usize, first: u32, second: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I0%%)(this, manager, player, first, second);
    if let Some(k) = award_vote_slot_index(manager) {
        let slot = this + AWARD_VOTE_SLOTS + k * AWARD_VOTE_SLOT_SIZE;
        let ok = read_usize(slot) == player && read_u32(slot + 4) == first && read_u32(slot + 8) == second;
        SCOREBOARD.record(ok, || format!("award_manager_set_pending_vote({:#x}, slot {}): {:#x} {} {}", this, k, read_usize(slot), read_u32(slot + 4), read_u32(slot + 8)));
    }
}

/// Rule: the nominations screen asks whether the human already picked a given player: his slot when it holds that
/// player, else nothing.
///
/// `AWARD_MANAGER::get_pending_vote(manager, player)`: no manager: 0; a manager outside the slots: the
/// award_manager.cpp box at line 0x3e1 and 0; otherwise the address of his slot when its player is `player`, else 0.
/// Original: `AWARD_MANAGER::get_pending_vote` (0x4149a0, award_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn award_manager_get_pending_vote(this: usize, manager: usize, player: usize) -> usize {
    if manager == 0 {
        return 0;
    }
    let Some(k) = award_vote_slot_index(manager) else {
        show_error_at(0x987e24, 0x3e1);
        return 0;
    };
    let slot = this + AWARD_VOTE_SLOTS + k * AWARD_VOTE_SLOT_SIZE;
    if read_usize(slot) == player {
        covered!("rx10_10 pending vote found");
        slot
    } else {
        0
    }
}

/// Comparer for `award_manager_get_pending_vote`: side by side (it only reads; outside the slots only the original,
/// so the box is not shown twice): the same return.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_get_pending_vote(this: usize, manager: usize, player: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I1%%)(this, manager, player);
    if manager != 0 && award_vote_slot_index(manager).is_none() {
        return r;
    }
    let c = award_manager_get_pending_vote(this, manager, player);
    SCOREBOARD.record(r == c, || format!("award_manager_get_pending_vote({:#x}, {:#x}, {:#x}): original {:#x} copy {:#x}", this, manager, player, r, c));
    r
}

/// Rule: the year awards' past winners shown on the award screen are the history records between two offsets of the
/// manager's first history file.
///
/// `AWARD_MANAGER::get_year_award_records(first, last)`: FILE_LLIST::get_nodes (`exe::file_llist_get_nodes`) on the
/// first history list (+0), a new LLIST (or 0).
/// Original: `AWARD_MANAGER::get_year_award_records` (0x4142e0, award_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn award_manager_get_year_award_records(this: usize, first: u32, last: u32) -> usize {
    covered!("rx10_10 year award records");
    exe::file_llist_get_nodes(read_usize(this), first, last) // rx10_10: the year awards' history file
}

/// Rule: the team awards' past teams shown on the award screen are the history records between two offsets of the
/// manager's second history file.
///
/// `AWARD_MANAGER::get_team_award_records(first, last)`: FILE_LLIST::get_nodes on the second history list (+4).
/// Original: `AWARD_MANAGER::get_team_award_records` (0x414320, award_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn award_manager_get_team_award_records(this: usize, first: u32, last: u32) -> usize {
    covered!("rx10_10 team award records");
    exe::file_llist_get_nodes(read_usize(this + 4), first, last) // rx10_10: the team awards' history file
}

/// The history list comparer: only the original (each call reads the file and allocates a new list; a second read
/// is not risked on a screen), then the plan: no list, or a list whose node count is not negative.
pub(crate) unsafe fn compare_award_history_records(name: &str, file_list: usize, first: u32, last: u32, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let ok = r == 0 || read_i16(r + llist::COUNT) >= 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}, {}, {}): list {:#x}", name, file_list, first, last, r));
    r
}

/// Comparer for `award_manager_get_year_award_records` (`compare_award_history_records`).
pub(crate) unsafe extern "thiscall" fn compare_award_manager_get_year_award_records(this: usize, first: u32, last: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I2%%);
    compare_award_history_records("award_manager_get_year_award_records", read_usize(this), first, last, || o(this, first, last))
}

/// Comparer for `award_manager_get_team_award_records` (`compare_award_history_records`).
pub(crate) unsafe extern "thiscall" fn compare_award_manager_get_team_award_records(this: usize, first: u32, last: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I3%%);
    compare_award_history_records("award_manager_get_team_award_records", read_usize(this + 4), first, last, || o(this, first, last))
}
