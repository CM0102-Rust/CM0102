// --- sx17_1: PLAYER_SEARCH::PLAYER_SEARCH (0x7ae390), PLAYER_SEARCH::save (0x7b2360) and PLAYER_SEARCH::load (0x7b17e0), player_search.cpp ---

/// A field of the PLAYER_SEARCH record as the saved game stores it: offset, element size, element count and the
/// player_search.cpp "Error" box line of a short write (`player_search_save`) or read (`player_search_load`).
type PlayerSearchSaveField = (usize, u32, u32, u32);

/// The size of the scouted club record (+0x170): the club id and 50 scouted player slots of 0x1a2 bytes.
const PLAYER_SEARCH_CLUB_RECORD_SIZE: u32 = 0x51a8;

/// Builds an empty player search: the dates (+0x184, the scouting start and end, +0x1b6, the competition date) and the
/// two filter records constructed, then the owner −1, the slot −1, the saved file magic +5 = 0x24a9e and format version
/// +9 = 1 (`player_search_read_file_header` checks both), the list type 4, +0xb 0, the manager name (+0x70) and password (+0x139) empty, +0x16c the game's random seed, no scout, no
/// assignment (nation, club, region, competition, target club −1; base and opponent flags 0), the scouting start
/// 27/04/1969 [inference: the date constructor's arguments are day 27, month 4, year 1969], no reports, no results
/// (array, capacity and count 0), the report mark 1, +0x2a8 and +0x2ac 0 and no scouted club record. Returns `this`.
/// Original: `PLAYER_SEARCH::PLAYER_SEARCH` (0x7ae390, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn new_player_search(this: usize) -> usize {
    for date in [0x184, player_search_fields::SCOUTING_START_DATE, player_search_fields::SCOUTING_END_DATE, 0x1b6, SCOUT_DAY_COMPETITION_DATE] {
        exe::date_date_54a110(this + date);
    }
    finder_call0(FINDER_FILTERS_SET_DEFAULTS, this + 0x1fa); // the SearchFilters record (0x99 bytes)
    search_filter_record_init(this + 0x293);
    write_i32(this + SCOUT_DAY_OWNER, -1);
    write_i8(this + SCOUT_DAY_SLOT, -1);
    write_u32(this + 5, 0x24a9e); // the saved file magic
    write_u8(this + 9, 1); // the saved file format version
    write_u8(this + player_search_fields::LIST_TYPE, 4); // sx17_1: the default list type
    write_u8(this + 0xb, 0);
    write_u8(this + 0x70, 0); // the manager's name (201 bytes)
    write_u8(this + 0x139, 0); // the manager's password
    write_u32(this + 0x16c, read_u32(G_GAME_RANDOM_SEED));
    write_i32(this + player_search_fields::SCOUT_ID, -1);
    write_i32(this + player_search_fields::ASSIGNED_NATION, -1);
    write_i8(this + player_search_fields::ASSIGNED_REGION, -1);
    write_i32(this + player_search_fields::ASSIGNED_CLUB, -1);
    write_i32(this + player_search_fields::ASSIGNED_COMP, -1);
    write_u32(this + player_search_fields::ASSIGNED_BASE, 0);
    write_u32(this + player_search_fields::ASSIGNED_OPPONENT, 0);
    write_i32(this + SCOUT_DAY_TARGET_CLUB, -1);
    let mut date = [0u16; 4];
    let start = exe::date_date_549f70(&mut date, 0x1b, 4, 0x7b1, u32::MAX);
    let mut copy = [0u8; 8];
    exe::date_operator_3d(this + player_search_fields::SCOUTING_START_DATE, copy.as_mut_ptr() as usize, start);
    write_u8(this + SCOUT_DAY_REPORTS_DONE, 0);
    write_usize(this + player_search_fields::RESULTS, 0);
    write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
    write_u32(this + player_search_fields::RESULT_COUNT, 0);
    write_u32(this + SCOUT_DAY_REPORT_MARK, 1);
    write_u32(this + 0x2a8, 0);
    write_u32(this + 0x2ac, 0);
    write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, 0);
    this
}

/// Comparer for `new_player_search`: side by side over the whole record (0x2b0 bytes) and the return; the bytes the
/// constructor does not write keep what was there on both sides.
pub(crate) unsafe extern "thiscall" fn compare_new_player_search(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x2b0)], || o(this), || new_player_search(this));
    SCOREBOARD.record(same && r == n, || format!("new_player_search({:#x}) {:?}", this, first_differences(&mo, &mr)));
    r
}

/// The fields `player_search_save` writes before the result list, in order.
const PLAYER_SEARCH_SAVED_HEAD: [PlayerSearchSaveField; 17] = [
    (SCOUT_DAY_OWNER, 4, 1, 0x90c), (SCOUT_DAY_SLOT, 1, 1, 0x913), (5, 0x16b, 1, 0x91a),
    (player_search_fields::SCOUT_NAME_REF, 0x18, 1, 0x921), (player_search_fields::ASSIGNED_CLUB, 4, 1, 0x928),
    (player_search_fields::ASSIGNED_NATION, 4, 1, 0x92f), (player_search_fields::ASSIGNED_REGION, 1, 1, 0x936),
    (player_search_fields::ASSIGNED_COMP, 4, 1, 0x93d), (player_search_fields::ASSIGNED_BASE, 4, 1, 0x944),
    (player_search_fields::ASSIGNED_OPPONENT, 4, 1, 0x94b), (SCOUT_DAY_TARGET_CLUB, 4, 1, 0x953),
    (player_search_fields::SCOUTING_END_DATE, 8, 1, 0x95a), (player_search_fields::SCOUTING_START_DATE, 8, 1, 0x961),
    (SCOUT_DAY_REPORTS_DONE, 1, 1, 0x968), (0x1b6, 8, 1, 0x96f), (SCOUT_DAY_COMPETITION_DATE, 8, 1, 0x976),
    (0x1c6, 4, 10, 0x97d), // ten dwords
];

/// The fields `player_search_save` writes after the result list: the SearchFilters record, the filter record, the
/// report mark, +0x2a8 and +0x2ac.
const PLAYER_SEARCH_SAVED_TAIL: [PlayerSearchSaveField; 5] = [
    (0x1fa, 0x99, 1, 0x9a0), (0x293, 0x11, 1, 0x9a7), (SCOUT_DAY_REPORT_MARK, 4, 1, 0x9ae), (0x2a8, 4, 1, 0x9b5),
    (0x2ac, 4, 1, 0x9bc),
];

/// Writes one search into the saved game (`zwrite` to the open item `save`): no item, the player_search.cpp "Error"
/// box (line 0x905) and 0. The fields of `PLAYER_SEARCH_SAVED_HEAD` in order (+5..+0x16f is the header a saved search
/// file also starts with); then the invalid results are dropped
/// (`purge_invalid_results` without relinking), the result capacity and count, the results themselves (0x2d bytes
/// each) when there are any, and the fields of `PLAYER_SEARCH_SAVED_TAIL`. A short write shows the box with the field's
/// line and answers 0. Last, with an assigned club (> −1) and its scouted club record, the record (0x51a8 bytes); a short
/// write there shows the box (line 0x1ffb) but still answers 1.
/// Original: `PLAYER_SEARCH::save` (0x7b2360, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_save(this: usize, save: usize) -> u32 {
    let fail = |line: u32| {
        show_error_box(0xa701b0, line); // player_search.cpp
        0
    };
    if save == 0 {
        return fail(0x905);
    }
    for (offset, size, count, line) in PLAYER_SEARCH_SAVED_HEAD {
        if exe::zwrite(this + offset, size, count, save) != count {
            return fail(line);
        }
    }
    exe::player_search_purge_invalid_results(this, 0);
    if exe::zwrite(this + SCOUT_DAY_RESULT_CAPACITY, 4, 1, save) != 1 {
        return fail(0x987);
    }
    let count = this + player_search_fields::RESULT_COUNT;
    if exe::zwrite(count, 4, 1, save) != 1 {
        return fail(0x98e);
    }
    if read_i32(count) > 0 {
        covered!("sx17_1 save the results");
        if exe::zwrite(read_usize(this + player_search_fields::RESULTS), SCOUT_DAY_RESULT_SIZE as u32, read_u32(count), save) != read_u32(count) {
            return fail(0x998);
        }
    }
    for (offset, size, count, line) in PLAYER_SEARCH_SAVED_TAIL {
        if exe::zwrite(this + offset, size, count, save) != count {
            return fail(line);
        }
    }
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    if read_i32(this + player_search_fields::ASSIGNED_CLUB) > -1 && record != 0 {
        covered!("sx17_1 save the scouted club record");
        if exe::zwrite(record, PLAYER_SEARCH_CLUB_RECORD_SIZE, 1, save) != 1 {
            show_error_box(0xa701b0, 0x1ffb); // player_search.cpp
        }
    }
    1
}

/// Comparer for `player_search_save`: it writes into the save item, so it cannot run twice; only the original runs.
/// The copy is proven by the save proof (the .qst written with and without the batch).
pub(crate) unsafe extern "thiscall" fn compare_player_search_save(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
}

/// True when the absolute game data version (`G_GAME_DATA_VERSION`) is above `version`: the fields added later are
/// only in saves from then on.
unsafe fn player_search_data_newer_than(version: i32) -> bool {
    read_i32(G_GAME_DATA_VERSION).wrapping_abs() > version
}

/// Reads the results of an old save (game data up to 0x3e032), 0x2c bytes each, into the 0x2d-byte entries: the
/// name ref (+0..+0xc), the two dates (`Date::operator=`), the flags +0x20, the ratings +0x21 and +0x25 and the bytes
/// +0x29..+0x2b; the new byte +0x2c is 0xff. The old buffer (`malloc`, not checked) is freed after the conversion;
/// answers false on a short read (the buffer is then left allocated, like the original).
unsafe fn player_search_load_old_results(this: usize, save: usize) -> bool {
    let old = exe::crt_malloc(read_u32(this + SCOUT_DAY_RESULT_CAPACITY).wrapping_mul(0x2c));
    let n = read_u32(this + player_search_fields::RESULT_COUNT);
    if exe::zread(old, 0x2c, n, save) != n {
        return false;
    }
    let mut k = 0usize;
    while (k as i32) < read_i32(this + player_search_fields::RESULT_COUNT) {
        let (from, to) = (old + k * 0x2c, read_usize(this + player_search_fields::RESULTS) + k * SCOUT_DAY_RESULT_SIZE);
        let mut copy = [0u8; 8];
        write_u8(to + 0x29, read_u8(from + 0x29));
        write_u32(to + 0x21, read_u32(from + 0x21));
        exe::date_operator_3d(to + search_result::LAST_UPDATE_DATE, copy.as_mut_ptr() as usize, from + search_result::LAST_UPDATE_DATE);
        write_u8(to + 0x2a, read_u8(from + 0x2a));
        write_u32(to + 0x25, read_u32(from + 0x25));
        for d in [0, 4, 8, 0xc] {
            write_u32(to + d, read_u32(from + d));
        }
        exe::date_operator_3d(to + 0x10, copy.as_mut_ptr() as usize, from + 0x10);
        write_u8(to + 0x2b, read_u8(from + 0x2b));
        write_u8(to + search_result::FLAGS, read_u8(from + search_result::FLAGS));
        write_u8(to + 0x29, read_u8(from + 0x29));
        write_u8(to + 0x2c, 0xff); // the byte old saves did not have
        k += 1;
    }
    exe::crt_free(old);
    true
}

/// Reads one search from the saved game (`zread` from the open item `save`): no item, the player_search.cpp "Error"
/// box (line 0x7fd) and 0. In order: the owner, the slot, +5 (0x16b bytes), the scout's name ref, the assigned club
/// (below −1 becomes −1), nation, region and competition; the base flag only from game data 0x3e011 on, the opponent
/// flag and the target club only from 0x3e03a on; the scouting end, the start (from 0x3e011 on), the reports, +0x1b6,
/// the competition date and the ten dwords at +0x1c6; the result capacity and count. The old result array is freed;
/// with a capacity, a new one (`malloc`, 0x2d bytes each; when it fails: the box, line 0x89f, `game_fatal_cleanup` and
/// `exit(-1)`), and with a count the results: 0x2d bytes each from game data 0x3e033 on, else the old 0x2c-byte
/// entries converted (`player_search_load_old_results`). Then the SearchFilters record, the filter record, the report
/// mark, +0x2a8 and +0x2ac. A short read shows the box with the field's line and answers 0. Last, with an assigned club
/// (> −1): a new scouted club record (`operator new`, 0x51a8 bytes) read from the save and added to the global list of
/// club records (`G_SCOUT_CLUB_TABLE_LIST`); a short read shows the box (line 0x1fc3) and the record is kept but not
/// listed; a failed allocation leaves 0. Answers 1.
/// Original: `PLAYER_SEARCH::load` (0x7b17e0, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_load(this: usize, save: usize) -> u32 {
    let fail = |line: u32| {
        show_error_box(0xa701b0, line); // player_search.cpp
        0
    };
    if save == 0 {
        return fail(0x7fd);
    }
    let read = |offset: usize, size: u32, count: u32| exe::zread(this + offset, size, count, save) == count;
    let head = [(SCOUT_DAY_OWNER, 4, 0x804), (SCOUT_DAY_SLOT, 1, 0x80b), (5, 0x16b, 0x812),
                (player_search_fields::SCOUT_NAME_REF, 0x18, 0x819), (player_search_fields::ASSIGNED_CLUB, 4, 0x820)];
    for (offset, size, line) in head {
        if !read(offset, size, 1) {
            return fail(line);
        }
    }
    if read_i32(this + player_search_fields::ASSIGNED_CLUB) < -1 {
        write_i32(this + player_search_fields::ASSIGNED_CLUB, -1);
    }
    let assignment = [(player_search_fields::ASSIGNED_NATION, 4, 0x82a), (player_search_fields::ASSIGNED_REGION, 1, 0x831),
                      (player_search_fields::ASSIGNED_COMP, 4, 0x838)];
    for (offset, size, line) in assignment {
        if !read(offset, size, 1) {
            return fail(line);
        }
    }
    if player_search_data_newer_than(0x3e010) && !read(player_search_fields::ASSIGNED_BASE, 4, 1) {
        return fail(0x841);
    }
    if player_search_data_newer_than(0x3e039) {
        if !read(player_search_fields::ASSIGNED_OPPONENT, 4, 1) {
            return fail(0x84b);
        }
        if !read(SCOUT_DAY_TARGET_CLUB, 4, 1) {
            return fail(0x851);
        }
    }
    if !read(player_search_fields::SCOUTING_END_DATE, 8, 1) {
        return fail(0x859);
    }
    if player_search_data_newer_than(0x3e010) && !read(player_search_fields::SCOUTING_START_DATE, 8, 1) {
        return fail(0x862);
    }
    let middle = [(SCOUT_DAY_REPORTS_DONE, 1, 1, 0x86a), (0x1b6, 8, 1, 0x871), (SCOUT_DAY_COMPETITION_DATE, 8, 1, 0x878),
                  (0x1c6, 4, 10, 0x87f), (SCOUT_DAY_RESULT_CAPACITY, 4, 1, 0x886), (player_search_fields::RESULT_COUNT, 4, 1, 0x88d)];
    for (offset, size, count, line) in middle {
        if !read(offset, size, count) {
            return fail(line);
        }
    }
    let results = this + player_search_fields::RESULTS;
    if read_usize(results) != 0 {
        exe::crt_free(read_usize(results));
        write_usize(results, 0);
    }
    if read_i32(this + SCOUT_DAY_RESULT_CAPACITY) > 0 {
        let array = exe::crt_malloc(read_u32(this + SCOUT_DAY_RESULT_CAPACITY).wrapping_mul(SCOUT_DAY_RESULT_SIZE as u32));
        write_usize(results, array);
        if array == 0 {
            show_error_box(0xa701b0, 0x89f); // player_search.cpp
            exe::game_fatal_cleanup();
            exe::shared_noop_ret0_5a8f60();
            exe::crt_exit(-1);
        }
        let n = read_u32(this + player_search_fields::RESULT_COUNT);
        if (n as i32) > 0 {
            covered!("sx17_1 load the results");
            if player_search_data_newer_than(0x3e032) {
                if exe::zread(array, SCOUT_DAY_RESULT_SIZE as u32, n, save) != n {
                    return fail(0x8a9);
                }
            } else if !player_search_load_old_results(this, save) {
                return fail(0x8b3);
            }
        }
    }
    let tail = [(0x1fa, 0x99, 0x8ce), (0x293, 0x11, 0x8d5), (SCOUT_DAY_REPORT_MARK, 4, 0x8dc), (0x2a8, 4, 0x8e3), (0x2ac, 4, 0x8ea)];
    for (offset, size, line) in tail {
        if !read(offset, size, 1) {
            return fail(line);
        }
    }
    if read_i32(this + player_search_fields::ASSIGNED_CLUB) > -1 {
        covered!("sx17_1 load the scouted club record");
        let record = exe::crt_operator_new(PLAYER_SEARCH_CLUB_RECORD_SIZE);
        if record != 0 {
            if exe::zread(record, PLAYER_SEARCH_CLUB_RECORD_SIZE, 1, save) != 1 {
                show_error_box(0xa701b0, 0x1fc3); // player_search.cpp
            } else {
                exe::llist_add_tail(G_SCOUT_CLUB_TABLE_LIST, record, 4); // sx17_1: the record joins the global list
            }
        }
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, record);
    }
    1
}

/// Comparer for `player_search_load`: it reads the save item and allocates, so it cannot run twice; only the original
/// runs. The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_player_search_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, save)
}
