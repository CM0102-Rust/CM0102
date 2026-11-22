// ---------- batch %%LEVA%%: the pundit list's constructor and destructor and the obscene gesture ban (printouts.cpp) ----------

/// The pundit files by language (pundit.eng for English, 1, 11, 12 and any other value), the fallback pundit.cfg,
/// the folder key, the open modes, the media debug option and its file, and its header and footer lines.
const PUNDIT_FILES: [usize; 13] = [0xa793ec, 0xa793ec, 0xa7944c, 0xa79440, 0xa79434, 0xa79410, 0xa793f8, 0xa79458, 0xa7941c, 0xa79428,
    0xa79404, 0xa793ec, 0xa793ec];
const PUNDIT_FILE_ENGLISH: usize = 0xa793ec;
const PUNDIT_FILE_DEFAULT: usize = 0xa793e0;
const PUNDIT_DATA_FOLDER: usize = 0x9b5348;
const PUNDIT_READ: usize = 0x9899fc;
const PUNDIT_MEDIA_OPTION: usize = 0xa793d8;
const PUNDIT_COMMAND_LINE: usize = 0xae2c98;
const PUNDIT_MEDIA_OUT: usize = 0xa793c8;
const PUNDIT_MEDIA_OUT_MODE: usize = 0xa793d4;
const PUNDIT_MEDIA_START: usize = 0xa793ac;
const PUNDIT_MEDIA_END: usize = 0xa791c0;
/// The media.out lines: local with city and nation, with the city only, with neither; national with both, with the
/// nation only, with the city only, with neither (gender, type, serial, name, then the places).
const PUNDIT_LOCAL_BOTH: usize = 0xa7936c;
const PUNDIT_LOCAL_CITY: usize = 0xa7932c;
const PUNDIT_LOCAL_NONE: usize = 0xa792e8;
const PUNDIT_NATIONAL_BOTH: usize = 0xa792a8;
const PUNDIT_NATIONAL_NATION: usize = 0xa79264;
const PUNDIT_NATIONAL_CITY: usize = 0xa79220;
const PUNDIT_NATIONAL_NONE: usize = 0xa791dc;
/// The nation record size (`G_NATION_TABLE`).
const PUNDIT_NATION_SIZE: usize = 0x122;

/// One media.out line per pundit of a list: `line(record)` gives the format and its place names.
unsafe fn pundit_list_print(file: usize, list: usize, line: impl Fn(usize) -> (usize, Option<usize>, Option<usize>)) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let fprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_9464F0);
    list_rewind(list);
    let mut k = 0i32;
    while k < read_i16(list + llist::COUNT) as i32 {
        let e = exe::llist_get_next(list);
        if e == 0 {
            break;
        }
        let (gender, kind, serial) = (read_i8(e + PUNDIT_ID) as i32, read_i8(e + PUNDIT_MEDIA) as i32, read_i16(e) as i32);
        let (format, first, second) = line(e);
        match (first, second) {
            (Some(a), Some(b)) => sprintf(JOB_TEXT, format, gender, kind, serial, e + PUNDIT_NAME, a, b),
            (Some(a), None) => sprintf(JOB_TEXT, format, gender, kind, serial, e + PUNDIT_NAME, a),
            _ => sprintf(JOB_TEXT, format, gender, kind, serial, e + PUNDIT_NAME),
        };
        fprintf(file, JOB_TEXT);
        k += 1;
    }
}

/// The name of city `k` and of nation `k`.
unsafe fn pundit_city_name(k: i32) -> usize {
    read_usize(G_CITY_TABLE).wrapping_add((k as usize).wrapping_mul(PUNDIT_CITY_SIZE)) + PUNDIT_PLACE_NAME
}
unsafe fn pundit_nation_name(k: i32) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add((k as usize).wrapping_mul(PUNDIT_NATION_SIZE)) + PUNDIT_PLACE_NAME
}

/// Builds the pundit lists (national and local) from the pundit file of the game's language.
///
/// `PunditList::PunditList(this, language)`: the serial counter = 0; the national list and the local list are made
/// (operator new 0xe, `LLIST::LLIST`; a failed one stays null). Without both lists nothing more is done. The file:
/// `pundit.<lang>` for the language (2 fra, 3 ger, 4 ita, 5 esp, 6 dan, 7 hol, 8 nor, 9 por, 10 swe, else eng) in the
/// data folder (`utils_build_file_path("CM3_DATA", ..)`, copied to a local path); when it does not open, languages 0
/// and 1 try `pundit.cfg`, the others stop there. Each line goes to `PunditList::read_pundit_line` until it gives 0,
/// and the file is closed. With "-media" on the command line, media.out ("w+") gets the start line, one line per
/// local pundit (gender, type, serial, name, the city and the nation when set), one per national pundit (the same
/// four cases), the end line, and is closed. Returns `this`.
/// Original: `PunditList::PunditList` (0x7da190, printouts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn pundit_list_construct(this: usize, language: u32) -> usize {
    write_u16(this + PUNDIT_LIST_SERIAL, 0);
    let new_list = || {
        let l = exe::crt_operator_new(0xe);
        if l != 0 { exe::new_llist(l) } else { 0 }
    };
    write_usize(this + PUNDIT_LIST_NATIONAL, new_list());
    write_usize(this + PUNDIT_LIST_LOCAL, new_list());
    if read_usize(this + PUNDIT_LIST_NATIONAL) == 0 || read_usize(this + PUNDIT_LIST_LOCAL) == 0 {
        return this;
    }
    covered!("tel39_8 pundit list");
    let lang = language as u8 as i8;
    let name = if (0..=12).contains(&lang) { PUNDIT_FILES[lang as usize] } else { PUNDIT_FILE_ENGLISH };
    let open = |file: usize| {
        let mut path = [0u8; 0x1f4];
        let p = exe::utils_build_file_path(PUNDIT_DATA_FOLDER, file);
        let n = core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes_with_nul().len();
        core::ptr::copy_nonoverlapping(p as *const u8, path.as_mut_ptr(), n.min(path.len()));
        exe::crt_fopen(path.as_ptr() as usize, PUNDIT_READ)
    };
    let mut file = open(name);
    if file == 0 {
        // tel39_8: only English (0 and 1) falls back to pundit.cfg
        if lang != 0 && lang != 1 {
            return this;
        }
        file = open(PUNDIT_FILE_DEFAULT);
        if file == 0 {
            return this;
        }
    }
    let mut line = [0u8; 0x3e8];
    while pundit_list_read_pundit_line(this, file, line.as_mut_ptr() as usize) != 0 {}
    exe::crt_fclose(file);
    if exe::crt_strstr(PUNDIT_COMMAND_LINE, PUNDIT_MEDIA_OPTION) == 0 {
        return this;
    }
    let out = exe::crt_fopen(PUNDIT_MEDIA_OUT, PUNDIT_MEDIA_OUT_MODE);
    if out == 0 {
        return this;
    }
    covered!("tel39_8 media pundit list");
    let fprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_9464F0);
    fprintf(out, PUNDIT_MEDIA_START);
    pundit_list_print(out, read_usize(this + PUNDIT_LIST_LOCAL), |e| {
        let (city, nation) = (read_i32(e + PUNDIT_CITY), read_i32(e + PUNDIT_NATION));
        if city == -1 {
            (PUNDIT_LOCAL_NONE, None, None)
        } else if nation != -1 {
            (PUNDIT_LOCAL_BOTH, Some(pundit_city_name(city)), Some(pundit_nation_name(nation)))
        } else {
            (PUNDIT_LOCAL_CITY, Some(pundit_city_name(city)), None)
        }
    });
    pundit_list_print(out, read_usize(this + PUNDIT_LIST_NATIONAL), |e| {
        let (city, nation) = (read_i32(e + PUNDIT_CITY), read_i32(e + PUNDIT_NATION));
        match (city != -1, nation != -1) {
            (true, true) => (PUNDIT_NATIONAL_BOTH, Some(pundit_city_name(city)), Some(pundit_nation_name(nation))),
            (false, true) => (PUNDIT_NATIONAL_NATION, Some(pundit_nation_name(nation)), None),
            (true, false) => (PUNDIT_NATIONAL_CITY, Some(pundit_city_name(city)), None),
            (false, false) => (PUNDIT_NATIONAL_NONE, None, None),
        }
    });
    fprintf(out, PUNDIT_MEDIA_END);
    exe::crt_fclose(out);
    this
}

/// Destroys the pundit lists: each list (national, then local) is destroyed (`LLIST::~LLIST`), freed and its pointer
/// cleared.
/// Original: `PunditList::~PunditList` (0x7da690, printouts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn pundit_list_destruct(this: usize) {
    covered!("tel39_8 pundit list destroyed");
    for k in [PUNDIT_LIST_NATIONAL, PUNDIT_LIST_LOCAL] {
        let l = read_usize(this + k);
        if l != 0 {
            exe::drop_llist(l);
            exe::crt_operator_delete(l);
            // tel39_8: the pointer cleared
            write_usize(this + k, 0);
        }
    }
}

/// The match incident code of an obscene gesture (the caller, `MATCH_DAY`, only calls on it) and the ban it gives.
const OBSCENE_BAN_DELAY_DAYS: u32 = 14;
const OBSCENE_BAN_REASON: u8 = 5;
/// The competition object's ruling body byte (the first rule of the ban).
const COMP_OBJECT_RULING_BODY: usize = 0x50;
/// The 79-byte staff state record ([G_STAFF_STATE_TABLE]) and its flags byte with the bit the ban needs (the bit
/// docs/MARCA_DO_REGEN.md reads as the regen mark).
const STAFF_STATE_SIZE: usize = 0x4f;
const STAFF_STATE_FLAGS: usize = 0xb;
const STAFF_STATE_BAN_BIT: u8 = 0x40;
/// `Media::queue_obscene_gesture_story` (0x7570b0, media.cpp, thiscall, ret 0xc) [inference: the name, from the
/// caller].
const FN_MEDIA_OBSCENE_GESTURE_STORY: usize = 0x7570b0;

/// The obscene gesture ban: a one-match ban from two weeks on, the press story and the rival club's dislike.
///
/// `apply_obscene_gesture_ban(person, club, fixture)` (the match day calls it on incident 0xb): nothing unless the
/// person, his `staff::CLUB_JOB`, that club's dword +0x57, the club and the fixture exist, the fixture's
/// `fixture::COMP_ID` ≥ 0, its competition is not a friendly (`Friendly::is_friendly_competition`), the person's id is
/// below `G_STAFF_COUNT`, the competition object [G_COMP_OBJECT_TABLE][comp id] exists and the person's staff state
/// record has bit 0x40 of +0xb. The ban record (20 bytes): 1 match, 0 served, days −1, the start date = today + 14
/// days (`Date::add_days`), the first rule = the competition object's ruling body byte (+0x50), the other three 0xff,
/// the reason 5 with amount 1, the flags' bit 0 = `Discipline::is_player_banned(person, competition, 1, 0)` and bit 1
/// clear (the other bits are stack garbage in the original, zero here). Then the press story (0x7570b0 on [G_MEDIA]
/// with the person, the club and the fixture), `Discipline::add_ban(id, ban)` and the other club of the fixture
/// (the away club when the club is at home, else the home club) dislikes the person (`club_add_disliked_staff(.., 1)`).
/// Original: `apply_obscene_gesture_ban` (0x7daf50, printouts.cpp, cdecl).
pub(crate) unsafe extern "C" fn apply_obscene_gesture_ban(person: usize, club: usize, fixture: usize) {
    if person == 0 || club == 0 || fixture == 0 {
        return;
    }
    let own = read_usize(person + staff::CLUB_JOB);
    if own == 0 || read_usize(own + 0x57) == 0 {
        return;
    }
    let comp_id = read_i32(fixture + fixture::COMP_ID);
    if comp_id < 0 || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(fixture + fixture::COMPETITION)) != 0 {
        return;
    }
    let id = read_i32(person);
    if id >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let comp_object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + comp_id as usize * 4);
    if comp_object == 0 {
        return;
    }
    let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(STAFF_STATE_SIZE));
    if read_u8(state + STAFF_STATE_FLAGS) & STAFF_STATE_BAN_BIT == 0 {
        return;
    }
    covered!("tel39_8 obscene gesture ban");
    let mut ban = [0u8; 0x14];
    let b = ban.as_mut_ptr() as usize;
    write_u8(b + ban_record::MATCHES, 1);
    write_u8(b + ban_record::MATCHES_SERVED, 0);
    write_u16(b + ban_record::DAYS, 0xffff);
    let mut start = [0u8; 8];
    // tel39_8: the ban starts two weeks on
    exe::date_add_days(G_CURRENT_DATE, start.as_mut_ptr() as usize, OBSCENE_BAN_DELAY_DAYS);
    copy_record_date(b + ban_record::START_DATE, start.as_ptr() as usize);
    write_u8(b + ban_record::REASON, OBSCENE_BAN_REASON);
    write_u16(b + ban_record::REASON_AMOUNT, 1);
    write_u8(b + ban_record::RULES, read_u8(comp_object + COMP_OBJECT_RULING_BODY));
    for k in 1..4 {
        write_u8(b + ban_record::RULES + k, 0xff);
    }
    let banned = exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), person, read_usize(fixture + fixture::COMPETITION), 1, 0) != 0;
    write_u8(b + ban_record::FLAGS, banned as u8);
    let story: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_MEDIA_OBSCENE_GESTURE_STORY);
    story(read_usize(G_MEDIA), person, club, fixture);
    add_ban(read_usize(G_DISCIPLINE), id as u32, b);
    let home = read_usize(fixture + fixture::HOME_CLUB);
    let rival = if home == club { read_usize(fixture + fixture::AWAY_CLUB) } else { home };
    club_add_disliked_staff(rival, person, 1);
}

/// The contents of a pundit list (the bytes the original writes: 0..0x66 and 0xca..0xd4 of each record).
unsafe fn pundit_list_contents(list: usize) -> Vec<Vec<u8>> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    list_rewind(list);
    loop {
        let e = exe::llist_get_next(list);
        if e == 0 {
            break;
        }
        let r = core::slice::from_raw_parts(e as *const u8, PUNDIT_RECORD_SIZE);
        v.push([&r[..0x66], &r[PUNDIT_MEDIA..]].concat());
    }
    v
}

/// Comparer for `pundit_list_construct`: the copy builds a local pundit list first (reading the same file), then the
/// original builds the real one; both lists and the serial counter must match. The local lists are freed.
pub(crate) unsafe extern "thiscall" fn compare_pundit_list_construct(this: usize, language: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_PUNDIT_LIST_CONSTRUCT);
    let mut local = [0usize; 3];
    let lp = local.as_mut_ptr() as usize;
    pundit_list_construct(lp, language);
    let r = o(this, language);
    let side = |t: usize| (pundit_list_contents(read_usize(t + PUNDIT_LIST_NATIONAL)), pundit_list_contents(read_usize(t + PUNDIT_LIST_LOCAL)),
        read_u16(t + PUNDIT_LIST_SERIAL));
    let (mo, mr) = (side(this), side(lp));
    pundit_list_destruct(lp);
    SCOREBOARD.record(r == this && mo == mr, || format!("PunditList::PunditList(language {}): original {} national, {} local, serial {}; rust {}, {}, {}",
        language as u8, mo.0.len(), mo.1.len(), mo.2, mr.0.len(), mr.1.len(), mr.2));
    r
}

/// Comparer for `pundit_list_destruct`: only the original runs (freeing does not run twice); then both pointers must
/// be null.
pub(crate) unsafe extern "thiscall" fn compare_pundit_list_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PUNDIT_LIST_DESTRUCT)(this);
    let (a, b) = (read_usize(this + PUNDIT_LIST_NATIONAL), read_usize(this + PUNDIT_LIST_LOCAL));
    SCOREBOARD.record(a == 0 && b == 0, || format!("PunditList::~PunditList: lists left {:#x} {:#x}", a, b));
}

/// Comparer for `apply_obscene_gesture_ban`: only the original runs (the ban, the press story and the dislike do not
/// run twice; `add_ban` and `club_add_disliked_staff` compare themselves); proven in Rust mode by the career.
pub(crate) unsafe extern "C" fn compare_apply_obscene_gesture_ban(person: usize, club: usize, fixture: usize) {
    original_fn::<unsafe extern "C" fn(usize, usize, usize)>(IDX_APPLY_OBSCENE_GESTURE_BAN)(person, club, fixture)
}
