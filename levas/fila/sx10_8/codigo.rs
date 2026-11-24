// ---------- sx10_8: formation files and position names (formation.cpp, tactics screens) ----------

/// The identifier a formation file starts with (the current .pct version); obfuscated files add
/// `FORMATION_FILE_OBFUSCATION`.
const FORMATION_FILE_ID: u32 = 0x98ec5e;
const FORMATION_FILE_OBFUSCATION: u32 = 0x75bcd15;
/// The oldest identifier whose password and creator can be read.
const FORMATION_FILE_OLDEST_ID: i32 = 0x98ec59;
/// The format marker appended to a file written on this machine; a file whose marker is `FORMATION_FILE_SWAPPED_MARKER`
/// has the other byte order.
const FORMATION_FILE_NATIVE_MARKER: u32 = 0xf2ff_fff2;
const FORMATION_FILE_SWAPPED_MARKER: u32 = 0xf1ff_fff1;
/// The fopen modes "wb" and "rb".
const FORMATION_WRITE_MODE: usize = 0x9a7f28;
const FORMATION_READ_MODE: usize = 0x98f928;
/// Seeks to the end of an open file and writes a 4-byte format marker (utils.cpp, cdecl(file, marker)).
const FN_UTILS_WRITE_FILE_FORMAT_MARKER: usize = 0x90c1a0;
/// "%s%s".
const FORMATION_TWO_STRINGS: usize = 0x99ec04;
/// "<%s - position> <%s - side>".
const FORMATION_POSITION_AND_SIDE: usize = 0x9ccab0;
/// The long names of the lines and sides (some carry a "<%s - COMMENT>" that takes the empty string).
const FORMATION_GOALKEEPER_NAME: usize = 0x9b7600; // "Goalkeeper"
const FORMATION_LINE_NAMES: [(u16, usize); 6] = [(0x2, 0x9b75dc), (0x4, 0x9b75b0), (0x8, 0x9b756c), (0x10, 0x9b74f8), (0x20, 0x9b7528), (0x40, 0x9b7484)];
const FORMATION_SIDE_NAMES: [(u16, usize); 5] = [(0x80, 0x9b7444), (0x100, 0x9ccacc), (0x200, 0x9b7420), (0x400, 0x9ccaa0), (0x800, 0x9b7464)];
const FORMATION_FREE_ROLE_NAME: usize = 0x9ccad8; // "Free Role"
const FORMATION_FREE_ROLE: u16 = 0x1000;
/// The abbreviations: "GK", the lines (SW, D, DM, M, AM, F) and the sides (L, C, C, C, R).
const FORMATION_GOALKEEPER_ABBREVIATION: usize = 0x9b7610;
const FORMATION_LINE_ABBREVIATIONS: [(u16, usize); 6] = [(0x2, 0x9b75e4), (0x4, 0x9b75c0), (0x8, 0x9b7588), (0x10, 0x9b7508), (0x20, 0x9b7544), (0x40, 0x9b74b4)];
const FORMATION_SIDE_ABBREVIATIONS: [(u16, usize); 5] = [(0x80, 0x9b744c), (0x100, 0x9b7428), (0x200, 0x9b7428), (0x400, 0x9b7428), (0x800, 0x9b746c)];
/// The answers of `Formation::check_file_password`.
const FORMATION_PASSWORD_UNREADABLE: u32 = 2;
const FORMATION_PASSWORD_NONE: u32 = 1;
const FORMATION_PASSWORD_MATCHES: u32 = 9;
const FORMATION_PASSWORD_DIFFERS: u32 = 5;

/// Inverts the bytes of a formation password in place (0xff stays), up to its terminator: the same call stores and
/// restores it.
unsafe fn formation_flip_password(password: usize) {
    let mut k = 0;
    while k < c_string_len(password) {
        let b = read_u8(password + k);
        write_u8(password + k, if b == 0xff { 0xff } else { !b });
        k += 1;
    }
}

/// `langlib_sprintf(out, format, EMPTY_STRING)`: the localised text of a name (the empty string fills the "<%s -
/// COMMENT>" some carry; the others ignore it).
unsafe fn formation_name_text(out: usize, format: usize) {
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    langlib(out, format, EMPTY_STRING); // ponytail: the extra argument is ignored by the names without a %s
}

/// Finds another slot standing on the same pitch point in one phase and ball zone.
///
/// `Formation::check_duplicate_coordinate(slot, phase, zone, out)`: slot outside 0..10: the formation.cpp 0x5ce box and
/// 0; phase outside 0..1: 0x5d6; zone outside band 0..8, height 0..0xb: 0x5df (wider than the 3 × 4 zones, as in
/// is_slot_coordinate_customised). Otherwise `out` = 0 and the first other slot whose stored coordinate
/// (`formation_fields::SLOT_COORDINATES`, + 0x30 with the ball) equals this slot's decides: a later slot sets `out` =
/// 1 and gives 1, an earlier one gives 0. No other slot there: 1.
/// Original: `Formation::check_duplicate_coordinate` (0x5b4ee0, formation.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn formation_check_duplicate_coordinate(this: usize, slot: u32, stage: u32, zone: u32, out: *mut u32) -> u32 {
    let (s, phase) = (slot as u8 as i8, stage as u8 as i8);
    let (band, height) = (zone as u16 as i16, (zone >> 16) as u16 as i16);
    let line = if !(0..=10).contains(&s) {
        0x5ce
    } else if !(0..=1).contains(&phase) {
        0x5d6
    } else if !(0..=8).contains(&band) || !(0..=0xb).contains(&height) {
        0x5df
    } else {
        0
    };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    let point = |k: i32| {
        let index = height as i32 + 4 * (band as i32 + 3 * phase as i32 + 6 * k);
        read_u32((this as isize + formation_fields::SLOT_COORDINATES as isize + 4 * index as isize) as usize)
    };
    *out = 0;
    for k in 0..11i32 {
        if k == s as i32 || point(k) != point(s as i32) {
            continue;
        }
        if k > s as i32 { // sx10_8: a later slot on the same point
            *out = 1;
            return 1;
        }
        return 0;
    }
    1
}

/// A failed write of `Formation::save_to_disk`: the formation.cpp box at `line` (unless `quiet`), the file closed and
/// 0.
unsafe fn formation_save_failed(quiet: u32, line: u32, file: usize) -> u32 {
    fail_read(quiet, line, file)
}

/// Writes the formation to a .pct file.
///
/// `Formation::save_to_disk(path, quiet, obfuscated)`: no path (line 0x9ca) or a file that does not open for writing
/// (0x9f4): 0, with the formation.cpp box unless `quiet`. Then, each write checked: the identifier 0x98ec5e (plus
/// 123456789 when `obfuscated`; 0xa03), the password (`formation_fields::PASSWORD`, 50 bytes) inverted for the file and
/// inverted back afterwards (0xa16), the creator (200 bytes, 0xa26), the formation mask (0xa33), the 11 general
/// positions (0xa40), the display order (0xa4d), the 11 extra positions (0xa5a) and the coordinates (0x420 bytes,
/// 0xa67): a failed one closes the file and gives 0 (the box unless `quiet`). The team instructions (0xa74), the player
/// instructions (0x58 bytes, 0xa7f) and the marking types (0xa8a) only show the box when they fail. The file ends with
/// the native format marker (`utils` 0x90c1a0) and is closed; the formation takes the file's name (`_splitpath` into
/// +0), changed = 1, has password = the password is not empty, saved = 1. Returns 1. The designated people are not
/// saved. The original also formats a specific message for each failure into a local that is never shown.
/// Original: `Formation::save_to_disk` (0x5b6900, formation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn formation_save_to_disk(this: usize, path: usize, quiet: u32, obfuscated: u32) -> u32 {
    if path == 0 {
        if quiet == 0 {
            formation_error_box(0x9ca);
        }
        return 0;
    }
    let file = exe::crt_fopen(path, FORMATION_WRITE_MODE);
    if file == 0 {
        if quiet == 0 {
            formation_error_box(0x9f4);
        }
        return 0;
    }
    let mut id = FORMATION_FILE_ID;
    if obfuscated != 0 {
        id = id.wrapping_add(FORMATION_FILE_OBFUSCATION);
    }
    let write = |at: usize, size: u32| exe::crt_fwrite(at, size, 1, file) == 1;
    if !write(&mut id as *mut u32 as usize, 4) {
        return formation_save_failed(quiet, 0xa03, file);
    }
    let password = this + formation_fields::PASSWORD;
    formation_flip_password(password); // sx10_8: the password inverted for the file
    let written = write(password, 0x32);
    formation_flip_password(password);
    if !written {
        return formation_save_failed(quiet, 0xa16, file);
    }
    for (offset, size, line) in [
        (formation_fields::CREATOR, 200, 0xa26),
        (formation_fields::FORMATION_MASK, 4, 0xa33),
        (formation_fields::SLOT_CELLS, 0x16, 0xa40),
        (formation_fields::DISPLAY_ORDER, 0xb, 0xa4d),
        (formation_fields::EXTRA_SLOT_CELLS, 0x16, 0xa5a),
        (formation_fields::SLOT_COORDINATES, 0x420, 0xa67),
    ] {
        if !write(this + offset, size) {
            return formation_save_failed(quiet, line, file);
        }
    }
    for (offset, size, line) in [(formation_fields::FLAGS, 4, 0xa74), (formation_fields::PLAYER_INSTRUCTIONS, 0x58, 0xa7f), (formation_fields::MARKING_TYPES, 0xb, 0xa8a)] {
        if !write(this + offset, size) && quiet == 0 {
            formation_error_box(line);
        }
    }
    let marker: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_UTILS_WRITE_FILE_FORMAT_MARKER);
    marker(file, FORMATION_FILE_NATIVE_MARKER);
    exe::crt_fclose(file);
    exe::crt_strtok(path, 0, 0, this as *mut u8, core::ptr::null_mut());
    write_u32(this + formation_fields::CHANGED_FLAG, 1);
    write_u32(this + formation_fields::HAS_PASSWORD, (read_u8(password) != 0) as u32);
    write_u32(this + 0x1887, 1); // saved [inference]
    covered!("sx10_8 formation saved");
    1
}

/// Writes the long name of a position code with its side, e.g. "Defender Centre Left".
///
/// `formation_get_position_name(code, out, size, strict)`: no buffer or a size below 100: the formation.cpp 0xc80 box
/// and 0. The goalkeeper (bit 1) gives "Goalkeeper". Otherwise the first line bit (sweeper 2, defender 4, defensive
/// midfielder 8, midfielder 0x10, attacking midfielder 0x20, forward 0x40) and the first side bit (left 0x80, centre
/// left 0x100, centre 0x200, centre right 0x400, right 0x800) are joined as "<position> <side>" (`langlib_sprintf`);
/// a missing one is empty, or with `strict` the box (0xcbd for the line, 0xcea for the side) and 0. Returns 1.
/// Original: `formation_get_position_name` (0x5b7580, formation.cpp, cdecl).
pub(crate) unsafe extern "C" fn formation_position_name(code: u32, out: usize, size: u32, strict: u32) -> u32 {
    let mut position = [0u8; 100];
    let mut side = [0u8; 100];
    position[0] = read_u8(EMPTY_STRING);
    side[0] = read_u8(EMPTY_STRING);
    if out == 0 || (size as i32) < 100 {
        formation_error_box(0xc80);
        return 0;
    }
    write_u8(out, 0);
    let code = code as u16;
    if code & 1 != 0 {
        formation_name_text(out, FORMATION_GOALKEEPER_NAME);
        return 1;
    }
    match FORMATION_LINE_NAMES.iter().find(|(bit, _)| code & bit != 0) {
        Some(&(_, name)) => formation_name_text(position.as_mut_ptr() as usize, name),
        None if strict != 0 => {
            formation_error_box(0xcbd);
            return 0;
        }
        None => {}
    }
    match FORMATION_SIDE_NAMES.iter().find(|(bit, _)| code & bit != 0) {
        Some(&(_, name)) => formation_name_text(side.as_mut_ptr() as usize, name), // sx10_8: the side's name
        None if strict != 0 => {
            formation_error_box(0xcea);
            return 0;
        }
        None => {}
    }
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    langlib(out, FORMATION_POSITION_AND_SIDE, position.as_ptr(), side.as_ptr());
    1
}

/// Writes the abbreviation of a position code, such as GK, DL or AMC.
///
/// `formation_get_position_abbreviation(code, out, size, line_only)`: no buffer or a size below 10: the formation.cpp
/// 0xd0c box and 0. The goalkeeper gives "GK". Otherwise the first line bit gives SW, D, DM, M, AM or F (none: the
/// 0xd47 box and 0); with `line_only` an abbreviation longer than one letter is the whole result (written with
/// `sprintf` using it as the format). The first side bit adds L, C (centre left, centre and centre right alike) or R
/// (none: the 0xd79 box and 0), joined with "%s%s". Returns 1. The line goes into an 8-byte local just before the side's:
/// a translated line abbreviation of 8 letters or more runs into it (a defect of the original, the side overwrites it).
/// Original: `formation_get_position_abbreviation` (0x5b7810, formation.cpp, cdecl).
pub(crate) unsafe extern "C" fn formation_position_abbreviation(code: u32, out: usize, size: u32, line_only: u32) -> u32 {
    if out == 0 || (size as i32) < 10 {
        formation_error_box(0xd0c);
        return 0;
    }
    write_u8(out, 0);
    let code = code as u16;
    if code & 1 != 0 {
        formation_name_text(out, FORMATION_GOALKEEPER_ABBREVIATION);
        return 1;
    }
    // the line at +0 and the side at +8, as the original's two adjacent locals
    let mut parts = [0u8; 0x18];
    let (line, side) = (parts.as_mut_ptr() as usize, parts.as_mut_ptr() as usize + 8);
    match FORMATION_LINE_ABBREVIATIONS.iter().find(|(bit, _)| code & bit != 0) {
        Some(&(_, name)) => formation_name_text(line, name),
        None => {
            formation_error_box(0xd47);
            return 0;
        }
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    if line_only != 0 && c_string_len(line) > 1 { // sx10_8: a two-letter line stands alone
        sprintf(out, line);
        return 1;
    }
    match FORMATION_SIDE_ABBREVIATIONS.iter().find(|(bit, _)| code & bit != 0) {
        Some(&(_, name)) => formation_name_text(side, name),
        None => {
            formation_error_box(0xd79);
            return 0;
        }
    }
    sprintf(out, FORMATION_TWO_STRINGS, line, side);
    1
}

/// Writes the name of a single line or side flag, or "Free Role" for 0x1000.
///
/// `formation_get_position_flag_name(flag, out, size)`: no buffer or a size below 100: the formation.cpp 0xd96 box and
/// 0. A flag equal to one line bit gives its long name (Goalkeeper ... Forward), one side bit its side (Left ... Right),
/// 0x1000 "Free Role"; anything else the empty string. Returns 1.
/// Original: `formation_get_position_flag_name` (0x5b7ab0, formation.cpp, cdecl).
pub(crate) unsafe extern "C" fn formation_position_flag_name(flag: u32, out: usize, size: u32) -> u32 {
    if out == 0 || (size as i32) < 100 {
        formation_error_box(0xd96);
        return 0;
    }
    write_u8(out, 0);
    let flag = flag as u16;
    let name = if flag == 1 {
        Some(FORMATION_GOALKEEPER_NAME)
    } else if flag == FORMATION_FREE_ROLE {
        Some(FORMATION_FREE_ROLE_NAME)
    } else {
        FORMATION_LINE_NAMES.iter().chain(FORMATION_SIDE_NAMES.iter()).find(|(bit, _)| *bit == flag).map(|&(_, name)| name)
    };
    match name {
        Some(name) => formation_name_text(out, name), // sx10_8: the flag's name
        None => copy_c_string(out, EMPTY_STRING),
    }
    1
}

/// Reads the password and creator of a .pct formation file.
///
/// `formation_read_file_password(path, password, creator, obfuscated)`: a missing argument: the formation.cpp 0x101b
/// box and 0. Both outputs are emptied; a file that does not open: 0. The identifier (byte-swapped when the file's
/// marker is the other order; minus 123456789 when `obfuscated`) must be at least 0x98ec59; then the 50-byte password,
/// inverted back (0xff stays), and the 200-byte creator. Any short read closes the file and gives 0; otherwise 1.
/// Original: `formation_read_file_password` (0x5b8600, formation.cpp, cdecl).
pub(crate) unsafe extern "C" fn formation_read_file_password(path: usize, password: *mut u8, creator: *mut u8, obfuscated: u32) -> u32 {
    if path == 0 || password.is_null() || creator.is_null() {
        formation_error_box(0x101b);
        return 0;
    }
    let (password, creator) = (password as usize, creator as usize);
    write_u8(password, 0);
    write_u8(creator, 0);
    let file = exe::crt_fopen(path, FORMATION_READ_MODE);
    if file == 0 {
        return 0;
    }
    let swapped = exe::utils_read_file_format_marker(file) == FORMATION_FILE_SWAPPED_MARKER;
    let read = |at: usize, size: u32| exe::crt_fread(at, size, 1, file) == 1;
    let mut id = 0u32;
    let ok = read(&mut id as *mut u32 as usize, 4) && {
        if swapped {
            id = id.swap_bytes();
        }
        if obfuscated != 0 {
            id = id.wrapping_sub(FORMATION_FILE_OBFUSCATION);
        }
        id as i32 >= FORMATION_FILE_OLDEST_ID // sx10_8: old files have no password
    } && read(password, 0x32) && {
        formation_flip_password(password);
        read(creator, 200)
    };
    exe::crt_fclose(file);
    ok as u32
}

/// Compares a formation file's password with the current human manager's: 2 when it cannot be read, 1 when it has
/// none, 9 when they match, 5 otherwise.
unsafe fn formation_file_password_verdict(path: usize, obfuscated: u32) -> u32 {
    let mut password = [0u8; 0x34];
    let mut creator = [0u8; 200];
    if exe::formation_read_file_password(path, password.as_mut_ptr(), creator.as_mut_ptr(), obfuscated) == 0 {
        return FORMATION_PASSWORD_UNREADABLE;
    }
    if password[0] == 0 {
        return FORMATION_PASSWORD_NONE;
    }
    let mine = exe::human_manager_get_password(read_usize(G_HUMAN_MANAGER), 0);
    if exe_strcmp(mine, password.as_ptr() as usize) == 0 { // sx10_8: the manager's own password
        FORMATION_PASSWORD_MATCHES
    } else {
        FORMATION_PASSWORD_DIFFERS
    }
}

/// Checks a plain formation file's password against the current human manager's.
///
/// `formation_check_file_password(path)`: reads the file's password (`formation_read_file_password`, not obfuscated):
/// 2 when it cannot be read, 1 when it is empty; otherwise 9 when it equals the current human manager's
/// (`HUMAN_MANAGER::get_password`), else 5. Called through the tactics screens' function pointers.
/// Original: `formation_check_file_password` (0x5b87e0, formation.cpp, cdecl).
pub(crate) unsafe extern "C" fn formation_check_file_password(path: usize) -> u32 {
    formation_file_password_verdict(path, 0)
}

/// Checks an obfuscated formation file's password against the current human manager's: the same answers as
/// `formation_check_file_password`, reading the file as obfuscated.
/// Original: `formation_check_file_password_obfuscated` (0x5b88a0, formation.cpp, cdecl).
pub(crate) unsafe extern "C" fn formation_check_file_password_obfuscated(path: usize) -> u32 {
    formation_file_password_verdict(path, 1)
}

/// Comparer for `formation_check_duplicate_coordinate`: the answer and the flag.
pub(crate) unsafe extern "thiscall" fn compare_formation_check_duplicate_coordinate(this: usize, slot: u32, stage: u32, zone: u32, out: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u32) -> u32>(%%I0%%);
    let (band, height) = (zone as u16 as i16, (zone >> 16) as u16 as i16);
    if !(0..=10).contains(&(slot as u8 as i8)) || !(0..=1).contains(&(stage as u8 as i8)) || !(0..=8).contains(&band) || !(0..=0xb).contains(&height) {
        return o(this, slot, stage, zone, out);
    }
    let (mut so, mut sr) = (0xdead_u32, 0xbeef_u32);
    let r = o(this, slot, stage, zone, &mut so);
    let n = formation_check_duplicate_coordinate(this, slot, stage, zone, &mut sr);
    SCOREBOARD.record(r == n && so == sr, || format!("Formation::check_duplicate_coordinate({}, {}, {:#x}) {}/{} out {}/{}", slot as u8, stage as u8, zone, r, n, so, sr));
    *out = so;
    r
}

/// Comparer for `formation_save_to_disk`: it writes a file, so only the original runs; the plan is checked afterwards
/// (after a save: changed = 1, has password = the password is not empty, saved = 1).
pub(crate) unsafe extern "thiscall" fn compare_formation_save_to_disk(this: usize, path: usize, quiet: u32, obfuscated: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I1%%)(this, path, quiet, obfuscated);
    if r == 1 {
        let plan = read_u32(this + formation_fields::CHANGED_FLAG) == 1
            && read_u32(this + formation_fields::HAS_PASSWORD) == (read_u8(this + formation_fields::PASSWORD) != 0) as u32
            && read_u32(this + 0x1887) == 1;
        SCOREBOARD.record(plan, || format!("Formation::save_to_disk({:#x}): the formation's flags after the save", this));
    }
    r
}

/// Side by side over the text a name routine writes (`size` bytes, at most 0x100).
unsafe fn formation_name_side_by_side(name: &str, out: usize, size: u32, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, (size as usize).min(0x100))], original, rust);
    SCOREBOARD.record(same && r == n, || format!("{} {}/{} {:?}", name, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `formation_position_name` (a bad buffer or an unnamed position with `strict` shows the box: original
/// only).
pub(crate) unsafe extern "C" fn compare_formation_position_name(code: u32, out: usize, size: u32, strict: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(%%I2%%);
    let has = |list: &[(u16, usize)]| list.iter().any(|(bit, _)| code as u16 & bit != 0);
    if out == 0 || (size as i32) < 100 || (strict != 0 && code & 1 == 0 && (!has(&FORMATION_LINE_NAMES) || !has(&FORMATION_SIDE_NAMES))) {
        return o(code, out, size, strict);
    }
    formation_name_side_by_side(&format!("formation_get_position_name({:#x})", code as u16), out, size, || o(code, out, size, strict), || formation_position_name(code, out, size, strict))
}

/// Comparer for `formation_position_abbreviation` (a bad buffer or a code without line or side shows the box: original
/// only).
pub(crate) unsafe extern "C" fn compare_formation_position_abbreviation(code: u32, out: usize, size: u32, line_only: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(%%I3%%);
    let has = |list: &[(u16, usize)]| list.iter().any(|(bit, _)| code as u16 & bit != 0);
    if out == 0 || (size as i32) < 10 || (code & 1 == 0 && (!has(&FORMATION_LINE_ABBREVIATIONS) || !has(&FORMATION_SIDE_ABBREVIATIONS))) {
        return o(code, out, size, line_only);
    }
    formation_name_side_by_side(&format!("formation_get_position_abbreviation({:#x}, {})", code as u16, line_only), out, size, || o(code, out, size, line_only), || formation_position_abbreviation(code, out, size, line_only))
}

/// Comparer for `formation_position_flag_name` (a bad buffer shows the box: original only).
pub(crate) unsafe extern "C" fn compare_formation_position_flag_name(flag: u32, out: usize, size: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32) -> u32>(%%I4%%);
    if out == 0 || (size as i32) < 100 {
        return o(flag, out, size);
    }
    formation_name_side_by_side(&format!("formation_get_position_flag_name({:#x})", flag as u16), out, size, || o(flag, out, size), || formation_position_flag_name(flag, out, size))
}

/// Comparer for `formation_read_file_password`: it only reads the file, so both run side by side over the two outputs
/// (a missing argument shows the box: original only).
pub(crate) unsafe extern "C" fn compare_formation_read_file_password(path: usize, password: *mut u8, creator: *mut u8, obfuscated: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u8, *mut u8, u32) -> u32>(%%I5%%);
    if path == 0 || password.is_null() || creator.is_null() {
        return o(path, password, creator, obfuscated);
    }
    let reg = [(password as usize, 0x32), (creator as usize, 200)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(path, password, creator, obfuscated), || formation_read_file_password(path, password, creator, obfuscated));
    SCOREBOARD.record(same && r == n, || format!("formation_read_file_password({}) {}/{} {:?}", obfuscated, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `formation_check_file_password`: it only reads the file, so both run and the answers are compared.
pub(crate) unsafe extern "C" fn compare_formation_check_file_password(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I6%%)(path);
    let n = formation_check_file_password(path);
    SCOREBOARD.record(r == n, || format!("formation_check_file_password {}/{}", r, n));
    r
}

/// Comparer for `formation_check_file_password_obfuscated`: both run and the answers are compared.
pub(crate) unsafe extern "C" fn compare_formation_check_file_password_obfuscated(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I7%%)(path);
    let n = formation_check_file_password_obfuscated(path);
    SCOREBOARD.record(r == n, || format!("formation_check_file_password_obfuscated {}/{}", r, n));
    r
}
