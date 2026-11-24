// ---------- cx10_12: the Irish Premier Division's progress codes and fixed draw slots ----------

/// The Irish Premier Division's fixed draw slot table: 10 entries of 5 bytes at +0xee, a club id (dword, −1 free) and
/// its slot (byte, at +4).
const IRE_DRAW_SLOTS: usize = 0xee;
const IRE_DRAW_SLOT_ENTRY: usize = 5;
const IRE_DRAW_SLOT_COUNT: usize = 10;

/// The address of entry `k` of the fixed draw slot table.
unsafe fn ire_draw_slot_entry(this: usize, k: usize) -> usize {
    this + IRE_DRAW_SLOTS + k * IRE_DRAW_SLOT_ENTRY
}

/// Stores a club's three progress codes in its statistics entry (`store_club_progress`), the worst code 10 becoming
/// 11; the current and best codes stay.
///
/// `set_club_progress(club, stage, current, best, worst)` (virtual `comp_vtable::SET_CLUB_PROGRESS`, ret 0x14), the
/// codes are bytes; the stage is not read.
/// Defect of the original: the best code is compared with 10 and the branch jumps to the next instruction, so nothing
/// happens (the remap of the best code was lost).
/// Original: `IrePrm::set_club_progress` (0x63ab60, ire_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ire_prm_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    let mut worst = worst as u8;
    if worst == 0xa {
        covered!("cx10_12 irish worst code 10");
        worst = 0xb;
    }
    store_club_progress(this, club, current as u8, best as u8, worst);
}

/// Returns the fixed draw slot stored for the club (its id at `club`), or −1 (0xff) when none of the 10 entries holds
/// it. The slot is returned as a byte (the upper bytes of eax are 0).
/// Original: `IrePrm::get_fixed_draw_slot` (0x63aba0, ire_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_prm_get_fixed_draw_slot(this: usize, club: usize) -> u32 {
    let id = read_u32(club);
    for k in 0..IRE_DRAW_SLOT_COUNT {
        let e = ire_draw_slot_entry(this, k);
        if read_u32(e) == id {
            return read_u8(e + 4) as u32;
        }
    }
    0xff
}

/// Stores the club's fixed draw slot: in its own entry when one holds its id; otherwise in the first entry that is free
/// (−1) or holds a club whose division (`club::DIVISION`) is not this competition (`league::COMPETITION`), which takes
/// the club's id; with none, nothing.
/// Original: `IrePrm::set_fixed_draw_slot` (0x63abe0, ire_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ire_prm_set_fixed_draw_slot(this: usize, club: usize, slot: u32) {
    let id = read_u32(club);
    for k in 0..IRE_DRAW_SLOT_COUNT {
        let e = ire_draw_slot_entry(this, k);
        if read_u32(e) == id {
            write_u8(e + 4, slot as u8);
            return;
        }
    }
    for k in 0..IRE_DRAW_SLOT_COUNT {
        let e = ire_draw_slot_entry(this, k);
        let held = read_i32(e);
        if held == -1 || read_usize(club_at(held as isize).addr() + club::DIVISION) != League(this).competition().addr() {
            covered!("cx10_12 irish draw slot taken");
            write_u32(e, id);
            write_u8(e + 4, slot as u8);
            return;
        }
    }
}

pub(crate) unsafe extern "thiscall" fn compare_ire_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_club_progress("ire_prm_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || ire_prm_set_club_progress(this, club, stage, current, best, worst));
}

/// Comparer of the slot reader: side by side, the returns.
pub(crate) unsafe extern "thiscall" fn compare_ire_prm_get_fixed_draw_slot(this: usize, club: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, club);
    let c = ire_prm_get_fixed_draw_slot(this, club);
    SCOREBOARD.record(r as u8 == c as u8, || format!("ire_prm_get_fixed_draw_slot({:#x}): original {:#x} copy {:#x}", club, r, c));
    r
}

/// Comparer of the slot writer: side by side over the 50-byte slot table.
pub(crate) unsafe extern "thiscall" fn compare_ire_prm_set_fixed_draw_slot(this: usize, club: usize, slot: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(this + IRE_DRAW_SLOTS, IRE_DRAW_SLOT_COUNT * IRE_DRAW_SLOT_ENTRY)],
        || o(this, club, slot), || ire_prm_set_fixed_draw_slot(this, club, slot));
    SCOREBOARD.record(same, || format!("ire_prm_set_fixed_draw_slot({:#x}, {}): original {:x?} copy {:x?}", club, slot as u8, mo, mr));
}
