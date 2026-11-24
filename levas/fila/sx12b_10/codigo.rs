// ---------- batch %%LEVA%%: the injury texts of the player screens (injury.cpp; sx12b_10) ----------

/// The physical record of a person (31 bytes, `injury_fields::PHYSICAL_RECORDS` by id) and its current injury slot
/// (+0xe, -1 = none).
unsafe fn injury_slot_of(this: usize, person: usize) -> i32 {
    read_i32(read_usize(this + injury_fields::PHYSICAL_RECORDS) + read_u32(person) as usize * 31 + 0xe)
}

/// `Injury::update_player_fitness` (thiscall(this, person), ret 4), called through the exe so that its own detour runs.
unsafe fn update_fitness(this: usize, person: usize) {
    let update: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_INJURY_UPDATE_PLAYER_FITNESS);
    update(this, person);
}

/// Returns the list of a person's past injuries from the injury history file (`FILE_LLIST::get_nodes` of the history
/// list with the first node and count kept in his physical record, +0x14 and +0x18), or 0 when there is no history
/// list. A null person: the "Error" box (injury.cpp line 0xd88) and 0.
/// Original: `Injury::get_player_injury_history` (0x62ec20, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_player_injury_history(this: usize, person: usize) -> usize {
    if person == 0 {
        show_error_at(0x9e7f28, 0xd88); // "...\injury.cpp"
        return 0;
    }
    let history = read_usize(this + injury_fields::INJURY_HISTORY);
    if history == 0 {
        return 0;
    }
    let record = read_usize(this + injury_fields::PHYSICAL_RECORDS) + read_u32(person) as usize * 31;
    covered!("sx12b_10 injury history");
    exe::file_llist_get_nodes(history, read_u32(record + 0x14), read_u32(record + 0x18)) // the history's first node and count
}

/// Writes the name of a person's current injury into `out` (`injury_get_type_name` of the injury's type, with `size`
/// and `article`), or the translated "None" when he has none; returns what the writer returns. Without the daily
/// fitness update (see `describe_player_injury`).
unsafe fn write_player_injury_name(this: usize, person: usize, out: usize, size: i32, article: u32) -> i32 {
    let slot = injury_slot_of(this, person);
    if slot == -1 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        return text(out, 0x9e7fd4, 0xad9c64usize); // "None<%s - COMMENT - no injury>"
    }
    let kind = read_u8(read_usize(this + injury_fields::INJURY_ENTRIES) + slot as usize * 0x30); // sx12b_10: the injury type
    let name: unsafe extern "thiscall" fn(usize, u32, usize, i32, u32) -> i32 = core::mem::transmute(FN_INJURY_GET_TYPE_NAME);
    name(this, kind as u32, out, size, article)
}

/// Writes the name of a person's current injury, or "None", into `out`, after his daily fitness update.
///
/// `Injury::describe_player_injury(person, out, size, article)`: a null person: the "Error" box (line 0xe63). A person
/// id beyond the staff table (`G_STAFF_COUNT`, a human manager row): nothing, returning the staff count. Otherwise
/// `Injury::update_player_fitness` and `write_player_injury_name`.
/// Original: `Injury::describe_player_injury` (0x62ee30, injury.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn describe_player_injury(this: usize, person: usize, out: usize, size: i32, article: u32) -> i32 {
    if person == 0 {
        show_error_at(0x9e7f28, 0xe63);
        return 0;
    }
    if read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return read_i32(G_STAFF_COUNT); // eax as the comparison left it: the staff count
    }
    update_fitness(this, person);
    write_player_injury_name(this, person, out, size, article)
}

/// Writes the time left of a person's injury as text: the days until full fitness (`full`, `injury_entry::DAYS_OUT`)
/// or until he trains again (`injury_entry::TRAINING_RETURN_DAYS`), minus the days since the injury, by
/// `format_duration_text`; without a daily fitness update.
unsafe fn write_days_out(this: usize, slot: i32, out: usize, size: u32, full: i8, style: u32) -> u32 {
    let e = read_usize(this + injury_fields::INJURY_ENTRIES) + slot as usize * 0x30;
    let mut gap = [0u8; 8];
    exe::date_get_difference(G_CURRENT_DATE, gap.as_mut_ptr(), read_u32(e + injury_entry::DATE), read_u32(e + injury_entry::DATE_LEAP_YEAR));
    let elapsed = read_u16(gap.as_ptr() as usize + 2) as i32 * 365 + read_i16(gap.as_ptr() as usize) as i32; // years and days
    let total = if full != 0 { read_i16(e + injury_entry::DAYS_OUT) } else { read_i16(e + injury_entry::TRAINING_RETURN_DAYS) } as i32;
    let format: unsafe extern "C" fn(i32, usize, u32, u32) -> u32 = core::mem::transmute(FN_FORMAT_DURATION_TEXT);
    format(total - elapsed, out, size, style) // sx12b_10: the days left
}

/// Writes the time until a person returns to training (or, with `full`, to full fitness) as text; with no injury the
/// buffer is emptied and 0 returned.
///
/// `Injury::format_days_out(person, out, size, full, style)`: a null person: the "Error" box (line 0x106d) and 0; an id
/// beyond the staff table: 0. Otherwise the daily fitness update (`Injury::update_player_fitness`); no current injury:
/// `out` emptied, 0; otherwise `write_days_out`.
/// Original: `Injury::format_days_out` (0x630af0, injury.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn format_days_out(this: usize, person: usize, out: usize, size: u32, full: i8, style: u32) -> u32 {
    if person == 0 {
        show_error_at(0x9e7f28, 0x106d);
        return 0;
    }
    if read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    update_fitness(this, person);
    let slot = injury_slot_of(this, person);
    if slot == -1 {
        write_u8(out, 0);
        return 0;
    }
    covered!("sx12b_10 days out");
    write_days_out(this, slot, out, size, full, style)
}

/// Writes into a Text the line saying in which match a player got injured: "Injured in <club>'s <score> win over /
/// defeat against / draw with <club>." (the score with the higher number first; the clubs' gender bytes for the
/// translation), the links to the two clubs, the competition's name in brackets when `comp_is_reserve_or_minor_comp`
/// answers yes for it, and a link from the score to the match when the match statistics keep it
/// (`Match_Stats::fixture_entry_matches`).
///
/// `injury_write_match_injury_text(record, text)`: the match record of an injury (clubs at +0x12 and +0x16, their goals
/// at +0x1a and +0x1b, the competition at +9, the date at +1/+3, the match at +0x1c).
/// Original: `injury_write_match_injury_text` (0x631dc0, injury.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn injury_write_match_injury_text(record: usize, text: usize) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (a, b) = (club_at(read_i32(record + 0x12) as isize), club_at(read_i32(record + 0x16) as isize));
    let (ga, gb) = (read_i8(record + 0x1a) as i32, read_i8(record + 0x1b) as i32);
    let mut score = [0u8; 0xc];
    let (high, low, line) = if ga > gb {
        (ga, gb, 0x9e9cfc) // "... win over ..."
    } else if ga < gb {
        (gb, ga, 0x9e9c84) // "... defeat against ..."
    } else {
        (ga, gb, 0x9e9c10) // "... draw with ..." sx12b_10: the result word
    };
    sprintf(score.as_mut_ptr() as usize, 0x9e9d6c, high, low); // "<%d - Team Goals 1>-<%d - Team Goals 2>"
    let (gender_a, gender_b) = (read_i8(a.0 + 0x52) as i32, read_i8(b.0 + 0x52) as i32); // the short names' gender
    let (name_a, name_b) = (a.0 + 0x38, b.0 + 0x38); // the short names
    sprintf(0xde1f64, line, gender_a, gender_a, gender_b, gender_b, name_a, score.as_ptr(), name_b);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, name_a as *const u8, 0x7d5, a.0);
    exe::text_add_link(text, name_b as *const u8, 0x7d5, b.0);
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32(record + 9) as usize).wrapping_mul(0x6b));
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) != 0 {
        sprintf(0xde1f64, 0x9e9bf4, comp + 0x38); // " (<%s - Competition Name>)"
        exe::text_append_string(text, 0xde1f64usize);
    }
    if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(record + 0x1c), read_i32(record + 0x12),
        read_i32(record + 0x16), read_u16(record + 1) as i32, read_u16(record + 3) as i32) != 0 {
        exe::text_add_link(text, score.as_ptr(), 0x7d3, read_u32(record + 0x1c) as usize);
    }
    covered!("sx12b_10 match injury line");
}

/// Comparer for `get_player_injury_history`: only the original (the history list it returns is a new allocation).
pub(crate) unsafe extern "thiscall" fn compare_get_player_injury_history(this: usize, person: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, person)
}

/// Comparer for `describe_player_injury`: the original runs (with the daily fitness update); then the copy writes the
/// name without the update into a buffer of its own; the texts and the returns are compared. A null person or a human
/// manager row: only the original.
pub(crate) unsafe extern "thiscall" fn compare_describe_player_injury(this: usize, person: usize, out: usize, size: i32, article: u32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> i32>(%%I1%%)(this, person, out, size, article);
    if person == 0 || out == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return r;
    }
    let mut copy = [0u8; 0x200];
    let n = write_player_injury_name(this, person, copy.as_mut_ptr() as usize, size, article);
    let original = core::ffi::CStr::from_ptr(out as *const core::ffi::c_char);
    let rust = core::ffi::CStr::from_ptr(copy.as_ptr() as *const core::ffi::c_char);
    SCOREBOARD.record(r == n && original == rust, || format!("Injury::describe_player_injury (player screens) original {:?} {} rust {:?} {}", original, r, rust, n));
    r
}

/// Comparer for `format_days_out`: as `compare_describe_player_injury` (the copy without the fitness update, into a
/// buffer of its own).
pub(crate) unsafe extern "thiscall" fn compare_format_days_out(this: usize, person: usize, out: usize, size: u32, full: i8, style: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i8, u32) -> u32>(%%I2%%)(this, person, out, size, full, style);
    if person == 0 || out == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return r;
    }
    let mut copy = [0u8; 0x200];
    let slot = injury_slot_of(this, person);
    let n = if slot == -1 { 0 } else { write_days_out(this, slot, copy.as_mut_ptr() as usize, size.min(0x200), full, style) };
    let original = core::ffi::CStr::from_ptr(out as *const core::ffi::c_char);
    let rust = core::ffi::CStr::from_ptr(copy.as_ptr() as *const core::ffi::c_char);
    SCOREBOARD.record(r == n && original == rust, || format!("Injury::format_days_out(full {}) (compare players screen) original {:?} rust {:?}", full, original, rust));
    r
}

/// Comparer for `injury_write_match_injury_text`: the Text side by side (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_injury_write_match_injury_text(record: usize, text: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize)>(%%I3%%);
    compare_text_writer(text, || o(record, text), |t| injury_write_match_injury_text(record, t),
        || "injury_write_match_injury_text (compare players screen)".to_string());
}
