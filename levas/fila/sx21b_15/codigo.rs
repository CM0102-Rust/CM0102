// ---------- sx21b_15: the format marker at the end of a data file (utils.cpp) ----------

/// The file format markers the game recognises; any other read gives `MARKER_INVALID`.
const MARKER_A: u32 = 0xf1ff_fff1;
const MARKER_B: u32 = 0xf2ff_fff2;
const MARKER_INVALID: u32 = 0xf3ff_fff3;

/// Reads the four-byte format marker of a file and restores the file position: the marker when it reads whole and is
/// 0xf1fffff1 or 0xf2fffff2, otherwise 0xf3fffff3.
///
/// `utils_read_file_format_marker(file)`: the position (`ftell`), `fseek(file, 4, SEEK_END)`, `fread` of 4 bytes, back
/// to the position (`fseek`, SEEK_SET). The seek goes 4 bytes past the end (not before it), so the read fails on an
/// ordinary file and the answer is 0xf3fffff3 (as the original).
/// Original: `utils_read_file_format_marker` (0x90c140, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_read_file_format_marker(file: usize) -> u32 {
    let at = exe::crt_ftell(file);
    exe::crt_fseek(file, 4, 2);
    let mut marker = 0u32;
    let read = exe::crt_fread(&mut marker as *mut u32 as usize, 4, 1, file) == 1;
    exe::crt_fseek(file, at, 0);
    if read && (marker == MARKER_A || marker == MARKER_B) {
        covered!("sx21b_15 format marker found");
        marker
    } else {
        MARKER_INVALID // sx21b_15: no marker
    }
}

/// Comparer for `utils_read_file_format_marker`: the file goes back to its position, so the copy runs after the
/// original and the values are compared.
pub(crate) unsafe extern "C" fn compare_utils_read_file_format_marker(file: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(file);
    let c = utils_read_file_format_marker(file);
    SCOREBOARD.record(r == c, || format!("utils_read_file_format_marker original={:#x} rust={:#x}", r, c));
    r
}
