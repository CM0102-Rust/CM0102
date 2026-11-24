// ---------- cx01_15: the competition texts and lists of the screens (comp.cpp, comp_util.cpp, Cup.cpp) ----------

/// The two clubs whose honours `compare_club_honours` ranks (set by `comp_util_collect_club_honours`).
const HONOURS_CLUB: usize = 0xae0ff8;
const HONOURS_SECOND_CLUB: usize = 0xae0ffc;
/// The second shared text buffer (after 0xde1f64), used by the fixture list order.
const SECOND_TEXT_BUFFER: usize = 0xde234c;
/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\comp.cpp" and "...\comp_util.cpp", for the error boxes.
const COMP_SOURCE_FILE: usize = 0x99b80c;
const COMP_UTIL_SOURCE_FILE: usize = 0x99ec20;

/// Writes a club's progress in a competition into `out` (of `size` bytes): a stage that belongs to a group set
/// (`exe::comp_util_get_parent_stage_id` not −1) gives "Qualified From Groups" (status 0, 6, 7 or 8; gendered by the
/// competition's long name) or "Group Stage"; otherwise status 1 gives "Winners"; with no stage nor round, status 2
/// "Runners Up" and 3 "Third Placed"; anything else the stage and round name (`exe::comp_util_format_stage_round_name`).
/// A buffer under 30 bytes shows the comp.cpp error box (line 0x1d5). The texts go through `FN_LANGLIB_SPRINTF`.
/// Original: `Comp::write_progress_text` (0x48cab0, comp.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn comp_write_progress_text(_this: usize, out: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) {
    if size < 0x1e {
        show_error_at(COMP_SOURCE_FILE, 0x1d5);
        return;
    }
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (stage, round, status) = (stage as u16 as i16, round as u16 as i16, status as u8);
    if exe::comp_util_get_parent_stage_id(stage as i32) != -1 {
        if matches!(status, 0 | 6 | 7 | 8) {
            let gender = Competition(comp).gender_name() as i8 as i32;
            lang_sprintf(out, 0x99b7d8, gender, 0xad9c64usize); // "Qualified {}<%s - Ignore>From Groups"
        } else {
            lang_sprintf(out, 0x99b800); // "Group Stage"
        }
        return;
    }
    let fixed = match status {
        1 => Some(0x99b7d0usize), // "Winners"
        2 if stage == 0 && round == 0 => Some(0x99b7c4), // "Runners Up"
        3 if stage == 0 && round == 0 => Some(0x99b7b4), // "Third Placed"
        _ => None,
    };
    match fixed {
        Some(text) => {
            lang_sprintf(out, text);
        }
        None => {
            exe::comp_util_format_stage_round_name(stage as i32, round as i32, out, size as u32, 0);
        }
    }
}

/// Writes a short label for a club into `out` and returns `out`: first an empty text; then, in the friendly competition
/// itself (the friendly object's competition record) or, elsewhere, in its main competition's stage `stage` (−1 the
/// main one; outside −1..`CURRENT_STAGE` the comp.cpp error box, line 0x607), a league stage gives the club's table
/// position as an ordinal (`exe::utils_format_ordinal`; a club not found gives the club count, an empty table 0); a club
/// competition (`CLUB_SIDES` 1 of this one) gives its division's three-letter name, a national one (2) its nation's.
/// Original: `Comp::get_club_label_text` (0x48dfa0, comp.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_get_club_label_text(this: usize, out: usize, club: usize, stage: i32) -> usize {
    let comp = League(this);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    sprintf(out, 0xad9c64); // the empty text
    let friendly = read_usize(G_FRIENDLY);
    let owner = if friendly != 0 && comp.competition().addr() == read_usize(friendly + 4) {
        comp
    } else {
        let main = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp.competition().addr()) as usize * 4));
        if stage < -1 || stage > main.current_stage() as i32 {
            show_error_at(COMP_SOURCE_FILE, 0x607);
            return out;
        }
        if stage > -1 { League(read_usize(main.stages() + 4 * stage as usize)) } else { main }
    };
    let copy = |text: usize| core::ptr::copy_nonoverlapping(text as *const u8, out as *mut u8, c_string_bytes(text).len() + 1);
    if owner.comp_type() == 2 {
        let mut position = 0i32;
        let mut k = 0i32;
        while k < owner.club_count() as i32 {
            position = k + 1;
            if read_usize(exe::league_get_table_row(owner, k)) == club {
                break;
            }
            k += 1;
        }
        covered!("cx01_15 club label: league position");
        copy(exe::utils_format_ordinal(position, -1));
        return out;
    }
    match comp.club_sides() {
        1 => {
            let division = Club(club).division();
            if !division.is_null() {
                copy(division.addr() + 0x53); // the division's three-letter name [inference]
            }
        }
        2 => {
            let nation = Club(club).nation();
            if !nation.is_null() {
                copy(nation.addr() + nation::THREE_LETTER_NAME);
            }
        }
        _ => {}
    }
    out
}

/// Records a club's projected progress at the cup's stage through `comp_vtable::SET_CLUB_PROGRESS` (the club, the
/// stage index, 1, 1 and the low byte of the team count) and returns 1: the cup form of the projected positions slot.
/// Original: `Cup::update_projected_positions` (0x522c50, Cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_update_projected_positions(this: usize, club: usize) -> u32 {
    let cup = Cup(this);
    let progress: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
    // simplification: the original pushes the two bytes in a register with stale high bytes; the slot reads the bytes
    progress(this, club, cup.stage_index() as u32, 1, 1, cup.team_count() as u8 as u32);
    1
}

/// Writes a fixture's competition, stage and round label into `buf` (`size` bytes) in `form`, returning what
/// `comp_util_format_comp_stage_round_label` returns: no fixture, no buffer or under 0x67 bytes shows the comp_util.cpp
/// error box (line 0x2587) and 0; a fixture without a competition the box of line 0x258f and 0.
/// Original: `comp_util_format_fixture_round_label` (0x4bf930, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_format_fixture_round_label(fixture: usize, form: u32, buf: usize, size: i32) -> u32 {
    if fixture == 0 || buf == 0 || size < 0x67 {
        show_error_at(COMP_UTIL_SOURCE_FILE, 0x2587);
        return 0;
    }
    let f = Fixture(fixture);
    if f.competition().is_null() {
        show_error_at(COMP_UTIL_SOURCE_FILE, 0x258f);
        return 0;
    }
    covered!("cx01_15 fixture round label");
    // simplification: the original pushes the prefix and the name with stale high halves; the label reads the words
    comp_util_format_comp_stage_round_label(f.competition().addr(), f.round_prefix() as u16 as u32, f.round_name() as u16 as u32, form, buf, size)
}

/// Whether a history row names `club` (or, when given, `other`) as champion, runner-up or third.
unsafe fn honours_row_names(row: usize, club: usize, other: usize) -> bool {
    let places = [comp_history_row::CHAMPION, comp_history_row::RUNNER_UP, comp_history_row::THIRD_PLACE].map(|p| read_usize(row + p));
    places.contains(&club) || (other != 0 && places.contains(&other))
}

/// Copies every competition history row (`G_CLUB_COMP_HISTORY_TABLE`, 0x1a bytes) in which `club` or `other` finishes
/// first, second or third into a new array (grown by 10 rows with `exe::crt_realloc`; failing: the comp_util.cpp error
/// box, line 0x2703, and 0 with `*out` left null), sorts it (`exe::crt_qsort` with `compare_club_honours`, the two clubs
/// set in `HONOURS_CLUB`/`HONOURS_SECOND_CLUB`) and returns the row count, the array in `*out`.
/// Original: `comp_util_collect_club_honours` (0x4c0140, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_collect_club_honours(club: usize, other: usize, out: usize) -> i32 {
    write_usize(out, 0);
    let (mut rows, mut n, mut capacity) = (0usize, 0usize, 0usize);
    for i in 0..read_i32(G_CLUB_COMP_HISTORY_COUNT).max(0) as usize {
        let row = read_usize(G_CLUB_COMP_HISTORY_TABLE) + i * 0x1a;
        if read_usize(row + 4) == 0 || !honours_row_names(row, club, other) {
            continue;
        }
        if n == capacity {
            capacity += 10;
            rows = exe::crt_realloc(rows, (capacity * 0x1a) as u32);
            if rows == 0 {
                show_error_at(COMP_UTIL_SOURCE_FILE, 0x2703);
                return 0;
            }
        }
        core::ptr::copy_nonoverlapping(row as *const u8, (rows + n * 0x1a) as *mut u8, 0x1a);
        n += 1;
    }
    covered!("cx01_15 club honours");
    write_usize(HONOURS_CLUB, club);
    write_usize(HONOURS_SECOND_CLUB, other);
    exe::crt_qsort(rows, n as u32, 0x1a, 0x4c02f0usize); // compare_club_honours
    write_usize(out, rows);
    n as i32
}

/// Whether a competition record counts as a top league for the honours order: the Spanish and French lower divisions
/// and the CFA always; otherwise a competition with a nation whose object is a league with clubs and a table whose
/// first club has it as its division or reserve division.
unsafe fn honours_comp_is_league(comp: Competition) -> bool {
    let id = read_u32(comp.addr());
    if [KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION].iter().any(|&k| read_u32(k) == id) {
        return true;
    }
    if read_usize(read_usize(G_CLUB_COMP_TABLE) + id as usize * 0x6b + competition::NATION) == 0 {
        return false;
    }
    let object = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4));
    if object.is_null() || object.comp_type() != 2 || object.club_count() == 0 || object.table() == 0 {
        return false;
    }
    let first = Club(read_usize(object.table()));
    first.division() == object.competition() || first.reserve_division() == object.competition()
}

/// The finishing place of the honours clubs in a history row: 1 champion, 2 runner-up, 3 otherwise.
unsafe fn honours_place(row: usize) -> i8 {
    let (club, other) = (read_usize(HONOURS_CLUB), read_usize(HONOURS_SECOND_CLUB));
    let is = |at: usize| { let c = read_usize(row + at); c == club || (other != 0 && c == other) };
    if is(comp_history_row::CHAMPION) {
        1
    } else if is(comp_history_row::RUNNER_UP) {
        2
    } else {
        3
    }
}

/// qsort order of two history rows for the club honours list: continental competitions (a continent) before the
/// others and national ones (a nation) before the rest; between two national ones, a competition that is not a top
/// league (`honours_comp_is_league`) before a league; then the higher competition reputation, the higher competition
/// id, the better finishing place of the honours clubs (`honours_place`) and the earlier season (+8).
/// Original: `compare_club_honours` (0x4c02f0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_club_honours(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (ka, kb) = (Competition(read_usize(a + 4)), Competition(read_usize(b + 4)));
    let (ca, cb) = (ka.continent(), kb.continent());
    let (na, nb) = (ka.nation().addr(), kb.nation().addr());
    if (ca == 0 && cb != 0) || (na == 0 && nb != 0) {
        return -1;
    }
    if (cb == 0 && ca != 0) || (nb == 0 && na != 0) {
        return 1;
    }
    if nb != 0 && na != 0 {
        match (honours_comp_is_league(ka), honours_comp_is_league(kb)) {
            (false, true) => return -1,
            (true, false) => return 1,
            _ => {}
        }
    }
    let by_reputation = kb.reputation() as i32 - ka.reputation() as i32;
    if by_reputation != 0 {
        return by_reputation;
    }
    let by_id = read_u32(ka.addr()).wrapping_sub(read_u32(kb.addr())) as i32;
    if by_id != 0 {
        return by_id;
    }
    let (pa, pb) = (honours_place(a), honours_place(b));
    if pa == pb {
        covered!("cx01_15 honours by season");
        return read_i16(a + 8) as i32 - read_i16(b + 8) as i32;
    }
    if pa < pb { -1 } else { 1 }
}

/// Sorts a linked list of 0x4f-byte fixture records (LLIST): the records copied into an array (`exe::crt_malloc`; the
/// cursor reset to the head and `exe::llist_get_next`), sorted (`exe::crt_qsort` with
/// `compare_fixtures_by_round_and_home_name`), the list cleared (`exe::llist_clear`) and rebuilt in order
/// (`exe::llist_add_tail`), the array freed; 1. No list (line 0x2782), an empty one (0x278d) or no memory (0x2796) shows
/// the comp_util.cpp error box and 0.
/// Original: `comp_util_sort_fixture_list` (0x4c0520, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_sort_fixture_list(list: usize) -> u32 {
    if list == 0 {
        show_error_at(COMP_UTIL_SOURCE_FILE, 0x2782);
        return 0;
    }
    let n = read_i16(list + llist::COUNT) as i32;
    if n == 0 {
        show_error_at(COMP_UTIL_SOURCE_FILE, 0x278d);
        return 0;
    }
    let array = exe::crt_malloc((n * 0x4f) as u32);
    if array == 0 {
        show_error_at(COMP_UTIL_SOURCE_FILE, 0x2796);
        return 0;
    }
    covered!("cx01_15 fixture list sorted");
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    for k in 0..n.max(0) as usize {
        core::ptr::copy_nonoverlapping(exe::llist_get_next(list) as *const u8, (array + k * 0x4f) as *mut u8, 0x4f);
    }
    exe::crt_qsort(array, n as u32, 0x4f, 0x4c06c0usize); // compare_fixtures_by_round_and_home_name
    exe::llist_clear(list);
    for k in 0..n.max(0) as usize {
        exe::llist_add_tail(list, array + k * 0x4f, 0x4f);
    }
    exe::crt_free(array);
    1
}

/// qsort order of two fixture records: by the stage index byte, the round prefix, the round number and the round name
/// (each as a signed difference), then the home clubs' short names (`exe::utils_string_compare` with 1, 0).
/// Original: `compare_fixtures_by_round_and_home_name` (0x4c06c0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_fixtures_by_round_and_home_name(a: *const usize, b: *const usize) -> i32 {
    let (fa, fb) = (Fixture(a as usize), Fixture(b as usize));
    for d in [fa.stage_index() as i8 as i32 - fb.stage_index() as i8 as i32, fa.round_prefix() as i32 - fb.round_prefix() as i32,
        fa.round_number() as i32 - fb.round_number() as i32, fa.round_name() as i32 - fb.round_name() as i32] {
        if d != 0 {
            return d;
        }
    }
    exe::utils_string_compare((fa.home_club().addr() + club::SHORT_NAME) as *const u8, (fb.home_club().addr() + club::SHORT_NAME) as *const u8, 1, 0)
}

/// One 20-byte entry of the match date list: +0 the stage, +4 the round, +8 the competition id, +0xc the day, +0xe the
/// year, +0x10 the stage label (the schedule's extra word) and +0x12 the round code.
unsafe fn write_match_date_entry(list: usize, n: &mut usize, comp: League, stage: i32, round: i32, date: usize, label: i16, code: i16) {
    let e = list + *n * 0x14;
    write_i32(e, stage);
    write_i32(e + 4, round);
    write_u32(e + 8, read_u32(comp.competition().addr()));
    write_u16(e + 0xc, read_u16(date));
    write_u16(e + 0xe, read_u16(date + 2));
    write_i16(e + 0x10, label);
    write_i16(e + 0x12, code);
    *n += 1;
}

/// The extra match days of a round: up to eight 7-byte slots from `slots` (a slot type 0xff ends them); a slot of type
/// 3 moves the round date to its weekday (`exe::date_get_nearest_weekday` with the slot's signed byte −2) and, unless
/// that is the same day with the same time slice as the round (`time_slice`), gives an entry with `code`.
unsafe fn add_round_slot_dates(list: usize, n: &mut usize, comp: League, stage: i32, round: i32, date: usize, slots: usize, time_slice: u8, label: i16, code: i16) {
    let mut moved = [0u8; 8];
    exe::date_date_54a110(moved.as_mut_ptr() as usize);
    for k in 0..8 {
        let slot = slots + 7 * k;
        match read_u8(slot) {
            0xff => break,
            3 => {
                // simplification: the original pushes the weekday word with stale high bytes; the date routine reads the word
                exe::date_get_nearest_weekday(date, read_u8(slot - 2) as i8 as i16 as u16 as u32, moved.as_mut_ptr() as usize);
                let m = moved.as_ptr() as usize;
                if read_u16(m) == read_u16(date) && read_u16(m + 2) == read_u16(date + 2) && read_u8(slot - 1) == time_slice {
                    continue;
                }
                write_match_date_entry(list, n, comp, stage, round, m, label, code);
            }
            _ => {}
        }
    }
}

/// Builds a date-sorted list of every match day of every stage of the competition and returns it (null when there is
/// none, `*count` then left at 1000).
///
/// `Comp::build_match_date_list(&count)` (thiscall, ret 4; SEH frame only for the dates). `*count` = 1000 and a list
/// of 1000 20-byte entries (`exe::crt_malloc`; none: the comp.cpp error box, line 0x3dd, and null). For each stage from
/// −1 below `STAGE_COUNT`, its schedule (`comp_vtable::CREATE_SCHEDULE`): a league schedule (0x41-byte rounds) gives,
/// per round, the round's slot dates (`add_round_slot_dates` from +0xb, code 0) and its own date (day +0, year +2,
/// relative to `YEAR`); a cup schedule (0x68-byte rounds) gives, per round, the slot dates of a round with the 0x100
/// tie rule (from +0x29, with the round code), then for a two-legged round its date with the code + 3 and the date
/// +0x22 days later with the code + 4, or else its date with the code. The schedule is freed. With entries, `*count` =
/// their number, the list shrunk to them (`exe::crt_realloc`; failing: the box of line 0x497 and null) and sorted
/// (`exe::crt_qsort` with `comp_compare_fixture_list_entries`).
/// Defect of the original: the list holds 1000 entries and is not bounds checked.
/// Original: `Comp::build_match_date_list` (0x48d500, comp.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_build_match_date_list(this: usize, count: usize) -> usize {
    let comp = League(this);
    write_u32(count, 1000);
    let list = exe::crt_malloc(1000 * 0x14);
    if list == 0 {
        show_error_at(COMP_SOURCE_FILE, 0x3dd);
        return 0;
    }
    let mut n = 0usize;
    let mut stage = -1i32;
    while stage < comp.stage_count() as i32 {
        let schedule: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, *mut i32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
        let (mut rounds, mut label, mut league) = (0i16, 0i16, 0i32);
        let s = schedule(this, stage, &mut rounds, &mut label, &mut league);
        if s != 0 {
            for r in 0..rounds.max(0) as usize {
                let mut date = [0u8; 8];
                let d = date.as_mut_ptr() as usize;
                if league != 0 {
                    let rec = s + r * 0x41;
                    exe::new_date(d, read_u16(rec) as u32, read_u16(rec + 2).wrapping_add(comp.year() as u16) as u32);
                    add_round_slot_dates(list, &mut n, comp, stage, r as i32, d, rec + 0xb, read_u8(rec + 4), label, 0);
                    write_match_date_entry(list, &mut n, comp, stage, r as i32, d, label, 0);
                } else {
                    let round = CupRound(s + r * cup_round::SIZE);
                    exe::new_date(d, round.day() as u16 as u32, (round.year_offset() as i8 as i16).wrapping_add(comp.year()) as u16 as u32);
                    if round.tie_rules() as u16 & 0x100 != 0 {
                        add_round_slot_dates(list, &mut n, comp, stage, r as i32, d, round.addr() + 0x29, round.time_slice(), label, round.round_code());
                    }
                    if round.legs() == 2 {
                        write_match_date_entry(list, &mut n, comp, stage, r as i32, d, label, round.round_code().wrapping_add(3));
                        let (mut later, mut scratch, mut second) = ([0u8; 8], [0u8; 8], [0u8; 8]);
                        exe::date_date_54a110(second.as_mut_ptr() as usize);
                        let l = exe::date_add_days(d, later.as_mut_ptr() as usize, round.leg_gap_days() as i8 as i16 as u16 as u32);
                        exe::date_operator_3d(second.as_mut_ptr() as usize, scratch.as_mut_ptr() as usize, l);
                        write_match_date_entry(list, &mut n, comp, stage, r as i32, second.as_ptr() as usize, label, round.round_code().wrapping_add(4));
                    } else {
                        write_match_date_entry(list, &mut n, comp, stage, r as i32, d, label, round.round_code());
                    }
                }
            }
            exe::crt_free(s);
        }
        stage += 1;
    }
    if n == 0 {
        return 0;
    }
    covered!("cx01_15 match date list");
    write_u32(count, n as u32);
    let list = exe::crt_realloc(list, (n * 0x14) as u32);
    if list == 0 {
        show_error_at(COMP_SOURCE_FILE, 0x497);
        return 0;
    }
    exe::crt_qsort(list, n as u32, 0x14, 0x48da90usize); // comp_compare_fixture_list_entries
    list
}

/// qsort order of two match date entries: the earlier year, then day; then the competition of higher reputation; then
/// the stage and round labels (form 1, written into the two shared text buffers by
/// `exe::comp_util_format_comp_stage_round_label`), as an unsigned byte comparison (−1, 0 or 1).
/// Original: `comp_compare_fixture_list_entries` (0x48da90, comp.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_compare_fixture_list_entries(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    for at in [0xe, 0xc] {
        let (x, y) = (read_i16(a + at), read_i16(b + at));
        if x != y {
            return if x > y { 1 } else { -1 };
        }
    }
    let reputation = |e: usize| read_i16(read_usize(G_CLUB_COMP_TABLE) + read_u32(e + 8) as usize * 0x6b + competition::REPUTATION) as i32;
    let by_reputation = reputation(b) - reputation(a);
    if by_reputation != 0 {
        return by_reputation;
    }
    // simplification: the original pushes the label words with stale high halves; the label reads the words
    exe::comp_util_format_comp_stage_round_label(0usize, read_u16(a + 0x10) as u32, read_u16(a + 0x12) as u32, 1, 0xde1f64 as *mut u8, 1000);
    exe::comp_util_format_comp_stage_round_label(0usize, read_u16(b + 0x10) as u32, read_u16(b + 0x12) as u32, 1, SECOND_TEXT_BUFFER as *mut u8, 1000);
    match c_string_bytes(0xde1f64).cmp(&c_string_bytes(SECOND_TEXT_BUFFER)) {
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Less => -1,
        core::cmp::Ordering::Greater => 1,
    }
}

/// Finds the last fixture already played between the two clubs of a cup tie: from the current date and slice
/// (`G_CURRENT_DATE`, `G_CURRENT_DAY_SLICE`) backwards (`find_next_fixture` with this competition, the round as the
/// round filter and the two clubs), a fixture still unplayed (`HOME_GOALS` 0xff) moves the cursor back one slice (from
/// slice 0 to slice 2 of the day before, `exe::date_sub_days`) and searches again. Returns the fixture or null; a
/// negative round (line 0x3b9) or tie (line 0x3c1) shows the Cup.cpp error box and null.
/// Original: `Cup::find_scheduled_fixture_for_tie` (0x519d90, Cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn cup_find_scheduled_fixture_for_tie(this: usize, round: i32, tie: i32) -> usize {
    if round < 0 {
        show_error_at(CUP_SOURCE, 0x3b9);
        return 0;
    }
    if tie < 0 {
        show_error_at(CUP_SOURCE, 0x3c1);
        return 0;
    }
    let cup = Cup(this);
    let t = read_usize(cup.tie_tables() + 4 * round as usize) + tie as usize * 0x35;
    let home = read_usize(cup.team_table() + read_i16(t) as isize as usize * 6);
    let away = read_usize(cup.team_table() + read_i16(t + 2) as isize as usize * 6);
    let comp_id = read_i32(League(this).competition().addr());
    let (mut year, mut day, mut slice) = (read_u16(G_CURRENT_DATE_YEAR), read_u16(G_CURRENT_DATE), read_u8(G_CURRENT_DAY_SLICE));
    loop {
        let f = find_next_fixture(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slice, 0, comp_id, round, read_i32(home), read_i32(away), u32::MAX, u32::MAX, u32::MAX);
        if f == 0 || Fixture(f).home_goals() as u8 != 0xff {
            return f;
        }
        covered!("cx01_15 tie fixture: one slice back");
        if slice == 0 {
            let (mut date, mut earlier, mut scratch) = ([0u8; 8], [0u8; 8], [0u8; 8]);
            exe::new_date(date.as_mut_ptr() as usize, day as u32, year as u32);
            let e = exe::date_sub_days(date.as_mut_ptr() as usize, earlier.as_mut_ptr() as usize, 1);
            exe::date_operator_3d(date.as_mut_ptr() as usize, scratch.as_mut_ptr() as usize, e);
            day = read_u16(date.as_ptr() as usize);
            year = read_u16(date.as_ptr() as usize + 2);
            slice = 2;
        } else {
            slice -= 1;
        }
    }
}

/// Comparer for `comp_write_progress_text`: the original into `out` and the copy into a local buffer of the same size;
/// the texts must match (a buffer under 30 bytes, the error box: only the original).
pub(crate) unsafe extern "thiscall" fn compare_comp_write_progress_text(this: usize, out: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize)>(%%I0%%);
    if size < 0x1e {
        return o(this, out, size, stage, round, status, comp);
    }
    let mut mine = vec![0u8; size as usize];
    comp_write_progress_text(this, mine.as_mut_ptr() as usize, size, stage, round, status, comp);
    o(this, out, size, stage, round, status, comp);
    let (a, b) = (c_string_bytes(out), c_string_bytes(mine.as_ptr() as usize));
    SCOREBOARD.record(a == b, || format!("comp_write_progress_text(stage {}, round {}, status {}): original {:?} rust {:?}", stage as i16, round as i16, status as u8,
        String::from_utf8_lossy(&a), String::from_utf8_lossy(&b)));
}

/// Comparer for `comp_get_club_label_text`: the original into `out` and the copy into a local buffer; the texts and the
/// returns (each its own buffer) must match. A stage out of range (the error box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_comp_get_club_label_text(this: usize, out: usize, club: usize, stage: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32) -> usize>(%%I1%%);
    let mut mine = [0u8; 0x100];
    let main = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(League(this).competition().addr()) as usize * 4));
    if !main.is_null() && (stage < -1 || stage > main.current_stage() as i32) {
        return o(this, out, club, stage);
    }
    let n = comp_get_club_label_text(this, mine.as_mut_ptr() as usize, club, stage);
    let r = o(this, out, club, stage);
    let (a, b) = (c_string_bytes(out), c_string_bytes(mine.as_ptr() as usize));
    SCOREBOARD.record(r == out && n == mine.as_ptr() as usize && a == b, || format!("comp_get_club_label_text({:#x}, club {:#x}, stage {}): original {:?} rust {:?}", this, club, stage,
        String::from_utf8_lossy(&a), String::from_utf8_lossy(&b)));
    r
}

/// Comparer for `cup_update_projected_positions`: the three position bytes of the club's entry in the statistics of
/// the competition that keeps them, side by side, and the return (`compare_update_projected_positions`'s method).
pub(crate) unsafe extern "thiscall" fn compare_cup_update_projected_positions(this: usize, club: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club), || cup_update_projected_positions(this, club));
    SCOREBOARD.record(same && r == n, || format!("cup_update_projected_positions({:#x}, club {:#x}): original {} {:x?} rust {} {:x?}", this, club, r, mo, n, mr));
    r
}

/// Comparer for `comp_util_format_fixture_round_label`: the original into `buf` and the copy into a local buffer of the
/// same size; the texts and the returns must match (the error boxes: only the original).
pub(crate) unsafe extern "C" fn compare_comp_util_format_fixture_round_label(fixture: usize, form: u32, buf: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, i32) -> u32>(%%I3%%);
    if fixture == 0 || buf == 0 || size < 0x67 || Fixture(fixture).competition().is_null() {
        return o(fixture, form, buf, size);
    }
    let mut mine = vec![0u8; size as usize];
    core::ptr::copy_nonoverlapping(buf as *const u8, mine.as_mut_ptr(), size as usize);
    let n = comp_util_format_fixture_round_label(fixture, form, mine.as_mut_ptr() as usize, size);
    let r = o(fixture, form, buf, size);
    let (a, b) = (c_string_bytes(buf), c_string_bytes(mine.as_ptr() as usize));
    SCOREBOARD.record(r == n && a == b, || format!("comp_util_format_fixture_round_label({:#x}, form {}): original {} {:?} rust {} {:?}", fixture, form, r,
        String::from_utf8_lossy(&a), n, String::from_utf8_lossy(&b)));
    r
}

/// Comparer for `comp_util_collect_club_honours`: the original's array and the copy's (freed afterwards) must hold the
/// same rows in the same order; the count must match.
pub(crate) unsafe extern "C" fn compare_comp_util_collect_club_honours(club: usize, other: usize, out: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> i32>(%%I4%%)(club, other, out);
    let theirs = read_usize(out);
    let mut mine = 0usize;
    let n = comp_util_collect_club_honours(club, other, &mut mine as *mut usize as usize);
    let rows = |p: usize, k: i32| if p != 0 && k > 0 { core::slice::from_raw_parts(p as *const u8, k as usize * 0x1a).to_vec() } else { Vec::new() };
    let same = r == n && rows(theirs, r) == rows(mine, n);
    if mine != 0 {
        exe::crt_free(mine);
    }
    SCOREBOARD.record(same, || format!("comp_util_collect_club_honours({:#x}, {:#x}): original {} rows, rust {}", club, other, r, n));
    r
}

compare_order!(compare_compare_club_honours, compare_club_honours, %%I5%%);

/// Comparer for `comp_util_sort_fixture_list`: it rebuilds the list (cannot run twice); only the original runs, and the
/// list must then be in the copy's order (`compare_fixtures_by_round_and_home_name` never above 0 between neighbours).
pub(crate) unsafe extern "C" fn compare_comp_util_sort_fixture_list(list: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I6%%)(list);
    if r != 1 {
        return r;
    }
    let mut node = read_usize(list + llist::HEAD);
    let mut ordered = true;
    while node != 0 && read_usize(node + llist_node::NEXT) != 0 {
        let next = read_usize(node + llist_node::NEXT);
        let (a, b) = (read_usize(node + llist_node::DATA), read_usize(next + llist_node::DATA));
        ordered &= compare_fixtures_by_round_and_home_name(a as *const usize, b as *const usize) <= 0;
        node = next;
    }
    SCOREBOARD.record(ordered, || format!("comp_util_sort_fixture_list({:#x}): the list is not in order", list));
    r
}

compare_order!(compare_compare_fixtures_by_round_and_home_name, compare_fixtures_by_round_and_home_name, %%I7%%);

/// Comparer for `comp_build_match_date_list`: the original's list and the copy's (freed afterwards) must hold the same
/// entries and counts.
pub(crate) unsafe extern "thiscall" fn compare_comp_build_match_date_list(this: usize, count: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I8%%)(this, count);
    let theirs = read_u32(count);
    let mut mine_count = 0u32;
    let n = comp_build_match_date_list(this, &mut mine_count as *mut u32 as usize);
    let entries = |p: usize, k: u32| if p != 0 { core::slice::from_raw_parts(p as *const u8, k as usize * 0x14).to_vec() } else { Vec::new() };
    let same = (r == 0) == (n == 0) && theirs == mine_count && entries(r, theirs) == entries(n, mine_count);
    if n != 0 {
        exe::crt_free(n);
    }
    SCOREBOARD.record(same, || format!("comp_build_match_date_list({:#x}): original {} entries, rust {}", this, theirs, mine_count));
    r
}

compare_order!(compare_comp_compare_fixture_list_entries, comp_compare_fixture_list_entries, %%I9%%);

/// Comparer for `cup_find_scheduled_fixture_for_tie` (the fixture found; a negative round or tie, the error box: only
/// the original).
pub(crate) unsafe extern "thiscall" fn compare_cup_find_scheduled_fixture_for_tie(this: usize, round: i32, tie: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> usize>(%%I10%%);
    if round < 0 || tie < 0 {
        return o(this, round, tie);
    }
    let r = o(this, round, tie);
    let n = cup_find_scheduled_fixture_for_tie(this, round, tie);
    SCOREBOARD.record(r == n, || format!("cup_find_scheduled_fixture_for_tie({:#x}, round {}, tie {}): original {:#x} rust {:#x}", this, round, tie, r, n));
    r
}
