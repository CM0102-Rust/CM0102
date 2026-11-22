// ---------- batch %%LEVA%%: the award finders of the award screens (award_screens.cpp) ----------

/// The award finders' common body: the manager's table of 30 award lists (the pointer at +8); in each non-null list
/// (+4 the item array, +8 i16 the count, read once) the first award whose dword at `at` equals `key` is returned;
/// 0 when none.
unsafe fn award_manager_find_by(manager: usize, key: u32, at: usize) -> usize {
    let lists = read_usize(manager + 8);
    for k in 0..0x1e {
        let list = read_usize(lists + k * 4);
        if list == 0 {
            continue;
        }
        let count = read_i16(list + 8) as i32;
        let items = read_usize(list + 4);
        for i in 0..count.max(0) as usize {
            if read_u32(read_usize(items + i * 4) + at) == key {
                return read_usize(read_usize(read_usize(read_usize(manager + 8) + k * 4) + 4) + i * 4);
            }
        }
    }
    0
}

/// The award of a staff competition: the first award, in the 30 lists of the manager, whose staff competition
/// (+5) is `comp`; 0 when none (`award_manager_find_by`).
/// Original: `AWARD_MANAGER::find_award_by_staff_comp` (0x414a60, award_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_manager_find_award_by_staff_comp(manager: usize, comp: u32) -> usize {
    award_manager_find_by(manager, comp, 5)
}

/// The award of a club competition: the first award whose club competition (+9) is `comp`; 0 when none.
/// Original: `AWARD_MANAGER::find_award_by_club_comp` (0x414ac0, award_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_manager_find_award_by_club_comp(manager: usize, comp: u32) -> usize {
    award_manager_find_by(manager, comp, 9)
}

compare_value!(compare_award_manager_find_award_by_staff_comp, award_manager_find_award_by_staff_comp, %%I0%%, (comp: u32) -> usize);
compare_value!(compare_award_manager_find_award_by_club_comp, award_manager_find_award_by_club_comp, %%I1%%, (comp: u32) -> usize);
