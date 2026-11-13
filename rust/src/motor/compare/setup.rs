use super::*;

// The comparers of the routines in ../setup.rs.

/// Set while a saved game loads (13/11/2026): every competition built inside the load takes its saved state, so the
/// constructor comparers skip the fixed fields a new career writes (`league_will_load_saved_state`).
pub(crate) static LOADING_SAVED_GAME: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Comparer for `load_game_data`: the original, with `LOADING_SAVED_GAME` set around it.
pub(crate) unsafe extern "C" fn compare_load_game_data(path: *const u8) -> u32 {
    LOADING_SAVED_GAME.store(true, core::sync::atomic::Ordering::Relaxed);
    let r = original_fn::<unsafe extern "C" fn(*const u8) -> u32>(IDX_LOAD_GAME_DATA)(path);
    LOADING_SAVED_GAME.store(false, core::sync::atomic::Ordering::Relaxed);
    r
}

/// Comparer for `create_game_from_setup_file`.
pub(crate) unsafe extern "C" fn compare_create_game_from_setup_file(cfg: *const u8, qst: *const u8) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(IDX_CREATE_GAME_FROM_SETUP_FILE)(cfg, qst)
}

/// Comparer for `save_game_data`.
pub(crate) unsafe extern "C" fn compare_save_game_data(path: *const u8, a4: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8, u32) -> u32>(IDX_SAVE_GAME_DATA)(path, a4)
}

/// Comparer for `check_saved_game_file`.
pub(crate) unsafe extern "C" fn compare_check_saved_game_file(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_SAVED_GAME_FILE)(path);
    if path == 0 {
        return r;
    }
    let n = check_saved_game_file(path);
    SCOREBOARD.record(r == n, || format!("check_saved_game_file {:#x} x {:#x}", r, n));
    r
}

/// Comparer for `check_quick_start_file`.
pub(crate) unsafe extern "C" fn compare_check_quick_start_file(path: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_QUICK_START_FILE)(path)
}

/// Comparer for `valid_saved_game_version`.
pub(crate) unsafe extern "C" fn compare_valid_saved_game_version(path: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_VALID_SAVED_GAME_VERSION)(path)
}

/// Comparer for `save_fifa_transfer_rules`.
pub(crate) unsafe extern "C" fn compare_save_fifa_transfer_rules(save: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_SAVE_FIFA_TRANSFER_RULES)(save)
}

/// Comparer for `load_fifa_transfer_rules`.
pub(crate) unsafe extern "C" fn compare_load_fifa_transfer_rules(save: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_LOAD_FIFA_TRANSFER_RULES)(save)
}

/// Comparer for `free_game_data`.
pub(crate) unsafe extern "C" fn compare_free_game_data(tela: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_FREE_GAME_DATA)(tela)
}

/// Comparer for `get_heading_colour`.
pub(crate) unsafe extern "C" fn compare_get_heading_colour(x: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_GET_HEADING_COLOUR)(x);
    let n = get_heading_colour(x);
    SCOREBOARD.record(r == n, || format!("get_heading_colour({:#x}) original={:#x} rust={:#x}", x, r, n));
    r
}
