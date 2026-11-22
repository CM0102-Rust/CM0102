// ---------- batch %%LEVA%%: printouts.cpp's static dates and the pundit list reader (printouts.cpp) ----------

/// Rule: printouts.cpp's first static date (0xb74360) is built empty at program start.
///
/// `printouts_static_init_date_b74360()`: `Date::Date` on 0xb74360 and its empty destructor registered with `atexit`.
/// Original: `printouts_static_init_date_b74360` (0x7d4400, printouts.cpp, cdecl).
pub(crate) unsafe extern "C" fn printouts_static_init_date_b74360() {
    exe::date_date_54a110(0xb74360usize);
    exe::crt_atexit(0x7d4420usize);
}

/// The destructor of the date 0xb74360: nothing.
/// Original: `printouts_static_destroy_date_b74360` (0x7d4420, printouts.cpp, cdecl).
pub(crate) unsafe extern "C" fn printouts_static_destroy_date_b74360() {}

/// Rule: printouts.cpp's second static date (0xb74358) is built empty at program start.
///
/// `printouts_static_init_date_b74358()`: `Date::Date` on 0xb74358 and its empty destructor registered with `atexit`.
/// Original: `printouts_static_init_date_b74358` (0x7d4430, printouts.cpp, cdecl).
pub(crate) unsafe extern "C" fn printouts_static_init_date_b74358() {
    exe::date_date_54a110(0xb74358usize);
    exe::crt_atexit(0x7d4450usize);
}

/// The destructor of the date 0xb74358: nothing.
/// Original: `printouts_static_destroy_date_b74358` (0x7d4450, printouts.cpp, cdecl).
pub(crate) unsafe extern "C" fn printouts_static_destroy_date_b74358() {}

/// The media type names of the pundit file and their codes.
const PUNDIT_MEDIA_TYPES: [(usize, i8); 6] = [(0xa794a8, 1), (0xa79498, 7), (0xa79490, 0), (0xa79488, 3), (0xa7947c, 5), (0xa79470, 4)];
/// "LOCAL_DAILY" (code 2; the last test).
const PUNDIT_LOCAL_DAILY: usize = 0xa79464;

/// Rule: a pundit's media type name gives its code: NATIONAL_DAILY 1, NATIONAL_SUNDAY 7, WEBSITE 0, RADIO 3, MAGAZINE
/// 5, LOCAL_RADIO 4, LOCAL_DAILY 2; any other name -1.
///
/// `parse_media_type_name(name)`: `strncmp` (100 characters) against each name in that order.
/// Original: `parse_media_type_name` (0x7daa70, printouts.cpp, thiscall, ret 4; `this` is not read).
pub(crate) unsafe extern "thiscall" fn parse_media_type_name(_this: usize, name: usize) -> i8 {
    for (text, code) in PUNDIT_MEDIA_TYPES {
        if exe::crt_strncmp(name, text, 0x64) == 0 {
            return code;
        }
    }
    if exe::crt_strncmp(name, PUNDIT_LOCAL_DAILY, 0x64) == 0 { 2 } else { -1 }
}

/// Rule: a pundit file token is the text between a pair of double quotes (at most 98 characters are kept); blanks
/// before it are skipped.
///
/// `PunditList::parse_quoted_token(src, out)`: reads `src` from the start: a quote opens the token (a second one
/// closes it and ends); spaces, CR and LF are skipped while nothing was kept (after that they are kept like any other
/// character while inside quotes); a NUL ends; past 0x61 kept characters it ends; characters inside the quotes are
/// kept. `out` is terminated and the address one past the character after the end comes back (one past the closing
/// quote's next character).
/// Original: `PunditList::parse_quoted_token` (0x7dab30, printouts.cpp, thiscall, ret 8; `this` is not read).
pub(crate) unsafe extern "thiscall" fn pundit_list_parse_quoted_token(_this: usize, src: usize, out: usize) -> usize {
    let (mut n, mut i, mut inside) = (0i32, 0usize, false);
    loop {
        let c = read_u8(src + i);
        i += 1;
        if c == b'"' {
            if inside {
                break;
            }
            inside = true;
            continue;
        }
        if (c == b' ' || c == b'\r' || c == b'\n') && n == 0 {
            continue;
        }
        if c == 0 {
            break;
        }
        if n > 0x61 {
            break;
        }
        if inside {
            write_u8(out + n as usize, c);
            n += 1;
        }
    }
    write_u8(out + n as usize, 0);
    src + i + 1
}

/// A pundit list record (0xd4 bytes, LLIST items): the serial (+0), the name (+2, 100 bytes), the media type (+0xca),
/// the city (+0xcb), the nation (+0xcf) and the id's low byte (+0xd3). Bytes 0x66..0xc9 are never written by the
/// original (stack garbage); the copy leaves them 0.
const PUNDIT_RECORD_SIZE: usize = 0xd4;
const PUNDIT_NAME: usize = 0x2;
const PUNDIT_MEDIA: usize = 0xca;
const PUNDIT_CITY: usize = 0xcb;
const PUNDIT_NATION: usize = 0xcf;
const PUNDIT_ID: usize = 0xd3;
/// The PunditList fields: the national list (+0, an LLIST pointer), the local list (+4) and the serial counter (+8).
const PUNDIT_LIST_NATIONAL: usize = 0x0;
const PUNDIT_LIST_LOCAL: usize = 0x4;
const PUNDIT_LIST_SERIAL: usize = 0x8;
/// The city record (0x38 bytes, name at +4) and the nation record name fields (+4 and +0x38).
const PUNDIT_CITY_SIZE: usize = 0x38;
const PUNDIT_PLACE_NAME: usize = 0x4;
const PUNDIT_PLACE_SHORT_NAME: usize = 0x38;

/// What a pundit line gives: 0 back (no line, or no number), 1 back without a record, or a record for the local (true)
/// or national list.
enum PunditLine {
    Fail,
    Skip,
    Add(bool, [u8; PUNDIT_RECORD_SIZE]),
}

/// Whether a place name field matches the token (non-empty and `strncmp` of 26 characters).
unsafe fn pundit_name_matches(field: usize, token: &[u8]) -> bool {
    read_u8(field) != 0 && exe::crt_strncmp(field, token.as_ptr() as usize, 0x1a) == 0
}

/// The parsing of one pundit line (`PunditList::read_pundit_line` after the read), with the serial the record gets.
unsafe fn pundit_parse_line(this: usize, line: usize, serial: u16) -> PunditLine {
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let copy: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_9466D0);
    let mut token = [0u8; 0x64];
    let mut id = 0i16;
    let mut name = [0u8; 0x64];
    let mut media_name = [0u8; 0x64];
    let mut place = [0u8; 0x64];
    let mut country = [0u8; 0x64];
    let mut p = pundit_list_parse_quoted_token(this, line, token.as_mut_ptr() as usize);
    if scan(token.as_ptr() as usize, 0xa6bc70, &mut id as *mut i16) == 0 {
        return PunditLine::Fail;
    }
    p = pundit_list_parse_quoted_token(this, p, token.as_mut_ptr() as usize);
    copy(name.as_mut_ptr() as usize, token.as_ptr() as usize, 0x64);
    name[0x63] = 0;
    p = pundit_list_parse_quoted_token(this, p, media_name.as_mut_ptr() as usize);
    let media = parse_media_type_name(this, media_name.as_ptr() as usize);
    let local = media == 2 || media == 4;
    let mut city = -1i32;
    if local {
        covered!("tel36_13 local pundit");
        p = pundit_list_parse_quoted_token(this, p, place.as_mut_ptr() as usize);
        if place[0] == 0 {
            return PunditLine::Skip;
        }
        let found = (0..read_i32(G_CITY_COUNT)).find(|&k| {
            pundit_name_matches(read_usize(G_CITY_TABLE) + k as usize * PUNDIT_CITY_SIZE + PUNDIT_PLACE_NAME, &place)
        });
        city = match found {
            Some(k) => k,
            None => {
                let club = (0..read_i32(G_CLUB_COUNT)).map(|k| read_usize(G_CLUB_TABLE) + k as usize * club::SIZE)
                    .find(|&c| pundit_name_matches(c + club::NAME, &place) || pundit_name_matches(c + club::SHORT_NAME, &place));
                let Some(club) = club else { return PunditLine::Skip };
                let stadium = read_usize(club + club::STADIUM);
                if stadium == 0 {
                    return PunditLine::Skip;
                }
                let town = read_usize(stadium + stadium::CITY);
                if town == 0 || read_i32(town) == -1 {
                    return PunditLine::Skip;
                }
                read_i32(town)
            }
        };
    }
    pundit_list_parse_quoted_token(this, p, country.as_mut_ptr() as usize);
    let nation = if country[0] == 0 {
        read_i32(KEY_NATION_ENGLAND)
    } else {
        (0..read_i32(G_NATION_COUNT)).find(|&k| {
            let n = read_usize(G_NATION_TABLE) + k as usize * nation::SIZE;
            pundit_name_matches(n + PUNDIT_PLACE_NAME, &country) || pundit_name_matches(n + PUNDIT_PLACE_SHORT_NAME, &country)
        }).unwrap_or(read_i32(KEY_NATION_ENGLAND))
    };
    let mut record = [0u8; PUNDIT_RECORD_SIZE];
    let r = record.as_mut_ptr() as usize;
    copy(r + PUNDIT_NAME, name.as_ptr() as usize, 0x64);
    write_u16(r, serial);
    write_u8(r + PUNDIT_NAME + 0x63, 0);
    write_u8(r + PUNDIT_MEDIA, media as u8);
    write_i32(r + PUNDIT_CITY, city);
    write_i32(r + PUNDIT_NATION, nation);
    write_u8(r + PUNDIT_ID, id as u8);
    PunditLine::Add(local, record)
}

/// Rule: each line of the pundit file gives a pundit: a number, a name, a media type and, for the local media (local
/// daily, local radio), a city or a club (its stadium's city); then a nation (England when empty or unknown). National
/// pundits go to one list, local ones to the other, each with a running serial; a local pundit whose place is unknown
/// is skipped.
///
/// `PunditList::read_pundit_line(file, line)`: `utils_read_config_line(file, line)`, 0 at the end. The quoted tokens
/// (`pundit_list_parse_quoted_token`, into 100-byte buffers): the number (`sscanf` "%hd"; 0 back when nothing is
/// read), the name (`strncpy` 100, the last byte zeroed), the media type (`parse_media_type_name`). Codes 2 and 4: the
/// place token (empty: 1 back); a city whose name (+4) matches (26 characters) gives its index, else a club whose name
/// or short name matches gives its stadium's city id (no club, no stadium, no city or city id -1: 1 back); other
/// codes: city -1. The nation token: empty, or no nation whose name (+4) or short name (+0x38) matches, gives
/// `KEY_NATION_ENGLAND`; otherwise the nation's index. The record (0xd4 bytes): the serial (`PUNDIT_LIST_SERIAL`, then
/// incremented), the name, the media code, the city, the nation and the number's low byte, added at the tail of the
/// local or the national list (`LLIST::add_tail`, 0xd4 bytes); 1 back. The buffer tests on the token addresses of the
/// original are always true.
/// Original: `PunditList::read_pundit_line` (0x7da6e0, printouts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn pundit_list_read_pundit_line(this: usize, file: usize, line: usize) -> u8 {
    if exe::utils_read_config_line(file, line as *mut u8) == 0 {
        return 0;
    }
    let serial = read_u16(this + PUNDIT_LIST_SERIAL);
    match pundit_parse_line(this, line, serial) {
        PunditLine::Fail => 0,
        PunditLine::Skip => 1,
        PunditLine::Add(local, record) => {
            covered!("tel36_13 pundit added");
            write_u16(this + PUNDIT_LIST_SERIAL, serial.wrapping_add(1));
            let list = read_usize(this + if local { PUNDIT_LIST_LOCAL } else { PUNDIT_LIST_NATIONAL });
            exe::llist_add_tail(list, record.as_ptr() as usize, PUNDIT_RECORD_SIZE as u32);
            1
        }
    }
}

/// Comparer for `printouts_static_init_date_b74360`.
pub(crate) unsafe extern "C" fn compare_printouts_static_init_date_b74360() {
    // simplification: the atexit registration does not run twice
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `printouts_static_destroy_date_b74360`.
pub(crate) unsafe extern "C" fn compare_printouts_static_destroy_date_b74360() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// Comparer for `printouts_static_init_date_b74358`.
pub(crate) unsafe extern "C" fn compare_printouts_static_init_date_b74358() {
    // simplification: the atexit registration does not run twice
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

/// Comparer for `printouts_static_destroy_date_b74358`.
pub(crate) unsafe extern "C" fn compare_printouts_static_destroy_date_b74358() {
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer for `pundit_list_read_pundit_line`: reading the file does not run twice, so the original runs and the
/// record it added (the tail of the list that grew) is checked against `pundit_parse_line` on the line it read; a line
/// it skipped must add nothing.
pub(crate) unsafe extern "thiscall" fn compare_pundit_list_read_pundit_line(this: usize, file: usize, line: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I4%%);
    let count = |k: usize| {
        let l = read_usize(this + k);
        if l == 0 { 0 } else { read_u16(l + llist::COUNT) }
    };
    let serial = read_u16(this + PUNDIT_LIST_SERIAL);
    let (national, local) = (count(PUNDIT_LIST_NATIONAL), count(PUNDIT_LIST_LOCAL));
    let r = o(this, file, line);
    if r == 0 {
        SCOREBOARD.record(count(PUNDIT_LIST_NATIONAL) == national && count(PUNDIT_LIST_LOCAL) == local,
            || "PunditList::read_pundit_line: 0 back but a record was added".to_string());
        return r;
    }
    let grew = (count(PUNDIT_LIST_NATIONAL) != national, count(PUNDIT_LIST_LOCAL) != local);
    let (same, what) = match pundit_parse_line(this, line, serial) {
        PunditLine::Fail => (false, "the copy reads no number".to_string()),
        PunditLine::Skip => (grew == (false, false) && read_u16(this + PUNDIT_LIST_SERIAL) == serial, "the copy skips the line".to_string()),
        PunditLine::Add(is_local, mine) => {
            let list = read_usize(this + if is_local { PUNDIT_LIST_LOCAL } else { PUNDIT_LIST_NATIONAL });
            let node = read_usize(list + llist::TAIL);
            let theirs = if node != 0 { read_usize(node + llist_node::DATA) } else { 0 };
            let equal = |a: usize, b: usize| theirs != 0 && (a..b).all(|k| read_u8(theirs + k) == mine[k]);
            (grew == (!is_local, is_local) && equal(0, 0x66) && equal(PUNDIT_MEDIA, PUNDIT_RECORD_SIZE),
                format!("the copy adds to the {} list: {:x?}", if is_local { "local" } else { "national" }, &mine[PUNDIT_MEDIA..]))
        }
    };
    SCOREBOARD.record(same, || format!("PunditList::read_pundit_line(line {:?}): {}", tel36_c_string(line, 0x80), what));
    r
}

/// Comparer for `parse_media_type_name`.
pub(crate) unsafe extern "thiscall" fn compare_parse_media_type_name(this: usize, name: usize) -> i8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(%%I5%%)(this, name);
    let n = parse_media_type_name(this, name);
    SCOREBOARD.record(r == n, || format!("parse_media_type_name({:?}): original={} rust={}", tel36_c_string(name, 0x64), r, n));
    r
}

/// Comparer for `pundit_list_parse_quoted_token`: the token written (100 bytes) and the address back, side by side.
pub(crate) unsafe extern "thiscall" fn compare_pundit_list_parse_quoted_token(this: usize, src: usize, out: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I6%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&[(out, 0x64)], || o(this, src, out), || pundit_list_parse_quoted_token(this, src, out));
    SCOREBOARD.record(same && r == n, || format!("PunditList::parse_quoted_token({:?}): original={:#x} rust={:#x} differences {:?}", tel36_c_string(src, 0x80), r, n, first_differences(&mo, &mr)));
    r
}

/// The C string at `a` (at most `n` bytes), for the messages.
pub(crate) unsafe fn tel36_c_string(a: usize, n: usize) -> String {
    let bytes: Vec<u8> = (0..n).map(|k| read_u8(a + k)).take_while(|&b| b != 0).collect();
    String::from_utf8_lossy(&bytes).into_owned()
}
