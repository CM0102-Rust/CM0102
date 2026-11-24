// ---------- sx21b_8: a training schedule on the training screens: players, flags, name and its file (training_schedule.cpp) ----------

/// "...\training_schedule.cpp", for the error box.
const TRAINING_SCHEDULE_FILE: usize = 0xa8cfa0;
/// The empty string passed to every translated schedule text ("%s" in the language file).
const EMPTY_TEXT_ARG: usize = 0xad9c64;
/// The schedule file's header magic and version.
const SCHEDULE_FILE_MAGIC: u32 = 0x200cb;

/// `langlib_sprintf(buffer, text, arg)`: a translated text into `buffer`.
unsafe fn schedule_text(buffer: usize, text: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(buffer, text, EMPTY_TEXT_ARG);
}

/// Counts (and lists) the players of a 50-entry squad list (5 bytes each: the person, his schedule slot) that train on
/// this schedule: those on its slot, all of them for the Overview (type 0xff) and the injured ones (`Injury` status 2)
/// for No Training (type 0).
///
/// `TRAINING_SCHEDULE::count_players(list, out, compare)`: with `compare`, the list is sorted first (`qsort`, 50 of 5
/// bytes); `out`, when given, is emptied (`exe::llist_clear`) and gets each counted entry (5 bytes). Returns the count
/// (a byte).
/// Original: `TRAINING_SCHEDULE::count_players` (0x8b5000, training_schedule.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn count_players(this: usize, list: usize, out: usize, compare: usize) -> u8 {
    if compare != 0 {
        exe::crt_qsort(list, 0x32, 5, compare);
    }
    if out != 0 {
        exe::llist_clear(out);
    }
    let mut count = 0u8;
    for k in 0..0x32 {
        let e = list + k * 5;
        let p = read_usize(e);
        if p == 0 {
            continue;
        }
        let on_it = read_i8(e + 4) as i32 == read_i32(this) || match read_u8(this + training_schedule::TYPE) {
            0xff => true,
            0 => exe::injury_get_player_injury_status(read_usize(0xb63d28), p) == 2, // sx21b_8: No Training lists the injured
            _ => false,
        };
        if on_it {
            covered!("sx21b_8 player counted");
            if out != 0 {
                exe::llist_add_tail(out, e, 5);
            }
            count = count.wrapping_add(1);
        }
    }
    count
}

/// Comparer for `count_players`: without `out`, the count and the (sorted) list side by side; with `out`, the list
/// gets new nodes, so only the original, and its count must match the list's.
pub(crate) unsafe extern "thiscall" fn compare_count_players(this: usize, list: usize, out: usize, compare: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8>(%%I0%%);
    if out != 0 {
        let r = o(this, list, out, compare);
        let listed = Llist(out).count();
        SCOREBOARD.record(listed as u8 == r, || format!("count_players (training screen): {} counted, {} listed", r, listed));
        return r;
    }
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(list, 250usize)], || o(this, list, out, compare), || count_players(this, list, out, compare));
    SCOREBOARD.record(equal && r == c, || format!("count_players (training screen): original {} rust {}, differences in {:?}", r, c, first_differences(&mo, &mr)));
    r
}

/// The schedule's training word (+0x16) with new bits in place of `keep`'s complement, and the saved flag (+0x93)
/// cleared; 1.
unsafe fn set_training_bits(this: usize, keep: u16, bits: u32) -> u32 {
    write_u16(this + 0x16, (read_u16(this + 0x16) & keep) | bits as u16);
    write_u32(this + 0x93, 0);
    1
}

/// Sets the position training bits of the schedule's training word (+0x16, the bits outside 0xdf80) and clears its
/// saved flag; returns 1.
/// Original: `TRAINING_SCHEDULE::set_position_training` (0x8b5090, training_schedule.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_position_training(this: usize, bits: u32) -> u32 {
    covered!("sx21b_8 position training bits");
    set_training_bits(this, 0xdf80, bits) // sx21b_8: the position bits
}

/// Comparer for `set_position_training`: the 0x9b-byte schedule and the value side by side.
pub(crate) unsafe extern "thiscall" fn compare_set_position_training(this: usize, bits: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%);
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x9busize)], || o(this, bits), || set_position_training(this, bits));
    SCOREBOARD.record(equal && r == c, || format!("set_position_training (training screen): bits {:#x}, differences in {:?}", bits, first_differences(&mo, &mr)));
    r
}

/// Sets the side training bits of the schedule's training word (+0x16, the bits outside 0xb57f) and clears its saved
/// flag; returns 1.
/// Original: `TRAINING_SCHEDULE::set_side_training` (0x8b50c0, training_schedule.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_side_training(this: usize, bits: u32) -> u32 {
    covered!("sx21b_8 side training bits");
    set_training_bits(this, 0xb57f, bits)
}

/// Comparer for `set_side_training`: the 0x9b-byte schedule and the value side by side.
pub(crate) unsafe extern "thiscall" fn compare_set_side_training(this: usize, bits: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%);
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x9busize)], || o(this, bits), || set_side_training(this, bits));
    SCOREBOARD.record(equal && r == c, || format!("set_side_training (training screen): bits {:#x}, differences in {:?}", bits, first_differences(&mo, &mr)));
    r
}

/// The translated default name text of each schedule type (0..=7), or 0 for type 8 (no name written).
fn default_name_text(kind: u8) -> Option<usize> {
    Some(match kind {
        0xff => 0xa8ce94, // "Overview"
        0 => 0xa8ce64,    // "No Training"
        1 => 0xa8ce1c,    // "General"
        2 => 0xa8cdf0,    // "Fitness"
        3 => 0xa8cdc4,    // "Shooting"
        4 => 0xa8cd9c,    // "Skill"
        5 => 0xa8cd70,    // "Tactics"
        6 => 0xa8cd48,    // "Gk"
        7 => 0xa8cd04,    // "Unused"
        8 => return None,
        _ => 0,
    })
}

/// Writes the default name of the schedule's type into its 11-character name field (+9).
///
/// `TRAINING_SCHEDULE::set_default_name()`: the translated name of `training_schedule::TYPE` (Overview, No Training,
/// General, Fitness, Shooting, Skill, Tactics, Gk, Unused) into a local buffer; the 12 bytes at +9 cleared and 11
/// characters copied (`strncpy`). Type 8 writes nothing. Defect of the original: any other type copies the local
/// buffer as the stack left it (here: an empty name).
/// Original: `TRAINING_SCHEDULE::set_default_name` (0x8b50f0, training_schedule.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn set_default_name(this: usize) {
    let text = match default_name_text(read_u8(this + training_schedule::TYPE)) {
        Some(t) => t,
        None => return,
    };
    let mut name = [0u8; 0x64];
    if text != 0 {
        covered!("sx21b_8 default name");
        schedule_text(name.as_mut_ptr() as usize, text);
    }
    core::ptr::write_bytes((this + 9) as *mut u8, 0, 0xc);
    exe::at_9466d0(this + 9, name.as_ptr(), 0xb); // sx21b_8: eleven characters
}

/// Comparer for `set_default_name`: the 12-byte name side by side (a type outside the known ones copies stack
/// leftovers: only the original).
pub(crate) unsafe extern "thiscall" fn compare_set_default_name(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%);
    if default_name_text(read_u8(this + training_schedule::TYPE)) == Some(0) {
        return o(this);
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this + 9, 0xcusize)], || o(this), || set_default_name(this));
    SCOREBOARD.record(equal, || format!("set_default_name (training screen): type {}, original {:x?} rust {:x?}", read_u8(this + training_schedule::TYPE), mo, mr));
}

/// The fields a schedule file holds after its 0x6a-byte header: the five levels, the name, the type, the training
/// word and the 0x11 bytes at +0x18, as (offset, size, count).
const SCHEDULE_FILE_FIELDS: [(usize, u32, u32); 5] = [(4, 1, 5), (9, 1, 0xc), (0x15, 1, 1), (0x16, 2, 1), (0x18, 0x11, 1)];

/// Loads a saved training schedule file into this schedule after checking its header; 1 when loaded.
///
/// `TRAINING_SCHEDULE::load_from_file(path)`: no path, or a file that does not open ("rb"): 0. The 0x6a-byte header
/// must read whole, with magic 0x200cb and version byte 1 (or 0 and the file closed); it goes to +0x29. Then the
/// fields of `SCHEDULE_FILE_FIELDS` (a short read: closed and 0). The file's name (no folder, no extension,
/// `_splitpath`) goes to +0x2e, and both saved flags (+0x93, +0x97) become 1.
/// Original: `TRAINING_SCHEDULE::load_from_file` (0x8b5220, training_schedule.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_from_file(this: usize, path: usize) -> u32 {
    if path == 0 {
        return 0;
    }
    let f = exe::crt_fopen(path, 0x98f928); // "rb"
    if f == 0 {
        return 0;
    }
    let mut header = [0u8; 0x6a];
    if exe::crt_fread(header.as_mut_ptr() as usize, 0x6a, 1, f) != 1
        || u32::from_le_bytes([header[0], header[1], header[2], header[3]]) != SCHEDULE_FILE_MAGIC || header[4] != 1 {
        exe::crt_fclose(f);
        return 0;
    }
    covered!("sx21b_8 schedule file loaded");
    core::ptr::copy_nonoverlapping(header.as_ptr(), (this + 0x29) as *mut u8, 0x6a);
    for (at, size, count) in SCHEDULE_FILE_FIELDS {
        if exe::crt_fread(this + at, size, count, f) != count {
            exe::crt_fclose(f);
            return 0;
        }
    }
    exe::crt_fclose(f);
    exe::crt_strtok(path, 0usize, 0usize, (this + 0x2e) as *mut u8, core::ptr::null_mut()); // _splitpath: the file's name
    write_u32(this + 0x93, 1);
    write_u32(this + 0x97, 1);
    1
}

/// Comparer for `load_from_file`: it only reads the file, so the 0x9b-byte schedule and the value go side by side.
pub(crate) unsafe extern "thiscall" fn compare_load_from_file(this: usize, path: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I4%%);
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x9busize)], || o(this, path), || load_from_file(this, path));
    SCOREBOARD.record(equal && r == c, || format!("load_from_file (training schedule screen): original {} rust {}, differences in {:?}", r, c, first_differences(&mo, &mr)));
    r
}

/// Writes this schedule to a training schedule file; 1 when written.
///
/// `TRAINING_SCHEDULE::save_to_file(path)`: no path, or a file that does not open ("wb"): 0. The header (+0x29, 0x6a
/// bytes) and the fields of `SCHEDULE_FILE_FIELDS` (a short write: closed and 0); then the file's name goes to +0x2e
/// (`_splitpath`) and both saved flags (+0x93, +0x97) become 1.
/// Original: `TRAINING_SCHEDULE::save_to_file` (0x8b53d0, training_schedule.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_to_file(this: usize, path: usize) -> u32 {
    if path == 0 {
        return 0;
    }
    let f = exe::crt_fopen(path, 0x9a7f28); // "wb"
    if f == 0 {
        return 0;
    }
    if exe::crt_fwrite(this + 0x29, 0x6a, 1, f) != 1 {
        exe::crt_fclose(f);
        return 0;
    }
    for (at, size, count) in SCHEDULE_FILE_FIELDS {
        if exe::crt_fwrite(this + at, size, count, f) != count {
            exe::crt_fclose(f);
            return 0;
        }
    }
    covered!("sx21b_8 schedule file saved");
    exe::crt_fclose(f);
    exe::crt_strtok(path, 0usize, 0usize, (this + 0x2e) as *mut u8, core::ptr::null_mut());
    write_u32(this + 0x93, 1); // sx21b_8: saved
    write_u32(this + 0x97, 1);
    1
}

/// Comparer for `save_to_file`: it writes a file, so only the original runs; a saved schedule has both flags at 1.
pub(crate) unsafe extern "thiscall" fn compare_save_to_file(this: usize, path: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I5%%)(this, path);
    if r == 1 {
        let flags = (read_u32(this + 0x93), read_u32(this + 0x97));
        SCOREBOARD.record(flags == (1, 1), || format!("save_to_file (training schedule screen): saved flags {:?}", flags));
    }
    r
}

/// The translated name of a training category (0 Fitness, 1 Tactics, 2 Shooting, 3 Skills, 4 Goalkeeping).
fn category_name_text(category: i8) -> Option<usize> {
    Some(match category {
        0 => 0xa8cf74,
        1 => 0xa8cf48,
        2 => 0xa8cf1c,
        3 => 0xa8cef4,
        4 => 0xa8cec0,
        _ => return None,
    })
}

/// The translated name of a training intensity (0 None, 10 Light, 25 Medium, 50 Intensive).
fn intensity_name_text(level: i8) -> Option<usize> {
    Some(match level {
        0 => 0xa8d060,
        0xa => 0xa8d034,
        0x19 => 0xa8d004,
        0x32 => 0xa8cfd4, // sx21b_8: Intensive
        _ => return None,
    })
}

/// Writes the name of one of the five training categories into a buffer; 1, or 0 with the box for a buffer under 20
/// bytes (line 0x25e) or another category (line 0x27e).
/// Original: `training_schedule_get_category_name` (0x8b5520, training_schedule.cpp, cdecl).
pub(crate) unsafe extern "C" fn training_schedule_get_category_name(category: i8, buffer: usize, size: i32) -> u32 {
    if size < 0x14 {
        show_error_at(TRAINING_SCHEDULE_FILE, 0x25e);
        return 0;
    }
    match category_name_text(category) {
        Some(text) => {
            covered!("sx21b_8 category name");
            schedule_text(buffer, text);
            1
        }
        None => {
            show_error_at(TRAINING_SCHEDULE_FILE, 0x27e);
            0
        }
    }
}

/// Comparer for `training_schedule_get_category_name`: the text side by side (the box cases: only the original).
pub(crate) unsafe extern "C" fn compare_training_schedule_get_category_name(category: i8, buffer: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i8, usize, i32) -> u32>(%%I6%%);
    if size < 0x14 || category_name_text(category).is_none() {
        return o(category, buffer, size);
    }
    compare_schedule_text("training_schedule_get_category_name", buffer, size, || o(category, buffer, size), || training_schedule_get_category_name(category, buffer, size))
}

/// Writes the name of a training intensity into a buffer; 1, or 0 with the box for a buffer under 20 bytes (line
/// 0x294) or another level (line 0x2af).
/// Original: `training_schedule_get_intensity_name` (0x8b5690, training_schedule.cpp, cdecl).
pub(crate) unsafe extern "C" fn training_schedule_get_intensity_name(level: i8, buffer: usize, size: i32) -> u32 {
    if size < 0x14 {
        show_error_at(TRAINING_SCHEDULE_FILE, 0x294);
        return 0;
    }
    match intensity_name_text(level) {
        Some(text) => {
            covered!("sx21b_8 intensity name");
            schedule_text(buffer, text);
            1
        }
        None => {
            show_error_at(TRAINING_SCHEDULE_FILE, 0x2af);
            0
        }
    }
}

/// Comparer for `training_schedule_get_intensity_name`: the text side by side (the box cases: only the original).
pub(crate) unsafe extern "C" fn compare_training_schedule_get_intensity_name(level: i8, buffer: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i8, usize, i32) -> u32>(%%I7%%);
    if size < 0x14 || intensity_name_text(level).is_none() {
        return o(level, buffer, size);
    }
    compare_schedule_text("training_schedule_get_intensity_name", buffer, size, || o(level, buffer, size), || training_schedule_get_intensity_name(level, buffer, size))
}

/// Side by side over a text buffer (at most 0x100 bytes of it) and the value.
unsafe fn compare_schedule_text(name: &str, buffer: usize, size: i32, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let n = (size as usize).min(0x100);
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(buffer, n)], original, rust);
    SCOREBOARD.record(equal && r == c, || format!("{} (training screen): original {} rust {}, differences in {:?}", name, r, c, first_differences(&mo, &mr)));
    r
}

/// Tells the file dialog whether a file is a valid training schedule file: 1 when its header reads with the right
/// magic and version (`training_schedule_read_file_header`), otherwise 2.
/// Original: `training_schedule_check_file` (0x8b57e0, training_schedule.cpp, cdecl).
pub(crate) unsafe extern "C" fn training_schedule_check_file(path: usize) -> u32 {
    let mut header = [0u8; 0x6c];
    if training_schedule_read_file_header(path, header.as_mut_ptr() as usize) != 0 {
        covered!("sx21b_8 valid schedule file");
        1
    } else {
        2 // sx21b_8: not a schedule file
    }
}

/// Comparer for `training_schedule_check_file`: the value.
pub(crate) unsafe extern "C" fn compare_training_schedule_check_file(path: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I8%%)(path);
    if path != 0 {
        let c = training_schedule_check_file(path);
        SCOREBOARD.record(r == c, || format!("training_schedule_check_file (file dialog): original {} rust {}", r, c));
    }
    r
}

/// Reads the 0x6a-byte header of a training schedule file into `out` (cleared first) and returns 1 when its magic is
/// 0x200cb and its version byte 1; no path: the box (line 0x2e0) and 0; a file that does not open or reads short: 0.
/// Original: `training_schedule_read_file_header` (0x8b5820, training_schedule.cpp, cdecl).
pub(crate) unsafe extern "C" fn training_schedule_read_file_header(path: usize, out: usize) -> u32 {
    core::ptr::write_bytes(out as *mut u8, 0, 0x6a);
    if path == 0 {
        show_error_at(TRAINING_SCHEDULE_FILE, 0x2e0);
        return 0;
    }
    let f = exe::crt_fopen(path, 0x98f928); // "rb"
    if f == 0 {
        return 0;
    }
    if exe::crt_fread(out, 0x6a, 1, f) != 1 {
        exe::crt_fclose(f);
        return 0;
    }
    exe::crt_fclose(f);
    (read_u32(out) == SCHEDULE_FILE_MAGIC && read_u8(out + 4) == 1) as u32
}

/// Comparer for `training_schedule_read_file_header`: the header and the value side by side (no path: only the
/// original, which shows the box).
pub(crate) unsafe extern "C" fn compare_training_schedule_read_file_header(path: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I9%%);
    if path == 0 {
        return o(path, out);
    }
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(out, 0x6ausize)], || o(path, out), || training_schedule_read_file_header(path, out));
    SCOREBOARD.record(equal && r == c, || format!("training_schedule_read_file_header (file dialog): original {} rust {}, differences in {:?}", r, c, first_differences(&mo, &mr)));
    r
}
