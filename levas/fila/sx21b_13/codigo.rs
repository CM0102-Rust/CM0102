// ---------- sx21b_13: files and folders: a file exists, a folder exists, the user's folder (utils.cpp) ----------

/// The C runtime's `_access`, `_stat` and `_getcwd` (cdecl).
const FN_CRT_ACCESS: usize = 0x9478d8;
const FN_CRT_STAT: usize = 0x9475b5;
const FN_CRT_GETCWD: usize = 0x946da0;

/// Whether a file exists: `_access(path, 0)` is 0.
/// Original: `utils_file_exists` (0x90c120, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_file_exists(path: usize) -> u32 {
    let access: unsafe extern "C" fn(usize, u32) -> i32 = core::mem::transmute(FN_CRT_ACCESS);
    (access(path, 0) == 0) as u32 // sx21b_13: mode 0, existence only
}

/// Comparer for `utils_file_exists`: the value.
pub(crate) unsafe extern "C" fn compare_utils_file_exists(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(path);
    let c = utils_file_exists(path);
    SCOREBOARD.record(r == c, || format!("utils_file_exists original={} rust={}", r, c));
    r
}

/// Whether a path names an existing folder: 1 when `_stat` finds it with the folder bit (0x4000 of the mode). A copy
/// of the path (`malloc` of its length plus 1) loses a final backslash, and gets one after a bare drive letter ("C:").
/// No path (line 0x29d) or an empty one (line 0x2a4): the box and 0. Defect of the original: the backslash after a
/// drive letter is written one byte past the copy's block.
/// Original: `utils_is_directory` (0x90bf90, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_is_directory(path: usize) -> u32 {
    if path == 0 {
        show_error_at(0xad4134, 0x29d);
        return 0;
    }
    let n = network_strlen(path);
    if n == 0 {
        show_error_at(0xad4134, 0x2a4);
        return 0;
    }
    let copy = exe::crt_malloc(n as u32 + 1);
    match_screens_c_strcpy(copy as *mut u8, path as *const u8);
    let len = network_strlen(copy);
    if read_u8(copy.wrapping_add(len).wrapping_sub(1)) == b'\\' {
        write_u8(copy.wrapping_add(len).wrapping_sub(1), 0);
    }
    let len = network_strlen(copy);
    if read_u8(copy.wrapping_add(len).wrapping_sub(1)) == b':' {
        covered!("sx21b_13 drive letter");
        match_screens_c_strcat(copy as *mut u8, 0x9c3730 as *const u8); // "\"
    }
    let mut st = [0u8; 0x40];
    let stat: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(FN_CRT_STAT);
    let r = stat(copy, st.as_mut_ptr());
    exe::crt_free(copy);
    if r == -1 {
        return 0;
    }
    (u16::from_le_bytes([st[6], st[7]]) & 0x4000 != 0) as u32 // sx21b_13: the folder bit of the mode
}

/// Comparer for `utils_is_directory`: the value (no path or an empty one shows the box: only the original).
pub(crate) unsafe extern "C" fn compare_utils_is_directory(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(path);
    if path != 0 && read_u8(path) != 0 {
        let c = utils_is_directory(path);
        SCOREBOARD.record(r == c, || format!("utils_is_directory original={} rust={}", r, c));
    }
    r
}

/// Builds a folder path under the current folder and creates the folder when it does not exist; 1, or 0 with the box
/// (line 0x28c, "unable to create directory") when it cannot be created.
///
/// `utils_get_user_folder(name, out)`: `_getcwd` (0x104 bytes) and `_makepath(out, 0, cwd, name, 0)`; an existing
/// folder (`utils_is_directory`) or one `_mkdir` creates gives 1.
/// Original: `utils_get_user_folder` (0x90bea0, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_get_user_folder(name: usize, out: usize) -> u32 {
    let mut cwd = [0u8; 0x104];
    let getcwd: unsafe extern "C" fn(*mut u8, u32) -> usize = core::mem::transmute(FN_CRT_GETCWD);
    getcwd(cwd.as_mut_ptr(), 0x104);
    exe::crt_makepath(out as *mut u8, 0usize, cwd.as_ptr() as usize, name, 0usize);
    let is_directory: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_UTILS_IS_DIRECTORY);
    if is_directory(out) != 0 {
        covered!("sx21b_13 user folder exists");
        return 1;
    }
    if exe::at_945bd2(out) == 0 { // _mkdir
        return 1;
    }
    let mut message = [0u8; 0x100];
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(message.as_mut_ptr() as usize, 0xad419c, name);
    show_error_at(0xad4134, 0x28c);
    0
}

/// Comparer for `utils_get_user_folder`: the original first (it may create the folder); when it gave 1, the copy runs
/// from the same buffer and the path (0x104 bytes) and the value are compared.
pub(crate) unsafe extern "C" fn compare_utils_get_user_folder(name: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I2%%);
    let regions = [(out, 0x104usize)];
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let r = o(name, out);
    if r != 1 {
        return r;
    }
    let after = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let c = utils_get_user_folder(name, out);
    let mine = crate::comparison::MemorySnapshot::read(&regions);
    after.write_back();
    SCOREBOARD.record(after == mine && r == c, || format!("utils_get_user_folder: original {} rust {}, differences in {:?}", r, c, first_differences(&after, &mine)));
    r
}
