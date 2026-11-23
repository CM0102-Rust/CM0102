use super::*;

/// Picks a random pundit: type 1 on Sunday becomes 7; local types (2, 4): the club's pundits; the others: of the type
/// and nation (−1 any); up to 20, random(n) and the id, or −1.
///
/// Ret 0xc; 0x1094-byte frame through 0x945610: `PunditList::pick_random_pundit(club, kind, nation)`, from the press
/// stories (`exe::media_create_player_media_story`, 0x733a60) and from the news texts. Type 1 on Sunday
/// (`exe::date_get_weekday` = 6) becomes 7. Types 2 and 4 (local): those of list `[this+4]` whose `pundit::CITY_ID` is
/// the id of the city of the club's `.stadium()` (`.city()`) (none: −1); the others: those of list `[this]` of the type
/// (`pundit::MEDIA_TYPE`) and the nation (`pundit::NATION_ID`; −1 accepts any). Up to 20; none: −1; otherwise the id
/// (word +0, signed) of one of them by random(n).
/// Original: `PunditList::pick_random_pundit` (0x7dab90, printouts.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn pick_random_pundit(this: usize, club: usize, kind: u32, nation: i32) -> i32 {
    let club = Club(club);
    let mut t = kind as i8;
    if t == 1 {
        if exe::date_get_weekday(G_CURRENT_DATE) as i16 == 6 {
            t = 7;
        }
    }
    let mut ids = [0i16; 20];
    let mut n = 0usize;
    if t == 2 || t == 4 {
        let est = club.stadium();
        if est.is_null() || est.city() == 0 {
            return -1;
        }
        let l = read_usize(this + 4);
        list_rewind(l);
        if read_i16_as_i32(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = contract_manager_list_next(l);
            if p == 0 {
                break;
            }
            if read_u32(p + pundit::CITY_ID) == read_u32(club.stadium().city()) { // cpz10: o comentarista local
                covered!("cpz_10 local commentator");
                ids[n] = read_i16_as_i32(p) as i16;
                n += 1;
                if n >= 20 {
                    break;
                }
            }
            i += 1;
            if i >= read_i16_as_i32(read_usize(this + 4)) {
                break;
            }
        }
    } else {
        let l = read_usize(this);
        list_rewind(l);
        if read_i16_as_i32(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = contract_manager_list_next(l);
            if p == 0 {
                break;
            }
            if read_i8(p + pundit::MEDIA_TYPE) == t {
                let accepted = if nation == -1 {
                    covered!("cpz_10 commentator of any nation");
                    true // cpz10: any nation
                } else {
                    read_u32(p + pundit::NATION_ID) as i32 == nation
                };
                if accepted {
                    ids[n] = read_i16_as_i32(p) as i16;
                    n += 1;
                    if n >= 20 {
                        break;
                    }
                }
            }
            i += 1;
            if i >= read_i16_as_i32(read_usize(this)) {
                break;
            }
        }
    }
    if n == 0 {
        return -1;
    }
    covered!("cpz_10 commentator draw");
    let r = exe::utils_random(n as i32); // cpz10: the commentator random draw
    ids[r as usize] as i32
}

/// Default media outlet name by type: 2 local paper, 3 radio, 4 local radio, 5 magazine, else national paper.
pub(crate) fn default_media_name(kind: i8) -> usize {
    match kind {
        2 => 0xa24284,
        3 => 0xa794cc,
        4 => 0xa241f4,
        5 => 0xa794b8,
        _ => 0xa242cc,
    }
}

/// Returns the name (+2) and flag (`pundit::GENDER`) of the pundit with the id, or the default outlet type name and
/// flag 0.
///
/// Ret 0x10: `PunditList::get_pundit_name(kind, id, name, mark)`, from the news texts. Without `name`: flag 0. Id −1:
/// the type's default name and flag 0. Otherwise the pundit with the id (word +0) in list `[this+4]` (types 2 and 4) or
/// `[this]`: the name (+2, sprintf "%s" `FN_CRT_SPRINTF`) and the flag (`pundit::GENDER`); without him, the default
/// name and flag 0.
/// Original: `PunditList::get_pundit_name` (0x7dad20, printouts.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn get_pundit_name(this: usize, kind: u32, id: u32, name: usize, mark: *mut u8) {
    if name == 0 {
        *mark = 0;
        return;
    }
    let sprintf: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let t = kind as i8;
    let id = id as i16;
    if id != -1 {
        let l = read_usize(this + if t == 2 || t == 4 { 4 } else { 0 });
        list_rewind(l);
        if read_i16_as_i32(l) > 0 {
            let mut i = 0;
            loop {
                let p = contract_manager_list_next(l);
                if p == 0 {
                    break;
                }
                if read_i16_as_i32(p) as i16 == id {
                    sprintf(name, 0x991944, p + 2);
                    *mark = read_u8(p + pundit::GENDER);
                    return;
                }
                i += 1;
                if i >= read_i16_as_i32(l) {
                    break;
                }
            }
        }
    }
    sprintf(name, 0x991944, default_media_name(t));
    *mark = 0;
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy carries the branch. Without news both do not
// fazem nada.

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
    let fixture = Fixture(fixture);
    let person = Staff(person);
    if person.is_null() || club == 0 || fixture.is_null() {
        return;
    }
    let own = person.club_job().addr();
    if own == 0 || read_usize(own + 0x57) == 0 {
        return;
    }
    let comp_id = fixture.comp_id();
    if comp_id < 0 || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), fixture.competition().addr()) != 0 {
        return;
    }
    let id = read_i32(person.addr());
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
    let banned = exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), person, fixture.competition().addr(), 1, 0) != 0;
    write_u8(b + ban_record::FLAGS, banned as u8);
    let story: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_MEDIA_OBSCENE_GESTURE_STORY);
    story(read_usize(G_MEDIA), person.addr(), club, fixture.addr());
    add_ban(read_usize(G_DISCIPLINE), id as u32, b);
    let home = fixture.home_club().addr();
    let rival = if home == club { fixture.away_club().addr() } else { home };
    club_add_disliked_staff(rival, person.addr(), 1);
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

/// Rule: a text is padded with spaces up to a width, or cut at it.
///
/// No text, or a width below 1: the printouts.cpp 1600 error box and 0. A text shorter than the width gets spaces
/// (each followed by the terminator) up to it; a longer or equal one is cut at the width (the terminator written
/// there). Returns the text.
/// Original: `printouts_pad_string` (0x7da070, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_pad_string(text: usize, width: u32) -> usize {
    if text == 0 || width < 1 {
        tel26_error_box(PRINTOUTS_CPP, 0x640);
        return 0;
    }
    let width = width as usize;
    let mut n = printout_text_length(text);
    if n >= width {
        write_u8(text + width, 0);
        return text;
    }
    covered!("tel25_3 pad a printout text");
    while n < width {
        write_u8(text + n, b' ');
        n += 1;
        write_u8(text + n, 0);
    }
    text
}

/// Comparer for `printouts_pad_string`: side by side over the text (up to the longer of its length and the width, and
/// the terminator) and the return; the error box (no text or no width) runs the original only.
pub(crate) unsafe extern "C" fn compare_printouts_pad_string(text: usize, width: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_PRINTOUTS_PAD_STRING);
    if text == 0 || width < 1 {
        return o(text, width);
    }
    let regions = [(text, printout_text_length(text).max(width as usize) + 1)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(text, width), || printouts_pad_string(text, width));
    SCOREBOARD.record(equal && ro == rr, || format!("printouts_pad_string(width {}): return {:#x} x {:#x}, differences in {:?}", width, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: the club transfers printout lists its transfers by date, earliest first.
///
/// `a` and `b` are two past transfer records (qsort over the records themselves). No first: 1; no second: −1; then
/// the dates at +0x1d (8 bytes): the first earlier than the second (`Date::operator<` 0x456770): −1; later
/// (`Date::operator>` 0x456740): 1; the same day: 0.
/// Original: `printouts_compare_by_date` (0x7da130, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_compare_by_date(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    if exe::date_operator_3c(a + 0x1d, read_u32(b + 0x1d), read_u32(b + 0x21)) != 0 {
        covered!("tel25_3 transfer earlier");
        return -1;
    }
    (exe::date_operator_3e(a + 0x1d, read_u32(b + 0x1d), read_u32(b + 0x21)) != 0) as i32
}

/// Rule: a competition's printout: its tables and statistics written to a file (by default "~printout.txt"), then
/// sent to the printer when asked; the temporary file is removed.
///
/// `print_comp_report(comp, kind, file, print)`. No competition: the printouts.cpp 1000 box and 0. The period is set
/// first: [`PRINTOUT_PERIOD_START`] from `printout_comp_period_start` and [`PRINTOUT_PERIOD_END`] a year later (both
/// through `Date::Date` and `Date::operator=`). The file opens for writing ("w"; failing: the 1025 box and 0). The
/// heading " <%s - comp> - {}<%s - date>" (langlib_sprintf with 0x64, the competition's name +4 and today's long date)
/// goes into [`PRINTOUT_LINE`]; the file gets a row of asterisks, a newline, the heading, a newline, the asterisks and
/// four newlines. Then by the bits of `kind`: 1 the league tables (`printouts_write_league_tables` 0x7d8120), 2, 4, 8
/// and 0x10 the statistics tables 2, 5, 0x11 and 0xc (`printouts_write_comp_stat_table` 0x7d9720); any of them
/// returning 0 makes the result 0 (otherwise 1). The file is closed. With `print`: it opens again for reading ("r";
/// failing: the 1094 box and 0), a print job of 0x8c starts and each line read (0xc8 bytes, its final newline cut)
/// becomes a page break when it is a single space, otherwise a printed line; then the job is printed and ended and the
/// file closed. Without a file given, "~printout.txt" is removed. Returns the result.
/// Original: `printouts_print_comp_report` (0x7d7c80, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_print_comp_report(comp: usize, kind: u32, file: usize, print: u32) -> u32 {
    if comp == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x3e8);
        return 0;
    }
    let (day, year) = printout_comp_period_start(comp);
    let (mut date, mut out) = ([0u8; 8], [0u8; 8]);
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    exe::date_operator_3d(PRINTOUT_PERIOD_START, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    exe::new_date(date.as_mut_ptr() as usize, read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2).wrapping_add(1) as u32);
    exe::date_operator_3d(PRINTOUT_PERIOD_END, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    let name = if file != 0 { file } else { PRINTOUT_TEMP_FILE };
    let f = exe::crt_fopen(name, 0x99c370usize);
    if f == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x401);
        return 0;
    }
    covered!("tel25_3 competition printout");
    let mut today = [0u8; 0xc8];
    exe::date_get_long_date_string(G_CURRENT_DATE, today.as_mut_ptr() as usize, 0xc8, 0xff);
    comp_lang_format()(PRINTOUT_LINE, 0xa78ef0, 0x64u32, comp + 4, today.as_ptr());
    for text in [PRINTOUT_STARS, PRINTOUT_NEWLINE, PRINTOUT_LINE, PRINTOUT_NEWLINE, PRINTOUT_STARS, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE] {
        printout_put(text, f);
    }
    let mut result = 1;
    if kind & 1 != 0 {
        let tables: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_PRINTOUTS_WRITE_LEAGUE_TABLES);
        if tables(comp, f) == 0 {
            result = 0;
        }
    }
    for (bit, table) in [(2u32, 2u32), (4, 5), (8, 0x11), (0x10, 0xc)] {
        if kind & bit != 0 && exe::printouts_write_comp_stat_table(comp, table, f) == 0 {
            result = 0;
        }
    }
    exe::crt_fclose(f);
    if print != 0 {
        let f = exe::crt_fopen(name, 0x9899fcusize);
        if f == 0 {
            tel26_error_box(PRINTOUTS_CPP, 0x446);
            return 0;
        }
        covered!("tel25_3 print the competition printout");
        exe::screenmanager_begin_print_job(G_SCREEN_MANAGER, 0x8c);
        while printout_get_line(PRINTOUT_LINE, 0xc8, f) != 0 {
            let n = printout_text_length(PRINTOUT_LINE);
            if n != 0 && read_u8(PRINTOUT_LINE + n - 1) == b'\n' {
                write_u8(PRINTOUT_LINE + n - 1, 0);
            }
            if read_u8(PRINTOUT_LINE) == b' ' && read_u8(PRINTOUT_LINE + 1) == 0 {
                exe::screenmanager_add_print_page_break(G_SCREEN_MANAGER);
            } else {
                exe::screenmanager_add_print_line(G_SCREEN_MANAGER, PRINTOUT_LINE);
            }
        }
        exe::screenmanager_print_job(G_SCREEN_MANAGER);
        exe::screenmanager_end_print_job(G_SCREEN_MANAGER);
        exe::crt_fclose(f);
    }
    if file == 0 {
        exe::crt_remove(PRINTOUT_TEMP_FILE as *const u8);
    }
    result
}

/// Comparer for `printouts_print_comp_report`: only the original runs (it writes a file and may print), and the period
/// it leaves ([`PRINTOUT_PERIOD_START`], [`PRINTOUT_PERIOD_END`]) is checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_printouts_print_comp_report(comp: usize, kind: u32, file: usize, print: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, u32) -> u32>(IDX_PRINTOUTS_PRINT_COMP_REPORT);
    if comp == 0 {
        return o(comp, kind, file, print);
    }
    let (day, year) = printout_comp_period_start(comp);
    let r = o(comp, kind, file, print);
    let got = (read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2) as u32,
               read_u16(PRINTOUT_PERIOD_END) as u32, read_u16(PRINTOUT_PERIOD_END + 2) as u32);
    let plan = (day, year, day, (year + 1) & 0xffff);
    SCOREBOARD.record(got == plan, || format!("competition printout ({:#x}, kind {:#x}): period {:?} x plan {:?}", comp, kind, got, plan));
    r
}

/// Rule: a competition's statistics table, the leading 30 players of one statistic.
///
/// `write_comp_stat_table(comp, table, file)`. The competition object is the entry of [`G_COMP_OBJECT_TABLE`] for
/// the competition's id (none: the printouts.cpp 1462 box and 0) and its statistics object is at `+8` (none: the 1470
/// box and 0). The heading " <%s - season> <%s - stat>" gets the competition's season from the printout's start year
/// (0x6697c0) and the statistic's label (`get_player_stat_label`, into the money text buffer); then the column titles
/// with the statistic's short name (2 "Gls", 5 "Asts", 0x11 "Av R", 0xc "MoM", any other "???"), the "Nation" column
/// when the competition object's kind (`+0x42`) is 4, else "Club", and the dashes. The rows come from
/// `get_sorted_player_table(table, &count, 30, all positions)`: none ("count" 0) prints "No stats available"; each
/// row whose staff id (`+4`) is not −1 prints its position (`utils_format_ordinal(k + 1, −1)`, 4 wide), the staff's
/// name (style 1, 25 wide), the nation's or the club's short name (none: empty; 25 wide), the appearances (statistic
/// 1 with the substitutes split, 7 wide) and the statistic itself (5 wide). The table is freed and the section ends
/// (two newlines and a lone space). Returns 1.
/// Original: `printouts_write_comp_stat_table` (0x7d9720, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_comp_stat_table(comp: usize, table: u32, file: usize) -> u32 {
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if object == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x5b6);
        return 0;
    }
    let stats = read_usize(object + 8);
    if stats == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x5be);
        return 0;
    }
    covered!("tel25_7 competition statistics table");
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_comp_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, comp, t, 0xc8);
    printout_stat_label(stats, table, PRINTOUT_SCRATCH_TEXT, 0x7d0);
    comp_lang_format()(PRINTOUT_LINE, 0xa791a4, t, PRINTOUT_SCRATCH_TEXT);
    printout_write_heading(file);
    match table as u8 {
        2 => { comp_lang_format()(t, 0xa78490); }
        5 => { comp_lang_format()(t, 0xa7845c); }
        0x11 => { comp_lang_format()(t, 0xa78434); }
        0xc => { comp_lang_format()(t, 0xa7843c); }
        _ => printout_copy(t, 0xa14f84),
    }
    let national = read_u8(object + 0x42) == PRINTOUT_NATIONAL_COMP;
    printout_write_titles(file, if national { 0xa79150 } else { 0xa790fc }, t, 0xa790b0);
    let mut count = 0i32;
    let rows = printout_sorted_stat_table(stats, table, &mut count, 0x1e, 0xffff_ffff);
    if count == 0 {
        comp_lang_format()(PRINTOUT_LINE, 0xa7909c);
        printout_write_row(file);
    } else {
        for k in 0..count.max(0) {
            let row = rows + k as usize * 0x2d;
            let id = read_i32(row + 4);
            if id == -1 {
                continue;
            }
            let person = Staff(read_usize(G_STAFF_TABLE) + id as isize as usize * staff::SIZE);
            write_u8(PRINTOUT_LINE, 0);
            printout_copy(t, exe::utils_format_ordinal(k + 1, -1));
            printout_column(t, 4);
            exe::database_format_staff_name(person, t, 0xc8, 1);
            printout_column(t, 0x19);
            let (owner, name) = if national { (person.nation().addr(), nation::SHORT_NAME) } else { (person.club_job().addr(), club::SHORT_NAME) };
            if owner != 0 {
                printout_copy(t, owner + name);
            } else {
                write_u8(t, 0);
            }
            printout_column(t, 0x19);
            printout_format_stat(stats, row, 1, t, 0xc8, 1);
            printout_column(t, 7);
            printout_format_stat(stats, row, table, t, 0xc8, 1);
            printout_column(t, 5);
            printout_write_row(file);
        }
    }
    if rows != 0 {
        exe::crt_free(rows);
    }
    printout_end_section(file);
    1
}

/// Comparer for `printouts_write_comp_stat_table`: side by side over the text written (`compare_printout_text`);
/// without the competition object or its statistics (the error boxes) only the original runs.
pub(crate) unsafe extern "C" fn compare_printouts_write_comp_stat_table(comp: usize, table: u32, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(IDX_PRINTOUTS_WRITE_COMP_STAT_TABLE);
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if object == 0 || read_usize(object + 8) == 0 {
        return o(comp, table, file);
    }
    compare_printout_text(file, || format!("competition statistics printout ({:#x}, table {:#x})", comp, table as u8),
                          |f| o(comp, table, f), |f| printouts_write_comp_stat_table(comp, table, f))
}

/// Rule: a competition's league tables, one per league stage, sorted first.
///
/// `write_league_tables(comp, file)`. The competition object is the entry of [`G_COMP_OBJECT_TABLE`] for the
/// competition's id (none: the printouts.cpp 1155 box and 0). Its stages −1 (the object itself) to `+0x30`
/// (`Comp::get_stage`) that are leagues (`+0x43` 2) are counted: more than one makes the heading " <%s - season>
/// Tables", otherwise "... Table" (the competition's season of the printout's start year). For each league stage:
/// with more than one, its name (`comp_util_get_stage_name` of `+0x3a`, or "Table" without one) and two newlines; the
/// column titles by the stage's flags (`+0xab`: 0x20 shoot-out wins "SOW", 1 penalty wins "Pen", else draws "Drn") and
/// the dashes. A stage whose draw date (day `+0xb6`, year `+0xb8` + `+0x40`; `+0xb6` 0xffff: none) is still after
/// today prints "Draw not yet made"; otherwise `+0xad` is cleared, the table sorted (`League::sort_table(stage, 0)`)
/// and each of its `+0x3e` rows (`League::get_table_row`) printed, with a dashes line before the rows where the
/// places change (`printout_table_rule_before`): the position (`utils_format_ordinal(+4 + 1, −1)`, 4 wide, one space),
/// the fate mark (2 wide, one space), the club's short name (25 wide), its nation's three letters when the object's
/// kind (`+0x42`) is 2 (3 wide), and the numbers of `PRINTOUT_TABLE_COLUMNS` (3 wide each). Each stage printed ends
/// with two newlines; the printout ends with a lone space. Returns 1.
/// Original: `printouts_write_league_tables` (0x7d8120, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_league_tables(comp: usize, file: usize) -> u32 {
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if object == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x483);
        return 0;
    }
    let mut leagues = 0i8;
    if read_i32(object + 0x30) >= -1 {
        let mut s = -1i8;
        loop {
            let stage = get_stage(object, s as i32);
            if stage != 0 && read_u8(stage + 0x43) == PRINTOUT_LEAGUE_STAGE {
                leagues = leagues.wrapping_add(1);
            }
            s = s.wrapping_add(1);
            if s as i32 > read_i32(object + 0x30) {
                break;
            }
        }
    }
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_comp_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, comp, t, 0xc8);
    comp_lang_format()(PRINTOUT_LINE, if leagues > 1 { 0xa79084 } else { 0xa7906c }, t);
    printout_write_heading(file);
    if read_i32(object + 0x30) >= -1 {
        let mut s = -1i8;
        loop {
            let stage = get_stage(object, s as i32);
            if stage != 0 && read_u8(stage + 0x43) == PRINTOUT_LEAGUE_STAGE {
                covered!("tel25_7 league table");
                if leagues > 1 {
                    let name = read_u16(stage + 0x3a);
                    if name != 0 {
                        exe::comp_util_get_stage_name(PRINTOUT_LINE as *mut u8, 0xc8, name as u32, 0);
                    } else {
                        comp_lang_format()(PRINTOUT_LINE, 0x99c144);
                    }
                    printout_write_all(file, &[PRINTOUT_LINE, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE]);
                }
                let flags = read_u16(stage + 0xab);
                let titles = if flags & 0x20 != 0 { 0xa79008 } else if flags & 1 != 0 { 0xa78fa4 } else { 0xa78f40 };
                printout_write_titles(file, titles, 0, PRINTOUT_WIDE_DASHES);
                let draw_day = read_u16(stage + 0xb6);
                let mut not_drawn = false;
                if draw_day != 0xffff {
                    let mut date = [0u8; 8];
                    let year = read_u16(stage + 0xb8).wrapping_add(read_u16(stage + 0x40));
                    let d = exe::new_date(date.as_mut_ptr() as usize, draw_day as u32, year as u32);
                    let (today_day, today_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
                    not_drawn = today_year < read_u16(d + 2) || (today_year == read_u16(d + 2) && today_day < read_i16(d));
                }
                if not_drawn {
                    covered!("tel25_7 draw not yet made");
                    comp_lang_format()(PRINTOUT_LINE, 0xa78f2c);
                    printout_write_row(file);
                } else {
                    write_u32(stage + 0xad, 0);
                    sort_table(stage, 0);
                    let mut k = 0i8;
                    while (k as i16) < read_i16(stage + 0x3e) {
                        if printout_table_rule_before(stage, k, read_i16(stage + 0x3e)) {
                            printout_write_all(file, &[PRINTOUT_WIDE_DASHES, PRINTOUT_NEWLINE]);
                        }
                        let row = league_record(stage, k as i32);
                        let team = Club(read_usize(row));
                        write_u8(PRINTOUT_LINE, 0);
                        printout_copy(t, exe::utils_format_ordinal(read_i8(row + 4) as i32 + 1, -1));
                        printouts_pad_string(t, 4);
                        printout_append(PRINTOUT_LINE, t);
                        printout_append(PRINTOUT_LINE, PRINTOUT_PAGE_BREAK);
                        printout_table_mark(read_u8(row + 0x37), t);
                        printouts_pad_string(t, 2);
                        printout_append(PRINTOUT_LINE, t);
                        printout_append(PRINTOUT_LINE, PRINTOUT_PAGE_BREAK);
                        printout_copy(t, team.addr() + club::SHORT_NAME);
                        printout_column(t, 0x19);
                        let nation = team.nation();
                        if read_u8(object + 0x42) == PRINTOUT_CLUBS_OF_NATIONS_COMP && !nation.is_null() {
                            printout_copy(t, nation.addr() + nation::THREE_LETTER_NAME);
                        } else {
                            write_u8(t, 0);
                        }
                        printout_column(t, 3);
                        for (at, word) in PRINTOUT_TABLE_COLUMNS {
                            let n = if word { read_i16(row + at) as i32 } else { read_i8(row + at) as i32 };
                            comp_crt_format()(t, PRINTOUT_FORMAT_NUMBER, n);
                            printout_column(t, 3);
                        }
                        printout_write_row(file);
                        k = k.wrapping_add(1);
                    }
                }
                printout_write(file, PRINTOUT_NEWLINE);
                printout_write(file, PRINTOUT_NEWLINE);
            }
            s = s.wrapping_add(1);
            if s as i32 > read_i32(object + 0x30) {
                break;
            }
        }
    }
    printout_fprintf()(file, PRINTOUT_FORMAT_LINE, PRINTOUT_PAGE_BREAK);
    1
}

/// Comparer for `printouts_write_league_tables`: side by side over the text written (`compare_printout_text`); the
/// original sorts the tables first and the copy sorts them again, finding them in order. Without the competition
/// object (the error box) only the original runs.
pub(crate) unsafe extern "C" fn compare_printouts_write_league_tables(comp: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_PRINTOUTS_WRITE_LEAGUE_TABLES);
    if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4) == 0 {
        return o(comp, file);
    }
    compare_printout_text(file, || format!("league tables printout ({:#x})", comp), |f| o(comp, f), |f| printouts_write_league_tables(comp, f))
}

/// Rule: a club's transfers in the printout's period, the ones it bought and then the ones it sold, by date.
///
/// `write_club_transfers(club, reserves, file)`; a national team writes nothing and returns 0. The transfers of the
/// period (`get_past_transfers` from the months and years of [`PRINTOUT_PERIOD_START`] and [`PRINTOUT_PERIOD_END`])
/// that neither bought nor sold by the club are dropped (the last record moved into the place, the place looked at
/// again) and the rest sorted by date (`qsort` with `printouts_compare_by_date`). The heading " <%s - season>
/// Transfers" takes the club's season of the start year. Then two passes: the players in ("Date Player In From Fee",
/// the club the buyer) and out ("... Player Out To ...", the club the seller); each row: the short date (8 wide), the
/// name (style 1, 25 wide), the other club's short name, or "Free Transfer" in / "Released" out without one (25 wide)
/// and the fee: with a fee, the money (flags 5), "<money> p/x" for an exchange; without one, "Exch", "Loan", "Bos" or
/// "Free" (8 wide). A pass without rows prints "None". Each pass ends with a newline; the array is freed and the
/// section ends. Returns 1.
/// Original: `printouts_write_club_transfers` (0x7d60c0, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_club_transfers(club: usize, _reserves: u32, file: usize) -> u32 {
    if exe::database_is_national_team(club) != 0 {
        return 0;
    }
    let (mut list, mut count) = (0usize, 0i32);
    let end_month = exe::date_get_month(PRINTOUT_PERIOD_END) as u32;
    let start_month = exe::date_get_month(PRINTOUT_PERIOD_START) as u32;
    printout_past_transfers(start_month, read_u16(PRINTOUT_PERIOD_START + 2) as u32, end_month, read_u16(PRINTOUT_PERIOD_END + 2) as u32, &mut list, &mut count);
    let id = read_u32(club);
    let mut k = 0i32;
    while k < count {
        let r = list + k as usize * PAST_TRANSFER_SIZE;
        if read_u32(r + past_transfer::TO_CLUB_ID) != id && read_u32(r + past_transfer::FROM_CLUB_ID) != id {
            count -= 1;
            let last = list + count as usize * PAST_TRANSFER_SIZE;
            core::ptr::copy(last as *const u8, r as *mut u8, past_transfer::DATE);
            let mut out = [0u8; 8];
            exe::date_operator_3d(r + past_transfer::DATE, out.as_mut_ptr() as usize, last + past_transfer::DATE);
            k -= 1;
        }
        k += 1;
    }
    if count != 0 {
        exe::crt_qsort(list, count as u32, PAST_TRANSFER_SIZE as u32, 0x7da130usize);
    }
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_club_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, club, t, 0xc8);
    comp_lang_format()(PRINTOUT_LINE, 0xa78c90, t);
    printout_write_heading(file);
    for pass in 1..=2u8 {
        printout_write_titles(file, if pass == 1 { 0xa78c4c } else { 0xa78c08 }, 0, PRINTOUT_TRANSFER_DASHES);
        let mut any = false;
        for k in 0..count.max(0) {
            let r = list + k as usize * PAST_TRANSFER_SIZE;
            let ours = if pass == 1 { read_u32(r + past_transfer::TO_CLUB_ID) } else { read_u32(r + past_transfer::FROM_CLUB_ID) };
            if ours != id {
                continue;
            }
            write_u8(PRINTOUT_LINE, 0);
            printout_short_date(r + past_transfer::DATE, t, 0xc8, 0xff);
            printout_column(t, 8);
            exe::database_format_name_from_ids(read_u32(r + past_transfer::FIRST_NAME_ID), read_u32(r + past_transfer::SECOND_NAME_ID), read_u32(r + past_transfer::COMMON_NAME_ID), t as *mut u8, 0xc8, 1, read_u32(r));
            printout_column(t, 0x19);
            let other = if pass == 1 { read_i32(r + past_transfer::FROM_CLUB_ID) } else { read_i32(r + past_transfer::TO_CLUB_ID) };
            if other > -1 {
                printout_copy(t, read_usize(G_CLUB_TABLE) + other as usize * club::SIZE + club::SHORT_NAME);
            } else if pass == 1 {
                comp_lang_format()(t, 0x98e604);
            } else {
                comp_lang_format()(t, 0xa78b88, PRINTOUT_NO_TEXT);
            }
            printout_column(t, 0x19);
            let fee = read_u32(r + past_transfer::FEE);
            let kind = read_u8(r + past_transfer::MOVE_TYPE);
            if fee != 0 {
                let cash = fee;
                let amount = exe::cash_get_display_amount(&cash, 5);
                if kind == 1 {
                    covered!("tel25_8 exchange with a fee");
                    exe::cash_format_money(PRINTOUT_SCRATCH_TEXT, 0x7d0, amount, 5);
                    comp_lang_format()(t, 0x98e628, PRINTOUT_SCRATCH_TEXT);
                } else {
                    exe::cash_format_money(t, 0xc8, amount, 5);
                }
            } else {
                let word = match kind {
                    1 => 0x98e620,
                    2 => 0x98e5fc,
                    4 => 0x98e61c,
                    _ => 0x98e614,
                };
                comp_lang_format()(t, word);
            }
            printout_column(t, 8);
            printout_write_row(file);
            any = true;
        }
        if !any {
            covered!("tel25_8 no transfers");
            comp_lang_format()(PRINTOUT_LINE, 0xa78b64, PRINTOUT_NO_TEXT);
            printout_write_row(file);
        }
        printout_write(file, PRINTOUT_NEWLINE);
    }
    if list != 0 {
        exe::crt_free(list);
    }
    printout_end_section(file);
    1
}

/// Comparer for `printouts_write_club_transfers`: side by side over the text written (`compare_printout_text`).
pub(crate) unsafe extern "C" fn compare_printouts_write_club_transfers(club: usize, reserves: u32, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(IDX_PRINTOUTS_WRITE_CLUB_TRANSFERS);
    compare_printout_text(file, || format!("club transfers printout ({:#x})", club), |f| o(club, reserves, f), |f| printouts_write_club_transfers(club, reserves, f))
}

/// Rule: a club's squad statistics, one row per player of the squad by name.
///
/// `write_club_squad_stats(club, reserves, file)`. The squad list by name (`build_squad_list_by_name_for_printout`
/// 0x4849c0 into a local `LLIST`); the scope is 5 for a national team, 6 otherwise. The heading " <%s - season> <%s -
/// scope> Stats" takes the club's season of the start year and the scope's label (into the money text buffer); then
/// the column titles and dashes. Each player of the list: the squad number (2 wide), the name (style 3, 25 wide) and
/// nine statistics of the player record (`+0x61`) in the scope: appearances (7 wide), goals, conceded (5), penalties
/// (7), assists, yellow and red cards, man of the match and the average rating (5 each). The section ends and the list
/// is freed. Returns 1.
/// Original: `printouts_write_club_squad_stats` (0x7d5280, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_club_squad_stats(club: usize, _reserves: u32, file: usize) -> u32 {
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    exe::build_squad_list_by_name_for_printout(club, l);
    let scope = if exe::database_is_national_team(club) != 0 { PRINTOUT_SCOPE_NATIONAL } else { PRINTOUT_SCOPE_CLUB } as u32;
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_club_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, club, t, 0xc8);
    printout_stats_scope_label(scope, PRINTOUT_SCRATCH_TEXT, 0x7d0);
    comp_lang_format()(PRINTOUT_LINE, 0xa78b40, t, PRINTOUT_SCRATCH_TEXT);
    printout_write_heading(file);
    printout_write_titles(file, 0xa78ae0, 0, PRINTOUT_WIDE_DASHES);
    printout_list_rewind(l);
    while printout_list_peek(l) != 0 {
        covered!("tel25_8 squad statistics row");
        let person = Staff(read_usize(exe::llist_get_next(l)));
        write_u8(PRINTOUT_LINE, 0);
        printout_squad_number(club, person.addr(), t);
        exe::database_format_staff_name(person, t, 0xc8, 3);
        printout_column(t, 0x19);
        let player = person.player().addr();
        for (stat, options, width) in PRINTOUT_SQUAD_STATS {
            printout_player_stat(player, scope, stat, t, options);
            printout_column(t, width);
        }
        printout_write_row(file);
    }
    printout_end_section(file);
    exe::drop_llist(l);
    1
}

/// Comparer for `printouts_write_club_squad_stats`: side by side over the text written (`compare_printout_text`).
pub(crate) unsafe extern "C" fn compare_printouts_write_club_squad_stats(club: usize, reserves: u32, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(IDX_PRINTOUTS_WRITE_CLUB_SQUAD_STATS);
    compare_printout_text(file, || format!("club squad statistics printout ({:#x})", club), |f| o(club, reserves, f), |f| printouts_write_club_squad_stats(club, reserves, f))
}

/// Rule: a club's printout: the squad, its statistics, its transfers and its fixtures written to a file (by default
/// "~printout.txt"), then sent to the printer when asked; the temporary file is removed.
///
/// `print_club_report(club, reserves, kind, file, print)`. No club: the printouts.cpp 88 box and 0. The period is set
/// first: [`PRINTOUT_PERIOD_START`] from `printout_club_period_start` and [`PRINTOUT_PERIOD_END`] a year later (both
/// through `Date::Date` and `Date::operator=`). The file opens for writing ("w"; failing: the 113 box and 0). The
/// heading " <%s - club> - {}<%s - date>" (langlib_sprintf with 0x64, the club's name from
/// `club_get_national_team_name(club, 0, .., 0xc8, 3, 1)` and today's long date in the money text buffer) gets a row
/// of asterisks, a newline, the heading, a newline, the asterisks and three newlines. Then by the bits of `kind`: 1
/// the squad (the rows of `printout_club_squad`: number 2 wide with "-" for none, name style 3 25 wide, positions 11,
/// the nation's three letters or "-" 3, the birth date 8, the age 3, the caps and international goals or "-" 3, the
/// shorter of the two wage texts or "-" 8, the contract's end or "-" 8, the value for a player with a club and a value
/// or "Free" 8; the contract is the loan one when the main contract is another club's and the loan is this club's), 2
/// the squad statistics (0x7d5280), 4 the transfers (0x7d60c0) and 8 the fixtures (0x7d69f0), each with (club,
/// reserves, file); any of them returning 0 makes the result 0 (otherwise 1). The file is closed. With `print`: it opens
/// again for reading ("r"; failing: the 175 box and 0), a print job of 0x8c starts and each line read (0xc8 bytes,
/// its final newline cut) becomes a page break when it is a single space, otherwise a printed line; then the job is
/// printed and ended and the file closed. Without a file given, "~printout.txt" is removed. Returns the result.
/// Original: `printouts_print_club_report` (0x7d4460, printouts.cpp, cdecl) [name: none in the map]
pub(crate) unsafe extern "C" fn printouts_print_club_report(club: usize, reserves: u32, kind: u32, file: usize, print: u32) -> u32 {
    if club == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x58);
        return 0;
    }
    let (day, year) = printout_club_period_start(club);
    let (mut date, mut out) = ([0u8; 8], [0u8; 8]);
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    exe::date_operator_3d(PRINTOUT_PERIOD_START, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    exe::new_date(date.as_mut_ptr() as usize, read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2).wrapping_add(1) as u32);
    exe::date_operator_3d(PRINTOUT_PERIOD_END, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    let name = if file != 0 { file } else { PRINTOUT_TEMP_FILE };
    let f = exe::crt_fopen(name, 0x99c370usize);
    if f == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x71);
        return 0;
    }
    covered!("tel25_8 club printout");
    let mut club_name = [0u8; 0xc8];
    (exe::club_get_national_team_name(club, (0) as u32 as usize, club_name.as_mut_ptr() as usize, 0xc8, 3, 1) as u32);
    exe::date_get_long_date_string(G_CURRENT_DATE, PRINTOUT_SCRATCH_TEXT, 0x7d0, 0xff);
    comp_lang_format()(PRINTOUT_LINE, 0xa78a20, 0x64u32, club_name.as_ptr(), PRINTOUT_SCRATCH_TEXT);
    printout_write_all(f, &[PRINTOUT_STARS, PRINTOUT_NEWLINE, PRINTOUT_LINE, PRINTOUT_NEWLINE, PRINTOUT_STARS, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE]);
    let kind = kind as u8;
    let mut result = 1;
    if kind & 1 != 0 {
        printout_club_squad(club, f);
    }
    for (bit, writer) in [(2u8, 0x7d5280usize), (4, 0x7d60c0), (8, 0x7d69f0)] {
        if kind & bit != 0 {
            let w: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(writer);
            if w(club, reserves, f) == 0 {
                result = 0;
            }
        }
    }
    exe::crt_fclose(f);
    if print != 0 {
        let f = exe::crt_fopen(name, 0x9899fcusize);
        if f == 0 {
            tel26_error_box(PRINTOUTS_CPP, 0xaf);
            return 0;
        }
        covered!("tel25_8 print the club printout");
        exe::screenmanager_begin_print_job(G_SCREEN_MANAGER, 0x8c);
        while printout_get_line(PRINTOUT_LINE, 0xc8, f) != 0 {
            let n = printout_text_length(PRINTOUT_LINE);
            if n != 0 && read_u8(PRINTOUT_LINE + n - 1) == b'\n' {
                write_u8(PRINTOUT_LINE + n - 1, 0);
            }
            if read_u8(PRINTOUT_LINE) == b' ' && read_u8(PRINTOUT_LINE + 1) == 0 {
                exe::screenmanager_add_print_page_break(G_SCREEN_MANAGER);
            } else {
                exe::screenmanager_add_print_line(G_SCREEN_MANAGER, PRINTOUT_LINE);
            }
        }
        exe::screenmanager_print_job(G_SCREEN_MANAGER);
        exe::screenmanager_end_print_job(G_SCREEN_MANAGER);
        exe::crt_fclose(f);
    }
    if file == 0 {
        exe::crt_remove(PRINTOUT_TEMP_FILE as *const u8);
    }
    result
}

/// Comparer for `printouts_print_club_report`. With a file given and no printing: the copy first writes the same
/// printout into a scratch file (its writers 0x7d5280, 0x7d60c0 and 0x7d69f0 through their own comparers), then the
/// original writes the real one, and the two files (and the returns and the period) must match; the scratch file is
/// removed. Otherwise (the temporary file or the printer) only the original runs, and the period it leaves
/// ([`PRINTOUT_PERIOD_START`], [`PRINTOUT_PERIOD_END`]) is checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_printouts_print_club_report(club: usize, reserves: u32, kind: u32, file: usize, print: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32>(IDX_PRINTOUTS_PRINT_CLUB_REPORT);
    if club == 0 {
        return o(club, reserves, kind, file, print);
    }
    let (day, year) = printout_club_period_start(club);
    let plan = (day, year, day, (year + 1) & 0xffff);
    let period = || (read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2) as u32,
                     read_u16(PRINTOUT_PERIOD_END) as u32, read_u16(PRINTOUT_PERIOD_END + 2) as u32);
    if file == 0 || print != 0 {
        let r = o(club, reserves, kind, file, print);
        let got = period();
        SCOREBOARD.record(got == plan, || format!("club printout ({:#x}, kind {:#x}): period {:?} x plan {:?}", club, kind, got, plan));
        return r;
    }
    let copy_name = PRINTOUT_CLUB_COPY_FILE.as_ptr() as usize;
    let rr = printouts_print_club_report(club, reserves, kind, copy_name, 0);
    let got_r = period();
    let ro = o(club, reserves, kind, file, print);
    let got = period();
    let (to, tr) = (printout_named_file_text(file), printout_named_file_text(copy_name));
    exe::crt_remove(copy_name as *const u8);
    let (to, tr) = (to.unwrap_or_default(), tr.unwrap_or_default());
    let k = to.iter().zip(tr.iter()).position(|(x, y)| x != y).unwrap_or(to.len().min(tr.len()));
    SCOREBOARD.record(ro == rr && got == plan && got_r == plan && to == tr, || {
        format!("club printout ({:#x}, kind {:#x}): return {} x {}, period {:?} / {:?} x plan {:?}, text {} x {} bytes, first difference at {} ({:?} x {:?})",
                club, kind, ro, rr, got, got_r, plan, to.len(), tr.len(), k, printout_snippet(&to, k), printout_snippet(&tr, k))
    });
    ro
}

/// Rule: a club's fixtures and results over the printout's period, with the scorers.
///
/// `write_club_fixtures(club, reserves, file)`. The heading " <%s - season> Fixtures" (the club's season of the
/// printout's start year, 0x669630), the column titles and their dashes. The club's fixtures between
/// [`PRINTOUT_PERIOD_START`] and [`PRINTOUT_PERIOD_END`] (`FIXTURE_MANAGER::find_fixtures` by the club's id, any
/// competition and kind, side mark `reserves` = 0) fill a local list; a match statistics record is allocated with its
/// two team blocks constructed (failing: the printouts.cpp 706 box, the list destroyed and 0). Each fixture is a row:
/// its short date (8 wide); the opposition (the away club when the club is at home, else the home club) by short
/// name (20 wide); the opposition's nation by its three letters when the club is not a national team and the two
/// nations differ, else empty (3 wide); the venue H, A or N (`comp_util_get_fixture_venue_side` 1, 2, other; 3 wide);
/// the competition's round label (form 3, 30 wide); the result from the club's side, after extra time when there is
/// one (`fixture::HOME_GOALS_EXTRA_TIME` not 0xff), else at full time, "P-P" for a postponed match (0xfe) and "---"
/// when not played (5 wide). A played match (full-time goals not negative) with a statistics slot whose entry
/// matches the fixture's clubs and date (`Match_Stats::fixture_entry_matches`) and reads
/// (`Match_Stats::read_match_record`) gets its attendance (6 wide) and the scorers of the club's side (the club is
/// side 1 when its id is not the record's home club's): for each of the 50 events up to the first with side 0xff,
/// not yet counted, a goal or penalty goal of the club's side or an own goal of the other side counts every event of
/// that kind (goal and penalty goal together) by the same player and side, marking them counted; the player's name
/// (`match_events_format_player_name`, full form) gives "name", "name pen", "name n", "name n (1 pen)", "name n
/// pens", "name n (p pens)", "name og" or "name n ogs !", joined by ", ". When the scorers text passes 50
/// characters it is cut where it stood before the last scorer, written after the row, and the line restarts with
/// spaces up to the scorers column and the last scorer. Any other fixture shows "-----" (6 wide) as its attendance.
/// The record is destroyed and the section ends (two newlines and a lone space). Returns 1.
/// Original: `printouts_write_club_fixtures` (0x7d69f0, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_club_fixtures(club: usize, reserves: u32, file: usize) -> u32 {
    let club = Club(club);
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2) as u32,
                                       read_u16(PRINTOUT_PERIOD_END) as u32, read_u16(PRINTOUT_PERIOD_END + 2) as u32, read_u32(club.addr()), l, u32::MAX, 0xffff_fffe, (reserves == 0) as u32);
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_club_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, club.addr(), t, 0xc8);
    comp_lang_format()(PRINTOUT_LINE, 0xa78ed8, t);
    printout_write_heading(file);
    printout_write_titles(file, PRINTOUT_FIXTURE_TITLES, 0, PRINTOUT_FIXTURE_DASHES);
    let record = exe::crt_operator_new(MATCH_STATS_RECORD_SIZE);
    if record == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x2c2);
        exe::drop_llist(l);
        return 0;
    }
    exe::crt_vector_constructor_iterator(record + match_record::HOME_TEAM, PRINTOUT_TEAM_BLOCK_SIZE, 2, PRINTOUT_TEAM_BLOCK_CONSTRUCTOR, PRINTOUT_TEAM_BLOCK_DESTRUCTOR);
    covered!("tel25_9 club fixtures");
    printout_list_rewind(l);
    loop {
        let game = Fixture(exe::llist_get_next(l));
        if game.is_null() {
            break;
        }
        write_u8(PRINTOUT_LINE, 0);
        let mut date = [0u8; 8];
        let d = exe::new_date(date.as_mut_ptr() as usize, game.day() as u32, game.year() as u32);
        printout_short_date(d, t, 0xc8, 0xff);
        printout_column(t, 8);
        let home = game.home_club() == club;
        let opposition = Club(read_usize(game.addr() + if home { fixture::AWAY_CLUB } else { fixture::HOME_CLUB }));
        printout_copy(t, opposition.addr() + club::SHORT_NAME);
        printout_column(t, 0x14);
        let nation = opposition.nation();
        if exe::database_is_national_team(club) == 0 && club.nation() != nation {
            printout_copy(t, nation.addr() + nation::THREE_LETTER_NAME);
        } else {
            write_u8(t, 0);
        }
        printout_column(t, 3);
        let venue = match exe::comp_util_get_fixture_venue_side(club, game) {
            1 => 0xa78de8,
            2 => 0xa78dcc,
            _ => 0xa78db0,
        };
        comp_lang_format()(t, venue, PRINTOUT_NO_TEXT);
        printout_column(t, 3);
        exe::comp_util_format_fixture_round_label(game, 3, t, 0xc8);
        printout_column(t, 0x1e);
        let extra = game.home_goals_extra_time() as u8;
        let goals = game.home_goals() as u8;
        let score = if extra != 0xff {
            Some((extra, (game.away_goals_extra_time() as u8)))
        } else if goals == 0xfe || goals == 0xff {
            None
        } else {
            Some((goals, (game.away_goals() as u8)))
        };
        match score {
            Some((h, a)) => {
                let (first, second) = if home { (h, a) } else { (a, h) };
                comp_lang_format()(t, 0x990bb4, first as i8 as i32, second as i8 as i32);
            }
            None if goals == 0xfe => { comp_lang_format()(t, 0xa78d90, PRINTOUT_NO_TEXT); }
            None => printout_copy(t, 0x98f858),
        }
        printout_column(t, 5);
        let slot = game.match_stats_slot() as i32;
        let read = goals as i8 > -1
            && slot > -1
            && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot, game.home_club_id() as i32, game.away_club_id() as i32,
                                                      game.day() as i32, game.year() as i32) != 0
            && exe::match_stats_read_match_record(read_usize(G_MATCH_STATS), slot as u32, record) != 0;
        if read {
            comp_crt_format()(t, 0x98c11c, read_i32(record + match_record::ATTENDANCE));
            printout_column(t, 6);
            let side = (read_u32(club.addr()) != read_u32(record + match_record::HOME_CLUB_ID)) as u8;
            printout_scorers(record, side, file);
        } else {
            printout_copy(t, 0x99c18c);
            printout_column(t, 6);
        }
        printout_write_row(file);
    }
    exe::crt_vector_destructor_iterator(record + match_record::HOME_TEAM, PRINTOUT_TEAM_BLOCK_SIZE, 2, PRINTOUT_TEAM_BLOCK_DESTRUCTOR);
    exe::crt_operator_delete(record);
    printout_end_section(file);
    exe::drop_llist(l);
    1
}

/// Comparer for `printouts_write_club_fixtures`: side by side over the text written (`compare_printout_text`); the
/// original and the copy each build and destroy their own match statistics record (no global effect).
pub(crate) unsafe extern "C" fn compare_printouts_write_club_fixtures(club: usize, reserves: u32, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(IDX_PRINTOUTS_WRITE_CLUB_FIXTURES);
    compare_printout_text(file, || format!("club fixtures printout ({:#x}, reserves {})", club, reserves),
                          |f| o(club, reserves, f), |f| printouts_write_club_fixtures(club, reserves, f))
}

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
pub(crate) const PUNDIT_MEDIA_TYPES: [(usize, i8); 6] = [(0xa794a8, 1), (0xa79498, 7), (0xa79490, 0), (0xa79488, 3), (0xa7947c, 5), (0xa79470, 4)];
/// "LOCAL_DAILY" (code 2; the last test).
pub(crate) const PUNDIT_LOCAL_DAILY: usize = 0xa79464;

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
    original_fn::<unsafe extern "C" fn()>(IDX_PRINTOUTS_STATIC_INIT_DATE_B74360)()
}

/// Comparer for `printouts_static_destroy_date_b74360`.
pub(crate) unsafe extern "C" fn compare_printouts_static_destroy_date_b74360() {
    original_fn::<unsafe extern "C" fn()>(IDX_PRINTOUTS_STATIC_DESTROY_DATE_B74360)()
}

/// Comparer for `printouts_static_init_date_b74358`.
pub(crate) unsafe extern "C" fn compare_printouts_static_init_date_b74358() {
    // simplification: the atexit registration does not run twice
    original_fn::<unsafe extern "C" fn()>(IDX_PRINTOUTS_STATIC_INIT_DATE_B74358)()
}

/// Comparer for `printouts_static_destroy_date_b74358`.
pub(crate) unsafe extern "C" fn compare_printouts_static_destroy_date_b74358() {
    original_fn::<unsafe extern "C" fn()>(IDX_PRINTOUTS_STATIC_DESTROY_DATE_B74358)()
}

/// Comparer for `pundit_list_read_pundit_line`: reading the file does not run twice, so the original runs and the
/// record it added (the tail of the list that grew) is checked against `pundit_parse_line` on the line it read; a line
/// it skipped must add nothing.
pub(crate) unsafe extern "thiscall" fn compare_pundit_list_read_pundit_line(this: usize, file: usize, line: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_PUNDIT_LIST_READ_PUNDIT_LINE);
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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_PARSE_MEDIA_TYPE_NAME)(this, name);
    let n = parse_media_type_name(this, name);
    SCOREBOARD.record(r == n, || format!("parse_media_type_name({:?}): original={} rust={}", tel36_c_string(name, 0x64), r, n));
    r
}

/// Comparer for `pundit_list_parse_quoted_token`: the token written (100 bytes) and the address back, side by side.
pub(crate) unsafe extern "thiscall" fn compare_pundit_list_parse_quoted_token(this: usize, src: usize, out: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_PUNDIT_LIST_PARSE_QUOTED_TOKEN);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&[(out, 0x64)], || o(this, src, out), || pundit_list_parse_quoted_token(this, src, out));
    SCOREBOARD.record(same && r == n, || format!("PunditList::parse_quoted_token({:?}): original={:#x} rust={:#x} differences {:?}", tel36_c_string(src, 0x80), r, n, first_differences(&mo, &mr)));
    r
}

/// One media.out line per pundit of a list: `line(record)` gives the format and its place names.
pub(crate) unsafe fn pundit_list_print(file: usize, list: usize, line: impl Fn(usize) -> (usize, Option<usize>, Option<usize>)) {
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
pub(crate) unsafe fn pundit_city_name(k: i32) -> usize {
    read_usize(G_CITY_TABLE).wrapping_add((k as usize).wrapping_mul(PUNDIT_CITY_SIZE)) + PUNDIT_PLACE_NAME
}

pub(crate) unsafe fn pundit_nation_name(k: i32) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add((k as usize).wrapping_mul(PUNDIT_NATION_SIZE)) + PUNDIT_PLACE_NAME
}

/// The contents of a pundit list (the bytes the original writes: 0..0x66 and 0xca..0xd4 of each record).
pub(crate) unsafe fn pundit_list_contents(list: usize) -> Vec<Vec<u8>> {
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

/// The text into the file through the exe's `fprintf` (0x9464f0, cdecl: the file, then the text as the format).
pub(crate) unsafe fn printout_put(text: usize, file: usize) {
    // fprintf(file, format) (0x9464f0, cdecl): at 0x7d7e6e the original pushes the text, then the file (esi)
    exe::at_9464f0(file, text);
}

/// `fgets` (0x94700c, cdecl): a line of at most `size` − 1 bytes into `buffer`; 0 at the end of the file.
pub(crate) unsafe fn printout_get_line(buffer: usize, size: u32, file: usize) -> usize {
    exe::at_94700c(buffer, size, file)
}

/// The length of the C string at `s`.
pub(crate) unsafe fn printout_text_length(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// The period `printouts_print_comp_report` covers for a competition: (day, year) of its start, from the nation's
/// season end day (`key_nation_record::SEASON_END_DAY` of the competition's entry in [`G_LEAGUE_TABLE`], 0x48 bytes
/// each, when the entry has its first dword and `SPLIT_SEASON_FLAG`): the last such day not after today (today before
/// it: last year's); without them, day 0 of this year.
pub(crate) unsafe fn printout_comp_period_start(comp: usize) -> (u32, u32) {
    let entry = G_LEAGUE_TABLE + exe::key_nation_index_of_comp(comp) as i8 as isize as usize * 0x48;
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    if read_usize(entry) != 0 && read_u32(entry + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
        let day = read_u16(entry + key_nation_record::SEASON_END_DAY);
        if (read_i16(G_CURRENT_DATE) as i32) < day as i32 {
            return (day as u32, year.wrapping_sub(1) & 0xffff);
        }
        return (day as u32, year);
    }
    (0, year)
}

/// The exe's `fprintf` (0x9464f0, cdecl): the file, the format and its arguments.
pub(crate) unsafe fn printout_fprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_9464F0)
}

/// A text into the printout file. The original passes the text itself as the format of `fprintf` (a '%' in a name
/// would be read as a conversion): so does the copy.
pub(crate) unsafe fn printout_write(file: usize, text: usize) {
    printout_fprintf()(file, text);
}

/// The texts of `texts` into the printout file, in order.
pub(crate) unsafe fn printout_write_all(file: usize, texts: &[usize]) {
    for &t in texts {
        printout_write(file, t);
    }
}

/// The end of a printout section: two newlines and a line with one space ("%s\n").
pub(crate) unsafe fn printout_end_section(file: usize) {
    printout_write(file, PRINTOUT_NEWLINE);
    printout_write(file, PRINTOUT_NEWLINE);
    printout_fprintf()(file, PRINTOUT_FORMAT_LINE, PRINTOUT_PAGE_BREAK);
}

/// A section heading already in [`PRINTOUT_LINE`]: equals, newline, the heading, newline, equals, two newlines.
pub(crate) unsafe fn printout_write_heading(file: usize) {
    printout_write_all(file, &[PRINTOUT_EQUALS, PRINTOUT_NEWLINE, PRINTOUT_LINE, PRINTOUT_NEWLINE, PRINTOUT_EQUALS, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE]);
}

/// The column titles `titles` (a format with up to one argument) and the dashes `dashes` under them.
pub(crate) unsafe fn printout_write_titles(file: usize, titles: usize, argument: usize, dashes: usize) {
    comp_lang_format()(PRINTOUT_LINE, titles, argument);
    printout_write_all(file, &[PRINTOUT_LINE, PRINTOUT_NEWLINE, dashes, PRINTOUT_NEWLINE]);
}

/// `strcpy`.
pub(crate) unsafe fn printout_copy(dest: usize, src: usize) {
    core::ptr::copy(src as *const u8, dest as *mut u8, printout_text_length(src) + 1);
}

/// `strcat`.
pub(crate) unsafe fn printout_append(dest: usize, src: usize) {
    printout_copy(dest + printout_text_length(dest), src);
}

/// A column of a printout row: `text` padded (or cut) to `width` (the inlined `printouts_pad_string`), appended to
/// [`PRINTOUT_LINE`] with the two spaces after it.
pub(crate) unsafe fn printout_column(text: usize, width: u32) {
    printouts_pad_string(text, width);
    printout_append(PRINTOUT_LINE, text);
    printout_append(PRINTOUT_LINE, PRINTOUT_GAP);
}

/// The row in [`PRINTOUT_LINE`] into the file, and a newline.
pub(crate) unsafe fn printout_write_row(file: usize) {
    printout_write(file, PRINTOUT_LINE);
    printout_write(file, PRINTOUT_NEWLINE);
}

/// Up to 40 bytes of a printout text from `k`, for the divergence message.
pub(crate) fn printout_snippet(t: &[u8], k: usize) -> String {
    String::from_utf8_lossy(&t[k.min(t.len())..(k + 40).min(t.len())]).into_owned()
}

/// Side-by-side comparison of a printout writer: the original writes into one scratch file and the copy into
/// another (the exe's own `fopen`, binary "w+b", so no newline is translated), each from the same [`PRINTOUT_LINE`]
/// and [`PRINTOUT_SCRATCH_TEXT`]; the two texts, the returns and those two buffers must match. Then the original's
/// text is copied into the real file (`fwrite`, which translates the newlines as the original's `fprintf` would have)
/// and the scratch files are removed: the real file gets the text once. Without scratch files only the original runs.
pub(crate) unsafe fn compare_printout_text(file: usize, what: impl FnOnce() -> String, original: impl FnOnce(usize) -> u32, rust: impl FnOnce(usize) -> u32) -> u32 {
    let (name_o, name_r) = (PRINTOUT_SCRATCH_ORIGINAL.as_ptr(), PRINTOUT_SCRATCH_COPY.as_ptr());
    let a = exe::crt_fopen(name_o as usize, b"w+b\0".as_ptr() as usize);
    let b = if a != 0 { exe::crt_fopen(name_r as usize, b"w+b\0".as_ptr() as usize) } else { 0 };
    if b == 0 {
        if a != 0 {
            exe::crt_fclose(a);
            exe::crt_remove(name_o);
        }
        return original(file);
    }
    let regions = [(PRINTOUT_LINE, 0xc8), (PRINTOUT_SCRATCH_TEXT, 0x7d0)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || original(a), || rust(b));
    let (to, tr) = (printout_file_text(a), printout_file_text(b));
    let k = to.iter().zip(tr.iter()).position(|(x, y)| x != y).unwrap_or(to.len().min(tr.len()));
    SCOREBOARD.record(equal && ro == rr && to == tr, || {
        format!("{}: return {} x {}, text {} x {} bytes, first difference at {} ({:?} x {:?}), buffers {:?}",
                what(), ro, rr, to.len(), tr.len(), k, printout_snippet(&to, k), printout_snippet(&tr, k), first_differences(&mo, &mr))
    });
    if !to.is_empty() {
        exe::crt_fwrite(to.as_ptr() as usize, 1, to.len() as u32, file);
    }
    exe::crt_fclose(a);
    exe::crt_fclose(b);
    exe::crt_remove(name_o);
    exe::crt_remove(name_r);
    ro
}

/// The competition's season label (0x6697c0, key_nation.cpp, cdecl): the start year, the competition, the buffer and
/// its size.
pub(crate) unsafe fn printout_comp_season(year: u32, comp: usize, out: usize, size: u32) {
    exe::key_nation_format_comp_season(year, comp, out, size);
}

/// The competition's statistics object (`Comp_Stats`) methods called by address: `get_player_stat_label` (0x4a2b10,
/// ret 0xc: the statistic, the buffer, its size), `get_sorted_player_table` (0x4a4dc0, ret 0x10: the statistic, the
/// count out, the most rows, the position mask; a malloc'd array of 0x2d-byte rows or null) and `format_player_stat`
/// (0x4a3ee0, ret 0x14: the row, the statistic, the buffer, its size, the substitutes split).
pub(crate) unsafe fn printout_stat_label(stats: usize, stat: u32, out: usize, size: u32) {
    exe::comp_stats_get_player_stat_label(stats, stat, out, size);
}

pub(crate) unsafe fn printout_sorted_stat_table(stats: usize, stat: u32, count: *mut i32, most: u32, positions: u32) -> usize {
    exe::comp_stats_get_sorted_player_table(stats, stat, count, most, positions)
}

pub(crate) unsafe fn printout_format_stat(stats: usize, row: usize, stat: u32, out: usize, size: u32, split: u32) {
    exe::comp_stats_format_player_stat(stats, row, stat, out, size, split);
}

/// Whether a dashes line goes before row `k` of a league stage of `rows` rows: after the first row when the stage
/// has no promotion places (`+0xbe`) and flag 1 of `+0xd9`; after the promotion places (`+0xbe`), after the play-off
/// places that follow them (`+0xbf`), before the relegation play-off places (`+0xc0`) and before the relegation
/// places (`+0xc1`) [inference from the row marks].
pub(crate) unsafe fn printout_table_rule_before(stage: usize, k: i8, rows: i16) -> bool {
    let (k, rows) = (k as i32, rows as i32);
    let up = read_i8(stage + 0xbe) as i32;
    if up == 0 && read_u8(stage + 0xd9) & 1 != 0 && k == 1 {
        return true;
    }
    let playoff = read_i8(stage + 0xbf) as i32;
    let down_playoff = read_i8(stage + 0xc0) as i32;
    let down = read_i8(stage + 0xc1) as i32;
    (up > 0 && k == up) || (playoff > 0 && k == playoff + up) || (down_playoff > 0 && k == rows - down - down_playoff) || (down > 0 && k == rows - down)
}

/// The mark of a league table row by its fate (`+0x37` of the row): 5 champions "C", 6/7/8 qualified "Q", 0
/// promoted "P", 1/2 play-offs "Pl", 3 relegated "R", any other none.
pub(crate) unsafe fn printout_table_mark(fate: u8, t: usize) {
    let format = match fate {
        5 => 0xa78f10,
        6 | 7 | 8 => 0x99c1b8,
        0 => 0x99c20c,
        1 | 2 => 0x99c1d4,
        3 => 0x99c1f0,
        _ => {
            write_u8(t, 0);
            return;
        }
    };
    comp_lang_format()(t, format, PRINTOUT_NO_TEXT);
}

/// `TRANSFER_MANAGER::get_past_transfers` (0x8cc030, thiscall, ret 0x18): the start month and year, the end month
/// and year, the array out (malloc'd 0x25-byte records) and the count out.
pub(crate) unsafe fn printout_past_transfers(start_month: u32, start_year: u32, end_month: u32, end_year: u32, list: *mut usize, count: *mut i32) {
    exe::transfer_manager_get_past_transfers(read_usize(G_TRANSFER_MANAGER), start_month, start_year, end_month, end_year, list, count);
}

/// `key_nation_format_club_season` (0x669630, cdecl): the year, the club, the buffer and its size.
pub(crate) unsafe fn printout_club_season(year: u32, club: usize, out: usize, size: u32) {
    (exe::key_nation_format_club_season(year, club, out, size) as u32);
}

/// `Date::get_short_date_string` (0x54a650, thiscall, ret 0xc): the date, the buffer, its size and the time of day
/// (0xff: none).
pub(crate) unsafe fn printout_short_date(date: usize, out: usize, size: u32, team: u32) {
    (exe::date_get_short_date_string(date, out, size, team) as u32);
}

/// `LLIST` (14 bytes): its constructor, the item under the cursor without moving it (0x68d2e0) and the rewind (the
/// cursor `+0xa` back to the head `+2`).
pub(crate) unsafe fn printout_list_peek(list: usize) -> usize {
    exe::llist_get_current(list)
}

pub(crate) unsafe fn printout_list_rewind(list: usize) {
    write_u32(list + 0xa, read_u32(list + 2));
}

/// The squad number column of a squad row: the absolute number when the club uses squad numbers and the player has
/// one (`get_squad_number`, asked twice as in the original), else "-"; 2 wide.
pub(crate) unsafe fn printout_squad_number(club: usize, person: usize, t: usize) {
    let manager = read_usize(G_SQUAD_MANAGER);
    if exe::squad_manager_club_uses_squad_numbers(manager, club, 0) != 0 && exe::squad_manager_get_squad_number(manager, person, club) != 0 {
        let n = exe::squad_manager_get_squad_number(manager, person, club) as i32;
        comp_crt_format()(t, PRINTOUT_FORMAT_NUMBER, n.abs());
    } else {
        comp_crt_format()(t, PRINTOUT_DASH);
    }
    printout_column(t, 2);
}

/// `PlayerStats` (at [`G_PLAYER_STATS`]): the scope's label (0x7c14b0, ret 0xc: the scope, the buffer, its size) and
/// a statistic of a player (0x7c25b0, ret 0x20: the player, the scope, the statistic, the buffer, its size and three
/// options) [names from player_stats.cpp's asserts; the options' meaning unknown].
pub(crate) unsafe fn printout_stats_scope_label(scope: u32, out: usize, size: u32) {
    exe::player_stats_get_category_name(read_usize(G_PLAYER_STATS), scope, (out) as usize as *mut u8, size);
}

pub(crate) unsafe fn printout_player_stat(player: usize, scope: u32, stat: u32, out: usize, options: [u32; 3]) {
    exe::playerstats_format_player_stat(read_usize(G_PLAYER_STATS), player, scope, stat, out, 0xc8, options[0], options[1], options[2]);
}

/// The period of a club's printout: (day, year) of its start, from the club's nation's season end day
/// (`key_nation_record::SEASON_END_DAY` of the entry of [`G_LEAGUE_TABLE`] for `key_nation_index_of_club`) when the
/// club is no national team and the entry has `SPLIT_SEASON_FLAG`: the last such day not after today (today before
/// it: last year's); otherwise day 0 of this year.
pub(crate) unsafe fn printout_club_period_start(club: usize) -> (u32, u32) {
    let entry = G_LEAGUE_TABLE + exe::key_nation_index_of_club(club) as i8 as isize as usize * 0x48;
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    if exe::database_is_national_team(club) != 0 || read_u32(entry + key_nation_record::SPLIT_SEASON_FLAG) == 0 {
        return (0, year);
    }
    let day = read_u16(entry + key_nation_record::SEASON_END_DAY);
    let before_the_end = (read_i16(G_CURRENT_DATE) as i32) < day as i32;
    (day as u32, if before_the_end { year.wrapping_sub(1) & 0xffff } else { year })
}

/// The squad section of a club's printout (bit 1): the squad by name, one row per player with his number, name,
/// positions, nation, birth date, age, caps, international goals, wages, contract end and value.
pub(crate) unsafe fn printout_club_squad(club: usize, file: usize) {
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    exe::build_squad_list_by_name_for_printout(club, l);
    comp_lang_format()(PRINTOUT_LINE, 0xa789b4);
    printout_write_heading(file);
    printout_write_titles(file, 0xa788ec, 0, PRINTOUT_SQUAD_DASHES);
    printout_list_rewind(l);
    let column = |t: usize, width: u32| {
        let s = exe::printouts_pad_string(t, width);
        printout_append(PRINTOUT_LINE, s);
        printout_append(PRINTOUT_LINE, PRINTOUT_GAP);
    };
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    while printout_list_peek(l) != 0 {
        covered!("tel25_8 club printout squad row");
        let person = Staff(read_usize(exe::llist_get_next(l)));
        let manager = read_usize(G_CONTRACT_MANAGER);
        let mut contract = exe::contract_manager_get_contract(manager, person);
        if contract != 0 && read_u32(club) != read_u32(contract + 4) && exe::contract_manager_get_loan_contract(manager, person) != 0
            && read_u32(exe::contract_manager_get_loan_contract(manager, person) + 4) == read_u32(club) {
            covered!("tel25_8 loan contract");
            contract = exe::contract_manager_get_loan_contract(manager, person);
        }
        write_u8(PRINTOUT_LINE, 0);
        // the squad number: the original pads it with the inlined pad_string, the rest through the call
        printout_squad_number(club, person.addr(), t);
        exe::database_format_staff_name(person, t, 0xc8, 3);
        column(t, 0x19);
        database_format_player_positions(person.player().addr(), t, 0xc8, 1);
        column(t, 0xb);
        let nation = person.nation();
        printout_copy(t, if !nation.is_null() { nation.addr() + nation::THREE_LETTER_NAME } else { PRINTOUT_DASH });
        column(t, 3);
        printout_short_date(person.addr() + staff::DATE_OF_BIRTH, t, 0xc8, 0xff);
        column(t, 8);
        comp_crt_format()(t, PRINTOUT_FORMAT_NUMBER, (person.age() as i8) as i32);
        column(t, 3);
        for field in [staff::INT_APPS, staff::INT_GOALS] {
            let n = read_u8(person.addr() + field);
            if n != 0 {
                comp_crt_format()(t, PRINTOUT_FORMAT_NUMBER, n as u32);
            } else {
                printout_copy(t, PRINTOUT_DASH);
            }
            column(t, 3);
        }
        if contract != 0 {
            printout_contract_wage(contract, t, 0, 2);
            printout_contract_wage(contract, PRINTOUT_SCRATCH_TEXT, 0, 1);
            if printout_text_length(PRINTOUT_SCRATCH_TEXT) <= printout_text_length(t) {
                printout_copy(t, PRINTOUT_SCRATCH_TEXT);
            }
        } else {
            printout_copy(t, PRINTOUT_DASH);
        }
        column(t, 8);
        if contract != 0 {
            exe::staff_contract_format_end_date_text(contract, t);
        } else {
            printout_copy(t, PRINTOUT_DASH);
        }
        column(t, 8);
        let value = person.value() as u32;
        if person.club_job().addr() != 0 && value != 0 {
            let cash = value;
            exe::cash_format_display_amount((&cash) as *const u32 as *const i32, t, 0xc8, 5);
        } else {
            comp_lang_format()(t, 0x98e614);
        }
        column(t, 8);
        printout_write_row(file);
    }
    printout_end_section(file);
    exe::drop_llist(l);
}

/// The whole text of a closed file by name ("rb"), or None.
pub(crate) unsafe fn printout_named_file_text(name: usize) -> Option<Vec<u8>> {
    let f = exe::crt_fopen(name, b"rb\0".as_ptr() as usize);
    if f == 0 {
        return None;
    }
    let t = printout_file_text(f);
    exe::crt_fclose(f);
    Some(t)
}

/// The scorers of the club's side in the match record `record` (the club `side` 0 home, 1 away) appended to
/// [`PRINTOUT_LINE`], breaking lines into the file: see `printouts_write_club_fixtures`.
pub(crate) unsafe fn printout_scorers(record: usize, side: u8, file: usize) {
    let mut scorers = [0u8; 0x400];
    let mut name = [0u8; 0x100];
    let mut entry = [0u8; 0x200];
    let (s, n, t) = (scorers.as_mut_ptr() as usize, name.as_mut_ptr() as usize, entry.as_mut_ptr() as usize);
    let indent = printout_text_length(PRINTOUT_LINE) as i16;
    let mut done = [0u32; PRINTOUT_EVENT_SLOTS];
    let mut cut = 0i16;
    for k in 0..PRINTOUT_EVENT_SLOTS {
        let event = record + k * 4;
        let who = read_u8(event + match_record::EVENTS);
        if who == 0xff {
            break;
        }
        if done[k] != 0 {
            continue;
        }
        let kind = read_u8(event + match_record::EVENT_KIND);
        let own_goal = match kind {
            PRINTOUT_EVENT_GOAL | PRINTOUT_EVENT_PENALTY_GOAL if who == side => false,
            PRINTOUT_EVENT_OWN_GOAL if who as i8 as i32 == 1 - side as i8 as i32 => true,
            _ => continue,
        };
        let player = read_u8(event + match_record::EVENT_PLAYER);
        let (mut count, mut pens) = (0i8, 0i8);
        for (j, flag) in done.iter_mut().enumerate() {
            let other = record + j * 4;
            let other_kind = read_u8(other + match_record::EVENT_KIND);
            let same = if own_goal { other_kind == PRINTOUT_EVENT_OWN_GOAL } else { other_kind == PRINTOUT_EVENT_GOAL || other_kind == PRINTOUT_EVENT_PENALTY_GOAL };
            if read_u8(other + match_record::EVENTS) == who && read_u8(other + match_record::EVENT_PLAYER) == player && same {
                count = count.wrapping_add(1);
                if other_kind == PRINTOUT_EVENT_PENALTY_GOAL {
                    pens = pens.wrapping_add(1);
                }
                *flag = 1;
            }
        }
        if own_goal {
            covered!("tel25_9 own goal scorer");
        }
        exe::match_events_format_player_name(record, player as i8 as i32, who as i8 as i32, n as *mut u8, 0xc8, 0, 0, 2);
        printout_scorer_entry(t, n, own_goal, count, pens);
        if read_u8(s) != 0 {
            printout_append(s, 0xa78d54);
        }
        printout_append(s, t);
        if printout_text_length(s) > PRINTOUT_SCORERS_WIDTH {
            covered!("tel25_9 scorers line break");
            write_u8((s as isize + cut as isize) as usize, 0);
            printout_append(PRINTOUT_LINE, s);
            printout_write_row(file);
            let mut end = 0i16;
            if indent > 0 {
                core::ptr::write_bytes(PRINTOUT_LINE as *mut u8, b' ', indent as usize);
                end = indent;
            }
            write_u8((PRINTOUT_LINE as isize + end as isize) as usize, 0);
            printout_copy(s, t);
        }
        cut = printout_text_length(s) as i16;
    }
    printout_append(PRINTOUT_LINE, s);
}

/// The parsing of one pundit line (`PunditList::read_pundit_line` after the read), with the serial the record gets.
pub(crate) unsafe fn pundit_parse_line(this: usize, line: usize, serial: u16) -> PunditLine {
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let mut token = [0u8; 0x64];
    let mut id = 0i16;
    let mut name = [0u8; 0x64];
    let mut average_name = [0u8; 0x64];
    let mut place = [0u8; 0x64];
    let mut country = [0u8; 0x64];
    let mut p = pundit_list_parse_quoted_token(this, line, token.as_mut_ptr() as usize);
    if scan(token.as_ptr() as usize, 0xa6bc70, &mut id as *mut i16) == 0 {
        return PunditLine::Fail;
    }
    p = pundit_list_parse_quoted_token(this, p, token.as_mut_ptr() as usize);
    exe::at_9466d0(name.as_mut_ptr() as usize, (token.as_ptr() as usize) as usize as *const u8, 0x64);
    name[0x63] = 0;
    p = pundit_list_parse_quoted_token(this, p, average_name.as_mut_ptr() as usize);
    let average = parse_media_type_name(this, average_name.as_ptr() as usize);
    let local = average == 2 || average == 4;
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
    exe::at_9466d0(r + PUNDIT_NAME, (name.as_ptr() as usize) as usize as *const u8, 0x64);
    write_u16(r, serial);
    write_u8(r + PUNDIT_NAME + 0x63, 0);
    write_u8(r + PUNDIT_MEDIA, average as u8);
    write_i32(r + PUNDIT_CITY, city);
    write_i32(r + PUNDIT_NATION, nation);
    write_u8(r + PUNDIT_ID, id as u8);
    PunditLine::Add(local, record)
}

/// The C string at `a` (at most `n` bytes), for the messages.
pub(crate) unsafe fn tel36_c_string(a: usize, n: usize) -> String {
    let bytes: Vec<u8> = (0..n).map(|k| read_u8(a + k)).take_while(|&b| b != 0).collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// The whole text of an open scratch file.
pub(crate) unsafe fn printout_file_text(f: usize) -> Vec<u8> {
    exe::crt_fseek(f, 0, 2);
    let n = exe::crt_ftell(f).max(0) as usize;
    exe::crt_fseek(f, 0, 0);
    let mut v = vec![0u8; n];
    if n > 0 {
        let got = exe::crt_fread(v.as_mut_ptr() as usize, 1, n as u32, f) as usize;
        v.truncate(got);
    }
    v
}

/// A contract's wage as text (0x85e750, staff_contracts.cpp, thiscall, ret 0xc): the buffer, a second argument (0
/// at both calls) and the style (2 and 1) [inference: the printout keeps the shorter of the two].
pub(crate) unsafe fn printout_contract_wage(contract: usize, out: usize, second: u32, style: u32) {
    exe::staff_contract_format_wage_text(contract, out, second, style);
}

/// A scorer's entry in `t`: the player's name `name` with his `count` goals (`pens` of them penalties), or his own
/// goals; an impossible count gives "???".
pub(crate) unsafe fn printout_scorer_entry(t: usize, name: usize, own_goal: bool, count: i8, pens: i8) {
    let (count, pens) = (count as i32, pens as i32);
    if own_goal {
        match count {
            1 => { comp_lang_format()(t, 0xa78d7c, name); }
            c if c > 1 => { comp_lang_format()(t, 0xa78d58, name, c); }
            _ => printout_copy(t, 0xa14f84),
        }
        return;
    }
    if count == 1 {
        match pens {
            0 => printout_copy(t, name),
            1 => { comp_lang_format()(t, 0xa78d40, name); }
            _ => printout_copy(t, 0xa14f84),
        }
    } else if count >= 2 {
        if pens == 0 {
            comp_lang_format()(t, 0xa78d24, name, count);
        } else if pens == 1 {
            comp_lang_format()(t, 0xa78d00, name, count);
        } else if pens == count {
            comp_lang_format()(t, 0xa78ce0, name, count);
        } else if pens >= 2 {
            comp_lang_format()(t, 0xa78cac, name, count, pens);
        } else {
            printout_copy(t, 0xa14f84);
        }
    } else {
        printout_copy(t, 0xa14f84);
    }
}

/// Whether a place name field matches the token (non-empty and `strncmp` of 26 characters).
pub(crate) unsafe fn pundit_name_matches(field: usize, token: &[u8]) -> bool {
    read_u8(field) != 0 && exe::crt_strncmp(field, token.as_ptr() as usize, 0x1a) == 0
}

/// The pundit files by language (pundit.eng for English, 1, 11, 12 and any other value), the fallback pundit.cfg,
/// the folder key, the open modes, the media debug option and its file, and its header and footer lines.
pub(crate) const PUNDIT_FILES: [usize; 13] = [0xa793ec, 0xa793ec, 0xa7944c, 0xa79440, 0xa79434, 0xa79410, 0xa793f8, 0xa79458, 0xa7941c, 0xa79428,
    0xa79404, 0xa793ec, 0xa793ec];

pub(crate) const PUNDIT_FILE_ENGLISH: usize = 0xa793ec;

pub(crate) const PUNDIT_FILE_DEFAULT: usize = 0xa793e0;

pub(crate) const PUNDIT_DATA_FOLDER: usize = 0x9b5348;

pub(crate) const PUNDIT_READ: usize = 0x9899fc;

pub(crate) const PUNDIT_MEDIA_OPTION: usize = 0xa793d8;

pub(crate) const PUNDIT_COMMAND_LINE: usize = 0xae2c98;

pub(crate) const PUNDIT_MEDIA_OUT: usize = 0xa793c8;

pub(crate) const PUNDIT_MEDIA_OUT_MODE: usize = 0xa793d4;

pub(crate) const PUNDIT_MEDIA_START: usize = 0xa793ac;

pub(crate) const PUNDIT_MEDIA_END: usize = 0xa791c0;

/// The media.out lines: local with city and nation, with the city only, with neither; national with both, with the
/// nation only, with the city only, with neither (gender, type, serial, name, then the places).
pub(crate) const PUNDIT_LOCAL_BOTH: usize = 0xa7936c;

pub(crate) const PUNDIT_LOCAL_CITY: usize = 0xa7932c;

pub(crate) const PUNDIT_LOCAL_NONE: usize = 0xa792e8;

pub(crate) const PUNDIT_NATIONAL_BOTH: usize = 0xa792a8;

pub(crate) const PUNDIT_NATIONAL_NATION: usize = 0xa79264;

pub(crate) const PUNDIT_NATIONAL_CITY: usize = 0xa79220;

pub(crate) const PUNDIT_NATIONAL_NONE: usize = 0xa791dc;

/// The nation record size (`G_NATION_TABLE`).
pub(crate) const PUNDIT_NATION_SIZE: usize = 0x122;

/// The match incident code of an obscene gesture (the caller, `MATCH_DAY`, only calls on it) and the ban it gives.
pub(crate) const OBSCENE_BAN_DELAY_DAYS: u32 = 14;

pub(crate) const OBSCENE_BAN_REASON: u8 = 5;

/// The competition object's ruling body byte (the first rule of the ban).
pub(crate) const COMP_OBJECT_RULING_BODY: usize = 0x50;

/// The 79-byte staff state record ([G_STAFF_STATE_TABLE]) and its flags byte with the bit the ban needs (the bit
/// docs/MARCA_DO_REGEN.md reads as the regen mark).
pub(crate) const STAFF_STATE_SIZE: usize = 0x4f;

pub(crate) const STAFF_STATE_FLAGS: usize = 0xb;

pub(crate) const STAFF_STATE_BAN_BIT: u8 = 0x40;

/// `Media::queue_obscene_gesture_story` (0x7570b0, media.cpp, thiscall, ret 0xc) [inference: the name, from the
/// caller].
pub(crate) const FN_MEDIA_OBSCENE_GESTURE_STORY: usize = 0x7570b0;

/// "E:\dev\CM3\cm3 00-01\cm3\code\printouts.cpp", the path the printouts.cpp error boxes print.
pub(crate) const PRINTOUTS_CPP: usize = 0xa78a50;

/// "~printout.txt", the file a printout goes to when no file is given (it is printed and removed).
pub(crate) const PRINTOUT_TEMP_FILE: usize = 0xa78a40;

/// The period a printout covers: its start [0xb74360] and end [0xb74358] (8-byte dates, the end a year after the
/// start), and the printout's line buffer [0xb74368] (0xc8 bytes).
pub(crate) const PRINTOUT_PERIOD_START: usize = 0xb74360;

pub(crate) const PRINTOUT_PERIOD_END: usize = 0xb74358;

pub(crate) const PRINTOUT_LINE: usize = 0xb74368;

/// The row of asterisks above and below a printout's heading (0xa789bc) and "\n" (0x99cae4).
pub(crate) const PRINTOUT_STARS: usize = 0xa789bc;

pub(crate) const PRINTOUT_NEWLINE: usize = 0x99cae4;

/// The row of equals signs under and over a printout section's heading (0xa78950).
pub(crate) const PRINTOUT_EQUALS: usize = 0xa78950;

/// The two spaces between a printout row's columns (0xa78880).
pub(crate) const PRINTOUT_GAP: usize = 0xa78880;

/// "%s\n" (0x9a7d28) and " " (0x9f1308): a section ends with a line holding one space (a page break when printed).
pub(crate) const PRINTOUT_FORMAT_LINE: usize = 0x9a7d28;

pub(crate) const PRINTOUT_PAGE_BREAK: usize = 0x9f1308;

/// The exe's money text buffer (0xde1f64, 0x7d0 bytes): the printouts write long dates, labels and money there.
pub(crate) const PRINTOUT_SCRATCH_TEXT: usize = 0xde1f64;

/// The scratch files of the side-by-side comparison of a printout writer.
pub(crate) const PRINTOUT_SCRATCH_ORIGINAL: &[u8] = b"~printout_original.tmp\0";

pub(crate) const PRINTOUT_SCRATCH_COPY: &[u8] = b"~printout_copy.tmp\0";

/// The kind of competition whose statistics list nations instead of clubs (`+0x42` of the competition object: 4,
/// the international ones) [inference from the column titles].
pub(crate) const PRINTOUT_NATIONAL_COMP: u8 = 4;

/// "%d" (0x98806c), "-" (0x98ba0c) and the empty text (0xad9c64) the comments of the language files take.
pub(crate) const PRINTOUT_FORMAT_NUMBER: usize = 0x98806c;

pub(crate) const PRINTOUT_DASH: usize = 0x98ba0c;

pub(crate) const PRINTOUT_NO_TEXT: usize = 0xad9c64;

/// The dashes under the league tables' and the squad statistics' column titles (0xa78a7c).
pub(crate) const PRINTOUT_WIDE_DASHES: usize = 0xa78a7c;

/// The kind of competition object whose league tables show each club's nation (`+0x42`: 2) [inference: the
/// three-letter column only filled there].
pub(crate) const PRINTOUT_CLUBS_OF_NATIONS_COMP: u8 = 2;

/// A stage of the competition object that is a league (`+0x43` of the stage: 2).
pub(crate) const PRINTOUT_LEAGUE_STAGE: u8 = 2;

/// The numeric columns of a league table row after the team and its nation (3 wide each, signed): played (`+0x11`),
/// home won, drawn (or won on penalties, or by shoot-out), lost, for and against (`+0x20`, `+0x21`, `+0x22` bytes,
/// `+0x23`, `+0x25` words), away the same (`+0x2e`, `+0x2f`, `+0x30` bytes, `+0x31`, `+0x33` words) and the points
/// (`+0x19` word): (offset, is a word) [inference from the column titles].
pub(crate) const PRINTOUT_TABLE_COLUMNS: [(usize, bool); 12] = [
    (0x11, false), (0x20, false), (0x21, false), (0x22, false), (0x23, true), (0x25, true),
    (0x2e, false), (0x2f, false), (0x30, false), (0x31, true), (0x33, true), (0x19, true),
];

/// The dashes under the transfers' column titles (0xa78bc0) and under the squad's (0xa78884).
pub(crate) const PRINTOUT_TRANSFER_DASHES: usize = 0xa78bc0;

pub(crate) const PRINTOUT_SQUAD_DASHES: usize = 0xa78884;

/// The size of a past transfer record (`get_past_transfers`; its first dword is the staff index the name routine
/// takes) [inference from this routine].
pub(crate) const PAST_TRANSFER_SIZE: usize = 0x25;

/// The statistics columns of the squad statistics printout: (statistic, options, width): appearances, goals,
/// conceded, penalties, assists, yellow cards, red cards, man of the match and the average rating.
pub(crate) const PRINTOUT_SQUAD_STATS: [(u32, [u32; 3], u32); 9] = [
    (1, [1, 0, 1], 7), (2, [0, 0, 1], 5), (3, [0, 0, 1], 5), (4, [1, 0, 1], 7), (5, [0, 0, 1], 5),
    (0xa, [0, 0, 1], 5), (0xb, [0, 0, 1], 5), (0xc, [0, 0, 1], 5), (0x11, [0, 0, 1], 5),
];

/// The scope of a club's statistics: 5 for a national team, 6 for a club [inference: the label 0x7c14b0 names it].
pub(crate) const PRINTOUT_SCOPE_NATIONAL: u8 = 5;

pub(crate) const PRINTOUT_SCOPE_CLUB: u8 = 6;

/// The file the comparer's copy of a club printout writes to.
pub(crate) const PRINTOUT_CLUB_COPY_FILE: &[u8] = b"~printout_club_copy.txt\0";

/// The fixtures' column titles (0xa78e74) and the dashes under them (0xa78e04).
pub(crate) const PRINTOUT_FIXTURE_TITLES: usize = 0xa78e74;

pub(crate) const PRINTOUT_FIXTURE_DASHES: usize = 0xa78e04;

/// The longest scorers text on one line: past it the line is written and the scorers go on under the column.
pub(crate) const PRINTOUT_SCORERS_WIDTH: usize = 50;

/// The match statistics record (`Match_Stats`, `MATCH_STATS_RECORD_SIZE` bytes) the fixtures printout reads each
/// played match into: only its two team blocks (`match_record::HOME_TEAM`, 0x625 bytes each) are constructed
/// (0x453c30, which only gives each block's 20 rows the default date) and destroyed (0x453c10, empty): no global
/// effect, so the comparer lets the original and the copy each build their own.
pub(crate) const PRINTOUT_TEAM_BLOCK_SIZE: u32 = 0x625;

pub(crate) const PRINTOUT_TEAM_BLOCK_CONSTRUCTOR: usize = 0x453c30;

pub(crate) const PRINTOUT_TEAM_BLOCK_DESTRUCTOR: usize = 0x453c10;

/// The kinds of a match record's events the scorers list (`match_record::EVENT_KIND`).
pub(crate) const PRINTOUT_EVENT_GOAL: u8 = 1;

pub(crate) const PRINTOUT_EVENT_OWN_GOAL: u8 = 2;

pub(crate) const PRINTOUT_EVENT_PENALTY_GOAL: u8 = 3;

/// The number of event slots of a match record (`match_record::EVENTS`, 4 bytes each; side 0xff ends the list).
pub(crate) const PRINTOUT_EVENT_SLOTS: usize = 50;

pub(crate) const PUNDIT_NAME: usize = 0x2;

pub(crate) const PUNDIT_MEDIA: usize = 0xca;

pub(crate) const PUNDIT_CITY: usize = 0xcb;

pub(crate) const PUNDIT_NATION: usize = 0xcf;

pub(crate) const PUNDIT_ID: usize = 0xd3;

/// The PunditList fields: the national list (+0, an LLIST pointer), the local list (+4) and the serial counter (+8).
pub(crate) const PUNDIT_LIST_NATIONAL: usize = 0x0;

pub(crate) const PUNDIT_LIST_LOCAL: usize = 0x4;

pub(crate) const PUNDIT_LIST_SERIAL: usize = 0x8;

/// The city record (0x38 bytes, name at +4) and the nation record name fields (+4 and +0x38).
pub(crate) const PUNDIT_CITY_SIZE: usize = 0x38;

pub(crate) const PUNDIT_PLACE_NAME: usize = 0x4;

pub(crate) const PUNDIT_PLACE_SHORT_NAME: usize = 0x38;

/// What a pundit line gives: 0 back (no line, or no number), 1 back without a record, or a record for the local (true)
/// or national list.
pub(crate) enum PunditLine {
    Fail,
    Skip,
    Add(bool, [u8; PUNDIT_RECORD_SIZE]),
}

/// A pundit list record (0xd4 bytes, LLIST items): the serial (+0), the name (+2, 100 bytes), the media type (+0xca),
/// the city (+0xcb), the nation (+0xcf) and the id's low byte (+0xd3). Bytes 0x66..0xc9 are never written by the
/// original (stack garbage); the copy leaves them 0.
pub(crate) const PUNDIT_RECORD_SIZE: usize = 0xd4;
