// ---------- batch 320: the year ratings filters of the award screens and the human manager's menu bar record (award_screens.cpp, Menubar.cpp) ----------

/// The award year ratings filter of the world ratings: a player counts when his current reputation is over 2000.
///
/// `award_rating_filter_reputation_over_2000(person, fixture)`: 1 when the person, his player sheet
/// (`staff::PLAYER`) and the sheet's `player_sheet::CURRENT_REPUTATION` (signed word) > 2000; else 0. The fixture is
/// not read.
/// Original: `award_rating_filter_reputation_over_2000` (0x414b20, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_rating_filter_reputation_over_2000(person: usize, _fixture: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    covered!("tel39_3 year ratings filter");
    let sheet = read_usize(person + staff::PLAYER);
    // tel39_3: the reputation threshold
    (sheet != 0 && read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 2000) as u32
}

/// Whether the nation record `nation` (a pointer, may be null) is the fixed nation of `key` (the key holds its id).
unsafe fn award_nation_is(nation: usize, key: usize) -> bool {
    nation != 0 && read_u32(nation) == read_u32(key)
}

/// The award year ratings filter of the second ratings: a player counts when he, or the fixture's competition,
/// belongs to Denmark, Belgium, France, Holland or Sweden.
///
/// `award_rating_filter_selected_nations(person, fixture)`, in the original's order: the person's `staff::NATION` is
/// Denmark; the fixture's `fixture::COMPETITION`'s `competition::NATION` is Belgium; the person's nation or the
/// competition's nation is France; the competition's nation is Holland (a Dutch player in a foreign competition does
/// not count, unlike France and Sweden); the person's or the competition's nation is Sweden (each nation by its fixed
/// key, the id compared). A null person or fixture, competition or nation skips that test. 1 when one holds, else 0.
/// Original: `award_rating_filter_selected_nations` (0x414b40, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_rating_filter_selected_nations(person: usize, fixture: usize) -> u32 {
    let own = |key: usize| person != 0 && award_nation_is(read_usize(person + staff::NATION), key);
    let comp = |key: usize| {
        if fixture == 0 {
            return false;
        }
        let c = read_usize(fixture + fixture::COMPETITION);
        c != 0 && award_nation_is(read_usize(c + competition::NATION), key)
    };
    // tel39_3: Denmark by the person, Belgium by the competition, France by either, Holland by the competition,
    // Sweden by either
    (own(KEY_NATION_DENMARK) || comp(KEY_NATION_BELGIUM) || own(KEY_NATION_FRANCE) || comp(KEY_NATION_FRANCE)
        || comp(KEY_NATION_HOLLAND) || own(KEY_NATION_SWEDEN) || comp(KEY_NATION_SWEDEN)) as u32
}

/// The menu bar record of a human manager: the one of the current slot's manager when `manager` is null; null when
/// there is none or his id is not among the last 16 staff records.
///
/// `get_human_manager_record(manager)`: a null manager is replaced by the slot's manager ([G_SCREEN_RECORDS_MANAGER +
/// slot × 0x300]); null stays null. k = id − G_STAFF_COUNT + 16; 0 ≤ k < 16 gives record k, else null (`MENUBAR_RECORDS`, 0xf6 bytes each).
/// Original: `get_human_manager_record` (0x765480, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_human_manager_record(manager: usize) -> usize {
    let mut m = manager;
    if m == 0 {
        m = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
        if m == 0 {
            return 0;
        }
    }
    covered!("tel39_3 human record");
    // tel39_3: the position among the last 16 staff records
    let k = read_i32(m).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(16);
    if !(0..16).contains(&k) {
        return 0;
    }
    MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE
}

/// Comparer for `award_rating_filter_reputation_over_2000`: the answer side by side (nothing is written).
pub(crate) unsafe extern "C" fn compare_award_rating_filter_reputation_over_2000(person: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_AWARD_RATING_FILTER_REPUTATION_OVER_2000)(person, fixture);
    let n = award_rating_filter_reputation_over_2000(person, fixture);
    SCOREBOARD.record(r == n, || format!("award_rating_filter_reputation_over_2000({:#x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `award_rating_filter_selected_nations`: the answer side by side (nothing is written).
pub(crate) unsafe extern "C" fn compare_award_rating_filter_selected_nations(person: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_AWARD_RATING_FILTER_SELECTED_NATIONS)(person, fixture);
    let n = award_rating_filter_selected_nations(person, fixture);
    SCOREBOARD.record(r == n, || format!("award_rating_filter_selected_nations({:#x}, {:#x}) original={} rust={}", person, fixture, r, n));
    r
}

/// Comparer for `get_human_manager_record`: the record side by side (nothing is written).
pub(crate) unsafe extern "C" fn compare_get_human_manager_record(manager: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_GET_HUMAN_MANAGER_RECORD)(manager);
    let n = get_human_manager_record(manager);
    SCOREBOARD.record(r == n, || format!("get_human_manager_record({:#x}) original={:#x} rust={:#x}", manager, r, n));
    r
}

// ---------- batch 320: the small constructors and destructor of club_screens.cpp: a match record's team pair, a dated record and a human's squad view settings ----------

/// The destructor of a match record's two team blocks: the C runtime's vector destructor over the two 0x625-byte
/// blocks at +0x11f, each with `match_team_block_destructor` (0x453c10: the vector destructor over its twenty
/// 0x49-byte entries at +0x71, whose destructor 0x5a53c0 is a bare `ret`), so nothing is actually written.
///
/// `match_team_pair_destructor(this)`: `exe::crt_vector_destructor_iterator(this + 0x11f, 0x625, 2, 0x453c10)`.
/// Original: `match_team_pair_destructor` (0x453bc0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_pair_destructor(this: usize) {
    covered!("tel39_4 team pair destructor");
    exe::crt_vector_destructor_iterator(this + MATCH_TEAM_PAIR, MATCH_TEAM_BLOCK_SIZE as u32, 2, MATCH_TEAM_BLOCK_DESTRUCTOR);
}

/// Where a match record keeps its two team blocks, the size of one and the block's destructor.
const MATCH_TEAM_PAIR: usize = 0x11f;
const MATCH_TEAM_BLOCK_SIZE: usize = 0x625;
const MATCH_TEAM_BLOCK_DESTRUCTOR: usize = 0x453c10;
/// The date field of a dated record (the average rating record and its kin).
const RATING_RECORD_DATE: usize = 0x14;

/// The constructor of a record with a date at +0x14 (the average rating record of the club and competition
/// records): only the date is built.
///
/// `rating_record_construct(this)`: `Date::Date` (0x54a110) on `this + 0x14`; returns `this`.
/// Original: `rating_record_construct` (0x453c50, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn rating_record_construct(this: usize) -> usize {
    covered!("tel39_4 rating record constructor");
    // tel39_4: the date at +0x14
    exe::date_date_54a110(this + RATING_RECORD_DATE);
    this
}

/// The constructor of a human manager's squad screen view settings (15 bytes, in his HUMAN_RECORD): the default
/// view, sort and filter.
///
/// `squad_view_settings_construct(this)`: dword +0 = 0x41, dword +4 = 8, dword +8 = 0x154a21, byte +0xc = 6, bytes
/// +0xd and +0xe = 0xff [inference: the columns shown, the sort column and the filters]; returns `this`.
/// Original: `squad_view_settings_construct` (0x487810, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn squad_view_settings_construct(this: usize) -> usize {
    covered!("tel39_4 squad view settings");
    write_u32(this, 0x41);
    write_u32(this + 4, 8);
    write_u8(this + 0xc, 6);
    // tel39_4: the default view word
    write_u32(this + 8, 0x154a21);
    write_u8(this + 0xd, 0xff);
    write_u8(this + 0xe, 0xff);
    this
}

/// Comparer for `match_team_pair_destructor`: only the original runs (a destructor does not run twice; its element
/// destructors write nothing, so there is no outcome to check).
pub(crate) unsafe extern "thiscall" fn compare_match_team_pair_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_MATCH_TEAM_PAIR_DESTRUCTOR)(this)
}

/// Comparer for `rating_record_construct`: side by side over the date and the return.
pub(crate) unsafe extern "thiscall" fn compare_rating_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_RATING_RECORD_CONSTRUCT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, RATING_RECORD_DATE + 8)], || o(this), || rating_record_construct(this));
    SCOREBOARD.record(same && r == n, || format!("rating_record_construct({:#x}) original={:#x} {:x?} rust={:#x} {:x?}", this, r, mo, n, mr));
    r
}

/// Comparer for `squad_view_settings_construct`: side by side over the 15 bytes and the return.
pub(crate) unsafe extern "thiscall" fn compare_squad_view_settings_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SQUAD_VIEW_SETTINGS_CONSTRUCT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 15)], || o(this), || squad_view_settings_construct(this));
    SCOREBOARD.record(same && r == n, || format!("squad_view_settings_construct({:#x}) original {:x?} rust {:x?}", this, mo, mr));
    r
}

// ---------- batch 320: the record copies and the date comparison of comp_screens.cpp: the youngest/oldest player record, the attendance record and Date::operator== ----------

/// The size of a youngest or oldest player record (Comp_Stats and Club_Records).
const PLAYER_AGE_RECORD_SIZE: usize = 0x3f;
/// The size of an attendance record: the attendance and the match record (0x1f bytes) after it.
const ATTENDANCE_RECORD_SIZE: usize = 4 + 0x1f;

/// Copies a youngest or oldest player record (0x3f bytes): the age as a date, the holder, the holder's birth date and
/// the match it happened in.
///
/// `player_age_record_assign(this, source)`: the date at +0 (`copy_record_date`), the dwords +8, +0xc, +0x10 and
/// +0x14 [inference: the person, his club, the age in days and years], the date at +0x18, then the match record at
/// +0x20 field by field as `match_record_assign` copies it (the original has it inline). Returns `this`.
/// Original: `player_age_record_assign` (0x4a1fb0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_age_record_assign(this: usize, source: usize) -> usize {
    covered!("tel39_5 player age record copy");
    copy_record_date(this, source);
    for o in [8usize, 0xc, 0x10, 0x14] {
        write_u32(this + o, read_u32(source + o));
    }
    // tel39_5: the second date
    copy_record_date(this + 0x18, source + 0x18);
    match_record_assign(this + 0x20, source + 0x20);
    this
}

/// Copies an attendance record (the highest and lowest attendances): the attendance (dword) and its match record.
///
/// `attendance_record_assign(this, source)`: dword +0, then `match_record_assign(this + 4, source + 4)`. Returns
/// `this`.
/// Original: `attendance_record_assign` (0x4ab5b0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn attendance_record_assign(this: usize, source: usize) -> usize {
    covered!("tel39_5 attendance record copy");
    // tel39_5: the attendance
    write_u32(this, read_u32(source));
    match_record_assign(this + 4, source + 4);
    this
}

/// Whether two game dates are the same day: the year (word +2) and the day of the year (word +0) equal; the third
/// field (the leap-year flag at +4) is not compared.
///
/// `Date::operator==(this, other)`: `other` comes by value (8 bytes on the stack: `day_year` holds the day in its low
/// word and the year in its high word, `_leap` the flag). 1 when both words match, else 0; the year is compared first.
/// Original: `Date::operator==` (0x4a7f20, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn date_equals(this: usize, day_year: u32, _leap: u32) -> u32 {
    covered!("tel39_5 date equals");
    // tel39_5: the year, then the day
    ((day_year >> 16) as u16 == read_u16(this + 2) && day_year as u16 == read_u16(this)) as u32
}

/// Comparer for `player_age_record_assign`: side by side over the record and the return.
pub(crate) unsafe extern "thiscall" fn compare_player_age_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_AGE_RECORD_ASSIGN);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, PLAYER_AGE_RECORD_SIZE)], || o(this, source), || player_age_record_assign(this, source));
    SCOREBOARD.record(same && r == n, || format!("player_age_record_assign({:#x}, {:#x}) original {:x?} rust {:x?}", this, source, mo, mr));
    r
}

/// Comparer for `attendance_record_assign`: side by side over the record and the return.
pub(crate) unsafe extern "thiscall" fn compare_attendance_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_ATTENDANCE_RECORD_ASSIGN);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, ATTENDANCE_RECORD_SIZE)], || o(this, source), || attendance_record_assign(this, source));
    SCOREBOARD.record(same && r == n, || format!("attendance_record_assign({:#x}, {:#x}) original {:x?} rust {:x?}", this, source, mo, mr));
    r
}

/// Comparer for `date_equals`: the answer side by side (nothing is written).
pub(crate) unsafe extern "thiscall" fn compare_date_equals(this: usize, day_year: u32, leap: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_DATE_EQUALS)(this, day_year, leap);
    let n = date_equals(this, day_year, leap);
    SCOREBOARD.record(r == n, || format!("Date::operator==({:#x}: {:#x}, {:#x}) original={} rust={}", this, read_u32(this), day_year, r, n));
    r
}

// ---------- batch 320: the Text class (1): setting and appending the string, adding a link (comp_text.cpp) ----------

/// "%s%s", the format of the appended string.
const COMP_TEXT_JOIN: usize = 0x99ec04;
/// A Text object (8 bytes): the string (+0, heap, operator new) and the link list (+4, an LLIST of 0xd0-byte links:
/// the phrase at +0, at most 200 bytes with its terminator, the link kind at +0xc8 and its data at +0xcc).
const TEXT_STRING: usize = 0;
const TEXT_LINKS: usize = 4;
const TEXT_LINK_SIZE: usize = 0xd0;
const TEXT_LINK_KIND: usize = 0xc8;
const TEXT_LINK_DATA: usize = 0xcc;
const TEXT_LINK_MAX_PHRASE: usize = 0xc8;

/// The length of a C string.
unsafe fn comp_text_len(s: usize) -> usize {
    core::ffi::CStr::from_ptr(s as *const core::ffi::c_char).to_bytes().len()
}

/// Replaces the string of a Text with a new copy of `source`.
///
/// `Text::set_string(this, source)`: a null source shows the comp_text.cpp 139 box and gives 0. Otherwise the old
/// string is freed (operator delete), the new one allocated (operator new, the length + 1) and stored even when null
/// (then the 151 box and 0); the source is copied with its terminator and 1 comes back.
/// Original: `Text::set_string` (0x4ae660, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_set_string(this: usize, source: usize) -> u32 {
    if source == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0x8b);
        return 0;
    }
    covered!("tel39_6 set the string");
    let old = read_usize(this + TEXT_STRING);
    if old != 0 {
        exe::crt_operator_delete(old);
    }
    // tel39_6: the length with the terminator
    let n = comp_text_len(source) + 1;
    let p = exe::crt_operator_new(n as u32);
    write_usize(this + TEXT_STRING, p);
    if p == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0x97);
        return 0;
    }
    core::ptr::copy_nonoverlapping(source as *const u8, p as *mut u8, n);
    1
}

/// Appends `source` to the string of a Text, or sets it when the Text has none.
///
/// `Text::append_string(this, source)`: a null source shows the comp_text.cpp 177 box and gives 0. With no string,
/// `Text::set_string(source)`. Otherwise a new block of both lengths + 1 (operator new) is stored even when null (then
/// the 192 box and 0, and the old string is lost: a leak of the original), the two are joined into it (`sprintf`
/// "%s%s"), the old string is freed and 1 comes back.
/// Original: `Text::append_string` (0x4ae770, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_append_string(this: usize, source: usize) -> u32 {
    if source == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0xb1);
        return 0;
    }
    let old = read_usize(this + TEXT_STRING);
    if old == 0 {
        return text_set_string(this, source);
    }
    covered!("tel39_6 append to the string");
    let p = exe::crt_operator_new((comp_text_len(old) + comp_text_len(source) + 1) as u32);
    write_usize(this + TEXT_STRING, p);
    if p == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0xc0);
        return 0;
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    // tel39_6: the old string, then the new one
    format(p, COMP_TEXT_JOIN, old, source);
    exe::crt_operator_delete(old);
    1
}

/// Adds a hyperlink to a Text: a phrase of the text with a link kind and its data.
///
/// `Text::add_link(this, phrase, kind, data)`: a null phrase shows the comp_text.cpp 227 box and a phrase of 200 bytes
/// or more (with the terminator past 200) the 235 box, both giving 0. Without a link list one is made (operator new
/// 0xe, `LLIST::LLIST`; a failed allocation stores null and shows the 247 box, 0). The list is read from its head: a
/// link with the same phrase (strcmp) gives 1 when its kind and data are the same, else 0, and nothing is added.
/// Otherwise a 0xd0-byte link (the phrase with its terminator, the kind at +0xc8, the data at +0xcc; the bytes past
/// the terminator are stack garbage in the original, zero here) is appended (`LLIST::add_tail`) and 1 comes back.
/// Original: `Text::add_link` (0x4ae8a0, comp_text.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn text_add_link(this: usize, phrase: usize, kind: u32, data: u32) -> u32 {
    if phrase == 0 {
        tel26_error_box(COMP_TEXT_CPP, 0xe3);
        return 0;
    }
    let n = comp_text_len(phrase) + 1;
    if n > TEXT_LINK_MAX_PHRASE {
        tel26_error_box(COMP_TEXT_CPP, 0xeb);
        return 0;
    }
    if read_usize(this + TEXT_LINKS) == 0 {
        let l = exe::crt_operator_new(0xe);
        let l = if l != 0 { exe::new_llist(l) } else { 0 };
        write_usize(this + TEXT_LINKS, l);
        if l == 0 {
            tel26_error_box(COMP_TEXT_CPP, 0xf7);
            return 0;
        }
    }
    let list = read_usize(this + TEXT_LINKS);
    list_rewind(list);
    let wanted = core::slice::from_raw_parts(phrase as *const u8, n);
    loop {
        let e = exe::llist_get_next(read_usize(this + TEXT_LINKS));
        if e == 0 {
            break;
        }
        if core::slice::from_raw_parts(e as *const u8, n) == wanted {
            covered!("tel39_6 link already there");
            // tel39_6: the same phrase linked to the same object
            return (read_u32(e + TEXT_LINK_KIND) == kind && read_u32(e + TEXT_LINK_DATA) == data) as u32;
        }
    }
    covered!("tel39_6 add a link");
    let mut link = [0u8; TEXT_LINK_SIZE];
    link[..n].copy_from_slice(wanted);
    link[TEXT_LINK_KIND..TEXT_LINK_KIND + 4].copy_from_slice(&kind.to_le_bytes());
    link[TEXT_LINK_DATA..TEXT_LINK_DATA + 4].copy_from_slice(&data.to_le_bytes());
    exe::llist_add_tail(read_usize(this + TEXT_LINKS), link.as_ptr() as usize, TEXT_LINK_SIZE as u32);
    1
}

/// A local Text for the comparers: a copy of the string (operator new) and of the link list of `t`.
unsafe fn comp_text_clone(t: usize, with_links: bool) -> [usize; 2] {
    let mut c = [0usize; 2];
    let s = read_usize(t + TEXT_STRING);
    if s != 0 {
        let n = comp_text_len(s) + 1;
        c[0] = exe::crt_operator_new(n as u32);
        core::ptr::copy_nonoverlapping(s as *const u8, c[0] as *mut u8, n);
    }
    let l = read_usize(t + TEXT_LINKS);
    if with_links && l != 0 {
        c[1] = exe::new_llist(exe::crt_operator_new(0xe));
        exe::llist_copy_from(c[1], l);
    }
    c
}

/// Frees a comparer's local Text (its string and its list), as `Text::clear` does.
unsafe fn comp_text_free(c: &mut [usize; 2]) {
    if c[0] != 0 {
        exe::crt_operator_delete(c[0]);
    }
    if c[1] != 0 {
        exe::drop_llist(c[1]);
        exe::crt_operator_delete(c[1]);
    }
    *c = [0, 0];
}

/// The comparison of a Text method: the copy runs first on a local clone of the Text (so it reads the source before
/// the original frees anything), then the original on the Text; the string, the links and the answers must match.
unsafe fn compare_text_method(name: &str, this: usize, with_links: bool, original: impl FnOnce() -> u32, rust: impl FnOnce(usize) -> u32) -> u32 {
    let mut local = comp_text_clone(this, with_links);
    let n = rust(local.as_mut_ptr() as usize);
    let r = original();
    let (mo, mr) = (text_contents(this), text_contents(local.as_ptr() as usize));
    let same = if with_links { mo == mr } else { mo.0 == mr.0 };
    comp_text_free(&mut local);
    SCOREBOARD.record(same && r == n, || format!("{}: original {} {:?} {:x?}, rust {} {:?} {:x?}", name, r, String::from_utf8_lossy(&mo.0), mo.1,
        n, String::from_utf8_lossy(&mr.0), mr.1));
    r
}

/// Comparer for `text_set_string`: side by side on a local Text (a null source, which shows the box, only the
/// original).
pub(crate) unsafe extern "thiscall" fn compare_text_set_string(this: usize, source: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TEXT_SET_STRING);
    if source == 0 {
        return o(this, source);
    }
    compare_text_method("Text::set_string", this, false, || o(this, source), |t| text_set_string(t, source))
}

/// Comparer for `text_append_string`: side by side on a local copy of the Text (a null source only the original).
pub(crate) unsafe extern "thiscall" fn compare_text_append_string(this: usize, source: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TEXT_APPEND_STRING);
    if source == 0 {
        return o(this, source);
    }
    compare_text_method("Text::append_string", this, false, || o(this, source), |t| text_append_string(t, source))
}

/// Comparer for `text_add_link`: side by side on a local copy of the Text and its links (a null or too long phrase,
/// which shows a box, only the original).
pub(crate) unsafe extern "thiscall" fn compare_text_add_link(this: usize, phrase: usize, kind: u32, data: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_TEXT_ADD_LINK);
    if phrase == 0 || comp_text_len(phrase) + 1 > TEXT_LINK_MAX_PHRASE {
        return o(this, phrase, kind, data);
    }
    compare_text_method("Text::add_link", this, true, || o(this, phrase, kind, data), |t| text_add_link(t, phrase, kind, data))
}

// ---------- batch 320: the Text class (2): the constructor, the destructor and clearing (comp_text.cpp) ----------

/// Constructs a Text with no string and no links, then copies the initial string when one is given.
///
/// `Text::Text(this, source)`: string and link list null; a non-null source goes to `Text::set_string`. Returns
/// `this`.
/// Original: `Text::Text` (0x4ae590, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_construct(this: usize, source: usize) -> usize {
    covered!("tel39_7 construct a text");
    write_usize(this + TEXT_STRING, 0);
    write_usize(this + TEXT_LINKS, 0);
    if source != 0 {
        text_set_string(this, source);
    }
    this
}

/// Destroys a Text: frees the string and the link list (the pointers are left as they were).
///
/// `Text::~Text(this)`: the string with operator delete; the list with `LLIST::~LLIST` and operator delete.
/// Original: `Text::~Text` (0x4ae630, comp_text.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn text_destruct(this: usize) {
    covered!("tel39_7 destroy a text");
    let s = read_usize(this + TEXT_STRING);
    if s != 0 {
        exe::crt_operator_delete(s);
    }
    let l = read_usize(this + TEXT_LINKS);
    if l != 0 {
        exe::drop_llist(l);
        exe::crt_operator_delete(l);
    }
}

/// Empties a Text so it can be reused: frees the string and the link list and sets both pointers to null.
///
/// `Text::clear(this)`: the string with operator delete, then null; the list with `LLIST::~LLIST` and operator
/// delete, then null. Returns 1.
/// Original: `Text::clear` (0x4afa40, comp_text.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn text_clear(this: usize) -> u32 {
    covered!("tel39_7 clear a text");
    let s = read_usize(this + TEXT_STRING);
    if s != 0 {
        exe::crt_operator_delete(s);
        write_usize(this + TEXT_STRING, 0);
    }
    let l = read_usize(this + TEXT_LINKS);
    if l != 0 {
        exe::drop_llist(l);
        exe::crt_operator_delete(l);
        // tel39_7: the list pointer cleared
        write_usize(this + TEXT_LINKS, 0);
    }
    1
}

/// Comparer for `text_construct`: the copy builds a local Text, the original the real one; string and links equal.
pub(crate) unsafe extern "thiscall" fn compare_text_construct(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TEXT_CONSTRUCT);
    let mut local = [0usize; 2];
    text_construct(local.as_mut_ptr() as usize, source);
    let r = o(this, source);
    let (mo, mr) = (text_contents(this), text_contents(local.as_ptr() as usize));
    comp_text_free(&mut local);
    SCOREBOARD.record(r == this && mo == mr, || format!("Text::Text: original {:?}, rust {:?}", String::from_utf8_lossy(&mo.0), String::from_utf8_lossy(&mr.0)));
    r
}

/// Comparer for `text_destruct`: only the original runs (freeing does not run twice; nothing is left to check).
pub(crate) unsafe extern "thiscall" fn compare_text_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEXT_DESTRUCT)(this)
}

/// Comparer for `text_clear`: only the original runs (freeing does not run twice); then both pointers must be null
/// and the answer 1, the outcome the copy guarantees.
pub(crate) unsafe extern "thiscall" fn compare_text_clear(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TEXT_CLEAR)(this);
    let (s, l) = (read_usize(this + TEXT_STRING), read_usize(this + TEXT_LINKS));
    SCOREBOARD.record(r == 1 && s == 0 && l == 0, || format!("Text::clear({:#x}): answer {}, string {:#x}, links {:#x}", this, r, s, l));
    r
}

// ---------- batch 320: the pundit list's constructor and destructor and the obscene gesture ban (printouts.cpp) ----------

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

// ---------- tel41_1: the competition statistics: finding a club's and a player's entry and a club's statistic (comp_screens.cpp) ----------

/// The regions the two entry finders may rewrite: a table (it may be re-sorted by id), its sort byte and the sort
/// context.
unsafe fn comp_stats_find_regions(stats: usize, players: bool) -> [(usize, usize); 3] {
    if players {
        [(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
         (stats + comp_stats_obj::PLAYER_SORT, 1), (G_COMP_STATS_SORT_CONTEXT, 4)]
    } else {
        [(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
         (stats + comp_stats_obj::CLUB_SORT, 1), (G_COMP_STATS_SORT_CONTEXT, 4)]
    }
}

/// `Comp_Stats` +0x566: the temporary club entry `get_club_projected_positions` lends to the stage's projection; while it
/// is set, `find_club_entry` returns it for any club.
const COMP_STATS_PROJECTED_ENTRY: usize = 0x566;

/// Rule: a club's entry in a competition's statistics is found by binary search on the club table sorted by club id
/// (re-sorted first when another statistic left it in another order).
///
/// `Comp_Stats::find_club_entry(club)`: while a projection lends its temporary entry (+0x566), that entry, whatever the
/// club. No club: the comp_stats.CPP 3297 box and 0. No club table: 0. Unless the table is already in id order (sort
/// byte 0xff), it is marked 0xff and sorted (`G_COMP_STATS_SORT_CONTEXT` = the stats, qsort of `CLUB_COUNT` entries
/// of 0x76 bytes with `comp_stats_compare_team_entries`). Then `bsearch` with a key entry holding the club's id (the
/// same order, the context set again). Returns the entry or 0. The original builds the key as a whole 0x76-byte entry
/// on the stack (its date and run constructors only touch the stack); the order reads only the id.
/// Original: `Comp_Stats::find_club_entry` (0x4a4850, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_find_club_entry(stats: usize, club: usize) -> usize {
    let lent = read_usize(stats + COMP_STATS_PROJECTED_ENTRY);
    if lent != 0 {
        covered!("tel41_1 projected entry lent");
        return lent;
    }
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0xce1);
        return 0;
    }
    if read_usize(stats + comp_stats_obj::CLUBS) == 0 {
        return 0;
    }
    if read_u8(stats + comp_stats_obj::CLUB_SORT) != 0xff {
        write_u8(stats + comp_stats_obj::CLUB_SORT, 0xff);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
            comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
    }
    let mut key = [0u8; comp_stats_obj::CLUB_ENTRY_SIZE];
    write_u32(key.as_mut_ptr() as usize, read_u32(club)); // tel41_1: the key is the club id
    write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
    exe::crt_bsearch(key.as_ptr() as usize, read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
        comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES)
}

/// Rule: a player's entry in a competition's statistics is found by binary search on the player table sorted by
/// player id (re-sorted first when another statistic left it in another order).
///
/// `Comp_Stats::find_player_entry(player)`: no player: the comp_stats.CPP 3342 box and 0. No player table: 0. Unless
/// the table is already in id order (sort byte 0xff), it is marked 0xff and sorted (context = the stats, qsort of
/// `PLAYER_COUNT` entries of 0x2d bytes with `comp_stats_compare_player_entries`). Then `bsearch` with the player's id
/// as the key (the context set again). Returns the entry or 0.
/// Original: `Comp_Stats::find_player_entry` (0x4a4a30, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_find_player_entry(stats: usize, player: usize) -> usize {
    if player == 0 {
        show_error_box(COMP_STATS_CPP, 0xd0e);
        return 0;
    }
    if read_usize(stats + comp_stats_obj::PLAYERS) == 0 {
        return 0;
    }
    if read_u8(stats + comp_stats_obj::PLAYER_SORT) != 0xff {
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xff);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
            comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES);
    }
    let key = read_u32(player); // tel41_1: the key is the player id
    write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
    exe::crt_bsearch(&key as *const u32 as usize, read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
        comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES)
}

/// Rule: a club's statistic in a competition is the value of its entry; a club with no entry has 0.
///
/// `Comp_Stats::get_club_stat(club, stat)`: no club: the comp_stats.CPP 2274 box and 0.0. The entry
/// (`comp_stats_find_club_entry`); none: 0.0; otherwise `comp_stats_get_team_entry_value(entry, stat)`. The value
/// comes back in st0 (0.0 is the float constant `rdata::F32_0`).
/// Original: `Comp_Stats::get_club_stat` (0x4a2e10, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_club_stat(stats: usize, club: usize, stat: u32) -> f64 {
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0x8e2);
        return read_f32(rdata::F32_0) as f64;
    }
    let entry = comp_stats_find_club_entry(stats, club);
    if entry == 0 {
        covered!("tel41_1 club stat with no entry");
        return read_f32(rdata::F32_0) as f64;
    }
    exe::comp_stats_get_team_entry_value(stats, entry, stat) // tel41_1: the club statistic
}

// ---------- comparers ----------

/// Comparer for `comp_stats_find_club_entry`: side by side over the club table (it may be sorted), the sort byte, the
/// context and the entry returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_find_club_entry(stats: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_STATS_FIND_CLUB_ENTRY);
    if club == 0 {
        return o(stats, club);
    }
    let regions = comp_stats_find_regions(stats, false);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club), || comp_stats_find_club_entry(stats, club));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::find_club_entry({:#x}, {:#x}): entry {:#x} x {:#x}, differences in {:?}", stats, club, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_find_player_entry`: side by side over the player table, the sort byte, the context and
/// the entry returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_find_player_entry(stats: usize, player: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_STATS_FIND_PLAYER_ENTRY);
    if player == 0 {
        return o(stats, player);
    }
    let regions = comp_stats_find_regions(stats, true);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, player), || comp_stats_find_player_entry(stats, player));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::find_player_entry({:#x}, {:#x}): entry {:#x} x {:#x}, differences in {:?}", stats, player, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_club_stat`: side by side over the club table, its sort byte, the context and the
/// value (bit for bit).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_club_stat(stats: usize, club: usize, stat: u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> f64>(IDX_COMP_STATS_GET_CLUB_STAT);
    if club == 0 {
        return o(stats, club, stat);
    }
    let regions = comp_stats_find_regions(stats, false);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club, stat), || comp_stats_get_club_stat(stats, club, stat));
    SCOREBOARD.record(equal && ro.to_bits() == rr.to_bits(), || format!("Comp_Stats::get_club_stat({:#x}, {:#x}, {}): {} x {}, differences in {:?}", stats, club, stat as u8, ro, rr, first_differences(&mo, &mr)));
    ro
}

// ---------- tel41_2: the competition statistics: a player's statistic and the sorted team and player tables (comp_screens.cpp) ----------

/// `Comp_Stats::find_player_entry` (0x4a4a30, thiscall(stats, player), ret 4), through its address.
unsafe fn comp_stats_player_entry_of(stats: usize, player: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4a30usize);
    f(stats, player)
}

/// Rule: a player's statistic in a competition is the value of his entry; a player with no entry has 0.
///
/// `Comp_Stats::get_player_stat(player, stat)`: no player: the comp_stats.CPP 2485 box and 0.0. The entry
/// (`Comp_Stats::find_player_entry`); none: 0.0; otherwise `Comp_Stats::get_player_entry_value(entry, stat)`. The value
/// comes back in st0.
/// Original: `Comp_Stats::get_player_stat` (0x4a31a0, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_player_stat(stats: usize, player: usize, stat: u32) -> f64 {
    if player == 0 {
        show_error_box(COMP_STATS_CPP, 0x9b5);
        return read_f32(rdata::F32_0) as f64;
    }
    let entry = comp_stats_player_entry_of(stats, player);
    if entry == 0 {
        covered!("tel41_2 player stat with no entry");
        return read_f32(rdata::F32_0) as f64;
    }
    exe::comp_stats_get_player_entry_value(stats, entry, stat) // tel41_2: the player statistic
}

/// Copies one club entry (0x76 bytes) as the original does it, field by field: the id, the two match counts, the
/// home attendance, the three 5-byte arrays at +0xa, the four runs (`run_record_assign`) and the bytes +0x69..+0x75.
unsafe fn comp_stats_copy_club_entry(dst: usize, src: usize) {
    write_u32(dst, read_u32(src));
    write_u8(dst + 4, read_u8(src + 4));
    write_u8(dst + 5, read_u8(src + 5));
    write_u32(dst + 6, read_u32(src + 6));
    for o in 0xa..0x19 {
        write_u8(dst + o, read_u8(src + o));
    }
    for o in [comp_stats_club::WINNING_RUN, comp_stats_club::LOSING_RUN, comp_stats_club::UNBEATEN_RUN, comp_stats_club::WINLESS_RUN] {
        run_record_assign(dst + o, src + o);
    }
    for o in 0x69..comp_stats_obj::CLUB_ENTRY_SIZE {
        write_u8(dst + o, read_u8(src + o));
    }
}

/// Rule: a competition's team table for one statistic lists the clubs best first, at most `max` (all when negative);
/// for every statistic but 3, 4 and 5 the clubs at the bottom with 0 are left out.
///
/// `Comp_Stats::get_sorted_team_table(stat, &count, max)`: no club table: count 0 and 0. `max` below 0 is the club
/// count. Unless already sorted by `stat`, the club table is sorted by it (sort byte = stat, context = the stats,
/// qsort with `comp_stats_compare_team_entries`). Count = the smaller of the club count and `max`; for statistics
/// other than 3, 4 and 5, while the count is positive and the last counted club's value
/// (`comp_stats_get_team_entry_value`) is 0.0, the count goes down. Count 0: 0. Otherwise a block of count × 0x76
/// bytes (`crt_malloc`; failing, the comp_stats.CPP 3436 box, count 0 and 0) takes the first entries, copied field by
/// field. Returns the block (the caller frees it).
/// Original: `Comp_Stats::get_sorted_team_table` (0x4a4b20, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_sorted_team_table(stats: usize, stat_arg: u32, count: *mut i32, max: i32) -> usize {
    if read_usize(stats + comp_stats_obj::CLUBS) == 0 {
        *count = 0;
        return 0;
    }
    let max = if max < 0 { read_i32(stats + comp_stats_obj::CLUB_COUNT) } else { max };
    let stat = stat_arg as u8;
    if read_u8(stats + comp_stats_obj::CLUB_SORT) != stat {
        write_u8(stats + comp_stats_obj::CLUB_SORT, stat);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
            comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
    }
    let n = read_i32(stats + comp_stats_obj::CLUB_COUNT);
    *count = if n < max { n } else { max };
    if stat != 4 && stat != 5 && stat != 3 {
        while *count > 0 {
            let last = read_usize(stats + comp_stats_obj::CLUBS) + (*count as usize - 1) * comp_stats_obj::CLUB_ENTRY_SIZE;
            // tel41_2: the trailing clubs with 0 leave the table
            if exe::comp_stats_get_team_entry_value(stats, last, stat_arg) != read_f32(rdata::F32_0) as f64 {
                break;
            }
            *count -= 1;
        }
    }
    if *count == 0 {
        return 0;
    }
    let block = exe::crt_malloc((*count as u32).wrapping_mul(comp_stats_obj::CLUB_ENTRY_SIZE as u32));
    if block == 0 {
        show_error_box(COMP_STATS_CPP, 0xd6c);
        *count = 0;
        return 0;
    }
    let mut i = 0;
    while i < *count {
        let k = i as usize * comp_stats_obj::CLUB_ENTRY_SIZE;
        comp_stats_copy_club_entry(block + k, read_usize(stats + comp_stats_obj::CLUBS) + k);
        i += 1;
    }
    block
}

/// Rule: a competition's player table for one statistic lists the players best first, at most `max` (all when
/// negative), only those of the position groups in `groups` (0xffff = all) and never one with 0.
///
/// `Comp_Stats::get_sorted_player_table(stat, &count, max, groups)`: no player table: count 0 and 0. `max` below 0 is
/// the player count. Unless already sorted by `stat`: with statistic 0x11 the club table is first put in id order
/// (sort byte 0xff, qsort with `comp_stats_compare_team_entries`) when it is not; then the player table is sorted by
/// `stat` (sort byte = stat, context = the stats, qsort with `comp_stats_compare_player_entries`). Count = the player
/// count; while it is positive and the last player's value (`Comp_Stats::get_player_entry_value`) is 0.0, it goes
/// down. Count 0: 0. Otherwise a block of count × 0x2d bytes (`crt_malloc`; failing, the comp_stats.CPP 3526 box,
/// count 0 and 0). The first `count` players are walked while fewer than `max` were taken: with `groups` other than
/// 0xffff a player whose position group (`comp_stats_get_player_position_group` of his record in
/// `G_PLAYER_DATA_TABLE`, by the id at +0) has no bit in `groups` is skipped, and so is one whose value is 0.0; the
/// others are copied whole (0x2d bytes). Count = how many were taken. Returns the block (the caller frees it).
/// Original: `Comp_Stats::get_sorted_player_table` (0x4a4dc0, comp_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_sorted_player_table(stats: usize, stat_arg: u32, count: *mut i32, max: i32, groups: u32) -> usize {
    if read_usize(stats + comp_stats_obj::PLAYERS) == 0 {
        *count = 0;
        return 0;
    }
    let max = if max < 0 { read_i32(stats + comp_stats_obj::PLAYER_COUNT) } else { max };
    let stat = stat_arg as u8;
    if read_u8(stats + comp_stats_obj::PLAYER_SORT) != stat {
        if stat == 0x11 && read_u8(stats + comp_stats_obj::CLUB_SORT) != 0xff {
            write_u8(stats + comp_stats_obj::CLUB_SORT, 0xff);
            write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
            exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
        }
        write_u8(stats + comp_stats_obj::PLAYER_SORT, stat);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
            comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES);
    }
    let zero = read_f32(rdata::F32_0) as f64;
    *count = read_i32(stats + comp_stats_obj::PLAYER_COUNT);
    while *count > 0 {
        let last = read_usize(stats + comp_stats_obj::PLAYERS) + (*count as usize - 1) * comp_stats_obj::PLAYER_ENTRY_SIZE;
        if exe::comp_stats_get_player_entry_value(stats, last, stat_arg) != zero {
            break;
        }
        *count -= 1;
    }
    if *count == 0 {
        return 0;
    }
    let block = exe::crt_malloc((*count as u32).wrapping_mul(comp_stats_obj::PLAYER_ENTRY_SIZE as u32));
    if block == 0 {
        show_error_box(COMP_STATS_CPP, 0xdc6);
        *count = 0;
        return 0;
    }
    let mut taken = 0i32;
    let mut i = 0i32;
    while i < *count {
        if taken >= max {
            break;
        }
        let entry = read_usize(stats + comp_stats_obj::PLAYERS) + i as usize * comp_stats_obj::PLAYER_ENTRY_SIZE;
        let wanted = groups as u16 == 0xffff || {
            let sheet = read_usize(G_PLAYER_DATA_TABLE) + (read_i32(entry) as usize).wrapping_mul(COMP_STATS_PLAYER_SHEET_SIZE);
            // tel41_2: the position group filter
            comp_stats_get_player_position_group(sheet) & groups as u16 != 0
        };
        if wanted && exe::comp_stats_get_player_entry_value(stats, entry, stat_arg) != zero {
            core::ptr::copy_nonoverlapping(entry as *const u8, (block + taken as usize * comp_stats_obj::PLAYER_ENTRY_SIZE) as *mut u8,
                comp_stats_obj::PLAYER_ENTRY_SIZE);
            taken += 1;
        }
        i += 1;
    }
    *count = taken;
    block
}

/// The player record of `G_PLAYER_DATA_TABLE` (0x46 bytes), indexed by the id at +0 of a player entry.
const COMP_STATS_PLAYER_SHEET_SIZE: usize = 0x46;

// ---------- comparers ----------

/// Comparer for `comp_stats_get_player_stat`: side by side over the player table (it may be sorted by id), its sort
/// byte, the context and the value (bit for bit).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_player_stat(stats: usize, player: usize, stat: u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> f64>(IDX_COMP_STATS_GET_PLAYER_STAT);
    if player == 0 {
        return o(stats, player, stat);
    }
    let regions = [
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (stats + comp_stats_obj::PLAYER_SORT, 1),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, player, stat), || comp_stats_get_player_stat(stats, player, stat));
    SCOREBOARD.record(equal && ro.to_bits() == rr.to_bits(), || format!("Comp_Stats::get_player_stat({:#x}, {:#x}, {}): {} x {}, differences in {:?}", stats, player, stat as u8, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// The regions a sorted table may rewrite: both tables (the sorts), both sort bytes and the context.
unsafe fn comp_stats_sorted_table_regions(stats: usize) -> [(usize, usize); 5] {
    [
        (read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (stats + comp_stats_obj::CLUB_SORT, 1),
        (stats + comp_stats_obj::PLAYER_SORT, 1),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ]
}

/// The two blocks (and counts) of a sorted table side by side: equal counts and bytes; the copy's block is freed.
unsafe fn comp_stats_sorted_blocks_equal(ro: usize, rr: usize, no: i32, nr: i32, size: usize) -> bool {
    let equal = no == nr && (ro == 0) == (rr == 0)
        && (ro == 0 || core::slice::from_raw_parts(ro as *const u8, no as usize * size) == core::slice::from_raw_parts(rr as *const u8, nr as usize * size));
    if rr != 0 {
        exe::crt_free(rr);
    }
    equal
}

/// Comparer for `comp_stats_get_sorted_team_table`: side by side over the tables, sort bytes and context (each side
/// allocates its own block); the two blocks and counts must be equal; the copy's block is freed.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_sorted_team_table(stats: usize, stat: u32, count: *mut i32, max: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i32, i32) -> usize>(IDX_COMP_STATS_GET_SORTED_TEAM_TABLE);
    let regions = comp_stats_sorted_table_regions(stats);
    let mut mine = 0i32;
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, stat, count, max),
        || comp_stats_get_sorted_team_table(stats, stat, &mut mine, max));
    let blocks = comp_stats_sorted_blocks_equal(ro, rr, *count, mine, comp_stats_obj::CLUB_ENTRY_SIZE);
    SCOREBOARD.record(equal && blocks, || format!("Comp_Stats::get_sorted_team_table({:#x}, stat {}, max {}): count {} x {}, blocks equal {}, differences in {:?}", stats, stat as u8, max, *count, mine, blocks, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_sorted_player_table`: as the team table's, over the player block.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_sorted_player_table(stats: usize, stat: u32, count: *mut i32, max: i32, groups: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i32, i32, u32) -> usize>(IDX_COMP_STATS_GET_SORTED_PLAYER_TABLE);
    let regions = comp_stats_sorted_table_regions(stats);
    let mut mine = 0i32;
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, stat, count, max, groups),
        || comp_stats_get_sorted_player_table(stats, stat, &mut mine, max, groups));
    let blocks = comp_stats_sorted_blocks_equal(ro, rr, *count, mine, comp_stats_obj::PLAYER_ENTRY_SIZE);
    SCOREBOARD.record(equal && blocks, || format!("Comp_Stats::get_sorted_player_table({:#x}, stat {}, max {}, groups {:#x}): count {} x {}, blocks equal {}, differences in {:?}", stats, stat as u8, max, groups as u16, *count, mine, blocks, first_differences(&mo, &mr)));
    ro
}

// ---------- tel41_3: the competition statistics at the end of a season: the all-time records, the archive and the record news (comp_screens.cpp) ----------

/// The `Comp_Stats` fields of the season end (offsets in the object; the all-time records start at +0x26 and the
/// season records at +0x28f, `comp_stats_obj`).
#[allow(dead_code)]
mod comp_stats_season_end {
    /// i32 / u32 / u16 in the all-time records (+0x6b): the most points in a season, the club and the season's year.
    pub(crate) const ALL_TIME_MOST_POINTS: usize = 0x91;
    pub(crate) const ALL_TIME_MOST_POINTS_CLUB: usize = 0x95;
    pub(crate) const ALL_TIME_MOST_POINTS_YEAR: usize = 0x99;
    /// The all-time rating record (+0x189): f32 rating, the holder's name reference, the games (byte), the club and
    /// the season's year (word).
    pub(crate) const ALL_TIME_RATING: usize = 0x1af;
    pub(crate) const ALL_TIME_RATING_HOLDER: usize = 0x1b3;
    pub(crate) const ALL_TIME_RATING_GAMES: usize = 0x1cb;
    pub(crate) const ALL_TIME_RATING_CLUB: usize = 0x1cc;
    pub(crate) const ALL_TIME_RATING_YEAR: usize = 0x1d0;
    /// i16: how many season ends the statistics went through; the record news waits for the third.
    pub(crate) const SEASONS_ENDED: usize = 0x500;
    /// The 22 "record broken" flags (dword each, record kinds 1..22) and the dword "some flag is set".
    pub(crate) const RECORD_FLAGS: usize = 0x50a;
    pub(crate) const ANY_RECORD_FLAG: usize = 0x562;
}

/// Record kind of a flag: `RECORD_FLAGS + 4 × (kind − 1)`.
const fn comp_stats_record_flag(kind: usize) -> usize {
    comp_stats_season_end::RECORD_FLAGS + 4 * (kind - 1)
}

/// The record kinds the season end flags: most titles (1), most points in a season (8) and the best average
/// rating (0x12).
const COMP_STATS_KIND_MOST_TITLES: usize = 1;
const COMP_STATS_KIND_MOST_POINTS: usize = 8;
const COMP_STATS_KIND_RATING: usize = 0x12;

/// x87 `fcomp` + `test ah, 0x40`: C3 set, i.e. equal or unordered.
fn comp_stats_c3(a: f64, b: f64) -> bool {
    a == b || a.is_nan() || b.is_nan()
}

/// Rule: at a season's end the all-time records of a competition take the season's most points and best average
/// rating when they beat them, and the most titles is recounted; from the third season end on, a broken record of a
/// league division (most points, rating with more than 3 games, most titles from 5) raises its news flag.
///
/// `Comp_Stats::update_season_end_records()`, for a competition that is not a friendly one
/// (`Friendly::is_friendly_competition` of the competition's record, asked again before each part):
/// 1. `Comp_Stats::update_most_titles_record`; when the record's club changed and the record (signed byte +0x26) is
///    at least 5: flag 1 and the "any flag" dword set.
/// 2. Unless the competition's byte +0x42 is 4: with no season most points yet (-1) and a club table, the club table
///    sorted by points (statistic 2, unless already) and the leader's points (`_ftol` of statistic 2), its id and 0
///    written as the season record. A season record above -1 that beats the all-time one (signed; or the all-time is
///    -1) is copied to it (points, club, the word, then the competition's year in the word); with at least 3 season
///    ends (signed word +0x500) and a league division (`comp_util_is_league_division` of the competition id): flag 8.
/// 3. With no season rating (-1.0, or unordered) `Comp_Stats::update_season_rating_record`. A season rating above
///    -1.0 that beats the all-time one (or the all-time is -1.0/unordered) is copied to it: the rating, the holder
///    (`record_holder_assign`), the games byte, the club, the word and then the competition's year; with at least 3
///    season ends, a league division and more than 3 games (signed byte): flag 0x12.
/// The season end count goes up by 1 (always). Returns 1.
/// Original: `Comp_Stats::update_season_end_records` (0x4a19f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_update_season_end_records(stats: usize) -> u32 {
    use comp_stats_season_end as f;
    let comp = read_usize(stats + comp_stats_obj::COMP);
    let friendly = || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION)) != 0;
    let league_division = || exe::comp_util_is_league_division(read_i32(read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION))) != 0;
    if !friendly() && comp_stats_update_most_titles_record(stats) != 0 && read_u8(stats + comp_stats_obj::MOST_TITLES) as i8 >= 5 {
        covered!("tel41_3 most titles record news");
        write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_MOST_TITLES), 1);
        write_u32(stats + f::ANY_RECORD_FLAG, 1);
    }
    if !friendly() && read_u8(comp + league::CLUB_SIDES) != 4 {
        if read_i32(stats + comp_stats_obj::SEASON_MOST_POINTS) == -1 && read_usize(stats + comp_stats_obj::CLUBS) != 0 {
            if read_u8(stats + comp_stats_obj::CLUB_SORT) != 2 {
                write_u8(stats + comp_stats_obj::CLUB_SORT, 2);
                write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
                exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                    comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
            }
            let leader = read_usize(stats + comp_stats_obj::CLUBS);
            write_i32(stats + comp_stats_obj::SEASON_MOST_POINTS, ftol(team_entry_value(stats, leader, 2)));
            write_u32(stats + comp_stats_obj::SEASON_MOST_POINTS_CLUB, read_u32(leader));
            write_u16(stats + comp_stats_obj::SEASON_MOST_POINTS_EXTRA, 0);
        }
        let season = read_i32(stats + comp_stats_obj::SEASON_MOST_POINTS);
        let all_time = read_i32(stats + f::ALL_TIME_MOST_POINTS);
        // tel41_3: the season's most points beats the all-time record
        if season > -1 && (all_time == -1 || season > all_time) {
            write_i32(stats + f::ALL_TIME_MOST_POINTS, season);
            write_u32(stats + f::ALL_TIME_MOST_POINTS_CLUB, read_u32(stats + comp_stats_obj::SEASON_MOST_POINTS_CLUB));
            write_u16(stats + f::ALL_TIME_MOST_POINTS_YEAR, read_u16(stats + comp_stats_obj::SEASON_MOST_POINTS_EXTRA));
            write_u16(stats + f::ALL_TIME_MOST_POINTS_YEAR, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
            if read_i16(stats + f::SEASONS_ENDED) >= 3 && league_division() {
                covered!("tel41_3 most points record news");
                write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_MOST_POINTS), 1);
                write_u32(stats + f::ANY_RECORD_FLAG, 1);
            }
        }
    }
    if !friendly() {
        let minus_one = read_f64(rdata::F64_NEG_1);
        if comp_stats_c3(read_f32(stats + comp_stats_obj::SEASON_RATING) as f64, minus_one) {
            comp_stats_update_season_rating_record(stats);
        }
        let season = read_f32(stats + comp_stats_obj::SEASON_RATING) as f64;
        if season > minus_one {
            let all_time = read_f32(stats + f::ALL_TIME_RATING) as f64;
            if comp_stats_c3(all_time, minus_one) || season > all_time {
                write_u32(stats + f::ALL_TIME_RATING, read_u32(stats + comp_stats_obj::SEASON_RATING));
                exe::record_holder_assign(stats + f::ALL_TIME_RATING_HOLDER, stats + comp_stats_obj::SEASON_RATING_HOLDER);
                write_u8(stats + f::ALL_TIME_RATING_GAMES, read_u8(stats + comp_stats_obj::SEASON_RATING_GAMES));
                write_u32(stats + f::ALL_TIME_RATING_CLUB, read_u32(stats + comp_stats_obj::SEASON_RATING_CLUB));
                write_u16(stats + f::ALL_TIME_RATING_YEAR, read_u16(stats + comp_stats_obj::SEASON_RATING_EXTRA));
                write_u16(stats + f::ALL_TIME_RATING_YEAR, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
                if read_i16(stats + f::SEASONS_ENDED) >= 3 && league_division() && read_u8(stats + f::ALL_TIME_RATING_GAMES) as i8 > 3 {
                    covered!("tel41_3 rating record news");
                    write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_RATING), 1);
                    write_u32(stats + f::ANY_RECORD_FLAG, 1);
                }
            }
        }
    }
    write_i16(stats + f::SEASONS_ENDED, read_i16(stats + f::SEASONS_ENDED).wrapping_add(1));
    1
}

/// `comp_season_records_construct` (0x49f160, thiscall, ret 0): the constructors of the sub-records of a season
/// record block (dates, runs, holders); writes only inside the block.
unsafe fn comp_season_records_construct(records: usize) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x49f160usize);
    f(records);
}

/// Rule: when a competition's season closes, its season records may be appended to the shared history file (the
/// records screen lists them later), then the season records are cleared, the runs kept for the next season and the
/// two tables freed.
///
/// `Comp_Stats::archive_season_and_reset(archive)`: with `archive` ≠ 0, a row of 0x26b bytes (on the stack, its
/// sub-records constructed first, `comp_season_records_construct`) takes the competition's year (word) and a copy of
/// the season records (+0x28f, field by field as `comp_season_records_assign`), and is appended to the shared file
/// list (`FILE_LLIST::add_node` of `COMP_STATS_ARCHIVE_LIST` with the two archive keys +0x4f8/+0x4fc, which it may
/// set). Then, always: `comp_season_records_clear` of the season records, `Comp_Stats::save_team_runs`, and the club
/// and player tables freed (`crt_free`; pointer, capacity and count 0, sort byte 0xfe). Returns 1.
/// Original: `Comp_Stats::archive_season_and_reset` (0x4a1c50, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_archive_season_and_reset(stats: usize, archive: u32) -> u32 {
    let mut row = [0u8; SEASON_RECORDS_ROW];
    let r = row.as_mut_ptr() as usize;
    comp_season_records_construct(r + 2);
    if archive != 0 {
        covered!("tel41_3 season archived");
        write_u16(r, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
        comp_season_records_assign(r + 2, stats + comp_stats_obj::SEASON_RECORDS);
        // tel41_3: the season row goes to the history file
        exe::file_llist_add_node(read_usize(COMP_STATS_ARCHIVE_LIST), stats + comp_stats_obj::ARCHIVE_KEY1,
            stats + comp_stats_obj::ARCHIVE_KEY2, row.as_ptr());
    }
    comp_season_records_clear(stats + comp_stats_obj::SEASON_RECORDS);
    comp_stats_save_team_runs(stats); // tel41_3: the runs kept for the next season
    if read_usize(stats + comp_stats_obj::CLUBS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::CLUBS));
        write_usize(stats + comp_stats_obj::CLUBS, 0);
        write_u32(stats + comp_stats_obj::CLUB_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::CLUB_COUNT, 0);
        write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    }
    if read_usize(stats + comp_stats_obj::PLAYERS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::PLAYERS));
        write_usize(stats + comp_stats_obj::PLAYERS, 0);
        write_u32(stats + comp_stats_obj::PLAYER_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::PLAYER_COUNT, 0);
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    }
    1
}

/// The size of a news item on the stack (`NewsItem::NewsItem`, 0xe0 bytes).
const COMP_STATS_NEWS_ITEM_BYTES: usize = 0xe0;
/// The news type of a broken competition record.
const COMP_STATS_RECORD_NEWS_TYPE: u32 = 0x3e9;

/// Rule: each competition record the season broke becomes one news item for the competition's region; then the
/// flags are cleared.
///
/// `Comp_Stats::send_record_news()`: with no flag set ("any flag" dword +0x562 = 0): 0. Otherwise, for kinds 1 to 22
/// in order, each set flag (+0x50a + 4 × (kind − 1)): a news item of type 0x3e9 (`NewsItem::NewsItem` on the stack),
/// filled by `comp_stats_write_record_to_news(competition id, all-time records +0x26, kind, item)` and sent to the
/// competition's region (`News::send_to_comp_region` of `G_NEWS` with the competition's record); the flag is cleared.
/// The "any flag" dword is cleared. Returns 1.
/// Original: `Comp_Stats::send_record_news` (0x4a2450, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_send_record_news(stats: usize) -> u32 {
    if read_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG) == 0 {
        return 0;
    }
    for kind in 1..=22usize {
        let flag = stats + comp_stats_record_flag(kind);
        if read_u32(flag) == 0 {
            continue;
        }
        covered!("tel41_3 record news sent");
        let mut item = [0u32; COMP_STATS_NEWS_ITEM_BYTES / 4];
        let n = item.as_mut_ptr() as usize;
        exe::new_newsitem(n, COMP_STATS_RECORD_NEWS_TYPE, 0);
        let record = read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION);
        // tel41_3: the record kind of the news
        comp_stats_write_record_to_news(read_u32(record), stats + comp_stats_obj::ALL_TIME_RECORDS, kind as u32, n);
        exe::news_send_to_comp_region(read_usize(G_NEWS), n, read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION));
        write_u32(flag, 0); // tel41_3: the flag cleared
    }
    write_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG, 0);
    1
}

// ---------- comparers ----------

/// The size of the `Comp_Stats` object.
const COMP_STATS_OBJECT_SIZE: usize = 0x5ce;

/// Comparer for `comp_stats_update_season_end_records`: side by side over the whole object, both tables (the sorts),
/// the history table (the most titles sort) and the sort context; the tally block of the most titles is each side's
/// own and the friendly check only reads.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_update_season_end_records(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_COMP_STATS_UPDATE_SEASON_END_RECORDS);
    let regions = [
        (stats, COMP_STATS_OBJECT_SIZE),
        (read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT) as usize * COMP_HISTORY_ROW_SIZE),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats), || comp_stats_update_season_end_records(stats));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::update_season_end_records({:#x}): differences in {:?}", stats, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_archive_season_and_reset`: it appends to the history file and frees, so only the original
/// runs; the plan, made before, is checked after it: the season records as `comp_season_records_clear` leaves them
/// (cleared on a copy beforehand), the saved runs one per club entry (as `Comp_Stats::save_team_runs` leaves the count), both tables freed and 1 returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_archive_season_and_reset(stats: usize, archive: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
    let mut cleared = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    core::ptr::copy_nonoverlapping((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.as_mut_ptr(), cleared.len());
    comp_season_records_clear(cleared.as_mut_ptr() as usize);
    let teams = read_u32(stats + comp_stats_obj::CLUB_COUNT);
    let clubs = if teams != 0 {
        teams
    } else if read_usize(stats + comp_stats_runs::SAVED) != 0 {
        0
    } else {
        read_u32(stats + comp_stats_runs::SAVED_COUNT)
    };
    let r = o(stats, archive);
    let records = core::slice::from_raw_parts((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.len());
    let ok = r == 1 && records == &cleared[..] && read_u32(stats + comp_stats_runs::SAVED_COUNT) == clubs
        && read_usize(stats + comp_stats_obj::CLUBS) == 0 && read_usize(stats + comp_stats_obj::PLAYERS) == 0;
    SCOREBOARD.record(ok, || format!("Comp_Stats::archive_season_and_reset({:#x}, {}): out of plan (return {}, saved runs {} for {} clubs)", stats, archive, r, read_u32(stats + comp_stats_runs::SAVED_COUNT), clubs));
    r
}

/// Comparer for `comp_stats_send_record_news`: it sends news, so only the original runs; the plan is checked after it:
/// with a flag set it returns 1 and clears the 22 flags and the "any flag" dword; without, 0 and nothing changes.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_send_record_news(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_COMP_STATS_SEND_RECORD_NEWS);
    let any = read_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG) != 0;
    let mut before = [0u8; 0x5c];
    core::ptr::copy_nonoverlapping((stats + comp_stats_season_end::RECORD_FLAGS) as *const u8, before.as_mut_ptr(), before.len());
    let r = o(stats);
    let after = core::slice::from_raw_parts((stats + comp_stats_season_end::RECORD_FLAGS) as *const u8, before.len());
    let ok = if any { r == 1 && after.iter().all(|&b| b == 0) } else { r == 0 && after == &before[..] };
    SCOREBOARD.record(ok, || format!("Comp_Stats::send_record_news({:#x}): out of plan (return {}, flags {:x?})", stats, r, after));
    r
}

// ---------- tel41_4: the competition statistics object: building it, destroying it and a club's projected positions (comp_screens.cpp) ----------

/// The sub-record constructors of the all-time records (thiscall, ret 0; each writes only inside its sub-record), by
/// their offset in the records block: the four match records (`FixedFixture::FixedFixture`), the two runs
/// (`run_record_construct`), the two other runs, the three tallies, the goals-in-match record, the rating record, the
/// discipline record and the two age records.
const COMP_STATS_ALL_TIME_CONSTRUCTORS: [(usize, usize); 16] = [
    (0x29, 0x645800), (0x4c, 0x645800), (0x75, 0x645800), (0x94, 0x645800),
    (0xb3, 0x448670), (0xcb, 0x448670), (0xe3, 0x49f2a0), (0xfb, 0x49f2f0),
    (0x113, 0x450c60), (0x132, 0x49f340), (0x16a, 0x450c60), (0x189, 0x453c50),
    (0x1ac, 0x450c60), (0x1cb, 0x450d30), (0x1eb, 0x49f390), (0x22a, 0x49f3f0),
];
/// `comp_season_records_construct` (0x49f160, thiscall, ret 0): the constructors of a season records block.
const COMP_STATS_SEASON_RECORDS_CONSTRUCT: usize = 0x49f160;
/// How many `Comp_Stats` objects are alive; the first opens the shared history file, the last closes it.
const COMP_STATS_INSTANCES: usize = 0xadae18;
/// `FILE_LLIST::FILE_LLIST` (0x58e280, thiscall(list, path, record size), ret 8) and `FILE_LLIST::~FILE_LLIST`
/// (0x58e330, thiscall, ret 0).
const FILE_LLIST_CONSTRUCT: usize = 0x58e280;
const FILE_LLIST_DESTRUCT: usize = 0x58e330;
/// "CM3_TEMP" and "comp records history.tmp", and the record size of the history file (a season row).
const COMP_STATS_TEMP_DIR: usize = 0x987da0;
const COMP_STATS_HISTORY_FILE: usize = 0x99cac8;
/// The `Comp_Stats` fields the constructor clears besides the tables: the dword block +0x506..+0x562 (the 22 record
/// flags at +0x50a among them), the "any flag" dword, the season end count and the block +0x56a..+0x5ce (0x64
/// bytes; saved since data version 0x3e008).
const COMP_STATS_FLAGS_BLOCK: usize = 0x506;
const COMP_STATS_FLAGS_BLOCK_SIZE: usize = 0x5c;
const COMP_STATS_ANY_FLAG: usize = 0x562;
const COMP_STATS_SEASONS_ENDED: usize = 0x500;
const COMP_STATS_LATE_BLOCK: usize = 0x56a;
const COMP_STATS_LATE_BLOCK_SIZE: usize = 0x64;

/// Rule: a new competition statistics object starts with no tables and every record empty; the first object alive
/// opens the shared file of the archived seasons.
///
/// `Comp_Stats::Comp_Stats(comp)`: the sub-records of the all-time records (+0x26) and the season records block
/// (+0x28f) are constructed (the compiler's member constructors, called by address), the lent projection entry
/// (+0x566) is 0. The instance count (`COMP_STATS_INSTANCES`) goes up; when it becomes 1, a list object of 8 bytes
/// (`crt_operator_new`) is built as the file list "CM3_TEMP\comp records history.tmp" (`utils_build_file_path`) with
/// 0x26b-byte rows (`FILE_LLIST::FILE_LLIST`) into `COMP_STATS_ARCHIVE_LIST` (0 when the allocation failed; then the
/// comp_stats.CPP 89 box, `game_fatal_cleanup` and `exit(-1)`). Both sort bytes 0xfe, the block +0x56a..+0x5ce, the
/// tables (+0xc..+0x14, +0x19..+0x21) and the saved runs (+0, +4, +8) zeroed; both record sets cleared
/// (`comp_season_records_clear`); the archive keys +0x4f8 and +0x4fc -1; the competition at +0x502; the season end
/// count (word +0x500) 0; the block +0x506..+0x562 and the "any flag" dword +0x562 zeroed; then
/// `Comp_Stats::update_most_titles_record` counts the titles already in the history table. Returns the object.
/// Original: `Comp_Stats::Comp_Stats` (0x49ee70, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_construct(stats: usize, comp: usize) -> usize {
    for (offset, constructor) in COMP_STATS_ALL_TIME_CONSTRUCTORS {
        let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(constructor);
        f(stats + comp_stats_obj::ALL_TIME_RECORDS + offset);
    }
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(COMP_STATS_SEASON_RECORDS_CONSTRUCT);
    f(stats + comp_stats_obj::SEASON_RECORDS);
    write_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT, 0);
    let alive = read_i32(COMP_STATS_INSTANCES).wrapping_add(1);
    write_i32(COMP_STATS_INSTANCES, alive);
    if alive == 1 {
        covered!("tel41_4 history file opened");
        let block = exe::crt_operator_new(8);
        let list = if block != 0 {
            let path = exe::utils_build_file_path(COMP_STATS_TEMP_DIR, COMP_STATS_HISTORY_FILE);
            let build: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FILE_LLIST_CONSTRUCT);
            build(block, path, SEASON_RECORDS_ROW as u32)
        } else {
            0
        };
        write_usize(COMP_STATS_ARCHIVE_LIST, list);
        if list == 0 {
            comp_stats_fatal(0x59);
        }
    }
    write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    core::ptr::write_bytes((stats + COMP_STATS_LATE_BLOCK) as *mut u8, 0, COMP_STATS_LATE_BLOCK_SIZE);
    for o in [comp_stats_obj::CLUBS, comp_stats_obj::CLUB_CAPACITY, comp_stats_obj::CLUB_COUNT, comp_stats_obj::PLAYERS,
              comp_stats_obj::PLAYER_CAPACITY, comp_stats_obj::PLAYER_COUNT, comp_stats_runs::SAVED, comp_stats_runs::SAVED_CAPACITY,
              comp_stats_runs::SAVED_COUNT] {
        write_u32(stats + o, 0);
    }
    comp_season_records_clear(stats + comp_stats_obj::ALL_TIME_RECORDS);
    comp_season_records_clear(stats + comp_stats_obj::SEASON_RECORDS);
    write_i32(stats + comp_stats_obj::ARCHIVE_KEY1, -1); // tel41_4: no archived season yet
    write_i32(stats + comp_stats_obj::ARCHIVE_KEY2, -1);
    write_usize(stats + comp_stats_obj::COMP, comp);
    write_u16(stats + COMP_STATS_SEASONS_ENDED, 0);
    core::ptr::write_bytes((stats + COMP_STATS_FLAGS_BLOCK) as *mut u8, 0, COMP_STATS_FLAGS_BLOCK_SIZE);
    write_u32(stats + COMP_STATS_ANY_FLAG, 0);
    // tel41_4: the titles already won count from the start
    comp_stats_update_most_titles_record(stats);
    stats
}

/// The lent projection entry (+0x566, see `get_club_projected_positions`).
const COMP_STATS_PROJECTED_ENTRY_SLOT: usize = 0x566;

/// Rule: destroying a competition statistics object frees its tables and saved runs; the last object alive closes the
/// shared file of the archived seasons.
///
/// `Comp_Stats::~Comp_Stats()`: the club and player tables freed (`crt_free`; pointer, capacity and count 0, sort byte
/// 0xfe); when this is the only object alive (instance count 1) and the file list exists, it is destroyed
/// (`FILE_LLIST::~FILE_LLIST`, `crt_operator_delete`) and `COMP_STATS_ARCHIVE_LIST` set to 0; the saved runs array
/// (+0) freed and its pointer cleared (capacity and count stay); the instance count goes down. The member destructors
/// that follow are empty (0x5a53c0).
/// Original: `Comp_Stats::~Comp_Stats` (0x49f450, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_destruct(stats: usize) {
    if read_usize(stats + comp_stats_obj::CLUBS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::CLUBS));
        write_usize(stats + comp_stats_obj::CLUBS, 0);
        write_u32(stats + comp_stats_obj::CLUB_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::CLUB_COUNT, 0);
        write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    }
    if read_usize(stats + comp_stats_obj::PLAYERS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::PLAYERS));
        write_usize(stats + comp_stats_obj::PLAYERS, 0);
        write_u32(stats + comp_stats_obj::PLAYER_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::PLAYER_COUNT, 0);
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    }
    let list = read_usize(COMP_STATS_ARCHIVE_LIST);
    if read_i32(COMP_STATS_INSTANCES) == 1 && list != 0 {
        covered!("tel41_4 history file closed");
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FILE_LLIST_DESTRUCT);
        destroy(list);
        exe::crt_operator_delete(list);
        write_usize(COMP_STATS_ARCHIVE_LIST, 0);
    }
    if read_usize(stats + comp_stats_runs::SAVED) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_runs::SAVED));
        write_usize(stats + comp_stats_runs::SAVED, 0);
    }
    write_i32(COMP_STATS_INSTANCES, read_i32(COMP_STATS_INSTANCES).wrapping_sub(1)); // tel41_4: one instance fewer
}

/// The two competition virtuals of the projection: slot 0x80, whether the competition (or stage) holds the club
/// [inference: read from this caller], and slot 0x84, `League::update_projected_positions` (cpr_11), which writes the
/// projected positions into the club's statistics entry (here, the lent one).
const COMP_VT_HOLDS_CLUB: usize = 0x80;
const COMP_VT_UPDATE_PROJECTED_POSITIONS: usize = 0x84;
/// `Date::Date` (0x54a110, thiscall, ret 0): the default date (day 0x28, year 0x7b4, 1).
const COMP_STATS_DATE_CONSTRUCT: usize = 0x54a110;

/// Rule: a club's projected final, best and worst positions come from the last stage of its competition that holds
/// it, which projects them into a scratch entry; when no stage holds it or the projection fails, the positions are
/// those of its statistics entry, if any.
///
/// `Comp_Stats::get_club_projected_positions(club, &best, &current, &worst)`: from the competition's current stage
/// index (signed byte +0x30) down to -1 (the competition itself; below -1 at the start: straight to the fallback),
/// the first stage (`STAGES[index]`, or the competition for -1) whose virtual 0x80 answers nonzero for the club. Then
/// a 0x76-byte scratch entry (`crt_operator_new`; its eight run dates constructed, `Date::Date`; 0 when the allocation
/// failed) is lent in +0x566 and the stage's virtual 0x84 (`update_projected_positions`) runs for the club: nonzero,
/// the entry's +0x74, +0x73 and +0x75 go to `best`, `current` and `worst`, the entry is freed (`crt_operator_delete`),
/// +0x566 = 0 and 1 is returned; zero, the entry is freed and +0x566 = 0. Fallback (no stage, or the projection
/// failed): the club's entry (`comp_stats_find_club_entry`), when there is one, gives the same three bytes; returns 0.
/// Original: `Comp_Stats::get_club_projected_positions` (0x4ae300, comp_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_club_projected_positions(stats: usize, club: usize, best: *mut u8, current: *mut u8, worst: *mut u8) -> u32 {
    let comp = read_usize(stats + comp_stats_obj::COMP);
    let mut index = read_u8(comp + league::CURRENT_STAGE) as i8;
    let mut stage = 0;
    while index >= -1 {
        let s = if index == -1 { read_usize(stats + comp_stats_obj::COMP) }
                else { read_usize(read_usize(read_usize(stats + comp_stats_obj::COMP) + league::STAGES) + index as isize as usize * 4) };
        let holds: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(s) + COMP_VT_HOLDS_CLUB));
        if holds(s, club) != 0 {
            stage = s;
            break;
        }
        index = index.wrapping_sub(1);
    }
    if stage != 0 {
        let entry = exe::crt_operator_new(comp_stats_obj::CLUB_ENTRY_SIZE as u32);
        if entry != 0 {
            let date: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(COMP_STATS_DATE_CONSTRUCT);
            for run in [comp_stats_club::WINNING_RUN, comp_stats_club::LOSING_RUN, comp_stats_club::UNBEATEN_RUN, comp_stats_club::WINLESS_RUN] {
                date(entry + run + 4);
                date(entry + run + 0xc);
            }
        }
        write_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT, entry);
        let project: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(stage) + COMP_VT_UPDATE_PROJECTED_POSITIONS));
        let done = project(stage, club) != 0;
        if done {
            covered!("tel41_4 positions projected");
            // the original reads the lent slot again for each byte (a failed allocation leaves 0 there)
            *best = read_u8(read_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT) + comp_stats_club::BEST_POSITION); // tel41_4: projected best
            *current = read_u8(read_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT) + comp_stats_club::CURRENT_POSITION);
            *worst = read_u8(read_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT) + comp_stats_club::WORST_POSITION);
        }
        let lent = read_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT);
        if lent != 0 {
            exe::crt_operator_delete(lent);
        }
        write_usize(stats + COMP_STATS_PROJECTED_ENTRY_SLOT, 0);
        if done {
            return 1;
        }
    }
    let entry = exe::comp_stats_find_club_entry(stats, club);
    if entry != 0 {
        // tel41_4: the fallback positions of the club's own entry
        *best = read_u8(entry + comp_stats_club::BEST_POSITION); // tel41_4: fallback best
        *current = read_u8(entry + comp_stats_club::CURRENT_POSITION);
        *worst = read_u8(entry + comp_stats_club::WORST_POSITION);
    }
    0
}

// ---------- comparers ----------

/// Comparer for `comp_stats_construct`: it counts instances and may open the shared file, so only the original runs;
/// the plan is checked after it: the tables and saved runs zeroed, both sort bytes 0xfe, both record sets as
/// `comp_season_records_clear` leaves them except the most titles (byte and club, which the titles count rewrites),
/// the keys -1, the competition, the counters and blocks zeroed, one more instance and the file list open.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_construct(stats: usize, comp: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COMP_STATS_CONSTRUCT);
    let alive = read_i32(COMP_STATS_INSTANCES);
    let r = o(stats, comp);
    let mut cleared = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    core::ptr::copy_nonoverlapping((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.as_mut_ptr(), cleared.len());
    comp_season_records_clear(cleared.as_mut_ptr() as usize);
    let season_ok = core::slice::from_raw_parts((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.len()) == &cleared[..];
    let mut all_time = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    core::ptr::copy_nonoverlapping((stats + comp_stats_obj::ALL_TIME_RECORDS) as *const u8, all_time.as_mut_ptr(), all_time.len());
    let mut all_time_cleared = all_time;
    comp_season_records_clear(all_time_cleared.as_mut_ptr() as usize);
    all_time_cleared[..5].copy_from_slice(&all_time[..5]); // the most titles (byte, club) of the count
    let zero = |o: usize, n: usize| core::slice::from_raw_parts((stats + o) as *const u8, n).iter().all(|&b| b == 0);
    let ok = r == stats && season_ok && all_time == all_time_cleared
        && zero(0, 0xc) && zero(comp_stats_obj::CLUBS, 0xc) && zero(comp_stats_obj::PLAYERS, 0xc)
        && read_u8(stats + comp_stats_obj::CLUB_SORT) == 0xfe && read_u8(stats + comp_stats_obj::PLAYER_SORT) == 0xfe
        && read_i32(stats + comp_stats_obj::ARCHIVE_KEY1) == -1 && read_i32(stats + comp_stats_obj::ARCHIVE_KEY2) == -1
        && read_usize(stats + comp_stats_obj::COMP) == comp && read_u16(stats + COMP_STATS_SEASONS_ENDED) == 0
        && zero(COMP_STATS_FLAGS_BLOCK, COMP_STATS_FLAGS_BLOCK_SIZE + 8) && zero(COMP_STATS_LATE_BLOCK, COMP_STATS_LATE_BLOCK_SIZE)
        && read_i32(COMP_STATS_INSTANCES) == alive.wrapping_add(1) && read_usize(COMP_STATS_ARCHIVE_LIST) != 0;
    SCOREBOARD.record(ok, || format!("Comp_Stats::Comp_Stats({:#x}, comp {:#x}): out of plan (instances {} -> {})", stats, comp, alive, read_i32(COMP_STATS_INSTANCES)));
    r
}

/// Comparer for `comp_stats_destruct`: it frees, so only the original runs; the plan is checked after it: tables and
/// saved runs pointer zeroed, one instance fewer and, when it was the last one, the file list gone.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_destruct(stats: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_COMP_STATS_DESTRUCT);
    let alive = read_i32(COMP_STATS_INSTANCES);
    o(stats);
    let ok = read_usize(stats + comp_stats_obj::CLUBS) == 0 && read_usize(stats + comp_stats_obj::PLAYERS) == 0
        && read_usize(stats + comp_stats_runs::SAVED) == 0 && read_i32(COMP_STATS_INSTANCES) == alive.wrapping_sub(1)
        && (alive != 1 || read_usize(COMP_STATS_ARCHIVE_LIST) == 0);
    SCOREBOARD.record(ok, || format!("Comp_Stats::~Comp_Stats({:#x}): out of plan (instances {} -> {})", stats, alive, read_i32(COMP_STATS_INSTANCES)));
}

/// Comparer for `comp_stats_get_club_projected_positions`: side by side over the three outputs, the lent entry slot
/// and the return (each side allocates and frees its own scratch entry; the projection only writes into it, cpr_11).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_club_projected_positions(stats: usize, club: usize, best: *mut u8, current: *mut u8, worst: *mut u8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8, *mut u8) -> u32>(IDX_COMP_STATS_GET_CLUB_PROJECTED_POSITIONS);
    let regions = [(best as usize, 1), (current as usize, 1), (worst as usize, 1), (stats + COMP_STATS_PROJECTED_ENTRY_SLOT, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club, best, current, worst),
        || comp_stats_get_club_projected_positions(stats, club, best, current, worst));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::get_club_projected_positions({:#x}, {:#x}): return {} x {}, differences in {:?}", stats, club, ro, rr, first_differences(&mo, &mr)));
    ro
}

// ---------- batch 320: the club history: recording a finished competition's season (club_history.cpp) ----------

/// The club history store as `club_history_record_comp_season` uses it (`G_CLUB_HISTORY`): +0 the league history file
/// list, +4 the cup history file list, +8 the per-club records (16 bytes, the club id the index: +0/+4 the first and
/// last file offsets of the club's league seasons, +8/+0xc those of its cup runs).
pub(crate) mod club_history_season {
    pub(crate) const LEAGUE_LIST: usize = 0x0;
    pub(crate) const CUP_LIST: usize = 0x4;
    pub(crate) const RECORDS: usize = 0x8;
    pub(crate) const RECORD_SIZE: usize = 0x10;
    pub(crate) const LEAGUE_FIRST: usize = 0x0;
    pub(crate) const LEAGUE_LAST: usize = 0x4;
    pub(crate) const CUP_FIRST: usize = 0x8;
    pub(crate) const CUP_LAST: usize = 0xc;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\club_history.cpp" (the error boxes).
    pub(crate) const CLUB_HISTORY_CPP: usize = 0x98b644;
}

/// The season of a finished competition in each club's history, with the file list append and the table sort given
/// from outside (the copy appends and sorts; the comparer's dry run records the appends and sorts).
///
/// See `club_history_record_comp_season`.
unsafe fn club_history_record_comp_season_with(this: usize, comp: usize, sort: &mut dyn FnMut(usize),
                                               add: &mut dyn FnMut(usize, usize, usize, &[u8])) -> u32 {
    use club_history_season as ch;
    use league as l;
    if comp == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xab);
        return 0;
    }
    if read_u8(comp + l::STAGE_INDEX) != 0xff {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xb3);
        return 0;
    }
    let competition = read_usize(comp + l::COMPETITION);
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), competition) != 0 {
        return 0;
    }
    let comp_id = read_u32(competition);
    let year = read_u16(comp + l::YEAR);
    let records = read_usize(this + ch::RECORDS);
    let polish = read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION);
    if exe::comp_util_is_league_division(comp_id as i32) != 0 {
        // a league: the table of the league and of each following stage that is a league
        let mut stage = comp;
        loop {
            let recorded = if read_i16(stage + l::ROUND_COUNT) > 7 && comp_id != polish {
                true
            } else {
                comp_id == polish && read_u8(stage + l::STAGE_INDEX) == 1 // tel42_2: the Polish second stage
            };
            if recorded {
                covered!("tel42_2 league table into the history");
                sort(stage);
                let mut rec = [0u8; 0x15];
                rec[0..4].copy_from_slice(&comp_id.to_le_bytes());
                rec[4..6].copy_from_slice(&year.to_le_bytes());
                for k in 0..read_i16(stage + l::CLUB_COUNT).max(0) as usize {
                    let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
                    let club = records + read_u32(read_usize(row + league_table_row::CLUB)) as usize * ch::RECORD_SIZE;
                    rec[6] = read_u8(row + league_table_row::POSITION); // tel42_2: the final position
                    for (i, b) in rec[7..0x15].iter_mut().enumerate() {
                        *b = read_u8(row + 0xd + i);
                    }
                    if read_u32(read_usize(stage + l::COMPETITION)) == polish {
                        // the Polish second stage counts the 14 games of the first one
                        covered!("tel42_2 Polish games played");
                        rec[0xb] = rec[0xb].wrapping_add(0xe);
                    }
                    add(read_usize(this + ch::LEAGUE_LIST), club + ch::LEAGUE_FIRST, club + ch::LEAGUE_LAST, &rec);
                }
            }
            let index = read_i8(stage + l::STAGE_INDEX) as i32;
            if index >= read_i32(comp + l::CURRENT_STAGE) {
                return 1;
            }
            let next = exe::comp_get_stage(comp, index + 1);
            if read_u8(next + l::COMP_TYPE) != 2 {
                return 1;
            }
            stage = next;
        }
    }
    if read_u32(comp + cup::SEASON_SET_UP) == 0 {
        return 0;
    }
    let marks = exe::crt_calloc(read_u32(G_CLUB_COUNT), 4);
    if marks == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xfd);
        return 0;
    }
    covered!("tel42_2 cup runs into the history");
    let mut rec = [0u8; 0xb];
    rec[0..4].copy_from_slice(&comp_id.to_le_bytes());
    rec[4..6].copy_from_slice(&year.to_le_bytes());
    // one record per club, from the last stage back to the competition itself (-1): a club keeps the furthest one
    let mut add_club = |club: usize, status: u8, label: u16, round: u16| {
        let id = read_u32(club) as usize;
        if read_u32(marks + id * 4) != 0 {
            return;
        }
        rec[6] = status;
        rec[7..9].copy_from_slice(&label.to_le_bytes());
        rec[9..0xb].copy_from_slice(&round.to_le_bytes());
        let r = records + id * ch::RECORD_SIZE;
        add(read_usize(this + ch::CUP_LIST), r + ch::CUP_FIRST, r + ch::CUP_LAST, &rec);
        write_u32(marks + id * 4, 1);
    };
    let mut s = read_i32(comp + cup::CURRENT_STAGE);
    while s >= -1 {
        let stage = exe::comp_get_stage(comp, s);
        if read_u8(stage + l::COMP_TYPE) == 2 {
            covered!("tel42_2 cup group stage");
            for k in 0..read_i16(stage + l::CLUB_COUNT).max(0) as usize {
                let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
                add_club(read_usize(row + league_table_row::CLUB), read_u8(row + league_table_row::STATUS),
                         read_u16(stage + l::STAGE_ID), 0xffff);
            }
        } else {
            let mut r = read_i16(stage + cup::ROUND_COUNT) as i32 - 1;
            while r >= 0 {
                let round = read_usize(stage + cup::ROUND_TABLE) + r as usize * cup_round::SIZE;
                let ties = read_usize(read_usize(stage + cup::TIE_TABLES) + r as usize * 4);
                for t in 0..read_i16(round + cup_round::TIES).max(0) as usize {
                    let tie = ties + t * 0x35;
                    for side in [cup_tie::HOME_TEAM, cup_tie::AWAY_TEAM] {
                        let team = read_i16(tie + side);
                        if team == -1 {
                            continue;
                        }
                        let entry = read_usize(stage + cup::TEAM_TABLE) + team as isize as usize * 6;
                        // tel42_2: the team's outcome and the round it reached
                        add_club(read_usize(entry), read_u8(entry + 5), read_u16(stage + cup::STAGE_LABEL),
                                 read_u16(round + cup_round::ROUND_CODE));
                    }
                }
                r -= 1;
            }
        }
        s -= 1;
    }
    exe::crt_free(marks);
    1
}

/// Adds the finished season of a competition to each club's history.
///
/// `Club_History::record_comp_season(comp)`: no competition: the club_history.cpp 0xab box and 0; a stage instead of
/// the competition itself (`league::STAGE_INDEX` ≠ 0xff): the 0xb3 box and 0; a friendly competition
/// (`Friendly::is_friendly_competition`): 0.
///
/// A league division (`comp_util_is_league_division` of the competition id): from the competition itself, each stage
/// whose table is recorded: with more than 7 rounds (`league::ROUND_COUNT`), except in the Polish first division, where
/// only its stage 1 is. The table is sorted (`League::sort_table`, 0) and each row gives a league record (21 bytes:
/// competition id, the competition's year, the row's position, then the 14 bytes of the totals from row +0xd: played,
/// won, drawn, lost, goals for and against, points; in the Polish first division played + 14) appended to the club's
/// league seasons (`FILE_LLIST::add_node` on the league list with the club's first and last offsets). Then the next
/// stage (`Comp::get_stage`, index + 1) while it exists (index below `league::CURRENT_STAGE`) and is a league
/// (`league::COMP_TYPE` = 2). Returns 1.
///
/// A cup: not set up for the season (`cup::SEASON_SET_UP` = 0): 0. A mark per club (`calloc`; none: the 0xfd box and
/// 0). From the current stage (`cup::CURRENT_STAGE`) back to -1 (the cup itself): in a league stage each table row's
/// club gets (competition id, year, the row's status `league_table_row::STATUS`, the stage id `league::STAGE_ID`,
/// -1); in a knockout stage the rounds from the last back, each tie's home and away team (index ≠ -1, the team table's
/// club) gets (id, year, the team's outcome +5, the stage label `cup::STAGE_LABEL`, the round code
/// `cup_round::ROUND_CODE`). Only the first record of each club (the furthest it got) goes to its cup runs (the cup
/// list, the club's +8/+0xc offsets). The marks freed; returns 1.
/// Original: `Club_History::record_comp_season` (0x442e80, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_record_comp_season(this: usize, comp: usize) -> u32 {
    club_history_record_comp_season_with(this, comp, &mut |stage| { exe::league_sort_table(stage, 0usize); },
        &mut |list, first, last, rec| { exe::file_llist_add_node(list, first, last, rec.as_ptr()); })
}

/// Comparer for `club_history_record_comp_season`: the copy plans, the original runs, the plan is checked in the files.
///
/// The records go into the history files (`FILE_LLIST::add_node` appends and moves the club's offsets): the routine
/// cannot run twice. The copy runs first as a dry run: it sorts the tables as the original does (`League::sort_table`
/// is a total order ending on the club id and stores the positions in that order: sorting again changes nothing) and
/// keeps each record it would append, with its club offsets; the original runs afterwards from the same state. Then,
/// for each club offset pair of the plan, the club's nodes are read back from the file (`FILE_LLIST::get_nodes`): the
/// newest ones must be the planned records, in order, and the returns equal. Without a competition or on a stage: only
/// the original (the error boxes).
pub(crate) unsafe extern "thiscall" fn compare_club_history_record_comp_season(this: usize, comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLUB_HISTORY_RECORD_COMP_SEASON);
    if comp == 0 || read_u8(comp + league::STAGE_INDEX) != 0xff {
        return o(this, comp);
    }
    // (list, first offset address, last offset address, record)
    let mut plan: Vec<(usize, usize, usize, Vec<u8>)> = Vec::new();
    let rr = club_history_record_comp_season_with(this, comp, &mut |stage| { exe::league_sort_table(stage, 0usize); },
        &mut |list, first, last, rec| plan.push((list, first, last, rec.to_vec())));
    let ro = o(this, comp);
    let mut wrong: Vec<String> = Vec::new();
    let mut done: Vec<usize> = Vec::new();
    for (list, first, last, _) in &plan {
        if done.contains(first) {
            continue;
        }
        done.push(*first);
        let wanted: Vec<&Vec<u8>> = plan.iter().filter(|p| p.1 == *first).map(|p| &p.3).collect();
        let nodes = exe::file_llist_get_nodes(*list, read_u32(*first), read_u32(*last));
        let mut got: Vec<Vec<u8>> = Vec::new();
        if nodes != 0 {
            write_usize(nodes + llist::CURSOR, read_usize(nodes + llist::TAIL));
            while got.len() < wanted.len() {
                let d = exe::llist_get_prev(nodes);
                if d == 0 {
                    break;
                }
                got.push((0..wanted[0].len()).map(|i| read_u8(d + i)).collect());
            }
            exe::llist_scalar_deleting_destructor(nodes, 1);
        }
        got.reverse();
        if got.len() != wanted.len() || got.iter().zip(wanted.iter()).any(|(g, w)| g != *w) {
            wrong.push(format!("club offsets {:#x}: planned {:02x?} in the file {:02x?}", first, wanted, got));
        }
    }
    SCOREBOARD.record(ro == rr && wrong.is_empty(), || format!("club_history_record_comp_season(comp {:#x}) original={} rust={} {} appends planned: {}",
        comp, ro, rr, plan.len(), wrong.join("; ")));
    ro
}
