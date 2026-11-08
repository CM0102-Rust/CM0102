use super::*;

// The comparers of the routines in ../setup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../setup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `load_game_data` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_load_game_data(path: *const u8) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8) -> u32>(IDX_LOAD_GAME_DATA)(path)
}

/// Comparer for `create_game_from_setup_file` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_create_game_from_setup_file(cfg: *const u8, qst: *const u8) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(IDX_CREATE_GAME_FROM_SETUP_FILE)(cfg, qst)
}

/// Comparer for `save_game_data` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_save_game_data(path: *const u8, a4: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8, u32) -> u32>(IDX_SAVE_GAME_DATA)(path, a4)
}

/// Comparer for `check_saved_game_file` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_check_saved_game_file(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_SAVED_GAME_FILE)(path);
    if path == 0 {
        return r;
    }
    let n = check_saved_game_file(path);
    SCOREBOARD.record(r == n, || format!("check_saved_game_file {:#x} x {:#x}", r, n));
    r
}

/// Comparer for `check_quick_start_file` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_check_quick_start_file(path: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_QUICK_START_FILE)(path)
}

/// Comparer for `valid_saved_game_version` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_valid_saved_game_version(path: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_VALID_SAVED_GAME_VERSION)(path)
}

/// Comparer for `save_fifa_transfer_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_save_fifa_transfer_rules(save: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_SAVE_FIFA_TRANSFER_RULES)(save)
}

/// Comparer for `load_fifa_transfer_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_load_fifa_transfer_rules(save: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_LOAD_FIFA_TRANSFER_RULES)(save)
}

/// Comparer for `free_game_data` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_free_game_data(tela: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_FREE_GAME_DATA)(tela)
}

/// Comparer for `get_heading_colour` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_get_heading_colour(x: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_GET_HEADING_COLOUR)(x);
    let n = get_heading_colour(x);
    SCOREBOARD.record(r == n, || format!("get_heading_colour({:#x}) original={:#x} rust={:#x}", x, r, n));
    r
}
