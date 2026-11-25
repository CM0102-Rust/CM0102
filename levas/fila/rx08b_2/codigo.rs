// ---------- rx08b_2: the club records history list and the season record copy (club_records.cpp) ----------

/// The history list entry: the season year (word) and a whole 0x38b-byte club record set after it; at most 500.
const HISTORY_ENTRY_SIZE: usize = 0x38d;
const HISTORY_MAX: u32 = 0x1f4;

/// The club_records.cpp source path of the history list's error boxes.
const HISTORY_LIST_CPP: usize = 0x98b6a0;

/// The 0x38b-byte club record set copied with the helpers the compiler's own `operator=` uses (the inline copy in
/// `build_history_list`): the two match records with their dwords, the attendance record (+0x54), the five match
/// records from +0x7d, the four runs (+0x118..), the man-of-the-match style records at +0x168, +0x183, +0x1d6 and
/// +0x210, the goals in a match (+0x19e), the rating (+0x1f1), the discipline record (+0x22b), the counts (+0x247,
/// +0x261, +0x2f9), the two age records (+0x27b, +0x2ba), the two transfer fees (+0x313, +0x33c), the two six-byte
/// pairs at +0x365 and +0x36b and the fans' player (+0x371).
unsafe fn club_record_set_copy(this: usize, source: usize) {
    let d = |o: usize| write_u32(this + o, read_u32(source + o));
    let w = |o: usize| write_u16(this + o, read_u16(source + o));
    let b = |o: usize| write_u8(this + o, read_u8(source + o));
    d(0);
    w(4);
    b(6);
    d(7);
    w(0xb);
    b(0xd);
    d(0xe);
    match_record_assign(this + 0x12, source + 0x12);
    d(0x31);
    match_record_assign(this + 0x35, source + 0x35);
    attendance_record_assign(this + 0x54, source + 0x54);
    d(0x77);
    w(0x7b);
    for o in [0x7d, 0x9c, 0xbb, 0xda, 0xf9] {
        match_record_assign(this + o, source + o);
    }
    for o in [0x118, 0x12c, 0x140, 0x154] {
        run_record_assign(this + o, source + o);
    }
    man_of_match_record_assign(this + 0x168, source + 0x168);
    man_of_match_record_assign(this + 0x183, source + 0x183);
    goals_in_match_record_assign(this + 0x19e, source + 0x19e);
    man_of_match_record_assign(this + 0x1d6, source + 0x1d6);
    rating_record_assign(this + 0x1f1, source + 0x1f1);
    man_of_match_record_assign(this + 0x210, source + 0x210);
    discipline_record_assign(this + 0x22b, source + 0x22b);
    count_record_assign(this + 0x247, source + 0x247);
    count_record_assign(this + 0x261, source + 0x261);
    player_age_record_assign(this + 0x27b, source + 0x27b);
    player_age_record_assign(this + 0x2ba, source + 0x2ba);
    count_record_assign(this + 0x2f9, source + 0x2f9);
    transfer_fee_record_assign(this + 0x313, source + 0x313);
    transfer_fee_record_assign(this + 0x33c, source + 0x33c);
    d(0x365);
    w(0x369);
    d(0x36b);
    w(0x36f);
    fans_player_record_assign(this + 0x371, source + 0x371);
}

/// rule: a season's club record set copied whole, field by field
///
/// `this.season_history_entry_assign(source)`: the same 0x38b bytes as `club_record_set_copy`, with the records split
/// into their parts (a byte or word before each record holder, `record_holder_assign`; the dword before each match
/// record, `match_record_assign`; the runs, `run_record_assign`; the age records, `player_age_record_assign`; the
/// transfer fees, `transfer_fee_record_assign`; the last word at +0x389). Returns `this`.
/// Original: `season_history_entry_assign` (0x451200, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn season_history_entry_assign(this: usize, source: usize) -> usize {
    covered!("rx08b_2 season record copy");
    let d = |o: usize| write_u32(this + o, read_u32(source + o));
    let w = |o: usize| write_u16(this + o, read_u16(source + o));
    let b = |o: usize| write_u8(this + o, read_u8(source + o));
    let holder = |o: usize| {
        record_holder_assign(this + o, source + o);
    };
    d(0);
    w(4);
    b(6);
    d(7);
    w(0xb);
    b(0xd);
    d(0xe);
    match_record_assign(this + 0x12, source + 0x12);
    d(0x31);
    match_record_assign(this + 0x35, source + 0x35);
    d(0x54);
    match_record_assign(this + 0x58, source + 0x58);
    d(0x77);
    w(0x7b);
    for o in [0x7d, 0x9c, 0xbb, 0xda, 0xf9] {
        match_record_assign(this + o, source + o);
    }
    for o in [0x118, 0x12c, 0x140, 0x154] {
        run_record_assign(this + o, source + o);
    }
    b(0x168);
    holder(0x169);
    w(0x181);
    b(0x183);
    holder(0x184);
    w(0x19c);
    b(0x19e);
    holder(0x19f);
    match_record_assign(this + 0x1b7, source + 0x1b7);
    b(0x1d6);
    holder(0x1d7);
    w(0x1ef);
    d(0x1f1);
    holder(0x1f5);
    b(0x20d);
    w(0x20e);
    b(0x210);
    holder(0x211);
    w(0x229);
    b(0x22b);
    b(0x22c);
    holder(0x22d);
    w(0x245);
    w(0x247);
    holder(0x249);
    w(0x261);
    holder(0x263);
    player_age_record_assign(this + 0x27b, source + 0x27b);
    player_age_record_assign(this + 0x2ba, source + 0x2ba);
    w(0x2f9);
    holder(0x2fb);
    transfer_fee_record_assign(this + 0x313, source + 0x313);
    transfer_fee_record_assign(this + 0x33c, source + 0x33c);
    d(0x365);
    w(0x369);
    d(0x36b);
    w(0x36f);
    holder(0x371);
    w(0x389);
    this
}

/// rule: a club's record history for the records screen: up to 500 entries of a year and a record set: first the
/// all-time records (year 0), then this season's (its year), then each archived season from the most recent back;
/// a missing set is reset to "none"
///
/// `this.build_history_list(club, count)`: `*count` = 0. No club: the club_records.cpp box of line 0x14c1 and 0. The
/// season and all-time records (`exe::club_records_get_season_record`, `exe::club_records_get_all_time_record`, without
/// creating). The array (`malloc` of 500 × 0x38d); none: the box of line 0x14cd and 0. Entry 0: year 0 and the
/// all-time set (`club_record_set_copy`, or `reset_records` without one). A season set whose rating (`club_record::
/// SEASON_RATING`, f32) is −1 gets its rating record first (`update_season_rating_record`). Entry 1: the club's season
/// year (`exe::key_nation_get_club_season_year`) and the season set (`season_history_entry_assign`, or reset). Then the
/// archived seasons: the history file's nodes for the club (`exe::file_llist_get_nodes` of the history list at +0
/// with the club entry's (`club_records_fields::CLUB_ENTRIES`) dwords +8 and +0xc), from the last (`exe::llist_get_prev`
/// from the tail) while fewer than 500: the node's year and its set (`season_history_entry_assign`). The node list is
/// destroyed and freed. Returns the array (the caller frees it).
/// Original: `Club_Records::build_history_list` (0x450d50, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn build_history_list(this: usize, club: usize, count: usize) -> usize {
    write_u32(count, 0);
    if club == 0 {
        show_error_at(HISTORY_LIST_CPP, 0x14c1);
        return 0;
    }
    let season = exe::club_records_get_season_record(this, club, 0);
    let all_time = exe::club_records_get_all_time_record(this, club, 0);
    let list = exe::crt_malloc(HISTORY_MAX * HISTORY_ENTRY_SIZE as u32);
    if list == 0 {
        show_error_at(HISTORY_LIST_CPP, 0x14cd);
        return 0;
    }
    covered!("rx08b_2 history list");
    let entry = |k: u32| list + k as usize * HISTORY_ENTRY_SIZE;
    write_u16(entry(0), 0);
    if all_time != 0 {
        club_record_set_copy(entry(0) + 2, all_time);
    } else {
        reset_records(entry(0) + 2);
    }
    write_u32(count, 1);
    if season != 0 && read_f32(season + club_record::SEASON_RATING) as f64 == read_f64(rdata::F64_NEG_1) {
        update_season_rating_record(this, club);
    }
    write_u16(entry(1), exe::key_nation_get_club_season_year(club));
    if season != 0 {
        season_history_entry_assign(entry(1) + 2, season);
    } else {
        reset_records(entry(1) + 2);
    }
    write_u32(count, 2);
    let club_entry = read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add((read_i32(club) as usize).wrapping_mul(0x6a));
    let nodes = exe::file_llist_get_nodes(read_usize(this), read_u32(club_entry + 8), read_u32(club_entry + 0xc)); // the club's span in the history file
    if nodes == 0 {
        return list;
    }
    write_u32(nodes + llist::CURSOR, read_u32(nodes + llist::TAIL));
    loop {
        let node = exe::llist_get_prev(nodes);
        let k = read_u32(count);
        if node == 0 || k == HISTORY_MAX {
            break;
        }
        write_u32(count, k + 1);
        write_u16(entry(k), read_u16(node));
        season_history_entry_assign(entry(k) + 2, node + 2);
    }
    exe::drop_llist(nodes);
    exe::crt_operator_delete(nodes);
    list
}

/// Comparer for `season_history_entry_assign`: side by side on the 0x38b bytes it writes and the answer.
pub(crate) unsafe extern "thiscall" fn compare_season_history_entry_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (a, b, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x38b)], || o(this, source), || season_history_entry_assign(this, source));
    SCOREBOARD.record(same && a == b, || format!("season_history_entry_assign (club records screen): {:?}", first_differences(&mo, &mr)));
    a
}

/// Comparer for `build_history_list`: both build their own array (the copy's runs second); the count and the arrays'
/// `count` entries must match and the copy's array is freed (the original's goes back to the caller). A missing club
/// runs only the original (the box).
pub(crate) unsafe extern "thiscall" fn compare_build_history_list(this: usize, club: usize, count: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I0%%);
    if club == 0 {
        return o(this, club, count);
    }
    let a = o(this, club, count);
    let n = read_u32(count);
    let mut n2 = 0u32;
    let b = build_history_list(this, club, &mut n2 as *mut u32 as usize);
    let size = n as usize * HISTORY_ENTRY_SIZE;
    let same = n == n2 && (a == 0) == (b == 0)
        && (a == 0 || core::slice::from_raw_parts(a as *const u8, size) == core::slice::from_raw_parts(b as *const u8, size));
    SCOREBOARD.record(same, || format!("build_history_list (club records screen, club {}): count original {} rust {}", read_i32(club), n, n2));
    if b != 0 {
        exe::crt_free(b);
    }
    a
}
