use super::*;

/// Rule: the next stage of the Dutch Cup: past the last stage, the error box; otherwise the current stage goes up by
/// one; on reaching stage 19 (after the group stages) the knockout stage is built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of hol_cup.cpp at line
/// 0x24f. Otherwise the current stage + 1; at 0x13, HolCup::create_knockout_stage (0x5f4480, thiscall, ret 0, not
/// ported).
/// Original: `HolCup::advance_stage` (0x5f4190, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9cedd4, 0x24f);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("mwc_25 dutch cup next stage");
    if stage == 0x13 {
        call_on_league(0x5f4480, l);
    }
}

/// Comparer for `hol_cup_advance_stage`: side by side on the current stage when nothing is built; at the last stage or
/// on reaching stage 0x13, only the original.
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_advance_stage(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_HOL_CUP_ADVANCE_STAGE);
    let l = this;
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) || stage + 1 == 0x13 {
        return o(this.addr());
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + league::CURRENT_STAGE, 4)], || o(this.addr()), || hol_cup_advance_stage(this.addr()));
    SCOREBOARD.record(same, || format!("hol_cup_advance_stage({:#x}) original={:x?} rust={:x?}", this.addr(), mo, mr));
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

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HOL_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("hol_cup_load_extra_data", this, file, || o(this, file), || hol_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_save_extra_data(this: usize, file: usize) -> u32 {
    compare_group_cup_entrants_save("hol_cup_save_extra_data", this, || original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HOL_CUP_SAVE_EXTRA_DATA)(this, file))
}
