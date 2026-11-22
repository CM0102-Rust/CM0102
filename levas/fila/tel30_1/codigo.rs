// ---------- batch %%LEVA%%: the background pictures (1): pics.cfg, the random picture and the finders (background.cpp) ----------

/// u32: the background pictures are on (the game options); off, every setter puts ":" (no picture).
pub(crate) const BACKGROUND_PICTURES_ON: usize = 0x989858;
/// The 11 sections of pics.cfg (nations, clubs, stadiums, comps, awards, staff, training, boardroom, trophy, transfer,
/// si), 10 bytes each: +0 the section name, +4 i16 the subsection count, +6 the subsection array.
pub(crate) const BACKGROUND_SECTIONS: usize = 0x989860;
pub(crate) const BACKGROUND_SECTION_SIZE: usize = 10;
pub(crate) const BACKGROUND_SECTION_COUNT: usize = 11;
/// The sections whose subsections name a database record.
pub(crate) const BACKGROUND_NATIONS: usize = 0;
pub(crate) const BACKGROUND_CLUBS: usize = 1;
pub(crate) const BACKGROUND_STADIUMS: usize = 2;
pub(crate) const BACKGROUND_COMPS: usize = 3;
pub(crate) const BACKGROUND_AWARDS: usize = 4;
pub(crate) const BACKGROUND_STAFF: usize = 5;
/// A subsection (16 bytes): +0 its name, +4 / +6 i16 the counts of its two picture lists, +8 / +0xc the lists
/// (arrays of file names).
#[allow(dead_code)]
pub(crate) mod background_sub {
    pub(crate) const NAME: usize = 0x0;
    pub(crate) const COUNT: usize = 0x4;
    pub(crate) const SECOND_COUNT: usize = 0x6;
    pub(crate) const LIST: usize = 0x8;
    pub(crate) const SECOND_LIST: usize = 0xc;
    pub(crate) const SIZE: usize = 0x10;
}
/// "CM3_PICTURES" (the folder key), "pics.cfg", "r", "-verifypics", the command line, ":" (no picture), "*" (an
/// unnamed staff part), "stadium", "old", "_%s", "%s %s %d/%d/%d" and the background.cpp path for the error box.
const BACKGROUND_FOLDER: usize = 0x9899e0;
const BACKGROUND_CFG: usize = 0x9899f0;
const BACKGROUND_READ: usize = 0x9899fc;
const BACKGROUND_VERIFY_OPTION: usize = 0x9899d4;
const BACKGROUND_COMMAND_LINE: usize = 0xae2c98;
pub(crate) const BACKGROUND_NONE: usize = 0x989adc;
const BACKGROUND_UNNAMED: usize = 0x989ad8;
const BACKGROUND_STADIUM_DIR: usize = 0x989a78;
const BACKGROUND_OLD_DIR: usize = 0x989a74;
const BACKGROUND_UNDERSCORE: usize = 0x989a70;
const BACKGROUND_STAFF_FORMAT: usize = 0x989ac8;
const BACKGROUND_CPP: usize = 0x989934;
/// The messages: "Invalid section found", "Subsection found before section", "Picture file \"%s\" not found",
/// "Unable to allocate memory", "%s \"%s\" not found" (verify) and "Verifying Pictures...".
const BACKGROUND_MSG_INVALID_SECTION: usize = 0x9899a0;
const BACKGROUND_MSG_SUBSECTION_FIRST: usize = 0x989964;
const BACKGROUND_MSG_PICTURE_MISSING: usize = 0x989a38;
const BACKGROUND_MSG_NO_MEMORY: usize = 0x989a00;
const BACKGROUND_MSG_NOT_FOUND: usize = 0x989a98;
const BACKGROUND_MSG_VERIFYING: usize = 0x989a80;
/// The shared text buffer (0xde1f64) and the "Error" title.
const BACKGROUND_TEXT: usize = 0xde1f64;
const BACKGROUND_ERROR_TITLE: usize = 0x9870e0;

/// Section `k`'s fields (its index taken as a signed 16-bit value, like the original's `movsx`).
pub(crate) fn background_section(k: i32) -> usize {
    (BACKGROUND_SECTIONS as isize + k as isize * BACKGROUND_SECTION_SIZE as isize) as usize
}
pub(crate) unsafe fn background_sub_count(k: i32) -> i16 {
    read_i16(background_section(k) + 4)
}
pub(crate) unsafe fn background_subs(k: i32) -> usize {
    read_usize(background_section(k) + 6)
}

/// `sprintf` of the game's CRT.
pub(crate) unsafe fn background_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// `ScreenManager::get_background_picture` / `set_background_picture` on the screen manager (`G_SCREEN_MANAGER`).
pub(crate) unsafe fn background_current() -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x7fee80usize);
    f(G_SCREEN_MANAGER)
}
pub(crate) unsafe fn background_set(title: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7feeb0usize);
    f(G_SCREEN_MANAGER, title)
}

/// The current screen takes a picture: its picture is still "\" (a new screen) or empty (asked twice, as the
/// original does).
pub(crate) unsafe fn background_screen_open() -> bool {
    read_u8(background_current()) == b'\\' || read_u8(background_current()) == 0
}

/// Sets the picture file `name` of the pictures folder (`utils_build_file_path("CM3_PICTURES", name)`).
pub(crate) unsafe fn background_set_file(name: usize) {
    background_set(exe::utils_build_file_path(BACKGROUND_FOLDER, name));
}

/// The "-verifypics" option is on the command line.
unsafe fn background_verifying() -> bool {
    exe::crt_strstr(BACKGROUND_COMMAND_LINE, BACKGROUND_VERIFY_OPTION) != 0
}

/// The fatal "Error" box of background.cpp at a line (`tel26_error_box`), then `game_fatal_cleanup`, the empty
/// 0x5a8f60 and `exit(-1)`.
unsafe fn background_fatal(line: u32) {
    tel26_error_box(BACKGROUND_CPP, line);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
}

/// Reads pics.cfg (the pictures folder) into the 11 sections: "[name]" lines choose the section, the other lines
/// are subsections "name:picture,picture{picture}..." whose pictures go to the lists.
///
/// Without the file nothing happens. Each line (`utils_read_config_line`, until the end-of-file flag 0x10 of the
/// FILE): a '[' line is cut at the first ']' and its name looked up (stricmp) among the 11 section names; a known one
/// becomes the current section, an unknown one leaves -1 (with "-verifypics" on the command line: the box
/// "setup_background_system() : Invalid section found"). Any other line before a section, with "-verifypics", shows
/// "Subsection found before section" and is skipped; without the option it is read into section -1 (a defect of the
/// original: the 10 bytes before the table). A subsection line: the text before ':' is its name; the subsection array
/// grows by one 16-byte entry (`realloc`; failing, the file is closed and the fatal box of background.cpp line 0x90),
/// the name is copied to a new block (`malloc`; failing, line 0x95) and the entry's counts and lists cleared. Then
/// the characters after ':' are read: spaces are skipped, ',', '}' and the end add the picture collected so far
/// (`background_add_picture` with the list flag, when the picture is not empty, then the flag goes back to 0; an error
/// text closes the file and gives the fatal box line 0xbe), '{' sets the flag to the second list first; the rest is
/// collected. A subsection left without pictures in both lists is freed and taken off the count. The file is closed
/// at the end.
/// Original: `setup_background_system` (0x41b3d0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_background_system() {
    covered!("tel30_1 pics.cfg");
    let f = exe::crt_fopen(exe::utils_build_file_path(BACKGROUND_FOLDER, BACKGROUND_CFG), BACKGROUND_READ);
    if f == 0 {
        return;
    }
    let mut section: i32 = -1;
    let mut line = [0u8; 0x800];
    let mut name = [0u8; 0x800];
    let mut picture = [0u8; 0x800];
    while read_u8(f + 0xc) & 0x10 == 0 {
        if exe::utils_read_config_line(f, line.as_mut_ptr()) == 0 {
            break;
        }
        if line[0] == b'[' {
            let mut k = 1;
            while line[k] != 0 && line[k] != b']' {
                k += 1;
            }
            line[k] = 0;
            section = -1;
            let mut found = false;
            for s in 0..BACKGROUND_SECTION_COUNT as i32 {
                if exe::crt_stricmp(read_usize(background_section(s)), line.as_ptr().add(1) as usize) == 0 {
                    section = s;
                    found = true;
                    break;
                }
            }
            if !found && background_verifying() {
                exe::gui_show_message_box(BACKGROUND_ERROR_TITLE, BACKGROUND_MSG_INVALID_SECTION, 0);
            }
            continue;
        }
        if section == -1 && background_verifying() {
            exe::gui_show_message_box(BACKGROUND_ERROR_TITLE, BACKGROUND_MSG_SUBSECTION_FIRST, 0);
            continue;
        }
        // the subsection name: up to ':' (or the end), then the pictures from the next character
        let mut k = 0;
        let mut last = line[0];
        name[0] = last;
        while last != 0 && last != b':' {
            k += 1;
            last = line[k];
            name[k] = last;
        }
        name[k] = 0;
        let mut at = k + 1;
        let entry_count = background_sub_count(section) as i32;
        let subs = exe::crt_realloc(background_subs(section), ((entry_count + 1) << 4) as u32);
        if subs == 0 {
            exe::crt_fclose(f);
            background_fatal(0x90);
            return;
        }
        let n = name.iter().position(|&b| b == 0).unwrap_or(0) + 1;
        let block = exe::crt_malloc(n as u32);
        let entry = (subs as isize + (entry_count as isize) * 16) as usize;
        write_usize(entry + background_sub::NAME, block);
        if block == 0 {
            exe::crt_fclose(f);
            background_fatal(0x95);
            return;
        }
        write_usize(background_section(section) + 6, subs);
        write_i16(background_section(section) + 4, read_i16(background_section(section) + 4).wrapping_add(1));
        write_u16(entry + background_sub::COUNT, 0);
        write_u16(entry + background_sub::SECOND_COUNT, 0);
        write_usize(entry + background_sub::LIST, 0);
        write_usize(entry + background_sub::SECOND_LIST, 0);
        core::ptr::copy_nonoverlapping(name.as_ptr(), block as *mut u8, n);
        let mut second = 0u32;
        let mut length: i16 = 0;
        let mut c = last;
        while c != 0 {
            c = line[at];
            at += 1;
            let add = match c {
                0 | b',' | b'}' => true,
                b'{' => {
                    second = 1;
                    true
                }
                b' ' => false,
                _ => {
                    // ponytail: a picture longer than the 0x800 buffer is cut (the original overruns its stack)
                    if let Some(b) = picture.get_mut(length as u16 as usize) {
                        *b = c;
                    }
                    length = length.wrapping_add(1);
                    false
                }
            };
            if add {
                if let Some(b) = picture.get_mut(length as u16 as usize) {
                    *b = 0;
                }
                if picture[0] != 0 {
                    if background_add_picture(entry, picture.as_mut_ptr() as usize, second) != 0 {
                        exe::crt_fclose(f);
                        background_fatal(0xbe);
                        return;
                    }
                    second = 0;
                    length = 0;
                }
            }
        }
        if read_u16(entry + background_sub::COUNT) == 0 && read_u16(entry + background_sub::SECOND_COUNT) == 0 {
            exe::crt_free(read_usize(entry + background_sub::NAME));
            write_usize(entry + background_sub::NAME, 0);
            write_i16(background_section(section) + 4, read_i16(background_section(section) + 4).wrapping_sub(1));
        }
    }
    exe::crt_fclose(f);
}

/// Adds the picture file `name` to one of the subsection's two lists, once the file is found; returns 0 or the error
/// text.
///
/// The file is looked for in the pictures folder as given, then under "stadium\", then under "old\"
/// (`_makepath`); while not found, a name already starting with '_' gives up (with "-verifypics": the box
/// "setup_background_system() : Picture file \"%s\" not found" in the shared text buffer) and returns 0, otherwise
/// the caller's name gets a '_' in front ("_%s") and the search starts again. The found file is closed; the list
/// (`second` 0: `COUNT`/`LIST`, otherwise `SECOND_COUNT`/`SECOND_LIST`) grows by one pointer (`realloc`), the count
/// goes up and the slot takes a copy of the path found (`_strdup`). A failed `realloc` (or copy) returns
/// "setup_background_system() : Unable to allocate memory".
/// Original: `background_add_picture` (0x41b8c0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_add_picture(sub: usize, name: usize, second: u32) -> usize {
    let mut local = [0u8; 0x810];
    copy_c_string(local.as_mut_ptr() as usize, name);
    let open = |path: *const u8| exe::crt_fopen(exe::utils_build_file_path(BACKGROUND_FOLDER, path as usize), BACKGROUND_READ);
    let f = loop {
        let f = open(local.as_ptr());
        if f != 0 {
            break f;
        }
        exe::crt_makepath(local.as_mut_ptr(), 0, BACKGROUND_STADIUM_DIR, name, 0);
        let f = open(local.as_ptr());
        if f != 0 {
            break f;
        }
        exe::crt_makepath(local.as_mut_ptr(), 0, BACKGROUND_OLD_DIR, name, 0);
        let f = open(local.as_ptr());
        if f != 0 {
            break f;
        }
        if read_u8(name) == b'_' {
            if background_verifying() {
                background_sprintf()(BACKGROUND_TEXT, BACKGROUND_MSG_PICTURE_MISSING, name);
                exe::gui_show_message_box(BACKGROUND_ERROR_TITLE, BACKGROUND_TEXT, 0);
            }
            return 0;
        }
        background_sprintf()(local.as_mut_ptr() as usize, BACKGROUND_UNDERSCORE, name);
        copy_c_string(name, local.as_ptr() as usize);
    };
    exe::crt_fclose(f);
    let (count_at, list_at) = if second == 0 {
        (sub + background_sub::COUNT, sub + background_sub::LIST)
    } else {
        (sub + background_sub::SECOND_COUNT, sub + background_sub::SECOND_LIST)
    };
    let n = read_i16(count_at) as i32;
    let list = exe::crt_realloc(read_usize(list_at), (n * 4 + 4) as u32);
    if list == 0 {
        return BACKGROUND_MSG_NO_MEMORY;
    }
    write_i16(count_at, read_i16(count_at).wrapping_add(1));
    write_usize(list_at, list);
    let strdup: unsafe extern "C" fn(*const u8) -> usize = core::mem::transmute(0x952f5busize);
    let slot = (read_usize(list_at) as isize + n as isize * 4) as usize;
    write_usize(slot, strdup(local.as_ptr()));
    if read_usize((read_usize(list_at) as isize + n as isize * 4) as usize) != 0 {
        return 0;
    }
    BACKGROUND_MSG_NO_MEMORY
}

/// The label of a staff subsection: "first second day/month/year" (`staff::FIRST_NAME` and `SECOND_NAME`, "*" when
/// missing; the day and month + 1 from the date of birth, the year its raw word).
pub(crate) unsafe fn background_staff_label(person: usize, out: &mut [u8; 0x200]) {
    let second = match read_usize(person + staff::SECOND_NAME) { 0 => BACKGROUND_UNNAMED, s => s };
    let first = match read_usize(person + staff::FIRST_NAME) { 0 => BACKGROUND_UNNAMED, s => s };
    let date = person + staff::DATE_OF_BIRTH;
    let year = read_u16(person + staff::DATE_OF_BIRTH_YEAR) as u32;
    let month = exe::date_get_month(date) as i8 as i32 + 1;
    let day = exe::date_get_day_of_month(date) as i16 as i32;
    background_sprintf()(out.as_mut_ptr() as usize, BACKGROUND_STAFF_FORMAT, first, second, day, month, year);
}

/// Checks the subsection names of the first six sections against the database (with "-verifypics"), showing the
/// progress.
///
/// The total is the sum of the subsection counts of the sections that have an array. Each subsection of each section:
/// a named one of nations, clubs, stadiums, comps (club competitions), awards (staff competitions, count at 0xae2388)
/// or staff is looked for among the records (stricmp with the English name; staff by `background_staff_label`), and a
/// missing one shows the box "verify_background_config() : %s \"%s\" not found" (section, subsection); the other
/// sections are not checked. After each subsection the text "Verifying Pictures..." (langlib) goes out as a status
/// message (`ScreenManager::send_network_status_message(text, done, total, -1)`).
/// Original: `verify_background_config` (0x41bc20, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn verify_background_config() {
    covered!("tel30_1 verify pics.cfg");
    let mut total: u32 = 0;
    for k in 0..BACKGROUND_SECTION_COUNT as i32 {
        if background_subs(k) != 0 {
            total = total.wrapping_add(background_sub_count(k) as i32 as u32);
        }
    }
    type NameOf = unsafe extern "C" fn(usize) -> usize;
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut done: i32 = 0;
    for k in 0..BACKGROUND_SECTION_COUNT as i32 {
        if background_subs(k) == 0 {
            continue;
        }
        let mut j: i16 = 0;
        while j < background_sub_count(k) {
            let name = read_usize(background_subs(k) + j as usize * 16);
            if name != 0 {
                let table: Option<(usize, usize, usize, NameOf)> = match k as usize {
                    BACKGROUND_NATIONS => Some((G_NATION_COUNT, G_NATION_TABLE, 0x122, core::mem::transmute(0x5453f0usize))),
                    BACKGROUND_CLUBS => Some((G_CLUB_COUNT, G_CLUB_TABLE, 0x245, core::mem::transmute(FN_CLUB_GET_ENGLISH_NAME))),
                    BACKGROUND_STADIUMS => Some((G_STADIUM_COUNT, G_STADIUM_TABLE, 0x4e, core::mem::transmute(0x545990usize))),
                    BACKGROUND_COMPS => Some((G_CLUB_COMP_COUNT, G_CLUB_COMP_TABLE, 0x6b, core::mem::transmute(0x5455a0usize))),
                    BACKGROUND_AWARDS => Some((0xae2388, G_STAFF_COMP_TABLE, 0x65, core::mem::transmute(0x5456a0usize))),
                    _ => None,
                };
                let found = if let Some((count, base, size, english)) = table {
                    let mut hit = false;
                    for i in 0..read_i32(count).max(0) as usize {
                        if exe::crt_stricmp(name, english(read_usize(base) + i * size)) == 0 {
                            hit = true;
                            break;
                        }
                    }
                    hit
                } else if k as usize == BACKGROUND_STAFF {
                    let mut hit = false;
                    let mut label = [0u8; 0x200];
                    for i in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
                        background_staff_label(read_usize(G_STAFF_TABLE) + i * staff::SIZE, &mut label);
                        if exe::crt_stricmp(name, label.as_ptr() as usize) == 0 {
                            hit = true;
                            break;
                        }
                    }
                    hit
                } else {
                    true
                };
                if !found {
                    background_sprintf()(BACKGROUND_TEXT, BACKGROUND_MSG_NOT_FOUND, read_usize(background_section(k)), name);
                    exe::gui_show_message_box(BACKGROUND_ERROR_TITLE, BACKGROUND_TEXT, 0);
                }
            }
            langlib(BACKGROUND_TEXT, BACKGROUND_MSG_VERIFYING);
            exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, BACKGROUND_TEXT, done, total, -1);
            done = done.wrapping_add(1);
            j = j.wrapping_add(1);
        }
    }
}

/// Puts a random picture on a screen that takes one: up to 6 sections from a random one of the first six (stadiums
/// skipped), each with a random subsection and a random picture of its first list; none found (or the pictures off)
/// puts ":".
///
/// Nothing happens when the screen's picture is neither "\" nor empty. With the pictures on: the section starts at
/// `utils_random(6)`; for each of 6 tries, a section with an array that is not the stadiums draws a subsection
/// (`utils_random(count)`) and, when it has a first list, a picture (`utils_random(its count)`): a non-null one is
/// set (in the pictures folder) and the routine ends; then the next section (back to 0 after 5).
/// Original: `background_set_random_picture` (0x41bfb0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_random_picture() {
    if !background_screen_open() {
        return;
    }
    if read_u32(BACKGROUND_PICTURES_ON) != 0 {
        let mut k = exe::utils_random(6) as i16;
        let mut tries: i16 = 0;
        while tries < 6 {
            let subs = background_subs(k as i32);
            if subs != 0 && k != BACKGROUND_STADIUMS as i16 {
                let s = exe::utils_random(background_sub_count(k as i32) as i32) as i16;
                let sub = (background_subs(k as i32) as isize + s as isize * 16) as usize;
                if read_usize(sub + background_sub::LIST) != 0 {
                    let p = exe::utils_random(read_i16(sub + background_sub::COUNT) as i32) as i16;
                    let list = read_usize((background_subs(k as i32) as isize + s as isize * 16) as usize + background_sub::LIST);
                    let file = read_usize((list as isize + p as isize * 4) as usize);
                    if file != 0 {
                        covered!("tel30_1 random picture");
                        background_set_file(file);
                        return;
                    }
                }
            }
            tries = tries.wrapping_add(1);
            k = k.wrapping_add(1);
            if k >= 6 {
                k = 0;
            }
        }
    }
    background_set(BACKGROUND_NONE);
}

/// Sets the given picture path on the current screen; with the pictures off a non-null path becomes ":" (a null one
/// passes as it is).
/// Original: `background_set_picture` (0x41c0d0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_picture(title: usize) {
    if read_u32(BACKGROUND_PICTURES_ON) == 0 && title != 0 {
        background_set(BACKGROUND_NONE);
        return;
    }
    background_set(title);
}

/// The finders' common body: with the pictures on and the section's array present, the subsection of `record`
/// (the last whose name matches the record's, stricmp; none gives 0) or, for a null record, a random one
/// (`utils_random(count)`); then a random picture of its first list (`second` 0) or second list, or 0 without that
/// list.
unsafe fn background_find(k: i32, record: usize, second: u32, matches: impl Fn(usize) -> bool) -> usize {
    if read_u32(BACKGROUND_PICTURES_ON) == 0 || background_subs(k) == 0 {
        return 0;
    }
    let s: i16 = if record == 0 {
        exe::utils_random(background_sub_count(k) as i32) as i16
    } else {
        let mut hit: i16 = -1;
        let mut j: i16 = 0;
        if background_sub_count(k) < 1 {
            return 0;
        }
        while j < background_sub_count(k) {
            if matches(read_usize((background_subs(k) as isize + j as isize * 16) as usize)) {
                hit = j;
            }
            j = j.wrapping_add(1);
        }
        hit
    };
    if s == -1 {
        return 0;
    }
    let (count, list) = if second != 0 {
        (background_sub::SECOND_COUNT, background_sub::SECOND_LIST)
    } else {
        (background_sub::COUNT, background_sub::LIST)
    };
    let sub = |k: i32| (background_subs(k) as isize + s as isize * 16) as usize;
    if read_usize(sub(k) + list) == 0 {
        return 0;
    }
    let p = exe::utils_random(read_i16(sub(k) + count) as i32) as i16;
    read_usize((read_usize(sub(k) + list) as isize + p as isize * 4) as usize)
}

/// A finder of the database sections: the subsection whose name matches (stricmp) the record's English name.
unsafe fn background_find_named(k: i32, record: usize, second: u32, english: usize) -> usize {
    let english: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(english);
    background_find(k, record, second, |name| exe::crt_stricmp(name, english(record)) == 0)
}

/// A picture of the nation's subsection (by `nation_get_english_name`) or, for a null nation, of a random one;
/// `second` picks the list; 0 when none (`background_find`).
/// Original: `background_find_nation_picture` (0x41c9d0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_find_nation_picture(record: usize, second: u32) -> usize {
    background_find_named(BACKGROUND_NATIONS as i32, record, second, 0x5453f0)
}

/// A picture of the club's subsection (by `club_get_english_name`) or of a random one (`background_find`).
/// Original: `background_find_club_picture` (0x41cad0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_find_club_picture(record: usize, second: u32) -> usize {
    background_find_named(BACKGROUND_CLUBS as i32, record, second, FN_CLUB_GET_ENGLISH_NAME)
}

/// A picture of the club competition's subsection (by `comp_get_english_name`) or of a random one.
/// Original: `background_find_comp_picture` (0x41cbd0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_find_comp_picture(record: usize, second: u32) -> usize {
    background_find_named(BACKGROUND_COMPS as i32, record, second, 0x5455a0)
}

/// A picture of the award's subsection (the staff competition, by `staff_comp_get_english_name`) or of a random one.
/// Original: `background_find_award_picture` (0x41ccd0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_find_award_picture(record: usize, second: u32) -> usize {
    background_find_named(BACKGROUND_AWARDS as i32, record, second, 0x5456a0)
}

/// A picture of the stadium's subsection (by `stadium_get_english_name`) or of a random one.
/// Original: `background_find_stadium_picture` (0x41cdd0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_find_stadium_picture(record: usize, second: u32) -> usize {
    background_find_named(BACKGROUND_STADIUMS as i32, record, second, 0x545990)
}

/// A picture of the staff member's subsection (by `background_staff_label`, rebuilt for each subsection) or of a
/// random one.
/// Original: `background_find_staff_picture` (0x41ced0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_find_staff_picture(record: usize, second: u32) -> usize {
    background_find(BACKGROUND_STAFF as i32, record, second, |name| {
        let mut label = [0u8; 0x200];
        background_staff_label(record, &mut label);
        exe::crt_stricmp(name, label.as_ptr() as usize) == 0
    })
}

/// The regions a background picture routine changes: the generator and the current screen's picture path.
pub(crate) unsafe fn background_regions() -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node != 0 {
        reg.push((node + screen_node_fields::PICTURE, screen_node_fields::PICTURE_SIZE));
    }
    reg
}

/// A background routine side by side over `background_regions` and its return value.
pub(crate) unsafe fn background_compare<R: PartialEq + core::fmt::Debug + Copy>(name: &str, original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> R {
    let reg = background_regions();
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, original, || crate::rng::without_counting(rust));
    SCOREBOARD.record(same && r == n, || format!("background {}: original={:?} rust={:?} differences {:?}", name, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `setup_background_system`.
pub(crate) unsafe extern "C" fn compare_setup_background_system() {
    // simplification: reading pics.cfg allocates the tables once; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `background_add_picture`.
pub(crate) unsafe extern "C" fn compare_background_add_picture(sub: usize, name: usize, second: u32) -> usize {
    // simplification: the list grows (realloc, strdup) once; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, usize, u32) -> usize>(%%I1%%)(sub, name, second)
}

/// Comparer for `verify_background_config`.
pub(crate) unsafe extern "C" fn compare_verify_background_config() {
    // simplification: the boxes and the status messages go out once; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

/// Comparer for `background_set_random_picture`: side by side over the generator and the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_random_picture() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I3%%);
    background_compare("set_random_picture", || o(), || background_set_random_picture());
}

/// Comparer for `background_set_picture`: side by side over the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_picture(title: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(%%I4%%);
    if title != 0 && c_string_len(title) >= screen_node_fields::PICTURE_SIZE {
        return o(title);
    }
    background_compare("set_picture", || o(title), || background_set_picture(title));
}

/// Comparer for `background_find_nation_picture`: the picture and the generator side by side.
pub(crate) unsafe extern "C" fn compare_background_find_nation_picture(record: usize, second: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I5%%);
    background_compare("find_nation_picture", || o(record, second), || background_find_nation_picture(record, second))
}

/// Comparer for `background_find_club_picture`: the picture and the generator side by side.
pub(crate) unsafe extern "C" fn compare_background_find_club_picture(record: usize, second: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I6%%);
    background_compare("find_club_picture", || o(record, second), || background_find_club_picture(record, second))
}

/// Comparer for `background_find_comp_picture`: the picture and the generator side by side.
pub(crate) unsafe extern "C" fn compare_background_find_comp_picture(record: usize, second: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I7%%);
    background_compare("find_comp_picture", || o(record, second), || background_find_comp_picture(record, second))
}

/// Comparer for `background_find_award_picture`: the picture and the generator side by side.
pub(crate) unsafe extern "C" fn compare_background_find_award_picture(record: usize, second: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I8%%);
    background_compare("find_award_picture", || o(record, second), || background_find_award_picture(record, second))
}

/// Comparer for `background_find_stadium_picture`: the picture and the generator side by side.
pub(crate) unsafe extern "C" fn compare_background_find_stadium_picture(record: usize, second: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I9%%);
    background_compare("find_stadium_picture", || o(record, second), || background_find_stadium_picture(record, second))
}

/// Comparer for `background_find_staff_picture`: the picture and the generator side by side.
pub(crate) unsafe extern "C" fn compare_background_find_staff_picture(record: usize, second: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I10%%);
    background_compare("find_staff_picture", || o(record, second), || background_find_staff_picture(record, second))
}
