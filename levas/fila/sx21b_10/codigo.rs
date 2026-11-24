// ---------- sx21b_10: the game's file paths: a folder by its key and a full path (utils.cpp) ----------

/// The shared buffer `utils_build_file_path` returns.
const G_FILE_PATH_BUFFER: usize = 0xde279c;
/// The game's command line (searched for "-ignoreenv").
const G_COMMAND_LINE: usize = 0xae2c98;
/// `utils_get_folder_for_key` (cdecl; this item).
const FN_UTILS_GET_FOLDER_FOR_KEY: usize = 0x90bc50;
/// `utils_copy_path_limited` (cdecl; this item).
const FN_UTILS_COPY_PATH_LIMITED: usize = 0x90cf20;
/// The C runtime's `getenv` (cdecl).
const FN_CRT_GETENV: usize = 0x947480;

/// The default folder of each folder key when the environment does not give one: the data folder for SI_DATA,
/// CM3_DATA and CM3_TACTICS, the game folder (empty) for CM3_TEMP, CM3_DEBUG and CM3_PREFS, and its own folder for the
/// others.
const FOLDER_KEYS: [(usize, usize); 12] = [
    (0x9c3718, 0x9c3824), // SI_DATA: "data"
    (0x9b5348, 0x9c3824), // CM3_DATA: "data"
    (0x987da0, 0xad9c64), // CM3_TEMP: ""
    (0x9b8df0, 0xad9c64), // CM3_DEBUG: ""
    (0x9cd36c, 0xad9c64), // CM3_PREFS: ""
    (0x9ccae4, 0x9c3824), // CM3_TACTICS: "data"
    (0xad4180, 0x9898f4), // CM3_TRAINING: "training"
    (0x9cedb8, 0xad4178), // CM3_HISTORY: "history"
    (0x9899e0, 0x9c3840), // CM3_PICTURES: "pictures"
    (0x9c382c, 0x9c3838), // CM3_SOUNDS: "sounds"
    (0xad416c, 0xad4164), // CM3_HELP: "help"
    (0xa80204, 0xad415c), // CM3_QSTART: "qstart"
];

/// The folder of a folder key (CM3_DATA, CM3_TEMP...): the environment variable of that name (`getenv`), unless the
/// command line has "-ignoreenv", and otherwise its default (`FOLDER_KEYS`, compared without case); an unknown key: 0.
/// Original: `utils_get_folder_for_key` (0x90bc50, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_get_folder_for_key(key: usize) -> usize {
    if exe::crt_strstr(G_COMMAND_LINE, 0xad4190usize) == 0 { // "-ignoreenv"
        let getenv: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_GETENV);
        let from_env = getenv(key);
        if from_env != 0 {
            return from_env;
        }
    }
    for &(name, folder) in FOLDER_KEYS.iter() {
        if exe::crt_stricmp(key, name) == 0 {
            covered!("sx21b_10 default folder");
            return folder; // sx21b_10: the key's default folder
        }
    }
    0
}

/// Comparer for `utils_get_folder_for_key`: the folder (a pointer to the environment or to a fixed text).
pub(crate) unsafe extern "C" fn compare_utils_get_folder_for_key(key: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I0%%)(key);
    let c = utils_get_folder_for_key(key);
    SCOREBOARD.record(r == c, || format!("utils_get_folder_for_key original={:#x} rust={:#x}", r, c));
    r
}

/// Builds the full path of a file in a folder key's folder into the shared buffer and returns it.
///
/// `utils_build_file_path(key, name)`: the buffer (`G_FILE_PATH_BUFFER`) emptied; the folder
/// (`utils_get_folder_for_key`), when not empty, and a backslash; then the name as `utils_copy_path_limited` gives it.
/// Original: `utils_build_file_path` (0x90bde0, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_build_file_path(key: usize, name: usize) -> usize {
    write_u8(G_FILE_PATH_BUFFER, 0);
    let folder_of: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_UTILS_GET_FOLDER_FOR_KEY);
    let folder = folder_of(key);
    let mut copy = [0u8; 0x100];
    let limited: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_UTILS_COPY_PATH_LIMITED);
    limited(name, copy.as_mut_ptr() as usize);
    if folder != 0 && read_u8(folder) != 0 {
        covered!("sx21b_10 path with a folder");
        match_screens_c_strcpy(G_FILE_PATH_BUFFER as *mut u8, folder as *const u8);
        match_screens_c_strcat(G_FILE_PATH_BUFFER as *mut u8, 0x9c3730 as *const u8); // sx21b_10: "\"
    }
    match_screens_c_strcat(G_FILE_PATH_BUFFER as *mut u8, copy.as_ptr());
    G_FILE_PATH_BUFFER
}

/// Comparer for `utils_build_file_path`: the shared buffer (0x104 bytes) side by side.
pub(crate) unsafe extern "C" fn compare_utils_build_file_path(key: usize, name: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I1%%);
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(G_FILE_PATH_BUFFER, 0x104usize)], || o(key, name), || utils_build_file_path(key, name));
    SCOREBOARD.record(equal && r == c, || format!("utils_build_file_path: differences in {:?}", first_differences(&mo, &mr)));
    r
}

/// Copies a path into a 256-byte buffer: whole when shorter than 256; otherwise without its spaces, and when even that
/// reaches 255 characters, cut there with the path's last 8 characters at the end (+0xf7).
/// Original: `utils_copy_path_limited` (0x90cf20, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_copy_path_limited(path: usize, out: usize) {
    let n = network_strlen(path);
    if n < 0x100 {
        match_screens_c_strcpy(out as *mut u8, path as *const u8);
        return;
    }
    covered!("sx21b_10 long path");
    let mut k = 0usize;
    let mut i = 0usize;
    loop {
        let c = read_u8(path + i);
        if c == 0 {
            break;
        }
        i += 1;
        if c == b' ' {
            continue;
        }
        if k == 0xff { // sx21b_10: the buffer is full
            write_u8(out + k, 0);
            exe::at_9466d0(out + 0xf7, (path + n - 8) as *const u8, 8);
            write_u8(out + 0xff, 0);
            return;
        }
        write_u8(out + k, c);
        k += 1;
    }
    write_u8(out + k, 0);
}

/// Comparer for `utils_copy_path_limited`: the 256-byte buffer side by side.
pub(crate) unsafe extern "C" fn compare_utils_copy_path_limited(path: usize, out: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(%%I2%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(out, 0x100usize)], || o(path, out), || utils_copy_path_limited(path, out));
    SCOREBOARD.record(equal, || format!("utils_copy_path_limited: differences in {:?}", first_differences(&mo, &mr)));
}
