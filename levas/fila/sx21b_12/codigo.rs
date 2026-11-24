// ---------- sx21b_12: the game's start: the random seed, the config file lines and trimming a text (utils.cpp) ----------

/// The random table's first entry and its end (the cursor is kept between them).
const RANDOM_TABLE_START: u32 = 0xaa2454;
const RANDOM_TABLE_END: u32 = 0xad4130;
/// The text the seed is logged into.
const G_SEED_TEXT: usize = 0xde1f64;

/// Seeds the random generator with the given value (or the time, for 0), logs the seed and picks a random start in the
/// random table.
///
/// `utils_seed_random(seed)`: `srand(seed)`; "Random Number seed: %d" into `G_SEED_TEXT`; the cursor (0xde1f60) at
/// the table start plus `rand() % 0xc738` entries of 4 bytes; the end (0xde2798) at the table's end; the offset
/// (0xde1f5c) `rand() % 0xffff`; a cursor outside the table goes back to its start. Returns the cursor as drawn.
/// Original: `utils_seed_random` (0x90d0a0, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_seed_random(seed: u32) -> u32 {
    let mut seed = seed;
    if seed == 0 {
        seed = exe::at_946ebb(core::ptr::null_mut()) as u32; // time(0)
    }
    exe::crt_srand(seed);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(G_SEED_TEXT, 0xad437c, seed); // "Random Number seed: %d"
    covered!("sx21b_12 random seeded");
    let cursor = RANDOM_TABLE_START.wrapping_add(((exe::crt_rand() % 0xc738) as u32).wrapping_mul(4)); // sx21b_12: 0xc738 entries
    write_u32(0xde1f60, cursor);
    write_u32(0xde2798, RANDOM_TABLE_END);
    write_u32(0xde1f5c, (exe::crt_rand() % 0xffff) as u32);
    if !(RANDOM_TABLE_START..=RANDOM_TABLE_END).contains(&cursor) {
        write_u32(0xde1f60, RANDOM_TABLE_START);
    }
    cursor
}

/// Comparer for `utils_seed_random`: the random state (with the CRT's seed), the table end and the logged text, side
/// by side.
pub(crate) unsafe extern "C" fn compare_utils_seed_random(seed: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I0%%);
    let mut regions = vec![(0xde2798usize, 4usize), (G_SEED_TEXT, 0x30)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(seed), || utils_seed_random(seed));
    SCOREBOARD.record(equal && r == c, || format!("utils_seed_random({}): differences in {:?}", seed, first_differences(&mo, &mr)));
    r
}

/// Reads the next non-empty line of a config file into a buffer; 1 when a line was read.
///
/// `utils_read_config_line(file, buffer)`, a character at a time (`fgetc`, 0xff ends the file): tabs, carriage returns
/// and spaces before the first character are skipped (later they are kept); a newline ends a line that has something
/// (an empty one is skipped); a hash skips the rest of its line, newline included (a line with text before the hash
/// goes on with the next line). At the end of the file the line ends there; 0 when nothing was read.
/// Original: `utils_read_config_line` (0x90cba0, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_read_config_line(file: usize, buffer: usize) -> u32 {
    let mut n = 0usize;
    let mut c = exe::at_9469e0(file) as u8;
    while c != 0xff {
        match c {
            9 | 0xd | b' ' => {
                if n != 0 {
                    write_u8(buffer + n, c);
                    n += 1;
                }
            }
            b'\n' => {
                if n != 0 {
                    covered!("sx21b_12 config line read");
                    write_u8(buffer + n, 0);
                    return 1;
                }
            }
            b'#' => {
                while c != b'\n' && c != 0xff { // sx21b_12: a comment runs to the end of the line
                    c = exe::at_9469e0(file) as u8;
                }
            }
            _ => {
                write_u8(buffer + n, c);
                n += 1;
            }
        }
        c = exe::at_9469e0(file) as u8;
    }
    write_u8(buffer + n, 0);
    (n != 0) as u32
}

/// Comparer for `utils_read_config_line`: reading moves the file on, so only the original runs.
pub(crate) unsafe extern "C" fn compare_utils_read_config_line(file: usize, buffer: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%)(file, buffer)
}

/// Removes the control characters (below 0x20) from the start and the end of a text, in place; no text: the box (line
/// 0x18f).
/// Original: `utils_trim_control_chars` (0x90bb70, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_trim_control_chars(text: usize) {
    if text == 0 {
        show_error_at(0xad4134, 0x18f);
        return;
    }
    let mut n = network_strlen(text);
    if n == 0 {
        return;
    }
    while read_u8(text) < 0x20 {
        covered!("sx21b_12 leading control character");
        let rest = network_strlen(text + 1) + 1;
        core::ptr::copy(((text + 1) as *const u8), text as *mut u8, rest);
        n -= 1;
        if n == 0 {
            return;
        }
    }
    while n != 0 && read_u8(text + n - 1) < 0x20 { // sx21b_12: trailing control characters
        write_u8(text + n - 1, 0);
        n -= 1;
    }
}

/// Comparer for `utils_trim_control_chars`: the text side by side (no text shows the box: only the original).
pub(crate) unsafe extern "C" fn compare_utils_trim_control_chars(text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(%%I2%%);
    if text == 0 {
        return o(text);
    }
    let n = network_strlen(text) + 1;
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(text, n)], || o(text), || utils_trim_control_chars(text));
    SCOREBOARD.record(equal, || format!("utils_trim_control_chars: differences in {:?}", first_differences(&mo, &mr)));
}
