// ---------- sx21b_14: utilities: swapping two buffers, copying a file and the version string (utils.cpp) ----------

/// The game version text "3.9.68" (for the asserts and the error boxes).
const VERSION_STRING: usize = 0xad4394;

/// Swaps `count` bytes between two buffers; a null first (line 0x16b) or second (line 0x172) buffer shows the box.
/// Original: `utils_swap_bytes` (0x90ba90, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_swap_bytes(a: usize, b: usize, count: u32) {
    if a == 0 {
        show_error_at(0xad4134, 0x16b);
        return;
    }
    if b == 0 {
        show_error_at(0xad4134, 0x172);
        return;
    }
    covered!("sx21b_14 bytes swapped");
    for k in 0..count as usize { // sx21b_14: every byte
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        write_u8(a + k, y);
        write_u8(b + k, x);
    }
}

/// Comparer for `utils_swap_bytes`: both buffers side by side (a null one shows the box: only the original).
pub(crate) unsafe extern "C" fn compare_utils_swap_bytes(a: usize, b: usize, count: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I0%%);
    if a == 0 || b == 0 || count == 0 {
        return o(a, b, count);
    }
    let regions = [(a, count as usize), (b, count as usize)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(a, b, count), || utils_swap_bytes(a, b, count));
    SCOREBOARD.record(equal, || format!("utils_swap_bytes({} bytes): differences in {:?}", count, first_differences(&mo, &mr)));
}

/// Copies a file to a new path through a heap buffer; 1 when copied, 0 when either file does not open.
///
/// `utils_copy_file(from, to)`: `fopen` "rb" and "wb"; the size from `fseek`/`ftell`; a buffer of that size, halved
/// down to 256 bytes while `malloc` fails; then blocks read (`fread`) and written (`fwrite`) until the size is copied;
/// the buffer freed, both files closed. Defect of the original: a file that reads short of its size loops for ever.
/// Original: `utils_copy_file` (0x90cc50, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_copy_file(from: usize, to: usize) -> u32 {
    let source = exe::crt_fopen(from, 0x98f928); // "rb"
    if source == 0 {
        return 0;
    }
    let target = exe::crt_fopen(to, 0x9a7f28); // "wb"
    if target == 0 {
        exe::crt_fclose(source);
        return 0;
    }
    exe::crt_fseek(source, 0, 2);
    let size = exe::crt_ftell(source) as u32;
    exe::crt_fseek(source, 0, 0);
    let mut block = size;
    let mut buffer = exe::crt_malloc(block);
    while buffer == 0 {
        block >>= 1;
        if block < 0x100 {
            block = 0x100; // sx21b_14: at least 256 bytes
        }
        buffer = exe::crt_malloc(block);
    }
    covered!("sx21b_14 file copied");
    let mut left = size;
    while left != 0 {
        let got = exe::crt_fread(buffer, 1, block, source);
        exe::crt_fwrite(buffer, 1, got, target);
        left = left.wrapping_sub(got);
    }
    exe::crt_free(buffer);
    exe::crt_fclose(target);
    exe::crt_fclose(source);
    1
}

/// Comparer for `utils_copy_file`: it writes a file, so only the original runs.
pub(crate) unsafe extern "C" fn compare_utils_copy_file(from: usize, to: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%)(from, to)
}

/// The game version text, "3.9.68", for the asserts and the error boxes.
/// Original: `utils_get_version_string` (0x90d130, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_get_version_string() -> usize {
    covered!("sx21b_14 version asked");
    VERSION_STRING // sx21b_14: "3.9.68"
}

/// Comparer for `utils_get_version_string`: the pointer.
pub(crate) unsafe extern "C" fn compare_utils_get_version_string() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(%%I2%%)();
    let c = utils_get_version_string();
    SCOREBOARD.record(r == c, || format!("utils_get_version_string original={:#x} rust={:#x}", r, c));
    r
}
