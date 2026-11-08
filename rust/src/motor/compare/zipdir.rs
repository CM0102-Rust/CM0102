use super::*;

// The comparers of the routines in ../zipdir.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../zipdir.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `zipdir_match_filename_pattern` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_zipdir_match_filename_pattern(padrao: *const u8, nome: *const u8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(IDX_ZIPDIR_MATCH_FILENAME_PATTERN)(padrao, nome);
    let n = zipdir_match_filename_pattern(padrao, nome);
    PLACAR.registrar(r == n, || {
        let t = |p: *const u8| String::from_utf8_lossy(core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes()).into_owned();
        format!("zipdir_match_filename_pattern({:?}, {:?}) {} x {}", t(padrao), t(nome), r, n)
    });
    r
}

/// Comparer for `zget_archive_id` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_zget_archive_id(caminho: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_ZGET_ARCHIVE_ID)(caminho);
    let n = zget_archive_id(caminho);
    PLACAR.registrar(r == n, || format!("zget_archive_id {:#x} x {:#x}", r, n));
    r
}

/// Comparer for `zget_matching_size` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_zget_matching_size(caminho: usize, padrao: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_ZGET_MATCHING_SIZE)(caminho, padrao);
    let n = zget_matching_size(caminho, padrao);
    PLACAR.registrar(r == n, || format!("zget_matching_size {} x {}", r, n));
    r
}

/// Comparer for `zadd_matching_files` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_zadd_matching_files(caminho: usize, pasta: usize, padrao: usize, a4: u32, tipo: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(IDX_ZADD_MATCHING_FILES)(caminho, pasta, padrao, a4, tipo)
}

/// Comparer for `zextract_matching_files` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_zextract_matching_files(caminho: usize, pasta: usize, padrao: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_ZEXTRACT_MATCHING_FILES)(caminho, pasta, padrao)
}

/// Comparer for `utils_clear_temp_folder` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_utils_clear_temp_folder(padrao: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_UTILS_CLEAR_TEMP_FOLDER)(padrao)
}

/// Comparer for `zipdir_zread` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_zipdir_zread(buf: *mut u8, tam: i32, quantos: i32, z: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(*mut u8, i32, i32, usize) -> i32>(IDX_ZIPDIR_ZREAD)(buf, tam, quantos, z)
}

/// Comparer for `zipdir_zwrite` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_zipdir_zwrite(buf: *const u8, tam: i32, quantos: i32, z: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(*const u8, i32, i32, usize) -> i32>(IDX_ZIPDIR_ZWRITE)(buf, tam, quantos, z)
}

/// Comparer for `zclose` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_zclose(zz: *mut usize) {
    original_fn::<unsafe extern "C" fn(*mut usize)>(IDX_ZCLOSE)(zz)
}
