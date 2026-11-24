// ---------- cx02_16: the group cups' entrant list in the saved game, writing, and the Dutch Cup's (gre_cup.cpp, hol_cup.cpp) ----------

/// The entrant list of a cup built on a league with group stages, written to its temporary file in the order
/// `load_group_cup_entrants` reads it: the count (`ita_cup_fields::ENTRANT_CLUB_COUNT`, 4 bytes; `exe::crt_fwrite`,
/// unchecked); the list's clubs turned into their ids (a null slot −1; the list and the count read again each time);
/// the count's ids written (with `skip_null_list`, the Dutch Cup, not when the list is null); the ids turned back into
/// clubs (`club_at`; −1 a null slot). Returns the count as last read (the eax the original leaves).
pub(crate) unsafe fn save_group_cup_entrants(this: usize, file: usize, skip_null_list: bool) -> u32 {
    let cup = ItaCupFields(this);
    let count = || read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT);
    exe::crt_fwrite(this + ita_cup_fields::ENTRANT_CLUB_COUNT, 4, 1, file);
    let mut k = 0i32;
    while k < count() {
        let slot = cup.entrant_clubs() + 4 * k as usize;
        let club = read_usize(slot);
        write_u32(slot, if club != 0 { read_u32(club) } else { u32::MAX });
        k += 1;
    }
    if !skip_null_list || cup.entrant_clubs() != 0 {
        covered!("cx02_16 group cup entrants saved");
        exe::crt_fwrite(cup.entrant_clubs(), 4, count() as u32, file);
    }
    let mut k = 0i32;
    while k < count() {
        let slot = cup.entrant_clubs() + 4 * k as usize;
        let id = read_i32(slot);
        write_usize(slot, if id == -1 { 0 } else { club_at(id as isize).addr() });
        k += 1;
    }
    count() as u32
}

/// The entrant list of the Greek Cup and of the other group cups that share this virtual (the Irish, Northern Irish
/// and Korean League Cups, the Coppa Italia, the Serie C cup and the Welsh League and Premier Cups):
/// `save_group_cup_entrants`, the list always written.
/// Original: `GreCup::save_extra_data` (0x5e9fb0, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn gre_cup_save_extra_data(this: usize, file: usize) -> u32 {
    save_group_cup_entrants(this, file, false)
}

/// The Dutch Cup entrant list (`load_group_cup_entrants`, hol_cup.cpp lines 0x44b, 0x455 and 0x45d; a count of 0
/// leaves the list null).
/// Original: `HolCup::load_extra_data` (0x5f4e90, hol_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hol_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9cedd4, (0x44b, 0x455, 0x45d), true)
}

/// The Dutch Cup entrant list written (`save_group_cup_entrants`, not written when the list is null).
/// Original: `HolCup::save_extra_data` (0x5f5070, hol_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hol_cup_save_extra_data(this: usize, file: usize) -> u32 {
    save_group_cup_entrants(this, file, true)
}

/// Comparer for `gre_cup_save_extra_data`: it writes the saved game (it cannot run twice), so only the original runs;
/// the list must come back as it was (the clubs, not the ids). The copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_save_extra_data(this: usize, file: usize) -> u32 {
    compare_group_cup_entrants_save("gre_cup_save_extra_data", this, || original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    compare_group_cup_entrants_load("hol_cup_load_extra_data", this, file, || o(this, file), || hol_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_save_extra_data(this: usize, file: usize) -> u32 {
    compare_group_cup_entrants_save("hol_cup_save_extra_data", this, || original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, file))
}

/// Runs only the original writer and checks that the entrant list is back as it was before (clubs, not ids).
unsafe fn compare_group_cup_entrants_save(name: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let list = ItaCupFields(this).entrant_clubs();
    let n = if list != 0 { read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT).max(0) as usize } else { 0 };
    let before: Vec<usize> = (0..n).map(|k| read_usize(list + 4 * k)).collect();
    let r = original();
    let after: Vec<usize> = (0..n).map(|k| read_usize(list + 4 * k)).collect();
    SCOREBOARD.record(before == after, || format!("{}({:#x}): the list changed ({} entries)", name, this, n));
    r
}
