// ---------- batch %%LEVA%%: the human manager rows of the staff table (Setup.cpp) ----------

/// The Setup.cpp source name the error box shows.
pub(crate) const SETUP_CPP_FILE: usize = 0xa801dc; // "E:\dev\CM3\cm3 00-01\cm3\code\Setup.cpp"

/// The human manager row `k` (0..16) counted back from the end of the staff table: the 16 human managers are the
/// last 16 staff records.
unsafe fn human_manager_row(k: i32) -> Staff {
    Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(G_STAFF_COUNT) - 1 - k) as usize * staff::SIZE))
}

/// The current screen slot's manager (`G_SCREEN_RECORDS_MANAGER`, 0x300 bytes per slot, slot
/// `G_SCREEN_MANAGER_CURRENT_SLOT`).
unsafe fn current_screen_manager() -> Staff {
    Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize) as usize))
}

/// The first of the 16 human manager rows that is free (classification 0), from the last staff record back, or 0.
///
/// `find_free_human_manager_row()`. Without the staff table, the game's error box (line 0x39b3) and 0.
/// Original: `find_free_human_manager_row` (0x838c60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn find_free_human_manager_row() -> usize {
    if read_usize(G_STAFF_TABLE) == 0 {
        show_error_at(SETUP_CPP_FILE, 0x39b3);
        return 0;
    }
    covered!("mwg_4 free row");
    (0..16).map(|k| human_manager_row(k)).find(|s| s.classification() == 0).map_or(0, |s| s.addr()) // mwg4: a free row
}

/// Comparer for `find_free_human_manager_row`.
pub(crate) unsafe extern "C" fn compare_find_free_human_manager_row() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(%%I0%%)();
    let n = find_free_human_manager_row();
    SCOREBOARD.record(r == n, || format!("find_free_human_manager_row() original={:#x} rust={:#x}", r, n));
    r
}

/// How many of the 16 human manager rows are in use (classification 1); with `network_only`, only those whose
/// manager is done on the screen manager (`exe::screenmanager_is_manager_done`).
///
/// `count_human_managers(network_only)`. Without the staff table, the game's error box (line 0x39d8) and 0.
/// Original: `count_human_managers` (0x838d10, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn count_human_managers(network_only: u32) -> u32 {
    if read_usize(G_STAFF_TABLE) == 0 {
        show_error_at(SETUP_CPP_FILE, 0x39d8);
        return 0;
    }
    covered!("mwg_4 count");
    (0..16)
        .map(|k| human_manager_row(k))
        .filter(|s| s.classification() == 1 && (network_only == 0 || exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, s.addr() as u32) != 0)) // mwg4: a human in use
        .count() as u32
}

/// Comparer for `count_human_managers`.
pub(crate) unsafe extern "C" fn compare_count_human_managers(network_only: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I1%%)(network_only);
    let n = count_human_managers(network_only);
    SCOREBOARD.record(r == n, || format!("count_human_managers({}) original={} rust={}", network_only, r, n));
    r
}

/// 1 when the current screen slot has a manager whose row is still being made (classification 5), else 0.
///
/// `is_current_manager_being_created()`.
/// Original: `is_current_manager_being_created` (0x838de0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn is_current_manager_being_created() -> u32 {
    covered!("mwg_4 being created");
    let s = current_screen_manager();
    (!s.is_null() && s.classification() == 5) as u32 // mwg4: being created
}

/// Comparer for `is_current_manager_being_created`.
pub(crate) unsafe extern "C" fn compare_is_current_manager_being_created() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(%%I2%%)();
    let n = is_current_manager_being_created();
    SCOREBOARD.record(r == n, || format!("is_current_manager_being_created() original={} rust={}", r, n));
    r
}
