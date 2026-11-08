use super::*;

// The comparers of the routines in ../setup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../setup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `load_game_data` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_load_game_data(path: *const u8) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8) -> u32>(IDX_LOAD_GAME_DATA)(path)
}

/// Comparer for `create_game_from_setup_file` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_create_game_from_setup_file(cfg: *const u8, qst: *const u8) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(IDX_CREATE_GAME_FROM_SETUP_FILE)(cfg, qst)
}

/// Comparer for `save_game_data` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_save_game_data(path: *const u8, a4: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(*const u8, u32) -> u32>(IDX_SAVE_GAME_DATA)(path, a4)
}

/// Comparer for `check_saved_game_file` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_check_saved_game_file(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_SAVED_GAME_FILE)(path);
    if path == 0 {
        return r;
    }
    let n = check_saved_game_file(path);
    PLACAR.registrar(r == n, || format!("check_saved_game_file {:#x} x {:#x}", r, n));
    r
}

/// Comparer for `check_quick_start_file` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_check_quick_start_file(path: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CHECK_QUICK_START_FILE)(path)
}

/// Comparer for `valid_saved_game_version` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_valid_saved_game_version(path: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_VALID_SAVED_GAME_VERSION)(path)
}

/// Comparer for `save_fifa_transfer_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_save_fifa_transfer_rules(save: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_SAVE_FIFA_TRANSFER_RULES)(save)
}

/// Comparer for `load_fifa_transfer_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_load_fifa_transfer_rules(save: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_LOAD_FIFA_TRANSFER_RULES)(save)
}

/// Comparer for `build_save_game_summary` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_build_save_game_summary(summary: *mut u8) {
    let regions = [(summary as usize, 0xf6c), (summary as usize + 0xf6c, 8), (0xdd7140, 8)];
    let (_, _, _, mo, mr) = crate::comparacao::lado_a_lado(&regions,
        || original_fn::<unsafe extern "C" fn(*mut u8)>(IDX_BUILD_SAVE_GAME_SUMMARY)(summary),
        || build_save_game_summary(summary));
    PLACAR.registrar(mo.0[0] == mr.0[0], || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("build_save_game_summary: primeira diferença em +{:#x}: {:#04x} x {:#04x}", k, a[k], b[k])
    });
}

/// Comparer for `free_game_data` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_free_game_data(tela: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_FREE_GAME_DATA)(tela)
}

/// Comparer for `get_heading_colour` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_get_heading_colour(x: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_GET_HEADING_COLOUR)(x);
    let n = get_heading_colour(x);
    PLACAR.registrar(r == n, || format!("get_heading_colour({:#x}) original={:#x} rust={:#x}", x, r, n));
    r
}
