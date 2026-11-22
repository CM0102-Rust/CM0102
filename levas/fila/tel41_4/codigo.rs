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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8, *mut u8) -> u32>(%%I2%%);
    let regions = [(best as usize, 1), (current as usize, 1), (worst as usize, 1), (stats + COMP_STATS_PROJECTED_ENTRY_SLOT, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club, best, current, worst),
        || comp_stats_get_club_projected_positions(stats, club, best, current, worst));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::get_club_projected_positions({:#x}, {:#x}): return {} x {}, differences in {:?}", stats, club, ro, rr, first_differences(&mo, &mr)));
    ro
}
