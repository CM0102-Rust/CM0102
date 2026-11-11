use super::*;

// The comparers of the routines in ../zipdir.rs.

/// Comparer for `zipdir_match_filename_pattern`.
pub(crate) unsafe extern "C" fn compare_zipdir_match_filename_pattern(default: *const u8, name: *const u8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(IDX_ZIPDIR_MATCH_FILENAME_PATTERN)(default, name);
    let n = zipdir_match_filename_pattern(default, name);
    SCOREBOARD.record(r == n, || {
        let t = |p: *const u8| String::from_utf8_lossy(core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes()).into_owned();
        format!("zipdir_match_filename_pattern({:?}, {:?}) {} x {}", t(default), t(name), r, n)
    });
    r
}

/// Comparer for `zget_archive_id`.
pub(crate) unsafe extern "C" fn compare_zget_archive_id(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_ZGET_ARCHIVE_ID)(path);
    let n = zget_archive_id(path);
    SCOREBOARD.record(r == n, || format!("zget_archive_id {:#x} x {:#x}", r, n));
    r
}

/// Comparer for `zget_matching_size`.
pub(crate) unsafe extern "C" fn compare_zget_matching_size(path: usize, default: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_ZGET_MATCHING_SIZE)(path, default);
    let n = zget_matching_size(path, default);
    SCOREBOARD.record(r == n, || format!("zget_matching_size {} x {}", r, n));
    r
}

/// Comparer for `zadd_matching_files`.
pub(crate) unsafe extern "C" fn compare_zadd_matching_files(path: usize, folder: usize, default: usize, a4: u32, kind: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(IDX_ZADD_MATCHING_FILES)(path, folder, default, a4, kind)
}

/// Comparer for `zextract_matching_files`.
pub(crate) unsafe extern "C" fn compare_zextract_matching_files(path: usize, folder: usize, default: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_ZEXTRACT_MATCHING_FILES)(path, folder, default)
}

/// Comparer for `zipdir_zread`.
pub(crate) unsafe extern "C" fn compare_zipdir_zread(buf: *mut u8, size: i32, how_many: i32, z: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(*mut u8, i32, i32, usize) -> i32>(IDX_ZIPDIR_ZREAD)(buf, size, how_many, z)
}

/// Comparer for `zipdir_zwrite`.
pub(crate) unsafe extern "C" fn compare_zipdir_zwrite(buf: *const u8, size: i32, how_many: i32, z: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(*const u8, i32, i32, usize) -> i32>(IDX_ZIPDIR_ZWRITE)(buf, size, how_many, z)
}

/// Comparer for `zclose`.
pub(crate) unsafe extern "C" fn compare_zclose(zz: *mut usize) {
    original_fn::<unsafe extern "C" fn(*mut usize)>(IDX_ZCLOSE)(zz)
}
